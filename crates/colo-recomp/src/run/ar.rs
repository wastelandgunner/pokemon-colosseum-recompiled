//! ARAM (the AR library and the ARAM DMA registers): 16 MB of auxiliary memory; transfers take their DMA time.

use super::*;

pub const AR_START_DMA: u32 = 0x800AC990;

pub(super) const IRQ_DSP_ARAM: u32 = 0x06;

/// ARAM (16 MB) and which bytes were written here (the snapshot has no ARAM; reads of bytes the game stored
/// before it are reported).
pub(super) struct Aram { pub(super) bytes: Vec<u8>, pub(super) known: Vec<bool>, pub(super) warned: bool }
pub(super) static ARAM: Mutex<Option<Aram>> = Mutex::new(None);

/// `ARStartDMA(type, mainmem, aram, length)` 0x800AC990: the original programs the AR registers; the transfer
/// happens at once (type 0: main -> ARAM, 1: ARAM -> main) and the ARAM interrupt is pending.
pub(super) fn ar_start_dma(c: &mut Cpu) {
    let (ty, mm, ar, len) = (c.r[3] & 0xFFFF, c.r[4] | 0x8000_0000, c.r[5] as usize, c.r[6] as usize);
    call_original(c, AR_START_DMA);
    if trace() { eprintln!("run: AR DMA type {ty} main {mm:#x} aram {ar:#x} len {len:#x}"); }
    let mut g = ARAM.lock().unwrap();
    let a = g.get_or_insert_with(|| Aram { bytes: vec![0; 16 << 20], known: vec![false; 16 << 20], warned: false });
    let n = len.min(a.bytes.len().saturating_sub(ar));
    if ty == 0 {
        a.bytes[ar..ar + n].copy_from_slice(c.m.read(mm, n as u32));
        a.known[ar..ar + n].iter_mut().for_each(|k| *k = true);
    } else {
        if !a.warned && a.known[ar..ar + n].iter().any(|k| !k) {
            eprintln!("run: ARAM read {ar:#x}+{n:#x} includes data stored before the snapshot (not captured)");
            a.warned = true;
        }
        c.m.write(mm, &a.bytes[ar..ar + n]);
    }
    PENDING.lock().unwrap().push(IRQ_DSP_ARAM);
}

/// The ARAM DMA registers (0xCC005020..0xCC00502B): writing the count's low half starts a transfer.
pub(super) fn dma_write(m: &mut crate::mem::Mem, a: u32, n: usize) {
// ARAM DMA: writing the count's low half starts it (dir: count bit 31, 1 = ARAM -> main memory)
if covers(a, n, 0xCC00_502A) || covers(a, n, 0xCC00_502B) {
    let mm = ((m.u16(0xCC00_5020) as u32) << 16 | m.u16(0xCC00_5022) as u32) & 0x03FF_FFE0 | 0x8000_0000;
    let ar = (((m.u16(0xCC00_5024) as u32) << 16 | m.u16(0xCC00_5026) as u32) & 0x03FF_FFE0) as usize;
    let hi = m.u16(0xCC00_5028) as u32;
    let len = ((hi & 0x7FFF) << 16 | m.u16(0xCC00_502A) as u32) as usize & 0x03FF_FFE0;
    let to_main = hi & 0x8000 != 0;
    {
        let mut g = ARAM.lock().unwrap();
        let x = g.get_or_insert_with(|| Aram { bytes: vec![0; 16 << 20], known: vec![false; 16 << 20], warned: false });
        let n = len.min(x.bytes.len().saturating_sub(ar));
        if trace() { eprintln!("run: AR DMA {} main {mm:#x} aram {ar:#x} len {len:#x}", if to_main { "ARAM->main" } else { "main->ARAM" }); }
        if !to_main {
            x.bytes[ar..ar + n].copy_from_slice(m.read(mm, n as u32));
            x.known[ar..ar + n].iter_mut().for_each(|k| *k = true);
        } else {
            if !x.warned && x.known[ar..ar + n].iter().any(|k| !k) {
                eprintln!("run: ARAM read {ar:#x}+{n:#x} includes data stored before the snapshot (not captured)");
                x.warned = true;
            }
            let d = x.bytes[ar..ar + n].to_vec();
            m.write(mm, &d);
        }
    }
    // the data moves at once (as in Dolphin); the DMA stays busy (DSPCR DMAState 0x200) for its transfer time,
    // then ARINT and the interrupt if ARINTMASK (`ar_due`)
    let v = DSPCR.load(std::sync::atomic::Ordering::Relaxed) | 0x200;
    DSPCR.store(v, std::sync::atomic::Ordering::Relaxed);
    m.set_u16(0xCC00_500A, v);
    *AR_BUSY.lock().unwrap() = Some((len as u32, None));
}
}

/// The ARAM DMA in flight: (bytes, completion time base once timed). Dolphin's measured rate: 246 CPU cycles per
/// 32 bytes (20.5 time base ticks). Like the DI, it is timed from the next point that has the CPU's clock.
pub(super) static AR_BUSY: Mutex<Option<(u32, Option<u64>)>> = Mutex::new(None);

pub(super) fn ar_next(c: &Cpu) -> Option<u64> {
    let mut g = AR_BUSY.lock().unwrap();
    let (len, due) = g.as_mut()?;
    Some(*due.get_or_insert_with(|| time_base(c) + (*len as u64 / 32) * 246 / 12))
}

/// The ARAM DMA finishes if its time has come: DMAState clears, ARINT sets, the interrupt is pending if
/// ARINTMASK. Takes no `&mut Cpu` so register reads can complete it (the register itself follows at `ar_due`).
fn ar_finish(c: &Cpu) {
    match ar_next(c) { Some(t) if t <= time_base(c) => {} _ => return }
    *AR_BUSY.lock().unwrap() = None;
    let v = DSPCR.load(std::sync::atomic::Ordering::Relaxed) & !0x200 | 0x20;
    DSPCR.store(v, std::sync::atomic::Ordering::Relaxed);
    DSPCR_STALE.store(true, std::sync::atomic::Ordering::Relaxed);
    if v & 0x10 != 0 { PENDING.lock().unwrap().push(IRQ_DSP_ARAM); }
}

/// The DSPCR register in memory lags DSPCR (a read finished the DMA) until `ar_due` stores it.
static DSPCR_STALE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub(super) fn ar_due(c: &mut Cpu) {
    ar_finish(c);
    if DSPCR_STALE.swap(false, std::sync::atomic::Ordering::Relaxed) {
        c.m.set_u16(0xCC00_500A, DSPCR.load(std::sync::atomic::Ordering::Relaxed));
    }
}

/// DSPCR reads: an ARAM DMA whose time has come finishes first (`__OSInitAudioSystem` and the AR library poll
/// ARINT / DMAState with interrupts off), so the value is DSPCR's.
pub(super) fn ar_dspcr_read(c: &Cpu, a: u32, n: u32) -> Option<u32> {
    if !(a <= 0xCC00_500A && 0xCC00_500A < a + n.max(1)) { return None; }
    if AR_BUSY.lock().unwrap().is_none() && !DSPCR_STALE.load(std::sync::atomic::Ordering::Relaxed) { return None; }
    ar_finish(c);
    let v = DSPCR.load(std::sync::atomic::Ordering::Relaxed);
    let w = c.m.u32(0xCC00_5008);
    Some(match (a, n) {
        (0xCC00_500A, 2) => v,
        (0xCC00_500B, 1) => v & 0xFF,
        (0xCC00_5008, 4) => w & 0xFFFF_0000 | v,
        _ => return None,
    })
}
