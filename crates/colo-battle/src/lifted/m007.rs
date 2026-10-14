#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// fn_80098790 (0x80098790, 436 bytes)
#[inline(never)]
pub fn f_80098790(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80098790) { return; }
        if c.prof { crate::prof::enter(c, 0x80098790); f_80098790_b(c, 0x80098790); crate::prof::exit(c, 0x80098790); return; }
    }
    f_80098790_b(c, 0x80098790)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80098790_at(c: &mut Cpu, pc: u32) { f_80098790_b(c, pc) }
#[inline(always)]
fn f_80098790_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80098790;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80098790, pc); }
        match pc {
            0x80098790 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 21..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[3].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[30] = c.r[0].wrapping_add(c.r[4]);
                c.cmps(0, c.r[31], 0x2);
                if !c.crbit(2) { pc = 0x800987c4; continue; }
                pc = 0x800987bc;
            }
            0x800987bc => {
                c.r[3] = 0x1;
                pc = 0x80098930; continue;
                pc = 0x800987c4;
            }
            0x800987c4 => {
                c.r[29] = 0x1;
                c.lr = 0x800987cc; f_8009df3c(c);
                pc = 0x800987cc;
            }
            0x800987cc => {
                c.r[25] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.r[28] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800988f8; continue; }
                pc = 0x800987f0;
            }
            0x800987f0 => {
                c.r[0] = c.r[28].rotate_left(0) & 0x800;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80098844; continue; }
                pc = 0x800987f8;
            }
            0x800987f8 => {
                c.r[0] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.r[27] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.stw(c.r[1].wrapping_add(0xc), c.r[27]);
                c.r[27] = c.r[27] & 0x7f5;
                c.rc(c.r[27]);
                c.r[27] = c.r[27] | 0x800;
                c.r[0] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[27]);
                c.r[4] = 0x0;
                c.stw(c.r[30].wrapping_add(0x20), c.r[4]);
                c.r[0] = c.r[31].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                pc = 0x80098844;
            }
            0x80098844 => {
                c.r[0] = c.r[28].rotate_left(0) & 0x1000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800988d8; continue; }
                pc = 0x8009884c;
            }
            0x8009884c => {
                c.r[23] = 0x64;
                c.r[24] = 0x0;
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[3] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = 0x3e8;
                c.r[21] = divwu(c.r[3], c.r[0]);
                c.r[22] = 0x0;
                c.lr = 0x80098870; crate::stubs::f_800a2778(c);
                pc = 0x80098870;
            }
            0x80098870 => {
                c.r[5] = c.r[22].wrapping_add(0x0);
                c.r[6] = c.r[21].wrapping_add(0x0);
                c.lr = 0x8009887c; f_800c4928(c);
                pc = 0x8009887c;
            }
            0x8009887c => {
                c.r[5] = c.r[24].wrapping_add(0x0);
                c.r[6] = c.r[23].wrapping_add(0x0);
                c.lr = 0x80098888; f_800c4928(c);
                pc = 0x80098888;
            }
            0x80098888 => {
                c.r[26] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[31].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800988b4; continue; }
                pc = 0x800988a4;
            }
            0x800988a4 => {
                c.r[0] = c.r[31].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[26]);
                pc = 0x800988b4;
            }
            0x800988b4 => {
                c.r[0] = c.r[31].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.r[26].wrapping_sub(c.r[0]);
                c.cmps(0, c.r[0], 0x3);
                if !c.crbit(0) { pc = 0x80098924; continue; }
                pc = 0x800988d0;
            }
            0x800988d0 => {
                c.r[29] = 0x0;
                pc = 0x80098924; continue;
                pc = 0x800988d8;
            }
            0x800988d8 => {
                c.r[4] = 0x0;
                c.stw(c.r[30].wrapping_add(0x20), c.r[4]);
                c.r[0] = c.r[31].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                c.r[29] = 0x0;
                pc = 0x80098924; continue;
                pc = 0x800988f8;
            }
            0x800988f8 => {
                c.r[0] = c.r[28].rotate_left(0) & 0x1000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80098908; continue; }
                pc = 0x80098900;
            }
            0x80098900 => {
                c.r[0] = c.r[28].rotate_left(0) & 0x800;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80098924; continue; }
                pc = 0x80098908;
            }
            0x80098908 => {
                c.r[4] = 0x0;
                c.stw(c.r[30].wrapping_add(0x20), c.r[4]);
                c.r[0] = c.r[31].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                c.r[29] = 0x0;
                pc = 0x80098924;
            }
            0x80098924 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x8009892c; f_8009df64(c);
                pc = 0x8009892c;
            }
            0x8009892c => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x80098930;
            }
            0x80098930 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 21..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80098790: bad pc {:#x}", pc),
        }
    }
}

/// fn_80098944 (0x80098944, 124 bytes)
#[inline(never)]
pub fn f_80098944(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80098944) { return; }
        if c.prof { crate::prof::enter(c, 0x80098944); f_80098944_b(c, 0x80098944); crate::prof::exit(c, 0x80098944); return; }
    }
    f_80098944_b(c, 0x80098944)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80098944_at(c: &mut Cpu, pc: u32) { f_80098944_b(c, pc) }
#[inline(always)]
fn f_80098944_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80098944;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80098944, pc); }
        match pc {
            0x80098944 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[3].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[29] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.lr = 0x80098970; f_80098790(c);
                pc = 0x80098970;
            }
            0x80098970 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x800989a8; continue; }
                pc = 0x80098978;
            }
            0x80098978 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x20));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800989a8; continue; }
                pc = 0x80098984;
            }
            0x80098984 => {
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = c.r[1].wrapping_add(0xc);
                c.lr = 0x80098994; f_80099400(c);
                pc = 0x80098994;
            }
            0x80098994 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800989a4; continue; }
                pc = 0x8009899c;
            }
            0x8009899c => {
                c.r[31] = 0x1;
                pc = 0x800989a8; continue;
                pc = 0x800989a4;
            }
            0x800989a4 => {
                c.r[31] = 0x0;
                pc = 0x800989a8;
            }
            0x800989a8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80098944: bad pc {:#x}", pc),
        }
    }
}

/// fn_800989C0 (0x800989c0, 296 bytes)
#[inline(never)]
pub fn f_800989c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800989c0) { return; }
        if c.prof { crate::prof::enter(c, 0x800989c0); f_800989c0_b(c, 0x800989c0); crate::prof::exit(c, 0x800989c0); return; }
    }
    f_800989c0_b(c, 0x800989c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800989c0_at(c: &mut Cpu, pc: u32) { f_800989c0_b(c, pc) }
#[inline(always)]
fn f_800989c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800989c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800989c0, pc); }
        match pc {
            0x800989c0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                c.r[3] = 0x80400000;
                c.r[30] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[0] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[24] = c.r[30].wrapping_add(c.r[0]);
                c.r[0] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[22] = c.r[30].wrapping_add(c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.lr = 0x800989f8; f_80098790(c);
                pc = 0x800989f8;
            }
            0x800989f8 => {
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                if c.crbit(2) { pc = 0x80098a20; continue; }
                pc = 0x80098a00;
            }
            0x80098a00 => {
                c.r[0] = c.lwz(c.r[22].wrapping_add(0x20));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80098a20; continue; }
                pc = 0x80098a0c;
            }
            0x80098a0c => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = c.r[1].wrapping_add(0x10);
                c.lr = 0x80098a1c; f_80099400(c);
                pc = 0x80098a1c;
            }
            0x80098a1c => {
                c.cmps(0, c.r[3], 0x0);
                pc = 0x80098a20;
            }
            0x80098a20 => {
                c.lr = 0x80098a24; f_8009df3c(c);
                pc = 0x80098a24;
            }
            0x80098a24 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[24].wrapping_add(0x20));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80098a44; continue; }
                pc = 0x80098a34;
            }
            0x80098a34 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80098a3c; f_8009df64(c);
                pc = 0x80098a3c;
            }
            0x80098a3c => {
                c.r[3] = 0x0;
                pc = 0x80098ad4; continue;
                pc = 0x80098a44;
            }
            0x80098a44 => {
                c.r[0] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[29] = c.r[30].wrapping_add(c.r[0]);
                c.lr = 0x80098a50; f_8009df3c(c);
                pc = 0x80098a50;
            }
            0x80098a50 => {
                c.r[25] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80098a70; continue; }
                pc = 0x80098a60;
            }
            0x80098a60 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80098a68; f_80098790(c);
                pc = 0x80098a68;
            }
            0x80098a68 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80098a80; continue; }
                pc = 0x80098a70;
            }
            0x80098a70 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80098a78; f_8009df64(c);
                pc = 0x80098a78;
            }
            0x80098a78 => {
                c.r[26] = 0x0;
                pc = 0x80098ac4; continue;
                pc = 0x80098a80;
            }
            0x80098a80 => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80098a94; f_800986a0(c);
                pc = 0x80098a94;
            }
            0x80098a94 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.stw(c.r[29].wrapping_add(0x8), c.r[0]);
                c.r[3] = 0x100000;
                c.r[0] = c.r[31].wrapping_mul(0x3);
                c.r[3] = srw(c.r[3], c.r[0]);
                c.lr = 0x80098aac; f_8009e38c(c);
                pc = 0x80098aac;
            }
            0x80098aac => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[0] = c.r[0] | 0x8;
                c.stw(c.r[29].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80098ac0; f_8009df64(c);
                pc = 0x80098ac0;
            }
            0x80098ac0 => {
                c.r[26] = 0x1;
                pc = 0x80098ac4;
            }
            0x80098ac4 => {
                c.r[23] = c.r[26].wrapping_add(0x0);
                c.r[3] = c.r[28].wrapping_add(0x0);
                c.lr = 0x80098ad0; f_8009df64(c);
                pc = 0x80098ad0;
            }
            0x80098ad0 => {
                c.r[3] = c.r[23] | c.r[23];
                pc = 0x80098ad4;
            }
            0x80098ad4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 22..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800989c0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80098AE8 (0x80098ae8, 172 bytes)
#[inline(never)]
pub fn f_80098ae8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80098ae8) { return; }
        if c.prof { crate::prof::enter(c, 0x80098ae8); f_80098ae8_b(c, 0x80098ae8); crate::prof::exit(c, 0x80098ae8); return; }
    }
    f_80098ae8_b(c, 0x80098ae8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80098ae8_at(c: &mut Cpu, pc: u32) { f_80098ae8_b(c, pc) }
#[inline(always)]
fn f_80098ae8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80098ae8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80098ae8, pc); }
        match pc {
            0x80098ae8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[3].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[31] = c.r[0].wrapping_add(c.r[4]);
                c.lr = 0x80098b10; f_8009df3c(c);
                pc = 0x80098b10;
            }
            0x80098b10 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80098b30; continue; }
                pc = 0x80098b20;
            }
            0x80098b20 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80098b28; f_8009df64(c);
                pc = 0x80098b28;
            }
            0x80098b28 => {
                c.r[3] = 0x1;
                pc = 0x80098b80; continue;
                pc = 0x80098b30;
            }
            0x80098b30 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80098b58; continue; }
                pc = 0x80098b3c;
            }
            0x80098b3c => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x18));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80098b58; continue; }
                pc = 0x80098b48;
            }
            0x80098b48 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80098b50; f_8009df64(c);
                pc = 0x80098b50;
            }
            0x80098b50 => {
                c.r[3] = 0x0;
                pc = 0x80098b80; continue;
                pc = 0x80098b58;
            }
            0x80098b58 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffff7;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[3] = 0x700000;
                c.r[0] = c.r[29].wrapping_mul(0x3);
                c.r[3] = srw(c.r[3], c.r[0]);
                c.lr = 0x80098b74; f_8009e304(c);
                pc = 0x80098b74;
            }
            0x80098b74 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80098b7c; f_8009df64(c);
                pc = 0x80098b7c;
            }
            0x80098b7c => {
                c.r[3] = 0x1;
                pc = 0x80098b80;
            }
            0x80098b80 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 29..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80098ae8: bad pc {:#x}", pc),
        }
    }
}

/// EXISelect (0x80098b94, 320 bytes)
#[inline(never)]
pub fn f_80098b94(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80098b94) { return; }
        if c.prof { crate::prof::enter(c, 0x80098b94); f_80098b94_b(c, 0x80098b94); crate::prof::exit(c, 0x80098b94); return; }
    }
    f_80098b94_b(c, 0x80098b94)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80098b94_at(c: &mut Cpu, pc: u32) { f_80098b94_b(c, pc) }
#[inline(always)]
fn f_80098b94_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80098b94;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80098b94, pc); }
        match pc {
            0x80098b94 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[27] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x10), c.r[5]);
                c.r[4] = c.r[3].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[31] = c.r[0].wrapping_add(c.r[4]);
                c.lr = 0x80098bc4; f_8009df3c(c);
                pc = 0x80098bc4;
            }
            0x80098bc4 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80098c18; continue; }
                pc = 0x80098bd4;
            }
            0x80098bd4 => {
                c.cmps(0, c.r[30], 0x2);
                if c.crbit(2) { pc = 0x80098c28; continue; }
                pc = 0x80098bdc;
            }
            0x80098bdc => {
                c.cmpu(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x80098c00; continue; }
                pc = 0x80098be4;
            }
            0x80098be4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80098c00; continue; }
                pc = 0x80098bf0;
            }
            0x80098bf0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80098bf8; f_80098790(c);
                pc = 0x80098bf8;
            }
            0x80098bf8 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80098c18; continue; }
                pc = 0x80098c00;
            }
            0x80098c00 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80098c18; continue; }
                pc = 0x80098c0c;
            }
            0x80098c0c => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x18));
                c.cmpu(0, c.r[0], c.r[27]);
                if c.crbit(2) { pc = 0x80098c28; continue; }
                pc = 0x80098c18;
            }
            0x80098c18 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80098c20; f_8009df64(c);
                pc = 0x80098c20;
            }
            0x80098c20 => {
                c.r[3] = 0x0;
                pc = 0x80098cc0; continue;
                pc = 0x80098c28;
            }
            0x80098c28 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0] | 0x4;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.r[30].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.r[29] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[29] = c.r[29] & 0x405;
                c.rc(c.r[29]);
                c.r[0] = 0x1;
                c.r[0] = slw(c.r[0], c.r[27]);
                c.r[3] = c.r[0].rotate_left(7) & 0xffffff80;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[0] = c.r[3] | c.r[0];
                c.r[29] = c.r[29] | c.r[0];
                c.r[0] = c.r[30].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[29]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80098cb4; continue; }
                pc = 0x80098c88;
            }
            0x80098c88 => {
                c.cmps(0, c.r[30], 0x1);
                if c.crbit(2) { pc = 0x80098cac; continue; }
                pc = 0x80098c90;
            }
            0x80098c90 => {
                if !c.crbit(0) { pc = 0x80098cb4; continue; }
                pc = 0x80098c94;
            }
            0x80098c94 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(0) { pc = 0x80098ca0; continue; }
                pc = 0x80098c9c;
            }
            0x80098c9c => {
                pc = 0x80098cb4; continue;
                pc = 0x80098ca0;
            }
            0x80098ca0 => {
                c.r[3] = 0x100000;
                c.lr = 0x80098ca8; f_8009e304(c);
                pc = 0x80098ca8;
            }
            0x80098ca8 => {
                pc = 0x80098cb4; continue;
                pc = 0x80098cac;
            }
            0x80098cac => {
                c.r[3] = 0x20000;
                c.lr = 0x80098cb4; f_8009e304(c);
                pc = 0x80098cb4;
            }
            0x80098cb4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80098cbc; f_8009df64(c);
                pc = 0x80098cbc;
            }
            0x80098cbc => {
                c.r[3] = 0x1;
                pc = 0x80098cc0;
            }
            0x80098cc0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80098b94: bad pc {:#x}", pc),
        }
    }
}

/// EXIDeselect (0x80098cd4, 264 bytes)
#[inline(never)]
pub fn f_80098cd4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80098cd4) { return; }
        if c.prof { crate::prof::enter(c, 0x80098cd4); f_80098cd4_b(c, 0x80098cd4); crate::prof::exit(c, 0x80098cd4); return; }
    }
    f_80098cd4_b(c, 0x80098cd4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80098cd4_at(c: &mut Cpu, pc: u32) { f_80098cd4_b(c, pc) }
#[inline(always)]
fn f_80098cd4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80098cd4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80098cd4, pc); }
        match pc {
            0x80098cd4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[3].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[30] = c.r[0].wrapping_add(c.r[4]);
                c.lr = 0x80098cfc; f_8009df3c(c);
                pc = 0x80098cfc;
            }
            0x80098cfc => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80098d1c; continue; }
                pc = 0x80098d0c;
            }
            0x80098d0c => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80098d14; f_8009df64(c);
                pc = 0x80098d14;
            }
            0x80098d14 => {
                c.r[3] = 0x0;
                pc = 0x80098dc8; continue;
                pc = 0x80098d1c;
            }
            0x80098d1c => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffb;
                c.stw(c.r[30].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.r[29] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[4] = c.r[29] & 0x405;
                c.rc(c.r[4]);
                c.r[0] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80098d8c; continue; }
                pc = 0x80098d60;
            }
            0x80098d60 => {
                c.cmps(0, c.r[31], 0x1);
                if c.crbit(2) { pc = 0x80098d84; continue; }
                pc = 0x80098d68;
            }
            0x80098d68 => {
                if !c.crbit(0) { pc = 0x80098d8c; continue; }
                pc = 0x80098d6c;
            }
            0x80098d6c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(0) { pc = 0x80098d78; continue; }
                pc = 0x80098d74;
            }
            0x80098d74 => {
                pc = 0x80098d8c; continue;
                pc = 0x80098d78;
            }
            0x80098d78 => {
                c.r[3] = 0x100000;
                c.lr = 0x80098d80; f_8009e38c(c);
                pc = 0x80098d80;
            }
            0x80098d80 => {
                pc = 0x80098d8c; continue;
                pc = 0x80098d84;
            }
            0x80098d84 => {
                c.r[3] = 0x20000;
                c.lr = 0x80098d8c; f_8009e38c(c);
                pc = 0x80098d8c;
            }
            0x80098d8c => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80098d94; f_8009df64(c);
                pc = 0x80098d94;
            }
            0x80098d94 => {
                c.cmps(0, c.r[31], 0x2);
                if c.crbit(2) { pc = 0x80098dc4; continue; }
                pc = 0x80098d9c;
            }
            0x80098d9c => {
                c.r[0] = c.r[29].rotate_left(0) & 0x80;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80098dc4; continue; }
                pc = 0x80098da4;
            }
            0x80098da4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80098dac; f_80098790(c);
                pc = 0x80098dac;
            }
            0x80098dac => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80098dbc; continue; }
                pc = 0x80098db4;
            }
            0x80098db4 => {
                c.r[3] = 0x1;
                pc = 0x80098dc8; continue;
                pc = 0x80098dbc;
            }
            0x80098dbc => {
                c.r[3] = 0x0;
                pc = 0x80098dc8; continue;
                pc = 0x80098dc4;
            }
            0x80098dc4 => {
                c.r[3] = 0x1;
                pc = 0x80098dc8;
            }
            0x80098dc8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80098cd4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80098DDC (0x80098ddc, 192 bytes)
#[inline(never)]
pub fn f_80098ddc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80098ddc) { return; }
        if c.prof { crate::prof::enter(c, 0x80098ddc); f_80098ddc_b(c, 0x80098ddc); crate::prof::exit(c, 0x80098ddc); return; }
    }
    f_80098ddc_b(c, 0x80098ddc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80098ddc_at(c: &mut Cpu, pc: u32) { f_80098ddc_b(c, pc) }
#[inline(always)]
fn f_80098ddc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80098ddc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80098ddc, pc); }
        match pc {
            0x80098ddc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffd08); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x2e4); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.sth(c.r[1].wrapping_add(0x8), c.r[3]);
                c.r[27] = c.r[4] | c.r[4];
                c.r[3] = c.lha(c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[3].wrapping_add(0xfffffff7);
                c.r[0] = 0x3;
                c.r[31] = divw(c.r[3], c.r[0]);
                c.r[4] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[28] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.r[30] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[30] & 0x7f5;
                c.rc(c.r[30]);
                c.r[30] = c.r[30] | 0x2;
                c.r[0] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[30]);
                c.r[29] = c.lwz(c.r[28]);
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x80098e88; continue; }
                pc = 0x80098e54;
            }
            0x80098e54 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x80098e5c; f_8009bd60(c);
                pc = 0x80098e5c;
            }
            0x80098e5c => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x80098e64; f_8009bb68(c);
                pc = 0x80098e64;
            }
            0x80098e64 => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[27].wrapping_add(0x0);
                c.r[12] = c.r[29].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x80098e78; (c.dispatch)(c, t); }
                pc = 0x80098e78;
            }
            0x80098e78 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x80098e80; f_8009bd60(c);
                pc = 0x80098e80;
            }
            0x80098e80 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x80098e88; f_8009bb68(c);
                pc = 0x80098e88;
            }
            0x80098e88 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2fc));
                { let mut ea = c.r[1].wrapping_add(0x2e4); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x2f8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80098ddc: bad pc {:#x}", pc),
        }
    }
}

/// fn_80098E9C (0x80098e9c, 348 bytes)
#[inline(never)]
pub fn f_80098e9c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80098e9c) { return; }
        if c.prof { crate::prof::enter(c, 0x80098e9c); f_80098e9c_b(c, 0x80098e9c); crate::prof::exit(c, 0x80098e9c); return; }
    }
    f_80098e9c_b(c, 0x80098e9c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80098e9c_at(c: &mut Cpu, pc: u32) { f_80098e9c_b(c, pc) }
#[inline(always)]
fn f_80098e9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80098e9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80098e9c, pc); }
        match pc {
            0x80098e9c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffcf0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x2e4); for n in 21..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[21] = c.r[3].wrapping_add(0x0);
                c.r[22] = c.r[4].wrapping_add(0x0);
                c.r[3] = c.r[21] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[3].wrapping_add(0xfffffff6);
                c.r[0] = 0x3;
                c.r[31] = divw(c.r[3], c.r[0]);
                c.r[4] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[27] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = 0x80000000;
                c.r[0] = c.r[21] as u16 as i16 as i32 as u32;
                c.r[3] = srw(c.r[3], c.r[0]);
                c.lr = 0x80098ee4; f_8009e304(c);
                pc = 0x80098ee4;
            }
            0x80098ee4 => {
                c.r[0] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.r[29] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.stw(c.r[1].wrapping_add(0x10), c.r[29]);
                c.r[29] = c.r[29] & 0x7f5;
                c.rc(c.r[29]);
                c.r[29] = c.r[29] | 0x8;
                c.r[0] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[29]);
                c.r[26] = c.lwz(c.r[27].wrapping_add(0x4));
                c.cmpu(0, c.r[26], 0x0);
                if c.crbit(2) { pc = 0x80098fe4; continue; }
                pc = 0x80098f24;
            }
            0x80098f24 => {
                c.r[0] = 0x0;
                c.stw(c.r[27].wrapping_add(0x4), c.r[0]);
                c.r[4] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[30] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x3;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80098fb0; continue; }
                pc = 0x80098f48;
            }
            0x80098f48 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80098fa4; continue; }
                pc = 0x80098f54;
            }
            0x80098f54 => {
                c.r[25] = c.lwz(c.r[30].wrapping_add(0x10));
                c.cmps(0, c.r[25], 0x0);
                if c.crbit(2) { pc = 0x80098fa4; continue; }
                pc = 0x80098f60;
            }
            0x80098f60 => {
                c.r[23] = c.lwz(c.r[30].wrapping_add(0x14));
                c.r[3] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[3].wrapping_add(0x4);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.r[24] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[28] = 0x0;
                pc = 0x80098f9c; continue;
                pc = 0x80098f84;
            }
            0x80098f84 => {
                c.r[0] = c.addc(!c.r[28], 0x3, 1);
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[3] = srw(c.r[24], c.r[0]);
                c.stb(c.r[23], c.r[3]);
                c.r[23] = c.r[23].wrapping_add(0x1);
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x80098f9c;
            }
            0x80098f9c => {
                c.cmps(0, c.r[28], c.r[25]);
                if c.crbit(0) { pc = 0x80098f84; continue; }
                pc = 0x80098fa4;
            }
            0x80098fa4 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffc;
                c.stw(c.r[30].wrapping_add(0xc), c.r[0]);
                pc = 0x80098fb0;
            }
            0x80098fb0 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x80098fb8; f_8009bd60(c);
                pc = 0x80098fb8;
            }
            0x80098fb8 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x80098fc0; f_8009bb68(c);
                pc = 0x80098fc0;
            }
            0x80098fc0 => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[22].wrapping_add(0x0);
                c.r[12] = c.r[26].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x80098fd4; (c.dispatch)(c, t); }
                pc = 0x80098fd4;
            }
            0x80098fd4 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x80098fdc; f_8009bd60(c);
                pc = 0x80098fdc;
            }
            0x80098fdc => {
                c.r[3] = c.r[22] | c.r[22];
                c.lr = 0x80098fe4; f_8009bb68(c);
                pc = 0x80098fe4;
            }
            0x80098fe4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x314));
                { let mut ea = c.r[1].wrapping_add(0x2e4); for n in 21..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x310);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80098e9c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80098FF8 (0x80098ff8, 200 bytes)
#[inline(never)]
pub fn f_80098ff8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80098ff8) { return; }
        if c.prof { crate::prof::enter(c, 0x80098ff8); f_80098ff8_b(c, 0x80098ff8); crate::prof::exit(c, 0x80098ff8); return; }
    }
    f_80098ff8_b(c, 0x80098ff8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80098ff8_at(c: &mut Cpu, pc: u32) { f_80098ff8_b(c, pc) }
#[inline(always)]
fn f_80098ff8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80098ff8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80098ff8, pc); }
        match pc {
            0x80098ff8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffd18); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x2d8); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.sth(c.r[1].wrapping_add(0x8), c.r[3]);
                c.r[28] = c.r[4] | c.r[4];
                c.r[3] = c.lha(c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[3].wrapping_add(0xfffffff5);
                c.r[0] = 0x3;
                c.r[31] = divw(c.r[3], c.r[0]);
                c.r[3] = 0x700000;
                c.r[0] = c.r[31].wrapping_mul(0x3);
                c.r[3] = srw(c.r[3], c.r[0]);
                c.lr = 0x80099030; f_8009e304(c);
                pc = 0x80099030;
            }
            0x80099030 => {
                c.r[4] = 0x0;
                c.r[0] = c.r[31].wrapping_mul(0x5);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                c.r[4] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[30] = c.r[0].wrapping_add(c.r[4]);
                c.r[29] = c.lwz(c.r[30].wrapping_add(0x8));
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffff7;
                c.stw(c.r[30].wrapping_add(0xc), c.r[0]);
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x800990ac; continue; }
                pc = 0x80099070;
            }
            0x80099070 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x80099078; f_8009bd60(c);
                pc = 0x80099078;
            }
            0x80099078 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x80099080; f_8009bb68(c);
                pc = 0x80099080;
            }
            0x80099080 => {
                c.r[0] = 0x0;
                c.stw(c.r[30].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[28].wrapping_add(0x0);
                c.r[12] = c.r[29].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x8009909c; (c.dispatch)(c, t); }
                pc = 0x8009909c;
            }
            0x8009909c => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800990a4; f_8009bd60(c);
                pc = 0x800990a4;
            }
            0x800990a4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x800990ac; f_8009bb68(c);
                pc = 0x800990ac;
            }
            0x800990ac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2ec));
                { let mut ea = c.r[1].wrapping_add(0x2d8); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x2e8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80098ff8: bad pc {:#x}", pc),
        }
    }
}

/// EXIInit (0x800990c0, 292 bytes)
#[inline(never)]
pub fn f_800990c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800990c0) { return; }
        if c.prof { crate::prof::enter(c, 0x800990c0); f_800990c0_b(c, 0x800990c0); crate::prof::exit(c, 0x800990c0); return; }
    }
    f_800990c0_b(c, 0x800990c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800990c0_at(c: &mut Cpu, pc: u32) { f_800990c0_b(c, pc) }
#[inline(always)]
fn f_800990c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800990c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800990c0, pc); }
        match pc {
            0x800990c0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[3] = 0x800000;
                c.r[3] = c.r[3].wrapping_add(0xffff8000);
                c.lr = 0x800990d8; f_8009e304(c);
                pc = 0x800990d8;
            }
            0x800990d8 => {
                c.r[0] = 0x0;
                c.r[3] = 0xcc000000;
                c.stw(c.r[3].wrapping_add(0x6800), c.r[0]);
                c.r[0] = 0x0;
                c.r[3] = 0xcc000000;
                c.stw(c.r[3].wrapping_add(0x6814), c.r[0]);
                c.r[0] = 0x0;
                c.r[3] = 0xcc000000;
                c.stw(c.r[3].wrapping_add(0x6828), c.r[0]);
                c.r[0] = 0x2000;
                c.r[3] = 0xcc000000;
                c.stw(c.r[3].wrapping_add(0x6800), c.r[0]);
                c.r[3] = 0x9;
                c.r[4] = 0x800a0000;
                c.r[4] = c.r[4].wrapping_add(0xffff8ddc);
                c.lr = 0x80099118; f_8009df88(c);
                pc = 0x80099118;
            }
            0x80099118 => {
                c.r[3] = 0xa;
                c.r[4] = 0x800a0000;
                c.r[4] = c.r[4].wrapping_add(0xffff8e9c);
                c.lr = 0x80099128; f_8009df88(c);
                pc = 0x80099128;
            }
            0x80099128 => {
                c.r[3] = 0xb;
                c.r[4] = 0x800a0000;
                c.r[4] = c.r[4].wrapping_add(0xffff8ff8);
                c.lr = 0x80099138; f_8009df88(c);
                pc = 0x80099138;
            }
            0x80099138 => {
                c.r[3] = 0xc;
                c.r[4] = 0x800a0000;
                c.r[4] = c.r[4].wrapping_add(0xffff8ddc);
                c.lr = 0x80099148; f_8009df88(c);
                pc = 0x80099148;
            }
            0x80099148 => {
                c.r[3] = 0xd;
                c.r[4] = 0x800a0000;
                c.r[4] = c.r[4].wrapping_add(0xffff8e9c);
                c.lr = 0x80099158; f_8009df88(c);
                pc = 0x80099158;
            }
            0x80099158 => {
                c.r[3] = 0xe;
                c.r[4] = 0x800a0000;
                c.r[4] = c.r[4].wrapping_add(0xffff8ff8);
                c.lr = 0x80099168; f_8009df88(c);
                pc = 0x80099168;
            }
            0x80099168 => {
                c.r[3] = 0xf;
                c.r[4] = 0x800a0000;
                c.r[4] = c.r[4].wrapping_add(0xffff8ddc);
                c.lr = 0x80099178; f_8009df88(c);
                pc = 0x80099178;
            }
            0x80099178 => {
                c.r[3] = 0x10;
                c.r[4] = 0x800a0000;
                c.r[4] = c.r[4].wrapping_add(0xffff8e9c);
                c.lr = 0x80099188; f_8009df88(c);
                pc = 0x80099188;
            }
            0x80099188 => {
                c.lr = 0x8009918c; f_800998b8(c);
                pc = 0x8009918c;
            }
            0x8009918c => {
                c.r[0] = c.r[3].rotate_left(0) & 0x10000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800991d4; continue; }
                pc = 0x80099194;
            }
            0x80099194 => {
                c.r[0] = 0x0;
                c.r[3] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0x30c4), c.r[0]);
                c.r[3] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0x30c0), c.r[0]);
                c.r[0] = 0x0;
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffb3c8);
                c.stw(c.r[3].wrapping_add(0x60), c.r[0]);
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffb3c8);
                c.stw(c.r[3].wrapping_add(0x20), c.r[0]);
                c.r[3] = 0x0;
                c.lr = 0x800991cc; f_80098790(c);
                pc = 0x800991cc;
            }
            0x800991cc => {
                c.r[3] = 0x1;
                c.lr = 0x800991d4; f_80098790(c);
                pc = 0x800991d4;
            }
            0x800991d4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800990c0: bad pc {:#x}", pc),
        }
    }
}

/// EXILock (0x800991e4, 252 bytes)
#[inline(never)]
pub fn f_800991e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800991e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800991e4); f_800991e4_b(c, 0x800991e4); crate::prof::exit(c, 0x800991e4); return; }
    }
    f_800991e4_b(c, 0x800991e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800991e4_at(c: &mut Cpu, pc: u32) { f_800991e4_b(c, pc) }
#[inline(always)]
fn f_800991e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800991e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800991e4, pc); }
        match pc {
            0x800991e4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3].wrapping_add(0x0);
                c.r[28] = c.r[4].wrapping_add(0x0);
                c.r[27] = c.r[5].wrapping_add(0x0);
                c.r[4] = c.r[3].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[31] = c.r[0].wrapping_add(c.r[4]);
                c.lr = 0x80099214; f_8009df3c(c);
                pc = 0x80099214;
            }
            0x80099214 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800992a4; continue; }
                pc = 0x80099224;
            }
            0x80099224 => {
                c.cmpu(0, c.r[27], 0x0);
                if c.crbit(2) { pc = 0x80099294; continue; }
                pc = 0x8009922c;
            }
            0x8009922c => {
                c.r[29] = 0x0;
                pc = 0x8009925c; continue;
                pc = 0x80099234;
            }
            0x80099234 => {
                c.r[3] = c.r[29].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[3].wrapping_add(0x28);
                c.r[0] = c.lwz(c.r[31].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[0], c.r[28]);
                if !c.crbit(2) { pc = 0x80099258; continue; }
                pc = 0x80099248;
            }
            0x80099248 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80099250; f_8009df64(c);
                pc = 0x80099250;
            }
            0x80099250 => {
                c.r[3] = 0x0;
                pc = 0x800992cc; continue;
                pc = 0x80099258;
            }
            0x80099258 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x8009925c;
            }
            0x8009925c => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x24));
                c.cmps(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x80099234; continue; }
                pc = 0x80099268;
            }
            0x80099268 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[3] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[3].wrapping_add(0x2c);
                c.stw(c.r[31].wrapping_add(c.r[0]), c.r[27]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[3] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[3].wrapping_add(0x28);
                c.stw(c.r[31].wrapping_add(c.r[0]), c.r[28]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[31].wrapping_add(0x24), c.r[0]);
                pc = 0x80099294;
            }
            0x80099294 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009929c; f_8009df64(c);
                pc = 0x8009929c;
            }
            0x8009929c => {
                c.r[3] = 0x0;
                pc = 0x800992cc; continue;
                pc = 0x800992a4;
            }
            0x800992a4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0] | 0x10;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x18), c.r[28]);
                c.r[3] = c.r[26].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.lr = 0x800992c0; f_80098110(c);
                pc = 0x800992c0;
            }
            0x800992c0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800992c8; f_8009df64(c);
                pc = 0x800992c8;
            }
            0x800992c8 => {
                c.r[3] = 0x1;
                pc = 0x800992cc;
            }
            0x800992cc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800991e4: bad pc {:#x}", pc),
        }
    }
}

/// EXIUnlock (0x800992e0, 200 bytes)
#[inline(never)]
pub fn f_800992e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800992e0) { return; }
        if c.prof { crate::prof::enter(c, 0x800992e0); f_800992e0_b(c, 0x800992e0); crate::prof::exit(c, 0x800992e0); return; }
    }
    f_800992e0_b(c, 0x800992e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800992e0_at(c: &mut Cpu, pc: u32) { f_800992e0_b(c, pc) }
#[inline(always)]
fn f_800992e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800992e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800992e0, pc); }
        match pc {
            0x800992e0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[3].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[31] = c.r[0].wrapping_add(c.r[4]);
                c.lr = 0x80099308; f_8009df3c(c);
                pc = 0x80099308;
            }
            0x80099308 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80099328; continue; }
                pc = 0x80099318;
            }
            0x80099318 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80099320; f_8009df64(c);
                pc = 0x80099320;
            }
            0x80099320 => {
                c.r[3] = 0x0;
                pc = 0x80099394; continue;
                pc = 0x80099328;
            }
            0x80099328 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffef;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.lr = 0x80099340; f_80098110(c);
                pc = 0x80099340;
            }
            0x80099340 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x24));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(1) { pc = 0x80099388; continue; }
                pc = 0x8009934c;
            }
            0x8009934c => {
                c.r[28] = c.lwz(c.r[31].wrapping_add(0x2c));
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                c.rc(c.r[0]);
                c.stw(c.r[31].wrapping_add(0x24), c.r[0]);
                if !c.crbit(1) { pc = 0x80099374; continue; }
                pc = 0x80099360;
            }
            0x80099360 => {
                c.r[3] = c.r[31].wrapping_add(0x28);
                c.r[4] = c.r[31].wrapping_add(0x30);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[5] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.lr = 0x80099374; f_800c8174(c);
                pc = 0x80099374;
            }
            0x80099374 => {
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[12] = c.r[28].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x80099388; (c.dispatch)(c, t); }
                pc = 0x80099388;
            }
            0x80099388 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80099390; f_8009df64(c);
                pc = 0x80099390;
            }
            0x80099390 => {
                c.r[3] = 0x1;
                pc = 0x80099394;
            }
            0x80099394 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 28..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800992e0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800993A8 (0x800993a8, 40 bytes)
#[inline(never)]
pub fn f_800993a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800993a8) { return; }
        if c.prof { crate::prof::enter(c, 0x800993a8); f_800993a8_b(c, 0x800993a8); crate::prof::exit(c, 0x800993a8); return; }
    }
    f_800993a8_b(c, 0x800993a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800993a8_at(c: &mut Cpu, pc: u32) { f_800993a8_b(c, pc) }
#[inline(always)]
fn f_800993a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800993a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800993a8, pc); }
        match pc {
            0x800993a8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[5] = c.r[3].rotate_left(6) & 0xffffffc0;
                c.r[4] = 0x80400000;
                c.r[0] = c.r[4].wrapping_add(0xffffb3c8);
                c.r[31] = c.r[0].wrapping_add(c.r[5]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                if true { return; }
                return;
            }
            _ => panic!("f_800993a8: bad pc {:#x}", pc),
        }
    }
}

/// fn_800993D0 (0x800993d0, 48 bytes)
#[inline(never)]
pub fn f_800993d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800993d0) { return; }
        if c.prof { crate::prof::enter(c, 0x800993d0); f_800993d0_b(c, 0x800993d0); crate::prof::exit(c, 0x800993d0); return; }
    }
    f_800993d0_b(c, 0x800993d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800993d0_at(c: &mut Cpu, pc: u32) { f_800993d0_b(c, pc) }
#[inline(always)]
fn f_800993d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800993d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800993d0, pc); }
        match pc {
            0x800993d0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[4] = 0x0;
                c.r[5] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800993f0; f_80099400(c);
                pc = 0x800993f0;
            }
            0x800993f0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800993d0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80099400 (0x80099400, 912 bytes)
#[inline(never)]
pub fn f_80099400(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80099400) { return; }
        if c.prof { crate::prof::enter(c, 0x80099400); f_80099400_b(c, 0x80099400); crate::prof::exit(c, 0x80099400); return; }
    }
    f_80099400_b(c, 0x80099400)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80099400_at(c: &mut Cpu, pc: u32) { f_80099400_b(c, pc) }
#[inline(always)]
fn f_80099400_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80099400;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80099400, pc); }
        match pc {
            0x80099400 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffa8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 16..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[26] = c.r[4].wrapping_add(0x0);
                c.r[18] = c.r[5].wrapping_add(0x0);
                c.r[3] = 0x80400000;
                c.r[25] = c.r[3].wrapping_add(0xffffb3c8);
                c.r[0] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[28] = c.r[25].wrapping_add(c.r[0]);
                c.cmps(0, c.r[31], 0x2);
                if !c.crbit(0) { pc = 0x80099520; continue; }
                pc = 0x80099434;
            }
            0x80099434 => {
                c.cmpu(0, c.r[26], 0x0);
                if !c.crbit(2) { pc = 0x80099520; continue; }
                pc = 0x8009943c;
            }
            0x8009943c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80099444; f_80098790(c);
                pc = 0x80099444;
            }
            0x80099444 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80099454; continue; }
                pc = 0x8009944c;
            }
            0x8009944c => {
                c.r[3] = 0x0;
                pc = 0x8009977c; continue;
                pc = 0x80099454;
            }
            0x80099454 => {
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x20));
                c.r[0] = c.r[31].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.cmps(0, c.r[4], c.r[0]);
                if !c.crbit(2) { pc = 0x80099480; continue; }
                pc = 0x80099470;
            }
            0x80099470 => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x1c));
                c.stw(c.r[18], c.r[0]);
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x20));
                pc = 0x8009977c; continue;
                pc = 0x80099480;
            }
            0x80099480 => {
                c.r[0] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[24] = c.r[25].wrapping_add(c.r[0]);
                c.lr = 0x8009948c; f_8009df3c(c);
                pc = 0x8009948c;
            }
            0x8009948c => {
                c.r[20] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[24].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800994ac; continue; }
                pc = 0x8009949c;
            }
            0x8009949c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800994a4; f_80098790(c);
                pc = 0x800994a4;
            }
            0x800994a4 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800994bc; continue; }
                pc = 0x800994ac;
            }
            0x800994ac => {
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x800994b4; f_8009df64(c);
                pc = 0x800994b4;
            }
            0x800994b4 => {
                c.r[21] = 0x0;
                pc = 0x80099500; continue;
                pc = 0x800994bc;
            }
            0x800994bc => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x800994d0; f_800986a0(c);
                pc = 0x800994d0;
            }
            0x800994d0 => {
                c.r[0] = 0x0;
                c.stw(c.r[24].wrapping_add(0x8), c.r[0]);
                c.r[3] = 0x100000;
                c.r[0] = c.r[31].wrapping_mul(0x3);
                c.r[3] = srw(c.r[3], c.r[0]);
                c.lr = 0x800994e8; f_8009e38c(c);
                pc = 0x800994e8;
            }
            0x800994e8 => {
                c.r[0] = c.lwz(c.r[24].wrapping_add(0xc));
                c.r[0] = c.r[0] | 0x8;
                c.stw(c.r[24].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x800994fc; f_8009df64(c);
                pc = 0x800994fc;
            }
            0x800994fc => {
                c.r[21] = 0x1;
                pc = 0x80099500;
            }
            0x80099500 => {
                c.cmps(0, c.r[21], 0x0);
                if !c.crbit(2) { pc = 0x80099510; continue; }
                pc = 0x80099508;
            }
            0x80099508 => {
                c.r[3] = 0x0;
                pc = 0x8009977c; continue;
                pc = 0x80099510;
            }
            0x80099510 => {
                c.r[0] = c.r[31].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.r[22] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x80099520;
            }
            0x80099520 => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[26].wrapping_add(0x0);
                c.cmps(0, c.r[31], 0x2);
                if !c.crbit(0) { pc = 0x80099544; continue; }
                pc = 0x80099530;
            }
            0x80099530 => {
                c.cmpu(0, c.r[26], 0x0);
                if !c.crbit(2) { pc = 0x80099544; continue; }
                pc = 0x80099538;
            }
            0x80099538 => {
                c.r[5] = 0x800a0000;
                c.r[5] = c.r[5].wrapping_add(0xffff93d0);
                pc = 0x80099548; continue;
                pc = 0x80099544;
            }
            0x80099544 => {
                c.r[5] = 0x0;
                pc = 0x80099548;
            }
            0x80099548 => {
                c.lr = 0x8009954c; f_800991e4(c);
                pc = 0x8009954c;
            }
            0x8009954c => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[30] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.rc(c.r[30]);
                if !c.crbit(2) { pc = 0x80099690; continue; }
                pc = 0x80099558;
            }
            0x80099558 => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[26].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.lr = 0x80099568; f_80098b94(c);
                pc = 0x80099568;
            }
            0x80099568 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[30] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.rc(c.r[30]);
                if !c.crbit(2) { pc = 0x80099600; continue; }
                pc = 0x80099574;
            }
            0x80099574 => {
                c.r[0] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[5] = 0x2;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x80099594; f_8009820c(c);
                pc = 0x80099594;
            }
            0x80099594 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[30] = c.r[30] | c.r[0];
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.lr = 0x800995a8; f_80098530(c);
                pc = 0x800995a8;
            }
            0x800995a8 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[30] = c.r[30] | c.r[0];
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[18].wrapping_add(0x0);
                c.r[5] = 0x4;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x800995cc; f_8009820c(c);
                pc = 0x800995cc;
            }
            0x800995cc => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[30] = c.r[30] | c.r[0];
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.lr = 0x800995e0; f_80098530(c);
                pc = 0x800995e0;
            }
            0x800995e0 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[30] = c.r[30] | c.r[0];
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.lr = 0x800995f4; f_80098cd4(c);
                pc = 0x800995f4;
            }
            0x800995f4 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[30] = c.r[30] | c.r[0];
                pc = 0x80099600;
            }
            0x80099600 => {
                c.r[0] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[29] = c.r[25].wrapping_add(c.r[0]);
                c.lr = 0x8009960c; f_8009df3c(c);
                pc = 0x8009960c;
            }
            0x8009960c => {
                c.r[19] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80099628; continue; }
                pc = 0x8009961c;
            }
            0x8009961c => {
                c.r[3] = c.r[19] | c.r[19];
                c.lr = 0x80099624; f_8009df64(c);
                pc = 0x80099624;
            }
            0x80099624 => {
                pc = 0x80099690; continue;
                pc = 0x80099628;
            }
            0x80099628 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffef;
                c.stw(c.r[29].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.lr = 0x80099640; f_80098110(c);
                pc = 0x80099640;
            }
            0x80099640 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x24));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(1) { pc = 0x80099688; continue; }
                pc = 0x8009964c;
            }
            0x8009964c => {
                c.r[16] = c.lwz(c.r[29].wrapping_add(0x2c));
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x24));
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                c.rc(c.r[0]);
                c.stw(c.r[29].wrapping_add(0x24), c.r[0]);
                if !c.crbit(1) { pc = 0x80099674; continue; }
                pc = 0x80099660;
            }
            0x80099660 => {
                c.r[3] = c.r[29].wrapping_add(0x28);
                c.r[4] = c.r[29].wrapping_add(0x30);
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x24));
                c.r[5] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.lr = 0x80099674; f_800c8174(c);
                pc = 0x80099674;
            }
            0x80099674 => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[12] = c.r[16].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x80099688; (c.dispatch)(c, t); }
                pc = 0x80099688;
            }
            0x80099688 => {
                c.r[3] = c.r[19] | c.r[19];
                c.lr = 0x80099690; f_8009df64(c);
                pc = 0x80099690;
            }
            0x80099690 => {
                c.cmps(0, c.r[31], 0x2);
                if !c.crbit(0) { pc = 0x80099768; continue; }
                pc = 0x80099698;
            }
            0x80099698 => {
                c.cmpu(0, c.r[26], 0x0);
                if !c.crbit(2) { pc = 0x80099768; continue; }
                pc = 0x800996a0;
            }
            0x800996a0 => {
                c.r[0] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[27] = c.r[25].wrapping_add(c.r[0]);
                c.lr = 0x800996ac; f_8009df3c(c);
                pc = 0x800996ac;
            }
            0x800996ac => {
                c.r[23] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[27].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800996c8; continue; }
                pc = 0x800996bc;
            }
            0x800996bc => {
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x800996c4; f_8009df64(c);
                pc = 0x800996c4;
            }
            0x800996c4 => {
                pc = 0x80099710; continue;
                pc = 0x800996c8;
            }
            0x800996c8 => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800996ec; continue; }
                pc = 0x800996d4;
            }
            0x800996d4 => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x18));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800996ec; continue; }
                pc = 0x800996e0;
            }
            0x800996e0 => {
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x800996e8; f_8009df64(c);
                pc = 0x800996e8;
            }
            0x800996e8 => {
                pc = 0x80099710; continue;
                pc = 0x800996ec;
            }
            0x800996ec => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffff7;
                c.stw(c.r[27].wrapping_add(0xc), c.r[0]);
                c.r[3] = 0x700000;
                c.r[0] = c.r[31].wrapping_mul(0x3);
                c.r[3] = srw(c.r[3], c.r[0]);
                c.lr = 0x80099708; f_8009e304(c);
                pc = 0x80099708;
            }
            0x80099708 => {
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x80099710; f_8009df64(c);
                pc = 0x80099710;
            }
            0x80099710 => {
                c.lr = 0x80099714; f_8009df3c(c);
                pc = 0x80099714;
            }
            0x80099714 => {
                c.r[17] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.r[31].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[3] = c.r[0].wrapping_sub(c.r[22]);
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[30] = c.r[30] | c.r[0];
                c.rc(c.r[30]);
                if !c.crbit(2) { pc = 0x80099748; continue; }
                pc = 0x8009973c;
            }
            0x8009973c => {
                c.r[0] = c.lwz(c.r[18]);
                c.stw(c.r[28].wrapping_add(0x1c), c.r[0]);
                c.stw(c.r[28].wrapping_add(0x20), c.r[22]);
                pc = 0x80099748;
            }
            0x80099748 => {
                c.r[3] = c.r[17] | c.r[17];
                c.lr = 0x80099750; f_8009df64(c);
                pc = 0x80099750;
            }
            0x80099750 => {
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x80099760; continue; }
                pc = 0x80099758;
            }
            0x80099758 => {
                c.r[3] = 0x0;
                pc = 0x8009977c; continue;
                pc = 0x80099760;
            }
            0x80099760 => {
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x20));
                pc = 0x8009977c; continue;
                pc = 0x80099768;
            }
            0x80099768 => {
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x80099778; continue; }
                pc = 0x80099770;
            }
            0x80099770 => {
                c.r[3] = 0x0;
                pc = 0x8009977c; continue;
                pc = 0x80099778;
            }
            0x80099778 => {
                c.r[3] = 0x1;
                pc = 0x8009977c;
            }
            0x8009977c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x5c));
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 16..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x58);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80099400: bad pc {:#x}", pc),
        }
    }
}

/// __OSFPRInit (0x80099790, 296 bytes)
#[inline(never)]
pub fn f_80099790(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80099790) { return; }
        if c.prof { crate::prof::enter(c, 0x80099790); f_80099790_b(c, 0x80099790); crate::prof::exit(c, 0x80099790); return; }
    }
    f_80099790_b(c, 0x80099790)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80099790_at(c: &mut Cpu, pc: u32) { f_80099790_b(c, pc) }
#[inline(always)]
fn f_80099790_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80099790;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80099790, pc); }
        match pc {
            0x80099790 => {
                c.r[3] = 0; // mfmsr
                c.r[3] = c.r[3] | 0x2000;
                // mtmsr
                c.r[3] = c.mfspr(920);
                c.r[3] = c.r[3].rotate_left(3) & 0x1;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x80099830; continue; }
                pc = 0x800997a8;
            }
            0x800997a8 => {
                c.r[3] = 0x80480000;
                c.r[3] = c.r[3].wrapping_add(0xffffa6b8);
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[1] = x; c.ps1[1] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[2] = x; c.ps1[2] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[3] = x; c.ps1[3] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[4] = x; c.ps1[4] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[5] = x; c.ps1[5] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[6] = x; c.ps1[6] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[7] = x; c.ps1[7] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[8] = x; c.ps1[8] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[9] = x; c.ps1[9] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[10] = x; c.ps1[10] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[11] = x; c.ps1[11] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[12] = x; c.ps1[12] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[13] = x; c.ps1[13] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[14] = x; c.ps1[14] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[15] = x; c.ps1[15] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[16] = x; c.ps1[16] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[17] = x; c.ps1[17] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[18] = x; c.ps1[18] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[19] = x; c.ps1[19] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[20] = x; c.ps1[20] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[21] = x; c.ps1[21] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[22] = x; c.ps1[22] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[23] = x; c.ps1[23] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[24] = x; c.ps1[24] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[25] = x; c.ps1[25] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[26] = x; c.ps1[26] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[27] = x; c.ps1[27] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[28] = x; c.ps1[28] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[29] = x; c.ps1[29] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[30] = x; c.ps1[30] = y; }
                { let (x, y) = (c.f[0], c.ps1[0]); c.f[31] = x; c.ps1[31] = y; }
                pc = 0x80099830;
            }
            0x80099830 => {
                { let ea = c.r[13].wrapping_add(0xffff9e90); c.f[0] = c.lfd(ea); }
                c.f[1] = c.f[0];
                c.f[2] = c.f[0];
                c.f[3] = c.f[0];
                c.f[4] = c.f[0];
                c.f[5] = c.f[0];
                c.f[6] = c.f[0];
                c.f[7] = c.f[0];
                c.f[8] = c.f[0];
                c.f[9] = c.f[0];
                c.f[10] = c.f[0];
                c.f[11] = c.f[0];
                c.f[12] = c.f[0];
                c.f[13] = c.f[0];
                c.f[14] = c.f[0];
                c.f[15] = c.f[0];
                c.f[16] = c.f[0];
                c.f[17] = c.f[0];
                c.f[18] = c.f[0];
                c.f[19] = c.f[0];
                c.f[20] = c.f[0];
                c.f[21] = c.f[0];
                c.f[22] = c.f[0];
                c.f[23] = c.f[0];
                c.f[24] = c.f[0];
                c.f[25] = c.f[0];
                c.f[26] = c.f[0];
                c.f[27] = c.f[0];
                c.f[28] = c.f[0];
                c.f[29] = c.f[0];
                c.f[30] = c.f[0];
                c.f[31] = c.f[0];
                c.fpscr = c.f[0].to_bits() as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_80099790: bad pc {:#x}", pc),
        }
    }
}

/// OSGetConsoleType (0x800998b8, 40 bytes)
#[inline(never)]
pub fn f_800998b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800998b8) { return; }
        if c.prof { crate::prof::enter(c, 0x800998b8); f_800998b8_b(c, 0x800998b8); crate::prof::exit(c, 0x800998b8); return; }
    }
    f_800998b8_b(c, 0x800998b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800998b8_at(c: &mut Cpu, pc: u32) { f_800998b8_b(c, pc) }
#[inline(always)]
fn f_800998b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800998b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800998b8, pc); }
        match pc {
            0x800998b8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9e80));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800998d0; continue; }
                pc = 0x800998c4;
            }
            0x800998c4 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x2c));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800998dc; continue; }
                pc = 0x800998d0;
            }
            0x800998d0 => {
                c.r[3] = 0x10000000;
                c.r[3] = c.r[3].wrapping_add(0x2);
                pc = 0x800998dc; continue;
                pc = 0x800998dc;
            }
            0x800998dc => {
                if true { return; }
                return;
            }
            _ => panic!("f_800998b8: bad pc {:#x}", pc),
        }
    }
}

/// ClearArena (0x800998e0, 296 bytes)
#[inline(never)]
pub fn f_800998e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800998e0) { return; }
        if c.prof { crate::prof::enter(c, 0x800998e0); f_800998e0_b(c, 0x800998e0); crate::prof::exit(c, 0x800998e0); return; }
    }
    f_800998e0_b(c, 0x800998e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800998e0_at(c: &mut Cpu, pc: u32) { f_800998e0_b(c, pc) }
#[inline(always)]
fn f_800998e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800998e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800998e0, pc); }
        match pc {
            0x800998e0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x800998f4; f_800a0290(c);
                pc = 0x800998f4;
            }
            0x800998f4 => {
                c.r[0] = c.r[3].wrapping_add(0x80000000);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80099930; continue; }
                pc = 0x80099900;
            }
            0x80099900 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9eac), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9ea8), c.r[0]);
                c.lr = 0x80099910; f_8009afb0(c);
                pc = 0x80099910;
            }
            0x80099910 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80099918; f_8009afb8(c);
                pc = 0x80099918;
            }
            0x80099918 => {
                c.r[31] = c.r[31].wrapping_sub(c.r[3]);
                c.lr = 0x80099920; f_8009afb8(c);
                pc = 0x80099920;
            }
            0x80099920 => {
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8009992c; f_8000540c(c);
                pc = 0x8009992c;
            }
            0x8009992c => {
                pc = 0x800999f4; continue;
                pc = 0x80099930;
            }
            0x80099930 => {
                c.r[4] = 0x81300000;
                c.r[3] = c.lwz(c.r[4].wrapping_add(0xffffdff0));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xffffdfec));
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffff9eac), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffff9ea8), c.r[0]);
                if !c.crbit(2) { pc = 0x80099970; continue; }
                pc = 0x8009994c;
            }
            0x8009994c => {
                c.lr = 0x80099950; f_8009afb0(c);
                pc = 0x80099950;
            }
            0x80099950 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80099958; f_8009afb8(c);
                pc = 0x80099958;
            }
            0x80099958 => {
                c.r[31] = c.r[31].wrapping_sub(c.r[3]);
                c.lr = 0x80099960; f_8009afb8(c);
                pc = 0x80099960;
            }
            0x80099960 => {
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x8009996c; f_8000540c(c);
                pc = 0x8009996c;
            }
            0x8009996c => {
                pc = 0x800999f4; continue;
                pc = 0x80099970;
            }
            0x80099970 => {
                c.lr = 0x80099974; f_8009afb8(c);
                pc = 0x80099974;
            }
            0x80099974 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9eac));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x800999f4; continue; }
                pc = 0x80099980;
            }
            0x80099980 => {
                c.lr = 0x80099984; f_8009afb0(c);
                pc = 0x80099984;
            }
            0x80099984 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9eac));
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(1) { pc = 0x800999b4; continue; }
                pc = 0x80099990;
            }
            0x80099990 => {
                c.lr = 0x80099994; f_8009afb0(c);
                pc = 0x80099994;
            }
            0x80099994 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8009999c; f_8009afb8(c);
                pc = 0x8009999c;
            }
            0x8009999c => {
                c.r[31] = c.r[31].wrapping_sub(c.r[3]);
                c.lr = 0x800999a4; f_8009afb8(c);
                pc = 0x800999a4;
            }
            0x800999a4 => {
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x800999b0; f_8000540c(c);
                pc = 0x800999b0;
            }
            0x800999b0 => {
                pc = 0x800999f4; continue;
                pc = 0x800999b4;
            }
            0x800999b4 => {
                c.lr = 0x800999b8; f_8009afb8(c);
                pc = 0x800999b8;
            }
            0x800999b8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9eac));
                c.r[31] = c.r[0].wrapping_sub(c.r[3]);
                c.lr = 0x800999c4; f_8009afb8(c);
                pc = 0x800999c4;
            }
            0x800999c4 => {
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x800999d0; f_8000540c(c);
                pc = 0x800999d0;
            }
            0x800999d0 => {
                c.lr = 0x800999d4; f_8009afb0(c);
                pc = 0x800999d4;
            }
            0x800999d4 => {
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffff9ea8));
                c.cmpu(0, c.r[3], c.r[31]);
                if !c.crbit(1) { pc = 0x800999f4; continue; }
                pc = 0x800999e0;
            }
            0x800999e0 => {
                c.lr = 0x800999e4; f_8009afb0(c);
                pc = 0x800999e4;
            }
            0x800999e4 => {
                c.r[5] = c.r[3].wrapping_sub(c.r[31]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x800999f4; f_8000540c(c);
                pc = 0x800999f4;
            }
            0x800999f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800998e0: bad pc {:#x}", pc),
        }
    }
}

/// InquiryCallback (0x80099a08, 60 bytes)
#[inline(never)]
pub fn f_80099a08(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80099a08) { return; }
        if c.prof { crate::prof::enter(c, 0x80099a08); f_80099a08_b(c, 0x80099a08); crate::prof::exit(c, 0x80099a08); return; }
    }
    f_80099a08_b(c, 0x80099a08)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80099a08_at(c: &mut Cpu, pc: u32) { f_80099a08_b(c, pc) }
#[inline(always)]
fn f_80099a08_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80099a08;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80099a08, pc); }
        match pc {
            0x80099a08 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80099a18; continue; }
                pc = 0x80099a14;
            }
            0x80099a14 => {
                pc = 0x80099a34; continue;
                pc = 0x80099a18;
            }
            0x80099a18 => {
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffb4a0);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x2));
                c.r[3] = 0x80000000;
                c.r[0] = c.r[0] | 0x8000;
                c.sth(c.r[3].wrapping_add(0x30e6), c.r[0]);
                pc = 0x80099a40; continue;
                pc = 0x80099a34;
            }
            0x80099a34 => {
                c.r[0] = 0x1;
                c.r[3] = 0x80000000;
                c.sth(c.r[3].wrapping_add(0x30e6), c.r[0]);
                pc = 0x80099a40;
            }
            0x80099a40 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80099a08: bad pc {:#x}", pc),
        }
    }
}

/// OSInit (0x80099a44, 984 bytes)
#[inline(never)]
pub fn f_80099a44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80099a44) { return; }
        if c.prof { crate::prof::enter(c, 0x80099a44); f_80099a44_b(c, 0x80099a44); crate::prof::exit(c, 0x80099a44); return; }
    }
    f_80099a44_b(c, 0x80099a44)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80099a44_at(c: &mut Cpu, pc: u32) { f_80099a44_b(c, pc) }
#[inline(always)]
fn f_80099a44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80099a44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80099a44, pc); }
        match pc {
            0x80099a44 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[29]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9ea0));
                c.r[3] = 0x80400000;
                c.r[30] = c.r[3].wrapping_add(0xffffb4a0);
                c.cmps(0, c.r[0], 0x0);
                c.r[3] = 0x80310000;
                c.r[31] = c.r[3].wrapping_add(0xffffffa0);
                if !c.crbit(2) { pc = 0x80099e00; continue; }
                pc = 0x80099a78;
            }
            0x80099a78 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9ea0), c.r[0]);
                c.lr = 0x80099a84; f_800a2798(c);
                pc = 0x80099a84;
            }
            0x80099a84 => {
                c.stw(c.r[13].wrapping_add(0xffff9ebc), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffff9eb8), c.r[3]);
                c.lr = 0x80099a90; f_8009df3c(c);
                pc = 0x80099a90;
            }
            0x80099a90 => {
                c.r[3] = 0x0;
                c.lr = 0x80099a98; f_80098050(c);
                pc = 0x80099a98;
            }
            0x80099a98 => {
                c.r[3] = 0x0;
                c.lr = 0x80099aa0; f_80098058(c);
                pc = 0x80099aa0;
            }
            0x80099aa0 => {
                c.r[3] = 0x0;
                c.lr = 0x80099aa8; f_80098060(c);
                pc = 0x80099aa8;
            }
            0x80099aa8 => {
                c.r[3] = 0x0;
                c.lr = 0x80099ab0; f_80098068(c);
                pc = 0x80099ab0;
            }
            0x80099ab0 => {
                c.r[3] = 0x0;
                c.lr = 0x80099ab8; f_80098070(c);
                pc = 0x80099ab8;
            }
            0x80099ab8 => {
                c.r[3] = 0x0;
                c.lr = 0x80099ac0; f_80098078(c);
                pc = 0x80099ac0;
            }
            0x80099ac0 => {
                c.lr = 0x80099ac4; f_800980e0(c);
                pc = 0x80099ac4;
            }
            0x80099ac4 => {
                c.lr = 0x80099ac8; f_80098108(c);
                pc = 0x80099ac8;
            }
            0x80099ac8 => {
                c.r[0] = 0x0;
                c.r[4] = 0x80000000;
                c.stw(c.r[13].wrapping_add(0xffff9e84), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9e80), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffff9fbc), c.r[0]);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0xf4));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80099b18; continue; }
                pc = 0x80099ae8;
            }
            0x80099ae8 => {
                c.r[0] = c.r[3].wrapping_add(0xc);
                c.stw(c.r[13].wrapping_add(0xffff9e84), c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x24));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9e84));
                c.stw(c.r[13].wrapping_add(0xffffa0a4), c.r[0]);
                c.r[0] = c.lwz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.stb(c.r[4].wrapping_add(0x30e8), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa0a4));
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.stb(c.r[4].wrapping_add(0x30e9), c.r[0]);
                pc = 0x80099b3c; continue;
                pc = 0x80099b18;
            }
            0x80099b18 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x34));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80099b3c; continue; }
                pc = 0x80099b24;
            }
            0x80099b24 => {
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x30e8));
                c.r[0] = c.r[13].wrapping_add(0xffff9e88);
                c.stw(c.r[13].wrapping_add(0xffff9e88), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffff9e84), c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x30e9));
                c.stw(c.r[13].wrapping_add(0xffffa0a4), c.r[0]);
                pc = 0x80099b3c;
            }
            0x80099b3c => {
                c.r[0] = 0x1;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9e80));
                c.stw(c.r[13].wrapping_add(0xffff9fbc), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x30));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80099b60; continue; }
                pc = 0x80099b54;
            }
            0x80099b54 => {
                c.r[3] = 0x80490000;
                c.r[3] = c.r[3].wrapping_add(0x6740);
                pc = 0x80099b60; continue;
                pc = 0x80099b60;
            }
            0x80099b60 => {
                c.lr = 0x80099b64; f_8009afc8(c);
                pc = 0x80099b64;
            }
            0x80099b64 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9e80));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x30));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80099ba0; continue; }
                pc = 0x80099b74;
            }
            0x80099b74 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9e84));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80099ba0; continue; }
                pc = 0x80099b80;
            }
            0x80099b80 => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(0) { pc = 0x80099ba0; continue; }
                pc = 0x80099b8c;
            }
            0x80099b8c => {
                c.r[3] = 0x80490000;
                c.r[3] = c.r[3].wrapping_add(0xffffe738);
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.lr = 0x80099ba0; f_8009afc8(c);
                pc = 0x80099ba0;
            }
            0x80099ba0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9e80));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x34));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80099bbc; continue; }
                pc = 0x80099bb0;
            }
            0x80099bb0 => {
                c.r[3] = 0x81700000;
                c.r[3] = c.r[3].wrapping_add(0x0);
                pc = 0x80099bbc; continue;
                pc = 0x80099bbc;
            }
            0x80099bbc => {
                c.lr = 0x80099bc0; f_8009afc0(c);
                pc = 0x80099bc0;
            }
            0x80099bc0 => {
                c.lr = 0x80099bc4; f_80099e1c(c);
                pc = 0x80099bc4;
            }
            0x80099bc4 => {
                c.lr = 0x80099bc8; f_800a1228(c);
                pc = 0x80099bc8;
            }
            0x80099bc8 => {
                c.lr = 0x80099bcc; f_8009a27c(c);
                pc = 0x80099bcc;
            }
            0x80099bcc => {
                c.lr = 0x80099bd0; f_8009f1b8(c);
                pc = 0x80099bd0;
            }
            0x80099bd0 => {
                c.lr = 0x80099bd4; f_8009dfb8(c);
                pc = 0x80099bd4;
            }
            0x80099bd4 => {
                c.r[3] = 0x800a0000;
                c.r[4] = c.r[3].wrapping_add(0x2c0);
                c.r[3] = 0x16;
                c.lr = 0x80099be4; f_8009df88(c);
                pc = 0x80099be4;
            }
            0x80099be4 => {
                c.lr = 0x80099be8; f_8009c16c(c);
                pc = 0x80099be8;
            }
            0x80099be8 => {
                c.lr = 0x80099bec; f_8009b820(c);
                pc = 0x80099bec;
            }
            0x80099bec => {
                c.lr = 0x80099bf0; f_800990c0(c);
                pc = 0x80099bf0;
            }
            0x80099bf0 => {
                c.lr = 0x80099bf4; f_800cfffc(c);
                pc = 0x80099bf4;
            }
            0x80099bf4 => {
                c.lr = 0x80099bf8; f_800a07c4(c);
                pc = 0x80099bf8;
            }
            0x80099bf8 => {
                c.lr = 0x80099bfc; f_800a1290(c);
                pc = 0x80099bfc;
            }
            0x80099bfc => {
                c.lr = 0x80099c00; f_8009affc(c);
                pc = 0x80099c00;
            }
            0x80099c00 => {
                c.lr = 0x80099c04; f_800980c8(c);
                pc = 0x80099c04;
            }
            0x80099c04 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xbfffffff;
                c.lr = 0x80099c0c; f_800980d0(c);
                pc = 0x80099c0c;
            }
            0x80099c0c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9eb0));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80099c1c; continue; }
                pc = 0x80099c18;
            }
            0x80099c18 => {
                c.lr = 0x80099c1c; f_8009f664(c);
                pc = 0x80099c1c;
            }
            0x80099c1c => {
                c.r[3] = c.r[31].wrapping_add(0x44);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80099c28; f_8009c2e0(c);
                pc = 0x80099c28;
            }
            0x80099c28 => {
                c.r[3] = c.r[31].wrapping_add(0x54);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = c.r[31].wrapping_add(0x6c);
                c.r[5] = c.r[31].wrapping_add(0x78);
                c.lr = 0x80099c3c; f_8009c2e0(c);
                pc = 0x80099c3c;
            }
            0x80099c3c => {
                c.r[3] = c.r[31].wrapping_add(0x84);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80099c48; f_8009c2e0(c);
                pc = 0x80099c48;
            }
            0x80099c48 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9e80));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80099c60; continue; }
                pc = 0x80099c54;
            }
            0x80099c54 => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x2c));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x80099c6c; continue; }
                pc = 0x80099c60;
            }
            0x80099c60 => {
                c.r[3] = 0x10000000;
                c.r[4] = c.r[3].wrapping_add(0x2);
                pc = 0x80099c6c; continue;
                pc = 0x80099c6c;
            }
            0x80099c6c => {
                c.r[3] = c.r[4].rotate_left(0) & 0xf0000000;
                c.r[0] = 0x10000000;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x80099cac; continue; }
                pc = 0x80099c7c;
            }
            0x80099c7c => {
                if !c.crbit(0) { pc = 0x80099c8c; continue; }
                pc = 0x80099c80;
            }
            0x80099c80 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80099c9c; continue; }
                pc = 0x80099c88;
            }
            0x80099c88 => {
                pc = 0x80099d40; continue;
                pc = 0x80099c8c;
            }
            0x80099c8c => {
                c.r[0] = 0x20000000;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x80099cac; continue; }
                pc = 0x80099c98;
            }
            0x80099c98 => {
                pc = 0x80099d40; continue;
                pc = 0x80099c9c;
            }
            0x80099c9c => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.lr = 0x80099ca8; f_8009c2e0(c);
                pc = 0x80099ca8;
            }
            0x80099ca8 => {
                pc = 0x80099d4c; continue;
                pc = 0x80099cac;
            }
            0x80099cac => {
                c.r[3] = 0x10000000;
                c.r[5] = c.r[4].rotate_left(0) & 0xfffffff;
                c.r[0] = c.r[3].wrapping_add(0x2);
                c.cmps(0, c.r[5], c.r[0]);
                if c.crbit(2) { pc = 0x80099d04; continue; }
                pc = 0x80099cc0;
            }
            0x80099cc0 => {
                if !c.crbit(0) { pc = 0x80099cd4; continue; }
                pc = 0x80099cc4;
            }
            0x80099cc4 => {
                c.cmps(0, c.r[5], c.r[3]);
                if c.crbit(2) { pc = 0x80099ce4; continue; }
                pc = 0x80099ccc;
            }
            0x80099ccc => {
                if !c.crbit(0) { pc = 0x80099cf4; continue; }
                pc = 0x80099cd0;
            }
            0x80099cd0 => {
                pc = 0x80099d24; continue;
                pc = 0x80099cd4;
            }
            0x80099cd4 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                c.cmps(0, c.r[5], c.r[0]);
                if !c.crbit(0) { pc = 0x80099d24; continue; }
                pc = 0x80099ce0;
            }
            0x80099ce0 => {
                pc = 0x80099d14; continue;
                pc = 0x80099ce4;
            }
            0x80099ce4 => {
                c.r[3] = c.r[31].wrapping_add(0xa0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80099cf0; f_8009c2e0(c);
                pc = 0x80099cf0;
            }
            0x80099cf0 => {
                pc = 0x80099d4c; continue;
                pc = 0x80099cf4;
            }
            0x80099cf4 => {
                c.r[3] = c.r[31].wrapping_add(0xb0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80099d00; f_8009c2e0(c);
                pc = 0x80099d00;
            }
            0x80099d00 => {
                pc = 0x80099d4c; continue;
                pc = 0x80099d04;
            }
            0x80099d04 => {
                c.r[3] = c.r[31].wrapping_add(0xc0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80099d10; f_8009c2e0(c);
                pc = 0x80099d10;
            }
            0x80099d10 => {
                pc = 0x80099d4c; continue;
                pc = 0x80099d14;
            }
            0x80099d14 => {
                c.r[3] = c.r[31].wrapping_add(0xd0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80099d20; f_8009c2e0(c);
                pc = 0x80099d20;
            }
            0x80099d20 => {
                pc = 0x80099d4c; continue;
                pc = 0x80099d24;
            }
            0x80099d24 => {
                c.r[6] = c.r[4].rotate_left(0) & 0xfffffff;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[5] = c.r[4] | c.r[4];
                c.r[3] = c.r[31].wrapping_add(0xe0);
                c.r[4] = c.r[6].wrapping_add(0xfffffffd);
                c.lr = 0x80099d3c; f_8009c2e0(c);
                pc = 0x80099d3c;
            }
            0x80099d3c => {
                pc = 0x80099d4c; continue;
                pc = 0x80099d40;
            }
            0x80099d40 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[3] = c.r[13].wrapping_add(0xffff8154);
                c.lr = 0x80099d4c; f_8009c2e0(c);
                pc = 0x80099d4c;
            }
            0x80099d4c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9e80));
                c.r[3] = c.r[31].wrapping_add(0xfc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x28));
                c.r[4] = c.r[0].rotate_left(12) & 0xfff;
                c.lr = 0x80099d64; f_8009c2e0(c);
                pc = 0x80099d64;
            }
            0x80099d64 => {
                c.lr = 0x80099d68; f_8009afb0(c);
                pc = 0x80099d68;
            }
            0x80099d68 => {
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x80099d70; f_8009afb8(c);
                pc = 0x80099d70;
            }
            0x80099d70 => {
                c.r[4] = c.r[3] | c.r[3];
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = c.r[31].wrapping_add(0x10c);
                c.lr = 0x80099d84; f_8009c2e0(c);
                pc = 0x80099d84;
            }
            0x80099d84 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8150));
                c.lr = 0x80099d8c; f_8009a250(c);
                pc = 0x80099d8c;
            }
            0x80099d8c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9e84));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80099da8; continue; }
                pc = 0x80099d98;
            }
            0x80099d98 => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x80099da8; continue; }
                pc = 0x80099da4;
            }
            0x80099da4 => {
                c.lr = 0x80099da8; f_800c339c(c);
                pc = 0x80099da8;
            }
            0x80099da8 => {
                c.lr = 0x80099dac; f_800998e0(c);
                pc = 0x80099dac;
            }
            0x80099dac => {
                c.lr = 0x80099db0; f_8009df50(c);
                pc = 0x80099db0;
            }
            0x80099db0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9eb0));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80099e00; continue; }
                pc = 0x80099dbc;
            }
            0x80099dbc => {
                c.lr = 0x80099dc0; f_800a5624(c);
                pc = 0x80099dc0;
            }
            0x80099dc0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9e8c));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80099de0; continue; }
                pc = 0x80099dcc;
            }
            0x80099dcc => {
                c.r[3] = 0x10000;
                c.r[0] = c.r[3].wrapping_add(0xffff9000);
                c.r[3] = 0x80000000;
                c.sth(c.r[3].wrapping_add(0x30e6), c.r[0]);
                pc = 0x80099e00; continue;
                pc = 0x80099de0;
            }
            0x80099de0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x20;
                c.lr = 0x80099dec; f_8009b2a4(c);
                pc = 0x80099dec;
            }
            0x80099dec => {
                c.r[3] = 0x800a0000;
                c.r[5] = c.r[3].wrapping_add(0xffff9a08);
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[30].wrapping_add(0x20);
                c.lr = 0x80099e00; f_800a7614(c);
                pc = 0x80099e00;
            }
            0x80099e00 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80099a44: bad pc {:#x}", pc),
        }
    }
}

/// OSExceptionInit (0x80099e1c, 640 bytes)
#[inline(never)]
pub fn f_80099e1c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80099e1c) { return; }
        if c.prof { crate::prof::enter(c, 0x80099e1c); f_80099e1c_b(c, 0x80099e1c); crate::prof::exit(c, 0x80099e1c); return; }
    }
    f_80099e1c_b(c, 0x80099e1c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80099e1c_at(c: &mut Cpu, pc: u32) { f_80099e1c_b(c, pc) }
#[inline(always)]
fn f_80099e1c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80099e1c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80099e1c, pc); }
        match pc {
            0x80099e1c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 20..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x60));
                c.r[4] = 0x800a0000;
                c.r[30] = c.r[4].wrapping_add(0xffffa15c);
                c.r[5] = 0x800a0000;
                c.r[25] = c.lwz(c.r[30]);
                c.r[4] = 0x800a0000;
                c.r[5] = c.r[5].wrapping_add(0xffffa0f4);
                c.r[4] = c.r[4].wrapping_add(0xffffa18c);
                c.r[6] = 0x80310000;
                c.cmpu(0, c.r[0], 0x0);
                c.r[24] = c.r[5] | c.r[5];
                c.r[29] = c.r[6].wrapping_add(0xffffffa0);
                c.r[23] = c.r[4].wrapping_sub(c.r[5]);
                c.r[20] = c.r[3].wrapping_add(0x60);
                if !c.crbit(2) { pc = 0x80099eb4; continue; }
                pc = 0x80099e6c;
            }
            0x80099e6c => {
                c.r[3] = c.r[29].wrapping_add(0x160);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80099e78; f_800a2ce8(c);
                pc = 0x80099e78;
            }
            0x80099e78 => {
                c.r[4] = 0x800a0000;
                c.r[3] = 0x800a0000;
                c.r[0] = c.r[3].wrapping_add(0xffffa0c0);
                c.r[4] = c.r[4].wrapping_add(0xffffa09c);
                c.r[21] = c.r[0].wrapping_sub(c.r[4]);
                c.r[3] = c.r[20] | c.r[20];
                c.r[5] = c.r[21] | c.r[21];
                c.lr = 0x80099e98; f_800054f4(c);
                pc = 0x80099e98;
            }
            0x80099e98 => {
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x80099ea4; f_8009b330(c);
                pc = 0x80099ea4;
            }
            0x80099ea4 => {
                // sync
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = c.r[21] | c.r[21];
                c.lr = 0x80099eb4; f_8009b3b4(c);
                pc = 0x80099eb4;
            }
            0x80099eb4 => {
                c.r[4] = 0x800a0000;
                c.r[3] = 0x800a0000;
                c.r[31] = c.r[4].wrapping_add(0xffffa0c0);
                c.r[0] = c.r[3].wrapping_add(0xffffa0c4);
                c.r[28] = c.r[29].wrapping_add(0x124);
                c.r[27] = c.r[0].wrapping_sub(c.r[31]);
                c.r[26] = 0x0;
                pc = 0x80099ed4; continue;
                pc = 0x80099ed4;
            }
            0x80099ed4 => {
                c.r[3] = 0x800a0000;
                c.r[21] = c.r[3].wrapping_add(0xffffa14c);
                c.r[22] = 0x60000000;
                pc = 0x80099ee4; continue;
                pc = 0x80099ee4;
            }
            0x80099ee4 => {
                pc = 0x8009a02c; continue;
                pc = 0x80099ee8;
            }
            0x80099ee8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9e84));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80099f24; continue; }
                pc = 0x80099ef4;
            }
            0x80099ef4 => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x80099f24; continue; }
                pc = 0x80099f00;
            }
            0x80099f00 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x80099f08; f_800a2ccc(c);
                pc = 0x80099f08;
            }
            0x80099f08 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80099f24; continue; }
                pc = 0x80099f10;
            }
            0x80099f10 => {
                c.r[3] = c.r[29].wrapping_add(0x17c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = c.r[26].rotate_left(0) & 0xff;
                c.lr = 0x80099f20; f_800a2ce8(c);
                pc = 0x80099f20;
            }
            0x80099f20 => {
                pc = 0x8009a024; continue;
                pc = 0x80099f24;
            }
            0x80099f24 => {
                c.r[20] = c.r[26].rotate_left(0) & 0xff;
                c.r[0] = c.r[25] | c.r[20];
                c.stw(c.r[30], c.r[0]);
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x80099f38; f_800a2ccc(c);
                pc = 0x80099f38;
            }
            0x80099f38 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80099f64; continue; }
                pc = 0x80099f40;
            }
            0x80099f40 => {
                c.r[4] = c.r[20] | c.r[20];
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[3] = c.r[29].wrapping_add(0x1ac);
                c.lr = 0x80099f50; f_800a2ce8(c);
                pc = 0x80099f50;
            }
            0x80099f50 => {
                c.r[3] = c.r[21] | c.r[21];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = c.r[27] | c.r[27];
                c.lr = 0x80099f60; f_800054f4(c);
                pc = 0x80099f60;
            }
            0x80099f60 => {
                pc = 0x80099ff0; continue;
                pc = 0x80099f64;
            }
            0x80099f64 => {
                c.r[4] = c.r[21] | c.r[21];
                pc = 0x80099f6c; continue;
                pc = 0x80099f6c;
            }
            0x80099f6c => {
                c.cmpu(0, c.r[27], 0x0);
                c.r[3] = c.r[27].wrapping_add(0x3);
                c.r[3] = c.r[3].rotate_left(30) & 0x3fffffff;
                if !c.crbit(1) { pc = 0x80099ff0; continue; }
                pc = 0x80099f7c;
            }
            0x80099f7c => {
                c.r[0] = c.r[3].rotate_left(29) & 0x1fffffff;
                c.cmpu(0, c.r[0], 0x0);
                c.mtspr(9, c.r[0]);
                if c.crbit(2) { pc = 0x80099fdc; continue; }
                pc = 0x80099f8c;
            }
            0x80099f8c => {
                pc = 0x80099f90; continue;
                pc = 0x80099f90;
            }
            0x80099f90 => {
                c.stw(c.r[4], c.r[22]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.stw(c.r[4], c.r[22]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.stw(c.r[4], c.r[22]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.stw(c.r[4], c.r[22]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.stw(c.r[4], c.r[22]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.stw(c.r[4], c.r[22]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.stw(c.r[4], c.r[22]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.stw(c.r[4], c.r[22]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80099f90; continue; }
                pc = 0x80099fd4;
            }
            0x80099fd4 => {
                c.r[3] = c.r[3] & 0x7;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x80099ff0; continue; }
                pc = 0x80099fdc;
            }
            0x80099fdc => {
                c.mtspr(9, c.r[3]);
                pc = 0x80099fe4; continue;
                pc = 0x80099fe4;
            }
            0x80099fe4 => {
                c.stw(c.r[4], c.r[22]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80099fe4; continue; }
                pc = 0x80099ff0;
            }
            0x80099ff0 => {
                c.r[3] = c.lwz(c.r[28]);
                c.r[4] = c.r[24] | c.r[24];
                c.r[5] = c.r[23] | c.r[23];
                c.r[20] = c.r[3].wrapping_add(0x80000000);
                c.r[3] = c.r[20] | c.r[20];
                c.lr = 0x8009a008; f_800054f4(c);
                pc = 0x8009a008;
            }
            0x8009a008 => {
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8009a014; f_8009b330(c);
                pc = 0x8009a014;
            }
            0x8009a014 => {
                // sync
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8009a024; f_8009b3b4(c);
                pc = 0x8009a024;
            }
            0x8009a024 => {
                c.r[28] = c.r[28].wrapping_add(0x4);
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x8009a02c;
            }
            0x8009a02c => {
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0xf);
                if c.crbit(0) { pc = 0x80099ee8; continue; }
                pc = 0x8009a038;
            }
            0x8009a038 => {
                c.r[3] = 0x80000000;
                c.r[0] = c.r[3].wrapping_add(0x3000);
                c.stw(c.r[13].wrapping_add(0xffff9ea4), c.r[0]);
                c.r[20] = 0x0;
                pc = 0x8009a04c; continue;
                pc = 0x8009a04c;
            }
            0x8009a04c => {
                c.r[3] = 0x800a0000;
                c.r[23] = c.r[3].wrapping_add(0xffffa190);
                pc = 0x8009a058; continue;
                pc = 0x8009a058;
            }
            0x8009a058 => {
                pc = 0x8009a06c; continue;
                pc = 0x8009a05c;
            }
            0x8009a05c => {
                c.r[3] = c.r[20] | c.r[20];
                c.r[4] = c.r[23] | c.r[23];
                c.lr = 0x8009a068; f_8009a0c4(c);
                pc = 0x8009a068;
            }
            0x8009a068 => {
                c.r[20] = c.r[20].wrapping_add(0x1);
                pc = 0x8009a06c;
            }
            0x8009a06c => {
                c.r[0] = c.r[20].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0xf);
                if c.crbit(0) { pc = 0x8009a05c; continue; }
                pc = 0x8009a078;
            }
            0x8009a078 => {
                c.stw(c.r[30], c.r[25]);
                c.r[3] = c.r[29].wrapping_add(0x1dc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009a088; f_800a2ce8(c);
                pc = 0x8009a088;
            }
            0x8009a088 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 20..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80099e1c: bad pc {:#x}", pc),
        }
    }
}

/// __OSDBIntegrator (0x8009a09c, 36 bytes)
#[inline(never)]
pub fn f_8009a09c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a09c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a09c); f_8009a09c_b(c, 0x8009a09c); crate::prof::exit(c, 0x8009a09c); return; }
    }
    f_8009a09c_b(c, 0x8009a09c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a09c_at(c: &mut Cpu, pc: u32) { f_8009a09c_b(c, pc) }
#[inline(always)]
fn f_8009a09c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a09c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a09c, pc); }
        match pc {
            0x8009a09c => {
                c.r[5] = 0x40;
                c.r[3] = c.mfspr(8);
                c.stw(c.r[5].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x8));
                c.r[3] = c.r[3] | 0x80000000;
                c.mtspr(8, c.r[3]);
                c.r[3] = 0x30;
                // mtmsr
                if true { return; }
                return;
            }
            _ => panic!("f_8009a09c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009A0C0 (0x8009a0c0, 4 bytes)
#[inline(never)]
pub fn f_8009a0c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a0c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a0c0); f_8009a0c0_b(c, 0x8009a0c0); crate::prof::exit(c, 0x8009a0c0); return; }
    }
    f_8009a0c0_b(c, 0x8009a0c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a0c0_at(c: &mut Cpu, pc: u32) { f_8009a0c0_b(c, pc) }
#[inline(always)]
fn f_8009a0c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a0c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a0c0, pc); }
        match pc {
            0x8009a0c0 => {
                c.lr = 0x8009a0c4; (c.dispatch)(c, 0x60);
                f_8009a0c4(c); return;
                return;
            }
            _ => panic!("f_8009a0c0: bad pc {:#x}", pc),
        }
    }
}

/// __OSSetExceptionHandler (0x8009a0c4, 28 bytes)
#[inline(never)]
pub fn f_8009a0c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a0c4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a0c4); f_8009a0c4_b(c, 0x8009a0c4); crate::prof::exit(c, 0x8009a0c4); return; }
    }
    f_8009a0c4_b(c, 0x8009a0c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a0c4_at(c: &mut Cpu, pc: u32) { f_8009a0c4_b(c, pc) }
#[inline(always)]
fn f_8009a0c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a0c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a0c4, pc); }
        match pc {
            0x8009a0c4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ea4));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[5] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lwz(c.r[5]);
                c.stw(c.r[5], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009a0c4: bad pc {:#x}", pc),
        }
    }
}

/// __OSGetExceptionHandler (0x8009a0e0, 20 bytes)
#[inline(never)]
pub fn f_8009a0e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a0e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a0e0); f_8009a0e0_b(c, 0x8009a0e0); crate::prof::exit(c, 0x8009a0e0); return; }
    }
    f_8009a0e0_b(c, 0x8009a0e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a0e0_at(c: &mut Cpu, pc: u32) { f_8009a0e0_b(c, pc) }
#[inline(always)]
fn f_8009a0e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a0e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a0e0, pc); }
        match pc {
            0x8009a0e0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ea4));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                if true { return; }
                return;
            }
            _ => panic!("f_8009a0e0: bad pc {:#x}", pc),
        }
    }
}

/// OSExceptionVector (0x8009a0f4, 156 bytes)
#[inline(never)]
pub fn f_8009a0f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a0f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a0f4); f_8009a0f4_b(c, 0x8009a0f4); crate::prof::exit(c, 0x8009a0f4); return; }
    }
    f_8009a0f4_b(c, 0x8009a0f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a0f4_at(c: &mut Cpu, pc: u32) { f_8009a0f4_b(c, pc) }
#[inline(always)]
fn f_8009a0f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a0f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a0f4, pc); }
        match pc {
            0x8009a0f4 => {
                c.mtspr(272, c.r[4]);
                c.r[4] = c.lwz(0xc0u32);
                c.stw(c.r[4].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.mfspr(272);
                c.stw(c.r[4].wrapping_add(0x10), c.r[3]);
                c.stw(c.r[4].wrapping_add(0x14), c.r[5]);
                c.r[3] = c.lhz(c.r[4].wrapping_add(0x1a2));
                c.r[3] = c.r[3] | 0x2;
                c.sth(c.r[4].wrapping_add(0x1a2), c.r[3]);
                c.r[3] = c.cr;
                c.stw(c.r[4].wrapping_add(0x80), c.r[3]);
                c.r[3] = c.mfspr(8);
                c.stw(c.r[4].wrapping_add(0x84), c.r[3]);
                c.r[3] = c.mfspr(9);
                c.stw(c.r[4].wrapping_add(0x88), c.r[3]);
                c.r[3] = c.mfspr(1);
                c.stw(c.r[4].wrapping_add(0x8c), c.r[3]);
                c.r[3] = c.mfspr(26);
                c.stw(c.r[4].wrapping_add(0x198), c.r[3]);
                c.r[3] = c.mfspr(27);
                c.stw(c.r[4].wrapping_add(0x19c), c.r[3]);
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = 0; // mfmsr
                c.r[3] = c.r[3] | 0x30;
                c.mtspr(27, c.r[3]);
                c.r[3] = 0x0;
                c.r[4] = c.lwz(0xd4u32);
                c.r[5] = c.r[5].rotate_left(0) & 0x2;
                c.rc(c.r[5]);
                if !c.crbit(2) { pc = 0x8009a17c; continue; }
                pc = 0x8009a16c;
            }
            0x8009a16c => {
                c.r[5] = 0x800a0000;
                c.r[5] = c.r[5].wrapping_add(0xffffa190);
                c.mtspr(26, c.r[5]);
                // rfi
                pc = 0x8009a17c;
            }
            0x8009a17c => {
                c.r[5] = c.r[3].rotate_left(2) & 0x3fc;
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x3000));
                c.mtspr(26, c.r[5]);
                // rfi
                f_8009a190(c); return;
                return;
            }
            _ => panic!("f_8009a0f4: bad pc {:#x}", pc),
        }
    }
}

/// OSDefaultExceptionHandler (0x8009a190, 88 bytes)
#[inline(never)]
pub fn f_8009a190(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a190) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a190); f_8009a190_b(c, 0x8009a190); crate::prof::exit(c, 0x8009a190); return; }
    }
    f_8009a190_b(c, 0x8009a190)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a190_at(c: &mut Cpu, pc: u32) { f_8009a190_b(c, pc) }
#[inline(always)]
fn f_8009a190_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a190;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a190, pc); }
        match pc {
            0x8009a190 => {
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[1]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[2]);
                { let mut ea = c.r[4].wrapping_add(0x18); for n in 6..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[0] = c.mfspr(913);
                c.stw(c.r[4].wrapping_add(0x1a8), c.r[0]);
                c.r[0] = c.mfspr(914);
                c.stw(c.r[4].wrapping_add(0x1ac), c.r[0]);
                c.r[0] = c.mfspr(915);
                c.stw(c.r[4].wrapping_add(0x1b0), c.r[0]);
                c.r[0] = c.mfspr(916);
                c.stw(c.r[4].wrapping_add(0x1b4), c.r[0]);
                c.r[0] = c.mfspr(917);
                c.stw(c.r[4].wrapping_add(0x1b8), c.r[0]);
                c.r[0] = c.mfspr(918);
                c.stw(c.r[4].wrapping_add(0x1bc), c.r[0]);
                c.r[0] = c.mfspr(919);
                c.stw(c.r[4].wrapping_add(0x1c0), c.r[0]);
                c.r[5] = c.mfspr(18);
                c.r[6] = c.mfspr(19);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                f_8009c578(c); return;
                return;
            }
            _ => panic!("f_8009a190: bad pc {:#x}", pc),
        }
    }
}

/// __OSPSInit (0x8009a1e8, 84 bytes)
#[inline(never)]
pub fn f_8009a1e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a1e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a1e8); f_8009a1e8_b(c, 0x8009a1e8); crate::prof::exit(c, 0x8009a1e8); return; }
    }
    f_8009a1e8_b(c, 0x8009a1e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a1e8_at(c: &mut Cpu, pc: u32) { f_8009a1e8_b(c, pc) }
#[inline(always)]
fn f_8009a1e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a1e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a1e8, pc); }
        match pc {
            0x8009a1e8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x8009a1f8; f_800980c8(c);
                pc = 0x8009a1f8;
            }
            0x8009a1f8 => {
                c.r[3] = c.r[3] | 0xa0000000;
                c.lr = 0x8009a200; f_800980d0(c);
                pc = 0x8009a200;
            }
            0x8009a200 => {
                c.lr = 0x8009a204; f_8009b3e8(c);
                pc = 0x8009a204;
            }
            0x8009a204 => {
                // sync
                c.r[3] = 0x0;
                c.mtspr(912, c.r[3]);
                c.mtspr(913, c.r[3]);
                c.mtspr(914, c.r[3]);
                c.mtspr(915, c.r[3]);
                c.mtspr(916, c.r[3]);
                c.mtspr(917, c.r[3]);
                c.mtspr(918, c.r[3]);
                c.mtspr(919, c.r[3]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009a1e8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009A23C (0x8009a23c, 20 bytes)
#[inline(never)]
pub fn f_8009a23c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a23c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a23c); f_8009a23c_b(c, 0x8009a23c); crate::prof::exit(c, 0x8009a23c); return; }
    }
    f_8009a23c_b(c, 0x8009a23c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a23c_at(c: &mut Cpu, pc: u32) { f_8009a23c_b(c, pc) }
#[inline(always)]
fn f_8009a23c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a23c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a23c, pc); }
        match pc {
            0x8009a23c => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6000);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x24));
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                if true { return; }
                return;
            }
            _ => panic!("f_8009a23c: bad pc {:#x}", pc),
        }
    }
}

/// OSRegisterVersion (0x8009a250, 44 bytes)
#[inline(never)]
pub fn f_8009a250(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a250) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a250); f_8009a250_b(c, 0x8009a250); crate::prof::exit(c, 0x8009a250); return; }
    }
    f_8009a250_b(c, 0x8009a250)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a250_at(c: &mut Cpu, pc: u32) { f_8009a250_b(c, pc) }
#[inline(always)]
fn f_8009a250_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a250;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a250, pc); }
        match pc {
            0x8009a250 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[4] = c.r[3] | c.r[3];
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[3] = c.r[13].wrapping_add(0xffff815c);
                c.lr = 0x8009a26c; f_8009c2e0(c);
                pc = 0x8009a26c;
            }
            0x8009a26c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009a250: bad pc {:#x}", pc),
        }
    }
}

/// OSInitAlarm (0x8009a27c, 76 bytes)
#[inline(never)]
pub fn f_8009a27c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a27c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a27c); f_8009a27c_b(c, 0x8009a27c); crate::prof::exit(c, 0x8009a27c); return; }
    }
    f_8009a27c_b(c, 0x8009a27c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a27c_at(c: &mut Cpu, pc: u32) { f_8009a27c_b(c, pc) }
#[inline(always)]
fn f_8009a27c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a27c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a27c, pc); }
        match pc {
            0x8009a27c => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x8;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x8009a290; f_8009a0e0(c);
                pc = 0x8009a290;
            }
            0x8009a290 => {
                c.r[4] = 0x800a0000;
                c.r[4] = c.r[4].wrapping_add(0xffffa8dc);
                c.cmpu(0, c.r[3], c.r[4]);
                if c.crbit(2) { pc = 0x8009a2b8; continue; }
                pc = 0x8009a2a0;
            }
            0x8009a2a0 => {
                c.r[0] = 0x0;
                c.r[3] = c.r[13].wrapping_add(0xffff9ec0);
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                c.r[3] = 0x8;
                c.stw(c.r[13].wrapping_add(0xffff9ec0), c.r[0]);
                c.lr = 0x8009a2b8; f_8009a0c4(c);
                pc = 0x8009a2b8;
            }
            0x8009a2b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009a27c: bad pc {:#x}", pc),
        }
    }
}

/// OSCreateAlarm (0x8009a2c8, 16 bytes)
#[inline(never)]
pub fn f_8009a2c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a2c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a2c8); f_8009a2c8_b(c, 0x8009a2c8); crate::prof::exit(c, 0x8009a2c8); return; }
    }
    f_8009a2c8_b(c, 0x8009a2c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a2c8_at(c: &mut Cpu, pc: u32) { f_8009a2c8_b(c, pc) }
#[inline(always)]
fn f_8009a2c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a2c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a2c8, pc); }
        match pc {
            0x8009a2c8 => {
                c.r[0] = 0x0;
                c.stw(c.r[3], c.r[0]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009a2c8: bad pc {:#x}", pc),
        }
    }
}

/// InsertAlarm (0x8009a2d8, 592 bytes)
#[inline(never)]
pub fn f_8009a2d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a2d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a2d8); f_8009a2d8_b(c, 0x8009a2d8); crate::prof::exit(c, 0x8009a2d8); return; }
    }
    f_8009a2d8_b(c, 0x8009a2d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a2d8_at(c: &mut Cpu, pc: u32) { f_8009a2d8_b(c, pc) }
#[inline(always)]
fn f_8009a2d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a2d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a2d8, pc); }
        match pc {
            0x8009a2d8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = 0x0;
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = c.r[28] ^ 0x80000000;
                c.r[25] = c.r[5].wrapping_add(0x0);
                c.r[30] = c.r[6].wrapping_add(0x0);
                c.r[31] = c.r[7].wrapping_add(0x0);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x18));
                c.r[8] = c.lwz(c.r[3].wrapping_add(0x1c));
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[8], c.r[28], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[4], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[4], c.r[4], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009a398; continue; }
                pc = 0x8009a320;
            }
            0x8009a320 => {
                c.lr = 0x8009a324; f_800a2798(c);
                pc = 0x8009a324;
            }
            0x8009a324 => {
                c.r[7] = c.lwz(c.r[29].wrapping_add(0x20));
                c.r[5] = c.r[3] ^ 0x80000000;
                c.r[8] = c.lwz(c.r[29].wrapping_add(0x24));
                c.r[6] = c.r[7] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[8], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[6], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                c.r[30] = c.r[8].wrapping_add(0x0);
                c.r[25] = c.r[7].wrapping_add(0x0);
                if c.crbit(2) { pc = 0x8009a398; continue; }
                pc = 0x8009a350;
            }
            0x8009a350 => {
                c.r[27] = c.lwz(c.r[29].wrapping_add(0x18));
                c.r[4] = c.addc(!c.r[8], c.r[4], 1);
                c.r[26] = c.lwz(c.r[29].wrapping_add(0x1c));
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[7], c.r[3], ci); }
                c.r[5] = c.r[27].wrapping_add(0x0);
                c.r[6] = c.r[26].wrapping_add(0x0);
                c.lr = 0x8009a36c; f_800c4928(c);
                pc = 0x8009a36c;
            }
            0x8009a36c => {
                c.r[0] = 0x1;
                c.r[5] = c.addc(c.r[4], c.r[0], 0);
                c.r[4] = c.r[27].wrapping_mul(c.r[5]);
                c.r[0] = mulhwu(c.r[26], c.r[5]);
                { let ci = c.ca(); c.r[3] = c.addc(c.r[3], c.r[28], ci); }
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.r[26].wrapping_mul(c.r[3]);
                c.r[3] = c.r[26].wrapping_mul(c.r[5]);
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.r[30] = c.addc(c.r[30], c.r[3], 0);
                { let ci = c.ca(); c.r[25] = c.addc(c.r[25], c.r[0], ci); }
                pc = 0x8009a398;
            }
            0x8009a398 => {
                c.stw(c.r[29], c.r[31]);
                c.r[4] = c.r[25] ^ 0x80000000;
                c.stw(c.r[29].wrapping_add(0xc), c.r[30]);
                c.stw(c.r[29].wrapping_add(0x8), c.r[25]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff9ec0));
                pc = 0x8009a470; continue;
                pc = 0x8009a3b0;
            }
            0x8009a3b0 => {
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x8));
                c.r[5] = c.lwz(c.r[6].wrapping_add(0xc));
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[5], c.r[30], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[4], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[4], c.r[4], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009a46c; continue; }
                pc = 0x8009a3d0;
            }
            0x8009a3d0 => {
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x10));
                c.stw(c.r[29].wrapping_add(0x10), c.r[0]);
                c.stw(c.r[6].wrapping_add(0x10), c.r[29]);
                c.stw(c.r[29].wrapping_add(0x14), c.r[6]);
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009a3f4; continue; }
                pc = 0x8009a3ec;
            }
            0x8009a3ec => {
                c.stw(c.r[3].wrapping_add(0x14), c.r[29]);
                pc = 0x8009a514; continue;
                pc = 0x8009a3f4;
            }
            0x8009a3f4 => {
                c.stw(c.r[13].wrapping_add(0xffff9ec0), c.r[29]);
                c.lr = 0x8009a3fc; f_800a2798(c);
                pc = 0x8009a3fc;
            }
            0x8009a3fc => {
                c.r[6] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[7] = 0x0;
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                c.r[5] = c.r[7] ^ 0x80000000;
                c.r[8] = c.addc(!c.r[4], c.r[6], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[3], c.r[0], ci); }
                c.r[6] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[7], c.r[8], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[6], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x8009a438; continue; }
                pc = 0x8009a42c;
            }
            0x8009a42c => {
                c.r[3] = 0x0;
                c.lr = 0x8009a434; f_8009802c(c);
                pc = 0x8009a434;
            }
            0x8009a434 => {
                pc = 0x8009a514; continue;
                pc = 0x8009a438;
            }
            0x8009a438 => {
                c.r[4] = 0x80000000;
                c.r[3] = c.r[7] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[8], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[6], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009a460; continue; }
                pc = 0x8009a454;
            }
            0x8009a454 => {
                c.r[3] = c.r[8] | c.r[8];
                c.lr = 0x8009a45c; f_8009802c(c);
                pc = 0x8009a45c;
            }
            0x8009a45c => {
                pc = 0x8009a514; continue;
                pc = 0x8009a460;
            }
            0x8009a460 => {
                c.r[3] = c.r[4].wrapping_add(0xffffffff);
                c.lr = 0x8009a468; f_8009802c(c);
                pc = 0x8009a468;
            }
            0x8009a468 => {
                pc = 0x8009a514; continue;
                pc = 0x8009a46c;
            }
            0x8009a46c => {
                c.r[6] = c.lwz(c.r[6].wrapping_add(0x14));
                pc = 0x8009a470;
            }
            0x8009a470 => {
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x8009a3b0; continue; }
                pc = 0x8009a478;
            }
            0x8009a478 => {
                c.r[30] = 0x0;
                c.stw(c.r[29].wrapping_add(0x14), c.r[30]);
                c.r[3] = c.r[13].wrapping_add(0xffff9ec0);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x4));
                { let ea = c.r[3].wrapping_add(0x4); c.stw(ea, c.r[29]); c.r[3] = ea; }
                c.cmpu(0, c.r[4], 0x0);
                c.stw(c.r[29].wrapping_add(0x10), c.r[4]);
                if c.crbit(2) { pc = 0x8009a4a0; continue; }
                pc = 0x8009a498;
            }
            0x8009a498 => {
                c.stw(c.r[4].wrapping_add(0x14), c.r[29]);
                pc = 0x8009a514; continue;
                pc = 0x8009a4a0;
            }
            0x8009a4a0 => {
                c.stw(c.r[3], c.r[29]);
                c.stw(c.r[13].wrapping_add(0xffff9ec0), c.r[29]);
                c.lr = 0x8009a4ac; f_800a2798(c);
                pc = 0x8009a4ac;
            }
            0x8009a4ac => {
                c.r[6] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[5] = c.r[30] ^ 0x80000000;
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                c.r[7] = c.addc(!c.r[4], c.r[6], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[3], c.r[0], ci); }
                c.r[6] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[30], c.r[7], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[6], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x8009a4e4; continue; }
                pc = 0x8009a4d8;
            }
            0x8009a4d8 => {
                c.r[3] = 0x0;
                c.lr = 0x8009a4e0; f_8009802c(c);
                pc = 0x8009a4e0;
            }
            0x8009a4e0 => {
                pc = 0x8009a514; continue;
                pc = 0x8009a4e4;
            }
            0x8009a4e4 => {
                c.r[4] = 0x80000000;
                c.r[3] = c.r[30] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[7], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[6], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009a50c; continue; }
                pc = 0x8009a500;
            }
            0x8009a500 => {
                c.r[3] = c.r[7] | c.r[7];
                c.lr = 0x8009a508; f_8009802c(c);
                pc = 0x8009a508;
            }
            0x8009a508 => {
                pc = 0x8009a514; continue;
                pc = 0x8009a50c;
            }
            0x8009a50c => {
                c.r[3] = c.r[4].wrapping_add(0xffffffff);
                c.lr = 0x8009a514; f_8009802c(c);
                pc = 0x8009a514;
            }
            0x8009a514 => {
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009a2d8: bad pc {:#x}", pc),
        }
    }
}

/// OSSetAlarm (0x8009a528, 104 bytes)
#[inline(never)]
pub fn f_8009a528(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a528) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a528); f_8009a528_b(c, 0x8009a528); crate::prof::exit(c, 0x8009a528); return; }
    }
    f_8009a528_b(c, 0x8009a528)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a528_at(c: &mut Cpu, pc: u32) { f_8009a528_b(c, pc) }
#[inline(always)]
fn f_8009a528_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a528;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a528, pc); }
        match pc {
            0x8009a528 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3].wrapping_add(0x0);
                c.r[29] = c.r[5].wrapping_add(0x0);
                c.r[28] = c.r[6].wrapping_add(0x0);
                c.r[30] = c.r[7].wrapping_add(0x0);
                c.lr = 0x8009a54c; f_8009df3c(c);
                pc = 0x8009a54c;
            }
            0x8009a54c => {
                c.r[0] = 0x0;
                c.stw(c.r[27].wrapping_add(0x1c), c.r[0]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[27].wrapping_add(0x18), c.r[0]);
                c.lr = 0x8009a560; f_800a2798(c);
                pc = 0x8009a560;
            }
            0x8009a560 => {
                c.r[6] = c.addc(c.r[28], c.r[4], 0);
                { let ci = c.ca(); c.r[5] = c.addc(c.r[29], c.r[3], ci); }
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[7] = c.r[30].wrapping_add(0x0);
                c.lr = 0x8009a574; f_8009a2d8(c);
                pc = 0x8009a574;
            }
            0x8009a574 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8009a57c; f_8009df64(c);
                pc = 0x8009a57c;
            }
            0x8009a57c => {
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009a528: bad pc {:#x}", pc),
        }
    }
}

/// OSCancelAlarm (0x8009a590, 284 bytes)
#[inline(never)]
pub fn f_8009a590(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a590) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a590); f_8009a590_b(c, 0x8009a590); crate::prof::exit(c, 0x8009a590); return; }
    }
    f_8009a590_b(c, 0x8009a590)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a590_at(c: &mut Cpu, pc: u32) { f_8009a590_b(c, pc) }
#[inline(always)]
fn f_8009a590_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a590;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a590, pc); }
        match pc {
            0x8009a590 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.lr = 0x8009a5b0; f_8009df3c(c);
                pc = 0x8009a5b0;
            }
            0x8009a5b0 => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009a5cc; continue; }
                pc = 0x8009a5c0;
            }
            0x8009a5c0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8009a5c8; f_8009df64(c);
                pc = 0x8009a5c8;
            }
            0x8009a5c8 => {
                pc = 0x8009a690; continue;
                pc = 0x8009a5cc;
            }
            0x8009a5cc => {
                c.r[29] = c.lwz(c.r[30].wrapping_add(0x14));
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x8009a5e8; continue; }
                pc = 0x8009a5d8;
            }
            0x8009a5d8 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[3] = c.r[13].wrapping_add(0xffff9ec0);
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x8009a5f0; continue;
                pc = 0x8009a5e8;
            }
            0x8009a5e8 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10));
                c.stw(c.r[29].wrapping_add(0x10), c.r[0]);
                pc = 0x8009a5f0;
            }
            0x8009a5f0 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009a604; continue; }
                pc = 0x8009a5fc;
            }
            0x8009a5fc => {
                c.stw(c.r[3].wrapping_add(0x14), c.r[29]);
                pc = 0x8009a680; continue;
                pc = 0x8009a604;
            }
            0x8009a604 => {
                c.cmpu(0, c.r[29], 0x0);
                c.stw(c.r[13].wrapping_add(0xffff9ec0), c.r[29]);
                if c.crbit(2) { pc = 0x8009a680; continue; }
                pc = 0x8009a610;
            }
            0x8009a610 => {
                c.lr = 0x8009a614; f_800a2798(c);
                pc = 0x8009a614;
            }
            0x8009a614 => {
                c.r[6] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[7] = 0x0;
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                c.r[5] = c.r[7] ^ 0x80000000;
                c.r[8] = c.addc(!c.r[4], c.r[6], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[3], c.r[0], ci); }
                c.r[6] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[7], c.r[8], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[6], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x8009a650; continue; }
                pc = 0x8009a644;
            }
            0x8009a644 => {
                c.r[3] = 0x0;
                c.lr = 0x8009a64c; f_8009802c(c);
                pc = 0x8009a64c;
            }
            0x8009a64c => {
                pc = 0x8009a680; continue;
                pc = 0x8009a650;
            }
            0x8009a650 => {
                c.r[4] = 0x80000000;
                c.r[3] = c.r[7] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[8], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[6], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009a678; continue; }
                pc = 0x8009a66c;
            }
            0x8009a66c => {
                c.r[3] = c.r[8] | c.r[8];
                c.lr = 0x8009a674; f_8009802c(c);
                pc = 0x8009a674;
            }
            0x8009a674 => {
                pc = 0x8009a680; continue;
                pc = 0x8009a678;
            }
            0x8009a678 => {
                c.r[3] = c.r[4].wrapping_add(0xffffffff);
                c.lr = 0x8009a680; f_8009802c(c);
                pc = 0x8009a680;
            }
            0x8009a680 => {
                c.r[0] = 0x0;
                c.stw(c.r[30], c.r[0]);
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8009a690; f_8009df64(c);
                pc = 0x8009a690;
            }
            0x8009a690 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009a590: bad pc {:#x}", pc),
        }
    }
}

/// DecrementerExceptionCallback (0x8009a6ac, 560 bytes)
#[inline(never)]
pub fn f_8009a6ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a6ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a6ac); f_8009a6ac_b(c, 0x8009a6ac); crate::prof::exit(c, 0x8009a6ac); return; }
    }
    f_8009a6ac_b(c, 0x8009a6ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a6ac_at(c: &mut Cpu, pc: u32) { f_8009a6ac_b(c, pc) }
#[inline(always)]
fn f_8009a6ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a6ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a6ac, pc); }
        match pc {
            0x8009a6ac => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffd10); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2ec), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x2e8), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x2e4), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x2e0), c.r[28]);
                c.lr = 0x8009a6d0; f_800a2798(c);
                pc = 0x8009a6d0;
            }
            0x8009a6d0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9ec0));
                c.r[28] = c.r[4].wrapping_add(0x0);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[0], 0x0);
                c.r[31] = c.r[0] | c.r[0];
                if !c.crbit(2) { pc = 0x8009a6f0; continue; }
                pc = 0x8009a6e8;
            }
            0x8009a6e8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009a6f0; f_8009bc50(c);
                pc = 0x8009a6f0;
            }
            0x8009a6f0 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[4] = c.r[30] ^ 0x80000000;
                c.r[5] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[5], c.r[28], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[4], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[4], c.r[4], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009a78c; continue; }
                pc = 0x8009a714;
            }
            0x8009a714 => {
                c.lr = 0x8009a718; f_800a2798(c);
                pc = 0x8009a718;
            }
            0x8009a718 => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[7] = 0x0;
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[5] = c.r[7] ^ 0x80000000;
                c.r[8] = c.addc(!c.r[4], c.r[6], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[3], c.r[0], ci); }
                c.r[6] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[7], c.r[8], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[6], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x8009a754; continue; }
                pc = 0x8009a748;
            }
            0x8009a748 => {
                c.r[3] = 0x0;
                c.lr = 0x8009a750; f_8009802c(c);
                pc = 0x8009a750;
            }
            0x8009a750 => {
                pc = 0x8009a784; continue;
                pc = 0x8009a754;
            }
            0x8009a754 => {
                c.r[4] = 0x80000000;
                c.r[3] = c.r[7] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[8], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[6], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009a77c; continue; }
                pc = 0x8009a770;
            }
            0x8009a770 => {
                c.r[3] = c.r[8] | c.r[8];
                c.lr = 0x8009a778; f_8009802c(c);
                pc = 0x8009a778;
            }
            0x8009a778 => {
                pc = 0x8009a784; continue;
                pc = 0x8009a77c;
            }
            0x8009a77c => {
                c.r[3] = c.r[4].wrapping_add(0xffffffff);
                c.lr = 0x8009a784; f_8009802c(c);
                pc = 0x8009a784;
            }
            0x8009a784 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009a78c; f_8009bc50(c);
                pc = 0x8009a78c;
            }
            0x8009a78c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x14));
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffff9ec0), c.r[3]);
                if !c.crbit(2) { pc = 0x8009a7ac; continue; }
                pc = 0x8009a79c;
            }
            0x8009a79c => {
                c.r[0] = 0x0;
                c.r[3] = c.r[13].wrapping_add(0xffff9ec0);
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x8009a7b4; continue;
                pc = 0x8009a7ac;
            }
            0x8009a7ac => {
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x10), c.r[0]);
                pc = 0x8009a7b4;
            }
            0x8009a7b4 => {
                c.r[30] = c.lwz(c.r[31]);
                c.r[6] = 0x0;
                c.r[4] = c.r[6] ^ 0x80000000;
                c.stw(c.r[31], c.r[6]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x18));
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x1c));
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[5], c.r[6], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[4], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[4], c.r[4], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009a7f8; continue; }
                pc = 0x8009a7e4;
            }
            0x8009a7e4 => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[7] = c.r[30].wrapping_add(0x0);
                c.r[6] = 0x0;
                c.r[5] = 0x0;
                c.lr = 0x8009a7f8; f_8009a2d8(c);
                pc = 0x8009a7f8;
            }
            0x8009a7f8 => {
                c.r[28] = c.lwz(c.r[13].wrapping_add(0xffff9ec0));
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x8009a874; continue; }
                pc = 0x8009a804;
            }
            0x8009a804 => {
                c.lr = 0x8009a808; f_800a2798(c);
                pc = 0x8009a808;
            }
            0x8009a808 => {
                c.r[6] = c.lwz(c.r[28].wrapping_add(0xc));
                c.r[7] = 0x0;
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x8));
                c.r[5] = c.r[7] ^ 0x80000000;
                c.r[8] = c.addc(!c.r[4], c.r[6], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[3], c.r[0], ci); }
                c.r[6] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[7], c.r[8], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[6], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x8009a844; continue; }
                pc = 0x8009a838;
            }
            0x8009a838 => {
                c.r[3] = 0x0;
                c.lr = 0x8009a840; f_8009802c(c);
                pc = 0x8009a840;
            }
            0x8009a840 => {
                pc = 0x8009a874; continue;
                pc = 0x8009a844;
            }
            0x8009a844 => {
                c.r[4] = 0x80000000;
                c.r[3] = c.r[7] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[8], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[6], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009a86c; continue; }
                pc = 0x8009a860;
            }
            0x8009a860 => {
                c.r[3] = c.r[8] | c.r[8];
                c.lr = 0x8009a868; f_8009802c(c);
                pc = 0x8009a868;
            }
            0x8009a868 => {
                pc = 0x8009a874; continue;
                pc = 0x8009a86c;
            }
            0x8009a86c => {
                c.r[3] = c.r[4].wrapping_add(0xffffffff);
                c.lr = 0x8009a874; f_8009802c(c);
                pc = 0x8009a874;
            }
            0x8009a874 => {
                c.lr = 0x8009a878; f_800a1404(c);
                pc = 0x8009a878;
            }
            0x8009a878 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x8009a880; f_8009bd60(c);
                pc = 0x8009a880;
            }
            0x8009a880 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x8009a888; f_8009bb68(c);
                pc = 0x8009a888;
            }
            0x8009a888 => {
                c.r[12] = c.r[30].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[29].wrapping_add(0x0);
                if true { let t = c.lr; c.lr = 0x8009a89c; (c.dispatch)(c, t); }
                pc = 0x8009a89c;
            }
            0x8009a89c => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.lr = 0x8009a8a4; f_8009bd60(c);
                pc = 0x8009a8a4;
            }
            0x8009a8a4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009a8ac; f_8009bb68(c);
                pc = 0x8009a8ac;
            }
            0x8009a8ac => {
                c.lr = 0x8009a8b0; f_800a1444(c);
                pc = 0x8009a8b0;
            }
            0x8009a8b0 => {
                c.lr = 0x8009a8b4; f_800a1960(c);
                pc = 0x8009a8b4;
            }
            0x8009a8b4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009a8bc; f_8009bc50(c);
                pc = 0x8009a8bc;
            }
            0x8009a8bc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2f4));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2ec));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x2e8));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x2e4));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x2e0));
                c.r[1] = c.r[1].wrapping_add(0x2f0);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009a6ac: bad pc {:#x}", pc),
        }
    }
}

/// DecrementerExceptionHandler (0x8009a8dc, 80 bytes)
#[inline(never)]
pub fn f_8009a8dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a8dc) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a8dc); f_8009a8dc_b(c, 0x8009a8dc); crate::prof::exit(c, 0x8009a8dc); return; }
    }
    f_8009a8dc_b(c, 0x8009a8dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a8dc_at(c: &mut Cpu, pc: u32) { f_8009a8dc_b(c, pc) }
#[inline(always)]
fn f_8009a8dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a8dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a8dc, pc); }
        match pc {
            0x8009a8dc => {
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[1]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[2]);
                { let mut ea = c.r[4].wrapping_add(0x18); for n in 6..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[0] = c.mfspr(913);
                c.stw(c.r[4].wrapping_add(0x1a8), c.r[0]);
                c.r[0] = c.mfspr(914);
                c.stw(c.r[4].wrapping_add(0x1ac), c.r[0]);
                c.r[0] = c.mfspr(915);
                c.stw(c.r[4].wrapping_add(0x1b0), c.r[0]);
                c.r[0] = c.mfspr(916);
                c.stw(c.r[4].wrapping_add(0x1b4), c.r[0]);
                c.r[0] = c.mfspr(917);
                c.stw(c.r[4].wrapping_add(0x1b8), c.r[0]);
                c.r[0] = c.mfspr(918);
                c.stw(c.r[4].wrapping_add(0x1bc), c.r[0]);
                c.r[0] = c.mfspr(919);
                c.stw(c.r[4].wrapping_add(0x1c0), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                f_8009a6ac(c); return;
                return;
            }
            _ => panic!("f_8009a8dc: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009A92C (0x8009a92c, 172 bytes)
#[inline(never)]
pub fn f_8009a92c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a92c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a92c); f_8009a92c_b(c, 0x8009a92c); crate::prof::exit(c, 0x8009a92c); return; }
    }
    f_8009a92c_b(c, 0x8009a92c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a92c_at(c: &mut Cpu, pc: u32) { f_8009a92c_b(c, pc) }
#[inline(always)]
fn f_8009a92c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a92c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a92c, pc); }
        match pc {
            0x8009a92c => {
                c.r[7] = c.r[3].wrapping_add(0x0);
                c.r[6] = 0x0;
                pc = 0x8009a948; continue;
                pc = 0x8009a938;
            }
            0x8009a938 => {
                c.cmpu(0, c.r[4], c.r[7]);
                if !c.crbit(1) { pc = 0x8009a950; continue; }
                pc = 0x8009a940;
            }
            0x8009a940 => {
                c.r[6] = c.r[7] | c.r[7];
                c.r[7] = c.lwz(c.r[7].wrapping_add(0x4));
                pc = 0x8009a948;
            }
            0x8009a948 => {
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(2) { pc = 0x8009a938; continue; }
                pc = 0x8009a950;
            }
            0x8009a950 => {
                c.stw(c.r[4].wrapping_add(0x4), c.r[7]);
                c.cmpu(0, c.r[7], 0x0);
                c.stw(c.r[4], c.r[6]);
                if c.crbit(2) { pc = 0x8009a994; continue; }
                pc = 0x8009a960;
            }
            0x8009a960 => {
                c.stw(c.r[7], c.r[4]);
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[4].wrapping_add(c.r[5]);
                c.cmpu(0, c.r[0], c.r[7]);
                if !c.crbit(2) { pc = 0x8009a994; continue; }
                pc = 0x8009a974;
            }
            0x8009a974 => {
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x8));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[0]);
                c.r[7] = c.lwz(c.r[7].wrapping_add(0x4));
                c.cmpu(0, c.r[7], 0x0);
                c.stw(c.r[4].wrapping_add(0x4), c.r[7]);
                if c.crbit(2) { pc = 0x8009a994; continue; }
                pc = 0x8009a990;
            }
            0x8009a990 => {
                c.stw(c.r[7], c.r[4]);
                pc = 0x8009a994;
            }
            0x8009a994 => {
                c.cmpu(0, c.r[6], 0x0);
                if c.crbit(2) { pc = 0x8009a9d0; continue; }
                pc = 0x8009a99c;
            }
            0x8009a99c => {
                c.stw(c.r[6].wrapping_add(0x4), c.r[4]);
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x8));
                c.r[0] = c.r[6].wrapping_add(c.r[5]);
                c.cmpu(0, c.r[0], c.r[4]);
                if !c.crbit(2) { return; }
                pc = 0x8009a9b0;
            }
            0x8009a9b0 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.cmpu(0, c.r[7], 0x0);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.stw(c.r[6].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[6].wrapping_add(0x4), c.r[7]);
                if c.crbit(2) { return; }
                pc = 0x8009a9c8;
            }
            0x8009a9c8 => {
                c.stw(c.r[7], c.r[6]);
                if true { return; }
                pc = 0x8009a9d0;
            }
            0x8009a9d0 => {
                c.r[3] = c.r[4] | c.r[4];
                if true { return; }
                return;
            }
            _ => panic!("f_8009a92c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009A9D8 (0x8009a9d8, 252 bytes)
#[inline(never)]
pub fn f_8009a9d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009a9d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009a9d8); f_8009a9d8_b(c, 0x8009a9d8); crate::prof::exit(c, 0x8009a9d8); return; }
    }
    f_8009a9d8_b(c, 0x8009a9d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009a9d8_at(c: &mut Cpu, pc: u32) { f_8009a9d8_b(c, pc) }
#[inline(always)]
fn f_8009a9d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009a9d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009a9d8, pc); }
        match pc {
            0x8009a9d8 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ec8));
                c.r[5] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.r[4].wrapping_add(0x3f);
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x8009aa04; continue;
                pc = 0x8009a9f4;
            }
            0x8009a9f4 => {
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x8));
                c.cmps(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x8009aa0c; continue; }
                pc = 0x8009aa00;
            }
            0x8009aa00 => {
                c.r[6] = c.lwz(c.r[6].wrapping_add(0x4));
                pc = 0x8009aa04;
            }
            0x8009aa04 => {
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x8009a9f4; continue; }
                pc = 0x8009aa0c;
            }
            0x8009aa0c => {
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x8009aa1c; continue; }
                pc = 0x8009aa14;
            }
            0x8009aa14 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8009aa1c;
            }
            0x8009aa1c => {
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x8));
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.cmpu(0, c.r[0], 0x40);
                if !c.crbit(0) { pc = 0x8009aa68; continue; }
                pc = 0x8009aa2c;
            }
            0x8009aa2c => {
                c.r[4] = c.lwz(c.r[6].wrapping_add(0x4));
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x8009aa44; continue; }
                pc = 0x8009aa3c;
            }
            0x8009aa3c => {
                c.r[0] = c.lwz(c.r[6]);
                c.stw(c.r[4], c.r[0]);
                pc = 0x8009aa44;
            }
            0x8009aa44 => {
                c.r[4] = c.lwz(c.r[6]);
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x8009aa58; continue; }
                pc = 0x8009aa50;
            }
            0x8009aa50 => {
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x4));
                pc = 0x8009aa60; continue;
                pc = 0x8009aa58;
            }
            0x8009aa58 => {
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x4));
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                pc = 0x8009aa60;
            }
            0x8009aa60 => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                pc = 0x8009aaac; continue;
                pc = 0x8009aa68;
            }
            0x8009aa68 => {
                c.stw(c.r[6].wrapping_add(0x8), c.r[3]);
                c.r[4] = c.r[6].wrapping_add(c.r[3]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.lwz(c.r[6]);
                c.stw(c.r[4], c.r[0]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x4));
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009aa94; continue; }
                pc = 0x8009aa90;
            }
            0x8009aa90 => {
                c.stw(c.r[3], c.r[4]);
                pc = 0x8009aa94;
            }
            0x8009aa94 => {
                c.r[3] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009aaa8; continue; }
                pc = 0x8009aaa0;
            }
            0x8009aaa0 => {
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                pc = 0x8009aaac; continue;
                pc = 0x8009aaa8;
            }
            0x8009aaa8 => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[4]);
                pc = 0x8009aaac;
            }
            0x8009aaac => {
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x8));
                c.r[0] = 0x0;
                c.stw(c.r[6].wrapping_add(0x4), c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[6], c.r[0]);
                if c.crbit(2) { pc = 0x8009aac8; continue; }
                pc = 0x8009aac4;
            }
            0x8009aac4 => {
                c.stw(c.r[3], c.r[6]);
                pc = 0x8009aac8;
            }
            0x8009aac8 => {
                c.stw(c.r[5].wrapping_add(0x8), c.r[6]);
                c.r[3] = c.r[6].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8009a9d8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009AAD4 (0x8009aad4, 124 bytes)
#[inline(never)]
pub fn f_8009aad4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009aad4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009aad4); f_8009aad4_b(c, 0x8009aad4); crate::prof::exit(c, 0x8009aad4); return; }
    }
    f_8009aad4_b(c, 0x8009aad4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009aad4_at(c: &mut Cpu, pc: u32) { f_8009aad4_b(c, pc) }
#[inline(always)]
fn f_8009aad4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009aad4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009aad4, pc); }
        match pc {
            0x8009aad4 => {
                c.r[0] = c.mfspr(8);
                c.r[6] = c.r[4].wrapping_add(0xffffffe0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9ec8));
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x4));
                c.r[31] = c.r[4].wrapping_add(c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[4] = c.r[6] | c.r[6];
                if c.crbit(2) { pc = 0x8009ab10; continue; }
                pc = 0x8009ab08;
            }
            0x8009ab08 => {
                c.r[0] = c.lwz(c.r[4]);
                c.stw(c.r[3], c.r[0]);
                pc = 0x8009ab10;
            }
            0x8009ab10 => {
                c.r[3] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009ab24; continue; }
                pc = 0x8009ab1c;
            }
            0x8009ab1c => {
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x4));
                pc = 0x8009ab2c; continue;
                pc = 0x8009ab24;
            }
            0x8009ab24 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                pc = 0x8009ab2c;
            }
            0x8009ab2c => {
                c.stw(c.r[31].wrapping_add(0x8), c.r[5]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x4));
                c.lr = 0x8009ab38; f_8009a92c(c);
                pc = 0x8009ab38;
            }
            0x8009ab38 => {
                c.stw(c.r[31].wrapping_add(0x4), c.r[3]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009aad4: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009AB50 (0x8009ab50, 16 bytes)
#[inline(never)]
pub fn f_8009ab50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ab50) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ab50); f_8009ab50_b(c, 0x8009ab50); crate::prof::exit(c, 0x8009ab50); return; }
    }
    f_8009ab50_b(c, 0x8009ab50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ab50_at(c: &mut Cpu, pc: u32) { f_8009ab50_b(c, pc) }
#[inline(always)]
fn f_8009ab50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ab50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ab50, pc); }
        match pc {
            0x8009ab50 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8160));
                c.stw(c.r[13].wrapping_add(0xffff8160), c.r[3]);
                c.r[3] = c.r[0] | c.r[0];
                if true { return; }
                return;
            }
            _ => panic!("f_8009ab50: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009AB60 (0x8009ab60, 112 bytes)
#[inline(never)]
pub fn f_8009ab60(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ab60) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ab60); f_8009ab60_b(c, 0x8009ab60); crate::prof::exit(c, 0x8009ab60); return; }
    }
    f_8009ab60_b(c, 0x8009ab60)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ab60_at(c: &mut Cpu, pc: u32) { f_8009ab60_b(c, pc) }
#[inline(always)]
fn f_8009ab60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ab60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ab60, pc); }
        match pc {
            0x8009ab60 => {
                c.r[7] = c.r[5].wrapping_mul(0xc);
                c.stw(c.r[13].wrapping_add(0xffff9ec8), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffff9ecc), c.r[5]);
                c.r[6] = 0x0;
                c.r[3] = c.r[6].wrapping_add(0x0);
                c.r[8] = 0x0;
                c.r[5] = 0xffffffff;
                pc = 0x8009ab9c; continue;
                pc = 0x8009ab80;
            }
            0x8009ab80 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9ec8));
                c.r[8] = c.r[8].wrapping_add(0x1);
                c.r[9] = c.r[0].wrapping_add(c.r[6]);
                c.stw(c.r[9], c.r[5]);
                c.r[6] = c.r[6].wrapping_add(0xc);
                c.stw(c.r[9].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[9].wrapping_add(0x4), c.r[3]);
                pc = 0x8009ab9c;
            }
            0x8009ab9c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9ecc));
                c.cmps(0, c.r[8], c.r[0]);
                if c.crbit(0) { pc = 0x8009ab80; continue; }
                pc = 0x8009aba8;
            }
            0x8009aba8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ec8));
                c.r[0] = c.r[4].rotate_left(0) & 0xffffffe0;
                c.r[4] = 0xffffffff;
                c.stw(c.r[13].wrapping_add(0xffff9ed4), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(c.r[7]);
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.stw(c.r[13].wrapping_add(0xffff8160), c.r[4]);
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.stw(c.r[13].wrapping_add(0xffff9ed0), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009ab60: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009ABD0 (0x8009abd0, 108 bytes)
#[inline(never)]
pub fn f_8009abd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009abd0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009abd0); f_8009abd0_b(c, 0x8009abd0); crate::prof::exit(c, 0x8009abd0); return; }
    }
    f_8009abd0_b(c, 0x8009abd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009abd0_at(c: &mut Cpu, pc: u32) { f_8009abd0_b(c, pc) }
#[inline(always)]
fn f_8009abd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009abd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009abd0, pc); }
        match pc {
            0x8009abd0 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff9ecc));
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9ec8));
                c.r[7] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.cmps(0, c.r[6], 0x0);
                c.mtspr(9, c.r[6]);
                c.r[4] = c.r[4].rotate_left(0) & 0xffffffe0;
                c.r[3] = 0x0;
                if !c.crbit(1) { pc = 0x8009ac34; continue; }
                pc = 0x8009abf4;
            }
            0x8009abf4 => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x8009ac28; continue; }
                pc = 0x8009ac00;
            }
            0x8009ac00 => {
                c.r[0] = c.r[4].wrapping_sub(c.r[7]);
                c.stw(c.r[5], c.r[0]);
                c.r[4] = 0x0;
                c.stw(c.r[7], c.r[4]);
                c.stw(c.r[7].wrapping_add(0x4), c.r[4]);
                c.r[0] = c.lwz(c.r[5]);
                c.stw(c.r[7].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x4), c.r[7]);
                c.stw(c.r[5].wrapping_add(0x8), c.r[4]);
                if true { return; }
                pc = 0x8009ac28;
            }
            0x8009ac28 => {
                c.r[5] = c.r[5].wrapping_add(0xc);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009abf4; continue; }
                pc = 0x8009ac34;
            }
            0x8009ac34 => {
                c.r[3] = 0xffffffff;
                if true { return; }
                return;
            }
            _ => panic!("f_8009abd0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009AC3C (0x8009ac3c, 20 bytes)
#[inline(never)]
pub fn f_8009ac3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ac3c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ac3c); f_8009ac3c_b(c, 0x8009ac3c); crate::prof::exit(c, 0x8009ac3c); return; }
    }
    f_8009ac3c_b(c, 0x8009ac3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ac3c_at(c: &mut Cpu, pc: u32) { f_8009ac3c_b(c, pc) }
#[inline(always)]
fn f_8009ac3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ac3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ac3c, pc); }
        match pc {
            0x8009ac3c => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ec8));
                c.r[4] = 0xffffffff;
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009ac3c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009AC50 (0x8009ac50, 864 bytes)
#[inline(never)]
pub fn f_8009ac50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ac50) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ac50); f_8009ac50_b(c, 0x8009ac50); crate::prof::exit(c, 0x8009ac50); return; }
    }
    f_8009ac50_b(c, 0x8009ac50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ac50_at(c: &mut Cpu, pc: u32) { f_8009ac50_b(c, pc) }
#[inline(always)]
fn f_8009ac50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ac50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ac50, pc); }
        match pc {
            0x8009ac50 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80310000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[6] = c.r[4].wrapping_add(0x198);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[4] = 0x0;
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff9ec8));
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(2) { pc = 0x8009ac90; continue; }
                pc = 0x8009ac78;
            }
            0x8009ac78 => {
                c.r[3] = c.r[6].wrapping_add(0x0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x37d;
                c.lr = 0x8009ac88; f_8009c2e0(c);
                pc = 0x8009ac88;
            }
            0x8009ac88 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ac90;
            }
            0x8009ac90 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(0) { pc = 0x8009aca4; continue; }
                pc = 0x8009ac98;
            }
            0x8009ac98 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9ecc));
                c.cmps(0, c.r[3], c.r[5]);
                if c.crbit(0) { pc = 0x8009acbc; continue; }
                pc = 0x8009aca4;
            }
            0x8009aca4 => {
                c.r[3] = c.r[6].wrapping_add(0x24);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x37e;
                c.lr = 0x8009acb4; f_8009c2e0(c);
                pc = 0x8009acb4;
            }
            0x8009acb4 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009acbc;
            }
            0x8009acbc => {
                c.r[3] = c.r[3].wrapping_mul(0xc);
                c.r[5] = c.r[7].wrapping_add(c.r[3]);
                c.r[3] = c.lwz(c.r[5]);
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x8009ace8; continue; }
                pc = 0x8009acd0;
            }
            0x8009acd0 => {
                c.r[3] = c.r[6].wrapping_add(0x5c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x381;
                c.lr = 0x8009ace0; f_8009c2e0(c);
                pc = 0x8009ace0;
            }
            0x8009ace0 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ace8;
            }
            0x8009ace8 => {
                c.r[7] = c.lwz(c.r[5].wrapping_add(0x8));
                c.cmpu(0, c.r[7], 0x0);
                if c.crbit(2) { pc = 0x8009ad18; continue; }
                pc = 0x8009acf4;
            }
            0x8009acf4 => {
                c.r[3] = c.lwz(c.r[7]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009ad18; continue; }
                pc = 0x8009ad00;
            }
            0x8009ad00 => {
                c.r[3] = c.r[6].wrapping_add(0x84);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x383;
                c.lr = 0x8009ad10; f_8009c2e0(c);
                pc = 0x8009ad10;
            }
            0x8009ad10 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ad18;
            }
            0x8009ad18 => {
                c.r[9] = c.lwz(c.r[13].wrapping_add(0xffff9ed0));
                c.r[8] = c.r[7] | c.r[7];
                c.r[10] = c.lwz(c.r[13].wrapping_add(0xffff9ed4));
                pc = 0x8009ae14; continue;
                pc = 0x8009ad28;
            }
            0x8009ad28 => {
                c.cmpu(0, c.r[9], c.r[8]);
                if c.crbit(1) { pc = 0x8009ad38; continue; }
                pc = 0x8009ad30;
            }
            0x8009ad30 => {
                c.cmpu(0, c.r[8], c.r[10]);
                if c.crbit(0) { pc = 0x8009ad50; continue; }
                pc = 0x8009ad38;
            }
            0x8009ad38 => {
                c.r[3] = c.r[6].wrapping_add(0xd4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x386;
                c.lr = 0x8009ad48; f_8009c2e0(c);
                pc = 0x8009ad48;
            }
            0x8009ad48 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ad50;
            }
            0x8009ad50 => {
                c.r[3] = c.r[8].rotate_left(0) & 0x1f;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009ad70; continue; }
                pc = 0x8009ad58;
            }
            0x8009ad58 => {
                c.r[3] = c.r[6].wrapping_add(0x114);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x387;
                c.lr = 0x8009ad68; f_8009c2e0(c);
                pc = 0x8009ad68;
            }
            0x8009ad68 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ad70;
            }
            0x8009ad70 => {
                c.r[7] = c.lwz(c.r[8].wrapping_add(0x4));
                c.cmpu(0, c.r[7], 0x0);
                if c.crbit(2) { pc = 0x8009ada0; continue; }
                pc = 0x8009ad7c;
            }
            0x8009ad7c => {
                c.r[3] = c.lwz(c.r[7]);
                c.cmpu(0, c.r[3], c.r[8]);
                if c.crbit(2) { pc = 0x8009ada0; continue; }
                pc = 0x8009ad88;
            }
            0x8009ad88 => {
                c.r[3] = c.r[6].wrapping_add(0x14c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x388;
                c.lr = 0x8009ad98; f_8009c2e0(c);
                pc = 0x8009ad98;
            }
            0x8009ad98 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ada0;
            }
            0x8009ada0 => {
                c.r[8] = c.lwz(c.r[8].wrapping_add(0x8));
                c.cmpu(0, c.r[8], 0x40);
                if !c.crbit(0) { pc = 0x8009adc4; continue; }
                pc = 0x8009adac;
            }
            0x8009adac => {
                c.r[3] = c.r[6].wrapping_add(0x198);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x389;
                c.lr = 0x8009adbc; f_8009c2e0(c);
                pc = 0x8009adbc;
            }
            0x8009adbc => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009adc4;
            }
            0x8009adc4 => {
                c.r[3] = c.r[8].rotate_left(0) & 0x1f;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009ade4; continue; }
                pc = 0x8009adcc;
            }
            0x8009adcc => {
                c.r[3] = c.r[6].wrapping_add(0x1cc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x38a;
                c.lr = 0x8009addc; f_8009c2e0(c);
                pc = 0x8009addc;
            }
            0x8009addc => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ade4;
            }
            0x8009ade4 => {
                c.r[0] = c.r[0].wrapping_add(c.r[8]);
                c.rc(c.r[0]);
                if !c.crbit(1) { pc = 0x8009adf8; continue; }
                pc = 0x8009adec;
            }
            0x8009adec => {
                c.r[3] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], c.r[3]);
                if !c.crbit(1) { pc = 0x8009ae10; continue; }
                pc = 0x8009adf8;
            }
            0x8009adf8 => {
                c.r[3] = c.r[6].wrapping_add(0x20c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x38d;
                c.lr = 0x8009ae08; f_8009c2e0(c);
                pc = 0x8009ae08;
            }
            0x8009ae08 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ae10;
            }
            0x8009ae10 => {
                c.r[8] = c.r[7] | c.r[7];
                pc = 0x8009ae14;
            }
            0x8009ae14 => {
                c.cmpu(0, c.r[8], 0x0);
                if !c.crbit(2) { pc = 0x8009ad28; continue; }
                pc = 0x8009ae1c;
            }
            0x8009ae1c => {
                c.r[11] = c.lwz(c.r[5].wrapping_add(0x4));
                c.cmpu(0, c.r[11], 0x0);
                if c.crbit(2) { pc = 0x8009af70; continue; }
                pc = 0x8009ae28;
            }
            0x8009ae28 => {
                c.r[3] = c.lwz(c.r[11]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009af70; continue; }
                pc = 0x8009ae34;
            }
            0x8009ae34 => {
                c.r[3] = c.r[6].wrapping_add(0x248);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x395;
                c.lr = 0x8009ae44; f_8009c2e0(c);
                pc = 0x8009ae44;
            }
            0x8009ae44 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ae4c;
            }
            0x8009ae4c => {
                pc = 0x8009af70; continue;
                pc = 0x8009ae50;
            }
            0x8009ae50 => {
                c.cmpu(0, c.r[9], c.r[11]);
                if c.crbit(1) { pc = 0x8009ae60; continue; }
                pc = 0x8009ae58;
            }
            0x8009ae58 => {
                c.cmpu(0, c.r[11], c.r[10]);
                if c.crbit(0) { pc = 0x8009ae78; continue; }
                pc = 0x8009ae60;
            }
            0x8009ae60 => {
                c.r[3] = c.r[6].wrapping_add(0xd4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x398;
                c.lr = 0x8009ae70; f_8009c2e0(c);
                pc = 0x8009ae70;
            }
            0x8009ae70 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ae78;
            }
            0x8009ae78 => {
                c.r[3] = c.r[11].rotate_left(0) & 0x1f;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009ae98; continue; }
                pc = 0x8009ae80;
            }
            0x8009ae80 => {
                c.r[3] = c.r[6].wrapping_add(0x114);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x399;
                c.lr = 0x8009ae90; f_8009c2e0(c);
                pc = 0x8009ae90;
            }
            0x8009ae90 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009ae98;
            }
            0x8009ae98 => {
                c.r[7] = c.lwz(c.r[11].wrapping_add(0x4));
                c.cmpu(0, c.r[7], 0x0);
                if c.crbit(2) { pc = 0x8009aec8; continue; }
                pc = 0x8009aea4;
            }
            0x8009aea4 => {
                c.r[3] = c.lwz(c.r[7]);
                c.cmpu(0, c.r[3], c.r[11]);
                if c.crbit(2) { pc = 0x8009aec8; continue; }
                pc = 0x8009aeb0;
            }
            0x8009aeb0 => {
                c.r[3] = c.r[6].wrapping_add(0x14c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x39a;
                c.lr = 0x8009aec0; f_8009c2e0(c);
                pc = 0x8009aec0;
            }
            0x8009aec0 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009aec8;
            }
            0x8009aec8 => {
                c.r[8] = c.lwz(c.r[11].wrapping_add(0x8));
                c.cmpu(0, c.r[8], 0x40);
                if !c.crbit(0) { pc = 0x8009aeec; continue; }
                pc = 0x8009aed4;
            }
            0x8009aed4 => {
                c.r[3] = c.r[6].wrapping_add(0x198);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x39b;
                c.lr = 0x8009aee4; f_8009c2e0(c);
                pc = 0x8009aee4;
            }
            0x8009aee4 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009aeec;
            }
            0x8009aeec => {
                c.r[3] = c.r[8].rotate_left(0) & 0x1f;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009af0c; continue; }
                pc = 0x8009aef4;
            }
            0x8009aef4 => {
                c.r[3] = c.r[6].wrapping_add(0x1cc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x39c;
                c.lr = 0x8009af04; f_8009c2e0(c);
                pc = 0x8009af04;
            }
            0x8009af04 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009af0c;
            }
            0x8009af0c => {
                c.cmpu(0, c.r[7], 0x0);
                if c.crbit(2) { pc = 0x8009af38; continue; }
                pc = 0x8009af14;
            }
            0x8009af14 => {
                c.r[3] = c.r[11].wrapping_add(c.r[8]);
                c.cmpu(0, c.r[3], c.r[7]);
                if c.crbit(0) { pc = 0x8009af38; continue; }
                pc = 0x8009af20;
            }
            0x8009af20 => {
                c.r[3] = c.r[6].wrapping_add(0x290);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x39d;
                c.lr = 0x8009af30; f_8009c2e0(c);
                pc = 0x8009af30;
            }
            0x8009af30 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009af38;
            }
            0x8009af38 => {
                c.r[0] = c.r[0].wrapping_add(c.r[8]);
                c.rc(c.r[0]);
                c.r[4] = c.r[8].wrapping_add(c.r[4]);
                c.r[4] = c.r[4].wrapping_add(0xffffffe0);
                if !c.crbit(1) { pc = 0x8009af54; continue; }
                pc = 0x8009af48;
            }
            0x8009af48 => {
                c.r[3] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], c.r[3]);
                if !c.crbit(1) { pc = 0x8009af6c; continue; }
                pc = 0x8009af54;
            }
            0x8009af54 => {
                c.r[3] = c.r[6].wrapping_add(0x20c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x3a1;
                c.lr = 0x8009af64; f_8009c2e0(c);
                pc = 0x8009af64;
            }
            0x8009af64 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009af6c;
            }
            0x8009af6c => {
                c.r[11] = c.r[7] | c.r[7];
                pc = 0x8009af70;
            }
            0x8009af70 => {
                c.cmpu(0, c.r[11], 0x0);
                if !c.crbit(2) { pc = 0x8009ae50; continue; }
                pc = 0x8009af78;
            }
            0x8009af78 => {
                c.r[3] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], c.r[3]);
                if c.crbit(2) { pc = 0x8009af9c; continue; }
                pc = 0x8009af84;
            }
            0x8009af84 => {
                c.r[3] = c.r[6].wrapping_add(0x2f0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x3a8;
                c.lr = 0x8009af94; f_8009c2e0(c);
                pc = 0x8009af94;
            }
            0x8009af94 => {
                c.r[3] = 0xffffffff;
                pc = 0x8009afa0; continue;
                pc = 0x8009af9c;
            }
            0x8009af9c => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x8009afa0;
            }
            0x8009afa0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009ac50: bad pc {:#x}", pc),
        }
    }
}

/// OSGetArenaHi (0x8009afb0, 8 bytes)
#[inline(never)]
pub fn f_8009afb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009afb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009afb0); f_8009afb0_b(c, 0x8009afb0); crate::prof::exit(c, 0x8009afb0); return; }
    }
    f_8009afb0_b(c, 0x8009afb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009afb0_at(c: &mut Cpu, pc: u32) { f_8009afb0_b(c, pc) }
#[inline(always)]
fn f_8009afb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009afb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009afb0, pc); }
        match pc {
            0x8009afb0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ed8));
                if true { return; }
                return;
            }
            _ => panic!("f_8009afb0: bad pc {:#x}", pc),
        }
    }
}

/// OSGetArenaLo (0x8009afb8, 8 bytes)
#[inline(never)]
pub fn f_8009afb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009afb8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009afb8); f_8009afb8_b(c, 0x8009afb8); crate::prof::exit(c, 0x8009afb8); return; }
    }
    f_8009afb8_b(c, 0x8009afb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009afb8_at(c: &mut Cpu, pc: u32) { f_8009afb8_b(c, pc) }
#[inline(always)]
fn f_8009afb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009afb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009afb8, pc); }
        match pc {
            0x8009afb8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8168));
                if true { return; }
                return;
            }
            _ => panic!("f_8009afb8: bad pc {:#x}", pc),
        }
    }
}

/// OSSetArenaHi (0x8009afc0, 8 bytes)
#[inline(never)]
pub fn f_8009afc0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009afc0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009afc0); f_8009afc0_b(c, 0x8009afc0); crate::prof::exit(c, 0x8009afc0); return; }
    }
    f_8009afc0_b(c, 0x8009afc0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009afc0_at(c: &mut Cpu, pc: u32) { f_8009afc0_b(c, pc) }
#[inline(always)]
fn f_8009afc0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009afc0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009afc0, pc); }
        match pc {
            0x8009afc0 => {
                c.stw(c.r[13].wrapping_add(0xffff9ed8), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009afc0: bad pc {:#x}", pc),
        }
    }
}

/// OSSetArenaLo (0x8009afc8, 8 bytes)
#[inline(never)]
pub fn f_8009afc8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009afc8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009afc8); f_8009afc8_b(c, 0x8009afc8); crate::prof::exit(c, 0x8009afc8); return; }
    }
    f_8009afc8_b(c, 0x8009afc8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009afc8_at(c: &mut Cpu, pc: u32) { f_8009afc8_b(c, pc) }
#[inline(always)]
fn f_8009afc8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009afc8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009afc8, pc); }
        match pc {
            0x8009afc8 => {
                c.stw(c.r[13].wrapping_add(0xffff8168), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009afc8: bad pc {:#x}", pc),
        }
    }
}

/// OSAllocFromArenaLo (0x8009afd0, 44 bytes)
#[inline(never)]
pub fn f_8009afd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009afd0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009afd0); f_8009afd0_b(c, 0x8009afd0); crate::prof::exit(c, 0x8009afd0); return; }
    }
    f_8009afd0_b(c, 0x8009afd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009afd0_at(c: &mut Cpu, pc: u32) { f_8009afd0_b(c, pc) }
#[inline(always)]
fn f_8009afd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009afd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009afd0, pc); }
        match pc {
            0x8009afd0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8168));
                c.r[4] = c.r[4].wrapping_add(0xffffffff);
                c.r[5] = !(c.r[4] | c.r[4]);
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.r[6] = c.r[5] & c.r[0];
                c.r[0] = c.r[6].wrapping_add(c.r[3]);
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.r[5] & c.r[0];
                c.stw(c.r[13].wrapping_add(0xffff8168), c.r[0]);
                c.r[3] = c.r[6] | c.r[6];
                if true { return; }
                return;
            }
            _ => panic!("f_8009afd0: bad pc {:#x}", pc),
        }
    }
}

/// __OSInitAudioSystem (0x8009affc, 444 bytes)
#[inline(never)]
pub fn f_8009affc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009affc) { return; }
        if c.prof { crate::prof::enter(c, 0x8009affc); f_8009affc_b(c, 0x8009affc); crate::prof::exit(c, 0x8009affc); return; }
    }
    f_8009affc_b(c, 0x8009affc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009affc_at(c: &mut Cpu, pc: u32) { f_8009affc_b(c, pc) }
#[inline(always)]
fn f_8009affc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009affc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009affc, pc); }
        match pc {
            0x8009affc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x8009b010; f_8009afb0(c);
                pc = 0x8009b010;
            }
            0x8009b010 => {
                c.r[4] = 0x81000000;
                c.r[5] = 0x80;
                c.r[3] = c.r[3].wrapping_add(0xffffff80);
                c.lr = 0x8009b020; f_800054f4(c);
                pc = 0x8009b020;
            }
            0x8009b020 => {
                c.r[3] = 0x80310000;
                c.r[4] = c.r[3].wrapping_add(0x530);
                c.r[3] = 0x81000000;
                c.r[5] = 0x80;
                c.lr = 0x8009b034; f_800054f4(c);
                pc = 0x8009b034;
            }
            0x8009b034 => {
                c.r[3] = 0x81000000;
                c.r[4] = 0x80;
                c.lr = 0x8009b040; f_8009b2d0(c);
                pc = 0x8009b040;
            }
            0x8009b040 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x5000);
                c.r[0] = 0x43;
                c.sth(c.r[3].wrapping_add(0x12), c.r[0]);
                c.r[31] = c.r[3].wrapping_add(0xa);
                c.r[0] = 0x8ac;
                c.sth(c.r[3].wrapping_add(0xa), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0xa));
                c.r[0] = c.r[0] | 0x1;
                c.sth(c.r[3].wrapping_add(0xa), c.r[0]);
                pc = 0x8009b068;
            }
            0x8009b068 => {
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009b068; continue; }
                pc = 0x8009b074;
            }
            0x8009b074 => {
                c.r[0] = 0x0;
                c.r[4] = 0xcc000000;
                c.sth(c.r[4].wrapping_add(0x5000), c.r[0]);
                pc = 0x8009b080;
            }
            0x8009b080 => {
                c.r[30] = c.r[4].wrapping_add(0x5000);
                { let ea = c.r[30].wrapping_add(0x4); c.r[3] = c.lhz(ea); c.r[30] = ea; }
                c.r[29] = c.r[4].wrapping_add(0x5000);
                { let ea = c.r[29].wrapping_add(0x6); c.r[0] = c.lhz(ea); c.r[29] = ea; }
                c.r[0] = (c.r[3].rotate_left(16) & 0xffff0000) | (c.r[0] & 0xffff);
                c.r[0] = c.r[0].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009b080; continue; }
                pc = 0x8009b09c;
            }
            0x8009b09c => {
                c.r[3] = 0xcc000000;
                c.r[0] = 0x1000000;
                c.r[27] = c.r[3].wrapping_add(0x5000);
                { let ea = c.r[27].wrapping_add(0x20); c.stw(ea, c.r[0]); c.r[27] = ea; }
                c.r[0] = 0x0;
                c.r[26] = c.r[3].wrapping_add(0x5000);
                { let ea = c.r[26].wrapping_add(0x24); c.stw(ea, c.r[0]); c.r[26] = ea; }
                c.r[0] = 0x20;
                c.r[25] = c.r[3].wrapping_add(0x5000);
                { let ea = c.r[25].wrapping_add(0x28); c.stw(ea, c.r[0]); c.r[25] = ea; }
                c.r[3] = c.lhz(c.r[31]);
                pc = 0x8009b0d0; continue;
                pc = 0x8009b0cc;
            }
            0x8009b0cc => {
                c.r[3] = c.lhz(c.r[31]);
                pc = 0x8009b0d0;
            }
            0x8009b0d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x20;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009b0cc; continue; }
                pc = 0x8009b0d8;
            }
            0x8009b0d8 => {
                c.sth(c.r[31], c.r[3]);
                c.lr = 0x8009b0e0; f_800a2790(c);
                pc = 0x8009b0e0;
            }
            0x8009b0e0 => {
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8009b0e4;
            }
            0x8009b0e4 => {
                c.lr = 0x8009b0e8; f_800a2790(c);
                pc = 0x8009b0e8;
            }
            0x8009b0e8 => {
                c.r[0] = c.r[3].wrapping_sub(c.r[28]);
                c.cmps(0, c.r[0], 0x892);
                if c.crbit(0) { pc = 0x8009b0e4; continue; }
                pc = 0x8009b0f4;
            }
            0x8009b0f4 => {
                c.r[0] = 0x1000000;
                c.stw(c.r[27], c.r[0]);
                c.r[3] = 0x0;
                c.r[0] = 0x20;
                c.stw(c.r[26], c.r[3]);
                c.stw(c.r[25], c.r[0]);
                c.r[3] = c.lhz(c.r[31]);
                pc = 0x8009b118; continue;
                pc = 0x8009b114;
            }
            0x8009b114 => {
                c.r[3] = c.lhz(c.r[31]);
                pc = 0x8009b118;
            }
            0x8009b118 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x20;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009b114; continue; }
                pc = 0x8009b120;
            }
            0x8009b120 => {
                c.sth(c.r[31], c.r[3]);
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(0) & 0xfffff7ff;
                c.sth(c.r[31], c.r[0]);
                pc = 0x8009b130;
            }
            0x8009b130 => {
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(0) & 0x400;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009b130; continue; }
                pc = 0x8009b13c;
            }
            0x8009b13c => {
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffb;
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[30]);
                pc = 0x8009b154; continue;
                pc = 0x8009b150;
            }
            0x8009b150 => {
                c.r[0] = c.lhz(c.r[30]);
                pc = 0x8009b154;
            }
            0x8009b154 => {
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].rotate_left(0) & 0x8000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009b150; continue; }
                pc = 0x8009b160;
            }
            0x8009b160 => {
                c.r[0] = c.lhz(c.r[31]);
                c.r[4] = c.lhz(c.r[29]);
                c.r[0] = c.r[0] | 0x4;
                c.sth(c.r[31], c.r[0]);
                c.r[0] = 0x8ac;
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0] | 0x1;
                c.sth(c.r[31], c.r[0]);
                pc = 0x8009b184;
            }
            0x8009b184 => {
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009b184; continue; }
                pc = 0x8009b190;
            }
            0x8009b190 => {
                c.lr = 0x8009b194; f_8009afb0(c);
                pc = 0x8009b194;
            }
            0x8009b194 => {
                c.r[4] = c.r[3].wrapping_add(0xffffff80);
                c.r[3] = 0x81000000;
                c.r[5] = 0x80;
                c.lr = 0x8009b1a4; f_800054f4(c);
                pc = 0x8009b1a4;
            }
            0x8009b1a4 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009affc: bad pc {:#x}", pc),
        }
    }
}

/// __OSStopAudioSystem (0x8009b1b8, 216 bytes)
#[inline(never)]
pub fn f_8009b1b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b1b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b1b8); f_8009b1b8_b(c, 0x8009b1b8); crate::prof::exit(c, 0x8009b1b8); return; }
    }
    f_8009b1b8_b(c, 0x8009b1b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b1b8_at(c: &mut Cpu, pc: u32) { f_8009b1b8_b(c, pc) }
#[inline(always)]
fn f_8009b1b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b1b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b1b8, pc); }
        match pc {
            0x8009b1b8 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0xcc000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x804;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x5000);
                c.r[3] = c.r[3].wrapping_add(0x5000);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.sth(c.r[31].wrapping_add(0xa), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x36));
                c.r[0] = c.r[0].rotate_left(0) & 0xffff7fff;
                c.sth(c.r[3].wrapping_add(0x36), c.r[0]);
                { let ea = c.r[31].wrapping_add(0xa); c.r[0] = c.lhz(ea); c.r[31] = ea; }
                pc = 0x8009b1f8; continue;
                pc = 0x8009b1f4;
            }
            0x8009b1f4 => {
                c.r[0] = c.lhz(c.r[31]);
                pc = 0x8009b1f8;
            }
            0x8009b1f8 => {
                c.r[0] = c.r[0].rotate_left(0) & 0x400;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009b1f4; continue; }
                pc = 0x8009b200;
            }
            0x8009b200 => {
                c.r[0] = c.lhz(c.r[31]);
                pc = 0x8009b20c; continue;
                pc = 0x8009b208;
            }
            0x8009b208 => {
                c.r[0] = c.lhz(c.r[31]);
                pc = 0x8009b20c;
            }
            0x8009b20c => {
                c.r[0] = c.r[0].rotate_left(0) & 0x200;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009b208; continue; }
                pc = 0x8009b214;
            }
            0x8009b214 => {
                c.r[0] = 0x8ac;
                c.r[3] = 0xcc000000;
                c.sth(c.r[31], c.r[0]);
                c.r[0] = 0x0;
                c.sth(c.r[3].wrapping_add(0x5000), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0x5000);
                pc = 0x8009b22c;
            }
            0x8009b22c => {
                c.r[3] = c.lhz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x6));
                c.r[0] = (c.r[3].rotate_left(16) & 0xffff0000) | (c.r[0] & 0xffff);
                c.r[0] = c.r[0].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009b22c; continue; }
                pc = 0x8009b240;
            }
            0x8009b240 => {
                c.lr = 0x8009b244; f_800a2790(c);
                pc = 0x8009b244;
            }
            0x8009b244 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x8009b248;
            }
            0x8009b248 => {
                c.lr = 0x8009b24c; f_800a2790(c);
                pc = 0x8009b24c;
            }
            0x8009b24c => {
                c.r[0] = c.r[3].wrapping_sub(c.r[30]);
                c.cmps(0, c.r[0], 0x2c);
                if c.crbit(0) { pc = 0x8009b248; continue; }
                pc = 0x8009b258;
            }
            0x8009b258 => {
                c.r[0] = c.lhz(c.r[31]);
                c.r[0] = c.r[0] | 0x1;
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[31]);
                pc = 0x8009b270; continue;
                pc = 0x8009b26c;
            }
            0x8009b26c => {
                c.r[0] = c.lhz(c.r[31]);
                pc = 0x8009b270;
            }
            0x8009b270 => {
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009b26c; continue; }
                pc = 0x8009b278;
            }
            0x8009b278 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009b1b8: bad pc {:#x}", pc),
        }
    }
}

/// DCEnable (0x8009b290, 20 bytes)
#[inline(never)]
pub fn f_8009b290(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b290) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b290); f_8009b290_b(c, 0x8009b290); crate::prof::exit(c, 0x8009b290); return; }
    }
    f_8009b290_b(c, 0x8009b290)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b290_at(c: &mut Cpu, pc: u32) { f_8009b290_b(c, pc) }
#[inline(always)]
fn f_8009b290_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b290;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b290, pc); }
        match pc {
            0x8009b290 => {
                // sync
                c.r[3] = c.mfspr(1008);
                c.r[3] = c.r[3] | 0x4000;
                c.mtspr(1008, c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009b290: bad pc {:#x}", pc),
        }
    }
}

/// DCInvalidateRange (0x8009b2a4, 44 bytes)
#[inline(never)]
pub fn f_8009b2a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b2a4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b2a4); f_8009b2a4_b(c, 0x8009b2a4); crate::prof::exit(c, 0x8009b2a4); return; }
    }
    f_8009b2a4_b(c, 0x8009b2a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b2a4_at(c: &mut Cpu, pc: u32) { f_8009b2a4_b(c, pc) }
#[inline(always)]
fn f_8009b2a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b2a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b2a4, pc); }
        match pc {
            0x8009b2a4 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(1) { return; }
                pc = 0x8009b2ac;
            }
            0x8009b2ac => {
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.r[4].wrapping_add(0x1f);
                c.r[4] = c.r[4].rotate_left(27) & 0x7ffffff;
                c.mtspr(9, c.r[4]);
                pc = 0x8009b2c0;
            }
            0x8009b2c0 => {
                // dcbi
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009b2c0; continue; }
                pc = 0x8009b2cc;
            }
            0x8009b2cc => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009b2a4: bad pc {:#x}", pc),
        }
    }
}

/// DCFlushRange (0x8009b2d0, 48 bytes)
#[inline(never)]
pub fn f_8009b2d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b2d0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b2d0); f_8009b2d0_b(c, 0x8009b2d0); crate::prof::exit(c, 0x8009b2d0); return; }
    }
    f_8009b2d0_b(c, 0x8009b2d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b2d0_at(c: &mut Cpu, pc: u32) { f_8009b2d0_b(c, pc) }
#[inline(always)]
fn f_8009b2d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b2d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b2d0, pc); }
        match pc {
            0x8009b2d0 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(1) { return; }
                pc = 0x8009b2d8;
            }
            0x8009b2d8 => {
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.r[4].wrapping_add(0x1f);
                c.r[4] = c.r[4].rotate_left(27) & 0x7ffffff;
                c.mtspr(9, c.r[4]);
                pc = 0x8009b2ec;
            }
            0x8009b2ec => {
                // dcbf
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009b2ec; continue; }
                pc = 0x8009b2f8;
            }
            0x8009b2f8 => {
                // sc
                if true { return; }
                return;
            }
            _ => panic!("f_8009b2d0: bad pc {:#x}", pc),
        }
    }
}

/// DCStoreRange (0x8009b300, 48 bytes)
#[inline(never)]
pub fn f_8009b300(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b300) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b300); f_8009b300_b(c, 0x8009b300); crate::prof::exit(c, 0x8009b300); return; }
    }
    f_8009b300_b(c, 0x8009b300)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b300_at(c: &mut Cpu, pc: u32) { f_8009b300_b(c, pc) }
#[inline(always)]
fn f_8009b300_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b300;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b300, pc); }
        match pc {
            0x8009b300 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(1) { return; }
                pc = 0x8009b308;
            }
            0x8009b308 => {
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.r[4].wrapping_add(0x1f);
                c.r[4] = c.r[4].rotate_left(27) & 0x7ffffff;
                c.mtspr(9, c.r[4]);
                pc = 0x8009b31c;
            }
            0x8009b31c => {
                // dcbst
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009b31c; continue; }
                pc = 0x8009b328;
            }
            0x8009b328 => {
                // sc
                if true { return; }
                return;
            }
            _ => panic!("f_8009b300: bad pc {:#x}", pc),
        }
    }
}

/// DCFlushRangeNoSync (0x8009b330, 44 bytes)
#[inline(never)]
pub fn f_8009b330(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b330) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b330); f_8009b330_b(c, 0x8009b330); crate::prof::exit(c, 0x8009b330); return; }
    }
    f_8009b330_b(c, 0x8009b330)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b330_at(c: &mut Cpu, pc: u32) { f_8009b330_b(c, pc) }
#[inline(always)]
fn f_8009b330_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b330;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b330, pc); }
        match pc {
            0x8009b330 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(1) { return; }
                pc = 0x8009b338;
            }
            0x8009b338 => {
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.r[4].wrapping_add(0x1f);
                c.r[4] = c.r[4].rotate_left(27) & 0x7ffffff;
                c.mtspr(9, c.r[4]);
                pc = 0x8009b34c;
            }
            0x8009b34c => {
                // dcbf
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009b34c; continue; }
                pc = 0x8009b358;
            }
            0x8009b358 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009b330: bad pc {:#x}", pc),
        }
    }
}

/// DCStoreRangeNoSync (0x8009b35c, 44 bytes)
#[inline(never)]
pub fn f_8009b35c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b35c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b35c); f_8009b35c_b(c, 0x8009b35c); crate::prof::exit(c, 0x8009b35c); return; }
    }
    f_8009b35c_b(c, 0x8009b35c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b35c_at(c: &mut Cpu, pc: u32) { f_8009b35c_b(c, pc) }
#[inline(always)]
fn f_8009b35c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b35c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b35c, pc); }
        match pc {
            0x8009b35c => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(1) { return; }
                pc = 0x8009b364;
            }
            0x8009b364 => {
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.r[4].wrapping_add(0x1f);
                c.r[4] = c.r[4].rotate_left(27) & 0x7ffffff;
                c.mtspr(9, c.r[4]);
                pc = 0x8009b378;
            }
            0x8009b378 => {
                // dcbst
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009b378; continue; }
                pc = 0x8009b384;
            }
            0x8009b384 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009b35c: bad pc {:#x}", pc),
        }
    }
}

/// DCZeroRange (0x8009b388, 44 bytes)
#[inline(never)]
pub fn f_8009b388(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b388) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b388); f_8009b388_b(c, 0x8009b388); crate::prof::exit(c, 0x8009b388); return; }
    }
    f_8009b388_b(c, 0x8009b388)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b388_at(c: &mut Cpu, pc: u32) { f_8009b388_b(c, pc) }
#[inline(always)]
fn f_8009b388_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b388;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b388, pc); }
        match pc {
            0x8009b388 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(1) { return; }
                pc = 0x8009b390;
            }
            0x8009b390 => {
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.r[4].wrapping_add(0x1f);
                c.r[4] = c.r[4].rotate_left(27) & 0x7ffffff;
                c.mtspr(9, c.r[4]);
                pc = 0x8009b3a4;
            }
            0x8009b3a4 => {
                { let ea = c.r[3] & !31; for k in 0..8 { c.stw(ea + 4 * k, 0); } }
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009b3a4; continue; }
                pc = 0x8009b3b0;
            }
            0x8009b3b0 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009b388: bad pc {:#x}", pc),
        }
    }
}

/// ICInvalidateRange (0x8009b3b4, 52 bytes)
#[inline(never)]
pub fn f_8009b3b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b3b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b3b4); f_8009b3b4_b(c, 0x8009b3b4); crate::prof::exit(c, 0x8009b3b4); return; }
    }
    f_8009b3b4_b(c, 0x8009b3b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b3b4_at(c: &mut Cpu, pc: u32) { f_8009b3b4_b(c, pc) }
#[inline(always)]
fn f_8009b3b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b3b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b3b4, pc); }
        match pc {
            0x8009b3b4 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(1) { return; }
                pc = 0x8009b3bc;
            }
            0x8009b3bc => {
                c.r[5] = c.r[3].rotate_left(0) & 0x1f;
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                c.r[4] = c.r[4].wrapping_add(0x1f);
                c.r[4] = c.r[4].rotate_left(27) & 0x7ffffff;
                c.mtspr(9, c.r[4]);
                pc = 0x8009b3d0;
            }
            0x8009b3d0 => {
                // icbi
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009b3d0; continue; }
                pc = 0x8009b3dc;
            }
            0x8009b3dc => {
                // sync
                // isync
                if true { return; }
                return;
            }
            _ => panic!("f_8009b3b4: bad pc {:#x}", pc),
        }
    }
}

/// ICFlashInvalidate (0x8009b3e8, 16 bytes)
#[inline(never)]
pub fn f_8009b3e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b3e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b3e8); f_8009b3e8_b(c, 0x8009b3e8); crate::prof::exit(c, 0x8009b3e8); return; }
    }
    f_8009b3e8_b(c, 0x8009b3e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b3e8_at(c: &mut Cpu, pc: u32) { f_8009b3e8_b(c, pc) }
#[inline(always)]
fn f_8009b3e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b3e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b3e8, pc); }
        match pc {
            0x8009b3e8 => {
                c.r[3] = c.mfspr(1008);
                c.r[3] = c.r[3] | 0x800;
                c.mtspr(1008, c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009b3e8: bad pc {:#x}", pc),
        }
    }
}

/// ICEnable (0x8009b3f8, 20 bytes)
#[inline(never)]
pub fn f_8009b3f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b3f8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b3f8); f_8009b3f8_b(c, 0x8009b3f8); crate::prof::exit(c, 0x8009b3f8); return; }
    }
    f_8009b3f8_b(c, 0x8009b3f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b3f8_at(c: &mut Cpu, pc: u32) { f_8009b3f8_b(c, pc) }
#[inline(always)]
fn f_8009b3f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b3f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b3f8, pc); }
        match pc {
            0x8009b3f8 => {
                // isync
                c.r[3] = c.mfspr(1008);
                c.r[3] = c.r[3] | 0x8000;
                c.mtspr(1008, c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009b3f8: bad pc {:#x}", pc),
        }
    }
}

/// __LCEnable (0x8009b40c, 204 bytes)
#[inline(never)]
pub fn f_8009b40c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b40c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b40c); f_8009b40c_b(c, 0x8009b40c); crate::prof::exit(c, 0x8009b40c); return; }
    }
    f_8009b40c_b(c, 0x8009b40c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b40c_at(c: &mut Cpu, pc: u32) { f_8009b40c_b(c, pc) }
#[inline(always)]
fn f_8009b40c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b40c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b40c, pc); }
        match pc {
            0x8009b40c => {
                c.r[5] = 0; // mfmsr
                c.r[5] = c.r[5] | 0x1000;
                // mtmsr
                c.r[3] = 0x80000000;
                c.r[4] = 0x400;
                c.mtspr(9, c.r[4]);
                pc = 0x8009b424;
            }
            0x8009b424 => {
                // dcbt
                // dcbst
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009b424; continue; }
                pc = 0x8009b434;
            }
            0x8009b434 => {
                c.r[4] = c.mfspr(920);
                c.r[4] = c.r[4] | 0x100f0000;
                c.mtspr(920, c.r[4]);
                c.r[3] = 0xe0000000;
                c.r[3] = c.r[3] | 0x2;
                c.mtspr(543, c.r[3]);
                c.r[3] = c.r[3] | 0x1fe;
                c.mtspr(542, c.r[3]);
                // isync
                c.r[3] = 0xe0000000;
                c.r[6] = 0x200;
                c.mtspr(9, c.r[6]);
                c.r[6] = 0x0;
                pc = 0x8009b498;
            }
            0x8009b498 => {
                { let ea = c.r[6].wrapping_add(c.r[3]) & !31; for k in 0..8 { c.stw(ea + 4 * k, 0); } }
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009b498; continue; }
                pc = 0x8009b4a4;
            }
            0x8009b4a4 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009b40c: bad pc {:#x}", pc),
        }
    }
}

/// LCEnable (0x8009b4d8, 56 bytes)
#[inline(never)]
pub fn f_8009b4d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b4d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b4d8); f_8009b4d8_b(c, 0x8009b4d8); crate::prof::exit(c, 0x8009b4d8); return; }
    }
    f_8009b4d8_b(c, 0x8009b4d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b4d8_at(c: &mut Cpu, pc: u32) { f_8009b4d8_b(c, pc) }
#[inline(always)]
fn f_8009b4d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b4d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b4d8, pc); }
        match pc {
            0x8009b4d8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x8009b4ec; f_8009df3c(c);
                pc = 0x8009b4ec;
            }
            0x8009b4ec => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8009b4f4; f_8009b40c(c);
                pc = 0x8009b4f4;
            }
            0x8009b4f4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8009b4fc; f_8009df64(c);
                pc = 0x8009b4fc;
            }
            0x8009b4fc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009b4d8: bad pc {:#x}", pc),
        }
    }
}

/// LCDisable (0x8009b510, 40 bytes)
#[inline(never)]
pub fn f_8009b510(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b510) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b510); f_8009b510_b(c, 0x8009b510); crate::prof::exit(c, 0x8009b510); return; }
    }
    f_8009b510_b(c, 0x8009b510)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b510_at(c: &mut Cpu, pc: u32) { f_8009b510_b(c, pc) }
#[inline(always)]
fn f_8009b510_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b510;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b510, pc); }
        match pc {
            0x8009b510 => {
                c.r[3] = 0xe0000000;
                c.r[4] = 0x200;
                c.mtspr(9, c.r[4]);
                pc = 0x8009b51c;
            }
            0x8009b51c => {
                // dcbi
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009b51c; continue; }
                pc = 0x8009b528;
            }
            0x8009b528 => {
                c.r[4] = c.mfspr(920);
                c.r[4] = c.r[4].rotate_left(0) & 0xefffffff;
                c.mtspr(920, c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009b510: bad pc {:#x}", pc),
        }
    }
}

/// LCStoreBlocks (0x8009b538, 36 bytes)
#[inline(never)]
pub fn f_8009b538(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b538) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b538); f_8009b538_b(c, 0x8009b538); crate::prof::exit(c, 0x8009b538); return; }
    }
    f_8009b538_b(c, 0x8009b538)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b538_at(c: &mut Cpu, pc: u32) { f_8009b538_b(c, pc) }
#[inline(always)]
fn f_8009b538_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b538;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b538, pc); }
        match pc {
            0x8009b538 => {
                c.r[6] = c.r[5].rotate_left(30) & 0x1f;
                c.r[3] = c.r[3].rotate_left(0) & 0xfffffff;
                c.r[6] = c.r[6] | c.r[3];
                c.mtspr(922, c.r[6]);
                c.r[6] = c.r[5].rotate_left(2) & 0xc;
                c.r[6] = c.r[6] | c.r[4];
                c.r[6] = c.r[6] | 0x2;
                c.mtspr(923, c.r[6]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009b538: bad pc {:#x}", pc),
        }
    }
}

/// LCStoreData (0x8009b55c, 172 bytes)
#[inline(never)]
pub fn f_8009b55c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b55c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b55c); f_8009b55c_b(c, 0x8009b55c); crate::prof::exit(c, 0x8009b55c); return; }
    }
    f_8009b55c_b(c, 0x8009b55c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b55c_at(c: &mut Cpu, pc: u32) { f_8009b55c_b(c, pc) }
#[inline(always)]
fn f_8009b55c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b55c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b55c, pc); }
        match pc {
            0x8009b55c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[0] = c.r[3].wrapping_add(0x7f);
                c.r[31] = c.r[3] | c.r[3];
                c.r[30] = c.r[0].rotate_left(25) & 0x1ffffff;
                pc = 0x8009b598; continue;
                pc = 0x8009b598;
            }
            0x8009b598 => {
                pc = 0x8009b59c; continue;
                pc = 0x8009b59c;
            }
            0x8009b59c => {
                pc = 0x8009b5dc; continue;
                pc = 0x8009b5a0;
            }
            0x8009b5a0 => {
                c.cmpu(0, c.r[31], 0x80);
                if !c.crbit(0) { pc = 0x8009b5c0; continue; }
                pc = 0x8009b5a8;
            }
            0x8009b5a8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x8009b5b8; f_8009b538(c);
                pc = 0x8009b5b8;
            }
            0x8009b5b8 => {
                c.r[31] = 0x0;
                pc = 0x8009b5dc; continue;
                pc = 0x8009b5c0;
            }
            0x8009b5c0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.lr = 0x8009b5d0; f_8009b538(c);
                pc = 0x8009b5d0;
            }
            0x8009b5d0 => {
                c.r[31] = c.r[31].wrapping_add(0xffffff80);
                c.r[28] = c.r[28].wrapping_add(0x1000);
                c.r[29] = c.r[29].wrapping_add(0x1000);
                pc = 0x8009b5dc;
            }
            0x8009b5dc => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8009b5a0; continue; }
                pc = 0x8009b5e4;
            }
            0x8009b5e4 => {
                c.r[3] = c.r[30] | c.r[30];
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
            _ => panic!("f_8009b55c: bad pc {:#x}", pc),
        }
    }
}

/// LCQueueLength (0x8009b608, 12 bytes)
#[inline(never)]
pub fn f_8009b608(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b608) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b608); f_8009b608_b(c, 0x8009b608); crate::prof::exit(c, 0x8009b608); return; }
    }
    f_8009b608_b(c, 0x8009b608)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b608_at(c: &mut Cpu, pc: u32) { f_8009b608_b(c, pc) }
#[inline(always)]
fn f_8009b608_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b608;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b608, pc); }
        match pc {
            0x8009b608 => {
                c.r[4] = c.mfspr(920);
                c.r[3] = c.r[4].rotate_left(8) & 0xf;
                if true { return; }
                return;
            }
            _ => panic!("f_8009b608: bad pc {:#x}", pc),
        }
    }
}

/// LCQueueWait (0x8009b614, 20 bytes)
#[inline(never)]
pub fn f_8009b614(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b614) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b614); f_8009b614_b(c, 0x8009b614); crate::prof::exit(c, 0x8009b614); return; }
    }
    f_8009b614_b(c, 0x8009b614)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b614_at(c: &mut Cpu, pc: u32) { f_8009b614_b(c, pc) }
#[inline(always)]
fn f_8009b614_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b614;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b614, pc); }
        match pc {
            0x8009b614 => {
                c.r[4] = c.mfspr(920);
                c.r[4] = c.r[4].rotate_left(8) & 0xf;
                c.cmps(0, c.r[4], c.r[3]);
                if c.crbit(1) { pc = 0x8009b614; continue; }
                pc = 0x8009b624;
            }
            0x8009b624 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009b614: bad pc {:#x}", pc),
        }
    }
}

/// L2GlobalInvalidate (0x8009b628, 152 bytes)
#[inline(never)]
pub fn f_8009b628(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b628) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b628); f_8009b628_b(c, 0x8009b628); crate::prof::exit(c, 0x8009b628); return; }
    }
    f_8009b628_b(c, 0x8009b628)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b628_at(c: &mut Cpu, pc: u32) { f_8009b628_b(c, pc) }
#[inline(always)]
fn f_8009b628_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b628;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b628, pc); }
        match pc {
            0x8009b628 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                // sync
                c.lr = 0x8009b640; f_8009801c(c);
                pc = 0x8009b640;
            }
            0x8009b640 => {
                c.r[3] = c.r[3].rotate_left(0) & 0x7fffffff;
                c.lr = 0x8009b648; f_80098024(c);
                pc = 0x8009b648;
            }
            0x8009b648 => {
                // sync
                c.lr = 0x8009b650; f_8009801c(c);
                pc = 0x8009b650;
            }
            0x8009b650 => {
                c.r[3] = c.r[3] | 0x200000;
                c.lr = 0x8009b658; f_80098024(c);
                pc = 0x8009b658;
            }
            0x8009b658 => {
                pc = 0x8009b65c; continue;
                pc = 0x8009b65c;
            }
            0x8009b65c => {
                pc = 0x8009b660; continue;
                pc = 0x8009b660;
            }
            0x8009b660 => {
                c.lr = 0x8009b664; f_8009801c(c);
                pc = 0x8009b664;
            }
            0x8009b664 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009b660; continue; }
                pc = 0x8009b670;
            }
            0x8009b670 => {
                c.lr = 0x8009b674; f_8009801c(c);
                pc = 0x8009b674;
            }
            0x8009b674 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffdfffff;
                c.lr = 0x8009b67c; f_80098024(c);
                pc = 0x8009b67c;
            }
            0x8009b67c => {
                pc = 0x8009b680; continue;
                pc = 0x8009b680;
            }
            0x8009b680 => {
                c.r[3] = 0x80310000;
                c.r[31] = c.r[3].wrapping_add(0x5b0);
                pc = 0x8009b68c; continue;
                pc = 0x8009b68c;
            }
            0x8009b68c => {
                pc = 0x8009b69c; continue;
                pc = 0x8009b690;
            }
            0x8009b690 => {
                c.r[3] = c.r[31] | c.r[31];
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b69c; f_800a2ce8(c);
                pc = 0x8009b69c;
            }
            0x8009b69c => {
                c.lr = 0x8009b6a0; f_8009801c(c);
                pc = 0x8009b6a0;
            }
            0x8009b6a0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009b690; continue; }
                pc = 0x8009b6ac;
            }
            0x8009b6ac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009b628: bad pc {:#x}", pc),
        }
    }
}

/// DMAErrorHandler (0x8009b6c0, 352 bytes)
#[inline(never)]
pub fn f_8009b6c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b6c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b6c0); f_8009b6c0_b(c, 0x8009b6c0); crate::prof::exit(c, 0x8009b6c0); return; }
    }
    f_8009b6c0_b(c, 0x8009b6c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b6c0_at(c: &mut Cpu, pc: u32) { f_8009b6c0_b(c, pc) }
#[inline(always)]
fn f_8009b6c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b6c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b6c0, pc); }
        match pc {
            0x8009b6c0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x7c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x78), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x74), c.r[29]);
                if !c.crbit(6) { pc = 0x8009b6fc; continue; }
                pc = 0x8009b6dc;
            }
            0x8009b6dc => {
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[1]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[3]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[4]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let v = c.f[5]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[6]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let v = c.f[7]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[8]; c.stfd(ea, v); }
                pc = 0x8009b6fc;
            }
            0x8009b6fc => {
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[10]);
                c.r[29] = c.r[4] | c.r[4];
                c.r[3] = 0x80310000;
                c.r[31] = c.r[3].wrapping_add(0x5b0);
                c.lr = 0x8009b72c; f_800980c8(c);
                pc = 0x8009b72c;
            }
            0x8009b72c => {
                c.r[30] = c.r[3] | c.r[3];
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[3] = c.r[31].wrapping_add(0x2c);
                c.lr = 0x8009b73c; f_8009c2e0(c);
                pc = 0x8009b73c;
            }
            0x8009b73c => {
                c.r[5] = c.lwz(c.r[29].wrapping_add(0x19c));
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[31].wrapping_add(0x44);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b750; f_8009c2e0(c);
                pc = 0x8009b750;
            }
            0x8009b750 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xf00000;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009b76c; continue; }
                pc = 0x8009b75c;
            }
            0x8009b75c => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x19c));
                c.r[0] = c.r[0].rotate_left(0) & 0x200000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009b784; continue; }
                pc = 0x8009b76c;
            }
            0x8009b76c => {
                c.r[3] = c.r[31].wrapping_add(0x60);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b778; f_8009c2e0(c);
                pc = 0x8009b778;
            }
            0x8009b778 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009b780; f_8009be40(c);
                pc = 0x8009b780;
            }
            0x8009b780 => {
                c.lr = 0x8009b784; f_8009803c(c);
                pc = 0x8009b784;
            }
            0x8009b784 => {
                c.r[3] = c.r[31].wrapping_add(0x90);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b790; f_8009c2e0(c);
                pc = 0x8009b790;
            }
            0x8009b790 => {
                c.r[3] = c.r[31].wrapping_add(0xcc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b79c; f_8009c2e0(c);
                pc = 0x8009b79c;
            }
            0x8009b79c => {
                c.r[0] = c.r[30].rotate_left(0) & 0x800000;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009b7b4; continue; }
                pc = 0x8009b7a8;
            }
            0x8009b7a8 => {
                c.r[3] = c.r[31].wrapping_add(0x104);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b7b4; f_8009c2e0(c);
                pc = 0x8009b7b4;
            }
            0x8009b7b4 => {
                c.r[0] = c.r[30].rotate_left(0) & 0x400000;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009b7cc; continue; }
                pc = 0x8009b7c0;
            }
            0x8009b7c0 => {
                c.r[3] = c.r[31].wrapping_add(0x144);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b7cc; f_8009c2e0(c);
                pc = 0x8009b7cc;
            }
            0x8009b7cc => {
                c.r[0] = c.r[30].rotate_left(0) & 0x200000;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009b7e4; continue; }
                pc = 0x8009b7d8;
            }
            0x8009b7d8 => {
                c.r[3] = c.r[31].wrapping_add(0x170);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b7e4; f_8009c2e0(c);
                pc = 0x8009b7e4;
            }
            0x8009b7e4 => {
                c.r[0] = c.r[30].rotate_left(0) & 0x100000;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009b7fc; continue; }
                pc = 0x8009b7f0;
            }
            0x8009b7f0 => {
                c.r[3] = c.r[31].wrapping_add(0x190);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b7fc; f_8009c2e0(c);
                pc = 0x8009b7fc;
            }
            0x8009b7fc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009b804; f_800980d0(c);
                pc = 0x8009b804;
            }
            0x8009b804 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x84));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x7c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x78));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x74));
                c.r[1] = c.r[1].wrapping_add(0x80);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009b6c0: bad pc {:#x}", pc),
        }
    }
}

/// __OSCacheInit (0x8009b820, 244 bytes)
#[inline(never)]
pub fn f_8009b820(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b820) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b820); f_8009b820_b(c, 0x8009b820); crate::prof::exit(c, 0x8009b820); return; }
    }
    f_8009b820_b(c, 0x8009b820)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b820_at(c: &mut Cpu, pc: u32) { f_8009b820_b(c, pc) }
#[inline(always)]
fn f_8009b820_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b820;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b820, pc); }
        match pc {
            0x8009b820 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[3] = 0x80310000;
                c.r[31] = c.r[3].wrapping_add(0x5b0);
                c.lr = 0x8009b840; f_8009800c(c);
                pc = 0x8009b840;
            }
            0x8009b840 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x8000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009b85c; continue; }
                pc = 0x8009b84c;
            }
            0x8009b84c => {
                c.lr = 0x8009b850; f_8009b3f8(c);
                pc = 0x8009b850;
            }
            0x8009b850 => {
                c.r[3] = c.r[31].wrapping_add(0x1ac);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b85c; f_800a2ce8(c);
                pc = 0x8009b85c;
            }
            0x8009b85c => {
                c.lr = 0x8009b860; f_8009800c(c);
                pc = 0x8009b860;
            }
            0x8009b860 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x4000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009b87c; continue; }
                pc = 0x8009b86c;
            }
            0x8009b86c => {
                c.lr = 0x8009b870; f_8009b290(c);
                pc = 0x8009b870;
            }
            0x8009b870 => {
                c.r[3] = c.r[31].wrapping_add(0x1c8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b87c; f_800a2ce8(c);
                pc = 0x8009b87c;
            }
            0x8009b87c => {
                c.lr = 0x8009b880; f_8009801c(c);
                pc = 0x8009b880;
            }
            0x8009b880 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x80000000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009b8e0; continue; }
                pc = 0x8009b88c;
            }
            0x8009b88c => {
                c.lr = 0x8009b890; f_80097ffc(c);
                pc = 0x8009b890;
            }
            0x8009b890 => {
                c.r[30] = c.r[3] | c.r[3];
                // sync
                c.r[3] = 0x30;
                c.lr = 0x8009b8a0; f_80098004(c);
                pc = 0x8009b8a0;
            }
            0x8009b8a0 => {
                // sync
                // sync
                c.lr = 0x8009b8ac; f_8009801c(c);
                pc = 0x8009b8ac;
            }
            0x8009b8ac => {
                c.r[3] = c.r[3].rotate_left(0) & 0x7fffffff;
                c.lr = 0x8009b8b4; f_80098024(c);
                pc = 0x8009b8b4;
            }
            0x8009b8b4 => {
                // sync
                c.lr = 0x8009b8bc; f_8009b628(c);
                pc = 0x8009b8bc;
            }
            0x8009b8bc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009b8c4; f_80098004(c);
                pc = 0x8009b8c4;
            }
            0x8009b8c4 => {
                c.lr = 0x8009b8c8; f_8009801c(c);
                pc = 0x8009b8c8;
            }
            0x8009b8c8 => {
                c.r[0] = c.r[3] | 0x80000000;
                c.r[3] = c.r[0].rotate_left(0) & 0xffdfffff;
                c.lr = 0x8009b8d4; f_80098024(c);
                pc = 0x8009b8d4;
            }
            0x8009b8d4 => {
                c.r[3] = c.r[31].wrapping_add(0x1e4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b8e0; f_800a2ce8(c);
                pc = 0x8009b8e0;
            }
            0x8009b8e0 => {
                c.r[3] = 0x800a0000;
                c.r[4] = c.r[3].wrapping_add(0xffffb6c0);
                c.r[3] = 0x1;
                c.lr = 0x8009b8f0; f_8009c360(c);
                pc = 0x8009b8f0;
            }
            0x8009b8f0 => {
                c.r[3] = c.r[31].wrapping_add(0x1fc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009b8fc; f_800a2ce8(c);
                pc = 0x8009b8fc;
            }
            0x8009b8fc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009b820: bad pc {:#x}", pc),
        }
    }
}

/// __OSLoadFPUContext (0x8009b914, 292 bytes)
#[inline(never)]
pub fn f_8009b914(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009b914) { return; }
        if c.prof { crate::prof::enter(c, 0x8009b914); f_8009b914_b(c, 0x8009b914); crate::prof::exit(c, 0x8009b914); return; }
    }
    f_8009b914_b(c, 0x8009b914)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009b914_at(c: &mut Cpu, pc: u32) { f_8009b914_b(c, pc) }
#[inline(always)]
fn f_8009b914_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009b914;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009b914, pc); }
        match pc {
            0x8009b914 => {
                c.r[5] = c.lhz(c.r[4].wrapping_add(0x1a2));
                c.r[5] = c.r[5].rotate_left(0) & 0x1;
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x8009ba34; continue; }
                pc = 0x8009b920;
            }
            0x8009b920 => {
                { let ea = c.r[4].wrapping_add(0x190); c.f[0] = c.lfd(ea); }
                c.fpscr = c.f[0].to_bits() as u32;
                c.r[5] = c.mfspr(920);
                c.r[5] = c.r[5].rotate_left(3) & 0x1;
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x8009b9b4; continue; }
                pc = 0x8009b934;
            }
            0x8009b934 => {
                { let ea = c.r[4].wrapping_add(0x1c8); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x1d0); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[4].wrapping_add(0x1d8); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[4].wrapping_add(0x1e0); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[4].wrapping_add(0x1e8); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[4].wrapping_add(0x1f0); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[4].wrapping_add(0x1f8); let (x, y) = c.psq_load(ea, false, 0); c.f[6] = x; c.ps1[6] = y; }
                { let ea = c.r[4].wrapping_add(0x200); let (x, y) = c.psq_load(ea, false, 0); c.f[7] = x; c.ps1[7] = y; }
                { let ea = c.r[4].wrapping_add(0x208); let (x, y) = c.psq_load(ea, false, 0); c.f[8] = x; c.ps1[8] = y; }
                { let ea = c.r[4].wrapping_add(0x210); let (x, y) = c.psq_load(ea, false, 0); c.f[9] = x; c.ps1[9] = y; }
                { let ea = c.r[4].wrapping_add(0x218); let (x, y) = c.psq_load(ea, false, 0); c.f[10] = x; c.ps1[10] = y; }
                { let ea = c.r[4].wrapping_add(0x220); let (x, y) = c.psq_load(ea, false, 0); c.f[11] = x; c.ps1[11] = y; }
                { let ea = c.r[4].wrapping_add(0x228); let (x, y) = c.psq_load(ea, false, 0); c.f[12] = x; c.ps1[12] = y; }
                { let ea = c.r[4].wrapping_add(0x230); let (x, y) = c.psq_load(ea, false, 0); c.f[13] = x; c.ps1[13] = y; }
                { let ea = c.r[4].wrapping_add(0x238); let (x, y) = c.psq_load(ea, false, 0); c.f[14] = x; c.ps1[14] = y; }
                { let ea = c.r[4].wrapping_add(0x240); let (x, y) = c.psq_load(ea, false, 0); c.f[15] = x; c.ps1[15] = y; }
                { let ea = c.r[4].wrapping_add(0x248); let (x, y) = c.psq_load(ea, false, 0); c.f[16] = x; c.ps1[16] = y; }
                { let ea = c.r[4].wrapping_add(0x250); let (x, y) = c.psq_load(ea, false, 0); c.f[17] = x; c.ps1[17] = y; }
                { let ea = c.r[4].wrapping_add(0x258); let (x, y) = c.psq_load(ea, false, 0); c.f[18] = x; c.ps1[18] = y; }
                { let ea = c.r[4].wrapping_add(0x260); let (x, y) = c.psq_load(ea, false, 0); c.f[19] = x; c.ps1[19] = y; }
                { let ea = c.r[4].wrapping_add(0x268); let (x, y) = c.psq_load(ea, false, 0); c.f[20] = x; c.ps1[20] = y; }
                { let ea = c.r[4].wrapping_add(0x270); let (x, y) = c.psq_load(ea, false, 0); c.f[21] = x; c.ps1[21] = y; }
                { let ea = c.r[4].wrapping_add(0x278); let (x, y) = c.psq_load(ea, false, 0); c.f[22] = x; c.ps1[22] = y; }
                { let ea = c.r[4].wrapping_add(0x280); let (x, y) = c.psq_load(ea, false, 0); c.f[23] = x; c.ps1[23] = y; }
                { let ea = c.r[4].wrapping_add(0x288); let (x, y) = c.psq_load(ea, false, 0); c.f[24] = x; c.ps1[24] = y; }
                { let ea = c.r[4].wrapping_add(0x290); let (x, y) = c.psq_load(ea, false, 0); c.f[25] = x; c.ps1[25] = y; }
                { let ea = c.r[4].wrapping_add(0x298); let (x, y) = c.psq_load(ea, false, 0); c.f[26] = x; c.ps1[26] = y; }
                { let ea = c.r[4].wrapping_add(0x2a0); let (x, y) = c.psq_load(ea, false, 0); c.f[27] = x; c.ps1[27] = y; }
                { let ea = c.r[4].wrapping_add(0x2a8); let (x, y) = c.psq_load(ea, false, 0); c.f[28] = x; c.ps1[28] = y; }
                { let ea = c.r[4].wrapping_add(0x2b0); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                { let ea = c.r[4].wrapping_add(0x2b8); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[4].wrapping_add(0x2c0); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                pc = 0x8009b9b4;
            }
            0x8009b9b4 => {
                { let ea = c.r[4].wrapping_add(0x90); c.f[0] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x98); c.f[1] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xa0); c.f[2] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xa8); c.f[3] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xb0); c.f[4] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xb8); c.f[5] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xc0); c.f[6] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xc8); c.f[7] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xd0); c.f[8] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xd8); c.f[9] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xe0); c.f[10] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xe8); c.f[11] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xf0); c.f[12] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0xf8); c.f[13] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x100); c.f[14] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x108); c.f[15] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x110); c.f[16] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x118); c.f[17] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x120); c.f[18] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x128); c.f[19] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x130); c.f[20] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x138); c.f[21] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x140); c.f[22] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x148); c.f[23] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x150); c.f[24] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x158); c.f[25] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x160); c.f[26] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x168); c.f[27] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x170); c.f[28] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x178); c.f[29] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x180); c.f[30] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x188); c.f[31] = c.lfd(ea); }
                pc = 0x8009ba34;
            }
            0x8009ba34 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009b914: bad pc {:#x}", pc),
        }
    }
}

/// __OSSaveFPUContext (0x8009ba38, 296 bytes)
#[inline(never)]
pub fn f_8009ba38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ba38) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ba38); f_8009ba38_b(c, 0x8009ba38); crate::prof::exit(c, 0x8009ba38); return; }
    }
    f_8009ba38_b(c, 0x8009ba38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ba38_at(c: &mut Cpu, pc: u32) { f_8009ba38_b(c, pc) }
#[inline(always)]
fn f_8009ba38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ba38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ba38, pc); }
        match pc {
            0x8009ba38 => {
                c.r[3] = c.lhz(c.r[5].wrapping_add(0x1a2));
                c.r[3] = c.r[3] | 0x1;
                c.sth(c.r[5].wrapping_add(0x1a2), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x90); let v = c.f[0]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x98); let v = c.f[1]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xa0); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xa8); let v = c.f[3]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xb0); let v = c.f[4]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xb8); let v = c.f[5]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xc0); let v = c.f[6]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xc8); let v = c.f[7]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xd0); let v = c.f[8]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xd8); let v = c.f[9]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xe0); let v = c.f[10]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xe8); let v = c.f[11]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xf0); let v = c.f[12]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0xf8); let v = c.f[13]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x100); let v = c.f[14]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x108); let v = c.f[15]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x110); let v = c.f[16]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x118); let v = c.f[17]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x120); let v = c.f[18]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x128); let v = c.f[19]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x130); let v = c.f[20]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x138); let v = c.f[21]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x140); let v = c.f[22]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x148); let v = c.f[23]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x150); let v = c.f[24]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x158); let v = c.f[25]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x160); let v = c.f[26]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x168); let v = c.f[27]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x170); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x178); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x180); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x188); let v = c.f[31]; c.stfd(ea, v); }
                c.f[0] = f64::from_bits(c.fpscr as u64);
                { let ea = c.r[5].wrapping_add(0x190); let v = c.f[0]; c.stfd(ea, v); }
                { let ea = c.r[5].wrapping_add(0x90); c.f[0] = c.lfd(ea); }
                c.r[3] = c.mfspr(920);
                c.r[3] = c.r[3].rotate_left(3) & 0x1;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x8009bb5c; continue; }
                pc = 0x8009badc;
            }
            0x8009badc => {
                { let ea = c.r[5].wrapping_add(0x1c8); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x1d0); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x1d8); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x1e0); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x1e8); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x1f0); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x1f8); let (x, y) = (c.f[6], c.ps1[6]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x200); let (x, y) = (c.f[7], c.ps1[7]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x208); let (x, y) = (c.f[8], c.ps1[8]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x210); let (x, y) = (c.f[9], c.ps1[9]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x218); let (x, y) = (c.f[10], c.ps1[10]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x220); let (x, y) = (c.f[11], c.ps1[11]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x228); let (x, y) = (c.f[12], c.ps1[12]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x230); let (x, y) = (c.f[13], c.ps1[13]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x238); let (x, y) = (c.f[14], c.ps1[14]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x240); let (x, y) = (c.f[15], c.ps1[15]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x248); let (x, y) = (c.f[16], c.ps1[16]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x250); let (x, y) = (c.f[17], c.ps1[17]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x258); let (x, y) = (c.f[18], c.ps1[18]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x260); let (x, y) = (c.f[19], c.ps1[19]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x268); let (x, y) = (c.f[20], c.ps1[20]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x270); let (x, y) = (c.f[21], c.ps1[21]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x278); let (x, y) = (c.f[22], c.ps1[22]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x280); let (x, y) = (c.f[23], c.ps1[23]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x288); let (x, y) = (c.f[24], c.ps1[24]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x290); let (x, y) = (c.f[25], c.ps1[25]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x298); let (x, y) = (c.f[26], c.ps1[26]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x2a0); let (x, y) = (c.f[27], c.ps1[27]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x2a8); let (x, y) = (c.f[28], c.ps1[28]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x2b0); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x2b8); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[5].wrapping_add(0x2c0); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                pc = 0x8009bb5c;
            }
            0x8009bb5c => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009ba38: bad pc {:#x}", pc),
        }
    }
}

/// OSSaveFPUContext (0x8009bb60, 8 bytes)
#[inline(never)]
pub fn f_8009bb60(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009bb60) { return; }
        if c.prof { crate::prof::enter(c, 0x8009bb60); f_8009bb60_b(c, 0x8009bb60); crate::prof::exit(c, 0x8009bb60); return; }
    }
    f_8009bb60_b(c, 0x8009bb60)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009bb60_at(c: &mut Cpu, pc: u32) { f_8009bb60_b(c, pc) }
#[inline(always)]
fn f_8009bb60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009bb60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009bb60, pc); }
        match pc {
            0x8009bb60 => {
                c.r[5] = c.r[3].wrapping_add(0x0);
                f_8009ba38(c); return;
                return;
            }
            _ => panic!("f_8009bb60: bad pc {:#x}", pc),
        }
    }
}

/// OSSetCurrentContext (0x8009bb68, 92 bytes)
#[inline(never)]
pub fn f_8009bb68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009bb68) { return; }
        if c.prof { crate::prof::enter(c, 0x8009bb68); f_8009bb68_b(c, 0x8009bb68); crate::prof::exit(c, 0x8009bb68); return; }
    }
    f_8009bb68_b(c, 0x8009bb68)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009bb68_at(c: &mut Cpu, pc: u32) { f_8009bb68_b(c, pc) }
#[inline(always)]
fn f_8009bb68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009bb68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009bb68, pc); }
        match pc {
            0x8009bb68 => {
                c.r[4] = 0x80000000;
                c.stw(c.r[4].wrapping_add(0xd4), c.r[3]);
                c.r[5] = c.r[3].rotate_left(0) & 0x3fffffff;
                c.stw(c.r[4].wrapping_add(0xc0), c.r[5]);
                c.r[5] = c.lwz(c.r[4].wrapping_add(0xd8));
                c.cmps(0, c.r[5], c.r[3]);
                if !c.crbit(2) { pc = 0x8009bba0; continue; }
                pc = 0x8009bb84;
            }
            0x8009bb84 => {
                c.r[6] = c.lwz(c.r[3].wrapping_add(0x19c));
                c.r[6] = c.r[6] | 0x2000;
                c.stw(c.r[3].wrapping_add(0x19c), c.r[6]);
                c.r[6] = 0; // mfmsr
                c.r[6] = c.r[6] | 0x2;
                // mtmsr
                if true { return; }
                pc = 0x8009bba0;
            }
            0x8009bba0 => {
                c.r[6] = c.lwz(c.r[3].wrapping_add(0x19c));
                c.r[6] = c.r[6].rotate_left(0) & 0xffffdfff;
                c.stw(c.r[3].wrapping_add(0x19c), c.r[6]);
                c.r[6] = 0; // mfmsr
                c.r[6] = c.r[6].rotate_left(0) & 0xffffdfff;
                c.r[6] = c.r[6] | 0x2;
                // mtmsr
                // isync
                if true { return; }
                return;
            }
            _ => panic!("f_8009bb68: bad pc {:#x}", pc),
        }
    }
}

/// OSGetCurrentContext (0x8009bbc4, 12 bytes)
#[inline(never)]
pub fn f_8009bbc4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009bbc4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009bbc4); f_8009bbc4_b(c, 0x8009bbc4); crate::prof::exit(c, 0x8009bbc4); return; }
    }
    f_8009bbc4_b(c, 0x8009bbc4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009bbc4_at(c: &mut Cpu, pc: u32) { f_8009bbc4_b(c, pc) }
#[inline(always)]
fn f_8009bbc4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009bbc4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009bbc4, pc); }
        match pc {
            0x8009bbc4 => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xd4));
                if true { return; }
                return;
            }
            _ => panic!("f_8009bbc4: bad pc {:#x}", pc),
        }
    }
}

/// OSSaveContext (0x8009bbd0, 128 bytes)
#[inline(never)]
pub fn f_8009bbd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009bbd0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009bbd0); f_8009bbd0_b(c, 0x8009bbd0); crate::prof::exit(c, 0x8009bbd0); return; }
    }
    f_8009bbd0_b(c, 0x8009bbd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009bbd0_at(c: &mut Cpu, pc: u32) { f_8009bbd0_b(c, pc) }
#[inline(always)]
fn f_8009bbd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009bbd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009bbd0, pc); }
        match pc {
            0x8009bbd0 => {
                { let mut ea = c.r[3].wrapping_add(0x34); for n in 13..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[0] = c.mfspr(913);
                c.stw(c.r[3].wrapping_add(0x1a8), c.r[0]);
                c.r[0] = c.mfspr(914);
                c.stw(c.r[3].wrapping_add(0x1ac), c.r[0]);
                c.r[0] = c.mfspr(915);
                c.stw(c.r[3].wrapping_add(0x1b0), c.r[0]);
                c.r[0] = c.mfspr(916);
                c.stw(c.r[3].wrapping_add(0x1b4), c.r[0]);
                c.r[0] = c.mfspr(917);
                c.stw(c.r[3].wrapping_add(0x1b8), c.r[0]);
                c.r[0] = c.mfspr(918);
                c.stw(c.r[3].wrapping_add(0x1bc), c.r[0]);
                c.r[0] = c.mfspr(919);
                c.stw(c.r[3].wrapping_add(0x1c0), c.r[0]);
                c.r[0] = c.cr;
                c.stw(c.r[3].wrapping_add(0x80), c.r[0]);
                c.r[0] = c.mfspr(8);
                c.stw(c.r[3].wrapping_add(0x84), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x198), c.r[0]);
                c.r[0] = 0; // mfmsr
                c.stw(c.r[3].wrapping_add(0x19c), c.r[0]);
                c.r[0] = c.mfspr(9);
                c.stw(c.r[3].wrapping_add(0x88), c.r[0]);
                c.r[0] = c.mfspr(1);
                c.stw(c.r[3].wrapping_add(0x8c), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[1]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[2]);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8009bbd0: bad pc {:#x}", pc),
        }
    }
}

/// OSLoadContext (0x8009bc50, 216 bytes)
#[inline(never)]
pub fn f_8009bc50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009bc50) { return; }
        if c.prof { crate::prof::enter(c, 0x8009bc50); f_8009bc50_b(c, 0x8009bc50); crate::prof::exit(c, 0x8009bc50); return; }
    }
    f_8009bc50_b(c, 0x8009bc50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009bc50_at(c: &mut Cpu, pc: u32) { f_8009bc50_b(c, pc) }
#[inline(always)]
fn f_8009bc50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009bc50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009bc50, pc); }
        match pc {
            0x8009bc50 => {
                c.r[4] = 0x800a0000;
                c.r[6] = c.lwz(c.r[3].wrapping_add(0x198));
                c.r[5] = c.r[4].wrapping_add(0xffffdf3c);
                c.cmpu(0, c.r[6], c.r[5]);
                if !c.crbit(1) { pc = 0x8009bc78; continue; }
                pc = 0x8009bc64;
            }
            0x8009bc64 => {
                c.r[4] = 0x800a0000;
                c.r[0] = c.r[4].wrapping_add(0xffffdf48);
                c.cmpu(0, c.r[6], c.r[0]);
                if !c.crbit(0) { pc = 0x8009bc78; continue; }
                pc = 0x8009bc74;
            }
            0x8009bc74 => {
                c.stw(c.r[3].wrapping_add(0x198), c.r[5]);
                pc = 0x8009bc78;
            }
            0x8009bc78 => {
                c.r[0] = c.lwz(c.r[3]);
                c.r[1] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[2] = c.lwz(c.r[3].wrapping_add(0x8));
                c.r[4] = c.lhz(c.r[3].wrapping_add(0x1a2));
                c.r[5] = c.r[4].rotate_left(0) & 0x2;
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x8009bca0; continue; }
                pc = 0x8009bc90;
            }
            0x8009bc90 => {
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffffd;
                c.sth(c.r[3].wrapping_add(0x1a2), c.r[4]);
                { let mut ea = c.r[3].wrapping_add(0x14); for n in 5..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                pc = 0x8009bca4; continue;
                pc = 0x8009bca0;
            }
            0x8009bca0 => {
                { let mut ea = c.r[3].wrapping_add(0x34); for n in 13..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                pc = 0x8009bca4;
            }
            0x8009bca4 => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x1a8));
                c.mtspr(913, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x1ac));
                c.mtspr(914, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x1b0));
                c.mtspr(915, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x1b4));
                c.mtspr(916, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x1b8));
                c.mtspr(917, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x1bc));
                c.mtspr(918, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x1c0));
                c.mtspr(919, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x80));
                c.cr = (c.cr & 0x0) | (c.r[4] & 0xffffffff);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x84));
                c.mtspr(8, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x88));
                c.mtspr(9, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x8c));
                c.mtspr(1, c.r[4]);
                c.r[4] = 0; // mfmsr
                c.r[4] = c.r[4].rotate_left(0) & 0xffff7fff;
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffffd;
                // mtmsr
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x198));
                c.mtspr(26, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x19c));
                c.mtspr(27, c.r[4]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x10));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                // rfi
                f_8009bd28(c); return;
                return;
            }
            _ => panic!("f_8009bc50: bad pc {:#x}", pc),
        }
    }
}

/// OSGetStackPointer (0x8009bd28, 8 bytes)
#[inline(never)]
pub fn f_8009bd28(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009bd28) { return; }
        if c.prof { crate::prof::enter(c, 0x8009bd28); f_8009bd28_b(c, 0x8009bd28); crate::prof::exit(c, 0x8009bd28); return; }
    }
    f_8009bd28_b(c, 0x8009bd28)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009bd28_at(c: &mut Cpu, pc: u32) { f_8009bd28_b(c, pc) }
#[inline(always)]
fn f_8009bd28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009bd28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009bd28, pc); }
        match pc {
            0x8009bd28 => {
                c.r[3] = c.r[1] | c.r[1];
                if true { return; }
                return;
            }
            _ => panic!("f_8009bd28: bad pc {:#x}", pc),
        }
    }
}

/// OSSwitchFiber (0x8009bd30, 48 bytes)
#[inline(never)]
pub fn f_8009bd30(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009bd30) { return; }
        if c.prof { crate::prof::enter(c, 0x8009bd30); f_8009bd30_b(c, 0x8009bd30); crate::prof::exit(c, 0x8009bd30); return; }
    }
    f_8009bd30_b(c, 0x8009bd30)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009bd30_at(c: &mut Cpu, pc: u32) { f_8009bd30_b(c, pc) }
#[inline(always)]
fn f_8009bd30_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009bd30;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009bd30, pc); }
        match pc {
            0x8009bd30 => {
                c.r[0] = c.mfspr(8);
                c.r[5] = c.r[1] | c.r[1];
                { let ea = c.r[4].wrapping_add(0xfffffff8); c.stw(ea, c.r[5]); c.r[4] = ea; }
                c.r[1] = c.r[4] | c.r[4];
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                c.mtspr(8, c.r[3]);
                if true { let t = c.lr; c.lr = 0x8009bd4c; (c.dispatch)(c, t); }
                pc = 0x8009bd4c;
            }
            0x8009bd4c => {
                c.r[5] = c.lwz(c.r[1]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[5] | c.r[5];
                if true { return; }
                return;
            }
            _ => panic!("f_8009bd30: bad pc {:#x}", pc),
        }
    }
}

/// OSClearContext (0x8009bd60, 36 bytes)
#[inline(never)]
pub fn f_8009bd60(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009bd60) { return; }
        if c.prof { crate::prof::enter(c, 0x8009bd60); f_8009bd60_b(c, 0x8009bd60); crate::prof::exit(c, 0x8009bd60); return; }
    }
    f_8009bd60_b(c, 0x8009bd60)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009bd60_at(c: &mut Cpu, pc: u32) { f_8009bd60_b(c, pc) }
#[inline(always)]
fn f_8009bd60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009bd60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009bd60, pc); }
        match pc {
            0x8009bd60 => {
                c.r[5] = 0x0;
                c.sth(c.r[3].wrapping_add(0x1a0), c.r[5]);
                c.r[4] = 0x80000000;
                c.sth(c.r[3].wrapping_add(0x1a2), c.r[5]);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xd8));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x8009bd80; continue; }
                pc = 0x8009bd7c;
            }
            0x8009bd7c => {
                c.stw(c.r[4].wrapping_add(0xd8), c.r[5]);
                pc = 0x8009bd80;
            }
            0x8009bd80 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009bd60: bad pc {:#x}", pc),
        }
    }
}

/// OSInitContext (0x8009bd84, 188 bytes)
#[inline(never)]
pub fn f_8009bd84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009bd84) { return; }
        if c.prof { crate::prof::enter(c, 0x8009bd84); f_8009bd84_b(c, 0x8009bd84); crate::prof::exit(c, 0x8009bd84); return; }
    }
    f_8009bd84_b(c, 0x8009bd84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009bd84_at(c: &mut Cpu, pc: u32) { f_8009bd84_b(c, pc) }
#[inline(always)]
fn f_8009bd84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009bd84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009bd84, pc); }
        match pc {
            0x8009bd84 => {
                c.stw(c.r[3].wrapping_add(0x198), c.r[4]);
                c.stw(c.r[3].wrapping_add(0x4), c.r[5]);
                c.r[11] = 0x0;
                c.r[11] = c.r[11] | 0x9032;
                c.stw(c.r[3].wrapping_add(0x19c), c.r[11]);
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x80), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x8c), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[2]);
                c.stw(c.r[3].wrapping_add(0x34), c.r[13]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x18), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x20), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x28), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x2c), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x30), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x38), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x3c), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x40), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x44), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x48), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x4c), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x50), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x54), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x58), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x5c), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x60), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x64), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x68), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x6c), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x70), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x74), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x78), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x7c), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1a4), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1a8), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1ac), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1b0), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1b4), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1b8), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1bc), c.r[0]);
                c.stw(c.r[3].wrapping_add(0x1c0), c.r[0]);
                f_8009bd60(c); return;
                return;
            }
            _ => panic!("f_8009bd84: bad pc {:#x}", pc),
        }
    }
}

/// OSDumpContext (0x8009be40, 680 bytes)
#[inline(never)]
pub fn f_8009be40(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009be40) { return; }
        if c.prof { crate::prof::enter(c, 0x8009be40); f_8009be40_b(c, 0x8009be40); crate::prof::exit(c, 0x8009be40); return; }
    }
    f_8009be40_b(c, 0x8009be40)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009be40_at(c: &mut Cpu, pc: u32) { f_8009be40_b(c, pc) }
#[inline(always)]
fn f_8009be40_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009be40;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009be40, pc); }
        match pc {
            0x8009be40 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffd08); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x2dc); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = 0x80310000;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[31] = c.r[3].wrapping_add(0x7e0);
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8009be6c; f_8009c2e0(c);
                pc = 0x8009be6c;
            }
            0x8009be6c => {
                c.r[25] = 0x0;
                c.r[0] = c.r[25].rotate_left(2) & 0xfffffffc;
                c.r[27] = c.r[28].wrapping_add(c.r[0]);
                pc = 0x8009be7c; continue;
                pc = 0x8009be7c;
            }
            0x8009be7c => {
                pc = 0x8009be80; continue;
                pc = 0x8009be80;
            }
            0x8009be80 => {
                pc = 0x8009be84; continue;
                pc = 0x8009be84;
            }
            0x8009be84 => {
                c.r[8] = c.lwz(c.r[27].wrapping_add(0x40));
                c.r[4] = c.r[25] | c.r[25];
                c.r[5] = c.lwz(c.r[27]);
                c.r[3] = c.r[31].wrapping_add(0x44);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[6] = c.r[5] | c.r[5];
                c.r[9] = c.r[8] | c.r[8];
                c.r[7] = c.r[25].wrapping_add(0x10);
                c.lr = 0x8009bea8; f_8009c2e0(c);
                pc = 0x8009bea8;
            }
            0x8009bea8 => {
                c.r[27] = c.r[27].wrapping_add(0x4);
                c.r[25] = c.r[25].wrapping_add(0x1);
                c.cmpu(0, c.r[25], 0x10);
                if c.crbit(0) { pc = 0x8009be84; continue; }
                pc = 0x8009beb8;
            }
            0x8009beb8 => {
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x84));
                c.r[3] = c.r[31].wrapping_add(0x74);
                c.r[5] = c.lwz(c.r[28].wrapping_add(0x80));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009becc; f_8009c2e0(c);
                pc = 0x8009becc;
            }
            0x8009becc => {
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x198));
                c.r[3] = c.r[31].wrapping_add(0xa4);
                c.r[5] = c.lwz(c.r[28].wrapping_add(0x19c));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009bee0; f_8009c2e0(c);
                pc = 0x8009bee0;
            }
            0x8009bee0 => {
                c.r[3] = c.r[31].wrapping_add(0xd4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009beec; f_8009c2e0(c);
                pc = 0x8009beec;
            }
            0x8009beec => {
                c.r[25] = 0x0;
                c.r[0] = c.r[25].rotate_left(2) & 0xfffffffc;
                c.r[27] = c.r[28].wrapping_add(c.r[0]);
                pc = 0x8009befc; continue;
                pc = 0x8009befc;
            }
            0x8009befc => {
                pc = 0x8009bf00; continue;
                pc = 0x8009bf00;
            }
            0x8009bf00 => {
                pc = 0x8009bf04; continue;
                pc = 0x8009bf04;
            }
            0x8009bf04 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x1a4));
                c.r[4] = c.r[25] | c.r[25];
                c.r[7] = c.lwz(c.r[27].wrapping_add(0x1b4));
                c.r[3] = c.r[31].wrapping_add(0xe8);
                c.r[6] = c.r[25].wrapping_add(0x4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009bf20; f_8009c2e0(c);
                pc = 0x8009bf20;
            }
            0x8009bf20 => {
                c.r[27] = c.r[27].wrapping_add(0x4);
                c.r[25] = c.r[25].wrapping_add(0x1);
                c.cmpu(0, c.r[25], 0x4);
                if c.crbit(0) { pc = 0x8009bf04; continue; }
                pc = 0x8009bf30;
            }
            0x8009bf30 => {
                c.r[0] = c.lhz(c.r[28].wrapping_add(0x1a2));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009c078; continue; }
                pc = 0x8009bf40;
            }
            0x8009bf40 => {
                c.lr = 0x8009bf44; f_8009df3c(c);
                pc = 0x8009bf44;
            }
            0x8009bf44 => {
                c.r[6] = 0x80000000;
                c.r[0] = c.lwz(c.r[6].wrapping_add(0xd4));
                c.r[5] = 0x0;
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.sth(c.r[1].wrapping_add(0x1b0), c.r[5]);
                c.r[30] = c.r[0] | c.r[0];
                c.r[29] = c.r[3] | c.r[3];
                c.sth(c.r[1].wrapping_add(0x1b2), c.r[5]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0xd8));
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(2) { pc = 0x8009bf74; continue; }
                pc = 0x8009bf70;
            }
            0x8009bf70 => {
                c.stw(c.r[6].wrapping_add(0xd8), c.r[5]);
                pc = 0x8009bf74;
            }
            0x8009bf74 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x8009bf7c; f_8009bb68(c);
                pc = 0x8009bf7c;
            }
            0x8009bf7c => {
                c.r[3] = c.r[31].wrapping_add(0x10c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009bf88; f_8009c2e0(c);
                pc = 0x8009bf88;
            }
            0x8009bf88 => {
                c.r[25] = 0x0;
                c.r[0] = c.r[25].rotate_left(3) & 0xfffffff8;
                c.r[26] = c.r[28].wrapping_add(c.r[0]);
                pc = 0x8009bf98; continue;
                pc = 0x8009bf98;
            }
            0x8009bf98 => {
                pc = 0x8009bf9c; continue;
                pc = 0x8009bf9c;
            }
            0x8009bf9c => {
                pc = 0x8009bfa0; continue;
                pc = 0x8009bfa0;
            }
            0x8009bfa0 => {
                { let ea = c.r[26].wrapping_add(0x98); c.f[1] = c.lfd(ea); }
                c.lr = 0x8009bfa8; f_800c46b0(c);
                pc = 0x8009bfa8;
            }
            0x8009bfa8 => {
                c.r[27] = c.r[3] | c.r[3];
                { let ea = c.r[26].wrapping_add(0x90); c.f[1] = c.lfd(ea); }
                c.lr = 0x8009bfb4; f_800c46b0(c);
                pc = 0x8009bfb4;
            }
            0x8009bfb4 => {
                c.r[5] = c.r[3] | c.r[3];
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = c.r[25] | c.r[25];
                c.r[7] = c.r[27] | c.r[27];
                c.r[3] = c.r[31].wrapping_add(0x120);
                c.r[6] = c.r[25].wrapping_add(0x1);
                c.lr = 0x8009bfd0; f_8009c2e0(c);
                pc = 0x8009bfd0;
            }
            0x8009bfd0 => {
                c.r[26] = c.r[26].wrapping_add(0x10);
                c.r[25] = c.r[25].wrapping_add(0x2);
                c.cmpu(0, c.r[25], 0x20);
                if c.crbit(0) { pc = 0x8009bfa0; continue; }
                pc = 0x8009bfe0;
            }
            0x8009bfe0 => {
                c.r[3] = c.r[31].wrapping_add(0x13c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009bfec; f_8009c2e0(c);
                pc = 0x8009bfec;
            }
            0x8009bfec => {
                c.r[25] = 0x0;
                c.r[0] = c.r[25].rotate_left(3) & 0xfffffff8;
                c.r[26] = c.r[28].wrapping_add(c.r[0]);
                pc = 0x8009bffc; continue;
                pc = 0x8009bffc;
            }
            0x8009bffc => {
                pc = 0x8009c000; continue;
                pc = 0x8009c000;
            }
            0x8009c000 => {
                pc = 0x8009c004; continue;
                pc = 0x8009c004;
            }
            0x8009c004 => {
                { let ea = c.r[26].wrapping_add(0x1d0); c.f[1] = c.lfd(ea); }
                c.lr = 0x8009c00c; f_800c46b0(c);
                pc = 0x8009c00c;
            }
            0x8009c00c => {
                c.r[27] = c.r[3] | c.r[3];
                { let ea = c.r[26].wrapping_add(0x1c8); c.f[1] = c.lfd(ea); }
                c.lr = 0x8009c018; f_800c46b0(c);
                pc = 0x8009c018;
            }
            0x8009c018 => {
                c.r[5] = c.r[3] | c.r[3];
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = c.r[25] | c.r[25];
                c.r[7] = c.r[27] | c.r[27];
                c.r[3] = c.r[31].wrapping_add(0x150);
                c.r[6] = c.r[25].wrapping_add(0x1);
                c.lr = 0x8009c034; f_8009c2e0(c);
                pc = 0x8009c034;
            }
            0x8009c034 => {
                c.r[26] = c.r[26].wrapping_add(0x10);
                c.r[25] = c.r[25].wrapping_add(0x2);
                c.cmpu(0, c.r[25], 0x20);
                if c.crbit(0) { pc = 0x8009c004; continue; }
                pc = 0x8009c044;
            }
            0x8009c044 => {
                c.r[5] = 0x0;
                c.sth(c.r[1].wrapping_add(0x1b0), c.r[5]);
                c.r[3] = 0x80000000;
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.sth(c.r[1].wrapping_add(0x1b2), c.r[5]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xd8));
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(2) { pc = 0x8009c068; continue; }
                pc = 0x8009c064;
            }
            0x8009c064 => {
                c.stw(c.r[3].wrapping_add(0xd8), c.r[5]);
                pc = 0x8009c068;
            }
            0x8009c068 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009c070; f_8009bb68(c);
                pc = 0x8009c070;
            }
            0x8009c070 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009c078; f_8009df64(c);
                pc = 0x8009c078;
            }
            0x8009c078 => {
                c.r[3] = c.r[31].wrapping_add(0x170);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009c084; f_8009c2e0(c);
                pc = 0x8009c084;
            }
            0x8009c084 => {
                c.r[25] = c.lwz(c.r[28].wrapping_add(0x4));
                c.r[26] = 0x0;
                pc = 0x8009c090; continue;
                pc = 0x8009c090;
            }
            0x8009c090 => {
                pc = 0x8009c094; continue;
                pc = 0x8009c094;
            }
            0x8009c094 => {
                pc = 0x8009c0b4; continue;
                pc = 0x8009c098;
            }
            0x8009c098 => {
                c.r[5] = c.lwz(c.r[25]);
                c.r[4] = c.r[25] | c.r[25];
                c.r[6] = c.lwz(c.r[25].wrapping_add(0x4));
                c.r[3] = c.r[31].wrapping_add(0x198);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009c0b0; f_8009c2e0(c);
                pc = 0x8009c0b0;
            }
            0x8009c0b0 => {
                c.r[25] = c.lwz(c.r[25]);
                pc = 0x8009c0b4;
            }
            0x8009c0b4 => {
                c.cmpu(0, c.r[25], 0x0);
                if c.crbit(2) { pc = 0x8009c0d4; continue; }
                pc = 0x8009c0bc;
            }
            0x8009c0bc => {
                c.r[0] = c.r[25].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x8009c0d4; continue; }
                pc = 0x8009c0c8;
            }
            0x8009c0c8 => {
                c.cmpu(0, c.r[26], 0x10);
                c.r[26] = c.r[26].wrapping_add(0x1);
                if c.crbit(0) { pc = 0x8009c098; continue; }
                pc = 0x8009c0d4;
            }
            0x8009c0d4 => {
                { let mut ea = c.r[1].wrapping_add(0x2dc); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2fc));
                c.r[1] = c.r[1].wrapping_add(0x2f8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009be40: bad pc {:#x}", pc),
        }
    }
}

/// OSSwitchFPUContext (0x8009c0e8, 132 bytes)
#[inline(never)]
pub fn f_8009c0e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009c0e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009c0e8); f_8009c0e8_b(c, 0x8009c0e8); crate::prof::exit(c, 0x8009c0e8); return; }
    }
    f_8009c0e8_b(c, 0x8009c0e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009c0e8_at(c: &mut Cpu, pc: u32) { f_8009c0e8_b(c, pc) }
#[inline(always)]
fn f_8009c0e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009c0e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009c0e8, pc); }
        match pc {
            0x8009c0e8 => {
                c.r[5] = 0; // mfmsr
                c.r[5] = c.r[5] | 0x2000;
                // mtmsr
                // isync
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x19c));
                c.r[5] = c.r[5] | 0x2000;
                c.mtspr(27, c.r[5]);
                c.r[3] = 0x80000000;
                c.r[5] = c.lwz(c.r[3].wrapping_add(0xd8));
                c.stw(c.r[3].wrapping_add(0xd8), c.r[4]);
                c.cmps(0, c.r[5], c.r[4]);
                if c.crbit(2) { pc = 0x8009c128; continue; }
                pc = 0x8009c118;
            }
            0x8009c118 => {
                c.cmps(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x8009c124; continue; }
                pc = 0x8009c120;
            }
            0x8009c120 => {
                c.lr = 0x8009c124; f_8009ba38(c);
                pc = 0x8009c124;
            }
            0x8009c124 => {
                c.lr = 0x8009c128; f_8009b914(c);
                pc = 0x8009c128;
            }
            0x8009c128 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x80));
                c.cr = (c.cr & 0x0) | (c.r[3] & 0xffffffff);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x84));
                c.mtspr(8, c.r[3]);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x198));
                c.mtspr(26, c.r[3]);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x88));
                c.mtspr(9, c.r[3]);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x8c));
                c.mtspr(1, c.r[3]);
                c.r[3] = c.lhz(c.r[4].wrapping_add(0x1a2));
                c.r[3] = c.r[3].rotate_left(0) & 0xfffffffd;
                c.sth(c.r[4].wrapping_add(0x1a2), c.r[3]);
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x14));
                c.r[3] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x10));
                // rfi
                f_8009c16c(c); return;
                return;
            }
            _ => panic!("f_8009c0e8: bad pc {:#x}", pc),
        }
    }
}

/// __OSContextInit (0x8009c16c, 72 bytes)
#[inline(never)]
pub fn f_8009c16c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009c16c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009c16c); f_8009c16c_b(c, 0x8009c16c); crate::prof::exit(c, 0x8009c16c); return; }
    }
    f_8009c16c_b(c, 0x8009c16c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009c16c_at(c: &mut Cpu, pc: u32) { f_8009c16c_b(c, pc) }
#[inline(always)]
fn f_8009c16c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009c16c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009c16c, pc); }
        match pc {
            0x8009c16c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[3] = 0x800a0000;
                c.r[4] = c.r[3].wrapping_add(0xffffc0e8);
                c.r[3] = 0x7;
                c.lr = 0x8009c188; f_8009a0c4(c);
                pc = 0x8009c188;
            }
            0x8009c188 => {
                c.r[0] = 0x0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[4] = 0x80000000;
                c.r[3] = 0x80310000;
                c.stw(c.r[4].wrapping_add(0xd8), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x994);
                c.lr = 0x8009c1a4; f_800a2ce8(c);
                pc = 0x8009c1a4;
            }
            0x8009c1a4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009c16c: bad pc {:#x}", pc),
        }
    }
}

/// OSFillFPUContext (0x8009c1b4, 300 bytes)
#[inline(never)]
pub fn f_8009c1b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009c1b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009c1b4); f_8009c1b4_b(c, 0x8009c1b4); crate::prof::exit(c, 0x8009c1b4); return; }
    }
    f_8009c1b4_b(c, 0x8009c1b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009c1b4_at(c: &mut Cpu, pc: u32) { f_8009c1b4_b(c, pc) }
#[inline(always)]
fn f_8009c1b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009c1b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009c1b4, pc); }
        match pc {
            0x8009c1b4 => {
                c.r[5] = 0; // mfmsr
                c.r[5] = c.r[5] | 0x2000;
                // mtmsr
                // isync
                { let ea = c.r[3].wrapping_add(0x90); let v = c.f[0]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[1]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xa8); let v = c.f[3]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xb0); let v = c.f[4]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xb8); let v = c.f[5]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xc0); let v = c.f[6]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xc8); let v = c.f[7]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xd0); let v = c.f[8]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xd8); let v = c.f[9]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xe0); let v = c.f[10]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xe8); let v = c.f[11]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xf0); let v = c.f[12]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0xf8); let v = c.f[13]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x100); let v = c.f[14]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x108); let v = c.f[15]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x110); let v = c.f[16]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x118); let v = c.f[17]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x120); let v = c.f[18]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x128); let v = c.f[19]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x130); let v = c.f[20]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x138); let v = c.f[21]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x140); let v = c.f[22]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x148); let v = c.f[23]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x150); let v = c.f[24]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x158); let v = c.f[25]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x160); let v = c.f[26]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x168); let v = c.f[27]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x170); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x178); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x180); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x188); let v = c.f[31]; c.stfd(ea, v); }
                c.f[0] = f64::from_bits(c.fpscr as u64);
                { let ea = c.r[3].wrapping_add(0x190); let v = c.f[0]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x90); c.f[0] = c.lfd(ea); }
                c.r[5] = c.mfspr(920);
                c.r[5] = c.r[5].rotate_left(3) & 0x1;
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x8009c2dc; continue; }
                pc = 0x8009c25c;
            }
            0x8009c25c => {
                { let ea = c.r[3].wrapping_add(0x1c8); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x1d0); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x1d8); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x1e0); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x1e8); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x1f0); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x1f8); let (x, y) = (c.f[6], c.ps1[6]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x200); let (x, y) = (c.f[7], c.ps1[7]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x208); let (x, y) = (c.f[8], c.ps1[8]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x210); let (x, y) = (c.f[9], c.ps1[9]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x218); let (x, y) = (c.f[10], c.ps1[10]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x220); let (x, y) = (c.f[11], c.ps1[11]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x228); let (x, y) = (c.f[12], c.ps1[12]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x230); let (x, y) = (c.f[13], c.ps1[13]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x238); let (x, y) = (c.f[14], c.ps1[14]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x240); let (x, y) = (c.f[15], c.ps1[15]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x248); let (x, y) = (c.f[16], c.ps1[16]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x250); let (x, y) = (c.f[17], c.ps1[17]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x258); let (x, y) = (c.f[18], c.ps1[18]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x260); let (x, y) = (c.f[19], c.ps1[19]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x268); let (x, y) = (c.f[20], c.ps1[20]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x270); let (x, y) = (c.f[21], c.ps1[21]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x278); let (x, y) = (c.f[22], c.ps1[22]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x280); let (x, y) = (c.f[23], c.ps1[23]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x288); let (x, y) = (c.f[24], c.ps1[24]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x290); let (x, y) = (c.f[25], c.ps1[25]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x298); let (x, y) = (c.f[26], c.ps1[26]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x2a0); let (x, y) = (c.f[27], c.ps1[27]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x2a8); let (x, y) = (c.f[28], c.ps1[28]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x2b0); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x2b8); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3].wrapping_add(0x2c0); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                pc = 0x8009c2dc;
            }
            0x8009c2dc => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009c1b4: bad pc {:#x}", pc),
        }
    }
}

/// OSReport (0x8009c2e0, 128 bytes)
#[inline(never)]
pub fn f_8009c2e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009c2e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009c2e0); f_8009c2e0_b(c, 0x8009c2e0); crate::prof::exit(c, 0x8009c2e0); return; }
    }
    f_8009c2e0_b(c, 0x8009c2e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009c2e0_at(c: &mut Cpu, pc: u32) { f_8009c2e0_b(c, pc) }
#[inline(always)]
fn f_8009c2e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009c2e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009c2e0, pc); }
        match pc {
            0x8009c2e0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffff88); c.stw(ea, c.r[1]); c.r[1] = ea; }
                if !c.crbit(6) { pc = 0x8009c310; continue; }
                pc = 0x8009c2f0;
            }
            0x8009c2f0 => {
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[1]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[3]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[4]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let v = c.f[5]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[6]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let v = c.f[7]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[8]; c.stfd(ea, v); }
                pc = 0x8009c310;
            }
            0x8009c310 => {
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                c.r[0] = 0x1000000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                c.r[4] = c.r[1].wrapping_add(0x6c);
                c.stw(c.r[1].wrapping_add(0x10), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x6c), c.r[0]);
                c.r[0] = c.r[1].wrapping_add(0x80);
                c.stw(c.r[1].wrapping_add(0x70), c.r[0]);
                c.r[0] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                c.lr = 0x8009c350; f_800c8678(c);
                pc = 0x8009c350;
            }
            0x8009c350 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x7c));
                c.r[1] = c.r[1].wrapping_add(0x78);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009c2e0: bad pc {:#x}", pc),
        }
    }
}

/// OSSetErrorHandler (0x8009c360, 536 bytes)
#[inline(never)]
pub fn f_8009c360(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009c360) { return; }
        if c.prof { crate::prof::enter(c, 0x8009c360); f_8009c360_b(c, 0x8009c360); crate::prof::exit(c, 0x8009c360); return; }
    }
    f_8009c360_b(c, 0x8009c360)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009c360_at(c: &mut Cpu, pc: u32) { f_8009c360_b(c, pc) }
#[inline(always)]
fn f_8009c360_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009c360;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009c360, pc); }
        match pc {
            0x8009c360 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[28]);
                c.r[28] = c.r[4].wrapping_add(0x0);
                c.lr = 0x8009c388; f_8009df3c(c);
                pc = 0x8009c388;
            }
            0x8009c388 => {
                c.r[4] = 0x80400000;
                c.r[5] = c.r[29].rotate_left(2) & 0x3fffc;
                c.r[0] = c.r[4].wrapping_add(0xffffb4f0);
                c.r[6] = c.r[29].rotate_left(0) & 0xffff;
                c.r[4] = c.r[0].wrapping_add(c.r[5]);
                c.r[30] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[6], 0x10);
                c.r[29] = c.r[3] | c.r[3];
                c.stw(c.r[4], c.r[28]);
                if !c.crbit(2) { pc = 0x8009c54c; continue; }
                pc = 0x8009c3b0;
            }
            0x8009c3b0 => {
                c.lr = 0x8009c3b4; f_80097ffc(c);
                pc = 0x8009c3b4;
            }
            0x8009c3b4 => {
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[31] | 0x2000;
                c.lr = 0x8009c3c0; f_80098004(c);
                pc = 0x8009c3c0;
            }
            0x8009c3c0 => {
                c.lr = 0x8009c3c4; f_80098080(c);
                pc = 0x8009c3c4;
            }
            0x8009c3c4 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x8009c4e0; continue; }
                pc = 0x8009c3cc;
            }
            0x8009c3cc => {
                c.r[5] = 0x80000000;
                c.r[4] = 0x60060000;
                c.r[6] = c.lwz(c.r[5].wrapping_add(0xdc));
                c.r[4] = c.r[4].wrapping_add(0xfffff8ff);
                pc = 0x8009c4c4; continue;
                pc = 0x8009c3e0;
            }
            0x8009c3e0 => {
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x19c));
                c.r[0] = c.r[0] | 0x900;
                c.stw(c.r[6].wrapping_add(0x19c), c.r[0]);
                c.r[5] = c.lhz(c.r[6].wrapping_add(0x1a2));
                c.r[0] = c.r[5].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009c4a0; continue; }
                pc = 0x8009c3f8;
            }
            0x8009c3f8 => {
                c.r[5] = c.r[5] | 0x1;
                c.r[0] = 0x4;
                c.sth(c.r[6].wrapping_add(0x1a2), c.r[5]);
                c.mtspr(9, c.r[0]);
                c.r[5] = c.r[6].wrapping_add(0x0);
                pc = 0x8009c40c;
            }
            0x8009c40c => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[5].wrapping_add(0x94), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x90), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1cc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1c8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x9c), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x98), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1d4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1d0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xa4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xa0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1dc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1d8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xac), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xa8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1e4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1e0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xb4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xb0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1ec), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1e8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xbc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xb8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1f4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1f0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xc4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xc0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1fc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1f8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xcc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xc8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x204), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x200), c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0x40);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009c40c; continue; }
                pc = 0x8009c498;
            }
            0x8009c498 => {
                c.r[0] = 0x4;
                c.stw(c.r[6].wrapping_add(0x194), c.r[0]);
                pc = 0x8009c4a0;
            }
            0x8009c4a0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8170));
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x194));
                c.r[0] = c.r[0].rotate_left(0) & 0xf8;
                c.r[0] = c.r[5] | c.r[0];
                c.stw(c.r[6].wrapping_add(0x194), c.r[0]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x194));
                c.r[0] = c.r[0] & c.r[4];
                c.stw(c.r[6].wrapping_add(0x194), c.r[0]);
                c.r[6] = c.lwz(c.r[6].wrapping_add(0x2fc));
                pc = 0x8009c4c4;
            }
            0x8009c4c4 => {
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x8009c3e0; continue; }
                pc = 0x8009c4cc;
            }
            0x8009c4cc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8170));
                c.r[31] = c.r[31] | 0x900;
                c.r[0] = c.r[0].rotate_left(0) & 0xf8;
                c.r[3] = c.r[3] | c.r[0];
                pc = 0x8009c534; continue;
                pc = 0x8009c4e0;
            }
            0x8009c4e0 => {
                c.r[5] = 0x80000000;
                c.r[4] = 0x60060000;
                c.r[6] = c.lwz(c.r[5].wrapping_add(0xdc));
                c.r[4] = c.r[4].wrapping_add(0xfffff8ff);
                c.r[5] = 0xfffff6ff;
                pc = 0x8009c520; continue;
                pc = 0x8009c4f8;
            }
            0x8009c4f8 => {
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x19c));
                c.r[0] = c.r[0] & c.r[5];
                c.stw(c.r[6].wrapping_add(0x19c), c.r[0]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x194));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff07;
                c.stw(c.r[6].wrapping_add(0x194), c.r[0]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x194));
                c.r[0] = c.r[0] & c.r[4];
                c.stw(c.r[6].wrapping_add(0x194), c.r[0]);
                c.r[6] = c.lwz(c.r[6].wrapping_add(0x2fc));
                pc = 0x8009c520;
            }
            0x8009c520 => {
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x8009c4f8; continue; }
                pc = 0x8009c528;
            }
            0x8009c528 => {
                c.r[0] = 0xfffff6ff;
                c.r[3] = c.r[3].rotate_left(0) & 0xffffff07;
                c.r[31] = c.r[31] & c.r[0];
                pc = 0x8009c534;
            }
            0x8009c534 => {
                c.r[4] = 0x60060000;
                c.r[0] = c.r[4].wrapping_add(0xfffff8ff);
                c.r[3] = c.r[3] & c.r[0];
                c.lr = 0x8009c544; f_800980a0(c);
                pc = 0x8009c544;
            }
            0x8009c544 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8009c54c; f_80098004(c);
                pc = 0x8009c54c;
            }
            0x8009c54c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009c554; f_8009df64(c);
                pc = 0x8009c554;
            }
            0x8009c554 => {
                c.r[3] = c.r[30] | c.r[30];
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
            _ => panic!("f_8009c360: bad pc {:#x}", pc),
        }
    }
}

/// __OSUnhandledException (0x8009c578, 744 bytes)
#[inline(never)]
pub fn f_8009c578(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009c578) { return; }
        if c.prof { crate::prof::enter(c, 0x8009c578); f_8009c578_b(c, 0x8009c578); crate::prof::exit(c, 0x8009c578); return; }
    }
    f_8009c578_b(c, 0x8009c578)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009c578_at(c: &mut Cpu, pc: u32) { f_8009c578_b(c, pc) }
#[inline(always)]
fn f_8009c578_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009c578;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009c578, pc); }
        match pc {
            0x8009c578 => {
                c.r[0] = c.mfspr(8);
                pc = 0x8009c57c;
            }
            0x8009c57c => {
                c.r[8] = 0x80400000;
                pc = 0x8009c580;
            }
            0x8009c580 => {
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                pc = 0x8009c584;
            }
            0x8009c584 => {
                c.r[7] = 0x80310000;
                pc = 0x8009c588;
            }
            0x8009c588 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x8009c58c;
            }
            0x8009c58c => {
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x8009c590;
            }
            0x8009c590 => {
                c.r[24] = c.r[3].wrapping_add(0x0);
                pc = 0x8009c594;
            }
            0x8009c594 => {
                c.r[25] = c.r[4].wrapping_add(0x0);
                pc = 0x8009c598;
            }
            0x8009c598 => {
                c.r[26] = c.r[5].wrapping_add(0x0);
                pc = 0x8009c59c;
            }
            0x8009c59c => {
                c.r[27] = c.r[6].wrapping_add(0x0);
                pc = 0x8009c5a0;
            }
            0x8009c5a0 => {
                c.r[30] = c.r[8].wrapping_add(0xffffb4f0);
                pc = 0x8009c5a4;
            }
            0x8009c5a4 => {
                c.r[31] = c.r[7].wrapping_add(0x9b8);
                pc = 0x8009c5a8;
            }
            0x8009c5a8 => {
                c.lr = 0x8009c5ac; crate::stubs::f_800a2778(c);
                pc = 0x8009c5ac;
            }
            0x8009c5ac => {
                c.r[5] = c.lwz(c.r[25].wrapping_add(0x19c));
                pc = 0x8009c5b0;
            }
            0x8009c5b0 => {
                c.r[28] = c.r[4].wrapping_add(0x0);
                pc = 0x8009c5b4;
            }
            0x8009c5b4 => {
                c.r[29] = c.r[3].wrapping_add(0x0);
                pc = 0x8009c5b8;
            }
            0x8009c5b8 => {
                c.r[0] = c.r[5].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                pc = 0x8009c5bc;
            }
            0x8009c5bc => {
                if !c.crbit(2) { pc = 0x8009c5d4; continue; }
                pc = 0x8009c5c0;
            }
            0x8009c5c0 => {
                c.r[3] = c.r[31].wrapping_add(0x5c);
                pc = 0x8009c5c4;
            }
            0x8009c5c4 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c5c8;
            }
            0x8009c5c8 => {
                c.r[4] = c.r[24].rotate_left(0) & 0xff;
                pc = 0x8009c5cc;
            }
            0x8009c5cc => {
                c.lr = 0x8009c5d0; f_8009c2e0(c);
                pc = 0x8009c5d0;
            }
            0x8009c5d0 => {
                pc = 0x8009c724; continue;
                pc = 0x8009c5d4;
            }
            0x8009c5d4 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xff;
                pc = 0x8009c5d8;
            }
            0x8009c5d8 => {
                c.cmpu(0, c.r[0], 0x6);
                pc = 0x8009c5dc;
            }
            0x8009c5dc => {
                if !c.crbit(2) { pc = 0x8009c6b4; continue; }
                pc = 0x8009c5e0;
            }
            0x8009c5e0 => {
                c.r[0] = c.r[5].rotate_left(0) & 0x100000;
                c.rc(c.r[0]);
                pc = 0x8009c5e4;
            }
            0x8009c5e4 => {
                if c.crbit(2) { pc = 0x8009c6b4; continue; }
                pc = 0x8009c5e8;
            }
            0x8009c5e8 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x40));
                pc = 0x8009c5ec;
            }
            0x8009c5ec => {
                c.cmpu(0, c.r[0], 0x0);
                pc = 0x8009c5f0;
            }
            0x8009c5f0 => {
                if c.crbit(2) { pc = 0x8009c6b4; continue; }
                pc = 0x8009c5f4;
            }
            0x8009c5f4 => {
                c.r[24] = 0x10;
                pc = 0x8009c5f8;
            }
            0x8009c5f8 => {
                c.lr = 0x8009c5fc; f_80097ffc(c);
                pc = 0x8009c5fc;
            }
            0x8009c5fc => {
                c.r[23] = c.r[3].wrapping_add(0x0);
                pc = 0x8009c600;
            }
            0x8009c600 => {
                c.r[3] = c.r[23] | 0x2000;
                pc = 0x8009c604;
            }
            0x8009c604 => {
                c.lr = 0x8009c608; f_80098004(c);
                pc = 0x8009c608;
            }
            0x8009c608 => {
                c.r[3] = 0x80000000;
                pc = 0x8009c60c;
            }
            0x8009c60c => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xd8));
                pc = 0x8009c610;
            }
            0x8009c610 => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x8009c614;
            }
            0x8009c614 => {
                if c.crbit(2) { pc = 0x8009c61c; continue; }
                pc = 0x8009c618;
            }
            0x8009c618 => {
                c.lr = 0x8009c61c; f_8009bb60(c);
                pc = 0x8009c61c;
            }
            0x8009c61c => {
                c.lr = 0x8009c620; f_80098080(c);
                pc = 0x8009c620;
            }
            0x8009c620 => {
                c.r[4] = 0x60060000;
                pc = 0x8009c624;
            }
            0x8009c624 => {
                c.r[22] = c.r[4].wrapping_add(0xfffff8ff);
                pc = 0x8009c628;
            }
            0x8009c628 => {
                c.r[3] = c.r[3] & c.r[22];
                pc = 0x8009c62c;
            }
            0x8009c62c => {
                c.lr = 0x8009c630; f_800980a0(c);
                pc = 0x8009c630;
            }
            0x8009c630 => {
                c.r[3] = c.r[23] | c.r[23];
                pc = 0x8009c634;
            }
            0x8009c634 => {
                c.lr = 0x8009c638; f_80098004(c);
                pc = 0x8009c638;
            }
            0x8009c638 => {
                c.r[23] = 0x80000000;
                pc = 0x8009c63c;
            }
            0x8009c63c => {
                c.r[0] = c.lwz(c.r[23].wrapping_add(0xd8));
                pc = 0x8009c640;
            }
            0x8009c640 => {
                c.cmpu(0, c.r[0], c.r[25]);
                pc = 0x8009c644;
            }
            0x8009c644 => {
                if !c.crbit(2) { pc = 0x8009c698; continue; }
                pc = 0x8009c648;
            }
            0x8009c648 => {
                c.lr = 0x8009c64c; f_800a1404(c);
                pc = 0x8009c64c;
            }
            0x8009c64c => {
                c.r[12] = c.lwz(c.r[30].wrapping_add(0x40));
                pc = 0x8009c650;
            }
            0x8009c650 => {
                c.r[4] = c.r[25].wrapping_add(0x0);
                pc = 0x8009c654;
            }
            0x8009c654 => {
                c.r[5] = c.r[26].wrapping_add(0x0);
                pc = 0x8009c658;
            }
            0x8009c658 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c65c;
            }
            0x8009c65c => {
                c.mtspr(8, c.r[12]);
                pc = 0x8009c660;
            }
            0x8009c660 => {
                c.r[6] = c.r[27].wrapping_add(0x0);
                pc = 0x8009c664;
            }
            0x8009c664 => {
                c.r[3] = 0x10;
                pc = 0x8009c668;
            }
            0x8009c668 => {
                if true { let t = c.lr; c.lr = 0x8009c66c; (c.dispatch)(c, t); }
                pc = 0x8009c66c;
            }
            0x8009c66c => {
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x19c));
                pc = 0x8009c670;
            }
            0x8009c670 => {
                c.r[0] = 0x0;
                pc = 0x8009c674;
            }
            0x8009c674 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffffdfff;
                pc = 0x8009c678;
            }
            0x8009c678 => {
                c.stw(c.r[25].wrapping_add(0x19c), c.r[3]);
                pc = 0x8009c67c;
            }
            0x8009c67c => {
                c.stw(c.r[23].wrapping_add(0xd8), c.r[0]);
                pc = 0x8009c680;
            }
            0x8009c680 => {
                c.r[0] = c.lwz(c.r[25].wrapping_add(0x194));
                pc = 0x8009c684;
            }
            0x8009c684 => {
                c.r[0] = c.r[0] & c.r[22];
                pc = 0x8009c688;
            }
            0x8009c688 => {
                c.stw(c.r[25].wrapping_add(0x194), c.r[0]);
                pc = 0x8009c68c;
            }
            0x8009c68c => {
                c.lr = 0x8009c690; f_800a1444(c);
                pc = 0x8009c690;
            }
            0x8009c690 => {
                c.lr = 0x8009c694; f_800a1960(c);
                pc = 0x8009c694;
            }
            0x8009c694 => {
                pc = 0x8009c6ac; continue;
                pc = 0x8009c698;
            }
            0x8009c698 => {
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x19c));
                pc = 0x8009c69c;
            }
            0x8009c69c => {
                c.r[0] = 0x0;
                pc = 0x8009c6a0;
            }
            0x8009c6a0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffffdfff;
                pc = 0x8009c6a4;
            }
            0x8009c6a4 => {
                c.stw(c.r[25].wrapping_add(0x19c), c.r[3]);
                pc = 0x8009c6a8;
            }
            0x8009c6a8 => {
                c.stw(c.r[23].wrapping_add(0xd8), c.r[0]);
                pc = 0x8009c6ac;
            }
            0x8009c6ac => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x8009c6b0;
            }
            0x8009c6b0 => {
                c.lr = 0x8009c6b4; f_8009bc50(c);
                pc = 0x8009c6b4;
            }
            0x8009c6b4 => {
                c.r[23] = c.r[24].rotate_left(0) & 0xff;
                pc = 0x8009c6b8;
            }
            0x8009c6b8 => {
                c.r[0] = c.r[24].rotate_left(2) & 0x3fc;
                pc = 0x8009c6bc;
            }
            0x8009c6bc => {
                c.r[22] = c.r[30].wrapping_add(c.r[0]);
                pc = 0x8009c6c0;
            }
            0x8009c6c0 => {
                c.r[0] = c.lwz(c.r[22]);
                pc = 0x8009c6c4;
            }
            0x8009c6c4 => {
                c.cmpu(0, c.r[0], 0x0);
                pc = 0x8009c6c8;
            }
            0x8009c6c8 => {
                if c.crbit(2) { pc = 0x8009c700; continue; }
                pc = 0x8009c6cc;
            }
            0x8009c6cc => {
                c.lr = 0x8009c6d0; f_800a1404(c);
                pc = 0x8009c6d0;
            }
            0x8009c6d0 => {
                c.r[12] = c.lwz(c.r[22]);
                pc = 0x8009c6d4;
            }
            0x8009c6d4 => {
                c.r[3] = c.r[23].wrapping_add(0x0);
                pc = 0x8009c6d8;
            }
            0x8009c6d8 => {
                c.r[4] = c.r[25].wrapping_add(0x0);
                pc = 0x8009c6dc;
            }
            0x8009c6dc => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c6e0;
            }
            0x8009c6e0 => {
                c.mtspr(8, c.r[12]);
                pc = 0x8009c6e4;
            }
            0x8009c6e4 => {
                c.r[5] = c.r[26].wrapping_add(0x0);
                pc = 0x8009c6e8;
            }
            0x8009c6e8 => {
                c.r[6] = c.r[27].wrapping_add(0x0);
                pc = 0x8009c6ec;
            }
            0x8009c6ec => {
                if true { let t = c.lr; c.lr = 0x8009c6f0; (c.dispatch)(c, t); }
                pc = 0x8009c6f0;
            }
            0x8009c6f0 => {
                c.lr = 0x8009c6f4; f_800a1444(c);
                pc = 0x8009c6f4;
            }
            0x8009c6f4 => {
                c.lr = 0x8009c6f8; f_800a1960(c);
                pc = 0x8009c6f8;
            }
            0x8009c6f8 => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x8009c6fc;
            }
            0x8009c6fc => {
                c.lr = 0x8009c700; f_8009bc50(c);
                pc = 0x8009c700;
            }
            0x8009c700 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xff;
                pc = 0x8009c704;
            }
            0x8009c704 => {
                c.cmpu(0, c.r[0], 0x8);
                pc = 0x8009c708;
            }
            0x8009c708 => {
                if !c.crbit(2) { pc = 0x8009c714; continue; }
                pc = 0x8009c70c;
            }
            0x8009c70c => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x8009c710;
            }
            0x8009c710 => {
                c.lr = 0x8009c714; f_8009bc50(c);
                pc = 0x8009c714;
            }
            0x8009c714 => {
                c.r[3] = c.r[31].wrapping_add(0x7c);
                pc = 0x8009c718;
            }
            0x8009c718 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c71c;
            }
            0x8009c71c => {
                c.r[4] = c.r[24].rotate_left(0) & 0xff;
                pc = 0x8009c720;
            }
            0x8009c720 => {
                c.lr = 0x8009c724; f_8009c2e0(c);
                pc = 0x8009c724;
            }
            0x8009c724 => {
                c.r[3] = c.r[13].wrapping_add(0xffff8174);
                pc = 0x8009c728;
            }
            0x8009c728 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c72c;
            }
            0x8009c72c => {
                c.lr = 0x8009c730; f_8009c2e0(c);
                pc = 0x8009c730;
            }
            0x8009c730 => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x8009c734;
            }
            0x8009c734 => {
                c.lr = 0x8009c738; f_8009be40(c);
                pc = 0x8009c738;
            }
            0x8009c738 => {
                c.r[4] = c.r[26].wrapping_add(0x0);
                pc = 0x8009c73c;
            }
            0x8009c73c => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c740;
            }
            0x8009c740 => {
                c.r[5] = c.r[27].wrapping_add(0x0);
                pc = 0x8009c744;
            }
            0x8009c744 => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                pc = 0x8009c748;
            }
            0x8009c748 => {
                c.lr = 0x8009c74c; f_8009c2e0(c);
                pc = 0x8009c74c;
            }
            0x8009c74c => {
                c.r[6] = c.r[28].wrapping_add(0x0);
                pc = 0x8009c750;
            }
            0x8009c750 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c754;
            }
            0x8009c754 => {
                c.r[5] = c.r[29].wrapping_add(0x0);
                pc = 0x8009c758;
            }
            0x8009c758 => {
                c.r[3] = c.r[31].wrapping_add(0xc8);
                pc = 0x8009c75c;
            }
            0x8009c75c => {
                c.lr = 0x8009c760; f_8009c2e0(c);
                pc = 0x8009c760;
            }
            0x8009c760 => {
                c.r[0] = c.r[24].rotate_left(0) & 0xff;
                pc = 0x8009c764;
            }
            0x8009c764 => {
                c.cmpu(0, c.r[0], 0xf);
                pc = 0x8009c768;
            }
            0x8009c768 => {
                if c.crbit(1) { pc = 0x8009c82c; continue; }
                pc = 0x8009c76c;
            }
            0x8009c76c => {
                c.r[3] = 0x80310000;
                pc = 0x8009c770;
            }
            0x8009c770 => {
                c.r[3] = c.r[3].wrapping_add(0xc94);
                pc = 0x8009c774;
            }
            0x8009c774 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x8009c778;
            }
            0x8009c778 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x8009c77c;
            }
            0x8009c77c => {
                c.mtspr(9, c.r[0]);
                pc = 0x8009c780;
            }
            0x8009c780 => {
                if true { let t = c.ctr; if (0x8009c578..0x8009c860).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x8009c784;
            }
            0x8009c784 => {
                c.r[4] = c.lwz(c.r[25].wrapping_add(0x198));
                pc = 0x8009c788;
            }
            0x8009c788 => {
                c.r[5] = c.r[27].wrapping_add(0x0);
                pc = 0x8009c78c;
            }
            0x8009c78c => {
                c.r[3] = c.r[31].wrapping_add(0xd8);
                pc = 0x8009c790;
            }
            0x8009c790 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c794;
            }
            0x8009c794 => {
                c.lr = 0x8009c798; f_8009c2e0(c);
                pc = 0x8009c798;
            }
            0x8009c798 => {
                pc = 0x8009c82c; continue;
                pc = 0x8009c79c;
            }
            0x8009c79c => {
                c.r[4] = c.lwz(c.r[25].wrapping_add(0x198));
                pc = 0x8009c7a0;
            }
            0x8009c7a0 => {
                c.r[3] = c.r[31].wrapping_add(0x138);
                pc = 0x8009c7a4;
            }
            0x8009c7a4 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c7a8;
            }
            0x8009c7a8 => {
                c.lr = 0x8009c7ac; f_8009c2e0(c);
                pc = 0x8009c7ac;
            }
            0x8009c7ac => {
                pc = 0x8009c82c; continue;
                pc = 0x8009c7b0;
            }
            0x8009c7b0 => {
                c.r[4] = c.lwz(c.r[25].wrapping_add(0x198));
                pc = 0x8009c7b4;
            }
            0x8009c7b4 => {
                c.r[5] = c.r[27].wrapping_add(0x0);
                pc = 0x8009c7b8;
            }
            0x8009c7b8 => {
                c.r[3] = c.r[31].wrapping_add(0x184);
                pc = 0x8009c7bc;
            }
            0x8009c7bc => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c7c0;
            }
            0x8009c7c0 => {
                c.lr = 0x8009c7c4; f_8009c2e0(c);
                pc = 0x8009c7c4;
            }
            0x8009c7c4 => {
                pc = 0x8009c82c; continue;
                pc = 0x8009c7c8;
            }
            0x8009c7c8 => {
                c.r[4] = c.lwz(c.r[25].wrapping_add(0x198));
                pc = 0x8009c7cc;
            }
            0x8009c7cc => {
                c.r[5] = c.r[27].wrapping_add(0x0);
                pc = 0x8009c7d0;
            }
            0x8009c7d0 => {
                c.r[3] = c.r[31].wrapping_add(0x1e8);
                pc = 0x8009c7d4;
            }
            0x8009c7d4 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c7d8;
            }
            0x8009c7d8 => {
                c.lr = 0x8009c7dc; f_8009c2e0(c);
                pc = 0x8009c7dc;
            }
            0x8009c7dc => {
                pc = 0x8009c82c; continue;
                pc = 0x8009c7e0;
            }
            0x8009c7e0 => {
                c.r[3] = c.r[13].wrapping_add(0xffff8174);
                pc = 0x8009c7e4;
            }
            0x8009c7e4 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c7e8;
            }
            0x8009c7e8 => {
                c.lr = 0x8009c7ec; f_8009c2e0(c);
                pc = 0x8009c7ec;
            }
            0x8009c7ec => {
                c.r[25] = 0xcc000000;
                pc = 0x8009c7f0;
            }
            0x8009c7f0 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c7f4;
            }
            0x8009c7f4 => {
                c.r[24] = c.r[25].wrapping_add(0x5000);
                pc = 0x8009c7f8;
            }
            0x8009c7f8 => {
                c.r[4] = c.lhz(c.r[25].wrapping_add(0x5030));
                pc = 0x8009c7fc;
            }
            0x8009c7fc => {
                c.r[3] = c.r[31].wrapping_add(0x248);
                pc = 0x8009c800;
            }
            0x8009c800 => {
                c.r[5] = c.lhz(c.r[25].wrapping_add(0x5032));
                pc = 0x8009c804;
            }
            0x8009c804 => {
                c.lr = 0x8009c808; f_8009c2e0(c);
                pc = 0x8009c808;
            }
            0x8009c808 => {
                c.r[4] = c.lhz(c.r[24].wrapping_add(0x20));
                pc = 0x8009c80c;
            }
            0x8009c80c => {
                c.r[3] = c.r[31].wrapping_add(0x268);
                pc = 0x8009c810;
            }
            0x8009c810 => {
                c.r[5] = c.lhz(c.r[24].wrapping_add(0x22));
                pc = 0x8009c814;
            }
            0x8009c814 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c818;
            }
            0x8009c818 => {
                c.lr = 0x8009c81c; f_8009c2e0(c);
                pc = 0x8009c81c;
            }
            0x8009c81c => {
                c.r[4] = c.lwz(c.r[25].wrapping_add(0x6014));
                pc = 0x8009c820;
            }
            0x8009c820 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c824;
            }
            0x8009c824 => {
                c.r[3] = c.r[31].wrapping_add(0x288);
                pc = 0x8009c828;
            }
            0x8009c828 => {
                c.lr = 0x8009c82c; f_8009c2e0(c);
                pc = 0x8009c82c;
            }
            0x8009c82c => {
                c.r[4] = c.lha(c.r[13].wrapping_add(0xffff9ef8));
                pc = 0x8009c830;
            }
            0x8009c830 => {
                c.r[3] = c.r[31].wrapping_add(0x2a4);
                pc = 0x8009c834;
            }
            0x8009c834 => {
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                pc = 0x8009c838;
            }
            0x8009c838 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9ef4));
                pc = 0x8009c83c;
            }
            0x8009c83c => {
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff9f00));
                pc = 0x8009c840;
            }
            0x8009c840 => {
                c.r[8] = c.lwz(c.r[13].wrapping_add(0xffff9f04));
                pc = 0x8009c844;
            }
            0x8009c844 => {
                c.lr = 0x8009c848; f_8009c2e0(c);
                pc = 0x8009c848;
            }
            0x8009c848 => {
                c.lr = 0x8009c84c; f_8009803c(c);
                pc = 0x8009c84c;
            }
            0x8009c84c => {
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 22..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                pc = 0x8009c850;
            }
            0x8009c850 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                pc = 0x8009c854;
            }
            0x8009c854 => {
                c.r[1] = c.r[1].wrapping_add(0x40);
                pc = 0x8009c858;
            }
            0x8009c858 => {
                c.mtspr(8, c.r[0]);
                pc = 0x8009c85c;
            }
            0x8009c85c => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009c578: bad pc {:#x}", pc),
        }
    }
}

/// ScreenReport (0x8009c860, 988 bytes)
#[inline(never)]
pub fn f_8009c860(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009c860) { return; }
        if c.prof { crate::prof::enter(c, 0x8009c860); f_8009c860_b(c, 0x8009c860); crate::prof::exit(c, 0x8009c860); return; }
    }
    f_8009c860_b(c, 0x8009c860)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009c860_at(c: &mut Cpu, pc: u32) { f_8009c860_b(c, pc) }
#[inline(always)]
fn f_8009c860_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009c860;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009c860, pc); }
        match pc {
            0x8009c860 => {
                c.r[0] = c.mfspr(8);
                c.r[11] = c.r[5].rotate_left(0) & 0xffff;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[5] = 0x80810000;
                { let ea = c.r[1].wrapping_add(0xfffffe60); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x160); for n in 16..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[21] = c.r[4].rotate_left(0) & 0xffff;
                c.r[4] = 0x88890000;
                c.r[26] = c.r[3].wrapping_add(0x0);
                c.r[27] = c.r[6].wrapping_add(0x0);
                c.r[28] = c.r[7].wrapping_add(0x0);
                c.r[29] = c.r[8].wrapping_add(0x0);
                c.r[30] = c.r[9].wrapping_add(0x0);
                c.r[16] = c.r[11].wrapping_add(0xffffffe8);
                c.r[22] = c.r[21].wrapping_add(0xffffffd0);
                c.r[25] = c.r[5].wrapping_add(0xffff8081);
                c.r[24] = c.r[4].wrapping_add(0xffff8889);
                c.r[23] = c.r[1].wrapping_add(0x3c);
                pc = 0x8009c8a8;
            }
            0x8009c8a8 => {
                c.cmps(0, c.r[16], c.r[29]);
                if c.crbit(0) { pc = 0x8009cc28; continue; }
                pc = 0x8009c8b0;
            }
            0x8009c8b0 => {
                c.r[0] = c.r[29].wrapping_mul(c.r[21]);
                c.r[0] = c.r[28].wrapping_add(c.r[0]);
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[31] = c.r[28].wrapping_add(0x0);
                c.r[19] = c.r[26].wrapping_add(c.r[0]);
                pc = 0x8009cc1c; continue;
                pc = 0x8009c8c8;
            }
            0x8009c8c8 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0xa);
                if !c.crbit(2) { pc = 0x8009c8e0; continue; }
                pc = 0x8009c8d4;
            }
            0x8009c8d4 => {
                c.r[29] = c.r[29].wrapping_add(c.r[30]);
                c.r[10] = c.r[10].wrapping_add(0x1);
                pc = 0x8009c8a8; continue;
                pc = 0x8009c8e0;
            }
            0x8009c8e0 => {
                c.cmps(0, c.r[22], c.r[31]);
                if !c.crbit(0) { pc = 0x8009c8f0; continue; }
                pc = 0x8009c8e8;
            }
            0x8009c8e8 => {
                c.r[29] = c.r[29].wrapping_add(c.r[30]);
                pc = 0x8009c8a8; continue;
                pc = 0x8009c8f0;
            }
            0x8009c8f0 => {
                c.r[0] = 0x3;
                c.mtspr(9, c.r[0]);
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                pc = 0x8009c900;
            }
            0x8009c900 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x7;
                c.r[6] = c.r[0].wrapping_add(c.r[5]);
                c.r[3] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[3].rotate_left(29) & 0x1fffffff;
                c.r[6] = c.r[6].rotate_left(2) & 0xfffffffc;
                c.r[9] = c.r[1].wrapping_add(0x3c);
                c.r[9] = c.r[9].wrapping_add(c.r[6]);
                c.r[20] = 0x0;
                c.stw(c.r[9], c.r[20]);
                c.r[0] = c.r[0].wrapping_mul(0x18);
                c.stw(c.r[9].wrapping_add(0x20), c.r[20]);
                c.r[3] = c.r[3].rotate_left(0) & 0x7;
                c.r[8] = c.r[4].wrapping_add(0x2);
                c.stw(c.r[9].wrapping_add(0x40), c.r[20]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[9] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[3] = c.r[3].wrapping_add(c.r[9]);
                c.stw(c.r[3], c.r[20]);
                c.r[7] = c.r[8].rotate_left(29) & 0x1fffffff;
                c.r[6] = c.r[4].wrapping_add(0x3);
                c.stw(c.r[3].wrapping_add(0x20), c.r[20]);
                c.r[0] = c.r[6].rotate_left(29) & 0x1fffffff;
                c.r[7] = c.r[7].wrapping_mul(0x18);
                c.stw(c.r[3].wrapping_add(0x40), c.r[20]);
                c.r[3] = c.r[8].rotate_left(0) & 0x7;
                c.r[3] = c.r[3].wrapping_add(c.r[7]);
                c.r[7] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[3] = c.r[3].wrapping_add(c.r[7]);
                c.stw(c.r[3], c.r[20]);
                c.r[18] = c.r[4].wrapping_add(0x4);
                c.r[12] = c.r[18].rotate_left(29) & 0x1fffffff;
                c.stw(c.r[3].wrapping_add(0x20), c.r[20]);
                c.r[11] = c.r[4].wrapping_add(0x5);
                c.r[9] = c.r[11].rotate_left(29) & 0x1fffffff;
                c.stw(c.r[3].wrapping_add(0x40), c.r[20]);
                c.r[3] = c.r[6].rotate_left(0) & 0x7;
                c.r[0] = c.r[0].wrapping_mul(0x18);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[17] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[3] = c.r[3].wrapping_add(c.r[17]);
                c.stw(c.r[3], c.r[20]);
                c.r[8] = c.r[4].wrapping_add(0x6);
                c.r[12] = c.r[12].wrapping_mul(0x18);
                c.stw(c.r[3].wrapping_add(0x20), c.r[20]);
                c.r[6] = c.r[4].wrapping_add(0x7);
                c.stw(c.r[3].wrapping_add(0x40), c.r[20]);
                c.r[17] = c.r[18].rotate_left(0) & 0x7;
                c.r[12] = c.r[17].wrapping_add(c.r[12]);
                c.r[0] = c.r[6].rotate_left(29) & 0x1fffffff;
                c.r[12] = c.r[12].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[3] = c.r[3].wrapping_add(c.r[12]);
                c.stw(c.r[3], c.r[20]);
                c.r[9] = c.r[9].wrapping_mul(0x18);
                c.stw(c.r[3].wrapping_add(0x20), c.r[20]);
                c.r[11] = c.r[11].rotate_left(0) & 0x7;
                c.r[9] = c.r[11].wrapping_add(c.r[9]);
                c.stw(c.r[3].wrapping_add(0x40), c.r[20]);
                c.r[3] = c.r[9].rotate_left(2) & 0xfffffffc;
                c.r[9] = c.r[1].wrapping_add(0x3c);
                c.r[9] = c.r[9].wrapping_add(c.r[3]);
                c.r[7] = c.r[8].rotate_left(29) & 0x1fffffff;
                c.stw(c.r[9], c.r[20]);
                c.r[3] = c.r[7].wrapping_mul(0x18);
                c.stw(c.r[9].wrapping_add(0x20), c.r[20]);
                c.r[7] = c.r[8].rotate_left(0) & 0x7;
                c.stw(c.r[9].wrapping_add(0x40), c.r[20]);
                c.r[3] = c.r[7].wrapping_add(c.r[3]);
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[7] = c.r[1].wrapping_add(0x3c);
                c.r[7] = c.r[7].wrapping_add(c.r[3]);
                c.stw(c.r[7], c.r[20]);
                c.r[0] = c.r[0].wrapping_mul(0x18);
                c.stw(c.r[7].wrapping_add(0x20), c.r[20]);
                c.r[3] = c.r[6].rotate_left(0) & 0x7;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[7].wrapping_add(0x40), c.r[20]);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[3], c.r[20]);
                c.r[5] = c.r[5].wrapping_add(0x18);
                c.r[4] = c.r[4].wrapping_add(0x8);
                c.stw(c.r[3].wrapping_add(0x20), c.r[20]);
                c.stw(c.r[3].wrapping_add(0x40), c.r[20]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009c900; continue; }
                pc = 0x8009ca64;
            }
            0x8009ca64 => {
                c.r[3] = c.r[10].wrapping_add(0x0);
                c.r[4] = c.r[1].wrapping_add(0x3c);
                c.r[7] = c.r[1].wrapping_add(0x15c);
                c.r[5] = 0x0;
                c.r[6] = 0x6;
                c.lr = 0x8009ca7c; f_8009dc38(c);
                pc = 0x8009ca7c;
            }
            0x8009ca7c => {
                c.r[10] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.r[20].wrapping_add(0x0);
                pc = 0x8009ca84;
            }
            0x8009ca84 => {
                c.r[3] = c.r[20].rotate_left(29) & 0x1fffffff;
                c.r[5] = c.r[3].wrapping_mul(0x18);
                c.r[4] = 0xc;
                c.r[3] = c.r[0] | c.r[0];
                c.mtspr(9, c.r[4]);
                c.r[6] = c.r[20].rotate_left(0) & 0x7;
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[5] = c.r[6].wrapping_add(c.r[5]);
                c.r[7] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[6] = 0x0;
                pc = 0x8009caac;
            }
            0x8009caac => {
                c.r[8] = c.r[6].rotate_left(0) & 0xfffffff8;
                c.r[8] = c.r[5].wrapping_add(c.r[8]);
                c.r[9] = c.r[8].rotate_left(2) & 0xfffffffc;
                c.r[8] = c.r[6].rotate_left(0) & 0x7;
                c.r[9] = c.lwz(c.r[23].wrapping_add(c.r[9]));
                c.r[8] = c.addc(!c.r[8], 0x7, 1);
                c.r[8] = c.r[8].rotate_left(2) & 0xfffffffc;
                c.r[8] = srw(c.r[9], c.r[8]);
                c.r[8] = c.r[8].rotate_left(0) & 0xf;
                c.rc(c.r[8]);
                if c.crbit(2) { pc = 0x8009cb44; continue; }
                pc = 0x8009cad4;
            }
            0x8009cad4 => {
                c.r[8] = c.r[8].wrapping_mul(0xef);
                c.r[9] = c.lbz(c.r[27]);
                c.r[8] = c.r[9].wrapping_mul(c.r[8]);
                c.r[9] = mulhw(c.r[25], c.r[8]);
                c.r[8] = c.r[9].wrapping_add(c.r[8]);
                { let v = c.r[8] as i32; c.set_ca(v < 0 && (v as u32) << 25 != 0); c.r[8] = (v >> 7) as u32; }
                c.r[9] = c.r[8].rotate_left(1) & 0x1;
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = mulhw(c.r[24], c.r[8]);
                c.r[8] = c.r[9].wrapping_add(c.r[8]);
                { let v = c.r[8] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[8] = (v >> 3) as u32; }
                c.r[9] = c.r[8].rotate_left(1) & 0x1;
                c.r[9] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.r[9].wrapping_add(0x10);
                c.r[8] = c.r[4].rotate_left(0) & 0x1;
                c.rc(c.r[8]);
                c.stb(c.r[19].wrapping_add(c.r[7]), c.r[9]);
                if c.crbit(2) { pc = 0x8009cb30; continue; }
                pc = 0x8009cb18;
            }
            0x8009cb18 => {
                c.r[8] = c.lbz(c.r[27].wrapping_add(0x1));
                c.r[9] = c.r[19].wrapping_add(c.r[7]);
                c.stb(c.r[9].wrapping_add(0xffffffff), c.r[8]);
                c.r[8] = c.lbz(c.r[27].wrapping_add(0x2));
                c.stb(c.r[9].wrapping_add(0x1), c.r[8]);
                pc = 0x8009cb44; continue;
                pc = 0x8009cb30;
            }
            0x8009cb30 => {
                c.r[8] = c.lbz(c.r[27].wrapping_add(0x2));
                c.r[9] = c.r[19].wrapping_add(c.r[7]);
                c.stb(c.r[9].wrapping_add(0xffffffff), c.r[8]);
                c.r[8] = c.lbz(c.r[27].wrapping_add(0x1));
                c.stb(c.r[9].wrapping_add(0x1), c.r[8]);
                pc = 0x8009cb44;
            }
            0x8009cb44 => {
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.r[8] = c.r[6].rotate_left(0) & 0xfffffff8;
                c.r[8] = c.r[5].wrapping_add(c.r[8]);
                c.r[9] = c.r[8].rotate_left(2) & 0xfffffffc;
                c.r[8] = c.r[6].rotate_left(0) & 0x7;
                c.r[9] = c.lwz(c.r[23].wrapping_add(c.r[9]));
                c.r[8] = c.addc(!c.r[8], 0x7, 1);
                c.r[8] = c.r[8].rotate_left(2) & 0xfffffffc;
                c.r[8] = srw(c.r[9], c.r[8]);
                c.r[8] = c.r[8].rotate_left(0) & 0xf;
                c.rc(c.r[8]);
                c.r[7] = c.r[7].wrapping_add(0x2);
                c.r[4] = c.r[4].wrapping_add(0x1);
                if c.crbit(2) { pc = 0x8009cbe8; continue; }
                pc = 0x8009cb78;
            }
            0x8009cb78 => {
                c.r[8] = c.r[8].wrapping_mul(0xef);
                c.r[9] = c.lbz(c.r[27]);
                c.r[8] = c.r[9].wrapping_mul(c.r[8]);
                c.r[9] = mulhw(c.r[25], c.r[8]);
                c.r[8] = c.r[9].wrapping_add(c.r[8]);
                { let v = c.r[8] as i32; c.set_ca(v < 0 && (v as u32) << 25 != 0); c.r[8] = (v >> 7) as u32; }
                c.r[9] = c.r[8].rotate_left(1) & 0x1;
                c.r[8] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = mulhw(c.r[24], c.r[8]);
                c.r[8] = c.r[9].wrapping_add(c.r[8]);
                { let v = c.r[8] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[8] = (v >> 3) as u32; }
                c.r[9] = c.r[8].rotate_left(1) & 0x1;
                c.r[9] = c.r[8].wrapping_add(c.r[9]);
                c.r[9] = c.r[9].wrapping_add(0x10);
                c.r[8] = c.r[4].rotate_left(0) & 0x1;
                c.rc(c.r[8]);
                c.stb(c.r[19].wrapping_add(c.r[7]), c.r[9]);
                if c.crbit(2) { pc = 0x8009cbd4; continue; }
                pc = 0x8009cbbc;
            }
            0x8009cbbc => {
                c.r[8] = c.lbz(c.r[27].wrapping_add(0x1));
                c.r[9] = c.r[19].wrapping_add(c.r[7]);
                c.stb(c.r[9].wrapping_add(0xffffffff), c.r[8]);
                c.r[8] = c.lbz(c.r[27].wrapping_add(0x2));
                c.stb(c.r[9].wrapping_add(0x1), c.r[8]);
                pc = 0x8009cbe8; continue;
                pc = 0x8009cbd4;
            }
            0x8009cbd4 => {
                c.r[8] = c.lbz(c.r[27].wrapping_add(0x2));
                c.r[9] = c.r[19].wrapping_add(c.r[7]);
                c.stb(c.r[9].wrapping_add(0xffffffff), c.r[8]);
                c.r[8] = c.lbz(c.r[27].wrapping_add(0x1));
                c.stb(c.r[9].wrapping_add(0x1), c.r[8]);
                pc = 0x8009cbe8;
            }
            0x8009cbe8 => {
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.r[7] = c.r[7].wrapping_add(0x2);
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009caac; continue; }
                pc = 0x8009cbfc;
            }
            0x8009cbfc => {
                c.r[20] = c.r[20].wrapping_add(0x1);
                c.cmpu(0, c.r[20], 0x18);
                c.r[0] = c.r[0].wrapping_add(c.r[21]);
                if c.crbit(0) { pc = 0x8009ca84; continue; }
                pc = 0x8009cc0c;
            }
            0x8009cc0c => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x15c));
                c.r[0] = c.r[3].rotate_left(1) & 0xfffffffe;
                c.r[19] = c.r[19].wrapping_add(c.r[0]);
                c.r[31] = c.r[31].wrapping_add(c.r[3]);
                pc = 0x8009cc1c;
            }
            0x8009cc1c => {
                c.r[3] = c.lbz(c.r[10]);
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009c8c8; continue; }
                pc = 0x8009cc28;
            }
            0x8009cc28 => {
                { let mut ea = c.r[1].wrapping_add(0x160); for n in 16..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1a4));
                c.r[1] = c.r[1].wrapping_add(0x1a0);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009c860: bad pc {:#x}", pc),
        }
    }
}

/// ConfigureVideo (0x8009cc3c, 252 bytes)
#[inline(never)]
pub fn f_8009cc3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009cc3c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009cc3c); f_8009cc3c_b(c, 0x8009cc3c); crate::prof::exit(c, 0x8009cc3c); return; }
    }
    f_8009cc3c_b(c, 0x8009cc3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009cc3c_at(c: &mut Cpu, pc: u32) { f_8009cc3c_b(c, pc) }
#[inline(always)]
fn f_8009cc3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009cc3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009cc3c, pc); }
        match pc {
            0x8009cc3c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x1e0;
                { let ea = c.r[1].wrapping_add(0xffffffb8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.sth(c.r[1].wrapping_add(0x10), c.r[3]);
                c.r[3] = 0x28;
                c.sth(c.r[1].wrapping_add(0x12), c.r[0]);
                c.r[0] = 0x280;
                c.sth(c.r[1].wrapping_add(0x14), c.r[4]);
                c.sth(c.r[1].wrapping_add(0x16), c.r[3]);
                c.sth(c.r[1].wrapping_add(0x1a), c.r[0]);
                c.sth(c.r[1].wrapping_add(0x1c), c.r[4]);
                c.lr = 0x8009cc70; f_800aa430(c);
                pc = 0x8009cc70;
            }
            0x8009cc70 => {
                c.cmps(0, c.r[3], 0x2);
                if c.crbit(2) { pc = 0x8009cc98; continue; }
                pc = 0x8009cc78;
            }
            0x8009cc78 => {
                if !c.crbit(0) { pc = 0x8009cc8c; continue; }
                pc = 0x8009cc7c;
            }
            0x8009cc7c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009cc98; continue; }
                pc = 0x8009cc84;
            }
            0x8009cc84 => {
                if !c.crbit(0) { pc = 0x8009ccf4; continue; }
                pc = 0x8009cc88;
            }
            0x8009cc88 => {
                pc = 0x8009cd0c; continue;
                pc = 0x8009cc8c;
            }
            0x8009cc8c => {
                c.cmps(0, c.r[3], 0x5);
                if c.crbit(2) { pc = 0x8009ccd8; continue; }
                pc = 0x8009cc94;
            }
            0x8009cc94 => {
                pc = 0x8009cd0c; continue;
                pc = 0x8009cc98;
            }
            0x8009cc98 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x206c));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009ccc0; continue; }
                pc = 0x8009cca8;
            }
            0x8009cca8 => {
                c.r[0] = 0x2;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.r[0] = 0x0;
                c.sth(c.r[1].wrapping_add(0x18), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[0]);
                pc = 0x8009cd0c; continue;
                pc = 0x8009ccc0;
            }
            0x8009ccc0 => {
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[0] = 0x1;
                c.sth(c.r[1].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[0]);
                pc = 0x8009cd0c; continue;
                pc = 0x8009ccd8;
            }
            0x8009ccd8 => {
                c.r[0] = 0x14;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.r[3] = 0x0;
                c.r[0] = 0x1;
                c.sth(c.r[1].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[0]);
                pc = 0x8009cd0c; continue;
                pc = 0x8009ccf4;
            }
            0x8009ccf4 => {
                c.r[0] = 0x4;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.r[3] = 0x2f;
                c.r[0] = 0x1;
                c.sth(c.r[1].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[0]);
                pc = 0x8009cd0c;
            }
            0x8009cd0c => {
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.lr = 0x8009cd14; f_800a94ac(c);
                pc = 0x8009cd14;
            }
            0x8009cd14 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x280;
                c.r[6] = 0x1e0;
                c.lr = 0x8009cd28; f_800a9cd4(c);
                pc = 0x8009cd28;
            }
            0x8009cd28 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x4c));
                c.r[1] = c.r[1].wrapping_add(0x48);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009cc3c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009CD38 (0x8009cd38, 340 bytes)
#[inline(never)]
pub fn f_8009cd38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009cd38) { return; }
        if c.prof { crate::prof::enter(c, 0x8009cd38); f_8009cd38_b(c, 0x8009cd38); crate::prof::exit(c, 0x8009cd38); return; }
    }
    f_8009cd38_b(c, 0x8009cd38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009cd38_at(c: &mut Cpu, pc: u32) { f_8009cd38_b(c, pc) }
#[inline(always)]
fn f_8009cd38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009cd38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009cd38, pc); }
        match pc {
            0x8009cd38 => {
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3].wrapping_add(0x0);
                c.r[28] = c.r[4].wrapping_add(0x0);
                c.r[29] = c.r[5].wrapping_add(0x0);
                c.r[31] = c.r[6].wrapping_add(0xffffb538);
                c.r[30] = 0x80000000;
                c.lr = 0x8009cd64; f_8009df3c(c);
                pc = 0x8009cd64;
            }
            0x8009cd64 => {
                c.lr = 0x8009cd68; f_800a1404(c);
                pc = 0x8009cd68;
            }
            0x8009cd68 => {
                c.r[3] = c.r[31].wrapping_add(0x10);
                c.lr = 0x8009cd70; f_8009bd60(c);
                pc = 0x8009cd70;
            }
            0x8009cd70 => {
                c.r[3] = c.r[31].wrapping_add(0x10);
                c.lr = 0x8009cd78; f_8009bb68(c);
                pc = 0x8009cd78;
            }
            0x8009cd78 => {
                c.lr = 0x8009cd7c; f_8009b1b8(c);
                pc = 0x8009cd7c;
            }
            0x8009cd7c => {
                c.r[3] = 0x0;
                c.lr = 0x8009cd84; f_800ac3e8(c);
                pc = 0x8009cd84;
            }
            0x8009cd84 => {
                c.r[3] = 0x0;
                c.lr = 0x8009cd8c; f_800ac414(c);
                pc = 0x8009cd8c;
            }
            0x8009cd8c => {
                c.lr = 0x8009cd90; f_800a8b34(c);
                pc = 0x8009cd90;
            }
            0x8009cd90 => {
                c.r[3] = 0x1;
                c.lr = 0x8009cd98; f_800aa204(c);
                pc = 0x8009cd98;
            }
            0x8009cd98 => {
                c.lr = 0x8009cd9c; f_800aa068(c);
                pc = 0x8009cd9c;
            }
            0x8009cd9c => {
                c.lr = 0x8009cda0; f_8009df50(c);
                pc = 0x8009cda0;
            }
            0x8009cda0 => {
                c.lr = 0x8009cda4; f_800aa280(c);
                pc = 0x8009cda4;
            }
            0x8009cda4 => {
                c.r[23] = c.r[3] | c.r[3];
                pc = 0x8009cda8;
            }
            0x8009cda8 => {
                c.lr = 0x8009cdac; f_800aa280(c);
                pc = 0x8009cdac;
            }
            0x8009cdac => {
                c.r[0] = c.r[3].wrapping_sub(c.r[23]);
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(0) { pc = 0x8009cda8; continue; }
                pc = 0x8009cdb8;
            }
            0x8009cdb8 => {
                c.lr = 0x8009cdbc; crate::stubs::f_800a2778(c);
                pc = 0x8009cdbc;
            }
            0x8009cdbc => {
                c.r[5] = 0x80000000;
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xf8));
                c.r[5] = 0x10620000;
                c.r[5] = c.r[5].wrapping_add(0x4dd3);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = mulhwu(c.r[5], c.r[0]);
                c.r[0] = c.r[0].rotate_left(26) & 0x3ffffff;
                c.r[25] = c.r[0].wrapping_mul(0x3e8);
                c.r[23] = c.r[4].wrapping_add(0x0);
                c.r[24] = c.r[3].wrapping_add(0x0);
                c.r[26] = 0x0;
                pc = 0x8009cde8;
            }
            0x8009cde8 => {
                c.r[3] = 0x0;
                c.lr = 0x8009cdf0; f_8009febc(c);
                pc = 0x8009cdf0;
            }
            0x8009cdf0 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009ce20; continue; }
                pc = 0x8009cdf8;
            }
            0x8009cdf8 => {
                c.lr = 0x8009cdfc; crate::stubs::f_800a2778(c);
                pc = 0x8009cdfc;
            }
            0x8009cdfc => {
                c.r[5] = c.addc(!c.r[23], c.r[4], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[24], c.r[3], ci); }
                c.r[4] = c.r[0] ^ 0x80000000;
                c.r[3] = c.r[26] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[25], c.r[5], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[4], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[4], c.r[4], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if !c.crbit(2) { pc = 0x8009cde8; continue; }
                pc = 0x8009ce20;
            }
            0x8009ce20 => {
                c.lr = 0x8009ce24; f_8009df3c(c);
                pc = 0x8009ce24;
            }
            0x8009ce24 => {
                c.r[3] = 0x1;
                c.lr = 0x8009ce2c; f_8009febc(c);
                pc = 0x8009ce2c;
            }
            0x8009ce2c => {
                c.r[3] = 0x800a0000;
                c.r[4] = c.r[3].wrapping_add(0xffffa190);
                c.r[3] = 0x8;
                c.lr = 0x8009ce3c; f_8009a0c4(c);
                pc = 0x8009ce3c;
            }
            0x8009ce3c => {
                c.lr = 0x8009ce40; f_800b8ae8(c);
                pc = 0x8009ce40;
            }
            0x8009ce40 => {
                c.r[3] = 0x81200000;
                c.lr = 0x8009ce48; f_8009afc8(c);
                pc = 0x8009ce48;
            }
            0x8009ce48 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x38));
                c.lr = 0x8009ce50; f_8009afc0(c);
                pc = 0x8009ce50;
            }
            0x8009ce50 => {
                c.r[0] = c.lwz(c.r[27]);
                c.stw(c.r[31], c.r[0]);
                c.r[0] = c.lwz(c.r[28]);
                c.stw(c.r[31].wrapping_add(0x4), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x8), c.r[29]);
                c.lr = 0x8009ce68; f_8009afb0(c);
                pc = 0x8009ce68;
            }
            0x8009ce68 => {
                c.r[5] = 0x800a0000;
                c.r[4] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[5].wrapping_add(0xffffce8c);
                c.lr = 0x8009ce78; f_8009bd30(c);
                pc = 0x8009ce78;
            }
            0x8009ce78 => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009cd38: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009CE8C (0x8009ce8c, 1668 bytes)
#[inline(never)]
pub fn f_8009ce8c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ce8c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ce8c); f_8009ce8c_b(c, 0x8009ce8c); crate::prof::exit(c, 0x8009ce8c); return; }
    }
    f_8009ce8c_b(c, 0x8009ce8c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ce8c_at(c: &mut Cpu, pc: u32) { f_8009ce8c_b(c, pc) }
#[inline(always)]
fn f_8009ce8c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ce8c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ce8c, pc); }
        match pc {
            0x8009ce8c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffff78); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x84), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x80), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x7c), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x78), c.r[28]);
                c.lr = 0x8009ceac; f_8009df50(c);
                pc = 0x8009ceac;
            }
            0x8009ceac => {
                c.r[3] = 0x80400000;
                c.r[31] = c.r[3].wrapping_add(0xffffb538);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.lr = 0x8009cebc; f_800caa20(c);
                pc = 0x8009cebc;
            }
            0x8009cebc => {
                c.r[28] = c.r[3].wrapping_add(0x1);
                c.r[3] = c.r[28].wrapping_add(0x0);
                c.r[4] = 0x20;
                c.lr = 0x8009cecc; f_8009afd0(c);
                pc = 0x8009cecc;
            }
            0x8009cecc => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[5] = c.r[28] | c.r[28];
                c.lr = 0x8009ced8; f_800c8174(c);
                pc = 0x8009ced8;
            }
            0x8009ced8 => {
                c.stw(c.r[31].wrapping_add(0x8), c.r[3]);
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x8009cee8; f_8009870c(c);
                pc = 0x8009cee8;
            }
            0x8009cee8 => {
                c.r[3] = 0x2;
                c.r[4] = 0x0;
                c.lr = 0x8009cef4; f_8009870c(c);
                pc = 0x8009cef4;
            }
            0x8009cef4 => {
                c.r[3] = 0xcc000000;
                c.r[28] = c.r[3].wrapping_add(0x6800);
                pc = 0x8009cf20; continue;
                pc = 0x8009cf00;
            }
            0x8009cf00 => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8009cf00; continue; }
                pc = 0x8009cf10;
            }
            0x8009cf10 => {
                c.r[3] = 0x0;
                c.lr = 0x8009cf18; f_80098cd4(c);
                pc = 0x8009cf18;
            }
            0x8009cf18 => {
                c.r[3] = 0x0;
                c.lr = 0x8009cf20; f_800992e0(c);
                pc = 0x8009cf20;
            }
            0x8009cf20 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x8009cf30; f_800991e4(c);
                pc = 0x8009cf30;
            }
            0x8009cf30 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009cf00; continue; }
                pc = 0x8009cf38;
            }
            0x8009cf38 => {
                c.r[3] = 0x0;
                c.lr = 0x8009cf40; f_800992e0(c);
                pc = 0x8009cf40;
            }
            0x8009cf40 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                pc = 0x8009cf48;
            }
            0x8009cf48 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8009cf48; continue; }
                pc = 0x8009cf58;
            }
            0x8009cf58 => {
                c.lr = 0x8009cf5c; f_8009d820(c);
                pc = 0x8009cf5c;
            }
            0x8009cf5c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8009cf80; continue; }
                pc = 0x8009cf68;
            }
            0x8009cf68 => {
                c.r[3] = 0x90000;
                c.r[3] = c.r[3].wrapping_add(0xee4);
                c.r[4] = 0x20;
                c.lr = 0x8009cf78; f_8009afd0(c);
                pc = 0x8009cf78;
            }
            0x8009cf78 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x8009cf94; continue;
                pc = 0x8009cf80;
            }
            0x8009cf80 => {
                c.r[3] = 0x10000;
                c.r[3] = c.r[3].wrapping_add(0x120);
                c.r[4] = 0x20;
                c.lr = 0x8009cf90; f_8009afd0(c);
                pc = 0x8009cf90;
            }
            0x8009cf90 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x8009cf94;
            }
            0x8009cf94 => {
                c.lr = 0x8009cf98; f_8009afb8(c);
                pc = 0x8009cf98;
            }
            0x8009cf98 => {
                c.r[4] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.lr = 0x8009cfa4; f_8009d904(c);
                pc = 0x8009cfa4;
            }
            0x8009cfa4 => {
                c.r[3] = 0x90000;
                c.r[3] = c.r[3].wrapping_add(0x6000);
                c.r[4] = 0x20;
                c.lr = 0x8009cfb4; f_8009afd0(c);
                pc = 0x8009cfb4;
            }
            0x8009cfb4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = 0x43300000;
                { let ea = c.r[2].wrapping_add(0xffff8bd8); c.f[0] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x28), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff8bac); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x28));
                c.r[3] = c.lbz(c.r[1].wrapping_add(0x29));
                c.stw(c.r[1].wrapping_add(0x6c), c.r[0]);
                c.r[5] = c.lbz(c.r[1].wrapping_add(0x2a));
                c.stw(c.r[1].wrapping_add(0x64), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff8bb0); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.stw(c.r[1].wrapping_add(0x68), c.r[4]);
                { let ea = c.r[2].wrapping_add(0xffff8ba8); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                c.stw(c.r[1].wrapping_add(0x60), c.r[4]);
                { let ea = c.r[1].wrapping_add(0x68); c.f[3] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x74), c.r[5]);
                { let ea = c.r[1].wrapping_add(0x60); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[3] - c.f[0]); c.f[3] = v; c.ps1[3] = v; }
                c.stw(c.r[1].wrapping_add(0x70), c.r[4]);
                { let v = s(c.f[1] - c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bc0); c.f[8] = c.lfs(ea); c.ps1[8] = c.f[8]; }
                c.stw(c.r[1].wrapping_add(0x4c), c.r[3]);
                { let ea = c.r[1].wrapping_add(0x70); c.f[5] = c.lfd(ea); }
                { let v = s(c.f[4] * c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[5] - c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bbc); c.f[9] = c.lfs(ea); c.ps1[9] = c.f[9]; }
                c.stw(c.r[1].wrapping_add(0x50), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let v = s(c.f[6] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x3c), c.r[3]);
                { let v = s(c.f[2] - c.f[0]); c.f[3] = v; c.ps1[3] = v; }
                c.stw(c.r[1].wrapping_add(0x5c), c.r[5]);
                { let v = s(c.f[4] + c.f[1]); c.f[11] = v; c.ps1[11] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bb8); c.f[10] = c.lfs(ea); c.ps1[10] = c.f[10]; }
                c.stw(c.r[1].wrapping_add(0x48), c.r[4]);
                { let v = s(c.f[9] * c.f[3]); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bc4); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[1].wrapping_add(0x48); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x40), c.r[4]);
                { let v = s(c.f[1] - c.f[0]); c.f[7] = v; c.ps1[7] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bc8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.stw(c.r[1].wrapping_add(0x38), c.r[4]);
                { let ea = c.r[1].wrapping_add(0x40); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x34), c.r[5]);
                { let ea = c.r[1].wrapping_add(0x38); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[2] - c.f[0]); c.f[6] = v; c.ps1[6] = v; }
                c.stw(c.r[1].wrapping_add(0x30), c.r[4]);
                { let v = s(c.f[8] * c.f[7]); c.f[7] = v; c.ps1[7] = v; }
                { let v = s(c.f[1] - c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                c.stw(c.r[1].wrapping_add(0x58), c.r[4]);
                { let ea = c.r[2].wrapping_add(0xffff8ba4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[10] * c.f[6]); c.f[6] = v; c.ps1[6] = v; }
                { let ea = c.r[1].wrapping_add(0x58); c.f[8] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x30); c.f[2] = c.lfd(ea); }
                { let v = s(c.f[5] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[8] - c.f[0]); c.f[8] = v; c.ps1[8] = v; }
                { let v = s(c.f[2] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8ba0); c.f[12] = c.lfs(ea); c.ps1[12] = c.f[12]; }
                { let v = s(c.f[9] - c.f[7]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[10] * c.f[8]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(c.f[1] + c.f[11]); c.f[11] = v; c.ps1[11] = v; }
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bb4); c.f[7] = c.lfs(ea); c.ps1[7] = c.f[7]; }
                { let v = s(c.f[5] + c.f[2]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(c.f[6] - c.f[4]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[12] + c.f[11]); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bcc); c.f[8] = c.lfs(ea); c.ps1[8] = c.f[8]; }
                { let v = s(c.f[7] + c.f[5]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[2] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.fcmp(0, c.f[9], c.f[8]);
                { let v = s(c.f[12] + c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[7] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[12] + c.f[0]); c.f[2] = v; c.ps1[2] = v; }
                if !c.crbit(1) { pc = 0x8009d0f4; continue; }
                pc = 0x8009d0f0;
            }
            0x8009d0f0 => {
                pc = 0x8009d108; continue;
                pc = 0x8009d0f4;
            }
            0x8009d0f4 => {
                c.fcmp(0, c.f[9], c.f[1]);
                if !c.crbit(0) { pc = 0x8009d100; continue; }
                pc = 0x8009d0fc;
            }
            0x8009d0fc => {
                pc = 0x8009d104; continue;
                pc = 0x8009d100;
            }
            0x8009d100 => {
                c.f[1] = c.f[9];
                pc = 0x8009d104;
            }
            0x8009d104 => {
                c.f[8] = c.f[1];
                pc = 0x8009d108;
            }
            0x8009d108 => {
                c.f[0] = fctiwz(c.f[8]);
                { let ea = c.r[2].wrapping_add(0xffff8bd0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[3], c.f[1]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.stb(c.r[1].wrapping_add(0x18), c.r[0]);
                if !c.crbit(1) { pc = 0x8009d128; continue; }
                pc = 0x8009d124;
            }
            0x8009d124 => {
                pc = 0x8009d13c; continue;
                pc = 0x8009d128;
            }
            0x8009d128 => {
                { let ea = c.r[2].wrapping_add(0xffff8ba4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[3], c.f[1]);
                if !c.crbit(0) { pc = 0x8009d138; continue; }
                pc = 0x8009d134;
            }
            0x8009d134 => {
                pc = 0x8009d13c; continue;
                pc = 0x8009d138;
            }
            0x8009d138 => {
                c.f[1] = c.f[3];
                pc = 0x8009d13c;
            }
            0x8009d13c => {
                c.f[0] = fctiwz(c.f[1]);
                { let ea = c.r[2].wrapping_add(0xffff8bd0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[2], c.f[1]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.stb(c.r[1].wrapping_add(0x19), c.r[0]);
                if !c.crbit(1) { pc = 0x8009d15c; continue; }
                pc = 0x8009d158;
            }
            0x8009d158 => {
                pc = 0x8009d170; continue;
                pc = 0x8009d15c;
            }
            0x8009d15c => {
                { let ea = c.r[2].wrapping_add(0xffff8ba4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[2], c.f[1]);
                if !c.crbit(0) { pc = 0x8009d16c; continue; }
                pc = 0x8009d168;
            }
            0x8009d168 => {
                pc = 0x8009d170; continue;
                pc = 0x8009d16c;
            }
            0x8009d16c => {
                c.f[1] = c.f[2];
                pc = 0x8009d170;
            }
            0x8009d170 => {
                c.f[0] = fctiwz(c.f[1]);
                c.r[4] = 0x0;
                c.r[3] = c.r[29].wrapping_add(0x0);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.stb(c.r[1].wrapping_add(0x1a), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x1b), c.r[4]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x18));
                c.stw(c.r[1].wrapping_add(0x2c), c.r[0]);
                c.r[5] = c.lbz(c.r[1].wrapping_add(0x2d));
                c.r[6] = c.lbz(c.r[1].wrapping_add(0x2e));
                pc = 0x8009d19c;
            }
            0x8009d19c => {
                c.r[0] = 0x28;
                c.mtspr(9, c.r[0]);
                pc = 0x8009d1a4;
            }
            0x8009d1a4 => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3], c.r[0]);
                c.stb(c.r[3].wrapping_add(0x1), c.r[5]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x2), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x3), c.r[6]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x4), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x5), c.r[5]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x6), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x7), c.r[6]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x8), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x9), c.r[5]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0xa), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xb), c.r[6]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0xc), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xd), c.r[5]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0xe), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xf), c.r[6]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x10), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x11), c.r[5]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x12), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x13), c.r[6]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x14), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x15), c.r[5]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x16), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x17), c.r[6]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x18), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x19), c.r[5]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x1a), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x1b), c.r[6]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x1c), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x1d), c.r[5]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x2c));
                c.stb(c.r[3].wrapping_add(0x1e), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x1f), c.r[6]);
                c.r[3] = c.r[3].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009d1a4; continue; }
                pc = 0x8009d26c;
            }
            0x8009d26c => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.cmps(0, c.r[4], 0x1e0);
                if c.crbit(0) { pc = 0x8009d19c; continue; }
                pc = 0x8009d278;
            }
            0x8009d278 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009d280; f_800aa198(c);
                pc = 0x8009d280;
            }
            0x8009d280 => {
                c.r[3] = 0x280;
                c.r[4] = 0x1e0;
                c.lr = 0x8009d28c; f_8009cc3c(c);
                pc = 0x8009d28c;
            }
            0x8009d28c => {
                c.lr = 0x8009d290; f_800aa068(c);
                pc = 0x8009d290;
            }
            0x8009d290 => {
                c.lr = 0x8009d294; f_800aa280(c);
                pc = 0x8009d294;
            }
            0x8009d294 => {
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8009d298;
            }
            0x8009d298 => {
                c.lr = 0x8009d29c; f_800aa280(c);
                pc = 0x8009d29c;
            }
            0x8009d29c => {
                c.r[0] = c.r[3].wrapping_sub(c.r[28]);
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8009d298; continue; }
                pc = 0x8009d2a8;
            }
            0x8009d2a8 => {
                c.r[0] = c.lwz(c.r[31]);
                c.r[3] = 0x43300000;
                { let ea = c.r[2].wrapping_add(0xffff8bd8); c.f[0] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x20), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff8bac); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x20));
                c.r[4] = c.lbz(c.r[1].wrapping_add(0x21));
                c.stw(c.r[1].wrapping_add(0x3c), c.r[0]);
                c.r[5] = c.lbz(c.r[1].wrapping_add(0x22));
                c.stw(c.r[1].wrapping_add(0x44), c.r[4]);
                { let ea = c.r[2].wrapping_add(0xffff8bb0); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.stw(c.r[1].wrapping_add(0x38), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff8ba8); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                c.stw(c.r[1].wrapping_add(0x40), c.r[3]);
                { let ea = c.r[1].wrapping_add(0x38); c.f[3] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x34), c.r[5]);
                { let ea = c.r[1].wrapping_add(0x40); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[3] - c.f[0]); c.f[3] = v; c.ps1[3] = v; }
                c.stw(c.r[1].wrapping_add(0x30), c.r[3]);
                { let v = s(c.f[1] - c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bc0); c.f[8] = c.lfs(ea); c.ps1[8] = c.f[8]; }
                c.stw(c.r[1].wrapping_add(0x5c), c.r[4]);
                { let ea = c.r[1].wrapping_add(0x30); c.f[5] = c.lfd(ea); }
                { let v = s(c.f[4] * c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[5] - c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bbc); c.f[9] = c.lfs(ea); c.ps1[9] = c.f[9]; }
                c.stw(c.r[1].wrapping_add(0x50), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                { let v = s(c.f[6] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x6c), c.r[4]);
                { let v = s(c.f[2] - c.f[0]); c.f[3] = v; c.ps1[3] = v; }
                c.stw(c.r[1].wrapping_add(0x4c), c.r[5]);
                { let v = s(c.f[4] + c.f[1]); c.f[11] = v; c.ps1[11] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bb8); c.f[10] = c.lfs(ea); c.ps1[10] = c.f[10]; }
                c.stw(c.r[1].wrapping_add(0x58), c.r[3]);
                { let v = s(c.f[9] * c.f[3]); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bc4); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[1].wrapping_add(0x58); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x60), c.r[3]);
                { let v = s(c.f[1] - c.f[0]); c.f[7] = v; c.ps1[7] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bc8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.stw(c.r[1].wrapping_add(0x68), c.r[3]);
                { let ea = c.r[1].wrapping_add(0x60); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x74), c.r[5]);
                { let ea = c.r[1].wrapping_add(0x68); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[2] - c.f[0]); c.f[6] = v; c.ps1[6] = v; }
                c.stw(c.r[1].wrapping_add(0x70), c.r[3]);
                { let v = s(c.f[8] * c.f[7]); c.f[7] = v; c.ps1[7] = v; }
                { let v = s(c.f[1] - c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                c.stw(c.r[1].wrapping_add(0x48), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff8ba4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[10] * c.f[6]); c.f[6] = v; c.ps1[6] = v; }
                { let ea = c.r[1].wrapping_add(0x48); c.f[8] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x70); c.f[2] = c.lfd(ea); }
                { let v = s(c.f[5] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[8] - c.f[0]); c.f[8] = v; c.ps1[8] = v; }
                { let v = s(c.f[2] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8ba0); c.f[12] = c.lfs(ea); c.ps1[12] = c.f[12]; }
                { let v = s(c.f[9] - c.f[7]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[10] * c.f[8]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(c.f[1] + c.f[11]); c.f[11] = v; c.ps1[11] = v; }
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bb4); c.f[7] = c.lfs(ea); c.ps1[7] = c.f[7]; }
                { let v = s(c.f[5] + c.f[2]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(c.f[6] - c.f[4]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[12] + c.f[11]); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8bcc); c.f[8] = c.lfs(ea); c.ps1[8] = c.f[8]; }
                { let v = s(c.f[7] + c.f[5]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[2] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.fcmp(0, c.f[9], c.f[8]);
                { let v = s(c.f[12] + c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[7] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[12] + c.f[0]); c.f[2] = v; c.ps1[2] = v; }
                if !c.crbit(1) { pc = 0x8009d3e4; continue; }
                pc = 0x8009d3e0;
            }
            0x8009d3e0 => {
                pc = 0x8009d3f8; continue;
                pc = 0x8009d3e4;
            }
            0x8009d3e4 => {
                c.fcmp(0, c.f[9], c.f[1]);
                if !c.crbit(0) { pc = 0x8009d3f0; continue; }
                pc = 0x8009d3ec;
            }
            0x8009d3ec => {
                pc = 0x8009d3f4; continue;
                pc = 0x8009d3f0;
            }
            0x8009d3f0 => {
                c.f[1] = c.f[9];
                pc = 0x8009d3f4;
            }
            0x8009d3f4 => {
                c.f[8] = c.f[1];
                pc = 0x8009d3f8;
            }
            0x8009d3f8 => {
                c.f[0] = fctiwz(c.f[8]);
                { let ea = c.r[2].wrapping_add(0xffff8bd0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[3], c.f[1]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.stb(c.r[1].wrapping_add(0x1c), c.r[0]);
                if !c.crbit(1) { pc = 0x8009d418; continue; }
                pc = 0x8009d414;
            }
            0x8009d414 => {
                pc = 0x8009d42c; continue;
                pc = 0x8009d418;
            }
            0x8009d418 => {
                { let ea = c.r[2].wrapping_add(0xffff8ba4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[3], c.f[1]);
                if !c.crbit(0) { pc = 0x8009d428; continue; }
                pc = 0x8009d424;
            }
            0x8009d424 => {
                pc = 0x8009d42c; continue;
                pc = 0x8009d428;
            }
            0x8009d428 => {
                c.f[1] = c.f[3];
                pc = 0x8009d42c;
            }
            0x8009d42c => {
                c.f[0] = fctiwz(c.f[1]);
                { let ea = c.r[2].wrapping_add(0xffff8bd0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[2], c.f[1]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.stb(c.r[1].wrapping_add(0x1d), c.r[0]);
                if !c.crbit(1) { pc = 0x8009d44c; continue; }
                pc = 0x8009d448;
            }
            0x8009d448 => {
                pc = 0x8009d460; continue;
                pc = 0x8009d44c;
            }
            0x8009d44c => {
                { let ea = c.r[2].wrapping_add(0xffff8ba4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[2], c.f[1]);
                if !c.crbit(0) { pc = 0x8009d45c; continue; }
                pc = 0x8009d458;
            }
            0x8009d458 => {
                pc = 0x8009d460; continue;
                pc = 0x8009d45c;
            }
            0x8009d45c => {
                c.f[1] = c.f[2];
                pc = 0x8009d460;
            }
            0x8009d460 => {
                c.f[0] = fctiwz(c.f[1]);
                c.r[0] = 0x0;
                c.r[6] = c.r[1].wrapping_add(0x24);
                c.r[3] = c.r[29].wrapping_add(0x0);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[4] = 0x280;
                c.r[7] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[5] = 0x1e0;
                c.r[8] = 0x64;
                c.stb(c.r[1].wrapping_add(0x1e), c.r[7]);
                c.r[7] = 0x30;
                c.stb(c.r[1].wrapping_add(0x1f), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[9] = c.lhz(c.r[30].wrapping_add(0xe));
                c.r[10] = c.lwz(c.r[31].wrapping_add(0x8));
                c.lr = 0x8009d4a4; f_8009c860(c);
                pc = 0x8009d4a4;
            }
            0x8009d4a4 => {
                c.r[4] = 0x90000;
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = c.r[4].wrapping_add(0x6000);
                c.lr = 0x8009d4b4; f_8009b2d0(c);
                pc = 0x8009d4b4;
            }
            0x8009d4b4 => {
                c.r[3] = 0x0;
                c.lr = 0x8009d4bc; f_800aa204(c);
                pc = 0x8009d4bc;
            }
            0x8009d4bc => {
                c.lr = 0x8009d4c0; f_800aa068(c);
                pc = 0x8009d4c0;
            }
            0x8009d4c0 => {
                c.lr = 0x8009d4c4; f_800aa280(c);
                pc = 0x8009d4c4;
            }
            0x8009d4c4 => {
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x8009d4c8;
            }
            0x8009d4c8 => {
                c.lr = 0x8009d4cc; f_800aa280(c);
                pc = 0x8009d4cc;
            }
            0x8009d4cc => {
                c.r[0] = c.r[3].wrapping_sub(c.r[29]);
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(0) { pc = 0x8009d4c8; continue; }
                pc = 0x8009d4d8;
            }
            0x8009d4d8 => {
                c.lr = 0x8009d4dc; f_8009df3c(c);
                pc = 0x8009d4dc;
            }
            0x8009d4dc => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x8));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.r[3] = c.r[13].wrapping_add(0xffff8178);
                c.lr = 0x8009d4ec; f_8009c2e0(c);
                pc = 0x8009d4ec;
            }
            0x8009d4ec => {
                c.lr = 0x8009d4f0; f_8009803c(c);
                pc = 0x8009d4f0;
            }
            0x8009d4f0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x8c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x84));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x80));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x7c));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x78));
                c.r[1] = c.r[1].wrapping_add(0x88);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009ce8c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009D510 (0x8009d510, 412 bytes)
#[inline(never)]
pub fn f_8009d510(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009d510) { return; }
        if c.prof { crate::prof::enter(c, 0x8009d510); f_8009d510_b(c, 0x8009d510); crate::prof::exit(c, 0x8009d510); return; }
    }
    f_8009d510_b(c, 0x8009d510)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009d510_at(c: &mut Cpu, pc: u32) { f_8009d510_b(c, pc) }
#[inline(always)]
fn f_8009d510_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009d510;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009d510, pc); }
        match pc {
            0x8009d510 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8009d528; f_8009d820(c);
                pc = 0x8009d528;
            }
            0x8009d528 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8009d678; continue; }
                pc = 0x8009d534;
            }
            0x8009d534 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x20);
                if c.crbit(0) { pc = 0x8009d560; continue; }
                pc = 0x8009d540;
            }
            0x8009d540 => {
                c.cmpu(0, c.r[0], 0xdf);
                if c.crbit(1) { pc = 0x8009d560; continue; }
                pc = 0x8009d548;
            }
            0x8009d548 => {
                c.r[3] = 0x80310000;
                c.r[3] = c.r[3].wrapping_add(0xcd8);
                c.r[0] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz(c.r[3].wrapping_add(0xffffffc0));
                pc = 0x8009d698; continue;
                pc = 0x8009d560;
            }
            0x8009d560 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x889e);
                if !c.crbit(1) { pc = 0x8009d5e4; continue; }
                pc = 0x8009d56c;
            }
            0x8009d56c => {
                c.cmpu(0, c.r[0], 0x9872);
                if c.crbit(1) { pc = 0x8009d5e4; continue; }
                pc = 0x8009d574;
            }
            0x8009d574 => {
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 24 != 0); c.r[3] = (v >> 8) as u32; }
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_add(0xffffff78);
                c.cmpu(0, c.r[4], 0x40);
                c.r[5] = c.r[0].wrapping_mul(0xbc);
                c.r[3] = 0x0;
                c.r[0] = c.r[3].wrapping_add(0x0);
                if c.crbit(0) { pc = 0x8009d5a0; continue; }
                pc = 0x8009d594;
            }
            0x8009d594 => {
                c.cmpu(0, c.r[4], 0xfc);
                if c.crbit(1) { pc = 0x8009d5a0; continue; }
                pc = 0x8009d59c;
            }
            0x8009d59c => {
                c.r[0] = 0x1;
                pc = 0x8009d5a0;
            }
            0x8009d5a0 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009d5b8; continue; }
                pc = 0x8009d5a8;
            }
            0x8009d5a8 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7f);
                if c.crbit(2) { pc = 0x8009d5b8; continue; }
                pc = 0x8009d5b4;
            }
            0x8009d5b4 => {
                c.r[3] = 0x1;
                pc = 0x8009d5b8;
            }
            0x8009d5b8 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009d5c8; continue; }
                pc = 0x8009d5c0;
            }
            0x8009d5c0 => {
                c.r[3] = 0x0;
                pc = 0x8009d698; continue;
                pc = 0x8009d5c8;
            }
            0x8009d5c8 => {
                c.r[3] = c.r[4].wrapping_add(0xffffffc0);
                c.cmps(0, c.r[3], 0x40);
                if c.crbit(0) { pc = 0x8009d5d8; continue; }
                pc = 0x8009d5d4;
            }
            0x8009d5d4 => {
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x8009d5d8;
            }
            0x8009d5d8 => {
                c.r[3] = c.r[3].wrapping_add(0x2be);
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                pc = 0x8009d698; continue;
                pc = 0x8009d5e4;
            }
            0x8009d5e4 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x8140);
                if c.crbit(0) { pc = 0x8009d694; continue; }
                pc = 0x8009d5f0;
            }
            0x8009d5f0 => {
                c.cmpu(0, c.r[0], 0x879e);
                if !c.crbit(0) { pc = 0x8009d694; continue; }
                pc = 0x8009d5f8;
            }
            0x8009d5f8 => {
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 24 != 0); c.r[3] = (v >> 8) as u32; }
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_add(0xffffff7f);
                c.cmpu(0, c.r[4], 0x40);
                c.r[5] = c.r[0].wrapping_mul(0xbc);
                c.r[3] = 0x0;
                c.r[0] = c.r[3].wrapping_add(0x0);
                if c.crbit(0) { pc = 0x8009d624; continue; }
                pc = 0x8009d618;
            }
            0x8009d618 => {
                c.cmpu(0, c.r[4], 0xfc);
                if c.crbit(1) { pc = 0x8009d624; continue; }
                pc = 0x8009d620;
            }
            0x8009d620 => {
                c.r[0] = 0x1;
                pc = 0x8009d624;
            }
            0x8009d624 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009d63c; continue; }
                pc = 0x8009d62c;
            }
            0x8009d62c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7f);
                if c.crbit(2) { pc = 0x8009d63c; continue; }
                pc = 0x8009d638;
            }
            0x8009d638 => {
                c.r[3] = 0x1;
                pc = 0x8009d63c;
            }
            0x8009d63c => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009d64c; continue; }
                pc = 0x8009d644;
            }
            0x8009d644 => {
                c.r[3] = 0x0;
                pc = 0x8009d698; continue;
                pc = 0x8009d64c;
            }
            0x8009d64c => {
                c.r[3] = c.r[4].wrapping_add(0xffffffc0);
                c.cmps(0, c.r[3], 0x40);
                if c.crbit(0) { pc = 0x8009d65c; continue; }
                pc = 0x8009d658;
            }
            0x8009d658 => {
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x8009d65c;
            }
            0x8009d65c => {
                c.r[0] = c.r[5].wrapping_add(c.r[3]);
                c.r[3] = 0x80310000;
                c.r[4] = c.r[0].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.r[3].wrapping_add(0xe58);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.lhz(c.r[3]);
                pc = 0x8009d698; continue;
                pc = 0x8009d678;
            }
            0x8009d678 => {
                c.r[3] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], 0x20);
                if !c.crbit(1) { pc = 0x8009d694; continue; }
                pc = 0x8009d684;
            }
            0x8009d684 => {
                c.cmpu(0, c.r[3], 0xff);
                if c.crbit(1) { pc = 0x8009d694; continue; }
                pc = 0x8009d68c;
            }
            0x8009d68c => {
                c.r[3] = c.r[3].wrapping_add(0xffffffe0);
                pc = 0x8009d698; continue;
                pc = 0x8009d694;
            }
            0x8009d694 => {
                c.r[3] = 0x0;
                pc = 0x8009d698;
            }
            0x8009d698 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009d510: bad pc {:#x}", pc),
        }
    }
}

/// Decode (0x8009d6ac, 372 bytes)
#[inline(never)]
pub fn f_8009d6ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009d6ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8009d6ac); f_8009d6ac_b(c, 0x8009d6ac); crate::prof::exit(c, 0x8009d6ac); return; }
    }
    f_8009d6ac_b(c, 0x8009d6ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009d6ac_at(c: &mut Cpu, pc: u32) { f_8009d6ac_b(c, pc) }
#[inline(always)]
fn f_8009d6ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009d6ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009d6ac, pc); }
        match pc {
            0x8009d6ac => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[6] = c.r[3].wrapping_add(0x10);
                c.r[10] = 0x0;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[31] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[11] = c.lwz(c.r[3].wrapping_add(0x8));
                c.r[12] = c.lwz(c.r[3].wrapping_add(0xc));
                pc = 0x8009d6d4;
            }
            0x8009d6d4 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8009d6e8; continue; }
                pc = 0x8009d6dc;
            }
            0x8009d6dc => {
                c.r[29] = c.lwz(c.r[6]);
                c.r[30] = 0x20;
                c.r[6] = c.r[6].wrapping_add(0x4);
                pc = 0x8009d6e8;
            }
            0x8009d6e8 => {
                c.r[0] = c.r[29].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009d70c; continue; }
                pc = 0x8009d6f0;
            }
            0x8009d6f0 => {
                c.r[0] = c.r[12] | c.r[12];
                c.r[5] = c.lbz(c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.r[10] | c.r[10];
                c.r[10] = c.r[10].wrapping_add(0x1);
                c.stb(c.r[4].wrapping_add(c.r[0]), c.r[5]);
                c.r[12] = c.r[12].wrapping_add(0x1);
                pc = 0x8009d7fc; continue;
                pc = 0x8009d70c;
            }
            0x8009d70c => {
                c.r[7] = c.r[3].wrapping_add(c.r[11]);
                c.r[5] = c.lbz(c.r[7]);
                c.r[11] = c.r[11].wrapping_add(0x2);
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x1));
                c.r[0] = (c.r[5].rotate_left(8) & 0xff00) | (c.r[0] & 0xffff00ff);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 20 != 0); c.r[5] = (v >> 12) as u32; }
                c.rc(c.r[5]);
                c.r[0] = c.r[0].rotate_left(0) & 0xfff;
                c.r[9] = c.r[10].wrapping_sub(c.r[0]);
                if !c.crbit(2) { pc = 0x8009d744; continue; }
                pc = 0x8009d730;
            }
            0x8009d730 => {
                c.r[0] = c.r[12] | c.r[12];
                c.r[5] = c.lbz(c.r[3].wrapping_add(c.r[0]));
                c.r[12] = c.r[12].wrapping_add(0x1);
                c.r[0] = c.r[5].wrapping_add(0x12);
                pc = 0x8009d748; continue;
                pc = 0x8009d744;
            }
            0x8009d744 => {
                c.r[0] = c.r[5].wrapping_add(0x2);
                pc = 0x8009d748;
            }
            0x8009d748 => {
                c.cmps(0, c.r[0], 0x0);
                c.r[5] = c.r[0] | c.r[0];
                c.r[7] = c.r[4].wrapping_add(c.r[10]);
                if !c.crbit(1) { pc = 0x8009d7fc; continue; }
                pc = 0x8009d758;
            }
            0x8009d758 => {
                c.r[0] = c.r[5].rotate_left(29) & 0x1fffffff;
                c.rc(c.r[0]);
                c.mtspr(9, c.r[0]);
                if c.crbit(2) { pc = 0x8009d7dc; continue; }
                pc = 0x8009d764;
            }
            0x8009d764 => {
                c.r[8] = c.r[9].wrapping_add(0xffffffff);
                c.r[0] = c.lbz(c.r[4].wrapping_add(c.r[8]));
                c.r[8] = c.r[9].wrapping_add(0x0);
                c.r[10] = c.r[10].wrapping_add(0x8);
                c.stb(c.r[7], c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(c.r[8]));
                c.r[8] = c.r[9].wrapping_add(0x1);
                c.stb(c.r[7].wrapping_add(0x1), c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(c.r[8]));
                c.r[8] = c.r[9].wrapping_add(0x2);
                c.stb(c.r[7].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(c.r[8]));
                c.r[8] = c.r[9].wrapping_add(0x3);
                c.stb(c.r[7].wrapping_add(0x3), c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(c.r[8]));
                c.r[8] = c.r[9].wrapping_add(0x4);
                c.stb(c.r[7].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(c.r[8]));
                c.r[8] = c.r[9].wrapping_add(0x5);
                c.stb(c.r[7].wrapping_add(0x5), c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(c.r[8]));
                c.r[8] = c.r[9].wrapping_add(0x6);
                c.r[9] = c.r[9].wrapping_add(0x8);
                c.stb(c.r[7].wrapping_add(0x6), c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(c.r[8]));
                c.stb(c.r[7].wrapping_add(0x7), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0x8);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009d764; continue; }
                pc = 0x8009d7d4;
            }
            0x8009d7d4 => {
                c.r[5] = c.r[5] & 0x7;
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x8009d7fc; continue; }
                pc = 0x8009d7dc;
            }
            0x8009d7dc => {
                c.mtspr(9, c.r[5]);
                pc = 0x8009d7e0;
            }
            0x8009d7e0 => {
                c.r[8] = c.r[9].wrapping_add(0xffffffff);
                c.r[0] = c.lbz(c.r[4].wrapping_add(c.r[8]));
                c.r[10] = c.r[10].wrapping_add(0x1);
                c.r[9] = c.r[9].wrapping_add(0x1);
                c.stb(c.r[7], c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009d7e0; continue; }
                pc = 0x8009d7fc;
            }
            0x8009d7fc => {
                c.cmps(0, c.r[10], c.r[31]);
                c.r[29] = c.r[29].rotate_left(1) & 0xfffffffe;
                c.r[30] = c.r[30].wrapping_add(0xffffffff);
                if c.crbit(0) { pc = 0x8009d6d4; continue; }
                pc = 0x8009d80c;
            }
            0x8009d80c => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8009d6ac: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009D820 (0x8009d820, 88 bytes)
#[inline(never)]
pub fn f_8009d820(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009d820) { return; }
        if c.prof { crate::prof::enter(c, 0x8009d820); f_8009d820_b(c, 0x8009d820); crate::prof::exit(c, 0x8009d820); return; }
    }
    f_8009d820_b(c, 0x8009d820)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009d820_at(c: &mut Cpu, pc: u32) { f_8009d820_b(c, pc) }
#[inline(always)]
fn f_8009d820_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009d820;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009d820, pc); }
        match pc {
            0x8009d820 => {
                c.r[3] = c.lhz(c.r[13].wrapping_add(0xffff8180));
                c.cmpu(0, c.r[3], 0x1);
                if !c.crbit(1) { return; }
                pc = 0x8009d82c;
            }
            0x8009d82c => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xcc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009d844; continue; }
                pc = 0x8009d83c;
            }
            0x8009d83c => {
                if c.crbit(0) { pc = 0x8009d868; continue; }
                pc = 0x8009d840;
            }
            0x8009d840 => {
                pc = 0x8009d868; continue;
                pc = 0x8009d844;
            }
            0x8009d844 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x206e));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009d85c; continue; }
                pc = 0x8009d854;
            }
            0x8009d854 => {
                c.r[0] = 0x1;
                pc = 0x8009d860; continue;
                pc = 0x8009d85c;
            }
            0x8009d85c => {
                c.r[0] = 0x0;
                pc = 0x8009d860;
            }
            0x8009d860 => {
                c.sth(c.r[13].wrapping_add(0xffff8180), c.r[0]);
                pc = 0x8009d870; continue;
                pc = 0x8009d868;
            }
            0x8009d868 => {
                c.r[0] = 0x0;
                c.sth(c.r[13].wrapping_add(0xffff8180), c.r[0]);
                pc = 0x8009d870;
            }
            0x8009d870 => {
                c.r[3] = c.lhz(c.r[13].wrapping_add(0xffff8180));
                if true { return; }
                return;
            }
            _ => panic!("f_8009d820: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009D878 (0x8009d878, 140 bytes)
#[inline(never)]
pub fn f_8009d878(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009d878) { return; }
        if c.prof { crate::prof::enter(c, 0x8009d878); f_8009d878_b(c, 0x8009d878); crate::prof::exit(c, 0x8009d878); return; }
    }
    f_8009d878_b(c, 0x8009d878)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009d878_at(c: &mut Cpu, pc: u32) { f_8009d878_b(c, pc) }
#[inline(always)]
fn f_8009d878_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009d878;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009d878, pc); }
        match pc {
            0x8009d878 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x18), c.r[28]);
                c.r[28] = c.r[3].wrapping_add(0x0);
                pc = 0x8009d8dc; continue;
                pc = 0x8009d8a4;
            }
            0x8009d8a4 => {
                c.cmps(0, c.r[29], 0x100);
                if c.crbit(1) { pc = 0x8009d8b4; continue; }
                pc = 0x8009d8ac;
            }
            0x8009d8ac => {
                c.r[31] = c.r[29] | c.r[29];
                pc = 0x8009d8b8; continue;
                pc = 0x8009d8b4;
            }
            0x8009d8b4 => {
                c.r[31] = 0x100;
                pc = 0x8009d8b8;
            }
            0x8009d8b8 => {
                c.r[29] = c.r[29].wrapping_sub(c.r[31]);
                pc = 0x8009d8bc;
            }
            0x8009d8bc => {
                c.r[3] = c.r[28].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[5] = c.r[30].wrapping_add(0x0);
                c.lr = 0x8009d8cc; f_800a0d10(c);
                pc = 0x8009d8cc;
            }
            0x8009d8cc => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009d8bc; continue; }
                pc = 0x8009d8d4;
            }
            0x8009d8d4 => {
                c.r[30] = c.r[30].wrapping_add(c.r[31]);
                c.r[28] = c.r[28].wrapping_add(c.r[31]);
                pc = 0x8009d8dc;
            }
            0x8009d8dc => {
                c.cmps(0, c.r[29], 0x0);
                if c.crbit(1) { pc = 0x8009d8a4; continue; }
                pc = 0x8009d8e4;
            }
            0x8009d8e4 => {
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
            _ => panic!("f_8009d878: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009D904 (0x8009d904, 820 bytes)
#[inline(never)]
pub fn f_8009d904(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009d904) { return; }
        if c.prof { crate::prof::enter(c, 0x8009d904); f_8009d904_b(c, 0x8009d904); crate::prof::exit(c, 0x8009d904); return; }
    }
    f_8009d904_b(c, 0x8009d904)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009d904_at(c: &mut Cpu, pc: u32) { f_8009d904_b(c, pc) }
#[inline(always)]
fn f_8009d904_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009d904;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009d904, pc); }
        match pc {
            0x8009d904 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x3c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x38), c.r[30]);
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x34), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[13].wrapping_add(0xffff9ee4), c.r[0]);
                c.lr = 0x8009d930; f_8009d820(c);
                pc = 0x8009d930;
            }
            0x8009d930 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8009d958; continue; }
                pc = 0x8009d93c;
            }
            0x8009d93c => {
                c.r[4] = 0x50000;
                c.r[5] = 0x1b0000;
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.r[4] = c.r[4].wrapping_add(0xffffd000);
                c.r[5] = c.r[5].wrapping_add(0xffffff00);
                c.lr = 0x8009d954; f_8009d878(c);
                pc = 0x8009d954;
            }
            0x8009d954 => {
                pc = 0x8009d96c; continue;
                pc = 0x8009d958;
            }
            0x8009d958 => {
                c.r[4] = 0x200000;
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.r[5] = c.r[4].wrapping_add(0xffffcf00);
                c.r[4] = 0x3000;
                c.lr = 0x8009d96c; f_8009d878(c);
                pc = 0x8009d96c;
            }
            0x8009d96c => {
                c.r[0] = c.lbz(c.r[30]);
                c.cmpu(0, c.r[0], 0x59);
                if !c.crbit(2) { pc = 0x8009d998; continue; }
                pc = 0x8009d978;
            }
            0x8009d978 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x61);
                if !c.crbit(2) { pc = 0x8009d998; continue; }
                pc = 0x8009d984;
            }
            0x8009d984 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x2));
                c.cmpu(0, c.r[0], 0x79);
                if !c.crbit(2) { pc = 0x8009d998; continue; }
                pc = 0x8009d990;
            }
            0x8009d990 => {
                c.r[31] = c.lwz(c.r[30].wrapping_add(0x4));
                pc = 0x8009d99c; continue;
                pc = 0x8009d998;
            }
            0x8009d998 => {
                c.r[31] = 0x0;
                pc = 0x8009d99c;
            }
            0x8009d99c => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8009dc18; continue; }
                pc = 0x8009d9a4;
            }
            0x8009d9a4 => {
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.lr = 0x8009d9b0; f_8009d6ac(c);
                pc = 0x8009d9b0;
            }
            0x8009d9b0 => {
                c.stw(c.r[13].wrapping_add(0xffff9ee0), c.r[29]);
                c.r[4] = c.lhz(c.r[13].wrapping_add(0xffff8180));
                c.r[0] = c.lhz(c.r[29].wrapping_add(0x22));
                c.cmpu(0, c.r[4], 0x1);
                c.r[0] = c.r[29].wrapping_add(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9ee8), c.r[0]);
                c.r[3] = c.lhz(c.r[29].wrapping_add(0x1a));
                c.r[0] = c.lhz(c.r[29].wrapping_add(0x1c));
                c.r[0] = c.r[3].wrapping_mul(c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9eec), c.r[0]);
                if c.crbit(1) { pc = 0x8009d9e0; continue; }
                pc = 0x8009d9dc;
            }
            0x8009d9dc => {
                pc = 0x8009da28; continue;
                pc = 0x8009d9e0;
            }
            0x8009d9e0 => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xcc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009d9f8; continue; }
                pc = 0x8009d9f0;
            }
            0x8009d9f0 => {
                if c.crbit(0) { pc = 0x8009da1c; continue; }
                pc = 0x8009d9f4;
            }
            0x8009d9f4 => {
                pc = 0x8009da1c; continue;
                pc = 0x8009d9f8;
            }
            0x8009d9f8 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x206e));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009da10; continue; }
                pc = 0x8009da08;
            }
            0x8009da08 => {
                c.r[0] = 0x1;
                pc = 0x8009da14; continue;
                pc = 0x8009da10;
            }
            0x8009da10 => {
                c.r[0] = 0x0;
                pc = 0x8009da14;
            }
            0x8009da14 => {
                c.sth(c.r[13].wrapping_add(0xffff8180), c.r[0]);
                pc = 0x8009da24; continue;
                pc = 0x8009da1c;
            }
            0x8009da1c => {
                c.r[0] = 0x0;
                c.sth(c.r[13].wrapping_add(0xffff8180), c.r[0]);
                pc = 0x8009da24;
            }
            0x8009da24 => {
                c.r[4] = c.lhz(c.r[13].wrapping_add(0xffff8180));
                pc = 0x8009da28;
            }
            0x8009da28 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8009dc18; continue; }
                pc = 0x8009da34;
            }
            0x8009da34 => {
                c.r[4] = c.lwz(c.r[2].wrapping_add(0xffff8be0));
                c.r[3] = 0x54;
                c.r[0] = c.lwz(c.r[2].wrapping_add(0xffff8be4));
                c.stw(c.r[1].wrapping_add(0x1c), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[0]);
                c.lr = 0x8009da4c; f_8009d510(c);
                pc = 0x8009da4c;
            }
            0x8009da4c => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9eec));
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9ee0));
                c.r[10] = divw(c.r[3], c.r[5]);
                c.r[6] = c.lhz(c.r[1].wrapping_add(0x1c));
                c.r[0] = c.lhz(c.r[12].wrapping_add(0x1e));
                c.r[4] = c.lwz(c.r[12].wrapping_add(0x14));
                c.r[11] = c.lwz(c.r[12].wrapping_add(0x24));
                c.r[8] = c.r[10].wrapping_mul(c.r[5]);
                c.r[9] = c.lhz(c.r[12].wrapping_add(0x1a));
                c.r[5] = c.lhz(c.r[12].wrapping_add(0x12));
                c.r[7] = c.lhz(c.r[12].wrapping_add(0x10));
                c.r[30] = c.r[3].wrapping_sub(c.r[8]);
                c.r[8] = divw(c.r[30], c.r[9]);
                c.r[3] = c.r[8].wrapping_mul(c.r[9]);
                c.r[5] = c.r[8].wrapping_mul(c.r[5]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[0] = (v >> 3) as u32; }
                c.r[3] = c.r[30].wrapping_sub(c.r[3]);
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[0] = c.r[0].rotate_left(5) & 0xffffffe0;
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[9] = (v >> 1) as u32; }
                c.r[10] = c.r[10].wrapping_mul(c.r[4]);
                c.r[3] = c.r[3].wrapping_mul(c.r[7]);
                { let ci = c.ca(); c.r[9] = c.addc(c.r[9], 0, ci); }
                c.r[0] = c.r[5].wrapping_add(0x4);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[4] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[4] = c.addc(c.r[4], 0, ci); }
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[8] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[8] = c.addc(c.r[8], 0, ci); }
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[7] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[7] = c.addc(c.r[7], 0, ci); }
                c.r[7] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[7] = c.addc(!c.r[7], c.r[0], 1);
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[0] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.addc(!c.r[0], c.r[3], 1);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[3] = (v >> 2) as u32; }
                c.r[0] = c.r[9].wrapping_mul(c.r[4]);
                c.r[4] = c.r[12].wrapping_add(c.r[11]);
                c.r[9] = c.r[10].rotate_left(31) & 0x7fffffff;
                c.r[4] = c.r[4].wrapping_add(c.r[9]);
                c.r[9] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.r[8].rotate_left(4) & 0xfffffff0;
                c.r[9] = c.r[9].wrapping_add(c.r[0]);
                c.r[7] = c.r[7].rotate_left(1) & 0xfffffffe;
                c.r[9] = c.r[9].wrapping_add(c.r[7]);
                { let ci = c.ca(); c.r[3] = c.addc(c.r[3], 0, ci); }
                c.r[9] = c.r[9].wrapping_add(c.r[3]);
                c.sth(c.r[9], c.r[6]);
                c.r[10] = c.r[5].wrapping_add(0x5);
                c.r[9] = c.r[5].wrapping_add(0x6);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff9ee0));
                c.r[6] = c.r[5].wrapping_add(0x7);
                c.r[5] = c.lhz(c.r[1].wrapping_add(0x1e));
                c.r[7] = c.lhz(c.r[7].wrapping_add(0x1e));
                { let v = c.r[7] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[7] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[7] = c.addc(c.r[7], 0, ci); }
                c.r[7] = c.r[7].rotate_left(5) & 0xffffffe0;
                { let v = c.r[7] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[8] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[8] = c.addc(c.r[8], 0, ci); }
                { let v = c.r[10] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[7] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[7] = c.addc(c.r[7], 0, ci); }
                c.r[8] = c.r[8].wrapping_mul(c.r[7]);
                { let v = c.r[10] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[7] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[7] = c.addc(c.r[7], 0, ci); }
                c.r[7] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[8] = c.r[4].wrapping_add(c.r[8]);
                c.r[7] = c.addc(!c.r[7], c.r[10], 1);
                c.r[8] = c.r[8].wrapping_add(c.r[0]);
                c.r[7] = c.r[7].rotate_left(1) & 0xfffffffe;
                c.r[8] = c.r[8].wrapping_add(c.r[7]);
                c.r[8] = c.r[8].wrapping_add(c.r[3]);
                c.sth(c.r[8], c.r[5]);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff9ee0));
                c.r[5] = c.lhz(c.r[1].wrapping_add(0x20));
                c.r[7] = c.lhz(c.r[7].wrapping_add(0x1e));
                { let v = c.r[7] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[7] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[7] = c.addc(c.r[7], 0, ci); }
                c.r[7] = c.r[7].rotate_left(5) & 0xffffffe0;
                { let v = c.r[7] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[8] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[8] = c.addc(c.r[8], 0, ci); }
                { let v = c.r[9] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[7] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[7] = c.addc(c.r[7], 0, ci); }
                c.r[8] = c.r[8].wrapping_mul(c.r[7]);
                { let v = c.r[9] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[7] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[7] = c.addc(c.r[7], 0, ci); }
                c.r[7] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[8] = c.r[4].wrapping_add(c.r[8]);
                c.r[7] = c.addc(!c.r[7], c.r[9], 1);
                c.r[8] = c.r[8].wrapping_add(c.r[0]);
                c.r[7] = c.r[7].rotate_left(1) & 0xfffffffe;
                c.r[8] = c.r[8].wrapping_add(c.r[7]);
                c.r[8] = c.r[8].wrapping_add(c.r[3]);
                c.sth(c.r[8], c.r[5]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9ee0));
                c.r[5] = c.lhz(c.r[5].wrapping_add(0x1e));
                { let v = c.r[5] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[5] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[5] = c.addc(c.r[5], 0, ci); }
                c.r[5] = c.r[5].rotate_left(5) & 0xffffffe0;
                { let v = c.r[5] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[7] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[7] = c.addc(c.r[7], 0, ci); }
                { let v = c.r[6] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[5] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[5] = c.addc(c.r[5], 0, ci); }
                c.r[5] = c.r[7].wrapping_mul(c.r[5]);
                c.r[7] = c.r[4].wrapping_add(c.r[5]);
                { let v = c.r[6] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[5] = (v >> 3) as u32; }
                c.r[4] = c.lhz(c.r[1].wrapping_add(0x22));
                { let ci = c.ca(); c.r[5] = c.addc(c.r[5], 0, ci); }
                c.r[5] = c.r[5].rotate_left(3) & 0xfffffff8;
                c.r[5] = c.addc(!c.r[5], c.r[6], 1);
                c.r[7] = c.r[7].wrapping_add(c.r[0]);
                c.r[0] = c.r[5].rotate_left(1) & 0xfffffffe;
                c.r[7] = c.r[7].wrapping_add(c.r[0]);
                c.r[7] = c.r[7].wrapping_add(c.r[3]);
                c.sth(c.r[7], c.r[4]);
                pc = 0x8009dc18;
            }
            0x8009dc18 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x38));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009d904: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009DC38 (0x8009dc38, 772 bytes)
#[inline(never)]
pub fn f_8009dc38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009dc38) { return; }
        if c.prof { crate::prof::enter(c, 0x8009dc38); f_8009dc38_b(c, 0x8009dc38); crate::prof::exit(c, 0x8009dc38); return; }
    }
    f_8009dc38_b(c, 0x8009dc38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009dc38_at(c: &mut Cpu, pc: u32) { f_8009dc38_b(c, pc) }
#[inline(always)]
fn f_8009dc38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009dc38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009dc38, pc); }
        match pc {
            0x8009dc38 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 17..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4].wrapping_add(0x0);
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.r[24] = c.r[6].wrapping_add(0x0);
                c.r[31] = c.r[7].wrapping_add(0x0);
                c.r[0] = c.lbz(c.r[3]);
                c.cmpu(0, c.r[0], 0x0);
                c.r[3] = c.r[0] | c.r[0];
                if !c.crbit(2) { pc = 0x8009dc74; continue; }
                pc = 0x8009dc6c;
            }
            0x8009dc6c => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x8009df28; continue;
                pc = 0x8009dc74;
            }
            0x8009dc74 => {
                c.r[0] = c.lhz(c.r[13].wrapping_add(0xffff8180));
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(1) { pc = 0x8009dc88; continue; }
                pc = 0x8009dc84;
            }
            0x8009dc84 => {
                pc = 0x8009dcd0; continue;
                pc = 0x8009dc88;
            }
            0x8009dc88 => {
                c.r[4] = 0x80000000;
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xcc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009dca0; continue; }
                pc = 0x8009dc98;
            }
            0x8009dc98 => {
                if c.crbit(0) { pc = 0x8009dcc4; continue; }
                pc = 0x8009dc9c;
            }
            0x8009dc9c => {
                pc = 0x8009dcc4; continue;
                pc = 0x8009dca0;
            }
            0x8009dca0 => {
                c.r[4] = 0xcc000000;
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x206e));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009dcb8; continue; }
                pc = 0x8009dcb0;
            }
            0x8009dcb0 => {
                c.r[0] = 0x1;
                pc = 0x8009dcbc; continue;
                pc = 0x8009dcb8;
            }
            0x8009dcb8 => {
                c.r[0] = 0x0;
                pc = 0x8009dcbc;
            }
            0x8009dcbc => {
                c.sth(c.r[13].wrapping_add(0xffff8180), c.r[0]);
                pc = 0x8009dccc; continue;
                pc = 0x8009dcc4;
            }
            0x8009dcc4 => {
                c.r[0] = 0x0;
                c.sth(c.r[13].wrapping_add(0xffff8180), c.r[0]);
                pc = 0x8009dccc;
            }
            0x8009dccc => {
                c.r[0] = c.lhz(c.r[13].wrapping_add(0xffff8180));
                pc = 0x8009dcd0;
            }
            0x8009dcd0 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8009dd4c; continue; }
                pc = 0x8009dcdc;
            }
            0x8009dcdc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x81);
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                if c.crbit(0) { pc = 0x8009dcfc; continue; }
                pc = 0x8009dcf0;
            }
            0x8009dcf0 => {
                c.cmpu(0, c.r[0], 0x9f);
                if c.crbit(1) { pc = 0x8009dcfc; continue; }
                pc = 0x8009dcf8;
            }
            0x8009dcf8 => {
                c.r[5] = c.r[4] | c.r[4];
                pc = 0x8009dcfc;
            }
            0x8009dcfc => {
                c.cmps(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8009dd2c; continue; }
                pc = 0x8009dd04;
            }
            0x8009dd04 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0xe0);
                c.r[5] = 0x0;
                if c.crbit(0) { pc = 0x8009dd20; continue; }
                pc = 0x8009dd14;
            }
            0x8009dd14 => {
                c.cmpu(0, c.r[0], 0xfc);
                if c.crbit(1) { pc = 0x8009dd20; continue; }
                pc = 0x8009dd1c;
            }
            0x8009dd1c => {
                c.r[5] = 0x1;
                pc = 0x8009dd20;
            }
            0x8009dd20 => {
                c.cmps(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8009dd2c; continue; }
                pc = 0x8009dd28;
            }
            0x8009dd28 => {
                c.r[4] = 0x0;
                pc = 0x8009dd2c;
            }
            0x8009dd2c => {
                c.cmps(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x8009dd4c; continue; }
                pc = 0x8009dd34;
            }
            0x8009dd34 => {
                c.r[4] = c.lbz(c.r[28]);
                c.r[0] = c.r[4] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009dd4c; continue; }
                pc = 0x8009dd40;
            }
            0x8009dd40 => {
                c.r[0] = c.r[3].rotate_left(8) & 0xffff00;
                c.r[3] = c.r[0] | c.r[4];
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x8009dd4c;
            }
            0x8009dd4c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9ee0));
                c.r[25] = c.r[4].wrapping_add(0x2c);
                c.lr = 0x8009dd58; f_8009d510(c);
                pc = 0x8009dd58;
            }
            0x8009dd58 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff9eec));
                c.r[0] = c.r[24].rotate_left(2) & 0xfffffffc;
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9ee0));
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[0] = (v >> 3) as u32; }
                c.r[11] = divw(c.r[3], c.r[6]);
                c.r[4] = c.lwz(c.r[12].wrapping_add(0x14));
                c.r[5] = c.lwz(c.r[12].wrapping_add(0x24));
                c.r[9] = c.lhz(c.r[12].wrapping_add(0x1a));
                c.r[7] = c.lhz(c.r[12].wrapping_add(0x12));
                c.r[8] = c.r[11].wrapping_mul(c.r[6]);
                c.r[6] = c.lhz(c.r[12].wrapping_add(0x10));
                c.r[17] = c.r[3].wrapping_sub(c.r[8]);
                c.r[10] = divw(c.r[17], c.r[9]);
                c.r[8] = c.r[10].wrapping_mul(c.r[9]);
                c.r[4] = c.r[11].wrapping_mul(c.r[4]);
                c.r[11] = c.r[17].wrapping_sub(c.r[8]);
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[10] = c.r[10].wrapping_mul(c.r[7]);
                c.r[11] = c.r[11].wrapping_mul(c.r[6]);
                c.r[24] = c.r[12].wrapping_add(c.r[5]);
                c.r[4] = c.r[4].rotate_left(31) & 0x7fffffff;
                c.r[24] = c.r[24].wrapping_add(c.r[4]);
                c.r[7] = c.r[0].rotate_left(5) & 0xffffffe0;
                c.r[27] = 0x0;
                pc = 0x8009df00; continue;
                pc = 0x8009ddbc;
            }
            0x8009ddbc => {
                c.r[4] = c.r[10].wrapping_add(c.r[27]);
                { let v = c.r[4] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[0] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                { let v = c.r[4] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[8] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[8] = c.addc(c.r[8], 0, ci); }
                c.r[8] = c.r[8].rotate_left(3) & 0xfffffff8;
                c.r[8] = c.addc(!c.r[8], c.r[4], 1);
                { let v = c.r[27] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[4] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[4] = c.addc(c.r[4], 0, ci); }
                c.r[5] = c.r[4].wrapping_mul(c.r[7]);
                { let v = c.r[27] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[6] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[6] = c.addc(c.r[6], 0, ci); }
                c.r[6] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[6] = c.addc(!c.r[6], c.r[27], 1);
                c.r[4] = c.r[8].rotate_left(1) & 0xfffffffe;
                c.r[5] = c.r[29].wrapping_add(c.r[5]);
                c.r[6] = c.r[6].rotate_left(2) & 0xfffffffc;
                c.r[12] = 0x0;
                pc = 0x8009deec; continue;
                pc = 0x8009de08;
            }
            0x8009de08 => {
                c.r[9] = c.lhz(c.r[9].wrapping_add(0x1e));
                c.r[21] = c.r[11].wrapping_add(c.r[12]);
                c.r[8] = c.r[30].wrapping_add(c.r[12]);
                { let v = c.r[9] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[9] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[9] = c.addc(c.r[9], 0, ci); }
                c.r[9] = c.r[9].rotate_left(5) & 0xffffffe0;
                { let v = c.r[9] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[18] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[18] = c.addc(c.r[18], 0, ci); }
                { let v = c.r[21] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[19] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[19] = c.addc(c.r[19], 0, ci); }
                { let v = c.r[21] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[9] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[9] = c.addc(c.r[9], 0, ci); }
                c.r[9] = c.r[9].rotate_left(3) & 0xfffffff8;
                c.r[9] = c.addc(!c.r[9], c.r[21], 1);
                { let v = c.r[9] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[20] = (v >> 2) as u32; }
                { let ci = c.ca(); c.r[20] = c.addc(c.r[20], 0, ci); }
                { let v = c.r[21] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[26] = (v >> 2) as u32; }
                { let ci = c.ca(); c.r[26] = c.addc(c.r[26], 0, ci); }
                c.r[26] = c.r[26].rotate_left(2) & 0xfffffffc;
                c.r[26] = c.addc(!c.r[26], c.r[21], 1);
                { let v = c.r[8] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[21] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[21] = c.addc(c.r[21], 0, ci); }
                { let v = c.r[8] as i32; c.set_ca(v < 0 && (v as u32) << 29 != 0); c.r[9] = (v >> 3) as u32; }
                { let ci = c.ca(); c.r[9] = c.addc(c.r[9], 0, ci); }
                c.r[9] = c.r[9].rotate_left(3) & 0xfffffff8;
                c.r[18] = c.r[18].wrapping_mul(c.r[0]);
                c.r[9] = c.addc(!c.r[9], c.r[8], 1);
                { let v = c.r[9] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[22] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[22] = c.addc(c.r[22], 0, ci); }
                { let v = c.r[8] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[23] = (v >> 1) as u32; }
                { let ci = c.ca(); c.r[23] = c.addc(c.r[23], 0, ci); }
                c.r[9] = c.r[21].rotate_left(5) & 0xffffffe0;
                c.r[23] = c.r[23].rotate_left(1) & 0xfffffffe;
                c.r[9] = c.r[5].wrapping_add(c.r[9]);
                c.r[9] = c.r[9].wrapping_add(c.r[6]);
                c.r[17] = c.r[24].wrapping_add(c.r[18]);
                c.r[21] = c.r[19].rotate_left(4) & 0xfffffff0;
                c.r[17] = c.r[17].wrapping_add(c.r[21]);
                c.r[17] = c.r[17].wrapping_add(c.r[4]);
                c.r[23] = c.addc(!c.r[23], c.r[8], 1);
                c.rc(c.r[23]);
                c.r[17] = c.r[17].wrapping_add(c.r[20]);
                c.r[9] = c.r[9].wrapping_add(c.r[22]);
                if c.crbit(2) { pc = 0x8009debc; continue; }
                pc = 0x8009deb4;
            }
            0x8009deb4 => {
                c.r[18] = 0xf;
                pc = 0x8009dec0; continue;
                pc = 0x8009debc;
            }
            0x8009debc => {
                c.r[18] = 0xf0;
                pc = 0x8009dec0;
            }
            0x8009dec0 => {
                c.r[8] = c.r[26].rotate_left(1) & 0xfffffffe;
                c.r[26] = c.lbz(c.r[17]);
                c.r[8] = c.addc(!c.r[8], 0x6, 1);
                c.r[23] = c.lbz(c.r[9]);
                { let v = c.r[26] as i32; let n = c.r[8] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[8] = r as u32; }
                c.r[8] = c.r[8].rotate_left(0) & 0x3;
                c.r[8] = c.lbz(c.r[25].wrapping_add(c.r[8]));
                c.r[12] = c.r[12].wrapping_add(0x1);
                c.r[8] = c.r[8] & c.r[18];
                c.r[8] = c.r[23] | c.r[8];
                c.stb(c.r[9], c.r[8]);
                pc = 0x8009deec;
            }
            0x8009deec => {
                c.r[9] = c.lwz(c.r[13].wrapping_add(0xffff9ee0));
                c.r[8] = c.lhz(c.r[9].wrapping_add(0x10));
                c.cmps(0, c.r[12], c.r[8]);
                if c.crbit(0) { pc = 0x8009de08; continue; }
                pc = 0x8009defc;
            }
            0x8009defc => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x8009df00;
            }
            0x8009df00 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9ee0));
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x12));
                c.cmps(0, c.r[27], c.r[0]);
                if c.crbit(0) { pc = 0x8009ddbc; continue; }
                pc = 0x8009df10;
            }
            0x8009df10 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8009df24; continue; }
                pc = 0x8009df18;
            }
            0x8009df18 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9ee8));
                c.r[0] = c.lbz(c.r[4].wrapping_add(c.r[3]));
                c.stw(c.r[31], c.r[0]);
                pc = 0x8009df24;
            }
            0x8009df24 => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x8009df28;
            }
            0x8009df28 => {
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 17..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                c.r[1] = c.r[1].wrapping_add(0x60);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009dc38: bad pc {:#x}", pc),
        }
    }
}

/// OSDisableInterrupts (0x8009df3c, 20 bytes)
#[inline(never)]
pub fn f_8009df3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009df3c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009df3c); f_8009df3c_b(c, 0x8009df3c); crate::prof::exit(c, 0x8009df3c); return; }
    }
    f_8009df3c_b(c, 0x8009df3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009df3c_at(c: &mut Cpu, pc: u32) { f_8009df3c_b(c, pc) }
#[inline(always)]
fn f_8009df3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009df3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009df3c, pc); }
        match pc {
            0x8009df3c => {
                c.r[3] = 0; // mfmsr
                c.r[4] = c.r[3].rotate_left(0) & 0xffff7fff;
                // mtmsr
                c.r[3] = c.r[3].rotate_left(17) & 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8009df3c: bad pc {:#x}", pc),
        }
    }
}

/// OSEnableInterrupts (0x8009df50, 20 bytes)
#[inline(never)]
pub fn f_8009df50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009df50) { return; }
        if c.prof { crate::prof::enter(c, 0x8009df50); f_8009df50_b(c, 0x8009df50); crate::prof::exit(c, 0x8009df50); return; }
    }
    f_8009df50_b(c, 0x8009df50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009df50_at(c: &mut Cpu, pc: u32) { f_8009df50_b(c, pc) }
#[inline(always)]
fn f_8009df50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009df50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009df50, pc); }
        match pc {
            0x8009df50 => {
                c.r[3] = 0; // mfmsr
                c.r[4] = c.r[3] | 0x8000;
                // mtmsr
                c.r[3] = c.r[3].rotate_left(17) & 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8009df50: bad pc {:#x}", pc),
        }
    }
}

/// OSRestoreInterrupts (0x8009df64, 36 bytes)
#[inline(never)]
pub fn f_8009df64(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009df64) { return; }
        if c.prof { crate::prof::enter(c, 0x8009df64); f_8009df64_b(c, 0x8009df64); crate::prof::exit(c, 0x8009df64); return; }
    }
    f_8009df64_b(c, 0x8009df64)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009df64_at(c: &mut Cpu, pc: u32) { f_8009df64_b(c, pc) }
#[inline(always)]
fn f_8009df64_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009df64;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009df64, pc); }
        match pc {
            0x8009df64 => {
                c.cmps(0, c.r[3], 0x0);
                c.r[4] = 0; // mfmsr
                if c.crbit(2) { pc = 0x8009df78; continue; }
                pc = 0x8009df70;
            }
            0x8009df70 => {
                c.r[5] = c.r[4] | 0x8000;
                pc = 0x8009df7c; continue;
                pc = 0x8009df78;
            }
            0x8009df78 => {
                c.r[5] = c.r[4].rotate_left(0) & 0xffff7fff;
                pc = 0x8009df7c;
            }
            0x8009df7c => {
                // mtmsr
                c.r[3] = c.r[4].rotate_left(17) & 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_8009df64: bad pc {:#x}", pc),
        }
    }
}

/// __OSSetInterruptHandler (0x8009df88, 28 bytes)
#[inline(never)]
pub fn f_8009df88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009df88) { return; }
        if c.prof { crate::prof::enter(c, 0x8009df88); f_8009df88_b(c, 0x8009df88); crate::prof::exit(c, 0x8009df88); return; }
    }
    f_8009df88_b(c, 0x8009df88)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009df88_at(c: &mut Cpu, pc: u32) { f_8009df88_b(c, pc) }
#[inline(always)]
fn f_8009df88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009df88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009df88, pc); }
        match pc {
            0x8009df88 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ef0));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[5] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lwz(c.r[5]);
                c.stw(c.r[5], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009df88: bad pc {:#x}", pc),
        }
    }
}

/// __OSGetInterruptHandler (0x8009dfa4, 20 bytes)
#[inline(never)]
pub fn f_8009dfa4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009dfa4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009dfa4); f_8009dfa4_b(c, 0x8009dfa4); crate::prof::exit(c, 0x8009dfa4); return; }
    }
    f_8009dfa4_b(c, 0x8009dfa4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009dfa4_at(c: &mut Cpu, pc: u32) { f_8009dfa4_b(c, pc) }
#[inline(always)]
fn f_8009dfa4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009dfa4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009dfa4, pc); }
        match pc {
            0x8009dfa4 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ef0));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                if true { return; }
                return;
            }
            _ => panic!("f_8009dfa4: bad pc {:#x}", pc),
        }
    }
}

/// __OSInterruptInit (0x8009dfb8, 116 bytes)
#[inline(never)]
pub fn f_8009dfb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009dfb8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009dfb8); f_8009dfb8_b(c, 0x8009dfb8); crate::prof::exit(c, 0x8009dfb8); return; }
    }
    f_8009dfb8_b(c, 0x8009dfb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009dfb8_at(c: &mut Cpu, pc: u32) { f_8009dfb8_b(c, pc) }
#[inline(always)]
fn f_8009dfb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009dfb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009dfb8, pc); }
        match pc {
            0x8009dfb8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = 0x80000000;
                c.r[0] = c.r[31].wrapping_add(0x3040);
                c.stw(c.r[13].wrapping_add(0xffff9ef0), c.r[0]);
                c.r[4] = 0x0;
                c.r[5] = 0x80;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ef0));
                c.lr = 0x8009dfe4; f_8000540c(c);
                pc = 0x8009dfe4;
            }
            0x8009dfe4 => {
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0xc4), c.r[0]);
                c.r[3] = 0xcc000000;
                c.r[4] = c.r[3].wrapping_add(0x3000);
                c.stw(c.r[31].wrapping_add(0xc8), c.r[0]);
                c.r[0] = 0xf0;
                c.r[3] = 0xffffffe0;
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                c.lr = 0x8009e008; f_8009e304(c);
                pc = 0x8009e008;
            }
            0x8009e008 => {
                c.r[3] = 0x800a0000;
                c.r[4] = c.r[3].wrapping_add(0xffffe758);
                c.r[3] = 0x4;
                c.lr = 0x8009e018; f_8009a0c4(c);
                pc = 0x8009e018;
            }
            0x8009e018 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009dfb8: bad pc {:#x}", pc),
        }
    }
}

/// SetInterruptMask (0x8009e02c, 728 bytes)
#[inline(never)]
pub fn f_8009e02c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009e02c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009e02c); f_8009e02c_b(c, 0x8009e02c); crate::prof::exit(c, 0x8009e02c); return; }
    }
    f_8009e02c_b(c, 0x8009e02c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009e02c_at(c: &mut Cpu, pc: u32) { f_8009e02c_b(c, pc) }
#[inline(always)]
fn f_8009e02c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009e02c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009e02c, pc); }
        match pc {
            0x8009e02c => {
                c.r[0] = c.r[3].leading_zeros();
                c.cmps(0, c.r[0], 0xc);
                if !c.crbit(0) { pc = 0x8009e058; continue; }
                pc = 0x8009e038;
            }
            0x8009e038 => {
                c.cmps(0, c.r[0], 0x8);
                if c.crbit(2) { pc = 0x8009e138; continue; }
                pc = 0x8009e040;
            }
            0x8009e040 => {
                if !c.crbit(0) { pc = 0x8009e168; continue; }
                pc = 0x8009e044;
            }
            0x8009e044 => {
                c.cmps(0, c.r[0], 0x5);
                if !c.crbit(0) { pc = 0x8009e0e4; continue; }
                pc = 0x8009e04c;
            }
            0x8009e04c => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x8009e078; continue; }
                pc = 0x8009e054;
            }
            0x8009e054 => {
                pc = 0x8009e300; continue;
                pc = 0x8009e058;
            }
            0x8009e058 => {
                c.cmps(0, c.r[0], 0x11);
                if !c.crbit(0) { pc = 0x8009e06c; continue; }
                pc = 0x8009e060;
            }
            0x8009e060 => {
                c.cmps(0, c.r[0], 0xf);
                if !c.crbit(0) { pc = 0x8009e20c; continue; }
                pc = 0x8009e068;
            }
            0x8009e068 => {
                pc = 0x8009e1b8; continue;
                pc = 0x8009e06c;
            }
            0x8009e06c => {
                c.cmps(0, c.r[0], 0x1b);
                if !c.crbit(0) { pc = 0x8009e300; continue; }
                pc = 0x8009e074;
            }
            0x8009e074 => {
                pc = 0x8009e24c; continue;
                pc = 0x8009e078;
            }
            0x8009e078 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x80000000;
                c.cmpu(0, c.r[0], 0x0);
                c.r[5] = 0x0;
                if !c.crbit(2) { pc = 0x8009e08c; continue; }
                pc = 0x8009e088;
            }
            0x8009e088 => {
                c.r[5] = c.r[5] | 0x1;
                pc = 0x8009e08c;
            }
            0x8009e08c => {
                c.r[0] = c.r[4].rotate_left(0) & 0x40000000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e09c; continue; }
                pc = 0x8009e098;
            }
            0x8009e098 => {
                c.r[5] = c.r[5] | 0x2;
                pc = 0x8009e09c;
            }
            0x8009e09c => {
                c.r[0] = c.r[4].rotate_left(0) & 0x20000000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e0ac; continue; }
                pc = 0x8009e0a8;
            }
            0x8009e0a8 => {
                c.r[5] = c.r[5] | 0x4;
                pc = 0x8009e0ac;
            }
            0x8009e0ac => {
                c.r[0] = c.r[4].rotate_left(0) & 0x10000000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e0bc; continue; }
                pc = 0x8009e0b8;
            }
            0x8009e0b8 => {
                c.r[5] = c.r[5] | 0x8;
                pc = 0x8009e0bc;
            }
            0x8009e0bc => {
                c.r[0] = c.r[4].rotate_left(0) & 0x8000000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e0cc; continue; }
                pc = 0x8009e0c8;
            }
            0x8009e0c8 => {
                c.r[5] = c.r[5] | 0x10;
                pc = 0x8009e0cc;
            }
            0x8009e0cc => {
                c.r[4] = 0xcc000000;
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.r[4] = c.r[4].wrapping_add(0x4000);
                c.sth(c.r[4].wrapping_add(0x1c), c.r[0]);
                c.r[3] = c.r[3].rotate_left(0) & 0x7ffffff;
                pc = 0x8009e300; continue;
                pc = 0x8009e0e4;
            }
            0x8009e0e4 => {
                c.r[5] = 0xcc000000;
                c.r[5] = c.r[5].wrapping_add(0x5000);
                c.r[5] = c.r[5].wrapping_add(0xa);
                c.r[0] = c.r[4].rotate_left(0) & 0x4000000;
                c.r[6] = c.lhz(c.r[5]);
                c.cmpu(0, c.r[0], 0x0);
                c.r[6] = c.r[6].rotate_left(0) & 0xfffffe07;
                if !c.crbit(2) { pc = 0x8009e108; continue; }
                pc = 0x8009e104;
            }
            0x8009e104 => {
                c.r[6] = c.r[6] | 0x10;
                pc = 0x8009e108;
            }
            0x8009e108 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x2000000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e118; continue; }
                pc = 0x8009e114;
            }
            0x8009e114 => {
                c.r[6] = c.r[6] | 0x40;
                pc = 0x8009e118;
            }
            0x8009e118 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x1000000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e128; continue; }
                pc = 0x8009e124;
            }
            0x8009e124 => {
                c.r[6] = c.r[6] | 0x100;
                pc = 0x8009e128;
            }
            0x8009e128 => {
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.sth(c.r[5], c.r[0]);
                c.r[3] = c.r[3].rotate_left(0) & 0xf8ffffff;
                pc = 0x8009e300; continue;
                pc = 0x8009e138;
            }
            0x8009e138 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x800000;
                c.r[4] = 0xcc000000;
                c.cmpu(0, c.r[0], 0x0);
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x6c00));
                c.r[0] = 0xffffffd3;
                c.r[5] = c.r[5] & c.r[0];
                if !c.crbit(2) { pc = 0x8009e158; continue; }
                pc = 0x8009e154;
            }
            0x8009e154 => {
                c.r[5] = c.r[5] | 0x4;
                pc = 0x8009e158;
            }
            0x8009e158 => {
                c.r[4] = 0xcc000000;
                c.stw(c.r[4].wrapping_add(0x6c00), c.r[5]);
                c.r[3] = c.r[3].rotate_left(0) & 0xff7fffff;
                pc = 0x8009e300; continue;
                pc = 0x8009e168;
            }
            0x8009e168 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x400000;
                c.r[5] = 0xcc000000;
                c.cmpu(0, c.r[0], 0x0);
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x6800));
                c.r[0] = 0xffffd3f0;
                c.r[5] = c.r[5] & c.r[0];
                if !c.crbit(2) { pc = 0x8009e188; continue; }
                pc = 0x8009e184;
            }
            0x8009e184 => {
                c.r[5] = c.r[5] | 0x1;
                pc = 0x8009e188;
            }
            0x8009e188 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x200000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e198; continue; }
                pc = 0x8009e194;
            }
            0x8009e194 => {
                c.r[5] = c.r[5] | 0x4;
                pc = 0x8009e198;
            }
            0x8009e198 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x100000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e1a8; continue; }
                pc = 0x8009e1a4;
            }
            0x8009e1a4 => {
                c.r[5] = c.r[5] | 0x400;
                pc = 0x8009e1a8;
            }
            0x8009e1a8 => {
                c.r[4] = 0xcc000000;
                c.stw(c.r[4].wrapping_add(0x6800), c.r[5]);
                c.r[3] = c.r[3].rotate_left(0) & 0xff8fffff;
                pc = 0x8009e300; continue;
                pc = 0x8009e1b8;
            }
            0x8009e1b8 => {
                c.r[5] = 0xcc000000;
                c.r[6] = c.r[5].wrapping_add(0x6800);
                c.r[6] = c.r[6].wrapping_add(0x14);
                c.r[0] = c.r[4].rotate_left(0) & 0x80000;
                c.r[7] = c.lwz(c.r[6]);
                c.r[5] = 0xfffff3f0;
                c.cmpu(0, c.r[0], 0x0);
                c.r[7] = c.r[7] & c.r[5];
                if !c.crbit(2) { pc = 0x8009e1e0; continue; }
                pc = 0x8009e1dc;
            }
            0x8009e1dc => {
                c.r[7] = c.r[7] | 0x1;
                pc = 0x8009e1e0;
            }
            0x8009e1e0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x40000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e1f0; continue; }
                pc = 0x8009e1ec;
            }
            0x8009e1ec => {
                c.r[7] = c.r[7] | 0x4;
                pc = 0x8009e1f0;
            }
            0x8009e1f0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x20000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e200; continue; }
                pc = 0x8009e1fc;
            }
            0x8009e1fc => {
                c.r[7] = c.r[7] | 0x400;
                pc = 0x8009e200;
            }
            0x8009e200 => {
                c.stw(c.r[6], c.r[7]);
                c.r[3] = c.r[3].rotate_left(0) & 0xfff1ffff;
                pc = 0x8009e300; continue;
                pc = 0x8009e20c;
            }
            0x8009e20c => {
                c.r[5] = 0xcc000000;
                c.r[5] = c.r[5].wrapping_add(0x6800);
                c.r[5] = c.r[5].wrapping_add(0x28);
                c.r[0] = c.r[4].rotate_left(0) & 0x10000;
                c.r[6] = c.lwz(c.r[5]);
                c.cmpu(0, c.r[0], 0x0);
                c.r[6] = c.r[6].rotate_left(0) & 0xfffffff0;
                if !c.crbit(2) { pc = 0x8009e230; continue; }
                pc = 0x8009e22c;
            }
            0x8009e22c => {
                c.r[6] = c.r[6] | 0x1;
                pc = 0x8009e230;
            }
            0x8009e230 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x8000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e240; continue; }
                pc = 0x8009e23c;
            }
            0x8009e23c => {
                c.r[6] = c.r[6] | 0x4;
                pc = 0x8009e240;
            }
            0x8009e240 => {
                c.stw(c.r[5], c.r[6]);
                c.r[3] = c.r[3].rotate_left(0) & 0xfffe7fff;
                pc = 0x8009e300; continue;
                pc = 0x8009e24c;
            }
            0x8009e24c => {
                c.r[0] = c.r[4].rotate_left(0) & 0x4000;
                c.cmpu(0, c.r[0], 0x0);
                c.r[5] = 0xf0;
                if !c.crbit(2) { pc = 0x8009e260; continue; }
                pc = 0x8009e25c;
            }
            0x8009e25c => {
                c.r[5] = c.r[5] | 0x800;
                pc = 0x8009e260;
            }
            0x8009e260 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x800;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e270; continue; }
                pc = 0x8009e26c;
            }
            0x8009e26c => {
                c.r[5] = c.r[5] | 0x8;
                pc = 0x8009e270;
            }
            0x8009e270 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x400;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e280; continue; }
                pc = 0x8009e27c;
            }
            0x8009e27c => {
                c.r[5] = c.r[5] | 0x4;
                pc = 0x8009e280;
            }
            0x8009e280 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x200;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e290; continue; }
                pc = 0x8009e28c;
            }
            0x8009e28c => {
                c.r[5] = c.r[5] | 0x2;
                pc = 0x8009e290;
            }
            0x8009e290 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x100;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e2a0; continue; }
                pc = 0x8009e29c;
            }
            0x8009e29c => {
                c.r[5] = c.r[5] | 0x1;
                pc = 0x8009e2a0;
            }
            0x8009e2a0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x80;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e2b0; continue; }
                pc = 0x8009e2ac;
            }
            0x8009e2ac => {
                c.r[5] = c.r[5] | 0x100;
                pc = 0x8009e2b0;
            }
            0x8009e2b0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x40;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e2c0; continue; }
                pc = 0x8009e2bc;
            }
            0x8009e2bc => {
                c.r[5] = c.r[5] | 0x1000;
                pc = 0x8009e2c0;
            }
            0x8009e2c0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x2000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e2d0; continue; }
                pc = 0x8009e2cc;
            }
            0x8009e2cc => {
                c.r[5] = c.r[5] | 0x200;
                pc = 0x8009e2d0;
            }
            0x8009e2d0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x1000;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e2e0; continue; }
                pc = 0x8009e2dc;
            }
            0x8009e2dc => {
                c.r[5] = c.r[5] | 0x400;
                pc = 0x8009e2e0;
            }
            0x8009e2e0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0x20;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e2f0; continue; }
                pc = 0x8009e2ec;
            }
            0x8009e2ec => {
                c.r[5] = c.r[5] | 0x2000;
                pc = 0x8009e2f0;
            }
            0x8009e2f0 => {
                c.r[4] = 0xcc000000;
                c.r[4] = c.r[4].wrapping_add(0x3000);
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                c.r[3] = c.r[3].rotate_left(0) & 0xffff801f;
                pc = 0x8009e300;
            }
            0x8009e300 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009e02c: bad pc {:#x}", pc),
        }
    }
}

/// __OSMaskInterrupts (0x8009e304, 136 bytes)
#[inline(never)]
pub fn f_8009e304(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009e304) { return; }
        if c.prof { crate::prof::enter(c, 0x8009e304); f_8009e304_b(c, 0x8009e304); crate::prof::exit(c, 0x8009e304); return; }
    }
    f_8009e304_b(c, 0x8009e304)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009e304_at(c: &mut Cpu, pc: u32) { f_8009e304_b(c, pc) }
#[inline(always)]
fn f_8009e304_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009e304;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009e304, pc); }
        match pc {
            0x8009e304 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8009e324; f_8009df3c(c);
                pc = 0x8009e324;
            }
            0x8009e324 => {
                c.r[4] = 0x80000000;
                c.r[29] = c.lwz(c.r[4].wrapping_add(0xc4));
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = c.lwz(c.r[4].wrapping_add(0xc8));
                c.r[0] = c.r[29] | c.r[5];
                c.r[3] = c.r[31] & !c.r[0];
                c.r[31] = c.r[31] | c.r[29];
                c.stw(c.r[4].wrapping_add(0xc4), c.r[31]);
                c.r[31] = c.r[31] | c.r[5];
                pc = 0x8009e34c; continue;
                pc = 0x8009e34c;
            }
            0x8009e34c => {
                pc = 0x8009e350; continue;
                pc = 0x8009e350;
            }
            0x8009e350 => {
                pc = 0x8009e35c; continue;
                pc = 0x8009e354;
            }
            0x8009e354 => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8009e35c; f_8009e02c(c);
                pc = 0x8009e35c;
            }
            0x8009e35c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009e354; continue; }
                pc = 0x8009e364;
            }
            0x8009e364 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009e36c; f_8009df64(c);
                pc = 0x8009e36c;
            }
            0x8009e36c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009e304: bad pc {:#x}", pc),
        }
    }
}

/// __OSUnmaskInterrupts (0x8009e38c, 136 bytes)
#[inline(never)]
pub fn f_8009e38c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009e38c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009e38c); f_8009e38c_b(c, 0x8009e38c); crate::prof::exit(c, 0x8009e38c); return; }
    }
    f_8009e38c_b(c, 0x8009e38c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009e38c_at(c: &mut Cpu, pc: u32) { f_8009e38c_b(c, pc) }
#[inline(always)]
fn f_8009e38c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009e38c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009e38c, pc); }
        match pc {
            0x8009e38c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8009e3ac; f_8009df3c(c);
                pc = 0x8009e3ac;
            }
            0x8009e3ac => {
                c.r[4] = 0x80000000;
                c.r[29] = c.lwz(c.r[4].wrapping_add(0xc4));
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = c.lwz(c.r[4].wrapping_add(0xc8));
                c.r[0] = c.r[29] | c.r[5];
                c.r[3] = c.r[31] & c.r[0];
                c.r[31] = c.r[29] & !c.r[31];
                c.stw(c.r[4].wrapping_add(0xc4), c.r[31]);
                c.r[31] = c.r[31] | c.r[5];
                pc = 0x8009e3d4; continue;
                pc = 0x8009e3d4;
            }
            0x8009e3d4 => {
                pc = 0x8009e3d8; continue;
                pc = 0x8009e3d8;
            }
            0x8009e3d8 => {
                pc = 0x8009e3e4; continue;
                pc = 0x8009e3dc;
            }
            0x8009e3dc => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8009e3e4; f_8009e02c(c);
                pc = 0x8009e3e4;
            }
            0x8009e3e4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009e3dc; continue; }
                pc = 0x8009e3ec;
            }
            0x8009e3ec => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009e3f4; f_8009df64(c);
                pc = 0x8009e3f4;
            }
            0x8009e3f4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009e38c: bad pc {:#x}", pc),
        }
    }
}

/// __OSDispatchInterrupt (0x8009e414, 836 bytes)
#[inline(never)]
pub fn f_8009e414(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009e414) { return; }
        if c.prof { crate::prof::enter(c, 0x8009e414); f_8009e414_b(c, 0x8009e414); crate::prof::exit(c, 0x8009e414); return; }
    }
    f_8009e414_b(c, 0x8009e414)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009e414_at(c: &mut Cpu, pc: u32) { f_8009e414_b(c, pc) }
#[inline(always)]
fn f_8009e414_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009e414;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009e414, pc); }
        match pc {
            0x8009e414 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[30] = c.r[4] | c.r[4];
                c.r[3] = 0xcc000000;
                c.r[31] = c.lwz(c.r[3].wrapping_add(0x3000));
                c.r[31] = c.r[31].rotate_left(0) & 0xfffeffff;
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8009e458; continue; }
                pc = 0x8009e444;
            }
            0x8009e444 => {
                c.r[3] = c.r[3].wrapping_add(0x3000);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[31] & c.r[0];
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009e460; continue; }
                pc = 0x8009e458;
            }
            0x8009e458 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009e460; f_8009bc50(c);
                pc = 0x8009e460;
            }
            0x8009e460 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x80;
                c.cmpu(0, c.r[0], 0x0);
                c.r[0] = 0x0;
                if c.crbit(2) { pc = 0x8009e4cc; continue; }
                pc = 0x8009e470;
            }
            0x8009e470 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x4000);
                c.r[4] = c.lhz(c.r[3].wrapping_add(0x1e));
                c.r[3] = c.r[4].rotate_left(0) & 0x1;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e48c; continue; }
                pc = 0x8009e488;
            }
            0x8009e488 => {
                c.r[0] = c.r[0] | 0x80000000;
                pc = 0x8009e48c;
            }
            0x8009e48c => {
                c.r[3] = c.r[4].rotate_left(0) & 0x2;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e49c; continue; }
                pc = 0x8009e498;
            }
            0x8009e498 => {
                c.r[0] = c.r[0] | 0x40000000;
                pc = 0x8009e49c;
            }
            0x8009e49c => {
                c.r[3] = c.r[4].rotate_left(0) & 0x4;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e4ac; continue; }
                pc = 0x8009e4a8;
            }
            0x8009e4a8 => {
                c.r[0] = c.r[0] | 0x20000000;
                pc = 0x8009e4ac;
            }
            0x8009e4ac => {
                c.r[3] = c.r[4].rotate_left(0) & 0x8;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e4bc; continue; }
                pc = 0x8009e4b8;
            }
            0x8009e4b8 => {
                c.r[0] = c.r[0] | 0x10000000;
                pc = 0x8009e4bc;
            }
            0x8009e4bc => {
                c.r[3] = c.r[4].rotate_left(0) & 0x10;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e4cc; continue; }
                pc = 0x8009e4c8;
            }
            0x8009e4c8 => {
                c.r[0] = c.r[0] | 0x8000000;
                pc = 0x8009e4cc;
            }
            0x8009e4cc => {
                c.r[3] = c.r[31].rotate_left(0) & 0x40;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e514; continue; }
                pc = 0x8009e4d8;
            }
            0x8009e4d8 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x5000);
                c.r[4] = c.lhz(c.r[3].wrapping_add(0xa));
                c.r[3] = c.r[4].rotate_left(0) & 0x8;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e4f4; continue; }
                pc = 0x8009e4f0;
            }
            0x8009e4f0 => {
                c.r[0] = c.r[0] | 0x4000000;
                pc = 0x8009e4f4;
            }
            0x8009e4f4 => {
                c.r[3] = c.r[4].rotate_left(0) & 0x20;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e504; continue; }
                pc = 0x8009e500;
            }
            0x8009e500 => {
                c.r[0] = c.r[0] | 0x2000000;
                pc = 0x8009e504;
            }
            0x8009e504 => {
                c.r[3] = c.r[4].rotate_left(0) & 0x80;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e514; continue; }
                pc = 0x8009e510;
            }
            0x8009e510 => {
                c.r[0] = c.r[0] | 0x1000000;
                pc = 0x8009e514;
            }
            0x8009e514 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x20;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e538; continue; }
                pc = 0x8009e520;
            }
            0x8009e520 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x6c00));
                c.r[3] = c.r[3].rotate_left(0) & 0x8;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e538; continue; }
                pc = 0x8009e534;
            }
            0x8009e534 => {
                c.r[0] = c.r[0] | 0x800000;
                pc = 0x8009e538;
            }
            0x8009e538 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x10;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e5e4; continue; }
                pc = 0x8009e544;
            }
            0x8009e544 => {
                c.r[3] = 0xcc000000;
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x6800));
                c.r[3] = c.r[4].rotate_left(0) & 0x2;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e55c; continue; }
                pc = 0x8009e558;
            }
            0x8009e558 => {
                c.r[0] = c.r[0] | 0x400000;
                pc = 0x8009e55c;
            }
            0x8009e55c => {
                c.r[3] = c.r[4].rotate_left(0) & 0x8;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e56c; continue; }
                pc = 0x8009e568;
            }
            0x8009e568 => {
                c.r[0] = c.r[0] | 0x200000;
                pc = 0x8009e56c;
            }
            0x8009e56c => {
                c.r[3] = c.r[4].rotate_left(0) & 0x800;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e57c; continue; }
                pc = 0x8009e578;
            }
            0x8009e578 => {
                c.r[0] = c.r[0] | 0x100000;
                pc = 0x8009e57c;
            }
            0x8009e57c => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x14));
                c.r[3] = c.r[4].rotate_left(0) & 0x2;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e598; continue; }
                pc = 0x8009e594;
            }
            0x8009e594 => {
                c.r[0] = c.r[0] | 0x80000;
                pc = 0x8009e598;
            }
            0x8009e598 => {
                c.r[3] = c.r[4].rotate_left(0) & 0x8;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e5a8; continue; }
                pc = 0x8009e5a4;
            }
            0x8009e5a4 => {
                c.r[0] = c.r[0] | 0x40000;
                pc = 0x8009e5a8;
            }
            0x8009e5a8 => {
                c.r[3] = c.r[4].rotate_left(0) & 0x800;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e5b8; continue; }
                pc = 0x8009e5b4;
            }
            0x8009e5b4 => {
                c.r[0] = c.r[0] | 0x20000;
                pc = 0x8009e5b8;
            }
            0x8009e5b8 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x28));
                c.r[3] = c.r[4].rotate_left(0) & 0x2;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e5d4; continue; }
                pc = 0x8009e5d0;
            }
            0x8009e5d0 => {
                c.r[0] = c.r[0] | 0x10000;
                pc = 0x8009e5d4;
            }
            0x8009e5d4 => {
                c.r[3] = c.r[4].rotate_left(0) & 0x8;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e5e4; continue; }
                pc = 0x8009e5e0;
            }
            0x8009e5e0 => {
                c.r[0] = c.r[0] | 0x8000;
                pc = 0x8009e5e4;
            }
            0x8009e5e4 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x2000;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e5f4; continue; }
                pc = 0x8009e5f0;
            }
            0x8009e5f0 => {
                c.r[0] = c.r[0] | 0x20;
                pc = 0x8009e5f4;
            }
            0x8009e5f4 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x1000;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e604; continue; }
                pc = 0x8009e600;
            }
            0x8009e600 => {
                c.r[0] = c.r[0] | 0x40;
                pc = 0x8009e604;
            }
            0x8009e604 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x400;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e614; continue; }
                pc = 0x8009e610;
            }
            0x8009e610 => {
                c.r[0] = c.r[0] | 0x1000;
                pc = 0x8009e614;
            }
            0x8009e614 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x200;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e624; continue; }
                pc = 0x8009e620;
            }
            0x8009e620 => {
                c.r[0] = c.r[0] | 0x2000;
                pc = 0x8009e624;
            }
            0x8009e624 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x100;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e634; continue; }
                pc = 0x8009e630;
            }
            0x8009e630 => {
                c.r[0] = c.r[0] | 0x80;
                pc = 0x8009e634;
            }
            0x8009e634 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x8;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e644; continue; }
                pc = 0x8009e640;
            }
            0x8009e640 => {
                c.r[0] = c.r[0] | 0x800;
                pc = 0x8009e644;
            }
            0x8009e644 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x4;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e654; continue; }
                pc = 0x8009e650;
            }
            0x8009e650 => {
                c.r[0] = c.r[0] | 0x400;
                pc = 0x8009e654;
            }
            0x8009e654 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x2;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e664; continue; }
                pc = 0x8009e660;
            }
            0x8009e660 => {
                c.r[0] = c.r[0] | 0x200;
                pc = 0x8009e664;
            }
            0x8009e664 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x800;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e674; continue; }
                pc = 0x8009e670;
            }
            0x8009e670 => {
                c.r[0] = c.r[0] | 0x4000;
                pc = 0x8009e674;
            }
            0x8009e674 => {
                c.r[3] = c.r[31].rotate_left(0) & 0x1;
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009e684; continue; }
                pc = 0x8009e680;
            }
            0x8009e680 => {
                c.r[0] = c.r[0] | 0x100;
                pc = 0x8009e684;
            }
            0x8009e684 => {
                c.r[3] = 0x80000000;
                c.r[4] = c.lwz(c.r[3].wrapping_add(0xc4));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc8));
                c.r[3] = c.r[4] | c.r[3];
                c.r[4] = c.r[0] & !c.r[3];
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x8009e734; continue; }
                pc = 0x8009e6a0;
            }
            0x8009e6a0 => {
                c.r[3] = 0x80310000;
                c.r[0] = c.r[3].wrapping_add(0x17e8);
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x8009e6b0; continue;
                pc = 0x8009e6b0;
            }
            0x8009e6b0 => {
                pc = 0x8009e6b4; continue;
                pc = 0x8009e6b4;
            }
            0x8009e6b4 => {
                c.r[0] = c.lwz(c.r[3]);
                c.r[0] = c.r[4] & c.r[0];
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009e6d0; continue; }
                pc = 0x8009e6c4;
            }
            0x8009e6c4 => {
                c.r[0] = c.r[0].leading_zeros();
                c.r[29] = c.r[0] as u16 as i16 as i32 as u32;
                pc = 0x8009e6d8; continue;
                pc = 0x8009e6d0;
            }
            0x8009e6d0 => {
                c.r[3] = c.r[3].wrapping_add(0x4);
                pc = 0x8009e6b4; continue;
                pc = 0x8009e6d8;
            }
            0x8009e6d8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff9ef0));
                c.r[0] = c.r[29].rotate_left(2) & 0xfffffffc;
                c.r[31] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8009e734; continue; }
                pc = 0x8009e6ec;
            }
            0x8009e6ec => {
                c.cmps(0, c.r[29], 0x4);
                if !c.crbit(1) { pc = 0x8009e70c; continue; }
                pc = 0x8009e6f4;
            }
            0x8009e6f4 => {
                c.sth(c.r[13].wrapping_add(0xffff9ef8), c.r[29]);
                c.lr = 0x8009e6fc; crate::stubs::f_800a2778(c);
                pc = 0x8009e6fc;
            }
            0x8009e6fc => {
                c.stw(c.r[13].wrapping_add(0xffff9f04), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffff9f00), c.r[3]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x198));
                c.stw(c.r[13].wrapping_add(0xffff9ef4), c.r[0]);
                pc = 0x8009e70c;
            }
            0x8009e70c => {
                c.lr = 0x8009e710; f_800a1404(c);
                pc = 0x8009e710;
            }
            0x8009e710 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[12] = c.r[31] | c.r[31];
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x8009e724; (c.dispatch)(c, t); }
                pc = 0x8009e724;
            }
            0x8009e724 => {
                c.lr = 0x8009e728; f_800a1444(c);
                pc = 0x8009e728;
            }
            0x8009e728 => {
                c.lr = 0x8009e72c; f_800a1960(c);
                pc = 0x8009e72c;
            }
            0x8009e72c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009e734; f_8009bc50(c);
                pc = 0x8009e734;
            }
            0x8009e734 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009e73c; f_8009bc50(c);
                pc = 0x8009e73c;
            }
            0x8009e73c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009e414: bad pc {:#x}", pc),
        }
    }
}

/// ExternalInterruptHandler (0x8009e758, 80 bytes)
#[inline(never)]
pub fn f_8009e758(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009e758) { return; }
        if c.prof { crate::prof::enter(c, 0x8009e758); f_8009e758_b(c, 0x8009e758); crate::prof::exit(c, 0x8009e758); return; }
    }
    f_8009e758_b(c, 0x8009e758)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009e758_at(c: &mut Cpu, pc: u32) { f_8009e758_b(c, pc) }
#[inline(always)]
fn f_8009e758_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009e758;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009e758, pc); }
        match pc {
            0x8009e758 => {
                c.stw(c.r[4], c.r[0]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[1]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[2]);
                { let mut ea = c.r[4].wrapping_add(0x18); for n in 6..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[0] = c.mfspr(913);
                c.stw(c.r[4].wrapping_add(0x1a8), c.r[0]);
                c.r[0] = c.mfspr(914);
                c.stw(c.r[4].wrapping_add(0x1ac), c.r[0]);
                c.r[0] = c.mfspr(915);
                c.stw(c.r[4].wrapping_add(0x1b0), c.r[0]);
                c.r[0] = c.mfspr(916);
                c.stw(c.r[4].wrapping_add(0x1b4), c.r[0]);
                c.r[0] = c.mfspr(917);
                c.stw(c.r[4].wrapping_add(0x1b8), c.r[0]);
                c.r[0] = c.mfspr(918);
                c.stw(c.r[4].wrapping_add(0x1bc), c.r[0]);
                c.r[0] = c.mfspr(919);
                c.stw(c.r[4].wrapping_add(0x1c0), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                f_8009e414(c); return;
                return;
            }
            _ => panic!("f_8009e758: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009E7A8 (0x8009e7a8, 4 bytes)
#[inline(never)]
pub fn f_8009e7a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009e7a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009e7a8); f_8009e7a8_b(c, 0x8009e7a8); crate::prof::exit(c, 0x8009e7a8); return; }
    }
    f_8009e7a8_b(c, 0x8009e7a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009e7a8_at(c: &mut Cpu, pc: u32) { f_8009e7a8_b(c, pc) }
#[inline(always)]
fn f_8009e7a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009e7a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009e7a8, pc); }
        match pc {
            0x8009e7a8 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009e7a8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009E7AC (0x8009e7ac, 4 bytes)
#[inline(never)]
pub fn f_8009e7ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009e7ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8009e7ac); f_8009e7ac_b(c, 0x8009e7ac); crate::prof::exit(c, 0x8009e7ac); return; }
    }
    f_8009e7ac_b(c, 0x8009e7ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009e7ac_at(c: &mut Cpu, pc: u32) { f_8009e7ac_b(c, pc) }
#[inline(always)]
fn f_8009e7ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009e7ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009e7ac, pc); }
        match pc {
            0x8009e7ac => {
                if true { return; }
                return;
            }
            _ => panic!("f_8009e7ac: bad pc {:#x}", pc),
        }
    }
}

/// OSLink (0x8009e7b0, 700 bytes)
#[inline(never)]
pub fn f_8009e7b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009e7b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009e7b0); f_8009e7b0_b(c, 0x8009e7b0); crate::prof::exit(c, 0x8009e7b0); return; }
    }
    f_8009e7b0_b(c, 0x8009e7b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009e7b0_at(c: &mut Cpu, pc: u32) { f_8009e7b0_b(c, pc) }
#[inline(always)]
fn f_8009e7b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009e7b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009e7b0, pc); }
        match pc {
            0x8009e7b0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                c.r[27] = c.r[4].wrapping_add(0x0);
                if c.crbit(2) { pc = 0x8009e7d4; continue; }
                pc = 0x8009e7cc;
            }
            0x8009e7cc => {
                c.r[31] = c.lwz(c.r[26]);
                pc = 0x8009e7d8; continue;
                pc = 0x8009e7d4;
            }
            0x8009e7d4 => {
                c.r[31] = 0x0;
                pc = 0x8009e7d8;
            }
            0x8009e7d8 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x28));
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x2c));
                c.r[4] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x7);
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.cmpu(0, c.r[4], c.r[3]);
                c.mtspr(9, c.r[0]);
                if !c.crbit(0) { pc = 0x8009e814; continue; }
                pc = 0x8009e800;
            }
            0x8009e800 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(2) { pc = 0x8009e81c; continue; }
                pc = 0x8009e80c;
            }
            0x8009e80c => {
                c.r[4] = c.r[4].wrapping_add(0x8);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009e800; continue; }
                pc = 0x8009e814;
            }
            0x8009e814 => {
                c.r[3] = 0x0;
                pc = 0x8009ea58; continue;
                pc = 0x8009e81c;
            }
            0x8009e81c => {
                c.r[3] = 0x80310000;
                c.r[30] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[25] = c.r[3].wrapping_add(0x1818);
                c.r[29] = 0x0;
                pc = 0x8009ea20; continue;
                pc = 0x8009e830;
            }
            0x8009e830 => {
                c.r[0] = c.lhz(c.r[30]);
                c.cmpu(0, c.r[31], 0x0);
                c.r[28] = c.r[28].wrapping_add(c.r[0]);
                if c.crbit(2) { pc = 0x8009e858; continue; }
                pc = 0x8009e840;
            }
            0x8009e840 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x3));
                c.r[3] = c.lwz(c.r[26].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[5] = c.r[0].rotate_left(0) & 0xfffffffe;
                pc = 0x8009e85c; continue;
                pc = 0x8009e858;
            }
            0x8009e858 => {
                c.r[5] = 0x0;
                pc = 0x8009e85c;
            }
            0x8009e85c => {
                c.cmps(0, c.r[4], 0x6);
                if c.crbit(2) { pc = 0x8009e924; continue; }
                pc = 0x8009e864;
            }
            0x8009e864 => {
                if !c.crbit(0) { pc = 0x8009e894; continue; }
                pc = 0x8009e868;
            }
            0x8009e868 => {
                c.cmps(0, c.r[4], 0x2);
                if c.crbit(2) { pc = 0x8009e8d4; continue; }
                pc = 0x8009e870;
            }
            0x8009e870 => {
                if !c.crbit(0) { pc = 0x8009e884; continue; }
                pc = 0x8009e874;
            }
            0x8009e874 => {
                c.cmps(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x8009ea1c; continue; }
                pc = 0x8009e87c;
            }
            0x8009e87c => {
                if !c.crbit(0) { pc = 0x8009e8c4; continue; }
                pc = 0x8009e880;
            }
            0x8009e880 => {
                pc = 0x8009ea10; continue;
                pc = 0x8009e884;
            }
            0x8009e884 => {
                c.cmps(0, c.r[4], 0x4);
                if c.crbit(2) { pc = 0x8009e900; continue; }
                pc = 0x8009e88c;
            }
            0x8009e88c => {
                if !c.crbit(0) { pc = 0x8009e910; continue; }
                pc = 0x8009e890;
            }
            0x8009e890 => {
                pc = 0x8009e8f0; continue;
                pc = 0x8009e894;
            }
            0x8009e894 => {
                c.cmps(0, c.r[4], 0xc9);
                if c.crbit(2) { pc = 0x8009ea1c; continue; }
                pc = 0x8009e89c;
            }
            0x8009e89c => {
                if !c.crbit(0) { pc = 0x8009e8b8; continue; }
                pc = 0x8009e8a0;
            }
            0x8009e8a0 => {
                c.cmps(0, c.r[4], 0xa);
                if c.crbit(2) { pc = 0x8009e96c; continue; }
                pc = 0x8009e8a8;
            }
            0x8009e8a8 => {
                if c.crbit(0) { pc = 0x8009e950; continue; }
                pc = 0x8009e8ac;
            }
            0x8009e8ac => {
                c.cmps(0, c.r[4], 0xe);
                if !c.crbit(0) { pc = 0x8009ea10; continue; }
                pc = 0x8009e8b4;
            }
            0x8009e8b4 => {
                pc = 0x8009e98c; continue;
                pc = 0x8009e8b8;
            }
            0x8009e8b8 => {
                c.cmps(0, c.r[4], 0xcb);
                if !c.crbit(0) { pc = 0x8009ea10; continue; }
                pc = 0x8009e8c0;
            }
            0x8009e8c0 => {
                pc = 0x8009e9ac; continue;
                pc = 0x8009e8c4;
            }
            0x8009e8c4 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.stw(c.r[28], c.r[0]);
                pc = 0x8009ea1c; continue;
                pc = 0x8009e8d4;
            }
            0x8009e8d4 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[3] = c.lwz(c.r[28]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.r[3] = c.r[3].rotate_left(0) & 0xfc000003;
                c.r[3] = (c.r[0].rotate_left(0) & 0x3fffffc) | (c.r[3] & 0xfc000003);
                c.stw(c.r[28], c.r[3]);
                pc = 0x8009ea1c; continue;
                pc = 0x8009e8f0;
            }
            0x8009e8f0 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[28], c.r[0]);
                pc = 0x8009ea1c; continue;
                pc = 0x8009e900;
            }
            0x8009e900 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[28], c.r[0]);
                pc = 0x8009ea1c; continue;
                pc = 0x8009e910;
            }
            0x8009e910 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.r[0].rotate_left(16) & 0xffff;
                c.sth(c.r[28], c.r[0]);
                pc = 0x8009ea1c; continue;
                pc = 0x8009e924;
            }
            0x8009e924 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[4] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0x8000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009e93c; continue; }
                pc = 0x8009e934;
            }
            0x8009e934 => {
                c.r[3] = 0x1;
                pc = 0x8009e940; continue;
                pc = 0x8009e93c;
            }
            0x8009e93c => {
                c.r[3] = 0x0;
                pc = 0x8009e940;
            }
            0x8009e940 => {
                c.r[0] = c.r[4].rotate_left(16) & 0xffff;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.sth(c.r[28], c.r[0]);
                pc = 0x8009ea1c; continue;
                pc = 0x8009e950;
            }
            0x8009e950 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[3] = c.lwz(c.r[28]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.r[3] = c.r[3].rotate_left(0) & 0xffff0003;
                c.r[3] = (c.r[0].rotate_left(0) & 0xfffc) | (c.r[3] & 0xffff0003);
                c.stw(c.r[28], c.r[3]);
                pc = 0x8009ea1c; continue;
                pc = 0x8009e96c;
            }
            0x8009e96c => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[3] = c.lwz(c.r[28]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.r[0].wrapping_sub(c.r[28]);
                c.r[3] = c.r[3].rotate_left(0) & 0xfc000003;
                c.r[3] = (c.r[0].rotate_left(0) & 0x3fffffc) | (c.r[3] & 0xfc000003);
                c.stw(c.r[28], c.r[3]);
                pc = 0x8009ea1c; continue;
                pc = 0x8009e98c;
            }
            0x8009e98c => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[3] = c.lwz(c.r[28]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.r[0].wrapping_sub(c.r[28]);
                c.r[3] = c.r[3].rotate_left(0) & 0xffff0003;
                c.r[3] = (c.r[0].rotate_left(0) & 0xfffc) | (c.r[3] & 0xffff0003);
                c.stw(c.r[28], c.r[3]);
                pc = 0x8009ea1c; continue;
                pc = 0x8009e9ac;
            }
            0x8009e9ac => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x3));
                c.cmpu(0, c.r[29], 0x0);
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3]);
                c.r[23] = c.r[3].wrapping_add(0x0);
                c.r[28] = c.r[0].rotate_left(0) & 0xfffffffe;
                if c.crbit(2) { pc = 0x8009e9f0; continue; }
                pc = 0x8009e9d0;
            }
            0x8009e9d0 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x4));
                c.r[24] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.r[3] = c.r[24].wrapping_add(0x0);
                c.lr = 0x8009e9e4; f_8009b2d0(c);
                pc = 0x8009e9e4;
            }
            0x8009e9e4 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x4));
                c.lr = 0x8009e9f0; f_8009b3b4(c);
                pc = 0x8009e9f0;
            }
            0x8009e9f0 => {
                c.r[0] = c.lwz(c.r[23]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009ea04; continue; }
                pc = 0x8009e9fc;
            }
            0x8009e9fc => {
                c.r[0] = c.r[23] | c.r[23];
                pc = 0x8009ea08; continue;
                pc = 0x8009ea04;
            }
            0x8009ea04 => {
                c.r[0] = 0x0;
                pc = 0x8009ea08;
            }
            0x8009ea08 => {
                c.r[29] = c.r[0] | c.r[0];
                pc = 0x8009ea1c; continue;
                pc = 0x8009ea10;
            }
            0x8009ea10 => {
                c.r[3] = c.r[25].wrapping_add(0x0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009ea1c; f_8009c2e0(c);
                pc = 0x8009ea1c;
            }
            0x8009ea1c => {
                c.r[30] = c.r[30].wrapping_add(0x8);
                pc = 0x8009ea20;
            }
            0x8009ea20 => {
                c.r[4] = c.lbz(c.r[30].wrapping_add(0x2));
                c.cmpu(0, c.r[4], 0xcb);
                if !c.crbit(2) { pc = 0x8009e830; continue; }
                pc = 0x8009ea2c;
            }
            0x8009ea2c => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x8009ea54; continue; }
                pc = 0x8009ea34;
            }
            0x8009ea34 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x4));
                c.r[25] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.r[3] = c.r[25].wrapping_add(0x0);
                c.lr = 0x8009ea48; f_8009b2d0(c);
                pc = 0x8009ea48;
            }
            0x8009ea48 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x4));
                c.lr = 0x8009ea54; f_8009b3b4(c);
                pc = 0x8009ea54;
            }
            0x8009ea54 => {
                c.r[3] = 0x1;
                pc = 0x8009ea58;
            }
            0x8009ea58 => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009e7b0: bad pc {:#x}", pc),
        }
    }
}

/// Link (0x8009ea6c, 736 bytes)
#[inline(never)]
pub fn f_8009ea6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ea6c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ea6c); f_8009ea6c_b(c, 0x8009ea6c); crate::prof::exit(c, 0x8009ea6c); return; }
    }
    f_8009ea6c_b(c, 0x8009ea6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ea6c_at(c: &mut Cpu, pc: u32) { f_8009ea6c_b(c, pc) }
#[inline(always)]
fn f_8009ea6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ea6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ea6c, pc); }
        match pc {
            0x8009ea6c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x18), c.r[28]);
                c.stb(c.r[3].wrapping_add(0x33), c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(1) { pc = 0x8009eae8; continue; }
                pc = 0x8009eaa8;
            }
            0x8009eaa8 => {
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8009eaf0; continue; }
                pc = 0x8009eab0;
            }
            0x8009eab0 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x40));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009eacc; continue; }
                pc = 0x8009eabc;
            }
            0x8009eabc => {
                c.r[0] = divwu(c.r[31], c.r[3]);
                c.r[0] = c.r[0].wrapping_mul(c.r[3]);
                c.r[0] = c.r[31].wrapping_sub(c.r[0]);
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8009eae8; continue; }
                pc = 0x8009eacc;
            }
            0x8009eacc => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x44));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009eaf0; continue; }
                pc = 0x8009ead8;
            }
            0x8009ead8 => {
                c.r[0] = divwu(c.r[29], c.r[3]);
                c.r[0] = c.r[0].wrapping_mul(c.r[3]);
                c.r[0] = c.r[29].wrapping_sub(c.r[0]);
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009eaf0; continue; }
                pc = 0x8009eae8;
            }
            0x8009eae8 => {
                c.r[3] = 0x0;
                pc = 0x8009ed2c; continue;
                pc = 0x8009eaf0;
            }
            0x8009eaf0 => {
                c.r[3] = 0x80000000;
                c.r[4] = c.r[3].wrapping_add(0x30c8);
                { let ea = c.r[4].wrapping_add(0x4); c.r[5] = c.lwz(ea); c.r[4] = ea; }
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8009eb0c; continue; }
                pc = 0x8009eb04;
            }
            0x8009eb04 => {
                c.stw(c.r[3].wrapping_add(0x30c8), c.r[31]);
                pc = 0x8009eb10; continue;
                pc = 0x8009eb0c;
            }
            0x8009eb0c => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[31]);
                pc = 0x8009eb10;
            }
            0x8009eb10 => {
                c.stw(c.r[31].wrapping_add(0x8), c.r[5]);
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x4), c.r[0]);
                c.stw(c.r[4], c.r[31]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[31].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[31].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x28));
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[31].wrapping_add(0x28), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x8009eb5c; continue; }
                pc = 0x8009eb50;
            }
            0x8009eb50 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[31].wrapping_add(0x48), c.r[0]);
                pc = 0x8009eb5c;
            }
            0x8009eb5c => {
                c.r[5] = 0x1;
                c.r[3] = 0x8;
                pc = 0x8009eba4; continue;
                pc = 0x8009eb68;
            }
            0x8009eb68 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[4] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009eb88; continue; }
                pc = 0x8009eb7c;
            }
            0x8009eb7c => {
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[4], c.r[0]);
                pc = 0x8009eb9c; continue;
                pc = 0x8009eb88;
            }
            0x8009eb88 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009eb9c; continue; }
                pc = 0x8009eb94;
            }
            0x8009eb94 => {
                c.stb(c.r[31].wrapping_add(0x33), c.r[5]);
                c.stw(c.r[4], c.r[29]);
                pc = 0x8009eb9c;
            }
            0x8009eb9c => {
                c.r[3] = c.r[3].wrapping_add(0x8);
                c.r[5] = c.r[5].wrapping_add(0x1);
                pc = 0x8009eba4;
            }
            0x8009eba4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.cmpu(0, c.r[5], c.r[0]);
                if c.crbit(0) { pc = 0x8009eb68; continue; }
                pc = 0x8009ebb0;
            }
            0x8009ebb0 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x28));
                pc = 0x8009ebc8; continue;
                pc = 0x8009ebb8;
            }
            0x8009ebb8 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x8);
                pc = 0x8009ebc8;
            }
            0x8009ebc8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x28));
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x2c));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x8009ebb8; continue; }
                pc = 0x8009ebdc;
            }
            0x8009ebdc => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x30));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009ec04; continue; }
                pc = 0x8009ebe8;
            }
            0x8009ebe8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x34));
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[31].wrapping_add(0x34), c.r[0]);
                pc = 0x8009ec04;
            }
            0x8009ec04 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x31));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009ec2c; continue; }
                pc = 0x8009ec10;
            }
            0x8009ec10 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x38));
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[31].wrapping_add(0x38), c.r[0]);
                pc = 0x8009ec2c;
            }
            0x8009ec2c => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x32));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009ec54; continue; }
                pc = 0x8009ec38;
            }
            0x8009ec38 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x3c));
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[31].wrapping_add(0x3c), c.r[0]);
                pc = 0x8009ec54;
            }
            0x8009ec54 => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x30d0));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009ec70; continue; }
                pc = 0x8009ec64;
            }
            0x8009ec64 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x14));
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.stw(c.r[31].wrapping_add(0x14), c.r[0]);
                pc = 0x8009ec70;
            }
            0x8009ec70 => {
                c.r[3] = 0x0;
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.lr = 0x8009ec7c; f_8009e7b0(c);
                pc = 0x8009ec7c;
            }
            0x8009ec7c => {
                c.r[3] = 0x80000000;
                c.r[28] = c.lwz(c.r[3].wrapping_add(0x30c8));
                pc = 0x8009ecac; continue;
                pc = 0x8009ec88;
            }
            0x8009ec88 => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[28].wrapping_add(0x0);
                c.lr = 0x8009ec94; f_8009e7b0(c);
                pc = 0x8009ec94;
            }
            0x8009ec94 => {
                c.cmpu(0, c.r[28], c.r[31]);
                if c.crbit(2) { pc = 0x8009eca8; continue; }
                pc = 0x8009ec9c;
            }
            0x8009ec9c => {
                c.r[3] = c.r[28].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.lr = 0x8009eca8; f_8009e7b0(c);
                pc = 0x8009eca8;
            }
            0x8009eca8 => {
                c.r[28] = c.lwz(c.r[28].wrapping_add(0x4));
                pc = 0x8009ecac;
            }
            0x8009ecac => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8009ec88; continue; }
                pc = 0x8009ecb4;
            }
            0x8009ecb4 => {
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8009ed10; continue; }
                pc = 0x8009ecbc;
            }
            0x8009ecbc => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x28));
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x2c));
                c.r[5] = c.r[4].wrapping_add(0x0);
                c.r[3] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x7);
                c.r[0] = c.r[0].wrapping_sub(c.r[5]);
                c.r[0] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.cmpu(0, c.r[5], c.r[3]);
                c.mtspr(9, c.r[0]);
                if !c.crbit(0) { pc = 0x8009ed10; continue; }
                pc = 0x8009ece4;
            }
            0x8009ece4 => {
                c.r[3] = c.lwz(c.r[5]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009ecfc; continue; }
                pc = 0x8009ecf0;
            }
            0x8009ecf0 => {
                c.r[0] = c.lwz(c.r[31]);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x8009ed08; continue; }
                pc = 0x8009ecfc;
            }
            0x8009ecfc => {
                c.r[0] = c.r[5].wrapping_sub(c.r[4]);
                c.stw(c.r[31].wrapping_add(0x2c), c.r[0]);
                pc = 0x8009ed10; continue;
                pc = 0x8009ed08;
            }
            0x8009ed08 => {
                c.r[5] = c.r[5].wrapping_add(0x8);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009ece4; continue; }
                pc = 0x8009ed10;
            }
            0x8009ed10 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x20));
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.lr = 0x8009ed20; f_8000540c(c);
                pc = 0x8009ed20;
            }
            0x8009ed20 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8009ed28; f_8009e7a8(c);
                pc = 0x8009ed28;
            }
            0x8009ed28 => {
                c.r[3] = 0x1;
                pc = 0x8009ed2c;
            }
            0x8009ed2c => {
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
            _ => panic!("f_8009ea6c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009ED4C (0x8009ed4c, 36 bytes)
#[inline(never)]
pub fn f_8009ed4c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ed4c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ed4c); f_8009ed4c_b(c, 0x8009ed4c); crate::prof::exit(c, 0x8009ed4c); return; }
    }
    f_8009ed4c_b(c, 0x8009ed4c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ed4c_at(c: &mut Cpu, pc: u32) { f_8009ed4c_b(c, pc) }
#[inline(always)]
fn f_8009ed4c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ed4c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ed4c, pc); }
        match pc {
            0x8009ed4c => {
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x8009ed60; f_8009ea6c(c);
                pc = 0x8009ed60;
            }
            0x8009ed60 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009ed4c: bad pc {:#x}", pc),
        }
    }
}

/// OSLinkFixed (0x8009ed70, 60 bytes)
#[inline(never)]
pub fn f_8009ed70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ed70) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ed70); f_8009ed70_b(c, 0x8009ed70); crate::prof::exit(c, 0x8009ed70); return; }
    }
    f_8009ed70_b(c, 0x8009ed70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ed70_at(c: &mut Cpu, pc: u32) { f_8009ed70_b(c, pc) }
#[inline(always)]
fn f_8009ed70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ed70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ed70, pc); }
        match pc {
            0x8009ed70 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(1) { pc = 0x8009ed8c; continue; }
                pc = 0x8009ed88;
            }
            0x8009ed88 => {
                if !c.crbit(0) { pc = 0x8009ed94; continue; }
                pc = 0x8009ed8c;
            }
            0x8009ed8c => {
                c.r[3] = 0x0;
                pc = 0x8009ed9c; continue;
                pc = 0x8009ed94;
            }
            0x8009ed94 => {
                c.r[5] = 0x1;
                c.lr = 0x8009ed9c; f_8009ea6c(c);
                pc = 0x8009ed9c;
            }
            0x8009ed9c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009ed70: bad pc {:#x}", pc),
        }
    }
}

/// OSUnlink (0x8009edac, 568 bytes)
#[inline(never)]
pub fn f_8009edac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009edac) { return; }
        if c.prof { crate::prof::enter(c, 0x8009edac); f_8009edac_b(c, 0x8009edac); crate::prof::exit(c, 0x8009edac); return; }
    }
    f_8009edac_b(c, 0x8009edac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009edac_at(c: &mut Cpu, pc: u32) { f_8009edac_b(c, pc) }
#[inline(always)]
fn f_8009edac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009edac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009edac, pc); }
        match pc {
            0x8009edac => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[4] | c.r[4];
                c.r[4] = c.lwz(c.r[3]);
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x28));
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x2c));
                c.r[5] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x7);
                c.r[0] = c.r[0].wrapping_sub(c.r[5]);
                c.r[0] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.cmpu(0, c.r[5], c.r[3]);
                c.mtspr(9, c.r[0]);
                if !c.crbit(0) { pc = 0x8009ee00; continue; }
                pc = 0x8009edec;
            }
            0x8009edec => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmpu(0, c.r[0], c.r[4]);
                if c.crbit(2) { pc = 0x8009ee08; continue; }
                pc = 0x8009edf8;
            }
            0x8009edf8 => {
                c.r[5] = c.r[5].wrapping_add(0x8);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8009edec; continue; }
                pc = 0x8009ee00;
            }
            0x8009ee00 => {
                c.r[3] = 0x0;
                pc = 0x8009efd0; continue;
                pc = 0x8009ee08;
            }
            0x8009ee08 => {
                c.r[3] = 0x80310000;
                c.r[31] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[27] = c.r[3].wrapping_add(0x1840);
                c.r[30] = 0x0;
                pc = 0x8009ef98; continue;
                pc = 0x8009ee1c;
            }
            0x8009ee1c => {
                c.r[0] = c.lhz(c.r[31]);
                c.cmps(0, c.r[4], 0x6);
                c.r[3] = 0x0;
                c.r[29] = c.r[29].wrapping_add(c.r[0]);
                if c.crbit(2) { pc = 0x8009eed0; continue; }
                pc = 0x8009ee30;
            }
            0x8009ee30 => {
                if !c.crbit(0) { pc = 0x8009ee60; continue; }
                pc = 0x8009ee34;
            }
            0x8009ee34 => {
                c.cmps(0, c.r[4], 0x2);
                if c.crbit(2) { pc = 0x8009ee9c; continue; }
                pc = 0x8009ee3c;
            }
            0x8009ee3c => {
                if !c.crbit(0) { pc = 0x8009ee50; continue; }
                pc = 0x8009ee40;
            }
            0x8009ee40 => {
                c.cmps(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x8009ef94; continue; }
                pc = 0x8009ee48;
            }
            0x8009ee48 => {
                if !c.crbit(0) { pc = 0x8009ee90; continue; }
                pc = 0x8009ee4c;
            }
            0x8009ee4c => {
                pc = 0x8009ef88; continue;
                pc = 0x8009ee50;
            }
            0x8009ee50 => {
                c.cmps(0, c.r[4], 0x4);
                if c.crbit(2) { pc = 0x8009eeb8; continue; }
                pc = 0x8009ee58;
            }
            0x8009ee58 => {
                if !c.crbit(0) { pc = 0x8009eec4; continue; }
                pc = 0x8009ee5c;
            }
            0x8009ee5c => {
                pc = 0x8009eeac; continue;
                pc = 0x8009ee60;
            }
            0x8009ee60 => {
                c.cmps(0, c.r[4], 0xc9);
                if c.crbit(2) { pc = 0x8009ef94; continue; }
                pc = 0x8009ee68;
            }
            0x8009ee68 => {
                if !c.crbit(0) { pc = 0x8009ee84; continue; }
                pc = 0x8009ee6c;
            }
            0x8009ee6c => {
                c.cmps(0, c.r[4], 0xa);
                if c.crbit(2) { pc = 0x8009eeec; continue; }
                pc = 0x8009ee74;
            }
            0x8009ee74 => {
                if c.crbit(0) { pc = 0x8009eedc; continue; }
                pc = 0x8009ee78;
            }
            0x8009ee78 => {
                c.cmps(0, c.r[4], 0xe);
                if !c.crbit(0) { pc = 0x8009ef88; continue; }
                pc = 0x8009ee80;
            }
            0x8009ee80 => {
                pc = 0x8009ef14; continue;
                pc = 0x8009ee84;
            }
            0x8009ee84 => {
                c.cmps(0, c.r[4], 0xcb);
                if !c.crbit(0) { pc = 0x8009ef88; continue; }
                pc = 0x8009ee8c;
            }
            0x8009ee8c => {
                pc = 0x8009ef24; continue;
                pc = 0x8009ee90;
            }
            0x8009ee90 => {
                c.r[0] = 0x0;
                c.stw(c.r[29], c.r[0]);
                pc = 0x8009ef94; continue;
                pc = 0x8009ee9c;
            }
            0x8009ee9c => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0xfc000003;
                c.stw(c.r[29], c.r[0]);
                pc = 0x8009ef94; continue;
                pc = 0x8009eeac;
            }
            0x8009eeac => {
                c.r[0] = 0x0;
                c.sth(c.r[29], c.r[0]);
                pc = 0x8009ef94; continue;
                pc = 0x8009eeb8;
            }
            0x8009eeb8 => {
                c.r[0] = 0x0;
                c.sth(c.r[29], c.r[0]);
                pc = 0x8009ef94; continue;
                pc = 0x8009eec4;
            }
            0x8009eec4 => {
                c.r[0] = 0x0;
                c.sth(c.r[29], c.r[0]);
                pc = 0x8009ef94; continue;
                pc = 0x8009eed0;
            }
            0x8009eed0 => {
                c.r[0] = 0x0;
                c.sth(c.r[29], c.r[0]);
                pc = 0x8009ef94; continue;
                pc = 0x8009eedc;
            }
            0x8009eedc => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff0003;
                c.stw(c.r[29], c.r[0]);
                pc = 0x8009ef94; continue;
                pc = 0x8009eeec;
            }
            0x8009eeec => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x32));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009ef00; continue; }
                pc = 0x8009eef8;
            }
            0x8009eef8 => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x3c));
                c.r[3] = c.r[0].wrapping_sub(c.r[29]);
                pc = 0x8009ef00;
            }
            0x8009ef00 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0xfc000003;
                c.r[0] = (c.r[3].rotate_left(0) & 0x3fffffc) | (c.r[0] & 0xfc000003);
                c.stw(c.r[29], c.r[0]);
                pc = 0x8009ef94; continue;
                pc = 0x8009ef14;
            }
            0x8009ef14 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff0003;
                c.stw(c.r[29], c.r[0]);
                pc = 0x8009ef94; continue;
                pc = 0x8009ef24;
            }
            0x8009ef24 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x3));
                c.cmpu(0, c.r[30], 0x0);
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3]);
                c.r[26] = c.r[3].wrapping_add(0x0);
                c.r[29] = c.r[0].rotate_left(0) & 0xfffffffe;
                if c.crbit(2) { pc = 0x8009ef68; continue; }
                pc = 0x8009ef48;
            }
            0x8009ef48 => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[25] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.r[3] = c.r[25].wrapping_add(0x0);
                c.lr = 0x8009ef5c; f_8009b2d0(c);
                pc = 0x8009ef5c;
            }
            0x8009ef5c => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x4));
                c.lr = 0x8009ef68; f_8009b3b4(c);
                pc = 0x8009ef68;
            }
            0x8009ef68 => {
                c.r[0] = c.lwz(c.r[26]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8009ef7c; continue; }
                pc = 0x8009ef74;
            }
            0x8009ef74 => {
                c.r[0] = c.r[26] | c.r[26];
                pc = 0x8009ef80; continue;
                pc = 0x8009ef7c;
            }
            0x8009ef7c => {
                c.r[0] = 0x0;
                pc = 0x8009ef80;
            }
            0x8009ef80 => {
                c.r[30] = c.r[0] | c.r[0];
                pc = 0x8009ef94; continue;
                pc = 0x8009ef88;
            }
            0x8009ef88 => {
                c.r[3] = c.r[27].wrapping_add(0x0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8009ef94; f_8009c2e0(c);
                pc = 0x8009ef94;
            }
            0x8009ef94 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x8009ef98;
            }
            0x8009ef98 => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x2));
                c.cmpu(0, c.r[4], 0xcb);
                if !c.crbit(2) { pc = 0x8009ee1c; continue; }
                pc = 0x8009efa4;
            }
            0x8009efa4 => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8009efcc; continue; }
                pc = 0x8009efac;
            }
            0x8009efac => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[27] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.lr = 0x8009efc0; f_8009b2d0(c);
                pc = 0x8009efc0;
            }
            0x8009efc0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x4));
                c.lr = 0x8009efcc; f_8009b3b4(c);
                pc = 0x8009efcc;
            }
            0x8009efcc => {
                c.r[3] = 0x1;
                pc = 0x8009efd0;
            }
            0x8009efd0 => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009edac: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009EFE4 (0x8009efe4, 468 bytes)
#[inline(never)]
pub fn f_8009efe4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009efe4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009efe4); f_8009efe4_b(c, 0x8009efe4); crate::prof::exit(c, 0x8009efe4); return; }
    }
    f_8009efe4_b(c, 0x8009efe4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009efe4_at(c: &mut Cpu, pc: u32) { f_8009efe4_b(c, pc) }
#[inline(always)]
fn f_8009efe4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009efe4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009efe4, pc); }
        match pc {
            0x8009efe4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[5] = c.lwz(c.r[3].wrapping_add(0x8));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x8009f018; continue; }
                pc = 0x8009f00c;
            }
            0x8009f00c => {
                c.r[3] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0x30cc), c.r[5]);
                pc = 0x8009f01c; continue;
                pc = 0x8009f018;
            }
            0x8009f018 => {
                c.stw(c.r[4].wrapping_add(0x8), c.r[5]);
                pc = 0x8009f01c;
            }
            0x8009f01c => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8009f030; continue; }
                pc = 0x8009f024;
            }
            0x8009f024 => {
                c.r[3] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0x30c8), c.r[4]);
                pc = 0x8009f034; continue;
                pc = 0x8009f030;
            }
            0x8009f030 => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[4]);
                pc = 0x8009f034;
            }
            0x8009f034 => {
                c.r[3] = 0x80000000;
                c.r[30] = c.lwz(c.r[3].wrapping_add(0x30c8));
                pc = 0x8009f050; continue;
                pc = 0x8009f040;
            }
            0x8009f040 => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.lr = 0x8009f04c; f_8009edac(c);
                pc = 0x8009f04c;
            }
            0x8009f04c => {
                c.r[30] = c.lwz(c.r[30].wrapping_add(0x4));
                pc = 0x8009f050;
            }
            0x8009f050 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8009f040; continue; }
                pc = 0x8009f058;
            }
            0x8009f058 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8009f060; f_8009e7ac(c);
                pc = 0x8009f060;
            }
            0x8009f060 => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x30d0));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009f07c; continue; }
                pc = 0x8009f070;
            }
            0x8009f070 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x14));
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[31].wrapping_add(0x14), c.r[0]);
                pc = 0x8009f07c;
            }
            0x8009f07c => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x30));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009f0a4; continue; }
                pc = 0x8009f088;
            }
            0x8009f088 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[3] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x34));
                c.r[3] = c.lwz(c.r[4].wrapping_add(c.r[3]));
                c.r[3] = c.r[3].rotate_left(0) & 0xfffffffe;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[31].wrapping_add(0x34), c.r[0]);
                pc = 0x8009f0a4;
            }
            0x8009f0a4 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x31));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009f0cc; continue; }
                pc = 0x8009f0b0;
            }
            0x8009f0b0 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[3] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x38));
                c.r[3] = c.lwz(c.r[4].wrapping_add(c.r[3]));
                c.r[3] = c.r[3].rotate_left(0) & 0xfffffffe;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[31].wrapping_add(0x38), c.r[0]);
                pc = 0x8009f0cc;
            }
            0x8009f0cc => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x32));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009f0f4; continue; }
                pc = 0x8009f0d8;
            }
            0x8009f0d8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[3] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x3c));
                c.r[3] = c.lwz(c.r[4].wrapping_add(c.r[3]));
                c.r[3] = c.r[3].rotate_left(0) & 0xfffffffe;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[31].wrapping_add(0x3c), c.r[0]);
                pc = 0x8009f0f4;
            }
            0x8009f0f4 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x28));
                pc = 0x8009f10c; continue;
                pc = 0x8009f0fc;
            }
            0x8009f0fc => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.r[0].wrapping_sub(c.r[31]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x8);
                pc = 0x8009f10c;
            }
            0x8009f10c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x28));
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x2c));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x8009f0fc; continue; }
                pc = 0x8009f120;
            }
            0x8009f120 => {
                c.r[6] = 0x1;
                c.r[5] = 0x8;
                c.r[3] = 0x0;
                pc = 0x8009f16c; continue;
                pc = 0x8009f130;
            }
            0x8009f130 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x33));
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x10));
                c.cmpu(0, c.r[6], c.r[0]);
                c.r[4] = c.r[4].wrapping_add(c.r[5]);
                if !c.crbit(2) { pc = 0x8009f150; continue; }
                pc = 0x8009f144;
            }
            0x8009f144 => {
                c.stb(c.r[31].wrapping_add(0x33), c.r[3]);
                c.stw(c.r[4], c.r[3]);
                pc = 0x8009f164; continue;
                pc = 0x8009f150;
            }
            0x8009f150 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009f164; continue; }
                pc = 0x8009f15c;
            }
            0x8009f15c => {
                c.r[0] = c.r[0].wrapping_sub(c.r[31]);
                c.stw(c.r[4], c.r[0]);
                pc = 0x8009f164;
            }
            0x8009f164 => {
                c.r[5] = c.r[5].wrapping_add(0x8);
                c.r[6] = c.r[6].wrapping_add(0x1);
                pc = 0x8009f16c;
            }
            0x8009f16c => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.cmpu(0, c.r[6], c.r[0]);
                if c.crbit(0) { pc = 0x8009f130; continue; }
                pc = 0x8009f178;
            }
            0x8009f178 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x24));
                c.r[3] = 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[31]);
                c.stw(c.r[31].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x28));
                c.r[0] = c.r[0].wrapping_sub(c.r[31]);
                c.stw(c.r[31].wrapping_add(0x28), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[0] = c.r[0].wrapping_sub(c.r[31]);
                c.stw(c.r[31].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009efe4: bad pc {:#x}", pc),
        }
    }
}

/// __OSModuleInit (0x8009f1b8, 24 bytes)
#[inline(never)]
pub fn f_8009f1b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f1b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f1b8); f_8009f1b8_b(c, 0x8009f1b8); crate::prof::exit(c, 0x8009f1b8); return; }
    }
    f_8009f1b8_b(c, 0x8009f1b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f1b8_at(c: &mut Cpu, pc: u32) { f_8009f1b8_b(c, pc) }
#[inline(always)]
fn f_8009f1b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f1b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f1b8, pc); }
        match pc {
            0x8009f1b8 => {
                c.r[4] = 0x80000000;
                c.r[0] = 0x0;
                c.stw(c.r[4].wrapping_add(0x30cc), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x30c8), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x30d0), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f1b8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009F1D0 (0x8009f1d0, 96 bytes)
#[inline(never)]
pub fn f_8009f1d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f1d0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f1d0); f_8009f1d0_b(c, 0x8009f1d0); crate::prof::exit(c, 0x8009f1d0); return; }
    }
    f_8009f1d0_b(c, 0x8009f1d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f1d0_at(c: &mut Cpu, pc: u32) { f_8009f1d0_b(c, pc) }
#[inline(always)]
fn f_8009f1d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f1d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f1d0, pc); }
        match pc {
            0x8009f1d0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.lr = 0x8009f1f8; f_800a13e8(c);
                pc = 0x8009f1f8;
            }
            0x8009f1f8 => {
                c.r[3] = c.r[29].wrapping_add(0x8);
                c.lr = 0x8009f200; f_800a13e8(c);
                pc = 0x8009f200;
            }
            0x8009f200 => {
                c.stw(c.r[29].wrapping_add(0x10), c.r[30]);
                c.r[0] = 0x0;
                c.stw(c.r[29].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[29].wrapping_add(0x18), c.r[0]);
                c.stw(c.r[29].wrapping_add(0x1c), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f1d0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009F230 (0x8009f230, 200 bytes)
#[inline(never)]
pub fn f_8009f230(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f230) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f230); f_8009f230_b(c, 0x8009f230); crate::prof::exit(c, 0x8009f230); return; }
    }
    f_8009f230_b(c, 0x8009f230)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f230_at(c: &mut Cpu, pc: u32) { f_8009f230_b(c, pc) }
#[inline(always)]
fn f_8009f230_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f230;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f230, pc); }
        match pc {
            0x8009f230 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x18), c.r[28]);
                c.r[28] = c.r[3].wrapping_add(0x0);
                c.lr = 0x8009f25c; f_8009df3c(c);
                pc = 0x8009f25c;
            }
            0x8009f25c => {
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[31] = c.r[31].rotate_left(0) & 0x1;
                pc = 0x8009f288; continue;
                pc = 0x8009f268;
            }
            0x8009f268 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8009f280; continue; }
                pc = 0x8009f270;
            }
            0x8009f270 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009f278; f_8009df64(c);
                pc = 0x8009f278;
            }
            0x8009f278 => {
                c.r[3] = 0x0;
                pc = 0x8009f2d8; continue;
                pc = 0x8009f280;
            }
            0x8009f280 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8009f288; f_800a238c(c);
                pc = 0x8009f288;
            }
            0x8009f288 => {
                c.r[6] = c.lwz(c.r[28].wrapping_add(0x14));
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x1c));
                c.cmps(0, c.r[6], c.r[4]);
                if !c.crbit(1) { pc = 0x8009f268; continue; }
                pc = 0x8009f298;
            }
            0x8009f298 => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x18));
                c.r[3] = c.r[28].wrapping_add(0x8);
                c.r[5] = c.lwz(c.r[28].wrapping_add(0x10));
                c.r[4] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = divw(c.r[4], c.r[6]);
                c.r[0] = c.r[0].wrapping_mul(c.r[6]);
                c.r[0] = c.r[4].wrapping_sub(c.r[0]);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[5].wrapping_add(c.r[0]), c.r[29]);
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x1c));
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[28].wrapping_add(0x1c), c.r[0]);
                c.lr = 0x8009f2cc; f_800a2478(c);
                pc = 0x8009f2cc;
            }
            0x8009f2cc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009f2d4; f_8009df64(c);
                pc = 0x8009f2d4;
            }
            0x8009f2d4 => {
                c.r[3] = 0x1;
                pc = 0x8009f2d8;
            }
            0x8009f2d8 => {
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
            _ => panic!("f_8009f230: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009F2F8 (0x8009f2f8, 220 bytes)
#[inline(never)]
pub fn f_8009f2f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f2f8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f2f8); f_8009f2f8_b(c, 0x8009f2f8); crate::prof::exit(c, 0x8009f2f8); return; }
    }
    f_8009f2f8_b(c, 0x8009f2f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f2f8_at(c: &mut Cpu, pc: u32) { f_8009f2f8_b(c, pc) }
#[inline(always)]
fn f_8009f2f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f2f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f2f8, pc); }
        match pc {
            0x8009f2f8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[28]);
                c.r[28] = c.r[4].wrapping_add(0x0);
                c.lr = 0x8009f324; f_8009df3c(c);
                pc = 0x8009f324;
            }
            0x8009f324 => {
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[30] = c.r[30].rotate_left(0) & 0x1;
                pc = 0x8009f350; continue;
                pc = 0x8009f330;
            }
            0x8009f330 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8009f348; continue; }
                pc = 0x8009f338;
            }
            0x8009f338 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009f340; f_8009df64(c);
                pc = 0x8009f340;
            }
            0x8009f340 => {
                c.r[3] = 0x0;
                pc = 0x8009f3b4; continue;
                pc = 0x8009f348;
            }
            0x8009f348 => {
                c.r[3] = c.r[31].wrapping_add(0x8);
                c.lr = 0x8009f350; f_800a238c(c);
                pc = 0x8009f350;
            }
            0x8009f350 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x1c));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009f330; continue; }
                pc = 0x8009f35c;
            }
            0x8009f35c => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x8009f378; continue; }
                pc = 0x8009f364;
            }
            0x8009f364 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x18));
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.stw(c.r[28], c.r[0]);
                pc = 0x8009f378;
            }
            0x8009f378 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x18));
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x14));
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.r[0] = divw(c.r[5], c.r[4]);
                c.r[0] = c.r[0].wrapping_mul(c.r[4]);
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                c.stw(c.r[31].wrapping_add(0x18), c.r[0]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x1c));
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                c.stw(c.r[31].wrapping_add(0x1c), c.r[0]);
                c.lr = 0x8009f3a8; f_800a2478(c);
                pc = 0x8009f3a8;
            }
            0x8009f3a8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009f3b0; f_8009df64(c);
                pc = 0x8009f3b0;
            }
            0x8009f3b0 => {
                c.r[3] = 0x1;
                pc = 0x8009f3b4;
            }
            0x8009f3b4 => {
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
            _ => panic!("f_8009f2f8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009F3D4 (0x8009f3d4, 12 bytes)
#[inline(never)]
pub fn f_8009f3d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f3d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f3d4); f_8009f3d4_b(c, 0x8009f3d4); crate::prof::exit(c, 0x8009f3d4); return; }
    }
    f_8009f3d4_b(c, 0x8009f3d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f3d4_at(c: &mut Cpu, pc: u32) { f_8009f3d4_b(c, pc) }
#[inline(always)]
fn f_8009f3d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f3d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f3d4, pc); }
        match pc {
            0x8009f3d4 => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x28));
                if true { return; }
                return;
            }
            _ => panic!("f_8009f3d4: bad pc {:#x}", pc),
        }
    }
}

/// OnReset_800AF628 (0x8009f3e0, 60 bytes)
#[inline(never)]
pub fn f_8009f3e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f3e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f3e0); f_8009f3e0_b(c, 0x8009f3e0); crate::prof::exit(c, 0x8009f3e0); return; }
    }
    f_8009f3e0_b(c, 0x8009f3e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f3e0_at(c: &mut Cpu, pc: u32) { f_8009f3e0_b(c, pc) }
#[inline(always)]
fn f_8009f3e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f3e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f3e0, pc); }
        match pc {
            0x8009f3e0 => {
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                if c.crbit(2) { pc = 0x8009f408; continue; }
                pc = 0x8009f3f4;
            }
            0x8009f3f4 => {
                c.r[3] = 0xcc000000;
                c.r[0] = 0xff;
                c.sth(c.r[3].wrapping_add(0x4010), c.r[0]);
                c.r[3] = 0xf0000000;
                c.lr = 0x8009f408; f_8009e304(c);
                pc = 0x8009f408;
            }
            0x8009f408 => {
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f3e0: bad pc {:#x}", pc),
        }
    }
}

/// MEMIntrruptHandler (0x8009f41c, 108 bytes)
#[inline(never)]
pub fn f_8009f41c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f41c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f41c); f_8009f41c_b(c, 0x8009f41c); crate::prof::exit(c, 0x8009f41c); return; }
    }
    f_8009f41c_b(c, 0x8009f41c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f41c_at(c: &mut Cpu, pc: u32) { f_8009f41c_b(c, pc) }
#[inline(always)]
fn f_8009f41c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f41c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f41c, pc); }
        match pc {
            0x8009f41c => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0xcc000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[8] = c.r[3].wrapping_add(0x4000);
                c.r[0] = 0x0;
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[7] = c.lhz(c.r[3].wrapping_add(0x4024));
                c.r[3] = 0x80400000;
                c.r[6] = c.lhz(c.r[8].wrapping_add(0x22));
                c.r[3] = c.r[3].wrapping_add(0xffffb4f0);
                c.r[5] = c.lhz(c.r[8].wrapping_add(0x1e));
                c.r[6] = (c.r[7].rotate_left(16) & 0x3ff0000) | (c.r[6] & 0xfc00ffff);
                c.sth(c.r[8].wrapping_add(0x20), c.r[0]);
                c.r[12] = c.lwz(c.r[3].wrapping_add(0x3c));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x8009f470; continue; }
                pc = 0x8009f45c;
            }
            0x8009f45c => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0xf;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                if true { let t = c.lr; c.lr = 0x8009f46c; (c.dispatch)(c, t); }
                pc = 0x8009f46c;
            }
            0x8009f46c => {
                pc = 0x8009f478; continue;
                pc = 0x8009f470;
            }
            0x8009f470 => {
                c.r[3] = 0xf;
                c.lr = 0x8009f478; f_8009c578(c);
                pc = 0x8009f478;
            }
            0x8009f478 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f41c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009F488 (0x8009f488, 196 bytes)
#[inline(never)]
pub fn f_8009f488(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f488) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f488); f_8009f488_b(c, 0x8009f488); crate::prof::exit(c, 0x8009f488); return; }
    }
    f_8009f488_b(c, 0x8009f488)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f488_at(c: &mut Cpu, pc: u32) { f_8009f488_b(c, pc) }
#[inline(always)]
fn f_8009f488_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f488;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f488, pc); }
        match pc {
            0x8009f488 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[26], 0x4);
                if !c.crbit(0) { pc = 0x8009f538; continue; }
                pc = 0x8009f4a4;
            }
            0x8009f4a4 => {
                c.r[3] = c.r[4].wrapping_add(c.r[5]);
                c.r[0] = c.r[3].wrapping_add(0x3ff);
                c.r[27] = c.r[4].rotate_left(0) & 0xfffffc00;
                c.r[29] = c.r[0].rotate_left(0) & 0xfffffc00;
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[31] = c.r[6].rotate_left(0) & 0x3;
                c.r[4] = c.r[29].wrapping_sub(c.r[27]);
                c.lr = 0x8009f4c4; f_8009b2d0(c);
                pc = 0x8009f4c4;
            }
            0x8009f4c4 => {
                c.lr = 0x8009f4c8; f_8009df3c(c);
                pc = 0x8009f4c8;
            }
            0x8009f4c8 => {
                c.r[0] = 0x80000000;
                c.r[30] = srw(c.r[0], c.r[26]);
                c.r[28] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.lr = 0x8009f4dc; f_8009e304(c);
                pc = 0x8009f4dc;
            }
            0x8009f4dc => {
                c.r[3] = 0xcc000000;
                c.r[5] = c.r[3].wrapping_add(0x4000);
                c.r[3] = c.r[26].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[27].rotate_left(22) & 0xffff;
                c.sth(c.r[5].wrapping_add(c.r[3]), c.r[0]);
                c.r[4] = c.r[29].rotate_left(22) & 0xffff;
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                c.r[4] = c.r[5].wrapping_add(0x10);
                c.r[6] = c.r[26].rotate_left(1) & 0xfffffffe;
                c.r[0] = 0x3;
                c.r[5] = c.lhz(c.r[5].wrapping_add(0x10));
                c.r[3] = slw(c.r[0], c.r[6]);
                c.r[0] = slw(c.r[31], c.r[6]);
                c.r[5] = c.r[5] & !c.r[3];
                c.r[5] = c.r[5] | c.r[0];
                c.cmpu(0, c.r[31], 0x3);
                c.sth(c.r[4], c.r[5]);
                if c.crbit(2) { pc = 0x8009f530; continue; }
                pc = 0x8009f528;
            }
            0x8009f528 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009f530; f_8009e38c(c);
                pc = 0x8009f530;
            }
            0x8009f530 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8009f538; f_8009df64(c);
                pc = 0x8009f538;
            }
            0x8009f538 => {
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f488: bad pc {:#x}", pc),
        }
    }
}

/// Config24MB (0x8009f54c, 128 bytes)
#[inline(never)]
pub fn f_8009f54c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f54c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f54c); f_8009f54c_b(c, 0x8009f54c); crate::prof::exit(c, 0x8009f54c); return; }
    }
    f_8009f54c_b(c, 0x8009f54c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f54c_at(c: &mut Cpu, pc: u32) { f_8009f54c_b(c, pc) }
#[inline(always)]
fn f_8009f54c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f54c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f54c, pc); }
        match pc {
            0x8009f54c => {
                c.r[7] = 0x0;
                c.r[4] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x2);
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x1ff);
                c.r[6] = 0x1000000;
                c.r[6] = c.r[6].wrapping_add(0x2);
                c.r[5] = 0x81000000;
                c.r[5] = c.r[5].wrapping_add(0xff);
                // isync
                c.mtspr(536, c.r[7]);
                c.mtspr(537, c.r[4]);
                c.mtspr(536, c.r[3]);
                // isync
                c.mtspr(528, c.r[7]);
                c.mtspr(529, c.r[4]);
                c.mtspr(528, c.r[3]);
                // isync
                c.mtspr(540, c.r[7]);
                c.mtspr(541, c.r[6]);
                c.mtspr(540, c.r[5]);
                // isync
                c.mtspr(532, c.r[7]);
                c.mtspr(533, c.r[6]);
                c.mtspr(532, c.r[5]);
                // isync
                c.r[3] = 0; // mfmsr
                c.r[3] = c.r[3] | 0x30;
                c.mtspr(27, c.r[3]);
                c.r[3] = c.mfspr(8);
                c.mtspr(26, c.r[3]);
                // rfi
                f_8009f5cc(c); return;
                return;
            }
            _ => panic!("f_8009f54c: bad pc {:#x}", pc),
        }
    }
}

/// Config48MB (0x8009f5cc, 128 bytes)
#[inline(never)]
pub fn f_8009f5cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f5cc) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f5cc); f_8009f5cc_b(c, 0x8009f5cc); crate::prof::exit(c, 0x8009f5cc); return; }
    }
    f_8009f5cc_b(c, 0x8009f5cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f5cc_at(c: &mut Cpu, pc: u32) { f_8009f5cc_b(c, pc) }
#[inline(always)]
fn f_8009f5cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f5cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f5cc, pc); }
        match pc {
            0x8009f5cc => {
                c.r[7] = 0x0;
                c.r[4] = 0x0;
                c.r[4] = c.r[4].wrapping_add(0x2);
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x3ff);
                c.r[6] = 0x2000000;
                c.r[6] = c.r[6].wrapping_add(0x2);
                c.r[5] = 0x82000000;
                c.r[5] = c.r[5].wrapping_add(0x1ff);
                // isync
                c.mtspr(536, c.r[7]);
                c.mtspr(537, c.r[4]);
                c.mtspr(536, c.r[3]);
                // isync
                c.mtspr(528, c.r[7]);
                c.mtspr(529, c.r[4]);
                c.mtspr(528, c.r[3]);
                // isync
                c.mtspr(540, c.r[7]);
                c.mtspr(541, c.r[6]);
                c.mtspr(540, c.r[5]);
                // isync
                c.mtspr(532, c.r[7]);
                c.mtspr(533, c.r[6]);
                c.mtspr(532, c.r[5]);
                // isync
                c.r[3] = 0; // mfmsr
                c.r[3] = c.r[3] | 0x30;
                c.mtspr(27, c.r[3]);
                c.r[3] = c.mfspr(8);
                c.mtspr(26, c.r[3]);
                // rfi
                f_8009f64c(c); return;
                return;
            }
            _ => panic!("f_8009f5cc: bad pc {:#x}", pc),
        }
    }
}

/// RealMode (0x8009f64c, 24 bytes)
#[inline(never)]
pub fn f_8009f64c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f64c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f64c); f_8009f64c_b(c, 0x8009f64c); crate::prof::exit(c, 0x8009f64c); return; }
    }
    f_8009f64c_b(c, 0x8009f64c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f64c_at(c: &mut Cpu, pc: u32) { f_8009f64c_b(c, pc) }
#[inline(always)]
fn f_8009f64c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f64c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f64c, pc); }
        match pc {
            0x8009f64c => {
                c.r[3] = c.r[3].rotate_left(0) & 0x3fffffff;
                c.mtspr(26, c.r[3]);
                c.r[3] = 0; // mfmsr
                c.r[3] = c.r[3].rotate_left(0) & 0xffffffcf;
                c.mtspr(27, c.r[3]);
                // rfi
                f_8009f664(c); return;
                return;
            }
            _ => panic!("f_8009f64c: bad pc {:#x}", pc),
        }
    }
}

/// __OSInitMemoryProtection (0x8009f664, 280 bytes)
#[inline(never)]
pub fn f_8009f664(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f664) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f664); f_8009f664_b(c, 0x8009f664); crate::prof::exit(c, 0x8009f664); return; }
    }
    f_8009f664_b(c, 0x8009f664)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f664_at(c: &mut Cpu, pc: u32) { f_8009f664_b(c, pc) }
#[inline(always)]
fn f_8009f664_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f664;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f664, pc); }
        match pc {
            0x8009f664 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = 0x80000000;
                c.r[31] = c.lwz(c.r[27].wrapping_add(0xf0));
                c.lr = 0x8009f680; f_8009df3c(c);
                pc = 0x8009f680;
            }
            0x8009f680 => {
                c.r[4] = 0xcc000000;
                c.r[28] = c.r[4].wrapping_add(0x4000);
                c.r[0] = 0x0;
                c.sth(c.r[28].wrapping_add(0x20), c.r[0]);
                c.r[0] = 0xff;
                c.r[30] = c.r[3] | c.r[3];
                c.sth(c.r[28].wrapping_add(0x10), c.r[0]);
                c.r[3] = 0xf0000000;
                c.lr = 0x8009f6a4; f_8009e304(c);
                pc = 0x8009f6a4;
            }
            0x8009f6a4 => {
                c.r[3] = 0x800a0000;
                c.r[29] = c.r[3].wrapping_add(0xfffff41c);
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.lr = 0x8009f6b8; f_8009df88(c);
                pc = 0x8009f6b8;
            }
            0x8009f6b8 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x1;
                c.lr = 0x8009f6c4; f_8009df88(c);
                pc = 0x8009f6c4;
            }
            0x8009f6c4 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x2;
                c.lr = 0x8009f6d0; f_8009df88(c);
                pc = 0x8009f6d0;
            }
            0x8009f6d0 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x3;
                c.lr = 0x8009f6dc; f_8009df88(c);
                pc = 0x8009f6dc;
            }
            0x8009f6dc => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x4;
                c.lr = 0x8009f6e8; f_8009df88(c);
                pc = 0x8009f6e8;
            }
            0x8009f6e8 => {
                c.r[3] = 0x80310000;
                c.r[3] = c.r[3].wrapping_add(0x1868);
                c.lr = 0x8009f6f4; f_8009fe38(c);
                pc = 0x8009f6f4;
            }
            0x8009f6f4 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0xf0));
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x28));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x8009f724; continue; }
                pc = 0x8009f704;
            }
            0x8009f704 => {
                c.r[0] = c.r[3].wrapping_add(0xfe800000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009f724; continue; }
                pc = 0x8009f710;
            }
            0x8009f710 => {
                c.r[3] = 0x81800000;
                c.r[4] = 0x1800000;
                c.lr = 0x8009f71c; f_8009b2a4(c);
                pc = 0x8009f71c;
            }
            0x8009f71c => {
                c.r[0] = 0x2;
                c.sth(c.r[28].wrapping_add(0x28), c.r[0]);
                pc = 0x8009f724;
            }
            0x8009f724 => {
                c.r[0] = 0x1800000;
                c.cmpu(0, c.r[31], c.r[0]);
                if c.crbit(1) { pc = 0x8009f740; continue; }
                pc = 0x8009f730;
            }
            0x8009f730 => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0xfffff54c);
                c.lr = 0x8009f73c; f_8009f64c(c);
                pc = 0x8009f73c;
            }
            0x8009f73c => {
                pc = 0x8009f758; continue;
                pc = 0x8009f740;
            }
            0x8009f740 => {
                c.r[0] = 0x3000000;
                c.cmpu(0, c.r[31], c.r[0]);
                if c.crbit(1) { pc = 0x8009f758; continue; }
                pc = 0x8009f74c;
            }
            0x8009f74c => {
                c.r[3] = 0x800a0000;
                c.r[3] = c.r[3].wrapping_add(0xfffff5cc);
                c.lr = 0x8009f758; f_8009f64c(c);
                pc = 0x8009f758;
            }
            0x8009f758 => {
                c.r[3] = 0x8000000;
                c.lr = 0x8009f760; f_8009e38c(c);
                pc = 0x8009f760;
            }
            0x8009f760 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009f768; f_8009df64(c);
                pc = 0x8009f768;
            }
            0x8009f768 => {
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[1] = c.r[1].wrapping_add(0x50);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f664: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009F77C (0x8009f77c, 56 bytes)
#[inline(never)]
pub fn f_8009f77c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f77c) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f77c); f_8009f77c_b(c, 0x8009f77c); crate::prof::exit(c, 0x8009f77c); return; }
    }
    f_8009f77c_b(c, 0x8009f77c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f77c_at(c: &mut Cpu, pc: u32) { f_8009f77c_b(c, pc) }
#[inline(always)]
fn f_8009f77c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f77c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f77c, pc); }
        match pc {
            0x8009f77c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8009f794; f_800a13e8(c);
                pc = 0x8009f794;
            }
            0x8009f794 => {
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f77c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009F7B4 (0x8009f7b4, 220 bytes)
#[inline(never)]
pub fn f_8009f7b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f7b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f7b4); f_8009f7b4_b(c, 0x8009f7b4); crate::prof::exit(c, 0x8009f7b4); return; }
    }
    f_8009f7b4_b(c, 0x8009f7b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f7b4_at(c: &mut Cpu, pc: u32) { f_8009f7b4_b(c, pc) }
#[inline(always)]
fn f_8009f7b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f7b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f7b4, pc); }
        match pc {
            0x8009f7b4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8009f7d8; f_8009df3c(c);
                pc = 0x8009f7d8;
            }
            0x8009f7d8 => {
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x8009f7e0; f_800a13f8(c);
                pc = 0x8009f7e0;
            }
            0x8009f7e0 => {
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[31] = 0x0;
                pc = 0x8009f7e8;
            }
            0x8009f7e8 => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009f830; continue; }
                pc = 0x8009f7f4;
            }
            0x8009f7f4 => {
                c.stw(c.r[28].wrapping_add(0x8), c.r[30]);
                c.r[3] = c.lwz(c.r[28].wrapping_add(0xc));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[28].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x2f8));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009f818; continue; }
                pc = 0x8009f810;
            }
            0x8009f810 => {
                c.stw(c.r[30].wrapping_add(0x2f4), c.r[28]);
                pc = 0x8009f81c; continue;
                pc = 0x8009f818;
            }
            0x8009f818 => {
                c.stw(c.r[3].wrapping_add(0x10), c.r[28]);
                pc = 0x8009f81c;
            }
            0x8009f81c => {
                c.stw(c.r[28].wrapping_add(0x14), c.r[3]);
                c.r[0] = 0x0;
                c.stw(c.r[28].wrapping_add(0x10), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x2f8), c.r[28]);
                pc = 0x8009f868; continue;
                pc = 0x8009f830;
            }
            0x8009f830 => {
                c.cmpu(0, c.r[0], c.r[30]);
                if !c.crbit(2) { pc = 0x8009f848; continue; }
                pc = 0x8009f838;
            }
            0x8009f838 => {
                c.r[3] = c.lwz(c.r[28].wrapping_add(0xc));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[28].wrapping_add(0xc), c.r[0]);
                pc = 0x8009f868; continue;
                pc = 0x8009f848;
            }
            0x8009f848 => {
                c.stw(c.r[30].wrapping_add(0x2f0), c.r[28]);
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x8));
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x2d0));
                c.lr = 0x8009f858; f_800a16e8(c);
                pc = 0x8009f858;
            }
            0x8009f858 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8009f860; f_800a238c(c);
                pc = 0x8009f860;
            }
            0x8009f860 => {
                c.stw(c.r[30].wrapping_add(0x2f0), c.r[31]);
                pc = 0x8009f7e8; continue;
                pc = 0x8009f868;
            }
            0x8009f868 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009f870; f_8009df64(c);
                pc = 0x8009f870;
            }
            0x8009f870 => {
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
            _ => panic!("f_8009f7b4: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009F890 (0x8009f890, 200 bytes)
#[inline(never)]
pub fn f_8009f890(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f890) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f890); f_8009f890_b(c, 0x8009f890); crate::prof::exit(c, 0x8009f890); return; }
    }
    f_8009f890_b(c, 0x8009f890)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f890_at(c: &mut Cpu, pc: u32) { f_8009f890_b(c, pc) }
#[inline(always)]
fn f_8009f890_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f890;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f890, pc); }
        match pc {
            0x8009f890 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x8009f8b0; f_8009df3c(c);
                pc = 0x8009f8b0;
            }
            0x8009f8b0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8009f8b8; f_800a13f8(c);
                pc = 0x8009f8b8;
            }
            0x8009f8b8 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[0], c.r[30]);
                if !c.crbit(2) { pc = 0x8009f934; continue; }
                pc = 0x8009f8c8;
            }
            0x8009f8c8 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[0] = c.addc(c.r[3], 0xffffffff, 0);
                c.rc(c.r[0]);
                c.stw(c.r[29].wrapping_add(0xc), c.r[0]);
                if !c.crbit(2) { pc = 0x8009f934; continue; }
                pc = 0x8009f8d8;
            }
            0x8009f8d8 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x14));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009f8f0; continue; }
                pc = 0x8009f8e8;
            }
            0x8009f8e8 => {
                c.stw(c.r[30].wrapping_add(0x2f8), c.r[4]);
                pc = 0x8009f8f4; continue;
                pc = 0x8009f8f0;
            }
            0x8009f8f0 => {
                c.stw(c.r[3].wrapping_add(0x14), c.r[4]);
                pc = 0x8009f8f4;
            }
            0x8009f8f4 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x8009f904; continue; }
                pc = 0x8009f8fc;
            }
            0x8009f8fc => {
                c.stw(c.r[30].wrapping_add(0x2f4), c.r[3]);
                pc = 0x8009f908; continue;
                pc = 0x8009f904;
            }
            0x8009f904 => {
                c.stw(c.r[4].wrapping_add(0x10), c.r[3]);
                pc = 0x8009f908;
            }
            0x8009f908 => {
                c.r[0] = 0x0;
                c.stw(c.r[29].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x2d0));
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x2d4));
                c.cmps(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x8009f92c; continue; }
                pc = 0x8009f920;
            }
            0x8009f920 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009f928; f_800a14ec(c);
                pc = 0x8009f928;
            }
            0x8009f928 => {
                c.stw(c.r[30].wrapping_add(0x2d0), c.r[3]);
                pc = 0x8009f92c;
            }
            0x8009f92c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8009f934; f_800a2478(c);
                pc = 0x8009f934;
            }
            0x8009f934 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8009f93c; f_8009df64(c);
                pc = 0x8009f93c;
            }
            0x8009f93c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f890: bad pc {:#x}", pc),
        }
    }
}

/// __OSUnlockAllMutex (0x8009f958, 112 bytes)
#[inline(never)]
pub fn f_8009f958(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f958) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f958); f_8009f958_b(c, 0x8009f958); crate::prof::exit(c, 0x8009f958); return; }
    }
    f_8009f958_b(c, 0x8009f958)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f958_at(c: &mut Cpu, pc: u32) { f_8009f958_b(c, pc) }
#[inline(always)]
fn f_8009f958_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f958;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f958, pc); }
        match pc {
            0x8009f958 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                pc = 0x8009f9a4; continue;
                pc = 0x8009f978;
            }
            0x8009f978 => {
                c.r[5] = c.lwz(c.r[4].wrapping_add(0x10));
                c.r[3] = c.r[4].wrapping_add(0x0);
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8009f990; continue; }
                pc = 0x8009f988;
            }
            0x8009f988 => {
                c.stw(c.r[30].wrapping_add(0x2f8), c.r[31]);
                pc = 0x8009f994; continue;
                pc = 0x8009f990;
            }
            0x8009f990 => {
                c.stw(c.r[5].wrapping_add(0x14), c.r[31]);
                pc = 0x8009f994;
            }
            0x8009f994 => {
                c.stw(c.r[30].wrapping_add(0x2f4), c.r[5]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[31]);
                c.lr = 0x8009f9a4; f_800a2478(c);
                pc = 0x8009f9a4;
            }
            0x8009f9a4 => {
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x2f4));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x8009f978; continue; }
                pc = 0x8009f9b0;
            }
            0x8009f9b0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f958: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009F9C8 (0x8009f9c8, 32 bytes)
#[inline(never)]
pub fn f_8009f9c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f9c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f9c8); f_8009f9c8_b(c, 0x8009f9c8); crate::prof::exit(c, 0x8009f9c8); return; }
    }
    f_8009f9c8_b(c, 0x8009f9c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f9c8_at(c: &mut Cpu, pc: u32) { f_8009f9c8_b(c, pc) }
#[inline(always)]
fn f_8009f9c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f9c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f9c8, pc); }
        match pc {
            0x8009f9c8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x8009f9d8; f_800a13e8(c);
                pc = 0x8009f9d8;
            }
            0x8009f9d8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f9c8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009F9E8 (0x8009f9e8, 212 bytes)
#[inline(never)]
pub fn f_8009f9e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009f9e8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009f9e8); f_8009f9e8_b(c, 0x8009f9e8); crate::prof::exit(c, 0x8009f9e8); return; }
    }
    f_8009f9e8_b(c, 0x8009f9e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009f9e8_at(c: &mut Cpu, pc: u32) { f_8009f9e8_b(c, pc) }
#[inline(always)]
fn f_8009f9e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009f9e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009f9e8, pc); }
        match pc {
            0x8009f9e8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3].wrapping_add(0x0);
                c.r[28] = c.r[4].wrapping_add(0x0);
                c.lr = 0x8009fa04; f_8009df3c(c);
                pc = 0x8009fa04;
            }
            0x8009fa04 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8009fa0c; f_800a13f8(c);
                pc = 0x8009fa0c;
            }
            0x8009fa0c => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x8));
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[0], c.r[30]);
                if !c.crbit(2) { pc = 0x8009faa0; continue; }
                pc = 0x8009fa1c;
            }
            0x8009fa1c => {
                c.r[29] = c.lwz(c.r[28].wrapping_add(0xc));
                c.r[0] = 0x0;
                c.stw(c.r[28].wrapping_add(0xc), c.r[0]);
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x10));
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x14));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009fa40; continue; }
                pc = 0x8009fa38;
            }
            0x8009fa38 => {
                c.stw(c.r[30].wrapping_add(0x2f8), c.r[4]);
                pc = 0x8009fa44; continue;
                pc = 0x8009fa40;
            }
            0x8009fa40 => {
                c.stw(c.r[3].wrapping_add(0x14), c.r[4]);
                pc = 0x8009fa44;
            }
            0x8009fa44 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x8009fa54; continue; }
                pc = 0x8009fa4c;
            }
            0x8009fa4c => {
                c.stw(c.r[30].wrapping_add(0x2f4), c.r[3]);
                pc = 0x8009fa58; continue;
                pc = 0x8009fa54;
            }
            0x8009fa54 => {
                c.stw(c.r[4].wrapping_add(0x10), c.r[3]);
                pc = 0x8009fa58;
            }
            0x8009fa58 => {
                c.r[0] = 0x0;
                c.stw(c.r[28].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x2d0));
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x2d4));
                c.cmps(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x8009fa7c; continue; }
                pc = 0x8009fa70;
            }
            0x8009fa70 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8009fa78; f_800a14ec(c);
                pc = 0x8009fa78;
            }
            0x8009fa78 => {
                c.stw(c.r[30].wrapping_add(0x2d0), c.r[3]);
                pc = 0x8009fa7c;
            }
            0x8009fa7c => {
                c.lr = 0x8009fa80; f_800a1404(c);
                pc = 0x8009fa80;
            }
            0x8009fa80 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8009fa88; f_800a2478(c);
                pc = 0x8009fa88;
            }
            0x8009fa88 => {
                c.lr = 0x8009fa8c; f_800a1444(c);
                pc = 0x8009fa8c;
            }
            0x8009fa8c => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8009fa94; f_800a238c(c);
                pc = 0x8009fa94;
            }
            0x8009fa94 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8009fa9c; f_8009f7b4(c);
                pc = 0x8009fa9c;
            }
            0x8009fa9c => {
                c.stw(c.r[28].wrapping_add(0xc), c.r[29]);
                pc = 0x8009faa0;
            }
            0x8009faa0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8009faa8; f_8009df64(c);
                pc = 0x8009faa8;
            }
            0x8009faa8 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009f9e8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009FABC (0x8009fabc, 32 bytes)
#[inline(never)]
pub fn f_8009fabc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009fabc) { return; }
        if c.prof { crate::prof::enter(c, 0x8009fabc); f_8009fabc_b(c, 0x8009fabc); crate::prof::exit(c, 0x8009fabc); return; }
    }
    f_8009fabc_b(c, 0x8009fabc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009fabc_at(c: &mut Cpu, pc: u32) { f_8009fabc_b(c, pc) }
#[inline(always)]
fn f_8009fabc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009fabc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009fabc, pc); }
        match pc {
            0x8009fabc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x8009facc; f_800a2478(c);
                pc = 0x8009facc;
            }
            0x8009facc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009fabc: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009FADC (0x8009fadc, 16 bytes)
#[inline(never)]
pub fn f_8009fadc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009fadc) { return; }
        if c.prof { crate::prof::enter(c, 0x8009fadc); f_8009fadc_b(c, 0x8009fadc); crate::prof::exit(c, 0x8009fadc); return; }
    }
    f_8009fadc_b(c, 0x8009fadc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009fadc_at(c: &mut Cpu, pc: u32) { f_8009fadc_b(c, pc) }
#[inline(always)]
fn f_8009fadc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009fadc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009fadc, pc); }
        match pc {
            0x8009fadc => {
                // sync
                // isync
                c.mtspr(8, c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009fadc: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009FAEC (0x8009faec, 12 bytes)
#[inline(never)]
pub fn f_8009faec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009faec) { return; }
        if c.prof { crate::prof::enter(c, 0x8009faec); f_8009faec_b(c, 0x8009faec); crate::prof::exit(c, 0x8009faec); return; }
    }
    f_8009faec_b(c, 0x8009faec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009faec_at(c: &mut Cpu, pc: u32) { f_8009faec_b(c, pc) }
#[inline(always)]
fn f_8009faec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009faec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009faec, pc); }
        match pc {
            0x8009faec => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f10), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009faec: bad pc {:#x}", pc),
        }
    }
}

/// __OSReboot (0x8009faf8, 832 bytes)
#[inline(never)]
pub fn f_8009faf8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009faf8) { return; }
        if c.prof { crate::prof::enter(c, 0x8009faf8); f_8009faf8_b(c, 0x8009faf8); crate::prof::exit(c, 0x8009faf8); return; }
    }
    f_8009faf8_b(c, 0x8009faf8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009faf8_at(c: &mut Cpu, pc: u32) { f_8009faf8_b(c, pc) }
#[inline(always)]
fn f_8009faf8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009faf8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009faf8, pc); }
        match pc {
            0x8009faf8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffc80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x368); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x80400000;
                c.r[30] = c.r[3].wrapping_add(0xffffb820);
                c.lr = 0x8009fb14; f_8009df3c(c);
                pc = 0x8009fb14;
            }
            0x8009fb14 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9f08));
                c.r[4] = 0x81300000;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f0c));
                c.r[27] = 0x0;
                c.r[31] = 0x81800000;
                c.r[6] = 0x1;
                c.stw(c.r[31].wrapping_add(0xfffffffc), c.r[27]);
                c.r[26] = 0x80000000;
                c.stw(c.r[31].wrapping_add(0xfffffff8), c.r[27]);
                c.r[3] = c.r[1].wrapping_add(0xa0);
                c.stb(c.r[26].wrapping_add(0x30e2), c.r[6]);
                c.stw(c.r[4].wrapping_add(0xffffdff0), c.r[5]);
                c.stw(c.r[4].wrapping_add(0xffffdfec), c.r[0]);
                c.lr = 0x8009fb4c; f_8009bd60(c);
                pc = 0x8009fb4c;
            }
            0x8009fb4c => {
                c.r[3] = c.r[1].wrapping_add(0xa0);
                c.lr = 0x8009fb54; f_8009bb68(c);
                pc = 0x8009fb54;
            }
            0x8009fb54 => {
                c.lr = 0x8009fb58; f_800a5624(c);
                pc = 0x8009fb58;
            }
            0x8009fb58 => {
                c.r[3] = 0x1;
                c.lr = 0x8009fb60; f_800a7820(c);
                pc = 0x8009fb60;
            }
            0x8009fb60 => {
                c.lr = 0x8009fb64; f_800a7830(c);
                pc = 0x8009fb64;
            }
            0x8009fb64 => {
                c.r[3] = 0x800a0000;
                c.stw(c.r[13].wrapping_add(0xffff9f10), c.r[27]);
                c.r[3] = c.r[3].wrapping_add(0xfffffaec);
                c.lr = 0x8009fb74; f_800a7ccc(c);
                pc = 0x8009fb74;
            }
            0x8009fb74 => {
                c.r[3] = 0xffffffe0;
                c.lr = 0x8009fb7c; f_8009e304(c);
                pc = 0x8009fb7c;
            }
            0x8009fb7c => {
                c.r[3] = 0x400;
                c.lr = 0x8009fb84; f_8009e38c(c);
                pc = 0x8009fb84;
            }
            0x8009fb84 => {
                c.lr = 0x8009fb88; f_8009df50(c);
                pc = 0x8009fb88;
            }
            0x8009fb88 => {
                c.lr = 0x8009fb8c; crate::stubs::f_800a2778(c);
                pc = 0x8009fb8c;
            }
            0x8009fb8c => {
                c.r[29] = c.r[4] | c.r[4];
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8009fb98; continue;
                pc = 0x8009fb98;
            }
            0x8009fb98 => {
                c.r[0] = c.lwz(c.r[26].wrapping_add(0xf8));
                c.r[26] = c.r[27] ^ 0x80000000;
                c.r[27] = c.r[0].rotate_left(30) & 0x3fffffff;
                pc = 0x8009fba8; continue;
                pc = 0x8009fba8;
            }
            0x8009fba8 => {
                pc = 0x8009fbe8; continue;
                pc = 0x8009fbac;
            }
            0x8009fbac => {
                c.lr = 0x8009fbb0; f_800a7bd4(c);
                pc = 0x8009fbb0;
            }
            0x8009fbb0 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009fbe0; continue; }
                pc = 0x8009fbb8;
            }
            0x8009fbb8 => {
                c.lr = 0x8009fbbc; crate::stubs::f_800a2778(c);
                pc = 0x8009fbbc;
            }
            0x8009fbbc => {
                c.r[4] = c.addc(!c.r[29], c.r[4], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[28], c.r[3], ci); }
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[27], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[26], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[26], c.r[26], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009fbe8; continue; }
                pc = 0x8009fbe0;
            }
            0x8009fbe0 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xfffffffc));
                c.lr = 0x8009fbe8; f_8009ffc0(c);
                pc = 0x8009fbe8;
            }
            0x8009fbe8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f10));
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8009fbac; continue; }
                pc = 0x8009fbf4;
            }
            0x8009fbf4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9e8c));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8009fcc4; continue; }
                pc = 0x8009fc00;
            }
            0x8009fc00 => {
                c.lr = 0x8009fc04; f_800a7bcc(c);
                pc = 0x8009fc04;
            }
            0x8009fc04 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009fc18; continue; }
                pc = 0x8009fc10;
            }
            0x8009fc10 => {
                c.r[0] = 0x1;
                pc = 0x8009fc1c; continue;
                pc = 0x8009fc18;
            }
            0x8009fc18 => {
                c.r[0] = 0x0;
                pc = 0x8009fc1c;
            }
            0x8009fc1c => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8009fcc4; continue; }
                pc = 0x8009fc24;
            }
            0x8009fc24 => {
                c.r[3] = 0x0;
                c.lr = 0x8009fc2c; f_800ac3e8(c);
                pc = 0x8009fc2c;
            }
            0x8009fc2c => {
                c.r[3] = 0x0;
                c.lr = 0x8009fc34; f_800ac414(c);
                pc = 0x8009fc34;
            }
            0x8009fc34 => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                c.r[4] = 0x0;
                c.lr = 0x8009fc40; f_800a7558(c);
                pc = 0x8009fc40;
            }
            0x8009fc40 => {
                c.lr = 0x8009fc44; crate::stubs::f_800a2778(c);
                pc = 0x8009fc44;
            }
            0x8009fc44 => {
                c.r[31] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x8009fc50; continue;
                pc = 0x8009fc50;
            }
            0x8009fc50 => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[0] = 0x0;
                c.r[27] = c.r[0] ^ 0x80000000;
                c.r[28] = c.r[3].rotate_left(30) & 0x3fffffff;
                c.r[26] = 0x81800000;
                pc = 0x8009fc6c; continue;
                pc = 0x8009fc6c;
            }
            0x8009fc6c => {
                pc = 0x8009fcac; continue;
                pc = 0x8009fc70;
            }
            0x8009fc70 => {
                c.lr = 0x8009fc74; f_800a7bd4(c);
                pc = 0x8009fc74;
            }
            0x8009fc74 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009fca4; continue; }
                pc = 0x8009fc7c;
            }
            0x8009fc7c => {
                c.lr = 0x8009fc80; crate::stubs::f_800a2778(c);
                pc = 0x8009fc80;
            }
            0x8009fc80 => {
                c.r[4] = c.addc(!c.r[31], c.r[4], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[29], c.r[3], ci); }
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[28], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[27], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[27], c.r[27], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009fcac; continue; }
                pc = 0x8009fca4;
            }
            0x8009fca4 => {
                c.r[3] = c.lwz(c.r[26].wrapping_add(0xfffffffc));
                c.lr = 0x8009fcac; f_8009ffc0(c);
                pc = 0x8009fcac;
            }
            0x8009fcac => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                c.lr = 0x8009fcb4; f_800a7728(c);
                pc = 0x8009fcb4;
            }
            0x8009fcb4 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009fc70; continue; }
                pc = 0x8009fcbc;
            }
            0x8009fcbc => {
                c.r[3] = 0x0;
                c.lr = 0x8009fcc4; f_800ac128(c);
                pc = 0x8009fcc4;
            }
            0x8009fcc4 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[1].wrapping_add(0x40);
                c.r[5] = 0x20;
                c.r[6] = 0x2440;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x8009fce0; f_800a720c(c);
                pc = 0x8009fce0;
            }
            0x8009fce0 => {
                c.lr = 0x8009fce4; crate::stubs::f_800a2778(c);
                pc = 0x8009fce4;
            }
            0x8009fce4 => {
                c.r[26] = c.r[4] | c.r[4];
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8009fcf0; continue;
                pc = 0x8009fcf0;
            }
            0x8009fcf0 => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[0] = 0x0;
                c.r[28] = c.r[0] ^ 0x80000000;
                c.r[29] = c.r[3].rotate_left(30) & 0x3fffffff;
                c.r[27] = 0x81800000;
                pc = 0x8009fd0c; continue;
                pc = 0x8009fd0c;
            }
            0x8009fd0c => {
                pc = 0x8009fd4c; continue;
                pc = 0x8009fd10;
            }
            0x8009fd10 => {
                c.lr = 0x8009fd14; f_800a7bd4(c);
                pc = 0x8009fd14;
            }
            0x8009fd14 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009fd44; continue; }
                pc = 0x8009fd1c;
            }
            0x8009fd1c => {
                c.lr = 0x8009fd20; crate::stubs::f_800a2778(c);
                pc = 0x8009fd20;
            }
            0x8009fd20 => {
                c.r[4] = c.addc(!c.r[26], c.r[4], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[31], c.r[3], ci); }
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[29], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[28], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[28], c.r[28], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009fd4c; continue; }
                pc = 0x8009fd44;
            }
            0x8009fd44 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0xfffffffc));
                c.lr = 0x8009fd4c; f_8009ffc0(c);
                pc = 0x8009fd4c;
            }
            0x8009fd4c => {
                c.r[3] = c.r[1].wrapping_add(0x40);
                c.lr = 0x8009fd54; f_800a7728(c);
                pc = 0x8009fd54;
            }
            0x8009fd54 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009fd10; continue; }
                pc = 0x8009fd5c;
            }
            0x8009fd5c => {
                c.r[5] = c.lwz(c.r[30].wrapping_add(0x18));
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.r[6] = c.lwz(c.r[30].wrapping_add(0x14));
                c.r[4] = 0x81300000;
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[6] = c.r[6].wrapping_add(0x20);
                c.r[26] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[6].wrapping_add(0x2440);
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x8009fd8c; f_800a720c(c);
                pc = 0x8009fd8c;
            }
            0x8009fd8c => {
                c.lr = 0x8009fd90; crate::stubs::f_800a2778(c);
                pc = 0x8009fd90;
            }
            0x8009fd90 => {
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8009fd9c; continue;
                pc = 0x8009fd9c;
            }
            0x8009fd9c => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[0] = 0x0;
                c.r[31] = c.r[0] ^ 0x80000000;
                c.r[29] = c.r[3].rotate_left(30) & 0x3fffffff;
                c.r[30] = 0x81800000;
                pc = 0x8009fdb8; continue;
                pc = 0x8009fdb8;
            }
            0x8009fdb8 => {
                pc = 0x8009fdf8; continue;
                pc = 0x8009fdbc;
            }
            0x8009fdbc => {
                c.lr = 0x8009fdc0; f_800a7bd4(c);
                pc = 0x8009fdc0;
            }
            0x8009fdc0 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009fdf0; continue; }
                pc = 0x8009fdc8;
            }
            0x8009fdc8 => {
                c.lr = 0x8009fdcc; crate::stubs::f_800a2778(c);
                pc = 0x8009fdcc;
            }
            0x8009fdcc => {
                c.r[4] = c.addc(!c.r[27], c.r[4], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[28], c.r[3], ci); }
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[29], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[31], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[31], c.r[31], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8009fdf8; continue; }
                pc = 0x8009fdf0;
            }
            0x8009fdf0 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xfffffffc));
                c.lr = 0x8009fdf8; f_8009ffc0(c);
                pc = 0x8009fdf8;
            }
            0x8009fdf8 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x8009fe00; f_800a7728(c);
                pc = 0x8009fe00;
            }
            0x8009fe00 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8009fdbc; continue; }
                pc = 0x8009fe08;
            }
            0x8009fe08 => {
                c.r[3] = 0x81300000;
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x8009fe14; f_8009b3b4(c);
                pc = 0x8009fe14;
            }
            0x8009fe14 => {
                c.lr = 0x8009fe18; f_8009df3c(c);
                pc = 0x8009fe18;
            }
            0x8009fe18 => {
                c.lr = 0x8009fe1c; f_8009b3e8(c);
                pc = 0x8009fe1c;
            }
            0x8009fe1c => {
                c.r[3] = 0x81300000;
                c.lr = 0x8009fe24; f_8009fadc(c);
                pc = 0x8009fe24;
            }
            0x8009fe24 => {
                { let mut ea = c.r[1].wrapping_add(0x368); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x384));
                c.r[1] = c.r[1].wrapping_add(0x380);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009faf8: bad pc {:#x}", pc),
        }
    }
}

/// OSRegisterResetFunction (0x8009fe38, 132 bytes)
#[inline(never)]
pub fn f_8009fe38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009fe38) { return; }
        if c.prof { crate::prof::enter(c, 0x8009fe38); f_8009fe38_b(c, 0x8009fe38); crate::prof::exit(c, 0x8009fe38); return; }
    }
    f_8009fe38_b(c, 0x8009fe38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009fe38_at(c: &mut Cpu, pc: u32) { f_8009fe38_b(c, pc) }
#[inline(always)]
fn f_8009fe38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009fe38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009fe38, pc); }
        match pc {
            0x8009fe38 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9f18));
                pc = 0x8009fe44; continue;
                pc = 0x8009fe40;
            }
            0x8009fe40 => {
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x8));
                pc = 0x8009fe44;
            }
            0x8009fe44 => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x8009fe5c; continue; }
                pc = 0x8009fe4c;
            }
            0x8009fe4c => {
                c.r[4] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(1) { pc = 0x8009fe40; continue; }
                pc = 0x8009fe5c;
            }
            0x8009fe5c => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8009fe94; continue; }
                pc = 0x8009fe64;
            }
            0x8009fe64 => {
                c.r[5] = c.r[13].wrapping_add(0xffff9f18);
                { let ea = c.r[5].wrapping_add(0x4); c.r[4] = c.lwz(ea); c.r[5] = ea; }
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x8009fe7c; continue; }
                pc = 0x8009fe74;
            }
            0x8009fe74 => {
                c.stw(c.r[13].wrapping_add(0xffff9f18), c.r[3]);
                pc = 0x8009fe80; continue;
                pc = 0x8009fe7c;
            }
            0x8009fe7c => {
                c.stw(c.r[4].wrapping_add(0x8), c.r[3]);
                pc = 0x8009fe80;
            }
            0x8009fe80 => {
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[5], c.r[3]);
                if true { return; }
                pc = 0x8009fe94;
            }
            0x8009fe94 => {
                c.stw(c.r[3].wrapping_add(0x8), c.r[5]);
                c.r[4] = c.lwz(c.r[5].wrapping_add(0xc));
                c.stw(c.r[5].wrapping_add(0xc), c.r[3]);
                c.cmpu(0, c.r[4], 0x0);
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if !c.crbit(2) { pc = 0x8009feb4; continue; }
                pc = 0x8009feac;
            }
            0x8009feac => {
                c.stw(c.r[13].wrapping_add(0xffff9f18), c.r[3]);
                if true { return; }
                pc = 0x8009feb4;
            }
            0x8009feb4 => {
                c.stw(c.r[4].wrapping_add(0x8), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009fe38: bad pc {:#x}", pc),
        }
    }
}

/// fn_8009FEBC (0x8009febc, 148 bytes)
#[inline(never)]
pub fn f_8009febc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009febc) { return; }
        if c.prof { crate::prof::enter(c, 0x8009febc); f_8009febc_b(c, 0x8009febc); crate::prof::exit(c, 0x8009febc); return; }
    }
    f_8009febc_b(c, 0x8009febc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009febc_at(c: &mut Cpu, pc: u32) { f_8009febc_b(c, pc) }
#[inline(always)]
fn f_8009febc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009febc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009febc, pc); }
        match pc {
            0x8009febc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffff9f18));
                pc = 0x8009ff04; continue;
                pc = 0x8009fee4;
            }
            0x8009fee4 => {
                c.r[12] = c.lwz(c.r[31]);
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x8009fef4; (c.dispatch)(c, t); }
                pc = 0x8009fef4;
            }
            0x8009fef4 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[31] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[30] = c.r[30] | c.r[0];
                pc = 0x8009ff04;
            }
            0x8009ff04 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8009ff14; continue; }
                pc = 0x8009ff0c;
            }
            0x8009ff0c => {
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8009fee4; continue; }
                pc = 0x8009ff14;
            }
            0x8009ff14 => {
                c.lr = 0x8009ff18; f_800a0d00(c);
                pc = 0x8009ff18;
            }
            0x8009ff18 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[30] = c.r[30] | c.r[0];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x8009ff30; continue; }
                pc = 0x8009ff28;
            }
            0x8009ff28 => {
                c.r[3] = 0x0;
                pc = 0x8009ff34; continue;
                pc = 0x8009ff30;
            }
            0x8009ff30 => {
                c.r[3] = 0x1;
                pc = 0x8009ff34;
            }
            0x8009ff34 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009febc: bad pc {:#x}", pc),
        }
    }
}

/// Reset (0x8009ff50, 112 bytes)
#[inline(never)]
pub fn f_8009ff50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ff50) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ff50); f_8009ff50_b(c, 0x8009ff50); crate::prof::exit(c, 0x8009ff50); return; }
    }
    f_8009ff50_b(c, 0x8009ff50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ff50_at(c: &mut Cpu, pc: u32) { f_8009ff50_b(c, pc) }
#[inline(always)]
fn f_8009ff50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ff50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ff50, pc); }
        match pc {
            0x8009ff50 => {
                pc = 0x8009ff70; continue;
                pc = 0x8009ff54;
            }
            0x8009ff54 => {
                c.r[8] = c.mfspr(1008);
                c.r[8] = c.r[8] | 0x8;
                c.mtspr(1008, c.r[8]);
                // isync
                // sync
                pc = 0x8009ff74; continue;
                pc = 0x8009ff70;
            }
            0x8009ff70 => {
                pc = 0x8009ff90; continue;
                pc = 0x8009ff74;
            }
            0x8009ff74 => {
                c.r[5] = 0; // mftb
                pc = 0x8009ff78;
            }
            0x8009ff78 => {
                c.r[6] = 0; // mftb
                c.r[7] = c.r[6].wrapping_sub(c.r[5]);
                c.cmpu(0, c.r[7], 0x1124);
                if c.crbit(0) { pc = 0x8009ff78; continue; }
                pc = 0x8009ff88;
            }
            0x8009ff88 => {
                pc = 0x8009ff94; continue;
                pc = 0x8009ff90;
            }
            0x8009ff90 => {
                pc = 0x8009ffb0; continue;
                pc = 0x8009ff94;
            }
            0x8009ff94 => {
                c.r[8] = 0xcc000000;
                c.r[8] = c.r[8] | 0x3000;
                c.r[4] = 0x3;
                c.stw(c.r[8].wrapping_add(0x24), c.r[4]);
                c.stw(c.r[8].wrapping_add(0x24), c.r[3]);
                pc = 0x8009ffb4; continue;
                pc = 0x8009ffb0;
            }
            0x8009ffb0 => {
                pc = 0x8009ffbc; continue;
                pc = 0x8009ffb4;
            }
            0x8009ffb4 => {
                pc = 0x8009ffb4; continue;
                pc = 0x8009ffbc;
            }
            0x8009ffbc => {
                pc = 0x8009ff54; continue;
                return;
            }
            _ => panic!("f_8009ff50: bad pc {:#x}", pc),
        }
    }
}

/// FUN_8009ff54 (0x8009ff54, 28 bytes)
#[inline(never)]
pub fn f_8009ff54(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ff54) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ff54); f_8009ff54_b(c, 0x8009ff54); crate::prof::exit(c, 0x8009ff54); return; }
    }
    f_8009ff54_b(c, 0x8009ff54)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ff54_at(c: &mut Cpu, pc: u32) { f_8009ff54_b(c, pc) }
#[inline(always)]
fn f_8009ff54_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ff54;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ff54, pc); }
        match pc {
            0x8009ff54 => {
                c.r[8] = c.mfspr(1008);
                c.r[8] = c.r[8] | 0x8;
                c.mtspr(1008, c.r[8]);
                // isync
                // sync
                f_8009ff74(c); return;
                return;
            }
            _ => panic!("f_8009ff54: bad pc {:#x}", pc),
        }
    }
}

/// thunk_FUN_8009ff54 (0x8009ff70, 4 bytes)
#[inline(never)]
pub fn f_8009ff70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ff70) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ff70); f_8009ff70_b(c, 0x8009ff70); crate::prof::exit(c, 0x8009ff70); return; }
    }
    f_8009ff70_b(c, 0x8009ff70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ff70_at(c: &mut Cpu, pc: u32) { f_8009ff70_b(c, pc) }
#[inline(always)]
fn f_8009ff70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ff70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ff70, pc); }
        match pc {
            0x8009ff70 => {
                f_8009ff90(c); return;
                return;
            }
            _ => panic!("f_8009ff70: bad pc {:#x}", pc),
        }
    }
}

/// FUN_8009ff74 (0x8009ff74, 28 bytes)
#[inline(never)]
pub fn f_8009ff74(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ff74) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ff74); f_8009ff74_b(c, 0x8009ff74); crate::prof::exit(c, 0x8009ff74); return; }
    }
    f_8009ff74_b(c, 0x8009ff74)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ff74_at(c: &mut Cpu, pc: u32) { f_8009ff74_b(c, pc) }
#[inline(always)]
fn f_8009ff74_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ff74;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ff74, pc); }
        match pc {
            0x8009ff74 => {
                c.r[5] = 0; // mftb
                pc = 0x8009ff78;
            }
            0x8009ff78 => {
                c.r[6] = 0; // mftb
                c.r[7] = c.r[6].wrapping_sub(c.r[5]);
                c.cmpu(0, c.r[7], 0x1124);
                if c.crbit(0) { pc = 0x8009ff78; continue; }
                pc = 0x8009ff88;
            }
            0x8009ff88 => {
                f_8009ff94(c); return;
                return;
            }
            _ => panic!("f_8009ff74: bad pc {:#x}", pc),
        }
    }
}

/// thunk_FUN_8009ff54 (0x8009ff90, 4 bytes)
#[inline(never)]
pub fn f_8009ff90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ff90) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ff90); f_8009ff90_b(c, 0x8009ff90); crate::prof::exit(c, 0x8009ff90); return; }
    }
    f_8009ff90_b(c, 0x8009ff90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ff90_at(c: &mut Cpu, pc: u32) { f_8009ff90_b(c, pc) }
#[inline(always)]
fn f_8009ff90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ff90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ff90, pc); }
        match pc {
            0x8009ff90 => {
                f_8009ffb0(c); return;
                return;
            }
            _ => panic!("f_8009ff90: bad pc {:#x}", pc),
        }
    }
}

/// FUN_8009ff94 (0x8009ff94, 28 bytes)
#[inline(never)]
pub fn f_8009ff94(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ff94) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ff94); f_8009ff94_b(c, 0x8009ff94); crate::prof::exit(c, 0x8009ff94); return; }
    }
    f_8009ff94_b(c, 0x8009ff94)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ff94_at(c: &mut Cpu, pc: u32) { f_8009ff94_b(c, pc) }
#[inline(always)]
fn f_8009ff94_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ff94;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ff94, pc); }
        match pc {
            0x8009ff94 => {
                c.r[8] = 0xcc000000;
                c.r[8] = c.r[8] | 0x3000;
                c.r[4] = 0x3;
                c.stw(c.r[8].wrapping_add(0x24), c.r[4]);
                c.stw(c.r[8].wrapping_add(0x24), c.r[3]);
                f_8009ffb4(c); return;
                return;
            }
            _ => panic!("f_8009ff94: bad pc {:#x}", pc),
        }
    }
}

/// thunk_FUN_8009ff54 (0x8009ffb0, 4 bytes)
#[inline(never)]
pub fn f_8009ffb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ffb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ffb0); f_8009ffb0_b(c, 0x8009ffb0); crate::prof::exit(c, 0x8009ffb0); return; }
    }
    f_8009ffb0_b(c, 0x8009ffb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ffb0_at(c: &mut Cpu, pc: u32) { f_8009ffb0_b(c, pc) }
#[inline(always)]
fn f_8009ffb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ffb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ffb0, pc); }
        match pc {
            0x8009ffb0 => {
                f_8009ffbc(c); return;
                return;
            }
            _ => panic!("f_8009ffb0: bad pc {:#x}", pc),
        }
    }
}

/// FUN_8009ffb4 (0x8009ffb4, 8 bytes)
#[inline(never)]
pub fn f_8009ffb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ffb4) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ffb4); f_8009ffb4_b(c, 0x8009ffb4); crate::prof::exit(c, 0x8009ffb4); return; }
    }
    f_8009ffb4_b(c, 0x8009ffb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ffb4_at(c: &mut Cpu, pc: u32) { f_8009ffb4_b(c, pc) }
#[inline(always)]
fn f_8009ffb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ffb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ffb4, pc); }
        match pc {
            0x8009ffb4 => {
                pc = 0x8009ffb4; continue;
                return;
            }
            _ => panic!("f_8009ffb4: bad pc {:#x}", pc),
        }
    }
}

/// thunk_FUN_8009ff54 (0x8009ffbc, 4 bytes)
#[inline(never)]
pub fn f_8009ffbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ffbc) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ffbc); f_8009ffbc_b(c, 0x8009ffbc); crate::prof::exit(c, 0x8009ffbc); return; }
    }
    f_8009ffbc_b(c, 0x8009ffbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ffbc_at(c: &mut Cpu, pc: u32) { f_8009ffbc_b(c, pc) }
#[inline(always)]
fn f_8009ffbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ffbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ffbc, pc); }
        match pc {
            0x8009ffbc => {
                f_8009ff54(c); return;
                return;
            }
            _ => panic!("f_8009ffbc: bad pc {:#x}", pc),
        }
    }
}

/// __OSDoHotReset (0x8009ffc0, 72 bytes)
#[inline(never)]
pub fn f_8009ffc0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8009ffc0) { return; }
        if c.prof { crate::prof::enter(c, 0x8009ffc0); f_8009ffc0_b(c, 0x8009ffc0); crate::prof::exit(c, 0x8009ffc0); return; }
    }
    f_8009ffc0_b(c, 0x8009ffc0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8009ffc0_at(c: &mut Cpu, pc: u32) { f_8009ffc0_b(c, pc) }
#[inline(always)]
fn f_8009ffc0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8009ffc0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8009ffc0, pc); }
        match pc {
            0x8009ffc0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8009ffd8; f_8009df3c(c);
                pc = 0x8009ffd8;
            }
            0x8009ffd8 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x2000);
                c.r[0] = 0x0;
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                c.lr = 0x8009ffec; f_8009b3e8(c);
                pc = 0x8009ffec;
            }
            0x8009ffec => {
                c.r[3] = c.r[31].rotate_left(3) & 0xfffffff8;
                c.lr = 0x8009fff4; f_8009ff50(c);
                pc = 0x8009fff4;
            }
            0x8009fff4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8009ffc0: bad pc {:#x}", pc),
        }
    }
}

/// OSResetSystem (0x800a0008, 648 bytes)
#[inline(never)]
pub fn f_800a0008(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0008) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0008); f_800a0008_b(c, 0x800a0008); crate::prof::exit(c, 0x800a0008); return; }
    }
    f_800a0008_b(c, 0x800a0008)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0008_at(c: &mut Cpu, pc: u32) { f_800a0008_b(c, pc) }
#[inline(always)]
fn f_800a0008_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0008;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0008, pc); }
        match pc {
            0x800a0008 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x28); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.lr = 0x800a0028; f_800a1404(c);
                pc = 0x800a0028;
            }
            0x800a0028 => {
                c.lr = 0x800a002c; f_8009b1b8(c);
                pc = 0x800a002c;
            }
            0x800a002c => {
                c.cmps(0, c.r[26], 0x2);
                if !c.crbit(2) { pc = 0x800a0040; continue; }
                pc = 0x800a0034;
            }
            0x800a0034 => {
                c.r[3] = 0x1;
                c.lr = 0x800a003c; f_800abfb0(c);
                pc = 0x800a003c;
            }
            0x800a003c => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800a0040;
            }
            0x800a0040 => {
                pc = 0x800a0044; continue;
                pc = 0x800a0044;
            }
            0x800a0044 => {
                pc = 0x800a0048; continue;
                pc = 0x800a0048;
            }
            0x800a0048 => {
                c.r[27] = c.lwz(c.r[13].wrapping_add(0xffff9f18));
                c.r[28] = 0x0;
                pc = 0x800a0054; continue;
                pc = 0x800a0054;
            }
            0x800a0054 => {
                pc = 0x800a0058; continue;
                pc = 0x800a0058;
            }
            0x800a0058 => {
                pc = 0x800a007c; continue;
                pc = 0x800a005c;
            }
            0x800a005c => {
                c.r[3] = 0x0;
                c.r[12] = c.lwz(c.r[27]);
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a006c; (c.dispatch)(c, t); }
                pc = 0x800a006c;
            }
            0x800a006c => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[27] = c.lwz(c.r[27].wrapping_add(0x8));
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[28] = c.r[28] | c.r[0];
                pc = 0x800a007c;
            }
            0x800a007c => {
                c.cmpu(0, c.r[27], 0x0);
                if c.crbit(2) { pc = 0x800a008c; continue; }
                pc = 0x800a0084;
            }
            0x800a0084 => {
                c.cmps(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x800a005c; continue; }
                pc = 0x800a008c;
            }
            0x800a008c => {
                c.lr = 0x800a0090; f_800a0d00(c);
                pc = 0x800a0090;
            }
            0x800a0090 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[28] = c.r[28] | c.r[0];
                c.cmps(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x800a00ac; continue; }
                pc = 0x800a00a4;
            }
            0x800a00a4 => {
                c.r[0] = 0x0;
                pc = 0x800a00b0; continue;
                pc = 0x800a00ac;
            }
            0x800a00ac => {
                c.r[0] = 0x1;
                pc = 0x800a00b0;
            }
            0x800a00b0 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a0048; continue; }
                pc = 0x800a00b8;
            }
            0x800a00b8 => {
                c.cmps(0, c.r[26], 0x1);
                if !c.crbit(2) { pc = 0x800a00f4; continue; }
                pc = 0x800a00c0;
            }
            0x800a00c0 => {
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x800a00f4; continue; }
                pc = 0x800a00c8;
            }
            0x800a00c8 => {
                c.lr = 0x800a00cc; f_800a08f8(c);
                pc = 0x800a00cc;
            }
            0x800a00cc => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x13));
                c.r[0] = c.r[0] | 0x40;
                c.stb(c.r[3].wrapping_add(0x13), c.r[0]);
                c.r[3] = 0x1;
                c.lr = 0x800a00e0; f_800a0cb8(c);
                pc = 0x800a00e0;
            }
            0x800a00e0 => {
                pc = 0x800a00e4; continue;
                pc = 0x800a00e4;
            }
            0x800a00e4 => {
                pc = 0x800a00e8; continue;
                pc = 0x800a00e8;
            }
            0x800a00e8 => {
                c.lr = 0x800a00ec; f_800a0d00(c);
                pc = 0x800a00ec;
            }
            0x800a00ec => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a00e8; continue; }
                pc = 0x800a00f4;
            }
            0x800a00f4 => {
                c.lr = 0x800a00f8; f_8009df3c(c);
                pc = 0x800a00f8;
            }
            0x800a00f8 => {
                c.r[28] = c.lwz(c.r[13].wrapping_add(0xffff9f18));
                c.r[27] = 0x0;
                pc = 0x800a0104; continue;
                pc = 0x800a0104;
            }
            0x800a0104 => {
                pc = 0x800a0108; continue;
                pc = 0x800a0108;
            }
            0x800a0108 => {
                pc = 0x800a012c; continue;
                pc = 0x800a010c;
            }
            0x800a010c => {
                c.r[3] = 0x1;
                c.r[12] = c.lwz(c.r[28]);
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a011c; (c.dispatch)(c, t); }
                pc = 0x800a011c;
            }
            0x800a011c => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[28] = c.lwz(c.r[28].wrapping_add(0x8));
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[27] = c.r[27] | c.r[0];
                pc = 0x800a012c;
            }
            0x800a012c => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x800a013c; continue; }
                pc = 0x800a0134;
            }
            0x800a0134 => {
                c.cmps(0, c.r[27], 0x0);
                if c.crbit(2) { pc = 0x800a010c; continue; }
                pc = 0x800a013c;
            }
            0x800a013c => {
                c.lr = 0x800a0140; f_800a0d00(c);
                pc = 0x800a0140;
            }
            0x800a0140 => {
                c.lr = 0x800a0144; f_8009b510(c);
                pc = 0x800a0144;
            }
            0x800a0144 => {
                c.cmps(0, c.r[26], 0x1);
                if !c.crbit(2) { pc = 0x800a0170; continue; }
                pc = 0x800a014c;
            }
            0x800a014c => {
                c.lr = 0x800a0150; f_8009df3c(c);
                pc = 0x800a0150;
            }
            0x800a0150 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x2000);
                c.r[0] = 0x0;
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                c.lr = 0x800a0164; f_8009b3e8(c);
                pc = 0x800a0164;
            }
            0x800a0164 => {
                c.r[3] = c.r[29].rotate_left(3) & 0xfffffff8;
                c.lr = 0x800a016c; f_8009ff50(c);
                pc = 0x800a016c;
            }
            0x800a016c => {
                pc = 0x800a01cc; continue;
                pc = 0x800a0170;
            }
            0x800a0170 => {
                c.cmps(0, c.r[26], 0x0);
                if !c.crbit(2) { pc = 0x800a01cc; continue; }
                pc = 0x800a0178;
            }
            0x800a0178 => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xdc));
                pc = 0x800a0184; continue;
                pc = 0x800a0184;
            }
            0x800a0184 => {
                pc = 0x800a0188; continue;
                pc = 0x800a0188;
            }
            0x800a0188 => {
                pc = 0x800a01b4; continue;
                pc = 0x800a018c;
            }
            0x800a018c => {
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x2c8));
                c.r[27] = c.lwz(c.r[3].wrapping_add(0x2fc));
                c.cmps(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x800a01ac; continue; }
                pc = 0x800a019c;
            }
            0x800a019c => {
                if !c.crbit(0) { pc = 0x800a01b0; continue; }
                pc = 0x800a01a0;
            }
            0x800a01a0 => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800a01ac; continue; }
                pc = 0x800a01a8;
            }
            0x800a01a8 => {
                pc = 0x800a01b0; continue;
                pc = 0x800a01ac;
            }
            0x800a01ac => {
                c.lr = 0x800a01b0; f_800a1c98(c);
                pc = 0x800a01b0;
            }
            0x800a01b0 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x800a01b4;
            }
            0x800a01b4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a018c; continue; }
                pc = 0x800a01bc;
            }
            0x800a01bc => {
                c.lr = 0x800a01c0; f_800a1444(c);
                pc = 0x800a01c0;
            }
            0x800a01c0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x800a01cc; f_8009faf8(c);
                pc = 0x800a01cc;
            }
            0x800a01cc => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xdc));
                pc = 0x800a01d8; continue;
                pc = 0x800a01d8;
            }
            0x800a01d8 => {
                pc = 0x800a01dc; continue;
                pc = 0x800a01dc;
            }
            0x800a01dc => {
                pc = 0x800a0208; continue;
                pc = 0x800a01e0;
            }
            0x800a01e0 => {
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x2c8));
                c.r[27] = c.lwz(c.r[3].wrapping_add(0x2fc));
                c.cmps(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x800a0200; continue; }
                pc = 0x800a01f0;
            }
            0x800a01f0 => {
                if !c.crbit(0) { pc = 0x800a0204; continue; }
                pc = 0x800a01f4;
            }
            0x800a01f4 => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800a0200; continue; }
                pc = 0x800a01fc;
            }
            0x800a01fc => {
                pc = 0x800a0204; continue;
                pc = 0x800a0200;
            }
            0x800a0200 => {
                c.lr = 0x800a0204; f_800a1c98(c);
                pc = 0x800a0204;
            }
            0x800a0204 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x800a0208;
            }
            0x800a0208 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a01e0; continue; }
                pc = 0x800a0210;
            }
            0x800a0210 => {
                c.r[29] = 0x80000000;
                c.r[3] = c.r[29].wrapping_add(0x40);
                c.r[4] = 0x0;
                c.r[5] = 0x8c;
                c.lr = 0x800a0224; f_8000540c(c);
                pc = 0x800a0224;
            }
            0x800a0224 => {
                c.r[3] = c.r[29].wrapping_add(0xd4);
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.lr = 0x800a0234; f_8000540c(c);
                pc = 0x800a0234;
            }
            0x800a0234 => {
                c.r[3] = c.r[29].wrapping_add(0xf4);
                c.r[4] = 0x0;
                c.r[5] = 0x4;
                c.lr = 0x800a0244; f_8000540c(c);
                pc = 0x800a0244;
            }
            0x800a0244 => {
                c.r[3] = c.r[29].wrapping_add(0x3000);
                c.r[4] = 0x0;
                c.r[5] = 0xc0;
                c.lr = 0x800a0254; f_8000540c(c);
                pc = 0x800a0254;
            }
            0x800a0254 => {
                c.r[3] = c.r[29].wrapping_add(0x30c8);
                c.r[4] = 0x0;
                c.r[5] = 0xc;
                c.lr = 0x800a0264; f_8000540c(c);
                pc = 0x800a0264;
            }
            0x800a0264 => {
                c.r[3] = c.r[29].wrapping_add(0x30e2);
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.lr = 0x800a0274; f_8000540c(c);
                pc = 0x800a0274;
            }
            0x800a0274 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a027c; f_800abfb0(c);
                pc = 0x800a027c;
            }
            0x800a027c => {
                { let mut ea = c.r[1].wrapping_add(0x28); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a0008: bad pc {:#x}", pc),
        }
    }
}

/// OSGetResetCode (0x800a0290, 48 bytes)
#[inline(never)]
pub fn f_800a0290(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0290) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0290); f_800a0290_b(c, 0x800a0290); crate::prof::exit(c, 0x800a0290); return; }
    }
    f_800a0290_b(c, 0x800a0290)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0290_at(c: &mut Cpu, pc: u32) { f_800a0290_b(c, pc) }
#[inline(always)]
fn f_800a0290_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0290;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0290, pc); }
        match pc {
            0x800a0290 => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x30e2));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a02a8; continue; }
                pc = 0x800a02a0;
            }
            0x800a02a0 => {
                c.r[3] = 0x80000000;
                pc = 0x800a02bc; continue;
                pc = 0x800a02a8;
            }
            0x800a02a8 => {
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x3000);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x24));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffff8;
                c.r[3] = c.r[0].rotate_left(29) & 0x1fffffff;
                pc = 0x800a02bc;
            }
            0x800a02bc => {
                if true { return; }
                return;
            }
            _ => panic!("f_800a0290: bad pc {:#x}", pc),
        }
    }
}

/// __OSResetSWInterruptHandler (0x800a02c0, 244 bytes)
#[inline(never)]
pub fn f_800a02c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a02c0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a02c0); f_800a02c0_b(c, 0x800a02c0); crate::prof::exit(c, 0x800a02c0); return; }
    }
    f_800a02c0_b(c, 0x800a02c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a02c0_at(c: &mut Cpu, pc: u32) { f_800a02c0_b(c, pc) }
#[inline(always)]
fn f_800a02c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a02c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a02c0, pc); }
        match pc {
            0x800a02c0 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.lr = 0x800a02dc; f_800a2798(c);
                pc = 0x800a02dc;
            }
            0x800a02dc => {
                c.r[5] = 0x80000000;
                c.stw(c.r[13].wrapping_add(0xffff9f3c), c.r[4]);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xf8));
                c.r[4] = 0x431c0000;
                c.r[4] = c.r[4].wrapping_add(0xffffde83);
                c.stw(c.r[13].wrapping_add(0xffff9f38), c.r[3]);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = mulhwu(c.r[4], c.r[0]);
                c.r[0] = c.r[0].rotate_left(17) & 0x1ffff;
                c.r[0] = c.r[0].wrapping_mul(0x64);
                c.r[29] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.r[30] = 0x0;
                c.r[31] = 0xcc000000;
                pc = 0x800a0310;
            }
            0x800a0310 => {
                c.lr = 0x800a0314; f_800a2798(c);
                pc = 0x800a0314;
            }
            0x800a0314 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff9f3c));
                c.r[5] = c.r[30] ^ 0x80000000;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f38));
                c.r[4] = c.addc(!c.r[6], c.r[4], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[29], c.r[4], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[3], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[3], c.r[3], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x800a034c; continue; }
                pc = 0x800a0340;
            }
            0x800a0340 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x3000));
                c.r[0] = c.r[0].rotate_left(0) & 0x10000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a0310; continue; }
                pc = 0x800a034c;
            }
            0x800a034c => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x3000));
                c.r[0] = c.r[0].rotate_left(0) & 0x10000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a038c; continue; }
                pc = 0x800a035c;
            }
            0x800a035c => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f24), c.r[0]);
                c.r[3] = 0x200;
                c.stw(c.r[13].wrapping_add(0xffff9f28), c.r[0]);
                c.lr = 0x800a0370; f_8009e304(c);
                pc = 0x800a0370;
            }
            0x800a0370 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff9f20));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800a038c; continue; }
                pc = 0x800a037c;
            }
            0x800a037c => {
                c.r[0] = 0x0;
                c.mtspr(8, c.r[12]);
                c.stw(c.r[13].wrapping_add(0xffff9f20), c.r[0]);
                if true { let t = c.lr; c.lr = 0x800a038c; (c.dispatch)(c, t); }
                pc = 0x800a038c;
            }
            0x800a038c => {
                c.r[0] = 0x2;
                c.r[3] = 0xcc000000;
                c.stw(c.r[3].wrapping_add(0x3000), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a02c0: bad pc {:#x}", pc),
        }
    }
}

/// OSGetResetButtonState (0x800a03b4, 664 bytes)
#[inline(never)]
pub fn f_800a03b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a03b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a03b4); f_800a03b4_b(c, 0x800a03b4); crate::prof::exit(c, 0x800a03b4); return; }
    }
    f_800a03b4_b(c, 0x800a03b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a03b4_at(c: &mut Cpu, pc: u32) { f_800a03b4_b(c, pc) }
#[inline(always)]
fn f_800a03b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a03b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a03b4, pc); }
        match pc {
            0x800a03b4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[29]);
                c.lr = 0x800a03d0; f_8009df3c(c);
                pc = 0x800a03d0;
            }
            0x800a03d0 => {
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800a03d8; f_800a2798(c);
                pc = 0x800a03d8;
            }
            0x800a03d8 => {
                c.r[5] = 0xcc000000;
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x3000));
                c.r[0] = c.r[0].rotate_left(0) & 0x10000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a04c0; continue; }
                pc = 0x800a03e8;
            }
            0x800a03e8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f24));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a0430; continue; }
                pc = 0x800a03f4;
            }
            0x800a03f4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f30));
                c.r[6] = 0x0;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9f34));
                c.r[7] = 0x1;
                c.r[0] = c.r[0] ^ c.r[6];
                c.r[5] = c.r[5] ^ c.r[6];
                c.stw(c.r[13].wrapping_add(0xffff9f24), c.r[7]);
                c.r[0] = c.r[5] | c.r[0];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a041c; continue; }
                pc = 0x800a0418;
            }
            0x800a0418 => {
                pc = 0x800a0420; continue;
                pc = 0x800a041c;
            }
            0x800a041c => {
                c.r[7] = c.r[6] | c.r[6];
                pc = 0x800a0420;
            }
            0x800a0420 => {
                c.stw(c.r[13].wrapping_add(0xffff9f3c), c.r[4]);
                c.r[29] = c.r[7] | c.r[7];
                c.stw(c.r[13].wrapping_add(0xffff9f38), c.r[3]);
                pc = 0x800a0574; continue;
                pc = 0x800a0430;
            }
            0x800a0430 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f30));
                c.r[9] = 0x0;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9f34));
                c.r[10] = 0x1;
                c.r[0] = c.r[0] ^ c.r[9];
                c.r[5] = c.r[5] ^ c.r[9];
                c.r[0] = c.r[5] | c.r[0];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a04a4; continue; }
                pc = 0x800a0450;
            }
            0x800a0450 => {
                c.r[6] = 0x80000000;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9f3c));
                c.r[7] = c.lwz(c.r[6].wrapping_add(0xf8));
                c.r[6] = 0x431c0000;
                c.r[8] = c.r[6].wrapping_add(0xffffde83);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f38));
                c.r[6] = c.r[7].rotate_left(30) & 0x3fffffff;
                c.r[6] = mulhwu(c.r[8], c.r[6]);
                c.r[6] = c.r[6].rotate_left(17) & 0x1ffff;
                c.r[6] = c.r[6].wrapping_mul(0x64);
                c.r[7] = c.addc(!c.r[5], c.r[4], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[8] = c.r[6].rotate_left(29) & 0x1fffffff;
                c.r[5] = c.r[0] ^ 0x80000000;
                c.r[6] = c.r[9] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[7], c.r[8], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[6], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if !c.crbit(2) { pc = 0x800a04a4; continue; }
                pc = 0x800a04a0;
            }
            0x800a04a0 => {
                c.r[10] = c.r[9] | c.r[9];
                pc = 0x800a04a4;
            }
            0x800a04a4 => {
                c.cmps(0, c.r[10], 0x0);
                if c.crbit(2) { pc = 0x800a04b4; continue; }
                pc = 0x800a04ac;
            }
            0x800a04ac => {
                c.r[0] = 0x1;
                pc = 0x800a04b8; continue;
                pc = 0x800a04b4;
            }
            0x800a04b4 => {
                c.r[0] = 0x0;
                pc = 0x800a04b8;
            }
            0x800a04b8 => {
                c.r[29] = c.r[0] | c.r[0];
                pc = 0x800a0574; continue;
                pc = 0x800a04c0;
            }
            0x800a04c0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f24));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a04fc; continue; }
                pc = 0x800a04cc;
            }
            0x800a04cc => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9f28));
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9f24), c.r[0]);
                c.cmps(0, c.r[5], 0x0);
                c.r[29] = c.r[5].wrapping_add(0x0);
                if c.crbit(2) { pc = 0x800a04f0; continue; }
                pc = 0x800a04e4;
            }
            0x800a04e4 => {
                c.stw(c.r[13].wrapping_add(0xffff9f34), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffff9f30), c.r[3]);
                pc = 0x800a0574; continue;
                pc = 0x800a04f0;
            }
            0x800a04f0 => {
                c.stw(c.r[13].wrapping_add(0xffff9f34), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9f30), c.r[0]);
                pc = 0x800a0574; continue;
                pc = 0x800a04fc;
            }
            0x800a04fc => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff9f30));
                c.r[8] = 0x0;
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff9f34));
                c.r[0] = c.r[6] ^ c.r[8];
                c.r[5] = c.r[7] ^ c.r[8];
                c.r[0] = c.r[5] | c.r[0];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a0564; continue; }
                pc = 0x800a0518;
            }
            0x800a0518 => {
                c.r[5] = 0x80000000;
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xf8));
                c.r[5] = 0x10620000;
                c.r[5] = c.r[5].wrapping_add(0x4dd3);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = mulhwu(c.r[5], c.r[0]);
                c.r[0] = c.r[0].rotate_left(26) & 0x3ffffff;
                c.r[0] = c.r[0].wrapping_mul(0x28);
                c.r[7] = c.addc(!c.r[7], c.r[4], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[3], ci); }
                c.r[6] = c.r[5] ^ 0x80000000;
                c.r[5] = c.r[8] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[0], c.r[7], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[6], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x800a0564; continue; }
                pc = 0x800a055c;
            }
            0x800a055c => {
                c.r[29] = 0x1;
                pc = 0x800a0574; continue;
                pc = 0x800a0564;
            }
            0x800a0564 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9f34), c.r[0]);
                c.r[29] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9f30), c.r[0]);
                pc = 0x800a0574;
            }
            0x800a0574 => {
                c.r[5] = 0x80000000;
                c.stw(c.r[13].wrapping_add(0xffff9f28), c.r[29]);
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x30e3));
                c.r[0] = c.r[0].rotate_left(0) & 0x3f;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a0624; continue; }
                pc = 0x800a0588;
            }
            0x800a0588 => {
                c.r[10] = c.r[0].wrapping_mul(0x3c);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xf8));
                c.r[9] = c.lwz(c.r[13].wrapping_add(0xffff9ebc));
                c.r[8] = c.lwz(c.r[13].wrapping_add(0xffff9eb8));
                c.r[6] = c.r[0].rotate_left(30) & 0x3fffffff;
                { let v = c.r[10] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[0] = (v >> 31) as u32; }
                c.r[7] = c.r[0].wrapping_mul(c.r[6]);
                c.r[0] = mulhwu(c.r[10], c.r[6]);
                c.r[5] = c.r[10].wrapping_mul(c.r[6]);
                c.r[9] = c.addc(c.r[9], c.r[5], 0);
                c.r[31] = 0x0;
                c.r[7] = c.r[7].wrapping_add(c.r[0]);
                c.r[0] = c.r[10].wrapping_mul(c.r[31]);
                c.r[0] = c.r[7].wrapping_add(c.r[0]);
                { let ci = c.ca(); c.r[8] = c.addc(c.r[8], c.r[0], ci); }
                c.r[7] = c.r[8] ^ 0x80000000;
                c.r[5] = c.r[3] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[9], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[7], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[7], c.r[7], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x800a0624; continue; }
                pc = 0x800a05e0;
            }
            0x800a05e0 => {
                c.r[4] = c.addc(!c.r[9], c.r[4], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[8], c.r[3], ci); }
                c.r[5] = 0x0;
                c.lr = 0x800a05f0; f_800c4928(c);
                pc = 0x800a05f0;
            }
            0x800a05f0 => {
                c.r[5] = 0x0;
                c.r[6] = 0x2;
                c.lr = 0x800a05fc; f_800c4928(c);
                pc = 0x800a05fc;
            }
            0x800a05fc => {
                c.r[0] = 0x1;
                c.r[4] = c.r[4] & c.r[0];
                c.r[0] = c.r[3] & c.r[31];
                c.r[3] = c.r[4] ^ c.r[31];
                c.r[0] = c.r[0] ^ c.r[31];
                c.r[0] = c.r[3] | c.r[0];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a0620; continue; }
                pc = 0x800a0618;
            }
            0x800a0618 => {
                c.r[29] = 0x1;
                pc = 0x800a0624; continue;
                pc = 0x800a0620;
            }
            0x800a0620 => {
                c.r[29] = 0x0;
                pc = 0x800a0624;
            }
            0x800a0624 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a062c; f_8009df64(c);
                pc = 0x800a062c;
            }
            0x800a062c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a03b4: bad pc {:#x}", pc),
        }
    }
}

/// WriteSramCallback (0x800a064c, 96 bytes)
#[inline(never)]
pub fn f_800a064c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a064c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a064c); f_800a064c_b(c, 0x800a064c); crate::prof::exit(c, 0x800a064c); return; }
    }
    f_800a064c_b(c, 0x800a064c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a064c_at(c: &mut Cpu, pc: u32) { f_800a064c_b(c, pc) }
#[inline(always)]
fn f_800a064c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a064c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a064c, pc); }
        match pc {
            0x800a064c => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffffb840);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[31].wrapping_add(0x40);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x40));
                c.r[3] = c.r[31].wrapping_add(c.r[4]);
                c.r[5] = c.addc(!c.r[4], 0x40, 1);
                c.lr = 0x800a067c; f_800a06ac(c);
                pc = 0x800a067c;
            }
            0x800a067c => {
                c.stw(c.r[31].wrapping_add(0x4c), c.r[3]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4c));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a0694; continue; }
                pc = 0x800a068c;
            }
            0x800a068c => {
                c.r[0] = 0x40;
                c.stw(c.r[30], c.r[0]);
                pc = 0x800a0694;
            }
            0x800a0694 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a064c: bad pc {:#x}", pc),
        }
    }
}

/// WriteSram (0x800a06ac, 280 bytes)
#[inline(never)]
pub fn f_800a06ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a06ac) { return; }
        if c.prof { crate::prof::enter(c, 0x800a06ac); f_800a06ac_b(c, 0x800a06ac); crate::prof::exit(c, 0x800a06ac); return; }
    }
    f_800a06ac_b(c, 0x800a06ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a06ac_at(c: &mut Cpu, pc: u32) { f_800a06ac_b(c, pc) }
#[inline(always)]
fn f_800a06ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a06ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a06ac, pc); }
        match pc {
            0x800a06ac => {
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x800a0000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[6].wrapping_add(0x64c);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.r[5] = c.r[0] | c.r[0];
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x0;
                c.lr = 0x800a06e8; f_800991e4(c);
                pc = 0x800a06e8;
            }
            0x800a06e8 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a06f8; continue; }
                pc = 0x800a06f0;
            }
            0x800a06f0 => {
                c.r[3] = 0x0;
                pc = 0x800a07a8; continue;
                pc = 0x800a06f8;
            }
            0x800a06f8 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.r[5] = 0x3;
                c.lr = 0x800a0708; f_80098b94(c);
                pc = 0x800a0708;
            }
            0x800a0708 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a0720; continue; }
                pc = 0x800a0710;
            }
            0x800a0710 => {
                c.r[3] = 0x0;
                c.lr = 0x800a0718; f_800992e0(c);
                pc = 0x800a0718;
            }
            0x800a0718 => {
                c.r[3] = 0x0;
                pc = 0x800a07a8; continue;
                pc = 0x800a0720;
            }
            0x800a0720 => {
                c.r[31] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.r[0] = c.r[31].wrapping_add(0x100);
                c.r[0] = c.r[0] | 0xa0000000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x800a0748; f_8009820c(c);
                pc = 0x800a0748;
            }
            0x800a0748 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[31] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[3] = 0x0;
                c.lr = 0x800a0758; f_80098530(c);
                pc = 0x800a0758;
            }
            0x800a0758 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.r[5] = c.r[30].wrapping_add(0x0);
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x800a0778; f_80098368(c);
                pc = 0x800a0778;
            }
            0x800a0778 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.lr = 0x800a078c; f_80098cd4(c);
                pc = 0x800a078c;
            }
            0x800a078c => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.lr = 0x800a07a0; f_800992e0(c);
                pc = 0x800a07a0;
            }
            0x800a07a0 => {
                c.r[0] = c.r[31].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800a07a8;
            }
            0x800a07a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a06ac: bad pc {:#x}", pc),
        }
    }
}

/// __OSInitSram (0x800a07c4, 308 bytes)
#[inline(never)]
pub fn f_800a07c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a07c4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a07c4); f_800a07c4_b(c, 0x800a07c4); crate::prof::exit(c, 0x800a07c4); return; }
    }
    f_800a07c4_b(c, 0x800a07c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a07c4_at(c: &mut Cpu, pc: u32) { f_800a07c4_b(c, pc) }
#[inline(always)]
fn f_800a07c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a07c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a07c4, pc); }
        match pc {
            0x800a07c4 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[4] = 0x40;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0xffffb840);
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.stw(c.r[30].wrapping_add(0x44), c.r[31]);
                c.stw(c.r[30].wrapping_add(0x48), c.r[31]);
                c.lr = 0x800a07f8; f_8009b2a4(c);
                pc = 0x800a07f8;
            }
            0x800a07f8 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x800a0808; f_800991e4(c);
                pc = 0x800a0808;
            }
            0x800a0808 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a0814; continue; }
                pc = 0x800a0810;
            }
            0x800a0810 => {
                pc = 0x800a08d4; continue;
                pc = 0x800a0814;
            }
            0x800a0814 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.r[5] = 0x3;
                c.lr = 0x800a0824; f_80098b94(c);
                pc = 0x800a0824;
            }
            0x800a0824 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a0838; continue; }
                pc = 0x800a082c;
            }
            0x800a082c => {
                c.r[3] = 0x0;
                c.lr = 0x800a0834; f_800992e0(c);
                pc = 0x800a0834;
            }
            0x800a0834 => {
                pc = 0x800a08d4; continue;
                pc = 0x800a0838;
            }
            0x800a0838 => {
                c.r[3] = 0x20000000;
                c.r[0] = c.r[3].wrapping_add(0x100);
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x800a085c; f_8009820c(c);
                pc = 0x800a085c;
            }
            0x800a085c => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[31] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[3] = 0x0;
                c.lr = 0x800a086c; f_80098530(c);
                pc = 0x800a086c;
            }
            0x800a086c => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.r[5] = 0x40;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x800a0890; f_80098408(c);
                pc = 0x800a0890;
            }
            0x800a0890 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.lr = 0x800a08a4; f_80098530(c);
                pc = 0x800a08a4;
            }
            0x800a08a4 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.lr = 0x800a08b8; f_80098cd4(c);
                pc = 0x800a08b8;
            }
            0x800a08b8 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.lr = 0x800a08cc; f_800992e0(c);
                pc = 0x800a08cc;
            }
            0x800a08cc => {
                c.r[0] = c.r[31].leading_zeros();
                c.r[31] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800a08d4;
            }
            0x800a08d4 => {
                c.stw(c.r[30].wrapping_add(0x4c), c.r[31]);
                c.r[0] = 0x40;
                c.stw(c.r[30].wrapping_add(0x40), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a07c4: bad pc {:#x}", pc),
        }
    }
}

/// __OSLockSram (0x800a08f8, 92 bytes)
#[inline(never)]
pub fn f_800a08f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a08f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a08f8); f_800a08f8_b(c, 0x800a08f8); crate::prof::exit(c, 0x800a08f8); return; }
    }
    f_800a08f8_b(c, 0x800a08f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a08f8_at(c: &mut Cpu, pc: u32) { f_800a08f8_b(c, pc) }
#[inline(always)]
fn f_800a08f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a08f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a08f8, pc); }
        match pc {
            0x800a08f8 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffffb840);
                c.lr = 0x800a0914; f_8009df3c(c);
                pc = 0x800a0914;
            }
            0x800a0914 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = c.r[31].wrapping_add(0x48);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a0930; continue; }
                pc = 0x800a0924;
            }
            0x800a0924 => {
                c.lr = 0x800a0928; f_8009df64(c);
                pc = 0x800a0928;
            }
            0x800a0928 => {
                c.r[31] = 0x0;
                pc = 0x800a093c; continue;
                pc = 0x800a0930;
            }
            0x800a0930 => {
                c.stw(c.r[31].wrapping_add(0x44), c.r[3]);
                c.r[0] = 0x1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800a093c;
            }
            0x800a093c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a08f8: bad pc {:#x}", pc),
        }
    }
}

/// __OSLockSramEx (0x800a0954, 92 bytes)
#[inline(never)]
pub fn f_800a0954(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0954) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0954); f_800a0954_b(c, 0x800a0954); crate::prof::exit(c, 0x800a0954); return; }
    }
    f_800a0954_b(c, 0x800a0954)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0954_at(c: &mut Cpu, pc: u32) { f_800a0954_b(c, pc) }
#[inline(always)]
fn f_800a0954_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0954;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0954, pc); }
        match pc {
            0x800a0954 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffffb840);
                c.lr = 0x800a0970; f_8009df3c(c);
                pc = 0x800a0970;
            }
            0x800a0970 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = c.r[31].wrapping_add(0x48);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a098c; continue; }
                pc = 0x800a0980;
            }
            0x800a0980 => {
                c.lr = 0x800a0984; f_8009df64(c);
                pc = 0x800a0984;
            }
            0x800a0984 => {
                c.r[3] = 0x0;
                pc = 0x800a099c; continue;
                pc = 0x800a098c;
            }
            0x800a098c => {
                c.stw(c.r[31].wrapping_add(0x44), c.r[3]);
                c.r[0] = 0x1;
                c.r[3] = c.r[31].wrapping_add(0x14);
                c.stw(c.r[4], c.r[0]);
                pc = 0x800a099c;
            }
            0x800a099c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a0954: bad pc {:#x}", pc),
        }
    }
}

/// fn_800A09B0 (0x800a09b0, 776 bytes)
#[inline(never)]
pub fn f_800a09b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a09b0) { return; }
        if c.prof { crate::prof::enter(c, 0x800a09b0); f_800a09b0_b(c, 0x800a09b0); crate::prof::exit(c, 0x800a09b0); return; }
    }
    f_800a09b0_b(c, 0x800a09b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a09b0_at(c: &mut Cpu, pc: u32) { f_800a09b0_b(c, pc) }
#[inline(always)]
fn f_800a09b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a09b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a09b0, pc); }
        match pc {
            0x800a09b0 => {
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = 0x80400000;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3].wrapping_add(0xffffb840);
                if c.crbit(2) { pc = 0x800a0c90; continue; }
                pc = 0x800a09d0;
            }
            0x800a09d0 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a0b84; continue; }
                pc = 0x800a09d8;
            }
            0x800a09d8 => {
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x13));
                c.r[0] = c.r[3].rotate_left(0) & 0x3;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(1) { pc = 0x800a09f0; continue; }
                pc = 0x800a09e8;
            }
            0x800a09e8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xfffffffc;
                c.stb(c.r[31].wrapping_add(0x13), c.r[0]);
                pc = 0x800a09f0;
            }
            0x800a09f0 => {
                c.r[0] = 0x0;
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                c.r[5] = c.r[31].wrapping_add(0x14);
                c.r[6] = c.r[31].wrapping_add(0xc);
                c.r[3] = c.r[5].wrapping_add(0x1);
                c.sth(c.r[31], c.r[0]);
                c.r[3] = c.r[3].wrapping_sub(c.r[6]);
                c.cmpu(0, c.r[6], c.r[5]);
                c.r[3] = c.r[3].rotate_left(31) & 0x7fffffff;
                if !c.crbit(0) { pc = 0x800a0b84; continue; }
                pc = 0x800a0a18;
            }
            0x800a0a18 => {
                c.r[0] = c.r[3].rotate_left(29) & 0x1fffffff;
                c.rc(c.r[0]);
                c.mtspr(9, c.r[0]);
                if c.crbit(2) { pc = 0x800a0b54; continue; }
                pc = 0x800a0a24;
            }
            0x800a0a24 => {
                c.r[5] = c.lhz(c.r[31]);
                c.r[0] = c.lhz(c.r[6]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[6]);
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.r[0] = !(c.r[0] | c.r[0]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                c.r[5] = c.lhz(c.r[31]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x2));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x2));
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.r[0] = !(c.r[0] | c.r[0]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                c.r[5] = c.lhz(c.r[31]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x4));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x4));
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.r[0] = !(c.r[0] | c.r[0]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                c.r[5] = c.lhz(c.r[31]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x6));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x6));
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.r[0] = !(c.r[0] | c.r[0]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                c.r[5] = c.lhz(c.r[31]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x8));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x8));
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.r[0] = !(c.r[0] | c.r[0]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                c.r[5] = c.lhz(c.r[31]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0xa));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0xa));
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.r[0] = !(c.r[0] | c.r[0]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                c.r[5] = c.lhz(c.r[31]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0xc));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0xc));
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.r[0] = !(c.r[0] | c.r[0]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                c.r[5] = c.lhz(c.r[31]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0xe));
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0xe));
                c.r[6] = c.r[6].wrapping_add(0x10);
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.r[0] = !(c.r[0] | c.r[0]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800a0a24; continue; }
                pc = 0x800a0b4c;
            }
            0x800a0b4c => {
                c.r[3] = c.r[3] & 0x7;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x800a0b84; continue; }
                pc = 0x800a0b54;
            }
            0x800a0b54 => {
                c.mtspr(9, c.r[3]);
                pc = 0x800a0b58;
            }
            0x800a0b58 => {
                c.r[5] = c.lhz(c.r[31]);
                c.r[0] = c.lhz(c.r[6]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31], c.r[0]);
                c.r[0] = c.lhz(c.r[6]);
                c.r[6] = c.r[6].wrapping_add(0x2);
                c.r[5] = c.lhz(c.r[31].wrapping_add(0x2));
                c.r[0] = !(c.r[0] | c.r[0]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.sth(c.r[31].wrapping_add(0x2), c.r[0]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800a0b58; continue; }
                pc = 0x800a0b84;
            }
            0x800a0b84 => {
                c.r[30] = c.r[31].wrapping_add(0x40);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x40));
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(0) { pc = 0x800a0b98; continue; }
                pc = 0x800a0b94;
            }
            0x800a0b94 => {
                c.stw(c.r[30], c.r[4]);
                pc = 0x800a0b98;
            }
            0x800a0b98 => {
                c.r[29] = c.lwz(c.r[30]);
                c.r[3] = 0x800a0000;
                c.r[5] = c.r[3].wrapping_add(0x64c);
                c.r[27] = c.addc(!c.r[29], 0x40, 1);
                c.r[28] = c.r[31].wrapping_add(c.r[29]);
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.lr = 0x800a0bb8; f_800991e4(c);
                pc = 0x800a0bb8;
            }
            0x800a0bb8 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a0bc8; continue; }
                pc = 0x800a0bc0;
            }
            0x800a0bc0 => {
                c.r[0] = 0x0;
                pc = 0x800a0c78; continue;
                pc = 0x800a0bc8;
            }
            0x800a0bc8 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.r[5] = 0x3;
                c.lr = 0x800a0bd8; f_80098b94(c);
                pc = 0x800a0bd8;
            }
            0x800a0bd8 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a0bf0; continue; }
                pc = 0x800a0be0;
            }
            0x800a0be0 => {
                c.r[3] = 0x0;
                c.lr = 0x800a0be8; f_800992e0(c);
                pc = 0x800a0be8;
            }
            0x800a0be8 => {
                c.r[0] = 0x0;
                pc = 0x800a0c78; continue;
                pc = 0x800a0bf0;
            }
            0x800a0bf0 => {
                c.r[3] = c.r[29].rotate_left(6) & 0xffffffc0;
                c.r[0] = c.r[3].wrapping_add(0x100);
                c.r[0] = c.r[0] | 0xa0000000;
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x800a0c18; f_8009820c(c);
                pc = 0x800a0c18;
            }
            0x800a0c18 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[29] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[3] = 0x0;
                c.lr = 0x800a0c28; f_80098530(c);
                pc = 0x800a0c28;
            }
            0x800a0c28 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[4] = c.r[28].wrapping_add(0x0);
                c.r[5] = c.r[27].wrapping_add(0x0);
                c.r[29] = c.r[29] | c.r[0];
                c.r[3] = 0x0;
                c.r[6] = 0x1;
                c.lr = 0x800a0c48; f_80098368(c);
                pc = 0x800a0c48;
            }
            0x800a0c48 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[29] = c.r[29] | c.r[0];
                c.r[3] = 0x0;
                c.lr = 0x800a0c5c; f_80098cd4(c);
                pc = 0x800a0c5c;
            }
            0x800a0c5c => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[29] = c.r[29] | c.r[0];
                c.r[3] = 0x0;
                c.lr = 0x800a0c70; f_800992e0(c);
                pc = 0x800a0c70;
            }
            0x800a0c70 => {
                c.r[0] = c.r[29].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800a0c78;
            }
            0x800a0c78 => {
                c.stw(c.r[31].wrapping_add(0x4c), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4c));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a0c90; continue; }
                pc = 0x800a0c88;
            }
            0x800a0c88 => {
                c.r[0] = 0x40;
                c.stw(c.r[30], c.r[0]);
                pc = 0x800a0c90;
            }
            0x800a0c90 => {
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x48), c.r[0]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x44));
                c.lr = 0x800a0ca0; f_8009df64(c);
                pc = 0x800a0ca0;
            }
            0x800a0ca0 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x4c));
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a09b0: bad pc {:#x}", pc),
        }
    }
}

/// __OSUnlockSram (0x800a0cb8, 36 bytes)
#[inline(never)]
pub fn f_800a0cb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0cb8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0cb8); f_800a0cb8_b(c, 0x800a0cb8); crate::prof::exit(c, 0x800a0cb8); return; }
    }
    f_800a0cb8_b(c, 0x800a0cb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0cb8_at(c: &mut Cpu, pc: u32) { f_800a0cb8_b(c, pc) }
#[inline(always)]
fn f_800a0cb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0cb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0cb8, pc); }
        match pc {
            0x800a0cb8 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a0ccc; f_800a09b0(c);
                pc = 0x800a0ccc;
            }
            0x800a0ccc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a0cb8: bad pc {:#x}", pc),
        }
    }
}

/// __OSUnlockSramEx (0x800a0cdc, 36 bytes)
#[inline(never)]
pub fn f_800a0cdc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0cdc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0cdc); f_800a0cdc_b(c, 0x800a0cdc); crate::prof::exit(c, 0x800a0cdc); return; }
    }
    f_800a0cdc_b(c, 0x800a0cdc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0cdc_at(c: &mut Cpu, pc: u32) { f_800a0cdc_b(c, pc) }
#[inline(always)]
fn f_800a0cdc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0cdc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0cdc, pc); }
        match pc {
            0x800a0cdc => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x14;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800a0cf0; f_800a09b0(c);
                pc = 0x800a0cf0;
            }
            0x800a0cf0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a0cdc: bad pc {:#x}", pc),
        }
    }
}

/// __OSSyncSram (0x800a0d00, 16 bytes)
#[inline(never)]
pub fn f_800a0d00(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0d00) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0d00); f_800a0d00_b(c, 0x800a0d00); crate::prof::exit(c, 0x800a0d00); return; }
    }
    f_800a0d00_b(c, 0x800a0d00)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0d00_at(c: &mut Cpu, pc: u32) { f_800a0d00_b(c, pc) }
#[inline(always)]
fn f_800a0d00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0d00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0d00, pc); }
        match pc {
            0x800a0d00 => {
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffb840);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4c));
                if true { return; }
                return;
            }
            _ => panic!("f_800a0d00: bad pc {:#x}", pc),
        }
    }
}

/// __OSReadROM (0x800a0d10, 292 bytes)
#[inline(never)]
pub fn f_800a0d10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0d10) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0d10); f_800a0d10_b(c, 0x800a0d10); crate::prof::exit(c, 0x800a0d10); return; }
    }
    f_800a0d10_b(c, 0x800a0d10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0d10_at(c: &mut Cpu, pc: u32) { f_800a0d10_b(c, pc) }
#[inline(always)]
fn f_800a0d10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0d10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0d10, pc); }
        match pc {
            0x800a0d10 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800a0d38; f_8009b2a4(c);
                pc = 0x800a0d38;
            }
            0x800a0d38 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x800a0d48; f_800991e4(c);
                pc = 0x800a0d48;
            }
            0x800a0d48 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a0d58; continue; }
                pc = 0x800a0d50;
            }
            0x800a0d50 => {
                c.r[3] = 0x0;
                pc = 0x800a0e18; continue;
                pc = 0x800a0d58;
            }
            0x800a0d58 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.r[5] = 0x3;
                c.lr = 0x800a0d68; f_80098b94(c);
                pc = 0x800a0d68;
            }
            0x800a0d68 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a0d80; continue; }
                pc = 0x800a0d70;
            }
            0x800a0d70 => {
                c.r[3] = 0x0;
                c.lr = 0x800a0d78; f_800992e0(c);
                pc = 0x800a0d78;
            }
            0x800a0d78 => {
                c.r[3] = 0x0;
                pc = 0x800a0e18; continue;
                pc = 0x800a0d80;
            }
            0x800a0d80 => {
                c.r[0] = c.r[31].rotate_left(6) & 0xffffffc0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[3] = 0x0;
                c.r[5] = 0x4;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x800a0da0; f_8009820c(c);
                pc = 0x800a0da0;
            }
            0x800a0da0 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[31] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[3] = 0x0;
                c.lr = 0x800a0db0; f_80098530(c);
                pc = 0x800a0db0;
            }
            0x800a0db0 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.r[5] = c.r[30].wrapping_add(0x0);
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x800a0dd4; f_80098408(c);
                pc = 0x800a0dd4;
            }
            0x800a0dd4 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.lr = 0x800a0de8; f_80098530(c);
                pc = 0x800a0de8;
            }
            0x800a0de8 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.lr = 0x800a0dfc; f_80098cd4(c);
                pc = 0x800a0dfc;
            }
            0x800a0dfc => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[31] = c.r[31] | c.r[0];
                c.r[3] = 0x0;
                c.lr = 0x800a0e10; f_800992e0(c);
                pc = 0x800a0e10;
            }
            0x800a0e10 => {
                c.r[0] = c.r[31].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800a0e18;
            }
            0x800a0e18 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a0d10: bad pc {:#x}", pc),
        }
    }
}

/// OSGetSoundMode (0x800a0e34, 128 bytes)
#[inline(never)]
pub fn f_800a0e34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0e34) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0e34); f_800a0e34_b(c, 0x800a0e34); crate::prof::exit(c, 0x800a0e34); return; }
    }
    f_800a0e34_b(c, 0x800a0e34)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0e34_at(c: &mut Cpu, pc: u32) { f_800a0e34_b(c, pc) }
#[inline(always)]
fn f_800a0e34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0e34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0e34, pc); }
        match pc {
            0x800a0e34 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffffb840);
                c.lr = 0x800a0e50; f_8009df3c(c);
                pc = 0x800a0e50;
            }
            0x800a0e50 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = c.r[31].wrapping_add(0x48);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a0e6c; continue; }
                pc = 0x800a0e60;
            }
            0x800a0e60 => {
                c.lr = 0x800a0e64; f_8009df64(c);
                pc = 0x800a0e64;
            }
            0x800a0e64 => {
                c.r[31] = 0x0;
                pc = 0x800a0e78; continue;
                pc = 0x800a0e6c;
            }
            0x800a0e6c => {
                c.stw(c.r[31].wrapping_add(0x44), c.r[3]);
                c.r[0] = 0x1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800a0e78;
            }
            0x800a0e78 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x13));
                c.r[0] = c.r[0].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a0e8c; continue; }
                pc = 0x800a0e84;
            }
            0x800a0e84 => {
                c.r[31] = 0x1;
                pc = 0x800a0e90; continue;
                pc = 0x800a0e8c;
            }
            0x800a0e8c => {
                c.r[31] = 0x0;
                pc = 0x800a0e90;
            }
            0x800a0e90 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x800a0e9c; f_800a09b0(c);
                pc = 0x800a0e9c;
            }
            0x800a0e9c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a0e34: bad pc {:#x}", pc),
        }
    }
}

/// fn_800A0EB4 (0x800a0eb4, 164 bytes)
#[inline(never)]
pub fn f_800a0eb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0eb4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0eb4); f_800a0eb4_b(c, 0x800a0eb4); crate::prof::exit(c, 0x800a0eb4); return; }
    }
    f_800a0eb4_b(c, 0x800a0eb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0eb4_at(c: &mut Cpu, pc: u32) { f_800a0eb4_b(c, pc) }
#[inline(always)]
fn f_800a0eb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0eb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0eb4, pc); }
        match pc {
            0x800a0eb4 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffb840);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3].rotate_left(2) & 0x4;
                c.lr = 0x800a0ed8; f_8009df3c(c);
                pc = 0x800a0ed8;
            }
            0x800a0ed8 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = c.r[31].wrapping_add(0x48);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a0ef4; continue; }
                pc = 0x800a0ee8;
            }
            0x800a0ee8 => {
                c.lr = 0x800a0eec; f_8009df64(c);
                pc = 0x800a0eec;
            }
            0x800a0eec => {
                c.r[31] = 0x0;
                pc = 0x800a0f00; continue;
                pc = 0x800a0ef4;
            }
            0x800a0ef4 => {
                c.stw(c.r[31].wrapping_add(0x44), c.r[3]);
                c.r[0] = 0x1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800a0f00;
            }
            0x800a0f00 => {
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x13));
                c.r[0] = c.r[3].rotate_left(0) & 0x4;
                c.cmpu(0, c.r[30], c.r[0]);
                if !c.crbit(2) { pc = 0x800a0f20; continue; }
                pc = 0x800a0f10;
            }
            0x800a0f10 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x800a0f1c; f_800a09b0(c);
                pc = 0x800a0f1c;
            }
            0x800a0f1c => {
                pc = 0x800a0f40; continue;
                pc = 0x800a0f20;
            }
            0x800a0f20 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xfffffffb;
                c.stb(c.r[31].wrapping_add(0x13), c.r[0]);
                c.r[3] = 0x1;
                c.r[4] = 0x0;
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x13));
                c.r[0] = c.r[0] | c.r[30];
                c.stb(c.r[31].wrapping_add(0x13), c.r[0]);
                c.lr = 0x800a0f40; f_800a09b0(c);
                pc = 0x800a0f40;
            }
            0x800a0f40 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a0eb4: bad pc {:#x}", pc),
        }
    }
}

/// OSGetProgressiveMode (0x800a0f58, 112 bytes)
#[inline(never)]
pub fn f_800a0f58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0f58) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0f58); f_800a0f58_b(c, 0x800a0f58); crate::prof::exit(c, 0x800a0f58); return; }
    }
    f_800a0f58_b(c, 0x800a0f58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0f58_at(c: &mut Cpu, pc: u32) { f_800a0f58_b(c, pc) }
#[inline(always)]
fn f_800a0f58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0f58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0f58, pc); }
        match pc {
            0x800a0f58 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffffb840);
                c.lr = 0x800a0f74; f_8009df3c(c);
                pc = 0x800a0f74;
            }
            0x800a0f74 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = c.r[31].wrapping_add(0x48);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a0f90; continue; }
                pc = 0x800a0f84;
            }
            0x800a0f84 => {
                c.lr = 0x800a0f88; f_8009df64(c);
                pc = 0x800a0f88;
            }
            0x800a0f88 => {
                c.r[31] = 0x0;
                pc = 0x800a0f9c; continue;
                pc = 0x800a0f90;
            }
            0x800a0f90 => {
                c.stw(c.r[31].wrapping_add(0x44), c.r[3]);
                c.r[0] = 0x1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800a0f9c;
            }
            0x800a0f9c => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x13));
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[31] = c.r[0].rotate_left(25) & 0x1;
                c.lr = 0x800a0fb0; f_800a09b0(c);
                pc = 0x800a0fb0;
            }
            0x800a0fb0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a0f58: bad pc {:#x}", pc),
        }
    }
}

/// fn_800A0FC8 (0x800a0fc8, 164 bytes)
#[inline(never)]
pub fn f_800a0fc8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a0fc8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a0fc8); f_800a0fc8_b(c, 0x800a0fc8); crate::prof::exit(c, 0x800a0fc8); return; }
    }
    f_800a0fc8_b(c, 0x800a0fc8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a0fc8_at(c: &mut Cpu, pc: u32) { f_800a0fc8_b(c, pc) }
#[inline(always)]
fn f_800a0fc8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a0fc8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a0fc8, pc); }
        match pc {
            0x800a0fc8 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffb840);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3].rotate_left(7) & 0x80;
                c.lr = 0x800a0fec; f_8009df3c(c);
                pc = 0x800a0fec;
            }
            0x800a0fec => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = c.r[31].wrapping_add(0x48);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a1008; continue; }
                pc = 0x800a0ffc;
            }
            0x800a0ffc => {
                c.lr = 0x800a1000; f_8009df64(c);
                pc = 0x800a1000;
            }
            0x800a1000 => {
                c.r[31] = 0x0;
                pc = 0x800a1014; continue;
                pc = 0x800a1008;
            }
            0x800a1008 => {
                c.stw(c.r[31].wrapping_add(0x44), c.r[3]);
                c.r[0] = 0x1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800a1014;
            }
            0x800a1014 => {
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x13));
                c.r[0] = c.r[3].rotate_left(0) & 0x80;
                c.cmpu(0, c.r[30], c.r[0]);
                if !c.crbit(2) { pc = 0x800a1034; continue; }
                pc = 0x800a1024;
            }
            0x800a1024 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x800a1030; f_800a09b0(c);
                pc = 0x800a1030;
            }
            0x800a1030 => {
                pc = 0x800a1054; continue;
                pc = 0x800a1034;
            }
            0x800a1034 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffffff7f;
                c.stb(c.r[31].wrapping_add(0x13), c.r[0]);
                c.r[3] = 0x1;
                c.r[4] = 0x0;
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x13));
                c.r[0] = c.r[0] | c.r[30];
                c.stb(c.r[31].wrapping_add(0x13), c.r[0]);
                c.lr = 0x800a1054; f_800a09b0(c);
                pc = 0x800a1054;
            }
            0x800a1054 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a0fc8: bad pc {:#x}", pc),
        }
    }
}

/// OSGetLanguage (0x800a106c, 108 bytes)
#[inline(never)]
pub fn f_800a106c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a106c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a106c); f_800a106c_b(c, 0x800a106c); crate::prof::exit(c, 0x800a106c); return; }
    }
    f_800a106c_b(c, 0x800a106c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a106c_at(c: &mut Cpu, pc: u32) { f_800a106c_b(c, pc) }
#[inline(always)]
fn f_800a106c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a106c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a106c, pc); }
        match pc {
            0x800a106c => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xffffb840);
                c.lr = 0x800a1088; f_8009df3c(c);
                pc = 0x800a1088;
            }
            0x800a1088 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = c.r[31].wrapping_add(0x48);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a10a4; continue; }
                pc = 0x800a1098;
            }
            0x800a1098 => {
                c.lr = 0x800a109c; f_8009df64(c);
                pc = 0x800a109c;
            }
            0x800a109c => {
                c.r[31] = 0x0;
                pc = 0x800a10b0; continue;
                pc = 0x800a10a4;
            }
            0x800a10a4 => {
                c.stw(c.r[31].wrapping_add(0x44), c.r[3]);
                c.r[0] = 0x1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800a10b0;
            }
            0x800a10b0 => {
                c.r[31] = c.lbz(c.r[31].wrapping_add(0x12));
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x800a10c0; f_800a09b0(c);
                pc = 0x800a10c0;
            }
            0x800a10c0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a106c: bad pc {:#x}", pc),
        }
    }
}

/// OSGetWirelessID (0x800a10d8, 132 bytes)
#[inline(never)]
pub fn f_800a10d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a10d8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a10d8); f_800a10d8_b(c, 0x800a10d8); crate::prof::exit(c, 0x800a10d8); return; }
    }
    f_800a10d8_b(c, 0x800a10d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a10d8_at(c: &mut Cpu, pc: u32) { f_800a10d8_b(c, pc) }
#[inline(always)]
fn f_800a10d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a10d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a10d8, pc); }
        match pc {
            0x800a10d8 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffb840);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800a10fc; f_8009df3c(c);
                pc = 0x800a10fc;
            }
            0x800a10fc => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = c.r[31].wrapping_add(0x48);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a1118; continue; }
                pc = 0x800a110c;
            }
            0x800a110c => {
                c.lr = 0x800a1110; f_8009df64(c);
                pc = 0x800a1110;
            }
            0x800a1110 => {
                c.r[3] = 0x0;
                pc = 0x800a1128; continue;
                pc = 0x800a1118;
            }
            0x800a1118 => {
                c.stw(c.r[31].wrapping_add(0x44), c.r[3]);
                c.r[0] = 0x1;
                c.r[3] = c.r[31].wrapping_add(0x14);
                c.stw(c.r[4], c.r[0]);
                pc = 0x800a1128;
            }
            0x800a1128 => {
                c.r[0] = c.r[30].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[31] = c.lhz(c.r[3].wrapping_add(0x1c));
                c.r[3] = 0x0;
                c.r[4] = 0x14;
                c.lr = 0x800a1140; f_800a09b0(c);
                pc = 0x800a1140;
            }
            0x800a1140 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a10d8: bad pc {:#x}", pc),
        }
    }
}

/// OSSetWirelessID (0x800a115c, 172 bytes)
#[inline(never)]
pub fn f_800a115c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a115c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a115c); f_800a115c_b(c, 0x800a115c); crate::prof::exit(c, 0x800a115c); return; }
    }
    f_800a115c_b(c, 0x800a115c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a115c_at(c: &mut Cpu, pc: u32) { f_800a115c_b(c, pc) }
#[inline(always)]
fn f_800a115c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a115c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a115c, pc); }
        match pc {
            0x800a115c => {
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[5].wrapping_add(0xffffb840);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800a1188; f_8009df3c(c);
                pc = 0x800a1188;
            }
            0x800a1188 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = c.r[31].wrapping_add(0x48);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a11a4; continue; }
                pc = 0x800a1198;
            }
            0x800a1198 => {
                c.lr = 0x800a119c; f_8009df64(c);
                pc = 0x800a119c;
            }
            0x800a119c => {
                c.r[3] = 0x0;
                pc = 0x800a11b4; continue;
                pc = 0x800a11a4;
            }
            0x800a11a4 => {
                c.stw(c.r[31].wrapping_add(0x44), c.r[3]);
                c.r[0] = 0x1;
                c.r[3] = c.r[31].wrapping_add(0x14);
                c.stw(c.r[4], c.r[0]);
                pc = 0x800a11b4;
            }
            0x800a11b4 => {
                c.r[0] = c.r[29].rotate_left(1) & 0xfffffffe;
                c.r[4] = c.r[3].wrapping_add(c.r[0]);
                { let ea = c.r[4].wrapping_add(0x1c); c.r[3] = c.lhz(ea); c.r[4] = ea; }
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x800a11e0; continue; }
                pc = 0x800a11cc;
            }
            0x800a11cc => {
                c.sth(c.r[4], c.r[30]);
                c.r[3] = 0x1;
                c.r[4] = 0x14;
                c.lr = 0x800a11dc; f_800a09b0(c);
                pc = 0x800a11dc;
            }
            0x800a11dc => {
                pc = 0x800a11ec; continue;
                pc = 0x800a11e0;
            }
            0x800a11e0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x14;
                c.lr = 0x800a11ec; f_800a09b0(c);
                pc = 0x800a11ec;
            }
            0x800a11ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a115c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800A1208 (0x800a1208, 32 bytes)
#[inline(never)]
pub fn f_800a1208(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1208) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1208); f_800a1208_b(c, 0x800a1208); crate::prof::exit(c, 0x800a1208); return; }
    }
    f_800a1208_b(c, 0x800a1208)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1208_at(c: &mut Cpu, pc: u32) { f_800a1208_b(c, pc) }
#[inline(always)]
fn f_800a1208_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1208;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1208, pc); }
        match pc {
            0x800a1208 => {
                c.r[9] = c.mfspr(1008);
                c.r[10] = c.r[9] | 0x8;
                c.mtspr(1008, c.r[10]);
                // isync
                // sync
                c.mtspr(1008, c.r[9]);
                // rfi
                f_800a1228(c); return;
                return;
            }
            _ => panic!("f_800a1208: bad pc {:#x}", pc),
        }
    }
}

/// __OSInitSystemCall (0x800a1228, 100 bytes)
#[inline(never)]
pub fn f_800a1228(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1228) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1228); f_800a1228_b(c, 0x800a1228); crate::prof::exit(c, 0x800a1228); return; }
    }
    f_800a1228_b(c, 0x800a1228)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1228_at(c: &mut Cpu, pc: u32) { f_800a1228_b(c, pc) }
#[inline(always)]
fn f_800a1228_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1228;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1228, pc); }
        match pc {
            0x800a1228 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[5] = 0x80000000;
                c.r[4] = 0x800a0000;
                c.r[3] = 0x800a0000;
                c.r[31] = c.r[5].wrapping_add(0xc00);
                c.r[0] = c.r[3].wrapping_add(0x1224);
                c.r[4] = c.r[4].wrapping_add(0x1208);
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[0].wrapping_sub(c.r[4]);
                c.lr = 0x800a125c; f_800054f4(c);
                pc = 0x800a125c;
            }
            0x800a125c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x100;
                c.lr = 0x800a1268; f_8009b330(c);
                pc = 0x800a1268;
            }
            0x800a1268 => {
                // sync
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x100;
                c.lr = 0x800a1278; f_8009b3b4(c);
                pc = 0x800a1278;
            }
            0x800a1278 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1228: bad pc {:#x}", pc),
        }
    }
}

/// fn_800A128C (0x800a128c, 4 bytes)
#[inline(never)]
pub fn f_800a128c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a128c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a128c); f_800a128c_b(c, 0x800a128c); crate::prof::exit(c, 0x800a128c); return; }
    }
    f_800a128c_b(c, 0x800a128c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a128c_at(c: &mut Cpu, pc: u32) { f_800a128c_b(c, pc) }
#[inline(always)]
fn f_800a128c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a128c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a128c, pc); }
        match pc {
            0x800a128c => {
                if true { return; }
                return;
            }
            _ => panic!("f_800a128c: bad pc {:#x}", pc),
        }
    }
}

/// __OSThreadInit (0x800a1290, 344 bytes)
#[inline(never)]
pub fn f_800a1290(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1290) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1290); f_800a1290_b(c, 0x800a1290); crate::prof::exit(c, 0x800a1290); return; }
    }
    f_800a1290_b(c, 0x800a1290)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1290_at(c: &mut Cpu, pc: u32) { f_800a1290_b(c, pc) }
#[inline(always)]
fn f_800a1290_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1290;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1290, pc); }
        match pc {
            0x800a1290 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x2;
                c.r[4] = 0x10;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[29]);
                c.r[29] = 0x0;
                c.stw(c.r[1].wrapping_add(0x8), c.r[28]);
                c.r[28] = c.r[3].wrapping_add(0xffffb898);
                c.r[31] = c.r[28].wrapping_add(0x418);
                c.sth(c.r[28].wrapping_add(0x6e0), c.r[0]);
                c.r[0] = 0x1;
                c.r[3] = c.r[31].wrapping_add(0x2e8);
                c.sth(c.r[28].wrapping_add(0x6e2), c.r[0]);
                c.r[0] = 0xffffffff;
                c.stw(c.r[28].wrapping_add(0x6ec), c.r[4]);
                c.stw(c.r[28].wrapping_add(0x6e8), c.r[4]);
                c.stw(c.r[28].wrapping_add(0x6e4), c.r[29]);
                c.stw(c.r[28].wrapping_add(0x6f0), c.r[0]);
                c.stw(c.r[28].wrapping_add(0x708), c.r[29]);
                c.lr = 0x800a12f0; f_800a13e8(c);
                pc = 0x800a12f0;
            }
            0x800a12f0 => {
                c.stw(c.r[28].wrapping_add(0x710), c.r[29]);
                c.r[30] = 0x80000000;
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.stw(c.r[28].wrapping_add(0x70c), c.r[29]);
                c.stw(c.r[30].wrapping_add(0xd8), c.r[31]);
                c.lr = 0x800a1308; f_8009bd60(c);
                pc = 0x800a1308;
            }
            0x800a1308 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a1310; f_8009bb68(c);
                pc = 0x800a1310;
            }
            0x800a1310 => {
                c.r[3] = 0x80490000;
                c.r[0] = c.r[3].wrapping_add(0xffffe738);
                c.r[3] = 0x80480000;
                c.stw(c.r[28].wrapping_add(0x71c), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0xffffe738);
                c.stw(c.r[28].wrapping_add(0x720), c.r[0]);
                c.r[3] = 0xdeae0000;
                c.r[0] = c.r[3].wrapping_add(0xffffbabe);
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x720));
                c.r[4] = c.r[31] | c.r[31];
                c.stw(c.r[3], c.r[0]);
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8188));
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xe4));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a134c; (c.dispatch)(c, t); }
                pc = 0x800a134c;
            }
            0x800a134c => {
                c.stw(c.r[30].wrapping_add(0xe4), c.r[31]);
                c.r[3] = 0x0;
                c.lr = 0x800a1358; f_800a26cc(c);
                pc = 0x800a1358;
            }
            0x800a1358 => {
                c.stw(c.r[13].wrapping_add(0xffff9f40), c.r[29]);
                c.r[30] = 0x0;
                c.r[0] = c.r[30].rotate_left(3) & 0xfffffff8;
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[29]);
                c.r[29] = c.r[28].wrapping_add(c.r[0]);
                pc = 0x800a136c;
            }
            0x800a136c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a1374; f_800a13e8(c);
                pc = 0x800a1374;
            }
            0x800a1374 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.cmps(0, c.r[30], 0x1f);
                c.r[29] = c.r[29].wrapping_add(0x8);
                if !c.crbit(1) { pc = 0x800a136c; continue; }
                pc = 0x800a1384;
            }
            0x800a1384 => {
                c.r[30] = 0x80000000;
                c.r[3] = c.r[30].wrapping_add(0xdc);
                c.lr = 0x800a1390; f_800a13e8(c);
                pc = 0x800a1390;
            }
            0x800a1390 => {
                c.r[4] = c.r[30].wrapping_add(0xdc);
                { let ea = c.r[4].wrapping_add(0x4); c.r[3] = c.lwz(ea); c.r[4] = ea; }
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a13a8; continue; }
                pc = 0x800a13a0;
            }
            0x800a13a0 => {
                c.stw(c.r[30].wrapping_add(0xdc), c.r[31]);
                pc = 0x800a13ac; continue;
                pc = 0x800a13a8;
            }
            0x800a13a8 => {
                c.stw(c.r[3].wrapping_add(0x2fc), c.r[31]);
                pc = 0x800a13ac;
            }
            0x800a13ac => {
                c.stw(c.r[31].wrapping_add(0x300), c.r[3]);
                c.r[30] = 0x0;
                c.r[3] = c.r[28].wrapping_add(0x730);
                c.stw(c.r[31].wrapping_add(0x2fc), c.r[30]);
                c.stw(c.r[4], c.r[31]);
                c.lr = 0x800a13c4; f_8009bd60(c);
                pc = 0x800a13c4;
            }
            0x800a13c4 => {
                c.stw(c.r[13].wrapping_add(0xffff9f48), c.r[30]);
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
            _ => panic!("f_800a1290: bad pc {:#x}", pc),
        }
    }
}

/// OSInitThreadQueue (0x800a13e8, 16 bytes)
#[inline(never)]
pub fn f_800a13e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a13e8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a13e8); f_800a13e8_b(c, 0x800a13e8); crate::prof::exit(c, 0x800a13e8); return; }
    }
    f_800a13e8_b(c, 0x800a13e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a13e8_at(c: &mut Cpu, pc: u32) { f_800a13e8_b(c, pc) }
#[inline(always)]
fn f_800a13e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a13e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a13e8, pc); }
        match pc {
            0x800a13e8 => {
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                c.stw(c.r[3], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a13e8: bad pc {:#x}", pc),
        }
    }
}

/// fn_800A13F8 (0x800a13f8, 12 bytes)
#[inline(never)]
pub fn f_800a13f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a13f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a13f8); f_800a13f8_b(c, 0x800a13f8); crate::prof::exit(c, 0x800a13f8); return; }
    }
    f_800a13f8_b(c, 0x800a13f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a13f8_at(c: &mut Cpu, pc: u32) { f_800a13f8_b(c, pc) }
#[inline(always)]
fn f_800a13f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a13f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a13f8, pc); }
        match pc {
            0x800a13f8 => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xe4));
                if true { return; }
                return;
            }
            _ => panic!("f_800a13f8: bad pc {:#x}", pc),
        }
    }
}

/// OSDisableScheduler (0x800a1404, 64 bytes)
#[inline(never)]
pub fn f_800a1404(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1404) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1404); f_800a1404_b(c, 0x800a1404); crate::prof::exit(c, 0x800a1404); return; }
    }
    f_800a1404_b(c, 0x800a1404)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1404_at(c: &mut Cpu, pc: u32) { f_800a1404_b(c, pc) }
#[inline(always)]
fn f_800a1404_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1404;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1404, pc); }
        match pc {
            0x800a1404 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x800a1418; f_8009df3c(c);
                pc = 0x800a1418;
            }
            0x800a1418 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9f48));
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[13].wrapping_add(0xffff9f48), c.r[0]);
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x800a142c; f_8009df64(c);
                pc = 0x800a142c;
            }
            0x800a142c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1404: bad pc {:#x}", pc),
        }
    }
}

/// OSEnableScheduler (0x800a1444, 64 bytes)
#[inline(never)]
pub fn f_800a1444(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1444) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1444); f_800a1444_b(c, 0x800a1444); crate::prof::exit(c, 0x800a1444); return; }
    }
    f_800a1444_b(c, 0x800a1444)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1444_at(c: &mut Cpu, pc: u32) { f_800a1444_b(c, pc) }
#[inline(always)]
fn f_800a1444_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1444;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1444, pc); }
        match pc {
            0x800a1444 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x800a1458; f_8009df3c(c);
                pc = 0x800a1458;
            }
            0x800a1458 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9f48));
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                c.stw(c.r[13].wrapping_add(0xffff9f48), c.r[0]);
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x800a146c; f_8009df64(c);
                pc = 0x800a146c;
            }
            0x800a146c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1444: bad pc {:#x}", pc),
        }
    }
}

/// UnsetRun (0x800a1484, 104 bytes)
#[inline(never)]
pub fn f_800a1484(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1484) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1484); f_800a1484_b(c, 0x800a1484); crate::prof::exit(c, 0x800a1484); return; }
    }
    f_800a1484_b(c, 0x800a1484)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1484_at(c: &mut Cpu, pc: u32) { f_800a1484_b(c, pc) }
#[inline(always)]
fn f_800a1484_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1484;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1484, pc); }
        match pc {
            0x800a1484 => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x2e0));
                c.r[5] = c.lwz(c.r[3].wrapping_add(0x2dc));
                c.cmpu(0, c.r[4], 0x0);
                c.r[6] = c.lwz(c.r[3].wrapping_add(0x2e4));
                if !c.crbit(2) { pc = 0x800a14a0; continue; }
                pc = 0x800a1498;
            }
            0x800a1498 => {
                c.stw(c.r[5].wrapping_add(0x4), c.r[6]);
                pc = 0x800a14a4; continue;
                pc = 0x800a14a0;
            }
            0x800a14a0 => {
                c.stw(c.r[4].wrapping_add(0x2e4), c.r[6]);
                pc = 0x800a14a4;
            }
            0x800a14a4 => {
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x800a14b4; continue; }
                pc = 0x800a14ac;
            }
            0x800a14ac => {
                c.stw(c.r[5], c.r[4]);
                pc = 0x800a14b8; continue;
                pc = 0x800a14b4;
            }
            0x800a14b4 => {
                c.stw(c.r[6].wrapping_add(0x2e0), c.r[4]);
                pc = 0x800a14b8;
            }
            0x800a14b8 => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a14e0; continue; }
                pc = 0x800a14c4;
            }
            0x800a14c4 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x2d0));
                c.r[4] = 0x1;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.r[0] = c.addc(!c.r[0], 0x1f, 1);
                c.r[0] = slw(c.r[4], c.r[0]);
                c.r[0] = c.r[5] & !c.r[0];
                c.stw(c.r[13].wrapping_add(0xffff9f40), c.r[0]);
                pc = 0x800a14e0;
            }
            0x800a14e0 => {
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x2dc), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1484: bad pc {:#x}", pc),
        }
    }
}

/// __OSGetEffectivePriority (0x800a14ec, 60 bytes)
#[inline(never)]
pub fn f_800a14ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a14ec) { return; }
        if c.prof { crate::prof::enter(c, 0x800a14ec); f_800a14ec_b(c, 0x800a14ec); crate::prof::exit(c, 0x800a14ec); return; }
    }
    f_800a14ec_b(c, 0x800a14ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a14ec_at(c: &mut Cpu, pc: u32) { f_800a14ec_b(c, pc) }
#[inline(always)]
fn f_800a14ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a14ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a14ec, pc); }
        match pc {
            0x800a14ec => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x2d4));
                c.r[5] = c.lwz(c.r[3].wrapping_add(0x2f4));
                pc = 0x800a1518; continue;
                pc = 0x800a14f8;
            }
            0x800a14f8 => {
                c.r[3] = c.lwz(c.r[5]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a1514; continue; }
                pc = 0x800a1504;
            }
            0x800a1504 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x2d0));
                c.cmps(0, c.r[0], c.r[4]);
                if !c.crbit(0) { pc = 0x800a1514; continue; }
                pc = 0x800a1510;
            }
            0x800a1510 => {
                c.r[4] = c.r[0] | c.r[0];
                pc = 0x800a1514;
            }
            0x800a1514 => {
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x10));
                pc = 0x800a1518;
            }
            0x800a1518 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a14f8; continue; }
                pc = 0x800a1520;
            }
            0x800a1520 => {
                c.r[3] = c.r[4] | c.r[4];
                if true { return; }
                return;
            }
            _ => panic!("f_800a14ec: bad pc {:#x}", pc),
        }
    }
}

/// SetEffectivePriority (0x800a1528, 448 bytes)
#[inline(never)]
pub fn f_800a1528(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1528) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1528); f_800a1528_b(c, 0x800a1528); crate::prof::exit(c, 0x800a1528); return; }
    }
    f_800a1528_b(c, 0x800a1528)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1528_at(c: &mut Cpu, pc: u32) { f_800a1528_b(c, pc) }
#[inline(always)]
fn f_800a1528_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1528;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1528, pc); }
        match pc {
            0x800a1528 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x2c8));
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x800a16cc; continue; }
                pc = 0x800a1550;
            }
            0x800a1550 => {
                if !c.crbit(0) { pc = 0x800a1564; continue; }
                pc = 0x800a1554;
            }
            0x800a1554 => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800a1570; continue; }
                pc = 0x800a155c;
            }
            0x800a155c => {
                if !c.crbit(0) { pc = 0x800a16c0; continue; }
                pc = 0x800a1560;
            }
            0x800a1560 => {
                pc = 0x800a16cc; continue;
                pc = 0x800a1564;
            }
            0x800a1564 => {
                c.cmps(0, c.r[0], 0x5);
                if !c.crbit(0) { pc = 0x800a16cc; continue; }
                pc = 0x800a156c;
            }
            0x800a156c => {
                pc = 0x800a15e8; continue;
                pc = 0x800a1570;
            }
            0x800a1570 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a1578; f_800a1484(c);
                pc = 0x800a1578;
            }
            0x800a1578 => {
                c.stw(c.r[31].wrapping_add(0x2d0), c.r[30]);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb898);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x2d0));
                c.r[3] = c.r[3].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.stw(c.r[31].wrapping_add(0x2dc), c.r[0]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x2dc));
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a15ac; continue; }
                pc = 0x800a15a4;
            }
            0x800a15a4 => {
                c.stw(c.r[4], c.r[31]);
                pc = 0x800a15b0; continue;
                pc = 0x800a15ac;
            }
            0x800a15ac => {
                c.stw(c.r[3].wrapping_add(0x2e0), c.r[31]);
                pc = 0x800a15b0;
            }
            0x800a15b0 => {
                c.stw(c.r[31].wrapping_add(0x2e4), c.r[3]);
                c.r[0] = 0x0;
                c.r[3] = 0x1;
                c.stw(c.r[31].wrapping_add(0x2e0), c.r[0]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x2dc));
                c.stw(c.r[4].wrapping_add(0x4), c.r[31]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x2d0));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.r[0] = c.addc(!c.r[0], 0x1f, 1);
                c.r[0] = slw(c.r[3], c.r[0]);
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffff9f40), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[3]);
                pc = 0x800a16cc; continue;
                pc = 0x800a15e8;
            }
            0x800a15e8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x2e0));
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x2e4));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a1604; continue; }
                pc = 0x800a15f8;
            }
            0x800a15f8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x2dc));
                c.stw(c.r[3].wrapping_add(0x4), c.r[5]);
                pc = 0x800a1608; continue;
                pc = 0x800a1604;
            }
            0x800a1604 => {
                c.stw(c.r[4].wrapping_add(0x2e4), c.r[5]);
                pc = 0x800a1608;
            }
            0x800a1608 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a161c; continue; }
                pc = 0x800a1610;
            }
            0x800a1610 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x2dc));
                c.stw(c.r[3], c.r[4]);
                pc = 0x800a1620; continue;
                pc = 0x800a161c;
            }
            0x800a161c => {
                c.stw(c.r[5].wrapping_add(0x2e0), c.r[4]);
                pc = 0x800a1620;
            }
            0x800a1620 => {
                c.stw(c.r[31].wrapping_add(0x2d0), c.r[30]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x2dc));
                c.r[5] = c.lwz(c.r[4]);
                pc = 0x800a1634; continue;
                pc = 0x800a1630;
            }
            0x800a1630 => {
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x2e0));
                pc = 0x800a1634;
            }
            0x800a1634 => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x800a164c; continue; }
                pc = 0x800a163c;
            }
            0x800a163c => {
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x2d0));
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x2d0));
                c.cmps(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x800a1630; continue; }
                pc = 0x800a164c;
            }
            0x800a164c => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a1684; continue; }
                pc = 0x800a1654;
            }
            0x800a1654 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a1668; continue; }
                pc = 0x800a1660;
            }
            0x800a1660 => {
                c.stw(c.r[4], c.r[31]);
                pc = 0x800a166c; continue;
                pc = 0x800a1668;
            }
            0x800a1668 => {
                c.stw(c.r[3].wrapping_add(0x2e0), c.r[31]);
                pc = 0x800a166c;
            }
            0x800a166c => {
                c.stw(c.r[31].wrapping_add(0x2e4), c.r[3]);
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x2e0), c.r[0]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x2dc));
                c.stw(c.r[3].wrapping_add(0x4), c.r[31]);
                pc = 0x800a16ac; continue;
                pc = 0x800a1684;
            }
            0x800a1684 => {
                c.stw(c.r[31].wrapping_add(0x2e0), c.r[5]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x2e4));
                c.stw(c.r[5].wrapping_add(0x2e4), c.r[31]);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[31].wrapping_add(0x2e4), c.r[3]);
                if !c.crbit(2) { pc = 0x800a16a8; continue; }
                pc = 0x800a169c;
            }
            0x800a169c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x2dc));
                c.stw(c.r[3], c.r[31]);
                pc = 0x800a16ac; continue;
                pc = 0x800a16a8;
            }
            0x800a16a8 => {
                c.stw(c.r[3].wrapping_add(0x2e0), c.r[31]);
                pc = 0x800a16ac;
            }
            0x800a16ac => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x2f0));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a16cc; continue; }
                pc = 0x800a16b8;
            }
            0x800a16b8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x800a16d0; continue;
                pc = 0x800a16c0;
            }
            0x800a16c0 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x2d0), c.r[30]);
                pc = 0x800a16cc;
            }
            0x800a16cc => {
                c.r[3] = 0x0;
                pc = 0x800a16d0;
            }
            0x800a16d0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1528: bad pc {:#x}", pc),
        }
    }
}

/// fn_800A16E8 (0x800a16e8, 80 bytes)
#[inline(never)]
pub fn f_800a16e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a16e8) { return; }
        if c.prof { crate::prof::enter(c, 0x800a16e8); f_800a16e8_b(c, 0x800a16e8); crate::prof::exit(c, 0x800a16e8); return; }
    }
    f_800a16e8_b(c, 0x800a16e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a16e8_at(c: &mut Cpu, pc: u32) { f_800a16e8_b(c, pc) }
#[inline(always)]
fn f_800a16e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a16e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a16e8, pc); }
        match pc {
            0x800a16e8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                pc = 0x800a16fc;
            }
            0x800a16fc => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x2cc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(1) { pc = 0x800a1724; continue; }
                pc = 0x800a1708;
            }
            0x800a1708 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x2d0));
                c.cmps(0, c.r[0], c.r[31]);
                if !c.crbit(1) { pc = 0x800a1724; continue; }
                pc = 0x800a1714;
            }
            0x800a1714 => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800a171c; f_800a1528(c);
                pc = 0x800a171c;
            }
            0x800a171c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a16fc; continue; }
                pc = 0x800a1724;
            }
            0x800a1724 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a16e8: bad pc {:#x}", pc),
        }
    }
}

/// SelectThread (0x800a1738, 552 bytes)
#[inline(never)]
pub fn f_800a1738(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1738) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1738); f_800a1738_b(c, 0x800a1738); crate::prof::exit(c, 0x800a1738); return; }
    }
    f_800a1738_b(c, 0x800a1738)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1738_at(c: &mut Cpu, pc: u32) { f_800a1738_b(c, pc) }
#[inline(always)]
fn f_800a1738_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1738;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1738, pc); }
        match pc {
            0x800a1738 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffb898);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f48));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(1) { pc = 0x800a176c; continue; }
                pc = 0x800a1764;
            }
            0x800a1764 => {
                c.r[3] = 0x0;
                pc = 0x800a1948; continue;
                pc = 0x800a176c;
            }
            0x800a176c => {
                c.lr = 0x800a1770; f_8009bbc4(c);
                pc = 0x800a1770;
            }
            0x800a1770 => {
                c.r[4] = 0x80000000;
                c.r[6] = c.lwz(c.r[4].wrapping_add(0xe4));
                c.cmpu(0, c.r[3], c.r[6]);
                c.r[3] = c.r[6].wrapping_add(0x0);
                if c.crbit(2) { pc = 0x800a178c; continue; }
                pc = 0x800a1784;
            }
            0x800a1784 => {
                c.r[3] = 0x0;
                pc = 0x800a1948; continue;
                pc = 0x800a178c;
            }
            0x800a178c => {
                c.cmpu(0, c.r[6], 0x0);
                if c.crbit(2) { pc = 0x800a184c; continue; }
                pc = 0x800a1794;
            }
            0x800a1794 => {
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x2c8));
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x800a182c; continue; }
                pc = 0x800a17a0;
            }
            0x800a17a0 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x800a17c4; continue; }
                pc = 0x800a17a8;
            }
            0x800a17a8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x2d0));
                c.r[4] = c.r[4].leading_zeros();
                c.cmps(0, c.r[0], c.r[4]);
                if c.crbit(1) { pc = 0x800a17c4; continue; }
                pc = 0x800a17bc;
            }
            0x800a17bc => {
                c.r[3] = 0x0;
                pc = 0x800a1948; continue;
                pc = 0x800a17c4;
            }
            0x800a17c4 => {
                c.r[0] = 0x1;
                c.sth(c.r[6].wrapping_add(0x2c8), c.r[0]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x2d0));
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[6].wrapping_add(0x2dc), c.r[0]);
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x2dc));
                c.r[4] = c.lwz(c.r[5].wrapping_add(0x4));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a17f4; continue; }
                pc = 0x800a17ec;
            }
            0x800a17ec => {
                c.stw(c.r[5], c.r[6]);
                pc = 0x800a17f8; continue;
                pc = 0x800a17f4;
            }
            0x800a17f4 => {
                c.stw(c.r[4].wrapping_add(0x2e0), c.r[6]);
                pc = 0x800a17f8;
            }
            0x800a17f8 => {
                c.stw(c.r[6].wrapping_add(0x2e4), c.r[4]);
                c.r[0] = 0x0;
                c.r[4] = 0x1;
                c.stw(c.r[6].wrapping_add(0x2e0), c.r[0]);
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x2dc));
                c.stw(c.r[5].wrapping_add(0x4), c.r[6]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x2d0));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.r[0] = c.addc(!c.r[0], 0x1f, 1);
                c.r[0] = slw(c.r[4], c.r[0]);
                c.r[0] = c.r[5] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffff9f40), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[4]);
                pc = 0x800a182c;
            }
            0x800a182c => {
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x1a2));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a184c; continue; }
                pc = 0x800a1838;
            }
            0x800a1838 => {
                c.lr = 0x800a183c; f_8009bbd0(c);
                pc = 0x800a183c;
            }
            0x800a183c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a184c; continue; }
                pc = 0x800a1844;
            }
            0x800a1844 => {
                c.r[3] = 0x0;
                pc = 0x800a1948; continue;
                pc = 0x800a184c;
            }
            0x800a184c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a18a8; continue; }
                pc = 0x800a1858;
            }
            0x800a1858 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8188));
                c.r[30] = 0x80000000;
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xe4));
                c.r[4] = 0x0;
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a1870; (c.dispatch)(c, t); }
                pc = 0x800a1870;
            }
            0x800a1870 => {
                c.r[0] = 0x0;
                c.stw(c.r[30].wrapping_add(0xe4), c.r[0]);
                c.r[3] = c.r[31].wrapping_add(0x730);
                c.lr = 0x800a1880; f_8009bb68(c);
                pc = 0x800a1880;
            }
            0x800a1880 => {
                c.lr = 0x800a1884; f_8009df50(c);
                pc = 0x800a1884;
            }
            0x800a1884 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a1884; continue; }
                pc = 0x800a1890;
            }
            0x800a1890 => {
                c.lr = 0x800a1894; f_8009df3c(c);
                pc = 0x800a1894;
            }
            0x800a1894 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a1880; continue; }
                pc = 0x800a18a0;
            }
            0x800a18a0 => {
                c.r[3] = c.r[31].wrapping_add(0x730);
                c.lr = 0x800a18a8; f_8009bd60(c);
                pc = 0x800a18a8;
            }
            0x800a18a8 => {
                c.r[3] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.r[7] = c.r[0].leading_zeros();
                c.r[0] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[4] = c.r[31].wrapping_add(c.r[0]);
                c.r[5] = c.lwz(c.r[4]);
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x2e0));
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x800a18dc; continue; }
                pc = 0x800a18d4;
            }
            0x800a18d4 => {
                c.stw(c.r[4].wrapping_add(0x4), c.r[3]);
                pc = 0x800a18e0; continue;
                pc = 0x800a18dc;
            }
            0x800a18dc => {
                c.stw(c.r[6].wrapping_add(0x2e4), c.r[3]);
                pc = 0x800a18e0;
            }
            0x800a18e0 => {
                c.stw(c.r[4], c.r[6]);
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a1908; continue; }
                pc = 0x800a18f0;
            }
            0x800a18f0 => {
                c.r[0] = c.addc(!c.r[7], 0x1f, 1);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.r[3] = 0x1;
                c.r[0] = slw(c.r[3], c.r[0]);
                c.r[0] = c.r[4] & !c.r[0];
                c.stw(c.r[13].wrapping_add(0xffff9f40), c.r[0]);
                pc = 0x800a1908;
            }
            0x800a1908 => {
                c.r[0] = 0x0;
                c.stw(c.r[30].wrapping_add(0x2dc), c.r[0]);
                c.r[0] = 0x2;
                c.r[31] = 0x80000000;
                c.sth(c.r[30].wrapping_add(0x2c8), c.r[0]);
                c.r[4] = c.r[30] | c.r[30];
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8188));
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xe4));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800a1930; (c.dispatch)(c, t); }
                pc = 0x800a1930;
            }
            0x800a1930 => {
                c.stw(c.r[31].wrapping_add(0xe4), c.r[30]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a193c; f_8009bb68(c);
                pc = 0x800a193c;
            }
            0x800a193c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a1944; f_8009bc50(c);
                pc = 0x800a1944;
            }
            0x800a1944 => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800a1948;
            }
            0x800a1948 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1738: bad pc {:#x}", pc),
        }
    }
}

/// __OSReschedule (0x800a1960, 48 bytes)
#[inline(never)]
pub fn f_800a1960(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1960) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1960); f_800a1960_b(c, 0x800a1960); crate::prof::exit(c, 0x800a1960); return; }
    }
    f_800a1960_b(c, 0x800a1960)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1960_at(c: &mut Cpu, pc: u32) { f_800a1960_b(c, pc) }
#[inline(always)]
fn f_800a1960_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1960;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1960, pc); }
        match pc {
            0x800a1960 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a1980; continue; }
                pc = 0x800a1978;
            }
            0x800a1978 => {
                c.r[3] = 0x0;
                c.lr = 0x800a1980; f_800a1738(c);
                pc = 0x800a1980;
            }
            0x800a1980 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1960: bad pc {:#x}", pc),
        }
    }
}

/// OSYieldThread (0x800a1990, 60 bytes)
#[inline(never)]
pub fn f_800a1990(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1990) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1990); f_800a1990_b(c, 0x800a1990); crate::prof::exit(c, 0x800a1990); return; }
    }
    f_800a1990_b(c, 0x800a1990)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1990_at(c: &mut Cpu, pc: u32) { f_800a1990_b(c, pc) }
#[inline(always)]
fn f_800a1990_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1990;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1990, pc); }
        match pc {
            0x800a1990 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x800a19a4; f_8009df3c(c);
                pc = 0x800a19a4;
            }
            0x800a19a4 => {
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x1;
                c.lr = 0x800a19b0; f_800a1738(c);
                pc = 0x800a19b0;
            }
            0x800a19b0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a19b8; f_8009df64(c);
                pc = 0x800a19b8;
            }
            0x800a19b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1990: bad pc {:#x}", pc),
        }
    }
}

/// OSCreateThread (0x800a19cc, 488 bytes)
#[inline(never)]
pub fn f_800a19cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a19cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a19cc); f_800a19cc_b(c, 0x800a19cc); crate::prof::exit(c, 0x800a19cc); return; }
    }
    f_800a19cc_b(c, 0x800a19cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a19cc_at(c: &mut Cpu, pc: u32) { f_800a19cc_b(c, pc) }
#[inline(always)]
fn f_800a19cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a19cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a19cc, pc); }
        match pc {
            0x800a19cc => {
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[8], 0x0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[27] = c.r[5].wrapping_add(0x0);
                c.r[28] = c.r[6].wrapping_add(0x0);
                c.r[29] = c.r[7].wrapping_add(0x0);
                if c.crbit(0) { pc = 0x800a19fc; continue; }
                pc = 0x800a19f4;
            }
            0x800a19f4 => {
                c.cmps(0, c.r[8], 0x1f);
                if !c.crbit(1) { pc = 0x800a1a04; continue; }
                pc = 0x800a19fc;
            }
            0x800a19fc => {
                c.r[3] = 0x0;
                pc = 0x800a1ba0; continue;
                pc = 0x800a1a04;
            }
            0x800a1a04 => {
                c.r[6] = 0x1;
                c.sth(c.r[31].wrapping_add(0x2c8), c.r[6]);
                c.r[0] = c.r[9].rotate_left(0) & 0x1;
                c.r[7] = c.r[28].rotate_left(0) & 0xfffffff8;
                c.sth(c.r[31].wrapping_add(0x2ca), c.r[0]);
                c.r[0] = 0xffffffff;
                c.r[30] = 0x0;
                c.stw(c.r[31].wrapping_add(0x2d4), c.r[8]);
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[5] = c.r[7].wrapping_add(0xfffffff8);
                c.stw(c.r[31].wrapping_add(0x2d0), c.r[8]);
                c.stw(c.r[31].wrapping_add(0x2cc), c.r[6]);
                c.stw(c.r[31].wrapping_add(0x2d8), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x2f0), c.r[30]);
                c.stw(c.r[31].wrapping_add(0x2ec), c.r[30]);
                c.stw(c.r[31].wrapping_add(0x2e8), c.r[30]);
                c.stw(c.r[31].wrapping_add(0x2f8), c.r[30]);
                c.stw(c.r[31].wrapping_add(0x2f4), c.r[30]);
                c.stw(c.r[7].wrapping_add(0xfffffff8), c.r[30]);
                c.stw(c.r[7].wrapping_add(0xfffffffc), c.r[30]);
                c.lr = 0x800a1a58; f_8009bd84(c);
                pc = 0x800a1a58;
            }
            0x800a1a58 => {
                c.r[3] = 0x800a0000;
                c.r[0] = c.r[3].wrapping_add(0x1bb4);
                c.stw(c.r[31].wrapping_add(0x84), c.r[0]);
                c.r[3] = 0xdeae0000;
                c.r[4] = c.r[28].wrapping_sub(c.r[29]);
                c.stw(c.r[31].wrapping_add(0xc), c.r[27]);
                c.r[0] = c.r[3].wrapping_add(0xffffbabe);
                c.stw(c.r[31].wrapping_add(0x304), c.r[28]);
                c.stw(c.r[31].wrapping_add(0x308), c.r[4]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x308));
                c.stw(c.r[3], c.r[0]);
                c.stw(c.r[31].wrapping_add(0x30c), c.r[30]);
                c.stw(c.r[31].wrapping_add(0x310), c.r[30]);
                c.stw(c.r[31].wrapping_add(0x314), c.r[30]);
                c.lr = 0x800a1a94; f_8009df3c(c);
                pc = 0x800a1a94;
            }
            0x800a1a94 => {
                c.r[4] = 0x80400000;
                c.r[4] = c.r[4].wrapping_add(0xffffb4f0);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x40));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a1b68; continue; }
                pc = 0x800a1aa8;
            }
            0x800a1aa8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x19c));
                c.r[0] = 0x4;
                c.mtspr(9, c.r[0]);
                c.r[5] = c.r[31].wrapping_add(0x0);
                c.r[0] = c.r[4] | 0x900;
                c.stw(c.r[31].wrapping_add(0x19c), c.r[0]);
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x1a2));
                c.r[0] = c.r[0] | 0x1;
                c.sth(c.r[31].wrapping_add(0x1a2), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8170));
                c.r[0] = c.r[0].rotate_left(0) & 0xf8;
                c.r[0] = c.r[0] | 0x4;
                c.stw(c.r[31].wrapping_add(0x194), c.r[0]);
                pc = 0x800a1adc;
            }
            0x800a1adc => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[5].wrapping_add(0x94), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x90), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1cc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1c8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x9c), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x98), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1d4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1d0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xa4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xa0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1dc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1d8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xac), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xa8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1e4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1e0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xb4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xb0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1ec), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1e8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xbc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xb8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1f4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1f0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xc4), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xc0), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1fc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x1f8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xcc), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xc8), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x204), c.r[0]);
                c.stw(c.r[5].wrapping_add(0x200), c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0x40);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800a1adc; continue; }
                pc = 0x800a1b68;
            }
            0x800a1b68 => {
                c.r[4] = 0x80000000;
                c.r[5] = c.r[4].wrapping_add(0xdc);
                { let ea = c.r[5].wrapping_add(0x4); c.r[6] = c.lwz(ea); c.r[5] = ea; }
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x800a1b84; continue; }
                pc = 0x800a1b7c;
            }
            0x800a1b7c => {
                c.stw(c.r[4].wrapping_add(0xdc), c.r[31]);
                pc = 0x800a1b88; continue;
                pc = 0x800a1b84;
            }
            0x800a1b84 => {
                c.stw(c.r[6].wrapping_add(0x2fc), c.r[31]);
                pc = 0x800a1b88;
            }
            0x800a1b88 => {
                c.stw(c.r[31].wrapping_add(0x300), c.r[6]);
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x2fc), c.r[0]);
                c.stw(c.r[5], c.r[31]);
                c.lr = 0x800a1b9c; f_8009df64(c);
                pc = 0x800a1b9c;
            }
            0x800a1b9c => {
                c.r[3] = 0x1;
                pc = 0x800a1ba0;
            }
            0x800a1ba0 => {
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[1] = c.r[1].wrapping_add(0x50);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a19cc: bad pc {:#x}", pc),
        }
    }
}

/// OSExitThread (0x800a1bb4, 228 bytes)
#[inline(never)]
pub fn f_800a1bb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1bb4) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1bb4); f_800a1bb4_b(c, 0x800a1bb4); crate::prof::exit(c, 0x800a1bb4); return; }
    }
    f_800a1bb4_b(c, 0x800a1bb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1bb4_at(c: &mut Cpu, pc: u32) { f_800a1bb4_b(c, pc) }
#[inline(always)]
fn f_800a1bb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1bb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1bb4, pc); }
        match pc {
            0x800a1bb4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x800a1bd8; f_8009df3c(c);
                pc = 0x800a1bd8;
            }
            0x800a1bd8 => {
                c.r[31] = 0x80000000;
                c.r[30] = c.lwz(c.r[31].wrapping_add(0xe4));
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.lr = 0x800a1bec; f_8009bd60(c);
                pc = 0x800a1bec;
            }
            0x800a1bec => {
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x2ca));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a1c38; continue; }
                pc = 0x800a1bf8;
            }
            0x800a1bf8 => {
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x2fc));
                c.r[5] = c.lwz(c.r[30].wrapping_add(0x300));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a1c10; continue; }
                pc = 0x800a1c08;
            }
            0x800a1c08 => {
                c.stw(c.r[31].wrapping_add(0xe0), c.r[5]);
                pc = 0x800a1c14; continue;
                pc = 0x800a1c10;
            }
            0x800a1c10 => {
                c.stw(c.r[4].wrapping_add(0x300), c.r[5]);
                pc = 0x800a1c14;
            }
            0x800a1c14 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a1c28; continue; }
                pc = 0x800a1c1c;
            }
            0x800a1c1c => {
                c.r[3] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0xdc), c.r[4]);
                pc = 0x800a1c2c; continue;
                pc = 0x800a1c28;
            }
            0x800a1c28 => {
                c.stw(c.r[5].wrapping_add(0x2fc), c.r[4]);
                pc = 0x800a1c2c;
            }
            0x800a1c2c => {
                c.r[0] = 0x0;
                c.sth(c.r[30].wrapping_add(0x2c8), c.r[0]);
                pc = 0x800a1c44; continue;
                pc = 0x800a1c38;
            }
            0x800a1c38 => {
                c.r[0] = 0x8;
                c.sth(c.r[30].wrapping_add(0x2c8), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x2d8), c.r[28]);
                pc = 0x800a1c44;
            }
            0x800a1c44 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a1c4c; f_8009f958(c);
                pc = 0x800a1c4c;
            }
            0x800a1c4c => {
                c.r[3] = c.r[30].wrapping_add(0x2e8);
                c.lr = 0x800a1c54; f_800a2478(c);
                pc = 0x800a1c54;
            }
            0x800a1c54 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a1c70; continue; }
                pc = 0x800a1c68;
            }
            0x800a1c68 => {
                c.r[3] = 0x0;
                c.lr = 0x800a1c70; f_800a1738(c);
                pc = 0x800a1c70;
            }
            0x800a1c70 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a1c78; f_8009df64(c);
                pc = 0x800a1c78;
            }
            0x800a1c78 => {
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
            _ => panic!("f_800a1bb4: bad pc {:#x}", pc),
        }
    }
}

/// OSCancelThread (0x800a1c98, 444 bytes)
#[inline(never)]
pub fn f_800a1c98(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1c98) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1c98); f_800a1c98_b(c, 0x800a1c98); crate::prof::exit(c, 0x800a1c98); return; }
    }
    f_800a1c98_b(c, 0x800a1c98)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1c98_at(c: &mut Cpu, pc: u32) { f_800a1c98_b(c, pc) }
#[inline(always)]
fn f_800a1c98_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1c98;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1c98, pc); }
        match pc {
            0x800a1c98 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.lr = 0x800a1cb8; f_8009df3c(c);
                pc = 0x800a1cb8;
            }
            0x800a1cb8 => {
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x2c8));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x800a1da0; continue; }
                pc = 0x800a1cc8;
            }
            0x800a1cc8 => {
                if !c.crbit(0) { pc = 0x800a1cdc; continue; }
                pc = 0x800a1ccc;
            }
            0x800a1ccc => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800a1ce8; continue; }
                pc = 0x800a1cd4;
            }
            0x800a1cd4 => {
                if !c.crbit(0) { pc = 0x800a1d00; continue; }
                pc = 0x800a1cd8;
            }
            0x800a1cd8 => {
                pc = 0x800a1da0; continue;
                pc = 0x800a1cdc;
            }
            0x800a1cdc => {
                c.cmps(0, c.r[0], 0x5);
                if !c.crbit(0) { pc = 0x800a1da0; continue; }
                pc = 0x800a1ce4;
            }
            0x800a1ce4 => {
                pc = 0x800a1d0c; continue;
                pc = 0x800a1ce8;
            }
            0x800a1ce8 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x2cc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(1) { pc = 0x800a1dac; continue; }
                pc = 0x800a1cf4;
            }
            0x800a1cf4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a1cfc; f_800a1484(c);
                pc = 0x800a1cfc;
            }
            0x800a1cfc => {
                pc = 0x800a1dac; continue;
                pc = 0x800a1d00;
            }
            0x800a1d00 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[0]);
                pc = 0x800a1dac; continue;
                pc = 0x800a1d0c;
            }
            0x800a1d0c => {
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x2e0));
                c.r[5] = c.lwz(c.r[30].wrapping_add(0x2e4));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a1d28; continue; }
                pc = 0x800a1d1c;
            }
            0x800a1d1c => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x2dc));
                c.stw(c.r[3].wrapping_add(0x4), c.r[5]);
                pc = 0x800a1d2c; continue;
                pc = 0x800a1d28;
            }
            0x800a1d28 => {
                c.stw(c.r[4].wrapping_add(0x2e4), c.r[5]);
                pc = 0x800a1d2c;
            }
            0x800a1d2c => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a1d40; continue; }
                pc = 0x800a1d34;
            }
            0x800a1d34 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x2dc));
                c.stw(c.r[3], c.r[4]);
                pc = 0x800a1d44; continue;
                pc = 0x800a1d40;
            }
            0x800a1d40 => {
                c.stw(c.r[5].wrapping_add(0x2e0), c.r[4]);
                pc = 0x800a1d44;
            }
            0x800a1d44 => {
                c.r[0] = 0x0;
                c.stw(c.r[30].wrapping_add(0x2dc), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x2cc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(1) { pc = 0x800a1dac; continue; }
                pc = 0x800a1d58;
            }
            0x800a1d58 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x2f0));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a1dac; continue; }
                pc = 0x800a1d64;
            }
            0x800a1d64 => {
                c.r[29] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x800a1d68;
            }
            0x800a1d68 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2cc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(1) { pc = 0x800a1dac; continue; }
                pc = 0x800a1d74;
            }
            0x800a1d74 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a1d7c; f_800a14ec(c);
                pc = 0x800a1d7c;
            }
            0x800a1d7c => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2d0));
                c.r[4] = c.r[3].wrapping_add(0x0);
                c.cmps(0, c.r[0], c.r[4]);
                if c.crbit(2) { pc = 0x800a1dac; continue; }
                pc = 0x800a1d8c;
            }
            0x800a1d8c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a1d94; f_800a1528(c);
                pc = 0x800a1d94;
            }
            0x800a1d94 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if !c.crbit(2) { pc = 0x800a1d68; continue; }
                pc = 0x800a1d9c;
            }
            0x800a1d9c => {
                pc = 0x800a1dac; continue;
                pc = 0x800a1da0;
            }
            0x800a1da0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a1da8; f_8009df64(c);
                pc = 0x800a1da8;
            }
            0x800a1da8 => {
                pc = 0x800a1e38; continue;
                pc = 0x800a1dac;
            }
            0x800a1dac => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a1db4; f_8009bd60(c);
                pc = 0x800a1db4;
            }
            0x800a1db4 => {
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x2ca));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800a1e04; continue; }
                pc = 0x800a1dc0;
            }
            0x800a1dc0 => {
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x2fc));
                c.r[5] = c.lwz(c.r[30].wrapping_add(0x300));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a1ddc; continue; }
                pc = 0x800a1dd0;
            }
            0x800a1dd0 => {
                c.r[3] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0xe0), c.r[5]);
                pc = 0x800a1de0; continue;
                pc = 0x800a1ddc;
            }
            0x800a1ddc => {
                c.stw(c.r[4].wrapping_add(0x300), c.r[5]);
                pc = 0x800a1de0;
            }
            0x800a1de0 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a1df4; continue; }
                pc = 0x800a1de8;
            }
            0x800a1de8 => {
                c.r[3] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0xdc), c.r[4]);
                pc = 0x800a1df8; continue;
                pc = 0x800a1df4;
            }
            0x800a1df4 => {
                c.stw(c.r[5].wrapping_add(0x2fc), c.r[4]);
                pc = 0x800a1df8;
            }
            0x800a1df8 => {
                c.r[0] = 0x0;
                c.sth(c.r[30].wrapping_add(0x2c8), c.r[0]);
                pc = 0x800a1e0c; continue;
                pc = 0x800a1e04;
            }
            0x800a1e04 => {
                c.r[0] = 0x8;
                c.sth(c.r[30].wrapping_add(0x2c8), c.r[0]);
                pc = 0x800a1e0c;
            }
            0x800a1e0c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a1e14; f_8009f958(c);
                pc = 0x800a1e14;
            }
            0x800a1e14 => {
                c.r[3] = c.r[30].wrapping_add(0x2e8);
                c.lr = 0x800a1e1c; f_800a2478(c);
                pc = 0x800a1e1c;
            }
            0x800a1e1c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a1e30; continue; }
                pc = 0x800a1e28;
            }
            0x800a1e28 => {
                c.r[3] = 0x0;
                c.lr = 0x800a1e30; f_800a1738(c);
                pc = 0x800a1e30;
            }
            0x800a1e30 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a1e38; f_8009df64(c);
                pc = 0x800a1e38;
            }
            0x800a1e38 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1c98: bad pc {:#x}", pc),
        }
    }
}

/// fn_800A1E54 (0x800a1e54, 320 bytes)
#[inline(never)]
pub fn f_800a1e54(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1e54) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1e54); f_800a1e54_b(c, 0x800a1e54); crate::prof::exit(c, 0x800a1e54); return; }
    }
    f_800a1e54_b(c, 0x800a1e54)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1e54_at(c: &mut Cpu, pc: u32) { f_800a1e54_b(c, pc) }
#[inline(always)]
fn f_800a1e54_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1e54;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1e54, pc); }
        match pc {
            0x800a1e54 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4].wrapping_add(0x0);
                c.lr = 0x800a1e78; f_8009df3c(c);
                pc = 0x800a1e78;
            }
            0x800a1e78 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x2ca));
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800a1f00; continue; }
                pc = 0x800a1e88;
            }
            0x800a1e88 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x2c8));
                c.cmpu(0, c.r[0], 0x8);
                if c.crbit(2) { pc = 0x800a1f00; continue; }
                pc = 0x800a1e94;
            }
            0x800a1e94 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x2e8));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a1f00; continue; }
                pc = 0x800a1ea0;
            }
            0x800a1ea0 => {
                c.r[3] = c.r[31].wrapping_add(0x2e8);
                c.lr = 0x800a1ea8; f_800a238c(c);
                pc = 0x800a1ea8;
            }
            0x800a1ea8 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x2c8));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a1ebc; continue; }
                pc = 0x800a1eb4;
            }
            0x800a1eb4 => {
                c.r[0] = 0x0;
                pc = 0x800a1ee8; continue;
                pc = 0x800a1ebc;
            }
            0x800a1ebc => {
                c.r[3] = 0x80000000;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xdc));
                pc = 0x800a1edc; continue;
                pc = 0x800a1ec8;
            }
            0x800a1ec8 => {
                c.cmpu(0, c.r[31], c.r[3]);
                if !c.crbit(2) { pc = 0x800a1ed8; continue; }
                pc = 0x800a1ed0;
            }
            0x800a1ed0 => {
                c.r[0] = 0x1;
                pc = 0x800a1ee8; continue;
                pc = 0x800a1ed8;
            }
            0x800a1ed8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x2fc));
                pc = 0x800a1edc;
            }
            0x800a1edc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a1ec8; continue; }
                pc = 0x800a1ee4;
            }
            0x800a1ee4 => {
                c.r[0] = 0x0;
                pc = 0x800a1ee8;
            }
            0x800a1ee8 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a1f00; continue; }
                pc = 0x800a1ef0;
            }
            0x800a1ef0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a1ef8; f_8009df64(c);
                pc = 0x800a1ef8;
            }
            0x800a1ef8 => {
                c.r[3] = 0x0;
                pc = 0x800a1f78; continue;
                pc = 0x800a1f00;
            }
            0x800a1f00 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x2c8));
                c.cmpu(0, c.r[0], 0x8);
                if !c.crbit(2) { pc = 0x800a1f6c; continue; }
                pc = 0x800a1f0c;
            }
            0x800a1f0c => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x800a1f1c; continue; }
                pc = 0x800a1f14;
            }
            0x800a1f14 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x2d8));
                c.stw(c.r[29], c.r[0]);
                pc = 0x800a1f1c;
            }
            0x800a1f1c => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x2fc));
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x300));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a1f38; continue; }
                pc = 0x800a1f2c;
            }
            0x800a1f2c => {
                c.r[3] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0xe0), c.r[5]);
                pc = 0x800a1f3c; continue;
                pc = 0x800a1f38;
            }
            0x800a1f38 => {
                c.stw(c.r[4].wrapping_add(0x300), c.r[5]);
                pc = 0x800a1f3c;
            }
            0x800a1f3c => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a1f50; continue; }
                pc = 0x800a1f44;
            }
            0x800a1f44 => {
                c.r[3] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0xdc), c.r[4]);
                pc = 0x800a1f54; continue;
                pc = 0x800a1f50;
            }
            0x800a1f50 => {
                c.stw(c.r[5].wrapping_add(0x2fc), c.r[4]);
                pc = 0x800a1f54;
            }
            0x800a1f54 => {
                c.r[0] = 0x0;
                c.sth(c.r[31].wrapping_add(0x2c8), c.r[0]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a1f64; f_8009df64(c);
                pc = 0x800a1f64;
            }
            0x800a1f64 => {
                c.r[3] = 0x1;
                pc = 0x800a1f78; continue;
                pc = 0x800a1f6c;
            }
            0x800a1f6c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a1f74; f_8009df64(c);
                pc = 0x800a1f74;
            }
            0x800a1f74 => {
                c.r[3] = 0x0;
                pc = 0x800a1f78;
            }
            0x800a1f78 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1e54: bad pc {:#x}", pc),
        }
    }
}

/// OSResumeThread (0x800a1f94, 648 bytes)
#[inline(never)]
pub fn f_800a1f94(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a1f94) { return; }
        if c.prof { crate::prof::enter(c, 0x800a1f94); f_800a1f94_b(c, 0x800a1f94); crate::prof::exit(c, 0x800a1f94); return; }
    }
    f_800a1f94_b(c, 0x800a1f94)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a1f94_at(c: &mut Cpu, pc: u32) { f_800a1f94_b(c, pc) }
#[inline(always)]
fn f_800a1f94_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a1f94;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a1f94, pc); }
        match pc {
            0x800a1f94 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x800a1fb4; f_8009df3c(c);
                pc = 0x800a1fb4;
            }
            0x800a1fb4 => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x2cc));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                c.stw(c.r[29].wrapping_add(0x2cc), c.r[0]);
                c.r[30] = c.r[4] | c.r[4];
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2cc));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x800a1fe0; continue; }
                pc = 0x800a1fd4;
            }
            0x800a1fd4 => {
                c.r[0] = 0x0;
                c.stw(c.r[29].wrapping_add(0x2cc), c.r[0]);
                pc = 0x800a21f4; continue;
                pc = 0x800a1fe0;
            }
            0x800a1fe0 => {
                if !c.crbit(2) { pc = 0x800a21f4; continue; }
                pc = 0x800a1fe4;
            }
            0x800a1fe4 => {
                c.r[0] = c.lhz(c.r[29].wrapping_add(0x2c8));
                c.cmps(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x800a20a4; continue; }
                pc = 0x800a1ff0;
            }
            0x800a1ff0 => {
                if !c.crbit(0) { pc = 0x800a21e0; continue; }
                pc = 0x800a1ff4;
            }
            0x800a1ff4 => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800a2000; continue; }
                pc = 0x800a1ffc;
            }
            0x800a1ffc => {
                pc = 0x800a21e0; continue;
                pc = 0x800a2000;
            }
            0x800a2000 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2d4));
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2f4));
                pc = 0x800a202c; continue;
                pc = 0x800a200c;
            }
            0x800a200c => {
                c.r[4] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800a2028; continue; }
                pc = 0x800a2018;
            }
            0x800a2018 => {
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x2d0));
                c.cmps(0, c.r[4], c.r[0]);
                if !c.crbit(0) { pc = 0x800a2028; continue; }
                pc = 0x800a2024;
            }
            0x800a2024 => {
                c.r[0] = c.r[4] | c.r[4];
                pc = 0x800a2028;
            }
            0x800a2028 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                pc = 0x800a202c;
            }
            0x800a202c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a200c; continue; }
                pc = 0x800a2034;
            }
            0x800a2034 => {
                c.stw(c.r[29].wrapping_add(0x2d0), c.r[0]);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffb898);
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2d0));
                c.r[3] = c.r[3].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.stw(c.r[29].wrapping_add(0x2dc), c.r[0]);
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a2068; continue; }
                pc = 0x800a2060;
            }
            0x800a2060 => {
                c.stw(c.r[4], c.r[29]);
                pc = 0x800a206c; continue;
                pc = 0x800a2068;
            }
            0x800a2068 => {
                c.stw(c.r[3].wrapping_add(0x2e0), c.r[29]);
                pc = 0x800a206c;
            }
            0x800a206c => {
                c.stw(c.r[29].wrapping_add(0x2e4), c.r[3]);
                c.r[0] = 0x0;
                c.r[3] = 0x1;
                c.stw(c.r[29].wrapping_add(0x2e0), c.r[0]);
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.stw(c.r[4].wrapping_add(0x4), c.r[29]);
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2d0));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.r[0] = c.addc(!c.r[0], 0x1f, 1);
                c.r[0] = slw(c.r[3], c.r[0]);
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffff9f40), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[3]);
                pc = 0x800a21e0; continue;
                pc = 0x800a20a4;
            }
            0x800a20a4 => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x2e0));
                c.r[5] = c.lwz(c.r[29].wrapping_add(0x2e4));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a20c0; continue; }
                pc = 0x800a20b4;
            }
            0x800a20b4 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.stw(c.r[3].wrapping_add(0x4), c.r[5]);
                pc = 0x800a20c4; continue;
                pc = 0x800a20c0;
            }
            0x800a20c0 => {
                c.stw(c.r[4].wrapping_add(0x2e4), c.r[5]);
                pc = 0x800a20c4;
            }
            0x800a20c4 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a20d8; continue; }
                pc = 0x800a20cc;
            }
            0x800a20cc => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.stw(c.r[3], c.r[4]);
                pc = 0x800a20dc; continue;
                pc = 0x800a20d8;
            }
            0x800a20d8 => {
                c.stw(c.r[5].wrapping_add(0x2e0), c.r[4]);
                pc = 0x800a20dc;
            }
            0x800a20dc => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2d4));
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2f4));
                pc = 0x800a2108; continue;
                pc = 0x800a20e8;
            }
            0x800a20e8 => {
                c.r[4] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800a2104; continue; }
                pc = 0x800a20f4;
            }
            0x800a20f4 => {
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x2d0));
                c.cmps(0, c.r[4], c.r[0]);
                if !c.crbit(0) { pc = 0x800a2104; continue; }
                pc = 0x800a2100;
            }
            0x800a2100 => {
                c.r[0] = c.r[4] | c.r[4];
                pc = 0x800a2104;
            }
            0x800a2104 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                pc = 0x800a2108;
            }
            0x800a2108 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a20e8; continue; }
                pc = 0x800a2110;
            }
            0x800a2110 => {
                c.stw(c.r[29].wrapping_add(0x2d0), c.r[0]);
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.r[5] = c.lwz(c.r[4]);
                pc = 0x800a2124; continue;
                pc = 0x800a2120;
            }
            0x800a2120 => {
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x2e0));
                pc = 0x800a2124;
            }
            0x800a2124 => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x800a213c; continue; }
                pc = 0x800a212c;
            }
            0x800a212c => {
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x2d0));
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2d0));
                c.cmps(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x800a2120; continue; }
                pc = 0x800a213c;
            }
            0x800a213c => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a2174; continue; }
                pc = 0x800a2144;
            }
            0x800a2144 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a2158; continue; }
                pc = 0x800a2150;
            }
            0x800a2150 => {
                c.stw(c.r[4], c.r[29]);
                pc = 0x800a215c; continue;
                pc = 0x800a2158;
            }
            0x800a2158 => {
                c.stw(c.r[3].wrapping_add(0x2e0), c.r[29]);
                pc = 0x800a215c;
            }
            0x800a215c => {
                c.stw(c.r[29].wrapping_add(0x2e4), c.r[3]);
                c.r[0] = 0x0;
                c.stw(c.r[29].wrapping_add(0x2e0), c.r[0]);
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.stw(c.r[3].wrapping_add(0x4), c.r[29]);
                pc = 0x800a219c; continue;
                pc = 0x800a2174;
            }
            0x800a2174 => {
                c.stw(c.r[29].wrapping_add(0x2e0), c.r[5]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x2e4));
                c.stw(c.r[5].wrapping_add(0x2e4), c.r[29]);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[29].wrapping_add(0x2e4), c.r[3]);
                if !c.crbit(2) { pc = 0x800a2198; continue; }
                pc = 0x800a218c;
            }
            0x800a218c => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.stw(c.r[3], c.r[29]);
                pc = 0x800a219c; continue;
                pc = 0x800a2198;
            }
            0x800a2198 => {
                c.stw(c.r[3].wrapping_add(0x2e0), c.r[29]);
                pc = 0x800a219c;
            }
            0x800a219c => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2f0));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a21e0; continue; }
                pc = 0x800a21a8;
            }
            0x800a21a8 => {
                c.r[29] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x800a21ac;
            }
            0x800a21ac => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2cc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(1) { pc = 0x800a21e0; continue; }
                pc = 0x800a21b8;
            }
            0x800a21b8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a21c0; f_800a14ec(c);
                pc = 0x800a21c0;
            }
            0x800a21c0 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2d0));
                c.r[4] = c.r[3].wrapping_add(0x0);
                c.cmps(0, c.r[0], c.r[4]);
                if c.crbit(2) { pc = 0x800a21e0; continue; }
                pc = 0x800a21d0;
            }
            0x800a21d0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a21d8; f_800a1528(c);
                pc = 0x800a21d8;
            }
            0x800a21d8 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if !c.crbit(2) { pc = 0x800a21ac; continue; }
                pc = 0x800a21e0;
            }
            0x800a21e0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a21f4; continue; }
                pc = 0x800a21ec;
            }
            0x800a21ec => {
                c.r[3] = 0x0;
                c.lr = 0x800a21f4; f_800a1738(c);
                pc = 0x800a21f4;
            }
            0x800a21f4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a21fc; f_8009df64(c);
                pc = 0x800a21fc;
            }
            0x800a21fc => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a1f94: bad pc {:#x}", pc),
        }
    }
}

/// OSSuspendThread (0x800a221c, 368 bytes)
#[inline(never)]
pub fn f_800a221c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a221c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a221c); f_800a221c_b(c, 0x800a221c); crate::prof::exit(c, 0x800a221c); return; }
    }
    f_800a221c_b(c, 0x800a221c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a221c_at(c: &mut Cpu, pc: u32) { f_800a221c_b(c, pc) }
#[inline(always)]
fn f_800a221c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a221c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a221c, pc); }
        match pc {
            0x800a221c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x800a223c; f_8009df3c(c);
                pc = 0x800a223c;
            }
            0x800a223c => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x2cc));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.r[30] = c.r[4] | c.r[4];
                c.rc(c.r[30]);
                c.stw(c.r[29].wrapping_add(0x2cc), c.r[0]);
                if !c.crbit(2) { pc = 0x800a2364; continue; }
                pc = 0x800a2254;
            }
            0x800a2254 => {
                c.r[0] = c.lhz(c.r[29].wrapping_add(0x2c8));
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x800a2350; continue; }
                pc = 0x800a2260;
            }
            0x800a2260 => {
                if !c.crbit(0) { pc = 0x800a2274; continue; }
                pc = 0x800a2264;
            }
            0x800a2264 => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800a2290; continue; }
                pc = 0x800a226c;
            }
            0x800a226c => {
                if !c.crbit(0) { pc = 0x800a2280; continue; }
                pc = 0x800a2270;
            }
            0x800a2270 => {
                pc = 0x800a2350; continue;
                pc = 0x800a2274;
            }
            0x800a2274 => {
                c.cmps(0, c.r[0], 0x5);
                if !c.crbit(0) { pc = 0x800a2350; continue; }
                pc = 0x800a227c;
            }
            0x800a227c => {
                pc = 0x800a229c; continue;
                pc = 0x800a2280;
            }
            0x800a2280 => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[0]);
                c.sth(c.r[29].wrapping_add(0x2c8), c.r[0]);
                pc = 0x800a2350; continue;
                pc = 0x800a2290;
            }
            0x800a2290 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a2298; f_800a1484(c);
                pc = 0x800a2298;
            }
            0x800a2298 => {
                pc = 0x800a2350; continue;
                pc = 0x800a229c;
            }
            0x800a229c => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x2e0));
                c.r[5] = c.lwz(c.r[29].wrapping_add(0x2e4));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800a22b8; continue; }
                pc = 0x800a22ac;
            }
            0x800a22ac => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.stw(c.r[3].wrapping_add(0x4), c.r[5]);
                pc = 0x800a22bc; continue;
                pc = 0x800a22b8;
            }
            0x800a22b8 => {
                c.stw(c.r[4].wrapping_add(0x2e4), c.r[5]);
                pc = 0x800a22bc;
            }
            0x800a22bc => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a22d0; continue; }
                pc = 0x800a22c4;
            }
            0x800a22c4 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.stw(c.r[3], c.r[4]);
                pc = 0x800a22d4; continue;
                pc = 0x800a22d0;
            }
            0x800a22d0 => {
                c.stw(c.r[5].wrapping_add(0x2e0), c.r[4]);
                pc = 0x800a22d4;
            }
            0x800a22d4 => {
                c.r[0] = 0x20;
                c.stw(c.r[29].wrapping_add(0x2d0), c.r[0]);
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a22f4; continue; }
                pc = 0x800a22ec;
            }
            0x800a22ec => {
                c.stw(c.r[4], c.r[29]);
                pc = 0x800a22f8; continue;
                pc = 0x800a22f4;
            }
            0x800a22f4 => {
                c.stw(c.r[3].wrapping_add(0x2e0), c.r[29]);
                pc = 0x800a22f8;
            }
            0x800a22f8 => {
                c.stw(c.r[29].wrapping_add(0x2e4), c.r[3]);
                c.r[0] = 0x0;
                c.stw(c.r[29].wrapping_add(0x2e0), c.r[0]);
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2dc));
                c.stw(c.r[3].wrapping_add(0x4), c.r[29]);
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2f0));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800a2350; continue; }
                pc = 0x800a2318;
            }
            0x800a2318 => {
                c.r[29] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x800a231c;
            }
            0x800a231c => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2cc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(1) { pc = 0x800a2350; continue; }
                pc = 0x800a2328;
            }
            0x800a2328 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a2330; f_800a14ec(c);
                pc = 0x800a2330;
            }
            0x800a2330 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2d0));
                c.r[4] = c.r[3].wrapping_add(0x0);
                c.cmps(0, c.r[0], c.r[4]);
                if c.crbit(2) { pc = 0x800a2350; continue; }
                pc = 0x800a2340;
            }
            0x800a2340 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800a2348; f_800a1528(c);
                pc = 0x800a2348;
            }
            0x800a2348 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if !c.crbit(2) { pc = 0x800a231c; continue; }
                pc = 0x800a2350;
            }
            0x800a2350 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a2364; continue; }
                pc = 0x800a235c;
            }
            0x800a235c => {
                c.r[3] = 0x0;
                c.lr = 0x800a2364; f_800a1738(c);
                pc = 0x800a2364;
            }
            0x800a2364 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a236c; f_8009df64(c);
                pc = 0x800a236c;
            }
            0x800a236c => {
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
            _ => panic!("f_800a221c: bad pc {:#x}", pc),
        }
    }
}

/// OSSleepThread (0x800a238c, 236 bytes)
#[inline(never)]
pub fn f_800a238c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a238c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a238c); f_800a238c_b(c, 0x800a238c); crate::prof::exit(c, 0x800a238c); return; }
    }
    f_800a238c_b(c, 0x800a238c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a238c_at(c: &mut Cpu, pc: u32) { f_800a238c_b(c, pc) }
#[inline(always)]
fn f_800a238c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a238c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a238c, pc); }
        match pc {
            0x800a238c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800a23a8; f_8009df3c(c);
                pc = 0x800a23a8;
            }
            0x800a23a8 => {
                c.r[4] = 0x80000000;
                c.r[4] = c.lwz(c.r[4].wrapping_add(0xe4));
                c.r[0] = 0x4;
                c.r[31] = c.r[3] | c.r[3];
                c.sth(c.r[4].wrapping_add(0x2c8), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x2dc), c.r[30]);
                c.r[5] = c.lwz(c.r[30]);
                pc = 0x800a23cc; continue;
                pc = 0x800a23c8;
            }
            0x800a23c8 => {
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x2e0));
                pc = 0x800a23cc;
            }
            0x800a23cc => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x800a23e4; continue; }
                pc = 0x800a23d4;
            }
            0x800a23d4 => {
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x2d0));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x2d0));
                c.cmps(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x800a23c8; continue; }
                pc = 0x800a23e4;
            }
            0x800a23e4 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800a2418; continue; }
                pc = 0x800a23ec;
            }
            0x800a23ec => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a2400; continue; }
                pc = 0x800a23f8;
            }
            0x800a23f8 => {
                c.stw(c.r[30], c.r[4]);
                pc = 0x800a2404; continue;
                pc = 0x800a2400;
            }
            0x800a2400 => {
                c.stw(c.r[3].wrapping_add(0x2e0), c.r[4]);
                pc = 0x800a2404;
            }
            0x800a2404 => {
                c.stw(c.r[4].wrapping_add(0x2e4), c.r[3]);
                c.r[0] = 0x0;
                c.stw(c.r[4].wrapping_add(0x2e0), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x4), c.r[4]);
                pc = 0x800a243c; continue;
                pc = 0x800a2418;
            }
            0x800a2418 => {
                c.stw(c.r[4].wrapping_add(0x2e0), c.r[5]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x2e4));
                c.stw(c.r[5].wrapping_add(0x2e4), c.r[4]);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[4].wrapping_add(0x2e4), c.r[3]);
                if !c.crbit(2) { pc = 0x800a2438; continue; }
                pc = 0x800a2430;
            }
            0x800a2430 => {
                c.stw(c.r[30], c.r[4]);
                pc = 0x800a243c; continue;
                pc = 0x800a2438;
            }
            0x800a2438 => {
                c.stw(c.r[3].wrapping_add(0x2e0), c.r[4]);
                pc = 0x800a243c;
            }
            0x800a243c => {
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a2458; continue; }
                pc = 0x800a2450;
            }
            0x800a2450 => {
                c.r[3] = 0x0;
                c.lr = 0x800a2458; f_800a1738(c);
                pc = 0x800a2458;
            }
            0x800a2458 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a2460; f_8009df64(c);
                pc = 0x800a2460;
            }
            0x800a2460 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a238c: bad pc {:#x}", pc),
        }
    }
}

/// OSWakeupThread (0x800a2478, 260 bytes)
#[inline(never)]
pub fn f_800a2478(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a2478) { return; }
        if c.prof { crate::prof::enter(c, 0x800a2478); f_800a2478_b(c, 0x800a2478); crate::prof::exit(c, 0x800a2478); return; }
    }
    f_800a2478_b(c, 0x800a2478)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a2478_at(c: &mut Cpu, pc: u32) { f_800a2478_b(c, pc) }
#[inline(always)]
fn f_800a2478_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a2478;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a2478, pc); }
        match pc {
            0x800a2478 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800a2494; f_8009df3c(c);
                pc = 0x800a2494;
            }
            0x800a2494 => {
                c.r[4] = 0x80400000;
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[5] = c.r[4].wrapping_add(0xffffb898);
                pc = 0x800a253c; continue;
                pc = 0x800a24a4;
            }
            0x800a24a4 => {
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x2e0));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a24bc; continue; }
                pc = 0x800a24b0;
            }
            0x800a24b0 => {
                c.r[0] = 0x0;
                c.stw(c.r[30].wrapping_add(0x4), c.r[0]);
                pc = 0x800a24c4; continue;
                pc = 0x800a24bc;
            }
            0x800a24bc => {
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x2e4), c.r[0]);
                pc = 0x800a24c4;
            }
            0x800a24c4 => {
                c.stw(c.r[30], c.r[3]);
                c.r[0] = 0x1;
                c.sth(c.r[6].wrapping_add(0x2c8), c.r[0]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x2cc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(1) { pc = 0x800a253c; continue; }
                pc = 0x800a24dc;
            }
            0x800a24dc => {
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x2d0));
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.stw(c.r[6].wrapping_add(0x2dc), c.r[0]);
                c.r[4] = c.lwz(c.r[6].wrapping_add(0x2dc));
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800a2504; continue; }
                pc = 0x800a24fc;
            }
            0x800a24fc => {
                c.stw(c.r[4], c.r[6]);
                pc = 0x800a2508; continue;
                pc = 0x800a2504;
            }
            0x800a2504 => {
                c.stw(c.r[3].wrapping_add(0x2e0), c.r[6]);
                pc = 0x800a2508;
            }
            0x800a2508 => {
                c.stw(c.r[6].wrapping_add(0x2e4), c.r[3]);
                c.r[0] = 0x0;
                c.r[3] = 0x1;
                c.stw(c.r[6].wrapping_add(0x2e0), c.r[0]);
                c.r[4] = c.lwz(c.r[6].wrapping_add(0x2dc));
                c.stw(c.r[4].wrapping_add(0x4), c.r[6]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x2d0));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff9f40));
                c.r[0] = c.addc(!c.r[0], 0x1f, 1);
                c.r[0] = slw(c.r[3], c.r[0]);
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[13].wrapping_add(0xffff9f40), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffff9f44), c.r[3]);
                pc = 0x800a253c;
            }
            0x800a253c => {
                c.r[6] = c.lwz(c.r[30]);
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x800a24a4; continue; }
                pc = 0x800a2548;
            }
            0x800a2548 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a255c; continue; }
                pc = 0x800a2554;
            }
            0x800a2554 => {
                c.r[3] = 0x0;
                c.lr = 0x800a255c; f_800a1738(c);
                pc = 0x800a255c;
            }
            0x800a255c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a2564; f_8009df64(c);
                pc = 0x800a2564;
            }
            0x800a2564 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a2478: bad pc {:#x}", pc),
        }
    }
}

/// fn_800A257C (0x800a257c, 192 bytes)
#[inline(never)]
pub fn f_800a257c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a257c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a257c); f_800a257c_b(c, 0x800a257c); crate::prof::exit(c, 0x800a257c); return; }
    }
    f_800a257c_b(c, 0x800a257c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a257c_at(c: &mut Cpu, pc: u32) { f_800a257c_b(c, pc) }
#[inline(always)]
fn f_800a257c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a257c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a257c, pc); }
        match pc {
            0x800a257c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.rc(c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                if c.crbit(0) { pc = 0x800a25a8; continue; }
                pc = 0x800a25a0;
            }
            0x800a25a0 => {
                c.cmps(0, c.r[31], 0x1f);
                if !c.crbit(1) { pc = 0x800a25b0; continue; }
                pc = 0x800a25a8;
            }
            0x800a25a8 => {
                c.r[3] = 0x0;
                pc = 0x800a2620; continue;
                pc = 0x800a25b0;
            }
            0x800a25b0 => {
                c.lr = 0x800a25b4; f_8009df3c(c);
                pc = 0x800a25b4;
            }
            0x800a25b4 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x2d4));
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.cmps(0, c.r[0], c.r[31]);
                if c.crbit(2) { pc = 0x800a2614; continue; }
                pc = 0x800a25c4;
            }
            0x800a25c4 => {
                c.stw(c.r[29].wrapping_add(0x2d4), c.r[31]);
                c.r[31] = c.r[29] | c.r[29];
                pc = 0x800a25cc;
            }
            0x800a25cc => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x2cc));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(1) { pc = 0x800a2600; continue; }
                pc = 0x800a25d8;
            }
            0x800a25d8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a25e0; f_800a14ec(c);
                pc = 0x800a25e0;
            }
            0x800a25e0 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x2d0));
                c.r[4] = c.r[3].wrapping_add(0x0);
                c.cmps(0, c.r[0], c.r[4]);
                if c.crbit(2) { pc = 0x800a2600; continue; }
                pc = 0x800a25f0;
            }
            0x800a25f0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800a25f8; f_800a1528(c);
                pc = 0x800a25f8;
            }
            0x800a25f8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if !c.crbit(2) { pc = 0x800a25cc; continue; }
                pc = 0x800a2600;
            }
            0x800a2600 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff9f44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a2614; continue; }
                pc = 0x800a260c;
            }
            0x800a260c => {
                c.r[3] = 0x0;
                c.lr = 0x800a2614; f_800a1738(c);
                pc = 0x800a2614;
            }
            0x800a2614 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800a261c; f_8009df64(c);
                pc = 0x800a261c;
            }
            0x800a261c => {
                c.r[3] = 0x1;
                pc = 0x800a2620;
            }
            0x800a2620 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a257c: bad pc {:#x}", pc),
        }
    }
}

/// OSSetIdleFunction (0x800a263c, 144 bytes)
#[inline(never)]
pub fn f_800a263c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a263c) { return; }
        if c.prof { crate::prof::enter(c, 0x800a263c); f_800a263c_b(c, 0x800a263c); crate::prof::exit(c, 0x800a263c); return; }
    }
    f_800a263c_b(c, 0x800a263c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a263c_at(c: &mut Cpu, pc: u32) { f_800a263c_b(c, pc) }
#[inline(always)]
fn f_800a263c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a263c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a263c, pc); }
        match pc {
            0x800a263c => {
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[8] = 0x80400000;
                c.r[10] = c.r[4].wrapping_add(0x0);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[9] = c.r[5].wrapping_add(0x0);
                c.r[7] = c.r[6].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[8].wrapping_add(0xffffb898);
                if c.crbit(2) { pc = 0x800a26a0; continue; }
                pc = 0x800a2668;
            }
            0x800a2668 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x3c8));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800a26b4; continue; }
                pc = 0x800a2674;
            }
            0x800a2674 => {
                c.r[4] = c.r[3].wrapping_add(0x0);
                c.r[6] = c.r[9].wrapping_add(0x0);
                c.r[5] = c.r[10].wrapping_add(0x0);
                c.r[3] = c.r[31].wrapping_add(0x100);
                c.r[8] = 0x1f;
                c.r[9] = 0x1;
                c.lr = 0x800a2690; f_800a19cc(c);
                pc = 0x800a2690;
            }
            0x800a2690 => {
                c.r[3] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800a2698; f_800a1f94(c);
                pc = 0x800a2698;
            }
            0x800a2698 => {
                c.r[3] = c.r[31].wrapping_add(0x100);
                pc = 0x800a26b8; continue;
                pc = 0x800a26a0;
            }
            0x800a26a0 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x3c8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800a26b4; continue; }
                pc = 0x800a26ac;
            }
            0x800a26ac => {
                c.r[3] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800a26b4; f_800a1c98(c);
                pc = 0x800a26b4;
            }
            0x800a26b4 => {
                c.r[3] = 0x0;
                pc = 0x800a26b8;
            }
            0x800a26b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a263c: bad pc {:#x}", pc),
        }
    }
}

/// OSClearStack (0x800a26cc, 172 bytes)
#[inline(never)]
pub fn f_800a26cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800a26cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800a26cc); f_800a26cc_b(c, 0x800a26cc); crate::prof::exit(c, 0x800a26cc); return; }
    }
    f_800a26cc_b(c, 0x800a26cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800a26cc_at(c: &mut Cpu, pc: u32) { f_800a26cc_b(c, pc) }
#[inline(always)]
fn f_800a26cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800a26cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800a26cc, pc); }
        match pc {
            0x800a26cc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[3].rotate_left(16) & 0xff0000;
                c.r[0] = (c.r[3].rotate_left(24) & 0xff000000) | (c.r[0] & 0xffffff);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = (c.r[3].rotate_left(8) & 0xff00) | (c.r[0] & 0xffff00ff);
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[0] | c.r[0];
                c.r[31] = (c.r[3].rotate_left(0) & 0xff) | (c.r[31] & 0xffffff00);
                c.lr = 0x800a26f4; f_8009bd28(c);
                pc = 0x800a26f4;
            }
            0x800a26f4 => {
                c.r[4] = 0x80000000;
                c.r[5] = c.lwz(c.r[4].wrapping_add(0xe4));
                c.r[4] = c.r[3].wrapping_add(0x3);
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x308));
                c.r[5] = c.r[5].wrapping_add(0x4);
                c.cmpu(0, c.r[5], c.r[3]);
                c.r[4] = c.r[4].wrapping_sub(c.r[5]);
                c.r[4] = c.r[4].rotate_left(30) & 0x3fffffff;
                if !c.crbit(0) { pc = 0x800a2764; continue; }
                pc = 0x800a2718;
            }
            0x800a2718 => {
                c.r[0] = c.r[4].rotate_left(29) & 0x1fffffff;
                c.rc(c.r[0]);
                c.mtspr(9, c.r[0]);
                if c.crbit(2) { pc = 0x800a2754; continue; }
                pc = 0x800a2724;
            }
            0x800a2724 => {
                c.stw(c.r[5], c.r[31]);
                c.stw(c.r[5].wrapping_add(0x4), c.r[31]);
                c.stw(c.r[5].wrapping_add(0x8), c.r[31]);
                c.stw(c.r[5].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[5].wrapping_add(0x10), c.r[31]);
                c.stw(c.r[5].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[5].wrapping_add(0x18), c.r[31]);
                c.stw(c.r[5].wrapping_add(0x1c), c.r[31]);
                c.r[5] = c.r[5].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800a2724; continue; }
                pc = 0x800a274c;
            }
            0x800a274c => {
                c.r[4] = c.r[4] & 0x7;
                c.rc(c.r[4]);
                if c.crbit(2) { pc = 0x800a2764; continue; }
                pc = 0x800a2754;
            }
            0x800a2754 => {
                c.mtspr(9, c.r[4]);
                pc = 0x800a2758;
            }
            0x800a2758 => {
                c.stw(c.r[5], c.r[31]);
                c.r[5] = c.r[5].wrapping_add(0x4);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800a2758; continue; }
                pc = 0x800a2764;
            }
            0x800a2764 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800a26cc: bad pc {:#x}", pc),
        }
    }
}

