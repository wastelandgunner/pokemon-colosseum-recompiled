#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fightWazaWriteJoutaiDataId (0x802097c8, 84 bytes)
#[inline(never)]
pub fn f_802097c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x802097c8) { return; }
        if c.prof { crate::prof::enter(c, 0x802097c8); p = true; }
    }
    f_802097c8_b(c, 0x802097c8);
    if p { crate::prof::exit(c, 0x802097c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802097c8_at(c: &mut Cpu, pc: u32) { f_802097c8_b(c, pc) }
#[inline(always)]
fn f_802097c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802097c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802097c8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802097c8 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802097ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802097ec;
            }
            0x802097ec => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2a);
                if !cr_bit(cr, 2) { pc = 0x80209808; continue; }
                pc = 0x802097f8;
            }
            0x802097f8 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80209808; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b2c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209808;
            }
            0x80209808 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802097c8: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaCheckWriteJoutaiDataId (0x8020981c, 84 bytes)
#[inline(never)]
pub fn f_8020981c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020981c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020981c); p = true; }
    }
    f_8020981c_b(c, 0x8020981c);
    if p { crate::prof::exit(c, 0x8020981c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020981c_at(c: &mut Cpu, pc: u32) { f_8020981c_b(c, pc) }
#[inline(always)]
fn f_8020981c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020981c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020981c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020981c => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020983c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020983c;
            }
            0x8020983c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2a);
                if cr_bit(cr, 2) { pc = 0x80209850; continue; }
                pc = 0x80209848;
            }
            0x80209848 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020985c; continue;
                pc = 0x80209850;
            }
            0x80209850 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8020985c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b444(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020985c;
            }
            0x8020985c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020981c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaIsJoutaiSousai (0x80209870, 156 bytes)
#[inline(never)]
pub fn f_80209870(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80209870) { return; }
        if c.prof { crate::prof::enter(c, 0x80209870); p = true; }
    }
    f_80209870_b(c, 0x80209870);
    if p { crate::prof::exit(c, 0x80209870); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209870_at(c: &mut Cpu, pc: u32) { f_80209870_b(c, pc) }
#[inline(always)]
fn f_80209870_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80209870;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80209870, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80209870 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x41;
                c.lr = 0x8020988c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020988c;
            }
            0x8020988c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2a);
                if cr_bit(cr, 2) { pc = 0x802098a0; continue; }
                pc = 0x80209898;
            }
            0x80209898 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802098ac; continue;
                pc = 0x802098a0;
            }
            0x802098a0 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x41;
                c.lr = 0x802098ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802098ac;
            }
            0x802098ac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802098f4; continue; }
                pc = 0x802098b8;
            }
            0x802098b8 => { ins += 2;
                c.r[3] = 0x42;
                c.lr = 0x802098c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802098c0;
            }
            0x802098c0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2a);
                if cr_bit(cr, 2) { pc = 0x802098d4; continue; }
                pc = 0x802098cc;
            }
            0x802098cc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x802098e0; continue;
                pc = 0x802098d4;
            }
            0x802098d4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x42;
                c.lr = 0x802098e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802098e0;
            }
            0x802098e0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x802098f4; continue; }
                pc = 0x802098ec;
            }
            0x802098ec => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x802098f8; continue;
                pc = 0x802098f4;
            }
            0x802098f4 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x802098f8;
            }
            0x802098f8 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80209870: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaIsJoutaiDataId (0x8020990c, 84 bytes)
#[inline(never)]
pub fn f_8020990c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020990c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020990c); p = true; }
    }
    f_8020990c_b(c, 0x8020990c);
    if p { crate::prof::exit(c, 0x8020990c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020990c_at(c: &mut Cpu, pc: u32) { f_8020990c_b(c, pc) }
#[inline(always)]
fn f_8020990c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020990c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020990c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020990c => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020992c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020992c;
            }
            0x8020992c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2a);
                if cr_bit(cr, 2) { pc = 0x80209940; continue; }
                pc = 0x80209938;
            }
            0x80209938 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020994c; continue;
                pc = 0x80209940;
            }
            0x80209940 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8020994c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b67c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020994c;
            }
            0x8020994c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020990c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaInitJoutaiDataId (0x80209960, 76 bytes)
#[inline(never)]
pub fn f_80209960(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80209960) { return; }
        if c.prof { crate::prof::enter(c, 0x80209960); p = true; }
    }
    f_80209960_b(c, 0x80209960);
    if p { crate::prof::exit(c, 0x80209960); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209960_at(c: &mut Cpu, pc: u32) { f_80209960_b(c, pc) }
#[inline(always)]
fn f_80209960_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80209960;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80209960, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80209960 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80209980; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209980;
            }
            0x80209980 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2a);
                if !cr_bit(cr, 2) { pc = 0x80209998; continue; }
                pc = 0x8020998c;
            }
            0x8020998c => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80209998; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b788(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209998;
            }
            0x80209998 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80209960: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaCreate (0x802099ac, 624 bytes)
#[inline(never)]
pub fn f_802099ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x802099ac) { return; }
        if c.prof { crate::prof::enter(c, 0x802099ac); p = true; }
    }
    f_802099ac_b(c, 0x802099ac);
    if p { crate::prof::exit(c, 0x802099ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802099ac_at(c: &mut Cpu, pc: u32) { f_802099ac_b(c, pc) }
#[inline(never)]
fn f_802099ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x802099ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x802099ac, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x802099ac => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[27]);
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                c.r[30] = c.r[7] | c.r[7];
                if cr_bit(cr, 2) { pc = 0x80209c08; continue; }
                pc = 0x802099d4;
            }
            0x802099d4 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x80209b34; continue; }
                pc = 0x802099d8;
            }
            0x802099d8 => { ins += 5;
                c.r[4] = 0x0;
                c.r[5] = 0x26;
                c.r[6] = 0x0;
                c.r[7] = 0xffffffff;
                c.lr = 0x802099ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x802099ec;
            }
            0x802099ec => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209a04; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209a04;
            }
            0x80209a04 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209a1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209a1c;
            }
            0x80209a1c => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209a34; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209a34;
            }
            0x80209a34 => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2a;
                c.r[6] = 0x0;
                c.lr = 0x80209a48; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209a48;
            }
            0x80209a48 => { ins += 2;
                c.r[4] = 0x9;
                c.lr = 0x80209a50; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b950(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209a50;
            }
            0x80209a50 => { ins += 2;
                c.r[3] = 0x3f;
                c.lr = 0x80209a58; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209a58;
            }
            0x80209a58 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2a);
                if !cr_bit(cr, 2) { pc = 0x80209a74; continue; }
                pc = 0x80209a64;
            }
            0x80209a64 => { ins += 4;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x3f;
                c.r[5] = 0x0;
                c.lr = 0x80209a74; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b2c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209a74;
            }
            0x80209a74 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2b;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209a8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209a8c;
            }
            0x80209a8c => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209aa4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209aa4;
            }
            0x80209aa4 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209abc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209abc;
            }
            0x80209abc => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2e;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209ad4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209ad4;
            }
            0x80209ad4 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209aec; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209aec;
            }
            0x80209aec => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.r[7] = 0x9;
                c.lr = 0x80209b04; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209b04;
            }
            0x80209b04 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x31;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209b1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209b1c;
            }
            0x80209b1c => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x32;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209b34; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209b34;
            }
            0x80209b34 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[28] as u8 as i8 as i32 as u32;
                c.r[4] = 0x0;
                c.r[5] = 0x26;
                c.r[6] = 0x0;
                c.lr = 0x80209b4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209b4c;
            }
            0x80209b4c => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.lr = 0x80209b64; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209b64;
            }
            0x80209b64 => { ins += 7;
                c.r[31] = c.r[29].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x80209b80; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209b80;
            }
            0x80209b80 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.lr = 0x80209b98; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209b98;
            }
            0x80209b98 => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x80209bac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209bac;
            }
            0x80209bac => { ins += 6;
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80209bc4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209bc4;
            }
            0x80209bc4 => { ins += 5;
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x80209bd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209bd8;
            }
            0x80209bd8 => { ins += 6;
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.lr = 0x80209bf0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209bf0;
            }
            0x80209bf0 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[30].rotate_left(0) & 0xff;
                c.r[4] = 0x0;
                c.r[5] = 0x32;
                c.r[6] = 0x0;
                c.lr = 0x80209c08; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209c08;
            }
            0x80209c08 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_802099ac: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaSetUseWazaStatus (0x80209c1c, 152 bytes)
#[inline(never)]
pub fn f_80209c1c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80209c1c) { return; }
        if c.prof { crate::prof::enter(c, 0x80209c1c); p = true; }
    }
    f_80209c1c_b(c, 0x80209c1c);
    if p { crate::prof::exit(c, 0x80209c1c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209c1c_at(c: &mut Cpu, pc: u32) { f_80209c1c_b(c, pc) }
#[inline(always)]
fn f_80209c1c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80209c1c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80209c1c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80209c1c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = c.r[4].rotate_left(0) & 0xffff;
                c.r[5] = 0x28;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.lr = 0x80209c48; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209c48;
            }
            0x80209c48 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x80209c5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209c5c;
            }
            0x80209c5c => { ins += 6;
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80209c74; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209c74;
            }
            0x80209c74 => { ins += 5;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x80209c88; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209c88;
            }
            0x80209c88 => { ins += 6;
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.lr = 0x80209ca0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209ca0;
            }
            0x80209ca0 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80209c1c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaCheckValid (0x80209cb4, 220 bytes)
#[inline(never)]
pub fn f_80209cb4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80209cb4) { return; }
        if c.prof { crate::prof::enter(c, 0x80209cb4); p = true; }
    }
    f_80209cb4_b(c, 0x80209cb4);
    if p { crate::prof::exit(c, 0x80209cb4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209cb4_at(c: &mut Cpu, pc: u32) { f_80209cb4_b(c, pc) }
#[inline(always)]
fn f_80209cb4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80209cb4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80209cb4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80209cb4 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x80209cd4; continue; }
                pc = 0x80209ccc;
            }
            0x80209ccc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80209d7c; continue;
                pc = 0x80209cd4;
            }
            0x80209cd4 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x80209ce4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209ce4;
            }
            0x80209ce4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80209cf4; continue; }
                pc = 0x80209cec;
            }
            0x80209cec => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80209d7c; continue;
                pc = 0x80209cf4;
            }
            0x80209cf4 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x80209d08; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209d08;
            }
            0x80209d08 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x163);
                if !cr_bit(cr, 2) { pc = 0x80209d18; continue; }
                pc = 0x80209d10;
            }
            0x80209d10 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80209d7c; continue;
                pc = 0x80209d18;
            }
            0x80209d18 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.lr = 0x80209d2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209d2c;
            }
            0x80209d2c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x80209d3c; continue; }
                pc = 0x80209d34;
            }
            0x80209d34 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80209d7c; continue;
                pc = 0x80209d3c;
            }
            0x80209d3c => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.lr = 0x80209d50; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209d50;
            }
            0x80209d50 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x163);
                if !cr_bit(cr, 2) { pc = 0x80209d60; continue; }
                pc = 0x80209d58;
            }
            0x80209d58 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x80209d7c; continue;
                pc = 0x80209d60;
            }
            0x80209d60 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.lr = 0x80209d74; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209d74;
            }
            0x80209d74 => { ins += 2;
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                pc = 0x80209d7c;
            }
            0x80209d7c => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80209cb4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaInit (0x80209d90, 392 bytes)
#[inline(never)]
pub fn f_80209d90(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80209d90) { return; }
        if c.prof { crate::prof::enter(c, 0x80209d90); p = true; }
    }
    f_80209d90_b(c, 0x80209d90);
    if p { crate::prof::exit(c, 0x80209d90); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209d90_at(c: &mut Cpu, pc: u32) { f_80209d90_b(c, pc) }
#[inline(always)]
fn f_80209d90_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80209d90;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80209d90, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80209d90 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if cr_bit(cr, 2) { pc = 0x80209f04; continue; }
                pc = 0x80209da8;
            }
            0x80209da8 => { ins += 5;
                c.r[4] = 0x0;
                c.r[5] = 0x26;
                c.r[6] = 0x0;
                c.r[7] = 0xffffffff;
                c.lr = 0x80209dbc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209dbc;
            }
            0x80209dbc => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209dd4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209dd4;
            }
            0x80209dd4 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209dec; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209dec;
            }
            0x80209dec => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209e04; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209e04;
            }
            0x80209e04 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2a;
                c.r[6] = 0x0;
                c.lr = 0x80209e18; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209e18;
            }
            0x80209e18 => { ins += 2;
                c.r[4] = 0x9;
                c.lr = 0x80209e20; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b950(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209e20;
            }
            0x80209e20 => { ins += 2;
                c.r[3] = 0x3f;
                c.lr = 0x80209e28; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209e28;
            }
            0x80209e28 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2a);
                if !cr_bit(cr, 2) { pc = 0x80209e44; continue; }
                pc = 0x80209e34;
            }
            0x80209e34 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3f;
                c.r[5] = 0x0;
                c.lr = 0x80209e44; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b2c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209e44;
            }
            0x80209e44 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2b;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209e5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209e5c;
            }
            0x80209e5c => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209e74; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209e74;
            }
            0x80209e74 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209e8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209e8c;
            }
            0x80209e8c => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2e;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209ea4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209ea4;
            }
            0x80209ea4 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209ebc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209ebc;
            }
            0x80209ebc => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.r[7] = 0x9;
                c.lr = 0x80209ed4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209ed4;
            }
            0x80209ed4 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x31;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209eec; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209eec;
            }
            0x80209eec => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x32;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209f04; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209f04;
            }
            0x80209f04 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80209d90: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaInitLoop (0x80209f18, 148 bytes)
#[inline(never)]
pub fn f_80209f18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80209f18) { return; }
        if c.prof { crate::prof::enter(c, 0x80209f18); p = true; }
    }
    f_80209f18_b(c, 0x80209f18);
    if p { crate::prof::exit(c, 0x80209f18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209f18_at(c: &mut Cpu, pc: u32) { f_80209f18_b(c, pc) }
#[inline(always)]
fn f_80209f18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80209f18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80209f18, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80209f18 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x2a;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80209f3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209f3c;
            }
            0x80209f3c => { ins += 2;
                c.r[4] = 0x9;
                c.lr = 0x80209f44; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b950(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209f44;
            }
            0x80209f44 => { ins += 2;
                c.r[3] = 0x3f;
                c.lr = 0x80209f4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209f4c;
            }
            0x80209f4c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2a);
                if !cr_bit(cr, 2) { pc = 0x80209f68; continue; }
                pc = 0x80209f58;
            }
            0x80209f58 => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3f;
                c.r[5] = 0x0;
                c.lr = 0x80209f68; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b2c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209f68;
            }
            0x80209f68 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2b;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209f80; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209f80;
            }
            0x80209f80 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209f98; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209f98;
            }
            0x80209f98 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80209f18: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaInitJoutai (0x80209fac, 100 bytes)
#[inline(never)]
pub fn f_80209fac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x80209fac) { return; }
        if c.prof { crate::prof::enter(c, 0x80209fac); p = true; }
    }
    f_80209fac_b(c, 0x80209fac);
    if p { crate::prof::exit(c, 0x80209fac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209fac_at(c: &mut Cpu, pc: u32) { f_80209fac_b(c, pc) }
#[inline(always)]
fn f_80209fac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x80209fac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x80209fac, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x80209fac => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x2a;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80209fd0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209fd0;
            }
            0x80209fd0 => { ins += 2;
                c.r[4] = 0x9;
                c.lr = 0x80209fd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b950(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209fd8;
            }
            0x80209fd8 => { ins += 2;
                c.r[3] = 0x3f;
                c.lr = 0x80209fe0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80119ed0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209fe0;
            }
            0x80209fe0 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2a);
                if !cr_bit(cr, 2) { pc = 0x80209ffc; continue; }
                pc = 0x80209fec;
            }
            0x80209fec => { ins += 4;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3f;
                c.r[5] = 0x0;
                c.lr = 0x80209ffc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011b2c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x80209ffc;
            }
            0x80209ffc => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_80209fac: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaHitKakurituDataBiosGetWaru (0x8020a010, 24 bytes)
#[inline(never)]
pub fn f_8020a010(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a010) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a010); p = true; }
    }
    f_8020a010_b(c, 0x8020a010);
    if p { crate::prof::exit(c, 0x8020a010); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a010_at(c: &mut Cpu, pc: u32) { f_8020a010_b(c, pc) }
#[inline(always)]
fn f_8020a010_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a010;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a010, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a010 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a020; continue; }
                pc = 0x8020a018;
            }
            0x8020a018 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a020;
            }
            0x8020a020 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a010: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaHitKakurituDataBiosGetKake (0x8020a028, 24 bytes)
#[inline(never)]
pub fn f_8020a028(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a028) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a028); p = true; }
    }
    f_8020a028_b(c, 0x8020a028);
    if p { crate::prof::exit(c, 0x8020a028); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a028_at(c: &mut Cpu, pc: u32) { f_8020a028_b(c, pc) }
#[inline(always)]
fn f_8020a028_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a028;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a028, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a028 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a038; continue; }
                pc = 0x8020a030;
            }
            0x8020a030 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a038;
            }
            0x8020a038 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a028: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaHitKakurituDataBiosGetPtr (0x8020a040, 40 bytes)
#[inline(never)]
pub fn f_8020a040(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a040) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a040); p = true; }
    }
    f_8020a040_b(c, 0x8020a040);
    if p { crate::prof::exit(c, 0x8020a040); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a040_at(c: &mut Cpu, pc: u32) { f_8020a040_b(c, pc) }
#[inline(always)]
fn f_8020a040_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a040;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a040, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a040 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8550));
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = 0x80370000;
                c.r[3] = c.r[3].rotate_left(1) & 0x1fffe;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], c.r[0]);
                c.r[0] = c.r[4].wrapping_add(0x5dd0);
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                if cr_bit(cr, 0) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a060;
            }
            0x8020a060 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a040: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaCriticalDataBiosGetBunbo (0x8020a068, 24 bytes)
#[inline(never)]
pub fn f_8020a068(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a068) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a068); p = true; }
    }
    f_8020a068_b(c, 0x8020a068);
    if p { crate::prof::exit(c, 0x8020a068); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a068_at(c: &mut Cpu, pc: u32) { f_8020a068_b(c, pc) }
#[inline(always)]
fn f_8020a068_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a068;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a068, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a068 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a078; continue; }
                pc = 0x8020a070;
            }
            0x8020a070 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a078;
            }
            0x8020a078 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a068: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaCriticalDataBiosGetPtr (0x8020a080, 36 bytes)
#[inline(never)]
pub fn f_8020a080(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a080) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a080); p = true; }
    }
    f_8020a080_b(c, 0x8020a080);
    if p { crate::prof::exit(c, 0x8020a080); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a080_at(c: &mut Cpu, pc: u32) { f_8020a080_b(c, pc) }
#[inline(always)]
fn f_8020a080_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a080;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a080, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a080 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8540));
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x80480000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0xffff8d58);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if cr_bit(cr, 0) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a09c;
            }
            0x8020a09c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a080: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetAutoMakeFlag (0x8020a0a4, 16 bytes)
#[inline(never)]
pub fn f_8020a0a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a0a4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0a4); p = true; }
    }
    f_8020a0a4_b(c, 0x8020a0a4);
    if p { crate::prof::exit(c, 0x8020a0a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0a4_at(c: &mut Cpu, pc: u32) { f_8020a0a4_b(c, pc) }
#[inline(always)]
fn f_8020a0a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a0a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a0a4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a0a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a0ac;
            }
            0x8020a0ac => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xa9), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a0a4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetKaisuu (0x8020a0b4, 16 bytes)
#[inline(never)]
pub fn f_8020a0b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a0b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0b4); p = true; }
    }
    f_8020a0b4_b(c, 0x8020a0b4);
    if p { crate::prof::exit(c, 0x8020a0b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0b4_at(c: &mut Cpu, pc: u32) { f_8020a0b4_b(c, pc) }
#[inline(always)]
fn f_8020a0b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a0b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a0b4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a0b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a0bc;
            }
            0x8020a0bc => { ins += 2;
                c.stb(c.r[3].wrapping_add(0xa8), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a0b4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetZokusei (0x8020a0c4, 16 bytes)
#[inline(never)]
pub fn f_8020a0c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a0c4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0c4); p = true; }
    }
    f_8020a0c4_b(c, 0x8020a0c4);
    if p { crate::prof::exit(c, 0x8020a0c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0c4_at(c: &mut Cpu, pc: u32) { f_8020a0c4_b(c, pc) }
#[inline(always)]
fn f_8020a0c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a0c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a0c4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a0c4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a0cc;
            }
            0x8020a0cc => { ins += 2;
                c.sth(c.r[3].wrapping_add(0xa6), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a0c4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetIryoku (0x8020a0d4, 16 bytes)
#[inline(never)]
pub fn f_8020a0d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a0d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0d4); p = true; }
    }
    f_8020a0d4_b(c, 0x8020a0d4);
    if p { crate::prof::exit(c, 0x8020a0d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0d4_at(c: &mut Cpu, pc: u32) { f_8020a0d4_b(c, pc) }
#[inline(always)]
fn f_8020a0d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a0d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a0d4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a0d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a0dc;
            }
            0x8020a0dc => { ins += 2;
                c.sth(c.r[3].wrapping_add(0xa4), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a0d4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetHitDamage (0x8020a0e4, 16 bytes)
#[inline(never)]
pub fn f_8020a0e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a0e4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0e4); p = true; }
    }
    f_8020a0e4_b(c, 0x8020a0e4);
    if p { crate::prof::exit(c, 0x8020a0e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0e4_at(c: &mut Cpu, pc: u32) { f_8020a0e4_b(c, pc) }
#[inline(always)]
fn f_8020a0e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a0e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a0e4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a0e4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a0ec;
            }
            0x8020a0ec => { ins += 2;
                c.stw(c.r[3].wrapping_add(0xa0), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a0e4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetDamage (0x8020a0f4, 16 bytes)
#[inline(never)]
pub fn f_8020a0f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a0f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0f4); p = true; }
    }
    f_8020a0f4_b(c, 0x8020a0f4);
    if p { crate::prof::exit(c, 0x8020a0f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0f4_at(c: &mut Cpu, pc: u32) { f_8020a0f4_b(c, pc) }
#[inline(always)]
fn f_8020a0f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a0f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a0f4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a0f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a0fc;
            }
            0x8020a0fc => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x9c), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a0f4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetDamageValue (0x8020a104, 16 bytes)
#[inline(never)]
pub fn f_8020a104(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a104) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a104); p = true; }
    }
    f_8020a104_b(c, 0x8020a104);
    if p { crate::prof::exit(c, 0x8020a104); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a104_at(c: &mut Cpu, pc: u32) { f_8020a104_b(c, pc) }
#[inline(always)]
fn f_8020a104_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a104;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a104, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a104 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a10c;
            }
            0x8020a10c => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x99), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a104: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetCritical (0x8020a114, 16 bytes)
#[inline(never)]
pub fn f_8020a114(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a114) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a114); p = true; }
    }
    f_8020a114_b(c, 0x8020a114);
    if p { crate::prof::exit(c, 0x8020a114); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a114_at(c: &mut Cpu, pc: u32) { f_8020a114_b(c, pc) }
#[inline(always)]
fn f_8020a114_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a114;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a114, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a114 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a11c;
            }
            0x8020a11c => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x98), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a114: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetTargetDataId (0x8020a124, 16 bytes)
#[inline(never)]
pub fn f_8020a124(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a124) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a124); p = true; }
    }
    f_8020a124_b(c, 0x8020a124);
    if p { crate::prof::exit(c, 0x8020a124); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a124_at(c: &mut Cpu, pc: u32) { f_8020a124_b(c, pc) }
#[inline(always)]
fn f_8020a124_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a124;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a124, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a124 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a12c;
            }
            0x8020a12c => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a124: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetUseWazaDataId (0x8020a134, 16 bytes)
#[inline(never)]
pub fn f_8020a134(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a134) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a134); p = true; }
    }
    f_8020a134_b(c, 0x8020a134);
    if p { crate::prof::exit(c, 0x8020a134); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a134_at(c: &mut Cpu, pc: u32) { f_8020a134_b(c, pc) }
#[inline(always)]
fn f_8020a134_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a134;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a134, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a134 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a13c;
            }
            0x8020a13c => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a134: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetMotoWazaDataId (0x8020a144, 16 bytes)
#[inline(never)]
pub fn f_8020a144(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a144) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a144); p = true; }
    }
    f_8020a144_b(c, 0x8020a144);
    if p { crate::prof::exit(c, 0x8020a144); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a144_at(c: &mut Cpu, pc: u32) { f_8020a144_b(c, pc) }
#[inline(always)]
fn f_8020a144_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a144;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a144, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a144 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a14c;
            }
            0x8020a14c => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a144: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetWazaBanme (0x8020a154, 16 bytes)
#[inline(never)]
pub fn f_8020a154(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a154) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a154); p = true; }
    }
    f_8020a154_b(c, 0x8020a154);
    if p { crate::prof::exit(c, 0x8020a154); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a154_at(c: &mut Cpu, pc: u32) { f_8020a154_b(c, pc) }
#[inline(always)]
fn f_8020a154_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a154;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a154, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a154 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a15c;
            }
            0x8020a15c => { ins += 2;
                c.stb(c.r[3], c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a154: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetAutoMakeFlag (0x8020a164, 24 bytes)
#[inline(never)]
pub fn f_8020a164(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a164) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a164); p = true; }
    }
    f_8020a164_b(c, 0x8020a164);
    if p { crate::prof::exit(c, 0x8020a164); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a164_at(c: &mut Cpu, pc: u32) { f_8020a164_b(c, pc) }
#[inline(always)]
fn f_8020a164_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a164;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a164, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a164 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a174; continue; }
                pc = 0x8020a16c;
            }
            0x8020a16c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a174;
            }
            0x8020a174 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xa9));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a164: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetKaisuu (0x8020a17c, 24 bytes)
#[inline(never)]
pub fn f_8020a17c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a17c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a17c); p = true; }
    }
    f_8020a17c_b(c, 0x8020a17c);
    if p { crate::prof::exit(c, 0x8020a17c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a17c_at(c: &mut Cpu, pc: u32) { f_8020a17c_b(c, pc) }
#[inline(always)]
fn f_8020a17c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a17c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a17c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a17c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a18c; continue; }
                pc = 0x8020a184;
            }
            0x8020a184 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a18c;
            }
            0x8020a18c => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xa8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a17c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetZokusei (0x8020a194, 24 bytes)
#[inline(never)]
pub fn f_8020a194(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a194) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a194); p = true; }
    }
    f_8020a194_b(c, 0x8020a194);
    if p { crate::prof::exit(c, 0x8020a194); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a194_at(c: &mut Cpu, pc: u32) { f_8020a194_b(c, pc) }
#[inline(always)]
fn f_8020a194_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a194;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a194, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a194 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a1a4; continue; }
                pc = 0x8020a19c;
            }
            0x8020a19c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a1a4;
            }
            0x8020a1a4 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xa6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a194: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetIryoku (0x8020a1ac, 24 bytes)
#[inline(never)]
pub fn f_8020a1ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a1ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a1ac); p = true; }
    }
    f_8020a1ac_b(c, 0x8020a1ac);
    if p { crate::prof::exit(c, 0x8020a1ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a1ac_at(c: &mut Cpu, pc: u32) { f_8020a1ac_b(c, pc) }
#[inline(always)]
fn f_8020a1ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a1ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a1ac, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a1ac => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a1bc; continue; }
                pc = 0x8020a1b4;
            }
            0x8020a1b4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a1bc;
            }
            0x8020a1bc => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0xa4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a1ac: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetHitDamage (0x8020a1c4, 24 bytes)
#[inline(never)]
pub fn f_8020a1c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a1c4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a1c4); p = true; }
    }
    f_8020a1c4_b(c, 0x8020a1c4);
    if p { crate::prof::exit(c, 0x8020a1c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a1c4_at(c: &mut Cpu, pc: u32) { f_8020a1c4_b(c, pc) }
#[inline(always)]
fn f_8020a1c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a1c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a1c4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a1c4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a1d4; continue; }
                pc = 0x8020a1cc;
            }
            0x8020a1cc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a1d4;
            }
            0x8020a1d4 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xa0));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a1c4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetDamage (0x8020a1dc, 24 bytes)
#[inline(never)]
pub fn f_8020a1dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a1dc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a1dc); p = true; }
    }
    f_8020a1dc_b(c, 0x8020a1dc);
    if p { crate::prof::exit(c, 0x8020a1dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a1dc_at(c: &mut Cpu, pc: u32) { f_8020a1dc_b(c, pc) }
#[inline(always)]
fn f_8020a1dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a1dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a1dc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a1dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a1ec; continue; }
                pc = 0x8020a1e4;
            }
            0x8020a1e4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a1ec;
            }
            0x8020a1ec => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x9c));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a1dc: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetDamageValue (0x8020a1f4, 24 bytes)
#[inline(never)]
pub fn f_8020a1f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a1f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a1f4); p = true; }
    }
    f_8020a1f4_b(c, 0x8020a1f4);
    if p { crate::prof::exit(c, 0x8020a1f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a1f4_at(c: &mut Cpu, pc: u32) { f_8020a1f4_b(c, pc) }
#[inline(always)]
fn f_8020a1f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a1f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a1f4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a1f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a204; continue; }
                pc = 0x8020a1fc;
            }
            0x8020a1fc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a204;
            }
            0x8020a204 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x99));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a1f4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetCritical (0x8020a20c, 24 bytes)
#[inline(never)]
pub fn f_8020a20c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a20c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a20c); p = true; }
    }
    f_8020a20c_b(c, 0x8020a20c);
    if p { crate::prof::exit(c, 0x8020a20c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a20c_at(c: &mut Cpu, pc: u32) { f_8020a20c_b(c, pc) }
#[inline(always)]
fn f_8020a20c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a20c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a20c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a20c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a21c; continue; }
                pc = 0x8020a214;
            }
            0x8020a214 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a21c;
            }
            0x8020a21c => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x98));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a20c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetJoutaiPtr (0x8020a224, 52 bytes)
#[inline(never)]
pub fn f_8020a224(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a224) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a224); p = true; }
    }
    f_8020a224_b(c, 0x8020a224);
    if p { crate::prof::exit(c, 0x8020a224); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a224_at(c: &mut Cpu, pc: u32) { f_8020a224_b(c, pc) }
#[inline(always)]
fn f_8020a224_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a224;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a224, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a224 => { ins += 2;
                c.r[5] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x8020a234; continue; }
                pc = 0x8020a22c;
            }
            0x8020a22c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a234;
            }
            0x8020a234 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x9);
                if cr_bit(cr, 0) { pc = 0x8020a248; continue; }
                pc = 0x8020a240;
            }
            0x8020a240 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a248;
            }
            0x8020a248 => { ins += 4;
                c.r[3] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(0x8);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a224: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetTargetDataId (0x8020a258, 24 bytes)
#[inline(never)]
pub fn f_8020a258(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a258) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a258); p = true; }
    }
    f_8020a258_b(c, 0x8020a258);
    if p { crate::prof::exit(c, 0x8020a258); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a258_at(c: &mut Cpu, pc: u32) { f_8020a258_b(c, pc) }
#[inline(always)]
fn f_8020a258_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a258;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a258, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a258 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a268; continue; }
                pc = 0x8020a260;
            }
            0x8020a260 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a268;
            }
            0x8020a268 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a258: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetUseWazaDataId (0x8020a270, 24 bytes)
#[inline(never)]
pub fn f_8020a270(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a270) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a270); p = true; }
    }
    f_8020a270_b(c, 0x8020a270);
    if p { crate::prof::exit(c, 0x8020a270); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a270_at(c: &mut Cpu, pc: u32) { f_8020a270_b(c, pc) }
#[inline(always)]
fn f_8020a270_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a270;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a270, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a270 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a280; continue; }
                pc = 0x8020a278;
            }
            0x8020a278 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a280;
            }
            0x8020a280 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a270: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetMotoWazaDataId (0x8020a288, 24 bytes)
#[inline(never)]
pub fn f_8020a288(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a288) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a288); p = true; }
    }
    f_8020a288_b(c, 0x8020a288);
    if p { crate::prof::exit(c, 0x8020a288); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a288_at(c: &mut Cpu, pc: u32) { f_8020a288_b(c, pc) }
#[inline(always)]
fn f_8020a288_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a288;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a288, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a288 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a298; continue; }
                pc = 0x8020a290;
            }
            0x8020a290 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a298;
            }
            0x8020a298 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a288: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetWazaBanme (0x8020a2a0, 24 bytes)
#[inline(never)]
pub fn f_8020a2a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a2a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a2a0); p = true; }
    }
    f_8020a2a0_b(c, 0x8020a2a0);
    if p { crate::prof::exit(c, 0x8020a2a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a2a0_at(c: &mut Cpu, pc: u32) { f_8020a2a0_b(c, pc) }
#[inline(always)]
fn f_8020a2a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a2a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a2a0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a2a0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a2b0; continue; }
                pc = 0x8020a2a8;
            }
            0x8020a2a8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a2b0;
            }
            0x8020a2b0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a2a0: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosCopy (0x8020a2b8, 64 bytes)
#[inline(never)]
pub fn f_8020a2b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a2b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a2b8); p = true; }
    }
    f_8020a2b8_b(c, 0x8020a2b8);
    if p { crate::prof::exit(c, 0x8020a2b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a2b8_at(c: &mut Cpu, pc: u32) { f_8020a2b8_b(c, pc) }
#[inline(always)]
fn f_8020a2b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a2b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a2b8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a2b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a2c0;
            }
            0x8020a2c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a2c8;
            }
            0x8020a2c8 => { ins += 5;
                c.r[0] = 0x15;
                c.r[5] = c.r[3].wrapping_add(0xfffffffc);
                c.r[4] = c.r[4].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x8020a2d8;
            }
            0x8020a2d8 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8020a2d8; continue; }
                pc = 0x8020a2ec;
            }
            0x8020a2ec => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a2b8: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosSetBuff (0x8020a2f8, 16 bytes)
