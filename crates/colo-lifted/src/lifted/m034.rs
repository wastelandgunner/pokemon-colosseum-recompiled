#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fightTrainerAiDataBiosSetPokemonDataOrderOutFlag (0x801fc268, 16 bytes)
#[inline(never)]
pub fn f_801fc268(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc268) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc268); p = true; }
    }
    f_801fc268_b(c, 0x801fc268);
    if p { crate::prof::exit(c, 0x801fc268); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc268_at(c: &mut Cpu, pc: u32) { f_801fc268_b(c, pc) }
#[inline(always)]
fn f_801fc268_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc268;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc268, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc268 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc270;
            }
            0x801fc270 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc268: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetPokemonSelectWeakPointFlag (0x801fc278, 16 bytes)
#[inline(never)]
pub fn f_801fc278(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc278) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc278); p = true; }
    }
    f_801fc278_b(c, 0x801fc278);
    if p { crate::prof::exit(c, 0x801fc278); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc278_at(c: &mut Cpu, pc: u32) { f_801fc278_b(c, pc) }
#[inline(always)]
fn f_801fc278_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc278;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc278, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc278 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc280;
            }
            0x801fc280 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x1), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc278: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosSetPokemonSelectRandomFlag (0x801fc288, 16 bytes)
#[inline(never)]
pub fn f_801fc288(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc288) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc288); p = true; }
    }
    f_801fc288_b(c, 0x801fc288);
    if p { crate::prof::exit(c, 0x801fc288); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc288_at(c: &mut Cpu, pc: u32) { f_801fc288_b(c, pc) }
#[inline(always)]
fn f_801fc288_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc288;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc288, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc288 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc290;
            }
            0x801fc290 => { ins += 2;
                c.stb(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc288: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetLastValueRevise (0x801fc298, 24 bytes)
#[inline(never)]
pub fn f_801fc298(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc298) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc298); p = true; }
    }
    f_801fc298_b(c, 0x801fc298);
    if p { crate::prof::exit(c, 0x801fc298); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc298_at(c: &mut Cpu, pc: u32) { f_801fc298_b(c, pc) }
#[inline(always)]
fn f_801fc298_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc298;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc298, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc298 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc2a8; continue; }
                pc = 0x801fc2a0;
            }
            0x801fc2a0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc2a8;
            }
            0x801fc2a8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x19));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc298: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetWazaNokoriPpValue (0x801fc2b0, 24 bytes)
#[inline(never)]
pub fn f_801fc2b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc2b0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc2b0); p = true; }
    }
    f_801fc2b0_b(c, 0x801fc2b0);
    if p { crate::prof::exit(c, 0x801fc2b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc2b0_at(c: &mut Cpu, pc: u32) { f_801fc2b0_b(c, pc) }
#[inline(always)]
fn f_801fc2b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc2b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc2b0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc2b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc2c0; continue; }
                pc = 0x801fc2b8;
            }
            0x801fc2b8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc2c0;
            }
            0x801fc2c0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x18));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc2b0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetWazaRiskFlag (0x801fc2c8, 24 bytes)
#[inline(never)]
pub fn f_801fc2c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc2c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc2c8); p = true; }
    }
    f_801fc2c8_b(c, 0x801fc2c8);
    if p { crate::prof::exit(c, 0x801fc2c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc2c8_at(c: &mut Cpu, pc: u32) { f_801fc2c8_b(c, pc) }
#[inline(always)]
fn f_801fc2c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc2c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc2c8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc2c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc2d8; continue; }
                pc = 0x801fc2d0;
            }
            0x801fc2d0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc2d8;
            }
            0x801fc2d8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x17));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc2c8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetWazaAvgValue (0x801fc2e0, 24 bytes)
#[inline(never)]
pub fn f_801fc2e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc2e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc2e0); p = true; }
    }
    f_801fc2e0_b(c, 0x801fc2e0);
    if p { crate::prof::exit(c, 0x801fc2e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc2e0_at(c: &mut Cpu, pc: u32) { f_801fc2e0_b(c, pc) }
#[inline(always)]
fn f_801fc2e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc2e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc2e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc2e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc2f0; continue; }
                pc = 0x801fc2e8;
            }
            0x801fc2e8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc2f0;
            }
            0x801fc2f0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x16));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc2e0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetWazaHitFlag (0x801fc2f8, 24 bytes)
#[inline(never)]
pub fn f_801fc2f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc2f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc2f8); p = true; }
    }
    f_801fc2f8_b(c, 0x801fc2f8);
    if p { crate::prof::exit(c, 0x801fc2f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc2f8_at(c: &mut Cpu, pc: u32) { f_801fc2f8_b(c, pc) }
#[inline(always)]
fn f_801fc2f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc2f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc2f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc2f8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc308; continue; }
                pc = 0x801fc300;
            }
            0x801fc300 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc308;
            }
            0x801fc308 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x15));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc2f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetWazaTypeReviseValue (0x801fc310, 84 bytes)
#[inline(never)]
pub fn f_801fc310(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc310) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc310); p = true; }
    }
    f_801fc310_b(c, 0x801fc310);
    if p { crate::prof::exit(c, 0x801fc310); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc310_at(c: &mut Cpu, pc: u32) { f_801fc310_b(c, pc) }
#[inline(always)]
fn f_801fc310_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc310;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc310, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc310 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc320; continue; }
                pc = 0x801fc318;
            }
            0x801fc318 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc320;
            }
            0x801fc320 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fc32c; continue; }
                pc = 0x801fc324;
            }
            0x801fc324 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc34c; continue;
                pc = 0x801fc32c;
            }
            0x801fc32c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801fc340; continue; }
                pc = 0x801fc338;
            }
            0x801fc338 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc34c; continue;
                pc = 0x801fc340;
            }
            0x801fc340 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[4] = c.r[4].wrapping_add(0x24);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc34c;
            }
            0x801fc34c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc35c; continue; }
                pc = 0x801fc354;
            }
            0x801fc354 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc35c;
            }
            0x801fc35c => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[4].wrapping_add(0x1));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc310: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetWazaTypeReviseTypeDataId (0x801fc364, 84 bytes)
#[inline(never)]
pub fn f_801fc364(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc364) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc364); p = true; }
    }
    f_801fc364_b(c, 0x801fc364);
    if p { crate::prof::exit(c, 0x801fc364); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc364_at(c: &mut Cpu, pc: u32) { f_801fc364_b(c, pc) }
#[inline(always)]
fn f_801fc364_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc364;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc364, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc364 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc374; continue; }
                pc = 0x801fc36c;
            }
            0x801fc36c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc374;
            }
            0x801fc374 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fc380; continue; }
                pc = 0x801fc378;
            }
            0x801fc378 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc3a0; continue;
                pc = 0x801fc380;
            }
            0x801fc380 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801fc394; continue; }
                pc = 0x801fc38c;
            }
            0x801fc38c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc3a0; continue;
                pc = 0x801fc394;
            }
            0x801fc394 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[4] = c.r[4].wrapping_add(0x24);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc3a0;
            }
            0x801fc3a0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc3b0; continue; }
                pc = 0x801fc3a8;
            }
            0x801fc3a8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc3b0;
            }
            0x801fc3b0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc364: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetPartFlag (0x801fc3b8, 24 bytes)
#[inline(never)]
pub fn f_801fc3b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc3b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc3b8); p = true; }
    }
    f_801fc3b8_b(c, 0x801fc3b8);
    if p { crate::prof::exit(c, 0x801fc3b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc3b8_at(c: &mut Cpu, pc: u32) { f_801fc3b8_b(c, pc) }
#[inline(always)]
fn f_801fc3b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc3b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc3b8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc3b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc3c8; continue; }
                pc = 0x801fc3c0;
            }
            0x801fc3c0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc3c8;
            }
            0x801fc3c8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x14));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc3b8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetWazaInitValueFlag (0x801fc3d0, 24 bytes)
#[inline(never)]
pub fn f_801fc3d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc3d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc3d0); p = true; }
    }
    f_801fc3d0_b(c, 0x801fc3d0);
    if p { crate::prof::exit(c, 0x801fc3d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc3d0_at(c: &mut Cpu, pc: u32) { f_801fc3d0_b(c, pc) }
#[inline(always)]
fn f_801fc3d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc3d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc3d0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc3d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc3e0; continue; }
                pc = 0x801fc3d8;
            }
            0x801fc3d8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc3e0;
            }
            0x801fc3e0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x13));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc3d0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetWazaRndSelectFlag (0x801fc3e8, 24 bytes)
#[inline(never)]
pub fn f_801fc3e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc3e8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc3e8); p = true; }
    }
    f_801fc3e8_b(c, 0x801fc3e8);
    if p { crate::prof::exit(c, 0x801fc3e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc3e8_at(c: &mut Cpu, pc: u32) { f_801fc3e8_b(c, pc) }
#[inline(always)]
fn f_801fc3e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc3e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc3e8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc3e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc3f8; continue; }
                pc = 0x801fc3f0;
            }
            0x801fc3f0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc3f8;
            }
            0x801fc3f8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x12));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc3e8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetDefensePokemonRndSelectFlag (0x801fc400, 24 bytes)
#[inline(never)]
pub fn f_801fc400(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc400) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc400); p = true; }
    }
    f_801fc400_b(c, 0x801fc400);
    if p { crate::prof::exit(c, 0x801fc400); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc400_at(c: &mut Cpu, pc: u32) { f_801fc400_b(c, pc) }
#[inline(always)]
fn f_801fc400_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc400;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc400, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc400 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc410; continue; }
                pc = 0x801fc408;
            }
            0x801fc408 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc410;
            }
            0x801fc410 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x11));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc400: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetAbicntMinValue (0x801fc418, 24 bytes)
#[inline(never)]
pub fn f_801fc418(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc418) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc418); p = true; }
    }
    f_801fc418_b(c, 0x801fc418);
    if p { crate::prof::exit(c, 0x801fc418); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc418_at(c: &mut Cpu, pc: u32) { f_801fc418_b(c, pc) }
#[inline(always)]
fn f_801fc418_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc418;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc418, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc418 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc428; continue; }
                pc = 0x801fc420;
            }
            0x801fc420 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc428;
            }
            0x801fc428 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x10));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc418: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetAbicntMaxValue (0x801fc430, 24 bytes)
#[inline(never)]
pub fn f_801fc430(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc430) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc430); p = true; }
    }
    f_801fc430_b(c, 0x801fc430);
    if p { crate::prof::exit(c, 0x801fc430); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc430_at(c: &mut Cpu, pc: u32) { f_801fc430_b(c, pc) }
#[inline(always)]
fn f_801fc430_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc430;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc430, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc430 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc440; continue; }
                pc = 0x801fc438;
            }
            0x801fc438 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc440;
            }
            0x801fc440 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xf));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc430: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetWazaDamageFlag (0x801fc448, 24 bytes)
#[inline(never)]
pub fn f_801fc448(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc448) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc448); p = true; }
    }
    f_801fc448_b(c, 0x801fc448);
    if p { crate::prof::exit(c, 0x801fc448); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc448_at(c: &mut Cpu, pc: u32) { f_801fc448_b(c, pc) }
#[inline(always)]
fn f_801fc448_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc448;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc448, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc448 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc458; continue; }
                pc = 0x801fc450;
            }
            0x801fc450 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc458;
            }
            0x801fc458 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xe));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc448: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetNokoriHpValue (0x801fc460, 24 bytes)
#[inline(never)]
pub fn f_801fc460(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc460) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc460); p = true; }
    }
    f_801fc460_b(c, 0x801fc460);
    if p { crate::prof::exit(c, 0x801fc460); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc460_at(c: &mut Cpu, pc: u32) { f_801fc460_b(c, pc) }
#[inline(always)]
fn f_801fc460_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc460;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc460, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc460 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc470; continue; }
                pc = 0x801fc468;
            }
            0x801fc468 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc470;
            }
            0x801fc470 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xd));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc460: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetTokuseiCheckFlag (0x801fc478, 24 bytes)
#[inline(never)]
pub fn f_801fc478(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc478) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc478); p = true; }
    }
    f_801fc478_b(c, 0x801fc478);
    if p { crate::prof::exit(c, 0x801fc478); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc478_at(c: &mut Cpu, pc: u32) { f_801fc478_b(c, pc) }
#[inline(always)]
fn f_801fc478_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc478;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc478, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc478 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc488; continue; }
                pc = 0x801fc480;
            }
            0x801fc480 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc488;
            }
            0x801fc488 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc478: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetZokuseiReviseValue (0x801fc490, 84 bytes)
#[inline(never)]
pub fn f_801fc490(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc490) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc490); p = true; }
    }
    f_801fc490_b(c, 0x801fc490);
    if p { crate::prof::exit(c, 0x801fc490); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc490_at(c: &mut Cpu, pc: u32) { f_801fc490_b(c, pc) }
#[inline(always)]
fn f_801fc490_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc490;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc490, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc490 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc4a0; continue; }
                pc = 0x801fc498;
            }
            0x801fc498 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc4a0;
            }
            0x801fc4a0 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fc4ac; continue; }
                pc = 0x801fc4a4;
            }
            0x801fc4a4 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc4cc; continue;
                pc = 0x801fc4ac;
            }
            0x801fc4ac => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801fc4c0; continue; }
                pc = 0x801fc4b8;
            }
            0x801fc4b8 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc4cc; continue;
                pc = 0x801fc4c0;
            }
            0x801fc4c0 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[4] = c.r[4].wrapping_add(0x20);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc4cc;
            }
            0x801fc4cc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc4dc; continue; }
                pc = 0x801fc4d4;
            }
            0x801fc4d4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc4dc;
            }
            0x801fc4dc => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[4].wrapping_add(0x1));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc490: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetZokuseiReviseZokuseiDataId (0x801fc4e4, 84 bytes)
#[inline(never)]
pub fn f_801fc4e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc4e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc4e4); p = true; }
    }
    f_801fc4e4_b(c, 0x801fc4e4);
    if p { crate::prof::exit(c, 0x801fc4e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc4e4_at(c: &mut Cpu, pc: u32) { f_801fc4e4_b(c, pc) }
#[inline(always)]
fn f_801fc4e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc4e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc4e4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc4e4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc4f4; continue; }
                pc = 0x801fc4ec;
            }
            0x801fc4ec => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc4f4;
            }
            0x801fc4f4 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x801fc500; continue; }
                pc = 0x801fc4f8;
            }
            0x801fc4f8 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc520; continue;
                pc = 0x801fc500;
            }
            0x801fc500 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801fc514; continue; }
                pc = 0x801fc50c;
            }
            0x801fc50c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc520; continue;
                pc = 0x801fc514;
            }
            0x801fc514 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[4] = c.r[4].wrapping_add(0x20);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc520;
            }
            0x801fc520 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc530; continue; }
                pc = 0x801fc528;
            }
            0x801fc528 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc530;
            }
            0x801fc530 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc4e4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetZokuseiCheckFlag (0x801fc538, 24 bytes)
#[inline(never)]
pub fn f_801fc538(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc538) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc538); p = true; }
    }
    f_801fc538_b(c, 0x801fc538);
    if p { crate::prof::exit(c, 0x801fc538); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc538_at(c: &mut Cpu, pc: u32) { f_801fc538_b(c, pc) }
#[inline(always)]
fn f_801fc538_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc538;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc538, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc538 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc548; continue; }
                pc = 0x801fc540;
            }
            0x801fc540 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc548;
            }
            0x801fc548 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xb));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc538: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetParamStoreFlag (0x801fc550, 24 bytes)
