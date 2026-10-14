#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fightWazaWriteJoutaiDataId (0x802097c8, 84 bytes)
#[inline(never)]
pub fn f_802097c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802097c8) { return; }
        if c.prof { crate::prof::enter(c, 0x802097c8); f_802097c8_b(c, 0x802097c8); crate::prof::exit(c, 0x802097c8); return; }
    }
    f_802097c8_b(c, 0x802097c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802097c8_at(c: &mut Cpu, pc: u32) { f_802097c8_b(c, pc) }
#[inline(always)]
fn f_802097c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802097c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802097c8, pc); }
        match pc {
            0x802097c8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x802097ec; f_80119ed0(c);
                pc = 0x802097ec;
            }
            0x802097ec => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2a);
                if !c.crbit(2) { pc = 0x80209808; continue; }
                pc = 0x802097f8;
            }
            0x802097f8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80209808; f_8011b2c0(c);
                pc = 0x80209808;
            }
            0x80209808 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802097c8: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaCheckWriteJoutaiDataId (0x8020981c, 84 bytes)
#[inline(never)]
pub fn f_8020981c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020981c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020981c); f_8020981c_b(c, 0x8020981c); crate::prof::exit(c, 0x8020981c); return; }
    }
    f_8020981c_b(c, 0x8020981c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020981c_at(c: &mut Cpu, pc: u32) { f_8020981c_b(c, pc) }
#[inline(always)]
fn f_8020981c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020981c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020981c, pc); }
        match pc {
            0x8020981c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020983c; f_80119ed0(c);
                pc = 0x8020983c;
            }
            0x8020983c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2a);
                if c.crbit(2) { pc = 0x80209850; continue; }
                pc = 0x80209848;
            }
            0x80209848 => {
                c.r[3] = 0x0;
                pc = 0x8020985c; continue;
                pc = 0x80209850;
            }
            0x80209850 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8020985c; f_8011b444(c);
                pc = 0x8020985c;
            }
            0x8020985c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020981c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaIsJoutaiSousai (0x80209870, 156 bytes)
#[inline(never)]
pub fn f_80209870(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80209870) { return; }
        if c.prof { crate::prof::enter(c, 0x80209870); f_80209870_b(c, 0x80209870); crate::prof::exit(c, 0x80209870); return; }
    }
    f_80209870_b(c, 0x80209870)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209870_at(c: &mut Cpu, pc: u32) { f_80209870_b(c, pc) }
#[inline(always)]
fn f_80209870_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80209870;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80209870, pc); }
        match pc {
            0x80209870 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x41;
                c.lr = 0x8020988c; f_80119ed0(c);
                pc = 0x8020988c;
            }
            0x8020988c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2a);
                if c.crbit(2) { pc = 0x802098a0; continue; }
                pc = 0x80209898;
            }
            0x80209898 => {
                c.r[3] = 0x0;
                pc = 0x802098ac; continue;
                pc = 0x802098a0;
            }
            0x802098a0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x41;
                c.lr = 0x802098ac; f_8011b67c(c);
                pc = 0x802098ac;
            }
            0x802098ac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802098f4; continue; }
                pc = 0x802098b8;
            }
            0x802098b8 => {
                c.r[3] = 0x42;
                c.lr = 0x802098c0; f_80119ed0(c);
                pc = 0x802098c0;
            }
            0x802098c0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2a);
                if c.crbit(2) { pc = 0x802098d4; continue; }
                pc = 0x802098cc;
            }
            0x802098cc => {
                c.r[3] = 0x0;
                pc = 0x802098e0; continue;
                pc = 0x802098d4;
            }
            0x802098d4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x42;
                c.lr = 0x802098e0; f_8011b67c(c);
                pc = 0x802098e0;
            }
            0x802098e0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x802098f4; continue; }
                pc = 0x802098ec;
            }
            0x802098ec => {
                c.r[3] = 0x1;
                pc = 0x802098f8; continue;
                pc = 0x802098f4;
            }
            0x802098f4 => {
                c.r[3] = 0x0;
                pc = 0x802098f8;
            }
            0x802098f8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80209870: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaIsJoutaiDataId (0x8020990c, 84 bytes)
#[inline(never)]
pub fn f_8020990c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020990c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020990c); f_8020990c_b(c, 0x8020990c); crate::prof::exit(c, 0x8020990c); return; }
    }
    f_8020990c_b(c, 0x8020990c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020990c_at(c: &mut Cpu, pc: u32) { f_8020990c_b(c, pc) }
#[inline(always)]
fn f_8020990c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020990c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020990c, pc); }
        match pc {
            0x8020990c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020992c; f_80119ed0(c);
                pc = 0x8020992c;
            }
            0x8020992c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2a);
                if c.crbit(2) { pc = 0x80209940; continue; }
                pc = 0x80209938;
            }
            0x80209938 => {
                c.r[3] = 0x0;
                pc = 0x8020994c; continue;
                pc = 0x80209940;
            }
            0x80209940 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8020994c; f_8011b67c(c);
                pc = 0x8020994c;
            }
            0x8020994c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020990c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaInitJoutaiDataId (0x80209960, 76 bytes)
#[inline(never)]
pub fn f_80209960(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80209960) { return; }
        if c.prof { crate::prof::enter(c, 0x80209960); f_80209960_b(c, 0x80209960); crate::prof::exit(c, 0x80209960); return; }
    }
    f_80209960_b(c, 0x80209960)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209960_at(c: &mut Cpu, pc: u32) { f_80209960_b(c, pc) }
#[inline(always)]
fn f_80209960_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80209960;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80209960, pc); }
        match pc {
            0x80209960 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80209980; f_80119ed0(c);
                pc = 0x80209980;
            }
            0x80209980 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2a);
                if !c.crbit(2) { pc = 0x80209998; continue; }
                pc = 0x8020998c;
            }
            0x8020998c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80209998; f_8011b788(c);
                pc = 0x80209998;
            }
            0x80209998 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80209960: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaCreate (0x802099ac, 624 bytes)
#[inline(never)]
pub fn f_802099ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x802099ac) { return; }
        if c.prof { crate::prof::enter(c, 0x802099ac); f_802099ac_b(c, 0x802099ac); crate::prof::exit(c, 0x802099ac); return; }
    }
    f_802099ac_b(c, 0x802099ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_802099ac_at(c: &mut Cpu, pc: u32) { f_802099ac_b(c, pc) }
#[inline(always)]
fn f_802099ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x802099ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x802099ac, pc); }
        match pc {
            0x802099ac => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                c.r[30] = c.r[7] | c.r[7];
                if c.crbit(2) { pc = 0x80209c08; continue; }
                pc = 0x802099d4;
            }
            0x802099d4 => {
                if c.crbit(2) { pc = 0x80209b34; continue; }
                pc = 0x802099d8;
            }
            0x802099d8 => {
                c.r[4] = 0x0;
                c.r[5] = 0x26;
                c.r[6] = 0x0;
                c.r[7] = 0xffffffff;
                c.lr = 0x802099ec; f_8011bbd8(c);
                pc = 0x802099ec;
            }
            0x802099ec => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209a04; f_8011bbd8(c);
                pc = 0x80209a04;
            }
            0x80209a04 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209a1c; f_8011bbd8(c);
                pc = 0x80209a1c;
            }
            0x80209a1c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209a34; f_8011bbd8(c);
                pc = 0x80209a34;
            }
            0x80209a34 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2a;
                c.r[6] = 0x0;
                c.lr = 0x80209a48; f_8011beb4(c);
                pc = 0x80209a48;
            }
            0x80209a48 => {
                c.r[4] = 0x9;
                c.lr = 0x80209a50; f_8011b950(c);
                pc = 0x80209a50;
            }
            0x80209a50 => {
                c.r[3] = 0x3f;
                c.lr = 0x80209a58; f_80119ed0(c);
                pc = 0x80209a58;
            }
            0x80209a58 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2a);
                if !c.crbit(2) { pc = 0x80209a74; continue; }
                pc = 0x80209a64;
            }
            0x80209a64 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x3f;
                c.r[5] = 0x0;
                c.lr = 0x80209a74; f_8011b2c0(c);
                pc = 0x80209a74;
            }
            0x80209a74 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2b;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209a8c; f_8011bbd8(c);
                pc = 0x80209a8c;
            }
            0x80209a8c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209aa4; f_8011bbd8(c);
                pc = 0x80209aa4;
            }
            0x80209aa4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209abc; f_8011bbd8(c);
                pc = 0x80209abc;
            }
            0x80209abc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2e;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209ad4; f_8011bbd8(c);
                pc = 0x80209ad4;
            }
            0x80209ad4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209aec; f_8011bbd8(c);
                pc = 0x80209aec;
            }
            0x80209aec => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.r[7] = 0x9;
                c.lr = 0x80209b04; f_8011bbd8(c);
                pc = 0x80209b04;
            }
            0x80209b04 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x31;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209b1c; f_8011bbd8(c);
                pc = 0x80209b1c;
            }
            0x80209b1c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x32;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209b34; f_8011bbd8(c);
                pc = 0x80209b34;
            }
            0x80209b34 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[28] as u8 as i8 as i32 as u32;
                c.r[4] = 0x0;
                c.r[5] = 0x26;
                c.r[6] = 0x0;
                c.lr = 0x80209b4c; f_8011bbd8(c);
                pc = 0x80209b4c;
            }
            0x80209b4c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[31].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.lr = 0x80209b64; f_8011bbd8(c);
                pc = 0x80209b64;
            }
            0x80209b64 => {
                c.r[31] = c.r[29].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x80209b80; f_8011bbd8(c);
                pc = 0x80209b80;
            }
            0x80209b80 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.lr = 0x80209b98; f_8011bbd8(c);
                pc = 0x80209b98;
            }
            0x80209b98 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x80209bac; f_8011beb4(c);
                pc = 0x80209bac;
            }
            0x80209bac => {
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80209bc4; f_8011bbd8(c);
                pc = 0x80209bc4;
            }
            0x80209bc4 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x80209bd8; f_8011beb4(c);
                pc = 0x80209bd8;
            }
            0x80209bd8 => {
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.lr = 0x80209bf0; f_8011bbd8(c);
                pc = 0x80209bf0;
            }
            0x80209bf0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[30].rotate_left(0) & 0xff;
                c.r[4] = 0x0;
                c.r[5] = 0x32;
                c.r[6] = 0x0;
                c.lr = 0x80209c08; f_8011bbd8(c);
                pc = 0x80209c08;
            }
            0x80209c08 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_802099ac: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaSetUseWazaStatus (0x80209c1c, 152 bytes)
#[inline(never)]
pub fn f_80209c1c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80209c1c) { return; }
        if c.prof { crate::prof::enter(c, 0x80209c1c); f_80209c1c_b(c, 0x80209c1c); crate::prof::exit(c, 0x80209c1c); return; }
    }
    f_80209c1c_b(c, 0x80209c1c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209c1c_at(c: &mut Cpu, pc: u32) { f_80209c1c_b(c, pc) }
#[inline(always)]
fn f_80209c1c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80209c1c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80209c1c, pc); }
        match pc {
            0x80209c1c => {
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
                c.lr = 0x80209c48; f_8011bbd8(c);
                pc = 0x80209c48;
            }
            0x80209c48 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x80209c5c; f_8011beb4(c);
                pc = 0x80209c5c;
            }
            0x80209c5c => {
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.lr = 0x80209c74; f_8011bbd8(c);
                pc = 0x80209c74;
            }
            0x80209c74 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x80209c88; f_8011beb4(c);
                pc = 0x80209c88;
            }
            0x80209c88 => {
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.lr = 0x80209ca0; f_8011bbd8(c);
                pc = 0x80209ca0;
            }
            0x80209ca0 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80209c1c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaCheckValid (0x80209cb4, 220 bytes)
#[inline(never)]
pub fn f_80209cb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80209cb4) { return; }
        if c.prof { crate::prof::enter(c, 0x80209cb4); f_80209cb4_b(c, 0x80209cb4); crate::prof::exit(c, 0x80209cb4); return; }
    }
    f_80209cb4_b(c, 0x80209cb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209cb4_at(c: &mut Cpu, pc: u32) { f_80209cb4_b(c, pc) }
#[inline(always)]
fn f_80209cb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80209cb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80209cb4, pc); }
        match pc {
            0x80209cb4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if !c.crbit(2) { pc = 0x80209cd4; continue; }
                pc = 0x80209ccc;
            }
            0x80209ccc => {
                c.r[3] = 0x0;
                pc = 0x80209d7c; continue;
                pc = 0x80209cd4;
            }
            0x80209cd4 => {
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x80209ce4; f_8011beb4(c);
                pc = 0x80209ce4;
            }
            0x80209ce4 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80209cf4; continue; }
                pc = 0x80209cec;
            }
            0x80209cec => {
                c.r[3] = 0x0;
                pc = 0x80209d7c; continue;
                pc = 0x80209cf4;
            }
            0x80209cf4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x80209d08; f_8011beb4(c);
                pc = 0x80209d08;
            }
            0x80209d08 => {
                c.cmps(0, c.r[3], 0x163);
                if !c.crbit(2) { pc = 0x80209d18; continue; }
                pc = 0x80209d10;
            }
            0x80209d10 => {
                c.r[3] = 0x0;
                pc = 0x80209d7c; continue;
                pc = 0x80209d18;
            }
            0x80209d18 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.lr = 0x80209d2c; f_8011beb4(c);
                pc = 0x80209d2c;
            }
            0x80209d2c => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80209d3c; continue; }
                pc = 0x80209d34;
            }
            0x80209d34 => {
                c.r[3] = 0x0;
                pc = 0x80209d7c; continue;
                pc = 0x80209d3c;
            }
            0x80209d3c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.lr = 0x80209d50; f_8011beb4(c);
                pc = 0x80209d50;
            }
            0x80209d50 => {
                c.cmps(0, c.r[3], 0x163);
                if !c.crbit(2) { pc = 0x80209d60; continue; }
                pc = 0x80209d58;
            }
            0x80209d58 => {
                c.r[3] = 0x0;
                pc = 0x80209d7c; continue;
                pc = 0x80209d60;
            }
            0x80209d60 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.lr = 0x80209d74; f_8011beb4(c);
                pc = 0x80209d74;
            }
            0x80209d74 => {
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[0], c.r[3], ci); }
                pc = 0x80209d7c;
            }
            0x80209d7c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80209cb4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaInit (0x80209d90, 392 bytes)
#[inline(never)]
pub fn f_80209d90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80209d90) { return; }
        if c.prof { crate::prof::enter(c, 0x80209d90); f_80209d90_b(c, 0x80209d90); crate::prof::exit(c, 0x80209d90); return; }
    }
    f_80209d90_b(c, 0x80209d90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209d90_at(c: &mut Cpu, pc: u32) { f_80209d90_b(c, pc) }
#[inline(always)]
fn f_80209d90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80209d90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80209d90, pc); }
        match pc {
            0x80209d90 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x80209f04; continue; }
                pc = 0x80209da8;
            }
            0x80209da8 => {
                c.r[4] = 0x0;
                c.r[5] = 0x26;
                c.r[6] = 0x0;
                c.r[7] = 0xffffffff;
                c.lr = 0x80209dbc; f_8011bbd8(c);
                pc = 0x80209dbc;
            }
            0x80209dbc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209dd4; f_8011bbd8(c);
                pc = 0x80209dd4;
            }
            0x80209dd4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209dec; f_8011bbd8(c);
                pc = 0x80209dec;
            }
            0x80209dec => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209e04; f_8011bbd8(c);
                pc = 0x80209e04;
            }
            0x80209e04 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2a;
                c.r[6] = 0x0;
                c.lr = 0x80209e18; f_8011beb4(c);
                pc = 0x80209e18;
            }
            0x80209e18 => {
                c.r[4] = 0x9;
                c.lr = 0x80209e20; f_8011b950(c);
                pc = 0x80209e20;
            }
            0x80209e20 => {
                c.r[3] = 0x3f;
                c.lr = 0x80209e28; f_80119ed0(c);
                pc = 0x80209e28;
            }
            0x80209e28 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2a);
                if !c.crbit(2) { pc = 0x80209e44; continue; }
                pc = 0x80209e34;
            }
            0x80209e34 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3f;
                c.r[5] = 0x0;
                c.lr = 0x80209e44; f_8011b2c0(c);
                pc = 0x80209e44;
            }
            0x80209e44 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2b;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209e5c; f_8011bbd8(c);
                pc = 0x80209e5c;
            }
            0x80209e5c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209e74; f_8011bbd8(c);
                pc = 0x80209e74;
            }
            0x80209e74 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2d;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209e8c; f_8011bbd8(c);
                pc = 0x80209e8c;
            }
            0x80209e8c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2e;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209ea4; f_8011bbd8(c);
                pc = 0x80209ea4;
            }
            0x80209ea4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2f;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209ebc; f_8011bbd8(c);
                pc = 0x80209ebc;
            }
            0x80209ebc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.r[7] = 0x9;
                c.lr = 0x80209ed4; f_8011bbd8(c);
                pc = 0x80209ed4;
            }
            0x80209ed4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x31;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209eec; f_8011bbd8(c);
                pc = 0x80209eec;
            }
            0x80209eec => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x32;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80209f04; f_8011bbd8(c);
                pc = 0x80209f04;
            }
            0x80209f04 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80209d90: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaInitLoop (0x80209f18, 148 bytes)
#[inline(never)]
pub fn f_80209f18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80209f18) { return; }
        if c.prof { crate::prof::enter(c, 0x80209f18); f_80209f18_b(c, 0x80209f18); crate::prof::exit(c, 0x80209f18); return; }
    }
    f_80209f18_b(c, 0x80209f18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209f18_at(c: &mut Cpu, pc: u32) { f_80209f18_b(c, pc) }
#[inline(always)]
fn f_80209f18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80209f18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80209f18, pc); }
        match pc {
            0x80209f18 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x2a;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80209f3c; f_8011beb4(c);
                pc = 0x80209f3c;
            }
            0x80209f3c => {
                c.r[4] = 0x9;
                c.lr = 0x80209f44; f_8011b950(c);
                pc = 0x80209f44;
            }
            0x80209f44 => {
                c.r[3] = 0x3f;
                c.lr = 0x80209f4c; f_80119ed0(c);
                pc = 0x80209f4c;
            }
            0x80209f4c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2a);
                if !c.crbit(2) { pc = 0x80209f68; continue; }
                pc = 0x80209f58;
            }
            0x80209f58 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3f;
                c.r[5] = 0x0;
                c.lr = 0x80209f68; f_8011b2c0(c);
                pc = 0x80209f68;
            }
            0x80209f68 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2b;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209f80; f_8011bbd8(c);
                pc = 0x80209f80;
            }
            0x80209f80 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x2c;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80209f98; f_8011bbd8(c);
                pc = 0x80209f98;
            }
            0x80209f98 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80209f18: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaInitJoutai (0x80209fac, 100 bytes)
#[inline(never)]
pub fn f_80209fac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80209fac) { return; }
        if c.prof { crate::prof::enter(c, 0x80209fac); f_80209fac_b(c, 0x80209fac); crate::prof::exit(c, 0x80209fac); return; }
    }
    f_80209fac_b(c, 0x80209fac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80209fac_at(c: &mut Cpu, pc: u32) { f_80209fac_b(c, pc) }
#[inline(always)]
fn f_80209fac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80209fac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80209fac, pc); }
        match pc {
            0x80209fac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x2a;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80209fd0; f_8011beb4(c);
                pc = 0x80209fd0;
            }
            0x80209fd0 => {
                c.r[4] = 0x9;
                c.lr = 0x80209fd8; f_8011b950(c);
                pc = 0x80209fd8;
            }
            0x80209fd8 => {
                c.r[3] = 0x3f;
                c.lr = 0x80209fe0; f_80119ed0(c);
                pc = 0x80209fe0;
            }
            0x80209fe0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2a);
                if !c.crbit(2) { pc = 0x80209ffc; continue; }
                pc = 0x80209fec;
            }
            0x80209fec => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x3f;
                c.r[5] = 0x0;
                c.lr = 0x80209ffc; f_8011b2c0(c);
                pc = 0x80209ffc;
            }
            0x80209ffc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80209fac: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaHitKakurituDataBiosGetWaru (0x8020a010, 24 bytes)
#[inline(never)]
pub fn f_8020a010(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a010) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a010); f_8020a010_b(c, 0x8020a010); crate::prof::exit(c, 0x8020a010); return; }
    }
    f_8020a010_b(c, 0x8020a010)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a010_at(c: &mut Cpu, pc: u32) { f_8020a010_b(c, pc) }
#[inline(always)]
fn f_8020a010_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a010;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a010, pc); }
        match pc {
            0x8020a010 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a020; continue; }
                pc = 0x8020a018;
            }
            0x8020a018 => {
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x8020a020;
            }
            0x8020a020 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a010: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaHitKakurituDataBiosGetKake (0x8020a028, 24 bytes)
#[inline(never)]
pub fn f_8020a028(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a028) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a028); f_8020a028_b(c, 0x8020a028); crate::prof::exit(c, 0x8020a028); return; }
    }
    f_8020a028_b(c, 0x8020a028)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a028_at(c: &mut Cpu, pc: u32) { f_8020a028_b(c, pc) }
#[inline(always)]
fn f_8020a028_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a028;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a028, pc); }
        match pc {
            0x8020a028 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a038; continue; }
                pc = 0x8020a030;
            }
            0x8020a030 => {
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x8020a038;
            }
            0x8020a038 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a028: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaHitKakurituDataBiosGetPtr (0x8020a040, 40 bytes)
#[inline(never)]
pub fn f_8020a040(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a040) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a040); f_8020a040_b(c, 0x8020a040); crate::prof::exit(c, 0x8020a040); return; }
    }
    f_8020a040_b(c, 0x8020a040)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a040_at(c: &mut Cpu, pc: u32) { f_8020a040_b(c, pc) }
#[inline(always)]
fn f_8020a040_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a040;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a040, pc); }
        match pc {
            0x8020a040 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8550));
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = 0x80370000;
                c.r[3] = c.r[3].rotate_left(1) & 0x1fffe;
                c.cmpu(0, c.r[5], c.r[0]);
                c.r[0] = c.r[4].wrapping_add(0x5dd0);
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                if c.crbit(0) { return; }
                pc = 0x8020a060;
            }
            0x8020a060 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020a040: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaCriticalDataBiosGetBunbo (0x8020a068, 24 bytes)
#[inline(never)]
pub fn f_8020a068(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a068) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a068); f_8020a068_b(c, 0x8020a068); crate::prof::exit(c, 0x8020a068); return; }
    }
    f_8020a068_b(c, 0x8020a068)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a068_at(c: &mut Cpu, pc: u32) { f_8020a068_b(c, pc) }
#[inline(always)]
fn f_8020a068_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a068;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a068, pc); }
        match pc {
            0x8020a068 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a078; continue; }
                pc = 0x8020a070;
            }
            0x8020a070 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a078;
            }
            0x8020a078 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a068: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaCriticalDataBiosGetPtr (0x8020a080, 36 bytes)
#[inline(never)]
pub fn f_8020a080(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a080) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a080); f_8020a080_b(c, 0x8020a080); crate::prof::exit(c, 0x8020a080); return; }
    }
    f_8020a080_b(c, 0x8020a080)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a080_at(c: &mut Cpu, pc: u32) { f_8020a080_b(c, pc) }
#[inline(always)]
fn f_8020a080_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a080;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a080, pc); }
        match pc {
            0x8020a080 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8540));
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x80480000;
                c.cmpu(0, c.r[4], c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0xffff8d58);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if c.crbit(0) { return; }
                pc = 0x8020a09c;
            }
            0x8020a09c => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8020a080: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetAutoMakeFlag (0x8020a0a4, 16 bytes)
#[inline(never)]
pub fn f_8020a0a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a0a4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0a4); f_8020a0a4_b(c, 0x8020a0a4); crate::prof::exit(c, 0x8020a0a4); return; }
    }
    f_8020a0a4_b(c, 0x8020a0a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0a4_at(c: &mut Cpu, pc: u32) { f_8020a0a4_b(c, pc) }
#[inline(always)]
fn f_8020a0a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a0a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a0a4, pc); }
        match pc {
            0x8020a0a4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a0ac;
            }
            0x8020a0ac => {
                c.stb(c.r[3].wrapping_add(0xa9), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a0a4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetKaisuu (0x8020a0b4, 16 bytes)
#[inline(never)]
pub fn f_8020a0b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a0b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0b4); f_8020a0b4_b(c, 0x8020a0b4); crate::prof::exit(c, 0x8020a0b4); return; }
    }
    f_8020a0b4_b(c, 0x8020a0b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0b4_at(c: &mut Cpu, pc: u32) { f_8020a0b4_b(c, pc) }
#[inline(always)]
fn f_8020a0b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a0b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a0b4, pc); }
        match pc {
            0x8020a0b4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a0bc;
            }
            0x8020a0bc => {
                c.stb(c.r[3].wrapping_add(0xa8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a0b4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetZokusei (0x8020a0c4, 16 bytes)
#[inline(never)]
pub fn f_8020a0c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a0c4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0c4); f_8020a0c4_b(c, 0x8020a0c4); crate::prof::exit(c, 0x8020a0c4); return; }
    }
    f_8020a0c4_b(c, 0x8020a0c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0c4_at(c: &mut Cpu, pc: u32) { f_8020a0c4_b(c, pc) }
#[inline(always)]
fn f_8020a0c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a0c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a0c4, pc); }
        match pc {
            0x8020a0c4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a0cc;
            }
            0x8020a0cc => {
                c.sth(c.r[3].wrapping_add(0xa6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a0c4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetIryoku (0x8020a0d4, 16 bytes)
#[inline(never)]
pub fn f_8020a0d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a0d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0d4); f_8020a0d4_b(c, 0x8020a0d4); crate::prof::exit(c, 0x8020a0d4); return; }
    }
    f_8020a0d4_b(c, 0x8020a0d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0d4_at(c: &mut Cpu, pc: u32) { f_8020a0d4_b(c, pc) }
#[inline(always)]
fn f_8020a0d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a0d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a0d4, pc); }
        match pc {
            0x8020a0d4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a0dc;
            }
            0x8020a0dc => {
                c.sth(c.r[3].wrapping_add(0xa4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a0d4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetHitDamage (0x8020a0e4, 16 bytes)
#[inline(never)]
pub fn f_8020a0e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a0e4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0e4); f_8020a0e4_b(c, 0x8020a0e4); crate::prof::exit(c, 0x8020a0e4); return; }
    }
    f_8020a0e4_b(c, 0x8020a0e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0e4_at(c: &mut Cpu, pc: u32) { f_8020a0e4_b(c, pc) }
#[inline(always)]
fn f_8020a0e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a0e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a0e4, pc); }
        match pc {
            0x8020a0e4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a0ec;
            }
            0x8020a0ec => {
                c.stw(c.r[3].wrapping_add(0xa0), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a0e4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetDamage (0x8020a0f4, 16 bytes)
#[inline(never)]
pub fn f_8020a0f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a0f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a0f4); f_8020a0f4_b(c, 0x8020a0f4); crate::prof::exit(c, 0x8020a0f4); return; }
    }
    f_8020a0f4_b(c, 0x8020a0f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a0f4_at(c: &mut Cpu, pc: u32) { f_8020a0f4_b(c, pc) }
#[inline(always)]
fn f_8020a0f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a0f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a0f4, pc); }
        match pc {
            0x8020a0f4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a0fc;
            }
            0x8020a0fc => {
                c.stw(c.r[3].wrapping_add(0x9c), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a0f4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetDamageValue (0x8020a104, 16 bytes)
#[inline(never)]
pub fn f_8020a104(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a104) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a104); f_8020a104_b(c, 0x8020a104); crate::prof::exit(c, 0x8020a104); return; }
    }
    f_8020a104_b(c, 0x8020a104)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a104_at(c: &mut Cpu, pc: u32) { f_8020a104_b(c, pc) }
#[inline(always)]
fn f_8020a104_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a104;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a104, pc); }
        match pc {
            0x8020a104 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a10c;
            }
            0x8020a10c => {
                c.stb(c.r[3].wrapping_add(0x99), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a104: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetCritical (0x8020a114, 16 bytes)
#[inline(never)]
pub fn f_8020a114(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a114) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a114); f_8020a114_b(c, 0x8020a114); crate::prof::exit(c, 0x8020a114); return; }
    }
    f_8020a114_b(c, 0x8020a114)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a114_at(c: &mut Cpu, pc: u32) { f_8020a114_b(c, pc) }
#[inline(always)]
fn f_8020a114_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a114;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a114, pc); }
        match pc {
            0x8020a114 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a11c;
            }
            0x8020a11c => {
                c.stb(c.r[3].wrapping_add(0x98), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a114: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetTargetDataId (0x8020a124, 16 bytes)
#[inline(never)]
pub fn f_8020a124(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a124) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a124); f_8020a124_b(c, 0x8020a124); crate::prof::exit(c, 0x8020a124); return; }
    }
    f_8020a124_b(c, 0x8020a124)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a124_at(c: &mut Cpu, pc: u32) { f_8020a124_b(c, pc) }
#[inline(always)]
fn f_8020a124_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a124;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a124, pc); }
        match pc {
            0x8020a124 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a12c;
            }
            0x8020a12c => {
                c.sth(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a124: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetUseWazaDataId (0x8020a134, 16 bytes)
#[inline(never)]
pub fn f_8020a134(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a134) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a134); f_8020a134_b(c, 0x8020a134); crate::prof::exit(c, 0x8020a134); return; }
    }
    f_8020a134_b(c, 0x8020a134)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a134_at(c: &mut Cpu, pc: u32) { f_8020a134_b(c, pc) }
#[inline(always)]
fn f_8020a134_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a134;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a134, pc); }
        match pc {
            0x8020a134 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a13c;
            }
            0x8020a13c => {
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a134: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetMotoWazaDataId (0x8020a144, 16 bytes)
#[inline(never)]
pub fn f_8020a144(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a144) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a144); f_8020a144_b(c, 0x8020a144); crate::prof::exit(c, 0x8020a144); return; }
    }
    f_8020a144_b(c, 0x8020a144)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a144_at(c: &mut Cpu, pc: u32) { f_8020a144_b(c, pc) }
#[inline(always)]
fn f_8020a144_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a144;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a144, pc); }
        match pc {
            0x8020a144 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a14c;
            }
            0x8020a14c => {
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a144: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosSetWazaBanme (0x8020a154, 16 bytes)
#[inline(never)]
pub fn f_8020a154(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a154) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a154); f_8020a154_b(c, 0x8020a154); crate::prof::exit(c, 0x8020a154); return; }
    }
    f_8020a154_b(c, 0x8020a154)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a154_at(c: &mut Cpu, pc: u32) { f_8020a154_b(c, pc) }
#[inline(always)]
fn f_8020a154_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a154;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a154, pc); }
        match pc {
            0x8020a154 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a15c;
            }
            0x8020a15c => {
                c.stb(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a154: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetAutoMakeFlag (0x8020a164, 24 bytes)
#[inline(never)]
pub fn f_8020a164(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a164) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a164); f_8020a164_b(c, 0x8020a164); crate::prof::exit(c, 0x8020a164); return; }
    }
    f_8020a164_b(c, 0x8020a164)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a164_at(c: &mut Cpu, pc: u32) { f_8020a164_b(c, pc) }
#[inline(always)]
fn f_8020a164_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a164;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a164, pc); }
        match pc {
            0x8020a164 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a174; continue; }
                pc = 0x8020a16c;
            }
            0x8020a16c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a174;
            }
            0x8020a174 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xa9));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a164: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetKaisuu (0x8020a17c, 24 bytes)
#[inline(never)]
pub fn f_8020a17c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a17c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a17c); f_8020a17c_b(c, 0x8020a17c); crate::prof::exit(c, 0x8020a17c); return; }
    }
    f_8020a17c_b(c, 0x8020a17c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a17c_at(c: &mut Cpu, pc: u32) { f_8020a17c_b(c, pc) }
#[inline(always)]
fn f_8020a17c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a17c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a17c, pc); }
        match pc {
            0x8020a17c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a18c; continue; }
                pc = 0x8020a184;
            }
            0x8020a184 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a18c;
            }
            0x8020a18c => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xa8));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a17c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetZokusei (0x8020a194, 24 bytes)
#[inline(never)]
pub fn f_8020a194(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a194) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a194); f_8020a194_b(c, 0x8020a194); crate::prof::exit(c, 0x8020a194); return; }
    }
    f_8020a194_b(c, 0x8020a194)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a194_at(c: &mut Cpu, pc: u32) { f_8020a194_b(c, pc) }
#[inline(always)]
fn f_8020a194_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a194;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a194, pc); }
        match pc {
            0x8020a194 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a1a4; continue; }
                pc = 0x8020a19c;
            }
            0x8020a19c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a1a4;
            }
            0x8020a1a4 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0xa6));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a194: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetIryoku (0x8020a1ac, 24 bytes)
#[inline(never)]
pub fn f_8020a1ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a1ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a1ac); f_8020a1ac_b(c, 0x8020a1ac); crate::prof::exit(c, 0x8020a1ac); return; }
    }
    f_8020a1ac_b(c, 0x8020a1ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a1ac_at(c: &mut Cpu, pc: u32) { f_8020a1ac_b(c, pc) }
#[inline(always)]
fn f_8020a1ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a1ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a1ac, pc); }
        match pc {
            0x8020a1ac => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a1bc; continue; }
                pc = 0x8020a1b4;
            }
            0x8020a1b4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a1bc;
            }
            0x8020a1bc => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0xa4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a1ac: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetHitDamage (0x8020a1c4, 24 bytes)
#[inline(never)]
pub fn f_8020a1c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a1c4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a1c4); f_8020a1c4_b(c, 0x8020a1c4); crate::prof::exit(c, 0x8020a1c4); return; }
    }
    f_8020a1c4_b(c, 0x8020a1c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a1c4_at(c: &mut Cpu, pc: u32) { f_8020a1c4_b(c, pc) }
#[inline(always)]
fn f_8020a1c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a1c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a1c4, pc); }
        match pc {
            0x8020a1c4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a1d4; continue; }
                pc = 0x8020a1cc;
            }
            0x8020a1cc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a1d4;
            }
            0x8020a1d4 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xa0));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a1c4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetDamage (0x8020a1dc, 24 bytes)
#[inline(never)]
pub fn f_8020a1dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a1dc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a1dc); f_8020a1dc_b(c, 0x8020a1dc); crate::prof::exit(c, 0x8020a1dc); return; }
    }
    f_8020a1dc_b(c, 0x8020a1dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a1dc_at(c: &mut Cpu, pc: u32) { f_8020a1dc_b(c, pc) }
#[inline(always)]
fn f_8020a1dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a1dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a1dc, pc); }
        match pc {
            0x8020a1dc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a1ec; continue; }
                pc = 0x8020a1e4;
            }
            0x8020a1e4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a1ec;
            }
            0x8020a1ec => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x9c));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a1dc: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetDamageValue (0x8020a1f4, 24 bytes)
#[inline(never)]
pub fn f_8020a1f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a1f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a1f4); f_8020a1f4_b(c, 0x8020a1f4); crate::prof::exit(c, 0x8020a1f4); return; }
    }
    f_8020a1f4_b(c, 0x8020a1f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a1f4_at(c: &mut Cpu, pc: u32) { f_8020a1f4_b(c, pc) }
#[inline(always)]
fn f_8020a1f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a1f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a1f4, pc); }
        match pc {
            0x8020a1f4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a204; continue; }
                pc = 0x8020a1fc;
            }
            0x8020a1fc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a204;
            }
            0x8020a204 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x99));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a1f4: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetCritical (0x8020a20c, 24 bytes)
#[inline(never)]
pub fn f_8020a20c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a20c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a20c); f_8020a20c_b(c, 0x8020a20c); crate::prof::exit(c, 0x8020a20c); return; }
    }
    f_8020a20c_b(c, 0x8020a20c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a20c_at(c: &mut Cpu, pc: u32) { f_8020a20c_b(c, pc) }
#[inline(always)]
fn f_8020a20c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a20c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a20c, pc); }
        match pc {
            0x8020a20c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a21c; continue; }
                pc = 0x8020a214;
            }
            0x8020a214 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a21c;
            }
            0x8020a21c => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x98));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a20c: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetJoutaiPtr (0x8020a224, 52 bytes)
#[inline(never)]
pub fn f_8020a224(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a224) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a224); f_8020a224_b(c, 0x8020a224); crate::prof::exit(c, 0x8020a224); return; }
    }
    f_8020a224_b(c, 0x8020a224)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a224_at(c: &mut Cpu, pc: u32) { f_8020a224_b(c, pc) }
#[inline(always)]
fn f_8020a224_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a224;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a224, pc); }
        match pc {
            0x8020a224 => {
                c.r[5] = c.r[3] | c.r[3];
                c.rc(c.r[5]);
                if !c.crbit(2) { pc = 0x8020a234; continue; }
                pc = 0x8020a22c;
            }
            0x8020a22c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a234;
            }
            0x8020a234 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x9);
                if c.crbit(0) { pc = 0x8020a248; continue; }
                pc = 0x8020a240;
            }
            0x8020a240 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a248;
            }
            0x8020a248 => {
                c.r[3] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(0x8);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a224: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetTargetDataId (0x8020a258, 24 bytes)
#[inline(never)]
pub fn f_8020a258(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a258) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a258); f_8020a258_b(c, 0x8020a258); crate::prof::exit(c, 0x8020a258); return; }
    }
    f_8020a258_b(c, 0x8020a258)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a258_at(c: &mut Cpu, pc: u32) { f_8020a258_b(c, pc) }
#[inline(always)]
fn f_8020a258_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a258;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a258, pc); }
        match pc {
            0x8020a258 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a268; continue; }
                pc = 0x8020a260;
            }
            0x8020a260 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a268;
            }
            0x8020a268 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x6));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a258: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetUseWazaDataId (0x8020a270, 24 bytes)
#[inline(never)]
pub fn f_8020a270(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a270) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a270); f_8020a270_b(c, 0x8020a270); crate::prof::exit(c, 0x8020a270); return; }
    }
    f_8020a270_b(c, 0x8020a270)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a270_at(c: &mut Cpu, pc: u32) { f_8020a270_b(c, pc) }
#[inline(always)]
fn f_8020a270_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a270;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a270, pc); }
        match pc {
            0x8020a270 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a280; continue; }
                pc = 0x8020a278;
            }
            0x8020a278 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a280;
            }
            0x8020a280 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a270: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetMotoWazaDataId (0x8020a288, 24 bytes)
#[inline(never)]
pub fn f_8020a288(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a288) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a288); f_8020a288_b(c, 0x8020a288); crate::prof::exit(c, 0x8020a288); return; }
    }
    f_8020a288_b(c, 0x8020a288)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a288_at(c: &mut Cpu, pc: u32) { f_8020a288_b(c, pc) }
#[inline(always)]
fn f_8020a288_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a288;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a288, pc); }
        match pc {
            0x8020a288 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a298; continue; }
                pc = 0x8020a290;
            }
            0x8020a290 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a298;
            }
            0x8020a298 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a288: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosGetWazaBanme (0x8020a2a0, 24 bytes)
