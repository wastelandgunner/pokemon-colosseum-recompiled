//! Audio interface (AI): the audio DMA's blocks and their interrupt, and the live sample counter.

use super::*;

/// The audio DMA as Dolphin runs it (SystemTimers `AudioDMACallback`, DSP.cpp `UpdateAudioDMA`; described, not
/// copied): every 121,392 CPU cycles (TB_PER_AI_TICK ticks; 8 stereo samples at the GameCube's "32 kHz", 108 MHz /
/// 3372 = 32,028.5 Hz) from power-on, while AIDCR (0xCC005036) bit 15 is on, 32 bytes at the current address go to
/// the speakers (else 32 bytes of silence); after the block's length (AIDCR bits 0-14, in 32-byte units) the DMA
/// reloads the start address registers and interrupts (`__AIDHandler`: MusyX queues the next block). Turning the
/// DMA on loads the registers and interrupts at once (Dolphin: 200 cycles later).
pub(super) const TB_PER_AI_TICK: u64 = 121_392 / 12;

pub(super) struct AiDma {
    /// The next 32-byte tick (time base); 0 = not started.
    next_tick: u64,
    cur: u32,
    remaining: u32,
    /// The DMA was on at the last tick or register write.
    on: bool,
    /// Turned on: interrupt at the next opportunity.
    start_irq: bool,
}
pub(super) static AI_DMA: Mutex<AiDma> = Mutex::new(AiDma { next_tick: 0, cur: 0, remaining: 0, on: false, start_irq: false });

/// The next AI event that needs the CPU (an AI interrupt), for time skips: 0 = none.
pub(super) static AI_NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn ai_src(m: &crate::mem::Mem) -> u32 { (m.u16(0xCC00_5030) as u32 & 0x3FF) << 16 | (m.u16(0xCC00_5032) as u32 & 0xFFE0) }

/// The DMA's ticks up to now: samples to the speakers (`Ax::out`) and the AI interrupt at each block's end.
pub(super) fn ai_due(c: &mut Cpu) {
    use std::sync::atomic::Ordering::Relaxed;
    let now = time_base(c);
    loop {
        let mut irq = false;
        {
            let mut d = AI_DMA.lock().unwrap();
            let dcr = c.m.u16(0xCC00_5036);
            let on = dcr & 0x8000 != 0;
            // a snapshot taken mid-stream: the DMA is already on
            if on && !d.on { d.on = true; d.cur = ai_src(&c.m); d.remaining = (dcr & 0x7FFF) as u32; }
            if d.next_tick == 0 { d.next_tick = (now / TB_PER_AI_TICK + 1) * TB_PER_AI_TICK; }
            if std::mem::take(&mut d.start_irq) {
                irq = true;
            } else if d.next_tick <= now {
                d.next_tick += TB_PER_AI_TICK;
                let mut ax = AX.lock().unwrap();
                if on {
                    let b = c.m.read(0x8000_0000 | d.cur, 32).to_vec();
                    if let Some(ax) = ax.as_mut() { ax.out.extend(b.chunks_exact(2).map(|x| i16::from_be_bytes([x[0], x[1]]))); }
                    if d.remaining != 0 { d.remaining -= 1; d.cur += 32; }
                    if d.remaining == 0 {
                        d.cur = ai_src(&c.m);
                        d.remaining = (dcr & 0x7FFF) as u32;
                        irq = true;
                    }
                } else if let Some(ax) = ax.as_mut() {
                    ax.out.extend([0i16; 16]);
                }
            } else {
                // next interrupt: this block's last tick
                let next = if on { d.next_tick + (d.remaining.max(1) as u64 - 1) * TB_PER_AI_TICK } else { 0 };
                AI_NEXT.store(next, Relaxed);
                return;
            }
        }
        if irq { raise(c, 0x05); }
    }
}

/// AIDCR (0xCC005036) writes: turning the DMA on loads the start address and length and interrupts.
pub(super) fn dma_control_write(m: &mut crate::mem::Mem, a: u32, n: usize) {
    if !(covers(a, n, 0xCC00_5036) || covers(a, n, 0xCC00_5037)) { return; }
    let dcr = m.u16(0xCC00_5036);
    let mut d = AI_DMA.lock().unwrap();
    let on = dcr & 0x8000 != 0;
    if on && !d.on {
        d.cur = ai_src(m);
        d.remaining = (dcr & 0x7FFF) as u32;
        d.start_irq = true;
        AI_NEXT.store(1, std::sync::atomic::Ordering::Relaxed);
    }
    d.on = on;
}

/// AI sample counter start (time base when AICR PSTAT went on), u64::MAX = stopped.
pub(super) static AI_START: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);

/// Live register reads: the AI sample counter (AISCNT 0xCC006C08) counts at the AI rate while playing.
pub(super) fn mmio_read(c: &Cpu, a: u32, n: u32) -> Option<u32> {
    use std::sync::atomic::Ordering::Relaxed;
    if let Some(v) = ar_dspcr_read(c, a, n) { return Some(v); }
    if (0xCC00_6C08..0xCC00_6C0C).contains(&a) {
        let cr = c.m.u32(0xCC00_6C00);
        if cr & 1 == 0 { return Some(c.m.u32(0xCC00_6C08)); }
        let now = time_base(c);
        let start = match AI_START.load(Relaxed) { u64::MAX => { AI_START.store(now, Relaxed); now } s => s };
        let rate = if cr & 0x40 != 0 { 32000 } else { 48000 };
        return Some(((now - start) * rate / TB_HZ) as u32);
    }
    None
}

/// AICR (0xCC006C00): clearing PSTAT stops the sample counter.
pub(super) fn control_write(m: &mut crate::mem::Mem, a: u32, n: usize) {
if covers(a, n, 0xCC00_6C03) && m.u32(0xCC00_6C00) & 1 == 0 {
    AI_START.store(u64::MAX, std::sync::atomic::Ordering::Relaxed);
}
}
