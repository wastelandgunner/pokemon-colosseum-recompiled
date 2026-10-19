#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fightFloorBiosGetFightOutPokemonPtrAry (0x801f6588, 56 bytes)
#[inline(never)]
pub fn f_801f6588(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6588) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6588); p = true; }
    }
    f_801f6588_b(c, 0x801f6588);
    if p { crate::prof::exit(c, 0x801f6588); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6588_at(c: &mut Cpu, pc: u32) { f_801f6588_b(c, pc) }
#[inline(always)]
fn f_801f6588_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6588;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6588, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6588 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6598; continue; }
                pc = 0x801f6590;
            }
            0x801f6590 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6598;
            }
            0x801f6598 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if cr_bit(cr, 0) { pc = 0x801f65ac; continue; }
                pc = 0x801f65a4;
            }
            0x801f65a4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f65ac;
            }
            0x801f65ac => { ins += 5;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[0] = c.r[4].rotate_left(2) & 0x3fffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa4c4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6588: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetFightPokemonEntryCnt (0x801f65c0, 20 bytes)
#[inline(never)]
pub fn f_801f65c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f65c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f65c0); p = true; }
    }
    f_801f65c0_b(c, 0x801f65c0);
    if p { crate::prof::exit(c, 0x801f65c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f65c0_at(c: &mut Cpu, pc: u32) { f_801f65c0_b(c, pc) }
#[inline(always)]
fn f_801f65c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f65c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f65c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f65c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f65c8;
            }
            0x801f65c8 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.sth(c.r[3].wrapping_add(0xffffa4c0), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f65c0: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetFightPokemonEntryCnt (0x801f65d4, 28 bytes)
#[inline(never)]
pub fn f_801f65d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f65d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f65d4); p = true; }
    }
    f_801f65d4_b(c, 0x801f65d4);
    if p { crate::prof::exit(c, 0x801f65d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f65d4_at(c: &mut Cpu, pc: u32) { f_801f65d4_b(c, pc) }
#[inline(always)]
fn f_801f65d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f65d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f65d4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f65d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f65e4; continue; }
                pc = 0x801f65dc;
            }
            0x801f65dc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f65e4;
            }
            0x801f65e4 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(0xffffa4c0));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f65d4: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosSetName (0x801f65f0, 16 bytes)
#[inline(never)]
pub fn f_801f65f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f65f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f65f0); p = true; }
    }
    f_801f65f0_b(c, 0x801f65f0);
    if p { crate::prof::exit(c, 0x801f65f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f65f0_at(c: &mut Cpu, pc: u32) { f_801f65f0_b(c, pc) }
#[inline(always)]
fn f_801f65f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f65f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f65f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f65f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f65f8;
            }
            0x801f65f8 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f65f0: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosGetName (0x801f6600, 24 bytes)
#[inline(never)]
pub fn f_801f6600(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6600) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6600); p = true; }
    }
    f_801f6600_b(c, 0x801f6600);
    if p { crate::prof::exit(c, 0x801f6600); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6600_at(c: &mut Cpu, pc: u32) { f_801f6600_b(c, pc) }
#[inline(always)]
fn f_801f6600_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6600;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6600, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6600 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6610; continue; }
                pc = 0x801f6608;
            }
            0x801f6608 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6610;
            }
            0x801f6610 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6600: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosSetEnvSndId (0x801f6618, 16 bytes)
#[inline(never)]
pub fn f_801f6618(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6618) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6618); p = true; }
    }
    f_801f6618_b(c, 0x801f6618);
    if p { crate::prof::exit(c, 0x801f6618); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6618_at(c: &mut Cpu, pc: u32) { f_801f6618_b(c, pc) }
#[inline(always)]
fn f_801f6618_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6618;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6618, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6618 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6620;
            }
            0x801f6620 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6618: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosSetBgmSndId (0x801f6628, 16 bytes)
#[inline(never)]
pub fn f_801f6628(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6628) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6628); p = true; }
    }
    f_801f6628_b(c, 0x801f6628);
    if p { crate::prof::exit(c, 0x801f6628); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6628_at(c: &mut Cpu, pc: u32) { f_801f6628_b(c, pc) }
#[inline(always)]
fn f_801f6628_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6628;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6628, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6628 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6630;
            }
            0x801f6630 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6628: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosSetSyoukaiWzxDataId (0x801f6638, 16 bytes)
#[inline(never)]
pub fn f_801f6638(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6638) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6638); p = true; }
    }
    f_801f6638_b(c, 0x801f6638);
    if p { crate::prof::exit(c, 0x801f6638); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6638_at(c: &mut Cpu, pc: u32) { f_801f6638_b(c, pc) }
#[inline(always)]
fn f_801f6638_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6638;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6638, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6638 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6640;
            }
            0x801f6640 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6638: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosSetTikeiDataId (0x801f6648, 16 bytes)
#[inline(never)]
pub fn f_801f6648(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6648) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6648); p = true; }
    }
    f_801f6648_b(c, 0x801f6648);
    if p { crate::prof::exit(c, 0x801f6648); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6648_at(c: &mut Cpu, pc: u32) { f_801f6648_b(c, pc) }
#[inline(always)]
fn f_801f6648_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6648;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6648, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6648 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6650;
            }
            0x801f6650 => { ins += 2;
                c.stb(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6648: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosSetFightSideDataId (0x801f6658, 36 bytes)
#[inline(never)]
pub fn f_801f6658(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6658) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6658); p = true; }
    }
    f_801f6658_b(c, 0x801f6658);
    if p { crate::prof::exit(c, 0x801f6658); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6658_at(c: &mut Cpu, pc: u32) { f_801f6658_b(c, pc) }
#[inline(always)]
fn f_801f6658_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6658;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6658, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6658 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6660;
            }
            0x801f6660 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f666c;
            }
            0x801f666c => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.sth(c.r[3].wrapping_add(0x14), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6658: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosSetFloorDataId (0x801f667c, 16 bytes)
#[inline(never)]
pub fn f_801f667c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f667c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f667c); p = true; }
    }
    f_801f667c_b(c, 0x801f667c);
    if p { crate::prof::exit(c, 0x801f667c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f667c_at(c: &mut Cpu, pc: u32) { f_801f667c_b(c, pc) }
#[inline(always)]
fn f_801f667c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f667c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f667c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f667c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6684;
            }
            0x801f6684 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f667c: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosGetEnvSndId (0x801f668c, 24 bytes)
#[inline(never)]
pub fn f_801f668c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f668c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f668c); p = true; }
    }
    f_801f668c_b(c, 0x801f668c);
    if p { crate::prof::exit(c, 0x801f668c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f668c_at(c: &mut Cpu, pc: u32) { f_801f668c_b(c, pc) }
#[inline(always)]
fn f_801f668c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f668c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f668c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f668c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f669c; continue; }
                pc = 0x801f6694;
            }
            0x801f6694 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f669c;
            }
            0x801f669c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x10));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f668c: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosGetBgmSndId (0x801f66a4, 24 bytes)
#[inline(never)]
pub fn f_801f66a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f66a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f66a4); p = true; }
    }
    f_801f66a4_b(c, 0x801f66a4);
    if p { crate::prof::exit(c, 0x801f66a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f66a4_at(c: &mut Cpu, pc: u32) { f_801f66a4_b(c, pc) }
#[inline(always)]
fn f_801f66a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f66a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f66a4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f66a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f66b4; continue; }
                pc = 0x801f66ac;
            }
            0x801f66ac => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f66b4;
            }
            0x801f66b4 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f66a4: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosGetSyoukaiWzxDataId (0x801f66bc, 24 bytes)
#[inline(never)]
pub fn f_801f66bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f66bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801f66bc); p = true; }
    }
    f_801f66bc_b(c, 0x801f66bc);
    if p { crate::prof::exit(c, 0x801f66bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f66bc_at(c: &mut Cpu, pc: u32) { f_801f66bc_b(c, pc) }
#[inline(always)]
fn f_801f66bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f66bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f66bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f66bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f66cc; continue; }
                pc = 0x801f66c4;
            }
            0x801f66c4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f66cc;
            }
            0x801f66cc => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f66bc: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosGetTikeiDataId (0x801f66d4, 24 bytes)
#[inline(never)]
pub fn f_801f66d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f66d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f66d4); p = true; }
    }
    f_801f66d4_b(c, 0x801f66d4);
    if p { crate::prof::exit(c, 0x801f66d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f66d4_at(c: &mut Cpu, pc: u32) { f_801f66d4_b(c, pc) }
#[inline(always)]
fn f_801f66d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f66d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f66d4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f66d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f66e4; continue; }
                pc = 0x801f66dc;
            }
            0x801f66dc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f66e4;
            }
            0x801f66e4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f66d4: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosGetFightSideDataId (0x801f66ec, 52 bytes)
#[inline(never)]
pub fn f_801f66ec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f66ec) { return; }
        if c.prof { crate::prof::enter(c, 0x801f66ec); p = true; }
    }
    f_801f66ec_b(c, 0x801f66ec);
    if p { crate::prof::exit(c, 0x801f66ec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f66ec_at(c: &mut Cpu, pc: u32) { f_801f66ec_b(c, pc) }
#[inline(always)]
fn f_801f66ec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f66ec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f66ec, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f66ec => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f66fc; continue; }
                pc = 0x801f66f4;
            }
            0x801f66f4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f66fc;
            }
            0x801f66fc => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f6710; continue; }
                pc = 0x801f6708;
            }
            0x801f6708 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6710;
            }
            0x801f6710 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x14));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f66ec: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosGetFloorDataId (0x801f6720, 24 bytes)
#[inline(never)]
pub fn f_801f6720(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6720) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6720); p = true; }
    }
    f_801f6720_b(c, 0x801f6720);
    if p { crate::prof::exit(c, 0x801f6720); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6720_at(c: &mut Cpu, pc: u32) { f_801f6720_b(c, pc) }
#[inline(always)]
fn f_801f6720_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6720;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6720, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6720 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6730; continue; }
                pc = 0x801f6728;
            }
            0x801f6728 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6730;
            }
            0x801f6730 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6720: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorDataBiosGetPtr (0x801f6738, 44 bytes)
#[inline(never)]
pub fn f_801f6738(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6738) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6738); p = true; }
    }
    f_801f6738_b(c, 0x801f6738);
    if p { crate::prof::exit(c, 0x801f6738); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6738_at(c: &mut Cpu, pc: u32) { f_801f6738_b(c, pc) }
#[inline(always)]
fn f_801f6738_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6738;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6738, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6738 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8728));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801f6754; continue; }
                pc = 0x801f674c;
            }
            0x801f674c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6754;
            }
            0x801f6754 => { ins += 4;
                c.r[0] = c.r[3].wrapping_mul(0x18);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff872c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6738: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAppointTokuseiDataId (0x801f6764, 20 bytes)
#[inline(never)]
pub fn f_801f6764(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6764) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6764); p = true; }
    }
    f_801f6764_b(c, 0x801f6764);
    if p { crate::prof::exit(c, 0x801f6764); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6764_at(c: &mut Cpu, pc: u32) { f_801f6764_b(c, pc) }
#[inline(always)]
fn f_801f6764_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6764;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6764, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6764 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f676c;
            }
            0x801f676c => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.sth(c.r[3].wrapping_add(0xffffa4be), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6764: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAppointItemDataId (0x801f6778, 20 bytes)
#[inline(never)]
pub fn f_801f6778(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6778) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6778); p = true; }
    }
    f_801f6778_b(c, 0x801f6778);
    if p { crate::prof::exit(c, 0x801f6778); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6778_at(c: &mut Cpu, pc: u32) { f_801f6778_b(c, pc) }
#[inline(always)]
fn f_801f6778_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6778;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6778, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6778 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6780;
            }
            0x801f6780 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.sth(c.r[3].wrapping_add(0xffffa4bc), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6778: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAppointWazaDataId (0x801f678c, 20 bytes)
#[inline(never)]
pub fn f_801f678c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f678c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f678c); p = true; }
    }
    f_801f678c_b(c, 0x801f678c);
    if p { crate::prof::exit(c, 0x801f678c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f678c_at(c: &mut Cpu, pc: u32) { f_801f678c_b(c, pc) }
#[inline(always)]
fn f_801f678c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f678c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f678c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f678c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6794;
            }
            0x801f6794 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.sth(c.r[3].wrapping_add(0xffffa4ba), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f678c: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAppointPokemonDataId (0x801f67a0, 20 bytes)
#[inline(never)]
pub fn f_801f67a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f67a0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f67a0); p = true; }
    }
    f_801f67a0_b(c, 0x801f67a0);
    if p { crate::prof::exit(c, 0x801f67a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f67a0_at(c: &mut Cpu, pc: u32) { f_801f67a0_b(c, pc) }
#[inline(always)]
fn f_801f67a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f67a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f67a0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f67a0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f67a8;
            }
            0x801f67a8 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.sth(c.r[3].wrapping_add(0xffffa4b8), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f67a0: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetWazakoukaMsgId (0x801f67b4, 20 bytes)
#[inline(never)]
pub fn f_801f67b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f67b4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f67b4); p = true; }
    }
    f_801f67b4_b(c, 0x801f67b4);
    if p { crate::prof::exit(c, 0x801f67b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f67b4_at(c: &mut Cpu, pc: u32) { f_801f67b4_b(c, pc) }
#[inline(always)]
fn f_801f67b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f67b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f67b4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f67b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f67bc;
            }
            0x801f67bc => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa4b0), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f67b4: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetCriticalMsgId (0x801f67c8, 20 bytes)
#[inline(never)]
pub fn f_801f67c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f67c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f67c8); p = true; }
    }
    f_801f67c8_b(c, 0x801f67c8);
    if p { crate::prof::exit(c, 0x801f67c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f67c8_at(c: &mut Cpu, pc: u32) { f_801f67c8_b(c, pc) }
#[inline(always)]
fn f_801f67c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f67c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f67c8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f67c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f67d0;
            }
            0x801f67d0 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa4ac), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f67c8: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAttackMsgId (0x801f67dc, 20 bytes)
#[inline(never)]
pub fn f_801f67dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f67dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801f67dc); p = true; }
    }
    f_801f67dc_b(c, 0x801f67dc);
    if p { crate::prof::exit(c, 0x801f67dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f67dc_at(c: &mut Cpu, pc: u32) { f_801f67dc_b(c, pc) }
#[inline(always)]
fn f_801f67dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f67dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f67dc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f67dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f67e4;
            }
            0x801f67e4 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa4a8), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f67dc: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAppointMsgId (0x801f67f0, 20 bytes)
#[inline(never)]
pub fn f_801f67f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f67f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f67f0); p = true; }
    }
    f_801f67f0_b(c, 0x801f67f0);
    if p { crate::prof::exit(c, 0x801f67f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f67f0_at(c: &mut Cpu, pc: u32) { f_801f67f0_b(c, pc) }
#[inline(always)]
fn f_801f67f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f67f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f67f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f67f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f67f8;
            }
            0x801f67f8 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa4b4), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f67f0: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAppointItemPtr (0x801f6804, 20 bytes)
#[inline(never)]
pub fn f_801f6804(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6804) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6804); p = true; }
    }
    f_801f6804_b(c, 0x801f6804);
    if p { crate::prof::exit(c, 0x801f6804); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6804_at(c: &mut Cpu, pc: u32) { f_801f6804_b(c, pc) }
#[inline(always)]
fn f_801f6804_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6804;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6804, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6804 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f680c;
            }
            0x801f680c => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa4a4), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6804: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAppointSidePtr (0x801f6818, 20 bytes)
#[inline(never)]
pub fn f_801f6818(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6818) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6818); p = true; }
    }
    f_801f6818_b(c, 0x801f6818);
    if p { crate::prof::exit(c, 0x801f6818); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6818_at(c: &mut Cpu, pc: u32) { f_801f6818_b(c, pc) }
#[inline(always)]
fn f_801f6818_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6818;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6818, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6818 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6820;
            }
            0x801f6820 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa4a0), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6818: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAppointWazaPtr (0x801f682c, 20 bytes)
#[inline(never)]
pub fn f_801f682c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f682c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f682c); p = true; }
    }
    f_801f682c_b(c, 0x801f682c);
    if p { crate::prof::exit(c, 0x801f682c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f682c_at(c: &mut Cpu, pc: u32) { f_801f682c_b(c, pc) }
#[inline(always)]
fn f_801f682c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f682c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f682c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f682c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6834;
            }
            0x801f6834 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa49c), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f682c: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAppointTrainerPtr (0x801f6840, 20 bytes)
#[inline(never)]
pub fn f_801f6840(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6840) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6840); p = true; }
    }
    f_801f6840_b(c, 0x801f6840);
    if p { crate::prof::exit(c, 0x801f6840); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6840_at(c: &mut Cpu, pc: u32) { f_801f6840_b(c, pc) }
#[inline(always)]
fn f_801f6840_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6840;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6840, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6840 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6848;
            }
            0x801f6848 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa498), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6840: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAppointPokemonPtr (0x801f6854, 20 bytes)
#[inline(never)]
pub fn f_801f6854(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6854) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6854); p = true; }
    }
    f_801f6854_b(c, 0x801f6854);
    if p { crate::prof::exit(c, 0x801f6854); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6854_at(c: &mut Cpu, pc: u32) { f_801f6854_b(c, pc) }
#[inline(always)]
fn f_801f6854_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6854;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6854, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6854 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f685c;
            }
            0x801f685c => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa494), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6854: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetIrekaePokemonPtr (0x801f6868, 20 bytes)
#[inline(never)]
pub fn f_801f6868(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6868) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6868); p = true; }
    }
    f_801f6868_b(c, 0x801f6868);
    if p { crate::prof::exit(c, 0x801f6868); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6868_at(c: &mut Cpu, pc: u32) { f_801f6868_b(c, pc) }
#[inline(always)]
fn f_801f6868_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6868;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6868, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6868 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6870;
            }
            0x801f6870 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa480), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6868: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetEscapePokemonPtr (0x801f687c, 20 bytes)
#[inline(never)]
pub fn f_801f687c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f687c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f687c); p = true; }
    }
    f_801f687c_b(c, 0x801f687c);
    if p { crate::prof::exit(c, 0x801f687c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f687c_at(c: &mut Cpu, pc: u32) { f_801f687c_b(c, pc) }
#[inline(always)]
fn f_801f687c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f687c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f687c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f687c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6884;
            }
            0x801f6884 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa47c), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f687c: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetDefensePokemonPtr (0x801f6890, 20 bytes)
#[inline(never)]
pub fn f_801f6890(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6890) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6890); p = true; }
    }
    f_801f6890_b(c, 0x801f6890);
    if p { crate::prof::exit(c, 0x801f6890); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6890_at(c: &mut Cpu, pc: u32) { f_801f6890_b(c, pc) }
#[inline(always)]
fn f_801f6890_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6890;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6890, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6890 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6898;
            }
            0x801f6898 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa478), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6890: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetAttackPokemonPtr (0x801f68a4, 20 bytes)
#[inline(never)]
pub fn f_801f68a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f68a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f68a4); p = true; }
    }
    f_801f68a4_b(c, 0x801f68a4);
    if p { crate::prof::exit(c, 0x801f68a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f68a4_at(c: &mut Cpu, pc: u32) { f_801f68a4_b(c, pc) }
#[inline(always)]
fn f_801f68a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f68a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f68a4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f68a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f68ac;
            }
            0x801f68ac => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.stw(c.r[3].wrapping_add(0xffffa474), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f68a4: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetEncountDataId (0x801f68b8, 16 bytes)
#[inline(never)]
pub fn f_801f68b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f68b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f68b8); p = true; }
    }
    f_801f68b8_b(c, 0x801f68b8);
    if p { crate::prof::exit(c, 0x801f68b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f68b8_at(c: &mut Cpu, pc: u32) { f_801f68b8_b(c, pc) }
#[inline(always)]
fn f_801f68b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f68b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f68b8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f68b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f68c0;
            }
            0x801f68c0 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x12), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f68b8: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosSetTurnCount (0x801f68c8, 16 bytes)
#[inline(never)]
pub fn f_801f68c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f68c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f68c8); p = true; }
    }
    f_801f68c8_b(c, 0x801f68c8);
    if p { crate::prof::exit(c, 0x801f68c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f68c8_at(c: &mut Cpu, pc: u32) { f_801f68c8_b(c, pc) }
#[inline(always)]
fn f_801f68c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f68c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f68c8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f68c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f68d0;
            }
            0x801f68d0 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f68c8: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAppointTokuseiDataId (0x801f68d8, 28 bytes)
#[inline(never)]
pub fn f_801f68d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f68d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f68d8); p = true; }
    }
    f_801f68d8_b(c, 0x801f68d8);
    if p { crate::prof::exit(c, 0x801f68d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f68d8_at(c: &mut Cpu, pc: u32) { f_801f68d8_b(c, pc) }
#[inline(always)]
fn f_801f68d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f68d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f68d8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f68d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f68e8; continue; }
                pc = 0x801f68e0;
            }
            0x801f68e0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f68e8;
            }
            0x801f68e8 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xffffa4be));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f68d8: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAppointItemDataId (0x801f68f4, 28 bytes)
#[inline(never)]
pub fn f_801f68f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f68f4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f68f4); p = true; }
    }
    f_801f68f4_b(c, 0x801f68f4);
    if p { crate::prof::exit(c, 0x801f68f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f68f4_at(c: &mut Cpu, pc: u32) { f_801f68f4_b(c, pc) }
#[inline(always)]
fn f_801f68f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f68f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f68f4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f68f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6904; continue; }
                pc = 0x801f68fc;
            }
            0x801f68fc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6904;
            }
            0x801f6904 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xffffa4bc));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f68f4: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAppointWazaDataId (0x801f6910, 28 bytes)
#[inline(never)]
pub fn f_801f6910(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6910) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6910); p = true; }
    }
    f_801f6910_b(c, 0x801f6910);
    if p { crate::prof::exit(c, 0x801f6910); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6910_at(c: &mut Cpu, pc: u32) { f_801f6910_b(c, pc) }
#[inline(always)]
fn f_801f6910_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6910;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6910, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6910 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6920; continue; }
                pc = 0x801f6918;
            }
            0x801f6918 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6920;
            }
            0x801f6920 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xffffa4ba));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6910: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAppointPokemonDataId (0x801f692c, 28 bytes)
#[inline(never)]
pub fn f_801f692c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f692c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f692c); p = true; }
    }
    f_801f692c_b(c, 0x801f692c);
    if p { crate::prof::exit(c, 0x801f692c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f692c_at(c: &mut Cpu, pc: u32) { f_801f692c_b(c, pc) }
#[inline(always)]
fn f_801f692c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f692c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f692c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f692c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f693c; continue; }
                pc = 0x801f6934;
            }
            0x801f6934 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f693c;
            }
            0x801f693c => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xffffa4b8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f692c: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetWazakoukaMsgId (0x801f6948, 28 bytes)
#[inline(never)]
pub fn f_801f6948(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6948) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6948); p = true; }
    }
    f_801f6948_b(c, 0x801f6948);
    if p { crate::prof::exit(c, 0x801f6948); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6948_at(c: &mut Cpu, pc: u32) { f_801f6948_b(c, pc) }
#[inline(always)]
fn f_801f6948_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6948;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6948, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6948 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6958; continue; }
                pc = 0x801f6950;
            }
            0x801f6950 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6958;
            }
            0x801f6958 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa4b0));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6948: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetCriticalMsgId (0x801f6964, 28 bytes)
#[inline(never)]
pub fn f_801f6964(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6964) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6964); p = true; }
    }
    f_801f6964_b(c, 0x801f6964);
    if p { crate::prof::exit(c, 0x801f6964); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6964_at(c: &mut Cpu, pc: u32) { f_801f6964_b(c, pc) }
#[inline(always)]
fn f_801f6964_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6964;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6964, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6964 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6974; continue; }
                pc = 0x801f696c;
            }
            0x801f696c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6974;
            }
            0x801f6974 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa4ac));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6964: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAttackMsgId (0x801f6980, 28 bytes)
#[inline(never)]
pub fn f_801f6980(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6980) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6980); p = true; }
    }
    f_801f6980_b(c, 0x801f6980);
    if p { crate::prof::exit(c, 0x801f6980); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6980_at(c: &mut Cpu, pc: u32) { f_801f6980_b(c, pc) }
#[inline(always)]
fn f_801f6980_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6980;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6980, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6980 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6990; continue; }
                pc = 0x801f6988;
            }
            0x801f6988 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6990;
            }
            0x801f6990 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa4a8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6980: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAppointMsgId (0x801f699c, 28 bytes)
#[inline(never)]
pub fn f_801f699c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f699c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f699c); p = true; }
    }
    f_801f699c_b(c, 0x801f699c);
    if p { crate::prof::exit(c, 0x801f699c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f699c_at(c: &mut Cpu, pc: u32) { f_801f699c_b(c, pc) }
#[inline(always)]
fn f_801f699c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f699c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f699c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f699c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f69ac; continue; }
                pc = 0x801f69a4;
            }
            0x801f69a4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f69ac;
            }
            0x801f69ac => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa4b4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f699c: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAppointItemPtr (0x801f69b8, 28 bytes)
#[inline(never)]
pub fn f_801f69b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f69b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f69b8); p = true; }
    }
    f_801f69b8_b(c, 0x801f69b8);
    if p { crate::prof::exit(c, 0x801f69b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f69b8_at(c: &mut Cpu, pc: u32) { f_801f69b8_b(c, pc) }
#[inline(always)]
fn f_801f69b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f69b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f69b8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f69b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f69c8; continue; }
                pc = 0x801f69c0;
            }
            0x801f69c0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f69c8;
            }
            0x801f69c8 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa4a4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f69b8: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAppointSidePtr (0x801f69d4, 28 bytes)
#[inline(never)]
pub fn f_801f69d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f69d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f69d4); p = true; }
    }
    f_801f69d4_b(c, 0x801f69d4);
    if p { crate::prof::exit(c, 0x801f69d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f69d4_at(c: &mut Cpu, pc: u32) { f_801f69d4_b(c, pc) }
#[inline(always)]
fn f_801f69d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f69d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f69d4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f69d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f69e4; continue; }
                pc = 0x801f69dc;
            }
            0x801f69dc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f69e4;
            }
            0x801f69e4 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa4a0));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f69d4: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAppointWazaPtr (0x801f69f0, 28 bytes)
#[inline(never)]
pub fn f_801f69f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f69f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f69f0); p = true; }
    }
    f_801f69f0_b(c, 0x801f69f0);
    if p { crate::prof::exit(c, 0x801f69f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f69f0_at(c: &mut Cpu, pc: u32) { f_801f69f0_b(c, pc) }
#[inline(always)]
fn f_801f69f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f69f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f69f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f69f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6a00; continue; }
                pc = 0x801f69f8;
            }
            0x801f69f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6a00;
            }
            0x801f6a00 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa49c));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f69f0: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAppointTrainerPtr (0x801f6a0c, 28 bytes)
#[inline(never)]
pub fn f_801f6a0c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6a0c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6a0c); p = true; }
    }
    f_801f6a0c_b(c, 0x801f6a0c);
    if p { crate::prof::exit(c, 0x801f6a0c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6a0c_at(c: &mut Cpu, pc: u32) { f_801f6a0c_b(c, pc) }
#[inline(always)]
fn f_801f6a0c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6a0c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6a0c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6a0c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6a1c; continue; }
                pc = 0x801f6a14;
            }
            0x801f6a14 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6a1c;
            }
            0x801f6a1c => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa498));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6a0c: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAppointPokemonPtr (0x801f6a28, 28 bytes)
#[inline(never)]
pub fn f_801f6a28(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6a28) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6a28); p = true; }
    }
    f_801f6a28_b(c, 0x801f6a28);
    if p { crate::prof::exit(c, 0x801f6a28); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6a28_at(c: &mut Cpu, pc: u32) { f_801f6a28_b(c, pc) }
#[inline(always)]
fn f_801f6a28_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6a28;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6a28, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6a28 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6a38; continue; }
                pc = 0x801f6a30;
            }
            0x801f6a30 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6a38;
            }
            0x801f6a38 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa494));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6a28: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetIrekaePokemonPtr (0x801f6a44, 28 bytes)
#[inline(never)]
pub fn f_801f6a44(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6a44) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6a44); p = true; }
    }
    f_801f6a44_b(c, 0x801f6a44);
    if p { crate::prof::exit(c, 0x801f6a44); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6a44_at(c: &mut Cpu, pc: u32) { f_801f6a44_b(c, pc) }
#[inline(always)]
fn f_801f6a44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6a44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6a44, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6a44 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6a54; continue; }
                pc = 0x801f6a4c;
            }
            0x801f6a4c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6a54;
            }
            0x801f6a54 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa480));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6a44: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetEscapePokemonPtr (0x801f6a60, 28 bytes)
#[inline(never)]
pub fn f_801f6a60(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6a60) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6a60); p = true; }
    }
    f_801f6a60_b(c, 0x801f6a60);
    if p { crate::prof::exit(c, 0x801f6a60); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6a60_at(c: &mut Cpu, pc: u32) { f_801f6a60_b(c, pc) }
#[inline(always)]
fn f_801f6a60_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6a60;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6a60, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6a60 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6a70; continue; }
                pc = 0x801f6a68;
            }
            0x801f6a68 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6a70;
            }
            0x801f6a70 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa47c));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6a60: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetDefensePokemonPtr (0x801f6a7c, 28 bytes)
#[inline(never)]
pub fn f_801f6a7c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6a7c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6a7c); p = true; }
    }
    f_801f6a7c_b(c, 0x801f6a7c);
    if p { crate::prof::exit(c, 0x801f6a7c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6a7c_at(c: &mut Cpu, pc: u32) { f_801f6a7c_b(c, pc) }
#[inline(always)]
fn f_801f6a7c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6a7c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6a7c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6a7c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6a8c; continue; }
                pc = 0x801f6a84;
            }
            0x801f6a84 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6a8c;
            }
            0x801f6a8c => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa478));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6a7c: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetAttackPokemonPtr (0x801f6a98, 28 bytes)
#[inline(never)]
pub fn f_801f6a98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6a98) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6a98); p = true; }
    }
    f_801f6a98_b(c, 0x801f6a98);
    if p { crate::prof::exit(c, 0x801f6a98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6a98_at(c: &mut Cpu, pc: u32) { f_801f6a98_b(c, pc) }
#[inline(always)]
fn f_801f6a98_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6a98;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6a98, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6a98 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6aa8; continue; }
                pc = 0x801f6aa0;
            }
            0x801f6aa0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6aa8;
            }
            0x801f6aa8 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x10000);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffa474));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6a98: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetFightSidePtr (0x801f6ab4, 52 bytes)
#[inline(never)]
pub fn f_801f6ab4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6ab4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6ab4); p = true; }
    }
    f_801f6ab4_b(c, 0x801f6ab4);
    if p { crate::prof::exit(c, 0x801f6ab4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6ab4_at(c: &mut Cpu, pc: u32) { f_801f6ab4_b(c, pc) }
#[inline(always)]
fn f_801f6ab4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6ab4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6ab4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6ab4 => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x801f6ac4; continue; }
                pc = 0x801f6abc;
            }
            0x801f6abc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6ac4;
            }
            0x801f6ac4 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f6ad8; continue; }
                pc = 0x801f6ad0;
            }
            0x801f6ad0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6ad8;
            }
            0x801f6ad8 => { ins += 4;
                c.r[3] = c.r[0].wrapping_mul(0x5230);
                c.r[3] = c.r[3].wrapping_add(0x14);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6ab4: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetEncountDataId (0x801f6ae8, 24 bytes)
#[inline(never)]
pub fn f_801f6ae8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6ae8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6ae8); p = true; }
    }
    f_801f6ae8_b(c, 0x801f6ae8);
    if p { crate::prof::exit(c, 0x801f6ae8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6ae8_at(c: &mut Cpu, pc: u32) { f_801f6ae8_b(c, pc) }
#[inline(always)]
fn f_801f6ae8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6ae8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6ae8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6ae8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6af8; continue; }
                pc = 0x801f6af0;
            }
            0x801f6af0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6af8;
            }
            0x801f6af8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x12));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6ae8: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetTurnCount (0x801f6b00, 24 bytes)
#[inline(never)]
pub fn f_801f6b00(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6b00) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6b00); p = true; }
    }
    f_801f6b00_b(c, 0x801f6b00);
    if p { crate::prof::exit(c, 0x801f6b00); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6b00_at(c: &mut Cpu, pc: u32) { f_801f6b00_b(c, pc) }
#[inline(always)]
fn f_801f6b00_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6b00;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6b00, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6b00 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6b10; continue; }
                pc = 0x801f6b08;
            }
            0x801f6b08 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6b10;
            }
            0x801f6b10 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x10));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6b00: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetJoutaiPtr (0x801f6b18, 48 bytes)
#[inline(never)]
pub fn f_801f6b18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6b18) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6b18); p = true; }
    }
    f_801f6b18_b(c, 0x801f6b18);
    if p { crate::prof::exit(c, 0x801f6b18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6b18_at(c: &mut Cpu, pc: u32) { f_801f6b18_b(c, pc) }
#[inline(always)]
fn f_801f6b18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6b18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6b18, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6b18 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6b28; continue; }
                pc = 0x801f6b20;
            }
            0x801f6b20 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6b28;
            }
            0x801f6b28 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 0) { pc = 0x801f6b3c; continue; }
                pc = 0x801f6b34;
            }
            0x801f6b34 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f6b3c;
            }
            0x801f6b3c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6b18: bad pc {:#x}", pc),
        }
    }
}

/// fightFloorBiosGetFightFloorPtr (0x801f6b48, 12 bytes)
#[inline(never)]
pub fn f_801f6b48(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6b48) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6b48); p = true; }
    }
    f_801f6b48_b(c, 0x801f6b48);
    if p { crate::prof::exit(c, 0x801f6b48); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6b48_at(c: &mut Cpu, pc: u32) { f_801f6b48_b(c, pc) }
#[inline(always)]
fn f_801f6b48_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6b48;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6b48, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6b48 => { ins += 3;
                c.r[3] = 0x80470000;
                c.r[3] = c.r[3].wrapping_add(0xffffdd90);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6b48: bad pc {:#x}", pc),
        }
    }
}

/// fightSideRegistFightSideEnemyPokemonFightAll (0x801f6b54, 248 bytes)
#[inline(never)]
pub fn f_801f6b54(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6b54) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6b54); p = true; }
    }
    f_801f6b54_b(c, 0x801f6b54);
    if p { crate::prof::exit(c, 0x801f6b54); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6b54_at(c: &mut Cpu, pc: u32) { f_801f6b54_b(c, pc) }
#[inline(always)]
fn f_801f6b54_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6b54;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6b54, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6b54 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[23] = c.r[3] | c.r[3];
                c.r[24] = c.r[4] | c.r[4];
                c.r[25] = c.r[7] | c.r[7];
                c.r[31] = c.r[6].rotate_left(0) & 0xffff;
                c.r[27] = 0x0;
                pc = 0x801f6c2c; continue;
                pc = 0x801f6b7c;
            }
            0x801f6b7c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6b8c; continue; }
                pc = 0x801f6b84;
            }
            0x801f6b84 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f6b98; continue;
                pc = 0x801f6b8c;
            }
            0x801f6b8c => { ins += 3;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x801f6b98; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6b98;
            }
            0x801f6b98 => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x801f6ba0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fa634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6ba0;
            }
            0x801f6ba0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f6c28; continue; }
                pc = 0x801f6ba8;
            }
            0x801f6ba8 => { ins += 3;
                c.r[30] = c.r[25].rotate_left(0) & 0xffff;
                c.r[26] = 0x0;
                pc = 0x801f6c1c; continue;
                pc = 0x801f6bb4;
            }
            0x801f6bb4 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f6bc8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6bc8;
            }
            0x801f6bc8 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x801f6bd0; { c.steps = st; c.instrs = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6bd0;
            }
            0x801f6bd0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f6c18; continue; }
                pc = 0x801f6bd8;
            }
            0x801f6bd8 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0xd5;
                c.r[6] = 0x0;
                c.lr = 0x801f6bec; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6bec;
            }
            0x801f6bec => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0xce;
                c.r[6] = 0x0;
                c.lr = 0x801f6bfc; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6bfc;
            }
            0x801f6bfc => { ins += 2;
                c.r[7] = c.r[3] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[7]);
                if cr_bit(cr, 0) { pc = 0x801f6c18; continue; }
                pc = 0x801f6c04;
            }
            0x801f6c04 => { ins += 5;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x57;
                c.r[6] = 0x0;
                c.lr = 0x801f6c18; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6c18;
            }
            0x801f6c18 => { ins += 1;
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x801f6c1c;
            }
            0x801f6c1c => { ins += 3;
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[30]);
                if cr_bit(cr, 0) { pc = 0x801f6bb4; continue; }
                pc = 0x801f6c28;
            }
            0x801f6c28 => { ins += 1;
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x801f6c2c;
            }
            0x801f6c2c => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x801f6b7c; continue; }
                pc = 0x801f6c38;
            }
            0x801f6c38 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6b54: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetJoutaiUserFightTargetId (0x801f6c4c, 84 bytes)
#[inline(never)]
pub fn f_801f6c4c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6c4c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6c4c); p = true; }
    }
    f_801f6c4c_b(c, 0x801f6c4c);
    if p { crate::prof::exit(c, 0x801f6c4c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6c4c_at(c: &mut Cpu, pc: u32) { f_801f6c4c_b(c, pc) }
#[inline(always)]
fn f_801f6c4c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6c4c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6c4c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6c4c => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f6c6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6c6c;
            }
            0x801f6c6c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 2) { pc = 0x801f6c80; continue; }
                pc = 0x801f6c78;
            }
            0x801f6c78 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f6c8c; continue;
                pc = 0x801f6c80;
            }
            0x801f6c80 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f6c8c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011a860(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6c8c;
            }
            0x801f6c8c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6c4c: bad pc {:#x}", pc),
        }
    }
}

/// fightSideSetNowKaisuuJoutaiDataId (0x801f6ca0, 84 bytes)
#[inline(never)]
pub fn f_801f6ca0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6ca0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6ca0); p = true; }
    }
    f_801f6ca0_b(c, 0x801f6ca0);
    if p { crate::prof::exit(c, 0x801f6ca0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6ca0_at(c: &mut Cpu, pc: u32) { f_801f6ca0_b(c, pc) }
#[inline(always)]
fn f_801f6ca0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6ca0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6ca0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6ca0 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801f6cc4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6cc4;
            }
            0x801f6cc4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 2) { pc = 0x801f6ce0; continue; }
                pc = 0x801f6cd0;
            }
            0x801f6cd0 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x801f6ce0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011ab50(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6ce0;
            }
            0x801f6ce0 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6ca0: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetNowKaisuuJoutaiDataId (0x801f6cf4, 84 bytes)
#[inline(never)]
pub fn f_801f6cf4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6cf4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6cf4); p = true; }
    }
    f_801f6cf4_b(c, 0x801f6cf4);
    if p { crate::prof::exit(c, 0x801f6cf4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6cf4_at(c: &mut Cpu, pc: u32) { f_801f6cf4_b(c, pc) }
#[inline(always)]
fn f_801f6cf4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6cf4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6cf4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6cf4 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f6d14; { c.steps = st; c.instrs = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6d14;
            }
            0x801f6d14 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 2) { pc = 0x801f6d28; continue; }
                pc = 0x801f6d20;
            }
            0x801f6d20 => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x801f6d34; continue;
                pc = 0x801f6d28;
            }
            0x801f6d28 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f6d34; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011acb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6d34;
            }
            0x801f6d34 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6cf4: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetKaisuuJoutaiDataId (0x801f6d48, 84 bytes)
#[inline(never)]
pub fn f_801f6d48(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6d48) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6d48); p = true; }
    }
    f_801f6d48_b(c, 0x801f6d48);
    if p { crate::prof::exit(c, 0x801f6d48); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6d48_at(c: &mut Cpu, pc: u32) { f_801f6d48_b(c, pc) }
#[inline(always)]
fn f_801f6d48_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6d48;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6d48, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6d48 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f6d68; { c.steps = st; c.instrs = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6d68;
            }
            0x801f6d68 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 2) { pc = 0x801f6d7c; continue; }
                pc = 0x801f6d74;
            }
            0x801f6d74 => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x801f6d88; continue;
                pc = 0x801f6d7c;
            }
            0x801f6d7c => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f6d88; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011ae40(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6d88;
            }
            0x801f6d88 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6d48: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetCountAsJoutaiDataId (0x801f6d9c, 84 bytes)
#[inline(never)]
pub fn f_801f6d9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6d9c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6d9c); p = true; }
    }
    f_801f6d9c_b(c, 0x801f6d9c);
    if p { crate::prof::exit(c, 0x801f6d9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6d9c_at(c: &mut Cpu, pc: u32) { f_801f6d9c_b(c, pc) }
#[inline(always)]
fn f_801f6d9c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6d9c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6d9c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6d9c => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f6dbc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6dbc;
            }
            0x801f6dbc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 2) { pc = 0x801f6dd0; continue; }
                pc = 0x801f6dc8;
            }
            0x801f6dc8 => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x801f6ddc; continue;
                pc = 0x801f6dd0;
            }
            0x801f6dd0 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f6ddc; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011b130(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6ddc;
            }
            0x801f6ddc => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6d9c: bad pc {:#x}", pc),
        }
    }
}

/// fightSideWriteJoutaiDataId (0x801f6df0, 84 bytes)
#[inline(never)]
pub fn f_801f6df0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6df0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6df0); p = true; }
    }
    f_801f6df0_b(c, 0x801f6df0);
    if p { crate::prof::exit(c, 0x801f6df0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6df0_at(c: &mut Cpu, pc: u32) { f_801f6df0_b(c, pc) }
#[inline(always)]
fn f_801f6df0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6df0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6df0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6df0 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801f6e14; { c.steps = st; c.instrs = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6e14;
            }
            0x801f6e14 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 2) { pc = 0x801f6e30; continue; }
                pc = 0x801f6e20;
            }
            0x801f6e20 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x801f6e30; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011b2c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6e30;
            }
            0x801f6e30 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6df0: bad pc {:#x}", pc),
        }
    }
}

/// fightSideCheckWriteJoutaiDataId (0x801f6e44, 84 bytes)
#[inline(never)]
pub fn f_801f6e44(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6e44) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6e44); p = true; }
    }
    f_801f6e44_b(c, 0x801f6e44);
    if p { crate::prof::exit(c, 0x801f6e44); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6e44_at(c: &mut Cpu, pc: u32) { f_801f6e44_b(c, pc) }
#[inline(always)]
fn f_801f6e44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6e44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6e44, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6e44 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f6e64; { c.steps = st; c.instrs = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6e64;
            }
            0x801f6e64 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 2) { pc = 0x801f6e78; continue; }
                pc = 0x801f6e70;
            }
            0x801f6e70 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f6e84; continue;
                pc = 0x801f6e78;
            }
            0x801f6e78 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f6e84; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011b444(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6e84;
            }
            0x801f6e84 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6e44: bad pc {:#x}", pc),
        }
    }
}

/// fightSideIsJoutaiDataId (0x801f6e98, 84 bytes)
#[inline(never)]
pub fn f_801f6e98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6e98) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6e98); p = true; }
    }
    f_801f6e98_b(c, 0x801f6e98);
    if p { crate::prof::exit(c, 0x801f6e98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6e98_at(c: &mut Cpu, pc: u32) { f_801f6e98_b(c, pc) }
#[inline(always)]
fn f_801f6e98_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6e98;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6e98, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6e98 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f6eb8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6eb8;
            }
            0x801f6eb8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 2) { pc = 0x801f6ecc; continue; }
                pc = 0x801f6ec4;
            }
            0x801f6ec4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f6ed8; continue;
                pc = 0x801f6ecc;
            }
            0x801f6ecc => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f6ed8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6ed8;
            }
            0x801f6ed8 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6e98: bad pc {:#x}", pc),
        }
    }
}

/// fightSideInitJoutaiDataId (0x801f6eec, 76 bytes)
#[inline(never)]
pub fn f_801f6eec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6eec) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6eec); p = true; }
    }
    f_801f6eec_b(c, 0x801f6eec);
    if p { crate::prof::exit(c, 0x801f6eec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6eec_at(c: &mut Cpu, pc: u32) { f_801f6eec_b(c, pc) }
#[inline(always)]
fn f_801f6eec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6eec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6eec, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6eec => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f6f0c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6f0c;
            }
            0x801f6f0c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 2) { pc = 0x801f6f24; continue; }
                pc = 0x801f6f18;
            }
            0x801f6f18 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f6f24; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011b788(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6f24;
            }
            0x801f6f24 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6eec: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetHikaeFightPokemonNum (0x801f6f38, 156 bytes)
#[inline(never)]
pub fn f_801f6f38(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6f38) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6f38); p = true; }
    }
    f_801f6f38_b(c, 0x801f6f38);
    if p { crate::prof::exit(c, 0x801f6f38); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6f38_at(c: &mut Cpu, pc: u32) { f_801f6f38_b(c, pc) }
#[inline(always)]
fn f_801f6f38_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6f38;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6f38, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6f38 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.r[26] = c.r[5] | c.r[5];
                c.r[27] = c.r[6] | c.r[6];
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.r[28] = 0x0;
                c.r[29] = 0x0;
                pc = 0x801f6fb0; continue;
                pc = 0x801f6f64;
            }
            0x801f6f64 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[25], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f6f74; continue; }
                pc = 0x801f6f6c;
            }
            0x801f6f6c => { ins += 2;
                c.r[30] = 0x0;
                pc = 0x801f6f84; continue;
                pc = 0x801f6f74;
            }
            0x801f6f74 => { ins += 3;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x801f6f80; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6f80;
            }
            0x801f6f80 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x801f6f84;
            }
            0x801f6f84 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801f6f8c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fa634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6f8c;
            }
            0x801f6f8c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f6fac; continue; }
                pc = 0x801f6f94;
            }
            0x801f6f94 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.lr = 0x801f6fa4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f81f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f6fa4;
            }
            0x801f6fa4 => { ins += 2;
                c.r[0] = c.r[28].wrapping_add(c.r[3]);
                c.r[28] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x801f6fac;
            }
            0x801f6fac => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f6fb0;
            }
            0x801f6fb0 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x801f6f64; continue; }
                pc = 0x801f6fbc;
            }
            0x801f6fbc => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6f38: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetFightPokemonNum (0x801f6fd4, 188 bytes)
#[inline(never)]
pub fn f_801f6fd4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f6fd4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f6fd4); p = true; }
    }
    f_801f6fd4_b(c, 0x801f6fd4);
    if p { crate::prof::exit(c, 0x801f6fd4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f6fd4_at(c: &mut Cpu, pc: u32) { f_801f6fd4_b(c, pc) }
#[inline(always)]
fn f_801f6fd4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f6fd4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f6fd4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f6fd4 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = 0x0;
                c.r[24] = c.r[3] | c.r[3];
                c.r[25] = c.r[5] | c.r[5];
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.r[27] = c.r[29] | c.r[29];
                pc = 0x801f706c; continue;
                pc = 0x801f6ffc;
            }
            0x801f6ffc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[24], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f700c; continue; }
                pc = 0x801f7004;
            }
            0x801f7004 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f7018; continue;
                pc = 0x801f700c;
            }
            0x801f700c => { ins += 3;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x801f7018; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7018;
            }
            0x801f7018 => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x801f7020; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fa634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7020;
            }
            0x801f7020 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7068; continue; }
                pc = 0x801f7028;
            }
            0x801f7028 => { ins += 3;
                c.r[30] = c.r[25].rotate_left(0) & 0xffff;
                c.r[28] = 0x0;
                pc = 0x801f705c; continue;
                pc = 0x801f7034;
            }
            0x801f7034 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x801f7040; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f986c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7040;
            }
            0x801f7040 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f7058; continue; }
                pc = 0x801f7048;
            }
            0x801f7048 => { ins += 1;
                c.lr = 0x801f704c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f704c;
            }
            0x801f704c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7058; continue; }
                pc = 0x801f7054;
            }
            0x801f7054 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f7058;
            }
            0x801f7058 => { ins += 1;
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x801f705c;
            }
            0x801f705c => { ins += 3;
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[30]);
                if cr_bit(cr, 0) { pc = 0x801f7034; continue; }
                pc = 0x801f7068;
            }
            0x801f7068 => { ins += 1;
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x801f706c;
            }
            0x801f706c => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x801f6ffc; continue; }
                pc = 0x801f7078;
            }
            0x801f7078 => { ins += 6;
                c.r[3] = c.r[29].rotate_left(0) & 0xffff;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f6fd4: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetFightPokemonMaxHp (0x801f7090, 228 bytes)
#[inline(never)]
pub fn f_801f7090(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7090) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7090); p = true; }
    }
    f_801f7090_b(c, 0x801f7090);
    if p { crate::prof::exit(c, 0x801f7090); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7090_at(c: &mut Cpu, pc: u32) { f_801f7090_b(c, pc) }
#[inline(always)]
fn f_801f7090_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7090;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7090, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7090 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[4].rotate_left(0) & 0xffff;
                c.r[31] = 0x0;
                c.r[23] = 0x0;
                pc = 0x801f7150; continue;
                pc = 0x801f70b8;
            }
            0x801f70b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f70c8; continue; }
                pc = 0x801f70c0;
            }
            0x801f70c0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f70d4; continue;
                pc = 0x801f70c8;
            }
            0x801f70c8 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x801f70d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f70d4;
            }
            0x801f70d4 => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x801f70dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fa634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f70dc;
            }
            0x801f70dc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f714c; continue; }
                pc = 0x801f70e4;
            }
            0x801f70e4 => { ins += 3;
                c.r[27] = c.r[30].rotate_left(0) & 0xffff;
                c.r[22] = 0x0;
                pc = 0x801f7140; continue;
                pc = 0x801f70f0;
            }
            0x801f70f0 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[22] | c.r[22];
                c.lr = 0x801f70fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f986c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f70fc;
            }
            0x801f70fc => { ins += 2;
                c.r[25] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[25]);
                if cr_bit(cr, 2) { pc = 0x801f713c; continue; }
                pc = 0x801f7104;
            }
            0x801f7104 => { ins += 1;
                c.lr = 0x801f7108; { c.steps = st; c.instrs = ins; c.cr = cr; f_80205be8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7108;
            }
            0x801f7108 => { ins += 2;
                c.r[24] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[24]);
                if cr_bit(cr, 2) { pc = 0x801f713c; continue; }
                pc = 0x801f7110;
            }
            0x801f7110 => { ins += 2;
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x801f7118; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7118;
            }
            0x801f7118 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f713c; continue; }
                pc = 0x801f7120;
            }
            0x801f7120 => { ins += 5;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x801f7134; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7134;
            }
            0x801f7134 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[31] = c.r[31].wrapping_add(c.r[0]);
                pc = 0x801f713c;
            }
            0x801f713c => { ins += 1;
                c.r[22] = c.r[22].wrapping_add(0x1);
                pc = 0x801f7140;
            }
            0x801f7140 => { ins += 3;
                c.r[0] = c.r[22].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[27]);
                if cr_bit(cr, 0) { pc = 0x801f70f0; continue; }
                pc = 0x801f714c;
            }
            0x801f714c => { ins += 1;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x801f7150;
            }
            0x801f7150 => { ins += 3;
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x801f70b8; continue; }
                pc = 0x801f715c;
            }
            0x801f715c => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 22..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7090: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetFightPokemonNokoriHp (0x801f7174, 228 bytes)
#[inline(never)]
pub fn f_801f7174(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7174) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7174); p = true; }
    }
    f_801f7174_b(c, 0x801f7174);
    if p { crate::prof::exit(c, 0x801f7174); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7174_at(c: &mut Cpu, pc: u32) { f_801f7174_b(c, pc) }
#[inline(always)]
fn f_801f7174_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7174;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7174, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7174 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[5] | c.r[5];
                c.r[28] = c.r[4].rotate_left(0) & 0xffff;
                c.r[31] = 0x0;
                c.r[23] = 0x0;
                pc = 0x801f7234; continue;
                pc = 0x801f719c;
            }
            0x801f719c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f71ac; continue; }
                pc = 0x801f71a4;
            }
            0x801f71a4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f71b8; continue;
                pc = 0x801f71ac;
            }
            0x801f71ac => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x801f71b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f71b8;
            }
            0x801f71b8 => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x801f71c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fa634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f71c0;
            }
            0x801f71c0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7230; continue; }
                pc = 0x801f71c8;
            }
            0x801f71c8 => { ins += 3;
                c.r[27] = c.r[30].rotate_left(0) & 0xffff;
                c.r[22] = 0x0;
                pc = 0x801f7224; continue;
                pc = 0x801f71d4;
            }
            0x801f71d4 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[22] | c.r[22];
                c.lr = 0x801f71e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f986c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f71e0;
            }
            0x801f71e0 => { ins += 2;
                c.r[25] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[25]);
                if cr_bit(cr, 2) { pc = 0x801f7220; continue; }
                pc = 0x801f71e8;
            }
            0x801f71e8 => { ins += 1;
                c.lr = 0x801f71ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_80205be8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f71ec;
            }
            0x801f71ec => { ins += 2;
                c.r[24] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[24]);
                if cr_bit(cr, 2) { pc = 0x801f7220; continue; }
                pc = 0x801f71f4;
            }
            0x801f71f4 => { ins += 2;
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x801f71fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f71fc;
            }
            0x801f71fc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7220; continue; }
                pc = 0x801f7204;
            }
            0x801f7204 => { ins += 5;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x801f7218; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7218;
            }
            0x801f7218 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[31] = c.r[31].wrapping_add(c.r[0]);
                pc = 0x801f7220;
            }
            0x801f7220 => { ins += 1;
                c.r[22] = c.r[22].wrapping_add(0x1);
                pc = 0x801f7224;
            }
            0x801f7224 => { ins += 3;
                c.r[0] = c.r[22].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[27]);
                if cr_bit(cr, 0) { pc = 0x801f71d4; continue; }
                pc = 0x801f7230;
            }
            0x801f7230 => { ins += 1;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x801f7234;
            }
            0x801f7234 => { ins += 3;
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x801f719c; continue; }
                pc = 0x801f7240;
            }
            0x801f7240 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 22..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7174: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetValidFightTrainerPtr (0x801f7258, 88 bytes)
#[inline(never)]
pub fn f_801f7258(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7258) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7258); p = true; }
    }
    f_801f7258_b(c, 0x801f7258);
    if p { crate::prof::exit(c, 0x801f7258); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7258_at(c: &mut Cpu, pc: u32) { f_801f7258_b(c, pc) }
#[inline(always)]
fn f_801f7258_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7258;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7258, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7258 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x801f7278; continue; }
                pc = 0x801f7270;
            }
            0x801f7270 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x801f7280; continue;
                pc = 0x801f7278;
            }
            0x801f7278 => { ins += 1;
                c.lr = 0x801f727c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f727c;
            }
            0x801f727c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x801f7280;
            }
            0x801f7280 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f7288; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fa634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7288;
            }
            0x801f7288 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f7298; continue; }
                pc = 0x801f7290;
            }
            0x801f7290 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f729c; continue;
                pc = 0x801f7298;
            }
            0x801f7298 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f729c;
            }
            0x801f729c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7258: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetFightTrainerGridParam (0x801f72b0, 216 bytes)
#[inline(never)]
pub fn f_801f72b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f72b0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f72b0); p = true; }
    }
    f_801f72b0_b(c, 0x801f72b0);
    if p { crate::prof::exit(c, 0x801f72b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f72b0_at(c: &mut Cpu, pc: u32) { f_801f72b0_b(c, pc) }
#[inline(always)]
fn f_801f72b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f72b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f72b0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f72b0 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[6] | c.r[6];
                cr_rc(&mut cr, c.xer, c.r[30]);
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[31] = c.r[7] | c.r[7];
                if cr_bit(cr, 2) { pc = 0x801f7374; continue; }
                pc = 0x801f72d4;
            }
            0x801f72d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f7374; continue; }
                pc = 0x801f72dc;
            }
            0x801f72dc => { ins += 1;
                c.lr = 0x801f72e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f7870(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f72e0;
            }
            0x801f72e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f72f0; continue; }
                pc = 0x801f72e8;
            }
            0x801f72e8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f72f8; continue;
                pc = 0x801f72f0;
            }
            0x801f72f0 => { ins += 1;
                c.lr = 0x801f72f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f7858(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f72f4;
            }
            0x801f72f4 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801f72f8;
            }
            0x801f72f8 => { ins += 6;
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.stb(c.r[30], c.r[3]);
                c.r[3] = 0x0;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                c.stb(c.r[31], c.r[3]);
                if cr_bit(cr, 2) { pc = 0x801f7374; continue; }
                pc = 0x801f7310;
            }
            0x801f7310 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x801f7374; continue; }
                pc = 0x801f7318;
            }
            0x801f7318 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[30]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f7350; continue; }
                pc = 0x801f7324;
            }
            0x801f7324 => { ins += 2;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f7338; continue; }
                pc = 0x801f732c;
            }
            0x801f732c => { ins += 3;
                c.r[0] = 0x1;
                c.stb(c.r[31], c.r[0]);
                pc = 0x801f7374; continue;
                pc = 0x801f7338;
            }
            0x801f7338 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f7374; continue; }
                pc = 0x801f7340;
            }
            0x801f7340 => { ins += 3;
                c.r[0] = 0xffffffff;
                c.stb(c.r[31], c.r[0]);
                pc = 0x801f7374; continue;
                pc = 0x801f734c;
            }
            0x801f734c => { ins += 1;
                pc = 0x801f7374; continue;
                pc = 0x801f7350;
            }
            0x801f7350 => { ins += 2;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f7364; continue; }
                pc = 0x801f7358;
            }
            0x801f7358 => { ins += 3;
                c.r[0] = 0xffffffff;
                c.stb(c.r[31], c.r[0]);
                pc = 0x801f7374; continue;
                pc = 0x801f7364;
            }
            0x801f7364 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f7374; continue; }
                pc = 0x801f736c;
            }
            0x801f736c => { ins += 2;
                c.r[0] = 0x1;
                c.stb(c.r[31], c.r[0]);
                pc = 0x801f7374;
            }
            0x801f7374 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f72b0: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetDoFightTrainerCount (0x801f7388, 124 bytes)
#[inline(never)]
pub fn f_801f7388(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7388) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7388); p = true; }
    }
    f_801f7388_b(c, 0x801f7388);
    if p { crate::prof::exit(c, 0x801f7388); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7388_at(c: &mut Cpu, pc: u32) { f_801f7388_b(c, pc) }
#[inline(always)]
fn f_801f7388_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7388;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7388, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7388 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = 0x0;
                c.r[31] = 0x0;
                pc = 0x801f73e0; continue;
                pc = 0x801f73a8;
            }
            0x801f73a8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f73b8; continue; }
                pc = 0x801f73b0;
            }
            0x801f73b0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f73c4; continue;
                pc = 0x801f73b8;
            }
            0x801f73b8 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.lr = 0x801f73c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f73c4;
            }
            0x801f73c4 => { ins += 1;
                c.lr = 0x801f73c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fa524(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f73c8;
            }
            0x801f73c8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f73dc; continue; }
                pc = 0x801f73d0;
            }
            0x801f73d0 => { ins += 3;
                c.r[3] = c.r[30].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[30] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x801f73dc;
            }
            0x801f73dc => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f73e0;
            }
            0x801f73e0 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f73a8; continue; }
                pc = 0x801f73ec;
            }
            0x801f73ec => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7388: bad pc {:#x}", pc),
        }
    }
}

/// fightSideCheckValid (0x801f7404, 124 bytes)
#[inline(never)]
pub fn f_801f7404(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7404) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7404); p = true; }
    }
    f_801f7404_b(c, 0x801f7404);
    if p { crate::prof::exit(c, 0x801f7404); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7404_at(c: &mut Cpu, pc: u32) { f_801f7404_b(c, pc) }
#[inline(always)]
fn f_801f7404_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7404;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7404, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7404 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x801f7424; continue; }
                pc = 0x801f741c;
            }
            0x801f741c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f746c; continue;
                pc = 0x801f7424;
            }
            0x801f7424 => { ins += 1;
                c.lr = 0x801f7428; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7428;
            }
            0x801f7428 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f743c; continue; }
                pc = 0x801f7434;
            }
            0x801f7434 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f746c; continue;
                pc = 0x801f743c;
            }
            0x801f743c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f744c; continue; }
                pc = 0x801f7444;
            }
            0x801f7444 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f7458; continue;
                pc = 0x801f744c;
            }
            0x801f744c => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f7454; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f793c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7454;
            }
            0x801f7454 => { ins += 1;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801f7458;
            }
            0x801f7458 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f7468; continue; }
                pc = 0x801f7460;
            }
            0x801f7460 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f746c; continue;
                pc = 0x801f7468;
            }
            0x801f7468 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x801f746c;
            }
            0x801f746c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7404: bad pc {:#x}", pc),
        }
    }
}

/// fightSideCreate (0x801f7480, 176 bytes)
#[inline(never)]
pub fn f_801f7480(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7480) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7480); p = true; }
    }
    f_801f7480_b(c, 0x801f7480);
    if p { crate::prof::exit(c, 0x801f7480); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7480_at(c: &mut Cpu, pc: u32) { f_801f7480_b(c, pc) }
#[inline(always)]
fn f_801f7480_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7480;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7480, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7480 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                c.r[31] = c.r[4] | c.r[4];
                if cr_bit(cr, 2) { pc = 0x801f751c; continue; }
                pc = 0x801f749c;
            }
            0x801f749c => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801f7508; continue; }
                pc = 0x801f74a0;
            }
            0x801f74a0 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801f74ac; continue; }
                pc = 0x801f74a4;
            }
            0x801f74a4 => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x801f74ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f74ac;
            }
            0x801f74ac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f74bc; continue; }
                pc = 0x801f74b4;
            }
            0x801f74b4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f74c8; continue;
                pc = 0x801f74bc;
            }
            0x801f74bc => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x801f74c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f7908(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f74c8;
            }
            0x801f74c8 => { ins += 2;
                c.r[4] = 0x6;
                c.lr = 0x801f74d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011b950(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f74d0;
            }
            0x801f74d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f74e0; continue; }
                pc = 0x801f74d8;
            }
            0x801f74d8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f74ec; continue;
                pc = 0x801f74e0;
            }
            0x801f74e0 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x801f74ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f74ec;
            }
            0x801f74ec => { ins += 2;
                c.r[4] = 0x2;
                c.lr = 0x801f74f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fa8cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f74f4;
            }
            0x801f74f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f7508; continue; }
                pc = 0x801f74fc;
            }
            0x801f74fc => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x801f7508; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f789c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7508;
            }
            0x801f7508 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f751c; continue; }
                pc = 0x801f7510;
            }
            0x801f7510 => { ins += 3;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801f751c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f751c;
            }
            0x801f751c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7480: bad pc {:#x}", pc),
        }
    }
}

/// fightSideInitAry (0x801f7530, 200 bytes)
#[inline(never)]
pub fn f_801f7530(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7530) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7530); p = true; }
    }
    f_801f7530_b(c, 0x801f7530);
    if p { crate::prof::exit(c, 0x801f7530); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7530_at(c: &mut Cpu, pc: u32) { f_801f7530_b(c, pc) }
#[inline(always)]
fn f_801f7530_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7530;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7530, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7530 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if cr_bit(cr, 2) { pc = 0x801f75e4; continue; }
                pc = 0x801f7548;
            }
            0x801f7548 => { ins += 3;
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.r[29] = 0x0;
                pc = 0x801f75d8; continue;
                pc = 0x801f7554;
            }
            0x801f7554 => { ins += 4;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].wrapping_mul(0x5230);
                c.r[30] = c.r[28].wrapping_add(c.r[0]);
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x801f75d4; continue; }
                pc = 0x801f7564;
            }
            0x801f7564 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f7578; continue; }
                pc = 0x801f756c;
            }
            0x801f756c => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x801f7578; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7578;
            }
            0x801f7578 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f7588; continue; }
                pc = 0x801f7580;
            }
            0x801f7580 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f7594; continue;
                pc = 0x801f7588;
            }
            0x801f7588 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x801f7594; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f7908(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7594;
            }
            0x801f7594 => { ins += 2;
                c.r[4] = 0x6;
                c.lr = 0x801f759c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011b950(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f759c;
            }
            0x801f759c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f75ac; continue; }
                pc = 0x801f75a4;
            }
            0x801f75a4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f75b8; continue;
                pc = 0x801f75ac;
            }
            0x801f75ac => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x801f75b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f75b8;
            }
            0x801f75b8 => { ins += 2;
                c.r[4] = 0x2;
                c.lr = 0x801f75c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fa8cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f75c0;
            }
            0x801f75c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f75d4; continue; }
                pc = 0x801f75c8;
            }
            0x801f75c8 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x801f75d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f789c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f75d4;
            }
            0x801f75d4 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f75d8;
            }
            0x801f75d8 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x801f7554; continue; }
                pc = 0x801f75e4;
            }
            0x801f75e4 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7530: bad pc {:#x}", pc),
        }
    }
}

/// fightSideSetStatus (0x801f75f8, 192 bytes)
#[inline(never)]
pub fn f_801f75f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f75f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f75f8); p = true; }
    }
    f_801f75f8_b(c, 0x801f75f8);
    if p { crate::prof::exit(c, 0x801f75f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f75f8_at(c: &mut Cpu, pc: u32) { f_801f75f8_b(c, pc) }
#[inline(never)]
fn f_801f75f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f75f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f75f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f75f8 => { ins += 1;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x801f75fc;
            }
            0x801f75fc => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x801f7600;
            }
            0x801f7600 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x801f7604;
            }
            0x801f7604 => { ins += 1;
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x801f7608;
            }
            0x801f7608 => { ins += 1;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x801f760c;
            }
            0x801f760c => { ins += 1;
                c.r[29] = c.r[5] | c.r[5];
                pc = 0x801f7610;
            }
            0x801f7610 => { ins += 1;
                c.r[30] = c.r[6] | c.r[6];
                pc = 0x801f7614;
            }
            0x801f7614 => { ins += 1;
                c.r[31] = c.r[7] | c.r[7];
                pc = 0x801f7618;
            }
            0x801f7618 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801f76a4; continue; }
                pc = 0x801f761c;
            }
            0x801f761c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xa);
                pc = 0x801f7620;
            }
            0x801f7620 => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x801f7628; continue; }
                pc = 0x801f7624;
            }
            0x801f7624 => { ins += 1;
                pc = 0x801f76a4; continue;
                pc = 0x801f7628;
            }
            0x801f7628 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                pc = 0x801f762c;
            }
            0x801f762c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801f7638; continue; }
                pc = 0x801f7630;
            }
            0x801f7630 => { ins += 1;
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x801f7634;
            }
            0x801f7634 => { ins += 1;
                c.lr = 0x801f7638; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f7870(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7638;
            }
            0x801f7638 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801f763c;
            }
            0x801f763c => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801f76a4; continue; }
                pc = 0x801f7640;
            }
            0x801f7640 => { ins += 1;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                pc = 0x801f7644;
            }
            0x801f7644 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                pc = 0x801f7648;
            }
            0x801f7648 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x801f76a4; continue; }
                pc = 0x801f764c;
            }
            0x801f764c => { ins += 1;
                c.r[4] = 0x80370000;
                pc = 0x801f7650;
            }
            0x801f7650 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x801f7654;
            }
            0x801f7654 => { ins += 1;
                c.r[4] = c.r[4].wrapping_add(0x5628);
                pc = 0x801f7658;
            }
            0x801f7658 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(c.r[0]));
                pc = 0x801f765c;
            }
            0x801f765c => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x801f7660;
            }
            0x801f7660 => { ins += 1;
                if true { let t = c.ctr; if (0x801f75f8..0x801f76b8).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f7664;
            }
            0x801f7664 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801f7668;
            }
            0x801f7668 => { ins += 1;
                c.lr = 0x801f766c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f77e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f766c;
            }
            0x801f766c => { ins += 1;
                pc = 0x801f76a4; continue;
                pc = 0x801f7670;
            }
            0x801f7670 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801f7674;
            }
            0x801f7674 => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801f7678;
            }
            0x801f7678 => { ins += 1;
                c.lr = 0x801f767c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f77bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f767c;
            }
            0x801f767c => { ins += 1;
                pc = 0x801f76a4; continue;
                pc = 0x801f7680;
            }
            0x801f7680 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801f7684;
            }
            0x801f7684 => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801f7688;
            }
            0x801f7688 => { ins += 1;
                c.lr = 0x801f768c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f7798(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f768c;
            }
            0x801f768c => { ins += 1;
                pc = 0x801f76a4; continue;
                pc = 0x801f7690;
            }
            0x801f7690 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801f7694;
            }
            0x801f7694 => { ins += 1;
                c.lr = 0x801f7698; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7698;
            }
            0x801f7698 => { ins += 1;
                pc = 0x801f76a4; continue;
                pc = 0x801f769c;
            }
            0x801f769c => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801f76a0;
            }
            0x801f76a0 => { ins += 1;
                c.lr = 0x801f76a4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f789c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f76a4;
            }
            0x801f76a4 => { ins += 1;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                pc = 0x801f76a8;
            }
            0x801f76a8 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                pc = 0x801f76ac;
            }
            0x801f76ac => { ins += 1;
                c.mtspr(8, c.r[0]);
                pc = 0x801f76b0;
            }
            0x801f76b0 => { ins += 1;
                c.r[1] = c.r[1].wrapping_add(0x20);
                pc = 0x801f76b4;
            }
            0x801f76b4 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f75f8: bad pc {:#x}", pc),
        }
    }
}

/// fightSideGetStatus (0x801f76b8, 224 bytes)
#[inline(never)]
pub fn f_801f76b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f76b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f76b8); p = true; }
    }
    f_801f76b8_b(c, 0x801f76b8);
    if p { crate::prof::exit(c, 0x801f76b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f76b8_at(c: &mut Cpu, pc: u32) { f_801f76b8_b(c, pc) }
#[inline(never)]
fn f_801f76b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f76b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f76b8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f76b8 => { ins += 1;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x801f76bc;
            }
            0x801f76bc => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x801f76c0;
            }
            0x801f76c0 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                pc = 0x801f76c4;
            }
            0x801f76c4 => { ins += 1;
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x801f76c8;
            }
            0x801f76c8 => { ins += 1;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x801f76cc;
            }
            0x801f76cc => { ins += 1;
                c.r[30] = c.r[5] | c.r[5];
                pc = 0x801f76d0;
            }
            0x801f76d0 => { ins += 1;
                c.r[31] = c.r[6] | c.r[6];
                pc = 0x801f76d4;
            }
            0x801f76d4 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801f76e0; continue; }
                pc = 0x801f76d8;
            }
            0x801f76d8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xa);
                pc = 0x801f76dc;
            }
            0x801f76dc => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x801f76e8; continue; }
                pc = 0x801f76e0;
            }
            0x801f76e0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f76e4;
            }
            0x801f76e4 => { ins += 1;
                pc = 0x801f7784; continue;
                pc = 0x801f76e8;
            }
            0x801f76e8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                pc = 0x801f76ec;
            }
            0x801f76ec => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801f76f8; continue; }
                pc = 0x801f76f0;
            }
            0x801f76f0 => { ins += 1;
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x801f76f4;
            }
            0x801f76f4 => { ins += 1;
                c.lr = 0x801f76f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f7870(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f76f8;
            }
            0x801f76f8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801f76fc;
            }
            0x801f76fc => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801f7708; continue; }
                pc = 0x801f7700;
            }
            0x801f7700 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f7704;
            }
            0x801f7704 => { ins += 1;
                pc = 0x801f7784; continue;
                pc = 0x801f7708;
            }
            0x801f7708 => { ins += 1;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                pc = 0x801f770c;
            }
            0x801f770c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                pc = 0x801f7710;
            }
            0x801f7710 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x801f7780; continue; }
                pc = 0x801f7714;
            }
            0x801f7714 => { ins += 1;
                c.r[4] = 0x80370000;
                pc = 0x801f7718;
            }
            0x801f7718 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x801f771c;
            }
            0x801f771c => { ins += 1;
                c.r[4] = c.r[4].wrapping_add(0x564c);
                pc = 0x801f7720;
            }
            0x801f7720 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(c.r[0]));
                pc = 0x801f7724;
            }
            0x801f7724 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x801f7728;
            }
            0x801f7728 => { ins += 1;
                if true { let t = c.ctr; if (0x801f76b8..0x801f7798).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f772c;
            }
            0x801f772c => { ins += 1;
                c.lr = 0x801f7730; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f7858(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7730;
            }
            0x801f7730 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801f7734;
            }
            0x801f7734 => { ins += 1;
                pc = 0x801f7784; continue;
                pc = 0x801f7738;
            }
            0x801f7738 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801f773c;
            }
            0x801f773c => { ins += 1;
                c.lr = 0x801f7740; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f7824(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7740;
            }
            0x801f7740 => { ins += 1;
                pc = 0x801f7784; continue;
                pc = 0x801f7744;
            }
            0x801f7744 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801f7748;
            }
            0x801f7748 => { ins += 1;
                c.lr = 0x801f774c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f77f0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f774c;
            }
            0x801f774c => { ins += 1;
                pc = 0x801f7784; continue;
                pc = 0x801f7750;
            }
            0x801f7750 => { ins += 1;
                c.lr = 0x801f7754; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f793c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7754;
            }
            0x801f7754 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801f7758;
            }
            0x801f7758 => { ins += 1;
                pc = 0x801f7784; continue;
                pc = 0x801f775c;
            }
            0x801f775c => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801f7760;
            }
            0x801f7760 => { ins += 1;
                c.lr = 0x801f7764; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f7908(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7764;
            }
            0x801f7764 => { ins += 1;
                pc = 0x801f7784; continue;
                pc = 0x801f7768;
            }
            0x801f7768 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801f776c;
            }
            0x801f776c => { ins += 1;
                c.lr = 0x801f7770; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7770;
            }
            0x801f7770 => { ins += 1;
                pc = 0x801f7784; continue;
                pc = 0x801f7774;
            }
            0x801f7774 => { ins += 1;
                c.lr = 0x801f7778; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f78bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7778;
            }
            0x801f7778 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801f777c;
            }
            0x801f777c => { ins += 1;
                pc = 0x801f7784; continue;
                pc = 0x801f7780;
            }
            0x801f7780 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f7784;
            }
            0x801f7784 => { ins += 1;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                pc = 0x801f7788;
            }
            0x801f7788 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x801f778c;
            }
            0x801f778c => { ins += 1;
                c.mtspr(8, c.r[0]);
                pc = 0x801f7790;
            }
            0x801f7790 => { ins += 1;
                c.r[1] = c.r[1].wrapping_add(0x10);
                pc = 0x801f7794;
            }
            0x801f7794 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f76b8: bad pc {:#x}", pc),
        }
    }
}

/// fightSideDataBiosSetFightoutPokemonStatusMenuDataId (0x801f7798, 36 bytes)
#[inline(never)]
pub fn f_801f7798(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7798) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7798); p = true; }
    }
    f_801f7798_b(c, 0x801f7798);
    if p { crate::prof::exit(c, 0x801f7798); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7798_at(c: &mut Cpu, pc: u32) { f_801f7798_b(c, pc) }
#[inline(always)]
fn f_801f7798_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7798;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7798, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7798 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f77a0;
            }
            0x801f77a0 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f77ac;
            }
            0x801f77ac => { ins += 4;
                c.r[0] = c.r[4].rotate_left(2) & 0x3fc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7798: bad pc {:#x}", pc),
        }
    }
}

/// fightSideDataBiosSetFightTrainerStatusMenuDataId (0x801f77bc, 36 bytes)
#[inline(never)]
pub fn f_801f77bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f77bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801f77bc); p = true; }
    }
    f_801f77bc_b(c, 0x801f77bc);
    if p { crate::prof::exit(c, 0x801f77bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f77bc_at(c: &mut Cpu, pc: u32) { f_801f77bc_b(c, pc) }
#[inline(always)]
fn f_801f77bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f77bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f77bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f77bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f77c4;
            }
            0x801f77c4 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f77d0;
            }
            0x801f77d0 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(2) & 0x3fc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f77bc: bad pc {:#x}", pc),
        }
    }
}

/// fightSideDataBiosSetYrot (0x801f77e0, 16 bytes)
#[inline(never)]
pub fn f_801f77e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f77e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f77e0); p = true; }
    }
    f_801f77e0_b(c, 0x801f77e0);
    if p { crate::prof::exit(c, 0x801f77e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f77e0_at(c: &mut Cpu, pc: u32) { f_801f77e0_b(c, pc) }
#[inline(always)]
fn f_801f77e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f77e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f77e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f77e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f77e8;
            }
            0x801f77e8 => { ins += 2;
                c.sth(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f77e0: bad pc {:#x}", pc),
        }
    }
}

/// fightSideDataBiosGetFightoutPokemonStatusMenuDataId (0x801f77f0, 52 bytes)
#[inline(never)]
pub fn f_801f77f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f77f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f77f0); p = true; }
    }
    f_801f77f0_b(c, 0x801f77f0);
    if p { crate::prof::exit(c, 0x801f77f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f77f0_at(c: &mut Cpu, pc: u32) { f_801f77f0_b(c, pc) }
#[inline(always)]
fn f_801f77f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f77f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f77f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f77f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f7800; continue; }
                pc = 0x801f77f8;
            }
            0x801f77f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f7800;
            }
            0x801f7800 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f7814; continue; }
                pc = 0x801f780c;
            }
            0x801f780c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f7814;
            }
            0x801f7814 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(2) & 0x3fc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f77f0: bad pc {:#x}", pc),
        }
    }
}

/// fightSideDataBiosGetFightTrainerStatusMenuDataId (0x801f7824, 52 bytes)
#[inline(never)]
pub fn f_801f7824(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7824) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7824); p = true; }
    }
    f_801f7824_b(c, 0x801f7824);
    if p { crate::prof::exit(c, 0x801f7824); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7824_at(c: &mut Cpu, pc: u32) { f_801f7824_b(c, pc) }
#[inline(always)]
fn f_801f7824_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7824;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7824, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7824 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f7834; continue; }
                pc = 0x801f782c;
            }
            0x801f782c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f7834;
            }
            0x801f7834 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f7848; continue; }
                pc = 0x801f7840;
            }
            0x801f7840 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f7848;
            }
            0x801f7848 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(2) & 0x3fc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7824: bad pc {:#x}", pc),
        }
    }
}

/// fightSideDataBiosGetYrot (0x801f7858, 24 bytes)
#[inline(never)]
pub fn f_801f7858(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7858) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7858); p = true; }
    }
    f_801f7858_b(c, 0x801f7858);
    if p { crate::prof::exit(c, 0x801f7858); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7858_at(c: &mut Cpu, pc: u32) { f_801f7858_b(c, pc) }
#[inline(always)]
fn f_801f7858_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7858;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7858, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7858 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f7868; continue; }
                pc = 0x801f7860;
            }
            0x801f7860 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f7868;
            }
            0x801f7868 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7858: bad pc {:#x}", pc),
        }
    }
}

/// fightSideDataBiosGetPtr (0x801f7870, 44 bytes)
#[inline(never)]
pub fn f_801f7870(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7870) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7870); p = true; }
    }
    f_801f7870_b(c, 0x801f7870);
    if p { crate::prof::exit(c, 0x801f7870); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7870_at(c: &mut Cpu, pc: u32) { f_801f7870_b(c, pc) }
#[inline(always)]
fn f_801f7870_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7870;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7870, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7870 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8718));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801f788c; continue; }
                pc = 0x801f7884;
            }
            0x801f7884 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f788c;
            }
            0x801f788c => { ins += 4;
                c.r[0] = c.r[3].wrapping_mul(0x14);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff871c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7870: bad pc {:#x}", pc),
        }
    }
}

/// fightSideBiosSetMakibisiCheckFlag (0x801f789c, 16 bytes)
#[inline(never)]
pub fn f_801f789c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f789c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f789c); p = true; }
    }
    f_801f789c_b(c, 0x801f789c);
    if p { crate::prof::exit(c, 0x801f789c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f789c_at(c: &mut Cpu, pc: u32) { f_801f789c_b(c, pc) }
#[inline(always)]
fn f_801f789c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f789c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f789c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f789c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f78a4;
            }
            0x801f78a4 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x522c), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f789c: bad pc {:#x}", pc),
        }
    }
}

/// fightSideBiosSetFightSideDataId (0x801f78ac, 16 bytes)
#[inline(never)]
pub fn f_801f78ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f78ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801f78ac); p = true; }
    }
    f_801f78ac_b(c, 0x801f78ac);
    if p { crate::prof::exit(c, 0x801f78ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f78ac_at(c: &mut Cpu, pc: u32) { f_801f78ac_b(c, pc) }
#[inline(always)]
fn f_801f78ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f78ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f78ac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f78ac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f78b4;
            }
            0x801f78b4 => { ins += 2;
                c.sth(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f78ac: bad pc {:#x}", pc),
        }
    }
}

/// fightSideBiosGetMakibisiCheckFlag (0x801f78bc, 24 bytes)
#[inline(never)]
pub fn f_801f78bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f78bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801f78bc); p = true; }
    }
    f_801f78bc_b(c, 0x801f78bc);
    if p { crate::prof::exit(c, 0x801f78bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f78bc_at(c: &mut Cpu, pc: u32) { f_801f78bc_b(c, pc) }
#[inline(always)]
fn f_801f78bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f78bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f78bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f78bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f78cc; continue; }
                pc = 0x801f78c4;
            }
            0x801f78c4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f78cc;
            }
            0x801f78cc => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x522c));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f78bc: bad pc {:#x}", pc),
        }
    }
}

/// fightSideBiosGetFightTrainerPtr (0x801f78d4, 52 bytes)
#[inline(never)]
pub fn f_801f78d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f78d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f78d4); p = true; }
    }
    f_801f78d4_b(c, 0x801f78d4);
    if p { crate::prof::exit(c, 0x801f78d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f78d4_at(c: &mut Cpu, pc: u32) { f_801f78d4_b(c, pc) }
#[inline(always)]
fn f_801f78d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f78d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f78d4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f78d4 => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x801f78e4; continue; }
                pc = 0x801f78dc;
            }
            0x801f78dc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f78e4;
            }
            0x801f78e4 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f78f8; continue; }
                pc = 0x801f78f0;
            }
            0x801f78f0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f78f8;
            }
            0x801f78f8 => { ins += 4;
                c.r[3] = c.r[0].wrapping_mul(0x28e4);
                c.r[3] = c.r[3].wrapping_add(0x64);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f78d4: bad pc {:#x}", pc),
        }
    }
}

/// fightSideBiosGetJoutaiPtr (0x801f7908, 52 bytes)
#[inline(never)]
pub fn f_801f7908(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7908) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7908); p = true; }
    }
    f_801f7908_b(c, 0x801f7908);
    if p { crate::prof::exit(c, 0x801f7908); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7908_at(c: &mut Cpu, pc: u32) { f_801f7908_b(c, pc) }
#[inline(always)]
fn f_801f7908_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7908;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7908, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7908 => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x801f7918; continue; }
                pc = 0x801f7910;
            }
            0x801f7910 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f7918;
            }
            0x801f7918 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f792c; continue; }
                pc = 0x801f7924;
            }
            0x801f7924 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f792c;
            }
            0x801f792c => { ins += 4;
                c.r[3] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7908: bad pc {:#x}", pc),
        }
    }
}

/// fightSideBiosGetFightSideDataId (0x801f793c, 24 bytes)
#[inline(never)]
pub fn f_801f793c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f793c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f793c); p = true; }
    }
    f_801f793c_b(c, 0x801f793c);
    if p { crate::prof::exit(c, 0x801f793c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f793c_at(c: &mut Cpu, pc: u32) { f_801f793c_b(c, pc) }
#[inline(always)]
fn f_801f793c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f793c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f793c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f793c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f794c; continue; }
                pc = 0x801f7944;
            }
            0x801f7944 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801f794c;
            }
            0x801f794c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f793c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerToMenuBallStatus (0x801f7954, 540 bytes)
#[inline(never)]
pub fn f_801f7954(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7954) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7954); p = true; }
    }
    f_801f7954_b(c, 0x801f7954);
    if p { crate::prof::exit(c, 0x801f7954); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7954_at(c: &mut Cpu, pc: u32) { f_801f7954_b(c, pc) }
#[inline(never)]
fn f_801f7954_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7954;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7954, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7954 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                c.r[29] = c.r[4] | c.r[4];
                if cr_bit(cr, 2) { pc = 0x801f7b5c; continue; }
                pc = 0x801f7970;
            }
            0x801f7970 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801f797c; continue; }
                pc = 0x801f7974;
            }
            0x801f7974 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f79f4; continue;
                pc = 0x801f797c;
            }
            0x801f797c => { ins += 1;
                c.lr = 0x801f7980; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7980;
            }
            0x801f7980 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f7994; continue; }
                pc = 0x801f798c;
            }
            0x801f798c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f79f4; continue;
                pc = 0x801f7994;
            }
            0x801f7994 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801f79a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f79a8;
            }
            0x801f79a8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f79b8; continue; }
                pc = 0x801f79b0;
            }
            0x801f79b0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f79f4; continue;
                pc = 0x801f79b8;
            }
            0x801f79b8 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801f79cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f79cc;
            }
            0x801f79cc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f79dc; continue; }
                pc = 0x801f79d4;
            }
            0x801f79d4 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f79f4; continue;
                pc = 0x801f79dc;
            }
            0x801f79dc => { ins += 1;
                c.lr = 0x801f79e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a130(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f79e0;
            }
            0x801f79e0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f79f0; continue; }
                pc = 0x801f79e8;
            }
            0x801f79e8 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f79f4; continue;
                pc = 0x801f79f0;
            }
            0x801f79f0 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x801f79f4;
            }
            0x801f79f4 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7b5c; continue; }
                pc = 0x801f79fc;
            }
            0x801f79fc => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801f7a10; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7a10;
            }
            0x801f7a10 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[3] = 0x0;
                pc = 0x801f7a2c; continue;
                pc = 0x801f7a20;
            }
            0x801f7a20 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.stb(c.r[29].wrapping_add(c.r[0]), c.r[3]);
                pc = 0x801f7a2c;
            }
            0x801f7a2c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f7a20; continue; }
                pc = 0x801f7a38;
            }
            0x801f7a38 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x801f7b50; continue;
                pc = 0x801f7a40;
            }
            0x801f7a40 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x3;
                c.lr = 0x801f7a50; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a5b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7a50;
            }
            0x801f7a50 => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x801f7a58; { c.steps = st; c.instrs = ins; c.cr = cr; f_80123fbc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7a58;
            }
            0x801f7a58 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7b4c; continue; }
                pc = 0x801f7a60;
            }
            0x801f7a60 => { ins += 2;
                c.r[25] = 0x0;
                pc = 0x801f7ab8; continue;
                pc = 0x801f7a68;
            }
            0x801f7a68 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801f7a7c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7a7c;
            }
            0x801f7a7c => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x801f7a84; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206a04(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7a84;
            }
            0x801f7a84 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7ab4; continue; }
                pc = 0x801f7a8c;
            }
            0x801f7a8c => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0xcb;
                c.r[6] = 0x0;
                c.lr = 0x801f7aa0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7aa0;
            }
            0x801f7aa0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f7ab4; continue; }
                pc = 0x801f7aa8;
            }
            0x801f7aa8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x801f7ab4; continue; }
                pc = 0x801f7ab0;
            }
            0x801f7ab0 => { ins += 1;
                pc = 0x801f7ac8; continue;
                pc = 0x801f7ab4;
            }
            0x801f7ab4 => { ins += 1;
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x801f7ab8;
            }
            0x801f7ab8 => { ins += 3;
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f7a68; continue; }
                pc = 0x801f7ac4;
            }
            0x801f7ac4 => { ins += 1;
                c.r[26] = 0x0;
                pc = 0x801f7ac8;
            }
            0x801f7ac8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f7b4c; continue; }
                pc = 0x801f7ad0;
            }
            0x801f7ad0 => { ins += 2;
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x801f7ad8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206a04(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7ad8;
            }
            0x801f7ad8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7b4c; continue; }
                pc = 0x801f7ae0;
            }
            0x801f7ae0 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0xd2;
                c.r[6] = 0x0;
                c.lr = 0x801f7af4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7af4;
            }
            0x801f7af4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                if cr_bit(cr, 2) { pc = 0x801f7b4c; continue; }
                pc = 0x801f7afc;
            }
            0x801f7afc => { ins += 2;
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x801f7b04; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7b04;
            }
            0x801f7b04 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f7b1c; continue; }
                pc = 0x801f7b0c;
            }
            0x801f7b0c => { ins += 4;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = 0x3;
                c.stb(c.r[29].wrapping_add(c.r[0]), c.r[3]);
                pc = 0x801f7b4c; continue;
                pc = 0x801f7b1c;
            }
            0x801f7b1c => { ins += 2;
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x801f7b24; { c.steps = st; c.instrs = ins; c.cr = cr; f_80205be8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7b24;
            }
            0x801f7b24 => { ins += 1;
                c.lr = 0x801f7b28; { c.steps = st; c.instrs = ins; c.cr = cr; f_80122ddc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7b28;
            }
            0x801f7b28 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f7b40; continue; }
                pc = 0x801f7b30;
            }
            0x801f7b30 => { ins += 4;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = 0x2;
                c.stb(c.r[29].wrapping_add(c.r[0]), c.r[3]);
                pc = 0x801f7b4c; continue;
                pc = 0x801f7b40;
            }
            0x801f7b40 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = 0x1;
                c.stb(c.r[29].wrapping_add(c.r[0]), c.r[3]);
                pc = 0x801f7b4c;
            }
            0x801f7b4c => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f7b50;
            }
            0x801f7b50 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f7a40; continue; }
                pc = 0x801f7b5c;
            }
            0x801f7b5c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7954: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCheckItemKind (0x801f7b70, 228 bytes)
#[inline(never)]
pub fn f_801f7b70(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7b70) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7b70); p = true; }
    }
    f_801f7b70_b(c, 0x801f7b70);
    if p { crate::prof::exit(c, 0x801f7b70); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7b70_at(c: &mut Cpu, pc: u32) { f_801f7b70_b(c, pc) }
#[inline(always)]
fn f_801f7b70_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7b70;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7b70, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7b70 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x801f7b90; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7b90;
            }
            0x801f7b90 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f7ba0; continue; }
                pc = 0x801f7b98;
            }
            0x801f7b98 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f7c40; continue;
                pc = 0x801f7ba0;
            }
            0x801f7ba0 => { ins += 6;
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x1;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x801f7bb8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80129bc8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7bb8;
            }
            0x801f7bb8 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x801f7bc8; continue; }
                pc = 0x801f7bc0;
            }
            0x801f7bc0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f7c40; continue;
                pc = 0x801f7bc8;
            }
            0x801f7bc8 => { ins += 2;
                c.r[29] = 0x0;
                pc = 0x801f7c2c; continue;
                pc = 0x801f7bd0;
            }
            0x801f7bd0 => { ins += 4;
                c.r[0] = c.r[29].rotate_left(2) & 0x3fffc;
                c.r[30] = c.r[31].wrapping_add(c.r[0]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801f7be0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801429e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7be0;
            }
            0x801f7be0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7c28; continue; }
                pc = 0x801f7be8;
            }
            0x801f7be8 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x1b;
                c.r[6] = 0x0;
                c.lr = 0x801f7bfc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7bfc;
            }
            0x801f7bfc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7c28; continue; }
                pc = 0x801f7c04;
            }
            0x801f7c04 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x1c;
                c.r[6] = 0x0;
                c.lr = 0x801f7c18; { c.steps = st; c.instrs = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7c18;
            }
            0x801f7c18 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7c28; continue; }
                pc = 0x801f7c20;
            }
            0x801f7c20 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801f7c40; continue;
                pc = 0x801f7c28;
            }
            0x801f7c28 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f7c2c;
            }
            0x801f7c2c => { ins += 4;
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801f7bd0; continue; }
                pc = 0x801f7c3c;
            }
            0x801f7c3c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f7c40;
            }
            0x801f7c40 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7b70: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetTemotiNormalItemDataIdAry (0x801f7c54, 524 bytes)
#[inline(never)]
pub fn f_801f7c54(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7c54) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7c54); p = true; }
    }
    f_801f7c54_b(c, 0x801f7c54);
    if p { crate::prof::exit(c, 0x801f7c54); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7c54_at(c: &mut Cpu, pc: u32) { f_801f7c54_b(c, pc) }
#[inline(never)]
fn f_801f7c54_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7c54;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7c54, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7c54 => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[24] = c.r[4] | c.r[4];
                c.r[25] = c.r[5] | c.r[5];
                c.r[26] = c.r[6] | c.r[6];
                c.r[27] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801f7c84; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7c84;
            }
            0x801f7c84 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f7c94; continue; }
                pc = 0x801f7c8c;
            }
            0x801f7c8c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f7e4c; continue;
                pc = 0x801f7c94;
            }
            0x801f7c94 => { ins += 6;
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x2;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x801f7cac; { c.steps = st; c.instrs = ins; c.cr = cr; f_80129bc8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7cac;
            }
            0x801f7cac => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x801f7cbc; continue; }
                pc = 0x801f7cb4;
            }
            0x801f7cb4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f7e4c; continue;
                pc = 0x801f7cbc;
            }
            0x801f7cbc => { ins += 4;
                c.r[5] = 0x0;
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x801f7cd8; continue;
                pc = 0x801f7ccc;
            }
            0x801f7ccc => { ins += 3;
                c.r[0] = c.r[5].rotate_left(1) & 0x1fffe;
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.sth(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                pc = 0x801f7cd8;
            }
            0x801f7cd8 => { ins += 3;
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f7ccc; continue; }
                pc = 0x801f7ce4;
            }
            0x801f7ce4 => { ins += 4;
                c.r[30] = 0x0;
                c.r[28] = c.r[1].wrapping_add(0xc);
                c.r[23] = c.r[30] | c.r[30];
                pc = 0x801f7d38; continue;
                pc = 0x801f7cf4;
            }
            0x801f7cf4 => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[6] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f7d08; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7d08;
            }
            0x801f7d08 => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x801f7d10; { c.steps = st; c.instrs = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7d10;
            }
            0x801f7d10 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7d34; continue; }
                pc = 0x801f7d18;
            }
            0x801f7d18 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801f7d20; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204c08(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7d20;
            }
            0x801f7d20 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7d34; continue; }
                pc = 0x801f7d28;
            }
            0x801f7d28 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(1) & 0x1fffe;
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.sth(c.r[28].wrapping_add(c.r[0]), c.r[3]);
                pc = 0x801f7d34;
            }
            0x801f7d34 => { ins += 1;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x801f7d38;
            }
            0x801f7d38 => { ins += 3;
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f7cf4; continue; }
                pc = 0x801f7d44;
            }
            0x801f7d44 => { ins += 4;
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                c.r[5] = 0x0;
                c.r[4] = 0x0;
                pc = 0x801f7d60; continue;
                pc = 0x801f7d54;
            }
            0x801f7d54 => { ins += 3;
                c.r[3] = c.r[5].rotate_left(1) & 0x1fffe;
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.sth(c.r[24].wrapping_add(c.r[3]), c.r[4]);
                pc = 0x801f7d60;
            }
            0x801f7d60 => { ins += 3;
                c.r[3] = c.r[5].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801f7d54; continue; }
                pc = 0x801f7d6c;
            }
            0x801f7d6c => { ins += 3;
                c.r[28] = 0x0;
                c.r[29] = 0x0;
                pc = 0x801f7e38; continue;
                pc = 0x801f7d78;
            }
            0x801f7d78 => { ins += 4;
                c.r[0] = c.r[29].rotate_left(2) & 0x3fffc;
                c.r[23] = c.r[31].wrapping_add(c.r[0]);
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x801f7d88; { c.steps = st; c.instrs = ins; c.cr = cr; f_801429e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7d88;
            }
            0x801f7d88 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7e34; continue; }
                pc = 0x801f7d90;
            }
            0x801f7d90 => { ins += 5;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x1b;
                c.r[6] = 0x0;
                c.lr = 0x801f7da4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7da4;
            }
            0x801f7da4 => { ins += 2;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[27]);
                if cr_bit(cr, 2) { pc = 0x801f7e34; continue; }
                pc = 0x801f7dac;
            }
            0x801f7dac => { ins += 5;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x0;
                c.r[5] = 0x1c;
                c.r[6] = 0x0;
                c.lr = 0x801f7dc0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7dc0;
            }
            0x801f7dc0 => { ins += 2;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[5]);
                if cr_bit(cr, 2) { pc = 0x801f7e34; continue; }
                pc = 0x801f7dc8;
            }
            0x801f7dc8 => { ins += 3;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f7e18; continue; }
                pc = 0x801f7dd4;
            }
            0x801f7dd4 => { ins += 4;
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.r[6] = 0x0;
                pc = 0x801f7e04; continue;
                pc = 0x801f7de4;
            }
            0x801f7de4 => { ins += 4;
                c.r[3] = c.r[6].rotate_left(1) & 0x1fffe;
                c.r[3] = c.lhz_t(ins, c.r[4].wrapping_add(c.r[3]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x801f7e00; continue; }
                pc = 0x801f7df4;
            }
            0x801f7df4 => { ins += 2;
                c.r[3] = c.r[5].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[3]);
                if cr_bit(cr, 2) { pc = 0x801f7e00; continue; }
                pc = 0x801f7dfc;
            }
            0x801f7dfc => { ins += 1;
                c.r[5] = c.r[5].wrapping_add(0xffffffff);
                pc = 0x801f7e00;
            }
            0x801f7e00 => { ins += 1;
                c.r[6] = c.r[6].wrapping_add(0x1);
                pc = 0x801f7e04;
            }
            0x801f7e04 => { ins += 3;
                c.r[3] = c.r[6].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801f7de4; continue; }
                pc = 0x801f7e10;
            }
            0x801f7e10 => { ins += 2;
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7e34; continue; }
                pc = 0x801f7e18;
            }
            0x801f7e18 => { ins += 4;
                c.r[3] = c.r[28].rotate_left(0) & 0xffff;
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x801f7e34; continue; }
                pc = 0x801f7e28;
            }
            0x801f7e28 => { ins += 3;
                c.r[0] = c.r[28].rotate_left(1) & 0x1fffe;
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.sth(c.r[24].wrapping_add(c.r[0]), c.r[27]);
                pc = 0x801f7e34;
            }
            0x801f7e34 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f7e38;
            }
            0x801f7e38 => { ins += 4;
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801f7d78; continue; }
                pc = 0x801f7e48;
            }
            0x801f7e48 => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x801f7e4c;
            }
            0x801f7e4c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 23..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7c54: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerIsUsedItem (0x801f7e60, 144 bytes)
#[inline(never)]
pub fn f_801f7e60(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7e60) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7e60); p = true; }
    }
    f_801f7e60_b(c, 0x801f7e60);
    if p { crate::prof::exit(c, 0x801f7e60); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7e60_at(c: &mut Cpu, pc: u32) { f_801f7e60_b(c, pc) }
#[inline(always)]
fn f_801f7e60_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7e60;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7e60, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7e60 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = 0x0;
                pc = 0x801f7ecc; continue;
                pc = 0x801f7e7c;
            }
            0x801f7e7c => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f7e90; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7e90;
            }
            0x801f7e90 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f7e98; { c.steps = st; c.instrs = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7e98;
            }
            0x801f7e98 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7ec8; continue; }
                pc = 0x801f7ea0;
            }
            0x801f7ea0 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x801f7eb4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204a5c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7eb4;
            }
            0x801f7eb4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f7ec8; continue; }
                pc = 0x801f7ec0;
            }
            0x801f7ec0 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801f7edc; continue;
                pc = 0x801f7ec8;
            }
            0x801f7ec8 => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x801f7ecc;
            }
            0x801f7ecc => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f7e7c; continue; }
                pc = 0x801f7ed8;
            }
            0x801f7ed8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f7edc;
            }
            0x801f7edc => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7e60: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerIsSelectedItemBall (0x801f7ef0, 144 bytes)
#[inline(never)]
pub fn f_801f7ef0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7ef0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7ef0); p = true; }
    }
    f_801f7ef0_b(c, 0x801f7ef0);
    if p { crate::prof::exit(c, 0x801f7ef0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7ef0_at(c: &mut Cpu, pc: u32) { f_801f7ef0_b(c, pc) }
#[inline(always)]
fn f_801f7ef0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7ef0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7ef0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7ef0 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = 0x0;
                pc = 0x801f7f5c; continue;
                pc = 0x801f7f0c;
            }
            0x801f7f0c => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f7f20; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7f20;
            }
            0x801f7f20 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f7f28; { c.steps = st; c.instrs = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7f28;
            }
            0x801f7f28 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f7f58; continue; }
                pc = 0x801f7f30;
            }
            0x801f7f30 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x801f7f44; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204a5c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7f44;
            }
            0x801f7f44 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f7f58; continue; }
                pc = 0x801f7f50;
            }
            0x801f7f50 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801f7f6c; continue;
                pc = 0x801f7f58;
            }
            0x801f7f58 => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x801f7f5c;
            }
            0x801f7f5c => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f7f0c; continue; }
                pc = 0x801f7f68;
            }
            0x801f7f68 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f7f6c;
            }
            0x801f7f6c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7ef0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSearchInvalidFightPokemonPtr (0x801f7f80, 128 bytes)
#[inline(never)]
pub fn f_801f7f80(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f7f80) { return; }
        if c.prof { crate::prof::enter(c, 0x801f7f80); p = true; }
    }
    f_801f7f80_b(c, 0x801f7f80);
    if p { crate::prof::exit(c, 0x801f7f80); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f7f80_at(c: &mut Cpu, pc: u32) { f_801f7f80_b(c, pc) }
#[inline(always)]
fn f_801f7f80_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f7f80;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f7f80, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f7f80 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if !cr_bit(cr, 2) { pc = 0x801f7fa0; continue; }
                pc = 0x801f7f98;
            }
            0x801f7f98 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f7fec; continue;
                pc = 0x801f7fa0;
            }
            0x801f7fa0 => { ins += 3;
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.r[30] = 0x0;
                pc = 0x801f7fdc; continue;
                pc = 0x801f7fac;
            }
            0x801f7fac => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801f7fc0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7fc0;
            }
            0x801f7fc0 => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x801f7fc8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206a04(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f7fc8;
            }
            0x801f7fc8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f7fd8; continue; }
                pc = 0x801f7fd0;
            }
            0x801f7fd0 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x801f7fec; continue;
                pc = 0x801f7fd8;
            }
            0x801f7fd8 => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x801f7fdc;
            }
            0x801f7fdc => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x801f7fac; continue; }
                pc = 0x801f7fe8;
            }
            0x801f7fe8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f7fec;
            }
            0x801f7fec => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f7f80: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetKindPrefixName (0x801f8000, 256 bytes)
#[inline(never)]
pub fn f_801f8000(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8000) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8000); p = true; }
    }
    f_801f8000_b(c, 0x801f8000);
    if p { crate::prof::exit(c, 0x801f8000); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8000_at(c: &mut Cpu, pc: u32) { f_801f8000_b(c, pc) }
#[inline(always)]
fn f_801f8000_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8000;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8000, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8000 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x801f8020; continue; }
                pc = 0x801f8018;
            }
            0x801f8018 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f80ec; continue;
                pc = 0x801f8020;
            }
            0x801f8020 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801f802c; continue; }
                pc = 0x801f8024;
            }
            0x801f8024 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f80a4; continue;
                pc = 0x801f802c;
            }
            0x801f802c => { ins += 1;
                c.lr = 0x801f8030; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8030;
            }
            0x801f8030 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8044; continue; }
                pc = 0x801f803c;
            }
            0x801f803c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f80a4; continue;
                pc = 0x801f8044;
            }
            0x801f8044 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801f8058; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8058;
            }
            0x801f8058 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8068; continue; }
                pc = 0x801f8060;
            }
            0x801f8060 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f80a4; continue;
                pc = 0x801f8068;
            }
            0x801f8068 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801f807c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f807c;
            }
            0x801f807c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f808c; continue; }
                pc = 0x801f8084;
            }
            0x801f8084 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f80a4; continue;
                pc = 0x801f808c;
            }
            0x801f808c => { ins += 1;
                c.lr = 0x801f8090; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a130(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8090;
            }
            0x801f8090 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f80a0; continue; }
                pc = 0x801f8098;
            }
            0x801f8098 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f80a4; continue;
                pc = 0x801f80a0;
            }
            0x801f80a0 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x801f80a4;
            }
            0x801f80a4 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f80b4; continue; }
                pc = 0x801f80ac;
            }
            0x801f80ac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f80ec; continue;
                pc = 0x801f80b4;
            }
            0x801f80b4 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801f80c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f80c8;
            }
            0x801f80c8 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801f80dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f80dc;
            }
            0x801f80dc => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x801f80e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbd58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f80e4;
            }
            0x801f80e4 => { ins += 1;
                c.lr = 0x801f80e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbd28(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f80e8;
            }
            0x801f80e8 => { ins += 1;
                c.lr = 0x801f80ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_800fa280(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f80ec;
            }
            0x801f80ec => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8000: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetNamePtr (0x801f8100, 248 bytes)
#[inline(never)]
pub fn f_801f8100(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8100) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8100); p = true; }
    }
    f_801f8100_b(c, 0x801f8100);
    if p { crate::prof::exit(c, 0x801f8100); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8100_at(c: &mut Cpu, pc: u32) { f_801f8100_b(c, pc) }
#[inline(always)]
fn f_801f8100_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8100;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8100, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8100 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x801f8120; continue; }
                pc = 0x801f8118;
            }
            0x801f8118 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f81e4; continue;
                pc = 0x801f8120;
            }
            0x801f8120 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801f812c; continue; }
                pc = 0x801f8124;
            }
            0x801f8124 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f81a4; continue;
                pc = 0x801f812c;
            }
            0x801f812c => { ins += 1;
                c.lr = 0x801f8130; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8130;
            }
            0x801f8130 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8144; continue; }
                pc = 0x801f813c;
            }
            0x801f813c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f81a4; continue;
                pc = 0x801f8144;
            }
            0x801f8144 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801f8158; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8158;
            }
            0x801f8158 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8168; continue; }
                pc = 0x801f8160;
            }
            0x801f8160 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f81a4; continue;
                pc = 0x801f8168;
            }
            0x801f8168 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801f817c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f817c;
            }
            0x801f817c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f818c; continue; }
                pc = 0x801f8184;
            }
            0x801f8184 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f81a4; continue;
                pc = 0x801f818c;
            }
            0x801f818c => { ins += 1;
                c.lr = 0x801f8190; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a130(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8190;
            }
            0x801f8190 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f81a0; continue; }
                pc = 0x801f8198;
            }
            0x801f8198 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f81a4; continue;
                pc = 0x801f81a0;
            }
            0x801f81a0 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x801f81a4;
            }
            0x801f81a4 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f81b4; continue; }
                pc = 0x801f81ac;
            }
            0x801f81ac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f81e4; continue;
                pc = 0x801f81b4;
            }
            0x801f81b4 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801f81c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f81c8;
            }
            0x801f81c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f81d8; continue; }
                pc = 0x801f81d0;
            }
            0x801f81d0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f81e4; continue;
                pc = 0x801f81d8;
            }
            0x801f81d8 => { ins += 3;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x801f81e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a5b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f81e4;
            }
            0x801f81e4 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8100: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetHikaeFightPokemonNum (0x801f81f8, 556 bytes)
#[inline(never)]
pub fn f_801f81f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f81f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f81f8); p = true; }
    }
    f_801f81f8_b(c, 0x801f81f8);
    if p { crate::prof::exit(c, 0x801f81f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f81f8_at(c: &mut Cpu, pc: u32) { f_801f81f8_b(c, pc) }
#[inline(never)]
fn f_801f81f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f81f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f81f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f81f8 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[26]);
                c.r[27] = c.r[5] | c.r[5];
                if !cr_bit(cr, 2) { pc = 0x801f821c; continue; }
                pc = 0x801f8214;
            }
            0x801f8214 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8410; continue;
                pc = 0x801f821c;
            }
            0x801f821c => { ins += 5;
                c.r[25] = c.r[4].rotate_left(0) & 0xffff;
                c.r[29] = 0x0;
                c.r[28] = 0x0;
                c.r[30] = 0x0;
                pc = 0x801f83d8; continue;
                pc = 0x801f8230;
            }
            0x801f8230 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801f8244; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8244;
            }
            0x801f8244 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f824c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206a04(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f824c;
            }
            0x801f824c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8258; continue; }
                pc = 0x801f8254;
            }
            0x801f8254 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x801f8258;
            }
            0x801f8258 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f83d4; continue; }
                pc = 0x801f8260;
            }
            0x801f8260 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f8268; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8268;
            }
            0x801f8268 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8278; continue; }
                pc = 0x801f8270;
            }
            0x801f8270 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f83b4; continue;
                pc = 0x801f8278;
            }
            0x801f8278 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8288; continue; }
                pc = 0x801f8280;
            }
            0x801f8280 => { ins += 2;
                c.r[23] = 0x0;
                pc = 0x801f8300; continue;
                pc = 0x801f8288;
            }
            0x801f8288 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8298; continue; }
                pc = 0x801f8290;
            }
            0x801f8290 => { ins += 2;
                c.r[23] = 0x0;
                pc = 0x801f8300; continue;
                pc = 0x801f8298;
            }
            0x801f8298 => { ins += 2;
                c.r[24] = 0x0;
                pc = 0x801f82f0; continue;
                pc = 0x801f82a0;
            }
            0x801f82a0 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f82b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f82b4;
            }
            0x801f82b4 => { ins += 2;
                c.r[23] = c.r[3] | c.r[3];
                c.lr = 0x801f82bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f82bc;
            }
            0x801f82bc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f82c8; continue; }
                pc = 0x801f82c4;
            }
            0x801f82c4 => { ins += 1;
                c.r[23] = 0x0;
                pc = 0x801f82c8;
            }
            0x801f82c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f82ec; continue; }
                pc = 0x801f82d0;
            }
            0x801f82d0 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x801f82dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204928(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f82dc;
            }
            0x801f82dc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f82ec; continue; }
                pc = 0x801f82e8;
            }
            0x801f82e8 => { ins += 1;
                pc = 0x801f8300; continue;
                pc = 0x801f82ec;
            }
            0x801f82ec => { ins += 1;
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x801f82f0;
            }
            0x801f82f0 => { ins += 3;
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f82a0; continue; }
                pc = 0x801f82fc;
            }
            0x801f82fc => { ins += 1;
                c.r[23] = 0x0;
                pc = 0x801f8300;
            }
            0x801f8300 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8310; continue; }
                pc = 0x801f8308;
            }
            0x801f8308 => { ins += 2;
                c.r[0] = 0x2;
                pc = 0x801f83b4; continue;
                pc = 0x801f8310;
            }
            0x801f8310 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8320; continue; }
                pc = 0x801f8318;
            }
            0x801f8318 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f839c; continue;
                pc = 0x801f8320;
            }
            0x801f8320 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8330; continue; }
                pc = 0x801f8328;
            }
            0x801f8328 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f839c; continue;
                pc = 0x801f8330;
            }
            0x801f8330 => { ins += 2;
                c.r[24] = 0x0;
                pc = 0x801f838c; continue;
                pc = 0x801f8338;
            }
            0x801f8338 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f834c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f834c;
            }
            0x801f834c => { ins += 2;
                c.r[23] = c.r[3] | c.r[3];
                c.lr = 0x801f8354; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8354;
            }
            0x801f8354 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8360; continue; }
                pc = 0x801f835c;
            }
            0x801f835c => { ins += 1;
                c.r[23] = 0x0;
                pc = 0x801f8360;
            }
            0x801f8360 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8388; continue; }
                pc = 0x801f8368;
            }
            0x801f8368 => { ins += 3;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f8374; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204854(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8374;
            }
            0x801f8374 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8388; continue; }
                pc = 0x801f8380;
            }
            0x801f8380 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f839c; continue;
                pc = 0x801f8388;
            }
            0x801f8388 => { ins += 1;
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x801f838c;
            }
            0x801f838c => { ins += 3;
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f8338; continue; }
                pc = 0x801f8398;
            }
            0x801f8398 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x801f839c;
            }
            0x801f839c => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f83b0; continue; }
                pc = 0x801f83a8;
            }
            0x801f83a8 => { ins += 2;
                c.r[0] = 0x3;
                pc = 0x801f83b4; continue;
                pc = 0x801f83b0;
            }
            0x801f83b0 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x801f83b4;
            }
            0x801f83b4 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x801f83d4; continue; }
                pc = 0x801f83c0;
            }
            0x801f83c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x801f83d0; continue; }
                pc = 0x801f83c8;
            }
            0x801f83c8 => { ins += 2;
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x801f83d4; continue;
                pc = 0x801f83d0;
            }
            0x801f83d0 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f83d4;
            }
            0x801f83d4 => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x801f83d8;
            }
            0x801f83d8 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[25]);
                if cr_bit(cr, 0) { pc = 0x801f8230; continue; }
                pc = 0x801f83e4;
            }
            0x801f83e4 => { ins += 1;
                c.r[3] = c.r[27].rotate_left(0) & 0xffff;
                pc = 0x801f83e8;
            }
            0x801f83e8 => { ins += 3;
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[3]);
                if !cr_bit(cr, 0) { pc = 0x801f840c; continue; }
                pc = 0x801f83f4;
            }
            0x801f83f4 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 0) { pc = 0x801f840c; continue; }
                pc = 0x801f8400;
            }
            0x801f8400 => { ins += 3;
                c.r[29] = c.r[29].wrapping_add(0xffffffff);
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x801f83e8; continue;
                pc = 0x801f840c;
            }
            0x801f840c => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x801f8410;
            }
            0x801f8410 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f81f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerIsAllyFightTargetPtr (0x801f8424, 120 bytes)
#[inline(never)]
pub fn f_801f8424(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8424) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8424); p = true; }
    }
    f_801f8424_b(c, 0x801f8424);
    if p { crate::prof::exit(c, 0x801f8424); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8424_at(c: &mut Cpu, pc: u32) { f_801f8424_b(c, pc) }
#[inline(always)]
fn f_801f8424_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8424;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8424, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8424 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                if !cr_bit(cr, 2) { pc = 0x801f844c; continue; }
                pc = 0x801f8444;
            }
            0x801f8444 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8488; continue;
                pc = 0x801f844c;
            }
            0x801f844c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f845c; continue; }
                pc = 0x801f8454;
            }
            0x801f8454 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8488; continue;
                pc = 0x801f845c;
            }
            0x801f845c => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.lr = 0x801f8468; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8468;
            }
            0x801f8468 => { ins += 5;
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.r[3] = 0x2;
                c.lr = 0x801f847c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f847c;
            }
            0x801f847c => { ins += 3;
                c.r[0] = c.r[3].wrapping_sub(c.r[31]);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0xff;
                pc = 0x801f8488;
            }
            0x801f8488 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8424: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerInitEnemyPokemonFightOutStatus (0x801f849c, 124 bytes)
#[inline(never)]
pub fn f_801f849c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f849c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f849c); p = true; }
    }
    f_801f849c_b(c, 0x801f849c);
    if p { crate::prof::exit(c, 0x801f849c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f849c_at(c: &mut Cpu, pc: u32) { f_801f849c_b(c, pc) }
#[inline(always)]
fn f_801f849c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f849c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f849c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f849c => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if cr_bit(cr, 2) { pc = 0x801f8504; continue; }
                pc = 0x801f84b4;
            }
            0x801f84b4 => { ins += 3;
                c.r[31] = c.r[4].rotate_left(0) & 0xff;
                c.r[29] = 0x0;
                pc = 0x801f84f8; continue;
                pc = 0x801f84c0;
            }
            0x801f84c0 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[29].rotate_left(0) & 0xff;
                c.r[4] = 0x0;
                c.r[5] = 0x4e;
                c.lr = 0x801f84d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f84d4;
            }
            0x801f84d4 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x1);
                c.r[30] = c.r[3] | c.r[3];
                if !cr_bit(cr, 2) { pc = 0x801f84ec; continue; }
                pc = 0x801f84e0;
            }
            0x801f84e0 => { ins += 1;
                c.lr = 0x801f84e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eca4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f84e4;
            }
            0x801f84e4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f84f4; continue; }
                pc = 0x801f84ec;
            }
            0x801f84ec => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801f84f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020edbc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f84f4;
            }
            0x801f84f4 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f84f8;
            }
            0x801f84f8 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x801f84c0; continue; }
                pc = 0x801f8504;
            }
            0x801f8504 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f849c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetNoActionFightOutPokemonPtr (0x801f8518, 152 bytes)
#[inline(never)]
pub fn f_801f8518(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8518) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8518); p = true; }
    }
    f_801f8518_b(c, 0x801f8518);
    if p { crate::prof::exit(c, 0x801f8518); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8518_at(c: &mut Cpu, pc: u32) { f_801f8518_b(c, pc) }
#[inline(always)]
fn f_801f8518_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8518;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8518, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8518 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0x46;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x801f8538; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8538;
            }
            0x801f8538 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f8540; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8540;
            }
            0x801f8540 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f854c; continue; }
                pc = 0x801f8548;
            }
            0x801f8548 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x801f854c;
            }
            0x801f854c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f855c; continue; }
                pc = 0x801f8554;
            }
            0x801f8554 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f859c; continue;
                pc = 0x801f855c;
            }
            0x801f855c => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xfe;
                c.r[6] = 0x0;
                c.lr = 0x801f8570; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8570;
            }
            0x801f8570 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8580; continue; }
                pc = 0x801f8578;
            }
            0x801f8578 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f859c; continue;
                pc = 0x801f8580;
            }
            0x801f8580 => { ins += 1;
                c.lr = 0x801f8584; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f1170(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8584;
            }
            0x801f8584 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8598; continue; }
                pc = 0x801f8590;
            }
            0x801f8590 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f859c; continue;
                pc = 0x801f8598;
            }
            0x801f8598 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f859c;
            }
            0x801f859c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8518: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetFightOutPokemonToTemotiBanme (0x801f85b0, 136 bytes)
#[inline(never)]
pub fn f_801f85b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f85b0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f85b0); p = true; }
    }
    f_801f85b0_b(c, 0x801f85b0);
    if p { crate::prof::exit(c, 0x801f85b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f85b0_at(c: &mut Cpu, pc: u32) { f_801f85b0_b(c, pc) }
#[inline(always)]
fn f_801f85b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f85b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f85b0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f85b0 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                c.r[30] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x801f85d4; continue; }
                pc = 0x801f85cc;
            }
            0x801f85cc => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x801f8624; continue;
                pc = 0x801f85d4;
            }
            0x801f85d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f85e4; continue; }
                pc = 0x801f85dc;
            }
            0x801f85dc => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x801f8624; continue;
                pc = 0x801f85e4;
            }
            0x801f85e4 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x801f8614; continue;
                pc = 0x801f85ec;
            }
            0x801f85ec => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f8600; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8600;
            }
            0x801f8600 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x801f8610; continue; }
                pc = 0x801f8608;
            }
            0x801f8608 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f8624; continue;
                pc = 0x801f8610;
            }
            0x801f8610 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f8614;
            }
            0x801f8614 => { ins += 3;
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f85ec; continue; }
                pc = 0x801f8620;
            }
            0x801f8620 => { ins += 1;
                c.r[3] = 0xffffffff;
                pc = 0x801f8624;
            }
            0x801f8624 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f85b0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetFightPokemonToTemotiBanme (0x801f8638, 136 bytes)
#[inline(never)]
pub fn f_801f8638(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8638) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8638); p = true; }
    }
    f_801f8638_b(c, 0x801f8638);
    if p { crate::prof::exit(c, 0x801f8638); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8638_at(c: &mut Cpu, pc: u32) { f_801f8638_b(c, pc) }
#[inline(always)]
fn f_801f8638_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8638;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8638, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8638 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                c.r[30] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x801f865c; continue; }
                pc = 0x801f8654;
            }
            0x801f8654 => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x801f86ac; continue;
                pc = 0x801f865c;
            }
            0x801f865c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f866c; continue; }
                pc = 0x801f8664;
            }
            0x801f8664 => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x801f86ac; continue;
                pc = 0x801f866c;
            }
            0x801f866c => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x801f869c; continue;
                pc = 0x801f8674;
            }
            0x801f8674 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801f8688; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8688;
            }
            0x801f8688 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x801f8698; continue; }
                pc = 0x801f8690;
            }
            0x801f8690 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f86ac; continue;
                pc = 0x801f8698;
            }
            0x801f8698 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f869c;
            }
            0x801f869c => { ins += 3;
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f8674; continue; }
                pc = 0x801f86a8;
            }
            0x801f86a8 => { ins += 1;
                c.r[3] = 0xffffffff;
                pc = 0x801f86ac;
            }
            0x801f86ac => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8638: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerBackFightPokemonToTemotiPokemon (0x801f86c0, 268 bytes)
#[inline(never)]
pub fn f_801f86c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f86c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801f86c0); p = true; }
    }
    f_801f86c0_b(c, 0x801f86c0);
    if p { crate::prof::exit(c, 0x801f86c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f86c0_at(c: &mut Cpu, pc: u32) { f_801f86c0_b(c, pc) }
#[inline(always)]
fn f_801f86c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f86c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f86c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f86c0 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[26]);
                c.r[27] = c.r[4] | c.r[4];
                if cr_bit(cr, 2) { pc = 0x801f87b8; continue; }
                pc = 0x801f86dc;
            }
            0x801f86dc => { ins += 2;
                c.r[29] = 0x0;
                pc = 0x801f87ac; continue;
                pc = 0x801f86e4;
            }
            0x801f86e4 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801f86f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f86f8;
            }
            0x801f86f8 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x801f8700; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206a04(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8700;
            }
            0x801f8700 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f870c; continue; }
                pc = 0x801f8708;
            }
            0x801f8708 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x801f870c;
            }
            0x801f870c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f87a8; continue; }
                pc = 0x801f8714;
            }
            0x801f8714 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xcc;
                c.r[6] = 0x0;
                c.lr = 0x801f8728; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8728;
            }
            0x801f8728 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if cr_bit(cr, 2) { pc = 0x801f87a8; continue; }
                pc = 0x801f8730;
            }
            0x801f8730 => { ins += 1;
                c.lr = 0x801f8734; { c.steps = st; c.instrs = ins; c.cr = cr; f_801230e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8734;
            }
            0x801f8734 => { ins += 6;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xcb;
                c.r[6] = 0x0;
                c.lr = 0x801f874c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f874c;
            }
            0x801f874c => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x801f87a8; continue; }
                pc = 0x801f8754;
            }
            0x801f8754 => { ins += 2;
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x801f875c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011f5fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f875c;
            }
            0x801f875c => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x4;
                c.lr = 0x801f8768; { c.steps = st; c.instrs = ins; c.cr = cr; f_80121adc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8768;
            }
            0x801f8768 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8784; continue; }
                pc = 0x801f8774;
            }
            0x801f8774 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x4;
                c.r[5] = 0x1;
                c.lr = 0x801f8784; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012190c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8784;
            }
            0x801f8784 => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f87a8; continue; }
                pc = 0x801f8790;
            }
            0x801f8790 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[7] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x82;
                c.r[6] = 0x0;
                c.lr = 0x801f87a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f87a8;
            }
            0x801f87a8 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f87ac;
            }
            0x801f87ac => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f86e4; continue; }
                pc = 0x801f87b8;
            }
            0x801f87b8 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f86c0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetIrekaeFightPokemonAry (0x801f87cc, 588 bytes)
#[inline(never)]
pub fn f_801f87cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f87cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801f87cc); p = true; }
    }
    f_801f87cc_b(c, 0x801f87cc);
    if p { crate::prof::exit(c, 0x801f87cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f87cc_at(c: &mut Cpu, pc: u32) { f_801f87cc_b(c, pc) }
#[inline(never)]
fn f_801f87cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f87cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f87cc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f87cc => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.sth(c.r[1].wrapping_add(0x8), c.r[5]);
                pc = 0x801f8804; continue;
                pc = 0x801f87f0;
            }
            0x801f87f0 => { ins += 5;
                c.r[3] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[0] = c.r[0].rotate_left(2) & 0x3fffc;
                c.stw(c.r[29].wrapping_add(c.r[0]), c.r[5]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x801f8804;
            }
            0x801f8804 => { ins += 3;
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f87f0; continue; }
                pc = 0x801f8810;
            }
            0x801f8810 => { ins += 3;
                c.r[0] = 0x0;
                c.r[30] = 0x0;
                c.sth(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x801f881c;
            }
            0x801f881c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f882c; continue; }
                pc = 0x801f8824;
            }
            0x801f8824 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801f89dc; continue;
                pc = 0x801f882c;
            }
            0x801f882c => { ins += 2;
                c.r[0] = c.addc(c.r[1], 0x8, 0);
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f89b8; continue; }
                pc = 0x801f8834;
            }
            0x801f8834 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801f89dc; continue;
                pc = 0x801f883c;
            }
            0x801f883c => { ins += 1;
                pc = 0x801f89b8; continue;
                pc = 0x801f8840;
            }
            0x801f8840 => { ins += 4;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801f8850; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8850;
            }
            0x801f8850 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f8858; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8858;
            }
            0x801f8858 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8868; continue; }
                pc = 0x801f8860;
            }
            0x801f8860 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f89a4; continue;
                pc = 0x801f8868;
            }
            0x801f8868 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8878; continue; }
                pc = 0x801f8870;
            }
            0x801f8870 => { ins += 2;
                c.r[26] = 0x0;
                pc = 0x801f88f0; continue;
                pc = 0x801f8878;
            }
            0x801f8878 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8888; continue; }
                pc = 0x801f8880;
            }
            0x801f8880 => { ins += 2;
                c.r[26] = 0x0;
                pc = 0x801f88f0; continue;
                pc = 0x801f8888;
            }
            0x801f8888 => { ins += 2;
                c.r[27] = 0x0;
                pc = 0x801f88e0; continue;
                pc = 0x801f8890;
            }
            0x801f8890 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f88a4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f88a4;
            }
            0x801f88a4 => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x801f88ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f88ac;
            }
            0x801f88ac => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f88b8; continue; }
                pc = 0x801f88b4;
            }
            0x801f88b4 => { ins += 1;
                c.r[26] = 0x0;
                pc = 0x801f88b8;
            }
            0x801f88b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f88dc; continue; }
                pc = 0x801f88c0;
            }
            0x801f88c0 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x801f88cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204928(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f88cc;
            }
            0x801f88cc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f88dc; continue; }
                pc = 0x801f88d8;
            }
            0x801f88d8 => { ins += 1;
                pc = 0x801f88f0; continue;
                pc = 0x801f88dc;
            }
            0x801f88dc => { ins += 1;
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x801f88e0;
            }
            0x801f88e0 => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f8890; continue; }
                pc = 0x801f88ec;
            }
            0x801f88ec => { ins += 1;
                c.r[26] = 0x0;
                pc = 0x801f88f0;
            }
            0x801f88f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8900; continue; }
                pc = 0x801f88f8;
            }
            0x801f88f8 => { ins += 2;
                c.r[0] = 0x2;
                pc = 0x801f89a4; continue;
                pc = 0x801f8900;
            }
            0x801f8900 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8910; continue; }
                pc = 0x801f8908;
            }
            0x801f8908 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f898c; continue;
                pc = 0x801f8910;
            }
            0x801f8910 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8920; continue; }
                pc = 0x801f8918;
            }
            0x801f8918 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f898c; continue;
                pc = 0x801f8920;
            }
            0x801f8920 => { ins += 2;
                c.r[27] = 0x0;
                pc = 0x801f897c; continue;
                pc = 0x801f8928;
            }
            0x801f8928 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f893c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f893c;
            }
            0x801f893c => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x801f8944; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8944;
            }
            0x801f8944 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8950; continue; }
                pc = 0x801f894c;
            }
            0x801f894c => { ins += 1;
                c.r[26] = 0x0;
                pc = 0x801f8950;
            }
            0x801f8950 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8978; continue; }
                pc = 0x801f8958;
            }
            0x801f8958 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f8964; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204854(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8964;
            }
            0x801f8964 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8978; continue; }
                pc = 0x801f8970;
            }
            0x801f8970 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f898c; continue;
                pc = 0x801f8978;
            }
            0x801f8978 => { ins += 1;
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x801f897c;
            }
            0x801f897c => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f8928; continue; }
                pc = 0x801f8988;
            }
            0x801f8988 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x801f898c;
            }
            0x801f898c => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f89a0; continue; }
                pc = 0x801f8998;
            }
            0x801f8998 => { ins += 2;
                c.r[0] = 0x3;
                pc = 0x801f89a4; continue;
                pc = 0x801f89a0;
            }
            0x801f89a0 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x801f89a4;
            }
            0x801f89a4 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f89c4; continue; }
                pc = 0x801f89ac;
            }
            0x801f89ac => { ins += 3;
                c.r[3] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x801f89b8;
            }
            0x801f89b8 => { ins += 3;
                c.r[6] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f8840; continue; }
                pc = 0x801f89c4;
            }
            0x801f89c4 => { ins += 3;
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f89d8; continue; }
                pc = 0x801f89d0;
            }
            0x801f89d0 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801f89dc; continue;
                pc = 0x801f89d8;
            }
            0x801f89d8 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801f89dc;
            }
            0x801f89dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8a00; continue; }
                pc = 0x801f89e4;
            }
            0x801f89e4 => { ins += 7;
                c.r[3] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[0] = c.r[30].rotate_left(2) & 0x3fffc;
                c.stw(c.r[29].wrapping_add(c.r[0]), c.r[4]);
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x801f881c; continue;
                pc = 0x801f8a00;
            }
            0x801f8a00 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f87cc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetIrekaeFightPokemon (0x801f8a18, 488 bytes)
#[inline(never)]
pub fn f_801f8a18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8a18) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8a18); p = true; }
    }
    f_801f8a18_b(c, 0x801f8a18);
    if p { crate::prof::exit(c, 0x801f8a18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8a18_at(c: &mut Cpu, pc: u32) { f_801f8a18_b(c, pc) }
#[inline(never)]
fn f_801f8a18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8a18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8a18, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8a18 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                c.r[30] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x801f8a3c; continue; }
                pc = 0x801f8a34;
            }
            0x801f8a34 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8bec; continue;
                pc = 0x801f8a3c;
            }
            0x801f8a3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8bc8; continue; }
                pc = 0x801f8a44;
            }
            0x801f8a44 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8bec; continue;
                pc = 0x801f8a4c;
            }
            0x801f8a4c => { ins += 1;
                pc = 0x801f8bc8; continue;
                pc = 0x801f8a50;
            }
            0x801f8a50 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801f8a60; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8a60;
            }
            0x801f8a60 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f8a68; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8a68;
            }
            0x801f8a68 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8a78; continue; }
                pc = 0x801f8a70;
            }
            0x801f8a70 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f8bb4; continue;
                pc = 0x801f8a78;
            }
            0x801f8a78 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8a88; continue; }
                pc = 0x801f8a80;
            }
            0x801f8a80 => { ins += 2;
                c.r[27] = 0x0;
                pc = 0x801f8b00; continue;
                pc = 0x801f8a88;
            }
            0x801f8a88 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8a98; continue; }
                pc = 0x801f8a90;
            }
            0x801f8a90 => { ins += 2;
                c.r[27] = 0x0;
                pc = 0x801f8b00; continue;
                pc = 0x801f8a98;
            }
            0x801f8a98 => { ins += 2;
                c.r[28] = 0x0;
                pc = 0x801f8af0; continue;
                pc = 0x801f8aa0;
            }
            0x801f8aa0 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f8ab4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8ab4;
            }
            0x801f8ab4 => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x801f8abc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8abc;
            }
            0x801f8abc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8ac8; continue; }
                pc = 0x801f8ac4;
            }
            0x801f8ac4 => { ins += 1;
                c.r[27] = 0x0;
                pc = 0x801f8ac8;
            }
            0x801f8ac8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8aec; continue; }
                pc = 0x801f8ad0;
            }
            0x801f8ad0 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x801f8adc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204928(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8adc;
            }
            0x801f8adc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8aec; continue; }
                pc = 0x801f8ae8;
            }
            0x801f8ae8 => { ins += 1;
                pc = 0x801f8b00; continue;
                pc = 0x801f8aec;
            }
            0x801f8aec => { ins += 1;
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x801f8af0;
            }
            0x801f8af0 => { ins += 3;
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f8aa0; continue; }
                pc = 0x801f8afc;
            }
            0x801f8afc => { ins += 1;
                c.r[27] = 0x0;
                pc = 0x801f8b00;
            }
            0x801f8b00 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8b10; continue; }
                pc = 0x801f8b08;
            }
            0x801f8b08 => { ins += 2;
                c.r[0] = 0x2;
                pc = 0x801f8bb4; continue;
                pc = 0x801f8b10;
            }
            0x801f8b10 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8b20; continue; }
                pc = 0x801f8b18;
            }
            0x801f8b18 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f8b9c; continue;
                pc = 0x801f8b20;
            }
            0x801f8b20 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8b30; continue; }
                pc = 0x801f8b28;
            }
            0x801f8b28 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f8b9c; continue;
                pc = 0x801f8b30;
            }
            0x801f8b30 => { ins += 2;
                c.r[28] = 0x0;
                pc = 0x801f8b8c; continue;
                pc = 0x801f8b38;
            }
            0x801f8b38 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f8b4c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8b4c;
            }
            0x801f8b4c => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x801f8b54; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8b54;
            }
            0x801f8b54 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8b60; continue; }
                pc = 0x801f8b5c;
            }
            0x801f8b5c => { ins += 1;
                c.r[27] = 0x0;
                pc = 0x801f8b60;
            }
            0x801f8b60 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8b88; continue; }
                pc = 0x801f8b68;
            }
            0x801f8b68 => { ins += 3;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f8b74; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204854(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8b74;
            }
            0x801f8b74 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8b88; continue; }
                pc = 0x801f8b80;
            }
            0x801f8b80 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f8b9c; continue;
                pc = 0x801f8b88;
            }
            0x801f8b88 => { ins += 1;
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x801f8b8c;
            }
            0x801f8b8c => { ins += 3;
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f8b38; continue; }
                pc = 0x801f8b98;
            }
            0x801f8b98 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x801f8b9c;
            }
            0x801f8b9c => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8bb0; continue; }
                pc = 0x801f8ba8;
            }
            0x801f8ba8 => { ins += 2;
                c.r[0] = 0x3;
                pc = 0x801f8bb4; continue;
                pc = 0x801f8bb0;
            }
            0x801f8bb0 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x801f8bb4;
            }
            0x801f8bb4 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f8bd4; continue; }
                pc = 0x801f8bbc;
            }
            0x801f8bbc => { ins += 3;
                c.r[3] = c.lhz_t(ins, c.r[30]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[30], c.r[0]);
                pc = 0x801f8bc8;
            }
            0x801f8bc8 => { ins += 3;
                c.r[6] = c.lhz_t(ins, c.r[30]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f8a50; continue; }
                pc = 0x801f8bd4;
            }
            0x801f8bd4 => { ins += 3;
                c.r[0] = c.lhz_t(ins, c.r[30]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f8be8; continue; }
                pc = 0x801f8be0;
            }
            0x801f8be0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8bec; continue;
                pc = 0x801f8be8;
            }
            0x801f8be8 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f8bec;
            }
            0x801f8bec => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8a18: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCheckCanIrekaeFightPokemon (0x801f8c00, 384 bytes)
#[inline(never)]
pub fn f_801f8c00(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8c00) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8c00); p = true; }
    }
    f_801f8c00_b(c, 0x801f8c00);
    if p { crate::prof::exit(c, 0x801f8c00); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8c00_at(c: &mut Cpu, pc: u32) { f_801f8c00_b(c, pc) }
#[inline(never)]
fn f_801f8c00_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8c00;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8c00, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8c00 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801f8c20; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8c20;
            }
            0x801f8c20 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8c30; continue; }
                pc = 0x801f8c28;
            }
            0x801f8c28 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801f8d6c; continue;
                pc = 0x801f8c30;
            }
            0x801f8c30 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8c40; continue; }
                pc = 0x801f8c38;
            }
            0x801f8c38 => { ins += 2;
                c.r[30] = 0x0;
                pc = 0x801f8cb8; continue;
                pc = 0x801f8c40;
            }
            0x801f8c40 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8c50; continue; }
                pc = 0x801f8c48;
            }
            0x801f8c48 => { ins += 2;
                c.r[30] = 0x0;
                pc = 0x801f8cb8; continue;
                pc = 0x801f8c50;
            }
            0x801f8c50 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x801f8ca8; continue;
                pc = 0x801f8c58;
            }
            0x801f8c58 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f8c6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8c6c;
            }
            0x801f8c6c => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x801f8c74; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8c74;
            }
            0x801f8c74 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8c80; continue; }
                pc = 0x801f8c7c;
            }
            0x801f8c7c => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x801f8c80;
            }
            0x801f8c80 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8ca4; continue; }
                pc = 0x801f8c88;
            }
            0x801f8c88 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x801f8c94; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204928(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8c94;
            }
            0x801f8c94 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8ca4; continue; }
                pc = 0x801f8ca0;
            }
            0x801f8ca0 => { ins += 1;
                pc = 0x801f8cb8; continue;
                pc = 0x801f8ca4;
            }
            0x801f8ca4 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f8ca8;
            }
            0x801f8ca8 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f8c58; continue; }
                pc = 0x801f8cb4;
            }
            0x801f8cb4 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x801f8cb8;
            }
            0x801f8cb8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8cc8; continue; }
                pc = 0x801f8cc0;
            }
            0x801f8cc0 => { ins += 2;
                c.r[3] = 0x2;
                pc = 0x801f8d6c; continue;
                pc = 0x801f8cc8;
            }
            0x801f8cc8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8cd8; continue; }
                pc = 0x801f8cd0;
            }
            0x801f8cd0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f8d54; continue;
                pc = 0x801f8cd8;
            }
            0x801f8cd8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8ce8; continue; }
                pc = 0x801f8ce0;
            }
            0x801f8ce0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f8d54; continue;
                pc = 0x801f8ce8;
            }
            0x801f8ce8 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x801f8d44; continue;
                pc = 0x801f8cf0;
            }
            0x801f8cf0 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f8d04; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8d04;
            }
            0x801f8d04 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x801f8d0c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8d0c;
            }
            0x801f8d0c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8d18; continue; }
                pc = 0x801f8d14;
            }
            0x801f8d14 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x801f8d18;
            }
            0x801f8d18 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8d40; continue; }
                pc = 0x801f8d20;
            }
            0x801f8d20 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x801f8d2c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204854(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8d2c;
            }
            0x801f8d2c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8d40; continue; }
                pc = 0x801f8d38;
            }
            0x801f8d38 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f8d54; continue;
                pc = 0x801f8d40;
            }
            0x801f8d40 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f8d44;
            }
            0x801f8d44 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f8cf0; continue; }
                pc = 0x801f8d50;
            }
            0x801f8d50 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x801f8d54;
            }
            0x801f8d54 => { ins += 6;
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = 0x3;
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                c.r[3] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[3], ci); }
                c.r[3] = c.r[0] & c.r[3];
                pc = 0x801f8d6c;
            }
            0x801f8d6c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8c00: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetFightPokemonPtrToFightOutPokemonPtr (0x801f8d80, 180 bytes)
#[inline(never)]
pub fn f_801f8d80(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8d80) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8d80); p = true; }
    }
    f_801f8d80_b(c, 0x801f8d80);
    if p { crate::prof::exit(c, 0x801f8d80); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8d80_at(c: &mut Cpu, pc: u32) { f_801f8d80_b(c, pc) }
#[inline(always)]
fn f_801f8d80_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8d80;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8d80, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8d80 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                c.r[29] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x801f8da4; continue; }
                pc = 0x801f8d9c;
            }
            0x801f8d9c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8e20; continue;
                pc = 0x801f8da4;
            }
            0x801f8da4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8db4; continue; }
                pc = 0x801f8dac;
            }
            0x801f8dac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8e20; continue;
                pc = 0x801f8db4;
            }
            0x801f8db4 => { ins += 2;
                c.r[30] = 0x0;
                pc = 0x801f8e10; continue;
                pc = 0x801f8dbc;
            }
            0x801f8dbc => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f8dd0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8dd0;
            }
            0x801f8dd0 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f8dd8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8dd8;
            }
            0x801f8dd8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8de4; continue; }
                pc = 0x801f8de0;
            }
            0x801f8de0 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x801f8de4;
            }
            0x801f8de4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8e0c; continue; }
                pc = 0x801f8dec;
            }
            0x801f8dec => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f8df8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204928(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8df8;
            }
            0x801f8df8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8e0c; continue; }
                pc = 0x801f8e04;
            }
            0x801f8e04 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f8e20; continue;
                pc = 0x801f8e0c;
            }
            0x801f8e0c => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x801f8e10;
            }
            0x801f8e10 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f8dbc; continue; }
                pc = 0x801f8e1c;
            }
            0x801f8e1c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f8e20;
            }
            0x801f8e20 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8d80: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCheckFightPokemonFightOut (0x801f8e34, 240 bytes)
#[inline(never)]
pub fn f_801f8e34(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8e34) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8e34); p = true; }
    }
    f_801f8e34_b(c, 0x801f8e34);
    if p { crate::prof::exit(c, 0x801f8e34); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8e34_at(c: &mut Cpu, pc: u32) { f_801f8e34_b(c, pc) }
#[inline(always)]
fn f_801f8e34_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8e34;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8e34, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8e34 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                c.r[29] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x801f8e58; continue; }
                pc = 0x801f8e50;
            }
            0x801f8e50 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8f10; continue;
                pc = 0x801f8e58;
            }
            0x801f8e58 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8e68; continue; }
                pc = 0x801f8e60;
            }
            0x801f8e60 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8f10; continue;
                pc = 0x801f8e68;
            }
            0x801f8e68 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8e78; continue; }
                pc = 0x801f8e70;
            }
            0x801f8e70 => { ins += 2;
                c.r[30] = 0x0;
                pc = 0x801f8ef0; continue;
                pc = 0x801f8e78;
            }
            0x801f8e78 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f8e88; continue; }
                pc = 0x801f8e80;
            }
            0x801f8e80 => { ins += 2;
                c.r[30] = 0x0;
                pc = 0x801f8ef0; continue;
                pc = 0x801f8e88;
            }
            0x801f8e88 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x801f8ee0; continue;
                pc = 0x801f8e90;
            }
            0x801f8e90 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f8ea4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8ea4;
            }
            0x801f8ea4 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x801f8eac; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8eac;
            }
            0x801f8eac => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8eb8; continue; }
                pc = 0x801f8eb4;
            }
            0x801f8eb4 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x801f8eb8;
            }
            0x801f8eb8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8edc; continue; }
                pc = 0x801f8ec0;
            }
            0x801f8ec0 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x801f8ecc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204928(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8ecc;
            }
            0x801f8ecc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f8edc; continue; }
                pc = 0x801f8ed8;
            }
            0x801f8ed8 => { ins += 1;
                pc = 0x801f8ef0; continue;
                pc = 0x801f8edc;
            }
            0x801f8edc => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f8ee0;
            }
            0x801f8ee0 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f8e90; continue; }
                pc = 0x801f8eec;
            }
            0x801f8eec => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x801f8ef0;
            }
            0x801f8ef0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8f0c; continue; }
                pc = 0x801f8ef8;
            }
            0x801f8ef8 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801f8f00; { c.steps = st; c.instrs = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8f00;
            }
            0x801f8f00 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8f0c; continue; }
                pc = 0x801f8f08;
            }
            0x801f8f08 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x801f8f0c;
            }
            0x801f8f0c => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x801f8f10;
            }
            0x801f8f10 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8e34: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetEntryIdToFightPokemonPtr (0x801f8f24, 180 bytes)
#[inline(never)]
pub fn f_801f8f24(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8f24) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8f24); p = true; }
    }
    f_801f8f24_b(c, 0x801f8f24);
    if p { crate::prof::exit(c, 0x801f8f24); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8f24_at(c: &mut Cpu, pc: u32) { f_801f8f24_b(c, pc) }
#[inline(always)]
fn f_801f8f24_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8f24;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8f24, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8f24 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[0] = c.r[29] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[28] = c.r[3] | c.r[3];
                if !cr_bit(cr, 0) { pc = 0x801f8f4c; continue; }
                pc = 0x801f8f44;
            }
            0x801f8f44 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f8fc4; continue;
                pc = 0x801f8f4c;
            }
            0x801f8f4c => { ins += 2;
                c.r[30] = 0x0;
                pc = 0x801f8fb4; continue;
                pc = 0x801f8f54;
            }
            0x801f8f54 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801f8f68; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8f68;
            }
            0x801f8f68 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f8f70; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206a04(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8f70;
            }
            0x801f8f70 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f8f7c; continue; }
                pc = 0x801f8f78;
            }
            0x801f8f78 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x801f8f7c;
            }
            0x801f8f7c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f8fb0; continue; }
                pc = 0x801f8f84;
            }
            0x801f8f84 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xce;
                c.r[6] = 0x0;
                c.lr = 0x801f8f98; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8f98;
            }
            0x801f8f98 => { ins += 4;
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[0] = c.r[29] as u16 as i16 as i32 as u32;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x801f8fb0; continue; }
                pc = 0x801f8fa8;
            }
            0x801f8fa8 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f8fc4; continue;
                pc = 0x801f8fb0;
            }
            0x801f8fb0 => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x801f8fb4;
            }
            0x801f8fb4 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f8f54; continue; }
                pc = 0x801f8fc0;
            }
            0x801f8fc0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f8fc4;
            }
            0x801f8fc4 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8f24: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCreateSequence (0x801f8fd8, 92 bytes)
#[inline(never)]
pub fn f_801f8fd8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f8fd8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f8fd8); p = true; }
    }
    f_801f8fd8_b(c, 0x801f8fd8);
    if p { crate::prof::exit(c, 0x801f8fd8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f8fd8_at(c: &mut Cpu, pc: u32) { f_801f8fd8_b(c, pc) }
#[inline(always)]
fn f_801f8fd8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f8fd8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f8fd8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f8fd8 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0x9;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x801f8ffc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f8ffc;
            }
            0x801f8ffc => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x801f9004; { c.steps = st; c.instrs = ins; c.cr = cr; f_801de418(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9004;
            }
            0x801f9004 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x801f9014; continue; }
                pc = 0x801f900c;
            }
            0x801f900c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f9020; continue;
                pc = 0x801f9014;
            }
            0x801f9014 => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x801f901c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da4e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f901c;
            }
            0x801f901c => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f9020;
            }
            0x801f9020 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f8fd8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCheckCanGetExp (0x801f9034, 144 bytes)
#[inline(never)]
pub fn f_801f9034(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f9034) { return; }
        if c.prof { crate::prof::enter(c, 0x801f9034); p = true; }
    }
    f_801f9034_b(c, 0x801f9034);
    if p { crate::prof::exit(c, 0x801f9034); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f9034_at(c: &mut Cpu, pc: u32) { f_801f9034_b(c, pc) }
#[inline(always)]
fn f_801f9034_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f9034;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f9034, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f9034 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.lr = 0x801f9050; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9050;
            }
            0x801f9050 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801f9064; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9064;
            }
            0x801f9064 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f9078; continue; }
                pc = 0x801f9070;
            }
            0x801f9070 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9094; continue;
                pc = 0x801f9078;
            }
            0x801f9078 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801f9088; continue; }
                pc = 0x801f9080;
            }
            0x801f9080 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801f9090; continue; }
                pc = 0x801f9088;
            }
            0x801f9088 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f9094; continue;
                pc = 0x801f9090;
            }
            0x801f9090 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x801f9094;
            }
            0x801f9094 => { ins += 5;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[0].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f90b0; continue; }
                pc = 0x801f90a8;
            }
            0x801f90a8 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801f90b4; continue;
                pc = 0x801f90b0;
            }
            0x801f90b0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f90b4;
            }
            0x801f90b4 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f9034: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerIsMineFightPokemon (0x801f90c4, 108 bytes)
#[inline(never)]
pub fn f_801f90c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f90c4) { return; }
        if c.prof { crate::prof::enter(c, 0x801f90c4); p = true; }
    }
    f_801f90c4_b(c, 0x801f90c4);
    if p { crate::prof::exit(c, 0x801f90c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f90c4_at(c: &mut Cpu, pc: u32) { f_801f90c4_b(c, pc) }
#[inline(always)]
fn f_801f90c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f90c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f90c4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f90c4 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[4] | c.r[4];
                c.lr = 0x801f90e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80205be8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f90e0;
            }
            0x801f90e0 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x801f90f0; continue; }
                pc = 0x801f90e8;
            }
            0x801f90e8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f911c; continue;
                pc = 0x801f90f0;
            }
            0x801f90f0 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801f9104; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9104;
            }
            0x801f9104 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f9114; continue; }
                pc = 0x801f910c;
            }
            0x801f910c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f911c; continue;
                pc = 0x801f9114;
            }
            0x801f9114 => { ins += 2;
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f911c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80129d64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f911c;
            }
            0x801f911c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f90c4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerTimeOutSelectFightAction (0x801f9130, 268 bytes)
#[inline(never)]
pub fn f_801f9130(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f9130) { return; }
        if c.prof { crate::prof::enter(c, 0x801f9130); p = true; }
    }
    f_801f9130_b(c, 0x801f9130);
    if p { crate::prof::exit(c, 0x801f9130); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f9130_at(c: &mut Cpu, pc: u32) { f_801f9130_b(c, pc) }
#[inline(always)]
fn f_801f9130_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f9130;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f9130, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f9130 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[4] = 0x1;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x801f9154; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ff1bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9154;
            }
            0x801f9154 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f9228; continue; }
                pc = 0x801f915c;
            }
            0x801f915c => { ins += 2;
                c.r[29] = 0x0;
                pc = 0x801f91c4; continue;
                pc = 0x801f9164;
            }
            0x801f9164 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x801f916c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80205b8c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f916c;
            }
            0x801f916c => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.r[6] = 0x0;
                c.lr = 0x801f917c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f917c;
            }
            0x801f917c => { ins += 7;
                c.r[31] = c.r[29] as u8 as i8 as i32 as u32;
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = 0x0;
                c.lr = 0x801f9198; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ffec8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9198;
            }
            0x801f9198 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f91c0; continue; }
                pc = 0x801f91a0;
            }
            0x801f91a0 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x801f91a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80205b8c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f91a8;
            }
            0x801f91a8 => { ins += 4;
                c.r[6] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x801f91b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f91b8;
            }
            0x801f91b8 => { ins += 2;
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801f91d0; continue;
                pc = 0x801f91c0;
            }
            0x801f91c0 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f91c4;
            }
            0x801f91c4 => { ins += 3;
                c.r[0] = c.r[29] as u8 as i8 as i32 as u32;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801f9164; continue; }
                pc = 0x801f91d0;
            }
            0x801f91d0 => { ins += 8;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.r[9] = 0xffffffff;
                c.lr = 0x801f91f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8022b2cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f91f0;
            }
            0x801f91f0 => { ins += 2;
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x801f91f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f0134(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f91f8;
            }
            0x801f91f8 => { ins += 12;
                c.r[0] = 0x0;
                c.r[4] = 0x80370000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[9] = c.r[3] | c.r[3];
                c.r[7] = c.r[4].wrapping_add(0x5ca8);
                c.r[3] = c.r[27] | c.r[27];
                c.r[10] = c.r[29] | c.r[29];
                c.r[8] = c.r[30].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x13;
                c.r[6] = 0x0;
                c.lr = 0x801f9228; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204f6c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9228;
            }
            0x801f9228 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f9130: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSelectFightAction (0x801f923c, 444 bytes)
#[inline(never)]
pub fn f_801f923c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f923c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f923c); p = true; }
    }
    f_801f923c_b(c, 0x801f923c);
    if p { crate::prof::exit(c, 0x801f923c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f923c_at(c: &mut Cpu, pc: u32) { f_801f923c_b(c, pc) }
#[inline(never)]
fn f_801f923c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f923c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f923c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f923c => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x4d;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.lr = 0x801f9264; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9264;
            }
            0x801f9264 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f9270; continue; }
                pc = 0x801f926c;
            }
            0x801f926c => { ins += 1;
                c.lr = 0x801f9270; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f1460(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9270;
            }
            0x801f9270 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x801f9294; continue;
                pc = 0x801f9278;
            }
            0x801f9278 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f928c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f928c;
            }
            0x801f928c => { ins += 1;
                c.lr = 0x801f9290; { c.steps = st; c.instrs = ins; c.cr = cr; f_80207760(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9290;
            }
            0x801f9290 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f9294;
            }
            0x801f9294 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f9278; continue; }
                pc = 0x801f92a0;
            }
            0x801f92a0 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x47;
                c.r[6] = 0x0;
                c.lr = 0x801f92b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f92b4;
            }
            0x801f92b4 => { ins += 1;
                c.lr = 0x801f92b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80207760(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f92b8;
            }
            0x801f92b8 => { ins += 1;
                c.lr = 0x801f92bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f92bc;
            }
            0x801f92bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f92d0; continue; }
                pc = 0x801f92c8;
            }
            0x801f92c8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f93e4; continue;
                pc = 0x801f92d0;
            }
            0x801f92d0 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801f92e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f92e4;
            }
            0x801f92e4 => { ins += 6;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x801f92fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f92fc;
            }
            0x801f92fc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f9314; continue; }
                pc = 0x801f9304;
            }
            0x801f9304 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x801f9310; { c.steps = st; c.instrs = ins; c.cr = cr; f_802342cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9310;
            }
            0x801f9310 => { ins += 1;
                pc = 0x801f93e0; continue;
                pc = 0x801f9314;
            }
            0x801f9314 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801f9328; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9328;
            }
            0x801f9328 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f933c; continue; }
                pc = 0x801f9334;
            }
            0x801f9334 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9358; continue;
                pc = 0x801f933c;
            }
            0x801f933c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801f934c; continue; }
                pc = 0x801f9344;
            }
            0x801f9344 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801f9354; continue; }
                pc = 0x801f934c;
            }
            0x801f934c => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f9358; continue;
                pc = 0x801f9354;
            }
            0x801f9354 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x801f9358;
            }
            0x801f9358 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f9374; continue; }
                pc = 0x801f9360;
            }
            0x801f9360 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x801f9370; { c.steps = st; c.instrs = ins; c.cr = cr; f_8026316c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9370;
            }
            0x801f9370 => { ins += 1;
                pc = 0x801f93e0; continue;
                pc = 0x801f9374;
            }
            0x801f9374 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801f9388; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9388;
            }
            0x801f9388 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f939c; continue; }
                pc = 0x801f9394;
            }
            0x801f9394 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f93b8; continue;
                pc = 0x801f939c;
            }
            0x801f939c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801f93ac; continue; }
                pc = 0x801f93a4;
            }
            0x801f93a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801f93b4; continue; }
                pc = 0x801f93ac;
            }
            0x801f93ac => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f93b8; continue;
                pc = 0x801f93b4;
            }
            0x801f93b4 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x801f93b8;
            }
            0x801f93b8 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f93e0; continue; }
                pc = 0x801f93c4;
            }
            0x801f93c4 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x801f93d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80262508(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f93d0;
            }
            0x801f93d0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f93e0; continue; }
                pc = 0x801f93d8;
            }
            0x801f93d8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f93e4; continue;
                pc = 0x801f93e0;
            }
            0x801f93e0 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x801f93e4;
            }
            0x801f93e4 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f923c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerTimeOutSelectIrekaeFightPokemon (0x801f93f8, 520 bytes)
#[inline(never)]
pub fn f_801f93f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f93f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f93f8); p = true; }
    }
    f_801f93f8_b(c, 0x801f93f8);
    if p { crate::prof::exit(c, 0x801f93f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f93f8_at(c: &mut Cpu, pc: u32) { f_801f93f8_b(c, pc) }
#[inline(never)]
fn f_801f93f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f93f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f93f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f93f8 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                c.r[3] = 0x0;
                c.sth(c.r[1].wrapping_add(0x8), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x801f941c; continue; }
                pc = 0x801f9418;
            }
            0x801f9418 => { ins += 1;
                pc = 0x801f95c8; continue;
                pc = 0x801f941c;
            }
            0x801f941c => { ins += 2;
                c.r[0] = c.addc(c.r[1], 0x8, 0);
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f95a4; continue; }
                pc = 0x801f9424;
            }
            0x801f9424 => { ins += 1;
                pc = 0x801f95c8; continue;
                pc = 0x801f9428;
            }
            0x801f9428 => { ins += 1;
                pc = 0x801f95a4; continue;
                pc = 0x801f942c;
            }
            0x801f942c => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801f943c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f943c;
            }
            0x801f943c => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f9444; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9444;
            }
            0x801f9444 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f9454; continue; }
                pc = 0x801f944c;
            }
            0x801f944c => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f9590; continue;
                pc = 0x801f9454;
            }
            0x801f9454 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f9464; continue; }
                pc = 0x801f945c;
            }
            0x801f945c => { ins += 2;
                c.r[28] = 0x0;
                pc = 0x801f94dc; continue;
                pc = 0x801f9464;
            }
            0x801f9464 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f9474; continue; }
                pc = 0x801f946c;
            }
            0x801f946c => { ins += 2;
                c.r[28] = 0x0;
                pc = 0x801f94dc; continue;
                pc = 0x801f9474;
            }
            0x801f9474 => { ins += 2;
                c.r[29] = 0x0;
                pc = 0x801f94cc; continue;
                pc = 0x801f947c;
            }
            0x801f947c => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f9490; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9490;
            }
            0x801f9490 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x801f9498; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9498;
            }
            0x801f9498 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f94a4; continue; }
                pc = 0x801f94a0;
            }
            0x801f94a0 => { ins += 1;
                c.r[28] = 0x0;
                pc = 0x801f94a4;
            }
            0x801f94a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f94c8; continue; }
                pc = 0x801f94ac;
            }
            0x801f94ac => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x801f94b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204928(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f94b8;
            }
            0x801f94b8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f94c8; continue; }
                pc = 0x801f94c4;
            }
            0x801f94c4 => { ins += 1;
                pc = 0x801f94dc; continue;
                pc = 0x801f94c8;
            }
            0x801f94c8 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f94cc;
            }
            0x801f94cc => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f947c; continue; }
                pc = 0x801f94d8;
            }
            0x801f94d8 => { ins += 1;
                c.r[28] = 0x0;
                pc = 0x801f94dc;
            }
            0x801f94dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f94ec; continue; }
                pc = 0x801f94e4;
            }
            0x801f94e4 => { ins += 2;
                c.r[0] = 0x2;
                pc = 0x801f9590; continue;
                pc = 0x801f94ec;
            }
            0x801f94ec => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f94fc; continue; }
                pc = 0x801f94f4;
            }
            0x801f94f4 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9578; continue;
                pc = 0x801f94fc;
            }
            0x801f94fc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f950c; continue; }
                pc = 0x801f9504;
            }
            0x801f9504 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9578; continue;
                pc = 0x801f950c;
            }
            0x801f950c => { ins += 2;
                c.r[29] = 0x0;
                pc = 0x801f9568; continue;
                pc = 0x801f9514;
            }
            0x801f9514 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f9528; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9528;
            }
            0x801f9528 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x801f9530; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9530;
            }
            0x801f9530 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f953c; continue; }
                pc = 0x801f9538;
            }
            0x801f9538 => { ins += 1;
                c.r[28] = 0x0;
                pc = 0x801f953c;
            }
            0x801f953c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f9564; continue; }
                pc = 0x801f9544;
            }
            0x801f9544 => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801f9550; { c.steps = st; c.instrs = ins; c.cr = cr; f_80204854(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9550;
            }
            0x801f9550 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f9564; continue; }
                pc = 0x801f955c;
            }
            0x801f955c => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f9578; continue;
                pc = 0x801f9564;
            }
            0x801f9564 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f9568;
            }
            0x801f9568 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f9514; continue; }
                pc = 0x801f9574;
            }
            0x801f9574 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x801f9578;
            }
            0x801f9578 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f958c; continue; }
                pc = 0x801f9584;
            }
            0x801f9584 => { ins += 2;
                c.r[0] = 0x3;
                pc = 0x801f9590; continue;
                pc = 0x801f958c;
            }
            0x801f958c => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x801f9590;
            }
            0x801f9590 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f95b0; continue; }
                pc = 0x801f9598;
            }
            0x801f9598 => { ins += 3;
                c.r[3] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.sth(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x801f95a4;
            }
            0x801f95a4 => { ins += 3;
                c.r[6] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f942c; continue; }
                pc = 0x801f95b0;
            }
            0x801f95b0 => { ins += 3;
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f95c4; continue; }
                pc = 0x801f95bc;
            }
            0x801f95bc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f95c8; continue;
                pc = 0x801f95c4;
            }
            0x801f95c4 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f95c8;
            }
            0x801f95c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f95e8; continue; }
                pc = 0x801f95d0;
            }
            0x801f95d0 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0xce;
                c.r[6] = 0x0;
                c.lr = 0x801f95e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f95e0;
            }
            0x801f95e0 => { ins += 2;
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x801f95ec; continue;
                pc = 0x801f95e8;
            }
            0x801f95e8 => { ins += 1;
                c.r[3] = 0xffffffff;
                pc = 0x801f95ec;
            }
            0x801f95ec => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f93f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSelectIrekaeFightPokemon (0x801f9600, 400 bytes)
#[inline(never)]
pub fn f_801f9600(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f9600) { return; }
        if c.prof { crate::prof::enter(c, 0x801f9600); p = true; }
    }
    f_801f9600_b(c, 0x801f9600);
    if p { crate::prof::exit(c, 0x801f9600); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f9600_at(c: &mut Cpu, pc: u32) { f_801f9600_b(c, pc) }
#[inline(never)]
fn f_801f9600_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f9600;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f9600, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f9600 => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[28] = c.r[6] | c.r[6];
                c.r[25] = c.r[3] | c.r[3];
                c.r[29] = 0xffffffff;
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801f9634; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9634;
            }
            0x801f9634 => { ins += 6;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x801f964c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f964c;
            }
            0x801f964c => { ins += 2;
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x801f9654; { c.steps = st; c.instrs = ins; c.cr = cr; f_80008174(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9654;
            }
            0x801f9654 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f9680; continue; }
                pc = 0x801f9660;
            }
            0x801f9660 => { ins += 6;
                c.r[3] = c.r[25] | c.r[25];
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801f9678; { c.steps = st; c.instrs = ins; c.cr = cr; f_80263de4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9678;
            }
            0x801f9678 => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x801f9778; continue;
                pc = 0x801f9680;
            }
            0x801f9680 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f96a4; continue; }
                pc = 0x801f9688;
            }
            0x801f9688 => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = c.r[28] | c.r[28];
                c.lr = 0x801f969c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8024e690(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f969c;
            }
            0x801f969c => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x801f9778; continue;
                pc = 0x801f96a4;
            }
            0x801f96a4 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801f96b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f96b8;
            }
            0x801f96b8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f96cc; continue; }
                pc = 0x801f96c4;
            }
            0x801f96c4 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f96e8; continue;
                pc = 0x801f96cc;
            }
            0x801f96cc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801f96dc; continue; }
                pc = 0x801f96d4;
            }
            0x801f96d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801f96e4; continue; }
                pc = 0x801f96dc;
            }
            0x801f96dc => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f96e8; continue;
                pc = 0x801f96e4;
            }
            0x801f96e4 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x801f96e8;
            }
            0x801f96e8 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f9710; continue; }
                pc = 0x801f96f0;
            }
            0x801f96f0 => { ins += 6;
                c.r[3] = c.r[25] | c.r[25];
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801f9708; { c.steps = st; c.instrs = ins; c.cr = cr; f_80263de4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9708;
            }
            0x801f9708 => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x801f9778; continue;
                pc = 0x801f9710;
            }
            0x801f9710 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801f9724; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9724;
            }
            0x801f9724 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f9738; continue; }
                pc = 0x801f9730;
            }
            0x801f9730 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9754; continue;
                pc = 0x801f9738;
            }
            0x801f9738 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801f9748; continue; }
                pc = 0x801f9740;
            }
            0x801f9740 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801f9750; continue; }
                pc = 0x801f9748;
            }
            0x801f9748 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f9754; continue;
                pc = 0x801f9750;
            }
            0x801f9750 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x801f9754;
            }
            0x801f9754 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f9778; continue; }
                pc = 0x801f9760;
            }
            0x801f9760 => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = c.r[28] | c.r[28];
                c.lr = 0x801f9774; { c.steps = st; c.instrs = ins; c.cr = cr; f_80262d3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9774;
            }
            0x801f9774 => { ins += 1;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x801f9778;
            }
            0x801f9778 => { ins += 6;
                c.r[3] = c.r[29] | c.r[29];
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f9600: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAllInitFightActionBuff (0x801f9790, 140 bytes)
#[inline(never)]
pub fn f_801f9790(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f9790) { return; }
        if c.prof { crate::prof::enter(c, 0x801f9790); p = true; }
    }
    f_801f9790_b(c, 0x801f9790);
    if p { crate::prof::exit(c, 0x801f9790); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f9790_at(c: &mut Cpu, pc: u32) { f_801f9790_b(c, pc) }
#[inline(always)]
fn f_801f9790_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f9790;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f9790, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f9790 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x4d;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x801f97b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f97b4;
            }
            0x801f97b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f97c0; continue; }
                pc = 0x801f97bc;
            }
            0x801f97bc => { ins += 1;
                c.lr = 0x801f97c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f1460(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f97c0;
            }
            0x801f97c0 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x801f97e4; continue;
                pc = 0x801f97c8;
            }
            0x801f97c8 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f97dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f97dc;
            }
            0x801f97dc => { ins += 1;
                c.lr = 0x801f97e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80207760(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f97e0;
            }
            0x801f97e0 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f97e4;
            }
            0x801f97e4 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f97c8; continue; }
                pc = 0x801f97f0;
            }
            0x801f97f0 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x47;
                c.r[6] = 0x0;
                c.lr = 0x801f9804; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9804;
            }
            0x801f9804 => { ins += 1;
                c.lr = 0x801f9808; { c.steps = st; c.instrs = ins; c.cr = cr; f_80207760(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9808;
            }
            0x801f9808 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f9790: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetValidFightOutPokemonPtr (0x801f981c, 80 bytes)
#[inline(never)]
pub fn f_801f981c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f981c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f981c); p = true; }
    }
    f_801f981c_b(c, 0x801f981c);
    if p { crate::prof::exit(c, 0x801f981c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f981c_at(c: &mut Cpu, pc: u32) { f_801f981c_b(c, pc) }
#[inline(always)]
fn f_801f981c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f981c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f981c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f981c => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0x46;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x801f983c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f983c;
            }
            0x801f983c => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f9844; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9844;
            }
            0x801f9844 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f9854; continue; }
                pc = 0x801f984c;
            }
            0x801f984c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f9858; continue;
                pc = 0x801f9854;
            }
            0x801f9854 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f9858;
            }
            0x801f9858 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f981c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetValidFightPokemonPtr (0x801f986c, 80 bytes)
#[inline(never)]
pub fn f_801f986c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f986c) { return; }
        if c.prof { crate::prof::enter(c, 0x801f986c); p = true; }
    }
    f_801f986c_b(c, 0x801f986c);
    if p { crate::prof::exit(c, 0x801f986c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f986c_at(c: &mut Cpu, pc: u32) { f_801f986c_b(c, pc) }
#[inline(always)]
fn f_801f986c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f986c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f986c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f986c => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0x45;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x801f988c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f988c;
            }
            0x801f988c => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801f9894; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206a04(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9894;
            }
            0x801f9894 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f98a4; continue; }
                pc = 0x801f989c;
            }
            0x801f989c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801f98a8; continue;
                pc = 0x801f98a4;
            }
            0x801f98a4 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801f98a8;
            }
            0x801f98a8 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f986c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetDoFightOutFightOutPokemonCount (0x801f98bc, 116 bytes)
#[inline(never)]
pub fn f_801f98bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f98bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801f98bc); p = true; }
    }
    f_801f98bc_b(c, 0x801f98bc);
    if p { crate::prof::exit(c, 0x801f98bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f98bc_at(c: &mut Cpu, pc: u32) { f_801f98bc_b(c, pc) }
#[inline(always)]
fn f_801f98bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f98bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f98bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f98bc => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = 0x0;
                c.r[31] = 0x0;
                pc = 0x801f990c; continue;
                pc = 0x801f98dc;
            }
            0x801f98dc => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[31].rotate_left(0) & 0xff;
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x801f98f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f98f0;
            }
            0x801f98f0 => { ins += 1;
                c.lr = 0x801f98f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f98f4;
            }
            0x801f98f4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f9908; continue; }
                pc = 0x801f98fc;
            }
            0x801f98fc => { ins += 3;
                c.r[3] = c.r[30].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[30] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x801f9908;
            }
            0x801f9908 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f990c;
            }
            0x801f990c => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801f98dc; continue; }
                pc = 0x801f9918;
            }
            0x801f9918 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f98bc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCheckTemotiPokemonFightEntry (0x801f9930, 152 bytes)
#[inline(never)]
pub fn f_801f9930(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f9930) { return; }
        if c.prof { crate::prof::enter(c, 0x801f9930); p = true; }
    }
    f_801f9930_b(c, 0x801f9930);
    if p { crate::prof::exit(c, 0x801f9930); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f9930_at(c: &mut Cpu, pc: u32) { f_801f9930_b(c, pc) }
#[inline(always)]
fn f_801f9930_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f9930;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f9930, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f9930 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = 0x0;
                pc = 0x801f99a4; continue;
                pc = 0x801f9950;
            }
            0x801f9950 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[6] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801f9964; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9964;
            }
            0x801f9964 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x801f996c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206a04(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f996c;
            }
            0x801f996c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f99a0; continue; }
                pc = 0x801f9974;
            }
            0x801f9974 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xcb;
                c.r[6] = 0x0;
                c.lr = 0x801f9988; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9988;
            }
            0x801f9988 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f99a0; continue; }
                pc = 0x801f9990;
            }
            0x801f9990 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x801f99a0; continue; }
                pc = 0x801f9998;
            }
            0x801f9998 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x801f99b4; continue;
                pc = 0x801f99a0;
            }
            0x801f99a0 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801f99a4;
            }
            0x801f99a4 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f9950; continue; }
                pc = 0x801f99b0;
            }
            0x801f99b0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801f99b4;
            }
            0x801f99b4 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f9930: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSortFightTrainerDataIdToHeroTemotiPokemon (0x801f99c8, 756 bytes)
#[inline(never)]
pub fn f_801f99c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f99c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801f99c8); p = true; }
    }
    f_801f99c8_b(c, 0x801f99c8);
    if p { crate::prof::exit(c, 0x801f99c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f99c8_at(c: &mut Cpu, pc: u32) { f_801f99c8_b(c, pc) }
#[inline(never)]
fn f_801f99c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f99c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f99c8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f99c8 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffff870); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x794), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x774); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                if cr_bit(cr, 2) { pc = 0x801f9ca8; continue; }
                pc = 0x801f99e8;
            }
            0x801f99e8 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801f99f4; continue; }
                pc = 0x801f99ec;
            }
            0x801f99ec => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9a6c; continue;
                pc = 0x801f99f4;
            }
            0x801f99f4 => { ins += 1;
                c.lr = 0x801f99f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f99f8;
            }
            0x801f99f8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f9a0c; continue; }
                pc = 0x801f9a04;
            }
            0x801f9a04 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9a6c; continue;
                pc = 0x801f9a0c;
            }
            0x801f9a0c => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801f9a20; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9a20;
            }
            0x801f9a20 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f9a30; continue; }
                pc = 0x801f9a28;
            }
            0x801f9a28 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9a6c; continue;
                pc = 0x801f9a30;
            }
            0x801f9a30 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801f9a44; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9a44;
            }
            0x801f9a44 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f9a54; continue; }
                pc = 0x801f9a4c;
            }
            0x801f9a4c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9a6c; continue;
                pc = 0x801f9a54;
            }
            0x801f9a54 => { ins += 1;
                c.lr = 0x801f9a58; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a130(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9a58;
            }
            0x801f9a58 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f9a68; continue; }
                pc = 0x801f9a60;
            }
            0x801f9a60 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9a6c; continue;
                pc = 0x801f9a68;
            }
            0x801f9a68 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x801f9a6c;
            }
            0x801f9a6c => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f9ca8; continue; }
                pc = 0x801f9a74;
            }
            0x801f9a74 => { ins += 5;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801f9a88; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9a88;
            }
            0x801f9a88 => { ins += 6;
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x801f9aa0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9aa0;
            }
            0x801f9aa0 => { ins += 6;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801f9ab8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9ab8;
            }
            0x801f9ab8 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x0);
                c.r[31] = c.r[3] | c.r[3];
                if cr_bit(cr, 2) { pc = 0x801f9ca8; continue; }
                pc = 0x801f9ac4;
            }
            0x801f9ac4 => { ins += 5;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801f9ad8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9ad8;
            }
            0x801f9ad8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f9aec; continue; }
                pc = 0x801f9ae4;
            }
            0x801f9ae4 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9b08; continue;
                pc = 0x801f9aec;
            }
            0x801f9aec => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801f9afc; continue; }
                pc = 0x801f9af4;
            }
            0x801f9af4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801f9b04; continue; }
                pc = 0x801f9afc;
            }
            0x801f9afc => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f9b08; continue;
                pc = 0x801f9b04;
            }
            0x801f9b04 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x801f9b08;
            }
            0x801f9b08 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f9ca8; continue; }
                pc = 0x801f9b10;
            }
            0x801f9b10 => { ins += 5;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801f9b24; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9b24;
            }
            0x801f9b24 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f9b38; continue; }
                pc = 0x801f9b30;
            }
            0x801f9b30 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9b54; continue;
                pc = 0x801f9b38;
            }
            0x801f9b38 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801f9b48; continue; }
                pc = 0x801f9b40;
            }
            0x801f9b40 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801f9b50; continue; }
                pc = 0x801f9b48;
            }
            0x801f9b48 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f9b54; continue;
                pc = 0x801f9b50;
            }
            0x801f9b50 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x801f9b54;
            }
            0x801f9b54 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f9b64; continue; }
                pc = 0x801f9b60;
            }
            0x801f9b60 => { ins += 1;
                pc = 0x801f9ca8; continue;
                pc = 0x801f9b64;
            }
            0x801f9b64 => { ins += 4;
                c.r[27] = c.r[1].wrapping_add(0x8);
                c.r[25] = 0x0;
                c.r[26] = 0x0;
                pc = 0x801f9b94; continue;
                pc = 0x801f9b74;
            }
            0x801f9b74 => { ins += 7;
                c.r[0] = c.r[25].rotate_left(0) & 0xff;
                c.r[4] = c.r[25].rotate_left(2) & 0x3fc;
                c.r[0] = c.r[0].wrapping_mul(0x138);
                c.r[3] = c.r[1].wrapping_add(0x20);
                c.stw(c.r[27].wrapping_add(c.r[4]), c.r[26]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x801f9b90; { c.steps = st; c.instrs = ins; c.cr = cr; f_80124a60(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9b90;
            }
            0x801f9b90 => { ins += 1;
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x801f9b94;
            }
            0x801f9b94 => { ins += 3;
                c.r[0] = c.r[25].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f9b74; continue; }
                pc = 0x801f9ba0;
            }
            0x801f9ba0 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 1) { pc = 0x801f9bb0; continue; }
                pc = 0x801f9bac;
            }
            0x801f9bac => { ins += 1;
                c.r[29] = 0x6;
                pc = 0x801f9bb0;
            }
            0x801f9bb0 => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[31] | c.r[31];
                c.r[6] = c.r[29] | c.r[29];
                c.r[7] = c.r[30] | c.r[30];
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.lr = 0x801f9bc8; { c.steps = st; c.instrs = ins; c.cr = cr; f_802331f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9bc8;
            }
            0x801f9bc8 => { ins += 5;
                c.r[26] = c.r[29].rotate_left(0) & 0xffff;
                c.r[28] = c.r[1].wrapping_add(0x8);
                c.r[25] = 0x0;
                c.r[27] = 0x0;
                pc = 0x801f9c38; continue;
                pc = 0x801f9bdc;
            }
            0x801f9bdc => { ins += 4;
                c.r[0] = c.r[27].rotate_left(2) & 0x3fc;
                c.r[29] = c.lwz_t(ins, c.r[28].wrapping_add(c.r[0]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f9c34; continue; }
                pc = 0x801f9bec;
            }
            0x801f9bec => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801f9bf4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80123fbc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9bf4;
            }
            0x801f9bf4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f9c34; continue; }
                pc = 0x801f9bfc;
            }
            0x801f9bfc => { ins += 8;
                c.r[0] = c.r[25].rotate_left(0) & 0xff;
                c.r[3] = c.r[1].wrapping_add(0x20);
                c.r[5] = c.r[0].wrapping_mul(0x138);
                c.r[0] = 0x27;
                c.r[4] = c.r[29].wrapping_add(0xfffffffc);
                c.r[5] = c.r[3].wrapping_add(c.r[5]);
                c.mtspr(9, c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0xfffffffc);
                pc = 0x801f9c1c;
            }
            0x801f9c1c => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x801f9c1c; continue; }
                pc = 0x801f9c30;
            }
            0x801f9c30 => { ins += 1;
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x801f9c34;
            }
            0x801f9c34 => { ins += 1;
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x801f9c38;
            }
            0x801f9c38 => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], c.r[26]);
                if cr_bit(cr, 0) { pc = 0x801f9bdc; continue; }
                pc = 0x801f9c44;
            }
            0x801f9c44 => { ins += 4;
                c.r[29] = c.r[1].wrapping_add(0x20);
                c.r[26] = 0x0;
                c.r[28] = 0x27;
                pc = 0x801f9c9c; continue;
                pc = 0x801f9c54;
            }
            0x801f9c54 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[26].rotate_left(0) & 0xff;
                c.r[4] = 0x3;
                c.lr = 0x801f9c64; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a5b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9c64;
            }
            0x801f9c64 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801f9c98; continue; }
                pc = 0x801f9c6c;
            }
            0x801f9c6c => { ins += 6;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                c.r[5] = c.r[3].wrapping_add(0xfffffffc);
                c.r[0] = c.r[0].wrapping_mul(0x138);
                c.r[4] = c.r[29].wrapping_add(c.r[0]);
                c.mtspr(9, c.r[28]);
                c.r[4] = c.r[4].wrapping_add(0xfffffffc);
                pc = 0x801f9c84;
            }
            0x801f9c84 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x801f9c84; continue; }
                pc = 0x801f9c98;
            }
            0x801f9c98 => { ins += 1;
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x801f9c9c;
            }
            0x801f9c9c => { ins += 3;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801f9c54; continue; }
                pc = 0x801f9ca8;
            }
            0x801f9ca8 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x774); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x794));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x790);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f99c8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCreateFightTrainerDataIdToHero (0x801f9cbc, 700 bytes)
#[inline(never)]
pub fn f_801f9cbc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f9cbc) { return; }
        if c.prof { crate::prof::enter(c, 0x801f9cbc); p = true; }
    }
    f_801f9cbc_b(c, 0x801f9cbc);
    if p { crate::prof::exit(c, 0x801f9cbc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f9cbc_at(c: &mut Cpu, pc: u32) { f_801f9cbc_b(c, pc) }
#[inline(never)]
fn f_801f9cbc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f9cbc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f9cbc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f9cbc => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffeb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x154), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x140); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[5] | c.r[5];
                cr_rc(&mut cr, c.xer, c.r[31]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                if cr_bit(cr, 2) { pc = 0x801f9f64; continue; }
                pc = 0x801f9cdc;
            }
            0x801f9cdc => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x801f9cf0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9cf0;
            }
            0x801f9cf0 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                if !cr_bit(cr, 2) { pc = 0x801f9d04; continue; }
                pc = 0x801f9cfc;
            }
            0x801f9cfc => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9d20; continue;
                pc = 0x801f9d04;
            }
            0x801f9d04 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f9d1c; continue; }
                pc = 0x801f9d0c;
            }
            0x801f9d0c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801f9d1c; continue; }
                pc = 0x801f9d14;
            }
            0x801f9d14 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9d20; continue;
                pc = 0x801f9d1c;
            }
            0x801f9d1c => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x801f9d20;
            }
            0x801f9d20 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f9f64; continue; }
                pc = 0x801f9d28;
            }
            0x801f9d28 => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801f9d3c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9d3c;
            }
            0x801f9d3c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801f9d50; continue; }
                pc = 0x801f9d48;
            }
            0x801f9d48 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801f9d6c; continue;
                pc = 0x801f9d50;
            }
            0x801f9d50 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801f9d60; continue; }
                pc = 0x801f9d58;
            }
            0x801f9d58 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801f9d68; continue; }
                pc = 0x801f9d60;
            }
            0x801f9d60 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801f9d6c; continue;
                pc = 0x801f9d68;
            }
            0x801f9d68 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x801f9d6c;
            }
            0x801f9d6c => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801f9d94; continue; }
                pc = 0x801f9d74;
            }
            0x801f9d74 => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x2;
                c.lr = 0x801f9d80; { c.steps = st; c.instrs = ins; c.cr = cr; f_80129280(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9d80;
            }
            0x801f9d80 => { ins += 2;
                c.r[4] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[4]);
                if cr_bit(cr, 2) { pc = 0x801f9f64; continue; }
                pc = 0x801f9d88;
            }
            0x801f9d88 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801f9d90; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012ac64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9d90;
            }
            0x801f9d90 => { ins += 1;
                pc = 0x801f9f64; continue;
                pc = 0x801f9d94;
            }
            0x801f9d94 => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801f9da8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9da8;
            }
            0x801f9da8 => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x801f9dbc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9dbc;
            }
            0x801f9dbc => { ins += 6;
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x801f9dd4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9dd4;
            }
            0x801f9dd4 => { ins += 1;
                c.lr = 0x801f9dd8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800fa280(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9dd8;
            }
            0x801f9dd8 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x801f9de0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800896b8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9de0;
            }
            0x801f9de0 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x801f9df4; continue; }
                pc = 0x801f9dec;
            }
            0x801f9dec => { ins += 1;
                c.lr = 0x801f9df0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800896c8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9df0;
            }
            0x801f9df0 => { ins += 1;
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x801f9df4;
            }
            0x801f9df4 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x801f9e04; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a1a4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9e04;
            }
            0x801f9e04 => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x801f9e18; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9e18;
            }
            0x801f9e18 => { ins += 1;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801f9e1c;
            }
            0x801f9e1c => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2;
                c.r[5] = 0x0;
                c.lr = 0x801f9e2c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a5b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9e2c;
            }
            0x801f9e2c => { ins += 4;
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x801f9e3c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f9f78(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9e3c;
            }
            0x801f9e3c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801f9e98; continue; }
                pc = 0x801f9e44;
            }
            0x801f9e44 => { ins += 6;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[7] = c.r[29].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0xc9;
                c.r[6] = 0x0;
                c.lr = 0x801f9e5c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9e5c;
            }
            0x801f9e5c => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[5] = 0x12;
                c.r[6] = 0x0;
                c.lr = 0x801f9e70; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9e70;
            }
            0x801f9e70 => { ins += 6;
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801f9e88; { c.steps = st; c.instrs = ins; c.cr = cr; f_80129f20(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9e88;
            }
            0x801f9e88 => { ins += 2;
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801f9e98; continue; }
                pc = 0x801f9e90;
            }
            0x801f9e90 => { ins += 2;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f9e1c; continue;
                pc = 0x801f9e98;
            }
            0x801f9e98 => { ins += 2;
                c.r[29] = 0x0;
                pc = 0x801f9ed8; continue;
                pc = 0x801f9ea0;
            }
            0x801f9ea0 => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[5] = 0x6;
                c.lr = 0x801f9eb4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9eb4;
            }
            0x801f9eb4 => { ins += 2;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[4]);
                if cr_bit(cr, 2) { pc = 0x801f9ed4; continue; }
                pc = 0x801f9ebc;
            }
            0x801f9ebc => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = 0x1;
                c.r[6] = 0xffffffff;
                c.lr = 0x801f9ecc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80129a78(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9ecc;
            }
            0x801f9ecc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 0) { pc = 0x801f9ee4; continue; }
                pc = 0x801f9ed4;
            }
            0x801f9ed4 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801f9ed8;
            }
            0x801f9ed8 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if cr_bit(cr, 0) { pc = 0x801f9ea0; continue; }
                pc = 0x801f9ee4;
            }
            0x801f9ee4 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0xf;
                c.r[5] = 0x0;
                c.lr = 0x801f9ef4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a450(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9ef4;
            }
            0x801f9ef4 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x10;
                c.r[5] = 0x0;
                c.lr = 0x801f9f04; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a450(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9f04;
            }
            0x801f9f04 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x11;
                c.r[5] = 0x0;
                c.lr = 0x801f9f14; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a450(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9f14;
            }
            0x801f9f14 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x12;
                c.r[5] = 0x0;
                c.lr = 0x801f9f24; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a450(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9f24;
            }
            0x801f9f24 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x13;
                c.r[5] = 0x0;
                c.lr = 0x801f9f34; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a450(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9f34;
            }
            0x801f9f34 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x14;
                c.r[5] = 0x0;
                c.lr = 0x801f9f44; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a450(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9f44;
            }
            0x801f9f44 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x15;
                c.r[5] = 0x0;
                c.lr = 0x801f9f54; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a450(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9f54;
            }
            0x801f9f54 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x16;
                c.r[5] = 0x0;
                c.lr = 0x801f9f64; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a450(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9f64;
            }
            0x801f9f64 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x140); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x154));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x150);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f9cbc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCreateFightTrainerPokemonDataIdToPokemon (0x801f9f78, 1340 bytes)
#[inline(never)]
pub fn f_801f9f78(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801f9f78) { return; }
        if c.prof { crate::prof::enter(c, 0x801f9f78); p = true; }
    }
    f_801f9f78_b(c, 0x801f9f78);
    if p { crate::prof::exit(c, 0x801f9f78); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801f9f78_at(c: &mut Cpu, pc: u32) { f_801f9f78_b(c, pc) }
#[inline(never)]
fn f_801f9f78_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801f9f78;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801f9f78, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801f9f78 => { ins += 26;
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x80280000;
                c.r[6] = 0x80280000;
                c.stw(c.r[1].wrapping_add(0x84), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x4c); for n in 19..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[25] = c.r[5] | c.r[5];
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x15;
                { let ea = c.r[7].wrapping_add(0xffff9c48); c.r[11] = c.lwz_t(ins, ea); c.r[7] = ea; }
                { let ea = c.r[6].wrapping_add(0xffff9c54); c.r[8] = c.lwz_t(ins, ea); c.r[6] = ea; }
                c.r[10] = c.lwz_t(ins, c.r[7].wrapping_add(0x4));
                c.r[9] = c.lwz_t(ins, c.r[7].wrapping_add(0x8));
                c.r[7] = c.lwz_t(ins, c.r[6].wrapping_add(0x4));
                c.r[0] = c.lwz_t(ins, c.r[6].wrapping_add(0x8));
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x20), c.r[11]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[0]);
                c.lr = 0x801f9fe0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801f9fe0;
            }
            0x801f9fe0 => { ins += 2;
                c.r[23] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[23]);
                if !cr_bit(cr, 2) { pc = 0x801f9ff0; continue; }
                pc = 0x801f9fe8;
            }
            0x801f9fe8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801fa4a0; continue;
                pc = 0x801f9ff0;
            }
            0x801f9ff0 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x13;
                c.r[6] = 0x0;
                c.lr = 0x801fa004; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa004;
            }
            0x801fa004 => { ins += 6;
                c.r[30] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x801fa01c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa01c;
            }
            0x801fa01c => { ins += 6;
                c.r[21] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0xe;
                c.r[6] = 0x0;
                c.lr = 0x801fa034; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa034;
            }
            0x801fa034 => { ins += 4;
                c.r[24] = c.r[1].wrapping_add(0xc);
                c.r[22] = c.r[3] | c.r[3];
                c.r[19] = 0x0;
                pc = 0x801fa064; continue;
                pc = 0x801fa044;
            }
            0x801fa044 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[19].rotate_left(0) & 0xff;
                c.r[3] = 0x0;
                c.r[5] = 0xf;
                c.lr = 0x801fa058; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa058;
            }
            0x801fa058 => { ins += 3;
                c.r[0] = c.r[19].rotate_left(0) & 0xff;
                c.r[19] = c.r[19].wrapping_add(0x1);
                c.stb(c.r[24].wrapping_add(c.r[0]), c.r[3]);
                pc = 0x801fa064;
            }
            0x801fa064 => { ins += 3;
                c.r[0] = c.r[19].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801fa044; continue; }
                pc = 0x801fa070;
            }
            0x801fa070 => { ins += 3;
                c.r[24] = c.r[1].wrapping_add(0x3c);
                c.r[19] = 0x0;
                pc = 0x801fa09c; continue;
                pc = 0x801fa07c;
            }
            0x801fa07c => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[19].rotate_left(0) & 0xff;
                c.r[3] = 0x0;
                c.r[5] = 0x10;
                c.lr = 0x801fa090; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa090;
            }
            0x801fa090 => { ins += 3;
                c.r[0] = c.r[19].rotate_left(1) & 0x1fe;
                c.r[19] = c.r[19].wrapping_add(0x1);
                c.sth(c.r[24].wrapping_add(c.r[0]), c.r[3]);
                pc = 0x801fa09c;
            }
            0x801fa09c => { ins += 3;
                c.r[0] = c.r[19].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801fa07c; continue; }
                pc = 0x801fa0a8;
            }
            0x801fa0a8 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x801fa0bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa0bc;
            }
            0x801fa0bc => { ins += 6;
                c.r[27] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                c.lr = 0x801fa0d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa0d4;
            }
            0x801fa0d4 => { ins += 5;
                c.r[24] = c.r[3] | c.r[3];
                c.r[29] = c.r[1].wrapping_add(0x2c);
                c.r[26] = c.r[1].wrapping_add(0x8);
                c.r[19] = 0x0;
                pc = 0x801fa124; continue;
                pc = 0x801fa0e8;
            }
            0x801fa0e8 => { ins += 6;
                c.r[20] = c.r[19].rotate_left(0) & 0xff;
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[20] | c.r[20];
                c.r[3] = 0x0;
                c.r[5] = 0x17;
                c.lr = 0x801fa100; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa100;
            }
            0x801fa100 => { ins += 7;
                c.r[0] = c.r[19].rotate_left(2) & 0x3fc;
                c.r[4] = c.r[28] | c.r[28];
                c.stw(c.r[29].wrapping_add(c.r[0]), c.r[3]);
                c.r[6] = c.r[20] | c.r[20];
                c.r[3] = 0x0;
                c.r[5] = 0x18;
                c.lr = 0x801fa11c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa11c;
            }
            0x801fa11c => { ins += 2;
                c.stb(c.r[26].wrapping_add(c.r[20]), c.r[3]);
                c.r[19] = c.r[19].wrapping_add(0x1);
                pc = 0x801fa124;
            }
            0x801fa124 => { ins += 3;
                c.r[0] = c.r[19].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801fa0e8; continue; }
                pc = 0x801fa130;
            }
            0x801fa130 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x19;
                c.r[6] = 0x0;
                c.lr = 0x801fa144; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa144;
            }
            0x801fa144 => { ins += 6;
                c.r[26] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x1a;
                c.r[6] = 0x0;
                c.lr = 0x801fa15c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa15c;
            }
            0x801fa15c => { ins += 6;
                c.r[29] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x1b;
                c.r[6] = 0x0;
                c.lr = 0x801fa174; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa174;
            }
            0x801fa174 => { ins += 4;
                c.r[28] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x801fa184; { c.steps = st; c.instrs = ins; c.cr = cr; f_80135938(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa184;
            }
            0x801fa184 => { ins += 5;
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[23] | c.r[23];
                c.r[5] = c.r[21] | c.r[21];
                c.lr = 0x801fa198; { c.steps = st; c.instrs = ins; c.cr = cr; f_801240c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa198;
            }
            0x801fa198 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[22], 0x0);
                if cr_bit(cr, 2) { pc = 0x801fa1c0; continue; }
                pc = 0x801fa1a0;
            }
            0x801fa1a0 => { ins += 2;
                c.r[3] = c.r[22] | c.r[22];
                c.lr = 0x801fa1a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800fa280(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa1a8;
            }
            0x801fa1a8 => { ins += 6;
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x77;
                c.r[6] = 0x0;
                c.lr = 0x801fa1c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa1c0;
            }
            0x801fa1c0 => { ins += 3;
                c.r[23] = c.r[30] | c.r[30];
                c.r[22] = 0x0;
                pc = 0x801fa254; continue;
                pc = 0x801fa1cc;
            }
            0x801fa1cc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801fa21c; continue; }
                pc = 0x801fa1d4;
            }
            0x801fa1d4 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801fa1dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ee8f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa1dc;
            }
            0x801fa1dc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801fa21c; continue; }
                pc = 0x801fa1e8;
            }
            0x801fa1e8 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[22].rotate_left(0) & 0xff;
                c.lr = 0x801fa1f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ee824(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa1f4;
            }
            0x801fa1f4 => { ins += 9;
                c.r[5] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[22].rotate_left(1) & 0x1fe;
                c.r[4] = c.r[1].wrapping_add(0x20);
                c.r[3] = c.r[31] | c.r[31];
                c.r[7] = c.r[5].rotate_left(0) & 0xffff;
                c.r[5] = c.lhz_t(ins, c.r[4].wrapping_add(c.r[0]));
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x801fa218; { c.steps = st; c.instrs = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa218;
            }
            0x801fa218 => { ins += 1;
                pc = 0x801fa250; continue;
                pc = 0x801fa21c;
            }
            0x801fa21c => { ins += 5;
                c.r[5] = c.r[22].rotate_left(0) & 0xff;
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[5]));
                c.r[0] = c.r[4] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801fa250; continue; }
                pc = 0x801fa230;
            }
            0x801fa230 => { ins += 8;
                c.r[0] = c.r[5].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[1].wrapping_add(0x20);
                c.r[5] = c.lhz_t(ins, c.r[3].wrapping_add(c.r[0]));
                c.r[3] = c.r[31] | c.r[31];
                c.r[7] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x801fa250; { c.steps = st; c.instrs = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa250;
            }
            0x801fa250 => { ins += 1;
                c.r[22] = c.r[22].wrapping_add(0x1);
                pc = 0x801fa254;
            }
            0x801fa254 => { ins += 3;
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801fa1cc; continue; }
                pc = 0x801fa260;
            }
            0x801fa260 => { ins += 4;
                c.r[22] = c.r[1].wrapping_add(0x3c);
                c.r[21] = c.r[1].wrapping_add(0x14);
                c.r[20] = 0x0;
                pc = 0x801fa298; continue;
                pc = 0x801fa270;
            }
            0x801fa270 => { ins += 4;
                c.r[3] = c.r[20].rotate_left(1) & 0x1fe;
                c.r[7] = c.lha_t(ins, c.r[22].wrapping_add(c.r[3]));
                c.r[0] = c.r[7] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801fa294; continue; }
                pc = 0x801fa280;
            }
            0x801fa280 => { ins += 5;
                c.r[5] = c.lhz_t(ins, c.r[21].wrapping_add(c.r[3]));
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x801fa294; { c.steps = st; c.instrs = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa294;
            }
            0x801fa294 => { ins += 1;
                c.r[20] = c.r[20].wrapping_add(0x1);
                pc = 0x801fa298;
            }
            0x801fa298 => { ins += 3;
                c.r[0] = c.r[20].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801fa270; continue; }
                pc = 0x801fa2a4;
            }
            0x801fa2a4 => { ins += 2;
                c.r[0] = c.r[27] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801fa2b8; continue; }
                pc = 0x801fa2ac;
            }
            0x801fa2ac => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27].rotate_left(0) & 0xff;
                c.lr = 0x801fa2b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80124978(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa2b8;
            }
            0x801fa2b8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[24], 0x0);
                if cr_bit(cr, 0) { pc = 0x801fa2e8; continue; }
                pc = 0x801fa2c0;
            }
            0x801fa2c0 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fa2d8; continue; }
                pc = 0x801fa2c4;
            }
            0x801fa2c4 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x801fa2d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80123110(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa2d4;
            }
            0x801fa2d4 => { ins += 1;
                pc = 0x801fa2e8; continue;
                pc = 0x801fa2d8;
            }
            0x801fa2d8 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[24].rotate_left(0) & 0xffff;
                c.r[5] = 0x1;
                c.lr = 0x801fa2e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80123110(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa2e8;
            }
            0x801fa2e8 => { ins += 3;
                c.r[20] = c.r[1].wrapping_add(0x2c);
                c.r[24] = 0x0;
                pc = 0x801fa36c; continue;
                pc = 0x801fa2f4;
            }
            0x801fa2f4 => { ins += 5;
                c.r[0] = c.r[24].rotate_left(2) & 0x3fc;
                c.r[21] = c.r[24].rotate_left(0) & 0xff;
                c.r[3] = c.lwz_t(ins, c.r[20].wrapping_add(c.r[0]));
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 0) { pc = 0x801fa368; continue; }
                pc = 0x801fa308;
            }
            0x801fa308 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fa334; continue; }
                pc = 0x801fa30c;
            }
            0x801fa30c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x164);
                if !cr_bit(cr, 2) { pc = 0x801fa334; continue; }
                pc = 0x801fa314;
            }
            0x801fa314 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x165);
                if !cr_bit(cr, 2) { pc = 0x801fa334; continue; }
                pc = 0x801fa31c;
            }
            0x801fa31c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x163);
                if !cr_bit(cr, 2) { pc = 0x801fa334; continue; }
                pc = 0x801fa324;
            }
            0x801fa324 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x801fa330; { c.steps = st; c.instrs = ins; c.cr = cr; f_80125314(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa330;
            }
            0x801fa330 => { ins += 1;
                pc = 0x801fa368; continue;
                pc = 0x801fa334;
            }
            0x801fa334 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[20].wrapping_add(c.r[0]));
                c.r[22] = c.r[24].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[22] | c.r[22];
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x801fa34c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80123d58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa34c;
            }
            0x801fa34c => { ins += 7;
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[3] = c.r[31] | c.r[31];
                c.r[7] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[21]));
                c.r[6] = c.r[22] | c.r[22];
                c.r[4] = 0x0;
                c.r[5] = 0x81;
                c.lr = 0x801fa368; { c.steps = st; c.instrs = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa368;
            }
            0x801fa368 => { ins += 1;
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x801fa36c;
            }
            0x801fa36c => { ins += 3;
                c.r[0] = c.r[24].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801fa2f4; continue; }
                pc = 0x801fa378;
            }
            0x801fa378 => { ins += 2;
                c.r[0] = c.r[26] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801fa398; continue; }
                pc = 0x801fa380;
            }
            0x801fa380 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[7] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x99;
                c.r[6] = 0x0;
                c.lr = 0x801fa398; { c.steps = st; c.instrs = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa398;
            }
            0x801fa398 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801fa3d8; continue; }
                pc = 0x801fa3a0;
            }
            0x801fa3a0 => { ins += 2;
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x801fa3a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ee8f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa3a8;
            }
            0x801fa3a8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801fa3d8; continue; }
                pc = 0x801fa3b4;
            }
            0x801fa3b4 => { ins += 2;
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x801fa3bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ee750(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa3bc;
            }
            0x801fa3bc => { ins += 6;
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x6f;
                c.r[6] = 0x0;
                c.lr = 0x801fa3d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa3d4;
            }
            0x801fa3d4 => { ins += 1;
                pc = 0x801fa408; continue;
                pc = 0x801fa3d8;
            }
            0x801fa3d8 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[28] | c.r[28];
                c.r[7] = c.r[25] | c.r[25];
                c.r[6] = 0x0;
                c.lr = 0x801fa3f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80124410(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa3f0;
            }
            0x801fa3f0 => { ins += 6;
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x6f;
                c.r[6] = 0x0;
                c.lr = 0x801fa408; { c.steps = st; c.instrs = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa408;
            }
            0x801fa408 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x801fa494; continue; }
                pc = 0x801fa410;
            }
            0x801fa410 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x801fa41c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8011fca4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa41c;
            }
            0x801fa41c => { ins += 2;
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x801fa424; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ee8f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa424;
            }
            0x801fa424 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801fa494; continue; }
                pc = 0x801fa42c;
            }
            0x801fa42c => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x6f;
                c.r[6] = 0x0;
                c.lr = 0x801fa440; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa440;
            }
            0x801fa440 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x801fa44c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ee7bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa44c;
            }
            0x801fa44c => { ins += 3;
                c.r[21] = c.r[1].wrapping_add(0x20);
                c.r[20] = 0x0;
                pc = 0x801fa488; continue;
                pc = 0x801fa458;
            }
            0x801fa458 => { ins += 6;
                c.r[0] = c.r[20].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.lhz_t(ins, c.r[21].wrapping_add(c.r[0]));
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x801fa470; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa470;
            }
            0x801fa470 => { ins += 5;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = c.r[20].rotate_left(0) & 0xff;
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                c.lr = 0x801fa484; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ee894(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa484;
            }
            0x801fa484 => { ins += 1;
                c.r[20] = c.r[20].wrapping_add(0x1);
                pc = 0x801fa488;
            }
            0x801fa488 => { ins += 3;
                c.r[0] = c.r[20].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801fa458; continue; }
                pc = 0x801fa494;
            }
            0x801fa494 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801fa49c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012546c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa49c;
            }
            0x801fa49c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x801fa4a0;
            }
            0x801fa4a0 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x4c); for n in 19..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801f9f78: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCheckTrainerDataIdValid (0x801fa4b4, 112 bytes)
#[inline(never)]
pub fn f_801fa4b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fa4b4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fa4b4); p = true; }
    }
    f_801fa4b4_b(c, 0x801fa4b4);
    if p { crate::prof::exit(c, 0x801fa4b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fa4b4_at(c: &mut Cpu, pc: u32) { f_801fa4b4_b(c, pc) }
#[inline(always)]
fn f_801fa4b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fa4b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fa4b4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fa4b4 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = 0x0;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x801fa4e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa4e0;
            }
            0x801fa4e0 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                if !cr_bit(cr, 2) { pc = 0x801fa4f4; continue; }
                pc = 0x801fa4ec;
            }
            0x801fa4ec => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801fa510; continue;
                pc = 0x801fa4f4;
            }
            0x801fa4f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fa50c; continue; }
                pc = 0x801fa4fc;
            }
            0x801fa4fc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fa50c; continue; }
                pc = 0x801fa504;
            }
            0x801fa504 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801fa510; continue;
                pc = 0x801fa50c;
            }
            0x801fa50c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x801fa510;
            }
            0x801fa510 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fa4b4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCheckDoFight (0x801fa524, 272 bytes)
#[inline(never)]
pub fn f_801fa524(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fa524) { return; }
        if c.prof { crate::prof::enter(c, 0x801fa524); p = true; }
    }
    f_801fa524_b(c, 0x801fa524);
    if p { crate::prof::exit(c, 0x801fa524); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fa524_at(c: &mut Cpu, pc: u32) { f_801fa524_b(c, pc) }
#[inline(always)]
fn f_801fa524_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fa524;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fa524, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fa524 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                if !cr_bit(cr, 2) { pc = 0x801fa544; continue; }
                pc = 0x801fa53c;
            }
            0x801fa53c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801fa5bc; continue;
                pc = 0x801fa544;
            }
            0x801fa544 => { ins += 1;
                c.lr = 0x801fa548; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa548;
            }
            0x801fa548 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801fa55c; continue; }
                pc = 0x801fa554;
            }
            0x801fa554 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801fa5bc; continue;
                pc = 0x801fa55c;
            }
            0x801fa55c => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801fa570; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa570;
            }
            0x801fa570 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fa580; continue; }
                pc = 0x801fa578;
            }
            0x801fa578 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801fa5bc; continue;
                pc = 0x801fa580;
            }
            0x801fa580 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801fa594; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa594;
            }
            0x801fa594 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fa5a4; continue; }
                pc = 0x801fa59c;
            }
            0x801fa59c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801fa5bc; continue;
                pc = 0x801fa5a4;
            }
            0x801fa5a4 => { ins += 1;
                c.lr = 0x801fa5a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a130(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa5a8;
            }
            0x801fa5a8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801fa5b8; continue; }
                pc = 0x801fa5b0;
            }
            0x801fa5b0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801fa5bc; continue;
                pc = 0x801fa5b8;
            }
            0x801fa5b8 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x801fa5bc;
            }
            0x801fa5bc => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801fa5cc; continue; }
                pc = 0x801fa5c4;
            }
            0x801fa5c4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801fa620; continue;
                pc = 0x801fa5cc;
            }
            0x801fa5cc => { ins += 3;
                c.r[31] = 0x0;
                c.r[30] = c.r[31] | c.r[31];
                pc = 0x801fa608; continue;
                pc = 0x801fa5d8;
            }
            0x801fa5d8 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[30].rotate_left(0) & 0xff;
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x801fa5ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa5ec;
            }
            0x801fa5ec => { ins += 1;
                c.lr = 0x801fa5f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa5f0;
            }
            0x801fa5f0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801fa604; continue; }
                pc = 0x801fa5f8;
            }
            0x801fa5f8 => { ins += 3;
                c.r[3] = c.r[31].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[31] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x801fa604;
            }
            0x801fa604 => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x801fa608;
            }
            0x801fa608 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801fa5d8; continue; }
                pc = 0x801fa614;
            }
            0x801fa614 => { ins += 3;
                c.r[3] = c.r[31].rotate_left(0) & 0xff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                pc = 0x801fa620;
            }
            0x801fa620 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fa524: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCheckValid (0x801fa634, 164 bytes)
#[inline(never)]
pub fn f_801fa634(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fa634) { return; }
        if c.prof { crate::prof::enter(c, 0x801fa634); p = true; }
    }
    f_801fa634_b(c, 0x801fa634);
    if p { crate::prof::exit(c, 0x801fa634); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fa634_at(c: &mut Cpu, pc: u32) { f_801fa634_b(c, pc) }
#[inline(always)]
fn f_801fa634_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fa634;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fa634, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fa634 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x801fa654; continue; }
                pc = 0x801fa64c;
            }
            0x801fa64c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801fa6c4; continue;
                pc = 0x801fa654;
            }
            0x801fa654 => { ins += 1;
                c.lr = 0x801fa658; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa658;
            }
            0x801fa658 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801fa66c; continue; }
                pc = 0x801fa664;
            }
            0x801fa664 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801fa6c4; continue;
                pc = 0x801fa66c;
            }
            0x801fa66c => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801fa680; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa680;
            }
            0x801fa680 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fa690; continue; }
                pc = 0x801fa688;
            }
            0x801fa688 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801fa6c4; continue;
                pc = 0x801fa690;
            }
            0x801fa690 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801fa6a4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa6a4;
            }
            0x801fa6a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fa6b4; continue; }
                pc = 0x801fa6ac;
            }
            0x801fa6ac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801fa6c4; continue;
                pc = 0x801fa6b4;
            }
            0x801fa6b4 => { ins += 1;
                c.lr = 0x801fa6b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a130(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa6b8;
            }
            0x801fa6b8 => { ins += 3;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                pc = 0x801fa6c4;
            }
            0x801fa6c4 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fa634: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerCreate (0x801fa6d8, 500 bytes)
#[inline(never)]
pub fn f_801fa6d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fa6d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fa6d8); p = true; }
    }
    f_801fa6d8_b(c, 0x801fa6d8);
    if p { crate::prof::exit(c, 0x801fa6d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fa6d8_at(c: &mut Cpu, pc: u32) { f_801fa6d8_b(c, pc) }
#[inline(never)]
fn f_801fa6d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fa6d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fa6d8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fa6d8 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[27]);
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = c.r[7] | c.r[7];
                if cr_bit(cr, 2) { pc = 0x801fa8b8; continue; }
                pc = 0x801fa700;
            }
            0x801fa700 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x801fa8b8; continue; }
                pc = 0x801fa708;
            }
            0x801fa708 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x0);
                if cr_bit(cr, 2) { pc = 0x801fa840; continue; }
                pc = 0x801fa710;
            }
            0x801fa710 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801fa720; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa720;
            }
            0x801fa720 => { ins += 1;
                c.lr = 0x801fa724; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a248(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa724;
            }
            0x801fa724 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801fa73c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa73c;
            }
            0x801fa73c => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.r[6] = 0x0;
                c.lr = 0x801fa750; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa750;
            }
            0x801fa750 => { ins += 2;
                c.r[4] = 0x6;
                c.lr = 0x801fa758; { c.steps = st; c.instrs = ins; c.cr = cr; f_802077d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa758;
            }
            0x801fa758 => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.r[6] = 0x0;
                c.lr = 0x801fa76c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa76c;
            }
            0x801fa76c => { ins += 2;
                c.r[4] = 0x2;
                c.lr = 0x801fa774; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206c3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa774;
            }
            0x801fa774 => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x47;
                c.r[6] = 0x0;
                c.lr = 0x801fa788; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa788;
            }
            0x801fa788 => { ins += 1;
                c.lr = 0x801fa78c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206c94(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa78c;
            }
            0x801fa78c => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x48;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x801fa7a4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa7a4;
            }
            0x801fa7a4 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x49;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801fa7bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa7bc;
            }
            0x801fa7bc => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x4a;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801fa7d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa7d4;
            }
            0x801fa7d4 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x4b;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801fa7ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa7ec;
            }
            0x801fa7ec => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801fa804; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa804;
            }
            0x801fa804 => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x4d;
                c.r[6] = 0x0;
                c.lr = 0x801fa818; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa818;
            }
            0x801fa818 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801fa824; continue; }
                pc = 0x801fa820;
            }
            0x801fa820 => { ins += 1;
                c.lr = 0x801fa824; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f1460(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa824;
            }
            0x801fa824 => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x4e;
                c.r[6] = 0x0;
                c.lr = 0x801fa838; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa838;
            }
            0x801fa838 => { ins += 2;
                c.r[4] = 0xc;
                c.lr = 0x801fa840; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020ece0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa840;
            }
            0x801fa840 => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801fa854; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa854;
            }
            0x801fa854 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801fa8b8; continue; }
                pc = 0x801fa85c;
            }
            0x801fa85c => { ins += 2;
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x801fa864; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012ac64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa864;
            }
            0x801fa864 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[29].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x801fa87c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa87c;
            }
            0x801fa87c => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x4b;
                c.r[6] = 0x0;
                c.lr = 0x801fa894; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa894;
            }
            0x801fa894 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x801fa8b8; continue; }
                pc = 0x801fa89c;
            }
            0x801fa89c => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x801fa8b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa8b4;
            }
            0x801fa8b4 => { ins += 1;
                c.lr = 0x801fa8b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f198c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa8b8;
            }
            0x801fa8b8 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fa6d8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerInit (0x801fa8cc, 396 bytes)
#[inline(never)]
pub fn f_801fa8cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fa8cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801fa8cc); p = true; }
    }
    f_801fa8cc_b(c, 0x801fa8cc);
    if p { crate::prof::exit(c, 0x801fa8cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fa8cc_at(c: &mut Cpu, pc: u32) { f_801fa8cc_b(c, pc) }
#[inline(always)]
fn f_801fa8cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fa8cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fa8cc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fa8cc => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if cr_bit(cr, 2) { pc = 0x801faa44; continue; }
                pc = 0x801fa8e4;
            }
            0x801fa8e4 => { ins += 3;
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.r[29] = 0x0;
                pc = 0x801faa38; continue;
                pc = 0x801fa8f0;
            }
            0x801fa8f0 => { ins += 4;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].wrapping_mul(0x28e4);
                c.r[30] = c.r[28].wrapping_add(c.r[0]);
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x801faa34; continue; }
                pc = 0x801fa900;
            }
            0x801fa900 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x801fa914; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa914;
            }
            0x801fa914 => { ins += 1;
                c.lr = 0x801fa918; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012a248(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa918;
            }
            0x801fa918 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801fa930; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa930;
            }
            0x801fa930 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.r[6] = 0x0;
                c.lr = 0x801fa944; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa944;
            }
            0x801fa944 => { ins += 2;
                c.r[4] = 0x6;
                c.lr = 0x801fa94c; { c.steps = st; c.instrs = ins; c.cr = cr; f_802077d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa94c;
            }
            0x801fa94c => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.r[6] = 0x0;
                c.lr = 0x801fa960; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa960;
            }
            0x801fa960 => { ins += 2;
                c.r[4] = 0x2;
                c.lr = 0x801fa968; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206c3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa968;
            }
            0x801fa968 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x47;
                c.r[6] = 0x0;
                c.lr = 0x801fa97c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa97c;
            }
            0x801fa97c => { ins += 1;
                c.lr = 0x801fa980; { c.steps = st; c.instrs = ins; c.cr = cr; f_80206c94(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa980;
            }
            0x801fa980 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x48;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x801fa998; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa998;
            }
            0x801fa998 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x49;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801fa9b0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa9b0;
            }
            0x801fa9b0 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x4a;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801fa9c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa9c8;
            }
            0x801fa9c8 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x4b;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801fa9e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa9e0;
            }
            0x801fa9e0 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801fa9f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801faa58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fa9f8;
            }
            0x801fa9f8 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x4d;
                c.r[6] = 0x0;
                c.lr = 0x801faa0c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faa0c;
            }
            0x801faa0c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x801faa18; continue; }
                pc = 0x801faa14;
            }
            0x801faa14 => { ins += 1;
                c.lr = 0x801faa18; { c.steps = st; c.instrs = ins; c.cr = cr; f_801f1460(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faa18;
            }
            0x801faa18 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x4e;
                c.r[6] = 0x0;
                c.lr = 0x801faa2c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faa2c;
            }
            0x801faa2c => { ins += 2;
                c.r[4] = 0xc;
                c.lr = 0x801faa34; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020ece0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faa34;
            }
            0x801faa34 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801faa38;
            }
            0x801faa38 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x801fa8f0; continue; }
                pc = 0x801faa44;
            }
            0x801faa44 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fa8cc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSetStatus (0x801faa58, 1896 bytes)
#[inline(never)]
pub fn f_801faa58(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801faa58) { return; }
        if c.prof { crate::prof::enter(c, 0x801faa58); p = true; }
    }
    f_801faa58_b(c, 0x801faa58);
    if p { crate::prof::exit(c, 0x801faa58); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801faa58_at(c: &mut Cpu, pc: u32) { f_801faa58_b(c, pc) }
#[inline(never)]
fn f_801faa58_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801faa58;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801faa58, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801faa58 => { ins += 1;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x801faa5c;
            }
            0x801faa5c => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x801faa60;
            }
            0x801faa60 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x801faa64;
            }
            0x801faa64 => { ins += 1;
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x801faa68;
            }
            0x801faa68 => { ins += 1;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x801faa6c;
            }
            0x801faa6c => { ins += 1;
                c.r[30] = c.r[5] | c.r[5];
                pc = 0x801faa70;
            }
            0x801faa70 => { ins += 1;
                c.r[28] = c.r[4] | c.r[4];
                pc = 0x801faa74;
            }
            0x801faa74 => { ins += 1;
                c.r[29] = c.r[6] | c.r[6];
                pc = 0x801faa78;
            }
            0x801faa78 => { ins += 1;
                c.r[31] = c.r[7] | c.r[7];
                pc = 0x801faa7c;
            }
            0x801faa7c => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801faa80;
            }
            0x801faa80 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5b);
                pc = 0x801faa84;
            }
            0x801faa84 => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x801faa8c; continue; }
                pc = 0x801faa88;
            }
            0x801faa88 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801faa8c;
            }
            0x801faa8c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xa);
                pc = 0x801faa90;
            }
            0x801faa90 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801faaa8; continue; }
                pc = 0x801faa94;
            }
            0x801faa94 => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x801faa98;
            }
            0x801faa98 => { ins += 1;
                c.lr = 0x801faa9c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fccc4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faa9c;
            }
            0x801faa9c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faaa0;
            }
            0x801faaa0 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fab14; continue; }
                pc = 0x801faaa4;
            }
            0x801faaa4 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801faaa8;
            }
            0x801faaa8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xd);
                pc = 0x801faaac;
            }
            0x801faaac => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801faac4; continue; }
                pc = 0x801faab0;
            }
            0x801faab0 => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x801faab4;
            }
            0x801faab4 => { ins += 1;
                c.lr = 0x801faab8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcad0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faab8;
            }
            0x801faab8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faabc;
            }
            0x801faabc => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fab14; continue; }
                pc = 0x801faac0;
            }
            0x801faac0 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801faac4;
            }
            0x801faac4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1e);
                pc = 0x801faac8;
            }
            0x801faac8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801faae0; continue; }
                pc = 0x801faacc;
            }
            0x801faacc => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x801faad0;
            }
            0x801faad0 => { ins += 1;
                c.lr = 0x801faad4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fca2c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faad4;
            }
            0x801faad4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faad8;
            }
            0x801faad8 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fab14; continue; }
                pc = 0x801faadc;
            }
            0x801faadc => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801faae0;
            }
            0x801faae0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3d);
                pc = 0x801faae4;
            }
            0x801faae4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801faafc; continue; }
                pc = 0x801faae8;
            }
            0x801faae8 => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x801faaec;
            }
            0x801faaec => { ins += 1;
                c.lr = 0x801faaf0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc658(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faaf0;
            }
            0x801faaf0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faaf4;
            }
            0x801faaf4 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fab14; continue; }
                pc = 0x801faaf8;
            }
            0x801faaf8 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801faafc;
            }
            0x801faafc => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x42);
                pc = 0x801fab00;
            }
            0x801fab00 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fab14; continue; }
                pc = 0x801fab04;
            }
            0x801fab04 => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x801fab08;
            }
            0x801fab08 => { ins += 1;
                c.lr = 0x801fab0c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbfbc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fab0c;
            }
            0x801fab0c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fab10;
            }
            0x801fab10 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801fab14;
            }
            0x801fab14 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fab18;
            }
            0x801fab18 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801fab1c;
            }
            0x801fab1c => { ins += 1;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                pc = 0x801fab20;
            }
            0x801fab20 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x59);
                pc = 0x801fab24;
            }
            0x801fab24 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x801fb1ac; continue; }
                pc = 0x801fab28;
            }
            0x801fab28 => { ins += 1;
                c.r[4] = 0x80370000;
                pc = 0x801fab2c;
            }
            0x801fab2c => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x801fab30;
            }
            0x801fab30 => { ins += 1;
                c.r[4] = c.r[4].wrapping_add(0x5670);
                pc = 0x801fab34;
            }
            0x801fab34 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(c.r[0]));
                pc = 0x801fab38;
            }
            0x801fab38 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x801fab3c;
            }
            0x801fab3c => { ins += 1;
                if true { let t = c.ctr; if (0x801faa58..0x801fb1c0).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fab40;
            }
            0x801fab40 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fab44;
            }
            0x801fab44 => { ins += 1;
                c.lr = 0x801fab48; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcb94(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fab48;
            }
            0x801fab48 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fab4c;
            }
            0x801fab4c => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801fab50;
            }
            0x801fab50 => { ins += 1;
                c.lr = 0x801fab54; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcb84(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fab54;
            }
            0x801fab54 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fab58;
            }
            0x801fab58 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fab5c;
            }
            0x801fab5c => { ins += 1;
                c.lr = 0x801fab60; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcb74(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fab60;
            }
            0x801fab60 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fab64;
            }
            0x801fab64 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801fab68;
            }
            0x801fab68 => { ins += 1;
                c.lr = 0x801fab6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcc54(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fab6c;
            }
            0x801fab6c => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fab70;
            }
            0x801fab70 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801fab74;
            }
            0x801fab74 => { ins += 1;
                c.lr = 0x801fab78; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcb64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fab78;
            }
            0x801fab78 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fab7c;
            }
            0x801fab7c => { ins += 1;
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                pc = 0x801fab80;
            }
            0x801fab80 => { ins += 1;
                c.r[5] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801fab84;
            }
            0x801fab84 => { ins += 1;
                c.lr = 0x801fab88; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcb40(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fab88;
            }
            0x801fab88 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fab8c;
            }
            0x801fab8c => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fab90;
            }
            0x801fab90 => { ins += 1;
                c.lr = 0x801fab94; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcb30(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fab94;
            }
            0x801fab94 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fab98;
            }
            0x801fab98 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801fab9c;
            }
            0x801fab9c => { ins += 1;
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                pc = 0x801faba0;
            }
            0x801faba0 => { ins += 1;
                c.lr = 0x801faba4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcb0c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faba4;
            }
            0x801faba4 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801faba8;
            }
            0x801faba8 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fabac;
            }
            0x801fabac => { ins += 1;
                c.lr = 0x801fabb0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcafc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fabb0;
            }
            0x801fabb0 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fabb4;
            }
            0x801fabb4 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fabb8;
            }
            0x801fabb8 => { ins += 1;
                c.lr = 0x801fabbc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fca78(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fabbc;
            }
            0x801fabbc => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fabc0;
            }
            0x801fabc0 => { ins += 1;
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                pc = 0x801fabc4;
            }
            0x801fabc4 => { ins += 1;
                c.r[5] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fabc8;
            }
            0x801fabc8 => { ins += 1;
                c.lr = 0x801fabcc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fca58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fabcc;
            }
            0x801fabcc => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fabd0;
            }
            0x801fabd0 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fabd4;
            }
            0x801fabd4 => { ins += 1;
                c.lr = 0x801fabd8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc828(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fabd8;
            }
            0x801fabd8 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fabdc;
            }
            0x801fabdc => { ins += 1;
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                pc = 0x801fabe0;
            }
            0x801fabe0 => { ins += 1;
                c.r[5] = c.r[31] as u8 as i8 as i32 as u32;
                pc = 0x801fabe4;
            }
            0x801fabe4 => { ins += 1;
                c.lr = 0x801fabe8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc808(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fabe8;
            }
            0x801fabe8 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fabec;
            }
            0x801fabec => { ins += 1;
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                pc = 0x801fabf0;
            }
            0x801fabf0 => { ins += 1;
                c.r[5] = c.r[31] as u16 as i16 as i32 as u32;
                pc = 0x801fabf4;
            }
            0x801fabf4 => { ins += 1;
                c.lr = 0x801fabf8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc7e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fabf8;
            }
            0x801fabf8 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fabfc;
            }
            0x801fabfc => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fac00;
            }
            0x801fac00 => { ins += 1;
                c.lr = 0x801fac04; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc7d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac04;
            }
            0x801fac04 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac08;
            }
            0x801fac08 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801fac0c;
            }
            0x801fac0c => { ins += 1;
                c.lr = 0x801fac10; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc7c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac10;
            }
            0x801fac10 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac14;
            }
            0x801fac14 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fac18;
            }
            0x801fac18 => { ins += 1;
                c.lr = 0x801fac1c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc7b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac1c;
            }
            0x801fac1c => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac20;
            }
            0x801fac20 => { ins += 1;
                c.r[4] = c.r[31] as u8 as i8 as i32 as u32;
                pc = 0x801fac24;
            }
            0x801fac24 => { ins += 1;
                c.lr = 0x801fac28; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc7a4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac28;
            }
            0x801fac28 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac2c;
            }
            0x801fac2c => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801fac30;
            }
            0x801fac30 => { ins += 1;
                c.lr = 0x801fac34; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc794(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac34;
            }
            0x801fac34 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac38;
            }
            0x801fac38 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fac3c;
            }
            0x801fac3c => { ins += 1;
                c.lr = 0x801fac40; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc784(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac40;
            }
            0x801fac40 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac44;
            }
            0x801fac44 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801fac48;
            }
            0x801fac48 => { ins += 1;
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                pc = 0x801fac4c;
            }
            0x801fac4c => { ins += 1;
                c.lr = 0x801fac50; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc744(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac50;
            }
            0x801fac50 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac54;
            }
            0x801fac54 => { ins += 1;
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                pc = 0x801fac58;
            }
            0x801fac58 => { ins += 1;
                c.r[5] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fac5c;
            }
            0x801fac5c => { ins += 1;
                c.lr = 0x801fac60; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc704(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac60;
            }
            0x801fac60 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac64;
            }
            0x801fac64 => { ins += 1;
                c.r[4] = c.r[31] as u16 as i16 as i32 as u32;
                pc = 0x801fac68;
            }
            0x801fac68 => { ins += 1;
                c.lr = 0x801fac6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc6f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac6c;
            }
            0x801fac6c => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac70;
            }
            0x801fac70 => { ins += 1;
                c.r[4] = c.r[31] as u8 as i8 as i32 as u32;
                pc = 0x801fac74;
            }
            0x801fac74 => { ins += 1;
                c.lr = 0x801fac78; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc6e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac78;
            }
            0x801fac78 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac7c;
            }
            0x801fac7c => { ins += 1;
                c.r[4] = c.r[31] as u8 as i8 as i32 as u32;
                pc = 0x801fac80;
            }
            0x801fac80 => { ins += 1;
                c.lr = 0x801fac84; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc6d4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac84;
            }
            0x801fac84 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac88;
            }
            0x801fac88 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fac8c;
            }
            0x801fac8c => { ins += 1;
                c.lr = 0x801fac90; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc694(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac90;
            }
            0x801fac90 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fac94;
            }
            0x801fac94 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fac98;
            }
            0x801fac98 => { ins += 1;
                c.lr = 0x801fac9c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc684(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fac9c;
            }
            0x801fac9c => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801faca0;
            }
            0x801faca0 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801faca4;
            }
            0x801faca4 => { ins += 1;
                c.lr = 0x801faca8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc288(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faca8;
            }
            0x801faca8 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801facac;
            }
            0x801facac => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801facb0;
            }
            0x801facb0 => { ins += 1;
                c.lr = 0x801facb4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc278(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801facb4;
            }
            0x801facb4 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801facb8;
            }
            0x801facb8 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801facbc;
            }
            0x801facbc => { ins += 1;
                c.lr = 0x801facc0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc268(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801facc0;
            }
            0x801facc0 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801facc4;
            }
            0x801facc4 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801facc8;
            }
            0x801facc8 => { ins += 1;
                c.lr = 0x801faccc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc258(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faccc;
            }
            0x801faccc => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801facd0;
            }
            0x801facd0 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801facd4;
            }
            0x801facd4 => { ins += 1;
                c.lr = 0x801facd8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc248(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801facd8;
            }
            0x801facd8 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801facdc;
            }
            0x801facdc => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801face0;
            }
            0x801face0 => { ins += 1;
                c.lr = 0x801face4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc238(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801face4;
            }
            0x801face4 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801face8;
            }
            0x801face8 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801facec;
            }
            0x801facec => { ins += 1;
                c.lr = 0x801facf0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc228(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801facf0;
            }
            0x801facf0 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801facf4;
            }
            0x801facf4 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801facf8;
            }
            0x801facf8 => { ins += 1;
                c.lr = 0x801facfc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc218(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801facfc;
            }
            0x801facfc => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad00;
            }
            0x801fad00 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad04;
            }
            0x801fad04 => { ins += 1;
                c.lr = 0x801fad08; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc208(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad08;
            }
            0x801fad08 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad0c;
            }
            0x801fad0c => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad10;
            }
            0x801fad10 => { ins += 1;
                c.lr = 0x801fad14; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc1f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad14;
            }
            0x801fad14 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad18;
            }
            0x801fad18 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad1c;
            }
            0x801fad1c => { ins += 1;
                c.lr = 0x801fad20; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc1e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad20;
            }
            0x801fad20 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad24;
            }
            0x801fad24 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad28;
            }
            0x801fad28 => { ins += 1;
                c.lr = 0x801fad2c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc1d8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad2c;
            }
            0x801fad2c => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad30;
            }
            0x801fad30 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad34;
            }
            0x801fad34 => { ins += 1;
                c.lr = 0x801fad38; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc140(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad38;
            }
            0x801fad38 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad3c;
            }
            0x801fad3c => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad40;
            }
            0x801fad40 => { ins += 1;
                c.lr = 0x801fad44; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc130(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad44;
            }
            0x801fad44 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad48;
            }
            0x801fad48 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad4c;
            }
            0x801fad4c => { ins += 1;
                c.lr = 0x801fad50; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc120(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad50;
            }
            0x801fad50 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad54;
            }
            0x801fad54 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad58;
            }
            0x801fad58 => { ins += 1;
                c.lr = 0x801fad5c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc110(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad5c;
            }
            0x801fad5c => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad60;
            }
            0x801fad60 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad64;
            }
            0x801fad64 => { ins += 1;
                c.lr = 0x801fad68; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc100(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad68;
            }
            0x801fad68 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad6c;
            }
            0x801fad6c => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad70;
            }
            0x801fad70 => { ins += 1;
                c.lr = 0x801fad74; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc0f0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad74;
            }
            0x801fad74 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad78;
            }
            0x801fad78 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad7c;
            }
            0x801fad7c => { ins += 1;
                c.lr = 0x801fad80; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc0e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad80;
            }
            0x801fad80 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad84;
            }
            0x801fad84 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad88;
            }
            0x801fad88 => { ins += 1;
                c.lr = 0x801fad8c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc0d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad8c;
            }
            0x801fad8c => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad90;
            }
            0x801fad90 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fad94;
            }
            0x801fad94 => { ins += 1;
                c.lr = 0x801fad98; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc0c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fad98;
            }
            0x801fad98 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fad9c;
            }
            0x801fad9c => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fada0;
            }
            0x801fada0 => { ins += 1;
                c.lr = 0x801fada4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc028(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fada4;
            }
            0x801fada4 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fada8;
            }
            0x801fada8 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fadac;
            }
            0x801fadac => { ins += 1;
                c.lr = 0x801fadb0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc018(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fadb0;
            }
            0x801fadb0 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fadb4;
            }
            0x801fadb4 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fadb8;
            }
            0x801fadb8 => { ins += 1;
                c.lr = 0x801fadbc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc008(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fadbc;
            }
            0x801fadbc => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fadc0;
            }
            0x801fadc0 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fadc4;
            }
            0x801fadc4 => { ins += 1;
                c.lr = 0x801fadc8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbff8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fadc8;
            }
            0x801fadc8 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fadcc;
            }
            0x801fadcc => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fadd0;
            }
            0x801fadd0 => { ins += 1;
                c.lr = 0x801fadd4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbfe8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fadd4;
            }
            0x801fadd4 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fadd8;
            }
            0x801fadd8 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x801faddc;
            }
            0x801faddc => { ins += 1;
                c.r[5] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fade0;
            }
            0x801fade0 => { ins += 1;
                c.lr = 0x801fade4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc194(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fade4;
            }
            0x801fade4 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fade8;
            }
            0x801fade8 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x801fadec;
            }
            0x801fadec => { ins += 1;
                c.r[5] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fadf0;
            }
            0x801fadf0 => { ins += 1;
                c.lr = 0x801fadf4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc150(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fadf4;
            }
            0x801fadf4 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fadf8;
            }
            0x801fadf8 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x801fadfc;
            }
            0x801fadfc => { ins += 1;
                c.r[5] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fae00;
            }
            0x801fae00 => { ins += 1;
                c.lr = 0x801fae04; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc07c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae04;
            }
            0x801fae04 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae08;
            }
            0x801fae08 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x801fae0c;
            }
            0x801fae0c => { ins += 1;
                c.r[5] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fae10;
            }
            0x801fae10 => { ins += 1;
                c.lr = 0x801fae14; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc038(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae14;
            }
            0x801fae14 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae18;
            }
            0x801fae18 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fae1c;
            }
            0x801fae1c => { ins += 1;
                c.lr = 0x801fae20; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbf4c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae20;
            }
            0x801fae20 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae24;
            }
            0x801fae24 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fae28;
            }
            0x801fae28 => { ins += 1;
                c.lr = 0x801fae2c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbf3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae2c;
            }
            0x801fae2c => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae30;
            }
            0x801fae30 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fae34;
            }
            0x801fae34 => { ins += 1;
                c.lr = 0x801fae38; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbf2c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae38;
            }
            0x801fae38 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae3c;
            }
            0x801fae3c => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fae40;
            }
            0x801fae40 => { ins += 1;
                c.lr = 0x801fae44; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbf1c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae44;
            }
            0x801fae44 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae48;
            }
            0x801fae48 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801fae4c;
            }
            0x801fae4c => { ins += 1;
                c.lr = 0x801fae50; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcd48(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae50;
            }
            0x801fae50 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae54;
            }
            0x801fae54 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fae58;
            }
            0x801fae58 => { ins += 1;
                c.lr = 0x801fae5c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcd38(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae5c;
            }
            0x801fae5c => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae60;
            }
            0x801fae60 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fae64;
            }
            0x801fae64 => { ins += 1;
                c.lr = 0x801fae68; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcd28(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae68;
            }
            0x801fae68 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae6c;
            }
            0x801fae6c => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fae70;
            }
            0x801fae70 => { ins += 1;
                c.lr = 0x801fae74; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcd18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae74;
            }
            0x801fae74 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae78;
            }
            0x801fae78 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fae7c;
            }
            0x801fae7c => { ins += 1;
                c.lr = 0x801fae80; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcd08(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae80;
            }
            0x801fae80 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae84;
            }
            0x801fae84 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801fae88;
            }
            0x801fae88 => { ins += 1;
                c.lr = 0x801fae8c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcd8c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fae8c;
            }
            0x801fae8c => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fae90;
            }
            0x801fae90 => { ins += 1;
                c.r[30] = c.r[28] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[30]);
                pc = 0x801fae94;
            }
            0x801fae94 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801faea0; continue; }
                pc = 0x801fae98;
            }
            0x801fae98 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fae9c;
            }
            0x801fae9c => { ins += 1;
                pc = 0x801faecc; continue;
                pc = 0x801faea0;
            }
            0x801faea0 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801faea4;
            }
            0x801faea4 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801faea8;
            }
            0x801faea8 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801faeac;
            }
            0x801faeac => { ins += 1;
                c.lr = 0x801faeb0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faeb0;
            }
            0x801faeb0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faeb4;
            }
            0x801faeb4 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801faec0; continue; }
                pc = 0x801faeb8;
            }
            0x801faeb8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801faebc;
            }
            0x801faebc => { ins += 1;
                pc = 0x801faecc; continue;
                pc = 0x801faec0;
            }
            0x801faec0 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x801faec4;
            }
            0x801faec4 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801faec8;
            }
            0x801faec8 => { ins += 1;
                c.lr = 0x801faecc; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faecc;
            }
            0x801faecc => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faed0;
            }
            0x801faed0 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801faed4;
            }
            0x801faed4 => { ins += 1;
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                pc = 0x801faed8;
            }
            0x801faed8 => { ins += 1;
                c.r[5] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801faedc;
            }
            0x801faedc => { ins += 1;
                c.lr = 0x801faee0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbdf4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faee0;
            }
            0x801faee0 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801faee4;
            }
            0x801faee4 => { ins += 1;
                c.r[30] = c.r[28] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[30]);
                pc = 0x801faee8;
            }
            0x801faee8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801faef4; continue; }
                pc = 0x801faeec;
            }
            0x801faeec => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801faef0;
            }
            0x801faef0 => { ins += 1;
                pc = 0x801faf20; continue;
                pc = 0x801faef4;
            }
            0x801faef4 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801faef8;
            }
            0x801faef8 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801faefc;
            }
            0x801faefc => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801faf00;
            }
            0x801faf00 => { ins += 1;
                c.lr = 0x801faf04; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faf04;
            }
            0x801faf04 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faf08;
            }
            0x801faf08 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801faf14; continue; }
                pc = 0x801faf0c;
            }
            0x801faf0c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801faf10;
            }
            0x801faf10 => { ins += 1;
                pc = 0x801faf20; continue;
                pc = 0x801faf14;
            }
            0x801faf14 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x801faf18;
            }
            0x801faf18 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801faf1c;
            }
            0x801faf1c => { ins += 1;
                c.lr = 0x801faf20; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faf20;
            }
            0x801faf20 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faf24;
            }
            0x801faf24 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801faf28;
            }
            0x801faf28 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801faf2c;
            }
            0x801faf2c => { ins += 1;
                c.lr = 0x801faf30; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbde4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faf30;
            }
            0x801faf30 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801faf34;
            }
            0x801faf34 => { ins += 1;
                c.r[30] = c.r[28] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[30]);
                pc = 0x801faf38;
            }
            0x801faf38 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801faf44; continue; }
                pc = 0x801faf3c;
            }
            0x801faf3c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801faf40;
            }
            0x801faf40 => { ins += 1;
                pc = 0x801faf70; continue;
                pc = 0x801faf44;
            }
            0x801faf44 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801faf48;
            }
            0x801faf48 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801faf4c;
            }
            0x801faf4c => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801faf50;
            }
            0x801faf50 => { ins += 1;
                c.lr = 0x801faf54; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faf54;
            }
            0x801faf54 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faf58;
            }
            0x801faf58 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801faf64; continue; }
                pc = 0x801faf5c;
            }
            0x801faf5c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801faf60;
            }
            0x801faf60 => { ins += 1;
                pc = 0x801faf70; continue;
                pc = 0x801faf64;
            }
            0x801faf64 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x801faf68;
            }
            0x801faf68 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801faf6c;
            }
            0x801faf6c => { ins += 1;
                c.lr = 0x801faf70; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faf70;
            }
            0x801faf70 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faf74;
            }
            0x801faf74 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801faf78;
            }
            0x801faf78 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801faf7c;
            }
            0x801faf7c => { ins += 1;
                c.lr = 0x801faf80; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbdd4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faf80;
            }
            0x801faf80 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801faf84;
            }
            0x801faf84 => { ins += 1;
                c.r[30] = c.r[28] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[30]);
                pc = 0x801faf88;
            }
            0x801faf88 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801faf94; continue; }
                pc = 0x801faf8c;
            }
            0x801faf8c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801faf90;
            }
            0x801faf90 => { ins += 1;
                pc = 0x801fafc0; continue;
                pc = 0x801faf94;
            }
            0x801faf94 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801faf98;
            }
            0x801faf98 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801faf9c;
            }
            0x801faf9c => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fafa0;
            }
            0x801fafa0 => { ins += 1;
                c.lr = 0x801fafa4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fafa4;
            }
            0x801fafa4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fafa8;
            }
            0x801fafa8 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fafb4; continue; }
                pc = 0x801fafac;
            }
            0x801fafac => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fafb0;
            }
            0x801fafb0 => { ins += 1;
                pc = 0x801fafc0; continue;
                pc = 0x801fafb4;
            }
            0x801fafb4 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x801fafb8;
            }
            0x801fafb8 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fafbc;
            }
            0x801fafbc => { ins += 1;
                c.lr = 0x801fafc0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fafc0;
            }
            0x801fafc0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fafc4;
            }
            0x801fafc4 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801fafc8;
            }
            0x801fafc8 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fafcc;
            }
            0x801fafcc => { ins += 1;
                c.lr = 0x801fafd0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbdc4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fafd0;
            }
            0x801fafd0 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fafd4;
            }
            0x801fafd4 => { ins += 1;
                c.r[30] = c.r[28] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[30]);
                pc = 0x801fafd8;
            }
            0x801fafd8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fafe4; continue; }
                pc = 0x801fafdc;
            }
            0x801fafdc => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fafe0;
            }
            0x801fafe0 => { ins += 1;
                pc = 0x801fb010; continue;
                pc = 0x801fafe4;
            }
            0x801fafe4 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fafe8;
            }
            0x801fafe8 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fafec;
            }
            0x801fafec => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801faff0;
            }
            0x801faff0 => { ins += 1;
                c.lr = 0x801faff4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801faff4;
            }
            0x801faff4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801faff8;
            }
            0x801faff8 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb004; continue; }
                pc = 0x801faffc;
            }
            0x801faffc => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb000;
            }
            0x801fb000 => { ins += 1;
                pc = 0x801fb010; continue;
                pc = 0x801fb004;
            }
            0x801fb004 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x801fb008;
            }
            0x801fb008 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb00c;
            }
            0x801fb00c => { ins += 1;
                c.lr = 0x801fb010; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb010;
            }
            0x801fb010 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb014;
            }
            0x801fb014 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801fb018;
            }
            0x801fb018 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fb01c;
            }
            0x801fb01c => { ins += 1;
                c.lr = 0x801fb020; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbdb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb020;
            }
            0x801fb020 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fb024;
            }
            0x801fb024 => { ins += 1;
                c.r[30] = c.r[28] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[30]);
                pc = 0x801fb028;
            }
            0x801fb028 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb034; continue; }
                pc = 0x801fb02c;
            }
            0x801fb02c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb030;
            }
            0x801fb030 => { ins += 1;
                pc = 0x801fb060; continue;
                pc = 0x801fb034;
            }
            0x801fb034 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb038;
            }
            0x801fb038 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb03c;
            }
            0x801fb03c => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb040;
            }
            0x801fb040 => { ins += 1;
                c.lr = 0x801fb044; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb044;
            }
            0x801fb044 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb048;
            }
            0x801fb048 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb054; continue; }
                pc = 0x801fb04c;
            }
            0x801fb04c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb050;
            }
            0x801fb050 => { ins += 1;
                pc = 0x801fb060; continue;
                pc = 0x801fb054;
            }
            0x801fb054 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x801fb058;
            }
            0x801fb058 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb05c;
            }
            0x801fb05c => { ins += 1;
                c.lr = 0x801fb060; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb060;
            }
            0x801fb060 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb064;
            }
            0x801fb064 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801fb068;
            }
            0x801fb068 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fb06c;
            }
            0x801fb06c => { ins += 1;
                c.lr = 0x801fb070; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbda4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb070;
            }
            0x801fb070 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fb074;
            }
            0x801fb074 => { ins += 1;
                c.r[30] = c.r[28] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[30]);
                pc = 0x801fb078;
            }
            0x801fb078 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb084; continue; }
                pc = 0x801fb07c;
            }
            0x801fb07c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb080;
            }
            0x801fb080 => { ins += 1;
                pc = 0x801fb0b0; continue;
                pc = 0x801fb084;
            }
            0x801fb084 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb088;
            }
            0x801fb088 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb08c;
            }
            0x801fb08c => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb090;
            }
            0x801fb090 => { ins += 1;
                c.lr = 0x801fb094; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb094;
            }
            0x801fb094 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb098;
            }
            0x801fb098 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb0a4; continue; }
                pc = 0x801fb09c;
            }
            0x801fb09c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb0a0;
            }
            0x801fb0a0 => { ins += 1;
                pc = 0x801fb0b0; continue;
                pc = 0x801fb0a4;
            }
            0x801fb0a4 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x801fb0a8;
            }
            0x801fb0a8 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb0ac;
            }
            0x801fb0ac => { ins += 1;
                c.lr = 0x801fb0b0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb0b0;
            }
            0x801fb0b0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb0b4;
            }
            0x801fb0b4 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801fb0b8;
            }
            0x801fb0b8 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fb0bc;
            }
            0x801fb0bc => { ins += 1;
                c.lr = 0x801fb0c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbd94(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb0c0;
            }
            0x801fb0c0 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fb0c4;
            }
            0x801fb0c4 => { ins += 1;
                c.r[30] = c.r[28] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[30]);
                pc = 0x801fb0c8;
            }
            0x801fb0c8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb0d4; continue; }
                pc = 0x801fb0cc;
            }
            0x801fb0cc => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb0d0;
            }
            0x801fb0d0 => { ins += 1;
                pc = 0x801fb100; continue;
                pc = 0x801fb0d4;
            }
            0x801fb0d4 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb0d8;
            }
            0x801fb0d8 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb0dc;
            }
            0x801fb0dc => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb0e0;
            }
            0x801fb0e0 => { ins += 1;
                c.lr = 0x801fb0e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb0e4;
            }
            0x801fb0e4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb0e8;
            }
            0x801fb0e8 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb0f4; continue; }
                pc = 0x801fb0ec;
            }
            0x801fb0ec => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb0f0;
            }
            0x801fb0f0 => { ins += 1;
                pc = 0x801fb100; continue;
                pc = 0x801fb0f4;
            }
            0x801fb0f4 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x801fb0f8;
            }
            0x801fb0f8 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb0fc;
            }
            0x801fb0fc => { ins += 1;
                c.lr = 0x801fb100; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb100;
            }
            0x801fb100 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb104;
            }
            0x801fb104 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801fb108;
            }
            0x801fb108 => { ins += 1;
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x801fb10c;
            }
            0x801fb10c => { ins += 1;
                c.lr = 0x801fb110; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbd84(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb110;
            }
            0x801fb110 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fb114;
            }
            0x801fb114 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb118;
            }
            0x801fb118 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb11c;
            }
            0x801fb11c => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb120;
            }
            0x801fb120 => { ins += 1;
                c.lr = 0x801fb124; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb124;
            }
            0x801fb124 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb128;
            }
            0x801fb128 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801fb12c;
            }
            0x801fb12c => { ins += 1;
                c.r[5] = c.r[31] as u16 as i16 as i32 as u32;
                pc = 0x801fb130;
            }
            0x801fb130 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb134;
            }
            0x801fb134 => { ins += 1;
                c.lr = 0x801fb138; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020e95c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb138;
            }
            0x801fb138 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fb13c;
            }
            0x801fb13c => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb140;
            }
            0x801fb140 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb144;
            }
            0x801fb144 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb148;
            }
            0x801fb148 => { ins += 1;
                c.lr = 0x801fb14c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb14c;
            }
            0x801fb14c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb150;
            }
            0x801fb150 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801fb154;
            }
            0x801fb154 => { ins += 1;
                c.r[5] = c.r[31] as u16 as i16 as i32 as u32;
                pc = 0x801fb158;
            }
            0x801fb158 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb15c;
            }
            0x801fb15c => { ins += 1;
                c.lr = 0x801fb160; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020e7ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb160;
            }
            0x801fb160 => { ins += 1;
                pc = 0x801fb1ac; continue;
                pc = 0x801fb164;
            }
            0x801fb164 => { ins += 1;
                c.r[30] = c.r[28] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[30]);
                pc = 0x801fb168;
            }
            0x801fb168 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb174; continue; }
                pc = 0x801fb16c;
            }
            0x801fb16c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb170;
            }
            0x801fb170 => { ins += 1;
                pc = 0x801fb1a0; continue;
                pc = 0x801fb174;
            }
            0x801fb174 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb178;
            }
            0x801fb178 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb17c;
            }
            0x801fb17c => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb180;
            }
            0x801fb180 => { ins += 1;
                c.lr = 0x801fb184; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb184;
            }
            0x801fb184 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb188;
            }
            0x801fb188 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb194; continue; }
                pc = 0x801fb18c;
            }
            0x801fb18c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb190;
            }
            0x801fb190 => { ins += 1;
                pc = 0x801fb1a0; continue;
                pc = 0x801fb194;
            }
            0x801fb194 => { ins += 1;
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x801fb198;
            }
            0x801fb198 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb19c;
            }
            0x801fb19c => { ins += 1;
                c.lr = 0x801fb1a0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb1a0;
            }
            0x801fb1a0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb1a4;
            }
            0x801fb1a4 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ac; continue; }
                pc = 0x801fb1a8;
            }
            0x801fb1a8 => { ins += 1;
                c.lr = 0x801fb1ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020edbc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb1ac;
            }
            0x801fb1ac => { ins += 1;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                pc = 0x801fb1b0;
            }
            0x801fb1b0 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                pc = 0x801fb1b4;
            }
            0x801fb1b4 => { ins += 1;
                c.mtspr(8, c.r[0]);
                pc = 0x801fb1b8;
            }
            0x801fb1b8 => { ins += 1;
                c.r[1] = c.r[1].wrapping_add(0x20);
                pc = 0x801fb1bc;
            }
            0x801fb1bc => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801faa58: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetStatus (0x801fb1c0, 1848 bytes)
#[inline(never)]
pub fn f_801fb1c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fb1c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fb1c0); p = true; }
    }
    f_801fb1c0_b(c, 0x801fb1c0);
    if p { crate::prof::exit(c, 0x801fb1c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fb1c0_at(c: &mut Cpu, pc: u32) { f_801fb1c0_b(c, pc) }
#[inline(never)]
fn f_801fb1c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fb1c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fb1c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fb1c0 => { ins += 1;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x801fb1c4;
            }
            0x801fb1c4 => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x801fb1c8;
            }
            0x801fb1c8 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x801fb1cc;
            }
            0x801fb1cc => { ins += 1;
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x801fb1d0;
            }
            0x801fb1d0 => { ins += 1;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x801fb1d4;
            }
            0x801fb1d4 => { ins += 1;
                c.r[31] = c.r[5] | c.r[5];
                pc = 0x801fb1d8;
            }
            0x801fb1d8 => { ins += 1;
                c.r[29] = c.r[4] | c.r[4];
                pc = 0x801fb1dc;
            }
            0x801fb1dc => { ins += 1;
                c.r[30] = c.r[6] | c.r[6];
                pc = 0x801fb1e0;
            }
            0x801fb1e0 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x801fb1ec; continue; }
                pc = 0x801fb1e4;
            }
            0x801fb1e4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5b);
                pc = 0x801fb1e8;
            }
            0x801fb1e8 => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x801fb1f4; continue; }
                pc = 0x801fb1ec;
            }
            0x801fb1ec => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb1f0;
            }
            0x801fb1f0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb1f4;
            }
            0x801fb1f4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xa);
                pc = 0x801fb1f8;
            }
            0x801fb1f8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb214; continue; }
                pc = 0x801fb1fc;
            }
            0x801fb1fc => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x801fb200;
            }
            0x801fb200 => { ins += 1;
                c.lr = 0x801fb204; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fccc4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb204;
            }
            0x801fb204 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb208;
            }
            0x801fb208 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb294; continue; }
                pc = 0x801fb20c;
            }
            0x801fb20c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb210;
            }
            0x801fb210 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb214;
            }
            0x801fb214 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xd);
                pc = 0x801fb218;
            }
            0x801fb218 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb234; continue; }
                pc = 0x801fb21c;
            }
            0x801fb21c => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x801fb220;
            }
            0x801fb220 => { ins += 1;
                c.lr = 0x801fb224; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcad0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb224;
            }
            0x801fb224 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb228;
            }
            0x801fb228 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb294; continue; }
                pc = 0x801fb22c;
            }
            0x801fb22c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb230;
            }
            0x801fb230 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb234;
            }
            0x801fb234 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1e);
                pc = 0x801fb238;
            }
            0x801fb238 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb254; continue; }
                pc = 0x801fb23c;
            }
            0x801fb23c => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x801fb240;
            }
            0x801fb240 => { ins += 1;
                c.lr = 0x801fb244; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fca2c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb244;
            }
            0x801fb244 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb248;
            }
            0x801fb248 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb294; continue; }
                pc = 0x801fb24c;
            }
            0x801fb24c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb250;
            }
            0x801fb250 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb254;
            }
            0x801fb254 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3d);
                pc = 0x801fb258;
            }
            0x801fb258 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb274; continue; }
                pc = 0x801fb25c;
            }
            0x801fb25c => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x801fb260;
            }
            0x801fb260 => { ins += 1;
                c.lr = 0x801fb264; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc658(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb264;
            }
            0x801fb264 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb268;
            }
            0x801fb268 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb294; continue; }
                pc = 0x801fb26c;
            }
            0x801fb26c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb270;
            }
            0x801fb270 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb274;
            }
            0x801fb274 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x42);
                pc = 0x801fb278;
            }
            0x801fb278 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb294; continue; }
                pc = 0x801fb27c;
            }
            0x801fb27c => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x801fb280;
            }
            0x801fb280 => { ins += 1;
                c.lr = 0x801fb284; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbfbc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb284;
            }
            0x801fb284 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb288;
            }
            0x801fb288 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb294; continue; }
                pc = 0x801fb28c;
            }
            0x801fb28c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb290;
            }
            0x801fb290 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb294;
            }
            0x801fb294 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb298;
            }
            0x801fb298 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb2a4; continue; }
                pc = 0x801fb29c;
            }
            0x801fb29c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb2a0;
            }
            0x801fb2a0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb2a4;
            }
            0x801fb2a4 => { ins += 1;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x801fb2a8;
            }
            0x801fb2a8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x56);
                pc = 0x801fb2ac;
            }
            0x801fb2ac => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x801fb8e0; continue; }
                pc = 0x801fb2b0;
            }
            0x801fb2b0 => { ins += 1;
                c.r[4] = 0x80370000;
                pc = 0x801fb2b4;
            }
            0x801fb2b4 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x801fb2b8;
            }
            0x801fb2b8 => { ins += 1;
                c.r[4] = c.r[4].wrapping_add(0x57d8);
                pc = 0x801fb2bc;
            }
            0x801fb2bc => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(c.r[0]));
                pc = 0x801fb2c0;
            }
            0x801fb2c0 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x801fb2c4;
            }
            0x801fb2c4 => { ins += 1;
                if true { let t = c.ctr; if (0x801fb1c0..0x801fb8f8).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fb2c8;
            }
            0x801fb2c8 => { ins += 1;
                c.lr = 0x801fb2cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fccac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb2cc;
            }
            0x801fb2cc => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb2d0;
            }
            0x801fb2d0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb2d4;
            }
            0x801fb2d4 => { ins += 1;
                c.lr = 0x801fb2d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcc94(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb2d8;
            }
            0x801fb2d8 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801fb2dc;
            }
            0x801fb2dc => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb2e0;
            }
            0x801fb2e0 => { ins += 1;
                c.lr = 0x801fb2e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcc7c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb2e4;
            }
            0x801fb2e4 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb2e8;
            }
            0x801fb2e8 => { ins += 1;
                c.lr = 0x801fb2ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcc64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb2ec;
            }
            0x801fb2ec => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801fb2f0;
            }
            0x801fb2f0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb2f4;
            }
            0x801fb2f4 => { ins += 1;
                c.lr = 0x801fb2f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcc3c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb2f8;
            }
            0x801fb2f8 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801fb2fc;
            }
            0x801fb2fc => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb300;
            }
            0x801fb300 => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801fb304;
            }
            0x801fb304 => { ins += 1;
                c.lr = 0x801fb308; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcc08(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb308;
            }
            0x801fb308 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801fb30c;
            }
            0x801fb30c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb310;
            }
            0x801fb310 => { ins += 1;
                c.lr = 0x801fb314; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcbf0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb314;
            }
            0x801fb314 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb318;
            }
            0x801fb318 => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801fb31c;
            }
            0x801fb31c => { ins += 1;
                c.lr = 0x801fb320; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcbbc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb320;
            }
            0x801fb320 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb324;
            }
            0x801fb324 => { ins += 1;
                c.lr = 0x801fb328; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcba4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb328;
            }
            0x801fb328 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb32c;
            }
            0x801fb32c => { ins += 1;
                c.lr = 0x801fb330; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcab8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb330;
            }
            0x801fb330 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb334;
            }
            0x801fb334 => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801fb338;
            }
            0x801fb338 => { ins += 1;
                c.lr = 0x801fb33c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fca88(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb33c;
            }
            0x801fb33c => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb340;
            }
            0x801fb340 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb344;
            }
            0x801fb344 => { ins += 1;
                c.lr = 0x801fb348; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fca14(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb348;
            }
            0x801fb348 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb34c;
            }
            0x801fb34c => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801fb350;
            }
            0x801fb350 => { ins += 1;
                c.lr = 0x801fb354; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc9e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb354;
            }
            0x801fb354 => { ins += 1;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x801fb358;
            }
            0x801fb358 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb35c;
            }
            0x801fb35c => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801fb360;
            }
            0x801fb360 => { ins += 1;
                c.lr = 0x801fb364; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc9ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb364;
            }
            0x801fb364 => { ins += 1;
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x801fb368;
            }
            0x801fb368 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb36c;
            }
            0x801fb36c => { ins += 1;
                c.lr = 0x801fb370; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc994(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb370;
            }
            0x801fb370 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb374;
            }
            0x801fb374 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb378;
            }
            0x801fb378 => { ins += 1;
                c.lr = 0x801fb37c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc97c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb37c;
            }
            0x801fb37c => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801fb380;
            }
            0x801fb380 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb384;
            }
            0x801fb384 => { ins += 1;
                c.lr = 0x801fb388; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc964(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb388;
            }
            0x801fb388 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb38c;
            }
            0x801fb38c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb390;
            }
            0x801fb390 => { ins += 1;
                c.lr = 0x801fb394; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc948(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb394;
            }
            0x801fb394 => { ins += 1;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x801fb398;
            }
            0x801fb398 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb39c;
            }
            0x801fb39c => { ins += 1;
                c.lr = 0x801fb3a0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc930(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb3a0;
            }
            0x801fb3a0 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801fb3a4;
            }
            0x801fb3a4 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb3a8;
            }
            0x801fb3a8 => { ins += 1;
                c.lr = 0x801fb3ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc918(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb3ac;
            }
            0x801fb3ac => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb3b0;
            }
            0x801fb3b0 => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801fb3b4;
            }
            0x801fb3b4 => { ins += 1;
                c.lr = 0x801fb3b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc8d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb3b8;
            }
            0x801fb3b8 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb3bc;
            }
            0x801fb3bc => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801fb3c0;
            }
            0x801fb3c0 => { ins += 1;
                c.lr = 0x801fb3c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc888(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb3c4;
            }
            0x801fb3c4 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb3c8;
            }
            0x801fb3c8 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb3cc;
            }
            0x801fb3cc => { ins += 1;
                c.lr = 0x801fb3d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc870(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb3d0;
            }
            0x801fb3d0 => { ins += 1;
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x801fb3d4;
            }
            0x801fb3d4 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb3d8;
            }
            0x801fb3d8 => { ins += 1;
                c.lr = 0x801fb3dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc854(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb3dc;
            }
            0x801fb3dc => { ins += 1;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x801fb3e0;
            }
            0x801fb3e0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb3e4;
            }
            0x801fb3e4 => { ins += 1;
                c.lr = 0x801fb3e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc838(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb3e8;
            }
            0x801fb3e8 => { ins += 1;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x801fb3ec;
            }
            0x801fb3ec => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb3f0;
            }
            0x801fb3f0 => { ins += 1;
                c.lr = 0x801fb3f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc6bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb3f4;
            }
            0x801fb3f4 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb3f8;
            }
            0x801fb3f8 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb3fc;
            }
            0x801fb3fc => { ins += 1;
                c.lr = 0x801fb400; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc6a4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb400;
            }
            0x801fb400 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb404;
            }
            0x801fb404 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb408;
            }
            0x801fb408 => { ins += 1;
                c.lr = 0x801fb40c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc640(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb40c;
            }
            0x801fb40c => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb410;
            }
            0x801fb410 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb414;
            }
            0x801fb414 => { ins += 1;
                c.lr = 0x801fb418; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc628(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb418;
            }
            0x801fb418 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb41c;
            }
            0x801fb41c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb420;
            }
            0x801fb420 => { ins += 1;
                c.lr = 0x801fb424; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc610(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb424;
            }
            0x801fb424 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb428;
            }
            0x801fb428 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb42c;
            }
            0x801fb42c => { ins += 1;
                c.lr = 0x801fb430; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc5f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb430;
            }
            0x801fb430 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb434;
            }
            0x801fb434 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb438;
            }
            0x801fb438 => { ins += 1;
                c.lr = 0x801fb43c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc5e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb43c;
            }
            0x801fb43c => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb440;
            }
            0x801fb440 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb444;
            }
            0x801fb444 => { ins += 1;
                c.lr = 0x801fb448; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc5c8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb448;
            }
            0x801fb448 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb44c;
            }
            0x801fb44c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb450;
            }
            0x801fb450 => { ins += 1;
                c.lr = 0x801fb454; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc5b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb454;
            }
            0x801fb454 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb458;
            }
            0x801fb458 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb45c;
            }
            0x801fb45c => { ins += 1;
                c.lr = 0x801fb460; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc598(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb460;
            }
            0x801fb460 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb464;
            }
            0x801fb464 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb468;
            }
            0x801fb468 => { ins += 1;
                c.lr = 0x801fb46c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc580(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb46c;
            }
            0x801fb46c => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb470;
            }
            0x801fb470 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb474;
            }
            0x801fb474 => { ins += 1;
                c.lr = 0x801fb478; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc568(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb478;
            }
            0x801fb478 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb47c;
            }
            0x801fb47c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb480;
            }
            0x801fb480 => { ins += 1;
                c.lr = 0x801fb484; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc550(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb484;
            }
            0x801fb484 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb488;
            }
            0x801fb488 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb48c;
            }
            0x801fb48c => { ins += 1;
                c.lr = 0x801fb490; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc538(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb490;
            }
            0x801fb490 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb494;
            }
            0x801fb494 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb498;
            }
            0x801fb498 => { ins += 1;
                c.lr = 0x801fb49c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc478(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb49c;
            }
            0x801fb49c => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb4a0;
            }
            0x801fb4a0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb4a4;
            }
            0x801fb4a4 => { ins += 1;
                c.lr = 0x801fb4a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc460(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb4a8;
            }
            0x801fb4a8 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb4ac;
            }
            0x801fb4ac => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb4b0;
            }
            0x801fb4b0 => { ins += 1;
                c.lr = 0x801fb4b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc448(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb4b4;
            }
            0x801fb4b4 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb4b8;
            }
            0x801fb4b8 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb4bc;
            }
            0x801fb4bc => { ins += 1;
                c.lr = 0x801fb4c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc430(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb4c0;
            }
            0x801fb4c0 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb4c4;
            }
            0x801fb4c4 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb4c8;
            }
            0x801fb4c8 => { ins += 1;
                c.lr = 0x801fb4cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc418(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb4cc;
            }
            0x801fb4cc => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb4d0;
            }
            0x801fb4d0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb4d4;
            }
            0x801fb4d4 => { ins += 1;
                c.lr = 0x801fb4d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc400(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb4d8;
            }
            0x801fb4d8 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb4dc;
            }
            0x801fb4dc => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb4e0;
            }
            0x801fb4e0 => { ins += 1;
                c.lr = 0x801fb4e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc3e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb4e4;
            }
            0x801fb4e4 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb4e8;
            }
            0x801fb4e8 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb4ec;
            }
            0x801fb4ec => { ins += 1;
                c.lr = 0x801fb4f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc3d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb4f0;
            }
            0x801fb4f0 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb4f4;
            }
            0x801fb4f4 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb4f8;
            }
            0x801fb4f8 => { ins += 1;
                c.lr = 0x801fb4fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc3b8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb4fc;
            }
            0x801fb4fc => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb500;
            }
            0x801fb500 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb504;
            }
            0x801fb504 => { ins += 1;
                c.lr = 0x801fb508; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc2f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb508;
            }
            0x801fb508 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb50c;
            }
            0x801fb50c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb510;
            }
            0x801fb510 => { ins += 1;
                c.lr = 0x801fb514; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc2e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb514;
            }
            0x801fb514 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb518;
            }
            0x801fb518 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb51c;
            }
            0x801fb51c => { ins += 1;
                c.lr = 0x801fb520; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc2c8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb520;
            }
            0x801fb520 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb524;
            }
            0x801fb524 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb528;
            }
            0x801fb528 => { ins += 1;
                c.lr = 0x801fb52c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc2b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb52c;
            }
            0x801fb52c => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb530;
            }
            0x801fb530 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb534;
            }
            0x801fb534 => { ins += 1;
                c.lr = 0x801fb538; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc298(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb538;
            }
            0x801fb538 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb53c;
            }
            0x801fb53c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb540;
            }
            0x801fb540 => { ins += 1;
                c.r[4] = c.r[30] | c.r[30];
                pc = 0x801fb544;
            }
            0x801fb544 => { ins += 1;
                c.lr = 0x801fb548; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc4e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb548;
            }
            0x801fb548 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb54c;
            }
            0x801fb54c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb550;
            }
            0x801fb550 => { ins += 1;
                c.r[4] = c.r[30] | c.r[30];
                pc = 0x801fb554;
            }
            0x801fb554 => { ins += 1;
                c.lr = 0x801fb558; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc490(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb558;
            }
            0x801fb558 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb55c;
            }
            0x801fb55c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb560;
            }
            0x801fb560 => { ins += 1;
                c.r[4] = c.r[30] | c.r[30];
                pc = 0x801fb564;
            }
            0x801fb564 => { ins += 1;
                c.lr = 0x801fb568; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc364(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb568;
            }
            0x801fb568 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb56c;
            }
            0x801fb56c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb570;
            }
            0x801fb570 => { ins += 1;
                c.r[4] = c.r[30] | c.r[30];
                pc = 0x801fb574;
            }
            0x801fb574 => { ins += 1;
                c.lr = 0x801fb578; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fc310(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb578;
            }
            0x801fb578 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb57c;
            }
            0x801fb57c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb580;
            }
            0x801fb580 => { ins += 1;
                c.lr = 0x801fb584; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbfa4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb584;
            }
            0x801fb584 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb588;
            }
            0x801fb588 => { ins += 1;
                c.lr = 0x801fb58c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbf8c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb58c;
            }
            0x801fb58c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb590;
            }
            0x801fb590 => { ins += 1;
                c.lr = 0x801fb594; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbf74(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb594;
            }
            0x801fb594 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb598;
            }
            0x801fb598 => { ins += 1;
                c.lr = 0x801fb59c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbf5c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb59c;
            }
            0x801fb59c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb5a0;
            }
            0x801fb5a0 => { ins += 1;
                c.lr = 0x801fb5a4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fceac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb5a4;
            }
            0x801fb5a4 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb5a8;
            }
            0x801fb5a8 => { ins += 1;
                c.lr = 0x801fb5ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fce94(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb5ac;
            }
            0x801fb5ac => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801fb5b0;
            }
            0x801fb5b0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb5b4;
            }
            0x801fb5b4 => { ins += 1;
                c.r[4] = c.r[30] | c.r[30];
                pc = 0x801fb5b8;
            }
            0x801fb5b8 => { ins += 1;
                c.lr = 0x801fb5bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fce60(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb5bc;
            }
            0x801fb5bc => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb5c0;
            }
            0x801fb5c0 => { ins += 1;
                c.r[4] = c.r[30] | c.r[30];
                pc = 0x801fb5c4;
            }
            0x801fb5c4 => { ins += 1;
                c.lr = 0x801fb5c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fce2c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb5c8;
            }
            0x801fb5c8 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb5cc;
            }
            0x801fb5cc => { ins += 1;
                c.lr = 0x801fb5d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fce14(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb5d0;
            }
            0x801fb5d0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb5d4;
            }
            0x801fb5d4 => { ins += 1;
                c.lr = 0x801fb5d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcdfc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb5d8;
            }
            0x801fb5d8 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb5dc;
            }
            0x801fb5dc => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb5e0;
            }
            0x801fb5e0 => { ins += 1;
                c.lr = 0x801fb5e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcde4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb5e4;
            }
            0x801fb5e4 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb5e8;
            }
            0x801fb5e8 => { ins += 1;
                c.lr = 0x801fb5ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcdcc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb5ec;
            }
            0x801fb5ec => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb5f0;
            }
            0x801fb5f0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb5f4;
            }
            0x801fb5f4 => { ins += 1;
                c.lr = 0x801fb5f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcdb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb5f8;
            }
            0x801fb5f8 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb5fc;
            }
            0x801fb5fc => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb600;
            }
            0x801fb600 => { ins += 1;
                c.lr = 0x801fb604; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fccf0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb604;
            }
            0x801fb604 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb608;
            }
            0x801fb608 => { ins += 1;
                c.lr = 0x801fb60c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcd9c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb60c;
            }
            0x801fb60c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb610;
            }
            0x801fb610 => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801fb614;
            }
            0x801fb614 => { ins += 1;
                c.lr = 0x801fb618; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fcd58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb618;
            }
            0x801fb618 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb61c;
            }
            0x801fb61c => { ins += 1;
                c.r[31] = c.r[29] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[31]);
                pc = 0x801fb620;
            }
            0x801fb620 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb62c; continue; }
                pc = 0x801fb624;
            }
            0x801fb624 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb628;
            }
            0x801fb628 => { ins += 1;
                pc = 0x801fb658; continue;
                pc = 0x801fb62c;
            }
            0x801fb62c => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb630;
            }
            0x801fb630 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb634;
            }
            0x801fb634 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb638;
            }
            0x801fb638 => { ins += 1;
                c.lr = 0x801fb63c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb63c;
            }
            0x801fb63c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb640;
            }
            0x801fb640 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb64c; continue; }
                pc = 0x801fb644;
            }
            0x801fb644 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb648;
            }
            0x801fb648 => { ins += 1;
                pc = 0x801fb658; continue;
                pc = 0x801fb64c;
            }
            0x801fb64c => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801fb650;
            }
            0x801fb650 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb654;
            }
            0x801fb654 => { ins += 1;
                c.lr = 0x801fb658; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb658;
            }
            0x801fb658 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb65c;
            }
            0x801fb65c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb668; continue; }
                pc = 0x801fb660;
            }
            0x801fb660 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb664;
            }
            0x801fb664 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb668;
            }
            0x801fb668 => { ins += 1;
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                pc = 0x801fb66c;
            }
            0x801fb66c => { ins += 1;
                c.lr = 0x801fb670; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbed0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb670;
            }
            0x801fb670 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801fb674;
            }
            0x801fb674 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb678;
            }
            0x801fb678 => { ins += 1;
                c.r[31] = c.r[29] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[31]);
                pc = 0x801fb67c;
            }
            0x801fb67c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb688; continue; }
                pc = 0x801fb680;
            }
            0x801fb680 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb684;
            }
            0x801fb684 => { ins += 1;
                pc = 0x801fb6b4; continue;
                pc = 0x801fb688;
            }
            0x801fb688 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb68c;
            }
            0x801fb68c => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb690;
            }
            0x801fb690 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb694;
            }
            0x801fb694 => { ins += 1;
                c.lr = 0x801fb698; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb698;
            }
            0x801fb698 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb69c;
            }
            0x801fb69c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb6a8; continue; }
                pc = 0x801fb6a0;
            }
            0x801fb6a0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb6a4;
            }
            0x801fb6a4 => { ins += 1;
                pc = 0x801fb6b4; continue;
                pc = 0x801fb6a8;
            }
            0x801fb6a8 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801fb6ac;
            }
            0x801fb6ac => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb6b0;
            }
            0x801fb6b0 => { ins += 1;
                c.lr = 0x801fb6b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb6b4;
            }
            0x801fb6b4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb6b8;
            }
            0x801fb6b8 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb6c4; continue; }
                pc = 0x801fb6bc;
            }
            0x801fb6bc => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb6c0;
            }
            0x801fb6c0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb6c4;
            }
            0x801fb6c4 => { ins += 1;
                c.lr = 0x801fb6c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbeb8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb6c8;
            }
            0x801fb6c8 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801fb6cc;
            }
            0x801fb6cc => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb6d0;
            }
            0x801fb6d0 => { ins += 1;
                c.r[31] = c.r[29] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[31]);
                pc = 0x801fb6d4;
            }
            0x801fb6d4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb6e0; continue; }
                pc = 0x801fb6d8;
            }
            0x801fb6d8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb6dc;
            }
            0x801fb6dc => { ins += 1;
                pc = 0x801fb70c; continue;
                pc = 0x801fb6e0;
            }
            0x801fb6e0 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb6e4;
            }
            0x801fb6e4 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb6e8;
            }
            0x801fb6e8 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb6ec;
            }
            0x801fb6ec => { ins += 1;
                c.lr = 0x801fb6f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb6f0;
            }
            0x801fb6f0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb6f4;
            }
            0x801fb6f4 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb700; continue; }
                pc = 0x801fb6f8;
            }
            0x801fb6f8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb6fc;
            }
            0x801fb6fc => { ins += 1;
                pc = 0x801fb70c; continue;
                pc = 0x801fb700;
            }
            0x801fb700 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801fb704;
            }
            0x801fb704 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb708;
            }
            0x801fb708 => { ins += 1;
                c.lr = 0x801fb70c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb70c;
            }
            0x801fb70c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb710;
            }
            0x801fb710 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb71c; continue; }
                pc = 0x801fb714;
            }
            0x801fb714 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb718;
            }
            0x801fb718 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb71c;
            }
            0x801fb71c => { ins += 1;
                c.lr = 0x801fb720; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbea0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb720;
            }
            0x801fb720 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801fb724;
            }
            0x801fb724 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb728;
            }
            0x801fb728 => { ins += 1;
                c.r[31] = c.r[29] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[31]);
                pc = 0x801fb72c;
            }
            0x801fb72c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb738; continue; }
                pc = 0x801fb730;
            }
            0x801fb730 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb734;
            }
            0x801fb734 => { ins += 1;
                pc = 0x801fb764; continue;
                pc = 0x801fb738;
            }
            0x801fb738 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb73c;
            }
            0x801fb73c => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb740;
            }
            0x801fb740 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb744;
            }
            0x801fb744 => { ins += 1;
                c.lr = 0x801fb748; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb748;
            }
            0x801fb748 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb74c;
            }
            0x801fb74c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb758; continue; }
                pc = 0x801fb750;
            }
            0x801fb750 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb754;
            }
            0x801fb754 => { ins += 1;
                pc = 0x801fb764; continue;
                pc = 0x801fb758;
            }
            0x801fb758 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801fb75c;
            }
            0x801fb75c => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb760;
            }
            0x801fb760 => { ins += 1;
                c.lr = 0x801fb764; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb764;
            }
            0x801fb764 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb768;
            }
            0x801fb768 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb774; continue; }
                pc = 0x801fb76c;
            }
            0x801fb76c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb770;
            }
            0x801fb770 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb774;
            }
            0x801fb774 => { ins += 1;
                c.lr = 0x801fb778; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbe88(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb778;
            }
            0x801fb778 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb77c;
            }
            0x801fb77c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb780;
            }
            0x801fb780 => { ins += 1;
                c.r[31] = c.r[29] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[31]);
                pc = 0x801fb784;
            }
            0x801fb784 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb790; continue; }
                pc = 0x801fb788;
            }
            0x801fb788 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb78c;
            }
            0x801fb78c => { ins += 1;
                pc = 0x801fb7bc; continue;
                pc = 0x801fb790;
            }
            0x801fb790 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb794;
            }
            0x801fb794 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb798;
            }
            0x801fb798 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb79c;
            }
            0x801fb79c => { ins += 1;
                c.lr = 0x801fb7a0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb7a0;
            }
            0x801fb7a0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb7a4;
            }
            0x801fb7a4 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb7b0; continue; }
                pc = 0x801fb7a8;
            }
            0x801fb7a8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb7ac;
            }
            0x801fb7ac => { ins += 1;
                pc = 0x801fb7bc; continue;
                pc = 0x801fb7b0;
            }
            0x801fb7b0 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801fb7b4;
            }
            0x801fb7b4 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb7b8;
            }
            0x801fb7b8 => { ins += 1;
                c.lr = 0x801fb7bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb7bc;
            }
            0x801fb7bc => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb7c0;
            }
            0x801fb7c0 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb7cc; continue; }
                pc = 0x801fb7c4;
            }
            0x801fb7c4 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb7c8;
            }
            0x801fb7c8 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb7cc;
            }
            0x801fb7cc => { ins += 1;
                c.lr = 0x801fb7d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbe70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb7d0;
            }
            0x801fb7d0 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb7d4;
            }
            0x801fb7d4 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb7d8;
            }
            0x801fb7d8 => { ins += 1;
                c.r[31] = c.r[29] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[31]);
                pc = 0x801fb7dc;
            }
            0x801fb7dc => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb7e8; continue; }
                pc = 0x801fb7e0;
            }
            0x801fb7e0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb7e4;
            }
            0x801fb7e4 => { ins += 1;
                pc = 0x801fb814; continue;
                pc = 0x801fb7e8;
            }
            0x801fb7e8 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb7ec;
            }
            0x801fb7ec => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb7f0;
            }
            0x801fb7f0 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb7f4;
            }
            0x801fb7f4 => { ins += 1;
                c.lr = 0x801fb7f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb7f8;
            }
            0x801fb7f8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb7fc;
            }
            0x801fb7fc => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb808; continue; }
                pc = 0x801fb800;
            }
            0x801fb800 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb804;
            }
            0x801fb804 => { ins += 1;
                pc = 0x801fb814; continue;
                pc = 0x801fb808;
            }
            0x801fb808 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801fb80c;
            }
            0x801fb80c => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb810;
            }
            0x801fb810 => { ins += 1;
                c.lr = 0x801fb814; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb814;
            }
            0x801fb814 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb818;
            }
            0x801fb818 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb824; continue; }
                pc = 0x801fb81c;
            }
            0x801fb81c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb820;
            }
            0x801fb820 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb824;
            }
            0x801fb824 => { ins += 1;
                c.lr = 0x801fb828; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbe58(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb828;
            }
            0x801fb828 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb82c;
            }
            0x801fb82c => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb830;
            }
            0x801fb830 => { ins += 1;
                c.r[31] = c.r[29] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[31]);
                pc = 0x801fb834;
            }
            0x801fb834 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb840; continue; }
                pc = 0x801fb838;
            }
            0x801fb838 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb83c;
            }
            0x801fb83c => { ins += 1;
                pc = 0x801fb86c; continue;
                pc = 0x801fb840;
            }
            0x801fb840 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb844;
            }
            0x801fb844 => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb848;
            }
            0x801fb848 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb84c;
            }
            0x801fb84c => { ins += 1;
                c.lr = 0x801fb850; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb850;
            }
            0x801fb850 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb854;
            }
            0x801fb854 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb860; continue; }
                pc = 0x801fb858;
            }
            0x801fb858 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb85c;
            }
            0x801fb85c => { ins += 1;
                pc = 0x801fb86c; continue;
                pc = 0x801fb860;
            }
            0x801fb860 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801fb864;
            }
            0x801fb864 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb868;
            }
            0x801fb868 => { ins += 1;
                c.lr = 0x801fb86c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb86c;
            }
            0x801fb86c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb870;
            }
            0x801fb870 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb87c; continue; }
                pc = 0x801fb874;
            }
            0x801fb874 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb878;
            }
            0x801fb878 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb87c;
            }
            0x801fb87c => { ins += 1;
                c.lr = 0x801fb880; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbe40(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb880;
            }
            0x801fb880 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb884;
            }
            0x801fb884 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb888;
            }
            0x801fb888 => { ins += 1;
                c.r[31] = c.r[29] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[31]);
                pc = 0x801fb88c;
            }
            0x801fb88c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x801fb898; continue; }
                pc = 0x801fb890;
            }
            0x801fb890 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb894;
            }
            0x801fb894 => { ins += 1;
                pc = 0x801fb8c4; continue;
                pc = 0x801fb898;
            }
            0x801fb898 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x801fb89c;
            }
            0x801fb89c => { ins += 1;
                c.r[5] = 0x4e;
                pc = 0x801fb8a0;
            }
            0x801fb8a0 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x801fb8a4;
            }
            0x801fb8a4 => { ins += 1;
                c.lr = 0x801fb8a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb8a8;
            }
            0x801fb8a8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb8ac;
            }
            0x801fb8ac => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb8b8; continue; }
                pc = 0x801fb8b0;
            }
            0x801fb8b0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb8b4;
            }
            0x801fb8b4 => { ins += 1;
                pc = 0x801fb8c4; continue;
                pc = 0x801fb8b8;
            }
            0x801fb8b8 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801fb8bc;
            }
            0x801fb8bc => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x801fb8c0;
            }
            0x801fb8c0 => { ins += 1;
                c.lr = 0x801fb8c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8020eba8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb8c4;
            }
            0x801fb8c4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                pc = 0x801fb8c8;
            }
            0x801fb8c8 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fb8d4; continue; }
                pc = 0x801fb8cc;
            }
            0x801fb8cc => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb8d0;
            }
            0x801fb8d0 => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb8d4;
            }
            0x801fb8d4 => { ins += 1;
                c.lr = 0x801fb8d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fbe28(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb8d8;
            }
            0x801fb8d8 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801fb8dc;
            }
            0x801fb8dc => { ins += 1;
                pc = 0x801fb8e4; continue;
                pc = 0x801fb8e0;
            }
            0x801fb8e0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801fb8e4;
            }
            0x801fb8e4 => { ins += 1;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                pc = 0x801fb8e8;
            }
            0x801fb8e8 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                pc = 0x801fb8ec;
            }
            0x801fb8ec => { ins += 1;
                c.mtspr(8, c.r[0]);
                pc = 0x801fb8f0;
            }
            0x801fb8f0 => { ins += 1;
                c.r[1] = c.r[1].wrapping_add(0x20);
                pc = 0x801fb8f4;
            }
            0x801fb8f4 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fb1c0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerIsGcHero (0x801fb8f8, 124 bytes)
#[inline(never)]
pub fn f_801fb8f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fb8f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fb8f8); p = true; }
    }
    f_801fb8f8_b(c, 0x801fb8f8);
    if p { crate::prof::exit(c, 0x801fb8f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fb8f8_at(c: &mut Cpu, pc: u32) { f_801fb8f8_b(c, pc) }
#[inline(always)]
fn f_801fb8f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fb8f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fb8f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fb8f8 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.lr = 0x801fb914; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb914;
            }
            0x801fb914 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801fb928; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb928;
            }
            0x801fb928 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801fb93c; continue; }
                pc = 0x801fb934;
            }
            0x801fb934 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801fb958; continue;
                pc = 0x801fb93c;
            }
            0x801fb93c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x801fb94c; continue; }
                pc = 0x801fb944;
            }
            0x801fb944 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801fb954; continue; }
                pc = 0x801fb94c;
            }
            0x801fb94c => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x801fb958; continue;
                pc = 0x801fb954;
            }
            0x801fb954 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x801fb958;
            }
            0x801fb958 => { ins += 7;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0xff;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fb8f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerWazaSeq57 (0x801fb974, 176 bytes)
#[inline(never)]
pub fn f_801fb974(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fb974) { return; }
        if c.prof { crate::prof::enter(c, 0x801fb974); p = true; }
    }
    f_801fb974_b(c, 0x801fb974);
    if p { crate::prof::exit(c, 0x801fb974); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fb974_at(c: &mut Cpu, pc: u32) { f_801fb974_b(c, pc) }
#[inline(always)]
fn f_801fb974_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fb974;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fb974, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fb974 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.lr = 0x801fb998; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb998;
            }
            0x801fb998 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if cr_bit(cr, 2) { pc = 0x801fba10; continue; }
                pc = 0x801fb9a0;
            }
            0x801fb9a0 => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801fb9bc; continue; }
                pc = 0x801fb9a8;
            }
            0x801fb9a8 => { ins += 4;
                c.r[4] = 0x57;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801fb9b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb9b8;
            }
            0x801fb9b8 => { ins += 1;
                pc = 0x801fba10; continue;
                pc = 0x801fb9bc;
            }
            0x801fb9bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801fb9d4; continue; }
                pc = 0x801fb9c4;
            }
            0x801fb9c4 => { ins += 3;
                c.r[4] = 0x57;
                c.r[5] = 0x4;
                c.lr = 0x801fb9d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb9d0;
            }
            0x801fb9d0 => { ins += 1;
                pc = 0x801fba10; continue;
                pc = 0x801fb9d4;
            }
            0x801fb9d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x801fb9fc; continue; }
                pc = 0x801fb9dc;
            }
            0x801fb9dc => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x57;
                c.r[5] = 0x4;
                c.lr = 0x801fb9ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb9ec;
            }
            0x801fb9ec => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801fba10; continue; }
                pc = 0x801fb9f4;
            }
            0x801fb9f4 => { ins += 1;
                c.lr = 0x801fb9f8; { c.steps = st; c.instrs = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fb9f8;
            }
            0x801fb9f8 => { ins += 1;
                pc = 0x801fb9dc; continue;
                pc = 0x801fb9fc;
            }
            0x801fb9fc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801fba10; continue; }
                pc = 0x801fba04;
            }
            0x801fba04 => { ins += 3;
                c.r[4] = 0x57;
                c.r[5] = 0x4;
                c.lr = 0x801fba10; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fba10;
            }
            0x801fba10 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fb974: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerWazaSeq74 (0x801fba24, 176 bytes)
#[inline(never)]
pub fn f_801fba24(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fba24) { return; }
        if c.prof { crate::prof::enter(c, 0x801fba24); p = true; }
    }
    f_801fba24_b(c, 0x801fba24);
    if p { crate::prof::exit(c, 0x801fba24); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fba24_at(c: &mut Cpu, pc: u32) { f_801fba24_b(c, pc) }
#[inline(always)]
fn f_801fba24_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fba24;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fba24, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fba24 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.lr = 0x801fba48; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fba48;
            }
            0x801fba48 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if cr_bit(cr, 2) { pc = 0x801fbac0; continue; }
                pc = 0x801fba50;
            }
            0x801fba50 => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801fba6c; continue; }
                pc = 0x801fba58;
            }
            0x801fba58 => { ins += 4;
                c.r[4] = 0x74;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801fba68; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fba68;
            }
            0x801fba68 => { ins += 1;
                pc = 0x801fbac0; continue;
                pc = 0x801fba6c;
            }
            0x801fba6c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801fba84; continue; }
                pc = 0x801fba74;
            }
            0x801fba74 => { ins += 3;
                c.r[4] = 0x74;
                c.r[5] = 0x4;
                c.lr = 0x801fba80; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fba80;
            }
            0x801fba80 => { ins += 1;
                pc = 0x801fbac0; continue;
                pc = 0x801fba84;
            }
            0x801fba84 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x801fbaac; continue; }
                pc = 0x801fba8c;
            }
            0x801fba8c => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x74;
                c.r[5] = 0x4;
                c.lr = 0x801fba9c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fba9c;
            }
            0x801fba9c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801fbac0; continue; }
                pc = 0x801fbaa4;
            }
            0x801fbaa4 => { ins += 1;
                c.lr = 0x801fbaa8; { c.steps = st; c.instrs = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbaa8;
            }
            0x801fbaa8 => { ins += 1;
                pc = 0x801fba8c; continue;
                pc = 0x801fbaac;
            }
            0x801fbaac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801fbac0; continue; }
                pc = 0x801fbab4;
            }
            0x801fbab4 => { ins += 3;
                c.r[4] = 0x74;
                c.r[5] = 0x4;
                c.lr = 0x801fbac0; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbac0;
            }
            0x801fbac0 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fba24: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerHokakuThrowEffect (0x801fbad4, 332 bytes)
#[inline(never)]
pub fn f_801fbad4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbad4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbad4); p = true; }
    }
    f_801fbad4_b(c, 0x801fbad4);
    if p { crate::prof::exit(c, 0x801fbad4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbad4_at(c: &mut Cpu, pc: u32) { f_801fbad4_b(c, pc) }
#[inline(always)]
fn f_801fbad4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbad4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbad4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbad4 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = c.r[5] | c.r[5];
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.lr = 0x801fbafc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbafc;
            }
            0x801fbafc => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x801fbc0c; continue; }
                pc = 0x801fbb04;
            }
            0x801fbb04 => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[5] = 0x12;
                c.r[6] = 0x0;
                c.lr = 0x801fbb18; { c.steps = st; c.instrs = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbb18;
            }
            0x801fbb18 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[31] = c.r[3] | c.r[3];
                if !cr_bit(cr, 2) { pc = 0x801fbb3c; continue; }
                pc = 0x801fbb24;
            }
            0x801fbb24 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801fbb38; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbb38;
            }
            0x801fbb38 => { ins += 1;
                pc = 0x801fbc0c; continue;
                pc = 0x801fbb3c;
            }
            0x801fbb3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801fbb74; continue; }
                pc = 0x801fbb44;
            }
            0x801fbb44 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x801fbb54; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbb54;
            }
            0x801fbb54 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x801fbb64; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbb64;
            }
            0x801fbb64 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801fbc0c; continue; }
                pc = 0x801fbb6c;
            }
            0x801fbb6c => { ins += 1;
                c.lr = 0x801fbb70; { c.steps = st; c.instrs = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbb70;
            }
            0x801fbb70 => { ins += 1;
                pc = 0x801fbb54; continue;
                pc = 0x801fbb74;
            }
            0x801fbb74 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x801fbb90; continue; }
                pc = 0x801fbb7c;
            }
            0x801fbb7c => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x801fbb8c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbb8c;
            }
            0x801fbb8c => { ins += 1;
                pc = 0x801fbc0c; continue;
                pc = 0x801fbb90;
            }
            0x801fbb90 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801fbbb0; continue; }
                pc = 0x801fbb98;
            }
            0x801fbb98 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x57;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801fbbac; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbbac;
            }
            0x801fbbac => { ins += 1;
                pc = 0x801fbc0c; continue;
                pc = 0x801fbbb0;
            }
            0x801fbbb0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 2) { pc = 0x801fbbcc; continue; }
                pc = 0x801fbbb8;
            }
            0x801fbbb8 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x57;
                c.r[5] = 0x4;
                c.lr = 0x801fbbc8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbbc8;
            }
            0x801fbbc8 => { ins += 1;
                pc = 0x801fbc0c; continue;
                pc = 0x801fbbcc;
            }
            0x801fbbcc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5);
                if !cr_bit(cr, 2) { pc = 0x801fbbf4; continue; }
                pc = 0x801fbbd4;
            }
            0x801fbbd4 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x57;
                c.r[5] = 0x4;
                c.lr = 0x801fbbe4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbbe4;
            }
            0x801fbbe4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801fbc0c; continue; }
                pc = 0x801fbbec;
            }
            0x801fbbec => { ins += 1;
                c.lr = 0x801fbbf0; { c.steps = st; c.instrs = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbbf0;
            }
            0x801fbbf0 => { ins += 1;
                pc = 0x801fbbd4; continue;
                pc = 0x801fbbf4;
            }
            0x801fbbf4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 2) { pc = 0x801fbc0c; continue; }
                pc = 0x801fbbfc;
            }
            0x801fbbfc => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x57;
                c.r[5] = 0x4;
                c.lr = 0x801fbc0c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbc0c;
            }
            0x801fbc0c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbad4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerBallThrowEffect (0x801fbc20, 240 bytes)
#[inline(never)]
pub fn f_801fbc20(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbc20) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbc20); p = true; }
    }
    f_801fbc20_b(c, 0x801fbc20);
    if p { crate::prof::exit(c, 0x801fbc20); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbc20_at(c: &mut Cpu, pc: u32) { f_801fbc20_b(c, pc) }
#[inline(always)]
fn f_801fbc20_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbc20;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbc20, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbc20 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[31] = c.r[5] | c.r[5];
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.lr = 0x801fbc48; { c.steps = st; c.instrs = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbc48;
            }
            0x801fbc48 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x801fbcfc; continue; }
                pc = 0x801fbc50;
            }
            0x801fbc50 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801fbc58; { c.steps = st; c.instrs = ins; c.cr = cr; f_80205b8c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbc58;
            }
            0x801fbc58 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x73;
                c.r[6] = 0x0;
                c.lr = 0x801fbc68; { c.steps = st; c.instrs = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbc68;
            }
            0x801fbc68 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x801fbc7c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbc7c;
            }
            0x801fbc7c => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[31] = c.r[3] | c.r[3];
                if !cr_bit(cr, 2) { pc = 0x801fbca0; continue; }
                pc = 0x801fbc88;
            }
            0x801fbc88 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x801fbc9c; { c.steps = st; c.instrs = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbc9c;
            }
            0x801fbc9c => { ins += 1;
                pc = 0x801fbcfc; continue;
                pc = 0x801fbca0;
            }
            0x801fbca0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801fbcbc; continue; }
                pc = 0x801fbca8;
            }
            0x801fbca8 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x801fbcb8; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbcb8;
            }
            0x801fbcb8 => { ins += 1;
                pc = 0x801fbcfc; continue;
                pc = 0x801fbcbc;
            }
            0x801fbcbc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x801fbce4; continue; }
                pc = 0x801fbcc4;
            }
            0x801fbcc4 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x801fbcd4; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbcd4;
            }
            0x801fbcd4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801fbcfc; continue; }
                pc = 0x801fbcdc;
            }
            0x801fbcdc => { ins += 1;
                c.lr = 0x801fbce0; { c.steps = st; c.instrs = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbce0;
            }
            0x801fbce0 => { ins += 1;
                pc = 0x801fbcc4; continue;
                pc = 0x801fbce4;
            }
            0x801fbce4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x801fbcfc; continue; }
                pc = 0x801fbcec;
            }
            0x801fbcec => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x801fbcfc; { c.steps = st; c.instrs = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x801fbcfc;
            }
            0x801fbcfc => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbc20: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerKindDataBiosGetBgmSndId (0x801fbd10, 24 bytes)
#[inline(never)]
pub fn f_801fbd10(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbd10) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbd10); p = true; }
    }
    f_801fbd10_b(c, 0x801fbd10);
    if p { crate::prof::exit(c, 0x801fbd10); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbd10_at(c: &mut Cpu, pc: u32) { f_801fbd10_b(c, pc) }
#[inline(always)]
fn f_801fbd10_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbd10;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbd10, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbd10 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbd20; continue; }
                pc = 0x801fbd18;
            }
            0x801fbd18 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbd20;
            }
            0x801fbd20 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbd10: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerKindDataBiosGetPrefixName (0x801fbd28, 24 bytes)
#[inline(never)]
pub fn f_801fbd28(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbd28) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbd28); p = true; }
    }
    f_801fbd28_b(c, 0x801fbd28);
    if p { crate::prof::exit(c, 0x801fbd28); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbd28_at(c: &mut Cpu, pc: u32) { f_801fbd28_b(c, pc) }
#[inline(always)]
fn f_801fbd28_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbd28;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbd28, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbd28 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbd38; continue; }
                pc = 0x801fbd30;
            }
            0x801fbd30 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbd38;
            }
            0x801fbd38 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbd28: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerKindDataBiosGetSyoukinBairitu (0x801fbd40, 24 bytes)
#[inline(never)]
pub fn f_801fbd40(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbd40) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbd40); p = true; }
    }
    f_801fbd40_b(c, 0x801fbd40);
    if p { crate::prof::exit(c, 0x801fbd40); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbd40_at(c: &mut Cpu, pc: u32) { f_801fbd40_b(c, pc) }
#[inline(always)]
fn f_801fbd40_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbd40;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbd40, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbd40 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbd50; continue; }
                pc = 0x801fbd48;
            }
            0x801fbd48 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbd50;
            }
            0x801fbd50 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbd40: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerKindDataBiosGetPtr (0x801fbd58, 44 bytes)
#[inline(never)]
pub fn f_801fbd58(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbd58) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbd58); p = true; }
    }
    f_801fbd58_b(c, 0x801fbd58);
    if p { crate::prof::exit(c, 0x801fbd58); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbd58_at(c: &mut Cpu, pc: u32) { f_801fbd58_b(c, pc) }
#[inline(always)]
fn f_801fbd58_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbd58;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbd58, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbd58 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86f8));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801fbd74; continue; }
                pc = 0x801fbd6c;
            }
            0x801fbd6c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbd74;
            }
            0x801fbd74 => { ins += 4;
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86fc));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbd58: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosSetParam1bantakaiFlag (0x801fbd84, 16 bytes)
#[inline(never)]
pub fn f_801fbd84(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbd84) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbd84); p = true; }
    }
    f_801fbd84_b(c, 0x801fbd84);
    if p { crate::prof::exit(c, 0x801fbd84); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbd84_at(c: &mut Cpu, pc: u32) { f_801fbd84_b(c, pc) }
#[inline(always)]
fn f_801fbd84_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbd84;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbd84, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbd84 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbd8c;
            }
            0x801fbd8c => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x12), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbd84: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosSetBadwazaHaveFlag (0x801fbd94, 16 bytes)
#[inline(never)]
pub fn f_801fbd94(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbd94) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbd94); p = true; }
    }
    f_801fbd94_b(c, 0x801fbd94);
    if p { crate::prof::exit(c, 0x801fbd94); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbd94_at(c: &mut Cpu, pc: u32) { f_801fbd94_b(c, pc) }
#[inline(always)]
fn f_801fbd94_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbd94;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbd94, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbd94 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbd9c;
            }
            0x801fbd9c => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x11), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbd94: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosSetDefense1banhikuiFlag (0x801fbda4, 16 bytes)
#[inline(never)]
pub fn f_801fbda4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbda4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbda4); p = true; }
    }
    f_801fbda4_b(c, 0x801fbda4);
    if p { crate::prof::exit(c, 0x801fbda4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbda4_at(c: &mut Cpu, pc: u32) { f_801fbda4_b(c, pc) }
#[inline(always)]
fn f_801fbda4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbda4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbda4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbda4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbdac;
            }
            0x801fbdac => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbda4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosSetLv1banhikuiFlag (0x801fbdb4, 16 bytes)
#[inline(never)]
pub fn f_801fbdb4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbdb4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbdb4); p = true; }
    }
    f_801fbdb4_b(c, 0x801fbdb4);
    if p { crate::prof::exit(c, 0x801fbdb4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbdb4_at(c: &mut Cpu, pc: u32) { f_801fbdb4_b(c, pc) }
#[inline(always)]
fn f_801fbdb4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbdb4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbdb4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbdb4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbdbc;
            }
            0x801fbdbc => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xf), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbdb4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosSetNowhp1banhikuiFlag (0x801fbdc4, 16 bytes)
#[inline(never)]
pub fn f_801fbdc4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbdc4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbdc4); p = true; }
    }
    f_801fbdc4_b(c, 0x801fbdc4);
    if p { crate::prof::exit(c, 0x801fbdc4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbdc4_at(c: &mut Cpu, pc: u32) { f_801fbdc4_b(c, pc) }
#[inline(always)]
fn f_801fbdc4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbdc4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbdc4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbdc4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbdcc;
            }
            0x801fbdcc => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xe), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbdc4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosSetStoreTokuseiData (0x801fbdd4, 16 bytes)
#[inline(never)]
pub fn f_801fbdd4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbdd4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbdd4); p = true; }
    }
    f_801fbdd4_b(c, 0x801fbdd4);
    if p { crate::prof::exit(c, 0x801fbdd4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbdd4_at(c: &mut Cpu, pc: u32) { f_801fbdd4_b(c, pc) }
#[inline(always)]
fn f_801fbdd4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbdd4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbdd4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbdd4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbddc;
            }
            0x801fbddc => { ins += 2;
                c.sth(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbdd4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosSetTokuseiFlag (0x801fbde4, 16 bytes)
#[inline(never)]
pub fn f_801fbde4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbde4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbde4); p = true; }
    }
    f_801fbde4_b(c, 0x801fbde4);
    if p { crate::prof::exit(c, 0x801fbde4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbde4_at(c: &mut Cpu, pc: u32) { f_801fbde4_b(c, pc) }
#[inline(always)]
fn f_801fbde4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbde4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbde4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbde4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbdec;
            }
            0x801fbdec => { ins += 2;
                c.sth(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbde4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyBiosSetEntry (0x801fbdf4, 36 bytes)
#[inline(never)]
pub fn f_801fbdf4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbdf4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbdf4); p = true; }
    }
    f_801fbdf4_b(c, 0x801fbdf4);
    if p { crate::prof::exit(c, 0x801fbdf4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbdf4_at(c: &mut Cpu, pc: u32) { f_801fbdf4_b(c, pc) }
#[inline(always)]
fn f_801fbdf4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbdf4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbdf4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbdf4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbdfc;
            }
            0x801fbdfc => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 1) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbe08;
            }
            0x801fbe08 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.sth(c.r[3].wrapping_add(0x2), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbdf4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosSetFightEntryeId (0x801fbe18, 16 bytes)
#[inline(never)]
pub fn f_801fbe18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbe18) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbe18); p = true; }
    }
    f_801fbe18_b(c, 0x801fbe18);
    if p { crate::prof::exit(c, 0x801fbe18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbe18_at(c: &mut Cpu, pc: u32) { f_801fbe18_b(c, pc) }
#[inline(always)]
fn f_801fbe18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbe18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbe18, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbe18 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbe20;
            }
            0x801fbe20 => { ins += 2;
                c.sth(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbe18: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosGetParam1bantakaiFlag (0x801fbe28, 24 bytes)
#[inline(never)]
pub fn f_801fbe28(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbe28) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbe28); p = true; }
    }
    f_801fbe28_b(c, 0x801fbe28);
    if p { crate::prof::exit(c, 0x801fbe28); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbe28_at(c: &mut Cpu, pc: u32) { f_801fbe28_b(c, pc) }
#[inline(always)]
fn f_801fbe28_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbe28;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbe28, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbe28 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbe38; continue; }
                pc = 0x801fbe30;
            }
            0x801fbe30 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbe38;
            }
            0x801fbe38 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x12));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbe28: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosGetBadwazaHaveFlag (0x801fbe40, 24 bytes)
#[inline(never)]
pub fn f_801fbe40(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbe40) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbe40); p = true; }
    }
    f_801fbe40_b(c, 0x801fbe40);
    if p { crate::prof::exit(c, 0x801fbe40); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbe40_at(c: &mut Cpu, pc: u32) { f_801fbe40_b(c, pc) }
#[inline(always)]
fn f_801fbe40_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbe40;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbe40, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbe40 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbe50; continue; }
                pc = 0x801fbe48;
            }
            0x801fbe48 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbe50;
            }
            0x801fbe50 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x11));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbe40: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosGetDefense1banhikuiFlag (0x801fbe58, 24 bytes)
#[inline(never)]
pub fn f_801fbe58(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbe58) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbe58); p = true; }
    }
    f_801fbe58_b(c, 0x801fbe58);
    if p { crate::prof::exit(c, 0x801fbe58); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbe58_at(c: &mut Cpu, pc: u32) { f_801fbe58_b(c, pc) }
#[inline(always)]
fn f_801fbe58_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbe58;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbe58, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbe58 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbe68; continue; }
                pc = 0x801fbe60;
            }
            0x801fbe60 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbe68;
            }
            0x801fbe68 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x10));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbe58: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosGetLv1banhikuiFlag (0x801fbe70, 24 bytes)
#[inline(never)]
pub fn f_801fbe70(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbe70) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbe70); p = true; }
    }
    f_801fbe70_b(c, 0x801fbe70);
    if p { crate::prof::exit(c, 0x801fbe70); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbe70_at(c: &mut Cpu, pc: u32) { f_801fbe70_b(c, pc) }
#[inline(always)]
fn f_801fbe70_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbe70;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbe70, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbe70 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbe80; continue; }
                pc = 0x801fbe78;
            }
            0x801fbe78 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbe80;
            }
            0x801fbe80 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xf));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbe70: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosGetNowhp1banhikuiFlag (0x801fbe88, 24 bytes)
#[inline(never)]
pub fn f_801fbe88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbe88) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbe88); p = true; }
    }
    f_801fbe88_b(c, 0x801fbe88);
    if p { crate::prof::exit(c, 0x801fbe88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbe88_at(c: &mut Cpu, pc: u32) { f_801fbe88_b(c, pc) }
#[inline(always)]
fn f_801fbe88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbe88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbe88, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbe88 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbe98; continue; }
                pc = 0x801fbe90;
            }
            0x801fbe90 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbe98;
            }
            0x801fbe98 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xe));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbe88: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosGetStoreTokuseiData (0x801fbea0, 24 bytes)
#[inline(never)]
pub fn f_801fbea0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbea0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbea0); p = true; }
    }
    f_801fbea0_b(c, 0x801fbea0);
    if p { crate::prof::exit(c, 0x801fbea0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbea0_at(c: &mut Cpu, pc: u32) { f_801fbea0_b(c, pc) }
#[inline(always)]
fn f_801fbea0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbea0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbea0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbea0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbeb0; continue; }
                pc = 0x801fbea8;
            }
            0x801fbea8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbeb0;
            }
            0x801fbeb0 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbea0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosGetTokuseiFlag (0x801fbeb8, 24 bytes)
#[inline(never)]
pub fn f_801fbeb8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbeb8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbeb8); p = true; }
    }
    f_801fbeb8_b(c, 0x801fbeb8);
    if p { crate::prof::exit(c, 0x801fbeb8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbeb8_at(c: &mut Cpu, pc: u32) { f_801fbeb8_b(c, pc) }
#[inline(always)]
fn f_801fbeb8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbeb8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbeb8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbeb8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbec8; continue; }
                pc = 0x801fbec0;
            }
            0x801fbec0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbec8;
            }
            0x801fbec8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbeb8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerGetU16Arr (0x801fbed0, 52 bytes)
#[inline(never)]
pub fn f_801fbed0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbed0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbed0); p = true; }
    }
    f_801fbed0_b(c, 0x801fbed0);
    if p { crate::prof::exit(c, 0x801fbed0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbed0_at(c: &mut Cpu, pc: u32) { f_801fbed0_b(c, pc) }
#[inline(always)]
fn f_801fbed0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbed0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbed0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbed0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbee0; continue; }
                pc = 0x801fbed8;
            }
            0x801fbed8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbee0;
            }
            0x801fbee0 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 1) { pc = 0x801fbef4; continue; }
                pc = 0x801fbeec;
            }
            0x801fbeec => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbef4;
            }
            0x801fbef4 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbed0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerEnemyPokemonBiosGetFightEntryeId (0x801fbf04, 24 bytes)
#[inline(never)]
pub fn f_801fbf04(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbf04) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbf04); p = true; }
    }
    f_801fbf04_b(c, 0x801fbf04);
    if p { crate::prof::exit(c, 0x801fbf04); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbf04_at(c: &mut Cpu, pc: u32) { f_801fbf04_b(c, pc) }
#[inline(always)]
fn f_801fbf04_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbf04;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbf04, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbf04 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbf14; continue; }
                pc = 0x801fbf0c;
            }
            0x801fbf0c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbf14;
            }
            0x801fbf14 => { ins += 2;
                c.r[3] = c.lha_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbf04: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiValueAddsubDataBiosSetPrefixName (0x801fbf1c, 16 bytes)
#[inline(never)]
pub fn f_801fbf1c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbf1c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbf1c); p = true; }
    }
    f_801fbf1c_b(c, 0x801fbf1c);
    if p { crate::prof::exit(c, 0x801fbf1c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbf1c_at(c: &mut Cpu, pc: u32) { f_801fbf1c_b(c, pc) }
#[inline(always)]
fn f_801fbf1c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbf1c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbf1c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbf1c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbf24;
            }
            0x801fbf24 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbf1c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiValueAddsubDataBiosSetKoudouName (0x801fbf2c, 16 bytes)
#[inline(never)]
pub fn f_801fbf2c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbf2c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbf2c); p = true; }
    }
    f_801fbf2c_b(c, 0x801fbf2c);
    if p { crate::prof::exit(c, 0x801fbf2c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbf2c_at(c: &mut Cpu, pc: u32) { f_801fbf2c_b(c, pc) }
#[inline(always)]
fn f_801fbf2c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbf2c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbf2c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbf2c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbf34;
            }
            0x801fbf34 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbf2c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiValueAddsubDataBiosSetName (0x801fbf3c, 16 bytes)
#[inline(never)]
pub fn f_801fbf3c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbf3c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbf3c); p = true; }
    }
    f_801fbf3c_b(c, 0x801fbf3c);
    if p { crate::prof::exit(c, 0x801fbf3c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbf3c_at(c: &mut Cpu, pc: u32) { f_801fbf3c_b(c, pc) }
#[inline(always)]
fn f_801fbf3c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbf3c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbf3c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbf3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbf44;
            }
            0x801fbf44 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbf3c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiValueAddsubDataBiosSetValue (0x801fbf4c, 16 bytes)
#[inline(never)]
pub fn f_801fbf4c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbf4c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbf4c); p = true; }
    }
    f_801fbf4c_b(c, 0x801fbf4c);
    if p { crate::prof::exit(c, 0x801fbf4c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbf4c_at(c: &mut Cpu, pc: u32) { f_801fbf4c_b(c, pc) }
#[inline(always)]
fn f_801fbf4c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbf4c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbf4c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbf4c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbf54;
            }
            0x801fbf54 => { ins += 2;
                c.stw(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbf4c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiValueAddsubDataBiosGetPrefixName (0x801fbf5c, 24 bytes)
#[inline(never)]
pub fn f_801fbf5c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbf5c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbf5c); p = true; }
    }
    f_801fbf5c_b(c, 0x801fbf5c);
    if p { crate::prof::exit(c, 0x801fbf5c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbf5c_at(c: &mut Cpu, pc: u32) { f_801fbf5c_b(c, pc) }
#[inline(always)]
fn f_801fbf5c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbf5c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbf5c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbf5c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbf6c; continue; }
                pc = 0x801fbf64;
            }
            0x801fbf64 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbf6c;
            }
            0x801fbf6c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbf5c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiValueAddsubDataBiosGetKoudouName (0x801fbf74, 24 bytes)
#[inline(never)]
pub fn f_801fbf74(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbf74) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbf74); p = true; }
    }
    f_801fbf74_b(c, 0x801fbf74);
    if p { crate::prof::exit(c, 0x801fbf74); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbf74_at(c: &mut Cpu, pc: u32) { f_801fbf74_b(c, pc) }
#[inline(always)]
fn f_801fbf74_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbf74;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbf74, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbf74 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbf84; continue; }
                pc = 0x801fbf7c;
            }
            0x801fbf7c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbf84;
            }
            0x801fbf84 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbf74: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiValueAddsubDataBiosGetName (0x801fbf8c, 24 bytes)
#[inline(never)]
pub fn f_801fbf8c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbf8c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbf8c); p = true; }
    }
    f_801fbf8c_b(c, 0x801fbf8c);
    if p { crate::prof::exit(c, 0x801fbf8c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbf8c_at(c: &mut Cpu, pc: u32) { f_801fbf8c_b(c, pc) }
#[inline(always)]
fn f_801fbf8c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbf8c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbf8c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbf8c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbf9c; continue; }
                pc = 0x801fbf94;
            }
            0x801fbf94 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbf9c;
            }
            0x801fbf9c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbf8c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiValueAddsubDataBiosGetValue (0x801fbfa4, 24 bytes)
#[inline(never)]
pub fn f_801fbfa4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbfa4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbfa4); p = true; }
    }
    f_801fbfa4_b(c, 0x801fbfa4);
    if p { crate::prof::exit(c, 0x801fbfa4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbfa4_at(c: &mut Cpu, pc: u32) { f_801fbfa4_b(c, pc) }
#[inline(always)]
fn f_801fbfa4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbfa4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbfa4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbfa4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fbfb4; continue; }
                pc = 0x801fbfac;
            }
            0x801fbfac => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbfb4;
            }
            0x801fbfb4 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbfa4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiValueAddsubDataBiosGetPtr (0x801fbfbc, 44 bytes)
#[inline(never)]
pub fn f_801fbfbc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbfbc) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbfbc); p = true; }
    }
    f_801fbfbc_b(c, 0x801fbfbc);
    if p { crate::prof::exit(c, 0x801fbfbc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbfbc_at(c: &mut Cpu, pc: u32) { f_801fbfbc_b(c, pc) }
#[inline(always)]
fn f_801fbfbc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbfbc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbfbc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbfbc => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8708));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801fbfd8; continue; }
                pc = 0x801fbfd0;
            }
            0x801fbfd0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbfd8;
            }
            0x801fbfd8 => { ins += 4;
                c.r[0] = c.r[3].wrapping_mul(0x14);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff870c));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbfbc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetLastValueRevise (0x801fbfe8, 16 bytes)
#[inline(never)]
pub fn f_801fbfe8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbfe8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbfe8); p = true; }
    }
    f_801fbfe8_b(c, 0x801fbfe8);
    if p { crate::prof::exit(c, 0x801fbfe8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbfe8_at(c: &mut Cpu, pc: u32) { f_801fbfe8_b(c, pc) }
#[inline(always)]
fn f_801fbfe8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbfe8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbfe8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbfe8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fbff0;
            }
            0x801fbff0 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x19), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbfe8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetWazaNokoriPpValue (0x801fbff8, 16 bytes)
#[inline(never)]
pub fn f_801fbff8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fbff8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fbff8); p = true; }
    }
    f_801fbff8_b(c, 0x801fbff8);
    if p { crate::prof::exit(c, 0x801fbff8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fbff8_at(c: &mut Cpu, pc: u32) { f_801fbff8_b(c, pc) }
#[inline(always)]
fn f_801fbff8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fbff8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fbff8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fbff8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc000;
            }
            0x801fc000 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x18), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fbff8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetWazaRiskFlag (0x801fc008, 16 bytes)
#[inline(never)]
pub fn f_801fc008(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc008) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc008); p = true; }
    }
    f_801fc008_b(c, 0x801fc008);
    if p { crate::prof::exit(c, 0x801fc008); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc008_at(c: &mut Cpu, pc: u32) { f_801fc008_b(c, pc) }
#[inline(always)]
fn f_801fc008_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc008;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc008, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc008 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc010;
            }
            0x801fc010 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x17), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc008: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetWazaAvgValue (0x801fc018, 16 bytes)
#[inline(never)]
pub fn f_801fc018(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc018) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc018); p = true; }
    }
    f_801fc018_b(c, 0x801fc018);
    if p { crate::prof::exit(c, 0x801fc018); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc018_at(c: &mut Cpu, pc: u32) { f_801fc018_b(c, pc) }
#[inline(always)]
fn f_801fc018_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc018;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc018, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc018 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc020;
            }
            0x801fc020 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x16), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc018: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetWazaHitFlag (0x801fc028, 16 bytes)
#[inline(never)]
pub fn f_801fc028(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc028) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc028); p = true; }
    }
    f_801fc028_b(c, 0x801fc028);
    if p { crate::prof::exit(c, 0x801fc028); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc028_at(c: &mut Cpu, pc: u32) { f_801fc028_b(c, pc) }
#[inline(always)]
fn f_801fc028_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc028;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc028, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc028 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc030;
            }
            0x801fc030 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x15), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc028: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetWazaTypeReviseValue (0x801fc038, 68 bytes)
#[inline(never)]
pub fn f_801fc038(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc038) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc038); p = true; }
    }
    f_801fc038_b(c, 0x801fc038);
    if p { crate::prof::exit(c, 0x801fc038); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc038_at(c: &mut Cpu, pc: u32) { f_801fc038_b(c, pc) }
#[inline(always)]
fn f_801fc038_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc038;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc038, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc038 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc040;
            }
            0x801fc040 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fc04c; continue; }
                pc = 0x801fc044;
            }
            0x801fc044 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc06c; continue;
                pc = 0x801fc04c;
            }
            0x801fc04c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801fc060; continue; }
                pc = 0x801fc058;
            }
            0x801fc058 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc06c; continue;
                pc = 0x801fc060;
            }
            0x801fc060 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[4] = c.r[4].wrapping_add(0x24);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc06c;
            }
            0x801fc06c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc074;
            }
            0x801fc074 => { ins += 2;
                c.stb(c.r[4].wrapping_add(0x1), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc038: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetWazaTypeReviseTypeDataId (0x801fc07c, 68 bytes)
#[inline(never)]
pub fn f_801fc07c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc07c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc07c); p = true; }
    }
    f_801fc07c_b(c, 0x801fc07c);
    if p { crate::prof::exit(c, 0x801fc07c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc07c_at(c: &mut Cpu, pc: u32) { f_801fc07c_b(c, pc) }
#[inline(always)]
fn f_801fc07c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc07c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc07c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc07c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc084;
            }
            0x801fc084 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fc090; continue; }
                pc = 0x801fc088;
            }
            0x801fc088 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc0b0; continue;
                pc = 0x801fc090;
            }
            0x801fc090 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801fc0a4; continue; }
                pc = 0x801fc09c;
            }
            0x801fc09c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc0b0; continue;
                pc = 0x801fc0a4;
            }
            0x801fc0a4 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[4] = c.r[4].wrapping_add(0x24);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc0b0;
            }
            0x801fc0b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc0b8;
            }
            0x801fc0b8 => { ins += 2;
                c.stb(c.r[4], c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc07c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetPartFlag (0x801fc0c0, 16 bytes)
#[inline(never)]
pub fn f_801fc0c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc0c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc0c0); p = true; }
    }
    f_801fc0c0_b(c, 0x801fc0c0);
    if p { crate::prof::exit(c, 0x801fc0c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc0c0_at(c: &mut Cpu, pc: u32) { f_801fc0c0_b(c, pc) }
#[inline(always)]
fn f_801fc0c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc0c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc0c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc0c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc0c8;
            }
            0x801fc0c8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x14), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc0c0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetWazaInitValueFlag (0x801fc0d0, 16 bytes)
#[inline(never)]
pub fn f_801fc0d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc0d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc0d0); p = true; }
    }
    f_801fc0d0_b(c, 0x801fc0d0);
    if p { crate::prof::exit(c, 0x801fc0d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc0d0_at(c: &mut Cpu, pc: u32) { f_801fc0d0_b(c, pc) }
#[inline(always)]
fn f_801fc0d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc0d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc0d0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc0d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc0d8;
            }
            0x801fc0d8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x13), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc0d0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetWazaRndSelectFlag (0x801fc0e0, 16 bytes)
#[inline(never)]
pub fn f_801fc0e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc0e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc0e0); p = true; }
    }
    f_801fc0e0_b(c, 0x801fc0e0);
    if p { crate::prof::exit(c, 0x801fc0e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc0e0_at(c: &mut Cpu, pc: u32) { f_801fc0e0_b(c, pc) }
#[inline(always)]
fn f_801fc0e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc0e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc0e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc0e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc0e8;
            }
            0x801fc0e8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x12), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc0e0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetDefensePokemonRndSelectFlag (0x801fc0f0, 16 bytes)
#[inline(never)]
pub fn f_801fc0f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc0f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc0f0); p = true; }
    }
    f_801fc0f0_b(c, 0x801fc0f0);
    if p { crate::prof::exit(c, 0x801fc0f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc0f0_at(c: &mut Cpu, pc: u32) { f_801fc0f0_b(c, pc) }
#[inline(always)]
fn f_801fc0f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc0f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc0f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc0f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc0f8;
            }
            0x801fc0f8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x11), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc0f0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetAbicntMinValue (0x801fc100, 16 bytes)
#[inline(never)]
pub fn f_801fc100(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc100) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc100); p = true; }
    }
    f_801fc100_b(c, 0x801fc100);
    if p { crate::prof::exit(c, 0x801fc100); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc100_at(c: &mut Cpu, pc: u32) { f_801fc100_b(c, pc) }
#[inline(always)]
fn f_801fc100_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc100;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc100, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc100 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc108;
            }
            0x801fc108 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc100: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetAbicntMaxValue (0x801fc110, 16 bytes)
#[inline(never)]
pub fn f_801fc110(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc110) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc110); p = true; }
    }
    f_801fc110_b(c, 0x801fc110);
    if p { crate::prof::exit(c, 0x801fc110); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc110_at(c: &mut Cpu, pc: u32) { f_801fc110_b(c, pc) }
#[inline(always)]
fn f_801fc110_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc110;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc110, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc110 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc118;
            }
            0x801fc118 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xf), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc110: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetWazaDamageFlag (0x801fc120, 16 bytes)
#[inline(never)]
pub fn f_801fc120(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc120) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc120); p = true; }
    }
    f_801fc120_b(c, 0x801fc120);
    if p { crate::prof::exit(c, 0x801fc120); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc120_at(c: &mut Cpu, pc: u32) { f_801fc120_b(c, pc) }
#[inline(always)]
fn f_801fc120_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc120;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc120, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc120 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc128;
            }
            0x801fc128 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xe), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc120: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetNokoriHpValue (0x801fc130, 16 bytes)
#[inline(never)]
pub fn f_801fc130(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc130) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc130); p = true; }
    }
    f_801fc130_b(c, 0x801fc130);
    if p { crate::prof::exit(c, 0x801fc130); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc130_at(c: &mut Cpu, pc: u32) { f_801fc130_b(c, pc) }
#[inline(always)]
fn f_801fc130_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc130;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc130, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc130 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc138;
            }
            0x801fc138 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xd), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc130: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetTokuseiCheckFlag (0x801fc140, 16 bytes)
#[inline(never)]
pub fn f_801fc140(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc140) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc140); p = true; }
    }
    f_801fc140_b(c, 0x801fc140);
    if p { crate::prof::exit(c, 0x801fc140); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc140_at(c: &mut Cpu, pc: u32) { f_801fc140_b(c, pc) }
#[inline(always)]
fn f_801fc140_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc140;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc140, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc140 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc148;
            }
            0x801fc148 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc140: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetZokuseiReviseValue (0x801fc150, 68 bytes)
#[inline(never)]
pub fn f_801fc150(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc150) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc150); p = true; }
    }
    f_801fc150_b(c, 0x801fc150);
    if p { crate::prof::exit(c, 0x801fc150); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc150_at(c: &mut Cpu, pc: u32) { f_801fc150_b(c, pc) }
#[inline(always)]
fn f_801fc150_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc150;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc150, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc150 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc158;
            }
            0x801fc158 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fc164; continue; }
                pc = 0x801fc15c;
            }
            0x801fc15c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc184; continue;
                pc = 0x801fc164;
            }
            0x801fc164 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801fc178; continue; }
                pc = 0x801fc170;
            }
            0x801fc170 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc184; continue;
                pc = 0x801fc178;
            }
            0x801fc178 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[4] = c.r[4].wrapping_add(0x20);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc184;
            }
            0x801fc184 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc18c;
            }
            0x801fc18c => { ins += 2;
                c.stb(c.r[4].wrapping_add(0x1), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc150: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetZokuseiReviseZokuseiDataId (0x801fc194, 68 bytes)
#[inline(never)]
pub fn f_801fc194(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc194) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc194); p = true; }
    }
    f_801fc194_b(c, 0x801fc194);
    if p { crate::prof::exit(c, 0x801fc194); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc194_at(c: &mut Cpu, pc: u32) { f_801fc194_b(c, pc) }
#[inline(always)]
fn f_801fc194_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc194;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc194, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc194 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc19c;
            }
            0x801fc19c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fc1a8; continue; }
                pc = 0x801fc1a0;
            }
            0x801fc1a0 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc1c8; continue;
                pc = 0x801fc1a8;
            }
            0x801fc1a8 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801fc1bc; continue; }
                pc = 0x801fc1b4;
            }
            0x801fc1b4 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc1c8; continue;
                pc = 0x801fc1bc;
            }
            0x801fc1bc => { ins += 3;
                c.r[4] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[4] = c.r[4].wrapping_add(0x20);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc1c8;
            }
            0x801fc1c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc1d0;
            }
            0x801fc1d0 => { ins += 2;
                c.stb(c.r[4], c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc194: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetZokuseiCheckFlag (0x801fc1d8, 16 bytes)
#[inline(never)]
pub fn f_801fc1d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc1d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc1d8); p = true; }
    }
    f_801fc1d8_b(c, 0x801fc1d8);
    if p { crate::prof::exit(c, 0x801fc1d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc1d8_at(c: &mut Cpu, pc: u32) { f_801fc1d8_b(c, pc) }
#[inline(always)]
fn f_801fc1d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc1d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc1d8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc1d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc1e0;
            }
            0x801fc1e0 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xb), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc1d8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetParamStoreFlag (0x801fc1e8, 16 bytes)
#[inline(never)]
pub fn f_801fc1e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc1e8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc1e8); p = true; }
    }
    f_801fc1e8_b(c, 0x801fc1e8);
    if p { crate::prof::exit(c, 0x801fc1e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc1e8_at(c: &mut Cpu, pc: u32) { f_801fc1e8_b(c, pc) }
#[inline(always)]
fn f_801fc1e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc1e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc1e8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc1e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc1f0;
            }
            0x801fc1f0 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc1e8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetParamExpectFlag (0x801fc1f8, 16 bytes)
#[inline(never)]
pub fn f_801fc1f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc1f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc1f8); p = true; }
    }
    f_801fc1f8_b(c, 0x801fc1f8);
    if p { crate::prof::exit(c, 0x801fc1f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc1f8_at(c: &mut Cpu, pc: u32) { f_801fc1f8_b(c, pc) }
#[inline(always)]
fn f_801fc1f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc1f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc1f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc1f8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc200;
            }
            0x801fc200 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x9), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc1f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetItemValue (0x801fc208, 16 bytes)
#[inline(never)]
pub fn f_801fc208(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc208) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc208); p = true; }
    }
    f_801fc208_b(c, 0x801fc208);
    if p { crate::prof::exit(c, 0x801fc208); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc208_at(c: &mut Cpu, pc: u32) { f_801fc208_b(c, pc) }
#[inline(always)]
fn f_801fc208_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc208;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc208, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc208 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc210;
            }
            0x801fc210 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc208: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetIrekaeValue (0x801fc218, 16 bytes)
#[inline(never)]
pub fn f_801fc218(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc218) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc218); p = true; }
    }
    f_801fc218_b(c, 0x801fc218);
    if p { crate::prof::exit(c, 0x801fc218); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc218_at(c: &mut Cpu, pc: u32) { f_801fc218_b(c, pc) }
#[inline(always)]
fn f_801fc218_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc218;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc218, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc218 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc220;
            }
            0x801fc220 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x7), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc218: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetComboValue (0x801fc228, 16 bytes)
#[inline(never)]
pub fn f_801fc228(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc228) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc228); p = true; }
    }
    f_801fc228_b(c, 0x801fc228);
    if p { crate::prof::exit(c, 0x801fc228); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc228_at(c: &mut Cpu, pc: u32) { f_801fc228_b(c, pc) }
#[inline(always)]
fn f_801fc228_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc228;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc228, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc228 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc230;
            }
            0x801fc230 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc228: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetPokemonJoutaiFlag (0x801fc238, 16 bytes)
#[inline(never)]
pub fn f_801fc238(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc238) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc238); p = true; }
    }
    f_801fc238_b(c, 0x801fc238);
    if p { crate::prof::exit(c, 0x801fc238); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc238_at(c: &mut Cpu, pc: u32) { f_801fc238_b(c, pc) }
#[inline(always)]
fn f_801fc238_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc238;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc238, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc238 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc240;
            }
            0x801fc240 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x5), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc238: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetKeyPlayerFlag (0x801fc248, 16 bytes)
#[inline(never)]
pub fn f_801fc248(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc248) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc248); p = true; }
    }
    f_801fc248_b(c, 0x801fc248);
    if p { crate::prof::exit(c, 0x801fc248); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc248_at(c: &mut Cpu, pc: u32) { f_801fc248_b(c, pc) }
#[inline(always)]
fn f_801fc248_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc248;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc248, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc248 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc250;
            }
            0x801fc250 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc248: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetPokemonDataOrderAceBossFlag (0x801fc258, 16 bytes)
#[inline(never)]
pub fn f_801fc258(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc258) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc258); p = true; }
    }
    f_801fc258_b(c, 0x801fc258);
    if p { crate::prof::exit(c, 0x801fc258); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc258_at(c: &mut Cpu, pc: u32) { f_801fc258_b(c, pc) }
#[inline(always)]
fn f_801fc258_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc258;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc258, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc258 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc260;
            }
            0x801fc260 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x3), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc258: bad pc {:#x}", pc),
        }
    }
}