#[inline(never)]
pub fn f_801fc550(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc550) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc550); p = true; }
    }
    f_801fc550_b(c, 0x801fc550);
    if p { crate::prof::exit(c, 0x801fc550); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc550_at(c: &mut Cpu, pc: u32) { f_801fc550_b(c, pc) }
#[inline(always)]
fn f_801fc550_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc550;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc550, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc550 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc560; continue; }
                pc = 0x801fc558;
            }
            0x801fc558 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc560;
            }
            0x801fc560 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc550: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetParamExpectFlag (0x801fc568, 24 bytes)
#[inline(never)]
pub fn f_801fc568(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc568) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc568); p = true; }
    }
    f_801fc568_b(c, 0x801fc568);
    if p { crate::prof::exit(c, 0x801fc568); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc568_at(c: &mut Cpu, pc: u32) { f_801fc568_b(c, pc) }
#[inline(always)]
fn f_801fc568_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc568;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc568, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc568 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc578; continue; }
                pc = 0x801fc570;
            }
            0x801fc570 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc578;
            }
            0x801fc578 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x9));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc568: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetItemValue (0x801fc580, 24 bytes)
#[inline(never)]
pub fn f_801fc580(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc580) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc580); p = true; }
    }
    f_801fc580_b(c, 0x801fc580);
    if p { crate::prof::exit(c, 0x801fc580); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc580_at(c: &mut Cpu, pc: u32) { f_801fc580_b(c, pc) }
#[inline(always)]
fn f_801fc580_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc580;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc580, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc580 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc590; continue; }
                pc = 0x801fc588;
            }
            0x801fc588 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc590;
            }
            0x801fc590 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc580: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetIrekaeValue (0x801fc598, 24 bytes)
#[inline(never)]
pub fn f_801fc598(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc598) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc598); p = true; }
    }
    f_801fc598_b(c, 0x801fc598);
    if p { crate::prof::exit(c, 0x801fc598); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc598_at(c: &mut Cpu, pc: u32) { f_801fc598_b(c, pc) }
#[inline(always)]
fn f_801fc598_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc598;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc598, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc598 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc5a8; continue; }
                pc = 0x801fc5a0;
            }
            0x801fc5a0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc5a8;
            }
            0x801fc5a8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x7));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc598: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetComboValue (0x801fc5b0, 24 bytes)
#[inline(never)]
pub fn f_801fc5b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc5b0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc5b0); p = true; }
    }
    f_801fc5b0_b(c, 0x801fc5b0);
    if p { crate::prof::exit(c, 0x801fc5b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc5b0_at(c: &mut Cpu, pc: u32) { f_801fc5b0_b(c, pc) }
#[inline(always)]
fn f_801fc5b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc5b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc5b0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc5b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc5c0; continue; }
                pc = 0x801fc5b8;
            }
            0x801fc5b8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc5c0;
            }
            0x801fc5c0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc5b0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetPokemonJoutaiFlag (0x801fc5c8, 24 bytes)
#[inline(never)]
pub fn f_801fc5c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc5c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc5c8); p = true; }
    }
    f_801fc5c8_b(c, 0x801fc5c8);
    if p { crate::prof::exit(c, 0x801fc5c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc5c8_at(c: &mut Cpu, pc: u32) { f_801fc5c8_b(c, pc) }
#[inline(always)]
fn f_801fc5c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc5c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc5c8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc5c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc5d8; continue; }
                pc = 0x801fc5d0;
            }
            0x801fc5d0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc5d8;
            }
            0x801fc5d8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x5));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc5c8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetKeyPlayerFlag (0x801fc5e0, 24 bytes)
#[inline(never)]
pub fn f_801fc5e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc5e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc5e0); p = true; }
    }
    f_801fc5e0_b(c, 0x801fc5e0);
    if p { crate::prof::exit(c, 0x801fc5e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc5e0_at(c: &mut Cpu, pc: u32) { f_801fc5e0_b(c, pc) }
#[inline(always)]
fn f_801fc5e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc5e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc5e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc5e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc5f0; continue; }
                pc = 0x801fc5e8;
            }
            0x801fc5e8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc5f0;
            }
            0x801fc5f0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc5e0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetPokemonDataOrderAceBossFlag (0x801fc5f8, 24 bytes)
#[inline(never)]
pub fn f_801fc5f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc5f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc5f8); p = true; }
    }
    f_801fc5f8_b(c, 0x801fc5f8);
    if p { crate::prof::exit(c, 0x801fc5f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc5f8_at(c: &mut Cpu, pc: u32) { f_801fc5f8_b(c, pc) }
#[inline(always)]
fn f_801fc5f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc5f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc5f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc5f8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc608; continue; }
                pc = 0x801fc600;
            }
            0x801fc600 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc608;
            }
            0x801fc608 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x3));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc5f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetPokemonDataOrderOutFlag (0x801fc610, 24 bytes)
#[inline(never)]
pub fn f_801fc610(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc610) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc610); p = true; }
    }
    f_801fc610_b(c, 0x801fc610);
    if p { crate::prof::exit(c, 0x801fc610); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc610_at(c: &mut Cpu, pc: u32) { f_801fc610_b(c, pc) }
#[inline(always)]
fn f_801fc610_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc610;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc610, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc610 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc620; continue; }
                pc = 0x801fc618;
            }
            0x801fc618 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc620;
            }
            0x801fc620 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc610: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetPokemonSelectWeakPointFlag (0x801fc628, 24 bytes)
#[inline(never)]
pub fn f_801fc628(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc628) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc628); p = true; }
    }
    f_801fc628_b(c, 0x801fc628);
    if p { crate::prof::exit(c, 0x801fc628); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc628_at(c: &mut Cpu, pc: u32) { f_801fc628_b(c, pc) }
#[inline(always)]
fn f_801fc628_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc628;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc628, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc628 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc638; continue; }
                pc = 0x801fc630;
            }
            0x801fc630 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc638;
            }
            0x801fc638 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc628: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetPokemonSelectRandomFlag (0x801fc640, 24 bytes)
#[inline(never)]
pub fn f_801fc640(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc640) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc640); p = true; }
    }
    f_801fc640_b(c, 0x801fc640);
    if p { crate::prof::exit(c, 0x801fc640); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc640_at(c: &mut Cpu, pc: u32) { f_801fc640_b(c, pc) }
#[inline(always)]
fn f_801fc640_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc640;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc640, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc640 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc650; continue; }
                pc = 0x801fc648;
            }
            0x801fc648 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc650;
            }
            0x801fc650 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc640: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerAiDataBiosGetPtr (0x801fc658, 44 bytes)
#[inline(never)]
pub fn f_801fc658(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc658) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc658); p = true; }
    }
    f_801fc658_b(c, 0x801fc658);
    if p { crate::prof::exit(c, 0x801fc658); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc658_at(c: &mut Cpu, pc: u32) { f_801fc658_b(c, pc) }
#[inline(always)]
fn f_801fc658_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc658;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc658, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc658 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8710));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801fc674; continue; }
                pc = 0x801fc66c;
            }
            0x801fc66c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc674;
            }
            0x801fc674 => { ins += 4;
                c.r[0] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8714));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc658: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetPartDataId (0x801fc684, 16 bytes)
#[inline(never)]
pub fn f_801fc684(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc684) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc684); p = true; }
    }
    f_801fc684_b(c, 0x801fc684);
    if p { crate::prof::exit(c, 0x801fc684); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc684_at(c: &mut Cpu, pc: u32) { f_801fc684_b(c, pc) }
#[inline(always)]
fn f_801fc684_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc684;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc684, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc684 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc68c;
            }
            0x801fc68c => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc684: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetKeyPlayerFlag (0x801fc694, 16 bytes)
#[inline(never)]
pub fn f_801fc694(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc694) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc694); p = true; }
    }
    f_801fc694_b(c, 0x801fc694);
    if p { crate::prof::exit(c, 0x801fc694); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc694_at(c: &mut Cpu, pc: u32) { f_801fc694_b(c, pc) }
#[inline(always)]
fn f_801fc694_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc694;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc694, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc694 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc69c;
            }
            0x801fc69c => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x5), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc694: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetPartDataId (0x801fc6a4, 24 bytes)
#[inline(never)]
pub fn f_801fc6a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc6a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc6a4); p = true; }
    }
    f_801fc6a4_b(c, 0x801fc6a4);
    if p { crate::prof::exit(c, 0x801fc6a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc6a4_at(c: &mut Cpu, pc: u32) { f_801fc6a4_b(c, pc) }
#[inline(always)]
fn f_801fc6a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc6a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc6a4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc6a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc6b4; continue; }
                pc = 0x801fc6ac;
            }
            0x801fc6ac => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc6b4;
            }
            0x801fc6b4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc6a4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetKeyPlayerFlag (0x801fc6bc, 24 bytes)
#[inline(never)]
pub fn f_801fc6bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc6bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc6bc); p = true; }
    }
    f_801fc6bc_b(c, 0x801fc6bc);
    if p { crate::prof::exit(c, 0x801fc6bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc6bc_at(c: &mut Cpu, pc: u32) { f_801fc6bc_b(c, pc) }
#[inline(always)]
fn f_801fc6bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc6bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc6bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc6bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc6cc; continue; }
                pc = 0x801fc6c4;
            }
            0x801fc6c4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc6cc;
            }
            0x801fc6cc => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x5));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc6bc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetSeikakuDataId (0x801fc6d4, 16 bytes)
#[inline(never)]
pub fn f_801fc6d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc6d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc6d4); p = true; }
    }
    f_801fc6d4_b(c, 0x801fc6d4);
    if p { crate::prof::exit(c, 0x801fc6d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc6d4_at(c: &mut Cpu, pc: u32) { f_801fc6d4_b(c, pc) }
#[inline(always)]
fn f_801fc6d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc6d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc6d4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc6d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc6dc;
            }
            0x801fc6dc => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc6d4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetSexDataId (0x801fc6e4, 16 bytes)
#[inline(never)]
pub fn f_801fc6e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc6e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc6e4); p = true; }
    }
    f_801fc6e4_b(c, 0x801fc6e4);
    if p { crate::prof::exit(c, 0x801fc6e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc6e4_at(c: &mut Cpu, pc: u32) { f_801fc6e4_b(c, pc) }
#[inline(always)]
fn f_801fc6e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc6e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc6e4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc6e4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc6ec;
            }
            0x801fc6ec => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x1), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc6e4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetFriend (0x801fc6f4, 16 bytes)
#[inline(never)]
pub fn f_801fc6f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc6f4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc6f4); p = true; }
    }
    f_801fc6f4_b(c, 0x801fc6f4);
    if p { crate::prof::exit(c, 0x801fc6f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc6f4_at(c: &mut Cpu, pc: u32) { f_801fc6f4_b(c, pc) }
#[inline(always)]
fn f_801fc6f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc6f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc6f4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc6f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc6fc;
            }
            0x801fc6fc => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc6f4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetPpCnt (0x801fc704, 64 bytes)
#[inline(never)]
pub fn f_801fc704(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc704) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc704); p = true; }
    }
    f_801fc704_b(c, 0x801fc704);
    if p { crate::prof::exit(c, 0x801fc704); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc704_at(c: &mut Cpu, pc: u32) { f_801fc704_b(c, pc) }
#[inline(always)]
fn f_801fc704_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc704;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc704, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc704 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc714; continue; }
                pc = 0x801fc70c;
            }
            0x801fc70c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc734; continue;
                pc = 0x801fc714;
            }
            0x801fc714 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801fc728; continue; }
                pc = 0x801fc720;
            }
            0x801fc720 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc734; continue;
                pc = 0x801fc728;
            }
            0x801fc728 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x30);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc734;
            }
            0x801fc734 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc73c;
            }
            0x801fc73c => { ins += 2;
                c.stb(c.r[4], c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc704: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetWazaDataId (0x801fc744, 64 bytes)
#[inline(never)]
pub fn f_801fc744(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc744) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc744); p = true; }
    }
    f_801fc744_b(c, 0x801fc744);
    if p { crate::prof::exit(c, 0x801fc744); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc744_at(c: &mut Cpu, pc: u32) { f_801fc744_b(c, pc) }
#[inline(always)]
fn f_801fc744_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc744;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc744, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc744 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc754; continue; }
                pc = 0x801fc74c;
            }
            0x801fc74c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc774; continue;
                pc = 0x801fc754;
            }
            0x801fc754 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801fc768; continue; }
                pc = 0x801fc760;
            }
            0x801fc760 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc774; continue;
                pc = 0x801fc768;
            }
            0x801fc768 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x30);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc774;
            }
            0x801fc774 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc77c;
            }
            0x801fc77c => { ins += 2;
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc744: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetItemDataId (0x801fc784, 16 bytes)
#[inline(never)]
pub fn f_801fc784(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc784) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc784); p = true; }
    }
    f_801fc784_b(c, 0x801fc784);
    if p { crate::prof::exit(c, 0x801fc784); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc784_at(c: &mut Cpu, pc: u32) { f_801fc784_b(c, pc) }
#[inline(always)]
fn f_801fc784_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc784;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc784, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc784 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc78c;
            }
            0x801fc78c => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc784: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetPokemonDataId (0x801fc794, 16 bytes)
#[inline(never)]
pub fn f_801fc794(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc794) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc794); p = true; }
    }
    f_801fc794_b(c, 0x801fc794);
    if p { crate::prof::exit(c, 0x801fc794); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc794_at(c: &mut Cpu, pc: u32) { f_801fc794_b(c, pc) }
#[inline(always)]
fn f_801fc794_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc794;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc794, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc794 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc79c;
            }
            0x801fc79c => { ins += 2;
                c.sth(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc794: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetTokuseiFlag (0x801fc7a4, 16 bytes)
#[inline(never)]
pub fn f_801fc7a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc7a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc7a4); p = true; }
    }
    f_801fc7a4_b(c, 0x801fc7a4);
    if p { crate::prof::exit(c, 0x801fc7a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc7a4_at(c: &mut Cpu, pc: u32) { f_801fc7a4_b(c, pc) }
#[inline(always)]
fn f_801fc7a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc7a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc7a4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc7a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc7ac;
            }
            0x801fc7ac => { ins += 2;
                c.stb(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc7a4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetDarkPokemonFlag (0x801fc7b4, 16 bytes)
#[inline(never)]
pub fn f_801fc7b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc7b4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc7b4); p = true; }
    }
    f_801fc7b4_b(c, 0x801fc7b4);
    if p { crate::prof::exit(c, 0x801fc7b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc7b4_at(c: &mut Cpu, pc: u32) { f_801fc7b4_b(c, pc) }
#[inline(always)]
fn f_801fc7b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc7b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc7b4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc7b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc7bc;
            }
            0x801fc7bc => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x3), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc7b4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetItemBallId (0x801fc7c4, 16 bytes)
