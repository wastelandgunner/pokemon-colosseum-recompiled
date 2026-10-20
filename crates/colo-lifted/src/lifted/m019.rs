#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// pokemonBiosSetConditionAmari (0x8011de38, 16 bytes)
#[inline(never)]
pub fn f_8011de38(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011de38) { return; }
        if c.prof { crate::prof::enter(c, 0x8011de38); p = true; }
    }
    f_8011de38_b(c, 0x8011de38);
    if p { crate::prof::exit(c, 0x8011de38); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011de38_at(c: &mut Cpu, pc: u32) { f_8011de38_b(c, pc) }
#[inline(always)]
fn f_8011de38_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011de38;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011de38, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011de38 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011de40;
            }
            0x8011de40 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x74), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011de38: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetLevel (0x8011de48, 32 bytes)
#[inline(never)]
pub fn f_8011de48(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011de48) { return; }
        if c.prof { crate::prof::enter(c, 0x8011de48); p = true; }
    }
    f_8011de48_b(c, 0x8011de48);
    if p { crate::prof::exit(c, 0x8011de48); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011de48_at(c: &mut Cpu, pc: u32) { f_8011de48_b(c, pc) }
#[inline(always)]
fn f_8011de48_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011de48;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011de48, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011de48 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011de50;
            }
            0x8011de50 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x64);
                if !cr_bit(cr, 1) { pc = 0x8011de60; continue; }
                pc = 0x8011de5c;
            }
            0x8011de5c => { ins += 1;
                c.r[4] = 0x64;
                pc = 0x8011de60;
            }
            0x8011de60 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x60), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011de48: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPoolFriend (0x8011de68, 32 bytes)
#[inline(never)]
pub fn f_8011de68(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011de68) { return; }
        if c.prof { crate::prof::enter(c, 0x8011de68); p = true; }
    }
    f_8011de68_b(c, 0x8011de68);
    if p { crate::prof::exit(c, 0x8011de68); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011de68_at(c: &mut Cpu, pc: u32) { f_8011de68_b(c, pc) }
#[inline(always)]
fn f_8011de68_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011de68;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011de68, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011de68 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011de70;
            }
            0x8011de70 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if !cr_bit(cr, 1) { pc = 0x8011de80; continue; }
                pc = 0x8011de7c;
            }
            0x8011de7c => { ins += 1;
                c.r[4] = 0xff;
                pc = 0x8011de80;
            }
            0x8011de80 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0xe4), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011de68: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPoolExp (0x8011de88, 16 bytes)
#[inline(never)]
pub fn f_8011de88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011de88) { return; }
        if c.prof { crate::prof::enter(c, 0x8011de88); p = true; }
    }
    f_8011de88_b(c, 0x8011de88);
    if p { crate::prof::exit(c, 0x8011de88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011de88_at(c: &mut Cpu, pc: u32) { f_8011de88_b(c, pc) }
#[inline(always)]
fn f_8011de88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011de88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011de88, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011de88 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011de90;
            }
            0x8011de90 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0xe0), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011de88: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetExp (0x8011de98, 16 bytes)
#[inline(never)]
pub fn f_8011de98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011de98) { return; }
        if c.prof { crate::prof::enter(c, 0x8011de98); p = true; }
    }
    f_8011de98_b(c, 0x8011de98);
    if p { crate::prof::exit(c, 0x8011de98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011de98_at(c: &mut Cpu, pc: u32) { f_8011de98_b(c, pc) }
#[inline(always)]
fn f_8011de98_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011de98;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011de98, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011de98 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011dea0;
            }
            0x8011dea0 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x5c), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011de98: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetNicknameOrgPtr (0x8011dea8, 60 bytes)
#[inline(never)]
pub fn f_8011dea8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dea8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dea8); p = true; }
    }
    f_8011dea8_b(c, 0x8011dea8);
    if p { crate::prof::exit(c, 0x8011dea8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dea8_at(c: &mut Cpu, pc: u32) { f_8011dea8_b(c, pc) }
#[inline(always)]
fn f_8011dea8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011dea8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011dea8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011dea8 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8011ded4; continue; }
                pc = 0x8011debc;
            }
            0x8011debc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011dec8; continue; }
                pc = 0x8011dec4;
            }
            0x8011dec4 => { ins += 1;
                pc = 0x8011ded4; continue;
                pc = 0x8011dec8;
            }
            0x8011dec8 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x44);
                c.r[5] = 0xb;
                c.lr = 0x8011ded4; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f9d24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011ded4;
            }
            0x8011ded4 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011dea8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetNicknamePtr (0x8011dee4, 112 bytes)
#[inline(never)]
pub fn f_8011dee4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dee4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dee4); p = true; }
    }
    f_8011dee4_b(c, 0x8011dee4);
    if p { crate::prof::exit(c, 0x8011dee4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dee4_at(c: &mut Cpu, pc: u32) { f_8011dee4_b(c, pc) }
#[inline(always)]
fn f_8011dee4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011dee4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011dee4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011dee4 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x8011df3c; continue; }
                pc = 0x8011df04;
            }
            0x8011df04 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011df10; continue; }
                pc = 0x8011df0c;
            }
            0x8011df0c => { ins += 1;
                pc = 0x8011df3c; continue;
                pc = 0x8011df10;
            }
            0x8011df10 => { ins += 3;
                c.r[3] = c.r[30].wrapping_add(0x2e);
                c.r[5] = 0xb;
                c.lr = 0x8011df1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f9d24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011df1c;
            }
            0x8011df1c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x8011df3c; continue; }
                pc = 0x8011df24;
            }
            0x8011df24 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x8011df3c; continue; }
                pc = 0x8011df2c;
            }
            0x8011df2c => { ins += 4;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[30].wrapping_add(0x44);
                c.r[5] = 0xb;
                c.lr = 0x8011df3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f9d24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011df3c;
            }
            0x8011df3c => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011dee4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetCatchTrainerNamePtr (0x8011df54, 60 bytes)
#[inline(never)]
pub fn f_8011df54(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011df54) { return; }
        if c.prof { crate::prof::enter(c, 0x8011df54); p = true; }
    }
    f_8011df54_b(c, 0x8011df54);
    if p { crate::prof::exit(c, 0x8011df54); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011df54_at(c: &mut Cpu, pc: u32) { f_8011df54_b(c, pc) }
#[inline(always)]
fn f_8011df54_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011df54;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011df54, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011df54 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8011df80; continue; }
                pc = 0x8011df68;
            }
            0x8011df68 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011df74; continue; }
                pc = 0x8011df70;
            }
            0x8011df70 => { ins += 1;
                pc = 0x8011df80; continue;
                pc = 0x8011df74;
            }
            0x8011df74 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(0x18);
                c.r[5] = 0xb;
                c.lr = 0x8011df80; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f9d24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011df80;
            }
            0x8011df80 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011df54: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetCatchTrainerRnd (0x8011df90, 16 bytes)
#[inline(never)]
pub fn f_8011df90(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011df90) { return; }
        if c.prof { crate::prof::enter(c, 0x8011df90); p = true; }
    }
    f_8011df90_b(c, 0x8011df90);
    if p { crate::prof::exit(c, 0x8011df90); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011df90_at(c: &mut Cpu, pc: u32) { f_8011df90_b(c, pc) }
#[inline(always)]
fn f_8011df90_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011df90;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011df90, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011df90 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011df98;
            }
            0x8011df98 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x14), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011df90: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetCatchTrainerSex (0x8011dfa0, 16 bytes)
#[inline(never)]
pub fn f_8011dfa0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dfa0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dfa0); p = true; }
    }
    f_8011dfa0_b(c, 0x8011dfa0);
    if p { crate::prof::exit(c, 0x8011dfa0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dfa0_at(c: &mut Cpu, pc: u32) { f_8011dfa0_b(c, pc) }
#[inline(always)]
fn f_8011dfa0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011dfa0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011dfa0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011dfa0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011dfa8;
            }
            0x8011dfa8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011dfa0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetCatchBallId (0x8011dfb0, 16 bytes)
#[inline(never)]
pub fn f_8011dfb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dfb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dfb0); p = true; }
    }
    f_8011dfb0_b(c, 0x8011dfb0);
    if p { crate::prof::exit(c, 0x8011dfb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dfb0_at(c: &mut Cpu, pc: u32) { f_8011dfb0_b(c, pc) }
#[inline(always)]
fn f_8011dfb0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011dfb0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011dfb0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011dfb0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011dfb8;
            }
            0x8011dfb8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xf), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011dfb0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetCatchLevel (0x8011dfc0, 16 bytes)
#[inline(never)]
pub fn f_8011dfc0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dfc0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dfc0); p = true; }
    }
    f_8011dfc0_b(c, 0x8011dfc0);
    if p { crate::prof::exit(c, 0x8011dfc0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dfc0_at(c: &mut Cpu, pc: u32) { f_8011dfc0_b(c, pc) }
#[inline(always)]
fn f_8011dfc0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011dfc0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011dfc0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011dfc0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011dfc8;
            }
            0x8011dfc8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xe), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011dfc0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetCatchFloorId (0x8011dfd0, 16 bytes)
#[inline(never)]
pub fn f_8011dfd0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dfd0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dfd0); p = true; }
    }
    f_8011dfd0_b(c, 0x8011dfd0);
    if p { crate::prof::exit(c, 0x8011dfd0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dfd0_at(c: &mut Cpu, pc: u32) { f_8011dfd0_b(c, pc) }
#[inline(always)]
fn f_8011dfd0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011dfd0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011dfd0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011dfd0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011dfd8;
            }
            0x8011dfd8 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011dfd0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetRnd (0x8011dfe0, 16 bytes)
#[inline(never)]
pub fn f_8011dfe0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dfe0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dfe0); p = true; }
    }
    f_8011dfe0_b(c, 0x8011dfe0);
    if p { crate::prof::exit(c, 0x8011dfe0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dfe0_at(c: &mut Cpu, pc: u32) { f_8011dfe0_b(c, pc) }
#[inline(always)]
fn f_8011dfe0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011dfe0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011dfe0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011dfe0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011dfe8;
            }
            0x8011dfe8 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011dfe0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPokemonDataId (0x8011dff0, 16 bytes)
#[inline(never)]
pub fn f_8011dff0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dff0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dff0); p = true; }
    }
    f_8011dff0_b(c, 0x8011dff0);
    if p { crate::prof::exit(c, 0x8011dff0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dff0_at(c: &mut Cpu, pc: u32) { f_8011dff0_b(c, pc) }
#[inline(always)]
fn f_8011dff0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011dff0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011dff0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011dff0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011dff8;
            }
            0x8011dff8 => { ins += 2;
                c.sth(c.r[3], c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011dff0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011E000 (0x8011e000, 24 bytes)
#[inline(never)]
pub fn f_8011e000(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e000) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e000); p = true; }
    }
    f_8011e000_b(c, 0x8011e000);
    if p { crate::prof::exit(c, 0x8011e000); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e000_at(c: &mut Cpu, pc: u32) { f_8011e000_b(c, pc) }
#[inline(always)]
fn f_8011e000_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e000;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e000, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e000 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e010; continue; }
                pc = 0x8011e008;
            }
            0x8011e008 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e010;
            }
            0x8011e010 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e000: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011E018 (0x8011e018, 24 bytes)
#[inline(never)]
pub fn f_8011e018(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e018) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e018); p = true; }
    }
    f_8011e018_b(c, 0x8011e018);
    if p { crate::prof::exit(c, 0x8011e018); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e018_at(c: &mut Cpu, pc: u32) { f_8011e018_b(c, pc) }
#[inline(always)]
fn f_8011e018_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e018;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e018, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e018 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e028; continue; }
                pc = 0x8011e020;
            }
            0x8011e020 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e028;
            }
            0x8011e028 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x3));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e018: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011E030 (0x8011e030, 24 bytes)
#[inline(never)]
pub fn f_8011e030(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e030) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e030); p = true; }
    }
    f_8011e030_b(c, 0x8011e030);
    if p { crate::prof::exit(c, 0x8011e030); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e030_at(c: &mut Cpu, pc: u32) { f_8011e030_b(c, pc) }
#[inline(always)]
fn f_8011e030_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e030;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e030, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e030 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e040; continue; }
                pc = 0x8011e038;
            }
            0x8011e038 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e040;
            }
            0x8011e040 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x16));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e030: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011E048 (0x8011e048, 48 bytes)
#[inline(never)]
pub fn f_8011e048(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e048) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e048); p = true; }
    }
    f_8011e048_b(c, 0x8011e048);
    if p { crate::prof::exit(c, 0x8011e048); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e048_at(c: &mut Cpu, pc: u32) { f_8011e048_b(c, pc) }
#[inline(always)]
fn f_8011e048_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e048;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e048, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e048 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e058; continue; }
                pc = 0x8011e050;
            }
            0x8011e050 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e058;
            }
            0x8011e058 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8011e06c; continue; }
                pc = 0x8011e064;
            }
            0x8011e064 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e06c;
            }
            0x8011e06c => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x6e));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e048: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetKowaza (0x8011e078, 52 bytes)
#[inline(never)]
pub fn f_8011e078(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e078) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e078); p = true; }
    }
    f_8011e078_b(c, 0x8011e078);
    if p { crate::prof::exit(c, 0x8011e078); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e078_at(c: &mut Cpu, pc: u32) { f_8011e078_b(c, pc) }
#[inline(always)]
fn f_8011e078_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e078;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e078, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e078 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e088; continue; }
                pc = 0x8011e080;
            }
            0x8011e080 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e088;
            }
            0x8011e088 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if cr_bit(cr, 0) { pc = 0x8011e09c; continue; }
                pc = 0x8011e094;
            }
            0x8011e094 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e09c;
            }
            0x8011e09c => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x74));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e078: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetPkxDataId (0x8011e0ac, 24 bytes)
#[inline(never)]
pub fn f_8011e0ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e0ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e0ac); p = true; }
    }
    f_8011e0ac_b(c, 0x8011e0ac);
    if p { crate::prof::exit(c, 0x8011e0ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e0ac_at(c: &mut Cpu, pc: u32) { f_8011e0ac_b(c, pc) }
#[inline(always)]
fn f_8011e0ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e0ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e0ac, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e0ac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e0bc; continue; }
                pc = 0x8011e0b4;
            }
            0x8011e0b4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e0bc;
            }
            0x8011e0bc => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x2c));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e0ac: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetNumPokemon (0x8011e0c4, 24 bytes)
#[inline(never)]
pub fn f_8011e0c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e0c4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e0c4); p = true; }
    }
    f_8011e0c4_b(c, 0x8011e0c4);
    if p { crate::prof::exit(c, 0x8011e0c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e0c4_at(c: &mut Cpu, pc: u32) { f_8011e0c4_b(c, pc) }
#[inline(always)]
fn f_8011e0c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e0c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e0c4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e0c4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e0d4; continue; }
                pc = 0x8011e0cc;
            }
            0x8011e0cc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e0d4;
            }
            0x8011e0d4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x12));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e0c4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetNumZukan (0x8011e0dc, 24 bytes)
#[inline(never)]
pub fn f_8011e0dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e0dc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e0dc); p = true; }
    }
    f_8011e0dc_b(c, 0x8011e0dc);
    if p { crate::prof::exit(c, 0x8011e0dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e0dc_at(c: &mut Cpu, pc: u32) { f_8011e0dc_b(c, pc) }
#[inline(always)]
fn f_8011e0dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e0dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e0dc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e0dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e0ec; continue; }
                pc = 0x8011e0e4;
            }
            0x8011e0e4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e0ec;
            }
            0x8011e0ec => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x14));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e0dc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetTukamaetaFlag (0x8011e0f4, 52 bytes)
#[inline(never)]
pub fn f_8011e0f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e0f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e0f4); p = true; }
    }
    f_8011e0f4_b(c, 0x8011e0f4);
    if p { crate::prof::exit(c, 0x8011e0f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e0f4_at(c: &mut Cpu, pc: u32) { f_8011e0f4_b(c, pc) }
#[inline(always)]
fn f_8011e0f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e0f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e0f4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e0f4 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011e110; continue; }
                pc = 0x8011e108;
            }
            0x8011e108 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e118; continue;
                pc = 0x8011e110;
            }
            0x8011e110 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x28));
                c.lr = 0x8011e118; { c.steps = st; c.cycles = ins; c.cr = cr; f_801906a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011e118;
            }
            0x8011e118 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e0f4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetMitaFlag (0x8011e128, 52 bytes)
#[inline(never)]
pub fn f_8011e128(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e128) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e128); p = true; }
    }
    f_8011e128_b(c, 0x8011e128);
    if p { crate::prof::exit(c, 0x8011e128); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e128_at(c: &mut Cpu, pc: u32) { f_8011e128_b(c, pc) }
#[inline(always)]
fn f_8011e128_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e128;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e128, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e128 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011e144; continue; }
                pc = 0x8011e13c;
            }
            0x8011e13c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e14c; continue;
                pc = 0x8011e144;
            }
            0x8011e144 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x24));
                c.lr = 0x8011e14c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801906a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011e14c;
            }
            0x8011e14c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e128: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetVoice (0x8011e15c, 24 bytes)
#[inline(never)]
pub fn f_8011e15c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e15c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e15c); p = true; }
    }
    f_8011e15c_b(c, 0x8011e15c);
    if p { crate::prof::exit(c, 0x8011e15c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e15c_at(c: &mut Cpu, pc: u32) { f_8011e15c_b(c, pc) }
#[inline(always)]
fn f_8011e15c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e15c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e15c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e15c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e16c; continue; }
                pc = 0x8011e164;
            }
            0x8011e164 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e16c;
            }
            0x8011e16c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xe));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e15c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetDoc (0x8011e174, 24 bytes)
#[inline(never)]
pub fn f_8011e174(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e174) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e174); p = true; }
    }
    f_8011e174_b(c, 0x8011e174);
    if p { crate::prof::exit(c, 0x8011e174); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e174_at(c: &mut Cpu, pc: u32) { f_8011e174_b(c, pc) }
#[inline(always)]
fn f_8011e174_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e174;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e174, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e174 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e184; continue; }
                pc = 0x8011e17c;
            }
            0x8011e17c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e184;
            }
            0x8011e184 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x20));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e174: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetWeight (0x8011e18c, 24 bytes)
#[inline(never)]
pub fn f_8011e18c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e18c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e18c); p = true; }
    }
    f_8011e18c_b(c, 0x8011e18c);
    if p { crate::prof::exit(c, 0x8011e18c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e18c_at(c: &mut Cpu, pc: u32) { f_8011e18c_b(c, pc) }
#[inline(always)]
fn f_8011e18c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e18c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e18c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e18c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e19c; continue; }
                pc = 0x8011e194;
            }
            0x8011e194 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e19c;
            }
            0x8011e19c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e18c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetHeight (0x8011e1a4, 24 bytes)
#[inline(never)]
pub fn f_8011e1a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e1a4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e1a4); p = true; }
    }
    f_8011e1a4_b(c, 0x8011e1a4);
    if p { crate::prof::exit(c, 0x8011e1a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e1a4_at(c: &mut Cpu, pc: u32) { f_8011e1a4_b(c, pc) }
#[inline(always)]
fn f_8011e1a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e1a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e1a4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e1a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e1b4; continue; }
                pc = 0x8011e1ac;
            }
            0x8011e1ac => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e1b4;
            }
            0x8011e1b4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e1a4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetTypeName (0x8011e1bc, 24 bytes)
#[inline(never)]
pub fn f_8011e1bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e1bc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e1bc); p = true; }
    }
    f_8011e1bc_b(c, 0x8011e1bc);
    if p { crate::prof::exit(c, 0x8011e1bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e1bc_at(c: &mut Cpu, pc: u32) { f_8011e1bc_b(c, pc) }
#[inline(always)]
fn f_8011e1bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e1bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e1bc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e1bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e1cc; continue; }
                pc = 0x8011e1c4;
            }
            0x8011e1c4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e1cc;
            }
            0x8011e1cc => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x1c));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e1bc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetColor (0x8011e1d4, 72 bytes)
#[inline(never)]
pub fn f_8011e1d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e1d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e1d4); p = true; }
    }
    f_8011e1d4_b(c, 0x8011e1d4);
    if p { crate::prof::exit(c, 0x8011e1d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e1d4_at(c: &mut Cpu, pc: u32) { f_8011e1d4_b(c, pc) }
#[inline(always)]
fn f_8011e1d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e1d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e1d4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e1d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e1e4; continue; }
                pc = 0x8011e1dc;
            }
            0x8011e1dc => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e204; continue;
                pc = 0x8011e1e4;
            }
            0x8011e1e4 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8011e1f8; continue; }
                pc = 0x8011e1f0;
            }
            0x8011e1f0 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e204; continue;
                pc = 0x8011e1f8;
            }
            0x8011e1f8 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7fff8;
                c.r[4] = c.r[4].wrapping_add(0x10c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011e204;
            }
            0x8011e204 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e214; continue; }
                pc = 0x8011e20c;
            }
            0x8011e20c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e214;
            }
            0x8011e214 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e1d4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetStatusFaceMenuSpriteId (0x8011e21c, 72 bytes)
#[inline(never)]
pub fn f_8011e21c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e21c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e21c); p = true; }
    }
    f_8011e21c_b(c, 0x8011e21c);
    if p { crate::prof::exit(c, 0x8011e21c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e21c_at(c: &mut Cpu, pc: u32) { f_8011e21c_b(c, pc) }
#[inline(always)]
fn f_8011e21c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e21c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e21c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e21c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e22c; continue; }
                pc = 0x8011e224;
            }
            0x8011e224 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e24c; continue;
                pc = 0x8011e22c;
            }
            0x8011e22c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8011e240; continue; }
                pc = 0x8011e238;
            }
            0x8011e238 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e24c; continue;
                pc = 0x8011e240;
            }
            0x8011e240 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7fff8;
                c.r[4] = c.r[4].wrapping_add(0x10c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011e24c;
            }
            0x8011e24c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e25c; continue; }
                pc = 0x8011e254;
            }
            0x8011e254 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e25c;
            }
            0x8011e25c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[4].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e21c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetPokebodyId (0x8011e264, 72 bytes)
#[inline(never)]
pub fn f_8011e264(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e264) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e264); p = true; }
    }
    f_8011e264_b(c, 0x8011e264);
    if p { crate::prof::exit(c, 0x8011e264); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e264_at(c: &mut Cpu, pc: u32) { f_8011e264_b(c, pc) }
#[inline(always)]
fn f_8011e264_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e264;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e264, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e264 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e274; continue; }
                pc = 0x8011e26c;
            }
            0x8011e26c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e294; continue;
                pc = 0x8011e274;
            }
            0x8011e274 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8011e288; continue; }
                pc = 0x8011e280;
            }
            0x8011e280 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e294; continue;
                pc = 0x8011e288;
            }
            0x8011e288 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7fff8;
                c.r[4] = c.r[4].wrapping_add(0x10c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011e294;
            }
            0x8011e294 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e2a4; continue; }
                pc = 0x8011e29c;
            }
            0x8011e29c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e2a4;
            }
            0x8011e2a4 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e264: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetWazaMcn (0x8011e2ac, 48 bytes)
#[inline(never)]
pub fn f_8011e2ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e2ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e2ac); p = true; }
    }
    f_8011e2ac_b(c, 0x8011e2ac);
    if p { crate::prof::exit(c, 0x8011e2ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e2ac_at(c: &mut Cpu, pc: u32) { f_8011e2ac_b(c, pc) }
#[inline(always)]
fn f_8011e2ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e2ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e2ac, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e2ac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e2bc; continue; }
                pc = 0x8011e2b4;
            }
            0x8011e2b4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e2bc;
            }
            0x8011e2bc => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3a);
                if cr_bit(cr, 0) { pc = 0x8011e2d0; continue; }
                pc = 0x8011e2c8;
            }
            0x8011e2c8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e2d0;
            }
            0x8011e2d0 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x34));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e2ac: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGetWazaDataId (0x8011e2dc, 72 bytes)
#[inline(never)]
pub fn f_8011e2dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e2dc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e2dc); p = true; }
    }
    f_8011e2dc_b(c, 0x8011e2dc);
    if p { crate::prof::exit(c, 0x8011e2dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e2dc_at(c: &mut Cpu, pc: u32) { f_8011e2dc_b(c, pc) }
#[inline(always)]
fn f_8011e2dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e2dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e2dc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e2dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e2ec; continue; }
                pc = 0x8011e2e4;
            }
            0x8011e2e4 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e30c; continue;
                pc = 0x8011e2ec;
            }
            0x8011e2ec => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x14);
                if cr_bit(cr, 0) { pc = 0x8011e300; continue; }
                pc = 0x8011e2f8;
            }
            0x8011e2f8 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e30c; continue;
                pc = 0x8011e300;
            }
            0x8011e300 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(2) & 0x3fffc;
                c.r[4] = c.r[4].wrapping_add(0xba);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011e30c;
            }
            0x8011e30c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e31c; continue; }
                pc = 0x8011e314;
            }
            0x8011e314 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e31c;
            }
            0x8011e31c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[4].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e2dc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGetWazaLevel (0x8011e324, 72 bytes)
#[inline(never)]
pub fn f_8011e324(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e324) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e324); p = true; }
    }
    f_8011e324_b(c, 0x8011e324);
    if p { crate::prof::exit(c, 0x8011e324); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e324_at(c: &mut Cpu, pc: u32) { f_8011e324_b(c, pc) }
#[inline(always)]
fn f_8011e324_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e324;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e324, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e324 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e334; continue; }
                pc = 0x8011e32c;
            }
            0x8011e32c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e354; continue;
                pc = 0x8011e334;
            }
            0x8011e334 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x14);
                if cr_bit(cr, 0) { pc = 0x8011e348; continue; }
                pc = 0x8011e340;
            }
            0x8011e340 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e354; continue;
                pc = 0x8011e348;
            }
            0x8011e348 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(2) & 0x3fffc;
                c.r[4] = c.r[4].wrapping_add(0xba);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011e354;
            }
            0x8011e354 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e364; continue; }
                pc = 0x8011e35c;
            }
            0x8011e35c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e364;
            }
            0x8011e364 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e324: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetSinkaPokemonDataId (0x8011e36c, 72 bytes)
#[inline(never)]
pub fn f_8011e36c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e36c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e36c); p = true; }
    }
    f_8011e36c_b(c, 0x8011e36c);
    if p { crate::prof::exit(c, 0x8011e36c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e36c_at(c: &mut Cpu, pc: u32) { f_8011e36c_b(c, pc) }
#[inline(always)]
fn f_8011e36c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e36c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e36c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e36c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e37c; continue; }
                pc = 0x8011e374;
            }
            0x8011e374 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e39c; continue;
                pc = 0x8011e37c;
            }
            0x8011e37c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5);
                if cr_bit(cr, 0) { pc = 0x8011e390; continue; }
                pc = 0x8011e388;
            }
            0x8011e388 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e39c; continue;
                pc = 0x8011e390;
            }
            0x8011e390 => { ins += 5;
                c.r[4] = c.r[0].wrapping_mul(0x6);
                c.r[4] = c.r[4].wrapping_add(0x9c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011e39c;
            }
            0x8011e39c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e3ac; continue; }
                pc = 0x8011e3a4;
            }
            0x8011e3a4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e3ac;
            }
            0x8011e3ac => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[4].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e36c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetSinkaBuff (0x8011e3b4, 72 bytes)
#[inline(never)]
pub fn f_8011e3b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e3b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e3b4); p = true; }
    }
    f_8011e3b4_b(c, 0x8011e3b4);
    if p { crate::prof::exit(c, 0x8011e3b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e3b4_at(c: &mut Cpu, pc: u32) { f_8011e3b4_b(c, pc) }
#[inline(always)]
fn f_8011e3b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e3b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e3b4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e3b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e3c4; continue; }
                pc = 0x8011e3bc;
            }
            0x8011e3bc => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e3e4; continue;
                pc = 0x8011e3c4;
            }
            0x8011e3c4 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5);
                if cr_bit(cr, 0) { pc = 0x8011e3d8; continue; }
                pc = 0x8011e3d0;
            }
            0x8011e3d0 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e3e4; continue;
                pc = 0x8011e3d8;
            }
            0x8011e3d8 => { ins += 5;
                c.r[4] = c.r[0].wrapping_mul(0x6);
                c.r[4] = c.r[4].wrapping_add(0x9c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011e3e4;
            }
            0x8011e3e4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e3f4; continue; }
                pc = 0x8011e3ec;
            }
            0x8011e3ec => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e3f4;
            }
            0x8011e3f4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[4].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e3b4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetSinkaKind (0x8011e3fc, 72 bytes)
#[inline(never)]
pub fn f_8011e3fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e3fc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e3fc); p = true; }
    }
    f_8011e3fc_b(c, 0x8011e3fc);
    if p { crate::prof::exit(c, 0x8011e3fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e3fc_at(c: &mut Cpu, pc: u32) { f_8011e3fc_b(c, pc) }
#[inline(always)]
fn f_8011e3fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e3fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e3fc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e3fc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e40c; continue; }
                pc = 0x8011e404;
            }
            0x8011e404 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e42c; continue;
                pc = 0x8011e40c;
            }
            0x8011e40c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5);
                if cr_bit(cr, 0) { pc = 0x8011e420; continue; }
                pc = 0x8011e418;
            }
            0x8011e418 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e42c; continue;
                pc = 0x8011e420;
            }
            0x8011e420 => { ins += 5;
                c.r[4] = c.r[0].wrapping_mul(0x6);
                c.r[4] = c.r[4].wrapping_add(0x9c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011e42c;
            }
            0x8011e42c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e43c; continue; }
                pc = 0x8011e434;
            }
            0x8011e434 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e43c;
            }
            0x8011e43c => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e3fc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetTokuseiDataId (0x8011e444, 48 bytes)
#[inline(never)]
pub fn f_8011e444(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e444) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e444); p = true; }
    }
    f_8011e444_b(c, 0x8011e444);
    if p { crate::prof::exit(c, 0x8011e444); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e444_at(c: &mut Cpu, pc: u32) { f_8011e444_b(c, pc) }
#[inline(always)]
fn f_8011e444_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e444;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e444, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e444 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e454; continue; }
                pc = 0x8011e44c;
            }
            0x8011e44c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e454;
            }
            0x8011e454 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8011e468; continue; }
                pc = 0x8011e460;
            }
            0x8011e460 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e468;
            }
            0x8011e468 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x32));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e444: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetZokuseiDataId (0x8011e474, 48 bytes)
#[inline(never)]
pub fn f_8011e474(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e474) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e474); p = true; }
    }
    f_8011e474_b(c, 0x8011e474);
    if p { crate::prof::exit(c, 0x8011e474); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e474_at(c: &mut Cpu, pc: u32) { f_8011e474_b(c, pc) }
#[inline(always)]
fn f_8011e474_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e474;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e474, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e474 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e484; continue; }
                pc = 0x8011e47c;
            }
            0x8011e47c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e484;
            }
            0x8011e484 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8011e498; continue; }
                pc = 0x8011e490;
            }
            0x8011e490 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e498;
            }
            0x8011e498 => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x30));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e474: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetItemDataId (0x8011e4a4, 52 bytes)
#[inline(never)]
pub fn f_8011e4a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e4a4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e4a4); p = true; }
    }
    f_8011e4a4_b(c, 0x8011e4a4);
    if p { crate::prof::exit(c, 0x8011e4a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e4a4_at(c: &mut Cpu, pc: u32) { f_8011e4a4_b(c, pc) }
#[inline(always)]
fn f_8011e4a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e4a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e4a4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e4a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e4b4; continue; }
                pc = 0x8011e4ac;
            }
            0x8011e4ac => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e4b4;
            }
            0x8011e4b4 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8011e4c8; continue; }
                pc = 0x8011e4c0;
            }
            0x8011e4c0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e4c8;
            }
            0x8011e4c8 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x70));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e4a4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetInitFriend (0x8011e4d8, 24 bytes)
