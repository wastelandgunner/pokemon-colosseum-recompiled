//! EXI library hooks: `EXIImm` / `EXIDma` / `EXISelect` / `EXIDeselect` drive the bus model (crate::exi).

use super::*;

pub(super) static CARD_A: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub const EXI_IMM: u32 = 0x8009820C;
pub const EXI_DMA: u32 = 0x80098408;
pub const EXI_SELECT: u32 = 0x80098B94;
pub const EXI_DESELECT: u32 = 0x80098CD4;

pub(super) static EXI: Mutex<Option<crate::exi::Exi>> = Mutex::new(None);

pub(super) fn exi_with<R>(f: impl FnOnce(&mut crate::exi::Exi) -> R) -> R {
    let mut g = EXI.lock().unwrap();
    f(g.get_or_insert_with(crate::exi::Exi::new))
}

/// `EXIImm` / `EXIDma`: the original starts the transfer, the bus runs it at once (exi.rs).
pub(super) fn exi_imm(c: &mut Cpu) {
    let ch = c.r[3] as usize;
    call_original(c, EXI_IMM);
    if c.r[3] != 0 && ch < 3 { if let Some(i) = exi_with(|e| e.transfer(c, ch)) { PENDING.lock().unwrap().push(i); } }
}
pub(super) fn exi_dma(c: &mut Cpu) {
    let ch = c.r[3] as usize;
    call_original(c, EXI_DMA);
    if c.r[3] != 0 && ch < 3 { if let Some(i) = exi_with(|e| e.transfer(c, ch)) { PENDING.lock().unwrap().push(i); } }
}
pub(super) fn exi_select(c: &mut Cpu) {
    let ch = c.r[3] as usize;
    call_original(c, EXI_SELECT);
    if ch < 3 { exi_with(|e| e.select(c, ch)); }
}
pub(super) fn exi_deselect(c: &mut Cpu) {
    let ch = c.r[3] as usize;
    call_original(c, EXI_DESELECT);
    if ch < 3 && exi_with(|e| { e.select(c, ch); std::mem::take(&mut e.card_irq) }) {
        // the card's EXI interrupt (EXI_0_EXI, __OSInterrupt 9): CSR EXIINT, taken once EXIINTMASK is on. The
        // SDK deselects with the EXI bus still locked (EXI interrupts masked) and unmasks in `EXIUnlock`; on the
        // console the erase / program finishes well after that, so the interrupt waits for the unmask (csr_write).
        let csr = c.m.u32(0xCC00_6800);
        let was = CARD_INT.swap(0, Relaxed); // (so this write isn't taken as the guest acknowledging it)
        c.m.set_u32(0xCC00_6800, csr | 2);
        CARD_INT.store(was.max(1), Relaxed);
        if csr & 1 != 0 && was != 2 { CARD_INT.store(2, Relaxed); PENDING.lock().unwrap().push(9); }
    }
}

/// The card's EXI interrupt: 0 none, 1 raised (EXIINT set, masked), 2 raised and queued for delivery.
static CARD_INT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);
use std::sync::atomic::Ordering::Relaxed;

/// EXI channel 0 CSR writes.
pub(super) fn csr_write(m: &mut crate::mem::Mem, a: u32, n: usize) {
// EXI channel 0 CSR: EXT (bit 12) is the hardware's "a device is in slot A", not writable
if covers(a, n, 0xCC00_6800) || covers(a, n, 0xCC00_6803) {
    if CARD_A.load(std::sync::atomic::Ordering::Relaxed) {
        let v = m.u32(0xCC00_6800);
        if v & 0x1000 == 0 { m.set_u32(0xCC00_6800, v | 0x1000); }
    }
    // EXIINT (bit 1) is write-1-to-clear: a write with it set acknowledges the card's interrupt
    // (EXIClearInterrupts); any other write leaves it raised, and unmasking it (EXIINTMASK, bit 0) takes it
    if CARD_INT.load(Relaxed) != 0 {
        let v = m.u32(0xCC00_6800);
        if v & 2 != 0 { m.set_u32(0xCC00_6800, v & !2); CARD_INT.store(0, Relaxed); }
        else {
            m.set_u32(0xCC00_6800, v | 2);
            if v & 1 != 0 && CARD_INT.load(Relaxed) == 1 { CARD_INT.store(2, Relaxed); PENDING.lock().unwrap().push(9); }
        }
    }
}
}