#[inline(never)]
pub fn f_801fc7c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc7c4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc7c4); p = true; }
    }
    f_801fc7c4_b(c, 0x801fc7c4);
    if p { crate::prof::exit(c, 0x801fc7c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc7c4_at(c: &mut Cpu, pc: u32) { f_801fc7c4_b(c, pc) }
#[inline(always)]
fn f_801fc7c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc7c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc7c4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc7c4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc7cc;
            }
            0x801fc7cc => { ins += 2;
                c.sth(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc7c4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetLevel (0x801fc7d4, 16 bytes)
#[inline(never)]
pub fn f_801fc7d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc7d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc7d4); p = true; }
    }
    f_801fc7d4_b(c, 0x801fc7d4);
    if p { crate::prof::exit(c, 0x801fc7d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc7d4_at(c: &mut Cpu, pc: u32) { f_801fc7d4_b(c, pc) }
#[inline(always)]
fn f_801fc7d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc7d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc7d4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc7d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc7dc;
            }
            0x801fc7dc => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc7d4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetStatusEffort (0x801fc7e4, 36 bytes)
#[inline(never)]
pub fn f_801fc7e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc7e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc7e4); p = true; }
    }
    f_801fc7e4_b(c, 0x801fc7e4);
    if p { crate::prof::exit(c, 0x801fc7e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc7e4_at(c: &mut Cpu, pc: u32) { f_801fc7e4_b(c, pc) }
#[inline(always)]
fn f_801fc7e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc7e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc7e4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc7e4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc7ec;
            }
            0x801fc7ec => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc7f8;
            }
            0x801fc7f8 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.sth(c.r[3].wrapping_add(0x22), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc7e4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetStatusRnd (0x801fc808, 32 bytes)
#[inline(never)]
pub fn f_801fc808(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc808) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc808); p = true; }
    }
    f_801fc808_b(c, 0x801fc808);
    if p { crate::prof::exit(c, 0x801fc808); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc808_at(c: &mut Cpu, pc: u32) { f_801fc808_b(c, pc) }
#[inline(always)]
fn f_801fc808_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc808;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc808, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc808 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc810;
            }
            0x801fc810 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc81c;
            }
            0x801fc81c => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x1c), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc808: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosSetNickname (0x801fc828, 16 bytes)
#[inline(never)]
pub fn f_801fc828(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc828) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc828); p = true; }
    }
    f_801fc828_b(c, 0x801fc828);
    if p { crate::prof::exit(c, 0x801fc828); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc828_at(c: &mut Cpu, pc: u32) { f_801fc828_b(c, pc) }
#[inline(always)]
fn f_801fc828_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc828;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc828, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc828 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc830;
            }
            0x801fc830 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x14), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc828: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetSeikakuDataId (0x801fc838, 28 bytes)
#[inline(never)]
pub fn f_801fc838(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc838) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc838); p = true; }
    }
    f_801fc838_b(c, 0x801fc838);
    if p { crate::prof::exit(c, 0x801fc838); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc838_at(c: &mut Cpu, pc: u32) { f_801fc838_b(c, pc) }
#[inline(always)]
fn f_801fc838_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc838;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc838, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc838 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc848; continue; }
                pc = 0x801fc840;
            }
            0x801fc840 => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc848;
            }
            0x801fc848 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x2));
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc838: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetSexDataId (0x801fc854, 28 bytes)
#[inline(never)]
pub fn f_801fc854(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc854) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc854); p = true; }
    }
    f_801fc854_b(c, 0x801fc854);
    if p { crate::prof::exit(c, 0x801fc854); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc854_at(c: &mut Cpu, pc: u32) { f_801fc854_b(c, pc) }
#[inline(always)]
fn f_801fc854_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc854;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc854, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc854 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc864; continue; }
                pc = 0x801fc85c;
            }
            0x801fc85c => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc864;
            }
            0x801fc864 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc854: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetFriend (0x801fc870, 24 bytes)
#[inline(never)]
pub fn f_801fc870(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc870) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc870); p = true; }
    }
    f_801fc870_b(c, 0x801fc870);
    if p { crate::prof::exit(c, 0x801fc870); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc870_at(c: &mut Cpu, pc: u32) { f_801fc870_b(c, pc) }
#[inline(always)]
fn f_801fc870_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc870;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc870, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc870 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc880; continue; }
                pc = 0x801fc878;
            }
            0x801fc878 => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc880;
            }
            0x801fc880 => { ins += 2;
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc870: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetPpCnt (0x801fc888, 72 bytes)
#[inline(never)]
pub fn f_801fc888(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc888) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc888); p = true; }
    }
    f_801fc888_b(c, 0x801fc888);
    if p { crate::prof::exit(c, 0x801fc888); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc888_at(c: &mut Cpu, pc: u32) { f_801fc888_b(c, pc) }
#[inline(always)]
fn f_801fc888_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc888;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc888, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc888 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc898; continue; }
                pc = 0x801fc890;
            }
            0x801fc890 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc8b8; continue;
                pc = 0x801fc898;
            }
            0x801fc898 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801fc8ac; continue; }
                pc = 0x801fc8a4;
            }
            0x801fc8a4 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc8b8; continue;
                pc = 0x801fc8ac;
            }
            0x801fc8ac => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x30);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc8b8;
            }
            0x801fc8b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc8c8; continue; }
                pc = 0x801fc8c0;
            }
            0x801fc8c0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc8c8;
            }
            0x801fc8c8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc888: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetWazaDataId (0x801fc8d0, 72 bytes)
#[inline(never)]
pub fn f_801fc8d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc8d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc8d0); p = true; }
    }
    f_801fc8d0_b(c, 0x801fc8d0);
    if p { crate::prof::exit(c, 0x801fc8d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc8d0_at(c: &mut Cpu, pc: u32) { f_801fc8d0_b(c, pc) }
#[inline(always)]
fn f_801fc8d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc8d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc8d0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc8d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc8e0; continue; }
                pc = 0x801fc8d8;
            }
            0x801fc8d8 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc900; continue;
                pc = 0x801fc8e0;
            }
            0x801fc8e0 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801fc8f4; continue; }
                pc = 0x801fc8ec;
            }
            0x801fc8ec => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x801fc900; continue;
                pc = 0x801fc8f4;
            }
            0x801fc8f4 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x30);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801fc900;
            }
            0x801fc900 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc910; continue; }
                pc = 0x801fc908;
            }
            0x801fc908 => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc910;
            }
            0x801fc910 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc8d0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetItemDataId (0x801fc918, 24 bytes)
#[inline(never)]
pub fn f_801fc918(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc918) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc918); p = true; }
    }
    f_801fc918_b(c, 0x801fc918);
    if p { crate::prof::exit(c, 0x801fc918); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc918_at(c: &mut Cpu, pc: u32) { f_801fc918_b(c, pc) }
#[inline(always)]
fn f_801fc918_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc918;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc918, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc918 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc928; continue; }
                pc = 0x801fc920;
            }
            0x801fc920 => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc928;
            }
            0x801fc928 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x10));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc918: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetPokemonDataId (0x801fc930, 24 bytes)
#[inline(never)]
pub fn f_801fc930(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc930) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc930); p = true; }
    }
    f_801fc930_b(c, 0x801fc930);
    if p { crate::prof::exit(c, 0x801fc930); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc930_at(c: &mut Cpu, pc: u32) { f_801fc930_b(c, pc) }
#[inline(always)]
fn f_801fc930_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc930;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc930, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc930 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc940; continue; }
                pc = 0x801fc938;
            }
            0x801fc938 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc940;
            }
            0x801fc940 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc930: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetTokuseiFlag (0x801fc948, 28 bytes)
#[inline(never)]
pub fn f_801fc948(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc948) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc948); p = true; }
    }
    f_801fc948_b(c, 0x801fc948);
    if p { crate::prof::exit(c, 0x801fc948); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc948_at(c: &mut Cpu, pc: u32) { f_801fc948_b(c, pc) }
#[inline(always)]
fn f_801fc948_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc948;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc948, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc948 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc958; continue; }
                pc = 0x801fc950;
            }
            0x801fc950 => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc958;
            }
            0x801fc958 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3]);
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc948: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetDarkPokemonFlag (0x801fc964, 24 bytes)
#[inline(never)]
pub fn f_801fc964(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc964) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc964); p = true; }
    }
    f_801fc964_b(c, 0x801fc964);
    if p { crate::prof::exit(c, 0x801fc964); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc964_at(c: &mut Cpu, pc: u32) { f_801fc964_b(c, pc) }
#[inline(always)]
fn f_801fc964_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc964;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc964, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc964 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc974; continue; }
                pc = 0x801fc96c;
            }
            0x801fc96c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc974;
            }
            0x801fc974 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x3));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc964: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetItemBallId (0x801fc97c, 24 bytes)
#[inline(never)]
pub fn f_801fc97c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc97c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc97c); p = true; }
    }
    f_801fc97c_b(c, 0x801fc97c);
    if p { crate::prof::exit(c, 0x801fc97c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc97c_at(c: &mut Cpu, pc: u32) { f_801fc97c_b(c, pc) }
#[inline(always)]
fn f_801fc97c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc97c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc97c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc97c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc98c; continue; }
                pc = 0x801fc984;
            }
            0x801fc984 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc98c;
            }
            0x801fc98c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc97c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetLevel (0x801fc994, 24 bytes)
#[inline(never)]
pub fn f_801fc994(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc994) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc994); p = true; }
    }
    f_801fc994_b(c, 0x801fc994);
    if p { crate::prof::exit(c, 0x801fc994); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc994_at(c: &mut Cpu, pc: u32) { f_801fc994_b(c, pc) }
#[inline(always)]
fn f_801fc994_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc994;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc994, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc994 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc9a4; continue; }
                pc = 0x801fc99c;
            }
            0x801fc99c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc9a4;
            }
            0x801fc9a4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc994: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetStatusEffort (0x801fc9ac, 52 bytes)
#[inline(never)]
pub fn f_801fc9ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc9ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc9ac); p = true; }
    }
    f_801fc9ac_b(c, 0x801fc9ac);
    if p { crate::prof::exit(c, 0x801fc9ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc9ac_at(c: &mut Cpu, pc: u32) { f_801fc9ac_b(c, pc) }
#[inline(always)]
fn f_801fc9ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc9ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc9ac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc9ac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc9bc; continue; }
                pc = 0x801fc9b4;
            }
            0x801fc9b4 => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc9bc;
            }
            0x801fc9bc => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801fc9d0; continue; }
                pc = 0x801fc9c8;
            }
            0x801fc9c8 => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc9d0;
            }
            0x801fc9d0 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(0x22));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc9ac: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetStatusRnd (0x801fc9e0, 52 bytes)
#[inline(never)]
pub fn f_801fc9e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fc9e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fc9e0); p = true; }
    }
    f_801fc9e0_b(c, 0x801fc9e0);
    if p { crate::prof::exit(c, 0x801fc9e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fc9e0_at(c: &mut Cpu, pc: u32) { f_801fc9e0_b(c, pc) }
#[inline(always)]
fn f_801fc9e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fc9e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fc9e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fc9e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fc9f0; continue; }
                pc = 0x801fc9e8;
            }
            0x801fc9e8 => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fc9f0;
            }
            0x801fc9f0 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801fca04; continue; }
                pc = 0x801fc9fc;
            }
            0x801fc9fc => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fca04;
            }
            0x801fca04 => { ins += 4;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fc9e0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetNickname (0x801fca14, 24 bytes)
#[inline(never)]
pub fn f_801fca14(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fca14) { return; }
        if c.prof { crate::prof::enter(c, 0x801fca14); p = true; }
    }
    f_801fca14_b(c, 0x801fca14);
    if p { crate::prof::exit(c, 0x801fca14); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fca14_at(c: &mut Cpu, pc: u32) { f_801fca14_b(c, pc) }
#[inline(always)]
fn f_801fca14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fca14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fca14, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fca14 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fca24; continue; }
                pc = 0x801fca1c;
            }
            0x801fca1c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fca24;
            }
            0x801fca24 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x14));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fca14: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonDataBiosGetPtr (0x801fca2c, 44 bytes)
#[inline(never)]
pub fn f_801fca2c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fca2c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fca2c); p = true; }
    }
    f_801fca2c_b(c, 0x801fca2c);
    if p { crate::prof::exit(c, 0x801fca2c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fca2c_at(c: &mut Cpu, pc: u32) { f_801fca2c_b(c, pc) }
#[inline(always)]
fn f_801fca2c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fca2c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fca2c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fca2c => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86f0));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801fca48; continue; }
                pc = 0x801fca40;
            }
            0x801fca40 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fca48;
            }
            0x801fca48 => { ins += 4;
                c.r[0] = c.r[3].wrapping_mul(0x50);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86f4));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fca2c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonPartDataBiosSetWazaTypeRevise (0x801fca58, 32 bytes)
#[inline(never)]
pub fn f_801fca58(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fca58) { return; }
        if c.prof { crate::prof::enter(c, 0x801fca58); p = true; }
    }
    f_801fca58_b(c, 0x801fca58);
    if p { crate::prof::exit(c, 0x801fca58); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fca58_at(c: &mut Cpu, pc: u32) { f_801fca58_b(c, pc) }
#[inline(always)]
fn f_801fca58_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fca58;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fca58, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fca58 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fca60;
            }
            0x801fca60 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fca6c;
            }
            0x801fca6c => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x8), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fca58: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonPartDataBiosSetName (0x801fca78, 16 bytes)
#[inline(never)]
pub fn f_801fca78(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fca78) { return; }
        if c.prof { crate::prof::enter(c, 0x801fca78); p = true; }
    }
    f_801fca78_b(c, 0x801fca78);
    if p { crate::prof::exit(c, 0x801fca78); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fca78_at(c: &mut Cpu, pc: u32) { f_801fca78_b(c, pc) }
#[inline(always)]
fn f_801fca78_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fca78;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fca78, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fca78 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fca80;
            }
            0x801fca80 => { ins += 2;
                c.stw(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fca78: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonPartDataBiosGetWazaTypeRevise (0x801fca88, 48 bytes)
#[inline(never)]
pub fn f_801fca88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fca88) { return; }
        if c.prof { crate::prof::enter(c, 0x801fca88); p = true; }
    }
    f_801fca88_b(c, 0x801fca88);
    if p { crate::prof::exit(c, 0x801fca88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fca88_at(c: &mut Cpu, pc: u32) { f_801fca88_b(c, pc) }
#[inline(always)]
fn f_801fca88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fca88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fca88, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fca88 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fca98; continue; }
                pc = 0x801fca90;
            }
            0x801fca90 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fca98;
            }
            0x801fca98 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x801fcaac; continue; }
                pc = 0x801fcaa4;
            }
            0x801fcaa4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcaac;
            }
            0x801fcaac => { ins += 3;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fca88: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonPartDataBiosGetName (0x801fcab8, 24 bytes)
#[inline(never)]
pub fn f_801fcab8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcab8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcab8); p = true; }
    }
    f_801fcab8_b(c, 0x801fcab8);
    if p { crate::prof::exit(c, 0x801fcab8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcab8_at(c: &mut Cpu, pc: u32) { f_801fcab8_b(c, pc) }
#[inline(always)]
fn f_801fcab8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcab8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcab8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcab8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcac8; continue; }
                pc = 0x801fcac0;
            }
            0x801fcac0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcac8;
            }
            0x801fcac8 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcab8: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerPokemonPartDataBiosGetPtr (0x801fcad0, 44 bytes)
#[inline(never)]
pub fn f_801fcad0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcad0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcad0); p = true; }
    }
    f_801fcad0_b(c, 0x801fcad0);
    if p { crate::prof::exit(c, 0x801fcad0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcad0_at(c: &mut Cpu, pc: u32) { f_801fcad0_b(c, pc) }
#[inline(always)]
fn f_801fcad0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcad0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcad0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcad0 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86e8));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801fcaec; continue; }
                pc = 0x801fcae4;
            }
            0x801fcae4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcaec;
            }
            0x801fcaec => { ins += 4;
                c.r[0] = c.r[3].wrapping_mul(0x14);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86ec));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcad0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSet10 (0x801fcafc, 16 bytes)