#[inline(never)]
pub fn f_8020a2a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a2a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a2a0); f_8020a2a0_b(c, 0x8020a2a0); crate::prof::exit(c, 0x8020a2a0); return; }
    }
    f_8020a2a0_b(c, 0x8020a2a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a2a0_at(c: &mut Cpu, pc: u32) { f_8020a2a0_b(c, pc) }
#[inline(always)]
fn f_8020a2a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a2a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a2a0, pc); }
        match pc {
            0x8020a2a0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a2b0; continue; }
                pc = 0x8020a2a8;
            }
            0x8020a2a8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a2b0;
            }
            0x8020a2b0 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a2a0: bad pc {:#x}", pc),
        }
    }
}

/// fightWazaBiosCopy (0x8020a2b8, 64 bytes)
#[inline(never)]
pub fn f_8020a2b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a2b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a2b8); f_8020a2b8_b(c, 0x8020a2b8); crate::prof::exit(c, 0x8020a2b8); return; }
    }
    f_8020a2b8_b(c, 0x8020a2b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a2b8_at(c: &mut Cpu, pc: u32) { f_8020a2b8_b(c, pc) }
#[inline(always)]
fn f_8020a2b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a2b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a2b8, pc); }
        match pc {
            0x8020a2b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a2c0;
            }
            0x8020a2c0 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a2c8;
            }
            0x8020a2c8 => {
                c.r[0] = 0x15;
                c.r[5] = c.r[3].wrapping_add(0xfffffffc);
                c.r[4] = c.r[4].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x8020a2d8;
            }
            0x8020a2d8 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8020a2d8; continue; }
                pc = 0x8020a2ec;
            }
            0x8020a2ec => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a2b8: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosSetBuff (0x8020a2f8, 16 bytes)
#[inline(never)]
pub fn f_8020a2f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a2f8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a2f8); f_8020a2f8_b(c, 0x8020a2f8); crate::prof::exit(c, 0x8020a2f8); return; }
    }
    f_8020a2f8_b(c, 0x8020a2f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a2f8_at(c: &mut Cpu, pc: u32) { f_8020a2f8_b(c, pc) }
#[inline(always)]
fn f_8020a2f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a2f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a2f8, pc); }
        match pc {
            0x8020a2f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a300;
            }
            0x8020a300 => {
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a2f8: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosSetCount (0x8020a308, 16 bytes)
#[inline(never)]
pub fn f_8020a308(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a308) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a308); f_8020a308_b(c, 0x8020a308); crate::prof::exit(c, 0x8020a308); return; }
    }
    f_8020a308_b(c, 0x8020a308)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a308_at(c: &mut Cpu, pc: u32) { f_8020a308_b(c, pc) }
#[inline(always)]
fn f_8020a308_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a308;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a308, pc); }
        match pc {
            0x8020a308 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a310;
            }
            0x8020a310 => {
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a308: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosSetTargetDataId (0x8020a318, 16 bytes)
#[inline(never)]
pub fn f_8020a318(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a318) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a318); f_8020a318_b(c, 0x8020a318); crate::prof::exit(c, 0x8020a318); return; }
    }
    f_8020a318_b(c, 0x8020a318)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a318_at(c: &mut Cpu, pc: u32) { f_8020a318_b(c, pc) }
#[inline(always)]
fn f_8020a318_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a318;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a318, pc); }
        match pc {
            0x8020a318 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a320;
            }
            0x8020a320 => {
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a318: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosSetItemDataId (0x8020a328, 16 bytes)
#[inline(never)]
pub fn f_8020a328(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a328) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a328); f_8020a328_b(c, 0x8020a328); crate::prof::exit(c, 0x8020a328); return; }
    }
    f_8020a328_b(c, 0x8020a328)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a328_at(c: &mut Cpu, pc: u32) { f_8020a328_b(c, pc) }
#[inline(always)]
fn f_8020a328_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a328;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a328, pc); }
        match pc {
            0x8020a328 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020a330;
            }
            0x8020a330 => {
                c.sth(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a328: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosGetBuff (0x8020a338, 24 bytes)
#[inline(never)]
pub fn f_8020a338(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a338) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a338); f_8020a338_b(c, 0x8020a338); crate::prof::exit(c, 0x8020a338); return; }
    }
    f_8020a338_b(c, 0x8020a338)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a338_at(c: &mut Cpu, pc: u32) { f_8020a338_b(c, pc) }
#[inline(always)]
fn f_8020a338_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a338;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a338, pc); }
        match pc {
            0x8020a338 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a348; continue; }
                pc = 0x8020a340;
            }
            0x8020a340 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a348;
            }
            0x8020a348 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a338: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosGetCount (0x8020a350, 24 bytes)
#[inline(never)]
pub fn f_8020a350(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a350) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a350); f_8020a350_b(c, 0x8020a350); crate::prof::exit(c, 0x8020a350); return; }
    }
    f_8020a350_b(c, 0x8020a350)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a350_at(c: &mut Cpu, pc: u32) { f_8020a350_b(c, pc) }
#[inline(always)]
fn f_8020a350_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a350;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a350, pc); }
        match pc {
            0x8020a350 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a360; continue; }
                pc = 0x8020a358;
            }
            0x8020a358 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a360;
            }
            0x8020a360 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a350: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosGetTargetDataId (0x8020a368, 24 bytes)
#[inline(never)]
pub fn f_8020a368(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a368) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a368); f_8020a368_b(c, 0x8020a368); crate::prof::exit(c, 0x8020a368); return; }
    }
    f_8020a368_b(c, 0x8020a368)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a368_at(c: &mut Cpu, pc: u32) { f_8020a368_b(c, pc) }
#[inline(always)]
fn f_8020a368_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a368;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a368, pc); }
        match pc {
            0x8020a368 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a378; continue; }
                pc = 0x8020a370;
            }
            0x8020a370 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a378;
            }
            0x8020a378 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a368: bad pc {:#x}", pc),
        }
    }
}

/// fightItemBiosGetItemDataId (0x8020a380, 24 bytes)
#[inline(never)]
pub fn f_8020a380(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a380) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a380); f_8020a380_b(c, 0x8020a380); crate::prof::exit(c, 0x8020a380); return; }
    }
    f_8020a380_b(c, 0x8020a380)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a380_at(c: &mut Cpu, pc: u32) { f_8020a380_b(c, pc) }
#[inline(always)]
fn f_8020a380_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a380;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a380, pc); }
        match pc {
            0x8020a380 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a390; continue; }
                pc = 0x8020a388;
            }
            0x8020a388 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a390;
            }
            0x8020a390 => {
                c.r[3] = c.lhz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a380: bad pc {:#x}", pc),
        }
    }
}

/// fightItemCreate (0x8020a398, 224 bytes)
#[inline(never)]
pub fn f_8020a398(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a398) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a398); f_8020a398_b(c, 0x8020a398); crate::prof::exit(c, 0x8020a398); return; }
    }
    f_8020a398_b(c, 0x8020a398)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a398_at(c: &mut Cpu, pc: u32) { f_8020a398_b(c, pc) }
#[inline(always)]
fn f_8020a398_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a398;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a398, pc); }
        match pc {
            0x8020a398 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[6] | c.r[6];
                if c.crbit(2) { pc = 0x8020a464; continue; }
                pc = 0x8020a3bc;
            }
            0x8020a3bc => {
                if c.crbit(2) { pc = 0x8020a41c; continue; }
                pc = 0x8020a3c0;
            }
            0x8020a3c0 => {
                c.r[4] = 0x0;
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a3d4; f_80142b24(c);
                pc = 0x8020a3d4;
            }
            0x8020a3d4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a3ec; f_80142b24(c);
                pc = 0x8020a3ec;
            }
            0x8020a3ec => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x20;
                c.r[6] = 0x0;
                c.r[7] = 0xffffffff;
                c.lr = 0x8020a404; f_80142b24(c);
                pc = 0x8020a404;
            }
            0x8020a404 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x21;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a41c; f_80142b24(c);
                pc = 0x8020a41c;
            }
            0x8020a41c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[7] = c.r[29].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.lr = 0x8020a434; f_80142b24(c);
                pc = 0x8020a434;
            }
            0x8020a434 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[7] = c.r[30].rotate_left(0) & 0xffff;
                c.r[4] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.lr = 0x8020a44c; f_80142b24(c);
                pc = 0x8020a44c;
            }
            0x8020a44c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[7] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x20;
                c.r[6] = 0x0;
                c.lr = 0x8020a464; f_80142b24(c);
                pc = 0x8020a464;
            }
            0x8020a464 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a398: bad pc {:#x}", pc),
        }
    }
}

/// fightItemInit (0x8020a478, 136 bytes)
#[inline(never)]
pub fn f_8020a478(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a478) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a478); f_8020a478_b(c, 0x8020a478); crate::prof::exit(c, 0x8020a478); return; }
    }
    f_8020a478_b(c, 0x8020a478)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a478_at(c: &mut Cpu, pc: u32) { f_8020a478_b(c, pc) }
#[inline(always)]
fn f_8020a478_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a478;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a478, pc); }
        match pc {
            0x8020a478 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x8020a4ec; continue; }
                pc = 0x8020a490;
            }
            0x8020a490 => {
                c.r[4] = 0x0;
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a4a4; f_80142b24(c);
                pc = 0x8020a4a4;
            }
            0x8020a4a4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a4bc; f_80142b24(c);
                pc = 0x8020a4bc;
            }
            0x8020a4bc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x20;
                c.r[6] = 0x0;
                c.r[7] = 0xffffffff;
                c.lr = 0x8020a4d4; f_80142b24(c);
                pc = 0x8020a4d4;
            }
            0x8020a4d4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x21;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8020a4ec; f_80142b24(c);
                pc = 0x8020a4ec;
            }
            0x8020a4ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a478: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A500 (0x8020a500, 64 bytes)
#[inline(never)]
pub fn f_8020a500(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a500) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a500); f_8020a500_b(c, 0x8020a500); crate::prof::exit(c, 0x8020a500); return; }
    }
    f_8020a500_b(c, 0x8020a500)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a500_at(c: &mut Cpu, pc: u32) { f_8020a500_b(c, pc) }
#[inline(always)]
fn f_8020a500_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a500;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a500, pc); }
        match pc {
            0x8020a500 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8518));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a518; continue; }
                pc = 0x8020a510;
            }
            0x8020a510 => {
                c.r[3] = 0x0;
                pc = 0x8020a528; continue;
                pc = 0x8020a518;
            }
            0x8020a518 => {
                c.r[4] = c.r[3].wrapping_mul(0x6);
                c.r[3] = 0x80480000;
                c.r[0] = c.r[3].wrapping_add(0xffff8d30);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8020a528;
            }
            0x8020a528 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a538; continue; }
                pc = 0x8020a530;
            }
            0x8020a530 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a538;
            }
            0x8020a538 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a500: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A540 (0x8020a540, 64 bytes)
#[inline(never)]
pub fn f_8020a540(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a540) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a540); f_8020a540_b(c, 0x8020a540); crate::prof::exit(c, 0x8020a540); return; }
    }
    f_8020a540_b(c, 0x8020a540)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a540_at(c: &mut Cpu, pc: u32) { f_8020a540_b(c, pc) }
#[inline(always)]
fn f_8020a540_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a540;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a540, pc); }
        match pc {
            0x8020a540 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8518));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a558; continue; }
                pc = 0x8020a550;
            }
            0x8020a550 => {
                c.r[3] = 0x0;
                pc = 0x8020a568; continue;
                pc = 0x8020a558;
            }
            0x8020a558 => {
                c.r[4] = c.r[3].wrapping_mul(0x6);
                c.r[3] = 0x80480000;
                c.r[0] = c.r[3].wrapping_add(0xffff8d30);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8020a568;
            }
            0x8020a568 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a578; continue; }
                pc = 0x8020a570;
            }
            0x8020a570 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a578;
            }
            0x8020a578 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a540: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A580 (0x8020a580, 64 bytes)
#[inline(never)]
pub fn f_8020a580(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a580) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a580); f_8020a580_b(c, 0x8020a580); crate::prof::exit(c, 0x8020a580); return; }
    }
    f_8020a580_b(c, 0x8020a580)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a580_at(c: &mut Cpu, pc: u32) { f_8020a580_b(c, pc) }
#[inline(always)]
fn f_8020a580_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a580;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a580, pc); }
        match pc {
            0x8020a580 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8518));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a598; continue; }
                pc = 0x8020a590;
            }
            0x8020a590 => {
                c.r[3] = 0x0;
                pc = 0x8020a5a8; continue;
                pc = 0x8020a598;
            }
            0x8020a598 => {
                c.r[4] = c.r[3].wrapping_mul(0x6);
                c.r[3] = 0x80480000;
                c.r[0] = c.r[3].wrapping_add(0xffff8d30);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8020a5a8;
            }
            0x8020a5a8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a5b8; continue; }
                pc = 0x8020a5b0;
            }
            0x8020a5b0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a5b8;
            }
            0x8020a5b8 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a580: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A5C0 (0x8020a5c0, 112 bytes)
#[inline(never)]
pub fn f_8020a5c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a5c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a5c0); f_8020a5c0_b(c, 0x8020a5c0); crate::prof::exit(c, 0x8020a5c0); return; }
    }
    f_8020a5c0_b(c, 0x8020a5c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a5c0_at(c: &mut Cpu, pc: u32) { f_8020a5c0_b(c, pc) }
#[inline(always)]
fn f_8020a5c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a5c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a5c0, pc); }
        match pc {
            0x8020a5c0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a5d8; continue; }
                pc = 0x8020a5d0;
            }
            0x8020a5d0 => {
                c.r[5] = 0x0;
                pc = 0x8020a5e8; continue;
                pc = 0x8020a5d8;
            }
            0x8020a5d8 => {
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a5e8;
            }
            0x8020a5e8 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8020a5f8; continue; }
                pc = 0x8020a5f0;
            }
            0x8020a5f0 => {
                c.r[3] = 0x0;
                pc = 0x8020a618; continue;
                pc = 0x8020a5f8;
            }
            0x8020a5f8 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8020a60c; continue; }
                pc = 0x8020a604;
            }
            0x8020a604 => {
                c.r[3] = 0x0;
                pc = 0x8020a618; continue;
                pc = 0x8020a60c;
            }
            0x8020a60c => {
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a618;
            }
            0x8020a618 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a628; continue; }
                pc = 0x8020a620;
            }
            0x8020a620 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a628;
            }
            0x8020a628 => {
                c.r[3] = c.lha(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a5c0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A630 (0x8020a630, 112 bytes)
#[inline(never)]
pub fn f_8020a630(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a630) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a630); f_8020a630_b(c, 0x8020a630); crate::prof::exit(c, 0x8020a630); return; }
    }
    f_8020a630_b(c, 0x8020a630)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a630_at(c: &mut Cpu, pc: u32) { f_8020a630_b(c, pc) }
#[inline(always)]
fn f_8020a630_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a630;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a630, pc); }
        match pc {
            0x8020a630 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a648; continue; }
                pc = 0x8020a640;
            }
            0x8020a640 => {
                c.r[5] = 0x0;
                pc = 0x8020a658; continue;
                pc = 0x8020a648;
            }
            0x8020a648 => {
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a658;
            }
            0x8020a658 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8020a668; continue; }
                pc = 0x8020a660;
            }
            0x8020a660 => {
                c.r[3] = 0x0;
                pc = 0x8020a688; continue;
                pc = 0x8020a668;
            }
            0x8020a668 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8020a67c; continue; }
                pc = 0x8020a674;
            }
            0x8020a674 => {
                c.r[3] = 0x0;
                pc = 0x8020a688; continue;
                pc = 0x8020a67c;
            }
            0x8020a67c => {
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a688;
            }
            0x8020a688 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a698; continue; }
                pc = 0x8020a690;
            }
            0x8020a690 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a698;
            }
            0x8020a698 => {
                c.r[3] = c.lha(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a630: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A6A0 (0x8020a6a0, 112 bytes)
#[inline(never)]
pub fn f_8020a6a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a6a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a6a0); f_8020a6a0_b(c, 0x8020a6a0); crate::prof::exit(c, 0x8020a6a0); return; }
    }
    f_8020a6a0_b(c, 0x8020a6a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a6a0_at(c: &mut Cpu, pc: u32) { f_8020a6a0_b(c, pc) }
#[inline(always)]
fn f_8020a6a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a6a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a6a0, pc); }
        match pc {
            0x8020a6a0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a6b8; continue; }
                pc = 0x8020a6b0;
            }
            0x8020a6b0 => {
                c.r[5] = 0x0;
                pc = 0x8020a6c8; continue;
                pc = 0x8020a6b8;
            }
            0x8020a6b8 => {
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a6c8;
            }
            0x8020a6c8 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8020a6d8; continue; }
                pc = 0x8020a6d0;
            }
            0x8020a6d0 => {
                c.r[3] = 0x0;
                pc = 0x8020a6f8; continue;
                pc = 0x8020a6d8;
            }
            0x8020a6d8 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8020a6ec; continue; }
                pc = 0x8020a6e4;
            }
            0x8020a6e4 => {
                c.r[3] = 0x0;
                pc = 0x8020a6f8; continue;
                pc = 0x8020a6ec;
            }
            0x8020a6ec => {
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a6f8;
            }
            0x8020a6f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a708; continue; }
                pc = 0x8020a700;
            }
            0x8020a700 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a708;
            }
            0x8020a708 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a6a0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A710 (0x8020a710, 112 bytes)
#[inline(never)]
pub fn f_8020a710(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a710) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a710); f_8020a710_b(c, 0x8020a710); crate::prof::exit(c, 0x8020a710); return; }
    }
    f_8020a710_b(c, 0x8020a710)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a710_at(c: &mut Cpu, pc: u32) { f_8020a710_b(c, pc) }
#[inline(always)]
fn f_8020a710_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a710;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a710, pc); }
        match pc {
            0x8020a710 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a728; continue; }
                pc = 0x8020a720;
            }
            0x8020a720 => {
                c.r[5] = 0x0;
                pc = 0x8020a738; continue;
                pc = 0x8020a728;
            }
            0x8020a728 => {
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a738;
            }
            0x8020a738 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8020a748; continue; }
                pc = 0x8020a740;
            }
            0x8020a740 => {
                c.r[3] = 0x0;
                pc = 0x8020a768; continue;
                pc = 0x8020a748;
            }
            0x8020a748 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8020a75c; continue; }
                pc = 0x8020a754;
            }
            0x8020a754 => {
                c.r[3] = 0x0;
                pc = 0x8020a768; continue;
                pc = 0x8020a75c;
            }
            0x8020a75c => {
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a768;
            }
            0x8020a768 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a778; continue; }
                pc = 0x8020a770;
            }
            0x8020a770 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a778;
            }
            0x8020a778 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a710: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A780 (0x8020a780, 112 bytes)
#[inline(never)]
pub fn f_8020a780(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a780) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a780); f_8020a780_b(c, 0x8020a780); crate::prof::exit(c, 0x8020a780); return; }
    }
    f_8020a780_b(c, 0x8020a780)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a780_at(c: &mut Cpu, pc: u32) { f_8020a780_b(c, pc) }
#[inline(always)]
fn f_8020a780_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a780;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a780, pc); }
        match pc {
            0x8020a780 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a798; continue; }
                pc = 0x8020a790;
            }
            0x8020a790 => {
                c.r[5] = 0x0;
                pc = 0x8020a7a8; continue;
                pc = 0x8020a798;
            }
            0x8020a798 => {
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a7a8;
            }
            0x8020a7a8 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8020a7b8; continue; }
                pc = 0x8020a7b0;
            }
            0x8020a7b0 => {
                c.r[3] = 0x0;
                pc = 0x8020a7d8; continue;
                pc = 0x8020a7b8;
            }
            0x8020a7b8 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8020a7cc; continue; }
                pc = 0x8020a7c4;
            }
            0x8020a7c4 => {
                c.r[3] = 0x0;
                pc = 0x8020a7d8; continue;
                pc = 0x8020a7cc;
            }
            0x8020a7cc => {
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a7d8;
            }
            0x8020a7d8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a7e8; continue; }
                pc = 0x8020a7e0;
            }
            0x8020a7e0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a7e8;
            }
            0x8020a7e8 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x6));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a780: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A7F0 (0x8020a7f0, 112 bytes)
#[inline(never)]
pub fn f_8020a7f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a7f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a7f0); f_8020a7f0_b(c, 0x8020a7f0); crate::prof::exit(c, 0x8020a7f0); return; }
    }
    f_8020a7f0_b(c, 0x8020a7f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a7f0_at(c: &mut Cpu, pc: u32) { f_8020a7f0_b(c, pc) }
#[inline(always)]
fn f_8020a7f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a7f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a7f0, pc); }
        match pc {
            0x8020a7f0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a808; continue; }
                pc = 0x8020a800;
            }
            0x8020a800 => {
                c.r[5] = 0x0;
                pc = 0x8020a818; continue;
                pc = 0x8020a808;
            }
            0x8020a808 => {
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8020a818;
            }
            0x8020a818 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8020a828; continue; }
                pc = 0x8020a820;
            }
            0x8020a820 => {
                c.r[3] = 0x0;
                pc = 0x8020a848; continue;
                pc = 0x8020a828;
            }
            0x8020a828 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8020a83c; continue; }
                pc = 0x8020a834;
            }
            0x8020a834 => {
                c.r[3] = 0x0;
                pc = 0x8020a848; continue;
                pc = 0x8020a83c;
            }
            0x8020a83c => {
                c.r[3] = c.r[0].wrapping_mul(0xa);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8020a848;
            }
            0x8020a848 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a858; continue; }
                pc = 0x8020a850;
            }
            0x8020a850 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a858;
            }
            0x8020a858 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a7f0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A860 (0x8020a860, 64 bytes)
#[inline(never)]
pub fn f_8020a860(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a860) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a860); f_8020a860_b(c, 0x8020a860); crate::prof::exit(c, 0x8020a860); return; }
    }
    f_8020a860_b(c, 0x8020a860)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a860_at(c: &mut Cpu, pc: u32) { f_8020a860_b(c, pc) }
#[inline(always)]
fn f_8020a860_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a860;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a860, pc); }
        match pc {
            0x8020a860 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a878; continue; }
                pc = 0x8020a870;
            }
            0x8020a870 => {
                c.r[3] = 0x0;
                pc = 0x8020a888; continue;
                pc = 0x8020a878;
            }
            0x8020a878 => {
                c.r[4] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8020a888;
            }
            0x8020a888 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a898; continue; }
                pc = 0x8020a890;
            }
            0x8020a890 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a898;
            }
            0x8020a898 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8020a860: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A8A0 (0x8020a8a0, 64 bytes)
#[inline(never)]
pub fn f_8020a8a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a8a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a8a0); f_8020a8a0_b(c, 0x8020a8a0); crate::prof::exit(c, 0x8020a8a0); return; }
    }
    f_8020a8a0_b(c, 0x8020a8a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a8a0_at(c: &mut Cpu, pc: u32) { f_8020a8a0_b(c, pc) }
#[inline(always)]
fn f_8020a8a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a8a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a8a0, pc); }
        match pc {
            0x8020a8a0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8508));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020a8b8; continue; }
                pc = 0x8020a8b0;
            }
            0x8020a8b0 => {
                c.r[3] = 0x0;
                pc = 0x8020a8c8; continue;
                pc = 0x8020a8b8;
            }
            0x8020a8b8 => {
                c.r[4] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5a08);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8020a8c8;
            }
            0x8020a8c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020a8d8; continue; }
                pc = 0x8020a8d0;
            }
            0x8020a8d0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020a8d8;
            }
            0x8020a8d8 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020a8a0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020A8E0 (0x8020a8e0, 1360 bytes)