#[inline(never)]
pub fn f_8011e4d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e4d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e4d8); p = true; }
    }
    f_8011e4d8_b(c, 0x8011e4d8);
    if p { crate::prof::exit(c, 0x8011e4d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e4d8_at(c: &mut Cpu, pc: u32) { f_8011e4d8_b(c, pc) }
#[inline(always)]
fn f_8011e4d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e4d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e4d8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e4d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e4e8; continue; }
                pc = 0x8011e4e0;
            }
            0x8011e4e0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e4e8;
            }
            0x8011e4e8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e4d8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetSexRatio (0x8011e4f0, 24 bytes)
#[inline(never)]
pub fn f_8011e4f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e4f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e4f0); p = true; }
    }
    f_8011e4f0_b(c, 0x8011e4f0);
    if p { crate::prof::exit(c, 0x8011e4f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e4f0_at(c: &mut Cpu, pc: u32) { f_8011e4f0_b(c, pc) }
#[inline(always)]
fn f_8011e4f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e4f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e4f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e4f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e500; continue; }
                pc = 0x8011e4f8;
            }
            0x8011e4f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e500;
            }
            0x8011e500 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e4f0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGet (0x8011e508, 24 bytes)
#[inline(never)]
pub fn f_8011e508(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e508) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e508); p = true; }
    }
    f_8011e508_b(c, 0x8011e508);
    if p { crate::prof::exit(c, 0x8011e508); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e508_at(c: &mut Cpu, pc: u32) { f_8011e508_b(c, pc) }
#[inline(always)]
fn f_8011e508_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e508;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e508, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e508 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e518; continue; }
                pc = 0x8011e510;
            }
            0x8011e510 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e518;
            }
            0x8011e518 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e508: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGrowDataId (0x8011e520, 24 bytes)
#[inline(never)]
pub fn f_8011e520(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e520) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e520); p = true; }
    }
    f_8011e520_b(c, 0x8011e520);
    if p { crate::prof::exit(c, 0x8011e520); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e520_at(c: &mut Cpu, pc: u32) { f_8011e520_b(c, pc) }
#[inline(always)]
fn f_8011e520_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e520;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e520, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e520 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e530; continue; }
                pc = 0x8011e528;
            }
            0x8011e528 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e530;
            }
            0x8011e530 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e520: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGiveExp (0x8011e538, 24 bytes)
#[inline(never)]
pub fn f_8011e538(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e538) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e538); p = true; }
    }
    f_8011e538_b(c, 0x8011e538);
    if p { crate::prof::exit(c, 0x8011e538); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e538_at(c: &mut Cpu, pc: u32) { f_8011e538_b(c, pc) }
#[inline(always)]
fn f_8011e538_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e538;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e538, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e538 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e548; continue; }
                pc = 0x8011e540;
            }
            0x8011e540 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e548;
            }
            0x8011e548 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e538: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGiveNimblenessEffort (0x8011e550, 44 bytes)
#[inline(never)]
pub fn f_8011e550(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e550) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e550); p = true; }
    }
    f_8011e550_b(c, 0x8011e550);
    if p { crate::prof::exit(c, 0x8011e550); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e550_at(c: &mut Cpu, pc: u32) { f_8011e550_b(c, pc) }
#[inline(always)]
fn f_8011e550_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e550;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e550, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e550 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e560; continue; }
                pc = 0x8011e558;
            }
            0x8011e558 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e564; continue;
                pc = 0x8011e560;
            }
            0x8011e560 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011e564;
            }
            0x8011e564 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e574; continue; }
                pc = 0x8011e56c;
            }
            0x8011e56c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e574;
            }
            0x8011e574 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e550: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGiveSpeDefEffort (0x8011e57c, 44 bytes)
#[inline(never)]
pub fn f_8011e57c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e57c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e57c); p = true; }
    }
    f_8011e57c_b(c, 0x8011e57c);
    if p { crate::prof::exit(c, 0x8011e57c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e57c_at(c: &mut Cpu, pc: u32) { f_8011e57c_b(c, pc) }
#[inline(always)]
fn f_8011e57c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e57c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e57c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e57c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e58c; continue; }
                pc = 0x8011e584;
            }
            0x8011e584 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e590; continue;
                pc = 0x8011e58c;
            }
            0x8011e58c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011e590;
            }
            0x8011e590 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e5a0; continue; }
                pc = 0x8011e598;
            }
            0x8011e598 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e5a0;
            }
            0x8011e5a0 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e57c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGiveSpeAtkEffort (0x8011e5a8, 44 bytes)
#[inline(never)]
pub fn f_8011e5a8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e5a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e5a8); p = true; }
    }
    f_8011e5a8_b(c, 0x8011e5a8);
    if p { crate::prof::exit(c, 0x8011e5a8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e5a8_at(c: &mut Cpu, pc: u32) { f_8011e5a8_b(c, pc) }
#[inline(always)]
fn f_8011e5a8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e5a8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e5a8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e5a8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e5b8; continue; }
                pc = 0x8011e5b0;
            }
            0x8011e5b0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e5bc; continue;
                pc = 0x8011e5b8;
            }
            0x8011e5b8 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011e5bc;
            }
            0x8011e5bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e5cc; continue; }
                pc = 0x8011e5c4;
            }
            0x8011e5c4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e5cc;
            }
            0x8011e5cc => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e5a8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGivePhyDefEffort (0x8011e5d4, 44 bytes)
#[inline(never)]
pub fn f_8011e5d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e5d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e5d4); p = true; }
    }
    f_8011e5d4_b(c, 0x8011e5d4);
    if p { crate::prof::exit(c, 0x8011e5d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e5d4_at(c: &mut Cpu, pc: u32) { f_8011e5d4_b(c, pc) }
#[inline(always)]
fn f_8011e5d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e5d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e5d4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e5d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e5e4; continue; }
                pc = 0x8011e5dc;
            }
            0x8011e5dc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e5e8; continue;
                pc = 0x8011e5e4;
            }
            0x8011e5e4 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011e5e8;
            }
            0x8011e5e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e5f8; continue; }
                pc = 0x8011e5f0;
            }
            0x8011e5f0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e5f8;
            }
            0x8011e5f8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e5d4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGivePhyAtkEffort (0x8011e600, 44 bytes)
#[inline(never)]
pub fn f_8011e600(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e600) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e600); p = true; }
    }
    f_8011e600_b(c, 0x8011e600);
    if p { crate::prof::exit(c, 0x8011e600); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e600_at(c: &mut Cpu, pc: u32) { f_8011e600_b(c, pc) }
#[inline(always)]
fn f_8011e600_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e600;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e600, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e600 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e610; continue; }
                pc = 0x8011e608;
            }
            0x8011e608 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e614; continue;
                pc = 0x8011e610;
            }
            0x8011e610 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011e614;
            }
            0x8011e614 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e624; continue; }
                pc = 0x8011e61c;
            }
            0x8011e61c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e624;
            }
            0x8011e624 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e600: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetGiveMaxHpEffort (0x8011e62c, 44 bytes)
#[inline(never)]
pub fn f_8011e62c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e62c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e62c); p = true; }
    }
    f_8011e62c_b(c, 0x8011e62c);
    if p { crate::prof::exit(c, 0x8011e62c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e62c_at(c: &mut Cpu, pc: u32) { f_8011e62c_b(c, pc) }
#[inline(always)]
fn f_8011e62c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e62c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e62c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e62c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e63c; continue; }
                pc = 0x8011e634;
            }
            0x8011e634 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e640; continue;
                pc = 0x8011e63c;
            }
            0x8011e63c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011e640;
            }
            0x8011e640 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e650; continue; }
                pc = 0x8011e648;
            }
            0x8011e648 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e650;
            }
            0x8011e650 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e62c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetBasisNimbleness (0x8011e658, 44 bytes)
#[inline(never)]
pub fn f_8011e658(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e658) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e658); p = true; }
    }
    f_8011e658_b(c, 0x8011e658);
    if p { crate::prof::exit(c, 0x8011e658); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e658_at(c: &mut Cpu, pc: u32) { f_8011e658_b(c, pc) }
#[inline(always)]
fn f_8011e658_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e658;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e658, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e658 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e668; continue; }
                pc = 0x8011e660;
            }
            0x8011e660 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e66c; continue;
                pc = 0x8011e668;
            }
            0x8011e668 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011e66c;
            }
            0x8011e66c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e67c; continue; }
                pc = 0x8011e674;
            }
            0x8011e674 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e67c;
            }
            0x8011e67c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e658: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetBasisSpeDef (0x8011e684, 44 bytes)
#[inline(never)]
pub fn f_8011e684(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e684) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e684); p = true; }
    }
    f_8011e684_b(c, 0x8011e684);
    if p { crate::prof::exit(c, 0x8011e684); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e684_at(c: &mut Cpu, pc: u32) { f_8011e684_b(c, pc) }
#[inline(always)]
fn f_8011e684_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e684;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e684, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e684 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e694; continue; }
                pc = 0x8011e68c;
            }
            0x8011e68c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e698; continue;
                pc = 0x8011e694;
            }
            0x8011e694 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011e698;
            }
            0x8011e698 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e6a8; continue; }
                pc = 0x8011e6a0;
            }
            0x8011e6a0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e6a8;
            }
            0x8011e6a8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e684: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetBasisSpeAtk (0x8011e6b0, 44 bytes)
#[inline(never)]
pub fn f_8011e6b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e6b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e6b0); p = true; }
    }
    f_8011e6b0_b(c, 0x8011e6b0);
    if p { crate::prof::exit(c, 0x8011e6b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e6b0_at(c: &mut Cpu, pc: u32) { f_8011e6b0_b(c, pc) }
#[inline(always)]
fn f_8011e6b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e6b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e6b0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e6b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e6c0; continue; }
                pc = 0x8011e6b8;
            }
            0x8011e6b8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e6c4; continue;
                pc = 0x8011e6c0;
            }
            0x8011e6c0 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011e6c4;
            }
            0x8011e6c4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e6d4; continue; }
                pc = 0x8011e6cc;
            }
            0x8011e6cc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e6d4;
            }
            0x8011e6d4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e6b0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetBasisPhyDef (0x8011e6dc, 44 bytes)
#[inline(never)]
pub fn f_8011e6dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e6dc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e6dc); p = true; }
    }
    f_8011e6dc_b(c, 0x8011e6dc);
    if p { crate::prof::exit(c, 0x8011e6dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e6dc_at(c: &mut Cpu, pc: u32) { f_8011e6dc_b(c, pc) }
#[inline(always)]
fn f_8011e6dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e6dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e6dc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e6dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e6ec; continue; }
                pc = 0x8011e6e4;
            }
            0x8011e6e4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e6f0; continue;
                pc = 0x8011e6ec;
            }
            0x8011e6ec => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011e6f0;
            }
            0x8011e6f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e700; continue; }
                pc = 0x8011e6f8;
            }
            0x8011e6f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e700;
            }
            0x8011e700 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e6dc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetBasisPhyAtk (0x8011e708, 44 bytes)
#[inline(never)]
pub fn f_8011e708(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e708) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e708); p = true; }
    }
    f_8011e708_b(c, 0x8011e708);
    if p { crate::prof::exit(c, 0x8011e708); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e708_at(c: &mut Cpu, pc: u32) { f_8011e708_b(c, pc) }
#[inline(always)]
fn f_8011e708_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e708;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e708, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e708 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e718; continue; }
                pc = 0x8011e710;
            }
            0x8011e710 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e71c; continue;
                pc = 0x8011e718;
            }
            0x8011e718 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011e71c;
            }
            0x8011e71c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e72c; continue; }
                pc = 0x8011e724;
            }
            0x8011e724 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e72c;
            }
            0x8011e72c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e708: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetBasisMaxHp (0x8011e734, 44 bytes)
#[inline(never)]
pub fn f_8011e734(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e734) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e734); p = true; }
    }
    f_8011e734_b(c, 0x8011e734);
    if p { crate::prof::exit(c, 0x8011e734); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e734_at(c: &mut Cpu, pc: u32) { f_8011e734_b(c, pc) }
#[inline(always)]
fn f_8011e734_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e734;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e734, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e734 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e744; continue; }
                pc = 0x8011e73c;
            }
            0x8011e73c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e748; continue;
                pc = 0x8011e744;
            }
            0x8011e744 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011e748;
            }
            0x8011e748 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e758; continue; }
                pc = 0x8011e750;
            }
            0x8011e750 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e758;
            }
            0x8011e758 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e734: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetName (0x8011e760, 24 bytes)
#[inline(never)]
pub fn f_8011e760(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e760) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e760); p = true; }
    }
    f_8011e760_b(c, 0x8011e760);
    if p { crate::prof::exit(c, 0x8011e760); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e760_at(c: &mut Cpu, pc: u32) { f_8011e760_b(c, pc) }
#[inline(always)]
fn f_8011e760_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e760;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e760, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e760 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e770; continue; }
                pc = 0x8011e768;
            }
            0x8011e768 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e770;
            }
            0x8011e770 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x18));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e760: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosGetPtr (0x8011e778, 44 bytes)
#[inline(never)]
pub fn f_8011e778(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e778) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e778); p = true; }
    }
    f_8011e778_b(c, 0x8011e778);
    if p { crate::prof::exit(c, 0x8011e778); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e778_at(c: &mut Cpu, pc: u32) { f_8011e778_b(c, pc) }
#[inline(always)]
fn f_8011e778_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e778;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e778, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e778 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8770));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8011e794; continue; }
                pc = 0x8011e78c;
            }
            0x8011e78c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e794;
            }
            0x8011e794 => { ins += 6;
                c.r[0] = c.r[3].wrapping_mul(0x11c);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8774));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e778: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetEventGetFlag (0x8011e7a4, 28 bytes)
#[inline(never)]
pub fn f_8011e7a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e7a4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e7a4); p = true; }
    }
    f_8011e7a4_b(c, 0x8011e7a4);
    if p { crate::prof::exit(c, 0x8011e7a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e7a4_at(c: &mut Cpu, pc: u32) { f_8011e7a4_b(c, pc) }
#[inline(always)]
fn f_8011e7a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e7a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e7a4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e7a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e7b4; continue; }
                pc = 0x8011e7ac;
            }
            0x8011e7ac => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e7b4;
            }
            0x8011e7b4 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xf8));
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e7a4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetFightTrainerPokemonDataId (0x8011e7c0, 24 bytes)
#[inline(never)]
pub fn f_8011e7c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e7c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e7c0); p = true; }
    }
    f_8011e7c0_b(c, 0x8011e7c0);
    if p { crate::prof::exit(c, 0x8011e7c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e7c0_at(c: &mut Cpu, pc: u32) { f_8011e7c0_b(c, pc) }
#[inline(always)]
fn f_8011e7c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e7c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e7c0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e7c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e7d0; continue; }
                pc = 0x8011e7c8;
            }
            0x8011e7c8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e7d0;
            }
            0x8011e7d0 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xd6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e7c0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPara1Amari (0x8011e7d8, 24 bytes)
#[inline(never)]
pub fn f_8011e7d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e7d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e7d8); p = true; }
    }
    f_8011e7d8_b(c, 0x8011e7d8);
    if p { crate::prof::exit(c, 0x8011e7d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e7d8_at(c: &mut Cpu, pc: u32) { f_8011e7d8_b(c, pc) }
#[inline(always)]
fn f_8011e7d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e7d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e7d8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e7d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e7e8; continue; }
                pc = 0x8011e7e0;
            }
            0x8011e7e0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e7e8;
            }
            0x8011e7e8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xd2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e7d8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetAmari (0x8011e7f0, 24 bytes)
#[inline(never)]
pub fn f_8011e7f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e7f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e7f0); p = true; }
    }
    f_8011e7f0_b(c, 0x8011e7f0);
    if p { crate::prof::exit(c, 0x8011e7f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e7f0_at(c: &mut Cpu, pc: u32) { f_8011e7f0_b(c, pc) }
#[inline(always)]
fn f_8011e7f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e7f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e7f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e7f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e800; continue; }
                pc = 0x8011e7f8;
            }
            0x8011e7f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e800;
            }
            0x8011e800 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xd4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e7f0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetMailId (0x8011e808, 24 bytes)
#[inline(never)]
pub fn f_8011e808(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e808) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e808); p = true; }
    }
    f_8011e808_b(c, 0x8011e808);
    if p { crate::prof::exit(c, 0x8011e808); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e808_at(c: &mut Cpu, pc: u32) { f_8011e808_b(c, pc) }
#[inline(always)]
fn f_8011e808_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e808;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e808, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e808 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e818; continue; }
                pc = 0x8011e810;
            }
            0x8011e810 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e818;
            }
            0x8011e818 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xd0));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e808: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPcboxMark (0x8011e820, 24 bytes)
#[inline(never)]
pub fn f_8011e820(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e820) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e820); p = true; }
    }
    f_8011e820_b(c, 0x8011e820);
    if p { crate::prof::exit(c, 0x8011e820); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e820_at(c: &mut Cpu, pc: u32) { f_8011e820_b(c, pc) }
#[inline(always)]
fn f_8011e820_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e820;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e820, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e820 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e830; continue; }
                pc = 0x8011e828;
            }
            0x8011e828 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e830;
            }
            0x8011e830 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xcf));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e820: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetFlagAmari (0x8011e838, 24 bytes)
#[inline(never)]
pub fn f_8011e838(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e838) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e838); p = true; }
    }
    f_8011e838_b(c, 0x8011e838);
    if p { crate::prof::exit(c, 0x8011e838); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e838_at(c: &mut Cpu, pc: u32) { f_8011e838_b(c, pc) }
#[inline(always)]
fn f_8011e838_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e838;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e838, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e838 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e848; continue; }
                pc = 0x8011e840;
            }
            0x8011e840 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e848;
            }
            0x8011e848 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xce));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e838: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetFuseiFlag (0x8011e850, 24 bytes)
#[inline(never)]
pub fn f_8011e850(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e850) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e850); p = true; }
    }
    f_8011e850_b(c, 0x8011e850);
    if p { crate::prof::exit(c, 0x8011e850); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e850_at(c: &mut Cpu, pc: u32) { f_8011e850_b(c, pc) }
#[inline(always)]
fn f_8011e850_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e850;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e850, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e850 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e860; continue; }
                pc = 0x8011e858;
            }
            0x8011e858 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e860;
            }
            0x8011e860 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xcd));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e850: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetTokuseiFlag (0x8011e868, 116 bytes)
#[inline(never)]
pub fn f_8011e868(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e868) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e868); p = true; }
    }
    f_8011e868_b(c, 0x8011e868);
    if p { crate::prof::exit(c, 0x8011e868); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e868_at(c: &mut Cpu, pc: u32) { f_8011e868_b(c, pc) }
#[inline(always)]
fn f_8011e868_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e868;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e868, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e868 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e878; continue; }
                pc = 0x8011e870;
            }
            0x8011e870 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e878;
            }
            0x8011e878 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8011e884; continue; }
                pc = 0x8011e87c;
            }
            0x8011e87c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011e888; continue;
                pc = 0x8011e884;
            }
            0x8011e884 => { ins += 1;
                c.r[0] = c.lhz_t(ins, c.r[3]);
                pc = 0x8011e888;
            }
            0x8011e888 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8770));
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8011e8a4; continue; }
                pc = 0x8011e89c;
            }
            0x8011e89c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8011e8b0; continue;
                pc = 0x8011e8a4;
            }
            0x8011e8a4 => { ins += 5;
                c.r[0] = c.r[5].wrapping_mul(0x11c);
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8774));
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                pc = 0x8011e8b0;
            }
            0x8011e8b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e8c0; continue; }
                pc = 0x8011e8b8;
            }
            0x8011e8b8 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011e8c4; continue;
                pc = 0x8011e8c0;
            }
            0x8011e8c0 => { ins += 1;
                c.r[0] = c.lbz_t(ins, c.r[4].wrapping_add(0x33));
                pc = 0x8011e8c4;
            }
            0x8011e8c4 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011e8d4; continue; }
                pc = 0x8011e8cc;
            }
            0x8011e8cc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e8d4;
            }
            0x8011e8d4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xcc));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e868: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetTamagoFlag (0x8011e8dc, 24 bytes)
#[inline(never)]
pub fn f_8011e8dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e8dc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e8dc); p = true; }
    }
    f_8011e8dc_b(c, 0x8011e8dc);
    if p { crate::prof::exit(c, 0x8011e8dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e8dc_at(c: &mut Cpu, pc: u32) { f_8011e8dc_b(c, pc) }
#[inline(always)]
fn f_8011e8dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e8dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e8dc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e8dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e8ec; continue; }
                pc = 0x8011e8e4;
            }
            0x8011e8e4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e8ec;
            }
            0x8011e8ec => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xcb));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e8dc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPokerus (0x8011e8f4, 24 bytes)
#[inline(never)]
pub fn f_8011e8f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e8f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e8f4); p = true; }
    }
    f_8011e8f4_b(c, 0x8011e8f4);
    if p { crate::prof::exit(c, 0x8011e8f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e8f4_at(c: &mut Cpu, pc: u32) { f_8011e8f4_b(c, pc) }
#[inline(always)]
fn f_8011e8f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e8f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e8f4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e8f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e904; continue; }
                pc = 0x8011e8fc;
            }
            0x8011e8fc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e904;
            }
            0x8011e904 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xca));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e8f4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetAmariRibbon (0x8011e90c, 44 bytes)
#[inline(never)]
pub fn f_8011e90c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e90c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e90c); p = true; }
    }
    f_8011e90c_b(c, 0x8011e90c);
    if p { crate::prof::exit(c, 0x8011e90c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e90c_at(c: &mut Cpu, pc: u32) { f_8011e90c_b(c, pc) }
#[inline(always)]
fn f_8011e90c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e90c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e90c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e90c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e91c; continue; }
                pc = 0x8011e914;
            }
            0x8011e914 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e920; continue;
                pc = 0x8011e91c;
            }
            0x8011e91c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011e920;
            }
            0x8011e920 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e930; continue; }
                pc = 0x8011e928;
            }
            0x8011e928 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e930;
            }
            0x8011e930 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e90c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetWorldRibbon (0x8011e938, 44 bytes)
#[inline(never)]
pub fn f_8011e938(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e938) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e938); p = true; }
    }
    f_8011e938_b(c, 0x8011e938);
    if p { crate::prof::exit(c, 0x8011e938); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e938_at(c: &mut Cpu, pc: u32) { f_8011e938_b(c, pc) }
#[inline(always)]
fn f_8011e938_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e938;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e938, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e938 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e948; continue; }
                pc = 0x8011e940;
            }
            0x8011e940 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e94c; continue;
                pc = 0x8011e948;
            }
            0x8011e948 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011e94c;
            }
            0x8011e94c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e95c; continue; }
                pc = 0x8011e954;
            }
            0x8011e954 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e95c;
            }
            0x8011e95c => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xb));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e938: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetEarthRibbon (0x8011e964, 44 bytes)
#[inline(never)]
pub fn f_8011e964(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e964) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e964); p = true; }
    }
    f_8011e964_b(c, 0x8011e964);
    if p { crate::prof::exit(c, 0x8011e964); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e964_at(c: &mut Cpu, pc: u32) { f_8011e964_b(c, pc) }
#[inline(always)]
fn f_8011e964_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e964;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e964, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e964 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e974; continue; }
                pc = 0x8011e96c;
            }
            0x8011e96c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e978; continue;
                pc = 0x8011e974;
            }
            0x8011e974 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011e978;
            }
            0x8011e978 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e988; continue; }
                pc = 0x8011e980;
            }
            0x8011e980 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e988;
            }
            0x8011e988 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e964: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetNationalRibbon (0x8011e990, 44 bytes)
#[inline(never)]
pub fn f_8011e990(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e990) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e990); p = true; }
    }
    f_8011e990_b(c, 0x8011e990);
    if p { crate::prof::exit(c, 0x8011e990); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e990_at(c: &mut Cpu, pc: u32) { f_8011e990_b(c, pc) }
#[inline(always)]
fn f_8011e990_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e990;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e990, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e990 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e9a0; continue; }
                pc = 0x8011e998;
            }
            0x8011e998 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e9a4; continue;
                pc = 0x8011e9a0;
            }
            0x8011e9a0 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011e9a4;
            }
            0x8011e9a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e9b4; continue; }
                pc = 0x8011e9ac;
            }
            0x8011e9ac => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e9b4;
            }
            0x8011e9b4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x9));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e990: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetCountryRibbon (0x8011e9bc, 44 bytes)
#[inline(never)]
pub fn f_8011e9bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e9bc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e9bc); p = true; }
    }
    f_8011e9bc_b(c, 0x8011e9bc);
    if p { crate::prof::exit(c, 0x8011e9bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e9bc_at(c: &mut Cpu, pc: u32) { f_8011e9bc_b(c, pc) }
#[inline(always)]
fn f_8011e9bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e9bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e9bc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e9bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e9cc; continue; }
                pc = 0x8011e9c4;
            }
            0x8011e9c4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e9d0; continue;
                pc = 0x8011e9cc;
            }
            0x8011e9cc => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011e9d0;
            }
            0x8011e9d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e9e0; continue; }
                pc = 0x8011e9d8;
            }
            0x8011e9d8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011e9e0;
            }
            0x8011e9e0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e9bc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetSkyRibbon (0x8011e9e8, 44 bytes)
#[inline(never)]
pub fn f_8011e9e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011e9e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011e9e8); p = true; }
    }
    f_8011e9e8_b(c, 0x8011e9e8);
    if p { crate::prof::exit(c, 0x8011e9e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011e9e8_at(c: &mut Cpu, pc: u32) { f_8011e9e8_b(c, pc) }
#[inline(always)]
fn f_8011e9e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011e9e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011e9e8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011e9e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011e9f8; continue; }
                pc = 0x8011e9f0;
            }
            0x8011e9f0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011e9fc; continue;
                pc = 0x8011e9f8;
            }
            0x8011e9f8 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011e9fc;
            }
            0x8011e9fc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ea0c; continue; }
                pc = 0x8011ea04;
            }
            0x8011ea04 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ea0c;
            }
            0x8011ea0c => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x7));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011e9e8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetLandRibbon (0x8011ea14, 44 bytes)
#[inline(never)]
pub fn f_8011ea14(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ea14) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ea14); p = true; }
    }
    f_8011ea14_b(c, 0x8011ea14);
    if p { crate::prof::exit(c, 0x8011ea14); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ea14_at(c: &mut Cpu, pc: u32) { f_8011ea14_b(c, pc) }
#[inline(always)]
fn f_8011ea14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ea14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ea14, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ea14 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ea24; continue; }
                pc = 0x8011ea1c;
            }
            0x8011ea1c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ea28; continue;
                pc = 0x8011ea24;
            }
            0x8011ea24 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011ea28;
            }
            0x8011ea28 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ea38; continue; }
                pc = 0x8011ea30;
            }
            0x8011ea30 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ea38;
            }
            0x8011ea38 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ea14: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetMarineRibbon (0x8011ea40, 44 bytes)
#[inline(never)]
pub fn f_8011ea40(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ea40) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ea40); p = true; }
    }
    f_8011ea40_b(c, 0x8011ea40);
    if p { crate::prof::exit(c, 0x8011ea40); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ea40_at(c: &mut Cpu, pc: u32) { f_8011ea40_b(c, pc) }
#[inline(always)]
fn f_8011ea40_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ea40;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ea40, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ea40 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ea50; continue; }
                pc = 0x8011ea48;
            }
            0x8011ea48 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ea54; continue;
                pc = 0x8011ea50;
            }
            0x8011ea50 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011ea54;
            }
            0x8011ea54 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ea64; continue; }
                pc = 0x8011ea5c;
            }
            0x8011ea5c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ea64;
            }
            0x8011ea64 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x5));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ea40: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetGanbaRibbon (0x8011ea6c, 44 bytes)
#[inline(never)]
pub fn f_8011ea6c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ea6c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ea6c); p = true; }
    }
    f_8011ea6c_b(c, 0x8011ea6c);
    if p { crate::prof::exit(c, 0x8011ea6c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ea6c_at(c: &mut Cpu, pc: u32) { f_8011ea6c_b(c, pc) }
#[inline(always)]
fn f_8011ea6c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ea6c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ea6c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ea6c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ea7c; continue; }
                pc = 0x8011ea74;
            }
            0x8011ea74 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ea80; continue;
                pc = 0x8011ea7c;
            }
            0x8011ea7c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011ea80;
            }
            0x8011ea80 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ea90; continue; }
                pc = 0x8011ea88;
            }
            0x8011ea88 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ea90;
            }
            0x8011ea90 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ea6c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetBromideRibbon (0x8011ea98, 44 bytes)
#[inline(never)]
pub fn f_8011ea98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ea98) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ea98); p = true; }
    }
    f_8011ea98_b(c, 0x8011ea98);
    if p { crate::prof::exit(c, 0x8011ea98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ea98_at(c: &mut Cpu, pc: u32) { f_8011ea98_b(c, pc) }
#[inline(always)]
fn f_8011ea98_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ea98;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ea98, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ea98 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eaa8; continue; }
                pc = 0x8011eaa0;
            }
            0x8011eaa0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011eaac; continue;
                pc = 0x8011eaa8;
            }
            0x8011eaa8 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011eaac;
            }
            0x8011eaac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eabc; continue; }
                pc = 0x8011eab4;
            }
            0x8011eab4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011eabc;
            }
            0x8011eabc => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x3));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ea98: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetVictoryRibbon (0x8011eac4, 44 bytes)
#[inline(never)]
pub fn f_8011eac4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011eac4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011eac4); p = true; }
    }
    f_8011eac4_b(c, 0x8011eac4);
    if p { crate::prof::exit(c, 0x8011eac4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011eac4_at(c: &mut Cpu, pc: u32) { f_8011eac4_b(c, pc) }
#[inline(always)]
fn f_8011eac4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011eac4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011eac4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011eac4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ead4; continue; }
                pc = 0x8011eacc;
            }
            0x8011eacc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ead8; continue;
                pc = 0x8011ead4;
            }
            0x8011ead4 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011ead8;
            }
            0x8011ead8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eae8; continue; }
                pc = 0x8011eae0;
            }
            0x8011eae0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011eae8;
            }
            0x8011eae8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011eac4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetWinningRibbon (0x8011eaf0, 44 bytes)
#[inline(never)]
pub fn f_8011eaf0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011eaf0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011eaf0); p = true; }
    }
    f_8011eaf0_b(c, 0x8011eaf0);
    if p { crate::prof::exit(c, 0x8011eaf0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011eaf0_at(c: &mut Cpu, pc: u32) { f_8011eaf0_b(c, pc) }
#[inline(always)]
fn f_8011eaf0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011eaf0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011eaf0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011eaf0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eb00; continue; }
                pc = 0x8011eaf8;
            }
            0x8011eaf8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011eb04; continue;
                pc = 0x8011eb00;
            }
            0x8011eb00 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011eb04;
            }
            0x8011eb04 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eb14; continue; }
                pc = 0x8011eb0c;
            }
            0x8011eb0c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011eb14;
            }
            0x8011eb14 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011eaf0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetChampRibbon (0x8011eb1c, 44 bytes)
#[inline(never)]
pub fn f_8011eb1c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011eb1c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011eb1c); p = true; }
    }
    f_8011eb1c_b(c, 0x8011eb1c);
    if p { crate::prof::exit(c, 0x8011eb1c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011eb1c_at(c: &mut Cpu, pc: u32) { f_8011eb1c_b(c, pc) }
#[inline(always)]
fn f_8011eb1c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011eb1c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011eb1c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011eb1c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eb2c; continue; }
                pc = 0x8011eb24;
            }
            0x8011eb24 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011eb30; continue;
                pc = 0x8011eb2c;
            }
            0x8011eb2c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011eb30;
            }
            0x8011eb30 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eb40; continue; }
                pc = 0x8011eb38;
            }
            0x8011eb38 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011eb40;
            }
            0x8011eb40 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011eb1c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetFur (0x8011eb48, 24 bytes)
#[inline(never)]
pub fn f_8011eb48(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011eb48) { return; }
        if c.prof { crate::prof::enter(c, 0x8011eb48); p = true; }
    }
    f_8011eb48_b(c, 0x8011eb48);
    if p { crate::prof::exit(c, 0x8011eb48); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011eb48_at(c: &mut Cpu, pc: u32) { f_8011eb48_b(c, pc) }
#[inline(always)]
fn f_8011eb48_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011eb48;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011eb48, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011eb48 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eb58; continue; }
                pc = 0x8011eb50;
            }
            0x8011eb50 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011eb58;
            }
            0x8011eb58 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xbc));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011eb48: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetStrongMedal (0x8011eb60, 44 bytes)