#[inline(never)]
pub fn f_801fcafc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcafc) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcafc); p = true; }
    }
    f_801fcafc_b(c, 0x801fcafc);
    if p { crate::prof::exit(c, 0x801fcafc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcafc_at(c: &mut Cpu, pc: u32) { f_801fcafc_b(c, pc) }
#[inline(always)]
fn f_801fcafc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcafc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcafc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcafc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcb04;
            }
            0x801fcb04 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcafc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSet24Arr (0x801fcb0c, 36 bytes)
#[inline(never)]
pub fn f_801fcb0c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcb0c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcb0c); p = true; }
    }
    f_801fcb0c_b(c, 0x801fcb0c);
    if p { crate::prof::exit(c, 0x801fcb0c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcb0c_at(c: &mut Cpu, pc: u32) { f_801fcb0c_b(c, pc) }
#[inline(always)]
fn f_801fcb0c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcb0c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcb0c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcb0c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcb14;
            }
            0x801fcb14 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcb20;
            }
            0x801fcb20 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(2) & 0x3fc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x24), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcb0c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSet0C (0x801fcb30, 16 bytes)
#[inline(never)]
pub fn f_801fcb30(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcb30) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcb30); p = true; }
    }
    f_801fcb30_b(c, 0x801fcb30);
    if p { crate::prof::exit(c, 0x801fcb30); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcb30_at(c: &mut Cpu, pc: u32) { f_801fcb30_b(c, pc) }
#[inline(always)]
fn f_801fcb30_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcb30;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcb30, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcb30 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcb38;
            }
            0x801fcb38 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcb30: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSetU16Arr14 (0x801fcb40, 36 bytes)
#[inline(never)]
pub fn f_801fcb40(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcb40) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcb40); p = true; }
    }
    f_801fcb40_b(c, 0x801fcb40);
    if p { crate::prof::exit(c, 0x801fcb40); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcb40_at(c: &mut Cpu, pc: u32) { f_801fcb40_b(c, pc) }
#[inline(always)]
fn f_801fcb40_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcb40;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcb40, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcb40 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcb48;
            }
            0x801fcb48 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcb54;
            }
            0x801fcb54 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.sth(c.r[3].wrapping_add(0x14), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcb40: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSet04 (0x801fcb64, 16 bytes)
#[inline(never)]
pub fn f_801fcb64(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcb64) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcb64); p = true; }
    }
    f_801fcb64_b(c, 0x801fcb64);
    if p { crate::prof::exit(c, 0x801fcb64); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcb64_at(c: &mut Cpu, pc: u32) { f_801fcb64_b(c, pc) }
#[inline(always)]
fn f_801fcb64_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcb64;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcb64, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcb64 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcb6c;
            }
            0x801fcb6c => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcb64: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSet08 (0x801fcb74, 16 bytes)
#[inline(never)]
pub fn f_801fcb74(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcb74) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcb74); p = true; }
    }
    f_801fcb74_b(c, 0x801fcb74);
    if p { crate::prof::exit(c, 0x801fcb74); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcb74_at(c: &mut Cpu, pc: u32) { f_801fcb74_b(c, pc) }
#[inline(always)]
fn f_801fcb74_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcb74;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcb74, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcb74 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcb7c;
            }
            0x801fcb7c => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcb74: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSet06 (0x801fcb84, 16 bytes)
#[inline(never)]
pub fn f_801fcb84(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcb84) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcb84); p = true; }
    }
    f_801fcb84_b(c, 0x801fcb84);
    if p { crate::prof::exit(c, 0x801fcb84); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcb84_at(c: &mut Cpu, pc: u32) { f_801fcb84_b(c, pc) }
#[inline(always)]
fn f_801fcb84_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcb84;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcb84, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcb84 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcb8c;
            }
            0x801fcb8c => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcb84: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerSet00 (0x801fcb94, 16 bytes)
#[inline(never)]
pub fn f_801fcb94(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcb94) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcb94); p = true; }
    }
    f_801fcb94_b(c, 0x801fcb94);
    if p { crate::prof::exit(c, 0x801fcb94); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcb94_at(c: &mut Cpu, pc: u32) { f_801fcb94_b(c, pc) }
#[inline(always)]
fn f_801fcb94_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcb94;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcb94, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcb94 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcb9c;
            }
            0x801fcb9c => { ins += 2;
                c.stb(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcb94: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosGetStatus09 (0x801fcba4, 24 bytes)
#[inline(never)]
pub fn f_801fcba4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcba4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcba4); p = true; }
    }
    f_801fcba4_b(c, 0x801fcba4);
    if p { crate::prof::exit(c, 0x801fcba4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcba4_at(c: &mut Cpu, pc: u32) { f_801fcba4_b(c, pc) }
#[inline(always)]
fn f_801fcba4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcba4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcba4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcba4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcbb4; continue; }
                pc = 0x801fcbac;
            }
            0x801fcbac => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcbb4;
            }
            0x801fcbb4 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x10));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcba4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosGetStatus08 (0x801fcbbc, 52 bytes)
#[inline(never)]
pub fn f_801fcbbc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcbbc) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcbbc); p = true; }
    }
    f_801fcbbc_b(c, 0x801fcbbc);
    if p { crate::prof::exit(c, 0x801fcbbc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcbbc_at(c: &mut Cpu, pc: u32) { f_801fcbbc_b(c, pc) }
#[inline(always)]
fn f_801fcbbc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcbbc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcbbc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcbbc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcbcc; continue; }
                pc = 0x801fcbc4;
            }
            0x801fcbc4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcbcc;
            }
            0x801fcbcc => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801fcbe0; continue; }
                pc = 0x801fcbd8;
            }
            0x801fcbd8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcbe0;
            }
            0x801fcbe0 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(2) & 0x3fc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x24));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcbbc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosGetStatus07 (0x801fcbf0, 24 bytes)
#[inline(never)]
pub fn f_801fcbf0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcbf0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcbf0); p = true; }
    }
    f_801fcbf0_b(c, 0x801fcbf0);
    if p { crate::prof::exit(c, 0x801fcbf0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcbf0_at(c: &mut Cpu, pc: u32) { f_801fcbf0_b(c, pc) }
#[inline(always)]
fn f_801fcbf0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcbf0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcbf0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcbf0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcc00; continue; }
                pc = 0x801fcbf8;
            }
            0x801fcbf8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcc00;
            }
            0x801fcc00 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcbf0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosGetStatus06 (0x801fcc08, 52 bytes)
#[inline(never)]
pub fn f_801fcc08(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcc08) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcc08); p = true; }
    }
    f_801fcc08_b(c, 0x801fcc08);
    if p { crate::prof::exit(c, 0x801fcc08); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcc08_at(c: &mut Cpu, pc: u32) { f_801fcc08_b(c, pc) }
#[inline(always)]
fn f_801fcc08_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcc08;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcc08, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcc08 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcc18; continue; }
                pc = 0x801fcc10;
            }
            0x801fcc10 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcc18;
            }
            0x801fcc18 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if cr_bit(cr, 0) { pc = 0x801fcc2c; continue; }
                pc = 0x801fcc24;
            }
            0x801fcc24 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcc2c;
            }
            0x801fcc2c => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x14));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcc08: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosGetStatus05 (0x801fcc3c, 24 bytes)
#[inline(never)]
pub fn f_801fcc3c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcc3c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcc3c); p = true; }
    }
    f_801fcc3c_b(c, 0x801fcc3c);
    if p { crate::prof::exit(c, 0x801fcc3c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcc3c_at(c: &mut Cpu, pc: u32) { f_801fcc3c_b(c, pc) }
#[inline(always)]
fn f_801fcc3c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcc3c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcc3c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcc3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcc4c; continue; }
                pc = 0x801fcc44;
            }
            0x801fcc44 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcc4c;
            }
            0x801fcc4c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcc3c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosSetKindDataId (0x801fcc54, 16 bytes)
#[inline(never)]
pub fn f_801fcc54(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcc54) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcc54); p = true; }
    }
    f_801fcc54_b(c, 0x801fcc54);
    if p { crate::prof::exit(c, 0x801fcc54); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcc54_at(c: &mut Cpu, pc: u32) { f_801fcc54_b(c, pc) }
#[inline(always)]
fn f_801fcc54_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcc54;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcc54, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcc54 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcc5c;
            }
            0x801fcc5c => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcc54: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosGetKindDataId (0x801fcc64, 24 bytes)
#[inline(never)]
pub fn f_801fcc64(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcc64) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcc64); p = true; }
    }
    f_801fcc64_b(c, 0x801fcc64);
    if p { crate::prof::exit(c, 0x801fcc64); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcc64_at(c: &mut Cpu, pc: u32) { f_801fcc64_b(c, pc) }
#[inline(always)]
fn f_801fcc64_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcc64;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcc64, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcc64 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcc74; continue; }
                pc = 0x801fcc6c;
            }
            0x801fcc6c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcc74;
            }
            0x801fcc74 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcc64: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosGetStatus03 (0x801fcc7c, 24 bytes)
#[inline(never)]
pub fn f_801fcc7c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcc7c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcc7c); p = true; }
    }
    f_801fcc7c_b(c, 0x801fcc7c);
    if p { crate::prof::exit(c, 0x801fcc7c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcc7c_at(c: &mut Cpu, pc: u32) { f_801fcc7c_b(c, pc) }
#[inline(always)]
fn f_801fcc7c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcc7c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcc7c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcc7c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcc8c; continue; }
                pc = 0x801fcc84;
            }
            0x801fcc84 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcc8c;
            }
            0x801fcc8c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcc7c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosGetStatus02 (0x801fcc94, 24 bytes)
#[inline(never)]
pub fn f_801fcc94(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcc94) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcc94); p = true; }
    }
    f_801fcc94_b(c, 0x801fcc94);
    if p { crate::prof::exit(c, 0x801fcc94); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcc94_at(c: &mut Cpu, pc: u32) { f_801fcc94_b(c, pc) }
#[inline(always)]
fn f_801fcc94_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcc94;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcc94, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcc94 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcca4; continue; }
                pc = 0x801fcc9c;
            }
            0x801fcc9c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcca4;
            }
            0x801fcca4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcc94: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosGetStatus01 (0x801fccac, 24 bytes)
#[inline(never)]
pub fn f_801fccac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fccac) { return; }
        if c.prof { crate::prof::enter(c, 0x801fccac); p = true; }
    }
    f_801fccac_b(c, 0x801fccac);
    if p { crate::prof::exit(c, 0x801fccac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fccac_at(c: &mut Cpu, pc: u32) { f_801fccac_b(c, pc) }
#[inline(always)]
fn f_801fccac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fccac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fccac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fccac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fccbc; continue; }
                pc = 0x801fccb4;
            }
            0x801fccb4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fccbc;
            }
            0x801fccbc => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fccac: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainerDataBiosGetPtr (0x801fccc4, 44 bytes)
#[inline(never)]
pub fn f_801fccc4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fccc4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fccc4); p = true; }
    }
    f_801fccc4_b(c, 0x801fccc4);
    if p { crate::prof::exit(c, 0x801fccc4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fccc4_at(c: &mut Cpu, pc: u32) { f_801fccc4_b(c, pc) }
#[inline(always)]
fn f_801fccc4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fccc4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fccc4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fccc4 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8700));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x801fcce0; continue; }
                pc = 0x801fccd8;
            }
            0x801fccd8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcce0;
            }
            0x801fcce0 => { ins += 4;
                c.r[0] = c.r[3].wrapping_mul(0x34);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8704));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fccc4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetFightActionBuffPtr (0x801fccf0, 24 bytes)
#[inline(never)]
pub fn f_801fccf0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fccf0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fccf0); p = true; }
    }
    f_801fccf0_b(c, 0x801fccf0);
    if p { crate::prof::exit(c, 0x801fccf0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fccf0_at(c: &mut Cpu, pc: u32) { f_801fccf0_b(c, pc) }
#[inline(always)]
fn f_801fccf0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fccf0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fccf0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fccf0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcd00; continue; }
                pc = 0x801fccf8;
            }
            0x801fccf8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcd00;
            }
            0x801fcd00 => { ins += 2;
                c.r[3] = c.r[3].wrapping_add(0x27c4);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fccf0: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_SetControllerId (0x801fcd08, 16 bytes)
#[inline(never)]
pub fn f_801fcd08(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcd08) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcd08); p = true; }
    }
    f_801fcd08_b(c, 0x801fcd08);
    if p { crate::prof::exit(c, 0x801fcd08); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcd08_at(c: &mut Cpu, pc: u32) { f_801fcd08_b(c, pc) }
#[inline(always)]
fn f_801fcd08_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcd08;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcd08, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcd08 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcd10;
            }
            0x801fcd10 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x27bc), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcd08: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_SetNigeruCount (0x801fcd18, 16 bytes)
#[inline(never)]
pub fn f_801fcd18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcd18) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcd18); p = true; }
    }
    f_801fcd18_b(c, 0x801fcd18);
    if p { crate::prof::exit(c, 0x801fcd18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcd18_at(c: &mut Cpu, pc: u32) { f_801fcd18_b(c, pc) }
#[inline(always)]
fn f_801fcd18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcd18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcd18, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcd18 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcd20;
            }
            0x801fcd20 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x27b5), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcd18: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_SetKoban (0x801fcd28, 16 bytes)
#[inline(never)]
pub fn f_801fcd28(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcd28) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcd28); p = true; }
    }
    f_801fcd28_b(c, 0x801fcd28);
    if p { crate::prof::exit(c, 0x801fcd28); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcd28_at(c: &mut Cpu, pc: u32) { f_801fcd28_b(c, pc) }
#[inline(always)]
fn f_801fcd28_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcd28;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcd28, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcd28 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcd30;
            }
            0x801fcd30 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x27b8), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcd28: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_SetOkaneBai (0x801fcd38, 16 bytes)
#[inline(never)]
pub fn f_801fcd38(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcd38) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcd38); p = true; }
    }
    f_801fcd38_b(c, 0x801fcd38);
    if p { crate::prof::exit(c, 0x801fcd38); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcd38_at(c: &mut Cpu, pc: u32) { f_801fcd38_b(c, pc) }
#[inline(always)]
fn f_801fcd38_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcd38;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcd38, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcd38 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcd40;
            }
            0x801fcd40 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x27b4), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcd38: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_SetFightTrainerDataId (0x801fcd48, 16 bytes)
