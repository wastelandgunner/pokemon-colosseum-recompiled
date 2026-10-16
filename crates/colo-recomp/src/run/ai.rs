//! Audio interface (AI): the audio DMA's blocks and their interrupt, and the live sample counter.

use super::*;

/// The next AI DMA block's end (time base): one block (160 samples at 32 kHz) every 5 ms = 202,500 ticks; 0 = not
/// started. Blocks end at their own times (not bunched at fields) so each `__AIDHandler` finds the DSP's last
/// frame done, as on hardware.
pub(super) static AI_NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
pub(super) const TB_PER_AI: u64 = 202_500;
/// The AI DMA block now playing: (physical address, bytes).
pub(super) static AI_LATCH: Mutex<Option<(u32, u32)>> = Mutex::new(None);

/// AI DMA blocks whose time has come: the AI interrupt for each, while the AI DMA runs (AIDCR 0xCC005036 bit 15).
pub(super) fn ai_due(c: &mut Cpu) {
    use std::sync::atomic::Ordering::Relaxed;
    loop {
        let now = time_base(c);
        let n = AI_NEXT.load(Relaxed);
        if n == 0 || now > n + 8 * TB_PER_AI { AI_NEXT.store(now + TB_PER_AI, Relaxed); return; }
        if now < n { return; }
        AI_NEXT.store(n + TB_PER_AI, Relaxed);
        let dcr = c.m.u16(0xCC00_5036);
        if dcr & 0x8000 != 0 {
            // the block that just played is what the speakers got (AX writes R, L 16-bit pairs); the DMA then
            // latches the registers for the next block and interrupts so the game can queue another
            let mut latch = AI_LATCH.lock().unwrap();
            if let Some((a, len)) = *latch {
                let b = c.m.read(0x8000_0000 | a, len).to_vec();
                if let Some(ax) = AX.lock().unwrap().as_mut() {
                    ax.out.extend(b.chunks_exact(2).map(|x| i16::from_be_bytes([x[0], x[1]])));
                }
            }
            let a = (c.m.u16(0xCC00_5030) as u32 & 0x3FF) << 16 | (c.m.u16(0xCC00_5032) as u32 & 0xFFE0);
            *latch = Some((a, (dcr as u32 & 0x7FFF) * 32));
            drop(latch);
            raise(c, 0x05);
        } else if trace() { eprintln!("run: AI DMA off (AIDCR {:#x})", c.m.u16(0xCC00_5036)); }
    }
}

/// AI sample counter start (time base when AICR PSTAT went on), u64::MAX = stopped.
pub(super) static AI_START: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);

/// Live register reads: the AI sample counter (AISCNT 0xCC006C08) counts at the AI rate while playing.
pub(super) fn mmio_read(c: &Cpu, a: u32, _n: u32) -> Option<u32> {
    use std::sync::atomic::Ordering::Relaxed;
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