#[inline(never)]
pub fn f_8011eb60(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011eb60) { return; }
        if c.prof { crate::prof::enter(c, 0x8011eb60); p = true; }
    }
    f_8011eb60_b(c, 0x8011eb60);
    if p { crate::prof::exit(c, 0x8011eb60); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011eb60_at(c: &mut Cpu, pc: u32) { f_8011eb60_b(c, pc) }
#[inline(always)]
fn f_8011eb60_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011eb60;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011eb60, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011eb60 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eb70; continue; }
                pc = 0x8011eb68;
            }
            0x8011eb68 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011eb74; continue;
                pc = 0x8011eb70;
            }
            0x8011eb70 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xb7);
                pc = 0x8011eb74;
            }
            0x8011eb74 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eb84; continue; }
                pc = 0x8011eb7c;
            }
            0x8011eb7c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011eb84;
            }
            0x8011eb84 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011eb60: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetCleverMedal (0x8011eb8c, 44 bytes)
#[inline(never)]
pub fn f_8011eb8c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011eb8c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011eb8c); p = true; }
    }
    f_8011eb8c_b(c, 0x8011eb8c);
    if p { crate::prof::exit(c, 0x8011eb8c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011eb8c_at(c: &mut Cpu, pc: u32) { f_8011eb8c_b(c, pc) }
#[inline(always)]
fn f_8011eb8c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011eb8c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011eb8c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011eb8c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eb9c; continue; }
                pc = 0x8011eb94;
            }
            0x8011eb94 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011eba0; continue;
                pc = 0x8011eb9c;
            }
            0x8011eb9c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xb7);
                pc = 0x8011eba0;
            }
            0x8011eba0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ebb0; continue; }
                pc = 0x8011eba8;
            }
            0x8011eba8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ebb0;
            }
            0x8011ebb0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x3));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011eb8c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetCuteMedal (0x8011ebb8, 44 bytes)
#[inline(never)]
pub fn f_8011ebb8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ebb8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ebb8); p = true; }
    }
    f_8011ebb8_b(c, 0x8011ebb8);
    if p { crate::prof::exit(c, 0x8011ebb8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ebb8_at(c: &mut Cpu, pc: u32) { f_8011ebb8_b(c, pc) }
#[inline(always)]
fn f_8011ebb8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ebb8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ebb8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ebb8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ebc8; continue; }
                pc = 0x8011ebc0;
            }
            0x8011ebc0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ebcc; continue;
                pc = 0x8011ebc8;
            }
            0x8011ebc8 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xb7);
                pc = 0x8011ebcc;
            }
            0x8011ebcc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ebdc; continue; }
                pc = 0x8011ebd4;
            }
            0x8011ebd4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ebdc;
            }
            0x8011ebdc => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ebb8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetBeautifulMedal (0x8011ebe4, 44 bytes)
#[inline(never)]
pub fn f_8011ebe4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ebe4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ebe4); p = true; }
    }
    f_8011ebe4_b(c, 0x8011ebe4);
    if p { crate::prof::exit(c, 0x8011ebe4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ebe4_at(c: &mut Cpu, pc: u32) { f_8011ebe4_b(c, pc) }
#[inline(always)]
fn f_8011ebe4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ebe4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ebe4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ebe4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ebf4; continue; }
                pc = 0x8011ebec;
            }
            0x8011ebec => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ebf8; continue;
                pc = 0x8011ebf4;
            }
            0x8011ebf4 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xb7);
                pc = 0x8011ebf8;
            }
            0x8011ebf8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ec08; continue; }
                pc = 0x8011ec00;
            }
            0x8011ec00 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ec08;
            }
            0x8011ec08 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ebe4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetStyleMedal (0x8011ec10, 44 bytes)
#[inline(never)]
pub fn f_8011ec10(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ec10) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ec10); p = true; }
    }
    f_8011ec10_b(c, 0x8011ec10);
    if p { crate::prof::exit(c, 0x8011ec10); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ec10_at(c: &mut Cpu, pc: u32) { f_8011ec10_b(c, pc) }
#[inline(always)]
fn f_8011ec10_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ec10;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ec10, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ec10 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ec20; continue; }
                pc = 0x8011ec18;
            }
            0x8011ec18 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ec24; continue;
                pc = 0x8011ec20;
            }
            0x8011ec20 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xb7);
                pc = 0x8011ec24;
            }
            0x8011ec24 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ec34; continue; }
                pc = 0x8011ec2c;
            }
            0x8011ec2c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ec34;
            }
            0x8011ec34 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ec10: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetStrong (0x8011ec3c, 44 bytes)
#[inline(never)]
pub fn f_8011ec3c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ec3c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ec3c); p = true; }
    }
    f_8011ec3c_b(c, 0x8011ec3c);
    if p { crate::prof::exit(c, 0x8011ec3c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ec3c_at(c: &mut Cpu, pc: u32) { f_8011ec3c_b(c, pc) }
#[inline(always)]
fn f_8011ec3c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ec3c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ec3c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ec3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ec4c; continue; }
                pc = 0x8011ec44;
            }
            0x8011ec44 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ec50; continue;
                pc = 0x8011ec4c;
            }
            0x8011ec4c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xb2);
                pc = 0x8011ec50;
            }
            0x8011ec50 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ec60; continue; }
                pc = 0x8011ec58;
            }
            0x8011ec58 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ec60;
            }
            0x8011ec60 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ec3c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetClever (0x8011ec68, 44 bytes)
#[inline(never)]
pub fn f_8011ec68(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ec68) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ec68); p = true; }
    }
    f_8011ec68_b(c, 0x8011ec68);
    if p { crate::prof::exit(c, 0x8011ec68); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ec68_at(c: &mut Cpu, pc: u32) { f_8011ec68_b(c, pc) }
#[inline(always)]
fn f_8011ec68_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ec68;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ec68, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ec68 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ec78; continue; }
                pc = 0x8011ec70;
            }
            0x8011ec70 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ec7c; continue;
                pc = 0x8011ec78;
            }
            0x8011ec78 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xb2);
                pc = 0x8011ec7c;
            }
            0x8011ec7c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ec8c; continue; }
                pc = 0x8011ec84;
            }
            0x8011ec84 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ec8c;
            }
            0x8011ec8c => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x3));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ec68: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetCute (0x8011ec94, 44 bytes)
#[inline(never)]
pub fn f_8011ec94(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ec94) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ec94); p = true; }
    }
    f_8011ec94_b(c, 0x8011ec94);
    if p { crate::prof::exit(c, 0x8011ec94); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ec94_at(c: &mut Cpu, pc: u32) { f_8011ec94_b(c, pc) }
#[inline(always)]
fn f_8011ec94_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ec94;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ec94, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ec94 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eca4; continue; }
                pc = 0x8011ec9c;
            }
            0x8011ec9c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011eca8; continue;
                pc = 0x8011eca4;
            }
            0x8011eca4 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xb2);
                pc = 0x8011eca8;
            }
            0x8011eca8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ecb8; continue; }
                pc = 0x8011ecb0;
            }
            0x8011ecb0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ecb8;
            }
            0x8011ecb8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ec94: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetBeautiful (0x8011ecc0, 44 bytes)
#[inline(never)]
pub fn f_8011ecc0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ecc0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ecc0); p = true; }
    }
    f_8011ecc0_b(c, 0x8011ecc0);
    if p { crate::prof::exit(c, 0x8011ecc0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ecc0_at(c: &mut Cpu, pc: u32) { f_8011ecc0_b(c, pc) }
#[inline(always)]
fn f_8011ecc0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ecc0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ecc0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ecc0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ecd0; continue; }
                pc = 0x8011ecc8;
            }
            0x8011ecc8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ecd4; continue;
                pc = 0x8011ecd0;
            }
            0x8011ecd0 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xb2);
                pc = 0x8011ecd4;
            }
            0x8011ecd4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ece4; continue; }
                pc = 0x8011ecdc;
            }
            0x8011ecdc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ece4;
            }
            0x8011ece4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ecc0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetStyle (0x8011ecec, 44 bytes)
#[inline(never)]
pub fn f_8011ecec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ecec) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ecec); p = true; }
    }
    f_8011ecec_b(c, 0x8011ecec);
    if p { crate::prof::exit(c, 0x8011ecec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ecec_at(c: &mut Cpu, pc: u32) { f_8011ecec_b(c, pc) }
#[inline(always)]
fn f_8011ecec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ecec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ecec, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ecec => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ecfc; continue; }
                pc = 0x8011ecf4;
            }
            0x8011ecf4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ed00; continue;
                pc = 0x8011ecfc;
            }
            0x8011ecfc => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xb2);
                pc = 0x8011ed00;
            }
            0x8011ed00 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ed10; continue; }
                pc = 0x8011ed08;
            }
            0x8011ed08 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ed10;
            }
            0x8011ed10 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ecec: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetInitDp (0x8011ed18, 80 bytes)
#[inline(never)]
pub fn f_8011ed18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ed18) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ed18); p = true; }
    }
    f_8011ed18_b(c, 0x8011ed18);
    if p { crate::prof::exit(c, 0x8011ed18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ed18_at(c: &mut Cpu, pc: u32) { f_8011ed18_b(c, pc) }
#[inline(always)]
fn f_8011ed18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ed18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ed18, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ed18 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011ed34; continue; }
                pc = 0x8011ed2c;
            }
            0x8011ed2c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ed58; continue;
                pc = 0x8011ed34;
            }
            0x8011ed34 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8011ed40; continue; }
                pc = 0x8011ed38;
            }
            0x8011ed38 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ed44; continue;
                pc = 0x8011ed40;
            }
            0x8011ed40 => { ins += 1;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xd8));
                pc = 0x8011ed44;
            }
            0x8011ed44 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011ed54; continue; }
                pc = 0x8011ed4c;
            }
            0x8011ed4c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ed58; continue;
                pc = 0x8011ed54;
            }
            0x8011ed54 => { ins += 1;
                c.lr = 0x8011ed58; { c.steps = st; c.cycles = ins; c.cr = cr; f_801eeeb8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011ed58;
            }
            0x8011ed58 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ed18: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetDarkFlag (0x8011ed68, 92 bytes)
#[inline(never)]
pub fn f_8011ed68(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ed68) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ed68); p = true; }
    }
    f_8011ed68_b(c, 0x8011ed68);
    if p { crate::prof::exit(c, 0x8011ed68); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ed68_at(c: &mut Cpu, pc: u32) { f_8011ed68_b(c, pc) }
#[inline(always)]
fn f_8011ed68_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ed68;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ed68, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ed68 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ed78; continue; }
                pc = 0x8011ed70;
            }
            0x8011ed70 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ed78;
            }
            0x8011ed78 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8011ed84; continue; }
                pc = 0x8011ed7c;
            }
            0x8011ed7c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011ed88; continue;
                pc = 0x8011ed84;
            }
            0x8011ed84 => { ins += 1;
                c.r[0] = c.lhz_t(ins, c.r[3].wrapping_add(0xd8));
                pc = 0x8011ed88;
            }
            0x8011ed88 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011ed98; continue; }
                pc = 0x8011ed90;
            }
            0x8011ed90 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ed98;
            }
            0x8011ed98 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eda8; continue; }
                pc = 0x8011eda0;
            }
            0x8011eda0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011edac; continue;
                pc = 0x8011eda8;
            }
            0x8011eda8 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xdc));
                pc = 0x8011edac;
            }
            0x8011edac => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8011edbc; continue; }
                pc = 0x8011edb4;
            }
            0x8011edb4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011edbc;
            }
            0x8011edbc => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ed68: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetDarkJoutaiPtr (0x8011edc4, 52 bytes)
#[inline(never)]
pub fn f_8011edc4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011edc4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011edc4); p = true; }
    }
    f_8011edc4_b(c, 0x8011edc4);
    if p { crate::prof::exit(c, 0x8011edc4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011edc4_at(c: &mut Cpu, pc: u32) { f_8011edc4_b(c, pc) }
#[inline(always)]
fn f_8011edc4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011edc4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011edc4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011edc4 => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x8011edd4; continue; }
                pc = 0x8011edcc;
            }
            0x8011edcc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011edd4;
            }
            0x8011edd4 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 0) { pc = 0x8011ede8; continue; }
                pc = 0x8011ede0;
            }
            0x8011ede0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ede8;
            }
            0x8011ede8 => { ins += 4;
                c.r[3] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(0xe8);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011edc4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPoolFriend (0x8011edf8, 24 bytes)
#[inline(never)]
pub fn f_8011edf8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011edf8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011edf8); p = true; }
    }
    f_8011edf8_b(c, 0x8011edf8);
    if p { crate::prof::exit(c, 0x8011edf8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011edf8_at(c: &mut Cpu, pc: u32) { f_8011edf8_b(c, pc) }
#[inline(always)]
fn f_8011edf8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011edf8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011edf8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011edf8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ee08; continue; }
                pc = 0x8011ee00;
            }
            0x8011ee00 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ee08;
            }
            0x8011ee08 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xe4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011edf8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPoolExp (0x8011ee10, 24 bytes)
#[inline(never)]
pub fn f_8011ee10(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ee10) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ee10); p = true; }
    }
    f_8011ee10_b(c, 0x8011ee10);
    if p { crate::prof::exit(c, 0x8011ee10); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ee10_at(c: &mut Cpu, pc: u32) { f_8011ee10_b(c, pc) }
#[inline(always)]
fn f_8011ee10_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ee10;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ee10, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ee10 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ee20; continue; }
                pc = 0x8011ee18;
            }
            0x8011ee18 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ee20;
            }
            0x8011ee20 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xe0));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ee10: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetDp (0x8011ee28, 24 bytes)
#[inline(never)]
pub fn f_8011ee28(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ee28) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ee28); p = true; }
    }
    f_8011ee28_b(c, 0x8011ee28);
    if p { crate::prof::exit(c, 0x8011ee28); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ee28_at(c: &mut Cpu, pc: u32) { f_8011ee28_b(c, pc) }
#[inline(always)]
fn f_8011ee28_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ee28;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ee28, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ee28 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ee38; continue; }
                pc = 0x8011ee30;
            }
            0x8011ee30 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ee38;
            }
            0x8011ee38 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xdc));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ee28: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetDarkpokemonDataId (0x8011ee40, 24 bytes)
#[inline(never)]
pub fn f_8011ee40(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ee40) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ee40); p = true; }
    }
    f_8011ee40_b(c, 0x8011ee40);
    if p { crate::prof::exit(c, 0x8011ee40); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ee40_at(c: &mut Cpu, pc: u32) { f_8011ee40_b(c, pc) }
#[inline(always)]
fn f_8011ee40_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ee40;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ee40, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ee40 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ee50; continue; }
                pc = 0x8011ee48;
            }
            0x8011ee48 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ee50;
            }
            0x8011ee50 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xd8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ee40: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetFriend (0x8011ee58, 24 bytes)
#[inline(never)]
pub fn f_8011ee58(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ee58) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ee58); p = true; }
    }
    f_8011ee58_b(c, 0x8011ee58);
    if p { crate::prof::exit(c, 0x8011ee58); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ee58_at(c: &mut Cpu, pc: u32) { f_8011ee58_b(c, pc) }
#[inline(always)]
fn f_8011ee58_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ee58;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ee58, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ee58 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ee68; continue; }
                pc = 0x8011ee60;
            }
            0x8011ee60 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ee68;
            }
            0x8011ee68 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xb0));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ee58: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetNimblenessRnd (0x8011ee70, 44 bytes)
#[inline(never)]
pub fn f_8011ee70(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ee70) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ee70); p = true; }
    }
    f_8011ee70_b(c, 0x8011ee70);
    if p { crate::prof::exit(c, 0x8011ee70); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ee70_at(c: &mut Cpu, pc: u32) { f_8011ee70_b(c, pc) }
#[inline(always)]
fn f_8011ee70_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ee70;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ee70, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ee70 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ee80; continue; }
                pc = 0x8011ee78;
            }
            0x8011ee78 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ee84; continue;
                pc = 0x8011ee80;
            }
            0x8011ee80 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011ee84;
            }
            0x8011ee84 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ee94; continue; }
                pc = 0x8011ee8c;
            }
            0x8011ee8c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ee94;
            }
            0x8011ee94 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ee70: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetSpeDefRnd (0x8011ee9c, 44 bytes)
#[inline(never)]
pub fn f_8011ee9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ee9c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ee9c); p = true; }
    }
    f_8011ee9c_b(c, 0x8011ee9c);
    if p { crate::prof::exit(c, 0x8011ee9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ee9c_at(c: &mut Cpu, pc: u32) { f_8011ee9c_b(c, pc) }
#[inline(always)]
fn f_8011ee9c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ee9c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ee9c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ee9c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eeac; continue; }
                pc = 0x8011eea4;
            }
            0x8011eea4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011eeb0; continue;
                pc = 0x8011eeac;
            }
            0x8011eeac => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011eeb0;
            }
            0x8011eeb0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eec0; continue; }
                pc = 0x8011eeb8;
            }
            0x8011eeb8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011eec0;
            }
            0x8011eec0 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ee9c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetSpeAtkRnd (0x8011eec8, 44 bytes)
#[inline(never)]
pub fn f_8011eec8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011eec8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011eec8); p = true; }
    }
    f_8011eec8_b(c, 0x8011eec8);
    if p { crate::prof::exit(c, 0x8011eec8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011eec8_at(c: &mut Cpu, pc: u32) { f_8011eec8_b(c, pc) }
#[inline(always)]
fn f_8011eec8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011eec8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011eec8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011eec8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eed8; continue; }
                pc = 0x8011eed0;
            }
            0x8011eed0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011eedc; continue;
                pc = 0x8011eed8;
            }
            0x8011eed8 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011eedc;
            }
            0x8011eedc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eeec; continue; }
                pc = 0x8011eee4;
            }
            0x8011eee4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011eeec;
            }
            0x8011eeec => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011eec8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPhyDefRnd (0x8011eef4, 44 bytes)
#[inline(never)]
pub fn f_8011eef4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011eef4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011eef4); p = true; }
    }
    f_8011eef4_b(c, 0x8011eef4);
    if p { crate::prof::exit(c, 0x8011eef4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011eef4_at(c: &mut Cpu, pc: u32) { f_8011eef4_b(c, pc) }
#[inline(always)]
fn f_8011eef4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011eef4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011eef4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011eef4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ef04; continue; }
                pc = 0x8011eefc;
            }
            0x8011eefc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ef08; continue;
                pc = 0x8011ef04;
            }
            0x8011ef04 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011ef08;
            }
            0x8011ef08 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ef18; continue; }
                pc = 0x8011ef10;
            }
            0x8011ef10 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ef18;
            }
            0x8011ef18 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011eef4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPhyAtkRnd (0x8011ef20, 44 bytes)
#[inline(never)]
pub fn f_8011ef20(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ef20) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ef20); p = true; }
    }
    f_8011ef20_b(c, 0x8011ef20);
    if p { crate::prof::exit(c, 0x8011ef20); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ef20_at(c: &mut Cpu, pc: u32) { f_8011ef20_b(c, pc) }
#[inline(always)]
fn f_8011ef20_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ef20;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ef20, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ef20 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ef30; continue; }
                pc = 0x8011ef28;
            }
            0x8011ef28 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ef34; continue;
                pc = 0x8011ef30;
            }
            0x8011ef30 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011ef34;
            }
            0x8011ef34 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ef44; continue; }
                pc = 0x8011ef3c;
            }
            0x8011ef3c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ef44;
            }
            0x8011ef44 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ef20: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetMaxHpRnd (0x8011ef4c, 44 bytes)
#[inline(never)]
pub fn f_8011ef4c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ef4c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ef4c); p = true; }
    }
    f_8011ef4c_b(c, 0x8011ef4c);
    if p { crate::prof::exit(c, 0x8011ef4c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ef4c_at(c: &mut Cpu, pc: u32) { f_8011ef4c_b(c, pc) }
#[inline(always)]
fn f_8011ef4c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ef4c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ef4c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ef4c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ef5c; continue; }
                pc = 0x8011ef54;
            }
            0x8011ef54 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ef60; continue;
                pc = 0x8011ef5c;
            }
            0x8011ef5c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011ef60;
            }
            0x8011ef60 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ef70; continue; }
                pc = 0x8011ef68;
            }
            0x8011ef68 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ef70;
            }
            0x8011ef70 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ef4c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetNimblenessEffort (0x8011ef78, 44 bytes)
#[inline(never)]
pub fn f_8011ef78(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ef78) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ef78); p = true; }
    }
    f_8011ef78_b(c, 0x8011ef78);
    if p { crate::prof::exit(c, 0x8011ef78); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ef78_at(c: &mut Cpu, pc: u32) { f_8011ef78_b(c, pc) }
#[inline(always)]
fn f_8011ef78_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011ef78;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011ef78, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011ef78 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ef88; continue; }
                pc = 0x8011ef80;
            }
            0x8011ef80 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ef8c; continue;
                pc = 0x8011ef88;
            }
            0x8011ef88 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011ef8c;
            }
            0x8011ef8c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ef9c; continue; }
                pc = 0x8011ef94;
            }
            0x8011ef94 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011ef9c;
            }
            0x8011ef9c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011ef78: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetSpeDefEffort (0x8011efa4, 44 bytes)
#[inline(never)]
pub fn f_8011efa4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011efa4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011efa4); p = true; }
    }
    f_8011efa4_b(c, 0x8011efa4);
    if p { crate::prof::exit(c, 0x8011efa4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011efa4_at(c: &mut Cpu, pc: u32) { f_8011efa4_b(c, pc) }
#[inline(always)]
fn f_8011efa4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011efa4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011efa4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011efa4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011efb4; continue; }
                pc = 0x8011efac;
            }
            0x8011efac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011efb8; continue;
                pc = 0x8011efb4;
            }
            0x8011efb4 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011efb8;
            }
            0x8011efb8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011efc8; continue; }
                pc = 0x8011efc0;
            }
            0x8011efc0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011efc8;
            }
            0x8011efc8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011efa4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetSpeAtkEffort (0x8011efd0, 44 bytes)
#[inline(never)]
pub fn f_8011efd0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011efd0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011efd0); p = true; }
    }
    f_8011efd0_b(c, 0x8011efd0);
    if p { crate::prof::exit(c, 0x8011efd0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011efd0_at(c: &mut Cpu, pc: u32) { f_8011efd0_b(c, pc) }
#[inline(always)]
fn f_8011efd0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011efd0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011efd0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011efd0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011efe0; continue; }
                pc = 0x8011efd8;
            }
            0x8011efd8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011efe4; continue;
                pc = 0x8011efe0;
            }
            0x8011efe0 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011efe4;
            }
            0x8011efe4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011eff4; continue; }
                pc = 0x8011efec;
            }
            0x8011efec => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011eff4;
            }
            0x8011eff4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011efd0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPhyDefEffort (0x8011effc, 44 bytes)
#[inline(never)]
pub fn f_8011effc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011effc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011effc); p = true; }
    }
    f_8011effc_b(c, 0x8011effc);
    if p { crate::prof::exit(c, 0x8011effc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011effc_at(c: &mut Cpu, pc: u32) { f_8011effc_b(c, pc) }
#[inline(always)]
fn f_8011effc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011effc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011effc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011effc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f00c; continue; }
                pc = 0x8011f004;
            }
            0x8011f004 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f010; continue;
                pc = 0x8011f00c;
            }
            0x8011f00c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011f010;
            }
            0x8011f010 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f020; continue; }
                pc = 0x8011f018;
            }
            0x8011f018 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f020;
            }
            0x8011f020 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011effc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPhyAtkEffort (0x8011f028, 44 bytes)
#[inline(never)]
pub fn f_8011f028(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f028) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f028); p = true; }
    }
    f_8011f028_b(c, 0x8011f028);
    if p { crate::prof::exit(c, 0x8011f028); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f028_at(c: &mut Cpu, pc: u32) { f_8011f028_b(c, pc) }
#[inline(always)]
fn f_8011f028_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f028;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f028, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f028 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f038; continue; }
                pc = 0x8011f030;
            }
            0x8011f030 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f03c; continue;
                pc = 0x8011f038;
            }
            0x8011f038 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011f03c;
            }
            0x8011f03c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f04c; continue; }
                pc = 0x8011f044;
            }
            0x8011f044 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f04c;
            }
            0x8011f04c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f028: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetMaxHpEffort (0x8011f054, 44 bytes)
#[inline(never)]
pub fn f_8011f054(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f054) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f054); p = true; }
    }
    f_8011f054_b(c, 0x8011f054);
    if p { crate::prof::exit(c, 0x8011f054); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f054_at(c: &mut Cpu, pc: u32) { f_8011f054_b(c, pc) }
#[inline(always)]
fn f_8011f054_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f054;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f054, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f054 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f064; continue; }
                pc = 0x8011f05c;
            }
            0x8011f05c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f068; continue;
                pc = 0x8011f064;
            }
            0x8011f064 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011f068;
            }
            0x8011f068 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f078; continue; }
                pc = 0x8011f070;
            }
            0x8011f070 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f078;
            }
            0x8011f078 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f054: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetNimbleness (0x8011f080, 44 bytes)
#[inline(never)]
pub fn f_8011f080(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f080) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f080); p = true; }
    }
    f_8011f080_b(c, 0x8011f080);
    if p { crate::prof::exit(c, 0x8011f080); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f080_at(c: &mut Cpu, pc: u32) { f_8011f080_b(c, pc) }
#[inline(always)]
fn f_8011f080_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f080;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f080, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f080 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f090; continue; }
                pc = 0x8011f088;
            }
            0x8011f088 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f094; continue;
                pc = 0x8011f090;
            }
            0x8011f090 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011f094;
            }
            0x8011f094 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f0a4; continue; }
                pc = 0x8011f09c;
            }
            0x8011f09c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f0a4;
            }
            0x8011f0a4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f080: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetSpeDef (0x8011f0ac, 44 bytes)
#[inline(never)]
pub fn f_8011f0ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f0ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f0ac); p = true; }
    }
    f_8011f0ac_b(c, 0x8011f0ac);
    if p { crate::prof::exit(c, 0x8011f0ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f0ac_at(c: &mut Cpu, pc: u32) { f_8011f0ac_b(c, pc) }
#[inline(always)]
fn f_8011f0ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f0ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f0ac, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f0ac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f0bc; continue; }
                pc = 0x8011f0b4;
            }
            0x8011f0b4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f0c0; continue;
                pc = 0x8011f0bc;
            }
            0x8011f0bc => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011f0c0;
            }
            0x8011f0c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f0d0; continue; }
                pc = 0x8011f0c8;
            }
            0x8011f0c8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f0d0;
            }
            0x8011f0d0 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f0ac: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetSpeAtk (0x8011f0d8, 44 bytes)
#[inline(never)]
pub fn f_8011f0d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f0d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f0d8); p = true; }
    }
    f_8011f0d8_b(c, 0x8011f0d8);
    if p { crate::prof::exit(c, 0x8011f0d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f0d8_at(c: &mut Cpu, pc: u32) { f_8011f0d8_b(c, pc) }
#[inline(always)]
fn f_8011f0d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f0d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f0d8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f0d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f0e8; continue; }
                pc = 0x8011f0e0;
            }
            0x8011f0e0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f0ec; continue;
                pc = 0x8011f0e8;
            }
            0x8011f0e8 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011f0ec;
            }
            0x8011f0ec => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f0fc; continue; }
                pc = 0x8011f0f4;
            }
            0x8011f0f4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f0fc;
            }
            0x8011f0fc => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f0d8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPhyDef (0x8011f104, 44 bytes)
#[inline(never)]
pub fn f_8011f104(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f104) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f104); p = true; }
    }
    f_8011f104_b(c, 0x8011f104);
    if p { crate::prof::exit(c, 0x8011f104); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f104_at(c: &mut Cpu, pc: u32) { f_8011f104_b(c, pc) }
#[inline(always)]
fn f_8011f104_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f104;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f104, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f104 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f114; continue; }
                pc = 0x8011f10c;
            }
            0x8011f10c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f118; continue;
                pc = 0x8011f114;
            }
            0x8011f114 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011f118;
            }
            0x8011f118 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f128; continue; }
                pc = 0x8011f120;
            }
            0x8011f120 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f128;
            }
            0x8011f128 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f104: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPhyAtk (0x8011f130, 44 bytes)
#[inline(never)]
pub fn f_8011f130(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f130) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f130); p = true; }
    }
    f_8011f130_b(c, 0x8011f130);
    if p { crate::prof::exit(c, 0x8011f130); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f130_at(c: &mut Cpu, pc: u32) { f_8011f130_b(c, pc) }
#[inline(always)]
fn f_8011f130_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f130;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f130, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f130 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f140; continue; }
                pc = 0x8011f138;
            }
            0x8011f138 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f144; continue;
                pc = 0x8011f140;
            }
            0x8011f140 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011f144;
            }
            0x8011f144 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f154; continue; }
                pc = 0x8011f14c;
            }
            0x8011f14c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f154;
            }
            0x8011f154 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f130: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetMaxHp (0x8011f15c, 44 bytes)
#[inline(never)]
pub fn f_8011f15c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f15c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f15c); p = true; }
    }
    f_8011f15c_b(c, 0x8011f15c);
    if p { crate::prof::exit(c, 0x8011f15c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f15c_at(c: &mut Cpu, pc: u32) { f_8011f15c_b(c, pc) }
#[inline(always)]
fn f_8011f15c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f15c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f15c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f15c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f16c; continue; }
                pc = 0x8011f164;
            }
            0x8011f164 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f170; continue;
                pc = 0x8011f16c;
            }
            0x8011f16c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011f170;
            }
            0x8011f170 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f180; continue; }
                pc = 0x8011f178;
            }
            0x8011f178 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f180;
            }
            0x8011f180 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f15c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetHp (0x8011f188, 24 bytes)
#[inline(never)]
pub fn f_8011f188(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f188) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f188); p = true; }
    }
    f_8011f188_b(c, 0x8011f188);
    if p { crate::prof::exit(c, 0x8011f188); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f188_at(c: &mut Cpu, pc: u32) { f_8011f188_b(c, pc) }
#[inline(always)]
fn f_8011f188_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f188;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f188, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f188 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f198; continue; }
                pc = 0x8011f190;
            }
            0x8011f190 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f198;
            }
            0x8011f198 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x8a));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f188: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetItemDataId (0x8011f1a0, 24 bytes)
#[inline(never)]
pub fn f_8011f1a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f1a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f1a0); p = true; }
    }
    f_8011f1a0_b(c, 0x8011f1a0);
    if p { crate::prof::exit(c, 0x8011f1a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f1a0_at(c: &mut Cpu, pc: u32) { f_8011f1a0_b(c, pc) }
#[inline(always)]
fn f_8011f1a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f1a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f1a0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f1a0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f1b0; continue; }
                pc = 0x8011f1a8;
            }
            0x8011f1a8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f1b0;
            }
            0x8011f1b0 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x88));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f1a0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPokemonWazaPpCount (0x8011f1b8, 56 bytes)
#[inline(never)]
pub fn f_8011f1b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f1b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f1b8); p = true; }
    }
    f_8011f1b8_b(c, 0x8011f1b8);
    if p { crate::prof::exit(c, 0x8011f1b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f1b8_at(c: &mut Cpu, pc: u32) { f_8011f1b8_b(c, pc) }
#[inline(always)]
fn f_8011f1b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f1b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f1b8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f1b8 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8011f1cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011f260(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f1cc;
            }
            0x8011f1cc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f1dc; continue; }
                pc = 0x8011f1d4;
            }
            0x8011f1d4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f1e0; continue;
                pc = 0x8011f1dc;
            }
            0x8011f1dc => { ins += 1;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x3));
                pc = 0x8011f1e0;
            }
            0x8011f1e0 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f1b8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPokemonWazaPp (0x8011f1f0, 56 bytes)