#[inline(never)]
pub fn f_801fcd48(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcd48) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcd48); p = true; }
    }
    f_801fcd48_b(c, 0x801fcd48);
    if p { crate::prof::exit(c, 0x801fcd48); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcd48_at(c: &mut Cpu, pc: u32) { f_801fcd48_b(c, pc) }
#[inline(always)]
fn f_801fcd48_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcd48;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcd48, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcd48 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcd50;
            }
            0x801fcd50 => { ins += 2;
                c.sth(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcd48: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetFightTrainerEnemyPokemonAryPtr (0x801fcd58, 52 bytes)
#[inline(never)]
pub fn f_801fcd58(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcd58) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcd58); p = true; }
    }
    f_801fcd58_b(c, 0x801fcd58);
    if p { crate::prof::exit(c, 0x801fcd58); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcd58_at(c: &mut Cpu, pc: u32) { f_801fcd58_b(c, pc) }
#[inline(always)]
fn f_801fcd58_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcd58;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcd58, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcd58 => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x801fcd68; continue; }
                pc = 0x801fcd60;
            }
            0x801fcd60 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcd68;
            }
            0x801fcd68 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xc);
                if cr_bit(cr, 0) { pc = 0x801fcd7c; continue; }
                pc = 0x801fcd74;
            }
            0x801fcd74 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcd7c;
            }
            0x801fcd7c => { ins += 4;
                c.r[3] = c.r[0].wrapping_mul(0x14);
                c.r[3] = c.r[3].wrapping_add(0x27f4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcd58: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_SetSequencePtr (0x801fcd8c, 16 bytes)
#[inline(never)]
pub fn f_801fcd8c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcd8c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcd8c); p = true; }
    }
    f_801fcd8c_b(c, 0x801fcd8c);
    if p { crate::prof::exit(c, 0x801fcd8c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcd8c_at(c: &mut Cpu, pc: u32) { f_801fcd8c_b(c, pc) }
#[inline(always)]
fn f_801fcd8c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcd8c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcd8c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcd8c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcd94;
            }
            0x801fcd94 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x27c0), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcd8c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetSequencePtr (0x801fcd9c, 24 bytes)
#[inline(never)]
pub fn f_801fcd9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcd9c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcd9c); p = true; }
    }
    f_801fcd9c_b(c, 0x801fcd9c);
    if p { crate::prof::exit(c, 0x801fcd9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcd9c_at(c: &mut Cpu, pc: u32) { f_801fcd9c_b(c, pc) }
#[inline(always)]
fn f_801fcd9c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcd9c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcd9c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcd9c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcdac; continue; }
                pc = 0x801fcda4;
            }
            0x801fcda4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcdac;
            }
            0x801fcdac => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x27c0));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcd9c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetControllerId (0x801fcdb4, 24 bytes)
#[inline(never)]
pub fn f_801fcdb4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcdb4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcdb4); p = true; }
    }
    f_801fcdb4_b(c, 0x801fcdb4);
    if p { crate::prof::exit(c, 0x801fcdb4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcdb4_at(c: &mut Cpu, pc: u32) { f_801fcdb4_b(c, pc) }
#[inline(always)]
fn f_801fcdb4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcdb4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcdb4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcdb4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcdc4; continue; }
                pc = 0x801fcdbc;
            }
            0x801fcdbc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcdc4;
            }
            0x801fcdc4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x27bc));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcdb4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetNigeruCount (0x801fcdcc, 24 bytes)
#[inline(never)]
pub fn f_801fcdcc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcdcc) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcdcc); p = true; }
    }
    f_801fcdcc_b(c, 0x801fcdcc);
    if p { crate::prof::exit(c, 0x801fcdcc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcdcc_at(c: &mut Cpu, pc: u32) { f_801fcdcc_b(c, pc) }
#[inline(always)]
fn f_801fcdcc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcdcc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcdcc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcdcc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcddc; continue; }
                pc = 0x801fcdd4;
            }
            0x801fcdd4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcddc;
            }
            0x801fcddc => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x27b5));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcdcc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetKoban (0x801fcde4, 24 bytes)
#[inline(never)]
pub fn f_801fcde4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcde4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcde4); p = true; }
    }
    f_801fcde4_b(c, 0x801fcde4);
    if p { crate::prof::exit(c, 0x801fcde4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcde4_at(c: &mut Cpu, pc: u32) { f_801fcde4_b(c, pc) }
#[inline(always)]
fn f_801fcde4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcde4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcde4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcde4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcdf4; continue; }
                pc = 0x801fcdec;
            }
            0x801fcdec => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcdf4;
            }
            0x801fcdf4 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x27b8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcde4: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetOkaneBai (0x801fcdfc, 24 bytes)
#[inline(never)]
pub fn f_801fcdfc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcdfc) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcdfc); p = true; }
    }
    f_801fcdfc_b(c, 0x801fcdfc);
    if p { crate::prof::exit(c, 0x801fcdfc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcdfc_at(c: &mut Cpu, pc: u32) { f_801fcdfc_b(c, pc) }
#[inline(always)]
fn f_801fcdfc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcdfc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcdfc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcdfc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fce0c; continue; }
                pc = 0x801fce04;
            }
            0x801fce04 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fce0c;
            }
            0x801fce0c => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x27b4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcdfc: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetFightoutPokemonBuffPtr (0x801fce14, 24 bytes)
#[inline(never)]
pub fn f_801fce14(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fce14) { return; }
        if c.prof { crate::prof::enter(c, 0x801fce14); p = true; }
    }
    f_801fce14_b(c, 0x801fce14);
    if p { crate::prof::exit(c, 0x801fce14); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fce14_at(c: &mut Cpu, pc: u32) { f_801fce14_b(c, pc) }
#[inline(always)]
fn f_801fce14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fce14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fce14, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fce14 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fce24; continue; }
                pc = 0x801fce1c;
            }
            0x801fce1c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fce24;
            }
            0x801fce24 => { ins += 2;
                c.r[3] = c.r[3].wrapping_add(0x20d4);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fce14: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetFightoutPokemonPtr (0x801fce2c, 52 bytes)
#[inline(never)]
pub fn f_801fce2c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fce2c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fce2c); p = true; }
    }
    f_801fce2c_b(c, 0x801fce2c);
    if p { crate::prof::exit(c, 0x801fce2c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fce2c_at(c: &mut Cpu, pc: u32) { f_801fce2c_b(c, pc) }
#[inline(always)]
fn f_801fce2c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fce2c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fce2c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fce2c => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x801fce3c; continue; }
                pc = 0x801fce34;
            }
            0x801fce34 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fce3c;
            }
            0x801fce3c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801fce50; continue; }
                pc = 0x801fce48;
            }
            0x801fce48 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fce50;
            }
            0x801fce50 => { ins += 4;
                c.r[3] = c.r[0].wrapping_mul(0x6e0);
                c.r[3] = c.r[3].wrapping_add(0x1314);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fce2c: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetFightPokemonPtr (0x801fce60, 52 bytes)
#[inline(never)]
pub fn f_801fce60(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fce60) { return; }
        if c.prof { crate::prof::enter(c, 0x801fce60); p = true; }
    }
    f_801fce60_b(c, 0x801fce60);
    if p { crate::prof::exit(c, 0x801fce60); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fce60_at(c: &mut Cpu, pc: u32) { f_801fce60_b(c, pc) }
#[inline(always)]
fn f_801fce60_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fce60;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fce60, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fce60 => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x801fce70; continue; }
                pc = 0x801fce68;
            }
            0x801fce68 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fce70;
            }
            0x801fce70 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x801fce84; continue; }
                pc = 0x801fce7c;
            }
            0x801fce7c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fce84;
            }
            0x801fce84 => { ins += 4;
                c.r[3] = c.r[0].wrapping_mul(0x154);
                c.r[3] = c.r[3].wrapping_add(0xb1c);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fce60: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetFightTrainerDataId (0x801fce94, 24 bytes)
#[inline(never)]
pub fn f_801fce94(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fce94) { return; }
        if c.prof { crate::prof::enter(c, 0x801fce94); p = true; }
    }
    f_801fce94_b(c, 0x801fce94);
    if p { crate::prof::exit(c, 0x801fce94); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fce94_at(c: &mut Cpu, pc: u32) { f_801fce94_b(c, pc) }
#[inline(always)]
fn f_801fce94_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fce94;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fce94, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fce94 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcea4; continue; }
                pc = 0x801fce9c;
            }
            0x801fce9c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcea4;
            }
            0x801fcea4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fce94: bad pc {:#x}", pc),
        }
    }
}

/// fightTrainer_GetHeroPtr (0x801fceac, 24 bytes)
#[inline(never)]
pub fn f_801fceac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fceac) { return; }
        if c.prof { crate::prof::enter(c, 0x801fceac); p = true; }
    }
    f_801fceac_b(c, 0x801fceac);
    if p { crate::prof::exit(c, 0x801fceac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fceac_at(c: &mut Cpu, pc: u32) { f_801fceac_b(c, pc) }
#[inline(always)]
fn f_801fceac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fceac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fceac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fceac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcebc; continue; }
                pc = 0x801fceb4;
            }
            0x801fceb4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcebc;
            }
            0x801fcebc => { ins += 2;
                c.r[3] = c.r[3].wrapping_add(0x4);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fceac: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosCopy (0x801fcec4, 56 bytes)
#[inline(never)]
pub fn f_801fcec4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcec4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcec4); p = true; }
    }
    f_801fcec4_b(c, 0x801fcec4);
    if p { crate::prof::exit(c, 0x801fcec4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcec4_at(c: &mut Cpu, pc: u32) { f_801fcec4_b(c, pc) }
#[inline(always)]
fn f_801fcec4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcec4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcec4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcec4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcecc;
            }
            0x801fcecc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fced4;
            }
            0x801fced4 => { ins += 4;
                c.r[0] = 0xdc;
                c.r[5] = c.r[3].wrapping_add(0xfffffffc);
                c.r[4] = c.r[4].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x801fcee4;
            }
            0x801fcee4 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x801fcee4; continue; }
                pc = 0x801fcef8;
            }
            0x801fcef8 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcec4: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetOumuWazaDataId (0x801fcefc, 16 bytes)
#[inline(never)]
pub fn f_801fcefc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcefc) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcefc); p = true; }
    }
    f_801fcefc_b(c, 0x801fcefc);
    if p { crate::prof::exit(c, 0x801fcefc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcefc_at(c: &mut Cpu, pc: u32) { f_801fcefc_b(c, pc) }
#[inline(always)]
fn f_801fcefc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcefc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcefc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcefc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcf04;
            }
            0x801fcf04 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x678), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcefc: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetGamanDamageTargetId (0x801fcf0c, 16 bytes)
#[inline(never)]
pub fn f_801fcf0c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcf0c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcf0c); p = true; }
    }
    f_801fcf0c_b(c, 0x801fcf0c);
    if p { crate::prof::exit(c, 0x801fcf0c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcf0c_at(c: &mut Cpu, pc: u32) { f_801fcf0c_b(c, pc) }
#[inline(always)]
fn f_801fcf0c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcf0c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcf0c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcf0c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcf14;
            }
            0x801fcf14 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x676), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcf0c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetGamanDamageValue (0x801fcf1c, 16 bytes)
#[inline(never)]
pub fn f_801fcf1c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcf1c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcf1c); p = true; }
    }
    f_801fcf1c_b(c, 0x801fcf1c);
    if p { crate::prof::exit(c, 0x801fcf1c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcf1c_at(c: &mut Cpu, pc: u32) { f_801fcf1c_b(c, pc) }
#[inline(always)]
fn f_801fcf1c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcf1c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcf1c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcf1c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcf24;
            }
            0x801fcf24 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x674), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcf1c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetHitWazaZokuseiDataId (0x801fcf2c, 16 bytes)
#[inline(never)]
pub fn f_801fcf2c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcf2c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcf2c); p = true; }
    }
    f_801fcf2c_b(c, 0x801fcf2c);
    if p { crate::prof::exit(c, 0x801fcf2c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcf2c_at(c: &mut Cpu, pc: u32) { f_801fcf2c_b(c, pc) }
#[inline(always)]
fn f_801fcf2c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcf2c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcf2c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcf2c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcf34;
            }
            0x801fcf34 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x672), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcf2c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetHitWazaDataId (0x801fcf3c, 16 bytes)
#[inline(never)]
pub fn f_801fcf3c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcf3c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcf3c); p = true; }
    }
    f_801fcf3c_b(c, 0x801fcf3c);
    if p { crate::prof::exit(c, 0x801fcf3c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcf3c_at(c: &mut Cpu, pc: u32) { f_801fcf3c_b(c, pc) }
#[inline(always)]
fn f_801fcf3c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcf3c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcf3c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcf3c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcf44;
            }
            0x801fcf44 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x670), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcf3c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetLastReceiveWazaTargetDataId (0x801fcf4c, 16 bytes)
#[inline(never)]
pub fn f_801fcf4c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcf4c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcf4c); p = true; }
    }
    f_801fcf4c_b(c, 0x801fcf4c);
    if p { crate::prof::exit(c, 0x801fcf4c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcf4c_at(c: &mut Cpu, pc: u32) { f_801fcf4c_b(c, pc) }
#[inline(always)]
fn f_801fcf4c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcf4c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcf4c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcf4c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcf54;
            }
            0x801fcf54 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x66e), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcf4c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetLastUseWazaDataId (0x801fcf5c, 16 bytes)
#[inline(never)]
pub fn f_801fcf5c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcf5c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcf5c); p = true; }
    }
    f_801fcf5c_b(c, 0x801fcf5c);
    if p { crate::prof::exit(c, 0x801fcf5c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcf5c_at(c: &mut Cpu, pc: u32) { f_801fcf5c_b(c, pc) }
#[inline(always)]
fn f_801fcf5c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcf5c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcf5c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcf5c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcf64;
            }
            0x801fcf64 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x66c), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcf5c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetLastSelectWazaDataId (0x801fcf6c, 16 bytes)
#[inline(never)]
pub fn f_801fcf6c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcf6c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcf6c); p = true; }
    }
    f_801fcf6c_b(c, 0x801fcf6c);
    if p { crate::prof::exit(c, 0x801fcf6c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcf6c_at(c: &mut Cpu, pc: u32) { f_801fcf6c_b(c, pc) }
#[inline(always)]
fn f_801fcf6c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcf6c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcf6c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcf6c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcf74;
            }
            0x801fcf74 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x66a), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcf6c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetSketchWazaDataId (0x801fcf7c, 16 bytes)
#[inline(never)]
pub fn f_801fcf7c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcf7c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcf7c); p = true; }
    }
    f_801fcf7c_b(c, 0x801fcf7c);
    if p { crate::prof::exit(c, 0x801fcf7c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcf7c_at(c: &mut Cpu, pc: u32) { f_801fcf7c_b(c, pc) }
#[inline(always)]
fn f_801fcf7c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcf7c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcf7c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcf7c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcf84;
            }
            0x801fcf84 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x668), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcf7c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetKeepFightWazaPtr (0x801fcf8c, 24 bytes)
#[inline(never)]
pub fn f_801fcf8c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcf8c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcf8c); p = true; }
    }
    f_801fcf8c_b(c, 0x801fcf8c);
    if p { crate::prof::exit(c, 0x801fcf8c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcf8c_at(c: &mut Cpu, pc: u32) { f_801fcf8c_b(c, pc) }
#[inline(always)]
fn f_801fcf8c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcf8c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcf8c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcf8c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcf9c; continue; }
                pc = 0x801fcf94;
            }
            0x801fcf94 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcf9c;
            }
            0x801fcf9c => { ins += 2;
                c.r[3] = c.r[3].wrapping_add(0x548);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcf8c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetOumuWazaDataId (0x801fcfa4, 24 bytes)
#[inline(never)]
pub fn f_801fcfa4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcfa4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcfa4); p = true; }
    }
    f_801fcfa4_b(c, 0x801fcfa4);
    if p { crate::prof::exit(c, 0x801fcfa4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcfa4_at(c: &mut Cpu, pc: u32) { f_801fcfa4_b(c, pc) }
#[inline(always)]
fn f_801fcfa4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcfa4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcfa4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcfa4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcfb4; continue; }
                pc = 0x801fcfac;
            }
            0x801fcfac => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcfb4;
            }
            0x801fcfb4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x678));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcfa4: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetGamanDamageTargetId (0x801fcfbc, 24 bytes)