#[inline(never)]
pub fn f_8020a2f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a2f8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a2f8); p = true; }
    }
    f_8020a2f8_b(c, 0x8020a2f8);
    if p { crate::prof::exit(c, 0x8020a2f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a2f8_at(c: &mut Cpu, pc: u32) { f_8020a2f8_b(c, pc) }
#[inline(always)]
fn f_8020a2f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a2f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a2f8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a2f8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a300;
            }
            0x8020a300 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a2f8: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosSetCount (0x8020a308, 16 bytes)
#[inline(never)]
pub fn f_8020a308(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a308) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a308); p = true; }
    }
    f_8020a308_b(c, 0x8020a308);
    if p { crate::prof::exit(c, 0x8020a308); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a308_at(c: &mut Cpu, pc: u32) { f_8020a308_b(c, pc) }
#[inline(always)]
fn f_8020a308_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a308;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a308, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a308 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a310;
            }
            0x8020a310 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a308: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosSetTargetDataId (0x8020a318, 16 bytes)
#[inline(never)]
pub fn f_8020a318(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a318) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a318); p = true; }
    }
    f_8020a318_b(c, 0x8020a318);
    if p { crate::prof::exit(c, 0x8020a318); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a318_at(c: &mut Cpu, pc: u32) { f_8020a318_b(c, pc) }
#[inline(always)]
fn f_8020a318_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a318;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a318, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a318 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a320;
            }
            0x8020a320 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a318: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosSetItemDataId (0x8020a328, 16 bytes)
#[inline(never)]
pub fn f_8020a328(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a328) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a328); p = true; }
    }
    f_8020a328_b(c, 0x8020a328);
    if p { crate::prof::exit(c, 0x8020a328); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a328_at(c: &mut Cpu, pc: u32) { f_8020a328_b(c, pc) }
#[inline(always)]
fn f_8020a328_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a328;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a328, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a328 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a330;
            }
            0x8020a330 => { ins += 2;
                c.sth(c.r[3], c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a328: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosGetBuff (0x8020a338, 24 bytes)
#[inline(never)]
pub fn f_8020a338(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a338) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a338); p = true; }
    }
    f_8020a338_b(c, 0x8020a338);
    if p { crate::prof::exit(c, 0x8020a338); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a338_at(c: &mut Cpu, pc: u32) { f_8020a338_b(c, pc) }
#[inline(always)]
fn f_8020a338_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a338;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a338, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a338 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a348; continue; }
                pc = 0x8020a340;
            }
            0x8020a340 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a348;
            }
            0x8020a348 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a338: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosGetCount (0x8020a350, 24 bytes)
#[inline(never)]
pub fn f_8020a350(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a350) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a350); p = true; }
    }
    f_8020a350_b(c, 0x8020a350);
    if p { crate::prof::exit(c, 0x8020a350); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a350_at(c: &mut Cpu, pc: u32) { f_8020a350_b(c, pc) }
#[inline(always)]
fn f_8020a350_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a350;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a350, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a350 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a360; continue; }
                pc = 0x8020a358;
            }
            0x8020a358 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a360;
            }
            0x8020a360 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a350: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosGetTargetDataId (0x8020a368, 24 bytes)
#[inline(never)]
pub fn f_8020a368(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a368) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a368); p = true; }
    }
    f_8020a368_b(c, 0x8020a368);
    if p { crate::prof::exit(c, 0x8020a368); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a368_at(c: &mut Cpu, pc: u32) { f_8020a368_b(c, pc) }
#[inline(always)]
fn f_8020a368_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a368;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a368, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a368 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a378; continue; }
                pc = 0x8020a370;
            }
            0x8020a370 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a378;
            }
            0x8020a378 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a368: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosGetItemDataId (0x8020a380, 24 bytes)
#[inline(never)]
pub fn f_8020a380(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a380) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a380); p = true; }
    }
    f_8020a380_b(c, 0x8020a380);
    if p { crate::prof::exit(c, 0x8020a380); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a380_at(c: &mut Cpu, pc: u32) { f_8020a380_b(c, pc) }
#[inline(always)]
fn f_8020a380_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a380;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a380, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a380 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a390; continue; }
                pc = 0x8020a388;
            }
            0x8020a388 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a390;
            }
            0x8020a390 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a380: bad pc {:#x}", pc),
        }
    }
}

/// fightItemCreate (0x8020a398, 224 bytes)
#[inline(never)]
pub fn f_8020a398(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a398) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a398); p = true; }
    }
    f_8020a398_b(c, 0x8020a398);
    if p { crate::prof::exit(c, 0x8020a398); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a398_at(c: &mut Cpu, pc: u32) { f_8020a398_b(c, pc) }
#[inline(always)]
fn f_8020a398_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a398;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a398, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a398 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                if cr_bit(cr, 2) { pc = 0x8020a464; continue; }
                pc = 0x8020a3bc;
            }
            0x8020a3bc => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020a41c; continue; }
                pc = 0x8020a3c0;
            }
            0x8020a3c0 => { ins += 5;
                c.r[4] = 0x0;
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a3d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a3d4;
            }
            0x8020a3d4 => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a3ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a3ec;
            }
            0x8020a3ec => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x20;
                c.r[6] = 0x0;
                c.r[7] = 0xffffffff;
                c.lr = 0x8020a404; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a404;
            }
            0x8020a404 => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x21;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a41c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a41c;
            }
            0x8020a41c => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                c.r[7] = c.r[29].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.lr = 0x8020a434; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a434;
            }
            0x8020a434 => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                c.r[7] = c.r[30].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.lr = 0x8020a44c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a44c;
            }
            0x8020a44c => { ins += 6;
                c.r[3] = c.r[28] | c.r[28];
                c.r[7] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x20;
                c.r[6] = 0x0;
                c.lr = 0x8020a464; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a464;
            }
            0x8020a464 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a398: bad pc {:#x}", pc),
        }
    }
}

/// fightItemInit (0x8020a478, 136 bytes)
#[inline(never)]
pub fn f_8020a478(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a478) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a478); p = true; }
    }
    f_8020a478_b(c, 0x8020a478);
    if p { crate::prof::exit(c, 0x8020a478); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a478_at(c: &mut Cpu, pc: u32) { f_8020a478_b(c, pc) }
#[inline(always)]
fn f_8020a478_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a478;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a478, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a478 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if cr_bit(cr, 2) { pc = 0x8020a4ec; continue; }
                pc = 0x8020a490;
            }
            0x8020a490 => { ins += 5;
                c.r[4] = 0x0;
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a4a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a4a4;
            }
            0x8020a4a4 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a4bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a4bc;
            }
            0x8020a4bc => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x20;
                c.r[6] = 0x0;
                c.r[7] = 0xffffffff;
                c.lr = 0x8020a4d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a4d4;
            }
            0x8020a4d4 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x21;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a4ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a4ec;
            }
            0x8020a4ec => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a478: bad pc {:#x}", pc),
        }
    }
}

/// joukenDataGet4 (0x8020a500, 64 bytes)
#[inline(never)]
pub fn f_8020a500(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a500) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a500); p = true; }
    }
    f_8020a500_b(c, 0x8020a500);
    if p { crate::prof::exit(c, 0x8020a500); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a500_at(c: &mut Cpu, pc: u32) { f_8020a500_b(c, pc) }
#[inline(always)]
fn f_8020a500_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a500;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a500, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a500 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8518));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a518; continue; }
                pc = 0x8020a510;
            }
            0x8020a510 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a528; continue;
                pc = 0x8020a518;
            }
            0x8020a518 => { ins += 6;
                c.r[4] = c.r[3].wrapping_mul(0x6);
                c.r[3] = 0x80480000;
                c.r[0] = c.r[3].wrapping_add(0xffff8d30);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8020a528;
            }
            0x8020a528 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a538; continue; }
                pc = 0x8020a530;
            }
            0x8020a530 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a538;
            }
            0x8020a538 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a500: bad pc {:#x}", pc),
        }
    }
}

/// joukenDataGet2 (0x8020a540, 64 bytes)
#[inline(never)]
pub fn f_8020a540(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a540) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a540); p = true; }
    }
    f_8020a540_b(c, 0x8020a540);
    if p { crate::prof::exit(c, 0x8020a540); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a540_at(c: &mut Cpu, pc: u32) { f_8020a540_b(c, pc) }
#[inline(always)]
fn f_8020a540_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a540;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a540, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a540 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8518));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a558; continue; }
                pc = 0x8020a550;
            }
            0x8020a550 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a568; continue;
                pc = 0x8020a558;
            }
            0x8020a558 => { ins += 6;
                c.r[4] = c.r[3].wrapping_mul(0x6);
                c.r[3] = 0x80480000;
                c.r[0] = c.r[3].wrapping_add(0xffff8d30);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8020a568;
            }
            0x8020a568 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a578; continue; }
                pc = 0x8020a570;
            }
            0x8020a570 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a578;
            }
            0x8020a578 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a540: bad pc {:#x}", pc),
        }
    }
}

/// joukenDataGet0 (0x8020a580, 64 bytes)
#[inline(never)]
pub fn f_8020a580(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a580) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a580); p = true; }
    }
    f_8020a580_b(c, 0x8020a580);
    if p { crate::prof::exit(c, 0x8020a580); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a580_at(c: &mut Cpu, pc: u32) { f_8020a580_b(c, pc) }
#[inline(always)]
fn f_8020a580_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a580;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a580, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a580 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8518));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a598; continue; }
                pc = 0x8020a590;
            }
            0x8020a590 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a5a8; continue;
                pc = 0x8020a598;
            }
            0x8020a598 => { ins += 6;
                c.r[4] = c.r[3].wrapping_mul(0x6);
                c.r[3] = 0x80480000;
                c.r[0] = c.r[3].wrapping_add(0xffff8d30);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8020a5a8;
            }
            0x8020a5a8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a5b8; continue; }
                pc = 0x8020a5b0;
            }
            0x8020a5b0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a5b8;
            }
            0x8020a5b8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a580: bad pc {:#x}", pc),
        }
    }
}

/// fightJoukenDataBiosGetTerm04 (0x8020a5c0, 112 bytes)
#[inline(never)]
pub fn f_8020a5c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a5c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a5c0); p = true; }
    }
    f_8020a5c0_b(c, 0x8020a5c0);
    if p { crate::prof::exit(c, 0x8020a5c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a5c0_at(c: &mut Cpu, pc: u32) { f_8020a5c0_b(c, pc) }
#[inline(always)]
fn f_8020a5c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a5c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a5c0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a5c0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a5d8; continue; }
                pc = 0x8020a5d0;
            }
            0x8020a5d0 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x8020a5e8; continue;
                pc = 0x8020a5d8;
            }
            0x8020a5d8 => { ins += 6;
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a5e8;
            }
            0x8020a5e8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a5f8; continue; }
                pc = 0x8020a5f0;
            }
            0x8020a5f0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a618; continue;
                pc = 0x8020a5f8;
            }
            0x8020a5f8 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8020a60c; continue; }
                pc = 0x8020a604;
            }
            0x8020a604 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a618; continue;
                pc = 0x8020a60c;
            }
            0x8020a60c => { ins += 5;
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a618;
            }
            0x8020a618 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a628; continue; }
                pc = 0x8020a620;
            }
            0x8020a620 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a628;
            }
            0x8020a628 => { ins += 2;
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a5c0: bad pc {:#x}", pc),
        }
    }
}

/// fightJoukenDataBiosGetTerm02 (0x8020a630, 112 bytes)
#[inline(never)]
pub fn f_8020a630(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a630) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a630); p = true; }
    }
    f_8020a630_b(c, 0x8020a630);
    if p { crate::prof::exit(c, 0x8020a630); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a630_at(c: &mut Cpu, pc: u32) { f_8020a630_b(c, pc) }
#[inline(always)]
fn f_8020a630_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a630;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a630, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a630 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a648; continue; }
                pc = 0x8020a640;
            }
            0x8020a640 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x8020a658; continue;
                pc = 0x8020a648;
            }
            0x8020a648 => { ins += 6;
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a658;
            }
            0x8020a658 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a668; continue; }
                pc = 0x8020a660;
            }
            0x8020a660 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a688; continue;
                pc = 0x8020a668;
            }
            0x8020a668 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8020a67c; continue; }
                pc = 0x8020a674;
            }
            0x8020a674 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a688; continue;
                pc = 0x8020a67c;
            }
            0x8020a67c => { ins += 5;
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a688;
            }
            0x8020a688 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a698; continue; }
                pc = 0x8020a690;
            }
            0x8020a690 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a698;
            }
            0x8020a698 => { ins += 2;
                c.r[3] = c.lha_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a630: bad pc {:#x}", pc),
        }
    }
}

/// fightJoukenDataBiosGetTerm01 (0x8020a6a0, 112 bytes)
#[inline(never)]
pub fn f_8020a6a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a6a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a6a0); p = true; }
    }
    f_8020a6a0_b(c, 0x8020a6a0);
    if p { crate::prof::exit(c, 0x8020a6a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a6a0_at(c: &mut Cpu, pc: u32) { f_8020a6a0_b(c, pc) }
#[inline(always)]
fn f_8020a6a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a6a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a6a0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a6a0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a6b8; continue; }
                pc = 0x8020a6b0;
            }
            0x8020a6b0 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x8020a6c8; continue;
                pc = 0x8020a6b8;
            }
            0x8020a6b8 => { ins += 6;
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a6c8;
            }
            0x8020a6c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a6d8; continue; }
                pc = 0x8020a6d0;
            }
            0x8020a6d0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a6f8; continue;
                pc = 0x8020a6d8;
            }
            0x8020a6d8 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8020a6ec; continue; }
                pc = 0x8020a6e4;
            }
            0x8020a6e4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a6f8; continue;
                pc = 0x8020a6ec;
            }
            0x8020a6ec => { ins += 5;
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a6f8;
            }
            0x8020a6f8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a708; continue; }
                pc = 0x8020a700;
            }
            0x8020a700 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a708;
            }
            0x8020a708 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a6a0: bad pc {:#x}", pc),
        }
    }
}

/// fightJoukenDataBiosGetTerm08 (0x8020a710, 112 bytes)
#[inline(never)]
pub fn f_8020a710(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a710) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a710); p = true; }
    }
    f_8020a710_b(c, 0x8020a710);
    if p { crate::prof::exit(c, 0x8020a710); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a710_at(c: &mut Cpu, pc: u32) { f_8020a710_b(c, pc) }
#[inline(always)]
fn f_8020a710_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a710;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a710, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a710 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a728; continue; }
                pc = 0x8020a720;
            }
            0x8020a720 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x8020a738; continue;
                pc = 0x8020a728;
            }
            0x8020a728 => { ins += 6;
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a738;
            }
            0x8020a738 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a748; continue; }
                pc = 0x8020a740;
            }
            0x8020a740 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a768; continue;
                pc = 0x8020a748;
            }
            0x8020a748 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8020a75c; continue; }
                pc = 0x8020a754;
            }
            0x8020a754 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a768; continue;
                pc = 0x8020a75c;
            }
            0x8020a75c => { ins += 5;
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a768;
            }
            0x8020a768 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a778; continue; }
                pc = 0x8020a770;
            }
            0x8020a770 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a778;
            }
            0x8020a778 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a710: bad pc {:#x}", pc),
        }
    }
}

/// fightJoukenDataBiosGetTerm06 (0x8020a780, 112 bytes)
#[inline(never)]
pub fn f_8020a780(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a780) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a780); p = true; }
    }
    f_8020a780_b(c, 0x8020a780);
    if p { crate::prof::exit(c, 0x8020a780); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a780_at(c: &mut Cpu, pc: u32) { f_8020a780_b(c, pc) }
#[inline(always)]
fn f_8020a780_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a780;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a780, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a780 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a798; continue; }
                pc = 0x8020a790;
            }
            0x8020a790 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x8020a7a8; continue;
                pc = 0x8020a798;
            }
            0x8020a798 => { ins += 6;
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a7a8;
            }
            0x8020a7a8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a7b8; continue; }
                pc = 0x8020a7b0;
            }
            0x8020a7b0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a7d8; continue;
                pc = 0x8020a7b8;
            }
            0x8020a7b8 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8020a7cc; continue; }
                pc = 0x8020a7c4;
            }
            0x8020a7c4 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a7d8; continue;
                pc = 0x8020a7cc;
            }
            0x8020a7cc => { ins += 5;
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a7d8;
            }
            0x8020a7d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a7e8; continue; }
                pc = 0x8020a7e0;
            }
            0x8020a7e0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a7e8;
            }
            0x8020a7e8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a780: bad pc {:#x}", pc),
        }
    }
}

/// fightJoukenDataBiosGetTermKind (0x8020a7f0, 112 bytes)
#[inline(never)]
pub fn f_8020a7f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a7f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a7f0); p = true; }
    }
    f_8020a7f0_b(c, 0x8020a7f0);
    if p { crate::prof::exit(c, 0x8020a7f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a7f0_at(c: &mut Cpu, pc: u32) { f_8020a7f0_b(c, pc) }
#[inline(always)]
fn f_8020a7f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a7f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a7f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a7f0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a808; continue; }
                pc = 0x8020a800;
            }
            0x8020a800 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x8020a818; continue;
                pc = 0x8020a808;
            }
            0x8020a808 => { ins += 6;
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a818;
            }
            0x8020a818 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a828; continue; }
                pc = 0x8020a820;
            }
            0x8020a820 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a848; continue;
                pc = 0x8020a828;
            }
            0x8020a828 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8020a83c; continue; }
                pc = 0x8020a834;
            }
            0x8020a834 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a848; continue;
                pc = 0x8020a83c;
            }
            0x8020a83c => { ins += 5;
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a848;
            }
            0x8020a848 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a858; continue; }
                pc = 0x8020a850;
            }
            0x8020a850 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a858;
            }
            0x8020a858 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a7f0: bad pc {:#x}", pc),
        }
    }
}

/// fightJoukenDataBiosGet02 (0x8020a860, 64 bytes)
#[inline(never)]
pub fn f_8020a860(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a860) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a860); p = true; }
    }
    f_8020a860_b(c, 0x8020a860);
    if p { crate::prof::exit(c, 0x8020a860); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a860_at(c: &mut Cpu, pc: u32) { f_8020a860_b(c, pc) }
#[inline(always)]
fn f_8020a860_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a860;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a860, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a860 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a878; continue; }
                pc = 0x8020a870;
            }
            0x8020a870 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a888; continue;
                pc = 0x8020a878;
            }
            0x8020a878 => { ins += 6;
                c.r[4] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8020a888;
            }
            0x8020a888 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a898; continue; }
                pc = 0x8020a890;
            }
            0x8020a890 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a898;
            }
            0x8020a898 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a860: bad pc {:#x}", pc),
        }
    }
}

/// fightJoukenDataBiosGetType (0x8020a8a0, 64 bytes)
#[inline(never)]
pub fn f_8020a8a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a8a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a8a0); p = true; }
    }
    f_8020a8a0_b(c, 0x8020a8a0);
    if p { crate::prof::exit(c, 0x8020a8a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a8a0_at(c: &mut Cpu, pc: u32) { f_8020a8a0_b(c, pc) }
#[inline(always)]
fn f_8020a8a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a8a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a8a0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a8a0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020a8b8; continue; }
                pc = 0x8020a8b0;
            }
            0x8020a8b0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020a8c8; continue;
                pc = 0x8020a8b8;
            }
            0x8020a8b8 => { ins += 6;
                c.r[4] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8020a8c8;
            }
            0x8020a8c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020a8d8; continue; }
                pc = 0x8020a8d0;
            }
            0x8020a8d0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020a8d8;
            }
            0x8020a8d8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a8a0: bad pc {:#x}", pc),
        }
    }
}