#[inline(never)]
pub fn f_8011f1f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f1f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f1f0); p = true; }
    }
    f_8011f1f0_b(c, 0x8011f1f0);
    if p { crate::prof::exit(c, 0x8011f1f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f1f0_at(c: &mut Cpu, pc: u32) { f_8011f1f0_b(c, pc) }
#[inline(always)]
fn f_8011f1f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f1f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f1f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f1f0 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8011f204; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011f260(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f204;
            }
            0x8011f204 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f214; continue; }
                pc = 0x8011f20c;
            }
            0x8011f20c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f218; continue;
                pc = 0x8011f214;
            }
            0x8011f214 => { ins += 1;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x2));
                pc = 0x8011f218;
            }
            0x8011f218 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f1f0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPokemonWazaDataId (0x8011f228, 56 bytes)
#[inline(never)]
pub fn f_8011f228(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f228) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f228); p = true; }
    }
    f_8011f228_b(c, 0x8011f228);
    if p { crate::prof::exit(c, 0x8011f228); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f228_at(c: &mut Cpu, pc: u32) { f_8011f228_b(c, pc) }
#[inline(always)]
fn f_8011f228_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f228;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f228, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f228 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8011f23c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011f260(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f23c;
            }
            0x8011f23c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f24c; continue; }
                pc = 0x8011f244;
            }
            0x8011f244 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f250; continue;
                pc = 0x8011f24c;
            }
            0x8011f24c => { ins += 1;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                pc = 0x8011f250;
            }
            0x8011f250 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f228: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPokemonWazaPtr (0x8011f260, 508 bytes)
#[inline(never)]
pub fn f_8011f260(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f260) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f260); p = true; }
    }
    f_8011f260_b(c, 0x8011f260);
    if p { crate::prof::exit(c, 0x8011f260); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f260_at(c: &mut Cpu, pc: u32) { f_8011f260_b(c, pc) }
#[inline(never)]
fn f_8011f260_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f260;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f260, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f260 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = 0x0;
                if !cr_bit(cr, 2) { pc = 0x8011f290; continue; }
                pc = 0x8011f288;
            }
            0x8011f288 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f440; continue;
                pc = 0x8011f290;
            }
            0x8011f290 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8011f29c; continue; }
                pc = 0x8011f294;
            }
            0x8011f294 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f2e4; continue;
                pc = 0x8011f29c;
            }
            0x8011f29c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8011f2a8; continue; }
                pc = 0x8011f2a0;
            }
            0x8011f2a0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f2ac; continue;
                pc = 0x8011f2a8;
            }
            0x8011f2a8 => { ins += 1;
                c.r[0] = c.lhz_t(ins, c.r[30].wrapping_add(0xd8));
                pc = 0x8011f2ac;
            }
            0x8011f2ac => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011f2bc; continue; }
                pc = 0x8011f2b4;
            }
            0x8011f2b4 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f2e4; continue;
                pc = 0x8011f2bc;
            }
            0x8011f2bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f2cc; continue; }
                pc = 0x8011f2c4;
            }
            0x8011f2c4 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f2d0; continue;
                pc = 0x8011f2cc;
            }
            0x8011f2cc => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0xdc));
                pc = 0x8011f2d0;
            }
            0x8011f2d0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8011f2e0; continue; }
                pc = 0x8011f2d8;
            }
            0x8011f2d8 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f2e4; continue;
                pc = 0x8011f2e0;
            }
            0x8011f2e0 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x8011f2e4;
            }
            0x8011f2e4 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8011f31c; continue; }
                pc = 0x8011f2f0;
            }
            0x8011f2f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f300; continue; }
                pc = 0x8011f2f8;
            }
            0x8011f2f8 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f304; continue;
                pc = 0x8011f300;
            }
            0x8011f300 => { ins += 1;
                c.r[0] = c.lhz_t(ins, c.r[30].wrapping_add(0xd6));
                pc = 0x8011f304;
            }
            0x8011f304 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011f31c; continue; }
                pc = 0x8011f30c;
            }
            0x8011f30c => { ins += 3;
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8011f31c; continue; }
                pc = 0x8011f318;
            }
            0x8011f318 => { ins += 1;
                c.r[29] = 0x1;
                pc = 0x8011f31c;
            }
            0x8011f31c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f32c; continue; }
                pc = 0x8011f324;
            }
            0x8011f324 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f374; continue;
                pc = 0x8011f32c;
            }
            0x8011f32c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8011f338; continue; }
                pc = 0x8011f330;
            }
            0x8011f330 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f33c; continue;
                pc = 0x8011f338;
            }
            0x8011f338 => { ins += 1;
                c.r[0] = c.lhz_t(ins, c.r[30].wrapping_add(0xd8));
                pc = 0x8011f33c;
            }
            0x8011f33c => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011f34c; continue; }
                pc = 0x8011f344;
            }
            0x8011f344 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f374; continue;
                pc = 0x8011f34c;
            }
            0x8011f34c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f35c; continue; }
                pc = 0x8011f354;
            }
            0x8011f354 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f360; continue;
                pc = 0x8011f35c;
            }
            0x8011f35c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0xdc));
                pc = 0x8011f360;
            }
            0x8011f360 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8011f370; continue; }
                pc = 0x8011f368;
            }
            0x8011f368 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8011f374; continue;
                pc = 0x8011f370;
            }
            0x8011f370 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x8011f374;
            }
            0x8011f374 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8011f398; continue; }
                pc = 0x8011f380;
            }
            0x8011f380 => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011f398; continue; }
                pc = 0x8011f388;
            }
            0x8011f388 => { ins += 3;
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8011f398; continue; }
                pc = 0x8011f394;
            }
            0x8011f394 => { ins += 1;
                c.r[29] = 0x1;
                pc = 0x8011f398;
            }
            0x8011f398 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011f3a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011f77c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f3a0;
            }
            0x8011f3a0 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8011f410; continue; }
                pc = 0x8011f3ac;
            }
            0x8011f3ac => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011f3bc; continue; }
                pc = 0x8011f3b4;
            }
            0x8011f3b4 => { ins += 2;
                c.r[3] = c.r[13].wrapping_add(0xffff8338);
                pc = 0x8011f440; continue;
                pc = 0x8011f3bc;
            }
            0x8011f3bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8011f3d8; continue; }
                pc = 0x8011f3c4;
            }
            0x8011f3c4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 0) { pc = 0x8011f410; continue; }
                pc = 0x8011f3d0;
            }
            0x8011f3d0 => { ins += 2;
                c.r[3] = c.r[13].wrapping_add(0xffff833c);
                pc = 0x8011f440; continue;
                pc = 0x8011f3d8;
            }
            0x8011f3d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x8011f3f4; continue; }
                pc = 0x8011f3e0;
            }
            0x8011f3e0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 0) { pc = 0x8011f410; continue; }
                pc = 0x8011f3ec;
            }
            0x8011f3ec => { ins += 2;
                c.r[3] = c.r[13].wrapping_add(0xffff833c);
                pc = 0x8011f440; continue;
                pc = 0x8011f3f4;
            }
            0x8011f3f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x8011f410; continue; }
                pc = 0x8011f3fc;
            }
            0x8011f3fc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5);
                if !cr_bit(cr, 0) { pc = 0x8011f410; continue; }
                pc = 0x8011f408;
            }
            0x8011f408 => { ins += 2;
                c.r[3] = c.r[13].wrapping_add(0xffff833c);
                pc = 0x8011f440; continue;
                pc = 0x8011f410;
            }
            0x8011f410 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x8011f434; continue; }
                pc = 0x8011f41c;
            }
            0x8011f41c => { ins += 4;
                c.r[31] = c.r[31].wrapping_add(0xfffffffc);
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x8011f434; continue; }
                pc = 0x8011f42c;
            }
            0x8011f42c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f440; continue;
                pc = 0x8011f434;
            }
            0x8011f434 => { ins += 3;
                c.r[3] = c.r[31].rotate_left(2) & 0x3fffc;
                c.r[3] = c.r[3].wrapping_add(0x78);
                c.r[3] = c.r[30].wrapping_add(c.r[3]);
                pc = 0x8011f440;
            }
            0x8011f440 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f260: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetConditionAmari (0x8011f45c, 24 bytes)
#[inline(never)]
pub fn f_8011f45c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f45c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f45c); p = true; }
    }
    f_8011f45c_b(c, 0x8011f45c);
    if p { crate::prof::exit(c, 0x8011f45c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f45c_at(c: &mut Cpu, pc: u32) { f_8011f45c_b(c, pc) }
#[inline(always)]
fn f_8011f45c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f45c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f45c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f45c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f46c; continue; }
                pc = 0x8011f464;
            }
            0x8011f464 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f46c;
            }
            0x8011f46c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x74));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f45c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetJoutaiPtr (0x8011f474, 52 bytes)
#[inline(never)]
pub fn f_8011f474(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f474) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f474); p = true; }
    }
    f_8011f474_b(c, 0x8011f474);
    if p { crate::prof::exit(c, 0x8011f474); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f474_at(c: &mut Cpu, pc: u32) { f_8011f474_b(c, pc) }
#[inline(always)]
fn f_8011f474_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f474;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f474, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f474 => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x8011f484; continue; }
                pc = 0x8011f47c;
            }
            0x8011f47c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f484;
            }
            0x8011f484 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 0) { pc = 0x8011f498; continue; }
                pc = 0x8011f490;
            }
            0x8011f490 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f498;
            }
            0x8011f498 => { ins += 4;
                c.r[3] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(0x64);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f474: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetLevel (0x8011f4a8, 24 bytes)
#[inline(never)]
pub fn f_8011f4a8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f4a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f4a8); p = true; }
    }
    f_8011f4a8_b(c, 0x8011f4a8);
    if p { crate::prof::exit(c, 0x8011f4a8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f4a8_at(c: &mut Cpu, pc: u32) { f_8011f4a8_b(c, pc) }
#[inline(always)]
fn f_8011f4a8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f4a8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f4a8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f4a8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f4b8; continue; }
                pc = 0x8011f4b0;
            }
            0x8011f4b0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f4b8;
            }
            0x8011f4b8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x60));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f4a8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetExp (0x8011f4c0, 24 bytes)
#[inline(never)]
pub fn f_8011f4c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f4c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f4c0); p = true; }
    }
    f_8011f4c0_b(c, 0x8011f4c0);
    if p { crate::prof::exit(c, 0x8011f4c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f4c0_at(c: &mut Cpu, pc: u32) { f_8011f4c0_b(c, pc) }
#[inline(always)]
fn f_8011f4c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f4c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f4c0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f4c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f4d0; continue; }
                pc = 0x8011f4c8;
            }
            0x8011f4c8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f4d0;
            }
            0x8011f4d0 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x5c));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f4c0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetNicknameOrgPtr (0x8011f4d8, 24 bytes)
#[inline(never)]
pub fn f_8011f4d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f4d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f4d8); p = true; }
    }
    f_8011f4d8_b(c, 0x8011f4d8);
    if p { crate::prof::exit(c, 0x8011f4d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f4d8_at(c: &mut Cpu, pc: u32) { f_8011f4d8_b(c, pc) }
#[inline(always)]
fn f_8011f4d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f4d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f4d8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f4d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f4e8; continue; }
                pc = 0x8011f4e0;
            }
            0x8011f4e0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f4e8;
            }
            0x8011f4e8 => { ins += 2;
                c.r[3] = c.r[3].wrapping_add(0x44);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f4d8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetNicknamePtr (0x8011f4f0, 24 bytes)
#[inline(never)]
pub fn f_8011f4f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f4f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f4f0); p = true; }
    }
    f_8011f4f0_b(c, 0x8011f4f0);
    if p { crate::prof::exit(c, 0x8011f4f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f4f0_at(c: &mut Cpu, pc: u32) { f_8011f4f0_b(c, pc) }
#[inline(always)]
fn f_8011f4f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f4f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f4f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f4f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f500; continue; }
                pc = 0x8011f4f8;
            }
            0x8011f4f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f500;
            }
            0x8011f500 => { ins += 2;
                c.r[3] = c.r[3].wrapping_add(0x2e);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f4f0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetCatchTrainerNamePtr (0x8011f508, 24 bytes)
#[inline(never)]
pub fn f_8011f508(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f508) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f508); p = true; }
    }
    f_8011f508_b(c, 0x8011f508);
    if p { crate::prof::exit(c, 0x8011f508); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f508_at(c: &mut Cpu, pc: u32) { f_8011f508_b(c, pc) }
#[inline(always)]
fn f_8011f508_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f508;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f508, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f508 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f518; continue; }
                pc = 0x8011f510;
            }
            0x8011f510 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f518;
            }
            0x8011f518 => { ins += 2;
                c.r[3] = c.r[3].wrapping_add(0x18);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f508: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetCatchTrainerRnd (0x8011f520, 24 bytes)
#[inline(never)]
pub fn f_8011f520(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f520) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f520); p = true; }
    }
    f_8011f520_b(c, 0x8011f520);
    if p { crate::prof::exit(c, 0x8011f520); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f520_at(c: &mut Cpu, pc: u32) { f_8011f520_b(c, pc) }
#[inline(always)]
fn f_8011f520_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f520;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f520, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f520 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f530; continue; }
                pc = 0x8011f528;
            }
            0x8011f528 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f530;
            }
            0x8011f530 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x14));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f520: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetCatchTrainerSex (0x8011f538, 24 bytes)
#[inline(never)]
pub fn f_8011f538(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f538) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f538); p = true; }
    }
    f_8011f538_b(c, 0x8011f538);
    if p { crate::prof::exit(c, 0x8011f538); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f538_at(c: &mut Cpu, pc: u32) { f_8011f538_b(c, pc) }
#[inline(always)]
fn f_8011f538_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f538;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f538, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f538 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f548; continue; }
                pc = 0x8011f540;
            }
            0x8011f540 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f548;
            }
            0x8011f548 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x10));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f538: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetCatchBallId (0x8011f550, 24 bytes)
#[inline(never)]
pub fn f_8011f550(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f550) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f550); p = true; }
    }
    f_8011f550_b(c, 0x8011f550);
    if p { crate::prof::exit(c, 0x8011f550); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f550_at(c: &mut Cpu, pc: u32) { f_8011f550_b(c, pc) }
#[inline(always)]
fn f_8011f550_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f550;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f550, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f550 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f560; continue; }
                pc = 0x8011f558;
            }
            0x8011f558 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f560;
            }
            0x8011f560 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xf));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f550: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetCatchLevel (0x8011f568, 24 bytes)
#[inline(never)]
pub fn f_8011f568(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f568) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f568); p = true; }
    }
    f_8011f568_b(c, 0x8011f568);
    if p { crate::prof::exit(c, 0x8011f568); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f568_at(c: &mut Cpu, pc: u32) { f_8011f568_b(c, pc) }
#[inline(always)]
fn f_8011f568_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f568;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f568, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f568 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f578; continue; }
                pc = 0x8011f570;
            }
            0x8011f570 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f578;
            }
            0x8011f578 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xe));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f568: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetCatchFloorId (0x8011f580, 24 bytes)
#[inline(never)]
pub fn f_8011f580(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f580) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f580); p = true; }
    }
    f_8011f580_b(c, 0x8011f580);
    if p { crate::prof::exit(c, 0x8011f580); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f580_at(c: &mut Cpu, pc: u32) { f_8011f580_b(c, pc) }
#[inline(always)]
fn f_8011f580_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f580;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f580, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f580 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f590; continue; }
                pc = 0x8011f588;
            }
            0x8011f588 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f590;
            }
            0x8011f590 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f580: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetAttest (0x8011f598, 24 bytes)
#[inline(never)]
pub fn f_8011f598(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f598) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f598); p = true; }
    }
    f_8011f598_b(c, 0x8011f598);
    if p { crate::prof::exit(c, 0x8011f598); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f598_at(c: &mut Cpu, pc: u32) { f_8011f598_b(c, pc) }
#[inline(always)]
fn f_8011f598_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f598;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f598, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f598 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f5a8; continue; }
                pc = 0x8011f5a0;
            }
            0x8011f5a0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f5a8;
            }
            0x8011f5a8 => { ins += 2;
                c.r[3] = c.r[3].wrapping_add(0x8);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f598: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetRnd (0x8011f5b0, 24 bytes)
#[inline(never)]
pub fn f_8011f5b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f5b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f5b0); p = true; }
    }
    f_8011f5b0_b(c, 0x8011f5b0);
    if p { crate::prof::exit(c, 0x8011f5b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f5b0_at(c: &mut Cpu, pc: u32) { f_8011f5b0_b(c, pc) }
#[inline(always)]
fn f_8011f5b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f5b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f5b0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f5b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f5c0; continue; }
                pc = 0x8011f5b8;
            }
            0x8011f5b8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f5c0;
            }
            0x8011f5c0 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f5b0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosGetPokemonDataId (0x8011f5c8, 24 bytes)
#[inline(never)]
pub fn f_8011f5c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f5c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f5c8); p = true; }
    }
    f_8011f5c8_b(c, 0x8011f5c8);
    if p { crate::prof::exit(c, 0x8011f5c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f5c8_at(c: &mut Cpu, pc: u32) { f_8011f5c8_b(c, pc) }
#[inline(always)]
fn f_8011f5c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f5c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f5c8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f5c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f5d8; continue; }
                pc = 0x8011f5d0;
            }
            0x8011f5d0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f5d8;
            }
            0x8011f5d8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f5c8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonWazaBiosCopy (0x8011f5e0, 28 bytes)
#[inline(never)]
pub fn f_8011f5e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f5e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f5e0); p = true; }
    }
    f_8011f5e0_b(c, 0x8011f5e0);
    if p { crate::prof::exit(c, 0x8011f5e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f5e0_at(c: &mut Cpu, pc: u32) { f_8011f5e0_b(c, pc) }
#[inline(always)]
fn f_8011f5e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f5e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f5e0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f5e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f5e8;
            }
            0x8011f5e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f5f0;
            }
            0x8011f5f0 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.stw(c.r[3], c.r[0]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f5e0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosCopy (0x8011f5fc, 56 bytes)
#[inline(never)]
pub fn f_8011f5fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f5fc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f5fc); p = true; }
    }
    f_8011f5fc_b(c, 0x8011f5fc);
    if p { crate::prof::exit(c, 0x8011f5fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f5fc_at(c: &mut Cpu, pc: u32) { f_8011f5fc_b(c, pc) }
#[inline(always)]
fn f_8011f5fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f5fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f5fc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f5fc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f604;
            }
            0x8011f604 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8011f60c;
            }
            0x8011f60c => { ins += 5;
                c.r[0] = 0x27;
                c.r[5] = c.r[3].wrapping_add(0xfffffffc);
                c.r[4] = c.r[4].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x8011f61c;
            }
            0x8011f61c => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8011f61c; continue; }
                pc = 0x8011f630;
            }
            0x8011f630 => { ins += 1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f5fc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetCallRate (0x8011f634, 164 bytes)
#[inline(never)]
pub fn f_8011f634(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f634) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f634); p = true; }
    }
    f_8011f634_b(c, 0x8011f634);
    if p { crate::prof::exit(c, 0x8011f634); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f634_at(c: &mut Cpu, pc: u32) { f_8011f634_b(c, pc) }
#[inline(always)]
fn f_8011f634_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f634;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f634, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f634 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x8011f658; continue; }
                pc = 0x8011f650;
            }
            0x8011f650 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f6c0; continue;
                pc = 0x8011f658;
            }
            0x8011f658 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0xc2;
                c.r[6] = 0x0;
                c.lr = 0x8011f668; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f668;
            }
            0x8011f668 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011f678; continue; }
                pc = 0x8011f670;
            }
            0x8011f670 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f6c0; continue;
                pc = 0x8011f678;
            }
            0x8011f678 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xbf;
                c.r[6] = 0x0;
                c.lr = 0x8011f68c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f68c;
            }
            0x8011f68c => { ins += 3;
                c.r[31] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011f698; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011f77c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f698;
            }
            0x8011f698 => { ins += 4;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[30] = c.r[3] | c.r[3];
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7);
                if cr_bit(cr, 1) { pc = 0x8011f6bc; continue; }
                pc = 0x8011f6a8;
            }
            0x8011f6a8 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011f6b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f6b0;
            }
            0x8011f6b0 => { ins += 2;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8011f6b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cbf4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f6b8;
            }
            0x8011f6b8 => { ins += 1;
                pc = 0x8011f6c0; continue;
                pc = 0x8011f6bc;
            }
            0x8011f6bc => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x8011f6c0;
            }
            0x8011f6c0 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f634: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetHyperModeRate (0x8011f6d8, 164 bytes)
#[inline(never)]
pub fn f_8011f6d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f6d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f6d8); p = true; }
    }
    f_8011f6d8_b(c, 0x8011f6d8);
    if p { crate::prof::exit(c, 0x8011f6d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f6d8_at(c: &mut Cpu, pc: u32) { f_8011f6d8_b(c, pc) }
#[inline(always)]
fn f_8011f6d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f6d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f6d8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f6d8 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x8011f6fc; continue; }
                pc = 0x8011f6f4;
            }
            0x8011f6f4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f764; continue;
                pc = 0x8011f6fc;
            }
            0x8011f6fc => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0xc2;
                c.r[6] = 0x0;
                c.lr = 0x8011f70c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f70c;
            }
            0x8011f70c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011f71c; continue; }
                pc = 0x8011f714;
            }
            0x8011f714 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f764; continue;
                pc = 0x8011f71c;
            }
            0x8011f71c => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xbf;
                c.r[6] = 0x0;
                c.lr = 0x8011f730; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f730;
            }
            0x8011f730 => { ins += 3;
                c.r[31] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011f73c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011f77c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f73c;
            }
            0x8011f73c => { ins += 4;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[30] = c.r[3] | c.r[3];
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7);
                if cr_bit(cr, 1) { pc = 0x8011f760; continue; }
                pc = 0x8011f74c;
            }
            0x8011f74c => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011f754; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f754;
            }
            0x8011f754 => { ins += 2;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8011f75c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cc24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f75c;
            }
            0x8011f75c => { ins += 1;
                pc = 0x8011f764; continue;
                pc = 0x8011f760;
            }
            0x8011f760 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x8011f764;
            }
            0x8011f764 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f6d8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetDarkPokemonLevel (0x8011f77c, 404 bytes)
#[inline(never)]
pub fn f_8011f77c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f77c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f77c); p = true; }
    }
    f_8011f77c_b(c, 0x8011f77c);
    if p { crate::prof::exit(c, 0x8011f77c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f77c_at(c: &mut Cpu, pc: u32) { f_8011f77c_b(c, pc) }
#[inline(never)]
fn f_8011f77c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f77c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f77c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f77c => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x8011f7a0; continue; }
                pc = 0x8011f798;
            }
            0x8011f798 => { ins += 2;
                c.r[3] = 0x7;
                pc = 0x8011f8f8; continue;
                pc = 0x8011f7a0;
            }
            0x8011f7a0 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0xc2;
                c.r[6] = 0x0;
                c.lr = 0x8011f7b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f7b0;
            }
            0x8011f7b0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8011f8f4; continue; }
                pc = 0x8011f7bc;
            }
            0x8011f7bc => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc4;
                c.r[6] = 0x0;
                c.lr = 0x8011f7d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f7d0;
            }
            0x8011f7d0 => { ins += 2;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x8011f7dc; continue; }
                pc = 0x8011f7d8;
            }
            0x8011f7d8 => { ins += 1;
                c.r[31] = 0x1;
                pc = 0x8011f7dc;
            }
            0x8011f7dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011f7ec; continue; }
                pc = 0x8011f7e4;
            }
            0x8011f7e4 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffff9950); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x8011f824; continue;
                pc = 0x8011f7ec;
            }
            0x8011f7ec => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                c.r[6] = 0x0;
                c.lr = 0x8011f800; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f800;
            }
            0x8011f800 => { ins += 25;
                c.r[3] = c.r[3] ^ 0x80000000;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9968); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[2] = v; c.ps1[2] = v; }
                pc = 0x8011f824;
            }
            0x8011f824 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9950); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[2], c.f[0]);
                if !cr_bit(cr, 0) { pc = 0x8011f838; continue; }
                pc = 0x8011f830;
            }
            0x8011f830 => { ins += 2;
                c.f[1] = c.f[0];
                pc = 0x8011f860; continue;
                pc = 0x8011f838;
            }
            0x8011f838 => { ins += 26;
                c.r[3] = c.r[31].rotate_left(0) & 0xffff;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9970); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[0] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] / c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x8011f860;
            }
            0x8011f860 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x8011f878; continue; }
                pc = 0x8011f870;
            }
            0x8011f870 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011f8f8; continue;
                pc = 0x8011f878;
            }
            0x8011f878 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9958); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x8011f890; continue; }
                pc = 0x8011f888;
            }
            0x8011f888 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x8011f8f8; continue;
                pc = 0x8011f890;
            }
            0x8011f890 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff995c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x8011f8a8; continue; }
                pc = 0x8011f8a0;
            }
            0x8011f8a0 => { ins += 2;
                c.r[3] = 0x2;
                pc = 0x8011f8f8; continue;
                pc = 0x8011f8a8;
            }
            0x8011f8a8 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9960); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x8011f8c0; continue; }
                pc = 0x8011f8b8;
            }
            0x8011f8b8 => { ins += 2;
                c.r[3] = 0x3;
                pc = 0x8011f8f8; continue;
                pc = 0x8011f8c0;
            }
            0x8011f8c0 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff9964); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x8011f8d8; continue; }
                pc = 0x8011f8d0;
            }
            0x8011f8d0 => { ins += 2;
                c.r[3] = 0x4;
                pc = 0x8011f8f8; continue;
                pc = 0x8011f8d8;
            }
            0x8011f8d8 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff9950); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if !cr_bit(cr, 1) { pc = 0x8011f8ec; continue; }
                pc = 0x8011f8e4;
            }
            0x8011f8e4 => { ins += 2;
                c.r[3] = 0x5;
                pc = 0x8011f8f8; continue;
                pc = 0x8011f8ec;
            }
            0x8011f8ec => { ins += 2;
                c.r[3] = 0x6;
                pc = 0x8011f8f8; continue;
                pc = 0x8011f8f4;
            }
            0x8011f8f4 => { ins += 1;
                c.r[3] = 0x7;
                pc = 0x8011f8f8;
            }
            0x8011f8f8 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f77c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonAddDpFormPokemonDpFilterId (0x8011f910, 700 bytes)
#[inline(never)]
pub fn f_8011f910(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011f910) { return; }
        if c.prof { crate::prof::enter(c, 0x8011f910); p = true; }
    }
    f_8011f910_b(c, 0x8011f910);
    if p { crate::prof::exit(c, 0x8011f910); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011f910_at(c: &mut Cpu, pc: u32) { f_8011f910_b(c, pc) }
#[inline(never)]
fn f_8011f910_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011f910;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011f910, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011f910 => { ins += 23;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[5] = 0xc2;
                c.r[6] = 0x0;
                c.lr = 0x8011f944; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f944;
            }
            0x8011f944 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8011fbb0; continue; }
                pc = 0x8011f94c;
            }
            0x8011f94c => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xbf;
                c.r[6] = 0x0;
                c.lr = 0x8011f960; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f960;
            }
            0x8011f960 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.lr = 0x8011f968; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f968;
            }
            0x8011f968 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if cr_bit(cr, 2) { pc = 0x8011fbb0; continue; }
                pc = 0x8011f970;
            }
            0x8011f970 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011f978; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cab8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f978;
            }
            0x8011f978 => { ins += 1;
                c.lr = 0x8011f97c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ca9c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f97c;
            }
            0x8011f97c => { ins += 11;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[3] = 0x43300000;
                c.r[4] = c.r[0] ^ 0x80000000;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                { let ea = c.r[2].wrapping_add(0xffff9968); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[1]); c.f[31] = v; c.ps1[31] = v; }
                if !cr_bit(cr, 2) { pc = 0x8011f9ec; continue; }
                pc = 0x8011f9a8;
            }
            0x8011f9a8 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011f9b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801440a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f9b0;
            }
            0x8011f9b0 => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[27]);
                if cr_bit(cr, 2) { pc = 0x8011fbb0; continue; }
                pc = 0x8011f9b8;
            }
            0x8011f9b8 => { ins += 1;
                c.lr = 0x8011f9bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80144014(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f9bc;
            }
            0x8011f9bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 2) { pc = 0x8011fbb0; continue; }
                pc = 0x8011f9c8;
            }
            0x8011f9c8 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011f9d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80143e14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011f9d0;
            }
            0x8011f9d0 => { ins += 7;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9970); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[31] * c.f[0]); c.f[31] = v; c.ps1[31] = v; }
                pc = 0x8011f9ec;
            }
            0x8011f9ec => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5);
                if !cr_bit(cr, 2) { pc = 0x8011fa48; continue; }
                pc = 0x8011f9f8;
            }
            0x8011f9f8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011fa08; continue; }
                pc = 0x8011fa00;
            }
            0x8011fa00 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffff9950); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8011fa40; continue;
                pc = 0x8011fa08;
            }
            0x8011fa08 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                c.r[6] = 0x0;
                c.lr = 0x8011fa1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fa1c;
            }
            0x8011fa1c => { ins += 25;
                c.r[3] = c.r[3] ^ 0x80000000;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9968); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x8011fa40;
            }
            0x8011fa40 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffff9978); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] * c.f[1]); c.f[31] = v; c.ps1[31] = v; }
                pc = 0x8011fa48;
            }
            0x8011fa48 => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011fa60; continue; }
                pc = 0x8011fa50;
            }
            0x8011fa50 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011fa58; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cd6c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fa58;
            }
            0x8011fa58 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8011fabc; continue;
                pc = 0x8011fa60;
            }
            0x8011fa60 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8011fa78; continue; }
                pc = 0x8011fa68;
            }
            0x8011fa68 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011fa70; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cd50(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fa70;
            }
            0x8011fa70 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8011fabc; continue;
                pc = 0x8011fa78;
            }
            0x8011fa78 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 2) { pc = 0x8011fa90; continue; }
                pc = 0x8011fa80;
            }
            0x8011fa80 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011fa88; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cd34(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fa88;
            }
            0x8011fa88 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8011fabc; continue;
                pc = 0x8011fa90;
            }
            0x8011fa90 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 2) { pc = 0x8011faa8; continue; }
                pc = 0x8011fa98;
            }
            0x8011fa98 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011faa0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cd18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011faa0;
            }
            0x8011faa0 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8011fabc; continue;
                pc = 0x8011faa8;
            }
            0x8011faa8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 2) { pc = 0x8011fabc; continue; }
                pc = 0x8011fab0;
            }
            0x8011fab0 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011fab8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ccfc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fab8;
            }
            0x8011fab8 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8011fabc;
            }
            0x8011fabc => { ins += 2;
                c.r[3] = c.r[31].rotate_left(0) & 0xff;
                c.lr = 0x8011fac4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cbc8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fac4;
            }
            0x8011fac4 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if cr_bit(cr, 2) { pc = 0x8011fbb0; continue; }
                pc = 0x8011facc;
            }
            0x8011facc => { ins += 1;
                c.lr = 0x8011fad0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cbb0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fad0;
            }
            0x8011fad0 => { ins += 3;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011fadc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cb98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fadc;
            }
            0x8011fadc => { ins += 2;
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[4]);
                if cr_bit(cr, 2) { pc = 0x8011fbb0; continue; }
                pc = 0x8011fae4;
            }
            0x8011fae4 => { ins += 30;
                c.r[0] = 0x43300000;
                c.r[3] = c.r[30].rotate_left(0) & 0xff;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9970); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x14), c.r[4]);
                { let v = s(c.f[0] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[31] * c.f[1]); c.f[31] = v; c.ps1[31] = v; }
                { let v = s(c.f[0] - c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[31] / c.f[0]); c.f[31] = v; c.ps1[31] = v; }
                pc = 0x8011fb20; continue;
                pc = 0x8011fb1c;
            }
            0x8011fb1c => { ins += 1;
                pc = 0x8011fbb0; continue;
                pc = 0x8011fb20;
            }
            0x8011fb20 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x8011fbb0; continue; }
                pc = 0x8011fb28;
            }
            0x8011fb28 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8011fb34; continue; }
                pc = 0x8011fb2c;
            }
            0x8011fb2c => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffff9950); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8011fb6c; continue;
                pc = 0x8011fb34;
            }
            0x8011fb34 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                c.r[6] = 0x0;
                c.lr = 0x8011fb48; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fb48;
            }
            0x8011fb48 => { ins += 25;
                c.r[3] = c.r[3] ^ 0x80000000;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9968); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x8011fb6c;
            }
            0x8011fb6c => { ins += 4;
                { let v = s(c.f[0] + c.f[31]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9950); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if !cr_bit(cr, 0) { pc = 0x8011fb80; continue; }
                pc = 0x8011fb7c;
            }
            0x8011fb7c => { ins += 1;
                c.f[1] = c.f[0];
                pc = 0x8011fb80;
            }
            0x8011fb80 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x8011fbb0; continue; }
                pc = 0x8011fb88;
            }
            0x8011fb88 => { ins += 10;
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                { let v = s(c.f[0] * c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                c.r[6] = 0x0;
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[0]; c.stfd(ea, v); }
                c.r[7] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.lr = 0x8011fbb0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fbb0;
            }
            0x8011fbb0 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x30); c.f[31] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011f910: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSetDp (0x8011fbcc, 72 bytes)
