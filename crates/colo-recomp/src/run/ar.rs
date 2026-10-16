//! ARAM (the AR library and the ARAM DMA registers): 16 MB of auxiliary memory, transfers complete at once.

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
    // done at once: ARINT set, interrupt if ARINTMASK
    let v = DSPCR.load(std::sync::atomic::Ordering::Relaxed) | 0x20;
    DSPCR.store(v, std::sync::atomic::Ordering::Relaxed);
    m.set_u16(0xCC00_500A, v);
    if v & 0x10 != 0 { PENDING.lock().unwrap().push(IRQ_DSP_ARAM); }
}
}