/// fightJoukenCheck (0x8020a8e0, 1360 bytes)
#[inline(never)]
pub fn f_8020a8e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020a8e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a8e0); p = true; }
    }
    f_8020a8e0_b(c, 0x8020a8e0);
    if p { crate::prof::exit(c, 0x8020a8e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a8e0_at(c: &mut Cpu, pc: u32) { f_8020a8e0_b(c, pc) }
#[inline(never)]
fn f_8020a8e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020a8e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020a8e0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020a8e0 => { ins += 1;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x8020a8e4;
            }
            0x8020a8e4 => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x8020a8e8;
            }
            0x8020a8e8 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                pc = 0x8020a8ec;
            }
            0x8020a8ec => { ins += 11;
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 19..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x8020a8f0;
            }
            0x8020a8f0 => { ins += 1;
                c.r[20] = c.r[3] | c.r[3];
                pc = 0x8020a8f4;
            }
            0x8020a8f4 => { ins += 1;
                c.r[31] = c.r[4] | c.r[4];
                pc = 0x8020a8f8;
            }
            0x8020a8f8 => { ins += 1;
                c.r[21] = c.r[1].wrapping_add(0x10);
                pc = 0x8020a8fc;
            }
            0x8020a8fc => { ins += 1;
                c.r[24] = 0x0;
                pc = 0x8020a900;
            }
            0x8020a900 => { ins += 1;
                pc = 0x8020aa64; continue;
                pc = 0x8020a904;
            }
            0x8020a904 => { ins += 1;
                c.r[29] = c.r[24].rotate_left(0) & 0xff;
                pc = 0x8020a908;
            }
            0x8020a908 => { ins += 1;
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a90c;
            }
            0x8020a90c => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a910;
            }
            0x8020a910 => { ins += 1;
                c.r[23] = 0x0;
                pc = 0x8020a914;
            }
            0x8020a914 => { ins += 1;
                c.lr = 0x8020a918; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a7f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a918;
            }
            0x8020a918 => { ins += 1;
                c.r[22] = c.r[3] | c.r[3];
                pc = 0x8020a91c;
            }
            0x8020a91c => { ins += 1;
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a920;
            }
            0x8020a920 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a924;
            }
            0x8020a924 => { ins += 1;
                c.lr = 0x8020a928; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a780(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a928;
            }
            0x8020a928 => { ins += 1;
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8020a92c;
            }
            0x8020a92c => { ins += 1;
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a930;
            }
            0x8020a930 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a934;
            }
            0x8020a934 => { ins += 1;
                c.lr = 0x8020a938; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a710(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a938;
            }
            0x8020a938 => { ins += 1;
                c.r[27] = c.r[3] | c.r[3];
                pc = 0x8020a93c;
            }
            0x8020a93c => { ins += 1;
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a940;
            }
            0x8020a940 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a944;
            }
            0x8020a944 => { ins += 1;
                c.lr = 0x8020a948; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a630(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a948;
            }
            0x8020a948 => { ins += 1;
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x8020a94c;
            }
            0x8020a94c => { ins += 1;
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a950;
            }
            0x8020a950 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a954;
            }
            0x8020a954 => { ins += 1;
                c.lr = 0x8020a958; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a5c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a958;
            }
            0x8020a958 => { ins += 1;
                c.r[25] = c.r[3] | c.r[3];
                pc = 0x8020a95c;
            }
            0x8020a95c => { ins += 1;
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a960;
            }
            0x8020a960 => { ins += 1;
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a964;
            }
            0x8020a964 => { ins += 1;
                c.lr = 0x8020a968; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a6a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a968;
            }
            0x8020a968 => { ins += 1;
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                pc = 0x8020a96c;
            }
            0x8020a96c => { ins += 1;
                c.r[22] = c.r[3] | c.r[3];
                pc = 0x8020a970;
            }
            0x8020a970 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                pc = 0x8020a974;
            }
            0x8020a974 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020a9a0; continue; }
                pc = 0x8020a978;
            }
            0x8020a978 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8020a98c; continue; }
                pc = 0x8020a97c;
            }
            0x8020a97c => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                pc = 0x8020a980;
            }
            0x8020a980 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020aa34; continue; }
                pc = 0x8020a984;
            }
            0x8020a984 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8020a998; continue; }
                pc = 0x8020a988;
            }
            0x8020a988 => { ins += 1;
                pc = 0x8020aa34; continue;
                pc = 0x8020a98c;
            }
            0x8020a98c => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x4);
                pc = 0x8020a990;
            }
            0x8020a990 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8020aa34; continue; }
                pc = 0x8020a994;
            }
            0x8020a994 => { ins += 1;
                pc = 0x8020a9b8; continue;
                pc = 0x8020a998;
            }
            0x8020a998 => { ins += 1;
                c.r[23] = c.r[28].rotate_left(0) & 0xffff;
                pc = 0x8020a99c;
            }
            0x8020a99c => { ins += 1;
                pc = 0x8020aa34; continue;
                pc = 0x8020a9a0;
            }
            0x8020a9a0 => { ins += 1;
                c.r[19] = c.r[28].rotate_left(0) & 0xffff;
                pc = 0x8020a9a4;
            }
            0x8020a9a4 => { ins += 1;
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                pc = 0x8020a9a8;
            }
            0x8020a9a8 => { ins += 1;
                c.r[3] = c.r[0].wrapping_sub(c.r[19]);
                pc = 0x8020a9ac;
            }
            0x8020a9ac => { ins += 1;
                c.lr = 0x8020a9b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_800e0c04(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a9b0;
            }
            0x8020a9b0 => { ins += 1;
                c.r[23] = c.r[19].wrapping_add(c.r[3]);
                pc = 0x8020a9b4;
            }
            0x8020a9b4 => { ins += 1;
                pc = 0x8020aa34; continue;
                pc = 0x8020a9b8;
            }
            0x8020a9b8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x8020a9bc;
            }
            0x8020a9bc => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x8020a9c0;
            }
            0x8020a9c0 => { ins += 1;
                c.r[5] = 0x14;
                pc = 0x8020a9c4;
            }
            0x8020a9c4 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x8020a9c8;
            }
            0x8020a9c8 => { ins += 1;
                c.lr = 0x8020a9cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a9cc;
            }
            0x8020a9cc => { ins += 1;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8020a9d0;
            }
            0x8020a9d0 => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x8020a9d4;
            }
            0x8020a9d4 => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8020a9d8;
            }
            0x8020a9d8 => { ins += 1;
                c.lr = 0x8020a9dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a9dc;
            }
            0x8020a9dc => { ins += 1;
                c.r[23] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[23]);
                pc = 0x8020a9e0;
            }
            0x8020a9e0 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020a9ec; continue; }
                pc = 0x8020a9e4;
            }
            0x8020a9e4 => { ins += 1;
                c.r[23] = 0x0;
                pc = 0x8020a9e8;
            }
            0x8020a9e8 => { ins += 1;
                pc = 0x8020aa58; continue;
                pc = 0x8020a9ec;
            }
            0x8020a9ec => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x8020a9f0;
            }
            0x8020a9f0 => { ins += 1;
                c.lr = 0x8020a9f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a9f4;
            }
            0x8020a9f4 => { ins += 1;
                c.lr = 0x8020a9f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f021c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020a9f8;
            }
            0x8020a9f8 => { ins += 1;
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x8020a9fc;
            }
            0x8020a9fc => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020aa1c; continue; }
                pc = 0x8020aa00;
            }
            0x8020aa00 => { ins += 1;
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x8020aa04;
            }
            0x8020aa04 => { ins += 1;
                c.r[6] = c.r[27] | c.r[27];
                pc = 0x8020aa08;
            }
            0x8020aa08 => { ins += 1;
                c.r[5] = c.r[26].rotate_left(0) & 0xffff;
                pc = 0x8020aa0c;
            }
            0x8020aa0c => { ins += 1;
                c.r[7] = c.r[25].rotate_left(0) & 0xffff;
                pc = 0x8020aa10;
            }
            0x8020aa10 => { ins += 1;
                c.lr = 0x8020aa14; { c.steps = st; c.cycles = ins; c.cr = cr; f_80135e44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020aa14;
            }
            0x8020aa14 => { ins += 1;
                c.r[23] = c.r[3] | c.r[3];
                pc = 0x8020aa18;
            }
            0x8020aa18 => { ins += 1;
                pc = 0x8020aa34; continue;
                pc = 0x8020aa1c;
            }
            0x8020aa1c => { ins += 1;
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x8020aa20;
            }
            0x8020aa20 => { ins += 1;
                c.r[6] = c.r[27] | c.r[27];
                pc = 0x8020aa24;
            }
            0x8020aa24 => { ins += 1;
                c.r[5] = 0x0;
                pc = 0x8020aa28;
            }
            0x8020aa28 => { ins += 1;
                c.r[7] = 0x0;
                pc = 0x8020aa2c;
            }
            0x8020aa2c => { ins += 1;
                c.lr = 0x8020aa30; { c.steps = st; c.cycles = ins; c.cr = cr; f_80135e44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020aa30;
            }
            0x8020aa30 => { ins += 1;
                c.r[23] = c.r[3] | c.r[3];
                pc = 0x8020aa34;
            }
            0x8020aa34 => { ins += 1;
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                pc = 0x8020aa38;
            }
            0x8020aa38 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                pc = 0x8020aa3c;
            }
            0x8020aa3c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020aa58; continue; }
                pc = 0x8020aa40;
            }
            0x8020aa40 => { ins += 1;
                c.r[3] = c.r[26] as u16 as i16 as i32 as u32;
                pc = 0x8020aa44;
            }
            0x8020aa44 => { ins += 1;
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x8020aa48;
            }
            0x8020aa48 => { ins += 5;
                c.r[23] = c.r[23].wrapping_mul(c.r[3]);
                pc = 0x8020aa4c;
            }
            0x8020aa4c => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020aa58; continue; }
                pc = 0x8020aa50;
            }
            0x8020aa50 => { ins += 1;
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                pc = 0x8020aa54;
            }
            0x8020aa54 => { ins += 40;
                c.r[23] = divw(c.r[23], c.r[0]);
                pc = 0x8020aa58;
            }
            0x8020aa58 => { ins += 1;
                c.r[0] = c.r[24].rotate_left(2) & 0x3fc;
                pc = 0x8020aa5c;
            }
            0x8020aa5c => { ins += 1;
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x8020aa60;
            }
            0x8020aa60 => { ins += 1;
                c.stw(c.r[21].wrapping_add(c.r[0]), c.r[23]);
                pc = 0x8020aa64;
            }
            0x8020aa64 => { ins += 1;
                c.r[0] = c.r[24].rotate_left(0) & 0xff;
                pc = 0x8020aa68;
            }
            0x8020aa68 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                pc = 0x8020aa6c;
            }
            0x8020aa6c => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x8020a904; continue; }
                pc = 0x8020aa70;
            }
            0x8020aa70 => { ins += 1;
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020aa74;
            }
            0x8020aa74 => { ins += 1;
                c.r[24] = 0x0;
                pc = 0x8020aa78;
            }
            0x8020aa78 => { ins += 1;
                c.lr = 0x8020aa7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a8a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020aa7c;
            }
            0x8020aa7c => { ins += 1;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8020aa80;
            }
            0x8020aa80 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                pc = 0x8020aa84;
            }
            0x8020aa84 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x8020ab34; continue; }
                pc = 0x8020aa88;
            }
            0x8020aa88 => { ins += 1;
                c.r[3] = 0x80370000;
                pc = 0x8020aa8c;
            }
            0x8020aa8c => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x8020aa90;
            }
            0x8020aa90 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x5954);
                pc = 0x8020aa94;
            }
            0x8020aa94 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[0]));
                pc = 0x8020aa98;
            }
            0x8020aa98 => { ins += 2;
                c.mtspr(9, c.r[0]);
                pc = 0x8020aa9c;
            }
            0x8020aa9c => { ins += 1;
                if true { let t = c.ctr; if (0x8020a8e0..0x8020ae30).contains(&t) { pc = t; continue; } { c.steps = st; c.cycles = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.cycles; cr = c.cr; }; { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020aaa0;
            }
            0x8020aaa0 => { ins += 1;
                c.r[24] = 0x1;
                pc = 0x8020aaa4;
            }
            0x8020aaa4 => { ins += 1;
                pc = 0x8020ab34; continue;
                pc = 0x8020aaa8;
            }
            0x8020aaa8 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                pc = 0x8020aaac;
            }
            0x8020aaac => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x8020aab0;
            }
            0x8020aab0 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020aab4;
            }
            0x8020aab4 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020ab34; continue; }
                pc = 0x8020aab8;
            }
            0x8020aab8 => { ins += 1;
                c.r[24] = 0x1;
                pc = 0x8020aabc;
            }
            0x8020aabc => { ins += 1;
                pc = 0x8020ab34; continue;
                pc = 0x8020aac0;
            }
            0x8020aac0 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                pc = 0x8020aac4;
            }
            0x8020aac4 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x8020aac8;
            }
            0x8020aac8 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020aacc;
            }
            0x8020aacc => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020ab34; continue; }
                pc = 0x8020aad0;
            }
            0x8020aad0 => { ins += 1;
                c.r[24] = 0x1;
                pc = 0x8020aad4;
            }
            0x8020aad4 => { ins += 1;
                pc = 0x8020ab34; continue;
                pc = 0x8020aad8;
            }
            0x8020aad8 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                pc = 0x8020aadc;
            }
            0x8020aadc => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x8020aae0;
            }
            0x8020aae0 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020aae4;
            }
            0x8020aae4 => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x8020ab34; continue; }
                pc = 0x8020aae8;
            }
            0x8020aae8 => { ins += 1;
                c.r[24] = 0x1;
                pc = 0x8020aaec;
            }
            0x8020aaec => { ins += 1;
                pc = 0x8020ab34; continue;
                pc = 0x8020aaf0;
            }
            0x8020aaf0 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                pc = 0x8020aaf4;
            }
            0x8020aaf4 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x8020aaf8;
            }
            0x8020aaf8 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020aafc;
            }
            0x8020aafc => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x8020ab34; continue; }
                pc = 0x8020ab00;
            }
            0x8020ab00 => { ins += 1;
                c.r[24] = 0x1;
                pc = 0x8020ab04;
            }
            0x8020ab04 => { ins += 1;
                pc = 0x8020ab34; continue;
                pc = 0x8020ab08;
            }
            0x8020ab08 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                pc = 0x8020ab0c;
            }
            0x8020ab0c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x8020ab10;
            }
            0x8020ab10 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020ab14;
            }
            0x8020ab14 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8020ab34; continue; }
                pc = 0x8020ab18;
            }
            0x8020ab18 => { ins += 1;
                c.r[24] = 0x1;
                pc = 0x8020ab1c;
            }
            0x8020ab1c => { ins += 1;
                pc = 0x8020ab34; continue;
                pc = 0x8020ab20;
            }
            0x8020ab20 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                pc = 0x8020ab24;
            }
            0x8020ab24 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x8020ab28;
            }
            0x8020ab28 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020ab2c;
            }
            0x8020ab2c => { ins += 1;
                if !cr_bit(cr, 1) { pc = 0x8020ab34; continue; }
                pc = 0x8020ab30;
            }
            0x8020ab30 => { ins += 1;
                c.r[24] = 0x1;
                pc = 0x8020ab34;
            }
            0x8020ab34 => { ins += 1;
                c.r[30] = c.r[24] | c.r[24];
                pc = 0x8020ab38;
            }
            0x8020ab38 => { ins += 1;
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020ab3c;
            }
            0x8020ab3c => { ins += 1;
                c.lr = 0x8020ab40; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a860(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ab40;
            }
            0x8020ab40 => { ins += 1;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x8020ab44;
            }
            0x8020ab44 => { ins += 1;
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8020ab48;
            }
            0x8020ab48 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020ab54; continue; }
                pc = 0x8020ab4c;
            }
            0x8020ab4c => { ins += 1;
                c.r[3] = c.r[24] | c.r[24];
                pc = 0x8020ab50;
            }
            0x8020ab50 => { ins += 1;
                pc = 0x8020ae1c; continue;
                pc = 0x8020ab54;
            }
            0x8020ab54 => { ins += 1;
                c.r[27] = c.r[1].wrapping_add(0x8);
                pc = 0x8020ab58;
            }
            0x8020ab58 => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x8020ab5c;
            }
            0x8020ab5c => { ins += 1;
                c.lr = 0x8020ab60; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a540(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ab60;
            }
            0x8020ab60 => { ins += 1;
                c.r[23] = 0x0;
                pc = 0x8020ab64;
            }
            0x8020ab64 => { ins += 1;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x8020ab68;
            }
            0x8020ab68 => { ins += 1;
                pc = 0x8020accc; continue;
                pc = 0x8020ab6c;
            }
            0x8020ab6c => { ins += 1;
                c.r[19] = c.r[23].rotate_left(0) & 0xff;
                pc = 0x8020ab70;
            }
            0x8020ab70 => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020ab74;
            }
            0x8020ab74 => { ins += 1;
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020ab78;
            }
            0x8020ab78 => { ins += 1;
                c.r[20] = 0x0;
                pc = 0x8020ab7c;
            }
            0x8020ab7c => { ins += 1;
                c.lr = 0x8020ab80; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a7f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ab80;
            }
            0x8020ab80 => { ins += 1;
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x8020ab84;
            }
            0x8020ab84 => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020ab88;
            }
            0x8020ab88 => { ins += 1;
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020ab8c;
            }
            0x8020ab8c => { ins += 1;
                c.lr = 0x8020ab90; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a780(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ab90;
            }
            0x8020ab90 => { ins += 1;
                c.r[21] = c.r[3] | c.r[3];
                pc = 0x8020ab94;
            }
            0x8020ab94 => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020ab98;
            }
            0x8020ab98 => { ins += 1;
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020ab9c;
            }
            0x8020ab9c => { ins += 1;
                c.lr = 0x8020aba0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a710(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020aba0;
            }
            0x8020aba0 => { ins += 1;
                c.r[22] = c.r[3] | c.r[3];
                pc = 0x8020aba4;
            }
            0x8020aba4 => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020aba8;
            }
            0x8020aba8 => { ins += 1;
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020abac;
            }
            0x8020abac => { ins += 1;
                c.lr = 0x8020abb0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a630(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020abb0;
            }
            0x8020abb0 => { ins += 1;
                c.r[24] = c.r[3] | c.r[3];
                pc = 0x8020abb4;
            }
            0x8020abb4 => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020abb8;
            }
            0x8020abb8 => { ins += 1;
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020abbc;
            }
            0x8020abbc => { ins += 1;
                c.lr = 0x8020abc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a5c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020abc0;
            }
            0x8020abc0 => { ins += 1;
                c.r[25] = c.r[3] | c.r[3];
                pc = 0x8020abc4;
            }
            0x8020abc4 => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020abc8;
            }
            0x8020abc8 => { ins += 1;
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020abcc;
            }
            0x8020abcc => { ins += 1;
                c.lr = 0x8020abd0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a6a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020abd0;
            }
            0x8020abd0 => { ins += 1;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                pc = 0x8020abd4;
            }
            0x8020abd4 => { ins += 1;
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x8020abd8;
            }
            0x8020abd8 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                pc = 0x8020abdc;
            }
            0x8020abdc => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020ac08; continue; }
                pc = 0x8020abe0;
            }
            0x8020abe0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8020abf4; continue; }
                pc = 0x8020abe4;
            }
            0x8020abe4 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                pc = 0x8020abe8;
            }
            0x8020abe8 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020ac9c; continue; }
                pc = 0x8020abec;
            }
            0x8020abec => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8020ac00; continue; }
                pc = 0x8020abf0;
            }
            0x8020abf0 => { ins += 1;
                pc = 0x8020ac9c; continue;
                pc = 0x8020abf4;
            }
            0x8020abf4 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x4);
                pc = 0x8020abf8;
            }
            0x8020abf8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8020ac9c; continue; }
                pc = 0x8020abfc;
            }
            0x8020abfc => { ins += 1;
                pc = 0x8020ac20; continue;
                pc = 0x8020ac00;
            }
            0x8020ac00 => { ins += 1;
                c.r[20] = c.r[21].rotate_left(0) & 0xffff;
                pc = 0x8020ac04;
            }
            0x8020ac04 => { ins += 1;
                pc = 0x8020ac9c; continue;
                pc = 0x8020ac08;
            }
            0x8020ac08 => { ins += 1;
                c.r[19] = c.r[21].rotate_left(0) & 0xffff;
                pc = 0x8020ac0c;
            }
            0x8020ac0c => { ins += 1;
                c.r[0] = c.r[22].rotate_left(0) & 0xffff;
                pc = 0x8020ac10;
            }
            0x8020ac10 => { ins += 1;
                c.r[3] = c.r[0].wrapping_sub(c.r[19]);
                pc = 0x8020ac14;
            }
            0x8020ac14 => { ins += 1;
                c.lr = 0x8020ac18; { c.steps = st; c.cycles = ins; c.cr = cr; f_800e0c04(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ac18;
            }
            0x8020ac18 => { ins += 1;
                c.r[20] = c.r[19].wrapping_add(c.r[3]);
                pc = 0x8020ac1c;
            }
            0x8020ac1c => { ins += 1;
                pc = 0x8020ac9c; continue;
                pc = 0x8020ac20;
            }
            0x8020ac20 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x8020ac24;
            }
            0x8020ac24 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x8020ac28;
            }
            0x8020ac28 => { ins += 1;
                c.r[5] = 0x14;
                pc = 0x8020ac2c;
            }
            0x8020ac2c => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x8020ac30;
            }
            0x8020ac30 => { ins += 1;
                c.lr = 0x8020ac34; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ac34;
            }
            0x8020ac34 => { ins += 1;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8020ac38;
            }
            0x8020ac38 => { ins += 1;
                c.r[3] = c.r[21] | c.r[21];
                pc = 0x8020ac3c;
            }
            0x8020ac3c => { ins += 1;
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8020ac40;
            }
            0x8020ac40 => { ins += 1;
                c.lr = 0x8020ac44; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ac44;
            }
            0x8020ac44 => { ins += 1;
                c.r[20] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[20]);
                pc = 0x8020ac48;
            }
            0x8020ac48 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020ac54; continue; }
                pc = 0x8020ac4c;
            }
            0x8020ac4c => { ins += 1;
                c.r[20] = 0x0;
                pc = 0x8020ac50;
            }
            0x8020ac50 => { ins += 1;
                pc = 0x8020acc0; continue;
                pc = 0x8020ac54;
            }
            0x8020ac54 => { ins += 1;
                c.r[3] = c.r[21] | c.r[21];
                pc = 0x8020ac58;
            }
            0x8020ac58 => { ins += 1;
                c.lr = 0x8020ac5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0234(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ac5c;
            }
            0x8020ac5c => { ins += 1;
                c.lr = 0x8020ac60; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f021c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ac60;
            }
            0x8020ac60 => { ins += 1;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x8020ac64;
            }
            0x8020ac64 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020ac84; continue; }
                pc = 0x8020ac68;
            }
            0x8020ac68 => { ins += 1;
                c.r[4] = c.r[20] | c.r[20];
                pc = 0x8020ac6c;
            }
            0x8020ac6c => { ins += 1;
                c.r[6] = c.r[22] | c.r[22];
                pc = 0x8020ac70;
            }
            0x8020ac70 => { ins += 1;
                c.r[5] = c.r[24].rotate_left(0) & 0xffff;
                pc = 0x8020ac74;
            }
            0x8020ac74 => { ins += 1;
                c.r[7] = c.r[25].rotate_left(0) & 0xffff;
                pc = 0x8020ac78;
            }
            0x8020ac78 => { ins += 1;
                c.lr = 0x8020ac7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80135e44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ac7c;
            }
            0x8020ac7c => { ins += 1;
                c.r[20] = c.r[3] | c.r[3];
                pc = 0x8020ac80;
            }
            0x8020ac80 => { ins += 1;
                pc = 0x8020ac9c; continue;
                pc = 0x8020ac84;
            }
            0x8020ac84 => { ins += 1;
                c.r[4] = c.r[20] | c.r[20];
                pc = 0x8020ac88;
            }
            0x8020ac88 => { ins += 1;
                c.r[6] = c.r[22] | c.r[22];
                pc = 0x8020ac8c;
            }
            0x8020ac8c => { ins += 1;
                c.r[5] = 0x0;
                pc = 0x8020ac90;
            }
            0x8020ac90 => { ins += 1;
                c.r[7] = 0x0;
                pc = 0x8020ac94;
            }
            0x8020ac94 => { ins += 1;
                c.lr = 0x8020ac98; { c.steps = st; c.cycles = ins; c.cr = cr; f_80135e44(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ac98;
            }
            0x8020ac98 => { ins += 1;
                c.r[20] = c.r[3] | c.r[3];
                pc = 0x8020ac9c;
            }
            0x8020ac9c => { ins += 1;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                pc = 0x8020aca0;
            }
            0x8020aca0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                pc = 0x8020aca4;
            }
            0x8020aca4 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020acc0; continue; }
                pc = 0x8020aca8;
            }
            0x8020aca8 => { ins += 1;
                c.r[3] = c.r[24] as u16 as i16 as i32 as u32;
                pc = 0x8020acac;
            }
            0x8020acac => { ins += 1;
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x8020acb0;
            }
            0x8020acb0 => { ins += 5;
                c.r[20] = c.r[20].wrapping_mul(c.r[3]);
                pc = 0x8020acb4;
            }
            0x8020acb4 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020acc0; continue; }
                pc = 0x8020acb8;
            }
            0x8020acb8 => { ins += 1;
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                pc = 0x8020acbc;
            }
            0x8020acbc => { ins += 40;
                c.r[20] = divw(c.r[20], c.r[0]);
                pc = 0x8020acc0;
            }
            0x8020acc0 => { ins += 1;
                c.r[0] = c.r[23].rotate_left(2) & 0x3fc;
                pc = 0x8020acc4;
            }
            0x8020acc4 => { ins += 1;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020acc8;
            }
            0x8020acc8 => { ins += 1;
                c.stw(c.r[27].wrapping_add(c.r[0]), c.r[20]);
                pc = 0x8020accc;
            }
            0x8020accc => { ins += 1;
                c.r[0] = c.r[23].rotate_left(0) & 0xff;
                pc = 0x8020acd0;
            }
            0x8020acd0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                pc = 0x8020acd4;
            }
            0x8020acd4 => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x8020ab6c; continue; }
                pc = 0x8020acd8;
            }
            0x8020acd8 => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020acdc;
            }
            0x8020acdc => { ins += 1;
                c.r[23] = 0x0;
                pc = 0x8020ace0;
            }
            0x8020ace0 => { ins += 1;
                c.lr = 0x8020ace4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a8a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ace4;
            }
            0x8020ace4 => { ins += 1;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8020ace8;
            }
            0x8020ace8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                pc = 0x8020acec;
            }
            0x8020acec => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x8020ad9c; continue; }
                pc = 0x8020acf0;
            }
            0x8020acf0 => { ins += 1;
                c.r[3] = 0x80370000;
                pc = 0x8020acf4;
            }
            0x8020acf4 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x8020acf8;
            }
            0x8020acf8 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x5938);
                pc = 0x8020acfc;
            }
            0x8020acfc => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[0]));
                pc = 0x8020ad00;
            }
            0x8020ad00 => { ins += 2;
                c.mtspr(9, c.r[0]);
                pc = 0x8020ad04;
            }
            0x8020ad04 => { ins += 1;
                if true { let t = c.ctr; if (0x8020a8e0..0x8020ae30).contains(&t) { pc = t; continue; } { c.steps = st; c.cycles = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.cycles; cr = c.cr; }; { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020ad08;
            }
            0x8020ad08 => { ins += 1;
                c.r[23] = 0x1;
                pc = 0x8020ad0c;
            }
            0x8020ad0c => { ins += 1;
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad10;
            }
            0x8020ad10 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                pc = 0x8020ad14;
            }
            0x8020ad14 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                pc = 0x8020ad18;
            }
            0x8020ad18 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020ad1c;
            }
            0x8020ad1c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad20;
            }
            0x8020ad20 => { ins += 1;
                c.r[23] = 0x1;
                pc = 0x8020ad24;
            }
            0x8020ad24 => { ins += 1;
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad28;
            }
            0x8020ad28 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                pc = 0x8020ad2c;
            }
            0x8020ad2c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                pc = 0x8020ad30;
            }
            0x8020ad30 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020ad34;
            }
            0x8020ad34 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad38;
            }
            0x8020ad38 => { ins += 1;
                c.r[23] = 0x1;
                pc = 0x8020ad3c;
            }
            0x8020ad3c => { ins += 1;
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad40;
            }
            0x8020ad40 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                pc = 0x8020ad44;
            }
            0x8020ad44 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                pc = 0x8020ad48;
            }
            0x8020ad48 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020ad4c;
            }
            0x8020ad4c => { ins += 1;
                if cr_bit(cr, 0) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad50;
            }
            0x8020ad50 => { ins += 1;
                c.r[23] = 0x1;
                pc = 0x8020ad54;
            }
            0x8020ad54 => { ins += 1;
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad58;
            }
            0x8020ad58 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                pc = 0x8020ad5c;
            }
            0x8020ad5c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                pc = 0x8020ad60;
            }
            0x8020ad60 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020ad64;
            }
            0x8020ad64 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad68;
            }
            0x8020ad68 => { ins += 1;
                c.r[23] = 0x1;
                pc = 0x8020ad6c;
            }
            0x8020ad6c => { ins += 1;
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad70;
            }
            0x8020ad70 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                pc = 0x8020ad74;
            }
            0x8020ad74 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                pc = 0x8020ad78;
            }
            0x8020ad78 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020ad7c;
            }
            0x8020ad7c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad80;
            }
            0x8020ad80 => { ins += 1;
                c.r[23] = 0x1;
                pc = 0x8020ad84;
            }
            0x8020ad84 => { ins += 1;
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad88;
            }
            0x8020ad88 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                pc = 0x8020ad8c;
            }
            0x8020ad8c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                pc = 0x8020ad90;
            }
            0x8020ad90 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x8020ad94;
            }
            0x8020ad94 => { ins += 1;
                if !cr_bit(cr, 1) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad98;
            }
            0x8020ad98 => { ins += 1;
                c.r[23] = 0x1;
                pc = 0x8020ad9c;
            }
            0x8020ad9c => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020ada0;
            }
            0x8020ada0 => { ins += 1;
                c.lr = 0x8020ada4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a580(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ada4;
            }
            0x8020ada4 => { ins += 1;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8020ada8;
            }
            0x8020ada8 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                pc = 0x8020adac;
            }
            0x8020adac => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020ade0; continue; }
                pc = 0x8020adb0;
            }
            0x8020adb0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8020adfc; continue; }
                pc = 0x8020adb4;
            }
            0x8020adb4 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x1);
                pc = 0x8020adb8;
            }
            0x8020adb8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x8020adc0; continue; }
                pc = 0x8020adbc;
            }
            0x8020adbc => { ins += 1;
                pc = 0x8020adfc; continue;
                pc = 0x8020adc0;
            }
            0x8020adc0 => { ins += 1;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x8020adc4;
            }
            0x8020adc4 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020add0; continue; }
                pc = 0x8020adc8;
            }
            0x8020adc8 => { ins += 1;
                c.r[0] = c.r[23].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x8020adcc;
            }
            0x8020adcc => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020add8; continue; }
                pc = 0x8020add0;
            }
            0x8020add0 => { ins += 1;
                c.r[30] = 0x1;
                pc = 0x8020add4;
            }
            0x8020add4 => { ins += 1;
                pc = 0x8020adfc; continue;
                pc = 0x8020add8;
            }
            0x8020add8 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x8020addc;
            }
            0x8020addc => { ins += 1;
                pc = 0x8020adfc; continue;
                pc = 0x8020ade0;
            }
            0x8020ade0 => { ins += 1;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x8020ade4;
            }
            0x8020ade4 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020adf8; continue; }
                pc = 0x8020ade8;
            }
            0x8020ade8 => { ins += 1;
                c.r[0] = c.r[23].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x8020adec;
            }
            0x8020adec => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x8020adf8; continue; }
                pc = 0x8020adf0;
            }
            0x8020adf0 => { ins += 1;
                c.r[30] = 0x1;
                pc = 0x8020adf4;
            }
            0x8020adf4 => { ins += 1;
                pc = 0x8020adfc; continue;
                pc = 0x8020adf8;
            }
            0x8020adf8 => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x8020adfc;
            }
            0x8020adfc => { ins += 1;
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x8020ae00;
            }
            0x8020ae00 => { ins += 1;
                c.lr = 0x8020ae04; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a500(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ae04;
            }
            0x8020ae04 => { ins += 1;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x8020ae08;
            }
            0x8020ae08 => { ins += 1;
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8020ae0c;
            }
            0x8020ae0c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020ab58; continue; }
                pc = 0x8020ae10;
            }
            0x8020ae10 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x8020ae14;
            }
            0x8020ae14 => { ins += 1;
                pc = 0x8020ae1c; continue;
                pc = 0x8020ae18;
            }
            0x8020ae18 => { ins += 1;
                pc = 0x8020ab54; continue;
                pc = 0x8020ae1c;
            }
            0x8020ae1c => { ins += 11;
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 19..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                pc = 0x8020ae20;
            }
            0x8020ae20 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                pc = 0x8020ae24;
            }
            0x8020ae24 => { ins += 2;
                c.mtspr(8, c.r[0]);
                pc = 0x8020ae28;
            }
            0x8020ae28 => { ins += 1;
                c.r[1] = c.r[1].wrapping_add(0x50);
                pc = 0x8020ae2c;
            }
            0x8020ae2c => { ins += 1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020a8e0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispWazaKiaipantiPre (0x8020ae30, 8 bytes)
#[inline(never)]
pub fn f_8020ae30(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae30) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae30); p = true; }
    }
    f_8020ae30_b(c, 0x8020ae30);
    if p { crate::prof::exit(c, 0x8020ae30); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae30_at(c: &mut Cpu, pc: u32) { f_8020ae30_b(c, pc) }
#[inline(always)]
fn f_8020ae30_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae30;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae30, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae30 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae30: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispTenkouInit (0x8020ae38, 8 bytes)
#[inline(never)]
pub fn f_8020ae38(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae38) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae38); p = true; }
    }
    f_8020ae38_b(c, 0x8020ae38);
    if p { crate::prof::exit(c, 0x8020ae38); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae38_at(c: &mut Cpu, pc: u32) { f_8020ae38_b(c, pc) }
#[inline(always)]
fn f_8020ae38_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae38;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae38, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae38 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae38: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispHeijou (0x8020ae40, 8 bytes)
#[inline(never)]
pub fn f_8020ae40(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae40) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae40); p = true; }
    }
    f_8020ae40_b(c, 0x8020ae40);
    if p { crate::prof::exit(c, 0x8020ae40); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae40_at(c: &mut Cpu, pc: u32) { f_8020ae40_b(c, pc) }
#[inline(always)]
fn f_8020ae40_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae40;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae40, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae40 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae40: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispSyuuryouPost (0x8020ae48, 8 bytes)
#[inline(never)]
pub fn f_8020ae48(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae48) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae48); p = true; }
    }
    f_8020ae48_b(c, 0x8020ae48);
    if p { crate::prof::exit(c, 0x8020ae48); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae48_at(c: &mut Cpu, pc: u32) { f_8020ae48_b(c, pc) }
#[inline(always)]
fn f_8020ae48_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae48;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae48, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae48 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae48: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispSyuuryou (0x8020ae50, 8 bytes)
#[inline(never)]
pub fn f_8020ae50(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae50) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae50); p = true; }
    }
    f_8020ae50_b(c, 0x8020ae50);
    if p { crate::prof::exit(c, 0x8020ae50); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae50_at(c: &mut Cpu, pc: u32) { f_8020ae50_b(c, pc) }
#[inline(always)]
fn f_8020ae50_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae50;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae50, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae50 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae50: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispSyuuryouPre (0x8020ae58, 8 bytes)
#[inline(never)]
pub fn f_8020ae58(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae58) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae58); p = true; }
    }
    f_8020ae58_b(c, 0x8020ae58);
    if p { crate::prof::exit(c, 0x8020ae58); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae58_at(c: &mut Cpu, pc: u32) { f_8020ae58_b(c, pc) }
#[inline(always)]
fn f_8020ae58_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae58;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae58, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae58 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae58: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispFightOutPokemonOutWaza (0x8020ae60, 8 bytes)
#[inline(never)]
pub fn f_8020ae60(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae60) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae60); p = true; }
    }
    f_8020ae60_b(c, 0x8020ae60);
    if p { crate::prof::exit(c, 0x8020ae60); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae60_at(c: &mut Cpu, pc: u32) { f_8020ae60_b(c, pc) }
#[inline(always)]
fn f_8020ae60_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae60;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae60, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae60 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae60: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispFightTrainerUseItem (0x8020ae68, 8 bytes)
#[inline(never)]
pub fn f_8020ae68(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae68) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae68); p = true; }
    }
    f_8020ae68_b(c, 0x8020ae68);
    if p { crate::prof::exit(c, 0x8020ae68); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae68_at(c: &mut Cpu, pc: u32) { f_8020ae68_b(c, pc) }
#[inline(always)]
fn f_8020ae68_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae68;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae68, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae68 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae68: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispFightTrainerCall (0x8020ae70, 8 bytes)
#[inline(never)]
pub fn f_8020ae70(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae70) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae70); p = true; }
    }
    f_8020ae70_b(c, 0x8020ae70);
    if p { crate::prof::exit(c, 0x8020ae70); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae70_at(c: &mut Cpu, pc: u32) { f_8020ae70_b(c, pc) }
#[inline(always)]
fn f_8020ae70_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae70;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae70, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae70 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae70: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispFightOutPokemonIrekae (0x8020ae78, 8 bytes)
#[inline(never)]
pub fn f_8020ae78(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae78) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae78); p = true; }
    }
    f_8020ae78_b(c, 0x8020ae78);
    if p { crate::prof::exit(c, 0x8020ae78); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae78_at(c: &mut Cpu, pc: u32) { f_8020ae78_b(c, pc) }
#[inline(always)]
fn f_8020ae78_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae78;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae78, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae78 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae78: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispFightNigeru (0x8020ae80, 8 bytes)
#[inline(never)]
pub fn f_8020ae80(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae80) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae80); p = true; }
    }
    f_8020ae80_b(c, 0x8020ae80);
    if p { crate::prof::exit(c, 0x8020ae80); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae80_at(c: &mut Cpu, pc: u32) { f_8020ae80_b(c, pc) }
#[inline(always)]
fn f_8020ae80_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae80;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae80, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae80 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae80: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispOneTurnPost (0x8020ae88, 8 bytes)
#[inline(never)]
pub fn f_8020ae88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae88) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae88); p = true; }
    }
    f_8020ae88_b(c, 0x8020ae88);
    if p { crate::prof::exit(c, 0x8020ae88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae88_at(c: &mut Cpu, pc: u32) { f_8020ae88_b(c, pc) }
#[inline(always)]
fn f_8020ae88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae88, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae88 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae88: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispAllFightOutPokemonDoFightAction (0x8020ae90, 8 bytes)
#[inline(never)]
pub fn f_8020ae90(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae90) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae90); p = true; }
    }
    f_8020ae90_b(c, 0x8020ae90);
    if p { crate::prof::exit(c, 0x8020ae90); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae90_at(c: &mut Cpu, pc: u32) { f_8020ae90_b(c, pc) }
#[inline(always)]
fn f_8020ae90_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae90;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae90, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae90 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae90: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispAllFightTrainerSelectFightAction (0x8020ae98, 8 bytes)
#[inline(never)]
pub fn f_8020ae98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ae98) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae98); p = true; }
    }
    f_8020ae98_b(c, 0x8020ae98);
    if p { crate::prof::exit(c, 0x8020ae98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae98_at(c: &mut Cpu, pc: u32) { f_8020ae98_b(c, pc) }
#[inline(always)]
fn f_8020ae98_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ae98;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ae98, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ae98 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ae98: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispKaisiPost (0x8020aea0, 8 bytes)
#[inline(never)]
pub fn f_8020aea0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020aea0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aea0); p = true; }
    }
    f_8020aea0_b(c, 0x8020aea0);
    if p { crate::prof::exit(c, 0x8020aea0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aea0_at(c: &mut Cpu, pc: u32) { f_8020aea0_b(c, pc) }
#[inline(always)]
fn f_8020aea0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020aea0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020aea0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020aea0 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020aea0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispKaisiPre (0x8020aea8, 8 bytes)
#[inline(never)]
pub fn f_8020aea8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020aea8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aea8); p = true; }
    }
    f_8020aea8_b(c, 0x8020aea8);
    if p { crate::prof::exit(c, 0x8020aea8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aea8_at(c: &mut Cpu, pc: u32) { f_8020aea8_b(c, pc) }
#[inline(always)]
fn f_8020aea8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020aea8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020aea8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020aea8 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020aea8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispKaisiNyuujouPokemon (0x8020aeb0, 8 bytes)
#[inline(never)]
pub fn f_8020aeb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020aeb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aeb0); p = true; }
    }
    f_8020aeb0_b(c, 0x8020aeb0);
    if p { crate::prof::exit(c, 0x8020aeb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aeb0_at(c: &mut Cpu, pc: u32) { f_8020aeb0_b(c, pc) }
#[inline(always)]
fn f_8020aeb0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020aeb0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020aeb0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020aeb0 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020aeb0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispKaisiNyuujouTrainer (0x8020aeb8, 8 bytes)
#[inline(never)]
pub fn f_8020aeb8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020aeb8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aeb8); p = true; }
    }
    f_8020aeb8_b(c, 0x8020aeb8);
    if p { crate::prof::exit(c, 0x8020aeb8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aeb8_at(c: &mut Cpu, pc: u32) { f_8020aeb8_b(c, pc) }
#[inline(always)]
fn f_8020aeb8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020aeb8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020aeb8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020aeb8 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020aeb8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispKaijou (0x8020aec0, 8 bytes)
#[inline(never)]
pub fn f_8020aec0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020aec0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aec0); p = true; }
    }
    f_8020aec0_b(c, 0x8020aec0);
    if p { crate::prof::exit(c, 0x8020aec0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aec0_at(c: &mut Cpu, pc: u32) { f_8020aec0_b(c, pc) }
#[inline(always)]
fn f_8020aec0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020aec0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020aec0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020aec0 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020aec0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispNullFunc (0x8020aec8, 8 bytes)
#[inline(never)]
pub fn f_8020aec8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020aec8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aec8); p = true; }
    }
    f_8020aec8_b(c, 0x8020aec8);
    if p { crate::prof::exit(c, 0x8020aec8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aec8_at(c: &mut Cpu, pc: u32) { f_8020aec8_b(c, pc) }
#[inline(always)]
fn f_8020aec8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020aec8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020aec8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020aec8 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020aec8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowWazaKiaipantiPre (0x8020aed0, 96 bytes)
#[inline(never)]
pub fn f_8020aed0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020aed0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aed0); p = true; }
    }
    f_8020aed0_b(c, 0x8020aed0);
    if p { crate::prof::exit(c, 0x8020aed0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aed0_at(c: &mut Cpu, pc: u32) { f_8020aed0_b(c, pc) }
#[inline(always)]
fn f_8020aed0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020aed0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020aed0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020aed0 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020aee8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d908(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020aee8;
            }
            0x8020aee8 => { ins += 6;
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020af00; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020af00;
            }
            0x8020af00 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020af08; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d8f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020af08;
            }
            0x8020af08 => { ins += 4;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = 0x0;
                c.lr = 0x8020af18; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211b94(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020af18;
            }
            0x8020af18 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020aed0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowTenkouInit (0x8020af30, 196 bytes)
#[inline(never)]
pub fn f_8020af30(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020af30) { return; }
        if c.prof { crate::prof::enter(c, 0x8020af30); p = true; }
    }
    f_8020af30_b(c, 0x8020af30);
    if p { crate::prof::exit(c, 0x8020af30); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020af30_at(c: &mut Cpu, pc: u32) { f_8020af30_b(c, pc) }
#[inline(always)]
fn f_8020af30_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020af30;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020af30, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020af30 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020af50; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020da14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020af50;
            }
            0x8020af50 => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8020af5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f453c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020af5c;
            }
            0x8020af5c => { ins += 9;
                c.r[0] = 0x0;
                c.r[4] = 0x80210000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[31] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[4].wrapping_add(0xffffaff4);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x8020af80; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020af80;
            }
            0x8020af80 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020afdc; continue; }
                pc = 0x8020af88;
            }
            0x8020af88 => { ins += 6;
                c.r[7] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020afa0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020afa0;
            }
            0x8020afa0 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020afa8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80136368(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020afa8;
            }
            0x8020afa8 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x50;
                c.r[6] = 0x0;
                c.lr = 0x8020afc4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020afc4;
            }
            0x8020afc4 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020afcc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d8f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020afcc;
            }
            0x8020afcc => { ins += 4;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x8020afdc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211b94(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020afdc;
            }
            0x8020afdc => { ins += 17;
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020af30: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowTenkouInitSubGetSeqFightOutPokemonPtr__FPvUsPv (0x8020aff4, 92 bytes)
#[inline(never)]
pub fn f_8020aff4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020aff4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aff4); p = true; }
    }
    f_8020aff4_b(c, 0x8020aff4);
    if p { crate::prof::exit(c, 0x8020aff4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aff4_at(c: &mut Cpu, pc: u32) { f_8020aff4_b(c, pc) }
#[inline(always)]
fn f_8020aff4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020aff4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020aff4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020aff4 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[5] | c.r[5];
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0xee;
                c.lr = 0x8020b01c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b01c;
            }
            0x8020b01c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020b038; continue; }
                pc = 0x8020b024;
            }
            0x8020b024 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020b030; continue; }
                pc = 0x8020b02c;
            }
            0x8020b02c => { ins += 1;
                c.stw(c.r[31], c.r[30]);
                pc = 0x8020b030;
            }
            0x8020b030 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020b03c; continue;
                pc = 0x8020b038;
            }
            0x8020b038 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8020b03c;
            }
            0x8020b03c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020aff4: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowHeijou (0x8020b050, 8 bytes)
