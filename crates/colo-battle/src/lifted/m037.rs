#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fightOutPokemonEnemySearchAry (0x8020e57c, 152 bytes)
#[inline(never)]
pub fn f_8020e57c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e57c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e57c); f_8020e57c_b(c, 0x8020e57c); crate::prof::exit(c, 0x8020e57c); return; }
    }
    f_8020e57c_b(c, 0x8020e57c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e57c_at(c: &mut Cpu, pc: u32) { f_8020e57c_b(c, pc) }
#[inline(always)]
fn f_8020e57c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e57c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e57c, pc); }
        match pc {
            0x8020e57c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                c.r[28] = c.r[5] | c.r[5];
                if !c.crbit(2) { pc = 0x8020e5a0; continue; }
                pc = 0x8020e598;
            }
            0x8020e598 => {
                c.r[3] = 0x0;
                pc = 0x8020e600; continue;
                pc = 0x8020e5a0;
            }
            0x8020e5a0 => {
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.r[29] = 0x0;
                pc = 0x8020e5f0; continue;
                pc = 0x8020e5ac;
            }
            0x8020e5ac => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].wrapping_mul(0xc);
                c.r[30] = c.r[27].wrapping_add(c.r[0]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020e5c0; f_801fd104(c);
                pc = 0x8020e5c0;
            }
            0x8020e5c0 => {
                c.r[3] = c.r[3].wrapping_neg();
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020e5ec; continue; }
                pc = 0x8020e5d4;
            }
            0x8020e5d4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020e5dc; f_801fd104(c);
                pc = 0x8020e5dc;
            }
            0x8020e5dc => {
                c.cmpu(0, c.r[3], c.r[28]);
                if !c.crbit(2) { pc = 0x8020e5ec; continue; }
                pc = 0x8020e5e4;
            }
            0x8020e5e4 => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x8020e600; continue;
                pc = 0x8020e5ec;
            }
            0x8020e5ec => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x8020e5f0;
            }
            0x8020e5f0 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020e5ac; continue; }
                pc = 0x8020e5fc;
            }
            0x8020e5fc => {
                c.r[3] = 0x0;
                pc = 0x8020e600;
            }
            0x8020e600 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e57c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyCheckValid (0x8020e614, 44 bytes)
#[inline(never)]
pub fn f_8020e614(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e614) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e614); f_8020e614_b(c, 0x8020e614); crate::prof::exit(c, 0x8020e614); return; }
    }
    f_8020e614_b(c, 0x8020e614)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e614_at(c: &mut Cpu, pc: u32) { f_8020e614_b(c, pc) }
#[inline(always)]
fn f_8020e614_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e614;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e614, pc); }
        match pc {
            0x8020e614 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8020e624; f_801fd104(c);
                pc = 0x8020e624;
            }
            0x8020e624 => {
                c.r[3] = c.r[3].wrapping_neg();
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e614: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyCreate (0x8020e640, 148 bytes)
#[inline(never)]
pub fn f_8020e640(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e640) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e640); f_8020e640_b(c, 0x8020e640); crate::prof::exit(c, 0x8020e640); return; }
    }
    f_8020e640_b(c, 0x8020e640)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e640_at(c: &mut Cpu, pc: u32) { f_8020e640_b(c, pc) }
#[inline(always)]
fn f_8020e640_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e640;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e640, pc); }
        match pc {
            0x8020e640 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                c.r[31] = c.r[4] | c.r[4];
                if c.crbit(2) { pc = 0x8020e6c0; continue; }
                pc = 0x8020e65c;
            }
            0x8020e65c => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8020e6c0; continue; }
                pc = 0x8020e664;
            }
            0x8020e664 => {
                c.r[4] = 0x0;
                c.lr = 0x8020e66c; f_801fd0ac(c);
                pc = 0x8020e66c;
            }
            0x8020e66c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8020e678; f_801fd09c(c);
                pc = 0x8020e678;
            }
            0x8020e678 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8020e684; f_801fd08c(c);
                pc = 0x8020e684;
            }
            0x8020e684 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8020e690; f_801fd07c(c);
                pc = 0x8020e690;
            }
            0x8020e690 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8020e69c; f_801fd0ac(c);
                pc = 0x8020e69c;
            }
            0x8020e69c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020e6a4; f_80205b8c(c);
                pc = 0x8020e6a4;
            }
            0x8020e6a4 => {
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x8020e6b4; f_8012640c(c);
                pc = 0x8020e6b4;
            }
            0x8020e6b4 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020e6c0; f_801fd08c(c);
                pc = 0x8020e6c0;
            }
            0x8020e6c0 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e640: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyInitAry (0x8020e6d4, 132 bytes)
#[inline(never)]
pub fn f_8020e6d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e6d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e6d4); f_8020e6d4_b(c, 0x8020e6d4); crate::prof::exit(c, 0x8020e6d4); return; }
    }
    f_8020e6d4_b(c, 0x8020e6d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e6d4_at(c: &mut Cpu, pc: u32) { f_8020e6d4_b(c, pc) }
#[inline(always)]
fn f_8020e6d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e6d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e6d4, pc); }
        match pc {
            0x8020e6d4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                if c.crbit(2) { pc = 0x8020e744; continue; }
                pc = 0x8020e6ec;
            }
            0x8020e6ec => {
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.r[29] = 0x0;
                pc = 0x8020e738; continue;
                pc = 0x8020e6f8;
            }
            0x8020e6f8 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[0] = c.r[0].wrapping_mul(0xc);
                c.r[30] = c.r[28].wrapping_add(c.r[0]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020e710; f_801fd0ac(c);
                pc = 0x8020e710;
            }
            0x8020e710 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8020e71c; f_801fd09c(c);
                pc = 0x8020e71c;
            }
            0x8020e71c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8020e728; f_801fd08c(c);
                pc = 0x8020e728;
            }
            0x8020e728 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8020e734; f_801fd07c(c);
                pc = 0x8020e734;
            }
            0x8020e734 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x8020e738;
            }
            0x8020e738 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020e6f8; continue; }
                pc = 0x8020e744;
            }
            0x8020e744 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e6d4: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyInit (0x8020e758, 84 bytes)
#[inline(never)]
pub fn f_8020e758(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e758) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e758); f_8020e758_b(c, 0x8020e758); crate::prof::exit(c, 0x8020e758); return; }
    }
    f_8020e758_b(c, 0x8020e758)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e758_at(c: &mut Cpu, pc: u32) { f_8020e758_b(c, pc) }
#[inline(always)]
fn f_8020e758_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e758;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e758, pc); }
        match pc {
            0x8020e758 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020e774; f_801fd0ac(c);
                pc = 0x8020e774;
            }
            0x8020e774 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8020e780; f_801fd09c(c);
                pc = 0x8020e780;
            }
            0x8020e780 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8020e78c; f_801fd08c(c);
                pc = 0x8020e78c;
            }
            0x8020e78c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8020e798; f_801fd07c(c);
                pc = 0x8020e798;
            }
            0x8020e798 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e758: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonEraseAry (0x8020e7ac, 432 bytes)
#[inline(never)]
pub fn f_8020e7ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e7ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e7ac); f_8020e7ac_b(c, 0x8020e7ac); crate::prof::exit(c, 0x8020e7ac); return; }
    }
    f_8020e7ac_b(c, 0x8020e7ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e7ac_at(c: &mut Cpu, pc: u32) { f_8020e7ac_b(c, pc) }
#[inline(always)]
fn f_8020e7ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e7ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e7ac, pc); }
        match pc {
            0x8020e7ac => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                c.r[28] = c.r[5] | c.r[5];
                if !c.crbit(2) { pc = 0x8020e7d0; continue; }
                pc = 0x8020e7c8;
            }
            0x8020e7c8 => {
                c.r[3] = 0x0;
                pc = 0x8020e948; continue;
                pc = 0x8020e7d0;
            }
            0x8020e7d0 => {
                c.r[0] = c.r[28] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8020e7e0; continue; }
                pc = 0x8020e7d8;
            }
            0x8020e7d8 => {
                c.r[3] = 0x0;
                pc = 0x8020e948; continue;
                pc = 0x8020e7e0;
            }
            0x8020e7e0 => {
                c.cmpu(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x8020e7f0; continue; }
                pc = 0x8020e7e8;
            }
            0x8020e7e8 => {
                c.r[29] = 0x0;
                pc = 0x8020e8ac; continue;
                pc = 0x8020e7f0;
            }
            0x8020e7f0 => {
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.r[30] = 0x0;
                pc = 0x8020e89c; continue;
                pc = 0x8020e7fc;
            }
            0x8020e7fc => {
                c.r[3] = c.r[30].rotate_left(0) & 0xffff;
                c.r[0] = c.r[28] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0x14);
                c.r[29] = c.r[27].wrapping_add(c.r[0]);
                if !c.crbit(0) { pc = 0x8020e848; continue; }
                pc = 0x8020e810;
            }
            0x8020e810 => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x8020e820; continue; }
                pc = 0x8020e818;
            }
            0x8020e818 => {
                c.r[0] = 0x0;
                pc = 0x8020e83c; continue;
                pc = 0x8020e820;
            }
            0x8020e820 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020e828; f_801fbf04(c);
                pc = 0x8020e828;
            }
            0x8020e828 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8020e838; continue; }
                pc = 0x8020e830;
            }
            0x8020e830 => {
                c.r[0] = 0x0;
                pc = 0x8020e83c; continue;
                pc = 0x8020e838;
            }
            0x8020e838 => {
                c.r[0] = 0x1;
                pc = 0x8020e83c;
            }
            0x8020e83c => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020e898; continue; }
                pc = 0x8020e844;
            }
            0x8020e844 => {
                pc = 0x8020e8ac; continue;
                pc = 0x8020e848;
            }
            0x8020e848 => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x8020e858; continue; }
                pc = 0x8020e850;
            }
            0x8020e850 => {
                c.r[0] = 0x0;
                pc = 0x8020e874; continue;
                pc = 0x8020e858;
            }
            0x8020e858 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020e860; f_801fbf04(c);
                pc = 0x8020e860;
            }
            0x8020e860 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8020e870; continue; }
                pc = 0x8020e868;
            }
            0x8020e868 => {
                c.r[0] = 0x0;
                pc = 0x8020e874; continue;
                pc = 0x8020e870;
            }
            0x8020e870 => {
                c.r[0] = 0x1;
                pc = 0x8020e874;
            }
            0x8020e874 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020e898; continue; }
                pc = 0x8020e87c;
            }
            0x8020e87c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020e884; f_801fbf04(c);
                pc = 0x8020e884;
            }
            0x8020e884 => {
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[0] = c.r[28] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8020e898; continue; }
                pc = 0x8020e894;
            }
            0x8020e894 => {
                pc = 0x8020e8ac; continue;
                pc = 0x8020e898;
            }
            0x8020e898 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x8020e89c;
            }
            0x8020e89c => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020e7fc; continue; }
                pc = 0x8020e8a8;
            }
            0x8020e8a8 => {
                c.r[29] = 0x0;
                pc = 0x8020e8ac;
            }
            0x8020e8ac => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x8020e8bc; continue; }
                pc = 0x8020e8b4;
            }
            0x8020e8b4 => {
                c.r[3] = 0x0;
                pc = 0x8020e948; continue;
                pc = 0x8020e8bc;
            }
            0x8020e8bc => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0xffffffff;
                c.lr = 0x8020e8c8; f_801fbe18(c);
                pc = 0x8020e8c8;
            }
            0x8020e8c8 => {
                c.r[30] = 0x0;
                pc = 0x8020e8e4; continue;
                pc = 0x8020e8d0;
            }
            0x8020e8d0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x8020e8e0; f_801fbdf4(c);
                pc = 0x8020e8e0;
            }
            0x8020e8e0 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x8020e8e4;
            }
            0x8020e8e4 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8020e8d0; continue; }
                pc = 0x8020e8f0;
            }
            0x8020e8f0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020e8fc; f_801fbde4(c);
                pc = 0x8020e8fc;
            }
            0x8020e8fc => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020e908; f_801fbdd4(c);
                pc = 0x8020e908;
            }
            0x8020e908 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020e914; f_801fbdc4(c);
                pc = 0x8020e914;
            }
            0x8020e914 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020e920; f_801fbdb4(c);
                pc = 0x8020e920;
            }
            0x8020e920 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020e92c; f_801fbda4(c);
                pc = 0x8020e92c;
            }
            0x8020e92c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020e938; f_801fbd94(c);
                pc = 0x8020e938;
            }
            0x8020e938 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020e944; f_801fbd84(c);
                pc = 0x8020e944;
            }
            0x8020e944 => {
                c.r[3] = 0x1;
                pc = 0x8020e948;
            }
            0x8020e948 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e7ac: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonRegistAry (0x8020e95c, 588 bytes)
#[inline(never)]
pub fn f_8020e95c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e95c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e95c); f_8020e95c_b(c, 0x8020e95c); crate::prof::exit(c, 0x8020e95c); return; }
    }
    f_8020e95c_b(c, 0x8020e95c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e95c_at(c: &mut Cpu, pc: u32) { f_8020e95c_b(c, pc) }
#[inline(always)]
fn f_8020e95c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e95c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e95c, pc); }
        match pc {
            0x8020e95c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = c.r[5] | c.r[5];
                if !c.crbit(2) { pc = 0x8020e984; continue; }
                pc = 0x8020e97c;
            }
            0x8020e97c => {
                c.r[3] = 0x0;
                pc = 0x8020eb94; continue;
                pc = 0x8020e984;
            }
            0x8020e984 => {
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8020e994; continue; }
                pc = 0x8020e98c;
            }
            0x8020e98c => {
                c.r[3] = 0x0;
                pc = 0x8020eb94; continue;
                pc = 0x8020e994;
            }
            0x8020e994 => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x8020e9a4; continue; }
                pc = 0x8020e99c;
            }
            0x8020e99c => {
                c.r[26] = 0x0;
                pc = 0x8020ea60; continue;
                pc = 0x8020e9a4;
            }
            0x8020e9a4 => {
                c.r[28] = c.r[30].rotate_left(0) & 0xffff;
                c.r[27] = 0x0;
                pc = 0x8020ea50; continue;
                pc = 0x8020e9b0;
            }
            0x8020e9b0 => {
                c.r[3] = c.r[27].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0x14);
                c.r[26] = c.r[29].wrapping_add(c.r[0]);
                if !c.crbit(0) { pc = 0x8020e9fc; continue; }
                pc = 0x8020e9c4;
            }
            0x8020e9c4 => {
                c.cmpu(0, c.r[26], 0x0);
                if !c.crbit(2) { pc = 0x8020e9d4; continue; }
                pc = 0x8020e9cc;
            }
            0x8020e9cc => {
                c.r[0] = 0x0;
                pc = 0x8020e9f0; continue;
                pc = 0x8020e9d4;
            }
            0x8020e9d4 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020e9dc; f_801fbf04(c);
                pc = 0x8020e9dc;
            }
            0x8020e9dc => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8020e9ec; continue; }
                pc = 0x8020e9e4;
            }
            0x8020e9e4 => {
                c.r[0] = 0x0;
                pc = 0x8020e9f0; continue;
                pc = 0x8020e9ec;
            }
            0x8020e9ec => {
                c.r[0] = 0x1;
                pc = 0x8020e9f0;
            }
            0x8020e9f0 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020ea4c; continue; }
                pc = 0x8020e9f8;
            }
            0x8020e9f8 => {
                pc = 0x8020ea60; continue;
                pc = 0x8020e9fc;
            }
            0x8020e9fc => {
                c.cmpu(0, c.r[26], 0x0);
                if !c.crbit(2) { pc = 0x8020ea0c; continue; }
                pc = 0x8020ea04;
            }
            0x8020ea04 => {
                c.r[0] = 0x0;
                pc = 0x8020ea28; continue;
                pc = 0x8020ea0c;
            }
            0x8020ea0c => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020ea14; f_801fbf04(c);
                pc = 0x8020ea14;
            }
            0x8020ea14 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8020ea24; continue; }
                pc = 0x8020ea1c;
            }
            0x8020ea1c => {
                c.r[0] = 0x0;
                pc = 0x8020ea28; continue;
                pc = 0x8020ea24;
            }
            0x8020ea24 => {
                c.r[0] = 0x1;
                pc = 0x8020ea28;
            }
            0x8020ea28 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020ea4c; continue; }
                pc = 0x8020ea30;
            }
            0x8020ea30 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020ea38; f_801fbf04(c);
                pc = 0x8020ea38;
            }
            0x8020ea38 => {
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8020ea4c; continue; }
                pc = 0x8020ea48;
            }
            0x8020ea48 => {
                pc = 0x8020ea60; continue;
                pc = 0x8020ea4c;
            }
            0x8020ea4c => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x8020ea50;
            }
            0x8020ea50 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[28]);
                if c.crbit(0) { pc = 0x8020e9b0; continue; }
                pc = 0x8020ea5c;
            }
            0x8020ea5c => {
                c.r[26] = 0x0;
                pc = 0x8020ea60;
            }
            0x8020ea60 => {
                c.cmpu(0, c.r[26], 0x0);
                if c.crbit(2) { pc = 0x8020ea70; continue; }
                pc = 0x8020ea68;
            }
            0x8020ea68 => {
                c.r[3] = 0x0;
                pc = 0x8020eb94; continue;
                pc = 0x8020ea70;
            }
            0x8020ea70 => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x8020ea80; continue; }
                pc = 0x8020ea78;
            }
            0x8020ea78 => {
                c.r[27] = 0x0;
                pc = 0x8020eae0; continue;
                pc = 0x8020ea80;
            }
            0x8020ea80 => {
                c.r[30] = c.r[30].rotate_left(0) & 0xffff;
                c.r[26] = 0x0;
                pc = 0x8020ead0; continue;
                pc = 0x8020ea8c;
            }
            0x8020ea8c => {
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].wrapping_mul(0x14);
                c.r[27] = c.r[29].wrapping_add(c.r[0]);
                c.rc(c.r[27]);
                if !c.crbit(2) { pc = 0x8020eaa4; continue; }
                pc = 0x8020ea9c;
            }
            0x8020ea9c => {
                c.r[0] = 0x0;
                pc = 0x8020eac0; continue;
                pc = 0x8020eaa4;
            }
            0x8020eaa4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020eaac; f_801fbf04(c);
                pc = 0x8020eaac;
            }
            0x8020eaac => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8020eabc; continue; }
                pc = 0x8020eab4;
            }
            0x8020eab4 => {
                c.r[0] = 0x0;
                pc = 0x8020eac0; continue;
                pc = 0x8020eabc;
            }
            0x8020eabc => {
                c.r[0] = 0x1;
                pc = 0x8020eac0;
            }
            0x8020eac0 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020eacc; continue; }
                pc = 0x8020eac8;
            }
            0x8020eac8 => {
                pc = 0x8020eae0; continue;
                pc = 0x8020eacc;
            }
            0x8020eacc => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8020ead0;
            }
            0x8020ead0 => {
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[30]);
                if c.crbit(0) { pc = 0x8020ea8c; continue; }
                pc = 0x8020eadc;
            }
            0x8020eadc => {
                c.r[27] = 0x0;
                pc = 0x8020eae0;
            }
            0x8020eae0 => {
                c.cmpu(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x8020eaf0; continue; }
                pc = 0x8020eae8;
            }
            0x8020eae8 => {
                c.r[3] = 0x0;
                pc = 0x8020eb94; continue;
                pc = 0x8020eaf0;
            }
            0x8020eaf0 => {
                if c.crbit(2) { pc = 0x8020eb90; continue; }
                pc = 0x8020eaf4;
            }
            0x8020eaf4 => {
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x8020eb90; continue; }
                pc = 0x8020eafc;
            }
            0x8020eafc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0xffffffff;
                c.lr = 0x8020eb08; f_801fbe18(c);
                pc = 0x8020eb08;
            }
            0x8020eb08 => {
                c.r[26] = 0x0;
                pc = 0x8020eb24; continue;
                pc = 0x8020eb10;
            }
            0x8020eb10 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x0;
                c.lr = 0x8020eb20; f_801fbdf4(c);
                pc = 0x8020eb20;
            }
            0x8020eb20 => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8020eb24;
            }
            0x8020eb24 => {
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8020eb10; continue; }
                pc = 0x8020eb30;
            }
            0x8020eb30 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.lr = 0x8020eb3c; f_801fbde4(c);
                pc = 0x8020eb3c;
            }
            0x8020eb3c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.lr = 0x8020eb48; f_801fbdd4(c);
                pc = 0x8020eb48;
            }
            0x8020eb48 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.lr = 0x8020eb54; f_801fbdc4(c);
                pc = 0x8020eb54;
            }
            0x8020eb54 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.lr = 0x8020eb60; f_801fbdb4(c);
                pc = 0x8020eb60;
            }
            0x8020eb60 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.lr = 0x8020eb6c; f_801fbda4(c);
                pc = 0x8020eb6c;
            }
            0x8020eb6c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.lr = 0x8020eb78; f_801fbd94(c);
                pc = 0x8020eb78;
            }
            0x8020eb78 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.lr = 0x8020eb84; f_801fbd84(c);
                pc = 0x8020eb84;
            }
            0x8020eb84 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8020eb90; f_801fbe18(c);
                pc = 0x8020eb90;
            }
            0x8020eb90 => {
                c.r[3] = 0x1;
                pc = 0x8020eb94;
            }
            0x8020eb94 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e95c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonSearchAry (0x8020eba8, 252 bytes)
#[inline(never)]
pub fn f_8020eba8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020eba8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020eba8); f_8020eba8_b(c, 0x8020eba8); crate::prof::exit(c, 0x8020eba8); return; }
    }
    f_8020eba8_b(c, 0x8020eba8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020eba8_at(c: &mut Cpu, pc: u32) { f_8020eba8_b(c, pc) }
#[inline(always)]
fn f_8020eba8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020eba8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020eba8, pc); }
        match pc {
            0x8020eba8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                c.r[28] = c.r[5] | c.r[5];
                if !c.crbit(2) { pc = 0x8020ebcc; continue; }
                pc = 0x8020ebc4;
            }
            0x8020ebc4 => {
                c.r[3] = 0x0;
                pc = 0x8020ec90; continue;
                pc = 0x8020ebcc;
            }
            0x8020ebcc => {
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.r[29] = 0x0;
                pc = 0x8020ec80; continue;
                pc = 0x8020ebd8;
            }
            0x8020ebd8 => {
                c.r[3] = c.r[29].rotate_left(0) & 0xffff;
                c.r[0] = c.r[28] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0x14);
                c.r[30] = c.r[27].wrapping_add(c.r[0]);
                if !c.crbit(0) { pc = 0x8020ec28; continue; }
                pc = 0x8020ebec;
            }
            0x8020ebec => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8020ebfc; continue; }
                pc = 0x8020ebf4;
            }
            0x8020ebf4 => {
                c.r[0] = 0x0;
                pc = 0x8020ec18; continue;
                pc = 0x8020ebfc;
            }
            0x8020ebfc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020ec04; f_801fbf04(c);
                pc = 0x8020ec04;
            }
            0x8020ec04 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8020ec14; continue; }
                pc = 0x8020ec0c;
            }
            0x8020ec0c => {
                c.r[0] = 0x0;
                pc = 0x8020ec18; continue;
                pc = 0x8020ec14;
            }
            0x8020ec14 => {
                c.r[0] = 0x1;
                pc = 0x8020ec18;
            }
            0x8020ec18 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020ec7c; continue; }
                pc = 0x8020ec20;
            }
            0x8020ec20 => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x8020ec90; continue;
                pc = 0x8020ec28;
            }
            0x8020ec28 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8020ec38; continue; }
                pc = 0x8020ec30;
            }
            0x8020ec30 => {
                c.r[0] = 0x0;
                pc = 0x8020ec54; continue;
                pc = 0x8020ec38;
            }
            0x8020ec38 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020ec40; f_801fbf04(c);
                pc = 0x8020ec40;
            }
            0x8020ec40 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8020ec50; continue; }
                pc = 0x8020ec48;
            }
            0x8020ec48 => {
                c.r[0] = 0x0;
                pc = 0x8020ec54; continue;
                pc = 0x8020ec50;
            }
            0x8020ec50 => {
                c.r[0] = 0x1;
                pc = 0x8020ec54;
            }
            0x8020ec54 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020ec7c; continue; }
                pc = 0x8020ec5c;
            }
            0x8020ec5c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020ec64; f_801fbf04(c);
                pc = 0x8020ec64;
            }
            0x8020ec64 => {
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[0] = c.r[28] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8020ec7c; continue; }
                pc = 0x8020ec74;
            }
            0x8020ec74 => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x8020ec90; continue;
                pc = 0x8020ec7c;
            }
            0x8020ec7c => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x8020ec80;
            }
            0x8020ec80 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020ebd8; continue; }
                pc = 0x8020ec8c;
            }
            0x8020ec8c => {
                c.r[3] = 0x0;
                pc = 0x8020ec90;
            }
            0x8020ec90 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020eba8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonCheckValid (0x8020eca4, 60 bytes)
#[inline(never)]
pub fn f_8020eca4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020eca4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020eca4); f_8020eca4_b(c, 0x8020eca4); crate::prof::exit(c, 0x8020eca4); return; }
    }
    f_8020eca4_b(c, 0x8020eca4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020eca4_at(c: &mut Cpu, pc: u32) { f_8020eca4_b(c, pc) }
#[inline(always)]
fn f_8020eca4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020eca4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020eca4, pc); }
        match pc {
            0x8020eca4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if !c.crbit(2) { pc = 0x8020ecc0; continue; }
                pc = 0x8020ecb8;
            }
            0x8020ecb8 => {
                c.r[3] = 0x0;
                pc = 0x8020ecd0; continue;
                pc = 0x8020ecc0;
            }
            0x8020ecc0 => {
                c.lr = 0x8020ecc4; f_801fbf04(c);
                pc = 0x8020ecc4;
            }
            0x8020ecc4 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[0] = c.r[0].rotate_left(1) & 0x1;
                c.r[3] = c.r[0] ^ 0x1;
                pc = 0x8020ecd0;
            }
            0x8020ecd0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020eca4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonInitAry (0x8020ece0, 220 bytes)
#[inline(never)]
pub fn f_8020ece0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ece0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ece0); f_8020ece0_b(c, 0x8020ece0); crate::prof::exit(c, 0x8020ece0); return; }
    }
    f_8020ece0_b(c, 0x8020ece0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ece0_at(c: &mut Cpu, pc: u32) { f_8020ece0_b(c, pc) }
#[inline(always)]
fn f_8020ece0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ece0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ece0, pc); }
        match pc {
            0x8020ece0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                if c.crbit(2) { pc = 0x8020eda8; continue; }
                pc = 0x8020ecf8;
            }
            0x8020ecf8 => {
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.r[28] = 0x0;
                pc = 0x8020ed9c; continue;
                pc = 0x8020ed04;
            }
            0x8020ed04 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.r[4] = 0xffffffff;
                c.r[0] = c.r[0].wrapping_mul(0x14);
                c.r[29] = c.r[27].wrapping_add(c.r[0]);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020ed1c; f_801fbe18(c);
                pc = 0x8020ed1c;
            }
            0x8020ed1c => {
                c.r[30] = 0x0;
                pc = 0x8020ed38; continue;
                pc = 0x8020ed24;
            }
            0x8020ed24 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x8020ed34; f_801fbdf4(c);
                pc = 0x8020ed34;
            }
            0x8020ed34 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x8020ed38;
            }
            0x8020ed38 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8020ed24; continue; }
                pc = 0x8020ed44;
            }
            0x8020ed44 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020ed50; f_801fbde4(c);
                pc = 0x8020ed50;
            }
            0x8020ed50 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020ed5c; f_801fbdd4(c);
                pc = 0x8020ed5c;
            }
            0x8020ed5c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020ed68; f_801fbdc4(c);
                pc = 0x8020ed68;
            }
            0x8020ed68 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020ed74; f_801fbdb4(c);
                pc = 0x8020ed74;
            }
            0x8020ed74 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020ed80; f_801fbda4(c);
                pc = 0x8020ed80;
            }
            0x8020ed80 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020ed8c; f_801fbd94(c);
                pc = 0x8020ed8c;
            }
            0x8020ed8c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020ed98; f_801fbd84(c);
                pc = 0x8020ed98;
            }
            0x8020ed98 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x8020ed9c;
            }
            0x8020ed9c => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020ed04; continue; }
                pc = 0x8020eda8;
            }
            0x8020eda8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020ece0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonInitFightOutStatus (0x8020edbc, 96 bytes)
#[inline(never)]
pub fn f_8020edbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020edbc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020edbc); f_8020edbc_b(c, 0x8020edbc); crate::prof::exit(c, 0x8020edbc); return; }
    }
    f_8020edbc_b(c, 0x8020edbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020edbc_at(c: &mut Cpu, pc: u32) { f_8020edbc_b(c, pc) }
#[inline(always)]
fn f_8020edbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020edbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020edbc, pc); }
        match pc {
            0x8020edbc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020edd8; f_801fbdc4(c);
                pc = 0x8020edd8;
            }
            0x8020edd8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8020ede4; f_801fbdb4(c);
                pc = 0x8020ede4;
            }
            0x8020ede4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8020edf0; f_801fbda4(c);
                pc = 0x8020edf0;
            }
            0x8020edf0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8020edfc; f_801fbd94(c);
                pc = 0x8020edfc;
            }
            0x8020edfc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8020ee08; f_801fbd84(c);
                pc = 0x8020ee08;
            }
            0x8020ee08 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020edbc: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncOiuchi (0x8020ee1c, 164 bytes)
#[inline(never)]
pub fn f_8020ee1c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ee1c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ee1c); f_8020ee1c_b(c, 0x8020ee1c); crate::prof::exit(c, 0x8020ee1c); return; }
    }
    f_8020ee1c_b(c, 0x8020ee1c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ee1c_at(c: &mut Cpu, pc: u32) { f_8020ee1c_b(c, pc) }
#[inline(always)]
fn f_8020ee1c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ee1c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ee1c, pc); }
        match pc {
            0x8020ee1c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = c.r[7] | c.r[7];
                c.r[5] = 0x1;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8020ee50; f_80208750(c);
                pc = 0x8020ee50;
            }
            0x8020ee50 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020ee64; f_80208750(c);
                pc = 0x8020ee64;
            }
            0x8020ee64 => {
                c.r[3] = 0x11;
                c.lr = 0x8020ee6c; f_801f0234(c);
                pc = 0x8020ee6c;
            }
            0x8020ee6c => {
                c.lr = 0x8020ee70; f_801f0204(c);
                pc = 0x8020ee70;
            }
            0x8020ee70 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020ee88; f_802085c4(c);
                pc = 0x8020ee88;
            }
            0x8020ee88 => {
                c.r[3] = 0x12;
                c.lr = 0x8020ee90; f_801f0234(c);
                pc = 0x8020ee90;
            }
            0x8020ee90 => {
                c.lr = 0x8020ee94; f_801f0204(c);
                pc = 0x8020ee94;
            }
            0x8020ee94 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020eeac; f_802085c4(c);
                pc = 0x8020eeac;
            }
            0x8020eeac => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020ee1c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncOiuchi (0x8020eec0, 20 bytes)
#[inline(never)]
pub fn f_8020eec0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020eec0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020eec0); f_8020eec0_b(c, 0x8020eec0); crate::prof::exit(c, 0x8020eec0); return; }
    }
    f_8020eec0_b(c, 0x8020eec0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020eec0_at(c: &mut Cpu, pc: u32) { f_8020eec0_b(c, pc) }
#[inline(always)]
fn f_8020eec0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020eec0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020eec0, pc); }
        match pc {
            0x8020eec0 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffade0));
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0xff;
                if true { return; }
                return;
            }
            _ => panic!("f_8020eec0: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncHuuin (0x8020eed4, 556 bytes)
#[inline(never)]
pub fn f_8020eed4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020eed4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020eed4); f_8020eed4_b(c, 0x8020eed4); crate::prof::exit(c, 0x8020eed4); return; }
    }
    f_8020eed4_b(c, 0x8020eed4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020eed4_at(c: &mut Cpu, pc: u32) { f_8020eed4_b(c, pc) }
#[inline(always)]
fn f_8020eed4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020eed4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020eed4, pc); }
        match pc {
            0x8020eed4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.r[27] = c.r[4] | c.r[4];
                c.r[29] = c.r[7] | c.r[7];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8020ef04; f_801f54a4(c);
                pc = 0x8020ef04;
            }
            0x8020ef04 => {
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[30] | c.r[30];
                c.r[3] = 0xf;
                c.lr = 0x8020ef18; f_801f02ac(c);
                pc = 0x8020ef18;
            }
            0x8020ef18 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[30] | c.r[30];
                c.r[3] = 0x10;
                c.lr = 0x8020ef2c; f_801f02ac(c);
                pc = 0x8020ef2c;
            }
            0x8020ef2c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.lr = 0x8020ef48; f_80208750(c);
                pc = 0x8020ef48;
            }
            0x8020ef48 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[29] | c.r[29];
                c.r[5] = 0x2;
                c.lr = 0x8020ef5c; f_80208750(c);
                pc = 0x8020ef5c;
            }
            0x8020ef5c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[29] | c.r[29];
                c.r[5] = 0x2;
                c.lr = 0x8020ef70; f_80208750(c);
                pc = 0x8020ef70;
            }
            0x8020ef70 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020ef78; f_802062fc(c);
                pc = 0x8020ef78;
            }
            0x8020ef78 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020f008; continue; }
                pc = 0x8020ef84;
            }
            0x8020ef84 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020ef8c; f_802062fc(c);
                pc = 0x8020ef8c;
            }
            0x8020ef8c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020f008; continue; }
                pc = 0x8020ef98;
            }
            0x8020ef98 => {
                c.r[3] = 0x11;
                c.lr = 0x8020efa0; f_801f0234(c);
                pc = 0x8020efa0;
            }
            0x8020efa0 => {
                c.lr = 0x8020efa4; f_801f0204(c);
                pc = 0x8020efa4;
            }
            0x8020efa4 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020efbc; f_802085c4(c);
                pc = 0x8020efbc;
            }
            0x8020efbc => {
                c.r[3] = 0x12;
                c.lr = 0x8020efc4; f_801f0234(c);
                pc = 0x8020efc4;
            }
            0x8020efc4 => {
                c.lr = 0x8020efc8; f_801f0204(c);
                pc = 0x8020efc8;
            }
            0x8020efc8 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.r[6] = 0x1;
                c.lr = 0x8020efe0; f_802085c4(c);
                pc = 0x8020efe0;
            }
            0x8020efe0 => {
                c.r[3] = 0x12;
                c.lr = 0x8020efe8; f_801f0234(c);
                pc = 0x8020efe8;
            }
            0x8020efe8 => {
                c.lr = 0x8020efec; f_801f0204(c);
                pc = 0x8020efec;
            }
            0x8020efec => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020f004; f_802085c4(c);
                pc = 0x8020f004;
            }
            0x8020f004 => {
                pc = 0x8020f0ec; continue;
                pc = 0x8020f008;
            }
            0x8020f008 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020f010; f_802062fc(c);
                pc = 0x8020f010;
            }
            0x8020f010 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020f068; continue; }
                pc = 0x8020f01c;
            }
            0x8020f01c => {
                c.r[3] = 0x11;
                c.lr = 0x8020f024; f_801f0234(c);
                pc = 0x8020f024;
            }
            0x8020f024 => {
                c.lr = 0x8020f028; f_801f0204(c);
                pc = 0x8020f028;
            }
            0x8020f028 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020f040; f_802085c4(c);
                pc = 0x8020f040;
            }
            0x8020f040 => {
                c.r[3] = 0x12;
                c.lr = 0x8020f048; f_801f0234(c);
                pc = 0x8020f048;
            }
            0x8020f048 => {
                c.lr = 0x8020f04c; f_801f0204(c);
                pc = 0x8020f04c;
            }
            0x8020f04c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020f064; f_802085c4(c);
                pc = 0x8020f064;
            }
            0x8020f064 => {
                pc = 0x8020f0ec; continue;
                pc = 0x8020f068;
            }
            0x8020f068 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020f070; f_802062fc(c);
                pc = 0x8020f070;
            }
            0x8020f070 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020f0c8; continue; }
                pc = 0x8020f07c;
            }
            0x8020f07c => {
                c.r[3] = 0x11;
                c.lr = 0x8020f084; f_801f0234(c);
                pc = 0x8020f084;
            }
            0x8020f084 => {
                c.lr = 0x8020f088; f_801f0204(c);
                pc = 0x8020f088;
            }
            0x8020f088 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020f0a0; f_802085c4(c);
                pc = 0x8020f0a0;
            }
            0x8020f0a0 => {
                c.r[3] = 0x12;
                c.lr = 0x8020f0a8; f_801f0234(c);
                pc = 0x8020f0a8;
            }
            0x8020f0a8 => {
                c.lr = 0x8020f0ac; f_801f0204(c);
                pc = 0x8020f0ac;
            }
            0x8020f0ac => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020f0c4; f_802085c4(c);
                pc = 0x8020f0c4;
            }
            0x8020f0c4 => {
                pc = 0x8020f0ec; continue;
                pc = 0x8020f0c8;
            }
            0x8020f0c8 => {
                c.r[3] = 0x11;
                c.lr = 0x8020f0d0; f_801f0234(c);
                pc = 0x8020f0d0;
            }
            0x8020f0d0 => {
                c.lr = 0x8020f0d4; f_801f0204(c);
                pc = 0x8020f0d4;
            }
            0x8020f0d4 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020f0ec; f_802085c4(c);
                pc = 0x8020f0ec;
            }
            0x8020f0ec => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020eed4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncHuuin (0x8020f100, 8 bytes)
#[inline(never)]
pub fn f_8020f100(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f100) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f100); f_8020f100_b(c, 0x8020f100); crate::prof::exit(c, 0x8020f100); return; }
    }
    f_8020f100_b(c, 0x8020f100)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f100_at(c: &mut Cpu, pc: u32) { f_8020f100_b(c, pc) }
#[inline(always)]
fn f_8020f100_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f100;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f100, pc); }
        match pc {
            0x8020f100 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020f100: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncMigawari (0x8020f108, 296 bytes)
#[inline(never)]
pub fn f_8020f108(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f108) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f108); f_8020f108_b(c, 0x8020f108); crate::prof::exit(c, 0x8020f108); return; }
    }
    f_8020f108_b(c, 0x8020f108)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f108_at(c: &mut Cpu, pc: u32) { f_8020f108_b(c, pc) }
#[inline(always)]
fn f_8020f108_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f108;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f108, pc); }
        match pc {
            0x8020f108 => {
                { let ea = c.r[1].wrapping_add(0xfffff900); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x704), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x6f0); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x8020f12c; f_801f453c(c);
                pc = 0x8020f12c;
            }
            0x8020f12c => {
                c.r[30] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.lr = 0x8020f144; f_8011beb4(c);
                pc = 0x8020f144;
            }
            0x8020f144 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xee;
                c.r[6] = 0x0;
                c.lr = 0x8020f158; f_8012640c(c);
                pc = 0x8020f158;
            }
            0x8020f158 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020f16c; f_80207c6c(c);
                pc = 0x8020f16c;
            }
            0x8020f16c => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8020f17c; f_801fcec4(c);
                pc = 0x8020f17c;
            }
            0x8020f17c => {
                c.r[7] = c.r[30] | c.r[30];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x0;
                c.r[5] = 0xee;
                c.r[6] = 0x0;
                c.lr = 0x8020f194; f_801254b4(c);
                pc = 0x8020f194;
            }
            0x8020f194 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x8020f1a8; f_80208750(c);
                pc = 0x8020f1a8;
            }
            0x8020f1a8 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x8020f1bc; f_80208750(c);
                pc = 0x8020f1bc;
            }
            0x8020f1bc => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x8020f1d4; f_802085c4(c);
                pc = 0x8020f1d4;
            }
            0x8020f1d4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020f1e0; f_801c3d64(c);
                pc = 0x8020f1e0;
            }
            0x8020f1e0 => {
                c.lr = 0x8020f1e4; f_801c3430(c);
                pc = 0x8020f1e4;
            }
            0x8020f1e4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[7] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xee;
                c.r[6] = 0x0;
                c.lr = 0x8020f1fc; f_801254b4(c);
                pc = 0x8020f1fc;
            }
            0x8020f1fc => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020f214; f_802085c4(c);
                pc = 0x8020f214;
            }
            0x8020f214 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020f21c; f_801db100(c);
                pc = 0x8020f21c;
            }
            0x8020f21c => {
                { let mut ea = c.r[1].wrapping_add(0x6f0); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x704));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x700);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f108: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncMigawari (0x8020f230, 8 bytes)
#[inline(never)]
pub fn f_8020f230(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f230) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f230); f_8020f230_b(c, 0x8020f230); crate::prof::exit(c, 0x8020f230); return; }
    }
    f_8020f230_b(c, 0x8020f230)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f230_at(c: &mut Cpu, pc: u32) { f_8020f230_b(c, pc) }
#[inline(always)]
fn f_8020f230_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f230;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f230, pc); }
        match pc {
            0x8020f230 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020f230: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncHensin (0x8020f238, 296 bytes)
#[inline(never)]
pub fn f_8020f238(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f238) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f238); f_8020f238_b(c, 0x8020f238); crate::prof::exit(c, 0x8020f238); return; }
    }
    f_8020f238_b(c, 0x8020f238)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f238_at(c: &mut Cpu, pc: u32) { f_8020f238_b(c, pc) }
#[inline(always)]
fn f_8020f238_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f238;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f238, pc); }
        match pc {
            0x8020f238 => {
                { let ea = c.r[1].wrapping_add(0xfffff900); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x704), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x6f0); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x8020f25c; f_801f453c(c);
                pc = 0x8020f25c;
            }
            0x8020f25c => {
                c.r[30] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.lr = 0x8020f274; f_8011beb4(c);
                pc = 0x8020f274;
            }
            0x8020f274 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xee;
                c.r[6] = 0x0;
                c.lr = 0x8020f288; f_8012640c(c);
                pc = 0x8020f288;
            }
            0x8020f288 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020f29c; f_80207c6c(c);
                pc = 0x8020f29c;
            }
            0x8020f29c => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8020f2ac; f_801fcec4(c);
                pc = 0x8020f2ac;
            }
            0x8020f2ac => {
                c.r[7] = c.r[30] | c.r[30];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x0;
                c.r[5] = 0xee;
                c.r[6] = 0x0;
                c.lr = 0x8020f2c4; f_801254b4(c);
                pc = 0x8020f2c4;
            }
            0x8020f2c4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x8020f2d8; f_80208750(c);
                pc = 0x8020f2d8;
            }
            0x8020f2d8 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x3;
                c.r[6] = 0x1;
                c.lr = 0x8020f2ec; f_80208750(c);
                pc = 0x8020f2ec;
            }
            0x8020f2ec => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x3;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x8020f304; f_802085c4(c);
                pc = 0x8020f304;
            }
            0x8020f304 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020f310; f_801c3d64(c);
                pc = 0x8020f310;
            }
            0x8020f310 => {
                c.lr = 0x8020f314; f_801c3430(c);
                pc = 0x8020f314;
            }
            0x8020f314 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[7] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xee;
                c.r[6] = 0x0;
                c.lr = 0x8020f32c; f_801254b4(c);
                pc = 0x8020f32c;
            }
            0x8020f32c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020f344; f_802085c4(c);
                pc = 0x8020f344;
            }
            0x8020f344 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020f34c; f_801db100(c);
                pc = 0x8020f34c;
            }
            0x8020f34c => {
                { let mut ea = c.r[1].wrapping_add(0x6f0); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x704));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x700);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f238: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncHensin (0x8020f360, 8 bytes)
#[inline(never)]
pub fn f_8020f360(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f360) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f360); f_8020f360_b(c, 0x8020f360); crate::prof::exit(c, 0x8020f360); return; }
    }
    f_8020f360_b(c, 0x8020f360)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f360_at(c: &mut Cpu, pc: u32) { f_8020f360_b(c, pc) }
#[inline(always)]
fn f_8020f360_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f360;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f360, pc); }
        match pc {
            0x8020f360 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020f360: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncMagiccort (0x8020f368, 128 bytes)
#[inline(never)]
pub fn f_8020f368(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f368) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f368); f_8020f368_b(c, 0x8020f368); crate::prof::exit(c, 0x8020f368); return; }
    }
    f_8020f368_b(c, 0x8020f368)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f368_at(c: &mut Cpu, pc: u32) { f_8020f368_b(c, pc) }
#[inline(always)]
fn f_8020f368_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f368;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f368, pc); }
        match pc {
            0x8020f368 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[7] | c.r[7];
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[5] | c.r[5];
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = 0x1;
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8020f398; f_80208750(c);
                pc = 0x8020f398;
            }
            0x8020f398 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x37;
                c.lr = 0x8020f3a4; f_802026e4(c);
                pc = 0x8020f3a4;
            }
            0x8020f3a4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020f3d4; continue; }
                pc = 0x8020f3b0;
            }
            0x8020f3b0 => {
                c.r[3] = 0x11;
                c.lr = 0x8020f3b8; f_801f0234(c);
                pc = 0x8020f3b8;
            }
            0x8020f3b8 => {
                c.lr = 0x8020f3bc; f_801f0204(c);
                pc = 0x8020f3bc;
            }
            0x8020f3bc => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x8020f3d4; f_802085c4(c);
                pc = 0x8020f3d4;
            }
            0x8020f3d4 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f368: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncMagiccort (0x8020f3e8, 8 bytes)
#[inline(never)]
pub fn f_8020f3e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f3e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f3e8); f_8020f3e8_b(c, 0x8020f3e8); crate::prof::exit(c, 0x8020f3e8); return; }
    }
    f_8020f3e8_b(c, 0x8020f3e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f3e8_at(c: &mut Cpu, pc: u32) { f_8020f3e8_b(c, pc) }
#[inline(always)]
fn f_8020f3e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f3e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f3e8, pc); }
        match pc {
            0x8020f3e8 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020f3e8: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncWeatherBall (0x8020f3f0, 164 bytes)
#[inline(never)]
pub fn f_8020f3f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f3f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f3f0); f_8020f3f0_b(c, 0x8020f3f0); crate::prof::exit(c, 0x8020f3f0); return; }
    }
    f_8020f3f0_b(c, 0x8020f3f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f3f0_at(c: &mut Cpu, pc: u32) { f_8020f3f0_b(c, pc) }
#[inline(always)]
fn f_8020f3f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f3f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f3f0, pc); }
        match pc {
            0x8020f3f0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[7] | c.r[7];
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.lr = 0x8020f424; f_80208750(c);
                pc = 0x8020f424;
            }
            0x8020f424 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020f438; f_80208750(c);
                pc = 0x8020f438;
            }
            0x8020f438 => {
                c.r[3] = 0x11;
                c.lr = 0x8020f440; f_801f0234(c);
                pc = 0x8020f440;
            }
            0x8020f440 => {
                c.lr = 0x8020f444; f_801f0204(c);
                pc = 0x8020f444;
            }
            0x8020f444 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020f45c; f_802085c4(c);
                pc = 0x8020f45c;
            }
            0x8020f45c => {
                c.r[3] = 0x12;
                c.lr = 0x8020f464; f_801f0234(c);
                pc = 0x8020f464;
            }
            0x8020f464 => {
                c.lr = 0x8020f468; f_801f0204(c);
                pc = 0x8020f468;
            }
            0x8020f468 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020f480; f_802085c4(c);
                pc = 0x8020f480;
            }
            0x8020f480 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f3f0: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncWeatherBall (0x8020f494, 132 bytes)
#[inline(never)]
pub fn f_8020f494(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f494) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f494); f_8020f494_b(c, 0x8020f494); crate::prof::exit(c, 0x8020f494); return; }
    }
    f_8020f494_b(c, 0x8020f494)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f494_at(c: &mut Cpu, pc: u32) { f_8020f494_b(c, pc) }
#[inline(always)]
fn f_8020f494_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f494;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f494, pc); }
        match pc {
            0x8020f494 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8020f4ac; f_801f453c(c);
                pc = 0x8020f4ac;
            }
            0x8020f4ac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x8020f4e4; continue; }
                pc = 0x8020f4b8;
            }
            0x8020f4b8 => {
                if !c.crbit(0) { pc = 0x8020f4cc; continue; }
                pc = 0x8020f4bc;
            }
            0x8020f4bc => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8020f4dc; continue; }
                pc = 0x8020f4c4;
            }
            0x8020f4c4 => {
                if !c.crbit(0) { pc = 0x8020f4f4; continue; }
                pc = 0x8020f4c8;
            }
            0x8020f4c8 => {
                pc = 0x8020f504; continue;
                pc = 0x8020f4cc;
            }
            0x8020f4cc => {
                c.cmps(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x8020f4ec; continue; }
                pc = 0x8020f4d4;
            }
            0x8020f4d4 => {
                if !c.crbit(0) { pc = 0x8020f504; continue; }
                pc = 0x8020f4d8;
            }
            0x8020f4d8 => {
                pc = 0x8020f4fc; continue;
                pc = 0x8020f4dc;
            }
            0x8020f4dc => {
                c.r[3] = 0x0;
                pc = 0x8020f508; continue;
                pc = 0x8020f4e4;
            }
            0x8020f4e4 => {
                c.r[3] = 0x1;
                pc = 0x8020f508; continue;
                pc = 0x8020f4ec;
            }
            0x8020f4ec => {
                c.r[3] = 0x2;
                pc = 0x8020f508; continue;
                pc = 0x8020f4f4;
            }
            0x8020f4f4 => {
                c.r[3] = 0x3;
                pc = 0x8020f508; continue;
                pc = 0x8020f4fc;
            }
            0x8020f4fc => {
                c.r[3] = 0x4;
                pc = 0x8020f508; continue;
                pc = 0x8020f504;
            }
            0x8020f504 => {
                c.r[3] = 0x0;
                pc = 0x8020f508;
            }
            0x8020f508 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f494: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncIceball (0x8020f518, 164 bytes)
#[inline(never)]
pub fn f_8020f518(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f518) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f518); f_8020f518_b(c, 0x8020f518); crate::prof::exit(c, 0x8020f518); return; }
    }
    f_8020f518_b(c, 0x8020f518)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f518_at(c: &mut Cpu, pc: u32) { f_8020f518_b(c, pc) }
#[inline(always)]
fn f_8020f518_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f518;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f518, pc); }
        match pc {
            0x8020f518 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[7] | c.r[7];
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.lr = 0x8020f54c; f_80208750(c);
                pc = 0x8020f54c;
            }
            0x8020f54c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020f560; f_80208750(c);
                pc = 0x8020f560;
            }
            0x8020f560 => {
                c.r[3] = 0x11;
                c.lr = 0x8020f568; f_801f0234(c);
                pc = 0x8020f568;
            }
            0x8020f568 => {
                c.lr = 0x8020f56c; f_801f0204(c);
                pc = 0x8020f56c;
            }
            0x8020f56c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020f584; f_802085c4(c);
                pc = 0x8020f584;
            }
            0x8020f584 => {
                c.r[3] = 0x12;
                c.lr = 0x8020f58c; f_801f0234(c);
                pc = 0x8020f58c;
            }
            0x8020f58c => {
                c.lr = 0x8020f590; f_801f0204(c);
                pc = 0x8020f590;
            }
            0x8020f590 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020f5a8; f_802085c4(c);
                pc = 0x8020f5a8;
            }
            0x8020f5a8 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f518: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncIceball (0x8020f5bc, 140 bytes)
#[inline(never)]
pub fn f_8020f5bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f5bc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f5bc); f_8020f5bc_b(c, 0x8020f5bc); crate::prof::exit(c, 0x8020f5bc); return; }
    }
    f_8020f5bc_b(c, 0x8020f5bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f5bc_at(c: &mut Cpu, pc: u32) { f_8020f5bc_b(c, pc) }
#[inline(always)]
fn f_8020f5bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f5bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f5bc, pc); }
        match pc {
            0x8020f5bc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x2f;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8020f5d4; f_80202360(c);
                pc = 0x8020f5d4;
            }
            0x8020f5d4 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x8020f614; continue; }
                pc = 0x8020f5e0;
            }
            0x8020f5e0 => {
                if !c.crbit(0) { pc = 0x8020f5fc; continue; }
                pc = 0x8020f5e4;
            }
            0x8020f5e4 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8020f634; continue; }
                pc = 0x8020f5ec;
            }
            0x8020f5ec => {
                if !c.crbit(0) { pc = 0x8020f60c; continue; }
                pc = 0x8020f5f0;
            }
            0x8020f5f0 => {
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(0) { pc = 0x8020f62c; continue; }
                pc = 0x8020f5f8;
            }
            0x8020f5f8 => {
                pc = 0x8020f634; continue;
                pc = 0x8020f5fc;
            }
            0x8020f5fc => {
                c.cmps(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x8020f624; continue; }
                pc = 0x8020f604;
            }
            0x8020f604 => {
                if !c.crbit(0) { pc = 0x8020f634; continue; }
                pc = 0x8020f608;
            }
            0x8020f608 => {
                pc = 0x8020f61c; continue;
                pc = 0x8020f60c;
            }
            0x8020f60c => {
                c.r[3] = 0x0;
                pc = 0x8020f638; continue;
                pc = 0x8020f614;
            }
            0x8020f614 => {
                c.r[3] = 0x1;
                pc = 0x8020f638; continue;
                pc = 0x8020f61c;
            }
            0x8020f61c => {
                c.r[3] = 0x2;
                pc = 0x8020f638; continue;
                pc = 0x8020f624;
            }
            0x8020f624 => {
                c.r[3] = 0x3;
                pc = 0x8020f638; continue;
                pc = 0x8020f62c;
            }
            0x8020f62c => {
                c.r[3] = 0x4;
                pc = 0x8020f638; continue;
                pc = 0x8020f634;
            }
            0x8020f634 => {
                c.r[3] = 0x0;
                pc = 0x8020f638;
            }
            0x8020f638 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f5bc: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncKawarawari (0x8020f648, 164 bytes)
#[inline(never)]
pub fn f_8020f648(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f648) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f648); f_8020f648_b(c, 0x8020f648); crate::prof::exit(c, 0x8020f648); return; }
    }
    f_8020f648_b(c, 0x8020f648)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f648_at(c: &mut Cpu, pc: u32) { f_8020f648_b(c, pc) }
#[inline(always)]
fn f_8020f648_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f648;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f648, pc); }
        match pc {
            0x8020f648 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = c.r[7] | c.r[7];
                c.r[5] = 0x1;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8020f67c; f_80208750(c);
                pc = 0x8020f67c;
            }
            0x8020f67c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020f690; f_80208750(c);
                pc = 0x8020f690;
            }
            0x8020f690 => {
                c.r[3] = 0x11;
                c.lr = 0x8020f698; f_801f0234(c);
                pc = 0x8020f698;
            }
            0x8020f698 => {
                c.lr = 0x8020f69c; f_801f0204(c);
                pc = 0x8020f69c;
            }
            0x8020f69c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020f6b4; f_802085c4(c);
                pc = 0x8020f6b4;
            }
            0x8020f6b4 => {
                c.r[3] = 0x12;
                c.lr = 0x8020f6bc; f_801f0234(c);
                pc = 0x8020f6bc;
            }
            0x8020f6bc => {
                c.lr = 0x8020f6c0; f_801f0204(c);
                pc = 0x8020f6c0;
            }
            0x8020f6c0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020f6d8; f_802085c4(c);
                pc = 0x8020f6d8;
            }
            0x8020f6d8 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f648: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncKawarawari (0x8020f6ec, 96 bytes)
#[inline(never)]
pub fn f_8020f6ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f6ec) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f6ec); f_8020f6ec_b(c, 0x8020f6ec); crate::prof::exit(c, 0x8020f6ec); return; }
    }
    f_8020f6ec_b(c, 0x8020f6ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f6ec_at(c: &mut Cpu, pc: u32) { f_8020f6ec_b(c, pc) }
#[inline(always)]
fn f_8020f6ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f6ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f6ec, pc); }
        match pc {
            0x8020f6ec => {
                c.r[3] = 0x80380000;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x6002));
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x8020f734; continue; }
                pc = 0x8020f704;
            }
            0x8020f704 => {
                if !c.crbit(0) { pc = 0x8020f718; continue; }
                pc = 0x8020f708;
            }
            0x8020f708 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8020f724; continue; }
                pc = 0x8020f710;
            }
            0x8020f710 => {
                if !c.crbit(0) { pc = 0x8020f72c; continue; }
                pc = 0x8020f714;
            }
            0x8020f714 => {
                pc = 0x8020f744; continue;
                pc = 0x8020f718;
            }
            0x8020f718 => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x8020f744; continue; }
                pc = 0x8020f720;
            }
            0x8020f720 => {
                pc = 0x8020f73c; continue;
                pc = 0x8020f724;
            }
            0x8020f724 => {
                c.r[3] = 0x3;
                if true { return; }
                pc = 0x8020f72c;
            }
            0x8020f72c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020f734;
            }
            0x8020f734 => {
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x8020f73c;
            }
            0x8020f73c => {
                c.r[3] = 0x2;
                if true { return; }
                pc = 0x8020f744;
            }
            0x8020f744 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020f6ec: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncNegaigoto (0x8020f74c, 100 bytes)
#[inline(never)]
pub fn f_8020f74c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f74c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f74c); f_8020f74c_b(c, 0x8020f74c); crate::prof::exit(c, 0x8020f74c); return; }
    }
    f_8020f74c_b(c, 0x8020f74c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f74c_at(c: &mut Cpu, pc: u32) { f_8020f74c_b(c, pc) }
#[inline(always)]
fn f_8020f74c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f74c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f74c, pc); }
        match pc {
            0x8020f74c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x1;
                c.r[6] = c.r[7] | c.r[7];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020f778; f_80208750(c);
                pc = 0x8020f778;
            }
            0x8020f778 => {
                c.r[3] = 0x11;
                c.lr = 0x8020f780; f_801f0234(c);
                pc = 0x8020f780;
            }
            0x8020f780 => {
                c.lr = 0x8020f784; f_801f0204(c);
                pc = 0x8020f784;
            }
            0x8020f784 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x8020f79c; f_802085c4(c);
                pc = 0x8020f79c;
            }
            0x8020f79c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f74c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncNegaigoto (0x8020f7b0, 8 bytes)
#[inline(never)]
pub fn f_8020f7b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f7b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f7b0); f_8020f7b0_b(c, 0x8020f7b0); crate::prof::exit(c, 0x8020f7b0); return; }
    }
    f_8020f7b0_b(c, 0x8020f7b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f7b0_at(c: &mut Cpu, pc: u32) { f_8020f7b0_b(c, pc) }
#[inline(always)]
fn f_8020f7b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f7b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f7b0, pc); }
        match pc {
            0x8020f7b0 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020f7b0: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncTedasuke (0x8020f7b8, 276 bytes)
#[inline(never)]
pub fn f_8020f7b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f7b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f7b8); f_8020f7b8_b(c, 0x8020f7b8); crate::prof::exit(c, 0x8020f7b8); return; }
    }
    f_8020f7b8_b(c, 0x8020f7b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f7b8_at(c: &mut Cpu, pc: u32) { f_8020f7b8_b(c, pc) }
#[inline(always)]
fn f_8020f7b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f7b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f7b8, pc); }
        match pc {
            0x8020f7b8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[7] | c.r[7];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8020f7e8; f_801f54a4(c);
                pc = 0x8020f7e8;
            }
            0x8020f7e8 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8020f800; f_8012640c(c);
                pc = 0x8020f800;
            }
            0x8020f800 => {
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.lr = 0x8020f810; f_8011beb4(c);
                pc = 0x8020f810;
            }
            0x8020f810 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8020f81c; f_801f00d0(c);
                pc = 0x8020f81c;
            }
            0x8020f81c => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = 0xe;
                c.lr = 0x8020f82c; f_801f02ac(c);
                pc = 0x8020f82c;
            }
            0x8020f82c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x1;
                c.lr = 0x8020f848; f_80208750(c);
                pc = 0x8020f848;
            }
            0x8020f848 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x3;
                c.lr = 0x8020f85c; f_80208750(c);
                pc = 0x8020f85c;
            }
            0x8020f85c => {
                c.r[3] = 0x11;
                c.lr = 0x8020f864; f_801f0234(c);
                pc = 0x8020f864;
            }
            0x8020f864 => {
                c.lr = 0x8020f868; f_801f0204(c);
                pc = 0x8020f868;
            }
            0x8020f868 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020f880; f_802085c4(c);
                pc = 0x8020f880;
            }
            0x8020f880 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020f888; f_802062fc(c);
                pc = 0x8020f888;
            }
            0x8020f888 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020f8b8; continue; }
                pc = 0x8020f894;
            }
            0x8020f894 => {
                c.r[3] = 0x11;
                c.lr = 0x8020f89c; f_801f0234(c);
                pc = 0x8020f89c;
            }
            0x8020f89c => {
                c.lr = 0x8020f8a0; f_801f0204(c);
                pc = 0x8020f8a0;
            }
            0x8020f8a0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x8020f8b8; f_802085c4(c);
                pc = 0x8020f8b8;
            }
            0x8020f8b8 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f7b8: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncTedasuke (0x8020f8cc, 8 bytes)
#[inline(never)]
pub fn f_8020f8cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f8cc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f8cc); f_8020f8cc_b(c, 0x8020f8cc); crate::prof::exit(c, 0x8020f8cc); return; }
    }
    f_8020f8cc_b(c, 0x8020f8cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f8cc_at(c: &mut Cpu, pc: u32) { f_8020f8cc_b(c, pc) }
#[inline(always)]
fn f_8020f8cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f8cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f8cc, pc); }
        match pc {
            0x8020f8cc => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020f8cc: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncWeatherHP (0x8020f8d4, 100 bytes)
#[inline(never)]
pub fn f_8020f8d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f8d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f8d4); f_8020f8d4_b(c, 0x8020f8d4); crate::prof::exit(c, 0x8020f8d4); return; }
    }
    f_8020f8d4_b(c, 0x8020f8d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f8d4_at(c: &mut Cpu, pc: u32) { f_8020f8d4_b(c, pc) }
#[inline(always)]
fn f_8020f8d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f8d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f8d4, pc); }
        match pc {
            0x8020f8d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x3;
                c.r[6] = c.r[7] | c.r[7];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020f900; f_80208750(c);
                pc = 0x8020f900;
            }
            0x8020f900 => {
                c.r[3] = 0x11;
                c.lr = 0x8020f908; f_801f0234(c);
                pc = 0x8020f908;
            }
            0x8020f908 => {
                c.lr = 0x8020f90c; f_801f0204(c);
                pc = 0x8020f90c;
            }
            0x8020f90c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x8020f924; f_802085c4(c);
                pc = 0x8020f924;
            }
            0x8020f924 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f8d4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncNomikomu (0x8020f938, 100 bytes)
#[inline(never)]
pub fn f_8020f938(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f938) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f938); f_8020f938_b(c, 0x8020f938); crate::prof::exit(c, 0x8020f938); return; }
    }
    f_8020f938_b(c, 0x8020f938)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f938_at(c: &mut Cpu, pc: u32) { f_8020f938_b(c, pc) }
#[inline(always)]
fn f_8020f938_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f938;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f938, pc); }
        match pc {
            0x8020f938 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x3;
                c.r[6] = c.r[7] | c.r[7];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020f964; f_80208750(c);
                pc = 0x8020f964;
            }
            0x8020f964 => {
                c.r[3] = 0x11;
                c.lr = 0x8020f96c; f_801f0234(c);
                pc = 0x8020f96c;
            }
            0x8020f96c => {
                c.lr = 0x8020f970; f_801f0204(c);
                pc = 0x8020f970;
            }
            0x8020f970 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x8020f988; f_802085c4(c);
                pc = 0x8020f988;
            }
            0x8020f988 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f938: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncNomikomu (0x8020f99c, 64 bytes)
#[inline(never)]
pub fn f_8020f99c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f99c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f99c); f_8020f99c_b(c, 0x8020f99c); crate::prof::exit(c, 0x8020f99c); return; }
    }
    f_8020f99c_b(c, 0x8020f99c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f99c_at(c: &mut Cpu, pc: u32) { f_8020f99c_b(c, pc) }
#[inline(always)]
fn f_8020f99c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f99c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f99c, pc); }
        match pc {
            0x8020f99c => {
                c.r[3] = 0x80380000;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x6002));
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x8020f9cc; continue; }
                pc = 0x8020f9b4;
            }
            0x8020f9b4 => {
                if !c.crbit(0) { pc = 0x8020f9d4; continue; }
                pc = 0x8020f9b8;
            }
            0x8020f9b8 => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(0) { pc = 0x8020f9c4; continue; }
                pc = 0x8020f9c0;
            }
            0x8020f9c0 => {
                pc = 0x8020f9d4; continue;
                pc = 0x8020f9c4;
            }
            0x8020f9c4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020f9cc;
            }
            0x8020f9cc => {
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x8020f9d4;
            }
            0x8020f9d4 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020f99c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncHakidasu (0x8020f9dc, 164 bytes)
#[inline(never)]
pub fn f_8020f9dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020f9dc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020f9dc); f_8020f9dc_b(c, 0x8020f9dc); crate::prof::exit(c, 0x8020f9dc); return; }
    }
    f_8020f9dc_b(c, 0x8020f9dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020f9dc_at(c: &mut Cpu, pc: u32) { f_8020f9dc_b(c, pc) }
#[inline(always)]
fn f_8020f9dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020f9dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020f9dc, pc); }
        match pc {
            0x8020f9dc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[7] | c.r[7];
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.lr = 0x8020fa10; f_80208750(c);
                pc = 0x8020fa10;
            }
            0x8020fa10 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020fa24; f_80208750(c);
                pc = 0x8020fa24;
            }
            0x8020fa24 => {
                c.r[3] = 0x11;
                c.lr = 0x8020fa2c; f_801f0234(c);
                pc = 0x8020fa2c;
            }
            0x8020fa2c => {
                c.lr = 0x8020fa30; f_801f0204(c);
                pc = 0x8020fa30;
            }
            0x8020fa30 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020fa48; f_802085c4(c);
                pc = 0x8020fa48;
            }
            0x8020fa48 => {
                c.r[3] = 0x12;
                c.lr = 0x8020fa50; f_801f0234(c);
                pc = 0x8020fa50;
            }
            0x8020fa50 => {
                c.lr = 0x8020fa54; f_801f0204(c);
                pc = 0x8020fa54;
            }
            0x8020fa54 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020fa6c; f_802085c4(c);
                pc = 0x8020fa6c;
            }
            0x8020fa6c => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020f9dc: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncHakidasu (0x8020fa80, 64 bytes)
#[inline(never)]
pub fn f_8020fa80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020fa80) { return; }
        if c.prof { crate::prof::enter(c, 0x8020fa80); f_8020fa80_b(c, 0x8020fa80); crate::prof::exit(c, 0x8020fa80); return; }
    }
    f_8020fa80_b(c, 0x8020fa80)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020fa80_at(c: &mut Cpu, pc: u32) { f_8020fa80_b(c, pc) }
#[inline(always)]
fn f_8020fa80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020fa80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020fa80, pc); }
        match pc {
            0x8020fa80 => {
                c.r[3] = 0x80380000;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x6002));
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x8020fab0; continue; }
                pc = 0x8020fa98;
            }
            0x8020fa98 => {
                if !c.crbit(0) { pc = 0x8020fab8; continue; }
                pc = 0x8020fa9c;
            }
            0x8020fa9c => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(0) { pc = 0x8020faa8; continue; }
                pc = 0x8020faa4;
            }
            0x8020faa4 => {
                pc = 0x8020fab8; continue;
                pc = 0x8020faa8;
            }
            0x8020faa8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020fab0;
            }
            0x8020fab0 => {
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x8020fab8;
            }
            0x8020fab8 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020fa80: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncAtt (0x8020fac0, 112 bytes)
#[inline(never)]
pub fn f_8020fac0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020fac0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020fac0); f_8020fac0_b(c, 0x8020fac0); crate::prof::exit(c, 0x8020fac0); return; }
    }
    f_8020fac0_b(c, 0x8020fac0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020fac0_at(c: &mut Cpu, pc: u32) { f_8020fac0_b(c, pc) }
#[inline(always)]
fn f_8020fac0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020fac0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020fac0, pc); }
        match pc {
            0x8020fac0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[6] | c.r[6];
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[7] | c.r[7];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8020faf0; f_80208750(c);
                pc = 0x8020faf0;
            }
            0x8020faf0 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020fb1c; continue; }
                pc = 0x8020faf8;
            }
            0x8020faf8 => {
                c.r[3] = 0x11;
                c.lr = 0x8020fb00; f_801f0234(c);
                pc = 0x8020fb00;
            }
            0x8020fb00 => {
                c.lr = 0x8020fb04; f_801f0204(c);
                pc = 0x8020fb04;
            }
            0x8020fb04 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x8020fb1c; f_802085c4(c);
                pc = 0x8020fb1c;
            }
            0x8020fb1c => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020fac0: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncAtt (0x8020fb30, 8 bytes)
#[inline(never)]
pub fn f_8020fb30(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020fb30) { return; }
        if c.prof { crate::prof::enter(c, 0x8020fb30); f_8020fb30_b(c, 0x8020fb30); crate::prof::exit(c, 0x8020fb30); return; }
    }
    f_8020fb30_b(c, 0x8020fb30)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020fb30_at(c: &mut Cpu, pc: u32) { f_8020fb30_b(c, pc) }
#[inline(always)]
fn f_8020fb30_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020fb30;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020fb30, pc); }
        match pc {
            0x8020fb30 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020fb30: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncItamiwake (0x8020fb38, 196 bytes)
#[inline(never)]
pub fn f_8020fb38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020fb38) { return; }
        if c.prof { crate::prof::enter(c, 0x8020fb38); f_8020fb38_b(c, 0x8020fb38); crate::prof::exit(c, 0x8020fb38); return; }
    }
    f_8020fb38_b(c, 0x8020fb38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020fb38_at(c: &mut Cpu, pc: u32) { f_8020fb38_b(c, pc) }
#[inline(always)]
fn f_8020fb38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020fb38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020fb38, pc); }
        match pc {
            0x8020fb38 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[7] | c.r[7];
                if c.crbit(2) { pc = 0x8020fb9c; continue; }
                pc = 0x8020fb60;
            }
            0x8020fb60 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020fb74; f_80208750(c);
                pc = 0x8020fb74;
            }
            0x8020fb74 => {
                c.r[3] = 0x12;
                c.lr = 0x8020fb7c; f_801f0234(c);
                pc = 0x8020fb7c;
            }
            0x8020fb7c => {
                c.lr = 0x8020fb80; f_801f0204(c);
                pc = 0x8020fb80;
            }
            0x8020fb80 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020fb98; f_802085c4(c);
                pc = 0x8020fb98;
            }
            0x8020fb98 => {
                pc = 0x8020fbe8; continue;
                pc = 0x8020fb9c;
            }
            0x8020fb9c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.lr = 0x8020fbb0; f_80208750(c);
                pc = 0x8020fbb0;
            }
            0x8020fbb0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020fbc4; f_80208750(c);
                pc = 0x8020fbc4;
            }
            0x8020fbc4 => {
                c.r[3] = 0x11;
                c.lr = 0x8020fbcc; f_801f0234(c);
                pc = 0x8020fbcc;
            }
            0x8020fbcc => {
                c.lr = 0x8020fbd0; f_801f0204(c);
                pc = 0x8020fbd0;
            }
            0x8020fbd0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x8020fbe8; f_802085c4(c);
                pc = 0x8020fbe8;
            }
            0x8020fbe8 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020fb38: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncItamiwake (0x8020fbfc, 8 bytes)
#[inline(never)]
pub fn f_8020fbfc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020fbfc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020fbfc); f_8020fbfc_b(c, 0x8020fbfc); crate::prof::exit(c, 0x8020fbfc); return; }
    }
    f_8020fbfc_b(c, 0x8020fbfc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020fbfc_at(c: &mut Cpu, pc: u32) { f_8020fbfc_b(c, pc) }
#[inline(always)]
fn f_8020fbfc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020fbfc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020fbfc, pc); }
        match pc {
            0x8020fbfc => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020fbfc: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncWeatherHP (0x8020fc04, 108 bytes)
#[inline(never)]
pub fn f_8020fc04(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020fc04) { return; }
        if c.prof { crate::prof::enter(c, 0x8020fc04); f_8020fc04_b(c, 0x8020fc04); crate::prof::exit(c, 0x8020fc04); return; }
    }
    f_8020fc04_b(c, 0x8020fc04)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020fc04_at(c: &mut Cpu, pc: u32) { f_8020fc04_b(c, pc) }
#[inline(always)]
fn f_8020fc04_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020fc04;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020fc04, pc); }
        match pc {
            0x8020fc04 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8020fc1c; f_801f453c(c);
                pc = 0x8020fc1c;
            }
            0x8020fc1c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8020fc44; continue; }
                pc = 0x8020fc28;
            }
            0x8020fc28 => {
                if !c.crbit(0) { pc = 0x8020fc38; continue; }
                pc = 0x8020fc2c;
            }
            0x8020fc2c => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x8020fc4c; continue; }
                pc = 0x8020fc34;
            }
            0x8020fc34 => {
                pc = 0x8020fc5c; continue;
                pc = 0x8020fc38;
            }
            0x8020fc38 => {
                c.cmps(0, c.r[0], 0x5);
                if !c.crbit(0) { pc = 0x8020fc5c; continue; }
                pc = 0x8020fc40;
            }
            0x8020fc40 => {
                pc = 0x8020fc54; continue;
                pc = 0x8020fc44;
            }
            0x8020fc44 => {
                c.r[3] = 0x0;
                pc = 0x8020fc60; continue;
                pc = 0x8020fc4c;
            }
            0x8020fc4c => {
                c.r[3] = 0x1;
                pc = 0x8020fc60; continue;
                pc = 0x8020fc54;
            }
            0x8020fc54 => {
                c.r[3] = 0x2;
                pc = 0x8020fc60; continue;
                pc = 0x8020fc5c;
            }
            0x8020fc5c => {
                c.r[3] = 0x0;
                pc = 0x8020fc60;
            }
            0x8020fc60 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020fc04: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncMagnitude (0x8020fc70, 284 bytes)
#[inline(never)]
pub fn f_8020fc70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020fc70) { return; }
        if c.prof { crate::prof::enter(c, 0x8020fc70); f_8020fc70_b(c, 0x8020fc70); crate::prof::exit(c, 0x8020fc70); return; }
    }
    f_8020fc70_b(c, 0x8020fc70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020fc70_at(c: &mut Cpu, pc: u32) { f_8020fc70_b(c, pc) }
#[inline(always)]
fn f_8020fc70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020fc70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020fc70, pc); }
        match pc {
            0x8020fc70 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = c.r[7] | c.r[7];
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x8020fca8; f_8011beb4(c);
                pc = 0x8020fca8;
            }
            0x8020fca8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x8020fcc4; continue; }
                pc = 0x8020fcb4;
            }
            0x8020fcb4 => {
                c.cmpu(0, c.r[0], 0x6);
                if c.crbit(2) { pc = 0x8020fcc4; continue; }
                pc = 0x8020fcbc;
            }
            0x8020fcbc => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020fd08; continue; }
                pc = 0x8020fcc4;
            }
            0x8020fcc4 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020fd08; continue; }
                pc = 0x8020fccc;
            }
            0x8020fccc => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020fce0; f_80208750(c);
                pc = 0x8020fce0;
            }
            0x8020fce0 => {
                c.r[3] = 0x12;
                c.lr = 0x8020fce8; f_801f0234(c);
                pc = 0x8020fce8;
            }
            0x8020fce8 => {
                c.lr = 0x8020fcec; f_801f0204(c);
                pc = 0x8020fcec;
            }
            0x8020fcec => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020fd04; f_802085c4(c);
                pc = 0x8020fd04;
            }
            0x8020fd04 => {
                pc = 0x8020fd78; continue;
                pc = 0x8020fd08;
            }
            0x8020fd08 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x8020fd1c; f_80208750(c);
                pc = 0x8020fd1c;
            }
            0x8020fd1c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020fd30; f_80208750(c);
                pc = 0x8020fd30;
            }
            0x8020fd30 => {
                c.r[3] = 0x11;
                c.lr = 0x8020fd38; f_801f0234(c);
                pc = 0x8020fd38;
            }
            0x8020fd38 => {
                c.lr = 0x8020fd3c; f_801f0204(c);
                pc = 0x8020fd3c;
            }
            0x8020fd3c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020fd54; f_802085c4(c);
                pc = 0x8020fd54;
            }
            0x8020fd54 => {
                c.r[3] = 0x12;
                c.lr = 0x8020fd5c; f_801f0234(c);
                pc = 0x8020fd5c;
            }
            0x8020fd5c => {
                c.lr = 0x8020fd60; f_801f0204(c);
                pc = 0x8020fd60;
            }
            0x8020fd60 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020fd78; f_802085c4(c);
                pc = 0x8020fd78;
            }
            0x8020fd78 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020fc70: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncMagnitude (0x8020fd8c, 180 bytes)
#[inline(never)]
pub fn f_8020fd8c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020fd8c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020fd8c); f_8020fd8c_b(c, 0x8020fd8c); crate::prof::exit(c, 0x8020fd8c); return; }
    }
    f_8020fd8c_b(c, 0x8020fd8c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020fd8c_at(c: &mut Cpu, pc: u32) { f_8020fd8c_b(c, pc) }
#[inline(always)]
fn f_8020fd8c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020fd8c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020fd8c, pc); }
        match pc {
            0x8020fd8c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8020fdac; f_8012640c(c);
                pc = 0x8020fdac;
            }
            0x8020fdac => {
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x8020fdbc; f_8011beb4(c);
                pc = 0x8020fdbc;
            }
            0x8020fdbc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[0], 0x46);
                if c.crbit(2) { pc = 0x8020fe1c; continue; }
                pc = 0x8020fdc8;
            }
            0x8020fdc8 => {
                if !c.crbit(0) { pc = 0x8020fdf0; continue; }
                pc = 0x8020fdcc;
            }
            0x8020fdcc => {
                c.cmps(0, c.r[0], 0x1e);
                if c.crbit(2) { pc = 0x8020fe14; continue; }
                pc = 0x8020fdd4;
            }
            0x8020fdd4 => {
                if !c.crbit(0) { pc = 0x8020fde4; continue; }
                pc = 0x8020fdd8;
            }
            0x8020fdd8 => {
                c.cmps(0, c.r[0], 0xa);
                if c.crbit(2) { pc = 0x8020fe14; continue; }
                pc = 0x8020fde0;
            }
            0x8020fde0 => {
                pc = 0x8020fe2c; continue;
                pc = 0x8020fde4;
            }
            0x8020fde4 => {
                c.cmps(0, c.r[0], 0x32);
                if c.crbit(2) { pc = 0x8020fe1c; continue; }
                pc = 0x8020fdec;
            }
            0x8020fdec => {
                pc = 0x8020fe2c; continue;
                pc = 0x8020fdf0;
            }
            0x8020fdf0 => {
                c.cmps(0, c.r[0], 0x6e);
                if c.crbit(2) { pc = 0x8020fe24; continue; }
                pc = 0x8020fdf8;
            }
            0x8020fdf8 => {
                if !c.crbit(0) { pc = 0x8020fe08; continue; }
                pc = 0x8020fdfc;
            }
            0x8020fdfc => {
                c.cmps(0, c.r[0], 0x5a);
                if c.crbit(2) { pc = 0x8020fe1c; continue; }
                pc = 0x8020fe04;
            }
            0x8020fe04 => {
                pc = 0x8020fe2c; continue;
                pc = 0x8020fe08;
            }
            0x8020fe08 => {
                c.cmps(0, c.r[0], 0x96);
                if c.crbit(2) { pc = 0x8020fe24; continue; }
                pc = 0x8020fe10;
            }
            0x8020fe10 => {
                pc = 0x8020fe2c; continue;
                pc = 0x8020fe14;
            }
            0x8020fe14 => {
                c.r[3] = 0x0;
                pc = 0x8020fe30; continue;
                pc = 0x8020fe1c;
            }
            0x8020fe1c => {
                c.r[3] = 0x1;
                pc = 0x8020fe30; continue;
                pc = 0x8020fe24;
            }
            0x8020fe24 => {
                c.r[3] = 0x2;
                pc = 0x8020fe30; continue;
                pc = 0x8020fe2c;
            }
            0x8020fe2c => {
                c.r[3] = 0x0;
                pc = 0x8020fe30;
            }
            0x8020fe30 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020fd8c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncYatuatari (0x8020fe40, 164 bytes)
#[inline(never)]
pub fn f_8020fe40(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020fe40) { return; }
        if c.prof { crate::prof::enter(c, 0x8020fe40); f_8020fe40_b(c, 0x8020fe40); crate::prof::exit(c, 0x8020fe40); return; }
    }
    f_8020fe40_b(c, 0x8020fe40)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020fe40_at(c: &mut Cpu, pc: u32) { f_8020fe40_b(c, pc) }
#[inline(always)]
fn f_8020fe40_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020fe40;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020fe40, pc); }
        match pc {
            0x8020fe40 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = c.r[7] | c.r[7];
                c.r[5] = 0x1;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8020fe74; f_80208750(c);
                pc = 0x8020fe74;
            }
            0x8020fe74 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020fe88; f_80208750(c);
                pc = 0x8020fe88;
            }
            0x8020fe88 => {
                c.r[3] = 0x11;
                c.lr = 0x8020fe90; f_801f0234(c);
                pc = 0x8020fe90;
            }
            0x8020fe90 => {
                c.lr = 0x8020fe94; f_801f0204(c);
                pc = 0x8020fe94;
            }
            0x8020fe94 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020feac; f_802085c4(c);
                pc = 0x8020feac;
            }
            0x8020feac => {
                c.r[3] = 0x12;
                c.lr = 0x8020feb4; f_801f0234(c);
                pc = 0x8020feb4;
            }
            0x8020feb4 => {
                c.lr = 0x8020feb8; f_801f0204(c);
                pc = 0x8020feb8;
            }
            0x8020feb8 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020fed0; f_802085c4(c);
                pc = 0x8020fed0;
            }
            0x8020fed0 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020fe40: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncYatuatari (0x8020fee4, 120 bytes)
#[inline(never)]
pub fn f_8020fee4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020fee4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020fee4); f_8020fee4_b(c, 0x8020fee4); crate::prof::exit(c, 0x8020fee4); return; }
    }
    f_8020fee4_b(c, 0x8020fee4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020fee4_at(c: &mut Cpu, pc: u32) { f_8020fee4_b(c, pc) }
#[inline(always)]
fn f_8020fee4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020fee4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020fee4, pc); }
        match pc {
            0x8020fee4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8020ff04; f_8012640c(c);
                pc = 0x8020ff04;
            }
            0x8020ff04 => {
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x8020ff14; f_8011beb4(c);
                pc = 0x8020ff14;
            }
            0x8020ff14 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[4], 0x59);
                if !c.crbit(1) { pc = 0x8020ff28; continue; }
                pc = 0x8020ff20;
            }
            0x8020ff20 => {
                c.r[3] = 0x3;
                pc = 0x8020ff4c; continue;
                pc = 0x8020ff28;
            }
            0x8020ff28 => {
                c.cmpu(0, c.r[4], 0x3e);
                if c.crbit(0) { pc = 0x8020ff38; continue; }
                pc = 0x8020ff30;
            }
            0x8020ff30 => {
                c.r[3] = 0x2;
                pc = 0x8020ff4c; continue;
                pc = 0x8020ff38;
            }
            0x8020ff38 => {
                c.r[0] = 0x16;
                c.r[3] = 0xffffffff;
                c.r[0] = c.addc(!c.r[0], c.r[4], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[3], 0, ci); }
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x8020ff4c;
            }
            0x8020ff4c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020fee4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncPresent (0x8020ff5c, 164 bytes)
#[inline(never)]
pub fn f_8020ff5c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ff5c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ff5c); f_8020ff5c_b(c, 0x8020ff5c); crate::prof::exit(c, 0x8020ff5c); return; }
    }
    f_8020ff5c_b(c, 0x8020ff5c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ff5c_at(c: &mut Cpu, pc: u32) { f_8020ff5c_b(c, pc) }
#[inline(always)]
fn f_8020ff5c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ff5c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ff5c, pc); }
        match pc {
            0x8020ff5c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = c.r[7] | c.r[7];
                c.r[5] = 0x1;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8020ff90; f_80208750(c);
                pc = 0x8020ff90;
            }
            0x8020ff90 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8020ffa4; f_80208750(c);
                pc = 0x8020ffa4;
            }
            0x8020ffa4 => {
                c.r[3] = 0x11;
                c.lr = 0x8020ffac; f_801f0234(c);
                pc = 0x8020ffac;
            }
            0x8020ffac => {
                c.lr = 0x8020ffb0; f_801f0204(c);
                pc = 0x8020ffb0;
            }
            0x8020ffb0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8020ffc8; f_802085c4(c);
                pc = 0x8020ffc8;
            }
            0x8020ffc8 => {
                c.r[3] = 0x12;
                c.lr = 0x8020ffd0; f_801f0234(c);
                pc = 0x8020ffd0;
            }
            0x8020ffd0 => {
                c.lr = 0x8020ffd4; f_801f0204(c);
                pc = 0x8020ffd4;
            }
            0x8020ffd4 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020ffec; f_802085c4(c);
                pc = 0x8020ffec;
            }
            0x8020ffec => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020ff5c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncPresent (0x80210000, 116 bytes)
#[inline(never)]
pub fn f_80210000(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210000) { return; }
        if c.prof { crate::prof::enter(c, 0x80210000); f_80210000_b(c, 0x80210000); crate::prof::exit(c, 0x80210000); return; }
    }
    f_80210000_b(c, 0x80210000)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210000_at(c: &mut Cpu, pc: u32) { f_80210000_b(c, pc) }
#[inline(always)]
fn f_80210000_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210000;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210000, pc); }
        match pc {
            0x80210000 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80210020; f_8012640c(c);
                pc = 0x80210020;
            }
            0x80210020 => {
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80210030; f_8011beb4(c);
                pc = 0x80210030;
            }
            0x80210030 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[0], 0x50);
                if c.crbit(2) { pc = 0x80210058; continue; }
                pc = 0x8021003c;
            }
            0x8021003c => {
                if !c.crbit(0) { pc = 0x8021004c; continue; }
                pc = 0x80210040;
            }
            0x80210040 => {
                c.cmps(0, c.r[0], 0x28);
                if c.crbit(2) { pc = 0x80210058; continue; }
                pc = 0x80210048;
            }
            0x80210048 => {
                pc = 0x80210060; continue;
                pc = 0x8021004c;
            }
            0x8021004c => {
                c.cmps(0, c.r[0], 0x78);
                if c.crbit(2) { pc = 0x80210058; continue; }
                pc = 0x80210054;
            }
            0x80210054 => {
                pc = 0x80210060; continue;
                pc = 0x80210058;
            }
            0x80210058 => {
                c.r[3] = 0x0;
                pc = 0x80210064; continue;
                pc = 0x80210060;
            }
            0x80210060 => {
                c.r[3] = 0x1;
                pc = 0x80210064;
            }
            0x80210064 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80210000: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncOngaesi (0x80210074, 164 bytes)
#[inline(never)]
pub fn f_80210074(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210074) { return; }
        if c.prof { crate::prof::enter(c, 0x80210074); f_80210074_b(c, 0x80210074); crate::prof::exit(c, 0x80210074); return; }
    }
    f_80210074_b(c, 0x80210074)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210074_at(c: &mut Cpu, pc: u32) { f_80210074_b(c, pc) }
#[inline(always)]
fn f_80210074_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210074;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210074, pc); }
        match pc {
            0x80210074 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = c.r[7] | c.r[7];
                c.r[5] = 0x1;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802100a8; f_80208750(c);
                pc = 0x802100a8;
            }
            0x802100a8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x802100bc; f_80208750(c);
                pc = 0x802100bc;
            }
            0x802100bc => {
                c.r[3] = 0x11;
                c.lr = 0x802100c4; f_801f0234(c);
                pc = 0x802100c4;
            }
            0x802100c4 => {
                c.lr = 0x802100c8; f_801f0204(c);
                pc = 0x802100c8;
            }
            0x802100c8 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x802100e0; f_802085c4(c);
                pc = 0x802100e0;
            }
            0x802100e0 => {
                c.r[3] = 0x12;
                c.lr = 0x802100e8; f_801f0234(c);
                pc = 0x802100e8;
            }
            0x802100e8 => {
                c.lr = 0x802100ec; f_801f0204(c);
                pc = 0x802100ec;
            }
            0x802100ec => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210104; f_802085c4(c);
                pc = 0x80210104;
            }
            0x80210104 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80210074: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncOngaesi (0x80210118, 120 bytes)
#[inline(never)]
pub fn f_80210118(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210118) { return; }
        if c.prof { crate::prof::enter(c, 0x80210118); f_80210118_b(c, 0x80210118); crate::prof::exit(c, 0x80210118); return; }
    }
    f_80210118_b(c, 0x80210118)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210118_at(c: &mut Cpu, pc: u32) { f_80210118_b(c, pc) }
#[inline(always)]
fn f_80210118_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210118;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210118, pc); }
        match pc {
            0x80210118 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80210138; f_8012640c(c);
                pc = 0x80210138;
            }
            0x80210138 => {
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80210148; f_8011beb4(c);
                pc = 0x80210148;
            }
            0x80210148 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], 0x18);
                if c.crbit(1) { pc = 0x8021015c; continue; }
                pc = 0x80210154;
            }
            0x80210154 => {
                c.r[3] = 0x0;
                pc = 0x80210180; continue;
                pc = 0x8021015c;
            }
            0x8021015c => {
                c.cmpu(0, c.r[3], 0x24);
                if c.crbit(1) { pc = 0x8021016c; continue; }
                pc = 0x80210164;
            }
            0x80210164 => {
                c.r[3] = 0x1;
                pc = 0x80210180; continue;
                pc = 0x8021016c;
            }
            0x8021016c => {
                c.r[0] = 0x50;
                c.r[0] = c.addc(!c.r[3], c.r[0], 1);
                { let ci = c.ca(); c.r[0] = c.addc(c.r[3], 0, ci); }
                c.r[3] = c.r[3].wrapping_sub(c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x3);
                pc = 0x80210180;
            }
            0x80210180 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80210118: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncRenzokugiri (0x80210190, 164 bytes)
#[inline(never)]
pub fn f_80210190(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210190) { return; }
        if c.prof { crate::prof::enter(c, 0x80210190); f_80210190_b(c, 0x80210190); crate::prof::exit(c, 0x80210190); return; }
    }
    f_80210190_b(c, 0x80210190)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210190_at(c: &mut Cpu, pc: u32) { f_80210190_b(c, pc) }
#[inline(always)]
fn f_80210190_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210190;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210190, pc); }
        match pc {
            0x80210190 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[7] | c.r[7];
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.lr = 0x802101c4; f_80208750(c);
                pc = 0x802101c4;
            }
            0x802101c4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x802101d8; f_80208750(c);
                pc = 0x802101d8;
            }
            0x802101d8 => {
                c.r[3] = 0x11;
                c.lr = 0x802101e0; f_801f0234(c);
                pc = 0x802101e0;
            }
            0x802101e0 => {
                c.lr = 0x802101e4; f_801f0204(c);
                pc = 0x802101e4;
            }
            0x802101e4 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x802101fc; f_802085c4(c);
                pc = 0x802101fc;
            }
            0x802101fc => {
                c.r[3] = 0x12;
                c.lr = 0x80210204; f_801f0234(c);
                pc = 0x80210204;
            }
            0x80210204 => {
                c.lr = 0x80210208; f_801f0204(c);
                pc = 0x80210208;
            }
            0x80210208 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210220; f_802085c4(c);
                pc = 0x80210220;
            }
            0x80210220 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80210190: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncRenzokugiri (0x80210234, 132 bytes)
#[inline(never)]
pub fn f_80210234(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210234) { return; }
        if c.prof { crate::prof::enter(c, 0x80210234); f_80210234_b(c, 0x80210234); crate::prof::exit(c, 0x80210234); return; }
    }
    f_80210234_b(c, 0x80210234)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210234_at(c: &mut Cpu, pc: u32) { f_80210234_b(c, pc) }
#[inline(always)]
fn f_80210234_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210234;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210234, pc); }
        match pc {
            0x80210234 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x2e;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8021024c; f_80202360(c);
                pc = 0x8021024c;
            }
            0x8021024c => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x8021028c; continue; }
                pc = 0x80210258;
            }
            0x80210258 => {
                if !c.crbit(0) { pc = 0x8021026c; continue; }
                pc = 0x8021025c;
            }
            0x8021025c => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8021027c; continue; }
                pc = 0x80210264;
            }
            0x80210264 => {
                if !c.crbit(0) { pc = 0x80210284; continue; }
                pc = 0x80210268;
            }
            0x80210268 => {
                pc = 0x802102a4; continue;
                pc = 0x8021026c;
            }
            0x8021026c => {
                c.cmps(0, c.r[0], 0x5);
                if c.crbit(2) { pc = 0x8021029c; continue; }
                pc = 0x80210274;
            }
            0x80210274 => {
                if !c.crbit(0) { pc = 0x802102a4; continue; }
                pc = 0x80210278;
            }
            0x80210278 => {
                pc = 0x80210294; continue;
                pc = 0x8021027c;
            }
            0x8021027c => {
                c.r[3] = 0x0;
                pc = 0x802102a8; continue;
                pc = 0x80210284;
            }
            0x80210284 => {
                c.r[3] = 0x1;
                pc = 0x802102a8; continue;
                pc = 0x8021028c;
            }
            0x8021028c => {
                c.r[3] = 0x2;
                pc = 0x802102a8; continue;
                pc = 0x80210294;
            }
            0x80210294 => {
                c.r[3] = 0x3;
                pc = 0x802102a8; continue;
                pc = 0x8021029c;
            }
            0x8021029c => {
                c.r[3] = 0x4;
                pc = 0x802102a8; continue;
                pc = 0x802102a4;
            }
            0x802102a4 => {
                c.r[3] = 0x0;
                pc = 0x802102a8;
            }
            0x802102a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80210234: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncKorogaru (0x802102b8, 164 bytes)
#[inline(never)]
pub fn f_802102b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802102b8) { return; }
        if c.prof { crate::prof::enter(c, 0x802102b8); f_802102b8_b(c, 0x802102b8); crate::prof::exit(c, 0x802102b8); return; }
    }
    f_802102b8_b(c, 0x802102b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802102b8_at(c: &mut Cpu, pc: u32) { f_802102b8_b(c, pc) }
#[inline(always)]
fn f_802102b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802102b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802102b8, pc); }
        match pc {
            0x802102b8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = c.r[7] | c.r[7];
                c.r[5] = 0x1;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802102ec; f_80208750(c);
                pc = 0x802102ec;
            }
            0x802102ec => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x80210300; f_80208750(c);
                pc = 0x80210300;
            }
            0x80210300 => {
                c.r[3] = 0x11;
                c.lr = 0x80210308; f_801f0234(c);
                pc = 0x80210308;
            }
            0x80210308 => {
                c.lr = 0x8021030c; f_801f0204(c);
                pc = 0x8021030c;
            }
            0x8021030c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x80210324; f_802085c4(c);
                pc = 0x80210324;
            }
            0x80210324 => {
                c.r[3] = 0x12;
                c.lr = 0x8021032c; f_801f0234(c);
                pc = 0x8021032c;
            }
            0x8021032c => {
                c.lr = 0x80210330; f_801f0204(c);
                pc = 0x80210330;
            }
            0x80210330 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210348; f_802085c4(c);
                pc = 0x80210348;
            }
            0x80210348 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802102b8: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncKorogaru (0x8021035c, 140 bytes)
#[inline(never)]
pub fn f_8021035c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8021035c) { return; }
        if c.prof { crate::prof::enter(c, 0x8021035c); f_8021035c_b(c, 0x8021035c); crate::prof::exit(c, 0x8021035c); return; }
    }
    f_8021035c_b(c, 0x8021035c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8021035c_at(c: &mut Cpu, pc: u32) { f_8021035c_b(c, pc) }
#[inline(always)]
fn f_8021035c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8021035c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8021035c, pc); }
        match pc {
            0x8021035c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x2f;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80210374; f_80202360(c);
                pc = 0x80210374;
            }
            0x80210374 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x802103b4; continue; }
                pc = 0x80210380;
            }
            0x80210380 => {
                if !c.crbit(0) { pc = 0x8021039c; continue; }
                pc = 0x80210384;
            }
            0x80210384 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x802103d4; continue; }
                pc = 0x8021038c;
            }
            0x8021038c => {
                if !c.crbit(0) { pc = 0x802103ac; continue; }
                pc = 0x80210390;
            }
            0x80210390 => {
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(0) { pc = 0x802103cc; continue; }
                pc = 0x80210398;
            }
            0x80210398 => {
                pc = 0x802103d4; continue;
                pc = 0x8021039c;
            }
            0x8021039c => {
                c.cmps(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x802103c4; continue; }
                pc = 0x802103a4;
            }
            0x802103a4 => {
                if !c.crbit(0) { pc = 0x802103d4; continue; }
                pc = 0x802103a8;
            }
            0x802103a8 => {
                pc = 0x802103bc; continue;
                pc = 0x802103ac;
            }
            0x802103ac => {
                c.r[3] = 0x0;
                pc = 0x802103d8; continue;
                pc = 0x802103b4;
            }
            0x802103b4 => {
                c.r[3] = 0x1;
                pc = 0x802103d8; continue;
                pc = 0x802103bc;
            }
            0x802103bc => {
                c.r[3] = 0x2;
                pc = 0x802103d8; continue;
                pc = 0x802103c4;
            }
            0x802103c4 => {
                c.r[3] = 0x3;
                pc = 0x802103d8; continue;
                pc = 0x802103cc;
            }
            0x802103cc => {
                c.r[3] = 0x4;
                pc = 0x802103d8; continue;
                pc = 0x802103d4;
            }
            0x802103d4 => {
                c.r[3] = 0x0;
                pc = 0x802103d8;
            }
            0x802103d8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8021035c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncTripleKick (0x802103e8, 256 bytes)
#[inline(never)]
pub fn f_802103e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802103e8) { return; }
        if c.prof { crate::prof::enter(c, 0x802103e8); f_802103e8_b(c, 0x802103e8); crate::prof::exit(c, 0x802103e8); return; }
    }
    f_802103e8_b(c, 0x802103e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802103e8_at(c: &mut Cpu, pc: u32) { f_802103e8_b(c, pc) }
#[inline(always)]
fn f_802103e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802103e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802103e8, pc); }
        match pc {
            0x802103e8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = c.r[7] | c.r[7];
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80210420; f_8011beb4(c);
                pc = 0x80210420;
            }
            0x80210420 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80210464; continue; }
                pc = 0x80210428;
            }
            0x80210428 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8021043c; f_80208750(c);
                pc = 0x8021043c;
            }
            0x8021043c => {
                c.r[3] = 0x12;
                c.lr = 0x80210444; f_801f0234(c);
                pc = 0x80210444;
            }
            0x80210444 => {
                c.lr = 0x80210448; f_801f0204(c);
                pc = 0x80210448;
            }
            0x80210448 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210460; f_802085c4(c);
                pc = 0x80210460;
            }
            0x80210460 => {
                pc = 0x802104d4; continue;
                pc = 0x80210464;
            }
            0x80210464 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.lr = 0x80210478; f_80208750(c);
                pc = 0x80210478;
            }
            0x80210478 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8021048c; f_80208750(c);
                pc = 0x8021048c;
            }
            0x8021048c => {
                c.r[3] = 0x11;
                c.lr = 0x80210494; f_801f0234(c);
                pc = 0x80210494;
            }
            0x80210494 => {
                c.lr = 0x80210498; f_801f0204(c);
                pc = 0x80210498;
            }
            0x80210498 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x802104b0; f_802085c4(c);
                pc = 0x802104b0;
            }
            0x802104b0 => {
                c.r[3] = 0x12;
                c.lr = 0x802104b8; f_801f0234(c);
                pc = 0x802104b8;
            }
            0x802104b8 => {
                c.lr = 0x802104bc; f_801f0204(c);
                pc = 0x802104bc;
            }
            0x802104bc => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x802104d4; f_802085c4(c);
                pc = 0x802104d4;
            }
            0x802104d4 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802103e8: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncTripleKick (0x802104e8, 132 bytes)
#[inline(never)]
pub fn f_802104e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802104e8) { return; }
        if c.prof { crate::prof::enter(c, 0x802104e8); f_802104e8_b(c, 0x802104e8); crate::prof::exit(c, 0x802104e8); return; }
    }
    f_802104e8_b(c, 0x802104e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802104e8_at(c: &mut Cpu, pc: u32) { f_802104e8_b(c, pc) }
#[inline(always)]
fn f_802104e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802104e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802104e8, pc); }
        match pc {
            0x802104e8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80210508; f_8012640c(c);
                pc = 0x80210508;
            }
            0x80210508 => {
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80210518; f_8011beb4(c);
                pc = 0x80210518;
            }
            0x80210518 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[0], 0x14);
                if c.crbit(2) { pc = 0x80210548; continue; }
                pc = 0x80210524;
            }
            0x80210524 => {
                if !c.crbit(0) { pc = 0x80210534; continue; }
                pc = 0x80210528;
            }
            0x80210528 => {
                c.cmps(0, c.r[0], 0xa);
                if c.crbit(2) { pc = 0x80210540; continue; }
                pc = 0x80210530;
            }
            0x80210530 => {
                pc = 0x80210558; continue;
                pc = 0x80210534;
            }
            0x80210534 => {
                c.cmps(0, c.r[0], 0x1e);
                if c.crbit(2) { pc = 0x80210550; continue; }
                pc = 0x8021053c;
            }
            0x8021053c => {
                pc = 0x80210558; continue;
                pc = 0x80210540;
            }
            0x80210540 => {
                c.r[3] = 0x0;
                pc = 0x8021055c; continue;
                pc = 0x80210548;
            }
            0x80210548 => {
                c.r[3] = 0x1;
                pc = 0x8021055c; continue;
                pc = 0x80210550;
            }
            0x80210550 => {
                c.r[3] = 0x2;
                pc = 0x8021055c; continue;
                pc = 0x80210558;
            }
            0x80210558 => {
                c.r[3] = 0x0;
                pc = 0x8021055c;
            }
            0x8021055c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802104e8: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncTikyuunage (0x8021056c, 164 bytes)
#[inline(never)]
pub fn f_8021056c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8021056c) { return; }
        if c.prof { crate::prof::enter(c, 0x8021056c); f_8021056c_b(c, 0x8021056c); crate::prof::exit(c, 0x8021056c); return; }
    }
    f_8021056c_b(c, 0x8021056c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8021056c_at(c: &mut Cpu, pc: u32) { f_8021056c_b(c, pc) }
#[inline(always)]
fn f_8021056c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8021056c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8021056c, pc); }
        match pc {
            0x8021056c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = c.r[7] | c.r[7];
                c.r[5] = 0x1;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802105a0; f_80208750(c);
                pc = 0x802105a0;
            }
            0x802105a0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x802105b4; f_80208750(c);
                pc = 0x802105b4;
            }
            0x802105b4 => {
                c.r[3] = 0x11;
                c.lr = 0x802105bc; f_801f0234(c);
                pc = 0x802105bc;
            }
            0x802105bc => {
                c.lr = 0x802105c0; f_801f0204(c);
                pc = 0x802105c0;
            }
            0x802105c0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x802105d8; f_802085c4(c);
                pc = 0x802105d8;
            }
            0x802105d8 => {
                c.r[3] = 0x12;
                c.lr = 0x802105e0; f_801f0234(c);
                pc = 0x802105e0;
            }
            0x802105e0 => {
                c.lr = 0x802105e4; f_801f0204(c);
                pc = 0x802105e4;
            }
            0x802105e4 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x802105fc; f_802085c4(c);
                pc = 0x802105fc;
            }
            0x802105fc => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8021056c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncTikyuunage (0x80210610, 72 bytes)
#[inline(never)]
pub fn f_80210610(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210610) { return; }
        if c.prof { crate::prof::enter(c, 0x80210610); f_80210610_b(c, 0x80210610); crate::prof::exit(c, 0x80210610); return; }
    }
    f_80210610_b(c, 0x80210610)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210610_at(c: &mut Cpu, pc: u32) { f_80210610_b(c, pc) }
#[inline(always)]
fn f_80210610_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210610;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210610, pc); }
        match pc {
            0x80210610 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80210624; f_80203e0c(c);
                pc = 0x80210624;
            }
            0x80210624 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[3], 0x21);
                if !c.crbit(0) { pc = 0x80210638; continue; }
                pc = 0x80210630;
            }
            0x80210630 => {
                c.r[3] = 0x0;
                pc = 0x80210648; continue;
                pc = 0x80210638;
            }
            0x80210638 => {
                c.r[0] = 0x42;
                c.r[0] = c.addc(!c.r[0], c.r[3], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[0], ci); }
                c.r[3] = c.r[3].wrapping_add(0x2);
                pc = 0x80210648;
            }
            0x80210648 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80210610: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncKetaguri (0x80210658, 164 bytes)
#[inline(never)]
pub fn f_80210658(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210658) { return; }
        if c.prof { crate::prof::enter(c, 0x80210658); f_80210658_b(c, 0x80210658); crate::prof::exit(c, 0x80210658); return; }
    }
    f_80210658_b(c, 0x80210658)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210658_at(c: &mut Cpu, pc: u32) { f_80210658_b(c, pc) }
#[inline(always)]
fn f_80210658_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210658;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210658, pc); }
        match pc {
            0x80210658 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[7] | c.r[7];
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.lr = 0x8021068c; f_80208750(c);
                pc = 0x8021068c;
            }
            0x8021068c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x802106a0; f_80208750(c);
                pc = 0x802106a0;
            }
            0x802106a0 => {
                c.r[3] = 0x11;
                c.lr = 0x802106a8; f_801f0234(c);
                pc = 0x802106a8;
            }
            0x802106a8 => {
                c.lr = 0x802106ac; f_801f0204(c);
                pc = 0x802106ac;
            }
            0x802106ac => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x802106c4; f_802085c4(c);
                pc = 0x802106c4;
            }
            0x802106c4 => {
                c.r[3] = 0x12;
                c.lr = 0x802106cc; f_801f0234(c);
                pc = 0x802106cc;
            }
            0x802106cc => {
                c.lr = 0x802106d0; f_801f0204(c);
                pc = 0x802106d0;
            }
            0x802106d0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x802106e8; f_802085c4(c);
                pc = 0x802106e8;
            }
            0x802106e8 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80210658: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncKetaguri (0x802106fc, 192 bytes)
#[inline(never)]
pub fn f_802106fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802106fc) { return; }
        if c.prof { crate::prof::enter(c, 0x802106fc); f_802106fc_b(c, 0x802106fc); crate::prof::exit(c, 0x802106fc); return; }
    }
    f_802106fc_b(c, 0x802106fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802106fc_at(c: &mut Cpu, pc: u32) { f_802106fc_b(c, pc) }
#[inline(always)]
fn f_802106fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802106fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802106fc, pc); }
        match pc {
            0x802106fc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8021071c; f_8012640c(c);
                pc = 0x8021071c;
            }
            0x8021071c => {
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x8021072c; f_8011beb4(c);
                pc = 0x8021072c;
            }
            0x8021072c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[0], 0x50);
                if c.crbit(2) { pc = 0x80210790; continue; }
                pc = 0x80210738;
            }
            0x80210738 => {
                if !c.crbit(0) { pc = 0x80210760; continue; }
                pc = 0x8021073c;
            }
            0x8021073c => {
                c.cmps(0, c.r[0], 0x28);
                if c.crbit(2) { pc = 0x80210780; continue; }
                pc = 0x80210744;
            }
            0x80210744 => {
                if !c.crbit(0) { pc = 0x80210754; continue; }
                pc = 0x80210748;
            }
            0x80210748 => {
                c.cmps(0, c.r[0], 0x14);
                if c.crbit(2) { pc = 0x80210778; continue; }
                pc = 0x80210750;
            }
            0x80210750 => {
                pc = 0x802107a8; continue;
                pc = 0x80210754;
            }
            0x80210754 => {
                c.cmps(0, c.r[0], 0x3c);
                if c.crbit(2) { pc = 0x80210788; continue; }
                pc = 0x8021075c;
            }
            0x8021075c => {
                pc = 0x802107a8; continue;
                pc = 0x80210760;
            }
            0x80210760 => {
                c.cmps(0, c.r[0], 0x78);
                if c.crbit(2) { pc = 0x802107a0; continue; }
                pc = 0x80210768;
            }
            0x80210768 => {
                if !c.crbit(0) { pc = 0x802107a8; continue; }
                pc = 0x8021076c;
            }
            0x8021076c => {
                c.cmps(0, c.r[0], 0x64);
                if c.crbit(2) { pc = 0x80210798; continue; }
                pc = 0x80210774;
            }
            0x80210774 => {
                pc = 0x802107a8; continue;
                pc = 0x80210778;
            }
            0x80210778 => {
                c.r[3] = 0x0;
                pc = 0x802107ac; continue;
                pc = 0x80210780;
            }
            0x80210780 => {
                c.r[3] = 0x1;
                pc = 0x802107ac; continue;
                pc = 0x80210788;
            }
            0x80210788 => {
                c.r[3] = 0x2;
                pc = 0x802107ac; continue;
                pc = 0x80210790;
            }
            0x80210790 => {
                c.r[3] = 0x3;
                pc = 0x802107ac; continue;
                pc = 0x80210798;
            }
            0x80210798 => {
                c.r[3] = 0x4;
                pc = 0x802107ac; continue;
                pc = 0x802107a0;
            }
            0x802107a0 => {
                c.r[3] = 0x5;
                pc = 0x802107ac; continue;
                pc = 0x802107a8;
            }
            0x802107a8 => {
                c.r[3] = 0x0;
                pc = 0x802107ac;
            }
            0x802107ac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802106fc: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncAttDef (0x802107bc, 196 bytes)
#[inline(never)]
pub fn f_802107bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802107bc) { return; }
        if c.prof { crate::prof::enter(c, 0x802107bc); f_802107bc_b(c, 0x802107bc); crate::prof::exit(c, 0x802107bc); return; }
    }
    f_802107bc_b(c, 0x802107bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802107bc_at(c: &mut Cpu, pc: u32) { f_802107bc_b(c, pc) }
#[inline(always)]
fn f_802107bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802107bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802107bc, pc); }
        match pc {
            0x802107bc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[7] | c.r[7];
                if !c.crbit(2) { pc = 0x80210834; continue; }
                pc = 0x802107e4;
            }
            0x802107e4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.lr = 0x802107f8; f_80208750(c);
                pc = 0x802107f8;
            }
            0x802107f8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x8021080c; f_80208750(c);
                pc = 0x8021080c;
            }
            0x8021080c => {
                c.r[3] = 0x11;
                c.lr = 0x80210814; f_801f0234(c);
                pc = 0x80210814;
            }
            0x80210814 => {
                c.lr = 0x80210818; f_801f0204(c);
                pc = 0x80210818;
            }
            0x80210818 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80210830; f_802085c4(c);
                pc = 0x80210830;
            }
            0x80210830 => {
                pc = 0x8021086c; continue;
                pc = 0x80210834;
            }
            0x80210834 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x80210848; f_80208750(c);
                pc = 0x80210848;
            }
            0x80210848 => {
                c.r[3] = 0x12;
                c.lr = 0x80210850; f_801f0234(c);
                pc = 0x80210850;
            }
            0x80210850 => {
                c.lr = 0x80210854; f_801f0204(c);
                pc = 0x80210854;
            }
            0x80210854 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8021086c; f_802085c4(c);
                pc = 0x8021086c;
            }
            0x8021086c => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802107bc: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncAttDef (0x80210880, 8 bytes)
#[inline(never)]
pub fn f_80210880(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210880) { return; }
        if c.prof { crate::prof::enter(c, 0x80210880); f_80210880_b(c, 0x80210880); crate::prof::exit(c, 0x80210880); return; }
    }
    f_80210880_b(c, 0x80210880)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210880_at(c: &mut Cpu, pc: u32) { f_80210880_b(c, pc) }
#[inline(always)]
fn f_80210880_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210880;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210880, pc); }
        match pc {
            0x80210880 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80210880: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncAllAlly (0x80210888, 264 bytes)
#[inline(never)]
pub fn f_80210888(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210888) { return; }
        if c.prof { crate::prof::enter(c, 0x80210888); f_80210888_b(c, 0x80210888); crate::prof::exit(c, 0x80210888); return; }
    }
    f_80210888_b(c, 0x80210888)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210888_at(c: &mut Cpu, pc: u32) { f_80210888_b(c, pc) }
#[inline(always)]
fn f_80210888_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210888;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210888, pc); }
        match pc {
            0x80210888 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[7] | c.r[7];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x802108b8; f_801f54a4(c);
                pc = 0x802108b8;
            }
            0x802108b8 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0xe;
                c.lr = 0x802108c8; f_801f02ac(c);
                pc = 0x802108c8;
            }
            0x802108c8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x3;
                c.lr = 0x802108e4; f_80208750(c);
                pc = 0x802108e4;
            }
            0x802108e4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x3;
                c.lr = 0x802108f8; f_80208750(c);
                pc = 0x802108f8;
            }
            0x802108f8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80210900; f_802062fc(c);
                pc = 0x80210900;
            }
            0x80210900 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80210958; continue; }
                pc = 0x8021090c;
            }
            0x8021090c => {
                c.r[3] = 0x11;
                c.lr = 0x80210914; f_801f0234(c);
                pc = 0x80210914;
            }
            0x80210914 => {
                c.lr = 0x80210918; f_801f0204(c);
                pc = 0x80210918;
            }
            0x80210918 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x3;
                c.r[6] = 0x1;
                c.lr = 0x80210930; f_802085c4(c);
                pc = 0x80210930;
            }
            0x80210930 => {
                c.r[3] = 0x11;
                c.lr = 0x80210938; f_801f0234(c);
                pc = 0x80210938;
            }
            0x80210938 => {
                c.lr = 0x8021093c; f_801f0204(c);
                pc = 0x8021093c;
            }
            0x8021093c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x80210954; f_802085c4(c);
                pc = 0x80210954;
            }
            0x80210954 => {
                pc = 0x8021097c; continue;
                pc = 0x80210958;
            }
            0x80210958 => {
                c.r[3] = 0x11;
                c.lr = 0x80210960; f_801f0234(c);
                pc = 0x80210960;
            }
            0x80210960 => {
                c.lr = 0x80210964; f_801f0204(c);
                pc = 0x80210964;
            }
            0x80210964 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x8021097c; f_802085c4(c);
                pc = 0x8021097c;
            }
            0x8021097c => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80210888: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncAllAlly (0x80210990, 8 bytes)
#[inline(never)]
pub fn f_80210990(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210990) { return; }
        if c.prof { crate::prof::enter(c, 0x80210990); f_80210990_b(c, 0x80210990); crate::prof::exit(c, 0x80210990); return; }
    }
    f_80210990_b(c, 0x80210990)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210990_at(c: &mut Cpu, pc: u32) { f_80210990_b(c, pc) }
#[inline(always)]
fn f_80210990_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210990;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210990, pc); }
        match pc {
            0x80210990 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80210990: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncAllEnemy (0x80210998, 360 bytes)
#[inline(never)]
pub fn f_80210998(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210998) { return; }
        if c.prof { crate::prof::enter(c, 0x80210998); f_80210998_b(c, 0x80210998); crate::prof::exit(c, 0x80210998); return; }
    }
    f_80210998_b(c, 0x80210998)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210998_at(c: &mut Cpu, pc: u32) { f_80210998_b(c, pc) }
#[inline(always)]
fn f_80210998_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210998;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210998, pc); }
        match pc {
            0x80210998 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[7] | c.r[7];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.lr = 0x802109cc; f_801f54a4(c);
                pc = 0x802109cc;
            }
            0x802109cc => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x10;
                c.lr = 0x802109dc; f_801f02ac(c);
                pc = 0x802109dc;
            }
            0x802109dc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x1;
                c.lr = 0x802109f8; f_80208750(c);
                pc = 0x802109f8;
            }
            0x802109f8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x2;
                c.lr = 0x80210a0c; f_80208750(c);
                pc = 0x80210a0c;
            }
            0x80210a0c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x2;
                c.lr = 0x80210a20; f_80208750(c);
                pc = 0x80210a20;
            }
            0x80210a20 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80210a28; f_802062fc(c);
                pc = 0x80210a28;
            }
            0x80210a28 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80210aa4; continue; }
                pc = 0x80210a34;
            }
            0x80210a34 => {
                c.r[3] = 0x11;
                c.lr = 0x80210a3c; f_801f0234(c);
                pc = 0x80210a3c;
            }
            0x80210a3c => {
                c.lr = 0x80210a40; f_801f0204(c);
                pc = 0x80210a40;
            }
            0x80210a40 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x80210a58; f_802085c4(c);
                pc = 0x80210a58;
            }
            0x80210a58 => {
                c.r[3] = 0x12;
                c.lr = 0x80210a60; f_801f0234(c);
                pc = 0x80210a60;
            }
            0x80210a60 => {
                c.lr = 0x80210a64; f_801f0204(c);
                pc = 0x80210a64;
            }
            0x80210a64 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x1;
                c.lr = 0x80210a7c; f_802085c4(c);
                pc = 0x80210a7c;
            }
            0x80210a7c => {
                c.r[3] = 0x12;
                c.lr = 0x80210a84; f_801f0234(c);
                pc = 0x80210a84;
            }
            0x80210a84 => {
                c.lr = 0x80210a88; f_801f0204(c);
                pc = 0x80210a88;
            }
            0x80210a88 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210aa0; f_802085c4(c);
                pc = 0x80210aa0;
            }
            0x80210aa0 => {
                pc = 0x80210aec; continue;
                pc = 0x80210aa4;
            }
            0x80210aa4 => {
                c.r[3] = 0x11;
                c.lr = 0x80210aac; f_801f0234(c);
                pc = 0x80210aac;
            }
            0x80210aac => {
                c.lr = 0x80210ab0; f_801f0204(c);
                pc = 0x80210ab0;
            }
            0x80210ab0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x80210ac8; f_802085c4(c);
                pc = 0x80210ac8;
            }
            0x80210ac8 => {
                c.r[3] = 0x12;
                c.lr = 0x80210ad0; f_801f0234(c);
                pc = 0x80210ad0;
            }
            0x80210ad0 => {
                c.lr = 0x80210ad4; f_801f0204(c);
                pc = 0x80210ad4;
            }
            0x80210ad4 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210aec; f_802085c4(c);
                pc = 0x80210aec;
            }
            0x80210aec => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80210998: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncAllEnemy (0x80210b00, 8 bytes)
#[inline(never)]
pub fn f_80210b00(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210b00) { return; }
        if c.prof { crate::prof::enter(c, 0x80210b00); f_80210b00_b(c, 0x80210b00); crate::prof::exit(c, 0x80210b00); return; }
    }
    f_80210b00_b(c, 0x80210b00)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210b00_at(c: &mut Cpu, pc: u32) { f_80210b00_b(c, pc) }
#[inline(always)]
fn f_80210b00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210b00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210b00, pc); }
        match pc {
            0x80210b00 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80210b00: bad pc {:#x}", pc),
        }
    }
}

/// fn_80210B08 (0x80210b08, 232 bytes)
#[inline(never)]
pub fn f_80210b08(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210b08) { return; }
        if c.prof { crate::prof::enter(c, 0x80210b08); f_80210b08_b(c, 0x80210b08); crate::prof::exit(c, 0x80210b08); return; }
    }
    f_80210b08_b(c, 0x80210b08)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210b08_at(c: &mut Cpu, pc: u32) { f_80210b08_b(c, pc) }
#[inline(always)]
fn f_80210b08_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210b08;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210b08, pc); }
        match pc {
            0x80210b08 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[7] | c.r[7];
                if c.crbit(2) { pc = 0x80210b6c; continue; }
                pc = 0x80210b30;
            }
            0x80210b30 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x80210b44; f_80208750(c);
                pc = 0x80210b44;
            }
            0x80210b44 => {
                c.r[3] = 0x12;
                c.lr = 0x80210b4c; f_801f0234(c);
                pc = 0x80210b4c;
            }
            0x80210b4c => {
                c.lr = 0x80210b50; f_801f0204(c);
                pc = 0x80210b50;
            }
            0x80210b50 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210b68; f_802085c4(c);
                pc = 0x80210b68;
            }
            0x80210b68 => {
                pc = 0x80210bdc; continue;
                pc = 0x80210b6c;
            }
            0x80210b6c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.lr = 0x80210b80; f_80208750(c);
                pc = 0x80210b80;
            }
            0x80210b80 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x80210b94; f_80208750(c);
                pc = 0x80210b94;
            }
            0x80210b94 => {
                c.r[3] = 0x11;
                c.lr = 0x80210b9c; f_801f0234(c);
                pc = 0x80210b9c;
            }
            0x80210b9c => {
                c.lr = 0x80210ba0; f_801f0204(c);
                pc = 0x80210ba0;
            }
            0x80210ba0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x80210bb8; f_802085c4(c);
                pc = 0x80210bb8;
            }
            0x80210bb8 => {
                c.r[3] = 0x12;
                c.lr = 0x80210bc0; f_801f0234(c);
                pc = 0x80210bc0;
            }
            0x80210bc0 => {
                c.lr = 0x80210bc4; f_801f0204(c);
                pc = 0x80210bc4;
            }
            0x80210bc4 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210bdc; f_802085c4(c);
                pc = 0x80210bdc;
            }
            0x80210bdc => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80210b08: bad pc {:#x}", pc),
        }
    }
}

/// fn_80210BF0 (0x80210bf0, 8 bytes)
#[inline(never)]
pub fn f_80210bf0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210bf0) { return; }
        if c.prof { crate::prof::enter(c, 0x80210bf0); f_80210bf0_b(c, 0x80210bf0); crate::prof::exit(c, 0x80210bf0); return; }
    }
    f_80210bf0_b(c, 0x80210bf0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210bf0_at(c: &mut Cpu, pc: u32) { f_80210bf0_b(c, pc) }
#[inline(always)]
fn f_80210bf0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210bf0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210bf0, pc); }
        match pc {
            0x80210bf0 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80210bf0: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncAttDefSpc (0x80210bf8, 260 bytes)
#[inline(never)]
pub fn f_80210bf8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210bf8) { return; }
        if c.prof { crate::prof::enter(c, 0x80210bf8); f_80210bf8_b(c, 0x80210bf8); crate::prof::exit(c, 0x80210bf8); return; }
    }
    f_80210bf8_b(c, 0x80210bf8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210bf8_at(c: &mut Cpu, pc: u32) { f_80210bf8_b(c, pc) }
#[inline(always)]
fn f_80210bf8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210bf8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210bf8, pc); }
        match pc {
            0x80210bf8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[7] | c.r[7];
                if !c.crbit(2) { pc = 0x80210ca8; continue; }
                pc = 0x80210c20;
            }
            0x80210c20 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.lr = 0x80210c34; f_80208750(c);
                pc = 0x80210c34;
            }
            0x80210c34 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x80210c48; f_80208750(c);
                pc = 0x80210c48;
            }
            0x80210c48 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x3;
                c.lr = 0x80210c5c; f_80208750(c);
                pc = 0x80210c5c;
            }
            0x80210c5c => {
                c.r[3] = 0x11;
                c.lr = 0x80210c64; f_801f0234(c);
                pc = 0x80210c64;
            }
            0x80210c64 => {
                c.lr = 0x80210c68; f_801f0204(c);
                pc = 0x80210c68;
            }
            0x80210c68 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x80210c80; f_802085c4(c);
                pc = 0x80210c80;
            }
            0x80210c80 => {
                c.r[3] = 0x12;
                c.lr = 0x80210c88; f_801f0234(c);
                pc = 0x80210c88;
            }
            0x80210c88 => {
                c.lr = 0x80210c8c; f_801f0204(c);
                pc = 0x80210c8c;
            }
            0x80210c8c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210ca4; f_802085c4(c);
                pc = 0x80210ca4;
            }
            0x80210ca4 => {
                pc = 0x80210ce8; continue;
                pc = 0x80210ca8;
            }
            0x80210ca8 => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80210ce8; continue; }
                pc = 0x80210cb0;
            }
            0x80210cb0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x3;
                c.lr = 0x80210cc4; f_80208750(c);
                pc = 0x80210cc4;
            }
            0x80210cc4 => {
                c.r[3] = 0x11;
                c.lr = 0x80210ccc; f_801f0234(c);
                pc = 0x80210ccc;
            }
            0x80210ccc => {
                c.lr = 0x80210cd0; f_801f0204(c);
                pc = 0x80210cd0;
            }
            0x80210cd0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x80210ce8; f_802085c4(c);
                pc = 0x80210ce8;
            }
            0x80210ce8 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80210bf8: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncAttDefSpc (0x80210cfc, 8 bytes)
#[inline(never)]
pub fn f_80210cfc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210cfc) { return; }
        if c.prof { crate::prof::enter(c, 0x80210cfc); f_80210cfc_b(c, 0x80210cfc); crate::prof::exit(c, 0x80210cfc); return; }
    }
    f_80210cfc_b(c, 0x80210cfc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210cfc_at(c: &mut Cpu, pc: u32) { f_80210cfc_b(c, pc) }
#[inline(always)]
fn f_80210cfc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210cfc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210cfc, pc); }
        match pc {
            0x80210cfc => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80210cfc: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncVampire (0x80210d04, 336 bytes)
#[inline(never)]
pub fn f_80210d04(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210d04) { return; }
        if c.prof { crate::prof::enter(c, 0x80210d04); f_80210d04_b(c, 0x80210d04); crate::prof::exit(c, 0x80210d04); return; }
    }
    f_80210d04_b(c, 0x80210d04)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210d04_at(c: &mut Cpu, pc: u32) { f_80210d04_b(c, pc) }
#[inline(always)]
fn f_80210d04_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210d04;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210d04, pc); }
        match pc {
            0x80210d04 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[4] | c.r[4];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                c.r[30] = c.r[7] | c.r[7];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80210d3c; f_8012640c(c);
                pc = 0x80210d3c;
            }
            0x80210d3c => {
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x80210d4c; f_8011beb4(c);
                pc = 0x80210d4c;
            }
            0x80210d4c => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x80210de0; continue; }
                pc = 0x80210d58;
            }
            0x80210d58 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x1;
                c.lr = 0x80210d6c; f_80208750(c);
                pc = 0x80210d6c;
            }
            0x80210d6c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x2;
                c.lr = 0x80210d80; f_80208750(c);
                pc = 0x80210d80;
            }
            0x80210d80 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x3;
                c.lr = 0x80210d94; f_80208750(c);
                pc = 0x80210d94;
            }
            0x80210d94 => {
                c.r[3] = 0x11;
                c.lr = 0x80210d9c; f_801f0234(c);
                pc = 0x80210d9c;
            }
            0x80210d9c => {
                c.lr = 0x80210da0; f_801f0204(c);
                pc = 0x80210da0;
            }
            0x80210da0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x80210db8; f_802085c4(c);
                pc = 0x80210db8;
            }
            0x80210db8 => {
                c.r[3] = 0x12;
                c.lr = 0x80210dc0; f_801f0234(c);
                pc = 0x80210dc0;
            }
            0x80210dc0 => {
                c.lr = 0x80210dc4; f_801f0204(c);
                pc = 0x80210dc4;
            }
            0x80210dc4 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210ddc; f_802085c4(c);
                pc = 0x80210ddc;
            }
            0x80210ddc => {
                pc = 0x80210e40; continue;
                pc = 0x80210de0;
            }
            0x80210de0 => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80210e40; continue; }
                pc = 0x80210de8;
            }
            0x80210de8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = 0x3;
                c.lr = 0x80210dfc; f_80208750(c);
                pc = 0x80210dfc;
            }
            0x80210dfc => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(1) { pc = 0x80210e1c; continue; }
                pc = 0x80210e04;
            }
            0x80210e04 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x32;
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x80210e18; f_802221ec(c);
                pc = 0x80210e18;
            }
            0x80210e18 => {
                pc = 0x80210e40; continue;
                pc = 0x80210e1c;
            }
            0x80210e1c => {
                c.r[3] = 0x11;
                c.lr = 0x80210e24; f_801f0234(c);
                pc = 0x80210e24;
            }
            0x80210e24 => {
                c.lr = 0x80210e28; f_801f0204(c);
                pc = 0x80210e28;
            }
            0x80210e28 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x80210e40; f_802085c4(c);
                pc = 0x80210e40;
            }
            0x80210e40 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80210d04: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncVampire (0x80210e54, 8 bytes)
#[inline(never)]
pub fn f_80210e54(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210e54) { return; }
        if c.prof { crate::prof::enter(c, 0x80210e54); f_80210e54_b(c, 0x80210e54); crate::prof::exit(c, 0x80210e54); return; }
    }
    f_80210e54_b(c, 0x80210e54)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210e54_at(c: &mut Cpu, pc: u32) { f_80210e54_b(c, pc) }
#[inline(always)]
fn f_80210e54_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210e54;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210e54, pc); }
        match pc {
            0x80210e54 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80210e54: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncSpc (0x80210e5c, 100 bytes)
#[inline(never)]
pub fn f_80210e5c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210e5c) { return; }
        if c.prof { crate::prof::enter(c, 0x80210e5c); f_80210e5c_b(c, 0x80210e5c); crate::prof::exit(c, 0x80210e5c); return; }
    }
    f_80210e5c_b(c, 0x80210e5c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210e5c_at(c: &mut Cpu, pc: u32) { f_80210e5c_b(c, pc) }
#[inline(always)]
fn f_80210e5c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210e5c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210e5c, pc); }
        match pc {
            0x80210e5c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x3;
                c.r[6] = c.r[7] | c.r[7];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80210e88; f_80208750(c);
                pc = 0x80210e88;
            }
            0x80210e88 => {
                c.r[3] = 0x11;
                c.lr = 0x80210e90; f_801f0234(c);
                pc = 0x80210e90;
            }
            0x80210e90 => {
                c.lr = 0x80210e94; f_801f0204(c);
                pc = 0x80210e94;
            }
            0x80210e94 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x80210eac; f_802085c4(c);
                pc = 0x80210eac;
            }
            0x80210eac => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80210e5c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncSpc (0x80210ec0, 8 bytes)
#[inline(never)]
pub fn f_80210ec0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210ec0) { return; }
        if c.prof { crate::prof::enter(c, 0x80210ec0); f_80210ec0_b(c, 0x80210ec0); crate::prof::exit(c, 0x80210ec0); return; }
    }
    f_80210ec0_b(c, 0x80210ec0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210ec0_at(c: &mut Cpu, pc: u32) { f_80210ec0_b(c, pc) }
#[inline(always)]
fn f_80210ec0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210ec0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210ec0, pc); }
        match pc {
            0x80210ec0 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80210ec0: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncTame (0x80210ec8, 368 bytes)
#[inline(never)]
pub fn f_80210ec8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80210ec8) { return; }
        if c.prof { crate::prof::enter(c, 0x80210ec8); f_80210ec8_b(c, 0x80210ec8); crate::prof::exit(c, 0x80210ec8); return; }
    }
    f_80210ec8_b(c, 0x80210ec8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80210ec8_at(c: &mut Cpu, pc: u32) { f_80210ec8_b(c, pc) }
#[inline(always)]
fn f_80210ec8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80210ec8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80210ec8, pc); }
        match pc {
            0x80210ec8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[8] = 0x80380000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[8].wrapping_add(0xffff9f58);
                c.r[28] = c.r[4] | c.r[4];
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = c.r[7] | c.r[7];
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x6002));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80210fec; continue; }
                pc = 0x80210f04;
            }
            0x80210f04 => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80210f18; f_8011beb4(c);
                pc = 0x80210f18;
            }
            0x80210f18 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x80210f34; continue; }
                pc = 0x80210f24;
            }
            0x80210f24 => {
                c.cmpu(0, c.r[0], 0x6);
                if c.crbit(2) { pc = 0x80210f34; continue; }
                pc = 0x80210f2c;
            }
            0x80210f2c => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80210f78; continue; }
                pc = 0x80210f34;
            }
            0x80210f34 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80210f78; continue; }
                pc = 0x80210f3c;
            }
            0x80210f3c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x80210f50; f_80208750(c);
                pc = 0x80210f50;
            }
            0x80210f50 => {
                c.r[3] = 0x12;
                c.lr = 0x80210f58; f_801f0234(c);
                pc = 0x80210f58;
            }
            0x80210f58 => {
                c.lr = 0x80210f5c; f_801f0204(c);
                pc = 0x80210f5c;
            }
            0x80210f5c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210f74; f_802085c4(c);
                pc = 0x80210f74;
            }
            0x80210f74 => {
                pc = 0x80211024; continue;
                pc = 0x80210f78;
            }
            0x80210f78 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.lr = 0x80210f8c; f_80208750(c);
                pc = 0x80210f8c;
            }
            0x80210f8c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x80210fa0; f_80208750(c);
                pc = 0x80210fa0;
            }
            0x80210fa0 => {
                c.r[3] = 0x11;
                c.lr = 0x80210fa8; f_801f0234(c);
                pc = 0x80210fa8;
            }
            0x80210fa8 => {
                c.lr = 0x80210fac; f_801f0204(c);
                pc = 0x80210fac;
            }
            0x80210fac => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x80210fc4; f_802085c4(c);
                pc = 0x80210fc4;
            }
            0x80210fc4 => {
                c.r[3] = 0x12;
                c.lr = 0x80210fcc; f_801f0234(c);
                pc = 0x80210fcc;
            }
            0x80210fcc => {
                c.lr = 0x80210fd0; f_801f0204(c);
                pc = 0x80210fd0;
            }
            0x80210fd0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80210fe8; f_802085c4(c);
                pc = 0x80210fe8;
            }
            0x80210fe8 => {
                pc = 0x80211024; continue;
                pc = 0x80210fec;
            }
            0x80210fec => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x3;
                c.lr = 0x80211000; f_80208750(c);
                pc = 0x80211000;
            }
            0x80211000 => {
                c.r[3] = 0x11;
                c.lr = 0x80211008; f_801f0234(c);
                pc = 0x80211008;
            }
            0x80211008 => {
                c.lr = 0x8021100c; f_801f0204(c);
                pc = 0x8021100c;
            }
            0x8021100c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x80211024; f_802085c4(c);
                pc = 0x80211024;
            }
            0x80211024 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80210ec8: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncTame (0x80211038, 8 bytes)
#[inline(never)]
pub fn f_80211038(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211038) { return; }
        if c.prof { crate::prof::enter(c, 0x80211038); f_80211038_b(c, 0x80211038); crate::prof::exit(c, 0x80211038); return; }
    }
    f_80211038_b(c, 0x80211038)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211038_at(c: &mut Cpu, pc: u32) { f_80211038_b(c, pc) }
#[inline(always)]
fn f_80211038_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211038;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211038, pc); }
        match pc {
            0x80211038 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80211038: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncNormal (0x80211040, 284 bytes)
#[inline(never)]
pub fn f_80211040(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211040) { return; }
        if c.prof { crate::prof::enter(c, 0x80211040); f_80211040_b(c, 0x80211040); crate::prof::exit(c, 0x80211040); return; }
    }
    f_80211040_b(c, 0x80211040)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211040_at(c: &mut Cpu, pc: u32) { f_80211040_b(c, pc) }
#[inline(always)]
fn f_80211040_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211040;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211040, pc); }
        match pc {
            0x80211040 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = c.r[7] | c.r[7];
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x80211078; f_8011beb4(c);
                pc = 0x80211078;
            }
            0x80211078 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x80211094; continue; }
                pc = 0x80211084;
            }
            0x80211084 => {
                c.cmpu(0, c.r[0], 0x6);
                if c.crbit(2) { pc = 0x80211094; continue; }
                pc = 0x8021108c;
            }
            0x8021108c => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802110d8; continue; }
                pc = 0x80211094;
            }
            0x80211094 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x802110d8; continue; }
                pc = 0x8021109c;
            }
            0x8021109c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x802110b0; f_80208750(c);
                pc = 0x802110b0;
            }
            0x802110b0 => {
                c.r[3] = 0x12;
                c.lr = 0x802110b8; f_801f0234(c);
                pc = 0x802110b8;
            }
            0x802110b8 => {
                c.lr = 0x802110bc; f_801f0204(c);
                pc = 0x802110bc;
            }
            0x802110bc => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x802110d4; f_802085c4(c);
                pc = 0x802110d4;
            }
            0x802110d4 => {
                pc = 0x80211148; continue;
                pc = 0x802110d8;
            }
            0x802110d8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.lr = 0x802110ec; f_80208750(c);
                pc = 0x802110ec;
            }
            0x802110ec => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.lr = 0x80211100; f_80208750(c);
                pc = 0x80211100;
            }
            0x80211100 => {
                c.r[3] = 0x11;
                c.lr = 0x80211108; f_801f0234(c);
                pc = 0x80211108;
            }
            0x80211108 => {
                c.lr = 0x8021110c; f_801f0204(c);
                pc = 0x8021110c;
            }
            0x8021110c => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x80211124; f_802085c4(c);
                pc = 0x80211124;
            }
            0x80211124 => {
                c.r[3] = 0x12;
                c.lr = 0x8021112c; f_801f0234(c);
                pc = 0x8021112c;
            }
            0x8021112c => {
                c.lr = 0x80211130; f_801f0204(c);
                pc = 0x80211130;
            }
            0x80211130 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80211148; f_802085c4(c);
                pc = 0x80211148;
            }
            0x80211148 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80211040: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncNormal (0x8021115c, 8 bytes)
#[inline(never)]
pub fn f_8021115c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8021115c) { return; }
        if c.prof { crate::prof::enter(c, 0x8021115c); f_8021115c_b(c, 0x8021115c); crate::prof::exit(c, 0x8021115c); return; }
    }
    f_8021115c_b(c, 0x8021115c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8021115c_at(c: &mut Cpu, pc: u32) { f_8021115c_b(c, pc) }
#[inline(always)]
fn f_8021115c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8021115c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8021115c, pc); }
        match pc {
            0x8021115c => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8021115c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxTypeFuncNull (0x80211164, 4 bytes)
#[inline(never)]
pub fn f_80211164(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211164) { return; }
        if c.prof { crate::prof::enter(c, 0x80211164); f_80211164_b(c, 0x80211164); crate::prof::exit(c, 0x80211164); return; }
    }
    f_80211164_b(c, 0x80211164)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211164_at(c: &mut Cpu, pc: u32) { f_80211164_b(c, pc) }
#[inline(always)]
fn f_80211164_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211164;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211164, pc); }
        match pc {
            0x80211164 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80211164: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaWzxVariationFuncNull (0x80211168, 8 bytes)
#[inline(never)]
pub fn f_80211168(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211168) { return; }
        if c.prof { crate::prof::enter(c, 0x80211168); f_80211168_b(c, 0x80211168); crate::prof::exit(c, 0x80211168); return; }
    }
    f_80211168_b(c, 0x80211168)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211168_at(c: &mut Cpu, pc: u32) { f_80211168_b(c, pc) }
#[inline(always)]
fn f_80211168_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211168;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211168, pc); }
        match pc {
            0x80211168 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80211168: bad pc {:#x}", pc),
        }
    }
}

/// fightSeqGetNromalWazaDamage (0x80211170, 1676 bytes)
#[inline(never)]
pub fn f_80211170(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211170) { return; }
        if c.prof { crate::prof::enter(c, 0x80211170); f_80211170_b(c, 0x80211170); crate::prof::exit(c, 0x80211170); return; }
    }
    f_80211170_b(c, 0x80211170)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211170_at(c: &mut Cpu, pc: u32) { f_80211170_b(c, pc) }
#[inline(always)]
fn f_80211170_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211170;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211170, pc); }
        match pc {
            0x80211170 => {
                { let ea = c.r[1].wrapping_add(0xfffff130); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xed4), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xe88); for n in 14..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[16] = c.r[3] | c.r[3];
                c.r[17] = c.r[4] | c.r[4];
                c.r[18] = c.r[5] | c.r[5];
                c.r[19] = c.r[6] | c.r[6];
                c.r[15] = c.r[7] | c.r[7];
                c.r[20] = c.r[8] | c.r[8];
                c.r[14] = c.r[9] | c.r[9];
                c.r[21] = c.r[10] | c.r[10];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x802111b4; f_801f54a4(c);
                pc = 0x802111b4;
            }
            0x802111b4 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0xe7c), c.r[0]);
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x802111d0; f_801f54a4(c);
                pc = 0x802111d0;
            }
            0x802111d0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[4] = 0x8;
                c.r[6] = c.r[13].wrapping_add(0xffff8558);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[23] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0xe80), c.r[0]);
                c.r[22] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.mtspr(9, c.r[4]);
                pc = 0x802111f0;
            }
            0x802111f0 => {
                c.r[0] = c.lbz(c.r[6]);
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.stb(c.r[5], c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x802111f0; continue; }
                pc = 0x80211204;
            }
            0x80211204 => {
                c.r[4] = c.r[18] | c.r[18];
                c.r[3] = c.r[1].wrapping_add(0x79c);
                c.lr = 0x80211210; f_801fcec4(c);
                pc = 0x80211210;
            }
            0x80211210 => {
                c.r[4] = c.r[19] | c.r[19];
                c.r[3] = c.r[1].wrapping_add(0xbc);
                c.lr = 0x8021121c; f_801fcec4(c);
                pc = 0x8021121c;
            }
            0x8021121c => {
                c.r[7] = c.r[18] | c.r[18];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x80211234; f_801f4c14(c);
                pc = 0x80211234;
            }
            0x80211234 => {
                c.r[7] = c.r[19] | c.r[19];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x8021124c; f_801f4c14(c);
                pc = 0x8021124c;
            }
            0x8021124c => {
                c.r[3] = c.r[18] | c.r[18];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80211260; f_8012640c(c);
                pc = 0x80211260;
            }
            0x80211260 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.r[24] = c.r[0] | c.r[0];
                c.r[4] = c.r[24] | c.r[24];
                c.lr = 0x80211274; f_8020a2b8(c);
                pc = 0x80211274;
            }
            0x80211274 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8021127c; f_80209d90(c);
                pc = 0x8021127c;
            }
            0x8021127c => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[5] = c.r[17] | c.r[17];
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80211294; f_802099ac(c);
                pc = 0x80211294;
            }
            0x80211294 => {
                c.r[4] = 0x0;
                c.r[0] = 0x8;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[4]);
                c.mtspr(9, c.r[0]);
                pc = 0x802112a8;
            }
            0x802112a8 => {
                c.stb(c.r[3], c.r[4]);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x802112a8; continue; }
                pc = 0x802112b4;
            }
            0x802112b4 => {
                c.cmpu(0, c.r[14], 0x0);
                if c.crbit(2) { pc = 0x802112d8; continue; }
                pc = 0x802112bc;
            }
            0x802112bc => {
                c.r[12] = c.r[14] | c.r[14];
                c.r[3] = c.r[16] | c.r[16];
                c.r[4] = c.r[17] | c.r[17];
                c.r[5] = c.r[18] | c.r[18];
                c.r[6] = c.r[19] | c.r[19];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x802112d8; (c.dispatch)(c, t); }
                pc = 0x802112d8;
            }
            0x802112d8 => {
                c.r[0] = c.r[15].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021158c; continue; }
                pc = 0x802112e4;
            }
            0x802112e4 => {
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x802112f0; f_801f025c(c);
                pc = 0x802112f0;
            }
            0x802112f0 => {
                c.r[25] = c.r[3] | c.r[3];
                c.lr = 0x802112f8; f_80207bf4(c);
                pc = 0x802112f8;
            }
            0x802112f8 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80211300; f_80203fe4(c);
                pc = 0x80211300;
            }
            0x80211300 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80211318; f_8012640c(c);
                pc = 0x80211318;
            }
            0x80211318 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80211324; f_80205184(c);
                pc = 0x80211324;
            }
            0x80211324 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80211330; f_80203d3c(c);
                pc = 0x80211330;
            }
            0x80211330 => {
                c.r[14] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.lr = 0x80211340; f_801f025c(c);
                pc = 0x80211340;
            }
            0x80211340 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.r[30] = 0x0;
                c.cmpu(0, c.r[0], 0x3f);
                c.r[26] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x80211364; continue; }
                pc = 0x80211354;
            }
            0x80211354 => {
                c.r[0] = c.r[14].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x71);
                if !c.crbit(2) { pc = 0x80211364; continue; }
                pc = 0x80211360;
            }
            0x80211360 => {
                c.r[30] = 0x1;
                pc = 0x80211364;
            }
            0x80211364 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.r[31] = 0x0;
                c.cmpu(0, c.r[0], 0x42);
                if !c.crbit(2) { pc = 0x80211384; continue; }
                pc = 0x80211374;
            }
            0x80211374 => {
                c.r[0] = c.r[14].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x53);
                if !c.crbit(2) { pc = 0x80211384; continue; }
                pc = 0x80211380;
            }
            0x80211380 => {
                c.r[31] = 0x1;
                pc = 0x80211384;
            }
            0x80211384 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0xf;
                c.lr = 0x80211390; f_802026e4(c);
                pc = 0x80211390;
            }
            0x80211390 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[3] = 0x0;
                c.r[0] = c.r[0].leading_zeros();
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.r[15] = c.r[0].rotate_left(28) & 0xffffffe;
                c.lr = 0x802113b4; f_8011beb4(c);
                pc = 0x802113b4;
            }
            0x802113b4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[0] = c.addc(!c.r[0], 0x2b, 1);
                c.r[3] = 0x0;
                c.r[0] = c.r[0].leading_zeros();
                c.r[5] = 0x9;
                c.r[14] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[6] = 0x0;
                c.lr = 0x802113d8; f_8011beb4(c);
                pc = 0x802113d8;
            }
            0x802113d8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[0] = c.addc(!c.r[0], 0x4b, 1);
                c.r[3] = 0x0;
                c.r[0] = c.r[0].leading_zeros();
                c.r[5] = 0x9;
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[6] = 0x0;
                c.r[15] = c.r[15].wrapping_add(c.r[0]);
                c.lr = 0x80211400; f_8011beb4(c);
                pc = 0x80211400;
            }
            0x80211400 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[0] = c.addc(!c.r[0], 0xc8, 1);
                c.r[3] = 0x0;
                c.r[0] = c.r[0].leading_zeros();
                c.r[5] = 0x9;
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[6] = 0x0;
                c.r[15] = c.r[15].wrapping_add(c.r[0]);
                c.lr = 0x80211428; f_8011beb4(c);
                pc = 0x80211428;
            }
            0x80211428 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8540));
                c.r[4] = c.addc(!c.r[0], 0xd1, 1);
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.r[6] = c.r[4].leading_zeros();
                c.r[4] = c.r[30].rotate_left(1) & 0xfffffffe;
                c.r[5] = c.addc(!c.r[0], 0x29, 1);
                c.r[0] = c.r[31].rotate_left(1) & 0xfffffffe;
                c.r[6] = c.r[6].rotate_left(27) & 0x7ffffff;
                c.r[7] = c.r[3].wrapping_add(0xffffffff);
                c.r[3] = c.r[5].leading_zeros();
                c.r[5] = c.r[15].wrapping_add(c.r[6]);
                c.r[3] = c.r[3].rotate_left(27) & 0x7ffffff;
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.r[14].wrapping_add(c.r[0]);
                c.r[14] = c.r[0].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[14], c.r[7]);
                if !c.crbit(1) { pc = 0x8021147c; continue; }
                pc = 0x80211478;
            }
            0x80211478 => {
                c.r[14] = c.r[7].rotate_left(0) & 0xffff;
                pc = 0x8021147c;
            }
            0x8021147c => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x80211484; f_80207bf4(c);
                pc = 0x80211484;
            }
            0x80211484 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x80211568; continue; }
                pc = 0x80211490;
            }
            0x80211490 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x80211498; f_80207bf4(c);
                pc = 0x80211498;
            }
            0x80211498 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x4b);
                if c.crbit(2) { pc = 0x80211568; continue; }
                pc = 0x802114a4;
            }
            0x802114a4 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.lr = 0x802114b8; f_801f54a4(c);
                pc = 0x802114b8;
            }
            0x802114b8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80211568; continue; }
                pc = 0x802114c4;
            }
            0x802114c4 => {
                c.r[3] = c.r[14] | c.r[14];
                c.lr = 0x802114cc; f_8020a080(c);
                pc = 0x802114cc;
            }
            0x802114cc => {
                c.lr = 0x802114d0; f_8020a068(c);
                pc = 0x802114d0;
            }
            0x802114d0 => {
                c.r[14] = c.r[3].rotate_left(0) & 0xff;
                c.lr = 0x802114d8; f_800e0c54(c);
                pc = 0x802114d8;
            }
            0x802114d8 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = divw(c.r[3], c.r[14]);
                c.r[0] = c.r[0].wrapping_mul(c.r[14]);
                c.r[0] = c.r[3].wrapping_sub(c.r[0]);
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80211530; continue; }
                pc = 0x802114ec;
            }
            0x802114ec => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x3e;
                c.lr = 0x802114f8; f_802026e4(c);
                pc = 0x802114f8;
            }
            0x802114f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021154c; continue; }
                pc = 0x80211504;
            }
            0x80211504 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x164);
                if !c.crbit(2) { pc = 0x8021154c; continue; }
                pc = 0x80211510;
            }
            0x80211510 => {
                c.lr = 0x80211514; f_800e0c54(c);
                pc = 0x80211514;
            }
            0x80211514 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x64;
                c.r[0] = divw(c.r[4], c.r[3]);
                c.r[0] = c.r[0].wrapping_mul(c.r[3]);
                c.r[0] = c.r[4].wrapping_sub(c.r[0]);
                c.cmps(0, c.r[0], 0x5a);
                if !c.crbit(0) { pc = 0x8021154c; continue; }
                pc = 0x80211530;
            }
            0x80211530 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2b;
                c.r[6] = 0x0;
                c.r[7] = 0x2;
                c.lr = 0x80211548; f_8011bbd8(c);
                pc = 0x80211548;
            }
            0x80211548 => {
                pc = 0x80211580; continue;
                pc = 0x8021154c;
            }
            0x8021154c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2b;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80211564; f_8011bbd8(c);
                pc = 0x80211564;
            }
            0x80211564 => {
                pc = 0x80211580; continue;
                pc = 0x80211568;
            }
            0x80211568 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2b;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80211580; f_8011bbd8(c);
                pc = 0x80211580;
            }
            0x80211580 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021158c;
            }
            0x8021158c => {
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80211598; f_801f025c(c);
                pc = 0x80211598;
            }
            0x80211598 => {
                c.r[15] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.lr = 0x802115a8; f_801f025c(c);
                pc = 0x802115a8;
            }
            0x802115a8 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x802115b8; f_801f025c(c);
                pc = 0x802115b8;
            }
            0x802115b8 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[15] | c.r[15];
                c.lr = 0x802115c4; f_80205184(c);
                pc = 0x802115c4;
            }
            0x802115c4 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x802115dc; f_8012640c(c);
                pc = 0x802115dc;
            }
            0x802115dc => {
                c.r[4] = 0x0;
                c.r[14] = c.r[3] | c.r[3];
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x802115f0; f_8011beb4(c);
                pc = 0x802115f0;
            }
            0x802115f0 => {
                c.r[25] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[14] | c.r[14];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.lr = 0x80211608; f_8011beb4(c);
                pc = 0x80211608;
            }
            0x80211608 => {
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[27] | c.r[27];
                c.r[7] = c.r[25] | c.r[25];
                c.r[8] = c.r[26] | c.r[26];
                c.lr = 0x80211628; f_80232110(c);
                pc = 0x80211628;
            }
            0x80211628 => {
                c.r[25] = c.r[3] | c.r[3];
                c.r[3] = c.r[14] | c.r[14];
                c.r[4] = 0x0;
                c.r[5] = 0x2b;
                c.r[6] = 0x0;
                c.lr = 0x80211640; f_8011beb4(c);
                pc = 0x80211640;
            }
            0x80211640 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[14] | c.r[14];
                c.r[25] = c.r[25].wrapping_mul(c.r[0]);
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.lr = 0x8021165c; f_8011beb4(c);
                pc = 0x8021165c;
            }
            0x8021165c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[15] | c.r[15];
                c.r[25] = c.r[25].wrapping_mul(c.r[0]);
                c.r[4] = 0x24;
                c.lr = 0x80211670; f_802026e4(c);
                pc = 0x80211670;
            }
            0x80211670 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80211688; continue; }
                pc = 0x8021167c;
            }
            0x8021167c => {
                c.cmpu(0, c.r[26], 0xd);
                if !c.crbit(2) { pc = 0x80211688; continue; }
                pc = 0x80211684;
            }
            0x80211684 => {
                c.r[25] = c.r[25].rotate_left(1) & 0xfffffffe;
                pc = 0x80211688;
            }
            0x80211688 => {
                c.r[3] = c.r[15] | c.r[15];
                c.r[4] = 0x32;
                c.lr = 0x80211694; f_802026e4(c);
                pc = 0x80211694;
            }
            0x80211694 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802116ac; continue; }
                pc = 0x802116a0;
            }
            0x802116a0 => {
                c.r[3] = c.r[25].wrapping_mul(0xf);
                c.r[0] = 0xa;
                c.r[25] = divw(c.r[3], c.r[0]);
                pc = 0x802116ac;
            }
            0x802116ac => {
                c.r[3] = c.r[14] | c.r[14];
                c.r[7] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x802116c4; f_8011bbd8(c);
                pc = 0x802116c4;
            }
            0x802116c4 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.r[0] = c.r[6].wrapping_add(0x1);
                c.r[6] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.lr = 0x802116e4; f_802274f0(c);
                pc = 0x802116e4;
            }
            0x802116e4 => {
                c.r[0] = c.r[20].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80211708; continue; }
                pc = 0x802116f0;
            }
            0x802116f0 => {
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.lr = 0x802116fc; f_802271e0(c);
                pc = 0x802116fc;
            }
            0x802116fc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80211708;
            }
            0x80211708 => {
                c.cmpu(0, c.r[21], 0x0);
                if c.crbit(2) { pc = 0x8021172c; continue; }
                pc = 0x80211710;
            }
            0x80211710 => {
                c.r[12] = c.r[21] | c.r[21];
                c.r[3] = c.r[16] | c.r[16];
                c.r[4] = c.r[17] | c.r[17];
                c.r[5] = c.r[18] | c.r[18];
                c.r[6] = c.r[19] | c.r[19];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x8021172c; (c.dispatch)(c, t); }
                pc = 0x8021172c;
            }
            0x8021172c => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x80211734; f_802096e8(c);
                pc = 0x80211734;
            }
            0x80211734 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80211758; continue; }
                pc = 0x80211740;
            }
            0x80211740 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x80211754; f_8011beb4(c);
                pc = 0x80211754;
            }
            0x80211754 => {
                pc = 0x8021175c; continue;
                pc = 0x80211758;
            }
            0x80211758 => {
                c.r[3] = 0x0;
                pc = 0x8021175c;
            }
            0x8021175c => {
                c.r[7] = c.lwz(c.r[1].wrapping_add(0xe7c));
                c.r[14] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x80211778; f_801f4c14(c);
                pc = 0x80211778;
            }
            0x80211778 => {
                c.r[7] = c.r[23] | c.r[23];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x80211790; f_801f4c14(c);
                pc = 0x80211790;
            }
            0x80211790 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.lr = 0x8021179c; f_8020a2b8(c);
                pc = 0x8021179c;
            }
            0x8021179c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xe80));
                c.r[3] = 0x8;
                c.r[4] = c.r[13].wrapping_add(0xffff8558);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[22]);
                c.mtspr(9, c.r[3]);
                pc = 0x802117b8;
            }
            0x802117b8 => {
                c.r[0] = c.lbz(c.r[5]);
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.stb(c.r[4], c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x802117b8; continue; }
                pc = 0x802117cc;
            }
            0x802117cc => {
                c.r[3] = c.r[18] | c.r[18];
                c.r[4] = c.r[1].wrapping_add(0x79c);
                c.lr = 0x802117d8; f_801fcec4(c);
                pc = 0x802117d8;
            }
            0x802117d8 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = c.r[1].wrapping_add(0xbc);
                c.lr = 0x802117e4; f_801fcec4(c);
                pc = 0x802117e4;
            }
            0x802117e4 => {
                c.r[3] = c.r[14] | c.r[14];
                { let mut ea = c.r[1].wrapping_add(0xe88); for n in 14..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xed4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xed0);
                if true { return; }
                return;
            }
            _ => panic!("f_80211170: bad pc {:#x}", pc),
        }
    }
}

/// fightSeqGetEffectAminFlag (0x802117fc, 20 bytes)
#[inline(never)]
pub fn f_802117fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802117fc) { return; }
        if c.prof { crate::prof::enter(c, 0x802117fc); f_802117fc_b(c, 0x802117fc); crate::prof::exit(c, 0x802117fc); return; }
    }
    f_802117fc_b(c, 0x802117fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802117fc_at(c: &mut Cpu, pc: u32) { f_802117fc_b(c, pc) }
#[inline(always)]
fn f_802117fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802117fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802117fc, pc); }
        match pc {
            0x802117fc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[0] = c.r[0].rotate_left(0) & 0x80;
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                if true { return; }
                return;
            }
            _ => panic!("f_802117fc: bad pc {:#x}", pc),
        }
    }
}

/// fightSeqSetEffectAminFlag (0x80211810, 32 bytes)
#[inline(never)]
pub fn f_80211810(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211810) { return; }
        if c.prof { crate::prof::enter(c, 0x80211810); f_80211810_b(c, 0x80211810); crate::prof::exit(c, 0x80211810); return; }
    }
    f_80211810_b(c, 0x80211810)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211810_at(c: &mut Cpu, pc: u32) { f_80211810_b(c, pc) }
#[inline(always)]
fn f_80211810_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211810;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211810, pc); }
        match pc {
            0x80211810 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.cmpu(0, c.r[3], 0x1);
                c.r[3] = c.r[0] | 0x80;
                if !c.crbit(2) { pc = 0x80211828; continue; }
                pc = 0x80211824;
            }
            0x80211824 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffffff7f;
                pc = 0x80211828;
            }
            0x80211828 => {
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_80211810: bad pc {:#x}", pc),
        }
    }
}

/// fightSeqInit (0x80211830, 204 bytes)
#[inline(never)]
pub fn f_80211830(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211830) { return; }
        if c.prof { crate::prof::enter(c, 0x80211830); f_80211830_b(c, 0x80211830); crate::prof::exit(c, 0x80211830); return; }
    }
    f_80211830_b(c, 0x80211830)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211830_at(c: &mut Cpu, pc: u32) { f_80211830_b(c, pc) }
#[inline(always)]
fn f_80211830_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211830;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211830, pc); }
        match pc {
            0x80211830 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80210000;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x3558);
                c.r[0] = 0x0;
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[3] = 0x0;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.lr = 0x80211860; f_801f37b0(c);
                pc = 0x80211860;
            }
            0x80211860 => {
                c.r[31] = 0x0;
                pc = 0x80211888; continue;
                pc = 0x80211868;
            }
            0x80211868 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x80211874; f_801f47b4(c);
                pc = 0x80211874;
            }
            0x80211874 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80211884; continue; }
                pc = 0x8021187c;
            }
            0x8021187c => {
                c.r[4] = 0x4d;
                c.lr = 0x80211884; f_801f6eec(c);
                pc = 0x80211884;
            }
            0x80211884 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80211888;
            }
            0x80211888 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x80211868; continue; }
                pc = 0x80211894;
            }
            0x80211894 => {
                c.r[4] = 0x80210000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x36a4);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802118ac; f_801f37b0(c);
                pc = 0x802118ac;
            }
            0x802118ac => {
                c.r[0] = 0x8;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.r[4] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x802118bc;
            }
            0x802118bc => {
                c.stb(c.r[3], c.r[4]);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x802118bc; continue; }
                pc = 0x802118c8;
            }
            0x802118c8 => {
                c.r[0] = 0x0;
                c.r[3] = 0x80210000;
                c.stw(c.r[13].wrapping_add(0xffffae0c), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x18fc);
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802118e8; f_801f37b0(c);
                pc = 0x802118e8;
            }
            0x802118e8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80211830: bad pc {:#x}", pc),
        }
    }
}

/// _fightSeqInitSubFightOutPokemon__FPvUsPv (0x802118fc, 76 bytes)
#[inline(never)]
pub fn f_802118fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802118fc) { return; }
        if c.prof { crate::prof::enter(c, 0x802118fc); f_802118fc_b(c, 0x802118fc); crate::prof::exit(c, 0x802118fc); return; }
    }
    f_802118fc_b(c, 0x802118fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802118fc_at(c: &mut Cpu, pc: u32) { f_802118fc_b(c, pc) }
#[inline(always)]
fn f_802118fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802118fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802118fc, pc); }
        match pc {
            0x802118fc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80211914; f_802062fc(c);
                pc = 0x80211914;
            }
            0x80211914 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80211924; continue; }
                pc = 0x8021191c;
            }
            0x8021191c => {
                c.r[3] = 0x1;
                pc = 0x80211934; continue;
                pc = 0x80211924;
            }
            0x80211924 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x11;
                c.lr = 0x80211930; f_80202810(c);
                pc = 0x80211930;
            }
            0x80211930 => {
                c.r[3] = 0x1;
                pc = 0x80211934;
            }
            0x80211934 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802118fc: bad pc {:#x}", pc),
        }
    }
}

/// fightSeqFightActionCreateAndFlowFifo (0x80211948, 140 bytes)
#[inline(never)]
pub fn f_80211948(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211948) { return; }
        if c.prof { crate::prof::enter(c, 0x80211948); f_80211948_b(c, 0x80211948); crate::prof::exit(c, 0x80211948); return; }
    }
    f_80211948_b(c, 0x80211948)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211948_at(c: &mut Cpu, pc: u32) { f_80211948_b(c, pc) }
#[inline(always)]
fn f_80211948_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211948;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211948, pc); }
        match pc {
            0x80211948 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[11] = c.r[4] | c.r[4];
                c.r[10] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.r[9] = c.r[6] | c.r[6];
                c.r[0] = c.r[7] | c.r[7];
                c.r[4] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x3c), c.r[31]);
                c.r[31] = c.r[8] | c.r[8];
                c.r[5] = c.r[11] | c.r[11];
                c.r[6] = c.r[10] | c.r[10];
                c.r[7] = c.r[9] | c.r[9];
                c.r[8] = c.r[0] | c.r[0];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80211988; f_801f11cc(c);
                pc = 0x80211988;
            }
            0x80211988 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80211998; continue; }
                pc = 0x80211994;
            }
            0x80211994 => {
                pc = 0x802119a8; continue;
                pc = 0x80211998;
            }
            0x80211998 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x802119a4; f_8020d888(c);
                pc = 0x802119a4;
            }
            0x802119a4 => {
                c.r[3] = 0x1;
                pc = 0x802119a8;
            }
            0x802119a8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x802119b8; continue; }
                pc = 0x802119b4;
            }
            0x802119b4 => {
                pc = 0x802119c0; continue;
                pc = 0x802119b8;
            }
            0x802119b8 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x802119c0; f_801f0f04(c);
                pc = 0x802119c0;
            }
            0x802119c0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_80211948: bad pc {:#x}", pc),
        }
    }
}

/// fightSeqPost (0x802119d4, 44 bytes)
#[inline(never)]
pub fn f_802119d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802119d4) { return; }
        if c.prof { crate::prof::enter(c, 0x802119d4); f_802119d4_b(c, 0x802119d4); crate::prof::exit(c, 0x802119d4); return; }
    }
    f_802119d4_b(c, 0x802119d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802119d4_at(c: &mut Cpu, pc: u32) { f_802119d4_b(c, pc) }
#[inline(always)]
fn f_802119d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802119d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802119d4, pc); }
        match pc {
            0x802119d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[0] = c.r[0].rotate_left(0) & 0xffefffff;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                c.lr = 0x802119f0; f_80213270(c);
                pc = 0x802119f0;
            }
            0x802119f0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802119d4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80211A00 (0x80211a00, 120 bytes)
#[inline(never)]
pub fn f_80211a00(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211a00) { return; }
        if c.prof { crate::prof::enter(c, 0x80211a00); f_80211a00_b(c, 0x80211a00); crate::prof::exit(c, 0x80211a00); return; }
    }
    f_80211a00_b(c, 0x80211a00)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211a00_at(c: &mut Cpu, pc: u32) { f_80211a00_b(c, pc) }
#[inline(always)]
fn f_80211a00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211a00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211a00, pc); }
        match pc {
            0x80211a00 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80210000;
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x1a78);
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x80211a24; f_801f37b0(c);
                pc = 0x80211a24;
            }
            0x80211a24 => {
                c.r[4] = 0x80230000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xfffffe20);
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x80211a3c; f_801f37b0(c);
                pc = 0x80211a3c;
            }
            0x80211a3c => {
                c.r[3] = 0x80380000;
                c.r[0] = 0x0;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[4] = c.r[13].wrapping_add(0xffff8558);
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stb(c.r[4].wrapping_add(0x3), c.r[0]);
                c.stb(c.r[13].wrapping_add(0xffffae05), c.r[0]);
                c.stb(c.r[4].wrapping_add(0x4), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x6002), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x60a1), c.r[0]);
                c.lr = 0x80211a68; f_801da7ac(c);
                pc = 0x80211a68;
            }
            0x80211a68 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80211a00: bad pc {:#x}", pc),
        }
    }
}

/// fn_80211A78 (0x80211a78, 284 bytes)
#[inline(never)]
pub fn f_80211a78(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211a78) { return; }
        if c.prof { crate::prof::enter(c, 0x80211a78); f_80211a78_b(c, 0x80211a78); crate::prof::exit(c, 0x80211a78); return; }
    }
    f_80211a78_b(c, 0x80211a78)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211a78_at(c: &mut Cpu, pc: u32) { f_80211a78_b(c, pc) }
#[inline(always)]
fn f_80211a78_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211a78;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211a78, pc); }
        match pc {
            0x80211a78 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x3c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80211a90; f_802062fc(c);
                pc = 0x80211a90;
            }
            0x80211a90 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80211aa0; continue; }
                pc = 0x80211a98;
            }
            0x80211a98 => {
                c.r[3] = 0x1;
                pc = 0x80211b80; continue;
                pc = 0x80211aa0;
            }
            0x80211aa0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xfe;
                c.r[6] = 0x0;
                c.lr = 0x80211ab4; f_8012640c(c);
                pc = 0x80211ab4;
            }
            0x80211ab4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80211ac4; continue; }
                pc = 0x80211abc;
            }
            0x80211abc => {
                c.r[3] = 0x1;
                pc = 0x80211b80; continue;
                pc = 0x80211ac4;
            }
            0x80211ac4 => {
                c.lr = 0x80211ac8; f_801f1170(c);
                pc = 0x80211ac8;
            }
            0x80211ac8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80211ad8; continue; }
                pc = 0x80211ad0;
            }
            0x80211ad0 => {
                c.r[3] = 0x1;
                pc = 0x80211b80; continue;
                pc = 0x80211ad8;
            }
            0x80211ad8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80211ae0; f_80205224(c);
                pc = 0x80211ae0;
            }
            0x80211ae0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x108);
                if !c.crbit(2) { pc = 0x80211b7c; continue; }
                pc = 0x80211aec;
            }
            0x80211aec => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x80211af8; f_802026e4(c);
                pc = 0x80211af8;
            }
            0x80211af8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80211b7c; continue; }
                pc = 0x80211b00;
            }
            0x80211b00 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xf9;
                c.r[6] = 0x0;
                c.lr = 0x80211b14; f_8012640c(c);
                pc = 0x80211b14;
            }
            0x80211b14 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80211b7c; continue; }
                pc = 0x80211b1c;
            }
            0x80211b1c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae0c));
                c.lr = 0x80211b24; f_8020d920(c);
                pc = 0x80211b24;
            }
            0x80211b24 => {
                c.r[6] = 0x80370000;
                c.r[4] = c.r[3] | c.r[3];
                c.r[8] = c.r[6].wrapping_add(0x5d30);
                c.r[5] = c.r[31] | c.r[31];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[6] = 0xc;
                c.r[7] = 0x0;
                c.lr = 0x80211b44; f_801f11cc(c);
                pc = 0x80211b44;
            }
            0x80211b44 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80211b54; continue; }
                pc = 0x80211b50;
            }
            0x80211b50 => {
                pc = 0x80211b68; continue;
                pc = 0x80211b54;
            }
            0x80211b54 => {
                c.r[4] = 0x80380000;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[4].wrapping_add(0xffff91fe);
                c.lr = 0x80211b64; f_8020d888(c);
                pc = 0x80211b64;
            }
            0x80211b64 => {
                c.r[3] = 0x1;
                pc = 0x80211b68;
            }
            0x80211b68 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80211b7c; continue; }
                pc = 0x80211b74;
            }
            0x80211b74 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80211b7c; f_801f0f04(c);
                pc = 0x80211b7c;
            }
            0x80211b7c => {
                c.r[3] = 0x1;
                pc = 0x80211b80;
            }
            0x80211b80 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_80211a78: bad pc {:#x}", pc),
        }
    }
}

/// fn_80211B94 (0x80211b94, 644 bytes)
#[inline(never)]
pub fn f_80211b94(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211b94) { return; }
        if c.prof { crate::prof::enter(c, 0x80211b94); f_80211b94_b(c, 0x80211b94); crate::prof::exit(c, 0x80211b94); return; }
    }
    f_80211b94_b(c, 0x80211b94)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211b94_at(c: &mut Cpu, pc: u32) { f_80211b94_b(c, pc) }
#[inline(always)]
fn f_80211b94_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211b94;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211b94, pc); }
        match pc {
            0x80211b94 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[23] = c.r[5] | c.r[5];
                c.r[24] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[26] = c.lwz(c.r[13].wrapping_add(0xffffae0c));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[4]);
                c.r[25] = c.lbz(c.r[13].wrapping_add(0xffffadf4));
                if !c.crbit(2) { pc = 0x80211bc8; continue; }
                pc = 0x80211bc0;
            }
            0x80211bc0 => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffadf4), c.r[0]);
                pc = 0x80211bc8;
            }
            0x80211bc8 => {
                c.r[4] = 0x80280000;
                c.stw(c.r[13].wrapping_add(0xffffae0c), c.r[3]);
                c.r[31] = c.r[23].rotate_left(0) & 0xff;
                c.r[30] = c.r[4].wrapping_add(0xffffa00c);
                pc = 0x80211bd8;
            }
            0x80211bd8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lbz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[12] = c.lwz(c.r[30].wrapping_add(c.r[0]));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80211bf0; (c.dispatch)(c, t); }
                pc = 0x80211bf0;
            }
            0x80211bf0 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80211dc8; continue; }
                pc = 0x80211bf8;
            }
            0x80211bf8 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffadf4));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80211cf8; continue; }
                pc = 0x80211c04;
            }
            0x80211c04 => {
                c.r[4] = 0x80230000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x11c);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80211c1c; f_801f37b0(c);
                pc = 0x80211c1c;
            }
            0x80211c1c => {
                c.r[27] = c.lwz(c.r[13].wrapping_add(0xffffae0c));
                c.r[4] = 0x80380000;
                c.r[28] = c.lbz(c.r[13].wrapping_add(0xffffadf4));
                c.r[0] = 0x0;
                c.r[3] = 0x80280000;
                c.r[29] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[4] = c.r[4].wrapping_add(0xffff8798);
                c.stb(c.r[13].wrapping_add(0xffffadf4), c.r[0]);
                c.r[22] = c.r[3].wrapping_add(0xffffa00c);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffae0c), c.r[27]);
                pc = 0x80211c48;
            }
            0x80211c48 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lbz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[12] = c.lwz(c.r[22].wrapping_add(c.r[0]));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80211c60; (c.dispatch)(c, t); }
                pc = 0x80211c60;
            }
            0x80211c60 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffadf4));
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80211c74; continue; }
                pc = 0x80211c6c;
            }
            0x80211c6c => {
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80211c48; continue; }
                pc = 0x80211c74;
            }
            0x80211c74 => {
                c.r[0] = 0x1;
                c.r[3] = 0x80230000;
                c.stw(c.r[13].wrapping_add(0xffffae0c), c.r[27]);
                c.r[4] = c.r[3].wrapping_add(0xffffe34c);
                c.r[5] = c.r[1].wrapping_add(0x9);
                c.r[3] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffadf4), c.r[28]);
                c.r[6] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[29]);
                c.stb(c.r[1].wrapping_add(0x9), c.r[0]);
                c.lr = 0x80211ca0; f_801f37b0(c);
                pc = 0x80211ca0;
            }
            0x80211ca0 => {
                c.r[4] = 0x80230000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffffe1f8);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80211cb8; f_801f37b0(c);
                pc = 0x80211cb8;
            }
            0x80211cb8 => {
                c.r[4] = 0x80230000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x88);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80211cd0; f_801f37b0(c);
                pc = 0x80211cd0;
            }
            0x80211cd0 => {
                c.r[0] = 0x0;
                c.r[3] = 0x80230000;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xffffeb9c);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80211cf0; f_801f37b0(c);
                pc = 0x80211cf0;
            }
            0x80211cf0 => {
                c.r[0] = 0x2;
                c.stb(c.r[13].wrapping_add(0xffffadf4), c.r[0]);
                pc = 0x80211cf8;
            }
            0x80211cf8 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffadf4));
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80211bd8; continue; }
                pc = 0x80211d04;
            }
            0x80211d04 => {
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80211d10; f_801f025c(c);
                pc = 0x80211d10;
            }
            0x80211d10 => {
                c.r[4] = 0x0;
                c.r[27] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80211d24; f_8012640c(c);
                pc = 0x80211d24;
            }
            0x80211d24 => {
                c.r[4] = 0x80210000;
                c.r[22] = c.r[3] | c.r[3];
                c.r[4] = c.r[4].wrapping_add(0x36a4);
                c.r[3] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80211d40; f_801f37b0(c);
                pc = 0x80211d40;
            }
            0x80211d40 => {
                c.r[4] = 0xf1e90000;
                c.r[3] = 0x80380000;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[4] = c.r[4].wrapping_add(0xffff92af);
                c.r[0] = 0x0;
                c.r[8] = c.r[3].wrapping_add(0x10000);
                c.r[9] = c.r[13].wrapping_add(0xffff8558);
                c.r[5] = c.r[5] & c.r[4];
                c.stb(c.r[9].wrapping_add(0x3), c.r[0]);
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[5]);
                c.r[5] = 0xf3;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffae05), c.r[0]);
                c.stb(c.r[9].wrapping_add(0x4), c.r[0]);
                c.stb(c.r[8].wrapping_add(0x6002), c.r[0]);
                c.stb(c.r[8].wrapping_add(0x60a1), c.r[0]);
                c.lr = 0x80211d94; f_801254b4(c);
                pc = 0x80211d94;
            }
            0x80211d94 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0xf4;
                c.r[6] = 0x0;
                c.r[7] = 0x9;
                c.lr = 0x80211dac; f_801254b4(c);
                pc = 0x80211dac;
            }
            0x80211dac => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80211dc4; f_8011bbd8(c);
                pc = 0x80211dc4;
            }
            0x80211dc4 => {
                pc = 0x80211ddc; continue;
                pc = 0x80211dc8;
            }
            0x80211dc8 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffadf4));
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80211ddc; continue; }
                pc = 0x80211dd4;
            }
            0x80211dd4 => {
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80211bd8; continue; }
                pc = 0x80211ddc;
            }
            0x80211ddc => {
                c.r[0] = c.r[23].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80211df8; continue; }
                pc = 0x80211de4;
            }
            0x80211de4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffdff;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                c.r[0] = c.r[0].rotate_left(0) & 0xfff7ffff;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                pc = 0x80211df8;
            }
            0x80211df8 => {
                c.stw(c.r[13].wrapping_add(0xffffae0c), c.r[26]);
                c.stb(c.r[13].wrapping_add(0xffffadf4), c.r[25]);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[24]);
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 22..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_80211b94: bad pc {:#x}", pc),
        }
    }
}

/// fn_80211E18 (0x80211e18, 2220 bytes)
#[inline(never)]
pub fn f_80211e18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80211e18) { return; }
        if c.prof { crate::prof::enter(c, 0x80211e18); f_80211e18_b(c, 0x80211e18); crate::prof::exit(c, 0x80211e18); return; }
    }
    f_80211e18_b(c, 0x80211e18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80211e18_at(c: &mut Cpu, pc: u32) { f_80211e18_b(c, pc) }
#[inline(always)]
fn f_80211e18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80211e18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80211e18, pc); }
        match pc {
            0x80211e18 => {
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 17..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[21] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80211e44; f_801f54a4(c);
                pc = 0x80211e44;
            }
            0x80211e44 => {
                c.r[20] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80211e54; f_801f025c(c);
                pc = 0x80211e54;
            }
            0x80211e54 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[19] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80211e68; f_801f025c(c);
                pc = 0x80211e68;
            }
            0x80211e68 => {
                c.r[22] = c.r[3] | c.r[3];
                c.r[4] = c.r[19] | c.r[19];
                c.r[3] = 0x0;
                c.lr = 0x80211e78; f_801f4354(c);
                pc = 0x80211e78;
            }
            0x80211e78 => {
                c.r[4] = 0x0;
                c.r[29] = c.r[3] | c.r[3];
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x80211e8c; f_801fb1c0(c);
                pc = 0x80211e8c;
            }
            0x80211e8c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[19] | c.r[19];
                c.r[23] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xe5;
                c.r[6] = 0x0;
                c.lr = 0x80211ea8; f_8012640c(c);
                pc = 0x80211ea8;
            }
            0x80211ea8 => {
                c.r[17] = c.r[3] | c.r[3];
                c.rc(c.r[17]);
                if c.crbit(2) { pc = 0x802126b0; continue; }
                pc = 0x80211eb0;
            }
            0x80211eb0 => {
                c.r[4] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.lr = 0x80211ec0; f_80142cf4(c);
                pc = 0x80211ec0;
            }
            0x80211ec0 => {
                c.r[3] = c.r[17] | c.r[17];
                c.r[4] = 0x0;
                c.r[5] = 0x20;
                c.r[6] = 0x0;
                c.lr = 0x80211ed4; f_80142cf4(c);
                pc = 0x80211ed4;
            }
            0x80211ed4 => {
                c.r[24] = c.r[3] | c.r[3];
                c.r[3] = c.r[17] | c.r[17];
                c.r[4] = 0x0;
                c.r[5] = 0x21;
                c.r[6] = 0x0;
                c.lr = 0x80211eec; f_80142cf4(c);
                pc = 0x80211eec;
            }
            0x80211eec => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80211ef8; f_801440a0(c);
                pc = 0x80211ef8;
            }
            0x80211ef8 => {
                c.lr = 0x80211efc; f_80143dfc(c);
                pc = 0x80211efc;
            }
            0x80211efc => {
                c.lr = 0x80211f00; f_80143a94(c);
                pc = 0x80211f00;
            }
            0x80211f00 => {
                c.r[4] = 0x80380000;
                c.r[0] = c.r[3] | c.r[3];
                c.r[4] = c.r[4].wrapping_add(0xffff9f58);
                c.r[3] = c.r[19] | c.r[19];
                c.r[30] = c.r[4].wrapping_add(0x10000);
                c.r[17] = c.r[0] | c.r[0];
                c.r[26] = c.lbz(c.r[30].wrapping_add(0x601e));
                c.r[4] = 0x2e;
                c.r[27] = c.lbz(c.r[30].wrapping_add(0x60a4));
                c.lr = 0x80211f28; f_802026e4(c);
                pc = 0x80211f28;
            }
            0x80211f28 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80211f40; continue; }
                pc = 0x80211f34;
            }
            0x80211f34 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x2e;
                c.lr = 0x80211f40; f_80202810(c);
                pc = 0x80211f40;
            }
            0x80211f40 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x15;
                c.lr = 0x80211f4c; f_802026e4(c);
                pc = 0x80211f4c;
            }
            0x80211f4c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80211f64; continue; }
                pc = 0x80211f58;
            }
            0x80211f58 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x15;
                c.lr = 0x80211f64; f_80202810(c);
                pc = 0x80211f64;
            }
            0x80211f64 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x28;
                c.lr = 0x80211f70; f_802026e4(c);
                pc = 0x80211f70;
            }
            0x80211f70 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80211f88; continue; }
                pc = 0x80211f7c;
            }
            0x80211f7c => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x28;
                c.lr = 0x80211f88; f_80202810(c);
                pc = 0x80211f88;
            }
            0x80211f88 => {
                c.cmpu(0, c.r[28], 0x1);
                if !c.crbit(2) { pc = 0x80211f9c; continue; }
                pc = 0x80211f90;
            }
            0x80211f90 => {
                c.r[3] = 0x80370000;
                c.r[17] = c.lwz(c.r[3].wrapping_add(0x5e24));
                pc = 0x802125dc; continue;
                pc = 0x80211f9c;
            }
            0x80211f9c => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x80211fb0; f_80142cf4(c);
                pc = 0x80211fb0;
            }
            0x80211fb0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80211fd0; continue; }
                pc = 0x80211fbc;
            }
            0x80211fbc => {
                c.r[3] = 0x80370000;
                c.r[0] = c.r[31].rotate_left(2) & 0x3fffc;
                c.r[3] = c.r[3].wrapping_add(0x5df0);
                c.r[17] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x802125dc; continue;
                pc = 0x80211fd0;
            }
            0x80211fd0 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x50);
                if c.crbit(2) { pc = 0x80211fe4; continue; }
                pc = 0x80211fdc;
            }
            0x80211fdc => {
                c.cmpu(0, c.r[0], 0x51);
                if !c.crbit(2) { pc = 0x80211ff0; continue; }
                pc = 0x80211fe4;
            }
            0x80211fe4 => {
                c.r[3] = 0x80370000;
                c.r[17] = c.lwz(c.r[3].wrapping_add(0x5e44));
                pc = 0x802125dc; continue;
                pc = 0x80211ff0;
            }
            0x80211ff0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80211ff8; f_801440a0(c);
                pc = 0x80211ff8;
            }
            0x80211ff8 => {
                c.lr = 0x80211ffc; f_80143dfc(c);
                pc = 0x80211ffc;
            }
            0x80211ffc => {
                c.lr = 0x80212000; f_80143a94(c);
                pc = 0x80212000;
            }
            0x80212000 => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if !c.crbit(2) { pc = 0x80212010; continue; }
                pc = 0x80212008;
            }
            0x80212008 => {
                c.r[25] = 0x7;
                pc = 0x8021214c; continue;
                pc = 0x80212010;
            }
            0x80212010 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x13);
                if !c.crbit(2) { pc = 0x80212024; continue; }
                pc = 0x8021201c;
            }
            0x8021201c => {
                c.r[25] = 0x1;
                pc = 0x8021214c; continue;
                pc = 0x80212024;
            }
            0x80212024 => {
                c.lr = 0x80212028; f_801437e0(c);
                pc = 0x80212028;
            }
            0x80212028 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80212038; continue; }
                pc = 0x80212030;
            }
            0x80212030 => {
                c.r[25] = 0x2;
                pc = 0x8021214c; continue;
                pc = 0x80212038;
            }
            0x80212038 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80212040; f_80143940(c);
                pc = 0x80212040;
            }
            0x80212040 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x802120b0; continue; }
                pc = 0x8021204c;
            }
            0x8021204c => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80212054; f_80143918(c);
                pc = 0x80212054;
            }
            0x80212054 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x802120b0; continue; }
                pc = 0x80212060;
            }
            0x80212060 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80212068; f_801438f0(c);
                pc = 0x80212068;
            }
            0x80212068 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x802120b0; continue; }
                pc = 0x80212074;
            }
            0x80212074 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8021207c; f_801438c8(c);
                pc = 0x8021207c;
            }
            0x8021207c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x802120b0; continue; }
                pc = 0x80212088;
            }
            0x80212088 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80212090; f_801438a0(c);
                pc = 0x80212090;
            }
            0x80212090 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x802120b0; continue; }
                pc = 0x8021209c;
            }
            0x8021209c => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x802120a4; f_80143878(c);
                pc = 0x802120a4;
            }
            0x802120a4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802120b8; continue; }
                pc = 0x802120b0;
            }
            0x802120b0 => {
                c.r[25] = 0x3;
                pc = 0x8021214c; continue;
                pc = 0x802120b8;
            }
            0x802120b8 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x802120c0; f_80143a44(c);
                pc = 0x802120c0;
            }
            0x802120c0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802120d4; continue; }
                pc = 0x802120cc;
            }
            0x802120cc => {
                c.r[25] = 0x4;
                pc = 0x8021214c; continue;
                pc = 0x802120d4;
            }
            0x802120d4 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x802120dc; f_80143a28(c);
                pc = 0x802120dc;
            }
            0x802120dc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212124; continue; }
                pc = 0x802120e4;
            }
            0x802120e4 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x802120ec; f_80143a0c(c);
                pc = 0x802120ec;
            }
            0x802120ec => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212124; continue; }
                pc = 0x802120f4;
            }
            0x802120f4 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x802120fc; f_801439f0(c);
                pc = 0x802120fc;
            }
            0x802120fc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212124; continue; }
                pc = 0x80212104;
            }
            0x80212104 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8021210c; f_801439d4(c);
                pc = 0x8021210c;
            }
            0x8021210c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212124; continue; }
                pc = 0x80212114;
            }
            0x80212114 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8021211c; f_801439b8(c);
                pc = 0x8021211c;
            }
            0x8021211c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8021212c; continue; }
                pc = 0x80212124;
            }
            0x80212124 => {
                c.r[25] = 0x5;
                pc = 0x8021214c; continue;
                pc = 0x8021212c;
            }
            0x8021212c => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80212134; f_80143990(c);
                pc = 0x80212134;
            }
            0x80212134 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212148; continue; }
                pc = 0x80212140;
            }
            0x80212140 => {
                c.r[25] = 0x6;
                pc = 0x8021214c; continue;
                pc = 0x80212148;
            }
            0x80212148 => {
                c.r[25] = 0x7;
                pc = 0x8021214c;
            }
            0x8021214c => {
                c.r[0] = c.r[25].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7);
                if !c.crbit(2) { pc = 0x80212164; continue; }
                pc = 0x80212158;
            }
            0x80212158 => {
                c.r[3] = 0x80370000;
                c.r[17] = c.lwz(c.r[3].wrapping_add(0x5e24));
                pc = 0x802125dc; continue;
                pc = 0x80212164;
            }
            0x80212164 => {
                c.r[7] = c.r[19] | c.r[19];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x4b;
                c.r[6] = 0x0;
                c.lr = 0x8021217c; f_801f4c14(c);
                pc = 0x8021217c;
            }
            0x8021217c => {
                c.r[0] = c.r[25].rotate_left(0) & 0xff;
                c.r[3] = 0x0;
                c.r[18] = c.r[13].wrapping_add(0xffff8558);
                c.cmps(0, c.r[0], 0x4);
                c.stb(c.r[18].wrapping_add(0x5), c.r[3]);
                if c.crbit(2) { pc = 0x802123b0; continue; }
                pc = 0x80212194;
            }
            0x80212194 => {
                if !c.crbit(0) { pc = 0x802121a4; continue; }
                pc = 0x80212198;
            }
            0x80212198 => {
                c.cmps(0, c.r[0], 0x3);
                if !c.crbit(0) { pc = 0x802121b4; continue; }
                pc = 0x802121a0;
            }
            0x802121a0 => {
                pc = 0x802125cc; continue;
                pc = 0x802121a4;
            }
            0x802121a4 => {
                c.cmps(0, c.r[0], 0x6);
                if c.crbit(2) { pc = 0x802125cc; continue; }
                pc = 0x802121ac;
            }
            0x802121ac => {
                if !c.crbit(0) { pc = 0x802125cc; continue; }
                pc = 0x802121b0;
            }
            0x802121b0 => {
                pc = 0x802123bc; continue;
                pc = 0x802121b4;
            }
            0x802121b4 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x802121bc; f_80143940(c);
                pc = 0x802121bc;
            }
            0x802121bc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802122f0; continue; }
                pc = 0x802121c8;
            }
            0x802121c8 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x802121d0; f_80143918(c);
                pc = 0x802121d0;
            }
            0x802121d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802122f0; continue; }
                pc = 0x802121dc;
            }
            0x802121dc => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x802121e4; f_801438f0(c);
                pc = 0x802121e4;
            }
            0x802121e4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802122f0; continue; }
                pc = 0x802121f0;
            }
            0x802121f0 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x802121f8; f_801438c8(c);
                pc = 0x802121f8;
            }
            0x802121f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802122f0; continue; }
                pc = 0x80212204;
            }
            0x80212204 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x8021220c; f_801438a0(c);
                pc = 0x8021220c;
            }
            0x8021220c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802122f0; continue; }
                pc = 0x80212218;
            }
            0x80212218 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x8;
                c.lr = 0x80212224; f_802026e4(c);
                pc = 0x80212224;
            }
            0x80212224 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021223c; continue; }
                pc = 0x80212230;
            }
            0x80212230 => {
                c.r[0] = 0x5;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x8021223c;
            }
            0x8021223c => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x3;
                c.lr = 0x80212248; f_802026e4(c);
                pc = 0x80212248;
            }
            0x80212248 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212260; continue; }
                pc = 0x80212254;
            }
            0x80212254 => {
                c.r[0] = 0x4;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x80212260;
            }
            0x80212260 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x4;
                c.lr = 0x8021226c; f_802026e4(c);
                pc = 0x8021226c;
            }
            0x8021226c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212284; continue; }
                pc = 0x80212278;
            }
            0x80212278 => {
                c.r[0] = 0x4;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x80212284;
            }
            0x80212284 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x6;
                c.lr = 0x80212290; f_802026e4(c);
                pc = 0x80212290;
            }
            0x80212290 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802122a8; continue; }
                pc = 0x8021229c;
            }
            0x8021229c => {
                c.r[0] = 0x3;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x802122a8;
            }
            0x802122a8 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x7;
                c.lr = 0x802122b4; f_802026e4(c);
                pc = 0x802122b4;
            }
            0x802122b4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802122cc; continue; }
                pc = 0x802122c0;
            }
            0x802122c0 => {
                c.r[0] = 0x2;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x802122cc;
            }
            0x802122cc => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x5;
                c.lr = 0x802122d8; f_802026e4(c);
                pc = 0x802122d8;
            }
            0x802122d8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802125cc; continue; }
                pc = 0x802122e4;
            }
            0x802122e4 => {
                c.r[0] = 0x1;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x802122f0;
            }
            0x802122f0 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x802122f8; f_80143940(c);
                pc = 0x802122f8;
            }
            0x802122f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212310; continue; }
                pc = 0x80212304;
            }
            0x80212304 => {
                c.r[0] = 0x5;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x80212310;
            }
            0x80212310 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x80212318; f_80143918(c);
                pc = 0x80212318;
            }
            0x80212318 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212330; continue; }
                pc = 0x80212324;
            }
            0x80212324 => {
                c.r[0] = 0x4;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x80212330;
            }
            0x80212330 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x80212338; f_801438f0(c);
                pc = 0x80212338;
            }
            0x80212338 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212350; continue; }
                pc = 0x80212344;
            }
            0x80212344 => {
                c.r[0] = 0x3;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x80212350;
            }
            0x80212350 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x80212358; f_801438c8(c);
                pc = 0x80212358;
            }
            0x80212358 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212370; continue; }
                pc = 0x80212364;
            }
            0x80212364 => {
                c.r[0] = 0x2;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x80212370;
            }
            0x80212370 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x80212378; f_801438a0(c);
                pc = 0x80212378;
            }
            0x80212378 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212390; continue; }
                pc = 0x80212384;
            }
            0x80212384 => {
                c.r[0] = 0x1;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x80212390;
            }
            0x80212390 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x80212398; f_80143878(c);
                pc = 0x80212398;
            }
            0x80212398 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802125cc; continue; }
                pc = 0x802123a4;
            }
            0x802123a4 => {
                c.r[0] = 0x0;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x802123b0;
            }
            0x802123b0 => {
                c.r[0] = 0x5;
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                pc = 0x802125cc; continue;
                pc = 0x802123bc;
            }
            0x802123bc => {
                c.r[0] = 0x4;
                c.r[3] = c.r[17] | c.r[17];
                c.stb(c.r[18].wrapping_add(0x5), c.r[0]);
                c.lr = 0x802123cc; f_80143a28(c);
                pc = 0x802123cc;
            }
            0x802123cc => {
                c.r[19] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[19]);
                if !c.crbit(1) { pc = 0x802123fc; continue; }
                pc = 0x802123d4;
            }
            0x802123d4 => {
                c.r[3] = 0x80280000;
                c.r[3] = c.r[3].wrapping_add(0xffff9e7c);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                c.lr = 0x802123e4; f_800fa280(c);
                pc = 0x802123e4;
            }
            0x802123e4 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x802123f0; f_80132a38(c);
                pc = 0x802123f0;
            }
            0x802123f0 => {
                c.r[0] = 0x1;
                c.stb(c.r[30].wrapping_add(0x601e), c.r[0]);
                pc = 0x802124d8; continue;
                pc = 0x802123fc;
            }
            0x802123fc => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x80212404; f_80143a0c(c);
                pc = 0x80212404;
            }
            0x80212404 => {
                c.r[19] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[19]);
                if !c.crbit(1) { pc = 0x80212434; continue; }
                pc = 0x8021240c;
            }
            0x8021240c => {
                c.r[3] = 0x80280000;
                c.r[3] = c.r[3].wrapping_add(0xffff9e7c);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                c.lr = 0x8021241c; f_800fa280(c);
                pc = 0x8021241c;
            }
            0x8021241c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80212428; f_80132a38(c);
                pc = 0x80212428;
            }
            0x80212428 => {
                c.r[0] = 0x2;
                c.stb(c.r[30].wrapping_add(0x601e), c.r[0]);
                pc = 0x802124d8; continue;
                pc = 0x80212434;
            }
            0x80212434 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x8021243c; f_801439f0(c);
                pc = 0x8021243c;
            }
            0x8021243c => {
                c.r[19] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[19]);
                if !c.crbit(1) { pc = 0x8021246c; continue; }
                pc = 0x80212444;
            }
            0x80212444 => {
                c.r[3] = 0x80280000;
                c.r[3] = c.r[3].wrapping_add(0xffff9e7c);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.lr = 0x80212454; f_800fa280(c);
                pc = 0x80212454;
            }
            0x80212454 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80212460; f_80132a38(c);
                pc = 0x80212460;
            }
            0x80212460 => {
                c.r[0] = 0x3;
                c.stb(c.r[30].wrapping_add(0x601e), c.r[0]);
                pc = 0x802124d8; continue;
                pc = 0x8021246c;
            }
            0x8021246c => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x80212474; f_801439d4(c);
                pc = 0x80212474;
            }
            0x80212474 => {
                c.r[19] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[19]);
                if !c.crbit(1) { pc = 0x802124a4; continue; }
                pc = 0x8021247c;
            }
            0x8021247c => {
                c.r[3] = 0x80280000;
                c.r[3] = c.r[3].wrapping_add(0xffff9e7c);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x18));
                c.lr = 0x8021248c; f_800fa280(c);
                pc = 0x8021248c;
            }
            0x8021248c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80212498; f_80132a38(c);
                pc = 0x80212498;
            }
            0x80212498 => {
                c.r[0] = 0x6;
                c.stb(c.r[30].wrapping_add(0x601e), c.r[0]);
                pc = 0x802124d8; continue;
                pc = 0x802124a4;
            }
            0x802124a4 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x802124ac; f_801439b8(c);
                pc = 0x802124ac;
            }
            0x802124ac => {
                c.r[19] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[19]);
                if !c.crbit(1) { pc = 0x802124d8; continue; }
                pc = 0x802124b4;
            }
            0x802124b4 => {
                c.r[3] = 0x80280000;
                c.r[3] = c.r[3].wrapping_add(0xffff9e7c);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x802124c4; f_800fa280(c);
                pc = 0x802124c4;
            }
            0x802124c4 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x802124d0; f_80132a38(c);
                pc = 0x802124d0;
            }
            0x802124d0 => {
                c.r[0] = 0x4;
                c.stb(c.r[30].wrapping_add(0x601e), c.r[0]);
                pc = 0x802124d8;
            }
            0x802124d8 => {
                c.r[0] = c.r[19] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80212558; continue; }
                pc = 0x802124e0;
            }
            0x802124e0 => {
                c.r[0] = c.r[19] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x802124f4; continue; }
                pc = 0x802124ec;
            }
            0x802124ec => {
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(2) { pc = 0x8021251c; continue; }
                pc = 0x802124f4;
            }
            0x802124f4 => {
                c.r[3] = 0x76bd;
                c.lr = 0x802124fc; f_800fa280(c);
                pc = 0x802124fc;
            }
            0x802124fc => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xe;
                c.lr = 0x80212508; f_80132a38(c);
                pc = 0x80212508;
            }
            0x80212508 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x601e));
                c.r[3] = c.r[0].rotate_left(0) & 0xf;
                c.r[0] = c.r[3].wrapping_add(0x15);
                c.stb(c.r[30].wrapping_add(0x601e), c.r[0]);
                pc = 0x80212540; continue;
                pc = 0x8021251c;
            }
            0x8021251c => {
                c.r[3] = 0x7628;
                c.lr = 0x80212524; f_800fa280(c);
                pc = 0x80212524;
            }
            0x80212524 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xe;
                c.lr = 0x80212530; f_80132a38(c);
                pc = 0x80212530;
            }
            0x80212530 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x601e));
                c.r[3] = c.r[0].rotate_left(0) & 0xf;
                c.r[0] = c.r[3].wrapping_add(0x2d);
                c.stb(c.r[30].wrapping_add(0x60a4), c.r[0]);
                pc = 0x80212540;
            }
            0x80212540 => {
                c.r[3] = 0x7629;
                c.lr = 0x80212548; f_800fa280(c);
                pc = 0x80212548;
            }
            0x80212548 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x41;
                c.lr = 0x80212554; f_80132a38(c);
                pc = 0x80212554;
            }
            0x80212554 => {
                pc = 0x802125cc; continue;
                pc = 0x80212558;
            }
            0x80212558 => {
                c.r[0] = c.r[19] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8021256c; continue; }
                pc = 0x80212564;
            }
            0x80212564 => {
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(2) { pc = 0x80212594; continue; }
                pc = 0x8021256c;
            }
            0x8021256c => {
                c.r[3] = 0x76bd;
                c.lr = 0x80212574; f_800fa280(c);
                pc = 0x80212574;
            }
            0x80212574 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xe;
                c.lr = 0x80212580; f_80132a38(c);
                pc = 0x80212580;
            }
            0x80212580 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x601e));
                c.r[3] = c.r[0].rotate_left(0) & 0xf;
                c.r[0] = c.r[3].wrapping_add(0xe);
                c.stb(c.r[30].wrapping_add(0x60a4), c.r[0]);
                pc = 0x802125b8; continue;
                pc = 0x80212594;
            }
            0x80212594 => {
                c.r[3] = 0x7626;
                c.lr = 0x8021259c; f_800fa280(c);
                pc = 0x8021259c;
            }
            0x8021259c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xe;
                c.lr = 0x802125a8; f_80132a38(c);
                pc = 0x802125a8;
            }
            0x802125a8 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x601e));
                c.r[3] = c.r[0].rotate_left(0) & 0xf;
                c.r[0] = c.r[3].wrapping_add(0x26);
                c.stb(c.r[30].wrapping_add(0x60a4), c.r[0]);
                pc = 0x802125b8;
            }
            0x802125b8 => {
                c.r[3] = 0x7627;
                c.lr = 0x802125c0; f_800fa280(c);
                pc = 0x802125c0;
            }
            0x802125c0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x41;
                c.lr = 0x802125cc; f_80132a38(c);
                pc = 0x802125cc;
            }
            0x802125cc => {
                c.r[3] = 0x80370000;
                c.r[0] = c.r[25].rotate_left(2) & 0x3fc;
                c.r[3] = c.r[3].wrapping_add(0x5e24);
                c.r[17] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x802125dc;
            }
            0x802125dc => {
                c.r[0] = 0x0;
                c.r[3] = c.r[29] | c.r[29];
                c.stb(c.r[13].wrapping_add(0xffffadf4), c.r[0]);
                c.lr = 0x802125ec; f_801f8000(c);
                pc = 0x802125ec;
            }
            0x802125ec => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x22;
                c.lr = 0x802125f8; f_80132a38(c);
                pc = 0x802125f8;
            }
            0x802125f8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80212600; f_801f8100(c);
                pc = 0x80212600;
            }
            0x80212600 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x23;
                c.lr = 0x8021260c; f_80132a38(c);
                pc = 0x8021260c;
            }
            0x8021260c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80212614; f_801f8100(c);
                pc = 0x80212614;
            }
            0x80212614 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x13;
                c.lr = 0x80212620; f_80132a38(c);
                pc = 0x80212620;
            }
            0x80212620 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80212634; f_80142cf4(c);
                pc = 0x80212634;
            }
            0x80212634 => {
                c.lr = 0x80212638; f_800fa280(c);
                pc = 0x80212638;
            }
            0x80212638 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x29;
                c.lr = 0x80212644; f_80132a38(c);
                pc = 0x80212644;
            }
            0x80212644 => {
                c.r[3] = 0x1;
                c.lr = 0x8021264c; f_801ef8f4(c);
                pc = 0x8021264c;
            }
            0x8021264c => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x80212664; continue; }
                pc = 0x80212654;
            }
            0x80212654 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[20] | c.r[20];
                c.r[5] = 0x1;
                c.lr = 0x80212664; f_80265598(c);
                pc = 0x80212664;
            }
            0x80212664 => {
                c.r[3] = c.r[21] | c.r[21];
                c.r[4] = c.r[17] | c.r[17];
                c.r[5] = 0x1;
                c.lr = 0x80212674; f_80211b94(c);
                pc = 0x80212674;
            }
            0x80212674 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x80212690; continue; }
                pc = 0x8021267c;
            }
            0x8021267c => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[24] as u16 as i16 as i32 as u32;
                c.r[5] = 0x1;
                c.lr = 0x80212690; f_801299c8(c);
                pc = 0x80212690;
            }
            0x80212690 => {
                c.stb(c.r[30].wrapping_add(0x601e), c.r[26]);
                c.stb(c.r[30].wrapping_add(0x60a4), c.r[27]);
                c.lr = 0x8021269c; f_801da7ac(c);
                pc = 0x8021269c;
            }
            0x8021269c => {
                c.r[3] = 0x0;
                c.lr = 0x802126a4; f_80261b68(c);
                pc = 0x802126a4;
            }
            0x802126a4 => {
                c.r[3] = 0x0;
                c.lr = 0x802126ac; f_80261e7c(c);
                pc = 0x802126ac;
            }
            0x802126ac => {
                c.lr = 0x802126b0; f_8026246c(c);
                pc = 0x802126b0;
            }
            0x802126b0 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 17..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { return; }
                return;
            }
            _ => panic!("f_80211e18: bad pc {:#x}", pc),
        }
    }
}

/// fightSeqGetItemType (0x802126c4, 380 bytes)
#[inline(never)]
pub fn f_802126c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802126c4) { return; }
        if c.prof { crate::prof::enter(c, 0x802126c4); f_802126c4_b(c, 0x802126c4); crate::prof::exit(c, 0x802126c4); return; }
    }
    f_802126c4_b(c, 0x802126c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802126c4_at(c: &mut Cpu, pc: u32) { f_802126c4_b(c, pc) }
#[inline(always)]
fn f_802126c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802126c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802126c4, pc); }
        match pc {
            0x802126c4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x802126dc; f_801440a0(c);
                pc = 0x802126dc;
            }
            0x802126dc => {
                c.lr = 0x802126e0; f_80143dfc(c);
                pc = 0x802126e0;
            }
            0x802126e0 => {
                c.lr = 0x802126e4; f_80143a94(c);
                pc = 0x802126e4;
            }
            0x802126e4 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if !c.crbit(2) { pc = 0x802126f4; continue; }
                pc = 0x802126ec;
            }
            0x802126ec => {
                c.r[3] = 0x7;
                pc = 0x8021282c; continue;
                pc = 0x802126f4;
            }
            0x802126f4 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x13);
                if !c.crbit(2) { pc = 0x80212708; continue; }
                pc = 0x80212700;
            }
            0x80212700 => {
                c.r[3] = 0x1;
                pc = 0x8021282c; continue;
                pc = 0x80212708;
            }
            0x80212708 => {
                c.lr = 0x8021270c; f_801437e0(c);
                pc = 0x8021270c;
            }
            0x8021270c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8021271c; continue; }
                pc = 0x80212714;
            }
            0x80212714 => {
                c.r[3] = 0x2;
                pc = 0x8021282c; continue;
                pc = 0x8021271c;
            }
            0x8021271c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80212724; f_80143940(c);
                pc = 0x80212724;
            }
            0x80212724 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80212794; continue; }
                pc = 0x80212730;
            }
            0x80212730 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80212738; f_80143918(c);
                pc = 0x80212738;
            }
            0x80212738 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80212794; continue; }
                pc = 0x80212744;
            }
            0x80212744 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8021274c; f_801438f0(c);
                pc = 0x8021274c;
            }
            0x8021274c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80212794; continue; }
                pc = 0x80212758;
            }
            0x80212758 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80212760; f_801438c8(c);
                pc = 0x80212760;
            }
            0x80212760 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80212794; continue; }
                pc = 0x8021276c;
            }
            0x8021276c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80212774; f_801438a0(c);
                pc = 0x80212774;
            }
            0x80212774 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80212794; continue; }
                pc = 0x80212780;
            }
            0x80212780 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80212788; f_80143878(c);
                pc = 0x80212788;
            }
            0x80212788 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021279c; continue; }
                pc = 0x80212794;
            }
            0x80212794 => {
                c.r[3] = 0x3;
                pc = 0x8021282c; continue;
                pc = 0x8021279c;
            }
            0x8021279c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802127a4; f_80143a44(c);
                pc = 0x802127a4;
            }
            0x802127a4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802127b8; continue; }
                pc = 0x802127b0;
            }
            0x802127b0 => {
                c.r[3] = 0x4;
                pc = 0x8021282c; continue;
                pc = 0x802127b8;
            }
            0x802127b8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802127c0; f_80143a28(c);
                pc = 0x802127c0;
            }
            0x802127c0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212808; continue; }
                pc = 0x802127c8;
            }
            0x802127c8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802127d0; f_80143a0c(c);
                pc = 0x802127d0;
            }
            0x802127d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212808; continue; }
                pc = 0x802127d8;
            }
            0x802127d8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802127e0; f_801439f0(c);
                pc = 0x802127e0;
            }
            0x802127e0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212808; continue; }
                pc = 0x802127e8;
            }
            0x802127e8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802127f0; f_801439d4(c);
                pc = 0x802127f0;
            }
            0x802127f0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212808; continue; }
                pc = 0x802127f8;
            }
            0x802127f8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80212800; f_801439b8(c);
                pc = 0x80212800;
            }
            0x80212800 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80212810; continue; }
                pc = 0x80212808;
            }
            0x80212808 => {
                c.r[3] = 0x5;
                pc = 0x8021282c; continue;
                pc = 0x80212810;
            }
            0x80212810 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80212818; f_80143990(c);
                pc = 0x80212818;
            }
            0x80212818 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.r[0] = c.addc(c.r[0], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[0], ci); }
                c.r[3] = c.r[3].wrapping_add(0x7);
                pc = 0x8021282c;
            }
            0x8021282c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802126c4: bad pc {:#x}", pc),
        }
    }
}

/// fightSeqSpecificationActionCounterInit (0x80212840, 144 bytes)
#[inline(never)]
pub fn f_80212840(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80212840) { return; }
        if c.prof { crate::prof::enter(c, 0x80212840); f_80212840_b(c, 0x80212840); crate::prof::exit(c, 0x80212840); return; }
    }
    f_80212840_b(c, 0x80212840)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80212840_at(c: &mut Cpu, pc: u32) { f_80212840_b(c, pc) }
#[inline(always)]
fn f_80212840_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80212840;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80212840, pc); }
        match pc {
            0x80212840 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x2e;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8021285c; f_802026e4(c);
                pc = 0x8021285c;
            }
            0x8021285c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212874; continue; }
                pc = 0x80212868;
            }
            0x80212868 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2e;
                c.lr = 0x80212874; f_80202810(c);
                pc = 0x80212874;
            }
            0x80212874 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x15;
                c.lr = 0x80212880; f_802026e4(c);
                pc = 0x80212880;
            }
            0x80212880 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212898; continue; }
                pc = 0x8021288c;
            }
            0x8021288c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x15;
                c.lr = 0x80212898; f_80202810(c);
                pc = 0x80212898;
            }
            0x80212898 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x28;
                c.lr = 0x802128a4; f_802026e4(c);
                pc = 0x802128a4;
            }
            0x802128a4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802128bc; continue; }
                pc = 0x802128b0;
            }
            0x802128b0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x28;
                c.lr = 0x802128bc; f_80202810(c);
                pc = 0x802128bc;
            }
            0x802128bc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80212840: bad pc {:#x}", pc),
        }
    }
}

/// fn_802128D0 (0x802128d0, 1180 bytes)
#[inline(never)]
pub fn f_802128d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802128d0) { return; }
        if c.prof { crate::prof::enter(c, 0x802128d0); f_802128d0_b(c, 0x802128d0); crate::prof::exit(c, 0x802128d0); return; }
    }
    f_802128d0_b(c, 0x802128d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802128d0_at(c: &mut Cpu, pc: u32) { f_802128d0_b(c, pc) }
#[inline(always)]
fn f_802128d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802128d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802128d0, pc); }
        match pc {
            0x802128d0 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.r[24] = c.r[4] | c.r[4];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x802128fc; f_801f54a4(c);
                pc = 0x802128fc;
            }
            0x802128fc => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x0;
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x80212914; f_8011beb4(c);
                pc = 0x80212914;
            }
            0x80212914 => {
                c.r[25] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80212924; f_801f025c(c);
                pc = 0x80212924;
            }
            0x80212924 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80212938; f_801f025c(c);
                pc = 0x80212938;
            }
            0x80212938 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.lr = 0x80212948; f_801f025c(c);
                pc = 0x80212948;
            }
            0x80212948 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[27] = c.r[0] | c.r[0];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8021295c; f_801f025c(c);
                pc = 0x8021295c;
            }
            0x8021295c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[26] = c.r[0] | c.r[0];
                c.lr = 0x8021296c; f_802062fc(c);
                pc = 0x8021296c;
            }
            0x8021296c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80212d58; continue; }
                pc = 0x80212974;
            }
            0x80212974 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0xfe;
                c.r[6] = 0x0;
                c.lr = 0x80212988; f_8012640c(c);
                pc = 0x80212988;
            }
            0x80212988 => {
                c.lr = 0x8021298c; f_801f1170(c);
                pc = 0x8021298c;
            }
            0x8021298c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80212d58; continue; }
                pc = 0x80212994;
            }
            0x80212994 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x51;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x802129ac; f_801f4c14(c);
                pc = 0x802129ac;
            }
            0x802129ac => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x52;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x802129c4; f_801f4c14(c);
                pc = 0x802129c4;
            }
            0x802129c4 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x53;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x802129dc; f_801f4c14(c);
                pc = 0x802129dc;
            }
            0x802129dc => {
                c.r[0] = 0x0;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x6), c.r[0]);
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x2a;
                c.stw(c.r[13].wrapping_add(0xffffae24), c.r[0]);
                c.lr = 0x802129f8; f_802026e4(c);
                pc = 0x802129f8;
            }
            0x802129f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212b00; continue; }
                pc = 0x80212a04;
            }
            0x80212a04 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xa5);
                if c.crbit(2) { pc = 0x80212b00; continue; }
                pc = 0x80212a10;
            }
            0x80212a10 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x2a;
                c.lr = 0x80212a1c; f_80201c58(c);
                pc = 0x80212a1c;
            }
            0x80212a1c => {
                c.r[26] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x2a;
                c.lr = 0x80212a2c; f_80201890(c);
                pc = 0x80212a2c;
            }
            0x80212a2c => {
                c.r[24] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80212a38; f_80205b8c(c);
                pc = 0x80212a38;
            }
            0x80212a38 => {
                c.r[6] = c.r[24].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x80212a48; f_8012640c(c);
                pc = 0x80212a48;
            }
            0x80212a48 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[27]);
                if c.crbit(2) { pc = 0x80212a64; continue; }
                pc = 0x80212a58;
            }
            0x80212a58 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x2a;
                c.lr = 0x80212a64; f_80202810(c);
                pc = 0x80212a64;
            }
            0x80212a64 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.r[8] = 0x1;
                c.r[9] = 0xffffffff;
                c.lr = 0x80212a84; f_8022b2cc(c);
                pc = 0x80212a84;
            }
            0x80212a84 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x80212a90; f_801f0134(c);
                pc = 0x80212a90;
            }
            0x80212a90 => {
                c.r[0] = 0x1;
                c.r[4] = 0x80370000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[9] = c.r[3] | c.r[3];
                c.r[7] = c.r[4].wrapping_add(0x5ca8);
                c.r[3] = c.r[28] | c.r[28];
                c.r[8] = c.r[27] | c.r[27];
                c.r[10] = c.r[24] as u8 as i8 as i32 as u32;
                c.r[4] = 0x0;
                c.r[5] = 0x13;
                c.r[6] = 0x0;
                c.lr = 0x80212ac0; f_80204f6c(c);
                pc = 0x80212ac0;
            }
            0x80212ac0 => {
                c.r[24] = c.r[27] | c.r[27];
                c.r[7] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x80212adc; f_801f4c14(c);
                pc = 0x80212adc;
            }
            0x80212adc => {
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.lr = 0x80212ae8; f_801f025c(c);
                pc = 0x80212ae8;
            }
            0x80212ae8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[27] = c.r[0] | c.r[0];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x80212afc; f_801f025c(c);
                pc = 0x80212afc;
            }
            0x80212afc => {
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x80212b00;
            }
            0x80212b00 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xa5);
                if c.crbit(2) { pc = 0x80212b14; continue; }
                pc = 0x80212b0c;
            }
            0x80212b0c => {
                c.cmpu(0, c.r[0], 0x164);
                if !c.crbit(2) { pc = 0x80212b20; continue; }
                pc = 0x80212b14;
            }
            0x80212b14 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[0] = c.r[0] | 0x800;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                pc = 0x80212b20;
            }
            0x80212b20 => {
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x0;
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x80212b34; f_8011beb4(c);
                pc = 0x80212b34;
            }
            0x80212b34 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x91);
                if c.crbit(2) { pc = 0x80212b68; continue; }
                pc = 0x80212b40;
            }
            0x80212b40 => {
                c.cmpu(0, c.r[0], 0x27);
                if c.crbit(2) { pc = 0x80212b68; continue; }
                pc = 0x80212b48;
            }
            0x80212b48 => {
                c.cmpu(0, c.r[0], 0x4b);
                if c.crbit(2) { pc = 0x80212b68; continue; }
                pc = 0x80212b50;
            }
            0x80212b50 => {
                c.cmpu(0, c.r[0], 0x97);
                if c.crbit(2) { pc = 0x80212b68; continue; }
                pc = 0x80212b58;
            }
            0x80212b58 => {
                c.cmpu(0, c.r[0], 0x9b);
                if c.crbit(2) { pc = 0x80212b68; continue; }
                pc = 0x80212b60;
            }
            0x80212b60 => {
                c.cmpu(0, c.r[0], 0x1a);
                if !c.crbit(2) { pc = 0x80212b70; continue; }
                pc = 0x80212b68;
            }
            0x80212b68 => {
                c.r[0] = 0x1;
                pc = 0x80212b74; continue;
                pc = 0x80212b70;
            }
            0x80212b70 => {
                c.r[0] = 0x0;
                pc = 0x80212b74;
            }
            0x80212b74 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212c0c; continue; }
                pc = 0x80212b80;
            }
            0x80212b80 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x22;
                c.lr = 0x80212b8c; f_802026e4(c);
                pc = 0x80212b8c;
            }
            0x80212b8c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212c0c; continue; }
                pc = 0x80212b94;
            }
            0x80212b94 => {
                c.cmpu(0, c.r[25], 0x97);
                if !c.crbit(2) { pc = 0x80212c60; continue; }
                pc = 0x80212b9c;
            }
            0x80212b9c => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x80212ba8; f_801f453c(c);
                pc = 0x80212ba8;
            }
            0x80212ba8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212c60; continue; }
                pc = 0x80212bb4;
            }
            0x80212bb4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[24] | c.r[24];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x1;
                c.r[9] = 0xffffffff;
                c.lr = 0x80212bd4; f_8022b2cc(c);
                pc = 0x80212bd4;
            }
            0x80212bd4 => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if c.crbit(2) { pc = 0x80212c60; continue; }
                pc = 0x80212bdc;
            }
            0x80212bdc => {
                c.r[27] = c.r[25] | c.r[25];
                c.r[7] = c.r[25] | c.r[25];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80212bf8; f_801f4c14(c);
                pc = 0x80212bf8;
            }
            0x80212bf8 => {
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0x2;
                c.lr = 0x80212c04; f_801f025c(c);
                pc = 0x80212c04;
            }
            0x80212c04 => {
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x80212c60; continue;
                pc = 0x80212c0c;
            }
            0x80212c0c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[24] | c.r[24];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x1;
                c.r[9] = 0xffffffff;
                c.lr = 0x80212c2c; f_8022b2cc(c);
                pc = 0x80212c2c;
            }
            0x80212c2c => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if c.crbit(2) { pc = 0x80212c60; continue; }
                pc = 0x80212c34;
            }
            0x80212c34 => {
                c.r[27] = c.r[25] | c.r[25];
                c.r[7] = c.r[25] | c.r[25];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80212c50; f_801f4c14(c);
                pc = 0x80212c50;
            }
            0x80212c50 => {
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0x2;
                c.lr = 0x80212c5c; f_801f025c(c);
                pc = 0x80212c5c;
            }
            0x80212c5c => {
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x80212c60;
            }
            0x80212c60 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x80212c68; f_802062fc(c);
                pc = 0x80212c68;
            }
            0x80212c68 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212ce0; continue; }
                pc = 0x80212c70;
            }
            0x80212c70 => {
                c.cmpu(0, c.r[30], c.r[26]);
                if !c.crbit(2) { pc = 0x80212c94; continue; }
                pc = 0x80212c78;
            }
            0x80212c78 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0xf;
                c.lr = 0x80212c84; f_801f025c(c);
                pc = 0x80212c84;
            }
            0x80212c84 => {
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x80212c8c; f_802062fc(c);
                pc = 0x80212c8c;
            }
            0x80212c8c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212cbc; continue; }
                pc = 0x80212c94;
            }
            0x80212c94 => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0xe;
                c.lr = 0x80212ca0; f_801f025c(c);
                pc = 0x80212ca0;
            }
            0x80212ca0 => {
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                if c.crbit(2) { pc = 0x80212d58; continue; }
                pc = 0x80212ca8;
            }
            0x80212ca8 => {
                c.lr = 0x80212cac; f_802062fc(c);
                pc = 0x80212cac;
            }
            0x80212cac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80212cbc; continue; }
                pc = 0x80212cb4;
            }
            0x80212cb4 => {
                pc = 0x80212d58; continue;
                pc = 0x80212cb8;
            }
            0x80212cb8 => {
                pc = 0x80212d58; continue;
                pc = 0x80212cbc;
            }
            0x80212cbc => {
                c.r[7] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80212cd4; f_801f4c14(c);
                pc = 0x80212cd4;
            }
            0x80212cd4 => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x2;
                c.lr = 0x80212ce0; f_801f025c(c);
                pc = 0x80212ce0;
            }
            0x80212ce0 => {
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x0;
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x80212cf4; f_8011beb4(c);
                pc = 0x80212cf4;
            }
            0x80212cf4 => {
                c.r[0] = 0x0;
                c.r[25] = c.r[3].rotate_left(0) & 0xffff;
                c.stb(c.r[13].wrapping_add(0xffffadf4), c.r[0]);
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80212d14; f_8011beb4(c);
                pc = 0x80212d14;
            }
            0x80212d14 => {
                c.lr = 0x80212d18; f_800fa280(c);
                pc = 0x80212d18;
            }
            0x80212d18 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x28;
                c.lr = 0x80212d24; f_80132a38(c);
                pc = 0x80212d24;
            }
            0x80212d24 => {
                c.r[3] = 0x80380000;
                c.r[0] = c.r[25].rotate_left(2) & 0x3fffc;
                c.r[4] = c.r[3].wrapping_add(0xffff9bff);
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                c.r[5] = 0x1;
                c.lr = 0x80212d40; f_80211b94(c);
                pc = 0x80212d40;
            }
            0x80212d40 => {
                c.lr = 0x80212d44; f_801da7ac(c);
                pc = 0x80212d44;
            }
            0x80212d44 => {
                c.r[3] = 0x0;
                c.lr = 0x80212d4c; f_80261b68(c);
                pc = 0x80212d4c;
            }
            0x80212d4c => {
                c.r[3] = 0x0;
                c.lr = 0x80212d54; f_80261e7c(c);
                pc = 0x80212d54;
            }
            0x80212d54 => {
                c.lr = 0x80212d58; f_8026246c(c);
                pc = 0x80212d58;
            }
            0x80212d58 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_802128d0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80212D6C (0x80212d6c, 1004 bytes)
#[inline(never)]
pub fn f_80212d6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80212d6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80212d6c); f_80212d6c_b(c, 0x80212d6c); crate::prof::exit(c, 0x80212d6c); return; }
    }
    f_80212d6c_b(c, 0x80212d6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80212d6c_at(c: &mut Cpu, pc: u32) { f_80212d6c_b(c, pc) }
#[inline(always)]
fn f_80212d6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80212d6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80212d6c, pc); }
        match pc {
            0x80212d6c => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = 0x0;
                c.r[27] = 0x0;
                c.lr = 0x80212d98; f_801f54a4(c);
                pc = 0x80212d98;
            }
            0x80212d98 => {
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x80212db0; f_801f54a4(c);
                pc = 0x80212db0;
            }
            0x80212db0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80212dc4; f_801f4354(c);
                pc = 0x80212dc4;
            }
            0x80212dc4 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x2e;
                c.lr = 0x80212dd8; f_802026e4(c);
                pc = 0x80212dd8;
            }
            0x80212dd8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212df0; continue; }
                pc = 0x80212de4;
            }
            0x80212de4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2e;
                c.lr = 0x80212df0; f_80202810(c);
                pc = 0x80212df0;
            }
            0x80212df0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x15;
                c.lr = 0x80212dfc; f_802026e4(c);
                pc = 0x80212dfc;
            }
            0x80212dfc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212e14; continue; }
                pc = 0x80212e08;
            }
            0x80212e08 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x15;
                c.lr = 0x80212e14; f_80202810(c);
                pc = 0x80212e14;
            }
            0x80212e14 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x28;
                c.lr = 0x80212e20; f_802026e4(c);
                pc = 0x80212e20;
            }
            0x80212e20 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212e38; continue; }
                pc = 0x80212e2c;
            }
            0x80212e2c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x28;
                c.lr = 0x80212e38; f_80202810(c);
                pc = 0x80212e38;
            }
            0x80212e38 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80212e40; f_80205b8c(c);
                pc = 0x80212e40;
            }
            0x80212e40 => {
                c.lr = 0x80212e44; f_8011fc74(c);
                pc = 0x80212e44;
            }
            0x80212e44 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212f38; continue; }
                pc = 0x80212e50;
            }
            0x80212e50 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80212e58; f_80205b8c(c);
                pc = 0x80212e58;
            }
            0x80212e58 => {
                c.lr = 0x80212e5c; f_8011f634(c);
                pc = 0x80212e5c;
            }
            0x80212e5c => {
                c.r[25] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3e;
                c.lr = 0x80212e6c; f_802026e4(c);
                pc = 0x80212e6c;
            }
            0x80212e6c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212f0c; continue; }
                pc = 0x80212e78;
            }
            0x80212e78 => {
                c.lr = 0x80212e7c; f_800e0c54(c);
                pc = 0x80212e7c;
            }
            0x80212e7c => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = 0x64;
                c.r[3] = divw(c.r[5], c.r[4]);
                c.r[0] = c.r[25].rotate_left(0) & 0xff;
                c.r[3] = c.r[3].wrapping_mul(c.r[4]);
                c.r[3] = c.r[5].wrapping_sub(c.r[3]);
                c.cmps(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x80212ec4; continue; }
                pc = 0x80212e9c;
            }
            0x80212e9c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[29] = 0x7707;
                c.r[27] = 0x1;
                c.r[4] = 0x8;
                c.lr = 0x80212eb0; f_802026e4(c);
                pc = 0x80212eb0;
            }
            0x80212eb0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212f60; continue; }
                pc = 0x80212ebc;
            }
            0x80212ebc => {
                c.r[28] = 0x1;
                pc = 0x80212f60; continue;
                pc = 0x80212ec4;
            }
            0x80212ec4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[29] = 0x7708;
                c.r[4] = 0x8;
                c.lr = 0x80212ed4; f_802026e4(c);
                pc = 0x80212ed4;
            }
            0x80212ed4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212f60; continue; }
                pc = 0x80212ee0;
            }
            0x80212ee0 => {
                c.lr = 0x80212ee4; f_800e0c54(c);
                pc = 0x80212ee4;
            }
            0x80212ee4 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = 0x64;
                c.r[3] = divw(c.r[5], c.r[4]);
                c.r[0] = c.r[25].rotate_left(31) & 0x7f;
                c.r[3] = c.r[3].wrapping_mul(c.r[4]);
                c.r[3] = c.r[5].wrapping_sub(c.r[3]);
                c.cmps(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x80212f60; continue; }
                pc = 0x80212f04;
            }
            0x80212f04 => {
                c.r[28] = 0x1;
                pc = 0x80212f60; continue;
                pc = 0x80212f0c;
            }
            0x80212f0c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x80212f18; f_802026e4(c);
                pc = 0x80212f18;
            }
            0x80212f18 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212f30; continue; }
                pc = 0x80212f24;
            }
            0x80212f24 => {
                c.r[29] = 0x7706;
                c.r[28] = 0x1;
                pc = 0x80212f60; continue;
                pc = 0x80212f30;
            }
            0x80212f30 => {
                c.r[29] = 0x771d;
                pc = 0x80212f60; continue;
                pc = 0x80212f38;
            }
            0x80212f38 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x80212f44; f_802026e4(c);
                pc = 0x80212f44;
            }
            0x80212f44 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80212f5c; continue; }
                pc = 0x80212f50;
            }
            0x80212f50 => {
                c.r[29] = 0x7706;
                c.r[28] = 0x1;
                pc = 0x80212f60; continue;
                pc = 0x80212f5c;
            }
            0x80212f5c => {
                c.r[29] = 0x7709;
                pc = 0x80212f60;
            }
            0x80212f60 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x80212f6c; f_801fb974(c);
                pc = 0x80212f6c;
            }
            0x80212f6c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80212f84; f_802080a8(c);
                pc = 0x80212f84;
            }
            0x80212f84 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80212f8c; f_801f8100(c);
                pc = 0x80212f8c;
            }
            0x80212f8c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x13;
                c.lr = 0x80212f98; f_80132a38(c);
                pc = 0x80212f98;
            }
            0x80212f98 => {
                c.r[3] = 0x1;
                c.lr = 0x80212fa0; f_801ef8f4(c);
                pc = 0x80212fa0;
            }
            0x80212fa0 => {
                c.r[3] = 0x7704;
                c.lr = 0x80212fa8; f_802624cc(c);
                pc = 0x80212fa8;
            }
            0x80212fa8 => {
                c.r[3] = 0x40;
                c.lr = 0x80212fb0; f_801f000c(c);
                pc = 0x80212fb0;
            }
            0x80212fb0 => {
                c.lr = 0x80212fb4; f_8026246c(c);
                pc = 0x80212fb4;
            }
            0x80212fb4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1;
                c.lr = 0x80212fc0; f_801fb974(c);
                pc = 0x80212fc0;
            }
            0x80212fc0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x1;
                c.lr = 0x80212fd0; f_80265598(c);
                pc = 0x80212fd0;
            }
            0x80212fd0 => {
                c.r[3] = 0x7705;
                c.lr = 0x80212fd8; f_802624cc(c);
                pc = 0x80212fd8;
            }
            0x80212fd8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x2;
                c.lr = 0x80212fe4; f_801fb974(c);
                pc = 0x80212fe4;
            }
            0x80212fe4 => {
                c.lr = 0x80212fe8; f_8026246c(c);
                pc = 0x80212fe8;
            }
            0x80212fe8 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213034; continue; }
                pc = 0x80212ff4;
            }
            0x80212ff4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x80213000; f_80202810(c);
                pc = 0x80213000;
            }
            0x80213000 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x17;
                c.lr = 0x8021300c; f_80202810(c);
                pc = 0x8021300c;
            }
            0x8021300c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80213014; f_801fecd4(c);
                pc = 0x80213014;
            }
            0x80213014 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213034; continue; }
                pc = 0x80213020;
            }
            0x80213020 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7c;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80213034; f_801fe7ec(c);
                pc = 0x80213034;
            }
            0x80213034 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802130c4; continue; }
                pc = 0x80213040;
            }
            0x80213040 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3e;
                c.lr = 0x8021304c; f_80202810(c);
                pc = 0x8021304c;
            }
            0x8021304c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80213054; f_801fecd4(c);
                pc = 0x80213054;
            }
            0x80213054 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213074; continue; }
                pc = 0x80213060;
            }
            0x80213060 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0xc8;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80213074; f_801fe7ec(c);
                pc = 0x80213074;
            }
            0x80213074 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8021307c; f_80204a10(c);
                pc = 0x8021307c;
            }
            0x8021307c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802130c4; continue; }
                pc = 0x80213088;
            }
            0x80213088 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80213090; f_80205b8c(c);
                pc = 0x80213090;
            }
            0x80213090 => {
                c.r[4] = 0x0;
                c.r[5] = 0x2;
                c.lr = 0x8021309c; f_8011f910(c);
                pc = 0x8021309c;
            }
            0x8021309c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802130a4; f_801fecd4(c);
                pc = 0x802130a4;
            }
            0x802130a4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802130c4; continue; }
                pc = 0x802130b0;
            }
            0x802130b0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0xc5;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802130c4; f_801fe7ec(c);
                pc = 0x802130c4;
            }
            0x802130c4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = 0x1;
                c.lr = 0x802130dc; f_802080a8(c);
                pc = 0x802130dc;
            }
            0x802130dc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = 0x0;
                c.r[7] = 0x2;
                c.lr = 0x802130f4; f_802080a8(c);
                pc = 0x802130f4;
            }
            0x802130f4 => {
                c.lr = 0x802130f8; f_8026246c(c);
                pc = 0x802130f8;
            }
            0x802130f8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x0;
                c.lr = 0x80213108; f_8026532c(c);
                pc = 0x80213108;
            }
            0x80213108 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x3;
                c.lr = 0x80213114; f_801fb974(c);
                pc = 0x80213114;
            }
            0x80213114 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = 0x0;
                c.r[7] = 0x3;
                c.lr = 0x8021312c; f_802080a8(c);
                pc = 0x8021312c;
            }
            0x8021312c => {
                c.lr = 0x80213130; f_801da7ac(c);
                pc = 0x80213130;
            }
            0x80213130 => {
                c.r[3] = 0x0;
                c.lr = 0x80213138; f_80261b68(c);
                pc = 0x80213138;
            }
            0x80213138 => {
                c.r[3] = 0x0;
                c.lr = 0x80213140; f_80261e7c(c);
                pc = 0x80213140;
            }
            0x80213140 => {
                c.lr = 0x80213144; f_8026246c(c);
                pc = 0x80213144;
            }
            0x80213144 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80212d6c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80213158 (0x80213158, 280 bytes)
#[inline(never)]
pub fn f_80213158(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213158) { return; }
        if c.prof { crate::prof::enter(c, 0x80213158); f_80213158_b(c, 0x80213158); crate::prof::exit(c, 0x80213158); return; }
    }
    f_80213158_b(c, 0x80213158)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213158_at(c: &mut Cpu, pc: u32) { f_80213158_b(c, pc) }
#[inline(always)]
fn f_80213158_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213158;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213158, pc); }
        match pc {
            0x80213158 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80213180; f_801f54a4(c);
                pc = 0x80213180;
            }
            0x80213180 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[7] = c.r[31] | c.r[31];
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x802131a0; f_801f4c14(c);
                pc = 0x802131a0;
            }
            0x802131a0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802131a8; f_802037dc(c);
                pc = 0x802131a8;
            }
            0x802131a8 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x802131b4; f_80132a38(c);
                pc = 0x802131b4;
            }
            0x802131b4 => {
                c.r[0] = 0x0;
                c.r[3] = 0x1;
                c.stb(c.r[13].wrapping_add(0xffffadf4), c.r[0]);
                c.lr = 0x802131c4; f_801ef8f4(c);
                pc = 0x802131c4;
            }
            0x802131c4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2e;
                c.lr = 0x802131d0; f_802026e4(c);
                pc = 0x802131d0;
            }
            0x802131d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802131e8; continue; }
                pc = 0x802131dc;
            }
            0x802131dc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2e;
                c.lr = 0x802131e8; f_80202810(c);
                pc = 0x802131e8;
            }
            0x802131e8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x15;
                c.lr = 0x802131f4; f_802026e4(c);
                pc = 0x802131f4;
            }
            0x802131f4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021320c; continue; }
                pc = 0x80213200;
            }
            0x80213200 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x15;
                c.lr = 0x8021320c; f_80202810(c);
                pc = 0x8021320c;
            }
            0x8021320c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x28;
                c.lr = 0x80213218; f_802026e4(c);
                pc = 0x80213218;
            }
            0x80213218 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213230; continue; }
                pc = 0x80213224;
            }
            0x80213224 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x28;
                c.lr = 0x80213230; f_80202810(c);
                pc = 0x80213230;
            }
            0x80213230 => {
                c.r[4] = 0x80380000;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[4].wrapping_add(0xffff889d);
                c.r[5] = 0x1;
                c.lr = 0x80213244; f_80211b94(c);
                pc = 0x80213244;
            }
            0x80213244 => {
                c.lr = 0x80213248; f_801da7ac(c);
                pc = 0x80213248;
            }
            0x80213248 => {
                c.r[3] = 0x0;
                c.lr = 0x80213250; f_80261b68(c);
                pc = 0x80213250;
            }
            0x80213250 => {
                c.r[3] = 0x0;
                c.lr = 0x80213258; f_80261e7c(c);
                pc = 0x80213258;
            }
            0x80213258 => {
                c.lr = 0x8021325c; f_8026246c(c);
                pc = 0x8021325c;
            }
            0x8021325c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80213158: bad pc {:#x}", pc),
        }
    }
}

/// fn_80213270 (0x80213270, 612 bytes)
#[inline(never)]
pub fn f_80213270(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213270) { return; }
        if c.prof { crate::prof::enter(c, 0x80213270); f_80213270_b(c, 0x80213270); crate::prof::exit(c, 0x80213270); return; }
    }
    f_80213270_b(c, 0x80213270)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213270_at(c: &mut Cpu, pc: u32) { f_80213270_b(c, pc) }
#[inline(always)]
fn f_80213270_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213270;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213270, pc); }
        match pc {
            0x80213270 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.lr = 0x80213288; f_80261b68(c);
                pc = 0x80213288;
            }
            0x80213288 => {
                c.r[3] = 0x0;
                c.lr = 0x80213290; f_80261e7c(c);
                pc = 0x80213290;
            }
            0x80213290 => {
                c.lr = 0x80213294; f_8026246c(c);
                pc = 0x80213294;
            }
            0x80213294 => {
                c.r[5] = 0x0;
                c.r[4] = 0x0;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                pc = 0x802132b0; continue;
                pc = 0x802132a4;
            }
            0x802132a4 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.stb(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                pc = 0x802132b0;
            }
            0x802132b0 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x8);
                if c.crbit(0) { pc = 0x802132a4; continue; }
                pc = 0x802132bc;
            }
            0x802132bc => {
                c.r[4] = 0x80210000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x34d4);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802132d4; f_801f37b0(c);
                pc = 0x802132d4;
            }
            0x802132d4 => {
                c.r[3] = 0x0;
                c.lr = 0x802132dc; f_801f27d4(c);
                pc = 0x802132dc;
            }
            0x802132dc => {
                c.r[0] = 0x1;
                c.r[3] = 0x80210000;
                c.stb(c.r[1].wrapping_add(0x9), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x3558);
                c.r[5] = c.r[1].wrapping_add(0x9);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802132fc; f_801f37b0(c);
                pc = 0x802132fc;
            }
            0x802132fc => {
                c.r[31] = 0x0;
                pc = 0x80213324; continue;
                pc = 0x80213304;
            }
            0x80213304 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x80213310; f_801f47b4(c);
                pc = 0x80213310;
            }
            0x80213310 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80213320; continue; }
                pc = 0x80213318;
            }
            0x80213318 => {
                c.r[4] = 0x4d;
                c.lr = 0x80213320; f_801f6eec(c);
                pc = 0x80213320;
            }
            0x80213320 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80213324;
            }
            0x80213324 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x80213304; continue; }
                pc = 0x80213330;
            }
            0x80213330 => {
                c.lr = 0x80213334; f_802317e4(c);
                pc = 0x80213334;
            }
            0x80213334 => {
                c.r[4] = 0x80230000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x568);
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x8021334c; f_801f37b0(c);
                pc = 0x8021334c;
            }
            0x8021334c => {
                c.lr = 0x80213350; f_8022fe80(c);
                pc = 0x80213350;
            }
            0x80213350 => {
                c.r[4] = 0x80230000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x318);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80213368; f_801f37b0(c);
                pc = 0x80213368;
            }
            0x80213368 => {
                c.lr = 0x8021336c; f_801da7ac(c);
                pc = 0x8021336c;
            }
            0x8021336c => {
                c.r[4] = 0x80230000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x1a8);
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x80213384; f_801f37b0(c);
                pc = 0x80213384;
            }
            0x80213384 => {
                c.lr = 0x80213388; f_801da7ac(c);
                pc = 0x80213388;
            }
            0x80213388 => {
                c.r[0] = 0x0;
                c.r[3] = 0x80210000;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x3558);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802133a8; f_801f37b0(c);
                pc = 0x802133a8;
            }
            0x802133a8 => {
                c.r[31] = 0x0;
                pc = 0x802133d0; continue;
                pc = 0x802133b0;
            }
            0x802133b0 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x802133bc; f_801f47b4(c);
                pc = 0x802133bc;
            }
            0x802133bc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x802133cc; continue; }
                pc = 0x802133c4;
            }
            0x802133c4 => {
                c.r[4] = 0x4d;
                c.lr = 0x802133cc; f_801f6eec(c);
                pc = 0x802133cc;
            }
            0x802133cc => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x802133d0;
            }
            0x802133d0 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x802133b0; continue; }
                pc = 0x802133dc;
            }
            0x802133dc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[3] = 0x80380000;
                c.r[4] = c.r[3].wrapping_add(0xffff9f58);
                c.r[5] = 0x0;
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffdff;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                c.r[0] = c.r[0].rotate_left(0) & 0xfff7ffff;
                c.r[4] = c.r[4].wrapping_add(0x10000);
                c.r[7] = c.r[5] | c.r[5];
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                c.r[6] = c.r[0].rotate_left(0) & 0xffbfffff;
                c.r[0] = c.r[6].rotate_left(0) & 0xffefffff;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[6]);
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                c.stb(c.r[4].wrapping_add(0x6002), c.r[5]);
                c.stb(c.r[4].wrapping_add(0x60a1), c.r[5]);
                pc = 0x80213430; continue;
                pc = 0x80213424;
            }
            0x80213424 => {
                c.r[0] = c.r[7].rotate_left(0) & 0xff;
                c.r[7] = c.r[7].wrapping_add(0x1);
                c.stb(c.r[3].wrapping_add(c.r[0]), c.r[5]);
                pc = 0x80213430;
            }
            0x80213430 => {
                c.r[0] = c.r[7].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x5);
                if c.crbit(0) { pc = 0x80213424; continue; }
                pc = 0x8021343c;
            }
            0x8021343c => {
                c.lr = 0x80213440; f_801ef634(c);
                pc = 0x80213440;
            }
            0x80213440 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802134c0; continue; }
                pc = 0x80213448;
            }
            0x80213448 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.r[6] = 0x0;
                c.lr = 0x8021345c; f_801f54a4(c);
                pc = 0x8021345c;
            }
            0x8021345c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xff);
                if !c.crbit(1) { pc = 0x80213474; continue; }
                pc = 0x80213470;
            }
            0x80213470 => {
                c.r[3] = 0xff;
                pc = 0x80213474;
            }
            0x80213474 => {
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.r[6] = 0x0;
                c.lr = 0x8021348c; f_801f4c14(c);
                pc = 0x8021348c;
            }
            0x8021348c => {
                c.lr = 0x80213490; f_800e0c54(c);
                pc = 0x80213490;
            }
            0x80213490 => {
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x5b;
                c.r[6] = 0x0;
                c.lr = 0x802134a8; f_801f4c14(c);
                pc = 0x802134a8;
            }
            0x802134a8 => {
                c.lr = 0x802134ac; f_801da7ac(c);
                pc = 0x802134ac;
            }
            0x802134ac => {
                c.r[3] = 0x0;
                c.lr = 0x802134b4; f_80261b68(c);
                pc = 0x802134b4;
            }
            0x802134b4 => {
                c.r[3] = 0x0;
                c.lr = 0x802134bc; f_80261e7c(c);
                pc = 0x802134bc;
            }
            0x802134bc => {
                c.lr = 0x802134c0; f_8026246c(c);
                pc = 0x802134c0;
            }
            0x802134c0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80213270: bad pc {:#x}", pc),
        }
    }
}

/// _fightSeqTurnCheckSubFightOutPokemon__FPvUsPv (0x802134d4, 132 bytes)
#[inline(never)]
pub fn f_802134d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802134d4) { return; }
        if c.prof { crate::prof::enter(c, 0x802134d4); f_802134d4_b(c, 0x802134d4); crate::prof::exit(c, 0x802134d4); return; }
    }
    f_802134d4_b(c, 0x802134d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802134d4_at(c: &mut Cpu, pc: u32) { f_802134d4_b(c, pc) }
#[inline(always)]
fn f_802134d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802134d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802134d4, pc); }
        match pc {
            0x802134d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x802134ec; f_802062fc(c);
                pc = 0x802134ec;
            }
            0x802134ec => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802134fc; continue; }
                pc = 0x802134f4;
            }
            0x802134f4 => {
                c.r[3] = 0x1;
                pc = 0x80213544; continue;
                pc = 0x802134fc;
            }
            0x802134fc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x11;
                c.lr = 0x80213508; f_80202810(c);
                pc = 0x80213508;
            }
            0x80213508 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x80213514; f_802026e4(c);
                pc = 0x80213514;
            }
            0x80213514 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213540; continue; }
                pc = 0x80213520;
            }
            0x80213520 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x22;
                c.lr = 0x8021352c; f_802026e4(c);
                pc = 0x8021352c;
            }
            0x8021352c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213540; continue; }
                pc = 0x80213538;
            }
            0x80213538 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80213540; f_80200b10(c);
                pc = 0x80213540;
            }
            0x80213540 => {
                c.r[3] = 0x1;
                pc = 0x80213544;
            }
            0x80213544 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802134d4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80213558 (0x80213558, 332 bytes)
#[inline(never)]
pub fn f_80213558(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213558) { return; }
        if c.prof { crate::prof::enter(c, 0x80213558); f_80213558_b(c, 0x80213558); crate::prof::exit(c, 0x80213558); return; }
    }
    f_80213558_b(c, 0x80213558)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213558_at(c: &mut Cpu, pc: u32) { f_80213558_b(c, pc) }
#[inline(always)]
fn f_80213558_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213558;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213558, pc); }
        match pc {
            0x80213558 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[5]);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213590; continue; }
                pc = 0x80213578;
            }
            0x80213578 => {
                c.r[4] = 0x2b;
                c.lr = 0x80213580; f_80202810(c);
                pc = 0x80213580;
            }
            0x80213580 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x2c;
                c.lr = 0x8021358c; f_80202810(c);
                pc = 0x8021358c;
            }
            0x8021358c => {
                pc = 0x80213658; continue;
                pc = 0x80213590;
            }
            0x80213590 => {
                c.lr = 0x80213594; f_802075a4(c);
                pc = 0x80213594;
            }
            0x80213594 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x32;
                c.lr = 0x802135a0; f_80202810(c);
                pc = 0x802135a0;
            }
            0x802135a0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x37;
                c.lr = 0x802135ac; f_80202810(c);
                pc = 0x802135ac;
            }
            0x802135ac => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x33;
                c.lr = 0x802135b8; f_80202810(c);
                pc = 0x802135b8;
            }
            0x802135b8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xed;
                c.r[6] = 0x0;
                c.lr = 0x802135cc; f_8012640c(c);
                pc = 0x802135cc;
            }
            0x802135cc => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x802135f0; continue; }
                pc = 0x802135d4;
            }
            0x802135d4 => {
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.r[3] = c.r[30] | c.r[30];
                c.r[7] = c.r[0].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0xed;
                c.r[6] = 0x0;
                c.lr = 0x802135f0; f_801254b4(c);
                pc = 0x802135f0;
            }
            0x802135f0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x12;
                c.lr = 0x802135fc; f_802026e4(c);
                pc = 0x802135fc;
            }
            0x802135fc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213658; continue; }
                pc = 0x80213608;
            }
            0x80213608 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x12;
                c.lr = 0x80213614; f_80202108(c);
                pc = 0x80213614;
            }
            0x80213614 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x12;
                c.lr = 0x80213624; f_80202234(c);
                pc = 0x80213624;
            }
            0x80213624 => {
                c.r[4] = c.r[31] as u8 as i8 as i32 as u32;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[5], c.r[3]);
                if c.crbit(0) { pc = 0x8021364c; continue; }
                pc = 0x8021363c;
            }
            0x8021363c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x12;
                c.lr = 0x80213648; f_80202810(c);
                pc = 0x80213648;
            }
            0x80213648 => {
                pc = 0x80213658; continue;
                pc = 0x8021364c;
            }
            0x8021364c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x12;
                c.lr = 0x80213658; f_80201fdc(c);
                pc = 0x80213658;
            }
            0x80213658 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x14;
                c.lr = 0x80213664; f_802026e4(c);
                pc = 0x80213664;
            }
            0x80213664 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021368c; continue; }
                pc = 0x80213670;
            }
            0x80213670 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80213678; f_801fef74(c);
                pc = 0x80213678;
            }
            0x80213678 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(1) { pc = 0x8021368c; continue; }
                pc = 0x80213680;
            }
            0x80213680 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x14;
                c.lr = 0x8021368c; f_80202810(c);
                pc = 0x8021368c;
            }
            0x8021368c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80213558: bad pc {:#x}", pc),
        }
    }
}

/// fn_802136A4 (0x802136a4, 36 bytes)
#[inline(never)]
pub fn f_802136a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802136a4) { return; }
        if c.prof { crate::prof::enter(c, 0x802136a4); f_802136a4_b(c, 0x802136a4); crate::prof::exit(c, 0x802136a4); return; }
    }
    f_802136a4_b(c, 0x802136a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802136a4_at(c: &mut Cpu, pc: u32) { f_802136a4_b(c, pc) }
#[inline(always)]
fn f_802136a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802136a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802136a4, pc); }
        match pc {
            0x802136a4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x802136b4; f_80207448(c);
                pc = 0x802136b4;
            }
            0x802136b4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802136a4: bad pc {:#x}", pc),
        }
    }
}

/// WS_STATUS_SET32 (0x802136c8, 136 bytes)
#[inline(never)]
pub fn f_802136c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802136c8) { return; }
        if c.prof { crate::prof::enter(c, 0x802136c8); f_802136c8_b(c, 0x802136c8); crate::prof::exit(c, 0x802136c8); return; }
    }
    f_802136c8_b(c, 0x802136c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802136c8_at(c: &mut Cpu, pc: u32) { f_802136c8_b(c, pc) }
#[inline(always)]
fn f_802136c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802136c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802136c8, pc); }
        match pc {
            0x802136c8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x2));
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1));
                c.cmpu(0, c.r[5], 0x0);
                c.r[6] = c.lhz(c.r[4].wrapping_add(0xa));
                c.r[7] = c.lwz(c.r[4].wrapping_add(0x6));
                c.r[8] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[9] = c.lwz(c.r[4].wrapping_add(0x10));
                if !c.crbit(2) { pc = 0x80213700; continue; }
                pc = 0x802136f8;
            }
            0x802136f8 => {
                c.r[4] = 0x0;
                pc = 0x80213704; continue;
                pc = 0x80213700;
            }
            0x80213700 => {
                c.r[4] = c.lwz(c.r[5]);
                pc = 0x80213704;
            }
            0x80213704 => {
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(2) { pc = 0x80213714; continue; }
                pc = 0x8021370c;
            }
            0x8021370c => {
                c.r[5] = 0x0;
                pc = 0x80213718; continue;
                pc = 0x80213714;
            }
            0x80213714 => {
                c.r[5] = c.lbz(c.r[7]);
                pc = 0x80213718;
            }
            0x80213718 => {
                c.cmpu(0, c.r[8], 0x0);
                if !c.crbit(2) { pc = 0x80213728; continue; }
                pc = 0x80213720;
            }
            0x80213720 => {
                c.r[7] = 0x0;
                pc = 0x8021372c; continue;
                pc = 0x80213728;
            }
            0x80213728 => {
                c.r[7] = c.lbz(c.r[8]);
                pc = 0x8021372c;
            }
            0x8021372c => {
                c.r[8] = c.lwz(c.r[9]);
                c.lr = 0x80213734; f_80135d10(c);
                pc = 0x80213734;
            }
            0x80213734 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x14);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802136c8: bad pc {:#x}", pc),
        }
    }
}

/// WS_STATUS_SET16 (0x80213750, 136 bytes)
#[inline(never)]
pub fn f_80213750(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213750) { return; }
        if c.prof { crate::prof::enter(c, 0x80213750); f_80213750_b(c, 0x80213750); crate::prof::exit(c, 0x80213750); return; }
    }
    f_80213750_b(c, 0x80213750)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213750_at(c: &mut Cpu, pc: u32) { f_80213750_b(c, pc) }
#[inline(always)]
fn f_80213750_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213750;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213750, pc); }
        match pc {
            0x80213750 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x2));
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1));
                c.cmpu(0, c.r[5], 0x0);
                c.r[6] = c.lhz(c.r[4].wrapping_add(0xa));
                c.r[7] = c.lwz(c.r[4].wrapping_add(0x6));
                c.r[8] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[9] = c.lwz(c.r[4].wrapping_add(0x10));
                if !c.crbit(2) { pc = 0x80213788; continue; }
                pc = 0x80213780;
            }
            0x80213780 => {
                c.r[4] = 0x0;
                pc = 0x8021378c; continue;
                pc = 0x80213788;
            }
            0x80213788 => {
                c.r[4] = c.lwz(c.r[5]);
                pc = 0x8021378c;
            }
            0x8021378c => {
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(2) { pc = 0x8021379c; continue; }
                pc = 0x80213794;
            }
            0x80213794 => {
                c.r[5] = 0x0;
                pc = 0x802137a0; continue;
                pc = 0x8021379c;
            }
            0x8021379c => {
                c.r[5] = c.lbz(c.r[7]);
                pc = 0x802137a0;
            }
            0x802137a0 => {
                c.cmpu(0, c.r[8], 0x0);
                if !c.crbit(2) { pc = 0x802137b0; continue; }
                pc = 0x802137a8;
            }
            0x802137a8 => {
                c.r[7] = 0x0;
                pc = 0x802137b4; continue;
                pc = 0x802137b0;
            }
            0x802137b0 => {
                c.r[7] = c.lbz(c.r[8]);
                pc = 0x802137b4;
            }
            0x802137b4 => {
                c.r[8] = c.lhz(c.r[9]);
                c.lr = 0x802137bc; f_80135d10(c);
                pc = 0x802137bc;
            }
            0x802137bc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x14);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80213750: bad pc {:#x}", pc),
        }
    }
}

/// WS_STATUS_SET8 (0x802137d8, 136 bytes)
#[inline(never)]
pub fn f_802137d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802137d8) { return; }
        if c.prof { crate::prof::enter(c, 0x802137d8); f_802137d8_b(c, 0x802137d8); crate::prof::exit(c, 0x802137d8); return; }
    }
    f_802137d8_b(c, 0x802137d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802137d8_at(c: &mut Cpu, pc: u32) { f_802137d8_b(c, pc) }
#[inline(always)]
fn f_802137d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802137d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802137d8, pc); }
        match pc {
            0x802137d8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x2));
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1));
                c.cmpu(0, c.r[5], 0x0);
                c.r[6] = c.lhz(c.r[4].wrapping_add(0xa));
                c.r[7] = c.lwz(c.r[4].wrapping_add(0x6));
                c.r[8] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[9] = c.lwz(c.r[4].wrapping_add(0x10));
                if !c.crbit(2) { pc = 0x80213810; continue; }
                pc = 0x80213808;
            }
            0x80213808 => {
                c.r[4] = 0x0;
                pc = 0x80213814; continue;
                pc = 0x80213810;
            }
            0x80213810 => {
                c.r[4] = c.lwz(c.r[5]);
                pc = 0x80213814;
            }
            0x80213814 => {
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(2) { pc = 0x80213824; continue; }
                pc = 0x8021381c;
            }
            0x8021381c => {
                c.r[5] = 0x0;
                pc = 0x80213828; continue;
                pc = 0x80213824;
            }
            0x80213824 => {
                c.r[5] = c.lbz(c.r[7]);
                pc = 0x80213828;
            }
            0x80213828 => {
                c.cmpu(0, c.r[8], 0x0);
                if !c.crbit(2) { pc = 0x80213838; continue; }
                pc = 0x80213830;
            }
            0x80213830 => {
                c.r[7] = 0x0;
                pc = 0x8021383c; continue;
                pc = 0x80213838;
            }
            0x80213838 => {
                c.r[7] = c.lbz(c.r[8]);
                pc = 0x8021383c;
            }
            0x8021383c => {
                c.r[8] = c.lbz(c.r[9]);
                c.lr = 0x80213844; f_80135d10(c);
                pc = 0x80213844;
            }
            0x80213844 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x14);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802137d8: bad pc {:#x}", pc),
        }
    }
}

/// WS_STATUS_GET32 (0x80213860, 144 bytes)
#[inline(never)]
pub fn f_80213860(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213860) { return; }
        if c.prof { crate::prof::enter(c, 0x80213860); f_80213860_b(c, 0x80213860); crate::prof::exit(c, 0x80213860); return; }
    }
    f_80213860_b(c, 0x80213860)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213860_at(c: &mut Cpu, pc: u32) { f_80213860_b(c, pc) }
#[inline(always)]
fn f_80213860_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213860;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213860, pc); }
        match pc {
            0x80213860 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x2));
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1));
                c.cmpu(0, c.r[5], 0x0);
                c.r[6] = c.lhz(c.r[4].wrapping_add(0xa));
                c.r[7] = c.lwz(c.r[4].wrapping_add(0x6));
                c.r[8] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x10));
                if !c.crbit(2) { pc = 0x8021389c; continue; }
                pc = 0x80213894;
            }
            0x80213894 => {
                c.r[4] = 0x0;
                pc = 0x802138a0; continue;
                pc = 0x8021389c;
            }
            0x8021389c => {
                c.r[4] = c.lwz(c.r[5]);
                pc = 0x802138a0;
            }
            0x802138a0 => {
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(2) { pc = 0x802138b0; continue; }
                pc = 0x802138a8;
            }
            0x802138a8 => {
                c.r[5] = 0x0;
                pc = 0x802138b4; continue;
                pc = 0x802138b0;
            }
            0x802138b0 => {
                c.r[5] = c.lbz(c.r[7]);
                pc = 0x802138b4;
            }
            0x802138b4 => {
                c.cmpu(0, c.r[8], 0x0);
                if !c.crbit(2) { pc = 0x802138c4; continue; }
                pc = 0x802138bc;
            }
            0x802138bc => {
                c.r[7] = 0x0;
                pc = 0x802138c8; continue;
                pc = 0x802138c4;
            }
            0x802138c4 => {
                c.r[7] = c.lbz(c.r[8]);
                pc = 0x802138c8;
            }
            0x802138c8 => {
                c.lr = 0x802138cc; f_80135e44(c);
                pc = 0x802138cc;
            }
            0x802138cc => {
                c.stw(c.r[31], c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x14);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80213860: bad pc {:#x}", pc),
        }
    }
}

/// WS_STATUS_GET16 (0x802138f0, 144 bytes)
#[inline(never)]
pub fn f_802138f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802138f0) { return; }
        if c.prof { crate::prof::enter(c, 0x802138f0); f_802138f0_b(c, 0x802138f0); crate::prof::exit(c, 0x802138f0); return; }
    }
    f_802138f0_b(c, 0x802138f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802138f0_at(c: &mut Cpu, pc: u32) { f_802138f0_b(c, pc) }
#[inline(always)]
fn f_802138f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802138f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802138f0, pc); }
        match pc {
            0x802138f0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x2));
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1));
                c.cmpu(0, c.r[5], 0x0);
                c.r[6] = c.lhz(c.r[4].wrapping_add(0xa));
                c.r[7] = c.lwz(c.r[4].wrapping_add(0x6));
                c.r[8] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x10));
                if !c.crbit(2) { pc = 0x8021392c; continue; }
                pc = 0x80213924;
            }
            0x80213924 => {
                c.r[4] = 0x0;
                pc = 0x80213930; continue;
                pc = 0x8021392c;
            }
            0x8021392c => {
                c.r[4] = c.lwz(c.r[5]);
                pc = 0x80213930;
            }
            0x80213930 => {
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(2) { pc = 0x80213940; continue; }
                pc = 0x80213938;
            }
            0x80213938 => {
                c.r[5] = 0x0;
                pc = 0x80213944; continue;
                pc = 0x80213940;
            }
            0x80213940 => {
                c.r[5] = c.lbz(c.r[7]);
                pc = 0x80213944;
            }
            0x80213944 => {
                c.cmpu(0, c.r[8], 0x0);
                if !c.crbit(2) { pc = 0x80213954; continue; }
                pc = 0x8021394c;
            }
            0x8021394c => {
                c.r[7] = 0x0;
                pc = 0x80213958; continue;
                pc = 0x80213954;
            }
            0x80213954 => {
                c.r[7] = c.lbz(c.r[8]);
                pc = 0x80213958;
            }
            0x80213958 => {
                c.lr = 0x8021395c; f_80135e44(c);
                pc = 0x8021395c;
            }
            0x8021395c => {
                c.sth(c.r[31], c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x14);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802138f0: bad pc {:#x}", pc),
        }
    }
}

/// WS_STATUS_GET8 (0x80213980, 144 bytes)
#[inline(never)]
pub fn f_80213980(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213980) { return; }
        if c.prof { crate::prof::enter(c, 0x80213980); f_80213980_b(c, 0x80213980); crate::prof::exit(c, 0x80213980); return; }
    }
    f_80213980_b(c, 0x80213980)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213980_at(c: &mut Cpu, pc: u32) { f_80213980_b(c, pc) }
#[inline(always)]
fn f_80213980_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213980;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213980, pc); }
        match pc {
            0x80213980 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x2));
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1));
                c.cmpu(0, c.r[5], 0x0);
                c.r[6] = c.lhz(c.r[4].wrapping_add(0xa));
                c.r[7] = c.lwz(c.r[4].wrapping_add(0x6));
                c.r[8] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x10));
                if !c.crbit(2) { pc = 0x802139bc; continue; }
                pc = 0x802139b4;
            }
            0x802139b4 => {
                c.r[4] = 0x0;
                pc = 0x802139c0; continue;
                pc = 0x802139bc;
            }
            0x802139bc => {
                c.r[4] = c.lwz(c.r[5]);
                pc = 0x802139c0;
            }
            0x802139c0 => {
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(2) { pc = 0x802139d0; continue; }
                pc = 0x802139c8;
            }
            0x802139c8 => {
                c.r[5] = 0x0;
                pc = 0x802139d4; continue;
                pc = 0x802139d0;
            }
            0x802139d0 => {
                c.r[5] = c.lbz(c.r[7]);
                pc = 0x802139d4;
            }
            0x802139d4 => {
                c.cmpu(0, c.r[8], 0x0);
                if !c.crbit(2) { pc = 0x802139e4; continue; }
                pc = 0x802139dc;
            }
            0x802139dc => {
                c.r[7] = 0x0;
                pc = 0x802139e8; continue;
                pc = 0x802139e4;
            }
            0x802139e4 => {
                c.r[7] = c.lbz(c.r[8]);
                pc = 0x802139e8;
            }
            0x802139e8 => {
                c.lr = 0x802139ec; f_80135e44(c);
                pc = 0x802139ec;
            }
            0x802139ec => {
                c.stb(c.r[31], c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x14);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80213980: bad pc {:#x}", pc),
        }
    }
}

/// WS_GETEND (0x80213a10, 12 bytes)
#[inline(never)]
pub fn f_80213a10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213a10) { return; }
        if c.prof { crate::prof::enter(c, 0x80213a10); f_80213a10_b(c, 0x80213a10); crate::prof::exit(c, 0x80213a10); return; }
    }
    f_80213a10_b(c, 0x80213a10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213a10_at(c: &mut Cpu, pc: u32) { f_80213a10_b(c, pc) }
#[inline(always)]
fn f_80213a10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213a10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213a10, pc); }
        match pc {
            0x80213a10 => {
                c.r[0] = 0x1;
                c.stb(c.r[13].wrapping_add(0xffffadf4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80213a10: bad pc {:#x}", pc),
        }
    }
}

/// WS_ITEMEND (0x80213a1c, 12 bytes)
#[inline(never)]
pub fn f_80213a1c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213a1c) { return; }
        if c.prof { crate::prof::enter(c, 0x80213a1c); f_80213a1c_b(c, 0x80213a1c); crate::prof::exit(c, 0x80213a1c); return; }
    }
    f_80213a1c_b(c, 0x80213a1c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213a1c_at(c: &mut Cpu, pc: u32) { f_80213a1c_b(c, pc) }
#[inline(always)]
fn f_80213a1c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213a1c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213a1c, pc); }
        match pc {
            0x80213a1c => {
                c.r[0] = 0x1;
                c.stb(c.r[13].wrapping_add(0xffffadf4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80213a1c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80213A28 (0x80213a28, 16 bytes)
#[inline(never)]
pub fn f_80213a28(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213a28) { return; }
        if c.prof { crate::prof::enter(c, 0x80213a28); f_80213a28_b(c, 0x80213a28); crate::prof::exit(c, 0x80213a28); return; }
    }
    f_80213a28_b(c, 0x80213a28)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213a28_at(c: &mut Cpu, pc: u32) { f_80213a28_b(c, pc) }
#[inline(always)]
fn f_80213a28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213a28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213a28, pc); }
        match pc {
            0x80213a28 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80213a28: bad pc {:#x}", pc),
        }
    }
}

/// fn_80213A38 (0x80213a38, 16 bytes)
#[inline(never)]
pub fn f_80213a38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213a38) { return; }
        if c.prof { crate::prof::enter(c, 0x80213a38); f_80213a38_b(c, 0x80213a38); crate::prof::exit(c, 0x80213a38); return; }
    }
    f_80213a38_b(c, 0x80213a38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213a38_at(c: &mut Cpu, pc: u32) { f_80213a38_b(c, pc) }
#[inline(always)]
fn f_80213a38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213a38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213a38, pc); }
        match pc {
            0x80213a38 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80213a38: bad pc {:#x}", pc),
        }
    }
}

/// fn_80213A48 (0x80213a48, 16 bytes)
#[inline(never)]
pub fn f_80213a48(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213a48) { return; }
        if c.prof { crate::prof::enter(c, 0x80213a48); f_80213a48_b(c, 0x80213a48); crate::prof::exit(c, 0x80213a48); return; }
    }
    f_80213a48_b(c, 0x80213a48)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213a48_at(c: &mut Cpu, pc: u32) { f_80213a48_b(c, pc) }
#[inline(always)]
fn f_80213a48_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213a48;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213a48, pc); }
        match pc {
            0x80213a48 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80213a48: bad pc {:#x}", pc),
        }
    }
}

/// fn_80213A58 (0x80213a58, 16 bytes)
#[inline(never)]
pub fn f_80213a58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213a58) { return; }
        if c.prof { crate::prof::enter(c, 0x80213a58); f_80213a58_b(c, 0x80213a58); crate::prof::exit(c, 0x80213a58); return; }
    }
    f_80213a58_b(c, 0x80213a58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213a58_at(c: &mut Cpu, pc: u32) { f_80213a58_b(c, pc) }
#[inline(always)]
fn f_80213a58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213a58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213a58, pc); }
        match pc {
            0x80213a58 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80213a58: bad pc {:#x}", pc),
        }
    }
}

/// fn_80213A68 (0x80213a68, 16 bytes)
#[inline(never)]
pub fn f_80213a68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213a68) { return; }
        if c.prof { crate::prof::enter(c, 0x80213a68); f_80213a68_b(c, 0x80213a68); crate::prof::exit(c, 0x80213a68); return; }
    }
    f_80213a68_b(c, 0x80213a68)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213a68_at(c: &mut Cpu, pc: u32) { f_80213a68_b(c, pc) }
#[inline(always)]
fn f_80213a68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213a68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213a68, pc); }
        match pc {
            0x80213a68 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80213a68: bad pc {:#x}", pc),
        }
    }
}

/// fn_80213A78 (0x80213a78, 1052 bytes)
#[inline(never)]
pub fn f_80213a78(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213a78) { return; }
        if c.prof { crate::prof::enter(c, 0x80213a78); f_80213a78_b(c, 0x80213a78); crate::prof::exit(c, 0x80213a78); return; }
    }
    f_80213a78_b(c, 0x80213a78)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213a78_at(c: &mut Cpu, pc: u32) { f_80213a78_b(c, pc) }
#[inline(always)]
fn f_80213a78_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213a78;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213a78, pc); }
        match pc {
            0x80213a78 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = 0x0;
                c.lr = 0x80213a98; f_801f025c(c);
                pc = 0x80213a98;
            }
            0x80213a98 => {
                c.r[4] = 0x0;
                c.r[26] = c.r[3] | c.r[3];
                c.r[5] = 0xe5;
                c.r[6] = 0x0;
                c.lr = 0x80213aac; f_8012640c(c);
                pc = 0x80213aac;
            }
            0x80213aac => {
                c.r[4] = 0x0;
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.lr = 0x80213abc; f_80142cf4(c);
                pc = 0x80213abc;
            }
            0x80213abc => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0xa;
                c.r[6] = 0x0;
                c.lr = 0x80213ad0; f_80142cf4(c);
                pc = 0x80213ad0;
            }
            0x80213ad0 => {
                c.r[22] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.lr = 0x80213ae0; f_801f4354(c);
                pc = 0x80213ae0;
            }
            0x80213ae0 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.lr = 0x80213af0; f_801f4220(c);
                pc = 0x80213af0;
            }
            0x80213af0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[24] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80213b04; f_801f025c(c);
                pc = 0x80213b04;
            }
            0x80213b04 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80213b18; f_801f4354(c);
                pc = 0x80213b18;
            }
            0x80213b18 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x4a;
                c.r[6] = 0x0;
                c.lr = 0x80213b34; f_801f54a4(c);
                pc = 0x80213b34;
            }
            0x80213b34 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[25] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xd5;
                c.r[6] = 0x0;
                c.lr = 0x80213b50; f_8012640c(c);
                pc = 0x80213b50;
            }
            0x80213b50 => {
                c.lr = 0x80213b54; f_80205be8(c);
                pc = 0x80213b54;
            }
            0x80213b54 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[27] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x17;
                c.r[6] = 0x0;
                c.lr = 0x80213b70; f_801f54a4(c);
                pc = 0x80213b70;
            }
            0x80213b70 => {
                c.r[23] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0xc9;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80213b8c; f_801254b4(c);
                pc = 0x80213b8c;
            }
            0x80213b8c => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[25] | c.r[25];
                c.r[6] = c.r[22] | c.r[22];
                c.r[7] = 0x1;
                c.lr = 0x80213ba4; f_80129f20(c);
                pc = 0x80213ba4;
            }
            0x80213ba4 => {
                c.r[26] = c.r[3] | c.r[3];
                c.r[0] = c.r[26] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(2) { pc = 0x80213bb8; continue; }
                pc = 0x80213bb4;
            }
            0x80213bb4 => {
                c.r[31] = 0x1;
                pc = 0x80213bb8;
            }
            0x80213bb8 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80213cbc; continue; }
                pc = 0x80213bc0;
            }
            0x80213bc0 => {
                c.r[0] = c.r[26] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80213cbc; continue; }
                pc = 0x80213bc8;
            }
            0x80213bc8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x80213bd4; f_801f7f80(c);
                pc = 0x80213bd4;
            }
            0x80213bd4 => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if c.crbit(2) { pc = 0x80213cbc; continue; }
                pc = 0x80213bdc;
            }
            0x80213bdc => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[5] = c.r[26].rotate_left(0) & 0xffff;
                c.r[4] = 0x3;
                c.lr = 0x80213bec; f_8012a5b0(c);
                pc = 0x80213bec;
            }
            0x80213bec => {
                c.r[24] = c.r[3] | c.r[3];
                c.rc(c.r[24]);
                if c.crbit(2) { pc = 0x80213cbc; continue; }
                pc = 0x80213bf4;
            }
            0x80213bf4 => {
                c.lr = 0x80213bf8; f_80123fbc(c);
                pc = 0x80213bf8;
            }
            0x80213bf8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213cbc; continue; }
                pc = 0x80213c04;
            }
            0x80213c04 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x4;
                c.lr = 0x80213c10; f_80121adc(c);
                pc = 0x80213c10;
            }
            0x80213c10 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213c2c; continue; }
                pc = 0x80213c1c;
            }
            0x80213c1c => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x4;
                c.r[5] = 0x1;
                c.lr = 0x80213c2c; f_8012190c(c);
                pc = 0x80213c2c;
            }
            0x80213c2c => {
                c.r[3] = 0x0;
                c.lr = 0x80213c34; f_801f4804(c);
                pc = 0x80213c34;
            }
            0x80213c34 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[24] | c.r[24];
                c.lr = 0x80213c44; f_80206aec(c);
                pc = 0x80213c44;
            }
            0x80213c44 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0xcf;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80213c5c; f_801254b4(c);
                pc = 0x80213c5c;
            }
            0x80213c5c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x80213c70; f_801f54a4(c);
                pc = 0x80213c70;
            }
            0x80213c70 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213cbc; continue; }
                pc = 0x80213c7c;
            }
            0x80213c7c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x2e;
                c.r[6] = 0x0;
                c.lr = 0x80213c90; f_801f54a4(c);
                pc = 0x80213c90;
            }
            0x80213c90 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213cbc; continue; }
                pc = 0x80213c9c;
            }
            0x80213c9c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80213ca4; f_801fb8f8(c);
                pc = 0x80213ca4;
            }
            0x80213ca4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213cbc; continue; }
                pc = 0x80213cb0;
            }
            0x80213cb0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x3;
                c.lr = 0x80213cbc; f_802032e4(c);
                pc = 0x80213cbc;
            }
            0x80213cbc => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.lr = 0x80213cd0; f_801f54a4(c);
                pc = 0x80213cd0;
            }
            0x80213cd0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213d08; continue; }
                pc = 0x80213cdc;
            }
            0x80213cdc => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80213ce4; f_801fb8f8(c);
                pc = 0x80213ce4;
            }
            0x80213ce4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213d08; continue; }
                pc = 0x80213cf0;
            }
            0x80213cf0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x1;
                c.lr = 0x80213cfc; f_80123368(c);
                pc = 0x80213cfc;
            }
            0x80213cfc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x1;
                c.lr = 0x80213d08; f_801232e0(c);
                pc = 0x80213d08;
            }
            0x80213d08 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80213d20; f_801254b4(c);
                pc = 0x80213d20;
            }
            0x80213d20 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80213d28; f_80205b8c(c);
                pc = 0x80213d28;
            }
            0x80213d28 => {
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80213d3c; f_801254b4(c);
                pc = 0x80213d3c;
            }
            0x80213d3c => {
                c.r[3] = 0x80230000;
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[3].wrapping_add(0x2fe4);
                c.r[6] = 0x0;
                c.r[3] = 0x0;
                c.lr = 0x80213d54; f_801f37b0(c);
                pc = 0x80213d54;
            }
            0x80213d54 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80213d5c; f_80205b8c(c);
                pc = 0x80213d5c;
            }
            0x80213d5c => {
                c.lr = 0x80213d60; f_801252e0(c);
                pc = 0x80213d60;
            }
            0x80213d60 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x80213d6c; f_80202998(c);
                pc = 0x80213d6c;
            }
            0x80213d6c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x17;
                c.lr = 0x80213d78; f_80202810(c);
                pc = 0x80213d78;
            }
            0x80213d78 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80213d80; f_801fecd4(c);
                pc = 0x80213d80;
            }
            0x80213d80 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213da0; continue; }
                pc = 0x80213d8c;
            }
            0x80213d8c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x7c;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80213da0; f_801fe7ec(c);
                pc = 0x80213da0;
            }
            0x80213da0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x3e;
                c.lr = 0x80213dac; f_802026e4(c);
                pc = 0x80213dac;
            }
            0x80213dac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213dec; continue; }
                pc = 0x80213db8;
            }
            0x80213db8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x3e;
                c.lr = 0x80213dc4; f_80202810(c);
                pc = 0x80213dc4;
            }
            0x80213dc4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80213dcc; f_801fecd4(c);
                pc = 0x80213dcc;
            }
            0x80213dcc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213dec; continue; }
                pc = 0x80213dd8;
            }
            0x80213dd8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0xc8;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80213dec; f_801fe7ec(c);
                pc = 0x80213dec;
            }
            0x80213dec => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xd6;
                c.r[6] = 0x0;
                c.lr = 0x80213e00; f_8012640c(c);
                pc = 0x80213e00;
            }
            0x80213e00 => {
                c.r[4] = 0x0;
                c.r[5] = 0xd2;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80213e14; f_801254b4(c);
                pc = 0x80213e14;
            }
            0x80213e14 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80213e1c; f_801fecd4(c);
                pc = 0x80213e1c;
            }
            0x80213e1c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213e38; continue; }
                pc = 0x80213e28;
            }
            0x80213e28 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0xd2;
                c.r[5] = 0x0;
                c.lr = 0x80213e38; f_801fe710(c);
                pc = 0x80213e38;
            }
            0x80213e38 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x4a;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80213e50; f_801faa58(c);
                pc = 0x80213e50;
            }
            0x80213e50 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80213e74; continue; }
                pc = 0x80213e5c;
            }
            0x80213e5c => {
                c.r[3] = 0x1;
                c.lr = 0x80213e64; f_801ef8f4(c);
                pc = 0x80213e64;
            }
            0x80213e64 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80213e80; continue;
                pc = 0x80213e74;
            }
            0x80213e74 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80213e80;
            }
            0x80213e80 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 22..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80213a78: bad pc {:#x}", pc),
        }
    }
}

/// fn_80213E94 (0x80213e94, 1468 bytes)
#[inline(never)]
pub fn f_80213e94(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80213e94) { return; }
        if c.prof { crate::prof::enter(c, 0x80213e94); f_80213e94_b(c, 0x80213e94); crate::prof::exit(c, 0x80213e94); return; }
    }
    f_80213e94_b(c, 0x80213e94)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80213e94_at(c: &mut Cpu, pc: u32) { f_80213e94_b(c, pc) }
#[inline(always)]
fn f_80213e94_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80213e94;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80213e94, pc); }
        match pc {
            0x80213e94 => {
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x80213e98;
            }
            0x80213e98 => {
                c.r[0] = c.mfspr(8);
                pc = 0x80213e9c;
            }
            0x80213e9c => {
                c.r[3] = 0x0;
                pc = 0x80213ea0;
            }
            0x80213ea0 => {
                c.r[4] = 0x0;
                pc = 0x80213ea4;
            }
            0x80213ea4 => {
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                pc = 0x80213ea8;
            }
            0x80213ea8 => {
                c.r[5] = 0xd;
                pc = 0x80213eac;
            }
            0x80213eac => {
                c.r[6] = 0x0;
                pc = 0x80213eb0;
            }
            0x80213eb0 => {
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 19..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x80213eb4;
            }
            0x80213eb4 => {
                c.r[27] = 0x0;
                pc = 0x80213eb8;
            }
            0x80213eb8 => {
                c.r[20] = 0x0;
                pc = 0x80213ebc;
            }
            0x80213ebc => {
                c.lr = 0x80213ec0; f_801f54a4(c);
                pc = 0x80213ec0;
            }
            0x80213ec0 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80213ec4;
            }
            0x80213ec4 => {
                c.r[3] = 0x0;
                pc = 0x80213ec8;
            }
            0x80213ec8 => {
                c.r[4] = 0x0;
                pc = 0x80213ecc;
            }
            0x80213ecc => {
                c.r[5] = 0xf;
                pc = 0x80213ed0;
            }
            0x80213ed0 => {
                c.r[6] = 0x0;
                pc = 0x80213ed4;
            }
            0x80213ed4 => {
                c.lr = 0x80213ed8; f_801f54a4(c);
                pc = 0x80213ed8;
            }
            0x80213ed8 => {
                c.r[22] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80213edc;
            }
            0x80213edc => {
                c.r[3] = 0x11;
                pc = 0x80213ee0;
            }
            0x80213ee0 => {
                c.r[4] = 0x0;
                pc = 0x80213ee4;
            }
            0x80213ee4 => {
                c.lr = 0x80213ee8; f_801f025c(c);
                pc = 0x80213ee8;
            }
            0x80213ee8 => {
                c.r[4] = 0x0;
                pc = 0x80213eec;
            }
            0x80213eec => {
                c.r[19] = c.r[3] | c.r[3];
                pc = 0x80213ef0;
            }
            0x80213ef0 => {
                c.r[5] = 0xe5;
                pc = 0x80213ef4;
            }
            0x80213ef4 => {
                c.r[6] = 0x0;
                pc = 0x80213ef8;
            }
            0x80213ef8 => {
                c.lr = 0x80213efc; f_8012640c(c);
                pc = 0x80213efc;
            }
            0x80213efc => {
                c.r[4] = 0x0;
                pc = 0x80213f00;
            }
            0x80213f00 => {
                c.r[21] = c.r[3] | c.r[3];
                pc = 0x80213f04;
            }
            0x80213f04 => {
                c.r[5] = 0x20;
                pc = 0x80213f08;
            }
            0x80213f08 => {
                c.r[6] = 0x0;
                pc = 0x80213f0c;
            }
            0x80213f0c => {
                c.lr = 0x80213f10; f_80142cf4(c);
                pc = 0x80213f10;
            }
            0x80213f10 => {
                c.r[3] = c.r[21] | c.r[21];
                pc = 0x80213f14;
            }
            0x80213f14 => {
                c.r[4] = 0x0;
                pc = 0x80213f18;
            }
            0x80213f18 => {
                c.r[5] = 0x1e;
                pc = 0x80213f1c;
            }
            0x80213f1c => {
                c.r[6] = 0x0;
                pc = 0x80213f20;
            }
            0x80213f20 => {
                c.lr = 0x80213f24; f_80142cf4(c);
                pc = 0x80213f24;
            }
            0x80213f24 => {
                c.r[23] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80213f28;
            }
            0x80213f28 => {
                c.r[3] = 0x12;
                pc = 0x80213f2c;
            }
            0x80213f2c => {
                c.r[4] = 0x0;
                pc = 0x80213f30;
            }
            0x80213f30 => {
                c.lr = 0x80213f34; f_801f025c(c);
                pc = 0x80213f34;
            }
            0x80213f34 => {
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x80213f38;
            }
            0x80213f38 => {
                c.lr = 0x80213f3c; f_80205b8c(c);
                pc = 0x80213f3c;
            }
            0x80213f3c => {
                c.r[4] = 0x0;
                pc = 0x80213f40;
            }
            0x80213f40 => {
                c.r[25] = c.r[3] | c.r[3];
                pc = 0x80213f44;
            }
            0x80213f44 => {
                c.r[5] = 0xc3;
                pc = 0x80213f48;
            }
            0x80213f48 => {
                c.r[6] = 0x0;
                pc = 0x80213f4c;
            }
            0x80213f4c => {
                c.lr = 0x80213f50; f_8012640c(c);
                pc = 0x80213f50;
            }
            0x80213f50 => {
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80213f54;
            }
            0x80213f54 => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x80213f58;
            }
            0x80213f58 => {
                c.r[4] = 0x0;
                pc = 0x80213f5c;
            }
            0x80213f5c => {
                c.r[5] = 0x6e;
                pc = 0x80213f60;
            }
            0x80213f60 => {
                c.r[6] = 0x0;
                pc = 0x80213f64;
            }
            0x80213f64 => {
                c.lr = 0x80213f68; f_8012640c(c);
                pc = 0x80213f68;
            }
            0x80213f68 => {
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80213f6c;
            }
            0x80213f6c => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x80213f70;
            }
            0x80213f70 => {
                c.lr = 0x80213f74; f_80203e0c(c);
                pc = 0x80213f74;
            }
            0x80213f74 => {
                c.r[21] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80213f78;
            }
            0x80213f78 => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x80213f7c;
            }
            0x80213f7c => {
                c.r[4] = 0x0;
                pc = 0x80213f80;
            }
            0x80213f80 => {
                c.r[5] = 0x83;
                pc = 0x80213f84;
            }
            0x80213f84 => {
                c.r[6] = 0x0;
                pc = 0x80213f88;
            }
            0x80213f88 => {
                c.lr = 0x80213f8c; f_8012640c(c);
                pc = 0x80213f8c;
            }
            0x80213f8c => {
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x80213f90;
            }
            0x80213f90 => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x80213f94;
            }
            0x80213f94 => {
                c.r[4] = 0x0;
                pc = 0x80213f98;
            }
            0x80213f98 => {
                c.r[5] = 0x87;
                pc = 0x80213f9c;
            }
            0x80213f9c => {
                c.r[6] = 0x0;
                pc = 0x80213fa0;
            }
            0x80213fa0 => {
                c.lr = 0x80213fa4; f_8012640c(c);
                pc = 0x80213fa4;
            }
            0x80213fa4 => {
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x80213fa8;
            }
            0x80213fa8 => {
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x80213fac;
            }
            0x80213fac => {
                c.r[3] = 0x0;
                pc = 0x80213fb0;
            }
            0x80213fb0 => {
                c.lr = 0x80213fb4; f_801f4354(c);
                pc = 0x80213fb4;
            }
            0x80213fb4 => {
                c.r[0] = c.r[3] | c.r[3];
                pc = 0x80213fb8;
            }
            0x80213fb8 => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x80213fbc;
            }
            0x80213fbc => {
                c.r[19] = c.r[0] | c.r[0];
                pc = 0x80213fc0;
            }
            0x80213fc0 => {
                c.lr = 0x80213fc4; f_8011fc74(c);
                pc = 0x80213fc4;
            }
            0x80213fc4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80213fc8;
            }
            0x80213fc8 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x80213fcc;
            }
            0x80213fcc => {
                if !c.crbit(2) { pc = 0x80213fe0; continue; }
                pc = 0x80213fd0;
            }
            0x80213fd0 => {
                c.r[3] = c.r[24] | c.r[24];
                pc = 0x80213fd4;
            }
            0x80213fd4 => {
                c.lr = 0x80213fd8; f_801eee44(c);
                pc = 0x80213fd8;
            }
            0x80213fd8 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x80213fdc;
            }
            0x80213fdc => {
                pc = 0x80213ff8; continue;
                pc = 0x80213fe0;
            }
            0x80213fe0 => {
                c.r[4] = c.r[30] | c.r[30];
                pc = 0x80213fe4;
            }
            0x80213fe4 => {
                c.r[3] = 0x0;
                pc = 0x80213fe8;
            }
            0x80213fe8 => {
                c.r[5] = 0x12;
                pc = 0x80213fec;
            }
            0x80213fec => {
                c.r[6] = 0x0;
                pc = 0x80213ff0;
            }
            0x80213ff0 => {
                c.lr = 0x80213ff4; f_8012640c(c);
                pc = 0x80213ff4;
            }
            0x80213ff4 => {
                c.r[30] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80213ff8;
            }
            0x80213ff8 => {
                c.cmpu(0, c.r[23], 0xc);
                pc = 0x80213ffc;
            }
            0x80213ffc => {
                if c.crbit(1) { pc = 0x80214164; continue; }
                pc = 0x80214000;
            }
            0x80214000 => {
                c.r[3] = 0x803a0000;
                pc = 0x80214004;
            }
            0x80214004 => {
                c.r[0] = c.r[23].rotate_left(2) & 0xfffffffc;
                pc = 0x80214008;
            }
            0x80214008 => {
                c.r[3] = c.r[3].wrapping_add(0xffff9ff4);
                pc = 0x8021400c;
            }
            0x8021400c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x80214010;
            }
            0x80214010 => {
                c.mtspr(9, c.r[0]);
                pc = 0x80214014;
            }
            0x80214014 => {
                if true { let t = c.ctr; if (0x80213e94..0x80214450).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80214018;
            }
            0x80214018 => {
                c.r[27] = 0x14;
                pc = 0x8021401c;
            }
            0x8021401c => {
                pc = 0x80214164; continue;
                pc = 0x80214020;
            }
            0x80214020 => {
                c.r[27] = 0xf;
                pc = 0x80214024;
            }
            0x80214024 => {
                pc = 0x80214164; continue;
                pc = 0x80214028;
            }
            0x80214028 => {
                c.r[27] = 0xa;
                pc = 0x8021402c;
            }
            0x8021402c => {
                pc = 0x80214164; continue;
                pc = 0x80214030;
            }
            0x80214030 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x80214034;
            }
            0x80214034 => {
                c.r[4] = 0xb;
                pc = 0x80214038;
            }
            0x80214038 => {
                c.lr = 0x8021403c; f_80207ae0(c);
                pc = 0x8021403c;
            }
            0x8021403c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80214040;
            }
            0x80214040 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x80214044;
            }
            0x80214044 => {
                if c.crbit(2) { pc = 0x80214060; continue; }
                pc = 0x80214048;
            }
            0x80214048 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x8021404c;
            }
            0x8021404c => {
                c.r[4] = 0x6;
                pc = 0x80214050;
            }
            0x80214050 => {
                c.lr = 0x80214054; f_80207ae0(c);
                pc = 0x80214054;
            }
            0x80214054 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80214058;
            }
            0x80214058 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x8021405c;
            }
            0x8021405c => {
                if !c.crbit(2) { pc = 0x80214068; continue; }
                pc = 0x80214060;
            }
            0x80214060 => {
                c.r[27] = 0x1e;
                pc = 0x80214064;
            }
            0x80214064 => {
                pc = 0x80214164; continue;
                pc = 0x80214068;
            }
            0x80214068 => {
                c.r[27] = 0xa;
                pc = 0x8021406c;
            }
            0x8021406c => {
                pc = 0x80214164; continue;
                pc = 0x80214070;
            }
            0x80214070 => {
                c.r[0] = c.r[22].wrapping_add(0xfffffffd);
                pc = 0x80214074;
            }
            0x80214074 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80214078;
            }
            0x80214078 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x8021407c;
            }
            0x8021407c => {
                if !c.crbit(1) { pc = 0x80214088; continue; }
                pc = 0x80214080;
            }
            0x80214080 => {
                c.cmpu(0, c.r[22], 0x5);
                pc = 0x80214084;
            }
            0x80214084 => {
                if !c.crbit(2) { pc = 0x80214090; continue; }
                pc = 0x80214088;
            }
            0x80214088 => {
                c.r[27] = 0x23;
                pc = 0x8021408c;
            }
            0x8021408c => {
                pc = 0x80214164; continue;
                pc = 0x80214090;
            }
            0x80214090 => {
                c.r[27] = 0xa;
                pc = 0x80214094;
            }
            0x80214094 => {
                pc = 0x80214164; continue;
                pc = 0x80214098;
            }
            0x80214098 => {
                c.cmpu(0, c.r[21], 0x28);
                pc = 0x8021409c;
            }
            0x8021409c => {
                if !c.crbit(0) { pc = 0x802140b8; continue; }
                pc = 0x802140a0;
            }
            0x802140a0 => {
                c.r[0] = c.addc(!c.r[21], 0x28, 1);
                pc = 0x802140a4;
            }
            0x802140a4 => {
                c.r[27] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x802140a8;
            }
            0x802140a8 => {
                c.cmpu(0, c.r[27], 0xa);
                pc = 0x802140ac;
            }
            0x802140ac => {
                if !c.crbit(0) { pc = 0x80214164; continue; }
                pc = 0x802140b0;
            }
            0x802140b0 => {
                c.r[27] = 0xa;
                pc = 0x802140b4;
            }
            0x802140b4 => {
                pc = 0x80214164; continue;
                pc = 0x802140b8;
            }
            0x802140b8 => {
                c.r[27] = 0xa;
                pc = 0x802140bc;
            }
            0x802140bc => {
                pc = 0x80214164; continue;
                pc = 0x802140c0;
            }
            0x802140c0 => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x802140c4;
            }
            0x802140c4 => {
                c.lr = 0x802140c8; f_8011fc74(c);
                pc = 0x802140c8;
            }
            0x802140c8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x802140cc;
            }
            0x802140cc => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x802140d0;
            }
            0x802140d0 => {
                if !c.crbit(2) { pc = 0x802140f0; continue; }
                pc = 0x802140d4;
            }
            0x802140d4 => {
                c.r[3] = c.r[24] | c.r[24];
                pc = 0x802140d8;
            }
            0x802140d8 => {
                c.lr = 0x802140dc; f_801eead0(c);
                pc = 0x802140dc;
            }
            0x802140dc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x802140e0;
            }
            0x802140e0 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x802140e4;
            }
            0x802140e4 => {
                if !c.crbit(2) { pc = 0x80214114; continue; }
                pc = 0x802140e8;
            }
            0x802140e8 => {
                c.r[20] = 0x1;
                pc = 0x802140ec;
            }
            0x802140ec => {
                pc = 0x80214114; continue;
                pc = 0x802140f0;
            }
            0x802140f0 => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x802140f4;
            }
            0x802140f4 => {
                c.r[4] = 0x0;
                pc = 0x802140f8;
            }
            0x802140f8 => {
                c.r[5] = 0x63;
                pc = 0x802140fc;
            }
            0x802140fc => {
                c.r[6] = 0x0;
                pc = 0x80214100;
            }
            0x80214100 => {
                c.lr = 0x80214104; f_8012640c(c);
                pc = 0x80214104;
            }
            0x80214104 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80214108;
            }
            0x80214108 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x8021410c;
            }
            0x8021410c => {
                if !c.crbit(2) { pc = 0x80214114; continue; }
                pc = 0x80214110;
            }
            0x80214110 => {
                c.r[20] = 0x1;
                pc = 0x80214114;
            }
            0x80214114 => {
                c.r[0] = c.r[20].rotate_left(0) & 0xff;
                pc = 0x80214118;
            }
            0x80214118 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x8021411c;
            }
            0x8021411c => {
                if !c.crbit(2) { pc = 0x80214128; continue; }
                pc = 0x80214120;
            }
            0x80214120 => {
                c.r[27] = 0x1e;
                pc = 0x80214124;
            }
            0x80214124 => {
                pc = 0x80214164; continue;
                pc = 0x80214128;
            }
            0x80214128 => {
                c.r[27] = 0xa;
                pc = 0x8021412c;
            }
            0x8021412c => {
                pc = 0x80214164; continue;
                pc = 0x80214130;
            }
            0x80214130 => {
                c.r[3] = 0x0;
                pc = 0x80214134;
            }
            0x80214134 => {
                c.r[4] = 0x0;
                pc = 0x80214138;
            }
            0x80214138 => {
                c.r[5] = 0xc;
                pc = 0x8021413c;
            }
            0x8021413c => {
                c.r[6] = 0x0;
                pc = 0x80214140;
            }
            0x80214140 => {
                c.lr = 0x80214144; f_801f54a4(c);
                pc = 0x80214144;
            }
            0x80214144 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80214148;
            }
            0x80214148 => {
                c.r[0] = c.r[3].wrapping_add(0xa);
                pc = 0x8021414c;
            }
            0x8021414c => {
                c.r[27] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x80214150;
            }
            0x80214150 => {
                c.cmpu(0, c.r[27], 0x28);
                pc = 0x80214154;
            }
            0x80214154 => {
                if !c.crbit(1) { pc = 0x80214164; continue; }
                pc = 0x80214158;
            }
            0x80214158 => {
                c.r[27] = 0x28;
                pc = 0x8021415c;
            }
            0x8021415c => {
                pc = 0x80214164; continue;
                pc = 0x80214160;
            }
            0x80214160 => {
                c.r[27] = 0xa;
                pc = 0x80214164;
            }
            0x80214164 => {
                c.cmpu(0, c.r[23], 0x1);
                pc = 0x80214168;
            }
            0x80214168 => {
                if c.crbit(2) { pc = 0x80214174; continue; }
                pc = 0x8021416c;
            }
            0x8021416c => {
                c.cmpu(0, c.r[31], 0x11);
                pc = 0x80214170;
            }
            0x80214170 => {
                if !c.crbit(2) { pc = 0x8021417c; continue; }
                pc = 0x80214174;
            }
            0x80214174 => {
                c.r[27] = 0x4;
                pc = 0x80214178;
            }
            0x80214178 => {
                pc = 0x802142d0; continue;
                pc = 0x8021417c;
            }
            0x8021417c => {
                c.r[3] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x80214180;
            }
            0x80214180 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                pc = 0x80214184;
            }
            0x80214184 => {
                c.r[6] = c.r[3].wrapping_mul(c.r[0]);
                pc = 0x80214188;
            }
            0x80214188 => {
                c.r[5] = 0xa;
                pc = 0x8021418c;
            }
            0x8021418c => {
                c.r[0] = c.r[28].rotate_left(1) & 0xfffffffe;
                pc = 0x80214190;
            }
            0x80214190 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x80214194;
            }
            0x80214194 => {
                c.r[4] = 0x8;
                pc = 0x80214198;
            }
            0x80214198 => {
                c.r[5] = divw(c.r[6], c.r[5]);
                pc = 0x8021419c;
            }
            0x8021419c => {
                c.r[6] = c.r[29].wrapping_mul(0x3);
                pc = 0x802141a0;
            }
            0x802141a0 => {
                c.r[0] = c.r[6].wrapping_sub(c.r[0]);
                pc = 0x802141a4;
            }
            0x802141a4 => {
                c.r[0] = c.r[5].wrapping_mul(c.r[0]);
                pc = 0x802141a8;
            }
            0x802141a8 => {
                c.r[20] = divw(c.r[0], c.r[6]);
                pc = 0x802141ac;
            }
            0x802141ac => {
                c.lr = 0x802141b0; f_802026e4(c);
                pc = 0x802141b0;
            }
            0x802141b0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x802141b4;
            }
            0x802141b4 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x802141b8;
            }
            0x802141b8 => {
                if c.crbit(2) { pc = 0x802141d4; continue; }
                pc = 0x802141bc;
            }
            0x802141bc => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x802141c0;
            }
            0x802141c0 => {
                c.r[4] = 0x7;
                pc = 0x802141c4;
            }
            0x802141c4 => {
                c.lr = 0x802141c8; f_802026e4(c);
                pc = 0x802141c8;
            }
            0x802141c8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x802141cc;
            }
            0x802141cc => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x802141d0;
            }
            0x802141d0 => {
                if !c.crbit(2) { pc = 0x802141d8; continue; }
                pc = 0x802141d4;
            }
            0x802141d4 => {
                c.r[20] = c.r[20].rotate_left(1) & 0xfffffffe;
                pc = 0x802141d8;
            }
            0x802141d8 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x802141dc;
            }
            0x802141dc => {
                c.r[4] = 0x3;
                pc = 0x802141e0;
            }
            0x802141e0 => {
                c.lr = 0x802141e4; f_802026e4(c);
                pc = 0x802141e4;
            }
            0x802141e4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x802141e8;
            }
            0x802141e8 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x802141ec;
            }
            0x802141ec => {
                if c.crbit(2) { pc = 0x80214238; continue; }
                pc = 0x802141f0;
            }
            0x802141f0 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x802141f4;
            }
            0x802141f4 => {
                c.r[4] = 0x4;
                pc = 0x802141f8;
            }
            0x802141f8 => {
                c.lr = 0x802141fc; f_802026e4(c);
                pc = 0x802141fc;
            }
            0x802141fc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80214200;
            }
            0x80214200 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x80214204;
            }
            0x80214204 => {
                if c.crbit(2) { pc = 0x80214238; continue; }
                pc = 0x80214208;
            }
            0x80214208 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x8021420c;
            }
            0x8021420c => {
                c.r[4] = 0x5;
                pc = 0x80214210;
            }
            0x80214210 => {
                c.lr = 0x80214214; f_802026e4(c);
                pc = 0x80214214;
            }
            0x80214214 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80214218;
            }
            0x80214218 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x8021421c;
            }
            0x8021421c => {
                if c.crbit(2) { pc = 0x80214238; continue; }
                pc = 0x80214220;
            }
            0x80214220 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x80214224;
            }
            0x80214224 => {
                c.r[4] = 0x6;
                pc = 0x80214228;
            }
            0x80214228 => {
                c.lr = 0x8021422c; f_802026e4(c);
                pc = 0x8021422c;
            }
            0x8021422c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80214230;
            }
            0x80214230 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x80214234;
            }
            0x80214234 => {
                if !c.crbit(2) { pc = 0x80214244; continue; }
                pc = 0x80214238;
            }
            0x80214238 => {
                c.r[3] = c.r[20].wrapping_mul(0xf);
                pc = 0x8021423c;
            }
            0x8021423c => {
                c.r[0] = 0xa;
                pc = 0x80214240;
            }
            0x80214240 => {
                c.r[20] = divwu(c.r[3], c.r[0]);
                pc = 0x80214244;
            }
            0x80214244 => {
                c.cmpu(0, c.r[20], 0xff);
                pc = 0x80214248;
            }
            0x80214248 => {
                if c.crbit(0) { pc = 0x80214254; continue; }
                pc = 0x8021424c;
            }
            0x8021424c => {
                c.r[27] = 0x4;
                pc = 0x80214250;
            }
            0x80214250 => {
                pc = 0x802142d0; continue;
                pc = 0x80214254;
            }
            0x80214254 => {
                c.r[3] = 0xff0000;
                pc = 0x80214258;
            }
            0x80214258 => {
                c.r[0] = 0x43300000;
                pc = 0x8021425c;
            }
            0x8021425c => {
                c.r[3] = divwu(c.r[3], c.r[20]);
                pc = 0x80214260;
            }
            0x80214260 => {
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                pc = 0x80214264;
            }
            0x80214264 => {
                { let ea = c.r[2].wrapping_add(0xffffaf70); c.f[1] = c.lfd(ea); }
                pc = 0x80214268;
            }
            0x80214268 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                pc = 0x8021426c;
            }
            0x8021426c => {
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfd(ea); }
                pc = 0x80214270;
            }
            0x80214270 => {
                c.f[1] = c.f[0] - c.f[1];
                pc = 0x80214274;
            }
            0x80214274 => {
                c.lr = 0x80214278; f_800ce77c(c);
                pc = 0x80214278;
            }
            0x80214278 => {
                c.lr = 0x8021427c; f_800c46b0(c);
                pc = 0x8021427c;
            }
            0x8021427c => {
                c.r[0] = 0x43300000;
                pc = 0x80214280;
            }
            0x80214280 => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[3]);
                pc = 0x80214284;
            }
            0x80214284 => {
                { let ea = c.r[2].wrapping_add(0xffffaf70); c.f[1] = c.lfd(ea); }
                pc = 0x80214288;
            }
            0x80214288 => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[0]);
                pc = 0x8021428c;
            }
            0x8021428c => {
                { let ea = c.r[1].wrapping_add(0x18); c.f[0] = c.lfd(ea); }
                pc = 0x80214290;
            }
            0x80214290 => {
                c.f[1] = c.f[0] - c.f[1];
                pc = 0x80214294;
            }
            0x80214294 => {
                c.lr = 0x80214298; f_800ce77c(c);
                pc = 0x80214298;
            }
            0x80214298 => {
                c.lr = 0x8021429c; f_800c46b0(c);
                pc = 0x8021429c;
            }
            0x8021429c => {
                c.r[4] = 0x100000;
                pc = 0x802142a0;
            }
            0x802142a0 => {
                c.r[27] = 0x0;
                pc = 0x802142a4;
            }
            0x802142a4 => {
                c.r[0] = c.r[4].wrapping_add(0xfffffff0);
                pc = 0x802142a8;
            }
            0x802142a8 => {
                c.r[20] = divwu(c.r[0], c.r[3]);
                pc = 0x802142ac;
            }
            0x802142ac => {
                pc = 0x802142c4; continue;
                pc = 0x802142b0;
            }
            0x802142b0 => {
                c.lr = 0x802142b4; f_800e0c54(c);
                pc = 0x802142b4;
            }
            0x802142b4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x802142b8;
            }
            0x802142b8 => {
                c.cmpu(0, c.r[0], c.r[20]);
                pc = 0x802142bc;
            }
            0x802142bc => {
                if !c.crbit(0) { pc = 0x802142d0; continue; }
                pc = 0x802142c0;
            }
            0x802142c0 => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x802142c4;
            }
            0x802142c4 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                pc = 0x802142c8;
            }
            0x802142c8 => {
                c.cmpu(0, c.r[0], 0x4);
                pc = 0x802142cc;
            }
            0x802142cc => {
                if c.crbit(0) { pc = 0x802142b0; continue; }
                pc = 0x802142d0;
            }
            0x802142d0 => {
                c.r[3] = c.r[19] | c.r[19];
                pc = 0x802142d4;
            }
            0x802142d4 => {
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x802142d8;
            }
            0x802142d8 => {
                c.r[5] = 0x0;
                pc = 0x802142dc;
            }
            0x802142dc => {
                c.lr = 0x802142e0; f_801fbad4(c);
                pc = 0x802142e0;
            }
            0x802142e0 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x802142e4;
            }
            0x802142e4 => {
                c.r[4] = c.r[27] | c.r[27];
                pc = 0x802142e8;
            }
            0x802142e8 => {
                c.r[5] = c.r[23] | c.r[23];
                pc = 0x802142ec;
            }
            0x802142ec => {
                c.r[7] = c.r[1].wrapping_add(0x8);
                pc = 0x802142f0;
            }
            0x802142f0 => {
                c.r[6] = 0x0;
                pc = 0x802142f4;
            }
            0x802142f4 => {
                c.lr = 0x802142f8; f_802087c0(c);
                pc = 0x802142f8;
            }
            0x802142f8 => {
                c.r[3] = c.r[19] | c.r[19];
                pc = 0x802142fc;
            }
            0x802142fc => {
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x80214300;
            }
            0x80214300 => {
                c.r[5] = 0x1;
                pc = 0x80214304;
            }
            0x80214304 => {
                c.lr = 0x80214308; f_801fbad4(c);
                pc = 0x80214308;
            }
            0x80214308 => {
                c.r[3] = c.r[19] | c.r[19];
                pc = 0x8021430c;
            }
            0x8021430c => {
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x80214310;
            }
            0x80214310 => {
                c.r[5] = 0x3;
                pc = 0x80214314;
            }
            0x80214314 => {
                c.lr = 0x80214318; f_801fbad4(c);
                pc = 0x80214318;
            }
            0x80214318 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x8021431c;
            }
            0x8021431c => {
                c.r[4] = c.r[27] | c.r[27];
                pc = 0x80214320;
            }
            0x80214320 => {
                c.r[5] = c.r[23] | c.r[23];
                pc = 0x80214324;
            }
            0x80214324 => {
                c.r[7] = c.r[1].wrapping_add(0x8);
                pc = 0x80214328;
            }
            0x80214328 => {
                c.r[6] = 0x1;
                pc = 0x8021432c;
            }
            0x8021432c => {
                c.lr = 0x80214330; f_802087c0(c);
                pc = 0x80214330;
            }
            0x80214330 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x80214334;
            }
            0x80214334 => {
                c.lr = 0x80214338; f_802037dc(c);
                pc = 0x80214338;
            }
            0x80214338 => {
                c.r[4] = c.r[3] | c.r[3];
                pc = 0x8021433c;
            }
            0x8021433c => {
                c.r[3] = 0x16;
                pc = 0x80214340;
            }
            0x80214340 => {
                c.lr = 0x80214344; f_80132a38(c);
                pc = 0x80214344;
            }
            0x80214344 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                pc = 0x80214348;
            }
            0x80214348 => {
                c.cmpu(0, c.r[0], 0x4);
                pc = 0x8021434c;
            }
            0x8021434c => {
                if c.crbit(0) { pc = 0x802143a8; continue; }
                pc = 0x80214350;
            }
            0x80214350 => {
                c.r[3] = c.r[19] | c.r[19];
                pc = 0x80214354;
            }
            0x80214354 => {
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x80214358;
            }
            0x80214358 => {
                c.r[5] = 0x4;
                pc = 0x8021435c;
            }
            0x8021435c => {
                c.lr = 0x80214360; f_801fbad4(c);
                pc = 0x80214360;
            }
            0x80214360 => {
                c.r[3] = 0x5d;
                pc = 0x80214364;
            }
            0x80214364 => {
                c.r[4] = 0x0;
                pc = 0x80214368;
            }
            0x80214368 => {
                c.lr = 0x8021436c; f_80132a38(c);
                pc = 0x8021436c;
            }
            0x8021436c => {
                c.r[3] = 0x771f;
                pc = 0x80214370;
            }
            0x80214370 => {
                c.lr = 0x80214374; f_802624cc(c);
                pc = 0x80214374;
            }
            0x80214374 => {
                c.r[3] = 0x3f6;
                pc = 0x80214378;
            }
            0x80214378 => {
                c.r[4] = 0x0;
                pc = 0x8021437c;
            }
            0x8021437c => {
                c.r[5] = 0xff;
                pc = 0x80214380;
            }
            0x80214380 => {
                c.lr = 0x80214384; f_80165668(c);
                pc = 0x80214384;
            }
            0x80214384 => {
                c.r[3] = c.r[19] | c.r[19];
                pc = 0x80214388;
            }
            0x80214388 => {
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x8021438c;
            }
            0x8021438c => {
                c.r[5] = 0x5;
                pc = 0x80214390;
            }
            0x80214390 => {
                c.lr = 0x80214394; f_801fbad4(c);
                pc = 0x80214394;
            }
            0x80214394 => {
                c.lr = 0x80214398; f_8026246c(c);
                pc = 0x80214398;
            }
            0x80214398 => {
                c.r[3] = 0x80370000;
                pc = 0x8021439c;
            }
            0x8021439c => {
                c.r[0] = c.r[3].wrapping_add(0x5e51);
                pc = 0x802143a0;
            }
            0x802143a0 => {
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802143a4;
            }
            0x802143a4 => {
                pc = 0x80214404; continue;
                pc = 0x802143a8;
            }
            0x802143a8 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x802143ac;
            }
            0x802143ac => {
                c.r[4] = c.r[27] | c.r[27];
                pc = 0x802143b0;
            }
            0x802143b0 => {
                c.r[5] = c.r[23] | c.r[23];
                pc = 0x802143b4;
            }
            0x802143b4 => {
                c.r[7] = c.r[1].wrapping_add(0x8);
                pc = 0x802143b8;
            }
            0x802143b8 => {
                c.r[6] = 0x2;
                pc = 0x802143bc;
            }
            0x802143bc => {
                c.lr = 0x802143c0; f_802087c0(c);
                pc = 0x802143c0;
            }
            0x802143c0 => {
                c.r[3] = 0x80280000;
                pc = 0x802143c4;
            }
            0x802143c4 => {
                c.r[4] = c.r[13].wrapping_add(0xffff8558);
                pc = 0x802143c8;
            }
            0x802143c8 => {
                c.r[0] = c.r[27].rotate_left(2) & 0x3fc;
                pc = 0x802143cc;
            }
            0x802143cc => {
                c.stb(c.r[4].wrapping_add(0x5), c.r[27]);
                pc = 0x802143d0;
            }
            0x802143d0 => {
                c.r[3] = c.r[3].wrapping_add(0xffff9e1c);
                pc = 0x802143d4;
            }
            0x802143d4 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x802143d8;
            }
            0x802143d8 => {
                c.lr = 0x802143dc; f_802624cc(c);
                pc = 0x802143dc;
            }
            0x802143dc => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x802143e0;
            }
            0x802143e0 => {
                c.r[4] = c.r[27] | c.r[27];
                pc = 0x802143e4;
            }
            0x802143e4 => {
                c.r[5] = c.r[23] | c.r[23];
                pc = 0x802143e8;
            }
            0x802143e8 => {
                c.r[7] = c.r[1].wrapping_add(0x8);
                pc = 0x802143ec;
            }
            0x802143ec => {
                c.r[6] = 0x3;
                pc = 0x802143f0;
            }
            0x802143f0 => {
                c.lr = 0x802143f4; f_802087c0(c);
                pc = 0x802143f4;
            }
            0x802143f4 => {
                c.lr = 0x802143f8; f_8026246c(c);
                pc = 0x802143f8;
            }
            0x802143f8 => {
                c.r[3] = 0x80370000;
                pc = 0x802143fc;
            }
            0x802143fc => {
                c.r[0] = c.r[3].wrapping_add(0x5e5f);
                pc = 0x80214400;
            }
            0x80214400 => {
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214404;
            }
            0x80214404 => {
                c.r[3] = c.r[19] | c.r[19];
                pc = 0x80214408;
            }
            0x80214408 => {
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x8021440c;
            }
            0x8021440c => {
                c.r[5] = 0x2;
                pc = 0x80214410;
            }
            0x80214410 => {
                c.lr = 0x80214414; f_801fbad4(c);
                pc = 0x80214414;
            }
            0x80214414 => {
                c.r[3] = c.r[19] | c.r[19];
                pc = 0x80214418;
            }
            0x80214418 => {
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x8021441c;
            }
            0x8021441c => {
                c.r[5] = 0x6;
                pc = 0x80214420;
            }
            0x80214420 => {
                c.lr = 0x80214424; f_801fbad4(c);
                pc = 0x80214424;
            }
            0x80214424 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x80214428;
            }
            0x80214428 => {
                c.r[4] = c.r[27] | c.r[27];
                pc = 0x8021442c;
            }
            0x8021442c => {
                c.r[5] = c.r[23] | c.r[23];
                pc = 0x80214430;
            }
            0x80214430 => {
                c.r[7] = c.r[1].wrapping_add(0x8);
                pc = 0x80214434;
            }
            0x80214434 => {
                c.r[6] = 0x4;
                pc = 0x80214438;
            }
            0x80214438 => {
                c.lr = 0x8021443c; f_802087c0(c);
                pc = 0x8021443c;
            }
            0x8021443c => {
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 19..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                pc = 0x80214440;
            }
            0x80214440 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                pc = 0x80214444;
            }
            0x80214444 => {
                c.mtspr(8, c.r[0]);
                pc = 0x80214448;
            }
            0x80214448 => {
                c.r[1] = c.r[1].wrapping_add(0x60);
                pc = 0x8021444c;
            }
            0x8021444c => {
                if true { return; }
                return;
            }
            _ => panic!("f_80213e94: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214450 (0x80214450, 376 bytes)
#[inline(never)]
pub fn f_80214450(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214450) { return; }
        if c.prof { crate::prof::enter(c, 0x80214450); f_80214450_b(c, 0x80214450); crate::prof::exit(c, 0x80214450); return; }
    }
    f_80214450_b(c, 0x80214450)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214450_at(c: &mut Cpu, pc: u32) { f_80214450_b(c, pc) }
#[inline(always)]
fn f_80214450_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214450;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214450, pc); }
        match pc {
            0x80214450 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x8021446c; f_801f025c(c);
                pc = 0x8021446c;
            }
            0x8021446c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x3;
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x8021447c; f_801f025c(c);
                pc = 0x8021447c;
            }
            0x8021447c => {
                c.r[4] = 0x49;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80214488; f_801f6e98(c);
                pc = 0x80214488;
            }
            0x80214488 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x802144ac; continue; }
                pc = 0x80214494;
            }
            0x80214494 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x48;
                c.lr = 0x802144a0; f_801f6e98(c);
                pc = 0x802144a0;
            }
            0x802144a0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80214590; continue; }
                pc = 0x802144ac;
            }
            0x802144ac => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x49;
                c.lr = 0x802144b8; f_801f6e98(c);
                pc = 0x802144b8;
            }
            0x802144b8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80214510; continue; }
                pc = 0x802144c4;
            }
            0x802144c4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x48;
                c.lr = 0x802144d0; f_801f6e98(c);
                pc = 0x802144d0;
            }
            0x802144d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80214510; continue; }
                pc = 0x802144dc;
            }
            0x802144dc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x49;
                c.lr = 0x802144e8; f_801f6eec(c);
                pc = 0x802144e8;
            }
            0x802144e8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x48;
                c.lr = 0x802144f4; f_801f6eec(c);
                pc = 0x802144f4;
            }
            0x802144f4 => {
                c.r[3] = 0x80380000;
                c.r[0] = 0x3;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stb(c.r[3].wrapping_add(0x6002), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x60a1), c.r[0]);
                pc = 0x802145a8; continue;
                pc = 0x80214510;
            }
            0x80214510 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x49;
                c.lr = 0x8021451c; f_801f6e98(c);
                pc = 0x8021451c;
            }
            0x8021451c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80214550; continue; }
                pc = 0x80214528;
            }
            0x80214528 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x49;
                c.lr = 0x80214534; f_801f6eec(c);
                pc = 0x80214534;
            }
            0x80214534 => {
                c.r[3] = 0x80380000;
                c.r[0] = 0x1;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stb(c.r[3].wrapping_add(0x6002), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x60a1), c.r[0]);
                pc = 0x802145a8; continue;
                pc = 0x80214550;
            }
            0x80214550 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x48;
                c.lr = 0x8021455c; f_801f6e98(c);
                pc = 0x8021455c;
            }
            0x8021455c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802145a8; continue; }
                pc = 0x80214568;
            }
            0x80214568 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x48;
                c.lr = 0x80214574; f_801f6eec(c);
                pc = 0x80214574;
            }
            0x80214574 => {
                c.r[3] = 0x80380000;
                c.r[0] = 0x2;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stb(c.r[3].wrapping_add(0x6002), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x60a1), c.r[0]);
                pc = 0x802145a8; continue;
                pc = 0x80214590;
            }
            0x80214590 => {
                c.r[3] = 0x80380000;
                c.r[0] = 0x0;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stb(c.r[3].wrapping_add(0x6002), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x60a1), c.r[0]);
                pc = 0x802145a8;
            }
            0x802145a8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80214450: bad pc {:#x}", pc),
        }
    }
}

/// fn_802145C8 (0x802145c8, 236 bytes)
#[inline(never)]
pub fn f_802145c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802145c8) { return; }
        if c.prof { crate::prof::enter(c, 0x802145c8); f_802145c8_b(c, 0x802145c8); crate::prof::exit(c, 0x802145c8); return; }
    }
    f_802145c8_b(c, 0x802145c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802145c8_at(c: &mut Cpu, pc: u32) { f_802145c8_b(c, pc) }
#[inline(always)]
fn f_802145c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802145c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802145c8, pc); }
        match pc {
            0x802145c8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802145e4; f_801f025c(c);
                pc = 0x802145e4;
            }
            0x802145e4 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x802145f8; f_801f025c(c);
                pc = 0x802145f8;
            }
            0x802145f8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x19;
                c.r[4] = 0x0;
                c.lr = 0x80214608; f_801f025c(c);
                pc = 0x80214608;
            }
            0x80214608 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[29] | c.r[29];
                c.r[7] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x47;
                c.r[6] = 0x0;
                c.lr = 0x80214628; f_801f4c14(c);
                pc = 0x80214628;
            }
            0x80214628 => {
                c.r[0] = c.r[31].wrapping_sub(c.r[29]);
                c.r[0] = c.r[0].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80214640; continue; }
                pc = 0x80214638;
            }
            0x80214638 => {
                c.r[30] = c.r[27] | c.r[27];
                c.r[29] = c.r[27] | c.r[27];
                pc = 0x80214640;
            }
            0x80214640 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8021464c; continue; }
                pc = 0x80214648;
            }
            0x80214648 => {
                c.r[30] = c.r[27] | c.r[27];
                pc = 0x8021464c;
            }
            0x8021464c => {
                c.r[7] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x4b;
                c.r[6] = 0x0;
                c.lr = 0x80214664; f_801f4c14(c);
                pc = 0x80214664;
            }
            0x80214664 => {
                c.r[7] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8021467c; f_801f4c14(c);
                pc = 0x8021467c;
            }
            0x8021467c => {
                c.r[7] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80214694; f_801f4c14(c);
                pc = 0x80214694;
            }
            0x80214694 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802145c8: bad pc {:#x}", pc),
        }
    }
}

/// fn_802146B4 (0x802146b4, 16 bytes)
#[inline(never)]
pub fn f_802146b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802146b4) { return; }
        if c.prof { crate::prof::enter(c, 0x802146b4); f_802146b4_b(c, 0x802146b4); crate::prof::exit(c, 0x802146b4); return; }
    }
    f_802146b4_b(c, 0x802146b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802146b4_at(c: &mut Cpu, pc: u32) { f_802146b4_b(c, pc) }
#[inline(always)]
fn f_802146b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802146b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802146b4, pc); }
        match pc {
            0x802146b4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_802146b4: bad pc {:#x}", pc),
        }
    }
}

/// fn_802146C4 (0x802146c4, 208 bytes)
#[inline(never)]
pub fn f_802146c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802146c4) { return; }
        if c.prof { crate::prof::enter(c, 0x802146c4); f_802146c4_b(c, 0x802146c4); crate::prof::exit(c, 0x802146c4); return; }
    }
    f_802146c4_b(c, 0x802146c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802146c4_at(c: &mut Cpu, pc: u32) { f_802146c4_b(c, pc) }
#[inline(always)]
fn f_802146c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802146c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802146c4, pc); }
        match pc {
            0x802146c4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[5] = 0xf;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802146e8; f_801f54a4(c);
                pc = 0x802146e8;
            }
            0x802146e8 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x802146f8; f_801f025c(c);
                pc = 0x802146f8;
            }
            0x802146f8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x80214708; f_80136428(c);
                pc = 0x80214708;
            }
            0x80214708 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[31] = c.r[0].rotate_left(0) & 0xff;
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8021471c; f_80207ae0(c);
                pc = 0x8021471c;
            }
            0x8021471c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80214774; continue; }
                pc = 0x80214724;
            }
            0x80214724 => {
                c.r[29] = 0x0;
                pc = 0x80214740; continue;
                pc = 0x8021472c;
            }
            0x8021472c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8021473c; f_80207b5c(c);
                pc = 0x8021473c;
            }
            0x8021473c => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x80214740;
            }
            0x80214740 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8021472c; continue; }
                pc = 0x8021474c;
            }
            0x8021474c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80214754; f_8010c4d4(c);
                pc = 0x80214754;
            }
            0x80214754 => {
                c.lr = 0x80214758; f_800fa280(c);
                pc = 0x80214758;
            }
            0x80214758 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80214764; f_80132a38(c);
                pc = 0x80214764;
            }
            0x80214764 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214780; continue;
                pc = 0x80214774;
            }
            0x80214774 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214780;
            }
            0x80214780 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802146c4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214794 (0x80214794, 208 bytes)
#[inline(never)]
pub fn f_80214794(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214794) { return; }
        if c.prof { crate::prof::enter(c, 0x80214794); f_80214794_b(c, 0x80214794); crate::prof::exit(c, 0x80214794); return; }
    }
    f_80214794_b(c, 0x80214794)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214794_at(c: &mut Cpu, pc: u32) { f_80214794_b(c, pc) }
#[inline(always)]
fn f_80214794_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214794;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214794, pc); }
        match pc {
            0x80214794 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802147b0; f_801f025c(c);
                pc = 0x802147b0;
            }
            0x802147b0 => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x802147b8; f_802040e8(c);
                pc = 0x802147b8;
            }
            0x802147b8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xfa;
                c.r[6] = 0x0;
                c.lr = 0x802147d0; f_8012640c(c);
                pc = 0x802147d0;
            }
            0x802147d0 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x80214844; continue; }
                pc = 0x802147d8;
            }
            0x802147d8 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80214844; continue; }
                pc = 0x802147e0;
            }
            0x802147e0 => {
                c.r[7] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x56;
                c.r[6] = 0x0;
                c.lr = 0x802147f8; f_801f4c14(c);
                pc = 0x802147f8;
            }
            0x802147f8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.r[6] = 0x1;
                c.lr = 0x8021480c; f_8020147c(c);
                pc = 0x8021480c;
            }
            0x8021480c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80214814; f_801fecd4(c);
                pc = 0x80214814;
            }
            0x80214814 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80214834; continue; }
                pc = 0x80214820;
            }
            0x80214820 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x82;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80214834; f_801fe7ec(c);
                pc = 0x80214834;
            }
            0x80214834 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214850; continue;
                pc = 0x80214844;
            }
            0x80214844 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214850;
            }
            0x80214850 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80214794: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214864 (0x80214864, 340 bytes)
#[inline(never)]
pub fn f_80214864(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214864) { return; }
        if c.prof { crate::prof::enter(c, 0x80214864); f_80214864_b(c, 0x80214864); crate::prof::exit(c, 0x80214864); return; }
    }
    f_80214864_b(c, 0x80214864)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214864_at(c: &mut Cpu, pc: u32) { f_80214864_b(c, pc) }
#[inline(always)]
fn f_80214864_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214864;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214864, pc); }
        match pc {
            0x80214864 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80214880; f_801f453c(c);
                pc = 0x80214880;
            }
            0x80214880 => {
                c.r[30] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80214890; f_801f025c(c);
                pc = 0x80214890;
            }
            0x80214890 => {
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x802148a0; f_8012640c(c);
                pc = 0x802148a0;
            }
            0x802148a0 => {
                c.cmpu(0, c.r[30], 0x2);
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x802148dc; continue; }
                pc = 0x802148ac;
            }
            0x802148ac => {
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.r[7] = 0xb;
                c.lr = 0x802148c0; f_8011bbd8(c);
                pc = 0x802148c0;
            }
            0x802148c0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x2;
                c.lr = 0x802148d8; f_8011bbd8(c);
                pc = 0x802148d8;
            }
            0x802148d8 => {
                pc = 0x80214998; continue;
                pc = 0x802148dc;
            }
            0x802148dc => {
                c.cmpu(0, c.r[30], 0x3);
                if !c.crbit(2) { pc = 0x80214914; continue; }
                pc = 0x802148e4;
            }
            0x802148e4 => {
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.r[7] = 0x5;
                c.lr = 0x802148f8; f_8011bbd8(c);
                pc = 0x802148f8;
            }
            0x802148f8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x2;
                c.lr = 0x80214910; f_8011bbd8(c);
                pc = 0x80214910;
            }
            0x80214910 => {
                pc = 0x80214998; continue;
                pc = 0x80214914;
            }
            0x80214914 => {
                c.cmpu(0, c.r[30], 0x1);
                if !c.crbit(2) { pc = 0x8021494c; continue; }
                pc = 0x8021491c;
            }
            0x8021491c => {
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.r[7] = 0xa;
                c.lr = 0x80214930; f_8011bbd8(c);
                pc = 0x80214930;
            }
            0x80214930 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x2;
                c.lr = 0x80214948; f_8011bbd8(c);
                pc = 0x80214948;
            }
            0x80214948 => {
                pc = 0x80214998; continue;
                pc = 0x8021494c;
            }
            0x8021494c => {
                c.cmpu(0, c.r[30], 0x4);
                if !c.crbit(2) { pc = 0x80214984; continue; }
                pc = 0x80214954;
            }
            0x80214954 => {
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.r[7] = 0xf;
                c.lr = 0x80214968; f_8011bbd8(c);
                pc = 0x80214968;
            }
            0x80214968 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x2;
                c.lr = 0x80214980; f_8011bbd8(c);
                pc = 0x80214980;
            }
            0x80214980 => {
                pc = 0x80214998; continue;
                pc = 0x80214984;
            }
            0x80214984 => {
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80214998; f_8011bbd8(c);
                pc = 0x80214998;
            }
            0x80214998 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80214864: bad pc {:#x}", pc),
        }
    }
}

/// fn_802149B8 (0x802149b8, 252 bytes)
#[inline(never)]
pub fn f_802149b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802149b8) { return; }
        if c.prof { crate::prof::enter(c, 0x802149b8); f_802149b8_b(c, 0x802149b8); crate::prof::exit(c, 0x802149b8); return; }
    }
    f_802149b8_b(c, 0x802149b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802149b8_at(c: &mut Cpu, pc: u32) { f_802149b8_b(c, pc) }
#[inline(always)]
fn f_802149b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802149b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802149b8, pc); }
        match pc {
            0x802149b8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802149d4; f_801f025c(c);
                pc = 0x802149d4;
            }
            0x802149d4 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x802149dc; f_80207bf4(c);
                pc = 0x802149dc;
            }
            0x802149dc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802149e4; f_80205184(c);
                pc = 0x802149e4;
            }
            0x802149e4 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x802149fc; f_8011beb4(c);
                pc = 0x802149fc;
            }
            0x802149fc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[30] = 0x0;
                c.cmpu(0, c.r[0], 0xc9);
                if !c.crbit(2) { pc = 0x80214a48; continue; }
                pc = 0x80214a0c;
            }
            0x80214a0c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x38;
                c.lr = 0x80214a18; f_802025b8(c);
                pc = 0x80214a18;
            }
            0x80214a18 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80214a80; continue; }
                pc = 0x80214a24;
            }
            0x80214a24 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x38;
                c.r[5] = 0x0;
                c.lr = 0x80214a34; f_8020248c(c);
                pc = 0x80214a34;
            }
            0x80214a34 => {
                c.r[0] = 0x0;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                c.r[30] = 0x1;
                pc = 0x80214a80; continue;
                pc = 0x80214a48;
            }
            0x80214a48 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x39;
                c.lr = 0x80214a54; f_802025b8(c);
                pc = 0x80214a54;
            }
            0x80214a54 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80214a80; continue; }
                pc = 0x80214a60;
            }
            0x80214a60 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x39;
                c.r[5] = 0x0;
                c.lr = 0x80214a70; f_8020248c(c);
                pc = 0x80214a70;
            }
            0x80214a70 => {
                c.r[0] = 0x1;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                c.r[30] = 0x1;
                pc = 0x80214a80;
            }
            0x80214a80 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80214a94; continue; }
                pc = 0x80214a88;
            }
            0x80214a88 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                pc = 0x80214a9c; continue;
                pc = 0x80214a94;
            }
            0x80214a94 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                pc = 0x80214a9c;
            }
            0x80214a9c => {
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802149b8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214AB4 (0x80214ab4, 72 bytes)
#[inline(never)]
pub fn f_80214ab4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214ab4) { return; }
        if c.prof { crate::prof::enter(c, 0x80214ab4); f_80214ab4_b(c, 0x80214ab4); crate::prof::exit(c, 0x80214ab4); return; }
    }
    f_80214ab4_b(c, 0x80214ab4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214ab4_at(c: &mut Cpu, pc: u32) { f_80214ab4_b(c, pc) }
#[inline(always)]
fn f_80214ab4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214ab4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214ab4, pc); }
        match pc {
            0x80214ab4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80230000;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = c.r[3].wrapping_add(0xffffeb9c);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[3] = 0x0;
                c.lr = 0x80214ae0; f_801f37b0(c);
                pc = 0x80214ae0;
            }
            0x80214ae0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80214ab4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214AFC (0x80214afc, 92 bytes)
#[inline(never)]
pub fn f_80214afc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214afc) { return; }
        if c.prof { crate::prof::enter(c, 0x80214afc); f_80214afc_b(c, 0x80214afc); crate::prof::exit(c, 0x80214afc); return; }
    }
    f_80214afc_b(c, 0x80214afc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214afc_at(c: &mut Cpu, pc: u32) { f_80214afc_b(c, pc) }
#[inline(always)]
fn f_80214afc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214afc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214afc, pc); }
        match pc {
            0x80214afc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x19;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80214b14; f_801f025c(c);
                pc = 0x80214b14;
            }
            0x80214b14 => {
                c.r[4] = 0x14;
                c.lr = 0x80214b1c; f_802026e4(c);
                pc = 0x80214b1c;
            }
            0x80214b1c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80214b3c; continue; }
                pc = 0x80214b24;
            }
            0x80214b24 => {
                c.r[3] = 0x80380000;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x609b));
                c.r[0] = c.r[0] | 0x80;
                c.stb(c.r[3].wrapping_add(0x609b), c.r[0]);
                pc = 0x80214b3c;
            }
            0x80214b3c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80214afc: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214B58 (0x80214b58, 16 bytes)
#[inline(never)]
pub fn f_80214b58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214b58) { return; }
        if c.prof { crate::prof::enter(c, 0x80214b58); f_80214b58_b(c, 0x80214b58); crate::prof::exit(c, 0x80214b58); return; }
    }
    f_80214b58_b(c, 0x80214b58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214b58_at(c: &mut Cpu, pc: u32) { f_80214b58_b(c, pc) }
#[inline(always)]
fn f_80214b58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214b58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214b58, pc); }
        match pc {
            0x80214b58 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80214b58: bad pc {:#x}", pc),
        }
    }
}

/// WS_HIMITUNOTIKARA (0x80214b68, 76 bytes)
#[inline(never)]
pub fn f_80214b68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214b68) { return; }
        if c.prof { crate::prof::enter(c, 0x80214b68); f_80214b68_b(c, 0x80214b68); crate::prof::exit(c, 0x80214b68); return; }
    }
    f_80214b68_b(c, 0x80214b68)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214b68_at(c: &mut Cpu, pc: u32) { f_80214b68_b(c, pc) }
#[inline(always)]
fn f_80214b68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214b68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214b68, pc); }
        match pc {
            0x80214b68 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0xf;
                c.r[6] = 0x0;
                c.lr = 0x80214b88; f_801f54a4(c);
                pc = 0x80214b88;
            }
            0x80214b88 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x80214b90; f_80136468(c);
                pc = 0x80214b90;
            }
            0x80214b90 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[5] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[5].wrapping_add(0x3), c.r[3]);
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80214b68: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214BB4 (0x80214bb4, 80 bytes)
#[inline(never)]
pub fn f_80214bb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214bb4) { return; }
        if c.prof { crate::prof::enter(c, 0x80214bb4); f_80214bb4_b(c, 0x80214bb4); crate::prof::exit(c, 0x80214bb4); return; }
    }
    f_80214bb4_b(c, 0x80214bb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214bb4_at(c: &mut Cpu, pc: u32) { f_80214bb4_b(c, pc) }
#[inline(always)]
fn f_80214bb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214bb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214bb4, pc); }
        match pc {
            0x80214bb4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                c.lr = 0x80214bd0; f_801f025c(c);
                pc = 0x80214bd0;
            }
            0x80214bd0 => {
                c.lr = 0x80214bd4; f_802062fc(c);
                pc = 0x80214bd4;
            }
            0x80214bd4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80214be8; continue; }
                pc = 0x80214bdc;
            }
            0x80214bdc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x2));
                pc = 0x80214bf0; continue;
                pc = 0x80214be8;
            }
            0x80214be8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x6);
                pc = 0x80214bf0;
            }
            0x80214bf0 => {
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80214bb4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214C04 (0x80214c04, 172 bytes)
#[inline(never)]
pub fn f_80214c04(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214c04) { return; }
        if c.prof { crate::prof::enter(c, 0x80214c04); f_80214c04_b(c, 0x80214c04); crate::prof::exit(c, 0x80214c04); return; }
    }
    f_80214c04_b(c, 0x80214c04)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214c04_at(c: &mut Cpu, pc: u32) { f_80214c04_b(c, pc) }
#[inline(always)]
fn f_80214c04_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214c04;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214c04, pc); }
        match pc {
            0x80214c04 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                c.lr = 0x80214c24; f_801f025c(c);
                pc = 0x80214c24;
            }
            0x80214c24 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80214c2c; f_80205b8c(c);
                pc = 0x80214c2c;
            }
            0x80214c2c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x80214c3c; f_80207bf4(c);
                pc = 0x80214c3c;
            }
            0x80214c3c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1e);
                if !c.crbit(2) { pc = 0x80214c90; continue; }
                pc = 0x80214c48;
            }
            0x80214c48 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80214c50; f_801252e0(c);
                pc = 0x80214c50;
            }
            0x80214c50 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x80214c5c; f_80202998(c);
                pc = 0x80214c5c;
            }
            0x80214c5c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x17;
                c.lr = 0x80214c68; f_80202810(c);
                pc = 0x80214c68;
            }
            0x80214c68 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80214c70; f_801fecd4(c);
                pc = 0x80214c70;
            }
            0x80214c70 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80214c90; continue; }
                pc = 0x80214c7c;
            }
            0x80214c7c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7c;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80214c90; f_801fe7ec(c);
                pc = 0x80214c90;
            }
            0x80214c90 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x2);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80214c04: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214CB0 (0x80214cb0, 76 bytes)
#[inline(never)]
pub fn f_80214cb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214cb0) { return; }
        if c.prof { crate::prof::enter(c, 0x80214cb0); f_80214cb0_b(c, 0x80214cb0); crate::prof::exit(c, 0x80214cb0); return; }
    }
    f_80214cb0_b(c, 0x80214cb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214cb0_at(c: &mut Cpu, pc: u32) { f_80214cb0_b(c, pc) }
#[inline(always)]
fn f_80214cb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214cb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214cb0, pc); }
        match pc {
            0x80214cb0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x19;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80214cc8; f_801f025c(c);
                pc = 0x80214cc8;
            }
            0x80214cc8 => {
                c.r[4] = 0x80210000;
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x4cfc);
                c.lr = 0x80214ce0; f_801f37b0(c);
                pc = 0x80214ce0;
            }
            0x80214ce0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80214cb0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214CFC (0x80214cfc, 180 bytes)
#[inline(never)]
pub fn f_80214cfc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214cfc) { return; }
        if c.prof { crate::prof::enter(c, 0x80214cfc); f_80214cfc_b(c, 0x80214cfc); crate::prof::exit(c, 0x80214cfc); return; }
    }
    f_80214cfc_b(c, 0x80214cfc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214cfc_at(c: &mut Cpu, pc: u32) { f_80214cfc_b(c, pc) }
#[inline(always)]
fn f_80214cfc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214cfc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214cfc, pc); }
        match pc {
            0x80214cfc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80214d1c; f_80207bf4(c);
                pc = 0x80214d1c;
            }
            0x80214d1c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[29] = c.r[0] | c.r[0];
                c.lr = 0x80214d2c; f_802062fc(c);
                pc = 0x80214d2c;
            }
            0x80214d2c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80214d3c; continue; }
                pc = 0x80214d34;
            }
            0x80214d34 => {
                c.r[3] = 0x1;
                pc = 0x80214d9c; continue;
                pc = 0x80214d3c;
            }
            0x80214d3c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80214d48; f_80202b88(c);
                pc = 0x80214d48;
            }
            0x80214d48 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80214d98; continue; }
                pc = 0x80214d50;
            }
            0x80214d50 => {
                c.r[7] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x80214d68; f_801f4c14(c);
                pc = 0x80214d68;
            }
            0x80214d68 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80214d70; f_8011cb6c(c);
                pc = 0x80214d70;
            }
            0x80214d70 => {
                c.lr = 0x80214d74; f_8011cb54(c);
                pc = 0x80214d74;
            }
            0x80214d74 => {
                c.lr = 0x80214d78; f_800fa280(c);
                pc = 0x80214d78;
            }
            0x80214d78 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80214d84; f_80132a38(c);
                pc = 0x80214d84;
            }
            0x80214d84 => {
                c.r[4] = 0x80380000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae0c));
                c.r[4] = c.r[4].wrapping_add(0xffff960a);
                c.r[5] = 0x0;
                c.lr = 0x80214d98; f_80211b94(c);
                pc = 0x80214d98;
            }
            0x80214d98 => {
                c.r[3] = 0x1;
                pc = 0x80214d9c;
            }
            0x80214d9c => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80214cfc: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214DB0 (0x80214db0, 160 bytes)
#[inline(never)]
pub fn f_80214db0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214db0) { return; }
        if c.prof { crate::prof::enter(c, 0x80214db0); f_80214db0_b(c, 0x80214db0); crate::prof::exit(c, 0x80214db0); return; }
    }
    f_80214db0_b(c, 0x80214db0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214db0_at(c: &mut Cpu, pc: u32) { f_80214db0_b(c, pc) }
#[inline(always)]
fn f_80214db0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214db0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214db0, pc); }
        match pc {
            0x80214db0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80214dcc; f_801f025c(c);
                pc = 0x80214dcc;
            }
            0x80214dcc => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0x118;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80214de4; f_801254b4(c);
                pc = 0x80214de4;
            }
            0x80214de4 => {
                c.r[3] = 0x0;
                c.lr = 0x80214dec; f_801f221c(c);
                pc = 0x80214dec;
            }
            0x80214dec => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80214e08; continue; }
                pc = 0x80214df8;
            }
            0x80214df8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214e3c; continue;
                pc = 0x80214e08;
            }
            0x80214e08 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x33;
                c.lr = 0x80214e14; f_802025b8(c);
                pc = 0x80214e14;
            }
            0x80214e14 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80214df8; continue; }
                pc = 0x80214e20;
            }
            0x80214e20 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x33;
                c.r[5] = 0x0;
                c.lr = 0x80214e30; f_8020248c(c);
                pc = 0x80214e30;
            }
            0x80214e30 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214e3c;
            }
            0x80214e3c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80214db0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214E50 (0x80214e50, 192 bytes)
#[inline(never)]
pub fn f_80214e50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214e50) { return; }
        if c.prof { crate::prof::enter(c, 0x80214e50); f_80214e50_b(c, 0x80214e50); crate::prof::exit(c, 0x80214e50); return; }
    }
    f_80214e50_b(c, 0x80214e50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214e50_at(c: &mut Cpu, pc: u32) { f_80214e50_b(c, pc) }
#[inline(always)]
fn f_80214e50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214e50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214e50, pc); }
        match pc {
            0x80214e50 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80214e6c; f_801f025c(c);
                pc = 0x80214e6c;
            }
            0x80214e6c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[7] = c.r[31] | c.r[31];
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80214e8c; f_801f4c14(c);
                pc = 0x80214e8c;
            }
            0x80214e8c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x118;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80214ea4; f_801254b4(c);
                pc = 0x80214ea4;
            }
            0x80214ea4 => {
                c.r[3] = 0x0;
                c.lr = 0x80214eac; f_801f221c(c);
                pc = 0x80214eac;
            }
            0x80214eac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80214ec8; continue; }
                pc = 0x80214eb8;
            }
            0x80214eb8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214efc; continue;
                pc = 0x80214ec8;
            }
            0x80214ec8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x37;
                c.lr = 0x80214ed4; f_802025b8(c);
                pc = 0x80214ed4;
            }
            0x80214ed4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80214eb8; continue; }
                pc = 0x80214ee0;
            }
            0x80214ee0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x37;
                c.r[5] = 0x0;
                c.lr = 0x80214ef0; f_8020248c(c);
                pc = 0x80214ef0;
            }
            0x80214ef0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214efc;
            }
            0x80214efc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80214e50: bad pc {:#x}", pc),
        }
    }
}

/// fn_80214F10 (0x80214f10, 248 bytes)
#[inline(never)]
pub fn f_80214f10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80214f10) { return; }
        if c.prof { crate::prof::enter(c, 0x80214f10); f_80214f10_b(c, 0x80214f10); crate::prof::exit(c, 0x80214f10); return; }
    }
    f_80214f10_b(c, 0x80214f10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80214f10_at(c: &mut Cpu, pc: u32) { f_80214f10_b(c, pc) }
#[inline(always)]
fn f_80214f10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80214f10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80214f10, pc); }
        match pc {
            0x80214f10 => {
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x44); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80214f34; f_801f54a4(c);
                pc = 0x80214f34;
            }
            0x80214f34 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80214f44; f_801f025c(c);
                pc = 0x80214f44;
            }
            0x80214f44 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80214f58; f_801f4354(c);
                pc = 0x80214f58;
            }
            0x80214f58 => {
                c.r[6] = c.r[30] | c.r[30];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x18;
                c.lr = 0x80214f68; f_80215008(c);
                pc = 0x80214f68;
            }
            0x80214f68 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x80214fe8; continue; }
                pc = 0x80214f70;
            }
            0x80214f70 => {
                c.lr = 0x80214f74; f_800e0c54(c);
                pc = 0x80214f74;
            }
            0x80214f74 => {
                c.r[10] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[6] = divw(c.r[10], c.r[31]);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffbff;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[29] | c.r[29];
                c.r[0] = c.r[6].wrapping_mul(c.r[31]);
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.r[8] = 0x1;
                c.r[9] = 0xffffffff;
                c.r[0] = c.r[10].wrapping_sub(c.r[0]);
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[4] = c.lhz(c.r[4].wrapping_add(c.r[0]));
                c.sth(c.r[13].wrapping_add(0xffffadec), c.r[4]);
                c.lr = 0x80214fbc; f_8022b2cc(c);
                pc = 0x80214fbc;
            }
            0x80214fbc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80214fd8; f_801f4c14(c);
                pc = 0x80214fd8;
            }
            0x80214fd8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214ff4; continue;
                pc = 0x80214fe8;
            }
            0x80214fe8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80214ff4;
            }
            0x80214ff4 => {
                { let mut ea = c.r[1].wrapping_add(0x44); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { return; }
                return;
            }
            _ => panic!("f_80214f10: bad pc {:#x}", pc),
        }
    }
}

/// fn_80215008 (0x80215008, 440 bytes)
#[inline(never)]
pub fn f_80215008(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215008) { return; }
        if c.prof { crate::prof::enter(c, 0x80215008); f_80215008_b(c, 0x80215008); crate::prof::exit(c, 0x80215008); return; }
    }
    f_80215008_b(c, 0x80215008)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215008_at(c: &mut Cpu, pc: u32) { f_80215008_b(c, pc) }
#[inline(always)]
fn f_80215008_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215008;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215008, pc); }
        match pc {
            0x80215008 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[4] | c.r[4];
                c.r[23] = c.r[5] | c.r[5];
                c.r[22] = c.r[6] | c.r[6];
                c.r[25] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x80215038; f_801fb1c0(c);
                pc = 0x80215038;
            }
            0x80215038 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[22] | c.r[22];
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xd5;
                c.r[6] = 0x0;
                c.lr = 0x80215054; f_8012640c(c);
                pc = 0x80215054;
            }
            0x80215054 => {
                c.r[30] = c.r[23].rotate_left(0) & 0xffff;
                c.rc(c.r[30]);
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.r[4] | c.r[4];
                c.mtspr(9, c.r[30]);
                if !c.crbit(1) { pc = 0x80215078; continue; }
                pc = 0x8021506c;
            }
            0x8021506c => {
                c.sth(c.r[26].wrapping_add(c.r[4]), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x2);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8021506c; continue; }
                pc = 0x80215078;
            }
            0x80215078 => {
                c.r[27] = 0x0;
                c.r[24] = 0x0;
                c.r[29] = 0x0;
                pc = 0x80215084;
            }
            0x80215084 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[5] = c.r[29].rotate_left(0) & 0xffff;
                c.r[4] = 0x3;
                c.lr = 0x80215094; f_8012a5b0(c);
                pc = 0x80215094;
            }
            0x80215094 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x802150a4; f_801f9930(c);
                pc = 0x802150a4;
            }
            0x802150a4 => {
                c.r[23] = c.r[3] | c.r[3];
                c.rc(c.r[23]);
                if c.crbit(2) { pc = 0x8021519c; continue; }
                pc = 0x802150ac;
            }
            0x802150ac => {
                c.lr = 0x802150b0; f_80206608(c);
                pc = 0x802150b0;
            }
            0x802150b0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8021519c; continue; }
                pc = 0x802150b8;
            }
            0x802150b8 => {
                c.cmpu(0, c.r[31], c.r[23]);
                if c.crbit(2) { pc = 0x8021519c; continue; }
                pc = 0x802150c0;
            }
            0x802150c0 => {
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x802150c8; f_80205be8(c);
                pc = 0x802150c8;
            }
            0x802150c8 => {
                c.r[22] = 0x0;
                c.r[23] = c.r[3] | c.r[3];
                pc = 0x802150d0;
            }
            0x802150d0 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[22].rotate_left(0) & 0xffff;
                c.lr = 0x802150dc; f_80123cd4(c);
                pc = 0x802150dc;
            }
            0x802150dc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80215190; continue; }
                pc = 0x802150e4;
            }
            0x802150e4 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[6] = c.r[22].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x802150f8; f_8012640c(c);
                pc = 0x802150f8;
            }
            0x802150f8 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x80215128; continue; }
                pc = 0x80215100;
            }
            0x80215100 => {
                c.cmpu(0, c.r[5], 0x165);
                if c.crbit(2) { pc = 0x80215128; continue; }
                pc = 0x80215108;
            }
            0x80215108 => {
                c.cmpu(0, c.r[5], 0xd6);
                if c.crbit(2) { pc = 0x80215128; continue; }
                pc = 0x80215110;
            }
            0x80215110 => {
                c.cmpu(0, c.r[5], 0x112);
                if c.crbit(2) { pc = 0x80215128; continue; }
                pc = 0x80215118;
            }
            0x80215118 => {
                c.cmpu(0, c.r[5], 0x77);
                if c.crbit(2) { pc = 0x80215128; continue; }
                pc = 0x80215120;
            }
            0x80215120 => {
                c.cmpu(0, c.r[5], 0x76);
                if !c.crbit(2) { pc = 0x80215130; continue; }
                pc = 0x80215128;
            }
            0x80215128 => {
                c.r[0] = 0x1;
                pc = 0x80215134; continue;
                pc = 0x80215130;
            }
            0x80215130 => {
                c.r[0] = 0x0;
                pc = 0x80215134;
            }
            0x80215134 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80215190; continue; }
                pc = 0x8021513c;
            }
            0x8021513c => {
                c.r[4] = 0x80280000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffff9fa0);
                pc = 0x80215158; continue;
                pc = 0x8021514c;
            }
            0x8021514c => {
                c.cmpu(0, c.r[5], c.r[0]);
                if c.crbit(2) { pc = 0x80215164; continue; }
                pc = 0x80215154;
            }
            0x80215154 => {
                c.r[3] = c.r[3].wrapping_add(0x2);
                pc = 0x80215158;
            }
            0x80215158 => {
                c.r[0] = c.lhz(c.r[4].wrapping_add(c.r[3]));
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x8021514c; continue; }
                pc = 0x80215164;
            }
            0x80215164 => {
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x80215190; continue; }
                pc = 0x8021516c;
            }
            0x8021516c => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x80215190; continue; }
                pc = 0x80215174;
            }
            0x80215174 => {
                c.cmpu(0, c.r[5], 0x165);
                if c.crbit(2) { pc = 0x80215190; continue; }
                pc = 0x8021517c;
            }
            0x8021517c => {
                c.cmps(0, c.r[27], c.r[30]);
                if !c.crbit(0) { pc = 0x80215190; continue; }
                pc = 0x80215184;
            }
            0x80215184 => {
                c.sth(c.r[26].wrapping_add(c.r[24]), c.r[5]);
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.r[24] = c.r[24].wrapping_add(0x2);
                pc = 0x80215190;
            }
            0x80215190 => {
                c.r[22] = c.r[22].wrapping_add(0x1);
                c.cmps(0, c.r[22], 0x4);
                if c.crbit(0) { pc = 0x802150d0; continue; }
                pc = 0x8021519c;
            }
            0x8021519c => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.cmps(0, c.r[29], 0x6);
                if c.crbit(0) { pc = 0x80215084; continue; }
                pc = 0x802151a8;
            }
            0x802151a8 => {
                c.r[3] = c.r[27] | c.r[27];
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 22..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80215008: bad pc {:#x}", pc),
        }
    }
}

/// fn_802151C0 (0x802151c0, 232 bytes)
#[inline(never)]
pub fn f_802151c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802151c0) { return; }
        if c.prof { crate::prof::enter(c, 0x802151c0); f_802151c0_b(c, 0x802151c0); crate::prof::exit(c, 0x802151c0); return; }
    }
    f_802151c0_b(c, 0x802151c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802151c0_at(c: &mut Cpu, pc: u32) { f_802151c0_b(c, pc) }
#[inline(always)]
fn f_802151c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802151c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802151c0, pc); }
        match pc {
            0x802151c0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x802151dc; f_801f025c(c);
                pc = 0x802151dc;
            }
            0x802151dc => {
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x802151ec; f_8012640c(c);
                pc = 0x802151ec;
            }
            0x802151ec => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80215200; f_801f025c(c);
                pc = 0x80215200;
            }
            0x80215200 => {
                c.lr = 0x80215204; f_80203d3c(c);
                pc = 0x80215204;
            }
            0x80215204 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x5f;
                c.r[6] = 0x0;
                c.lr = 0x8021521c; f_8012640c(c);
                pc = 0x8021521c;
            }
            0x8021521c => {
                c.r[4] = 0x80280000;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[4].wrapping_add(0xffff9f88);
                c.r[8] = 0x0;
                pc = 0x8021523c; continue;
                pc = 0x80215230;
            }
            0x80215230 => {
                c.cmpu(0, c.r[0], c.r[5]);
                if c.crbit(1) { pc = 0x8021524c; continue; }
                pc = 0x80215238;
            }
            0x80215238 => {
                c.r[8] = c.r[8].wrapping_add(0x2);
                pc = 0x8021523c;
            }
            0x8021523c => {
                c.r[0] = c.r[8].rotate_left(1) & 0x1fffe;
                c.r[0] = c.lhz(c.r[3].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x80215230; continue; }
                pc = 0x8021524c;
            }
            0x8021524c => {
                c.cmpu(0, c.r[0], 0xffff);
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                if c.crbit(2) { pc = 0x8021527c; continue; }
                pc = 0x80215264;
            }
            0x80215264 => {
                c.r[7] = 0x80280000;
                c.r[0] = c.r[8].rotate_left(1) & 0x1fffe;
                c.r[7] = c.r[7].wrapping_add(0xffff9f88);
                c.r[7] = c.r[7].wrapping_add(c.r[0]);
                c.r[0] = c.lhz(c.r[7].wrapping_add(0x2));
                pc = 0x80215280; continue;
                pc = 0x8021527c;
            }
            0x8021527c => {
                c.r[0] = 0x78;
                pc = 0x80215280;
            }
            0x80215280 => {
                c.r[7] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x80215288; f_8011bbd8(c);
                pc = 0x80215288;
            }
            0x80215288 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802151c0: bad pc {:#x}", pc),
        }
    }
}

/// fightSeqGetKetaguriIryoku (0x802152a8, 88 bytes)
#[inline(never)]
pub fn f_802152a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802152a8) { return; }
        if c.prof { crate::prof::enter(c, 0x802152a8); f_802152a8_b(c, 0x802152a8); crate::prof::exit(c, 0x802152a8); return; }
    }
    f_802152a8_b(c, 0x802152a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802152a8_at(c: &mut Cpu, pc: u32) { f_802152a8_b(c, pc) }
#[inline(always)]
fn f_802152a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802152a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802152a8, pc); }
        match pc {
            0x802152a8 => {
                c.r[4] = 0x80280000;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[4].wrapping_add(0xffff9f88);
                c.r[4] = 0x0;
                pc = 0x802152c8; continue;
                pc = 0x802152bc;
            }
            0x802152bc => {
                c.cmpu(0, c.r[0], c.r[5]);
                if c.crbit(1) { pc = 0x802152d8; continue; }
                pc = 0x802152c4;
            }
            0x802152c4 => {
                c.r[4] = c.r[4].wrapping_add(0x2);
                pc = 0x802152c8;
            }
            0x802152c8 => {
                c.r[0] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[0] = c.lhz(c.r[3].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[0], 0xffff);
                if !c.crbit(2) { pc = 0x802152bc; continue; }
                pc = 0x802152d8;
            }
            0x802152d8 => {
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x802152f8; continue; }
                pc = 0x802152e0;
            }
            0x802152e0 => {
                c.r[3] = 0x80280000;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[3] = c.r[3].wrapping_add(0xffff9f88);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2));
                if true { return; }
                pc = 0x802152f8;
            }
            0x802152f8 => {
                c.r[3] = 0x78;
                if true { return; }
                return;
            }
            _ => panic!("f_802152a8: bad pc {:#x}", pc),
        }
    }
}

/// WS_ONNEN (0x80215300, 116 bytes)
#[inline(never)]
pub fn f_80215300(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215300) { return; }
        if c.prof { crate::prof::enter(c, 0x80215300); f_80215300_b(c, 0x80215300); crate::prof::exit(c, 0x80215300); return; }
    }
    f_80215300_b(c, 0x80215300)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215300_at(c: &mut Cpu, pc: u32) { f_80215300_b(c, pc) }
#[inline(always)]
fn f_80215300_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215300;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215300, pc); }
        match pc {
            0x80215300 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x8021531c; f_801f025c(c);
                pc = 0x8021531c;
            }
            0x8021531c => {
                c.r[4] = 0x28;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80215328; f_802025b8(c);
                pc = 0x80215328;
            }
            0x80215328 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80215354; continue; }
                pc = 0x80215334;
            }
            0x80215334 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x28;
                c.r[5] = 0x0;
                c.lr = 0x80215344; f_8020248c(c);
                pc = 0x80215344;
            }
            0x80215344 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215360; continue;
                pc = 0x80215354;
            }
            0x80215354 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215360;
            }
            0x80215360 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80215300: bad pc {:#x}", pc),
        }
    }
}

/// fn_80215374 (0x80215374, 436 bytes)
#[inline(never)]
pub fn f_80215374(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215374) { return; }
        if c.prof { crate::prof::enter(c, 0x80215374); f_80215374_b(c, 0x80215374); crate::prof::exit(c, 0x80215374); return; }
    }
    f_80215374_b(c, 0x80215374)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215374_at(c: &mut Cpu, pc: u32) { f_80215374_b(c, pc) }
#[inline(always)]
fn f_80215374_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215374;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215374, pc); }
        match pc {
            0x80215374 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80215390; f_801f025c(c);
                pc = 0x80215390;
            }
            0x80215390 => {
                c.r[4] = 0x27;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8021539c; f_802025b8(c);
                pc = 0x8021539c;
            }
            0x8021539c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x802153b8; continue; }
                pc = 0x802153a8;
            }
            0x802153a8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215514; continue;
                pc = 0x802153b8;
            }
            0x802153b8 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x802154b8; continue; }
                pc = 0x802153c0;
            }
            0x802153c0 => {
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[4] = 0x2e;
                c.r[5] = 0x2;
                c.lr = 0x802153d4; f_801f3624(c);
                pc = 0x802153d4;
            }
            0x802153d4 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x802154b8; continue; }
                pc = 0x802153dc;
            }
            0x802153dc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802153e4; f_80205b8c(c);
                pc = 0x802153e4;
            }
            0x802153e4 => {
                c.r[4] = 0x11e;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x802153f0; f_80123b5c(c);
                pc = 0x802153f0;
            }
            0x802153f0 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x802154b8; continue; }
                pc = 0x802153fc;
            }
            0x802153fc => {
                c.r[27] = c.r[30] as u8 as i8 as i32 as u32;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x80;
                c.lr = 0x80215414; f_8012640c(c);
                pc = 0x80215414;
            }
            0x80215414 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[5] = c.addc(!c.r[0], c.r[29], 1);
                c.r[6] = c.r[27] | c.r[27];
                c.r[0] = c.r[0].wrapping_sub(c.r[29]);
                c.r[4] = 0x0;
                { let ci = c.ca(); c.r[7] = c.addc(!c.r[5], c.r[5], ci); }
                c.r[5] = 0x80;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[0] & c.r[7];
                c.r[7] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x80215444; f_801254b4(c);
                pc = 0x80215444;
            }
            0x80215444 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[28] = c.r[30].rotate_left(0) & 0xff;
                c.r[4] = 0x10;
                c.lr = 0x80215454; f_802026e4(c);
                pc = 0x80215454;
            }
            0x80215454 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802154b8; continue; }
                pc = 0x8021545c;
            }
            0x8021545c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x31;
                c.lr = 0x80215468; f_802026e4(c);
                pc = 0x80215468;
            }
            0x80215468 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802154b8; continue; }
                pc = 0x80215474;
            }
            0x80215474 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x31;
                c.lr = 0x80215480; f_80201890(c);
                pc = 0x80215480;
            }
            0x80215480 => {
                c.r[0] = 0x1;
                c.r[0] = slw(c.r[0], c.r[28]);
                c.r[0] = c.r[3] & c.r[0];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802154b8; continue; }
                pc = 0x80215490;
            }
            0x80215490 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80215498; f_801fecd4(c);
                pc = 0x80215498;
            }
            0x80215498 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802154b8; continue; }
                pc = 0x802154a4;
            }
            0x802154a4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[4] = 0x80;
                c.r[6] = 0x0;
                c.lr = 0x802154b8; f_801fe7ec(c);
                pc = 0x802154b8;
            }
            0x802154b8 => {
                c.r[0] = 0x0;
                c.r[3] = 0x80210000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[31]);
                c.r[4] = c.r[3].wrapping_add(0x5528);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.r[6] = 0x0;
                c.lr = 0x802154dc; f_801f37b0(c);
                pc = 0x802154dc;
            }
            0x802154dc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x802154f8; continue; }
                pc = 0x802154e8;
            }
            0x802154e8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215514; continue;
                pc = 0x802154f8;
            }
            0x802154f8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x27;
                c.r[5] = 0x0;
                c.lr = 0x80215508; f_8020248c(c);
                pc = 0x80215508;
            }
            0x80215508 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215514;
            }
            0x80215514 => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80215374: bad pc {:#x}", pc),
        }
    }
}

/// fn_80215528 (0x80215528, 236 bytes)
#[inline(never)]
pub fn f_80215528(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215528) { return; }
        if c.prof { crate::prof::enter(c, 0x80215528); f_80215528_b(c, 0x80215528); crate::prof::exit(c, 0x80215528); return; }
    }
    f_80215528_b(c, 0x80215528)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215528_at(c: &mut Cpu, pc: u32) { f_80215528_b(c, pc) }
#[inline(always)]
fn f_80215528_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215528;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215528, pc); }
        match pc {
            0x80215528 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[5] | c.r[5];
                c.r[31] = c.r[3] | c.r[3];
                c.r[30] = c.lwz(c.r[5]);
                c.lr = 0x80215548; f_802062fc(c);
                pc = 0x80215548;
            }
            0x80215548 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80215558; continue; }
                pc = 0x80215550;
            }
            0x80215550 => {
                c.r[3] = 0x1;
                pc = 0x80215600; continue;
                pc = 0x80215558;
            }
            0x80215558 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80215564; f_80202b88(c);
                pc = 0x80215564;
            }
            0x80215564 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80215578; continue; }
                pc = 0x80215570;
            }
            0x80215570 => {
                c.r[3] = 0x1;
                pc = 0x80215600; continue;
                pc = 0x80215578;
            }
            0x80215578 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80215580; f_80205b8c(c);
                pc = 0x80215580;
            }
            0x80215580 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x80215590; f_80205b8c(c);
                pc = 0x80215590;
            }
            0x80215590 => {
                c.r[28] = 0x0;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x802155f0; continue;
                pc = 0x8021559c;
            }
            0x8021559c => {
                c.r[31] = c.r[28] as u8 as i8 as i32 as u32;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802155ac; f_80123cd4(c);
                pc = 0x802155ac;
            }
            0x802155ac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x802155ec; continue; }
                pc = 0x802155b4;
            }
            0x802155b4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x802155c8; f_8012640c(c);
                pc = 0x802155c8;
            }
            0x802155c8 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x802155d4; f_80123b5c(c);
                pc = 0x802155d4;
            }
            0x802155d4 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x802155ec; continue; }
                pc = 0x802155dc;
            }
            0x802155dc => {
                c.r[0] = 0x1;
                c.r[3] = 0x0;
                c.stw(c.r[27].wrapping_add(0x4), c.r[0]);
                pc = 0x80215600; continue;
                pc = 0x802155ec;
            }
            0x802155ec => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x802155f0;
            }
            0x802155f0 => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8021559c; continue; }
                pc = 0x802155fc;
            }
            0x802155fc => {
                c.r[3] = 0x1;
                pc = 0x80215600;
            }
            0x80215600 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80215528: bad pc {:#x}", pc),
        }
    }
}

/// fn_80215614 (0x80215614, 268 bytes)
#[inline(never)]
pub fn f_80215614(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215614) { return; }
        if c.prof { crate::prof::enter(c, 0x80215614); f_80215614_b(c, 0x80215614); crate::prof::exit(c, 0x80215614); return; }
    }
    f_80215614_b(c, 0x80215614)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215614_at(c: &mut Cpu, pc: u32) { f_80215614_b(c, pc) }
#[inline(always)]
fn f_80215614_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215614;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215614, pc); }
        match pc {
            0x80215614 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80215630; f_801f025c(c);
                pc = 0x80215630;
            }
            0x80215630 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80215644; f_8012640c(c);
                pc = 0x80215644;
            }
            0x80215644 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x80215654; f_80207bf4(c);
                pc = 0x80215654;
            }
            0x80215654 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[28] = c.r[0] | c.r[0];
                c.lr = 0x80215664; f_80205184(c);
                pc = 0x80215664;
            }
            0x80215664 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[26] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80215678; f_801f025c(c);
                pc = 0x80215678;
            }
            0x80215678 => {
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x80215680; f_80207bf4(c);
                pc = 0x80215680;
            }
            0x80215680 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[27] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x80215694; continue; }
                pc = 0x8021568c;
            }
            0x8021568c => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x802156d8; continue; }
                pc = 0x80215694;
            }
            0x80215694 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x19);
                if c.crbit(2) { pc = 0x802156d8; continue; }
                pc = 0x802156a0;
            }
            0x802156a0 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x19);
                if c.crbit(2) { pc = 0x802156d8; continue; }
                pc = 0x802156ac;
            }
            0x802156ac => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802156b4; f_802096e8(c);
                pc = 0x802156b4;
            }
            0x802156b4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x802156d8; continue; }
                pc = 0x802156bc;
            }
            0x802156bc => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x802156cc; f_80229934(c);
                pc = 0x802156cc;
            }
            0x802156cc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802156e8; continue; }
                pc = 0x802156d8;
            }
            0x802156d8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021570c; continue;
                pc = 0x802156e8;
            }
            0x802156e8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x802156f4; f_80207bc0(c);
                pc = 0x802156f4;
            }
            0x802156f4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80215700; f_80207bc0(c);
                pc = 0x80215700;
            }
            0x80215700 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021570c;
            }
            0x8021570c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80215614: bad pc {:#x}", pc),
        }
    }
}

/// fn_80215720 (0x80215720, 232 bytes)
#[inline(never)]
pub fn f_80215720(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215720) { return; }
        if c.prof { crate::prof::enter(c, 0x80215720); f_80215720_b(c, 0x80215720); crate::prof::exit(c, 0x80215720); return; }
    }
    f_80215720_b(c, 0x80215720)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215720_at(c: &mut Cpu, pc: u32) { f_80215720_b(c, pc) }
#[inline(always)]
fn f_80215720_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215720;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215720, pc); }
        match pc {
            0x80215720 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[0] = c.r[0].rotate_left(0) & 0x200;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802157e8; continue; }
                pc = 0x8021573c;
            }
            0x8021573c => {
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80215748; f_801f025c(c);
                pc = 0x80215748;
            }
            0x80215748 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8021575c; f_8012640c(c);
                pc = 0x8021575c;
            }
            0x8021575c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x8021576c; f_80205184(c);
                pc = 0x8021576c;
            }
            0x8021576c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80215780; f_8011beb4(c);
                pc = 0x80215780;
            }
            0x80215780 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8021578c; f_80205b8c(c);
                pc = 0x8021578c;
            }
            0x8021578c => {
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x8021579c; f_8012640c(c);
                pc = 0x8021579c;
            }
            0x8021579c => {
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802157a8; f_80205b8c(c);
                pc = 0x802157a8;
            }
            0x802157a8 => {
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x802157b8; f_8012640c(c);
                pc = 0x802157b8;
            }
            0x802157b8 => {
                c.r[0] = c.r[29].wrapping_mul(c.r[28]);
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = divw(c.r[0], c.r[3]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802157d0; continue; }
                pc = 0x802157cc;
            }
            0x802157cc => {
                c.r[0] = 0x1;
                pc = 0x802157d0;
            }
            0x802157d0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[7] = c.r[0].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x802157e8; f_8011bbd8(c);
                pc = 0x802157e8;
            }
            0x802157e8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80215720: bad pc {:#x}", pc),
        }
    }
}

/// fn_80215808 (0x80215808, 200 bytes)
#[inline(never)]
pub fn f_80215808(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215808) { return; }
        if c.prof { crate::prof::enter(c, 0x80215808); f_80215808_b(c, 0x80215808); crate::prof::exit(c, 0x80215808); return; }
    }
    f_80215808_b(c, 0x80215808)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215808_at(c: &mut Cpu, pc: u32) { f_80215808_b(c, pc) }
#[inline(always)]
fn f_80215808_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215808;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215808, pc); }
        match pc {
            0x80215808 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80215824; f_801f025c(c);
                pc = 0x80215824;
            }
            0x80215824 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80215838; f_8012640c(c);
                pc = 0x80215838;
            }
            0x80215838 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x80215848; f_80205b8c(c);
                pc = 0x80215848;
            }
            0x80215848 => {
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x80215858; f_8012640c(c);
                pc = 0x80215858;
            }
            0x80215858 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.lr = 0x80215868; f_801f025c(c);
                pc = 0x80215868;
            }
            0x80215868 => {
                c.lr = 0x8021586c; f_80205b8c(c);
                pc = 0x8021586c;
            }
            0x8021586c => {
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x8021587c; f_8012640c(c);
                pc = 0x8021587c;
            }
            0x8021587c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[0], c.r[31]);
                if c.crbit(1) { pc = 0x80215898; continue; }
                pc = 0x80215888;
            }
            0x80215888 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802158bc; continue;
                pc = 0x80215898;
            }
            0x80215898 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[7] = c.r[0].wrapping_sub(c.r[31]);
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x802158b0; f_8011bbd8(c);
                pc = 0x802158b0;
            }
            0x802158b0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802158bc;
            }
            0x802158bc => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80215808: bad pc {:#x}", pc),
        }
    }
}

/// WS_AKUBI (0x802158d0, 132 bytes)
#[inline(never)]
pub fn f_802158d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802158d0) { return; }
        if c.prof { crate::prof::enter(c, 0x802158d0); f_802158d0_b(c, 0x802158d0); crate::prof::exit(c, 0x802158d0); return; }
    }
    f_802158d0_b(c, 0x802158d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802158d0_at(c: &mut Cpu, pc: u32) { f_802158d0_b(c, pc) }
#[inline(always)]
fn f_802158d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802158d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802158d0, pc); }
        match pc {
            0x802158d0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x802158ec; f_801f025c(c);
                pc = 0x802158ec;
            }
            0x802158ec => {
                c.r[4] = 0x26;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x802158f8; f_802025b8(c);
                pc = 0x802158f8;
            }
            0x802158f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80215914; continue; }
                pc = 0x80215904;
            }
            0x80215904 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8021590c; f_80203ccc(c);
                pc = 0x8021590c;
            }
            0x8021590c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80215924; continue; }
                pc = 0x80215914;
            }
            0x80215914 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215940; continue;
                pc = 0x80215924;
            }
            0x80215924 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x26;
                c.r[5] = 0x0;
                c.lr = 0x80215934; f_8020248c(c);
                pc = 0x80215934;
            }
            0x80215934 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215940;
            }
            0x80215940 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802158d0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80215954 (0x80215954, 292 bytes)
#[inline(never)]
pub fn f_80215954(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215954) { return; }
        if c.prof { crate::prof::enter(c, 0x80215954); f_80215954_b(c, 0x80215954); crate::prof::exit(c, 0x80215954); return; }
    }
    f_80215954_b(c, 0x80215954)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215954_at(c: &mut Cpu, pc: u32) { f_80215954_b(c, pc) }
#[inline(always)]
fn f_80215954_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215954;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215954, pc); }
        match pc {
            0x80215954 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80215978; f_801f54a4(c);
                pc = 0x80215978;
            }
            0x80215978 => {
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80215988; f_801f025c(c);
                pc = 0x80215988;
            }
            0x80215988 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8021599c; f_8012640c(c);
                pc = 0x8021599c;
            }
            0x8021599c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x102;
                c.r[6] = 0x0;
                c.lr = 0x802159b8; f_8012640c(c);
                pc = 0x802159b8;
            }
            0x802159b8 => {
                c.r[28] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x103;
                c.r[6] = 0x0;
                c.lr = 0x802159d0; f_8012640c(c);
                pc = 0x802159d0;
            }
            0x802159d0 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x104;
                c.r[6] = 0x0;
                c.lr = 0x802159e8; f_8012640c(c);
                pc = 0x802159e8;
            }
            0x802159e8 => {
                c.r[27] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x105;
                c.r[6] = 0x0;
                c.lr = 0x80215a00; f_8012640c(c);
                pc = 0x80215a00;
            }
            0x80215a00 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.lr = 0x80215a10; f_801f025c(c);
                pc = 0x80215a10;
            }
            0x80215a10 => {
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x80215a18; f_801f0134(c);
                pc = 0x80215a18;
            }
            0x80215a18 => {
                c.r[0] = c.r[28] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80215a2c; continue; }
                pc = 0x80215a20;
            }
            0x80215a20 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(2) { pc = 0x80215a40; continue; }
                pc = 0x80215a2c;
            }
            0x80215a2c => {
                c.r[0] = c.r[27] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80215a58; continue; }
                pc = 0x80215a34;
            }
            0x80215a34 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[31], c.r[0]);
                if !c.crbit(2) { pc = 0x80215a58; continue; }
                pc = 0x80215a40;
            }
            0x80215a40 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x2;
                c.lr = 0x80215a58; f_8011bbd8(c);
                pc = 0x80215a58;
            }
            0x80215a58 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80215954: bad pc {:#x}", pc),
        }
    }
}

/// WS_NEWOHARU (0x80215a78, 116 bytes)
#[inline(never)]
pub fn f_80215a78(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215a78) { return; }
        if c.prof { crate::prof::enter(c, 0x80215a78); f_80215a78_b(c, 0x80215a78); crate::prof::exit(c, 0x80215a78); return; }
    }
    f_80215a78_b(c, 0x80215a78)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215a78_at(c: &mut Cpu, pc: u32) { f_80215a78_b(c, pc) }
#[inline(always)]
fn f_80215a78_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215a78;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215a78, pc); }
        match pc {
            0x80215a78 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80215a94; f_801f025c(c);
                pc = 0x80215a94;
            }
            0x80215a94 => {
                c.r[4] = 0x25;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80215aa0; f_802025b8(c);
                pc = 0x80215aa0;
            }
            0x80215aa0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x80215abc; continue; }
                pc = 0x80215aac;
            }
            0x80215aac => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215ad8; continue;
                pc = 0x80215abc;
            }
            0x80215abc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x25;
                c.r[5] = 0x0;
                c.lr = 0x80215acc; f_8020248c(c);
                pc = 0x80215acc;
            }
            0x80215acc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215ad8;
            }
            0x80215ad8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80215a78: bad pc {:#x}", pc),
        }
    }
}

/// fn_80215AEC (0x80215aec, 388 bytes)
#[inline(never)]
pub fn f_80215aec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215aec) { return; }
        if c.prof { crate::prof::enter(c, 0x80215aec); f_80215aec_b(c, 0x80215aec); crate::prof::exit(c, 0x80215aec); return; }
    }
    f_80215aec_b(c, 0x80215aec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215aec_at(c: &mut Cpu, pc: u32) { f_80215aec_b(c, pc) }
#[inline(always)]
fn f_80215aec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215aec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215aec, pc); }
        match pc {
            0x80215aec => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80215bbc; continue; }
                pc = 0x80215b0c;
            }
            0x80215b0c => {
                if !c.crbit(0) { pc = 0x80215c5c; continue; }
                pc = 0x80215b10;
            }
            0x80215b10 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x80215b1c; continue; }
                pc = 0x80215b18;
            }
            0x80215b18 => {
                pc = 0x80215c5c; continue;
                pc = 0x80215b1c;
            }
            0x80215b1c => {
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80215b28; f_801f025c(c);
                pc = 0x80215b28;
            }
            0x80215b28 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd5;
                c.r[6] = 0x0;
                c.lr = 0x80215b3c; f_8012640c(c);
                pc = 0x80215b3c;
            }
            0x80215b3c => {
                c.r[4] = 0x0;
                c.r[5] = 0xcb;
                c.r[6] = 0x0;
                c.lr = 0x80215b4c; f_8012640c(c);
                pc = 0x80215b4c;
            }
            0x80215b4c => {
                c.r[4] = 0x0;
                c.r[5] = 0x77;
                c.r[6] = 0x0;
                c.lr = 0x80215b5c; f_8012640c(c);
                pc = 0x80215b5c;
            }
            0x80215b5c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x35;
                c.lr = 0x80215b70; f_802025b8(c);
                pc = 0x80215b70;
            }
            0x80215b70 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x80215b8c; continue; }
                pc = 0x80215b7c;
            }
            0x80215b7c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x2));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215c5c; continue;
                pc = 0x80215b8c;
            }
            0x80215b8c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x35;
                c.r[5] = 0x0;
                c.lr = 0x80215b9c; f_8020248c(c);
                pc = 0x80215b9c;
            }
            0x80215b9c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = 0x35;
                c.lr = 0x80215bac; f_80201764(c);
                pc = 0x80215bac;
            }
            0x80215bac => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x6);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215c5c; continue;
                pc = 0x80215bbc;
            }
            0x80215bbc => {
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80215bc8; f_801f025c(c);
                pc = 0x80215bc8;
            }
            0x80215bc8 => {
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80215bd8; f_8012640c(c);
                pc = 0x80215bd8;
            }
            0x80215bd8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80215bec; f_801f025c(c);
                pc = 0x80215bec;
            }
            0x80215bec => {
                c.r[4] = 0x35;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80215bf8; f_80201890(c);
                pc = 0x80215bf8;
            }
            0x80215bf8 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80215c04; f_80132a38(c);
                pc = 0x80215c04;
            }
            0x80215c04 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2;
                c.lr = 0x80215c10; f_80203b5c(c);
                pc = 0x80215c10;
            }
            0x80215c10 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[7] = c.r[0].wrapping_neg();
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x80215c2c; f_8011bbd8(c);
                pc = 0x80215c2c;
            }
            0x80215c2c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80215c34; f_80201704(c);
                pc = 0x80215c34;
            }
            0x80215c34 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80215c50; continue; }
                pc = 0x80215c40;
            }
            0x80215c40 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x2));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215c5c; continue;
                pc = 0x80215c50;
            }
            0x80215c50 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x6);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215c5c;
            }
            0x80215c5c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80215aec: bad pc {:#x}", pc),
        }
    }
}

/// WS_NARIKIRI (0x80215c70, 128 bytes)
#[inline(never)]
pub fn f_80215c70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215c70) { return; }
        if c.prof { crate::prof::enter(c, 0x80215c70); f_80215c70_b(c, 0x80215c70); crate::prof::exit(c, 0x80215c70); return; }
    }
    f_80215c70_b(c, 0x80215c70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215c70_at(c: &mut Cpu, pc: u32) { f_80215c70_b(c, pc) }
#[inline(always)]
fn f_80215c70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215c70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215c70, pc); }
        match pc {
            0x80215c70 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80215c8c; f_801f025c(c);
                pc = 0x80215c8c;
            }
            0x80215c8c => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80215c94; f_80207bf4(c);
                pc = 0x80215c94;
            }
            0x80215c94 => {
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.lr = 0x80215ca0; f_801f025c(c);
                pc = 0x80215ca0;
            }
            0x80215ca0 => {
                c.lr = 0x80215ca4; f_80207bf4(c);
                pc = 0x80215ca4;
            }
            0x80215ca4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[4] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x80215cd0; continue; }
                pc = 0x80215cb0;
            }
            0x80215cb0 => {
                c.cmpu(0, c.r[0], 0x19);
                if c.crbit(2) { pc = 0x80215cd0; continue; }
                pc = 0x80215cb8;
            }
            0x80215cb8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80215cc0; f_80207bc0(c);
                pc = 0x80215cc0;
            }
            0x80215cc0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215cdc; continue;
                pc = 0x80215cd0;
            }
            0x80215cd0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80215cdc;
            }
            0x80215cdc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80215c70: bad pc {:#x}", pc),
        }
    }
}

/// fn_80215CF0 (0x80215cf0, 856 bytes)
#[inline(never)]
pub fn f_80215cf0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80215cf0) { return; }
        if c.prof { crate::prof::enter(c, 0x80215cf0); f_80215cf0_b(c, 0x80215cf0); crate::prof::exit(c, 0x80215cf0); return; }
    }
    f_80215cf0_b(c, 0x80215cf0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80215cf0_at(c: &mut Cpu, pc: u32) { f_80215cf0_b(c, pc) }
#[inline(always)]
fn f_80215cf0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80215cf0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80215cf0, pc); }
        match pc {
            0x80215cf0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80215d0c; f_801f025c(c);
                pc = 0x80215d0c;
            }
            0x80215d0c => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x80215d14; f_802040e8(c);
                pc = 0x80215d14;
            }
            0x80215d14 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80215d28; f_801f025c(c);
                pc = 0x80215d28;
            }
            0x80215d28 => {
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x80215d30; f_80207bf4(c);
                pc = 0x80215d30;
            }
            0x80215d30 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80215d3c; f_802040e8(c);
                pc = 0x80215d3c;
            }
            0x80215d3c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[27] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80215d58; f_801f54a4(c);
                pc = 0x80215d58;
            }
            0x80215d58 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80215d7c; continue; }
                pc = 0x80215d64;
            }
            0x80215d64 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80215d6c; f_80204a10(c);
                pc = 0x80215d6c;
            }
            0x80215d6c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80215d7c; continue; }
                pc = 0x80215d74;
            }
            0x80215d74 => {
                c.r[0] = 0x0;
                pc = 0x80215dc0; continue;
                pc = 0x80215d7c;
            }
            0x80215d7c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80215d90; f_801f54a4(c);
                pc = 0x80215d90;
            }
            0x80215d90 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80215dbc; continue; }
                pc = 0x80215d9c;
            }
            0x80215d9c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x3d;
                c.lr = 0x80215da8; f_802026e4(c);
                pc = 0x80215da8;
            }
            0x80215da8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80215dbc; continue; }
                pc = 0x80215db4;
            }
            0x80215db4 => {
                c.r[0] = 0x0;
                pc = 0x80215dc0; continue;
                pc = 0x80215dbc;
            }
            0x80215dbc => {
                c.r[0] = 0x1;
                pc = 0x80215dc0;
            }
            0x80215dc0 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80215dd8; continue; }
                pc = 0x80215dc8;
            }
            0x80215dc8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216034; continue;
                pc = 0x80215dd8;
            }
            0x80215dd8 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80215dec; f_801f54a4(c);
                pc = 0x80215dec;
            }
            0x80215dec => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80215e24; continue; }
                pc = 0x80215df4;
            }
            0x80215df4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x3d;
                c.lr = 0x80215e00; f_802026e4(c);
                pc = 0x80215e00;
            }
            0x80215e00 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80215e1c; continue; }
                pc = 0x80215e08;
            }
            0x80215e08 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x3d;
                c.lr = 0x80215e14; f_802026e4(c);
                pc = 0x80215e14;
            }
            0x80215e14 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80215e24; continue; }
                pc = 0x80215e1c;
            }
            0x80215e1c => {
                c.r[0] = 0x0;
                pc = 0x80215e28; continue;
                pc = 0x80215e24;
            }
            0x80215e24 => {
                c.r[0] = 0x1;
                pc = 0x80215e28;
            }
            0x80215e28 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80215e40; continue; }
                pc = 0x80215e30;
            }
            0x80215e30 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216034; continue;
                pc = 0x80215e40;
            }
            0x80215e40 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80215e50; continue; }
                pc = 0x80215e48;
            }
            0x80215e48 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80215e98; continue; }
                pc = 0x80215e50;
            }
            0x80215e50 => {
                c.r[3] = c.r[28].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], 0xaf);
                if c.crbit(2) { pc = 0x80215e98; continue; }
                pc = 0x80215e5c;
            }
            0x80215e5c => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xaf);
                if c.crbit(2) { pc = 0x80215e98; continue; }
                pc = 0x80215e68;
            }
            0x80215e68 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80215e80; continue; }
                pc = 0x80215e70;
            }
            0x80215e70 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80215e78; f_80142984(c);
                pc = 0x80215e78;
            }
            0x80215e78 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80215e98; continue; }
                pc = 0x80215e80;
            }
            0x80215e80 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80215ea8; continue; }
                pc = 0x80215e88;
            }
            0x80215e88 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x80215e90; f_80142984(c);
                pc = 0x80215e90;
            }
            0x80215e90 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80215ea8; continue; }
                pc = 0x80215e98;
            }
            0x80215e98 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216034; continue;
                pc = 0x80215ea8;
            }
            0x80215ea8 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x3c);
                if !c.crbit(2) { pc = 0x80215ec4; continue; }
                pc = 0x80215eb4;
            }
            0x80215eb4 => {
                c.r[3] = 0x80380000;
                c.r[0] = c.r[3].wrapping_add(0xffff98d8);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216034; continue;
                pc = 0x80215ec4;
            }
            0x80215ec4 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80215ee0; continue; }
                pc = 0x80215ecc;
            }
            0x80215ecc => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80215ee0; f_8020147c(c);
                pc = 0x80215ee0;
            }
            0x80215ee0 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80215efc; continue; }
                pc = 0x80215ee8;
            }
            0x80215ee8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80215efc; f_8020147c(c);
                pc = 0x80215efc;
            }
            0x80215efc => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80215f18; continue; }
                pc = 0x80215f04;
            }
            0x80215f04 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80215f18; f_8020147c(c);
                pc = 0x80215f18;
            }
            0x80215f18 => {
                c.r[7] = c.r[27].rotate_left(0) & 0xffff;
                c.rc(c.r[7]);
                if c.crbit(2) { pc = 0x80215f34; continue; }
                pc = 0x80215f20;
            }
            0x80215f20 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xfb;
                c.r[6] = 0x0;
                c.lr = 0x80215f34; f_801254b4(c);
                pc = 0x80215f34;
            }
            0x80215f34 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80215f3c; f_801fecd4(c);
                pc = 0x80215f3c;
            }
            0x80215f3c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80215f5c; continue; }
                pc = 0x80215f48;
            }
            0x80215f48 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x82;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80215f5c; f_801fe7ec(c);
                pc = 0x80215f5c;
            }
            0x80215f5c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80215f64; f_801fecd4(c);
                pc = 0x80215f64;
            }
            0x80215f64 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80215f84; continue; }
                pc = 0x80215f70;
            }
            0x80215f70 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x82;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80215f84; f_801fe7ec(c);
                pc = 0x80215f84;
            }
            0x80215f84 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x36;
                c.lr = 0x80215f90; f_80202810(c);
                pc = 0x80215f90;
            }
            0x80215f90 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x36;
                c.lr = 0x80215f9c; f_80202810(c);
                pc = 0x80215f9c;
            }
            0x80215f9c => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80215fb0; f_80142cf4(c);
                pc = 0x80215fb0;
            }
            0x80215fb0 => {
                c.lr = 0x80215fb4; f_800fa280(c);
                pc = 0x80215fb4;
            }
            0x80215fb4 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80215fc0; f_80132a38(c);
                pc = 0x80215fc0;
            }
            0x80215fc0 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80215fd4; f_80142cf4(c);
                pc = 0x80215fd4;
            }
            0x80215fd4 => {
                c.lr = 0x80215fd8; f_800fa280(c);
                pc = 0x80215fd8;
            }
            0x80215fd8 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xe;
                c.lr = 0x80215fe4; f_80132a38(c);
                pc = 0x80215fe4;
            }
            0x80215fe4 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80216004; continue; }
                pc = 0x80215fec;
            }
            0x80215fec => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80216004; continue; }
                pc = 0x80215ff4;
            }
            0x80215ff4 => {
                c.r[0] = 0x2;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x80216028; continue;
                pc = 0x80216004;
            }
            0x80216004 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8021601c; continue; }
                pc = 0x8021600c;
            }
            0x8021600c => {
                c.r[0] = 0x0;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x80216028; continue;
                pc = 0x8021601c;
            }
            0x8021601c => {
                c.r[0] = 0x1;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x80216028;
            }
            0x80216028 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216034;
            }
            0x80216034 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80215cf0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80216048 (0x80216048, 164 bytes)
#[inline(never)]
pub fn f_80216048(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216048) { return; }
        if c.prof { crate::prof::enter(c, 0x80216048); f_80216048_b(c, 0x80216048); crate::prof::exit(c, 0x80216048); return; }
    }
    f_80216048_b(c, 0x80216048)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216048_at(c: &mut Cpu, pc: u32) { f_80216048_b(c, pc) }
#[inline(always)]
fn f_80216048_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216048;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216048, pc); }
        match pc {
            0x80216048 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80216070; f_801f54a4(c);
                pc = 0x80216070;
            }
            0x80216070 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80216094; continue; }
                pc = 0x8021607c;
            }
            0x8021607c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80216084; f_80204a10(c);
                pc = 0x80216084;
            }
            0x80216084 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80216094; continue; }
                pc = 0x8021608c;
            }
            0x8021608c => {
                c.r[3] = 0x0;
                pc = 0x802160d8; continue;
                pc = 0x80216094;
            }
            0x80216094 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x802160a8; f_801f54a4(c);
                pc = 0x802160a8;
            }
            0x802160a8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802160d4; continue; }
                pc = 0x802160b4;
            }
            0x802160b4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3d;
                c.lr = 0x802160c0; f_802026e4(c);
                pc = 0x802160c0;
            }
            0x802160c0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802160d4; continue; }
                pc = 0x802160cc;
            }
            0x802160cc => {
                c.r[3] = 0x0;
                pc = 0x802160d8; continue;
                pc = 0x802160d4;
            }
            0x802160d4 => {
                c.r[3] = 0x1;
                pc = 0x802160d8;
            }
            0x802160d8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80216048: bad pc {:#x}", pc),
        }
    }
}

/// fn_802160EC (0x802160ec, 260 bytes)
#[inline(never)]
pub fn f_802160ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802160ec) { return; }
        if c.prof { crate::prof::enter(c, 0x802160ec); f_802160ec_b(c, 0x802160ec); crate::prof::exit(c, 0x802160ec); return; }
    }
    f_802160ec_b(c, 0x802160ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802160ec_at(c: &mut Cpu, pc: u32) { f_802160ec_b(c, pc) }
#[inline(always)]
fn f_802160ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802160ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802160ec, pc); }
        match pc {
            0x802160ec => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80216108; f_801f025c(c);
                pc = 0x80216108;
            }
            0x80216108 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0xe;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8021611c; f_801f025c(c);
                pc = 0x8021611c;
            }
            0x8021611c => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x802161d0; continue; }
                pc = 0x80216124;
            }
            0x80216124 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x19;
                c.r[6] = 0x0;
                c.lr = 0x80216138; f_801f54a4(c);
                pc = 0x80216138;
            }
            0x80216138 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x802161d0; continue; }
                pc = 0x80216144;
            }
            0x80216144 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8021614c; f_802062fc(c);
                pc = 0x8021614c;
            }
            0x8021614c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802161d0; continue; }
                pc = 0x80216158;
            }
            0x80216158 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x32;
                c.lr = 0x80216164; f_802026e4(c);
                pc = 0x80216164;
            }
            0x80216164 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802161d0; continue; }
                pc = 0x8021616c;
            }
            0x8021616c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x32;
                c.lr = 0x80216178; f_802026e4(c);
                pc = 0x80216178;
            }
            0x80216178 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802161d0; continue; }
                pc = 0x80216180;
            }
            0x80216180 => {
                c.r[7] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80216198; f_801f4c14(c);
                pc = 0x80216198;
            }
            0x80216198 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x32;
                c.lr = 0x802161a4; f_802025b8(c);
                pc = 0x802161a4;
            }
            0x802161a4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x802161c0; continue; }
                pc = 0x802161b0;
            }
            0x802161b0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x32;
                c.r[5] = 0x0;
                c.lr = 0x802161c0; f_8020248c(c);
                pc = 0x802161c0;
            }
            0x802161c0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802161dc; continue;
                pc = 0x802161d0;
            }
            0x802161d0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802161dc;
            }
            0x802161dc => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802160ec: bad pc {:#x}", pc),
        }
    }
}

/// WS_CHOUHATSU (0x802161f0, 116 bytes)
#[inline(never)]
pub fn f_802161f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802161f0) { return; }
        if c.prof { crate::prof::enter(c, 0x802161f0); f_802161f0_b(c, 0x802161f0); crate::prof::exit(c, 0x802161f0); return; }
    }
    f_802161f0_b(c, 0x802161f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802161f0_at(c: &mut Cpu, pc: u32) { f_802161f0_b(c, pc) }
#[inline(always)]
fn f_802161f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802161f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802161f0, pc); }
        match pc {
            0x802161f0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x8021620c; f_801f025c(c);
                pc = 0x8021620c;
            }
            0x8021620c => {
                c.r[4] = 0x30;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80216218; f_802025b8(c);
                pc = 0x80216218;
            }
            0x80216218 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x80216234; continue; }
                pc = 0x80216224;
            }
            0x80216224 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216250; continue;
                pc = 0x80216234;
            }
            0x80216234 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x30;
                c.r[5] = 0x0;
                c.lr = 0x80216244; f_8020248c(c);
                pc = 0x80216244;
            }
            0x80216244 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216250;
            }
            0x80216250 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802161f0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80216264 (0x80216264, 140 bytes)
#[inline(never)]
pub fn f_80216264(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216264) { return; }
        if c.prof { crate::prof::enter(c, 0x80216264); f_80216264_b(c, 0x80216264); crate::prof::exit(c, 0x80216264); return; }
    }
    f_80216264_b(c, 0x80216264)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216264_at(c: &mut Cpu, pc: u32) { f_80216264_b(c, pc) }
#[inline(always)]
fn f_80216264_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216264;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216264, pc); }
        match pc {
            0x80216264 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80216280; f_801f025c(c);
                pc = 0x80216280;
            }
            0x80216280 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0x102;
                c.r[6] = 0x0;
                c.lr = 0x80216294; f_8012640c(c);
                pc = 0x80216294;
            }
            0x80216294 => {
                c.r[30] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x104;
                c.r[6] = 0x0;
                c.lr = 0x802162ac; f_8012640c(c);
                pc = 0x802162ac;
            }
            0x802162ac => {
                c.r[0] = c.r[30] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                if !c.crbit(2) { pc = 0x802162c0; continue; }
                pc = 0x802162b8;
            }
            0x802162b8 => {
                c.r[0] = c.r[0] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x802162d0; continue; }
                pc = 0x802162c0;
            }
            0x802162c0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802162dc; continue;
                pc = 0x802162d0;
            }
            0x802162d0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802162dc;
            }
            0x802162dc => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80216264: bad pc {:#x}", pc),
        }
    }
}

/// WS_ICHAMON (0x802162f0, 116 bytes)
#[inline(never)]
pub fn f_802162f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802162f0) { return; }
        if c.prof { crate::prof::enter(c, 0x802162f0); f_802162f0_b(c, 0x802162f0); crate::prof::exit(c, 0x802162f0); return; }
    }
    f_802162f0_b(c, 0x802162f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802162f0_at(c: &mut Cpu, pc: u32) { f_802162f0_b(c, pc) }
#[inline(always)]
fn f_802162f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802162f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802162f0, pc); }
        match pc {
            0x802162f0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x8021630c; f_801f025c(c);
                pc = 0x8021630c;
            }
            0x8021630c => {
                c.r[4] = 0x1b;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80216318; f_802025b8(c);
                pc = 0x80216318;
            }
            0x80216318 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x80216334; continue; }
                pc = 0x80216324;
            }
            0x80216324 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216350; continue;
                pc = 0x80216334;
            }
            0x80216334 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1b;
                c.r[5] = 0x0;
                c.lr = 0x80216344; f_8020248c(c);
                pc = 0x80216344;
            }
            0x80216344 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216350;
            }
            0x80216350 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802162f0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80216364 (0x80216364, 172 bytes)
#[inline(never)]
pub fn f_80216364(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216364) { return; }
        if c.prof { crate::prof::enter(c, 0x80216364); f_80216364_b(c, 0x80216364); crate::prof::exit(c, 0x80216364); return; }
    }
    f_80216364_b(c, 0x80216364)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216364_at(c: &mut Cpu, pc: u32) { f_80216364_b(c, pc) }
#[inline(always)]
fn f_80216364_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216364;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216364, pc); }
        match pc {
            0x80216364 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80216380; f_801f025c(c);
                pc = 0x80216380;
            }
            0x80216380 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80216388; f_80205b8c(c);
                pc = 0x80216388;
            }
            0x80216388 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x80216398; f_80203c5c(c);
                pc = 0x80216398;
            }
            0x80216398 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802163f0; continue; }
                pc = 0x802163a4;
            }
            0x802163a4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802163ac; f_801252e0(c);
                pc = 0x802163ac;
            }
            0x802163ac => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x802163b8; f_80202998(c);
                pc = 0x802163b8;
            }
            0x802163b8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.r[4].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.lr = 0x802163cc; f_801fecd4(c);
                pc = 0x802163cc;
            }
            0x802163cc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802163fc; continue; }
                pc = 0x802163d8;
            }
            0x802163d8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7c;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802163ec; f_801fe7ec(c);
                pc = 0x802163ec;
            }
            0x802163ec => {
                pc = 0x802163fc; continue;
                pc = 0x802163f0;
            }
            0x802163f0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802163fc;
            }
            0x802163fc => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80216364: bad pc {:#x}", pc),
        }
    }
}

/// fn_80216410 (0x80216410, 320 bytes)
#[inline(never)]
pub fn f_80216410(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216410) { return; }
        if c.prof { crate::prof::enter(c, 0x80216410); f_80216410_b(c, 0x80216410); crate::prof::exit(c, 0x80216410); return; }
    }
    f_80216410_b(c, 0x80216410)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216410_at(c: &mut Cpu, pc: u32) { f_80216410_b(c, pc) }
#[inline(always)]
fn f_80216410_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216410;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216410, pc); }
        match pc {
            0x80216410 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80216434; f_801f54a4(c);
                pc = 0x80216434;
            }
            0x80216434 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80216444; f_801f025c(c);
                pc = 0x80216444;
            }
            0x80216444 => {
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80216458; f_8012640c(c);
                pc = 0x80216458;
            }
            0x80216458 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xf;
                c.r[6] = 0x0;
                c.lr = 0x80216474; f_801f54a4(c);
                pc = 0x80216474;
            }
            0x80216474 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8021647c; f_801363e8(c);
                pc = 0x8021647c;
            }
            0x8021647c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[28] = c.r[0] | c.r[0];
                c.r[5] = 0x9;
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = 0x0;
                c.lr = 0x80216498; f_8011beb4(c);
                pc = 0x80216498;
            }
            0x80216498 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x802164a8; f_80209c1c(c);
                pc = 0x802164a8;
            }
            0x802164a8 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x802164bc; f_8011beb4(c);
                pc = 0x802164bc;
            }
            0x802164bc => {
                c.lr = 0x802164c0; f_800fa280(c);
                pc = 0x802164c0;
            }
            0x802164c0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x28;
                c.lr = 0x802164cc; f_80132a38(c);
                pc = 0x802164cc;
            }
            0x802164cc => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[31] | c.r[31];
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.r[8] = 0x1;
                c.r[9] = 0xffffffff;
                c.lr = 0x802164ec; f_8022b2cc(c);
                pc = 0x802164ec;
            }
            0x802164ec => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80216508; f_801f4c14(c);
                pc = 0x80216508;
            }
            0x80216508 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[3] = 0x80370000;
                c.r[4] = c.r[3].wrapping_add(0x7ea4);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae0c));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffbff;
                c.r[5] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                c.lr = 0x80216528; f_80211b94(c);
                pc = 0x80216528;
            }
            0x80216528 => {
                c.r[3] = 0x80380000;
                c.r[0] = c.r[27].rotate_left(2) & 0x3fffc;
                c.r[3] = c.r[3].wrapping_add(0xffff9bff);
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80216410: bad pc {:#x}", pc),
        }
    }
}

/// WS_JUUDEN (0x80216550, 100 bytes)
#[inline(never)]
pub fn f_80216550(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216550) { return; }
        if c.prof { crate::prof::enter(c, 0x80216550); f_80216550_b(c, 0x80216550); crate::prof::exit(c, 0x80216550); return; }
    }
    f_80216550_b(c, 0x80216550)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216550_at(c: &mut Cpu, pc: u32) { f_80216550_b(c, pc) }
#[inline(always)]
fn f_80216550_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216550;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216550, pc); }
        match pc {
            0x80216550 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x8021656c; f_801f025c(c);
                pc = 0x8021656c;
            }
            0x8021656c => {
                c.r[4] = 0x24;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80216578; f_802025b8(c);
                pc = 0x80216578;
            }
            0x80216578 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80216594; continue; }
                pc = 0x80216584;
            }
            0x80216584 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x24;
                c.r[5] = 0x0;
                c.lr = 0x80216594; f_8020248c(c);
                pc = 0x80216594;
            }
            0x80216594 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80216550: bad pc {:#x}", pc),
        }
    }
}

/// fn_802165B4 (0x802165b4, 156 bytes)
#[inline(never)]
pub fn f_802165b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802165b4) { return; }
        if c.prof { crate::prof::enter(c, 0x802165b4); f_802165b4_b(c, 0x802165b4); crate::prof::exit(c, 0x802165b4); return; }
    }
    f_802165b4_b(c, 0x802165b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802165b4_at(c: &mut Cpu, pc: u32) { f_802165b4_b(c, pc) }
#[inline(always)]
fn f_802165b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802165b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802165b4, pc); }
        match pc {
            0x802165b4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802165d0; f_801f025c(c);
                pc = 0x802165d0;
            }
            0x802165d0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802165e4; f_801f025c(c);
                pc = 0x802165e4;
            }
            0x802165e4 => {
                c.r[4] = 0x4d;
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x802165f0; f_801f6e44(c);
                pc = 0x802165f0;
            }
            0x802165f0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80216630; continue; }
                pc = 0x802165fc;
            }
            0x802165fc => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x80216610; f_801f54a4(c);
                pc = 0x80216610;
            }
            0x80216610 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8021661c; f_801f0134(c);
                pc = 0x8021661c;
            }
            0x8021661c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[0] | c.r[0];
                c.r[4] = 0x4d;
                c.lr = 0x80216630; f_801f6df0(c);
                pc = 0x80216630;
            }
            0x80216630 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802165b4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80216650 (0x80216650, 304 bytes)
#[inline(never)]
pub fn f_80216650(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216650) { return; }
        if c.prof { crate::prof::enter(c, 0x80216650); f_80216650_b(c, 0x80216650); crate::prof::exit(c, 0x80216650); return; }
    }
    f_80216650_b(c, 0x80216650)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216650_at(c: &mut Cpu, pc: u32) { f_80216650_b(c, pc) }
#[inline(always)]
fn f_80216650_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216650;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216650, pc); }
        match pc {
            0x80216650 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80216674; f_801f54a4(c);
                pc = 0x80216674;
            }
            0x80216674 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80216684; f_801f025c(c);
                pc = 0x80216684;
            }
            0x80216684 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80216698; f_8012640c(c);
                pc = 0x80216698;
            }
            0x80216698 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.lr = 0x802166b0; f_802656ac(c);
                pc = 0x802166b0;
            }
            0x802166b0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[27] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x802166c4; f_801f025c(c);
                pc = 0x802166c4;
            }
            0x802166c4 => {
                c.r[4] = 0x0;
                c.r[29] = c.r[3] | c.r[3];
                c.r[5] = 0xe6;
                c.r[6] = 0x0;
                c.lr = 0x802166d8; f_8012640c(c);
                pc = 0x802166d8;
            }
            0x802166d8 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xe8;
                c.r[6] = 0x0;
                c.lr = 0x802166f0; f_8012640c(c);
                pc = 0x802166f0;
            }
            0x802166f0 => {
                c.cmpu(0, c.r[28], 0x0);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                if !c.crbit(2) { pc = 0x80216724; continue; }
                pc = 0x802166fc;
            }
            0x802166fc => {
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80216724; continue; }
                pc = 0x80216704;
            }
            0x80216704 => {
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x6));
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80216724; continue; }
                pc = 0x80216714;
            }
            0x80216714 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021676c; continue;
                pc = 0x80216724;
            }
            0x80216724 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8021672c; f_80205b8c(c);
                pc = 0x8021672c;
            }
            0x8021672c => {
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x8021673c; f_8012640c(c);
                pc = 0x8021673c;
            }
            0x8021673c => {
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x80216754; f_8011bbd8(c);
                pc = 0x80216754;
            }
            0x80216754 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.lr = 0x80216760; f_80011e68(c);
                pc = 0x80216760;
            }
            0x80216760 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021676c;
            }
            0x8021676c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80216650: bad pc {:#x}", pc),
        }
    }
}

/// fn_80216780 (0x80216780, 132 bytes)
#[inline(never)]
pub fn f_80216780(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216780) { return; }
        if c.prof { crate::prof::enter(c, 0x80216780); f_80216780_b(c, 0x80216780); crate::prof::exit(c, 0x80216780); return; }
    }
    f_80216780_b(c, 0x80216780)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216780_at(c: &mut Cpu, pc: u32) { f_80216780_b(c, pc) }
#[inline(always)]
fn f_80216780_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216780;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216780, pc); }
        match pc {
            0x80216780 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x52;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80216798; f_801f2988(c);
                pc = 0x80216798;
            }
            0x80216798 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x802167cc; continue; }
                pc = 0x802167a4;
            }
            0x802167a4 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x3b;
                c.r[6] = 0x0;
                c.r[7] = 0x40;
                c.lr = 0x802167bc; f_801f4c14(c);
                pc = 0x802167bc;
            }
            0x802167bc => {
                c.r[0] = 0x2;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x802167e8; continue;
                pc = 0x802167cc;
            }
            0x802167cc => {
                c.r[3] = 0x0;
                c.r[4] = 0x52;
                c.r[5] = 0x0;
                c.lr = 0x802167dc; f_801f2934(c);
                pc = 0x802167dc;
            }
            0x802167dc => {
                c.r[0] = 0x5;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x802167e8;
            }
            0x802167e8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80216780: bad pc {:#x}", pc),
        }
    }
}

/// WS_CHIISAKUNARU (0x80216804, 112 bytes)
#[inline(never)]
pub fn f_80216804(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216804) { return; }
        if c.prof { crate::prof::enter(c, 0x80216804); f_80216804_b(c, 0x80216804); crate::prof::exit(c, 0x80216804); return; }
    }
    f_80216804_b(c, 0x80216804)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216804_at(c: &mut Cpu, pc: u32) { f_80216804_b(c, pc) }
#[inline(always)]
fn f_80216804_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216804;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216804, pc); }
        match pc {
            0x80216804 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80216820; f_801f025c(c);
                pc = 0x80216820;
            }
            0x80216820 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.r[0].rotate_left(0) & 0x2000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80216854; continue; }
                pc = 0x80216830;
            }
            0x80216830 => {
                c.r[4] = 0x23;
                c.lr = 0x80216838; f_802025b8(c);
                pc = 0x80216838;
            }
            0x80216838 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80216854; continue; }
                pc = 0x80216844;
            }
            0x80216844 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x23;
                c.r[5] = 0x0;
                c.lr = 0x80216854; f_8020248c(c);
                pc = 0x80216854;
            }
            0x80216854 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80216804: bad pc {:#x}", pc),
        }
    }
}

/// WS_KIERUTAME_AFTAR (0x80216874, 236 bytes)
#[inline(never)]
pub fn f_80216874(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216874) { return; }
        if c.prof { crate::prof::enter(c, 0x80216874); f_80216874_b(c, 0x80216874); crate::prof::exit(c, 0x80216874); return; }
    }
    f_80216874_b(c, 0x80216874)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216874_at(c: &mut Cpu, pc: u32) { f_80216874_b(c, pc) }
#[inline(always)]
fn f_80216874_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216874;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216874, pc); }
        match pc {
            0x80216874 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80216890; f_801f025c(c);
                pc = 0x80216890;
            }
            0x80216890 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80216898; f_80205184(c);
                pc = 0x80216898;
            }
            0x80216898 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[0], 0x123);
                if c.crbit(2) { pc = 0x8021691c; continue; }
                pc = 0x802168a4;
            }
            0x802168a4 => {
                if !c.crbit(0) { pc = 0x802168c0; continue; }
                pc = 0x802168a8;
            }
            0x802168a8 => {
                c.cmps(0, c.r[0], 0x5b);
                if c.crbit(2) { pc = 0x802168f4; continue; }
                pc = 0x802168b0;
            }
            0x802168b0 => {
                if !c.crbit(0) { pc = 0x80216940; continue; }
                pc = 0x802168b4;
            }
            0x802168b4 => {
                c.cmps(0, c.r[0], 0x13);
                if c.crbit(2) { pc = 0x802168cc; continue; }
                pc = 0x802168bc;
            }
            0x802168bc => {
                pc = 0x80216940; continue;
                pc = 0x802168c0;
            }
            0x802168c0 => {
                c.cmps(0, c.r[0], 0x154);
                if c.crbit(2) { pc = 0x802168cc; continue; }
                pc = 0x802168c8;
            }
            0x802168c8 => {
                pc = 0x80216940; continue;
                pc = 0x802168cc;
            }
            0x802168cc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1f;
                c.lr = 0x802168d8; f_802026e4(c);
                pc = 0x802168d8;
            }
            0x802168d8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80216940; continue; }
                pc = 0x802168e4;
            }
            0x802168e4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1f;
                c.lr = 0x802168f0; f_80202810(c);
                pc = 0x802168f0;
            }
            0x802168f0 => {
                pc = 0x80216940; continue;
                pc = 0x802168f4;
            }
            0x802168f4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x20;
                c.lr = 0x80216900; f_802026e4(c);
                pc = 0x80216900;
            }
            0x80216900 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80216940; continue; }
                pc = 0x8021690c;
            }
            0x8021690c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x20;
                c.lr = 0x80216918; f_80202810(c);
                pc = 0x80216918;
            }
            0x80216918 => {
                pc = 0x80216940; continue;
                pc = 0x8021691c;
            }
            0x8021691c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x21;
                c.lr = 0x80216928; f_802026e4(c);
                pc = 0x80216928;
            }
            0x80216928 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80216940; continue; }
                pc = 0x80216934;
            }
            0x80216934 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x21;
                c.lr = 0x80216940; f_80202810(c);
                pc = 0x80216940;
            }
            0x80216940 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80216874: bad pc {:#x}", pc),
        }
    }
}

/// WS_KIERUTAME (0x80216960, 248 bytes)
#[inline(never)]
pub fn f_80216960(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216960) { return; }
        if c.prof { crate::prof::enter(c, 0x80216960); f_80216960_b(c, 0x80216960); crate::prof::exit(c, 0x80216960); return; }
    }
    f_80216960_b(c, 0x80216960)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216960_at(c: &mut Cpu, pc: u32) { f_80216960_b(c, pc) }
#[inline(always)]
fn f_80216960_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216960;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216960, pc); }
        match pc {
            0x80216960 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x8021697c; f_801f025c(c);
                pc = 0x8021697c;
            }
            0x8021697c => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80216984; f_80205184(c);
                pc = 0x80216984;
            }
            0x80216984 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[0], 0x123);
                if c.crbit(2) { pc = 0x80216a10; continue; }
                pc = 0x80216990;
            }
            0x80216990 => {
                if !c.crbit(0) { pc = 0x802169ac; continue; }
                pc = 0x80216994;
            }
            0x80216994 => {
                c.cmps(0, c.r[0], 0x5b);
                if c.crbit(2) { pc = 0x802169e4; continue; }
                pc = 0x8021699c;
            }
            0x8021699c => {
                if !c.crbit(0) { pc = 0x80216a38; continue; }
                pc = 0x802169a0;
            }
            0x802169a0 => {
                c.cmps(0, c.r[0], 0x13);
                if c.crbit(2) { pc = 0x802169b8; continue; }
                pc = 0x802169a8;
            }
            0x802169a8 => {
                pc = 0x80216a38; continue;
                pc = 0x802169ac;
            }
            0x802169ac => {
                c.cmps(0, c.r[0], 0x154);
                if c.crbit(2) { pc = 0x802169b8; continue; }
                pc = 0x802169b4;
            }
            0x802169b4 => {
                pc = 0x80216a38; continue;
                pc = 0x802169b8;
            }
            0x802169b8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1f;
                c.lr = 0x802169c4; f_802025b8(c);
                pc = 0x802169c4;
            }
            0x802169c4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80216a38; continue; }
                pc = 0x802169d0;
            }
            0x802169d0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1f;
                c.r[5] = 0x0;
                c.lr = 0x802169e0; f_8020248c(c);
                pc = 0x802169e0;
            }
            0x802169e0 => {
                pc = 0x80216a38; continue;
                pc = 0x802169e4;
            }
            0x802169e4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x20;
                c.lr = 0x802169f0; f_802025b8(c);
                pc = 0x802169f0;
            }
            0x802169f0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80216a38; continue; }
                pc = 0x802169fc;
            }
            0x802169fc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x20;
                c.r[5] = 0x0;
                c.lr = 0x80216a0c; f_8020248c(c);
                pc = 0x80216a0c;
            }
            0x80216a0c => {
                pc = 0x80216a38; continue;
                pc = 0x80216a10;
            }
            0x80216a10 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x21;
                c.lr = 0x80216a1c; f_802025b8(c);
                pc = 0x80216a1c;
            }
            0x80216a1c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80216a38; continue; }
                pc = 0x80216a28;
            }
            0x80216a28 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x21;
                c.r[5] = 0x0;
                c.lr = 0x80216a38; f_8020248c(c);
                pc = 0x80216a38;
            }
            0x80216a38 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80216960: bad pc {:#x}", pc),
        }
    }
}

/// fn_80216A58 (0x80216a58, 672 bytes)
#[inline(never)]
pub fn f_80216a58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216a58) { return; }
        if c.prof { crate::prof::enter(c, 0x80216a58); f_80216a58_b(c, 0x80216a58); crate::prof::exit(c, 0x80216a58); return; }
    }
    f_80216a58_b(c, 0x80216a58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216a58_at(c: &mut Cpu, pc: u32) { f_80216a58_b(c, pc) }
#[inline(always)]
fn f_80216a58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216a58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216a58, pc); }
        match pc {
            0x80216a58 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80216a74; f_801f025c(c);
                pc = 0x80216a74;
            }
            0x80216a74 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80216a88; f_8012640c(c);
                pc = 0x80216a88;
            }
            0x80216a88 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[25] = c.r[0] | c.r[0];
                c.lr = 0x80216a98; f_80205184(c);
                pc = 0x80216a98;
            }
            0x80216a98 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.lr = 0x80216aa8; f_801f4354(c);
                pc = 0x80216aa8;
            }
            0x80216aa8 => {
                c.r[4] = 0x0;
                c.r[29] = c.r[3] | c.r[3];
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x80216abc; f_801fb1c0(c);
                pc = 0x80216abc;
            }
            0x80216abc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80216ad0; f_801f025c(c);
                pc = 0x80216ad0;
            }
            0x80216ad0 => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x80216ad8; f_802062fc(c);
                pc = 0x80216ad8;
            }
            0x80216ad8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80216af0; continue; }
                pc = 0x80216ae0;
            }
            0x80216ae0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216ce4; continue;
                pc = 0x80216af0;
            }
            0x80216af0 => {
                c.r[26] = c.lbz(c.r[13].wrapping_add(0xffff8558));
                c.r[23] = 0x0;
                pc = 0x80216b6c; continue;
                pc = 0x80216afc;
            }
            0x80216afc => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x3;
                c.lr = 0x80216b08; f_8012a5b0(c);
                pc = 0x80216b08;
            }
            0x80216b08 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[23] = c.r[0] | c.r[0];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x80216b1c; f_801f9930(c);
                pc = 0x80216b1c;
            }
            0x80216b1c => {
                c.r[24] = c.r[3] | c.r[3];
                c.rc(c.r[24]);
                if c.crbit(2) { pc = 0x80216b60; continue; }
                pc = 0x80216b24;
            }
            0x80216b24 => {
                c.lr = 0x80216b28; f_80206a04(c);
                pc = 0x80216b28;
            }
            0x80216b28 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80216b60; continue; }
                pc = 0x80216b30;
            }
            0x80216b30 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x80216b38; f_80205be8(c);
                pc = 0x80216b38;
            }
            0x80216b38 => {
                c.r[23] = c.r[3] | c.r[3];
                c.lr = 0x80216b40; f_801233f4(c);
                pc = 0x80216b40;
            }
            0x80216b40 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80216b60; continue; }
                pc = 0x80216b4c;
            }
            0x80216b4c => {
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x80216b54; f_80122ddc(c);
                pc = 0x80216b54;
            }
            0x80216b54 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80216b78; continue; }
                pc = 0x80216b60;
            }
            0x80216b60 => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffff8558));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stb(c.r[13].wrapping_add(0xffff8558), c.r[0]);
                pc = 0x80216b6c;
            }
            0x80216b6c => {
                c.r[5] = c.lbz(c.r[13].wrapping_add(0xffff8558));
                c.cmpu(0, c.r[5], 0x6);
                if c.crbit(0) { pc = 0x80216afc; continue; }
                pc = 0x80216b78;
            }
            0x80216b78 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffff8558));
                c.cmpu(0, c.r[0], 0x6);
                if !c.crbit(0) { pc = 0x80216cc0; continue; }
                pc = 0x80216b84;
            }
            0x80216b84 => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80216b98; f_8012640c(c);
                pc = 0x80216b98;
            }
            0x80216b98 => {
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x7a;
                c.r[6] = 0x0;
                c.lr = 0x80216bb0; f_8012640c(c);
                pc = 0x80216bb0;
            }
            0x80216bb0 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80216bbc; f_80203d3c(c);
                pc = 0x80216bbc;
            }
            0x80216bbc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[28] = c.r[0] | c.r[0];
                c.lr = 0x80216bcc; f_80203e0c(c);
                pc = 0x80216bcc;
            }
            0x80216bcc => {
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x77;
                c.r[6] = 0x0;
                c.lr = 0x80216be0; f_8012640c(c);
                pc = 0x80216be0;
            }
            0x80216be0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80216bec; f_80132a38(c);
                pc = 0x80216bec;
            }
            0x80216bec => {
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x80216c00; f_8012640c(c);
                pc = 0x80216c00;
            }
            0x80216c00 => {
                c.r[26] = c.r[3] | c.r[3];
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x80216c18; f_8011beb4(c);
                pc = 0x80216c18;
            }
            0x80216c18 => {
                c.r[4] = c.r[29].rotate_left(1) & 0x1fe;
                c.r[0] = 0x5;
                c.r[7] = divw(c.r[4], c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.r[26] = c.r[26].wrapping_mul(c.r[0]);
                c.r[0] = c.r[7].wrapping_add(0x2);
                c.r[26] = c.r[26].wrapping_mul(c.r[0]);
                c.lr = 0x80216c48; f_8012640c(c);
                pc = 0x80216c48;
            }
            0x80216c48 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(1) { pc = 0x80216c54; continue; }
                pc = 0x80216c50;
            }
            0x80216c50 => {
                c.r[3] = 0x1;
                pc = 0x80216c54;
            }
            0x80216c54 => {
                c.r[26] = divw(c.r[26], c.r[3]);
                c.r[0] = 0x32;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x32;
                c.r[5] = divw(c.r[26], c.r[0]);
                c.r[26] = c.r[5].wrapping_add(0x2);
                c.r[23] = c.r[26] | c.r[26];
                c.lr = 0x80216c74; f_802026e4(c);
                pc = 0x80216c74;
            }
            0x80216c74 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80216c8c; continue; }
                pc = 0x80216c80;
            }
            0x80216c80 => {
                c.r[3] = c.r[26].wrapping_mul(0xf);
                c.r[0] = 0xa;
                c.r[23] = divw(c.r[3], c.r[0]);
                pc = 0x80216c8c;
            }
            0x80216c8c => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[7] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x80216ca4; f_8011bbd8(c);
                pc = 0x80216ca4;
            }
            0x80216ca4 => {
                c.r[4] = c.lbz(c.r[13].wrapping_add(0xffff8558));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[3].wrapping_add(0x9);
                c.stb(c.r[13].wrapping_add(0xffff8558), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216ce4; continue;
                pc = 0x80216cc0;
            }
            0x80216cc0 => {
                c.cmpu(0, c.r[26], 0x0);
                if c.crbit(2) { pc = 0x80216cd8; continue; }
                pc = 0x80216cc8;
            }
            0x80216cc8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216ce4; continue;
                pc = 0x80216cd8;
            }
            0x80216cd8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x5));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216ce4;
            }
            0x80216ce4 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80216a58: bad pc {:#x}", pc),
        }
    }
}

/// fn_80216CF8 (0x80216cf8, 164 bytes)
#[inline(never)]
pub fn f_80216cf8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216cf8) { return; }
        if c.prof { crate::prof::enter(c, 0x80216cf8); f_80216cf8_b(c, 0x80216cf8); crate::prof::exit(c, 0x80216cf8); return; }
    }
    f_80216cf8_b(c, 0x80216cf8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216cf8_at(c: &mut Cpu, pc: u32) { f_80216cf8_b(c, pc) }
#[inline(always)]
fn f_80216cf8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216cf8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216cf8, pc); }
        match pc {
            0x80216cf8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x80216d24; f_8012640c(c);
                pc = 0x80216d24;
            }
            0x80216d24 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x80216d3c; f_8011beb4(c);
                pc = 0x80216d3c;
            }
            0x80216d3c => {
                c.r[4] = c.r[29].rotate_left(1) & 0x1fffe;
                c.r[0] = 0x5;
                c.r[6] = divw(c.r[4], c.r[0]);
                c.r[5] = 0x5;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[31] = c.r[31].wrapping_mul(c.r[0]);
                c.r[0] = c.r[6].wrapping_add(0x2);
                c.r[6] = 0x0;
                c.r[31] = c.r[31].wrapping_mul(c.r[0]);
                c.lr = 0x80216d6c; f_8012640c(c);
                pc = 0x80216d6c;
            }
            0x80216d6c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(1) { pc = 0x80216d78; continue; }
                pc = 0x80216d74;
            }
            0x80216d74 => {
                c.r[3] = 0x1;
                pc = 0x80216d78;
            }
            0x80216d78 => {
                c.r[31] = divw(c.r[31], c.r[3]);
                c.r[0] = 0x32;
                c.r[3] = divw(c.r[31], c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x2);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80216cf8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80216D9C (0x80216d9c, 436 bytes)
#[inline(never)]
pub fn f_80216d9c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216d9c) { return; }
        if c.prof { crate::prof::enter(c, 0x80216d9c); f_80216d9c_b(c, 0x80216d9c); crate::prof::exit(c, 0x80216d9c); return; }
    }
    f_80216d9c_b(c, 0x80216d9c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216d9c_at(c: &mut Cpu, pc: u32) { f_80216d9c_b(c, pc) }
#[inline(always)]
fn f_80216d9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216d9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216d9c, pc); }
        match pc {
            0x80216d9c => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80216dc0; f_801f54a4(c);
                pc = 0x80216dc0;
            }
            0x80216dc0 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80216dd0; f_801f025c(c);
                pc = 0x80216dd0;
            }
            0x80216dd0 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80216ddc; f_801f0134(c);
                pc = 0x80216ddc;
            }
            0x80216ddc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80216df8; f_8012640c(c);
                pc = 0x80216df8;
            }
            0x80216df8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x80216e08; f_80205184(c);
                pc = 0x80216e08;
            }
            0x80216e08 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80216e24; f_8011beb4(c);
                pc = 0x80216e24;
            }
            0x80216e24 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.lr = 0x80216e3c; f_8011beb4(c);
                pc = 0x80216e3c;
            }
            0x80216e3c => {
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.lr = 0x80216e4c; f_801f025c(c);
                pc = 0x80216e4c;
            }
            0x80216e4c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80216e60; f_801f025c(c);
                pc = 0x80216e60;
            }
            0x80216e60 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[25] = c.r[0] | c.r[0];
                c.r[4] = 0x34;
                c.lr = 0x80216e74; f_802025b8(c);
                pc = 0x80216e74;
            }
            0x80216e74 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x80216e90; continue; }
                pc = 0x80216e80;
            }
            0x80216e80 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216f3c; continue;
                pc = 0x80216e90;
            }
            0x80216e90 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[29] | c.r[29];
                c.r[4] = 0x34;
                c.lr = 0x80216ea0; f_8020248c(c);
                pc = 0x80216ea0;
            }
            0x80216ea0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[4] = 0x34;
                c.lr = 0x80216eb0; f_80201b2c(c);
                pc = 0x80216eb0;
            }
            0x80216eb0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[25] | c.r[25];
                c.r[6] = c.r[28] | c.r[28];
                c.r[7] = c.r[27] | c.r[27];
                c.r[8] = c.r[26] | c.r[26];
                c.lr = 0x80216ecc; f_80232110(c);
                pc = 0x80216ecc;
            }
            0x80216ecc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[26] = c.r[0] | c.r[0];
                c.r[4] = 0x32;
                c.lr = 0x80216ee0; f_802026e4(c);
                pc = 0x80216ee0;
            }
            0x80216ee0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80216ef8; continue; }
                pc = 0x80216eec;
            }
            0x80216eec => {
                c.r[3] = c.r[26].wrapping_mul(0xf);
                c.r[0] = 0xa;
                c.r[26] = divw(c.r[3], c.r[0]);
                pc = 0x80216ef8;
            }
            0x80216ef8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[26] | c.r[26];
                c.r[4] = 0x34;
                c.lr = 0x80216f08; f_80201764(c);
                pc = 0x80216f08;
            }
            0x80216f08 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x161);
                if !c.crbit(2) { pc = 0x80216f24; continue; }
                pc = 0x80216f14;
            }
            0x80216f14 => {
                c.r[0] = 0x1;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x80216f30; continue;
                pc = 0x80216f24;
            }
            0x80216f24 => {
                c.r[0] = 0x0;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x80216f30;
            }
            0x80216f30 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80216f3c;
            }
            0x80216f3c => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80216d9c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80216F50 (0x80216f50, 200 bytes)
#[inline(never)]
pub fn f_80216f50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80216f50) { return; }
        if c.prof { crate::prof::enter(c, 0x80216f50); f_80216f50_b(c, 0x80216f50); crate::prof::exit(c, 0x80216f50); return; }
    }
    f_80216f50_b(c, 0x80216f50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80216f50_at(c: &mut Cpu, pc: u32) { f_80216f50_b(c, pc) }
#[inline(always)]
fn f_80216f50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80216f50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80216f50, pc); }
        match pc {
            0x80216f50 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80216f6c; f_801f025c(c);
                pc = 0x80216f6c;
            }
            0x80216f6c => {
                c.r[4] = 0x8;
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffae29), c.r[4]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[29] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffae28), c.r[0]);
                pc = 0x80216fc8; continue;
                pc = 0x80216f88;
            }
            0x80216f88 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x5d;
                c.lr = 0x80216f98; f_801f54a4(c);
                pc = 0x80216f98;
            }
            0x80216f98 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x80216fbc; continue; }
                pc = 0x80216fa0;
            }
            0x80216fa0 => {
                c.lr = 0x80216fa4; f_802062fc(c);
                pc = 0x80216fa4;
            }
            0x80216fa4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80216fbc; continue; }
                pc = 0x80216fac;
            }
            0x80216fac => {
                c.cmpu(0, c.r[31], c.r[30]);
                if c.crbit(2) { pc = 0x80216fbc; continue; }
                pc = 0x80216fb4;
            }
            0x80216fb4 => {
                c.r[29] = c.r[30] | c.r[30];
                pc = 0x80216fd8; continue;
                pc = 0x80216fbc;
            }
            0x80216fbc => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffffae28));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stb(c.r[13].wrapping_add(0xffffae28), c.r[0]);
                pc = 0x80216fc8;
            }
            0x80216fc8 => {
                c.r[6] = c.lbz(c.r[13].wrapping_add(0xffffae28));
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffae29));
                c.cmpu(0, c.r[6], c.r[0]);
                if c.crbit(0) { pc = 0x80216f88; continue; }
                pc = 0x80216fd8;
            }
            0x80216fd8 => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x80216ff8; continue; }
                pc = 0x80216fe0;
            }
            0x80216fe0 => {
                c.r[7] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80216ff8; f_801f4c14(c);
                pc = 0x80216ff8;
            }
            0x80216ff8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80216f50: bad pc {:#x}", pc),
        }
    }
}

/// fn_80217018 (0x80217018, 156 bytes)
#[inline(never)]
pub fn f_80217018(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80217018) { return; }
        if c.prof { crate::prof::enter(c, 0x80217018); f_80217018_b(c, 0x80217018); crate::prof::exit(c, 0x80217018); return; }
    }
    f_80217018_b(c, 0x80217018)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80217018_at(c: &mut Cpu, pc: u32) { f_80217018_b(c, pc) }
#[inline(always)]
fn f_80217018_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80217018;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80217018, pc); }
        match pc {
            0x80217018 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.lr = 0x80217034; f_801f025c(c);
                pc = 0x80217034;
            }
            0x80217034 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80217048; f_8012640c(c);
                pc = 0x80217048;
            }
            0x80217048 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.r[0] | c.r[0];
                c.lr = 0x80217058; f_80205b8c(c);
                pc = 0x80217058;
            }
            0x80217058 => {
                c.r[4] = c.r[1].wrapping_add(0xa);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80217064; f_80120b00(c);
                pc = 0x80217064;
            }
            0x80217064 => {
                c.r[7] = c.lhz(c.r[1].wrapping_add(0xa));
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x8021707c; f_8011bbd8(c);
                pc = 0x8021707c;
            }
            0x8021707c => {
                c.r[7] = c.lhz(c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.lr = 0x80217094; f_8011bbd8(c);
                pc = 0x80217094;
            }
            0x80217094 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80217018: bad pc {:#x}", pc),
        }
    }
}

/// fn_802170B4 (0x802170b4, 264 bytes)
#[inline(never)]
pub fn f_802170b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802170b4) { return; }
        if c.prof { crate::prof::enter(c, 0x802170b4); f_802170b4_b(c, 0x802170b4); crate::prof::exit(c, 0x802170b4); return; }
    }
    f_802170b4_b(c, 0x802170b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802170b4_at(c: &mut Cpu, pc: u32) { f_802170b4_b(c, pc) }
#[inline(always)]
fn f_802170b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802170b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802170b4, pc); }
        match pc {
            0x802170b4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802170d0; f_801f025c(c);
                pc = 0x802170d0;
            }
            0x802170d0 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x802170e4; f_8012640c(c);
                pc = 0x802170e4;
            }
            0x802170e4 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[7] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80217100; f_801f4c14(c);
                pc = 0x80217100;
            }
            0x80217100 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x8021710c; f_801f453c(c);
                pc = 0x8021710c;
            }
            0x8021710c => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80217118; f_80201704(c);
                pc = 0x80217118;
            }
            0x80217118 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8021719c; continue; }
                pc = 0x80217120;
            }
            0x80217120 => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x8021713c; continue; }
                pc = 0x80217128;
            }
            0x80217128 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2;
                c.lr = 0x80217134; f_80203b5c(c);
                pc = 0x80217134;
            }
            0x80217134 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80217174; continue;
                pc = 0x8021713c;
            }
            0x8021713c => {
                c.cmpu(0, c.r[29], 0x1);
                if !c.crbit(2) { pc = 0x80217164; continue; }
                pc = 0x80217144;
            }
            0x80217144 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x80217150; f_80203b5c(c);
                pc = 0x80217150;
            }
            0x80217150 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = 0x1e;
                c.r[3] = c.r[3].wrapping_mul(0x14);
                c.r[0] = divw(c.r[3], c.r[0]);
                pc = 0x80217174; continue;
                pc = 0x80217164;
            }
            0x80217164 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4;
                c.lr = 0x80217170; f_80203b5c(c);
                pc = 0x80217170;
            }
            0x80217170 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80217174;
            }
            0x80217174 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[7] = c.r[0].wrapping_neg();
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x8021718c; f_8011bbd8(c);
                pc = 0x8021718c;
            }
            0x8021718c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802171a8; continue;
                pc = 0x8021719c;
            }
            0x8021719c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802171a8;
            }
            0x802171a8 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802170b4: bad pc {:#x}", pc),
        }
    }
}

/// WS_MARUKUNARU (0x802171bc, 100 bytes)
#[inline(never)]
pub fn f_802171bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802171bc) { return; }
        if c.prof { crate::prof::enter(c, 0x802171bc); f_802171bc_b(c, 0x802171bc); crate::prof::exit(c, 0x802171bc); return; }
    }
    f_802171bc_b(c, 0x802171bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802171bc_at(c: &mut Cpu, pc: u32) { f_802171bc_b(c, pc) }
#[inline(always)]
fn f_802171bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802171bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802171bc, pc); }
        match pc {
            0x802171bc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x802171d8; f_801f025c(c);
                pc = 0x802171d8;
            }
            0x802171d8 => {
                c.r[4] = 0x1a;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x802171e4; f_802025b8(c);
                pc = 0x802171e4;
            }
            0x802171e4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80217200; continue; }
                pc = 0x802171f0;
            }
            0x802171f0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1a;
                c.r[5] = 0x0;
                c.lr = 0x80217200; f_8020248c(c);
                pc = 0x80217200;
            }
            0x80217200 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802171bc: bad pc {:#x}", pc),
        }
    }
}

/// fn_80217220 (0x80217220, 436 bytes)
#[inline(never)]
pub fn f_80217220(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80217220) { return; }
        if c.prof { crate::prof::enter(c, 0x80217220); f_80217220_b(c, 0x80217220); crate::prof::exit(c, 0x80217220); return; }
    }
    f_80217220_b(c, 0x80217220)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80217220_at(c: &mut Cpu, pc: u32) { f_80217220_b(c, pc) }
#[inline(always)]
fn f_80217220_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80217220;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80217220, pc); }
        match pc {
            0x80217220 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80217244; f_801f54a4(c);
                pc = 0x80217244;
            }
            0x80217244 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80217254; f_801f025c(c);
                pc = 0x80217254;
            }
            0x80217254 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80217268; f_801f025c(c);
                pc = 0x80217268;
            }
            0x80217268 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x8021727c; f_801f025c(c);
                pc = 0x8021727c;
            }
            0x8021727c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = 0xe;
                c.lr = 0x80217290; f_802026e4(c);
                pc = 0x80217290;
            }
            0x80217290 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80217344; continue; }
                pc = 0x8021729c;
            }
            0x8021729c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0xe;
                c.lr = 0x802172a8; f_80201d84(c);
                pc = 0x802172a8;
            }
            0x802172a8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80217344; continue; }
                pc = 0x802172b0;
            }
            0x802172b0 => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802172b8; f_801f00d0(c);
                pc = 0x802172b8;
            }
            0x802172b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80217344; continue; }
                pc = 0x802172c0;
            }
            0x802172c0 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x802172d8; f_801f4c14(c);
                pc = 0x802172d8;
            }
            0x802172d8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0xe;
                c.lr = 0x802172e4; f_80201c58(c);
                pc = 0x802172e4;
            }
            0x802172e4 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x802172fc; f_8011beb4(c);
                pc = 0x802172fc;
            }
            0x802172fc => {
                c.lr = 0x80217300; f_800fa280(c);
                pc = 0x80217300;
            }
            0x80217300 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x8021730c; f_80132a38(c);
                pc = 0x8021730c;
            }
            0x8021730c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0xe;
                c.lr = 0x80217318; f_80202810(c);
                pc = 0x80217318;
            }
            0x80217318 => {
                c.r[4] = 0x80380000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae0c));
                c.r[4] = c.r[4].wrapping_add(0xffff8efd);
                c.r[5] = 0x0;
                c.lr = 0x8021732c; f_80211b94(c);
                pc = 0x8021732c;
            }
            0x8021732c => {
                c.r[7] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x80217344; f_801f4c14(c);
                pc = 0x80217344;
            }
            0x80217344 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1c;
                c.lr = 0x80217350; f_802026e4(c);
                pc = 0x80217350;
            }
            0x80217350 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021737c; continue; }
                pc = 0x8021735c;
            }
            0x8021735c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1c;
                c.lr = 0x80217368; f_80202810(c);
                pc = 0x80217368;
            }
            0x80217368 => {
                c.r[4] = 0x80380000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae0c));
                c.r[4] = c.r[4].wrapping_add(0xffff8f11);
                c.r[5] = 0x0;
                c.lr = 0x8021737c; f_80211b94(c);
                pc = 0x8021737c;
            }
            0x8021737c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x4a;
                c.lr = 0x80217388; f_801f6e98(c);
                pc = 0x80217388;
            }
            0x80217388 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802173b4; continue; }
                pc = 0x80217394;
            }
            0x80217394 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x4a;
                c.lr = 0x802173a0; f_801f6eec(c);
                pc = 0x802173a0;
            }
            0x802173a0 => {
                c.r[4] = 0x80380000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffae0c));
                c.r[4] = c.r[4].wrapping_add(0xffff8f25);
                c.r[5] = 0x0;
                c.lr = 0x802173b4; f_80211b94(c);
                pc = 0x802173b4;
            }
            0x802173b4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80217220: bad pc {:#x}", pc),
        }
    }
}

/// WS_JIKOANJI (0x802173d4, 96 bytes)
#[inline(never)]
pub fn f_802173d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802173d4) { return; }
        if c.prof { crate::prof::enter(c, 0x802173d4); f_802173d4_b(c, 0x802173d4); crate::prof::exit(c, 0x802173d4); return; }
    }
    f_802173d4_b(c, 0x802173d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802173d4_at(c: &mut Cpu, pc: u32) { f_802173d4_b(c, pc) }
#[inline(always)]
fn f_802173d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802173d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802173d4, pc); }
        match pc {
            0x802173d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x802173f0; f_801f025c(c);
                pc = 0x802173f0;
            }
            0x802173f0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80217404; f_801f025c(c);
                pc = 0x80217404;
            }
            0x80217404 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x80217414; f_80201600(c);
                pc = 0x80217414;
            }
            0x80217414 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802173d4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80217434 (0x80217434, 240 bytes)
#[inline(never)]
pub fn f_80217434(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80217434) { return; }
        if c.prof { crate::prof::enter(c, 0x80217434); f_80217434_b(c, 0x80217434); crate::prof::exit(c, 0x80217434); return; }
    }
    f_80217434_b(c, 0x80217434)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80217434_at(c: &mut Cpu, pc: u32) { f_80217434_b(c, pc) }
#[inline(always)]
fn f_80217434_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80217434;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80217434, pc); }
        match pc {
            0x80217434 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80217450; f_801f025c(c);
                pc = 0x80217450;
            }
            0x80217450 => {
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80217464; f_8012640c(c);
                pc = 0x80217464;
            }
            0x80217464 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[29] = c.r[0] | c.r[0];
                c.lr = 0x80217474; f_80205b8c(c);
                pc = 0x80217474;
            }
            0x80217474 => {
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x80217484; f_8012640c(c);
                pc = 0x80217484;
            }
            0x80217484 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x2;
                c.lr = 0x80217494; f_80203b5c(c);
                pc = 0x80217494;
            }
            0x80217494 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xe6;
                c.r[6] = 0x0;
                c.lr = 0x802174ac; f_8012640c(c);
                pc = 0x802174ac;
            }
            0x802174ac => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0xc);
                if !c.crbit(0) { pc = 0x80217504; continue; }
                pc = 0x802174b8;
            }
            0x802174b8 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[28], c.r[0]);
                if !c.crbit(1) { pc = 0x80217504; continue; }
                pc = 0x802174c4;
            }
            0x802174c4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xe6;
                c.r[6] = 0x0;
                c.r[7] = 0xc;
                c.lr = 0x802174dc; f_801254b4(c);
                pc = 0x802174dc;
            }
            0x802174dc => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[7] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x802174f4; f_8011bbd8(c);
                pc = 0x802174f4;
            }
            0x802174f4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80217510; continue;
                pc = 0x80217504;
            }
            0x80217504 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80217510;
            }
            0x80217510 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80217434: bad pc {:#x}", pc),
        }
    }
}

/// fn_80217524 (0x80217524, 132 bytes)
#[inline(never)]
pub fn f_80217524(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80217524) { return; }
        if c.prof { crate::prof::enter(c, 0x80217524); f_80217524_b(c, 0x80217524); crate::prof::exit(c, 0x80217524); return; }
    }
    f_80217524_b(c, 0x80217524)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80217524_at(c: &mut Cpu, pc: u32) { f_80217524_b(c, pc) }
#[inline(always)]
fn f_80217524_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80217524;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80217524, pc); }
        match pc {
            0x80217524 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x53;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8021753c; f_801f2988(c);
                pc = 0x8021753c;
            }
            0x8021753c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x80217570; continue; }
                pc = 0x80217548;
            }
            0x80217548 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x3b;
                c.r[6] = 0x0;
                c.r[7] = 0x40;
                c.lr = 0x80217560; f_801f4c14(c);
                pc = 0x80217560;
            }
            0x80217560 => {
                c.r[0] = 0x2;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x8021758c; continue;
                pc = 0x80217570;
            }
            0x80217570 => {
                c.r[3] = 0x0;
                c.r[4] = 0x53;
                c.r[5] = 0x0;
                c.lr = 0x80217580; f_801f2934(c);
                pc = 0x80217580;
            }
            0x80217580 => {
                c.r[0] = 0x4;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x8021758c;
            }
            0x8021758c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80217524: bad pc {:#x}", pc),
        }
    }
}

/// fn_802175A8 (0x802175a8, 572 bytes)
#[inline(never)]
pub fn f_802175a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802175a8) { return; }
        if c.prof { crate::prof::enter(c, 0x802175a8); f_802175a8_b(c, 0x802175a8); crate::prof::exit(c, 0x802175a8); return; }
    }
    f_802175a8_b(c, 0x802175a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802175a8_at(c: &mut Cpu, pc: u32) { f_802175a8_b(c, pc) }
#[inline(always)]
fn f_802175a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802175a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802175a8, pc); }
        match pc {
            0x802175a8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x14;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802175c4; f_801f025c(c);
                pc = 0x802175c4;
            }
            0x802175c4 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x802175e0; f_801f54a4(c);
                pc = 0x802175e0;
            }
            0x802175e0 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802175ec; f_801f0134(c);
                pc = 0x802175ec;
            }
            0x802175ec => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802175f8; f_802062fc(c);
                pc = 0x802175f8;
            }
            0x802175f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8021760c; continue; }
                pc = 0x80217600;
            }
            0x80217600 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffae29));
                c.stb(c.r[13].wrapping_add(0xffffae28), c.r[0]);
                pc = 0x802177c4; continue;
                pc = 0x8021760c;
            }
            0x8021760c => {
                c.r[29] = 0x0;
                c.r[28] = 0x0;
                pc = 0x8021774c; continue;
                pc = 0x80217618;
            }
            0x80217618 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x5d;
                c.lr = 0x80217628; f_801f54a4(c);
                pc = 0x80217628;
            }
            0x80217628 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x80217630;
            }
            0x80217630 => {
                c.lr = 0x80217634; f_802062fc(c);
                pc = 0x80217634;
            }
            0x80217634 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x8021763c;
            }
            0x8021763c => {
                c.cmpu(0, c.r[30], c.r[29]);
                if c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x80217644;
            }
            0x80217644 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80217650; f_80202b88(c);
                pc = 0x80217650;
            }
            0x80217650 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x8021765c;
            }
            0x8021765c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xfe;
                c.r[6] = 0x0;
                c.lr = 0x80217670; f_8012640c(c);
                pc = 0x80217670;
            }
            0x80217670 => {
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                if c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x80217678;
            }
            0x80217678 => {
                c.lr = 0x8021767c; f_801f1170(c);
                pc = 0x8021767c;
            }
            0x8021767c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x80217684;
            }
            0x80217684 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8021768c; f_801f0898(c);
                pc = 0x8021768c;
            }
            0x8021768c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x13);
                if !c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x80217698;
            }
            0x80217698 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x802176ac; f_8012640c(c);
                pc = 0x802176ac;
            }
            0x802176ac => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[27] = c.r[0] | c.r[0];
                c.lr = 0x802176bc; f_80205184(c);
                pc = 0x802176bc;
            }
            0x802176bc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xe4);
                if !c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x802176c8;
            }
            0x802176c8 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.lr = 0x802176dc; f_8011beb4(c);
                pc = 0x802176dc;
            }
            0x802176dc => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x802176ec;
            }
            0x802176ec => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x8;
                c.lr = 0x802176f8; f_802026e4(c);
                pc = 0x802176f8;
            }
            0x802176f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x80217704;
            }
            0x80217704 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x7;
                c.lr = 0x80217710; f_802026e4(c);
                pc = 0x80217710;
            }
            0x80217710 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x8021771c;
            }
            0x8021771c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xf9;
                c.r[6] = 0x0;
                c.lr = 0x80217730; f_8012640c(c);
                pc = 0x80217730;
            }
            0x80217730 => {
                c.cmps(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x80217740; continue; }
                pc = 0x80217738;
            }
            0x80217738 => {
                c.r[28] = c.r[29] | c.r[29];
                pc = 0x8021775c; continue;
                pc = 0x80217740;
            }
            0x80217740 => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffffae28));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stb(c.r[13].wrapping_add(0xffffae28), c.r[0]);
                pc = 0x8021774c;
            }
            0x8021774c => {
                c.r[6] = c.lbz(c.r[13].wrapping_add(0xffffae28));
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffae29));
                c.cmpu(0, c.r[6], c.r[0]);
                if c.crbit(0) { pc = 0x80217618; continue; }
                pc = 0x8021775c;
            }
            0x8021775c => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x802177c4; continue; }
                pc = 0x80217764;
            }
            0x80217764 => {
                c.r[7] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x8021777c; f_801f4c14(c);
                pc = 0x8021777c;
            }
            0x8021777c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[3] = 0x80380000;
                c.r[3] = c.r[3].wrapping_add(0xffff9f58);
                c.r[4] = 0x1;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffbff;
                c.stb(c.r[3].wrapping_add(0x6002), c.r[4]);
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x802177b4; f_801254b4(c);
                pc = 0x802177b4;
            }
            0x802177b4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802177d0; continue;
                pc = 0x802177c4;
            }
            0x802177c4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802177d0;
            }
            0x802177d0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802175a8: bad pc {:#x}", pc),
        }
    }
}

/// fn_802177E4 (0x802177e4, 272 bytes)
#[inline(never)]
pub fn f_802177e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802177e4) { return; }
        if c.prof { crate::prof::enter(c, 0x802177e4); f_802177e4_b(c, 0x802177e4); crate::prof::exit(c, 0x802177e4); return; }
    }
    f_802177e4_b(c, 0x802177e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802177e4_at(c: &mut Cpu, pc: u32) { f_802177e4_b(c, pc) }
#[inline(always)]
fn f_802177e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802177e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802177e4, pc); }
        match pc {
            0x802177e4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80217800; f_801f025c(c);
                pc = 0x80217800;
            }
            0x80217800 => {
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80217810; f_8012640c(c);
                pc = 0x80217810;
            }
            0x80217810 => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x80217818; f_800e0c54(c);
                pc = 0x80217818;
            }
            0x80217818 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x64;
                c.r[0] = divw(c.r[4], c.r[3]);
                c.r[0] = c.r[0].wrapping_mul(c.r[3]);
                c.r[0] = c.r[4].wrapping_sub(c.r[0]);
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x5);
                if !c.crbit(0) { pc = 0x80217844; continue; }
                pc = 0x80217838;
            }
            0x80217838 => {
                c.r[0] = 0xa;
                c.r[31] = 0x4;
                pc = 0x802178b0; continue;
                pc = 0x80217844;
            }
            0x80217844 => {
                c.cmpu(0, c.r[0], 0xf);
                if !c.crbit(0) { pc = 0x80217858; continue; }
                pc = 0x8021784c;
            }
            0x8021784c => {
                c.r[0] = 0x1e;
                c.r[31] = 0x5;
                pc = 0x802178b0; continue;
                pc = 0x80217858;
            }
            0x80217858 => {
                c.cmpu(0, c.r[0], 0x23);
                if !c.crbit(0) { pc = 0x8021786c; continue; }
                pc = 0x80217860;
            }
            0x80217860 => {
                c.r[0] = 0x32;
                c.r[31] = 0x6;
                pc = 0x802178b0; continue;
                pc = 0x8021786c;
            }
            0x8021786c => {
                c.cmpu(0, c.r[0], 0x41);
                if !c.crbit(0) { pc = 0x80217880; continue; }
                pc = 0x80217874;
            }
            0x80217874 => {
                c.r[0] = 0x46;
                c.r[31] = 0x7;
                pc = 0x802178b0; continue;
                pc = 0x80217880;
            }
            0x80217880 => {
                c.cmpu(0, c.r[0], 0x55);
                if !c.crbit(0) { pc = 0x80217894; continue; }
                pc = 0x80217888;
            }
            0x80217888 => {
                c.r[0] = 0x5a;
                c.r[31] = 0x8;
                pc = 0x802178b0; continue;
                pc = 0x80217894;
            }
            0x80217894 => {
                c.cmpu(0, c.r[0], 0x5f);
                if !c.crbit(0) { pc = 0x802178a8; continue; }
                pc = 0x8021789c;
            }
            0x8021789c => {
                c.r[0] = 0x6e;
                c.r[31] = 0x9;
                pc = 0x802178b0; continue;
                pc = 0x802178a8;
            }
            0x802178a8 => {
                c.r[0] = 0x96;
                c.r[31] = 0xa;
                pc = 0x802178b0;
            }
            0x802178b0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[7] = c.r[0].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x802178c8; f_8011bbd8(c);
                pc = 0x802178c8;
            }
            0x802178c8 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.r[3] = 0x2f;
                c.lr = 0x802178d4; f_80132a38(c);
                pc = 0x802178d4;
            }
            0x802178d4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802177e4: bad pc {:#x}", pc),
        }
    }
}

/// fn_802178F4 (0x802178f4, 168 bytes)
#[inline(never)]
pub fn f_802178f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802178f4) { return; }
        if c.prof { crate::prof::enter(c, 0x802178f4); f_802178f4_b(c, 0x802178f4); crate::prof::exit(c, 0x802178f4); return; }
    }
    f_802178f4_b(c, 0x802178f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802178f4_at(c: &mut Cpu, pc: u32) { f_802178f4_b(c, pc) }
#[inline(always)]
fn f_802178f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802178f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802178f4, pc); }
        match pc {
            0x802178f4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80217910; f_801f025c(c);
                pc = 0x80217910;
            }
            0x80217910 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[4] = c.r[0] | c.r[0];
                c.lr = 0x80217920; f_801f025c(c);
                pc = 0x80217920;
            }
            0x80217920 => {
                c.r[4] = 0x4b;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8021792c; f_801f6e44(c);
                pc = 0x8021792c;
            }
            0x8021792c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x80217960; continue; }
                pc = 0x80217938;
            }
            0x80217938 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x3b;
                c.r[6] = 0x0;
                c.r[7] = 0x40;
                c.lr = 0x80217950; f_801f4c14(c);
                pc = 0x80217950;
            }
            0x80217950 => {
                c.r[0] = 0x0;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x8021797c; continue;
                pc = 0x80217960;
            }
            0x80217960 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4b;
                c.r[5] = 0x0;
                c.lr = 0x80217970; f_801f6df0(c);
                pc = 0x80217970;
            }
            0x80217970 => {
                c.r[0] = 0x5;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                pc = 0x8021797c;
            }
            0x8021797c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802178f4: bad pc {:#x}", pc),
        }
    }
}

/// fn_8021799C (0x8021799c, 328 bytes)
#[inline(never)]
pub fn f_8021799c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8021799c) { return; }
        if c.prof { crate::prof::enter(c, 0x8021799c); f_8021799c_b(c, 0x8021799c); crate::prof::exit(c, 0x8021799c); return; }
    }
    f_8021799c_b(c, 0x8021799c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8021799c_at(c: &mut Cpu, pc: u32) { f_8021799c_b(c, pc) }
#[inline(always)]
fn f_8021799c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8021799c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8021799c, pc); }
        match pc {
            0x8021799c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802179b8; f_801f025c(c);
                pc = 0x802179b8;
            }
            0x802179b8 => {
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x802179c8; f_8012640c(c);
                pc = 0x802179c8;
            }
            0x802179c8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x802179dc; f_801f025c(c);
                pc = 0x802179dc;
            }
            0x802179dc => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x802179e4; f_800e0c54(c);
                pc = 0x802179e4;
            }
            0x802179e4 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[29], 0x66);
                if !c.crbit(0) { pc = 0x80217a0c; continue; }
                pc = 0x802179f0;
            }
            0x802179f0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.r[7] = 0x28;
                c.lr = 0x80217a08; f_8011bbd8(c);
                pc = 0x80217a08;
            }
            0x80217a08 => {
                pc = 0x80217a7c; continue;
                pc = 0x80217a0c;
            }
            0x80217a0c => {
                c.cmpu(0, c.r[29], 0xb2);
                if !c.crbit(0) { pc = 0x80217a30; continue; }
                pc = 0x80217a14;
            }
            0x80217a14 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.r[7] = 0x50;
                c.lr = 0x80217a2c; f_8011bbd8(c);
                pc = 0x80217a2c;
            }
            0x80217a2c => {
                pc = 0x80217a7c; continue;
                pc = 0x80217a30;
            }
            0x80217a30 => {
                c.cmpu(0, c.r[29], 0xcc);
                if !c.crbit(0) { pc = 0x80217a54; continue; }
                pc = 0x80217a38;
            }
            0x80217a38 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.r[7] = 0x78;
                c.lr = 0x80217a50; f_8011bbd8(c);
                pc = 0x80217a50;
            }
            0x80217a50 => {
                pc = 0x80217a7c; continue;
                pc = 0x80217a54;
            }
            0x80217a54 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4;
                c.lr = 0x80217a60; f_80203b5c(c);
                pc = 0x80217a60;
            }
            0x80217a60 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[7] = c.r[0].wrapping_neg();
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x80217a7c; f_8011bbd8(c);
                pc = 0x80217a7c;
            }
            0x80217a7c => {
                c.cmpu(0, c.r[29], 0xcc);
                if !c.crbit(0) { pc = 0x80217a94; continue; }
                pc = 0x80217a84;
            }
            0x80217a84 => {
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5f98);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80217ad0; continue;
                pc = 0x80217a94;
            }
            0x80217a94 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80217a9c; f_80201704(c);
                pc = 0x80217a9c;
            }
            0x80217a9c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80217ab8; continue; }
                pc = 0x80217aa8;
            }
            0x80217aa8 => {
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x7ad9);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80217ad0; continue;
                pc = 0x80217ab8;
            }
            0x80217ab8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x43;
                c.lr = 0x80217ac4; f_80209960(c);
                pc = 0x80217ac4;
            }
            0x80217ac4 => {
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x7ab8);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80217ad0;
            }
            0x80217ad0 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8021799c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80217AE4 (0x80217ae4, 236 bytes)
#[inline(never)]
pub fn f_80217ae4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80217ae4) { return; }
        if c.prof { crate::prof::enter(c, 0x80217ae4); f_80217ae4_b(c, 0x80217ae4); crate::prof::exit(c, 0x80217ae4); return; }
    }
    f_80217ae4_b(c, 0x80217ae4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80217ae4_at(c: &mut Cpu, pc: u32) { f_80217ae4_b(c, pc) }
#[inline(always)]
fn f_80217ae4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80217ae4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80217ae4, pc); }
        match pc {
            0x80217ae4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80217b00; f_801f025c(c);
                pc = 0x80217b00;
            }
            0x80217b00 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80217b08; f_80205b8c(c);
                pc = 0x80217b08;
            }
            0x80217b08 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80217b24; f_8012640c(c);
                pc = 0x80217b24;
            }
            0x80217b24 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.r[0] | c.r[0];
                c.lr = 0x80217b34; f_80205184(c);
                pc = 0x80217b34;
            }
            0x80217b34 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x80217b4c; f_8011beb4(c);
                pc = 0x80217b4c;
            }
            0x80217b4c => {
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x99;
                c.r[6] = 0x0;
                c.lr = 0x80217b64; f_8012640c(c);
                pc = 0x80217b64;
            }
            0x80217b64 => {
                c.cmpu(0, c.r[30], 0x79);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                if !c.crbit(2) { pc = 0x80217b94; continue; }
                pc = 0x80217b80;
            }
            0x80217b80 => {
                c.r[7] = c.r[0].wrapping_mul(0xa);
                c.r[0] = 0x19;
                c.r[0] = divw(c.r[7], c.r[0]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80217ba8; continue;
                pc = 0x80217b94;
            }
            0x80217b94 => {
                c.r[7] = c.addc(!c.r[0], 0xff, 1);
                c.r[0] = 0x19;
                c.r[7] = c.r[7].wrapping_mul(0xa);
                c.r[0] = divw(c.r[7], c.r[0]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80217ba8;
            }
            0x80217ba8 => {
                c.r[7] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x80217bb0; f_8011bbd8(c);
                pc = 0x80217bb0;
            }
            0x80217bb0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80217ae4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80217BD0 (0x80217bd0, 28 bytes)
#[inline(never)]
pub fn f_80217bd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80217bd0) { return; }
        if c.prof { crate::prof::enter(c, 0x80217bd0); f_80217bd0_b(c, 0x80217bd0); crate::prof::exit(c, 0x80217bd0); return; }
    }
    f_80217bd0_b(c, 0x80217bd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80217bd0_at(c: &mut Cpu, pc: u32) { f_80217bd0_b(c, pc) }
#[inline(always)]
fn f_80217bd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80217bd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80217bd0, pc); }
        match pc {
            0x80217bd0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = 0x19;
                c.r[3] = c.addc(!c.r[3], 0xff, 1);
                c.r[3] = c.r[3].wrapping_mul(0xa);
                c.r[0] = divw(c.r[3], c.r[0]);
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                return;
            }
            _ => panic!("f_80217bd0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80217BEC (0x80217bec, 24 bytes)
#[inline(never)]
pub fn f_80217bec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80217bec) { return; }
        if c.prof { crate::prof::enter(c, 0x80217bec); f_80217bec_b(c, 0x80217bec); crate::prof::exit(c, 0x80217bec); return; }
    }
    f_80217bec_b(c, 0x80217bec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80217bec_at(c: &mut Cpu, pc: u32) { f_80217bec_b(c, pc) }
#[inline(always)]
fn f_80217bec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80217bec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80217bec, pc); }
        match pc {
            0x80217bec => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = 0x19;
                c.r[3] = c.r[3].wrapping_mul(0xa);
                c.r[0] = divw(c.r[3], c.r[0]);
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                return;
            }
            _ => panic!("f_80217bec: bad pc {:#x}", pc),
        }
    }
}

/// fn_80217C04 (0x80217c04, 304 bytes)
#[inline(never)]
pub fn f_80217c04(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80217c04) { return; }
        if c.prof { crate::prof::enter(c, 0x80217c04); f_80217c04_b(c, 0x80217c04); crate::prof::exit(c, 0x80217c04); return; }
    }
    f_80217c04_b(c, 0x80217c04)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80217c04_at(c: &mut Cpu, pc: u32) { f_80217c04_b(c, pc) }
#[inline(always)]
fn f_80217c04_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80217c04;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80217c04, pc); }
        match pc {
            0x80217c04 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80217c20; f_801f025c(c);
                pc = 0x80217c20;
            }
            0x80217c20 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80217c34; f_8012640c(c);
                pc = 0x80217c34;
            }
            0x80217c34 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x80217c44; f_80205184(c);
                pc = 0x80217c44;
            }
            0x80217c44 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[29] = c.r[0] | c.r[0];
                c.lr = 0x80217c54; f_802096e8(c);
                pc = 0x80217c54;
            }
            0x80217c54 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80217c90; continue; }
                pc = 0x80217c5c;
            }
            0x80217c5c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2e;
                c.lr = 0x80217c68; f_802026e4(c);
                pc = 0x80217c68;
            }
            0x80217c68 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80217c80; continue; }
                pc = 0x80217c74;
            }
            0x80217c74 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2e;
                c.lr = 0x80217c80; f_80202810(c);
                pc = 0x80217c80;
            }
            0x80217c80 => {
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5fdf);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80217d20; continue;
                pc = 0x80217c90;
            }
            0x80217c90 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2e;
                c.lr = 0x80217c9c; f_802025b8(c);
                pc = 0x80217c9c;
            }
            0x80217c9c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80217cb8; continue; }
                pc = 0x80217ca8;
            }
            0x80217ca8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2e;
                c.r[5] = 0x0;
                c.lr = 0x80217cb8; f_8020248c(c);
                pc = 0x80217cb8;
            }
            0x80217cb8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2e;
                c.lr = 0x80217cc4; f_80202360(c);
                pc = 0x80217cc4;
            }
            0x80217cc4 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x80217cdc; f_8011beb4(c);
                pc = 0x80217cdc;
            }
            0x80217cdc => {
                c.r[4] = c.r[31] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                c.r[7] = c.r[3] | c.r[3];
                c.mtspr(9, c.r[0]);
                c.cmps(0, c.r[4], 0x1);
                if !c.crbit(1) { pc = 0x80217d00; continue; }
                pc = 0x80217cf8;
            }
            0x80217cf8 => {
                c.r[7] = c.r[7].rotate_left(1) & 0xfffffffe;
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80217cf8; continue; }
                pc = 0x80217d00;
            }
            0x80217d00 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80217d14; f_8011bbd8(c);
                pc = 0x80217d14;
            }
            0x80217d14 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80217d20;
            }
            0x80217d20 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80217c04: bad pc {:#x}", pc),
        }
    }
}

/// fn_80217D34 (0x80217d34, 236 bytes)
#[inline(never)]
pub fn f_80217d34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80217d34) { return; }
        if c.prof { crate::prof::enter(c, 0x80217d34); f_80217d34_b(c, 0x80217d34); crate::prof::exit(c, 0x80217d34); return; }
    }
    f_80217d34_b(c, 0x80217d34)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80217d34_at(c: &mut Cpu, pc: u32) { f_80217d34_b(c, pc) }
#[inline(always)]
fn f_80217d34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80217d34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80217d34, pc); }
        match pc {
            0x80217d34 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x80217d38;
            }
            0x80217d38 => {
                c.r[0] = c.mfspr(8);
                pc = 0x80217d3c;
            }
            0x80217d3c => {
                c.r[3] = 0x12;
                pc = 0x80217d40;
            }
            0x80217d40 => {
                c.r[4] = 0x0;
                pc = 0x80217d44;
            }
            0x80217d44 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                pc = 0x80217d48;
            }
            0x80217d48 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x80217d4c;
            }
            0x80217d4c => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                pc = 0x80217d50;
            }
            0x80217d50 => {
                c.r[31] = c.lbz(c.r[5].wrapping_add(0x1));
                pc = 0x80217d54;
            }
            0x80217d54 => {
                c.lr = 0x80217d58; f_801f025c(c);
                pc = 0x80217d58;
            }
            0x80217d58 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x80217d5c;
            }
            0x80217d5c => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80217d60;
            }
            0x80217d60 => {
                c.cmpu(0, c.r[0], 0x7);
                pc = 0x80217d64;
            }
            0x80217d64 => {
                if c.crbit(1) { pc = 0x80217db8; continue; }
                pc = 0x80217d68;
            }
            0x80217d68 => {
                c.r[3] = 0x803a0000;
                pc = 0x80217d6c;
            }
            0x80217d6c => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80217d70;
            }
            0x80217d70 => {
                c.r[3] = c.r[3].wrapping_add(0xffffa028);
                pc = 0x80217d74;
            }
            0x80217d74 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x80217d78;
            }
            0x80217d78 => {
                c.mtspr(9, c.r[0]);
                pc = 0x80217d7c;
            }
            0x80217d7c => {
                if true { let t = c.ctr; if (0x80217d34..0x80217e20).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80217d80;
            }
            0x80217d80 => {
                c.r[5] = 0xe6;
                pc = 0x80217d84;
            }
            0x80217d84 => {
                pc = 0x80217dbc; continue;
                pc = 0x80217d88;
            }
            0x80217d88 => {
                c.r[5] = 0xe7;
                pc = 0x80217d8c;
            }
            0x80217d8c => {
                pc = 0x80217dbc; continue;
                pc = 0x80217d90;
            }
            0x80217d90 => {
                c.r[5] = 0xea;
                pc = 0x80217d94;
            }
            0x80217d94 => {
                pc = 0x80217dbc; continue;
                pc = 0x80217d98;
            }
            0x80217d98 => {
                c.r[5] = 0xe8;
                pc = 0x80217d9c;
            }
            0x80217d9c => {
                pc = 0x80217dbc; continue;
                pc = 0x80217da0;
            }
            0x80217da0 => {
                c.r[5] = 0xe9;
                pc = 0x80217da4;
            }
            0x80217da4 => {
                pc = 0x80217dbc; continue;
                pc = 0x80217da8;
            }
            0x80217da8 => {
                c.r[5] = 0xeb;
                pc = 0x80217dac;
            }
            0x80217dac => {
                pc = 0x80217dbc; continue;
                pc = 0x80217db0;
            }
            0x80217db0 => {
                c.r[5] = 0xec;
                pc = 0x80217db4;
            }
            0x80217db4 => {
                pc = 0x80217dbc; continue;
                pc = 0x80217db8;
            }
            0x80217db8 => {
                c.r[5] = 0x0;
                pc = 0x80217dbc;
            }
            0x80217dbc => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80217dc0;
            }
            0x80217dc0 => {
                c.r[4] = 0x0;
                pc = 0x80217dc4;
            }
            0x80217dc4 => {
                c.r[6] = 0x0;
                pc = 0x80217dc8;
            }
            0x80217dc8 => {
                c.lr = 0x80217dcc; f_8012640c(c);
                pc = 0x80217dcc;
            }
            0x80217dcc => {
                c.r[30] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x80217dd0;
            }
            0x80217dd0 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80217dd4;
            }
            0x80217dd4 => {
                c.r[4] = 0x9;
                pc = 0x80217dd8;
            }
            0x80217dd8 => {
                c.lr = 0x80217ddc; f_802026e4(c);
                pc = 0x80217ddc;
            }
            0x80217ddc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80217de0;
            }
            0x80217de0 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x80217de4;
            }
            0x80217de4 => {
                if !c.crbit(2) { pc = 0x80217e00; continue; }
                pc = 0x80217de8;
            }
            0x80217de8 => {
                c.cmps(0, c.r[30], 0xc);
                pc = 0x80217dec;
            }
            0x80217dec => {
                if c.crbit(0) { pc = 0x80217e00; continue; }
                pc = 0x80217df0;
            }
            0x80217df0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                pc = 0x80217df4;
            }
            0x80217df4 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x2));
                pc = 0x80217df8;
            }
            0x80217df8 => {
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80217dfc;
            }
            0x80217dfc => {
                pc = 0x80217e0c; continue;
                pc = 0x80217e00;
            }
            0x80217e00 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                pc = 0x80217e04;
            }
            0x80217e04 => {
                c.r[0] = c.r[3].wrapping_add(0x6);
                pc = 0x80217e08;
            }
            0x80217e08 => {
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80217e0c;
            }
            0x80217e0c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                pc = 0x80217e10;
            }
            0x80217e10 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x80217e14;
            }
            0x80217e14 => {
                c.mtspr(8, c.r[0]);
                pc = 0x80217e18;
            }
            0x80217e18 => {
                c.r[1] = c.r[1].wrapping_add(0x10);
                pc = 0x80217e1c;
            }
            0x80217e1c => {
                if true { return; }
                return;
            }
            _ => panic!("f_80217d34: bad pc {:#x}", pc),
        }
    }
}

/// fn_80217E20 (0x80217e20, 504 bytes)
#[inline(never)]
pub fn f_80217e20(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80217e20) { return; }
        if c.prof { crate::prof::enter(c, 0x80217e20); f_80217e20_b(c, 0x80217e20); crate::prof::exit(c, 0x80217e20); return; }
    }
    f_80217e20_b(c, 0x80217e20)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80217e20_at(c: &mut Cpu, pc: u32) { f_80217e20_b(c, pc) }
#[inline(always)]
fn f_80217e20_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80217e20;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80217e20, pc); }
        match pc {
            0x80217e20 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80217e3c; f_801f025c(c);
                pc = 0x80217e3c;
            }
            0x80217e3c => {
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80217e50; f_8012640c(c);
                pc = 0x80217e50;
            }
            0x80217e50 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xf8;
                c.r[6] = 0x0;
                c.lr = 0x80217e6c; f_8012640c(c);
                pc = 0x80217e6c;
            }
            0x80217e6c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[31] = c.r[0] | c.r[0];
                c.lr = 0x80217e7c; f_80205184(c);
                pc = 0x80217e7c;
            }
            0x80217e7c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[28] = c.r[0] | c.r[0];
                c.lr = 0x80217e8c; f_802096e8(c);
                pc = 0x80217e8c;
            }
            0x80217e8c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80217eac; continue; }
                pc = 0x80217e94;
            }
            0x80217e94 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80217e9c; f_80200b10(c);
                pc = 0x80217e9c;
            }
            0x80217e9c => {
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5fdf);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218004; continue;
                pc = 0x80217eac;
            }
            0x80217eac => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x2f;
                c.lr = 0x80217eb8; f_802025b8(c);
                pc = 0x80217eb8;
            }
            0x80217eb8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80217f04; continue; }
                pc = 0x80217ec4;
            }
            0x80217ec4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x2f;
                c.r[5] = 0x0;
                c.lr = 0x80217ed4; f_8020248c(c);
                pc = 0x80217ed4;
            }
            0x80217ed4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x22;
                c.lr = 0x80217ee0; f_802026e4(c);
                pc = 0x80217ee0;
            }
            0x80217ee0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80217f04; continue; }
                pc = 0x80217ee8;
            }
            0x80217ee8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x22;
                c.r[5] = 0x0;
                c.lr = 0x80217ef8; f_8020248c(c);
                pc = 0x80217ef8;
            }
            0x80217ef8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80217f04; f_8020a2b8(c);
                pc = 0x80217f04;
            }
            0x80217f04 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[27] = 0x1;
                c.r[4] = 0x2f;
                c.lr = 0x80217f14; f_802026e4(c);
                pc = 0x80217f14;
            }
            0x80217f14 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80217f60; continue; }
                pc = 0x80217f20;
            }
            0x80217f20 => {
                c.r[3] = 0x2f;
                c.lr = 0x80217f28; f_80119dd0(c);
                pc = 0x80217f28;
            }
            0x80217f28 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x2f;
                c.lr = 0x80217f38; f_80202360(c);
                pc = 0x80217f38;
            }
            0x80217f38 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[0] = c.r[27] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], c.r[31]);
                if !c.crbit(2) { pc = 0x80217f60; continue; }
                pc = 0x80217f48;
            }
            0x80217f48 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x2f;
                c.lr = 0x80217f54; f_80202810(c);
                pc = 0x80217f54;
            }
            0x80217f54 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x22;
                c.lr = 0x80217f60; f_80202810(c);
                pc = 0x80217f60;
            }
            0x80217f60 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x80217f74; f_8011beb4(c);
                pc = 0x80217f74;
            }
            0x80217f74 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] as u16 as i16 as i32 as u32;
                c.r[5] = 0x1;
                pc = 0x80217f8c; continue;
                pc = 0x80217f84;
            }
            0x80217f84 => {
                c.r[4] = c.r[4].rotate_left(1) & 0xfffe;
                c.r[5] = c.r[5].wrapping_add(0x1);
                pc = 0x80217f8c;
            }
            0x80217f8c => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[0], c.r[3]);
                if c.crbit(0) { pc = 0x80217f84; continue; }
                pc = 0x80217f98;
            }
            0x80217f98 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[7] = c.r[4].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80217fb0; f_8011bbd8(c);
                pc = 0x80217fb0;
            }
            0x80217fb0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1a;
                c.lr = 0x80217fbc; f_802026e4(c);
                pc = 0x80217fbc;
            }
            0x80217fbc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80217ff8; continue; }
                pc = 0x80217fc8;
            }
            0x80217fc8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80217fdc; f_8011beb4(c);
                pc = 0x80217fdc;
            }
            0x80217fdc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[7] = c.r[0].rotate_left(1) & 0xfffe;
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80217ff8; f_8011bbd8(c);
                pc = 0x80217ff8;
            }
            0x80217ff8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218004;
            }
            0x80218004 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80217e20: bad pc {:#x}", pc),
        }
    }
}

/// fn_80218018 (0x80218018, 448 bytes)
#[inline(never)]
pub fn f_80218018(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80218018) { return; }
        if c.prof { crate::prof::enter(c, 0x80218018); f_80218018_b(c, 0x80218018); crate::prof::exit(c, 0x80218018); return; }
    }
    f_80218018_b(c, 0x80218018)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80218018_at(c: &mut Cpu, pc: u32) { f_80218018_b(c, pc) }
#[inline(always)]
fn f_80218018_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80218018;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80218018, pc); }
        match pc {
            0x80218018 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[0] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.lr = 0x8021803c; f_801f025c(c);
                pc = 0x8021803c;
            }
            0x8021803c => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80218044; f_80205184(c);
                pc = 0x80218044;
            }
            0x80218044 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[27] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80218058; f_801f025c(c);
                pc = 0x80218058;
            }
            0x80218058 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[5] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8021806c; f_80229934(c);
                pc = 0x8021806c;
            }
            0x8021806c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80218088; continue; }
                pc = 0x80218078;
            }
            0x80218078 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802181c4; continue;
                pc = 0x80218088;
            }
            0x80218088 => {
                c.r[3] = 0x80220000;
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[3].wrapping_add(0xffff81d8);
                c.r[6] = 0x0;
                c.r[3] = 0x0;
                c.lr = 0x802180a0; f_801f37b0(c);
                pc = 0x802180a0;
            }
            0x802180a0 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x802181a0; continue; }
                pc = 0x802180a8;
            }
            0x802180a8 => {
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[4] = 0x2e;
                c.r[5] = 0x0;
                c.lr = 0x802180bc; f_801f3624(c);
                pc = 0x802180bc;
            }
            0x802180bc => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x802181a0; continue; }
                pc = 0x802180c4;
            }
            0x802180c4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802180cc; f_80205b8c(c);
                pc = 0x802180cc;
            }
            0x802180cc => {
                c.r[4] = 0xc3;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x802180d8; f_80123b5c(c);
                pc = 0x802180d8;
            }
            0x802180d8 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x802181a0; continue; }
                pc = 0x802180e4;
            }
            0x802180e4 => {
                c.r[27] = c.r[30] as u8 as i8 as i32 as u32;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x80;
                c.lr = 0x802180fc; f_8012640c(c);
                pc = 0x802180fc;
            }
            0x802180fc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[5] = c.addc(!c.r[0], c.r[29], 1);
                c.r[6] = c.r[27] | c.r[27];
                c.r[0] = c.r[0].wrapping_sub(c.r[29]);
                c.r[4] = 0x0;
                { let ci = c.ca(); c.r[7] = c.addc(!c.r[5], c.r[5], ci); }
                c.r[5] = 0x80;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[0] & c.r[7];
                c.r[7] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x8021812c; f_801254b4(c);
                pc = 0x8021812c;
            }
            0x8021812c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[28] = c.r[30].rotate_left(0) & 0xff;
                c.r[4] = 0x10;
                c.lr = 0x8021813c; f_802026e4(c);
                pc = 0x8021813c;
            }
            0x8021813c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802181a0; continue; }
                pc = 0x80218144;
            }
            0x80218144 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x31;
                c.lr = 0x80218150; f_802026e4(c);
                pc = 0x80218150;
            }
            0x80218150 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802181a0; continue; }
                pc = 0x8021815c;
            }
            0x8021815c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x31;
                c.lr = 0x80218168; f_80201890(c);
                pc = 0x80218168;
            }
            0x80218168 => {
                c.r[0] = 0x1;
                c.r[0] = slw(c.r[0], c.r[28]);
                c.r[0] = c.r[3] & c.r[0];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x802181a0; continue; }
                pc = 0x80218178;
            }
            0x80218178 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80218180; f_801fecd4(c);
                pc = 0x80218180;
            }
            0x80218180 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802181a0; continue; }
                pc = 0x8021818c;
            }
            0x8021818c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[4] = 0x80;
                c.r[6] = 0x0;
                c.lr = 0x802181a0; f_801fe7ec(c);
                pc = 0x802181a0;
            }
            0x802181a0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x8));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x802181b8; continue; }
                pc = 0x802181ac;
            }
            0x802181ac => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                pc = 0x802181c0; continue;
                pc = 0x802181b8;
            }
            0x802181b8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                pc = 0x802181c0;
            }
            0x802181c0 => {
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802181c4;
            }
            0x802181c4 => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80218018: bad pc {:#x}", pc),
        }
    }
}

/// _fightSeqWsHorobinoutaSub__FPvUsPv (0x802181d8, 152 bytes)
#[inline(never)]
pub fn f_802181d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802181d8) { return; }
        if c.prof { crate::prof::enter(c, 0x802181d8); f_802181d8_b(c, 0x802181d8); crate::prof::exit(c, 0x802181d8); return; }
    }
    f_802181d8_b(c, 0x802181d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802181d8_at(c: &mut Cpu, pc: u32) { f_802181d8_b(c, pc) }
#[inline(always)]
fn f_802181d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802181d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802181d8, pc); }
        match pc {
            0x802181d8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x802181f4; f_802062fc(c);
                pc = 0x802181f4;
            }
            0x802181f4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80218204; continue; }
                pc = 0x802181fc;
            }
            0x802181fc => {
                c.r[3] = 0x1;
                pc = 0x8021825c; continue;
                pc = 0x80218204;
            }
            0x80218204 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8021820c; f_80207bf4(c);
                pc = 0x8021820c;
            }
            0x8021820c => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1e;
                c.lr = 0x8021821c; f_802025b8(c);
                pc = 0x8021821c;
            }
            0x8021821c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80218234; continue; }
                pc = 0x80218228;
            }
            0x80218228 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2b);
                if !c.crbit(2) { pc = 0x8021823c; continue; }
                pc = 0x80218234;
            }
            0x80218234 => {
                c.r[3] = 0x1;
                pc = 0x8021825c; continue;
                pc = 0x8021823c;
            }
            0x8021823c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1e;
                c.r[5] = 0x0;
                c.lr = 0x8021824c; f_8020248c(c);
                pc = 0x8021824c;
            }
            0x8021824c => {
                c.r[4] = c.lwz(c.r[29]);
                c.r[3] = 0x1;
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[29], c.r[0]);
                pc = 0x8021825c;
            }
            0x8021825c => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802181d8: bad pc {:#x}", pc),
        }
    }
}

/// WS_MIYABURU (0x80218270, 100 bytes)
#[inline(never)]
pub fn f_80218270(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80218270) { return; }
        if c.prof { crate::prof::enter(c, 0x80218270); f_80218270_b(c, 0x80218270); crate::prof::exit(c, 0x80218270); return; }
    }
    f_80218270_b(c, 0x80218270)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80218270_at(c: &mut Cpu, pc: u32) { f_80218270_b(c, pc) }
#[inline(always)]
fn f_80218270_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80218270;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80218270, pc); }
        match pc {
            0x80218270 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x8021828c; f_801f025c(c);
                pc = 0x8021828c;
            }
            0x8021828c => {
                c.r[4] = 0x19;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80218298; f_802025b8(c);
                pc = 0x80218298;
            }
            0x80218298 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x802182b4; continue; }
                pc = 0x802182a4;
            }
            0x802182a4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x19;
                c.r[5] = 0x0;
                c.lr = 0x802182b4; f_8020248c(c);
                pc = 0x802182b4;
            }
            0x802182b4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80218270: bad pc {:#x}", pc),
        }
    }
}

/// fn_802182D4 (0x802182d4, 232 bytes)
#[inline(never)]
pub fn f_802182d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802182d4) { return; }
        if c.prof { crate::prof::enter(c, 0x802182d4); f_802182d4_b(c, 0x802182d4); crate::prof::exit(c, 0x802182d4); return; }
    }
    f_802182d4_b(c, 0x802182d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802182d4_at(c: &mut Cpu, pc: u32) { f_802182d4_b(c, pc) }
#[inline(always)]
fn f_802182d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802182d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802182d4, pc); }
        match pc {
            0x802182d4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802182f0; f_801f025c(c);
                pc = 0x802182f0;
            }
            0x802182f0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x3;
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80218304; f_801f025c(c);
                pc = 0x80218304;
            }
            0x80218304 => {
                c.r[4] = 0x4a;
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x80218310; f_801f6e44(c);
                pc = 0x80218310;
            }
            0x80218310 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80218364; continue; }
                pc = 0x8021831c;
            }
            0x8021831c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x4a;
                c.lr = 0x80218328; f_801f6e98(c);
                pc = 0x80218328;
            }
            0x80218328 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80218348; continue; }
                pc = 0x80218334;
            }
            0x80218334 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x4a;
                c.lr = 0x80218340; f_801f6d9c(c);
                pc = 0x80218340;
            }
            0x80218340 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8021834c; continue;
                pc = 0x80218348;
            }
            0x80218348 => {
                c.r[31] = 0x0;
                pc = 0x8021834c;
            }
            0x8021834c => {
                c.r[3] = 0x4a;
                c.lr = 0x80218354; f_80119dd0(c);
                pc = 0x80218354;
            }
            0x80218354 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8021838c; continue; }
                pc = 0x80218364;
            }
            0x80218364 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x118;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x8021837c; f_801254b4(c);
                pc = 0x8021837c;
            }
            0x8021837c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802183a8; continue;
                pc = 0x8021838c;
            }
            0x8021838c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x4a;
                c.r[5] = 0x0;
                c.lr = 0x8021839c; f_801f6df0(c);
                pc = 0x8021839c;
            }
            0x8021839c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x802183a8;
            }
            0x802183a8 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802182d4: bad pc {:#x}", pc),
        }
    }
}

/// fn_802183BC (0x802183bc, 192 bytes)
#[inline(never)]
pub fn f_802183bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802183bc) { return; }
        if c.prof { crate::prof::enter(c, 0x802183bc); f_802183bc_b(c, 0x802183bc); crate::prof::exit(c, 0x802183bc); return; }
    }
    f_802183bc_b(c, 0x802183bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802183bc_at(c: &mut Cpu, pc: u32) { f_802183bc_b(c, pc) }
#[inline(always)]
fn f_802183bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802183bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802183bc, pc); }
        match pc {
            0x802183bc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802183d8; f_801f025c(c);
                pc = 0x802183d8;
            }
            0x802183d8 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x802183ec; f_8012640c(c);
                pc = 0x802183ec;
            }
            0x802183ec => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80218400; f_801f025c(c);
                pc = 0x80218400;
            }
            0x80218400 => {
                c.r[4] = 0x18;
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x8021840c; f_802025b8(c);
                pc = 0x8021840c;
            }
            0x8021840c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x80218428; continue; }
                pc = 0x80218418;
            }
            0x80218418 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218468; continue;
                pc = 0x80218428;
            }
            0x80218428 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x18;
                c.r[5] = 0x0;
                c.lr = 0x80218438; f_8020248c(c);
                pc = 0x80218438;
            }
            0x80218438 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2;
                c.lr = 0x80218444; f_80203b5c(c);
                pc = 0x80218444;
            }
            0x80218444 => {
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.lr = 0x8021845c; f_8011bbd8(c);
                pc = 0x8021845c;
            }
            0x8021845c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218468;
            }
            0x80218468 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802183bc: bad pc {:#x}", pc),
        }
    }
}

/// fn_8021847C (0x8021847c, 936 bytes)
#[inline(never)]
pub fn f_8021847c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8021847c) { return; }
        if c.prof { crate::prof::enter(c, 0x8021847c); f_8021847c_b(c, 0x8021847c); crate::prof::exit(c, 0x8021847c); return; }
    }
    f_8021847c_b(c, 0x8021847c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8021847c_at(c: &mut Cpu, pc: u32) { f_8021847c_b(c, pc) }
#[inline(always)]
fn f_8021847c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8021847c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8021847c, pc); }
        match pc {
            0x8021847c => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80218498; f_801f025c(c);
                pc = 0x80218498;
            }
            0x80218498 => {
                c.r[25] = c.r[3] | c.r[3];
                c.lr = 0x802184a0; f_80205184(c);
                pc = 0x802184a0;
            }
            0x802184a0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0xe;
                c.lr = 0x802184b0; f_801f025c(c);
                pc = 0x802184b0;
            }
            0x802184b0 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0x2;
                c.lr = 0x802184c0; f_801f025c(c);
                pc = 0x802184c0;
            }
            0x802184c0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                c.lr = 0x802184dc; f_801f54a4(c);
                pc = 0x802184dc;
            }
            0x802184dc => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xd7);
                if !c.crbit(2) { pc = 0x802186b0; continue; }
                pc = 0x802184ec;
            }
            0x802184ec => {
                c.r[0] = 0x0;
                c.r[28] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[28].wrapping_add(0x5), c.r[0]);
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80218500; f_80207bf4(c);
                pc = 0x80218500;
            }
            0x80218500 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2b);
                if c.crbit(2) { pc = 0x8021855c; continue; }
                pc = 0x8021850c;
            }
            0x8021850c => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80218514; f_80205b8c(c);
                pc = 0x80218514;
            }
            0x80218514 => {
                c.lr = 0x80218518; f_801252e0(c);
                pc = 0x80218518;
            }
            0x80218518 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.lr = 0x80218524; f_80202998(c);
                pc = 0x80218524;
            }
            0x80218524 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x17;
                c.lr = 0x80218530; f_80202810(c);
                pc = 0x80218530;
            }
            0x80218530 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80218538; f_801fecd4(c);
                pc = 0x80218538;
            }
            0x80218538 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80218568; continue; }
                pc = 0x80218544;
            }
            0x80218544 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x7c;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80218558; f_801fe7ec(c);
                pc = 0x80218558;
            }
            0x80218558 => {
                pc = 0x80218568; continue;
                pc = 0x8021855c;
            }
            0x8021855c => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x5));
                c.r[0] = c.r[0] | 0x1;
                c.stb(c.r[28].wrapping_add(0x5), c.r[0]);
                pc = 0x80218568;
            }
            0x80218568 => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8021860c; continue; }
                pc = 0x80218570;
            }
            0x80218570 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80218578; f_80206780(c);
                pc = 0x80218578;
            }
            0x80218578 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021860c; continue; }
                pc = 0x80218584;
            }
            0x80218584 => {
                c.r[7] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x4b;
                c.r[6] = 0x0;
                c.lr = 0x8021859c; f_801f4c14(c);
                pc = 0x8021859c;
            }
            0x8021859c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802185a4; f_80207bf4(c);
                pc = 0x802185a4;
            }
            0x802185a4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2b);
                if c.crbit(2) { pc = 0x80218600; continue; }
                pc = 0x802185b0;
            }
            0x802185b0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802185b8; f_80205b8c(c);
                pc = 0x802185b8;
            }
            0x802185b8 => {
                c.lr = 0x802185bc; f_801252e0(c);
                pc = 0x802185bc;
            }
            0x802185bc => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x802185c8; f_80202998(c);
                pc = 0x802185c8;
            }
            0x802185c8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x17;
                c.lr = 0x802185d4; f_80202810(c);
                pc = 0x802185d4;
            }
            0x802185d4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802185dc; f_801fecd4(c);
                pc = 0x802185dc;
            }
            0x802185dc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021860c; continue; }
                pc = 0x802185e8;
            }
            0x802185e8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x7c;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802185fc; f_801fe7ec(c);
                pc = 0x802185fc;
            }
            0x802185fc => {
                pc = 0x8021860c; continue;
                pc = 0x80218600;
            }
            0x80218600 => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x5));
                c.r[0] = c.r[0] | 0x2;
                c.stb(c.r[28].wrapping_add(0x5), c.r[0]);
                pc = 0x8021860c;
            }
            0x8021860c => {
                c.r[26] = 0x0;
                pc = 0x802186a4; continue;
                pc = 0x80218614;
            }
            0x80218614 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[26].rotate_left(0) & 0xffff;
                c.lr = 0x80218620; f_801f7258(c);
                pc = 0x80218620;
            }
            0x80218620 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x802186a0; continue; }
                pc = 0x80218628;
            }
            0x80218628 => {
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x80218638; f_801fb1c0(c);
                pc = 0x80218638;
            }
            0x80218638 => {
                c.r[27] = 0x0;
                c.r[24] = c.r[3] | c.r[3];
                pc = 0x80218640;
            }
            0x80218640 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[5] = c.r[27].rotate_left(0) & 0xffff;
                c.r[4] = 0x3;
                c.lr = 0x80218650; f_8012a5b0(c);
                pc = 0x80218650;
            }
            0x80218650 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8021865c; f_801f9930(c);
                pc = 0x8021865c;
            }
            0x8021865c => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if c.crbit(2) { pc = 0x80218694; continue; }
                pc = 0x80218664;
            }
            0x80218664 => {
                c.lr = 0x80218668; f_80206a04(c);
                pc = 0x80218668;
            }
            0x80218668 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80218694; continue; }
                pc = 0x80218670;
            }
            0x80218670 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80218678; f_80205be8(c);
                pc = 0x80218678;
            }
            0x80218678 => {
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x80218680; f_801248c4(c);
                pc = 0x80218680;
            }
            0x80218680 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2b);
                if c.crbit(2) { pc = 0x80218694; continue; }
                pc = 0x8021868c;
            }
            0x8021868c => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80218694; f_801252e0(c);
                pc = 0x80218694;
            }
            0x80218694 => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.cmpu(0, c.r[27], 0x6);
                if c.crbit(0) { pc = 0x80218640; continue; }
                pc = 0x802186a0;
            }
            0x802186a0 => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x802186a4;
            }
            0x802186a4 => {
                c.cmpu(0, c.r[26], c.r[31]);
                if c.crbit(0) { pc = 0x80218614; continue; }
                pc = 0x802186ac;
            }
            0x802186ac => {
                pc = 0x80218804; continue;
                pc = 0x802186b0;
            }
            0x802186b0 => {
                c.r[0] = 0x4;
                c.r[3] = c.r[13].wrapping_add(0xffff8558);
                c.stb(c.r[3].wrapping_add(0x5), c.r[0]);
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x802186c4; f_80205b8c(c);
                pc = 0x802186c4;
            }
            0x802186c4 => {
                c.lr = 0x802186c8; f_801252e0(c);
                pc = 0x802186c8;
            }
            0x802186c8 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x17;
                c.lr = 0x802186d4; f_80202810(c);
                pc = 0x802186d4;
            }
            0x802186d4 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x802186dc; f_801fecd4(c);
                pc = 0x802186dc;
            }
            0x802186dc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802186fc; continue; }
                pc = 0x802186e8;
            }
            0x802186e8 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x7c;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x802186fc; f_801fe7ec(c);
                pc = 0x802186fc;
            }
            0x802186fc => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8021877c; continue; }
                pc = 0x80218704;
            }
            0x80218704 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8021870c; f_80206780(c);
                pc = 0x8021870c;
            }
            0x8021870c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021877c; continue; }
                pc = 0x80218718;
            }
            0x80218718 => {
                c.r[7] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x4b;
                c.r[6] = 0x0;
                c.lr = 0x80218730; f_801f4c14(c);
                pc = 0x80218730;
            }
            0x80218730 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80218738; f_80205b8c(c);
                pc = 0x80218738;
            }
            0x80218738 => {
                c.lr = 0x8021873c; f_801252e0(c);
                pc = 0x8021873c;
            }
            0x8021873c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x80218748; f_80202998(c);
                pc = 0x80218748;
            }
            0x80218748 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x17;
                c.lr = 0x80218754; f_80202810(c);
                pc = 0x80218754;
            }
            0x80218754 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8021875c; f_801fecd4(c);
                pc = 0x8021875c;
            }
            0x8021875c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021877c; continue; }
                pc = 0x80218768;
            }
            0x80218768 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x7c;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x8021877c; f_801fe7ec(c);
                pc = 0x8021877c;
            }
            0x8021877c => {
                c.r[28] = 0x0;
                pc = 0x802187fc; continue;
                pc = 0x80218784;
            }
            0x80218784 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.lr = 0x80218790; f_801f7258(c);
                pc = 0x80218790;
            }
            0x80218790 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x802187f8; continue; }
                pc = 0x80218798;
            }
            0x80218798 => {
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x802187a8; f_801fb1c0(c);
                pc = 0x802187a8;
            }
            0x802187a8 => {
                c.r[26] = 0x0;
                c.r[27] = c.r[3] | c.r[3];
                pc = 0x802187b0;
            }
            0x802187b0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[5] = c.r[26].rotate_left(0) & 0xffff;
                c.r[4] = 0x3;
                c.lr = 0x802187c0; f_8012a5b0(c);
                pc = 0x802187c0;
            }
            0x802187c0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802187cc; f_801f9930(c);
                pc = 0x802187cc;
            }
            0x802187cc => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if c.crbit(2) { pc = 0x802187ec; continue; }
                pc = 0x802187d4;
            }
            0x802187d4 => {
                c.lr = 0x802187d8; f_80206a04(c);
                pc = 0x802187d8;
            }
            0x802187d8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x802187ec; continue; }
                pc = 0x802187e0;
            }
            0x802187e0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x802187e8; f_80205be8(c);
                pc = 0x802187e8;
            }
            0x802187e8 => {
                c.lr = 0x802187ec; f_801252e0(c);
                pc = 0x802187ec;
            }
            0x802187ec => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                c.cmpu(0, c.r[26], 0x6);
                if c.crbit(0) { pc = 0x802187b0; continue; }
                pc = 0x802187f8;
            }
            0x802187f8 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x802187fc;
            }
            0x802187fc => {
                c.cmpu(0, c.r[28], c.r[31]);
                if c.crbit(0) { pc = 0x80218784; continue; }
                pc = 0x80218804;
            }
            0x80218804 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8021847c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80218824 (0x80218824, 584 bytes)
#[inline(never)]
pub fn f_80218824(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80218824) { return; }
        if c.prof { crate::prof::enter(c, 0x80218824); f_80218824_b(c, 0x80218824); crate::prof::exit(c, 0x80218824); return; }
    }
    f_80218824_b(c, 0x80218824)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80218824_at(c: &mut Cpu, pc: u32) { f_80218824_b(c, pc) }
#[inline(always)]
fn f_80218824_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80218824;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80218824, pc); }
        match pc {
            0x80218824 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80218840; f_801f025c(c);
                pc = 0x80218840;
            }
            0x80218840 => {
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.lr = 0x8021884c; f_801f025c(c);
                pc = 0x8021884c;
            }
            0x8021884c => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80218854; f_80205b8c(c);
                pc = 0x80218854;
            }
            0x80218854 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xf0;
                c.r[6] = 0x0;
                c.lr = 0x80218870; f_8012640c(c);
                pc = 0x80218870;
            }
            0x80218870 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x80218a4c; continue; }
                pc = 0x80218878;
            }
            0x80218878 => {
                c.cmpu(0, c.r[29], 0x165);
                if c.crbit(2) { pc = 0x80218a4c; continue; }
                pc = 0x80218880;
            }
            0x80218880 => {
                c.cmpu(0, c.r[29], 0xffff);
                if c.crbit(2) { pc = 0x80218a4c; continue; }
                pc = 0x80218888;
            }
            0x80218888 => {
                c.cmpu(0, c.r[29], 0x164);
                if c.crbit(2) { pc = 0x80218a4c; continue; }
                pc = 0x80218890;
            }
            0x80218890 => {
                c.r[27] = 0x0;
                c.r[28] = 0x0;
                pc = 0x802188f4; continue;
                pc = 0x8021889c;
            }
            0x8021889c => {
                c.r[26] = c.r[28].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x802188ac; f_80123cd4(c);
                pc = 0x802188ac;
            }
            0x802188ac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x802188f0; continue; }
                pc = 0x802188b4;
            }
            0x802188b4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x802188c8; f_8012640c(c);
                pc = 0x802188c8;
            }
            0x802188c8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[29], c.r[0]);
                if !c.crbit(2) { pc = 0x802188f0; continue; }
                pc = 0x802188d4;
            }
            0x802188d4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x80;
                c.lr = 0x802188e8; f_8012640c(c);
                pc = 0x802188e8;
            }
            0x802188e8 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80218900; continue;
                pc = 0x802188f0;
            }
            0x802188f0 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x802188f4;
            }
            0x802188f4 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8021889c; continue; }
                pc = 0x80218900;
            }
            0x80218900 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8021891c; continue; }
                pc = 0x8021890c;
            }
            0x8021890c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218a58; continue;
                pc = 0x8021891c;
            }
            0x8021891c => {
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(0) { pc = 0x80218938; continue; }
                pc = 0x80218928;
            }
            0x80218928 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218a58; continue;
                pc = 0x80218938;
            }
            0x80218938 => {
                c.lr = 0x8021893c; f_800e0c54(c);
                pc = 0x8021893c;
            }
            0x8021893c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].rotate_left(30) & 0xc0000000;
                c.r[4] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                c.r[3] = c.r[3].wrapping_sub(c.r[4]);
                c.r[3] = c.r[3].rotate_left(2) & 0xffffffff;
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.r[3] = c.r[3].wrapping_add(0x2);
                c.r[26] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], c.r[26]);
                if !c.crbit(0) { pc = 0x8021896c; continue; }
                pc = 0x80218968;
            }
            0x80218968 => {
                c.r[26] = c.r[27] | c.r[27];
                pc = 0x8021896c;
            }
            0x8021896c => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80218980; f_8011beb4(c);
                pc = 0x80218980;
            }
            0x80218980 => {
                c.lr = 0x80218984; f_800fa280(c);
                pc = 0x80218984;
            }
            0x80218984 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80218990; f_80132a38(c);
                pc = 0x80218990;
            }
            0x80218990 => {
                c.r[4] = c.r[26].rotate_left(0) & 0xff;
                c.r[3] = 0x2f;
                c.lr = 0x8021899c; f_80132a38(c);
                pc = 0x8021899c;
            }
            0x8021899c => {
                c.r[27] = c.r[27].wrapping_sub(c.r[26]);
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[28].rotate_left(0) & 0xff;
                c.r[4] = 0x0;
                c.r[7] = c.r[27].rotate_left(0) & 0xff;
                c.r[5] = 0x80;
                c.lr = 0x802189b8; f_801254b4(c);
                pc = 0x802189b8;
            }
            0x802189b8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x10;
                c.lr = 0x802189c4; f_802026e4(c);
                pc = 0x802189c4;
            }
            0x802189c4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80218a2c; continue; }
                pc = 0x802189cc;
            }
            0x802189cc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x31;
                c.lr = 0x802189d8; f_802026e4(c);
                pc = 0x802189d8;
            }
            0x802189d8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80218a2c; continue; }
                pc = 0x802189e4;
            }
            0x802189e4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x31;
                c.lr = 0x802189f0; f_80201890(c);
                pc = 0x802189f0;
            }
            0x802189f0 => {
                c.r[26] = c.r[28].rotate_left(0) & 0xff;
                c.r[0] = 0x1;
                c.r[0] = slw(c.r[0], c.r[26]);
                c.r[0] = c.r[3] & c.r[0];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80218a2c; continue; }
                pc = 0x80218a04;
            }
            0x80218a04 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80218a0c; f_801fecd4(c);
                pc = 0x80218a0c;
            }
            0x80218a0c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80218a2c; continue; }
                pc = 0x80218a18;
            }
            0x80218a18 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[26] | c.r[26];
                c.r[4] = 0x80;
                c.r[6] = 0x0;
                c.lr = 0x80218a2c; f_801fe7ec(c);
                pc = 0x80218a2c;
            }
            0x80218a2c => {
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80218a3c; continue; }
                pc = 0x80218a34;
            }
            0x80218a34 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80218a3c; f_80200b10(c);
                pc = 0x80218a3c;
            }
            0x80218a3c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218a58; continue;
                pc = 0x80218a4c;
            }
            0x80218a4c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218a58;
            }
            0x80218a58 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80218824: bad pc {:#x}", pc),
        }
    }
}

/// fn_80218A6C (0x80218a6c, 256 bytes)
#[inline(never)]
pub fn f_80218a6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80218a6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80218a6c); f_80218a6c_b(c, 0x80218a6c); crate::prof::exit(c, 0x80218a6c); return; }
    }
    f_80218a6c_b(c, 0x80218a6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80218a6c_at(c: &mut Cpu, pc: u32) { f_80218a6c_b(c, pc) }
#[inline(always)]
fn f_80218a6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80218a6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80218a6c, pc); }
        match pc {
            0x80218a6c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80218a88; f_801f025c(c);
                pc = 0x80218a88;
            }
            0x80218a88 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80218a9c; f_8012640c(c);
                pc = 0x80218a9c;
            }
            0x80218a9c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.r[0] | c.r[0];
                c.lr = 0x80218aac; f_80205b8c(c);
                pc = 0x80218aac;
            }
            0x80218aac => {
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x80218ac0; f_8012640c(c);
                pc = 0x80218ac0;
            }
            0x80218ac0 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x80218ad8; f_8012640c(c);
                pc = 0x80218ad8;
            }
            0x80218ad8 => {
                c.r[0] = c.r[29].wrapping_mul(0x30);
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = divw(c.r[0], c.r[3]);
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[4]);
                if !c.crbit(2) { pc = 0x80218af8; continue; }
                pc = 0x80218aec;
            }
            0x80218aec => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x80218af8; continue; }
                pc = 0x80218af4;
            }
            0x80218af4 => {
                c.r[4] = 0x1;
                pc = 0x80218af8;
            }
            0x80218af8 => {
                c.r[3] = 0x80280000;
                c.r[0] = 0x6;
                c.r[3] = c.r[3].wrapping_add(0xffff9f7c);
                c.r[4] = c.r[4].rotate_left(0) & 0xff;
                c.r[5] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x80218b10;
            }
            0x80218b10 => {
                c.r[0] = c.lbz(c.r[3]);
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(1) { pc = 0x80218b28; continue; }
                pc = 0x80218b1c;
            }
            0x80218b1c => {
                c.r[5] = c.r[5].wrapping_add(0x2);
                c.r[3] = c.r[3].wrapping_add(0x2);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80218b10; continue; }
                pc = 0x80218b28;
            }
            0x80218b28 => {
                c.r[4] = 0x80280000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.r[4].wrapping_add(0xffff9f7c);
                c.r[6] = c.r[0].wrapping_add(c.r[5]);
                c.r[4] = 0x0;
                c.r[7] = c.lbz(c.r[6].wrapping_add(0x1));
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80218b4c; f_8011bbd8(c);
                pc = 0x80218b4c;
            }
            0x80218b4c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80218a6c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80218B6C (0x80218b6c, 104 bytes)
#[inline(never)]
pub fn f_80218b6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80218b6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80218b6c); f_80218b6c_b(c, 0x80218b6c); crate::prof::exit(c, 0x80218b6c); return; }
    }
    f_80218b6c_b(c, 0x80218b6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80218b6c_at(c: &mut Cpu, pc: u32) { f_80218b6c_b(c, pc) }
#[inline(always)]
fn f_80218b6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80218b6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80218b6c, pc); }
        match pc {
            0x80218b6c => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.r[3] = c.r[5].wrapping_mul(0x30);
                c.r[0] = divw(c.r[3], c.r[0]);
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[4]);
                if !c.crbit(2) { pc = 0x80218b90; continue; }
                pc = 0x80218b84;
            }
            0x80218b84 => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x80218b90; continue; }
                pc = 0x80218b8c;
            }
            0x80218b8c => {
                c.r[4] = 0x1;
                pc = 0x80218b90;
            }
            0x80218b90 => {
                c.r[3] = 0x80280000;
                c.r[0] = 0x6;
                c.r[3] = c.r[3].wrapping_add(0xffff9f7c);
                c.r[4] = c.r[4].rotate_left(0) & 0xff;
                c.r[5] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x80218ba8;
            }
            0x80218ba8 => {
                c.r[0] = c.lbz(c.r[3]);
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(1) { pc = 0x80218bc0; continue; }
                pc = 0x80218bb4;
            }
            0x80218bb4 => {
                c.r[5] = c.r[5].wrapping_add(0x2);
                c.r[3] = c.r[3].wrapping_add(0x2);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80218ba8; continue; }
                pc = 0x80218bc0;
            }
            0x80218bc0 => {
                c.r[3] = 0x80280000;
                c.r[0] = c.r[3].wrapping_add(0xffff9f7c);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_80218b6c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80218BD4 (0x80218bd4, 160 bytes)
#[inline(never)]
pub fn f_80218bd4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80218bd4) { return; }
        if c.prof { crate::prof::enter(c, 0x80218bd4); f_80218bd4_b(c, 0x80218bd4); crate::prof::exit(c, 0x80218bd4); return; }
    }
    f_80218bd4_b(c, 0x80218bd4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80218bd4_at(c: &mut Cpu, pc: u32) { f_80218bd4_b(c, pc) }
#[inline(always)]
fn f_80218bd4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80218bd4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80218bd4, pc); }
        match pc {
            0x80218bd4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80218bf0; f_801f025c(c);
                pc = 0x80218bf0;
            }
            0x80218bf0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.lr = 0x80218bfc; f_801f025c(c);
                pc = 0x80218bfc;
            }
            0x80218bfc => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[4] = 0x0;
                c.lr = 0x80218c0c; f_801f025c(c);
                pc = 0x80218c0c;
            }
            0x80218c0c => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80218c1c; f_801f025c(c);
                pc = 0x80218c1c;
            }
            0x80218c1c => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x15;
                c.lr = 0x80218c2c; f_802026e4(c);
                pc = 0x80218c2c;
            }
            0x80218c2c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80218c54; continue; }
                pc = 0x80218c38;
            }
            0x80218c38 => {
                c.cmpu(0, c.r[30], c.r[31]);
                if c.crbit(2) { pc = 0x80218c54; continue; }
                pc = 0x80218c40;
            }
            0x80218c40 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[0] = c.r[3].rotate_left(0) & 0x1000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80218c54; continue; }
                pc = 0x80218c4c;
            }
            0x80218c4c => {
                c.r[0] = c.r[3] | 0x40;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                pc = 0x80218c54;
            }
            0x80218c54 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80218bd4: bad pc {:#x}", pc),
        }
    }
}

/// WS_MICHIDURE (0x80218c74, 176 bytes)
#[inline(never)]
pub fn f_80218c74(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80218c74) { return; }
        if c.prof { crate::prof::enter(c, 0x80218c74); f_80218c74_b(c, 0x80218c74); crate::prof::exit(c, 0x80218c74); return; }
    }
    f_80218c74_b(c, 0x80218c74)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80218c74_at(c: &mut Cpu, pc: u32) { f_80218c74_b(c, pc) }
#[inline(always)]
fn f_80218c74_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80218c74;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80218c74, pc); }
        match pc {
            0x80218c74 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80218c90; f_801f025c(c);
                pc = 0x80218c90;
            }
            0x80218c90 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80218c98; f_80205184(c);
                pc = 0x80218c98;
            }
            0x80218c98 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80218cac; f_801f025c(c);
                pc = 0x80218cac;
            }
            0x80218cac => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80218cc0; f_80229934(c);
                pc = 0x80218cc0;
            }
            0x80218cc0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80218cdc; continue; }
                pc = 0x80218ccc;
            }
            0x80218ccc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218d10; continue;
                pc = 0x80218cdc;
            }
            0x80218cdc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x15;
                c.lr = 0x80218ce8; f_802025b8(c);
                pc = 0x80218ce8;
            }
            0x80218ce8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80218d04; continue; }
                pc = 0x80218cf4;
            }
            0x80218cf4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x15;
                c.r[5] = 0x0;
                c.lr = 0x80218d04; f_8020248c(c);
                pc = 0x80218d04;
            }
            0x80218d04 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218d10;
            }
            0x80218d10 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80218c74: bad pc {:#x}", pc),
        }
    }
}

/// fn_80218D24 (0x80218d24, 696 bytes)
#[inline(never)]
pub fn f_80218d24(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80218d24) { return; }
        if c.prof { crate::prof::enter(c, 0x80218d24); f_80218d24_b(c, 0x80218d24); crate::prof::exit(c, 0x80218d24); return; }
    }
    f_80218d24_b(c, 0x80218d24)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80218d24_at(c: &mut Cpu, pc: u32) { f_80218d24_b(c, pc) }
#[inline(always)]
fn f_80218d24_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80218d24;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80218d24, pc); }
        match pc {
            0x80218d24 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80218d48; f_801f54a4(c);
                pc = 0x80218d48;
            }
            0x80218d48 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80218d58; f_801f025c(c);
                pc = 0x80218d58;
            }
            0x80218d58 => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x80218d60; f_80205b8c(c);
                pc = 0x80218d60;
            }
            0x80218d60 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[26] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x80218d7c; f_8012640c(c);
                pc = 0x80218d7c;
            }
            0x80218d7c => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x0;
                c.r[3] = 0xffffffff;
                pc = 0x80218d9c; continue;
                pc = 0x80218d90;
            }
            0x80218d90 => {
                c.r[0] = c.r[5].rotate_left(1) & 0x1fe;
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.sth(c.r[4].wrapping_add(c.r[0]), c.r[3]);
                pc = 0x80218d9c;
            }
            0x80218d9c => {
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x80218d90; continue; }
                pc = 0x80218da8;
            }
            0x80218da8 => {
                c.r[29] = 0x0;
                c.r[28] = 0x0;
                pc = 0x80218edc; continue;
                pc = 0x80218db4;
            }
            0x80218db4 => {
                c.r[25] = c.r[28].rotate_left(0) & 0xffff;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[25] | c.r[25];
                c.lr = 0x80218dc4; f_80123cd4(c);
                pc = 0x80218dc4;
            }
            0x80218dc4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80218ed8; continue; }
                pc = 0x80218dcc;
            }
            0x80218dcc => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[25] | c.r[25];
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80218de0; f_801ffec8(c);
                pc = 0x80218de0;
            }
            0x80218de0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80218df0; continue; }
                pc = 0x80218de8;
            }
            0x80218de8 => {
                c.cmpu(0, c.r[0], 0x6);
                if !c.crbit(2) { pc = 0x80218ed8; continue; }
                pc = 0x80218df0;
            }
            0x80218df0 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x80218e04; f_8012640c(c);
                pc = 0x80218e04;
            }
            0x80218e04 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[4]);
                if c.crbit(2) { pc = 0x80218ed8; continue; }
                pc = 0x80218e0c;
            }
            0x80218e0c => {
                c.cmpu(0, c.r[4], 0x165);
                if c.crbit(2) { pc = 0x80218ed8; continue; }
                pc = 0x80218e14;
            }
            0x80218e14 => {
                c.cmpu(0, c.r[4], 0x163);
                if c.crbit(2) { pc = 0x80218ed8; continue; }
                pc = 0x80218e1c;
            }
            0x80218e1c => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x80218e4c; continue; }
                pc = 0x80218e24;
            }
            0x80218e24 => {
                c.cmpu(0, c.r[4], 0x165);
                if c.crbit(2) { pc = 0x80218e4c; continue; }
                pc = 0x80218e2c;
            }
            0x80218e2c => {
                c.cmpu(0, c.r[4], 0xd6);
                if c.crbit(2) { pc = 0x80218e4c; continue; }
                pc = 0x80218e34;
            }
            0x80218e34 => {
                c.cmpu(0, c.r[4], 0x112);
                if c.crbit(2) { pc = 0x80218e4c; continue; }
                pc = 0x80218e3c;
            }
            0x80218e3c => {
                c.cmpu(0, c.r[4], 0x77);
                if c.crbit(2) { pc = 0x80218e4c; continue; }
                pc = 0x80218e44;
            }
            0x80218e44 => {
                c.cmpu(0, c.r[4], 0x76);
                if !c.crbit(2) { pc = 0x80218e54; continue; }
                pc = 0x80218e4c;
            }
            0x80218e4c => {
                c.r[0] = 0x1;
                pc = 0x80218e58; continue;
                pc = 0x80218e54;
            }
            0x80218e54 => {
                c.r[0] = 0x0;
                pc = 0x80218e58;
            }
            0x80218e58 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80218ed8; continue; }
                pc = 0x80218e60;
            }
            0x80218e60 => {
                c.cmpu(0, c.r[4], 0x108);
                if c.crbit(2) { pc = 0x80218ed8; continue; }
                pc = 0x80218e68;
            }
            0x80218e68 => {
                c.cmpu(0, c.r[4], 0xfd);
                if c.crbit(2) { pc = 0x80218ed8; continue; }
                pc = 0x80218e70;
            }
            0x80218e70 => {
                c.r[3] = 0x0;
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x80218e80; f_8011beb4(c);
                pc = 0x80218e80;
            }
            0x80218e80 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x91);
                if c.crbit(2) { pc = 0x80218eb4; continue; }
                pc = 0x80218e8c;
            }
            0x80218e8c => {
                c.cmpu(0, c.r[0], 0x27);
                if c.crbit(2) { pc = 0x80218eb4; continue; }
                pc = 0x80218e94;
            }
            0x80218e94 => {
                c.cmpu(0, c.r[0], 0x4b);
                if c.crbit(2) { pc = 0x80218eb4; continue; }
                pc = 0x80218e9c;
            }
            0x80218e9c => {
                c.cmpu(0, c.r[0], 0x97);
                if c.crbit(2) { pc = 0x80218eb4; continue; }
                pc = 0x80218ea4;
            }
            0x80218ea4 => {
                c.cmpu(0, c.r[0], 0x9b);
                if c.crbit(2) { pc = 0x80218eb4; continue; }
                pc = 0x80218eac;
            }
            0x80218eac => {
                c.cmpu(0, c.r[0], 0x1a);
                if !c.crbit(2) { pc = 0x80218ebc; continue; }
                pc = 0x80218eb4;
            }
            0x80218eb4 => {
                c.r[0] = 0x1;
                pc = 0x80218ec0; continue;
                pc = 0x80218ebc;
            }
            0x80218ebc => {
                c.r[0] = 0x0;
                pc = 0x80218ec0;
            }
            0x80218ec0 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80218ed8; continue; }
                pc = 0x80218ec8;
            }
            0x80218ec8 => {
                c.r[0] = c.r[29].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.sth(c.r[3].wrapping_add(c.r[0]), c.r[28]);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x80218ed8;
            }
            0x80218ed8 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x80218edc;
            }
            0x80218edc => {
                c.r[0] = c.r[28] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x80218db4; continue; }
                pc = 0x80218ee8;
            }
            0x80218ee8 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80218f00; continue; }
                pc = 0x80218ef0;
            }
            0x80218ef0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218fc8; continue;
                pc = 0x80218f00;
            }
            0x80218f00 => {
                c.lr = 0x80218f04; f_800e0c54(c);
                pc = 0x80218f04;
            }
            0x80218f04 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                c.r[0] = divw(c.r[5], c.r[4]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.r[0].wrapping_mul(c.r[4]);
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                c.r[0] = c.r[0].rotate_left(1) & 0x1fe;
                c.r[25] = c.lha(c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80218ef0; continue; }
                pc = 0x80218f2c;
            }
            0x80218f2c => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[25].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x80218f40; f_8012640c(c);
                pc = 0x80218f40;
            }
            0x80218f40 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80218ef0; continue; }
                pc = 0x80218f48;
            }
            0x80218f48 => {
                c.cmpu(0, c.r[0], 0x165);
                if c.crbit(2) { pc = 0x80218ef0; continue; }
                pc = 0x80218f50;
            }
            0x80218f50 => {
                c.cmpu(0, c.r[0], 0x163);
                if c.crbit(2) { pc = 0x80218ef0; continue; }
                pc = 0x80218f58;
            }
            0x80218f58 => {
                c.sth(c.r[13].wrapping_add(0xffffadec), c.r[0]);
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x26;
                c.r[6] = 0x0;
                c.lr = 0x80218f74; f_8011bbd8(c);
                pc = 0x80218f74;
            }
            0x80218f74 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffadf8));
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.lhz(c.r[13].wrapping_add(0xffffadec));
                c.r[5] = c.r[31] | c.r[31];
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffbff;
                c.r[6] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffadf8), c.r[0]);
                c.r[7] = 0x1;
                c.r[8] = 0x1;
                c.r[9] = 0xffffffff;
                c.lr = 0x80218fa0; f_8022b2cc(c);
                pc = 0x80218fa0;
            }
            0x80218fa0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x80218fbc; f_801f4c14(c);
                pc = 0x80218fbc;
            }
            0x80218fbc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x80218fc8;
            }
            0x80218fc8 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80218d24: bad pc {:#x}", pc),
        }
    }
}

/// fightSeqRendouWazaCheck (0x80218fdc, 64 bytes)
#[inline(never)]
pub fn f_80218fdc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80218fdc) { return; }
        if c.prof { crate::prof::enter(c, 0x80218fdc); f_80218fdc_b(c, 0x80218fdc); crate::prof::exit(c, 0x80218fdc); return; }
    }
    f_80218fdc_b(c, 0x80218fdc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80218fdc_at(c: &mut Cpu, pc: u32) { f_80218fdc_b(c, pc) }
#[inline(always)]
fn f_80218fdc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80218fdc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80218fdc, pc); }
        match pc {
            0x80218fdc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8021900c; continue; }
                pc = 0x80218fe4;
            }
            0x80218fe4 => {
                c.cmpu(0, c.r[0], 0x165);
                if c.crbit(2) { pc = 0x8021900c; continue; }
                pc = 0x80218fec;
            }
            0x80218fec => {
                c.cmpu(0, c.r[0], 0xd6);
                if c.crbit(2) { pc = 0x8021900c; continue; }
                pc = 0x80218ff4;
            }
            0x80218ff4 => {
                c.cmpu(0, c.r[0], 0x112);
                if c.crbit(2) { pc = 0x8021900c; continue; }
                pc = 0x80218ffc;
            }
            0x80218ffc => {
                c.cmpu(0, c.r[0], 0x77);
                if c.crbit(2) { pc = 0x8021900c; continue; }
                pc = 0x80219004;
            }
            0x80219004 => {
                c.cmpu(0, c.r[0], 0x76);
                if !c.crbit(2) { pc = 0x80219014; continue; }
                pc = 0x8021900c;
            }
            0x8021900c => {
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x80219014;
            }
            0x80219014 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80218fdc: bad pc {:#x}", pc),
        }
    }
}

/// fn_8021901C (0x8021901c, 112 bytes)
#[inline(never)]
pub fn f_8021901c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8021901c) { return; }
        if c.prof { crate::prof::enter(c, 0x8021901c); f_8021901c_b(c, 0x8021901c); crate::prof::exit(c, 0x8021901c); return; }
    }
    f_8021901c_b(c, 0x8021901c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8021901c_at(c: &mut Cpu, pc: u32) { f_8021901c_b(c, pc) }
#[inline(always)]
fn f_8021901c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8021901c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8021901c, pc); }
        match pc {
            0x8021901c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x8021903c; f_8011beb4(c);
                pc = 0x8021903c;
            }
            0x8021903c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x91);
                if c.crbit(2) { pc = 0x80219070; continue; }
                pc = 0x80219048;
            }
            0x80219048 => {
                c.cmpu(0, c.r[0], 0x27);
                if c.crbit(2) { pc = 0x80219070; continue; }
                pc = 0x80219050;
            }
            0x80219050 => {
                c.cmpu(0, c.r[0], 0x4b);
                if c.crbit(2) { pc = 0x80219070; continue; }
                pc = 0x80219058;
            }
            0x80219058 => {
                c.cmpu(0, c.r[0], 0x97);
                if c.crbit(2) { pc = 0x80219070; continue; }
                pc = 0x80219060;
            }
            0x80219060 => {
                c.cmpu(0, c.r[0], 0x9b);
                if c.crbit(2) { pc = 0x80219070; continue; }
                pc = 0x80219068;
            }
            0x80219068 => {
                c.cmpu(0, c.r[0], 0x1a);
                if !c.crbit(2) { pc = 0x80219078; continue; }
                pc = 0x80219070;
            }
            0x80219070 => {
                c.r[3] = 0x1;
                pc = 0x8021907c; continue;
                pc = 0x80219078;
            }
            0x80219078 => {
                c.r[3] = 0x0;
                pc = 0x8021907c;
            }
            0x8021907c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8021901c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8021908C (0x8021908c, 484 bytes)
#[inline(never)]
pub fn f_8021908c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8021908c) { return; }
        if c.prof { crate::prof::enter(c, 0x8021908c); f_8021908c_b(c, 0x8021908c); crate::prof::exit(c, 0x8021908c); return; }
    }
    f_8021908c_b(c, 0x8021908c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8021908c_at(c: &mut Cpu, pc: u32) { f_8021908c_b(c, pc) }
#[inline(always)]
fn f_8021908c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8021908c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8021908c, pc); }
        match pc {
            0x8021908c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802190a8; f_801f025c(c);
                pc = 0x802190a8;
            }
            0x802190a8 => {
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x802190bc; f_8012640c(c);
                pc = 0x802190bc;
            }
            0x802190bc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[29] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x802190d0; f_801f025c(c);
                pc = 0x802190d0;
            }
            0x802190d0 => {
                c.r[4] = 0x0;
                c.r[5] = 0xef;
                c.r[6] = 0x0;
                c.lr = 0x802190e0; f_8012640c(c);
                pc = 0x802190e0;
            }
            0x802190e0 => {
                c.r[5] = 0x10000;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[7] = c.r[5].wrapping_add(0xffffffff);
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x80219100; f_8011bbd8(c);
                pc = 0x80219100;
            }
            0x80219100 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x10;
                c.lr = 0x8021910c; f_802026e4(c);
                pc = 0x8021910c;
            }
            0x8021910c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80219160; continue; }
                pc = 0x80219118;
            }
            0x80219118 => {
                c.cmpu(0, c.r[31], 0x164);
                if c.crbit(2) { pc = 0x80219148; continue; }
                pc = 0x80219120;
            }
            0x80219120 => {
                c.r[0] = c.r[31].wrapping_add(0xffffff5b);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x80219148; continue; }
                pc = 0x80219130;
            }
            0x80219130 => {
                c.cmpu(0, c.r[31], 0xffff);
                if c.crbit(2) { pc = 0x80219148; continue; }
                pc = 0x80219138;
            }
            0x80219138 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80219148; continue; }
                pc = 0x80219140;
            }
            0x80219140 => {
                c.cmpu(0, c.r[31], 0x165);
                if !c.crbit(2) { pc = 0x80219150; continue; }
                pc = 0x80219148;
            }
            0x80219148 => {
                c.r[0] = 0x1;
                pc = 0x80219154; continue;
                pc = 0x80219150;
            }
            0x80219150 => {
                c.r[0] = 0x0;
                pc = 0x80219154;
            }
            0x80219154 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80219170; continue; }
                pc = 0x80219160;
            }
            0x80219160 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021925c; continue;
                pc = 0x80219170;
            }
            0x80219170 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xd5;
                c.r[6] = 0x0;
                c.lr = 0x80219184; f_8012640c(c);
                pc = 0x80219184;
            }
            0x80219184 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[28] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xd6;
                c.r[6] = 0x0;
                c.lr = 0x802191a0; f_8012640c(c);
                pc = 0x802191a0;
            }
            0x802191a0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[28] = c.r[0] | c.r[0];
                c.lr = 0x802191b0; f_80205be8(c);
                pc = 0x802191b0;
            }
            0x802191b0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[28] = c.r[0] | c.r[0];
                c.lr = 0x802191c0; f_80205be8(c);
                pc = 0x802191c0;
            }
            0x802191c0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[27] = c.r[0] | c.r[0];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x802191d4; f_80123b5c(c);
                pc = 0x802191d4;
            }
            0x802191d4 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80219160; continue; }
                pc = 0x802191dc;
            }
            0x802191dc => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x26;
                c.r[6] = 0x0;
                c.lr = 0x802191f0; f_8011beb4(c);
                pc = 0x802191f0;
            }
            0x802191f0 => {
                c.r[29] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[29]);
                if c.crbit(0) { pc = 0x80219160; continue; }
                pc = 0x802191f8;
            }
            0x802191f8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80219208; f_80123d58(c);
                pc = 0x80219208;
            }
            0x80219208 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80219210; f_801fecd4(c);
                pc = 0x80219210;
            }
            0x80219210 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8021922c; continue; }
                pc = 0x8021921c;
            }
            0x8021921c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8021922c; f_80123d58(c);
                pc = 0x8021922c;
            }
            0x8021922c => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80219240; f_8011beb4(c);
                pc = 0x80219240;
            }
            0x80219240 => {
                c.lr = 0x80219244; f_800fa280(c);
                pc = 0x80219244;
            }
            0x80219244 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80219250; f_80132a38(c);
                pc = 0x80219250;
            }
            0x80219250 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021925c;
            }
            0x8021925c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8021908c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80219270 (0x80219270, 68 bytes)
#[inline(never)]
pub fn f_80219270(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80219270) { return; }
        if c.prof { crate::prof::enter(c, 0x80219270); f_80219270_b(c, 0x80219270); crate::prof::exit(c, 0x80219270); return; }
    }
    f_80219270_b(c, 0x80219270)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80219270_at(c: &mut Cpu, pc: u32) { f_80219270_b(c, pc) }
#[inline(always)]
fn f_80219270_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80219270;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80219270, pc); }
        match pc {
            0x80219270 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[4], 0x164);
                if c.crbit(2) { pc = 0x802192a4; continue; }
                pc = 0x8021927c;
            }
            0x8021927c => {
                c.r[0] = c.r[3].wrapping_add(0xffffff5b);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x802192a4; continue; }
                pc = 0x8021928c;
            }
            0x8021928c => {
                c.cmpu(0, c.r[4], 0xffff);
                if c.crbit(2) { pc = 0x802192a4; continue; }
                pc = 0x80219294;
            }
            0x80219294 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x802192a4; continue; }
                pc = 0x8021929c;
            }
            0x8021929c => {
                c.cmpu(0, c.r[4], 0x165);
                if !c.crbit(2) { pc = 0x802192ac; continue; }
                pc = 0x802192a4;
            }
            0x802192a4 => {
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x802192ac;
            }
            0x802192ac => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80219270: bad pc {:#x}", pc),
        }
    }
}

/// fn_802192B4 (0x802192b4, 160 bytes)
#[inline(never)]
pub fn f_802192b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802192b4) { return; }
        if c.prof { crate::prof::enter(c, 0x802192b4); f_802192b4_b(c, 0x802192b4); crate::prof::exit(c, 0x802192b4); return; }
    }
    f_802192b4_b(c, 0x802192b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802192b4_at(c: &mut Cpu, pc: u32) { f_802192b4_b(c, pc) }
#[inline(always)]
fn f_802192b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802192b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802192b4, pc); }
        match pc {
            0x802192b4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x802192d0; f_801f025c(c);
                pc = 0x802192d0;
            }
            0x802192d0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x802192ec; f_801f54a4(c);
                pc = 0x802192ec;
            }
            0x802192ec => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x802192f8; f_801f0134(c);
                pc = 0x802192f8;
            }
            0x802192f8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x12;
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x8021930c; f_801f025c(c);
                pc = 0x8021930c;
            }
            0x8021930c => {
                c.r[4] = 0x1d;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80219318; f_802025b8(c);
                pc = 0x80219318;
            }
            0x80219318 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80219334; continue; }
                pc = 0x80219324;
            }
            0x80219324 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = 0x1d;
                c.lr = 0x80219334; f_8020248c(c);
                pc = 0x80219334;
            }
            0x80219334 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_802192b4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80219354 (0x80219354, 588 bytes)
#[inline(never)]
pub fn f_80219354(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80219354) { return; }
        if c.prof { crate::prof::enter(c, 0x80219354); f_80219354_b(c, 0x80219354); crate::prof::exit(c, 0x80219354); return; }
    }
    f_80219354_b(c, 0x80219354)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80219354_at(c: &mut Cpu, pc: u32) { f_80219354_b(c, pc) }
#[inline(always)]
fn f_80219354_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80219354;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80219354, pc); }
        match pc {
            0x80219354 => {
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x50); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x80219378; f_801f54a4(c);
                pc = 0x80219378;
            }
            0x80219378 => {
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x11;
                c.r[4] = 0x0;
                c.lr = 0x80219388; f_801f025c(c);
                pc = 0x80219388;
            }
            0x80219388 => {
                c.r[4] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8021939c; f_8012640c(c);
                pc = 0x8021939c;
            }
            0x8021939c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xf3;
                c.r[6] = 0x0;
                c.lr = 0x802193b0; f_8012640c(c);
                pc = 0x802193b0;
            }
            0x802193b0 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x102;
                c.r[6] = 0x0;
                c.lr = 0x802193c8; f_8012640c(c);
                pc = 0x802193c8;
            }
            0x802193c8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x104;
                c.r[6] = 0x0;
                c.lr = 0x802193dc; f_8012640c(c);
                pc = 0x802193dc;
            }
            0x802193dc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xf4;
                c.r[6] = 0x0;
                c.lr = 0x802193f0; f_8012640c(c);
                pc = 0x802193f0;
            }
            0x802193f0 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xf2;
                c.r[6] = 0x0;
                c.lr = 0x80219408; f_8012640c(c);
                pc = 0x80219408;
            }
            0x80219408 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80219414; f_801f00d0(c);
                pc = 0x80219414;
            }
            0x80219414 => {
                c.cmpu(0, c.r[29], 0x0);
                c.r[30] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x80219438; continue; }
                pc = 0x80219420;
            }
            0x80219420 => {
                c.cmpu(0, c.r[29], 0x165);
                if c.crbit(2) { pc = 0x80219438; continue; }
                pc = 0x80219428;
            }
            0x80219428 => {
                c.cmpu(0, c.r[29], 0xffff);
                if c.crbit(2) { pc = 0x80219438; continue; }
                pc = 0x80219430;
            }
            0x80219430 => {
                c.cmpu(0, c.r[29], 0x164);
                if !c.crbit(2) { pc = 0x80219448; continue; }
                pc = 0x80219438;
            }
            0x80219438 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021958c; continue;
                pc = 0x80219448;
            }
            0x80219448 => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x802194d4; continue; }
                pc = 0x80219450;
            }
            0x80219450 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.lr = 0x80219464; f_8011beb4(c);
                pc = 0x80219464;
            }
            0x80219464 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x91);
                if c.crbit(2) { pc = 0x80219498; continue; }
                pc = 0x80219470;
            }
            0x80219470 => {
                c.cmpu(0, c.r[0], 0x27);
                if c.crbit(2) { pc = 0x80219498; continue; }
                pc = 0x80219478;
            }
            0x80219478 => {
                c.cmpu(0, c.r[0], 0x4b);
                if c.crbit(2) { pc = 0x80219498; continue; }
                pc = 0x80219480;
            }
            0x80219480 => {
                c.cmpu(0, c.r[0], 0x97);
                if c.crbit(2) { pc = 0x80219498; continue; }
                pc = 0x80219488;
            }
            0x80219488 => {
                c.cmpu(0, c.r[0], 0x9b);
                if c.crbit(2) { pc = 0x80219498; continue; }
                pc = 0x80219490;
            }
            0x80219490 => {
                c.cmpu(0, c.r[0], 0x1a);
                if !c.crbit(2) { pc = 0x802194a0; continue; }
                pc = 0x80219498;
            }
            0x80219498 => {
                c.r[0] = 0x1;
                pc = 0x802194a4; continue;
                pc = 0x802194a0;
            }
            0x802194a0 => {
                c.r[0] = 0x0;
                pc = 0x802194a4;
            }
            0x802194a4 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x802194d4; continue; }
                pc = 0x802194ac;
            }
            0x802194ac => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x22;
                c.lr = 0x802194b8; f_802026e4(c);
                pc = 0x802194b8;
            }
            0x802194b8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802194d4; continue; }
                pc = 0x802194c4;
            }
            0x802194c4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021958c; continue;
                pc = 0x802194d4;
            }
            0x802194d4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.lr = 0x802194e4; f_802079dc(c);
                pc = 0x802194e4;
            }
            0x802194e4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[30] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x80219500; continue; }
                pc = 0x802194f0;
            }
            0x802194f0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021958c; continue;
                pc = 0x80219500;
            }
            0x80219500 => {
                c.lr = 0x80219504; f_800e0c54(c);
                pc = 0x80219504;
            }
            0x80219504 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30].rotate_left(0) & 0xffff;
                c.r[0] = divw(c.r[4], c.r[3]);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.r[0].wrapping_mul(c.r[3]);
                c.r[0] = c.r[4].wrapping_sub(c.r[0]);
                c.r[0] = c.r[0].rotate_left(2) & 0x3fffc;
                c.r[0] = c.lwz(c.r[5].wrapping_add(c.r[0]));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x8021953c; continue; }
                pc = 0x8021952c;
            }
            0x8021952c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1));
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021958c; continue;
                pc = 0x8021953c;
            }
            0x8021953c => {
                c.r[28] = c.r[0].rotate_left(0) & 0xffff;
                c.r[29] = 0x0;
                pc = 0x8021955c; continue;
                pc = 0x80219548;
            }
            0x80219548 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                c.lr = 0x80219558; f_80207b5c(c);
                pc = 0x80219558;
            }
            0x80219558 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x8021955c;
            }
            0x8021955c => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x80219548; continue; }
                pc = 0x80219568;
            }
            0x80219568 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80219570; f_8010c4d4(c);
                pc = 0x80219570;
            }
            0x80219570 => {
                c.lr = 0x80219574; f_800fa280(c);
                pc = 0x80219574;
            }
            0x80219574 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0xd;
                c.lr = 0x80219580; f_80132a38(c);
                pc = 0x80219580;
            }
            0x80219580 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffadf0));
                c.r[0] = c.r[3].wrapping_add(0x5);
                c.stw(c.r[13].wrapping_add(0xffffadf0), c.r[0]);
                pc = 0x8021958c;
            }
            0x8021958c => {
                { let mut ea = c.r[1].wrapping_add(0x50); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x60);
                if true { return; }
                return;
            }
            _ => panic!("f_80219354: bad pc {:#x}", pc),
        }
    }
}