#[inline(never)]
pub fn f_8020a8e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020a8e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020a8e0); f_8020a8e0_b(c, 0x8020a8e0); crate::prof::exit(c, 0x8020a8e0); return; }
    }
    f_8020a8e0_b(c, 0x8020a8e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020a8e0_at(c: &mut Cpu, pc: u32) { f_8020a8e0_b(c, pc) }
#[inline(always)]
fn f_8020a8e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020a8e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020a8e0, pc); }
        match pc {
            0x8020a8e0 => {
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x8020a8e4;
            }
            0x8020a8e4 => {
                c.r[0] = c.mfspr(8);
                pc = 0x8020a8e8;
            }
            0x8020a8e8 => {
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                pc = 0x8020a8ec;
            }
            0x8020a8ec => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 19..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x8020a8f0;
            }
            0x8020a8f0 => {
                c.r[20] = c.r[3] | c.r[3];
                pc = 0x8020a8f4;
            }
            0x8020a8f4 => {
                c.r[31] = c.r[4] | c.r[4];
                pc = 0x8020a8f8;
            }
            0x8020a8f8 => {
                c.r[21] = c.r[1].wrapping_add(0x10);
                pc = 0x8020a8fc;
            }
            0x8020a8fc => {
                c.r[24] = 0x0;
                pc = 0x8020a900;
            }
            0x8020a900 => {
                pc = 0x8020aa64; continue;
                pc = 0x8020a904;
            }
            0x8020a904 => {
                c.r[29] = c.r[24].rotate_left(0) & 0xff;
                pc = 0x8020a908;
            }
            0x8020a908 => {
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a90c;
            }
            0x8020a90c => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a910;
            }
            0x8020a910 => {
                c.r[23] = 0x0;
                pc = 0x8020a914;
            }
            0x8020a914 => {
                c.lr = 0x8020a918; f_8020a7f0(c);
                pc = 0x8020a918;
            }
            0x8020a918 => {
                c.r[22] = c.r[3] | c.r[3];
                pc = 0x8020a91c;
            }
            0x8020a91c => {
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a920;
            }
            0x8020a920 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a924;
            }
            0x8020a924 => {
                c.lr = 0x8020a928; f_8020a780(c);
                pc = 0x8020a928;
            }
            0x8020a928 => {
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8020a92c;
            }
            0x8020a92c => {
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a930;
            }
            0x8020a930 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a934;
            }
            0x8020a934 => {
                c.lr = 0x8020a938; f_8020a710(c);
                pc = 0x8020a938;
            }
            0x8020a938 => {
                c.r[27] = c.r[3] | c.r[3];
                pc = 0x8020a93c;
            }
            0x8020a93c => {
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a940;
            }
            0x8020a940 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a944;
            }
            0x8020a944 => {
                c.lr = 0x8020a948; f_8020a630(c);
                pc = 0x8020a948;
            }
            0x8020a948 => {
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x8020a94c;
            }
            0x8020a94c => {
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a950;
            }
            0x8020a950 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a954;
            }
            0x8020a954 => {
                c.lr = 0x8020a958; f_8020a5c0(c);
                pc = 0x8020a958;
            }
            0x8020a958 => {
                c.r[25] = c.r[3] | c.r[3];
                pc = 0x8020a95c;
            }
            0x8020a95c => {
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020a960;
            }
            0x8020a960 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x8020a964;
            }
            0x8020a964 => {
                c.lr = 0x8020a968; f_8020a6a0(c);
                pc = 0x8020a968;
            }
            0x8020a968 => {
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                pc = 0x8020a96c;
            }
            0x8020a96c => {
                c.r[22] = c.r[3] | c.r[3];
                pc = 0x8020a970;
            }
            0x8020a970 => {
                c.cmps(0, c.r[0], 0x2);
                pc = 0x8020a974;
            }
            0x8020a974 => {
                if c.crbit(2) { pc = 0x8020a9a0; continue; }
                pc = 0x8020a978;
            }
            0x8020a978 => {
                if !c.crbit(0) { pc = 0x8020a98c; continue; }
                pc = 0x8020a97c;
            }
            0x8020a97c => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x8020a980;
            }
            0x8020a980 => {
                if c.crbit(2) { pc = 0x8020aa34; continue; }
                pc = 0x8020a984;
            }
            0x8020a984 => {
                if !c.crbit(0) { pc = 0x8020a998; continue; }
                pc = 0x8020a988;
            }
            0x8020a988 => {
                pc = 0x8020aa34; continue;
                pc = 0x8020a98c;
            }
            0x8020a98c => {
                c.cmps(0, c.r[0], 0x4);
                pc = 0x8020a990;
            }
            0x8020a990 => {
                if !c.crbit(0) { pc = 0x8020aa34; continue; }
                pc = 0x8020a994;
            }
            0x8020a994 => {
                pc = 0x8020a9b8; continue;
                pc = 0x8020a998;
            }
            0x8020a998 => {
                c.r[23] = c.r[28].rotate_left(0) & 0xffff;
                pc = 0x8020a99c;
            }
            0x8020a99c => {
                pc = 0x8020aa34; continue;
                pc = 0x8020a9a0;
            }
            0x8020a9a0 => {
                c.r[19] = c.r[28].rotate_left(0) & 0xffff;
                pc = 0x8020a9a4;
            }
            0x8020a9a4 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                pc = 0x8020a9a8;
            }
            0x8020a9a8 => {
                c.r[3] = c.r[0].wrapping_sub(c.r[19]);
                pc = 0x8020a9ac;
            }
            0x8020a9ac => {
                c.lr = 0x8020a9b0; f_800e0c04(c);
                pc = 0x8020a9b0;
            }
            0x8020a9b0 => {
                c.r[23] = c.r[19].wrapping_add(c.r[3]);
                pc = 0x8020a9b4;
            }
            0x8020a9b4 => {
                pc = 0x8020aa34; continue;
                pc = 0x8020a9b8;
            }
            0x8020a9b8 => {
                c.r[3] = 0x0;
                pc = 0x8020a9bc;
            }
            0x8020a9bc => {
                c.r[4] = 0x0;
                pc = 0x8020a9c0;
            }
            0x8020a9c0 => {
                c.r[5] = 0x14;
                pc = 0x8020a9c4;
            }
            0x8020a9c4 => {
                c.r[6] = 0x0;
                pc = 0x8020a9c8;
            }
            0x8020a9c8 => {
                c.lr = 0x8020a9cc; f_801f54a4(c);
                pc = 0x8020a9cc;
            }
            0x8020a9cc => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8020a9d0;
            }
            0x8020a9d0 => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x8020a9d4;
            }
            0x8020a9d4 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8020a9d8;
            }
            0x8020a9d8 => {
                c.lr = 0x8020a9dc; f_801f02ac(c);
                pc = 0x8020a9dc;
            }
            0x8020a9dc => {
                c.r[23] = c.r[3] | c.r[3];
                c.rc(c.r[23]);
                pc = 0x8020a9e0;
            }
            0x8020a9e0 => {
                if !c.crbit(2) { pc = 0x8020a9ec; continue; }
                pc = 0x8020a9e4;
            }
            0x8020a9e4 => {
                c.r[23] = 0x0;
                pc = 0x8020a9e8;
            }
            0x8020a9e8 => {
                pc = 0x8020aa58; continue;
                pc = 0x8020a9ec;
            }
            0x8020a9ec => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x8020a9f0;
            }
            0x8020a9f0 => {
                c.lr = 0x8020a9f4; f_801f0234(c);
                pc = 0x8020a9f4;
            }
            0x8020a9f4 => {
                c.lr = 0x8020a9f8; f_801f021c(c);
                pc = 0x8020a9f8;
            }
            0x8020a9f8 => {
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                pc = 0x8020a9fc;
            }
            0x8020a9fc => {
                if !c.crbit(2) { pc = 0x8020aa1c; continue; }
                pc = 0x8020aa00;
            }
            0x8020aa00 => {
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x8020aa04;
            }
            0x8020aa04 => {
                c.r[6] = c.r[27] | c.r[27];
                pc = 0x8020aa08;
            }
            0x8020aa08 => {
                c.r[5] = c.r[26].rotate_left(0) & 0xffff;
                pc = 0x8020aa0c;
            }
            0x8020aa0c => {
                c.r[7] = c.r[25].rotate_left(0) & 0xffff;
                pc = 0x8020aa10;
            }
            0x8020aa10 => {
                c.lr = 0x8020aa14; f_80135e44(c);
                pc = 0x8020aa14;
            }
            0x8020aa14 => {
                c.r[23] = c.r[3] | c.r[3];
                pc = 0x8020aa18;
            }
            0x8020aa18 => {
                pc = 0x8020aa34; continue;
                pc = 0x8020aa1c;
            }
            0x8020aa1c => {
                c.r[4] = c.r[23] | c.r[23];
                pc = 0x8020aa20;
            }
            0x8020aa20 => {
                c.r[6] = c.r[27] | c.r[27];
                pc = 0x8020aa24;
            }
            0x8020aa24 => {
                c.r[5] = 0x0;
                pc = 0x8020aa28;
            }
            0x8020aa28 => {
                c.r[7] = 0x0;
                pc = 0x8020aa2c;
            }
            0x8020aa2c => {
                c.lr = 0x8020aa30; f_80135e44(c);
                pc = 0x8020aa30;
            }
            0x8020aa30 => {
                c.r[23] = c.r[3] | c.r[3];
                pc = 0x8020aa34;
            }
            0x8020aa34 => {
                c.r[0] = c.r[22].rotate_left(0) & 0xff;
                pc = 0x8020aa38;
            }
            0x8020aa38 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x8020aa3c;
            }
            0x8020aa3c => {
                if !c.crbit(2) { pc = 0x8020aa58; continue; }
                pc = 0x8020aa40;
            }
            0x8020aa40 => {
                c.r[3] = c.r[26] as u16 as i16 as i32 as u32;
                pc = 0x8020aa44;
            }
            0x8020aa44 => {
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                pc = 0x8020aa48;
            }
            0x8020aa48 => {
                c.r[23] = c.r[23].wrapping_mul(c.r[3]);
                pc = 0x8020aa4c;
            }
            0x8020aa4c => {
                if c.crbit(2) { pc = 0x8020aa58; continue; }
                pc = 0x8020aa50;
            }
            0x8020aa50 => {
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                pc = 0x8020aa54;
            }
            0x8020aa54 => {
                c.r[23] = divw(c.r[23], c.r[0]);
                pc = 0x8020aa58;
            }
            0x8020aa58 => {
                c.r[0] = c.r[24].rotate_left(2) & 0x3fc;
                pc = 0x8020aa5c;
            }
            0x8020aa5c => {
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x8020aa60;
            }
            0x8020aa60 => {
                c.stw(c.r[21].wrapping_add(c.r[0]), c.r[23]);
                pc = 0x8020aa64;
            }
            0x8020aa64 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xff;
                pc = 0x8020aa68;
            }
            0x8020aa68 => {
                c.cmpu(0, c.r[0], 0x2);
                pc = 0x8020aa6c;
            }
            0x8020aa6c => {
                if c.crbit(0) { pc = 0x8020a904; continue; }
                pc = 0x8020aa70;
            }
            0x8020aa70 => {
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020aa74;
            }
            0x8020aa74 => {
                c.r[24] = 0x0;
                pc = 0x8020aa78;
            }
            0x8020aa78 => {
                c.lr = 0x8020aa7c; f_8020a8a0(c);
                pc = 0x8020aa7c;
            }
            0x8020aa7c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8020aa80;
            }
            0x8020aa80 => {
                c.cmpu(0, c.r[0], 0x6);
                pc = 0x8020aa84;
            }
            0x8020aa84 => {
                if c.crbit(1) { pc = 0x8020ab34; continue; }
                pc = 0x8020aa88;
            }
            0x8020aa88 => {
                c.r[3] = 0x80370000;
                pc = 0x8020aa8c;
            }
            0x8020aa8c => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x8020aa90;
            }
            0x8020aa90 => {
                c.r[3] = c.r[3].wrapping_add(0x5954);
                pc = 0x8020aa94;
            }
            0x8020aa94 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x8020aa98;
            }
            0x8020aa98 => {
                c.mtspr(9, c.r[0]);
                pc = 0x8020aa9c;
            }
            0x8020aa9c => {
                if true { let t = c.ctr; if (0x8020a8e0..0x8020ae30).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x8020aaa0;
            }
            0x8020aaa0 => {
                c.r[24] = 0x1;
                pc = 0x8020aaa4;
            }
            0x8020aaa4 => {
                pc = 0x8020ab34; continue;
                pc = 0x8020aaa8;
            }
            0x8020aaa8 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x10));
                pc = 0x8020aaac;
            }
            0x8020aaac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x8020aab0;
            }
            0x8020aab0 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020aab4;
            }
            0x8020aab4 => {
                if !c.crbit(2) { pc = 0x8020ab34; continue; }
                pc = 0x8020aab8;
            }
            0x8020aab8 => {
                c.r[24] = 0x1;
                pc = 0x8020aabc;
            }
            0x8020aabc => {
                pc = 0x8020ab34; continue;
                pc = 0x8020aac0;
            }
            0x8020aac0 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x10));
                pc = 0x8020aac4;
            }
            0x8020aac4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x8020aac8;
            }
            0x8020aac8 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020aacc;
            }
            0x8020aacc => {
                if c.crbit(2) { pc = 0x8020ab34; continue; }
                pc = 0x8020aad0;
            }
            0x8020aad0 => {
                c.r[24] = 0x1;
                pc = 0x8020aad4;
            }
            0x8020aad4 => {
                pc = 0x8020ab34; continue;
                pc = 0x8020aad8;
            }
            0x8020aad8 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x10));
                pc = 0x8020aadc;
            }
            0x8020aadc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x8020aae0;
            }
            0x8020aae0 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020aae4;
            }
            0x8020aae4 => {
                if c.crbit(0) { pc = 0x8020ab34; continue; }
                pc = 0x8020aae8;
            }
            0x8020aae8 => {
                c.r[24] = 0x1;
                pc = 0x8020aaec;
            }
            0x8020aaec => {
                pc = 0x8020ab34; continue;
                pc = 0x8020aaf0;
            }
            0x8020aaf0 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x10));
                pc = 0x8020aaf4;
            }
            0x8020aaf4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x8020aaf8;
            }
            0x8020aaf8 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020aafc;
            }
            0x8020aafc => {
                if c.crbit(1) { pc = 0x8020ab34; continue; }
                pc = 0x8020ab00;
            }
            0x8020ab00 => {
                c.r[24] = 0x1;
                pc = 0x8020ab04;
            }
            0x8020ab04 => {
                pc = 0x8020ab34; continue;
                pc = 0x8020ab08;
            }
            0x8020ab08 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x10));
                pc = 0x8020ab0c;
            }
            0x8020ab0c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x8020ab10;
            }
            0x8020ab10 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020ab14;
            }
            0x8020ab14 => {
                if !c.crbit(0) { pc = 0x8020ab34; continue; }
                pc = 0x8020ab18;
            }
            0x8020ab18 => {
                c.r[24] = 0x1;
                pc = 0x8020ab1c;
            }
            0x8020ab1c => {
                pc = 0x8020ab34; continue;
                pc = 0x8020ab20;
            }
            0x8020ab20 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x10));
                pc = 0x8020ab24;
            }
            0x8020ab24 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x8020ab28;
            }
            0x8020ab28 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020ab2c;
            }
            0x8020ab2c => {
                if !c.crbit(1) { pc = 0x8020ab34; continue; }
                pc = 0x8020ab30;
            }
            0x8020ab30 => {
                c.r[24] = 0x1;
                pc = 0x8020ab34;
            }
            0x8020ab34 => {
                c.r[30] = c.r[24] | c.r[24];
                pc = 0x8020ab38;
            }
            0x8020ab38 => {
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x8020ab3c;
            }
            0x8020ab3c => {
                c.lr = 0x8020ab40; f_8020a860(c);
                pc = 0x8020ab40;
            }
            0x8020ab40 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                pc = 0x8020ab44;
            }
            0x8020ab44 => {
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8020ab48;
            }
            0x8020ab48 => {
                if !c.crbit(2) { pc = 0x8020ab54; continue; }
                pc = 0x8020ab4c;
            }
            0x8020ab4c => {
                c.r[3] = c.r[24] | c.r[24];
                pc = 0x8020ab50;
            }
            0x8020ab50 => {
                pc = 0x8020ae1c; continue;
                pc = 0x8020ab54;
            }
            0x8020ab54 => {
                c.r[27] = c.r[1].wrapping_add(0x8);
                pc = 0x8020ab58;
            }
            0x8020ab58 => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x8020ab5c;
            }
            0x8020ab5c => {
                c.lr = 0x8020ab60; f_8020a540(c);
                pc = 0x8020ab60;
            }
            0x8020ab60 => {
                c.r[23] = 0x0;
                pc = 0x8020ab64;
            }
            0x8020ab64 => {
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x8020ab68;
            }
            0x8020ab68 => {
                pc = 0x8020accc; continue;
                pc = 0x8020ab6c;
            }
            0x8020ab6c => {
                c.r[19] = c.r[23].rotate_left(0) & 0xff;
                pc = 0x8020ab70;
            }
            0x8020ab70 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020ab74;
            }
            0x8020ab74 => {
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020ab78;
            }
            0x8020ab78 => {
                c.r[20] = 0x0;
                pc = 0x8020ab7c;
            }
            0x8020ab7c => {
                c.lr = 0x8020ab80; f_8020a7f0(c);
                pc = 0x8020ab80;
            }
            0x8020ab80 => {
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x8020ab84;
            }
            0x8020ab84 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020ab88;
            }
            0x8020ab88 => {
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020ab8c;
            }
            0x8020ab8c => {
                c.lr = 0x8020ab90; f_8020a780(c);
                pc = 0x8020ab90;
            }
            0x8020ab90 => {
                c.r[21] = c.r[3] | c.r[3];
                pc = 0x8020ab94;
            }
            0x8020ab94 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020ab98;
            }
            0x8020ab98 => {
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020ab9c;
            }
            0x8020ab9c => {
                c.lr = 0x8020aba0; f_8020a710(c);
                pc = 0x8020aba0;
            }
            0x8020aba0 => {
                c.r[22] = c.r[3] | c.r[3];
                pc = 0x8020aba4;
            }
            0x8020aba4 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020aba8;
            }
            0x8020aba8 => {
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020abac;
            }
            0x8020abac => {
                c.lr = 0x8020abb0; f_8020a630(c);
                pc = 0x8020abb0;
            }
            0x8020abb0 => {
                c.r[24] = c.r[3] | c.r[3];
                pc = 0x8020abb4;
            }
            0x8020abb4 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020abb8;
            }
            0x8020abb8 => {
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020abbc;
            }
            0x8020abbc => {
                c.lr = 0x8020abc0; f_8020a5c0(c);
                pc = 0x8020abc0;
            }
            0x8020abc0 => {
                c.r[25] = c.r[3] | c.r[3];
                pc = 0x8020abc4;
            }
            0x8020abc4 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020abc8;
            }
            0x8020abc8 => {
                c.r[4] = c.r[19] | c.r[19];
                pc = 0x8020abcc;
            }
            0x8020abcc => {
                c.lr = 0x8020abd0; f_8020a6a0(c);
                pc = 0x8020abd0;
            }
            0x8020abd0 => {
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                pc = 0x8020abd4;
            }
            0x8020abd4 => {
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x8020abd8;
            }
            0x8020abd8 => {
                c.cmps(0, c.r[0], 0x2);
                pc = 0x8020abdc;
            }
            0x8020abdc => {
                if c.crbit(2) { pc = 0x8020ac08; continue; }
                pc = 0x8020abe0;
            }
            0x8020abe0 => {
                if !c.crbit(0) { pc = 0x8020abf4; continue; }
                pc = 0x8020abe4;
            }
            0x8020abe4 => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x8020abe8;
            }
            0x8020abe8 => {
                if c.crbit(2) { pc = 0x8020ac9c; continue; }
                pc = 0x8020abec;
            }
            0x8020abec => {
                if !c.crbit(0) { pc = 0x8020ac00; continue; }
                pc = 0x8020abf0;
            }
            0x8020abf0 => {
                pc = 0x8020ac9c; continue;
                pc = 0x8020abf4;
            }
            0x8020abf4 => {
                c.cmps(0, c.r[0], 0x4);
                pc = 0x8020abf8;
            }
            0x8020abf8 => {
                if !c.crbit(0) { pc = 0x8020ac9c; continue; }
                pc = 0x8020abfc;
            }
            0x8020abfc => {
                pc = 0x8020ac20; continue;
                pc = 0x8020ac00;
            }
            0x8020ac00 => {
                c.r[20] = c.r[21].rotate_left(0) & 0xffff;
                pc = 0x8020ac04;
            }
            0x8020ac04 => {
                pc = 0x8020ac9c; continue;
                pc = 0x8020ac08;
            }
            0x8020ac08 => {
                c.r[19] = c.r[21].rotate_left(0) & 0xffff;
                pc = 0x8020ac0c;
            }
            0x8020ac0c => {
                c.r[0] = c.r[22].rotate_left(0) & 0xffff;
                pc = 0x8020ac10;
            }
            0x8020ac10 => {
                c.r[3] = c.r[0].wrapping_sub(c.r[19]);
                pc = 0x8020ac14;
            }
            0x8020ac14 => {
                c.lr = 0x8020ac18; f_800e0c04(c);
                pc = 0x8020ac18;
            }
            0x8020ac18 => {
                c.r[20] = c.r[19].wrapping_add(c.r[3]);
                pc = 0x8020ac1c;
            }
            0x8020ac1c => {
                pc = 0x8020ac9c; continue;
                pc = 0x8020ac20;
            }
            0x8020ac20 => {
                c.r[3] = 0x0;
                pc = 0x8020ac24;
            }
            0x8020ac24 => {
                c.r[4] = 0x0;
                pc = 0x8020ac28;
            }
            0x8020ac28 => {
                c.r[5] = 0x14;
                pc = 0x8020ac2c;
            }
            0x8020ac2c => {
                c.r[6] = 0x0;
                pc = 0x8020ac30;
            }
            0x8020ac30 => {
                c.lr = 0x8020ac34; f_801f54a4(c);
                pc = 0x8020ac34;
            }
            0x8020ac34 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8020ac38;
            }
            0x8020ac38 => {
                c.r[3] = c.r[21] | c.r[21];
                pc = 0x8020ac3c;
            }
            0x8020ac3c => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8020ac40;
            }
            0x8020ac40 => {
                c.lr = 0x8020ac44; f_801f02ac(c);
                pc = 0x8020ac44;
            }
            0x8020ac44 => {
                c.r[20] = c.r[3] | c.r[3];
                c.rc(c.r[20]);
                pc = 0x8020ac48;
            }
            0x8020ac48 => {
                if !c.crbit(2) { pc = 0x8020ac54; continue; }
                pc = 0x8020ac4c;
            }
            0x8020ac4c => {
                c.r[20] = 0x0;
                pc = 0x8020ac50;
            }
            0x8020ac50 => {
                pc = 0x8020acc0; continue;
                pc = 0x8020ac54;
            }
            0x8020ac54 => {
                c.r[3] = c.r[21] | c.r[21];
                pc = 0x8020ac58;
            }
            0x8020ac58 => {
                c.lr = 0x8020ac5c; f_801f0234(c);
                pc = 0x8020ac5c;
            }
            0x8020ac5c => {
                c.lr = 0x8020ac60; f_801f021c(c);
                pc = 0x8020ac60;
            }
            0x8020ac60 => {
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                pc = 0x8020ac64;
            }
            0x8020ac64 => {
                if !c.crbit(2) { pc = 0x8020ac84; continue; }
                pc = 0x8020ac68;
            }
            0x8020ac68 => {
                c.r[4] = c.r[20] | c.r[20];
                pc = 0x8020ac6c;
            }
            0x8020ac6c => {
                c.r[6] = c.r[22] | c.r[22];
                pc = 0x8020ac70;
            }
            0x8020ac70 => {
                c.r[5] = c.r[24].rotate_left(0) & 0xffff;
                pc = 0x8020ac74;
            }
            0x8020ac74 => {
                c.r[7] = c.r[25].rotate_left(0) & 0xffff;
                pc = 0x8020ac78;
            }
            0x8020ac78 => {
                c.lr = 0x8020ac7c; f_80135e44(c);
                pc = 0x8020ac7c;
            }
            0x8020ac7c => {
                c.r[20] = c.r[3] | c.r[3];
                pc = 0x8020ac80;
            }
            0x8020ac80 => {
                pc = 0x8020ac9c; continue;
                pc = 0x8020ac84;
            }
            0x8020ac84 => {
                c.r[4] = c.r[20] | c.r[20];
                pc = 0x8020ac88;
            }
            0x8020ac88 => {
                c.r[6] = c.r[22] | c.r[22];
                pc = 0x8020ac8c;
            }
            0x8020ac8c => {
                c.r[5] = 0x0;
                pc = 0x8020ac90;
            }
            0x8020ac90 => {
                c.r[7] = 0x0;
                pc = 0x8020ac94;
            }
            0x8020ac94 => {
                c.lr = 0x8020ac98; f_80135e44(c);
                pc = 0x8020ac98;
            }
            0x8020ac98 => {
                c.r[20] = c.r[3] | c.r[3];
                pc = 0x8020ac9c;
            }
            0x8020ac9c => {
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                pc = 0x8020aca0;
            }
            0x8020aca0 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x8020aca4;
            }
            0x8020aca4 => {
                if !c.crbit(2) { pc = 0x8020acc0; continue; }
                pc = 0x8020aca8;
            }
            0x8020aca8 => {
                c.r[3] = c.r[24] as u16 as i16 as i32 as u32;
                pc = 0x8020acac;
            }
            0x8020acac => {
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                pc = 0x8020acb0;
            }
            0x8020acb0 => {
                c.r[20] = c.r[20].wrapping_mul(c.r[3]);
                pc = 0x8020acb4;
            }
            0x8020acb4 => {
                if c.crbit(2) { pc = 0x8020acc0; continue; }
                pc = 0x8020acb8;
            }
            0x8020acb8 => {
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                pc = 0x8020acbc;
            }
            0x8020acbc => {
                c.r[20] = divw(c.r[20], c.r[0]);
                pc = 0x8020acc0;
            }
            0x8020acc0 => {
                c.r[0] = c.r[23].rotate_left(2) & 0x3fc;
                pc = 0x8020acc4;
            }
            0x8020acc4 => {
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020acc8;
            }
            0x8020acc8 => {
                c.stw(c.r[27].wrapping_add(c.r[0]), c.r[20]);
                pc = 0x8020accc;
            }
            0x8020accc => {
                c.r[0] = c.r[23].rotate_left(0) & 0xff;
                pc = 0x8020acd0;
            }
            0x8020acd0 => {
                c.cmpu(0, c.r[0], 0x2);
                pc = 0x8020acd4;
            }
            0x8020acd4 => {
                if c.crbit(0) { pc = 0x8020ab6c; continue; }
                pc = 0x8020acd8;
            }
            0x8020acd8 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020acdc;
            }
            0x8020acdc => {
                c.r[23] = 0x0;
                pc = 0x8020ace0;
            }
            0x8020ace0 => {
                c.lr = 0x8020ace4; f_8020a8a0(c);
                pc = 0x8020ace4;
            }
            0x8020ace4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8020ace8;
            }
            0x8020ace8 => {
                c.cmpu(0, c.r[0], 0x6);
                pc = 0x8020acec;
            }
            0x8020acec => {
                if c.crbit(1) { pc = 0x8020ad9c; continue; }
                pc = 0x8020acf0;
            }
            0x8020acf0 => {
                c.r[3] = 0x80370000;
                pc = 0x8020acf4;
            }
            0x8020acf4 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x8020acf8;
            }
            0x8020acf8 => {
                c.r[3] = c.r[3].wrapping_add(0x5938);
                pc = 0x8020acfc;
            }
            0x8020acfc => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x8020ad00;
            }
            0x8020ad00 => {
                c.mtspr(9, c.r[0]);
                pc = 0x8020ad04;
            }
            0x8020ad04 => {
                if true { let t = c.ctr; if (0x8020a8e0..0x8020ae30).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x8020ad08;
            }
            0x8020ad08 => {
                c.r[23] = 0x1;
                pc = 0x8020ad0c;
            }
            0x8020ad0c => {
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad10;
            }
            0x8020ad10 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x8));
                pc = 0x8020ad14;
            }
            0x8020ad14 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x8020ad18;
            }
            0x8020ad18 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020ad1c;
            }
            0x8020ad1c => {
                if !c.crbit(2) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad20;
            }
            0x8020ad20 => {
                c.r[23] = 0x1;
                pc = 0x8020ad24;
            }
            0x8020ad24 => {
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad28;
            }
            0x8020ad28 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x8));
                pc = 0x8020ad2c;
            }
            0x8020ad2c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x8020ad30;
            }
            0x8020ad30 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020ad34;
            }
            0x8020ad34 => {
                if c.crbit(2) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad38;
            }
            0x8020ad38 => {
                c.r[23] = 0x1;
                pc = 0x8020ad3c;
            }
            0x8020ad3c => {
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad40;
            }
            0x8020ad40 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x8));
                pc = 0x8020ad44;
            }
            0x8020ad44 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x8020ad48;
            }
            0x8020ad48 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020ad4c;
            }
            0x8020ad4c => {
                if c.crbit(0) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad50;
            }
            0x8020ad50 => {
                c.r[23] = 0x1;
                pc = 0x8020ad54;
            }
            0x8020ad54 => {
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad58;
            }
            0x8020ad58 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x8));
                pc = 0x8020ad5c;
            }
            0x8020ad5c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x8020ad60;
            }
            0x8020ad60 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020ad64;
            }
            0x8020ad64 => {
                if c.crbit(1) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad68;
            }
            0x8020ad68 => {
                c.r[23] = 0x1;
                pc = 0x8020ad6c;
            }
            0x8020ad6c => {
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad70;
            }
            0x8020ad70 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x8));
                pc = 0x8020ad74;
            }
            0x8020ad74 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x8020ad78;
            }
            0x8020ad78 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020ad7c;
            }
            0x8020ad7c => {
                if !c.crbit(0) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad80;
            }
            0x8020ad80 => {
                c.r[23] = 0x1;
                pc = 0x8020ad84;
            }
            0x8020ad84 => {
                pc = 0x8020ad9c; continue;
                pc = 0x8020ad88;
            }
            0x8020ad88 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x8));
                pc = 0x8020ad8c;
            }
            0x8020ad8c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x8020ad90;
            }
            0x8020ad90 => {
                c.cmps(0, c.r[3], c.r[0]);
                pc = 0x8020ad94;
            }
            0x8020ad94 => {
                if !c.crbit(1) { pc = 0x8020ad9c; continue; }
                pc = 0x8020ad98;
            }
            0x8020ad98 => {
                c.r[23] = 0x1;
                pc = 0x8020ad9c;
            }
            0x8020ad9c => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x8020ada0;
            }
            0x8020ada0 => {
                c.lr = 0x8020ada4; f_8020a580(c);
                pc = 0x8020ada4;
            }
            0x8020ada4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8020ada8;
            }
            0x8020ada8 => {
                c.cmps(0, c.r[0], 0x2);
                pc = 0x8020adac;
            }
            0x8020adac => {
                if c.crbit(2) { pc = 0x8020ade0; continue; }
                pc = 0x8020adb0;
            }
            0x8020adb0 => {
                if !c.crbit(0) { pc = 0x8020adfc; continue; }
                pc = 0x8020adb4;
            }
            0x8020adb4 => {
                c.cmps(0, c.r[0], 0x1);
                pc = 0x8020adb8;
            }
            0x8020adb8 => {
                if !c.crbit(0) { pc = 0x8020adc0; continue; }
                pc = 0x8020adbc;
            }
            0x8020adbc => {
                pc = 0x8020adfc; continue;
                pc = 0x8020adc0;
            }
            0x8020adc0 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                pc = 0x8020adc4;
            }
            0x8020adc4 => {
                if !c.crbit(2) { pc = 0x8020add0; continue; }
                pc = 0x8020adc8;
            }
            0x8020adc8 => {
                c.r[0] = c.r[23].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                pc = 0x8020adcc;
            }
            0x8020adcc => {
                if c.crbit(2) { pc = 0x8020add8; continue; }
                pc = 0x8020add0;
            }
            0x8020add0 => {
                c.r[30] = 0x1;
                pc = 0x8020add4;
            }
            0x8020add4 => {
                pc = 0x8020adfc; continue;
                pc = 0x8020add8;
            }
            0x8020add8 => {
                c.r[30] = 0x0;
                pc = 0x8020addc;
            }
            0x8020addc => {
                pc = 0x8020adfc; continue;
                pc = 0x8020ade0;
            }
            0x8020ade0 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                pc = 0x8020ade4;
            }
            0x8020ade4 => {
                if c.crbit(2) { pc = 0x8020adf8; continue; }
                pc = 0x8020ade8;
            }
            0x8020ade8 => {
                c.r[0] = c.r[23].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                pc = 0x8020adec;
            }
            0x8020adec => {
                if c.crbit(2) { pc = 0x8020adf8; continue; }
                pc = 0x8020adf0;
            }
            0x8020adf0 => {
                c.r[30] = 0x1;
                pc = 0x8020adf4;
            }
            0x8020adf4 => {
                pc = 0x8020adfc; continue;
                pc = 0x8020adf8;
            }
            0x8020adf8 => {
                c.r[30] = 0x0;
                pc = 0x8020adfc;
            }
            0x8020adfc => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x8020ae00;
            }
            0x8020ae00 => {
                c.lr = 0x8020ae04; f_8020a500(c);
                pc = 0x8020ae04;
            }
            0x8020ae04 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                pc = 0x8020ae08;
            }
            0x8020ae08 => {
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8020ae0c;
            }
            0x8020ae0c => {
                if !c.crbit(2) { pc = 0x8020ab58; continue; }
                pc = 0x8020ae10;
            }
            0x8020ae10 => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x8020ae14;
            }
            0x8020ae14 => {
                pc = 0x8020ae1c; continue;
                pc = 0x8020ae18;
            }
            0x8020ae18 => {
                pc = 0x8020ab54; continue;
                pc = 0x8020ae1c;
            }
            0x8020ae1c => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 19..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                pc = 0x8020ae20;
            }
            0x8020ae20 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                pc = 0x8020ae24;
            }
            0x8020ae24 => {
                c.mtspr(8, c.r[0]);
                pc = 0x8020ae28;
            }
            0x8020ae28 => {
                c.r[1] = c.r[1].wrapping_add(0x50);
                pc = 0x8020ae2c;
            }
            0x8020ae2c => {
                if true { return; }
                return;
            }
            _ => panic!("f_8020a8e0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispWazaKiaipantiPre (0x8020ae30, 8 bytes)
#[inline(never)]
pub fn f_8020ae30(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae30) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae30); f_8020ae30_b(c, 0x8020ae30); crate::prof::exit(c, 0x8020ae30); return; }
    }
    f_8020ae30_b(c, 0x8020ae30)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae30_at(c: &mut Cpu, pc: u32) { f_8020ae30_b(c, pc) }
#[inline(always)]
fn f_8020ae30_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae30;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae30, pc); }
        match pc {
            0x8020ae30 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae30: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispTenkouInit (0x8020ae38, 8 bytes)
#[inline(never)]
pub fn f_8020ae38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae38) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae38); f_8020ae38_b(c, 0x8020ae38); crate::prof::exit(c, 0x8020ae38); return; }
    }
    f_8020ae38_b(c, 0x8020ae38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae38_at(c: &mut Cpu, pc: u32) { f_8020ae38_b(c, pc) }
#[inline(always)]
fn f_8020ae38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae38, pc); }
        match pc {
            0x8020ae38 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae38: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispHeijou (0x8020ae40, 8 bytes)
#[inline(never)]
pub fn f_8020ae40(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae40) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae40); f_8020ae40_b(c, 0x8020ae40); crate::prof::exit(c, 0x8020ae40); return; }
    }
    f_8020ae40_b(c, 0x8020ae40)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae40_at(c: &mut Cpu, pc: u32) { f_8020ae40_b(c, pc) }
#[inline(always)]
fn f_8020ae40_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae40;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae40, pc); }
        match pc {
            0x8020ae40 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae40: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispSyuuryouPost (0x8020ae48, 8 bytes)
#[inline(never)]
pub fn f_8020ae48(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae48) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae48); f_8020ae48_b(c, 0x8020ae48); crate::prof::exit(c, 0x8020ae48); return; }
    }
    f_8020ae48_b(c, 0x8020ae48)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae48_at(c: &mut Cpu, pc: u32) { f_8020ae48_b(c, pc) }
#[inline(always)]
fn f_8020ae48_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae48;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae48, pc); }
        match pc {
            0x8020ae48 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae48: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispSyuuryou (0x8020ae50, 8 bytes)
#[inline(never)]
pub fn f_8020ae50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae50) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae50); f_8020ae50_b(c, 0x8020ae50); crate::prof::exit(c, 0x8020ae50); return; }
    }
    f_8020ae50_b(c, 0x8020ae50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae50_at(c: &mut Cpu, pc: u32) { f_8020ae50_b(c, pc) }
#[inline(always)]
fn f_8020ae50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae50, pc); }
        match pc {
            0x8020ae50 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae50: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispSyuuryouPre (0x8020ae58, 8 bytes)
#[inline(never)]
pub fn f_8020ae58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae58) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae58); f_8020ae58_b(c, 0x8020ae58); crate::prof::exit(c, 0x8020ae58); return; }
    }
    f_8020ae58_b(c, 0x8020ae58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae58_at(c: &mut Cpu, pc: u32) { f_8020ae58_b(c, pc) }
#[inline(always)]
fn f_8020ae58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae58, pc); }
        match pc {
            0x8020ae58 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae58: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispFightOutPokemonOutWaza (0x8020ae60, 8 bytes)
#[inline(never)]
pub fn f_8020ae60(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae60) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae60); f_8020ae60_b(c, 0x8020ae60); crate::prof::exit(c, 0x8020ae60); return; }
    }
    f_8020ae60_b(c, 0x8020ae60)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae60_at(c: &mut Cpu, pc: u32) { f_8020ae60_b(c, pc) }
#[inline(always)]
fn f_8020ae60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae60, pc); }
        match pc {
            0x8020ae60 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae60: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispFightTrainerUseItem (0x8020ae68, 8 bytes)
#[inline(never)]
pub fn f_8020ae68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae68) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae68); f_8020ae68_b(c, 0x8020ae68); crate::prof::exit(c, 0x8020ae68); return; }
    }
    f_8020ae68_b(c, 0x8020ae68)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae68_at(c: &mut Cpu, pc: u32) { f_8020ae68_b(c, pc) }
#[inline(always)]
fn f_8020ae68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae68, pc); }
        match pc {
            0x8020ae68 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae68: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispFightTrainerCall (0x8020ae70, 8 bytes)
#[inline(never)]
pub fn f_8020ae70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae70) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae70); f_8020ae70_b(c, 0x8020ae70); crate::prof::exit(c, 0x8020ae70); return; }
    }
    f_8020ae70_b(c, 0x8020ae70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae70_at(c: &mut Cpu, pc: u32) { f_8020ae70_b(c, pc) }
#[inline(always)]
fn f_8020ae70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae70, pc); }
        match pc {
            0x8020ae70 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae70: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispFightOutPokemonIrekae (0x8020ae78, 8 bytes)
#[inline(never)]
pub fn f_8020ae78(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae78) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae78); f_8020ae78_b(c, 0x8020ae78); crate::prof::exit(c, 0x8020ae78); return; }
    }
    f_8020ae78_b(c, 0x8020ae78)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae78_at(c: &mut Cpu, pc: u32) { f_8020ae78_b(c, pc) }
#[inline(always)]
fn f_8020ae78_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae78;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae78, pc); }
        match pc {
            0x8020ae78 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae78: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispFightNigeru (0x8020ae80, 8 bytes)
#[inline(never)]
pub fn f_8020ae80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae80) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae80); f_8020ae80_b(c, 0x8020ae80); crate::prof::exit(c, 0x8020ae80); return; }
    }
    f_8020ae80_b(c, 0x8020ae80)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae80_at(c: &mut Cpu, pc: u32) { f_8020ae80_b(c, pc) }
#[inline(always)]
fn f_8020ae80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae80, pc); }
        match pc {
            0x8020ae80 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae80: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispOneTurnPost (0x8020ae88, 8 bytes)
#[inline(never)]
pub fn f_8020ae88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae88) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae88); f_8020ae88_b(c, 0x8020ae88); crate::prof::exit(c, 0x8020ae88); return; }
    }
    f_8020ae88_b(c, 0x8020ae88)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae88_at(c: &mut Cpu, pc: u32) { f_8020ae88_b(c, pc) }
#[inline(always)]
fn f_8020ae88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae88, pc); }
        match pc {
            0x8020ae88 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae88: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispAllFightOutPokemonDoFightAction (0x8020ae90, 8 bytes)
#[inline(never)]
pub fn f_8020ae90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae90) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae90); f_8020ae90_b(c, 0x8020ae90); crate::prof::exit(c, 0x8020ae90); return; }
    }
    f_8020ae90_b(c, 0x8020ae90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae90_at(c: &mut Cpu, pc: u32) { f_8020ae90_b(c, pc) }
#[inline(always)]
fn f_8020ae90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae90, pc); }
        match pc {
            0x8020ae90 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae90: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispAllFightTrainerSelectFightAction (0x8020ae98, 8 bytes)
#[inline(never)]
pub fn f_8020ae98(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ae98) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ae98); f_8020ae98_b(c, 0x8020ae98); crate::prof::exit(c, 0x8020ae98); return; }
    }
    f_8020ae98_b(c, 0x8020ae98)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ae98_at(c: &mut Cpu, pc: u32) { f_8020ae98_b(c, pc) }
#[inline(always)]
fn f_8020ae98_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ae98;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ae98, pc); }
        match pc {
            0x8020ae98 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020ae98: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispKaisiPost (0x8020aea0, 8 bytes)
#[inline(never)]
pub fn f_8020aea0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020aea0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aea0); f_8020aea0_b(c, 0x8020aea0); crate::prof::exit(c, 0x8020aea0); return; }
    }
    f_8020aea0_b(c, 0x8020aea0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aea0_at(c: &mut Cpu, pc: u32) { f_8020aea0_b(c, pc) }
#[inline(always)]
fn f_8020aea0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020aea0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020aea0, pc); }
        match pc {
            0x8020aea0 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020aea0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispKaisiPre (0x8020aea8, 8 bytes)
#[inline(never)]
pub fn f_8020aea8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020aea8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aea8); f_8020aea8_b(c, 0x8020aea8); crate::prof::exit(c, 0x8020aea8); return; }
    }
    f_8020aea8_b(c, 0x8020aea8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aea8_at(c: &mut Cpu, pc: u32) { f_8020aea8_b(c, pc) }
#[inline(always)]
fn f_8020aea8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020aea8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020aea8, pc); }
        match pc {
            0x8020aea8 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020aea8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispKaisiNyuujouPokemon (0x8020aeb0, 8 bytes)
#[inline(never)]
pub fn f_8020aeb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020aeb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aeb0); f_8020aeb0_b(c, 0x8020aeb0); crate::prof::exit(c, 0x8020aeb0); return; }
    }
    f_8020aeb0_b(c, 0x8020aeb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aeb0_at(c: &mut Cpu, pc: u32) { f_8020aeb0_b(c, pc) }
#[inline(always)]
fn f_8020aeb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020aeb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020aeb0, pc); }
        match pc {
            0x8020aeb0 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020aeb0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispKaisiNyuujouTrainer (0x8020aeb8, 8 bytes)
#[inline(never)]
pub fn f_8020aeb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020aeb8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aeb8); f_8020aeb8_b(c, 0x8020aeb8); crate::prof::exit(c, 0x8020aeb8); return; }
    }
    f_8020aeb8_b(c, 0x8020aeb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aeb8_at(c: &mut Cpu, pc: u32) { f_8020aeb8_b(c, pc) }
#[inline(always)]
fn f_8020aeb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020aeb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020aeb8, pc); }
        match pc {
            0x8020aeb8 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020aeb8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispKaijou (0x8020aec0, 8 bytes)
#[inline(never)]
pub fn f_8020aec0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020aec0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aec0); f_8020aec0_b(c, 0x8020aec0); crate::prof::exit(c, 0x8020aec0); return; }
    }
    f_8020aec0_b(c, 0x8020aec0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aec0_at(c: &mut Cpu, pc: u32) { f_8020aec0_b(c, pc) }
#[inline(always)]
fn f_8020aec0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020aec0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020aec0, pc); }
        match pc {
            0x8020aec0 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020aec0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDispNullFunc (0x8020aec8, 8 bytes)
#[inline(never)]
pub fn f_8020aec8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020aec8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aec8); f_8020aec8_b(c, 0x8020aec8); crate::prof::exit(c, 0x8020aec8); return; }
    }
    f_8020aec8_b(c, 0x8020aec8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aec8_at(c: &mut Cpu, pc: u32) { f_8020aec8_b(c, pc) }
#[inline(always)]
fn f_8020aec8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020aec8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020aec8, pc); }
        match pc {
            0x8020aec8 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020aec8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowWazaKiaipantiPre (0x8020aed0, 96 bytes)
#[inline(never)]
pub fn f_8020aed0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020aed0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aed0); f_8020aed0_b(c, 0x8020aed0); crate::prof::exit(c, 0x8020aed0); return; }
    }
    f_8020aed0_b(c, 0x8020aed0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aed0_at(c: &mut Cpu, pc: u32) { f_8020aed0_b(c, pc) }
#[inline(always)]
fn f_8020aed0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020aed0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020aed0, pc); }
        match pc {
            0x8020aed0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020aee8; f_8020d908(c);
                pc = 0x8020aee8;
            }
            0x8020aee8 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020af00; f_801f4c14(c);
                pc = 0x8020af00;
            }
            0x8020af00 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020af08; f_8020d8f0(c);
                pc = 0x8020af08;
            }
            0x8020af08 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = 0x0;
                c.lr = 0x8020af18; f_80211b94(c);
                pc = 0x8020af18;
            }
            0x8020af18 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020aed0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowTenkouInit (0x8020af30, 196 bytes)
#[inline(never)]
pub fn f_8020af30(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020af30) { return; }
        if c.prof { crate::prof::enter(c, 0x8020af30); f_8020af30_b(c, 0x8020af30); crate::prof::exit(c, 0x8020af30); return; }
    }
    f_8020af30_b(c, 0x8020af30)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020af30_at(c: &mut Cpu, pc: u32) { f_8020af30_b(c, pc) }
#[inline(always)]
fn f_8020af30_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020af30;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020af30, pc); }
        match pc {
            0x8020af30 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020af50; f_8020da14(c);
                pc = 0x8020af50;
            }
            0x8020af50 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8020af5c; f_801f453c(c);
                pc = 0x8020af5c;
            }
            0x8020af5c => {
                c.r[0] = 0x0;
                c.r[4] = 0x80210000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[31] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = c.r[4].wrapping_add(0xffffaff4);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x8020af80; f_801f37b0(c);
                pc = 0x8020af80;
            }
            0x8020af80 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8020afdc; continue; }
                pc = 0x8020af88;
            }
            0x8020af88 => {
                c.r[7] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020afa0; f_801f4c14(c);
                pc = 0x8020afa0;
            }
            0x8020afa0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020afa8; f_80136368(c);
                pc = 0x8020afa8;
            }
            0x8020afa8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x50;
                c.r[6] = 0x0;
                c.lr = 0x8020afc4; f_801f4c14(c);
                pc = 0x8020afc4;
            }
            0x8020afc4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020afcc; f_8020d8f0(c);
                pc = 0x8020afcc;
            }
            0x8020afcc => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x8020afdc; f_80211b94(c);
                pc = 0x8020afdc;
            }
            0x8020afdc => {
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020af30: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowTenkouInitSubGetSeqFightOutPokemonPtr__FPvUsPv (0x8020aff4, 92 bytes)
#[inline(never)]
pub fn f_8020aff4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020aff4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020aff4); f_8020aff4_b(c, 0x8020aff4); crate::prof::exit(c, 0x8020aff4); return; }
    }
    f_8020aff4_b(c, 0x8020aff4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020aff4_at(c: &mut Cpu, pc: u32) { f_8020aff4_b(c, pc) }
#[inline(always)]
fn f_8020aff4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020aff4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020aff4, pc); }
        match pc {
            0x8020aff4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[5] | c.r[5];
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0xee;
                c.lr = 0x8020b01c; f_8012640c(c);
                pc = 0x8020b01c;
            }
            0x8020b01c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020b038; continue; }
                pc = 0x8020b024;
            }
            0x8020b024 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8020b030; continue; }
                pc = 0x8020b02c;
            }
            0x8020b02c => {
                c.stw(c.r[31], c.r[30]);
                pc = 0x8020b030;
            }
            0x8020b030 => {
                c.r[3] = 0x0;
                pc = 0x8020b03c; continue;
                pc = 0x8020b038;
            }
            0x8020b038 => {
                c.r[3] = 0x1;
                pc = 0x8020b03c;
            }
            0x8020b03c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020aff4: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowHeijou (0x8020b050, 8 bytes)
#[inline(never)]
pub fn f_8020b050(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020b050) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b050); f_8020b050_b(c, 0x8020b050); crate::prof::exit(c, 0x8020b050); return; }
    }
    f_8020b050_b(c, 0x8020b050)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b050_at(c: &mut Cpu, pc: u32) { f_8020b050_b(c, pc) }
#[inline(always)]
fn f_8020b050_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020b050;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020b050, pc); }
        match pc {
            0x8020b050 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020b050: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowSyuuryouPost (0x8020b058, 728 bytes)
#[inline(never)]
pub fn f_8020b058(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020b058) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b058); f_8020b058_b(c, 0x8020b058); crate::prof::exit(c, 0x8020b058); return; }
    }
    f_8020b058_b(c, 0x8020b058)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b058_at(c: &mut Cpu, pc: u32) { f_8020b058_b(c, pc) }
#[inline(always)]
fn f_8020b058_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020b058;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020b058, pc); }
        match pc {
            0x8020b058 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020b07c; f_801f54a4(c);
                pc = 0x8020b07c;
            }
            0x8020b07c => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8020b084; f_801ef634(c);
                pc = 0x8020b084;
            }
            0x8020b084 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b098; continue; }
                pc = 0x8020b090;
            }
            0x8020b090 => {
                c.r[3] = 0x1;
                pc = 0x8020b31c; continue;
                pc = 0x8020b098;
            }
            0x8020b098 => {
                c.r[3] = 0x0;
                c.lr = 0x8020b0a0; f_801f2a7c(c);
                pc = 0x8020b0a0;
            }
            0x8020b0a0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x8020b268; continue; }
                pc = 0x8020b0a8;
            }
            0x8020b0a8 => {
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x8020b0b8; f_801fb1c0(c);
                pc = 0x8020b0b8;
            }
            0x8020b0b8 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x8020b268; continue; }
                pc = 0x8020b0c0;
            }
            0x8020b0c0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8020b0cc; f_801f86c0(c);
                pc = 0x8020b0cc;
            }
            0x8020b0cc => {
                c.lr = 0x8020b0d0; f_801ef634(c);
                pc = 0x8020b0d0;
            }
            0x8020b0d0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020b0dc; f_801f1dbc(c);
                pc = 0x8020b0dc;
            }
            0x8020b0dc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b22c; continue; }
                pc = 0x8020b0e8;
            }
            0x8020b0e8 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x24;
                c.r[6] = 0x0;
                c.lr = 0x8020b0fc; f_801f54a4(c);
                pc = 0x8020b0fc;
            }
            0x8020b0fc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b1f0; continue; }
                pc = 0x8020b108;
            }
            0x8020b108 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020b110; f_801f9034(c);
                pc = 0x8020b110;
            }
            0x8020b110 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b1f0; continue; }
                pc = 0x8020b11c;
            }
            0x8020b11c => {
                c.r[26] = 0x0;
                pc = 0x8020b1e4; continue;
                pc = 0x8020b124;
            }
            0x8020b124 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[26].rotate_left(0) & 0xffff;
                c.r[4] = 0x3;
                c.lr = 0x8020b134; f_8012a5b0(c);
                pc = 0x8020b134;
            }
            0x8020b134 => {
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x8020b13c; f_801233f4(c);
                pc = 0x8020b13c;
            }
            0x8020b13c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b144;
            }
            0x8020b144 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020b14c; f_8011fc74(c);
                pc = 0x8020b14c;
            }
            0x8020b14c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b158;
            }
            0x8020b158 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8020b164; f_801f9930(c);
                pc = 0x8020b164;
            }
            0x8020b164 => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if c.crbit(2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b16c;
            }
            0x8020b16c => {
                c.lr = 0x8020b170; f_80206608(c);
                pc = 0x8020b170;
            }
            0x8020b170 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b178;
            }
            0x8020b178 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0xd0;
                c.r[6] = 0x0;
                c.lr = 0x8020b18c; f_8012640c(c);
                pc = 0x8020b18c;
            }
            0x8020b18c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b194;
            }
            0x8020b194 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[7] = c.r[1].wrapping_add(0xc);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8020b1ac; f_80128a64(c);
                pc = 0x8020b1ac;
            }
            0x8020b1ac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[4] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x8020b1e0; continue; }
                pc = 0x8020b1b8;
            }
            0x8020b1b8 => {
                c.r[5] = c.lhz(c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[27] | c.r[27];
                c.r[7] = c.r[30] | c.r[30];
                c.r[6] = c.r[1].wrapping_add(0xc);
                c.r[8] = 0x1;
                c.r[9] = 0x1;
                c.r[10] = 0x0;
                c.lr = 0x8020b1d8; f_8012805c(c);
                pc = 0x8020b1d8;
            }
            0x8020b1d8 => {
                c.r[3] = 0xa;
                c.lr = 0x8020b1e0; f_801effc4(c);
                pc = 0x8020b1e0;
            }
            0x8020b1e0 => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8020b1e4;
            }
            0x8020b1e4 => {
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x6);
                if c.crbit(0) { pc = 0x8020b124; continue; }
                pc = 0x8020b1f0;
            }
            0x8020b1f0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x30;
                c.r[6] = 0x0;
                c.lr = 0x8020b204; f_801f54a4(c);
                pc = 0x8020b204;
            }
            0x8020b204 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b218; continue; }
                pc = 0x8020b210;
            }
            0x8020b210 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020b218; f_80129840(c);
                pc = 0x8020b218;
            }
            0x8020b218 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x28;
                c.r[6] = 0x0;
                c.lr = 0x8020b22c; f_801f54a4(c);
                pc = 0x8020b22c;
            }
            0x8020b22c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x1c;
                c.r[6] = 0x0;
                c.lr = 0x8020b240; f_801f54a4(c);
                pc = 0x8020b240;
            }
            0x8020b240 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b268; continue; }
                pc = 0x8020b24c;
            }
            0x8020b24c => {
                c.r[3] = 0x0;
                c.r[4] = 0x2;
                c.lr = 0x8020b258; f_80129280(c);
                pc = 0x8020b258;
            }
            0x8020b258 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020b268; continue; }
                pc = 0x8020b260;
            }
            0x8020b260 => {
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020b268; f_8012ac64(c);
                pc = 0x8020b268;
            }
            0x8020b268 => {
                c.lr = 0x8020b26c; f_8006b57c(c);
                pc = 0x8020b26c;
            }
            0x8020b26c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b318; continue; }
                pc = 0x8020b278;
            }
            0x8020b278 => {
                c.r[26] = 0x0;
                pc = 0x8020b30c; continue;
                pc = 0x8020b280;
            }
            0x8020b280 => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.lr = 0x8020b28c; f_801f47b4(c);
                pc = 0x8020b28c;
            }
            0x8020b28c => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if c.crbit(2) { pc = 0x8020b308; continue; }
                pc = 0x8020b294;
            }
            0x8020b294 => {
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.r[27] = 0x0;
                c.r[30] = c.r[0].wrapping_mul(c.r[29]);
                pc = 0x8020b2fc; continue;
                pc = 0x8020b2a4;
            }
            0x8020b2a4 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8020b2b0; f_801f7258(c);
                pc = 0x8020b2b0;
            }
            0x8020b2b0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x8020b2f8; continue; }
                pc = 0x8020b2b8;
            }
            0x8020b2b8 => {
                c.r[4] = 0x0;
                c.lr = 0x8020b2c0; f_801f86c0(c);
                pc = 0x8020b2c0;
            }
            0x8020b2c0 => {
                c.r[0] = c.r[27].wrapping_add(c.r[30]);
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x8020b2cc; f_8006b0f8(c);
                pc = 0x8020b2cc;
            }
            0x8020b2cc => {
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                if c.crbit(2) { pc = 0x8020b2f8; continue; }
                pc = 0x8020b2d4;
            }
            0x8020b2d4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x8020b2e8; f_801fb1c0(c);
                pc = 0x8020b2e8;
            }
            0x8020b2e8 => {
                c.r[4] = c.r[3] | c.r[3];
                c.rc(c.r[4]);
                if c.crbit(2) { pc = 0x8020b2f8; continue; }
                pc = 0x8020b2f0;
            }
            0x8020b2f0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020b2f8; f_8012ac64(c);
                pc = 0x8020b2f8;
            }
            0x8020b2f8 => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x8020b2fc;
            }
            0x8020b2fc => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[29]);
                if c.crbit(0) { pc = 0x8020b2a4; continue; }
                pc = 0x8020b308;
            }
            0x8020b308 => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8020b30c;
            }
            0x8020b30c => {
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8020b280; continue; }
                pc = 0x8020b318;
            }
            0x8020b318 => {
                c.r[3] = 0x1;
                pc = 0x8020b31c;
            }
            0x8020b31c => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8020b058: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowSyuuryou (0x8020b330, 932 bytes)