#[inline(never)]
pub fn f_8020b050(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020b050) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b050); p = true; }
    }
    f_8020b050_b(c, 0x8020b050);
    if p { crate::prof::exit(c, 0x8020b050); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b050_at(c: &mut Cpu, pc: u32) { f_8020b050_b(c, pc) }
#[inline(always)]
fn f_8020b050_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020b050;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020b050, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020b050 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020b050: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowSyuuryouPost (0x8020b058, 728 bytes)
#[inline(never)]
pub fn f_8020b058(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020b058) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b058); p = true; }
    }
    f_8020b058_b(c, 0x8020b058);
    if p { crate::prof::exit(c, 0x8020b058); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b058_at(c: &mut Cpu, pc: u32) { f_8020b058_b(c, pc) }
#[inline(never)]
fn f_8020b058_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020b058;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020b058, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020b058 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020b07c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b07c;
            }
            0x8020b07c => { ins += 2;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8020b084; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b084;
            }
            0x8020b084 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b098; continue; }
                pc = 0x8020b090;
            }
            0x8020b090 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x8020b31c; continue;
                pc = 0x8020b098;
            }
            0x8020b098 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020b0a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2a7c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b0a0;
            }
            0x8020b0a0 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if cr_bit(cr, 2) { pc = 0x8020b268; continue; }
                pc = 0x8020b0a8;
            }
            0x8020b0a8 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x8020b0b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b0b8;
            }
            0x8020b0b8 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x8020b268; continue; }
                pc = 0x8020b0c0;
            }
            0x8020b0c0 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8020b0cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f86c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b0cc;
            }
            0x8020b0cc => { ins += 1;
                c.lr = 0x8020b0d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b0d0;
            }
            0x8020b0d0 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020b0dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1dbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b0dc;
            }
            0x8020b0dc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b22c; continue; }
                pc = 0x8020b0e8;
            }
            0x8020b0e8 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x24;
                c.r[6] = 0x0;
                c.lr = 0x8020b0fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b0fc;
            }
            0x8020b0fc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b1f0; continue; }
                pc = 0x8020b108;
            }
            0x8020b108 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020b110; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f9034(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b110;
            }
            0x8020b110 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b1f0; continue; }
                pc = 0x8020b11c;
            }
            0x8020b11c => { ins += 2;
                c.r[26] = 0x0;
                pc = 0x8020b1e4; continue;
                pc = 0x8020b124;
            }
            0x8020b124 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[26].rotate_left(0) & 0xffff;
                c.r[4] = 0x3;
                c.lr = 0x8020b134; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012a5b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b134;
            }
            0x8020b134 => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x8020b13c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801233f4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b13c;
            }
            0x8020b13c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b144;
            }
            0x8020b144 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020b14c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011fc74(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b14c;
            }
            0x8020b14c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b158;
            }
            0x8020b158 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8020b164; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f9930(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b164;
            }
            0x8020b164 => { ins += 2;
                c.r[25] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[25]);
                if cr_bit(cr, 2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b16c;
            }
            0x8020b16c => { ins += 1;
                c.lr = 0x8020b170; { c.steps = st; c.cycles = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b170;
            }
            0x8020b170 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b178;
            }
            0x8020b178 => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0xd0;
                c.r[6] = 0x0;
                c.lr = 0x8020b18c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b18c;
            }
            0x8020b18c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b194;
            }
            0x8020b194 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[7] = c.r[1].wrapping_add(0xc);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8020b1ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80128a64(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b1ac;
            }
            0x8020b1ac => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[4] = c.r[3] | c.r[3];
                if cr_bit(cr, 2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b1b8;
            }
            0x8020b1b8 => { ins += 8;
                c.r[5] = c.lhz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[30] | c.r[30];
                c.r[6] = c.r[1].wrapping_add(0xc);
                c.r[8] = 0x1;
                c.r[9] = 0x1;
                c.r[10] = 0x0;
                c.lr = 0x8020b1d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012805c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b1d8;
            }
            0x8020b1d8 => { ins += 2;
                c.r[3] = 0xa;
                c.lr = 0x8020b1e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801effc4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b1e0;
            }
            0x8020b1e0 => { ins += 1;
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8020b1e4;
            }
            0x8020b1e4 => { ins += 3;
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x8020b124; continue; }
                pc = 0x8020b1f0;
            }
            0x8020b1f0 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.lr = 0x8020b204; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b204;
            }
            0x8020b204 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b218; continue; }
                pc = 0x8020b210;
            }
            0x8020b210 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020b218; { c.steps = st; c.cycles = ins; c.cr = cr; f_80129840(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b218;
            }
            0x8020b218 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.lr = 0x8020b22c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b22c;
            }
            0x8020b22c => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x1c;
                c.r[6] = 0x0;
                c.lr = 0x8020b240; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b240;
            }
            0x8020b240 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b268; continue; }
                pc = 0x8020b24c;
            }
            0x8020b24c => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x2;
                c.lr = 0x8020b258; { c.steps = st; c.cycles = ins; c.cr = cr; f_80129280(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b258;
            }
            0x8020b258 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020b268; continue; }
                pc = 0x8020b260;
            }
            0x8020b260 => { ins += 2;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020b268; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012ac64(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b268;
            }
            0x8020b268 => { ins += 1;
                c.lr = 0x8020b26c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8006b57c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b26c;
            }
            0x8020b26c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b318; continue; }
                pc = 0x8020b278;
            }
            0x8020b278 => { ins += 2;
                c.r[26] = 0x0;
                pc = 0x8020b30c; continue;
                pc = 0x8020b280;
            }
            0x8020b280 => { ins += 3;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.lr = 0x8020b28c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f47b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b28c;
            }
            0x8020b28c => { ins += 2;
                c.r[25] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[25]);
                if cr_bit(cr, 2) { pc = 0x8020b308; continue; }
                pc = 0x8020b294;
            }
            0x8020b294 => { ins += 8;
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.r[27] = 0x0;
                c.r[30] = c.r[0].wrapping_mul(c.r[29]);
                pc = 0x8020b2fc; continue;
                pc = 0x8020b2a4;
            }
            0x8020b2a4 => { ins += 3;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8020b2b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7258(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b2b0;
            }
            0x8020b2b0 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if cr_bit(cr, 2) { pc = 0x8020b2f8; continue; }
                pc = 0x8020b2b8;
            }
            0x8020b2b8 => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x8020b2c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f86c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b2c0;
            }
            0x8020b2c0 => { ins += 3;
                c.r[0] = c.r[27].wrapping_add(c.r[30]);
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x8020b2cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_8006b0f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b2cc;
            }
            0x8020b2cc => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if cr_bit(cr, 2) { pc = 0x8020b2f8; continue; }
                pc = 0x8020b2d4;
            }
            0x8020b2d4 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x8020b2e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b2e8;
            }
            0x8020b2e8 => { ins += 2;
                c.r[4] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[4]);
                if cr_bit(cr, 2) { pc = 0x8020b2f8; continue; }
                pc = 0x8020b2f0;
            }
            0x8020b2f0 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020b2f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012ac64(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b2f8;
            }
            0x8020b2f8 => { ins += 1;
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x8020b2fc;
            }
            0x8020b2fc => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[29]);
                if cr_bit(cr, 0) { pc = 0x8020b2a4; continue; }
                pc = 0x8020b308;
            }
            0x8020b308 => { ins += 1;
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8020b30c;
            }
            0x8020b30c => { ins += 3;
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8020b280; continue; }
                pc = 0x8020b318;
            }
            0x8020b318 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8020b31c;
            }
            0x8020b31c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020b058: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowSyuuryou (0x8020b330, 932 bytes)
#[inline(never)]
pub fn f_8020b330(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020b330) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b330); p = true; }
    }
    f_8020b330_b(c, 0x8020b330);
    if p { crate::prof::exit(c, 0x8020b330); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b330_at(c: &mut Cpu, pc: u32) { f_8020b330_b(c, pc) }
#[inline(never)]
fn f_8020b330_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020b330;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020b330, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020b330 => { ins += 16;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020b348; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d920(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b348;
            }
            0x8020b348 => { ins += 1;
                c.lr = 0x8020b34c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d814(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b34c;
            }
            0x8020b34c => { ins += 4;
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0xb;
                c.r[4] = 0x0;
                c.lr = 0x8020b35c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b35c;
            }
            0x8020b35c => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x9;
                c.r[25] = c.r[0] | c.r[0];
                c.r[4] = c.r[25] | c.r[25];
                c.lr = 0x8020b370; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b370;
            }
            0x8020b370 => { ins += 5;
                c.r[4] = 0x0;
                c.r[24] = c.r[3] | c.r[3];
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x8020b384; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b384;
            }
            0x8020b384 => { ins += 6;
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020b39c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b39c;
            }
            0x8020b39c => { ins += 6;
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x1;
                c.lr = 0x8020b3b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b3b4;
            }
            0x8020b3b4 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8020b3bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_800896b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b3bc;
            }
            0x8020b3bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x8020b3ec; continue; }
                pc = 0x8020b3c4;
            }
            0x8020b3c4 => { ins += 1;
                c.lr = 0x8020b3c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_800896c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b3c8;
            }
            0x8020b3c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020b3d8; continue; }
                pc = 0x8020b3d0;
            }
            0x8020b3d0 => { ins += 2;
                c.r[23] = 0x0;
                pc = 0x8020b458; continue;
                pc = 0x8020b3d8;
            }
            0x8020b3d8 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x24;
                c.lr = 0x8020b3e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b3e4;
            }
            0x8020b3e4 => { ins += 2;
                c.r[23] = 0x7531;
                pc = 0x8020b458; continue;
                pc = 0x8020b3ec;
            }
            0x8020b3ec => { ins += 5;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x4a;
                c.r[6] = 0x0;
                c.lr = 0x8020b400; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b400;
            }
            0x8020b400 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020b424; continue; }
                pc = 0x8020b408;
            }
            0x8020b408 => { ins += 5;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x2;
                c.lr = 0x8020b41c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b41c;
            }
            0x8020b41c => { ins += 2;
                c.r[23] = c.r[3] | c.r[3];
                pc = 0x8020b458; continue;
                pc = 0x8020b424;
            }
            0x8020b424 => { ins += 5;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x3;
                c.lr = 0x8020b438; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b438;
            }
            0x8020b438 => { ins += 2;
                c.r[23] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[23]);
                if !cr_bit(cr, 2) { pc = 0x8020b458; continue; }
                pc = 0x8020b440;
            }
            0x8020b440 => { ins += 5;
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x2;
                c.lr = 0x8020b454; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b454;
            }
            0x8020b454 => { ins += 1;
                c.r[23] = c.r[3] | c.r[3];
                pc = 0x8020b458;
            }
            0x8020b458 => { ins += 2;
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020b460; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8000(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b460;
            }
            0x8020b460 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x22;
                c.lr = 0x8020b46c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b46c;
            }
            0x8020b46c => { ins += 2;
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020b474; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8100(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b474;
            }
            0x8020b474 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x23;
                c.lr = 0x8020b480; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b480;
            }
            0x8020b480 => { ins += 2;
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8020b488; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8100(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b488;
            }
            0x8020b488 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x13;
                c.lr = 0x8020b494; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b494;
            }
            0x8020b494 => { ins += 2;
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020b49c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8100(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b49c;
            }
            0x8020b49c => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x25;
                c.lr = 0x8020b4a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b4a8;
            }
            0x8020b4a8 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020b4bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b4bc;
            }
            0x8020b4bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b654; continue; }
                pc = 0x8020b4c8;
            }
            0x8020b4c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x2);
                if !cr_bit(cr, 2) { pc = 0x8020b57c; continue; }
                pc = 0x8020b4d0;
            }
            0x8020b4d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020b4f4; continue; }
                pc = 0x8020b4d8;
            }
            0x8020b4d8 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x5a;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020b4ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b4ec;
            }
            0x8020b4ec => { ins += 1;
                c.lr = 0x8020b4f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c2d54(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b4f0;
            }
            0x8020b4f0 => { ins += 1;
                c.r[27] = c.r[3] | c.r[3];
                pc = 0x8020b4f4;
            }
            0x8020b4f4 => { ins += 4;
                c.r[3] = 0x3f5;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x8020b504; { c.steps = st; c.cycles = ins; c.cr = cr; f_80165668(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b504;
            }
            0x8020b504 => { ins += 3;
                c.r[3] = 0x5d;
                c.r[4] = 0x0;
                c.lr = 0x8020b510; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b510;
            }
            0x8020b510 => { ins += 2;
                c.r[3] = 0x766c;
                c.lr = 0x8020b518; { c.steps = st; c.cycles = ins; c.cr = cr; f_802624cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b518;
            }
            0x8020b518 => { ins += 1;
                c.lr = 0x8020b51c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026246c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b51c;
            }
            0x8020b51c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020b654; continue; }
                pc = 0x8020b524;
            }
            0x8020b524 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x5a;
                c.r[5] = 0x4;
                c.lr = 0x8020b534; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b534;
            }
            0x8020b534 => { ins += 2;
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x8020b53c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80262490(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b53c;
            }
            0x8020b53c => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x5a;
                c.r[5] = 0x4;
                c.lr = 0x8020b54c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b54c;
            }
            0x8020b54c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020b55c; continue; }
                pc = 0x8020b554;
            }
            0x8020b554 => { ins += 1;
                c.lr = 0x8020b558; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b558;
            }
            0x8020b558 => { ins += 1;
                pc = 0x8020b53c; continue;
                pc = 0x8020b55c;
            }
            0x8020b55c => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020b564; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef8f4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b564;
            }
            0x8020b564 => { ins += 1;
                c.lr = 0x8020b568; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026246c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b568;
            }
            0x8020b568 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x5a;
                c.r[5] = 0x4;
                c.lr = 0x8020b578; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b578;
            }
            0x8020b578 => { ins += 1;
                pc = 0x8020b654; continue;
                pc = 0x8020b57c;
            }
            0x8020b57c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x3);
                if !cr_bit(cr, 2) { pc = 0x8020b620; continue; }
                pc = 0x8020b584;
            }
            0x8020b584 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020b5a8; continue; }
                pc = 0x8020b58c;
            }
            0x8020b58c => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x59;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020b5a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b5a0;
            }
            0x8020b5a0 => { ins += 1;
                c.lr = 0x8020b5a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c2d54(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b5a4;
            }
            0x8020b5a4 => { ins += 1;
                c.r[27] = c.r[3] | c.r[3];
                pc = 0x8020b5a8;
            }
            0x8020b5a8 => { ins += 2;
                c.r[3] = 0x7547;
                c.lr = 0x8020b5b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_802624cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b5b0;
            }
            0x8020b5b0 => { ins += 1;
                c.lr = 0x8020b5b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026246c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b5b4;
            }
            0x8020b5b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020b610; continue; }
                pc = 0x8020b5bc;
            }
            0x8020b5bc => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x59;
                c.r[5] = 0x4;
                c.lr = 0x8020b5cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b5cc;
            }
            0x8020b5cc => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020b5d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80262490(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b5d4;
            }
            0x8020b5d4 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x59;
                c.r[5] = 0x4;
                c.lr = 0x8020b5e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b5e4;
            }
            0x8020b5e4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020b5f4; continue; }
                pc = 0x8020b5ec;
            }
            0x8020b5ec => { ins += 1;
                c.lr = 0x8020b5f0; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b5f0;
            }
            0x8020b5f0 => { ins += 1;
                pc = 0x8020b5d4; continue;
                pc = 0x8020b5f4;
            }
            0x8020b5f4 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020b5fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef8f4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b5fc;
            }
            0x8020b5fc => { ins += 1;
                c.lr = 0x8020b600; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026246c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b600;
            }
            0x8020b600 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x59;
                c.r[5] = 0x4;
                c.lr = 0x8020b610; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b610;
            }
            0x8020b610 => { ins += 2;
                c.r[3] = 0x7548;
                c.lr = 0x8020b618; { c.steps = st; c.cycles = ins; c.cr = cr; f_802624cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b618;
            }
            0x8020b618 => { ins += 1;
                c.lr = 0x8020b61c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026246c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b61c;
            }
            0x8020b61c => { ins += 1;
                pc = 0x8020b654; continue;
                pc = 0x8020b620;
            }
            0x8020b620 => { ins += 4;
                c.r[0] = c.r[30].wrapping_add(0xfffffffc);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 1) { pc = 0x8020b654; continue; }
                pc = 0x8020b630;
            }
            0x8020b630 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x7);
                if cr_bit(cr, 2) { pc = 0x8020b640; continue; }
                pc = 0x8020b638;
            }
            0x8020b638 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x6);
                if !cr_bit(cr, 2) { pc = 0x8020b654; continue; }
                pc = 0x8020b640;
            }
            0x8020b640 => { ins += 2;
                c.r[3] = 0x7640;
                c.lr = 0x8020b648; { c.steps = st; c.cycles = ins; c.cr = cr; f_802624cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b648;
            }
            0x8020b648 => { ins += 2;
                c.r[3] = 0x40;
                c.lr = 0x8020b650; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f000c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b650;
            }
            0x8020b650 => { ins += 1;
                c.lr = 0x8020b654; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026246c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b654;
            }
            0x8020b654 => { ins += 1;
                c.lr = 0x8020b658; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b658;
            }
            0x8020b658 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020b664; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1dbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b664;
            }
            0x8020b664 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b6b8; continue; }
                pc = 0x8020b670;
            }
            0x8020b670 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x25;
                c.r[6] = 0x0;
                c.lr = 0x8020b684; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b684;
            }
            0x8020b684 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b6b8; continue; }
                pc = 0x8020b690;
            }
            0x8020b690 => { ins += 5;
                c.r[4] = 0x80380000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[4].wrapping_add(0xffff8801);
                c.r[5] = 0x0;
                c.lr = 0x8020b6a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211b94(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b6a4;
            }
            0x8020b6a4 => { ins += 5;
                c.r[4] = 0x80380000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[4].wrapping_add(0xffff880f);
                c.r[5] = 0x0;
                c.lr = 0x8020b6b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211b94(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b6b8;
            }
            0x8020b6b8 => { ins += 1;
                c.lr = 0x8020b6bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef2d4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b6bc;
            }
            0x8020b6bc => { ins += 17;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020b330: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowSyuuryouPre (0x8020b6d4, 88 bytes)
#[inline(never)]
pub fn f_8020b6d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020b6d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b6d4); p = true; }
    }
    f_8020b6d4_b(c, 0x8020b6d4);
    if p { crate::prof::exit(c, 0x8020b6d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b6d4_at(c: &mut Cpu, pc: u32) { f_8020b6d4_b(c, pc) }
#[inline(always)]
fn f_8020b6d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020b6d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020b6d4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020b6d4 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0x12;
                c.r[6] = 0x0;
                c.lr = 0x8020b6f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b6f4;
            }
            0x8020b6f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020b718; continue; }
                pc = 0x8020b6fc;
            }
            0x8020b6fc => { ins += 5;
                c.r[3] = 0x1;
                c.r[4] = 0x3e8;
                c.r[5] = 0x3e8;
                c.r[6] = 0xff;
                c.lr = 0x8020b710; { c.steps = st; c.cycles = ins; c.cr = cr; f_8016597c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b710;
            }
            0x8020b710 => { ins += 2;
                c.r[3] = 0x3c;
                c.lr = 0x8020b718; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f000c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b718;
            }
            0x8020b718 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020b6d4: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowFightOutPokemonOutWaza (0x8020b72c, 484 bytes)
#[inline(never)]
pub fn f_8020b72c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020b72c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b72c); p = true; }
    }
    f_8020b72c_b(c, 0x8020b72c);
    if p { crate::prof::exit(c, 0x8020b72c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b72c_at(c: &mut Cpu, pc: u32) { f_8020b72c_b(c, pc) }
#[inline(never)]
fn f_8020b72c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020b72c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020b72c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020b72c => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020b754; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b754;
            }
            0x8020b754 => { ins += 3;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020b760; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d908(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b760;
            }
            0x8020b760 => { ins += 5;
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8020b774; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b774;
            }
            0x8020b774 => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x8020b77c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80209cb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b77c;
            }
            0x8020b77c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020b78c; continue; }
                pc = 0x8020b784;
            }
            0x8020b784 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020b8fc; continue;
                pc = 0x8020b78c;
            }
            0x8020b78c => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.lr = 0x8020b7a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b7a0;
            }
            0x8020b7a0 => { ins += 3;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8020b7ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f00d0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b7ac;
            }
            0x8020b7ac => { ins += 7;
                c.r[29] = c.r[3] | c.r[3];
                c.r[7] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020b7c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b7c8;
            }
            0x8020b7c8 => { ins += 6;
                c.r[7] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x8020b7e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b7e0;
            }
            0x8020b7e0 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020b7e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205b8c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b7e8;
            }
            0x8020b7e8 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[25] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8020b804; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b804;
            }
            0x8020b804 => { ins += 3;
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020b810; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205224(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b810;
            }
            0x8020b810 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[26] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x26;
                c.r[6] = 0x0;
                c.lr = 0x8020b82c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b82c;
            }
            0x8020b82c => { ins += 6;
                c.r[29] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x32;
                c.r[6] = 0x0;
                c.lr = 0x8020b844; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011beb4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b844;
            }
            0x8020b844 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020b8ec; continue; }
                pc = 0x8020b84c;
            }
            0x8020b84c => { ins += 6;
                c.r[29] = c.r[29].rotate_left(0) & 0xff;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.r[6] = c.r[29] | c.r[29];
                c.lr = 0x8020b864; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b864;
            }
            0x8020b864 => { ins += 4;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[3]);
                if cr_bit(cr, 2) { pc = 0x8020b8ec; continue; }
                pc = 0x8020b874;
            }
            0x8020b874 => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[6] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x8020b888; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b888;
            }
            0x8020b888 => { ins += 7;
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[7] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x8020b8a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011bbd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b8a4;
            }
            0x8020b8a4 => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x8020b8b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80209c1c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b8b0;
            }
            0x8020b8b0 => { ins += 8;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.r[9] = 0xffffffff;
                c.lr = 0x8020b8d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022b2cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b8d0;
            }
            0x8020b8d0 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x8020b8ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b8ec;
            }
            0x8020b8ec => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x8020b8f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802128d0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b8f8;
            }
            0x8020b8f8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8020b8fc;
            }
            0x8020b8fc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020b72c: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowFightTrainerUseItem (0x8020b910, 260 bytes)
#[inline(never)]
pub fn f_8020b910(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020b910) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b910); p = true; }
    }
    f_8020b910_b(c, 0x8020b910);
    if p { crate::prof::exit(c, 0x8020b910); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b910_at(c: &mut Cpu, pc: u32) { f_8020b910_b(c, pc) }
#[inline(always)]
fn f_8020b910_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020b910;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020b910, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020b910 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020b938; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b938;
            }
            0x8020b938 => { ins += 3;
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020b944; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d908(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b944;
            }
            0x8020b944 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[7] = c.r[31] | c.r[31];
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020b964; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b964;
            }
            0x8020b964 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xe5;
                c.r[6] = 0x0;
                c.lr = 0x8020b978; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b978;
            }
            0x8020b978 => { ins += 5;
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.lr = 0x8020b98c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b98c;
            }
            0x8020b98c => { ins += 6;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020b9a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b9a4;
            }
            0x8020b9a4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020b9d4; continue; }
                pc = 0x8020b9b0;
            }
            0x8020b9b0 => { ins += 5;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.lr = 0x8020b9c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80142cf4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b9c4;
            }
            0x8020b9c4 => { ins += 3;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8020b9d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f00d0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b9d0;
            }
            0x8020b9d0 => { ins += 1;
                pc = 0x8020b9d8; continue;
                pc = 0x8020b9d4;
            }
            0x8020b9d4 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x8020b9d8;
            }
            0x8020b9d8 => { ins += 6;
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x8020b9f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b9f0;
            }
            0x8020b9f0 => { ins += 3;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8020b9fc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211e18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020b9fc;
            }
            0x8020b9fc => { ins += 17;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020b910: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowFightTrainerCall (0x8020ba14, 108 bytes)
#[inline(never)]
pub fn f_8020ba14(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ba14) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ba14); p = true; }
    }
    f_8020ba14_b(c, 0x8020ba14);
    if p { crate::prof::exit(c, 0x8020ba14); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ba14_at(c: &mut Cpu, pc: u32) { f_8020ba14_b(c, pc) }
#[inline(always)]
fn f_8020ba14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ba14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ba14, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ba14 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020ba3c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ba3c;
            }
            0x8020ba3c => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020ba44; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d908(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ba44;
            }
            0x8020ba44 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020ba60; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ba60;
            }
            0x8020ba60 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020ba68; { c.steps = st; c.cycles = ins; c.cr = cr; f_80212d6c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ba68;
            }
            0x8020ba68 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ba14: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowFightOutPokemonIrekae (0x8020ba80, 120 bytes)
#[inline(never)]
pub fn f_8020ba80(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ba80) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ba80); p = true; }
    }
    f_8020ba80_b(c, 0x8020ba80);
    if p { crate::prof::exit(c, 0x8020ba80); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ba80_at(c: &mut Cpu, pc: u32) { f_8020ba80_b(c, pc) }
#[inline(always)]
fn f_8020ba80_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ba80;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ba80, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ba80 => { ins += 16;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x8020ba98; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d908(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ba98;
            }
            0x8020ba98 => { ins += 8;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[7] = c.r[31] | c.r[31];
                c.r[5] = 0x45;
                c.r[6] = 0x0;
                c.lr = 0x8020bab8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bab8;
            }
            0x8020bab8 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020bac0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d8d8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bac0;
            }
            0x8020bac0 => { ins += 6;
                c.r[7] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x121;
                c.r[6] = 0x0;
                c.lr = 0x8020bad8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bad8;
            }
            0x8020bad8 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020bae0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80213158(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bae0;
            }
            0x8020bae0 => { ins += 17;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ba80: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowFightNigeru (0x8020baf8, 172 bytes)
#[inline(never)]
pub fn f_8020baf8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020baf8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020baf8); p = true; }
    }
    f_8020baf8_b(c, 0x8020baf8);
    if p { crate::prof::exit(c, 0x8020baf8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020baf8_at(c: &mut Cpu, pc: u32) { f_8020baf8_b(c, pc) }
#[inline(always)]
fn f_8020baf8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020baf8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020baf8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020baf8 => { ins += 20;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020bb20; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bb20;
            }
            0x8020bb20 => { ins += 3;
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020bb2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d908(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bb2c;
            }
            0x8020bb2c => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020bb34; { c.steps = st; c.cycles = ins; c.cr = cr; f_80212840(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bb34;
            }
            0x8020bb34 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020bb40; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0058(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bb40;
            }
            0x8020bb40 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020bb5c; continue; }
                pc = 0x8020bb4c;
            }
            0x8020bb4c => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x4;
                c.lr = 0x8020bb58; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f3984(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bb58;
            }
            0x8020bb58 => { ins += 1;
                pc = 0x8020bb68; continue;
                pc = 0x8020bb5c;
            }
            0x8020bb5c => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x5;
                c.lr = 0x8020bb68; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f3984(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bb68;
            }
            0x8020bb68 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020bb8c; continue; }
                pc = 0x8020bb74;
            }
            0x8020bb74 => { ins += 6;
                c.r[7] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x8020bb8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bb8c;
            }
            0x8020bb8c => { ins += 17;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020baf8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowOneTurnPost (0x8020bba4, 88 bytes)
#[inline(never)]
pub fn f_8020bba4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020bba4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020bba4); p = true; }
    }
    f_8020bba4_b(c, 0x8020bba4);
    if p { crate::prof::exit(c, 0x8020bba4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020bba4_at(c: &mut Cpu, pc: u32) { f_8020bba4_b(c, pc) }
#[inline(always)]
fn f_8020bba4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020bba4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020bba4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020bba4 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020bbbc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bbbc;
            }
            0x8020bbbc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020bbcc; continue; }
                pc = 0x8020bbc4;
            }
            0x8020bbc4 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x8020bbe8; continue;
                pc = 0x8020bbcc;
            }
            0x8020bbcc => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020bbd4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802119d4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bbd4;
            }
            0x8020bbd4 => { ins += 2;
                c.r[3] = 0x5;
                c.lr = 0x8020bbdc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f000c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bbdc;
            }
            0x8020bbdc => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020bbe4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4ac0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bbe4;
            }
            0x8020bbe4 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8020bbe8;
            }
            0x8020bbe8 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020bba4: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowAllFightOutPokemonDoFightAction (0x8020bbfc, 152 bytes)
#[inline(never)]
pub fn f_8020bbfc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020bbfc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020bbfc); p = true; }
    }
    f_8020bbfc_b(c, 0x8020bbfc);
    if p { crate::prof::exit(c, 0x8020bbfc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020bbfc_at(c: &mut Cpu, pc: u32) { f_8020bbfc_b(c, pc) }
#[inline(always)]
fn f_8020bbfc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020bbfc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020bbfc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020bbfc => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020bc18; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4718(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bc18;
            }
            0x8020bc18 => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x8020bc24; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f3b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bc24;
            }
            0x8020bc24 => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8020bc30; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020bc94(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bc30;
            }
            0x8020bc30 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8020bc40; continue; }
                pc = 0x8020bc3c;
            }
            0x8020bc3c => { ins += 1;
                pc = 0x8020bc80; continue;
                pc = 0x8020bc40;
            }
            0x8020bc40 => { ins += 1;
                c.lr = 0x8020bc44; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bc44;
            }
            0x8020bc44 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020bc54; continue; }
                pc = 0x8020bc4c;
            }
            0x8020bc4c => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x8020bc80; continue;
                pc = 0x8020bc54;
            }
            0x8020bc54 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020bc5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211a00(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bc5c;
            }
            0x8020bc5c => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x8020bc68; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020bc94(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bc68;
            }
            0x8020bc68 => { ins += 4;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = 0x1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8020bc7c; continue; }
                pc = 0x8020bc78;
            }
            0x8020bc78 => { ins += 1;
                c.r[4] = c.r[3] | c.r[3];
                pc = 0x8020bc7c;
            }
            0x8020bc7c => { ins += 1;
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x8020bc80;
            }
            0x8020bc80 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020bbfc: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowAllFightOutPokemonDoFightActionOneLoop__FP11FIGHT_FLOORUc (0x8020bc94, 420 bytes)
#[inline(never)]
pub fn f_8020bc94(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020bc94) { return; }
        if c.prof { crate::prof::enter(c, 0x8020bc94); p = true; }
    }
    f_8020bc94_b(c, 0x8020bc94);
    if p { crate::prof::exit(c, 0x8020bc94); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020bc94_at(c: &mut Cpu, pc: u32) { f_8020bc94_b(c, pc) }
#[inline(never)]
fn f_8020bc94_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020bc94;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020bc94, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020bc94 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = 0x0;
                pc = 0x8020be14; continue;
                pc = 0x8020bcb4;
            }
            0x8020bcb4 => { ins += 5;
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x59;
                c.lr = 0x8020bcc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bcc8;
            }
            0x8020bcc8 => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[27]);
                if cr_bit(cr, 2) { pc = 0x8020be10; continue; }
                pc = 0x8020bcd0;
            }
            0x8020bcd0 => { ins += 1;
                c.lr = 0x8020bcd4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802062fc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bcd4;
            }
            0x8020bcd4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020bcf8; continue; }
                pc = 0x8020bcdc;
            }
            0x8020bcdc => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x8020bcf4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bcf4;
            }
            0x8020bcf4 => { ins += 1;
                pc = 0x8020be10; continue;
                pc = 0x8020bcf8;
            }
            0x8020bcf8 => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0xfe;
                c.r[6] = 0x0;
                c.lr = 0x8020bd0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bd0c;
            }
            0x8020bd0c => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if !cr_bit(cr, 2) { pc = 0x8020bd30; continue; }
                pc = 0x8020bd14;
            }
            0x8020bd14 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x8020bd2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bd2c;
            }
            0x8020bd2c => { ins += 1;
                pc = 0x8020be10; continue;
                pc = 0x8020bd30;
            }
            0x8020bd30 => { ins += 1;
                c.lr = 0x8020bd34; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1170(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bd34;
            }
            0x8020bd34 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020bd58; continue; }
                pc = 0x8020bd3c;
            }
            0x8020bd3c => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x8020bd54; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bd54;
            }
            0x8020bd54 => { ins += 1;
                pc = 0x8020be10; continue;
                pc = 0x8020bd58;
            }
            0x8020bd58 => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020bd78; continue; }
                pc = 0x8020bd60;
            }
            0x8020bd60 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020bd68; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0898(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bd68;
            }
            0x8020bd68 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if !cr_bit(cr, 2) { pc = 0x8020be10; continue; }
                pc = 0x8020bd74;
            }
            0x8020bd74 => { ins += 1;
                pc = 0x8020bd8c; continue;
                pc = 0x8020bd78;
            }
            0x8020bd78 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020bd80; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0898(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bd80;
            }
            0x8020bd80 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if cr_bit(cr, 2) { pc = 0x8020be10; continue; }
                pc = 0x8020bd8c;
            }
            0x8020bd8c => { ins += 5;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.r[6] = 0x0;
                c.lr = 0x8020bda0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bda0;
            }
            0x8020bda0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                if cr_bit(cr, 2) { pc = 0x8020be10; continue; }
                pc = 0x8020bda8;
            }
            0x8020bda8 => { ins += 6;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x8020bdc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801254b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bdc0;
            }
            0x8020bdc0 => { ins += 5;
                c.r[0] = 0x6;
                c.r[5] = c.r[1].wrapping_add(0x4);
                c.r[4] = c.r[28].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x8020bdd0;
            }
            0x8020bdd0 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8020bdd0; continue; }
                pc = 0x8020bde4;
            }
            0x8020bde4 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8020bdec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f0f04(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bdec;
            }
            0x8020bdec => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020be10; continue; }
                pc = 0x8020bdf4;
            }
            0x8020bdf4 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020bdfc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4ac0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bdfc;
            }
            0x8020bdfc => { ins += 1;
                c.lr = 0x8020be00; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020be00;
            }
            0x8020be00 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020be10; continue; }
                pc = 0x8020be08;
            }
            0x8020be08 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x8020be24; continue;
                pc = 0x8020be10;
            }
            0x8020be10 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x8020be14;
            }
            0x8020be14 => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if cr_bit(cr, 0) { pc = 0x8020bcb4; continue; }
                pc = 0x8020be20;
            }
            0x8020be20 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8020be24;
            }
            0x8020be24 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020bc94: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowAllFightTrainerSelectFightAction (0x8020be38, 264 bytes)
#[inline(never)]
pub fn f_8020be38(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020be38) { return; }
        if c.prof { crate::prof::enter(c, 0x8020be38); p = true; }
    }
    f_8020be38_b(c, 0x8020be38);
    if p { crate::prof::exit(c, 0x8020be38); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020be38_at(c: &mut Cpu, pc: u32) { f_8020be38_b(c, pc) }
#[inline(always)]
fn f_8020be38_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020be38;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020be38, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020be38 => { ins += 15;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020be4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80008174(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020be4c;
            }
            0x8020be4c => { ins += 4;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[30] = c.r[3] | c.r[3];
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020bf10; continue; }
                pc = 0x8020be5c;
            }
            0x8020be5c => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8020be70; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020be70;
            }
            0x8020be70 => { ins += 6;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                c.lr = 0x8020be88; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020be88;
            }
            0x8020be88 => { ins += 3;
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[25] = 0x0;
                pc = 0x8020bf00; continue;
                pc = 0x8020be94;
            }
            0x8020be94 => { ins += 3;
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0x0;
                c.lr = 0x8020bea0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f47b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bea0;
            }
            0x8020bea0 => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[29]);
                if cr_bit(cr, 2) { pc = 0x8020befc; continue; }
                pc = 0x8020bea8;
            }
            0x8020bea8 => { ins += 3;
                c.r[31] = c.r[25].rotate_left(0) & 0xffff;
                c.r[26] = 0x0;
                pc = 0x8020bef0; continue;
                pc = 0x8020beb4;
            }
            0x8020beb4 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x8020bec0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7258(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bec0;
            }
            0x8020bec0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020beec; continue; }
                pc = 0x8020bec8;
            }
            0x8020bec8 => { ins += 3;
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8020bed4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026316c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bed4;
            }
            0x8020bed4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020beec; continue; }
                pc = 0x8020bedc;
            }
            0x8020bedc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020beb4; continue; }
                pc = 0x8020bee4;
            }
            0x8020bee4 => { ins += 2;
                c.r[25] = c.r[25].wrapping_add(0xffffffff);
                pc = 0x8020be94; continue;
                pc = 0x8020beec;
            }
            0x8020beec => { ins += 1;
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8020bef0;
            }
            0x8020bef0 => { ins += 3;
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x8020beb4; continue; }
                pc = 0x8020befc;
            }
            0x8020befc => { ins += 1;
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x8020bf00;
            }
            0x8020bf00 => { ins += 3;
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 0) { pc = 0x8020be94; continue; }
                pc = 0x8020bf0c;
            }
            0x8020bf0c => { ins += 1;
                pc = 0x8020bf28; continue;
                pc = 0x8020bf10;
            }
            0x8020bf10 => { ins += 6;
                c.r[4] = 0x80210000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffffbf40);
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x8020bf28; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2b5c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bf28;
            }
            0x8020bf28 => { ins += 17;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020be38: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowFightTrainerSelectFightAction__FPvUsPv (0x8020bf40, 96 bytes)
#[inline(never)]
pub fn f_8020bf40(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020bf40) { return; }
        if c.prof { crate::prof::enter(c, 0x8020bf40); p = true; }
    }
    f_8020bf40_b(c, 0x8020bf40);
    if p { crate::prof::exit(c, 0x8020bf40); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020bf40_at(c: &mut Cpu, pc: u32) { f_8020bf40_b(c, pc) }
#[inline(always)]
fn f_8020bf40_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020bf40;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020bf40, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020bf40 => { ins += 17;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x8020bf5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bf5c;
            }
            0x8020bf5c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020bf6c; continue; }
                pc = 0x8020bf64;
            }
            0x8020bf64 => { ins += 2;
                c.r[3] = 0x1;
                pc = 0x8020bf8c; continue;
                pc = 0x8020bf6c;
            }
            0x8020bf6c => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8020bf78; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f923c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bf78;
            }
            0x8020bf78 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020bf88; continue; }
                pc = 0x8020bf80;
            }
            0x8020bf80 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020bf88; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f150c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bf88;
            }
            0x8020bf88 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8020bf8c;
            }
            0x8020bf8c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020bf40: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaisiPost (0x8020bfa0, 288 bytes)