#[inline(never)]
pub fn f_8011fbcc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011fbcc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011fbcc); p = true; }
    }
    f_8011fbcc_b(c, 0x8011fbcc);
    if p { crate::prof::exit(c, 0x8011fbcc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011fbcc_at(c: &mut Cpu, pc: u32) { f_8011fbcc_b(c, pc) }
#[inline(always)]
fn f_8011fbcc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011fbcc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011fbcc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011fbcc => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8011fc04; continue; }
                pc = 0x8011fbe0;
            }
            0x8011fbe0 => { ins += 9;
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                c.r[6] = 0x0;
                { let v = s(c.f[0] * c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[7] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.lr = 0x8011fc04; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fc04;
            }
            0x8011fc04 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011fbcc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetDp (0x8011fc14, 96 bytes)
#[inline(never)]
pub fn f_8011fc14(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011fc14) { return; }
        if c.prof { crate::prof::enter(c, 0x8011fc14); p = true; }
    }
    f_8011fc14_b(c, 0x8011fc14);
    if p { crate::prof::exit(c, 0x8011fc14); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011fc14_at(c: &mut Cpu, pc: u32) { f_8011fc14_b(c, pc) }
#[inline(always)]
fn f_8011fc14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011fc14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011fc14, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011fc14 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011fc30; continue; }
                pc = 0x8011fc28;
            }
            0x8011fc28 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffff9950); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x8011fc64; continue;
                pc = 0x8011fc30;
            }
            0x8011fc30 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                c.r[6] = 0x0;
                c.lr = 0x8011fc40; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fc40;
            }
            0x8011fc40 => { ins += 25;
                c.r[3] = c.r[3] ^ 0x80000000;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9968); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x8011fc64;
            }
            0x8011fc64 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011fc14: bad pc {:#x}", pc),
        }
    }
}

/// pokemonIsDarkPokemon (0x8011fc74, 48 bytes)
#[inline(never)]
pub fn f_8011fc74(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011fc74) { return; }
        if c.prof { crate::prof::enter(c, 0x8011fc74); p = true; }
    }
    f_8011fc74_b(c, 0x8011fc74);
    if p { crate::prof::exit(c, 0x8011fc74); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011fc74_at(c: &mut Cpu, pc: u32) { f_8011fc74_b(c, pc) }
#[inline(always)]
fn f_8011fc74_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011fc74;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011fc74, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011fc74 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0xc2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.lr = 0x8011fc90; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fc90;
            }
            0x8011fc90 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011fc74: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSetDarkPokemonStatus (0x8011fca4, 292 bytes)
#[inline(never)]
pub fn f_8011fca4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011fca4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011fca4); p = true; }
    }
    f_8011fca4_b(c, 0x8011fca4);
    if p { crate::prof::exit(c, 0x8011fca4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011fca4_at(c: &mut Cpu, pc: u32) { f_8011fca4_b(c, pc) }
#[inline(always)]
fn f_8011fca4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011fca4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011fca4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011fca4 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x8011fdb0; continue; }
                pc = 0x8011fcc4;
            }
            0x8011fcc4 => { ins += 5;
                c.r[4] = 0x0;
                c.r[5] = 0xc3;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011fcd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fcd8;
            }
            0x8011fcd8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x8011fcf8; continue; }
                pc = 0x8011fce0;
            }
            0x8011fce0 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                c.r[6] = 0x0;
                c.r[7] = 0xffffff9c;
                c.lr = 0x8011fcf8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fcf8;
            }
            0x8011fcf8 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc6;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011fd10; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fd10;
            }
            0x8011fd10 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc7;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011fd28; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fd28;
            }
            0x8011fd28 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc8;
                c.r[6] = 0x0;
                c.lr = 0x8011fd3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fd3c;
            }
            0x8011fd3c => { ins += 2;
                c.r[4] = 0x1;
                c.lr = 0x8011fd44; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b950(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fd44;
            }
            0x8011fd44 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[7] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0xc3;
                c.r[6] = 0x0;
                c.lr = 0x8011fd5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fd5c;
            }
            0x8011fd5c => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011fd64; { c.steps = st; c.cycles = ins; c.cr = cr; f_801eeeb8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fd64;
            }
            0x8011fd64 => { ins += 9;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                { let ea = c.r[2].wrapping_add(0xffff9970); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                if cr_bit(cr, 2) { pc = 0x8011fdb0; continue; }
                pc = 0x8011fd88;
            }
            0x8011fd88 => { ins += 10;
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                { let v = s(c.f[0] * c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                c.r[6] = 0x0;
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[7] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.lr = 0x8011fdb0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fdb0;
            }
            0x8011fdb0 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011fca4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonToMenuPokemonStatus (0x8011fdc8, 1284 bytes)
#[inline(never)]
pub fn f_8011fdc8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011fdc8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011fdc8); p = true; }
    }
    f_8011fdc8_b(c, 0x8011fdc8);
    if p { crate::prof::exit(c, 0x8011fdc8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011fdc8_at(c: &mut Cpu, pc: u32) { f_8011fdc8_b(c, pc) }
#[inline(never)]
fn f_8011fdc8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8011fdc8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8011fdc8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8011fdc8 => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.lr = 0x8011fdfc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fdfc;
            }
            0x8011fdfc => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x77;
                c.r[6] = 0x0;
                c.lr = 0x8011fe10; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fe10;
            }
            0x8011fe10 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011fe1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800f9e70(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fe1c;
            }
            0x8011fe1c => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x7a;
                c.r[6] = 0x0;
                c.lr = 0x8011fe30; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fe30;
            }
            0x8011fe30 => { ins += 6;
                c.stb(c.r[30].wrapping_add(0x17), c.r[3]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x8011fe48; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fe48;
            }
            0x8011fe48 => { ins += 6;
                c.sth(c.r[30].wrapping_add(0x18), c.r[3]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x8011fe60; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fe60;
            }
            0x8011fe60 => { ins += 6;
                c.sth(c.r[30].wrapping_add(0x1a), c.r[3]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x7a;
                c.r[6] = 0x0;
                c.lr = 0x8011fe78; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fe78;
            }
            0x8011fe78 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                if !cr_bit(cr, 2) { pc = 0x8011fe8c; continue; }
                pc = 0x8011fe84;
            }
            0x8011fe84 => { ins += 2;
                c.r[28] = 0x0;
                pc = 0x8011fed8; continue;
                pc = 0x8011fe8c;
            }
            0x8011fe8c => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x8011fea0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fea0;
            }
            0x8011fea0 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x8011feb4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011feb4;
            }
            0x8011feb4 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.lr = 0x8011febc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011febc;
            }
            0x8011febc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011fecc; continue; }
                pc = 0x8011fec4;
            }
            0x8011fec4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011fed4; continue;
                pc = 0x8011fecc;
            }
            0x8011fecc => { ins += 2;
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8011fed4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011fed4;
            }
            0x8011fed4 => { ins += 1;
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8011fed8;
            }
            0x8011fed8 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xc2;
                c.r[6] = 0x0;
                c.lr = 0x8011feec; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011feec;
            }
            0x8011feec => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8011ff7c; continue; }
                pc = 0x8011fef4;
            }
            0x8011fef4 => { ins += 4;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                c.r[0] = c.r[29].wrapping_add(0x1);
                c.r[29] = c.r[0].rotate_left(0) & 0xff;
                if !cr_bit(cr, 2) { pc = 0x8011ff0c; continue; }
                pc = 0x8011ff04;
            }
            0x8011ff04 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ff54; continue;
                pc = 0x8011ff0c;
            }
            0x8011ff0c => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x8011ff20; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011ff20;
            }
            0x8011ff20 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x8011ff34; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011ff34;
            }
            0x8011ff34 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.lr = 0x8011ff3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011ff3c;
            }
            0x8011ff3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8011ff4c; continue; }
                pc = 0x8011ff44;
            }
            0x8011ff44 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8011ff54; continue;
                pc = 0x8011ff4c;
            }
            0x8011ff4c => { ins += 2;
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8011ff54; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011ff54;
            }
            0x8011ff54 => { ins += 7;
                c.r[0] = c.r[3].wrapping_sub(c.r[28]);
                c.r[3] = c.r[31] | c.r[31];
                c.stw(c.r[30].wrapping_add(0x1c), c.r[0]);
                c.r[4] = 0x0;
                c.r[5] = 0x79;
                c.r[6] = 0x0;
                c.lr = 0x8011ff70; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011ff70;
            }
            0x8011ff70 => { ins += 3;
                c.r[0] = c.r[3].wrapping_sub(c.r[28]);
                c.stw(c.r[30].wrapping_add(0x20), c.r[0]);
                pc = 0x80120004; continue;
                pc = 0x8011ff7c;
            }
            0x8011ff7c => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xc3;
                c.r[6] = 0x0;
                c.lr = 0x8011ff90; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011ff90;
            }
            0x8011ff90 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8011ff98; { c.steps = st; c.cycles = ins; c.cr = cr; f_801eeeb8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011ff98;
            }
            0x8011ff98 => { ins += 10;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                { let ea = c.r[2].wrapping_add(0xffff9970); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[30].wrapping_add(0x1c); let v = c.f[0]; c.stfs(ea, v); }
                if !cr_bit(cr, 2) { pc = 0x8011ffc8; continue; }
                pc = 0x8011ffc0;
            }
            0x8011ffc0 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffff9950); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80120000; continue;
                pc = 0x8011ffc8;
            }
            0x8011ffc8 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                c.r[6] = 0x0;
                c.lr = 0x8011ffdc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8011ffdc;
            }
            0x8011ffdc => { ins += 25;
                c.r[3] = c.r[3] ^ 0x80000000;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9968); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80120000;
            }
            0x80120000 => { ins += 1;
                { let ea = c.r[30].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80120004;
            }
            0x80120004 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80120014; continue; }
                pc = 0x8012000c;
            }
            0x8012000c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801201f8; continue;
                pc = 0x80120014;
            }
            0x80120014 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x8012001c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012001c;
            }
            0x8012001c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120044; continue; }
                pc = 0x80120028;
            }
            0x80120028 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80120030; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120030;
            }
            0x80120030 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80120044; continue; }
                pc = 0x8012003c;
            }
            0x8012003c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120050; continue;
                pc = 0x80120044;
            }
            0x80120044 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3;
                c.lr = 0x80120050; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120050;
            }
            0x80120050 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80120064; continue; }
                pc = 0x8012005c;
            }
            0x8012005c => { ins += 2;
                c.r[0] = 0x3a;
                pc = 0x801201f8; continue;
                pc = 0x80120064;
            }
            0x80120064 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x8012006c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012006c;
            }
            0x8012006c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120094; continue; }
                pc = 0x80120078;
            }
            0x80120078 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80120080; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120080;
            }
            0x80120080 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80120094; continue; }
                pc = 0x8012008c;
            }
            0x8012008c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801200a0; continue;
                pc = 0x80120094;
            }
            0x80120094 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4;
                c.lr = 0x801200a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801200a0;
            }
            0x801200a0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801200b4; continue; }
                pc = 0x801200ac;
            }
            0x801200ac => { ins += 2;
                c.r[0] = 0x3a;
                pc = 0x801201f8; continue;
                pc = 0x801200b4;
            }
            0x801200b4 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x801200bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801200bc;
            }
            0x801200bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801200e4; continue; }
                pc = 0x801200c8;
            }
            0x801200c8 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x801200d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801200d0;
            }
            0x801200d0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801200e4; continue; }
                pc = 0x801200dc;
            }
            0x801200dc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801200f0; continue;
                pc = 0x801200e4;
            }
            0x801200e4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x5;
                c.lr = 0x801200f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801200f0;
            }
            0x801200f0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80120104; continue; }
                pc = 0x801200fc;
            }
            0x801200fc => { ins += 2;
                c.r[0] = 0x3b;
                pc = 0x801201f8; continue;
                pc = 0x80120104;
            }
            0x80120104 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x8012010c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012010c;
            }
            0x8012010c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120134; continue; }
                pc = 0x80120118;
            }
            0x80120118 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80120120; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120120;
            }
            0x80120120 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80120134; continue; }
                pc = 0x8012012c;
            }
            0x8012012c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120140; continue;
                pc = 0x80120134;
            }
            0x80120134 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x6;
                c.lr = 0x80120140; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120140;
            }
            0x80120140 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80120154; continue; }
                pc = 0x8012014c;
            }
            0x8012014c => { ins += 2;
                c.r[0] = 0x3c;
                pc = 0x801201f8; continue;
                pc = 0x80120154;
            }
            0x80120154 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x8012015c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012015c;
            }
            0x8012015c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120184; continue; }
                pc = 0x80120168;
            }
            0x80120168 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80120170; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120170;
            }
            0x80120170 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80120184; continue; }
                pc = 0x8012017c;
            }
            0x8012017c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120190; continue;
                pc = 0x80120184;
            }
            0x80120184 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x80120190; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120190;
            }
            0x80120190 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801201a4; continue; }
                pc = 0x8012019c;
            }
            0x8012019c => { ins += 2;
                c.r[0] = 0x3d;
                pc = 0x801201f8; continue;
                pc = 0x801201a4;
            }
            0x801201a4 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x801201ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801201ac;
            }
            0x801201ac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801201d4; continue; }
                pc = 0x801201b8;
            }
            0x801201b8 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x801201c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801201c0;
            }
            0x801201c0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801201d4; continue; }
                pc = 0x801201cc;
            }
            0x801201cc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801201e0; continue;
                pc = 0x801201d4;
            }
            0x801201d4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x801201e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801201e0;
            }
            0x801201e0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801201f4; continue; }
                pc = 0x801201ec;
            }
            0x801201ec => { ins += 2;
                c.r[0] = 0x3e;
                pc = 0x801201f8; continue;
                pc = 0x801201f4;
            }
            0x801201f4 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x801201f8;
            }
            0x801201f8 => { ins += 3;
                c.sth(c.r[30].wrapping_add(0x24), c.r[0]);
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80120204; { c.steps = st; c.cycles = ins; c.cr = cr; f_8010bbb8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120204;
            }
            0x80120204 => { ins += 3;
                c.sth(c.r[30].wrapping_add(0x26), c.r[3]);
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80120210; { c.steps = st; c.cycles = ins; c.cr = cr; f_8001d994(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120210;
            }
            0x80120210 => { ins += 6;
                c.stb(c.r[30].wrapping_add(0x28), c.r[3]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xc2;
                c.r[6] = 0x0;
                c.lr = 0x80120228; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120228;
            }
            0x80120228 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8012023c; continue; }
                pc = 0x80120230;
            }
            0x80120230 => { ins += 3;
                c.r[0] = 0x0;
                c.stb(c.r[30].wrapping_add(0x29), c.r[0]);
                pc = 0x80120294; continue;
                pc = 0x8012023c;
            }
            0x8012023c => { ins += 2;
                c.r[3] = 0x3e;
                c.lr = 0x80120244; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120244;
            }
            0x80120244 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x8012026c; continue; }
                pc = 0x80120250;
            }
            0x80120250 => { ins += 2;
                c.r[3] = 0x3e;
                c.lr = 0x80120258; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120258;
            }
            0x80120258 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x8012026c; continue; }
                pc = 0x80120264;
            }
            0x80120264 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120278; continue;
                pc = 0x8012026c;
            }
            0x8012026c => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3e;
                c.lr = 0x80120278; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120278;
            }
            0x80120278 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8012028c; continue; }
                pc = 0x80120280;
            }
            0x80120280 => { ins += 3;
                c.r[0] = 0x1;
                c.stb(c.r[30].wrapping_add(0x29), c.r[0]);
                pc = 0x80120294; continue;
                pc = 0x8012028c;
            }
            0x8012028c => { ins += 2;
                c.r[0] = 0x2;
                c.stb(c.r[30].wrapping_add(0x29), c.r[0]);
                pc = 0x80120294;
            }
            0x80120294 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x82;
                c.r[6] = 0x0;
                c.lr = 0x801202a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801202a8;
            }
            0x801202a8 => { ins += 10;
                c.sth(c.r[30].wrapping_add(0x2a), c.r[3]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8011fdc8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonToMenuPokemonStatusSubBar (0x801202cc, 476 bytes)
#[inline(never)]
pub fn f_801202cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801202cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801202cc); p = true; }
    }
    f_801202cc_b(c, 0x801202cc);
    if p { crate::prof::exit(c, 0x801202cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801202cc_at(c: &mut Cpu, pc: u32) { f_801202cc_b(c, pc) }
#[inline(never)]
fn f_801202cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801202cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801202cc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801202cc => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x7a;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.lr = 0x80120300; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120300;
            }
            0x80120300 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                if !cr_bit(cr, 2) { pc = 0x80120314; continue; }
                pc = 0x8012030c;
            }
            0x8012030c => { ins += 2;
                c.r[29] = 0x0;
                pc = 0x80120360; continue;
                pc = 0x80120314;
            }
            0x80120314 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80120328; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120328;
            }
            0x80120328 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x8012033c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012033c;
            }
            0x8012033c => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.lr = 0x80120344; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120344;
            }
            0x80120344 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80120354; continue; }
                pc = 0x8012034c;
            }
            0x8012034c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8012035c; continue;
                pc = 0x80120354;
            }
            0x80120354 => { ins += 2;
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8012035c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012035c;
            }
            0x8012035c => { ins += 1;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x80120360;
            }
            0x80120360 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc2;
                c.r[6] = 0x0;
                c.lr = 0x80120374; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120374;
            }
            0x80120374 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80120400; continue; }
                pc = 0x8012037c;
            }
            0x8012037c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8012038c; continue; }
                pc = 0x80120384;
            }
            0x80120384 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801203d8; continue;
                pc = 0x8012038c;
            }
            0x8012038c => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x801203a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801203a0;
            }
            0x801203a0 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x801203b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801203b4;
            }
            0x801203b4 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.lr = 0x801203bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801203bc;
            }
            0x801203bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801203cc; continue; }
                pc = 0x801203c4;
            }
            0x801203c4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801203d8; continue;
                pc = 0x801203cc;
            }
            0x801203cc => { ins += 3;
                c.r[0] = c.r[28].wrapping_add(0x1);
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x801203d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801203d8;
            }
            0x801203d8 => { ins += 7;
                c.r[0] = c.r[3].wrapping_sub(c.r[29]);
                c.r[3] = c.r[30] | c.r[30];
                c.stw(c.r[31].wrapping_add(0x1c), c.r[0]);
                c.r[4] = 0x0;
                c.r[5] = 0x79;
                c.r[6] = 0x0;
                c.lr = 0x801203f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801203f4;
            }
            0x801203f4 => { ins += 3;
                c.r[0] = c.r[3].wrapping_sub(c.r[29]);
                c.stw(c.r[31].wrapping_add(0x20), c.r[0]);
                pc = 0x80120488; continue;
                pc = 0x80120400;
            }
            0x80120400 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc3;
                c.r[6] = 0x0;
                c.lr = 0x80120414; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120414;
            }
            0x80120414 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8012041c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801eeeb8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012041c;
            }
            0x8012041c => { ins += 10;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                { let ea = c.r[2].wrapping_add(0xffff9970); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31].wrapping_add(0x1c); let v = c.f[0]; c.stfs(ea, v); }
                if !cr_bit(cr, 2) { pc = 0x8012044c; continue; }
                pc = 0x80120444;
            }
            0x80120444 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffff9950); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80120484; continue;
                pc = 0x8012044c;
            }
            0x8012044c => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0xc5;
                c.r[6] = 0x0;
                c.lr = 0x80120460; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120460;
            }
            0x80120460 => { ins += 25;
                c.r[3] = c.r[3] ^ 0x80000000;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9968); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9954); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80120484;
            }
            0x80120484 => { ins += 1;
                { let ea = c.r[31].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80120488;
            }
            0x80120488 => { ins += 9;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801202cc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonToMenuWazaStatus (0x801204a8, 460 bytes)
#[inline(never)]
pub fn f_801204a8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801204a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801204a8); p = true; }
    }
    f_801204a8_b(c, 0x801204a8);
    if p { crate::prof::exit(c, 0x801204a8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801204a8_at(c: &mut Cpu, pc: u32) { f_801204a8_b(c, pc) }
#[inline(never)]
fn f_801204a8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801204a8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801204a8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801204a8 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x48;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801204d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801204d0;
            }
            0x801204d0 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x77;
                c.r[6] = 0x0;
                c.lr = 0x801204e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801204e4;
            }
            0x801204e4 => { ins += 3;
                c.stw(c.r[31], c.r[3]);
                c.r[26] = 0x0;
                pc = 0x80120654; continue;
                pc = 0x801204f0;
            }
            0x801204f0 => { ins += 8;
                c.r[29] = c.r[26].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                c.r[3] = c.r[29].wrapping_mul(0xc);
                c.r[25] = c.r[3].wrapping_add(0x4);
                c.r[25] = c.r[31].wrapping_add(c.r[25]);
                if !cr_bit(cr, 2) { pc = 0x80120510; continue; }
                pc = 0x80120508;
            }
            0x80120508 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8012055c; continue;
                pc = 0x80120510;
            }
            0x80120510 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x80120524; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120524;
            }
            0x80120524 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80120534; continue; }
                pc = 0x8012052c;
            }
            0x8012052c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8012055c; continue;
                pc = 0x80120534;
            }
            0x80120534 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x80120548; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120548;
            }
            0x80120548 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x163);
                if !cr_bit(cr, 2) { pc = 0x80120558; continue; }
                pc = 0x80120550;
            }
            0x80120550 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8012055c; continue;
                pc = 0x80120558;
            }
            0x80120558 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x8012055c;
            }
            0x8012055c => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8012057c; continue; }
                pc = 0x80120564;
            }
            0x80120564 => { ins += 6;
                c.r[0] = 0x0;
                c.stw(c.r[25], c.r[0]);
                c.stw(c.r[25].wrapping_add(0x4), c.r[0]);
                c.stb(c.r[25].wrapping_add(0xa), c.r[0]);
                c.stb(c.r[25].wrapping_add(0xb), c.r[0]);
                pc = 0x80120650; continue;
                pc = 0x8012057c;
            }
            0x8012057c => { ins += 6;
                c.r[28] = c.r[26].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x80120594; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120594;
            }
            0x80120594 => { ins += 6;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x801205ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801205ac;
            }
            0x801205ac => { ins += 6;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x801205c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801205c4;
            }
            0x801205c4 => { ins += 1;
                c.lr = 0x801205c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_800fa280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801205c8;
            }
            0x801205c8 => { ins += 3;
                c.stw(c.r[25], c.r[3]);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801205d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8010c4d4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801205d4;
            }
            0x801205d4 => { ins += 1;
                c.lr = 0x801205d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_800fa280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801205d8;
            }
            0x801205d8 => { ins += 3;
                c.stw(c.r[25].wrapping_add(0x4), c.r[3]);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801205e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8010c46c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801205e4;
            }
            0x801205e4 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                c.sth(c.r[25].wrapping_add(0x8), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x801205f8; continue; }
                pc = 0x801205f0;
            }
            0x801205f0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120634; continue;
                pc = 0x801205f8;
            }
            0x801205f8 => { ins += 6;
                c.r[27] = c.r[28].wrapping_add(0x4);
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[27].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x80120610; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120610;
            }
            0x80120610 => { ins += 6;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[27].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x81;
                c.lr = 0x80120628; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120628;
            }
            0x80120628 => { ins += 3;
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80120634; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bb6c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120634;
            }
            0x80120634 => { ins += 6;
                c.stb(c.r[25].wrapping_add(0xa), c.r[3]);
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x80;
                c.lr = 0x8012064c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012064c;
            }
            0x8012064c => { ins += 1;
                c.stb(c.r[25].wrapping_add(0xb), c.r[3]);
                pc = 0x80120650;
            }
            0x80120650 => { ins += 1;
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x80120654;
            }
            0x80120654 => { ins += 3;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801204f0; continue; }
                pc = 0x80120660;
            }
            0x80120660 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801204a8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonCheckSetMonohiroi (0x80120674, 504 bytes)
#[inline(never)]
pub fn f_80120674(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80120674) { return; }
        if c.prof { crate::prof::enter(c, 0x80120674); p = true; }
    }
    f_80120674_b(c, 0x80120674);
    if p { crate::prof::exit(c, 0x80120674); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80120674_at(c: &mut Cpu, pc: u32) { f_80120674_b(c, pc) }
#[inline(never)]
fn f_80120674_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80120674;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80120674, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80120674 => { ins += 29;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x3c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                c.stw(c.r[1].wrapping_add(0x38), c.r[30]);
                c.r[30] = c.r[4].wrapping_add(0x296c);
                c.r[12] = c.lwz_t(ins, c.r[30]);
                c.r[11] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                c.r[10] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                c.r[9] = c.lwz_t(ins, c.r[30].wrapping_add(0xc));
                c.r[8] = c.lwz_t(ins, c.r[30].wrapping_add(0x10));
                c.r[7] = c.lwz_t(ins, c.r[30].wrapping_add(0x14));
                c.r[6] = c.lwz_t(ins, c.r[30].wrapping_add(0x18));
                c.r[5] = c.lwz_t(ins, c.r[30].wrapping_add(0x1c));
                c.r[4] = c.lwz_t(ins, c.r[30].wrapping_add(0x20));
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x24));
                c.stw(c.r[1].wrapping_add(0x8), c.r[12]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[11]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x2c), c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801206f0; continue; }
                pc = 0x801206e8;
            }
            0x801206e8 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80120768; continue;
                pc = 0x801206f0;
            }
            0x801206f0 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80120700; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120700;
            }
            0x80120700 => { ins += 6;
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x17;
                c.r[6] = 0x1;
                c.lr = 0x80120718; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120718;
            }
            0x80120718 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8012073c; continue; }
                pc = 0x80120720;
            }
            0x80120720 => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x17;
                c.r[6] = 0x0;
                c.lr = 0x80120734; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120734;
            }
            0x80120734 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80120768; continue;
                pc = 0x8012073c;
            }
            0x8012073c => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xb7;
                c.r[6] = 0x0;
                c.lr = 0x80120750; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120750;
            }
            0x80120750 => { ins += 5;
                c.r[6] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x17;
                c.lr = 0x80120764; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120764;
            }
            0x80120764 => { ins += 1;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80120768;
            }
            0x80120768 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x35);
                if !cr_bit(cr, 2) { pc = 0x80120850; continue; }
                pc = 0x80120774;
            }
            0x80120774 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x82;
                c.r[6] = 0x0;
                c.lr = 0x80120788; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120788;
            }
            0x80120788 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80120850; continue; }
                pc = 0x80120790;
            }
            0x80120790 => { ins += 1;
                c.lr = 0x80120794; { c.steps = st; c.cycles = ins; c.cr = cr; f_800e0c54(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120794;
            }
            0x80120794 => { ins += 16;
                c.r[4] = 0x66660000;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[4].wrapping_add(0x6667);
                c.r[0] = mulhw(c.r[0], c.r[5]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[0] = (v >> 2) as u32; }
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.r[0].wrapping_mul(0xa);
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80120850; continue; }
                pc = 0x801207bc;
            }
            0x801207bc => { ins += 1;
                c.lr = 0x801207c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_800e0c54(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801207c0;
            }
            0x801207c0 => { ins += 19;
                c.r[4] = 0x51ec0000;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[4].wrapping_add(0xffff851f);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[0] = mulhw(c.r[0], c.r[5]);
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[0] = (v >> 5) as u32; }
                c.r[4] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.r[0].wrapping_mul(0x64);
                c.r[5] = c.r[5].wrapping_sub(c.r[0]);
                pc = 0x80120818; continue;
                pc = 0x801207f4;
            }
            0x801207f4 => { ins += 5;
                c.r[4] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.r[4].wrapping_add(0x2);
                c.r[0] = c.lhz_t(ins, c.r[3].wrapping_add(c.r[0]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[5]);
                if !cr_bit(cr, 1) { pc = 0x80120814; continue; }
                pc = 0x80120808;
            }
            0x80120808 => { ins += 3;
                c.r[0] = c.r[7].rotate_left(1) & 0x1fffe;
                c.r[6] = c.lhz_t(ins, c.r[3].wrapping_add(c.r[0]));
                pc = 0x80120824; continue;
                pc = 0x80120814;
            }
            0x80120814 => { ins += 1;
                c.r[7] = c.r[7].wrapping_add(0x2);
                pc = 0x80120818;
            }
            0x80120818 => { ins += 3;
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x14);
                if cr_bit(cr, 0) { pc = 0x801207f4; continue; }
                pc = 0x80120824;
            }
            0x80120824 => { ins += 2;
                c.r[7] = c.r[6].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[7]);
                if cr_bit(cr, 2) { pc = 0x80120850; continue; }
                pc = 0x8012082c;
            }
            0x8012082c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x80120848; continue; }
                pc = 0x80120834;
            }
            0x80120834 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x82;
                c.r[6] = 0x0;
                c.lr = 0x80120848; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120848;
            }
            0x80120848 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80120854; continue;
                pc = 0x80120850;
            }
            0x80120850 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x80120854;
            }
            0x80120854 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x3c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x38));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80120674: bad pc {:#x}", pc),
        }
    }
}