#[inline(never)]
pub fn f_8020b330(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020b330) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b330); f_8020b330_b(c, 0x8020b330); crate::prof::exit(c, 0x8020b330); return; }
    }
    f_8020b330_b(c, 0x8020b330)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b330_at(c: &mut Cpu, pc: u32) { f_8020b330_b(c, pc) }
#[inline(always)]
fn f_8020b330_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020b330;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020b330, pc); }
        match pc {
            0x8020b330 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020b348; f_8020d920(c);
                pc = 0x8020b348;
            }
            0x8020b348 => {
                c.lr = 0x8020b34c; f_8020d814(c);
                pc = 0x8020b34c;
            }
            0x8020b34c => {
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0xb;
                c.r[4] = 0x0;
                c.lr = 0x8020b35c; f_801f025c(c);
                pc = 0x8020b35c;
            }
            0x8020b35c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x9;
                c.r[25] = c.r[0] | c.r[0];
                c.r[4] = c.r[25] | c.r[25];
                c.lr = 0x8020b370; f_801f025c(c);
                pc = 0x8020b370;
            }
            0x8020b370 => {
                c.r[4] = 0x0;
                c.r[24] = c.r[3] | c.r[3];
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x8020b384; f_801fb1c0(c);
                pc = 0x8020b384;
            }
            0x8020b384 => {
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020b39c; f_801fb1c0(c);
                pc = 0x8020b39c;
            }
            0x8020b39c => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x1;
                c.lr = 0x8020b3b4; f_801fb1c0(c);
                pc = 0x8020b3b4;
            }
            0x8020b3b4 => {
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8020b3bc; f_800896b8(c);
                pc = 0x8020b3bc;
            }
            0x8020b3bc => {
                c.cmpu(0, c.r[26], c.r[3]);
                if !c.crbit(2) { pc = 0x8020b3ec; continue; }
                pc = 0x8020b3c4;
            }
            0x8020b3c4 => {
                c.lr = 0x8020b3c8; f_800896c0(c);
                pc = 0x8020b3c8;
            }
            0x8020b3c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020b3d8; continue; }
                pc = 0x8020b3d0;
            }
            0x8020b3d0 => {
                c.r[23] = 0x0;
                pc = 0x8020b458; continue;
                pc = 0x8020b3d8;
            }
            0x8020b3d8 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x24;
                c.lr = 0x8020b3e4; f_80132a38(c);
                pc = 0x8020b3e4;
            }
            0x8020b3e4 => {
                c.r[23] = 0x7531;
                pc = 0x8020b458; continue;
                pc = 0x8020b3ec;
            }
            0x8020b3ec => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x4a;
                c.r[6] = 0x0;
                c.lr = 0x8020b400; f_801fb1c0(c);
                pc = 0x8020b400;
            }
            0x8020b400 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020b424; continue; }
                pc = 0x8020b408;
            }
            0x8020b408 => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x2;
                c.lr = 0x8020b41c; f_801fb1c0(c);
                pc = 0x8020b41c;
            }
            0x8020b41c => {
                c.r[23] = c.r[3] | c.r[3];
                pc = 0x8020b458; continue;
                pc = 0x8020b424;
            }
            0x8020b424 => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x3;
                c.lr = 0x8020b438; f_801fb1c0(c);
                pc = 0x8020b438;
            }
            0x8020b438 => {
                c.r[23] = c.r[3] | c.r[3];
                c.rc(c.r[23]);
                if !c.crbit(2) { pc = 0x8020b458; continue; }
                pc = 0x8020b440;
            }
            0x8020b440 => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x2;
                c.lr = 0x8020b454; f_801fb1c0(c);
                pc = 0x8020b454;
            }
            0x8020b454 => {
                c.r[23] = c.r[3] | c.r[3];
                pc = 0x8020b458;
            }
            0x8020b458 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020b460; f_801f8000(c);
                pc = 0x8020b460;
            }
            0x8020b460 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x22;
                c.lr = 0x8020b46c; f_80132a38(c);
                pc = 0x8020b46c;
            }
            0x8020b46c => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020b474; f_801f8100(c);
                pc = 0x8020b474;
            }
            0x8020b474 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x23;
                c.lr = 0x8020b480; f_80132a38(c);
                pc = 0x8020b480;
            }
            0x8020b480 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8020b488; f_801f8100(c);
                pc = 0x8020b488;
            }
            0x8020b488 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x13;
                c.lr = 0x8020b494; f_80132a38(c);
                pc = 0x8020b494;
            }
            0x8020b494 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020b49c; f_801f8100(c);
                pc = 0x8020b49c;
            }
            0x8020b49c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x25;
                c.lr = 0x8020b4a8; f_80132a38(c);
                pc = 0x8020b4a8;
            }
            0x8020b4a8 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020b4bc; f_801f54a4(c);
                pc = 0x8020b4bc;
            }
            0x8020b4bc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b654; continue; }
                pc = 0x8020b4c8;
            }
            0x8020b4c8 => {
                c.cmpu(0, c.r[30], 0x2);
                if !c.crbit(2) { pc = 0x8020b57c; continue; }
                pc = 0x8020b4d0;
            }
            0x8020b4d0 => {
                c.cmpu(0, c.r[23], 0x0);
                if c.crbit(2) { pc = 0x8020b4f4; continue; }
                pc = 0x8020b4d8;
            }
            0x8020b4d8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x5a;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020b4ec; f_801ddd28(c);
                pc = 0x8020b4ec;
            }
            0x8020b4ec => {
                c.lr = 0x8020b4f0; f_801c2d54(c);
                pc = 0x8020b4f0;
            }
            0x8020b4f0 => {
                c.r[27] = c.r[3] | c.r[3];
                pc = 0x8020b4f4;
            }
            0x8020b4f4 => {
                c.r[3] = 0x3f5;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x8020b504; f_80165668(c);
                pc = 0x8020b504;
            }
            0x8020b504 => {
                c.r[3] = 0x5d;
                c.r[4] = 0x0;
                c.lr = 0x8020b510; f_80132a38(c);
                pc = 0x8020b510;
            }
            0x8020b510 => {
                c.r[3] = 0x766c;
                c.lr = 0x8020b518; f_802624cc(c);
                pc = 0x8020b518;
            }
            0x8020b518 => {
                c.lr = 0x8020b51c; f_8026246c(c);
                pc = 0x8020b51c;
            }
            0x8020b51c => {
                c.cmpu(0, c.r[23], 0x0);
                if c.crbit(2) { pc = 0x8020b654; continue; }
                pc = 0x8020b524;
            }
            0x8020b524 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x5a;
                c.r[5] = 0x4;
                c.lr = 0x8020b534; f_801da9e8(c);
                pc = 0x8020b534;
            }
            0x8020b534 => {
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x8020b53c; f_80262490(c);
                pc = 0x8020b53c;
            }
            0x8020b53c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x5a;
                c.r[5] = 0x4;
                c.lr = 0x8020b54c; f_801da94c(c);
                pc = 0x8020b54c;
            }
            0x8020b54c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020b55c; continue; }
                pc = 0x8020b554;
            }
            0x8020b554 => {
                c.lr = 0x8020b558; crate::stubs::f_800f0308(c);
                pc = 0x8020b558;
            }
            0x8020b558 => {
                pc = 0x8020b53c; continue;
                pc = 0x8020b55c;
            }
            0x8020b55c => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020b564; f_801ef8f4(c);
                pc = 0x8020b564;
            }
            0x8020b564 => {
                c.lr = 0x8020b568; f_8026246c(c);
                pc = 0x8020b568;
            }
            0x8020b568 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x5a;
                c.r[5] = 0x4;
                c.lr = 0x8020b578; f_801da8c4(c);
                pc = 0x8020b578;
            }
            0x8020b578 => {
                pc = 0x8020b654; continue;
                pc = 0x8020b57c;
            }
            0x8020b57c => {
                c.cmpu(0, c.r[30], 0x3);
                if !c.crbit(2) { pc = 0x8020b620; continue; }
                pc = 0x8020b584;
            }
            0x8020b584 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x8020b5a8; continue; }
                pc = 0x8020b58c;
            }
            0x8020b58c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x59;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020b5a0; f_801ddd28(c);
                pc = 0x8020b5a0;
            }
            0x8020b5a0 => {
                c.lr = 0x8020b5a4; f_801c2d54(c);
                pc = 0x8020b5a4;
            }
            0x8020b5a4 => {
                c.r[27] = c.r[3] | c.r[3];
                pc = 0x8020b5a8;
            }
            0x8020b5a8 => {
                c.r[3] = 0x7547;
                c.lr = 0x8020b5b0; f_802624cc(c);
                pc = 0x8020b5b0;
            }
            0x8020b5b0 => {
                c.lr = 0x8020b5b4; f_8026246c(c);
                pc = 0x8020b5b4;
            }
            0x8020b5b4 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x8020b610; continue; }
                pc = 0x8020b5bc;
            }
            0x8020b5bc => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x59;
                c.r[5] = 0x4;
                c.lr = 0x8020b5cc; f_801da9e8(c);
                pc = 0x8020b5cc;
            }
            0x8020b5cc => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020b5d4; f_80262490(c);
                pc = 0x8020b5d4;
            }
            0x8020b5d4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x59;
                c.r[5] = 0x4;
                c.lr = 0x8020b5e4; f_801da94c(c);
                pc = 0x8020b5e4;
            }
            0x8020b5e4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020b5f4; continue; }
                pc = 0x8020b5ec;
            }
            0x8020b5ec => {
                c.lr = 0x8020b5f0; crate::stubs::f_800f0308(c);
                pc = 0x8020b5f0;
            }
            0x8020b5f0 => {
                pc = 0x8020b5d4; continue;
                pc = 0x8020b5f4;
            }
            0x8020b5f4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020b5fc; f_801ef8f4(c);
                pc = 0x8020b5fc;
            }
            0x8020b5fc => {
                c.lr = 0x8020b600; f_8026246c(c);
                pc = 0x8020b600;
            }
            0x8020b600 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x59;
                c.r[5] = 0x4;
                c.lr = 0x8020b610; f_801da8c4(c);
                pc = 0x8020b610;
            }
            0x8020b610 => {
                c.r[3] = 0x7548;
                c.lr = 0x8020b618; f_802624cc(c);
                pc = 0x8020b618;
            }
            0x8020b618 => {
                c.lr = 0x8020b61c; f_8026246c(c);
                pc = 0x8020b61c;
            }
            0x8020b61c => {
                pc = 0x8020b654; continue;
                pc = 0x8020b620;
            }
            0x8020b620 => {
                c.r[0] = c.r[30].wrapping_add(0xfffffffc);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x8020b654; continue; }
                pc = 0x8020b630;
            }
            0x8020b630 => {
                c.cmpu(0, c.r[30], 0x7);
                if c.crbit(2) { pc = 0x8020b640; continue; }
                pc = 0x8020b638;
            }
            0x8020b638 => {
                c.cmpu(0, c.r[30], 0x6);
                if !c.crbit(2) { pc = 0x8020b654; continue; }
                pc = 0x8020b640;
            }
            0x8020b640 => {
                c.r[3] = 0x7640;
                c.lr = 0x8020b648; f_802624cc(c);
                pc = 0x8020b648;
            }
            0x8020b648 => {
                c.r[3] = 0x40;
                c.lr = 0x8020b650; f_801f000c(c);
                pc = 0x8020b650;
            }
            0x8020b650 => {
                c.lr = 0x8020b654; f_8026246c(c);
                pc = 0x8020b654;
            }
            0x8020b654 => {
                c.lr = 0x8020b658; f_801ef634(c);
                pc = 0x8020b658;
            }
            0x8020b658 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020b664; f_801f1dbc(c);
                pc = 0x8020b664;
            }
            0x8020b664 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b6b8; continue; }
                pc = 0x8020b670;
            }
            0x8020b670 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x25;
                c.r[6] = 0x0;
                c.lr = 0x8020b684; f_801f54a4(c);
                pc = 0x8020b684;
            }
            0x8020b684 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b6b8; continue; }
                pc = 0x8020b690;
            }
            0x8020b690 => {
                c.r[4] = 0x80380000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[4].wrapping_add(0xffff8801);
                c.r[5] = 0x0;
                c.lr = 0x8020b6a4; f_80211b94(c);
                pc = 0x8020b6a4;
            }
            0x8020b6a4 => {
                c.r[4] = 0x80380000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[4].wrapping_add(0xffff880f);
                c.r[5] = 0x0;
                c.lr = 0x8020b6b8; f_80211b94(c);
                pc = 0x8020b6b8;
            }
            0x8020b6b8 => {
                c.lr = 0x8020b6bc; f_801ef2d4(c);
                pc = 0x8020b6bc;
            }
            0x8020b6bc => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8020b330: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowSyuuryouPre (0x8020b6d4, 88 bytes)
#[inline(never)]
pub fn f_8020b6d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020b6d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b6d4); f_8020b6d4_b(c, 0x8020b6d4); crate::prof::exit(c, 0x8020b6d4); return; }
    }
    f_8020b6d4_b(c, 0x8020b6d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b6d4_at(c: &mut Cpu, pc: u32) { f_8020b6d4_b(c, pc) }
#[inline(always)]
fn f_8020b6d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020b6d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020b6d4, pc); }
        match pc {
            0x8020b6d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0x12;
                c.r[6] = 0x0;
                c.lr = 0x8020b6f4; f_801f54a4(c);
                pc = 0x8020b6f4;
            }
            0x8020b6f4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020b718; continue; }
                pc = 0x8020b6fc;
            }
            0x8020b6fc => {
                c.r[3] = 0x1;
                c.r[4] = 0x3e8;
                c.r[5] = 0x3e8;
                c.r[6] = 0xff;
                c.lr = 0x8020b710; f_8016597c(c);
                pc = 0x8020b710;
            }
            0x8020b710 => {
                c.r[3] = 0x3c;
                c.lr = 0x8020b718; f_801f000c(c);
                pc = 0x8020b718;
            }
            0x8020b718 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020b6d4: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowFightOutPokemonOutWaza (0x8020b72c, 484 bytes)
#[inline(never)]
pub fn f_8020b72c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020b72c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b72c); f_8020b72c_b(c, 0x8020b72c); crate::prof::exit(c, 0x8020b72c); return; }
    }
    f_8020b72c_b(c, 0x8020b72c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b72c_at(c: &mut Cpu, pc: u32) { f_8020b72c_b(c, pc) }
#[inline(always)]
fn f_8020b72c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020b72c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020b72c, pc); }
        match pc {
            0x8020b72c => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020b754; f_801f54a4(c);
                pc = 0x8020b754;
            }
            0x8020b754 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020b760; f_8020d908(c);
                pc = 0x8020b760;
            }
            0x8020b760 => {
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8020b774; f_8012640c(c);
                pc = 0x8020b774;
            }
            0x8020b774 => {
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x8020b77c; f_80209cb4(c);
                pc = 0x8020b77c;
            }
            0x8020b77c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020b78c; continue; }
                pc = 0x8020b784;
            }
            0x8020b784 => {
                c.r[3] = 0x0;
                pc = 0x8020b8fc; continue;
                pc = 0x8020b78c;
            }
            0x8020b78c => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x29;
                c.r[6] = 0x0;
                c.lr = 0x8020b7a0; f_8011beb4(c);
                pc = 0x8020b7a0;
            }
            0x8020b7a0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8020b7ac; f_801f00d0(c);
                pc = 0x8020b7ac;
            }
            0x8020b7ac => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[7] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020b7c8; f_801f4c14(c);
                pc = 0x8020b7c8;
            }
            0x8020b7c8 => {
                c.r[7] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x8020b7e0; f_801f4c14(c);
                pc = 0x8020b7e0;
            }
            0x8020b7e0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020b7e8; f_80205b8c(c);
                pc = 0x8020b7e8;
            }
            0x8020b7e8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[25] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0xd9;
                c.r[6] = 0x0;
                c.lr = 0x8020b804; f_8012640c(c);
                pc = 0x8020b804;
            }
            0x8020b804 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020b810; f_80205224(c);
                pc = 0x8020b810;
            }
            0x8020b810 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[26] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x26;
                c.r[6] = 0x0;
                c.lr = 0x8020b82c; f_8011beb4(c);
                pc = 0x8020b82c;
            }
            0x8020b82c => {
                c.r[29] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.r[5] = 0x32;
                c.r[6] = 0x0;
                c.lr = 0x8020b844; f_8011beb4(c);
                pc = 0x8020b844;
            }
            0x8020b844 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020b8ec; continue; }
                pc = 0x8020b84c;
            }
            0x8020b84c => {
                c.r[29] = c.r[29].rotate_left(0) & 0xff;
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.r[6] = c.r[29] | c.r[29];
                c.lr = 0x8020b864; f_8012640c(c);
                pc = 0x8020b864;
            }
            0x8020b864 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if c.crbit(2) { pc = 0x8020b8ec; continue; }
                pc = 0x8020b874;
            }
            0x8020b874 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[6] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0x7f;
                c.lr = 0x8020b888; f_8012640c(c);
                pc = 0x8020b888;
            }
            0x8020b888 => {
                c.r[26] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.r[7] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x8020b8a4; f_8011bbd8(c);
                pc = 0x8020b8a4;
            }
            0x8020b8a4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x8020b8b0; f_80209c1c(c);
                pc = 0x8020b8b0;
            }
            0x8020b8b0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                c.r[9] = 0xffffffff;
                c.lr = 0x8020b8d0; f_8022b2cc(c);
                pc = 0x8020b8d0;
            }
            0x8020b8d0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x8020b8ec; f_801f4c14(c);
                pc = 0x8020b8ec;
            }
            0x8020b8ec => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x8020b8f8; f_802128d0(c);
                pc = 0x8020b8f8;
            }
            0x8020b8f8 => {
                c.r[3] = 0x1;
                pc = 0x8020b8fc;
            }
            0x8020b8fc => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8020b72c: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowFightTrainerUseItem (0x8020b910, 260 bytes)
#[inline(never)]
pub fn f_8020b910(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020b910) { return; }
        if c.prof { crate::prof::enter(c, 0x8020b910); f_8020b910_b(c, 0x8020b910); crate::prof::exit(c, 0x8020b910); return; }
    }
    f_8020b910_b(c, 0x8020b910)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020b910_at(c: &mut Cpu, pc: u32) { f_8020b910_b(c, pc) }
#[inline(always)]
fn f_8020b910_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020b910;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020b910, pc); }
        match pc {
            0x8020b910 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020b938; f_801f54a4(c);
                pc = 0x8020b938;
            }
            0x8020b938 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020b944; f_8020d908(c);
                pc = 0x8020b944;
            }
            0x8020b944 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[7] = c.r[31] | c.r[31];
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020b964; f_801f4c14(c);
                pc = 0x8020b964;
            }
            0x8020b964 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0xe5;
                c.r[6] = 0x0;
                c.lr = 0x8020b978; f_8012640c(c);
                pc = 0x8020b978;
            }
            0x8020b978 => {
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.lr = 0x8020b98c; f_80142cf4(c);
                pc = 0x8020b98c;
            }
            0x8020b98c => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020b9a4; f_80142cf4(c);
                pc = 0x8020b9a4;
            }
            0x8020b9a4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020b9d4; continue; }
                pc = 0x8020b9b0;
            }
            0x8020b9b0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x1f;
                c.r[6] = 0x0;
                c.lr = 0x8020b9c4; f_80142cf4(c);
                pc = 0x8020b9c4;
            }
            0x8020b9c4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8020b9d0; f_801f00d0(c);
                pc = 0x8020b9d0;
            }
            0x8020b9d0 => {
                pc = 0x8020b9d8; continue;
                pc = 0x8020b9d4;
            }
            0x8020b9d4 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x8020b9d8;
            }
            0x8020b9d8 => {
                c.r[7] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x42;
                c.r[6] = 0x0;
                c.lr = 0x8020b9f0; f_801f4c14(c);
                pc = 0x8020b9f0;
            }
            0x8020b9f0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8020b9fc; f_80211e18(c);
                pc = 0x8020b9fc;
            }
            0x8020b9fc => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020b910: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowFightTrainerCall (0x8020ba14, 108 bytes)
#[inline(never)]
pub fn f_8020ba14(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ba14) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ba14); f_8020ba14_b(c, 0x8020ba14); crate::prof::exit(c, 0x8020ba14); return; }
    }
    f_8020ba14_b(c, 0x8020ba14)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ba14_at(c: &mut Cpu, pc: u32) { f_8020ba14_b(c, pc) }
#[inline(always)]
fn f_8020ba14_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ba14;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ba14, pc); }
        match pc {
            0x8020ba14 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020ba3c; f_801f54a4(c);
                pc = 0x8020ba3c;
            }
            0x8020ba3c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020ba44; f_8020d908(c);
                pc = 0x8020ba44;
            }
            0x8020ba44 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020ba60; f_801f4c14(c);
                pc = 0x8020ba60;
            }
            0x8020ba60 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020ba68; f_80212d6c(c);
                pc = 0x8020ba68;
            }
            0x8020ba68 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020ba14: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowFightOutPokemonIrekae (0x8020ba80, 120 bytes)
#[inline(never)]
pub fn f_8020ba80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ba80) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ba80); f_8020ba80_b(c, 0x8020ba80); crate::prof::exit(c, 0x8020ba80); return; }
    }
    f_8020ba80_b(c, 0x8020ba80)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ba80_at(c: &mut Cpu, pc: u32) { f_8020ba80_b(c, pc) }
#[inline(always)]
fn f_8020ba80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ba80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ba80, pc); }
        match pc {
            0x8020ba80 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x8020ba98; f_8020d908(c);
                pc = 0x8020ba98;
            }
            0x8020ba98 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[7] = c.r[31] | c.r[31];
                c.r[5] = 0x45;
                c.r[6] = 0x0;
                c.lr = 0x8020bab8; f_801f4c14(c);
                pc = 0x8020bab8;
            }
            0x8020bab8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020bac0; f_8020d8d8(c);
                pc = 0x8020bac0;
            }
            0x8020bac0 => {
                c.r[7] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x121;
                c.r[6] = 0x0;
                c.lr = 0x8020bad8; f_801254b4(c);
                pc = 0x8020bad8;
            }
            0x8020bad8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020bae0; f_80213158(c);
                pc = 0x8020bae0;
            }
            0x8020bae0 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020ba80: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowFightNigeru (0x8020baf8, 172 bytes)
#[inline(never)]
pub fn f_8020baf8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020baf8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020baf8); f_8020baf8_b(c, 0x8020baf8); crate::prof::exit(c, 0x8020baf8); return; }
    }
    f_8020baf8_b(c, 0x8020baf8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020baf8_at(c: &mut Cpu, pc: u32) { f_8020baf8_b(c, pc) }
#[inline(always)]
fn f_8020baf8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020baf8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020baf8, pc); }
        match pc {
            0x8020baf8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020bb20; f_801f54a4(c);
                pc = 0x8020bb20;
            }
            0x8020bb20 => {
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020bb2c; f_8020d908(c);
                pc = 0x8020bb2c;
            }
            0x8020bb2c => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020bb34; f_80212840(c);
                pc = 0x8020bb34;
            }
            0x8020bb34 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8020bb40; f_801f0058(c);
                pc = 0x8020bb40;
            }
            0x8020bb40 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020bb5c; continue; }
                pc = 0x8020bb4c;
            }
            0x8020bb4c => {
                c.r[3] = 0x0;
                c.r[4] = 0x4;
                c.lr = 0x8020bb58; f_801f3984(c);
                pc = 0x8020bb58;
            }
            0x8020bb58 => {
                pc = 0x8020bb68; continue;
                pc = 0x8020bb5c;
            }
            0x8020bb5c => {
                c.r[3] = 0x0;
                c.r[4] = 0x5;
                c.lr = 0x8020bb68; f_801f3984(c);
                pc = 0x8020bb68;
            }
            0x8020bb68 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020bb8c; continue; }
                pc = 0x8020bb74;
            }
            0x8020bb74 => {
                c.r[7] = c.r[31] | c.r[31];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x8020bb8c; f_801f4c14(c);
                pc = 0x8020bb8c;
            }
            0x8020bb8c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020baf8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowOneTurnPost (0x8020bba4, 88 bytes)
#[inline(never)]
pub fn f_8020bba4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020bba4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020bba4); f_8020bba4_b(c, 0x8020bba4); crate::prof::exit(c, 0x8020bba4); return; }
    }
    f_8020bba4_b(c, 0x8020bba4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020bba4_at(c: &mut Cpu, pc: u32) { f_8020bba4_b(c, pc) }
#[inline(always)]
fn f_8020bba4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020bba4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020bba4, pc); }
        match pc {
            0x8020bba4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020bbbc; f_801ef634(c);
                pc = 0x8020bbbc;
            }
            0x8020bbbc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020bbcc; continue; }
                pc = 0x8020bbc4;
            }
            0x8020bbc4 => {
                c.r[3] = 0x1;
                pc = 0x8020bbe8; continue;
                pc = 0x8020bbcc;
            }
            0x8020bbcc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020bbd4; f_802119d4(c);
                pc = 0x8020bbd4;
            }
            0x8020bbd4 => {
                c.r[3] = 0x5;
                c.lr = 0x8020bbdc; f_801f000c(c);
                pc = 0x8020bbdc;
            }
            0x8020bbdc => {
                c.r[3] = 0x0;
                c.lr = 0x8020bbe4; f_801f4ac0(c);
                pc = 0x8020bbe4;
            }
            0x8020bbe4 => {
                c.r[3] = 0x1;
                pc = 0x8020bbe8;
            }
            0x8020bbe8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020bba4: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowAllFightOutPokemonDoFightAction (0x8020bbfc, 152 bytes)
#[inline(never)]
pub fn f_8020bbfc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020bbfc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020bbfc); f_8020bbfc_b(c, 0x8020bbfc); crate::prof::exit(c, 0x8020bbfc); return; }
    }
    f_8020bbfc_b(c, 0x8020bbfc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020bbfc_at(c: &mut Cpu, pc: u32) { f_8020bbfc_b(c, pc) }
#[inline(always)]
fn f_8020bbfc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020bbfc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020bbfc, pc); }
        match pc {
            0x8020bbfc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020bc18; f_801f4718(c);
                pc = 0x8020bc18;
            }
            0x8020bc18 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x8020bc24; f_801f3b24(c);
                pc = 0x8020bc24;
            }
            0x8020bc24 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8020bc30; f_8020bc94(c);
                pc = 0x8020bc30;
            }
            0x8020bc30 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8020bc40; continue; }
                pc = 0x8020bc3c;
            }
            0x8020bc3c => {
                pc = 0x8020bc80; continue;
                pc = 0x8020bc40;
            }
            0x8020bc40 => {
                c.lr = 0x8020bc44; f_801ef634(c);
                pc = 0x8020bc44;
            }
            0x8020bc44 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020bc54; continue; }
                pc = 0x8020bc4c;
            }
            0x8020bc4c => {
                c.r[3] = 0x1;
                pc = 0x8020bc80; continue;
                pc = 0x8020bc54;
            }
            0x8020bc54 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020bc5c; f_80211a00(c);
                pc = 0x8020bc5c;
            }
            0x8020bc5c => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x8020bc68; f_8020bc94(c);
                pc = 0x8020bc68;
            }
            0x8020bc68 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[4] = 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8020bc7c; continue; }
                pc = 0x8020bc78;
            }
            0x8020bc78 => {
                c.r[4] = c.r[3] | c.r[3];
                pc = 0x8020bc7c;
            }
            0x8020bc7c => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x8020bc80;
            }
            0x8020bc80 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020bbfc: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowAllFightOutPokemonDoFightActionOneLoop__FP11FIGHT_FLOORUc (0x8020bc94, 420 bytes)
#[inline(never)]
pub fn f_8020bc94(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020bc94) { return; }
        if c.prof { crate::prof::enter(c, 0x8020bc94); f_8020bc94_b(c, 0x8020bc94); crate::prof::exit(c, 0x8020bc94); return; }
    }
    f_8020bc94_b(c, 0x8020bc94)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020bc94_at(c: &mut Cpu, pc: u32) { f_8020bc94_b(c, pc) }
#[inline(always)]
fn f_8020bc94_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020bc94;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020bc94, pc); }
        match pc {
            0x8020bc94 => {
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
            0x8020bcb4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.r[5] = 0x59;
                c.lr = 0x8020bcc8; f_801f54a4(c);
                pc = 0x8020bcc8;
            }
            0x8020bcc8 => {
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                if c.crbit(2) { pc = 0x8020be10; continue; }
                pc = 0x8020bcd0;
            }
            0x8020bcd0 => {
                c.lr = 0x8020bcd4; f_802062fc(c);
                pc = 0x8020bcd4;
            }
            0x8020bcd4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020bcf8; continue; }
                pc = 0x8020bcdc;
            }
            0x8020bcdc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x8020bcf4; f_801254b4(c);
                pc = 0x8020bcf4;
            }
            0x8020bcf4 => {
                pc = 0x8020be10; continue;
                pc = 0x8020bcf8;
            }
            0x8020bcf8 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0xfe;
                c.r[6] = 0x0;
                c.lr = 0x8020bd0c; f_8012640c(c);
                pc = 0x8020bd0c;
            }
            0x8020bd0c => {
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                if !c.crbit(2) { pc = 0x8020bd30; continue; }
                pc = 0x8020bd14;
            }
            0x8020bd14 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x8020bd2c; f_801254b4(c);
                pc = 0x8020bd2c;
            }
            0x8020bd2c => {
                pc = 0x8020be10; continue;
                pc = 0x8020bd30;
            }
            0x8020bd30 => {
                c.lr = 0x8020bd34; f_801f1170(c);
                pc = 0x8020bd34;
            }
            0x8020bd34 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020bd58; continue; }
                pc = 0x8020bd3c;
            }
            0x8020bd3c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x8020bd54; f_801254b4(c);
                pc = 0x8020bd54;
            }
            0x8020bd54 => {
                pc = 0x8020be10; continue;
                pc = 0x8020bd58;
            }
            0x8020bd58 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020bd78; continue; }
                pc = 0x8020bd60;
            }
            0x8020bd60 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020bd68; f_801f0898(c);
                pc = 0x8020bd68;
            }
            0x8020bd68 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x8);
                if !c.crbit(2) { pc = 0x8020be10; continue; }
                pc = 0x8020bd74;
            }
            0x8020bd74 => {
                pc = 0x8020bd8c; continue;
                pc = 0x8020bd78;
            }
            0x8020bd78 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020bd80; f_801f0898(c);
                pc = 0x8020bd80;
            }
            0x8020bd80 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x8);
                if c.crbit(2) { pc = 0x8020be10; continue; }
                pc = 0x8020bd8c;
            }
            0x8020bd8c => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.r[6] = 0x0;
                c.lr = 0x8020bda0; f_8012640c(c);
                pc = 0x8020bda0;
            }
            0x8020bda0 => {
                c.cmps(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x8020be10; continue; }
                pc = 0x8020bda8;
            }
            0x8020bda8 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x0;
                c.r[5] = 0x112;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x8020bdc0; f_801254b4(c);
                pc = 0x8020bdc0;
            }
            0x8020bdc0 => {
                c.r[0] = 0x6;
                c.r[5] = c.r[1].wrapping_add(0x4);
                c.r[4] = c.r[28].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x8020bdd0;
            }
            0x8020bdd0 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8020bdd0; continue; }
                pc = 0x8020bde4;
            }
            0x8020bde4 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8020bdec; f_801f0f04(c);
                pc = 0x8020bdec;
            }
            0x8020bdec => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020be10; continue; }
                pc = 0x8020bdf4;
            }
            0x8020bdf4 => {
                c.r[3] = 0x0;
                c.lr = 0x8020bdfc; f_801f4ac0(c);
                pc = 0x8020bdfc;
            }
            0x8020bdfc => {
                c.lr = 0x8020be00; f_801ef634(c);
                pc = 0x8020be00;
            }
            0x8020be00 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020be10; continue; }
                pc = 0x8020be08;
            }
            0x8020be08 => {
                c.r[3] = 0x1;
                pc = 0x8020be24; continue;
                pc = 0x8020be10;
            }
            0x8020be10 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x8020be14;
            }
            0x8020be14 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x8);
                if c.crbit(0) { pc = 0x8020bcb4; continue; }
                pc = 0x8020be20;
            }
            0x8020be20 => {
                c.r[3] = 0x1;
                pc = 0x8020be24;
            }
            0x8020be24 => {
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { return; }
                return;
            }
            _ => panic!("f_8020bc94: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowAllFightTrainerSelectFightAction (0x8020be38, 264 bytes)
#[inline(never)]
pub fn f_8020be38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020be38) { return; }
        if c.prof { crate::prof::enter(c, 0x8020be38); f_8020be38_b(c, 0x8020be38); crate::prof::exit(c, 0x8020be38); return; }
    }
    f_8020be38_b(c, 0x8020be38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020be38_at(c: &mut Cpu, pc: u32) { f_8020be38_b(c, pc) }
#[inline(always)]
fn f_8020be38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020be38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020be38, pc); }
        match pc {
            0x8020be38 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020be4c; f_80008174(c);
                pc = 0x8020be4c;
            }
            0x8020be4c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[30] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020bf10; continue; }
                pc = 0x8020be5c;
            }
            0x8020be5c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8020be70; f_801f54a4(c);
                pc = 0x8020be70;
            }
            0x8020be70 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                c.lr = 0x8020be88; f_801f54a4(c);
                pc = 0x8020be88;
            }
            0x8020be88 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[25] = 0x0;
                pc = 0x8020bf00; continue;
                pc = 0x8020be94;
            }
            0x8020be94 => {
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0x0;
                c.lr = 0x8020bea0; f_801f47b4(c);
                pc = 0x8020bea0;
            }
            0x8020bea0 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x8020befc; continue; }
                pc = 0x8020bea8;
            }
            0x8020bea8 => {
                c.r[31] = c.r[25].rotate_left(0) & 0xffff;
                c.r[26] = 0x0;
                pc = 0x8020bef0; continue;
                pc = 0x8020beb4;
            }
            0x8020beb4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x8020bec0; f_801f7258(c);
                pc = 0x8020bec0;
            }
            0x8020bec0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020beec; continue; }
                pc = 0x8020bec8;
            }
            0x8020bec8 => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8020bed4; f_8026316c(c);
                pc = 0x8020bed4;
            }
            0x8020bed4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020beec; continue; }
                pc = 0x8020bedc;
            }
            0x8020bedc => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8020beb4; continue; }
                pc = 0x8020bee4;
            }
            0x8020bee4 => {
                c.r[25] = c.r[25].wrapping_add(0xffffffff);
                pc = 0x8020be94; continue;
                pc = 0x8020beec;
            }
            0x8020beec => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8020bef0;
            }
            0x8020bef0 => {
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[28]);
                if c.crbit(0) { pc = 0x8020beb4; continue; }
                pc = 0x8020befc;
            }
            0x8020befc => {
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x8020bf00;
            }
            0x8020bf00 => {
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8020be94; continue; }
                pc = 0x8020bf0c;
            }
            0x8020bf0c => {
                pc = 0x8020bf28; continue;
                pc = 0x8020bf10;
            }
            0x8020bf10 => {
                c.r[4] = 0x80210000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffffbf40);
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x8020bf28; f_801f2b5c(c);
                pc = 0x8020bf28;
            }
            0x8020bf28 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8020be38: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowFightTrainerSelectFightAction__FPvUsPv (0x8020bf40, 96 bytes)
#[inline(never)]
pub fn f_8020bf40(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020bf40) { return; }
        if c.prof { crate::prof::enter(c, 0x8020bf40); f_8020bf40_b(c, 0x8020bf40); crate::prof::exit(c, 0x8020bf40); return; }
    }
    f_8020bf40_b(c, 0x8020bf40)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020bf40_at(c: &mut Cpu, pc: u32) { f_8020bf40_b(c, pc) }
