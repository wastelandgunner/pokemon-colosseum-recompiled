//! Hand-written stand-ins for main.dol functions the lifter skips (`Working/tools/ppclift/stubs.txt`).
#![allow(non_snake_case)]

use crate::ppc::Cpu;

/// `_threadSwitch` 0x800F0308: the GS cooperative yield. Headless it returns at once (GS threads run inline);
/// a host `hook_fn` on it (colo-viewer) parks the battle thread until the next frame.
pub fn f_800f0308(c: &mut Cpu) { if c.hooks { c.stub_hit(0x800F0308); } }

/// `OSGetTime` 0x800A2778: 64-bit time base in r3:r4. Fixed (0) so lifted code is deterministic.
/// A host `hook_fn` (run.rs's time base) replaces it.
pub fn f_800a2778(c: &mut Cpu) { if c.hooks && c.stub_hit(0x800A2778) { return; } c.r[3] = 0; c.r[4] = 0; }

thread_local! {
    /// Battle RNG draws seen by `HSD_Rand` (return address, seed before), for replay checks.
    pub static RAND_LOG: std::cell::RefCell<Vec<(u32, u32)>> = const { std::cell::RefCell::new(Vec::new()) };
    /// Seeds to force before successive draws (replays: the game's particles also draw from this seed, so a
    /// replay feeds in the seed the game had at each battle draw).
    pub static RAND_FORCE: std::cell::RefCell<std::collections::VecDeque<u32>> = const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

/// `HSD_Rand` (0x801ADCD8): seed = *(0x80478C94) * 0x343FD + 0x269EC3; returns seed >> 16.
pub fn f_801adcd8(c: &mut Cpu) {
    let p = c.m.u32(0x80478C94);
    if let Some(f) = RAND_FORCE.with(|q| q.borrow_mut().pop_front()) { c.m.set_u32(p, f); }
    let s = c.m.u32(p);
    RAND_LOG.with(|l| l.borrow_mut().push((c.lr, s)));
    let n = s.wrapping_mul(0x343FD).wrapping_add(0x269EC3);
    c.m.set_u32(p, n);
    c.r[3] = n >> 16;
}