/// pokemonAllKaihuku (0x8012086c, 660 bytes)
#[inline(never)]
pub fn f_8012086c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8012086c) { return; }
        if c.prof { crate::prof::enter(c, 0x8012086c); p = true; }
    }
    f_8012086c_b(c, 0x8012086c);
    if p { crate::prof::exit(c, 0x8012086c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8012086c_at(c: &mut Cpu, pc: u32) { f_8012086c_b(c, pc) }
#[inline(never)]
fn f_8012086c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8012086c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8012086c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8012086c => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                if cr_bit(cr, 2) { pc = 0x80120ae0; continue; }
                pc = 0x80120890;
            }
            0x80120890 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x801208a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801208a0;
            }
            0x801208a0 => { ins += 6;
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x801208b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801208b8;
            }
            0x801208b8 => { ins += 2;
                c.r[28] = 0x0;
                pc = 0x8012099c; continue;
                pc = 0x801208c0;
            }
            0x801208c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801208d0; continue; }
                pc = 0x801208c8;
            }
            0x801208c8 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80120920; continue;
                pc = 0x801208d0;
            }
            0x801208d0 => { ins += 6;
                c.r[30] = c.r[28].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[6] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x801208e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801208e8;
            }
            0x801208e8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801208f8; continue; }
                pc = 0x801208f0;
            }
            0x801208f0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80120920; continue;
                pc = 0x801208f8;
            }
            0x801208f8 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[6] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x8012090c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012090c;
            }
            0x8012090c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x163);
                if !cr_bit(cr, 2) { pc = 0x8012091c; continue; }
                pc = 0x80120914;
            }
            0x80120914 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80120920; continue;
                pc = 0x8012091c;
            }
            0x8012091c => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x80120920;
            }
            0x80120920 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80120998; continue; }
                pc = 0x8012092c;
            }
            0x8012092c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8012093c; continue; }
                pc = 0x80120934;
            }
            0x80120934 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80120980; continue;
                pc = 0x8012093c;
            }
            0x8012093c => { ins += 7;
                c.r[4] = c.r[28].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[29] = c.r[4].wrapping_add(0x4);
                c.r[4] = 0x0;
                c.r[6] = c.r[29].rotate_left(0) & 0xffff;
                c.r[5] = 0x7f;
                c.lr = 0x80120958; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120958;
            }
            0x80120958 => { ins += 6;
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.r[6] = c.r[29].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x81;
                c.lr = 0x80120970; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120970;
            }
            0x80120970 => { ins += 3;
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8012097c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bb6c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012097c;
            }
            0x8012097c => { ins += 1;
                c.r[0] = c.r[3] | c.r[3];
                pc = 0x80120980;
            }
            0x80120980 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[6] = c.r[28].rotate_left(0) & 0xff;
                c.r[7] = c.r[0].rotate_left(0) & 0xff;
                c.r[4] = 0x0;
                c.r[5] = 0x80;
                c.lr = 0x80120998; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120998;
            }
            0x80120998 => { ins += 1;
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x8012099c;
            }
            0x8012099c => { ins += 3;
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801208c0; continue; }
                pc = 0x801209a8;
            }
            0x801209a8 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x801209b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801209b0;
            }
            0x801209b0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801209d0; continue; }
                pc = 0x801209bc;
            }
            0x801209bc => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x801209c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801209c4;
            }
            0x801209c4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x801209dc; continue; }
                pc = 0x801209d0;
            }
            0x801209d0 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3;
                c.lr = 0x801209dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b788(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801209dc;
            }
            0x801209dc => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x801209e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801209e4;
            }
            0x801209e4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120a04; continue; }
                pc = 0x801209f0;
            }
            0x801209f0 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x801209f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801209f8;
            }
            0x801209f8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80120a10; continue; }
                pc = 0x80120a04;
            }
            0x80120a04 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4;
                c.lr = 0x80120a10; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b788(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120a10;
            }
            0x80120a10 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x80120a18; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120a18;
            }
            0x80120a18 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120a38; continue; }
                pc = 0x80120a24;
            }
            0x80120a24 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x80120a2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120a2c;
            }
            0x80120a2c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80120a44; continue; }
                pc = 0x80120a38;
            }
            0x80120a38 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x5;
                c.lr = 0x80120a44; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b788(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120a44;
            }
            0x80120a44 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80120a4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120a4c;
            }
            0x80120a4c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120a6c; continue; }
                pc = 0x80120a58;
            }
            0x80120a58 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80120a60; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120a60;
            }
            0x80120a60 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80120a78; continue; }
                pc = 0x80120a6c;
            }
            0x80120a6c => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x6;
                c.lr = 0x80120a78; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b788(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120a78;
            }
            0x80120a78 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80120a80; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120a80;
            }
            0x80120a80 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120aa0; continue; }
                pc = 0x80120a8c;
            }
            0x80120a8c => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80120a94; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120a94;
            }
            0x80120a94 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80120aac; continue; }
                pc = 0x80120aa0;
            }
            0x80120aa0 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x80120aac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b788(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120aac;
            }
            0x80120aac => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80120ab4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120ab4;
            }
            0x80120ab4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120ad4; continue; }
                pc = 0x80120ac0;
            }
            0x80120ac0 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80120ac8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120ac8;
            }
            0x80120ac8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80120ae0; continue; }
                pc = 0x80120ad4;
            }
            0x80120ad4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x80120ae0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b788(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120ae0;
            }
            0x80120ae0 => { ins += 9;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8012086c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetMezamerupower (0x80120b00, 364 bytes)
#[inline(never)]
pub fn f_80120b00(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80120b00) { return; }
        if c.prof { crate::prof::enter(c, 0x80120b00); p = true; }
    }
    f_80120b00_b(c, 0x80120b00);
    if p { crate::prof::exit(c, 0x80120b00); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80120b00_at(c: &mut Cpu, pc: u32) { f_80120b00_b(c, pc) }
#[inline(always)]
fn f_80120b00_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80120b00;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80120b00, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80120b00 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[24] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[5] = 0x93;
                c.lr = 0x80120b2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120b2c;
            }
            0x80120b2c => { ins += 6;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x94;
                c.r[6] = 0x0;
                c.lr = 0x80120b44; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120b44;
            }
            0x80120b44 => { ins += 6;
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x95;
                c.r[6] = 0x0;
                c.lr = 0x80120b5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120b5c;
            }
            0x80120b5c => { ins += 6;
                c.r[25] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x96;
                c.r[6] = 0x0;
                c.lr = 0x80120b74; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120b74;
            }
            0x80120b74 => { ins += 6;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x97;
                c.r[6] = 0x0;
                c.lr = 0x80120b8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120b8c;
            }
            0x80120b8c => { ins += 6;
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x98;
                c.r[6] = 0x0;
                c.lr = 0x80120ba4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120ba4;
            }
            0x80120ba4 => { ins += 49;
                c.r[4] = c.r[27].rotate_left(0) & 0x2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                { let v = c.r[4] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[5] = (v >> 1) as u32; }
                c.r[0] = c.r[26].rotate_left(1) & 0x2;
                c.r[5] = (c.r[26].rotate_left(0) & 0x2) | (c.r[5] & 0xfffffffd);
                c.r[4] = 0x82080000;
                c.r[5] = (c.r[25].rotate_left(1) & 0x4) | (c.r[5] & 0xfffffffb);
                c.r[0] = (c.r[27].rotate_left(0) & 0x1) | (c.r[0] & 0xfffffffe);
                c.r[5] = (c.r[3].rotate_left(2) & 0x8) | (c.r[5] & 0xfffffff7);
                c.r[6] = c.r[4].wrapping_add(0x2083);
                c.r[5] = (c.r[31].rotate_left(3) & 0x10) | (c.r[5] & 0xffffffef);
                c.r[0] = (c.r[25].rotate_left(2) & 0x4) | (c.r[0] & 0xfffffffb);
                c.r[0] = (c.r[3].rotate_left(3) & 0x8) | (c.r[0] & 0xfffffff7);
                c.r[5] = (c.r[30].rotate_left(4) & 0x20) | (c.r[5] & 0xffffffdf);
                c.r[0] = (c.r[31].rotate_left(4) & 0x10) | (c.r[0] & 0xffffffef);
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                c.r[0] = (c.r[30].rotate_left(5) & 0x20) | (c.r[0] & 0xffffffdf);
                c.r[4] = c.r[3].wrapping_mul(0x28);
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[0].wrapping_mul(0xf);
                c.r[5] = mulhw(c.r[6], c.r[4]);
                c.r[3] = mulhw(c.r[6], c.r[0]);
                c.r[4] = c.r[5].wrapping_add(c.r[4]);
                { let v = c.r[4] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[4] = (v >> 5) as u32; }
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.r[5] = c.r[4].rotate_left(1) & 0x1;
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[0] = (v >> 5) as u32; }
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[4] = c.r[4].wrapping_add(0x1e);
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x9);
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                c.r[0] = c.r[3] | c.r[3];
                if cr_bit(cr, 0) { pc = 0x80120c40; continue; }
                pc = 0x80120c38;
            }
            0x80120c38 => { ins += 2;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80120c40;
            }
            0x80120c40 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x80120c4c; continue; }
                pc = 0x80120c48;
            }
            0x80120c48 => { ins += 1;
                c.sth(c.r[28], c.r[4]);
                pc = 0x80120c4c;
            }
            0x80120c4c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x80120c58; continue; }
                pc = 0x80120c54;
            }
            0x80120c54 => { ins += 1;
                c.sth(c.r[29], c.r[0]);
                pc = 0x80120c58;
            }
            0x80120c58 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80120b00: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataCheckValid (0x80120c6c, 112 bytes)
#[inline(never)]
pub fn f_80120c6c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80120c6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80120c6c); p = true; }
    }
    f_80120c6c_b(c, 0x80120c6c);
    if p { crate::prof::exit(c, 0x80120c6c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80120c6c_at(c: &mut Cpu, pc: u32) { f_80120c6c_b(c, pc) }
#[inline(always)]
fn f_80120c6c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80120c6c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80120c6c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80120c6c => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x80120c90; continue; }
                pc = 0x80120c88;
            }
            0x80120c88 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120cc8; continue;
                pc = 0x80120c90;
            }
            0x80120c90 => { ins += 3;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80120c9c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120c9c;
            }
            0x80120c9c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80120cac; continue; }
                pc = 0x80120ca4;
            }
            0x80120ca4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120cc8; continue;
                pc = 0x80120cac;
            }
            0x80120cac => { ins += 7;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8770));
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.r[3] ^ c.r[0];
                c.r[0] = c.r[0].leading_zeros();
                c.r[0] = slw(c.r[3], c.r[0]);
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                pc = 0x80120cc8;
            }
            0x80120cc8 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80120c6c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetNowLevelToExp (0x80120cdc, 144 bytes)
#[inline(never)]
pub fn f_80120cdc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80120cdc) { return; }
        if c.prof { crate::prof::enter(c, 0x80120cdc); p = true; }
    }
    f_80120cdc_b(c, 0x80120cdc);
    if p { crate::prof::exit(c, 0x80120cdc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80120cdc_at(c: &mut Cpu, pc: u32) { f_80120cdc_b(c, pc) }
#[inline(always)]
fn f_80120cdc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80120cdc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80120cdc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80120cdc => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x80120d04; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120d04;
            }
            0x80120d04 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x80120d18; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120d18;
            }
            0x80120d18 => { ins += 6;
                c.r[31] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x7a;
                c.r[6] = 0x0;
                c.lr = 0x80120d30; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120d30;
            }
            0x80120d30 => { ins += 3;
                c.r[30] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80120d3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120d3c;
            }
            0x80120d3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80120d4c; continue; }
                pc = 0x80120d44;
            }
            0x80120d44 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120d54; continue;
                pc = 0x80120d4c;
            }
            0x80120d4c => { ins += 2;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80120d54; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120d54;
            }
            0x80120d54 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80120cdc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetTasteLike (0x80120d6c, 100 bytes)
#[inline(never)]
pub fn f_80120d6c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80120d6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80120d6c); p = true; }
    }
    f_80120d6c_b(c, 0x80120d6c);
    if p { crate::prof::exit(c, 0x80120d6c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80120d6c_at(c: &mut Cpu, pc: u32) { f_80120d6c_b(c, pc) }
#[inline(always)]
fn f_80120d6c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80120d6c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80120d6c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80120d6c => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x80120d94; continue; }
                pc = 0x80120d8c;
            }
            0x80120d8c => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x80120db8; continue;
                pc = 0x80120d94;
            }
            0x80120d94 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0xbf;
                c.r[6] = 0x0;
                c.lr = 0x80120da4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120da4;
            }
            0x80120da4 => { ins += 3;
                c.r[31] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80120db0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80143b08(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120db0;
            }
            0x80120db0 => { ins += 2;
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80120db8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80143abc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120db8;
            }
            0x80120db8 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80120d6c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetJoutaiMsgId (0x80120dd0, 528 bytes)
#[inline(never)]
pub fn f_80120dd0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80120dd0) { return; }
        if c.prof { crate::prof::enter(c, 0x80120dd0); p = true; }
    }
    f_80120dd0_b(c, 0x80120dd0);
    if p { crate::prof::exit(c, 0x80120dd0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80120dd0_at(c: &mut Cpu, pc: u32) { f_80120dd0_b(c, pc) }
#[inline(never)]
fn f_80120dd0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80120dd0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80120dd0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80120dd0 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x80120df8; continue; }
                pc = 0x80120df0;
            }
            0x80120df0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120fc8; continue;
                pc = 0x80120df8;
            }
            0x80120df8 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80120e00; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120e00;
            }
            0x80120e00 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120e28; continue; }
                pc = 0x80120e0c;
            }
            0x80120e0c => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80120e14; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120e14;
            }
            0x80120e14 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80120e28; continue; }
                pc = 0x80120e20;
            }
            0x80120e20 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120e34; continue;
                pc = 0x80120e28;
            }
            0x80120e28 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x3;
                c.lr = 0x80120e34; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120e34;
            }
            0x80120e34 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80120e44; continue; }
                pc = 0x80120e40;
            }
            0x80120e40 => { ins += 1;
                c.r[31] = 0x3;
                pc = 0x80120e44;
            }
            0x80120e44 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80120e4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120e4c;
            }
            0x80120e4c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120e74; continue; }
                pc = 0x80120e58;
            }
            0x80120e58 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80120e60; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120e60;
            }
            0x80120e60 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80120e74; continue; }
                pc = 0x80120e6c;
            }
            0x80120e6c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120e80; continue;
                pc = 0x80120e74;
            }
            0x80120e74 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x4;
                c.lr = 0x80120e80; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120e80;
            }
            0x80120e80 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80120e90; continue; }
                pc = 0x80120e8c;
            }
            0x80120e8c => { ins += 1;
                c.r[31] = 0x4;
                pc = 0x80120e90;
            }
            0x80120e90 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80120e98; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120e98;
            }
            0x80120e98 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120ec0; continue; }
                pc = 0x80120ea4;
            }
            0x80120ea4 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80120eac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120eac;
            }
            0x80120eac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80120ec0; continue; }
                pc = 0x80120eb8;
            }
            0x80120eb8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120ecc; continue;
                pc = 0x80120ec0;
            }
            0x80120ec0 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x8;
                c.lr = 0x80120ecc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120ecc;
            }
            0x80120ecc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80120edc; continue; }
                pc = 0x80120ed8;
            }
            0x80120ed8 => { ins += 1;
                c.r[31] = 0x8;
                pc = 0x80120edc;
            }
            0x80120edc => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x80120ee4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120ee4;
            }
            0x80120ee4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120f0c; continue; }
                pc = 0x80120ef0;
            }
            0x80120ef0 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x80120ef8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120ef8;
            }
            0x80120ef8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80120f0c; continue; }
                pc = 0x80120f04;
            }
            0x80120f04 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120f18; continue;
                pc = 0x80120f0c;
            }
            0x80120f0c => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x5;
                c.lr = 0x80120f18; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120f18;
            }
            0x80120f18 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80120f28; continue; }
                pc = 0x80120f24;
            }
            0x80120f24 => { ins += 1;
                c.r[31] = 0x5;
                pc = 0x80120f28;
            }
            0x80120f28 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80120f30; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120f30;
            }
            0x80120f30 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120f58; continue; }
                pc = 0x80120f3c;
            }
            0x80120f3c => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80120f44; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120f44;
            }
            0x80120f44 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80120f58; continue; }
                pc = 0x80120f50;
            }
            0x80120f50 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120f64; continue;
                pc = 0x80120f58;
            }
            0x80120f58 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x6;
                c.lr = 0x80120f64; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120f64;
            }
            0x80120f64 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80120f74; continue; }
                pc = 0x80120f70;
            }
            0x80120f70 => { ins += 1;
                c.r[31] = 0x6;
                pc = 0x80120f74;
            }
            0x80120f74 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80120f7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120f7c;
            }
            0x80120f7c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80120fa4; continue; }
                pc = 0x80120f88;
            }
            0x80120f88 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80120f90; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120f90;
            }
            0x80120f90 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80120fa4; continue; }
                pc = 0x80120f9c;
            }
            0x80120f9c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80120fb0; continue;
                pc = 0x80120fa4;
            }
            0x80120fa4 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x7;
                c.lr = 0x80120fb0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120fb0;
            }
            0x80120fb0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80120fc0; continue; }
                pc = 0x80120fbc;
            }
            0x80120fbc => { ins += 1;
                c.r[31] = 0x7;
                pc = 0x80120fc0;
            }
            0x80120fc0 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80120fc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119f50(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80120fc8;
            }
            0x80120fc8 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80120dd0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetJoutaiMenuSpriteId (0x80120fe0, 536 bytes)
#[inline(never)]
pub fn f_80120fe0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80120fe0) { return; }
        if c.prof { crate::prof::enter(c, 0x80120fe0); p = true; }
    }
    f_80120fe0_b(c, 0x80120fe0);
    if p { crate::prof::exit(c, 0x80120fe0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80120fe0_at(c: &mut Cpu, pc: u32) { f_80120fe0_b(c, pc) }
#[inline(never)]
fn f_80120fe0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80120fe0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80120fe0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80120fe0 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x80121000; continue; }
                pc = 0x80120ff8;
            }
            0x80120ff8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801211e4; continue;
                pc = 0x80121000;
            }
            0x80121000 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80121008; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121008;
            }
            0x80121008 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121030; continue; }
                pc = 0x80121014;
            }
            0x80121014 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x8012101c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012101c;
            }
            0x8012101c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121030; continue; }
                pc = 0x80121028;
            }
            0x80121028 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8012103c; continue;
                pc = 0x80121030;
            }
            0x80121030 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3;
                c.lr = 0x8012103c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012103c;
            }
            0x8012103c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121050; continue; }
                pc = 0x80121048;
            }
            0x80121048 => { ins += 2;
                c.r[3] = 0x3a;
                pc = 0x801211e4; continue;
                pc = 0x80121050;
            }
            0x80121050 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80121058; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121058;
            }
            0x80121058 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121080; continue; }
                pc = 0x80121064;
            }
            0x80121064 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x8012106c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012106c;
            }
            0x8012106c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121080; continue; }
                pc = 0x80121078;
            }
            0x80121078 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8012108c; continue;
                pc = 0x80121080;
            }
            0x80121080 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4;
                c.lr = 0x8012108c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012108c;
            }
            0x8012108c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801210a0; continue; }
                pc = 0x80121098;
            }
            0x80121098 => { ins += 2;
                c.r[3] = 0x3a;
                pc = 0x801211e4; continue;
                pc = 0x801210a0;
            }
            0x801210a0 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x801210a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801210a8;
            }
            0x801210a8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801210d0; continue; }
                pc = 0x801210b4;
            }
            0x801210b4 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x801210bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801210bc;
            }
            0x801210bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801210d0; continue; }
                pc = 0x801210c8;
            }
            0x801210c8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801210dc; continue;
                pc = 0x801210d0;
            }
            0x801210d0 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x5;
                c.lr = 0x801210dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801210dc;
            }
            0x801210dc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801210f0; continue; }
                pc = 0x801210e8;
            }
            0x801210e8 => { ins += 2;
                c.r[3] = 0x3b;
                pc = 0x801211e4; continue;
                pc = 0x801210f0;
            }
            0x801210f0 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x801210f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801210f8;
            }
            0x801210f8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121120; continue; }
                pc = 0x80121104;
            }
            0x80121104 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x8012110c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012110c;
            }
            0x8012110c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121120; continue; }
                pc = 0x80121118;
            }
            0x80121118 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8012112c; continue;
                pc = 0x80121120;
            }
            0x80121120 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x6;
                c.lr = 0x8012112c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012112c;
            }
            0x8012112c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121140; continue; }
                pc = 0x80121138;
            }
            0x80121138 => { ins += 2;
                c.r[3] = 0x3c;
                pc = 0x801211e4; continue;
                pc = 0x80121140;
            }
            0x80121140 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80121148; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121148;
            }
            0x80121148 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121170; continue; }
                pc = 0x80121154;
            }
            0x80121154 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x8012115c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012115c;
            }
            0x8012115c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121170; continue; }
                pc = 0x80121168;
            }
            0x80121168 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8012117c; continue;
                pc = 0x80121170;
            }
            0x80121170 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x8012117c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012117c;
            }
            0x8012117c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121190; continue; }
                pc = 0x80121188;
            }
            0x80121188 => { ins += 2;
                c.r[3] = 0x3d;
                pc = 0x801211e4; continue;
                pc = 0x80121190;
            }
            0x80121190 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80121198; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121198;
            }
            0x80121198 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801211c0; continue; }
                pc = 0x801211a4;
            }
            0x801211a4 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x801211ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801211ac;
            }
            0x801211ac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801211c0; continue; }
                pc = 0x801211b8;
            }
            0x801211b8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801211cc; continue;
                pc = 0x801211c0;
            }
            0x801211c0 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x801211cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801211cc;
            }
            0x801211cc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801211e0; continue; }
                pc = 0x801211d8;
            }
            0x801211d8 => { ins += 2;
                c.r[3] = 0x3e;
                pc = 0x801211e4; continue;
                pc = 0x801211e0;
            }
            0x801211e0 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801211e4;
            }
            0x801211e4 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80120fe0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetJoutaiDataId (0x801211f8, 536 bytes)
#[inline(never)]
pub fn f_801211f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801211f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801211f8); p = true; }
    }
    f_801211f8_b(c, 0x801211f8);
    if p { crate::prof::exit(c, 0x801211f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801211f8_at(c: &mut Cpu, pc: u32) { f_801211f8_b(c, pc) }
#[inline(never)]
fn f_801211f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801211f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801211f8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801211f8 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x80121218; continue; }
                pc = 0x80121210;
            }
            0x80121210 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801213fc; continue;
                pc = 0x80121218;
            }
            0x80121218 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80121220; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121220;
            }
            0x80121220 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121248; continue; }
                pc = 0x8012122c;
            }
            0x8012122c => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80121234; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121234;
            }
            0x80121234 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121248; continue; }
                pc = 0x80121240;
            }
            0x80121240 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121254; continue;
                pc = 0x80121248;
            }
            0x80121248 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3;
                c.lr = 0x80121254; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121254;
            }
            0x80121254 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121268; continue; }
                pc = 0x80121260;
            }
            0x80121260 => { ins += 2;
                c.r[3] = 0x3;
                pc = 0x801213fc; continue;
                pc = 0x80121268;
            }
            0x80121268 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80121270; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121270;
            }
            0x80121270 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121298; continue; }
                pc = 0x8012127c;
            }
            0x8012127c => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80121284; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121284;
            }
            0x80121284 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121298; continue; }
                pc = 0x80121290;
            }
            0x80121290 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801212a4; continue;
                pc = 0x80121298;
            }
            0x80121298 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4;
                c.lr = 0x801212a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801212a4;
            }
            0x801212a4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801212b8; continue; }
                pc = 0x801212b0;
            }
            0x801212b0 => { ins += 2;
                c.r[3] = 0x4;
                pc = 0x801213fc; continue;
                pc = 0x801212b8;
            }
            0x801212b8 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x801212c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801212c0;
            }
            0x801212c0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801212e8; continue; }
                pc = 0x801212cc;
            }
            0x801212cc => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x801212d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801212d4;
            }
            0x801212d4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801212e8; continue; }
                pc = 0x801212e0;
            }
            0x801212e0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801212f4; continue;
                pc = 0x801212e8;
            }
            0x801212e8 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x5;
                c.lr = 0x801212f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801212f4;
            }
            0x801212f4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121308; continue; }
                pc = 0x80121300;
            }
            0x80121300 => { ins += 2;
                c.r[3] = 0x5;
                pc = 0x801213fc; continue;
                pc = 0x80121308;
            }
            0x80121308 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80121310; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121310;
            }
            0x80121310 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121338; continue; }
                pc = 0x8012131c;
            }
            0x8012131c => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80121324; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121324;
            }
            0x80121324 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121338; continue; }
                pc = 0x80121330;
            }
            0x80121330 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121344; continue;
                pc = 0x80121338;
            }
            0x80121338 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x6;
                c.lr = 0x80121344; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121344;
            }
            0x80121344 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121358; continue; }
                pc = 0x80121350;
            }
            0x80121350 => { ins += 2;
                c.r[3] = 0x6;
                pc = 0x801213fc; continue;
                pc = 0x80121358;
            }
            0x80121358 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80121360; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121360;
            }
            0x80121360 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121388; continue; }
                pc = 0x8012136c;
            }
            0x8012136c => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80121374; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121374;
            }
            0x80121374 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121388; continue; }
                pc = 0x80121380;
            }
            0x80121380 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121394; continue;
                pc = 0x80121388;
            }
            0x80121388 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x80121394; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121394;
            }
            0x80121394 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801213a8; continue; }
                pc = 0x801213a0;
            }
            0x801213a0 => { ins += 2;
                c.r[3] = 0x7;
                pc = 0x801213fc; continue;
                pc = 0x801213a8;
            }
            0x801213a8 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x801213b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801213b0;
            }
            0x801213b0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801213d8; continue; }
                pc = 0x801213bc;
            }
            0x801213bc => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x801213c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801213c4;
            }
            0x801213c4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801213d8; continue; }
                pc = 0x801213d0;
            }
            0x801213d0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801213e4; continue;
                pc = 0x801213d8;
            }
            0x801213d8 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x801213e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801213e4;
            }
            0x801213e4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801213f8; continue; }
                pc = 0x801213f0;
            }
            0x801213f0 => { ins += 2;
                c.r[3] = 0x8;
                pc = 0x801213fc; continue;
                pc = 0x801213f8;
            }
            0x801213f8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801213fc;
            }
            0x801213fc => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801211f8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonIsHpMantan (0x80121410, 116 bytes)
#[inline(never)]
pub fn f_80121410(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80121410) { return; }
        if c.prof { crate::prof::enter(c, 0x80121410); p = true; }
    }
    f_80121410_b(c, 0x80121410);
    if p { crate::prof::exit(c, 0x80121410); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80121410_at(c: &mut Cpu, pc: u32) { f_80121410_b(c, pc) }
#[inline(always)]
fn f_80121410_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80121410;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80121410, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80121410 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x80121434; continue; }
                pc = 0x8012142c;
            }
            0x8012142c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8012146c; continue;
                pc = 0x80121434;
            }
            0x80121434 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x80121444; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121444;
            }
            0x80121444 => { ins += 6;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x8012145c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012145c;
            }
            0x8012145c => { ins += 4;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].wrapping_sub(c.r[31]);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0xff;
                pc = 0x8012146c;
            }
            0x8012146c => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80121410: bad pc {:#x}", pc),
        }
    }
}

/// pokemonCopyJoutai (0x80121484, 120 bytes)
#[inline(never)]
pub fn f_80121484(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80121484) { return; }
        if c.prof { crate::prof::enter(c, 0x80121484); p = true; }
    }
    f_80121484_b(c, 0x80121484);
    if p { crate::prof::exit(c, 0x80121484); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80121484_at(c: &mut Cpu, pc: u32) { f_80121484_b(c, pc) }
#[inline(always)]
fn f_80121484_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80121484;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80121484, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80121484 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801214b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801214b0;
            }
            0x801214b0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801214d0; continue; }
                pc = 0x801214bc;
            }
            0x801214bc => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801214c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801214c4;
            }
            0x801214c4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x801214e0; continue; }
                pc = 0x801214d0;
            }
            0x801214d0 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x801214e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011a0a8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801214e0;
            }
            0x801214e0 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80121484: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSetJoutai08 (0x801214fc, 120 bytes)
#[inline(never)]
pub fn f_801214fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801214fc) { return; }
        if c.prof { crate::prof::enter(c, 0x801214fc); p = true; }
    }
    f_801214fc_b(c, 0x801214fc);
    if p { crate::prof::exit(c, 0x801214fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801214fc_at(c: &mut Cpu, pc: u32) { f_801214fc_b(c, pc) }
#[inline(always)]
fn f_801214fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801214fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801214fc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801214fc => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80121528; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121528;
            }
            0x80121528 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121548; continue; }
                pc = 0x80121534;
            }
            0x80121534 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8012153c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012153c;
            }
            0x8012153c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80121558; continue; }
                pc = 0x80121548;
            }
            0x80121548 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80121558; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011a280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121558;
            }
            0x80121558 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801214fc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetJoutai08 (0x80121574, 112 bytes)
#[inline(never)]
pub fn f_80121574(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80121574) { return; }
        if c.prof { crate::prof::enter(c, 0x80121574); p = true; }
    }
    f_80121574_b(c, 0x80121574);
    if p { crate::prof::exit(c, 0x80121574); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80121574_at(c: &mut Cpu, pc: u32) { f_80121574_b(c, pc) }
#[inline(always)]
fn f_80121574_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80121574;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80121574, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80121574 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121598; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121598;
            }
            0x80121598 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801215c0; continue; }
                pc = 0x801215a4;
            }
            0x801215a4 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801215ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801215ac;
            }
            0x801215ac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801215c0; continue; }
                pc = 0x801215b8;
            }
            0x801215b8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801215cc; continue;
                pc = 0x801215c0;
            }
            0x801215c0 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801215cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011a3e4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801215cc;
            }
            0x801215cc => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80121574: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSetJoutai0C (0x801215e4, 120 bytes)
#[inline(never)]
pub fn f_801215e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801215e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801215e4); p = true; }
    }
    f_801215e4_b(c, 0x801215e4);
    if p { crate::prof::exit(c, 0x801215e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801215e4_at(c: &mut Cpu, pc: u32) { f_801215e4_b(c, pc) }
#[inline(always)]
fn f_801215e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801215e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801215e4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801215e4 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80121610; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121610;
            }
            0x80121610 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121630; continue; }
                pc = 0x8012161c;
            }
            0x8012161c => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80121624; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121624;
            }
            0x80121624 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80121640; continue; }
                pc = 0x80121630;
            }
            0x80121630 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80121640; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011a570(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121640;
            }
            0x80121640 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801215e4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetJoutai0C (0x8012165c, 112 bytes)
#[inline(never)]
pub fn f_8012165c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8012165c) { return; }
        if c.prof { crate::prof::enter(c, 0x8012165c); p = true; }
    }
    f_8012165c_b(c, 0x8012165c);
    if p { crate::prof::exit(c, 0x8012165c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8012165c_at(c: &mut Cpu, pc: u32) { f_8012165c_b(c, pc) }
#[inline(always)]
fn f_8012165c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8012165c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8012165c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8012165c => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121680; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121680;
            }
            0x80121680 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801216a8; continue; }
                pc = 0x8012168c;
            }
            0x8012168c => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121694; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121694;
            }
            0x80121694 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801216a8; continue; }
                pc = 0x801216a0;
            }
            0x801216a0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801216b4; continue;
                pc = 0x801216a8;
            }
            0x801216a8 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801216b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011a6d4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801216b4;
            }
            0x801216b4 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8012165c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetJoutaiParam (0x801216cc, 112 bytes)
#[inline(never)]
pub fn f_801216cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801216cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801216cc); p = true; }
    }
    f_801216cc_b(c, 0x801216cc);
    if p { crate::prof::exit(c, 0x801216cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801216cc_at(c: &mut Cpu, pc: u32) { f_801216cc_b(c, pc) }
#[inline(always)]
fn f_801216cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801216cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801216cc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801216cc => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801216f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801216f0;
            }
            0x801216f0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121718; continue; }
                pc = 0x801216fc;
            }
            0x801216fc => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121704; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121704;
            }
            0x80121704 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121718; continue; }
                pc = 0x80121710;
            }
            0x80121710 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121724; continue;
                pc = 0x80121718;
            }
            0x80121718 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80121724; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011a860(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121724;
            }
            0x80121724 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801216cc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSetJoutaiKaisuu (0x8012173c, 120 bytes)
#[inline(never)]
pub fn f_8012173c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8012173c) { return; }
        if c.prof { crate::prof::enter(c, 0x8012173c); p = true; }
    }
    f_8012173c_b(c, 0x8012173c);
    if p { crate::prof::exit(c, 0x8012173c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8012173c_at(c: &mut Cpu, pc: u32) { f_8012173c_b(c, pc) }
#[inline(always)]
fn f_8012173c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8012173c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8012173c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8012173c => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80121768; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121768;
            }
            0x80121768 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121788; continue; }
                pc = 0x80121774;
            }
            0x80121774 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8012177c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012177c;
            }
            0x8012177c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80121798; continue; }
                pc = 0x80121788;
            }
            0x80121788 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80121798; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011a9ec(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121798;
            }
            0x80121798 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8012173c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSetJoutai05 (0x801217b4, 120 bytes)
#[inline(never)]
pub fn f_801217b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801217b4) { return; }
        if c.prof { crate::prof::enter(c, 0x801217b4); p = true; }
    }
    f_801217b4_b(c, 0x801217b4);
    if p { crate::prof::exit(c, 0x801217b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801217b4_at(c: &mut Cpu, pc: u32) { f_801217b4_b(c, pc) }
#[inline(always)]
fn f_801217b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801217b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801217b4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801217b4 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801217e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801217e0;
            }
            0x801217e0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121800; continue; }
                pc = 0x801217ec;
            }
            0x801217ec => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801217f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801217f4;
            }
            0x801217f4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80121810; continue; }
                pc = 0x80121800;
            }
            0x80121800 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80121810; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ab50(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121810;
            }
            0x80121810 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801217b4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetJoutai05 (0x8012182c, 112 bytes)