#[inline(never)]
pub fn f_801fcfbc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcfbc) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcfbc); p = true; }
    }
    f_801fcfbc_b(c, 0x801fcfbc);
    if p { crate::prof::exit(c, 0x801fcfbc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcfbc_at(c: &mut Cpu, pc: u32) { f_801fcfbc_b(c, pc) }
#[inline(always)]
fn f_801fcfbc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcfbc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcfbc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcfbc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcfcc; continue; }
                pc = 0x801fcfc4;
            }
            0x801fcfc4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcfcc;
            }
            0x801fcfcc => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x676));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcfbc: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetGamanDamageValue (0x801fcfd4, 24 bytes)
#[inline(never)]
pub fn f_801fcfd4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcfd4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcfd4); p = true; }
    }
    f_801fcfd4_b(c, 0x801fcfd4);
    if p { crate::prof::exit(c, 0x801fcfd4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcfd4_at(c: &mut Cpu, pc: u32) { f_801fcfd4_b(c, pc) }
#[inline(always)]
fn f_801fcfd4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcfd4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcfd4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcfd4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcfe4; continue; }
                pc = 0x801fcfdc;
            }
            0x801fcfdc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcfe4;
            }
            0x801fcfe4 => { ins += 2;
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(0x674));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcfd4: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetHitWazaZokuseiDataId (0x801fcfec, 24 bytes)
#[inline(never)]
pub fn f_801fcfec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fcfec) { return; }
        if c.prof { crate::prof::enter(c, 0x801fcfec); p = true; }
    }
    f_801fcfec_b(c, 0x801fcfec);
    if p { crate::prof::exit(c, 0x801fcfec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fcfec_at(c: &mut Cpu, pc: u32) { f_801fcfec_b(c, pc) }
#[inline(always)]
fn f_801fcfec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fcfec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fcfec, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fcfec => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fcffc; continue; }
                pc = 0x801fcff4;
            }
            0x801fcff4 => { ins += 2;
                c.r[3] = 0x9;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fcffc;
            }
            0x801fcffc => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x672));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fcfec: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetHitWazaDataId (0x801fd004, 24 bytes)
#[inline(never)]
pub fn f_801fd004(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd004) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd004); p = true; }
    }
    f_801fd004_b(c, 0x801fd004);
    if p { crate::prof::exit(c, 0x801fd004); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd004_at(c: &mut Cpu, pc: u32) { f_801fd004_b(c, pc) }
#[inline(always)]
fn f_801fd004_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd004;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd004, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd004 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd014; continue; }
                pc = 0x801fd00c;
            }
            0x801fd00c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd014;
            }
            0x801fd014 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x670));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd004: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetLastReceiveWazaTargetDataId (0x801fd01c, 24 bytes)
#[inline(never)]
pub fn f_801fd01c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd01c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd01c); p = true; }
    }
    f_801fd01c_b(c, 0x801fd01c);
    if p { crate::prof::exit(c, 0x801fd01c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd01c_at(c: &mut Cpu, pc: u32) { f_801fd01c_b(c, pc) }
#[inline(always)]
fn f_801fd01c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd01c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd01c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd01c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd02c; continue; }
                pc = 0x801fd024;
            }
            0x801fd024 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd02c;
            }
            0x801fd02c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x66e));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd01c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetLastUseWazaDataId (0x801fd034, 24 bytes)
#[inline(never)]
pub fn f_801fd034(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd034) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd034); p = true; }
    }
    f_801fd034_b(c, 0x801fd034);
    if p { crate::prof::exit(c, 0x801fd034); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd034_at(c: &mut Cpu, pc: u32) { f_801fd034_b(c, pc) }
#[inline(always)]
fn f_801fd034_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd034;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd034, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd034 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd044; continue; }
                pc = 0x801fd03c;
            }
            0x801fd03c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd044;
            }
            0x801fd044 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x66c));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd034: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetLastSelectWazaDataId (0x801fd04c, 24 bytes)
#[inline(never)]
pub fn f_801fd04c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd04c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd04c); p = true; }
    }
    f_801fd04c_b(c, 0x801fd04c);
    if p { crate::prof::exit(c, 0x801fd04c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd04c_at(c: &mut Cpu, pc: u32) { f_801fd04c_b(c, pc) }
#[inline(always)]
fn f_801fd04c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd04c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd04c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd04c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd05c; continue; }
                pc = 0x801fd054;
            }
            0x801fd054 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd05c;
            }
            0x801fd05c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x66a));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd04c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetSketchWazaDataId (0x801fd064, 24 bytes)
#[inline(never)]
pub fn f_801fd064(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd064) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd064); p = true; }
    }
    f_801fd064_b(c, 0x801fd064);
    if p { crate::prof::exit(c, 0x801fd064); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd064_at(c: &mut Cpu, pc: u32) { f_801fd064_b(c, pc) }
#[inline(always)]
fn f_801fd064_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd064;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd064, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd064 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd074; continue; }
                pc = 0x801fd06c;
            }
            0x801fd06c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd074;
            }
            0x801fd074 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x668));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd064: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyBiosSetDamage (0x801fd07c, 16 bytes)
#[inline(never)]
pub fn f_801fd07c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd07c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd07c); p = true; }
    }
    f_801fd07c_b(c, 0x801fd07c);
    if p { crate::prof::exit(c, 0x801fd07c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd07c_at(c: &mut Cpu, pc: u32) { f_801fd07c_b(c, pc) }
#[inline(always)]
fn f_801fd07c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd07c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd07c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd07c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd084;
            }
            0x801fd084 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd07c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyBiosSetInitHp (0x801fd08c, 16 bytes)
#[inline(never)]
pub fn f_801fd08c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd08c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd08c); p = true; }
    }
    f_801fd08c_b(c, 0x801fd08c);
    if p { crate::prof::exit(c, 0x801fd08c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd08c_at(c: &mut Cpu, pc: u32) { f_801fd08c_b(c, pc) }
#[inline(always)]
fn f_801fd08c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd08c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd08c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd08c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd094;
            }
            0x801fd094 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd08c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyBiosSetOumuWazaDataId (0x801fd09c, 16 bytes)
#[inline(never)]
pub fn f_801fd09c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd09c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd09c); p = true; }
    }
    f_801fd09c_b(c, 0x801fd09c);
    if p { crate::prof::exit(c, 0x801fd09c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd09c_at(c: &mut Cpu, pc: u32) { f_801fd09c_b(c, pc) }
#[inline(always)]
fn f_801fd09c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd09c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd09c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd09c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd0a4;
            }
            0x801fd0a4 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd09c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyBiosSetTargetFightOutPokemonPtr (0x801fd0ac, 16 bytes)
#[inline(never)]
pub fn f_801fd0ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd0ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd0ac); p = true; }
    }
    f_801fd0ac_b(c, 0x801fd0ac);
    if p { crate::prof::exit(c, 0x801fd0ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd0ac_at(c: &mut Cpu, pc: u32) { f_801fd0ac_b(c, pc) }
#[inline(always)]
fn f_801fd0ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd0ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd0ac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd0ac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd0b4;
            }
            0x801fd0b4 => { ins += 2;
                c.stw(c.r[3], c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd0ac: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyBiosGetDamage (0x801fd0bc, 24 bytes)
#[inline(never)]
pub fn f_801fd0bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd0bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd0bc); p = true; }
    }
    f_801fd0bc_b(c, 0x801fd0bc);
    if p { crate::prof::exit(c, 0x801fd0bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd0bc_at(c: &mut Cpu, pc: u32) { f_801fd0bc_b(c, pc) }
#[inline(always)]
fn f_801fd0bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd0bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd0bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd0bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd0cc; continue; }
                pc = 0x801fd0c4;
            }
            0x801fd0c4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd0cc;
            }
            0x801fd0cc => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd0bc: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyBiosGetInitHp (0x801fd0d4, 24 bytes)
#[inline(never)]
pub fn f_801fd0d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd0d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd0d4); p = true; }
    }
    f_801fd0d4_b(c, 0x801fd0d4);
    if p { crate::prof::exit(c, 0x801fd0d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd0d4_at(c: &mut Cpu, pc: u32) { f_801fd0d4_b(c, pc) }
#[inline(always)]
fn f_801fd0d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd0d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd0d4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd0d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd0e4; continue; }
                pc = 0x801fd0dc;
            }
            0x801fd0dc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd0e4;
            }
            0x801fd0e4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd0d4: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyBiosGetOumuWazaDataId (0x801fd0ec, 24 bytes)
#[inline(never)]
pub fn f_801fd0ec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd0ec) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd0ec); p = true; }
    }
    f_801fd0ec_b(c, 0x801fd0ec);
    if p { crate::prof::exit(c, 0x801fd0ec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd0ec_at(c: &mut Cpu, pc: u32) { f_801fd0ec_b(c, pc) }
#[inline(always)]
fn f_801fd0ec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd0ec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd0ec, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd0ec => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd0fc; continue; }
                pc = 0x801fd0f4;
            }
            0x801fd0f4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd0fc;
            }
            0x801fd0fc => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd0ec: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonEnemyBiosGetTargetFightOutPokemonPtr (0x801fd104, 24 bytes)
#[inline(never)]
pub fn f_801fd104(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd104) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd104); p = true; }
    }
    f_801fd104_b(c, 0x801fd104);
    if p { crate::prof::exit(c, 0x801fd104); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd104_at(c: &mut Cpu, pc: u32) { f_801fd104_b(c, pc) }
#[inline(always)]
fn f_801fd104_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd104;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd104, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd104 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd114; continue; }
                pc = 0x801fd10c;
            }
            0x801fd10c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd114;
            }
            0x801fd114 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd104: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetFightOutPokemonEnemyPtr (0x801fd11c, 52 bytes)
#[inline(never)]
pub fn f_801fd11c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd11c) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd11c); p = true; }
    }
    f_801fd11c_b(c, 0x801fd11c);
    if p { crate::prof::exit(c, 0x801fd11c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd11c_at(c: &mut Cpu, pc: u32) { f_801fd11c_b(c, pc) }
#[inline(always)]
fn f_801fd11c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd11c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd11c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd11c => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x801fd12c; continue; }
                pc = 0x801fd124;
            }
            0x801fd124 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd12c;
            }
            0x801fd12c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x801fd140; continue; }
                pc = 0x801fd138;
            }
            0x801fd138 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd140;
            }
            0x801fd140 => { ins += 4;
                c.r[3] = c.r[0].wrapping_mul(0xc);
                c.r[3] = c.r[3].wrapping_add(0x6b0);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd11c: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetIrekaeTargetEntryId (0x801fd150, 16 bytes)
#[inline(never)]
pub fn f_801fd150(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd150) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd150); p = true; }
    }
    f_801fd150_b(c, 0x801fd150);
    if p { crate::prof::exit(c, 0x801fd150); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd150_at(c: &mut Cpu, pc: u32) { f_801fd150_b(c, pc) }
#[inline(always)]
fn f_801fd150_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd150;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd150, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd150 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd158;
            }
            0x801fd158 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x6ae), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd150: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetIrekaeTargetEntryId (0x801fd160, 24 bytes)
#[inline(never)]
pub fn f_801fd160(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd160) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd160); p = true; }
    }
    f_801fd160_b(c, 0x801fd160);
    if p { crate::prof::exit(c, 0x801fd160); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd160_at(c: &mut Cpu, pc: u32) { f_801fd160_b(c, pc) }
#[inline(always)]
fn f_801fd160_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd160;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd160, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd160 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd170; continue; }
                pc = 0x801fd168;
            }
            0x801fd168 => { ins += 2;
                c.r[3] = 0xffffffff;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd170;
            }
            0x801fd170 => { ins += 2;
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(0x6ae));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd160: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetKizetuFlag (0x801fd178, 16 bytes)
#[inline(never)]
pub fn f_801fd178(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd178) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd178); p = true; }
    }
    f_801fd178_b(c, 0x801fd178);
    if p { crate::prof::exit(c, 0x801fd178); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd178_at(c: &mut Cpu, pc: u32) { f_801fd178_b(c, pc) }
#[inline(always)]
fn f_801fd178_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd178;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd178, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd178 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd180;
            }
            0x801fd180 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x6ac), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd178: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetKizetuFlag (0x801fd188, 24 bytes)
#[inline(never)]
pub fn f_801fd188(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd188) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd188); p = true; }
    }
    f_801fd188_b(c, 0x801fd188);
    if p { crate::prof::exit(c, 0x801fd188); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd188_at(c: &mut Cpu, pc: u32) { f_801fd188_b(c, pc) }
#[inline(always)]
fn f_801fd188_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd188;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd188, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd188 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd198; continue; }
                pc = 0x801fd190;
            }
            0x801fd190 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd198;
            }
            0x801fd198 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x6ac));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd188: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetMyselfDamageSpeTargetId (0x801fd1a0, 16 bytes)
#[inline(never)]
pub fn f_801fd1a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd1a0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd1a0); p = true; }
    }
    f_801fd1a0_b(c, 0x801fd1a0);
    if p { crate::prof::exit(c, 0x801fd1a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd1a0_at(c: &mut Cpu, pc: u32) { f_801fd1a0_b(c, pc) }
#[inline(always)]
fn f_801fd1a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd1a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd1a0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd1a0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd1a8;
            }
            0x801fd1a8 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x6aa), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd1a0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetMyselfDamageSpeValue (0x801fd1b0, 16 bytes)
#[inline(never)]
pub fn f_801fd1b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd1b0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd1b0); p = true; }
    }
    f_801fd1b0_b(c, 0x801fd1b0);
    if p { crate::prof::exit(c, 0x801fd1b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd1b0_at(c: &mut Cpu, pc: u32) { f_801fd1b0_b(c, pc) }
#[inline(always)]
fn f_801fd1b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd1b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd1b0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd1b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd1b8;
            }
            0x801fd1b8 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x6a8), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd1b0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetMyselfDamageAtkTargetId (0x801fd1c0, 16 bytes)
#[inline(never)]
pub fn f_801fd1c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd1c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd1c0); p = true; }
    }
    f_801fd1c0_b(c, 0x801fd1c0);
    if p { crate::prof::exit(c, 0x801fd1c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd1c0_at(c: &mut Cpu, pc: u32) { f_801fd1c0_b(c, pc) }
#[inline(always)]
fn f_801fd1c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd1c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd1c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd1c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd1c8;
            }
            0x801fd1c8 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x6a6), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd1c0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetMyselfDamageAtkValue (0x801fd1d0, 16 bytes)
#[inline(never)]
pub fn f_801fd1d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd1d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd1d0); p = true; }
    }
    f_801fd1d0_b(c, 0x801fd1d0);
    if p { crate::prof::exit(c, 0x801fd1d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd1d0_at(c: &mut Cpu, pc: u32) { f_801fd1d0_b(c, pc) }
#[inline(always)]
fn f_801fd1d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd1d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd1d0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd1d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd1d8;
            }
            0x801fd1d8 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x6a4), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd1d0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetKaigaraDamageValue (0x801fd1e0, 16 bytes)
