#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// OSGetTick (0x800a2790, 8 bytes)
#[inline(never)]
pub fn f_800a2790(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2790) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2790); p = true; }
    }
    f_800a2790_b(c, 0x800a2790);
    if p { crate::prof::exit(c, 0x800a2790); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2790_at(c: &mut Cpu, pc: u32) { f_800a2790_b(c, pc) }
#[inline(always)]
fn f_800a2790_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2790;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2790, pc); }
        match pc {
            0x800a2790 => {
                c.r[3] = 0; // mftb
                if true { return; }
                return;
            }
            _ => panic!("f_800a2790: bad pc {:#x}", pc),
        }
    }
}

/// __OSGetSystemTime (0x800a2798, 100 bytes)
#[inline(never)]
pub fn f_800a2798(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2798) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2798); p = true; }
    }
    f_800a2798_b(c, 0x800a2798);
    if p { crate::prof::exit(c, 0x800a2798); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2798_at(c: &mut Cpu, pc: u32) { f_800a2798_b(c, pc) }
#[inline(always)]
fn f_800a2798_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2798;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2798, pc); }
        match pc {
            0x800a2798 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.lr = 0x800a27b4; f_8009df3c(c);
                pc = 0x800a27b4;
            }
            0x800a27b4 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800a27bc; crate::stubs::f_800a2778(c);
                pc = 0x800a27bc;
            }
            0x800a27bc => {
                c.r[6] = 0x80000000;
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x30dc));
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x30d8));
                c.r[29] = c.addc(c.r[5], c.r[4], 0);
                { let ci = c.ca(); c.r[30] = c.addc(c.r[0], c.r[3], ci); }
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a27d8; f_8009df64(c);
                pc = 0x800a27d8;
            }
            0x800a27d8 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2798: bad pc {:#x}", pc),
        }
    }
}

/// GetDates (0x800a27fc, 412 bytes)
#[inline(never)]
pub fn f_800a27fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a27fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a27fc); p = true; }
    }
    f_800a27fc_b(c, 0x800a27fc);
    if p { crate::prof::exit(c, 0x800a27fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a27fc_at(c: &mut Cpu, pc: u32) { f_800a27fc_b(c, pc) }
#[inline(always)]
fn f_800a27fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a27fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a27fc, pc); }
        match pc {
            0x800a27fc => {
                c.r[5] = 0x92490000;
                c.r[0] = c.r[5].wrapping_add(0x2493);
                c.r[7] = c.r[3].wrapping_add(0x6);
                c.r[6] = mulhw(c.r[0], c.r[7]);
                c.r[5] = 0xb38d0000;
                c.r[0] = c.r[5].wrapping_add(0xfffff9b1);
                c.r[0] = mulhw(c.r[0], c.r[3]);
                c.r[5] = c.r[6].wrapping_add(c.r[7]);
                { let v = c.r[5] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[5] = (v >> 2) as u32; }
                c.r[6] = c.r[5].rotate_left(1) & 0x1;
                c.r[5] = c.r[5].wrapping_add(c.r[6]);
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[6] = c.r[5].wrapping_mul(0x7);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 24 != 0); c.r[0] = (v >> 8) as u32; }
                c.r[5] = c.r[0].rotate_left(1) & 0x1;
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = c.r[7].wrapping_sub(c.r[6]);
                c.r[11] = c.r[5].wrapping_mul(0x16d);
                c.stw(c.r[4].wrapping_add(0x18), c.r[0]);
                pc = 0x800a284c; continue;
                pc = 0x800a284c;
            }
            0x800a284c => {
                c.r[6] = 0x51ec0000;
                c.r[10] = c.r[6].wrapping_add(0xffff851f);
                pc = 0x800a2858; continue;
                pc = 0x800a2858;
            }
            0x800a2858 => {
                pc = 0x800a2864; continue;
                pc = 0x800a285c;
            }
            0x800a285c => {
                c.r[11] = c.r[11].wrapping_add(0xfffffe93);
                c.r[5] = c.r[5].wrapping_add(0xffffffff);
                pc = 0x800a2864;
            }
            0x800a2864 => {
                c.cmps(0, c.r[5], 0x1);
                if !c.crbit(0) { pc = 0x800a2874; continue; }
                pc = 0x800a286c;
            }
            0x800a286c => {
                c.r[0] = 0x0;
                pc = 0x800a28a8; continue;
                pc = 0x800a2874;
            }
            0x800a2874 => {
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                c.r[0] = mulhw(c.r[10], c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 25 != 0); c.r[8] = (v >> 7) as u32; }
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[6] = (v >> 5) as u32; }
                c.r[0] = c.r[5].wrapping_add(0x3);
                c.r[7] = c.r[6].rotate_left(1) & 0x1;
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[0] = (v >> 2) as u32; }
                c.r[9] = c.r[8].rotate_left(1) & 0x1;
                c.r[6] = c.r[6].wrapping_add(c.r[7]);
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[7] = c.r[8].wrapping_add(c.r[9]);
                c.r[0] = c.r[0].wrapping_sub(c.r[6]);
                c.r[0] = c.r[7].wrapping_add(c.r[0]);
                pc = 0x800a28a8;
            }
            0x800a28a8 => {
                c.r[0] = c.r[11].wrapping_add(c.r[0]);
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x800a285c; continue; }
                pc = 0x800a28b4;
            }
            0x800a28b4 => {
                { let v = c.r[5] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[6] = (v >> 2) as u32; }
                c.stw(c.r[4].wrapping_add(0x14), c.r[5]);
                { let ci = c.ca(); c.r[6] = c.addc(c.r[6], 0, ci); }
                c.r[6] = c.r[6].rotate_left(2) & 0xfffffffc;
                c.r[6] = c.addc(!c.r[6], c.r[5], 1);
                c.r[0] = c.r[3].wrapping_sub(c.r[0]);
                c.cmps(0, c.r[6], 0x0);
                c.stw(c.r[4].wrapping_add(0x1c), c.r[0]);
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                if !c.crbit(2) { pc = 0x800a290c; continue; }
                pc = 0x800a28e0;
            }
            0x800a28e0 => {
                c.r[3] = 0x51ec0000;
                c.r[3] = c.r[3].wrapping_add(0xffff851f);
                c.r[3] = mulhw(c.r[3], c.r[5]);
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[3] = (v >> 5) as u32; }
                c.r[6] = c.r[3].rotate_left(1) & 0x1;
                c.r[3] = c.r[3].wrapping_add(c.r[6]);
                c.r[3] = c.r[3].wrapping_mul(0x64);
                c.r[3] = c.r[5].wrapping_sub(c.r[3]);
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a290c; continue; }
                pc = 0x800a2908;
            }
            0x800a2908 => {
                c.r[8] = c.r[7] | c.r[7];
                pc = 0x800a290c;
            }
            0x800a290c => {
                c.cmps(0, c.r[8], 0x0);
                if !c.crbit(2) { pc = 0x800a2940; continue; }
                pc = 0x800a2914;
            }
            0x800a2914 => {
                c.r[3] = 0x51ec0000;
                c.r[3] = c.r[3].wrapping_add(0xffff851f);
                c.r[3] = mulhw(c.r[3], c.r[5]);
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 25 != 0); c.r[3] = (v >> 7) as u32; }
                c.r[6] = c.r[3].rotate_left(1) & 0x1;
                c.r[3] = c.r[3].wrapping_add(c.r[6]);
                c.r[3] = c.r[3].wrapping_mul(0x190);
                c.r[3] = c.r[5].wrapping_sub(c.r[3]);
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a2940; continue; }
                pc = 0x800a293c;
            }
            0x800a293c => {
                c.r[7] = 0x0;
                pc = 0x800a2940;
            }
            0x800a2940 => {
                c.cmps(0, c.r[7], 0x0);
                if c.crbit(2) { pc = 0x800a2954; continue; }
                pc = 0x800a2948;
            }
            0x800a2948 => {
                c.r[3] = 0x80310000;
                c.r[6] = c.r[3].wrapping_add(0x18a8);
                pc = 0x800a295c; continue;
                pc = 0x800a2954;
            }
            0x800a2954 => {
                c.r[3] = 0x80310000;
                c.r[6] = c.r[3].wrapping_add(0x1878);
                pc = 0x800a295c;
            }
            0x800a295c => {
                c.r[7] = 0xc;
                c.r[3] = 0x30;
                pc = 0x800a2968; continue;
                pc = 0x800a2968;
            }
            0x800a2968 => {
                pc = 0x800a296c; continue;
                pc = 0x800a296c;
            }
            0x800a296c => {
                c.r[3] = c.r[3].wrapping_add(0xfffffffc);
                c.r[5] = c.lwz(c.r[6].wrapping_add(c.r[3]));
                c.r[7] = c.r[7].wrapping_add(0xffffffff);
                c.cmps(0, c.r[0], c.r[5]);
                if c.crbit(0) { pc = 0x800a296c; continue; }
                pc = 0x800a2980;
            }
            0x800a2980 => {
                c.stw(c.r[4].wrapping_add(0x10), c.r[7]);
                c.r[3] = c.lwz(c.r[6].wrapping_add(c.r[3]));
                c.r[3] = c.r[0].wrapping_sub(c.r[3]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a27fc: bad pc {:#x}", pc),
        }
    }
}

/// OSTicksToCalendarTime (0x800a2998, 516 bytes)
#[inline(never)]
pub fn f_800a2998(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2998) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2998); p = true; }
    }
    f_800a2998_b(c, 0x800a2998);
    if p { crate::prof::exit(c, 0x800a2998); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2998_at(c: &mut Cpu, pc: u32) { f_800a2998_b(c, pc) }
#[inline(always)]
fn f_800a2998_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2998;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2998, pc); }
        match pc {
            0x800a2998 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = c.r[5] | c.r[5];
                c.r[27] = 0x80000000;
                c.r[0] = c.lwz(c.r[27].wrapping_add(0xf8));
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[6] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[5] = 0x0;
                c.lr = 0x800a29d0; f_800c4b44(c);
                pc = 0x800a29d0;
            }
            0x800a29d0 => {
                c.r[26] = c.r[3] | c.r[3];
                c.r[5] = 0x0;
                c.r[25] = c.r[4] | c.r[4];
                c.r[4] = c.r[26] ^ 0x80000000;
                c.r[3] = c.r[5] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[5], c.r[25], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[4], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[4], c.r[4], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a2a0c; continue; }
                pc = 0x800a29fc;
            }
            0x800a29fc => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0xf8));
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[25] = c.addc(c.r[25], c.r[0], 0);
                { let ci = c.ca(); c.r[26] = c.addc(c.r[26], c.r[5], ci); }
                pc = 0x800a2a0c;
            }
            0x800a2a0c => {
                c.r[4] = 0x8;
                c.r[3] = c.r[26].wrapping_mul(c.r[4]);
                c.r[0] = mulhwu(c.r[25], c.r[4]);
                c.r[27] = 0x80000000;
                c.r[6] = c.lwz(c.r[27].wrapping_add(0xf8));
                c.r[5] = 0x431c0000;
                c.r[5] = c.r[5].wrapping_add(0xffffde83);
                c.r[6] = c.r[6].rotate_left(30) & 0x3fffffff;
                c.r[5] = mulhwu(c.r[5], c.r[6]);
                c.r[6] = c.r[5].rotate_left(17) & 0x1ffff;
                c.r[28] = 0x0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.r[25].wrapping_mul(c.r[28]);
                c.r[4] = c.r[25].wrapping_mul(c.r[4]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[5] = 0x0;
                c.lr = 0x800a2a50; f_800c4928(c);
                pc = 0x800a2a50;
            }
            0x800a2a50 => {
                c.r[5] = 0x0;
                c.r[6] = 0x3e8;
                c.lr = 0x800a2a5c; f_800c4b44(c);
                pc = 0x800a2a5c;
            }
            0x800a2a5c => {
                c.stw(c.r[31].wrapping_add(0x24), c.r[4]);
                c.r[3] = 0x10620000;
                c.r[5] = c.r[3].wrapping_add(0x4dd3);
                c.r[0] = c.lwz(c.r[27].wrapping_add(0xf8));
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[25] | c.r[25];
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = mulhwu(c.r[5], c.r[0]);
                c.r[6] = c.r[0].rotate_left(26) & 0x3ffffff;
                c.r[5] = 0x0;
                c.lr = 0x800a2a88; f_800c4928(c);
                pc = 0x800a2a88;
            }
            0x800a2a88 => {
                c.r[5] = 0x0;
                c.r[6] = 0x3e8;
                c.lr = 0x800a2a94; f_800c4b44(c);
                pc = 0x800a2a94;
            }
            0x800a2a94 => {
                c.stw(c.r[31].wrapping_add(0x20), c.r[4]);
                c.r[30] = c.addc(!c.r[25], c.r[30], 1);
                { let ci = c.ca(); c.r[29] = c.addc(!c.r[26], c.r[29], ci); }
                c.r[0] = c.lwz(c.r[27].wrapping_add(0xf8));
                c.r[5] = 0x10000;
                c.r[25] = c.r[5].wrapping_add(0x5180);
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x800a2ac0; f_800c4928(c);
                pc = 0x800a2ac0;
            }
            0x800a2ac0 => {
                c.r[6] = c.r[25] | c.r[25];
                c.r[5] = 0x0;
                c.lr = 0x800a2acc; f_800c4928(c);
                pc = 0x800a2acc;
            }
            0x800a2acc => {
                c.r[5] = 0xb0000;
                c.r[0] = c.lwz(c.r[27].wrapping_add(0xf8));
                c.r[5] = c.r[5].wrapping_add(0x2575);
                c.r[26] = c.addc(c.r[4], c.r[5], 0);
                c.r[6] = c.r[0].rotate_left(30) & 0x3fffffff;
                { let ci = c.ca(); c.r[0] = c.addc(c.r[3], c.r[28], ci); }
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.lr = 0x800a2af4; f_800c4928(c);
                pc = 0x800a2af4;
            }
            0x800a2af4 => {
                c.r[6] = c.r[25] | c.r[25];
                c.r[5] = 0x0;
                c.lr = 0x800a2b00; f_800c4b44(c);
                pc = 0x800a2b00;
            }
            0x800a2b00 => {
                c.r[27] = c.r[4] | c.r[4];
                c.cmps(0, c.r[27], 0x0);
                if !c.crbit(0) { pc = 0x800a2b18; continue; }
                pc = 0x800a2b0c;
            }
            0x800a2b0c => {
                c.r[27] = c.r[27].wrapping_add(0x10000);
                c.r[26] = c.r[26].wrapping_add(0xffffffff);
                c.r[27] = c.r[27].wrapping_add(0x5180);
                pc = 0x800a2b18;
            }
            0x800a2b18 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800a2b24; f_800a27fc(c);
                pc = 0x800a2b24;
            }
            0x800a2b24 => {
                c.r[3] = 0x88890000;
                c.r[5] = c.r[3].wrapping_add(0xffff8889);
                c.r[0] = mulhw(c.r[5], c.r[27]);
                c.r[4] = c.r[0].wrapping_add(c.r[27]);
                { let v = c.r[4] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[0] = (v >> 5) as u32; }
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[7] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = mulhw(c.r[5], c.r[7]);
                c.r[0] = c.r[0].wrapping_add(c.r[7]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[5] = (v >> 5) as u32; }
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[0] = (v >> 5) as u32; }
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                { let v = c.r[4] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[0] = (v >> 5) as u32; }
                c.r[6] = c.r[5].rotate_left(1) & 0x1;
                c.r[4] = c.r[3].wrapping_mul(0x3c);
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[5] = c.r[5].wrapping_add(c.r[6]);
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.stw(c.r[31].wrapping_add(0x8), c.r[5]);
                c.r[0] = c.r[0].wrapping_mul(0x3c);
                c.r[3] = c.r[7].wrapping_sub(c.r[4]);
                c.stw(c.r[31].wrapping_add(0x4), c.r[3]);
                c.r[0] = c.r[27].wrapping_sub(c.r[0]);
                c.stw(c.r[31], c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2998: bad pc {:#x}", pc),
        }
    }
}

/// __init_user (0x800a2b9c, 32 bytes)
#[inline(never)]
pub fn f_800a2b9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2b9c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2b9c); p = true; }
    }
    f_800a2b9c_b(c, 0x800a2b9c);
    if p { crate::prof::exit(c, 0x800a2b9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2b9c_at(c: &mut Cpu, pc: u32) { f_800a2b9c_b(c, pc) }
#[inline(always)]
fn f_800a2b9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2b9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2b9c, pc); }
        match pc {
            0x800a2b9c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a2bac; f_800a2bbc(c);
                pc = 0x800a2bac;
            }
            0x800a2bac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2b9c: bad pc {:#x}", pc),
        }
    }
}

/// __init_cpp (0x800a2bbc, 84 bytes)
#[inline(never)]
pub fn f_800a2bbc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2bbc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2bbc); p = true; }
    }
    f_800a2bbc_b(c, 0x800a2bbc);
    if p { crate::prof::exit(c, 0x800a2bbc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2bbc_at(c: &mut Cpu, pc: u32) { f_800a2bbc_b(c, pc) }
#[inline(always)]
fn f_800a2bbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2bbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2bbc, pc); }
        match pc {
            0x800a2bbc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[3] = 0x80260000;
                c.r[0] = c.r[3].wrapping_add(0x6360);
                c.r[31] = c.r[0] | c.r[0];
                pc = 0x800a2bdc; continue;
                pc = 0x800a2bdc;
            }
            0x800a2bdc => {
                pc = 0x800a2be0; continue;
                pc = 0x800a2be0;
            }
            0x800a2be0 => {
                pc = 0x800a2bf0; continue;
                pc = 0x800a2be4;
            }
            0x800a2be4 => {
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a2bec; (c.dispatch)(c, t); }
                pc = 0x800a2bec;
            }
            0x800a2bec => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800a2bf0;
            }
            0x800a2bf0 => {
                c.r[12] = c.lwz(c.r[31]);
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x800a2be4; continue; }
                pc = 0x800a2bfc;
            }
            0x800a2bfc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2bbc: bad pc {:#x}", pc),
        }
    }
}

/// _ExitProcess (0x800a2c10, 32 bytes)
#[inline(never)]
pub fn f_800a2c10(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2c10) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2c10); p = true; }
    }
    f_800a2c10_b(c, 0x800a2c10);
    if p { crate::prof::exit(c, 0x800a2c10); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2c10_at(c: &mut Cpu, pc: u32) { f_800a2c10_b(c, pc) }
#[inline(always)]
fn f_800a2c10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2c10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2c10, pc); }
        match pc {
            0x800a2c10 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a2c20; f_8009803c(c);
                pc = 0x800a2c20;
            }
            0x800a2c20 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2c10: bad pc {:#x}", pc),
        }
    }
}

/// DBInit (0x800a2c30, 40 bytes)
#[inline(never)]
pub fn f_800a2c30(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2c30) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2c30); p = true; }
    }
    f_800a2c30_b(c, 0x800a2c30);
    if p { crate::prof::exit(c, 0x800a2c30); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2c30_at(c: &mut Cpu, pc: u32) { f_800a2c30_b(c, pc) }
#[inline(always)]
fn f_800a2c30_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2c30;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2c30, pc); }
        match pc {
            0x800a2c30 => {
                c.r[4] = 0x80000000;
                c.r[0] = c.r[4].wrapping_add(0x40);
                c.r[3] = 0x800a0000;
                c.stw(c.r[13].wrapping_add(0xffff9f58), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x2cbc);
                c.r[0] = c.r[3].wrapping_add(0x80000000);
                c.stw(c.r[4].wrapping_add(0x48), c.r[0]);
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f5c), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2c30: bad pc {:#x}", pc),
        }
    }
}

/// DBIsDebuggerPresent (0x800a2c58, 28 bytes)
#[inline(never)]
pub fn f_800a2c58(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2c58) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2c58); p = true; }
    }
    f_800a2c58_b(c, 0x800a2c58);
    if p { crate::prof::exit(c, 0x800a2c58); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2c58_at(c: &mut Cpu, pc: u32) { f_800a2c58_b(c, pc) }
#[inline(always)]
fn f_800a2c58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2c58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2c58, pc); }
        match pc {
            0x800a2c58 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9f58));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a2c6c; continue; }
                pc = 0x800a2c64;
            }
            0x800a2c64 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x800a2c6c;
            }
            0x800a2c6c => {
                c.r[3] = c.lwz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2c58: bad pc {:#x}", pc),
        }
    }
}

/// __DBExceptionDestinationAux (0x800a2c74, 72 bytes)
#[inline(never)]
pub fn f_800a2c74(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2c74) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2c74); p = true; }
    }
    f_800a2c74_b(c, 0x800a2c74);
    if p { crate::prof::exit(c, 0x800a2c74); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2c74_at(c: &mut Cpu, pc: u32) { f_800a2c74_b(c, pc) }
#[inline(always)]
fn f_800a2c74_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2c74;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2c74, pc); }
        match pc {
            0x800a2c74 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80310000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x18d8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[4] = c.lwz(0xc0u32);
                c.r[31] = c.r[4].wrapping_add(0x80000000);
                c.lr = 0x800a2c9c; f_8009c2e0(c);
                pc = 0x800a2c9c;
            }
            0x800a2c9c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a2ca4; f_8009be40(c);
                pc = 0x800a2ca4;
            }
            0x800a2ca4 => {
                c.lr = 0x800a2ca8; f_8009803c(c);
                pc = 0x800a2ca8;
            }
            0x800a2ca8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2c74: bad pc {:#x}", pc),
        }
    }
}

/// __DBExceptionDestination (0x800a2cbc, 16 bytes)
#[inline(never)]
pub fn f_800a2cbc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2cbc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2cbc); p = true; }
    }
    f_800a2cbc_b(c, 0x800a2cbc);
    if p { crate::prof::exit(c, 0x800a2cbc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2cbc_at(c: &mut Cpu, pc: u32) { f_800a2cbc_b(c, pc) }
#[inline(always)]
fn f_800a2cbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2cbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2cbc, pc); }
        match pc {
            0x800a2cbc => {
                c.r[3] = 0; // mfmsr
                c.r[3] = c.r[3] | 0x30;
                // mtmsr
                f_800a2c74(c); return;
                return;
            }
            _ => panic!("f_800a2cbc: bad pc {:#x}", pc),
        }
    }
}

/// __DBIsExceptionMarked (0x800a2ccc, 28 bytes)
#[inline(never)]
pub fn f_800a2ccc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2ccc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2ccc); p = true; }
    }
    f_800a2ccc_b(c, 0x800a2ccc);
    if p { crate::prof::exit(c, 0x800a2ccc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2ccc_at(c: &mut Cpu, pc: u32) { f_800a2ccc_b(c, pc) }
#[inline(always)]
fn f_800a2ccc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2ccc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2ccc, pc); }
        match pc {
            0x800a2ccc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9f58));
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x1;
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = slw(c.r[3], c.r[0]);
                c.r[3] = c.r[4] & c.r[0];
                if true { return; }
                return;
            }
            _ => panic!("f_800a2ccc: bad pc {:#x}", pc),
        }
    }
}

/// DBPrintf (0x800a2ce8, 80 bytes)
#[inline(never)]
pub fn f_800a2ce8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2ce8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2ce8); p = true; }
    }
    f_800a2ce8_b(c, 0x800a2ce8);
    if p { crate::prof::exit(c, 0x800a2ce8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2ce8_at(c: &mut Cpu, pc: u32) { f_800a2ce8_b(c, pc) }
#[inline(always)]
fn f_800a2ce8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2ce8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2ce8, pc); }
        match pc {
            0x800a2ce8 => {
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                if !c.crbit(6) { pc = 0x800a2d10; continue; }
                pc = 0x800a2cf0;
            }
            0x800a2cf0 => {
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[1]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[3]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[4]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let v = c.f[5]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[6]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let v = c.f[7]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[8]; c.stfd(ea, v); }
                pc = 0x800a2d10;
            }
            0x800a2d10 => {
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[10]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2ce8: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXIdentity (0x800a2d38, 44 bytes)
#[inline(never)]
pub fn f_800a2d38(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2d38) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2d38); p = true; }
    }
    f_800a2d38_b(c, 0x800a2d38);
    if p { crate::prof::exit(c, 0x800a2d38); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2d38_at(c: &mut Cpu, pc: u32) { f_800a2d38_b(c, pc) }
#[inline(always)]
fn f_800a2d38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2d38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2d38, pc); }
        match pc {
            0x800a2d38 => {
                { let ea = c.r[2].wrapping_add(0xffff8bec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[2].wrapping_add(0xffff8be8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (c.f[0], c.ps1[1]); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (c.ps1[1], c.f[0]); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a2d38: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXCopy (0x800a2d64, 52 bytes)
#[inline(never)]
pub fn f_800a2d64(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2d64) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2d64); p = true; }
    }
    f_800a2d64_b(c, 0x800a2d64);
    if p { crate::prof::exit(c, 0x800a2d64); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2d64_at(c: &mut Cpu, pc: u32) { f_800a2d64_b(c, pc) }
#[inline(always)]
fn f_800a2d64_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2d64;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2d64, pc); }
        match pc {
            0x800a2d64 => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x10); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[4].wrapping_add(0x18); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[4].wrapping_add(0x20); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[4].wrapping_add(0x28); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a2d64: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXConcat (0x800a2d98, 204 bytes)
#[inline(never)]
pub fn f_800a2d98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2d98) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2d98); p = true; }
    }
    f_800a2d98_b(c, 0x800a2d98);
    if p { crate::prof::exit(c, 0x800a2d98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2d98_at(c: &mut Cpu, pc: u32) { f_800a2d98_b(c, pc) }
#[inline(always)]
fn f_800a2d98_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2d98;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2d98, pc); }
        match pc {
            0x800a2d98 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[14]; c.stfd(ea, v); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[6] = x; c.ps1[6] = y; }
                c.r[6] = 0x80480000;
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[15]; c.stfd(ea, v); }
                c.r[6] = c.r[6].wrapping_add(0xffff89b0);
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[4].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[6] * c.f[0]), s(c.ps1[6] * c.f[0])); c.f[12] = x; c.ps1[12] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[7] * c.f[0]), s(c.ps1[7] * c.f[0])); c.f[13] = x; c.ps1[13] = y; }
                { let ea = c.r[6].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let (x, y) = (s(c.f[6] * c.f[2]), s(c.ps1[6] * c.f[2])); c.f[14] = x; c.ps1[14] = y; }
                { let ea = c.r[4].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (s(c.f[7] * c.f[2]), s(c.ps1[7] * c.f[2])); c.f[15] = x; c.ps1[15] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[8].mul_add(c.ps1[0], c.f[12])), s(c.ps1[8].mul_add(c.ps1[0], c.ps1[12]))); c.f[12] = x; c.ps1[12] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[8].mul_add(c.ps1[2], c.f[14])), s(c.ps1[8].mul_add(c.ps1[2], c.ps1[14]))); c.f[14] = x; c.ps1[14] = y; }
                { let ea = c.r[4].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[10] = x; c.ps1[10] = y; }
                { let (x, y) = (s(c.f[9].mul_add(c.ps1[0], c.f[13])), s(c.ps1[9].mul_add(c.ps1[0], c.ps1[13]))); c.f[13] = x; c.ps1[13] = y; }
                { let ea = c.r[4].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (s(c.f[9].mul_add(c.ps1[2], c.f[15])), s(c.ps1[9].mul_add(c.ps1[2], c.ps1[15]))); c.f[15] = x; c.ps1[15] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[10].mul_add(c.f[1], c.f[12])), s(c.ps1[10].mul_add(c.f[1], c.ps1[12]))); c.f[12] = x; c.ps1[12] = y; }
                { let (x, y) = (s(c.f[11].mul_add(c.f[1], c.f[13])), s(c.ps1[11].mul_add(c.f[1], c.ps1[13]))); c.f[13] = x; c.ps1[13] = y; }
                { let (x, y) = (s(c.f[10].mul_add(c.f[3], c.f[14])), s(c.ps1[10].mul_add(c.f[3], c.ps1[14]))); c.f[14] = x; c.ps1[14] = y; }
                { let (x, y) = (s(c.f[11].mul_add(c.f[3], c.f[15])), s(c.ps1[11].mul_add(c.f[3], c.ps1[15]))); c.f[15] = x; c.ps1[15] = y; }
                { let ea = c.r[5].wrapping_add(0x0); let (x, y) = (c.f[12], c.ps1[12]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[6] * c.f[4]), s(c.ps1[6] * c.f[4])); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[31].mul_add(c.ps1[1], c.f[13])), s(c.ps1[31].mul_add(c.ps1[1], c.ps1[13]))); c.f[13] = x; c.ps1[13] = y; }
                { let (x, y) = (s(c.f[7] * c.f[4]), s(c.ps1[7] * c.f[4])); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[5].wrapping_add(0x10); let (x, y) = (c.f[14], c.ps1[14]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[31].mul_add(c.ps1[3], c.f[15])), s(c.ps1[31].mul_add(c.ps1[3], c.ps1[15]))); c.f[15] = x; c.ps1[15] = y; }
                { let ea = c.r[5].wrapping_add(0x8); let (x, y) = (c.f[13], c.ps1[13]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[8].mul_add(c.ps1[4], c.f[2])), s(c.ps1[8].mul_add(c.ps1[4], c.ps1[2]))); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[9].mul_add(c.ps1[4], c.f[0])), s(c.ps1[9].mul_add(c.ps1[4], c.ps1[0]))); c.f[0] = x; c.ps1[0] = y; }
                { let (x, y) = (s(c.f[10].mul_add(c.f[5], c.f[2])), s(c.ps1[10].mul_add(c.f[5], c.ps1[2]))); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[14] = c.lfd(ea); }
                { let ea = c.r[5].wrapping_add(0x18); let (x, y) = (c.f[15], c.ps1[15]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[11].mul_add(c.f[5], c.f[0])), s(c.ps1[11].mul_add(c.f[5], c.ps1[0]))); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[5].wrapping_add(0x20); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[31].mul_add(c.ps1[5], c.f[0])), s(c.ps1[31].mul_add(c.ps1[5], c.ps1[0]))); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[15] = c.lfd(ea); }
                { let ea = c.r[5].wrapping_add(0x28); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x28); c.f[31] = c.lfd(ea); }
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2d98: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXTranspose (0x800a2e64, 80 bytes)
#[inline(never)]
pub fn f_800a2e64(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2e64) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2e64); p = true; }
    }
    f_800a2e64_b(c, 0x800a2e64);
    if p { crate::prof::exit(c, 0x800a2e64); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2e64_at(c: &mut Cpu, pc: u32) { f_800a2e64_b(c, pc) }
#[inline(always)]
fn f_800a2e64_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2e64;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2e64, pc); }
        match pc {
            0x800a2e64 => {
                { let ea = c.r[2].wrapping_add(0xffff8bec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[4].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (c.f[1], c.f[2]); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, true, 0); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (c.ps1[1], c.ps1[2]); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, true, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (c.f[3], c.f[2]); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x10); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (c.f[1], c.f[0]); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[4].wrapping_add(0x20); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (c.ps1[1], c.f[0]); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x28); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4].wrapping_add(0x18); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x28); let v = c.f[3]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a2e64: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXInverse (0x800a2eb4, 248 bytes)
#[inline(never)]
pub fn f_800a2eb4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2eb4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2eb4); p = true; }
    }
    f_800a2eb4_b(c, 0x800a2eb4);
    if p { crate::prof::exit(c, 0x800a2eb4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2eb4_at(c: &mut Cpu, pc: u32) { f_800a2eb4_b(c, pc) }
#[inline(always)]
fn f_800a2eb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2eb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2eb4, pc); }
        match pc {
            0x800a2eb4 => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, true, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[3].wrapping_add(0x4); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, true, 0); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (c.ps1[1], c.f[0]); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[3].wrapping_add(0x14); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, true, 0); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (c.ps1[3], c.f[2]); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[3].wrapping_add(0x24); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[3] * c.f[6]), s(c.ps1[3] * c.ps1[6])); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (s(c.f[5] * c.f[7]), s(c.ps1[5] * c.ps1[7])); c.f[13] = x; c.ps1[13] = y; }
                { let (x, y) = (c.ps1[5], c.f[4]); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[1].mul_add(c.f[7], -c.f[11])), s(c.ps1[1].mul_add(c.ps1[7], -c.ps1[11]))); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (s(c.f[1] * c.f[8]), s(c.ps1[1] * c.ps1[8])); c.f[12] = x; c.ps1[12] = y; }
                { let (x, y) = (s(c.f[3].mul_add(c.f[8], -c.f[13])), s(c.ps1[3].mul_add(c.ps1[8], -c.ps1[13]))); c.f[13] = x; c.ps1[13] = y; }
                { let (x, y) = (s(c.f[3] * c.f[4]), s(c.ps1[3] * c.ps1[4])); c.f[10] = x; c.ps1[10] = y; }
                { let (x, y) = (s(c.f[5].mul_add(c.f[6], -c.f[12])), s(c.ps1[5].mul_add(c.ps1[6], -c.ps1[12]))); c.f[12] = x; c.ps1[12] = y; }
                { let (x, y) = (s(c.f[0] * c.f[5]), s(c.ps1[0] * c.ps1[5])); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (s(c.f[1] * c.f[2]), s(c.ps1[1] * c.ps1[2])); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[6] - c.f[6]), s(c.ps1[6] - c.ps1[6])); c.f[6] = x; c.ps1[6] = y; }
                { let (x, y) = (s(c.f[2].mul_add(c.f[5], -c.f[10])), s(c.ps1[2].mul_add(c.ps1[5], -c.ps1[10]))); c.f[10] = x; c.ps1[10] = y; }
                { let (x, y) = (s(c.f[0] * c.f[13]), s(c.ps1[0] * c.ps1[13])); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[1].mul_add(c.f[4], -c.f[9])), s(c.ps1[1].mul_add(c.ps1[4], -c.ps1[9]))); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (s(c.f[2].mul_add(c.f[12], c.f[7])), s(c.ps1[2].mul_add(c.ps1[12], c.ps1[7]))); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[0].mul_add(c.f[3], -c.f[8])), s(c.ps1[0].mul_add(c.ps1[3], -c.ps1[8]))); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[4].mul_add(c.f[11], c.f[7])), s(c.ps1[4].mul_add(c.ps1[11], c.ps1[7]))); c.f[7] = x; c.ps1[7] = y; }
                c.fcmp(0, c.f[7], c.f[6]);
                if !c.crbit(2) { pc = 0x800a2f28; continue; }
                pc = 0x800a2f20;
            }
            0x800a2f20 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x800a2f28;
            }
            0x800a2f28 => {
                { let v = fres(c.f[7]); c.f[0] = v; c.ps1[0] = v; }
                { let (x, y) = (s(c.f[0] + c.f[0]), s(c.ps1[0] + c.ps1[0])); c.f[6] = x; c.ps1[6] = y; }
                { let (x, y) = (s(c.f[0] * c.f[0]), s(c.ps1[0] * c.ps1[0])); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(-(c.f[7].mul_add(c.f[5], -c.f[6]))), s(-(c.ps1[7].mul_add(c.ps1[5], -c.ps1[6])))); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[3].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let (x, y) = (s(c.f[13] * c.f[0]), s(c.ps1[13] * c.f[0])); c.f[13] = x; c.ps1[13] = y; }
                { let ea = c.r[3].wrapping_add(0x1c); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let (x, y) = (s(c.f[12] * c.f[0]), s(c.ps1[12] * c.f[0])); c.f[12] = x; c.ps1[12] = y; }
                { let ea = c.r[3].wrapping_add(0x2c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let (x, y) = (s(c.f[11] * c.f[0]), s(c.ps1[11] * c.f[0])); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (c.f[13], c.f[12]); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[10] * c.f[0]), s(c.ps1[10] * c.f[0])); c.f[10] = x; c.ps1[10] = y; }
                { let (x, y) = (c.ps1[13], c.ps1[12]); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (s(c.f[9] * c.f[0]), s(c.ps1[9] * c.f[0])); c.f[9] = x; c.ps1[9] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[13] * c.f[1]), s(c.ps1[13] * c.ps1[1])); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[4].wrapping_add(0x10); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[8] * c.f[0]), s(c.ps1[8] * c.f[0])); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[12].mul_add(c.f[2], c.f[6])), s(c.ps1[12].mul_add(c.ps1[2], c.ps1[6]))); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[4].wrapping_add(0x20); let (x, y) = (c.f[10], c.ps1[10]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(-(c.f[11].mul_add(c.f[3], c.f[6]))), s(-(c.ps1[11].mul_add(c.ps1[3], c.ps1[6])))); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[4].wrapping_add(0x24); let (x, y) = (c.f[9], c.ps1[9]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(c.f[10] * c.f[1]), s(c.ps1[10] * c.ps1[1])); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (c.f[11], c.f[6]); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[4].wrapping_add(0x28); let (x, y) = (c.f[8], c.ps1[8]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (c.ps1[11], c.ps1[6]); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[9].mul_add(c.f[2], c.f[7])), s(c.ps1[9].mul_add(c.ps1[2], c.ps1[7]))); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[4].wrapping_add(0x18); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(-(c.f[8].mul_add(c.f[3], c.f[7]))), s(-(c.ps1[8].mul_add(c.ps1[3], c.ps1[7])))); c.f[7] = x; c.ps1[7] = y; }
                c.r[3] = 0x1;
                { let ea = c.r[4].wrapping_add(0x2c); let (x, y) = (c.f[7], c.ps1[7]); c.psq_store(ea, x, y, true, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a2eb4: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXInvXpose (0x800a2fac, 200 bytes)
#[inline(never)]
pub fn f_800a2fac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a2fac) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2fac); p = true; }
    }
    f_800a2fac_b(c, 0x800a2fac);
    if p { crate::prof::exit(c, 0x800a2fac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2fac_at(c: &mut Cpu, pc: u32) { f_800a2fac_b(c, pc) }
#[inline(always)]
fn f_800a2fac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2fac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2fac, pc); }
        match pc {
            0x800a2fac => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, true, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[3].wrapping_add(0x4); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, true, 0); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (c.ps1[1], c.f[0]); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[3].wrapping_add(0x14); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, true, 0); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (c.ps1[3], c.f[2]); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[3].wrapping_add(0x24); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[3] * c.f[6]), s(c.ps1[3] * c.ps1[6])); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (c.ps1[5], c.f[4]); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[5] * c.f[7]), s(c.ps1[5] * c.ps1[7])); c.f[13] = x; c.ps1[13] = y; }
                { let (x, y) = (s(c.f[1].mul_add(c.f[7], -c.f[11])), s(c.ps1[1].mul_add(c.ps1[7], -c.ps1[11]))); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (s(c.f[1] * c.f[8]), s(c.ps1[1] * c.ps1[8])); c.f[12] = x; c.ps1[12] = y; }
                { let (x, y) = (s(c.f[3].mul_add(c.f[8], -c.f[13])), s(c.ps1[3].mul_add(c.ps1[8], -c.ps1[13]))); c.f[13] = x; c.ps1[13] = y; }
                { let (x, y) = (s(c.f[5].mul_add(c.f[6], -c.f[12])), s(c.ps1[5].mul_add(c.ps1[6], -c.ps1[12]))); c.f[12] = x; c.ps1[12] = y; }
                { let (x, y) = (s(c.f[3] * c.f[4]), s(c.ps1[3] * c.ps1[4])); c.f[10] = x; c.ps1[10] = y; }
                { let (x, y) = (s(c.f[0] * c.f[5]), s(c.ps1[0] * c.ps1[5])); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (s(c.f[1] * c.f[2]), s(c.ps1[1] * c.ps1[2])); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[2].mul_add(c.f[5], -c.f[10])), s(c.ps1[2].mul_add(c.ps1[5], -c.ps1[10]))); c.f[10] = x; c.ps1[10] = y; }
                { let (x, y) = (s(c.f[1].mul_add(c.f[4], -c.f[9])), s(c.ps1[1].mul_add(c.ps1[4], -c.ps1[9]))); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (s(c.f[0].mul_add(c.f[3], -c.f[8])), s(c.ps1[0].mul_add(c.ps1[3], -c.ps1[8]))); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[0] * c.f[13]), s(c.ps1[0] * c.ps1[13])); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[1] - c.f[1]), s(c.ps1[1] - c.ps1[1])); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[2].mul_add(c.f[12], c.f[7])), s(c.ps1[2].mul_add(c.ps1[12], c.ps1[7]))); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[4].mul_add(c.f[11], c.f[7])), s(c.ps1[4].mul_add(c.ps1[11], c.ps1[7]))); c.f[7] = x; c.ps1[7] = y; }
                c.fcmp(0, c.f[7], c.f[1]);
                if !c.crbit(2) { pc = 0x800a3020; continue; }
                pc = 0x800a3018;
            }
            0x800a3018 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x800a3020;
            }
            0x800a3020 => {
                { let v = fres(c.f[7]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[4].wrapping_add(0xc); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(c.f[0] + c.f[0]), s(c.ps1[0] + c.ps1[0])); c.f[6] = x; c.ps1[6] = y; }
                { let (x, y) = (s(c.f[0] * c.f[0]), s(c.ps1[0] * c.ps1[0])); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[4].wrapping_add(0x1c); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(-(c.f[7].mul_add(c.f[5], -c.f[6]))), s(-(c.ps1[7].mul_add(c.ps1[5], -c.ps1[6])))); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x2c); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(c.f[13] * c.f[0]), s(c.ps1[13] * c.f[0])); c.f[13] = x; c.ps1[13] = y; }
                { let (x, y) = (s(c.f[12] * c.f[0]), s(c.ps1[12] * c.f[0])); c.f[12] = x; c.ps1[12] = y; }
                { let (x, y) = (s(c.f[11] * c.f[0]), s(c.ps1[11] * c.f[0])); c.f[11] = x; c.ps1[11] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[13], c.ps1[13]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x10); let (x, y) = (c.f[12], c.ps1[12]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[10] * c.f[0]), s(c.ps1[10] * c.f[0])); c.f[10] = x; c.ps1[10] = y; }
                { let (x, y) = (s(c.f[9] * c.f[0]), s(c.ps1[9] * c.f[0])); c.f[9] = x; c.ps1[9] = y; }
                { let ea = c.r[4].wrapping_add(0x20); let (x, y) = (c.f[11], c.ps1[11]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = (c.f[10], c.ps1[10]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(c.f[8] * c.f[0]), s(c.ps1[8] * c.f[0])); c.f[8] = x; c.ps1[8] = y; }
                c.r[3] = 0x1;
                { let ea = c.r[4].wrapping_add(0x18); let (x, y) = (c.f[9], c.ps1[9]); c.psq_store(ea, x, y, true, 0); }
                { let ea = c.r[4].wrapping_add(0x28); let (x, y) = (c.f[8], c.ps1[8]); c.psq_store(ea, x, y, true, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a2fac: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXRotRad (0x800a3074, 112 bytes)
#[inline(never)]
pub fn f_800a3074(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3074) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3074); p = true; }
    }
    f_800a3074_b(c, 0x800a3074);
    if p { crate::prof::exit(c, 0x800a3074); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3074_at(c: &mut Cpu, pc: u32) { f_800a3074_b(c, pc) }
#[inline(always)]
fn f_800a3074_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3074;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3074, pc); }
        match pc {
            0x800a3074 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[31]; c.stfd(ea, v); }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.f[31] = c.f[1];
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.f[1] = c.f[31];
                c.lr = 0x800a30a0; f_800ce6ac(c);
                pc = 0x800a30a0;
            }
            0x800a30a0 => {
                c.f[0] = c.f[1];
                c.f[1] = c.f[31];
                c.f[31] = c.f[0];
                c.lr = 0x800a30b0; f_800ce6d0(c);
                pc = 0x800a30b0;
            }
            0x800a30b0 => {
                c.f[0] = c.f[1];
                c.r[3] = c.r[30] | c.r[30];
                c.f[1] = c.f[31];
                c.r[4] = c.r[31] | c.r[31];
                c.f[2] = c.f[0];
                c.lr = 0x800a30c8; f_800a30e4(c);
                pc = 0x800a30c8;
            }
            0x800a30c8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                { let ea = c.r[1].wrapping_add(0x20); c.f[31] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a3074: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXRotTrig (0x800a30e4, 176 bytes)
#[inline(never)]
pub fn f_800a30e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a30e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a30e4); p = true; }
    }
    f_800a30e4_b(c, 0x800a30e4);
    if p { crate::prof::exit(c, 0x800a30e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a30e4_at(c: &mut Cpu, pc: u32) { f_800a30e4_b(c, pc) }
#[inline(always)]
fn f_800a30e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a30e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a30e4, pc); }
        match pc {
            0x800a30e4 => {
                { let v = s(c.f[1]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(c.f[2]); c.f[4] = v; c.ps1[4] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[2].wrapping_add(0xffff8be8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[0] = c.r[4] | 0x20;
                { let (x, y) = (-c.f[5], -c.ps1[5]); c.f[2] = x; c.ps1[2] = y; }
                c.cmpu(0, c.r[0], 0x78);
                if c.crbit(2) { pc = 0x800a3118; continue; }
                pc = 0x800a3104;
            }
            0x800a3104 => {
                c.cmpu(0, c.r[0], 0x79);
                if c.crbit(2) { pc = 0x800a3140; continue; }
                pc = 0x800a310c;
            }
            0x800a310c => {
                c.cmpu(0, c.r[0], 0x7a);
                if c.crbit(2) { pc = 0x800a316c; continue; }
                pc = 0x800a3114;
            }
            0x800a3114 => {
                pc = 0x800a3190; continue;
                pc = 0x800a3118;
            }
            0x800a3118 => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, true, 0); }
                { let ea = c.r[3].wrapping_add(0x4); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (c.f[5], c.f[4]); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[3].wrapping_add(0xc); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (c.f[4], c.f[2]); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x1c); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x2c); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, true, 0); }
                { let ea = c.r[3].wrapping_add(0x24); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x14); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                pc = 0x800a3190; continue;
                pc = 0x800a3140;
            }
            0x800a3140 => {
                { let (x, y) = (c.f[4], c.f[0]); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (c.f[0], c.f[1]); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (c.f[2], c.f[0]); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (c.f[5], c.f[0]); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                pc = 0x800a3190; continue;
                pc = 0x800a316c;
            }
            0x800a316c => {
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (c.f[5], c.f[4]); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (c.f[4], c.f[2]); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (c.f[1], c.f[0]); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                pc = 0x800a3190;
            }
            0x800a3190 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800a30e4: bad pc {:#x}", pc),
        }
    }
}

/// __PSMTXRotAxisRadInternal (0x800a3194, 176 bytes)
#[inline(never)]
pub fn f_800a3194(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3194) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3194); p = true; }
    }
    f_800a3194_b(c, 0x800a3194);
    if p { crate::prof::exit(c, 0x800a3194); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3194_at(c: &mut Cpu, pc: u32) { f_800a3194_b(c, pc) }
#[inline(always)]
fn f_800a3194_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3194;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3194, pc); }
        match pc {
            0x800a3194 => {
                { let ea = c.r[2].wrapping_add(0xffff8bf0); c.f[10] = c.lfs(ea); c.ps1[10] = c.f[10]; }
                { let ea = c.r[2].wrapping_add(0xffff8bf4); c.f[9] = c.lfs(ea); c.ps1[9] = c.f[9]; }
                { let v = s(c.f[2]); c.f[11] = v; c.ps1[11] = v; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let v = s(c.f[1]); c.f[12] = v; c.ps1[12] = v; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let (x, y) = (s(c.f[2] * c.f[2]), s(c.ps1[2] * c.ps1[2])); c.f[4] = x; c.ps1[4] = y; }
                { let v = s(c.f[10] + c.f[10]); c.f[8] = v; c.ps1[8] = v; }
                { let (x, y) = (s(c.f[3].mul_add(c.f[3], c.f[4])), s(c.ps1[3].mul_add(c.ps1[3], c.ps1[4]))); c.f[5] = x; c.ps1[5] = y; }
                { let v = s(c.f[10] - c.f[10]); c.f[1] = v; c.ps1[1] = v; }
                { let (x, y) = (s(c.f[5] + c.ps1[4]), s(c.ps1[3])); c.f[6] = x; c.ps1[6] = y; }
                { let v = s(c.f[8] - c.f[11]); c.f[0] = v; c.ps1[0] = v; }
                c.f[7] = frsqrte(c.f[6]);
                { let v = s(c.f[7] * c.f[7]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[7] * c.f[10]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(-(c.f[4].mul_add(c.f[6], -c.f[9]))); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[4] * c.f[5]); c.f[7] = v; c.ps1[7] = v; }
                { let (x, y) = (c.f[11], c.f[11]); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (s(c.f[2] * c.f[7]), s(c.ps1[2] * c.f[7])); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[3] * c.f[7]), s(c.ps1[3] * c.f[7])); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[2] * c.f[0]), s(c.ps1[2] * c.f[0])); c.f[6] = x; c.ps1[6] = y; }
                { let (x, y) = (s(c.f[2] * c.f[12]), s(c.ps1[2] * c.f[12])); c.f[10] = x; c.ps1[10] = y; }
                { let (x, y) = (s(c.f[3] * c.f[0]), s(c.ps1[3] * c.f[0])); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[6] * c.ps1[2]), s(c.ps1[6] * c.ps1[2])); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[6] * c.f[2]), s(c.ps1[6] * c.f[2])); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (s(c.f[6] * c.f[3]), s(c.ps1[6] * c.f[3])); c.f[6] = x; c.ps1[6] = y; }
                { let v = s(-(c.f[3].mul_add(c.f[12], -c.f[5]))); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[3].mul_add(c.f[12], c.f[5])); c.f[8] = v; c.ps1[8] = v; }
                { let (x, y) = (-c.f[10], -c.ps1[10]); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[6] + c.ps1[10]), s(c.ps1[1])); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (s(c.f[4] + c.ps1[11]), s(c.ps1[0])); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (s(c.f[8]), s(c.f[11] + c.ps1[5])); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[2] + c.ps1[6]), s(c.ps1[1])); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = (c.f[9], c.ps1[9]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[6] + c.ps1[2]), s(c.ps1[6])); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[7] * c.f[3]), s(c.ps1[7] * c.f[3])); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[2]), s(c.f[10] + c.ps1[6])); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[7] + c.ps1[11]), s(c.ps1[1])); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = (c.f[6], c.ps1[6]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = (c.f[7], c.ps1[7]); c.psq_store(ea, x, y, false, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3194: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXRotAxisRad (0x800a3244, 112 bytes)
#[inline(never)]
pub fn f_800a3244(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3244) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3244); p = true; }
    }
    f_800a3244_b(c, 0x800a3244);
    if p { crate::prof::exit(c, 0x800a3244); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3244_at(c: &mut Cpu, pc: u32) { f_800a3244_b(c, pc) }
#[inline(always)]
fn f_800a3244_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3244;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3244, pc); }
        match pc {
            0x800a3244 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[31]; c.stfd(ea, v); }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.f[31] = c.f[1];
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.f[1] = c.f[31];
                c.lr = 0x800a3270; f_800ce6ac(c);
                pc = 0x800a3270;
            }
            0x800a3270 => {
                c.f[0] = c.f[1];
                c.f[1] = c.f[31];
                c.f[31] = c.f[0];
                c.lr = 0x800a3280; f_800ce6d0(c);
                pc = 0x800a3280;
            }
            0x800a3280 => {
                c.f[0] = c.f[1];
                c.r[3] = c.r[30] | c.r[30];
                c.f[1] = c.f[31];
                c.r[4] = c.r[31] | c.r[31];
                c.f[2] = c.f[0];
                c.lr = 0x800a3298; f_800a3194(c);
                pc = 0x800a3298;
            }
            0x800a3298 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                { let ea = c.r[1].wrapping_add(0x20); c.f[31] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a3244: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXTrans (0x800a32b4, 52 bytes)
#[inline(never)]
pub fn f_800a32b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a32b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a32b4); p = true; }
    }
    f_800a32b4_b(c, 0x800a32b4);
    if p { crate::prof::exit(c, 0x800a32b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a32b4_at(c: &mut Cpu, pc: u32) { f_800a32b4_b(c, pc) }
#[inline(always)]
fn f_800a32b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a32b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a32b4, pc); }
        match pc {
            0x800a32b4 => {
                { let ea = c.r[2].wrapping_add(0xffff8bec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[2].wrapping_add(0xffff8be8); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[3].wrapping_add(0xc); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x1c); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x4); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x10); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x14); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x18); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x2c); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3]; let v = c.f[4]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a32b4: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXTransApply (0x800a32e8, 76 bytes)
#[inline(never)]
pub fn f_800a32e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a32e8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a32e8); p = true; }
    }
    f_800a32e8_b(c, 0x800a32e8);
    if p { crate::prof::exit(c, 0x800a32e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a32e8_at(c: &mut Cpu, pc: u32) { f_800a32e8_b(c, pc) }
#[inline(always)]
fn f_800a32e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a32e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a32e8, pc); }
        match pc {
            0x800a32e8 => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let v = s(c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let v = s(c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[7] = x; c.ps1[7] = y; }
                { let v = s(c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[8] = x; c.ps1[8] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[5]), s(c.f[1] + c.ps1[5])); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[7]), s(c.f[2] + c.ps1[7])); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[9] = x; c.ps1[9] = y; }
                { let ea = c.r[4].wrapping_add(0x10); let (x, y) = (c.f[6], c.ps1[6]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[8]), s(c.f[3] + c.ps1[8])); c.f[8] = x; c.ps1[8] = y; }
                { let ea = c.r[4].wrapping_add(0x18); let (x, y) = (c.f[7], c.ps1[7]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x20); let (x, y) = (c.f[9], c.ps1[9]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x28); let (x, y) = (c.f[8], c.ps1[8]); c.psq_store(ea, x, y, false, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a32e8: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXScale (0x800a3334, 40 bytes)
#[inline(never)]
pub fn f_800a3334(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3334) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3334); p = true; }
    }
    f_800a3334_b(c, 0x800a3334);
    if p { crate::prof::exit(c, 0x800a3334); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3334_at(c: &mut Cpu, pc: u32) { f_800a3334_b(c, pc) }
#[inline(always)]
fn f_800a3334_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3334;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3334, pc); }
        match pc {
            0x800a3334 => {
                { let ea = c.r[2].wrapping_add(0xffff8bec); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3]; let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x4); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0xc); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x14); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3334: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXScaleApply (0x800a335c, 88 bytes)
#[inline(never)]
pub fn f_800a335c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a335c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a335c); p = true; }
    }
    f_800a335c_b(c, 0x800a335c);
    if p { crate::prof::exit(c, 0x800a335c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a335c_at(c: &mut Cpu, pc: u32) { f_800a335c_b(c, pc) }
#[inline(always)]
fn f_800a335c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a335c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a335c, pc); }
        match pc {
            0x800a335c => {
                { let v = s(c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let v = s(c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let v = s(c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let (x, y) = (s(c.f[4] * c.f[1]), s(c.ps1[4] * c.f[1])); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[6] = x; c.ps1[6] = y; }
                { let (x, y) = (s(c.f[5] * c.f[1]), s(c.ps1[5] * c.f[1])); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[6] * c.f[2]), s(c.ps1[6] * c.f[2])); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[8] = x; c.ps1[8] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[7] * c.f[2]), s(c.ps1[7] * c.f[2])); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[8] * c.f[3]), s(c.ps1[8] * c.f[3])); c.f[8] = x; c.ps1[8] = y; }
                { let ea = c.r[4].wrapping_add(0x10); let (x, y) = (c.f[6], c.ps1[6]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[2] * c.f[3]), s(c.ps1[2] * c.f[3])); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x18); let (x, y) = (c.f[7], c.ps1[7]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x20); let (x, y) = (c.f[8], c.ps1[8]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x28); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a335c: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXQuat (0x800a33b4, 164 bytes)
#[inline(never)]
pub fn f_800a33b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a33b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a33b4); p = true; }
    }
    f_800a33b4_b(c, 0x800a33b4);
    if p { crate::prof::exit(c, 0x800a33b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a33b4_at(c: &mut Cpu, pc: u32) { f_800a33b4_b(c, pc) }
#[inline(always)]
fn f_800a33b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a33b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a33b4, pc); }
        match pc {
            0x800a33b4 => {
                { let ea = c.r[2].wrapping_add(0xffff8be8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let v = s(c.f[1] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[1]); c.f[2] = v; c.ps1[2] = v; }
                { let (x, y) = (s(c.f[4] * c.f[4]), s(c.ps1[4] * c.ps1[4])); c.f[6] = x; c.ps1[6] = y; }
                { let (x, y) = (c.ps1[4], c.f[4]); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (s(c.f[5].mul_add(c.f[5], c.f[6])), s(c.ps1[5].mul_add(c.ps1[5], c.ps1[6]))); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[5] * c.f[5]), s(c.ps1[5] * c.ps1[5])); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[8] + c.ps1[8]), s(c.ps1[8])); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[9] * c.ps1[5]), s(c.ps1[9] * c.ps1[5])); c.f[10] = x; c.ps1[10] = y; }
                { let v = fres(c.f[3]); c.f[11] = v; c.ps1[11] = v; }
                { let (x, y) = (s(c.f[8]), s(c.f[7] + c.ps1[6])); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(-(c.f[3].mul_add(c.f[11], -c.f[2]))), s(-(c.ps1[3].mul_add(c.ps1[11], -c.ps1[2])))); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[5] * c.ps1[5]), s(c.ps1[5] * c.ps1[5])); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[11] * c.f[3]), s(c.ps1[11] * c.ps1[3])); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[6] + c.ps1[6]), s(c.ps1[6])); c.f[6] = x; c.ps1[6] = y; }
                { let v = s(c.f[3] * c.f[2]); c.f[3] = v; c.ps1[3] = v; }
                { let (x, y) = (s(c.f[4].mul_add(c.f[9], c.f[7])), s(c.ps1[4].mul_add(c.ps1[9], c.ps1[7]))); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (s(c.f[4].mul_add(c.f[9], -c.f[7])), s(c.ps1[4].mul_add(c.ps1[9], -c.ps1[7]))); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[3].wrapping_add(0xc); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(-(c.f[6].mul_add(c.f[3], -c.f[1]))), s(-(c.ps1[6].mul_add(c.ps1[3], -c.ps1[1])))); c.f[6] = x; c.ps1[6] = y; }
                { let (x, y) = (s(-(c.f[8].mul_add(c.f[3], -c.f[1]))), s(-(c.ps1[8].mul_add(c.ps1[3], -c.ps1[1])))); c.f[8] = x; c.ps1[8] = y; }
                { let ea = c.r[3].wrapping_add(0x2c); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(c.f[11] * c.f[3]), s(c.ps1[11] * c.ps1[3])); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (s(c.f[7] * c.f[3]), s(c.ps1[7] * c.ps1[3])); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = (c.f[6], c.ps1[6]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(c.f[4].mul_add(c.f[5], c.f[10])), s(c.ps1[4].mul_add(c.f[5], c.ps1[10]))); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (c.f[11], c.f[8]); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(-(c.f[10].mul_add(c.f[2], -c.f[9]))), s(-(c.ps1[10].mul_add(c.ps1[2], -c.ps1[9])))); c.f[10] = x; c.ps1[10] = y; }
                { let (x, y) = (c.ps1[8], c.f[7]); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[9] * c.f[3]), s(c.ps1[9] * c.ps1[3])); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (s(c.f[10] * c.f[3]), s(c.ps1[10] * c.ps1[3])); c.f[10] = x; c.ps1[10] = y; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = (c.f[9], c.ps1[9]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (c.ps1[10], c.f[0]); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (c.f[10], c.ps1[9]); c.f[11] = x; c.ps1[11] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = (c.f[7], c.ps1[7]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = (c.f[11], c.ps1[11]); c.psq_store(ea, x, y, false, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a33b4: bad pc {:#x}", pc),
        }
    }
}

/// C_MTXLookAt (0x800a3458, 396 bytes)
#[inline(never)]
pub fn f_800a3458(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3458) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3458); p = true; }
    }
    f_800a3458_b(c, 0x800a3458);
    if p { crate::prof::exit(c, 0x800a3458); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3458_at(c: &mut Cpu, pc: u32) { f_800a3458_b(c, pc) }
#[inline(always)]
fn f_800a3458_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3458;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3458, pc); }
        match pc {
            0x800a3458 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x4c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x48), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x44), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = c.r[5] | c.r[5];
                { let ea = c.r[30]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x30);
                { let ea = c.r[6]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.r[3] | c.r[3];
                { let v = s(c.f[1] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[6].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x34); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[6].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x800a34b8; f_800a3adc(c);
                pc = 0x800a34b8;
            }
            0x800a34b8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[1].wrapping_add(0x30);
                c.r[5] = c.r[1].wrapping_add(0x24);
                c.lr = 0x800a34c8; f_800a3b9c(c);
                pc = 0x800a34c8;
            }
            0x800a34c8 => {
                c.r[3] = c.r[1].wrapping_add(0x24);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800a34d4; f_800a3adc(c);
                pc = 0x800a34d4;
            }
            0x800a34d4 => {
                c.r[3] = c.r[1].wrapping_add(0x30);
                c.r[4] = c.r[1].wrapping_add(0x24);
                c.r[5] = c.r[1].wrapping_add(0x18);
                c.lr = 0x800a34e4; f_800a3b9c(c);
                pc = 0x800a34e4;
            }
            0x800a34e4 => {
                { let ea = c.r[1].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29]; let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30]; c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x24); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[30].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[30].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x2c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[2] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = -c.f[0];
                { let ea = c.r[29].wrapping_add(0xc); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x10); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x1c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x14); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x20); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x18); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30]; c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[30].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x1c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[30].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x20); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[2] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = -c.f[0];
                { let ea = c.r[29].wrapping_add(0x1c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x30); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x34); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30]; c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x30); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[30].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x34); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[30].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x38); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[2] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = -c.f[0];
                { let ea = c.r[29].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x4c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x48));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[1] = c.r[1].wrapping_add(0x50);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a3458: bad pc {:#x}", pc),
        }
    }
}

/// C_MTXLightFrustum (0x800a35e4, 148 bytes)
#[inline(never)]
pub fn f_800a35e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a35e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a35e4); p = true; }
    }
    f_800a35e4_b(c, 0x800a35e4);
    if p { crate::prof::exit(c, 0x800a35e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a35e4_at(c: &mut Cpu, pc: u32) { f_800a35e4_b(c, pc) }
#[inline(always)]
fn f_800a35e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a35e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a35e4, pc); }
        match pc {
            0x800a35e4 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x38); c.f[12] = c.lfs(ea); c.ps1[12] = c.f[12]; }
                { let v = s(c.f[4] - c.f[3]); c.f[10] = v; c.ps1[10] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8be8); c.f[11] = c.lfs(ea); c.ps1[11] = c.f[11]; }
                { let v = s(c.f[1] - c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bf8); c.f[9] = c.lfs(ea); c.ps1[9] = c.f[9]; }
                { let v = s(c.f[4] + c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[11] / c.f[10]); c.f[10] = v; c.ps1[10] = v; }
                { let v = s(c.f[9] * c.f[5]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(c.f[11] / c.f[0]); c.f[9] = v; c.ps1[9] = v; }
                { let v = s(c.f[5] * c.f[10]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[10] * c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[1] + c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[6] * c.f[4]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[6] * c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[9] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3]; let v = c.f[1]; c.stfs(ea, v); }
                { let v = s(c.f[5] * c.f[9]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[2] - c.f[8]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bec); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[7] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[3]; c.stfs(ea, v); }
                { let v = s(c.f[7] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[2]; c.stfs(ea, v); }
                { let v = s(c.f[0] - c.f[12]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0xc); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x10); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x14); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x18); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x1c); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff8bfc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x2c); let v = c.f[3]; c.stfs(ea, v); }
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_800a35e4: bad pc {:#x}", pc),
        }
    }
}

/// C_MTXLightPerspective (0x800a3678, 204 bytes)
#[inline(never)]
pub fn f_800a3678(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3678) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3678); p = true; }
    }
    f_800a3678_b(c, 0x800a3678);
    if p { crate::prof::exit(c, 0x800a3678); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3678_at(c: &mut Cpu, pc: u32) { f_800a3678_b(c, pc) }
#[inline(always)]
fn f_800a3678_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3678;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3678, pc); }
        match pc {
            0x800a3678 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffa8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[27]; c.stfd(ea, v); }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.f[27] = c.f[2];
                c.r[31] = c.r[3] | c.r[3];
                c.f[28] = c.f[3];
                c.f[29] = c.f[4];
                c.f[30] = c.f[5];
                c.f[31] = c.f[6];
                { let ea = c.r[2].wrapping_add(0xffff8bf0); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[2].wrapping_add(0xffff8c00); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x800a36c8; f_800ce688(c);
                pc = 0x800a36c8;
            }
            0x800a36c8 => {
                { let ea = c.r[2].wrapping_add(0xffff8be8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.f[2] = -c.f[30];
                c.f[0] = -c.f[31];
                { let v = s(c.f[3] / c.f[1]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[4] / c.f[27]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[28] * c.f[1]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[4] * c.f[29]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[31]; let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff8bec); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[31].wrapping_add(0x4); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x8); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0xc); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x10); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x14); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x18); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x1c); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x20); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x24); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff8bfc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x2c); let v = c.f[3]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x5c));
                { let ea = c.r[1].wrapping_add(0x50); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x48); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x40); c.f[29] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x38); c.f[28] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x30); c.f[27] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[1] = c.r[1].wrapping_add(0x58);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a3678: bad pc {:#x}", pc),
        }
    }
}

/// C_MTXLightOrtho (0x800a3744, 136 bytes)
#[inline(never)]
pub fn f_800a3744(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3744) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3744); p = true; }
    }
    f_800a3744_b(c, 0x800a3744);
    if p { crate::prof::exit(c, 0x800a3744); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3744_at(c: &mut Cpu, pc: u32) { f_800a3744_b(c, pc) }
#[inline(always)]
fn f_800a3744_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3744;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3744, pc); }
        match pc {
            0x800a3744 => {
                { let v = s(c.f[4] - c.f[3]); c.f[10] = v; c.ps1[10] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8be8); c.f[11] = c.lfs(ea); c.ps1[11] = c.f[11]; }
                { let v = s(c.f[1] - c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bf8); c.f[9] = c.lfs(ea); c.ps1[9] = c.f[9]; }
                { let v = s(c.f[4] + c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[11] / c.f[10]); c.f[12] = v; c.ps1[12] = v; }
                { let v = s(c.f[11] / c.f[0]); c.f[10] = v; c.ps1[10] = v; }
                { let v = s(c.f[9] * c.f[12]); c.f[4] = v; c.ps1[4] = v; }
                c.f[3] = -c.f[3];
                { let v = s(c.f[1] + c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[4] * c.f[5]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[12] * c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                c.f[0] = -c.f[0];
                { let ea = c.r[3]; let v = c.f[1]; c.stfs(ea, v); }
                { let v = s(c.f[9] * c.f[10]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[5] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bec); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[10] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[3]; c.stfs(ea, v); }
                { let v = s(c.f[7] + c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[6]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[3]; c.stfs(ea, v); }
                { let v = s(c.f[6] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0xc); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x10); let v = c.f[3]; c.stfs(ea, v); }
                { let v = s(c.f[8] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x14); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x18); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x1c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x2c); let v = c.f[11]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3744: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXMultVec (0x800a37cc, 84 bytes)
#[inline(never)]
pub fn f_800a37cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a37cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a37cc); p = true; }
    }
    f_800a37cc_b(c, 0x800a37cc);
    if p { crate::prof::exit(c, 0x800a37cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a37cc_at(c: &mut Cpu, pc: u32) { f_800a37cc_b(c, pc) }
#[inline(always)]
fn f_800a37cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a37cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a37cc, pc); }
        match pc {
            0x800a37cc => {
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = c.psq_load(ea, true, 0); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[2] * c.f[0]), s(c.ps1[2] * c.ps1[0])); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[3].mul_add(c.f[1], c.f[4])), s(c.ps1[3].mul_add(c.ps1[1], c.ps1[4]))); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[5] + c.ps1[5]), s(c.ps1[6])); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (s(c.f[8] * c.f[0]), s(c.ps1[8] * c.ps1[0])); c.f[10] = x; c.ps1[10] = y; }
                { let ea = c.r[5].wrapping_add(0x0); let (x, y) = (c.f[6], c.ps1[6]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(c.f[9].mul_add(c.f[1], c.f[10])), s(c.ps1[9].mul_add(c.ps1[1], c.ps1[10]))); c.f[11] = x; c.ps1[11] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[11] + c.ps1[11]), s(c.ps1[12])); c.f[12] = x; c.ps1[12] = y; }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[2] * c.f[0]), s(c.ps1[2] * c.ps1[0])); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[5].wrapping_add(0x4); let (x, y) = (c.f[12], c.ps1[12]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(c.f[3].mul_add(c.f[1], c.f[4])), s(c.ps1[3].mul_add(c.ps1[1], c.ps1[4]))); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[5] + c.ps1[5]), s(c.ps1[6])); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[5].wrapping_add(0x8); let (x, y) = (c.f[6], c.ps1[6]); c.psq_store(ea, x, y, true, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a37cc: bad pc {:#x}", pc),
        }
    }
}

/// PSMTXMultVecSR (0x800a3820, 84 bytes)
#[inline(never)]
pub fn f_800a3820(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3820) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3820); p = true; }
    }
    f_800a3820_b(c, 0x800a3820);
    if p { crate::prof::exit(c, 0x800a3820); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3820_at(c: &mut Cpu, pc: u32) { f_800a3820_b(c, pc) }
#[inline(always)]
fn f_800a3820_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3820;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3820, pc); }
        match pc {
            0x800a3820 => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[0] * c.f[6]), s(c.ps1[0] * c.ps1[6])); c.f[8] = x; c.ps1[8] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (s(c.f[2] * c.f[6]), s(c.ps1[2] * c.ps1[6])); c.f[10] = x; c.ps1[10] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = c.psq_load(ea, true, 0); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[4] * c.f[6]), s(c.ps1[4] * c.ps1[6])); c.f[12] = x; c.ps1[12] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[8] + c.ps1[8]), s(c.ps1[8])); c.f[8] = x; c.ps1[8] = y; }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[10] + c.ps1[10]), s(c.ps1[10])); c.f[10] = x; c.ps1[10] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[12] + c.ps1[12]), s(c.ps1[12])); c.f[12] = x; c.ps1[12] = y; }
                { let (x, y) = (s(c.f[1].mul_add(c.f[7], c.f[8])), s(c.ps1[1].mul_add(c.ps1[7], c.ps1[8]))); c.f[9] = x; c.ps1[9] = y; }
                { let ea = c.r[5].wrapping_add(0x0); let (x, y) = (c.f[9], c.ps1[9]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(c.f[3].mul_add(c.f[7], c.f[10])), s(c.ps1[3].mul_add(c.ps1[7], c.ps1[10]))); c.f[11] = x; c.ps1[11] = y; }
                { let ea = c.r[5].wrapping_add(0x4); let (x, y) = (c.f[11], c.ps1[11]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (s(c.f[5].mul_add(c.f[7], c.f[12])), s(c.ps1[5].mul_add(c.ps1[7], c.ps1[12]))); c.f[13] = x; c.ps1[13] = y; }
                { let ea = c.r[5].wrapping_add(0x8); let (x, y) = (c.f[13], c.ps1[13]); c.psq_store(ea, x, y, true, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3820: bad pc {:#x}", pc),
        }
    }
}

/// C_MTXFrustum (0x800a3874, 156 bytes)
#[inline(never)]
pub fn f_800a3874(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3874) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3874); p = true; }
    }
    f_800a3874_b(c, 0x800a3874);
    if p { crate::prof::exit(c, 0x800a3874); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3874_at(c: &mut Cpu, pc: u32) { f_800a3874_b(c, pc) }
#[inline(always)]
fn f_800a3874_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3874;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3874, pc); }
        match pc {
            0x800a3874 => {
                { let v = s(c.f[4] - c.f[3]); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c08); c.f[10] = c.lfs(ea); c.ps1[10] = c.f[10]; }
                { let v = s(c.f[1] - c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c0c); c.f[8] = c.lfs(ea); c.ps1[8] = c.f[8]; }
                { let v = s(c.f[6] - c.f[5]); c.f[7] = v; c.ps1[7] = v; }
                { let v = s(c.f[10] / c.f[9]); c.f[11] = v; c.ps1[11] = v; }
                { let v = s(c.f[8] * c.f[5]); c.f[8] = v; c.ps1[8] = v; }
                { let v = s(c.f[10] / c.f[0]); c.f[9] = v; c.ps1[9] = v; }
                { let v = s(c.f[8] * c.f[11]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[4] + c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[1] + c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3]; let v = c.f[0]; c.stfs(ea, v); }
                { let v = s(c.f[6] * c.f[5]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[10] / c.f[7]); c.f[6] = v; c.ps1[6] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c10); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[4]; c.stfs(ea, v); }
                { let v = s(c.f[11] * c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                c.f[1] = -c.f[5];
                c.f[0] = -c.f[0];
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[3]; c.stfs(ea, v); }
                { let v = s(c.f[8] * c.f[9]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[9] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0xc); let v = c.f[4]; c.stfs(ea, v); }
                { let v = s(c.f[1] * c.f[6]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[3].wrapping_add(0x10); let v = c.f[4]; c.stfs(ea, v); }
                { let v = s(c.f[6] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x14); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x18); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x1c); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x30); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x34); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff8c14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x38); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x3c); let v = c.f[4]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3874: bad pc {:#x}", pc),
        }
    }
}

/// C_MTXPerspective (0x800a3910, 208 bytes)
#[inline(never)]
pub fn f_800a3910(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3910) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3910); p = true; }
    }
    f_800a3910_b(c, 0x800a3910);
    if p { crate::prof::exit(c, 0x800a3910); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3910_at(c: &mut Cpu, pc: u32) { f_800a3910_b(c, pc) }
#[inline(always)]
fn f_800a3910_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3910;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3910, pc); }
        match pc {
            0x800a3910 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[31]; c.stfd(ea, v); }
                c.f[31] = c.f[4];
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[30]; c.stfd(ea, v); }
                c.f[30] = c.f[3];
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[29]; c.stfd(ea, v); }
                c.f[29] = c.f[2];
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                { let ea = c.r[2].wrapping_add(0xffff8c18); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[2].wrapping_add(0xffff8c1c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[5] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x800a3950; f_800ce688(c);
                pc = 0x800a3950;
            }
            0x800a3950 => {
                { let ea = c.r[2].wrapping_add(0xffff8c08); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[31] - c.f[30]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[31] * c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[3] / c.f[1]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[4] / c.f[29]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[31]; let v = c.f[1]; c.stfs(ea, v); }
                { let v = s(c.f[3] / c.f[2]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c10); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[31].wrapping_add(0x4); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x8); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0xc); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x10); let v = c.f[2]; c.stfs(ea, v); }
                c.f[1] = -c.f[30];
                c.f[0] = -c.f[0];
                { let ea = c.r[31].wrapping_add(0x14); let v = c.f[4]; c.stfs(ea, v); }
                { let v = s(c.f[1] * c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[31].wrapping_add(0x18); let v = c.f[2]; c.stfs(ea, v); }
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31].wrapping_add(0x1c); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x20); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x24); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x28); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x30); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x34); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff8c14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0x38); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x3c); let v = c.f[2]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                { let ea = c.r[1].wrapping_add(0x38); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x30); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x28); c.f[29] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a3910: bad pc {:#x}", pc),
        }
    }
}

/// C_MTXOrtho (0x800a39e0, 152 bytes)
#[inline(never)]
pub fn f_800a39e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a39e0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a39e0); p = true; }
    }
    f_800a39e0_b(c, 0x800a39e0);
    if p { crate::prof::exit(c, 0x800a39e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a39e0_at(c: &mut Cpu, pc: u32) { f_800a39e0_b(c, pc) }
#[inline(always)]
fn f_800a39e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a39e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a39e0, pc); }
        match pc {
            0x800a39e0 => {
                { let v = s(c.f[4] - c.f[3]); c.f[8] = v; c.ps1[8] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c08); c.f[9] = c.lfs(ea); c.ps1[9] = c.f[9]; }
                { let v = s(c.f[1] - c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c0c); c.f[7] = c.lfs(ea); c.ps1[7] = c.f[7]; }
                { let v = s(c.f[4] + c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[9] / c.f[8]); c.f[10] = v; c.ps1[10] = v; }
                { let v = s(c.f[9] / c.f[0]); c.f[8] = v; c.ps1[8] = v; }
                { let v = s(c.f[7] * c.f[10]); c.f[4] = v; c.ps1[4] = v; }
                c.f[3] = -c.f[3];
                { let v = s(c.f[6] - c.f[5]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3]; let v = c.f[4]; c.stfs(ea, v); }
                { let v = s(c.f[1] + c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[10] * c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c10); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.f[1] = -c.f[1];
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[3]; c.stfs(ea, v); }
                { let v = s(c.f[9] / c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0xc); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x10); let v = c.f[3]; c.stfs(ea, v); }
                { let v = s(c.f[7] * c.f[8]); c.f[2] = v; c.ps1[2] = v; }
                c.f[0] = -c.f[6];
                { let v = s(c.f[8] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[3].wrapping_add(0x14); let v = c.f[2]; c.stfs(ea, v); }
                { let v = s(c.f[0] * c.f[4]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x18); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x1c); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff8c14); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[1] * c.f[4]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x30); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x34); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x38); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x3c); let v = c.f[9]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a39e0: bad pc {:#x}", pc),
        }
    }
}

/// PSVECAdd (0x800a3a78, 36 bytes)
#[inline(never)]
pub fn f_800a3a78(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3a78) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3a78); p = true; }
    }
    f_800a3a78_b(c, 0x800a3a78);
    if p { crate::prof::exit(c, 0x800a3a78); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3a78_at(c: &mut Cpu, pc: u32) { f_800a3a78_b(c, pc) }
#[inline(always)]
fn f_800a3a78_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3a78;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3a78, pc); }
        match pc {
            0x800a3a78 => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (s(c.f[2] + c.f[4]), s(c.ps1[2] + c.ps1[4])); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[5].wrapping_add(0x0); let (x, y) = (c.f[6], c.ps1[6]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, true, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = c.psq_load(ea, true, 0); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[3] + c.f[5]), s(c.ps1[3] + c.ps1[5])); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[5].wrapping_add(0x8); let (x, y) = (c.f[7], c.ps1[7]); c.psq_store(ea, x, y, true, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3a78: bad pc {:#x}", pc),
        }
    }
}

/// PSVECSubtract (0x800a3a9c, 36 bytes)
#[inline(never)]
pub fn f_800a3a9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3a9c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3a9c); p = true; }
    }
    f_800a3a9c_b(c, 0x800a3a9c);
    if p { crate::prof::exit(c, 0x800a3a9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3a9c_at(c: &mut Cpu, pc: u32) { f_800a3a9c_b(c, pc) }
#[inline(always)]
fn f_800a3a9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3a9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3a9c, pc); }
        match pc {
            0x800a3a9c => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (s(c.f[2] - c.f[4]), s(c.ps1[2] - c.ps1[4])); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[5].wrapping_add(0x0); let (x, y) = (c.f[6], c.ps1[6]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, true, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = c.psq_load(ea, true, 0); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[3] - c.f[5]), s(c.ps1[3] - c.ps1[5])); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[5].wrapping_add(0x8); let (x, y) = (c.f[7], c.ps1[7]); c.psq_store(ea, x, y, true, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3a9c: bad pc {:#x}", pc),
        }
    }
}

/// PSVECScale (0x800a3ac0, 28 bytes)
#[inline(never)]
pub fn f_800a3ac0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3ac0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3ac0); p = true; }
    }
    f_800a3ac0_b(c, 0x800a3ac0);
    if p { crate::prof::exit(c, 0x800a3ac0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3ac0_at(c: &mut Cpu, pc: u32) { f_800a3ac0_b(c, pc) }
#[inline(always)]
fn f_800a3ac0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3ac0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3ac0, pc); }
        match pc {
            0x800a3ac0 => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, true, 0); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[0] * c.f[1]), s(c.ps1[0] * c.f[1])); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[2] * c.f[1]), s(c.ps1[2] * c.f[1])); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, true, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3ac0: bad pc {:#x}", pc),
        }
    }
}

/// PSVECNormalize (0x800a3adc, 68 bytes)
#[inline(never)]
pub fn f_800a3adc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3adc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3adc); p = true; }
    }
    f_800a3adc_b(c, 0x800a3adc);
    if p { crate::prof::exit(c, 0x800a3adc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3adc_at(c: &mut Cpu, pc: u32) { f_800a3adc_b(c, pc) }
#[inline(always)]
fn f_800a3adc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3adc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3adc, pc); }
        match pc {
            0x800a3adc => {
                { let ea = c.r[2].wrapping_add(0xffff8c20); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[2].wrapping_add(0xffff8c24); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[2] * c.f[2]), s(c.ps1[2] * c.ps1[2])); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, true, 0); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[3].mul_add(c.f[3], c.f[5])), s(c.ps1[3].mul_add(c.ps1[3], c.ps1[5]))); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (s(c.f[4] + c.ps1[5]), s(c.ps1[3])); c.f[4] = x; c.ps1[4] = y; }
                c.f[5] = frsqrte(c.f[4]);
                { let v = s(c.f[5] * c.f[5]); c.f[6] = v; c.ps1[6] = v; }
                { let v = s(c.f[5] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(-(c.f[6].mul_add(c.f[4], -c.f[1]))); c.f[6] = v; c.ps1[6] = v; }
                { let v = s(c.f[6] * c.f[0]); c.f[5] = v; c.ps1[5] = v; }
                { let (x, y) = (s(c.f[2] * c.f[5]), s(c.ps1[2] * c.f[5])); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[3] * c.f[5]), s(c.ps1[3] * c.f[5])); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, true, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3adc: bad pc {:#x}", pc),
        }
    }
}

/// PSVECSquareMag (0x800a3b20, 24 bytes)
#[inline(never)]
pub fn f_800a3b20(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3b20) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3b20); p = true; }
    }
    f_800a3b20_b(c, 0x800a3b20);
    if p { crate::prof::exit(c, 0x800a3b20); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3b20_at(c: &mut Cpu, pc: u32) { f_800a3b20_b(c, pc) }
#[inline(always)]
fn f_800a3b20_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3b20;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3b20, pc); }
        match pc {
            0x800a3b20 => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let (x, y) = (s(c.f[0] * c.f[0]), s(c.ps1[0] * c.ps1[0])); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let (x, y) = (s(c.f[1].mul_add(c.f[1], c.f[0])), s(c.ps1[1].mul_add(c.ps1[1], c.ps1[0]))); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[1] + c.ps1[0]), s(c.ps1[0])); c.f[1] = x; c.ps1[1] = y; }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3b20: bad pc {:#x}", pc),
        }
    }
}

/// PSVECMag (0x800a3b38, 68 bytes)
#[inline(never)]
pub fn f_800a3b38(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3b38) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3b38); p = true; }
    }
    f_800a3b38_b(c, 0x800a3b38);
    if p { crate::prof::exit(c, 0x800a3b38); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3b38_at(c: &mut Cpu, pc: u32) { f_800a3b38_b(c, pc) }
#[inline(always)]
fn f_800a3b38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3b38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3b38, pc); }
        match pc {
            0x800a3b38 => {
                { let ea = c.r[2].wrapping_add(0xffff8c20); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let (x, y) = (s(c.f[0] * c.f[0]), s(c.ps1[0] * c.ps1[0])); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[4] - c.f[4]); c.f[2] = v; c.ps1[2] = v; }
                { let (x, y) = (s(c.f[1].mul_add(c.f[1], c.f[0])), s(c.ps1[1].mul_add(c.ps1[1], c.ps1[0]))); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[1] + c.ps1[0]), s(c.ps1[0])); c.f[1] = x; c.ps1[1] = y; }
                c.fcmp(0, c.f[1], c.f[2]);
                if c.crbit(2) { pc = 0x800a3b78; continue; }
                pc = 0x800a3b5c;
            }
            0x800a3b5c => {
                c.f[0] = frsqrte(c.f[1]);
                { let ea = c.r[2].wrapping_add(0xffff8c24); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[0] * c.f[0]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[0] * c.f[4]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(-(c.f[2].mul_add(c.f[1], -c.f[3]))); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[2] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x800a3b78;
            }
            0x800a3b78 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800a3b38: bad pc {:#x}", pc),
        }
    }
}

/// PSVECDotProduct (0x800a3b7c, 32 bytes)
#[inline(never)]
pub fn f_800a3b7c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3b7c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3b7c); p = true; }
    }
    f_800a3b7c_b(c, 0x800a3b7c);
    if p { crate::prof::exit(c, 0x800a3b7c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3b7c_at(c: &mut Cpu, pc: u32) { f_800a3b7c_b(c, pc) }
#[inline(always)]
fn f_800a3b7c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3b7c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3b7c, pc); }
        match pc {
            0x800a3b7c => {
                { let ea = c.r[3].wrapping_add(0x4); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x4); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[2] * c.f[3]), s(c.ps1[2] * c.ps1[3])); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (s(c.f[5].mul_add(c.f[4], c.f[2])), s(c.ps1[5].mul_add(c.ps1[4], c.ps1[2]))); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (s(c.f[3] + c.ps1[2]), s(c.ps1[2])); c.f[1] = x; c.ps1[1] = y; }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3b7c: bad pc {:#x}", pc),
        }
    }
}

/// PSVECCrossProduct (0x800a3b9c, 60 bytes)
#[inline(never)]
pub fn f_800a3b9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3b9c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3b9c); p = true; }
    }
    f_800a3b9c_b(c, 0x800a3b9c);
    if p { crate::prof::exit(c, 0x800a3b9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3b9c_at(c: &mut Cpu, pc: u32) { f_800a3b9c_b(c, pc) }
#[inline(always)]
fn f_800a3b9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3b9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3b9c, pc); }
        match pc {
            0x800a3b9c => {
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let (x, y) = (c.ps1[1], c.f[1]); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let (x, y) = (s(c.f[1] * c.f[2]), s(c.ps1[1] * c.ps1[2])); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (s(c.f[1] * c.f[0]), s(c.ps1[1] * c.f[0])); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[0].mul_add(c.f[3], -c.f[4])), s(c.ps1[0].mul_add(c.ps1[3], -c.ps1[4]))); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[0].mul_add(c.f[6], -c.f[7])), s(c.ps1[0].mul_add(c.ps1[6], -c.ps1[7]))); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (c.ps1[5], c.ps1[5]); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (c.f[5], c.ps1[8]); c.f[10] = x; c.ps1[10] = y; }
                { let ea = c.r[5].wrapping_add(0x0); let (x, y) = (c.f[9], c.ps1[9]); c.psq_store(ea, x, y, true, 0); }
                { let (x, y) = (-c.f[10], -c.ps1[10]); c.f[10] = x; c.ps1[10] = y; }
                { let ea = c.r[5].wrapping_add(0x4); let (x, y) = (c.f[10], c.ps1[10]); c.psq_store(ea, x, y, false, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3b9c: bad pc {:#x}", pc),
        }
    }
}

/// PSVECSquareDistance (0x800a3bd8, 40 bytes)
#[inline(never)]
pub fn f_800a3bd8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3bd8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3bd8); p = true; }
    }
    f_800a3bd8_b(c, 0x800a3bd8);
    if p { crate::prof::exit(c, 0x800a3bd8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3bd8_at(c: &mut Cpu, pc: u32) { f_800a3bd8_b(c, pc) }
#[inline(always)]
fn f_800a3bd8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3bd8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3bd8, pc); }
        match pc {
            0x800a3bd8 => {
                { let ea = c.r[3].wrapping_add(0x4); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x4); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[0] - c.f[1]), s(c.ps1[0] - c.ps1[1])); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[2] * c.f[2]), s(c.ps1[2] * c.ps1[2])); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[0] - c.f[1]), s(c.ps1[0] - c.ps1[1])); c.f[0] = x; c.ps1[0] = y; }
                { let (x, y) = (s(c.f[0].mul_add(c.f[0], c.f[2])), s(c.ps1[0].mul_add(c.ps1[0], c.ps1[2]))); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[1] + c.ps1[2]), s(c.ps1[2])); c.f[1] = x; c.ps1[1] = y; }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3bd8: bad pc {:#x}", pc),
        }
    }
}

/// PSVECDistance (0x800a3c00, 84 bytes)
#[inline(never)]
pub fn f_800a3c00(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3c00) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3c00); p = true; }
    }
    f_800a3c00_b(c, 0x800a3c00);
    if p { crate::prof::exit(c, 0x800a3c00); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3c00_at(c: &mut Cpu, pc: u32) { f_800a3c00_b(c, pc) }
#[inline(always)]
fn f_800a3c00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3c00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3c00, pc); }
        match pc {
            0x800a3c00 => {
                { let ea = c.r[3].wrapping_add(0x4); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x4); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[0] - c.f[1]), s(c.ps1[0] - c.ps1[1])); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[2] * c.f[2]), s(c.ps1[2] * c.ps1[2])); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[0] - c.f[1]), s(c.ps1[0] - c.ps1[1])); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[2].wrapping_add(0xffff8c20); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let (x, y) = (s(c.f[0].mul_add(c.f[0], c.f[2])), s(c.ps1[0].mul_add(c.ps1[0], c.ps1[2]))); c.f[1] = x; c.ps1[1] = y; }
                { let v = s(c.f[3] - c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                { let (x, y) = (s(c.f[1] + c.ps1[2]), s(c.ps1[2])); c.f[1] = x; c.ps1[1] = y; }
                c.fcmp(0, c.f[0], c.f[1]);
                if c.crbit(2) { pc = 0x800a3c50; continue; }
                pc = 0x800a3c34;
            }
            0x800a3c34 => {
                { let ea = c.r[2].wrapping_add(0xffff8c24); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.f[0] = frsqrte(c.f[1]);
                { let v = s(c.f[0] * c.f[0]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[0] * c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(-(c.f[2].mul_add(c.f[1], -c.f[4]))); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[2] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x800a3c50;
            }
            0x800a3c50 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800a3c00: bad pc {:#x}", pc),
        }
    }
}

/// PSQUATMultiply (0x800a3c54, 92 bytes)
#[inline(never)]
pub fn f_800a3c54(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3c54) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3c54); p = true; }
    }
    f_800a3c54_b(c, 0x800a3c54);
    if p { crate::prof::exit(c, 0x800a3c54); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3c54_at(c: &mut Cpu, pc: u32) { f_800a3c54_b(c, pc) }
#[inline(always)]
fn f_800a3c54_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3c54;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3c54, pc); }
        match pc {
            0x800a3c54 => {
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (-c.f[0], -c.ps1[0]); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[4].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (-c.f[1], -c.ps1[1]); c.f[6] = x; c.ps1[6] = y; }
                { let (x, y) = (c.f[5], c.ps1[0]); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (s(c.f[1] * c.f[2]), s(c.ps1[1] * c.f[2])); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[5] * c.f[2]), s(c.ps1[5] * c.f[2])); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (c.f[6], c.ps1[1]); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (s(c.f[4] * c.ps1[2]), s(c.ps1[4] * c.ps1[2])); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (s(c.f[4].mul_add(c.f[3], c.f[7])), s(c.ps1[4].mul_add(c.f[3], c.ps1[7]))); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[1] * c.ps1[2]), s(c.ps1[1] * c.ps1[2])); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (s(c.f[1].mul_add(c.f[3], c.f[5])), s(c.ps1[1].mul_add(c.f[3], c.ps1[5]))); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[6].mul_add(c.ps1[3], c.f[8])), s(c.ps1[6].mul_add(c.ps1[3], c.ps1[8]))); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (c.ps1[7], c.f[7]); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (s(c.f[0].mul_add(c.ps1[3], c.f[2])), s(c.ps1[0].mul_add(c.ps1[3], c.ps1[2]))); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (c.ps1[5], c.f[5]); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (s(c.f[7] + c.f[2]), s(c.ps1[7] + c.ps1[2])); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[5].wrapping_add(0x0); let (x, y) = (c.f[7], c.ps1[7]); c.psq_store(ea, x, y, false, 0); }
                { let (x, y) = (s(c.f[5] - c.f[8]), s(c.ps1[5] - c.ps1[8])); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[5].wrapping_add(0x8); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                if true { return; }
                return;
            }
            _ => panic!("f_800a3c54: bad pc {:#x}", pc),
        }
    }
}

/// C_QUATRotAxisRad (0x800a3cb0, 140 bytes)
#[inline(never)]
pub fn f_800a3cb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3cb0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3cb0); p = true; }
    }
    f_800a3cb0_b(c, 0x800a3cb0);
    if p { crate::prof::exit(c, 0x800a3cb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3cb0_at(c: &mut Cpu, pc: u32) { f_800a3cb0_b(c, pc) }
#[inline(always)]
fn f_800a3cb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3cb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3cb0, pc); }
        match pc {
            0x800a3cb0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[30]; c.stfd(ea, v); }
                c.f[30] = c.f[1];
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[4].wrapping_add(0x0);
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.lr = 0x800a3cdc; f_800a3adc(c);
                pc = 0x800a3cdc;
            }
            0x800a3cdc => {
                { let ea = c.r[2].wrapping_add(0xffff8c30); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] * c.f[30]); c.f[30] = v; c.ps1[30] = v; }
                c.f[1] = c.f[30];
                c.lr = 0x800a3cec; f_800ce6ac(c);
                pc = 0x800a3cec;
            }
            0x800a3cec => {
                c.f[31] = c.f[1];
                c.f[1] = c.f[30];
                c.lr = 0x800a3cf8; f_800ce6d0(c);
                pc = 0x800a3cf8;
            }
            0x800a3cf8 => {
                { let ea = c.r[1].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[31] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31]; let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[31] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x1c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[31] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0xc); let v = c.f[1]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                { let ea = c.r[1].wrapping_add(0x30); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x28); c.f[30] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a3cb0: bad pc {:#x}", pc),
        }
    }
}

/// C_QUATSlerp (0x800a3d3c, 372 bytes)
#[inline(never)]
pub fn f_800a3d3c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3d3c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3d3c); p = true; }
    }
    f_800a3d3c_b(c, 0x800a3d3c);
    if p { crate::prof::exit(c, 0x800a3d3c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3d3c_at(c: &mut Cpu, pc: u32) { f_800a3d3c_b(c, pc) }
#[inline(always)]
fn f_800a3d3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3d3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3d3c, pc); }
        match pc {
            0x800a3d3c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x48); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[29]; c.stfd(ea, v); }
                c.f[29] = c.f[1];
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[27]; c.stfd(ea, v); }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                { let ea = c.r[3]; c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[4]; c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[3].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[4] * c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[3].wrapping_add(0xc); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                { let v = s(c.f[4] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[4].wrapping_add(0xc); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let v = s(c.f[3] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[6] * c.f[5]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c2c); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                { let v = s(c.f[2] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[3] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(0) { pc = 0x800a3dcc; continue; }
                pc = 0x800a3dc4;
            }
            0x800a3dc4 => {
                c.f[1] = -c.f[1];
                c.f[30] = -c.f[30];
                pc = 0x800a3dcc;
            }
            0x800a3dcc => {
                { let ea = c.r[2].wrapping_add(0xffff8c34); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                { let (a, b) = (c.crbit(0), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x800a3e14; continue; }
                pc = 0x800a3ddc;
            }
            0x800a3ddc => {
                c.lr = 0x800a3de0; f_800ce6f4(c);
                pc = 0x800a3de0;
            }
            0x800a3de0 => {
                c.f[27] = c.f[1];
                c.lr = 0x800a3de8; f_800ce6ac(c);
                pc = 0x800a3de8;
            }
            0x800a3de8 => {
                { let ea = c.r[2].wrapping_add(0xffff8c2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[28] = c.f[1];
                { let v = s(c.f[0] - c.f[29]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] * c.f[27]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x800a3dfc; f_800ce6ac(c);
                pc = 0x800a3dfc;
            }
            0x800a3dfc => {
                { let v = s(c.f[1] / c.f[28]); c.f[31] = v; c.ps1[31] = v; }
                { let v = s(c.f[29] * c.f[27]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x800a3e08; f_800ce6ac(c);
                pc = 0x800a3e08;
            }
            0x800a3e08 => {
                { let v = s(c.f[1] / c.f[28]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[30] * c.f[0]); c.f[30] = v; c.ps1[30] = v; }
                pc = 0x800a3e20; continue;
                pc = 0x800a3e14;
            }
            0x800a3e14 => {
                { let ea = c.r[2].wrapping_add(0xffff8c2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[30] * c.f[29]); c.f[30] = v; c.ps1[30] = v; }
                { let v = s(c.f[0] - c.f[29]); c.f[31] = v; c.ps1[31] = v; }
                pc = 0x800a3e20;
            }
            0x800a3e20 => {
                { let ea = c.r[29]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[30]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[31] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[30] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31]; let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[30].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[31] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[30] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[30].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[31] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[30] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[30].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[31] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[30] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31].wrapping_add(0xc); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                { let ea = c.r[1].wrapping_add(0x48); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x40); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x38); c.f[29] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x30); c.f[28] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x28); c.f[27] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x50);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a3d3c: bad pc {:#x}", pc),
        }
    }
}

/// __DVDInitWA (0x800a3eb0, 64 bytes)
#[inline(never)]
pub fn f_800a3eb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3eb0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3eb0); p = true; }
    }
    f_800a3eb0_b(c, 0x800a3eb0);
    if p { crate::prof::exit(c, 0x800a3eb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3eb0_at(c: &mut Cpu, pc: u32) { f_800a3eb0_b(c, pc) }
#[inline(always)]
fn f_800a3eb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3eb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3eb0, pc); }
        match pc {
            0x800a3eb0 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = 0x0;
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[0]);
                c.r[0] = 0xffffffff;
                c.stw(c.r[3].wrapping_add(0xffffc290), c.r[0]);
                c.r[3] = 0x0;
                c.lr = 0x800a3edc; f_800a4cac(c);
                pc = 0x800a3edc;
            }
            0x800a3edc => {
                c.lr = 0x800a3ee0; f_8009a27c(c);
                pc = 0x800a3ee0;
            }
            0x800a3ee0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a3eb0: bad pc {:#x}", pc),
        }
    }
}

/// __DVDInterruptHandler (0x800a3ef0, 736 bytes)
#[inline(never)]
pub fn f_800a3ef0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a3ef0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a3ef0); p = true; }
    }
    f_800a3ef0_b(c, 0x800a3ef0);
    if p { crate::prof::exit(c, 0x800a3ef0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a3ef0_at(c: &mut Cpu, pc: u32) { f_800a3ef0_b(c, pc) }
#[inline(never)]
fn f_800a3ef0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a3ef0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a3ef0, pc); }
        match pc {
            0x800a3ef0 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffd10); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x2dc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[4].wrapping_add(0x0);
                c.r[30] = c.r[3].wrapping_add(0xffffc290);
                c.r[29] = 0x0;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fa0));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a3f58; continue; }
                pc = 0x800a3f1c;
            }
            0x800a3f1c => {
                c.lr = 0x800a3f20; f_800a2798(c);
                pc = 0x800a3f20;
            }
            0x800a3f20 => {
                c.stw(c.r[13].wrapping_add(0xffff9f94), c.r[4]);
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9f90), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffff8198), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc4));
                c.stw(c.r[30].wrapping_add(0xb8), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc8));
                c.stw(c.r[30].wrapping_add(0xbc), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xcc));
                c.stw(c.r[30].wrapping_add(0xc0), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f60));
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800a3f58; continue; }
                pc = 0x800a3f54;
            }
            0x800a3f54 => {
                c.r[29] = c.r[29] | 0x8;
                pc = 0x800a3f58;
            }
            0x800a3f58 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fa0), c.r[0]);
                c.r[3] = 0xcc000000;
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6000));
                c.r[31] = c.r[0] & 0x2a;
                c.rc(c.r[31]);
                c.r[3] = c.r[0] & 0x54;
                c.rc(c.r[3]);
                c.r[0] = c.r[31].rotate_left(1) & 0xfffffffe;
                c.r[27] = c.r[3] & c.r[0];
                c.r[0] = c.r[27].rotate_left(0) & 0x40;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a3f88; continue; }
                pc = 0x800a3f84;
            }
            0x800a3f84 => {
                c.r[29] = c.r[29] | 0x8;
                pc = 0x800a3f88;
            }
            0x800a3f88 => {
                c.r[0] = c.r[27].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a3f94; continue; }
                pc = 0x800a3f90;
            }
            0x800a3f90 => {
                c.r[29] = c.r[29] | 0x1;
                pc = 0x800a3f94;
            }
            0x800a3f94 => {
                c.r[0] = c.r[27].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a3fa0; continue; }
                pc = 0x800a3f9c;
            }
            0x800a3f9c => {
                c.r[29] = c.r[29] | 0x2;
                pc = 0x800a3fa0;
            }
            0x800a3fa0 => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x800a3fb8; continue; }
                pc = 0x800a3fa8;
            }
            0x800a3fa8 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9f78), c.r[0]);
                c.r[3] = c.r[30].wrapping_add(0x68);
                c.lr = 0x800a3fb8; f_8009a590(c);
                pc = 0x800a3fb8;
            }
            0x800a3fb8 => {
                c.r[0] = c.r[27] | c.r[31];
                c.r[31] = 0xcc000000;
                c.stw(c.r[31].wrapping_add(0x6000), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f78));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a406c; continue; }
                pc = 0x800a3fd0;
            }
            0x800a3fd0 => {
                c.lr = 0x800a3fd4; f_800a2798(c);
                pc = 0x800a3fd4;
            }
            0x800a3fd4 => {
                c.r[5] = 0x80000000;
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff9f70));
                c.r[6] = c.lwz(c.r[5].wrapping_add(0xf8));
                c.r[5] = 0x10620000;
                c.r[5] = c.r[5].wrapping_add(0x4dd3);
                c.r[8] = c.lwz(c.r[13].wrapping_add(0xffff9f74));
                c.r[6] = c.r[6].rotate_left(30) & 0x3fffffff;
                c.r[5] = mulhwu(c.r[5], c.r[6]);
                c.r[5] = c.r[5].rotate_left(26) & 0x3ffffff;
                c.r[5] = c.r[5].wrapping_mul(0xc8);
                c.r[6] = c.addc(!c.r[8], c.r[4], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[7], c.r[3], ci); }
                c.r[0] = 0x0;
                c.r[4] = c.r[3] ^ 0x80000000;
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[5], c.r[6], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[4], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[4], c.r[4], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x800a406c; continue; }
                pc = 0x800a4024;
            }
            0x800a4024 => {
                c.r[27] = c.r[31].wrapping_add(0x6000);
                { let ea = c.r[27].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[27] = ea; }
                c.r[3] = c.r[0].rotate_left(0) & 0x4;
                c.r[0] = c.r[0].rotate_left(1) & 0x4;
                c.r[0] = c.r[3] & c.r[0];
                c.r[0] = c.r[0].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a4060; continue; }
                pc = 0x800a4040;
            }
            0x800a4040 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9f6c));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a4058; continue; }
                pc = 0x800a404c;
            }
            0x800a404c => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0x4;
                if true { let t = c.lr; c.lr = 0x800a4058; (c.dispatch)(c, t); }
                pc = 0x800a4058;
            }
            0x800a4058 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9f6c), c.r[0]);
                pc = 0x800a4060;
            }
            0x800a4060 => {
                c.r[0] = c.lwz(c.r[27]);
                c.stw(c.r[27], c.r[0]);
                pc = 0x800a40c0; continue;
                pc = 0x800a406c;
            }
            0x800a406c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f7c));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a40b4; continue; }
                pc = 0x800a4078;
            }
            0x800a4078 => {
                c.r[3] = 0xcc000000;
                c.r[5] = c.r[3].wrapping_add(0x6000);
                { let ea = c.r[5].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[5] = ea; }
                c.r[4] = c.r[0].rotate_left(0) & 0x2;
                c.r[3] = c.r[0].rotate_left(0) & 0x4;
                c.r[0] = c.r[0].rotate_left(1) & 0x4;
                c.r[3] = c.r[3] & c.r[0];
                c.r[0] = c.r[3].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a40a0; continue; }
                pc = 0x800a409c;
            }
            0x800a409c => {
                c.r[29] = c.r[29] | 0x4;
                pc = 0x800a40a0;
            }
            0x800a40a0 => {
                c.r[0] = c.r[3] | c.r[4];
                c.stw(c.r[5], c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9f7c), c.r[0]);
                pc = 0x800a40c0; continue;
                pc = 0x800a40b4;
            }
            0x800a40b4 => {
                c.r[3] = 0xcc000000;
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x6004), c.r[0]);
                pc = 0x800a40c0;
            }
            0x800a40c0 => {
                c.r[0] = c.r[29].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a40d8; continue; }
                pc = 0x800a40c8;
            }
            0x800a40c8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f80));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a40d8; continue; }
                pc = 0x800a40d4;
            }
            0x800a40d4 => {
                c.r[29] = c.r[29].rotate_left(0) & 0xfffffff7;
                pc = 0x800a40d8;
            }
            0x800a40d8 => {
                c.r[0] = c.r[29].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a415c; continue; }
                pc = 0x800a40e0;
            }
            0x800a40e0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fa4));
                c.r[0] = c.r[0].wrapping_mul(0x14);
                c.r[3] = c.lwz(c.r[30].wrapping_add(c.r[0]));
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x800a4120; continue; }
                pc = 0x800a40f4;
            }
            0x800a40f4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fa4));
                c.r[6] = c.r[30].wrapping_add(c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[0]);
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x4));
                c.r[4] = c.lwz(c.r[6].wrapping_add(0x8));
                c.r[5] = c.lwz(c.r[6].wrapping_add(0xc));
                c.r[6] = c.lwz(c.r[6].wrapping_add(0x10));
                c.lr = 0x800a4118; f_800a42c4(c);
                pc = 0x800a4118;
            }
            0x800a4118 => {
                c.r[0] = 0x1;
                pc = 0x800a4150; continue;
                pc = 0x800a4120;
            }
            0x800a4120 => {
                c.cmps(0, c.r[3], 0x2);
                if !c.crbit(2) { pc = 0x800a414c; continue; }
                pc = 0x800a4128;
            }
            0x800a4128 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fa4));
                c.r[4] = c.r[30].wrapping_add(c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[0]);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x10));
                c.lr = 0x800a4144; f_800a46ec(c);
                pc = 0x800a4144;
            }
            0x800a4144 => {
                c.r[0] = 0x1;
                pc = 0x800a4150; continue;
                pc = 0x800a414c;
            }
            0x800a414c => {
                c.r[0] = 0x0;
                pc = 0x800a4150;
            }
            0x800a4150 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a416c; continue; }
                pc = 0x800a4158;
            }
            0x800a4158 => {
                pc = 0x800a41bc; continue;
                pc = 0x800a415c;
            }
            0x800a415c => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[30], c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[0]);
                pc = 0x800a416c;
            }
            0x800a416c => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800a4174; f_8009bd60(c);
                pc = 0x800a4174;
            }
            0x800a4174 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800a417c; f_8009bb68(c);
                pc = 0x800a417c;
            }
            0x800a417c => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x800a41ac; continue; }
                pc = 0x800a4184;
            }
            0x800a4184 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9f68));
                c.r[0] = 0x0;
                c.cmpu(0, c.r[12], 0x0);
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[0]);
                if c.crbit(2) { pc = 0x800a41a4; continue; }
                pc = 0x800a4198;
            }
            0x800a4198 => {
                c.mtspr(8, c.r[12]);
                c.r[3] = c.r[29].wrapping_add(0x0);
                if true { let t = c.lr; c.lr = 0x800a41a4; (c.dispatch)(c, t); }
                pc = 0x800a41a4;
            }
            0x800a41a4 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9f80), c.r[0]);
                pc = 0x800a41ac;
            }
            0x800a41ac => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800a41b4; f_8009bd60(c);
                pc = 0x800a41b4;
            }
            0x800a41b4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x800a41bc; f_8009bb68(c);
                pc = 0x800a41bc;
            }
            0x800a41bc => {
                { let mut ea = c.r[1].wrapping_add(0x2dc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2f4));
                c.r[1] = c.r[1].wrapping_add(0x2f0);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a3ef0: bad pc {:#x}", pc),
        }
    }
}

/// AlarmHandler (0x800a41d0, 132 bytes)
#[inline(never)]
pub fn f_800a41d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a41d0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a41d0); p = true; }
    }
    f_800a41d0_b(c, 0x800a41d0);
    if p { crate::prof::exit(c, 0x800a41d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a41d0_at(c: &mut Cpu, pc: u32) { f_800a41d0_b(c, pc) }
#[inline(always)]
fn f_800a41d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a41d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a41d0, pc); }
        match pc {
            0x800a41d0 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xffffc290);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fa4));
                c.r[0] = c.r[0].wrapping_mul(0x14);
                c.r[3] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x800a4220; continue; }
                pc = 0x800a41f8;
            }
            0x800a41f8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fa4));
                c.r[6] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[0]);
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x4));
                c.r[4] = c.lwz(c.r[6].wrapping_add(0x8));
                c.r[5] = c.lwz(c.r[6].wrapping_add(0xc));
                c.r[6] = c.lwz(c.r[6].wrapping_add(0x10));
                c.lr = 0x800a421c; f_800a42c4(c);
                pc = 0x800a421c;
            }
            0x800a421c => {
                pc = 0x800a4244; continue;
                pc = 0x800a4220;
            }
            0x800a4220 => {
                c.cmps(0, c.r[3], 0x2);
                if !c.crbit(2) { pc = 0x800a4244; continue; }
                pc = 0x800a4228;
            }
            0x800a4228 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fa4));
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[0]);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x10));
                c.lr = 0x800a4244; f_800a46ec(c);
                pc = 0x800a4244;
            }
            0x800a4244 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a41d0: bad pc {:#x}", pc),
        }
    }
}

/// AlarmHandlerForTimeout (0x800a4254, 112 bytes)
#[inline(never)]
pub fn f_800a4254(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4254) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4254); p = true; }
    }
    f_800a4254_b(c, 0x800a4254);
    if p { crate::prof::exit(c, 0x800a4254); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4254_at(c: &mut Cpu, pc: u32) { f_800a4254_b(c, pc) }
#[inline(always)]
fn f_800a4254_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4254;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4254, pc); }
        match pc {
            0x800a4254 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x400;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffd20); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2dc), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.lr = 0x800a4270; f_8009e304(c);
                pc = 0x800a4270;
            }
            0x800a4270 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800a4278; f_8009bd60(c);
                pc = 0x800a4278;
            }
            0x800a4278 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800a4280; f_8009bb68(c);
                pc = 0x800a4280;
            }
            0x800a4280 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9f68));
                c.r[0] = 0x0;
                c.cmpu(0, c.r[12], 0x0);
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[0]);
                if c.crbit(2) { pc = 0x800a42a0; continue; }
                pc = 0x800a4294;
            }
            0x800a4294 => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0x10;
                if true { let t = c.lr; c.lr = 0x800a42a0; (c.dispatch)(c, t); }
                pc = 0x800a42a0;
            }
            0x800a42a0 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800a42a8; f_8009bd60(c);
                pc = 0x800a42a8;
            }
            0x800a42a8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a42b0; f_8009bb68(c);
                pc = 0x800a42b0;
            }
            0x800a42b0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2e4));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2dc));
                c.r[1] = c.r[1].wrapping_add(0x2e0);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4254: bad pc {:#x}", pc),
        }
    }
}

/// Read (0x800a42c4, 272 bytes)
#[inline(never)]
pub fn f_800a42c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a42c4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a42c4); p = true; }
    }
    f_800a42c4_b(c, 0x800a42c4);
    if p { crate::prof::exit(c, 0x800a42c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a42c4_at(c: &mut Cpu, pc: u32) { f_800a42c4_b(c, pc) }
#[inline(always)]
fn f_800a42c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a42c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a42c4, pc); }
        match pc {
            0x800a42c4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x18), c.r[28]);
                c.r[28] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[6]);
                c.r[6] = 0x80400000;
                c.r[31] = c.r[6].wrapping_add(0xffffc290);
                c.stw(c.r[13].wrapping_add(0xffff9fa0), c.r[0]);
                c.lr = 0x800a430c; f_800a2798(c);
                pc = 0x800a430c;
            }
            0x800a430c => {
                c.stw(c.r[13].wrapping_add(0xffff9f9c), c.r[4]);
                c.r[4] = 0xcc000000;
                c.r[0] = 0xa00000;
                c.stw(c.r[13].wrapping_add(0xffff9f98), c.r[3]);
                c.r[4] = c.r[4].wrapping_add(0x6000);
                c.r[3] = 0xa8000000;
                c.stw(c.r[4].wrapping_add(0x8), c.r[3]);
                c.r[3] = c.r[30].rotate_left(30) & 0x3fffffff;
                c.cmpu(0, c.r[29], c.r[0]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[3]);
                c.r[0] = 0x3;
                c.stw(c.r[4].wrapping_add(0x10), c.r[29]);
                c.stw(c.r[4].wrapping_add(0x14), c.r[28]);
                c.stw(c.r[4].wrapping_add(0x18), c.r[29]);
                c.stw(c.r[13].wrapping_add(0xffff9f64), c.r[29]);
                c.stw(c.r[4].wrapping_add(0x1c), c.r[0]);
                if !c.crbit(1) { pc = 0x800a4384; continue; }
                pc = 0x800a4350;
            }
            0x800a4350 => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[3] = c.r[31].wrapping_add(0x68);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[30] = c.r[0].wrapping_mul(0x14);
                c.lr = 0x800a4368; f_8009a2c8(c);
                pc = 0x800a4368;
            }
            0x800a4368 => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x4254);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[3] = c.r[31].wrapping_add(0x68);
                c.r[5] = 0x0;
                c.lr = 0x800a4380; f_8009a528(c);
                pc = 0x800a4380;
            }
            0x800a4380 => {
                pc = 0x800a43b4; continue;
                pc = 0x800a4384;
            }
            0x800a4384 => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[3] = c.r[31].wrapping_add(0x68);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[30] = c.r[0].wrapping_mul(0xa);
                c.lr = 0x800a439c; f_8009a2c8(c);
                pc = 0x800a439c;
            }
            0x800a439c => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x4254);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[3] = c.r[31].wrapping_add(0x68);
                c.r[5] = 0x0;
                c.lr = 0x800a43b4; f_8009a528(c);
                pc = 0x800a43b4;
            }
            0x800a43b4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a42c4: bad pc {:#x}", pc),
        }
    }
}

/// SeekTwiceBeforeRead (0x800a43d4, 128 bytes)
#[inline(never)]
pub fn f_800a43d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a43d4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a43d4); p = true; }
    }
    f_800a43d4_b(c, 0x800a43d4);
    if p { crate::prof::exit(c, 0x800a43d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a43d4_at(c: &mut Cpu, pc: u32) { f_800a43d4_b(c, pc) }
#[inline(always)]
fn f_800a43d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a43d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a43d4, pc); }
        match pc {
            0x800a43d4 => {
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[8] = c.r[5].rotate_left(0) & 0xffff8000;
                c.rc(c.r[8]);
                c.r[9] = c.r[7].wrapping_add(0xffffc290);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                if !c.crbit(2) { pc = 0x800a43f8; continue; }
                pc = 0x800a43f0;
            }
            0x800a43f0 => {
                c.r[10] = 0x0;
                pc = 0x800a4400; continue;
                pc = 0x800a43f8;
            }
            0x800a43f8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f88));
                c.r[10] = c.r[8].wrapping_add(c.r[0]);
                pc = 0x800a4400;
            }
            0x800a4400 => {
                c.r[0] = 0x2;
                c.stw(c.r[9], c.r[0]);
                c.r[8] = 0x1;
                c.r[7] = 0xffffffff;
                c.stw(c.r[9].wrapping_add(0xc), c.r[10]);
                c.r[0] = 0x0;
                c.stw(c.r[9].wrapping_add(0x10), c.r[6]);
                c.stw(c.r[9].wrapping_add(0x14), c.r[8]);
                c.stw(c.r[9].wrapping_add(0x18), c.r[3]);
                c.r[3] = c.r[10] | c.r[10];
                c.stw(c.r[9].wrapping_add(0x1c), c.r[4]);
                c.r[4] = c.r[6] | c.r[6];
                c.stw(c.r[9].wrapping_add(0x20), c.r[5]);
                c.stw(c.r[9].wrapping_add(0x24), c.r[6]);
                c.stw(c.r[9].wrapping_add(0x28), c.r[7]);
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[0]);
                c.lr = 0x800a4444; f_800a46ec(c);
                pc = 0x800a4444;
            }
            0x800a4444 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a43d4: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowRead (0x800a4454, 664 bytes)
#[inline(never)]
pub fn f_800a4454(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4454) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4454); p = true; }
    }
    f_800a4454_b(c, 0x800a4454);
    if p { crate::prof::exit(c, 0x800a4454); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4454_at(c: &mut Cpu, pc: u32) { f_800a4454_b(c, pc) }
#[inline(never)]
fn f_800a4454_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4454;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4454, pc); }
        match pc {
            0x800a4454 => {
                c.r[0] = c.mfspr(8);
                c.r[7] = 0xcc000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[4].wrapping_add(0x0);
                c.r[4] = c.r[7].wrapping_add(0x6000);
                c.r[7] = 0x80400000;
                c.r[31] = c.r[7].wrapping_add(0xffffc290);
                c.r[24] = c.r[3].wrapping_add(0x0);
                c.r[26] = c.r[5].wrapping_add(0x0);
                c.r[30] = c.r[31].wrapping_add(0xcc);
                c.r[27] = c.r[6] | c.r[6];
                c.stw(c.r[4].wrapping_add(0x18), c.r[25]);
                c.stw(c.r[31].wrapping_add(0xc4), c.r[24]);
                c.stw(c.r[31].wrapping_add(0xc8), c.r[25]);
                c.stw(c.r[31].wrapping_add(0xcc), c.r[26]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f84));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a44cc; continue; }
                pc = 0x800a44a4;
            }
            0x800a44a4 => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[31], c.r[0]);
                c.r[0] = 0x0;
                c.r[3] = c.r[24].wrapping_add(0x0);
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[0]);
                c.r[4] = c.r[25].wrapping_add(0x0);
                c.r[5] = c.r[26].wrapping_add(0x0);
                c.r[6] = c.r[27].wrapping_add(0x0);
                c.lr = 0x800a44c8; f_800a42c4(c);
                pc = 0x800a44c8;
            }
            0x800a44c8 => {
                pc = 0x800a46d4; continue;
                pc = 0x800a44cc;
            }
            0x800a44cc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f84));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800a46d4; continue; }
                pc = 0x800a44d8;
            }
            0x800a44d8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8198));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a44fc; continue; }
                pc = 0x800a44e4;
            }
            0x800a44e4 => {
                c.r[3] = c.r[24].wrapping_add(0x0);
                c.r[4] = c.r[25].wrapping_add(0x0);
                c.r[5] = c.r[26].wrapping_add(0x0);
                c.r[6] = c.r[27].wrapping_add(0x0);
                c.lr = 0x800a44f8; f_800a43d4(c);
                pc = 0x800a44f8;
            }
            0x800a44f8 => {
                pc = 0x800a46d4; continue;
                pc = 0x800a44fc;
            }
            0x800a44fc => {
                c.r[29] = c.r[31].wrapping_add(0xbc);
                c.r[0] = c.lwz(c.r[30]);
                c.r[28] = c.r[31].wrapping_add(0xc0);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xbc));
                c.r[4] = c.lwz(c.r[31].wrapping_add(0xc0));
                c.r[23] = c.r[0].rotate_left(17) & 0x1ffff;
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.r[22] = c.r[0].rotate_left(17) & 0x1ffff;
                c.lr = 0x800a4524; f_800a7bcc(c);
                pc = 0x800a4524;
            }
            0x800a4524 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a4538; continue; }
                pc = 0x800a4530;
            }
            0x800a4530 => {
                c.r[0] = 0x1;
                pc = 0x800a453c; continue;
                pc = 0x800a4538;
            }
            0x800a4538 => {
                c.r[0] = 0x0;
                pc = 0x800a453c;
            }
            0x800a453c => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a454c; continue; }
                pc = 0x800a4544;
            }
            0x800a4544 => {
                c.r[3] = 0x5;
                pc = 0x800a4550; continue;
                pc = 0x800a454c;
            }
            0x800a454c => {
                c.r[3] = 0xf;
                pc = 0x800a4550;
            }
            0x800a4550 => {
                c.r[0] = c.r[22].wrapping_add(0xfffffffe);
                c.cmpu(0, c.r[23], c.r[0]);
                if c.crbit(1) { pc = 0x800a456c; continue; }
                pc = 0x800a455c;
            }
            0x800a455c => {
                c.r[0] = c.r[3].wrapping_add(0x3);
                c.r[0] = c.r[22].wrapping_add(c.r[0]);
                c.cmpu(0, c.r[23], c.r[0]);
                if !c.crbit(0) { pc = 0x800a4574; continue; }
                pc = 0x800a456c;
            }
            0x800a456c => {
                c.r[0] = 0x1;
                pc = 0x800a4578; continue;
                pc = 0x800a4574;
            }
            0x800a4574 => {
                c.r[0] = 0x0;
                pc = 0x800a4578;
            }
            0x800a4578 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a45a8; continue; }
                pc = 0x800a4580;
            }
            0x800a4580 => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[31], c.r[0]);
                c.r[0] = 0x0;
                c.r[3] = c.r[24].wrapping_add(0x0);
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[0]);
                c.r[4] = c.r[25].wrapping_add(0x0);
                c.r[5] = c.r[26].wrapping_add(0x0);
                c.r[6] = c.r[27].wrapping_add(0x0);
                c.lr = 0x800a45a4; f_800a42c4(c);
                pc = 0x800a45a4;
            }
            0x800a45a4 => {
                pc = 0x800a46d4; continue;
                pc = 0x800a45a8;
            }
            0x800a45a8 => {
                c.r[3] = c.lwz(c.r[29]);
                c.r[4] = c.lwz(c.r[28]);
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                c.r[0] = c.lwz(c.r[30]);
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[3] = c.r[3].rotate_left(17) & 0x1ffff;
                c.r[4] = c.r[0].rotate_left(17) & 0x1ffff;
                c.cmpu(0, c.r[3], c.r[4]);
                if c.crbit(2) { pc = 0x800a45d8; continue; }
                pc = 0x800a45cc;
            }
            0x800a45cc => {
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], c.r[4]);
                if !c.crbit(2) { pc = 0x800a46c0; continue; }
                pc = 0x800a45d8;
            }
            0x800a45d8 => {
                c.lr = 0x800a45dc; f_800a2798(c);
                pc = 0x800a45dc;
            }
            0x800a45dc => {
                c.r[5] = 0x80000000;
                c.r[8] = c.lwz(c.r[13].wrapping_add(0xffff9f90));
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xf8));
                c.r[5] = 0x10620000;
                c.r[9] = c.lwz(c.r[13].wrapping_add(0xffff9f94));
                c.r[6] = 0x0;
                c.r[7] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = c.r[5].wrapping_add(0x4dd3);
                c.r[0] = mulhwu(c.r[0], c.r[7]);
                c.r[0] = c.r[0].rotate_left(26) & 0x3ffffff;
                c.r[9] = c.addc(!c.r[9], c.r[4], 1);
                { let ci = c.ca(); c.r[8] = c.addc(!c.r[8], c.r[3], ci); }
                c.r[5] = c.r[0].wrapping_mul(0x5);
                c.r[4] = c.r[6] ^ 0x80000000;
                c.r[3] = c.r[8] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[9], c.r[5], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[4], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[4], c.r[4], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x800a4650; continue; }
                pc = 0x800a462c;
            }
            0x800a462c => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[31], c.r[0]);
                c.r[3] = c.r[24].wrapping_add(0x0);
                c.r[4] = c.r[25].wrapping_add(0x0);
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[6]);
                c.r[5] = c.r[26].wrapping_add(0x0);
                c.r[6] = c.r[27].wrapping_add(0x0);
                c.lr = 0x800a464c; f_800a42c4(c);
                pc = 0x800a464c;
            }
            0x800a464c => {
                pc = 0x800a46d4; continue;
                pc = 0x800a4650;
            }
            0x800a4650 => {
                c.r[0] = 0x1;
                c.stw(c.r[31], c.r[0]);
                c.r[3] = 0x431c0000;
                c.r[0] = c.r[3].wrapping_add(0xffffde83);
                c.stw(c.r[31].wrapping_add(0x4), c.r[24]);
                c.r[0] = mulhwu(c.r[0], c.r[7]);
                c.stw(c.r[31].wrapping_add(0x8), c.r[25]);
                c.stw(c.r[31].wrapping_add(0xc), c.r[26]);
                c.r[0] = c.r[0].rotate_left(17) & 0x1ffff;
                c.r[3] = c.r[0].wrapping_mul(0x1f4);
                c.stw(c.r[31].wrapping_add(0x10), c.r[27]);
                c.r[0] = 0xffffffff;
                c.stw(c.r[31].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.addc(!c.r[9], c.r[5], 1);
                { let ci = c.ca(); c.r[4] = c.addc(!c.r[8], c.r[6], ci); }
                c.r[0] = c.r[3].rotate_left(29) & 0x1fffffff;
                c.stw(c.r[13].wrapping_add(0xffff9fa4), c.r[6]);
                c.r[23] = c.addc(c.r[5], c.r[0], 0);
                { let ci = c.ca(); c.r[22] = c.addc(c.r[4], c.r[6], ci); }
                c.r[3] = c.r[31].wrapping_add(0x40);
                c.lr = 0x800a46a4; f_8009a2c8(c);
                pc = 0x800a46a4;
            }
            0x800a46a4 => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x41d0);
                c.r[6] = c.r[23].wrapping_add(0x0);
                c.r[5] = c.r[22].wrapping_add(0x0);
                c.r[3] = c.r[31].wrapping_add(0x40);
                c.lr = 0x800a46bc; f_8009a528(c);
                pc = 0x800a46bc;
            }
            0x800a46bc => {
                pc = 0x800a46d4; continue;
                pc = 0x800a46c0;
            }
            0x800a46c0 => {
                c.r[3] = c.r[24].wrapping_add(0x0);
                c.r[4] = c.r[25].wrapping_add(0x0);
                c.r[5] = c.r[26].wrapping_add(0x0);
                c.r[6] = c.r[27].wrapping_add(0x0);
                c.lr = 0x800a46d4; f_800a43d4(c);
                pc = 0x800a46d4;
            }
            0x800a46d4 => {
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 22..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4454: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowSeek (0x800a46ec, 148 bytes)
#[inline(never)]
pub fn f_800a46ec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a46ec) { return; }
        if c.prof { crate::prof::enter(c, 0x800a46ec); p = true; }
    }
    f_800a46ec_b(c, 0x800a46ec);
    if p { crate::prof::exit(c, 0x800a46ec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a46ec_at(c: &mut Cpu, pc: u32) { f_800a46ec_b(c, pc) }
#[inline(always)]
fn f_800a46ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a46ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a46ec, pc); }
        match pc {
            0x800a46ec => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[4]);
                c.r[4] = 0xcc000000;
                c.r[4] = c.r[4].wrapping_add(0x6000);
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                c.r[0] = 0xab000000;
                c.stw(c.r[4].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.r[3].rotate_left(30) & 0x3fffffff;
                c.r[3] = 0x80400000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[0] = 0x1;
                c.r[31] = c.r[3].wrapping_add(0xffffc2f8);
                c.stw(c.r[4].wrapping_add(0x1c), c.r[0]);
                c.r[4] = 0x80000000;
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xf8));
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[30] = c.r[0].wrapping_mul(0xa);
                c.lr = 0x800a474c; f_8009a2c8(c);
                pc = 0x800a474c;
            }
            0x800a474c => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x4254);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.lr = 0x800a4764; f_8009a528(c);
                pc = 0x800a4764;
            }
            0x800a4764 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a46ec: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowWaitCoverClose (0x800a4780, 44 bytes)
#[inline(never)]
pub fn f_800a4780(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4780) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4780); p = true; }
    }
    f_800a4780_b(c, 0x800a4780);
    if p { crate::prof::exit(c, 0x800a4780); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4780_at(c: &mut Cpu, pc: u32) { f_800a4780_b(c, pc) }
#[inline(always)]
fn f_800a4780_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4780;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4780, pc); }
        match pc {
            0x800a4780 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[3]);
                c.r[3] = 0xcc000000;
                c.stw(c.r[13].wrapping_add(0xffff9f7c), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = c.r[3].wrapping_add(0x6000);
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                c.r[0] = 0x2;
                c.r[3] = 0x1;
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4780: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowReadDiskID (0x800a47ac, 164 bytes)
#[inline(never)]
pub fn f_800a47ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a47ac) { return; }
        if c.prof { crate::prof::enter(c, 0x800a47ac); p = true; }
    }
    f_800a47ac_b(c, 0x800a47ac);
    if p { crate::prof::exit(c, 0x800a47ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a47ac_at(c: &mut Cpu, pc: u32) { f_800a47ac_b(c, pc) }
#[inline(always)]
fn f_800a47ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a47ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a47ac, pc); }
        match pc {
            0x800a47ac => {
                c.r[0] = c.mfspr(8);
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[5] = 0xa8000000;
                c.r[0] = c.r[5].wrapping_add(0x40);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[6] = 0x20;
                c.r[5] = 0x80000000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[4]);
                c.r[4] = 0xcc000000;
                c.r[7] = c.r[4].wrapping_add(0x6000);
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[8]);
                c.stw(c.r[4].wrapping_add(0x6008), c.r[0]);
                c.r[4] = 0x80400000;
                c.r[0] = 0x3;
                c.stw(c.r[7].wrapping_add(0xc), c.r[8]);
                c.r[31] = c.r[4].wrapping_add(0xffffc2f8);
                c.stw(c.r[7].wrapping_add(0x10), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x14), c.r[3]);
                c.r[3] = c.r[31] | c.r[31];
                c.stw(c.r[7].wrapping_add(0x18), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x1c), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xf8));
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[30] = c.r[0].wrapping_mul(0xa);
                c.lr = 0x800a481c; f_8009a2c8(c);
                pc = 0x800a481c;
            }
            0x800a481c => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x4254);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.lr = 0x800a4834; f_8009a528(c);
                pc = 0x800a4834;
            }
            0x800a4834 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a47ac: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowStopMotor (0x800a4850, 140 bytes)
#[inline(never)]
pub fn f_800a4850(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4850) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4850); p = true; }
    }
    f_800a4850_b(c, 0x800a4850);
    if p { crate::prof::exit(c, 0x800a4850); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4850_at(c: &mut Cpu, pc: u32) { f_800a4850_b(c, pc) }
#[inline(always)]
fn f_800a4850_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4850;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4850, pc); }
        match pc {
            0x800a4850 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[3]);
                c.r[3] = 0xcc000000;
                c.r[4] = c.r[3].wrapping_add(0x6000);
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                c.r[0] = 0xe3000000;
                c.stw(c.r[3].wrapping_add(0x6008), c.r[0]);
                c.r[0] = 0x1;
                c.r[3] = 0x80400000;
                c.stw(c.r[4].wrapping_add(0x1c), c.r[0]);
                c.r[4] = 0x80000000;
                c.r[31] = c.r[3].wrapping_add(0xffffc2f8);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xf8));
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[30] = c.r[0].wrapping_mul(0xa);
                c.lr = 0x800a48a8; f_8009a2c8(c);
                pc = 0x800a48a8;
            }
            0x800a48a8 => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x4254);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.lr = 0x800a48c0; f_8009a528(c);
                pc = 0x800a48c0;
            }
            0x800a48c0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4850: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowRequestError (0x800a48dc, 140 bytes)
#[inline(never)]
pub fn f_800a48dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a48dc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a48dc); p = true; }
    }
    f_800a48dc_b(c, 0x800a48dc);
    if p { crate::prof::exit(c, 0x800a48dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a48dc_at(c: &mut Cpu, pc: u32) { f_800a48dc_b(c, pc) }
#[inline(always)]
fn f_800a48dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a48dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a48dc, pc); }
        match pc {
            0x800a48dc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[3]);
                c.r[3] = 0xcc000000;
                c.r[4] = c.r[3].wrapping_add(0x6000);
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                c.r[0] = 0xe0000000;
                c.stw(c.r[3].wrapping_add(0x6008), c.r[0]);
                c.r[0] = 0x1;
                c.r[3] = 0x80400000;
                c.stw(c.r[4].wrapping_add(0x1c), c.r[0]);
                c.r[4] = 0x80000000;
                c.r[31] = c.r[3].wrapping_add(0xffffc2f8);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xf8));
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[30] = c.r[0].wrapping_mul(0xa);
                c.lr = 0x800a4934; f_8009a2c8(c);
                pc = 0x800a4934;
            }
            0x800a4934 => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x4254);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.lr = 0x800a494c; f_8009a528(c);
                pc = 0x800a494c;
            }
            0x800a494c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a48dc: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowInquiry (0x800a4968, 156 bytes)
#[inline(never)]
pub fn f_800a4968(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4968) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4968); p = true; }
    }
    f_800a4968_b(c, 0x800a4968);
    if p { crate::prof::exit(c, 0x800a4968); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4968_at(c: &mut Cpu, pc: u32) { f_800a4968_b(c, pc) }
#[inline(always)]
fn f_800a4968_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4968;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4968, pc); }
        match pc {
            0x800a4968 => {
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x20;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                c.r[5] = 0x80000000;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[4]);
                c.r[4] = 0xcc000000;
                c.r[7] = c.r[4].wrapping_add(0x6000);
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                c.r[0] = 0x12000000;
                c.stw(c.r[4].wrapping_add(0x6008), c.r[0]);
                c.r[4] = 0x80400000;
                c.r[0] = 0x3;
                c.stw(c.r[7].wrapping_add(0x10), c.r[6]);
                c.r[31] = c.r[4].wrapping_add(0xffffc2f8);
                c.stw(c.r[7].wrapping_add(0x14), c.r[3]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.stw(c.r[7].wrapping_add(0x18), c.r[6]);
                c.stw(c.r[7].wrapping_add(0x1c), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xf8));
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[30] = c.r[0].wrapping_mul(0xa);
                c.lr = 0x800a49d0; f_8009a2c8(c);
                pc = 0x800a49d0;
            }
            0x800a49d0 => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x4254);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.lr = 0x800a49e8; f_8009a528(c);
                pc = 0x800a49e8;
            }
            0x800a49e8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4968: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowAudioStream (0x800a4a04, 152 bytes)
#[inline(never)]
pub fn f_800a4a04(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4a04) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4a04); p = true; }
    }
    f_800a4a04_b(c, 0x800a4a04);
    if p { crate::prof::exit(c, 0x800a4a04); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4a04_at(c: &mut Cpu, pc: u32) { f_800a4a04_b(c, pc) }
#[inline(always)]
fn f_800a4a04_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4a04;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4a04, pc); }
        match pc {
            0x800a4a04 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[6]);
                c.r[6] = 0xcc000000;
                c.r[6] = c.r[6].wrapping_add(0x6000);
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                c.r[0] = c.r[3] | 0xe1000000;
                c.r[3] = 0x80400000;
                c.stw(c.r[6].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.r[5].rotate_left(30) & 0x3fffffff;
                c.r[31] = c.r[3].wrapping_add(0xffffc2f8);
                c.stw(c.r[6].wrapping_add(0xc), c.r[0]);
                c.r[0] = 0x1;
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.stw(c.r[6].wrapping_add(0x10), c.r[4]);
                c.r[4] = 0x80000000;
                c.stw(c.r[6].wrapping_add(0x1c), c.r[0]);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xf8));
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[30] = c.r[0].wrapping_mul(0xa);
                c.lr = 0x800a4a68; f_8009a2c8(c);
                pc = 0x800a4a68;
            }
            0x800a4a68 => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x4254);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.lr = 0x800a4a80; f_8009a528(c);
                pc = 0x800a4a80;
            }
            0x800a4a80 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4a04: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowRequestAudioStatus (0x800a4a9c, 140 bytes)
#[inline(never)]
pub fn f_800a4a9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4a9c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4a9c); p = true; }
    }
    f_800a4a9c_b(c, 0x800a4a9c);
    if p { crate::prof::exit(c, 0x800a4a9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4a9c_at(c: &mut Cpu, pc: u32) { f_800a4a9c_b(c, pc) }
#[inline(always)]
fn f_800a4a9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4a9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4a9c, pc); }
        match pc {
            0x800a4a9c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[4]);
                c.r[4] = 0xcc000000;
                c.r[4] = c.r[4].wrapping_add(0x6000);
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                c.r[0] = c.r[3] | 0xe2000000;
                c.r[3] = 0x80400000;
                c.stw(c.r[4].wrapping_add(0x8), c.r[0]);
                c.r[0] = 0x1;
                c.r[31] = c.r[3].wrapping_add(0xffffc2f8);
                c.stw(c.r[4].wrapping_add(0x1c), c.r[0]);
                c.r[4] = 0x80000000;
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xf8));
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[30] = c.r[0].wrapping_mul(0xa);
                c.lr = 0x800a4af4; f_8009a2c8(c);
                pc = 0x800a4af4;
            }
            0x800a4af4 => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x4254);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.lr = 0x800a4b0c; f_8009a528(c);
                pc = 0x800a4b0c;
            }
            0x800a4b0c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4a9c: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowAudioBufferConfig (0x800a4b28, 156 bytes)
#[inline(never)]
pub fn f_800a4b28(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4b28) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4b28); p = true; }
    }
    f_800a4b28_b(c, 0x800a4b28);
    if p { crate::prof::exit(c, 0x800a4b28); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4b28_at(c: &mut Cpu, pc: u32) { f_800a4b28_b(c, pc) }
#[inline(always)]
fn f_800a4b28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4b28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4b28, pc); }
        match pc {
            0x800a4b28 => {
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                if c.crbit(2) { pc = 0x800a4b54; continue; }
                pc = 0x800a4b50;
            }
            0x800a4b50 => {
                c.r[0] = 0x10000;
                pc = 0x800a4b54;
            }
            0x800a4b54 => {
                c.r[0] = c.r[0] | 0xe4000000;
                c.r[3] = 0xcc000000;
                c.r[0] = c.r[4] | c.r[0];
                c.r[3] = c.r[3].wrapping_add(0x6000);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.r[0] = 0x1;
                c.r[4] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0x1c), c.r[0]);
                c.r[3] = 0x80400000;
                c.r[31] = c.r[3].wrapping_add(0xffffc2f8);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xf8));
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[30] = c.r[0].wrapping_mul(0xa);
                c.lr = 0x800a4b90; f_8009a2c8(c);
                pc = 0x800a4b90;
            }
            0x800a4b90 => {
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x4254);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.lr = 0x800a4ba8; f_8009a528(c);
                pc = 0x800a4ba8;
            }
            0x800a4ba8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4b28: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowReset (0x800a4bc4, 188 bytes)
#[inline(never)]
pub fn f_800a4bc4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4bc4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4bc4); p = true; }
    }
    f_800a4bc4_b(c, 0x800a4bc4);
    if p { crate::prof::exit(c, 0x800a4bc4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4bc4_at(c: &mut Cpu, pc: u32) { f_800a4bc4_b(c, pc) }
#[inline(always)]
fn f_800a4bc4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4bc4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4bc4, pc); }
        match pc {
            0x800a4bc4 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0xcc000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x2;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4].wrapping_add(0x3000);
                c.stw(c.r[4].wrapping_add(0x6004), c.r[0]);
                c.r[30] = c.lwz(c.r[4].wrapping_add(0x3024));
                c.r[0] = c.r[30].rotate_left(0) & 0xfffffffb;
                c.r[0] = c.r[0] | 0x1;
                { let ea = c.r[31].wrapping_add(0x24); c.stw(ea, c.r[0]); c.r[31] = ea; }
                c.lr = 0x800a4bf8; f_800a2798(c);
                pc = 0x800a4bf8;
            }
            0x800a4bf8 => {
                c.r[5] = 0x80000000;
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xf8));
                c.r[5] = 0x431c0000;
                c.r[5] = c.r[5].wrapping_add(0xffffde83);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = mulhwu(c.r[5], c.r[0]);
                c.r[0] = c.r[0].rotate_left(17) & 0x1ffff;
                c.r[0] = c.r[0].wrapping_mul(0xc);
                c.r[26] = c.r[4].wrapping_add(0x0);
                c.r[27] = c.r[3].wrapping_add(0x0);
                c.r[28] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.r[29] = 0x0;
                pc = 0x800a4c28;
            }
            0x800a4c28 => {
                c.lr = 0x800a4c2c; f_800a2798(c);
                pc = 0x800a4c2c;
            }
            0x800a4c2c => {
                c.r[5] = c.addc(!c.r[26], c.r[4], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[27], c.r[3], ci); }
                c.r[4] = c.r[0] ^ 0x80000000;
                c.r[3] = c.r[29] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[28], c.r[5], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[4], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[4], c.r[4], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if !c.crbit(2) { pc = 0x800a4c28; continue; }
                pc = 0x800a4c50;
            }
            0x800a4c50 => {
                c.r[0] = c.r[30] | 0x5;
                c.stw(c.r[31], c.r[0]);
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f78), c.r[0]);
                c.lr = 0x800a4c64; f_800a2798(c);
                pc = 0x800a4c64;
            }
            0x800a4c64 => {
                c.stw(c.r[13].wrapping_add(0xffff9f74), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffff9f70), c.r[3]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4bc4: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowBreak (0x800a4c80, 20 bytes)
#[inline(never)]
pub fn f_800a4c80(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4c80) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4c80); p = true; }
    }
    f_800a4c80_b(c, 0x800a4c80);
    if p { crate::prof::exit(c, 0x800a4c80); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4c80_at(c: &mut Cpu, pc: u32) { f_800a4c80_b(c, pc) }
#[inline(always)]
fn f_800a4c80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4c80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4c80, pc); }
        match pc {
            0x800a4c80 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f60), c.r[0]);
                c.r[3] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f80), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4c80: bad pc {:#x}", pc),
        }
    }
}

/// DVDLowClearCallback (0x800a4c94, 24 bytes)
#[inline(never)]
pub fn f_800a4c94(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4c94) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4c94); p = true; }
    }
    f_800a4c94_b(c, 0x800a4c94);
    if p { crate::prof::exit(c, 0x800a4c94); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4c94_at(c: &mut Cpu, pc: u32) { f_800a4c94_b(c, pc) }
#[inline(always)]
fn f_800a4c94_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4c94;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4c94, pc); }
        match pc {
            0x800a4c94 => {
                c.r[3] = 0xcc000000;
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x6004), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9f68));
                c.stw(c.r[13].wrapping_add(0xffff9f68), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4c94: bad pc {:#x}", pc),
        }
    }
}

/// __DVDLowSetWAType (0x800a4cac, 68 bytes)
#[inline(never)]
pub fn f_800a4cac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4cac) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4cac); p = true; }
    }
    f_800a4cac_b(c, 0x800a4cac);
    if p { crate::prof::exit(c, 0x800a4cac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4cac_at(c: &mut Cpu, pc: u32) { f_800a4cac_b(c, pc) }
#[inline(always)]
fn f_800a4cac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4cac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4cac, pc); }
        match pc {
            0x800a4cac => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800a4ccc; f_8009df3c(c);
                pc = 0x800a4ccc;
            }
            0x800a4ccc => {
                c.stw(c.r[13].wrapping_add(0xffff9f84), c.r[30]);
                c.stw(c.r[13].wrapping_add(0xffff9f88), c.r[31]);
                c.lr = 0x800a4cd8; f_8009df64(c);
                pc = 0x800a4cd8;
            }
            0x800a4cd8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4cac: bad pc {:#x}", pc),
        }
    }
}

/// __DVDFSInit (0x800a4cf0, 56 bytes)
#[inline(never)]
pub fn f_800a4cf0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4cf0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4cf0); p = true; }
    }
    f_800a4cf0_b(c, 0x800a4cf0);
    if p { crate::prof::exit(c, 0x800a4cf0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4cf0_at(c: &mut Cpu, pc: u32) { f_800a4cf0_b(c, pc) }
#[inline(always)]
fn f_800a4cf0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4cf0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4cf0, pc); }
        match pc {
            0x800a4cf0 => {
                c.r[3] = 0x80000000;
                c.stw(c.r[13].wrapping_add(0xffff9fa8), c.r[3]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x38));
                c.stw(c.r[13].wrapping_add(0xffff9fac), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fac));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x800a4d0c;
            }
            0x800a4d0c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x8));
                c.stw(c.r[13].wrapping_add(0xffff9fb4), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fb4));
                c.r[0] = c.r[0].wrapping_mul(0xc);
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9fb0), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4cf0: bad pc {:#x}", pc),
        }
    }
}

/// DVDConvertPathToEntrynum (0x800a4d28, 756 bytes)
#[inline(never)]
pub fn f_800a4d28(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a4d28) { return; }
        if c.prof { crate::prof::enter(c, 0x800a4d28); p = true; }
    }
    f_800a4d28_b(c, 0x800a4d28);
    if p { crate::prof::exit(c, 0x800a4d28); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a4d28_at(c: &mut Cpu, pc: u32) { f_800a4d28_b(c, pc) }
#[inline(never)]
fn f_800a4d28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a4d28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a4d28, pc); }
        match pc {
            0x800a4d28 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffb8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 20..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[23] = c.r[3] | c.r[3];
                c.r[3] = 0x80310000;
                c.r[25] = c.r[23].wrapping_add(0x0);
                c.r[31] = c.r[3].wrapping_add(0x18f0);
                c.r[26] = c.lwz(c.r[13].wrapping_add(0xffff9fb8));
                pc = 0x800a4d4c;
            }
            0x800a4d4c => {
                c.r[3] = c.lbz(c.r[23]);
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a4d60; continue; }
                pc = 0x800a4d58;
            }
            0x800a4d58 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x800a5008; continue;
                pc = 0x800a4d60;
            }
            0x800a4d60 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x2f);
                if !c.crbit(2) { pc = 0x800a4d78; continue; }
                pc = 0x800a4d6c;
            }
            0x800a4d6c => {
                c.r[26] = 0x0;
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x800a4d4c; continue;
                pc = 0x800a4d78;
            }
            0x800a4d78 => {
                c.cmps(0, c.r[0], 0x2e);
                if !c.crbit(2) { pc = 0x800a4df0; continue; }
                pc = 0x800a4d80;
            }
            0x800a4d80 => {
                c.r[3] = c.lbz(c.r[23].wrapping_add(0x1));
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x2e);
                if !c.crbit(2) { pc = 0x800a4dd0; continue; }
                pc = 0x800a4d90;
            }
            0x800a4d90 => {
                c.r[3] = c.lbz(c.r[23].wrapping_add(0x2));
                c.cmps(0, c.r[3], 0x2f);
                if !c.crbit(2) { pc = 0x800a4db4; continue; }
                pc = 0x800a4d9c;
            }
            0x800a4d9c => {
                c.r[3] = c.r[26].wrapping_mul(0xc);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fac));
                c.r[0] = c.r[3].wrapping_add(0x4);
                c.r[26] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                c.r[23] = c.r[23].wrapping_add(0x3);
                pc = 0x800a4d4c; continue;
                pc = 0x800a4db4;
            }
            0x800a4db4 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a4df0; continue; }
                pc = 0x800a4dbc;
            }
            0x800a4dbc => {
                c.r[0] = c.r[26].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fac));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x800a5008; continue;
                pc = 0x800a4dd0;
            }
            0x800a4dd0 => {
                c.cmps(0, c.r[0], 0x2f);
                if !c.crbit(2) { pc = 0x800a4de0; continue; }
                pc = 0x800a4dd8;
            }
            0x800a4dd8 => {
                c.r[23] = c.r[23].wrapping_add(0x2);
                pc = 0x800a4d4c; continue;
                pc = 0x800a4de0;
            }
            0x800a4de0 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a4df0; continue; }
                pc = 0x800a4de8;
            }
            0x800a4de8 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x800a5008; continue;
                pc = 0x800a4df0;
            }
            0x800a4df0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fbc));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a4ea4; continue; }
                pc = 0x800a4dfc;
            }
            0x800a4dfc => {
                c.r[28] = c.r[23].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.r[4] = 0x0;
                pc = 0x800a4e50; continue;
                pc = 0x800a4e0c;
            }
            0x800a4e0c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x2e);
                if !c.crbit(2) { pc = 0x800a4e40; continue; }
                pc = 0x800a4e18;
            }
            0x800a4e18 => {
                c.r[0] = c.r[28].wrapping_sub(c.r[23]);
                c.cmps(0, c.r[0], 0x8);
                if c.crbit(1) { pc = 0x800a4e2c; continue; }
                pc = 0x800a4e24;
            }
            0x800a4e24 => {
                c.cmps(0, c.r[5], 0x1);
                if !c.crbit(2) { pc = 0x800a4e34; continue; }
                pc = 0x800a4e2c;
            }
            0x800a4e2c => {
                c.r[4] = 0x1;
                pc = 0x800a4e68; continue;
                pc = 0x800a4e34;
            }
            0x800a4e34 => {
                c.r[24] = c.r[28].wrapping_add(0x1);
                c.r[5] = 0x1;
                pc = 0x800a4e4c; continue;
                pc = 0x800a4e40;
            }
            0x800a4e40 => {
                c.cmps(0, c.r[0], 0x20);
                if !c.crbit(2) { pc = 0x800a4e4c; continue; }
                pc = 0x800a4e48;
            }
            0x800a4e48 => {
                c.r[4] = 0x1;
                pc = 0x800a4e4c;
            }
            0x800a4e4c => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x800a4e50;
            }
            0x800a4e50 => {
                c.r[3] = c.lbz(c.r[28]);
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a4e68; continue; }
                pc = 0x800a4e5c;
            }
            0x800a4e5c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x2f);
                if !c.crbit(2) { pc = 0x800a4e0c; continue; }
                pc = 0x800a4e68;
            }
            0x800a4e68 => {
                c.cmps(0, c.r[5], 0x1);
                if !c.crbit(2) { pc = 0x800a4e80; continue; }
                pc = 0x800a4e70;
            }
            0x800a4e70 => {
                c.r[0] = c.r[28].wrapping_sub(c.r[24]);
                c.cmps(0, c.r[0], 0x3);
                if !c.crbit(1) { pc = 0x800a4e80; continue; }
                pc = 0x800a4e7c;
            }
            0x800a4e7c => {
                c.r[4] = 0x1;
                pc = 0x800a4e80;
            }
            0x800a4e80 => {
                c.cmps(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800a4ec8; continue; }
                pc = 0x800a4e88;
            }
            0x800a4e88 => {
                c.r[5] = c.r[31].wrapping_add(0x0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[6] = c.r[25].wrapping_add(0x0);
                c.r[3] = c.r[13].wrapping_add(0xffff81a0);
                c.r[4] = 0x17b;
                c.lr = 0x800a4ea0; f_800060f0(c);
                pc = 0x800a4ea0;
            }
            0x800a4ea0 => {
                pc = 0x800a4ec8; continue;
                pc = 0x800a4ea4;
            }
            0x800a4ea4 => {
                c.r[28] = c.r[23] | c.r[23];
                pc = 0x800a4eb0; continue;
                pc = 0x800a4eac;
            }
            0x800a4eac => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x800a4eb0;
            }
            0x800a4eb0 => {
                c.r[3] = c.lbz(c.r[28]);
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a4ec8; continue; }
                pc = 0x800a4ebc;
            }
            0x800a4ebc => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x2f);
                if !c.crbit(2) { pc = 0x800a4eac; continue; }
                pc = 0x800a4ec8;
            }
            0x800a4ec8 => {
                c.r[0] = c.lbz(c.r[28]);
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a4edc; continue; }
                pc = 0x800a4ed4;
            }
            0x800a4ed4 => {
                c.r[30] = 0x0;
                pc = 0x800a4ee0; continue;
                pc = 0x800a4edc;
            }
            0x800a4edc => {
                c.r[30] = 0x1;
                pc = 0x800a4ee0;
            }
            0x800a4ee0 => {
                c.r[29] = c.r[26].wrapping_mul(0xc);
                c.r[27] = c.r[28].wrapping_sub(c.r[23]);
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x800a4fd0; continue;
                pc = 0x800a4ef0;
            }
            0x800a4ef0 => {
                c.r[28] = c.r[26].wrapping_mul(0xc);
                c.r[4] = c.lwz(c.r[3].wrapping_add(c.r[28]));
                c.r[0] = c.r[4].rotate_left(0) & 0xff000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a4f08; continue; }
                pc = 0x800a4f00;
            }
            0x800a4f00 => {
                c.r[0] = 0x0;
                pc = 0x800a4f0c; continue;
                pc = 0x800a4f08;
            }
            0x800a4f08 => {
                c.r[0] = 0x1;
                pc = 0x800a4f0c;
            }
            0x800a4f0c => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a4f1c; continue; }
                pc = 0x800a4f14;
            }
            0x800a4f14 => {
                c.cmps(0, c.r[30], 0x1);
                if c.crbit(2) { pc = 0x800a4f98; continue; }
                pc = 0x800a4f1c;
            }
            0x800a4f1c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fb0));
                c.r[0] = c.r[4].rotate_left(0) & 0xffffff;
                c.r[21] = c.r[23].wrapping_add(0x0);
                c.r[20] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x800a4f64; continue;
                pc = 0x800a4f30;
            }
            0x800a4f30 => {
                c.r[0] = c.lbz(c.r[20]);
                c.r[20] = c.r[20].wrapping_add(0x1);
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                c.lr = 0x800a4f40; f_800c7558(c);
                pc = 0x800a4f40;
            }
            0x800a4f40 => {
                c.r[0] = c.lbz(c.r[21]);
                c.r[22] = c.r[3].wrapping_add(0x0);
                c.r[21] = c.r[21].wrapping_add(0x1);
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                c.lr = 0x800a4f54; f_800c7558(c);
                pc = 0x800a4f54;
            }
            0x800a4f54 => {
                c.cmps(0, c.r[3], c.r[22]);
                if c.crbit(2) { pc = 0x800a4f64; continue; }
                pc = 0x800a4f5c;
            }
            0x800a4f5c => {
                c.r[0] = 0x0;
                pc = 0x800a4f90; continue;
                pc = 0x800a4f64;
            }
            0x800a4f64 => {
                c.r[0] = c.lbz(c.r[20]);
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a4f30; continue; }
                pc = 0x800a4f70;
            }
            0x800a4f70 => {
                c.r[3] = c.lbz(c.r[21]);
                c.cmps(0, c.r[3], 0x2f);
                if c.crbit(2) { pc = 0x800a4f84; continue; }
                pc = 0x800a4f7c;
            }
            0x800a4f7c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a4f8c; continue; }
                pc = 0x800a4f84;
            }
            0x800a4f84 => {
                c.r[0] = 0x1;
                pc = 0x800a4f90; continue;
                pc = 0x800a4f8c;
            }
            0x800a4f8c => {
                c.r[0] = 0x0;
                pc = 0x800a4f90;
            }
            0x800a4f90 => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800a4fec; continue; }
                pc = 0x800a4f98;
            }
            0x800a4f98 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fac));
                c.r[3] = c.r[0].wrapping_add(c.r[28]);
                c.r[0] = c.lwz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(0) & 0xff000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a4fb4; continue; }
                pc = 0x800a4fac;
            }
            0x800a4fac => {
                c.r[0] = 0x0;
                pc = 0x800a4fb8; continue;
                pc = 0x800a4fb4;
            }
            0x800a4fb4 => {
                c.r[0] = 0x1;
                pc = 0x800a4fb8;
            }
            0x800a4fb8 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a4fc8; continue; }
                pc = 0x800a4fc0;
            }
            0x800a4fc0 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x800a4fcc; continue;
                pc = 0x800a4fc8;
            }
            0x800a4fc8 => {
                c.r[0] = c.r[26].wrapping_add(0x1);
                pc = 0x800a4fcc;
            }
            0x800a4fcc => {
                c.r[26] = c.r[0] | c.r[0];
                pc = 0x800a4fd0;
            }
            0x800a4fd0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fac));
                c.r[0] = c.r[3].wrapping_add(0x8);
                c.r[0] = c.lwz(c.r[29].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[26], c.r[0]);
                if c.crbit(0) { pc = 0x800a4ef0; continue; }
                pc = 0x800a4fe4;
            }
            0x800a4fe4 => {
                c.r[3] = 0xffffffff;
                pc = 0x800a5008; continue;
                pc = 0x800a4fec;
            }
            0x800a4fec => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x800a4ffc; continue; }
                pc = 0x800a4ff4;
            }
            0x800a4ff4 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x800a5008; continue;
                pc = 0x800a4ffc;
            }
            0x800a4ffc => {
                c.r[23] = c.r[27].wrapping_add(c.r[23]);
                c.r[23] = c.r[23].wrapping_add(0x1);
                pc = 0x800a4d4c; continue;
                pc = 0x800a5008;
            }
            0x800a5008 => {
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 20..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x4c));
                c.r[1] = c.r[1].wrapping_add(0x48);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a4d28: bad pc {:#x}", pc),
        }
    }
}

/// DVDOpen (0x800a501c, 200 bytes)
#[inline(never)]
pub fn f_800a501c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a501c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a501c); p = true; }
    }
    f_800a501c_b(c, 0x800a501c);
    if p { crate::prof::exit(c, 0x800a501c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a501c_at(c: &mut Cpu, pc: u32) { f_800a501c_b(c, pc) }
#[inline(always)]
fn f_800a501c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a501c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a501c, pc); }
        match pc {
            0x800a501c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffff68); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x94), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x90), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800a503c; f_800a4d28(c);
                pc = 0x800a503c;
            }
            0x800a503c => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x800a5070; continue; }
                pc = 0x800a5044;
            }
            0x800a5044 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.r[4] = 0x80;
                c.lr = 0x800a5050; f_800a5268(c);
                pc = 0x800a5050;
            }
            0x800a5050 => {
                c.r[3] = 0x80310000;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[3] = c.r[3].wrapping_add(0x19b8);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[5] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800a5068; f_8009c2e0(c);
                pc = 0x800a5068;
            }
            0x800a5068 => {
                c.r[3] = 0x0;
                pc = 0x800a50cc; continue;
                pc = 0x800a5070;
            }
            0x800a5070 => {
                c.r[5] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fac));
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[5]));
                c.r[0] = c.r[0].rotate_left(0) & 0xff000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a508c; continue; }
                pc = 0x800a5084;
            }
            0x800a5084 => {
                c.r[0] = 0x0;
                pc = 0x800a5090; continue;
                pc = 0x800a508c;
            }
            0x800a508c => {
                c.r[0] = 0x1;
                pc = 0x800a5090;
            }
            0x800a5090 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a50a0; continue; }
                pc = 0x800a5098;
            }
            0x800a5098 => {
                c.r[3] = 0x0;
                pc = 0x800a50cc; continue;
                pc = 0x800a50a0;
            }
            0x800a50a0 => {
                c.r[3] = c.r[3].wrapping_add(c.r[5]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[0] = 0x0;
                c.r[3] = 0x1;
                c.stw(c.r[31].wrapping_add(0x30), c.r[4]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fac));
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x8));
                c.stw(c.r[31].wrapping_add(0x34), c.r[4]);
                c.stw(c.r[31].wrapping_add(0x38), c.r[0]);
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                pc = 0x800a50cc;
            }
            0x800a50cc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x9c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x94));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x90));
                c.r[1] = c.r[1].wrapping_add(0x98);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a501c: bad pc {:#x}", pc),
        }
    }
}

/// DVDClose (0x800a50e4, 36 bytes)
#[inline(never)]
pub fn f_800a50e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a50e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a50e4); p = true; }
    }
    f_800a50e4_b(c, 0x800a50e4);
    if p { crate::prof::exit(c, 0x800a50e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a50e4_at(c: &mut Cpu, pc: u32) { f_800a50e4_b(c, pc) }
#[inline(always)]
fn f_800a50e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a50e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a50e4, pc); }
        match pc {
            0x800a50e4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a50f4; f_800a7afc(c);
                pc = 0x800a50f4;
            }
            0x800a50f4 => {
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a50e4: bad pc {:#x}", pc),
        }
    }
}

/// entryToPath (0x800a5108, 352 bytes)
#[inline(never)]
pub fn f_800a5108(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5108) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5108); p = true; }
    }
    f_800a5108_b(c, 0x800a5108);
    if p { crate::prof::exit(c, 0x800a5108); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5108_at(c: &mut Cpu, pc: u32) { f_800a5108_b(c, pc) }
#[inline(always)]
fn f_800a5108_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5108;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5108, pc); }
        match pc {
            0x800a5108 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[29] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[28]);
                if !c.crbit(2) { pc = 0x800a513c; continue; }
                pc = 0x800a5134;
            }
            0x800a5134 => {
                c.r[3] = 0x0;
                pc = 0x800a5248; continue;
                pc = 0x800a513c;
            }
            0x800a513c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fac));
                c.r[3] = c.r[3].wrapping_mul(0xc);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff9fb0));
                c.r[5] = c.r[4].wrapping_add(0x4);
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[3]));
                c.r[3] = c.lwz(c.r[5].wrapping_add(c.r[3]));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff;
                c.cmpu(0, c.r[3], 0x0);
                c.r[31] = c.r[6].wrapping_add(c.r[0]);
                if !c.crbit(2) { pc = 0x800a516c; continue; }
                pc = 0x800a5164;
            }
            0x800a5164 => {
                c.r[3] = 0x0;
                pc = 0x800a51e8; continue;
                pc = 0x800a516c;
            }
            0x800a516c => {
                c.r[3] = c.r[3].wrapping_mul(0xc);
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[3]));
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.lwz(c.r[5].wrapping_add(c.r[3]));
                c.r[5] = c.r[30].wrapping_add(0x0);
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff;
                c.r[28] = c.r[6].wrapping_add(c.r[0]);
                c.lr = 0x800a518c; f_800a5108(c);
                pc = 0x800a518c;
            }
            0x800a518c => {
                c.cmpu(0, c.r[3], c.r[30]);
                if !c.crbit(2) { pc = 0x800a5198; continue; }
                pc = 0x800a5194;
            }
            0x800a5194 => {
                pc = 0x800a51e8; continue;
                pc = 0x800a5198;
            }
            0x800a5198 => {
                c.r[0] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.r[4] = 0x2f;
                c.r[6] = c.r[30].wrapping_sub(c.r[3]);
                c.stb(c.r[29].wrapping_add(c.r[0]), c.r[4]);
                c.r[4] = c.r[6].wrapping_add(0x0);
                c.r[5] = c.r[29].wrapping_add(c.r[3]);
                pc = 0x800a51cc; continue;
                pc = 0x800a51b8;
            }
            0x800a51b8 => {
                c.r[0] = c.lbz(c.r[28]);
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.r[4] = c.r[4].wrapping_add(0xffffffff);
                c.stb(c.r[5], c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0x1);
                pc = 0x800a51cc;
            }
            0x800a51cc => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800a51e0; continue; }
                pc = 0x800a51d4;
            }
            0x800a51d4 => {
                c.r[0] = c.lbz(c.r[28]);
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a51b8; continue; }
                pc = 0x800a51e0;
            }
            0x800a51e0 => {
                c.r[0] = c.r[6].wrapping_sub(c.r[4]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x800a51e8;
            }
            0x800a51e8 => {
                c.cmpu(0, c.r[3], c.r[30]);
                if !c.crbit(2) { pc = 0x800a51f4; continue; }
                pc = 0x800a51f0;
            }
            0x800a51f0 => {
                pc = 0x800a5248; continue;
                pc = 0x800a51f4;
            }
            0x800a51f4 => {
                c.r[0] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.r[4] = 0x2f;
                c.r[7] = c.r[30].wrapping_sub(c.r[3]);
                c.stb(c.r[29].wrapping_add(c.r[0]), c.r[4]);
                c.r[6] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[7].wrapping_add(0x0);
                c.r[5] = c.r[29].wrapping_add(c.r[3]);
                pc = 0x800a522c; continue;
                pc = 0x800a5218;
            }
            0x800a5218 => {
                c.r[0] = c.lbz(c.r[6]);
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.r[4] = c.r[4].wrapping_add(0xffffffff);
                c.stb(c.r[5], c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0x1);
                pc = 0x800a522c;
            }
            0x800a522c => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800a5240; continue; }
                pc = 0x800a5234;
            }
            0x800a5234 => {
                c.r[0] = c.lbz(c.r[6]);
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a5218; continue; }
                pc = 0x800a5240;
            }
            0x800a5240 => {
                c.r[0] = c.r[7].wrapping_sub(c.r[4]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x800a5248;
            }
            0x800a5248 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5108: bad pc {:#x}", pc),
        }
    }
}

/// DVDGetCurrentDir (0x800a5268, 196 bytes)
#[inline(never)]
pub fn f_800a5268(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5268) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5268); p = true; }
    }
    f_800a5268_b(c, 0x800a5268);
    if p { crate::prof::exit(c, 0x800a5268); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5268_at(c: &mut Cpu, pc: u32) { f_800a5268_b(c, pc) }
#[inline(always)]
fn f_800a5268_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5268;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5268, pc); }
        match pc {
            0x800a5268 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.r[5] = c.r[30].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffff9fb8));
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.lr = 0x800a529c; f_800a5108(c);
                pc = 0x800a529c;
            }
            0x800a529c => {
                c.cmpu(0, c.r[3], c.r[30]);
                if !c.crbit(2) { pc = 0x800a52b4; continue; }
                pc = 0x800a52a4;
            }
            0x800a52a4 => {
                c.r[0] = 0x0;
                c.r[3] = c.r[29].wrapping_add(c.r[30]);
                c.stb(c.r[3].wrapping_add(0xffffffff), c.r[0]);
                pc = 0x800a530c; continue;
                pc = 0x800a52b4;
            }
            0x800a52b4 => {
                c.r[0] = c.r[31].wrapping_mul(0xc);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fac));
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].rotate_left(0) & 0xff000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a52d0; continue; }
                pc = 0x800a52c8;
            }
            0x800a52c8 => {
                c.r[0] = 0x0;
                pc = 0x800a52d4; continue;
                pc = 0x800a52d0;
            }
            0x800a52d0 => {
                c.r[0] = 0x1;
                pc = 0x800a52d4;
            }
            0x800a52d4 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a5300; continue; }
                pc = 0x800a52dc;
            }
            0x800a52dc => {
                c.r[0] = c.r[30].wrapping_add(0xffffffff);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a52f4; continue; }
                pc = 0x800a52e8;
            }
            0x800a52e8 => {
                c.r[0] = 0x0;
                c.stb(c.r[29].wrapping_add(c.r[3]), c.r[0]);
                pc = 0x800a530c; continue;
                pc = 0x800a52f4;
            }
            0x800a52f4 => {
                c.r[0] = 0x2f;
                c.stb(c.r[29].wrapping_add(c.r[3]), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x1);
                pc = 0x800a5300;
            }
            0x800a5300 => {
                c.r[0] = 0x0;
                c.stb(c.r[29].wrapping_add(c.r[3]), c.r[0]);
                c.r[0] = 0x1;
                pc = 0x800a530c;
            }
            0x800a530c => {
                c.r[3] = c.r[0] | c.r[0];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5268: bad pc {:#x}", pc),
        }
    }
}

/// DVDReadAsync (0x800a532c, 192 bytes)
#[inline(never)]
pub fn f_800a532c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a532c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a532c); p = true; }
    }
    f_800a532c_b(c, 0x800a532c);
    if p { crate::prof::exit(c, 0x800a532c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a532c_at(c: &mut Cpu, pc: u32) { f_800a532c_b(c, pc) }
#[inline(always)]
fn f_800a532c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a532c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a532c, pc); }
        match pc {
            0x800a532c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x20); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[6] | c.r[6];
                c.rc(c.r[29]);
                c.r[26] = c.r[3].wrapping_add(0x0);
                c.r[27] = c.r[4].wrapping_add(0x0);
                c.r[28] = c.r[5].wrapping_add(0x0);
                c.r[30] = c.r[7].wrapping_add(0x0);
                c.r[31] = c.r[8].wrapping_add(0x0);
                if c.crbit(0) { pc = 0x800a5364; continue; }
                pc = 0x800a5358;
            }
            0x800a5358 => {
                c.r[0] = c.lwz(c.r[26].wrapping_add(0x34));
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x800a537c; continue; }
                pc = 0x800a5364;
            }
            0x800a5364 => {
                c.r[3] = 0x80310000;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[5] = c.r[3].wrapping_add(0x19f0);
                c.r[3] = c.r[13].wrapping_add(0xffff81a0);
                c.r[4] = 0x2e6;
                c.lr = 0x800a537c; f_800060f0(c);
                pc = 0x800a537c;
            }
            0x800a537c => {
                c.r[4] = c.r[29].wrapping_add(c.r[28]);
                c.rc(c.r[4]);
                if c.crbit(0) { pc = 0x800a5394; continue; }
                pc = 0x800a5384;
            }
            0x800a5384 => {
                c.r[3] = c.lwz(c.r[26].wrapping_add(0x34));
                c.r[0] = c.r[3].wrapping_add(0x20);
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x800a53ac; continue; }
                pc = 0x800a5394;
            }
            0x800a5394 => {
                c.r[3] = 0x80310000;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[5] = c.r[3].wrapping_add(0x19f0);
                c.r[3] = c.r[13].wrapping_add(0xffff81a0);
                c.r[4] = 0x2ec;
                c.lr = 0x800a53ac; f_800060f0(c);
                pc = 0x800a53ac;
            }
            0x800a53ac => {
                c.stw(c.r[26].wrapping_add(0x38), c.r[30]);
                c.r[3] = 0x800a0000;
                c.r[7] = c.r[3].wrapping_add(0x53ec);
                c.r[0] = c.lwz(c.r[26].wrapping_add(0x30));
                c.r[3] = c.r[26].wrapping_add(0x0);
                c.r[4] = c.r[27].wrapping_add(0x0);
                c.r[5] = c.r[28].wrapping_add(0x0);
                c.r[8] = c.r[31].wrapping_add(0x0);
                c.r[6] = c.r[0].wrapping_add(c.r[29]);
                c.lr = 0x800a53d4; f_800a720c(c);
                pc = 0x800a53d4;
            }
            0x800a53d4 => {
                { let mut ea = c.r[1].wrapping_add(0x20); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a532c: bad pc {:#x}", pc),
        }
    }
}

/// cbForReadAsync (0x800a53ec, 48 bytes)
#[inline(never)]
pub fn f_800a53ec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a53ec) { return; }
        if c.prof { crate::prof::enter(c, 0x800a53ec); p = true; }
    }
    f_800a53ec_b(c, 0x800a53ec);
    if p { crate::prof::exit(c, 0x800a53ec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a53ec_at(c: &mut Cpu, pc: u32) { f_800a53ec_b(c, pc) }
#[inline(always)]
fn f_800a53ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a53ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a53ec, pc); }
        match pc {
            0x800a53ec => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x38));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a540c; continue; }
                pc = 0x800a5404;
            }
            0x800a5404 => {
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a540c; (c.dispatch)(c, t); }
                pc = 0x800a540c;
            }
            0x800a540c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a53ec: bad pc {:#x}", pc),
        }
    }
}

/// DVDRead (0x800a541c, 280 bytes)
#[inline(never)]
pub fn f_800a541c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a541c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a541c); p = true; }
    }
    f_800a541c_b(c, 0x800a541c);
    if p { crate::prof::exit(c, 0x800a541c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a541c_at(c: &mut Cpu, pc: u32) { f_800a541c_b(c, pc) }
#[inline(always)]
fn f_800a541c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a541c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a541c, pc); }
        match pc {
            0x800a541c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[6] | c.r[6];
                c.rc(c.r[29]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[27] = c.r[4].wrapping_add(0x0);
                c.r[28] = c.r[5].wrapping_add(0x0);
                c.r[30] = c.r[7].wrapping_add(0x0);
                if c.crbit(0) { pc = 0x800a5450; continue; }
                pc = 0x800a5444;
            }
            0x800a5444 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x34));
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x800a5468; continue; }
                pc = 0x800a5450;
            }
            0x800a5450 => {
                c.r[3] = 0x80310000;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[5] = c.r[3].wrapping_add(0x1a24);
                c.r[3] = c.r[13].wrapping_add(0xffff81a0);
                c.r[4] = 0x32c;
                c.lr = 0x800a5468; f_800060f0(c);
                pc = 0x800a5468;
            }
            0x800a5468 => {
                c.r[4] = c.r[29].wrapping_add(c.r[28]);
                c.rc(c.r[4]);
                if c.crbit(0) { pc = 0x800a5480; continue; }
                pc = 0x800a5470;
            }
            0x800a5470 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x34));
                c.r[0] = c.r[3].wrapping_add(0x20);
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x800a5498; continue; }
                pc = 0x800a5480;
            }
            0x800a5480 => {
                c.r[3] = 0x80310000;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[5] = c.r[3].wrapping_add(0x1a24);
                c.r[3] = c.r[13].wrapping_add(0xffff81a0);
                c.r[4] = 0x332;
                c.lr = 0x800a5498; f_800060f0(c);
                pc = 0x800a5498;
            }
            0x800a5498 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x30));
                c.r[4] = 0x800a0000;
                c.r[7] = c.r[4].wrapping_add(0x5534);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[27].wrapping_add(0x0);
                c.r[5] = c.r[28].wrapping_add(0x0);
                c.r[8] = c.r[30].wrapping_add(0x0);
                c.r[6] = c.r[0].wrapping_add(c.r[29]);
                c.lr = 0x800a54bc; f_800a720c(c);
                pc = 0x800a54bc;
            }
            0x800a54bc => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a54cc; continue; }
                pc = 0x800a54c4;
            }
            0x800a54c4 => {
                c.r[3] = 0xffffffff;
                pc = 0x800a5520; continue;
                pc = 0x800a54cc;
            }
            0x800a54cc => {
                c.lr = 0x800a54d0; f_8009df3c(c);
                pc = 0x800a54d0;
            }
            0x800a54d0 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800a54d4;
            }
            0x800a54d4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a54e8; continue; }
                pc = 0x800a54e0;
            }
            0x800a54e0 => {
                c.r[31] = c.lwz(c.r[31].wrapping_add(0x20));
                pc = 0x800a5514; continue;
                pc = 0x800a54e8;
            }
            0x800a54e8 => {
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(2) { pc = 0x800a54f8; continue; }
                pc = 0x800a54f0;
            }
            0x800a54f0 => {
                c.r[31] = 0xffffffff;
                pc = 0x800a5514; continue;
                pc = 0x800a54f8;
            }
            0x800a54f8 => {
                c.cmps(0, c.r[0], 0xa);
                if !c.crbit(2) { pc = 0x800a5508; continue; }
                pc = 0x800a5500;
            }
            0x800a5500 => {
                c.r[31] = 0xfffffffd;
                pc = 0x800a5514; continue;
                pc = 0x800a5508;
            }
            0x800a5508 => {
                c.r[3] = c.r[13].wrapping_add(0xffff9fc0);
                c.lr = 0x800a5510; f_800a238c(c);
                pc = 0x800a5510;
            }
            0x800a5510 => {
                pc = 0x800a54d4; continue;
                pc = 0x800a5514;
            }
            0x800a5514 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a551c; f_8009df64(c);
                pc = 0x800a551c;
            }
            0x800a551c => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800a5520;
            }
            0x800a5520 => {
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a541c: bad pc {:#x}", pc),
        }
    }
}

/// cbForReadSync (0x800a5534, 36 bytes)
#[inline(never)]
pub fn f_800a5534(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5534) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5534); p = true; }
    }
    f_800a5534_b(c, 0x800a5534);
    if p { crate::prof::exit(c, 0x800a5534); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5534_at(c: &mut Cpu, pc: u32) { f_800a5534_b(c, pc) }
#[inline(always)]
fn f_800a5534_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5534;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5534, pc); }
        match pc {
            0x800a5534 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[13].wrapping_add(0xffff9fc0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a5548; f_800a2478(c);
                pc = 0x800a5548;
            }
            0x800a5548 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5534: bad pc {:#x}", pc),
        }
    }
}

/// DVDSeekAsyncPrio (0x800a5558, 152 bytes)
#[inline(never)]
pub fn f_800a5558(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5558) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5558); p = true; }
    }
    f_800a5558_b(c, 0x800a5558);
    if p { crate::prof::exit(c, 0x800a5558); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5558_at(c: &mut Cpu, pc: u32) { f_800a5558_b(c, pc) }
#[inline(always)]
fn f_800a5558_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5558;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5558, pc); }
        match pc {
            0x800a5558 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[6].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.rc(c.r[29]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[28]);
                c.r[28] = c.r[3].wrapping_add(0x0);
                if c.crbit(0) { pc = 0x800a5594; continue; }
                pc = 0x800a5588;
            }
            0x800a5588 => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x34));
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x800a55ac; continue; }
                pc = 0x800a5594;
            }
            0x800a5594 => {
                c.r[3] = 0x80310000;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[5] = c.r[3].wrapping_add(0x1a54);
                c.r[3] = c.r[13].wrapping_add(0xffff81a0);
                c.r[4] = 0x383;
                c.lr = 0x800a55ac; f_800060f0(c);
                pc = 0x800a55ac;
            }
            0x800a55ac => {
                c.stw(c.r[28].wrapping_add(0x38), c.r[30]);
                c.r[3] = 0x800a0000;
                c.r[5] = c.r[3].wrapping_add(0x55f0);
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x30));
                c.r[3] = c.r[28].wrapping_add(0x0);
                c.r[6] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[0].wrapping_add(c.r[29]);
                c.lr = 0x800a55cc; f_800a72e8(c);
                pc = 0x800a55cc;
            }
            0x800a55cc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5558: bad pc {:#x}", pc),
        }
    }
}

/// cbForSeekAsync (0x800a55f0, 48 bytes)
#[inline(never)]
pub fn f_800a55f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a55f0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a55f0); p = true; }
    }
    f_800a55f0_b(c, 0x800a55f0);
    if p { crate::prof::exit(c, 0x800a55f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a55f0_at(c: &mut Cpu, pc: u32) { f_800a55f0_b(c, pc) }
#[inline(always)]
fn f_800a55f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a55f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a55f0, pc); }
        match pc {
            0x800a55f0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x38));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a5610; continue; }
                pc = 0x800a5608;
            }
            0x800a5608 => {
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a5610; (c.dispatch)(c, t); }
                pc = 0x800a5610;
            }
            0x800a5610 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a55f0: bad pc {:#x}", pc),
        }
    }
}

/// defaultOptionalCommandChecker (0x800a5620, 4 bytes)
#[inline(never)]
pub fn f_800a5620(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5620) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5620); p = true; }
    }
    f_800a5620_b(c, 0x800a5620);
    if p { crate::prof::exit(c, 0x800a5620); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5620_at(c: &mut Cpu, pc: u32) { f_800a5620_b(c, pc) }
#[inline(always)]
fn f_800a5620_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5620;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5620, pc); }
        match pc {
            0x800a5620 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800a5620: bad pc {:#x}", pc),
        }
    }
}

/// DVDInit (0x800a5624, 204 bytes)
#[inline(never)]
pub fn f_800a5624(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5624) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5624); p = true; }
    }
    f_800a5624_b(c, 0x800a5624);
    if p { crate::prof::exit(c, 0x800a5624); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5624_at(c: &mut Cpu, pc: u32) { f_800a5624_b(c, pc) }
#[inline(always)]
fn f_800a5624_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5624;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5624, pc); }
        match pc {
            0x800a5624 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa008));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a56dc; continue; }
                pc = 0x800a5640;
            }
            0x800a5640 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81a8));
                c.lr = 0x800a5648; f_8009a250(c);
                pc = 0x800a5648;
            }
            0x800a5648 => {
                c.r[31] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa008), c.r[31]);
                c.lr = 0x800a5654; f_800a4cf0(c);
                pc = 0x800a5654;
            }
            0x800a5654 => {
                c.lr = 0x800a5658; f_800a7de8(c);
                pc = 0x800a5658;
            }
            0x800a5658 => {
                c.lr = 0x800a565c; f_800a3eb0(c);
                pc = 0x800a565c;
            }
            0x800a565c => {
                c.r[0] = 0x80000000;
                c.r[3] = 0x800a0000;
                c.stw(c.r[13].wrapping_add(0xffff9fd0), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x3ef0);
                c.stw(c.r[13].wrapping_add(0xffff9fcc), c.r[0]);
                c.r[3] = 0x15;
                c.lr = 0x800a5678; f_8009df88(c);
                pc = 0x800a5678;
            }
            0x800a5678 => {
                c.r[3] = 0x400;
                c.lr = 0x800a5680; f_8009e38c(c);
                pc = 0x800a5680;
            }
            0x800a5680 => {
                c.r[3] = c.r[13].wrapping_add(0xffff9fc0);
                c.lr = 0x800a5688; f_800a13e8(c);
                pc = 0x800a5688;
            }
            0x800a5688 => {
                c.r[3] = 0xcc000000;
                c.r[0] = 0x2a;
                c.stw(c.r[3].wrapping_add(0x6000), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x6004), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fd0));
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.r[3] = c.lwz(c.r[3]);
                c.r[0] = c.r[3].wrapping_add(0x1ae00000);
                c.cmpu(0, c.r[0], 0x7c22);
                if !c.crbit(2) { pc = 0x800a56cc; continue; }
                pc = 0x800a56b4;
            }
            0x800a56b4 => {
                c.r[3] = 0x80310000;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[3] = c.r[3].wrapping_add(0x1ac8);
                c.lr = 0x800a56c4; f_8009c2e0(c);
                pc = 0x800a56c4;
            }
            0x800a56c4 => {
                c.lr = 0x800a56c8; f_800a8474(c);
                pc = 0x800a56c8;
            }
            0x800a56c8 => {
                pc = 0x800a56dc; continue;
                pc = 0x800a56cc;
            }
            0x800a56cc => {
                c.r[0] = c.r[3].wrapping_add(0xf2eb0000);
                c.cmpu(0, c.r[0], 0xea5e);
                if c.crbit(2) { pc = 0x800a56dc; continue; }
                pc = 0x800a56d8;
            }
            0x800a56d8 => {
                c.stw(c.r[13].wrapping_add(0xffffa004), c.r[31]);
                pc = 0x800a56dc;
            }
            0x800a56dc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5624: bad pc {:#x}", pc),
        }
    }
}

/// stateReadingFST (0x800a56f0, 148 bytes)
#[inline(never)]
pub fn f_800a56f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a56f0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a56f0); p = true; }
    }
    f_800a56f0_b(c, 0x800a56f0);
    if p { crate::prof::exit(c, 0x800a56f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a56f0_at(c: &mut Cpu, pc: u32) { f_800a56f0_b(c, pc) }
#[inline(always)]
fn f_800a56f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a56f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a56f0, pc); }
        match pc {
            0x800a56f0 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x800a0000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x56f0);
                c.r[3] = 0x80400000;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[3] = c.r[3].wrapping_add(0xffffc360);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x8);
                c.stw(c.r[13].wrapping_add(0xffffa00c), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fd0));
                c.r[0] = c.lwz(c.r[31]);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x3c));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x800a5744; continue; }
                pc = 0x800a572c;
            }
            0x800a572c => {
                c.r[3] = 0x80310000;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[5] = c.r[3].wrapping_add(0x1ad4);
                c.r[3] = c.r[13].wrapping_add(0xffff81b4);
                c.r[4] = 0x287;
                c.lr = 0x800a5744; f_800060f0(c);
                pc = 0x800a5744;
            }
            0x800a5744 => {
                c.r[3] = 0x80400000;
                c.r[6] = c.lwz(c.r[31]);
                c.r[5] = c.r[3].wrapping_add(0xffffc360);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff9fd0));
                c.r[4] = 0x800a0000;
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[6].wrapping_add(0x1f);
                c.r[3] = c.lwz(c.r[7].wrapping_add(0x38));
                c.r[6] = c.r[4].wrapping_add(0x5784);
                c.r[4] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.lr = 0x800a5770; f_800a4454(c);
                pc = 0x800a5770;
            }
            0x800a5770 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a56f0: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateReadingFST (0x800a5784, 140 bytes)
#[inline(never)]
pub fn f_800a5784(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5784) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5784); p = true; }
    }
    f_800a5784_b(c, 0x800a5784);
    if p { crate::prof::exit(c, 0x800a5784); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5784_at(c: &mut Cpu, pc: u32) { f_800a5784_b(c, pc) }
#[inline(always)]
fn f_800a5784_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5784;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5784, pc); }
        match pc {
            0x800a5784 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                if !c.crbit(2) { pc = 0x800a57b0; continue; }
                pc = 0x800a579c;
            }
            0x800a579c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.lr = 0x800a57ac; f_800a58bc(c);
                pc = 0x800a57ac;
            }
            0x800a57ac => {
                pc = 0x800a57fc; continue;
                pc = 0x800a57b0;
            }
            0x800a57b0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a57f8; continue; }
                pc = 0x800a57b8;
            }
            0x800a57b8 => {
                c.r[31] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9ffc), c.r[31]);
                c.lr = 0x800a57c4; f_800a4cf0(c);
                pc = 0x800a57c4;
            }
            0x800a57c4 => {
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = c.r[3].wrapping_add(0xffffc3a0);
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[31]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a57f0; continue; }
                pc = 0x800a57e4;
            }
            0x800a57e4 => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a57f0; (c.dispatch)(c, t); }
                pc = 0x800a57f0;
            }
            0x800a57f0 => {
                c.lr = 0x800a57f4; f_800a6684(c);
                pc = 0x800a57f4;
            }
            0x800a57f4 => {
                pc = 0x800a57fc; continue;
                pc = 0x800a57f8;
            }
            0x800a57f8 => {
                c.lr = 0x800a57fc; f_800a58f0(c);
                pc = 0x800a57fc;
            }
            0x800a57fc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5784: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateError (0x800a5810, 172 bytes)
#[inline(never)]
pub fn f_800a5810(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5810) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5810); p = true; }
    }
    f_800a5810_b(c, 0x800a5810);
    if p { crate::prof::exit(c, 0x800a5810); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5810_at(c: &mut Cpu, pc: u32) { f_800a5810_b(c, pc) }
#[inline(always)]
fn f_800a5810_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5810;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5810, pc); }
        match pc {
            0x800a5810 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                if !c.crbit(2) { pc = 0x800a583c; continue; }
                pc = 0x800a5828;
            }
            0x800a5828 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.lr = 0x800a5838; f_800a58bc(c);
                pc = 0x800a5838;
            }
            0x800a5838 => {
                pc = 0x800a58a8; continue;
                pc = 0x800a583c;
            }
            0x800a583c => {
                c.lr = 0x800a5840; f_800a836c(c);
                pc = 0x800a5840;
            }
            0x800a5840 => {
                c.r[3] = 0x80400000;
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = c.r[3].wrapping_add(0xffffc3a0);
                c.r[3] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9fe0), c.r[3]);
                c.r[12] = c.lwz(c.r[31].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a5874; continue; }
                pc = 0x800a5864;
            }
            0x800a5864 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0xffffffff;
                if true { let t = c.lr; c.lr = 0x800a5874; (c.dispatch)(c, t); }
                pc = 0x800a5874;
            }
            0x800a5874 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a58a4; continue; }
                pc = 0x800a5880;
            }
            0x800a5880 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9fec));
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[0]);
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a58a4; continue; }
                pc = 0x800a5894;
            }
            0x800a5894 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a58a4; (c.dispatch)(c, t); }
                pc = 0x800a58a4;
            }
            0x800a58a4 => {
                c.lr = 0x800a58a8; f_800a6684(c);
                pc = 0x800a58a8;
            }
            0x800a58a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5810: bad pc {:#x}", pc),
        }
    }
}

/// stateTimeout (0x800a58bc, 52 bytes)
#[inline(never)]
pub fn f_800a58bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a58bc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a58bc); p = true; }
    }
    f_800a58bc_b(c, 0x800a58bc);
    if p { crate::prof::exit(c, 0x800a58bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a58bc_at(c: &mut Cpu, pc: u32) { f_800a58bc_b(c, pc) }
#[inline(always)]
fn f_800a58bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a58bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a58bc, pc); }
        match pc {
            0x800a58bc => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x1230000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a58d4; f_800a80fc(c);
                pc = 0x800a58d4;
            }
            0x800a58d4 => {
                c.lr = 0x800a58d8; f_800a76e4(c);
                pc = 0x800a58d8;
            }
            0x800a58d8 => {
                c.r[3] = 0x0;
                c.lr = 0x800a58e0; f_800a5810(c);
                pc = 0x800a58e0;
            }
            0x800a58e0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a58bc: bad pc {:#x}", pc),
        }
    }
}

/// stateGettingError (0x800a58f0, 40 bytes)
#[inline(never)]
pub fn f_800a58f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a58f0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a58f0); p = true; }
    }
    f_800a58f0_b(c, 0x800a58f0);
    if p { crate::prof::exit(c, 0x800a58f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a58f0_at(c: &mut Cpu, pc: u32) { f_800a58f0_b(c, pc) }
#[inline(always)]
fn f_800a58f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a58f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a58f0, pc); }
        match pc {
            0x800a58f0 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x800a0000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x59cc);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a5908; f_800a48dc(c);
                pc = 0x800a5908;
            }
            0x800a5908 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a58f0: bad pc {:#x}", pc),
        }
    }
}

/// CategorizeError (0x800a5918, 180 bytes)
#[inline(never)]
pub fn f_800a5918(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5918) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5918); p = true; }
    }
    f_800a5918_b(c, 0x800a5918);
    if p { crate::prof::exit(c, 0x800a5918); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5918_at(c: &mut Cpu, pc: u32) { f_800a5918_b(c, pc) }
#[inline(always)]
fn f_800a5918_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5918;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5918, pc); }
        match pc {
            0x800a5918 => {
                c.r[0] = c.r[3].wrapping_add(0xfffe0000);
                c.cmpu(0, c.r[0], 0x400);
                if !c.crbit(2) { pc = 0x800a5930; continue; }
                pc = 0x800a5924;
            }
            0x800a5924 => {
                c.stw(c.r[13].wrapping_add(0xffff9ff8), c.r[3]);
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x800a5930;
            }
            0x800a5930 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffffff;
                c.r[0] = c.r[4].wrapping_add(0xfffa0000);
                c.cmpu(0, c.r[0], 0x2800);
                if c.crbit(2) { pc = 0x800a5958; continue; }
                pc = 0x800a5940;
            }
            0x800a5940 => {
                c.r[0] = c.r[4].wrapping_add(0xfffe0000);
                c.cmpu(0, c.r[0], 0x3a00);
                if c.crbit(2) { pc = 0x800a5958; continue; }
                pc = 0x800a594c;
            }
            0x800a594c => {
                c.r[0] = c.r[4].wrapping_add(0xfff50000);
                c.cmpu(0, c.r[0], 0x5a01);
                if !c.crbit(2) { pc = 0x800a5960; continue; }
                pc = 0x800a5958;
            }
            0x800a5958 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x800a5960;
            }
            0x800a5960 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ffc));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffff9ffc), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9ffc));
                c.cmps(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x800a599c; continue; }
                pc = 0x800a5978;
            }
            0x800a5978 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9ff8));
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(2) { pc = 0x800a5990; continue; }
                pc = 0x800a5984;
            }
            0x800a5984 => {
                c.stw(c.r[13].wrapping_add(0xffff9ff8), c.r[4]);
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x800a5990;
            }
            0x800a5990 => {
                c.stw(c.r[13].wrapping_add(0xffff9ff8), c.r[4]);
                c.r[3] = 0x2;
                if true { return; }
                pc = 0x800a599c;
            }
            0x800a599c => {
                c.r[0] = c.r[4].wrapping_add(0xfffd0000);
                c.stw(c.r[13].wrapping_add(0xffff9ff8), c.r[4]);
                c.cmpu(0, c.r[0], 0x1100);
                if c.crbit(2) { pc = 0x800a59bc; continue; }
                pc = 0x800a59ac;
            }
            0x800a59ac => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x5);
                if !c.crbit(2) { pc = 0x800a59c4; continue; }
                pc = 0x800a59bc;
            }
            0x800a59bc => {
                c.r[3] = 0x2;
                if true { return; }
                pc = 0x800a59c4;
            }
            0x800a59c4 => {
                c.r[3] = 0x3;
                if true { return; }
                return;
            }
            _ => panic!("f_800a5918: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateGettingError (0x800a59cc, 660 bytes)
#[inline(never)]
pub fn f_800a59cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a59cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a59cc); p = true; }
    }
    f_800a59cc_b(c, 0x800a59cc);
    if p { crate::prof::exit(c, 0x800a59cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a59cc_at(c: &mut Cpu, pc: u32) { f_800a59cc_b(c, pc) }
#[inline(never)]
fn f_800a59cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a59cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a59cc, pc); }
        match pc {
            0x800a59cc => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                if !c.crbit(2) { pc = 0x800a5a18; continue; }
                pc = 0x800a59f0;
            }
            0x800a59f0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                c.lr = 0x800a5a08; f_800a80fc(c);
                pc = 0x800a5a08;
            }
            0x800a5a08 => {
                c.lr = 0x800a5a0c; f_800a76e4(c);
                pc = 0x800a5a0c;
            }
            0x800a5a0c => {
                c.r[3] = 0x0;
                c.lr = 0x800a5a14; f_800a5810(c);
                pc = 0x800a5a14;
            }
            0x800a5a14 => {
                pc = 0x800a5c40; continue;
                pc = 0x800a5a18;
            }
            0x800a5a18 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a5a48; continue; }
                pc = 0x800a5a20;
            }
            0x800a5a20 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4567);
                c.lr = 0x800a5a38; f_800a80fc(c);
                pc = 0x800a5a38;
            }
            0x800a5a38 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5810);
                c.lr = 0x800a5a44; f_800a4850(c);
                pc = 0x800a5a44;
            }
            0x800a5a44 => {
                pc = 0x800a5c40; continue;
                pc = 0x800a5a48;
            }
            0x800a5a48 => {
                c.r[3] = 0xcc000000;
                c.r[29] = c.lwz(c.r[3].wrapping_add(0x6020));
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[28] = c.r[29].rotate_left(0) & 0xff000000;
                c.lr = 0x800a5a5c; f_800a5918(c);
                pc = 0x800a5a5c;
            }
            0x800a5a5c => {
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[31], 0x1);
                if !c.crbit(2) { pc = 0x800a5a8c; continue; }
                pc = 0x800a5a68;
            }
            0x800a5a68 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.lr = 0x800a5a7c; f_800a80fc(c);
                pc = 0x800a5a7c;
            }
            0x800a5a7c => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5810);
                c.lr = 0x800a5a88; f_800a4850(c);
                pc = 0x800a5a88;
            }
            0x800a5a88 => {
                pc = 0x800a5c40; continue;
                pc = 0x800a5a8c;
            }
            0x800a5a8c => {
                c.r[0] = c.r[31].wrapping_add(0xfffffffe);
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(1) { pc = 0x800a5aa0; continue; }
                pc = 0x800a5a98;
            }
            0x800a5a98 => {
                c.r[4] = 0x0;
                pc = 0x800a5ae0; continue;
                pc = 0x800a5aa0;
            }
            0x800a5aa0 => {
                c.r[0] = c.r[28].wrapping_add(0xff000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a5ab4; continue; }
                pc = 0x800a5aac;
            }
            0x800a5aac => {
                c.r[4] = 0x4;
                pc = 0x800a5ae0; continue;
                pc = 0x800a5ab4;
            }
            0x800a5ab4 => {
                c.r[0] = c.r[28].wrapping_add(0xfe000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a5ac8; continue; }
                pc = 0x800a5ac0;
            }
            0x800a5ac0 => {
                c.r[4] = 0x6;
                pc = 0x800a5ae0; continue;
                pc = 0x800a5ac8;
            }
            0x800a5ac8 => {
                c.r[0] = c.r[28].wrapping_add(0xfd000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a5adc; continue; }
                pc = 0x800a5ad4;
            }
            0x800a5ad4 => {
                c.r[4] = 0x3;
                pc = 0x800a5ae0; continue;
                pc = 0x800a5adc;
            }
            0x800a5adc => {
                c.r[4] = 0x5;
                pc = 0x800a5ae0;
            }
            0x800a5ae0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a5b54; continue; }
                pc = 0x800a5aec;
            }
            0x800a5aec => {
                c.r[3] = 0x80400000;
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[4]);
                c.r[0] = 0x0;
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[3] = c.r[3].wrapping_add(0xffffc3a0);
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[0]);
                c.r[0] = 0xa;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[3]);
                c.stw(c.r[30].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[30].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a5b2c; continue; }
                pc = 0x800a5b1c;
            }
            0x800a5b1c => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0xfffffffd;
                if true { let t = c.lr; c.lr = 0x800a5b2c; (c.dispatch)(c, t); }
                pc = 0x800a5b2c;
            }
            0x800a5b2c => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9fec));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a5b48; continue; }
                pc = 0x800a5b38;
            }
            0x800a5b38 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a5b48; (c.dispatch)(c, t); }
                pc = 0x800a5b48;
            }
            0x800a5b48 => {
                c.lr = 0x800a5b4c; f_800a6684(c);
                pc = 0x800a5b4c;
            }
            0x800a5b4c => {
                c.r[0] = 0x1;
                pc = 0x800a5b58; continue;
                pc = 0x800a5b54;
            }
            0x800a5b54 => {
                c.r[0] = 0x0;
                pc = 0x800a5b58;
            }
            0x800a5b58 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a5c40; continue; }
                pc = 0x800a5b60;
            }
            0x800a5b60 => {
                c.cmpu(0, c.r[31], 0x2);
                if !c.crbit(2) { pc = 0x800a5b78; continue; }
                pc = 0x800a5b68;
            }
            0x800a5b68 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a5b70; f_800a80fc(c);
                pc = 0x800a5b70;
            }
            0x800a5b70 => {
                c.lr = 0x800a5b74; f_800a5d60(c);
                pc = 0x800a5b74;
            }
            0x800a5b74 => {
                pc = 0x800a5c40; continue;
                pc = 0x800a5b78;
            }
            0x800a5b78 => {
                c.cmpu(0, c.r[31], 0x3);
                if !c.crbit(2) { pc = 0x800a5bbc; continue; }
                pc = 0x800a5b80;
            }
            0x800a5b80 => {
                c.r[3] = c.r[29].rotate_left(0) & 0xffffff;
                c.r[0] = c.r[3].wrapping_add(0xfffd0000);
                c.cmpu(0, c.r[0], 0x1100);
                if !c.crbit(2) { pc = 0x800a5ba8; continue; }
                pc = 0x800a5b90;
            }
            0x800a5b90 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[3] = 0x800a0000;
                c.r[4] = c.r[3].wrapping_add(0x5c60);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x10));
                c.lr = 0x800a5ba4; f_800a46ec(c);
                pc = 0x800a5ba4;
            }
            0x800a5ba4 => {
                pc = 0x800a5c40; continue;
                pc = 0x800a5ba8;
            }
            0x800a5ba8 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa00c));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a5bb8; (c.dispatch)(c, t); }
                pc = 0x800a5bb8;
            }
            0x800a5bb8 => {
                pc = 0x800a5c40; continue;
                pc = 0x800a5bbc;
            }
            0x800a5bbc => {
                c.r[0] = c.r[28].wrapping_add(0xff000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a5bdc; continue; }
                pc = 0x800a5bc8;
            }
            0x800a5bc8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0x5;
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.lr = 0x800a5bd8; f_800a6578(c);
                pc = 0x800a5bd8;
            }
            0x800a5bd8 => {
                pc = 0x800a5c40; continue;
                pc = 0x800a5bdc;
            }
            0x800a5bdc => {
                c.r[0] = c.r[28].wrapping_add(0xfe000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a5bfc; continue; }
                pc = 0x800a5be8;
            }
            0x800a5be8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0x3;
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.lr = 0x800a5bf8; f_800a640c(c);
                pc = 0x800a5bf8;
            }
            0x800a5bf8 => {
                pc = 0x800a5c40; continue;
                pc = 0x800a5bfc;
            }
            0x800a5bfc => {
                c.r[0] = c.r[28].wrapping_add(0xfd000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a5c1c; continue; }
                pc = 0x800a5c08;
            }
            0x800a5c08 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0x4;
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.lr = 0x800a5c18; f_800a6578(c);
                pc = 0x800a5c18;
            }
            0x800a5c18 => {
                pc = 0x800a5c40; continue;
                pc = 0x800a5c1c;
            }
            0x800a5c1c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4567);
                c.lr = 0x800a5c34; f_800a80fc(c);
                pc = 0x800a5c34;
            }
            0x800a5c34 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5810);
                c.lr = 0x800a5c40; f_800a4850(c);
                pc = 0x800a5c40;
            }
            0x800a5c40 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a59cc: bad pc {:#x}", pc),
        }
    }
}

/// cbForUnrecoveredError (0x800a5c60, 104 bytes)
#[inline(never)]
pub fn f_800a5c60(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5c60) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5c60); p = true; }
    }
    f_800a5c60_b(c, 0x800a5c60);
    if p { crate::prof::exit(c, 0x800a5c60); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5c60_at(c: &mut Cpu, pc: u32) { f_800a5c60_b(c, pc) }
#[inline(always)]
fn f_800a5c60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5c60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5c60, pc); }
        match pc {
            0x800a5c60 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                if !c.crbit(2) { pc = 0x800a5c9c; continue; }
                pc = 0x800a5c74;
            }
            0x800a5c74 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                c.lr = 0x800a5c8c; f_800a80fc(c);
                pc = 0x800a5c8c;
            }
            0x800a5c8c => {
                c.lr = 0x800a5c90; f_800a76e4(c);
                pc = 0x800a5c90;
            }
            0x800a5c90 => {
                c.r[3] = 0x0;
                c.lr = 0x800a5c98; f_800a5810(c);
                pc = 0x800a5c98;
            }
            0x800a5c98 => {
                pc = 0x800a5cb8; continue;
                pc = 0x800a5c9c;
            }
            0x800a5c9c => {
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a5cac; continue; }
                pc = 0x800a5ca4;
            }
            0x800a5ca4 => {
                c.lr = 0x800a5ca8; f_800a5d60(c);
                pc = 0x800a5ca8;
            }
            0x800a5ca8 => {
                pc = 0x800a5cb8; continue;
                pc = 0x800a5cac;
            }
            0x800a5cac => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5cc8);
                c.lr = 0x800a5cb8; f_800a48dc(c);
                pc = 0x800a5cb8;
            }
            0x800a5cb8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5c60: bad pc {:#x}", pc),
        }
    }
}

/// cbForUnrecoveredErrorRetry (0x800a5cc8, 152 bytes)
#[inline(never)]
pub fn f_800a5cc8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5cc8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5cc8); p = true; }
    }
    f_800a5cc8_b(c, 0x800a5cc8);
    if p { crate::prof::exit(c, 0x800a5cc8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5cc8_at(c: &mut Cpu, pc: u32) { f_800a5cc8_b(c, pc) }
#[inline(always)]
fn f_800a5cc8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5cc8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5cc8, pc); }
        match pc {
            0x800a5cc8 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                if !c.crbit(2) { pc = 0x800a5d04; continue; }
                pc = 0x800a5cdc;
            }
            0x800a5cdc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                c.lr = 0x800a5cf4; f_800a80fc(c);
                pc = 0x800a5cf4;
            }
            0x800a5cf4 => {
                c.lr = 0x800a5cf8; f_800a76e4(c);
                pc = 0x800a5cf8;
            }
            0x800a5cf8 => {
                c.r[3] = 0x0;
                c.lr = 0x800a5d00; f_800a5810(c);
                pc = 0x800a5d00;
            }
            0x800a5d00 => {
                pc = 0x800a5d50; continue;
                pc = 0x800a5d04;
            }
            0x800a5d04 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[4] = 0xffffffff;
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if c.crbit(2) { pc = 0x800a5d34; continue; }
                pc = 0x800a5d18;
            }
            0x800a5d18 => {
                c.r[3] = 0x1230000;
                c.r[3] = c.r[3].wrapping_add(0x4567);
                c.lr = 0x800a5d24; f_800a80fc(c);
                pc = 0x800a5d24;
            }
            0x800a5d24 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5810);
                c.lr = 0x800a5d30; f_800a4850(c);
                pc = 0x800a5d30;
            }
            0x800a5d30 => {
                pc = 0x800a5d50; continue;
                pc = 0x800a5d34;
            }
            0x800a5d34 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6000);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x20));
                c.lr = 0x800a5d44; f_800a80fc(c);
                pc = 0x800a5d44;
            }
            0x800a5d44 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5810);
                c.lr = 0x800a5d50; f_800a4850(c);
                pc = 0x800a5d50;
            }
            0x800a5d50 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5cc8: bad pc {:#x}", pc),
        }
    }
}

/// stateGoToRetry (0x800a5d60, 40 bytes)
#[inline(never)]
pub fn f_800a5d60(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5d60) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5d60); p = true; }
    }
    f_800a5d60_b(c, 0x800a5d60);
    if p { crate::prof::exit(c, 0x800a5d60); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5d60_at(c: &mut Cpu, pc: u32) { f_800a5d60_b(c, pc) }
#[inline(always)]
fn f_800a5d60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5d60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5d60, pc); }
        match pc {
            0x800a5d60 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x800a0000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x5d88);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a5d78; f_800a4850(c);
                pc = 0x800a5d78;
            }
            0x800a5d78 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5d60: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateGoToRetry (0x800a5d88, 344 bytes)
#[inline(never)]
pub fn f_800a5d88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5d88) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5d88); p = true; }
    }
    f_800a5d88_b(c, 0x800a5d88);
    if p { crate::prof::exit(c, 0x800a5d88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5d88_at(c: &mut Cpu, pc: u32) { f_800a5d88_b(c, pc) }
#[inline(always)]
fn f_800a5d88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5d88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5d88, pc); }
        match pc {
            0x800a5d88 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                if !c.crbit(2) { pc = 0x800a5dc8; continue; }
                pc = 0x800a5da0;
            }
            0x800a5da0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                c.lr = 0x800a5db8; f_800a80fc(c);
                pc = 0x800a5db8;
            }
            0x800a5db8 => {
                c.lr = 0x800a5dbc; f_800a76e4(c);
                pc = 0x800a5dbc;
            }
            0x800a5dbc => {
                c.r[3] = 0x0;
                c.lr = 0x800a5dc4; f_800a5810(c);
                pc = 0x800a5dc4;
            }
            0x800a5dc4 => {
                pc = 0x800a5ecc; continue;
                pc = 0x800a5dc8;
            }
            0x800a5dc8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a5df8; continue; }
                pc = 0x800a5dd0;
            }
            0x800a5dd0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4567);
                c.lr = 0x800a5de8; f_800a80fc(c);
                pc = 0x800a5de8;
            }
            0x800a5de8 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5810);
                c.lr = 0x800a5df4; f_800a4850(c);
                pc = 0x800a5df4;
            }
            0x800a5df4 => {
                pc = 0x800a5ecc; continue;
                pc = 0x800a5df8;
            }
            0x800a5df8 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9ffc), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x800a5e30; continue; }
                pc = 0x800a5e0c;
            }
            0x800a5e0c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0x5);
                if c.crbit(2) { pc = 0x800a5e30; continue; }
                pc = 0x800a5e18;
            }
            0x800a5e18 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0xd);
                if c.crbit(2) { pc = 0x800a5e30; continue; }
                pc = 0x800a5e24;
            }
            0x800a5e24 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0xf);
                if !c.crbit(2) { pc = 0x800a5e38; continue; }
                pc = 0x800a5e30;
            }
            0x800a5e30 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa000), c.r[0]);
                pc = 0x800a5e38;
            }
            0x800a5e38 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a5eb0; continue; }
                pc = 0x800a5e44;
            }
            0x800a5e44 => {
                c.r[0] = 0x2;
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[3] = 0x80400000;
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0xffffc3a0);
                c.r[3] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                c.r[0] = 0xa;
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[3]);
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[31].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a5e88; continue; }
                pc = 0x800a5e78;
            }
            0x800a5e78 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0xfffffffd;
                if true { let t = c.lr; c.lr = 0x800a5e88; (c.dispatch)(c, t); }
                pc = 0x800a5e88;
            }
            0x800a5e88 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9fec));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a5ea4; continue; }
                pc = 0x800a5e94;
            }
            0x800a5e94 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a5ea4; (c.dispatch)(c, t); }
                pc = 0x800a5ea4;
            }
            0x800a5ea4 => {
                c.lr = 0x800a5ea8; f_800a6684(c);
                pc = 0x800a5ea8;
            }
            0x800a5ea8 => {
                c.r[0] = 0x1;
                pc = 0x800a5eb4; continue;
                pc = 0x800a5eb0;
            }
            0x800a5eb0 => {
                c.r[0] = 0x0;
                pc = 0x800a5eb4;
            }
            0x800a5eb4 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a5ecc; continue; }
                pc = 0x800a5ebc;
            }
            0x800a5ebc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xb;
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.lr = 0x800a5ecc; f_800a6578(c);
                pc = 0x800a5ecc;
            }
            0x800a5ecc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5d88: bad pc {:#x}", pc),
        }
    }
}

/// stateCheckID (0x800a5ee0, 224 bytes)
#[inline(never)]
pub fn f_800a5ee0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5ee0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5ee0); p = true; }
    }
    f_800a5ee0_b(c, 0x800a5ee0);
    if p { crate::prof::exit(c, 0x800a5ee0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5ee0_at(c: &mut Cpu, pc: u32) { f_800a5ee0_b(c, pc) }
#[inline(always)]
fn f_800a5ee0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5ee0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5ee0, pc); }
        match pc {
            0x800a5ee0 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffffc360);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x800a5f08; continue; }
                pc = 0x800a5f04;
            }
            0x800a5f04 => {
                pc = 0x800a5f70; continue;
                pc = 0x800a5f08;
            }
            0x800a5f08 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[3] = c.r[31].wrapping_add(0x20);
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x24));
                c.lr = 0x800a5f18; f_800a8178(c);
                pc = 0x800a5f18;
            }
            0x800a5f18 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a5f60; continue; }
                pc = 0x800a5f20;
            }
            0x800a5f20 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fcc));
                c.r[4] = c.r[31].wrapping_add(0x20);
                c.r[5] = 0x20;
                c.lr = 0x800a5f30; f_800054f4(c);
                pc = 0x800a5f30;
            }
            0x800a5f30 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0x1;
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[4] = 0x20;
                c.lr = 0x800a5f48; f_8009b2a4(c);
                pc = 0x800a5f48;
            }
            0x800a5f48 => {
                c.r[4] = 0x800a0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = c.r[4].wrapping_add(0x5ff4);
                c.stw(c.r[13].wrapping_add(0xffffa00c), c.r[0]);
                c.lr = 0x800a5f5c; f_800a5ff4(c);
                pc = 0x800a5f5c;
            }
            0x800a5f5c => {
                pc = 0x800a5fac; continue;
                pc = 0x800a5f60;
            }
            0x800a5f60 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x60d4);
                c.lr = 0x800a5f6c; f_800a4850(c);
                pc = 0x800a5f6c;
            }
            0x800a5f6c => {
                pc = 0x800a5fac; continue;
                pc = 0x800a5f70;
            }
            0x800a5f70 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fcc));
                c.r[3] = c.r[31].wrapping_add(0x20);
                c.r[5] = 0x20;
                c.lr = 0x800a5f80; f_800c80d0(c);
                pc = 0x800a5f80;
            }
            0x800a5f80 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a5f98; continue; }
                pc = 0x800a5f88;
            }
            0x800a5f88 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x60d4);
                c.lr = 0x800a5f94; f_800a4850(c);
                pc = 0x800a5f94;
            }
            0x800a5f94 => {
                pc = 0x800a5fac; continue;
                pc = 0x800a5f98;
            }
            0x800a5f98 => {
                c.r[4] = 0x800a0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = c.r[4].wrapping_add(0x5fc0);
                c.stw(c.r[13].wrapping_add(0xffffa00c), c.r[0]);
                c.lr = 0x800a5fac; f_800a5fc0(c);
                pc = 0x800a5fac;
            }
            0x800a5fac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5ee0: bad pc {:#x}", pc),
        }
    }
}

/// stateCheckID3 (0x800a5fc0, 52 bytes)
#[inline(never)]
pub fn f_800a5fc0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5fc0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5fc0); p = true; }
    }
    f_800a5fc0_b(c, 0x800a5fc0);
    if p { crate::prof::exit(c, 0x800a5fc0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5fc0_at(c: &mut Cpu, pc: u32) { f_800a5fc0_b(c, pc) }
#[inline(always)]
fn f_800a5fc0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5fc0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5fc0, pc); }
        match pc {
            0x800a5fc0 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x800a0000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[5] = c.r[3].wrapping_add(0x62cc);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fcc));
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x8));
                c.r[4] = 0xa;
                c.lr = 0x800a5fe4; f_800a4b28(c);
                pc = 0x800a5fe4;
            }
            0x800a5fe4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5fc0: bad pc {:#x}", pc),
        }
    }
}

/// stateCheckID2a (0x800a5ff4, 52 bytes)
#[inline(never)]
pub fn f_800a5ff4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a5ff4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a5ff4); p = true; }
    }
    f_800a5ff4_b(c, 0x800a5ff4);
    if p { crate::prof::exit(c, 0x800a5ff4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a5ff4_at(c: &mut Cpu, pc: u32) { f_800a5ff4_b(c, pc) }
#[inline(always)]
fn f_800a5ff4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a5ff4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a5ff4, pc); }
        match pc {
            0x800a5ff4 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x800a0000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[5] = c.r[3].wrapping_add(0x6028);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fcc));
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x8));
                c.r[4] = 0xa;
                c.lr = 0x800a6018; f_800a4b28(c);
                pc = 0x800a6018;
            }
            0x800a6018 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a5ff4: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateCheckID2a (0x800a6028, 116 bytes)
#[inline(never)]
pub fn f_800a6028(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a6028) { return; }
        if c.prof { crate::prof::enter(c, 0x800a6028); p = true; }
    }
    f_800a6028_b(c, 0x800a6028);
    if p { crate::prof::exit(c, 0x800a6028); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a6028_at(c: &mut Cpu, pc: u32) { f_800a6028_b(c, pc) }
#[inline(always)]
fn f_800a6028_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a6028;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a6028, pc); }
        match pc {
            0x800a6028 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                if !c.crbit(2) { pc = 0x800a6064; continue; }
                pc = 0x800a603c;
            }
            0x800a603c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                c.lr = 0x800a6054; f_800a80fc(c);
                pc = 0x800a6054;
            }
            0x800a6054 => {
                c.lr = 0x800a6058; f_800a76e4(c);
                pc = 0x800a6058;
            }
            0x800a6058 => {
                c.r[3] = 0x0;
                c.lr = 0x800a6060; f_800a5810(c);
                pc = 0x800a6060;
            }
            0x800a6060 => {
                pc = 0x800a608c; continue;
                pc = 0x800a6064;
            }
            0x800a6064 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a6080; continue; }
                pc = 0x800a606c;
            }
            0x800a606c => {
                c.r[0] = 0x0;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.stw(c.r[13].wrapping_add(0xffff9ffc), c.r[0]);
                c.lr = 0x800a607c; f_800a609c(c);
                pc = 0x800a607c;
            }
            0x800a607c => {
                pc = 0x800a608c; continue;
                pc = 0x800a6080;
            }
            0x800a6080 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x59cc);
                c.lr = 0x800a608c; f_800a48dc(c);
                pc = 0x800a608c;
            }
            0x800a608c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a6028: bad pc {:#x}", pc),
        }
    }
}

/// stateCheckID2 (0x800a609c, 56 bytes)
#[inline(never)]
pub fn f_800a609c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a609c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a609c); p = true; }
    }
    f_800a609c_b(c, 0x800a609c);
    if p { crate::prof::exit(c, 0x800a609c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a609c_at(c: &mut Cpu, pc: u32) { f_800a609c_b(c, pc) }
#[inline(always)]
fn f_800a609c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a609c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a609c, pc); }
        match pc {
            0x800a609c => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[4] = 0x800a0000;
                c.r[6] = c.r[4].wrapping_add(0x61e8);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[3] = c.r[3].wrapping_add(0xffffc360);
                c.r[4] = 0x20;
                c.r[5] = 0x420;
                c.lr = 0x800a60c4; f_800a4454(c);
                pc = 0x800a60c4;
            }
            0x800a60c4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a609c: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateCheckID1 (0x800a60d4, 276 bytes)
#[inline(never)]
pub fn f_800a60d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a60d4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a60d4); p = true; }
    }
    f_800a60d4_b(c, 0x800a60d4);
    if p { crate::prof::exit(c, 0x800a60d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a60d4_at(c: &mut Cpu, pc: u32) { f_800a60d4_b(c, pc) }
#[inline(always)]
fn f_800a60d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a60d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a60d4, pc); }
        match pc {
            0x800a60d4 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                if !c.crbit(2) { pc = 0x800a6114; continue; }
                pc = 0x800a60ec;
            }
            0x800a60ec => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                c.lr = 0x800a6104; f_800a80fc(c);
                pc = 0x800a6104;
            }
            0x800a6104 => {
                c.lr = 0x800a6108; f_800a76e4(c);
                pc = 0x800a6108;
            }
            0x800a6108 => {
                c.r[3] = 0x0;
                c.lr = 0x800a6110; f_800a5810(c);
                pc = 0x800a6110;
            }
            0x800a6110 => {
                pc = 0x800a61d4; continue;
                pc = 0x800a6114;
            }
            0x800a6114 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a6144; continue; }
                pc = 0x800a611c;
            }
            0x800a611c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4567);
                c.lr = 0x800a6134; f_800a80fc(c);
                pc = 0x800a6134;
            }
            0x800a6134 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5810);
                c.lr = 0x800a6140; f_800a4850(c);
                pc = 0x800a6140;
            }
            0x800a6140 => {
                pc = 0x800a61d4; continue;
                pc = 0x800a6144;
            }
            0x800a6144 => {
                c.r[4] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9ffc), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a61bc; continue; }
                pc = 0x800a6158;
            }
            0x800a6158 => {
                c.r[0] = 0x1;
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[0]);
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffc3a0);
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[4]);
                c.r[0] = 0xa;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[3]);
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[31].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6198; continue; }
                pc = 0x800a6188;
            }
            0x800a6188 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0xfffffffd;
                if true { let t = c.lr; c.lr = 0x800a6198; (c.dispatch)(c, t); }
                pc = 0x800a6198;
            }
            0x800a6198 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9fec));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a61b4; continue; }
                pc = 0x800a61a4;
            }
            0x800a61a4 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a61b4; (c.dispatch)(c, t); }
                pc = 0x800a61b4;
            }
            0x800a61b4 => {
                c.lr = 0x800a61b8; f_800a6684(c);
                pc = 0x800a61b8;
            }
            0x800a61b8 => {
                c.r[4] = 0x1;
                pc = 0x800a61bc;
            }
            0x800a61bc => {
                c.cmps(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a61d4; continue; }
                pc = 0x800a61c4;
            }
            0x800a61c4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0x6;
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.lr = 0x800a61d4; f_800a6578(c);
                pc = 0x800a61d4;
            }
            0x800a61d4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a60d4: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateCheckID2 (0x800a61e8, 228 bytes)
#[inline(never)]
pub fn f_800a61e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a61e8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a61e8); p = true; }
    }
    f_800a61e8_b(c, 0x800a61e8);
    if p { crate::prof::exit(c, 0x800a61e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a61e8_at(c: &mut Cpu, pc: u32) { f_800a61e8_b(c, pc) }
#[inline(always)]
fn f_800a61e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a61e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a61e8, pc); }
        match pc {
            0x800a61e8 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                if !c.crbit(2) { pc = 0x800a6228; continue; }
                pc = 0x800a6200;
            }
            0x800a6200 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                c.lr = 0x800a6218; f_800a80fc(c);
                pc = 0x800a6218;
            }
            0x800a6218 => {
                c.lr = 0x800a621c; f_800a76e4(c);
                pc = 0x800a621c;
            }
            0x800a621c => {
                c.r[3] = 0x0;
                c.lr = 0x800a6224; f_800a5810(c);
                pc = 0x800a6224;
            }
            0x800a6224 => {
                pc = 0x800a62b8; continue;
                pc = 0x800a6228;
            }
            0x800a6228 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a62ac; continue; }
                pc = 0x800a6230;
            }
            0x800a6230 => {
                c.r[3] = 0x800a0000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fd0));
                c.r[0] = c.r[3].wrapping_add(0x56f0);
                c.r[5] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa00c), c.r[0]);
                c.r[3] = 0x80400000;
                c.stw(c.r[13].wrapping_add(0xffff9ffc), c.r[5]);
                c.r[3] = c.r[3].wrapping_add(0xffffc360);
                c.r[31] = c.r[3].wrapping_add(0x8);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x3c));
                c.r[0] = c.lwz(c.r[31]);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x800a627c; continue; }
                pc = 0x800a6264;
            }
            0x800a6264 => {
                c.r[3] = 0x80310000;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[5] = c.r[3].wrapping_add(0x1ad4);
                c.r[3] = c.r[13].wrapping_add(0xffff81b4);
                c.r[4] = 0x287;
                c.lr = 0x800a627c; f_800060f0(c);
                pc = 0x800a627c;
            }
            0x800a627c => {
                c.r[3] = 0x80400000;
                c.r[6] = c.lwz(c.r[31]);
                c.r[5] = c.r[3].wrapping_add(0xffffc360);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff9fd0));
                c.r[4] = 0x800a0000;
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[6].wrapping_add(0x1f);
                c.r[3] = c.lwz(c.r[7].wrapping_add(0x38));
                c.r[6] = c.r[4].wrapping_add(0x5784);
                c.r[4] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.lr = 0x800a62a8; f_800a4454(c);
                pc = 0x800a62a8;
            }
            0x800a62a8 => {
                pc = 0x800a62b8; continue;
                pc = 0x800a62ac;
            }
            0x800a62ac => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x59cc);
                c.lr = 0x800a62b8; f_800a48dc(c);
                pc = 0x800a62b8;
            }
            0x800a62b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a61e8: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateCheckID3 (0x800a62cc, 252 bytes)
#[inline(never)]
pub fn f_800a62cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a62cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a62cc); p = true; }
    }
    f_800a62cc_b(c, 0x800a62cc);
    if p { crate::prof::exit(c, 0x800a62cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a62cc_at(c: &mut Cpu, pc: u32) { f_800a62cc_b(c, pc) }
#[inline(always)]
fn f_800a62cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a62cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a62cc, pc); }
        match pc {
            0x800a62cc => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                if !c.crbit(2) { pc = 0x800a630c; continue; }
                pc = 0x800a62e4;
            }
            0x800a62e4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                c.lr = 0x800a62fc; f_800a80fc(c);
                pc = 0x800a62fc;
            }
            0x800a62fc => {
                c.lr = 0x800a6300; f_800a76e4(c);
                pc = 0x800a6300;
            }
            0x800a6300 => {
                c.r[3] = 0x0;
                c.lr = 0x800a6308; f_800a5810(c);
                pc = 0x800a6308;
            }
            0x800a6308 => {
                pc = 0x800a63b4; continue;
                pc = 0x800a630c;
            }
            0x800a630c => {
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a63a8; continue; }
                pc = 0x800a6314;
            }
            0x800a6314 => {
                c.r[4] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9ffc), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a6388; continue; }
                pc = 0x800a6328;
            }
            0x800a6328 => {
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[4]);
                c.r[3] = 0x80400000;
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[3] = c.r[3].wrapping_add(0xffffc3a0);
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[4]);
                c.r[0] = 0xa;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[3]);
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[31].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6364; continue; }
                pc = 0x800a6354;
            }
            0x800a6354 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0xfffffffd;
                if true { let t = c.lr; c.lr = 0x800a6364; (c.dispatch)(c, t); }
                pc = 0x800a6364;
            }
            0x800a6364 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9fec));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6380; continue; }
                pc = 0x800a6370;
            }
            0x800a6370 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a6380; (c.dispatch)(c, t); }
                pc = 0x800a6380;
            }
            0x800a6380 => {
                c.lr = 0x800a6384; f_800a6684(c);
                pc = 0x800a6384;
            }
            0x800a6384 => {
                c.r[4] = 0x1;
                pc = 0x800a6388;
            }
            0x800a6388 => {
                c.cmps(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a63b4; continue; }
                pc = 0x800a6390;
            }
            0x800a6390 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.lr = 0x800a63a4; f_800a68b4(c);
                pc = 0x800a63a4;
            }
            0x800a63a4 => {
                pc = 0x800a63b4; continue;
                pc = 0x800a63a8;
            }
            0x800a63a8 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x59cc);
                c.lr = 0x800a63b4; f_800a48dc(c);
                pc = 0x800a63b4;
            }
            0x800a63b4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a62cc: bad pc {:#x}", pc),
        }
    }
}

/// AlarmHandler (0x800a63c8, 68 bytes)
#[inline(never)]
pub fn f_800a63c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a63c8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a63c8); p = true; }
    }
    f_800a63c8_b(c, 0x800a63c8);
    if p { crate::prof::exit(c, 0x800a63c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a63c8_at(c: &mut Cpu, pc: u32) { f_800a63c8_b(c, pc) }
#[inline(always)]
fn f_800a63c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a63c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a63c8, pc); }
        match pc {
            0x800a63c8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a63d8; f_800a76e4(c);
                pc = 0x800a63d8;
            }
            0x800a63d8 => {
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffc380);
                c.r[4] = 0x20;
                c.lr = 0x800a63e8; f_8009b2a4(c);
                pc = 0x800a63e8;
            }
            0x800a63e8 => {
                c.r[4] = 0x800a0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = c.r[4].wrapping_add(0x64d8);
                c.stw(c.r[13].wrapping_add(0xffffa00c), c.r[0]);
                c.lr = 0x800a63fc; f_800a64d8(c);
                pc = 0x800a63fc;
            }
            0x800a63fc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a63c8: bad pc {:#x}", pc),
        }
    }
}

/// stateCoverClosed (0x800a640c, 204 bytes)
#[inline(never)]
pub fn f_800a640c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a640c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a640c); p = true; }
    }
    f_800a640c_b(c, 0x800a640c);
    if p { crate::prof::exit(c, 0x800a640c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a640c_at(c: &mut Cpu, pc: u32) { f_800a640c_b(c, pc) }
#[inline(always)]
fn f_800a640c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a640c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a640c, pc); }
        match pc {
            0x800a640c => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffffc360);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmps(0, c.r[0], 0xd);
                if c.crbit(2) { pc = 0x800a6454; continue; }
                pc = 0x800a6430;
            }
            0x800a6430 => {
                if !c.crbit(0) { pc = 0x800a6448; continue; }
                pc = 0x800a6434;
            }
            0x800a6434 => {
                c.cmps(0, c.r[0], 0x6);
                if !c.crbit(0) { pc = 0x800a6484; continue; }
                pc = 0x800a643c;
            }
            0x800a643c => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x800a6454; continue; }
                pc = 0x800a6444;
            }
            0x800a6444 => {
                pc = 0x800a6484; continue;
                pc = 0x800a6448;
            }
            0x800a6448 => {
                c.cmps(0, c.r[0], 0xf);
                if c.crbit(2) { pc = 0x800a6454; continue; }
                pc = 0x800a6450;
            }
            0x800a6450 => {
                pc = 0x800a6484; continue;
                pc = 0x800a6454;
            }
            0x800a6454 => {
                c.lr = 0x800a6458; f_800a7de8(c);
                pc = 0x800a6458;
            }
            0x800a6458 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = c.r[31].wrapping_add(0x40);
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a647c; continue; }
                pc = 0x800a6470;
            }
            0x800a6470 => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0xfffffffc;
                if true { let t = c.lr; c.lr = 0x800a647c; (c.dispatch)(c, t); }
                pc = 0x800a647c;
            }
            0x800a647c => {
                c.lr = 0x800a6480; f_800a6684(c);
                pc = 0x800a6480;
            }
            0x800a6480 => {
                pc = 0x800a64c4; continue;
                pc = 0x800a6484;
            }
            0x800a6484 => {
                c.lr = 0x800a6488; f_800a76e4(c);
                pc = 0x800a6488;
            }
            0x800a6488 => {
                c.r[3] = c.r[31].wrapping_add(0x70);
                c.lr = 0x800a6490; f_8009a2c8(c);
                pc = 0x800a6490;
            }
            0x800a6490 => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[4] = 0x10620000;
                c.r[3] = 0x800a0000;
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[4] = c.r[4].wrapping_add(0x4dd3);
                c.r[0] = mulhwu(c.r[4], c.r[0]);
                c.r[0] = c.r[0].rotate_left(26) & 0x3ffffff;
                c.r[6] = c.r[0].wrapping_mul(0x47e);
                c.r[7] = c.r[3].wrapping_add(0x63c8);
                c.r[3] = c.r[31].wrapping_add(0x70);
                c.r[5] = 0x0;
                c.lr = 0x800a64c4; f_8009a528(c);
                pc = 0x800a64c4;
            }
            0x800a64c4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a640c: bad pc {:#x}", pc),
        }
    }
}

/// stateCoverClosed_CMD (0x800a64d8, 48 bytes)
#[inline(never)]
pub fn f_800a64d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a64d8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a64d8); p = true; }
    }
    f_800a64d8_b(c, 0x800a64d8);
    if p { crate::prof::exit(c, 0x800a64d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a64d8_at(c: &mut Cpu, pc: u32) { f_800a64d8_b(c, pc) }
#[inline(always)]
fn f_800a64d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a64d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a64d8, pc); }
        match pc {
            0x800a64d8 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[4] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0xffffc380);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[4] = c.r[4].wrapping_add(0x6508);
                c.lr = 0x800a64f8; f_800a47ac(c);
                pc = 0x800a64f8;
            }
            0x800a64f8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a64d8: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateCoverClosed (0x800a6508, 112 bytes)
#[inline(never)]
pub fn f_800a6508(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a6508) { return; }
        if c.prof { crate::prof::enter(c, 0x800a6508); p = true; }
    }
    f_800a6508_b(c, 0x800a6508);
    if p { crate::prof::exit(c, 0x800a6508); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a6508_at(c: &mut Cpu, pc: u32) { f_800a6508_b(c, pc) }
#[inline(always)]
fn f_800a6508_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a6508;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a6508, pc); }
        match pc {
            0x800a6508 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                if !c.crbit(2) { pc = 0x800a6544; continue; }
                pc = 0x800a651c;
            }
            0x800a651c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                c.lr = 0x800a6534; f_800a80fc(c);
                pc = 0x800a6534;
            }
            0x800a6534 => {
                c.lr = 0x800a6538; f_800a76e4(c);
                pc = 0x800a6538;
            }
            0x800a6538 => {
                c.r[3] = 0x0;
                c.lr = 0x800a6540; f_800a5810(c);
                pc = 0x800a6540;
            }
            0x800a6540 => {
                pc = 0x800a6568; continue;
                pc = 0x800a6544;
            }
            0x800a6544 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a655c; continue; }
                pc = 0x800a654c;
            }
            0x800a654c => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9ffc), c.r[0]);
                c.lr = 0x800a6558; f_800a5ee0(c);
                pc = 0x800a6558;
            }
            0x800a6558 => {
                pc = 0x800a6568; continue;
                pc = 0x800a655c;
            }
            0x800a655c => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x59cc);
                c.lr = 0x800a6568; f_800a48dc(c);
                pc = 0x800a6568;
            }
            0x800a6568 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a6508: bad pc {:#x}", pc),
        }
    }
}

/// stateMotorStopped (0x800a6578, 40 bytes)
#[inline(never)]
pub fn f_800a6578(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a6578) { return; }
        if c.prof { crate::prof::enter(c, 0x800a6578); p = true; }
    }
    f_800a6578_b(c, 0x800a6578);
    if p { crate::prof::exit(c, 0x800a6578); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a6578_at(c: &mut Cpu, pc: u32) { f_800a6578_b(c, pc) }
#[inline(always)]
fn f_800a6578_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a6578;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a6578, pc); }
        match pc {
            0x800a6578 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x800a0000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x65a0);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a6590; f_800a4780(c);
                pc = 0x800a6590;
            }
            0x800a6590 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a6578: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateMotorStopped (0x800a65a0, 228 bytes)
#[inline(never)]
pub fn f_800a65a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a65a0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a65a0); p = true; }
    }
    f_800a65a0_b(c, 0x800a65a0);
    if p { crate::prof::exit(c, 0x800a65a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a65a0_at(c: &mut Cpu, pc: u32) { f_800a65a0_b(c, pc) }
#[inline(always)]
fn f_800a65a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a65a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a65a0, pc); }
        match pc {
            0x800a65a0 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0xcc000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = 0x80400000;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffc360);
                c.stw(c.r[3].wrapping_add(0x6004), c.r[0]);
                c.r[0] = 0x3;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmps(0, c.r[0], 0xd);
                if c.crbit(2) { pc = 0x800a6600; continue; }
                pc = 0x800a65dc;
            }
            0x800a65dc => {
                if !c.crbit(0) { pc = 0x800a65f4; continue; }
                pc = 0x800a65e0;
            }
            0x800a65e0 => {
                c.cmps(0, c.r[0], 0x6);
                if !c.crbit(0) { pc = 0x800a6630; continue; }
                pc = 0x800a65e8;
            }
            0x800a65e8 => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x800a6600; continue; }
                pc = 0x800a65f0;
            }
            0x800a65f0 => {
                pc = 0x800a6630; continue;
                pc = 0x800a65f4;
            }
            0x800a65f4 => {
                c.cmps(0, c.r[0], 0xf);
                if c.crbit(2) { pc = 0x800a6600; continue; }
                pc = 0x800a65fc;
            }
            0x800a65fc => {
                pc = 0x800a6630; continue;
                pc = 0x800a6600;
            }
            0x800a6600 => {
                c.lr = 0x800a6604; f_800a7de8(c);
                pc = 0x800a6604;
            }
            0x800a6604 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = c.r[31].wrapping_add(0x40);
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6628; continue; }
                pc = 0x800a661c;
            }
            0x800a661c => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0xfffffffc;
                if true { let t = c.lr; c.lr = 0x800a6628; (c.dispatch)(c, t); }
                pc = 0x800a6628;
            }
            0x800a6628 => {
                c.lr = 0x800a662c; f_800a6684(c);
                pc = 0x800a662c;
            }
            0x800a662c => {
                pc = 0x800a6670; continue;
                pc = 0x800a6630;
            }
            0x800a6630 => {
                c.lr = 0x800a6634; f_800a76e4(c);
                pc = 0x800a6634;
            }
            0x800a6634 => {
                c.r[3] = c.r[31].wrapping_add(0x70);
                c.lr = 0x800a663c; f_8009a2c8(c);
                pc = 0x800a663c;
            }
            0x800a663c => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[4] = 0x10620000;
                c.r[3] = 0x800a0000;
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[4] = c.r[4].wrapping_add(0x4dd3);
                c.r[0] = mulhwu(c.r[4], c.r[0]);
                c.r[0] = c.r[0].rotate_left(26) & 0x3ffffff;
                c.r[6] = c.r[0].wrapping_mul(0x47e);
                c.r[7] = c.r[3].wrapping_add(0x63c8);
                c.r[3] = c.r[31].wrapping_add(0x70);
                c.r[5] = 0x0;
                c.lr = 0x800a6670; f_8009a528(c);
                pc = 0x800a6670;
            }
            0x800a6670 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a65a0: bad pc {:#x}", pc),
        }
    }
}

/// stateReady (0x800a6684, 560 bytes)
#[inline(never)]
pub fn f_800a6684(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a6684) { return; }
        if c.prof { crate::prof::enter(c, 0x800a6684); p = true; }
    }
    f_800a6684_b(c, 0x800a6684);
    if p { crate::prof::exit(c, 0x800a6684); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a6684_at(c: &mut Cpu, pc: u32) { f_800a6684_b(c, pc) }
#[inline(never)]
fn f_800a6684_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a6684;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a6684, pc); }
        match pc {
            0x800a6684 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffffc360);
                c.lr = 0x800a66a0; f_800a7f28(c);
                pc = 0x800a66a0;
            }
            0x800a66a0 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a66b4; continue; }
                pc = 0x800a66a8;
            }
            0x800a66a8 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                pc = 0x800a68a0; continue;
                pc = 0x800a66b4;
            }
            0x800a66b4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd4));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a66d4; continue; }
                pc = 0x800a66c0;
            }
            0x800a66c0 => {
                c.r[3] = 0x1;
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fd8), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                pc = 0x800a68a0; continue;
                pc = 0x800a66d4;
            }
            0x800a66d4 => {
                c.lr = 0x800a66d8; f_800a7e88(c);
                pc = 0x800a66d8;
            }
            0x800a66d8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe0));
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[3]);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a6720; continue; }
                pc = 0x800a66e8;
            }
            0x800a66e8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[4] = 0xffffffff;
                c.r[0] = c.r[31].wrapping_add(0x40);
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6718; continue; }
                pc = 0x800a670c;
            }
            0x800a670c => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0xffffffff;
                if true { let t = c.lr; c.lr = 0x800a6718; (c.dispatch)(c, t); }
                pc = 0x800a6718;
            }
            0x800a6718 => {
                c.lr = 0x800a671c; f_800a6684(c);
                pc = 0x800a671c;
            }
            0x800a671c => {
                pc = 0x800a68a0; continue;
                pc = 0x800a6720;
            }
            0x800a6720 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.stw(c.r[13].wrapping_add(0xffff9fe4), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9ff0));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a6890; continue; }
                pc = 0x800a6738;
            }
            0x800a6738 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9ff0));
                c.cmps(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x800a67a4; continue; }
                pc = 0x800a6744;
            }
            0x800a6744 => {
                if !c.crbit(0) { pc = 0x800a6760; continue; }
                pc = 0x800a6748;
            }
            0x800a6748 => {
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x800a6774; continue; }
                pc = 0x800a6750;
            }
            0x800a6750 => {
                if !c.crbit(0) { pc = 0x800a678c; continue; }
                pc = 0x800a6754;
            }
            0x800a6754 => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(0) { pc = 0x800a67bc; continue; }
                pc = 0x800a675c;
            }
            0x800a675c => {
                pc = 0x800a6884; continue;
                pc = 0x800a6760;
            }
            0x800a6760 => {
                c.cmps(0, c.r[0], 0x8);
                if !c.crbit(0) { pc = 0x800a6884; continue; }
                pc = 0x800a6768;
            }
            0x800a6768 => {
                c.cmps(0, c.r[0], 0x6);
                if !c.crbit(0) { pc = 0x800a67bc; continue; }
                pc = 0x800a6770;
            }
            0x800a6770 => {
                pc = 0x800a6868; continue;
                pc = 0x800a6774;
            }
            0x800a6774 => {
                c.r[0] = 0xb;
                c.r[3] = 0x800a0000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x65a0);
                c.lr = 0x800a6788; f_800a4780(c);
                pc = 0x800a6788;
            }
            0x800a6788 => {
                pc = 0x800a6884; continue;
                pc = 0x800a678c;
            }
            0x800a678c => {
                c.r[0] = 0x4;
                c.r[3] = 0x800a0000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x65a0);
                c.lr = 0x800a67a0; f_800a4780(c);
                pc = 0x800a67a0;
            }
            0x800a67a0 => {
                pc = 0x800a6884; continue;
                pc = 0x800a67a4;
            }
            0x800a67a4 => {
                c.r[0] = 0x5;
                c.r[3] = 0x800a0000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x65a0);
                c.lr = 0x800a67b8; f_800a4780(c);
                pc = 0x800a67b8;
            }
            0x800a67b8 => {
                pc = 0x800a6884; continue;
                pc = 0x800a67bc;
            }
            0x800a67bc => {
                c.r[0] = 0x3;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmps(0, c.r[0], 0xd);
                if c.crbit(2) { pc = 0x800a67f4; continue; }
                pc = 0x800a67d0;
            }
            0x800a67d0 => {
                if !c.crbit(0) { pc = 0x800a67e8; continue; }
                pc = 0x800a67d4;
            }
            0x800a67d4 => {
                c.cmps(0, c.r[0], 0x6);
                if !c.crbit(0) { pc = 0x800a6824; continue; }
                pc = 0x800a67dc;
            }
            0x800a67dc => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x800a67f4; continue; }
                pc = 0x800a67e4;
            }
            0x800a67e4 => {
                pc = 0x800a6824; continue;
                pc = 0x800a67e8;
            }
            0x800a67e8 => {
                c.cmps(0, c.r[0], 0xf);
                if c.crbit(2) { pc = 0x800a67f4; continue; }
                pc = 0x800a67f0;
            }
            0x800a67f0 => {
                pc = 0x800a6824; continue;
                pc = 0x800a67f4;
            }
            0x800a67f4 => {
                c.lr = 0x800a67f8; f_800a7de8(c);
                pc = 0x800a67f8;
            }
            0x800a67f8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = c.r[31].wrapping_add(0x40);
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a681c; continue; }
                pc = 0x800a6810;
            }
            0x800a6810 => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0xfffffffc;
                if true { let t = c.lr; c.lr = 0x800a681c; (c.dispatch)(c, t); }
                pc = 0x800a681c;
            }
            0x800a681c => {
                c.lr = 0x800a6820; f_800a6684(c);
                pc = 0x800a6820;
            }
            0x800a6820 => {
                pc = 0x800a6884; continue;
                pc = 0x800a6824;
            }
            0x800a6824 => {
                c.lr = 0x800a6828; f_800a76e4(c);
                pc = 0x800a6828;
            }
            0x800a6828 => {
                c.r[3] = c.r[31].wrapping_add(0x70);
                c.lr = 0x800a6830; f_8009a2c8(c);
                pc = 0x800a6830;
            }
            0x800a6830 => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[4] = 0x10620000;
                c.r[3] = 0x800a0000;
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[4] = c.r[4].wrapping_add(0x4dd3);
                c.r[0] = mulhwu(c.r[4], c.r[0]);
                c.r[0] = c.r[0].rotate_left(26) & 0x3ffffff;
                c.r[6] = c.r[0].wrapping_mul(0x47e);
                c.r[7] = c.r[3].wrapping_add(0x63c8);
                c.r[3] = c.r[31].wrapping_add(0x70);
                c.r[5] = 0x0;
                c.lr = 0x800a6864; f_8009a528(c);
                pc = 0x800a6864;
            }
            0x800a6864 => {
                pc = 0x800a6884; continue;
                pc = 0x800a6868;
            }
            0x800a6868 => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ff4));
                c.lr = 0x800a6878; f_800a80fc(c);
                pc = 0x800a6878;
            }
            0x800a6878 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5810);
                c.lr = 0x800a6884; f_800a4850(c);
                pc = 0x800a6884;
            }
            0x800a6884 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[0]);
                pc = 0x800a68a0; continue;
                pc = 0x800a6890;
            }
            0x800a6890 => {
                c.r[0] = 0x1;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.lr = 0x800a68a0; f_800a68b4(c);
                pc = 0x800a68a0;
            }
            0x800a68a0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a6684: bad pc {:#x}", pc),
        }
    }
}

/// stateBusy (0x800a68b4, 800 bytes)
#[inline(never)]
pub fn f_800a68b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a68b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a68b4); p = true; }
    }
    f_800a68b4_b(c, 0x800a68b4);
    if p { crate::prof::exit(c, 0x800a68b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a68b4_at(c: &mut Cpu, pc: u32) { f_800a68b4_b(c, pc) }
#[inline(never)]
fn f_800a68b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a68b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a68b4, pc); }
        match pc {
            0x800a68b4 => {
                c.r[0] = c.mfspr(8);
                pc = 0x800a68b8;
            }
            0x800a68b8 => {
                c.r[4] = 0x800a0000;
                pc = 0x800a68bc;
            }
            0x800a68bc => {
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                pc = 0x800a68c0;
            }
            0x800a68c0 => {
                c.r[0] = c.r[4].wrapping_add(0x68b4);
                pc = 0x800a68c4;
            }
            0x800a68c4 => {
                c.r[7] = c.r[3] | c.r[3];
                pc = 0x800a68c8;
            }
            0x800a68c8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800a68cc;
            }
            0x800a68cc => {
                c.stw(c.r[13].wrapping_add(0xffffa00c), c.r[0]);
                pc = 0x800a68d0;
            }
            0x800a68d0 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x800a68d4;
            }
            0x800a68d4 => {
                c.cmpu(0, c.r[0], 0xf);
                pc = 0x800a68d8;
            }
            0x800a68d8 => {
                if c.crbit(1) { pc = 0x800a6bac; continue; }
                pc = 0x800a68dc;
            }
            0x800a68dc => {
                c.r[3] = 0x80310000;
                pc = 0x800a68e0;
            }
            0x800a68e0 => {
                c.r[3] = c.r[3].wrapping_add(0x1b08);
                pc = 0x800a68e4;
            }
            0x800a68e4 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800a68e8;
            }
            0x800a68e8 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x800a68ec;
            }
            0x800a68ec => {
                c.mtspr(9, c.r[0]);
                pc = 0x800a68f0;
            }
            0x800a68f0 => {
                if true { let t = c.ctr; if (0x800a68b4..0x800a6bd4).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x800a68f4;
            }
            0x800a68f4 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a68f8;
            }
            0x800a68f8 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6004));
                pc = 0x800a68fc;
            }
            0x800a68fc => {
                c.r[5] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a6900;
            }
            0x800a6900 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6904;
            }
            0x800a6904 => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6908;
            }
            0x800a6908 => {
                c.r[0] = 0x20;
                pc = 0x800a690c;
            }
            0x800a690c => {
                c.r[4] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6910;
            }
            0x800a6910 => {
                c.stw(c.r[7].wrapping_add(0x1c), c.r[0]);
                pc = 0x800a6914;
            }
            0x800a6914 => {
                c.r[3] = c.lwz(c.r[7].wrapping_add(0x18));
                pc = 0x800a6918;
            }
            0x800a6918 => {
                c.lr = 0x800a691c; f_800a47ac(c);
                pc = 0x800a691c;
            }
            0x800a691c => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6920;
            }
            0x800a6920 => {
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x14));
                pc = 0x800a6924;
            }
            0x800a6924 => {
                c.cmpu(0, c.r[0], 0x0);
                pc = 0x800a6928;
            }
            0x800a6928 => {
                if !c.crbit(2) { pc = 0x800a6964; continue; }
                pc = 0x800a692c;
            }
            0x800a692c => {
                c.r[3] = 0x80400000;
                pc = 0x800a6930;
            }
            0x800a6930 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                pc = 0x800a6934;
            }
            0x800a6934 => {
                c.r[0] = c.r[3].wrapping_add(0xffffc3a0);
                pc = 0x800a6938;
            }
            0x800a6938 => {
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                pc = 0x800a693c;
            }
            0x800a693c => {
                c.r[0] = 0x0;
                pc = 0x800a6940;
            }
            0x800a6940 => {
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                pc = 0x800a6944;
            }
            0x800a6944 => {
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                pc = 0x800a6948;
            }
            0x800a6948 => {
                c.cmpu(0, c.r[12], 0x0);
                pc = 0x800a694c;
            }
            0x800a694c => {
                if c.crbit(2) { pc = 0x800a695c; continue; }
                pc = 0x800a6950;
            }
            0x800a6950 => {
                c.mtspr(8, c.r[12]);
                pc = 0x800a6954;
            }
            0x800a6954 => {
                c.r[3] = 0x0;
                pc = 0x800a6958;
            }
            0x800a6958 => {
                if true { let t = c.lr; c.lr = 0x800a695c; (c.dispatch)(c, t); }
                pc = 0x800a695c;
            }
            0x800a695c => {
                c.lr = 0x800a6960; f_800a6684(c);
                pc = 0x800a6960;
            }
            0x800a6960 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6964;
            }
            0x800a6964 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a6968;
            }
            0x800a6968 => {
                c.r[3] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a696c;
            }
            0x800a696c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x800a6970;
            }
            0x800a6970 => {
                c.r[4] = 0x80000;
                pc = 0x800a6974;
            }
            0x800a6974 => {
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6978;
            }
            0x800a6978 => {
                c.r[3] = c.lwz(c.r[7].wrapping_add(0x20));
                pc = 0x800a697c;
            }
            0x800a697c => {
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x14));
                pc = 0x800a6980;
            }
            0x800a6980 => {
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                pc = 0x800a6984;
            }
            0x800a6984 => {
                c.cmpu(0, c.r[0], c.r[4]);
                pc = 0x800a6988;
            }
            0x800a6988 => {
                if !c.crbit(1) { pc = 0x800a6990; continue; }
                pc = 0x800a698c;
            }
            0x800a698c => {
                pc = 0x800a6994; continue;
                pc = 0x800a6990;
            }
            0x800a6990 => {
                c.r[4] = c.r[0] | c.r[0];
                pc = 0x800a6994;
            }
            0x800a6994 => {
                c.stw(c.r[7].wrapping_add(0x1c), c.r[4]);
                pc = 0x800a6998;
            }
            0x800a6998 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a699c;
            }
            0x800a699c => {
                c.r[6] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a69a0;
            }
            0x800a69a0 => {
                c.r[5] = c.lwz(c.r[7].wrapping_add(0x20));
                pc = 0x800a69a4;
            }
            0x800a69a4 => {
                c.r[3] = c.lwz(c.r[7].wrapping_add(0x18));
                pc = 0x800a69a8;
            }
            0x800a69a8 => {
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x10));
                pc = 0x800a69ac;
            }
            0x800a69ac => {
                c.r[3] = c.r[3].wrapping_add(c.r[5]);
                pc = 0x800a69b0;
            }
            0x800a69b0 => {
                c.r[4] = c.lwz(c.r[7].wrapping_add(0x1c));
                pc = 0x800a69b4;
            }
            0x800a69b4 => {
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x800a69b8;
            }
            0x800a69b8 => {
                c.lr = 0x800a69bc; f_800a4454(c);
                pc = 0x800a69bc;
            }
            0x800a69bc => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a69c0;
            }
            0x800a69c0 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a69c4;
            }
            0x800a69c4 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6004));
                pc = 0x800a69c8;
            }
            0x800a69c8 => {
                c.r[5] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a69cc;
            }
            0x800a69cc => {
                c.r[3] = 0x800a0000;
                pc = 0x800a69d0;
            }
            0x800a69d0 => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                pc = 0x800a69d4;
            }
            0x800a69d4 => {
                c.r[4] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a69d8;
            }
            0x800a69d8 => {
                c.r[3] = c.lwz(c.r[7].wrapping_add(0x10));
                pc = 0x800a69dc;
            }
            0x800a69dc => {
                c.lr = 0x800a69e0; f_800a46ec(c);
                pc = 0x800a69e0;
            }
            0x800a69e0 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a69e4;
            }
            0x800a69e4 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a69e8;
            }
            0x800a69e8 => {
                c.r[3] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a69ec;
            }
            0x800a69ec => {
                c.lr = 0x800a69f0; f_800a4850(c);
                pc = 0x800a69f0;
            }
            0x800a69f0 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a69f4;
            }
            0x800a69f4 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a69f8;
            }
            0x800a69f8 => {
                c.r[3] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a69fc;
            }
            0x800a69fc => {
                c.lr = 0x800a6a00; f_800a4850(c);
                pc = 0x800a6a00;
            }
            0x800a6a00 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6a04;
            }
            0x800a6a04 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a6a08;
            }
            0x800a6a08 => {
                c.r[3] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a6a0c;
            }
            0x800a6a0c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x800a6a10;
            }
            0x800a6a10 => {
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6a14;
            }
            0x800a6a14 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fdc));
                pc = 0x800a6a18;
            }
            0x800a6a18 => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x800a6a1c;
            }
            0x800a6a1c => {
                if c.crbit(2) { pc = 0x800a6a40; continue; }
                pc = 0x800a6a20;
            }
            0x800a6a20 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                pc = 0x800a6a24;
            }
            0x800a6a24 => {
                c.r[0] = 0x0;
                pc = 0x800a6a28;
            }
            0x800a6a28 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6a2c;
            }
            0x800a6a2c => {
                c.stw(c.r[5].wrapping_add(0x1c), c.r[0]);
                pc = 0x800a6a30;
            }
            0x800a6a30 => {
                c.r[4] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6a34;
            }
            0x800a6a34 => {
                c.r[3] = 0x0;
                pc = 0x800a6a38;
            }
            0x800a6a38 => {
                c.lr = 0x800a6a3c; f_800a4a9c(c);
                pc = 0x800a6a3c;
            }
            0x800a6a3c => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6a40;
            }
            0x800a6a40 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                pc = 0x800a6a44;
            }
            0x800a6a44 => {
                c.r[0] = 0x1;
                pc = 0x800a6a48;
            }
            0x800a6a48 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6a4c;
            }
            0x800a6a4c => {
                c.stw(c.r[4].wrapping_add(0x1c), c.r[0]);
                pc = 0x800a6a50;
            }
            0x800a6a50 => {
                c.r[6] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6a54;
            }
            0x800a6a54 => {
                c.r[3] = 0x0;
                pc = 0x800a6a58;
            }
            0x800a6a58 => {
                c.r[4] = c.lwz(c.r[7].wrapping_add(0x14));
                pc = 0x800a6a5c;
            }
            0x800a6a5c => {
                c.r[5] = c.lwz(c.r[7].wrapping_add(0x10));
                pc = 0x800a6a60;
            }
            0x800a6a60 => {
                c.lr = 0x800a6a64; f_800a4a04(c);
                pc = 0x800a6a64;
            }
            0x800a6a64 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6a68;
            }
            0x800a6a68 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a6a6c;
            }
            0x800a6a6c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6004));
                pc = 0x800a6a70;
            }
            0x800a6a70 => {
                c.r[4] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a6a74;
            }
            0x800a6a74 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6a78;
            }
            0x800a6a78 => {
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6a7c;
            }
            0x800a6a7c => {
                c.r[6] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6a80;
            }
            0x800a6a80 => {
                c.r[3] = 0x10000;
                pc = 0x800a6a84;
            }
            0x800a6a84 => {
                c.r[4] = 0x0;
                pc = 0x800a6a88;
            }
            0x800a6a88 => {
                c.r[5] = 0x0;
                pc = 0x800a6a8c;
            }
            0x800a6a8c => {
                c.lr = 0x800a6a90; f_800a4a04(c);
                pc = 0x800a6a90;
            }
            0x800a6a90 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6a94;
            }
            0x800a6a94 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a6a98;
            }
            0x800a6a98 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6004));
                pc = 0x800a6a9c;
            }
            0x800a6a9c => {
                c.r[4] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a6aa0;
            }
            0x800a6aa0 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6aa4;
            }
            0x800a6aa4 => {
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6aa8;
            }
            0x800a6aa8 => {
                c.r[0] = 0x1;
                pc = 0x800a6aac;
            }
            0x800a6aac => {
                c.r[6] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6ab0;
            }
            0x800a6ab0 => {
                c.stw(c.r[13].wrapping_add(0xffff9fdc), c.r[0]);
                pc = 0x800a6ab4;
            }
            0x800a6ab4 => {
                c.r[3] = 0x0;
                pc = 0x800a6ab8;
            }
            0x800a6ab8 => {
                c.r[4] = 0x0;
                pc = 0x800a6abc;
            }
            0x800a6abc => {
                c.r[5] = 0x0;
                pc = 0x800a6ac0;
            }
            0x800a6ac0 => {
                c.lr = 0x800a6ac4; f_800a4a04(c);
                pc = 0x800a6ac4;
            }
            0x800a6ac4 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6ac8;
            }
            0x800a6ac8 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a6acc;
            }
            0x800a6acc => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6004));
                pc = 0x800a6ad0;
            }
            0x800a6ad0 => {
                c.r[5] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a6ad4;
            }
            0x800a6ad4 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6ad8;
            }
            0x800a6ad8 => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6adc;
            }
            0x800a6adc => {
                c.r[4] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6ae0;
            }
            0x800a6ae0 => {
                c.r[3] = 0x0;
                pc = 0x800a6ae4;
            }
            0x800a6ae4 => {
                c.lr = 0x800a6ae8; f_800a4a9c(c);
                pc = 0x800a6ae8;
            }
            0x800a6ae8 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6aec;
            }
            0x800a6aec => {
                c.r[3] = 0xcc000000;
                pc = 0x800a6af0;
            }
            0x800a6af0 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6004));
                pc = 0x800a6af4;
            }
            0x800a6af4 => {
                c.r[5] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a6af8;
            }
            0x800a6af8 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6afc;
            }
            0x800a6afc => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6b00;
            }
            0x800a6b00 => {
                c.r[4] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6b04;
            }
            0x800a6b04 => {
                c.r[3] = 0x10000;
                pc = 0x800a6b08;
            }
            0x800a6b08 => {
                c.lr = 0x800a6b0c; f_800a4a9c(c);
                pc = 0x800a6b0c;
            }
            0x800a6b0c => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6b10;
            }
            0x800a6b10 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a6b14;
            }
            0x800a6b14 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6004));
                pc = 0x800a6b18;
            }
            0x800a6b18 => {
                c.r[5] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a6b1c;
            }
            0x800a6b1c => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6b20;
            }
            0x800a6b20 => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6b24;
            }
            0x800a6b24 => {
                c.r[4] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6b28;
            }
            0x800a6b28 => {
                c.r[3] = 0x20000;
                pc = 0x800a6b2c;
            }
            0x800a6b2c => {
                c.lr = 0x800a6b30; f_800a4a9c(c);
                pc = 0x800a6b30;
            }
            0x800a6b30 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6b34;
            }
            0x800a6b34 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a6b38;
            }
            0x800a6b38 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6004));
                pc = 0x800a6b3c;
            }
            0x800a6b3c => {
                c.r[5] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a6b40;
            }
            0x800a6b40 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6b44;
            }
            0x800a6b44 => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6b48;
            }
            0x800a6b48 => {
                c.r[4] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6b4c;
            }
            0x800a6b4c => {
                c.r[3] = 0x30000;
                pc = 0x800a6b50;
            }
            0x800a6b50 => {
                c.lr = 0x800a6b54; f_800a4a9c(c);
                pc = 0x800a6b54;
            }
            0x800a6b54 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6b58;
            }
            0x800a6b58 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a6b5c;
            }
            0x800a6b5c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6004));
                pc = 0x800a6b60;
            }
            0x800a6b60 => {
                c.r[4] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a6b64;
            }
            0x800a6b64 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6b68;
            }
            0x800a6b68 => {
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6b6c;
            }
            0x800a6b6c => {
                c.r[5] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6b70;
            }
            0x800a6b70 => {
                c.r[3] = c.lwz(c.r[7].wrapping_add(0x10));
                pc = 0x800a6b74;
            }
            0x800a6b74 => {
                c.r[4] = c.lwz(c.r[7].wrapping_add(0x14));
                pc = 0x800a6b78;
            }
            0x800a6b78 => {
                c.lr = 0x800a6b7c; f_800a4b28(c);
                pc = 0x800a6b7c;
            }
            0x800a6b7c => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6b80;
            }
            0x800a6b80 => {
                c.r[3] = 0xcc000000;
                pc = 0x800a6b84;
            }
            0x800a6b84 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6004));
                pc = 0x800a6b88;
            }
            0x800a6b88 => {
                c.r[5] = c.r[3].wrapping_add(0x6000);
                pc = 0x800a6b8c;
            }
            0x800a6b8c => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6b90;
            }
            0x800a6b90 => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                pc = 0x800a6b94;
            }
            0x800a6b94 => {
                c.r[0] = 0x20;
                pc = 0x800a6b98;
            }
            0x800a6b98 => {
                c.r[4] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6b9c;
            }
            0x800a6b9c => {
                c.stw(c.r[7].wrapping_add(0x1c), c.r[0]);
                pc = 0x800a6ba0;
            }
            0x800a6ba0 => {
                c.r[3] = c.lwz(c.r[7].wrapping_add(0x18));
                pc = 0x800a6ba4;
            }
            0x800a6ba4 => {
                c.lr = 0x800a6ba8; f_800a4968(c);
                pc = 0x800a6ba8;
            }
            0x800a6ba8 => {
                pc = 0x800a6bc4; continue;
                pc = 0x800a6bac;
            }
            0x800a6bac => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff81b0));
                pc = 0x800a6bb0;
            }
            0x800a6bb0 => {
                c.r[3] = 0x800a0000;
                pc = 0x800a6bb4;
            }
            0x800a6bb4 => {
                c.r[4] = c.r[3].wrapping_add(0x6bd4);
                pc = 0x800a6bb8;
            }
            0x800a6bb8 => {
                c.mtspr(8, c.r[12]);
                pc = 0x800a6bbc;
            }
            0x800a6bbc => {
                c.r[3] = c.r[7].wrapping_add(0x0);
                pc = 0x800a6bc0;
            }
            0x800a6bc0 => {
                if true { let t = c.lr; c.lr = 0x800a6bc4; (c.dispatch)(c, t); }
                pc = 0x800a6bc4;
            }
            0x800a6bc4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x800a6bc8;
            }
            0x800a6bc8 => {
                c.r[1] = c.r[1].wrapping_add(0x8);
                pc = 0x800a6bcc;
            }
            0x800a6bcc => {
                c.mtspr(8, c.r[0]);
                pc = 0x800a6bd0;
            }
            0x800a6bd0 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800a68b4: bad pc {:#x}", pc),
        }
    }
}

/// cbForStateBusy (0x800a6bd4, 1592 bytes)
#[inline(never)]
pub fn f_800a6bd4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a6bd4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a6bd4); p = true; }
    }
    f_800a6bd4_b(c, 0x800a6bd4);
    if p { crate::prof::exit(c, 0x800a6bd4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a6bd4_at(c: &mut Cpu, pc: u32) { f_800a6bd4_b(c, pc) }
#[inline(never)]
fn f_800a6bd4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a6bd4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a6bd4, pc); }
        match pc {
            0x800a6bd4 => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x10);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[4] = 0x80400000;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffc360);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                if !c.crbit(2) { pc = 0x800a6c20; continue; }
                pc = 0x800a6bf8;
            }
            0x800a6bf8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4568);
                c.lr = 0x800a6c10; f_800a80fc(c);
                pc = 0x800a6c10;
            }
            0x800a6c10 => {
                c.lr = 0x800a6c14; f_800a76e4(c);
                pc = 0x800a6c14;
            }
            0x800a6c14 => {
                c.r[3] = 0x0;
                c.lr = 0x800a6c1c; f_800a5810(c);
                pc = 0x800a6c1c;
            }
            0x800a6c1c => {
                pc = 0x800a71f4; continue;
                pc = 0x800a6c20;
            }
            0x800a6c20 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x800a6c38; continue; }
                pc = 0x800a6c2c;
            }
            0x800a6c2c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0xf);
                if !c.crbit(2) { pc = 0x800a6d20; continue; }
                pc = 0x800a6c38;
            }
            0x800a6c38 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a6c68; continue; }
                pc = 0x800a6c40;
            }
            0x800a6c40 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4567);
                c.lr = 0x800a6c58; f_800a80fc(c);
                pc = 0x800a6c58;
            }
            0x800a6c58 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5810);
                c.lr = 0x800a6c64; f_800a4850(c);
                pc = 0x800a6c64;
            }
            0x800a6c64 => {
                pc = 0x800a71f4; continue;
                pc = 0x800a6c68;
            }
            0x800a6c68 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9ffc), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0xf);
                if !c.crbit(2) { pc = 0x800a6c84; continue; }
                pc = 0x800a6c7c;
            }
            0x800a6c7c => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa000), c.r[0]);
                pc = 0x800a6c84;
            }
            0x800a6c84 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a6cf8; continue; }
                pc = 0x800a6c90;
            }
            0x800a6c90 => {
                c.r[0] = 0x7;
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x40);
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[0]);
                c.r[0] = 0xa;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[3]);
                c.stw(c.r[30].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[30].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6cd0; continue; }
                pc = 0x800a6cc0;
            }
            0x800a6cc0 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0xfffffffd;
                if true { let t = c.lr; c.lr = 0x800a6cd0; (c.dispatch)(c, t); }
                pc = 0x800a6cd0;
            }
            0x800a6cd0 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9fec));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6cec; continue; }
                pc = 0x800a6cdc;
            }
            0x800a6cdc => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a6cec; (c.dispatch)(c, t); }
                pc = 0x800a6cec;
            }
            0x800a6cec => {
                c.lr = 0x800a6cf0; f_800a6684(c);
                pc = 0x800a6cf0;
            }
            0x800a6cf0 => {
                c.r[0] = 0x1;
                pc = 0x800a6cfc; continue;
                pc = 0x800a6cf8;
            }
            0x800a6cf8 => {
                c.r[0] = 0x0;
                pc = 0x800a6cfc;
            }
            0x800a6cfc => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a71f4; continue; }
                pc = 0x800a6d04;
            }
            0x800a6d04 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0x7;
                c.r[3] = 0x800a0000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x65a0);
                c.lr = 0x800a6d1c; f_800a4780(c);
                pc = 0x800a6d1c;
            }
            0x800a6d1c => {
                pc = 0x800a71f4; continue;
                pc = 0x800a6d20;
            }
            0x800a6d20 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[4], 0x1);
                if c.crbit(2) { pc = 0x800a6d40; continue; }
                pc = 0x800a6d2c;
            }
            0x800a6d2c => {
                c.r[0] = c.r[4].wrapping_add(0xfffffffc);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x800a6d40; continue; }
                pc = 0x800a6d38;
            }
            0x800a6d38 => {
                c.cmpu(0, c.r[4], 0xe);
                if !c.crbit(2) { pc = 0x800a6d48; continue; }
                pc = 0x800a6d40;
            }
            0x800a6d40 => {
                c.r[0] = 0x1;
                pc = 0x800a6d60; continue;
                pc = 0x800a6d48;
            }
            0x800a6d48 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81bc));
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(2) { pc = 0x800a6d5c; continue; }
                pc = 0x800a6d54;
            }
            0x800a6d54 => {
                c.r[0] = 0x1;
                pc = 0x800a6d60; continue;
                pc = 0x800a6d5c;
            }
            0x800a6d5c => {
                c.r[0] = 0x0;
                pc = 0x800a6d60;
            }
            0x800a6d60 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a6d8c; continue; }
                pc = 0x800a6d68;
            }
            0x800a6d68 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[4] = 0xcc000000;
                c.r[4] = c.r[4].wrapping_add(0x6000);
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x18));
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x1c));
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x20));
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.stw(c.r[6].wrapping_add(0x20), c.r[0]);
                pc = 0x800a6d8c;
            }
            0x800a6d8c => {
                c.r[0] = c.r[3].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a6df0; continue; }
                pc = 0x800a6d94;
            }
            0x800a6d94 => {
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[3] = c.r[31].wrapping_add(0x40);
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[0]);
                c.r[0] = 0xa;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[3]);
                c.stw(c.r[30].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[30].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6dcc; continue; }
                pc = 0x800a6dbc;
            }
            0x800a6dbc => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0xfffffffd;
                if true { let t = c.lr; c.lr = 0x800a6dcc; (c.dispatch)(c, t); }
                pc = 0x800a6dcc;
            }
            0x800a6dcc => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9fec));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6de8; continue; }
                pc = 0x800a6dd8;
            }
            0x800a6dd8 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a6de8; (c.dispatch)(c, t); }
                pc = 0x800a6de8;
            }
            0x800a6de8 => {
                c.lr = 0x800a6dec; f_800a6684(c);
                pc = 0x800a6dec;
            }
            0x800a6dec => {
                pc = 0x800a71f4; continue;
                pc = 0x800a6df0;
            }
            0x800a6df0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a70c4; continue; }
                pc = 0x800a6df8;
            }
            0x800a6df8 => {
                c.r[4] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9ffc), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a6e68; continue; }
                pc = 0x800a6e0c;
            }
            0x800a6e0c => {
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[4]);
                c.r[3] = c.r[31].wrapping_add(0x40);
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xa;
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[3]);
                c.stw(c.r[30].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[30].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6e44; continue; }
                pc = 0x800a6e34;
            }
            0x800a6e34 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0xfffffffd;
                if true { let t = c.lr; c.lr = 0x800a6e44; (c.dispatch)(c, t); }
                pc = 0x800a6e44;
            }
            0x800a6e44 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9fec));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6e60; continue; }
                pc = 0x800a6e50;
            }
            0x800a6e50 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a6e60; (c.dispatch)(c, t); }
                pc = 0x800a6e60;
            }
            0x800a6e60 => {
                c.lr = 0x800a6e64; f_800a6684(c);
                pc = 0x800a6e64;
            }
            0x800a6e64 => {
                c.r[4] = 0x1;
                pc = 0x800a6e68;
            }
            0x800a6e68 => {
                c.cmps(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a71f4; continue; }
                pc = 0x800a6e70;
            }
            0x800a6e70 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x800a6e90; continue; }
                pc = 0x800a6e7c;
            }
            0x800a6e7c => {
                c.r[0] = c.r[3].wrapping_add(0xfffffffc);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x800a6e90; continue; }
                pc = 0x800a6e88;
            }
            0x800a6e88 => {
                c.cmpu(0, c.r[3], 0xe);
                if !c.crbit(2) { pc = 0x800a6e98; continue; }
                pc = 0x800a6e90;
            }
            0x800a6e90 => {
                c.r[0] = 0x1;
                pc = 0x800a6eb0; continue;
                pc = 0x800a6e98;
            }
            0x800a6e98 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81bc));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a6eac; continue; }
                pc = 0x800a6ea4;
            }
            0x800a6ea4 => {
                c.r[0] = 0x1;
                pc = 0x800a6eb0; continue;
                pc = 0x800a6eac;
            }
            0x800a6eac => {
                c.r[0] = 0x0;
                pc = 0x800a6eb0;
            }
            0x800a6eb0 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a6f08; continue; }
                pc = 0x800a6eb8;
            }
            0x800a6eb8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x20));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x14));
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(2) { pc = 0x800a6ed4; continue; }
                pc = 0x800a6ecc;
            }
            0x800a6ecc => {
                c.lr = 0x800a6ed0; f_800a68b4(c);
                pc = 0x800a6ed0;
            }
            0x800a6ed0 => {
                pc = 0x800a71f4; continue;
                pc = 0x800a6ed4;
            }
            0x800a6ed4 => {
                c.r[0] = c.r[31].wrapping_add(0x40);
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[3].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6f00; continue; }
                pc = 0x800a6ef4;
            }
            0x800a6ef4 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x20));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a6f00; (c.dispatch)(c, t); }
                pc = 0x800a6f00;
            }
            0x800a6f00 => {
                c.lr = 0x800a6f04; f_800a6684(c);
                pc = 0x800a6f04;
            }
            0x800a6f04 => {
                pc = 0x800a71f4; continue;
                pc = 0x800a6f08;
            }
            0x800a6f08 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.r[0] = c.r[4].wrapping_add(0xfffffff7);
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(1) { pc = 0x800a6f20; continue; }
                pc = 0x800a6f18;
            }
            0x800a6f18 => {
                c.r[0] = 0x1;
                pc = 0x800a6f64; continue;
                pc = 0x800a6f20;
            }
            0x800a6f20 => {
                c.r[3] = 0x80310000;
                { let ea = c.r[3].wrapping_add(0x1b48); c.r[0] = c.lwz(ea); c.r[3] = ea; }
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(2) { pc = 0x800a6f38; continue; }
                pc = 0x800a6f30;
            }
            0x800a6f30 => {
                c.r[0] = 0x1;
                pc = 0x800a6f64; continue;
                pc = 0x800a6f38;
            }
            0x800a6f38 => {
                { let ea = c.r[3].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[3] = ea; }
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(2) { pc = 0x800a6f4c; continue; }
                pc = 0x800a6f44;
            }
            0x800a6f44 => {
                c.r[0] = 0x1;
                pc = 0x800a6f64; continue;
                pc = 0x800a6f4c;
            }
            0x800a6f4c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(2) { pc = 0x800a6f60; continue; }
                pc = 0x800a6f58;
            }
            0x800a6f58 => {
                c.r[0] = 0x1;
                pc = 0x800a6f64; continue;
                pc = 0x800a6f60;
            }
            0x800a6f60 => {
                c.r[0] = 0x0;
                pc = 0x800a6f64;
            }
            0x800a6f64 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a6fd0; continue; }
                pc = 0x800a6f6c;
            }
            0x800a6f6c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0xb);
                if c.crbit(2) { pc = 0x800a6f84; continue; }
                pc = 0x800a6f78;
            }
            0x800a6f78 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0xa);
                if !c.crbit(2) { pc = 0x800a6f94; continue; }
                pc = 0x800a6f84;
            }
            0x800a6f84 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6020));
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800a6fa0; continue;
                pc = 0x800a6f94;
            }
            0x800a6f94 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6000);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x20));
                pc = 0x800a6fa0;
            }
            0x800a6fa0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[5] = c.r[31].wrapping_add(0x40);
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[5]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a6fc8; continue; }
                pc = 0x800a6fc0;
            }
            0x800a6fc0 => {
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a6fc8; (c.dispatch)(c, t); }
                pc = 0x800a6fc8;
            }
            0x800a6fc8 => {
                c.lr = 0x800a6fcc; f_800a6684(c);
                pc = 0x800a6fcc;
            }
            0x800a6fcc => {
                pc = 0x800a71f4; continue;
                pc = 0x800a6fd0;
            }
            0x800a6fd0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0x6);
                if !c.crbit(2) { pc = 0x800a7090; continue; }
                pc = 0x800a6fdc;
            }
            0x800a6fdc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[5] = c.r[4].wrapping_add(0x1c);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a7060; continue; }
                pc = 0x800a6ff0;
            }
            0x800a6ff0 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6020));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a7030; continue; }
                pc = 0x800a7000;
            }
            0x800a7000 => {
                c.r[0] = c.r[31].wrapping_add(0x40);
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                c.r[0] = 0x9;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a7028; continue; }
                pc = 0x800a701c;
            }
            0x800a701c => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0xfffffffe;
                if true { let t = c.lr; c.lr = 0x800a7028; (c.dispatch)(c, t); }
                pc = 0x800a7028;
            }
            0x800a7028 => {
                c.lr = 0x800a702c; f_800a6684(c);
                pc = 0x800a702c;
            }
            0x800a702c => {
                pc = 0x800a71f4; continue;
                pc = 0x800a7030;
            }
            0x800a7030 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fdc), c.r[0]);
                c.r[0] = 0x1;
                c.r[3] = 0x800a0000;
                c.stw(c.r[5], c.r[0]);
                c.r[6] = c.r[3].wrapping_add(0x6bd4);
                c.r[3] = 0x0;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[4] = c.lwz(c.r[5].wrapping_add(0x14));
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x10));
                c.lr = 0x800a705c; f_800a4a04(c);
                pc = 0x800a705c;
            }
            0x800a705c => {
                pc = 0x800a71f4; continue;
                pc = 0x800a7060;
            }
            0x800a7060 => {
                c.r[0] = c.r[31].wrapping_add(0x40);
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a7088; continue; }
                pc = 0x800a707c;
            }
            0x800a707c => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a7088; (c.dispatch)(c, t); }
                pc = 0x800a7088;
            }
            0x800a7088 => {
                c.lr = 0x800a708c; f_800a6684(c);
                pc = 0x800a708c;
            }
            0x800a708c => {
                pc = 0x800a71f4; continue;
                pc = 0x800a7090;
            }
            0x800a7090 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[3] = c.r[31].wrapping_add(0x40);
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[3]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a70bc; continue; }
                pc = 0x800a70b0;
            }
            0x800a70b0 => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a70bc; (c.dispatch)(c, t); }
                pc = 0x800a70bc;
            }
            0x800a70bc => {
                c.lr = 0x800a70c0; f_800a6684(c);
                pc = 0x800a70c0;
            }
            0x800a70c0 => {
                pc = 0x800a71f4; continue;
                pc = 0x800a70c4;
            }
            0x800a70c4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0xe);
                if !c.crbit(2) { pc = 0x800a70f8; continue; }
                pc = 0x800a70d0;
            }
            0x800a70d0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[0] = 0xffffffff;
                c.r[3] = 0x1230000;
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x4567);
                c.lr = 0x800a70e8; f_800a80fc(c);
                pc = 0x800a70e8;
            }
            0x800a70e8 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x5810);
                c.lr = 0x800a70f4; f_800a4850(c);
                pc = 0x800a70f4;
            }
            0x800a70f4 => {
                pc = 0x800a71f4; continue;
                pc = 0x800a70f8;
            }
            0x800a70f8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800a7128; continue; }
                pc = 0x800a7104;
            }
            0x800a7104 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x800a7128; continue; }
                pc = 0x800a7110;
            }
            0x800a7110 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0x5);
                if c.crbit(2) { pc = 0x800a7128; continue; }
                pc = 0x800a711c;
            }
            0x800a711c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe4));
                c.cmpu(0, c.r[0], 0xe);
                if !c.crbit(2) { pc = 0x800a71e8; continue; }
                pc = 0x800a7128;
            }
            0x800a7128 => {
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x20));
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x14));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a71e8; continue; }
                pc = 0x800a713c;
            }
            0x800a713c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a71a8; continue; }
                pc = 0x800a7148;
            }
            0x800a7148 => {
                c.r[4] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[4]);
                c.r[3] = c.r[31].wrapping_add(0x40);
                c.r[0] = 0xa;
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[3]);
                c.stw(c.r[30].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[30].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a7180; continue; }
                pc = 0x800a7170;
            }
            0x800a7170 => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0xfffffffd;
                if true { let t = c.lr; c.lr = 0x800a7180; (c.dispatch)(c, t); }
                pc = 0x800a7180;
            }
            0x800a7180 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9fec));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a719c; continue; }
                pc = 0x800a718c;
            }
            0x800a718c => {
                c.mtspr(8, c.r[12]);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a719c; (c.dispatch)(c, t); }
                pc = 0x800a719c;
            }
            0x800a719c => {
                c.lr = 0x800a71a0; f_800a6684(c);
                pc = 0x800a71a0;
            }
            0x800a71a0 => {
                c.r[0] = 0x1;
                pc = 0x800a71ac; continue;
                pc = 0x800a71a8;
            }
            0x800a71a8 => {
                c.r[0] = 0x0;
                pc = 0x800a71ac;
            }
            0x800a71ac => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a71f4; continue; }
                pc = 0x800a71b4;
            }
            0x800a71b4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[3] = c.r[31].wrapping_add(0x40);
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[3]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x28));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a71e0; continue; }
                pc = 0x800a71d4;
            }
            0x800a71d4 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x20));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a71e0; (c.dispatch)(c, t); }
                pc = 0x800a71e0;
            }
            0x800a71e0 => {
                c.lr = 0x800a71e4; f_800a6684(c);
                pc = 0x800a71e4;
            }
            0x800a71e4 => {
                pc = 0x800a71f4; continue;
                pc = 0x800a71e8;
            }
            0x800a71e8 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0x59cc);
                c.lr = 0x800a71f4; f_800a48dc(c);
                pc = 0x800a71f4;
            }
            0x800a71f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a6bd4: bad pc {:#x}", pc),
        }
    }
}

/// DVDReadAbsAsyncPrio (0x800a720c, 220 bytes)
#[inline(never)]
pub fn f_800a720c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a720c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a720c); p = true; }
    }
    f_800a720c_b(c, 0x800a720c);
    if p { crate::prof::exit(c, 0x800a720c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a720c_at(c: &mut Cpu, pc: u32) { f_800a720c_b(c, pc) }
#[inline(always)]
fn f_800a720c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a720c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a720c, pc); }
        match pc {
            0x800a720c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x1;
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x34), c.r[31]);
                c.r[31] = c.r[8].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x30), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x2c), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x18), c.r[4]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[5]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[6]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x28), c.r[7]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ac));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a7284; continue; }
                pc = 0x800a7258;
            }
            0x800a7258 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x8));
                c.cmpu(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x800a7278; continue; }
                pc = 0x800a7264;
            }
            0x800a7264 => {
                c.r[0] = c.r[3].wrapping_add(0xfffffffc);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x800a7278; continue; }
                pc = 0x800a7270;
            }
            0x800a7270 => {
                c.cmpu(0, c.r[3], 0xe);
                if !c.crbit(2) { pc = 0x800a7284; continue; }
                pc = 0x800a7278;
            }
            0x800a7278 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x18));
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x14));
                c.lr = 0x800a7284; f_8009b2a4(c);
                pc = 0x800a7284;
            }
            0x800a7284 => {
                c.lr = 0x800a7288; f_8009df3c(c);
                pc = 0x800a7288;
            }
            0x800a7288 => {
                c.r[0] = 0x2;
                c.stw(c.r[29].wrapping_add(0xc), c.r[0]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.lr = 0x800a72a0; f_800a7e20(c);
                pc = 0x800a72a0;
            }
            0x800a72a0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a72c0; continue; }
                pc = 0x800a72b0;
            }
            0x800a72b0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd4));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a72c0; continue; }
                pc = 0x800a72bc;
            }
            0x800a72bc => {
                c.lr = 0x800a72c0; f_800a6684(c);
                pc = 0x800a72c0;
            }
            0x800a72c0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a72c8; f_8009df64(c);
                pc = 0x800a72c8;
            }
            0x800a72c8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x30));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a720c: bad pc {:#x}", pc),
        }
    }
}

/// DVDSeekAbsAsyncPrio (0x800a72e8, 204 bytes)
#[inline(never)]
pub fn f_800a72e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a72e8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a72e8); p = true; }
    }
    f_800a72e8_b(c, 0x800a72e8);
    if p { crate::prof::exit(c, 0x800a72e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a72e8_at(c: &mut Cpu, pc: u32) { f_800a72e8_b(c, pc) }
#[inline(always)]
fn f_800a72e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a72e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a72e8, pc); }
        match pc {
            0x800a72e8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x2;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.r[31] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[4]);
                c.stw(c.r[3].wrapping_add(0x28), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ac));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a7350; continue; }
                pc = 0x800a7324;
            }
            0x800a7324 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x8));
                c.cmpu(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x800a7344; continue; }
                pc = 0x800a7330;
            }
            0x800a7330 => {
                c.r[0] = c.r[3].wrapping_add(0xfffffffc);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x800a7344; continue; }
                pc = 0x800a733c;
            }
            0x800a733c => {
                c.cmpu(0, c.r[3], 0xe);
                if !c.crbit(2) { pc = 0x800a7350; continue; }
                pc = 0x800a7344;
            }
            0x800a7344 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x18));
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x14));
                c.lr = 0x800a7350; f_8009b2a4(c);
                pc = 0x800a7350;
            }
            0x800a7350 => {
                c.lr = 0x800a7354; f_8009df3c(c);
                pc = 0x800a7354;
            }
            0x800a7354 => {
                c.r[0] = 0x2;
                c.stw(c.r[29].wrapping_add(0xc), c.r[0]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.lr = 0x800a736c; f_800a7e20(c);
                pc = 0x800a736c;
            }
            0x800a736c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a738c; continue; }
                pc = 0x800a737c;
            }
            0x800a737c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd4));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a738c; continue; }
                pc = 0x800a7388;
            }
            0x800a7388 => {
                c.lr = 0x800a738c; f_800a6684(c);
                pc = 0x800a738c;
            }
            0x800a738c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a7394; f_8009df64(c);
                pc = 0x800a7394;
            }
            0x800a7394 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a72e8: bad pc {:#x}", pc),
        }
    }
}

/// DVDReadAbsAsyncForBS (0x800a73b4, 208 bytes)
#[inline(never)]
pub fn f_800a73b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a73b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a73b4); p = true; }
    }
    f_800a73b4_b(c, 0x800a73b4);
    if p { crate::prof::exit(c, 0x800a73b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a73b4_at(c: &mut Cpu, pc: u32) { f_800a73b4_b(c, pc) }
#[inline(always)]
fn f_800a73b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a73b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a73b4, pc); }
        match pc {
            0x800a73b4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x4;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x18), c.r[4]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[5]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[6]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x28), c.r[7]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ac));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a7424; continue; }
                pc = 0x800a73f8;
            }
            0x800a73f8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x800a7418; continue; }
                pc = 0x800a7404;
            }
            0x800a7404 => {
                c.r[0] = c.r[3].wrapping_add(0xfffffffc);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x800a7418; continue; }
                pc = 0x800a7410;
            }
            0x800a7410 => {
                c.cmpu(0, c.r[3], 0xe);
                if !c.crbit(2) { pc = 0x800a7424; continue; }
                pc = 0x800a7418;
            }
            0x800a7418 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x18));
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x14));
                c.lr = 0x800a7424; f_8009b2a4(c);
                pc = 0x800a7424;
            }
            0x800a7424 => {
                c.lr = 0x800a7428; f_8009df3c(c);
                pc = 0x800a7428;
            }
            0x800a7428 => {
                c.r[0] = 0x2;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0x2;
                c.lr = 0x800a7440; f_800a7e20(c);
                pc = 0x800a7440;
            }
            0x800a7440 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a7460; continue; }
                pc = 0x800a7450;
            }
            0x800a7450 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd4));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a7460; continue; }
                pc = 0x800a745c;
            }
            0x800a745c => {
                c.lr = 0x800a7460; f_800a6684(c);
                pc = 0x800a7460;
            }
            0x800a7460 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a7468; f_8009df64(c);
                pc = 0x800a7468;
            }
            0x800a7468 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a73b4: bad pc {:#x}", pc),
        }
    }
}

/// DVDReadDiskID (0x800a7484, 212 bytes)
#[inline(never)]
pub fn f_800a7484(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7484) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7484); p = true; }
    }
    f_800a7484_b(c, 0x800a7484);
    if p { crate::prof::exit(c, 0x800a7484); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7484_at(c: &mut Cpu, pc: u32) { f_800a7484_b(c, pc) }
#[inline(always)]
fn f_800a7484_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7484;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7484, pc); }
        match pc {
            0x800a7484 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x5;
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.r[3] = 0x20;
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x18), c.r[4]);
                c.stw(c.r[31].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[31].wrapping_add(0x10), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x20), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x28), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ac));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a74f8; continue; }
                pc = 0x800a74cc;
            }
            0x800a74cc => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x800a74ec; continue; }
                pc = 0x800a74d8;
            }
            0x800a74d8 => {
                c.r[0] = c.r[3].wrapping_add(0xfffffffc);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x800a74ec; continue; }
                pc = 0x800a74e4;
            }
            0x800a74e4 => {
                c.cmpu(0, c.r[3], 0xe);
                if !c.crbit(2) { pc = 0x800a74f8; continue; }
                pc = 0x800a74ec;
            }
            0x800a74ec => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x18));
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x14));
                c.lr = 0x800a74f8; f_8009b2a4(c);
                pc = 0x800a74f8;
            }
            0x800a74f8 => {
                c.lr = 0x800a74fc; f_8009df3c(c);
                pc = 0x800a74fc;
            }
            0x800a74fc => {
                c.r[0] = 0x2;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0x2;
                c.lr = 0x800a7514; f_800a7e20(c);
                pc = 0x800a7514;
            }
            0x800a7514 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a7534; continue; }
                pc = 0x800a7524;
            }
            0x800a7524 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd4));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a7534; continue; }
                pc = 0x800a7530;
            }
            0x800a7530 => {
                c.lr = 0x800a7534; f_800a6684(c);
                pc = 0x800a7534;
            }
            0x800a7534 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a753c; f_8009df64(c);
                pc = 0x800a753c;
            }
            0x800a753c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7484: bad pc {:#x}", pc),
        }
    }
}

/// DVDCancelStreamAsync (0x800a7558, 188 bytes)
#[inline(never)]
pub fn f_800a7558(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7558) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7558); p = true; }
    }
    f_800a7558_b(c, 0x800a7558);
    if p { crate::prof::exit(c, 0x800a7558); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7558_at(c: &mut Cpu, pc: u32) { f_800a7558_b(c, pc) }
#[inline(always)]
fn f_800a7558_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7558;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7558, pc); }
        match pc {
            0x800a7558 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x7;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x28), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ac));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a75b4; continue; }
                pc = 0x800a7588;
            }
            0x800a7588 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x800a75a8; continue; }
                pc = 0x800a7594;
            }
            0x800a7594 => {
                c.r[0] = c.r[3].wrapping_add(0xfffffffc);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x800a75a8; continue; }
                pc = 0x800a75a0;
            }
            0x800a75a0 => {
                c.cmpu(0, c.r[3], 0xe);
                if !c.crbit(2) { pc = 0x800a75b4; continue; }
                pc = 0x800a75a8;
            }
            0x800a75a8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x18));
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x14));
                c.lr = 0x800a75b4; f_8009b2a4(c);
                pc = 0x800a75b4;
            }
            0x800a75b4 => {
                c.lr = 0x800a75b8; f_8009df3c(c);
                pc = 0x800a75b8;
            }
            0x800a75b8 => {
                c.r[0] = 0x2;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0x1;
                c.lr = 0x800a75d0; f_800a7e20(c);
                pc = 0x800a75d0;
            }
            0x800a75d0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a75f0; continue; }
                pc = 0x800a75e0;
            }
            0x800a75e0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd4));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a75f0; continue; }
                pc = 0x800a75ec;
            }
            0x800a75ec => {
                c.lr = 0x800a75f0; f_800a6684(c);
                pc = 0x800a75f0;
            }
            0x800a75f0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a75f8; f_8009df64(c);
                pc = 0x800a75f8;
            }
            0x800a75f8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7558: bad pc {:#x}", pc),
        }
    }
}

/// DVDInquiryAsync (0x800a7614, 208 bytes)
#[inline(never)]
pub fn f_800a7614(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7614) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7614); p = true; }
    }
    f_800a7614_b(c, 0x800a7614);
    if p { crate::prof::exit(c, 0x800a7614); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7614_at(c: &mut Cpu, pc: u32) { f_800a7614_b(c, pc) }
#[inline(always)]
fn f_800a7614_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7614;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7614, pc); }
        match pc {
            0x800a7614 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0xe;
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.r[3] = 0x20;
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x18), c.r[4]);
                c.stw(c.r[31].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[31].wrapping_add(0x20), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x28), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ac));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a7684; continue; }
                pc = 0x800a7658;
            }
            0x800a7658 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x800a7678; continue; }
                pc = 0x800a7664;
            }
            0x800a7664 => {
                c.r[0] = c.r[3].wrapping_add(0xfffffffc);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x800a7678; continue; }
                pc = 0x800a7670;
            }
            0x800a7670 => {
                c.cmpu(0, c.r[3], 0xe);
                if !c.crbit(2) { pc = 0x800a7684; continue; }
                pc = 0x800a7678;
            }
            0x800a7678 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x18));
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x14));
                c.lr = 0x800a7684; f_8009b2a4(c);
                pc = 0x800a7684;
            }
            0x800a7684 => {
                c.lr = 0x800a7688; f_8009df3c(c);
                pc = 0x800a7688;
            }
            0x800a7688 => {
                c.r[0] = 0x2;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0x2;
                c.lr = 0x800a76a0; f_800a7e20(c);
                pc = 0x800a76a0;
            }
            0x800a76a0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a76c0; continue; }
                pc = 0x800a76b0;
            }
            0x800a76b0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd4));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a76c0; continue; }
                pc = 0x800a76bc;
            }
            0x800a76bc => {
                c.lr = 0x800a76c0; f_800a6684(c);
                pc = 0x800a76c0;
            }
            0x800a76c0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a76c8; f_8009df64(c);
                pc = 0x800a76c8;
            }
            0x800a76c8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7614: bad pc {:#x}", pc),
        }
    }
}

/// DVDReset (0x800a76e4, 68 bytes)
#[inline(never)]
pub fn f_800a76e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a76e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a76e4); p = true; }
    }
    f_800a76e4_b(c, 0x800a76e4);
    if p { crate::prof::exit(c, 0x800a76e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a76e4_at(c: &mut Cpu, pc: u32) { f_800a76e4_b(c, pc) }
#[inline(always)]
fn f_800a76e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a76e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a76e4, pc); }
        match pc {
            0x800a76e4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a76f4; f_800a4bc4(c);
                pc = 0x800a76f4;
            }
            0x800a76f4 => {
                c.r[3] = 0xcc000000;
                c.r[0] = 0x2a;
                c.stw(c.r[3].wrapping_add(0x6000), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x6000);
                c.r[0] = 0x0;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x6004));
                c.stw(c.r[4].wrapping_add(0x4), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa000), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a76e4: bad pc {:#x}", pc),
        }
    }
}

/// DVDGetCommandBlockStatus (0x800a7728, 76 bytes)
#[inline(never)]
pub fn f_800a7728(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7728) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7728); p = true; }
    }
    f_800a7728_b(c, 0x800a7728);
    if p { crate::prof::exit(c, 0x800a7728); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7728_at(c: &mut Cpu, pc: u32) { f_800a7728_b(c, pc) }
#[inline(always)]
fn f_800a7728_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7728;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7728, pc); }
        match pc {
            0x800a7728 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800a7740; f_8009df3c(c);
                pc = 0x800a7740;
            }
            0x800a7740 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.cmps(0, c.r[0], 0x3);
                if !c.crbit(2) { pc = 0x800a7754; continue; }
                pc = 0x800a774c;
            }
            0x800a774c => {
                c.r[31] = 0x1;
                pc = 0x800a7758; continue;
                pc = 0x800a7754;
            }
            0x800a7754 => {
                c.r[31] = c.r[0] | c.r[0];
                pc = 0x800a7758;
            }
            0x800a7758 => {
                c.lr = 0x800a775c; f_8009df64(c);
                pc = 0x800a775c;
            }
            0x800a775c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7728: bad pc {:#x}", pc),
        }
    }
}

/// DVDGetDriveStatus (0x800a7774, 172 bytes)
#[inline(never)]
pub fn f_800a7774(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7774) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7774); p = true; }
    }
    f_800a7774_b(c, 0x800a7774);
    if p { crate::prof::exit(c, 0x800a7774); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7774_at(c: &mut Cpu, pc: u32) { f_800a7774_b(c, pc) }
#[inline(always)]
fn f_800a7774_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7774;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7774, pc); }
        match pc {
            0x800a7774 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.lr = 0x800a778c; f_8009df3c(c);
                pc = 0x800a778c;
            }
            0x800a778c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe0));
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a77a4; continue; }
                pc = 0x800a779c;
            }
            0x800a779c => {
                c.r[31] = 0xffffffff;
                pc = 0x800a77fc; continue;
                pc = 0x800a77a4;
            }
            0x800a77a4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd8));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a77b8; continue; }
                pc = 0x800a77b0;
            }
            0x800a77b0 => {
                c.r[31] = 0x8;
                pc = 0x800a77fc; continue;
                pc = 0x800a77b8;
            }
            0x800a77b8 => {
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x800a77cc; continue; }
                pc = 0x800a77c4;
            }
            0x800a77c4 => {
                c.r[31] = 0x0;
                pc = 0x800a77fc; continue;
                pc = 0x800a77cc;
            }
            0x800a77cc => {
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffc3a0);
                c.cmpu(0, c.r[31], c.r[0]);
                if !c.crbit(2) { pc = 0x800a77e4; continue; }
                pc = 0x800a77dc;
            }
            0x800a77dc => {
                c.r[31] = 0x0;
                pc = 0x800a77fc; continue;
                pc = 0x800a77e4;
            }
            0x800a77e4 => {
                c.lr = 0x800a77e8; f_8009df3c(c);
                pc = 0x800a77e8;
            }
            0x800a77e8 => {
                c.r[31] = c.lwz(c.r[31].wrapping_add(0xc));
                c.cmps(0, c.r[31], 0x3);
                if !c.crbit(2) { pc = 0x800a77f8; continue; }
                pc = 0x800a77f4;
            }
            0x800a77f4 => {
                c.r[31] = 0x1;
                pc = 0x800a77f8;
            }
            0x800a77f8 => {
                c.lr = 0x800a77fc; f_8009df64(c);
                pc = 0x800a77fc;
            }
            0x800a77fc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a7804; f_8009df64(c);
                pc = 0x800a7804;
            }
            0x800a7804 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7774: bad pc {:#x}", pc),
        }
    }
}

/// DVDSetAutoInvalidation (0x800a7820, 16 bytes)
#[inline(never)]
pub fn f_800a7820(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7820) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7820); p = true; }
    }
    f_800a7820_b(c, 0x800a7820);
    if p { crate::prof::exit(c, 0x800a7820); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7820_at(c: &mut Cpu, pc: u32) { f_800a7820_b(c, pc) }
#[inline(always)]
fn f_800a7820_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7820;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7820, pc); }
        match pc {
            0x800a7820 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ac));
                c.stw(c.r[13].wrapping_add(0xffff81ac), c.r[3]);
                c.r[3] = c.r[0] | c.r[0];
                if true { return; }
                return;
            }
            _ => panic!("f_800a7820: bad pc {:#x}", pc),
        }
    }
}

/// DVDResume (0x800a7830, 80 bytes)
#[inline(never)]
pub fn f_800a7830(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7830) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7830); p = true; }
    }
    f_800a7830_b(c, 0x800a7830);
    if p { crate::prof::exit(c, 0x800a7830); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7830_at(c: &mut Cpu, pc: u32) { f_800a7830_b(c, pc) }
#[inline(always)]
fn f_800a7830_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7830;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7830, pc); }
        match pc {
            0x800a7830 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x800a7844; f_8009df3c(c);
                pc = 0x800a7844;
            }
            0x800a7844 => {
                c.r[4] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fd4), c.r[4]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd8));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a7864; continue; }
                pc = 0x800a785c;
            }
            0x800a785c => {
                c.stw(c.r[13].wrapping_add(0xffff9fd8), c.r[4]);
                c.lr = 0x800a7864; f_800a6684(c);
                pc = 0x800a7864;
            }
            0x800a7864 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a786c; f_8009df64(c);
                pc = 0x800a786c;
            }
            0x800a786c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7830: bad pc {:#x}", pc),
        }
    }
}

/// DVDCancelAsync (0x800a7880, 636 bytes)
#[inline(never)]
pub fn f_800a7880(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7880) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7880); p = true; }
    }
    f_800a7880_b(c, 0x800a7880);
    if p { crate::prof::exit(c, 0x800a7880); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7880_at(c: &mut Cpu, pc: u32) { f_800a7880_b(c, pc) }
#[inline(never)]
fn f_800a7880_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7880;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7880, pc); }
        match pc {
            0x800a7880 => {
                c.r[0] = c.mfspr(8);
                pc = 0x800a7884;
            }
            0x800a7884 => {
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                pc = 0x800a7888;
            }
            0x800a7888 => {
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800a788c;
            }
            0x800a788c => {
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                pc = 0x800a7890;
            }
            0x800a7890 => {
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                pc = 0x800a7894;
            }
            0x800a7894 => {
                c.r[30] = c.r[4].wrapping_add(0x0);
                pc = 0x800a7898;
            }
            0x800a7898 => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                pc = 0x800a789c;
            }
            0x800a789c => {
                c.r[29] = c.r[3].wrapping_add(0x0);
                pc = 0x800a78a0;
            }
            0x800a78a0 => {
                c.lr = 0x800a78a4; f_8009df3c(c);
                pc = 0x800a78a4;
            }
            0x800a78a4 => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x800a78a8;
            }
            0x800a78a8 => {
                c.r[31] = c.r[3].wrapping_add(0x0);
                pc = 0x800a78ac;
            }
            0x800a78ac => {
                c.r[0] = c.r[4].wrapping_add(0x1);
                pc = 0x800a78b0;
            }
            0x800a78b0 => {
                c.cmpu(0, c.r[0], 0xc);
                pc = 0x800a78b4;
            }
            0x800a78b4 => {
                if c.crbit(1) { pc = 0x800a7ad4; continue; }
                pc = 0x800a78b8;
            }
            0x800a78b8 => {
                c.r[3] = 0x80310000;
                pc = 0x800a78bc;
            }
            0x800a78bc => {
                c.r[3] = c.r[3].wrapping_add(0x1b98);
                pc = 0x800a78c0;
            }
            0x800a78c0 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800a78c4;
            }
            0x800a78c4 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x800a78c8;
            }
            0x800a78c8 => {
                c.mtspr(9, c.r[0]);
                pc = 0x800a78cc;
            }
            0x800a78cc => {
                if true { let t = c.ctr; if (0x800a7880..0x800a7afc).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x800a78d0;
            }
            0x800a78d0 => {
                c.cmpu(0, c.r[30], 0x0);
                pc = 0x800a78d4;
            }
            0x800a78d4 => {
                if c.crbit(2) { pc = 0x800a7ad4; continue; }
                pc = 0x800a78d8;
            }
            0x800a78d8 => {
                c.r[12] = c.r[30].wrapping_add(0x0);
                pc = 0x800a78dc;
            }
            0x800a78dc => {
                c.mtspr(8, c.r[12]);
                pc = 0x800a78e0;
            }
            0x800a78e0 => {
                c.r[4] = c.r[29].wrapping_add(0x0);
                pc = 0x800a78e4;
            }
            0x800a78e4 => {
                c.r[3] = 0x0;
                pc = 0x800a78e8;
            }
            0x800a78e8 => {
                if true { let t = c.lr; c.lr = 0x800a78ec; (c.dispatch)(c, t); }
                pc = 0x800a78ec;
            }
            0x800a78ec => {
                pc = 0x800a7ad4; continue;
                pc = 0x800a78f0;
            }
            0x800a78f0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                pc = 0x800a78f4;
            }
            0x800a78f4 => {
                c.cmpu(0, c.r[0], 0x0);
                pc = 0x800a78f8;
            }
            0x800a78f8 => {
                if c.crbit(2) { pc = 0x800a790c; continue; }
                pc = 0x800a78fc;
            }
            0x800a78fc => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800a7900;
            }
            0x800a7900 => {
                c.lr = 0x800a7904; f_8009df64(c);
                pc = 0x800a7904;
            }
            0x800a7904 => {
                c.r[3] = 0x0;
                pc = 0x800a7908;
            }
            0x800a7908 => {
                pc = 0x800a7ae0; continue;
                pc = 0x800a790c;
            }
            0x800a790c => {
                c.r[0] = 0x1;
                pc = 0x800a7910;
            }
            0x800a7910 => {
                c.stw(c.r[13].wrapping_add(0xffff9fec), c.r[30]);
                pc = 0x800a7914;
            }
            0x800a7914 => {
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[0]);
                pc = 0x800a7918;
            }
            0x800a7918 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                pc = 0x800a791c;
            }
            0x800a791c => {
                c.cmpu(0, c.r[0], 0x4);
                pc = 0x800a7920;
            }
            0x800a7920 => {
                if c.crbit(2) { pc = 0x800a792c; continue; }
                pc = 0x800a7924;
            }
            0x800a7924 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x800a7928;
            }
            0x800a7928 => {
                if !c.crbit(2) { pc = 0x800a7ad4; continue; }
                pc = 0x800a792c;
            }
            0x800a792c => {
                c.lr = 0x800a7930; f_800a4c80(c);
                pc = 0x800a7930;
            }
            0x800a7930 => {
                pc = 0x800a7ad4; continue;
                pc = 0x800a7934;
            }
            0x800a7934 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x800a7938;
            }
            0x800a7938 => {
                c.lr = 0x800a793c; f_800a7f80(c);
                pc = 0x800a793c;
            }
            0x800a793c => {
                c.r[0] = 0xa;
                pc = 0x800a7940;
            }
            0x800a7940 => {
                c.stw(c.r[29].wrapping_add(0xc), c.r[0]);
                pc = 0x800a7944;
            }
            0x800a7944 => {
                c.r[12] = c.lwz(c.r[29].wrapping_add(0x28));
                pc = 0x800a7948;
            }
            0x800a7948 => {
                c.cmpu(0, c.r[12], 0x0);
                pc = 0x800a794c;
            }
            0x800a794c => {
                if c.crbit(2) { pc = 0x800a7960; continue; }
                pc = 0x800a7950;
            }
            0x800a7950 => {
                c.mtspr(8, c.r[12]);
                pc = 0x800a7954;
            }
            0x800a7954 => {
                c.r[4] = c.r[29].wrapping_add(0x0);
                pc = 0x800a7958;
            }
            0x800a7958 => {
                c.r[3] = 0xfffffffd;
                pc = 0x800a795c;
            }
            0x800a795c => {
                if true { let t = c.lr; c.lr = 0x800a7960; (c.dispatch)(c, t); }
                pc = 0x800a7960;
            }
            0x800a7960 => {
                c.cmpu(0, c.r[30], 0x0);
                pc = 0x800a7964;
            }
            0x800a7964 => {
                if c.crbit(2) { pc = 0x800a7ad4; continue; }
                pc = 0x800a7968;
            }
            0x800a7968 => {
                c.r[12] = c.r[30].wrapping_add(0x0);
                pc = 0x800a796c;
            }
            0x800a796c => {
                c.mtspr(8, c.r[12]);
                pc = 0x800a7970;
            }
            0x800a7970 => {
                c.r[4] = c.r[29].wrapping_add(0x0);
                pc = 0x800a7974;
            }
            0x800a7974 => {
                c.r[3] = 0x0;
                pc = 0x800a7978;
            }
            0x800a7978 => {
                if true { let t = c.lr; c.lr = 0x800a797c; (c.dispatch)(c, t); }
                pc = 0x800a797c;
            }
            0x800a797c => {
                pc = 0x800a7ad4; continue;
                pc = 0x800a7980;
            }
            0x800a7980 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                pc = 0x800a7984;
            }
            0x800a7984 => {
                c.cmps(0, c.r[0], 0xd);
                pc = 0x800a7988;
            }
            0x800a7988 => {
                if c.crbit(2) { pc = 0x800a79b0; continue; }
                pc = 0x800a798c;
            }
            0x800a798c => {
                if !c.crbit(0) { pc = 0x800a79a4; continue; }
                pc = 0x800a7990;
            }
            0x800a7990 => {
                c.cmps(0, c.r[0], 0x6);
                pc = 0x800a7994;
            }
            0x800a7994 => {
                if !c.crbit(0) { pc = 0x800a79d0; continue; }
                pc = 0x800a7998;
            }
            0x800a7998 => {
                c.cmps(0, c.r[0], 0x4);
                pc = 0x800a799c;
            }
            0x800a799c => {
                if !c.crbit(0) { pc = 0x800a79b0; continue; }
                pc = 0x800a79a0;
            }
            0x800a79a0 => {
                pc = 0x800a79d0; continue;
                pc = 0x800a79a4;
            }
            0x800a79a4 => {
                c.cmps(0, c.r[0], 0xf);
                pc = 0x800a79a8;
            }
            0x800a79a8 => {
                if c.crbit(2) { pc = 0x800a79b0; continue; }
                pc = 0x800a79ac;
            }
            0x800a79ac => {
                pc = 0x800a79d0; continue;
                pc = 0x800a79b0;
            }
            0x800a79b0 => {
                c.cmpu(0, c.r[30], 0x0);
                pc = 0x800a79b4;
            }
            0x800a79b4 => {
                if c.crbit(2) { pc = 0x800a7ad4; continue; }
                pc = 0x800a79b8;
            }
            0x800a79b8 => {
                c.r[12] = c.r[30].wrapping_add(0x0);
                pc = 0x800a79bc;
            }
            0x800a79bc => {
                c.mtspr(8, c.r[12]);
                pc = 0x800a79c0;
            }
            0x800a79c0 => {
                c.r[4] = c.r[29].wrapping_add(0x0);
                pc = 0x800a79c4;
            }
            0x800a79c4 => {
                c.r[3] = 0x0;
                pc = 0x800a79c8;
            }
            0x800a79c8 => {
                if true { let t = c.lr; c.lr = 0x800a79cc; (c.dispatch)(c, t); }
                pc = 0x800a79cc;
            }
            0x800a79cc => {
                pc = 0x800a7ad4; continue;
                pc = 0x800a79d0;
            }
            0x800a79d0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                pc = 0x800a79d4;
            }
            0x800a79d4 => {
                c.cmpu(0, c.r[0], 0x0);
                pc = 0x800a79d8;
            }
            0x800a79d8 => {
                if c.crbit(2) { pc = 0x800a79ec; continue; }
                pc = 0x800a79dc;
            }
            0x800a79dc => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800a79e0;
            }
            0x800a79e0 => {
                c.lr = 0x800a79e4; f_8009df64(c);
                pc = 0x800a79e4;
            }
            0x800a79e4 => {
                c.r[3] = 0x0;
                pc = 0x800a79e8;
            }
            0x800a79e8 => {
                pc = 0x800a7ae0; continue;
                pc = 0x800a79ec;
            }
            0x800a79ec => {
                c.r[0] = 0x1;
                pc = 0x800a79f0;
            }
            0x800a79f0 => {
                c.stw(c.r[13].wrapping_add(0xffff9fec), c.r[30]);
                pc = 0x800a79f4;
            }
            0x800a79f4 => {
                c.stw(c.r[13].wrapping_add(0xffff9fe8), c.r[0]);
                pc = 0x800a79f8;
            }
            0x800a79f8 => {
                pc = 0x800a7ad4; continue;
                pc = 0x800a79fc;
            }
            0x800a79fc => {
                c.lr = 0x800a7a00; f_800a4c94(c);
                pc = 0x800a7a00;
            }
            0x800a7a00 => {
                c.r[4] = 0x800a0000;
                pc = 0x800a7a04;
            }
            0x800a7a04 => {
                c.r[0] = c.r[4].wrapping_add(0x65a0);
                pc = 0x800a7a08;
            }
            0x800a7a08 => {
                c.cmpu(0, c.r[3], c.r[0]);
                pc = 0x800a7a0c;
            }
            0x800a7a0c => {
                if c.crbit(2) { pc = 0x800a7a20; continue; }
                pc = 0x800a7a10;
            }
            0x800a7a10 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800a7a14;
            }
            0x800a7a14 => {
                c.lr = 0x800a7a18; f_8009df64(c);
                pc = 0x800a7a18;
            }
            0x800a7a18 => {
                c.r[3] = 0x0;
                pc = 0x800a7a1c;
            }
            0x800a7a1c => {
                pc = 0x800a7ae0; continue;
                pc = 0x800a7a20;
            }
            0x800a7a20 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x800a7a24;
            }
            0x800a7a24 => {
                c.cmps(0, c.r[0], 0x4);
                pc = 0x800a7a28;
            }
            0x800a7a28 => {
                if !c.crbit(2) { pc = 0x800a7a34; continue; }
                pc = 0x800a7a2c;
            }
            0x800a7a2c => {
                c.r[0] = 0x3;
                pc = 0x800a7a30;
            }
            0x800a7a30 => {
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[0]);
                pc = 0x800a7a34;
            }
            0x800a7a34 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x800a7a38;
            }
            0x800a7a38 => {
                c.cmps(0, c.r[0], 0x5);
                pc = 0x800a7a3c;
            }
            0x800a7a3c => {
                if !c.crbit(2) { pc = 0x800a7a48; continue; }
                pc = 0x800a7a40;
            }
            0x800a7a40 => {
                c.r[0] = 0x4;
                pc = 0x800a7a44;
            }
            0x800a7a44 => {
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[0]);
                pc = 0x800a7a48;
            }
            0x800a7a48 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x800a7a4c;
            }
            0x800a7a4c => {
                c.cmps(0, c.r[0], 0x6);
                pc = 0x800a7a50;
            }
            0x800a7a50 => {
                if !c.crbit(2) { pc = 0x800a7a5c; continue; }
                pc = 0x800a7a54;
            }
            0x800a7a54 => {
                c.r[0] = 0x1;
                pc = 0x800a7a58;
            }
            0x800a7a58 => {
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[0]);
                pc = 0x800a7a5c;
            }
            0x800a7a5c => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x800a7a60;
            }
            0x800a7a60 => {
                c.cmps(0, c.r[0], 0xb);
                pc = 0x800a7a64;
            }
            0x800a7a64 => {
                if !c.crbit(2) { pc = 0x800a7a70; continue; }
                pc = 0x800a7a68;
            }
            0x800a7a68 => {
                c.r[0] = 0x2;
                pc = 0x800a7a6c;
            }
            0x800a7a6c => {
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[0]);
                pc = 0x800a7a70;
            }
            0x800a7a70 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x800a7a74;
            }
            0x800a7a74 => {
                c.cmps(0, c.r[0], 0x7);
                pc = 0x800a7a78;
            }
            0x800a7a78 => {
                if !c.crbit(2) { pc = 0x800a7a84; continue; }
                pc = 0x800a7a7c;
            }
            0x800a7a7c => {
                c.r[0] = 0x7;
                pc = 0x800a7a80;
            }
            0x800a7a80 => {
                c.stw(c.r[13].wrapping_add(0xffff9ff0), c.r[0]);
                pc = 0x800a7a84;
            }
            0x800a7a84 => {
                c.r[3] = 0x80400000;
                pc = 0x800a7a88;
            }
            0x800a7a88 => {
                c.r[0] = c.r[3].wrapping_add(0xffffc3a0);
                pc = 0x800a7a8c;
            }
            0x800a7a8c => {
                c.stw(c.r[13].wrapping_add(0xffff9fc8), c.r[0]);
                pc = 0x800a7a90;
            }
            0x800a7a90 => {
                c.r[0] = 0xa;
                pc = 0x800a7a94;
            }
            0x800a7a94 => {
                c.stw(c.r[29].wrapping_add(0xc), c.r[0]);
                pc = 0x800a7a98;
            }
            0x800a7a98 => {
                c.r[12] = c.lwz(c.r[29].wrapping_add(0x28));
                pc = 0x800a7a9c;
            }
            0x800a7a9c => {
                c.cmpu(0, c.r[12], 0x0);
                pc = 0x800a7aa0;
            }
            0x800a7aa0 => {
                if c.crbit(2) { pc = 0x800a7ab4; continue; }
                pc = 0x800a7aa4;
            }
            0x800a7aa4 => {
                c.mtspr(8, c.r[12]);
                pc = 0x800a7aa8;
            }
            0x800a7aa8 => {
                c.r[4] = c.r[29].wrapping_add(0x0);
                pc = 0x800a7aac;
            }
            0x800a7aac => {
                c.r[3] = 0xfffffffd;
                pc = 0x800a7ab0;
            }
            0x800a7ab0 => {
                if true { let t = c.lr; c.lr = 0x800a7ab4; (c.dispatch)(c, t); }
                pc = 0x800a7ab4;
            }
            0x800a7ab4 => {
                c.cmpu(0, c.r[30], 0x0);
                pc = 0x800a7ab8;
            }
            0x800a7ab8 => {
                if c.crbit(2) { pc = 0x800a7ad0; continue; }
                pc = 0x800a7abc;
            }
            0x800a7abc => {
                c.r[12] = c.r[30].wrapping_add(0x0);
                pc = 0x800a7ac0;
            }
            0x800a7ac0 => {
                c.mtspr(8, c.r[12]);
                pc = 0x800a7ac4;
            }
            0x800a7ac4 => {
                c.r[4] = c.r[29].wrapping_add(0x0);
                pc = 0x800a7ac8;
            }
            0x800a7ac8 => {
                c.r[3] = 0x0;
                pc = 0x800a7acc;
            }
            0x800a7acc => {
                if true { let t = c.lr; c.lr = 0x800a7ad0; (c.dispatch)(c, t); }
                pc = 0x800a7ad0;
            }
            0x800a7ad0 => {
                c.lr = 0x800a7ad4; f_800a6684(c);
                pc = 0x800a7ad4;
            }
            0x800a7ad4 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800a7ad8;
            }
            0x800a7ad8 => {
                c.lr = 0x800a7adc; f_8009df64(c);
                pc = 0x800a7adc;
            }
            0x800a7adc => {
                c.r[3] = 0x1;
                pc = 0x800a7ae0;
            }
            0x800a7ae0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                pc = 0x800a7ae4;
            }
            0x800a7ae4 => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x800a7ae8;
            }
            0x800a7ae8 => {
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                pc = 0x800a7aec;
            }
            0x800a7aec => {
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                pc = 0x800a7af0;
            }
            0x800a7af0 => {
                c.r[1] = c.r[1].wrapping_add(0x28);
                pc = 0x800a7af4;
            }
            0x800a7af4 => {
                c.mtspr(8, c.r[0]);
                pc = 0x800a7af8;
            }
            0x800a7af8 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800a7880: bad pc {:#x}", pc),
        }
    }
}

/// DVDCancel (0x800a7afc, 172 bytes)
#[inline(never)]
pub fn f_800a7afc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7afc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7afc); p = true; }
    }
    f_800a7afc_b(c, 0x800a7afc);
    if p { crate::prof::exit(c, 0x800a7afc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7afc_at(c: &mut Cpu, pc: u32) { f_800a7afc_b(c, pc) }
#[inline(always)]
fn f_800a7afc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7afc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7afc, pc); }
        match pc {
            0x800a7afc => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x800a0000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x7ba8);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800a7b20; f_800a7880(c);
                pc = 0x800a7b20;
            }
            0x800a7b20 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a7b30; continue; }
                pc = 0x800a7b28;
            }
            0x800a7b28 => {
                c.r[3] = 0xffffffff;
                pc = 0x800a7b90; continue;
                pc = 0x800a7b30;
            }
            0x800a7b30 => {
                c.lr = 0x800a7b34; f_8009df3c(c);
                pc = 0x800a7b34;
            }
            0x800a7b34 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800a7b38;
            }
            0x800a7b38 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x800a7b84; continue; }
                pc = 0x800a7b48;
            }
            0x800a7b48 => {
                c.cmps(0, c.r[3], 0xa);
                if c.crbit(2) { pc = 0x800a7b84; continue; }
                pc = 0x800a7b50;
            }
            0x800a7b50 => {
                c.cmps(0, c.r[3], 0x3);
                if !c.crbit(2) { pc = 0x800a7b78; continue; }
                pc = 0x800a7b58;
            }
            0x800a7b58 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0xfffffffc);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(1) { pc = 0x800a7b84; continue; }
                pc = 0x800a7b68;
            }
            0x800a7b68 => {
                c.cmpu(0, c.r[3], 0xd);
                if c.crbit(2) { pc = 0x800a7b84; continue; }
                pc = 0x800a7b70;
            }
            0x800a7b70 => {
                c.cmpu(0, c.r[3], 0xf);
                if c.crbit(2) { pc = 0x800a7b84; continue; }
                pc = 0x800a7b78;
            }
            0x800a7b78 => {
                c.r[3] = c.r[13].wrapping_add(0xffff9fc0);
                c.lr = 0x800a7b80; f_800a238c(c);
                pc = 0x800a7b80;
            }
            0x800a7b80 => {
                pc = 0x800a7b38; continue;
                pc = 0x800a7b84;
            }
            0x800a7b84 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a7b8c; f_8009df64(c);
                pc = 0x800a7b8c;
            }
            0x800a7b8c => {
                c.r[3] = 0x0;
                pc = 0x800a7b90;
            }
            0x800a7b90 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7afc: bad pc {:#x}", pc),
        }
    }
}

/// cbForCancelSync (0x800a7ba8, 36 bytes)
#[inline(never)]
pub fn f_800a7ba8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7ba8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7ba8); p = true; }
    }
    f_800a7ba8_b(c, 0x800a7ba8);
    if p { crate::prof::exit(c, 0x800a7ba8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7ba8_at(c: &mut Cpu, pc: u32) { f_800a7ba8_b(c, pc) }
#[inline(always)]
fn f_800a7ba8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7ba8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7ba8, pc); }
        match pc {
            0x800a7ba8 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = c.r[13].wrapping_add(0xffff9fc0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a7bbc; f_800a2478(c);
                pc = 0x800a7bbc;
            }
            0x800a7bbc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7ba8: bad pc {:#x}", pc),
        }
    }
}

/// DVDGetCurrentDiskID (0x800a7bcc, 8 bytes)
#[inline(never)]
pub fn f_800a7bcc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7bcc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7bcc); p = true; }
    }
    f_800a7bcc_b(c, 0x800a7bcc);
    if p { crate::prof::exit(c, 0x800a7bcc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7bcc_at(c: &mut Cpu, pc: u32) { f_800a7bcc_b(c, pc) }
#[inline(always)]
fn f_800a7bcc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7bcc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7bcc, pc); }
        match pc {
            0x800a7bcc => {
                c.r[3] = 0x80000000;
                if true { return; }
                return;
            }
            _ => panic!("f_800a7bcc: bad pc {:#x}", pc),
        }
    }
}

/// DVDCheckDisk (0x800a7bd4, 248 bytes)
#[inline(never)]
pub fn f_800a7bd4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7bd4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7bd4); p = true; }
    }
    f_800a7bd4_b(c, 0x800a7bd4);
    if p { crate::prof::exit(c, 0x800a7bd4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7bd4_at(c: &mut Cpu, pc: u32) { f_800a7bd4_b(c, pc) }
#[inline(never)]
fn f_800a7bd4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7bd4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7bd4, pc); }
        match pc {
            0x800a7bd4 => {
                c.r[0] = c.mfspr(8);
                pc = 0x800a7bd8;
            }
            0x800a7bd8 => {
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                pc = 0x800a7bdc;
            }
            0x800a7bdc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800a7be0;
            }
            0x800a7be0 => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                pc = 0x800a7be4;
            }
            0x800a7be4 => {
                c.lr = 0x800a7be8; f_8009df3c(c);
                pc = 0x800a7be8;
            }
            0x800a7be8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe0));
                pc = 0x800a7bec;
            }
            0x800a7bec => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x800a7bf0;
            }
            0x800a7bf0 => {
                if c.crbit(2) { pc = 0x800a7bfc; continue; }
                pc = 0x800a7bf4;
            }
            0x800a7bf4 => {
                c.r[4] = 0xffffffff;
                pc = 0x800a7bf8;
            }
            0x800a7bf8 => {
                pc = 0x800a7c40; continue;
                pc = 0x800a7bfc;
            }
            0x800a7bfc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd8));
                pc = 0x800a7c00;
            }
            0x800a7c00 => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x800a7c04;
            }
            0x800a7c04 => {
                if c.crbit(2) { pc = 0x800a7c10; continue; }
                pc = 0x800a7c08;
            }
            0x800a7c08 => {
                c.r[4] = 0x8;
                pc = 0x800a7c0c;
            }
            0x800a7c0c => {
                pc = 0x800a7c40; continue;
                pc = 0x800a7c10;
            }
            0x800a7c10 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                pc = 0x800a7c14;
            }
            0x800a7c14 => {
                c.cmpu(0, c.r[5], 0x0);
                pc = 0x800a7c18;
            }
            0x800a7c18 => {
                if !c.crbit(2) { pc = 0x800a7c24; continue; }
                pc = 0x800a7c1c;
            }
            0x800a7c1c => {
                c.r[4] = 0x0;
                pc = 0x800a7c20;
            }
            0x800a7c20 => {
                pc = 0x800a7c40; continue;
                pc = 0x800a7c24;
            }
            0x800a7c24 => {
                c.r[4] = 0x80400000;
                pc = 0x800a7c28;
            }
            0x800a7c28 => {
                c.r[0] = c.r[4].wrapping_add(0xffffc3a0);
                pc = 0x800a7c2c;
            }
            0x800a7c2c => {
                c.cmpu(0, c.r[5], c.r[0]);
                pc = 0x800a7c30;
            }
            0x800a7c30 => {
                if !c.crbit(2) { pc = 0x800a7c3c; continue; }
                pc = 0x800a7c34;
            }
            0x800a7c34 => {
                c.r[4] = 0x0;
                pc = 0x800a7c38;
            }
            0x800a7c38 => {
                pc = 0x800a7c40; continue;
                pc = 0x800a7c3c;
            }
            0x800a7c3c => {
                c.r[4] = c.lwz(c.r[5].wrapping_add(0xc));
                pc = 0x800a7c40;
            }
            0x800a7c40 => {
                c.r[0] = c.r[4].wrapping_add(0x1);
                pc = 0x800a7c44;
            }
            0x800a7c44 => {
                c.cmpu(0, c.r[0], 0xc);
                pc = 0x800a7c48;
            }
            0x800a7c48 => {
                if c.crbit(1) { pc = 0x800a7cb0; continue; }
                pc = 0x800a7c4c;
            }
            0x800a7c4c => {
                c.r[4] = 0x80310000;
                pc = 0x800a7c50;
            }
            0x800a7c50 => {
                c.r[4] = c.r[4].wrapping_add(0x1bcc);
                pc = 0x800a7c54;
            }
            0x800a7c54 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800a7c58;
            }
            0x800a7c58 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                pc = 0x800a7c5c;
            }
            0x800a7c5c => {
                c.mtspr(9, c.r[0]);
                pc = 0x800a7c60;
            }
            0x800a7c60 => {
                if true { let t = c.ctr; if (0x800a7bd4..0x800a7ccc).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x800a7c64;
            }
            0x800a7c64 => {
                c.r[31] = 0x1;
                pc = 0x800a7c68;
            }
            0x800a7c68 => {
                pc = 0x800a7cb0; continue;
                pc = 0x800a7c6c;
            }
            0x800a7c6c => {
                c.r[31] = 0x0;
                pc = 0x800a7c70;
            }
            0x800a7c70 => {
                pc = 0x800a7cb0; continue;
                pc = 0x800a7c74;
            }
            0x800a7c74 => {
                c.r[4] = 0xcc000000;
                pc = 0x800a7c78;
            }
            0x800a7c78 => {
                c.r[4] = c.r[4].wrapping_add(0x6000);
                pc = 0x800a7c7c;
            }
            0x800a7c7c => {
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x4));
                pc = 0x800a7c80;
            }
            0x800a7c80 => {
                c.r[0] = c.r[4].rotate_left(30) & 0x1;
                c.rc(c.r[0]);
                pc = 0x800a7c84;
            }
            0x800a7c84 => {
                if !c.crbit(2) { pc = 0x800a7c90; continue; }
                pc = 0x800a7c88;
            }
            0x800a7c88 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                pc = 0x800a7c8c;
            }
            0x800a7c8c => {
                if c.crbit(2) { pc = 0x800a7c98; continue; }
                pc = 0x800a7c90;
            }
            0x800a7c90 => {
                c.r[31] = 0x0;
                pc = 0x800a7c94;
            }
            0x800a7c94 => {
                pc = 0x800a7cb0; continue;
                pc = 0x800a7c98;
            }
            0x800a7c98 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9ff0));
                pc = 0x800a7c9c;
            }
            0x800a7c9c => {
                c.cmpu(0, c.r[0], 0x0);
                pc = 0x800a7ca0;
            }
            0x800a7ca0 => {
                if c.crbit(2) { pc = 0x800a7cac; continue; }
                pc = 0x800a7ca4;
            }
            0x800a7ca4 => {
                c.r[31] = 0x0;
                pc = 0x800a7ca8;
            }
            0x800a7ca8 => {
                pc = 0x800a7cb0; continue;
                pc = 0x800a7cac;
            }
            0x800a7cac => {
                c.r[31] = 0x1;
                pc = 0x800a7cb0;
            }
            0x800a7cb0 => {
                c.lr = 0x800a7cb4; f_8009df64(c);
                pc = 0x800a7cb4;
            }
            0x800a7cb4 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800a7cb8;
            }
            0x800a7cb8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x800a7cbc;
            }
            0x800a7cbc => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x800a7cc0;
            }
            0x800a7cc0 => {
                c.r[1] = c.r[1].wrapping_add(0x10);
                pc = 0x800a7cc4;
            }
            0x800a7cc4 => {
                c.mtspr(8, c.r[0]);
                pc = 0x800a7cc8;
            }
            0x800a7cc8 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800a7bd4: bad pc {:#x}", pc),
        }
    }
}

/// __DVDPrepareResetAsync (0x800a7ccc, 284 bytes)
#[inline(never)]
pub fn f_800a7ccc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7ccc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7ccc); p = true; }
    }
    f_800a7ccc_b(c, 0x800a7ccc);
    if p { crate::prof::exit(c, 0x800a7ccc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7ccc_at(c: &mut Cpu, pc: u32) { f_800a7ccc_b(c, pc) }
#[inline(always)]
fn f_800a7ccc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7ccc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7ccc, pc); }
        match pc {
            0x800a7ccc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.lr = 0x800a7cec; f_8009df3c(c);
                pc = 0x800a7cec;
            }
            0x800a7cec => {
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x800a7cf4; f_800a7de8(c);
                pc = 0x800a7cf4;
            }
            0x800a7cf4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fe8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a7d08; continue; }
                pc = 0x800a7d00;
            }
            0x800a7d00 => {
                c.stw(c.r[13].wrapping_add(0xffff9fec), c.r[30]);
                pc = 0x800a7dc4; continue;
                pc = 0x800a7d08;
            }
            0x800a7d08 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a7d1c; continue; }
                pc = 0x800a7d14;
            }
            0x800a7d14 => {
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x28), c.r[0]);
                pc = 0x800a7d1c;
            }
            0x800a7d1c => {
                c.lr = 0x800a7d20; f_8009df3c(c);
                pc = 0x800a7d20;
            }
            0x800a7d20 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800a7d28; f_8009df3c(c);
                pc = 0x800a7d28;
            }
            0x800a7d28 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.r[4] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9fd4), c.r[4]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a7d40; continue; }
                pc = 0x800a7d3c;
            }
            0x800a7d3c => {
                c.stw(c.r[13].wrapping_add(0xffff9fd8), c.r[4]);
                pc = 0x800a7d40;
            }
            0x800a7d40 => {
                c.lr = 0x800a7d44; f_8009df64(c);
                pc = 0x800a7d44;
            }
            0x800a7d44 => {
                pc = 0x800a7d50; continue;
                pc = 0x800a7d48;
            }
            0x800a7d48 => {
                c.r[4] = 0x0;
                c.lr = 0x800a7d50; f_800a7880(c);
                pc = 0x800a7d50;
            }
            0x800a7d50 => {
                c.lr = 0x800a7d54; f_800a7e88(c);
                pc = 0x800a7d54;
            }
            0x800a7d54 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a7d48; continue; }
                pc = 0x800a7d5c;
            }
            0x800a7d5c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9fc8));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a7d74; continue; }
                pc = 0x800a7d68;
            }
            0x800a7d68 => {
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x800a7d70; f_800a7880(c);
                pc = 0x800a7d70;
            }
            0x800a7d70 => {
                pc = 0x800a7d90; continue;
                pc = 0x800a7d74;
            }
            0x800a7d74 => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x800a7d90; continue; }
                pc = 0x800a7d7c;
            }
            0x800a7d7c => {
                c.r[12] = c.r[30].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                if true { let t = c.lr; c.lr = 0x800a7d90; (c.dispatch)(c, t); }
                pc = 0x800a7d90;
            }
            0x800a7d90 => {
                c.lr = 0x800a7d94; f_8009df3c(c);
                pc = 0x800a7d94;
            }
            0x800a7d94 => {
                c.r[4] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9fd4), c.r[4]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9fd8));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a7db4; continue; }
                pc = 0x800a7dac;
            }
            0x800a7dac => {
                c.stw(c.r[13].wrapping_add(0xffff9fd8), c.r[4]);
                c.lr = 0x800a7db4; f_800a6684(c);
                pc = 0x800a7db4;
            }
            0x800a7db4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a7dbc; f_8009df64(c);
                pc = 0x800a7dbc;
            }
            0x800a7dbc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a7dc4; f_8009df64(c);
                pc = 0x800a7dc4;
            }
            0x800a7dc4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a7dcc; f_8009df64(c);
                pc = 0x800a7dcc;
            }
            0x800a7dcc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7ccc: bad pc {:#x}", pc),
        }
    }
}

/// __DVDClearWaitingQueue (0x800a7de8, 56 bytes)
#[inline(never)]
pub fn f_800a7de8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7de8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7de8); p = true; }
    }
    f_800a7de8_b(c, 0x800a7de8);
    if p { crate::prof::exit(c, 0x800a7de8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7de8_at(c: &mut Cpu, pc: u32) { f_800a7de8_b(c, pc) }
#[inline(always)]
fn f_800a7de8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7de8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7de8, pc); }
        match pc {
            0x800a7de8 => {
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffc3f8);
                c.stw(c.r[3], c.r[3]);
                c.r[5] = c.r[3].wrapping_add(0x8);
                c.r[4] = c.r[3].wrapping_add(0x10);
                c.stw(c.r[3].wrapping_add(0x4), c.r[3]);
                c.r[3] = c.r[3].wrapping_add(0x18);
                c.stw(c.r[5], c.r[5]);
                c.stw(c.r[5].wrapping_add(0x4), c.r[5]);
                c.stw(c.r[4], c.r[4]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[4]);
                c.stw(c.r[3], c.r[3]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7de8: bad pc {:#x}", pc),
        }
    }
}

/// __DVDPushWaitingQueue (0x800a7e20, 104 bytes)
#[inline(never)]
pub fn f_800a7e20(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7e20) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7e20); p = true; }
    }
    f_800a7e20_b(c, 0x800a7e20);
    if p { crate::prof::exit(c, 0x800a7e20); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7e20_at(c: &mut Cpu, pc: u32) { f_800a7e20_b(c, pc) }
#[inline(always)]
fn f_800a7e20_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7e20;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7e20, pc); }
        match pc {
            0x800a7e20 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800a7e40; f_8009df3c(c);
                pc = 0x800a7e40;
            }
            0x800a7e40 => {
                c.r[4] = 0x80400000;
                c.r[5] = c.r[30].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[4].wrapping_add(0xffffc3f8);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                c.r[4] = c.lwz(c.r[5].wrapping_add(0x4));
                c.stw(c.r[4], c.r[31]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.stw(c.r[31].wrapping_add(0x4), c.r[0]);
                c.stw(c.r[31], c.r[5]);
                c.stw(c.r[5].wrapping_add(0x4), c.r[31]);
                c.lr = 0x800a7e6c; f_8009df64(c);
                pc = 0x800a7e6c;
            }
            0x800a7e6c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7e20: bad pc {:#x}", pc),
        }
    }
}

/// __DVDPopWaitingQueue (0x800a7e88, 160 bytes)
#[inline(never)]
pub fn f_800a7e88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7e88) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7e88); p = true; }
    }
    f_800a7e88_b(c, 0x800a7e88);
    if p { crate::prof::exit(c, 0x800a7e88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7e88_at(c: &mut Cpu, pc: u32) { f_800a7e88_b(c, pc) }
#[inline(always)]
fn f_800a7e88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7e88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7e88, pc); }
        match pc {
            0x800a7e88 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x800a7e9c; f_8009df3c(c);
                pc = 0x800a7e9c;
            }
            0x800a7e9c => {
                c.r[0] = 0x4;
                c.r[4] = 0x80400000;
                c.mtspr(9, c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0xffffc3f8);
                c.r[31] = 0x0;
                pc = 0x800a7eb0;
            }
            0x800a7eb0 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], c.r[4]);
                if c.crbit(2) { pc = 0x800a7f00; continue; }
                pc = 0x800a7ebc;
            }
            0x800a7ebc => {
                c.lr = 0x800a7ec0; f_8009df64(c);
                pc = 0x800a7ec0;
            }
            0x800a7ec0 => {
                c.lr = 0x800a7ec4; f_8009df3c(c);
                pc = 0x800a7ec4;
            }
            0x800a7ec4 => {
                c.r[4] = 0x80400000;
                c.r[5] = c.r[31].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[4].wrapping_add(0xffffc3f8);
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                c.r[31] = c.lwz(c.r[5]);
                c.r[0] = c.lwz(c.r[31]);
                c.stw(c.r[5], c.r[0]);
                c.r[4] = c.lwz(c.r[31]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                c.lr = 0x800a7eec; f_8009df64(c);
                pc = 0x800a7eec;
            }
            0x800a7eec => {
                c.r[0] = 0x0;
                c.stw(c.r[31], c.r[0]);
                c.r[3] = c.r[31] | c.r[31];
                c.stw(c.r[31].wrapping_add(0x4), c.r[0]);
                pc = 0x800a7f14; continue;
                pc = 0x800a7f00;
            }
            0x800a7f00 => {
                c.r[4] = c.r[4].wrapping_add(0x8);
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800a7eb0; continue; }
                pc = 0x800a7f0c;
            }
            0x800a7f0c => {
                c.lr = 0x800a7f10; f_8009df64(c);
                pc = 0x800a7f10;
            }
            0x800a7f10 => {
                c.r[3] = 0x0;
                pc = 0x800a7f14;
            }
            0x800a7f14 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7e88: bad pc {:#x}", pc),
        }
    }
}

/// __DVDCheckWaitingQueue (0x800a7f28, 88 bytes)
#[inline(never)]
pub fn f_800a7f28(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7f28) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7f28); p = true; }
    }
    f_800a7f28_b(c, 0x800a7f28);
    if p { crate::prof::exit(c, 0x800a7f28); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7f28_at(c: &mut Cpu, pc: u32) { f_800a7f28_b(c, pc) }
#[inline(always)]
fn f_800a7f28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7f28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7f28, pc); }
        match pc {
            0x800a7f28 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a7f38; f_8009df3c(c);
                pc = 0x800a7f38;
            }
            0x800a7f38 => {
                c.r[0] = 0x4;
                c.r[4] = 0x80400000;
                c.mtspr(9, c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0xffffc3f8);
                pc = 0x800a7f48;
            }
            0x800a7f48 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], c.r[4]);
                if c.crbit(2) { pc = 0x800a7f60; continue; }
                pc = 0x800a7f54;
            }
            0x800a7f54 => {
                c.lr = 0x800a7f58; f_8009df64(c);
                pc = 0x800a7f58;
            }
            0x800a7f58 => {
                c.r[3] = 0x1;
                pc = 0x800a7f70; continue;
                pc = 0x800a7f60;
            }
            0x800a7f60 => {
                c.r[4] = c.r[4].wrapping_add(0x8);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800a7f48; continue; }
                pc = 0x800a7f68;
            }
            0x800a7f68 => {
                c.lr = 0x800a7f6c; f_8009df64(c);
                pc = 0x800a7f6c;
            }
            0x800a7f6c => {
                c.r[3] = 0x0;
                pc = 0x800a7f70;
            }
            0x800a7f70 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7f28: bad pc {:#x}", pc),
        }
    }
}

/// __DVDDequeueWaitingQueue (0x800a7f80, 96 bytes)
#[inline(never)]
pub fn f_800a7f80(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7f80) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7f80); p = true; }
    }
    f_800a7f80_b(c, 0x800a7f80);
    if p { crate::prof::exit(c, 0x800a7f80); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7f80_at(c: &mut Cpu, pc: u32) { f_800a7f80_b(c, pc) }
#[inline(always)]
fn f_800a7f80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7f80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7f80, pc); }
        match pc {
            0x800a7f80 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800a7f98; f_8009df3c(c);
                pc = 0x800a7f98;
            }
            0x800a7f98 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[5] = c.lwz(c.r[31]);
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800a7fb0; continue; }
                pc = 0x800a7fa8;
            }
            0x800a7fa8 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a7fbc; continue; }
                pc = 0x800a7fb0;
            }
            0x800a7fb0 => {
                c.lr = 0x800a7fb4; f_8009df64(c);
                pc = 0x800a7fb4;
            }
            0x800a7fb4 => {
                c.r[3] = 0x0;
                pc = 0x800a7fcc; continue;
                pc = 0x800a7fbc;
            }
            0x800a7fbc => {
                c.stw(c.r[4], c.r[5]);
                c.stw(c.r[5].wrapping_add(0x4), c.r[4]);
                c.lr = 0x800a7fc8; f_8009df64(c);
                pc = 0x800a7fc8;
            }
            0x800a7fc8 => {
                c.r[3] = 0x1;
                pc = 0x800a7fcc;
            }
            0x800a7fcc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a7f80: bad pc {:#x}", pc),
        }
    }
}

/// ErrorCode2Num (0x800a7fe0, 284 bytes)
#[inline(never)]
pub fn f_800a7fe0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a7fe0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a7fe0); p = true; }
    }
    f_800a7fe0_b(c, 0x800a7fe0);
    if p { crate::prof::exit(c, 0x800a7fe0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a7fe0_at(c: &mut Cpu, pc: u32) { f_800a7fe0_b(c, pc) }
#[inline(always)]
fn f_800a7fe0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a7fe0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a7fe0, pc); }
        match pc {
            0x800a7fe0 => {
                c.r[0] = 0x2;
                c.r[4] = 0x80310000;
                c.mtspr(9, c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x1c00);
                c.r[5] = 0x0;
                pc = 0x800a7ff4;
            }
            0x800a7ff4 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a8008; continue; }
                pc = 0x800a8000;
            }
            0x800a8000 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if true { return; }
                pc = 0x800a8008;
            }
            0x800a8008 => {
                { let ea = c.r[4].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a8020; continue; }
                pc = 0x800a8018;
            }
            0x800a8018 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if true { return; }
                pc = 0x800a8020;
            }
            0x800a8020 => {
                { let ea = c.r[4].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a8038; continue; }
                pc = 0x800a8030;
            }
            0x800a8030 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if true { return; }
                pc = 0x800a8038;
            }
            0x800a8038 => {
                { let ea = c.r[4].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a8050; continue; }
                pc = 0x800a8048;
            }
            0x800a8048 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if true { return; }
                pc = 0x800a8050;
            }
            0x800a8050 => {
                { let ea = c.r[4].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a8068; continue; }
                pc = 0x800a8060;
            }
            0x800a8060 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if true { return; }
                pc = 0x800a8068;
            }
            0x800a8068 => {
                { let ea = c.r[4].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a8080; continue; }
                pc = 0x800a8078;
            }
            0x800a8078 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if true { return; }
                pc = 0x800a8080;
            }
            0x800a8080 => {
                { let ea = c.r[4].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a8098; continue; }
                pc = 0x800a8090;
            }
            0x800a8090 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if true { return; }
                pc = 0x800a8098;
            }
            0x800a8098 => {
                { let ea = c.r[4].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a80b0; continue; }
                pc = 0x800a80a8;
            }
            0x800a80a8 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if true { return; }
                pc = 0x800a80b0;
            }
            0x800a80b0 => {
                { let ea = c.r[4].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800a80c8; continue; }
                pc = 0x800a80c0;
            }
            0x800a80c0 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xff;
                if true { return; }
                pc = 0x800a80c8;
            }
            0x800a80c8 => {
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800a7ff4; continue; }
                pc = 0x800a80d4;
            }
            0x800a80d4 => {
                c.r[4] = 0x100000;
                c.cmpu(0, c.r[3], c.r[4]);
                if c.crbit(0) { pc = 0x800a80f4; continue; }
                pc = 0x800a80e0;
            }
            0x800a80e0 => {
                c.r[0] = c.r[4].wrapping_add(0x8);
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(1) { pc = 0x800a80f4; continue; }
                pc = 0x800a80ec;
            }
            0x800a80ec => {
                c.r[3] = 0x11;
                if true { return; }
                pc = 0x800a80f4;
            }
            0x800a80f4 => {
                c.r[3] = 0x1d;
                if true { return; }
                return;
            }
            _ => panic!("f_800a7fe0: bad pc {:#x}", pc),
        }
    }
}

/// __DVDStoreErrorCode (0x800a80fc, 124 bytes)
#[inline(never)]
pub fn f_800a80fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a80fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a80fc); p = true; }
    }
    f_800a80fc_b(c, 0x800a80fc);
    if p { crate::prof::exit(c, 0x800a80fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a80fc_at(c: &mut Cpu, pc: u32) { f_800a80fc_b(c, pc) }
#[inline(always)]
fn f_800a80fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a80fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a80fc, pc); }
        match pc {
            0x800a80fc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0xfedd0000);
                c.cmpu(0, c.r[0], 0x4567);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                if !c.crbit(2) { pc = 0x800a8120; continue; }
                pc = 0x800a8118;
            }
            0x800a8118 => {
                c.r[31] = 0xff;
                pc = 0x800a8154; continue;
                pc = 0x800a8120;
            }
            0x800a8120 => {
                c.cmpu(0, c.r[0], 0x4568);
                if !c.crbit(2) { pc = 0x800a8130; continue; }
                pc = 0x800a8128;
            }
            0x800a8128 => {
                c.r[31] = 0xfe;
                pc = 0x800a8154; continue;
                pc = 0x800a8130;
            }
            0x800a8130 => {
                c.r[31] = c.r[3].rotate_left(8) & 0xff;
                c.r[3] = c.r[3].rotate_left(0) & 0xffffff;
                c.lr = 0x800a813c; f_800a7fe0(c);
                pc = 0x800a813c;
            }
            0x800a813c => {
                c.cmpu(0, c.r[31], 0x6);
                if c.crbit(0) { pc = 0x800a8148; continue; }
                pc = 0x800a8144;
            }
            0x800a8144 => {
                c.r[31] = 0x6;
                pc = 0x800a8148;
            }
            0x800a8148 => {
                c.r[0] = c.r[31].wrapping_mul(0x1e);
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x800a8154;
            }
            0x800a8154 => {
                c.lr = 0x800a8158; f_800a0954(c);
                pc = 0x800a8158;
            }
            0x800a8158 => {
                c.stb(c.r[3].wrapping_add(0x24), c.r[31]);
                c.r[3] = 0x1;
                c.lr = 0x800a8164; f_800a0cdc(c);
                pc = 0x800a8164;
            }
            0x800a8164 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a80fc: bad pc {:#x}", pc),
        }
    }
}

/// DVDCompareDiskID (0x800a8178, 248 bytes)
#[inline(never)]
pub fn f_800a8178(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a8178) { return; }
        if c.prof { crate::prof::enter(c, 0x800a8178); p = true; }
    }
    f_800a8178_b(c, 0x800a8178);
    if p { crate::prof::exit(c, 0x800a8178); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a8178_at(c: &mut Cpu, pc: u32) { f_800a8178_b(c, pc) }
#[inline(always)]
fn f_800a8178_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a8178;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a8178, pc); }
        match pc {
            0x800a8178 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[3]);
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a81cc; continue; }
                pc = 0x800a81a0;
            }
            0x800a81a0 => {
                c.r[0] = c.lbz(c.r[31]);
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a81cc; continue; }
                pc = 0x800a81ac;
            }
            0x800a81ac => {
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[5] = 0x4;
                c.lr = 0x800a81bc; f_800ca7bc(c);
                pc = 0x800a81bc;
            }
            0x800a81bc => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a81cc; continue; }
                pc = 0x800a81c4;
            }
            0x800a81c4 => {
                c.r[3] = 0x0;
                pc = 0x800a8258; continue;
                pc = 0x800a81cc;
            }
            0x800a81cc => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x4));
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a81fc; continue; }
                pc = 0x800a81d8;
            }
            0x800a81d8 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x4));
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a81fc; continue; }
                pc = 0x800a81e4;
            }
            0x800a81e4 => {
                c.r[3] = c.r[30].wrapping_add(0x4);
                c.r[4] = c.r[31].wrapping_add(0x4);
                c.r[5] = 0x2;
                c.lr = 0x800a81f4; f_800ca7bc(c);
                pc = 0x800a81f4;
            }
            0x800a81f4 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a8204; continue; }
                pc = 0x800a81fc;
            }
            0x800a81fc => {
                c.r[3] = 0x0;
                pc = 0x800a8258; continue;
                pc = 0x800a8204;
            }
            0x800a8204 => {
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x6));
                c.cmpu(0, c.r[3], 0xff);
                if c.crbit(2) { pc = 0x800a822c; continue; }
                pc = 0x800a8210;
            }
            0x800a8210 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x6));
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x800a822c; continue; }
                pc = 0x800a821c;
            }
            0x800a821c => {
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x800a822c; continue; }
                pc = 0x800a8224;
            }
            0x800a8224 => {
                c.r[3] = 0x0;
                pc = 0x800a8258; continue;
                pc = 0x800a822c;
            }
            0x800a822c => {
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x7));
                c.cmpu(0, c.r[3], 0xff);
                if c.crbit(2) { pc = 0x800a8254; continue; }
                pc = 0x800a8238;
            }
            0x800a8238 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x7));
                c.cmpu(0, c.r[0], 0xff);
                if c.crbit(2) { pc = 0x800a8254; continue; }
                pc = 0x800a8244;
            }
            0x800a8244 => {
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x800a8254; continue; }
                pc = 0x800a824c;
            }
            0x800a824c => {
                c.r[3] = 0x0;
                pc = 0x800a8258; continue;
                pc = 0x800a8254;
            }
            0x800a8254 => {
                c.r[3] = 0x1;
                pc = 0x800a8258;
            }
            0x800a8258 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a8178: bad pc {:#x}", pc),
        }
    }
}

/// ShowMessage (0x800a8270, 140 bytes)
#[inline(never)]
pub fn f_800a8270(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a8270) { return; }
        if c.prof { crate::prof::enter(c, 0x800a8270); p = true; }
    }
    f_800a8270_b(c, 0x800a8270);
    if p { crate::prof::exit(c, 0x800a8270); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a8270_at(c: &mut Cpu, pc: u32) { f_800a8270_b(c, pc) }
#[inline(always)]
fn f_800a8270_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a8270;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a8270, pc); }
        match pc {
            0x800a8270 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[3] = c.lwz(c.r[2].wrapping_add(0xffff8c38));
                c.r[0] = c.lwz(c.r[2].wrapping_add(0xffff8c3c));
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x800a8290; f_800aa430(c);
                pc = 0x800a8290;
            }
            0x800a8290 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a82b8; continue; }
                pc = 0x800a8298;
            }
            0x800a8298 => {
                c.lr = 0x800a829c; f_8009d820(c);
                pc = 0x800a829c;
            }
            0x800a829c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800a82b0; continue; }
                pc = 0x800a82a8;
            }
            0x800a82a8 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff81c0));
                pc = 0x800a82d0; continue;
                pc = 0x800a82b0;
            }
            0x800a82b0 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff81c4));
                pc = 0x800a82d0; continue;
                pc = 0x800a82b8;
            }
            0x800a82b8 => {
                c.lr = 0x800a82bc; f_800a106c(c);
                pc = 0x800a82bc;
            }
            0x800a82bc => {
                c.r[4] = 0x80270000;
                c.r[3] = c.r[3].rotate_left(2) & 0x3fc;
                c.r[0] = c.r[4].wrapping_add(0xfffff5f8);
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800a82d0;
            }
            0x800a82d0 => {
                c.r[6] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.stw(c.r[1].wrapping_add(0x8), c.r[6]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.lr = 0x800a82ec; f_8009cd38(c);
                pc = 0x800a82ec;
            }
            0x800a82ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a8270: bad pc {:#x}", pc),
        }
    }
}

/// DVDSetAutoFatalMessaging (0x800a82fc, 112 bytes)
#[inline(never)]
pub fn f_800a82fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a82fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a82fc); p = true; }
    }
    f_800a82fc_b(c, 0x800a82fc);
    if p { crate::prof::exit(c, 0x800a82fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a82fc_at(c: &mut Cpu, pc: u32) { f_800a82fc_b(c, pc) }
#[inline(always)]
fn f_800a82fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a82fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a82fc, pc); }
        match pc {
            0x800a82fc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800a8318; f_8009df3c(c);
                pc = 0x800a8318;
            }
            0x800a8318 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa010));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a832c; continue; }
                pc = 0x800a8324;
            }
            0x800a8324 => {
                c.r[31] = 0x1;
                pc = 0x800a8330; continue;
                pc = 0x800a832c;
            }
            0x800a832c => {
                c.r[31] = 0x0;
                pc = 0x800a8330;
            }
            0x800a8330 => {
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x800a8344; continue; }
                pc = 0x800a8338;
            }
            0x800a8338 => {
                c.r[4] = 0x800b0000;
                c.r[0] = c.r[4].wrapping_add(0xffff8270);
                pc = 0x800a8348; continue;
                pc = 0x800a8344;
            }
            0x800a8344 => {
                c.r[0] = 0x0;
                pc = 0x800a8348;
            }
            0x800a8348 => {
                c.stw(c.r[13].wrapping_add(0xffffa010), c.r[0]);
                c.lr = 0x800a8350; f_8009df64(c);
                pc = 0x800a8350;
            }
            0x800a8350 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a82fc: bad pc {:#x}", pc),
        }
    }
}

/// __DVDPrintFatalMessage (0x800a836c, 48 bytes)
#[inline(never)]
pub fn f_800a836c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a836c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a836c); p = true; }
    }
    f_800a836c_b(c, 0x800a836c);
    if p { crate::prof::exit(c, 0x800a836c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a836c_at(c: &mut Cpu, pc: u32) { f_800a836c_b(c, pc) }
#[inline(always)]
fn f_800a836c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a836c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a836c, pc); }
        match pc {
            0x800a836c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa010));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a838c; continue; }
                pc = 0x800a8384;
            }
            0x800a8384 => {
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a838c; (c.dispatch)(c, t); }
                pc = 0x800a838c;
            }
            0x800a838c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a836c: bad pc {:#x}", pc),
        }
    }
}

/// cb (0x800a839c, 216 bytes)
#[inline(never)]
pub fn f_800a839c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a839c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a839c); p = true; }
    }
    f_800a839c_b(c, 0x800a839c);
    if p { crate::prof::exit(c, 0x800a839c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a839c_at(c: &mut Cpu, pc: u32) { f_800a839c_b(c, pc) }
#[inline(always)]
fn f_800a839c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a839c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a839c, pc); }
        match pc {
            0x800a839c => {
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                if !c.crbit(1) { pc = 0x800a8430; continue; }
                pc = 0x800a83b8;
            }
            0x800a83b8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa018));
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800a83fc; continue; }
                pc = 0x800a83c4;
            }
            0x800a83c4 => {
                if !c.crbit(0) { pc = 0x800a8460; continue; }
                pc = 0x800a83c8;
            }
            0x800a83c8 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x800a83d4; continue; }
                pc = 0x800a83d0;
            }
            0x800a83d0 => {
                pc = 0x800a8460; continue;
                pc = 0x800a83d4;
            }
            0x800a83d4 => {
                c.r[0] = 0x1;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa01c));
                c.r[3] = 0x800b0000;
                c.stw(c.r[13].wrapping_add(0xffffa018), c.r[0]);
                c.r[7] = c.r[3].wrapping_add(0xffff839c);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[5] = 0x20;
                c.r[6] = 0x420;
                c.lr = 0x800a83f8; f_800a73b4(c);
                pc = 0x800a83f8;
            }
            0x800a83f8 => {
                pc = 0x800a8460; continue;
                pc = 0x800a83fc;
            }
            0x800a83fc => {
                c.r[0] = 0x2;
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa01c));
                c.stw(c.r[13].wrapping_add(0xffffa018), c.r[0]);
                c.r[3] = 0x800b0000;
                c.r[7] = c.r[3].wrapping_add(0xffff839c);
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x8));
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.lwz(c.r[6].wrapping_add(0x10));
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[6] = c.lwz(c.r[6].wrapping_add(0x4));
                c.r[5] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.lr = 0x800a842c; f_800a73b4(c);
                pc = 0x800a842c;
            }
            0x800a842c => {
                pc = 0x800a8460; continue;
                pc = 0x800a8430;
            }
            0x800a8430 => {
                c.cmps(0, c.r[3], 0xffffffff);
                if c.crbit(2) { pc = 0x800a8460; continue; }
                pc = 0x800a8438;
            }
            0x800a8438 => {
                c.cmps(0, c.r[3], 0xfffffffc);
                if !c.crbit(2) { pc = 0x800a8460; continue; }
                pc = 0x800a8440;
            }
            0x800a8440 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa018), c.r[0]);
                c.lr = 0x800a844c; f_800a76e4(c);
                pc = 0x800a844c;
            }
            0x800a844c => {
                c.r[3] = 0x800b0000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa020));
                c.r[5] = c.r[3].wrapping_add(0xffff839c);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.lr = 0x800a8460; f_800a7484(c);
                pc = 0x800a8460;
            }
            0x800a8460 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a839c: bad pc {:#x}", pc),
        }
    }
}

/// __fstLoad (0x800a8474, 360 bytes)
#[inline(never)]
pub fn f_800a8474(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a8474) { return; }
        if c.prof { crate::prof::enter(c, 0x800a8474); p = true; }
    }
    f_800a8474_b(c, 0x800a8474);
    if p { crate::prof::exit(c, 0x800a8474); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a8474_at(c: &mut Cpu, pc: u32) { f_800a8474_b(c, pc) }
#[inline(always)]
fn f_800a8474_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a8474;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a8474, pc); }
        match pc {
            0x800a8474 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80310000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x5c), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x2078);
                c.stw(c.r[1].wrapping_add(0x58), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x54), c.r[29]);
                c.lr = 0x800a8498; f_8009afb0(c);
                pc = 0x800a8498;
            }
            0x800a8498 => {
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffc418);
                c.r[4] = c.r[1].wrapping_add(0x2b);
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.r[3] = c.r[4].rotate_left(0) & 0xffffffe0;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.stw(c.r[13].wrapping_add(0xffffa020), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa01c), c.r[0]);
                c.lr = 0x800a84bc; f_800a76e4(c);
                pc = 0x800a84bc;
            }
            0x800a84bc => {
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa020));
                c.r[5] = 0x800b0000;
                c.r[3] = c.r[3].wrapping_add(0xffffc458);
                c.r[5] = c.r[5].wrapping_add(0xffff839c);
                c.lr = 0x800a84d4; f_800a7484(c);
                pc = 0x800a84d4;
            }
            0x800a84d4 => {
                c.lr = 0x800a84d8; f_800a7774(c);
                pc = 0x800a84d8;
            }
            0x800a84d8 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a84d4; continue; }
                pc = 0x800a84e0;
            }
            0x800a84e0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa01c));
                c.r[29] = 0x80000000;
                c.r[30] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x10));
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[5] = 0x20;
                c.stw(c.r[30].wrapping_add(0x38), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa01c));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xc));
                c.stw(c.r[30].wrapping_add(0x3c), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa020));
                c.lr = 0x800a8510; f_800054f4(c);
                pc = 0x800a8510;
            }
            0x800a8510 => {
                c.r[3] = c.r[13].wrapping_add(0xffff81c8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800a851c; f_8009c2e0(c);
                pc = 0x800a851c;
            }
            0x800a851c => {
                c.r[4] = c.lbz(c.r[29]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.lbz(c.r[29].wrapping_add(0x1));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[6] = c.lbz(c.r[29].wrapping_add(0x2));
                c.r[7] = c.lbz(c.r[29].wrapping_add(0x3));
                c.r[4] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[5] = c.r[5] as u8 as i8 as i32 as u32;
                c.r[6] = c.r[6] as u8 as i8 as i32 as u32;
                c.r[7] = c.r[7] as u8 as i8 as i32 as u32;
                c.lr = 0x800a8548; f_8009c2e0(c);
                pc = 0x800a8548;
            }
            0x800a8548 => {
                c.r[4] = c.lbz(c.r[29].wrapping_add(0x4));
                c.r[3] = c.r[31].wrapping_add(0x1c);
                c.r[5] = c.lbz(c.r[29].wrapping_add(0x5));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[5] = c.r[5] as u8 as i8 as i32 as u32;
                c.lr = 0x800a8564; f_8009c2e0(c);
                pc = 0x800a8564;
            }
            0x800a8564 => {
                c.r[4] = c.lbz(c.r[29].wrapping_add(0x6));
                c.r[3] = c.r[31].wrapping_add(0x34);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800a8574; f_8009c2e0(c);
                pc = 0x800a8574;
            }
            0x800a8574 => {
                c.r[4] = c.lbz(c.r[29].wrapping_add(0x7));
                c.r[3] = c.r[31].wrapping_add(0x48);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800a8584; f_8009c2e0(c);
                pc = 0x800a8584;
            }
            0x800a8584 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a8598; continue; }
                pc = 0x800a8590;
            }
            0x800a8590 => {
                c.r[4] = c.r[13].wrapping_add(0xffff81cc);
                pc = 0x800a859c; continue;
                pc = 0x800a8598;
            }
            0x800a8598 => {
                c.r[4] = c.r[13].wrapping_add(0xffff81d0);
                pc = 0x800a859c;
            }
            0x800a859c => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[3] = c.r[31].wrapping_add(0x5c);
                c.lr = 0x800a85a8; f_8009c2e0(c);
                pc = 0x800a85a8;
            }
            0x800a85a8 => {
                c.r[3] = c.r[13].wrapping_add(0xffff81c8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800a85b4; f_8009c2e0(c);
                pc = 0x800a85b4;
            }
            0x800a85b4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa01c));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x800a85c0; f_8009afc0(c);
                pc = 0x800a85c0;
            }
            0x800a85c0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x5c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x58));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[1] = c.r[1].wrapping_add(0x60);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a8474: bad pc {:#x}", pc),
        }
    }
}

/// __VIRetraceHandler (0x800a85dc, 560 bytes)
#[inline(never)]
pub fn f_800a85dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a85dc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a85dc); p = true; }
    }
    f_800a85dc_b(c, 0x800a85dc);
    if p { crate::prof::exit(c, 0x800a85dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a85dc_at(c: &mut Cpu, pc: u32) { f_800a85dc_b(c, pc) }
#[inline(never)]
fn f_800a85dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a85dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a85dc, pc); }
        match pc {
            0x800a85dc => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0xcc000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[5] = c.r[3].wrapping_add(0x2000);
                c.r[3] = 0x80400000;
                { let ea = c.r[1].wrapping_add(0xfffffd08); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[7] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x2e4); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.r[31] = c.r[3].wrapping_add(0xffffc488);
                { let ea = c.r[5].wrapping_add(0x30); c.r[6] = c.lhz(ea); c.r[5] = ea; }
                c.r[0] = c.r[6].rotate_left(0) & 0x8000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a861c; continue; }
                pc = 0x800a8610;
            }
            0x800a8610 => {
                c.r[0] = c.r[6].rotate_left(0) & 0xffff7fff;
                c.sth(c.r[5], c.r[0]);
                c.r[7] = c.r[7] | 0x1;
                pc = 0x800a861c;
            }
            0x800a861c => {
                c.r[3] = 0xcc000000;
                { let ea = c.r[3].wrapping_add(0x2034); c.r[4] = c.lhz(ea); c.r[3] = ea; }
                c.r[0] = c.r[4].rotate_left(0) & 0x8000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a8638; continue; }
                pc = 0x800a862c;
            }
            0x800a862c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff7fff;
                c.sth(c.r[3], c.r[0]);
                c.r[7] = c.r[7] | 0x2;
                pc = 0x800a8638;
            }
            0x800a8638 => {
                c.r[3] = 0xcc000000;
                { let ea = c.r[3].wrapping_add(0x2038); c.r[4] = c.lhz(ea); c.r[3] = ea; }
                c.r[0] = c.r[4].rotate_left(0) & 0x8000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a8654; continue; }
                pc = 0x800a8648;
            }
            0x800a8648 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff7fff;
                c.sth(c.r[3], c.r[0]);
                c.r[7] = c.r[7] | 0x4;
                pc = 0x800a8654;
            }
            0x800a8654 => {
                c.r[3] = 0xcc000000;
                { let ea = c.r[3].wrapping_add(0x203c); c.r[4] = c.lhz(ea); c.r[3] = ea; }
                c.r[0] = c.r[4].rotate_left(0) & 0x8000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a8670; continue; }
                pc = 0x800a8664;
            }
            0x800a8664 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff7fff;
                c.sth(c.r[3], c.r[0]);
                c.r[7] = c.r[7] | 0x8;
                pc = 0x800a8670;
            }
            0x800a8670 => {
                c.r[0] = c.r[7].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a8680; continue; }
                pc = 0x800a8678;
            }
            0x800a8678 => {
                c.r[0] = c.r[7].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a868c; continue; }
                pc = 0x800a8680;
            }
            0x800a8680 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a8688; f_8009bb68(c);
                pc = 0x800a8688;
            }
            0x800a8688 => {
                pc = 0x800a87f8; continue;
                pc = 0x800a868c;
            }
            0x800a868c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa02c));
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffffa02c), c.r[0]);
                c.lr = 0x800a86a0; f_8009bd60(c);
                pc = 0x800a86a0;
            }
            0x800a86a0 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x800a86a8; f_8009bb68(c);
                pc = 0x800a86a8;
            }
            0x800a86a8 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa03c));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a86c0; continue; }
                pc = 0x800a86b4;
            }
            0x800a86b4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa02c));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a86c0; (c.dispatch)(c, t); }
                pc = 0x800a86c0;
            }
            0x800a86c0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa030));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a87bc; continue; }
                pc = 0x800a86cc;
            }
            0x800a86cc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa058));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800a86e4; continue; }
                pc = 0x800a86d8;
            }
            0x800a86d8 => {
                c.lr = 0x800a86dc; f_800aa288(c);
                pc = 0x800a86dc;
            }
            0x800a86dc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a87a4; continue; }
                pc = 0x800a86e4;
            }
            0x800a86e4 => {
                c.r[3] = 0xcc000000;
                c.r[29] = c.r[3].wrapping_add(0x2000);
                pc = 0x800a8764; continue;
                pc = 0x800a86f0;
            }
            0x800a86f0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa060));
                c.r[5] = 0x20;
                c.r[27] = c.lwz(c.r[13].wrapping_add(0xffffa064));
                c.r[4] = c.r[27].wrapping_add(0x0);
                c.lr = 0x800a8704; f_800c4c74(c);
                pc = 0x800a8704;
            }
            0x800a8704 => {
                c.r[4] = c.r[4].leading_zeros();
                c.cmps(0, c.r[4], 0x20);
                c.r[0] = 0xffffffff;
                c.r[0] = c.r[27] & c.r[0];
                if !c.crbit(0) { pc = 0x800a871c; continue; }
                pc = 0x800a8718;
            }
            0x800a8718 => {
                pc = 0x800a8724; continue;
                pc = 0x800a871c;
            }
            0x800a871c => {
                c.r[3] = c.r[0].leading_zeros();
                c.r[4] = c.r[3].wrapping_add(0x20);
                pc = 0x800a8724;
            }
            0x800a8724 => {
                c.r[6] = c.r[4].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[31].wrapping_add(c.r[6]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x78));
                c.r[5] = c.addc(!c.r[4], 0x3f, 1);
                c.r[3] = 0x0;
                c.sth(c.r[29].wrapping_add(c.r[6]), c.r[0]);
                c.r[4] = 0x1;
                c.lr = 0x800a8744; f_800c4c50(c);
                pc = 0x800a8744;
            }
            0x800a8744 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa060));
                c.r[5] = !(c.r[3] | c.r[3]);
                c.r[4] = !(c.r[4] | c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa064));
                c.r[0] = c.r[0] & c.r[5];
                c.r[3] = c.r[3] & c.r[4];
                c.stw(c.r[13].wrapping_add(0xffffa064), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa060), c.r[0]);
                pc = 0x800a8764;
            }
            0x800a8764 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa060));
                c.r[4] = 0x0;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa064));
                c.r[0] = c.r[0] ^ c.r[4];
                c.r[3] = c.r[3] ^ c.r[4];
                c.r[0] = c.r[3] | c.r[0];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a86f0; continue; }
                pc = 0x800a8780;
            }
            0x800a8780 => {
                c.stw(c.r[13].wrapping_add(0xffffa058), c.r[4]);
                c.r[4] = 0x1;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa070));
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x144));
                c.stw(c.r[13].wrapping_add(0xffffa068), c.r[3]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x118));
                c.stw(c.r[13].wrapping_add(0xffffa06c), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa074), c.r[0]);
                pc = 0x800a87a8; continue;
                pc = 0x800a87a4;
            }
            0x800a87a4 => {
                c.r[4] = 0x0;
                pc = 0x800a87a8;
            }
            0x800a87a8 => {
                c.cmps(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800a87bc; continue; }
                pc = 0x800a87b0;
            }
            0x800a87b0 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa030), c.r[0]);
                c.lr = 0x800a87bc; f_800d104c(c);
                pc = 0x800a87bc;
            }
            0x800a87bc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa040));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a87e0; continue; }
                pc = 0x800a87c8;
            }
            0x800a87c8 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x800a87d0; f_8009bd60(c);
                pc = 0x800a87d0;
            }
            0x800a87d0 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa040));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa02c));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a87e0; (c.dispatch)(c, t); }
                pc = 0x800a87e0;
            }
            0x800a87e0 => {
                c.r[3] = c.r[13].wrapping_add(0xffffa034);
                c.lr = 0x800a87e8; f_800a2478(c);
                pc = 0x800a87e8;
            }
            0x800a87e8 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x800a87f0; f_8009bd60(c);
                pc = 0x800a87f0;
            }
            0x800a87f0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a87f8; f_8009bb68(c);
                pc = 0x800a87f8;
            }
            0x800a87f8 => {
                { let mut ea = c.r[1].wrapping_add(0x2e4); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2fc));
                c.r[1] = c.r[1].wrapping_add(0x2f8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a85dc: bad pc {:#x}", pc),
        }
    }
}

/// VISetPreRetraceCallback (0x800a880c, 68 bytes)
#[inline(never)]
pub fn f_800a880c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a880c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a880c); p = true; }
    }
    f_800a880c_b(c, 0x800a880c);
    if p { crate::prof::exit(c, 0x800a880c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a880c_at(c: &mut Cpu, pc: u32) { f_800a880c_b(c, pc) }
#[inline(always)]
fn f_800a880c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a880c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a880c, pc); }
        match pc {
            0x800a880c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa03c));
                c.lr = 0x800a882c; f_8009df3c(c);
                pc = 0x800a882c;
            }
            0x800a882c => {
                c.stw(c.r[13].wrapping_add(0xffffa03c), c.r[30]);
                c.lr = 0x800a8834; f_8009df64(c);
                pc = 0x800a8834;
            }
            0x800a8834 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a880c: bad pc {:#x}", pc),
        }
    }
}

/// VISetPostRetraceCallback (0x800a8850, 68 bytes)
#[inline(never)]
pub fn f_800a8850(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a8850) { return; }
        if c.prof { crate::prof::enter(c, 0x800a8850); p = true; }
    }
    f_800a8850_b(c, 0x800a8850);
    if p { crate::prof::exit(c, 0x800a8850); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a8850_at(c: &mut Cpu, pc: u32) { f_800a8850_b(c, pc) }
#[inline(always)]
fn f_800a8850_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a8850;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a8850, pc); }
        match pc {
            0x800a8850 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa040));
                c.lr = 0x800a8870; f_8009df3c(c);
                pc = 0x800a8870;
            }
            0x800a8870 => {
                c.stw(c.r[13].wrapping_add(0xffffa040), c.r[30]);
                c.lr = 0x800a8878; f_8009df64(c);
                pc = 0x800a8878;
            }
            0x800a8878 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a8850: bad pc {:#x}", pc),
        }
    }
}

/// getTiming (0x800a8894, 160 bytes)
#[inline(never)]
pub fn f_800a8894(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a8894) { return; }
        if c.prof { crate::prof::enter(c, 0x800a8894); p = true; }
    }
    f_800a8894_b(c, 0x800a8894);
    if p { crate::prof::exit(c, 0x800a8894); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a8894_at(c: &mut Cpu, pc: u32) { f_800a8894_b(c, pc) }
#[inline(always)]
fn f_800a8894_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a8894;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a8894, pc); }
        match pc {
            0x800a8894 => {
                c.cmpu(0, c.r[3], 0x1a);
                pc = 0x800a8898;
            }
            0x800a8898 => {
                c.r[4] = 0x80310000;
                pc = 0x800a889c;
            }
            0x800a889c => {
                c.r[5] = c.r[4].wrapping_add(0x20e8);
                pc = 0x800a88a0;
            }
            0x800a88a0 => {
                if c.crbit(1) { pc = 0x800a892c; continue; }
                pc = 0x800a88a4;
            }
            0x800a88a4 => {
                c.r[4] = 0x80310000;
                pc = 0x800a88a8;
            }
            0x800a88a8 => {
                c.r[4] = c.r[4].wrapping_add(0x22dc);
                pc = 0x800a88ac;
            }
            0x800a88ac => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                pc = 0x800a88b0;
            }
            0x800a88b0 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                pc = 0x800a88b4;
            }
            0x800a88b4 => {
                c.mtspr(9, c.r[0]);
                pc = 0x800a88b8;
            }
            0x800a88b8 => {
                if true { let t = c.ctr; if (0x800a8894..0x800a8934).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x800a88bc;
            }
            0x800a88bc => {
                c.r[3] = c.r[5].wrapping_add(0x44);
                pc = 0x800a88c0;
            }
            0x800a88c0 => {
                if true { return; }
                pc = 0x800a88c4;
            }
            0x800a88c4 => {
                c.r[3] = c.r[5].wrapping_add(0x6a);
                pc = 0x800a88c8;
            }
            0x800a88c8 => {
                if true { return; }
                pc = 0x800a88cc;
            }
            0x800a88cc => {
                c.r[3] = c.r[5].wrapping_add(0x90);
                pc = 0x800a88d0;
            }
            0x800a88d0 => {
                if true { return; }
                pc = 0x800a88d4;
            }
            0x800a88d4 => {
                c.r[3] = c.r[5].wrapping_add(0xb6);
                pc = 0x800a88d8;
            }
            0x800a88d8 => {
                if true { return; }
                pc = 0x800a88dc;
            }
            0x800a88dc => {
                c.r[3] = c.r[5].wrapping_add(0x44);
                pc = 0x800a88e0;
            }
            0x800a88e0 => {
                if true { return; }
                pc = 0x800a88e4;
            }
            0x800a88e4 => {
                c.r[3] = c.r[5].wrapping_add(0x6a);
                pc = 0x800a88e8;
            }
            0x800a88e8 => {
                if true { return; }
                pc = 0x800a88ec;
            }
            0x800a88ec => {
                c.r[3] = c.r[5].wrapping_add(0xdc);
                pc = 0x800a88f0;
            }
            0x800a88f0 => {
                if true { return; }
                pc = 0x800a88f4;
            }
            0x800a88f4 => {
                c.r[3] = c.r[5].wrapping_add(0x102);
                pc = 0x800a88f8;
            }
            0x800a88f8 => {
                if true { return; }
                pc = 0x800a88fc;
            }
            0x800a88fc => {
                c.r[3] = c.r[5].wrapping_add(0x128);
                pc = 0x800a8900;
            }
            0x800a8900 => {
                if true { return; }
                pc = 0x800a8904;
            }
            0x800a8904 => {
                c.r[3] = c.r[5].wrapping_add(0x14e);
                pc = 0x800a8908;
            }
            0x800a8908 => {
                if true { return; }
                pc = 0x800a890c;
            }
            0x800a890c => {
                c.r[3] = c.r[5].wrapping_add(0x90);
                pc = 0x800a8910;
            }
            0x800a8910 => {
                if true { return; }
                pc = 0x800a8914;
            }
            0x800a8914 => {
                c.r[3] = c.r[5].wrapping_add(0xb6);
                pc = 0x800a8918;
            }
            0x800a8918 => {
                if true { return; }
                pc = 0x800a891c;
            }
            0x800a891c => {
                c.r[3] = c.r[5].wrapping_add(0x174);
                pc = 0x800a8920;
            }
            0x800a8920 => {
                if true { return; }
                pc = 0x800a8924;
            }
            0x800a8924 => {
                c.r[3] = c.r[5].wrapping_add(0x19a);
                pc = 0x800a8928;
            }
            0x800a8928 => {
                if true { return; }
                pc = 0x800a892c;
            }
            0x800a892c => {
                c.r[3] = 0x0;
                pc = 0x800a8930;
            }
            0x800a8930 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800a8894: bad pc {:#x}", pc),
        }
    }
}

/// __VIInit (0x800a8934, 512 bytes)
#[inline(never)]
pub fn f_800a8934(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a8934) { return; }
        if c.prof { crate::prof::enter(c, 0x800a8934); p = true; }
    }
    f_800a8934_b(c, 0x800a8934);
    if p { crate::prof::exit(c, 0x800a8934); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a8934_at(c: &mut Cpu, pc: u32) { f_800a8934_b(c, pc) }
#[inline(always)]
fn f_800a8934_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a8934;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a8934, pc); }
        match pc {
            0x800a8934 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.r[31] = c.r[3].rotate_left(30) & 0x3fffffff;
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0xcc), c.r[31]);
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[30] = c.r[29].rotate_left(0) & 0x2;
                c.lr = 0x800a8968; f_800a8894(c);
                pc = 0x800a8968;
            }
            0x800a8968 => {
                c.r[4] = 0xcc000000;
                c.r[0] = 0x2;
                c.r[5] = c.r[4].wrapping_add(0x2000);
                { let ea = c.r[5].wrapping_add(0x2); c.sth(ea, c.r[0]); c.r[5] = ea; }
                c.r[0] = 0x0;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x3e8);
                if !c.crbit(0) { pc = 0x800a89a8; continue; }
                pc = 0x800a898c;
            }
            0x800a898c => {
                pc = 0x800a899c; continue;
                pc = 0x800a8990;
            }
            0x800a8990 => {
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[0] = c.r[4].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[0]);
                pc = 0x800a899c;
            }
            0x800a899c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x3e8);
                if c.crbit(0) { pc = 0x800a8990; continue; }
                pc = 0x800a89a8;
            }
            0x800a89a8 => {
                c.r[0] = 0x0;
                c.sth(c.r[5], c.r[0]);
                c.r[12] = 0xcc000000;
                c.r[8] = 0x2828;
                c.r[6] = c.lhz(c.r[3].wrapping_add(0x1a));
                c.r[7] = 0x1;
                c.cmps(0, c.r[29], 0x2);
                c.sth(c.r[12].wrapping_add(0x2006), c.r[6]);
                c.r[6] = 0x1001;
                c.r[4] = c.r[12].wrapping_add(0x2000);
                c.r[9] = c.lbz(c.r[3].wrapping_add(0x1d));
                c.r[10] = c.lbz(c.r[3].wrapping_add(0x1e));
                c.r[10] = (c.r[9].rotate_left(8) & 0xff00) | (c.r[10] & 0xffff00ff);
                c.sth(c.r[12].wrapping_add(0x2004), c.r[10]);
                c.r[9] = c.lbz(c.r[3].wrapping_add(0x1f));
                c.r[10] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.r[9] = c.r[9].rotate_left(7) & 0xffffff80;
                c.r[9] = c.r[10] | c.r[9];
                c.sth(c.r[12].wrapping_add(0x200a), c.r[9]);
                c.r[9] = c.lhz(c.r[3].wrapping_add(0x20));
                c.r[9] = c.r[9].rotate_left(1) & 0xfffe;
                c.sth(c.r[12].wrapping_add(0x2008), c.r[9]);
                c.r[9] = c.lhz(c.r[3].wrapping_add(0x2));
                c.r[11] = c.lbz(c.r[3]);
                c.r[9] = c.r[9].rotate_left(1) & 0xfffffffe;
                c.r[10] = c.lhz(c.r[3].wrapping_add(0x4));
                c.r[9] = c.r[9].wrapping_add(0xfffffffe);
                c.sth(c.r[12].wrapping_add(0x2000), c.r[11]);
                c.r[9] = c.r[10].wrapping_add(c.r[9]);
                c.sth(c.r[12].wrapping_add(0x200e), c.r[9]);
                c.r[9] = c.lhz(c.r[3].wrapping_add(0x8));
                c.r[9] = c.r[9].wrapping_add(0x2);
                c.sth(c.r[12].wrapping_add(0x200c), c.r[9]);
                c.r[9] = c.lhz(c.r[3].wrapping_add(0x2));
                c.r[10] = c.lhz(c.r[3].wrapping_add(0x6));
                c.r[9] = c.r[9].rotate_left(1) & 0xfffffffe;
                c.r[9] = c.r[9].wrapping_add(0xfffffffe);
                c.r[9] = c.r[10].wrapping_add(c.r[9]);
                c.sth(c.r[12].wrapping_add(0x2012), c.r[9]);
                c.r[9] = c.lhz(c.r[3].wrapping_add(0xa));
                c.r[9] = c.r[9].wrapping_add(0x2);
                c.sth(c.r[12].wrapping_add(0x2010), c.r[9]);
                c.r[9] = c.lhz(c.r[3].wrapping_add(0x10));
                c.r[10] = c.lbz(c.r[3].wrapping_add(0xc));
                c.r[9] = c.r[9].rotate_left(5) & 0xffffffe0;
                c.r[9] = c.r[10] | c.r[9];
                c.sth(c.r[12].wrapping_add(0x2016), c.r[9]);
                c.r[9] = c.lhz(c.r[3].wrapping_add(0x14));
                c.r[10] = c.lbz(c.r[3].wrapping_add(0xe));
                c.r[9] = c.r[9].rotate_left(5) & 0xffffffe0;
                c.r[9] = c.r[10] | c.r[9];
                c.sth(c.r[12].wrapping_add(0x2014), c.r[9]);
                c.r[9] = c.lhz(c.r[3].wrapping_add(0x12));
                c.r[10] = c.lbz(c.r[3].wrapping_add(0xd));
                c.r[9] = c.r[9].rotate_left(5) & 0xffffffe0;
                c.r[9] = c.r[10] | c.r[9];
                c.sth(c.r[12].wrapping_add(0x201a), c.r[9]);
                c.r[9] = c.lhz(c.r[3].wrapping_add(0x16));
                c.r[10] = c.lbz(c.r[3].wrapping_add(0xf));
                c.r[9] = c.r[9].rotate_left(5) & 0xffffffe0;
                c.r[9] = c.r[10] | c.r[9];
                c.sth(c.r[12].wrapping_add(0x2018), c.r[9]);
                c.sth(c.r[12].wrapping_add(0x2048), c.r[8]);
                c.sth(c.r[12].wrapping_add(0x2036), c.r[7]);
                c.sth(c.r[12].wrapping_add(0x2034), c.r[6]);
                c.r[6] = c.lhz(c.r[3].wrapping_add(0x18));
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1a));
                { let v = c.r[6] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[6] = (v >> 1) as u32; }
                c.r[3] = c.r[3].wrapping_add(0x1);
                { let ci = c.ca(); c.r[6] = c.addc(c.r[6], 0, ci); }
                c.sth(c.r[12].wrapping_add(0x2032), c.r[3]);
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.r[3] = c.r[6] | 0x1000;
                c.sth(c.r[12].wrapping_add(0x2030), c.r[3]);
                if c.crbit(2) { pc = 0x800a8b00; continue; }
                pc = 0x800a8ad4;
            }
            0x800a8ad4 => {
                c.cmps(0, c.r[29], 0x3);
                if c.crbit(2) { pc = 0x800a8b00; continue; }
                pc = 0x800a8adc;
            }
            0x800a8adc => {
                c.cmps(0, c.r[29], 0x1a);
                if c.crbit(2) { pc = 0x800a8b00; continue; }
                pc = 0x800a8ae4;
            }
            0x800a8ae4 => {
                c.r[3] = c.r[30].rotate_left(2) & 0xfffffffc;
                c.r[6] = c.r[3] | 0x1;
                c.r[3] = c.r[31].rotate_left(8) & 0xffffff00;
                c.r[3] = c.r[6] | c.r[3];
                c.sth(c.r[5], c.r[3]);
                c.sth(c.r[4].wrapping_add(0x6c), c.r[0]);
                pc = 0x800a8b18; continue;
                pc = 0x800a8b00;
            }
            0x800a8b00 => {
                c.r[0] = c.r[31].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[0] | 0x5;
                c.sth(c.r[5], c.r[0]);
                c.r[3] = 0xcc000000;
                c.r[0] = 0x1;
                c.sth(c.r[3].wrapping_add(0x206c), c.r[0]);
                pc = 0x800a8b18;
            }
            0x800a8b18 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a8934: bad pc {:#x}", pc),
        }
    }
}

/// VIInit (0x800a8b34, 1200 bytes)
#[inline(never)]
pub fn f_800a8b34(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a8b34) { return; }
        if c.prof { crate::prof::enter(c, 0x800a8b34); p = true; }
    }
    f_800a8b34_b(c, 0x800a8b34);
    if p { crate::prof::exit(c, 0x800a8b34); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a8b34_at(c: &mut Cpu, pc: u32) { f_800a8b34_b(c, pc) }
#[inline(never)]
fn f_800a8b34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a8b34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a8b34, pc); }
        match pc {
            0x800a8b34 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80310000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = 0x80400000;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0xffffc488);
                c.stw(c.r[1].wrapping_add(0xc), c.r[29]);
                c.r[29] = c.r[4].wrapping_add(0x20e8);
                c.stw(c.r[1].wrapping_add(0x8), c.r[28]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa028));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a8fc4; continue; }
                pc = 0x800a8b6c;
            }
            0x800a8b6c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81d8));
                c.lr = 0x800a8b74; f_8009a250(c);
                pc = 0x800a8b74;
            }
            0x800a8b74 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa028), c.r[0]);
                c.r[3] = 0xcc000000;
                c.r[28] = c.r[3].wrapping_add(0x2000);
                c.stw(c.r[13].wrapping_add(0xffffa044), c.r[0]);
                { let ea = c.r[28].wrapping_add(0x2); c.r[0] = c.lhz(ea); c.r[28] = ea; }
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a8b9c; continue; }
                pc = 0x800a8b94;
            }
            0x800a8b94 => {
                c.r[3] = 0x0;
                c.lr = 0x800a8b9c; f_800a8934(c);
                pc = 0x800a8b9c;
            }
            0x800a8b9c => {
                c.r[31] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa02c), c.r[31]);
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x2000);
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[31]);
                c.r[0] = 0x280;
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[31]);
                c.stw(c.r[13].wrapping_add(0xffffa064), c.r[31]);
                c.stw(c.r[13].wrapping_add(0xffffa060), c.r[31]);
                c.stw(c.r[13].wrapping_add(0xffffa04c), c.r[31]);
                c.stw(c.r[13].wrapping_add(0xffffa058), c.r[31]);
                c.stw(c.r[13].wrapping_add(0xffffa030), c.r[31]);
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1c2));
                c.r[6] = c.lhz(c.r[29].wrapping_add(0x1c0));
                c.r[5] = c.r[5].rotate_left(10) & 0xfc00;
                c.r[5] = c.r[6] | c.r[5];
                c.sth(c.r[3].wrapping_add(0x4e), c.r[5]);
                c.r[6] = c.lhz(c.r[29].wrapping_add(0x1c2));
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1c4));
                { let v = c.r[6] as i32; c.set_ca(v < 0 && (v as u32) << 26 != 0); c.r[6] = (v >> 6) as u32; }
                c.r[5] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[5] = c.r[6] | c.r[5];
                c.sth(c.r[3].wrapping_add(0x4c), c.r[5]);
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1c8));
                c.r[6] = c.lhz(c.r[29].wrapping_add(0x1c6));
                c.r[5] = c.r[5].rotate_left(10) & 0xfc00;
                c.r[5] = c.r[6] | c.r[5];
                c.sth(c.r[3].wrapping_add(0x52), c.r[5]);
                c.r[6] = c.lhz(c.r[29].wrapping_add(0x1c8));
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1ca));
                { let v = c.r[6] as i32; c.set_ca(v < 0 && (v as u32) << 26 != 0); c.r[6] = (v >> 6) as u32; }
                c.r[5] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[5] = c.r[6] | c.r[5];
                c.sth(c.r[3].wrapping_add(0x50), c.r[5]);
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1ce));
                c.r[6] = c.lhz(c.r[29].wrapping_add(0x1cc));
                c.r[5] = c.r[5].rotate_left(10) & 0xfc00;
                c.r[5] = c.r[6] | c.r[5];
                c.sth(c.r[3].wrapping_add(0x56), c.r[5]);
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1ce));
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x1d0));
                { let v = c.r[5] as i32; c.set_ca(v < 0 && (v as u32) << 26 != 0); c.r[5] = (v >> 6) as u32; }
                c.r[4] = c.r[4].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.r[5] | c.r[4];
                c.sth(c.r[3].wrapping_add(0x54), c.r[4]);
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x1d4));
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1d2));
                c.r[4] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[5] | c.r[4];
                c.sth(c.r[3].wrapping_add(0x5a), c.r[4]);
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x1d8));
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1d6));
                c.r[4] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[5] | c.r[4];
                c.sth(c.r[3].wrapping_add(0x58), c.r[4]);
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x1dc));
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1da));
                c.r[4] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[5] | c.r[4];
                c.sth(c.r[3].wrapping_add(0x5e), c.r[4]);
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x1e0));
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1de));
                c.r[4] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[5] | c.r[4];
                c.sth(c.r[3].wrapping_add(0x5c), c.r[4]);
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x1e4));
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1e2));
                c.r[4] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[5] | c.r[4];
                c.sth(c.r[3].wrapping_add(0x62), c.r[4]);
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x1e8));
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1e6));
                c.r[4] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[5] | c.r[4];
                c.sth(c.r[3].wrapping_add(0x60), c.r[4]);
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x1ec));
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1ea));
                c.r[4] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[5] | c.r[4];
                c.sth(c.r[3].wrapping_add(0x66), c.r[4]);
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x1f0));
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x1ee));
                c.r[4] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[5] | c.r[4];
                c.sth(c.r[3].wrapping_add(0x64), c.r[4]);
                c.sth(c.r[3].wrapping_add(0x70), c.r[0]);
                c.lr = 0x800a8cf8; f_800a08f8(c);
                pc = 0x800a8cf8;
            }
            0x800a8cf8 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x10));
                c.r[3] = 0x0;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.sth(c.r[13].wrapping_add(0xffffa04a), c.r[31]);
                c.sth(c.r[13].wrapping_add(0xffffa048), c.r[0]);
                c.lr = 0x800a8d10; f_800a0cb8(c);
                pc = 0x800a8d10;
            }
            0x800a8d10 => {
                c.r[29] = c.lhz(c.r[28]);
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xcc));
                c.r[4] = c.r[30].wrapping_add(0x114);
                c.r[0] = c.r[29].rotate_left(30) & 0x1;
                c.stw(c.r[30].wrapping_add(0x114), c.r[0]);
                c.r[0] = c.r[29].rotate_left(24) & 0x3;
                c.cmpu(0, c.r[3], 0x1);
                c.stw(c.r[30].wrapping_add(0x118), c.r[0]);
                c.r[28] = c.r[30].wrapping_add(0x118);
                if !c.crbit(2) { pc = 0x800a8d50; continue; }
                pc = 0x800a8d3c;
            }
            0x800a8d3c => {
                c.r[0] = c.lwz(c.r[28]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a8d50; continue; }
                pc = 0x800a8d48;
            }
            0x800a8d48 => {
                c.r[0] = 0x5;
                c.stw(c.r[28], c.r[0]);
                pc = 0x800a8d50;
            }
            0x800a8d50 => {
                c.r[3] = c.lwz(c.r[28]);
                c.cmpu(0, c.r[3], 0x3);
                if !c.crbit(2) { pc = 0x800a8d60; continue; }
                pc = 0x800a8d5c;
            }
            0x800a8d5c => {
                c.r[3] = 0x0;
                pc = 0x800a8d60;
            }
            0x800a8d60 => {
                c.r[0] = c.lwz(c.r[4]);
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x800a8d70; f_800a8894(c);
                pc = 0x800a8d70;
            }
            0x800a8d70 => {
                c.stw(c.r[30].wrapping_add(0x144), c.r[3]);
                c.r[4] = c.r[30].wrapping_add(0x144);
                c.r[0] = 0x280;
                c.sth(c.r[30].wrapping_add(0x2), c.r[29]);
                c.r[8] = 0x0;
                c.r[6] = c.r[30].wrapping_add(0xf6);
                c.r[4] = c.lwz(c.r[4]);
                c.r[3] = c.r[30].wrapping_add(0xf2);
                c.stw(c.r[13].wrapping_add(0xffffa068), c.r[4]);
                c.r[4] = c.lwz(c.r[28]);
                c.stw(c.r[13].wrapping_add(0xffffa06c), c.r[4]);
                c.sth(c.r[30].wrapping_add(0xf4), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa068));
                { let ea = c.r[4].wrapping_add(0x2); c.r[0] = c.lhz(ea); c.r[4] = ea; }
                c.r[0] = c.r[0].rotate_left(1) & 0xfffe;
                c.sth(c.r[30].wrapping_add(0xf6), c.r[0]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0xf4));
                c.r[0] = c.addc(!c.r[0], 0x2d0, 1);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.sth(c.r[30].wrapping_add(0xf0), c.r[0]);
                c.sth(c.r[30].wrapping_add(0xf2), c.r[8]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0xf4));
                c.r[7] = c.lha(c.r[30].wrapping_add(0xf0));
                c.r[5] = c.lha(c.r[13].wrapping_add(0xffffa048));
                c.r[0] = c.addc(!c.r[0], 0x2d0, 1);
                c.r[9] = c.lhz(c.r[4]);
                c.r[5] = c.r[7].wrapping_add(c.r[5]);
                c.cmps(0, c.r[5], c.r[0]);
                if !c.crbit(1) { pc = 0x800a8dec; continue; }
                pc = 0x800a8de8;
            }
            0x800a8de8 => {
                pc = 0x800a8e00; continue;
                pc = 0x800a8dec;
            }
            0x800a8dec => {
                c.cmps(0, c.r[5], 0x0);
                if !c.crbit(0) { pc = 0x800a8df8; continue; }
                pc = 0x800a8df4;
            }
            0x800a8df4 => {
                pc = 0x800a8dfc; continue;
                pc = 0x800a8df8;
            }
            0x800a8df8 => {
                c.r[8] = c.r[5] | c.r[5];
                pc = 0x800a8dfc;
            }
            0x800a8dfc => {
                c.r[0] = c.r[8] | c.r[8];
                pc = 0x800a8e00;
            }
            0x800a8e00 => {
                c.sth(c.r[30].wrapping_add(0xf8), c.r[0]);
                c.r[8] = c.r[30].wrapping_add(0x110);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x110));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a8e1c; continue; }
                pc = 0x800a8e14;
            }
            0x800a8e14 => {
                c.r[11] = 0x2;
                pc = 0x800a8e20; continue;
                pc = 0x800a8e1c;
            }
            0x800a8e1c => {
                c.r[11] = 0x1;
                pc = 0x800a8e20;
            }
            0x800a8e20 => {
                c.r[0] = c.lhz(c.r[3]);
                c.r[5] = c.lha(c.r[13].wrapping_add(0xffffa04a));
                c.r[7] = c.r[0] as u16 as i16 as i32 as u32;
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.r[7] = c.r[7].wrapping_add(c.r[5]);
                c.cmps(0, c.r[7], c.r[0]);
                if !c.crbit(1) { pc = 0x800a8e40; continue; }
                pc = 0x800a8e3c;
            }
            0x800a8e3c => {
                pc = 0x800a8e44; continue;
                pc = 0x800a8e40;
            }
            0x800a8e40 => {
                c.r[7] = c.r[0] | c.r[0];
                pc = 0x800a8e44;
            }
            0x800a8e44 => {
                c.sth(c.r[30].wrapping_add(0xfa), c.r[7]);
                c.r[7] = c.r[9] as u16 as i16 as i32 as u32;
                c.r[7] = c.r[7].rotate_left(1) & 0xfffffffe;
                c.r[28] = c.lhz(c.r[6]);
                c.r[12] = c.r[7].wrapping_sub(c.r[0]);
                c.r[10] = c.lha(c.r[3]);
                c.r[7] = c.r[28] as u16 as i16 as i32 as u32;
                c.r[9] = c.r[7].wrapping_add(c.r[5]);
                c.r[9] = c.r[10].wrapping_add(c.r[9]);
                c.r[7] = c.r[9].wrapping_sub(c.r[12]);
                c.rc(c.r[7]);
                if !c.crbit(1) { pc = 0x800a8e78; continue; }
                pc = 0x800a8e70;
            }
            0x800a8e70 => {
                c.r[9] = c.r[9].wrapping_sub(c.r[12]);
                pc = 0x800a8e7c; continue;
                pc = 0x800a8e78;
            }
            0x800a8e78 => {
                c.r[9] = 0x0;
                pc = 0x800a8e7c;
            }
            0x800a8e7c => {
                c.r[10] = c.r[10].wrapping_add(c.r[5]);
                c.r[7] = c.r[10].wrapping_sub(c.r[0]);
                c.rc(c.r[7]);
                if !c.crbit(0) { pc = 0x800a8e90; continue; }
                pc = 0x800a8e88;
            }
            0x800a8e88 => {
                c.r[7] = c.r[10].wrapping_sub(c.r[0]);
                pc = 0x800a8e94; continue;
                pc = 0x800a8e90;
            }
            0x800a8e90 => {
                c.r[7] = 0x0;
                pc = 0x800a8e94;
            }
            0x800a8e94 => {
                c.r[7] = c.r[28].wrapping_add(c.r[7]);
                c.r[7] = c.r[7].wrapping_sub(c.r[9]);
                c.sth(c.r[30].wrapping_add(0xfc), c.r[7]);
                c.r[7] = c.lha(c.r[3]);
                c.r[9] = c.r[7].wrapping_add(c.r[5]);
                c.r[7] = c.r[9].wrapping_sub(c.r[0]);
                c.rc(c.r[7]);
                if !c.crbit(0) { pc = 0x800a8eb8; continue; }
                pc = 0x800a8eb0;
            }
            0x800a8eb0 => {
                c.r[7] = c.r[9].wrapping_sub(c.r[0]);
                pc = 0x800a8ebc; continue;
                pc = 0x800a8eb8;
            }
            0x800a8eb8 => {
                c.r[7] = 0x0;
                pc = 0x800a8ebc;
            }
            0x800a8ebc => {
                c.r[10] = divw(c.r[7], c.r[11]);
                c.r[7] = c.r[30].wrapping_add(0x108);
                c.r[9] = c.lhz(c.r[30].wrapping_add(0x108));
                c.r[9] = c.r[9].wrapping_sub(c.r[10]);
                c.sth(c.r[30].wrapping_add(0xfe), c.r[9]);
                c.r[6] = c.lha(c.r[6]);
                c.r[9] = c.lha(c.r[3]);
                c.r[6] = c.r[6].wrapping_add(c.r[5]);
                c.r[6] = c.r[9].wrapping_add(c.r[6]);
                c.r[3] = c.r[6].wrapping_sub(c.r[12]);
                c.rc(c.r[3]);
                if !c.crbit(1) { pc = 0x800a8ef0; continue; }
                pc = 0x800a8ee8;
            }
            0x800a8ee8 => {
                c.r[6] = c.r[6].wrapping_sub(c.r[12]);
                pc = 0x800a8ef4; continue;
                pc = 0x800a8ef0;
            }
            0x800a8ef0 => {
                c.r[6] = 0x0;
                pc = 0x800a8ef4;
            }
            0x800a8ef4 => {
                c.r[5] = c.r[9].wrapping_add(c.r[5]);
                c.r[3] = c.r[5].wrapping_sub(c.r[0]);
                c.rc(c.r[3]);
                if !c.crbit(0) { pc = 0x800a8f08; continue; }
                pc = 0x800a8f00;
            }
            0x800a8f00 => {
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                pc = 0x800a8f0c; continue;
                pc = 0x800a8f08;
            }
            0x800a8f08 => {
                c.r[0] = 0x0;
                pc = 0x800a8f0c;
            }
            0x800a8f0c => {
                c.r[0] = divw(c.r[0], c.r[11]);
                c.r[3] = c.lhz(c.r[30].wrapping_add(0x10c));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.r[5] = divw(c.r[6], c.r[11]);
                c.r[0] = c.r[0].wrapping_sub(c.r[5]);
                c.sth(c.r[30].wrapping_add(0x100), c.r[0]);
                c.r[9] = 0x280;
                c.r[28] = 0x0;
                c.sth(c.r[30].wrapping_add(0x102), c.r[9]);
                c.r[5] = 0x28;
                c.r[0] = 0x1;
                c.r[6] = c.lhz(c.r[4]);
                c.r[3] = c.r[13].wrapping_add(0xffffa034);
                c.r[6] = c.r[6].rotate_left(1) & 0xfffe;
                c.sth(c.r[30].wrapping_add(0x104), c.r[6]);
                c.sth(c.r[30].wrapping_add(0x106), c.r[28]);
                c.sth(c.r[7], c.r[28]);
                c.sth(c.r[30].wrapping_add(0x10a), c.r[9]);
                c.r[4] = c.lhz(c.r[4]);
                c.r[4] = c.r[4].rotate_left(1) & 0xfffe;
                c.sth(c.r[30].wrapping_add(0x10c), c.r[4]);
                c.stw(c.r[8], c.r[28]);
                c.stb(c.r[30].wrapping_add(0x11c), c.r[5]);
                c.stb(c.r[30].wrapping_add(0x11d), c.r[5]);
                c.stb(c.r[30].wrapping_add(0x11e), c.r[5]);
                c.stb(c.r[30].wrapping_add(0x12c), c.r[28]);
                c.stw(c.r[30].wrapping_add(0x130), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x134), c.r[28]);
                c.lr = 0x800a8f80; f_800a13e8(c);
                pc = 0x800a8f80;
            }
            0x800a8f80 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x2030));
                c.r[4] = c.r[3].wrapping_add(0x2000);
                c.r[5] = c.r[3].wrapping_add(0x2000);
                c.r[0] = c.r[0].rotate_left(0) & 0x7fff;
                c.sth(c.r[4].wrapping_add(0x30), c.r[0]);
                c.r[3] = 0x800b0000;
                c.r[4] = c.r[3].wrapping_add(0xffff85dc);
                c.r[0] = c.lhz(c.r[5].wrapping_add(0x34));
                c.r[3] = 0x18;
                c.r[0] = c.r[0].rotate_left(0) & 0x7fff;
                c.sth(c.r[5].wrapping_add(0x34), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa03c), c.r[28]);
                c.stw(c.r[13].wrapping_add(0xffffa040), c.r[28]);
                c.lr = 0x800a8fbc; f_8009df88(c);
                pc = 0x800a8fbc;
            }
            0x800a8fbc => {
                c.r[3] = 0x80;
                c.lr = 0x800a8fc4; f_8009e38c(c);
                pc = 0x800a8fc4;
            }
            0x800a8fc4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a8b34: bad pc {:#x}", pc),
        }
    }
}

/// VIWaitForRetrace (0x800a8fe4, 84 bytes)
#[inline(never)]
pub fn f_800a8fe4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a8fe4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a8fe4); p = true; }
    }
    f_800a8fe4_b(c, 0x800a8fe4);
    if p { crate::prof::exit(c, 0x800a8fe4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a8fe4_at(c: &mut Cpu, pc: u32) { f_800a8fe4_b(c, pc) }
#[inline(always)]
fn f_800a8fe4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a8fe4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a8fe4, pc); }
        match pc {
            0x800a8fe4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.lr = 0x800a8ffc; f_8009df3c(c);
                pc = 0x800a8ffc;
            }
            0x800a8ffc => {
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa02c));
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800a9004;
            }
            0x800a9004 => {
                c.r[3] = c.r[13].wrapping_add(0xffffa034);
                c.lr = 0x800a900c; f_800a238c(c);
                pc = 0x800a900c;
            }
            0x800a900c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa02c));
                c.cmpu(0, c.r[30], c.r[0]);
                if c.crbit(2) { pc = 0x800a9004; continue; }
                pc = 0x800a9018;
            }
            0x800a9018 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a9020; f_8009df64(c);
                pc = 0x800a9020;
            }
            0x800a9020 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a8fe4: bad pc {:#x}", pc),
        }
    }
}

/// setFbbRegs (0x800a9038, 724 bytes)
#[inline(never)]
pub fn f_800a9038(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a9038) { return; }
        if c.prof { crate::prof::enter(c, 0x800a9038); p = true; }
    }
    f_800a9038_b(c, 0x800a9038);
    if p { crate::prof::exit(c, 0x800a9038); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a9038_at(c: &mut Cpu, pc: u32) { f_800a9038_b(c, pc) }
#[inline(never)]
fn f_800a9038_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a9038;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a9038, pc); }
        match pc {
            0x800a9038 => {
                { let ea = c.r[1].wrapping_add(0xffffffb8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[9] = 0x80400000;
                c.r[9] = c.r[9].wrapping_add(0xffffc488);
                c.stw(c.r[1].wrapping_add(0x44), c.r[31]);
                c.r[8] = c.lbz(c.r[3].wrapping_add(0x2c));
                c.r[0] = c.lhz(c.r[3].wrapping_add(0xe));
                c.r[31] = c.r[8].rotate_left(5) & 0xffffffe0;
                c.r[8] = c.lhz(c.r[3].wrapping_add(0x16));
                c.r[0] = c.r[31].wrapping_mul(c.r[0]);
                c.r[11] = c.lwz(c.r[3].wrapping_add(0x20));
                c.r[10] = c.lwz(c.r[3].wrapping_add(0x30));
                c.r[12] = c.lhz(c.r[3].wrapping_add(0xa));
                c.r[8] = c.r[8].rotate_left(1) & 0xffffffe0;
                c.r[0] = c.r[8].wrapping_add(c.r[0]);
                c.r[0] = c.r[10].wrapping_add(c.r[0]);
                c.cmps(0, c.r[11], 0x0);
                c.stw(c.r[4], c.r[0]);
                if !c.crbit(2) { pc = 0x800a9088; continue; }
                pc = 0x800a9080;
            }
            0x800a9080 => {
                c.r[8] = c.lwz(c.r[4]);
                pc = 0x800a9090; continue;
                pc = 0x800a9088;
            }
            0x800a9088 => {
                c.r[0] = c.lwz(c.r[4]);
                c.r[8] = c.r[0].wrapping_add(c.r[31]);
                pc = 0x800a9090;
            }
            0x800a9090 => {
                { let v = c.r[12] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.stw(c.r[5], c.r[8]);
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.addc(!c.r[0], c.r[12], 1);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800a90bc; continue; }
                pc = 0x800a90ac;
            }
            0x800a90ac => {
                c.r[8] = c.lwz(c.r[4]);
                c.r[0] = c.lwz(c.r[5]);
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[5], c.r[8]);
                pc = 0x800a90bc;
            }
            0x800a90bc => {
                c.r[0] = c.lwz(c.r[4]);
                c.r[0] = c.r[0].rotate_left(0) & 0x3fffffff;
                c.stw(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(0) & 0x3fffffff;
                c.stw(c.r[5], c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a916c; continue; }
                pc = 0x800a90e0;
            }
            0x800a90e0 => {
                c.r[8] = c.lbz(c.r[3].wrapping_add(0x2c));
                c.r[0] = c.lhz(c.r[3].wrapping_add(0xe));
                c.r[31] = c.r[8].rotate_left(5) & 0xffffffe0;
                c.r[8] = c.lhz(c.r[3].wrapping_add(0x16));
                c.r[0] = c.r[31].wrapping_mul(c.r[0]);
                c.r[11] = c.lwz(c.r[3].wrapping_add(0x20));
                c.r[10] = c.lwz(c.r[3].wrapping_add(0x48));
                c.r[12] = c.lhz(c.r[3].wrapping_add(0xa));
                c.r[8] = c.r[8].rotate_left(1) & 0xffffffe0;
                c.r[0] = c.r[8].wrapping_add(c.r[0]);
                c.r[0] = c.r[10].wrapping_add(c.r[0]);
                c.cmps(0, c.r[11], 0x0);
                c.stw(c.r[6], c.r[0]);
                if !c.crbit(2) { pc = 0x800a9120; continue; }
                pc = 0x800a9118;
            }
            0x800a9118 => {
                c.r[8] = c.lwz(c.r[6]);
                pc = 0x800a9128; continue;
                pc = 0x800a9120;
            }
            0x800a9120 => {
                c.r[0] = c.lwz(c.r[6]);
                c.r[8] = c.r[0].wrapping_add(c.r[31]);
                pc = 0x800a9128;
            }
            0x800a9128 => {
                { let v = c.r[12] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.stw(c.r[7], c.r[8]);
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.addc(!c.r[0], c.r[12], 1);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800a9154; continue; }
                pc = 0x800a9144;
            }
            0x800a9144 => {
                c.r[8] = c.lwz(c.r[6]);
                c.r[0] = c.lwz(c.r[7]);
                c.stw(c.r[6], c.r[0]);
                c.stw(c.r[7], c.r[8]);
                pc = 0x800a9154;
            }
            0x800a9154 => {
                c.r[0] = c.lwz(c.r[6]);
                c.r[0] = c.r[0].rotate_left(0) & 0x3fffffff;
                c.stw(c.r[6], c.r[0]);
                c.r[0] = c.lwz(c.r[7]);
                c.r[0] = c.r[0].rotate_left(0) & 0x3fffffff;
                c.stw(c.r[7], c.r[0]);
                pc = 0x800a916c;
            }
            0x800a916c => {
                c.r[0] = c.lwz(c.r[4]);
                c.r[8] = 0x1000000;
                c.cmpu(0, c.r[0], c.r[8]);
                if !c.crbit(0) { pc = 0x800a91a8; continue; }
                pc = 0x800a917c;
            }
            0x800a917c => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmpu(0, c.r[0], c.r[8]);
                if !c.crbit(0) { pc = 0x800a91a8; continue; }
                pc = 0x800a9188;
            }
            0x800a9188 => {
                c.r[0] = c.lwz(c.r[6]);
                c.cmpu(0, c.r[0], c.r[8]);
                if !c.crbit(0) { pc = 0x800a91a8; continue; }
                pc = 0x800a9194;
            }
            0x800a9194 => {
                c.r[0] = c.lwz(c.r[7]);
                c.cmpu(0, c.r[0], c.r[8]);
                if !c.crbit(0) { pc = 0x800a91a8; continue; }
                pc = 0x800a91a0;
            }
            0x800a91a0 => {
                c.r[10] = 0x0;
                pc = 0x800a91ac; continue;
                pc = 0x800a91a8;
            }
            0x800a91a8 => {
                c.r[10] = 0x1;
                pc = 0x800a91ac;
            }
            0x800a91ac => {
                c.cmpu(0, c.r[10], 0x0);
                if c.crbit(2) { pc = 0x800a91e4; continue; }
                pc = 0x800a91b4;
            }
            0x800a91b4 => {
                c.r[0] = c.lwz(c.r[4]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.stw(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.stw(c.r[5], c.r[0]);
                c.r[0] = c.lwz(c.r[6]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.stw(c.r[6], c.r[0]);
                c.r[0] = c.lwz(c.r[7]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.stw(c.r[7], c.r[0]);
                pc = 0x800a91e4;
            }
            0x800a91e4 => {
                c.r[0] = c.lwz(c.r[4]);
                c.r[11] = c.r[10].rotate_left(12) & 0xfffff000;
                c.r[8] = 0x10000;
                c.sth(c.r[9].wrapping_add(0x1e), c.r[0]);
                c.r[0] = 0x20000;
                c.r[10] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[10] = c.r[10] | c.r[8];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[12]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[10]);
                c.r[10] = c.lwz(c.r[4]);
                c.r[4] = c.lbz(c.r[3].wrapping_add(0x3c));
                c.r[10] = c.r[10].rotate_left(16) & 0xffff;
                c.r[4] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[10] | c.r[4];
                c.r[4] = c.r[11] | c.r[4];
                c.sth(c.r[9].wrapping_add(0x1c), c.r[4]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[10] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[10]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.sth(c.r[9].wrapping_add(0x26), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | 0x1000;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0xffff;
                c.sth(c.r[9].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | 0x2000;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a9300; continue; }
                pc = 0x800a9284;
            }
            0x800a9284 => {
                c.r[3] = c.lwz(c.r[6]);
                c.r[0] = c.r[8].wrapping_add(0xffff8000);
                c.sth(c.r[9].wrapping_add(0x22), c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[3] = c.r[3] | 0x4000;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[3]);
                c.r[3] = c.lwz(c.r[6]);
                c.r[3] = c.r[3].rotate_left(16) & 0xffff;
                c.sth(c.r[9].wrapping_add(0x20), c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lwz(c.r[7]);
                c.sth(c.r[9].wrapping_add(0x2a), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | 0x400;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lwz(c.r[7]);
                c.r[0] = c.r[0].rotate_left(16) & 0xffff;
                c.sth(c.r[9].wrapping_add(0x28), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | 0x800;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                pc = 0x800a9300;
            }
            0x800a9300 => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[1] = c.r[1].wrapping_add(0x48);
                if true { return; }
                return;
            }
            _ => panic!("f_800a9038: bad pc {:#x}", pc),
        }
    }
}

/// setVerticalRegs (0x800a930c, 416 bytes)
#[inline(never)]
pub fn f_800a930c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a930c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a930c); p = true; }
    }
    f_800a930c_b(c, 0x800a930c);
    if p { crate::prof::exit(c, 0x800a930c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a930c_at(c: &mut Cpu, pc: u32) { f_800a930c_b(c, pc) }
#[inline(always)]
fn f_800a930c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a930c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a930c, pc); }
        match pc {
            0x800a930c => {
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[11] = 0x80400000;
                c.r[11] = c.r[11].wrapping_add(0xffffc488);
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x30));
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[0] = c.lhz(c.r[11].wrapping_add(0x6c));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a9340; continue; }
                pc = 0x800a9334;
            }
            0x800a9334 => {
                c.r[12] = 0x1;
                c.r[29] = 0x2;
                pc = 0x800a9348; continue;
                pc = 0x800a9340;
            }
            0x800a9340 => {
                c.r[12] = 0x2;
                c.r[29] = 0x1;
                pc = 0x800a9348;
            }
            0x800a9348 => {
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                { let v = c.r[30] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.addc(!c.r[0], c.r[30], 1);
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a9398; continue; }
                pc = 0x800a9360;
            }
            0x800a9360 => {
                c.r[3] = c.r[12].rotate_left(0) & 0xffff;
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.r[0] = c.r[3].wrapping_mul(c.r[0]);
                c.r[3] = c.r[4].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.r[3] = c.r[29].rotate_left(0) & 0xffff;
                c.r[29] = c.r[3].wrapping_mul(c.r[30]);
                c.r[0] = c.r[0].wrapping_sub(c.r[30]);
                c.r[30] = c.r[3].wrapping_mul(c.r[0]);
                c.r[0] = c.r[7].wrapping_add(c.r[29]);
                c.r[6] = c.r[9].wrapping_add(c.r[30]);
                c.r[3] = c.r[8].wrapping_add(c.r[29]);
                c.r[7] = c.r[10].wrapping_add(c.r[30]);
                pc = 0x800a93cc; continue;
                pc = 0x800a9398;
            }
            0x800a9398 => {
                c.r[3] = c.r[12].rotate_left(0) & 0xffff;
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.r[0] = c.r[3].wrapping_mul(c.r[0]);
                c.r[3] = c.r[4].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.r[3] = c.r[29].rotate_left(0) & 0xffff;
                c.r[29] = c.r[3].wrapping_mul(c.r[30]);
                c.r[0] = c.r[0].wrapping_sub(c.r[30]);
                c.r[30] = c.r[3].wrapping_mul(c.r[0]);
                c.r[0] = c.r[8].wrapping_add(c.r[29]);
                c.r[6] = c.r[10].wrapping_add(c.r[30]);
                c.r[3] = c.r[7].wrapping_add(c.r[29]);
                c.r[7] = c.r[9].wrapping_add(c.r[30]);
                pc = 0x800a93cc;
            }
            0x800a93cc => {
                c.r[8] = c.r[4].rotate_left(0) & 0xffff;
                c.r[4] = c.r[12].rotate_left(0) & 0xffff;
                c.r[4] = divw(c.r[8], c.r[4]);
                c.cmps(0, c.r[31], 0x0);
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                if c.crbit(2) { pc = 0x800a9400; continue; }
                pc = 0x800a93e4;
            }
            0x800a93e4 => {
                c.r[4] = c.r[4].rotate_left(1) & 0xfffffffe;
                c.r[4] = c.r[4].wrapping_add(0xfffffffe);
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.r[4] = 0x0;
                c.r[6] = c.r[6].wrapping_add(0x2);
                c.r[7] = c.r[7].wrapping_add(0x2);
                pc = 0x800a9400;
            }
            0x800a9400 => {
                c.r[5] = c.r[5].rotate_left(0) & 0xff;
                c.r[4] = c.r[4].rotate_left(4) & 0xffff0;
                c.r[4] = c.r[5] | c.r[4];
                c.sth(c.r[11], c.r[4]);
                c.r[4] = 0x80000000;
                c.r[9] = 0x1000000;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[8] = 0x2000000;
                c.r[10] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[4] = c.r[5] | c.r[4];
                c.r[5] = 0x400000;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[10]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[4]);
                c.r[4] = 0x800000;
                c.sth(c.r[11].wrapping_add(0xe), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[10] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | c.r[9];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[10]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.sth(c.r[11].wrapping_add(0xc), c.r[6]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | c.r[8];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[6]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.sth(c.r[11].wrapping_add(0x12), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | c.r[5];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.sth(c.r[11].wrapping_add(0x10), c.r[7]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | c.r[4];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                if true { return; }
                return;
            }
            _ => panic!("f_800a930c: bad pc {:#x}", pc),
        }
    }
}

/// VIConfigure (0x800a94ac, 2088 bytes)
#[inline(never)]
pub fn f_800a94ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a94ac) { return; }
        if c.prof { crate::prof::enter(c, 0x800a94ac); p = true; }
    }
    f_800a94ac_b(c, 0x800a94ac);
    if p { crate::prof::exit(c, 0x800a94ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a94ac_at(c: &mut Cpu, pc: u32) { f_800a94ac_b(c, pc) }
#[inline(never)]
fn f_800a94ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a94ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a94ac, pc); }
        match pc {
            0x800a94ac => {
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x80310000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[4] = 0x80400000;
                { let ea = c.r[1].wrapping_add(0xffffff88); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 14..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[15] = c.r[5].wrapping_add(0x20e8);
                c.r[29] = c.r[4].wrapping_add(0xffffc488);
                c.lr = 0x800a94d4; f_8009df3c(c);
                pc = 0x800a94d4;
            }
            0x800a94d4 => {
                c.r[4] = c.lwz(c.r[31]);
                c.r[28] = c.r[29].wrapping_add(0x114);
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x114));
                c.r[14] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[4].rotate_left(0) & 0x3;
                c.cmpu(0, c.r[0], c.r[4]);
                if c.crbit(2) { pc = 0x800a94fc; continue; }
                pc = 0x800a94f0;
            }
            0x800a94f0 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa04c), c.r[0]);
                c.stw(c.r[28], c.r[4]);
                pc = 0x800a94fc;
            }
            0x800a94fc => {
                c.r[0] = c.lwz(c.r[31]);
                c.r[3] = 0x80000000;
                c.r[16] = c.lwz(c.r[3].wrapping_add(0xcc));
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.cmpu(0, c.r[0], 0x4);
                c.r[17] = c.r[0] | c.r[0];
                if !c.crbit(2) { pc = 0x800a9580; continue; }
                pc = 0x800a9518;
            }
            0x800a9518 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa07c));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a9580; continue; }
                pc = 0x800a9524;
            }
            0x800a9524 => {
                c.r[0] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.stw(c.r[13].wrapping_add(0xffffa07c), c.r[0]);
                c.r[3] = c.r[15].wrapping_add(0x260);
                c.lr = 0x800a9538; f_8009c2e0(c);
                pc = 0x800a9538;
            }
            0x800a9538 => {
                c.r[3] = c.r[15].wrapping_add(0x28c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800a9544; f_8009c2e0(c);
                pc = 0x800a9544;
            }
            0x800a9544 => {
                c.r[3] = c.r[15].wrapping_add(0x2b8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800a9550; f_8009c2e0(c);
                pc = 0x800a9550;
            }
            0x800a9550 => {
                c.r[3] = c.r[15].wrapping_add(0x2e4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800a955c; f_8009c2e0(c);
                pc = 0x800a955c;
            }
            0x800a955c => {
                c.r[3] = c.r[15].wrapping_add(0x310);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800a9568; f_8009c2e0(c);
                pc = 0x800a9568;
            }
            0x800a9568 => {
                c.r[3] = c.r[15].wrapping_add(0x33c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800a9574; f_8009c2e0(c);
                pc = 0x800a9574;
            }
            0x800a9574 => {
                c.r[3] = c.r[15].wrapping_add(0x260);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800a9580; f_8009c2e0(c);
                pc = 0x800a9580;
            }
            0x800a9580 => {
                c.cmps(0, c.r[16], 0x2);
                if c.crbit(2) { pc = 0x800a95b4; continue; }
                pc = 0x800a9588;
            }
            0x800a9588 => {
                if !c.crbit(0) { pc = 0x800a959c; continue; }
                pc = 0x800a958c;
            }
            0x800a958c => {
                c.cmps(0, c.r[16], 0x0);
                if c.crbit(2) { pc = 0x800a95b4; continue; }
                pc = 0x800a9594;
            }
            0x800a9594 => {
                if !c.crbit(0) { pc = 0x800a95d0; continue; }
                pc = 0x800a9598;
            }
            0x800a9598 => {
                pc = 0x800a95e0; continue;
                pc = 0x800a959c;
            }
            0x800a959c => {
                c.cmps(0, c.r[16], 0x6);
                if c.crbit(2) { pc = 0x800a95b4; continue; }
                pc = 0x800a95a4;
            }
            0x800a95a4 => {
                if !c.crbit(0) { pc = 0x800a95e0; continue; }
                pc = 0x800a95a8;
            }
            0x800a95a8 => {
                c.cmps(0, c.r[16], 0x5);
                if !c.crbit(0) { pc = 0x800a95d0; continue; }
                pc = 0x800a95b0;
            }
            0x800a95b0 => {
                pc = 0x800a95e0; continue;
                pc = 0x800a95b4;
            }
            0x800a95b4 => {
                c.cmpu(0, c.r[17], 0x0);
                if c.crbit(2) { pc = 0x800a95fc; continue; }
                pc = 0x800a95bc;
            }
            0x800a95bc => {
                c.cmpu(0, c.r[17], 0x2);
                if c.crbit(2) { pc = 0x800a95fc; continue; }
                pc = 0x800a95c4;
            }
            0x800a95c4 => {
                c.cmpu(0, c.r[17], 0x6);
                if !c.crbit(2) { pc = 0x800a95e0; continue; }
                pc = 0x800a95cc;
            }
            0x800a95cc => {
                pc = 0x800a95fc; continue;
                pc = 0x800a95d0;
            }
            0x800a95d0 => {
                c.cmpu(0, c.r[17], 0x1);
                if c.crbit(2) { pc = 0x800a95fc; continue; }
                pc = 0x800a95d8;
            }
            0x800a95d8 => {
                c.cmpu(0, c.r[17], 0x5);
                if c.crbit(2) { pc = 0x800a95fc; continue; }
                pc = 0x800a95e0;
            }
            0x800a95e0 => {
                c.r[6] = c.r[16].wrapping_add(0x0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[7] = c.r[17].wrapping_add(0x0);
                c.r[5] = c.r[15].wrapping_add(0x368);
                c.r[3] = c.r[13].wrapping_add(0xffff81dc);
                c.r[4] = 0x75c;
                c.lr = 0x800a95fc; f_800060f0(c);
                pc = 0x800a95fc;
            }
            0x800a95fc => {
                c.cmpu(0, c.r[17], 0x0);
                if c.crbit(2) { pc = 0x800a960c; continue; }
                pc = 0x800a9604;
            }
            0x800a9604 => {
                c.cmpu(0, c.r[17], 0x2);
                if !c.crbit(2) { pc = 0x800a9614; continue; }
                pc = 0x800a960c;
            }
            0x800a960c => {
                c.stw(c.r[29].wrapping_add(0x118), c.r[16]);
                pc = 0x800a9618; continue;
                pc = 0x800a9614;
            }
            0x800a9614 => {
                c.stw(c.r[29].wrapping_add(0x118), c.r[17]);
                pc = 0x800a9618;
            }
            0x800a9618 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0xa));
                c.sth(c.r[29].wrapping_add(0xf0), c.r[0]);
                c.r[0] = c.lwz(c.r[28]);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800a9638; continue; }
                pc = 0x800a962c;
            }
            0x800a962c => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(1) & 0xfffe;
                pc = 0x800a963c; continue;
                pc = 0x800a9638;
            }
            0x800a9638 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0xc));
                pc = 0x800a963c;
            }
            0x800a963c => {
                c.sth(c.r[29].wrapping_add(0xf2), c.r[0]);
                c.r[23] = c.r[29].wrapping_add(0xf2);
                c.r[24] = c.r[29].wrapping_add(0xf4);
                c.r[0] = c.lhz(c.r[31].wrapping_add(0xe));
                c.r[27] = c.r[29].wrapping_add(0x102);
                c.r[25] = c.r[29].wrapping_add(0x110);
                c.sth(c.r[29].wrapping_add(0xf4), c.r[0]);
                c.r[0] = 0x0;
                c.r[16] = c.r[29].wrapping_add(0x10a);
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x4));
                c.r[26] = c.r[29].wrapping_add(0x10c);
                c.r[15] = c.r[29].wrapping_add(0x106);
                c.sth(c.r[29].wrapping_add(0x102), c.r[3]);
                c.r[17] = c.r[29].wrapping_add(0x108);
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x8));
                c.sth(c.r[29].wrapping_add(0x104), c.r[3]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x14));
                c.stw(c.r[29].wrapping_add(0x110), c.r[3]);
                c.r[3] = c.lhz(c.r[29].wrapping_add(0x102));
                c.sth(c.r[29].wrapping_add(0x10a), c.r[3]);
                c.r[3] = c.lhz(c.r[29].wrapping_add(0x104));
                c.sth(c.r[29].wrapping_add(0x10c), c.r[3]);
                c.sth(c.r[29].wrapping_add(0x106), c.r[0]);
                c.sth(c.r[29].wrapping_add(0x108), c.r[0]);
                c.r[0] = c.lwz(c.r[28]);
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x800a96b0; continue; }
                pc = 0x800a96a8;
            }
            0x800a96a8 => {
                c.r[0] = c.lhz(c.r[26]);
                pc = 0x800a96dc; continue;
                pc = 0x800a96b0;
            }
            0x800a96b0 => {
                c.cmpu(0, c.r[0], 0x3);
                if !c.crbit(2) { pc = 0x800a96c0; continue; }
                pc = 0x800a96b8;
            }
            0x800a96b8 => {
                c.r[0] = c.lhz(c.r[26]);
                pc = 0x800a96dc; continue;
                pc = 0x800a96c0;
            }
            0x800a96c0 => {
                c.r[0] = c.lwz(c.r[25]);
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a96d8; continue; }
                pc = 0x800a96cc;
            }
            0x800a96cc => {
                c.r[0] = c.lhz(c.r[26]);
                c.r[0] = c.r[0].rotate_left(1) & 0xfffe;
                pc = 0x800a96dc; continue;
                pc = 0x800a96d8;
            }
            0x800a96d8 => {
                c.r[0] = c.lhz(c.r[26]);
                pc = 0x800a96dc;
            }
            0x800a96dc => {
                c.r[22] = c.r[29].wrapping_add(0xf6);
                c.sth(c.r[29].wrapping_add(0xf6), c.r[0]);
                c.r[0] = c.lwz(c.r[28]);
                c.cmpu(0, c.r[0], 0x3);
                if !c.crbit(2) { pc = 0x800a96f8; continue; }
                pc = 0x800a96f0;
            }
            0x800a96f0 => {
                c.r[0] = 0x1;
                pc = 0x800a96fc; continue;
                pc = 0x800a96f8;
            }
            0x800a96f8 => {
                c.r[0] = 0x0;
                pc = 0x800a96fc;
            }
            0x800a96fc => {
                c.r[18] = c.r[29].wrapping_add(0x134);
                c.stw(c.r[29].wrapping_add(0x134), c.r[0]);
                c.r[21] = c.r[29].wrapping_add(0x118);
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x118));
                c.r[0] = c.lwz(c.r[28]);
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x800a971c; f_800a8894(c);
                pc = 0x800a971c;
            }
            0x800a971c => {
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[29].wrapping_add(0x144), c.r[30]);
                c.r[0] = c.lhz(c.r[24]);
                c.r[4] = c.lha(c.r[29].wrapping_add(0xf0));
                c.r[3] = c.lha(c.r[13].wrapping_add(0xffffa048));
                c.r[5] = c.addc(!c.r[0], 0x2d0, 1);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x2));
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.cmps(0, c.r[3], c.r[5]);
                if !c.crbit(1) { pc = 0x800a9748; continue; }
                pc = 0x800a9744;
            }
            0x800a9744 => {
                pc = 0x800a9758; continue;
                pc = 0x800a9748;
            }
            0x800a9748 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x800a9754; continue; }
                pc = 0x800a9750;
            }
            0x800a9750 => {
                c.r[3] = 0x0;
                pc = 0x800a9754;
            }
            0x800a9754 => {
                c.r[5] = c.r[3] | c.r[3];
                pc = 0x800a9758;
            }
            0x800a9758 => {
                c.sth(c.r[29].wrapping_add(0xf8), c.r[5]);
                c.r[3] = c.r[29].wrapping_add(0xf8);
                c.r[4] = c.lwz(c.r[25]);
                c.cmps(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a9774; continue; }
                pc = 0x800a976c;
            }
            0x800a976c => {
                c.r[4] = 0x2;
                pc = 0x800a9778; continue;
                pc = 0x800a9774;
            }
            0x800a9774 => {
                c.r[4] = 0x1;
                pc = 0x800a9778;
            }
            0x800a9778 => {
                c.r[5] = c.lhz(c.r[23]);
                c.r[6] = c.lha(c.r[13].wrapping_add(0xffffa04a));
                c.r[7] = c.r[5] as u16 as i16 as i32 as u32;
                c.r[5] = c.r[5].rotate_left(0) & 0x1;
                c.r[7] = c.r[7].wrapping_add(c.r[6]);
                c.cmps(0, c.r[7], c.r[5]);
                if !c.crbit(1) { pc = 0x800a9798; continue; }
                pc = 0x800a9794;
            }
            0x800a9794 => {
                pc = 0x800a979c; continue;
                pc = 0x800a9798;
            }
            0x800a9798 => {
                c.r[7] = c.r[5] | c.r[5];
                pc = 0x800a979c;
            }
            0x800a979c => {
                c.sth(c.r[29].wrapping_add(0xfa), c.r[7]);
                c.r[0] = c.r[0] as u16 as i16 as i32 as u32;
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[8] = c.lhz(c.r[22]);
                c.r[0] = c.r[0].wrapping_sub(c.r[5]);
                c.r[7] = c.lha(c.r[23]);
                c.r[19] = c.r[29].wrapping_add(0xfa);
                c.r[9] = c.r[8] as u16 as i16 as i32 as u32;
                c.r[10] = c.r[9].wrapping_add(c.r[6]);
                c.r[10] = c.r[7].wrapping_add(c.r[10]);
                c.r[9] = c.r[10].wrapping_sub(c.r[0]);
                c.rc(c.r[9]);
                if !c.crbit(1) { pc = 0x800a97d4; continue; }
                pc = 0x800a97cc;
            }
            0x800a97cc => {
                c.r[9] = c.r[10].wrapping_sub(c.r[0]);
                pc = 0x800a97d8; continue;
                pc = 0x800a97d4;
            }
            0x800a97d4 => {
                c.r[9] = 0x0;
                pc = 0x800a97d8;
            }
            0x800a97d8 => {
                c.r[10] = c.r[7].wrapping_add(c.r[6]);
                c.r[7] = c.r[10].wrapping_sub(c.r[5]);
                c.rc(c.r[7]);
                if !c.crbit(0) { pc = 0x800a97ec; continue; }
                pc = 0x800a97e4;
            }
            0x800a97e4 => {
                c.r[7] = c.r[10].wrapping_sub(c.r[5]);
                pc = 0x800a97f0; continue;
                pc = 0x800a97ec;
            }
            0x800a97ec => {
                c.r[7] = 0x0;
                pc = 0x800a97f0;
            }
            0x800a97f0 => {
                c.r[7] = c.r[8].wrapping_add(c.r[7]);
                c.r[7] = c.r[7].wrapping_sub(c.r[9]);
                c.r[20] = c.r[29].wrapping_add(0xfc);
                c.sth(c.r[29].wrapping_add(0xfc), c.r[7]);
                c.r[7] = c.lha(c.r[23]);
                c.r[8] = c.r[7].wrapping_add(c.r[6]);
                c.r[7] = c.r[8].wrapping_sub(c.r[5]);
                c.rc(c.r[7]);
                if !c.crbit(0) { pc = 0x800a9818; continue; }
                pc = 0x800a9810;
            }
            0x800a9810 => {
                c.r[7] = c.r[8].wrapping_sub(c.r[5]);
                pc = 0x800a981c; continue;
                pc = 0x800a9818;
            }
            0x800a9818 => {
                c.r[7] = 0x0;
                pc = 0x800a981c;
            }
            0x800a981c => {
                c.r[8] = divw(c.r[7], c.r[4]);
                c.r[7] = c.lhz(c.r[17]);
                c.r[7] = c.r[7].wrapping_sub(c.r[8]);
                c.sth(c.r[29].wrapping_add(0xfe), c.r[7]);
                c.r[7] = c.lha(c.r[22]);
                c.r[9] = c.lha(c.r[23]);
                c.r[8] = c.r[7].wrapping_add(c.r[6]);
                c.r[8] = c.r[9].wrapping_add(c.r[8]);
                c.r[7] = c.r[8].wrapping_sub(c.r[0]);
                c.rc(c.r[7]);
                if !c.crbit(1) { pc = 0x800a984c; continue; }
                pc = 0x800a9844;
            }
            0x800a9844 => {
                c.r[7] = c.r[8].wrapping_sub(c.r[0]);
                pc = 0x800a9850; continue;
                pc = 0x800a984c;
            }
            0x800a984c => {
                c.r[7] = 0x0;
                pc = 0x800a9850;
            }
            0x800a9850 => {
                c.r[6] = c.r[9].wrapping_add(c.r[6]);
                c.r[0] = c.r[6].wrapping_sub(c.r[5]);
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x800a9864; continue; }
                pc = 0x800a985c;
            }
            0x800a985c => {
                c.r[0] = c.r[6].wrapping_sub(c.r[5]);
                pc = 0x800a9868; continue;
                pc = 0x800a9864;
            }
            0x800a9864 => {
                c.r[0] = 0x0;
                pc = 0x800a9868;
            }
            0x800a9868 => {
                c.r[0] = divw(c.r[0], c.r[4]);
                c.r[5] = c.lhz(c.r[26]);
                c.r[4] = divw(c.r[7], c.r[4]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.sth(c.r[29].wrapping_add(0x100), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa044));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a9894; continue; }
                pc = 0x800a988c;
            }
            0x800a988c => {
                c.r[0] = 0x3;
                c.stw(c.r[21], c.r[0]);
                pc = 0x800a9894;
            }
            0x800a9894 => {
                c.r[5] = c.lhz(c.r[30].wrapping_add(0x18));
                { let v = c.r[5] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[4] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[4] = c.addc(c.r[4], 0, ci); }
                { let v = c.r[5] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.addc(!c.r[0], c.r[5], 1);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                if c.crbit(2) { pc = 0x800a98c4; continue; }
                pc = 0x800a98bc;
            }
            0x800a98bc => {
                c.r[5] = c.lhz(c.r[30].wrapping_add(0x1a));
                pc = 0x800a98c8; continue;
                pc = 0x800a98c4;
            }
            0x800a98c4 => {
                c.r[5] = 0x0;
                pc = 0x800a98c8;
            }
            0x800a98c8 => {
                c.r[0] = c.r[5].wrapping_add(0x1);
                c.sth(c.r[29].wrapping_add(0x32), c.r[0]);
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[0] | 0x1000;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[4] = c.r[29].wrapping_add(0x2);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[5] = c.r[5] | 0x40;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[6]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[5]);
                c.sth(c.r[29].wrapping_add(0x30), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | 0x80;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lwz(c.r[28]);
                c.r[5] = c.lhz(c.r[29].wrapping_add(0x2));
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x800a9920; continue; }
                pc = 0x800a9918;
            }
            0x800a9918 => {
                c.cmpu(0, c.r[0], 0x3);
                if !c.crbit(2) { pc = 0x800a992c; continue; }
                pc = 0x800a9920;
            }
            0x800a9920 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xfffffffb;
                c.r[5] = c.r[0] | 0x4;
                pc = 0x800a9934; continue;
                pc = 0x800a992c;
            }
            0x800a992c => {
                c.r[5] = c.r[5].rotate_left(0) & 0xfffffffb;
                c.r[5] = (c.r[0].rotate_left(2) & 0x4) | (c.r[5] & 0xfffffffb);
                pc = 0x800a9934;
            }
            0x800a9934 => {
                c.r[6] = c.lwz(c.r[21]);
                c.r[5] = c.r[5].rotate_left(0) & 0xfffffff7;
                c.r[7] = c.lwz(c.r[18]);
                c.cmpu(0, c.r[6], 0x4);
                c.r[0] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[5] = c.r[5] | c.r[0];
                if c.crbit(2) { pc = 0x800a995c; continue; }
                pc = 0x800a9950;
            }
            0x800a9950 => {
                c.r[0] = c.r[6].wrapping_add(0xfffffffb);
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(1) { pc = 0x800a9964; continue; }
                pc = 0x800a995c;
            }
            0x800a995c => {
                c.r[0] = c.r[5].rotate_left(0) & 0xfffffcff;
                pc = 0x800a9970; continue;
                pc = 0x800a9964;
            }
            0x800a9964 => {
                c.r[5] = c.r[5].rotate_left(0) & 0xfffffcff;
                c.r[0] = c.r[6].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[5] | c.r[0];
                pc = 0x800a9970;
            }
            0x800a9970 => {
                c.sth(c.r[4], c.r[0]);
                c.r[0] = 0x40000000;
                c.r[6] = c.r[29].wrapping_add(0x6c);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lwz(c.r[31]);
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x6c));
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x800a99b0; continue; }
                pc = 0x800a99a0;
            }
            0x800a99a0 => {
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x800a99b0; continue; }
                pc = 0x800a99a8;
            }
            0x800a99a8 => {
                c.cmps(0, c.r[0], 0x1a);
                if !c.crbit(2) { pc = 0x800a99bc; continue; }
                pc = 0x800a99b0;
            }
            0x800a99b0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffffe;
                c.r[0] = c.r[0] | 0x1;
                pc = 0x800a99c0; continue;
                pc = 0x800a99bc;
            }
            0x800a99bc => {
                c.r[0] = c.r[4].rotate_left(0) & 0xfffffffe;
                pc = 0x800a99c0;
            }
            0x800a99c0 => {
                c.sth(c.r[6], c.r[0]);
                c.cmps(0, c.r[7], 0x0);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | 0x200;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[4]);
                c.r[4] = c.lhz(c.r[24]);
                c.r[5] = c.lhz(c.r[16]);
                if c.crbit(2) { pc = 0x800a99f0; continue; }
                pc = 0x800a99e8;
            }
            0x800a99e8 => {
                c.r[0] = c.r[5].rotate_left(1) & 0xfffffffe;
                pc = 0x800a99f4; continue;
                pc = 0x800a99f0;
            }
            0x800a99f0 => {
                c.r[0] = c.r[5] | c.r[5];
                pc = 0x800a99f4;
            }
            0x800a99f4 => {
                c.r[8] = c.r[0].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[8], c.r[4]);
                if !c.crbit(0) { pc = 0x800a9a4c; continue; }
                pc = 0x800a9a00;
            }
            0x800a9a00 => {
                c.r[6] = c.r[8].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[6].wrapping_add(0xffffffff);
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = divwu(c.r[0], c.r[4]);
                c.r[0] = c.r[0] | 0x1000;
                c.sth(c.r[29].wrapping_add(0x4a), c.r[0]);
                c.r[0] = 0x4000000;
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[7] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[6]);
                c.sth(c.r[29].wrapping_add(0x70), c.r[8]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | 0x80;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[6]);
                pc = 0x800a9a6c; continue;
                pc = 0x800a9a4c;
            }
            0x800a9a4c => {
                c.r[0] = 0x100;
                c.sth(c.r[29].wrapping_add(0x4a), c.r[0]);
                c.r[0] = 0x4000000;
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[7] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[6]);
                pc = 0x800a9a6c;
            }
            0x800a9a6c => {
                c.r[0] = c.lhz(c.r[3]);
                c.r[12] = c.addc(!c.r[4], 0x2d0, 1);
                c.r[6] = c.lhz(c.r[30].wrapping_add(0x1a));
                c.r[3] = 0x10000000;
                c.r[8] = 0x20000000;
                c.sth(c.r[29].wrapping_add(0x6), c.r[6]);
                c.r[9] = 0x4000000;
                c.r[7] = 0x8000000;
                c.r[10] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[6] = 0x100000;
                c.r[4] = 0x200000;
                c.r[11] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[10] = c.r[10] | c.r[3];
                c.r[3] = 0x40000;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[11]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[10]);
                c.r[10] = c.lbz(c.r[30].wrapping_add(0x1d));
                c.r[11] = c.lbz(c.r[30].wrapping_add(0x1e));
                c.r[11] = (c.r[10].rotate_left(8) & 0xff00) | (c.r[11] & 0xffff00ff);
                c.sth(c.r[29].wrapping_add(0x4), c.r[11]);
                c.r[10] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[11] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[8] = c.r[10] | c.r[8];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[11]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[8]);
                c.r[8] = c.lbz(c.r[30].wrapping_add(0x1f));
                c.r[10] = c.lbz(c.r[30].wrapping_add(0x1c));
                c.r[16] = c.r[8].wrapping_add(c.r[0]);
                c.r[11] = c.lhz(c.r[30].wrapping_add(0x20));
                c.r[16] = c.r[16].wrapping_add(0xffffffd8);
                c.r[8] = c.r[16].rotate_left(7) & 0xff80;
                c.r[8] = c.r[10] | c.r[8];
                c.sth(c.r[29].wrapping_add(0xa), c.r[8]);
                c.r[8] = c.r[11].wrapping_add(c.r[0]);
                c.r[0] = c.r[8].wrapping_add(0x28);
                c.r[10] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[0] = c.r[0].wrapping_sub(c.r[12]);
                c.r[8] = c.r[16].rotate_left(23) & 0x7fffff;
                c.r[11] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[9] = c.r[10] | c.r[9];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[11]);
                c.r[0] = c.r[8] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[9]);
                c.sth(c.r[29].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[8] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | c.r[7];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[8]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x10));
                c.r[7] = c.lbz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(5) & 0xffffffe0;
                c.r[0] = c.r[7] | c.r[0];
                c.sth(c.r[29].wrapping_add(0x16), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | c.r[6];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[7]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x14));
                c.r[6] = c.lbz(c.r[30].wrapping_add(0xe));
                c.r[0] = c.r[0].rotate_left(5) & 0xffffffe0;
                c.r[0] = c.r[6] | c.r[0];
                c.sth(c.r[29].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | c.r[4];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[6]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x12));
                c.r[4] = c.lbz(c.r[30].wrapping_add(0xd));
                c.r[0] = c.r[0].rotate_left(5) & 0xffffffe0;
                c.r[0] = c.r[4] | c.r[0];
                c.sth(c.r[29].wrapping_add(0x1a), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | c.r[3];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x16));
                c.r[3] = c.lbz(c.r[30].wrapping_add(0xf));
                c.r[0] = c.r[0].rotate_left(5) & 0xffffffe0;
                c.r[0] = c.r[3] | c.r[0];
                c.sth(c.r[29].wrapping_add(0x18), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[0] = 0x80000;
                c.r[6] = c.r[29].wrapping_add(0x11c);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                c.r[3] = c.lhz(c.r[27]);
                c.r[4] = c.lwz(c.r[25]);
                c.r[0] = c.r[3].wrapping_add(0xf);
                c.r[3] = c.lhz(c.r[15]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 28 != 0); c.r[0] = (v >> 4) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.cmps(0, c.r[4], 0x0);
                c.stb(c.r[29].wrapping_add(0x11c), c.r[0]);
                if !c.crbit(2) { pc = 0x800a9c08; continue; }
                pc = 0x800a9c00;
            }
            0x800a9c00 => {
                c.r[4] = c.lbz(c.r[6]);
                pc = 0x800a9c10; continue;
                pc = 0x800a9c08;
            }
            0x800a9c08 => {
                c.r[0] = c.lbz(c.r[6]);
                c.r[4] = c.r[0].rotate_left(1) & 0xfe;
                pc = 0x800a9c10;
            }
            0x800a9c10 => {
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 28 != 0); c.r[0] = (v >> 4) as u32; }
                c.stb(c.r[29].wrapping_add(0x11d), c.r[4]);
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[0] = c.addc(!c.r[0], c.r[3], 1);
                c.stb(c.r[29].wrapping_add(0x12c), c.r[0]);
                c.r[0] = c.r[5].wrapping_add(0xf);
                c.r[3] = 0x8000000;
                c.r[4] = c.lbz(c.r[29].wrapping_add(0x12c));
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 28 != 0); c.r[0] = (v >> 4) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.stb(c.r[29].wrapping_add(0x11e), c.r[0]);
                c.r[0] = c.lbz(c.r[29].wrapping_add(0x11e));
                c.r[4] = c.lbz(c.r[29].wrapping_add(0x11d));
                c.r[4] = (c.r[0].rotate_left(8) & 0xff00) | (c.r[4] & 0xffff00ff);
                c.sth(c.r[29].wrapping_add(0x48), c.r[4]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa078));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.cmpu(0, c.r[0], 0x0);
                c.r[0] = c.r[5] | c.r[3];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[4]);
                if c.crbit(2) { pc = 0x800a9c8c; continue; }
                pc = 0x800a9c74;
            }
            0x800a9c74 => {
                c.r[3] = c.r[29].wrapping_add(0xf0);
                c.r[4] = c.r[29].wrapping_add(0x124);
                c.r[5] = c.r[29].wrapping_add(0x128);
                c.r[6] = c.r[29].wrapping_add(0x13c);
                c.r[7] = c.r[29].wrapping_add(0x140);
                c.lr = 0x800a9c8c; f_800a9038(c);
                pc = 0x800a9c8c;
            }
            0x800a9c8c => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x130));
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lhz(c.r[19]);
                c.r[4] = c.lhz(c.r[20]);
                c.r[5] = c.lbz(c.r[30]);
                c.r[6] = c.lhz(c.r[30].wrapping_add(0x2));
                c.r[7] = c.lhz(c.r[30].wrapping_add(0x4));
                c.r[8] = c.lhz(c.r[30].wrapping_add(0x6));
                c.r[9] = c.lhz(c.r[30].wrapping_add(0x8));
                c.r[10] = c.lhz(c.r[30].wrapping_add(0xa));
                c.lr = 0x800a9cb8; f_800a930c(c);
                pc = 0x800a9cb8;
            }
            0x800a9cb8 => {
                c.r[3] = c.r[14] | c.r[14];
                c.lr = 0x800a9cc0; f_8009df64(c);
                pc = 0x800a9cc0;
            }
            0x800a9cc0 => {
                { let mut ea = c.r[1].wrapping_add(0x30); for n in 14..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x7c));
                c.r[1] = c.r[1].wrapping_add(0x78);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a94ac: bad pc {:#x}", pc),
        }
    }
}

/// VIConfigurePan (0x800a9cd4, 916 bytes)
#[inline(never)]
pub fn f_800a9cd4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800a9cd4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a9cd4); p = true; }
    }
    f_800a9cd4_b(c, 0x800a9cd4);
    if p { crate::prof::exit(c, 0x800a9cd4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a9cd4_at(c: &mut Cpu, pc: u32) { f_800a9cd4_b(c, pc) }
#[inline(never)]
fn f_800a9cd4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a9cd4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a9cd4, pc); }
        match pc {
            0x800a9cd4 => {
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3].wrapping_add(0x0);
                c.r[25] = c.r[4].wrapping_add(0x0);
                c.r[24] = c.r[5].wrapping_add(0x0);
                c.r[23] = c.r[6].wrapping_add(0x0);
                c.r[29] = c.r[7].wrapping_add(0xffffc488);
                c.lr = 0x800a9d00; f_8009df3c(c);
                pc = 0x800a9d00;
            }
            0x800a9d00 => {
                c.sth(c.r[29].wrapping_add(0x106), c.r[26]);
                c.r[11] = c.r[29].wrapping_add(0x106);
                c.r[8] = c.r[29].wrapping_add(0x108);
                c.sth(c.r[29].wrapping_add(0x108), c.r[25]);
                c.r[10] = c.r[29].wrapping_add(0x10a);
                c.r[4] = c.r[29].wrapping_add(0x10c);
                c.sth(c.r[29].wrapping_add(0x10a), c.r[24]);
                c.r[30] = c.r[3] | c.r[3];
                c.sth(c.r[29].wrapping_add(0x10c), c.r[23]);
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x114));
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x800a9d38; continue; }
                pc = 0x800a9d30;
            }
            0x800a9d30 => {
                c.r[0] = c.lhz(c.r[4]);
                pc = 0x800a9d64; continue;
                pc = 0x800a9d38;
            }
            0x800a9d38 => {
                c.cmpu(0, c.r[0], 0x3);
                if !c.crbit(2) { pc = 0x800a9d48; continue; }
                pc = 0x800a9d40;
            }
            0x800a9d40 => {
                c.r[0] = c.lhz(c.r[4]);
                pc = 0x800a9d64; continue;
                pc = 0x800a9d48;
            }
            0x800a9d48 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x110));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a9d60; continue; }
                pc = 0x800a9d54;
            }
            0x800a9d54 => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0].rotate_left(1) & 0xfffe;
                pc = 0x800a9d64; continue;
                pc = 0x800a9d60;
            }
            0x800a9d60 => {
                c.r[0] = c.lhz(c.r[4]);
                pc = 0x800a9d64;
            }
            0x800a9d64 => {
                c.r[27] = c.r[29].wrapping_add(0xf6);
                c.sth(c.r[29].wrapping_add(0xf6), c.r[0]);
                c.r[9] = c.r[29].wrapping_add(0xf4);
                c.r[0] = c.lhz(c.r[29].wrapping_add(0xf4));
                c.r[5] = c.lha(c.r[29].wrapping_add(0xf0));
                c.r[3] = c.lha(c.r[13].wrapping_add(0xffffa048));
                c.r[0] = c.addc(!c.r[0], 0x2d0, 1);
                c.r[28] = c.lwz(c.r[29].wrapping_add(0x144));
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                c.cmps(0, c.r[3], c.r[0]);
                c.r[7] = c.lhz(c.r[28].wrapping_add(0x2));
                if !c.crbit(1) { pc = 0x800a9d98; continue; }
                pc = 0x800a9d94;
            }
            0x800a9d94 => {
                pc = 0x800a9da8; continue;
                pc = 0x800a9d98;
            }
            0x800a9d98 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x800a9da4; continue; }
                pc = 0x800a9da0;
            }
            0x800a9da0 => {
                c.r[3] = 0x0;
                pc = 0x800a9da4;
            }
            0x800a9da4 => {
                c.r[0] = c.r[3] | c.r[3];
                pc = 0x800a9da8;
            }
            0x800a9da8 => {
                c.sth(c.r[29].wrapping_add(0xf8), c.r[0]);
                c.r[12] = c.r[29].wrapping_add(0x110);
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x110));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a9dc4; continue; }
                pc = 0x800a9dbc;
            }
            0x800a9dbc => {
                c.r[0] = 0x2;
                pc = 0x800a9dc8; continue;
                pc = 0x800a9dc4;
            }
            0x800a9dc4 => {
                c.r[0] = 0x1;
                pc = 0x800a9dc8;
            }
            0x800a9dc8 => {
                c.r[3] = c.lhz(c.r[29].wrapping_add(0xf2));
                c.r[6] = c.r[29].wrapping_add(0xf2);
                c.r[5] = c.lha(c.r[13].wrapping_add(0xffffa04a));
                c.r[25] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[3].rotate_left(0) & 0x1;
                c.r[26] = c.r[25].wrapping_add(c.r[5]);
                c.cmps(0, c.r[26], c.r[3]);
                if !c.crbit(1) { pc = 0x800a9dec; continue; }
                pc = 0x800a9de8;
            }
            0x800a9de8 => {
                pc = 0x800a9df0; continue;
                pc = 0x800a9dec;
            }
            0x800a9dec => {
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x800a9df0;
            }
            0x800a9df0 => {
                c.sth(c.r[29].wrapping_add(0xfa), c.r[26]);
                c.r[7] = c.r[7] as u16 as i16 as i32 as u32;
                c.r[7] = c.r[7].rotate_left(1) & 0xfffffffe;
                c.r[23] = c.lhz(c.r[27]);
                c.r[7] = c.r[7].wrapping_sub(c.r[3]);
                c.r[24] = c.lha(c.r[6]);
                c.r[31] = c.r[29].wrapping_add(0xfa);
                c.r[26] = c.r[23] as u16 as i16 as i32 as u32;
                c.r[25] = c.r[26].wrapping_add(c.r[5]);
                c.r[25] = c.r[24].wrapping_add(c.r[25]);
                c.r[26] = c.r[25].wrapping_sub(c.r[7]);
                c.rc(c.r[26]);
                if !c.crbit(1) { pc = 0x800a9e28; continue; }
                pc = 0x800a9e20;
            }
            0x800a9e20 => {
                c.r[25] = c.r[25].wrapping_sub(c.r[7]);
                pc = 0x800a9e2c; continue;
                pc = 0x800a9e28;
            }
            0x800a9e28 => {
                c.r[25] = 0x0;
                pc = 0x800a9e2c;
            }
            0x800a9e2c => {
                c.r[24] = c.r[24].wrapping_add(c.r[5]);
                c.r[26] = c.r[24].wrapping_sub(c.r[3]);
                c.rc(c.r[26]);
                if !c.crbit(0) { pc = 0x800a9e40; continue; }
                pc = 0x800a9e38;
            }
            0x800a9e38 => {
                c.r[26] = c.r[24].wrapping_sub(c.r[3]);
                pc = 0x800a9e44; continue;
                pc = 0x800a9e40;
            }
            0x800a9e40 => {
                c.r[26] = 0x0;
                pc = 0x800a9e44;
            }
            0x800a9e44 => {
                c.r[26] = c.r[23].wrapping_add(c.r[26]);
                c.r[26] = c.r[26].wrapping_sub(c.r[25]);
                c.sth(c.r[29].wrapping_add(0xfc), c.r[26]);
                c.r[26] = c.lha(c.r[6]);
                c.r[23] = c.r[26].wrapping_add(c.r[5]);
                c.r[26] = c.r[23].wrapping_sub(c.r[3]);
                c.rc(c.r[26]);
                if !c.crbit(0) { pc = 0x800a9e68; continue; }
                pc = 0x800a9e60;
            }
            0x800a9e60 => {
                c.r[25] = c.r[23].wrapping_sub(c.r[3]);
                pc = 0x800a9e6c; continue;
                pc = 0x800a9e68;
            }
            0x800a9e68 => {
                c.r[25] = 0x0;
                pc = 0x800a9e6c;
            }
            0x800a9e6c => {
                c.r[26] = divw(c.r[25], c.r[0]);
                c.r[8] = c.lhz(c.r[8]);
                c.r[8] = c.r[8].wrapping_sub(c.r[26]);
                c.sth(c.r[29].wrapping_add(0xfe), c.r[8]);
                c.r[8] = c.lha(c.r[27]);
                c.r[23] = c.lha(c.r[6]);
                c.r[8] = c.r[8].wrapping_add(c.r[5]);
                c.r[8] = c.r[23].wrapping_add(c.r[8]);
                c.r[6] = c.r[8].wrapping_sub(c.r[7]);
                c.rc(c.r[6]);
                if !c.crbit(1) { pc = 0x800a9e9c; continue; }
                pc = 0x800a9e94;
            }
            0x800a9e94 => {
                c.r[6] = c.r[8].wrapping_sub(c.r[7]);
                pc = 0x800a9ea0; continue;
                pc = 0x800a9e9c;
            }
            0x800a9e9c => {
                c.r[6] = 0x0;
                pc = 0x800a9ea0;
            }
            0x800a9ea0 => {
                c.r[7] = c.r[23].wrapping_add(c.r[5]);
                c.r[5] = c.r[7].wrapping_sub(c.r[3]);
                c.rc(c.r[5]);
                if !c.crbit(0) { pc = 0x800a9eb4; continue; }
                pc = 0x800a9eac;
            }
            0x800a9eac => {
                c.r[3] = c.r[7].wrapping_sub(c.r[3]);
                pc = 0x800a9eb8; continue;
                pc = 0x800a9eb4;
            }
            0x800a9eb4 => {
                c.r[3] = 0x0;
                pc = 0x800a9eb8;
            }
            0x800a9eb8 => {
                c.r[3] = divw(c.r[3], c.r[0]);
                c.r[4] = c.lhz(c.r[4]);
                c.r[5] = divw(c.r[6], c.r[0]);
                c.r[0] = c.r[4].wrapping_add(c.r[3]);
                c.r[0] = c.r[0].wrapping_sub(c.r[5]);
                c.sth(c.r[29].wrapping_add(0x100), c.r[0]);
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x134));
                c.r[5] = c.lhz(c.r[9]);
                c.cmps(0, c.r[0], 0x0);
                c.r[3] = c.lhz(c.r[10]);
                if c.crbit(2) { pc = 0x800a9eec; continue; }
                pc = 0x800a9ee4;
            }
            0x800a9ee4 => {
                c.r[0] = c.r[3].rotate_left(1) & 0xfffffffe;
                pc = 0x800a9ef0; continue;
                pc = 0x800a9eec;
            }
            0x800a9eec => {
                c.r[0] = c.r[3] | c.r[3];
                pc = 0x800a9ef0;
            }
            0x800a9ef0 => {
                c.r[6] = c.r[0].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[6], c.r[5]);
                if !c.crbit(0) { pc = 0x800a9f48; continue; }
                pc = 0x800a9efc;
            }
            0x800a9efc => {
                c.r[4] = c.r[6].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = divwu(c.r[0], c.r[5]);
                c.r[0] = c.r[0] | 0x1000;
                c.sth(c.r[29].wrapping_add(0x4a), c.r[0]);
                c.r[0] = 0x4000000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[5] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[4]);
                c.sth(c.r[29].wrapping_add(0x70), c.r[6]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] | 0x80;
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[4]);
                pc = 0x800a9f68; continue;
                pc = 0x800a9f48;
            }
            0x800a9f48 => {
                c.r[0] = 0x100;
                c.sth(c.r[29].wrapping_add(0x4a), c.r[0]);
                c.r[0] = 0x4000000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[5] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[4]);
                pc = 0x800a9f68;
            }
            0x800a9f68 => {
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x102));
                c.r[6] = c.r[29].wrapping_add(0x11c);
                c.r[5] = c.lwz(c.r[12]);
                c.r[0] = c.r[4].wrapping_add(0xf);
                c.r[4] = c.lhz(c.r[11]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 28 != 0); c.r[0] = (v >> 4) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.cmps(0, c.r[5], 0x0);
                c.stb(c.r[29].wrapping_add(0x11c), c.r[0]);
                if !c.crbit(2) { pc = 0x800a9f98; continue; }
                pc = 0x800a9f90;
            }
            0x800a9f90 => {
                c.r[5] = c.lbz(c.r[6]);
                pc = 0x800a9fa0; continue;
                pc = 0x800a9f98;
            }
            0x800a9f98 => {
                c.r[0] = c.lbz(c.r[6]);
                c.r[5] = c.r[0].rotate_left(1) & 0xfe;
                pc = 0x800a9fa0;
            }
            0x800a9fa0 => {
                { let v = c.r[4] as i32; c.set_ca(v < 0 && (v as u32) << 28 != 0); c.r[0] = (v >> 4) as u32; }
                c.stb(c.r[29].wrapping_add(0x11d), c.r[5]);
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[0] = c.addc(!c.r[0], c.r[4], 1);
                c.stb(c.r[29].wrapping_add(0x12c), c.r[0]);
                c.r[4] = c.r[29].wrapping_add(0x12c);
                c.r[0] = c.r[3].wrapping_add(0xf);
                c.r[4] = c.lbz(c.r[4]);
                c.r[3] = 0x8000000;
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 28 != 0); c.r[0] = (v >> 4) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.stb(c.r[29].wrapping_add(0x11e), c.r[0]);
                c.r[0] = c.lbz(c.r[29].wrapping_add(0x11e));
                c.r[4] = c.lbz(c.r[29].wrapping_add(0x11d));
                c.r[4] = (c.r[0].rotate_left(8) & 0xff00) | (c.r[4] & 0xffff00ff);
                c.sth(c.r[29].wrapping_add(0x48), c.r[4]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa078));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.cmpu(0, c.r[0], 0x0);
                c.r[0] = c.r[5] | c.r[3];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[4]);
                if c.crbit(2) { pc = 0x800aa020; continue; }
                pc = 0x800aa008;
            }
            0x800aa008 => {
                c.r[3] = c.r[29].wrapping_add(0xf0);
                c.r[4] = c.r[29].wrapping_add(0x124);
                c.r[5] = c.r[29].wrapping_add(0x128);
                c.r[6] = c.r[29].wrapping_add(0x13c);
                c.r[7] = c.r[29].wrapping_add(0x140);
                c.lr = 0x800aa020; f_800a9038(c);
                pc = 0x800aa020;
            }
            0x800aa020 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x130));
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lhz(c.r[31]);
                c.r[4] = c.lhz(c.r[27]);
                c.r[5] = c.lbz(c.r[28]);
                c.r[6] = c.lhz(c.r[28].wrapping_add(0x2));
                c.r[7] = c.lhz(c.r[28].wrapping_add(0x4));
                c.r[8] = c.lhz(c.r[28].wrapping_add(0x6));
                c.r[9] = c.lhz(c.r[28].wrapping_add(0x8));
                c.r[10] = c.lhz(c.r[28].wrapping_add(0xa));
                c.lr = 0x800aa04c; f_800a930c(c);
                pc = 0x800aa04c;
            }
            0x800aa04c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800aa054; f_8009df64(c);
                pc = 0x800aa054;
            }
            0x800aa054 => {
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[1] = c.r[1].wrapping_add(0x50);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a9cd4: bad pc {:#x}", pc),
        }
    }
}

/// VIFlush (0x800aa068, 304 bytes)
#[inline(never)]
pub fn f_800aa068(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa068) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa068); p = true; }
    }
    f_800aa068_b(c, 0x800aa068);
    if p { crate::prof::exit(c, 0x800aa068); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa068_at(c: &mut Cpu, pc: u32) { f_800aa068_b(c, pc) }
#[inline(always)]
fn f_800aa068_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa068;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa068, pc); }
        match pc {
            0x800aa068 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffffc488);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.lr = 0x800aa090; f_8009df3c(c);
                pc = 0x800aa090;
            }
            0x800aa090 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa058));
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[0] = 0x0;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa04c));
                c.r[3] = c.r[5] | c.r[4];
                c.stw(c.r[13].wrapping_add(0xffffa058), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa04c), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa060));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa064));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[4] | c.r[0];
                c.r[3] = c.r[5] | c.r[3];
                c.stw(c.r[13].wrapping_add(0xffffa064), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa060), c.r[0]);
                pc = 0x800aa144; continue;
                pc = 0x800aa0d0;
            }
            0x800aa0d0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[5] = 0x20;
                c.r[28] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[4] = c.r[28].wrapping_add(0x0);
                c.lr = 0x800aa0e4; f_800c4c74(c);
                pc = 0x800aa0e4;
            }
            0x800aa0e4 => {
                c.r[4] = c.r[4].leading_zeros();
                c.cmps(0, c.r[4], 0x20);
                c.r[0] = 0xffffffff;
                c.r[0] = c.r[28] & c.r[0];
                if !c.crbit(0) { pc = 0x800aa0fc; continue; }
                pc = 0x800aa0f8;
            }
            0x800aa0f8 => {
                pc = 0x800aa104; continue;
                pc = 0x800aa0fc;
            }
            0x800aa0fc => {
                c.r[3] = c.r[0].leading_zeros();
                c.r[4] = c.r[3].wrapping_add(0x20);
                pc = 0x800aa104;
            }
            0x800aa104 => {
                c.r[3] = c.r[4].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.lhz(c.r[31].wrapping_add(c.r[3]));
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.r[5] = c.addc(!c.r[4], 0x3f, 1);
                c.sth(c.r[3].wrapping_add(0x78), c.r[0]);
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x800aa124; f_800c4c50(c);
                pc = 0x800aa124;
            }
            0x800aa124 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[5] = !(c.r[3] | c.r[3]);
                c.r[4] = !(c.r[4] | c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] & c.r[5];
                c.r[3] = c.r[3] & c.r[4];
                c.stw(c.r[13].wrapping_add(0xffffa054), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa050), c.r[0]);
                pc = 0x800aa144;
            }
            0x800aa144 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa050));
                c.r[3] = 0x0;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa054));
                c.r[0] = c.r[0] ^ c.r[3];
                c.r[3] = c.r[4] ^ c.r[3];
                c.r[0] = c.r[3] | c.r[0];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800aa0d0; continue; }
                pc = 0x800aa160;
            }
            0x800aa160 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa030), c.r[0]);
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x120));
                c.stw(c.r[13].wrapping_add(0xffffa070), c.r[0]);
                c.lr = 0x800aa178; f_8009df64(c);
                pc = 0x800aa178;
            }
            0x800aa178 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa068: bad pc {:#x}", pc),
        }
    }
}

/// VISetNextFrameBuffer (0x800aa198, 108 bytes)
#[inline(never)]
pub fn f_800aa198(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa198) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa198); p = true; }
    }
    f_800aa198_b(c, 0x800aa198);
    if p { crate::prof::exit(c, 0x800aa198); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa198_at(c: &mut Cpu, pc: u32) { f_800aa198_b(c, pc) }
#[inline(always)]
fn f_800aa198_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa198;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa198, pc); }
        match pc {
            0x800aa198 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffc488);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800aa1bc; f_8009df3c(c);
                pc = 0x800aa1bc;
            }
            0x800aa1bc => {
                c.stw(c.r[31].wrapping_add(0x120), c.r[30]);
                c.r[0] = 0x1;
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[13].wrapping_add(0xffffa078), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0xf0);
                c.r[4] = c.r[31].wrapping_add(0x124);
                c.r[5] = c.r[31].wrapping_add(0x128);
                c.r[6] = c.r[31].wrapping_add(0x13c);
                c.r[7] = c.r[31].wrapping_add(0x140);
                c.lr = 0x800aa1e4; f_800a9038(c);
                pc = 0x800aa1e4;
            }
            0x800aa1e4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800aa1ec; f_8009df64(c);
                pc = 0x800aa1ec;
            }
            0x800aa1ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa198: bad pc {:#x}", pc),
        }
    }
}

/// VISetBlack (0x800aa204, 124 bytes)
#[inline(never)]
pub fn f_800aa204(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa204) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa204); p = true; }
    }
    f_800aa204_b(c, 0x800aa204);
    if p { crate::prof::exit(c, 0x800aa204); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa204_at(c: &mut Cpu, pc: u32) { f_800aa204_b(c, pc) }
#[inline(always)]
fn f_800aa204_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa204;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa204, pc); }
        match pc {
            0x800aa204 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffc488);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800aa228; f_8009df3c(c);
                pc = 0x800aa228;
            }
            0x800aa228 => {
                c.stw(c.r[31].wrapping_add(0x130), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[10] = c.lwz(c.r[31].wrapping_add(0x144));
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x130));
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lhz(c.r[31].wrapping_add(0xfa));
                c.r[4] = c.lhz(c.r[31].wrapping_add(0xf6));
                c.r[5] = c.lbz(c.r[10]);
                c.r[6] = c.lhz(c.r[10].wrapping_add(0x2));
                c.r[7] = c.lhz(c.r[10].wrapping_add(0x4));
                c.r[8] = c.lhz(c.r[10].wrapping_add(0x6));
                c.r[9] = c.lhz(c.r[10].wrapping_add(0x8));
                c.r[10] = c.lhz(c.r[10].wrapping_add(0xa));
                c.lr = 0x800aa260; f_800a930c(c);
                pc = 0x800aa260;
            }
            0x800aa260 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800aa268; f_8009df64(c);
                pc = 0x800aa268;
            }
            0x800aa268 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa204: bad pc {:#x}", pc),
        }
    }
}

/// VIGetRetraceCount (0x800aa280, 8 bytes)
#[inline(never)]
pub fn f_800aa280(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa280) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa280); p = true; }
    }
    f_800aa280_b(c, 0x800aa280);
    if p { crate::prof::exit(c, 0x800aa280); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa280_at(c: &mut Cpu, pc: u32) { f_800aa280_b(c, pc) }
#[inline(always)]
fn f_800aa280_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa280;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa280, pc); }
        match pc {
            0x800aa280 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa02c));
                if true { return; }
                return;
            }
            _ => panic!("f_800aa280: bad pc {:#x}", pc),
        }
    }
}

/// getCurrentFieldEvenOdd (0x800aa288, 104 bytes)
#[inline(never)]
pub fn f_800aa288(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa288) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa288); p = true; }
    }
    f_800aa288_b(c, 0x800aa288);
    if p { crate::prof::exit(c, 0x800aa288); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa288_at(c: &mut Cpu, pc: u32) { f_800aa288_b(c, pc) }
#[inline(always)]
fn f_800aa288_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa288;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa288, pc); }
        match pc {
            0x800aa288 => {
                c.r[3] = 0xcc000000;
                c.r[7] = c.r[3].wrapping_add(0x2000);
                { let ea = c.r[7].wrapping_add(0x2c); c.r[0] = c.lhz(ea); c.r[7] = ea; }
                c.r[4] = c.r[3].wrapping_add(0x2000);
                c.r[5] = c.r[0].rotate_left(0) & 0x7ff;
                pc = 0x800aa29c;
            }
            0x800aa29c => {
                c.r[0] = c.lhz(c.r[7]);
                c.r[6] = c.r[5] | c.r[5];
                c.r[3] = c.lhz(c.r[4].wrapping_add(0x2e));
                c.r[5] = c.r[0].rotate_left(0) & 0x7ff;
                c.cmpu(0, c.r[6], c.r[5]);
                c.r[3] = c.r[3].rotate_left(0) & 0x7ff;
                if !c.crbit(2) { pc = 0x800aa29c; continue; }
                pc = 0x800aa2b8;
            }
            0x800aa2b8 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa068));
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                c.r[4] = c.r[3].wrapping_add(0xffffffff);
                c.r[3] = c.lhz(c.r[6].wrapping_add(0x1a));
                c.r[5] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x18));
                c.r[3] = divwu(c.r[4], c.r[3]);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x800aa2e8; continue; }
                pc = 0x800aa2e0;
            }
            0x800aa2e0 => {
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x800aa2e8;
            }
            0x800aa2e8 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_800aa288: bad pc {:#x}", pc),
        }
    }
}

/// VIGetNextField (0x800aa2f0, 168 bytes)
#[inline(never)]
pub fn f_800aa2f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa2f0) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa2f0); p = true; }
    }
    f_800aa2f0_b(c, 0x800aa2f0);
    if p { crate::prof::exit(c, 0x800aa2f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa2f0_at(c: &mut Cpu, pc: u32) { f_800aa2f0_b(c, pc) }
#[inline(always)]
fn f_800aa2f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa2f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa2f0, pc); }
        match pc {
            0x800aa2f0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.lr = 0x800aa304; f_8009df3c(c);
                pc = 0x800aa304;
            }
            0x800aa304 => {
                c.r[4] = 0xcc000000;
                c.r[8] = c.r[4].wrapping_add(0x2000);
                { let ea = c.r[8].wrapping_add(0x2c); c.r[0] = c.lhz(ea); c.r[8] = ea; }
                c.r[5] = c.r[4].wrapping_add(0x2000);
                c.r[6] = c.r[0].rotate_left(0) & 0x7ff;
                pc = 0x800aa318;
            }
            0x800aa318 => {
                c.r[0] = c.lhz(c.r[8]);
                c.r[7] = c.r[6] | c.r[6];
                c.r[4] = c.lhz(c.r[5].wrapping_add(0x2e));
                c.r[6] = c.r[0].rotate_left(0) & 0x7ff;
                c.cmpu(0, c.r[7], c.r[6]);
                c.r[4] = c.r[4].rotate_left(0) & 0x7ff;
                if !c.crbit(2) { pc = 0x800aa318; continue; }
                pc = 0x800aa334;
            }
            0x800aa334 => {
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa068));
                c.r[0] = c.r[6].wrapping_add(0xffffffff);
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                c.r[4] = c.lhz(c.r[7].wrapping_add(0x1a));
                c.r[6] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.lhz(c.r[7].wrapping_add(0x18));
                c.r[4] = divwu(c.r[5], c.r[4]);
                c.r[4] = c.r[6].wrapping_add(c.r[4]);
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(0) { pc = 0x800aa364; continue; }
                pc = 0x800aa35c;
            }
            0x800aa35c => {
                c.r[31] = 0x1;
                pc = 0x800aa368; continue;
                pc = 0x800aa364;
            }
            0x800aa364 => {
                c.r[31] = 0x0;
                pc = 0x800aa368;
            }
            0x800aa368 => {
                c.lr = 0x800aa36c; f_8009df64(c);
                pc = 0x800aa36c;
            }
            0x800aa36c => {
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffc578);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0xa));
                c.r[3] = c.r[31] ^ 0x1;
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.r[3] = c.r[3] ^ c.r[0];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa2f0: bad pc {:#x}", pc),
        }
    }
}

/// VIGetCurrentLine (0x800aa398, 152 bytes)
#[inline(never)]
pub fn f_800aa398(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa398) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa398); p = true; }
    }
    f_800aa398_b(c, 0x800aa398);
    if p { crate::prof::exit(c, 0x800aa398); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa398_at(c: &mut Cpu, pc: u32) { f_800aa398_b(c, pc) }
#[inline(always)]
fn f_800aa398_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa398;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa398, pc); }
        match pc {
            0x800aa398 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa068));
                c.lr = 0x800aa3b4; f_8009df3c(c);
                pc = 0x800aa3b4;
            }
            0x800aa3b4 => {
                c.r[4] = 0xcc000000;
                c.r[8] = c.r[4].wrapping_add(0x2000);
                { let ea = c.r[8].wrapping_add(0x2c); c.r[0] = c.lhz(ea); c.r[8] = ea; }
                c.r[5] = c.r[4].wrapping_add(0x2000);
                c.r[6] = c.r[0].rotate_left(0) & 0x7ff;
                pc = 0x800aa3c8;
            }
            0x800aa3c8 => {
                c.r[0] = c.lhz(c.r[8]);
                c.r[7] = c.r[6] | c.r[6];
                c.r[4] = c.lhz(c.r[5].wrapping_add(0x2e));
                c.r[6] = c.r[0].rotate_left(0) & 0x7ff;
                c.cmpu(0, c.r[7], c.r[6]);
                c.r[7] = c.r[4].rotate_left(0) & 0x7ff;
                if !c.crbit(2) { pc = 0x800aa3c8; continue; }
                pc = 0x800aa3e4;
            }
            0x800aa3e4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa068));
                c.r[5] = c.r[7].wrapping_add(0xffffffff);
                c.r[6] = c.r[6].wrapping_add(0xffffffff);
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x1a));
                c.r[4] = c.r[6].rotate_left(1) & 0xfffffffe;
                c.r[0] = divwu(c.r[5], c.r[0]);
                c.r[30] = c.r[4].wrapping_add(c.r[0]);
                c.lr = 0x800aa404; f_8009df64(c);
                pc = 0x800aa404;
            }
            0x800aa404 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x18));
                c.cmpu(0, c.r[30], c.r[0]);
                if c.crbit(0) { pc = 0x800aa414; continue; }
                pc = 0x800aa410;
            }
            0x800aa410 => {
                c.r[30] = c.r[30].wrapping_sub(c.r[0]);
                pc = 0x800aa414;
            }
            0x800aa414 => {
                c.r[3] = c.r[30].rotate_left(31) & 0x7fffffff;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa398: bad pc {:#x}", pc),
        }
    }
}

/// VIGetTvFormat (0x800aa430, 104 bytes)
#[inline(never)]
pub fn f_800aa430(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa430) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa430); p = true; }
    }
    f_800aa430_b(c, 0x800aa430);
    if p { crate::prof::exit(c, 0x800aa430); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa430_at(c: &mut Cpu, pc: u32) { f_800aa430_b(c, pc) }
#[inline(always)]
fn f_800aa430_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa430;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa430, pc); }
        match pc {
            0x800aa430 => {
                c.r[0] = c.mfspr(8);
                pc = 0x800aa434;
            }
            0x800aa434 => {
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                pc = 0x800aa438;
            }
            0x800aa438 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800aa43c;
            }
            0x800aa43c => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                pc = 0x800aa440;
            }
            0x800aa440 => {
                c.lr = 0x800aa444; f_8009df3c(c);
                pc = 0x800aa444;
            }
            0x800aa444 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa06c));
                pc = 0x800aa448;
            }
            0x800aa448 => {
                c.cmpu(0, c.r[5], 0x6);
                pc = 0x800aa44c;
            }
            0x800aa44c => {
                if c.crbit(1) { pc = 0x800aa47c; continue; }
                pc = 0x800aa450;
            }
            0x800aa450 => {
                c.r[4] = 0x80310000;
                pc = 0x800aa454;
            }
            0x800aa454 => {
                c.r[4] = c.r[4].wrapping_add(0x249c);
                pc = 0x800aa458;
            }
            0x800aa458 => {
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                pc = 0x800aa45c;
            }
            0x800aa45c => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                pc = 0x800aa460;
            }
            0x800aa460 => {
                c.mtspr(9, c.r[0]);
                pc = 0x800aa464;
            }
            0x800aa464 => {
                if true { let t = c.ctr; if (0x800aa430..0x800aa498).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x800aa468;
            }
            0x800aa468 => {
                c.r[31] = 0x0;
                pc = 0x800aa46c;
            }
            0x800aa46c => {
                pc = 0x800aa47c; continue;
                pc = 0x800aa470;
            }
            0x800aa470 => {
                c.r[31] = 0x1;
                pc = 0x800aa474;
            }
            0x800aa474 => {
                pc = 0x800aa47c; continue;
                pc = 0x800aa478;
            }
            0x800aa478 => {
                c.r[31] = c.r[5] | c.r[5];
                pc = 0x800aa47c;
            }
            0x800aa47c => {
                c.lr = 0x800aa480; f_8009df64(c);
                pc = 0x800aa480;
            }
            0x800aa480 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800aa484;
            }
            0x800aa484 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x800aa488;
            }
            0x800aa488 => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x800aa48c;
            }
            0x800aa48c => {
                c.r[1] = c.r[1].wrapping_add(0x10);
                pc = 0x800aa490;
            }
            0x800aa490 => {
                c.mtspr(8, c.r[0]);
                pc = 0x800aa494;
            }
            0x800aa494 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800aa430: bad pc {:#x}", pc),
        }
    }
}

/// VIGetDTVStatus (0x800aa498, 60 bytes)
#[inline(never)]
pub fn f_800aa498(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa498) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa498); p = true; }
    }
    f_800aa498_b(c, 0x800aa498);
    if p { crate::prof::exit(c, 0x800aa498); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa498_at(c: &mut Cpu, pc: u32) { f_800aa498_b(c, pc) }
#[inline(always)]
fn f_800aa498_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa498;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa498, pc); }
        match pc {
            0x800aa498 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x800aa4ac; f_8009df3c(c);
                pc = 0x800aa4ac;
            }
            0x800aa4ac => {
                c.r[4] = 0xcc000000;
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x206e));
                c.r[31] = c.r[0].rotate_left(0) & 0x3;
                c.lr = 0x800aa4bc; f_8009df64(c);
                pc = 0x800aa4bc;
            }
            0x800aa4bc => {
                c.r[3] = c.r[31].rotate_left(0) & 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa498: bad pc {:#x}", pc),
        }
    }
}

/// UpdateOrigin (0x800aa4d4, 420 bytes)
#[inline(never)]
pub fn f_800aa4d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa4d4) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa4d4); p = true; }
    }
    f_800aa4d4_b(c, 0x800aa4d4);
    if p { crate::prof::exit(c, 0x800aa4d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa4d4_at(c: &mut Cpu, pc: u32) { f_800aa4d4_b(c, pc) }
#[inline(never)]
fn f_800aa4d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa4d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa4d4, pc); }
        match pc {
            0x800aa4d4 => {
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[6] = c.r[3].wrapping_mul(0xc);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[5] = c.r[5].wrapping_add(0xffffc5e0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[5].wrapping_add(c.r[6]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81f4));
                c.r[4] = c.r[0].rotate_left(0) & 0x700;
                c.cmps(0, c.r[4], 0x400);
                c.r[0] = 0x80000000;
                c.r[0] = srw(c.r[0], c.r[3]);
                if c.crbit(2) { pc = 0x800aa5fc; continue; }
                pc = 0x800aa50c;
            }
            0x800aa50c => {
                if !c.crbit(0) { pc = 0x800aa540; continue; }
                pc = 0x800aa510;
            }
            0x800aa510 => {
                c.cmps(0, c.r[4], 0x200);
                if c.crbit(2) { pc = 0x800aa5cc; continue; }
                pc = 0x800aa518;
            }
            0x800aa518 => {
                if !c.crbit(0) { pc = 0x800aa534; continue; }
                pc = 0x800aa51c;
            }
            0x800aa51c => {
                c.cmps(0, c.r[4], 0x100);
                if c.crbit(2) { pc = 0x800aa598; continue; }
                pc = 0x800aa524;
            }
            0x800aa524 => {
                if !c.crbit(0) { pc = 0x800aa5fc; continue; }
                pc = 0x800aa528;
            }
            0x800aa528 => {
                c.cmps(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800aa564; continue; }
                pc = 0x800aa530;
            }
            0x800aa530 => {
                pc = 0x800aa5fc; continue;
                pc = 0x800aa534;
            }
            0x800aa534 => {
                c.cmps(0, c.r[4], 0x300);
                if c.crbit(2) { pc = 0x800aa5fc; continue; }
                pc = 0x800aa53c;
            }
            0x800aa53c => {
                pc = 0x800aa5fc; continue;
                pc = 0x800aa540;
            }
            0x800aa540 => {
                c.cmps(0, c.r[4], 0x600);
                if c.crbit(2) { pc = 0x800aa564; continue; }
                pc = 0x800aa548;
            }
            0x800aa548 => {
                if !c.crbit(0) { pc = 0x800aa558; continue; }
                pc = 0x800aa54c;
            }
            0x800aa54c => {
                c.cmps(0, c.r[4], 0x500);
                if c.crbit(2) { pc = 0x800aa564; continue; }
                pc = 0x800aa554;
            }
            0x800aa554 => {
                pc = 0x800aa5fc; continue;
                pc = 0x800aa558;
            }
            0x800aa558 => {
                c.cmps(0, c.r[4], 0x700);
                if c.crbit(2) { pc = 0x800aa564; continue; }
                pc = 0x800aa560;
            }
            0x800aa560 => {
                pc = 0x800aa5fc; continue;
                pc = 0x800aa564;
            }
            0x800aa564 => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x6));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x6), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x7));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x7), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x8));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x8), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x9));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x9), c.r[4]);
                pc = 0x800aa5fc; continue;
                pc = 0x800aa598;
            }
            0x800aa598 => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x4));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x4), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x5));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x5), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x8));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x8), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x9));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x9), c.r[4]);
                pc = 0x800aa5fc; continue;
                pc = 0x800aa5cc;
            }
            0x800aa5cc => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x4));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x4), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x5));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x5), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x6));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x6), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x7));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff0;
                c.stb(c.r[31].wrapping_add(0x7), c.r[4]);
                pc = 0x800aa5fc;
            }
            0x800aa5fc => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x2));
                c.r[4] = c.r[4].wrapping_add(0xffffff80);
                c.stb(c.r[31].wrapping_add(0x2), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x3));
                c.r[4] = c.r[4].wrapping_add(0xffffff80);
                c.stb(c.r[31].wrapping_add(0x3), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x4));
                c.r[4] = c.r[4].wrapping_add(0xffffff80);
                c.stb(c.r[31].wrapping_add(0x4), c.r[4]);
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x5));
                c.r[4] = c.r[4].wrapping_add(0xffffff80);
                c.stb(c.r[31].wrapping_add(0x5), c.r[4]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff81f0));
                c.r[0] = c.r[4] & c.r[0];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aa664; continue; }
                pc = 0x800aa638;
            }
            0x800aa638 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x2));
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x40);
                if !c.crbit(1) { pc = 0x800aa664; continue; }
                pc = 0x800aa648;
            }
            0x800aa648 => {
                c.lr = 0x800aa64c; f_800d0af8(c);
                pc = 0x800aa64c;
            }
            0x800aa64c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff0000;
                c.r[0] = c.r[3].wrapping_add(0xf7000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800aa664; continue; }
                pc = 0x800aa65c;
            }
            0x800aa65c => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x2), c.r[0]);
                pc = 0x800aa664;
            }
            0x800aa664 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa4d4: bad pc {:#x}", pc),
        }
    }
}

/// PADOriginCallback (0x800aa678, 196 bytes)
#[inline(never)]
pub fn f_800aa678(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa678) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa678); p = true; }
    }
    f_800aa678_b(c, 0x800aa678);
    if p { crate::prof::exit(c, 0x800aa678); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa678_at(c: &mut Cpu, pc: u32) { f_800aa678_b(c, pc) }
#[inline(always)]
fn f_800aa678_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa678;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa678, pc); }
        match pc {
            0x800aa678 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xf;
                c.rc(c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                if !c.crbit(2) { pc = 0x800aa6d4; continue; }
                pc = 0x800aa690;
            }
            0x800aa690 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.lr = 0x800aa698; f_800aa4d4(c);
                pc = 0x800aa698;
            }
            0x800aa698 => {
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[0] = 0x80000000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[4] = c.r[1].wrapping_add(0x1c);
                c.r[0] = srw(c.r[0], c.r[31]);
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[0]);
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800aa6bc; f_800d05a4(c);
                pc = 0x800aa6bc;
            }
            0x800aa6bc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81f4));
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[0] | 0x400000;
                c.lr = 0x800aa6cc; f_800d0338(c);
                pc = 0x800aa6cc;
            }
            0x800aa6cc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.lr = 0x800aa6d4; f_800d03c8(c);
                pc = 0x800aa6d4;
            }
            0x800aa6d4 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[5].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[4], 0x20);
                if c.crbit(2) { pc = 0x800aa728; continue; }
                pc = 0x800aa6ec;
            }
            0x800aa6ec => {
                c.r[0] = 0x80000000;
                c.r[0] = srw(c.r[0], c.r[4]);
                c.r[0] = c.r[5] & !c.r[0];
                c.r[4] = c.r[4].wrapping_mul(0xc);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[0]);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffc5e0);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.lr = 0x800aa718; f_8000540c(c);
                pc = 0x800aa718;
            }
            0x800aa718 => {
                c.r[4] = 0x800b0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[4].wrapping_add(0xffffa8d4);
                c.lr = 0x800aa728; f_800d0cbc(c);
                pc = 0x800aa728;
            }
            0x800aa728 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa678: bad pc {:#x}", pc),
        }
    }
}

/// PADOriginUpdateCallback (0x800aa73c, 192 bytes)
#[inline(never)]
pub fn f_800aa73c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa73c) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa73c); p = true; }
    }
    f_800aa73c_b(c, 0x800aa73c);
    if p { crate::prof::exit(c, 0x800aa73c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa73c_at(c: &mut Cpu, pc: u32) { f_800aa73c_b(c, pc) }
#[inline(always)]
fn f_800aa73c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa73c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa73c, pc); }
        match pc {
            0x800aa73c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x80000000;
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = srw(c.r[0], c.r[29]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[0] = c.r[3] & c.r[30];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aa7e0; continue; }
                pc = 0x800aa770;
            }
            0x800aa770 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xf;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800aa780; continue; }
                pc = 0x800aa778;
            }
            0x800aa778 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800aa780; f_800aa4d4(c);
                pc = 0x800aa780;
            }
            0x800aa780 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aa7e0; continue; }
                pc = 0x800aa788;
            }
            0x800aa788 => {
                c.lr = 0x800aa78c; f_8009df3c(c);
                pc = 0x800aa78c;
            }
            0x800aa78c => {
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.lr = 0x800aa798; f_800d0464(c);
                pc = 0x800aa798;
            }
            0x800aa798 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[8] = !(c.r[30] | c.r[30]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa098));
                c.r[7] = c.r[6] & c.r[8];
                c.r[6] = c.r[5] & c.r[8];
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[7]);
                c.r[5] = c.r[4] & c.r[8];
                c.r[0] = c.r[0] & c.r[8];
                c.stw(c.r[13].wrapping_add(0xffffa090), c.r[6]);
                c.r[4] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa094), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa098), c.r[0]);
                c.lr = 0x800aa7d8; f_800a115c(c);
                pc = 0x800aa7d8;
            }
            0x800aa7d8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800aa7e0; f_8009df64(c);
                pc = 0x800aa7e0;
            }
            0x800aa7e0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa73c: bad pc {:#x}", pc),
        }
    }
}

/// PADProbeCallback (0x800aa7fc, 216 bytes)
#[inline(never)]
pub fn f_800aa7fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa7fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa7fc); p = true; }
    }
    f_800aa7fc_b(c, 0x800aa7fc);
    if p { crate::prof::exit(c, 0x800aa7fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa7fc_at(c: &mut Cpu, pc: u32) { f_800aa7fc_b(c, pc) }
#[inline(always)]
fn f_800aa7fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa7fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa7fc, pc); }
        match pc {
            0x800aa7fc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xf;
                c.rc(c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                if !c.crbit(2) { pc = 0x800aa868; continue; }
                pc = 0x800aa818;
            }
            0x800aa818 => {
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[31] = 0x80000000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[4] = c.r[1].wrapping_add(0x1c);
                c.r[0] = srw(c.r[31], c.r[30]);
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[0]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800aa83c; f_800d05a4(c);
                pc = 0x800aa83c;
            }
            0x800aa83c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81f4));
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.r[4] = c.r[0] | 0x400000;
                c.lr = 0x800aa84c; f_800d0338(c);
                pc = 0x800aa84c;
            }
            0x800aa84c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.lr = 0x800aa854; f_800d03c8(c);
                pc = 0x800aa854;
            }
            0x800aa854 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[0] = srw(c.r[31], c.r[0]);
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffffa090), c.r[0]);
                pc = 0x800aa868;
            }
            0x800aa868 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[5].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[4], 0x20);
                if c.crbit(2) { pc = 0x800aa8bc; continue; }
                pc = 0x800aa880;
            }
            0x800aa880 => {
                c.r[0] = 0x80000000;
                c.r[0] = srw(c.r[0], c.r[4]);
                c.r[0] = c.r[5] & !c.r[0];
                c.r[4] = c.r[4].wrapping_mul(0xc);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[0]);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffc5e0);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.lr = 0x800aa8ac; f_8000540c(c);
                pc = 0x800aa8ac;
            }
            0x800aa8ac => {
                c.r[4] = 0x800b0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[4].wrapping_add(0xffffa8d4);
                c.lr = 0x800aa8bc; f_800d0cbc(c);
                pc = 0x800aa8bc;
            }
            0x800aa8bc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa7fc: bad pc {:#x}", pc),
        }
    }
}

/// PADTypeAndStatusCallback (0x800aa8d4, 812 bytes)
#[inline(never)]
pub fn f_800aa8d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aa8d4) { return; }
        if c.prof { crate::prof::enter(c, 0x800aa8d4); p = true; }
    }
    f_800aa8d4_b(c, 0x800aa8d4);
    if p { crate::prof::exit(c, 0x800aa8d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aa8d4_at(c: &mut Cpu, pc: u32) { f_800aa8d4_b(c, pc) }
#[inline(never)]
fn f_800aa8d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aa8d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aa8d4, pc); }
        match pc {
            0x800aa8d4 => {
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xf;
                c.rc(c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x3c), c.r[31]);
                c.r[31] = 0x80000000;
                c.stw(c.r[1].wrapping_add(0x38), c.r[30]);
                c.r[30] = c.r[6].wrapping_add(0xffffc5d0);
                c.stw(c.r[1].wrapping_add(0x34), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x30), c.r[28]);
                c.r[29] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa08c));
                c.r[28] = srw(c.r[31], c.r[29]);
                c.r[3] = c.r[5] & !c.r[28];
                c.stw(c.r[13].wrapping_add(0xffffa08c), c.r[3]);
                c.r[5] = c.r[5] & c.r[28];
                c.r[3] = 0x1;
                if c.crbit(2) { pc = 0x800aa970; continue; }
                pc = 0x800aa920;
            }
            0x800aa920 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[4].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[3], 0x20);
                if c.crbit(2) { pc = 0x800aabe0; continue; }
                pc = 0x800aa938;
            }
            0x800aa938 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = srw(c.r[31], c.r[3]);
                c.r[4] = c.r[4] & !c.r[3];
                c.r[3] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.lr = 0x800aa95c; f_8000540c(c);
                pc = 0x800aa95c;
            }
            0x800aa95c => {
                c.r[4] = 0x800b0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[4].wrapping_add(0xffffa8d4);
                c.lr = 0x800aa96c; f_800d0cbc(c);
                pc = 0x800aa96c;
            }
            0x800aa96c => {
                pc = 0x800aabe0; continue;
                pc = 0x800aa970;
            }
            0x800aa970 => {
                c.r[6] = c.r[4].rotate_left(0) & 0xffffff00;
                c.r[4] = c.r[4].rotate_left(0) & 0x18000000;
                c.r[0] = c.r[4].wrapping_add(0xf8000000);
                c.r[4] = c.r[29].rotate_left(2) & 0xfffffffc;
                c.cmpu(0, c.r[0], 0x0);
                c.stw(c.r[30].wrapping_add(c.r[4]), c.r[6]);
                if !c.crbit(2) { pc = 0x800aa994; continue; }
                pc = 0x800aa98c;
            }
            0x800aa98c => {
                c.r[0] = c.r[6].rotate_left(0) & 0x1000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800aa9e8; continue; }
                pc = 0x800aa994;
            }
            0x800aa994 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[5].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[4], 0x20);
                if c.crbit(2) { pc = 0x800aabe0; continue; }
                pc = 0x800aa9ac;
            }
            0x800aa9ac => {
                c.r[3] = 0x80000000;
                c.r[0] = c.r[4].wrapping_mul(0xc);
                c.r[3] = srw(c.r[3], c.r[4]);
                c.r[4] = c.r[5] & !c.r[3];
                c.r[3] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.lr = 0x800aa9d4; f_8000540c(c);
                pc = 0x800aa9d4;
            }
            0x800aa9d4 => {
                c.r[4] = 0x800b0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[4].wrapping_add(0xffffa8d4);
                c.lr = 0x800aa9e4; f_800d0cbc(c);
                pc = 0x800aa9e4;
            }
            0x800aa9e4 => {
                pc = 0x800aabe0; continue;
                pc = 0x800aa9e8;
            }
            0x800aa9e8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81f8));
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(0) { pc = 0x800aaa74; continue; }
                pc = 0x800aa9f4;
            }
            0x800aa9f4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = c.r[1].wrapping_add(0x1c);
                c.r[0] = c.r[0] | c.r[28];
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[0]);
                c.lr = 0x800aaa0c; f_800d05a4(c);
                pc = 0x800aaa0c;
            }
            0x800aaa0c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81f4));
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = c.r[0] | 0x400000;
                c.lr = 0x800aaa1c; f_800d0338(c);
                pc = 0x800aaa1c;
            }
            0x800aaa1c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.lr = 0x800aaa24; f_800d03c8(c);
                pc = 0x800aaa24;
            }
            0x800aaa24 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[4].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[3], 0x20);
                if c.crbit(2) { pc = 0x800aabe0; continue; }
                pc = 0x800aaa3c;
            }
            0x800aaa3c => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = srw(c.r[31], c.r[3]);
                c.r[4] = c.r[4] & !c.r[3];
                c.r[3] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.lr = 0x800aaa60; f_8000540c(c);
                pc = 0x800aaa60;
            }
            0x800aaa60 => {
                c.r[4] = 0x800b0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[4].wrapping_add(0xffffa8d4);
                c.lr = 0x800aaa70; f_800d0cbc(c);
                pc = 0x800aaa70;
            }
            0x800aaa70 => {
                pc = 0x800aabe0; continue;
                pc = 0x800aaa74;
            }
            0x800aaa74 => {
                c.r[0] = c.r[6].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aaa84; continue; }
                pc = 0x800aaa7c;
            }
            0x800aaa7c => {
                c.r[0] = c.r[6].rotate_left(0) & 0x4000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aaaf4; continue; }
                pc = 0x800aaa84;
            }
            0x800aaa84 => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x800aaac0; continue; }
                pc = 0x800aaa8c;
            }
            0x800aaa8c => {
                c.r[0] = c.r[29].wrapping_mul(0xc);
                c.r[3] = 0x800b0000;
                c.r[6] = c.r[30].wrapping_add(c.r[0]);
                c.r[8] = c.r[3].wrapping_add(0xffffa678);
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = c.r[13].wrapping_add(0xffff8204);
                c.r[5] = 0x3;
                c.r[7] = 0xa;
                c.r[10] = 0x0;
                c.r[9] = 0x0;
                c.r[6] = c.r[6].wrapping_add(0x10);
                c.lr = 0x800aaabc; f_800d06f4(c);
                pc = 0x800aaabc;
            }
            0x800aaabc => {
                pc = 0x800aab7c; continue;
                pc = 0x800aaac0;
            }
            0x800aaac0 => {
                c.r[0] = c.r[29].wrapping_mul(0xc);
                c.r[3] = 0x800b0000;
                c.r[6] = c.r[30].wrapping_add(c.r[0]);
                c.r[8] = c.r[3].wrapping_add(0xffffa678);
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = c.r[13].wrapping_add(0xffff8200);
                c.r[5] = 0x1;
                c.r[7] = 0xa;
                c.r[10] = 0x0;
                c.r[9] = 0x0;
                c.r[6] = c.r[6].wrapping_add(0x10);
                c.lr = 0x800aaaf0; f_800d06f4(c);
                pc = 0x800aaaf0;
            }
            0x800aaaf0 => {
                pc = 0x800aab7c; continue;
                pc = 0x800aaaf4;
            }
            0x800aaaf4 => {
                c.r[0] = c.r[6].rotate_left(0) & 0x100000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aab7c; continue; }
                pc = 0x800aaafc;
            }
            0x800aaafc => {
                c.r[0] = c.r[6].rotate_left(0) & 0x80000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800aab7c; continue; }
                pc = 0x800aab04;
            }
            0x800aab04 => {
                c.r[0] = c.r[6].rotate_left(0) & 0x40000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800aab7c; continue; }
                pc = 0x800aab0c;
            }
            0x800aab0c => {
                c.r[0] = c.r[6].rotate_left(0) & 0x40000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aab48; continue; }
                pc = 0x800aab14;
            }
            0x800aab14 => {
                c.r[0] = c.r[29].wrapping_mul(0xc);
                c.r[3] = 0x800b0000;
                c.r[6] = c.r[30].wrapping_add(c.r[0]);
                c.r[8] = c.r[3].wrapping_add(0xffffa678);
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = c.r[13].wrapping_add(0xffff8200);
                c.r[5] = 0x1;
                c.r[7] = 0xa;
                c.r[10] = 0x0;
                c.r[9] = 0x0;
                c.r[6] = c.r[6].wrapping_add(0x10);
                c.lr = 0x800aab44; f_800d06f4(c);
                pc = 0x800aab44;
            }
            0x800aab44 => {
                pc = 0x800aab7c; continue;
                pc = 0x800aab48;
            }
            0x800aab48 => {
                c.r[0] = c.r[29].wrapping_mul(0xc);
                c.r[3] = 0x800b0000;
                c.r[4] = c.r[30].wrapping_add(c.r[4]);
                c.r[6] = c.r[30].wrapping_add(c.r[0]);
                c.r[8] = c.r[3].wrapping_add(0xffffa7fc);
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[5] = 0x3;
                c.r[7] = 0x8;
                c.r[10] = 0x0;
                c.r[9] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x40);
                c.r[6] = c.r[6].wrapping_add(0x10);
                c.lr = 0x800aab7c; f_800d06f4(c);
                pc = 0x800aab7c;
            }
            0x800aab7c => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800aabe0; continue; }
                pc = 0x800aab84;
            }
            0x800aab84 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa098));
                c.r[0] = c.r[5].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[0] = c.r[3] | c.r[28];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.stw(c.r[13].wrapping_add(0xffffa098), c.r[0]);
                c.cmps(0, c.r[4], 0x20);
                if c.crbit(2) { pc = 0x800aabe0; continue; }
                pc = 0x800aaba8;
            }
            0x800aaba8 => {
                c.r[3] = 0x80000000;
                c.r[0] = c.r[4].wrapping_mul(0xc);
                c.r[3] = srw(c.r[3], c.r[4]);
                c.r[4] = c.r[5] & !c.r[3];
                c.r[3] = c.r[30].wrapping_add(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.lr = 0x800aabd0; f_8000540c(c);
                pc = 0x800aabd0;
            }
            0x800aabd0 => {
                c.r[4] = 0x800b0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[4].wrapping_add(0xffffa8d4);
                c.lr = 0x800aabe0; f_800d0cbc(c);
                pc = 0x800aabe0;
            }
            0x800aabe0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x38));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x30));
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aa8d4: bad pc {:#x}", pc),
        }
    }
}

/// PADReceiveCheckCallback (0x800aac00, 308 bytes)
#[inline(never)]
pub fn f_800aac00(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aac00) { return; }
        if c.prof { crate::prof::enter(c, 0x800aac00); p = true; }
    }
    f_800aac00_b(c, 0x800aac00);
    if p { crate::prof::exit(c, 0x800aac00); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aac00_at(c: &mut Cpu, pc: u32) { f_800aac00_b(c, pc) }
#[inline(always)]
fn f_800aac00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aac00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aac00, pc); }
        match pc {
            0x800aac00 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x80000000;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = srw(c.r[3], c.r[31]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[0] = c.r[0] & c.r[29];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aad18; continue; }
                pc = 0x800aac30;
            }
            0x800aac30 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[6] = !(c.r[29] | c.r[29]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[0] = c.r[4].rotate_left(0) & 0xf;
                c.rc(c.r[0]);
                c.r[5] = c.r[5] & c.r[6];
                c.r[3] = c.r[3] & c.r[6];
                c.stw(c.r[13].wrapping_add(0xffffa090), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa094), c.r[3]);
                c.r[3] = c.r[4].rotate_left(0) & 0xffffff00;
                if !c.crbit(2) { pc = 0x800aacc0; continue; }
                pc = 0x800aac58;
            }
            0x800aac58 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aacc0; continue; }
                pc = 0x800aac60;
            }
            0x800aac60 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x100000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aacc0; continue; }
                pc = 0x800aac68;
            }
            0x800aac68 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x40000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800aacc0; continue; }
                pc = 0x800aac70;
            }
            0x800aac70 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x4000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800aacc0; continue; }
                pc = 0x800aac78;
            }
            0x800aac78 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x80000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800aacc0; continue; }
                pc = 0x800aac80;
            }
            0x800aac80 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x40000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800aacc0; continue; }
                pc = 0x800aac88;
            }
            0x800aac88 => {
                c.r[4] = c.r[31].wrapping_mul(0xc);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffc5e0);
                c.r[3] = 0x800b0000;
                c.r[6] = c.r[0].wrapping_add(c.r[4]);
                c.r[8] = c.r[3].wrapping_add(0xffffa73c);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[13].wrapping_add(0xffff8200);
                c.r[5] = 0x1;
                c.r[7] = 0xa;
                c.r[10] = 0x0;
                c.r[9] = 0x0;
                c.lr = 0x800aacbc; f_800d06f4(c);
                pc = 0x800aacbc;
            }
            0x800aacbc => {
                pc = 0x800aad18; continue;
                pc = 0x800aacc0;
            }
            0x800aacc0 => {
                c.lr = 0x800aacc4; f_8009df3c(c);
                pc = 0x800aacc4;
            }
            0x800aacc4 => {
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.lr = 0x800aacd0; f_800d0464(c);
                pc = 0x800aacd0;
            }
            0x800aacd0 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[8] = !(c.r[29] | c.r[29]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa098));
                c.r[7] = c.r[6] & c.r[8];
                c.r[6] = c.r[5] & c.r[8];
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[7]);
                c.r[5] = c.r[4] & c.r[8];
                c.r[0] = c.r[0] & c.r[8];
                c.stw(c.r[13].wrapping_add(0xffffa090), c.r[6]);
                c.r[4] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa094), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa098), c.r[0]);
                c.lr = 0x800aad10; f_800a115c(c);
                pc = 0x800aad10;
            }
            0x800aad10 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800aad18; f_8009df64(c);
                pc = 0x800aad18;
            }
            0x800aad18 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aac00: bad pc {:#x}", pc),
        }
    }
}

/// PADReset (0x800aad34, 256 bytes)
#[inline(never)]
pub fn f_800aad34(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aad34) { return; }
        if c.prof { crate::prof::enter(c, 0x800aad34); p = true; }
    }
    f_800aad34_b(c, 0x800aad34);
    if p { crate::prof::exit(c, 0x800aad34); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aad34_at(c: &mut Cpu, pc: u32) { f_800aad34_b(c, pc) }
#[inline(always)]
fn f_800aad34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aad34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aad34, pc); }
        match pc {
            0x800aad34 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800aad50; f_8009df3c(c);
                pc = 0x800aad50;
            }
            0x800aad50 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa098));
                c.r[7] = 0x0;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[30] = c.r[30] | c.r[4];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[5] = c.r[5] | c.r[0];
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81f8));
                c.r[30] = c.r[30] & !c.r[5];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[6] = c.r[4] | c.r[30];
                c.stw(c.r[13].wrapping_add(0xffffa098), c.r[7]);
                c.r[4] = c.r[5] & !c.r[30];
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[6]);
                c.cmpu(0, c.r[0], 0x4);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[4]);
                c.r[3] = c.r[0] & c.r[5];
                if !c.crbit(2) { pc = 0x800aadac; continue; }
                pc = 0x800aada0;
            }
            0x800aada0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa08c));
                c.r[0] = c.r[0] | c.r[30];
                c.stw(c.r[13].wrapping_add(0xffffa08c), c.r[0]);
                pc = 0x800aadac;
            }
            0x800aadac => {
                c.lr = 0x800aadb0; f_800d0464(c);
                pc = 0x800aadb0;
            }
            0x800aadb0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[0], 0x20);
                if !c.crbit(2) { pc = 0x800aae10; continue; }
                pc = 0x800aadbc;
            }
            0x800aadbc => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[5].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[4], 0x20);
                if c.crbit(2) { pc = 0x800aae10; continue; }
                pc = 0x800aadd4;
            }
            0x800aadd4 => {
                c.r[0] = 0x80000000;
                c.r[0] = srw(c.r[0], c.r[4]);
                c.r[0] = c.r[5] & !c.r[0];
                c.r[4] = c.r[4].wrapping_mul(0xc);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[0]);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffc5e0);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.lr = 0x800aae00; f_8000540c(c);
                pc = 0x800aae00;
            }
            0x800aae00 => {
                c.r[4] = 0x800b0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[4].wrapping_add(0xffffa8d4);
                c.lr = 0x800aae10; f_800d0cbc(c);
                pc = 0x800aae10;
            }
            0x800aae10 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800aae18; f_8009df64(c);
                pc = 0x800aae18;
            }
            0x800aae18 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aad34: bad pc {:#x}", pc),
        }
    }
}

/// PADRecalibrate (0x800aae34, 260 bytes)
#[inline(never)]
pub fn f_800aae34(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aae34) { return; }
        if c.prof { crate::prof::enter(c, 0x800aae34); p = true; }
    }
    f_800aae34_b(c, 0x800aae34);
    if p { crate::prof::exit(c, 0x800aae34); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aae34_at(c: &mut Cpu, pc: u32) { f_800aae34_b(c, pc) }
#[inline(always)]
fn f_800aae34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aae34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aae34, pc); }
        match pc {
            0x800aae34 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800aae50; f_8009df3c(c);
                pc = 0x800aae50;
            }
            0x800aae50 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa098));
                c.r[7] = 0x0;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[4] = 0x80000000;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[30] = c.r[30] | c.r[6];
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[5] = c.r[5] | c.r[0];
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x30e3));
                c.r[30] = c.r[30] & !c.r[5];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[4] = c.r[6] | c.r[30];
                c.stw(c.r[13].wrapping_add(0xffffa098), c.r[7]);
                c.r[0] = c.r[0].rotate_left(0) & 0x40;
                c.rc(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[4]);
                c.r[4] = c.r[5] & !c.r[30];
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[4]);
                c.r[3] = c.r[6] & c.r[5];
                if !c.crbit(2) { pc = 0x800aaeb0; continue; }
                pc = 0x800aaea4;
            }
            0x800aaea4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa08c));
                c.r[0] = c.r[0] | c.r[30];
                c.stw(c.r[13].wrapping_add(0xffffa08c), c.r[0]);
                pc = 0x800aaeb0;
            }
            0x800aaeb0 => {
                c.lr = 0x800aaeb4; f_800d0464(c);
                pc = 0x800aaeb4;
            }
            0x800aaeb4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[0], 0x20);
                if !c.crbit(2) { pc = 0x800aaf14; continue; }
                pc = 0x800aaec0;
            }
            0x800aaec0 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[5].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[4], 0x20);
                if c.crbit(2) { pc = 0x800aaf14; continue; }
                pc = 0x800aaed8;
            }
            0x800aaed8 => {
                c.r[0] = 0x80000000;
                c.r[0] = srw(c.r[0], c.r[4]);
                c.r[0] = c.r[5] & !c.r[0];
                c.r[4] = c.r[4].wrapping_mul(0xc);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[0]);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffc5e0);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.lr = 0x800aaf04; f_8000540c(c);
                pc = 0x800aaf04;
            }
            0x800aaf04 => {
                c.r[4] = 0x800b0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[4].wrapping_add(0xffffa8d4);
                c.lr = 0x800aaf14; f_800d0cbc(c);
                pc = 0x800aaf14;
            }
            0x800aaf14 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800aaf1c; f_8009df64(c);
                pc = 0x800aaf1c;
            }
            0x800aaf1c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aae34: bad pc {:#x}", pc),
        }
    }
}

/// PADInit (0x800aaf38, 536 bytes)
#[inline(never)]
pub fn f_800aaf38(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800aaf38) { return; }
        if c.prof { crate::prof::enter(c, 0x800aaf38); p = true; }
    }
    f_800aaf38_b(c, 0x800aaf38);
    if p { crate::prof::exit(c, 0x800aaf38); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800aaf38_at(c: &mut Cpu, pc: u32) { f_800aaf38_b(c, pc) }
#[inline(never)]
fn f_800aaf38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800aaf38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800aaf38, pc); }
        match pc {
            0x800aaf38 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3].wrapping_add(0xffffc5d0);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa080));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800aaf64; continue; }
                pc = 0x800aaf5c;
            }
            0x800aaf5c => {
                c.r[3] = 0x1;
                pc = 0x800ab13c; continue;
                pc = 0x800aaf64;
            }
            0x800aaf64 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81e8));
                c.lr = 0x800aaf6c; f_8009a250(c);
                pc = 0x800aaf6c;
            }
            0x800aaf6c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa0a4));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800aaf7c; continue; }
                pc = 0x800aaf78;
            }
            0x800aaf78 => {
                c.lr = 0x800aaf7c; f_800ab5b4(c);
                pc = 0x800aaf7c;
            }
            0x800aaf7c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa238));
                c.r[3] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa080), c.r[3]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800ab018; continue; }
                pc = 0x800aaf90;
            }
            0x800aaf90 => {
                c.lr = 0x800aaf94; crate::stubs::f_800a2778(c);
                pc = 0x800aaf94;
            }
            0x800aaf94 => {
                c.r[25] = c.r[4].wrapping_add(0x0);
                c.r[26] = c.r[3].wrapping_add(0x0);
                c.r[5] = 0x10;
                c.lr = 0x800aafa4; f_800c4c98(c);
                pc = 0x800aafa4;
            }
            0x800aafa4 => {
                c.r[5] = 0x10000;
                c.r[27] = c.r[5].wrapping_add(0xffffffff);
                c.r[28] = 0x0;
                c.r[6] = c.r[4] & c.r[27];
                c.r[4] = c.r[25] & c.r[27];
                c.r[5] = c.r[3] & c.r[28];
                c.r[0] = c.r[26] & c.r[28];
                c.r[29] = c.addc(c.r[4], c.r[6], 0);
                c.r[3] = c.r[26].wrapping_add(0x0);
                c.r[4] = c.r[25].wrapping_add(0x0);
                { let ci = c.ca(); c.r[30] = c.addc(c.r[0], c.r[5], ci); }
                c.r[5] = 0x20;
                c.lr = 0x800aafd8; f_800c4c98(c);
                pc = 0x800aafd8;
            }
            0x800aafd8 => {
                c.r[4] = c.r[4] & c.r[27];
                c.r[0] = c.r[3] & c.r[28];
                c.r[29] = c.addc(c.r[4], c.r[29], 0);
                c.r[3] = c.r[26].wrapping_add(0x0);
                c.r[4] = c.r[25].wrapping_add(0x0);
                { let ci = c.ca(); c.r[30] = c.addc(c.r[0], c.r[30], ci); }
                c.r[5] = 0x30;
                c.lr = 0x800aaff8; f_800c4c98(c);
                pc = 0x800aaff8;
            }
            0x800aaff8 => {
                c.r[0] = c.r[4] & c.r[27];
                c.r[5] = c.addc(c.r[0], c.r[29], 0);
                c.r[0] = 0xf0000000;
                c.r[4] = 0x3fff;
                c.stw(c.r[13].wrapping_add(0xffffa08c), c.r[0]);
                c.r[0] = c.r[5] & c.r[4];
                c.r[3] = 0x80000000;
                c.sth(c.r[3].wrapping_add(0x30e0), c.r[0]);
                pc = 0x800ab018;
            }
            0x800ab018 => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x30e0));
                c.r[0] = c.r[0].rotate_left(8) & 0x3fff00;
                c.r[0] = c.r[0] | 0x4d000000;
                c.stw(c.r[31].wrapping_add(0x40), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x30e0));
                c.r[0] = c.r[0].rotate_left(8) & 0x3fff00;
                c.r[0] = c.r[0] | 0x4d400000;
                c.stw(c.r[31].wrapping_add(0x44), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x30e0));
                c.r[0] = c.r[0].rotate_left(8) & 0x3fff00;
                c.r[0] = c.r[0] | 0x4d800000;
                c.stw(c.r[31].wrapping_add(0x48), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x30e0));
                c.r[0] = c.r[0].rotate_left(8) & 0x3fff00;
                c.r[0] = c.r[0] | 0x4dc00000;
                c.stw(c.r[31].wrapping_add(0x4c), c.r[0]);
                c.lr = 0x800ab060; f_800d104c(c);
                pc = 0x800ab060;
            }
            0x800ab060 => {
                c.r[3] = 0x80310000;
                c.r[3] = c.r[3].wrapping_add(0x2500);
                c.lr = 0x800ab06c; f_8009fe38(c);
                pc = 0x800ab06c;
            }
            0x800ab06c => {
                c.r[28] = 0xf0000000;
                c.lr = 0x800ab074; f_8009df3c(c);
                pc = 0x800ab074;
            }
            0x800ab074 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa098));
                c.r[7] = 0x0;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[27] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[28] = c.r[28] | c.r[4];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[5] = c.r[5] | c.r[0];
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81f8));
                c.r[28] = c.r[28] & !c.r[5];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[6] = c.r[4] | c.r[28];
                c.stw(c.r[13].wrapping_add(0xffffa098), c.r[7]);
                c.r[4] = c.r[5] & !c.r[28];
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[6]);
                c.cmpu(0, c.r[0], 0x4);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[4]);
                c.r[3] = c.r[0] & c.r[5];
                if !c.crbit(2) { pc = 0x800ab0d0; continue; }
                pc = 0x800ab0c4;
            }
            0x800ab0c4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa08c));
                c.r[0] = c.r[0] | c.r[28];
                c.stw(c.r[13].wrapping_add(0xffffa08c), c.r[0]);
                pc = 0x800ab0d0;
            }
            0x800ab0d0 => {
                c.lr = 0x800ab0d4; f_800d0464(c);
                pc = 0x800ab0d4;
            }
            0x800ab0d4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[0], 0x20);
                if !c.crbit(2) { pc = 0x800ab130; continue; }
                pc = 0x800ab0e0;
            }
            0x800ab0e0 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[5].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[4], 0x20);
                if c.crbit(2) { pc = 0x800ab130; continue; }
                pc = 0x800ab0f8;
            }
            0x800ab0f8 => {
                c.r[3] = 0x80000000;
                c.r[0] = c.r[4].wrapping_mul(0xc);
                c.r[3] = srw(c.r[3], c.r[4]);
                c.r[4] = c.r[5] & !c.r[3];
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.lr = 0x800ab120; f_8000540c(c);
                pc = 0x800ab120;
            }
            0x800ab120 => {
                c.r[4] = 0x800b0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[4].wrapping_add(0xffffa8d4);
                c.lr = 0x800ab130; f_800d0cbc(c);
                pc = 0x800ab130;
            }
            0x800ab130 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x800ab138; f_8009df64(c);
                pc = 0x800ab138;
            }
            0x800ab138 => {
                c.r[3] = 0x1;
                pc = 0x800ab13c;
            }
            0x800ab13c => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800aaf38: bad pc {:#x}", pc),
        }
    }
}

/// PADRead (0x800ab150, 940 bytes)
#[inline(never)]
pub fn f_800ab150(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ab150) { return; }
        if c.prof { crate::prof::enter(c, 0x800ab150); p = true; }
    }
    f_800ab150_b(c, 0x800ab150);
    if p { crate::prof::exit(c, 0x800ab150); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ab150_at(c: &mut Cpu, pc: u32) { f_800ab150_b(c, pc) }
#[inline(never)]
fn f_800ab150_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ab150;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ab150, pc); }
        match pc {
            0x800ab150 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x20); for n in 20..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800ab168; f_8009df3c(c);
                pc = 0x800ab168;
            }
            0x800ab168 => {
                c.r[21] = 0x0;
                c.r[0] = c.r[21].wrapping_mul(0xc);
                c.r[4] = 0x80400000;
                c.r[26] = c.r[4].wrapping_add(0xffffc5e0);
                c.r[4] = 0x800b0000;
                c.r[5] = 0x800b0000;
                c.r[6] = 0x800b0000;
                c.r[24] = c.r[26].wrapping_add(c.r[0]);
                c.r[22] = c.r[3].wrapping_add(0x0);
                c.r[30] = c.r[4].wrapping_add(0xffffa73c);
                c.r[29] = c.r[5].wrapping_add(0xffffac00);
                c.r[28] = c.r[6].wrapping_add(0xffffa8d4);
                c.r[20] = 0x0;
                c.r[27] = 0x80000000;
                pc = 0x800ab1a0;
            }
            0x800ab1a0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa098));
                c.r[23] = srw(c.r[27], c.r[21]);
                c.r[0] = c.r[0] & c.r[23];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab284; continue; }
                pc = 0x800ab1b0;
            }
            0x800ab1b0 => {
                c.lr = 0x800ab1b4; f_8009df3c(c);
                pc = 0x800ab1b4;
            }
            0x800ab1b4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[6] = 0x0;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[25] = c.r[3] | c.r[3];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa098));
                c.r[4] = c.r[4] | c.r[0];
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[7] = c.r[5] & !c.r[4];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[4] = c.r[0] | c.r[7];
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81f8));
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[4]);
                c.r[4] = c.r[5] & !c.r[7];
                c.cmpu(0, c.r[0], 0x4);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.stw(c.r[13].wrapping_add(0xffffa098), c.r[6]);
                c.r[3] = c.r[0] & c.r[5];
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[4]);
                if !c.crbit(2) { pc = 0x800ab20c; continue; }
                pc = 0x800ab200;
            }
            0x800ab200 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa08c));
                c.r[0] = c.r[0] | c.r[7];
                c.stw(c.r[13].wrapping_add(0xffffa08c), c.r[0]);
                pc = 0x800ab20c;
            }
            0x800ab20c => {
                c.lr = 0x800ab210; f_800d0464(c);
                pc = 0x800ab210;
            }
            0x800ab210 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[0], 0x20);
                if !c.crbit(2) { pc = 0x800ab260; continue; }
                pc = 0x800ab21c;
            }
            0x800ab21c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[4].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[3], 0x20);
                if c.crbit(2) { pc = 0x800ab260; continue; }
                pc = 0x800ab234;
            }
            0x800ab234 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = srw(c.r[27], c.r[3]);
                c.r[4] = c.r[4] & !c.r[3];
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[4]);
                c.r[3] = c.r[26].wrapping_add(c.r[0]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.lr = 0x800ab254; f_8000540c(c);
                pc = 0x800ab254;
            }
            0x800ab254 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x800ab260; f_800d0cbc(c);
                pc = 0x800ab260;
            }
            0x800ab260 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x800ab268; f_8009df64(c);
                pc = 0x800ab268;
            }
            0x800ab268 => {
                c.r[0] = 0xfffffffe;
                c.stb(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = 0xa;
                c.lr = 0x800ab280; f_8000540c(c);
                pc = 0x800ab280;
            }
            0x800ab280 => {
                pc = 0x800ab4c8; continue;
                pc = 0x800ab284;
            }
            0x800ab284 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[0] & c.r[23];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ab29c; continue; }
                pc = 0x800ab290;
            }
            0x800ab290 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[0], c.r[21]);
                if !c.crbit(2) { pc = 0x800ab2b8; continue; }
                pc = 0x800ab29c;
            }
            0x800ab29c => {
                c.r[0] = 0xfffffffe;
                c.stb(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = 0xa;
                c.lr = 0x800ab2b4; f_8000540c(c);
                pc = 0x800ab2b4;
            }
            0x800ab2b4 => {
                pc = 0x800ab4c8; continue;
                pc = 0x800ab2b8;
            }
            0x800ab2b8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[0] = c.r[0] & c.r[23];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ab2e0; continue; }
                pc = 0x800ab2c4;
            }
            0x800ab2c4 => {
                c.r[0] = 0xffffffff;
                c.stb(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = 0xa;
                c.lr = 0x800ab2dc; f_8000540c(c);
                pc = 0x800ab2dc;
            }
            0x800ab2dc => {
                pc = 0x800ab4c8; continue;
                pc = 0x800ab2e0;
            }
            0x800ab2e0 => {
                c.r[3] = c.r[21] | c.r[21];
                c.lr = 0x800ab2e8; f_800cf728(c);
                pc = 0x800ab2e8;
            }
            0x800ab2e8 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800ab30c; continue; }
                pc = 0x800ab2f0;
            }
            0x800ab2f0 => {
                c.r[0] = 0xfffffffd;
                c.stb(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = 0xa;
                c.lr = 0x800ab308; f_8000540c(c);
                pc = 0x800ab308;
            }
            0x800ab308 => {
                pc = 0x800ab4c8; continue;
                pc = 0x800ab30c;
            }
            0x800ab30c => {
                c.r[3] = c.r[21] | c.r[21];
                c.lr = 0x800ab314; f_800d02bc(c);
                pc = 0x800ab314;
            }
            0x800ab314 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab3e4; continue; }
                pc = 0x800ab31c;
            }
            0x800ab31c => {
                c.r[3] = c.r[21].wrapping_add(0x0);
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.lr = 0x800ab328; f_800d05a4(c);
                pc = 0x800ab328;
            }
            0x800ab328 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[0] = c.r[0] & c.r[23];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab370; continue; }
                pc = 0x800ab334;
            }
            0x800ab334 => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = 0xa;
                c.lr = 0x800ab34c; f_8000540c(c);
                pc = 0x800ab34c;
            }
            0x800ab34c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[0] = c.r[3] & c.r[23];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ab4c8; continue; }
                pc = 0x800ab358;
            }
            0x800ab358 => {
                c.r[0] = c.r[3] | c.r[23];
                c.stw(c.r[13].wrapping_add(0xffffa094), c.r[0]);
                c.r[3] = c.r[21].wrapping_add(0x0);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.lr = 0x800ab36c; f_800d0cbc(c);
                pc = 0x800ab36c;
            }
            0x800ab36c => {
                pc = 0x800ab4c8; continue;
                pc = 0x800ab370;
            }
            0x800ab370 => {
                c.lr = 0x800ab374; f_8009df3c(c);
                pc = 0x800ab374;
            }
            0x800ab374 => {
                c.r[25] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[23].wrapping_add(0x0);
                c.lr = 0x800ab380; f_800d0464(c);
                pc = 0x800ab380;
            }
            0x800ab380 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[8] = !(c.r[23] | c.r[23]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[3] = c.r[21] | c.r[21];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa098));
                c.r[7] = c.r[6] & c.r[8];
                c.r[6] = c.r[5] & c.r[8];
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[7]);
                c.r[5] = c.r[4] & c.r[8];
                c.r[0] = c.r[0] & c.r[8];
                c.stw(c.r[13].wrapping_add(0xffffa090), c.r[6]);
                c.r[4] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa094), c.r[5]);
                c.stw(c.r[13].wrapping_add(0xffffa098), c.r[0]);
                c.lr = 0x800ab3c0; f_800a115c(c);
                pc = 0x800ab3c0;
            }
            0x800ab3c0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x800ab3c8; f_8009df64(c);
                pc = 0x800ab3c8;
            }
            0x800ab3c8 => {
                c.r[0] = 0xffffffff;
                c.stb(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = 0xa;
                c.lr = 0x800ab3e0; f_8000540c(c);
                pc = 0x800ab3e0;
            }
            0x800ab3e0 => {
                pc = 0x800ab4c8; continue;
                pc = 0x800ab3e4;
            }
            0x800ab3e4 => {
                c.r[3] = c.r[21] | c.r[21];
                c.lr = 0x800ab3ec; f_800d0af8(c);
                pc = 0x800ab3ec;
            }
            0x800ab3ec => {
                c.r[0] = c.r[3].rotate_left(0) & 0x20000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ab3f8; continue; }
                pc = 0x800ab3f4;
            }
            0x800ab3f4 => {
                c.r[20] = c.r[20] | c.r[23];
                pc = 0x800ab3f8;
            }
            0x800ab3f8 => {
                c.r[3] = c.r[21].wrapping_add(0x0);
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.lr = 0x800ab404; f_800d05a4(c);
                pc = 0x800ab404;
            }
            0x800ab404 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800ab428; continue; }
                pc = 0x800ab40c;
            }
            0x800ab40c => {
                c.r[0] = 0xfffffffd;
                c.stb(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = 0xa;
                c.lr = 0x800ab424; f_8000540c(c);
                pc = 0x800ab424;
            }
            0x800ab424 => {
                pc = 0x800ab4c8; continue;
                pc = 0x800ab428;
            }
            0x800ab428 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[0] = c.r[0].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab450; continue; }
                pc = 0x800ab434;
            }
            0x800ab434 => {
                c.r[0] = 0xfffffffd;
                c.stb(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = 0xa;
                c.lr = 0x800ab44c; f_8000540c(c);
                pc = 0x800ab44c;
            }
            0x800ab44c => {
                pc = 0x800ab4c8; continue;
                pc = 0x800ab450;
            }
            0x800ab450 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff81fc));
                c.r[3] = c.r[21].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                c.r[5] = c.r[1].wrapping_add(0x14);
                if true { let t = c.lr; c.lr = 0x800ab468; (c.dispatch)(c, t); }
                pc = 0x800ab468;
            }
            0x800ab468 => {
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(0) & 0x2000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab4b4; continue; }
                pc = 0x800ab474;
            }
            0x800ab474 => {
                c.r[0] = 0xfffffffd;
                c.stb(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = 0xa;
                c.lr = 0x800ab48c; f_8000540c(c);
                pc = 0x800ab48c;
            }
            0x800ab48c => {
                c.r[3] = c.r[21].wrapping_add(0x0);
                c.r[6] = c.r[24].wrapping_add(0x0);
                c.r[8] = c.r[30].wrapping_add(0x0);
                c.r[4] = c.r[13].wrapping_add(0xffff8200);
                c.r[5] = 0x1;
                c.r[7] = 0xa;
                c.r[10] = 0x0;
                c.r[9] = 0x0;
                c.lr = 0x800ab4b0; f_800d06f4(c);
                pc = 0x800ab4b0;
            }
            0x800ab4b0 => {
                pc = 0x800ab4c8; continue;
                pc = 0x800ab4b4;
            }
            0x800ab4b4 => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff7f;
                c.sth(c.r[31], c.r[0]);
                pc = 0x800ab4c8;
            }
            0x800ab4c8 => {
                c.r[21] = c.r[21].wrapping_add(0x1);
                c.cmps(0, c.r[21], 0x4);
                c.r[24] = c.r[24].wrapping_add(0xc);
                c.r[31] = c.r[31].wrapping_add(0xc);
                if c.crbit(0) { pc = 0x800ab1a0; continue; }
                pc = 0x800ab4dc;
            }
            0x800ab4dc => {
                c.r[3] = c.r[22] | c.r[22];
                c.lr = 0x800ab4e4; f_8009df64(c);
                pc = 0x800ab4e4;
            }
            0x800ab4e4 => {
                c.r[3] = c.r[20] | c.r[20];
                { let mut ea = c.r[1].wrapping_add(0x20); for n in 20..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[1] = c.r[1].wrapping_add(0x50);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ab150: bad pc {:#x}", pc),
        }
    }
}

/// PADControlMotor (0x800ab4fc, 184 bytes)
#[inline(never)]
pub fn f_800ab4fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ab4fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800ab4fc); p = true; }
    }
    f_800ab4fc_b(c, 0x800ab4fc);
    if p { crate::prof::exit(c, 0x800ab4fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ab4fc_at(c: &mut Cpu, pc: u32) { f_800ab4fc_b(c, pc) }
#[inline(always)]
fn f_800ab4fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ab4fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ab4fc, pc); }
        match pc {
            0x800ab4fc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x800ab514; f_8009df3c(c);
                pc = 0x800ab514;
            }
            0x800ab514 => {
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[28] = 0x0;
                c.r[29] = 0x0;
                c.r[31] = 0x80000000;
                pc = 0x800ab524;
            }
            0x800ab524 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[0] = srw(c.r[31], c.r[29]);
                c.r[0] = c.r[3] & c.r[0];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab57c; continue; }
                pc = 0x800ab534;
            }
            0x800ab534 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800ab53c; f_800d0af8(c);
                pc = 0x800ab53c;
            }
            0x800ab53c => {
                c.r[0] = c.r[3].rotate_left(0) & 0x20000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ab57c; continue; }
                pc = 0x800ab544;
            }
            0x800ab544 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81f8));
                c.r[3] = c.lwz(c.r[27]);
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(0) { pc = 0x800ab560; continue; }
                pc = 0x800ab554;
            }
            0x800ab554 => {
                c.cmpu(0, c.r[3], 0x2);
                if !c.crbit(2) { pc = 0x800ab560; continue; }
                pc = 0x800ab55c;
            }
            0x800ab55c => {
                c.r[3] = 0x0;
                pc = 0x800ab560;
            }
            0x800ab560 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff81f4));
                c.r[0] = c.r[3].rotate_left(0) & 0x3;
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = c.r[4] | 0x400000;
                c.r[4] = c.r[4] | c.r[0];
                c.lr = 0x800ab578; f_800d0338(c);
                pc = 0x800ab578;
            }
            0x800ab578 => {
                c.r[28] = 0x1;
                pc = 0x800ab57c;
            }
            0x800ab57c => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.cmps(0, c.r[29], 0x4);
                c.r[27] = c.r[27].wrapping_add(0x4);
                if c.crbit(0) { pc = 0x800ab524; continue; }
                pc = 0x800ab58c;
            }
            0x800ab58c => {
                c.cmps(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x800ab598; continue; }
                pc = 0x800ab594;
            }
            0x800ab594 => {
                c.lr = 0x800ab598; f_800d034c(c);
                pc = 0x800ab598;
            }
            0x800ab598 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800ab5a0; f_8009df64(c);
                pc = 0x800ab5a0;
            }
            0x800ab5a0 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ab4fc: bad pc {:#x}", pc),
        }
    }
}

/// PADSetSpec (0x800ab5b4, 96 bytes)
#[inline(never)]
pub fn f_800ab5b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ab5b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800ab5b4); p = true; }
    }
    f_800ab5b4_b(c, 0x800ab5b4);
    if p { crate::prof::exit(c, 0x800ab5b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ab5b4_at(c: &mut Cpu, pc: u32) { f_800ab5b4_b(c, pc) }
#[inline(always)]
fn f_800ab5b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ab5b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ab5b4, pc); }
        match pc {
            0x800ab5b4 => {
                c.r[0] = 0x0;
                c.cmps(0, c.r[3], 0x1);
                c.stw(c.r[13].wrapping_add(0xffffa0a4), c.r[0]);
                if c.crbit(2) { pc = 0x800ab5f0; continue; }
                pc = 0x800ab5c4;
            }
            0x800ab5c4 => {
                if !c.crbit(0) { pc = 0x800ab5d4; continue; }
                pc = 0x800ab5c8;
            }
            0x800ab5c8 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x800ab5e0; continue; }
                pc = 0x800ab5d0;
            }
            0x800ab5d0 => {
                pc = 0x800ab60c; continue;
                pc = 0x800ab5d4;
            }
            0x800ab5d4 => {
                c.cmps(0, c.r[3], 0x6);
                if !c.crbit(0) { pc = 0x800ab60c; continue; }
                pc = 0x800ab5dc;
            }
            0x800ab5dc => {
                pc = 0x800ab600; continue;
                pc = 0x800ab5e0;
            }
            0x800ab5e0 => {
                c.r[4] = 0x800b0000;
                c.r[0] = c.r[4].wrapping_add(0xffffb614);
                c.stw(c.r[13].wrapping_add(0xffff81fc), c.r[0]);
                pc = 0x800ab60c; continue;
                pc = 0x800ab5f0;
            }
            0x800ab5f0 => {
                c.r[4] = 0x800b0000;
                c.r[0] = c.r[4].wrapping_add(0xffffb788);
                c.stw(c.r[13].wrapping_add(0xffff81fc), c.r[0]);
                pc = 0x800ab60c; continue;
                pc = 0x800ab600;
            }
            0x800ab600 => {
                c.r[4] = 0x800b0000;
                c.r[0] = c.r[4].wrapping_add(0xffffb8fc);
                c.stw(c.r[13].wrapping_add(0xffff81fc), c.r[0]);
                pc = 0x800ab60c;
            }
            0x800ab60c => {
                c.stw(c.r[13].wrapping_add(0xffff81f8), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ab5b4: bad pc {:#x}", pc),
        }
    }
}

/// SPEC0_MakeStatus (0x800ab614, 372 bytes)
#[inline(never)]
pub fn f_800ab614(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ab614) { return; }
        if c.prof { crate::prof::enter(c, 0x800ab614); p = true; }
    }
    f_800ab614_b(c, 0x800ab614);
    if p { crate::prof::exit(c, 0x800ab614); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ab614_at(c: &mut Cpu, pc: u32) { f_800ab614_b(c, pc) }
#[inline(always)]
fn f_800ab614_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ab614;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ab614, pc); }
        match pc {
            0x800ab614 => {
                c.r[3] = 0x0;
                c.sth(c.r[4], c.r[3]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab62c; continue; }
                pc = 0x800ab628;
            }
            0x800ab628 => {
                c.r[3] = 0x100;
                pc = 0x800ab62c;
            }
            0x800ab62c => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | c.r[3];
                c.sth(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x20;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab64c; continue; }
                pc = 0x800ab644;
            }
            0x800ab644 => {
                c.r[3] = 0x200;
                pc = 0x800ab650; continue;
                pc = 0x800ab64c;
            }
            0x800ab64c => {
                c.r[3] = 0x0;
                pc = 0x800ab650;
            }
            0x800ab650 => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | c.r[3];
                c.sth(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x100;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab670; continue; }
                pc = 0x800ab668;
            }
            0x800ab668 => {
                c.r[3] = 0x400;
                pc = 0x800ab674; continue;
                pc = 0x800ab670;
            }
            0x800ab670 => {
                c.r[3] = 0x0;
                pc = 0x800ab674;
            }
            0x800ab674 => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | c.r[3];
                c.sth(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab694; continue; }
                pc = 0x800ab68c;
            }
            0x800ab68c => {
                c.r[3] = 0x800;
                pc = 0x800ab698; continue;
                pc = 0x800ab694;
            }
            0x800ab694 => {
                c.r[3] = 0x0;
                pc = 0x800ab698;
            }
            0x800ab698 => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | c.r[3];
                c.sth(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x10;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab6b8; continue; }
                pc = 0x800ab6b0;
            }
            0x800ab6b0 => {
                c.r[6] = 0x1000;
                pc = 0x800ab6bc; continue;
                pc = 0x800ab6b8;
            }
            0x800ab6b8 => {
                c.r[6] = 0x0;
                pc = 0x800ab6bc;
            }
            0x800ab6bc => {
                c.r[3] = c.lhz(c.r[4]);
                c.r[0] = 0x0;
                c.r[3] = c.r[3] | c.r[6];
                c.sth(c.r[4], c.r[3]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[3] = c.r[3].rotate_left(16) & 0xffff;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x2), c.r[3]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[3] = c.r[3].rotate_left(8) & 0xff;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x3), c.r[3]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x4), c.r[3]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[3] = c.r[3].rotate_left(24) & 0xffffff;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x5), c.r[3]);
                c.r[3] = c.lwz(c.r[5]);
                c.r[3] = c.r[3].rotate_left(24) & 0xff;
                c.stb(c.r[4].wrapping_add(0x6), c.r[3]);
                c.r[3] = c.lwz(c.r[5]);
                c.stb(c.r[4].wrapping_add(0x7), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x8), c.r[0]);
                c.stb(c.r[4].wrapping_add(0x9), c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x6));
                c.cmpu(0, c.r[0], 0xaa);
                if c.crbit(0) { pc = 0x800ab73c; continue; }
                pc = 0x800ab730;
            }
            0x800ab730 => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | 0x40;
                c.sth(c.r[4], c.r[0]);
                pc = 0x800ab73c;
            }
            0x800ab73c => {
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x7));
                c.cmpu(0, c.r[0], 0xaa);
                if c.crbit(0) { pc = 0x800ab754; continue; }
                pc = 0x800ab748;
            }
            0x800ab748 => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | 0x20;
                c.sth(c.r[4], c.r[0]);
                pc = 0x800ab754;
            }
            0x800ab754 => {
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x2));
                c.r[0] = c.r[3].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x2), c.r[0]);
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x3));
                c.r[0] = c.r[3].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x3), c.r[0]);
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.r[3].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x5));
                c.r[0] = c.r[3].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x5), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ab614: bad pc {:#x}", pc),
        }
    }
}

/// SPEC1_MakeStatus (0x800ab788, 372 bytes)
#[inline(never)]
pub fn f_800ab788(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ab788) { return; }
        if c.prof { crate::prof::enter(c, 0x800ab788); p = true; }
    }
    f_800ab788_b(c, 0x800ab788);
    if p { crate::prof::exit(c, 0x800ab788); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ab788_at(c: &mut Cpu, pc: u32) { f_800ab788_b(c, pc) }
#[inline(always)]
fn f_800ab788_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ab788;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ab788, pc); }
        match pc {
            0x800ab788 => {
                c.r[3] = 0x0;
                c.sth(c.r[4], c.r[3]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x80;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab7a0; continue; }
                pc = 0x800ab79c;
            }
            0x800ab79c => {
                c.r[3] = 0x100;
                pc = 0x800ab7a0;
            }
            0x800ab7a0 => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | c.r[3];
                c.sth(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x100;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab7c0; continue; }
                pc = 0x800ab7b8;
            }
            0x800ab7b8 => {
                c.r[3] = 0x200;
                pc = 0x800ab7c4; continue;
                pc = 0x800ab7c0;
            }
            0x800ab7c0 => {
                c.r[3] = 0x0;
                pc = 0x800ab7c4;
            }
            0x800ab7c4 => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | c.r[3];
                c.sth(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x20;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab7e4; continue; }
                pc = 0x800ab7dc;
            }
            0x800ab7dc => {
                c.r[3] = 0x400;
                pc = 0x800ab7e8; continue;
                pc = 0x800ab7e4;
            }
            0x800ab7e4 => {
                c.r[3] = 0x0;
                pc = 0x800ab7e8;
            }
            0x800ab7e8 => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | c.r[3];
                c.sth(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x10;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab808; continue; }
                pc = 0x800ab800;
            }
            0x800ab800 => {
                c.r[3] = 0x800;
                pc = 0x800ab80c; continue;
                pc = 0x800ab808;
            }
            0x800ab808 => {
                c.r[3] = 0x0;
                pc = 0x800ab80c;
            }
            0x800ab80c => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | c.r[3];
                c.sth(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x200;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ab82c; continue; }
                pc = 0x800ab824;
            }
            0x800ab824 => {
                c.r[6] = 0x1000;
                pc = 0x800ab830; continue;
                pc = 0x800ab82c;
            }
            0x800ab82c => {
                c.r[6] = 0x0;
                pc = 0x800ab830;
            }
            0x800ab830 => {
                c.r[3] = c.lhz(c.r[4]);
                c.r[0] = 0x0;
                c.r[3] = c.r[3] | c.r[6];
                c.sth(c.r[4], c.r[3]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[3] = c.r[3].rotate_left(16) & 0xffff;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x2), c.r[3]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[3] = c.r[3].rotate_left(8) & 0xff;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x3), c.r[3]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x4), c.r[3]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[3] = c.r[3].rotate_left(24) & 0xffffff;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x5), c.r[3]);
                c.r[3] = c.lwz(c.r[5]);
                c.r[3] = c.r[3].rotate_left(24) & 0xff;
                c.stb(c.r[4].wrapping_add(0x6), c.r[3]);
                c.r[3] = c.lwz(c.r[5]);
                c.stb(c.r[4].wrapping_add(0x7), c.r[3]);
                c.stb(c.r[4].wrapping_add(0x8), c.r[0]);
                c.stb(c.r[4].wrapping_add(0x9), c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x6));
                c.cmpu(0, c.r[0], 0xaa);
                if c.crbit(0) { pc = 0x800ab8b0; continue; }
                pc = 0x800ab8a4;
            }
            0x800ab8a4 => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | 0x40;
                c.sth(c.r[4], c.r[0]);
                pc = 0x800ab8b0;
            }
            0x800ab8b0 => {
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x7));
                c.cmpu(0, c.r[0], 0xaa);
                if c.crbit(0) { pc = 0x800ab8c8; continue; }
                pc = 0x800ab8bc;
            }
            0x800ab8bc => {
                c.r[0] = c.lhz(c.r[4]);
                c.r[0] = c.r[0] | 0x20;
                c.sth(c.r[4], c.r[0]);
                pc = 0x800ab8c8;
            }
            0x800ab8c8 => {
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x2));
                c.r[0] = c.r[3].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x2), c.r[0]);
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x3));
                c.r[0] = c.r[3].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x3), c.r[0]);
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.r[3].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x5));
                c.r[0] = c.r[3].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x5), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ab788: bad pc {:#x}", pc),
        }
    }
}

/// SPEC2_MakeStatus (0x800ab8fc, 1016 bytes)
#[inline(never)]
pub fn f_800ab8fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ab8fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800ab8fc); p = true; }
    }
    f_800ab8fc_b(c, 0x800ab8fc);
    if p { crate::prof::exit(c, 0x800ab8fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ab8fc_at(c: &mut Cpu, pc: u32) { f_800ab8fc_b(c, pc) }
#[inline(never)]
fn f_800ab8fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ab8fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ab8fc, pc); }
        match pc {
            0x800ab8fc => {
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(16) & 0x3fff;
                c.sth(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0].rotate_left(24) & 0xffffff;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x3), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81f4));
                c.r[0] = c.r[0].rotate_left(0) & 0x700;
                c.cmps(0, c.r[0], 0x400);
                if c.crbit(2) { pc = 0x800abac8; continue; }
                pc = 0x800ab934;
            }
            0x800ab934 => {
                if !c.crbit(0) { pc = 0x800ab968; continue; }
                pc = 0x800ab938;
            }
            0x800ab938 => {
                c.cmps(0, c.r[0], 0x200);
                if c.crbit(2) { pc = 0x800aba34; continue; }
                pc = 0x800ab940;
            }
            0x800ab940 => {
                if !c.crbit(0) { pc = 0x800ab95c; continue; }
                pc = 0x800ab944;
            }
            0x800ab944 => {
                c.cmps(0, c.r[0], 0x100);
                if c.crbit(2) { pc = 0x800ab9e0; continue; }
                pc = 0x800ab94c;
            }
            0x800ab94c => {
                if !c.crbit(0) { pc = 0x800abb08; continue; }
                pc = 0x800ab950;
            }
            0x800ab950 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800ab98c; continue; }
                pc = 0x800ab958;
            }
            0x800ab958 => {
                pc = 0x800abb08; continue;
                pc = 0x800ab95c;
            }
            0x800ab95c => {
                c.cmps(0, c.r[0], 0x300);
                if c.crbit(2) { pc = 0x800aba84; continue; }
                pc = 0x800ab964;
            }
            0x800ab964 => {
                pc = 0x800abb08; continue;
                pc = 0x800ab968;
            }
            0x800ab968 => {
                c.cmps(0, c.r[0], 0x600);
                if c.crbit(2) { pc = 0x800ab98c; continue; }
                pc = 0x800ab970;
            }
            0x800ab970 => {
                if !c.crbit(0) { pc = 0x800ab980; continue; }
                pc = 0x800ab974;
            }
            0x800ab974 => {
                c.cmps(0, c.r[0], 0x500);
                if c.crbit(2) { pc = 0x800ab98c; continue; }
                pc = 0x800ab97c;
            }
            0x800ab97c => {
                pc = 0x800abb08; continue;
                pc = 0x800ab980;
            }
            0x800ab980 => {
                c.cmps(0, c.r[0], 0x700);
                if c.crbit(2) { pc = 0x800ab98c; continue; }
                pc = 0x800ab988;
            }
            0x800ab988 => {
                pc = 0x800abb08; continue;
                pc = 0x800ab98c;
            }
            0x800ab98c => {
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(8) & 0xff;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(16) & 0xffff;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x5), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(24) & 0xf0;
                c.stb(c.r[4].wrapping_add(0x6), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(28) & 0xf0;
                c.stb(c.r[4].wrapping_add(0x7), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(0) & 0xf0;
                c.stb(c.r[4].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(4) & 0xf0;
                c.stb(c.r[4].wrapping_add(0x9), c.r[0]);
                pc = 0x800abb08; continue;
                pc = 0x800ab9e0;
            }
            0x800ab9e0 => {
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(8) & 0xf0;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(12) & 0xf0;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x5), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(16) & 0xff;
                c.stb(c.r[4].wrapping_add(0x6), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(24) & 0xff;
                c.stb(c.r[4].wrapping_add(0x7), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(0) & 0xf0;
                c.stb(c.r[4].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(4) & 0xf0;
                c.stb(c.r[4].wrapping_add(0x9), c.r[0]);
                pc = 0x800abb08; continue;
                pc = 0x800aba34;
            }
            0x800aba34 => {
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(8) & 0xf0;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(12) & 0xf0;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x5), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(16) & 0xf0;
                c.stb(c.r[4].wrapping_add(0x6), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(20) & 0xf0;
                c.stb(c.r[4].wrapping_add(0x7), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(24) & 0xff;
                c.stb(c.r[4].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.stb(c.r[4].wrapping_add(0x9), c.r[0]);
                pc = 0x800abb08; continue;
                pc = 0x800aba84;
            }
            0x800aba84 => {
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = 0x0;
                c.r[6] = c.r[6].rotate_left(8) & 0xff;
                c.r[6] = c.r[6] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x4), c.r[6]);
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[6] = c.r[6].rotate_left(16) & 0xffff;
                c.r[6] = c.r[6] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x5), c.r[6]);
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[6] = c.r[6].rotate_left(24) & 0xff;
                c.stb(c.r[4].wrapping_add(0x6), c.r[6]);
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x4));
                c.stb(c.r[4].wrapping_add(0x7), c.r[5]);
                c.stb(c.r[4].wrapping_add(0x8), c.r[0]);
                c.stb(c.r[4].wrapping_add(0x9), c.r[0]);
                pc = 0x800abb08; continue;
                pc = 0x800abac8;
            }
            0x800abac8 => {
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = 0x0;
                c.r[6] = c.r[6].rotate_left(8) & 0xff;
                c.r[6] = c.r[6] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x4), c.r[6]);
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[6] = c.r[6].rotate_left(16) & 0xffff;
                c.r[6] = c.r[6] as u8 as i8 as i32 as u32;
                c.stb(c.r[4].wrapping_add(0x5), c.r[6]);
                c.stb(c.r[4].wrapping_add(0x6), c.r[0]);
                c.stb(c.r[4].wrapping_add(0x7), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(24) & 0xff;
                c.stb(c.r[4].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.stb(c.r[4].wrapping_add(0x9), c.r[0]);
                pc = 0x800abb08;
            }
            0x800abb08 => {
                c.r[6] = c.lbz(c.r[4].wrapping_add(0x2));
                c.r[5] = 0x80400000;
                c.r[3] = c.r[3].wrapping_mul(0xc);
                c.r[0] = c.r[6].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.r[5].wrapping_add(0xffffc5e0);
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                c.r[5] = c.lbz(c.r[4].wrapping_add(0x3));
                c.r[0] = c.r[5].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x3), c.r[0]);
                c.r[5] = c.lbz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.r[5].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[5] = c.lbz(c.r[4].wrapping_add(0x5));
                c.r[0] = c.r[5].wrapping_add(0xffffff80);
                c.stb(c.r[4].wrapping_add(0x5), c.r[0]);
                c.r[7] = c.lbz(c.r[3].wrapping_add(0x2));
                c.r[6] = c.lbz(c.r[4].wrapping_add(0x2));
                c.r[0] = c.r[7] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(1) { pc = 0x800abb78; continue; }
                pc = 0x800abb58;
            }
            0x800abb58 => {
                c.r[5] = c.r[7] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[5].wrapping_add(0xffffff80);
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[6] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], c.r[5]);
                if !c.crbit(0) { pc = 0x800abb9c; continue; }
                pc = 0x800abb70;
            }
            0x800abb70 => {
                c.r[6] = c.r[5] | c.r[5];
                pc = 0x800abb9c; continue;
                pc = 0x800abb78;
            }
            0x800abb78 => {
                c.r[0] = c.r[7] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x800abb9c; continue; }
                pc = 0x800abb80;
            }
            0x800abb80 => {
                c.r[5] = c.r[7] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[5].wrapping_add(0x7f);
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[6] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[5], c.r[0]);
                if !c.crbit(0) { pc = 0x800abb9c; continue; }
                pc = 0x800abb98;
            }
            0x800abb98 => {
                c.r[6] = c.r[5] | c.r[5];
                pc = 0x800abb9c;
            }
            0x800abb9c => {
                c.r[6] = c.r[6].wrapping_sub(c.r[7]);
                c.stb(c.r[4].wrapping_add(0x2), c.r[6]);
                c.r[7] = c.lbz(c.r[3].wrapping_add(0x3));
                c.r[6] = c.lbz(c.r[4].wrapping_add(0x3));
                c.r[0] = c.r[7] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(1) { pc = 0x800abbd4; continue; }
                pc = 0x800abbb4;
            }
            0x800abbb4 => {
                c.r[5] = c.r[7] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[5].wrapping_add(0xffffff80);
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[6] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], c.r[5]);
                if !c.crbit(0) { pc = 0x800abbf8; continue; }
                pc = 0x800abbcc;
            }
            0x800abbcc => {
                c.r[6] = c.r[5] | c.r[5];
                pc = 0x800abbf8; continue;
                pc = 0x800abbd4;
            }
            0x800abbd4 => {
                c.r[0] = c.r[7] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x800abbf8; continue; }
                pc = 0x800abbdc;
            }
            0x800abbdc => {
                c.r[5] = c.r[7] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[5].wrapping_add(0x7f);
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[6] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[5], c.r[0]);
                if !c.crbit(0) { pc = 0x800abbf8; continue; }
                pc = 0x800abbf4;
            }
            0x800abbf4 => {
                c.r[6] = c.r[5] | c.r[5];
                pc = 0x800abbf8;
            }
            0x800abbf8 => {
                c.r[6] = c.r[6].wrapping_sub(c.r[7]);
                c.stb(c.r[4].wrapping_add(0x3), c.r[6]);
                c.r[7] = c.lbz(c.r[3].wrapping_add(0x4));
                c.r[6] = c.lbz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.r[7] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(1) { pc = 0x800abc30; continue; }
                pc = 0x800abc10;
            }
            0x800abc10 => {
                c.r[5] = c.r[7] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[5].wrapping_add(0xffffff80);
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[6] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], c.r[5]);
                if !c.crbit(0) { pc = 0x800abc54; continue; }
                pc = 0x800abc28;
            }
            0x800abc28 => {
                c.r[6] = c.r[5] | c.r[5];
                pc = 0x800abc54; continue;
                pc = 0x800abc30;
            }
            0x800abc30 => {
                c.r[0] = c.r[7] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x800abc54; continue; }
                pc = 0x800abc38;
            }
            0x800abc38 => {
                c.r[5] = c.r[7] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[5].wrapping_add(0x7f);
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[6] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[5], c.r[0]);
                if !c.crbit(0) { pc = 0x800abc54; continue; }
                pc = 0x800abc50;
            }
            0x800abc50 => {
                c.r[6] = c.r[5] | c.r[5];
                pc = 0x800abc54;
            }
            0x800abc54 => {
                c.r[6] = c.r[6].wrapping_sub(c.r[7]);
                c.stb(c.r[4].wrapping_add(0x4), c.r[6]);
                c.r[7] = c.lbz(c.r[3].wrapping_add(0x5));
                c.r[6] = c.lbz(c.r[4].wrapping_add(0x5));
                c.r[0] = c.r[7] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(1) { pc = 0x800abc8c; continue; }
                pc = 0x800abc6c;
            }
            0x800abc6c => {
                c.r[5] = c.r[7] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[5].wrapping_add(0xffffff80);
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[6] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], c.r[5]);
                if !c.crbit(0) { pc = 0x800abcb0; continue; }
                pc = 0x800abc84;
            }
            0x800abc84 => {
                c.r[6] = c.r[5] | c.r[5];
                pc = 0x800abcb0; continue;
                pc = 0x800abc8c;
            }
            0x800abc8c => {
                c.r[0] = c.r[7] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x800abcb0; continue; }
                pc = 0x800abc94;
            }
            0x800abc94 => {
                c.r[5] = c.r[7] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[5].wrapping_add(0x7f);
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[6] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[5], c.r[0]);
                if !c.crbit(0) { pc = 0x800abcb0; continue; }
                pc = 0x800abcac;
            }
            0x800abcac => {
                c.r[6] = c.r[5] | c.r[5];
                pc = 0x800abcb0;
            }
            0x800abcb0 => {
                c.r[6] = c.r[6].wrapping_sub(c.r[7]);
                c.stb(c.r[4].wrapping_add(0x5), c.r[6]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x6));
                c.r[5] = c.lbz(c.r[4].wrapping_add(0x6));
                c.cmpu(0, c.r[5], c.r[0]);
                if !c.crbit(0) { pc = 0x800abccc; continue; }
                pc = 0x800abcc8;
            }
            0x800abcc8 => {
                c.r[5] = c.r[0] | c.r[0];
                pc = 0x800abccc;
            }
            0x800abccc => {
                c.r[5] = c.r[5].wrapping_sub(c.r[0]);
                c.stb(c.r[4].wrapping_add(0x6), c.r[5]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x7));
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x7));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x800abce8; continue; }
                pc = 0x800abce4;
            }
            0x800abce4 => {
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x800abce8;
            }
            0x800abce8 => {
                c.r[3] = c.r[3].wrapping_sub(c.r[0]);
                c.stb(c.r[4].wrapping_add(0x7), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ab8fc: bad pc {:#x}", pc),
        }
    }
}

/// PADSetAnalogMode (0x800abcf4, 116 bytes)
#[inline(never)]
pub fn f_800abcf4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800abcf4) { return; }
        if c.prof { crate::prof::enter(c, 0x800abcf4); p = true; }
    }
    f_800abcf4_b(c, 0x800abcf4);
    if p { crate::prof::exit(c, 0x800abcf4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800abcf4_at(c: &mut Cpu, pc: u32) { f_800abcf4_b(c, pc) }
#[inline(always)]
fn f_800abcf4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800abcf4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800abcf4, pc); }
        match pc {
            0x800abcf4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800abd0c; f_8009df3c(c);
                pc = 0x800abd0c;
            }
            0x800abd0c => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[6] = c.r[31].rotate_left(8) & 0xffffff00;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[31] = c.r[3] | c.r[3];
                c.r[8] = c.r[5].wrapping_add(0x0);
                c.r[7] = !(c.r[8] | c.r[8]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[5] = c.r[5] & !c.r[5];
                c.stw(c.r[13].wrapping_add(0xffff81f4), c.r[6]);
                c.r[4] = c.r[4] & c.r[7];
                c.r[0] = c.r[0] & c.r[7];
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[5]);
                c.r[3] = c.r[8] | c.r[8];
                c.stw(c.r[13].wrapping_add(0xffffa090), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa094), c.r[0]);
                c.lr = 0x800abd4c; f_800d0464(c);
                pc = 0x800abd4c;
            }
            0x800abd4c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800abd54; f_8009df64(c);
                pc = 0x800abd54;
            }
            0x800abd54 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800abcf4: bad pc {:#x}", pc),
        }
    }
}

/// OnReset (0x800abd68, 404 bytes)
#[inline(never)]
pub fn f_800abd68(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800abd68) { return; }
        if c.prof { crate::prof::enter(c, 0x800abd68); p = true; }
    }
    f_800abd68_b(c, 0x800abd68);
    if p { crate::prof::exit(c, 0x800abd68); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800abd68_at(c: &mut Cpu, pc: u32) { f_800abd68_b(c, pc) }
#[inline(never)]
fn f_800abd68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800abd68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800abd68, pc); }
        match pc {
            0x800abd68 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa09c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800abd94; continue; }
                pc = 0x800abd8c;
            }
            0x800abd8c => {
                c.r[3] = 0x0;
                c.lr = 0x800abd94; f_800abf5c(c);
                pc = 0x800abd94;
            }
            0x800abd94 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x800abed8; continue; }
                pc = 0x800abd9c;
            }
            0x800abd9c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[30] = 0x0;
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800abdc0; continue; }
                pc = 0x800abdb0;
            }
            0x800abdb0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[0], 0x20);
                if !c.crbit(2) { pc = 0x800abdc0; continue; }
                pc = 0x800abdbc;
            }
            0x800abdbc => {
                c.r[3] = 0x1;
                pc = 0x800abdc0;
            }
            0x800abdc0 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800abdd8; continue; }
                pc = 0x800abdc8;
            }
            0x800abdc8 => {
                c.lr = 0x800abdcc; f_800cf708(c);
                pc = 0x800abdcc;
            }
            0x800abdcc => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800abdd8; continue; }
                pc = 0x800abdd4;
            }
            0x800abdd4 => {
                c.r[30] = 0x1;
                pc = 0x800abdd8;
            }
            0x800abdd8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa0a0));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800abed0; continue; }
                pc = 0x800abde4;
            }
            0x800abde4 => {
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x800abed0; continue; }
                pc = 0x800abdec;
            }
            0x800abdec => {
                c.r[30] = 0xf0000000;
                c.lr = 0x800abdf4; f_8009df3c(c);
                pc = 0x800abdf4;
            }
            0x800abdf4 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa098));
                c.r[7] = 0x0;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa090));
                c.r[4] = 0x80000000;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa094));
                c.r[30] = c.r[30] | c.r[6];
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[5] = c.r[5] | c.r[0];
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x30e3));
                c.r[30] = c.r[30] & !c.r[5];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa084));
                c.r[4] = c.r[6] | c.r[30];
                c.stw(c.r[13].wrapping_add(0xffffa098), c.r[7]);
                c.r[0] = c.r[0].rotate_left(0) & 0x40;
                c.rc(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[4]);
                c.r[4] = c.r[5] & !c.r[30];
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.stw(c.r[13].wrapping_add(0xffffa084), c.r[4]);
                c.r[3] = c.r[6] & c.r[5];
                if !c.crbit(2) { pc = 0x800abe54; continue; }
                pc = 0x800abe48;
            }
            0x800abe48 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa08c));
                c.r[0] = c.r[0] | c.r[30];
                c.stw(c.r[13].wrapping_add(0xffffa08c), c.r[0]);
                pc = 0x800abe54;
            }
            0x800abe54 => {
                c.lr = 0x800abe58; f_800d0464(c);
                pc = 0x800abe58;
            }
            0x800abe58 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[0], 0x20);
                if !c.crbit(2) { pc = 0x800abeb8; continue; }
                pc = 0x800abe64;
            }
            0x800abe64 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa088));
                c.r[0] = c.r[5].leading_zeros();
                c.stw(c.r[13].wrapping_add(0xffff81ec), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.cmps(0, c.r[4], 0x20);
                if c.crbit(2) { pc = 0x800abeb8; continue; }
                pc = 0x800abe7c;
            }
            0x800abe7c => {
                c.r[0] = 0x80000000;
                c.r[0] = srw(c.r[0], c.r[4]);
                c.r[0] = c.r[5] & !c.r[0];
                c.r[4] = c.r[4].wrapping_mul(0xc);
                c.stw(c.r[13].wrapping_add(0xffffa088), c.r[0]);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffc5e0);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.lr = 0x800abea8; f_8000540c(c);
                pc = 0x800abea8;
            }
            0x800abea8 => {
                c.r[4] = 0x800b0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff81ec));
                c.r[4] = c.r[4].wrapping_add(0xffffa8d4);
                c.lr = 0x800abeb8; f_800d0cbc(c);
                pc = 0x800abeb8;
            }
            0x800abeb8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800abec0; f_8009df64(c);
                pc = 0x800abec0;
            }
            0x800abec0 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa0a0), c.r[0]);
                c.r[3] = 0x0;
                pc = 0x800abee4; continue;
                pc = 0x800abed0;
            }
            0x800abed0 => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800abee4; continue;
                pc = 0x800abed8;
            }
            0x800abed8 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa0a0), c.r[0]);
                c.r[3] = 0x1;
                pc = 0x800abee4;
            }
            0x800abee4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800abd68: bad pc {:#x}", pc),
        }
    }
}

/// SamplingHandler (0x800abefc, 96 bytes)
#[inline(never)]
pub fn f_800abefc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800abefc) { return; }
        if c.prof { crate::prof::enter(c, 0x800abefc); p = true; }
    }
    f_800abefc_b(c, 0x800abefc);
    if p { crate::prof::exit(c, 0x800abefc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800abefc_at(c: &mut Cpu, pc: u32) { f_800abefc_b(c, pc) }
#[inline(always)]
fn f_800abefc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800abefc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800abefc, pc); }
        match pc {
            0x800abefc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffd20); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2dc), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa09c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800abf48; continue; }
                pc = 0x800abf1c;
            }
            0x800abf1c => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800abf24; f_8009bd60(c);
                pc = 0x800abf24;
            }
            0x800abf24 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800abf2c; f_8009bb68(c);
                pc = 0x800abf2c;
            }
            0x800abf2c => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa09c));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800abf38; (c.dispatch)(c, t); }
                pc = 0x800abf38;
            }
            0x800abf38 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800abf40; f_8009bd60(c);
                pc = 0x800abf40;
            }
            0x800abf40 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800abf48; f_8009bb68(c);
                pc = 0x800abf48;
            }
            0x800abf48 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2e4));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2dc));
                c.r[1] = c.r[1].wrapping_add(0x2e0);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800abefc: bad pc {:#x}", pc),
        }
    }
}

/// PADSetSamplingCallback (0x800abf5c, 84 bytes)
#[inline(never)]
pub fn f_800abf5c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800abf5c) { return; }
        if c.prof { crate::prof::enter(c, 0x800abf5c); p = true; }
    }
    f_800abf5c_b(c, 0x800abf5c);
    if p { crate::prof::exit(c, 0x800abf5c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800abf5c_at(c: &mut Cpu, pc: u32) { f_800abf5c_b(c, pc) }
#[inline(always)]
fn f_800abf5c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800abf5c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800abf5c, pc); }
        match pc {
            0x800abf5c => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa09c));
                c.stw(c.r[13].wrapping_add(0xffffa09c), c.r[3]);
                if c.crbit(2) { pc = 0x800abf8c; continue; }
                pc = 0x800abf7c;
            }
            0x800abf7c => {
                c.r[3] = 0x800b0000;
                c.r[3] = c.r[3].wrapping_add(0xffffbefc);
                c.lr = 0x800abf88; f_800cfe3c(c);
                pc = 0x800abf88;
            }
            0x800abf88 => {
                pc = 0x800abf98; continue;
                pc = 0x800abf8c;
            }
            0x800abf8c => {
                c.r[3] = 0x800b0000;
                c.r[3] = c.r[3].wrapping_add(0xffffbefc);
                c.lr = 0x800abf98; f_800cff08(c);
                pc = 0x800abf98;
            }
            0x800abf98 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800abf5c: bad pc {:#x}", pc),
        }
    }
}

/// __PADDisableRecalibration (0x800abfb0, 124 bytes)
#[inline(never)]
pub fn f_800abfb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800abfb0) { return; }
        if c.prof { crate::prof::enter(c, 0x800abfb0); p = true; }
    }
    f_800abfb0_b(c, 0x800abfb0);
    if p { crate::prof::exit(c, 0x800abfb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800abfb0_at(c: &mut Cpu, pc: u32) { f_800abfb0_b(c, pc) }
#[inline(always)]
fn f_800abfb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800abfb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800abfb0, pc); }
        match pc {
            0x800abfb0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800abfcc; f_8009df3c(c);
                pc = 0x800abfcc;
            }
            0x800abfcc => {
                c.r[4] = 0x80000000;
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x30e3));
                c.r[0] = c.r[0].rotate_left(0) & 0x40;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800abfe4; continue; }
                pc = 0x800abfdc;
            }
            0x800abfdc => {
                c.r[31] = 0x1;
                pc = 0x800abfe8; continue;
                pc = 0x800abfe4;
            }
            0x800abfe4 => {
                c.r[31] = 0x0;
                pc = 0x800abfe8;
            }
            0x800abfe8 => {
                c.r[4] = 0x80000000;
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x30e3));
                c.r[0] = c.r[0] & 0xbf;
                c.rc(c.r[0]);
                c.cmps(0, c.r[30], 0x0);
                c.stb(c.r[4].wrapping_add(0x30e3), c.r[0]);
                if c.crbit(2) { pc = 0x800ac00c; continue; }
                pc = 0x800ac000;
            }
            0x800ac000 => {
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x30e3));
                c.r[0] = c.r[0] | 0x40;
                c.stb(c.r[4].wrapping_add(0x30e3), c.r[0]);
                pc = 0x800ac00c;
            }
            0x800ac00c => {
                c.lr = 0x800ac010; f_8009df64(c);
                pc = 0x800ac010;
            }
            0x800ac010 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800abfb0: bad pc {:#x}", pc),
        }
    }
}

/// AIRegisterDMACallback (0x800ac02c, 68 bytes)
#[inline(never)]
pub fn f_800ac02c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac02c) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac02c); p = true; }
    }
    f_800ac02c_b(c, 0x800ac02c);
    if p { crate::prof::exit(c, 0x800ac02c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac02c_at(c: &mut Cpu, pc: u32) { f_800ac02c_b(c, pc) }
#[inline(always)]
fn f_800ac02c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac02c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac02c, pc); }
        match pc {
            0x800ac02c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa0ac));
                c.lr = 0x800ac04c; f_8009df3c(c);
                pc = 0x800ac04c;
            }
            0x800ac04c => {
                c.stw(c.r[13].wrapping_add(0xffffa0ac), c.r[30]);
                c.lr = 0x800ac054; f_8009df64(c);
                pc = 0x800ac054;
            }
            0x800ac054 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac02c: bad pc {:#x}", pc),
        }
    }
}

/// AIInitDMA (0x800ac070, 136 bytes)
#[inline(never)]
pub fn f_800ac070(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac070) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac070); p = true; }
    }
    f_800ac070_b(c, 0x800ac070);
    if p { crate::prof::exit(c, 0x800ac070); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac070_at(c: &mut Cpu, pc: u32) { f_800ac070_b(c, pc) }
#[inline(always)]
fn f_800ac070_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac070;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac070, pc); }
        match pc {
            0x800ac070 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800ac090; f_8009df3c(c);
                pc = 0x800ac090;
            }
            0x800ac090 => {
                c.r[4] = 0xcc000000;
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x5030));
                c.r[5] = c.r[4].wrapping_add(0x5000);
                c.r[6] = c.r[4].wrapping_add(0x5000);
                c.r[7] = c.r[4].wrapping_add(0x5000);
                c.r[4] = c.r[0].rotate_left(0) & 0xfffffc00;
                c.r[0] = c.r[30].rotate_left(16) & 0xffff;
                c.r[0] = c.r[4] | c.r[0];
                c.sth(c.r[5].wrapping_add(0x30), c.r[0]);
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.r[4] = c.lhz(c.r[6].wrapping_add(0x32));
                c.r[4] = c.r[4].rotate_left(0) & 0xffff001f;
                c.r[0] = c.r[4] | c.r[0];
                c.sth(c.r[6].wrapping_add(0x32), c.r[0]);
                c.r[0] = c.r[31].rotate_left(27) & 0xffff;
                c.r[4] = c.lhz(c.r[7].wrapping_add(0x36));
                c.r[4] = c.r[4].rotate_left(0) & 0xffff8000;
                c.r[0] = c.r[4] | c.r[0];
                c.sth(c.r[7].wrapping_add(0x36), c.r[0]);
                c.lr = 0x800ac0e0; f_8009df64(c);
                pc = 0x800ac0e0;
            }
            0x800ac0e0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac070: bad pc {:#x}", pc),
        }
    }
}

/// AIStartDMA (0x800ac0f8, 24 bytes)
#[inline(never)]
pub fn f_800ac0f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac0f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac0f8); p = true; }
    }
    f_800ac0f8_b(c, 0x800ac0f8);
    if p { crate::prof::exit(c, 0x800ac0f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac0f8_at(c: &mut Cpu, pc: u32) { f_800ac0f8_b(c, pc) }
#[inline(always)]
fn f_800ac0f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac0f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac0f8, pc); }
        match pc {
            0x800ac0f8 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x5000);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x36));
                c.r[0] = c.r[0] | 0x8000;
                c.sth(c.r[3].wrapping_add(0x36), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac0f8: bad pc {:#x}", pc),
        }
    }
}

/// AIStopDMA (0x800ac110, 24 bytes)
#[inline(never)]
pub fn f_800ac110(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac110) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac110); p = true; }
    }
    f_800ac110_b(c, 0x800ac110);
    if p { crate::prof::exit(c, 0x800ac110); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac110_at(c: &mut Cpu, pc: u32) { f_800ac110_b(c, pc) }
#[inline(always)]
fn f_800ac110_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac110;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac110, pc); }
        match pc {
            0x800ac110 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x5000);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x36));
                c.r[0] = c.r[0].rotate_left(0) & 0xffff7fff;
                c.sth(c.r[3].wrapping_add(0x36), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac110: bad pc {:#x}", pc),
        }
    }
}

/// AISetStreamPlayState (0x800ac128, 216 bytes)
#[inline(never)]
pub fn f_800ac128(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac128) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac128); p = true; }
    }
    f_800ac128_b(c, 0x800ac128);
    if p { crate::prof::exit(c, 0x800ac128); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac128_at(c: &mut Cpu, pc: u32) { f_800ac128_b(c, pc) }
#[inline(always)]
fn f_800ac128_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac128;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac128, pc); }
        match pc {
            0x800ac128 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x800ac148; f_800ac200(c);
                pc = 0x800ac148;
            }
            0x800ac148 => {
                c.cmpu(0, c.r[29], c.r[3]);
                if c.crbit(2) { pc = 0x800ac1e4; continue; }
                pc = 0x800ac150;
            }
            0x800ac150 => {
                c.lr = 0x800ac154; f_800ac3d8(c);
                pc = 0x800ac154;
            }
            0x800ac154 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800ac1d0; continue; }
                pc = 0x800ac15c;
            }
            0x800ac15c => {
                c.cmpu(0, c.r[29], 0x1);
                if !c.crbit(2) { pc = 0x800ac1d0; continue; }
                pc = 0x800ac164;
            }
            0x800ac164 => {
                c.lr = 0x800ac168; f_800ac430(c);
                pc = 0x800ac168;
            }
            0x800ac168 => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800ac170; f_800ac404(c);
                pc = 0x800ac170;
            }
            0x800ac170 => {
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x0;
                c.lr = 0x800ac17c; f_800ac414(c);
                pc = 0x800ac17c;
            }
            0x800ac17c => {
                c.r[3] = 0x0;
                c.lr = 0x800ac184; f_800ac3e8(c);
                pc = 0x800ac184;
            }
            0x800ac184 => {
                c.lr = 0x800ac188; f_8009df3c(c);
                pc = 0x800ac188;
            }
            0x800ac188 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800ac190; f_800ac72c(c);
                pc = 0x800ac190;
            }
            0x800ac190 => {
                c.r[4] = 0xcc000000;
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x6c00));
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffdf;
                c.r[0] = c.r[0] | 0x20;
                c.stw(c.r[4].wrapping_add(0x6c00), c.r[0]);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x6c00));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.r[0] = c.r[0] | 0x1;
                c.stw(c.r[4].wrapping_add(0x6c00), c.r[0]);
                c.lr = 0x800ac1bc; f_8009df64(c);
                pc = 0x800ac1bc;
            }
            0x800ac1bc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800ac1c4; f_800ac3e8(c);
                pc = 0x800ac1c4;
            }
            0x800ac1c4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800ac1cc; f_800ac414(c);
                pc = 0x800ac1cc;
            }
            0x800ac1cc => {
                pc = 0x800ac1e4; continue;
                pc = 0x800ac1d0;
            }
            0x800ac1d0 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6c00));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.r[0] = c.r[0] | c.r[29];
                c.stw(c.r[3].wrapping_add(0x6c00), c.r[0]);
                pc = 0x800ac1e4;
            }
            0x800ac1e4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac128: bad pc {:#x}", pc),
        }
    }
}

/// AIGetStreamPlayState (0x800ac200, 16 bytes)
#[inline(never)]
pub fn f_800ac200(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac200) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac200); p = true; }
    }
    f_800ac200_b(c, 0x800ac200);
    if p { crate::prof::exit(c, 0x800ac200); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac200_at(c: &mut Cpu, pc: u32) { f_800ac200_b(c, pc) }
#[inline(always)]
fn f_800ac200_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac200;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac200, pc); }
        match pc {
            0x800ac200 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6c00));
                c.r[3] = c.r[0].rotate_left(0) & 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_800ac200: bad pc {:#x}", pc),
        }
    }
}

/// AISetDSPSampleRate (0x800ac210, 224 bytes)
#[inline(never)]
pub fn f_800ac210(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac210) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac210); p = true; }
    }
    f_800ac210_b(c, 0x800ac210);
    if p { crate::prof::exit(c, 0x800ac210); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac210_at(c: &mut Cpu, pc: u32) { f_800ac210_b(c, pc) }
#[inline(always)]
fn f_800ac210_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac210;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac210, pc); }
        match pc {
            0x800ac210 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x800ac228; f_800ac2f0(c);
                pc = 0x800ac228;
            }
            0x800ac228 => {
                c.cmpu(0, c.r[26], c.r[3]);
                if c.crbit(2) { pc = 0x800ac2dc; continue; }
                pc = 0x800ac230;
            }
            0x800ac230 => {
                c.r[31] = 0xcc000000;
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.cmpu(0, c.r[26], 0x0);
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffbf;
                c.stw(c.r[31].wrapping_add(0x6c00), c.r[0]);
                if !c.crbit(2) { pc = 0x800ac2dc; continue; }
                pc = 0x800ac248;
            }
            0x800ac248 => {
                c.lr = 0x800ac24c; f_800ac404(c);
                pc = 0x800ac24c;
            }
            0x800ac24c => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800ac254; f_800ac430(c);
                pc = 0x800ac254;
            }
            0x800ac254 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[27] = c.r[0].rotate_left(0) & 0x1;
                c.lr = 0x800ac264; f_800ac3d8(c);
                pc = 0x800ac264;
            }
            0x800ac264 => {
                c.r[28] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x0;
                c.lr = 0x800ac270; f_800ac3e8(c);
                pc = 0x800ac270;
            }
            0x800ac270 => {
                c.r[3] = 0x0;
                c.lr = 0x800ac278; f_800ac414(c);
                pc = 0x800ac278;
            }
            0x800ac278 => {
                c.lr = 0x800ac27c; f_8009df3c(c);
                pc = 0x800ac27c;
            }
            0x800ac27c => {
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x800ac284; f_800ac72c(c);
                pc = 0x800ac284;
            }
            0x800ac284 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[0] = c.r[28].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[26].wrapping_add(0x0);
                c.r[4] = c.r[4].rotate_left(0) & 0xffffffdf;
                c.r[4] = c.r[4] | 0x20;
                c.stw(c.r[31].wrapping_add(0x6c00), c.r[4]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffffd;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[31].wrapping_add(0x6c00), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.r[0] = c.r[0] | c.r[27];
                c.stw(c.r[31].wrapping_add(0x6c00), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[0] = c.r[0] | 0x40;
                c.stw(c.r[31].wrapping_add(0x6c00), c.r[0]);
                c.lr = 0x800ac2cc; f_8009df64(c);
                pc = 0x800ac2cc;
            }
            0x800ac2cc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800ac2d4; f_800ac3e8(c);
                pc = 0x800ac2d4;
            }
            0x800ac2d4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800ac2dc; f_800ac414(c);
                pc = 0x800ac2dc;
            }
            0x800ac2dc => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac210: bad pc {:#x}", pc),
        }
    }
}

/// AIGetDSPSampleRate (0x800ac2f0, 20 bytes)
#[inline(never)]
pub fn f_800ac2f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac2f0) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac2f0); p = true; }
    }
    f_800ac2f0_b(c, 0x800ac2f0);
    if p { crate::prof::exit(c, 0x800ac2f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac2f0_at(c: &mut Cpu, pc: u32) { f_800ac2f0_b(c, pc) }
#[inline(always)]
fn f_800ac2f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac2f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac2f0, pc); }
        match pc {
            0x800ac2f0 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6c00));
                c.r[0] = c.r[0].rotate_left(26) & 0x1;
                c.r[3] = c.r[0] ^ 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_800ac2f0: bad pc {:#x}", pc),
        }
    }
}

/// __AI_set_stream_sample_rate (0x800ac304, 212 bytes)
#[inline(never)]
pub fn f_800ac304(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac304) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac304); p = true; }
    }
    f_800ac304_b(c, 0x800ac304);
    if p { crate::prof::exit(c, 0x800ac304); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac304_at(c: &mut Cpu, pc: u32) { f_800ac304_b(c, pc) }
#[inline(always)]
fn f_800ac304_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac304;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac304, pc); }
        match pc {
            0x800ac304 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.lr = 0x800ac31c; f_800ac3d8(c);
                pc = 0x800ac31c;
            }
            0x800ac31c => {
                c.cmpu(0, c.r[25], c.r[3]);
                if c.crbit(2) { pc = 0x800ac3c4; continue; }
                pc = 0x800ac324;
            }
            0x800ac324 => {
                c.r[31] = 0xcc000000;
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.r[29] = c.r[0] | c.r[0];
                c.lr = 0x800ac338; f_800ac404(c);
                pc = 0x800ac338;
            }
            0x800ac338 => {
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x800ac340; f_800ac430(c);
                pc = 0x800ac340;
            }
            0x800ac340 => {
                c.r[27] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x0;
                c.lr = 0x800ac34c; f_800ac414(c);
                pc = 0x800ac34c;
            }
            0x800ac34c => {
                c.r[3] = 0x0;
                c.lr = 0x800ac354; f_800ac3e8(c);
                pc = 0x800ac354;
            }
            0x800ac354 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[26] = c.r[3].rotate_left(0) & 0x40;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffbf;
                c.stw(c.r[31].wrapping_add(0x6c00), c.r[0]);
                c.lr = 0x800ac36c; f_8009df3c(c);
                pc = 0x800ac36c;
            }
            0x800ac36c => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800ac374; f_800ac72c(c);
                pc = 0x800ac374;
            }
            0x800ac374 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[0] = c.r[25].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.r[4] = c.r[4] | c.r[26];
                c.stw(c.r[31].wrapping_add(0x6c00), c.r[4]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[4] = c.r[4].rotate_left(0) & 0xffffffdf;
                c.r[4] = c.r[4] | 0x20;
                c.stw(c.r[31].wrapping_add(0x6c00), c.r[4]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffffd;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[31].wrapping_add(0x6c00), c.r[0]);
                c.lr = 0x800ac3ac; f_8009df64(c);
                pc = 0x800ac3ac;
            }
            0x800ac3ac => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800ac3b4; f_800ac128(c);
                pc = 0x800ac3b4;
            }
            0x800ac3b4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x800ac3bc; f_800ac3e8(c);
                pc = 0x800ac3bc;
            }
            0x800ac3bc => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x800ac3c4; f_800ac414(c);
                pc = 0x800ac3c4;
            }
            0x800ac3c4 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac304: bad pc {:#x}", pc),
        }
    }
}

/// AIGetStreamSampleRate (0x800ac3d8, 16 bytes)
#[inline(never)]
pub fn f_800ac3d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac3d8) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac3d8); p = true; }
    }
    f_800ac3d8_b(c, 0x800ac3d8);
    if p { crate::prof::exit(c, 0x800ac3d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac3d8_at(c: &mut Cpu, pc: u32) { f_800ac3d8_b(c, pc) }
#[inline(always)]
fn f_800ac3d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac3d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac3d8, pc); }
        match pc {
            0x800ac3d8 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6c00));
                c.r[3] = c.r[0].rotate_left(31) & 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_800ac3d8: bad pc {:#x}", pc),
        }
    }
}

/// AISetStreamVolLeft (0x800ac3e8, 28 bytes)
#[inline(never)]
pub fn f_800ac3e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac3e8) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac3e8); p = true; }
    }
    f_800ac3e8_b(c, 0x800ac3e8);
    if p { crate::prof::exit(c, 0x800ac3e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac3e8_at(c: &mut Cpu, pc: u32) { f_800ac3e8_b(c, pc) }
#[inline(always)]
fn f_800ac3e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac3e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac3e8, pc); }
        match pc {
            0x800ac3e8 => {
                c.r[4] = 0xcc000000;
                c.r[4] = c.r[4].wrapping_add(0x6c00);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff00;
                c.r[0] = (c.r[3].rotate_left(0) & 0xff) | (c.r[0] & 0xffffff00);
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac3e8: bad pc {:#x}", pc),
        }
    }
}

/// AIGetStreamVolLeft (0x800ac404, 16 bytes)
#[inline(never)]
pub fn f_800ac404(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac404) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac404); p = true; }
    }
    f_800ac404_b(c, 0x800ac404);
    if p { crate::prof::exit(c, 0x800ac404); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac404_at(c: &mut Cpu, pc: u32) { f_800ac404_b(c, pc) }
#[inline(always)]
fn f_800ac404_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac404;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac404, pc); }
        match pc {
            0x800ac404 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6c04));
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                if true { return; }
                return;
            }
            _ => panic!("f_800ac404: bad pc {:#x}", pc),
        }
    }
}

/// AISetStreamVolRight (0x800ac414, 28 bytes)
#[inline(never)]
pub fn f_800ac414(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac414) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac414); p = true; }
    }
    f_800ac414_b(c, 0x800ac414);
    if p { crate::prof::exit(c, 0x800ac414); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac414_at(c: &mut Cpu, pc: u32) { f_800ac414_b(c, pc) }
#[inline(always)]
fn f_800ac414_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac414;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac414, pc); }
        match pc {
            0x800ac414 => {
                c.r[4] = 0xcc000000;
                c.r[4] = c.r[4].wrapping_add(0x6c00);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(0) & 0xffff00ff;
                c.r[0] = (c.r[3].rotate_left(8) & 0xff00) | (c.r[0] & 0xffff00ff);
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac414: bad pc {:#x}", pc),
        }
    }
}

/// AIGetStreamVolRight (0x800ac430, 16 bytes)
#[inline(never)]
pub fn f_800ac430(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac430) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac430); p = true; }
    }
    f_800ac430_b(c, 0x800ac430);
    if p { crate::prof::exit(c, 0x800ac430); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac430_at(c: &mut Cpu, pc: u32) { f_800ac430_b(c, pc) }
#[inline(always)]
fn f_800ac430_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac430;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac430, pc); }
        match pc {
            0x800ac430 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6c04));
                c.r[3] = c.r[0].rotate_left(24) & 0xff;
                if true { return; }
                return;
            }
            _ => panic!("f_800ac430: bad pc {:#x}", pc),
        }
    }
}

/// AIInit (0x800ac440, 364 bytes)
#[inline(never)]
pub fn f_800ac440(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac440) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac440); p = true; }
    }
    f_800ac440_b(c, 0x800ac440);
    if p { crate::prof::exit(c, 0x800ac440); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac440_at(c: &mut Cpu, pc: u32) { f_800ac440_b(c, pc) }
#[inline(always)]
fn f_800ac440_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac440;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac440, pc); }
        match pc {
            0x800ac440 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa0b8));
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800ac594; continue; }
                pc = 0x800ac464;
            }
            0x800ac464 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8208));
                c.lr = 0x800ac46c; f_8009a250(c);
                pc = 0x800ac46c;
            }
            0x800ac46c => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[3] = 0x431c0000;
                c.r[4] = 0x10000;
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[3] = c.r[3].wrapping_add(0xffffde83);
                c.r[0] = mulhwu(c.r[3], c.r[0]);
                c.r[9] = c.r[0].rotate_left(17) & 0x1ffff;
                c.r[5] = c.r[4].wrapping_add(0xffffa428);
                c.r[3] = c.r[4].wrapping_add(0xffffa410);
                c.r[0] = c.r[4].wrapping_add(0xfffff618);
                c.r[4] = 0x10620000;
                c.r[7] = c.r[9].wrapping_mul(c.r[5]);
                c.r[10] = c.r[4].wrapping_add(0x4dd3);
                c.r[5] = c.r[9].wrapping_mul(c.r[3]);
                c.r[4] = c.r[9].wrapping_mul(c.r[0]);
                c.r[8] = c.r[9].wrapping_mul(0x7b24);
                c.r[3] = c.r[9].wrapping_mul(0xbb8);
                c.r[8] = mulhwu(c.r[10], c.r[8]);
                c.r[7] = mulhwu(c.r[10], c.r[7]);
                c.r[5] = mulhwu(c.r[10], c.r[5]);
                c.r[4] = mulhwu(c.r[10], c.r[4]);
                c.r[3] = mulhwu(c.r[10], c.r[3]);
                c.r[8] = c.r[8].rotate_left(23) & 0x7fffff;
                c.r[7] = c.r[7].rotate_left(23) & 0x7fffff;
                c.stw(c.r[13].wrapping_add(0xffffa0c4), c.r[8]);
                c.r[5] = c.r[5].rotate_left(23) & 0x7fffff;
                c.r[4] = c.r[4].rotate_left(23) & 0x7fffff;
                c.stw(c.r[13].wrapping_add(0xffffa0cc), c.r[7]);
                c.r[31] = 0x0;
                c.r[3] = c.r[3].rotate_left(23) & 0x7fffff;
                c.stw(c.r[13].wrapping_add(0xffffa0d4), c.r[5]);
                c.r[6] = 0xcc000000;
                c.stw(c.r[13].wrapping_add(0xffffa0e4), c.r[3]);
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x6c00));
                c.stw(c.r[13].wrapping_add(0xffffa0dc), c.r[4]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffdf;
                c.r[0] = c.r[0] | 0x20;
                c.stw(c.r[13].wrapping_add(0xffffa0c0), c.r[31]);
                c.stw(c.r[13].wrapping_add(0xffffa0c8), c.r[31]);
                c.stw(c.r[13].wrapping_add(0xffffa0d0), c.r[31]);
                c.stw(c.r[13].wrapping_add(0xffffa0d8), c.r[31]);
                c.stw(c.r[13].wrapping_add(0xffffa0e0), c.r[31]);
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x6c04));
                c.stw(c.r[6].wrapping_add(0x6c00), c.r[0]);
                c.r[0] = c.r[5].rotate_left(0) & 0xffff00ff;
                c.stw(c.r[6].wrapping_add(0x6c04), c.r[0]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x6c04));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff00;
                c.stw(c.r[6].wrapping_add(0x6c04), c.r[0]);
                c.stw(c.r[6].wrapping_add(0x6c0c), c.r[31]);
                c.lr = 0x800ac548; f_800ac304(c);
                pc = 0x800ac548;
            }
            0x800ac548 => {
                c.r[3] = 0x0;
                c.lr = 0x800ac550; f_800ac210(c);
                pc = 0x800ac550;
            }
            0x800ac550 => {
                c.r[3] = 0x800b0000;
                c.stw(c.r[13].wrapping_add(0xffffa0a8), c.r[31]);
                c.r[4] = c.r[3].wrapping_add(0xffffc628);
                c.stw(c.r[13].wrapping_add(0xffffa0ac), c.r[31]);
                c.r[3] = 0x5;
                c.stw(c.r[13].wrapping_add(0xffffa0b0), c.r[30]);
                c.lr = 0x800ac56c; f_8009df88(c);
                pc = 0x800ac56c;
            }
            0x800ac56c => {
                c.r[3] = 0x4000000;
                c.lr = 0x800ac574; f_8009e38c(c);
                pc = 0x800ac574;
            }
            0x800ac574 => {
                c.r[3] = 0x800b0000;
                c.r[4] = c.r[3].wrapping_add(0xffffc5ac);
                c.r[3] = 0x8;
                c.lr = 0x800ac584; f_8009df88(c);
                pc = 0x800ac584;
            }
            0x800ac584 => {
                c.r[3] = 0x800000;
                c.lr = 0x800ac58c; f_8009e38c(c);
                pc = 0x800ac58c;
            }
            0x800ac58c => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa0b8), c.r[0]);
                pc = 0x800ac594;
            }
            0x800ac594 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac440: bad pc {:#x}", pc),
        }
    }
}

/// __AISHandler (0x800ac5ac, 124 bytes)
#[inline(never)]
pub fn f_800ac5ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac5ac) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac5ac); p = true; }
    }
    f_800ac5ac_b(c, 0x800ac5ac);
    if p { crate::prof::exit(c, 0x800ac5ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac5ac_at(c: &mut Cpu, pc: u32) { f_800ac5ac_b(c, pc) }
#[inline(always)]
fn f_800ac5ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac5ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac5ac, pc); }
        match pc {
            0x800ac5ac => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffd20); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2dc), c.r[31]);
                c.r[31] = 0xcc000000;
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x6c00));
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.stw(c.r[1].wrapping_add(0x2d8), c.r[30]);
                c.r[0] = c.r[0] | 0x8;
                c.stw(c.r[31].wrapping_add(0x6c00), c.r[0]);
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.lr = 0x800ac5dc; f_8009bd60(c);
                pc = 0x800ac5dc;
            }
            0x800ac5dc => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800ac5e4; f_8009bb68(c);
                pc = 0x800ac5e4;
            }
            0x800ac5e4 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa0a8));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800ac600; continue; }
                pc = 0x800ac5f0;
            }
            0x800ac5f0 => {
                c.r[3] = c.r[31].wrapping_add(0x6c00);
                c.mtspr(8, c.r[12]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { let t = c.lr; c.lr = 0x800ac600; (c.dispatch)(c, t); }
                pc = 0x800ac600;
            }
            0x800ac600 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800ac608; f_8009bd60(c);
                pc = 0x800ac608;
            }
            0x800ac608 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800ac610; f_8009bb68(c);
                pc = 0x800ac610;
            }
            0x800ac610 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2e4));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2dc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x2d8));
                c.r[1] = c.r[1].wrapping_add(0x2e0);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac5ac: bad pc {:#x}", pc),
        }
    }
}

/// __AIDHandler (0x800ac628, 172 bytes)
#[inline(never)]
pub fn f_800ac628(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac628) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac628); p = true; }
    }
    f_800ac628_b(c, 0x800ac628);
    if p { crate::prof::exit(c, 0x800ac628); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac628_at(c: &mut Cpu, pc: u32) { f_800ac628_b(c, pc) }
#[inline(always)]
fn f_800ac628_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac628;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac628, pc); }
        match pc {
            0x800ac628 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0xcc000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x5000);
                c.r[0] = 0xffffff5f;
                { let ea = c.r[1].wrapping_add(0xfffffd20); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2dc), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.r[5] = c.lhz(c.r[3].wrapping_add(0xa));
                c.r[0] = c.r[5] & c.r[0];
                c.r[0] = c.r[0] | 0x8;
                c.sth(c.r[3].wrapping_add(0xa), c.r[0]);
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800ac660; f_8009bd60(c);
                pc = 0x800ac660;
            }
            0x800ac660 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800ac668; f_8009bb68(c);
                pc = 0x800ac668;
            }
            0x800ac668 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa0ac));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800ac6b0; continue; }
                pc = 0x800ac674;
            }
            0x800ac674 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa0bc));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800ac6b0; continue; }
                pc = 0x800ac680;
            }
            0x800ac680 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa0b0));
                c.r[4] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa0bc), c.r[4]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800ac69c; continue; }
                pc = 0x800ac694;
            }
            0x800ac694 => {
                c.lr = 0x800ac698; f_800ac6d4(c);
                pc = 0x800ac698;
            }
            0x800ac698 => {
                pc = 0x800ac6a8; continue;
                pc = 0x800ac69c;
            }
            0x800ac69c => {
                c.r[12] = c.r[3].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800ac6a8; (c.dispatch)(c, t); }
                pc = 0x800ac6a8;
            }
            0x800ac6a8 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa0bc), c.r[0]);
                pc = 0x800ac6b0;
            }
            0x800ac6b0 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800ac6b8; f_8009bd60(c);
                pc = 0x800ac6b8;
            }
            0x800ac6b8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800ac6c0; f_8009bb68(c);
                pc = 0x800ac6c0;
            }
            0x800ac6c0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2e4));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2dc));
                c.r[1] = c.r[1].wrapping_add(0x2e0);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac628: bad pc {:#x}", pc),
        }
    }
}

/// __AICallbackStackSwitch (0x800ac6d4, 88 bytes)
#[inline(never)]
pub fn f_800ac6d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ac6d4) { return; }
        if c.prof { crate::prof::enter(c, 0x800ac6d4); p = true; }
    }
    f_800ac6d4_b(c, 0x800ac6d4);
    if p { crate::prof::exit(c, 0x800ac6d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ac6d4_at(c: &mut Cpu, pc: u32) { f_800ac6d4_b(c, pc) }
#[inline(always)]
fn f_800ac6d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ac6d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ac6d4, pc); }
        match pc {
            0x800ac6d4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = 0x80480000;
                c.r[5] = c.r[5].wrapping_add(0xffffa8d4);
                c.stw(c.r[5], c.r[1]);
                c.r[5] = 0x80480000;
                c.r[5] = c.r[5].wrapping_add(0xffffa8d0);
                c.r[1] = c.lwz(c.r[5]);
                c.r[1] = c.r[1].wrapping_add(0xfffffff8);
                c.mtspr(8, c.r[31]);
                if true { let t = c.lr; c.lr = 0x800ac70c; (c.dispatch)(c, t); }
                pc = 0x800ac70c;
            }
            0x800ac70c => {
                c.r[5] = 0x80480000;
                c.r[5] = c.r[5].wrapping_add(0xffffa8d4);
                c.r[1] = c.lwz(c.r[5]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ac6d4: bad pc {:#x}", pc),
        }
    }
}