#[inline(never)]
pub fn f_8020bfa0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020bfa0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020bfa0); p = true; }
    }
    f_8020bfa0_b(c, 0x8020bfa0);
    if p { crate::prof::exit(c, 0x8020bfa0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020bfa0_at(c: &mut Cpu, pc: u32) { f_8020bfa0_b(c, pc) }
#[inline(always)]
fn f_8020bfa0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020bfa0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020bfa0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020bfa0 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020bfbc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4718(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bfbc;
            }
            0x8020bfbc => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8020bfc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f3b24(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bfc8;
            }
            0x8020bfc8 => { ins += 1;
                c.lr = 0x8020bfcc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211830(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bfcc;
            }
            0x8020bfcc => { ins += 8;
                c.r[0] = 0x0;
                c.r[3] = 0x80210000;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xffffc108);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x8020bfec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bfec;
            }
            0x8020bfec => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020bff4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d920(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020bff4;
            }
            0x8020bff4 => { ins += 8;
                c.r[4] = 0x80370000;
                c.r[5] = 0x80380000;
                c.r[7] = c.r[4].wrapping_add(0x5cc8);
                c.r[6] = 0x0;
                c.r[8] = c.r[5].wrapping_add(0xffff8aa0);
                c.r[4] = 0x0;
                c.r[5] = 0x6;
                c.lr = 0x8020c014; { c.steps = st; c.cycles = ins; c.cr = cr; f_80211948(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c014;
            }
            0x8020c014 => { ins += 6;
                c.r[4] = 0x80210000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffffc0e4);
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x8020c02c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c02c;
            }
            0x8020c02c => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x8020c034; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022e314(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c034;
            }
            0x8020c034 => { ins += 1;
                c.lr = 0x8020c038; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022e1c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c038;
            }
            0x8020c038 => { ins += 6;
                c.r[4] = 0x80210000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffffc0c0);
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x8020c050; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c050;
            }
            0x8020c050 => { ins += 8;
                c.r[0] = 0x1;
                c.r[3] = 0x80210000;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xffffc108);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x8020c070; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f37b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c070;
            }
            0x8020c070 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020c078; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f3178(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c078;
            }
            0x8020c078 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020c080; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f3074(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c080;
            }
            0x8020c080 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020c088; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f2f3c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c088;
            }
            0x8020c088 => { ins += 1;
                c.lr = 0x8020c08c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800e0c54(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c08c;
            }
            0x8020c08c => { ins += 6;
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x5b;
                c.r[6] = 0x0;
                c.lr = 0x8020c0a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c0a4;
            }
            0x8020c0a4 => { ins += 1;
                c.lr = 0x8020c0a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da7ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c0a8;
            }
            0x8020c0a8 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[3] = 0x1;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020bfa0: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowKaisiPostSubFightOutPokemonSoubiItemCheckAppear__FPvUsPv (0x8020c0c0, 36 bytes)
#[inline(never)]
pub fn f_8020c0c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020c0c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020c0c0); p = true; }
    }
    f_8020c0c0_b(c, 0x8020c0c0);
    if p { crate::prof::exit(c, 0x8020c0c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020c0c0_at(c: &mut Cpu, pc: u32) { f_8020c0c0_b(c, pc) }
#[inline(always)]
fn f_8020c0c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020c0c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020c0c0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020c0c0 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8020c0d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022d084(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c0d0;
            }
            0x8020c0d0 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020c0c0: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowKaisiPostSubFightOutPokemonTokuseiCheckAppear__FPvUsPv (0x8020c0e4, 36 bytes)
#[inline(never)]
pub fn f_8020c0e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020c0e4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020c0e4); p = true; }
    }
    f_8020c0e4_b(c, 0x8020c0e4);
    if p { crate::prof::exit(c, 0x8020c0e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020c0e4_at(c: &mut Cpu, pc: u32) { f_8020c0e4_b(c, pc) }
#[inline(always)]
fn f_8020c0e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020c0e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020c0e4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020c0e4 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8020c0f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022e410(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c0f4;
            }
            0x8020c0f4 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020c0e4: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowKaisiPostSubFightOutPokemonDarkCheckAppear__FPvUsPv (0x8020c108, 84 bytes)
#[inline(never)]
pub fn f_8020c108(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020c108) { return; }
        if c.prof { crate::prof::enter(c, 0x8020c108); p = true; }
    }
    f_8020c108_b(c, 0x8020c108);
    if p { crate::prof::exit(c, 0x8020c108); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020c108_at(c: &mut Cpu, pc: u32) { f_8020c108_b(c, pc) }
#[inline(always)]
fn f_8020c108_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020c108;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020c108, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020c108 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                if cr_bit(cr, 2) { pc = 0x8020c130; continue; }
                pc = 0x8020c124;
            }
            0x8020c124 => { ins += 2;
                c.r[4] = c.lbz_t(ins, c.r[5]);
                c.lr = 0x8020c12c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022e6f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c12c;
            }
            0x8020c12c => { ins += 1;
                pc = 0x8020c144; continue;
                pc = 0x8020c130;
            }
            0x8020c130 => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x8020c138; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022e6f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c138;
            }
            0x8020c138 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x8020c144; { c.steps = st; c.cycles = ins; c.cr = cr; f_8022e6f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c144;
            }
            0x8020c144 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020c108: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaisiPre (0x8020c15c, 1764 bytes)
#[inline(never)]
pub fn f_8020c15c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020c15c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020c15c); p = true; }
    }
    f_8020c15c_b(c, 0x8020c15c);
    if p { crate::prof::exit(c, 0x8020c15c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020c15c_at(c: &mut Cpu, pc: u32) { f_8020c15c_b(c, pc) }
#[inline(never)]
fn f_8020c15c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020c15c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020c15c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020c15c => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x84), c.r[0]);
                c.r[0] = 0x0;
                c.r[5] = 0xe;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x60); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.lr = 0x8020c188; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c188;
            }
            0x8020c188 => { ins += 4;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0xb;
                c.r[4] = 0x0;
                c.lr = 0x8020c198; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c198;
            }
            0x8020c198 => { ins += 5;
                c.r[4] = 0x0;
                c.r[25] = c.r[3] | c.r[3];
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020c1ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c1ac;
            }
            0x8020c1ac => { ins += 4;
                c.r[4] = c.r[25] | c.r[25];
                c.r[25] = c.r[3] | c.r[3];
                c.r[3] = 0x9;
                c.lr = 0x8020c1bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c1bc;
            }
            0x8020c1bc => { ins += 5;
                c.r[4] = 0x0;
                c.r[29] = c.r[3] | c.r[3];
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020c1d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c1d0;
            }
            0x8020c1d0 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[26] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x8020c1ec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c1ec;
            }
            0x8020c1ec => { ins += 6;
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x10;
                c.r[6] = 0x0;
                c.lr = 0x8020c204; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c204;
            }
            0x8020c204 => { ins += 6;
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0xd;
                c.r[6] = 0x0;
                c.lr = 0x8020c21c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c21c;
            }
            0x8020c21c => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8020c224; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c224;
            }
            0x8020c224 => { ins += 1;
                c.lr = 0x8020c228; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020dfc0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c228;
            }
            0x8020c228 => { ins += 6;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[24] | c.r[24];
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x8020c240; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c240;
            }
            0x8020c240 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[28]);
                if !cr_bit(cr, 2) { pc = 0x8020c24c; continue; }
                pc = 0x8020c248;
            }
            0x8020c248 => { ins += 1;
                c.r[28] = 0x5f;
                pc = 0x8020c24c;
            }
            0x8020c24c => { ins += 5;
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x0;
                c.lr = 0x8020c260; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c260;
            }
            0x8020c260 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[27] = c.r[0] | c.r[0];
                c.lr = 0x8020c270; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1888(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c270;
            }
            0x8020c270 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020c69c; continue; }
                pc = 0x8020c278;
            }
            0x8020c278 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c430; continue; }
                pc = 0x8020c280;
            }
            0x8020c280 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c298; continue; }
                pc = 0x8020c288;
            }
            0x8020c288 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c298; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c298;
            }
            0x8020c298 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c2a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c2a8;
            }
            0x8020c2a8 => { ins += 2;
                c.r[0] = c.addc(c.r[1], 0x8, 0);
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020c2b8; continue; }
                pc = 0x8020c2b0;
            }
            0x8020c2b0 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8020c2ec; continue;
                pc = 0x8020c2b8;
            }
            0x8020c2b8 => { ins += 3;
                c.r[3] = c.r[1].wrapping_add(0x44);
                c.r[4] = 0x1;
                c.lr = 0x8020c2c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80103ba8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c2c4;
            }
            0x8020c2c4 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020c2cc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c40f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c2cc;
            }
            0x8020c2cc => { ins += 2;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020c2e8; continue; }
                pc = 0x8020c2d4;
            }
            0x8020c2d4 => { ins += 3;
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0x44));
                c.r[0] = c.r[0].rotate_left(0) & 0x20;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c2e8; continue; }
                pc = 0x8020c2e0;
            }
            0x8020c2e0 => { ins += 2;
                c.r[0] = 0x1;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8020c2e8;
            }
            0x8020c2e8 => { ins += 1;
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x8));
                pc = 0x8020c2ec;
            }
            0x8020c2ec => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8020c430; continue; }
                pc = 0x8020c2f8;
            }
            0x8020c2f8 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c308; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c308;
            }
            0x8020c308 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c318; continue; }
                pc = 0x8020c310;
            }
            0x8020c310 => { ins += 1;
                c.lr = 0x8020c314; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c314;
            }
            0x8020c314 => { ins += 1;
                pc = 0x8020c2a8; continue;
                pc = 0x8020c318;
            }
            0x8020c318 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c32c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c32c;
            }
            0x8020c32c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c3b8; continue; }
                pc = 0x8020c338;
            }
            0x8020c338 => { ins += 4;
                c.r[4] = c.lhz_t(ins, c.r[13].wrapping_add(0xffffadd8));
                c.r[3] = c.r[26] | c.r[26];
                c.r[5] = 0x4;
                c.lr = 0x8020c348; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c348;
            }
            0x8020c348 => { ins += 2;
                c.r[0] = c.addc(c.r[1], 0x8, 0);
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020c358; continue; }
                pc = 0x8020c350;
            }
            0x8020c350 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8020c38c; continue;
                pc = 0x8020c358;
            }
            0x8020c358 => { ins += 3;
                c.r[3] = c.r[1].wrapping_add(0x28);
                c.r[4] = 0x1;
                c.lr = 0x8020c364; { c.steps = st; c.cycles = ins; c.cr = cr; f_80103ba8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c364;
            }
            0x8020c364 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020c36c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c40f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c36c;
            }
            0x8020c36c => { ins += 2;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020c388; continue; }
                pc = 0x8020c374;
            }
            0x8020c374 => { ins += 3;
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0x28));
                c.r[0] = c.r[0].rotate_left(0) & 0x20;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c388; continue; }
                pc = 0x8020c380;
            }
            0x8020c380 => { ins += 2;
                c.r[0] = 0x1;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8020c388;
            }
            0x8020c388 => { ins += 1;
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x8));
                pc = 0x8020c38c;
            }
            0x8020c38c => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8020c430; continue; }
                pc = 0x8020c398;
            }
            0x8020c398 => { ins += 4;
                c.r[4] = c.lhz_t(ins, c.r[13].wrapping_add(0xffffadd8));
                c.r[3] = c.r[26] | c.r[26];
                c.r[5] = 0x4;
                c.lr = 0x8020c3a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c3a8;
            }
            0x8020c3a8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c3b8; continue; }
                pc = 0x8020c3b0;
            }
            0x8020c3b0 => { ins += 1;
                c.lr = 0x8020c3b4; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c3b4;
            }
            0x8020c3b4 => { ins += 1;
                pc = 0x8020c348; continue;
                pc = 0x8020c3b8;
            }
            0x8020c3b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c430; continue; }
                pc = 0x8020c3c0;
            }
            0x8020c3c0 => { ins += 2;
                c.r[0] = c.addc(c.r[1], 0x8, 0);
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020c3d0; continue; }
                pc = 0x8020c3c8;
            }
            0x8020c3c8 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8020c404; continue;
                pc = 0x8020c3d0;
            }
            0x8020c3d0 => { ins += 3;
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = 0x1;
                c.lr = 0x8020c3dc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80103ba8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c3dc;
            }
            0x8020c3dc => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020c3e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c40f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c3e4;
            }
            0x8020c3e4 => { ins += 2;
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020c400; continue; }
                pc = 0x8020c3ec;
            }
            0x8020c3ec => { ins += 3;
                c.r[0] = c.lhz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x20;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c400; continue; }
                pc = 0x8020c3f8;
            }
            0x8020c3f8 => { ins += 2;
                c.r[0] = 0x1;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8020c400;
            }
            0x8020c400 => { ins += 1;
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x8));
                pc = 0x8020c404;
            }
            0x8020c404 => { ins += 3;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x8020c430; continue; }
                pc = 0x8020c410;
            }
            0x8020c410 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c420; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c420;
            }
            0x8020c420 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c430; continue; }
                pc = 0x8020c428;
            }
            0x8020c428 => { ins += 1;
                c.lr = 0x8020c42c; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c42c;
            }
            0x8020c42c => { ins += 1;
                pc = 0x8020c3c0; continue;
                pc = 0x8020c430;
            }
            0x8020c430 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c4a8; continue; }
                pc = 0x8020c43c;
            }
            0x8020c43c => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffffae80); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x3;
                c.lr = 0x8020c448; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c41c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c448;
            }
            0x8020c448 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x8020c450; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c40f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c450;
            }
            0x8020c450 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c468; continue; }
                pc = 0x8020c458;
            }
            0x8020c458 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c468; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c468;
            }
            0x8020c468 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c478; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c478;
            }
            0x8020c478 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c48c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c48c;
            }
            0x8020c48c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c4a8; continue; }
                pc = 0x8020c498;
            }
            0x8020c498 => { ins += 4;
                c.r[4] = c.lhz_t(ins, c.r[13].wrapping_add(0xffffadd8));
                c.r[3] = c.r[26] | c.r[26];
                c.r[5] = 0x4;
                c.lr = 0x8020c4a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c4a8;
            }
            0x8020c4a8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c50c; continue; }
                pc = 0x8020c4b0;
            }
            0x8020c4b0 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x5f;
                c.r[5] = 0x4;
                c.lr = 0x8020c4c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c4c0;
            }
            0x8020c4c0 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c4e0; continue; }
                pc = 0x8020c4cc;
            }
            0x8020c4cc => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffffae80); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8020c4d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c41c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c4d8;
            }
            0x8020c4d8 => { ins += 2;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8020c4e0;
            }
            0x8020c4e0 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020c4e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80262490(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c4e8;
            }
            0x8020c4e8 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x5f;
                c.r[5] = 0x4;
                c.lr = 0x8020c4f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c4f8;
            }
            0x8020c4f8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c508; continue; }
                pc = 0x8020c500;
            }
            0x8020c500 => { ins += 1;
                c.lr = 0x8020c504; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c504;
            }
            0x8020c504 => { ins += 1;
                pc = 0x8020c4e8; continue;
                pc = 0x8020c508;
            }
            0x8020c508 => { ins += 1;
                c.lr = 0x8020c50c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026246c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c50c;
            }
            0x8020c50c => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x8020c520; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c520;
            }
            0x8020c520 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c534; continue; }
                pc = 0x8020c528;
            }
            0x8020c528 => { ins += 3;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x8020c534; { c.steps = st; c.cycles = ins; c.cr = cr; f_80165a20(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c534;
            }
            0x8020c534 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c548; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c548;
            }
            0x8020c548 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c5d8; continue; }
                pc = 0x8020c554;
            }
            0x8020c554 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020c55c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8000(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c55c;
            }
            0x8020c55c => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x22;
                c.lr = 0x8020c568; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c568;
            }
            0x8020c568 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020c570; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8100(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c570;
            }
            0x8020c570 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x23;
                c.lr = 0x8020c57c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c57c;
            }
            0x8020c57c => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c58c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c58c;
            }
            0x8020c58c => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c5ac; continue; }
                pc = 0x8020c598;
            }
            0x8020c598 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffffae80); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8020c5a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c41c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c5a4;
            }
            0x8020c5a4 => { ins += 2;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8020c5ac;
            }
            0x8020c5ac => { ins += 2;
                c.r[3] = 0x766d;
                c.lr = 0x8020c5b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80262490(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c5b4;
            }
            0x8020c5b4 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c5c4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c5c4;
            }
            0x8020c5c4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c5d4; continue; }
                pc = 0x8020c5cc;
            }
            0x8020c5cc => { ins += 1;
                c.lr = 0x8020c5d0; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c5d0;
            }
            0x8020c5d0 => { ins += 1;
                pc = 0x8020c5b4; continue;
                pc = 0x8020c5d4;
            }
            0x8020c5d4 => { ins += 1;
                c.lr = 0x8020c5d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026246c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c5d8;
            }
            0x8020c5d8 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c5f0; continue; }
                pc = 0x8020c5e4;
            }
            0x8020c5e4 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffffae80); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8020c5f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c41c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c5f0;
            }
            0x8020c5f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c650; continue; }
                pc = 0x8020c5f8;
            }
            0x8020c5f8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c610; continue; }
                pc = 0x8020c600;
            }
            0x8020c600 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c610; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c610;
            }
            0x8020c610 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c620; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c620;
            }
            0x8020c620 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c634; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c634;
            }
            0x8020c634 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c650; continue; }
                pc = 0x8020c640;
            }
            0x8020c640 => { ins += 4;
                c.r[4] = c.lhz_t(ins, c.r[13].wrapping_add(0xffffadd8));
                c.r[3] = c.r[26] | c.r[26];
                c.r[5] = 0x4;
                c.lr = 0x8020c650; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c650;
            }
            0x8020c650 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c668; continue; }
                pc = 0x8020c658;
            }
            0x8020c658 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x5f;
                c.r[5] = 0x4;
                c.lr = 0x8020c668; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c668;
            }
            0x8020c668 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c67c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c67c;
            }
            0x8020c67c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c828; continue; }
                pc = 0x8020c688;
            }
            0x8020c688 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c698; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c698;
            }
            0x8020c698 => { ins += 1;
                pc = 0x8020c828; continue;
                pc = 0x8020c69c;
            }
            0x8020c69c => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020c6a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef7c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c6a4;
            }
            0x8020c6a4 => { ins += 3;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x1;
                c.lr = 0x8020c6b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da4e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c6b0;
            }
            0x8020c6b0 => { ins += 4;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x54;
                c.r[5] = 0x4;
                c.lr = 0x8020c6c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c6c0;
            }
            0x8020c6c0 => { ins += 4;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x54;
                c.r[5] = 0x4;
                c.lr = 0x8020c6d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c6d0;
            }
            0x8020c6d0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c6e0; continue; }
                pc = 0x8020c6d8;
            }
            0x8020c6d8 => { ins += 1;
                c.lr = 0x8020c6dc; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c6dc;
            }
            0x8020c6dc => { ins += 1;
                pc = 0x8020c6c0; continue;
                pc = 0x8020c6e0;
            }
            0x8020c6e0 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020c6e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef7c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c6e8;
            }
            0x8020c6e8 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x1;
                c.lr = 0x8020c6f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da4e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c6f4;
            }
            0x8020c6f4 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x55;
                c.r[5] = 0x4;
                c.lr = 0x8020c704; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c704;
            }
            0x8020c704 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c718; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c718;
            }
            0x8020c718 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c754; continue; }
                pc = 0x8020c724;
            }
            0x8020c724 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020c72c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8000(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c72c;
            }
            0x8020c72c => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x22;
                c.lr = 0x8020c738; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c738;
            }
            0x8020c738 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020c740; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8100(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c740;
            }
            0x8020c740 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x23;
                c.lr = 0x8020c74c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c74c;
            }
            0x8020c74c => { ins += 2;
                c.r[3] = 0x766d;
                c.lr = 0x8020c754; { c.steps = st; c.cycles = ins; c.cr = cr; f_80262490(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c754;
            }
            0x8020c754 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x55;
                c.r[5] = 0x4;
                c.lr = 0x8020c764; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c764;
            }
            0x8020c764 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c774; continue; }
                pc = 0x8020c76c;
            }
            0x8020c76c => { ins += 1;
                c.lr = 0x8020c770; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c770;
            }
            0x8020c770 => { ins += 1;
                pc = 0x8020c754; continue;
                pc = 0x8020c774;
            }
            0x8020c774 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c788; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c788;
            }
            0x8020c788 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c798; continue; }
                pc = 0x8020c794;
            }
            0x8020c794 => { ins += 1;
                c.lr = 0x8020c798; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026246c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c798;
            }
            0x8020c798 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x8020c7a0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef7c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c7a0;
            }
            0x8020c7a0 => { ins += 4;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x56;
                c.r[5] = 0x4;
                c.lr = 0x8020c7b0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c7b0;
            }
            0x8020c7b0 => { ins += 4;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x56;
                c.r[5] = 0x4;
                c.lr = 0x8020c7c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da94c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c7c0;
            }
            0x8020c7c0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020c7d0; continue; }
                pc = 0x8020c7c8;
            }
            0x8020c7c8 => { ins += 1;
                c.lr = 0x8020c7cc; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c7cc;
            }
            0x8020c7cc => { ins += 1;
                pc = 0x8020c7b0; continue;
                pc = 0x8020c7d0;
            }
            0x8020c7d0 => { ins += 4;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x54;
                c.r[5] = 0x4;
                c.lr = 0x8020c7e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c7e0;
            }
            0x8020c7e0 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x55;
                c.r[5] = 0x4;
                c.lr = 0x8020c7f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c7f0;
            }
            0x8020c7f0 => { ins += 4;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x56;
                c.r[5] = 0x4;
                c.lr = 0x8020c800; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da8c4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c800;
            }
            0x8020c800 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x8020c814; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c814;
            }
            0x8020c814 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c828; continue; }
                pc = 0x8020c81c;
            }
            0x8020c81c => { ins += 3;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x8020c828; { c.steps = st; c.cycles = ins; c.cr = cr; f_80165a20(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c828;
            }
            0x8020c828 => { ins += 17;
                { let mut ea = c.r[1].wrapping_add(0x60); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020c15c: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaisiPreSubLoad (0x8020c840, 600 bytes)
#[inline(never)]
pub fn f_8020c840(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020c840) { return; }
        if c.prof { crate::prof::enter(c, 0x8020c840); p = true; }
    }
    f_8020c840_b(c, 0x8020c840);
    if p { crate::prof::exit(c, 0x8020c840); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020c840_at(c: &mut Cpu, pc: u32) { f_8020c840_b(c, pc) }
#[inline(never)]
fn f_8020c840_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020c840;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020c840, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020c840 => { ins += 19;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[5] = 0xe;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020c864; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c864;
            }
            0x8020c864 => { ins += 4;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0xb;
                c.r[4] = 0x0;
                c.lr = 0x8020c874; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c874;
            }
            0x8020c874 => { ins += 5;
                c.r[4] = 0x0;
                c.r[28] = c.r[3] | c.r[3];
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020c888; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c888;
            }
            0x8020c888 => { ins += 4;
                c.r[26] = c.r[3] | c.r[3];
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x9;
                c.lr = 0x8020c898; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f025c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c898;
            }
            0x8020c898 => { ins += 5;
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020c8ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c8ac;
            }
            0x8020c8ac => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x8020c8c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c8c8;
            }
            0x8020c8c8 => { ins += 6;
                c.r[25] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x10;
                c.r[6] = 0x0;
                c.lr = 0x8020c8e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c8e0;
            }
            0x8020c8e0 => { ins += 6;
                c.r[28] = c.r[3] | c.r[3];
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0xd;
                c.r[6] = 0x0;
                c.lr = 0x8020c8f8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c8f8;
            }
            0x8020c8f8 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8020c900; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c900;
            }
            0x8020c900 => { ins += 1;
                c.lr = 0x8020c904; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020dfc0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c904;
            }
            0x8020c904 => { ins += 6;
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[25] | c.r[25];
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x8020c91c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c91c;
            }
            0x8020c91c => { ins += 2;
                c.r[27] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[27]);
                if !cr_bit(cr, 2) { pc = 0x8020c928; continue; }
                pc = 0x8020c924;
            }
            0x8020c924 => { ins += 1;
                c.r[27] = 0x5f;
                pc = 0x8020c928;
            }
            0x8020c928 => { ins += 5;
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x0;
                c.lr = 0x8020c93c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c93c;
            }
            0x8020c93c => { ins += 3;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020c948; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1888(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c948;
            }
            0x8020c948 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020ca48; continue; }
                pc = 0x8020c950;
            }
            0x8020c950 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c9f4; continue; }
                pc = 0x8020c958;
            }
            0x8020c958 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020c974; continue; }
                pc = 0x8020c960;
            }
            0x8020c960 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[29].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020c974; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c974;
            }
            0x8020c974 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020c988; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c988;
            }
            0x8020c988 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c99c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c99c;
            }
            0x8020c99c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020c9ec; continue; }
                pc = 0x8020c9a8;
            }
            0x8020c9a8 => { ins += 1;
                c.lr = 0x8020c9ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_800e0c54(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c9ac;
            }
            0x8020c9ac => { ins += 58;
                c.r[7] = c.lbz_t(ins, c.r[13].wrapping_add(0xffff84f8));
                c.r[8] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = 0x80370000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = divw(c.r[8], c.r[7]);
                c.r[5] = 0x4;
                c.r[4] = c.r[4].wrapping_add(0x5970);
                c.r[6] = 0x0;
                c.r[0] = c.r[0].wrapping_mul(c.r[7]);
                c.r[7] = c.r[8].wrapping_sub(c.r[0]);
                c.r[0] = c.r[7].rotate_left(1) & 0x1fffe;
                c.sth(c.r[13].wrapping_add(0xffffadd8), c.r[7]);
                c.r[4] = c.lhz_t(ins, c.r[4].wrapping_add(c.r[0]));
                c.sth(c.r[13].wrapping_add(0xffffadd8), c.r[4]);
                c.lr = 0x8020c9e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020c9e8;
            }
            0x8020c9e8 => { ins += 1;
                pc = 0x8020c9f4; continue;
                pc = 0x8020c9ec;
            }
            0x8020c9ec => { ins += 2;
                c.r[0] = 0x0;
                c.sth(c.r[13].wrapping_add(0xffffadd8), c.r[0]);
                pc = 0x8020c9f4;
            }
            0x8020c9f4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020ca10; continue; }
                pc = 0x8020c9fc;
            }
            0x8020c9fc => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x5f;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020ca10; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ca10;
            }
            0x8020ca10 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020ca24; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ca24;
            }
            0x8020ca24 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020ca84; continue; }
                pc = 0x8020ca30;
            }
            0x8020ca30 => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020ca44; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ca44;
            }
            0x8020ca44 => { ins += 1;
                pc = 0x8020ca84; continue;
                pc = 0x8020ca48;
            }
            0x8020ca48 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x54;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020ca5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ca5c;
            }
            0x8020ca5c => { ins += 5;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x55;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020ca70; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ca70;
            }
            0x8020ca70 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x56;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020ca84; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ddd28(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ca84;
            }
            0x8020ca84 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020c840: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaisiNyuujouPokemon (0x8020ca98, 1352 bytes)