#[inline(never)]
pub fn f_8012182c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8012182c) { return; }
        if c.prof { crate::prof::enter(c, 0x8012182c); p = true; }
    }
    f_8012182c_b(c, 0x8012182c);
    if p { crate::prof::exit(c, 0x8012182c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8012182c_at(c: &mut Cpu, pc: u32) { f_8012182c_b(c, pc) }
#[inline(always)]
fn f_8012182c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8012182c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8012182c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8012182c => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121850; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121850;
            }
            0x80121850 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121878; continue; }
                pc = 0x8012185c;
            }
            0x8012185c => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121864; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121864;
            }
            0x80121864 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121878; continue; }
                pc = 0x80121870;
            }
            0x80121870 => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x80121884; continue;
                pc = 0x80121878;
            }
            0x80121878 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80121884; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011acb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121884;
            }
            0x80121884 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8012182c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetJoutaiKaisuu (0x8012189c, 112 bytes)
#[inline(never)]
pub fn f_8012189c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8012189c) { return; }
        if c.prof { crate::prof::enter(c, 0x8012189c); p = true; }
    }
    f_8012189c_b(c, 0x8012189c);
    if p { crate::prof::exit(c, 0x8012189c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8012189c_at(c: &mut Cpu, pc: u32) { f_8012189c_b(c, pc) }
#[inline(always)]
fn f_8012189c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8012189c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8012189c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8012189c => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801218c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801218c0;
            }
            0x801218c0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801218e8; continue; }
                pc = 0x801218cc;
            }
            0x801218cc => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801218d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801218d4;
            }
            0x801218d4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801218e8; continue; }
                pc = 0x801218e0;
            }
            0x801218e0 => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x801218f4; continue;
                pc = 0x801218e8;
            }
            0x801218e8 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801218f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ae40(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801218f4;
            }
            0x801218f4 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8012189c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSetJoutaiStack (0x8012190c, 120 bytes)
#[inline(never)]
pub fn f_8012190c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8012190c) { return; }
        if c.prof { crate::prof::enter(c, 0x8012190c); p = true; }
    }
    f_8012190c_b(c, 0x8012190c);
    if p { crate::prof::exit(c, 0x8012190c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8012190c_at(c: &mut Cpu, pc: u32) { f_8012190c_b(c, pc) }
#[inline(always)]
fn f_8012190c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8012190c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8012190c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8012190c => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80121938; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121938;
            }
            0x80121938 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121958; continue; }
                pc = 0x80121944;
            }
            0x80121944 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8012194c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012194c;
            }
            0x8012194c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80121968; continue; }
                pc = 0x80121958;
            }
            0x80121958 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80121968; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011afcc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121968;
            }
            0x80121968 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8012190c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetJoutaiStack (0x80121984, 112 bytes)
#[inline(never)]
pub fn f_80121984(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80121984) { return; }
        if c.prof { crate::prof::enter(c, 0x80121984); p = true; }
    }
    f_80121984_b(c, 0x80121984);
    if p { crate::prof::exit(c, 0x80121984); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80121984_at(c: &mut Cpu, pc: u32) { f_80121984_b(c, pc) }
#[inline(always)]
fn f_80121984_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80121984;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80121984, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80121984 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801219a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801219a8;
            }
            0x801219a8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801219d0; continue; }
                pc = 0x801219b4;
            }
            0x801219b4 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801219bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801219bc;
            }
            0x801219bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801219d0; continue; }
                pc = 0x801219c8;
            }
            0x801219c8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801219dc; continue;
                pc = 0x801219d0;
            }
            0x801219d0 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801219dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b130(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801219dc;
            }
            0x801219dc => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80121984: bad pc {:#x}", pc),
        }
    }
}

/// pokemonWriteJoutaiDataId (0x801219f4, 120 bytes)
#[inline(never)]
pub fn f_801219f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801219f4) { return; }
        if c.prof { crate::prof::enter(c, 0x801219f4); p = true; }
    }
    f_801219f4_b(c, 0x801219f4);
    if p { crate::prof::exit(c, 0x801219f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801219f4_at(c: &mut Cpu, pc: u32) { f_801219f4_b(c, pc) }
#[inline(always)]
fn f_801219f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801219f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801219f4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801219f4 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80121a20; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121a20;
            }
            0x80121a20 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121a40; continue; }
                pc = 0x80121a2c;
            }
            0x80121a2c => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80121a34; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121a34;
            }
            0x80121a34 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80121a50; continue; }
                pc = 0x80121a40;
            }
            0x80121a40 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80121a50; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b2c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121a50;
            }
            0x80121a50 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801219f4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonCheckWriteJoutaiDataId (0x80121a6c, 112 bytes)
#[inline(never)]
pub fn f_80121a6c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80121a6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80121a6c); p = true; }
    }
    f_80121a6c_b(c, 0x80121a6c);
    if p { crate::prof::exit(c, 0x80121a6c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80121a6c_at(c: &mut Cpu, pc: u32) { f_80121a6c_b(c, pc) }
#[inline(always)]
fn f_80121a6c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80121a6c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80121a6c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80121a6c => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121a90; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121a90;
            }
            0x80121a90 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121ab8; continue; }
                pc = 0x80121a9c;
            }
            0x80121a9c => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121aa4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121aa4;
            }
            0x80121aa4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121ab8; continue; }
                pc = 0x80121ab0;
            }
            0x80121ab0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121ac4; continue;
                pc = 0x80121ab8;
            }
            0x80121ab8 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80121ac4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b444(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121ac4;
            }
            0x80121ac4 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80121a6c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonIsJoutaiDataId (0x80121adc, 112 bytes)
#[inline(never)]
pub fn f_80121adc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80121adc) { return; }
        if c.prof { crate::prof::enter(c, 0x80121adc); p = true; }
    }
    f_80121adc_b(c, 0x80121adc);
    if p { crate::prof::exit(c, 0x80121adc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80121adc_at(c: &mut Cpu, pc: u32) { f_80121adc_b(c, pc) }
#[inline(always)]
fn f_80121adc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80121adc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80121adc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80121adc => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121b00; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121b00;
            }
            0x80121b00 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121b28; continue; }
                pc = 0x80121b0c;
            }
            0x80121b0c => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121b14; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121b14;
            }
            0x80121b14 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121b28; continue; }
                pc = 0x80121b20;
            }
            0x80121b20 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121b34; continue;
                pc = 0x80121b28;
            }
            0x80121b28 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80121b34; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121b34;
            }
            0x80121b34 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80121adc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonInitPersistentJoutai (0x80121b4c, 104 bytes)
#[inline(never)]
pub fn f_80121b4c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80121b4c) { return; }
        if c.prof { crate::prof::enter(c, 0x80121b4c); p = true; }
    }
    f_80121b4c_b(c, 0x80121b4c);
    if p { crate::prof::exit(c, 0x80121b4c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80121b4c_at(c: &mut Cpu, pc: u32) { f_80121b4c_b(c, pc) }
#[inline(always)]
fn f_80121b4c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80121b4c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80121b4c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80121b4c => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121b70; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121b70;
            }
            0x80121b70 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121b90; continue; }
                pc = 0x80121b7c;
            }
            0x80121b7c => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80121b84; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121b84;
            }
            0x80121b84 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if !cr_bit(cr, 2) { pc = 0x80121b9c; continue; }
                pc = 0x80121b90;
            }
            0x80121b90 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80121b9c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b788(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121b9c;
            }
            0x80121b9c => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80121b4c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonReplace (0x80121bb4, 100 bytes)
#[inline(never)]
pub fn f_80121bb4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80121bb4) { return; }
        if c.prof { crate::prof::enter(c, 0x80121bb4); p = true; }
    }
    f_80121bb4_b(c, 0x80121bb4);
    if p { crate::prof::exit(c, 0x80121bb4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80121bb4_at(c: &mut Cpu, pc: u32) { f_80121bb4_b(c, pc) }
#[inline(always)]
fn f_80121bb4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80121bb4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80121bb4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80121bb4 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffeb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x154), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x14c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x148), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x80121c00; continue; }
                pc = 0x80121bd4;
            }
            0x80121bd4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x80121c00; continue; }
                pc = 0x80121bdc;
            }
            0x80121bdc => { ins += 3;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80121be8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011f5fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121be8;
            }
            0x80121be8 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80121bf4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011f5fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121bf4;
            }
            0x80121bf4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80121c00; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011f5fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121c00;
            }
            0x80121c00 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x154));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x14c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x148));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x150);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80121bb4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonCreateSequence (0x80121c18, 1064 bytes)
#[inline(never)]
pub fn f_80121c18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80121c18) { return; }
        if c.prof { crate::prof::enter(c, 0x80121c18); p = true; }
    }
    f_80121c18_b(c, 0x80121c18);
    if p { crate::prof::exit(c, 0x80121c18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80121c18_at(c: &mut Cpu, pc: u32) { f_80121c18_b(c, pc) }
#[inline(never)]
fn f_80121c18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80121c18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80121c18, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80121c18 => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x6f;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.lr = 0x80121c48; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121c48;
            }
            0x80121c48 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80121c64; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121c64;
            }
            0x80121c64 => { ins += 2;
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[28]);
                if !cr_bit(cr, 2) { pc = 0x80121c74; continue; }
                pc = 0x80121c6c;
            }
            0x80121c6c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80121cb4; continue;
                pc = 0x80121c74;
            }
            0x80121c74 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80121c88; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121c88;
            }
            0x80121c88 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80121c98; continue; }
                pc = 0x80121c90;
            }
            0x80121c90 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80121cb4; continue;
                pc = 0x80121c98;
            }
            0x80121c98 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8770));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80121cb0; continue; }
                pc = 0x80121ca8;
            }
            0x80121ca8 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80121cb4; continue;
                pc = 0x80121cb0;
            }
            0x80121cb0 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x80121cb4;
            }
            0x80121cb4 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80121cc4; continue; }
                pc = 0x80121cbc;
            }
            0x80121cbc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122020; continue;
                pc = 0x80121cc4;
            }
            0x80121cc4 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x66;
                c.r[6] = 0x0;
                c.lr = 0x80121cd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121cd8;
            }
            0x80121cd8 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                c.r[28] = c.r[3] | c.r[3];
                if !cr_bit(cr, 2) { pc = 0x80121cec; continue; }
                pc = 0x80121ce4;
            }
            0x80121ce4 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x80121d48; continue;
                pc = 0x80121cec;
            }
            0x80121cec => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x75;
                c.r[6] = 0x0;
                c.lr = 0x80121d00; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121d00;
            }
            0x80121d00 => { ins += 6;
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x6f;
                c.r[6] = 0x0;
                c.lr = 0x80121d18; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121d18;
            }
            0x80121d18 => { ins += 12;
                c.r[4] = c.r[29].rotate_left(16) & 0xffff;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.r[5] = c.r[3].rotate_left(16) & 0xffff;
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[4] ^ c.r[0];
                c.r[3] = 0x8;
                c.r[0] = c.r[5] ^ c.r[0];
                c.r[0] = c.r[6] ^ c.r[0];
                c.r[0] = c.r[3] ^ c.r[0];
                c.r[0] = c.r[0].leading_zeros();
                c.r[0] = slw(c.r[3], c.r[0]);
                c.r[5] = c.r[0].rotate_left(1) & 0x1;
                pc = 0x80121d48;
            }
            0x80121d48 => { ins += 3;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[28].rotate_left(0) & 0xffff;
                c.lr = 0x80121d54; { c.steps = st; c.cycles = ins; c.cr = cr; f_801de190(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121d54;
            }
            0x80121d54 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x8;
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x80121d64; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121d64;
            }
            0x80121d64 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121d8c; continue; }
                pc = 0x80121d70;
            }
            0x80121d70 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80121d78; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121d78;
            }
            0x80121d78 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121d8c; continue; }
                pc = 0x80121d84;
            }
            0x80121d84 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121d98; continue;
                pc = 0x80121d8c;
            }
            0x80121d8c => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x80121d98; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121d98;
            }
            0x80121d98 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121db0; continue; }
                pc = 0x80121da4;
            }
            0x80121da4 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1;
                c.lr = 0x80121db0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da3cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121db0;
            }
            0x80121db0 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80121db8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121db8;
            }
            0x80121db8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121de0; continue; }
                pc = 0x80121dc4;
            }
            0x80121dc4 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80121dcc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121dcc;
            }
            0x80121dcc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121de0; continue; }
                pc = 0x80121dd8;
            }
            0x80121dd8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121dec; continue;
                pc = 0x80121de0;
            }
            0x80121de0 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x80121dec; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121dec;
            }
            0x80121dec => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121e04; continue; }
                pc = 0x80121df8;
            }
            0x80121df8 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x2;
                c.lr = 0x80121e04; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da3cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121e04;
            }
            0x80121e04 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80121e14; continue; }
                pc = 0x80121e0c;
            }
            0x80121e0c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80121ff8; continue;
                pc = 0x80121e14;
            }
            0x80121e14 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80121e1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121e1c;
            }
            0x80121e1c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121e44; continue; }
                pc = 0x80121e28;
            }
            0x80121e28 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80121e30; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121e30;
            }
            0x80121e30 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121e44; continue; }
                pc = 0x80121e3c;
            }
            0x80121e3c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121e50; continue;
                pc = 0x80121e44;
            }
            0x80121e44 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3;
                c.lr = 0x80121e50; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121e50;
            }
            0x80121e50 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121e64; continue; }
                pc = 0x80121e5c;
            }
            0x80121e5c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80121ff8; continue;
                pc = 0x80121e64;
            }
            0x80121e64 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80121e6c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121e6c;
            }
            0x80121e6c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121e94; continue; }
                pc = 0x80121e78;
            }
            0x80121e78 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80121e80; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121e80;
            }
            0x80121e80 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121e94; continue; }
                pc = 0x80121e8c;
            }
            0x80121e8c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121ea0; continue;
                pc = 0x80121e94;
            }
            0x80121e94 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4;
                c.lr = 0x80121ea0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121ea0;
            }
            0x80121ea0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121eb4; continue; }
                pc = 0x80121eac;
            }
            0x80121eac => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80121ff8; continue;
                pc = 0x80121eb4;
            }
            0x80121eb4 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x80121ebc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121ebc;
            }
            0x80121ebc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121ee4; continue; }
                pc = 0x80121ec8;
            }
            0x80121ec8 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x80121ed0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121ed0;
            }
            0x80121ed0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121ee4; continue; }
                pc = 0x80121edc;
            }
            0x80121edc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121ef0; continue;
                pc = 0x80121ee4;
            }
            0x80121ee4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x5;
                c.lr = 0x80121ef0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121ef0;
            }
            0x80121ef0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121f04; continue; }
                pc = 0x80121efc;
            }
            0x80121efc => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80121ff8; continue;
                pc = 0x80121f04;
            }
            0x80121f04 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80121f0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121f0c;
            }
            0x80121f0c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121f34; continue; }
                pc = 0x80121f18;
            }
            0x80121f18 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80121f20; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121f20;
            }
            0x80121f20 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121f34; continue; }
                pc = 0x80121f2c;
            }
            0x80121f2c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121f40; continue;
                pc = 0x80121f34;
            }
            0x80121f34 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x6;
                c.lr = 0x80121f40; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121f40;
            }
            0x80121f40 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121f54; continue; }
                pc = 0x80121f4c;
            }
            0x80121f4c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80121ff8; continue;
                pc = 0x80121f54;
            }
            0x80121f54 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80121f5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121f5c;
            }
            0x80121f5c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121f84; continue; }
                pc = 0x80121f68;
            }
            0x80121f68 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80121f70; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121f70;
            }
            0x80121f70 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121f84; continue; }
                pc = 0x80121f7c;
            }
            0x80121f7c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121f90; continue;
                pc = 0x80121f84;
            }
            0x80121f84 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x80121f90; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121f90;
            }
            0x80121f90 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121fa4; continue; }
                pc = 0x80121f9c;
            }
            0x80121f9c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80121ff8; continue;
                pc = 0x80121fa4;
            }
            0x80121fa4 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80121fac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121fac;
            }
            0x80121fac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80121fd4; continue; }
                pc = 0x80121fb8;
            }
            0x80121fb8 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80121fc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121fc0;
            }
            0x80121fc0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80121fd4; continue; }
                pc = 0x80121fcc;
            }
            0x80121fcc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80121fe0; continue;
                pc = 0x80121fd4;
            }
            0x80121fd4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x80121fe0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80121fe0;
            }
            0x80121fe0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80121ff4; continue; }
                pc = 0x80121fec;
            }
            0x80121fec => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80121ff8; continue;
                pc = 0x80121ff4;
            }
            0x80121ff4 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x80121ff8;
            }
            0x80121ff8 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8012201c; continue; }
                pc = 0x80122004;
            }
            0x80122004 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1;
                c.lr = 0x80122010; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da36c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122010;
            }
            0x80122010 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x2;
                c.lr = 0x8012201c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da36c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012201c;
            }
            0x8012201c => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x80122020;
            }
            0x80122020 => { ins += 9;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80121c18: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSetSequenceStatus (0x80122040, 756 bytes)
#[inline(never)]
pub fn f_80122040(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80122040) { return; }
        if c.prof { crate::prof::enter(c, 0x80122040); p = true; }
    }
    f_80122040_b(c, 0x80122040);
    if p { crate::prof::exit(c, 0x80122040); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80122040_at(c: &mut Cpu, pc: u32) { f_80122040_b(c, pc) }
#[inline(never)]
fn f_80122040_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80122040;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80122040, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80122040 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x8;
                c.lr = 0x80122064; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122064;
            }
            0x80122064 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x8012208c; continue; }
                pc = 0x80122070;
            }
            0x80122070 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80122078; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122078;
            }
            0x80122078 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x8012208c; continue; }
                pc = 0x80122084;
            }
            0x80122084 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122098; continue;
                pc = 0x8012208c;
            }
            0x8012208c => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x8;
                c.lr = 0x80122098; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122098;
            }
            0x80122098 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801220b0; continue; }
                pc = 0x801220a4;
            }
            0x801220a4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x801220b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da3cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801220b0;
            }
            0x801220b0 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x801220b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801220b8;
            }
            0x801220b8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801220e0; continue; }
                pc = 0x801220c4;
            }
            0x801220c4 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x801220cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801220cc;
            }
            0x801220cc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801220e0; continue; }
                pc = 0x801220d8;
            }
            0x801220d8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801220ec; continue;
                pc = 0x801220e0;
            }
            0x801220e0 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x7;
                c.lr = 0x801220ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801220ec;
            }
            0x801220ec => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122104; continue; }
                pc = 0x801220f8;
            }
            0x801220f8 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2;
                c.lr = 0x80122104; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da3cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122104;
            }
            0x80122104 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80122114; continue; }
                pc = 0x8012210c;
            }
            0x8012210c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801222f8; continue;
                pc = 0x80122114;
            }
            0x80122114 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x8012211c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012211c;
            }
            0x8012211c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122144; continue; }
                pc = 0x80122128;
            }
            0x80122128 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80122130; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122130;
            }
            0x80122130 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122144; continue; }
                pc = 0x8012213c;
            }
            0x8012213c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122150; continue;
                pc = 0x80122144;
            }
            0x80122144 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x3;
                c.lr = 0x80122150; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122150;
            }
            0x80122150 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122164; continue; }
                pc = 0x8012215c;
            }
            0x8012215c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801222f8; continue;
                pc = 0x80122164;
            }
            0x80122164 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x8012216c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012216c;
            }
            0x8012216c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122194; continue; }
                pc = 0x80122178;
            }
            0x80122178 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80122180; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122180;
            }
            0x80122180 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122194; continue; }
                pc = 0x8012218c;
            }
            0x8012218c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801221a0; continue;
                pc = 0x80122194;
            }
            0x80122194 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x4;
                c.lr = 0x801221a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801221a0;
            }
            0x801221a0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801221b4; continue; }
                pc = 0x801221ac;
            }
            0x801221ac => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801222f8; continue;
                pc = 0x801221b4;
            }
            0x801221b4 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x801221bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801221bc;
            }
            0x801221bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801221e4; continue; }
                pc = 0x801221c8;
            }
            0x801221c8 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x801221d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801221d0;
            }
            0x801221d0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801221e4; continue; }
                pc = 0x801221dc;
            }
            0x801221dc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801221f0; continue;
                pc = 0x801221e4;
            }
            0x801221e4 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x5;
                c.lr = 0x801221f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801221f0;
            }
            0x801221f0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122204; continue; }
                pc = 0x801221fc;
            }
            0x801221fc => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801222f8; continue;
                pc = 0x80122204;
            }
            0x80122204 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x8012220c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012220c;
            }
            0x8012220c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122234; continue; }
                pc = 0x80122218;
            }
            0x80122218 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80122220; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122220;
            }
            0x80122220 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122234; continue; }
                pc = 0x8012222c;
            }
            0x8012222c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122240; continue;
                pc = 0x80122234;
            }
            0x80122234 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x6;
                c.lr = 0x80122240; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122240;
            }
            0x80122240 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122254; continue; }
                pc = 0x8012224c;
            }
            0x8012224c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801222f8; continue;
                pc = 0x80122254;
            }
            0x80122254 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x8012225c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012225c;
            }
            0x8012225c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122284; continue; }
                pc = 0x80122268;
            }
            0x80122268 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80122270; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122270;
            }
            0x80122270 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122284; continue; }
                pc = 0x8012227c;
            }
            0x8012227c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122290; continue;
                pc = 0x80122284;
            }
            0x80122284 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x7;
                c.lr = 0x80122290; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122290;
            }
            0x80122290 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801222a4; continue; }
                pc = 0x8012229c;
            }
            0x8012229c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801222f8; continue;
                pc = 0x801222a4;
            }
            0x801222a4 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x801222ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801222ac;
            }
            0x801222ac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x801222d4; continue; }
                pc = 0x801222b8;
            }
            0x801222b8 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x801222c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801222c0;
            }
            0x801222c0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x801222d4; continue; }
                pc = 0x801222cc;
            }
            0x801222cc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801222e0; continue;
                pc = 0x801222d4;
            }
            0x801222d4 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x8;
                c.lr = 0x801222e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801222e0;
            }
            0x801222e0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801222f4; continue; }
                pc = 0x801222ec;
            }
            0x801222ec => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801222f8; continue;
                pc = 0x801222f4;
            }
            0x801222f4 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x801222f8;
            }
            0x801222f8 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8012231c; continue; }
                pc = 0x80122304;
            }
            0x80122304 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x80122310; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da36c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122310;
            }
            0x80122310 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x2;
                c.lr = 0x8012231c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da36c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012231c;
            }
            0x8012231c => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80122040: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetAnnonKatati (0x80122334, 60 bytes)
#[inline(never)]
pub fn f_80122334(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80122334) { return; }
        if c.prof { crate::prof::enter(c, 0x80122334); p = true; }
    }
    f_80122334_b(c, 0x80122334);
    if p { crate::prof::exit(c, 0x80122334); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80122334_at(c: &mut Cpu, pc: u32) { f_80122334_b(c, pc) }
#[inline(always)]
fn f_80122334_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80122334;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80122334, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80122334 => { ins += 21;
                c.r[4] = 0x24920000;
                c.r[5] = c.r[3].rotate_left(20) & 0x30;
                c.r[5] = (c.r[3].rotate_left(14) & 0xc0) | (c.r[5] & 0xffffff3f);
                c.r[5] = (c.r[3].rotate_left(26) & 0xc) | (c.r[5] & 0xfffffff3);
                c.r[0] = c.r[4].wrapping_add(0x4925);
                c.r[5] = (c.r[3].rotate_left(0) & 0x3) | (c.r[5] & 0xfffffffc);
                c.r[3] = mulhwu(c.r[0], c.r[5]);
                c.r[0] = c.r[5].wrapping_sub(c.r[3]);
                c.r[0] = c.r[0].rotate_left(31) & 0x7fffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.r[0].rotate_left(28) & 0xfffffff;
                c.r[0] = c.r[0].wrapping_mul(0x1c);
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80122334: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetFriendFormPokemonFriendFilterId (0x80122370, 864 bytes)
#[inline(never)]
pub fn f_80122370(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80122370) { return; }
        if c.prof { crate::prof::enter(c, 0x80122370); p = true; }
    }
    f_80122370_b(c, 0x80122370);
    if p { crate::prof::exit(c, 0x80122370); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80122370_at(c: &mut Cpu, pc: u32) { f_80122370_b(c, pc) }
#[inline(never)]
fn f_80122370_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80122370;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80122370, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80122370 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = c.r[5] | c.r[5];
                if cr_bit(cr, 2) { pc = 0x801226bc; continue; }
                pc = 0x80122390;
            }
            0x80122390 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8012239c; continue; }
                pc = 0x80122394;
            }
            0x80122394 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8012246c; continue;
                pc = 0x8012239c;
            }
            0x8012239c => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x801223ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801223ac;
            }
            0x801223ac => { ins += 2;
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[28]);
                if !cr_bit(cr, 2) { pc = 0x801223bc; continue; }
                pc = 0x801223b4;
            }
            0x801223b4 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8012246c; continue;
                pc = 0x801223bc;
            }
            0x801223bc => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801223c8; continue; }
                pc = 0x801223c0;
            }
            0x801223c0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122408; continue;
                pc = 0x801223c8;
            }
            0x801223c8 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x801223dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801223dc;
            }
            0x801223dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801223ec; continue; }
                pc = 0x801223e4;
            }
            0x801223e4 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122408; continue;
                pc = 0x801223ec;
            }
            0x801223ec => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8770));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x80122404; continue; }
                pc = 0x801223fc;
            }
            0x801223fc => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122408; continue;
                pc = 0x80122404;
            }
            0x80122404 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x80122408;
            }
            0x80122408 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80122418; continue; }
                pc = 0x80122410;
            }
            0x80122410 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8012246c; continue;
                pc = 0x80122418;
            }
            0x80122418 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x70;
                c.r[6] = 0x0;
                c.lr = 0x8012242c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012242c;
            }
            0x8012242c => { ins += 1;
                c.lr = 0x80122430; { c.steps = st; c.cycles = ins; c.cr = cr; f_80135530(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122430;
            }
            0x80122430 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80122440; continue; }
                pc = 0x80122438;
            }
            0x80122438 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8012246c; continue;
                pc = 0x80122440;
            }
            0x80122440 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xb8;
                c.r[6] = 0x0;
                c.lr = 0x80122454; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122454;
            }
            0x80122454 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122468; continue; }
                pc = 0x80122460;
            }
            0x80122460 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8012246c; continue;
                pc = 0x80122468;
            }
            0x80122468 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x8012246c;
            }
            0x8012246c => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8012247c; continue; }
                pc = 0x80122474;
            }
            0x80122474 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122510; continue;
                pc = 0x8012247c;
            }
            0x8012247c => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80122490; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122490;
            }
            0x80122490 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x19c);
                if !cr_bit(cr, 2) { pc = 0x801224a0; continue; }
                pc = 0x80122498;
            }
            0x80122498 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122510; continue;
                pc = 0x801224a0;
            }
            0x801224a0 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xb6;
                c.r[6] = 0x0;
                c.lr = 0x801224b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801224b4;
            }
            0x801224b4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801224c8; continue; }
                pc = 0x801224c0;
            }
            0x801224c0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122510; continue;
                pc = 0x801224c8;
            }
            0x801224c8 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 2) { pc = 0x8012250c; continue; }
                pc = 0x801224d4;
            }
            0x801224d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7);
                if cr_bit(cr, 2) { pc = 0x8012250c; continue; }
                pc = 0x801224dc;
            }
            0x801224dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if cr_bit(cr, 2) { pc = 0x8012250c; continue; }
                pc = 0x801224e4;
            }
            0x801224e4 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x7b;
                c.r[6] = 0x0;
                c.lr = 0x801224f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801224f8;
            }
            0x801224f8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8012250c; continue; }
                pc = 0x80122504;
            }
            0x80122504 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122510; continue;
                pc = 0x8012250c;
            }
            0x8012250c => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x80122510;
            }
            0x80122510 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801226bc; continue; }
                pc = 0x80122518;
            }
            0x80122518 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x99;
                c.r[6] = 0x0;
                c.lr = 0x8012252c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012252c;
            }
            0x8012252c => { ins += 6;
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xc2;
                c.r[6] = 0x0;
                c.lr = 0x80122544; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122544;
            }
            0x80122544 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122568; continue; }
                pc = 0x80122550;
            }
            0x80122550 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xc7;
                c.r[6] = 0x0;
                c.lr = 0x80122564; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122564;
            }
            0x80122564 => { ins += 1;
                c.r[28] = c.r[28].wrapping_add(c.r[3]);
                pc = 0x80122568;
            }
            0x80122568 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x64);
                c.r[27] = 0x0;
                if cr_bit(cr, 0) { pc = 0x80122578; continue; }
                pc = 0x80122574;
            }
            0x80122574 => { ins += 1;
                c.r[27] = 0x1;
                pc = 0x80122578;
            }
            0x80122578 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0xc8);
                if cr_bit(cr, 0) { pc = 0x8012258c; continue; }
                pc = 0x80122580;
            }
            0x80122580 => { ins += 3;
                c.r[3] = c.r[27].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[27] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x8012258c;
            }
            0x8012258c => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5);
                if !cr_bit(cr, 2) { pc = 0x801225a4; continue; }
                pc = 0x80122598;
            }
            0x80122598 => { ins += 1;
                c.lr = 0x8012259c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800e0c54(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012259c;
            }
            0x8012259c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801226bc; continue; }
                pc = 0x801225a4;
            }
            0x801225a4 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801225ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cb10(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801225ac;
            }
            0x801225ac => { ins += 2;
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x801225b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011cae0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801225b4;
            }
            0x801225b4 => { ins += 2;
                c.r[31] = c.r[3] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 1) { pc = 0x801225e4; continue; }
                pc = 0x801225bc;
            }
            0x801225bc => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1b);
                if !cr_bit(cr, 2) { pc = 0x801225e4; continue; }
                pc = 0x801225c8;
            }
            0x801225c8 => { ins += 13;
                c.r[0] = c.r[31].wrapping_mul(0x96);
                c.r[3] = 0x51ec0000;
                c.r[3] = c.r[3].wrapping_add(0xffff851f);
                c.r[0] = mulhw(c.r[3], c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[0] = (v >> 5) as u32; }
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[31] = c.r[0].wrapping_add(c.r[3]);
                pc = 0x801225e4;
            }
            0x801225e4 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                c.r[30] = c.r[31] | c.r[31];
                if !cr_bit(cr, 1) { pc = 0x80122614; continue; }
                pc = 0x801225f0;
            }
            0x801225f0 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x73;
                c.r[6] = 0x0;
                c.lr = 0x80122604; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122604;
            }
            0x80122604 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xb);
                if !cr_bit(cr, 2) { pc = 0x80122614; continue; }
                pc = 0x80122610;
            }
            0x80122610 => { ins += 1;
                c.r[30] = c.r[31].wrapping_add(0x1);
                pc = 0x80122614;
            }
            0x80122614 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xc2;
                c.r[6] = 0x0;
                c.lr = 0x80122628; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122628;
            }
            0x80122628 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8012268c; continue; }
                pc = 0x80122634;
            }
            0x80122634 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8012263c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011f77c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012263c;
            }
            0x8012263c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3);
                if cr_bit(cr, 0) { pc = 0x801226bc; continue; }
                pc = 0x80122648;
            }
            0x80122648 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xc7;
                c.r[6] = 0x0;
                c.lr = 0x8012265c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012265c;
            }
            0x8012265c => { ins += 2;
                c.r[7] = c.r[3].wrapping_add(c.r[30]);
                cr_rc(&mut cr, c.xer, c.r[7]);
                if !cr_bit(cr, 0) { pc = 0x80122668; continue; }
                pc = 0x80122664;
            }
            0x80122664 => { ins += 1;
                c.r[7] = 0x0;
                pc = 0x80122668;
            }
            0x80122668 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[7], 0xff);
                if !cr_bit(cr, 1) { pc = 0x80122674; continue; }
                pc = 0x80122670;
            }
            0x80122670 => { ins += 1;
                c.r[7] = 0xff;
                pc = 0x80122674;
            }
            0x80122674 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xc7;
                c.r[6] = 0x0;
                c.lr = 0x80122688; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122688;
            }
            0x80122688 => { ins += 1;
                pc = 0x801226bc; continue;
                pc = 0x8012268c;
            }
            0x8012268c => { ins += 2;
                c.r[28] = c.r[28].wrapping_add(c.r[30]);
                cr_rc(&mut cr, c.xer, c.r[28]);
                if !cr_bit(cr, 0) { pc = 0x80122698; continue; }
                pc = 0x80122694;
            }
            0x80122694 => { ins += 1;
                c.r[28] = 0x0;
                pc = 0x80122698;
            }
            0x80122698 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0xff);
                if !cr_bit(cr, 1) { pc = 0x801226a4; continue; }
                pc = 0x801226a0;
            }
            0x801226a0 => { ins += 1;
                c.r[28] = 0xff;
                pc = 0x801226a4;
            }
            0x801226a4 => { ins += 6;
                c.r[3] = c.r[29] | c.r[29];
                c.r[7] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x99;
                c.r[6] = 0x0;
                c.lr = 0x801226bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801226bc;
            }
            0x801226bc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80122370: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetEffortFromPokemon (0x801226d0, 804 bytes)