#[inline(never)]
pub fn f_801fd1e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd1e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd1e0); p = true; }
    }
    f_801fd1e0_b(c, 0x801fd1e0);
    if p { crate::prof::exit(c, 0x801fd1e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd1e0_at(c: &mut Cpu, pc: u32) { f_801fd1e0_b(c, pc) }
#[inline(always)]
fn f_801fd1e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd1e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd1e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd1e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd1e8;
            }
            0x801fd1e8 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x6a0), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd1e0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetItemKoraetaFlag (0x801fd1f0, 16 bytes)
#[inline(never)]
pub fn f_801fd1f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd1f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd1f0); p = true; }
    }
    f_801fd1f0_b(c, 0x801fd1f0);
    if p { crate::prof::exit(c, 0x801fd1f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd1f0_at(c: &mut Cpu, pc: u32) { f_801fd1f0_b(c, pc) }
#[inline(always)]
fn f_801fd1f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd1f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd1f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd1f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd1f8;
            }
            0x801fd1f8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x69e), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd1f0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetIrekaetaFlag (0x801fd200, 16 bytes)
#[inline(never)]
pub fn f_801fd200(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd200) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd200); p = true; }
    }
    f_801fd200_b(c, 0x801fd200);
    if p { crate::prof::exit(c, 0x801fd200); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd200_at(c: &mut Cpu, pc: u32) { f_801fd200_b(c, pc) }
#[inline(always)]
fn f_801fd200_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd200;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd200, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd200 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd208;
            }
            0x801fd208 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x69d), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd200: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetNoPressureFlag (0x801fd210, 16 bytes)
#[inline(never)]
pub fn f_801fd210(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd210) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd210); p = true; }
    }
    f_801fd210_b(c, 0x801fd210);
    if p { crate::prof::exit(c, 0x801fd210); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd210_at(c: &mut Cpu, pc: u32) { f_801fd210_b(c, pc) }
#[inline(always)]
fn f_801fd210_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd210;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd210, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd210 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd218;
            }
            0x801fd218 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x69c), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd210: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetDoTraceFlag (0x801fd220, 16 bytes)
#[inline(never)]
pub fn f_801fd220(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd220) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd220); p = true; }
    }
    f_801fd220_b(c, 0x801fd220);
    if p { crate::prof::exit(c, 0x801fd220); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd220_at(c: &mut Cpu, pc: u32) { f_801fd220_b(c, pc) }
#[inline(always)]
fn f_801fd220_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd220;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd220, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd220 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd228;
            }
            0x801fd228 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x69b), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd220: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetDoIkakuFlag (0x801fd230, 16 bytes)
#[inline(never)]
pub fn f_801fd230(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd230) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd230); p = true; }
    }
    f_801fd230_b(c, 0x801fd230);
    if p { crate::prof::exit(c, 0x801fd230); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd230_at(c: &mut Cpu, pc: u32) { f_801fd230_b(c, pc) }
#[inline(always)]
fn f_801fd230_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd230;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd230, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd230 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd238;
            }
            0x801fd238 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x69a), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd230: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetVanishoffFlag (0x801fd240, 16 bytes)
#[inline(never)]
pub fn f_801fd240(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd240) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd240); p = true; }
    }
    f_801fd240_b(c, 0x801fd240);
    if p { crate::prof::exit(c, 0x801fd240); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd240_at(c: &mut Cpu, pc: u32) { f_801fd240_b(c, pc) }
#[inline(always)]
fn f_801fd240_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd240;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd240, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd240 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd248;
            }
            0x801fd248 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x699), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd240: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetReceivesWazaHiraishinFlag (0x801fd250, 16 bytes)
#[inline(never)]
pub fn f_801fd250(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd250) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd250); p = true; }
    }
    f_801fd250_b(c, 0x801fd250);
    if p { crate::prof::exit(c, 0x801fd250); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd250_at(c: &mut Cpu, pc: u32) { f_801fd250_b(c, pc) }
#[inline(always)]
fn f_801fd250_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd250;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd250, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd250 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd258;
            }
            0x801fd258 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x698), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd250: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetDoClearbodyFlag (0x801fd260, 16 bytes)
#[inline(never)]
pub fn f_801fd260(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd260) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd260); p = true; }
    }
    f_801fd260_b(c, 0x801fd260);
    if p { crate::prof::exit(c, 0x801fd260); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd260_at(c: &mut Cpu, pc: u32) { f_801fd260_b(c, pc) }
#[inline(always)]
fn f_801fd260_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd260;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd260, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd260 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd268;
            }
            0x801fd268 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x697), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd260: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetFightActionFlag (0x801fd270, 16 bytes)
#[inline(never)]
pub fn f_801fd270(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd270) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd270); p = true; }
    }
    f_801fd270_b(c, 0x801fd270);
    if p { crate::prof::exit(c, 0x801fd270); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd270_at(c: &mut Cpu, pc: u32) { f_801fd270_b(c, pc) }
#[inline(always)]
fn f_801fd270_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd270;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd270, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd270 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd278;
            }
            0x801fd278 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x696), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd270: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetPassPpdecFlag (0x801fd280, 16 bytes)
#[inline(never)]
pub fn f_801fd280(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd280) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd280); p = true; }
    }
    f_801fd280_b(c, 0x801fd280);
    if p { crate::prof::exit(c, 0x801fd280); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd280_at(c: &mut Cpu, pc: u32) { f_801fd280_b(c, pc) }
#[inline(always)]
fn f_801fd280_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd280;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd280, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd280 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd288;
            }
            0x801fd288 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x695), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd280: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetHirumuNoAttackFlag (0x801fd290, 16 bytes)
#[inline(never)]
pub fn f_801fd290(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd290) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd290); p = true; }
    }
    f_801fd290_b(c, 0x801fd290);
    if p { crate::prof::exit(c, 0x801fd290); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd290_at(c: &mut Cpu, pc: u32) { f_801fd290_b(c, pc) }
#[inline(always)]
fn f_801fd290_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd290;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd290, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd290 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd298;
            }
            0x801fd298 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x694), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd290: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetIchamonNoAttackFlag (0x801fd2a0, 16 bytes)
#[inline(never)]
pub fn f_801fd2a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd2a0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd2a0); p = true; }
    }
    f_801fd2a0_b(c, 0x801fd2a0);
    if p { crate::prof::exit(c, 0x801fd2a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd2a0_at(c: &mut Cpu, pc: u32) { f_801fd2a0_b(c, pc) }
#[inline(always)]
fn f_801fd2a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd2a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd2a0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd2a0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd2a8;
            }
            0x801fd2a8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x693), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd2a0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetChouhatsuNoAttackFlag (0x801fd2b0, 16 bytes)
#[inline(never)]
pub fn f_801fd2b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd2b0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd2b0); p = true; }
    }
    f_801fd2b0_b(c, 0x801fd2b0);
    if p { crate::prof::exit(c, 0x801fd2b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd2b0_at(c: &mut Cpu, pc: u32) { f_801fd2b0_b(c, pc) }
#[inline(always)]
fn f_801fd2b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd2b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd2b0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd2b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd2b8;
            }
            0x801fd2b8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x692), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd2b0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetKanashibariNoAttackFlag (0x801fd2c0, 16 bytes)
#[inline(never)]
pub fn f_801fd2c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd2c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd2c0); p = true; }
    }
    f_801fd2c0_b(c, 0x801fd2c0);
    if p { crate::prof::exit(c, 0x801fd2c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd2c0_at(c: &mut Cpu, pc: u32) { f_801fd2c0_b(c, pc) }
#[inline(always)]
fn f_801fd2c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd2c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd2c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd2c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd2c8;
            }
            0x801fd2c8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x691), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd2c0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetMeroMeroNoAttackFlag (0x801fd2d0, 16 bytes)
#[inline(never)]
pub fn f_801fd2d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd2d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd2d0); p = true; }
    }
    f_801fd2d0_b(c, 0x801fd2d0);
    if p { crate::prof::exit(c, 0x801fd2d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd2d0_at(c: &mut Cpu, pc: u32) { f_801fd2d0_b(c, pc) }
#[inline(always)]
fn f_801fd2d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd2d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd2d0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd2d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd2d8;
            }
            0x801fd2d8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x690), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd2d0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetHuuinNoAttackFlag (0x801fd2e0, 16 bytes)
#[inline(never)]
pub fn f_801fd2e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd2e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd2e0); p = true; }
    }
    f_801fd2e0_b(c, 0x801fd2e0);
    if p { crate::prof::exit(c, 0x801fd2e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd2e0_at(c: &mut Cpu, pc: u32) { f_801fd2e0_b(c, pc) }
#[inline(always)]
fn f_801fd2e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd2e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd2e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd2e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd2e8;
            }
            0x801fd2e8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x68f), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd2e0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetItemNigeruFlag (0x801fd2f0, 16 bytes)
#[inline(never)]
pub fn f_801fd2f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd2f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd2f0); p = true; }
    }
    f_801fd2f0_b(c, 0x801fd2f0);
    if p { crate::prof::exit(c, 0x801fd2f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd2f0_at(c: &mut Cpu, pc: u32) { f_801fd2f0_b(c, pc) }
#[inline(always)]
fn f_801fd2f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd2f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd2f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd2f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd2f8;
            }
            0x801fd2f8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x68e), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd2f0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetTameWazaFlag (0x801fd300, 16 bytes)
#[inline(never)]
pub fn f_801fd300(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd300) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd300); p = true; }
    }
    f_801fd300_b(c, 0x801fd300);
    if p { crate::prof::exit(c, 0x801fd300); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd300_at(c: &mut Cpu, pc: u32) { f_801fd300_b(c, pc) }
#[inline(always)]
fn f_801fd300_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd300;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd300, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd300 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd308;
            }
            0x801fd308 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x68d), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd300: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetOutWazaKoukanaiFlag (0x801fd310, 16 bytes)
#[inline(never)]
pub fn f_801fd310(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd310) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd310); p = true; }
    }
    f_801fd310_b(c, 0x801fd310);
    if p { crate::prof::exit(c, 0x801fd310); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd310_at(c: &mut Cpu, pc: u32) { f_801fd310_b(c, pc) }
#[inline(always)]
fn f_801fd310_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd310;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd310, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd310 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd318;
            }
            0x801fd318 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x68c), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd310: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetKonranMyselfAttackFlag (0x801fd320, 16 bytes)
#[inline(never)]
pub fn f_801fd320(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd320) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd320); p = true; }
    }
    f_801fd320_b(c, 0x801fd320);
    if p { crate::prof::exit(c, 0x801fd320); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd320_at(c: &mut Cpu, pc: u32) { f_801fd320_b(c, pc) }
#[inline(always)]
fn f_801fd320_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd320;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd320, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd320 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd328;
            }
            0x801fd328 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x68b), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd320: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetMahiNoAttackFlag (0x801fd330, 16 bytes)
#[inline(never)]
pub fn f_801fd330(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd330) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd330); p = true; }
    }
    f_801fd330_b(c, 0x801fd330);
    if p { crate::prof::exit(c, 0x801fd330); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd330_at(c: &mut Cpu, pc: u32) { f_801fd330_b(c, pc) }
#[inline(always)]
fn f_801fd330_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd330;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd330, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd330 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd338;
            }
            0x801fd338 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x68a), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd330: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetMyselfDamageSpeTargetId (0x801fd340, 24 bytes)
#[inline(never)]
pub fn f_801fd340(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd340) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd340); p = true; }
    }
    f_801fd340_b(c, 0x801fd340);
    if p { crate::prof::exit(c, 0x801fd340); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd340_at(c: &mut Cpu, pc: u32) { f_801fd340_b(c, pc) }
#[inline(always)]
fn f_801fd340_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd340;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd340, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd340 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd350; continue; }
                pc = 0x801fd348;
            }
            0x801fd348 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd350;
            }
            0x801fd350 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6aa));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd340: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetMyselfDamageSpeValue (0x801fd358, 24 bytes)
#[inline(never)]
pub fn f_801fd358(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd358) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd358); p = true; }
    }
    f_801fd358_b(c, 0x801fd358);
    if p { crate::prof::exit(c, 0x801fd358); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd358_at(c: &mut Cpu, pc: u32) { f_801fd358_b(c, pc) }
#[inline(always)]
fn f_801fd358_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd358;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd358, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd358 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd368; continue; }
                pc = 0x801fd360;
            }
            0x801fd360 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd368;
            }
            0x801fd368 => { ins += 2;
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(0x6a8));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd358: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetMyselfDamageAtkTargetId (0x801fd370, 24 bytes)
#[inline(never)]
pub fn f_801fd370(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd370) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd370); p = true; }
    }
    f_801fd370_b(c, 0x801fd370);
    if p { crate::prof::exit(c, 0x801fd370); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd370_at(c: &mut Cpu, pc: u32) { f_801fd370_b(c, pc) }
#[inline(always)]
fn f_801fd370_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd370;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd370, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd370 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd380; continue; }
                pc = 0x801fd378;
            }
            0x801fd378 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd380;
            }
            0x801fd380 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6a6));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd370: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetMyselfDamageAtkValue (0x801fd388, 24 bytes)
#[inline(never)]
pub fn f_801fd388(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd388) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd388); p = true; }
    }
    f_801fd388_b(c, 0x801fd388);
    if p { crate::prof::exit(c, 0x801fd388); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd388_at(c: &mut Cpu, pc: u32) { f_801fd388_b(c, pc) }
#[inline(always)]
fn f_801fd388_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd388;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd388, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd388 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd398; continue; }
                pc = 0x801fd390;
            }
            0x801fd390 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd398;
            }
            0x801fd398 => { ins += 2;
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(0x6a4));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd388: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetKaigaraDamageValue (0x801fd3a0, 24 bytes)
#[inline(never)]
pub fn f_801fd3a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd3a0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd3a0); p = true; }
    }
    f_801fd3a0_b(c, 0x801fd3a0);
    if p { crate::prof::exit(c, 0x801fd3a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd3a0_at(c: &mut Cpu, pc: u32) { f_801fd3a0_b(c, pc) }
#[inline(always)]
fn f_801fd3a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd3a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd3a0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd3a0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd3b0; continue; }
                pc = 0x801fd3a8;
            }
            0x801fd3a8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd3b0;
            }
            0x801fd3b0 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x6a0));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd3a0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetItemKoraetaFlag (0x801fd3b8, 24 bytes)
#[inline(never)]
pub fn f_801fd3b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd3b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd3b8); p = true; }
    }
    f_801fd3b8_b(c, 0x801fd3b8);
    if p { crate::prof::exit(c, 0x801fd3b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd3b8_at(c: &mut Cpu, pc: u32) { f_801fd3b8_b(c, pc) }
#[inline(always)]
fn f_801fd3b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd3b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd3b8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd3b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd3c8; continue; }
                pc = 0x801fd3c0;
            }
            0x801fd3c0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd3c8;
            }
            0x801fd3c8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x69e));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd3b8: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetIrekaetaFlag (0x801fd3d0, 24 bytes)
#[inline(never)]
pub fn f_801fd3d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd3d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd3d0); p = true; }
    }
    f_801fd3d0_b(c, 0x801fd3d0);
    if p { crate::prof::exit(c, 0x801fd3d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd3d0_at(c: &mut Cpu, pc: u32) { f_801fd3d0_b(c, pc) }
#[inline(always)]
fn f_801fd3d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd3d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd3d0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd3d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd3e0; continue; }
                pc = 0x801fd3d8;
            }
            0x801fd3d8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd3e0;
            }
            0x801fd3e0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x69d));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd3d0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetNoPressureFlag (0x801fd3e8, 24 bytes)