#[inline(never)]
pub fn f_8020ca98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ca98) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ca98); p = true; }
    }
    f_8020ca98_b(c, 0x8020ca98);
    if p { crate::prof::exit(c, 0x8020ca98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ca98_at(c: &mut Cpu, pc: u32) { f_8020ca98_b(c, pc) }
#[inline(never)]
fn f_8020ca98_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ca98;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ca98, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ca98 => { ins += 15;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 19..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020caac; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d920(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020caac;
            }
            0x8020caac => { ins += 1;
                c.lr = 0x8020cab0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d814(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cab0;
            }
            0x8020cab0 => { ins += 6;
                c.r[21] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8020cac8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cac8;
            }
            0x8020cac8 => { ins += 6;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x18;
                c.r[6] = 0x0;
                c.lr = 0x8020cae0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cae0;
            }
            0x8020cae0 => { ins += 6;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                c.lr = 0x8020caf8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020caf8;
            }
            0x8020caf8 => { ins += 5;
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[21] | c.r[21];
                c.r[5] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020cb0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cb0c;
            }
            0x8020cb0c => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x8020cb14; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7404(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cb14;
            }
            0x8020cb14 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020cb24; continue; }
                pc = 0x8020cb1c;
            }
            0x8020cb1c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020cfcc; continue;
                pc = 0x8020cb24;
            }
            0x8020cb24 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020cb2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7388(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cb2c;
            }
            0x8020cb2c => { ins += 3;
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                c.r[23] = 0x0;
                pc = 0x8020cc6c; continue;
                pc = 0x8020cb38;
            }
            0x8020cb38 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8020cb44; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7258(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cb44;
            }
            0x8020cb44 => { ins += 2;
                c.r[25] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[25]);
                if cr_bit(cr, 2) { pc = 0x8020cc68; continue; }
                pc = 0x8020cb4c;
            }
            0x8020cb4c => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020cb5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cb5c;
            }
            0x8020cb5c => { ins += 2;
                c.r[21] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[21]);
                if cr_bit(cr, 2) { pc = 0x8020cc68; continue; }
                pc = 0x8020cb64;
            }
            0x8020cb64 => { ins += 2;
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8020cb6c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f98bc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cb6c;
            }
            0x8020cb6c => { ins += 3;
                c.r[24] = 0x0;
                c.r[22] = 0x0;
                pc = 0x8020cc5c; continue;
                pc = 0x8020cb78;
            }
            0x8020cb78 => { ins += 3;
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if !cr_bit(cr, 0) { pc = 0x8020cc68; continue; }
                pc = 0x8020cb84;
            }
            0x8020cb84 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                if !cr_bit(cr, 0) { pc = 0x8020cc68; continue; }
                pc = 0x8020cb8c;
            }
            0x8020cb8c => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[6] = c.r[22] | c.r[22];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x8020cba0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cba0;
            }
            0x8020cba0 => { ins += 2;
                c.r[20] = c.r[3] | c.r[3];
                c.lr = 0x8020cba8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80206608(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cba8;
            }
            0x8020cba8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020cc58; continue; }
                pc = 0x8020cbb0;
            }
            0x8020cbb0 => { ins += 4;
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8020cbc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8010ae2c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cbc0;
            }
            0x8020cbc0 => { ins += 2;
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x8020cbc8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205be8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cbc8;
            }
            0x8020cbc8 => { ins += 1;
                c.lr = 0x8020cbcc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80121c18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cbcc;
            }
            0x8020cbcc => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[19] = c.r[0] | c.r[0];
                c.r[6] = c.r[22] | c.r[22];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x8020cbe8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cbe8;
            }
            0x8020cbe8 => { ins += 4;
                c.r[4] = c.r[20] | c.r[20];
                c.r[26] = c.r[3] | c.r[3];
                c.r[5] = c.r[19] | c.r[19];
                c.lr = 0x8020cbf8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802068c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cbf8;
            }
            0x8020cbf8 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[24] = c.r[24].wrapping_add(0x1);
                c.lr = 0x8020cc04; { c.steps = st; c.cycles = ins; c.cr = cr; f_80208028(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cc04;
            }
            0x8020cc04 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.lr = 0x8020cc18; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cc18;
            }
            0x8020cc18 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020cc4c; continue; }
                pc = 0x8020cc24;
            }
            0x8020cc24 => { ins += 2;
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020cc2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80204a10(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cc2c;
            }
            0x8020cc2c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020cc4c; continue; }
                pc = 0x8020cc34;
            }
            0x8020cc34 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.lr = 0x8020cc40; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205ad4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cc40;
            }
            0x8020cc40 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.lr = 0x8020cc4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205a7c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cc4c;
            }
            0x8020cc4c => { ins += 3;
                c.r[3] = c.r[21] | c.r[21];
                c.r[4] = c.r[19] | c.r[19];
                c.lr = 0x8020cc58; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c3e3c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cc58;
            }
            0x8020cc58 => { ins += 1;
                c.r[22] = c.r[22].wrapping_add(0x1);
                pc = 0x8020cc5c;
            }
            0x8020cc5c => { ins += 3;
                c.r[0] = c.r[22].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x8020cb78; continue; }
                pc = 0x8020cc68;
            }
            0x8020cc68 => { ins += 1;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020cc6c;
            }
            0x8020cc6c => { ins += 3;
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x8020cb38; continue; }
                pc = 0x8020cc78;
            }
            0x8020cc78 => { ins += 2;
                c.r[21] = 0x0;
                pc = 0x8020cd08; continue;
                pc = 0x8020cc80;
            }
            0x8020cc80 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8020cc8c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7258(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cc8c;
            }
            0x8020cc8c => { ins += 2;
                c.r[22] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[22]);
                if cr_bit(cr, 2) { pc = 0x8020cd04; continue; }
                pc = 0x8020cc94;
            }
            0x8020cc94 => { ins += 2;
                c.r[23] = 0x0;
                pc = 0x8020ccb4; continue;
                pc = 0x8020cc9c;
            }
            0x8020cc9c => { ins += 3;
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8020cca8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cca8;
            }
            0x8020cca8 => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[26]);
                if !cr_bit(cr, 2) { pc = 0x8020ccc0; continue; }
                pc = 0x8020ccb0;
            }
            0x8020ccb0 => { ins += 1;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020ccb4;
            }
            0x8020ccb4 => { ins += 3;
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x8020cc9c; continue; }
                pc = 0x8020ccc0;
            }
            0x8020ccc0 => { ins += 4;
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x0;
                c.lr = 0x8020ccd0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fbc20(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ccd0;
            }
            0x8020ccd0 => { ins += 2;
                c.r[23] = 0x0;
                pc = 0x8020ccf8; continue;
                pc = 0x8020ccd8;
            }
            0x8020ccd8 => { ins += 3;
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8020cce4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cce4;
            }
            0x8020cce4 => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[26]);
                if cr_bit(cr, 2) { pc = 0x8020ccf4; continue; }
                pc = 0x8020ccec;
            }
            0x8020ccec => { ins += 2;
                c.r[4] = 0x0;
                c.lr = 0x8020ccf4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80208c18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ccf4;
            }
            0x8020ccf4 => { ins += 1;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020ccf8;
            }
            0x8020ccf8 => { ins += 3;
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x8020ccd8; continue; }
                pc = 0x8020cd04;
            }
            0x8020cd04 => { ins += 1;
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020cd08;
            }
            0x8020cd08 => { ins += 3;
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x8020cc80; continue; }
                pc = 0x8020cd14;
            }
            0x8020cd14 => { ins += 2;
                c.r[25] = 0x0;
                pc = 0x8020cf1c; continue;
                pc = 0x8020cd1c;
            }
            0x8020cd1c => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[25] | c.r[25];
                c.lr = 0x8020cd28; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7258(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cd28;
            }
            0x8020cd28 => { ins += 2;
                c.r[24] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[24]);
                if cr_bit(cr, 2) { pc = 0x8020cf18; continue; }
                pc = 0x8020cd30;
            }
            0x8020cd30 => { ins += 1;
                c.lr = 0x8020cd34; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f98bc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cd34;
            }
            0x8020cd34 => { ins += 3;
                c.r[19] = c.r[3].rotate_left(0) & 0xff;
                c.r[21] = 0x0;
                pc = 0x8020cd78; continue;
                pc = 0x8020cd40;
            }
            0x8020cd40 => { ins += 3;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8020cd4c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cd4c;
            }
            0x8020cd4c => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[26]);
                if cr_bit(cr, 2) { pc = 0x8020cd74; continue; }
                pc = 0x8020cd54;
            }
            0x8020cd54 => { ins += 8;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = c.r[19] | c.r[19];
                c.r[7] = c.r[25] | c.r[25];
                c.r[8] = c.r[21] | c.r[21];
                c.r[9] = 0x0;
                c.lr = 0x8020cd74; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020cfe0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cd74;
            }
            0x8020cd74 => { ins += 1;
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020cd78;
            }
            0x8020cd78 => { ins += 3;
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x8020cd40; continue; }
                pc = 0x8020cd84;
            }
            0x8020cd84 => { ins += 2;
                c.r[21] = 0x0;
                pc = 0x8020cda4; continue;
                pc = 0x8020cd8c;
            }
            0x8020cd8c => { ins += 3;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8020cd98; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cd98;
            }
            0x8020cd98 => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[26]);
                if !cr_bit(cr, 2) { pc = 0x8020cdb0; continue; }
                pc = 0x8020cda0;
            }
            0x8020cda0 => { ins += 1;
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020cda4;
            }
            0x8020cda4 => { ins += 3;
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x8020cd8c; continue; }
                pc = 0x8020cdb0;
            }
            0x8020cdb0 => { ins += 1;
                c.lr = 0x8020cdb4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c3430(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cdb4;
            }
            0x8020cdb4 => { ins += 4;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x1;
                c.lr = 0x8020cdc4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fbc20(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cdc4;
            }
            0x8020cdc4 => { ins += 2;
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020cdcc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8000(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cdcc;
            }
            0x8020cdcc => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x22;
                c.lr = 0x8020cdd8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cdd8;
            }
            0x8020cdd8 => { ins += 2;
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020cde0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8100(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cde0;
            }
            0x8020cde0 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x23;
                c.lr = 0x8020cdec; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cdec;
            }
            0x8020cdec => { ins += 2;
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020cdf4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8100(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cdf4;
            }
            0x8020cdf4 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x25;
                c.lr = 0x8020ce00; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ce00;
            }
            0x8020ce00 => { ins += 8;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = c.r[19] | c.r[19];
                c.r[7] = c.r[25] | c.r[25];
                c.r[8] = c.r[21] | c.r[21];
                c.r[9] = 0x1;
                c.lr = 0x8020ce20; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020cfe0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ce20;
            }
            0x8020ce20 => { ins += 4;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x2;
                c.lr = 0x8020ce30; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fbc20(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ce30;
            }
            0x8020ce30 => { ins += 2;
                c.r[21] = 0x0;
                pc = 0x8020cf0c; continue;
                pc = 0x8020ce38;
            }
            0x8020ce38 => { ins += 3;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8020ce44; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ce44;
            }
            0x8020ce44 => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[26]);
                if cr_bit(cr, 2) { pc = 0x8020cf08; continue; }
                pc = 0x8020ce4c;
            }
            0x8020ce4c => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020ce60; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ce60;
            }
            0x8020ce60 => { ins += 7;
                c.r[20] = c.r[3] | c.r[3];
                c.r[7] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020ce7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ce7c;
            }
            0x8020ce7c => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x1;
                c.lr = 0x8020ce88; { c.steps = st; c.cycles = ins; c.cr = cr; f_80208c18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ce88;
            }
            0x8020ce88 => { ins += 2;
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020ce90; { c.steps = st; c.cycles = ins; c.cr = cr; f_80204a10(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ce90;
            }
            0x8020ce90 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020ceac; continue; }
                pc = 0x8020ce98;
            }
            0x8020ce98 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.lr = 0x8020cea8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80265598(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cea8;
            }
            0x8020cea8 => { ins += 1;
                pc = 0x8020cebc; continue;
                pc = 0x8020ceac;
            }
            0x8020ceac => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.lr = 0x8020cebc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80265598(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cebc;
            }
            0x8020cebc => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x2;
                c.lr = 0x8020cec8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80208c18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cec8;
            }
            0x8020cec8 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x3;
                c.lr = 0x8020ced4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80208c18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ced4;
            }
            0x8020ced4 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x4;
                c.lr = 0x8020cee0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80208c18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cee0;
            }
            0x8020cee0 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.lr = 0x8020cef0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026532c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cef0;
            }
            0x8020cef0 => { ins += 6;
                c.r[7] = c.r[20] | c.r[20];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020cf08; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cf08;
            }
            0x8020cf08 => { ins += 1;
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020cf0c;
            }
            0x8020cf0c => { ins += 3;
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x8020ce38; continue; }
                pc = 0x8020cf18;
            }
            0x8020cf18 => { ins += 1;
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x8020cf1c;
            }
            0x8020cf1c => { ins += 3;
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x8020cd1c; continue; }
                pc = 0x8020cf28;
            }
            0x8020cf28 => { ins += 1;
                c.lr = 0x8020cf2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8026246c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cf2c;
            }
            0x8020cf2c => { ins += 2;
                c.r[21] = 0x0;
                pc = 0x8020cfbc; continue;
                pc = 0x8020cf34;
            }
            0x8020cf34 => { ins += 3;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8020cf40; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7258(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cf40;
            }
            0x8020cf40 => { ins += 2;
                c.r[22] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[22]);
                if cr_bit(cr, 2) { pc = 0x8020cfb8; continue; }
                pc = 0x8020cf48;
            }
            0x8020cf48 => { ins += 2;
                c.r[23] = 0x0;
                pc = 0x8020cf68; continue;
                pc = 0x8020cf50;
            }
            0x8020cf50 => { ins += 3;
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8020cf5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cf5c;
            }
            0x8020cf5c => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[26]);
                if !cr_bit(cr, 2) { pc = 0x8020cf74; continue; }
                pc = 0x8020cf64;
            }
            0x8020cf64 => { ins += 1;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020cf68;
            }
            0x8020cf68 => { ins += 3;
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x8020cf50; continue; }
                pc = 0x8020cf74;
            }
            0x8020cf74 => { ins += 4;
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x3;
                c.lr = 0x8020cf84; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fbc20(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cf84;
            }
            0x8020cf84 => { ins += 2;
                c.r[23] = 0x0;
                pc = 0x8020cfac; continue;
                pc = 0x8020cf8c;
            }
            0x8020cf8c => { ins += 3;
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8020cf98; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f981c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cf98;
            }
            0x8020cf98 => { ins += 2;
                c.r[26] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[26]);
                if cr_bit(cr, 2) { pc = 0x8020cfa8; continue; }
                pc = 0x8020cfa0;
            }
            0x8020cfa0 => { ins += 2;
                c.r[4] = 0x5;
                c.lr = 0x8020cfa8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80208c18(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020cfa8;
            }
            0x8020cfa8 => { ins += 1;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020cfac;
            }
            0x8020cfac => { ins += 3;
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if cr_bit(cr, 0) { pc = 0x8020cf8c; continue; }
                pc = 0x8020cfb8;
            }
            0x8020cfb8 => { ins += 1;
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020cfbc;
            }
            0x8020cfbc => { ins += 3;
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x8020cf34; continue; }
                pc = 0x8020cfc8;
            }
            0x8020cfc8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8020cfcc;
            }
            0x8020cfcc => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 19..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ca98: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowKaisiNyuujouPokemonSubAppearMsg__FP13FIGHT_TRAINERP15FightOutPokemonUsUsUsUsUc (0x8020cfe0, 540 bytes)
#[inline(never)]
pub fn f_8020cfe0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020cfe0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020cfe0); p = true; }
    }
    f_8020cfe0_b(c, 0x8020cfe0);
    if p { crate::prof::exit(c, 0x8020cfe0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020cfe0_at(c: &mut Cpu, pc: u32) { f_8020cfe0_b(c, pc) }
#[inline(never)]
fn f_8020cfe0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020cfe0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020cfe0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020cfe0 => { ins += 22;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[26] = c.r[5] | c.r[5];
                c.r[27] = c.r[6] | c.r[6];
                c.r[28] = c.r[8] | c.r[8];
                c.r[31] = c.r[9] | c.r[9];
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8020d010; { c.steps = st; c.cycles = ins; c.cr = cr; f_80204a10(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d010;
            }
            0x8020d010 => { ins += 6;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x0;
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[30] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.lr = 0x8020d028; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f18dc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d028;
            }
            0x8020d028 => { ins += 6;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[29] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.lr = 0x8020d040; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8000(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d040;
            }
            0x8020d040 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d054; continue; }
                pc = 0x8020d048;
            }
            0x8020d048 => { ins += 2;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020d054; continue; }
                pc = 0x8020d050;
            }
            0x8020d050 => { ins += 1;
                c.r[29] = 0x1;
                pc = 0x8020d054;
            }
            0x8020d054 => { ins += 2;
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8020d05c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80205b8c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d05c;
            }
            0x8020d05c => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x77;
                c.r[6] = 0x0;
                c.lr = 0x8020d06c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012640c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d06c;
            }
            0x8020d06c => { ins += 3;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[31] = c.r[3] | c.r[3];
                if !cr_bit(cr, 2) { pc = 0x8020d150; continue; }
                pc = 0x8020d078;
            }
            0x8020d078 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d0c4; continue; }
                pc = 0x8020d084;
            }
            0x8020d084 => { ins += 2;
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020d0a8; continue; }
                pc = 0x8020d08c;
            }
            0x8020d08c => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x14;
                c.lr = 0x8020d098; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d098;
            }
            0x8020d098 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x16;
                c.lr = 0x8020d0a4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d0a4;
            }
            0x8020d0a4 => { ins += 1;
                pc = 0x8020d1e8; continue;
                pc = 0x8020d0a8;
            }
            0x8020d0a8 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x15;
                c.lr = 0x8020d0b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d0b4;
            }
            0x8020d0b4 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x17;
                c.lr = 0x8020d0c0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d0c0;
            }
            0x8020d0c0 => { ins += 1;
                pc = 0x8020d1e8; continue;
                pc = 0x8020d0c4;
            }
            0x8020d0c4 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d110; continue; }
                pc = 0x8020d0d0;
            }
            0x8020d0d0 => { ins += 2;
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020d0f4; continue; }
                pc = 0x8020d0d8;
            }
            0x8020d0d8 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x15;
                c.lr = 0x8020d0e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d0e4;
            }
            0x8020d0e4 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x17;
                c.lr = 0x8020d0f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d0f0;
            }
            0x8020d0f0 => { ins += 1;
                pc = 0x8020d1e8; continue;
                pc = 0x8020d0f4;
            }
            0x8020d0f4 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x14;
                c.lr = 0x8020d100; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d100;
            }
            0x8020d100 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x16;
                c.lr = 0x8020d10c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d10c;
            }
            0x8020d10c => { ins += 1;
                pc = 0x8020d1e8; continue;
                pc = 0x8020d110;
            }
            0x8020d110 => { ins += 2;
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020d134; continue; }
                pc = 0x8020d118;
            }
            0x8020d118 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x14;
                c.lr = 0x8020d124; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d124;
            }
            0x8020d124 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x16;
                c.lr = 0x8020d130; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d130;
            }
            0x8020d130 => { ins += 1;
                pc = 0x8020d1e8; continue;
                pc = 0x8020d134;
            }
            0x8020d134 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x15;
                c.lr = 0x8020d140; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d140;
            }
            0x8020d140 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x17;
                c.lr = 0x8020d14c; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d14c;
            }
            0x8020d14c => { ins += 1;
                pc = 0x8020d1e8; continue;
                pc = 0x8020d150;
            }
            0x8020d150 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d1e8; continue; }
                pc = 0x8020d158;
            }
            0x8020d158 => { ins += 3;
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 1) { pc = 0x8020d1a0; continue; }
                pc = 0x8020d164;
            }
            0x8020d164 => { ins += 3;
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 1) { pc = 0x8020d1a0; continue; }
                pc = 0x8020d170;
            }
            0x8020d170 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d184; continue; }
                pc = 0x8020d17c;
            }
            0x8020d17c => { ins += 2;
                c.r[3] = 0x7674;
                pc = 0x8020d1e4; continue;
                pc = 0x8020d184;
            }
            0x8020d184 => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d198; continue; }
                pc = 0x8020d190;
            }
            0x8020d190 => { ins += 2;
                c.r[3] = 0x7679;
                pc = 0x8020d1e4; continue;
                pc = 0x8020d198;
            }
            0x8020d198 => { ins += 2;
                c.r[3] = 0x7671;
                pc = 0x8020d1e4; continue;
                pc = 0x8020d1a0;
            }
            0x8020d1a0 => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x14;
                c.lr = 0x8020d1ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d1ac;
            }
            0x8020d1ac => { ins += 3;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x16;
                c.lr = 0x8020d1b8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d1b8;
            }
            0x8020d1b8 => { ins += 3;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d1cc; continue; }
                pc = 0x8020d1c4;
            }
            0x8020d1c4 => { ins += 2;
                c.r[3] = 0x7673;
                pc = 0x8020d1e4; continue;
                pc = 0x8020d1cc;
            }
            0x8020d1cc => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d1e0; continue; }
                pc = 0x8020d1d8;
            }
            0x8020d1d8 => { ins += 2;
                c.r[3] = 0x7678;
                pc = 0x8020d1e4; continue;
                pc = 0x8020d1e0;
            }
            0x8020d1e0 => { ins += 1;
                c.r[3] = 0x7670;
                pc = 0x8020d1e4;
            }
            0x8020d1e4 => { ins += 1;
                c.lr = 0x8020d1e8; { c.steps = st; c.cycles = ins; c.cr = cr; f_802624cc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d1e8;
            }
            0x8020d1e8 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020cfe0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaisiNyuujouTrainer (0x8020d1fc, 1180 bytes)
#[inline(never)]
pub fn f_8020d1fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d1fc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d1fc); p = true; }
    }
    f_8020d1fc_b(c, 0x8020d1fc);
    if p { crate::prof::exit(c, 0x8020d1fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d1fc_at(c: &mut Cpu, pc: u32) { f_8020d1fc_b(c, pc) }
#[inline(never)]
fn f_8020d1fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d1fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d1fc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d1fc => { ins += 15;
                { let ea = c.r[1].wrapping_add(0xfffff4a0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xb64), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xb28); for n in 18..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020d210; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d920(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d210;
            }
            0x8020d210 => { ins += 1;
                c.lr = 0x8020d214; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d814(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d214;
            }
            0x8020d214 => { ins += 6;
                c.r[18] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0xd;
                c.r[6] = 0x0;
                c.lr = 0x8020d22c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d22c;
            }
            0x8020d22c => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8020d234; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d234;
            }
            0x8020d234 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[23] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8020d250; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d250;
            }
            0x8020d250 => { ins += 6;
                c.r[19] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                c.lr = 0x8020d268; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d268;
            }
            0x8020d268 => { ins += 6;
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x17;
                c.r[6] = 0x0;
                c.lr = 0x8020d280; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d280;
            }
            0x8020d280 => { ins += 6;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x18;
                c.r[6] = 0x0;
                c.lr = 0x8020d298; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d298;
            }
            0x8020d298 => { ins += 5;
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[18] | c.r[18];
                c.r[5] = c.r[19] | c.r[19];
                c.r[4] = 0x0;
                c.lr = 0x8020d2ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d2ac;
            }
            0x8020d2ac => { ins += 2;
                c.r[25] = c.r[3] | c.r[3];
                c.lr = 0x8020d2b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7404(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d2b4;
            }
            0x8020d2b4 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020d2c4; continue; }
                pc = 0x8020d2bc;
            }
            0x8020d2bc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020d684; continue;
                pc = 0x8020d2c4;
            }
            0x8020d2c4 => { ins += 8;
                c.r[5] = c.addc(!c.r[18], 0x4, 1);
                c.r[3] = c.r[25] | c.r[25];
                c.r[0] = c.addc(c.r[5], 0xffffffff, 0);
                c.r[4] = 0x0;
                { let ci = c.ca(); c.r[18] = c.addc(!c.r[0], c.r[5], ci); }
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x8020d2e4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d2e4;
            }
            0x8020d2e4 => { ins += 9;
                c.r[0] = c.r[18].rotate_left(0) & 0xffff;
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[29] = c.r[0].wrapping_mul(c.r[28]);
                c.r[24] = 0x0;
                pc = 0x8020d5ec; continue;
                pc = 0x8020d2f8;
            }
            0x8020d2f8 => { ins += 5;
                c.r[3] = c.r[25] | c.r[25];
                c.r[6] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x7;
                c.lr = 0x8020d30c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f76b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d30c;
            }
            0x8020d30c => { ins += 7;
                c.r[0] = c.r[24].wrapping_add(c.r[29]);
                c.r[4] = c.r[3] | c.r[3];
                c.r[20] = c.r[0].rotate_left(0) & 0xff;
                c.r[3] = c.r[23] | c.r[23];
                c.r[26] = c.r[4] | c.r[4];
                c.r[4] = c.r[20] | c.r[20];
                c.lr = 0x8020d328; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e068(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d328;
            }
            0x8020d328 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.r[18] = c.r[0] | c.r[0];
                c.r[4] = c.r[20] | c.r[20];
                c.lr = 0x8020d33c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e020(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d33c;
            }
            0x8020d33c => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[18] | c.r[18];
                c.r[19] = c.r[0] | c.r[0];
                c.r[4] = c.r[19] | c.r[19];
                c.lr = 0x8020d350; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fa4b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d350;
            }
            0x8020d350 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020d5e8; continue; }
                pc = 0x8020d358;
            }
            0x8020d358 => { ins += 1;
                c.lr = 0x8020d35c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8006b57c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d35c;
            }
            0x8020d35c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d380; continue; }
                pc = 0x8020d368;
            }
            0x8020d368 => { ins += 2;
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x8020d370; { c.steps = st; c.cycles = ins; c.cr = cr; f_8006b0f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d370;
            }
            0x8020d370 => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.lr = 0x8020d37c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012ac64(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d37c;
            }
            0x8020d37c => { ins += 1;
                pc = 0x8020d390; continue;
                pc = 0x8020d380;
            }
            0x8020d380 => { ins += 4;
                c.r[3] = c.r[18] | c.r[18];
                c.r[4] = c.r[19] | c.r[19];
                c.r[5] = c.r[1].wrapping_add(0xc);
                c.lr = 0x8020d390; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f9cbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d390;
            }
            0x8020d390 => { ins += 2;
                c.r[3] = c.r[18] | c.r[18];
                c.lr = 0x8020d398; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f8fd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d398;
            }
            0x8020d398 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[7] = c.r[0] | c.r[0];
                c.r[5] = c.r[18] | c.r[18];
                c.r[6] = c.r[19] | c.r[19];
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.lr = 0x8020d3b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fa6d8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d3b4;
            }
            0x8020d3b4 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x8020d3c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d3c8;
            }
            0x8020d3c8 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[22] = c.r[0] | c.r[0];
                c.lr = 0x8020d3d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fa634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d3d8;
            }
            0x8020d3d8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020d5e8; continue; }
                pc = 0x8020d3e0;
            }
            0x8020d3e0 => { ins += 4;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8020d3f0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f99c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d3f0;
            }
            0x8020d3f0 => { ins += 3;
                c.r[21] = 0x0;
                c.r[19] = 0x0;
                pc = 0x8020d4e8; continue;
                pc = 0x8020d3fc;
            }
            0x8020d3fc => { ins += 3;
                c.r[0] = c.r[21] as u8 as i8 as i32 as u32;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], c.r[31]);
                if !cr_bit(cr, 0) { pc = 0x8020d4f4; continue; }
                pc = 0x8020d408;
            }
            0x8020d408 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], c.r[27]);
                if !cr_bit(cr, 0) { pc = 0x8020d4f4; continue; }
                pc = 0x8020d410;
            }
            0x8020d410 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 0) { pc = 0x8020d4f4; continue; }
                pc = 0x8020d418;
            }
            0x8020d418 => { ins += 4;
                c.r[3] = c.r[22] | c.r[22];
                c.r[5] = c.r[19] | c.r[19];
                c.r[4] = 0x3;
                c.lr = 0x8020d428; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012a5b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d428;
            }
            0x8020d428 => { ins += 2;
                c.r[18] = c.r[3] | c.r[3];
                c.lr = 0x8020d430; { c.steps = st; c.cycles = ins; c.cr = cr; f_801233f4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d430;
            }
            0x8020d430 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020d4e4; continue; }
                pc = 0x8020d438;
            }
            0x8020d438 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[18] | c.r[18];
                c.lr = 0x8020d444; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f9930(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d444;
            }
            0x8020d444 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d4e4; continue; }
                pc = 0x8020d44c;
            }
            0x8020d44c => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[21] as u8 as i8 as i32 as u32;
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x8020d460; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d460;
            }
            0x8020d460 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[20] = c.r[0] | c.r[0];
                c.lr = 0x8020d470; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4804(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d470;
            }
            0x8020d470 => { ins += 4;
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = c.r[18] | c.r[18];
                c.lr = 0x8020d480; { c.steps = st; c.cycles = ins; c.cr = cr; f_80206aec(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d480;
            }
            0x8020d480 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x8020d494; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d494;
            }
            0x8020d494 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d4e0; continue; }
                pc = 0x8020d4a0;
            }
            0x8020d4a0 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x2e;
                c.r[6] = 0x0;
                c.lr = 0x8020d4b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d4b4;
            }
            0x8020d4b4 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d4e0; continue; }
                pc = 0x8020d4c0;
            }
            0x8020d4c0 => { ins += 2;
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020d4c8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb8f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d4c8;
            }
            0x8020d4c8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d4e0; continue; }
                pc = 0x8020d4d4;
            }
            0x8020d4d4 => { ins += 3;
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = 0x3;
                c.lr = 0x8020d4e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_802032e4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d4e0;
            }
            0x8020d4e0 => { ins += 1;
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020d4e4;
            }
            0x8020d4e4 => { ins += 1;
                c.r[19] = c.r[19].wrapping_add(0x1);
                pc = 0x8020d4e8;
            }
            0x8020d4e8 => { ins += 3;
                c.r[0] = c.r[19].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x8020d3fc; continue; }
                pc = 0x8020d4f4;
            }
            0x8020d4f4 => { ins += 2;
                c.r[20] = 0x0;
                pc = 0x8020d5dc; continue;
                pc = 0x8020d4fc;
            }
            0x8020d4fc => { ins += 3;
                c.r[0] = c.r[21] as u8 as i8 as i32 as u32;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], c.r[27]);
                if !cr_bit(cr, 0) { pc = 0x8020d5e8; continue; }
                pc = 0x8020d508;
            }
            0x8020d508 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x6);
                if !cr_bit(cr, 0) { pc = 0x8020d5e8; continue; }
                pc = 0x8020d510;
            }
            0x8020d510 => { ins += 4;
                c.r[3] = c.r[22] | c.r[22];
                c.r[5] = c.r[20] | c.r[20];
                c.r[4] = 0x3;
                c.lr = 0x8020d520; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012a5b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d520;
            }
            0x8020d520 => { ins += 2;
                c.r[18] = c.r[3] | c.r[3];
                c.lr = 0x8020d528; { c.steps = st; c.cycles = ins; c.cr = cr; f_80123fbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d528;
            }
            0x8020d528 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020d5d8; continue; }
                pc = 0x8020d530;
            }
            0x8020d530 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[18] | c.r[18];
                c.lr = 0x8020d53c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f9930(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d53c;
            }
            0x8020d53c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d5d8; continue; }
                pc = 0x8020d544;
            }
            0x8020d544 => { ins += 5;
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[21] as u8 as i8 as i32 as u32;
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x8020d558; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d558;
            }
            0x8020d558 => { ins += 3;
                c.r[19] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020d564; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4804(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d564;
            }
            0x8020d564 => { ins += 4;
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = c.r[18] | c.r[18];
                c.lr = 0x8020d574; { c.steps = st; c.cycles = ins; c.cr = cr; f_80206aec(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d574;
            }
            0x8020d574 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x8020d588; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d588;
            }
            0x8020d588 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d5d4; continue; }
                pc = 0x8020d594;
            }
            0x8020d594 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x2e;
                c.r[6] = 0x0;
                c.lr = 0x8020d5a8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d5a8;
            }
            0x8020d5a8 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d5d4; continue; }
                pc = 0x8020d5b4;
            }
            0x8020d5b4 => { ins += 2;
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020d5bc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb8f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d5bc;
            }
            0x8020d5bc => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020d5d4; continue; }
                pc = 0x8020d5c8;
            }
            0x8020d5c8 => { ins += 3;
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x3;
                c.lr = 0x8020d5d4; { c.steps = st; c.cycles = ins; c.cr = cr; f_802032e4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d5d4;
            }
            0x8020d5d4 => { ins += 1;
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020d5d8;
            }
            0x8020d5d8 => { ins += 1;
                c.r[20] = c.r[20].wrapping_add(0x1);
                pc = 0x8020d5dc;
            }
            0x8020d5dc => { ins += 3;
                c.r[0] = c.r[20].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                if cr_bit(cr, 0) { pc = 0x8020d4fc; continue; }
                pc = 0x8020d5e8;
            }
            0x8020d5e8 => { ins += 1;
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x8020d5ec;
            }
            0x8020d5ec => { ins += 3;
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x8020d2f8; continue; }
                pc = 0x8020d5f8;
            }
            0x8020d5f8 => { ins += 2;
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8020d600; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7388(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d600;
            }
            0x8020d600 => { ins += 3;
                c.r[18] = c.r[3].rotate_left(0) & 0xff;
                c.r[20] = 0x0;
                pc = 0x8020d674; continue;
                pc = 0x8020d60c;
            }
            0x8020d60c => { ins += 3;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[20] | c.r[20];
                c.lr = 0x8020d618; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7258(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d618;
            }
            0x8020d618 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020d670; continue; }
                pc = 0x8020d620;
            }
            0x8020d620 => { ins += 4;
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020d630; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d630;
            }
            0x8020d630 => { ins += 2;
                c.r[19] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[19]);
                if cr_bit(cr, 2) { pc = 0x8020d670; continue; }
                pc = 0x8020d638;
            }
            0x8020d638 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[18] | c.r[18];
                c.r[5] = c.r[20] | c.r[20];
                c.r[6] = c.r[1].wrapping_add(0x9);
                c.r[7] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8020d650; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f72b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d650;
            }
            0x8020d650 => { ins += 4;
                c.r[4] = c.lbz_t(ins, c.r[1].wrapping_add(0x9));
                c.r[3] = c.r[19] | c.r[19];
                c.r[5] = c.lbz_t(ins, c.r[1].wrapping_add(0x8));
                c.lr = 0x8020d660; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c3fbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d660;
            }
            0x8020d660 => { ins += 1;
                c.lr = 0x8020d664; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c3430(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d664;
            }
            0x8020d664 => { ins += 3;
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x1;
                c.lr = 0x8020d670; { c.steps = st; c.cycles = ins; c.cr = cr; f_801da4e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d670;
            }
            0x8020d670 => { ins += 1;
                c.r[20] = c.r[20].wrapping_add(0x1);
                pc = 0x8020d674;
            }
            0x8020d674 => { ins += 3;
                c.r[0] = c.r[20].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x8020d60c; continue; }
                pc = 0x8020d680;
            }
            0x8020d680 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8020d684;
            }
            0x8020d684 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xb28); for n in 18..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xb64));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xb60);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d1fc: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaijou (0x8020d698, 236 bytes)
#[inline(never)]
pub fn f_8020d698(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d698) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d698); p = true; }
    }
    f_8020d698_b(c, 0x8020d698);
    if p { crate::prof::exit(c, 0x8020d698); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d698_at(c: &mut Cpu, pc: u32) { f_8020d698_b(c, pc) }
#[inline(always)]
fn f_8020d698_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d698;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d698, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d698 => { ins += 15;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020d6ac; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef624(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d6ac;
            }
            0x8020d6ac => { ins += 2;
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x8020d6b4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d6b4;
            }
            0x8020d6b4 => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x8020d6d0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d6d0;
            }
            0x8020d6d0 => { ins += 2;
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8020d6d8; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4860(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d6d8;
            }
            0x8020d6d8 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020d6e0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f17b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d6e0;
            }
            0x8020d6e0 => { ins += 5;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8020d6f4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d6f4;
            }
            0x8020d6f4 => { ins += 3;
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020d700; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d700;
            }
            0x8020d700 => { ins += 5;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = 0x4;
                c.r[4] = 0x0;
                c.lr = 0x8020d714; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d714;
            }
            0x8020d714 => { ins += 6;
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x8020d72c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d72c;
            }
            0x8020d72c => { ins += 3;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020d738; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7480(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d738;
            }
            0x8020d738 => { ins += 4;
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = 0x5;
                c.r[4] = 0x0;
                c.lr = 0x8020d748; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d748;
            }
            0x8020d748 => { ins += 6;
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.r[6] = 0x1;
                c.lr = 0x8020d760; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d760;
            }
            0x8020d760 => { ins += 3;
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020d76c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f7480(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020d76c;
            }
            0x8020d76c => { ins += 17;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d698: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowNullFunc (0x8020d784, 8 bytes)
#[inline(never)]
pub fn f_8020d784(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d784) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d784); p = true; }
    }
    f_8020d784_b(c, 0x8020d784);
    if p { crate::prof::exit(c, 0x8020d784); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d784_at(c: &mut Cpu, pc: u32) { f_8020d784_b(c, pc) }
#[inline(always)]
fn f_8020d784_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d784;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d784, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d784 => { ins += 2;
                c.r[3] = 0x1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d784: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetFifoBanme (0x8020d78c, 16 bytes)
#[inline(never)]
pub fn f_8020d78c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d78c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d78c); p = true; }
    }
    f_8020d78c_b(c, 0x8020d78c);
    if p { crate::prof::exit(c, 0x8020d78c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d78c_at(c: &mut Cpu, pc: u32) { f_8020d78c_b(c, pc) }
#[inline(always)]
fn f_8020d78c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d78c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d78c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d78c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d794;
            }
            0x8020d794 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d78c: bad pc {:#x}", pc),
        }
    }
}

/// fightActionKindDataBiosGetDispFuncPtr (0x8020d79c, 24 bytes)
#[inline(never)]
pub fn f_8020d79c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d79c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d79c); p = true; }
    }
    f_8020d79c_b(c, 0x8020d79c);
    if p { crate::prof::exit(c, 0x8020d79c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d79c_at(c: &mut Cpu, pc: u32) { f_8020d79c_b(c, pc) }
#[inline(always)]
fn f_8020d79c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d79c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d79c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d79c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d7ac; continue; }
                pc = 0x8020d7a4;
            }
            0x8020d7a4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d7ac;
            }
            0x8020d7ac => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d79c: bad pc {:#x}", pc),
        }
    }
}

/// fightActionKindDataBiosGetFlowFuncPtr (0x8020d7b4, 24 bytes)
#[inline(never)]
pub fn f_8020d7b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d7b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d7b4); p = true; }
    }
    f_8020d7b4_b(c, 0x8020d7b4);
    if p { crate::prof::exit(c, 0x8020d7b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d7b4_at(c: &mut Cpu, pc: u32) { f_8020d7b4_b(c, pc) }
#[inline(always)]
fn f_8020d7b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d7b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d7b4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d7b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d7c4; continue; }
                pc = 0x8020d7bc;
            }
            0x8020d7bc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d7c4;
            }
            0x8020d7c4 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d7b4: bad pc {:#x}", pc),
        }
    }
}

/// fightActionKindDataBiosGetPri (0x8020d7cc, 28 bytes)
#[inline(never)]
pub fn f_8020d7cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d7cc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d7cc); p = true; }
    }
    f_8020d7cc_b(c, 0x8020d7cc);
    if p { crate::prof::exit(c, 0x8020d7cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d7cc_at(c: &mut Cpu, pc: u32) { f_8020d7cc_b(c, pc) }
#[inline(always)]
fn f_8020d7cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d7cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d7cc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d7cc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d7dc; continue; }
                pc = 0x8020d7d4;
            }
            0x8020d7d4 => { ins += 2;
                c.r[3] = 0xffffff80;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d7dc;
            }
            0x8020d7dc => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[3]);
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d7cc: bad pc {:#x}", pc),
        }
    }
}

/// fightActionKindDataBiosGetPtr (0x8020d7e8, 44 bytes)
#[inline(never)]
pub fn f_8020d7e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d7e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d7e8); p = true; }
    }
    f_8020d7e8_b(c, 0x8020d7e8);
    if p { crate::prof::exit(c, 0x8020d7e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d7e8_at(c: &mut Cpu, pc: u32) { f_8020d7e8_b(c, pc) }
#[inline(always)]
fn f_8020d7e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d7e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d7e8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d7e8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8528));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020d800; continue; }
                pc = 0x8020d7f8;
            }
            0x8020d7f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d800;
            }
            0x8020d800 => { ins += 7;
                c.r[4] = c.r[3].wrapping_mul(0xc);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5bb8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d7e8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDataBiosGetBuff (0x8020d814, 24 bytes)
#[inline(never)]
pub fn f_8020d814(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d814) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d814); p = true; }
    }
    f_8020d814_b(c, 0x8020d814);
    if p { crate::prof::exit(c, 0x8020d814); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d814_at(c: &mut Cpu, pc: u32) { f_8020d814_b(c, pc) }
#[inline(always)]
fn f_8020d814_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d814;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d814, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d814 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d824; continue; }
                pc = 0x8020d81c;
            }
            0x8020d81c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d824;
            }
            0x8020d824 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d814: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDataBiosGetKind (0x8020d82c, 24 bytes)