#[inline(never)]
pub fn f_801226d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801226d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801226d0); p = true; }
    }
    f_801226d0_b(c, 0x801226d0);
    if p { crate::prof::exit(c, 0x801226d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801226d0_at(c: &mut Cpu, pc: u32) { f_801226d0_b(c, pc) }
#[inline(never)]
fn f_801226d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801226d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801226d0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801226d0 => { ins += 39;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.r[12] = c.r[7].wrapping_add(0x2948);
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[26] = c.r[6] | c.r[6];
                c.r[25] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[25]);
                c.r[11] = c.lwz_t(ins, c.r[12]);
                c.r[10] = c.lwz_t(ins, c.r[12].wrapping_add(0x4));
                c.r[9] = c.lwz_t(ins, c.r[12].wrapping_add(0x8));
                c.r[8] = c.lwz_t(ins, c.r[12].wrapping_add(0xc));
                c.r[7] = c.lwz_t(ins, c.r[12].wrapping_add(0x10));
                c.r[6] = c.lwz_t(ins, c.r[12].wrapping_add(0x14));
                c.r[5] = c.lwz_t(ins, c.r[12].wrapping_add(0x18));
                c.r[4] = c.lwz_t(ins, c.r[12].wrapping_add(0x1c));
                c.r[0] = c.lwz_t(ins, c.r[12].wrapping_add(0x20));
                c.stw(c.r[1].wrapping_add(0x8), c.r[11]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801229e0; continue; }
                pc = 0x80122744;
            }
            0x80122744 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x80122750; continue; }
                pc = 0x80122748;
            }
            0x80122748 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122820; continue;
                pc = 0x80122750;
            }
            0x80122750 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80122760; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122760;
            }
            0x80122760 => { ins += 2;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[27]);
                if !cr_bit(cr, 2) { pc = 0x80122770; continue; }
                pc = 0x80122768;
            }
            0x80122768 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122820; continue;
                pc = 0x80122770;
            }
            0x80122770 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8012277c; continue; }
                pc = 0x80122774;
            }
            0x80122774 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801227bc; continue;
                pc = 0x8012277c;
            }
            0x8012277c => { ins += 5;
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80122790; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122790;
            }
            0x80122790 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801227a0; continue; }
                pc = 0x80122798;
            }
            0x80122798 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801227bc; continue;
                pc = 0x801227a0;
            }
            0x801227a0 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8770));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801227b8; continue; }
                pc = 0x801227b0;
            }
            0x801227b0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801227bc; continue;
                pc = 0x801227b8;
            }
            0x801227b8 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x801227bc;
            }
            0x801227bc => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801227cc; continue; }
                pc = 0x801227c4;
            }
            0x801227c4 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122820; continue;
                pc = 0x801227cc;
            }
            0x801227cc => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x70;
                c.r[6] = 0x0;
                c.lr = 0x801227e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801227e0;
            }
            0x801227e0 => { ins += 1;
                c.lr = 0x801227e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80135530(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801227e4;
            }
            0x801227e4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x801227f4; continue; }
                pc = 0x801227ec;
            }
            0x801227ec => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122820; continue;
                pc = 0x801227f4;
            }
            0x801227f4 => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0xb8;
                c.r[6] = 0x0;
                c.lr = 0x80122808; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122808;
            }
            0x80122808 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8012281c; continue; }
                pc = 0x80122814;
            }
            0x80122814 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x80122820; continue;
                pc = 0x8012281c;
            }
            0x8012281c => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x80122820;
            }
            0x80122820 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80122830; continue; }
                pc = 0x80122828;
            }
            0x80122828 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801228a8; continue;
                pc = 0x80122830;
            }
            0x80122830 => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80122844; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122844;
            }
            0x80122844 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x19c);
                if !cr_bit(cr, 2) { pc = 0x80122854; continue; }
                pc = 0x8012284c;
            }
            0x8012284c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801228a8; continue;
                pc = 0x80122854;
            }
            0x80122854 => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0xb6;
                c.r[6] = 0x0;
                c.lr = 0x80122868; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122868;
            }
            0x80122868 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8012287c; continue; }
                pc = 0x80122874;
            }
            0x80122874 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801228a8; continue;
                pc = 0x8012287c;
            }
            0x8012287c => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x7b;
                c.r[6] = 0x0;
                c.lr = 0x80122890; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122890;
            }
            0x80122890 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x801228a4; continue; }
                pc = 0x8012289c;
            }
            0x8012289c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x801228a8; continue;
                pc = 0x801228a4;
            }
            0x801228a4 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x801228a8;
            }
            0x801228a8 => { ins += 2;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x801229e0; continue; }
                pc = 0x801228b0;
            }
            0x801228b0 => { ins += 4;
                c.r[24] = 0x1;
                c.r[27] = 0x0;
                c.r[31] = 0x0;
                pc = 0x801228f4; continue;
                pc = 0x801228c0;
            }
            0x801228c0 => { ins += 11;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.r[30] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.r[0].wrapping_mul(0x6);
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.r[30] = c.r[30].wrapping_add(c.r[0]);
                c.r[5] = c.lhz_t(ins, c.r[30]);
                c.lr = 0x801228e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801228e4;
            }
            0x801228e4 => { ins += 4;
                c.sth(c.r[30].wrapping_add(0x4), c.r[3]);
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.r[0] = c.lhz_t(ins, c.r[30].wrapping_add(0x4));
                c.r[27] = c.r[27].wrapping_add(c.r[0]);
                pc = 0x801228f4;
            }
            0x801228f4 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801228c0; continue; }
                pc = 0x80122900;
            }
            0x80122900 => { ins += 3;
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122910; continue; }
                pc = 0x8012290c;
            }
            0x8012290c => { ins += 1;
                c.r[24] = 0x2;
                pc = 0x80122910;
            }
            0x80122910 => { ins += 4;
                c.r[30] = c.r[24].rotate_left(0) & 0xff;
                c.r[31] = c.r[29].rotate_left(0) & 0xffff;
                c.r[28] = 0x0;
                pc = 0x801229d4; continue;
                pc = 0x80122920;
            }
            0x80122920 => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1fe);
                if !cr_bit(cr, 0) { pc = 0x801229e0; continue; }
                pc = 0x8012292c;
            }
            0x8012292c => { ins += 11;
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                c.r[29] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.r[0].wrapping_mul(0x6);
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.r[29] = c.r[29].wrapping_add(c.r[0]);
                c.r[5] = c.lhz_t(ins, c.r[29].wrapping_add(0x2));
                c.lr = 0x80122950; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122950;
            }
            0x80122950 => { ins += 9;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x18);
                c.r[0] = c.r[30].wrapping_mul(c.r[0]);
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                if !cr_bit(cr, 2) { pc = 0x80122968; continue; }
                pc = 0x80122964;
            }
            0x80122964 => { ins += 1;
                c.r[5] = c.r[5].rotate_left(1) & 0xfffe;
                pc = 0x80122968;
            }
            0x80122968 => { ins += 5;
                c.r[3] = c.r[27].rotate_left(0) & 0xffff;
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1fe);
                if !cr_bit(cr, 1) { pc = 0x80122988; continue; }
                pc = 0x8012297c;
            }
            0x8012297c => { ins += 3;
                c.r[0] = c.r[3].wrapping_add(0xfffffe02);
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80122988;
            }
            0x80122988 => { ins += 5;
                c.r[0] = c.lhz_t(ins, c.r[29].wrapping_add(0x4));
                c.r[3] = c.r[5].rotate_left(0) & 0xffff;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xff);
                if !cr_bit(cr, 1) { pc = 0x801229a8; continue; }
                pc = 0x8012299c;
            }
            0x8012299c => { ins += 3;
                c.r[0] = c.r[3].wrapping_add(0xffffff01);
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x801229a8;
            }
            0x801229a8 => { ins += 10;
                c.r[0] = c.lhz_t(ins, c.r[29].wrapping_add(0x4));
                c.r[3] = c.r[25] | c.r[25];
                c.r[27] = c.r[27].wrapping_add(c.r[5]);
                c.r[4] = 0x0;
                c.r[0] = c.r[0].wrapping_add(c.r[5]);
                c.r[6] = 0x0;
                c.sth(c.r[29].wrapping_add(0x4), c.r[0]);
                c.r[5] = c.lhz_t(ins, c.r[29]);
                c.r[7] = c.lhz_t(ins, c.r[29].wrapping_add(0x4));
                c.lr = 0x801229d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801229d0;
            }
            0x801229d0 => { ins += 1;
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x801229d4;
            }
            0x801229d4 => { ins += 3;
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x80122920; continue; }
                pc = 0x801229e0;
            }
            0x801229e0 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801226d0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetLevelToExp (0x801229f4, 124 bytes)
#[inline(never)]
pub fn f_801229f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801229f4) { return; }
        if c.prof { crate::prof::enter(c, 0x801229f4); p = true; }
    }
    f_801229f4_b(c, 0x801229f4);
    if p { crate::prof::exit(c, 0x801229f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801229f4_at(c: &mut Cpu, pc: u32) { f_801229f4_b(c, pc) }
#[inline(always)]
fn f_801229f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801229f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801229f4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801229f4 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x80122a18; continue; }
                pc = 0x80122a10;
            }
            0x80122a10 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122a5c; continue;
                pc = 0x80122a18;
            }
            0x80122a18 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80122a28; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122a28;
            }
            0x80122a28 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x80122a3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122a3c;
            }
            0x80122a3c => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.lr = 0x80122a44; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122a44;
            }
            0x80122a44 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80122a54; continue; }
                pc = 0x80122a4c;
            }
            0x80122a4c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122a5c; continue;
                pc = 0x80122a54;
            }
            0x80122a54 => { ins += 2;
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80122a5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011ce44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122a5c;
            }
            0x80122a5c => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801229f4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetNowHpPercentage (0x80122a70, 112 bytes)
#[inline(never)]
pub fn f_80122a70(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80122a70) { return; }
        if c.prof { crate::prof::enter(c, 0x80122a70); p = true; }
    }
    f_80122a70_b(c, 0x80122a70);
    if p { crate::prof::exit(c, 0x80122a70); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80122a70_at(c: &mut Cpu, pc: u32) { f_80122a70_b(c, pc) }
#[inline(always)]
fn f_80122a70_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80122a70;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80122a70, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80122a70 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x80122a94; continue; }
                pc = 0x80122a8c;
            }
            0x80122a8c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122ac8; continue;
                pc = 0x80122a94;
            }
            0x80122a94 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x80122aa4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122aa4;
            }
            0x80122aa4 => { ins += 6;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x80122abc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122abc;
            }
            0x80122abc => { ins += 44;
                c.r[0] = c.r[31].wrapping_mul(0x64);
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = divw(c.r[0], c.r[3]);
                pc = 0x80122ac8;
            }
            0x80122ac8 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80122a70: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetNowHpWaruValue (0x80122ae0, 112 bytes)
#[inline(never)]
pub fn f_80122ae0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80122ae0) { return; }
        if c.prof { crate::prof::enter(c, 0x80122ae0); p = true; }
    }
    f_80122ae0_b(c, 0x80122ae0);
    if p { crate::prof::exit(c, 0x80122ae0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80122ae0_at(c: &mut Cpu, pc: u32) { f_80122ae0_b(c, pc) }
#[inline(always)]
fn f_80122ae0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80122ae0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80122ae0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80122ae0 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x80122b04; continue; }
                pc = 0x80122afc;
            }
            0x80122afc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122b3c; continue;
                pc = 0x80122b04;
            }
            0x80122b04 => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80122b14; continue; }
                pc = 0x80122b0c;
            }
            0x80122b0c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122b3c; continue;
                pc = 0x80122b14;
            }
            0x80122b14 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x80122b24; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122b24;
            }
            0x80122b24 => { ins += 44;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.r[0] = divw(c.r[3], c.r[0]);
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x80122b3c; continue; }
                pc = 0x80122b38;
            }
            0x80122b38 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80122b3c;
            }
            0x80122b3c => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80122ae0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetMaxHpWaruValue (0x80122b50, 112 bytes)
#[inline(never)]
pub fn f_80122b50(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80122b50) { return; }
        if c.prof { crate::prof::enter(c, 0x80122b50); p = true; }
    }
    f_80122b50_b(c, 0x80122b50);
    if p { crate::prof::exit(c, 0x80122b50); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80122b50_at(c: &mut Cpu, pc: u32) { f_80122b50_b(c, pc) }
#[inline(always)]
fn f_80122b50_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80122b50;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80122b50, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80122b50 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x80122b74; continue; }
                pc = 0x80122b6c;
            }
            0x80122b6c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122bac; continue;
                pc = 0x80122b74;
            }
            0x80122b74 => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80122b84; continue; }
                pc = 0x80122b7c;
            }
            0x80122b7c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122bac; continue;
                pc = 0x80122b84;
            }
            0x80122b84 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x80122b94; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122b94;
            }
            0x80122b94 => { ins += 44;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.r[0] = divw(c.r[3], c.r[0]);
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x80122bac; continue; }
                pc = 0x80122ba8;
            }
            0x80122ba8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80122bac;
            }
            0x80122bac => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80122b50: bad pc {:#x}", pc),
        }
    }
}

/// pokemonIsNokoriHpFollowing (0x80122bc0, 164 bytes)
#[inline(never)]
pub fn f_80122bc0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80122bc0) { return; }
        if c.prof { crate::prof::enter(c, 0x80122bc0); p = true; }
    }
    f_80122bc0_b(c, 0x80122bc0);
    if p { crate::prof::exit(c, 0x80122bc0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80122bc0_at(c: &mut Cpu, pc: u32) { f_80122bc0_b(c, pc) }
#[inline(always)]
fn f_80122bc0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80122bc0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80122bc0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80122bc0 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                if !cr_bit(cr, 2) { pc = 0x80122bf0; continue; }
                pc = 0x80122be8;
            }
            0x80122be8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122c48; continue;
                pc = 0x80122bf0;
            }
            0x80122bf0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80122c00; continue; }
                pc = 0x80122bf8;
            }
            0x80122bf8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122c48; continue;
                pc = 0x80122c00;
            }
            0x80122c00 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x83;
                c.r[6] = 0x0;
                c.lr = 0x80122c10; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122c10;
            }
            0x80122c10 => { ins += 6;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x87;
                c.r[6] = 0x0;
                c.lr = 0x80122c28; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122c28;
            }
            0x80122c28 => { ins += 47;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.r[0] = divw(c.r[3], c.r[0]);
                c.r[3] = c.r[31].rotate_left(1) & 0x1;
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[4] = (v >> 31) as u32; }
                c.r[0] = c.addc(!c.r[31], c.r[0], 1);
                { let ci = c.ca(); c.r[0] = c.addc(c.r[4], c.r[3], ci); }
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x80122c48;
            }
            0x80122c48 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80122bc0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonIsJoutaiKaragenki (0x80122c64, 376 bytes)
#[inline(never)]
pub fn f_80122c64(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80122c64) { return; }
        if c.prof { crate::prof::enter(c, 0x80122c64); p = true; }
    }
    f_80122c64_b(c, 0x80122c64);
    if p { crate::prof::exit(c, 0x80122c64); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80122c64_at(c: &mut Cpu, pc: u32) { f_80122c64_b(c, pc) }
#[inline(never)]
fn f_80122c64_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80122c64;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80122c64, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80122c64 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x80122c84; continue; }
                pc = 0x80122c7c;
            }
            0x80122c7c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122dc8; continue;
                pc = 0x80122c84;
            }
            0x80122c84 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80122c8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122c8c;
            }
            0x80122c8c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122cb4; continue; }
                pc = 0x80122c98;
            }
            0x80122c98 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80122ca0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122ca0;
            }
            0x80122ca0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122cb4; continue; }
                pc = 0x80122cac;
            }
            0x80122cac => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122cc0; continue;
                pc = 0x80122cb4;
            }
            0x80122cb4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3;
                c.lr = 0x80122cc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122cc0;
            }
            0x80122cc0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122cd4; continue; }
                pc = 0x80122ccc;
            }
            0x80122ccc => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80122dc8; continue;
                pc = 0x80122cd4;
            }
            0x80122cd4 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x80122cdc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122cdc;
            }
            0x80122cdc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122d04; continue; }
                pc = 0x80122ce8;
            }
            0x80122ce8 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x80122cf0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122cf0;
            }
            0x80122cf0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122d04; continue; }
                pc = 0x80122cfc;
            }
            0x80122cfc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122d10; continue;
                pc = 0x80122d04;
            }
            0x80122d04 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x5;
                c.lr = 0x80122d10; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122d10;
            }
            0x80122d10 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122d24; continue; }
                pc = 0x80122d1c;
            }
            0x80122d1c => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80122dc8; continue;
                pc = 0x80122d24;
            }
            0x80122d24 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80122d2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122d2c;
            }
            0x80122d2c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122d54; continue; }
                pc = 0x80122d38;
            }
            0x80122d38 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80122d40; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122d40;
            }
            0x80122d40 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122d54; continue; }
                pc = 0x80122d4c;
            }
            0x80122d4c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122d60; continue;
                pc = 0x80122d54;
            }
            0x80122d54 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x6;
                c.lr = 0x80122d60; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122d60;
            }
            0x80122d60 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122d74; continue; }
                pc = 0x80122d6c;
            }
            0x80122d6c => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80122dc8; continue;
                pc = 0x80122d74;
            }
            0x80122d74 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80122d7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122d7c;
            }
            0x80122d7c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122da4; continue; }
                pc = 0x80122d88;
            }
            0x80122d88 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80122d90; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122d90;
            }
            0x80122d90 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122da4; continue; }
                pc = 0x80122d9c;
            }
            0x80122d9c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122db0; continue;
                pc = 0x80122da4;
            }
            0x80122da4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4;
                c.lr = 0x80122db0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122db0;
            }
            0x80122db0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122dc4; continue; }
                pc = 0x80122dbc;
            }
            0x80122dbc => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x80122dc8; continue;
                pc = 0x80122dc4;
            }
            0x80122dc4 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x80122dc8;
            }
            0x80122dc8 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80122c64: bad pc {:#x}", pc),
        }
    }
}

/// pokemonIsJoutaiNormal (0x80122ddc, 536 bytes)
#[inline(never)]
pub fn f_80122ddc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80122ddc) { return; }
        if c.prof { crate::prof::enter(c, 0x80122ddc); p = true; }
    }
    f_80122ddc_b(c, 0x80122ddc);
    if p { crate::prof::exit(c, 0x80122ddc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80122ddc_at(c: &mut Cpu, pc: u32) { f_80122ddc_b(c, pc) }
#[inline(never)]
fn f_80122ddc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80122ddc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80122ddc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80122ddc => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x80122dfc; continue; }
                pc = 0x80122df4;
            }
            0x80122df4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122fe0; continue;
                pc = 0x80122dfc;
            }
            0x80122dfc => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80122e04; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122e04;
            }
            0x80122e04 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122e2c; continue; }
                pc = 0x80122e10;
            }
            0x80122e10 => { ins += 2;
                c.r[3] = 0x3;
                c.lr = 0x80122e18; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122e18;
            }
            0x80122e18 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122e2c; continue; }
                pc = 0x80122e24;
            }
            0x80122e24 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122e38; continue;
                pc = 0x80122e2c;
            }
            0x80122e2c => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3;
                c.lr = 0x80122e38; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122e38;
            }
            0x80122e38 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122e4c; continue; }
                pc = 0x80122e44;
            }
            0x80122e44 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122fe0; continue;
                pc = 0x80122e4c;
            }
            0x80122e4c => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80122e54; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122e54;
            }
            0x80122e54 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122e7c; continue; }
                pc = 0x80122e60;
            }
            0x80122e60 => { ins += 2;
                c.r[3] = 0x4;
                c.lr = 0x80122e68; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122e68;
            }
            0x80122e68 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122e7c; continue; }
                pc = 0x80122e74;
            }
            0x80122e74 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122e88; continue;
                pc = 0x80122e7c;
            }
            0x80122e7c => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x4;
                c.lr = 0x80122e88; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122e88;
            }
            0x80122e88 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122e9c; continue; }
                pc = 0x80122e94;
            }
            0x80122e94 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122fe0; continue;
                pc = 0x80122e9c;
            }
            0x80122e9c => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x80122ea4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122ea4;
            }
            0x80122ea4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122ecc; continue; }
                pc = 0x80122eb0;
            }
            0x80122eb0 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x80122eb8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122eb8;
            }
            0x80122eb8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122ecc; continue; }
                pc = 0x80122ec4;
            }
            0x80122ec4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122ed8; continue;
                pc = 0x80122ecc;
            }
            0x80122ecc => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x5;
                c.lr = 0x80122ed8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122ed8;
            }
            0x80122ed8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122eec; continue; }
                pc = 0x80122ee4;
            }
            0x80122ee4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122fe0; continue;
                pc = 0x80122eec;
            }
            0x80122eec => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80122ef4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122ef4;
            }
            0x80122ef4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122f1c; continue; }
                pc = 0x80122f00;
            }
            0x80122f00 => { ins += 2;
                c.r[3] = 0x6;
                c.lr = 0x80122f08; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122f08;
            }
            0x80122f08 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122f1c; continue; }
                pc = 0x80122f14;
            }
            0x80122f14 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122f28; continue;
                pc = 0x80122f1c;
            }
            0x80122f1c => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x6;
                c.lr = 0x80122f28; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122f28;
            }
            0x80122f28 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122f3c; continue; }
                pc = 0x80122f34;
            }
            0x80122f34 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122fe0; continue;
                pc = 0x80122f3c;
            }
            0x80122f3c => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80122f44; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122f44;
            }
            0x80122f44 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122f6c; continue; }
                pc = 0x80122f50;
            }
            0x80122f50 => { ins += 2;
                c.r[3] = 0x7;
                c.lr = 0x80122f58; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122f58;
            }
            0x80122f58 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122f6c; continue; }
                pc = 0x80122f64;
            }
            0x80122f64 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122f78; continue;
                pc = 0x80122f6c;
            }
            0x80122f6c => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x7;
                c.lr = 0x80122f78; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122f78;
            }
            0x80122f78 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122f8c; continue; }
                pc = 0x80122f84;
            }
            0x80122f84 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122fe0; continue;
                pc = 0x80122f8c;
            }
            0x80122f8c => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80122f94; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122f94;
            }
            0x80122f94 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x7c);
                if cr_bit(cr, 2) { pc = 0x80122fbc; continue; }
                pc = 0x80122fa0;
            }
            0x80122fa0 => { ins += 2;
                c.r[3] = 0x8;
                c.lr = 0x80122fa8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122fa8;
            }
            0x80122fa8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc8);
                if cr_bit(cr, 2) { pc = 0x80122fbc; continue; }
                pc = 0x80122fb4;
            }
            0x80122fb4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122fc8; continue;
                pc = 0x80122fbc;
            }
            0x80122fbc => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x8;
                c.lr = 0x80122fc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80122fc8;
            }
            0x80122fc8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80122fdc; continue; }
                pc = 0x80122fd4;
            }
            0x80122fd4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80122fe0; continue;
                pc = 0x80122fdc;
            }
            0x80122fdc => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x80122fe0;
            }
            0x80122fe0 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80122ddc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetSoubiItemBuff (0x80122ff4, 156 bytes)
#[inline(never)]
pub fn f_80122ff4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80122ff4) { return; }
        if c.prof { crate::prof::enter(c, 0x80122ff4); p = true; }
    }
    f_80122ff4_b(c, 0x80122ff4);
    if p { crate::prof::exit(c, 0x80122ff4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80122ff4_at(c: &mut Cpu, pc: u32) { f_80122ff4_b(c, pc) }
#[inline(always)]
fn f_80122ff4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80122ff4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80122ff4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80122ff4 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x82;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80123014; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123014;
            }
            0x80123014 => { ins += 2;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x80123024; continue; }
                pc = 0x8012301c;
            }
            0x8012301c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8012307c; continue;
                pc = 0x80123024;
            }
            0x80123024 => { ins += 1;
                c.lr = 0x80123028; { c.steps = st; c.cycles = ins; c.cr = cr; f_80008154(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123028;
            }
            0x80123028 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x80123064; continue; }
                pc = 0x80123034;
            }
            0x80123034 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x80123048; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123048;
            }
            0x80123048 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1a);
                if cr_bit(cr, 2) { pc = 0x8012305c; continue; }
                pc = 0x80123054;
            }
            0x80123054 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1e);
                if !cr_bit(cr, 2) { pc = 0x80123064; continue; }
                pc = 0x8012305c;
            }
            0x8012305c => { ins += 2;
                c.r[3] = 0x63;
                pc = 0x8012307c; continue;
                pc = 0x80123064;
            }
            0x80123064 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0xa;
                c.r[6] = 0x0;
                c.lr = 0x80123078; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123078;
            }
            0x80123078 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8012307c;
            }
            0x8012307c => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80122ff4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetSoubiItemSoubiDataId (0x80123090, 80 bytes)
#[inline(never)]
pub fn f_80123090(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80123090) { return; }
        if c.prof { crate::prof::enter(c, 0x80123090); p = true; }
    }
    f_80123090_b(c, 0x80123090);
    if p { crate::prof::exit(c, 0x80123090); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80123090_at(c: &mut Cpu, pc: u32) { f_80123090_b(c, pc) }
#[inline(always)]
fn f_80123090_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80123090;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80123090, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80123090 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x82;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.lr = 0x801230ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801230ac;
            }
            0x801230ac => { ins += 2;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[4]);
                if !cr_bit(cr, 2) { pc = 0x801230bc; continue; }
                pc = 0x801230b4;
            }
            0x801230b4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801230d0; continue;
                pc = 0x801230bc;
            }
            0x801230bc => { ins += 4;
                c.r[3] = 0x0;
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x801230cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801230cc;
            }
            0x801230cc => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x801230d0;
            }
            0x801230d0 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80123090: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetSoubiItemDataId (0x801230e0, 48 bytes)
#[inline(never)]
pub fn f_801230e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801230e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801230e0); p = true; }
    }
    f_801230e0_b(c, 0x801230e0);
    if p { crate::prof::exit(c, 0x801230e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801230e0_at(c: &mut Cpu, pc: u32) { f_801230e0_b(c, pc) }
#[inline(always)]
fn f_801230e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801230e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801230e0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801230e0 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x82;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.lr = 0x801230fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801230fc;
            }
            0x801230fc => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801230e0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDoItemSoubi (0x80123110, 148 bytes)
#[inline(never)]
pub fn f_80123110(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80123110) { return; }
        if c.prof { crate::prof::enter(c, 0x80123110); p = true; }
    }
    f_80123110_b(c, 0x80123110);
    if p { crate::prof::exit(c, 0x80123110); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80123110_at(c: &mut Cpu, pc: u32) { f_80123110_b(c, pc) }
#[inline(always)]
fn f_80123110_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80123110;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80123110, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80123110 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x80123138; continue; }
                pc = 0x80123130;
            }
            0x80123130 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8012318c; continue;
                pc = 0x80123138;
            }
            0x80123138 => { ins += 2;
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80123174; continue; }
                pc = 0x80123140;
            }
            0x80123140 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x82;
                c.r[6] = 0x0;
                c.lr = 0x80123150; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123150;
            }
            0x80123150 => { ins += 8;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x82;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80123170; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123170;
            }
            0x80123170 => { ins += 1;
                pc = 0x80123188; continue;
                pc = 0x80123174;
            }
            0x80123174 => { ins += 5;
                c.r[7] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x82;
                c.r[6] = 0x0;
                c.lr = 0x80123188; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123188;
            }
            0x80123188 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x8012318c;
            }
            0x8012318c => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80123110: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGetSex (0x801231a4, 316 bytes)
#[inline(never)]
pub fn f_801231a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801231a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801231a4); p = true; }
    }
    f_801231a4_b(c, 0x801231a4);
    if p { crate::prof::exit(c, 0x801231a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801231a4_at(c: &mut Cpu, pc: u32) { f_801231a4_b(c, pc) }
#[inline(always)]
fn f_801231a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x801231a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x801231a4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x801231a4 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                if !cr_bit(cr, 2) { pc = 0x801231cc; continue; }
                pc = 0x801231c4;
            }
            0x801231c4 => { ins += 2;
                c.r[3] = 0x2;
                pc = 0x801232c4; continue;
                pc = 0x801231cc;
            }
            0x801231cc => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x6f;
                c.r[6] = 0x0;
                c.lr = 0x801231dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801231dc;
            }
            0x801231dc => { ins += 6;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x801231f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x801231f4;
            }
            0x801231f4 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x13;
                c.r[6] = 0x0;
                c.lr = 0x80123208; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123208;
            }
            0x80123208 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                if !cr_bit(cr, 2) { pc = 0x8012321c; continue; }
                pc = 0x80123214;
            }
            0x80123214 => { ins += 2;
                c.r[3] = 0x2;
                pc = 0x801232a0; continue;
                pc = 0x8012321c;
            }
            0x8012321c => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x6e;
                c.r[6] = 0x0;
                c.lr = 0x80123230; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123230;
            }
            0x80123230 => { ins += 5;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x13;
                c.r[6] = 0x0;
                c.lr = 0x80123244; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123244;
            }
            0x80123244 => { ins += 3;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.lr = 0x80123250; { c.steps = st; c.cycles = ins; c.cr = cr; f_80131574(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123250;
            }
            0x80123250 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80123264; continue; }
                pc = 0x8012325c;
            }
            0x8012325c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x801232a0; continue;
                pc = 0x80123264;
            }
            0x80123264 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x8012326c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80131574(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8012326c;
            }
            0x8012326c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x80123280; continue; }
                pc = 0x80123278;
            }
            0x80123278 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801232a0; continue;
                pc = 0x80123280;
            }
            0x80123280 => { ins += 2;
                c.r[3] = 0x2;
                c.lr = 0x80123288; { c.steps = st; c.cycles = ins; c.cr = cr; f_80131574(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80123288;
            }
            0x80123288 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8012329c; continue; }
                pc = 0x80123294;
            }
            0x80123294 => { ins += 2;
                c.r[3] = 0x2;
                pc = 0x801232a0; continue;
                pc = 0x8012329c;
            }
            0x8012329c => { ins += 1;
                c.r[3] = 0xffffffff;
                pc = 0x801232a0;
            }
            0x801232a0 => { ins += 2;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x801232c0; continue; }
                pc = 0x801232a8;
            }
            0x801232a8 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x801232bc; continue; }
                pc = 0x801232b4;
            }
            0x801232b4 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x801232c0; continue;
                pc = 0x801232bc;
            }
            0x801232bc => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x801232c0;
            }
            0x801232c0 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801232c4;
            }
            0x801232c4 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801231a4: bad pc {:#x}", pc),
        }
    }
}