#[inline(always)]
fn f_8020bf40_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020bf40;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020bf40, pc); }
        match pc {
            0x8020bf40 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x8020bf5c; f_801ef634(c);
                pc = 0x8020bf5c;
            }
            0x8020bf5c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020bf6c; continue; }
                pc = 0x8020bf64;
            }
            0x8020bf64 => {
                c.r[3] = 0x1;
                pc = 0x8020bf8c; continue;
                pc = 0x8020bf6c;
            }
            0x8020bf6c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8020bf78; f_801f923c(c);
                pc = 0x8020bf78;
            }
            0x8020bf78 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020bf88; continue; }
                pc = 0x8020bf80;
            }
            0x8020bf80 => {
                c.r[3] = 0x0;
                c.lr = 0x8020bf88; f_801f150c(c);
                pc = 0x8020bf88;
            }
            0x8020bf88 => {
                c.r[3] = 0x1;
                pc = 0x8020bf8c;
            }
            0x8020bf8c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020bf40: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaisiPost (0x8020bfa0, 288 bytes)
#[inline(never)]
pub fn f_8020bfa0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020bfa0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020bfa0); f_8020bfa0_b(c, 0x8020bfa0); crate::prof::exit(c, 0x8020bfa0); return; }
    }
    f_8020bfa0_b(c, 0x8020bfa0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020bfa0_at(c: &mut Cpu, pc: u32) { f_8020bfa0_b(c, pc) }
#[inline(always)]
fn f_8020bfa0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020bfa0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020bfa0, pc); }
        match pc {
            0x8020bfa0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020bfbc; f_801f4718(c);
                pc = 0x8020bfbc;
            }
            0x8020bfbc => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8020bfc8; f_801f3b24(c);
                pc = 0x8020bfc8;
            }
            0x8020bfc8 => {
                c.lr = 0x8020bfcc; f_80211830(c);
                pc = 0x8020bfcc;
            }
            0x8020bfcc => {
                c.r[0] = 0x0;
                c.r[3] = 0x80210000;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xffffc108);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x8020bfec; f_801f37b0(c);
                pc = 0x8020bfec;
            }
            0x8020bfec => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020bff4; f_8020d920(c);
                pc = 0x8020bff4;
            }
            0x8020bff4 => {
                c.r[4] = 0x80370000;
                c.r[5] = 0x80380000;
                c.r[7] = c.r[4].wrapping_add(0x5cc8);
                c.r[6] = 0x0;
                c.r[8] = c.r[5].wrapping_add(0xffff8aa0);
                c.r[4] = 0x0;
                c.r[5] = 0x6;
                c.lr = 0x8020c014; f_80211948(c);
                pc = 0x8020c014;
            }
            0x8020c014 => {
                c.r[4] = 0x80210000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffffc0e4);
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x8020c02c; f_801f37b0(c);
                pc = 0x8020c02c;
            }
            0x8020c02c => {
                c.r[3] = 0x1;
                c.lr = 0x8020c034; f_8022e314(c);
                pc = 0x8020c034;
            }
            0x8020c034 => {
                c.lr = 0x8020c038; f_8022e1c4(c);
                pc = 0x8020c038;
            }
            0x8020c038 => {
                c.r[4] = 0x80210000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0xffffc0c0);
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x8020c050; f_801f37b0(c);
                pc = 0x8020c050;
            }
            0x8020c050 => {
                c.r[0] = 0x1;
                c.r[3] = 0x80210000;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xffffc108);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x8020c070; f_801f37b0(c);
                pc = 0x8020c070;
            }
            0x8020c070 => {
                c.r[3] = 0x0;
                c.lr = 0x8020c078; f_801f3178(c);
                pc = 0x8020c078;
            }
            0x8020c078 => {
                c.r[3] = 0x0;
                c.lr = 0x8020c080; f_801f3074(c);
                pc = 0x8020c080;
            }
            0x8020c080 => {
                c.r[3] = 0x0;
                c.lr = 0x8020c088; f_801f2f3c(c);
                pc = 0x8020c088;
            }
            0x8020c088 => {
                c.lr = 0x8020c08c; f_800e0c54(c);
                pc = 0x8020c08c;
            }
            0x8020c08c => {
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x5b;
                c.r[6] = 0x0;
                c.lr = 0x8020c0a4; f_801f4c14(c);
                pc = 0x8020c0a4;
            }
            0x8020c0a4 => {
                c.lr = 0x8020c0a8; f_801da7ac(c);
                pc = 0x8020c0a8;
            }
            0x8020c0a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020bfa0: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowKaisiPostSubFightOutPokemonSoubiItemCheckAppear__FPvUsPv (0x8020c0c0, 36 bytes)
#[inline(never)]
pub fn f_8020c0c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020c0c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020c0c0); f_8020c0c0_b(c, 0x8020c0c0); crate::prof::exit(c, 0x8020c0c0); return; }
    }
    f_8020c0c0_b(c, 0x8020c0c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020c0c0_at(c: &mut Cpu, pc: u32) { f_8020c0c0_b(c, pc) }
#[inline(always)]
fn f_8020c0c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020c0c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020c0c0, pc); }
        match pc {
            0x8020c0c0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8020c0d0; f_8022d084(c);
                pc = 0x8020c0d0;
            }
            0x8020c0d0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020c0c0: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowKaisiPostSubFightOutPokemonTokuseiCheckAppear__FPvUsPv (0x8020c0e4, 36 bytes)
#[inline(never)]
pub fn f_8020c0e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020c0e4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020c0e4); f_8020c0e4_b(c, 0x8020c0e4); crate::prof::exit(c, 0x8020c0e4); return; }
    }
    f_8020c0e4_b(c, 0x8020c0e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020c0e4_at(c: &mut Cpu, pc: u32) { f_8020c0e4_b(c, pc) }
#[inline(always)]
fn f_8020c0e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020c0e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020c0e4, pc); }
        match pc {
            0x8020c0e4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8020c0f4; f_8022e410(c);
                pc = 0x8020c0f4;
            }
            0x8020c0f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020c0e4: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowKaisiPostSubFightOutPokemonDarkCheckAppear__FPvUsPv (0x8020c108, 84 bytes)
#[inline(never)]
pub fn f_8020c108(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020c108) { return; }
        if c.prof { crate::prof::enter(c, 0x8020c108); f_8020c108_b(c, 0x8020c108); crate::prof::exit(c, 0x8020c108); return; }
    }
    f_8020c108_b(c, 0x8020c108)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020c108_at(c: &mut Cpu, pc: u32) { f_8020c108_b(c, pc) }
#[inline(always)]
fn f_8020c108_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020c108;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020c108, pc); }
        match pc {
            0x8020c108 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[5], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x8020c130; continue; }
                pc = 0x8020c124;
            }
            0x8020c124 => {
                c.r[4] = c.lbz(c.r[5]);
                c.lr = 0x8020c12c; f_8022e6f0(c);
                pc = 0x8020c12c;
            }
            0x8020c12c => {
                pc = 0x8020c144; continue;
                pc = 0x8020c130;
            }
            0x8020c130 => {
                c.r[4] = 0x0;
                c.lr = 0x8020c138; f_8022e6f0(c);
                pc = 0x8020c138;
            }
            0x8020c138 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x1;
                c.lr = 0x8020c144; f_8022e6f0(c);
                pc = 0x8020c144;
            }
            0x8020c144 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020c108: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaisiPre (0x8020c15c, 1764 bytes)
#[inline(never)]
pub fn f_8020c15c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020c15c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020c15c); f_8020c15c_b(c, 0x8020c15c); crate::prof::exit(c, 0x8020c15c); return; }
    }
    f_8020c15c_b(c, 0x8020c15c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020c15c_at(c: &mut Cpu, pc: u32) { f_8020c15c_b(c, pc) }
#[inline(always)]
fn f_8020c15c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020c15c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020c15c, pc); }
        match pc {
            0x8020c15c => {
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
                c.lr = 0x8020c188; f_801f54a4(c);
                pc = 0x8020c188;
            }
            0x8020c188 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0xb;
                c.r[4] = 0x0;
                c.lr = 0x8020c198; f_801f025c(c);
                pc = 0x8020c198;
            }
            0x8020c198 => {
                c.r[4] = 0x0;
                c.r[25] = c.r[3] | c.r[3];
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020c1ac; f_801fb1c0(c);
                pc = 0x8020c1ac;
            }
            0x8020c1ac => {
                c.r[4] = c.r[25] | c.r[25];
                c.r[25] = c.r[3] | c.r[3];
                c.r[3] = 0x9;
                c.lr = 0x8020c1bc; f_801f025c(c);
                pc = 0x8020c1bc;
            }
            0x8020c1bc => {
                c.r[4] = 0x0;
                c.r[29] = c.r[3] | c.r[3];
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020c1d0; f_801fb1c0(c);
                pc = 0x8020c1d0;
            }
            0x8020c1d0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[26] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x8020c1ec; f_801fb1c0(c);
                pc = 0x8020c1ec;
            }
            0x8020c1ec => {
                c.r[24] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x10;
                c.r[6] = 0x0;
                c.lr = 0x8020c204; f_801f54a4(c);
                pc = 0x8020c204;
            }
            0x8020c204 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0xd;
                c.r[6] = 0x0;
                c.lr = 0x8020c21c; f_801f54a4(c);
                pc = 0x8020c21c;
            }
            0x8020c21c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8020c224; f_8020e0f8(c);
                pc = 0x8020c224;
            }
            0x8020c224 => {
                c.lr = 0x8020c228; f_8020dfc0(c);
                pc = 0x8020c228;
            }
            0x8020c228 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[24] | c.r[24];
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x8020c240; f_801fb1c0(c);
                pc = 0x8020c240;
            }
            0x8020c240 => {
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                if !c.crbit(2) { pc = 0x8020c24c; continue; }
                pc = 0x8020c248;
            }
            0x8020c248 => {
                c.r[28] = 0x5f;
                pc = 0x8020c24c;
            }
            0x8020c24c => {
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x0;
                c.lr = 0x8020c260; f_801fb1c0(c);
                pc = 0x8020c260;
            }
            0x8020c260 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[27] = c.r[0] | c.r[0];
                c.lr = 0x8020c270; f_801f1888(c);
                pc = 0x8020c270;
            }
            0x8020c270 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020c69c; continue; }
                pc = 0x8020c278;
            }
            0x8020c278 => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8020c430; continue; }
                pc = 0x8020c280;
            }
            0x8020c280 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8020c298; continue; }
                pc = 0x8020c288;
            }
            0x8020c288 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c298; f_801da9e8(c);
                pc = 0x8020c298;
            }
            0x8020c298 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c2a8; f_801da9e8(c);
                pc = 0x8020c2a8;
            }
            0x8020c2a8 => {
                c.r[0] = c.addc(c.r[1], 0x8, 0);
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020c2b8; continue; }
                pc = 0x8020c2b0;
            }
            0x8020c2b0 => {
                c.r[0] = 0x0;
                pc = 0x8020c2ec; continue;
                pc = 0x8020c2b8;
            }
            0x8020c2b8 => {
                c.r[3] = c.r[1].wrapping_add(0x44);
                c.r[4] = 0x1;
                c.lr = 0x8020c2c4; f_80103ba8(c);
                pc = 0x8020c2c4;
            }
            0x8020c2c4 => {
                c.r[3] = 0x0;
                c.lr = 0x8020c2cc; f_801c40f0(c);
                pc = 0x8020c2cc;
            }
            0x8020c2cc => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020c2e8; continue; }
                pc = 0x8020c2d4;
            }
            0x8020c2d4 => {
                c.r[0] = c.lhz(c.r[1].wrapping_add(0x44));
                c.r[0] = c.r[0].rotate_left(0) & 0x20;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c2e8; continue; }
                pc = 0x8020c2e0;
            }
            0x8020c2e0 => {
                c.r[0] = 0x1;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8020c2e8;
            }
            0x8020c2e8 => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x8));
                pc = 0x8020c2ec;
            }
            0x8020c2ec => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8020c430; continue; }
                pc = 0x8020c2f8;
            }
            0x8020c2f8 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c308; f_801da94c(c);
                pc = 0x8020c308;
            }
            0x8020c308 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c318; continue; }
                pc = 0x8020c310;
            }
            0x8020c310 => {
                c.lr = 0x8020c314; crate::stubs::f_800f0308(c);
                pc = 0x8020c314;
            }
            0x8020c314 => {
                pc = 0x8020c2a8; continue;
                pc = 0x8020c318;
            }
            0x8020c318 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c32c; f_801f54a4(c);
                pc = 0x8020c32c;
            }
            0x8020c32c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c3b8; continue; }
                pc = 0x8020c338;
            }
            0x8020c338 => {
                c.r[4] = c.lhz(c.r[13].wrapping_add(0xffffadd8));
                c.r[3] = c.r[26] | c.r[26];
                c.r[5] = 0x4;
                c.lr = 0x8020c348; f_801da9e8(c);
                pc = 0x8020c348;
            }
            0x8020c348 => {
                c.r[0] = c.addc(c.r[1], 0x8, 0);
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020c358; continue; }
                pc = 0x8020c350;
            }
            0x8020c350 => {
                c.r[0] = 0x0;
                pc = 0x8020c38c; continue;
                pc = 0x8020c358;
            }
            0x8020c358 => {
                c.r[3] = c.r[1].wrapping_add(0x28);
                c.r[4] = 0x1;
                c.lr = 0x8020c364; f_80103ba8(c);
                pc = 0x8020c364;
            }
            0x8020c364 => {
                c.r[3] = 0x0;
                c.lr = 0x8020c36c; f_801c40f0(c);
                pc = 0x8020c36c;
            }
            0x8020c36c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020c388; continue; }
                pc = 0x8020c374;
            }
            0x8020c374 => {
                c.r[0] = c.lhz(c.r[1].wrapping_add(0x28));
                c.r[0] = c.r[0].rotate_left(0) & 0x20;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c388; continue; }
                pc = 0x8020c380;
            }
            0x8020c380 => {
                c.r[0] = 0x1;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8020c388;
            }
            0x8020c388 => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x8));
                pc = 0x8020c38c;
            }
            0x8020c38c => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8020c430; continue; }
                pc = 0x8020c398;
            }
            0x8020c398 => {
                c.r[4] = c.lhz(c.r[13].wrapping_add(0xffffadd8));
                c.r[3] = c.r[26] | c.r[26];
                c.r[5] = 0x4;
                c.lr = 0x8020c3a8; f_801da94c(c);
                pc = 0x8020c3a8;
            }
            0x8020c3a8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c3b8; continue; }
                pc = 0x8020c3b0;
            }
            0x8020c3b0 => {
                c.lr = 0x8020c3b4; crate::stubs::f_800f0308(c);
                pc = 0x8020c3b4;
            }
            0x8020c3b4 => {
                pc = 0x8020c348; continue;
                pc = 0x8020c3b8;
            }
            0x8020c3b8 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8020c430; continue; }
                pc = 0x8020c3c0;
            }
            0x8020c3c0 => {
                c.r[0] = c.addc(c.r[1], 0x8, 0);
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020c3d0; continue; }
                pc = 0x8020c3c8;
            }
            0x8020c3c8 => {
                c.r[0] = 0x0;
                pc = 0x8020c404; continue;
                pc = 0x8020c3d0;
            }
            0x8020c3d0 => {
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = 0x1;
                c.lr = 0x8020c3dc; f_80103ba8(c);
                pc = 0x8020c3dc;
            }
            0x8020c3dc => {
                c.r[3] = 0x0;
                c.lr = 0x8020c3e4; f_801c40f0(c);
                pc = 0x8020c3e4;
            }
            0x8020c3e4 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020c400; continue; }
                pc = 0x8020c3ec;
            }
            0x8020c3ec => {
                c.r[0] = c.lhz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x20;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c400; continue; }
                pc = 0x8020c3f8;
            }
            0x8020c3f8 => {
                c.r[0] = 0x1;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8020c400;
            }
            0x8020c400 => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x8));
                pc = 0x8020c404;
            }
            0x8020c404 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8020c430; continue; }
                pc = 0x8020c410;
            }
            0x8020c410 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c420; f_801da94c(c);
                pc = 0x8020c420;
            }
            0x8020c420 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c430; continue; }
                pc = 0x8020c428;
            }
            0x8020c428 => {
                c.lr = 0x8020c42c; crate::stubs::f_800f0308(c);
                pc = 0x8020c42c;
            }
            0x8020c42c => {
                pc = 0x8020c3c0; continue;
                pc = 0x8020c430;
            }
            0x8020c430 => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c4a8; continue; }
                pc = 0x8020c43c;
            }
            0x8020c43c => {
                { let ea = c.r[2].wrapping_add(0xffffae80); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x3;
                c.lr = 0x8020c448; f_801c41c8(c);
                pc = 0x8020c448;
            }
            0x8020c448 => {
                c.r[3] = 0x1;
                c.lr = 0x8020c450; f_801c40f0(c);
                pc = 0x8020c450;
            }
            0x8020c450 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8020c468; continue; }
                pc = 0x8020c458;
            }
            0x8020c458 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c468; f_801da9b4(c);
                pc = 0x8020c468;
            }
            0x8020c468 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c478; f_801da9b4(c);
                pc = 0x8020c478;
            }
            0x8020c478 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c48c; f_801f54a4(c);
                pc = 0x8020c48c;
            }
            0x8020c48c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c4a8; continue; }
                pc = 0x8020c498;
            }
            0x8020c498 => {
                c.r[4] = c.lhz(c.r[13].wrapping_add(0xffffadd8));
                c.r[3] = c.r[26] | c.r[26];
                c.r[5] = 0x4;
                c.lr = 0x8020c4a8; f_801da9b4(c);
                pc = 0x8020c4a8;
            }
            0x8020c4a8 => {
                c.cmpu(0, c.r[27], 0x0);
                if c.crbit(2) { pc = 0x8020c50c; continue; }
                pc = 0x8020c4b0;
            }
            0x8020c4b0 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x5f;
                c.r[5] = 0x4;
                c.lr = 0x8020c4c0; f_801da9e8(c);
                pc = 0x8020c4c0;
            }
            0x8020c4c0 => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c4e0; continue; }
                pc = 0x8020c4cc;
            }
            0x8020c4cc => {
                { let ea = c.r[2].wrapping_add(0xffffae80); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8020c4d8; f_801c41c8(c);
                pc = 0x8020c4d8;
            }
            0x8020c4d8 => {
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8020c4e0;
            }
            0x8020c4e0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020c4e8; f_80262490(c);
                pc = 0x8020c4e8;
            }
            0x8020c4e8 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x5f;
                c.r[5] = 0x4;
                c.lr = 0x8020c4f8; f_801da94c(c);
                pc = 0x8020c4f8;
            }
            0x8020c4f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c508; continue; }
                pc = 0x8020c500;
            }
            0x8020c500 => {
                c.lr = 0x8020c504; crate::stubs::f_800f0308(c);
                pc = 0x8020c504;
            }
            0x8020c504 => {
                pc = 0x8020c4e8; continue;
                pc = 0x8020c508;
            }
            0x8020c508 => {
                c.lr = 0x8020c50c; f_8026246c(c);
                pc = 0x8020c50c;
            }
            0x8020c50c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x8020c520; f_801f54a4(c);
                pc = 0x8020c520;
            }
            0x8020c520 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020c534; continue; }
                pc = 0x8020c528;
            }
            0x8020c528 => {
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x8020c534; f_80165a20(c);
                pc = 0x8020c534;
            }
            0x8020c534 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c548; f_801f54a4(c);
                pc = 0x8020c548;
            }
            0x8020c548 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c5d8; continue; }
                pc = 0x8020c554;
            }
            0x8020c554 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020c55c; f_801f8000(c);
                pc = 0x8020c55c;
            }
            0x8020c55c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x22;
                c.lr = 0x8020c568; f_80132a38(c);
                pc = 0x8020c568;
            }
            0x8020c568 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020c570; f_801f8100(c);
                pc = 0x8020c570;
            }
            0x8020c570 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x23;
                c.lr = 0x8020c57c; f_80132a38(c);
                pc = 0x8020c57c;
            }
            0x8020c57c => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c58c; f_801da9e8(c);
                pc = 0x8020c58c;
            }
            0x8020c58c => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c5ac; continue; }
                pc = 0x8020c598;
            }
            0x8020c598 => {
                { let ea = c.r[2].wrapping_add(0xffffae80); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8020c5a4; f_801c41c8(c);
                pc = 0x8020c5a4;
            }
            0x8020c5a4 => {
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8020c5ac;
            }
            0x8020c5ac => {
                c.r[3] = 0x766d;
                c.lr = 0x8020c5b4; f_80262490(c);
                pc = 0x8020c5b4;
            }
            0x8020c5b4 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c5c4; f_801da94c(c);
                pc = 0x8020c5c4;
            }
            0x8020c5c4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c5d4; continue; }
                pc = 0x8020c5cc;
            }
            0x8020c5cc => {
                c.lr = 0x8020c5d0; crate::stubs::f_800f0308(c);
                pc = 0x8020c5d0;
            }
            0x8020c5d0 => {
                pc = 0x8020c5b4; continue;
                pc = 0x8020c5d4;
            }
            0x8020c5d4 => {
                c.lr = 0x8020c5d8; f_8026246c(c);
                pc = 0x8020c5d8;
            }
            0x8020c5d8 => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c5f0; continue; }
                pc = 0x8020c5e4;
            }
            0x8020c5e4 => {
                { let ea = c.r[2].wrapping_add(0xffffae80); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8020c5f0; f_801c41c8(c);
                pc = 0x8020c5f0;
            }
            0x8020c5f0 => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8020c650; continue; }
                pc = 0x8020c5f8;
            }
            0x8020c5f8 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8020c610; continue; }
                pc = 0x8020c600;
            }
            0x8020c600 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c610; f_801da8c4(c);
                pc = 0x8020c610;
            }
            0x8020c610 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c620; f_801da8c4(c);
                pc = 0x8020c620;
            }
            0x8020c620 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c634; f_801f54a4(c);
                pc = 0x8020c634;
            }
            0x8020c634 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c650; continue; }
                pc = 0x8020c640;
            }
            0x8020c640 => {
                c.r[4] = c.lhz(c.r[13].wrapping_add(0xffffadd8));
                c.r[3] = c.r[26] | c.r[26];
                c.r[5] = 0x4;
                c.lr = 0x8020c650; f_801da8c4(c);
                pc = 0x8020c650;
            }
            0x8020c650 => {
                c.cmpu(0, c.r[27], 0x0);
                if c.crbit(2) { pc = 0x8020c668; continue; }
                pc = 0x8020c658;
            }
            0x8020c658 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x5f;
                c.r[5] = 0x4;
                c.lr = 0x8020c668; f_801da8c4(c);
                pc = 0x8020c668;
            }
            0x8020c668 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c67c; f_801f54a4(c);
                pc = 0x8020c67c;
            }
            0x8020c67c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c828; continue; }
                pc = 0x8020c688;
            }
            0x8020c688 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.lr = 0x8020c698; f_801da8c4(c);
                pc = 0x8020c698;
            }
            0x8020c698 => {
                pc = 0x8020c828; continue;
                pc = 0x8020c69c;
            }
            0x8020c69c => {
                c.r[3] = 0x0;
                c.lr = 0x8020c6a4; f_801ef7c4(c);
                pc = 0x8020c6a4;
            }
            0x8020c6a4 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x1;
                c.lr = 0x8020c6b0; f_801da4e8(c);
                pc = 0x8020c6b0;
            }
            0x8020c6b0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x54;
                c.r[5] = 0x4;
                c.lr = 0x8020c6c0; f_801da9e8(c);
                pc = 0x8020c6c0;
            }
            0x8020c6c0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x54;
                c.r[5] = 0x4;
                c.lr = 0x8020c6d0; f_801da94c(c);
                pc = 0x8020c6d0;
            }
            0x8020c6d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c6e0; continue; }
                pc = 0x8020c6d8;
            }
            0x8020c6d8 => {
                c.lr = 0x8020c6dc; crate::stubs::f_800f0308(c);
                pc = 0x8020c6dc;
            }
            0x8020c6dc => {
                pc = 0x8020c6c0; continue;
                pc = 0x8020c6e0;
            }
            0x8020c6e0 => {
                c.r[3] = 0x0;
                c.lr = 0x8020c6e8; f_801ef7c4(c);
                pc = 0x8020c6e8;
            }
            0x8020c6e8 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x1;
                c.lr = 0x8020c6f4; f_801da4e8(c);
                pc = 0x8020c6f4;
            }
            0x8020c6f4 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x55;
                c.r[5] = 0x4;
                c.lr = 0x8020c704; f_801da9e8(c);
                pc = 0x8020c704;
            }
            0x8020c704 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c718; f_801f54a4(c);
                pc = 0x8020c718;
            }
            0x8020c718 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c754; continue; }
                pc = 0x8020c724;
            }
            0x8020c724 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020c72c; f_801f8000(c);
                pc = 0x8020c72c;
            }
            0x8020c72c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x22;
                c.lr = 0x8020c738; f_80132a38(c);
                pc = 0x8020c738;
            }
            0x8020c738 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020c740; f_801f8100(c);
                pc = 0x8020c740;
            }
            0x8020c740 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x23;
                c.lr = 0x8020c74c; f_80132a38(c);
                pc = 0x8020c74c;
            }
            0x8020c74c => {
                c.r[3] = 0x766d;
                c.lr = 0x8020c754; f_80262490(c);
                pc = 0x8020c754;
            }
            0x8020c754 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x55;
                c.r[5] = 0x4;
                c.lr = 0x8020c764; f_801da94c(c);
                pc = 0x8020c764;
            }
            0x8020c764 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c774; continue; }
                pc = 0x8020c76c;
            }
            0x8020c76c => {
                c.lr = 0x8020c770; crate::stubs::f_800f0308(c);
                pc = 0x8020c770;
            }
            0x8020c770 => {
                pc = 0x8020c754; continue;
                pc = 0x8020c774;
            }
            0x8020c774 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c788; f_801f54a4(c);
                pc = 0x8020c788;
            }
            0x8020c788 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c798; continue; }
                pc = 0x8020c794;
            }
            0x8020c794 => {
                c.lr = 0x8020c798; f_8026246c(c);
                pc = 0x8020c798;
            }
            0x8020c798 => {
                c.r[3] = 0x1;
                c.lr = 0x8020c7a0; f_801ef7c4(c);
                pc = 0x8020c7a0;
            }
            0x8020c7a0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x56;
                c.r[5] = 0x4;
                c.lr = 0x8020c7b0; f_801da9e8(c);
                pc = 0x8020c7b0;
            }
            0x8020c7b0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x56;
                c.r[5] = 0x4;
                c.lr = 0x8020c7c0; f_801da94c(c);
                pc = 0x8020c7c0;
            }
            0x8020c7c0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020c7d0; continue; }
                pc = 0x8020c7c8;
            }
            0x8020c7c8 => {
                c.lr = 0x8020c7cc; crate::stubs::f_800f0308(c);
                pc = 0x8020c7cc;
            }
            0x8020c7cc => {
                pc = 0x8020c7b0; continue;
                pc = 0x8020c7d0;
            }
            0x8020c7d0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x54;
                c.r[5] = 0x4;
                c.lr = 0x8020c7e0; f_801da8c4(c);
                pc = 0x8020c7e0;
            }
            0x8020c7e0 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x55;
                c.r[5] = 0x4;
                c.lr = 0x8020c7f0; f_801da8c4(c);
                pc = 0x8020c7f0;
            }
            0x8020c7f0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = 0x56;
                c.r[5] = 0x4;
                c.lr = 0x8020c800; f_801da8c4(c);
                pc = 0x8020c800;
            }
            0x8020c800 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x11;
                c.r[6] = 0x0;
                c.lr = 0x8020c814; f_801f54a4(c);
                pc = 0x8020c814;
            }
            0x8020c814 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020c828; continue; }
                pc = 0x8020c81c;
            }
            0x8020c81c => {
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x8020c828; f_80165a20(c);
                pc = 0x8020c828;
            }
            0x8020c828 => {
                { let mut ea = c.r[1].wrapping_add(0x60); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x84));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x80);
                if true { return; }
                return;
            }
            _ => panic!("f_8020c15c: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaisiPreSubLoad (0x8020c840, 600 bytes)
#[inline(never)]
pub fn f_8020c840(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020c840) { return; }
        if c.prof { crate::prof::enter(c, 0x8020c840); f_8020c840_b(c, 0x8020c840); crate::prof::exit(c, 0x8020c840); return; }
    }
    f_8020c840_b(c, 0x8020c840)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020c840_at(c: &mut Cpu, pc: u32) { f_8020c840_b(c, pc) }
#[inline(always)]
fn f_8020c840_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020c840;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020c840, pc); }
        match pc {
            0x8020c840 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[5] = 0xe;
                c.r[6] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020c864; f_801f54a4(c);
                pc = 0x8020c864;
            }
            0x8020c864 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0xb;
                c.r[4] = 0x0;
                c.lr = 0x8020c874; f_801f025c(c);
                pc = 0x8020c874;
            }
            0x8020c874 => {
                c.r[4] = 0x0;
                c.r[28] = c.r[3] | c.r[3];
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020c888; f_801fb1c0(c);
                pc = 0x8020c888;
            }
            0x8020c888 => {
                c.r[26] = c.r[3] | c.r[3];
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x9;
                c.lr = 0x8020c898; f_801f025c(c);
                pc = 0x8020c898;
            }
            0x8020c898 => {
                c.r[4] = 0x0;
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020c8ac; f_801fb1c0(c);
                pc = 0x8020c8ac;
            }
            0x8020c8ac => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x43;
                c.r[6] = 0x0;
                c.lr = 0x8020c8c8; f_801fb1c0(c);
                pc = 0x8020c8c8;
            }
            0x8020c8c8 => {
                c.r[25] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x10;
                c.r[6] = 0x0;
                c.lr = 0x8020c8e0; f_801f54a4(c);
                pc = 0x8020c8e0;
            }
            0x8020c8e0 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[4] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[5] = 0xd;
                c.r[6] = 0x0;
                c.lr = 0x8020c8f8; f_801f54a4(c);
                pc = 0x8020c8f8;
            }
            0x8020c8f8 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8020c900; f_8020e0f8(c);
                pc = 0x8020c900;
            }
            0x8020c900 => {
                c.lr = 0x8020c904; f_8020dfc0(c);
                pc = 0x8020c904;
            }
            0x8020c904 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[25] | c.r[25];
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x8020c91c; f_801fb1c0(c);
                pc = 0x8020c91c;
            }
            0x8020c91c => {
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                if !c.crbit(2) { pc = 0x8020c928; continue; }
                pc = 0x8020c924;
            }
            0x8020c924 => {
                c.r[27] = 0x5f;
                pc = 0x8020c928;
            }
            0x8020c928 => {
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = 0x0;
                c.r[5] = 0x8;
                c.r[6] = 0x0;
                c.lr = 0x8020c93c; f_801fb1c0(c);
                pc = 0x8020c93c;
            }
            0x8020c93c => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020c948; f_801f1888(c);
                pc = 0x8020c948;
            }
            0x8020c948 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020ca48; continue; }
                pc = 0x8020c950;
            }
            0x8020c950 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x8020c9f4; continue; }
                pc = 0x8020c958;
            }
            0x8020c958 => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x8020c974; continue; }
                pc = 0x8020c960;
            }
            0x8020c960 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[29].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020c974; f_801ddd28(c);
                pc = 0x8020c974;
            }
            0x8020c974 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020c988; f_801ddd28(c);
                pc = 0x8020c988;
            }
            0x8020c988 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020c99c; f_801f54a4(c);
                pc = 0x8020c99c;
            }
            0x8020c99c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020c9ec; continue; }
                pc = 0x8020c9a8;
            }
            0x8020c9a8 => {
                c.lr = 0x8020c9ac; f_800e0c54(c);
                pc = 0x8020c9ac;
            }
            0x8020c9ac => {
                c.r[7] = c.lbz(c.r[13].wrapping_add(0xffff84f8));
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
                c.r[4] = c.lhz(c.r[4].wrapping_add(c.r[0]));
                c.sth(c.r[13].wrapping_add(0xffffadd8), c.r[4]);
                c.lr = 0x8020c9e8; f_801ddd28(c);
                pc = 0x8020c9e8;
            }
            0x8020c9e8 => {
                pc = 0x8020c9f4; continue;
                pc = 0x8020c9ec;
            }
            0x8020c9ec => {
                c.r[0] = 0x0;
                c.sth(c.r[13].wrapping_add(0xffffadd8), c.r[0]);
                pc = 0x8020c9f4;
            }
            0x8020c9f4 => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8020ca10; continue; }
                pc = 0x8020c9fc;
            }
            0x8020c9fc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x5f;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020ca10; f_801ddd28(c);
                pc = 0x8020ca10;
            }
            0x8020ca10 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x33;
                c.r[6] = 0x0;
                c.lr = 0x8020ca24; f_801f54a4(c);
                pc = 0x8020ca24;
            }
            0x8020ca24 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020ca84; continue; }
                pc = 0x8020ca30;
            }
            0x8020ca30 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27].rotate_left(0) & 0xffff;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020ca44; f_801ddd28(c);
                pc = 0x8020ca44;
            }
            0x8020ca44 => {
                pc = 0x8020ca84; continue;
                pc = 0x8020ca48;
            }
            0x8020ca48 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x54;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020ca5c; f_801ddd28(c);
                pc = 0x8020ca5c;
            }
            0x8020ca5c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x55;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020ca70; f_801ddd28(c);
                pc = 0x8020ca70;
            }
            0x8020ca70 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x56;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020ca84; f_801ddd28(c);
                pc = 0x8020ca84;
            }
            0x8020ca84 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8020c840: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaisiNyuujouPokemon (0x8020ca98, 1352 bytes)
#[inline(never)]
pub fn f_8020ca98(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ca98) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ca98); f_8020ca98_b(c, 0x8020ca98); crate::prof::exit(c, 0x8020ca98); return; }
    }
    f_8020ca98_b(c, 0x8020ca98)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ca98_at(c: &mut Cpu, pc: u32) { f_8020ca98_b(c, pc) }
