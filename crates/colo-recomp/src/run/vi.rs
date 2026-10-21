//! Video interface (VI): the retrace interrupt each field, and the frame buffer the VI shows next.

use super::*;

/// One video field: the VI retrace interrupt, then the SI poll that completes during it (si::poll: SISR RDSTn for
/// each controller that answered, SICOMCSR RDSTINT), which runs the polling handlers (the game's PADRead through
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
                if di > time_base(c) { TB_BASE.store(di, Relaxed); TB_MARK.store(c.cycles, Relaxed); }
                dev_due(c);
                raise_pending(c);
                continue;
            }
            if ai == 0 || ai > f { break; }
            if ai > time_base(c) { TB_BASE.store(ai, Relaxed); TB_MARK.store(c.cycles, Relaxed); }
            ai_due(c);
            raise_pending(c);
        }
    }
    tb_field(c);
    raise(c, 0x18);
    // the SI poll, once polling is on: RDSTINT (with its mask, RDSTINTMSK, as the SDK keeps it) runs the polling
    // handlers (the game's PAD sampling callback, which calls PADRead)
    if !si::poll(&mut c.m) { return; }
    let csr = c.m.u32(0xCC00_6434);
    c.m.set_u32(0xCC00_6434, csr | 0x1000_0000);
    if csr & 0x0800_0000 != 0 { raise(c, 0x14); }
    si::poll_done(&mut c.m);
}

pub const VI_SET_NEXT_FB: u32 = 0x800AA198;
/// The XFB the game asked the VI to show next (`VISetNextFrameBuffer`, physical).
pub(super) static NEXT_XFB: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
pub(super) fn vi_set_next_fb(c: &mut Cpu) {
    NEXT_XFB.store(c.r[3] & 0x01FF_FFFF, std::sync::atomic::Ordering::Relaxed);
    call_original(c, VI_SET_NEXT_FB);
}

/// `GSgfxWaitRetrace` 0x800D361C (the frame's wait for its fields, mainRetraceDraw) and `GSgfxSetFrameInterval`
/// 0x800D3074 (fields a frame waits for: `gfx`+0x58, gfx = *(r13-0x5DA0); the game asks 1, 60 updates/s, on the
/// title and 2, 30/s, in battle and the field). The game advances by the fields its frame took (gfx+0x54, which the
/// wait leaves at least the interval), so capping the interval (`FRAME_INTERVAL`, 0 = the game's own) changes how
/// often it updates, not its speed.
pub const GFX_WAIT_RETRACE: u32 = 0x800D361C;
pub const GFX_SET_FRAME_INTERVAL: u32 = 0x800D3074;
const GFX: u32 = 0x5DA0; // r13 -
static FRAME_INTERVAL: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// The interval the game asked for, and the value last written over it (u32::MAX: none). gfx+0x58 differing from
/// what was written means the game set it (its setter, or its init's direct store): that is its own choice.
static GAME_INTERVAL: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);
static WROTE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(u32::MAX);

pub(super) fn gfx_set_frame_interval(c: &mut Cpu) {
    use std::sync::atomic::Ordering::Relaxed;
    if c.r[3] != 0 { GAME_INTERVAL.store(c.r[3], Relaxed); WROTE.store(u32::MAX, Relaxed); }
    call_original(c, GFX_SET_FRAME_INTERVAL);
    apply_interval(c);
}

pub(super) fn gfx_wait_retrace(c: &mut Cpu) {
    apply_interval(c);
    call_original(c, GFX_WAIT_RETRACE);
}

/// gfx+0x58 = the game's interval, capped by the setting; with the setting off and nothing written, untouched.
fn apply_interval(c: &mut Cpu) {
    use std::sync::atomic::Ordering::Relaxed;
    let gfx = c.m.u32(c.r[13].wrapping_sub(GFX));
    if gfx == 0 { return; }
    let cur = c.m.u32(gfx + 0x58);
    if cur != WROTE.load(Relaxed) { GAME_INTERVAL.store(cur, Relaxed); WROTE.store(u32::MAX, Relaxed); }
    let (game, cap) = (GAME_INTERVAL.load(Relaxed), FRAME_INTERVAL.load(Relaxed));
    let want = if cap != 0 { game.min(cap) } else { game };
    if want != cur { c.m.set_u32(gfx + 0x58, want); WROTE.store(want, Relaxed); }
}

/// Fields per frame at most (1: 60 updates a second), 0 for the game's own choice.
pub fn set_frame_interval(n: u32) { FRAME_INTERVAL.store(n, std::sync::atomic::Ordering::Relaxed); }