#[inline(never)]
pub fn f_8020d82c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d82c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d82c); p = true; }
    }
    f_8020d82c_b(c, 0x8020d82c);
    if p { crate::prof::exit(c, 0x8020d82c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d82c_at(c: &mut Cpu, pc: u32) { f_8020d82c_b(c, pc) }
#[inline(always)]
fn f_8020d82c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d82c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d82c, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d82c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d83c; continue; }
                pc = 0x8020d834;
            }
            0x8020d834 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d83c;
            }
            0x8020d83c => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d82c: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetDispBuff (0x8020d844, 36 bytes)
#[inline(never)]
pub fn f_8020d844(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d844) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d844); p = true; }
    }
    f_8020d844_b(c, 0x8020d844);
    if p { crate::prof::exit(c, 0x8020d844); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d844_at(c: &mut Cpu, pc: u32) { f_8020d844_b(c, pc) }
#[inline(always)]
fn f_8020d844_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d844;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d844, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d844 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d84c;
            }
            0x8020d84c => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 0) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d858;
            }
            0x8020d858 => { ins += 4;
                c.r[0] = c.r[4].rotate_left(2) & 0x3fffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[5]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d844: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetMotoFightActionDataPtr (0x8020d868, 16 bytes)
#[inline(never)]
pub fn f_8020d868(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d868) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d868); p = true; }
    }
    f_8020d868_b(c, 0x8020d868);
    if p { crate::prof::exit(c, 0x8020d868); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d868_at(c: &mut Cpu, pc: u32) { f_8020d868_b(c, pc) }
#[inline(always)]
fn f_8020d868_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d868;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d868, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d868 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d870;
            }
            0x8020d870 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x18), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d868: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetBuffDataId (0x8020d878, 16 bytes)
#[inline(never)]
pub fn f_8020d878(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d878) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d878); p = true; }
    }
    f_8020d878_b(c, 0x8020d878);
    if p { crate::prof::exit(c, 0x8020d878); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d878_at(c: &mut Cpu, pc: u32) { f_8020d878_b(c, pc) }
#[inline(always)]
fn f_8020d878_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d878;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d878, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d878 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d880;
            }
            0x8020d880 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d878: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetBuffDataPtr (0x8020d888, 16 bytes)
#[inline(never)]
pub fn f_8020d888(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d888) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d888); p = true; }
    }
    f_8020d888_b(c, 0x8020d888);
    if p { crate::prof::exit(c, 0x8020d888); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d888_at(c: &mut Cpu, pc: u32) { f_8020d888_b(c, pc) }
#[inline(always)]
fn f_8020d888_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d888;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d888, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d888 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d890;
            }
            0x8020d890 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d888: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetActorFightTargetPtr (0x8020d898, 16 bytes)
#[inline(never)]
pub fn f_8020d898(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d898) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d898); p = true; }
    }
    f_8020d898_b(c, 0x8020d898);
    if p { crate::prof::exit(c, 0x8020d898); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d898_at(c: &mut Cpu, pc: u32) { f_8020d898_b(c, pc) }
#[inline(always)]
fn f_8020d898_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d898;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d898, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d898 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d8a0;
            }
            0x8020d8a0 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x14), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d898: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetFightActionDataPtr (0x8020d8a8, 16 bytes)
#[inline(never)]
pub fn f_8020d8a8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d8a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d8a8); p = true; }
    }
    f_8020d8a8_b(c, 0x8020d8a8);
    if p { crate::prof::exit(c, 0x8020d8a8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d8a8_at(c: &mut Cpu, pc: u32) { f_8020d8a8_b(c, pc) }
#[inline(always)]
fn f_8020d8a8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d8a8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d8a8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d8a8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d8b0;
            }
            0x8020d8b0 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d8a8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetBuff (0x8020d8b8, 16 bytes)
#[inline(never)]
pub fn f_8020d8b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d8b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d8b8); p = true; }
    }
    f_8020d8b8_b(c, 0x8020d8b8);
    if p { crate::prof::exit(c, 0x8020d8b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d8b8_at(c: &mut Cpu, pc: u32) { f_8020d8b8_b(c, pc) }
#[inline(always)]
fn f_8020d8b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d8b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d8b8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d8b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d8c0;
            }
            0x8020d8c0 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d8b8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetKind (0x8020d8c8, 16 bytes)
#[inline(never)]
pub fn f_8020d8c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d8c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d8c8); p = true; }
    }
    f_8020d8c8_b(c, 0x8020d8c8);
    if p { crate::prof::exit(c, 0x8020d8c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d8c8_at(c: &mut Cpu, pc: u32) { f_8020d8c8_b(c, pc) }
#[inline(always)]
fn f_8020d8c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d8c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d8c8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d8c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d8d0;
            }
            0x8020d8d0 => { ins += 2;
                c.sth(c.r[3], c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d8c8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetBuffDataId (0x8020d8d8, 24 bytes)
#[inline(never)]
pub fn f_8020d8d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d8d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d8d8); p = true; }
    }
    f_8020d8d8_b(c, 0x8020d8d8);
    if p { crate::prof::exit(c, 0x8020d8d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d8d8_at(c: &mut Cpu, pc: u32) { f_8020d8d8_b(c, pc) }
#[inline(always)]
fn f_8020d8d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d8d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d8d8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d8d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d8e8; continue; }
                pc = 0x8020d8e0;
            }
            0x8020d8e0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d8e8;
            }
            0x8020d8e8 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x10));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d8d8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetBuffDataPtr (0x8020d8f0, 24 bytes)
#[inline(never)]
pub fn f_8020d8f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d8f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d8f0); p = true; }
    }
    f_8020d8f0_b(c, 0x8020d8f0);
    if p { crate::prof::exit(c, 0x8020d8f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d8f0_at(c: &mut Cpu, pc: u32) { f_8020d8f0_b(c, pc) }
#[inline(always)]
fn f_8020d8f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d8f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d8f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d8f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d900; continue; }
                pc = 0x8020d8f8;
            }
            0x8020d8f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d900;
            }
            0x8020d900 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d8f0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetActorFightTargetPtr (0x8020d908, 24 bytes)
#[inline(never)]
pub fn f_8020d908(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d908) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d908); p = true; }
    }
    f_8020d908_b(c, 0x8020d908);
    if p { crate::prof::exit(c, 0x8020d908); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d908_at(c: &mut Cpu, pc: u32) { f_8020d908_b(c, pc) }
#[inline(always)]
fn f_8020d908_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d908;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d908, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d908 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d918; continue; }
                pc = 0x8020d910;
            }
            0x8020d910 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d918;
            }
            0x8020d918 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x14));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d908: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetFightActionDataPtr (0x8020d920, 24 bytes)
#[inline(never)]
pub fn f_8020d920(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d920) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d920); p = true; }
    }
    f_8020d920_b(c, 0x8020d920);
    if p { crate::prof::exit(c, 0x8020d920); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d920_at(c: &mut Cpu, pc: u32) { f_8020d920_b(c, pc) }
#[inline(always)]
fn f_8020d920_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d920;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d920, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d920 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d930; continue; }
                pc = 0x8020d928;
            }
            0x8020d928 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d930;
            }
            0x8020d930 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d920: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetBuff (0x8020d938, 24 bytes)
#[inline(never)]
pub fn f_8020d938(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d938) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d938); p = true; }
    }
    f_8020d938_b(c, 0x8020d938);
    if p { crate::prof::exit(c, 0x8020d938); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d938_at(c: &mut Cpu, pc: u32) { f_8020d938_b(c, pc) }
#[inline(always)]
fn f_8020d938_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d938;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d938, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d938 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d948; continue; }
                pc = 0x8020d940;
            }
            0x8020d940 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d948;
            }
            0x8020d948 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d938: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetKind (0x8020d950, 24 bytes)
#[inline(never)]
pub fn f_8020d950(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d950) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d950); p = true; }
    }
    f_8020d950_b(c, 0x8020d950);
    if p { crate::prof::exit(c, 0x8020d950); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d950_at(c: &mut Cpu, pc: u32) { f_8020d950_b(c, pc) }
#[inline(always)]
fn f_8020d950_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d950;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d950, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d950 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d960; continue; }
                pc = 0x8020d958;
            }
            0x8020d958 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d960;
            }
            0x8020d960 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d950: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosCopy (0x8020d968, 56 bytes)
#[inline(never)]
pub fn f_8020d968(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d968) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d968); p = true; }
    }
    f_8020d968_b(c, 0x8020d968);
    if p { crate::prof::exit(c, 0x8020d968); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d968_at(c: &mut Cpu, pc: u32) { f_8020d968_b(c, pc) }
#[inline(always)]
fn f_8020d968_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d968;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d968, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d968 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d970;
            }
            0x8020d970 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d978;
            }
            0x8020d978 => { ins += 5;
                c.r[0] = 0x6;
                c.r[5] = c.r[3].wrapping_add(0xfffffffc);
                c.r[4] = c.r[4].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x8020d988;
            }
            0x8020d988 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8020d988; continue; }
                pc = 0x8020d99c;
            }
            0x8020d99c => { ins += 1;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d968: bad pc {:#x}", pc),
        }
    }
}

/// fightKoukaDataBiosGetKoukaDataId (0x8020d9a0, 24 bytes)
#[inline(never)]
pub fn f_8020d9a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d9a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d9a0); p = true; }
    }
    f_8020d9a0_b(c, 0x8020d9a0);
    if p { crate::prof::exit(c, 0x8020d9a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d9a0_at(c: &mut Cpu, pc: u32) { f_8020d9a0_b(c, pc) }
#[inline(always)]
fn f_8020d9a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d9a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d9a0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d9a0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d9b0; continue; }
                pc = 0x8020d9a8;
            }
            0x8020d9a8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d9b0;
            }
            0x8020d9b0 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d9a0: bad pc {:#x}", pc),
        }
    }
}

/// fightKoukaDataBiosGetFightTargetDataId (0x8020d9b8, 24 bytes)
#[inline(never)]
pub fn f_8020d9b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d9b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d9b8); p = true; }
    }
    f_8020d9b8_b(c, 0x8020d9b8);
    if p { crate::prof::exit(c, 0x8020d9b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d9b8_at(c: &mut Cpu, pc: u32) { f_8020d9b8_b(c, pc) }
#[inline(always)]
fn f_8020d9b8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d9b8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d9b8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d9b8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d9c8; continue; }
                pc = 0x8020d9c0;
            }
            0x8020d9c0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d9c8;
            }
            0x8020d9c8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d9b8: bad pc {:#x}", pc),
        }
    }
}

/// fightKoukaDataBiosGetFightJoukenDataId (0x8020d9d0, 24 bytes)
#[inline(never)]
pub fn f_8020d9d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d9d0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d9d0); p = true; }
    }
    f_8020d9d0_b(c, 0x8020d9d0);
    if p { crate::prof::exit(c, 0x8020d9d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d9d0_at(c: &mut Cpu, pc: u32) { f_8020d9d0_b(c, pc) }
#[inline(always)]
fn f_8020d9d0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d9d0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d9d0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d9d0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020d9e0; continue; }
                pc = 0x8020d9d8;
            }
            0x8020d9d8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020d9e0;
            }
            0x8020d9e0 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d9d0: bad pc {:#x}", pc),
        }
    }
}

/// fightKoukaDataBiosGetPtr (0x8020d9e8, 44 bytes)
#[inline(never)]
pub fn f_8020d9e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020d9e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d9e8); p = true; }
    }
    f_8020d9e8_b(c, 0x8020d9e8);
    if p { crate::prof::exit(c, 0x8020d9e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d9e8_at(c: &mut Cpu, pc: u32) { f_8020d9e8_b(c, pc) }
#[inline(always)]
fn f_8020d9e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020d9e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020d9e8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020d9e8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8530));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020da00; continue; }
                pc = 0x8020d9f8;
            }
            0x8020d9f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020da00;
            }
            0x8020da00 => { ins += 7;
                c.r[4] = c.r[3].wrapping_mul(0x6);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5cb8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020d9e8: bad pc {:#x}", pc),
        }
    }
}

/// fightKoukaDoFightKoukaJoukenAndKouka (0x8020da14, 188 bytes)
#[inline(never)]
pub fn f_8020da14(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020da14) { return; }
        if c.prof { crate::prof::enter(c, 0x8020da14); p = true; }
    }
    f_8020da14_b(c, 0x8020da14);
    if p { crate::prof::exit(c, 0x8020da14); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020da14_at(c: &mut Cpu, pc: u32) { f_8020da14_b(c, pc) }
#[inline(always)]
fn f_8020da14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020da14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020da14, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020da14 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020da34; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020da34;
            }
            0x8020da34 => { ins += 1;
                c.lr = 0x8020da38; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d9d0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020da38;
            }
            0x8020da38 => { ins += 3;
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020da44; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020da44;
            }
            0x8020da44 => { ins += 1;
                c.lr = 0x8020da48; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d9b8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020da48;
            }
            0x8020da48 => { ins += 3;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020da54; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d9e8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020da54;
            }
            0x8020da54 => { ins += 1;
                c.lr = 0x8020da58; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020d9a0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020da58;
            }
            0x8020da58 => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[29] = 0x0;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8020da74; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020da74;
            }
            0x8020da74 => { ins += 4;
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8020da84; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f02ac(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020da84;
            }
            0x8020da84 => { ins += 4;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8020da94; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020a8e0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020da94;
            }
            0x8020da94 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x8020dab8; continue; }
                pc = 0x8020daa0;
            }
            0x8020daa0 => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[27] | c.r[27];
                c.r[3] = c.r[31].rotate_left(0) & 0xffff;
                c.r[6] = 0x0;
                c.lr = 0x8020dab4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80136078(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dab4;
            }
            0x8020dab4 => { ins += 1;
                c.r[29] = 0x1;
                pc = 0x8020dab8;
            }
            0x8020dab8 => { ins += 17;
                c.r[3] = c.r[29] | c.r[29];
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020da14: bad pc {:#x}", pc),
        }
    }
}

/// battleStart (0x8020dad0, 628 bytes)
#[inline(never)]
pub fn f_8020dad0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020dad0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dad0); p = true; }
    }
    f_8020dad0_b(c, 0x8020dad0);
    if p { crate::prof::exit(c, 0x8020dad0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dad0_at(c: &mut Cpu, pc: u32) { f_8020dad0_b(c, pc) }
#[inline(never)]
fn f_8020dad0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020dad0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020dad0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020dad0 => { ins += 17;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                if !cr_bit(cr, 2) { pc = 0x8020daf4; continue; }
                pc = 0x8020daec;
            }
            0x8020daec => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x8020dd30; continue;
                pc = 0x8020daf4;
            }
            0x8020daf4 => { ins += 1;
                c.lr = 0x8020daf8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020daf8;
            }
            0x8020daf8 => { ins += 4;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[29] = c.r[0] | c.r[0];
                c.lr = 0x8020db08; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef62c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db08;
            }
            0x8020db08 => { ins += 2;
                c.r[3] = 0x9b0;
                c.lr = 0x8020db10; { c.steps = st; c.cycles = ins; c.cr = cr; f_801903b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db10;
            }
            0x8020db10 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020db18; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef61c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db18;
            }
            0x8020db18 => { ins += 1;
                c.lr = 0x8020db1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_800ff56c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db1c;
            }
            0x8020db1c => { ins += 7;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x4a;
                c.r[6] = 0x0;
                c.lr = 0x8020db38; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f4c14(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db38;
            }
            0x8020db38 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020db40; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db40;
            }
            0x8020db40 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020db58; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db58;
            }
            0x8020db58 => { ins += 2;
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8020db60; { c.steps = st; c.cycles = ins; c.cr = cr; f_801d23c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db60;
            }
            0x8020db60 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020db68; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0e0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db68;
            }
            0x8020db68 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.lr = 0x8020db70; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e488(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db70;
            }
            0x8020db70 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020dbcc; continue; }
                pc = 0x8020db78;
            }
            0x8020db78 => { ins += 1;
                c.lr = 0x8020db7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e260(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db7c;
            }
            0x8020db7c => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020dbcc; continue; }
                pc = 0x8020db84;
            }
            0x8020db84 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x1;
                c.lr = 0x8020db90; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e068(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db90;
            }
            0x8020db90 => { ins += 1;
                c.lr = 0x8020db94; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fccc4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020db94;
            }
            0x8020db94 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020dbcc; continue; }
                pc = 0x8020db9c;
            }
            0x8020db9c => { ins += 1;
                c.lr = 0x8020dba0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fcc7c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dba0;
            }
            0x8020dba0 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8020dba8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80261388(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dba8;
            }
            0x8020dba8 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020dbc0; continue; }
                pc = 0x8020dbb0;
            }
            0x8020dbb0 => { ins += 1;
                c.lr = 0x8020dbb4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80261444(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dbb4;
            }
            0x8020dbb4 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[3]);
                if cr_bit(cr, 2) { pc = 0x8020dbc0; continue; }
                pc = 0x8020dbbc;
            }
            0x8020dbbc => { ins += 1;
                c.lr = 0x8020dbc0; { c.steps = st; c.cycles = ins; c.cr = cr; f_80190528(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dbc0;
            }
            0x8020dbc0 => { ins += 3;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x59;
                c.lr = 0x8020dbcc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80132a38(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dbcc;
            }
            0x8020dbcc => { ins += 4;
                c.r[3] = 0x1;
                c.r[4] = 0x3e8;
                c.r[5] = 0xff;
                c.lr = 0x8020dbdc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80165a20(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dbdc;
            }
            0x8020dbdc => { ins += 2;
                c.r[3] = 0x3e8;
                c.lr = 0x8020dbe4; { c.steps = st; c.cycles = ins; c.cr = cr; f_801657d0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dbe4;
            }
            0x8020dbe4 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020dbec; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020dff0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dbec;
            }
            0x8020dbec => { ins += 1;
                c.lr = 0x8020dbf0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020deb0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dbf0;
            }
            0x8020dbf0 => { ins += 2;
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8020dbf8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020de80(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dbf8;
            }
            0x8020dbf8 => { ins += 3;
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020dc04; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020de98(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc04;
            }
            0x8020dc04 => { ins += 3;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020dc10; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020de68(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc10;
            }
            0x8020dc10 => { ins += 6;
                c.f[2] = c.f[1];
                { let ea = c.r[2].wrapping_add(0xffffae88); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[30] | c.r[30];
                c.r[3] = 0x9;
                c.lr = 0x8020dc28; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c4164(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc28;
            }
            0x8020dc28 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020dc30; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020de50(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc30;
            }
            0x8020dc30 => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[3]);
                if cr_bit(cr, 2) { pc = 0x8020dc44; continue; }
                pc = 0x8020dc38;
            }
            0x8020dc38 => { ins += 3;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8020dc44; { c.steps = st; c.cycles = ins; c.cr = cr; f_80166ab8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc44;
            }
            0x8020dc44 => { ins += 1;
                c.lr = 0x8020dc48; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef7b4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc48;
            }
            0x8020dc48 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020dc50; { c.steps = st; c.cycles = ins; c.cr = cr; f_800ff730(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc50;
            }
            0x8020dc50 => { ins += 3;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8020dc5c; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011288c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc5c;
            }
            0x8020dc5c => { ins += 1;
                c.lr = 0x8020dc60; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc60;
            }
            0x8020dc60 => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020dc68; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011395c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc68;
            }
            0x8020dc68 => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020dc70; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020dfd8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc70;
            }
            0x8020dc70 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020dcf0; continue; }
                pc = 0x8020dc78;
            }
            0x8020dc78 => { ins += 1;
                c.lr = 0x8020dc7c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc7c;
            }
            0x8020dc7c => { ins += 3;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020dc88; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f1dbc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc88;
            }
            0x8020dc88 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020dcf0; continue; }
                pc = 0x8020dc90;
            }
            0x8020dc90 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020dc98; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef61c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dc98;
            }
            0x8020dc98 => { ins += 2;
                c.r[3] = 0xe05;
                c.lr = 0x8020dca0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801903b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dca0;
            }
            0x8020dca0 => { ins += 4;
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.r[5] = 0x0;
                c.lr = 0x8020dcb0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8012a5b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dcb0;
            }
            0x8020dcb0 => { ins += 5;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[4] = c.addc(c.r[0], 0, ci); }
                c.lr = 0x8020dcc4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80129474(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dcc4;
            }
            0x8020dcc4 => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x8020dccc; { c.steps = st; c.cycles = ins; c.cr = cr; f_801d0afc(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dccc;
            }
            0x8020dccc => { ins += 1;
                c.lr = 0x8020dcd0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8018da88(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dcd0;
            }
            0x8020dcd0 => { ins += 1;
                c.lr = 0x8020dcd4; { c.steps = st; c.cycles = ins; c.cr = cr; f_80113fe8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dcd4;
            }
            0x8020dcd4 => { ins += 4;
                c.r[4] = 0x5960000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x8);
                c.lr = 0x8020dce4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8011288c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dce4;
            }
            0x8020dce4 => { ins += 1;
                c.lr = 0x8020dce8; { c.steps = st; c.cycles = ins; c.cr = cr; crate::stubs::f_800f0308(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dce8;
            }
            0x8020dce8 => { ins += 1;
                c.lr = 0x8020dcec; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dcec;
            }
            0x8020dcec => { ins += 1;
                pc = 0x8020dd30; continue;
                pc = 0x8020dcf0;
            }
            0x8020dcf0 => { ins += 2;
                c.r[3] = 0x9b0;
                c.lr = 0x8020dcf8; { c.steps = st; c.cycles = ins; c.cr = cr; f_80190528(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dcf8;
            }
            0x8020dcf8 => { ins += 1;
                c.lr = 0x8020dcfc; { c.steps = st; c.cycles = ins; c.cr = cr; f_80112700(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dcfc;
            }
            0x8020dcfc => { ins += 1;
                c.lr = 0x8020dd00; { c.steps = st; c.cycles = ins; c.cr = cr; f_801140c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dd00;
            }
            0x8020dd00 => { ins += 2;
                c.r[3] = 0xe05;
                c.lr = 0x8020dd08; { c.steps = st; c.cycles = ins; c.cr = cr; f_801902e0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dd08;
            }
            0x8020dd08 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020dd24; continue; }
                pc = 0x8020dd10;
            }
            0x8020dd10 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffffae8c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8020dd1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c41c8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dd1c;
            }
            0x8020dd1c => { ins += 2;
                c.r[3] = 0x1;
                c.lr = 0x8020dd24; { c.steps = st; c.cycles = ins; c.cr = cr; f_801c40f0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dd24;
            }
            0x8020dd24 => { ins += 2;
                c.r[3] = 0x0;
                c.lr = 0x8020dd2c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef61c(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dd2c;
            }
            0x8020dd2c => { ins += 1;
                c.lr = 0x8020dd30; { c.steps = st; c.cycles = ins; c.cr = cr; f_801ef634(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dd30;
            }
            0x8020dd30 => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020dad0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountGetEnvSndDataId (0x8020dd44, 60 bytes)
#[inline(never)]
pub fn f_8020dd44(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020dd44) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dd44); p = true; }
    }
    f_8020dd44_b(c, 0x8020dd44);
    if p { crate::prof::exit(c, 0x8020dd44); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dd44_at(c: &mut Cpu, pc: u32) { f_8020dd44_b(c, pc) }
#[inline(always)]
fn f_8020dd44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020dd44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020dd44, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020dd44 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8020dd54; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dd54;
            }
            0x8020dd54 => { ins += 1;
                c.lr = 0x8020dd58; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dd58;
            }
            0x8020dd58 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x8020dd70; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dd70;
            }
            0x8020dd70 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020dd44: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountGetBgmSndDataId (0x8020dd80, 208 bytes)
#[inline(never)]
pub fn f_8020dd80(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020dd80) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dd80); p = true; }
    }
    f_8020dd80_b(c, 0x8020dd80);
    if p { crate::prof::exit(c, 0x8020dd80); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dd80_at(c: &mut Cpu, pc: u32) { f_8020dd80_b(c, pc) }
#[inline(always)]
fn f_8020dd80_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020dd80;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020dd80, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020dd80 => { ins += 16;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x8020dd98; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dd98;
            }
            0x8020dd98 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020dda0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e008(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020dda0;
            }
            0x8020dda0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020ddac; continue; }
                pc = 0x8020dda8;
            }
            0x8020dda8 => { ins += 1;
                pc = 0x8020de3c; continue;
                pc = 0x8020ddac;
            }
            0x8020ddac => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020ddb4; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0f8(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ddb4;
            }
            0x8020ddb4 => { ins += 1;
                c.lr = 0x8020ddb8; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e0b0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ddb8;
            }
            0x8020ddb8 => { ins += 6;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x6;
                c.r[6] = 0x0;
                c.lr = 0x8020ddd0; { c.steps = st; c.cycles = ins; c.cr = cr; f_801f54a4(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ddd0;
            }
            0x8020ddd0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020dddc; continue; }
                pc = 0x8020ddd8;
            }
            0x8020ddd8 => { ins += 1;
                pc = 0x8020de3c; continue;
                pc = 0x8020dddc;
            }
            0x8020dddc => { ins += 2;
                c.r[30] = 0x0;
                pc = 0x8020de2c; continue;
                pc = 0x8020dde4;
            }
            0x8020dde4 => { ins += 3;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                c.lr = 0x8020ddf0; { c.steps = st; c.cycles = ins; c.cr = cr; f_8020e068(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020ddf0;
            }
            0x8020ddf0 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x8020de28; continue; }
                pc = 0x8020ddf8;
            }
            0x8020ddf8 => { ins += 5;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020de0c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fb1c0(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020de0c;
            }
            0x8020de0c => { ins += 2;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[3]);
                if cr_bit(cr, 2) { pc = 0x8020de28; continue; }
                pc = 0x8020de14;
            }
            0x8020de14 => { ins += 1;
                c.lr = 0x8020de18; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fbd58(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020de18;
            }
            0x8020de18 => { ins += 1;
                c.lr = 0x8020de1c; { c.steps = st; c.cycles = ins; c.cr = cr; f_801fbd10(c); st = c.steps; ins = c.cycles; cr = c.cr; };
                pc = 0x8020de1c;
            }
            0x8020de1c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x8020de28; continue; }
                pc = 0x8020de24;
            }
            0x8020de24 => { ins += 1;
                pc = 0x8020de3c; continue;
                pc = 0x8020de28;
            }
            0x8020de28 => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x8020de2c;
            }
            0x8020de2c => { ins += 3;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x8020dde4; continue; }
                pc = 0x8020de38;
            }
            0x8020de38 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x8020de3c;
            }
            0x8020de3c => { ins += 16;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020dd80: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetWipeEffectSndID (0x8020de50, 24 bytes)
#[inline(never)]
pub fn f_8020de50(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020de50) { return; }
        if c.prof { crate::prof::enter(c, 0x8020de50); p = true; }
    }
    f_8020de50_b(c, 0x8020de50);
    if p { crate::prof::exit(c, 0x8020de50); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020de50_at(c: &mut Cpu, pc: u32) { f_8020de50_b(c, pc) }
#[inline(always)]
fn f_8020de50_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020de50;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020de50, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020de50 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020de60; continue; }
                pc = 0x8020de58;
            }
            0x8020de58 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020de60;
            }
            0x8020de60 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020de50: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetWipeEffectTime (0x8020de68, 24 bytes)
#[inline(never)]
pub fn f_8020de68(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020de68) { return; }
        if c.prof { crate::prof::enter(c, 0x8020de68); p = true; }
    }
    f_8020de68_b(c, 0x8020de68);
    if p { crate::prof::exit(c, 0x8020de68); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020de68_at(c: &mut Cpu, pc: u32) { f_8020de68_b(c, pc) }
#[inline(always)]
fn f_8020de68_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020de68;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020de68, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020de68 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020de78; continue; }
                pc = 0x8020de70;
            }
            0x8020de70 => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffffae90); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020de78;
            }
            0x8020de78 => { ins += 2;
                { let ea = c.r[3].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020de68: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetWipeSnapshotUse (0x8020de80, 24 bytes)
#[inline(never)]
pub fn f_8020de80(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020de80) { return; }
        if c.prof { crate::prof::enter(c, 0x8020de80); p = true; }
    }
    f_8020de80_b(c, 0x8020de80);
    if p { crate::prof::exit(c, 0x8020de80); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020de80_at(c: &mut Cpu, pc: u32) { f_8020de80_b(c, pc) }
#[inline(always)]
fn f_8020de80_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020de80;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020de80, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020de80 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020de90; continue; }
                pc = 0x8020de88;
            }
            0x8020de88 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020de90;
            }
            0x8020de90 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020de80: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetWipeFunction (0x8020de98, 24 bytes)
#[inline(never)]
pub fn f_8020de98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020de98) { return; }
        if c.prof { crate::prof::enter(c, 0x8020de98); p = true; }
    }
    f_8020de98_b(c, 0x8020de98);
    if p { crate::prof::exit(c, 0x8020de98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020de98_at(c: &mut Cpu, pc: u32) { f_8020de98_b(c, pc) }
#[inline(always)]
fn f_8020de98_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020de98;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020de98, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020de98 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020dea8; continue; }
                pc = 0x8020dea0;
            }
            0x8020dea0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020dea8;
            }
            0x8020dea8 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020de98: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountWipeDataBiosGetPtr (0x8020deb0, 40 bytes)
#[inline(never)]
pub fn f_8020deb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020deb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020deb0); p = true; }
    }
    f_8020deb0_b(c, 0x8020deb0);
    if p { crate::prof::exit(c, 0x8020deb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020deb0_at(c: &mut Cpu, pc: u32) { f_8020deb0_b(c, pc) }
#[inline(always)]
fn f_8020deb0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020deb0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020deb0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020deb0 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8500));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020dec4; continue; }
                pc = 0x8020debc;
            }
            0x8020debc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020dec4;
            }
            0x8020dec4 => { ins += 7;
                c.r[4] = c.r[3].wrapping_mul(0xc);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5980);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020deb0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetFightName (0x8020ded8, 24 bytes)
#[inline(never)]
pub fn f_8020ded8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020ded8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ded8); p = true; }
    }
    f_8020ded8_b(c, 0x8020ded8);
    if p { crate::prof::exit(c, 0x8020ded8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ded8_at(c: &mut Cpu, pc: u32) { f_8020ded8_b(c, pc) }
#[inline(always)]
fn f_8020ded8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020ded8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020ded8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020ded8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020dee8; continue; }
                pc = 0x8020dee0;
            }
            0x8020dee0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020dee8;
            }
            0x8020dee8 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020ded8: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetSyoukaiWzxDataId (0x8020def0, 16 bytes)
#[inline(never)]
pub fn f_8020def0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020def0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020def0); p = true; }
    }
    f_8020def0_b(c, 0x8020def0);
    if p { crate::prof::exit(c, 0x8020def0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020def0_at(c: &mut Cpu, pc: u32) { f_8020def0_b(c, pc) }
#[inline(always)]
fn f_8020def0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020def0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020def0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020def0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020def8;
            }
            0x8020def8 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0x14), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020def0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetBgmSndId (0x8020df00, 16 bytes)
#[inline(never)]
pub fn f_8020df00(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020df00) { return; }
        if c.prof { crate::prof::enter(c, 0x8020df00); p = true; }
    }
    f_8020df00_b(c, 0x8020df00);
    if p { crate::prof::exit(c, 0x8020df00); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020df00_at(c: &mut Cpu, pc: u32) { f_8020df00_b(c, pc) }
#[inline(always)]
fn f_8020df00_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020df00;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020df00, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020df00 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020df08;
            }
            0x8020df08 => { ins += 2;
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020df00: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetGSInputDevice (0x8020df10, 64 bytes)
#[inline(never)]
pub fn f_8020df10(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020df10) { return; }
        if c.prof { crate::prof::enter(c, 0x8020df10); p = true; }
    }
    f_8020df10_b(c, 0x8020df10);
    if p { crate::prof::exit(c, 0x8020df10); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020df10_at(c: &mut Cpu, pc: u32) { f_8020df10_b(c, pc) }
#[inline(always)]
fn f_8020df10_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020df10;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020df10, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020df10 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020df20; continue; }
                pc = 0x8020df18;
            }
            0x8020df18 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8020df40; continue;
                pc = 0x8020df20;
            }
            0x8020df20 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x8020df34; continue; }
                pc = 0x8020df2c;
            }
            0x8020df2c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8020df40; continue;
                pc = 0x8020df34;
            }
            0x8020df34 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x18);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8020df40;
            }
            0x8020df40 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020df48;
            }
            0x8020df48 => { ins += 2;
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020df10: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetFightTrainerDataId (0x8020df50, 64 bytes)
#[inline(never)]
pub fn f_8020df50(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020df50) { return; }
        if c.prof { crate::prof::enter(c, 0x8020df50); p = true; }
    }
    f_8020df50_b(c, 0x8020df50);
    if p { crate::prof::exit(c, 0x8020df50); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020df50_at(c: &mut Cpu, pc: u32) { f_8020df50_b(c, pc) }
#[inline(always)]
fn f_8020df50_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020df50;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020df50, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020df50 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020df60; continue; }
                pc = 0x8020df58;
            }
            0x8020df58 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8020df80; continue;
                pc = 0x8020df60;
            }
            0x8020df60 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x8020df74; continue; }
                pc = 0x8020df6c;
            }
            0x8020df6c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8020df80; continue;
                pc = 0x8020df74;
            }
            0x8020df74 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x18);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8020df80;
            }
            0x8020df80 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020df88;
            }
            0x8020df88 => { ins += 2;
                c.sth(c.r[4], c.r[5]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020df50: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetFightFloorDataId (0x8020df90, 16 bytes)
#[inline(never)]
pub fn f_8020df90(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020df90) { return; }
        if c.prof { crate::prof::enter(c, 0x8020df90); p = true; }
    }
    f_8020df90_b(c, 0x8020df90);
    if p { crate::prof::exit(c, 0x8020df90); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020df90_at(c: &mut Cpu, pc: u32) { f_8020df90_b(c, pc) }
#[inline(always)]
fn f_8020df90_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020df90;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020df90, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020df90 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020df98;
            }
            0x8020df98 => { ins += 2;
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020df90: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetTrainer (0x8020dfa0, 16 bytes)
#[inline(never)]
pub fn f_8020dfa0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020dfa0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dfa0); p = true; }
    }
    f_8020dfa0_b(c, 0x8020dfa0);
    if p { crate::prof::exit(c, 0x8020dfa0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dfa0_at(c: &mut Cpu, pc: u32) { f_8020dfa0_b(c, pc) }
#[inline(always)]
fn f_8020dfa0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020dfa0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020dfa0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020dfa0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020dfa8;
            }
            0x8020dfa8 => { ins += 2;
                c.stb(c.r[3].wrapping_add(0x1), c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020dfa0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetFightKind (0x8020dfb0, 16 bytes)
#[inline(never)]
pub fn f_8020dfb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020dfb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dfb0); p = true; }
    }
    f_8020dfb0_b(c, 0x8020dfb0);
    if p { crate::prof::exit(c, 0x8020dfb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dfb0_at(c: &mut Cpu, pc: u32) { f_8020dfb0_b(c, pc) }
#[inline(always)]
fn f_8020dfb0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020dfb0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020dfb0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020dfb0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020dfb8;
            }
            0x8020dfb8 => { ins += 2;
                c.stb(c.r[3], c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020dfb0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetSyoukaiWzxDataId (0x8020dfc0, 24 bytes)
#[inline(never)]
pub fn f_8020dfc0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020dfc0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dfc0); p = true; }
    }
    f_8020dfc0_b(c, 0x8020dfc0);
    if p { crate::prof::exit(c, 0x8020dfc0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dfc0_at(c: &mut Cpu, pc: u32) { f_8020dfc0_b(c, pc) }
#[inline(always)]
fn f_8020dfc0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020dfc0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020dfc0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020dfc0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020dfd0; continue; }
                pc = 0x8020dfc8;
            }
            0x8020dfc8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020dfd0;
            }
            0x8020dfd0 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x14));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020dfc0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetZenmetuFlag (0x8020dfd8, 24 bytes)