#[inline(always)]
fn f_8020ca98_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ca98;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ca98, pc); }
        match pc {
            0x8020ca98 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 19..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020caac; f_8020d920(c);
                pc = 0x8020caac;
            }
            0x8020caac => {
                c.lr = 0x8020cab0; f_8020d814(c);
                pc = 0x8020cab0;
            }
            0x8020cab0 => {
                c.r[21] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8020cac8; f_801f54a4(c);
                pc = 0x8020cac8;
            }
            0x8020cac8 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x18;
                c.r[6] = 0x0;
                c.lr = 0x8020cae0; f_801f54a4(c);
                pc = 0x8020cae0;
            }
            0x8020cae0 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                c.lr = 0x8020caf8; f_801f54a4(c);
                pc = 0x8020caf8;
            }
            0x8020caf8 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[21] | c.r[21];
                c.r[5] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8020cb0c; f_801f02ac(c);
                pc = 0x8020cb0c;
            }
            0x8020cb0c => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x8020cb14; f_801f7404(c);
                pc = 0x8020cb14;
            }
            0x8020cb14 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020cb24; continue; }
                pc = 0x8020cb1c;
            }
            0x8020cb1c => {
                c.r[3] = 0x0;
                pc = 0x8020cfcc; continue;
                pc = 0x8020cb24;
            }
            0x8020cb24 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020cb2c; f_801f7388(c);
                pc = 0x8020cb2c;
            }
            0x8020cb2c => {
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                c.r[23] = 0x0;
                pc = 0x8020cc6c; continue;
                pc = 0x8020cb38;
            }
            0x8020cb38 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8020cb44; f_801f7258(c);
                pc = 0x8020cb44;
            }
            0x8020cb44 => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if c.crbit(2) { pc = 0x8020cc68; continue; }
                pc = 0x8020cb4c;
            }
            0x8020cb4c => {
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020cb5c; f_801fb1c0(c);
                pc = 0x8020cb5c;
            }
            0x8020cb5c => {
                c.r[21] = c.r[3] | c.r[3];
                c.rc(c.r[21]);
                if c.crbit(2) { pc = 0x8020cc68; continue; }
                pc = 0x8020cb64;
            }
            0x8020cb64 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8020cb6c; f_801f98bc(c);
                pc = 0x8020cb6c;
            }
            0x8020cb6c => {
                c.r[24] = 0x0;
                c.r[22] = 0x0;
                pc = 0x8020cc5c; continue;
                pc = 0x8020cb78;
            }
            0x8020cb78 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if !c.crbit(0) { pc = 0x8020cc68; continue; }
                pc = 0x8020cb84;
            }
            0x8020cb84 => {
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(0) { pc = 0x8020cc68; continue; }
                pc = 0x8020cb8c;
            }
            0x8020cb8c => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[6] = c.r[22] | c.r[22];
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x8020cba0; f_801fb1c0(c);
                pc = 0x8020cba0;
            }
            0x8020cba0 => {
                c.r[20] = c.r[3] | c.r[3];
                c.lr = 0x8020cba8; f_80206608(c);
                pc = 0x8020cba8;
            }
            0x8020cba8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020cc58; continue; }
                pc = 0x8020cbb0;
            }
            0x8020cbb0 => {
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8020cbc0; f_8010ae2c(c);
                pc = 0x8020cbc0;
            }
            0x8020cbc0 => {
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x8020cbc8; f_80205be8(c);
                pc = 0x8020cbc8;
            }
            0x8020cbc8 => {
                c.lr = 0x8020cbcc; f_80121c18(c);
                pc = 0x8020cbcc;
            }
            0x8020cbcc => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.r[19] = c.r[0] | c.r[0];
                c.r[6] = c.r[22] | c.r[22];
                c.r[4] = 0x0;
                c.r[5] = 0x46;
                c.lr = 0x8020cbe8; f_801fb1c0(c);
                pc = 0x8020cbe8;
            }
            0x8020cbe8 => {
                c.r[4] = c.r[20] | c.r[20];
                c.r[26] = c.r[3] | c.r[3];
                c.r[5] = c.r[19] | c.r[19];
                c.lr = 0x8020cbf8; f_802068c8(c);
                pc = 0x8020cbf8;
            }
            0x8020cbf8 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[24] = c.r[24].wrapping_add(0x1);
                c.lr = 0x8020cc04; f_80208028(c);
                pc = 0x8020cc04;
            }
            0x8020cc04 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x1e;
                c.r[6] = 0x0;
                c.lr = 0x8020cc18; f_801f54a4(c);
                pc = 0x8020cc18;
            }
            0x8020cc18 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020cc4c; continue; }
                pc = 0x8020cc24;
            }
            0x8020cc24 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020cc2c; f_80204a10(c);
                pc = 0x8020cc2c;
            }
            0x8020cc2c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020cc4c; continue; }
                pc = 0x8020cc34;
            }
            0x8020cc34 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.lr = 0x8020cc40; f_80205ad4(c);
                pc = 0x8020cc40;
            }
            0x8020cc40 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.lr = 0x8020cc4c; f_80205a7c(c);
                pc = 0x8020cc4c;
            }
            0x8020cc4c => {
                c.r[3] = c.r[21] | c.r[21];
                c.r[4] = c.r[19] | c.r[19];
                c.lr = 0x8020cc58; f_801c3e3c(c);
                pc = 0x8020cc58;
            }
            0x8020cc58 => {
                c.r[22] = c.r[22].wrapping_add(0x1);
                pc = 0x8020cc5c;
            }
            0x8020cc5c => {
                c.r[0] = c.r[22].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x6);
                if c.crbit(0) { pc = 0x8020cb78; continue; }
                pc = 0x8020cc68;
            }
            0x8020cc68 => {
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020cc6c;
            }
            0x8020cc6c => {
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[28]);
                if c.crbit(0) { pc = 0x8020cb38; continue; }
                pc = 0x8020cc78;
            }
            0x8020cc78 => {
                c.r[21] = 0x0;
                pc = 0x8020cd08; continue;
                pc = 0x8020cc80;
            }
            0x8020cc80 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8020cc8c; f_801f7258(c);
                pc = 0x8020cc8c;
            }
            0x8020cc8c => {
                c.r[22] = c.r[3] | c.r[3];
                c.rc(c.r[22]);
                if c.crbit(2) { pc = 0x8020cd04; continue; }
                pc = 0x8020cc94;
            }
            0x8020cc94 => {
                c.r[23] = 0x0;
                pc = 0x8020ccb4; continue;
                pc = 0x8020cc9c;
            }
            0x8020cc9c => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8020cca8; f_801f981c(c);
                pc = 0x8020cca8;
            }
            0x8020cca8 => {
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                if !c.crbit(2) { pc = 0x8020ccc0; continue; }
                pc = 0x8020ccb0;
            }
            0x8020ccb0 => {
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020ccb4;
            }
            0x8020ccb4 => {
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020cc9c; continue; }
                pc = 0x8020ccc0;
            }
            0x8020ccc0 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x0;
                c.lr = 0x8020ccd0; f_801fbc20(c);
                pc = 0x8020ccd0;
            }
            0x8020ccd0 => {
                c.r[23] = 0x0;
                pc = 0x8020ccf8; continue;
                pc = 0x8020ccd8;
            }
            0x8020ccd8 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8020cce4; f_801f981c(c);
                pc = 0x8020cce4;
            }
            0x8020cce4 => {
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                if c.crbit(2) { pc = 0x8020ccf4; continue; }
                pc = 0x8020ccec;
            }
            0x8020ccec => {
                c.r[4] = 0x0;
                c.lr = 0x8020ccf4; f_80208c18(c);
                pc = 0x8020ccf4;
            }
            0x8020ccf4 => {
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020ccf8;
            }
            0x8020ccf8 => {
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020ccd8; continue; }
                pc = 0x8020cd04;
            }
            0x8020cd04 => {
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020cd08;
            }
            0x8020cd08 => {
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[28]);
                if c.crbit(0) { pc = 0x8020cc80; continue; }
                pc = 0x8020cd14;
            }
            0x8020cd14 => {
                c.r[25] = 0x0;
                pc = 0x8020cf1c; continue;
                pc = 0x8020cd1c;
            }
            0x8020cd1c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[25] | c.r[25];
                c.lr = 0x8020cd28; f_801f7258(c);
                pc = 0x8020cd28;
            }
            0x8020cd28 => {
                c.r[24] = c.r[3] | c.r[3];
                c.rc(c.r[24]);
                if c.crbit(2) { pc = 0x8020cf18; continue; }
                pc = 0x8020cd30;
            }
            0x8020cd30 => {
                c.lr = 0x8020cd34; f_801f98bc(c);
                pc = 0x8020cd34;
            }
            0x8020cd34 => {
                c.r[19] = c.r[3].rotate_left(0) & 0xff;
                c.r[21] = 0x0;
                pc = 0x8020cd78; continue;
                pc = 0x8020cd40;
            }
            0x8020cd40 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8020cd4c; f_801f981c(c);
                pc = 0x8020cd4c;
            }
            0x8020cd4c => {
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                if c.crbit(2) { pc = 0x8020cd74; continue; }
                pc = 0x8020cd54;
            }
            0x8020cd54 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = c.r[19] | c.r[19];
                c.r[7] = c.r[25] | c.r[25];
                c.r[8] = c.r[21] | c.r[21];
                c.r[9] = 0x0;
                c.lr = 0x8020cd74; f_8020cfe0(c);
                pc = 0x8020cd74;
            }
            0x8020cd74 => {
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020cd78;
            }
            0x8020cd78 => {
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020cd40; continue; }
                pc = 0x8020cd84;
            }
            0x8020cd84 => {
                c.r[21] = 0x0;
                pc = 0x8020cda4; continue;
                pc = 0x8020cd8c;
            }
            0x8020cd8c => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8020cd98; f_801f981c(c);
                pc = 0x8020cd98;
            }
            0x8020cd98 => {
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                if !c.crbit(2) { pc = 0x8020cdb0; continue; }
                pc = 0x8020cda0;
            }
            0x8020cda0 => {
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020cda4;
            }
            0x8020cda4 => {
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020cd8c; continue; }
                pc = 0x8020cdb0;
            }
            0x8020cdb0 => {
                c.lr = 0x8020cdb4; f_801c3430(c);
                pc = 0x8020cdb4;
            }
            0x8020cdb4 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x1;
                c.lr = 0x8020cdc4; f_801fbc20(c);
                pc = 0x8020cdc4;
            }
            0x8020cdc4 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020cdcc; f_801f8000(c);
                pc = 0x8020cdcc;
            }
            0x8020cdcc => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x22;
                c.lr = 0x8020cdd8; f_80132a38(c);
                pc = 0x8020cdd8;
            }
            0x8020cdd8 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020cde0; f_801f8100(c);
                pc = 0x8020cde0;
            }
            0x8020cde0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x23;
                c.lr = 0x8020cdec; f_80132a38(c);
                pc = 0x8020cdec;
            }
            0x8020cdec => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8020cdf4; f_801f8100(c);
                pc = 0x8020cdf4;
            }
            0x8020cdf4 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x25;
                c.lr = 0x8020ce00; f_80132a38(c);
                pc = 0x8020ce00;
            }
            0x8020ce00 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = c.r[19] | c.r[19];
                c.r[7] = c.r[25] | c.r[25];
                c.r[8] = c.r[21] | c.r[21];
                c.r[9] = 0x1;
                c.lr = 0x8020ce20; f_8020cfe0(c);
                pc = 0x8020ce20;
            }
            0x8020ce20 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x2;
                c.lr = 0x8020ce30; f_801fbc20(c);
                pc = 0x8020ce30;
            }
            0x8020ce30 => {
                c.r[21] = 0x0;
                pc = 0x8020cf0c; continue;
                pc = 0x8020ce38;
            }
            0x8020ce38 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8020ce44; f_801f981c(c);
                pc = 0x8020ce44;
            }
            0x8020ce44 => {
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                if c.crbit(2) { pc = 0x8020cf08; continue; }
                pc = 0x8020ce4c;
            }
            0x8020ce4c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020ce60; f_801f54a4(c);
                pc = 0x8020ce60;
            }
            0x8020ce60 => {
                c.r[20] = c.r[3] | c.r[3];
                c.r[7] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020ce7c; f_801f4c14(c);
                pc = 0x8020ce7c;
            }
            0x8020ce7c => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x1;
                c.lr = 0x8020ce88; f_80208c18(c);
                pc = 0x8020ce88;
            }
            0x8020ce88 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020ce90; f_80204a10(c);
                pc = 0x8020ce90;
            }
            0x8020ce90 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020ceac; continue; }
                pc = 0x8020ce98;
            }
            0x8020ce98 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.lr = 0x8020cea8; f_80265598(c);
                pc = 0x8020cea8;
            }
            0x8020cea8 => {
                pc = 0x8020cebc; continue;
                pc = 0x8020ceac;
            }
            0x8020ceac => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.lr = 0x8020cebc; f_80265598(c);
                pc = 0x8020cebc;
            }
            0x8020cebc => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x2;
                c.lr = 0x8020cec8; f_80208c18(c);
                pc = 0x8020cec8;
            }
            0x8020cec8 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x3;
                c.lr = 0x8020ced4; f_80208c18(c);
                pc = 0x8020ced4;
            }
            0x8020ced4 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x4;
                c.lr = 0x8020cee0; f_80208c18(c);
                pc = 0x8020cee0;
            }
            0x8020cee0 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.lr = 0x8020cef0; f_8026532c(c);
                pc = 0x8020cef0;
            }
            0x8020cef0 => {
                c.r[7] = c.r[20] | c.r[20];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x36;
                c.r[6] = 0x0;
                c.lr = 0x8020cf08; f_801f4c14(c);
                pc = 0x8020cf08;
            }
            0x8020cf08 => {
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020cf0c;
            }
            0x8020cf0c => {
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020ce38; continue; }
                pc = 0x8020cf18;
            }
            0x8020cf18 => {
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x8020cf1c;
            }
            0x8020cf1c => {
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[28]);
                if c.crbit(0) { pc = 0x8020cd1c; continue; }
                pc = 0x8020cf28;
            }
            0x8020cf28 => {
                c.lr = 0x8020cf2c; f_8026246c(c);
                pc = 0x8020cf2c;
            }
            0x8020cf2c => {
                c.r[21] = 0x0;
                pc = 0x8020cfbc; continue;
                pc = 0x8020cf34;
            }
            0x8020cf34 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x8020cf40; f_801f7258(c);
                pc = 0x8020cf40;
            }
            0x8020cf40 => {
                c.r[22] = c.r[3] | c.r[3];
                c.rc(c.r[22]);
                if c.crbit(2) { pc = 0x8020cfb8; continue; }
                pc = 0x8020cf48;
            }
            0x8020cf48 => {
                c.r[23] = 0x0;
                pc = 0x8020cf68; continue;
                pc = 0x8020cf50;
            }
            0x8020cf50 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8020cf5c; f_801f981c(c);
                pc = 0x8020cf5c;
            }
            0x8020cf5c => {
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                if !c.crbit(2) { pc = 0x8020cf74; continue; }
                pc = 0x8020cf64;
            }
            0x8020cf64 => {
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020cf68;
            }
            0x8020cf68 => {
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020cf50; continue; }
                pc = 0x8020cf74;
            }
            0x8020cf74 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = 0x3;
                c.lr = 0x8020cf84; f_801fbc20(c);
                pc = 0x8020cf84;
            }
            0x8020cf84 => {
                c.r[23] = 0x0;
                pc = 0x8020cfac; continue;
                pc = 0x8020cf8c;
            }
            0x8020cf8c => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8020cf98; f_801f981c(c);
                pc = 0x8020cf98;
            }
            0x8020cf98 => {
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                if c.crbit(2) { pc = 0x8020cfa8; continue; }
                pc = 0x8020cfa0;
            }
            0x8020cfa0 => {
                c.r[4] = 0x5;
                c.lr = 0x8020cfa8; f_80208c18(c);
                pc = 0x8020cfa8;
            }
            0x8020cfa8 => {
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x8020cfac;
            }
            0x8020cfac => {
                c.r[0] = c.r[23].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8020cf8c; continue; }
                pc = 0x8020cfb8;
            }
            0x8020cfb8 => {
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020cfbc;
            }
            0x8020cfbc => {
                c.r[0] = c.r[21].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[28]);
                if c.crbit(0) { pc = 0x8020cf34; continue; }
                pc = 0x8020cfc8;
            }
            0x8020cfc8 => {
                c.r[3] = 0x1;
                pc = 0x8020cfcc;
            }
            0x8020cfcc => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 19..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_8020ca98: bad pc {:#x}", pc),
        }
    }
}

/// _fightActionFlowKaisiNyuujouPokemonSubAppearMsg__FP13FIGHT_TRAINERP15FightOutPokemonUsUsUsUsUc (0x8020cfe0, 540 bytes)
#[inline(never)]
pub fn f_8020cfe0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020cfe0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020cfe0); f_8020cfe0_b(c, 0x8020cfe0); crate::prof::exit(c, 0x8020cfe0); return; }
    }
    f_8020cfe0_b(c, 0x8020cfe0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020cfe0_at(c: &mut Cpu, pc: u32) { f_8020cfe0_b(c, pc) }
#[inline(always)]
fn f_8020cfe0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020cfe0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020cfe0, pc); }
        match pc {
            0x8020cfe0 => {
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
                c.lr = 0x8020d010; f_80204a10(c);
                pc = 0x8020d010;
            }
            0x8020d010 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x0;
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[30] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.lr = 0x8020d028; f_801f18dc(c);
                pc = 0x8020d028;
            }
            0x8020d028 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[29] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.lr = 0x8020d040; f_801f8000(c);
                pc = 0x8020d040;
            }
            0x8020d040 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d054; continue; }
                pc = 0x8020d048;
            }
            0x8020d048 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020d054; continue; }
                pc = 0x8020d050;
            }
            0x8020d050 => {
                c.r[29] = 0x1;
                pc = 0x8020d054;
            }
            0x8020d054 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8020d05c; f_80205b8c(c);
                pc = 0x8020d05c;
            }
            0x8020d05c => {
                c.r[4] = 0x0;
                c.r[5] = 0x77;
                c.r[6] = 0x0;
                c.lr = 0x8020d06c; f_8012640c(c);
                pc = 0x8020d06c;
            }
            0x8020d06c => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x8020d150; continue; }
                pc = 0x8020d078;
            }
            0x8020d078 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d0c4; continue; }
                pc = 0x8020d084;
            }
            0x8020d084 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020d0a8; continue; }
                pc = 0x8020d08c;
            }
            0x8020d08c => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x14;
                c.lr = 0x8020d098; f_80132a38(c);
                pc = 0x8020d098;
            }
            0x8020d098 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x16;
                c.lr = 0x8020d0a4; f_80132a38(c);
                pc = 0x8020d0a4;
            }
            0x8020d0a4 => {
                pc = 0x8020d1e8; continue;
                pc = 0x8020d0a8;
            }
            0x8020d0a8 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x15;
                c.lr = 0x8020d0b4; f_80132a38(c);
                pc = 0x8020d0b4;
            }
            0x8020d0b4 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x17;
                c.lr = 0x8020d0c0; f_80132a38(c);
                pc = 0x8020d0c0;
            }
            0x8020d0c0 => {
                pc = 0x8020d1e8; continue;
                pc = 0x8020d0c4;
            }
            0x8020d0c4 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d110; continue; }
                pc = 0x8020d0d0;
            }
            0x8020d0d0 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020d0f4; continue; }
                pc = 0x8020d0d8;
            }
            0x8020d0d8 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x15;
                c.lr = 0x8020d0e4; f_80132a38(c);
                pc = 0x8020d0e4;
            }
            0x8020d0e4 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x17;
                c.lr = 0x8020d0f0; f_80132a38(c);
                pc = 0x8020d0f0;
            }
            0x8020d0f0 => {
                pc = 0x8020d1e8; continue;
                pc = 0x8020d0f4;
            }
            0x8020d0f4 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x14;
                c.lr = 0x8020d100; f_80132a38(c);
                pc = 0x8020d100;
            }
            0x8020d100 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x16;
                c.lr = 0x8020d10c; f_80132a38(c);
                pc = 0x8020d10c;
            }
            0x8020d10c => {
                pc = 0x8020d1e8; continue;
                pc = 0x8020d110;
            }
            0x8020d110 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020d134; continue; }
                pc = 0x8020d118;
            }
            0x8020d118 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x14;
                c.lr = 0x8020d124; f_80132a38(c);
                pc = 0x8020d124;
            }
            0x8020d124 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x16;
                c.lr = 0x8020d130; f_80132a38(c);
                pc = 0x8020d130;
            }
            0x8020d130 => {
                pc = 0x8020d1e8; continue;
                pc = 0x8020d134;
            }
            0x8020d134 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x15;
                c.lr = 0x8020d140; f_80132a38(c);
                pc = 0x8020d140;
            }
            0x8020d140 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x17;
                c.lr = 0x8020d14c; f_80132a38(c);
                pc = 0x8020d14c;
            }
            0x8020d14c => {
                pc = 0x8020d1e8; continue;
                pc = 0x8020d150;
            }
            0x8020d150 => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d1e8; continue; }
                pc = 0x8020d158;
            }
            0x8020d158 => {
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(1) { pc = 0x8020d1a0; continue; }
                pc = 0x8020d164;
            }
            0x8020d164 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x8020d1a0; continue; }
                pc = 0x8020d170;
            }
            0x8020d170 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d184; continue; }
                pc = 0x8020d17c;
            }
            0x8020d17c => {
                c.r[3] = 0x7674;
                pc = 0x8020d1e4; continue;
                pc = 0x8020d184;
            }
            0x8020d184 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d198; continue; }
                pc = 0x8020d190;
            }
            0x8020d190 => {
                c.r[3] = 0x7679;
                pc = 0x8020d1e4; continue;
                pc = 0x8020d198;
            }
            0x8020d198 => {
                c.r[3] = 0x7671;
                pc = 0x8020d1e4; continue;
                pc = 0x8020d1a0;
            }
            0x8020d1a0 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x14;
                c.lr = 0x8020d1ac; f_80132a38(c);
                pc = 0x8020d1ac;
            }
            0x8020d1ac => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x16;
                c.lr = 0x8020d1b8; f_80132a38(c);
                pc = 0x8020d1b8;
            }
            0x8020d1b8 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d1cc; continue; }
                pc = 0x8020d1c4;
            }
            0x8020d1c4 => {
                c.r[3] = 0x7673;
                pc = 0x8020d1e4; continue;
                pc = 0x8020d1cc;
            }
            0x8020d1cc => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d1e0; continue; }
                pc = 0x8020d1d8;
            }
            0x8020d1d8 => {
                c.r[3] = 0x7678;
                pc = 0x8020d1e4; continue;
                pc = 0x8020d1e0;
            }
            0x8020d1e0 => {
                c.r[3] = 0x7670;
                pc = 0x8020d1e4;
            }
            0x8020d1e4 => {
                c.lr = 0x8020d1e8; f_802624cc(c);
                pc = 0x8020d1e8;
            }
            0x8020d1e8 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8020cfe0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaisiNyuujouTrainer (0x8020d1fc, 1180 bytes)
#[inline(never)]
pub fn f_8020d1fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d1fc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d1fc); f_8020d1fc_b(c, 0x8020d1fc); crate::prof::exit(c, 0x8020d1fc); return; }
    }
    f_8020d1fc_b(c, 0x8020d1fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d1fc_at(c: &mut Cpu, pc: u32) { f_8020d1fc_b(c, pc) }
#[inline(always)]
fn f_8020d1fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d1fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d1fc, pc); }
        match pc {
            0x8020d1fc => {
                { let ea = c.r[1].wrapping_add(0xfffff4a0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xb64), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xb28); for n in 18..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020d210; f_8020d920(c);
                pc = 0x8020d210;
            }
            0x8020d210 => {
                c.lr = 0x8020d214; f_8020d814(c);
                pc = 0x8020d214;
            }
            0x8020d214 => {
                c.r[18] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0xd;
                c.r[6] = 0x0;
                c.lr = 0x8020d22c; f_801f54a4(c);
                pc = 0x8020d22c;
            }
            0x8020d22c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8020d234; f_8020e0f8(c);
                pc = 0x8020d234;
            }
            0x8020d234 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[23] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8020d250; f_801f54a4(c);
                pc = 0x8020d250;
            }
            0x8020d250 => {
                c.r[19] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x16;
                c.r[6] = 0x0;
                c.lr = 0x8020d268; f_801f54a4(c);
                pc = 0x8020d268;
            }
            0x8020d268 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x17;
                c.r[6] = 0x0;
                c.lr = 0x8020d280; f_801f54a4(c);
                pc = 0x8020d280;
            }
            0x8020d280 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x18;
                c.r[6] = 0x0;
                c.lr = 0x8020d298; f_801f54a4(c);
                pc = 0x8020d298;
            }
            0x8020d298 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[18] | c.r[18];
                c.r[5] = c.r[19] | c.r[19];
                c.r[4] = 0x0;
                c.lr = 0x8020d2ac; f_801f02ac(c);
                pc = 0x8020d2ac;
            }
            0x8020d2ac => {
                c.r[25] = c.r[3] | c.r[3];
                c.lr = 0x8020d2b4; f_801f7404(c);
                pc = 0x8020d2b4;
            }
            0x8020d2b4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020d2c4; continue; }
                pc = 0x8020d2bc;
            }
            0x8020d2bc => {
                c.r[3] = 0x0;
                pc = 0x8020d684; continue;
                pc = 0x8020d2c4;
            }
            0x8020d2c4 => {
                c.r[5] = c.addc(!c.r[18], 0x4, 1);
                c.r[3] = c.r[25] | c.r[25];
                c.r[0] = c.addc(c.r[5], 0xffffffff, 0);
                c.r[4] = 0x0;
                { let ci = c.ca(); c.r[18] = c.addc(!c.r[0], c.r[5], ci); }
                c.r[5] = 0x5;
                c.r[6] = 0x0;
                c.lr = 0x8020d2e4; f_801f76b8(c);
                pc = 0x8020d2e4;
            }
            0x8020d2e4 => {
                c.r[0] = c.r[18].rotate_left(0) & 0xffff;
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[29] = c.r[0].wrapping_mul(c.r[28]);
                c.r[24] = 0x0;
                pc = 0x8020d5ec; continue;
                pc = 0x8020d2f8;
            }
            0x8020d2f8 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[6] = c.r[24] | c.r[24];
                c.r[4] = 0x0;
                c.r[5] = 0x7;
                c.lr = 0x8020d30c; f_801f76b8(c);
                pc = 0x8020d30c;
            }
            0x8020d30c => {
                c.r[0] = c.r[24].wrapping_add(c.r[29]);
                c.r[4] = c.r[3] | c.r[3];
                c.r[20] = c.r[0].rotate_left(0) & 0xff;
                c.r[3] = c.r[23] | c.r[23];
                c.r[26] = c.r[4] | c.r[4];
                c.r[4] = c.r[20] | c.r[20];
                c.lr = 0x8020d328; f_8020e068(c);
                pc = 0x8020d328;
            }
            0x8020d328 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.r[18] = c.r[0] | c.r[0];
                c.r[4] = c.r[20] | c.r[20];
                c.lr = 0x8020d33c; f_8020e020(c);
                pc = 0x8020d33c;
            }
            0x8020d33c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[18] | c.r[18];
                c.r[19] = c.r[0] | c.r[0];
                c.r[4] = c.r[19] | c.r[19];
                c.lr = 0x8020d350; f_801fa4b4(c);
                pc = 0x8020d350;
            }
            0x8020d350 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020d5e8; continue; }
                pc = 0x8020d358;
            }
            0x8020d358 => {
                c.lr = 0x8020d35c; f_8006b57c(c);
                pc = 0x8020d35c;
            }
            0x8020d35c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d380; continue; }
                pc = 0x8020d368;
            }
            0x8020d368 => {
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x8020d370; f_8006b0f8(c);
                pc = 0x8020d370;
            }
            0x8020d370 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.lr = 0x8020d37c; f_8012ac64(c);
                pc = 0x8020d37c;
            }
            0x8020d37c => {
                pc = 0x8020d390; continue;
                pc = 0x8020d380;
            }
            0x8020d380 => {
                c.r[3] = c.r[18] | c.r[18];
                c.r[4] = c.r[19] | c.r[19];
                c.r[5] = c.r[1].wrapping_add(0xc);
                c.lr = 0x8020d390; f_801f9cbc(c);
                pc = 0x8020d390;
            }
            0x8020d390 => {
                c.r[3] = c.r[18] | c.r[18];
                c.lr = 0x8020d398; f_801f8fd8(c);
                pc = 0x8020d398;
            }
            0x8020d398 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[7] = c.r[0] | c.r[0];
                c.r[5] = c.r[18] | c.r[18];
                c.r[6] = c.r[19] | c.r[19];
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.lr = 0x8020d3b4; f_801fa6d8(c);
                pc = 0x8020d3b4;
            }
            0x8020d3b4 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = 0x0;
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.lr = 0x8020d3c8; f_801fb1c0(c);
                pc = 0x8020d3c8;
            }
            0x8020d3c8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[26] | c.r[26];
                c.r[22] = c.r[0] | c.r[0];
                c.lr = 0x8020d3d8; f_801fa634(c);
                pc = 0x8020d3d8;
            }
            0x8020d3d8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020d5e8; continue; }
                pc = 0x8020d3e0;
            }
            0x8020d3e0 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8020d3f0; f_801f99c8(c);
                pc = 0x8020d3f0;
            }
            0x8020d3f0 => {
                c.r[21] = 0x0;
                c.r[19] = 0x0;
                pc = 0x8020d4e8; continue;
                pc = 0x8020d3fc;
            }
            0x8020d3fc => {
                c.r[0] = c.r[21] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], c.r[31]);
                if !c.crbit(0) { pc = 0x8020d4f4; continue; }
                pc = 0x8020d408;
            }
            0x8020d408 => {
                c.cmps(0, c.r[0], c.r[27]);
                if !c.crbit(0) { pc = 0x8020d4f4; continue; }
                pc = 0x8020d410;
            }
            0x8020d410 => {
                c.cmps(0, c.r[0], 0x6);
                if !c.crbit(0) { pc = 0x8020d4f4; continue; }
                pc = 0x8020d418;
            }
            0x8020d418 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[5] = c.r[19] | c.r[19];
                c.r[4] = 0x3;
                c.lr = 0x8020d428; f_8012a5b0(c);
                pc = 0x8020d428;
            }
            0x8020d428 => {
                c.r[18] = c.r[3] | c.r[3];
                c.lr = 0x8020d430; f_801233f4(c);
                pc = 0x8020d430;
            }
            0x8020d430 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020d4e4; continue; }
                pc = 0x8020d438;
            }
            0x8020d438 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[18] | c.r[18];
                c.lr = 0x8020d444; f_801f9930(c);
                pc = 0x8020d444;
            }
            0x8020d444 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d4e4; continue; }
                pc = 0x8020d44c;
            }
            0x8020d44c => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[21] as u8 as i8 as i32 as u32;
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x8020d460; f_801fb1c0(c);
                pc = 0x8020d460;
            }
            0x8020d460 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[20] = c.r[0] | c.r[0];
                c.lr = 0x8020d470; f_801f4804(c);
                pc = 0x8020d470;
            }
            0x8020d470 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = c.r[18] | c.r[18];
                c.lr = 0x8020d480; f_80206aec(c);
                pc = 0x8020d480;
            }
            0x8020d480 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x8020d494; f_801f54a4(c);
                pc = 0x8020d494;
            }
            0x8020d494 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d4e0; continue; }
                pc = 0x8020d4a0;
            }
            0x8020d4a0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x2e;
                c.r[6] = 0x0;
                c.lr = 0x8020d4b4; f_801f54a4(c);
                pc = 0x8020d4b4;
            }
            0x8020d4b4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d4e0; continue; }
                pc = 0x8020d4c0;
            }
            0x8020d4c0 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020d4c8; f_801fb8f8(c);
                pc = 0x8020d4c8;
            }
            0x8020d4c8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d4e0; continue; }
                pc = 0x8020d4d4;
            }
            0x8020d4d4 => {
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = 0x3;
                c.lr = 0x8020d4e0; f_802032e4(c);
                pc = 0x8020d4e0;
            }
            0x8020d4e0 => {
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020d4e4;
            }
            0x8020d4e4 => {
                c.r[19] = c.r[19].wrapping_add(0x1);
                pc = 0x8020d4e8;
            }
            0x8020d4e8 => {
                c.r[0] = c.r[19].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x6);
                if c.crbit(0) { pc = 0x8020d3fc; continue; }
                pc = 0x8020d4f4;
            }
            0x8020d4f4 => {
                c.r[20] = 0x0;
                pc = 0x8020d5dc; continue;
                pc = 0x8020d4fc;
            }
            0x8020d4fc => {
                c.r[0] = c.r[21] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], c.r[27]);
                if !c.crbit(0) { pc = 0x8020d5e8; continue; }
                pc = 0x8020d508;
            }
            0x8020d508 => {
                c.cmps(0, c.r[0], 0x6);
                if !c.crbit(0) { pc = 0x8020d5e8; continue; }
                pc = 0x8020d510;
            }
            0x8020d510 => {
                c.r[3] = c.r[22] | c.r[22];
                c.r[5] = c.r[20] | c.r[20];
                c.r[4] = 0x3;
                c.lr = 0x8020d520; f_8012a5b0(c);
                pc = 0x8020d520;
            }
            0x8020d520 => {
                c.r[18] = c.r[3] | c.r[3];
                c.lr = 0x8020d528; f_80123fbc(c);
                pc = 0x8020d528;
            }
            0x8020d528 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020d5d8; continue; }
                pc = 0x8020d530;
            }
            0x8020d530 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[18] | c.r[18];
                c.lr = 0x8020d53c; f_801f9930(c);
                pc = 0x8020d53c;
            }
            0x8020d53c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d5d8; continue; }
                pc = 0x8020d544;
            }
            0x8020d544 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[21] as u8 as i8 as i32 as u32;
                c.r[4] = 0x0;
                c.r[5] = 0x45;
                c.lr = 0x8020d558; f_801fb1c0(c);
                pc = 0x8020d558;
            }
            0x8020d558 => {
                c.r[19] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020d564; f_801f4804(c);
                pc = 0x8020d564;
            }
            0x8020d564 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = c.r[18] | c.r[18];
                c.lr = 0x8020d574; f_80206aec(c);
                pc = 0x8020d574;
            }
            0x8020d574 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x27;
                c.r[6] = 0x0;
                c.lr = 0x8020d588; f_801f54a4(c);
                pc = 0x8020d588;
            }
            0x8020d588 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d5d4; continue; }
                pc = 0x8020d594;
            }
            0x8020d594 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x2e;
                c.r[6] = 0x0;
                c.lr = 0x8020d5a8; f_801f54a4(c);
                pc = 0x8020d5a8;
            }
            0x8020d5a8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d5d4; continue; }
                pc = 0x8020d5b4;
            }
            0x8020d5b4 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x8020d5bc; f_801fb8f8(c);
                pc = 0x8020d5bc;
            }
            0x8020d5bc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020d5d4; continue; }
                pc = 0x8020d5c8;
            }
            0x8020d5c8 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x3;
                c.lr = 0x8020d5d4; f_802032e4(c);
                pc = 0x8020d5d4;
            }
            0x8020d5d4 => {
                c.r[21] = c.r[21].wrapping_add(0x1);
                pc = 0x8020d5d8;
            }
            0x8020d5d8 => {
                c.r[20] = c.r[20].wrapping_add(0x1);
                pc = 0x8020d5dc;
            }
            0x8020d5dc => {
                c.r[0] = c.r[20].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x6);
                if c.crbit(0) { pc = 0x8020d4fc; continue; }
                pc = 0x8020d5e8;
            }
            0x8020d5e8 => {
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x8020d5ec;
            }
            0x8020d5ec => {
                c.r[0] = c.r[24].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[28]);
                if c.crbit(0) { pc = 0x8020d2f8; continue; }
                pc = 0x8020d5f8;
            }
            0x8020d5f8 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8020d600; f_801f7388(c);
                pc = 0x8020d600;
            }
            0x8020d600 => {
                c.r[18] = c.r[3].rotate_left(0) & 0xff;
                c.r[20] = 0x0;
                pc = 0x8020d674; continue;
                pc = 0x8020d60c;
            }
            0x8020d60c => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[20] | c.r[20];
                c.lr = 0x8020d618; f_801f7258(c);
                pc = 0x8020d618;
            }
            0x8020d618 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020d670; continue; }
                pc = 0x8020d620;
            }
            0x8020d620 => {
                c.r[4] = 0x0;
                c.r[5] = 0x4c;
                c.r[6] = 0x0;
                c.lr = 0x8020d630; f_801fb1c0(c);
                pc = 0x8020d630;
            }
            0x8020d630 => {
                c.r[19] = c.r[3] | c.r[3];
                c.rc(c.r[19]);
                if c.crbit(2) { pc = 0x8020d670; continue; }
                pc = 0x8020d638;
            }
            0x8020d638 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[18] | c.r[18];
                c.r[5] = c.r[20] | c.r[20];
                c.r[6] = c.r[1].wrapping_add(0x9);
                c.r[7] = c.r[1].wrapping_add(0x8);
                c.lr = 0x8020d650; f_801f72b0(c);
                pc = 0x8020d650;
            }
            0x8020d650 => {
                c.r[4] = c.lbz(c.r[1].wrapping_add(0x9));
                c.r[3] = c.r[19] | c.r[19];
                c.r[5] = c.lbz(c.r[1].wrapping_add(0x8));
                c.lr = 0x8020d660; f_801c3fbc(c);
                pc = 0x8020d660;
            }
            0x8020d660 => {
                c.lr = 0x8020d664; f_801c3430(c);
                pc = 0x8020d664;
            }
            0x8020d664 => {
                c.r[3] = c.r[19] | c.r[19];
                c.r[4] = 0x1;
                c.lr = 0x8020d670; f_801da4e8(c);
                pc = 0x8020d670;
            }
            0x8020d670 => {
                c.r[20] = c.r[20].wrapping_add(0x1);
                pc = 0x8020d674;
            }
            0x8020d674 => {
                c.r[0] = c.r[20].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[28]);
                if c.crbit(0) { pc = 0x8020d60c; continue; }
                pc = 0x8020d680;
            }
            0x8020d680 => {
                c.r[3] = 0x1;
                pc = 0x8020d684;
            }
            0x8020d684 => {
                { let mut ea = c.r[1].wrapping_add(0xb28); for n in 18..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xb64));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xb60);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d1fc: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowKaijou (0x8020d698, 236 bytes)
#[inline(never)]
pub fn f_8020d698(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d698) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d698); f_8020d698_b(c, 0x8020d698); crate::prof::exit(c, 0x8020d698); return; }
    }
    f_8020d698_b(c, 0x8020d698)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d698_at(c: &mut Cpu, pc: u32) { f_8020d698_b(c, pc) }
#[inline(always)]
fn f_8020d698_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d698;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d698, pc); }
        match pc {
            0x8020d698 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8020d6ac; f_801ef624(c);
                pc = 0x8020d6ac;
            }
            0x8020d6ac => {
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x8020d6b4; f_8020e0f8(c);
                pc = 0x8020d6b4;
            }
            0x8020d6b4 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x8020d6d0; f_801f54a4(c);
                pc = 0x8020d6d0;
            }
            0x8020d6d0 => {
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8020d6d8; f_801f4860(c);
                pc = 0x8020d6d8;
            }
            0x8020d6d8 => {
                c.r[3] = 0x0;
                c.lr = 0x8020d6e0; f_801f17b0(c);
                pc = 0x8020d6e0;
            }
            0x8020d6e0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8020d6f4; f_801f54a4(c);
                pc = 0x8020d6f4;
            }
            0x8020d6f4 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020d700; f_8020e0b0(c);
                pc = 0x8020d700;
            }
            0x8020d700 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = 0x4;
                c.r[4] = 0x0;
                c.lr = 0x8020d714; f_801f02ac(c);
                pc = 0x8020d714;
            }
            0x8020d714 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.r[6] = 0x0;
                c.lr = 0x8020d72c; f_801f54a4(c);
                pc = 0x8020d72c;
            }
            0x8020d72c => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020d738; f_801f7480(c);
                pc = 0x8020d738;
            }
            0x8020d738 => {
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = 0x5;
                c.r[4] = 0x0;
                c.lr = 0x8020d748; f_801f02ac(c);
                pc = 0x8020d748;
            }
            0x8020d748 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.r[5] = 0x3;
                c.r[6] = 0x1;
                c.lr = 0x8020d760; f_801f54a4(c);
                pc = 0x8020d760;
            }
            0x8020d760 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8020d76c; f_801f7480(c);
                pc = 0x8020d76c;
            }
            0x8020d76c => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d698: bad pc {:#x}", pc),
        }
    }
}

/// fightActionFlowNullFunc (0x8020d784, 8 bytes)
#[inline(never)]
pub fn f_8020d784(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d784) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d784); f_8020d784_b(c, 0x8020d784); crate::prof::exit(c, 0x8020d784); return; }
    }
    f_8020d784_b(c, 0x8020d784)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d784_at(c: &mut Cpu, pc: u32) { f_8020d784_b(c, pc) }
#[inline(always)]
fn f_8020d784_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d784;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d784, pc); }
        match pc {
            0x8020d784 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8020d784: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetFifoBanme (0x8020d78c, 16 bytes)
#[inline(never)]
pub fn f_8020d78c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d78c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d78c); f_8020d78c_b(c, 0x8020d78c); crate::prof::exit(c, 0x8020d78c); return; }
    }
    f_8020d78c_b(c, 0x8020d78c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d78c_at(c: &mut Cpu, pc: u32) { f_8020d78c_b(c, pc) }
#[inline(always)]
fn f_8020d78c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d78c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d78c, pc); }
        match pc {
            0x8020d78c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d794;
            }
            0x8020d794 => {
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d78c: bad pc {:#x}", pc),
        }
    }
}

/// fightActionKindDataBiosGetDispFuncPtr (0x8020d79c, 24 bytes)
#[inline(never)]
pub fn f_8020d79c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d79c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d79c); f_8020d79c_b(c, 0x8020d79c); crate::prof::exit(c, 0x8020d79c); return; }
    }
    f_8020d79c_b(c, 0x8020d79c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d79c_at(c: &mut Cpu, pc: u32) { f_8020d79c_b(c, pc) }
#[inline(always)]
fn f_8020d79c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d79c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d79c, pc); }
        match pc {
            0x8020d79c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d7ac; continue; }
                pc = 0x8020d7a4;
            }
            0x8020d7a4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d7ac;
            }
            0x8020d7ac => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8020d79c: bad pc {:#x}", pc),
        }
    }
}

/// fightActionKindDataBiosGetFlowFuncPtr (0x8020d7b4, 24 bytes)
#[inline(never)]
pub fn f_8020d7b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d7b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d7b4); f_8020d7b4_b(c, 0x8020d7b4); crate::prof::exit(c, 0x8020d7b4); return; }
    }
    f_8020d7b4_b(c, 0x8020d7b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d7b4_at(c: &mut Cpu, pc: u32) { f_8020d7b4_b(c, pc) }
#[inline(always)]
fn f_8020d7b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d7b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d7b4, pc); }
        match pc {
            0x8020d7b4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d7c4; continue; }
                pc = 0x8020d7bc;
            }
            0x8020d7bc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d7c4;
            }
            0x8020d7c4 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020d7b4: bad pc {:#x}", pc),
        }
    }
}