#[inline(never)]
pub fn f_801fd3e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd3e8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd3e8); p = true; }
    }
    f_801fd3e8_b(c, 0x801fd3e8);
    if p { crate::prof::exit(c, 0x801fd3e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd3e8_at(c: &mut Cpu, pc: u32) { f_801fd3e8_b(c, pc) }
#[inline(always)]
fn f_801fd3e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd3e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd3e8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd3e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd3f8; continue; }
                pc = 0x801fd3f0;
            }
            0x801fd3f0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd3f8;
            }
            0x801fd3f8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x69c));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd3e8: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetDoTraceFlag (0x801fd400, 24 bytes)
#[inline(never)]
pub fn f_801fd400(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd400) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd400); p = true; }
    }
    f_801fd400_b(c, 0x801fd400);
    if p { crate::prof::exit(c, 0x801fd400); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd400_at(c: &mut Cpu, pc: u32) { f_801fd400_b(c, pc) }
#[inline(always)]
fn f_801fd400_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd400;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd400, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd400 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd410; continue; }
                pc = 0x801fd408;
            }
            0x801fd408 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd410;
            }
            0x801fd410 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x69b));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd400: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetDoIkakuFlag (0x801fd418, 24 bytes)
#[inline(never)]
pub fn f_801fd418(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd418) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd418); p = true; }
    }
    f_801fd418_b(c, 0x801fd418);
    if p { crate::prof::exit(c, 0x801fd418); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd418_at(c: &mut Cpu, pc: u32) { f_801fd418_b(c, pc) }
#[inline(always)]
fn f_801fd418_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd418;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd418, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd418 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd428; continue; }
                pc = 0x801fd420;
            }
            0x801fd420 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd428;
            }
            0x801fd428 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x69a));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd418: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetVanishoffFlag (0x801fd430, 24 bytes)
#[inline(never)]
pub fn f_801fd430(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd430) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd430); p = true; }
    }
    f_801fd430_b(c, 0x801fd430);
    if p { crate::prof::exit(c, 0x801fd430); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd430_at(c: &mut Cpu, pc: u32) { f_801fd430_b(c, pc) }
#[inline(always)]
fn f_801fd430_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd430;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd430, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd430 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd440; continue; }
                pc = 0x801fd438;
            }
            0x801fd438 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd440;
            }
            0x801fd440 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x699));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd430: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetReceivesWazaHiraishinFlag (0x801fd448, 24 bytes)
#[inline(never)]
pub fn f_801fd448(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd448) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd448); p = true; }
    }
    f_801fd448_b(c, 0x801fd448);
    if p { crate::prof::exit(c, 0x801fd448); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd448_at(c: &mut Cpu, pc: u32) { f_801fd448_b(c, pc) }
#[inline(always)]
fn f_801fd448_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd448;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd448, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd448 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd458; continue; }
                pc = 0x801fd450;
            }
            0x801fd450 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd458;
            }
            0x801fd458 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x698));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd448: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetDoClearbodyFlag (0x801fd460, 24 bytes)
#[inline(never)]
pub fn f_801fd460(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd460) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd460); p = true; }
    }
    f_801fd460_b(c, 0x801fd460);
    if p { crate::prof::exit(c, 0x801fd460); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd460_at(c: &mut Cpu, pc: u32) { f_801fd460_b(c, pc) }
#[inline(always)]
fn f_801fd460_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd460;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd460, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd460 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd470; continue; }
                pc = 0x801fd468;
            }
            0x801fd468 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd470;
            }
            0x801fd470 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x697));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd460: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetFightActionFlag (0x801fd478, 24 bytes)
#[inline(never)]
pub fn f_801fd478(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd478) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd478); p = true; }
    }
    f_801fd478_b(c, 0x801fd478);
    if p { crate::prof::exit(c, 0x801fd478); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd478_at(c: &mut Cpu, pc: u32) { f_801fd478_b(c, pc) }
#[inline(always)]
fn f_801fd478_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd478;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd478, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd478 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd488; continue; }
                pc = 0x801fd480;
            }
            0x801fd480 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd488;
            }
            0x801fd488 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x696));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd478: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetPassPpdecFlag (0x801fd490, 24 bytes)
#[inline(never)]
pub fn f_801fd490(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd490) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd490); p = true; }
    }
    f_801fd490_b(c, 0x801fd490);
    if p { crate::prof::exit(c, 0x801fd490); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd490_at(c: &mut Cpu, pc: u32) { f_801fd490_b(c, pc) }
#[inline(always)]
fn f_801fd490_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd490;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd490, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd490 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd4a0; continue; }
                pc = 0x801fd498;
            }
            0x801fd498 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd4a0;
            }
            0x801fd4a0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x695));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd490: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetHirumuNoAttackFlag (0x801fd4a8, 24 bytes)
#[inline(never)]
pub fn f_801fd4a8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd4a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd4a8); p = true; }
    }
    f_801fd4a8_b(c, 0x801fd4a8);
    if p { crate::prof::exit(c, 0x801fd4a8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd4a8_at(c: &mut Cpu, pc: u32) { f_801fd4a8_b(c, pc) }
#[inline(always)]
fn f_801fd4a8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd4a8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd4a8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd4a8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd4b8; continue; }
                pc = 0x801fd4b0;
            }
            0x801fd4b0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd4b8;
            }
            0x801fd4b8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x694));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd4a8: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetIchamonNoAttackFlag (0x801fd4c0, 24 bytes)
#[inline(never)]
pub fn f_801fd4c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd4c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd4c0); p = true; }
    }
    f_801fd4c0_b(c, 0x801fd4c0);
    if p { crate::prof::exit(c, 0x801fd4c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd4c0_at(c: &mut Cpu, pc: u32) { f_801fd4c0_b(c, pc) }
#[inline(always)]
fn f_801fd4c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd4c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd4c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd4c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd4d0; continue; }
                pc = 0x801fd4c8;
            }
            0x801fd4c8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd4d0;
            }
            0x801fd4d0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x693));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd4c0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetChouhatsuNoAttackFlag (0x801fd4d8, 24 bytes)
#[inline(never)]
pub fn f_801fd4d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd4d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd4d8); p = true; }
    }
    f_801fd4d8_b(c, 0x801fd4d8);
    if p { crate::prof::exit(c, 0x801fd4d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd4d8_at(c: &mut Cpu, pc: u32) { f_801fd4d8_b(c, pc) }
#[inline(always)]
fn f_801fd4d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd4d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd4d8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd4d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd4e8; continue; }
                pc = 0x801fd4e0;
            }
            0x801fd4e0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd4e8;
            }
            0x801fd4e8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x692));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd4d8: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetKanashibariNoAttackFlag (0x801fd4f0, 24 bytes)
#[inline(never)]
pub fn f_801fd4f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd4f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd4f0); p = true; }
    }
    f_801fd4f0_b(c, 0x801fd4f0);
    if p { crate::prof::exit(c, 0x801fd4f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd4f0_at(c: &mut Cpu, pc: u32) { f_801fd4f0_b(c, pc) }
#[inline(always)]
fn f_801fd4f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd4f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd4f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd4f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd500; continue; }
                pc = 0x801fd4f8;
            }
            0x801fd4f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd500;
            }
            0x801fd500 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x691));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd4f0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetMeroMeroNoAttackFlag (0x801fd508, 24 bytes)
#[inline(never)]
pub fn f_801fd508(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd508) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd508); p = true; }
    }
    f_801fd508_b(c, 0x801fd508);
    if p { crate::prof::exit(c, 0x801fd508); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd508_at(c: &mut Cpu, pc: u32) { f_801fd508_b(c, pc) }
#[inline(always)]
fn f_801fd508_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd508;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd508, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd508 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd518; continue; }
                pc = 0x801fd510;
            }
            0x801fd510 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd518;
            }
            0x801fd518 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x690));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd508: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetHuuinNoAttackFlag (0x801fd520, 24 bytes)
#[inline(never)]
pub fn f_801fd520(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd520) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd520); p = true; }
    }
    f_801fd520_b(c, 0x801fd520);
    if p { crate::prof::exit(c, 0x801fd520); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd520_at(c: &mut Cpu, pc: u32) { f_801fd520_b(c, pc) }
#[inline(always)]
fn f_801fd520_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd520;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd520, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd520 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd530; continue; }
                pc = 0x801fd528;
            }
            0x801fd528 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd530;
            }
            0x801fd530 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x68f));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd520: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetItemNigeruFlag (0x801fd538, 24 bytes)
#[inline(never)]
pub fn f_801fd538(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd538) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd538); p = true; }
    }
    f_801fd538_b(c, 0x801fd538);
    if p { crate::prof::exit(c, 0x801fd538); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd538_at(c: &mut Cpu, pc: u32) { f_801fd538_b(c, pc) }
#[inline(always)]
fn f_801fd538_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd538;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd538, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd538 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd548; continue; }
                pc = 0x801fd540;
            }
            0x801fd540 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd548;
            }
            0x801fd548 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x68e));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd538: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetTameWazaFlag (0x801fd550, 24 bytes)
#[inline(never)]
pub fn f_801fd550(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd550) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd550); p = true; }
    }
    f_801fd550_b(c, 0x801fd550);
    if p { crate::prof::exit(c, 0x801fd550); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd550_at(c: &mut Cpu, pc: u32) { f_801fd550_b(c, pc) }
#[inline(always)]
fn f_801fd550_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd550;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd550, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd550 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd560; continue; }
                pc = 0x801fd558;
            }
            0x801fd558 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd560;
            }
            0x801fd560 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x68d));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd550: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetOutWazaKoukanaiFlag (0x801fd568, 24 bytes)
#[inline(never)]
pub fn f_801fd568(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd568) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd568); p = true; }
    }
    f_801fd568_b(c, 0x801fd568);
    if p { crate::prof::exit(c, 0x801fd568); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd568_at(c: &mut Cpu, pc: u32) { f_801fd568_b(c, pc) }
#[inline(always)]
fn f_801fd568_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd568;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd568, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd568 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd578; continue; }
                pc = 0x801fd570;
            }
            0x801fd570 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd578;
            }
            0x801fd578 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x68c));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd568: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetKonranMyselfAttackFlag (0x801fd580, 24 bytes)
#[inline(never)]
pub fn f_801fd580(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd580) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd580); p = true; }
    }
    f_801fd580_b(c, 0x801fd580);
    if p { crate::prof::exit(c, 0x801fd580); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd580_at(c: &mut Cpu, pc: u32) { f_801fd580_b(c, pc) }
#[inline(always)]
fn f_801fd580_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd580;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd580, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd580 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd590; continue; }
                pc = 0x801fd588;
            }
            0x801fd588 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd590;
            }
            0x801fd590 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x68b));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd580: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetMahiNoAttackFlag (0x801fd598, 24 bytes)
#[inline(never)]
pub fn f_801fd598(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd598) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd598); p = true; }
    }
    f_801fd598_b(c, 0x801fd598);
    if p { crate::prof::exit(c, 0x801fd598); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd598_at(c: &mut Cpu, pc: u32) { f_801fd598_b(c, pc) }
#[inline(always)]
fn f_801fd598_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd598;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd598, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd598 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd5a8; continue; }
                pc = 0x801fd5a0;
            }
            0x801fd5a0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd5a8;
            }
            0x801fd5a8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x68a));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd598: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetWazaMenuCurPtr (0x801fd5b0, 24 bytes)
#[inline(never)]
pub fn f_801fd5b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd5b0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd5b0); p = true; }
    }
    f_801fd5b0_b(c, 0x801fd5b0);
    if p { crate::prof::exit(c, 0x801fd5b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd5b0_at(c: &mut Cpu, pc: u32) { f_801fd5b0_b(c, pc) }
#[inline(always)]
fn f_801fd5b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd5b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd5b0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd5b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd5c0; continue; }
                pc = 0x801fd5b8;
            }
            0x801fd5b8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd5c0;
            }
            0x801fd5c0 => { ins += 2;
                c.r[3] = c.r[3].wrapping_add(0x664);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd5b0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetTokuseiDataId (0x801fd5c8, 16 bytes)
#[inline(never)]
pub fn f_801fd5c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd5c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd5c8); p = true; }
    }
    f_801fd5c8_b(c, 0x801fd5c8);
    if p { crate::prof::exit(c, 0x801fd5c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd5c8_at(c: &mut Cpu, pc: u32) { f_801fd5c8_b(c, pc) }
#[inline(always)]
fn f_801fd5c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd5c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd5c8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd5c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd5d0;
            }
            0x801fd5d0 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x660), c.r[4]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd5c8: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetTokuseiDataId (0x801fd5d8, 24 bytes)
#[inline(never)]
pub fn f_801fd5d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd5d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd5d8); p = true; }
    }
    f_801fd5d8_b(c, 0x801fd5d8);
    if p { crate::prof::exit(c, 0x801fd5d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd5d8_at(c: &mut Cpu, pc: u32) { f_801fd5d8_b(c, pc) }
#[inline(always)]
fn f_801fd5d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd5d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd5d8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd5d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd5e8; continue; }
                pc = 0x801fd5e0;
            }
            0x801fd5e0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd5e8;
            }
            0x801fd5e8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x660));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd5d8: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosSetZokuseiDataId (0x801fd5f0, 36 bytes)
#[inline(never)]
pub fn f_801fd5f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd5f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd5f0); p = true; }
    }
    f_801fd5f0_b(c, 0x801fd5f0);
    if p { crate::prof::exit(c, 0x801fd5f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd5f0_at(c: &mut Cpu, pc: u32) { f_801fd5f0_b(c, pc) }
#[inline(always)]
fn f_801fd5f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd5f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd5f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd5f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd5f8;
            }
            0x801fd5f8 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd604;
            }
            0x801fd604 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.sth(c.r[3].wrapping_add(0x65c), c.r[5]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd5f0: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetZokuseiDataId (0x801fd614, 52 bytes)
#[inline(never)]
pub fn f_801fd614(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd614) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd614); p = true; }
    }
    f_801fd614_b(c, 0x801fd614);
    if p { crate::prof::exit(c, 0x801fd614); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd614_at(c: &mut Cpu, pc: u32) { f_801fd614_b(c, pc) }
#[inline(always)]
fn f_801fd614_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd614;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd614, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd614 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd624; continue; }
                pc = 0x801fd61c;
            }
            0x801fd61c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd624;
            }
            0x801fd624 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x801fd638; continue; }
                pc = 0x801fd630;
            }
            0x801fd630 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd638;
            }
            0x801fd638 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x65c));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd614: bad pc {:#x}", pc),
        }
    }
}

/// fightOutPokemonBiosGetFightActionBuffPtr (0x801fd648, 24 bytes)
#[inline(never)]
pub fn f_801fd648(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x801fd648) { return; }
        if c.prof { crate::prof::enter(c, 0x801fd648); p = true; }
    }
    f_801fd648_b(c, 0x801fd648);
    if p { crate::prof::exit(c, 0x801fd648); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801fd648_at(c: &mut Cpu, pc: u32) { f_801fd648_b(c, pc) }
#[inline(always)]
fn f_801fd648_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x801fd648;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x801fd648, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x801fd648 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x801fd658; continue; }
                pc = 0x801fd650;
            }
            0x801fd650 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x801fd658;
            }
            0x801fd658 => { ins += 2;
                c.r[3] = c.r[3].wrapping_add(0x62c);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_801fd648: bad pc {:#x}", pc),
        }
    }
}