#[inline(never)]
pub fn f_8020dfd8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020dfd8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dfd8); p = true; }
    }
    f_8020dfd8_b(c, 0x8020dfd8);
    if p { crate::prof::exit(c, 0x8020dfd8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dfd8_at(c: &mut Cpu, pc: u32) { f_8020dfd8_b(c, pc) }
#[inline(always)]
fn f_8020dfd8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020dfd8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020dfd8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020dfd8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020dfe8; continue; }
                pc = 0x8020dfe0;
            }
            0x8020dfe0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020dfe8;
            }
            0x8020dfe8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020dfd8: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetWipeId (0x8020dff0, 24 bytes)
#[inline(never)]
pub fn f_8020dff0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020dff0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dff0); p = true; }
    }
    f_8020dff0_b(c, 0x8020dff0);
    if p { crate::prof::exit(c, 0x8020dff0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dff0_at(c: &mut Cpu, pc: u32) { f_8020dff0_b(c, pc) }
#[inline(always)]
fn f_8020dff0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020dff0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020dff0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020dff0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e000; continue; }
                pc = 0x8020dff8;
            }
            0x8020dff8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e000;
            }
            0x8020e000 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x10));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020dff0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetBgmSndId (0x8020e008, 24 bytes)
#[inline(never)]
pub fn f_8020e008(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e008) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e008); p = true; }
    }
    f_8020e008_b(c, 0x8020e008);
    if p { crate::prof::exit(c, 0x8020e008); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e008_at(c: &mut Cpu, pc: u32) { f_8020e008_b(c, pc) }
#[inline(always)]
fn f_8020e008_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e008;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e008, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e008 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e018; continue; }
                pc = 0x8020e010;
            }
            0x8020e010 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e018;
            }
            0x8020e018 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e008: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetGSInputDevice (0x8020e020, 72 bytes)
#[inline(never)]
pub fn f_8020e020(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e020) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e020); p = true; }
    }
    f_8020e020_b(c, 0x8020e020);
    if p { crate::prof::exit(c, 0x8020e020); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e020_at(c: &mut Cpu, pc: u32) { f_8020e020_b(c, pc) }
#[inline(always)]
fn f_8020e020_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e020;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e020, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e020 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e030; continue; }
                pc = 0x8020e028;
            }
            0x8020e028 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8020e050; continue;
                pc = 0x8020e030;
            }
            0x8020e030 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x8020e044; continue; }
                pc = 0x8020e03c;
            }
            0x8020e03c => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8020e050; continue;
                pc = 0x8020e044;
            }
            0x8020e044 => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x18);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8020e050;
            }
            0x8020e050 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e060; continue; }
                pc = 0x8020e058;
            }
            0x8020e058 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e060;
            }
            0x8020e060 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e020: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetFightTrainerDataId (0x8020e068, 72 bytes)
#[inline(never)]
pub fn f_8020e068(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e068) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e068); p = true; }
    }
    f_8020e068_b(c, 0x8020e068);
    if p { crate::prof::exit(c, 0x8020e068); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e068_at(c: &mut Cpu, pc: u32) { f_8020e068_b(c, pc) }
#[inline(always)]
fn f_8020e068_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e068;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e068, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e068 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e078; continue; }
                pc = 0x8020e070;
            }
            0x8020e070 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8020e098; continue;
                pc = 0x8020e078;
            }
            0x8020e078 => { ins += 3;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 0) { pc = 0x8020e08c; continue; }
                pc = 0x8020e084;
            }
            0x8020e084 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8020e098; continue;
                pc = 0x8020e08c;
            }
            0x8020e08c => { ins += 3;
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x18);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8020e098;
            }
            0x8020e098 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e0a8; continue; }
                pc = 0x8020e0a0;
            }
            0x8020e0a0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e0a8;
            }
            0x8020e0a8 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[4]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e068: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetFightFloorDataId (0x8020e0b0, 24 bytes)
#[inline(never)]
pub fn f_8020e0b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e0b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e0b0); p = true; }
    }
    f_8020e0b0_b(c, 0x8020e0b0);
    if p { crate::prof::exit(c, 0x8020e0b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e0b0_at(c: &mut Cpu, pc: u32) { f_8020e0b0_b(c, pc) }
#[inline(always)]
fn f_8020e0b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e0b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e0b0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e0b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e0c0; continue; }
                pc = 0x8020e0b8;
            }
            0x8020e0b8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e0c0;
            }
            0x8020e0c0 => { ins += 2;
                c.r[3] = c.lhz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e0b0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetTrainer (0x8020e0c8, 24 bytes)
#[inline(never)]
pub fn f_8020e0c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e0c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e0c8); p = true; }
    }
    f_8020e0c8_b(c, 0x8020e0c8);
    if p { crate::prof::exit(c, 0x8020e0c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e0c8_at(c: &mut Cpu, pc: u32) { f_8020e0c8_b(c, pc) }
#[inline(always)]
fn f_8020e0c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e0c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e0c8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e0c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e0d8; continue; }
                pc = 0x8020e0d0;
            }
            0x8020e0d0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e0d8;
            }
            0x8020e0d8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e0c8: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetFightKind (0x8020e0e0, 24 bytes)
#[inline(never)]
pub fn f_8020e0e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e0e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e0e0); p = true; }
    }
    f_8020e0e0_b(c, 0x8020e0e0);
    if p { crate::prof::exit(c, 0x8020e0e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e0e0_at(c: &mut Cpu, pc: u32) { f_8020e0e0_b(c, pc) }
#[inline(always)]
fn f_8020e0e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e0e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e0e0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e0e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e0f0; continue; }
                pc = 0x8020e0e8;
            }
            0x8020e0e8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e0f0;
            }
            0x8020e0f0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e0e0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetPtr (0x8020e0f8, 44 bytes)
#[inline(never)]
pub fn f_8020e0f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e0f8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e0f8); p = true; }
    }
    f_8020e0f8_b(c, 0x8020e0f8);
    if p { crate::prof::exit(c, 0x8020e0f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e0f8_at(c: &mut Cpu, pc: u32) { f_8020e0f8_b(c, pc) }
#[inline(always)]
fn f_8020e0f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e0f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e0f8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e0f8 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8730));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x8020e114; continue; }
                pc = 0x8020e10c;
            }
            0x8020e10c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e114;
            }
            0x8020e114 => { ins += 6;
                c.r[0] = c.r[3].wrapping_mul(0x38);
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8734));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e0f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeGetFightSideFightOutPokemonMax (0x8020e124, 128 bytes)
#[inline(never)]
pub fn f_8020e124(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e124) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e124); p = true; }
    }
    f_8020e124_b(c, 0x8020e124);
    if p { crate::prof::exit(c, 0x8020e124); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e124_at(c: &mut Cpu, pc: u32) { f_8020e124_b(c, pc) }
#[inline(always)]
fn f_8020e124_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e124;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e124, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e124 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86e0));
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[6] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], c.r[6]);
                if !cr_bit(cr, 1) { pc = 0x8020e140; continue; }
                pc = 0x8020e138;
            }
            0x8020e138 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8020e14c; continue;
                pc = 0x8020e140;
            }
            0x8020e140 => { ins += 3;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86e4));
                c.r[0] = c.r[3].rotate_left(3) & 0x7fff8;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                pc = 0x8020e14c;
            }
            0x8020e14c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e15c; continue; }
                pc = 0x8020e154;
            }
            0x8020e154 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x8020e160; continue;
                pc = 0x8020e15c;
            }
            0x8020e15c => { ins += 1;
                c.r[5] = c.lbz_t(ins, c.r[4]);
                pc = 0x8020e160;
            }
            0x8020e160 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], c.r[6]);
                if !cr_bit(cr, 1) { pc = 0x8020e170; continue; }
                pc = 0x8020e168;
            }
            0x8020e168 => { ins += 2;
                c.r[4] = 0x0;
                pc = 0x8020e17c; continue;
                pc = 0x8020e170;
            }
            0x8020e170 => { ins += 3;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86e4));
                c.r[0] = c.r[3].rotate_left(3) & 0x7fff8;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                pc = 0x8020e17c;
            }
            0x8020e17c => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x0);
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if !cr_bit(cr, 2) { pc = 0x8020e190; continue; }
                pc = 0x8020e188;
            }
            0x8020e188 => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x8020e194; continue;
                pc = 0x8020e190;
            }
            0x8020e190 => { ins += 1;
                c.r[0] = c.lbz_t(ins, c.r[4].wrapping_add(0x2));
                pc = 0x8020e194;
            }
            0x8020e194 => { ins += 8;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_mul(c.r[0]);
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e124: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeDataBiosGetFightoutPokemonNum (0x8020e1a4, 24 bytes)
#[inline(never)]
pub fn f_8020e1a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e1a4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e1a4); p = true; }
    }
    f_8020e1a4_b(c, 0x8020e1a4);
    if p { crate::prof::exit(c, 0x8020e1a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e1a4_at(c: &mut Cpu, pc: u32) { f_8020e1a4_b(c, pc) }
#[inline(always)]
fn f_8020e1a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e1a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e1a4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e1a4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e1b4; continue; }
                pc = 0x8020e1ac;
            }
            0x8020e1ac => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e1b4;
            }
            0x8020e1b4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e1a4: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeDataBiosGetEntryPokemonNum (0x8020e1bc, 24 bytes)
#[inline(never)]
pub fn f_8020e1bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e1bc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e1bc); p = true; }
    }
    f_8020e1bc_b(c, 0x8020e1bc);
    if p { crate::prof::exit(c, 0x8020e1bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e1bc_at(c: &mut Cpu, pc: u32) { f_8020e1bc_b(c, pc) }
#[inline(always)]
fn f_8020e1bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e1bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e1bc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e1bc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e1cc; continue; }
                pc = 0x8020e1c4;
            }
            0x8020e1c4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e1cc;
            }
            0x8020e1cc => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e1bc: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeDataBiosGetTrainerNum (0x8020e1d4, 24 bytes)
#[inline(never)]
pub fn f_8020e1d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e1d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e1d4); p = true; }
    }
    f_8020e1d4_b(c, 0x8020e1d4);
    if p { crate::prof::exit(c, 0x8020e1d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e1d4_at(c: &mut Cpu, pc: u32) { f_8020e1d4_b(c, pc) }
#[inline(always)]
fn f_8020e1d4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e1d4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e1d4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e1d4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e1e4; continue; }
                pc = 0x8020e1dc;
            }
            0x8020e1dc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e1e4;
            }
            0x8020e1e4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e1d4: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeDataBiosGetName (0x8020e1ec, 24 bytes)
#[inline(never)]
pub fn f_8020e1ec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e1ec) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e1ec); p = true; }
    }
    f_8020e1ec_b(c, 0x8020e1ec);
    if p { crate::prof::exit(c, 0x8020e1ec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e1ec_at(c: &mut Cpu, pc: u32) { f_8020e1ec_b(c, pc) }
#[inline(always)]
fn f_8020e1ec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e1ec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e1ec, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e1ec => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e1fc; continue; }
                pc = 0x8020e1f4;
            }
            0x8020e1f4 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e1fc;
            }
            0x8020e1fc => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e1ec: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeDataBiosGetPtr (0x8020e204, 44 bytes)
#[inline(never)]
pub fn f_8020e204(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e204) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e204); p = true; }
    }
    f_8020e204_b(c, 0x8020e204);
    if p { crate::prof::exit(c, 0x8020e204); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e204_at(c: &mut Cpu, pc: u32) { f_8020e204_b(c, pc) }
#[inline(always)]
fn f_8020e204_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e204;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e204, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e204 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86e0));
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x8020e220; continue; }
                pc = 0x8020e218;
            }
            0x8020e218 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e220;
            }
            0x8020e220 => { ins += 4;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff86e4));
                c.r[0] = c.r[3].rotate_left(3) & 0x7fff8;
                c.r[3] = c.r[4].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e204: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetHostEnemyMsgFlag (0x8020e230, 24 bytes)
#[inline(never)]
pub fn f_8020e230(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e230) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e230); p = true; }
    }
    f_8020e230_b(c, 0x8020e230);
    if p { crate::prof::exit(c, 0x8020e230); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e230_at(c: &mut Cpu, pc: u32) { f_8020e230_b(c, pc) }
#[inline(always)]
fn f_8020e230_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e230;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e230, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e230 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e240; continue; }
                pc = 0x8020e238;
            }
            0x8020e238 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e240;
            }
            0x8020e240 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x18));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e230: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetName (0x8020e248, 24 bytes)
#[inline(never)]
pub fn f_8020e248(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e248) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e248); p = true; }
    }
    f_8020e248_b(c, 0x8020e248);
    if p { crate::prof::exit(c, 0x8020e248); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e248_at(c: &mut Cpu, pc: u32) { f_8020e248_b(c, pc) }
#[inline(always)]
fn f_8020e248_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e248;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e248, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e248 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e258; continue; }
                pc = 0x8020e250;
            }
            0x8020e250 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e258;
            }
            0x8020e258 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x1c));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e248: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetPokemonStatusMenuSubbarFlag (0x8020e260, 24 bytes)
#[inline(never)]
pub fn f_8020e260(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e260) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e260); p = true; }
    }
    f_8020e260_b(c, 0x8020e260);
    if p { crate::prof::exit(c, 0x8020e260); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e260_at(c: &mut Cpu, pc: u32) { f_8020e260_b(c, pc) }
#[inline(always)]
fn f_8020e260_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e260;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e260, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e260 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e270; continue; }
                pc = 0x8020e268;
            }
            0x8020e268 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e270;
            }
            0x8020e270 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x17));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e260: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDarkpokemonHypermodeFlag (0x8020e278, 24 bytes)
#[inline(never)]
pub fn f_8020e278(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e278) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e278); p = true; }
    }
    f_8020e278_b(c, 0x8020e278);
    if p { crate::prof::exit(c, 0x8020e278); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e278_at(c: &mut Cpu, pc: u32) { f_8020e278_b(c, pc) }
#[inline(always)]
fn f_8020e278_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e278;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e278, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e278 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e288; continue; }
                pc = 0x8020e280;
            }
            0x8020e280 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e288;
            }
            0x8020e288 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x16));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e278: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetMonohiroiFlag (0x8020e290, 24 bytes)
#[inline(never)]
pub fn f_8020e290(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e290) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e290); p = true; }
    }
    f_8020e290_b(c, 0x8020e290);
    if p { crate::prof::exit(c, 0x8020e290); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e290_at(c: &mut Cpu, pc: u32) { f_8020e290_b(c, pc) }
#[inline(always)]
fn f_8020e290_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e290;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e290, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e290 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e2a0; continue; }
                pc = 0x8020e298;
            }
            0x8020e298 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e2a0;
            }
            0x8020e2a0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x15));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e290: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDorobouFlag (0x8020e2a8, 24 bytes)
#[inline(never)]
pub fn f_8020e2a8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e2a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e2a8); p = true; }
    }
    f_8020e2a8_b(c, 0x8020e2a8);
    if p { crate::prof::exit(c, 0x8020e2a8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e2a8_at(c: &mut Cpu, pc: u32) { f_8020e2a8_b(c, pc) }
#[inline(always)]
fn f_8020e2a8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e2a8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e2a8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e2a8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e2b8; continue; }
                pc = 0x8020e2b0;
            }
            0x8020e2b0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e2b8;
            }
            0x8020e2b8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x14));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e2a8: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetBossFlag (0x8020e2c0, 24 bytes)
#[inline(never)]
pub fn f_8020e2c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e2c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e2c0); p = true; }
    }
    f_8020e2c0_b(c, 0x8020e2c0);
    if p { crate::prof::exit(c, 0x8020e2c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e2c0_at(c: &mut Cpu, pc: u32) { f_8020e2c0_b(c, pc) }
#[inline(always)]
fn f_8020e2c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e2c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e2c0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e2c0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e2d0; continue; }
                pc = 0x8020e2c8;
            }
            0x8020e2c8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e2d0;
            }
            0x8020e2d0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x13));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e2c0: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetKeikentihueruFlag (0x8020e2d8, 24 bytes)
#[inline(never)]
pub fn f_8020e2d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e2d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e2d8); p = true; }
    }
    f_8020e2d8_b(c, 0x8020e2d8);
    if p { crate::prof::exit(c, 0x8020e2d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e2d8_at(c: &mut Cpu, pc: u32) { f_8020e2d8_b(c, pc) }
#[inline(always)]
fn f_8020e2d8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e2d8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e2d8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e2d8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e2e8; continue; }
                pc = 0x8020e2e0;
            }
            0x8020e2e0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e2e8;
            }
            0x8020e2e8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x12));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e2d8: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoItemSoubiTokukoutokubouupFlag (0x8020e2f0, 24 bytes)
#[inline(never)]
pub fn f_8020e2f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e2f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e2f0); p = true; }
    }
    f_8020e2f0_b(c, 0x8020e2f0);
    if p { crate::prof::exit(c, 0x8020e2f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e2f0_at(c: &mut Cpu, pc: u32) { f_8020e2f0_b(c, pc) }
#[inline(always)]
fn f_8020e2f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e2f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e2f0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e2f0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e300; continue; }
                pc = 0x8020e2f8;
            }
            0x8020e2f8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e300;
            }
            0x8020e300 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x11));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e2f0: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoHizukiMiyaburiFlag (0x8020e308, 24 bytes)
#[inline(never)]
pub fn f_8020e308(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e308) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e308); p = true; }
    }
    f_8020e308_b(c, 0x8020e308);
    if p { crate::prof::exit(c, 0x8020e308); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e308_at(c: &mut Cpu, pc: u32) { f_8020e308_b(c, pc) }
#[inline(always)]
fn f_8020e308_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e308;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e308, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e308 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e318; continue; }
                pc = 0x8020e310;
            }
            0x8020e310 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e318;
            }
            0x8020e318 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x10));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e308: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoHizukiAiFlag (0x8020e320, 24 bytes)
#[inline(never)]
pub fn f_8020e320(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e320) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e320); p = true; }
    }
    f_8020e320_b(c, 0x8020e320);
    if p { crate::prof::exit(c, 0x8020e320); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e320_at(c: &mut Cpu, pc: u32) { f_8020e320_b(c, pc) }
#[inline(always)]
fn f_8020e320_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e320;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e320, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e320 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e330; continue; }
                pc = 0x8020e328;
            }
            0x8020e328 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e330;
            }
            0x8020e330 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xf));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e320: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoCriticalAttackFlag (0x8020e338, 24 bytes)
#[inline(never)]
pub fn f_8020e338(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e338) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e338); p = true; }
    }
    f_8020e338_b(c, 0x8020e338);
    if p { crate::prof::exit(c, 0x8020e338); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e338_at(c: &mut Cpu, pc: u32) { f_8020e338_b(c, pc) }
#[inline(always)]
fn f_8020e338_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e338;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e338, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e338 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e348; continue; }
                pc = 0x8020e340;
            }
            0x8020e340 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e348;
            }
            0x8020e348 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xe));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e338: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetGetInfectPokerusFlag (0x8020e350, 24 bytes)
#[inline(never)]
pub fn f_8020e350(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e350) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e350); p = true; }
    }
    f_8020e350_b(c, 0x8020e350);
    if p { crate::prof::exit(c, 0x8020e350); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e350_at(c: &mut Cpu, pc: u32) { f_8020e350_b(c, pc) }
#[inline(always)]
fn f_8020e350_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e350;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e350, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e350 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e360; continue; }
                pc = 0x8020e358;
            }
            0x8020e358 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e360;
            }
            0x8020e360 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xd));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e350: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetGetFriendFlag (0x8020e368, 24 bytes)
#[inline(never)]
pub fn f_8020e368(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e368) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e368); p = true; }
    }
    f_8020e368_b(c, 0x8020e368);
    if p { crate::prof::exit(c, 0x8020e368); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e368_at(c: &mut Cpu, pc: u32) { f_8020e368_b(c, pc) }
#[inline(always)]
fn f_8020e368_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e368;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e368, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e368 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e378; continue; }
                pc = 0x8020e370;
            }
            0x8020e370 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e378;
            }
            0x8020e378 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xc));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e368: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetGetNekoniKobanFlag (0x8020e380, 24 bytes)
#[inline(never)]
pub fn f_8020e380(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e380) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e380); p = true; }
    }
    f_8020e380_b(c, 0x8020e380);
    if p { crate::prof::exit(c, 0x8020e380); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e380_at(c: &mut Cpu, pc: u32) { f_8020e380_b(c, pc) }
#[inline(always)]
fn f_8020e380_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e380;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e380, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e380 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e390; continue; }
                pc = 0x8020e388;
            }
            0x8020e388 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e390;
            }
            0x8020e390 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xb));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e380: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetGetOkaneFlag (0x8020e398, 24 bytes)
#[inline(never)]
pub fn f_8020e398(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e398) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e398); p = true; }
    }
    f_8020e398_b(c, 0x8020e398);
    if p { crate::prof::exit(c, 0x8020e398); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e398_at(c: &mut Cpu, pc: u32) { f_8020e398_b(c, pc) }
#[inline(always)]
fn f_8020e398_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e398;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e398, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e398 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e3a8; continue; }
                pc = 0x8020e3a0;
            }
            0x8020e3a0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e3a8;
            }
            0x8020e3a8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x9));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e398: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetOkanePoolFlag (0x8020e3b0, 24 bytes)
#[inline(never)]
pub fn f_8020e3b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e3b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e3b0); p = true; }
    }
    f_8020e3b0_b(c, 0x8020e3b0);
    if p { crate::prof::exit(c, 0x8020e3b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e3b0_at(c: &mut Cpu, pc: u32) { f_8020e3b0_b(c, pc) }
#[inline(always)]
fn f_8020e3b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e3b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e3b0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e3b0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e3c0; continue; }
                pc = 0x8020e3b8;
            }
            0x8020e3b8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e3c0;
            }
            0x8020e3c0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0xa));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e3b0: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetGetExpFlag (0x8020e3c8, 24 bytes)
#[inline(never)]
pub fn f_8020e3c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e3c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e3c8); p = true; }
    }
    f_8020e3c8_b(c, 0x8020e3c8);
    if p { crate::prof::exit(c, 0x8020e3c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e3c8_at(c: &mut Cpu, pc: u32) { f_8020e3c8_b(c, pc) }
#[inline(always)]
fn f_8020e3c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e3c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e3c8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e3c8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e3d8; continue; }
                pc = 0x8020e3d0;
            }
            0x8020e3d0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e3d8;
            }
            0x8020e3d8 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x8));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e3c8: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDrawFlag (0x8020e3e0, 24 bytes)
#[inline(never)]
pub fn f_8020e3e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e3e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e3e0); p = true; }
    }
    f_8020e3e0_b(c, 0x8020e3e0);
    if p { crate::prof::exit(c, 0x8020e3e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e3e0_at(c: &mut Cpu, pc: u32) { f_8020e3e0_b(c, pc) }
#[inline(always)]
fn f_8020e3e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e3e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e3e0, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e3e0 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e3f0; continue; }
                pc = 0x8020e3e8;
            }
            0x8020e3e8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e3f0;
            }
            0x8020e3f0 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x7));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e3e0: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetNigeruFlag (0x8020e3f8, 24 bytes)
#[inline(never)]
pub fn f_8020e3f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e3f8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e3f8); p = true; }
    }
    f_8020e3f8_b(c, 0x8020e3f8);
    if p { crate::prof::exit(c, 0x8020e3f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e3f8_at(c: &mut Cpu, pc: u32) { f_8020e3f8_b(c, pc) }
#[inline(always)]
fn f_8020e3f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e3f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e3f8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e3f8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e408; continue; }
                pc = 0x8020e400;
            }
            0x8020e400 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e408;
            }
            0x8020e408 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x6));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e3f8: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetCallFlag (0x8020e410, 24 bytes)
#[inline(never)]
pub fn f_8020e410(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e410) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e410); p = true; }
    }
    f_8020e410_b(c, 0x8020e410);
    if p { crate::prof::exit(c, 0x8020e410); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e410_at(c: &mut Cpu, pc: u32) { f_8020e410_b(c, pc) }
#[inline(always)]
fn f_8020e410_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e410;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e410, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e410 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e420; continue; }
                pc = 0x8020e418;
            }
            0x8020e418 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e420;
            }
            0x8020e420 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x5));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e410: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetUseItemFlag (0x8020e428, 24 bytes)
#[inline(never)]
pub fn f_8020e428(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e428) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e428); p = true; }
    }
    f_8020e428_b(c, 0x8020e428);
    if p { crate::prof::exit(c, 0x8020e428); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e428_at(c: &mut Cpu, pc: u32) { f_8020e428_b(c, pc) }
#[inline(always)]
fn f_8020e428_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e428;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e428, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e428 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e438; continue; }
                pc = 0x8020e430;
            }
            0x8020e430 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e438;
            }
            0x8020e438 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x4));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e428: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoZukanTukamaetaFlag (0x8020e440, 24 bytes)
#[inline(never)]
pub fn f_8020e440(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e440) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e440); p = true; }
    }
    f_8020e440_b(c, 0x8020e440);
    if p { crate::prof::exit(c, 0x8020e440); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e440_at(c: &mut Cpu, pc: u32) { f_8020e440_b(c, pc) }
#[inline(always)]
fn f_8020e440_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e440;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e440, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e440 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e450; continue; }
                pc = 0x8020e448;
            }
            0x8020e448 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e450;
            }
            0x8020e450 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x3));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e440: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoZukanMitaFlag (0x8020e458, 24 bytes)
#[inline(never)]
pub fn f_8020e458(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e458) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e458); p = true; }
    }
    f_8020e458_b(c, 0x8020e458);
    if p { crate::prof::exit(c, 0x8020e458); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e458_at(c: &mut Cpu, pc: u32) { f_8020e458_b(c, pc) }
#[inline(always)]
fn f_8020e458_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e458;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e458, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e458 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e468; continue; }
                pc = 0x8020e460;
            }
            0x8020e460 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e468;
            }
            0x8020e468 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x2));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e458: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoBadgeCheckFlag (0x8020e470, 24 bytes)
#[inline(never)]
pub fn f_8020e470(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e470) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e470); p = true; }
    }
    f_8020e470_b(c, 0x8020e470);
    if p { crate::prof::exit(c, 0x8020e470); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e470_at(c: &mut Cpu, pc: u32) { f_8020e470_b(c, pc) }
#[inline(always)]
fn f_8020e470_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e470;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e470, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e470 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e480; continue; }
                pc = 0x8020e478;
            }
            0x8020e478 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e480;
            }
            0x8020e480 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3].wrapping_add(0x1));
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e470: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetPtr (0x8020e488, 44 bytes)
#[inline(never)]
pub fn f_8020e488(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e488) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e488); p = true; }
    }
    f_8020e488_b(c, 0x8020e488);
    if p { crate::prof::exit(c, 0x8020e488); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e488_at(c: &mut Cpu, pc: u32) { f_8020e488_b(c, pc) }
#[inline(always)]
fn f_8020e488_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e488;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e488, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e488 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8720));
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x8020e4a4; continue; }
                pc = 0x8020e49c;
            }
            0x8020e49c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e4a4;
            }
            0x8020e4a4 => { ins += 4;
                c.r[4] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8724));
                c.r[0] = c.r[3].rotate_left(5) & 0x1fffe0;
                c.r[3] = c.r[4].wrapping_add(c.r[0]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e488: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetBackSaveDataFlag (0x8020e4b4, 24 bytes)
#[inline(never)]
pub fn f_8020e4b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e4b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e4b4); p = true; }
    }
    f_8020e4b4_b(c, 0x8020e4b4);
    if p { crate::prof::exit(c, 0x8020e4b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e4b4_at(c: &mut Cpu, pc: u32) { f_8020e4b4_b(c, pc) }
#[inline(always)]
fn f_8020e4b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e4b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e4b4, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e4b4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e4c4; continue; }
                pc = 0x8020e4bc;
            }
            0x8020e4bc => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e4c4;
            }
            0x8020e4c4 => { ins += 2;
                c.r[3] = c.lbz_t(ins, c.r[3]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e4b4: bad pc {:#x}", pc),
        }
    }
}

/// fightAbicntFitMinMax (0x8020e4cc, 28 bytes)
#[inline(never)]
pub fn f_8020e4cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e4cc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e4cc); p = true; }
    }
    f_8020e4cc_b(c, 0x8020e4cc);
    if p { crate::prof::exit(c, 0x8020e4cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e4cc_at(c: &mut Cpu, pc: u32) { f_8020e4cc_b(c, pc) }
#[inline(always)]
fn f_8020e4cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e4cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e4cc, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e4cc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x8020e4d8; continue; }
                pc = 0x8020e4d4;
            }
            0x8020e4d4 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x8020e4d8;
            }
            0x8020e4d8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xc);
                if !cr_bit(cr, 1) { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                pc = 0x8020e4e0;
            }
            0x8020e4e0 => { ins += 2;
                c.r[3] = 0xc;
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e4cc: bad pc {:#x}", pc),
        }
    }
}

/// fightAbicntDoKakeWaru (0x8020e4e8, 148 bytes)
#[inline(never)]
pub fn f_8020e4e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8020e4e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e4e8); p = true; }
    }
    f_8020e4e8_b(c, 0x8020e4e8);
    if p { crate::prof::exit(c, 0x8020e4e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e4e8_at(c: &mut Cpu, pc: u32) { f_8020e4e8_b(c, pc) }
#[inline(always)]
fn f_8020e4e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.cycles;
    let mut cr = c.cr;
    c.last_fn = 0x8020e4e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.cycles = ins; c.cr = cr; c.step_event(0x8020e4e8, pc); st = c.steps; ins = c.cycles; cr = c.cr; }
        match pc {
            0x8020e4e8 => { ins += 8;
                c.r[7] = c.lwz_t(ins, c.r[13].wrapping_add(0xffff8548));
                c.r[8] = c.r[3].rotate_left(0) & 0xffff;
                c.r[5] = 0x80370000;
                c.r[6] = c.r[3].rotate_left(1) & 0x1fffe;
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], c.r[7]);
                c.r[0] = c.r[5].wrapping_add(0x5d10);
                c.r[5] = c.r[0].wrapping_add(c.r[6]);
                if cr_bit(cr, 0) { pc = 0x8020e50c; continue; }
                pc = 0x8020e508;
            }
            0x8020e508 => { ins += 1;
                c.r[5] = 0x0;
                pc = 0x8020e50c;
            }
            0x8020e50c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x8020e51c; continue; }
                pc = 0x8020e514;
            }
            0x8020e514 => { ins += 2;
                c.r[6] = 0x0;
                pc = 0x8020e52c; continue;
                pc = 0x8020e51c;
            }
            0x8020e51c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020e528; continue; }
                pc = 0x8020e520;
            }
            0x8020e520 => { ins += 2;
                c.r[6] = 0x0;
                pc = 0x8020e52c; continue;
                pc = 0x8020e528;
            }
            0x8020e528 => { ins += 1;
                c.r[6] = c.lbz_t(ins, c.r[5]);
                pc = 0x8020e52c;
            }
            0x8020e52c => { ins += 6;
                c.r[5] = 0x80370000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], c.r[7]);
                c.r[3] = c.r[3].rotate_left(1) & 0x1fffe;
                c.r[0] = c.r[5].wrapping_add(0x5d10);
                c.r[5] = c.r[0].wrapping_add(c.r[3]);
                if cr_bit(cr, 0) { pc = 0x8020e548; continue; }
                pc = 0x8020e544;
            }
            0x8020e544 => { ins += 1;
                c.r[5] = 0x0;
                pc = 0x8020e548;
            }
            0x8020e548 => { ins += 8;
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                c.r[3] = c.r[4].wrapping_mul(c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x8020e560; continue; }
                pc = 0x8020e558;
            }
            0x8020e558 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x8020e570; continue;
                pc = 0x8020e560;
            }
            0x8020e560 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x8020e56c; continue; }
                pc = 0x8020e564;
            }
            0x8020e564 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x8020e570; continue;
                pc = 0x8020e56c;
            }
            0x8020e56c => { ins += 1;
                c.r[0] = c.lbz_t(ins, c.r[5].wrapping_add(0x1));
                pc = 0x8020e570;
            }
            0x8020e570 => { ins += 42;
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[3] = divwu(c.r[3], c.r[0]);
                if true { { c.steps = st; c.cycles = ins; c.cr = cr; return; } }
                { c.steps = st; c.cycles = ins; c.cr = cr; return; }
            }
            _ => panic!("f_8020e4e8: bad pc {:#x}", pc),
        }
    }
}