/// fightActionKindDataBiosGetPri (0x8020d7cc, 28 bytes)
#[inline(never)]
pub fn f_8020d7cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d7cc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d7cc); f_8020d7cc_b(c, 0x8020d7cc); crate::prof::exit(c, 0x8020d7cc); return; }
    }
    f_8020d7cc_b(c, 0x8020d7cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d7cc_at(c: &mut Cpu, pc: u32) { f_8020d7cc_b(c, pc) }
#[inline(always)]
fn f_8020d7cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d7cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d7cc, pc); }
        match pc {
            0x8020d7cc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d7dc; continue; }
                pc = 0x8020d7d4;
            }
            0x8020d7d4 => {
                c.r[3] = 0xffffff80;
                if true { return; }
                pc = 0x8020d7dc;
            }
            0x8020d7dc => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_8020d7cc: bad pc {:#x}", pc),
        }
    }
}

/// fightActionKindDataBiosGetPtr (0x8020d7e8, 44 bytes)
#[inline(never)]
pub fn f_8020d7e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d7e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d7e8); f_8020d7e8_b(c, 0x8020d7e8); crate::prof::exit(c, 0x8020d7e8); return; }
    }
    f_8020d7e8_b(c, 0x8020d7e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d7e8_at(c: &mut Cpu, pc: u32) { f_8020d7e8_b(c, pc) }
#[inline(always)]
fn f_8020d7e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d7e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d7e8, pc); }
        match pc {
            0x8020d7e8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8528));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020d800; continue; }
                pc = 0x8020d7f8;
            }
            0x8020d7f8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d800;
            }
            0x8020d800 => {
                c.r[4] = c.r[3].wrapping_mul(0xc);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5bb8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d7e8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDataBiosGetBuff (0x8020d814, 24 bytes)
#[inline(never)]
pub fn f_8020d814(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d814) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d814); f_8020d814_b(c, 0x8020d814); crate::prof::exit(c, 0x8020d814); return; }
    }
    f_8020d814_b(c, 0x8020d814)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d814_at(c: &mut Cpu, pc: u32) { f_8020d814_b(c, pc) }
#[inline(always)]
fn f_8020d814_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d814;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d814, pc); }
        match pc {
            0x8020d814 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d824; continue; }
                pc = 0x8020d81c;
            }
            0x8020d81c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d824;
            }
            0x8020d824 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020d814: bad pc {:#x}", pc),
        }
    }
}

/// fightActionDataBiosGetKind (0x8020d82c, 24 bytes)
#[inline(never)]
pub fn f_8020d82c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d82c) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d82c); f_8020d82c_b(c, 0x8020d82c); crate::prof::exit(c, 0x8020d82c); return; }
    }
    f_8020d82c_b(c, 0x8020d82c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d82c_at(c: &mut Cpu, pc: u32) { f_8020d82c_b(c, pc) }
#[inline(always)]
fn f_8020d82c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d82c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d82c, pc); }
        match pc {
            0x8020d82c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d83c; continue; }
                pc = 0x8020d834;
            }
            0x8020d834 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d83c;
            }
            0x8020d83c => {
                c.r[3] = c.lhz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d82c: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetDispBuff (0x8020d844, 36 bytes)
#[inline(never)]
pub fn f_8020d844(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d844) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d844); f_8020d844_b(c, 0x8020d844); crate::prof::exit(c, 0x8020d844); return; }
    }
    f_8020d844_b(c, 0x8020d844)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d844_at(c: &mut Cpu, pc: u32) { f_8020d844_b(c, pc) }
#[inline(always)]
fn f_8020d844_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d844;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d844, pc); }
        match pc {
            0x8020d844 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d84c;
            }
            0x8020d84c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x4);
                if !c.crbit(0) { return; }
                pc = 0x8020d858;
            }
            0x8020d858 => {
                c.r[0] = c.r[4].rotate_left(2) & 0x3fffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d844: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetMotoFightActionDataPtr (0x8020d868, 16 bytes)
#[inline(never)]
pub fn f_8020d868(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d868) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d868); f_8020d868_b(c, 0x8020d868); crate::prof::exit(c, 0x8020d868); return; }
    }
    f_8020d868_b(c, 0x8020d868)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d868_at(c: &mut Cpu, pc: u32) { f_8020d868_b(c, pc) }
#[inline(always)]
fn f_8020d868_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d868;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d868, pc); }
        match pc {
            0x8020d868 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d870;
            }
            0x8020d870 => {
                c.stw(c.r[3].wrapping_add(0x18), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d868: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetBuffDataId (0x8020d878, 16 bytes)
#[inline(never)]
pub fn f_8020d878(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d878) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d878); f_8020d878_b(c, 0x8020d878); crate::prof::exit(c, 0x8020d878); return; }
    }
    f_8020d878_b(c, 0x8020d878)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d878_at(c: &mut Cpu, pc: u32) { f_8020d878_b(c, pc) }
#[inline(always)]
fn f_8020d878_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d878;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d878, pc); }
        match pc {
            0x8020d878 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d880;
            }
            0x8020d880 => {
                c.stw(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d878: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetBuffDataPtr (0x8020d888, 16 bytes)
#[inline(never)]
pub fn f_8020d888(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d888) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d888); f_8020d888_b(c, 0x8020d888); crate::prof::exit(c, 0x8020d888); return; }
    }
    f_8020d888_b(c, 0x8020d888)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d888_at(c: &mut Cpu, pc: u32) { f_8020d888_b(c, pc) }
#[inline(always)]
fn f_8020d888_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d888;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d888, pc); }
        match pc {
            0x8020d888 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d890;
            }
            0x8020d890 => {
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d888: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetActorFightTargetPtr (0x8020d898, 16 bytes)
#[inline(never)]
pub fn f_8020d898(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d898) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d898); f_8020d898_b(c, 0x8020d898); crate::prof::exit(c, 0x8020d898); return; }
    }
    f_8020d898_b(c, 0x8020d898)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d898_at(c: &mut Cpu, pc: u32) { f_8020d898_b(c, pc) }
#[inline(always)]
fn f_8020d898_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d898;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d898, pc); }
        match pc {
            0x8020d898 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d8a0;
            }
            0x8020d8a0 => {
                c.stw(c.r[3].wrapping_add(0x14), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d898: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetFightActionDataPtr (0x8020d8a8, 16 bytes)
#[inline(never)]
pub fn f_8020d8a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d8a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d8a8); f_8020d8a8_b(c, 0x8020d8a8); crate::prof::exit(c, 0x8020d8a8); return; }
    }
    f_8020d8a8_b(c, 0x8020d8a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d8a8_at(c: &mut Cpu, pc: u32) { f_8020d8a8_b(c, pc) }
#[inline(always)]
fn f_8020d8a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d8a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d8a8, pc); }
        match pc {
            0x8020d8a8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d8b0;
            }
            0x8020d8b0 => {
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d8a8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetBuff (0x8020d8b8, 16 bytes)
#[inline(never)]
pub fn f_8020d8b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d8b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d8b8); f_8020d8b8_b(c, 0x8020d8b8); crate::prof::exit(c, 0x8020d8b8); return; }
    }
    f_8020d8b8_b(c, 0x8020d8b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d8b8_at(c: &mut Cpu, pc: u32) { f_8020d8b8_b(c, pc) }
#[inline(always)]
fn f_8020d8b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d8b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d8b8, pc); }
        match pc {
            0x8020d8b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d8c0;
            }
            0x8020d8c0 => {
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d8b8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosSetKind (0x8020d8c8, 16 bytes)
#[inline(never)]
pub fn f_8020d8c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d8c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d8c8); f_8020d8c8_b(c, 0x8020d8c8); crate::prof::exit(c, 0x8020d8c8); return; }
    }
    f_8020d8c8_b(c, 0x8020d8c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d8c8_at(c: &mut Cpu, pc: u32) { f_8020d8c8_b(c, pc) }
#[inline(always)]
fn f_8020d8c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d8c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d8c8, pc); }
        match pc {
            0x8020d8c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d8d0;
            }
            0x8020d8d0 => {
                c.sth(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d8c8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetBuffDataId (0x8020d8d8, 24 bytes)
#[inline(never)]
pub fn f_8020d8d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d8d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d8d8); f_8020d8d8_b(c, 0x8020d8d8); crate::prof::exit(c, 0x8020d8d8); return; }
    }
    f_8020d8d8_b(c, 0x8020d8d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d8d8_at(c: &mut Cpu, pc: u32) { f_8020d8d8_b(c, pc) }
#[inline(always)]
fn f_8020d8d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d8d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d8d8, pc); }
        match pc {
            0x8020d8d8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d8e8; continue; }
                pc = 0x8020d8e0;
            }
            0x8020d8e0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d8e8;
            }
            0x8020d8e8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                if true { return; }
                return;
            }
            _ => panic!("f_8020d8d8: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetBuffDataPtr (0x8020d8f0, 24 bytes)
#[inline(never)]
pub fn f_8020d8f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d8f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d8f0); f_8020d8f0_b(c, 0x8020d8f0); crate::prof::exit(c, 0x8020d8f0); return; }
    }
    f_8020d8f0_b(c, 0x8020d8f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d8f0_at(c: &mut Cpu, pc: u32) { f_8020d8f0_b(c, pc) }
#[inline(always)]
fn f_8020d8f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d8f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d8f0, pc); }
        match pc {
            0x8020d8f0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d900; continue; }
                pc = 0x8020d8f8;
            }
            0x8020d8f8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d900;
            }
            0x8020d900 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                if true { return; }
                return;
            }
            _ => panic!("f_8020d8f0: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetActorFightTargetPtr (0x8020d908, 24 bytes)
#[inline(never)]
pub fn f_8020d908(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d908) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d908); f_8020d908_b(c, 0x8020d908); crate::prof::exit(c, 0x8020d908); return; }
    }
    f_8020d908_b(c, 0x8020d908)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d908_at(c: &mut Cpu, pc: u32) { f_8020d908_b(c, pc) }
#[inline(always)]
fn f_8020d908_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d908;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d908, pc); }
        match pc {
            0x8020d908 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d918; continue; }
                pc = 0x8020d910;
            }
            0x8020d910 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d918;
            }
            0x8020d918 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                if true { return; }
                return;
            }
            _ => panic!("f_8020d908: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetFightActionDataPtr (0x8020d920, 24 bytes)
#[inline(never)]
pub fn f_8020d920(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d920) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d920); f_8020d920_b(c, 0x8020d920); crate::prof::exit(c, 0x8020d920); return; }
    }
    f_8020d920_b(c, 0x8020d920)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d920_at(c: &mut Cpu, pc: u32) { f_8020d920_b(c, pc) }
#[inline(always)]
fn f_8020d920_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d920;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d920, pc); }
        match pc {
            0x8020d920 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d930; continue; }
                pc = 0x8020d928;
            }
            0x8020d928 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d930;
            }
            0x8020d930 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8020d920: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetBuff (0x8020d938, 24 bytes)
#[inline(never)]
pub fn f_8020d938(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d938) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d938); f_8020d938_b(c, 0x8020d938); crate::prof::exit(c, 0x8020d938); return; }
    }
    f_8020d938_b(c, 0x8020d938)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d938_at(c: &mut Cpu, pc: u32) { f_8020d938_b(c, pc) }
#[inline(always)]
fn f_8020d938_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d938;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d938, pc); }
        match pc {
            0x8020d938 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d948; continue; }
                pc = 0x8020d940;
            }
            0x8020d940 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d948;
            }
            0x8020d948 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020d938: bad pc {:#x}", pc),
        }
    }
}

/// fightActionBiosGetKind (0x8020d950, 24 bytes)
#[inline(never)]
pub fn f_8020d950(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d950) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d950); f_8020d950_b(c, 0x8020d950); crate::prof::exit(c, 0x8020d950); return; }
    }
    f_8020d950_b(c, 0x8020d950)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d950_at(c: &mut Cpu, pc: u32) { f_8020d950_b(c, pc) }
#[inline(always)]
fn f_8020d950_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d950;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d950, pc); }
        match pc {
            0x8020d950 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d960; continue; }
                pc = 0x8020d958;
            }
            0x8020d958 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d960;
            }
            0x8020d960 => {
                c.r[3] = c.lhz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d950: bad pc {:#x}", pc),
        }
    }
}

/// fn_8020D968 (0x8020d968, 56 bytes)
#[inline(never)]
pub fn f_8020d968(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d968) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d968); f_8020d968_b(c, 0x8020d968); crate::prof::exit(c, 0x8020d968); return; }
    }
    f_8020d968_b(c, 0x8020d968)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d968_at(c: &mut Cpu, pc: u32) { f_8020d968_b(c, pc) }
#[inline(always)]
fn f_8020d968_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d968;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d968, pc); }
        match pc {
            0x8020d968 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d970;
            }
            0x8020d970 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020d978;
            }
            0x8020d978 => {
                c.r[0] = 0x6;
                c.r[5] = c.r[3].wrapping_add(0xfffffffc);
                c.r[4] = c.r[4].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x8020d988;
            }
            0x8020d988 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8020d988; continue; }
                pc = 0x8020d99c;
            }
            0x8020d99c => {
                if true { return; }
                return;
            }
            _ => panic!("f_8020d968: bad pc {:#x}", pc),
        }
    }
}

/// fightKoukaDataBiosGetKoukaDataId (0x8020d9a0, 24 bytes)
#[inline(never)]
pub fn f_8020d9a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d9a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d9a0); f_8020d9a0_b(c, 0x8020d9a0); crate::prof::exit(c, 0x8020d9a0); return; }
    }
    f_8020d9a0_b(c, 0x8020d9a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d9a0_at(c: &mut Cpu, pc: u32) { f_8020d9a0_b(c, pc) }
#[inline(always)]
fn f_8020d9a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d9a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d9a0, pc); }
        match pc {
            0x8020d9a0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d9b0; continue; }
                pc = 0x8020d9a8;
            }
            0x8020d9a8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d9b0;
            }
            0x8020d9b0 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020d9a0: bad pc {:#x}", pc),
        }
    }
}

/// fightKoukaDataBiosGetFightTargetDataId (0x8020d9b8, 24 bytes)
#[inline(never)]
pub fn f_8020d9b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d9b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d9b8); f_8020d9b8_b(c, 0x8020d9b8); crate::prof::exit(c, 0x8020d9b8); return; }
    }
    f_8020d9b8_b(c, 0x8020d9b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d9b8_at(c: &mut Cpu, pc: u32) { f_8020d9b8_b(c, pc) }
#[inline(always)]
fn f_8020d9b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d9b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d9b8, pc); }
        match pc {
            0x8020d9b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d9c8; continue; }
                pc = 0x8020d9c0;
            }
            0x8020d9c0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d9c8;
            }
            0x8020d9c8 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8020d9b8: bad pc {:#x}", pc),
        }
    }
}

/// fightKoukaDataBiosGetFightJoukenDataId (0x8020d9d0, 24 bytes)
#[inline(never)]
pub fn f_8020d9d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d9d0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d9d0); f_8020d9d0_b(c, 0x8020d9d0); crate::prof::exit(c, 0x8020d9d0); return; }
    }
    f_8020d9d0_b(c, 0x8020d9d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d9d0_at(c: &mut Cpu, pc: u32) { f_8020d9d0_b(c, pc) }
#[inline(always)]
fn f_8020d9d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d9d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d9d0, pc); }
        match pc {
            0x8020d9d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020d9e0; continue; }
                pc = 0x8020d9d8;
            }
            0x8020d9d8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020d9e0;
            }
            0x8020d9e0 => {
                c.r[3] = c.lhz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d9d0: bad pc {:#x}", pc),
        }
    }
}

/// fightKoukaDataBiosGetPtr (0x8020d9e8, 44 bytes)
#[inline(never)]
pub fn f_8020d9e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020d9e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020d9e8); f_8020d9e8_b(c, 0x8020d9e8); crate::prof::exit(c, 0x8020d9e8); return; }
    }
    f_8020d9e8_b(c, 0x8020d9e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020d9e8_at(c: &mut Cpu, pc: u32) { f_8020d9e8_b(c, pc) }
#[inline(always)]
fn f_8020d9e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020d9e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020d9e8, pc); }
        match pc {
            0x8020d9e8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8530));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020da00; continue; }
                pc = 0x8020d9f8;
            }
            0x8020d9f8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020da00;
            }
            0x8020da00 => {
                c.r[4] = c.r[3].wrapping_mul(0x6);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5cb8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020d9e8: bad pc {:#x}", pc),
        }
    }
}

/// fightKoukaDoFightKoukaJoukenAndKouka (0x8020da14, 188 bytes)
#[inline(never)]
pub fn f_8020da14(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020da14) { return; }
        if c.prof { crate::prof::enter(c, 0x8020da14); f_8020da14_b(c, 0x8020da14); crate::prof::exit(c, 0x8020da14); return; }
    }
    f_8020da14_b(c, 0x8020da14)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020da14_at(c: &mut Cpu, pc: u32) { f_8020da14_b(c, pc) }
#[inline(always)]
fn f_8020da14_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020da14;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020da14, pc); }
        match pc {
            0x8020da14 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020da34; f_8020d9e8(c);
                pc = 0x8020da34;
            }
            0x8020da34 => {
                c.lr = 0x8020da38; f_8020d9d0(c);
                pc = 0x8020da38;
            }
            0x8020da38 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020da44; f_8020d9e8(c);
                pc = 0x8020da44;
            }
            0x8020da44 => {
                c.lr = 0x8020da48; f_8020d9b8(c);
                pc = 0x8020da48;
            }
            0x8020da48 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020da54; f_8020d9e8(c);
                pc = 0x8020da54;
            }
            0x8020da54 => {
                c.lr = 0x8020da58; f_8020d9a0(c);
                pc = 0x8020da58;
            }
            0x8020da58 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[29] = 0x0;
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x8020da74; f_801f54a4(c);
                pc = 0x8020da74;
            }
            0x8020da74 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8020da84; f_801f02ac(c);
                pc = 0x8020da84;
            }
            0x8020da84 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8020da94; f_8020a8e0(c);
                pc = 0x8020da94;
            }
            0x8020da94 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8020dab8; continue; }
                pc = 0x8020daa0;
            }
            0x8020daa0 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[27] | c.r[27];
                c.r[3] = c.r[31].rotate_left(0) & 0xffff;
                c.r[6] = 0x0;
                c.lr = 0x8020dab4; f_80136078(c);
                pc = 0x8020dab4;
            }
            0x8020dab4 => {
                c.r[29] = 0x1;
                pc = 0x8020dab8;
            }
            0x8020dab8 => {
                c.r[3] = c.r[29] | c.r[29];
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020da14: bad pc {:#x}", pc),
        }
    }
}

/// fightStartEncount (0x8020dad0, 628 bytes)
#[inline(never)]
pub fn f_8020dad0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020dad0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dad0); f_8020dad0_b(c, 0x8020dad0); crate::prof::exit(c, 0x8020dad0); return; }
    }
    f_8020dad0_b(c, 0x8020dad0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dad0_at(c: &mut Cpu, pc: u32) { f_8020dad0_b(c, pc) }
#[inline(always)]
fn f_8020dad0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020dad0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020dad0, pc); }
        match pc {
            0x8020dad0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x8020daf4; continue; }
                pc = 0x8020daec;
            }
            0x8020daec => {
                c.r[3] = 0x0;
                pc = 0x8020dd30; continue;
                pc = 0x8020daf4;
            }
            0x8020daf4 => {
                c.lr = 0x8020daf8; f_8020e0f8(c);
                pc = 0x8020daf8;
            }
            0x8020daf8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[29] = c.r[0] | c.r[0];
                c.lr = 0x8020db08; f_801ef62c(c);
                pc = 0x8020db08;
            }
            0x8020db08 => {
                c.r[3] = 0x9b0;
                c.lr = 0x8020db10; f_801903b0(c);
                pc = 0x8020db10;
            }
            0x8020db10 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020db18; f_801ef61c(c);
                pc = 0x8020db18;
            }
            0x8020db18 => {
                c.lr = 0x8020db1c; f_800ff56c(c);
                pc = 0x8020db1c;
            }
            0x8020db1c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[7] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.r[5] = 0x4a;
                c.r[6] = 0x0;
                c.lr = 0x8020db38; f_801f4c14(c);
                pc = 0x8020db38;
            }
            0x8020db38 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020db40; f_8020e0b0(c);
                pc = 0x8020db40;
            }
            0x8020db40 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.lr = 0x8020db58; f_801f54a4(c);
                pc = 0x8020db58;
            }
            0x8020db58 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffff;
                c.lr = 0x8020db60; f_801d23c0(c);
                pc = 0x8020db60;
            }
            0x8020db60 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020db68; f_8020e0e0(c);
                pc = 0x8020db68;
            }
            0x8020db68 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.lr = 0x8020db70; f_8020e488(c);
                pc = 0x8020db70;
            }
            0x8020db70 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020dbcc; continue; }
                pc = 0x8020db78;
            }
            0x8020db78 => {
                c.lr = 0x8020db7c; f_8020e260(c);
                pc = 0x8020db7c;
            }
            0x8020db7c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020dbcc; continue; }
                pc = 0x8020db84;
            }
            0x8020db84 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x1;
                c.lr = 0x8020db90; f_8020e068(c);
                pc = 0x8020db90;
            }
            0x8020db90 => {
                c.lr = 0x8020db94; f_801fccc4(c);
                pc = 0x8020db94;
            }
            0x8020db94 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020dbcc; continue; }
                pc = 0x8020db9c;
            }
            0x8020db9c => {
                c.lr = 0x8020dba0; f_801fcc7c(c);
                pc = 0x8020dba0;
            }
            0x8020dba0 => {
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8020dba8; f_80261388(c);
                pc = 0x8020dba8;
            }
            0x8020dba8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020dbc0; continue; }
                pc = 0x8020dbb0;
            }
            0x8020dbb0 => {
                c.lr = 0x8020dbb4; f_80261444(c);
                pc = 0x8020dbb4;
            }
            0x8020dbb4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8020dbc0; continue; }
                pc = 0x8020dbbc;
            }
            0x8020dbbc => {
                c.lr = 0x8020dbc0; f_80190528(c);
                pc = 0x8020dbc0;
            }
            0x8020dbc0 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x59;
                c.lr = 0x8020dbcc; f_80132a38(c);
                pc = 0x8020dbcc;
            }
            0x8020dbcc => {
                c.r[3] = 0x1;
                c.r[4] = 0x3e8;
                c.r[5] = 0xff;
                c.lr = 0x8020dbdc; f_80165a20(c);
                pc = 0x8020dbdc;
            }
            0x8020dbdc => {
                c.r[3] = 0x3e8;
                c.lr = 0x8020dbe4; f_801657d0(c);
                pc = 0x8020dbe4;
            }
            0x8020dbe4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020dbec; f_8020dff0(c);
                pc = 0x8020dbec;
            }
            0x8020dbec => {
                c.lr = 0x8020dbf0; f_8020deb0(c);
                pc = 0x8020dbf0;
            }
            0x8020dbf0 => {
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8020dbf8; f_8020de80(c);
                pc = 0x8020dbf8;
            }
            0x8020dbf8 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020dc04; f_8020de98(c);
                pc = 0x8020dc04;
            }
            0x8020dc04 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020dc10; f_8020de68(c);
                pc = 0x8020dc10;
            }
            0x8020dc10 => {
                c.f[2] = c.f[1];
                { let ea = c.r[2].wrapping_add(0xffffae88); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[30] | c.r[30];
                c.r[3] = 0x9;
                c.lr = 0x8020dc28; f_801c4164(c);
                pc = 0x8020dc28;
            }
            0x8020dc28 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8020dc30; f_8020de50(c);
                pc = 0x8020dc30;
            }
            0x8020dc30 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8020dc44; continue; }
                pc = 0x8020dc38;
            }
            0x8020dc38 => {
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8020dc44; f_80166ab8(c);
                pc = 0x8020dc44;
            }
            0x8020dc44 => {
                c.lr = 0x8020dc48; f_801ef7b4(c);
                pc = 0x8020dc48;
            }
            0x8020dc48 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020dc50; f_800ff730(c);
                pc = 0x8020dc50;
            }
            0x8020dc50 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8020dc5c; f_8011288c(c);
                pc = 0x8020dc5c;
            }
            0x8020dc5c => {
                c.lr = 0x8020dc60; crate::stubs::f_800f0308(c);
                pc = 0x8020dc60;
            }
            0x8020dc60 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8020dc68; f_8011395c(c);
                pc = 0x8020dc68;
            }
            0x8020dc68 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8020dc70; f_8020dfd8(c);
                pc = 0x8020dc70;
            }
            0x8020dc70 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020dcf0; continue; }
                pc = 0x8020dc78;
            }
            0x8020dc78 => {
                c.lr = 0x8020dc7c; f_801ef634(c);
                pc = 0x8020dc7c;
            }
            0x8020dc7c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8020dc88; f_801f1dbc(c);
                pc = 0x8020dc88;
            }
            0x8020dc88 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020dcf0; continue; }
                pc = 0x8020dc90;
            }
            0x8020dc90 => {
                c.r[3] = 0x0;
                c.lr = 0x8020dc98; f_801ef61c(c);
                pc = 0x8020dc98;
            }
            0x8020dc98 => {
                c.r[3] = 0xe05;
                c.lr = 0x8020dca0; f_801903b0(c);
                pc = 0x8020dca0;
            }
            0x8020dca0 => {
                c.r[3] = 0x0;
                c.r[4] = 0xc;
                c.r[5] = 0x0;
                c.lr = 0x8020dcb0; f_8012a5b0(c);
                pc = 0x8020dcb0;
            }
            0x8020dcb0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[4] = c.addc(c.r[0], 0, ci); }
                c.lr = 0x8020dcc4; f_80129474(c);
                pc = 0x8020dcc4;
            }
            0x8020dcc4 => {
                c.r[3] = 0x1;
                c.lr = 0x8020dccc; f_801d0afc(c);
                pc = 0x8020dccc;
            }
            0x8020dccc => {
                c.lr = 0x8020dcd0; f_8018da88(c);
                pc = 0x8020dcd0;
            }
            0x8020dcd0 => {
                c.lr = 0x8020dcd4; f_80113fe8(c);
                pc = 0x8020dcd4;
            }
            0x8020dcd4 => {
                c.r[4] = 0x5960000;
                c.r[3] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x8);
                c.lr = 0x8020dce4; f_8011288c(c);
                pc = 0x8020dce4;
            }
            0x8020dce4 => {
                c.lr = 0x8020dce8; crate::stubs::f_800f0308(c);
                pc = 0x8020dce8;
            }
            0x8020dce8 => {
                c.lr = 0x8020dcec; f_801ef634(c);
                pc = 0x8020dcec;
            }
            0x8020dcec => {
                pc = 0x8020dd30; continue;
                pc = 0x8020dcf0;
            }
            0x8020dcf0 => {
                c.r[3] = 0x9b0;
                c.lr = 0x8020dcf8; f_80190528(c);
                pc = 0x8020dcf8;
            }
            0x8020dcf8 => {
                c.lr = 0x8020dcfc; f_80112700(c);
                pc = 0x8020dcfc;
            }
            0x8020dcfc => {
                c.lr = 0x8020dd00; f_801140c8(c);
                pc = 0x8020dd00;
            }
            0x8020dd00 => {
                c.r[3] = 0xe05;
                c.lr = 0x8020dd08; f_801902e0(c);
                pc = 0x8020dd08;
            }
            0x8020dd08 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8020dd24; continue; }
                pc = 0x8020dd10;
            }
            0x8020dd10 => {
                { let ea = c.r[2].wrapping_add(0xffffae8c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x2;
                c.lr = 0x8020dd1c; f_801c41c8(c);
                pc = 0x8020dd1c;
            }
            0x8020dd1c => {
                c.r[3] = 0x1;
                c.lr = 0x8020dd24; f_801c40f0(c);
                pc = 0x8020dd24;
            }
            0x8020dd24 => {
                c.r[3] = 0x0;
                c.lr = 0x8020dd2c; f_801ef61c(c);
                pc = 0x8020dd2c;
            }
            0x8020dd2c => {
                c.lr = 0x8020dd30; f_801ef634(c);
                pc = 0x8020dd30;
            }
            0x8020dd30 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8020dad0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountGetEnvSndDataId (0x8020dd44, 60 bytes)
#[inline(never)]
pub fn f_8020dd44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020dd44) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dd44); f_8020dd44_b(c, 0x8020dd44); crate::prof::exit(c, 0x8020dd44); return; }
    }
    f_8020dd44_b(c, 0x8020dd44)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dd44_at(c: &mut Cpu, pc: u32) { f_8020dd44_b(c, pc) }
#[inline(always)]
fn f_8020dd44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020dd44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020dd44, pc); }
        match pc {
            0x8020dd44 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8020dd54; f_8020e0f8(c);
                pc = 0x8020dd54;
            }
            0x8020dd54 => {
                c.lr = 0x8020dd58; f_8020e0b0(c);
                pc = 0x8020dd58;
            }
            0x8020dd58 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x7;
                c.r[6] = 0x0;
                c.lr = 0x8020dd70; f_801f54a4(c);
                pc = 0x8020dd70;
            }
            0x8020dd70 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020dd44: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountGetBgmSndDataId (0x8020dd80, 208 bytes)
#[inline(never)]
pub fn f_8020dd80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020dd80) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dd80); f_8020dd80_b(c, 0x8020dd80); crate::prof::exit(c, 0x8020dd80); return; }
    }
    f_8020dd80_b(c, 0x8020dd80)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dd80_at(c: &mut Cpu, pc: u32) { f_8020dd80_b(c, pc) }
#[inline(always)]
fn f_8020dd80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020dd80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020dd80, pc); }
        match pc {
            0x8020dd80 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x8020dd98; f_8020e0f8(c);
                pc = 0x8020dd98;
            }
            0x8020dd98 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8020dda0; f_8020e008(c);
                pc = 0x8020dda0;
            }
            0x8020dda0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020ddac; continue; }
                pc = 0x8020dda8;
            }
            0x8020dda8 => {
                pc = 0x8020de3c; continue;
                pc = 0x8020ddac;
            }
            0x8020ddac => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8020ddb4; f_8020e0f8(c);
                pc = 0x8020ddb4;
            }
            0x8020ddb4 => {
                c.lr = 0x8020ddb8; f_8020e0b0(c);
                pc = 0x8020ddb8;
            }
            0x8020ddb8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[4] = c.r[0] | c.r[0];
                c.r[5] = 0x6;
                c.r[6] = 0x0;
                c.lr = 0x8020ddd0; f_801f54a4(c);
                pc = 0x8020ddd0;
            }
            0x8020ddd0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020dddc; continue; }
                pc = 0x8020ddd8;
            }
            0x8020ddd8 => {
                pc = 0x8020de3c; continue;
                pc = 0x8020dddc;
            }
            0x8020dddc => {
                c.r[30] = 0x0;
                pc = 0x8020de2c; continue;
                pc = 0x8020dde4;
            }
            0x8020dde4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                c.lr = 0x8020ddf0; f_8020e068(c);
                pc = 0x8020ddf0;
            }
            0x8020ddf0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8020de28; continue; }
                pc = 0x8020ddf8;
            }
            0x8020ddf8 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.lr = 0x8020de0c; f_801fb1c0(c);
                pc = 0x8020de0c;
            }
            0x8020de0c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8020de28; continue; }
                pc = 0x8020de14;
            }
            0x8020de14 => {
                c.lr = 0x8020de18; f_801fbd58(c);
                pc = 0x8020de18;
            }
            0x8020de18 => {
                c.lr = 0x8020de1c; f_801fbd10(c);
                pc = 0x8020de1c;
            }
            0x8020de1c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8020de28; continue; }
                pc = 0x8020de24;
            }
            0x8020de24 => {
                pc = 0x8020de3c; continue;
                pc = 0x8020de28;
            }
            0x8020de28 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x8020de2c;
            }
            0x8020de2c => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8020dde4; continue; }
                pc = 0x8020de38;
            }
            0x8020de38 => {
                c.r[3] = 0x1;
                pc = 0x8020de3c;
            }
            0x8020de3c => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 30..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8020dd80: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetWipeEffectSndID (0x8020de50, 24 bytes)
#[inline(never)]
pub fn f_8020de50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020de50) { return; }
        if c.prof { crate::prof::enter(c, 0x8020de50); f_8020de50_b(c, 0x8020de50); crate::prof::exit(c, 0x8020de50); return; }
    }
    f_8020de50_b(c, 0x8020de50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020de50_at(c: &mut Cpu, pc: u32) { f_8020de50_b(c, pc) }
#[inline(always)]
fn f_8020de50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020de50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020de50, pc); }
        match pc {
            0x8020de50 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020de60; continue; }
                pc = 0x8020de58;
            }
            0x8020de58 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020de60;
            }
            0x8020de60 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8020de50: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetWipeEffectTime (0x8020de68, 24 bytes)
#[inline(never)]
pub fn f_8020de68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020de68) { return; }
        if c.prof { crate::prof::enter(c, 0x8020de68); f_8020de68_b(c, 0x8020de68); crate::prof::exit(c, 0x8020de68); return; }
    }
    f_8020de68_b(c, 0x8020de68)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020de68_at(c: &mut Cpu, pc: u32) { f_8020de68_b(c, pc) }
#[inline(always)]
fn f_8020de68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020de68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020de68, pc); }
        match pc {
            0x8020de68 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020de78; continue; }
                pc = 0x8020de70;
            }
            0x8020de70 => {
                { let ea = c.r[2].wrapping_add(0xffffae90); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { return; }
                pc = 0x8020de78;
            }
            0x8020de78 => {
                { let ea = c.r[3].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { return; }
                return;
            }
            _ => panic!("f_8020de68: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetWipeSnapshotUse (0x8020de80, 24 bytes)
#[inline(never)]
pub fn f_8020de80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020de80) { return; }
        if c.prof { crate::prof::enter(c, 0x8020de80); f_8020de80_b(c, 0x8020de80); crate::prof::exit(c, 0x8020de80); return; }
    }
    f_8020de80_b(c, 0x8020de80)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020de80_at(c: &mut Cpu, pc: u32) { f_8020de80_b(c, pc) }
#[inline(always)]
fn f_8020de80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020de80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020de80, pc); }
        match pc {
            0x8020de80 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020de90; continue; }
                pc = 0x8020de88;
            }
            0x8020de88 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020de90;
            }
            0x8020de90 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020de80: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetWipeFunction (0x8020de98, 24 bytes)
#[inline(never)]
pub fn f_8020de98(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020de98) { return; }
        if c.prof { crate::prof::enter(c, 0x8020de98); f_8020de98_b(c, 0x8020de98); crate::prof::exit(c, 0x8020de98); return; }
    }
    f_8020de98_b(c, 0x8020de98)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020de98_at(c: &mut Cpu, pc: u32) { f_8020de98_b(c, pc) }
#[inline(always)]
fn f_8020de98_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020de98;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020de98, pc); }
        match pc {
            0x8020de98 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020dea8; continue; }
                pc = 0x8020dea0;
            }
            0x8020dea0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020dea8;
            }
            0x8020dea8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8020de98: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountWipeDataBiosGetPtr (0x8020deb0, 40 bytes)
#[inline(never)]
pub fn f_8020deb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020deb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020deb0); f_8020deb0_b(c, 0x8020deb0); crate::prof::exit(c, 0x8020deb0); return; }
    }
    f_8020deb0_b(c, 0x8020deb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020deb0_at(c: &mut Cpu, pc: u32) { f_8020deb0_b(c, pc) }
#[inline(always)]
fn f_8020deb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020deb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020deb0, pc); }
        match pc {
            0x8020deb0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8500));
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020dec4; continue; }
                pc = 0x8020debc;
            }
            0x8020debc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020dec4;
            }
            0x8020dec4 => {
                c.r[4] = c.r[3].wrapping_mul(0xc);
                c.r[3] = 0x80370000;
                c.r[0] = c.r[3].wrapping_add(0x5980);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020deb0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetFightName (0x8020ded8, 24 bytes)
#[inline(never)]
pub fn f_8020ded8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020ded8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020ded8); f_8020ded8_b(c, 0x8020ded8); crate::prof::exit(c, 0x8020ded8); return; }
    }
    f_8020ded8_b(c, 0x8020ded8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020ded8_at(c: &mut Cpu, pc: u32) { f_8020ded8_b(c, pc) }
#[inline(always)]
fn f_8020ded8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020ded8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020ded8, pc); }
        match pc {
            0x8020ded8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020dee8; continue; }
                pc = 0x8020dee0;
            }
            0x8020dee0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020dee8;
            }
            0x8020dee8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8020ded8: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetSyoukaiWzxDataId (0x8020def0, 16 bytes)
