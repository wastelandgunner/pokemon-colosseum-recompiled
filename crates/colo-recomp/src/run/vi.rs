//! Video interface (VI): the retrace interrupt each field, and the frame buffer the VI shows next.

use super::*;

/// One video field: the VI retrace interrupt, then the SI poll that completes during it (SICOMCSR RDSTINT +
/// mask, SISR RDST0: controller 1 answered), which runs the polling handlers (PADRead through
/// inputPadCallback).
pub(super) fn field(c: &mut Cpu) {
    // time jumps to the field: the AI blocks, disc command and ARAM DMA that end before it play out first, each with its
    // interrupt
    {
        use std::sync::atomic::Ordering::Relaxed;
        let f = FIELD_TB.load(Relaxed) + TB_PER_FIELD;
        loop {
            let ai = AI_NEXT.load(Relaxed);
            let di = dev_next(c);
            if time_base(c) >= f { break; }
            if let Some(di) = di.filter(|&d| d <= f && (ai == 0 || d <= ai)) {
                if di > time_base(c) { TB_BASE.store(di, Relaxed); TB_MARK.store(c.instrs, Relaxed); }
                dev_due(c);
                raise_pending(c);
                continue;
            }
            if ai == 0 || ai > f { break; }
            if ai > time_base(c) { TB_BASE.store(ai, Relaxed); TB_MARK.store(c.instrs, Relaxed); }
            ai_due(c);
            raise_pending(c);
        }
    }
    tb_field(c);
    raise(c, 0x18);
    // the SI poll only once polling is on (SIPOLL 0xCC006430 EN0-3, bits 7..4)
    if c.m.u32(0xCC00_6430) & 0xF0 == 0 { return; }
    let csr = c.m.u32(0xCC00_6434);
    c.m.set_u32(0xCC00_6434, csr | 0x1800_0000);
    let sr = c.m.u32(0xCC00_6438);
    c.m.set_u32(0xCC00_6438, sr | 0x2000_0000);
    raise(c, 0x14);
    let csr = c.m.u32(0xCC00_6434);
    c.m.set_u32(0xCC00_6434, csr & !0x1000_0000);
}

pub const VI_SET_NEXT_FB: u32 = 0x800AA198;
/// The XFB the game asked the VI to show next (`VISetNextFrameBuffer`, physical).
pub(super) static NEXT_XFB: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
pub(super) fn vi_set_next_fb(c: &mut Cpu) {
    NEXT_XFB.store(c.r[3] & 0x01FF_FFFF, std::sync::atomic::Ordering::Relaxed);
    call_original(c, VI_SET_NEXT_FB);
}