#[inline(never)]
pub fn f_8020def0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020def0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020def0); f_8020def0_b(c, 0x8020def0); crate::prof::exit(c, 0x8020def0); return; }
    }
    f_8020def0_b(c, 0x8020def0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020def0_at(c: &mut Cpu, pc: u32) { f_8020def0_b(c, pc) }
#[inline(always)]
fn f_8020def0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020def0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020def0, pc); }
        match pc {
            0x8020def0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020def8;
            }
            0x8020def8 => {
                c.stw(c.r[3].wrapping_add(0x14), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020def0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetBgmSndId (0x8020df00, 16 bytes)
#[inline(never)]
pub fn f_8020df00(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020df00) { return; }
        if c.prof { crate::prof::enter(c, 0x8020df00); f_8020df00_b(c, 0x8020df00); crate::prof::exit(c, 0x8020df00); return; }
    }
    f_8020df00_b(c, 0x8020df00)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020df00_at(c: &mut Cpu, pc: u32) { f_8020df00_b(c, pc) }
#[inline(always)]
fn f_8020df00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020df00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020df00, pc); }
        match pc {
            0x8020df00 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020df08;
            }
            0x8020df08 => {
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020df00: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetGSInputDevice (0x8020df10, 64 bytes)
#[inline(never)]
pub fn f_8020df10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020df10) { return; }
        if c.prof { crate::prof::enter(c, 0x8020df10); f_8020df10_b(c, 0x8020df10); crate::prof::exit(c, 0x8020df10); return; }
    }
    f_8020df10_b(c, 0x8020df10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020df10_at(c: &mut Cpu, pc: u32) { f_8020df10_b(c, pc) }
#[inline(always)]
fn f_8020df10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020df10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020df10, pc); }
        match pc {
            0x8020df10 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020df20; continue; }
                pc = 0x8020df18;
            }
            0x8020df18 => {
                c.r[4] = 0x0;
                pc = 0x8020df40; continue;
                pc = 0x8020df20;
            }
            0x8020df20 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8020df34; continue; }
                pc = 0x8020df2c;
            }
            0x8020df2c => {
                c.r[4] = 0x0;
                pc = 0x8020df40; continue;
                pc = 0x8020df34;
            }
            0x8020df34 => {
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x18);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8020df40;
            }
            0x8020df40 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020df48;
            }
            0x8020df48 => {
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020df10: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetFightTrainerDataId (0x8020df50, 64 bytes)
#[inline(never)]
pub fn f_8020df50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020df50) { return; }
        if c.prof { crate::prof::enter(c, 0x8020df50); f_8020df50_b(c, 0x8020df50); crate::prof::exit(c, 0x8020df50); return; }
    }
    f_8020df50_b(c, 0x8020df50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020df50_at(c: &mut Cpu, pc: u32) { f_8020df50_b(c, pc) }
#[inline(always)]
fn f_8020df50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020df50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020df50, pc); }
        match pc {
            0x8020df50 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020df60; continue; }
                pc = 0x8020df58;
            }
            0x8020df58 => {
                c.r[4] = 0x0;
                pc = 0x8020df80; continue;
                pc = 0x8020df60;
            }
            0x8020df60 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8020df74; continue; }
                pc = 0x8020df6c;
            }
            0x8020df6c => {
                c.r[4] = 0x0;
                pc = 0x8020df80; continue;
                pc = 0x8020df74;
            }
            0x8020df74 => {
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x18);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8020df80;
            }
            0x8020df80 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020df88;
            }
            0x8020df88 => {
                c.sth(c.r[4], c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020df50: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetFightFloorDataId (0x8020df90, 16 bytes)
#[inline(never)]
pub fn f_8020df90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020df90) { return; }
        if c.prof { crate::prof::enter(c, 0x8020df90); f_8020df90_b(c, 0x8020df90); crate::prof::exit(c, 0x8020df90); return; }
    }
    f_8020df90_b(c, 0x8020df90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020df90_at(c: &mut Cpu, pc: u32) { f_8020df90_b(c, pc) }
#[inline(always)]
fn f_8020df90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020df90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020df90, pc); }
        match pc {
            0x8020df90 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020df98;
            }
            0x8020df98 => {
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020df90: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetTrainer (0x8020dfa0, 16 bytes)
#[inline(never)]
pub fn f_8020dfa0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020dfa0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dfa0); f_8020dfa0_b(c, 0x8020dfa0); crate::prof::exit(c, 0x8020dfa0); return; }
    }
    f_8020dfa0_b(c, 0x8020dfa0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dfa0_at(c: &mut Cpu, pc: u32) { f_8020dfa0_b(c, pc) }
#[inline(always)]
fn f_8020dfa0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020dfa0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020dfa0, pc); }
        match pc {
            0x8020dfa0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020dfa8;
            }
            0x8020dfa8 => {
                c.stb(c.r[3].wrapping_add(0x1), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020dfa0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosSetFightKind (0x8020dfb0, 16 bytes)
#[inline(never)]
pub fn f_8020dfb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020dfb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dfb0); f_8020dfb0_b(c, 0x8020dfb0); crate::prof::exit(c, 0x8020dfb0); return; }
    }
    f_8020dfb0_b(c, 0x8020dfb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dfb0_at(c: &mut Cpu, pc: u32) { f_8020dfb0_b(c, pc) }
#[inline(always)]
fn f_8020dfb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020dfb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020dfb0, pc); }
        match pc {
            0x8020dfb0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8020dfb8;
            }
            0x8020dfb8 => {
                c.stb(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020dfb0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetSyoukaiWzxDataId (0x8020dfc0, 24 bytes)
#[inline(never)]
pub fn f_8020dfc0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020dfc0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dfc0); f_8020dfc0_b(c, 0x8020dfc0); crate::prof::exit(c, 0x8020dfc0); return; }
    }
    f_8020dfc0_b(c, 0x8020dfc0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dfc0_at(c: &mut Cpu, pc: u32) { f_8020dfc0_b(c, pc) }
#[inline(always)]
fn f_8020dfc0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020dfc0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020dfc0, pc); }
        match pc {
            0x8020dfc0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020dfd0; continue; }
                pc = 0x8020dfc8;
            }
            0x8020dfc8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020dfd0;
            }
            0x8020dfd0 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                if true { return; }
                return;
            }
            _ => panic!("f_8020dfc0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetZenmetuFlag (0x8020dfd8, 24 bytes)
#[inline(never)]
pub fn f_8020dfd8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020dfd8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dfd8); f_8020dfd8_b(c, 0x8020dfd8); crate::prof::exit(c, 0x8020dfd8); return; }
    }
    f_8020dfd8_b(c, 0x8020dfd8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dfd8_at(c: &mut Cpu, pc: u32) { f_8020dfd8_b(c, pc) }
#[inline(always)]
fn f_8020dfd8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020dfd8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020dfd8, pc); }
        match pc {
            0x8020dfd8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020dfe8; continue; }
                pc = 0x8020dfe0;
            }
            0x8020dfe0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020dfe8;
            }
            0x8020dfe8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8020dfd8: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetWipeId (0x8020dff0, 24 bytes)
#[inline(never)]
pub fn f_8020dff0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020dff0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020dff0); f_8020dff0_b(c, 0x8020dff0); crate::prof::exit(c, 0x8020dff0); return; }
    }
    f_8020dff0_b(c, 0x8020dff0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020dff0_at(c: &mut Cpu, pc: u32) { f_8020dff0_b(c, pc) }
#[inline(always)]
fn f_8020dff0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020dff0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020dff0, pc); }
        match pc {
            0x8020dff0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e000; continue; }
                pc = 0x8020dff8;
            }
            0x8020dff8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e000;
            }
            0x8020e000 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                if true { return; }
                return;
            }
            _ => panic!("f_8020dff0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetBgmSndId (0x8020e008, 24 bytes)
#[inline(never)]
pub fn f_8020e008(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e008) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e008); f_8020e008_b(c, 0x8020e008); crate::prof::exit(c, 0x8020e008); return; }
    }
    f_8020e008_b(c, 0x8020e008)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e008_at(c: &mut Cpu, pc: u32) { f_8020e008_b(c, pc) }
#[inline(always)]
fn f_8020e008_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e008;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e008, pc); }
        match pc {
            0x8020e008 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e018; continue; }
                pc = 0x8020e010;
            }
            0x8020e010 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e018;
            }
            0x8020e018 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e008: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetGSInputDevice (0x8020e020, 72 bytes)
#[inline(never)]
pub fn f_8020e020(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e020) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e020); f_8020e020_b(c, 0x8020e020); crate::prof::exit(c, 0x8020e020); return; }
    }
    f_8020e020_b(c, 0x8020e020)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e020_at(c: &mut Cpu, pc: u32) { f_8020e020_b(c, pc) }
#[inline(always)]
fn f_8020e020_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e020;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e020, pc); }
        match pc {
            0x8020e020 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e030; continue; }
                pc = 0x8020e028;
            }
            0x8020e028 => {
                c.r[4] = 0x0;
                pc = 0x8020e050; continue;
                pc = 0x8020e030;
            }
            0x8020e030 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8020e044; continue; }
                pc = 0x8020e03c;
            }
            0x8020e03c => {
                c.r[4] = 0x0;
                pc = 0x8020e050; continue;
                pc = 0x8020e044;
            }
            0x8020e044 => {
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x18);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8020e050;
            }
            0x8020e050 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x8020e060; continue; }
                pc = 0x8020e058;
            }
            0x8020e058 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e060;
            }
            0x8020e060 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e020: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetFightTrainerDataId (0x8020e068, 72 bytes)
#[inline(never)]
pub fn f_8020e068(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e068) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e068); f_8020e068_b(c, 0x8020e068); crate::prof::exit(c, 0x8020e068); return; }
    }
    f_8020e068_b(c, 0x8020e068)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e068_at(c: &mut Cpu, pc: u32) { f_8020e068_b(c, pc) }
#[inline(always)]
fn f_8020e068_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e068;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e068, pc); }
        match pc {
            0x8020e068 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e078; continue; }
                pc = 0x8020e070;
            }
            0x8020e070 => {
                c.r[4] = 0x0;
                pc = 0x8020e098; continue;
                pc = 0x8020e078;
            }
            0x8020e078 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(0) { pc = 0x8020e08c; continue; }
                pc = 0x8020e084;
            }
            0x8020e084 => {
                c.r[4] = 0x0;
                pc = 0x8020e098; continue;
                pc = 0x8020e08c;
            }
            0x8020e08c => {
                c.r[4] = c.r[4].rotate_left(3) & 0x7f8;
                c.r[4] = c.r[4].wrapping_add(0x18);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8020e098;
            }
            0x8020e098 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x8020e0a8; continue; }
                pc = 0x8020e0a0;
            }
            0x8020e0a0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e0a8;
            }
            0x8020e0a8 => {
                c.r[3] = c.lhz(c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e068: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetFightFloorDataId (0x8020e0b0, 24 bytes)
#[inline(never)]
pub fn f_8020e0b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e0b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e0b0); f_8020e0b0_b(c, 0x8020e0b0); crate::prof::exit(c, 0x8020e0b0); return; }
    }
    f_8020e0b0_b(c, 0x8020e0b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e0b0_at(c: &mut Cpu, pc: u32) { f_8020e0b0_b(c, pc) }
#[inline(always)]
fn f_8020e0b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e0b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e0b0, pc); }
        match pc {
            0x8020e0b0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e0c0; continue; }
                pc = 0x8020e0b8;
            }
            0x8020e0b8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e0c0;
            }
            0x8020e0c0 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e0b0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetTrainer (0x8020e0c8, 24 bytes)
#[inline(never)]
pub fn f_8020e0c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e0c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e0c8); f_8020e0c8_b(c, 0x8020e0c8); crate::prof::exit(c, 0x8020e0c8); return; }
    }
    f_8020e0c8_b(c, 0x8020e0c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e0c8_at(c: &mut Cpu, pc: u32) { f_8020e0c8_b(c, pc) }
#[inline(always)]
fn f_8020e0c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e0c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e0c8, pc); }
        match pc {
            0x8020e0c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e0d8; continue; }
                pc = 0x8020e0d0;
            }
            0x8020e0d0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e0d8;
            }
            0x8020e0d8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e0c8: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetFightKind (0x8020e0e0, 24 bytes)
#[inline(never)]
pub fn f_8020e0e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e0e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e0e0); f_8020e0e0_b(c, 0x8020e0e0); crate::prof::exit(c, 0x8020e0e0); return; }
    }
    f_8020e0e0_b(c, 0x8020e0e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e0e0_at(c: &mut Cpu, pc: u32) { f_8020e0e0_b(c, pc) }
#[inline(always)]
fn f_8020e0e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e0e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e0e0, pc); }
        match pc {
            0x8020e0e0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e0f0; continue; }
                pc = 0x8020e0e8;
            }
            0x8020e0e8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e0f0;
            }
            0x8020e0f0 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e0e0: bad pc {:#x}", pc),
        }
    }
}

/// fightEncountDataBiosGetPtr (0x8020e0f8, 44 bytes)
#[inline(never)]
pub fn f_8020e0f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e0f8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e0f8); f_8020e0f8_b(c, 0x8020e0f8); crate::prof::exit(c, 0x8020e0f8); return; }
    }
    f_8020e0f8_b(c, 0x8020e0f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e0f8_at(c: &mut Cpu, pc: u32) { f_8020e0f8_b(c, pc) }
#[inline(always)]
fn f_8020e0f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e0f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e0f8, pc); }
        match pc {
            0x8020e0f8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8730));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8020e114; continue; }
                pc = 0x8020e10c;
            }
            0x8020e10c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e114;
            }
            0x8020e114 => {
                c.r[0] = c.r[3].wrapping_mul(0x38);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8734));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e0f8: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeGetFightSideFightOutPokemonMax (0x8020e124, 128 bytes)
#[inline(never)]
pub fn f_8020e124(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e124) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e124); f_8020e124_b(c, 0x8020e124); crate::prof::exit(c, 0x8020e124); return; }
    }
    f_8020e124_b(c, 0x8020e124)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e124_at(c: &mut Cpu, pc: u32) { f_8020e124_b(c, pc) }
#[inline(always)]
fn f_8020e124_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e124;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e124, pc); }
        match pc {
            0x8020e124 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff86e0));
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.r[6] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[7], c.r[6]);
                if !c.crbit(1) { pc = 0x8020e140; continue; }
                pc = 0x8020e138;
            }
            0x8020e138 => {
                c.r[4] = 0x0;
                pc = 0x8020e14c; continue;
                pc = 0x8020e140;
            }
            0x8020e140 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff86e4));
                c.r[0] = c.r[3].rotate_left(3) & 0x7fff8;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                pc = 0x8020e14c;
            }
            0x8020e14c => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x8020e15c; continue; }
                pc = 0x8020e154;
            }
            0x8020e154 => {
                c.r[5] = 0x0;
                pc = 0x8020e160; continue;
                pc = 0x8020e15c;
            }
            0x8020e15c => {
                c.r[5] = c.lbz(c.r[4]);
                pc = 0x8020e160;
            }
            0x8020e160 => {
                c.cmpu(0, c.r[7], c.r[6]);
                if !c.crbit(1) { pc = 0x8020e170; continue; }
                pc = 0x8020e168;
            }
            0x8020e168 => {
                c.r[4] = 0x0;
                pc = 0x8020e17c; continue;
                pc = 0x8020e170;
            }
            0x8020e170 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff86e4));
                c.r[0] = c.r[3].rotate_left(3) & 0x7fff8;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                pc = 0x8020e17c;
            }
            0x8020e17c => {
                c.cmpu(0, c.r[4], 0x0);
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if !c.crbit(2) { pc = 0x8020e190; continue; }
                pc = 0x8020e188;
            }
            0x8020e188 => {
                c.r[0] = 0x0;
                pc = 0x8020e194; continue;
                pc = 0x8020e190;
            }
            0x8020e190 => {
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x2));
                pc = 0x8020e194;
            }
            0x8020e194 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_mul(c.r[0]);
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                if true { return; }
                return;
            }
            _ => panic!("f_8020e124: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeDataBiosGetFightoutPokemonNum (0x8020e1a4, 24 bytes)
#[inline(never)]
pub fn f_8020e1a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e1a4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e1a4); f_8020e1a4_b(c, 0x8020e1a4); crate::prof::exit(c, 0x8020e1a4); return; }
    }
    f_8020e1a4_b(c, 0x8020e1a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e1a4_at(c: &mut Cpu, pc: u32) { f_8020e1a4_b(c, pc) }
#[inline(always)]
fn f_8020e1a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e1a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e1a4, pc); }
        match pc {
            0x8020e1a4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e1b4; continue; }
                pc = 0x8020e1ac;
            }
            0x8020e1ac => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e1b4;
            }
            0x8020e1b4 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e1a4: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeDataBiosGetEntryPokemonNum (0x8020e1bc, 24 bytes)
#[inline(never)]
pub fn f_8020e1bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e1bc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e1bc); f_8020e1bc_b(c, 0x8020e1bc); crate::prof::exit(c, 0x8020e1bc); return; }
    }
    f_8020e1bc_b(c, 0x8020e1bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e1bc_at(c: &mut Cpu, pc: u32) { f_8020e1bc_b(c, pc) }
#[inline(always)]
fn f_8020e1bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e1bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e1bc, pc); }
        match pc {
            0x8020e1bc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e1cc; continue; }
                pc = 0x8020e1c4;
            }
            0x8020e1c4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e1cc;
            }
            0x8020e1cc => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e1bc: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeDataBiosGetTrainerNum (0x8020e1d4, 24 bytes)
#[inline(never)]
pub fn f_8020e1d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e1d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e1d4); f_8020e1d4_b(c, 0x8020e1d4); crate::prof::exit(c, 0x8020e1d4); return; }
    }
    f_8020e1d4_b(c, 0x8020e1d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e1d4_at(c: &mut Cpu, pc: u32) { f_8020e1d4_b(c, pc) }
#[inline(always)]
fn f_8020e1d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e1d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e1d4, pc); }
        match pc {
            0x8020e1d4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e1e4; continue; }
                pc = 0x8020e1dc;
            }
            0x8020e1dc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e1e4;
            }
            0x8020e1e4 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e1d4: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeDataBiosGetName (0x8020e1ec, 24 bytes)
#[inline(never)]
pub fn f_8020e1ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e1ec) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e1ec); f_8020e1ec_b(c, 0x8020e1ec); crate::prof::exit(c, 0x8020e1ec); return; }
    }
    f_8020e1ec_b(c, 0x8020e1ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e1ec_at(c: &mut Cpu, pc: u32) { f_8020e1ec_b(c, pc) }
#[inline(always)]
fn f_8020e1ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e1ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e1ec, pc); }
        match pc {
            0x8020e1ec => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e1fc; continue; }
                pc = 0x8020e1f4;
            }
            0x8020e1f4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e1fc;
            }
            0x8020e1fc => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e1ec: bad pc {:#x}", pc),
        }
    }
}

/// fightTypeDataBiosGetPtr (0x8020e204, 44 bytes)
#[inline(never)]
pub fn f_8020e204(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e204) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e204); f_8020e204_b(c, 0x8020e204); crate::prof::exit(c, 0x8020e204); return; }
    }
    f_8020e204_b(c, 0x8020e204)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e204_at(c: &mut Cpu, pc: u32) { f_8020e204_b(c, pc) }
#[inline(always)]
fn f_8020e204_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e204;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e204, pc); }
        match pc {
            0x8020e204 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff86e0));
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[5], c.r[0]);
                if !c.crbit(1) { pc = 0x8020e220; continue; }
                pc = 0x8020e218;
            }
            0x8020e218 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e220;
            }
            0x8020e220 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff86e4));
                c.r[0] = c.r[3].rotate_left(3) & 0x7fff8;
                c.r[3] = c.r[4].wrapping_add(c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e204: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetHostEnemyMsgFlag (0x8020e230, 24 bytes)
#[inline(never)]
pub fn f_8020e230(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e230) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e230); f_8020e230_b(c, 0x8020e230); crate::prof::exit(c, 0x8020e230); return; }
    }
    f_8020e230_b(c, 0x8020e230)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e230_at(c: &mut Cpu, pc: u32) { f_8020e230_b(c, pc) }
#[inline(always)]
fn f_8020e230_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e230;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e230, pc); }
        match pc {
            0x8020e230 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e240; continue; }
                pc = 0x8020e238;
            }
            0x8020e238 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e240;
            }
            0x8020e240 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x18));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e230: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetName (0x8020e248, 24 bytes)
#[inline(never)]
pub fn f_8020e248(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e248) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e248); f_8020e248_b(c, 0x8020e248); crate::prof::exit(c, 0x8020e248); return; }
    }
    f_8020e248_b(c, 0x8020e248)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e248_at(c: &mut Cpu, pc: u32) { f_8020e248_b(c, pc) }
#[inline(always)]
fn f_8020e248_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e248;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e248, pc); }
        match pc {
            0x8020e248 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e258; continue; }
                pc = 0x8020e250;
            }
            0x8020e250 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e258;
            }
            0x8020e258 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x1c));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e248: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetPokemonStatusMenuSubbarFlag (0x8020e260, 24 bytes)
#[inline(never)]
pub fn f_8020e260(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e260) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e260); f_8020e260_b(c, 0x8020e260); crate::prof::exit(c, 0x8020e260); return; }
    }
    f_8020e260_b(c, 0x8020e260)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e260_at(c: &mut Cpu, pc: u32) { f_8020e260_b(c, pc) }
#[inline(always)]
fn f_8020e260_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e260;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e260, pc); }
        match pc {
            0x8020e260 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e270; continue; }
                pc = 0x8020e268;
            }
            0x8020e268 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e270;
            }
            0x8020e270 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x17));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e260: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDarkpokemonHypermodeFlag (0x8020e278, 24 bytes)
#[inline(never)]
pub fn f_8020e278(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e278) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e278); f_8020e278_b(c, 0x8020e278); crate::prof::exit(c, 0x8020e278); return; }
    }
    f_8020e278_b(c, 0x8020e278)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e278_at(c: &mut Cpu, pc: u32) { f_8020e278_b(c, pc) }
#[inline(always)]
fn f_8020e278_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e278;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e278, pc); }
        match pc {
            0x8020e278 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e288; continue; }
                pc = 0x8020e280;
            }
            0x8020e280 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e288;
            }
            0x8020e288 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x16));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e278: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetMonohiroiFlag (0x8020e290, 24 bytes)
#[inline(never)]
pub fn f_8020e290(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e290) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e290); f_8020e290_b(c, 0x8020e290); crate::prof::exit(c, 0x8020e290); return; }
    }
    f_8020e290_b(c, 0x8020e290)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e290_at(c: &mut Cpu, pc: u32) { f_8020e290_b(c, pc) }
#[inline(always)]
fn f_8020e290_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e290;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e290, pc); }
        match pc {
            0x8020e290 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e2a0; continue; }
                pc = 0x8020e298;
            }
            0x8020e298 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e2a0;
            }
            0x8020e2a0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x15));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e290: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDorobouFlag (0x8020e2a8, 24 bytes)
#[inline(never)]
pub fn f_8020e2a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e2a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e2a8); f_8020e2a8_b(c, 0x8020e2a8); crate::prof::exit(c, 0x8020e2a8); return; }
    }
    f_8020e2a8_b(c, 0x8020e2a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e2a8_at(c: &mut Cpu, pc: u32) { f_8020e2a8_b(c, pc) }
#[inline(always)]
fn f_8020e2a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e2a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e2a8, pc); }
        match pc {
            0x8020e2a8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e2b8; continue; }
                pc = 0x8020e2b0;
            }
            0x8020e2b0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e2b8;
            }
            0x8020e2b8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x14));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e2a8: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetBossFlag (0x8020e2c0, 24 bytes)
#[inline(never)]
pub fn f_8020e2c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e2c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e2c0); f_8020e2c0_b(c, 0x8020e2c0); crate::prof::exit(c, 0x8020e2c0); return; }
    }
    f_8020e2c0_b(c, 0x8020e2c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e2c0_at(c: &mut Cpu, pc: u32) { f_8020e2c0_b(c, pc) }
#[inline(always)]
fn f_8020e2c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e2c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e2c0, pc); }
        match pc {
            0x8020e2c0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e2d0; continue; }
                pc = 0x8020e2c8;
            }
            0x8020e2c8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e2d0;
            }
            0x8020e2d0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x13));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e2c0: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetKeikentihueruFlag (0x8020e2d8, 24 bytes)
#[inline(never)]
pub fn f_8020e2d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e2d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e2d8); f_8020e2d8_b(c, 0x8020e2d8); crate::prof::exit(c, 0x8020e2d8); return; }
    }
    f_8020e2d8_b(c, 0x8020e2d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e2d8_at(c: &mut Cpu, pc: u32) { f_8020e2d8_b(c, pc) }
#[inline(always)]
fn f_8020e2d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e2d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e2d8, pc); }
        match pc {
            0x8020e2d8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e2e8; continue; }
                pc = 0x8020e2e0;
            }
            0x8020e2e0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e2e8;
            }
            0x8020e2e8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x12));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e2d8: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoItemSoubiTokukoutokubouupFlag (0x8020e2f0, 24 bytes)
#[inline(never)]
pub fn f_8020e2f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e2f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e2f0); f_8020e2f0_b(c, 0x8020e2f0); crate::prof::exit(c, 0x8020e2f0); return; }
    }
    f_8020e2f0_b(c, 0x8020e2f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e2f0_at(c: &mut Cpu, pc: u32) { f_8020e2f0_b(c, pc) }
#[inline(always)]
fn f_8020e2f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e2f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e2f0, pc); }
        match pc {
            0x8020e2f0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e300; continue; }
                pc = 0x8020e2f8;
            }
            0x8020e2f8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e300;
            }
            0x8020e300 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x11));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e2f0: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoHizukiMiyaburiFlag (0x8020e308, 24 bytes)
#[inline(never)]
pub fn f_8020e308(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e308) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e308); f_8020e308_b(c, 0x8020e308); crate::prof::exit(c, 0x8020e308); return; }
    }
    f_8020e308_b(c, 0x8020e308)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e308_at(c: &mut Cpu, pc: u32) { f_8020e308_b(c, pc) }
#[inline(always)]
fn f_8020e308_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e308;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e308, pc); }
        match pc {
            0x8020e308 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e318; continue; }
                pc = 0x8020e310;
            }
            0x8020e310 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e318;
            }
            0x8020e318 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x10));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e308: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoHizukiAiFlag (0x8020e320, 24 bytes)
#[inline(never)]
pub fn f_8020e320(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e320) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e320); f_8020e320_b(c, 0x8020e320); crate::prof::exit(c, 0x8020e320); return; }
    }
    f_8020e320_b(c, 0x8020e320)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e320_at(c: &mut Cpu, pc: u32) { f_8020e320_b(c, pc) }
#[inline(always)]
fn f_8020e320_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e320;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e320, pc); }
        match pc {
            0x8020e320 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e330; continue; }
                pc = 0x8020e328;
            }
            0x8020e328 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e330;
            }
            0x8020e330 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xf));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e320: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoCriticalAttackFlag (0x8020e338, 24 bytes)
#[inline(never)]
pub fn f_8020e338(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e338) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e338); f_8020e338_b(c, 0x8020e338); crate::prof::exit(c, 0x8020e338); return; }
    }
    f_8020e338_b(c, 0x8020e338)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e338_at(c: &mut Cpu, pc: u32) { f_8020e338_b(c, pc) }
#[inline(always)]
fn f_8020e338_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e338;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e338, pc); }
        match pc {
            0x8020e338 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e348; continue; }
                pc = 0x8020e340;
            }
            0x8020e340 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e348;
            }
            0x8020e348 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xe));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e338: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetGetInfectPokerusFlag (0x8020e350, 24 bytes)
#[inline(never)]
pub fn f_8020e350(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e350) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e350); f_8020e350_b(c, 0x8020e350); crate::prof::exit(c, 0x8020e350); return; }
    }
    f_8020e350_b(c, 0x8020e350)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e350_at(c: &mut Cpu, pc: u32) { f_8020e350_b(c, pc) }
#[inline(always)]
fn f_8020e350_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e350;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e350, pc); }
        match pc {
            0x8020e350 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e360; continue; }
                pc = 0x8020e358;
            }
            0x8020e358 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e360;
            }
            0x8020e360 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xd));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e350: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetGetFriendFlag (0x8020e368, 24 bytes)
#[inline(never)]
pub fn f_8020e368(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e368) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e368); f_8020e368_b(c, 0x8020e368); crate::prof::exit(c, 0x8020e368); return; }
    }
    f_8020e368_b(c, 0x8020e368)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e368_at(c: &mut Cpu, pc: u32) { f_8020e368_b(c, pc) }
#[inline(always)]
fn f_8020e368_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e368;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e368, pc); }
        match pc {
            0x8020e368 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e378; continue; }
                pc = 0x8020e370;
            }
            0x8020e370 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e378;
            }
            0x8020e378 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xc));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e368: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetGetNekoniKobanFlag (0x8020e380, 24 bytes)
#[inline(never)]
pub fn f_8020e380(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e380) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e380); f_8020e380_b(c, 0x8020e380); crate::prof::exit(c, 0x8020e380); return; }
    }
    f_8020e380_b(c, 0x8020e380)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e380_at(c: &mut Cpu, pc: u32) { f_8020e380_b(c, pc) }
#[inline(always)]
fn f_8020e380_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e380;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e380, pc); }
        match pc {
            0x8020e380 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e390; continue; }
                pc = 0x8020e388;
            }
            0x8020e388 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e390;
            }
            0x8020e390 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xb));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e380: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetGetOkaneFlag (0x8020e398, 24 bytes)
#[inline(never)]
pub fn f_8020e398(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e398) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e398); f_8020e398_b(c, 0x8020e398); crate::prof::exit(c, 0x8020e398); return; }
    }
    f_8020e398_b(c, 0x8020e398)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e398_at(c: &mut Cpu, pc: u32) { f_8020e398_b(c, pc) }
#[inline(always)]
fn f_8020e398_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e398;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e398, pc); }
        match pc {
            0x8020e398 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e3a8; continue; }
                pc = 0x8020e3a0;
            }
            0x8020e3a0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e3a8;
            }
            0x8020e3a8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x9));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e398: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetOkanePoolFlag (0x8020e3b0, 24 bytes)
#[inline(never)]
pub fn f_8020e3b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e3b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e3b0); f_8020e3b0_b(c, 0x8020e3b0); crate::prof::exit(c, 0x8020e3b0); return; }
    }
    f_8020e3b0_b(c, 0x8020e3b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e3b0_at(c: &mut Cpu, pc: u32) { f_8020e3b0_b(c, pc) }
#[inline(always)]
fn f_8020e3b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e3b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e3b0, pc); }
        match pc {
            0x8020e3b0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e3c0; continue; }
                pc = 0x8020e3b8;
            }
            0x8020e3b8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e3c0;
            }
            0x8020e3c0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xa));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e3b0: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetGetExpFlag (0x8020e3c8, 24 bytes)
#[inline(never)]
pub fn f_8020e3c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e3c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e3c8); f_8020e3c8_b(c, 0x8020e3c8); crate::prof::exit(c, 0x8020e3c8); return; }
    }
    f_8020e3c8_b(c, 0x8020e3c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e3c8_at(c: &mut Cpu, pc: u32) { f_8020e3c8_b(c, pc) }
#[inline(always)]
fn f_8020e3c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e3c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e3c8, pc); }
        match pc {
            0x8020e3c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e3d8; continue; }
                pc = 0x8020e3d0;
            }
            0x8020e3d0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e3d8;
            }
            0x8020e3d8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e3c8: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDrawFlag (0x8020e3e0, 24 bytes)
#[inline(never)]
pub fn f_8020e3e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e3e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e3e0); f_8020e3e0_b(c, 0x8020e3e0); crate::prof::exit(c, 0x8020e3e0); return; }
    }
    f_8020e3e0_b(c, 0x8020e3e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e3e0_at(c: &mut Cpu, pc: u32) { f_8020e3e0_b(c, pc) }
#[inline(always)]
fn f_8020e3e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e3e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e3e0, pc); }
        match pc {
            0x8020e3e0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e3f0; continue; }
                pc = 0x8020e3e8;
            }
            0x8020e3e8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e3f0;
            }
            0x8020e3f0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x7));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e3e0: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetNigeruFlag (0x8020e3f8, 24 bytes)
#[inline(never)]
pub fn f_8020e3f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e3f8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e3f8); f_8020e3f8_b(c, 0x8020e3f8); crate::prof::exit(c, 0x8020e3f8); return; }
    }
    f_8020e3f8_b(c, 0x8020e3f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e3f8_at(c: &mut Cpu, pc: u32) { f_8020e3f8_b(c, pc) }
#[inline(always)]
fn f_8020e3f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e3f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e3f8, pc); }
        match pc {
            0x8020e3f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e408; continue; }
                pc = 0x8020e400;
            }
            0x8020e400 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e408;
            }
            0x8020e408 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x6));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e3f8: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetCallFlag (0x8020e410, 24 bytes)
#[inline(never)]
pub fn f_8020e410(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e410) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e410); f_8020e410_b(c, 0x8020e410); crate::prof::exit(c, 0x8020e410); return; }
    }
    f_8020e410_b(c, 0x8020e410)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e410_at(c: &mut Cpu, pc: u32) { f_8020e410_b(c, pc) }
#[inline(always)]
fn f_8020e410_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e410;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e410, pc); }
        match pc {
            0x8020e410 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e420; continue; }
                pc = 0x8020e418;
            }
            0x8020e418 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e420;
            }
            0x8020e420 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x5));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e410: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetUseItemFlag (0x8020e428, 24 bytes)
#[inline(never)]
pub fn f_8020e428(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e428) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e428); f_8020e428_b(c, 0x8020e428); crate::prof::exit(c, 0x8020e428); return; }
    }
    f_8020e428_b(c, 0x8020e428)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e428_at(c: &mut Cpu, pc: u32) { f_8020e428_b(c, pc) }
#[inline(always)]
fn f_8020e428_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e428;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e428, pc); }
        match pc {
            0x8020e428 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e438; continue; }
                pc = 0x8020e430;
            }
            0x8020e430 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e438;
            }
            0x8020e438 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e428: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoZukanTukamaetaFlag (0x8020e440, 24 bytes)
#[inline(never)]
pub fn f_8020e440(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e440) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e440); f_8020e440_b(c, 0x8020e440); crate::prof::exit(c, 0x8020e440); return; }
    }
    f_8020e440_b(c, 0x8020e440)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e440_at(c: &mut Cpu, pc: u32) { f_8020e440_b(c, pc) }
#[inline(always)]
fn f_8020e440_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e440;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e440, pc); }
        match pc {
            0x8020e440 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e450; continue; }
                pc = 0x8020e448;
            }
            0x8020e448 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e450;
            }
            0x8020e450 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x3));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e440: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoZukanMitaFlag (0x8020e458, 24 bytes)
#[inline(never)]
pub fn f_8020e458(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e458) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e458); f_8020e458_b(c, 0x8020e458); crate::prof::exit(c, 0x8020e458); return; }
    }
    f_8020e458_b(c, 0x8020e458)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e458_at(c: &mut Cpu, pc: u32) { f_8020e458_b(c, pc) }
#[inline(always)]
fn f_8020e458_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e458;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e458, pc); }
        match pc {
            0x8020e458 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e468; continue; }
                pc = 0x8020e460;
            }
            0x8020e460 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e468;
            }
            0x8020e468 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e458: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetDoBadgeCheckFlag (0x8020e470, 24 bytes)
#[inline(never)]
pub fn f_8020e470(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e470) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e470); f_8020e470_b(c, 0x8020e470); crate::prof::exit(c, 0x8020e470); return; }
    }
    f_8020e470_b(c, 0x8020e470)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e470_at(c: &mut Cpu, pc: u32) { f_8020e470_b(c, pc) }
#[inline(always)]
fn f_8020e470_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e470;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e470, pc); }
        match pc {
            0x8020e470 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e480; continue; }
                pc = 0x8020e478;
            }
            0x8020e478 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e480;
            }
            0x8020e480 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_8020e470: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetPtr (0x8020e488, 44 bytes)
#[inline(never)]
pub fn f_8020e488(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e488) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e488); f_8020e488_b(c, 0x8020e488); crate::prof::exit(c, 0x8020e488); return; }
    }
    f_8020e488_b(c, 0x8020e488)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e488_at(c: &mut Cpu, pc: u32) { f_8020e488_b(c, pc) }
#[inline(always)]
fn f_8020e488_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e488;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e488, pc); }
        match pc {
            0x8020e488 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8720));
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[5], c.r[0]);
                if !c.crbit(1) { pc = 0x8020e4a4; continue; }
                pc = 0x8020e49c;
            }
            0x8020e49c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e4a4;
            }
            0x8020e4a4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8724));
                c.r[0] = c.r[3].rotate_left(5) & 0x1fffe0;
                c.r[3] = c.r[4].wrapping_add(c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e488: bad pc {:#x}", pc),
        }
    }
}

/// fightKindDataBiosGetBackSaveDataFlag (0x8020e4b4, 24 bytes)
#[inline(never)]
pub fn f_8020e4b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e4b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e4b4); f_8020e4b4_b(c, 0x8020e4b4); crate::prof::exit(c, 0x8020e4b4); return; }
    }
    f_8020e4b4_b(c, 0x8020e4b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e4b4_at(c: &mut Cpu, pc: u32) { f_8020e4b4_b(c, pc) }
#[inline(always)]
fn f_8020e4b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e4b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e4b4, pc); }
        match pc {
            0x8020e4b4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8020e4c4; continue; }
                pc = 0x8020e4bc;
            }
            0x8020e4bc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8020e4c4;
            }
            0x8020e4c4 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e4b4: bad pc {:#x}", pc),
        }
    }
}

/// fightAbicntFitMinMax (0x8020e4cc, 28 bytes)
#[inline(never)]
pub fn f_8020e4cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e4cc) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e4cc); f_8020e4cc_b(c, 0x8020e4cc); crate::prof::exit(c, 0x8020e4cc); return; }
    }
    f_8020e4cc_b(c, 0x8020e4cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e4cc_at(c: &mut Cpu, pc: u32) { f_8020e4cc_b(c, pc) }
#[inline(always)]
fn f_8020e4cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e4cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e4cc, pc); }
        match pc {
            0x8020e4cc => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x8020e4d8; continue; }
                pc = 0x8020e4d4;
            }
            0x8020e4d4 => {
                c.r[3] = 0x0;
                pc = 0x8020e4d8;
            }
            0x8020e4d8 => {
                c.cmps(0, c.r[3], 0xc);
                if !c.crbit(1) { return; }
                pc = 0x8020e4e0;
            }
            0x8020e4e0 => {
                c.r[3] = 0xc;
                if true { return; }
                return;
            }
            _ => panic!("f_8020e4cc: bad pc {:#x}", pc),
        }
    }
}

/// fightAbicntDoKakeWaru (0x8020e4e8, 148 bytes)
#[inline(never)]
pub fn f_8020e4e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8020e4e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8020e4e8); f_8020e4e8_b(c, 0x8020e4e8); crate::prof::exit(c, 0x8020e4e8); return; }
    }
    f_8020e4e8_b(c, 0x8020e4e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8020e4e8_at(c: &mut Cpu, pc: u32) { f_8020e4e8_b(c, pc) }
#[inline(always)]
fn f_8020e4e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8020e4e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8020e4e8, pc); }
        match pc {
            0x8020e4e8 => {
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff8548));
                c.r[8] = c.r[3].rotate_left(0) & 0xffff;
                c.r[5] = 0x80370000;
                c.r[6] = c.r[3].rotate_left(1) & 0x1fffe;
                c.cmpu(0, c.r[8], c.r[7]);
                c.r[0] = c.r[5].wrapping_add(0x5d10);
                c.r[5] = c.r[0].wrapping_add(c.r[6]);
                if c.crbit(0) { pc = 0x8020e50c; continue; }
                pc = 0x8020e508;
            }
            0x8020e508 => {
                c.r[5] = 0x0;
                pc = 0x8020e50c;
            }
            0x8020e50c => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8020e51c; continue; }
                pc = 0x8020e514;
            }
            0x8020e514 => {
                c.r[6] = 0x0;
                pc = 0x8020e52c; continue;
                pc = 0x8020e51c;
            }
            0x8020e51c => {
                if !c.crbit(2) { pc = 0x8020e528; continue; }
                pc = 0x8020e520;
            }
            0x8020e520 => {
                c.r[6] = 0x0;
                pc = 0x8020e52c; continue;
                pc = 0x8020e528;
            }
            0x8020e528 => {
                c.r[6] = c.lbz(c.r[5]);
                pc = 0x8020e52c;
            }
            0x8020e52c => {
                c.r[5] = 0x80370000;
                c.cmpu(0, c.r[8], c.r[7]);
                c.r[3] = c.r[3].rotate_left(1) & 0x1fffe;
                c.r[0] = c.r[5].wrapping_add(0x5d10);
                c.r[5] = c.r[0].wrapping_add(c.r[3]);
                if c.crbit(0) { pc = 0x8020e548; continue; }
                pc = 0x8020e544;
            }
            0x8020e544 => {
                c.r[5] = 0x0;
                pc = 0x8020e548;
            }
            0x8020e548 => {
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[5], 0x0);
                c.r[3] = c.r[4].wrapping_mul(c.r[0]);
                if !c.crbit(2) { pc = 0x8020e560; continue; }
                pc = 0x8020e558;
            }
            0x8020e558 => {
                c.r[0] = 0x1;
                pc = 0x8020e570; continue;
                pc = 0x8020e560;
            }
            0x8020e560 => {
                if !c.crbit(2) { pc = 0x8020e56c; continue; }
                pc = 0x8020e564;
            }
            0x8020e564 => {
                c.r[0] = 0x1;
                pc = 0x8020e570; continue;
                pc = 0x8020e56c;
            }
            0x8020e56c => {
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x1));
                pc = 0x8020e570;
            }
            0x8020e570 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[3] = divwu(c.r[3], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8020e4e8: bad pc {:#x}", pc),
        }
    }
}

