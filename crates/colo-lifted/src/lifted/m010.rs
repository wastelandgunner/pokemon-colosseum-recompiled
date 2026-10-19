#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// GXInitLightDistAttn (0x800ba344, 208 bytes)
#[inline(never)]
pub fn f_800ba344(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba344) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba344); p = true; }
    }
    f_800ba344_b(c, 0x800ba344);
    if p { crate::prof::exit(c, 0x800ba344); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba344_at(c: &mut Cpu, pc: u32) { f_800ba344_b(c, pc) }
#[inline(always)]
fn f_800ba344_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba344;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba344, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba344 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff8c78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if !cr_bit(cr, 0) { pc = 0x800ba354; continue; }
                pc = 0x800ba350;
            }
            0x800ba350 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800ba354;
            }
            0x800ba354 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff8c78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[2], c.f[0]);
                { let (a, b) = (cr_bit(cr, 0), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if cr_bit(cr, 2) { pc = 0x800ba374; continue; }
                pc = 0x800ba364;
            }
            0x800ba364 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff8c90); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[2], c.f[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x800ba378; continue; }
                pc = 0x800ba374;
            }
            0x800ba374 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800ba378;
            }
            0x800ba378 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x2);
                if cr_bit(cr, 2) { pc = 0x800ba3b8; continue; }
                pc = 0x800ba380;
            }
            0x800ba380 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800ba394; continue; }
                pc = 0x800ba384;
            }
            0x800ba384 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { pc = 0x800ba3f8; continue; }
                pc = 0x800ba38c;
            }
            0x800ba38c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800ba3a0; continue; }
                pc = 0x800ba390;
            }
            0x800ba390 => { ins += 1;
                pc = 0x800ba3f8; continue;
                pc = 0x800ba394;
            }
            0x800ba394 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x4);
                if !cr_bit(cr, 0) { pc = 0x800ba3f8; continue; }
                pc = 0x800ba39c;
            }
            0x800ba39c => { ins += 1;
                pc = 0x800ba3dc; continue;
                pc = 0x800ba3a0;
            }
            0x800ba3a0 => { ins += 6;
                { let ea = c.r[2].wrapping_add(0xffff8c90); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let v = s(c.f[2] * c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c78); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let v = s(c.f[5] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[3] = v; c.ps1[3] = v; }
                pc = 0x800ba404; continue;
                pc = 0x800ba3b8;
            }
            0x800ba3b8 => { ins += 9;
                { let ea = c.r[2].wrapping_add(0xffff8c90); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let v = s(c.f[2] * c.f[1]); c.f[4] = v; c.ps1[4] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8ca4); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[5] - c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[4]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[3] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[4]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                pc = 0x800ba404; continue;
                pc = 0x800ba3dc;
            }
            0x800ba3dc => { ins += 7;
                { let v = s(c.f[2] * c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8c90); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[2].wrapping_add(0xffff8c78); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[5] - c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] / c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                pc = 0x800ba404; continue;
                pc = 0x800ba3f8;
            }
            0x800ba3f8 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff8c78); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[2].wrapping_add(0xffff8c90); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                c.f[4] = c.f[3];
                pc = 0x800ba404;
            }
            0x800ba404 => { ins += 4;
                { let ea = c.r[3].wrapping_add(0x1c); let v = c.f[5]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[4]; c.stfs(ea, v); }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba344: bad pc {:#x}", pc),
        }
    }
}

/// GXInitLightPos (0x800ba414, 16 bytes)
#[inline(never)]
pub fn f_800ba414(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba414) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba414); p = true; }
    }
    f_800ba414_b(c, 0x800ba414);
    if p { crate::prof::exit(c, 0x800ba414); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba414_at(c: &mut Cpu, pc: u32) { f_800ba414_b(c, pc) }
#[inline(always)]
fn f_800ba414_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba414;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba414, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba414 => { ins += 4;
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x2c); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x30); let v = c.f[3]; c.stfs(ea, v); }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba414: bad pc {:#x}", pc),
        }
    }
}

/// GXInitLightDir (0x800ba424, 28 bytes)
#[inline(never)]
pub fn f_800ba424(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba424) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba424); p = true; }
    }
    f_800ba424_b(c, 0x800ba424);
    if p { crate::prof::exit(c, 0x800ba424); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba424_at(c: &mut Cpu, pc: u32) { f_800ba424_b(c, pc) }
#[inline(always)]
fn f_800ba424_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba424;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba424, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba424 => { ins += 7;
                c.f[4] = -c.f[1];
                c.f[1] = -c.f[2];
                c.f[0] = -c.f[3];
                { let ea = c.r[3].wrapping_add(0x34); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x38); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x3c); let v = c.f[0]; c.stfs(ea, v); }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba424: bad pc {:#x}", pc),
        }
    }
}

/// GXInitLightColor (0x800ba440, 12 bytes)
#[inline(never)]
pub fn f_800ba440(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba440) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba440); p = true; }
    }
    f_800ba440_b(c, 0x800ba440);
    if p { crate::prof::exit(c, 0x800ba440); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba440_at(c: &mut Cpu, pc: u32) { f_800ba440_b(c, pc) }
#[inline(always)]
fn f_800ba440_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba440;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba440, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba440 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba440: bad pc {:#x}", pc),
        }
    }
}

/// GXLoadLightObjImm (0x800ba44c, 124 bytes)
#[inline(never)]
pub fn f_800ba44c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba44c) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba44c); p = true; }
    }
    f_800ba44c_b(c, 0x800ba44c);
    if p { crate::prof::exit(c, 0x800ba44c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba44c_at(c: &mut Cpu, pc: u32) { f_800ba44c_b(c, pc) }
#[inline(always)]
fn f_800ba44c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba44c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba44c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba44c => { ins += 31;
                c.r[0] = c.r[4].leading_zeros();
                c.r[0] = c.addc(!c.r[0], 0x1f, 1);
                c.r[5] = c.r[0].rotate_left(4) & 0x70;
                c.r[4] = 0xcc010000;
                c.r[0] = 0x10;
                c.r[5] = c.r[5].wrapping_add(0x600);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.r[5] | 0xf0000;
                { let ea = c.r[4].wrapping_add(0xffff8000); c.stw(ea, c.r[0]); c.r[4] = ea; }
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                c.r[6] = c.r[6] ^ c.r[6];
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[3].wrapping_add(0x30); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x38); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                c.stw(c.r[4], c.r[6]);
                c.stw(c.r[4], c.r[6]);
                c.stw(c.r[4], c.r[6]);
                c.stw(c.r[4], c.r[0]);
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x1;
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba44c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetChanAmbColor (0x800ba4c8, 244 bytes)
#[inline(never)]
pub fn f_800ba4c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba4c8) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba4c8); p = true; }
    }
    f_800ba4c8_b(c, 0x800ba4c8);
    if p { crate::prof::exit(c, 0x800ba4c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba4c8_at(c: &mut Cpu, pc: u32) { f_800ba4c8_b(c, pc) }
#[inline(always)]
fn f_800ba4c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba4c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba4c8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba4c8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x3);
                if cr_bit(cr, 2) { pc = 0x800ba554; continue; }
                pc = 0x800ba4d0;
            }
            0x800ba4d0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800ba4ec; continue; }
                pc = 0x800ba4d4;
            }
            0x800ba4d4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                if cr_bit(cr, 2) { pc = 0x800ba51c; continue; }
                pc = 0x800ba4dc;
            }
            0x800ba4dc => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800ba53c; continue; }
                pc = 0x800ba4e0;
            }
            0x800ba4e0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x800ba4fc; continue; }
                pc = 0x800ba4e8;
            }
            0x800ba4e8 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800ba4ec;
            }
            0x800ba4ec => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x5);
                if cr_bit(cr, 2) { pc = 0x800ba578; continue; }
                pc = 0x800ba4f4;
            }
            0x800ba4f4 => { ins += 1;
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800ba4f8;
            }
            0x800ba4f8 => { ins += 1;
                pc = 0x800ba56c; continue;
                pc = 0x800ba4fc;
            }
            0x800ba4fc => { ins += 8;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0x0;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xa8));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff00;
                c.r[7] = c.r[0] | c.r[0];
                c.r[7] = (c.r[3].rotate_left(0) & 0xff) | (c.r[7] & 0xffffff00);
                pc = 0x800ba588; continue;
                pc = 0x800ba51c;
            }
            0x800ba51c => { ins += 8;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xac));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff00;
                c.r[7] = c.r[0] | c.r[0];
                c.r[7] = (c.r[3].rotate_left(0) & 0xff) | (c.r[7] & 0xffffff00);
                pc = 0x800ba588; continue;
                pc = 0x800ba53c;
            }
            0x800ba53c => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0x0;
                c.r[7] = c.lbz_t(ins, c.r[4].wrapping_add(0x3));
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xa8));
                c.r[7] = (c.r[3].rotate_left(0) & 0xffffff00) | (c.r[7] & 0xff);
                pc = 0x800ba588; continue;
                pc = 0x800ba554;
            }
            0x800ba554 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0x1;
                c.r[7] = c.lbz_t(ins, c.r[4].wrapping_add(0x3));
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xac));
                c.r[7] = (c.r[3].rotate_left(0) & 0xffffff00) | (c.r[7] & 0xff);
                pc = 0x800ba588; continue;
                pc = 0x800ba56c;
            }
            0x800ba56c => { ins += 3;
                c.r[7] = c.lwz_t(ins, c.r[4]);
                c.r[5] = 0x0;
                pc = 0x800ba588; continue;
                pc = 0x800ba578;
            }
            0x800ba578 => { ins += 3;
                c.r[7] = c.lwz_t(ins, c.r[4]);
                c.r[5] = 0x1;
                pc = 0x800ba588; continue;
                pc = 0x800ba584;
            }
            0x800ba584 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800ba588;
            }
            0x800ba588 => { ins += 13;
                c.r[0] = 0x10;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[6] = 0xcc010000;
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[0]);
                c.r[3] = c.r[5].wrapping_add(0x100a);
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[3]);
                c.r[5] = 0x1;
                c.r[3] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[7]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[5]);
                c.stw(c.r[3].wrapping_add(0xa8), c.r[7]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba4c8: bad pc {:#x}", pc),
        }
    }
}

/// GXSetChanMatColor (0x800ba5bc, 244 bytes)
#[inline(never)]
pub fn f_800ba5bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba5bc) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba5bc); p = true; }
    }
    f_800ba5bc_b(c, 0x800ba5bc);
    if p { crate::prof::exit(c, 0x800ba5bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba5bc_at(c: &mut Cpu, pc: u32) { f_800ba5bc_b(c, pc) }
#[inline(always)]
fn f_800ba5bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba5bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba5bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba5bc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x3);
                if cr_bit(cr, 2) { pc = 0x800ba648; continue; }
                pc = 0x800ba5c4;
            }
            0x800ba5c4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800ba5e0; continue; }
                pc = 0x800ba5c8;
            }
            0x800ba5c8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                if cr_bit(cr, 2) { pc = 0x800ba610; continue; }
                pc = 0x800ba5d0;
            }
            0x800ba5d0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800ba630; continue; }
                pc = 0x800ba5d4;
            }
            0x800ba5d4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 0) { pc = 0x800ba5f0; continue; }
                pc = 0x800ba5dc;
            }
            0x800ba5dc => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800ba5e0;
            }
            0x800ba5e0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x5);
                if cr_bit(cr, 2) { pc = 0x800ba66c; continue; }
                pc = 0x800ba5e8;
            }
            0x800ba5e8 => { ins += 1;
                if !cr_bit(cr, 0) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800ba5ec;
            }
            0x800ba5ec => { ins += 1;
                pc = 0x800ba660; continue;
                pc = 0x800ba5f0;
            }
            0x800ba5f0 => { ins += 8;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0x0;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xb0));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff00;
                c.r[7] = c.r[0] | c.r[0];
                c.r[7] = (c.r[3].rotate_left(0) & 0xff) | (c.r[7] & 0xffffff00);
                pc = 0x800ba67c; continue;
                pc = 0x800ba610;
            }
            0x800ba610 => { ins += 8;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xb4));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff00;
                c.r[7] = c.r[0] | c.r[0];
                c.r[7] = (c.r[3].rotate_left(0) & 0xff) | (c.r[7] & 0xffffff00);
                pc = 0x800ba67c; continue;
                pc = 0x800ba630;
            }
            0x800ba630 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0x0;
                c.r[7] = c.lbz_t(ins, c.r[4].wrapping_add(0x3));
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xb0));
                c.r[7] = (c.r[3].rotate_left(0) & 0xffffff00) | (c.r[7] & 0xff);
                pc = 0x800ba67c; continue;
                pc = 0x800ba648;
            }
            0x800ba648 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0x1;
                c.r[7] = c.lbz_t(ins, c.r[4].wrapping_add(0x3));
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0xb4));
                c.r[7] = (c.r[3].rotate_left(0) & 0xffffff00) | (c.r[7] & 0xff);
                pc = 0x800ba67c; continue;
                pc = 0x800ba660;
            }
            0x800ba660 => { ins += 3;
                c.r[7] = c.lwz_t(ins, c.r[4]);
                c.r[5] = 0x0;
                pc = 0x800ba67c; continue;
                pc = 0x800ba66c;
            }
            0x800ba66c => { ins += 3;
                c.r[7] = c.lwz_t(ins, c.r[4]);
                c.r[5] = 0x1;
                pc = 0x800ba67c; continue;
                pc = 0x800ba678;
            }
            0x800ba678 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800ba67c;
            }
            0x800ba67c => { ins += 13;
                c.r[0] = 0x10;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[6] = 0xcc010000;
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[0]);
                c.r[3] = c.r[5].wrapping_add(0x100c);
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[3]);
                c.r[5] = 0x1;
                c.r[3] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[7]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[5]);
                c.stw(c.r[3].wrapping_add(0xb0), c.r[7]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba5bc: bad pc {:#x}", pc),
        }
    }
}

/// GXSetNumChans (0x800ba6b0, 68 bytes)
#[inline(never)]
pub fn f_800ba6b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba6b0) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba6b0); p = true; }
    }
    f_800ba6b0_b(c, 0x800ba6b0);
    if p { crate::prof::exit(c, 0x800ba6b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba6b0_at(c: &mut Cpu, pc: u32) { f_800ba6b0_b(c, pc) }
#[inline(always)]
fn f_800ba6b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba6b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba6b0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba6b0 => { ins += 17;
                c.r[6] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[3].rotate_left(4) & 0xff0;
                c.r[8] = c.r[3].rotate_left(0) & 0xff;
                c.r[5] = c.lwz_t(ins, c.r[6].wrapping_add(0x204));
                c.r[4] = 0x10;
                c.r[3] = 0xcc010000;
                c.r[5] = c.r[5].rotate_left(0) & 0xffffff8f;
                c.r[0] = c.r[5] | c.r[0];
                c.stw(c.r[6].wrapping_add(0x204), c.r[0]);
                c.r[0] = 0x1009;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[8]);
                c.r[0] = c.lwz_t(ins, c.r[6].wrapping_add(0x4f4));
                c.r[0] = c.r[0] | 0x4;
                c.stw(c.r[6].wrapping_add(0x4f4), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba6b0: bad pc {:#x}", pc),
        }
    }
}

/// GXSetChanCtrl (0x800ba6f4, 204 bytes)
#[inline(never)]
pub fn f_800ba6f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba6f4) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba6f4); p = true; }
    }
    f_800ba6f4_b(c, 0x800ba6f4);
    if p { crate::prof::exit(c, 0x800ba6f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba6f4_at(c: &mut Cpu, pc: u32) { f_800ba6f4_b(c, pc) }
#[inline(always)]
fn f_800ba6f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba6f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba6f4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba6f4 => { ins += 8;
                c.r[0] = c.r[4].rotate_left(1) & 0x1fe;
                c.r[0] = c.r[0] | c.r[6];
                c.r[4] = c.r[0].rotate_left(0) & 0xffffffbf;
                c.r[0] = c.r[5].rotate_left(6) & 0xffffffc0;
                cr_cmps(&mut cr, c.xer, 0, c.r[9], 0x0);
                c.r[10] = c.r[3].rotate_left(0) & 0x3;
                c.r[6] = c.r[4] | c.r[0];
                if !cr_bit(cr, 2) { pc = 0x800ba718; continue; }
                pc = 0x800ba714;
            }
            0x800ba714 => { ins += 1;
                c.r[8] = 0x0;
                pc = 0x800ba718;
            }
            0x800ba718 => { ins += 27;
                c.r[4] = c.addc(!c.r[9], 0x2, 1);
                c.r[0] = c.addc(c.r[4], 0xffffffff, 0);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[0], c.r[4], ci); }
                c.r[4] = c.r[9].wrapping_neg();
                c.r[0] = c.addc(c.r[4], 0xffffffff, 0);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[4], ci); }
                c.r[6] = c.r[6].rotate_left(0) & 0xfffffe7f;
                c.r[4] = c.r[8].rotate_left(7) & 0xffffff80;
                c.r[4] = c.r[6] | c.r[4];
                c.r[6] = c.r[4].rotate_left(0) & 0xfffffdff;
                c.r[4] = c.r[5].rotate_left(9) & 0xfffffe00;
                c.r[4] = c.r[6] | c.r[4];
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffbff;
                c.r[0] = c.r[0].rotate_left(10) & 0xfffffc00;
                c.r[6] = c.r[4] | c.r[0];
                c.r[6] = c.r[6].rotate_left(0) & 0xffffffc3;
                c.r[6] = (c.r[7].rotate_left(2) & 0x3c) | (c.r[6] & 0xffffffc3);
                c.r[6] = c.r[6].rotate_left(0) & 0xffff87ff;
                c.r[5] = 0x10;
                c.r[4] = 0xcc010000;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[0] = c.r[10].wrapping_add(0x100e);
                c.r[6] = (c.r[7].rotate_left(7) & 0x7800) | (c.r[6] & 0xffff87ff);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x4);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                if !cr_bit(cr, 2) { pc = 0x800ba798; continue; }
                pc = 0x800ba784;
            }
            0x800ba784 => { ins += 5;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[0] = 0x1010;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                pc = 0x800ba7b0; continue;
                pc = 0x800ba798;
            }
            0x800ba798 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x5);
                if !cr_bit(cr, 2) { pc = 0x800ba7b0; continue; }
                pc = 0x800ba7a0;
            }
            0x800ba7a0 => { ins += 4;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[0] = 0x1011;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                pc = 0x800ba7b0;
            }
            0x800ba7b0 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x1;
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba6f4: bad pc {:#x}", pc),
        }
    }
}

/// GXGetTexBufferSize (0x800ba7c0, 348 bytes)
#[inline(never)]
pub fn f_800ba7c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba7c0) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba7c0); p = true; }
    }
    f_800ba7c0_b(c, 0x800ba7c0);
    if p { crate::prof::exit(c, 0x800ba7c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba7c0_at(c: &mut Cpu, pc: u32) { f_800ba7c0_b(c, pc) }
#[inline(never)]
fn f_800ba7c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba7c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba7c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba7c0 => { ins += 1;
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800ba7c4;
            }
            0x800ba7c4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x3c);
                pc = 0x800ba7c8;
            }
            0x800ba7c8 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                pc = 0x800ba7cc;
            }
            0x800ba7cc => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x800ba80c; continue; }
                pc = 0x800ba7d0;
            }
            0x800ba7d0 => { ins += 1;
                c.r[8] = 0x80310000;
                pc = 0x800ba7d4;
            }
            0x800ba7d4 => { ins += 1;
                c.r[8] = c.r[8].wrapping_add(0x3368);
                pc = 0x800ba7d8;
            }
            0x800ba7d8 => { ins += 1;
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                pc = 0x800ba7dc;
            }
            0x800ba7dc => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[8].wrapping_add(c.r[0]));
                pc = 0x800ba7e0;
            }
            0x800ba7e0 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x800ba7e4;
            }
            0x800ba7e4 => { ins += 1;
                if true { let t = c.ctr; if (0x800ba7c0..0x800ba91c).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800ba7e8;
            }
            0x800ba7e8 => { ins += 1;
                c.r[0] = 0x3;
                pc = 0x800ba7ec;
            }
            0x800ba7ec => { ins += 1;
                c.r[8] = 0x3;
                pc = 0x800ba7f0;
            }
            0x800ba7f0 => { ins += 1;
                pc = 0x800ba814; continue;
                pc = 0x800ba7f4;
            }
            0x800ba7f4 => { ins += 1;
                c.r[0] = 0x3;
                pc = 0x800ba7f8;
            }
            0x800ba7f8 => { ins += 1;
                c.r[8] = 0x2;
                pc = 0x800ba7fc;
            }
            0x800ba7fc => { ins += 1;
                pc = 0x800ba814; continue;
                pc = 0x800ba800;
            }
            0x800ba800 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x800ba804;
            }
            0x800ba804 => { ins += 1;
                c.r[8] = 0x2;
                pc = 0x800ba808;
            }
            0x800ba808 => { ins += 1;
                pc = 0x800ba814; continue;
                pc = 0x800ba80c;
            }
            0x800ba80c => { ins += 1;
                c.r[8] = 0x0;
                pc = 0x800ba810;
            }
            0x800ba810 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800ba814;
            }
            0x800ba814 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x6);
                pc = 0x800ba818;
            }
            0x800ba818 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800ba824; continue; }
                pc = 0x800ba81c;
            }
            0x800ba81c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x16);
                pc = 0x800ba820;
            }
            0x800ba820 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800ba82c; continue; }
                pc = 0x800ba824;
            }
            0x800ba824 => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800ba828;
            }
            0x800ba828 => { ins += 1;
                pc = 0x800ba830; continue;
                pc = 0x800ba82c;
            }
            0x800ba82c => { ins += 1;
                c.r[5] = 0x20;
                pc = 0x800ba830;
            }
            0x800ba830 => { ins += 1;
                c.r[6] = c.r[6].rotate_left(0) & 0xff;
                pc = 0x800ba834;
            }
            0x800ba834 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], 0x1);
                pc = 0x800ba838;
            }
            0x800ba838 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800ba8d8; continue; }
                pc = 0x800ba83c;
            }
            0x800ba83c => { ins += 1;
                c.r[9] = c.r[7].rotate_left(0) & 0xff;
                pc = 0x800ba840;
            }
            0x800ba840 => { ins += 1;
                c.r[6] = 0x1;
                pc = 0x800ba844;
            }
            0x800ba844 => { ins += 1;
                c.mtspr(9, c.r[9]);
                pc = 0x800ba848;
            }
            0x800ba848 => { ins += 1;
                c.r[7] = slw(c.r[6], c.r[8]);
                pc = 0x800ba84c;
            }
            0x800ba84c => { ins += 1;
                c.r[6] = slw(c.r[6], c.r[0]);
                pc = 0x800ba850;
            }
            0x800ba850 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[9], 0x0);
                pc = 0x800ba854;
            }
            0x800ba854 => { ins += 1;
                c.r[10] = c.r[6].wrapping_add(0xffffffff);
                pc = 0x800ba858;
            }
            0x800ba858 => { ins += 1;
                c.r[7] = c.r[7].wrapping_add(0xffffffff);
                pc = 0x800ba85c;
            }
            0x800ba85c => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x800ba860;
            }
            0x800ba860 => { ins += 1;
                if !cr_bit(cr, 1) { pc = 0x800ba90c; continue; }
                pc = 0x800ba864;
            }
            0x800ba864 => { ins += 1;
                c.r[11] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x800ba868;
            }
            0x800ba868 => { ins += 1;
                c.r[6] = c.r[11].wrapping_add(c.r[10]);
                pc = 0x800ba86c;
            }
            0x800ba86c => { ins += 1;
                c.r[12] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x800ba870;
            }
            0x800ba870 => { ins += 1;
                { let v = c.r[6] as i32; let n = c.r[0] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[9] = r as u32; }
                pc = 0x800ba874;
            }
            0x800ba874 => { ins += 1;
                c.r[6] = c.r[12].wrapping_add(c.r[7]);
                pc = 0x800ba878;
            }
            0x800ba878 => { ins += 1;
                { let v = c.r[6] as i32; let n = c.r[8] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[6] = r as u32; }
                pc = 0x800ba87c;
            }
            0x800ba87c => { ins += 1;
                c.r[6] = c.r[9].wrapping_mul(c.r[6]);
                pc = 0x800ba880;
            }
            0x800ba880 => { ins += 1;
                c.r[6] = c.r[5].wrapping_mul(c.r[6]);
                pc = 0x800ba884;
            }
            0x800ba884 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[11], 0x1);
                pc = 0x800ba888;
            }
            0x800ba888 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(c.r[6]);
                pc = 0x800ba88c;
            }
            0x800ba88c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800ba898; continue; }
                pc = 0x800ba890;
            }
            0x800ba890 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[12], 0x1);
                pc = 0x800ba894;
            }
            0x800ba894 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800ba90c; continue; }
                pc = 0x800ba898;
            }
            0x800ba898 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x800ba89c;
            }
            0x800ba89c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x1);
                pc = 0x800ba8a0;
            }
            0x800ba8a0 => { ins += 1;
                if !cr_bit(cr, 1) { pc = 0x800ba8ac; continue; }
                pc = 0x800ba8a4;
            }
            0x800ba8a4 => { ins += 1;
                { let v = c.r[11] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[6] = (v >> 1) as u32; }
                pc = 0x800ba8a8;
            }
            0x800ba8a8 => { ins += 1;
                pc = 0x800ba8b0; continue;
                pc = 0x800ba8ac;
            }
            0x800ba8ac => { ins += 1;
                c.r[6] = 0x1;
                pc = 0x800ba8b0;
            }
            0x800ba8b0 => { ins += 1;
                c.r[3] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x800ba8b4;
            }
            0x800ba8b4 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x1);
                pc = 0x800ba8b8;
            }
            0x800ba8b8 => { ins += 1;
                c.r[3] = c.r[6].rotate_left(0) & 0xffff;
                pc = 0x800ba8bc;
            }
            0x800ba8bc => { ins += 1;
                if !cr_bit(cr, 1) { pc = 0x800ba8c8; continue; }
                pc = 0x800ba8c0;
            }
            0x800ba8c0 => { ins += 1;
                { let v = c.r[12] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[4] = (v >> 1) as u32; }
                pc = 0x800ba8c4;
            }
            0x800ba8c4 => { ins += 1;
                pc = 0x800ba8cc; continue;
                pc = 0x800ba8c8;
            }
            0x800ba8c8 => { ins += 1;
                c.r[4] = 0x1;
                pc = 0x800ba8cc;
            }
            0x800ba8cc => { ins += 1;
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x800ba8d0;
            }
            0x800ba8d0 => { ins += 1;
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800ba864; continue; }
                pc = 0x800ba8d4;
            }
            0x800ba8d4 => { ins += 1;
                pc = 0x800ba90c; continue;
                pc = 0x800ba8d8;
            }
            0x800ba8d8 => { ins += 1;
                c.r[6] = 0x1;
                pc = 0x800ba8dc;
            }
            0x800ba8dc => { ins += 1;
                c.r[7] = slw(c.r[6], c.r[0]);
                pc = 0x800ba8e0;
            }
            0x800ba8e0 => { ins += 1;
                c.r[6] = slw(c.r[6], c.r[8]);
                pc = 0x800ba8e4;
            }
            0x800ba8e4 => { ins += 1;
                c.r[9] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x800ba8e8;
            }
            0x800ba8e8 => { ins += 1;
                c.r[3] = c.r[7].wrapping_add(0xffffffff);
                pc = 0x800ba8ec;
            }
            0x800ba8ec => { ins += 1;
                c.r[7] = c.r[9].wrapping_add(c.r[3]);
                pc = 0x800ba8f0;
            }
            0x800ba8f0 => { ins += 1;
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x800ba8f4;
            }
            0x800ba8f4 => { ins += 1;
                c.r[3] = c.r[6].wrapping_add(0xffffffff);
                pc = 0x800ba8f8;
            }
            0x800ba8f8 => { ins += 1;
                { let v = c.r[7] as i32; let n = c.r[0] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[6] = r as u32; }
                pc = 0x800ba8fc;
            }
            0x800ba8fc => { ins += 1;
                c.r[0] = c.r[4].wrapping_add(c.r[3]);
                pc = 0x800ba900;
            }
            0x800ba900 => { ins += 1;
                { let v = c.r[0] as i32; let n = c.r[8] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[0] = r as u32; }
                pc = 0x800ba904;
            }
            0x800ba904 => { ins += 1;
                c.r[0] = c.r[6].wrapping_mul(c.r[0]);
                pc = 0x800ba908;
            }
            0x800ba908 => { ins += 1;
                c.r[31] = c.r[5].wrapping_mul(c.r[0]);
                pc = 0x800ba90c;
            }
            0x800ba90c => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800ba910;
            }
            0x800ba910 => { ins += 1;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                pc = 0x800ba914;
            }
            0x800ba914 => { ins += 1;
                c.r[1] = c.r[1].wrapping_add(0x28);
                pc = 0x800ba918;
            }
            0x800ba918 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba7c0: bad pc {:#x}", pc),
        }
    }
}

/// __GetImageTileCount (0x800ba91c, 200 bytes)
#[inline(never)]
pub fn f_800ba91c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba91c) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba91c); p = true; }
    }
    f_800ba91c_b(c, 0x800ba91c);
    if p { crate::prof::exit(c, 0x800ba91c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba91c_at(c: &mut Cpu, pc: u32) { f_800ba91c_b(c, pc) }
#[inline(never)]
fn f_800ba91c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba91c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba91c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba91c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x3c);
                pc = 0x800ba920;
            }
            0x800ba920 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x800ba960; continue; }
                pc = 0x800ba924;
            }
            0x800ba924 => { ins += 1;
                c.r[9] = 0x80310000;
                pc = 0x800ba928;
            }
            0x800ba928 => { ins += 1;
                c.r[9] = c.r[9].wrapping_add(0x345c);
                pc = 0x800ba92c;
            }
            0x800ba92c => { ins += 1;
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                pc = 0x800ba930;
            }
            0x800ba930 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[9].wrapping_add(c.r[0]));
                pc = 0x800ba934;
            }
            0x800ba934 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x800ba938;
            }
            0x800ba938 => { ins += 1;
                if true { let t = c.ctr; if (0x800ba91c..0x800ba9e4).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800ba93c;
            }
            0x800ba93c => { ins += 1;
                c.r[11] = 0x3;
                pc = 0x800ba940;
            }
            0x800ba940 => { ins += 1;
                c.r[12] = 0x3;
                pc = 0x800ba944;
            }
            0x800ba944 => { ins += 1;
                pc = 0x800ba968; continue;
                pc = 0x800ba948;
            }
            0x800ba948 => { ins += 1;
                c.r[11] = 0x3;
                pc = 0x800ba94c;
            }
            0x800ba94c => { ins += 1;
                c.r[12] = 0x2;
                pc = 0x800ba950;
            }
            0x800ba950 => { ins += 1;
                pc = 0x800ba968; continue;
                pc = 0x800ba954;
            }
            0x800ba954 => { ins += 1;
                c.r[11] = 0x2;
                pc = 0x800ba958;
            }
            0x800ba958 => { ins += 1;
                c.r[12] = 0x2;
                pc = 0x800ba95c;
            }
            0x800ba95c => { ins += 1;
                pc = 0x800ba968; continue;
                pc = 0x800ba960;
            }
            0x800ba960 => { ins += 1;
                c.r[12] = 0x0;
                pc = 0x800ba964;
            }
            0x800ba964 => { ins += 1;
                c.r[11] = 0x0;
                pc = 0x800ba968;
            }
            0x800ba968 => { ins += 1;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x800ba96c;
            }
            0x800ba96c => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800ba974; continue; }
                pc = 0x800ba970;
            }
            0x800ba970 => { ins += 1;
                c.r[4] = 0x1;
                pc = 0x800ba974;
            }
            0x800ba974 => { ins += 1;
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x800ba978;
            }
            0x800ba978 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800ba980; continue; }
                pc = 0x800ba97c;
            }
            0x800ba97c => { ins += 1;
                c.r[5] = 0x1;
                pc = 0x800ba980;
            }
            0x800ba980 => { ins += 1;
                c.r[10] = 0x1;
                pc = 0x800ba984;
            }
            0x800ba984 => { ins += 1;
                c.r[9] = slw(c.r[10], c.r[11]);
                pc = 0x800ba988;
            }
            0x800ba988 => { ins += 1;
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x800ba98c;
            }
            0x800ba98c => { ins += 1;
                c.r[0] = c.r[9].wrapping_add(0xffffffff);
                pc = 0x800ba990;
            }
            0x800ba990 => { ins += 1;
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                pc = 0x800ba994;
            }
            0x800ba994 => { ins += 1;
                { let v = c.r[0] as i32; let n = c.r[11] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[0] = r as u32; }
                pc = 0x800ba998;
            }
            0x800ba998 => { ins += 1;
                c.r[4] = slw(c.r[10], c.r[12]);
                pc = 0x800ba99c;
            }
            0x800ba99c => { ins += 1;
                c.stw(c.r[6], c.r[0]);
                pc = 0x800ba9a0;
            }
            0x800ba9a0 => { ins += 1;
                c.r[5] = c.r[5].rotate_left(0) & 0xffff;
                pc = 0x800ba9a4;
            }
            0x800ba9a4 => { ins += 1;
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                pc = 0x800ba9a8;
            }
            0x800ba9a8 => { ins += 1;
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                pc = 0x800ba9ac;
            }
            0x800ba9ac => { ins += 1;
                { let v = c.r[0] as i32; let n = c.r[12] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[0] = r as u32; }
                pc = 0x800ba9b0;
            }
            0x800ba9b0 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x6);
                pc = 0x800ba9b4;
            }
            0x800ba9b4 => { ins += 1;
                c.stw(c.r[7], c.r[0]);
                pc = 0x800ba9b8;
            }
            0x800ba9b8 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800ba9c8; continue; }
                pc = 0x800ba9bc;
            }
            0x800ba9bc => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x16);
                pc = 0x800ba9c0;
            }
            0x800ba9c0 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800ba9c8; continue; }
                pc = 0x800ba9c4;
            }
            0x800ba9c4 => { ins += 1;
                c.r[10] = 0x0;
                pc = 0x800ba9c8;
            }
            0x800ba9c8 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[10], 0x0);
                pc = 0x800ba9cc;
            }
            0x800ba9cc => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800ba9d8; continue; }
                pc = 0x800ba9d0;
            }
            0x800ba9d0 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x800ba9d4;
            }
            0x800ba9d4 => { ins += 1;
                pc = 0x800ba9dc; continue;
                pc = 0x800ba9d8;
            }
            0x800ba9d8 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x800ba9dc;
            }
            0x800ba9dc => { ins += 1;
                c.stw(c.r[8], c.r[0]);
                pc = 0x800ba9e0;
            }
            0x800ba9e0 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba91c: bad pc {:#x}", pc),
        }
    }
}

/// GXInitTexObj (0x800ba9e4, 628 bytes)
#[inline(never)]
pub fn f_800ba9e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800ba9e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800ba9e4); p = true; }
    }
    f_800ba9e4_b(c, 0x800ba9e4);
    if p { crate::prof::exit(c, 0x800ba9e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ba9e4_at(c: &mut Cpu, pc: u32) { f_800ba9e4_b(c, pc) }
#[inline(never)]
fn f_800ba9e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800ba9e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800ba9e4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800ba9e4 => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x800ba9e8;
            }
            0x800ba9e8 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                pc = 0x800ba9ec;
            }
            0x800ba9ec => { ins += 1;
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800ba9f0;
            }
            0x800ba9f0 => { ins += 1;
                { let mut ea = c.r[1].wrapping_add(0x40); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x800ba9f4;
            }
            0x800ba9f4 => { ins += 1;
                c.r[27] = c.r[4].wrapping_add(0x0);
                pc = 0x800ba9f8;
            }
            0x800ba9f8 => { ins += 1;
                c.r[28] = c.r[5].wrapping_add(0x0);
                pc = 0x800ba9fc;
            }
            0x800ba9fc => { ins += 1;
                c.r[31] = c.r[3].wrapping_add(0x0);
                pc = 0x800baa00;
            }
            0x800baa00 => { ins += 1;
                c.r[29] = c.r[6].wrapping_add(0x0);
                pc = 0x800baa04;
            }
            0x800baa04 => { ins += 1;
                c.r[30] = c.r[7].wrapping_add(0x0);
                pc = 0x800baa08;
            }
            0x800baa08 => { ins += 1;
                c.r[24] = c.r[8].wrapping_add(0x0);
                pc = 0x800baa0c;
            }
            0x800baa0c => { ins += 1;
                c.r[25] = c.r[9].wrapping_add(0x0);
                pc = 0x800baa10;
            }
            0x800baa10 => { ins += 1;
                c.r[26] = c.r[10].wrapping_add(0x0);
                pc = 0x800baa14;
            }
            0x800baa14 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800baa18;
            }
            0x800baa18 => { ins += 1;
                c.r[5] = 0x20;
                pc = 0x800baa1c;
            }
            0x800baa1c => { ins += 1;
                c.lr = 0x800baa20; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800baa20;
            }
            0x800baa20 => { ins += 1;
                c.r[4] = c.lwz_t(ins, c.r[31]);
                pc = 0x800baa24;
            }
            0x800baa24 => { ins += 1;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x800baa28;
            }
            0x800baa28 => { ins += 1;
                c.r[3] = c.r[25].rotate_left(2) & 0xfffffffc;
                pc = 0x800baa2c;
            }
            0x800baa2c => { ins += 1;
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffffc;
                pc = 0x800baa30;
            }
            0x800baa30 => { ins += 1;
                c.r[4] = c.r[4] | c.r[24];
                pc = 0x800baa34;
            }
            0x800baa34 => { ins += 1;
                c.stw(c.r[31], c.r[4]);
                pc = 0x800baa38;
            }
            0x800baa38 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                pc = 0x800baa3c;
            }
            0x800baa3c => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffff3;
                pc = 0x800baa40;
            }
            0x800baa40 => { ins += 1;
                c.r[0] = c.r[0] | c.r[3];
                pc = 0x800baa44;
            }
            0x800baa44 => { ins += 1;
                c.stw(c.r[31], c.r[0]);
                pc = 0x800baa48;
            }
            0x800baa48 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                pc = 0x800baa4c;
            }
            0x800baa4c => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffef;
                pc = 0x800baa50;
            }
            0x800baa50 => { ins += 1;
                c.r[0] = c.r[0] | 0x10;
                pc = 0x800baa54;
            }
            0x800baa54 => { ins += 1;
                c.stw(c.r[31], c.r[0]);
                pc = 0x800baa58;
            }
            0x800baa58 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800baafc; continue; }
                pc = 0x800baa5c;
            }
            0x800baa5c => { ins += 1;
                c.r[3] = c.lbz_t(ins, c.r[31].wrapping_add(0x1f));
                pc = 0x800baa60;
            }
            0x800baa60 => { ins += 1;
                c.r[0] = c.r[30].wrapping_add(0xfffffff8);
                pc = 0x800baa64;
            }
            0x800baa64 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x2);
                pc = 0x800baa68;
            }
            0x800baa68 => { ins += 1;
                c.r[0] = c.r[3] | 0x1;
                pc = 0x800baa6c;
            }
            0x800baa6c => { ins += 1;
                c.stb(c.r[31].wrapping_add(0x1f), c.r[0]);
                pc = 0x800baa70;
            }
            0x800baa70 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x800baa88; continue; }
                pc = 0x800baa74;
            }
            0x800baa74 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                pc = 0x800baa78;
            }
            0x800baa78 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff1f;
                pc = 0x800baa7c;
            }
            0x800baa7c => { ins += 1;
                c.r[0] = c.r[0] | 0xa0;
                pc = 0x800baa80;
            }
            0x800baa80 => { ins += 1;
                c.stw(c.r[31], c.r[0]);
                pc = 0x800baa84;
            }
            0x800baa84 => { ins += 1;
                pc = 0x800baa98; continue;
                pc = 0x800baa88;
            }
            0x800baa88 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                pc = 0x800baa8c;
            }
            0x800baa8c => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff1f;
                pc = 0x800baa90;
            }
            0x800baa90 => { ins += 1;
                c.r[0] = c.r[0] | 0xc0;
                pc = 0x800baa94;
            }
            0x800baa94 => { ins += 1;
                c.stw(c.r[31], c.r[0]);
                pc = 0x800baa98;
            }
            0x800baa98 => { ins += 1;
                c.r[3] = c.r[28].rotate_left(0) & 0xffff;
                pc = 0x800baa9c;
            }
            0x800baa9c => { ins += 1;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                pc = 0x800baaa0;
            }
            0x800baaa0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                pc = 0x800baaa4;
            }
            0x800baaa4 => { ins += 1;
                if !cr_bit(cr, 1) { pc = 0x800baab4; continue; }
                pc = 0x800baaa8;
            }
            0x800baaa8 => { ins += 1;
                c.r[0] = c.r[3].leading_zeros();
                pc = 0x800baaac;
            }
            0x800baaac => { ins += 1;
                c.r[0] = c.addc(!c.r[0], 0x1f, 1);
                pc = 0x800baab0;
            }
            0x800baab0 => { ins += 1;
                pc = 0x800baabc; continue;
                pc = 0x800baab4;
            }
            0x800baab4 => { ins += 1;
                c.r[0] = c.r[0].leading_zeros();
                pc = 0x800baab8;
            }
            0x800baab8 => { ins += 1;
                c.r[0] = c.addc(!c.r[0], 0x1f, 1);
                pc = 0x800baabc;
            }
            0x800baabc => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x3c), c.r[0]);
                pc = 0x800baac0;
            }
            0x800baac0 => { ins += 1;
                c.r[0] = 0x43300000;
                pc = 0x800baac4;
            }
            0x800baac4 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                pc = 0x800baac8;
            }
            0x800baac8 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x38), c.r[0]);
                pc = 0x800baacc;
            }
            0x800baacc => { ins += 1;
                { let ea = c.r[2].wrapping_add(0xffff8cb0); c.f[1] = c.lfd(ea); }
                pc = 0x800baad0;
            }
            0x800baad0 => { ins += 1;
                c.r[3] = c.r[3].rotate_left(0) & 0xffff00ff;
                pc = 0x800baad4;
            }
            0x800baad4 => { ins += 1;
                { let ea = c.r[1].wrapping_add(0x38); c.f[0] = c.lfd(ea); }
                pc = 0x800baad8;
            }
            0x800baad8 => { ins += 1;
                { let ea = c.r[2].wrapping_add(0xffff8ca8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800baadc;
            }
            0x800baadc => { ins += 1;
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x800baae0;
            }
            0x800baae0 => { ins += 1;
                { let v = s(c.f[2] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x800baae4;
            }
            0x800baae4 => { ins += 1;
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x800baae8;
            }
            0x800baae8 => { ins += 1;
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x800baaec;
            }
            0x800baaec => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                pc = 0x800baaf0;
            }
            0x800baaf0 => { ins += 1;
                c.r[3] = (c.r[0].rotate_left(8) & 0xff00) | (c.r[3] & 0xffff00ff);
                pc = 0x800baaf4;
            }
            0x800baaf4 => { ins += 1;
                c.stw(c.r[31].wrapping_add(0x4), c.r[3]);
                pc = 0x800baaf8;
            }
            0x800baaf8 => { ins += 1;
                pc = 0x800bab0c; continue;
                pc = 0x800baafc;
            }
            0x800baafc => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                pc = 0x800bab00;
            }
            0x800bab00 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff1f;
                pc = 0x800bab04;
            }
            0x800bab04 => { ins += 1;
                c.r[0] = c.r[0] | 0x80;
                pc = 0x800bab08;
            }
            0x800bab08 => { ins += 1;
                c.stw(c.r[31], c.r[0]);
                pc = 0x800bab0c;
            }
            0x800bab0c => { ins += 1;
                c.stw(c.r[31].wrapping_add(0x14), c.r[30]);
                pc = 0x800bab10;
            }
            0x800bab10 => { ins += 1;
                c.r[3] = c.r[29].rotate_left(0) & 0xffff;
                pc = 0x800bab14;
            }
            0x800bab14 => { ins += 1;
                c.r[7] = c.r[30].rotate_left(0) & 0xf;
                pc = 0x800bab18;
            }
            0x800bab18 => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                pc = 0x800bab1c;
            }
            0x800bab1c => { ins += 1;
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                pc = 0x800bab20;
            }
            0x800bab20 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x800bab24;
            }
            0x800bab24 => { ins += 1;
                c.r[6] = c.r[5].rotate_left(0) & 0xfffffc00;
                pc = 0x800bab28;
            }
            0x800bab28 => { ins += 1;
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                pc = 0x800bab2c;
            }
            0x800bab2c => { ins += 1;
                c.r[5] = c.r[6] | c.r[5];
                pc = 0x800bab30;
            }
            0x800bab30 => { ins += 1;
                c.stw(c.r[31].wrapping_add(0x8), c.r[5]);
                pc = 0x800bab34;
            }
            0x800bab34 => { ins += 1;
                c.r[5] = c.r[0].rotate_left(10) & 0xfffffc00;
                pc = 0x800bab38;
            }
            0x800bab38 => { ins += 1;
                c.r[0] = c.r[27].rotate_left(27) & 0x1ffffff;
                pc = 0x800bab3c;
            }
            0x800bab3c => { ins += 1;
                c.r[6] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                pc = 0x800bab40;
            }
            0x800bab40 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], 0xe);
                pc = 0x800bab44;
            }
            0x800bab44 => { ins += 1;
                c.r[6] = c.r[6].rotate_left(0) & 0xfff003ff;
                pc = 0x800bab48;
            }
            0x800bab48 => { ins += 1;
                c.r[5] = c.r[6] | c.r[5];
                pc = 0x800bab4c;
            }
            0x800bab4c => { ins += 1;
                c.stw(c.r[31].wrapping_add(0x8), c.r[5]);
                pc = 0x800bab50;
            }
            0x800bab50 => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                pc = 0x800bab54;
            }
            0x800bab54 => { ins += 1;
                c.r[5] = c.r[5].rotate_left(0) & 0xff0fffff;
                pc = 0x800bab58;
            }
            0x800bab58 => { ins += 1;
                c.r[5] = (c.r[30].rotate_left(20) & 0xf00000) | (c.r[5] & 0xff0fffff);
                pc = 0x800bab5c;
            }
            0x800bab5c => { ins += 1;
                c.stw(c.r[31].wrapping_add(0x8), c.r[5]);
                pc = 0x800bab60;
            }
            0x800bab60 => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                pc = 0x800bab64;
            }
            0x800bab64 => { ins += 1;
                c.r[5] = c.r[5].rotate_left(0) & 0xffe00000;
                pc = 0x800bab68;
            }
            0x800bab68 => { ins += 1;
                c.r[0] = c.r[5] | c.r[0];
                pc = 0x800bab6c;
            }
            0x800bab6c => { ins += 1;
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                pc = 0x800bab70;
            }
            0x800bab70 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x800babf0; continue; }
                pc = 0x800bab74;
            }
            0x800bab74 => { ins += 1;
                c.r[5] = 0x80310000;
                pc = 0x800bab78;
            }
            0x800bab78 => { ins += 1;
                c.r[5] = c.r[5].wrapping_add(0x3550);
                pc = 0x800bab7c;
            }
            0x800bab7c => { ins += 1;
                c.r[0] = c.r[7].rotate_left(2) & 0xfffffffc;
                pc = 0x800bab80;
            }
            0x800bab80 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(c.r[0]));
                pc = 0x800bab84;
            }
            0x800bab84 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x800bab88;
            }
            0x800bab88 => { ins += 1;
                if true { let t = c.ctr; if (0x800ba9e4..0x800bac58).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800bab8c;
            }
            0x800bab8c => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x800bab90;
            }
            0x800bab90 => { ins += 1;
                c.stb(c.r[31].wrapping_add(0x1e), c.r[0]);
                pc = 0x800bab94;
            }
            0x800bab94 => { ins += 1;
                c.r[0] = 0x3;
                pc = 0x800bab98;
            }
            0x800bab98 => { ins += 1;
                c.r[7] = 0x3;
                pc = 0x800bab9c;
            }
            0x800bab9c => { ins += 1;
                pc = 0x800bac00; continue;
                pc = 0x800baba0;
            }
            0x800baba0 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x800baba4;
            }
            0x800baba4 => { ins += 1;
                c.stb(c.r[31].wrapping_add(0x1e), c.r[0]);
                pc = 0x800baba8;
            }
            0x800baba8 => { ins += 1;
                c.r[0] = 0x3;
                pc = 0x800babac;
            }
            0x800babac => { ins += 1;
                c.r[7] = 0x2;
                pc = 0x800babb0;
            }
            0x800babb0 => { ins += 1;
                pc = 0x800bac00; continue;
                pc = 0x800babb4;
            }
            0x800babb4 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x800babb8;
            }
            0x800babb8 => { ins += 1;
                c.stb(c.r[31].wrapping_add(0x1e), c.r[0]);
                pc = 0x800babbc;
            }
            0x800babbc => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x800babc0;
            }
            0x800babc0 => { ins += 1;
                c.r[7] = 0x2;
                pc = 0x800babc4;
            }
            0x800babc4 => { ins += 1;
                pc = 0x800bac00; continue;
                pc = 0x800babc8;
            }
            0x800babc8 => { ins += 1;
                c.r[0] = 0x3;
                pc = 0x800babcc;
            }
            0x800babcc => { ins += 1;
                c.stb(c.r[31].wrapping_add(0x1e), c.r[0]);
                pc = 0x800babd0;
            }
            0x800babd0 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x800babd4;
            }
            0x800babd4 => { ins += 1;
                c.r[7] = 0x2;
                pc = 0x800babd8;
            }
            0x800babd8 => { ins += 1;
                pc = 0x800bac00; continue;
                pc = 0x800babdc;
            }
            0x800babdc => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800babe0;
            }
            0x800babe0 => { ins += 1;
                c.stb(c.r[31].wrapping_add(0x1e), c.r[0]);
                pc = 0x800babe4;
            }
            0x800babe4 => { ins += 1;
                c.r[0] = 0x3;
                pc = 0x800babe8;
            }
            0x800babe8 => { ins += 1;
                c.r[7] = 0x3;
                pc = 0x800babec;
            }
            0x800babec => { ins += 1;
                pc = 0x800bac00; continue;
                pc = 0x800babf0;
            }
            0x800babf0 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x800babf4;
            }
            0x800babf4 => { ins += 1;
                c.stb(c.r[31].wrapping_add(0x1e), c.r[0]);
                pc = 0x800babf8;
            }
            0x800babf8 => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x800babfc;
            }
            0x800babfc => { ins += 1;
                c.r[7] = 0x2;
                pc = 0x800bac00;
            }
            0x800bac00 => { ins += 1;
                c.r[8] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800bac04;
            }
            0x800bac04 => { ins += 1;
                c.r[6] = 0x1;
                pc = 0x800bac08;
            }
            0x800bac08 => { ins += 1;
                c.r[5] = slw(c.r[6], c.r[8]);
                pc = 0x800bac0c;
            }
            0x800bac0c => { ins += 1;
                c.r[7] = c.r[7].rotate_left(0) & 0xffff;
                pc = 0x800bac10;
            }
            0x800bac10 => { ins += 1;
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                pc = 0x800bac14;
            }
            0x800bac14 => { ins += 1;
                c.r[5] = slw(c.r[6], c.r[7]);
                pc = 0x800bac18;
            }
            0x800bac18 => { ins += 1;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                pc = 0x800bac1c;
            }
            0x800bac1c => { ins += 1;
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                pc = 0x800bac20;
            }
            0x800bac20 => { ins += 1;
                { let v = c.r[4] as i32; let n = c.r[8] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[4] = r as u32; }
                pc = 0x800bac24;
            }
            0x800bac24 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x800bac28;
            }
            0x800bac28 => { ins += 1;
                { let v = c.r[0] as i32; let n = c.r[7] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[0] = r as u32; }
                pc = 0x800bac2c;
            }
            0x800bac2c => { ins += 1;
                c.r[0] = c.r[4].wrapping_mul(c.r[0]);
                pc = 0x800bac30;
            }
            0x800bac30 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0x7fff;
                pc = 0x800bac34;
            }
            0x800bac34 => { ins += 1;
                c.sth(c.r[31].wrapping_add(0x1c), c.r[0]);
                pc = 0x800bac38;
            }
            0x800bac38 => { ins += 1;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x1f));
                pc = 0x800bac3c;
            }
            0x800bac3c => { ins += 1;
                c.r[0] = c.r[0] | 0x2;
                pc = 0x800bac40;
            }
            0x800bac40 => { ins += 1;
                c.stb(c.r[31].wrapping_add(0x1f), c.r[0]);
                pc = 0x800bac44;
            }
            0x800bac44 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x64));
                pc = 0x800bac48;
            }
            0x800bac48 => { ins += 1;
                { let mut ea = c.r[1].wrapping_add(0x40); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                pc = 0x800bac4c;
            }
            0x800bac4c => { ins += 1;
                c.r[1] = c.r[1].wrapping_add(0x60);
                pc = 0x800bac50;
            }
            0x800bac50 => { ins += 1;
                c.mtspr(8, c.r[0]);
                pc = 0x800bac54;
            }
            0x800bac54 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800ba9e4: bad pc {:#x}", pc),
        }
    }
}

/// GXInitTexObjCI (0x800bac58, 72 bytes)
#[inline(never)]
pub fn f_800bac58(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bac58) { return; }
        if c.prof { crate::prof::enter(c, 0x800bac58); p = true; }
    }
    f_800bac58_b(c, 0x800bac58);
    if p { crate::prof::exit(c, 0x800bac58); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bac58_at(c: &mut Cpu, pc: u32) { f_800bac58_b(c, pc) }
#[inline(always)]
fn f_800bac58_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bac58;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bac58, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bac58 => { ins += 8;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x38));
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800bac78; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ba9e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bac78;
            }
            0x800bac78 => { ins += 10;
                c.r[0] = c.lbz_t(ins, c.r[30].wrapping_add(0x1f));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffd;
                c.stb(c.r[30].wrapping_add(0x1f), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x18), c.r[31]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x28));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bac58: bad pc {:#x}", pc),
        }
    }
}

/// GXInitTexObjLOD (0x800baca0, 404 bytes)
#[inline(never)]
pub fn f_800baca0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800baca0) { return; }
        if c.prof { crate::prof::enter(c, 0x800baca0); p = true; }
    }
    f_800baca0_b(c, 0x800baca0);
    if p { crate::prof::exit(c, 0x800baca0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800baca0_at(c: &mut Cpu, pc: u32) { f_800baca0_b(c, pc) }
#[inline(always)]
fn f_800baca0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800baca0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800baca0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800baca0 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[2].wrapping_add(0xffff8cb8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[3], c.f[0]);
                if !cr_bit(cr, 0) { pc = 0x800bacb8; continue; }
                pc = 0x800bacb0;
            }
            0x800bacb0 => { ins += 2;
                c.f[3] = c.f[0];
                pc = 0x800baccc; continue;
                pc = 0x800bacb8;
            }
            0x800bacb8 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff8cbc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[3], c.f[0]);
                { let (a, b) = (cr_bit(cr, 1), cr_bit(cr, 2)); cr_set_bit(&mut cr, 2, a | b); }
                if !cr_bit(cr, 2) { pc = 0x800baccc; continue; }
                pc = 0x800bacc8;
            }
            0x800bacc8 => { ins += 1;
                { let ea = c.r[2].wrapping_add(0xffff8cc0); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                pc = 0x800baccc;
            }
            0x800baccc => { ins += 11;
                { let ea = c.r[2].wrapping_add(0xffff8cc4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x1);
                c.r[0] = c.lwz_t(ins, c.r[3]);
                { let v = s(c.f[0] * c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                c.r[5] = c.r[0].rotate_left(0) & 0xfffe01ff;
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.r[5] = (c.r[0].rotate_left(9) & 0x1fe00) | (c.r[5] & 0xfffe01ff);
                c.stw(c.r[3], c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x800bad00; continue; }
                pc = 0x800bacf8;
            }
            0x800bacf8 => { ins += 2;
                c.r[5] = 0x1;
                pc = 0x800bad04; continue;
                pc = 0x800bad00;
            }
            0x800bad00 => { ins += 1;
                c.r[5] = 0x0;
                pc = 0x800bad04;
            }
            0x800bad04 => { ins += 14;
                c.r[9] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.r[7].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[5] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[7] = c.r[9].rotate_left(0) & 0xffffffef;
                c.r[5] = c.r[7] | c.r[5];
                c.stw(c.r[3], c.r[5]);
                c.r[5] = c.r[13].wrapping_add(0xffff8290);
                c.r[0] = c.lbz_t(ins, c.r[5].wrapping_add(c.r[4]));
                c.r[7] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.r[0].rotate_left(5) & 0xffffffe0;
                c.r[4] = c.r[7].rotate_left(0) & 0xffffff1f;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[3], c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800bad44; continue; }
                pc = 0x800bad3c;
            }
            0x800bad3c => { ins += 2;
                c.r[0] = 0x0;
                pc = 0x800bad48; continue;
                pc = 0x800bad44;
            }
            0x800bad44 => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x800bad48;
            }
            0x800bad48 => { ins += 24;
                c.r[4] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.r[0].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffeff;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[3], c.r[0]);
                c.r[4] = c.r[8].rotate_left(19) & 0xfff80000;
                c.r[0] = c.r[6].rotate_left(21) & 0x1fe00000;
                c.r[5] = c.lwz_t(ins, c.r[3]);
                c.r[5] = c.r[5].rotate_left(0) & 0xfffdffff;
                c.stw(c.r[3], c.r[5]);
                c.r[5] = c.lwz_t(ins, c.r[3]);
                c.r[5] = c.r[5].rotate_left(0) & 0xfffbffff;
                c.stw(c.r[3], c.r[5]);
                c.r[5] = c.lwz_t(ins, c.r[3]);
                c.r[5] = c.r[5].rotate_left(0) & 0xffe7ffff;
                c.r[4] = c.r[5] | c.r[4];
                c.stw(c.r[3], c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[3]);
                c.r[4] = c.r[4].rotate_left(0) & 0xffdfffff;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[3], c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff8cc8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if !cr_bit(cr, 0) { pc = 0x800badb0; continue; }
                pc = 0x800bada8;
            }
            0x800bada8 => { ins += 2;
                c.f[1] = c.f[0];
                pc = 0x800badc0; continue;
                pc = 0x800badb0;
            }
            0x800badb0 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff8ccc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if !cr_bit(cr, 1) { pc = 0x800badc0; continue; }
                pc = 0x800badbc;
            }
            0x800badbc => { ins += 1;
                c.f[1] = c.f[0];
                pc = 0x800badc0;
            }
            0x800badc0 => { ins += 8;
                { let ea = c.r[2].wrapping_add(0xffff8ca8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[2].wrapping_add(0xffff8cc8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[2], c.f[0]);
                c.f[1] = fctiwz(c.f[1]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[1]; c.stfd(ea, v); }
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                if !cr_bit(cr, 0) { pc = 0x800bade8; continue; }
                pc = 0x800bade0;
            }
            0x800bade0 => { ins += 2;
                c.f[2] = c.f[0];
                pc = 0x800badf8; continue;
                pc = 0x800bade8;
            }
            0x800bade8 => { ins += 3;
                { let ea = c.r[2].wrapping_add(0xffff8ccc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[2], c.f[0]);
                if !cr_bit(cr, 1) { pc = 0x800badf8; continue; }
                pc = 0x800badf4;
            }
            0x800badf4 => { ins += 1;
                c.f[2] = c.f[0];
                pc = 0x800badf8;
            }
            0x800badf8 => { ins += 15;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff00;
                c.r[0] = (c.r[4].rotate_left(0) & 0xff) | (c.r[0] & 0xffffff00);
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff8ca8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                { let v = s(c.f[0] * c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                c.r[4] = c.r[0].rotate_left(0) & 0xffff00ff;
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.r[4] = (c.r[0].rotate_left(8) & 0xff00) | (c.r[4] & 0xffff00ff);
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                c.r[1] = c.r[1].wrapping_add(0x38);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800baca0: bad pc {:#x}", pc),
        }
    }
}

/// GXInitTexObjWrapMode (0x800bae34, 40 bytes)
#[inline(never)]
pub fn f_800bae34(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bae34) { return; }
        if c.prof { crate::prof::enter(c, 0x800bae34); p = true; }
    }
    f_800bae34_b(c, 0x800bae34);
    if p { crate::prof::exit(c, 0x800bae34); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bae34_at(c: &mut Cpu, pc: u32) { f_800bae34_b(c, pc) }
#[inline(always)]
fn f_800bae34_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bae34;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bae34, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bae34 => { ins += 10;
                c.r[6] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[5] = c.r[6].rotate_left(0) & 0xfffffffc;
                c.r[4] = c.r[5] | c.r[4];
                c.stw(c.r[3], c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[3]);
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff3;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[3], c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bae34: bad pc {:#x}", pc),
        }
    }
}

/// GXGetTexObjFmt (0x800bae5c, 8 bytes)
#[inline(never)]
pub fn f_800bae5c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bae5c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bae5c); p = true; }
    }
    f_800bae5c_b(c, 0x800bae5c);
    if p { crate::prof::exit(c, 0x800bae5c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bae5c_at(c: &mut Cpu, pc: u32) { f_800bae5c_b(c, pc) }
#[inline(always)]
fn f_800bae5c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bae5c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bae5c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bae5c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x14));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bae5c: bad pc {:#x}", pc),
        }
    }
}

/// GXLoadTexObjPreLoaded (0x800bae64, 408 bytes)
#[inline(never)]
pub fn f_800bae64(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bae64) { return; }
        if c.prof { crate::prof::enter(c, 0x800bae64); p = true; }
    }
    f_800bae64_b(c, 0x800bae64);
    if p { crate::prof::exit(c, 0x800bae64); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bae64_at(c: &mut Cpu, pc: u32) { f_800bae64_b(c, pc) }
#[inline(always)]
fn f_800bae64_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bae64;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bae64, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bae64 => { ins += 68;
                c.r[0] = c.mfspr(8);
                c.r[7] = c.r[13].wrapping_add(0xffff8268);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = 0xcc010000;
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = 0x61;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.r[29] = c.r[5].wrapping_add(0x0);
                c.r[5] = c.r[13].wrapping_add(0xffff8278);
                c.stw(c.r[1].wrapping_add(0x18), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[6] = c.lwz_t(ins, c.r[3]);
                c.r[3] = c.r[13].wrapping_add(0xffff8258);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[29]));
                c.r[3] = c.r[13].wrapping_add(0xffff8260);
                c.r[0] = c.r[0].rotate_left(24) & 0xff000000;
                c.r[0] = (c.r[6].rotate_left(0) & 0xffffff) | (c.r[0] & 0xff000000);
                c.stw(c.r[28], c.r[0]);
                c.r[6] = c.r[13].wrapping_add(0xffff8270);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[29]));
                c.r[3] = c.r[13].wrapping_add(0xffff8280);
                c.r[8] = c.lwz_t(ins, c.r[28].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(24) & 0xff000000;
                c.r[0] = (c.r[8].rotate_left(0) & 0xffffff) | (c.r[0] & 0xff000000);
                c.stw(c.r[28].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[7].wrapping_add(c.r[29]));
                c.r[8] = c.lwz_t(ins, c.r[28].wrapping_add(0x8));
                c.r[0] = c.r[0].rotate_left(24) & 0xff000000;
                c.r[0] = (c.r[8].rotate_left(0) & 0xffffff) | (c.r[0] & 0xff000000);
                c.stw(c.r[28].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[6].wrapping_add(c.r[29]));
                c.r[7] = c.lwz_t(ins, c.r[4]);
                c.r[0] = c.r[0].rotate_left(24) & 0xff000000;
                c.r[0] = (c.r[7].rotate_left(0) & 0xffffff) | (c.r[0] & 0xff000000);
                c.stw(c.r[4], c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[5].wrapping_add(c.r[29]));
                c.r[6] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(24) & 0xff000000;
                c.r[0] = (c.r[6].rotate_left(0) & 0xffffff) | (c.r[0] & 0xff000000);
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(c.r[29]));
                c.r[5] = c.lwz_t(ins, c.r[28].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(24) & 0xff000000;
                c.r[0] = (c.r[5].rotate_left(0) & 0xffffff) | (c.r[0] & 0xff000000);
                c.stw(c.r[28].wrapping_add(0xc), c.r[0]);
                c.stb(c.r[31].wrapping_add(0xffff8000), c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[28]);
                c.stw(c.r[31].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[31].wrapping_add(0xffff8000), c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[28].wrapping_add(0x4));
                c.stw(c.r[31].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[31].wrapping_add(0xffff8000), c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[28].wrapping_add(0x8));
                c.stw(c.r[31].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[31].wrapping_add(0xffff8000), c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.stw(c.r[31].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[31].wrapping_add(0xffff8000), c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                c.stw(c.r[31].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[31].wrapping_add(0xffff8000), c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[28].wrapping_add(0xc));
                c.stw(c.r[31].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[28].wrapping_add(0x1f));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800bafac; continue; }
                pc = 0x800baf74;
            }
            0x800baf74 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[3] = c.lwz_t(ins, c.r[28].wrapping_add(0x18));
                c.r[12] = c.lwz_t(ins, c.r[4].wrapping_add(0x414));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800baf88; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800baf88;
            }
            0x800baf88 => { ins += 9;
                c.r[4] = c.r[13].wrapping_add(0xffff8288);
                c.r[5] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[0] = c.lbz_t(ins, c.r[4].wrapping_add(c.r[29]));
                c.r[0] = c.r[0].rotate_left(24) & 0xff000000;
                c.r[0] = (c.r[5].rotate_left(0) & 0xffffff) | (c.r[0] & 0xff000000);
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                c.stb(c.r[31].wrapping_add(0xffff8000), c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.stw(c.r[31].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bafac;
            }
            0x800bafac => { ins += 20;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[4] = c.r[29].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz_t(ins, c.r[28].wrapping_add(0x8));
                c.r[0] = 0x0;
                c.r[4] = c.r[5].wrapping_add(c.r[4]);
                c.stw(c.r[4].wrapping_add(0x45c), c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[28]);
                c.stw(c.r[4].wrapping_add(0x47c), c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[5].wrapping_add(0x4f4));
                c.r[3] = c.r[3] | 0x1;
                c.stw(c.r[5].wrapping_add(0x4f4), c.r[3]);
                c.sth(c.r[5].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bae64: bad pc {:#x}", pc),
        }
    }
}

/// GXLoadTexObj (0x800baffc, 84 bytes)
#[inline(never)]
pub fn f_800baffc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800baffc) { return; }
        if c.prof { crate::prof::enter(c, 0x800baffc); p = true; }
    }
    f_800baffc_b(c, 0x800baffc);
    if p { crate::prof::exit(c, 0x800baffc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800baffc_at(c: &mut Cpu, pc: u32) { f_800baffc_b(c, pc) }
#[inline(always)]
fn f_800baffc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800baffc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800baffc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800baffc => { ins += 11;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[12] = c.lwz_t(ins, c.r[5].wrapping_add(0x410));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800bb028; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800bb028;
            }
            0x800bb028 => { ins += 4;
                c.r[4] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.r[5] = c.r[31].wrapping_add(0x0);
                c.lr = 0x800bb038; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bae64(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bb038;
            }
            0x800bb038 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800baffc: bad pc {:#x}", pc),
        }
    }
}

/// GXInitTlutObj (0x800bb050, 72 bytes)
#[inline(never)]
pub fn f_800bb050(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb050) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb050); p = true; }
    }
    f_800bb050_b(c, 0x800bb050);
    if p { crate::prof::exit(c, 0x800bb050); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb050_at(c: &mut Cpu, pc: u32) { f_800bb050_b(c, pc) }
#[inline(always)]
fn f_800bb050_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb050;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb050, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb050 => { ins += 18;
                c.r[0] = 0x0;
                c.stw(c.r[3], c.r[0]);
                c.r[5] = c.r[5].rotate_left(10) & 0xfffffc00;
                c.r[0] = c.r[4].rotate_left(27) & 0x1ffffff;
                c.r[7] = c.lwz_t(ins, c.r[3]);
                c.r[4] = c.r[7].rotate_left(0) & 0xfffff3ff;
                c.r[4] = c.r[4] | c.r[5];
                c.stw(c.r[3], c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[4] = c.r[4].rotate_left(0) & 0xffe00000;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff;
                c.r[0] = c.r[0] | 0x64000000;
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                c.sth(c.r[3].wrapping_add(0x8), c.r[6]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb050: bad pc {:#x}", pc),
        }
    }
}

/// GXLoadTlut (0x800bb098, 156 bytes)
#[inline(never)]
pub fn f_800bb098(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb098) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb098); p = true; }
    }
    f_800bb098_b(c, 0x800bb098);
    if p { crate::prof::exit(c, 0x800bb098); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb098_at(c: &mut Cpu, pc: u32) { f_800bb098_b(c, pc) }
#[inline(always)]
fn f_800bb098_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb098;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb098, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb098 => { ins += 11;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[4].wrapping_add(0x0);
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[12] = c.lwz_t(ins, c.r[5].wrapping_add(0x414));
                c.mtspr(8, c.r[12]);
                if true { let t = c.lr; c.lr = 0x800bb0c4; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800bb0c4;
            }
            0x800bb0c4 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800bb0cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bc0f0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bb0cc;
            }
            0x800bb0cc => { ins += 9;
                c.r[4] = 0x61;
                c.r[3] = 0xcc010000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[31]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.lr = 0x800bb0f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bc0f0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bb0f0;
            }
            0x800bb0f0 => { ins += 17;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                c.r[3] = c.lwz_t(ins, c.r[30]);
                c.r[0] = c.r[0].rotate_left(0) & 0x3ff;
                c.r[0] = (c.r[3].rotate_left(0) & 0xfffffc00) | (c.r[0] & 0x3ff);
                c.stw(c.r[30], c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                c.stw(c.r[31].wrapping_add(0x4), c.r[3]);
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb098: bad pc {:#x}", pc),
        }
    }
}

/// GXInitTexCacheRegion (0x800bb134, 288 bytes)
#[inline(never)]
pub fn f_800bb134(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb134) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb134); p = true; }
    }
    f_800bb134_b(c, 0x800bb134);
    if p { crate::prof::exit(c, 0x800bb134); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb134_at(c: &mut Cpu, pc: u32) { f_800bb134_b(c, pc) }
#[inline(always)]
fn f_800bb134_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb134;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb134, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb134 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x1);
                if cr_bit(cr, 2) { pc = 0x800bb160; continue; }
                pc = 0x800bb13c;
            }
            0x800bb13c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bb14c; continue; }
                pc = 0x800bb140;
            }
            0x800bb140 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x0);
                if !cr_bit(cr, 0) { pc = 0x800bb158; continue; }
                pc = 0x800bb148;
            }
            0x800bb148 => { ins += 1;
                pc = 0x800bb16c; continue;
                pc = 0x800bb14c;
            }
            0x800bb14c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x3);
                if !cr_bit(cr, 0) { pc = 0x800bb16c; continue; }
                pc = 0x800bb154;
            }
            0x800bb154 => { ins += 1;
                pc = 0x800bb168; continue;
                pc = 0x800bb158;
            }
            0x800bb158 => { ins += 2;
                c.r[10] = 0x3;
                pc = 0x800bb16c; continue;
                pc = 0x800bb160;
            }
            0x800bb160 => { ins += 2;
                c.r[10] = 0x4;
                pc = 0x800bb16c; continue;
                pc = 0x800bb168;
            }
            0x800bb168 => { ins += 1;
                c.r[10] = 0x5;
                pc = 0x800bb16c;
            }
            0x800bb16c => { ins += 22;
                c.r[0] = 0x0;
                c.stw(c.r[3], c.r[0]);
                c.r[6] = c.r[5].rotate_left(27) & 0x7ffffff;
                c.r[5] = c.r[10].rotate_left(15) & 0xffff8000;
                c.r[9] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.r[10].rotate_left(18) & 0xfffc0000;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x2);
                c.r[9] = c.r[9].rotate_left(0) & 0xffff8000;
                c.r[6] = c.r[9] | c.r[6];
                c.stw(c.r[3], c.r[6]);
                c.r[6] = c.lwz_t(ins, c.r[3]);
                c.r[6] = c.r[6].rotate_left(0) & 0xfffc7fff;
                c.r[5] = c.r[6] | c.r[5];
                c.stw(c.r[3], c.r[5]);
                c.r[5] = c.lwz_t(ins, c.r[3]);
                c.r[5] = c.r[5].rotate_left(0) & 0xffe3ffff;
                c.r[0] = c.r[5] | c.r[0];
                c.stw(c.r[3], c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffdfffff;
                c.stw(c.r[3], c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800bb1f4; continue; }
                pc = 0x800bb1c4;
            }
            0x800bb1c4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bb1d8; continue; }
                pc = 0x800bb1c8;
            }
            0x800bb1c8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bb1e4; continue; }
                pc = 0x800bb1d0;
            }
            0x800bb1d0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bb1ec; continue; }
                pc = 0x800bb1d4;
            }
            0x800bb1d4 => { ins += 1;
                pc = 0x800bb200; continue;
                pc = 0x800bb1d8;
            }
            0x800bb1d8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[8], 0x4);
                if !cr_bit(cr, 0) { pc = 0x800bb200; continue; }
                pc = 0x800bb1e0;
            }
            0x800bb1e0 => { ins += 1;
                pc = 0x800bb1fc; continue;
                pc = 0x800bb1e4;
            }
            0x800bb1e4 => { ins += 2;
                c.r[10] = 0x3;
                pc = 0x800bb200; continue;
                pc = 0x800bb1ec;
            }
            0x800bb1ec => { ins += 2;
                c.r[10] = 0x4;
                pc = 0x800bb200; continue;
                pc = 0x800bb1f4;
            }
            0x800bb1f4 => { ins += 2;
                c.r[10] = 0x5;
                pc = 0x800bb200; continue;
                pc = 0x800bb1fc;
            }
            0x800bb1fc => { ins += 1;
                c.r[10] = 0x0;
                pc = 0x800bb200;
            }
            0x800bb200 => { ins += 21;
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                c.r[7] = c.r[7].rotate_left(27) & 0x7ffffff;
                c.r[6] = c.r[10].rotate_left(15) & 0xffff8000;
                c.r[8] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[5] = c.r[10].rotate_left(18) & 0xfffc0000;
                c.r[0] = 0x1;
                c.r[8] = c.r[8].rotate_left(0) & 0xffff8000;
                c.r[7] = c.r[8] | c.r[7];
                c.stw(c.r[3].wrapping_add(0x4), c.r[7]);
                c.r[7] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[7] = c.r[7].rotate_left(0) & 0xfffc7fff;
                c.r[6] = c.r[7] | c.r[6];
                c.stw(c.r[3].wrapping_add(0x4), c.r[6]);
                c.r[6] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[6] = c.r[6].rotate_left(0) & 0xffe3ffff;
                c.r[5] = c.r[6] | c.r[5];
                c.stw(c.r[3].wrapping_add(0x4), c.r[5]);
                c.stb(c.r[3].wrapping_add(0xc), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xd), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb134: bad pc {:#x}", pc),
        }
    }
}

/// GXInitTlutRegion (0x800bb254, 72 bytes)
#[inline(never)]
pub fn f_800bb254(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb254) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb254); p = true; }
    }
    f_800bb254_b(c, 0x800bb254);
    if p { crate::prof::exit(c, 0x800bb254); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb254_at(c: &mut Cpu, pc: u32) { f_800bb254_b(c, pc) }
#[inline(always)]
fn f_800bb254_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb254;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb254, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb254 => { ins += 18;
                c.r[0] = 0x0;
                c.stw(c.r[3], c.r[0]);
                c.r[0] = c.r[4].wrapping_add(0xfff80000);
                c.r[4] = c.r[0].rotate_left(23) & 0x7fffff;
                c.r[6] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.r[5].rotate_left(10) & 0xfffffc00;
                c.r[5] = c.r[6].rotate_left(0) & 0xfffffc00;
                c.r[4] = c.r[5] | c.r[4];
                c.stw(c.r[3], c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[3]);
                c.r[4] = c.r[4].rotate_left(0) & 0xffe003ff;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[3], c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff;
                c.r[0] = c.r[0] | 0x65000000;
                c.stw(c.r[3], c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb254: bad pc {:#x}", pc),
        }
    }
}

/// GXInvalidateTexAll (0x800bb29c, 72 bytes)
#[inline(never)]
pub fn f_800bb29c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb29c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb29c); p = true; }
    }
    f_800bb29c_b(c, 0x800bb29c);
    if p { crate::prof::exit(c, 0x800bb29c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb29c_at(c: &mut Cpu, pc: u32) { f_800bb29c_b(c, pc) }
#[inline(always)]
fn f_800bb29c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb29c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb29c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb29c => { ins += 4;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800bb2ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bc0f0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bb2ac;
            }
            0x800bb2ac => { ins += 10;
                c.r[6] = 0x61;
                c.r[3] = 0x66000000;
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[6]);
                c.r[4] = c.r[3].wrapping_add(0x1000);
                c.r[0] = c.r[3].wrapping_add(0x1100);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[4]);
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[6]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.lr = 0x800bb2d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bc0f0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bb2d4;
            }
            0x800bb2d4 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb29c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTexRegionCallback (0x800bb2e4, 20 bytes)
#[inline(never)]
pub fn f_800bb2e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb2e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb2e4); p = true; }
    }
    f_800bb2e4_b(c, 0x800bb2e4);
    if p { crate::prof::exit(c, 0x800bb2e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb2e4_at(c: &mut Cpu, pc: u32) { f_800bb2e4_b(c, pc) }
#[inline(always)]
fn f_800bb2e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb2e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb2e4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb2e4 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                { let ea = c.r[4].wrapping_add(0x410); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[4], c.r[3]);
                c.r[3] = c.r[0] | c.r[0];
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb2e4: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTlutRegionCallback (0x800bb2f8, 20 bytes)
#[inline(never)]
pub fn f_800bb2f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb2f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb2f8); p = true; }
    }
    f_800bb2f8_b(c, 0x800bb2f8);
    if p { crate::prof::exit(c, 0x800bb2f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb2f8_at(c: &mut Cpu, pc: u32) { f_800bb2f8_b(c, pc) }
#[inline(always)]
fn f_800bb2f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb2f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb2f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb2f8 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                { let ea = c.r[4].wrapping_add(0x414); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.stw(c.r[4], c.r[3]);
                c.r[3] = c.r[0] | c.r[0];
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb2f8: bad pc {:#x}", pc),
        }
    }
}

/// __SetSURegs (0x800bb30c, 184 bytes)
#[inline(never)]
pub fn f_800bb30c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb30c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb30c); p = true; }
    }
    f_800bb30c_b(c, 0x800bb30c);
    if p { crate::prof::exit(c, 0x800bb30c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb30c_at(c: &mut Cpu, pc: u32) { f_800bb30c_b(c, pc) }
#[inline(always)]
fn f_800bb30c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb30c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb30c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb30c => { ins += 46;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[8] = c.r[4].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.r[6] = c.r[5].wrapping_add(c.r[8]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x45c));
                c.r[0] = c.lwz_t(ins, c.r[6].wrapping_add(0xb8));
                c.r[7] = c.r[5].wrapping_add(c.r[8]);
                c.r[9] = c.r[5].wrapping_add(c.r[8]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff0000;
                c.r[0] = (c.r[4].rotate_left(0) & 0x3ff) | (c.r[0] & 0xfffffc00);
                c.stw(c.r[6].wrapping_add(0xb8), c.r[0]);
                c.r[10] = c.r[5].wrapping_add(c.r[8]);
                c.r[0] = c.lwz_t(ins, c.r[7].wrapping_add(0xd8));
                c.r[0] = c.r[0].rotate_left(0) & 0xffff0000;
                c.r[0] = (c.r[4].rotate_left(22) & 0x3ff) | (c.r[0] & 0xfffffc00);
                c.stw(c.r[7].wrapping_add(0xd8), c.r[0]);
                c.r[7] = 0x61;
                c.r[6] = c.lwz_t(ins, c.r[3].wrapping_add(0x47c));
                c.r[3] = c.lwz_t(ins, c.r[9].wrapping_add(0xb8));
                c.r[0] = c.r[6].rotate_left(30) & 0x3;
                c.r[4] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[6].rotate_left(0) & 0x3;
                c.r[0] = c.addc(!c.r[0], 0x1, 1);
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[3].rotate_left(0) & 0xfffeffff;
                c.r[0] = c.r[0].rotate_left(11) & 0xff0000;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[9].wrapping_add(0xb8), c.r[0]);
                c.r[0] = c.r[4].leading_zeros();
                c.r[0] = c.r[0].rotate_left(11) & 0xff0000;
                c.r[3] = c.lwz_t(ins, c.r[10].wrapping_add(0xd8));
                c.r[6] = 0xcc010000;
                c.r[4] = c.r[5].wrapping_add(c.r[8]);
                c.r[3] = c.r[3].rotate_left(0) & 0xfffeffff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[10].wrapping_add(0xd8), c.r[0]);
                c.r[0] = 0x0;
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[7]);
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0xb8));
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[3]);
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[7]);
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0xd8));
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[5].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb30c: bad pc {:#x}", pc),
        }
    }
}

/// __GXSetSUTexRegs (0x800bb3c4, 380 bytes)
#[inline(never)]
pub fn f_800bb3c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb3c4) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb3c4); p = true; }
    }
    f_800bb3c4_b(c, 0x800bb3c4);
    if p { crate::prof::exit(c, 0x800bb3c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb3c4_at(c: &mut Cpu, pc: u32) { f_800bb3c4_b(c, pc) }
#[inline(never)]
fn f_800bb3c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb3c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb3c4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb3c4 => { ins += 8;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4dc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xff);
                if cr_bit(cr, 2) { pc = 0x800bb52c; continue; }
                pc = 0x800bb3e4;
            }
            0x800bb3e4 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x204));
                c.r[31] = 0x0;
                c.r[3] = c.r[0].rotate_left(22) & 0xf;
                c.r[30] = c.r[3].wrapping_add(0x1);
                c.r[27] = c.r[0].rotate_left(16) & 0x7;
                pc = 0x800bb498; continue;
                pc = 0x800bb3fc;
            }
            0x800bb3fc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x2);
                if cr_bit(cr, 2) { pc = 0x800bb44c; continue; }
                pc = 0x800bb404;
            }
            0x800bb404 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bb418; continue; }
                pc = 0x800bb408;
            }
            0x800bb408 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bb424; continue; }
                pc = 0x800bb410;
            }
            0x800bb410 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bb438; continue; }
                pc = 0x800bb414;
            }
            0x800bb414 => { ins += 1;
                pc = 0x800bb470; continue;
                pc = 0x800bb418;
            }
            0x800bb418 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x4);
                if !cr_bit(cr, 0) { pc = 0x800bb470; continue; }
                pc = 0x800bb420;
            }
            0x800bb420 => { ins += 1;
                pc = 0x800bb460; continue;
                pc = 0x800bb424;
            }
            0x800bb424 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x120));
                c.r[29] = c.r[0].rotate_left(0) & 0x7;
                c.r[28] = c.r[0].rotate_left(29) & 0x7;
                pc = 0x800bb470; continue;
                pc = 0x800bb438;
            }
            0x800bb438 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x120));
                c.r[29] = c.r[0].rotate_left(26) & 0x7;
                c.r[28] = c.r[0].rotate_left(23) & 0x7;
                pc = 0x800bb470; continue;
                pc = 0x800bb44c;
            }
            0x800bb44c => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x120));
                c.r[29] = c.r[0].rotate_left(20) & 0x7;
                c.r[28] = c.r[0].rotate_left(17) & 0x7;
                pc = 0x800bb470; continue;
                pc = 0x800bb460;
            }
            0x800bb460 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x120));
                c.r[29] = c.r[0].rotate_left(14) & 0x7;
                c.r[28] = c.r[0].rotate_left(11) & 0x7;
                pc = 0x800bb470;
            }
            0x800bb470 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x1;
                c.r[0] = slw(c.r[0], c.r[28]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4dc));
                c.r[0] = c.r[3] & c.r[0];
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800bb494; continue; }
                pc = 0x800bb488;
            }
            0x800bb488 => { ins += 3;
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = c.r[28].wrapping_add(0x0);
                c.lr = 0x800bb494; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb30c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bb494;
            }
            0x800bb494 => { ins += 1;
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x800bb498;
            }
            0x800bb498 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], c.r[27]);
                if cr_bit(cr, 0) { pc = 0x800bb3fc; continue; }
                pc = 0x800bb4a0;
            }
            0x800bb4a0 => { ins += 3;
                c.r[31] = 0x0;
                c.r[27] = c.r[31].wrapping_add(0x0);
                pc = 0x800bb524; continue;
                pc = 0x800bb4ac;
            }
            0x800bb4ac => { ins += 9;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[3] = c.r[27].wrapping_add(0x49c);
                c.r[4] = c.r[31].rotate_left(1) & 0xfffffffc;
                c.r[3] = c.lwz_t(ins, c.r[5].wrapping_add(c.r[3]));
                c.r[0] = c.r[31].rotate_left(0) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x100);
                c.r[4] = c.r[5].wrapping_add(c.r[4]);
                c.r[29] = c.r[3].rotate_left(0) & 0xfffffeff;
                if cr_bit(cr, 2) { pc = 0x800bb4dc; continue; }
                pc = 0x800bb4d0;
            }
            0x800bb4d0 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.r[28] = c.r[0].rotate_left(17) & 0x7;
                pc = 0x800bb4e4; continue;
                pc = 0x800bb4dc;
            }
            0x800bb4dc => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[4]);
                c.r[28] = c.r[0].rotate_left(29) & 0x7;
                pc = 0x800bb4e4;
            }
            0x800bb4e4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0xff);
                if cr_bit(cr, 2) { pc = 0x800bb51c; continue; }
                pc = 0x800bb4ec;
            }
            0x800bb4ec => { ins += 5;
                c.r[4] = 0x1;
                c.r[3] = c.lwz_t(ins, c.r[5].wrapping_add(0x4dc));
                c.r[0] = slw(c.r[4], c.r[28]);
                c.r[0] = c.r[3] & c.r[0];
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800bb51c; continue; }
                pc = 0x800bb500;
            }
            0x800bb500 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[5].wrapping_add(0x4e0));
                c.r[0] = slw(c.r[4], c.r[31]);
                c.r[0] = c.r[3] & c.r[0];
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800bb51c; continue; }
                pc = 0x800bb510;
            }
            0x800bb510 => { ins += 3;
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.r[4] = c.r[28].wrapping_add(0x0);
                c.lr = 0x800bb51c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb30c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bb51c;
            }
            0x800bb51c => { ins += 2;
                c.r[27] = c.r[27].wrapping_add(0x4);
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x800bb524;
            }
            0x800bb524 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], c.r[30]);
                if cr_bit(cr, 0) { pc = 0x800bb4ac; continue; }
                pc = 0x800bb52c;
            }
            0x800bb52c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb3c4: bad pc {:#x}", pc),
        }
    }
}

/// __GXSetTmemConfig (0x800bb540, 576 bytes)
#[inline(never)]
pub fn f_800bb540(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb540) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb540); p = true; }
    }
    f_800bb540_b(c, 0x800bb540);
    if p { crate::prof::exit(c, 0x800bb540); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb540_at(c: &mut Cpu, pc: u32) { f_800bb540_b(c, pc) }
#[inline(always)]
fn f_800bb540_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb540;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb540, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb540 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                if cr_bit(cr, 2) { pc = 0x800bb55c; continue; }
                pc = 0x800bb554;
            }
            0x800bb554 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bb668; continue; }
                pc = 0x800bb558;
            }
            0x800bb558 => { ins += 1;
                pc = 0x800bb668; continue;
                pc = 0x800bb55c;
            }
            0x800bb55c => { ins += 67;
                c.r[0] = 0x61;
                c.r[3] = 0xcc010000;
                c.r[4] = 0x8c0e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0xffff8000);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.r[4] = 0x900e0000;
                c.r[7] = 0x8d0e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0xffffc000);
                c.r[6] = 0x910e0000;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.r[5] = 0x8e0e0000;
                c.r[4] = 0x920e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0xffff8800);
                c.r[30] = 0x8f0e0000;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[7]);
                c.r[12] = 0x930e0000;
                c.r[11] = 0xac0e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[6] = c.r[6].wrapping_add(0xffffc800);
                c.r[10] = 0xb00e0000;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[6]);
                c.r[9] = 0xad0e0000;
                c.r[8] = 0xb10e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0xffff9000);
                c.r[7] = 0xae0e0000;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                c.r[6] = 0xb20e0000;
                c.r[5] = 0xaf0e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[31] = c.r[4].wrapping_add(0xffffd000);
                c.r[4] = 0xb30e0000;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[31]);
                c.r[30] = c.r[30].wrapping_add(0xffff9800);
                c.r[12] = c.r[12].wrapping_add(0xffffd800);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[11] = c.r[11].wrapping_add(0xffffa000);
                c.r[10] = c.r[10].wrapping_add(0xffffe000);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[30]);
                c.r[9] = c.r[9].wrapping_add(0xffffa800);
                c.r[8] = c.r[8].wrapping_add(0xffffe800);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0xffffb000);
                c.r[6] = c.r[6].wrapping_add(0xfffff000);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[12]);
                c.r[5] = c.r[5].wrapping_add(0xffffb800);
                c.r[4] = c.r[4].wrapping_add(0xfffff800);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[11]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[10]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[9]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[8]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[7]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[6]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                pc = 0x800bb770; continue;
                pc = 0x800bb668;
            }
            0x800bb668 => { ins += 66;
                c.r[0] = 0x61;
                c.r[3] = 0xcc010000;
                c.r[4] = 0x8c0e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0xffff8000);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.r[4] = 0x900e0000;
                c.r[7] = 0x8d0e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0xffffc000);
                c.r[6] = 0x910e0000;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.r[5] = 0x8e0e0000;
                c.r[4] = 0x920e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0xffff8400);
                c.r[31] = 0x8f0e0000;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[7]);
                c.r[12] = 0x930e0000;
                c.r[11] = 0xac0e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[6] = c.r[6].wrapping_add(0xffffc400);
                c.r[10] = 0xb00e0000;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[6]);
                c.r[9] = 0xad0e0000;
                c.r[8] = 0xb10e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0xffff8800);
                c.r[7] = 0xae0e0000;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                c.r[6] = 0xb20e0000;
                c.r[5] = 0xaf0e0000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[30] = c.r[4].wrapping_add(0xffffc800);
                c.r[4] = 0xb30e0000;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[30]);
                c.r[31] = c.r[31].wrapping_add(0xffff8c00);
                c.r[12] = c.r[12].wrapping_add(0xffffcc00);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[11] = c.r[11].wrapping_add(0xffff9000);
                c.r[10] = c.r[10].wrapping_add(0xffffd000);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[31]);
                c.r[9] = c.r[9].wrapping_add(0xffff9400);
                c.r[8] = c.r[8].wrapping_add(0xffffd400);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0xffff9800);
                c.r[6] = c.r[6].wrapping_add(0xffffd800);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[12]);
                c.r[5] = c.r[5].wrapping_add(0xffff9c00);
                c.r[4] = c.r[4].wrapping_add(0xffffdc00);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[11]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[10]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[9]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[8]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[7]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[6]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                pc = 0x800bb770;
            }
            0x800bb770 => { ins += 4;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb540: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevIndirect (0x800bb780, 156 bytes)
#[inline(never)]
pub fn f_800bb780(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb780) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb780); p = true; }
    }
    f_800bb780_b(c, 0x800bb780);
    if p { crate::prof::exit(c, 0x800bb780); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb780_at(c: &mut Cpu, pc: u32) { f_800bb780_b(c, pc) }
#[inline(always)]
fn f_800bb780_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb780;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb780, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb780 => { ins += 39;
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff3;
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4] | c.r[0];
                c.r[11] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.r[5] = c.r[0].rotate_left(0) & 0xffffff8f;
                c.r[12] = c.lbz_t(ins, c.r[1].wrapping_add(0x33));
                c.r[0] = c.r[6].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[5] | c.r[0];
                c.r[5] = c.r[0].rotate_left(0) & 0xfffffe7f;
                c.r[0] = c.r[11].rotate_left(7) & 0xffffff80;
                c.r[0] = c.r[5] | c.r[0];
                c.r[5] = c.r[0].rotate_left(0) & 0xffffe1ff;
                c.r[0] = c.r[7].rotate_left(9) & 0xfffffe00;
                c.r[0] = c.r[5] | c.r[0];
                c.r[5] = c.r[0].rotate_left(0) & 0xffff1fff;
                c.r[0] = c.r[8].rotate_left(13) & 0xffffe000;
                c.r[0] = c.r[5] | c.r[0];
                c.r[5] = c.r[3].wrapping_add(0x10);
                c.r[3] = c.r[0].rotate_left(0) & 0xfff8ffff;
                c.r[0] = c.r[9].rotate_left(16) & 0xffff0000;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xfff7ffff;
                c.r[0] = c.r[12].rotate_left(19) & 0xfff80000;
                c.r[0] = c.r[3] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffefffff;
                c.r[0] = c.r[10].rotate_left(20) & 0xff00000;
                c.r[6] = c.r[3] | c.r[0];
                c.r[0] = 0x61;
                c.r[3] = 0xcc010000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.r[5].rotate_left(24) & 0xff000000;
                c.r[0] = (c.r[6].rotate_left(0) & 0xffffff) | (c.r[0] & 0xff000000);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x28);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb780: bad pc {:#x}", pc),
        }
    }
}

/// GXSetIndTexMtx (0x800bb81c, 352 bytes)
#[inline(never)]
pub fn f_800bb81c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb81c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb81c); p = true; }
    }
    f_800bb81c_b(c, 0x800bb81c);
    if p { crate::prof::exit(c, 0x800bb81c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb81c_at(c: &mut Cpu, pc: u32) { f_800bb81c_b(c, pc) }
#[inline(always)]
fn f_800bb81c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb81c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb81c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb81c => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x8);
                { let ea = c.r[1].wrapping_add(0xffffff88); c.stw(ea, c.r[1]); c.r[1] = ea; }
                if cr_bit(cr, 2) { pc = 0x800bb868; continue; }
                pc = 0x800bb828;
            }
            0x800bb828 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bb844; continue; }
                pc = 0x800bb82c;
            }
            0x800bb82c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x4);
                if cr_bit(cr, 2) { pc = 0x800bb868; continue; }
                pc = 0x800bb834;
            }
            0x800bb834 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bb858; continue; }
                pc = 0x800bb838;
            }
            0x800bb838 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                if !cr_bit(cr, 0) { pc = 0x800bb850; continue; }
                pc = 0x800bb840;
            }
            0x800bb840 => { ins += 1;
                pc = 0x800bb868; continue;
                pc = 0x800bb844;
            }
            0x800bb844 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xc);
                if !cr_bit(cr, 0) { pc = 0x800bb868; continue; }
                pc = 0x800bb84c;
            }
            0x800bb84c => { ins += 1;
                pc = 0x800bb860; continue;
                pc = 0x800bb850;
            }
            0x800bb850 => { ins += 2;
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x800bb86c; continue;
                pc = 0x800bb858;
            }
            0x800bb858 => { ins += 2;
                c.r[0] = c.r[3].wrapping_add(0xfffffffb);
                pc = 0x800bb86c; continue;
                pc = 0x800bb860;
            }
            0x800bb860 => { ins += 2;
                c.r[0] = c.r[3].wrapping_add(0xfffffff7);
                pc = 0x800bb86c; continue;
                pc = 0x800bb868;
            }
            0x800bb868 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800bb86c;
            }
            0x800bb86c => { ins += 68;
                { let ea = c.r[2].wrapping_add(0xffff8cd0); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[3] = c.r[0].wrapping_mul(0x3);
                { let ea = c.r[4]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.r[11] = c.r[5].wrapping_add(0x11);
                { let v = s(c.f[2] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.r[0] = c.r[3].wrapping_add(0x6);
                c.r[11] = c.r[11] as u8 as i8 as i32 as u32;
                c.f[1] = fctiwz(c.f[1]);
                c.r[10] = 0x61;
                c.f[0] = fctiwz(c.f[0]);
                c.r[9] = 0xcc010000;
                c.stb(c.r[9].wrapping_add(0xffff8000), c.r[10]);
                { let ea = c.r[1].wrapping_add(0x70); let v = c.f[1]; c.stfd(ea, v); }
                c.r[0] = c.r[0].rotate_left(24) & 0xff000000;
                c.r[6] = c.r[3].wrapping_add(0x7);
                { let ea = c.r[1].wrapping_add(0x68); let v = c.f[0]; c.stfd(ea, v); }
                c.r[5] = c.r[3].wrapping_add(0x8);
                c.r[8] = c.lwz_t(ins, c.r[1].wrapping_add(0x74));
                c.r[7] = c.lwz_t(ins, c.r[1].wrapping_add(0x6c));
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[7] = c.r[7].rotate_left(11) & 0x3ff800;
                c.r[7] = (c.r[8].rotate_left(0) & 0x7ff) | (c.r[7] & 0xfffff800);
                c.r[7] = c.r[7].rotate_left(0) & 0xff3fffff;
                c.r[7] = (c.r[11].rotate_left(22) & 0xc00000) | (c.r[7] & 0xff3fffff);
                c.r[0] = (c.r[7].rotate_left(0) & 0xffffff) | (c.r[0] & 0xff000000);
                c.stw(c.r[9].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                { let ea = c.r[4].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[2] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.stb(c.r[9].wrapping_add(0xffff8000), c.r[10]);
                c.f[1] = fctiwz(c.f[1]);
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[1]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let v = c.f[0]; c.stfd(ea, v); }
                c.r[8] = c.lwz_t(ins, c.r[1].wrapping_add(0x64));
                c.r[7] = c.lwz_t(ins, c.r[1].wrapping_add(0x5c));
                c.r[7] = c.r[7].rotate_left(11) & 0x3ff800;
                c.r[7] = (c.r[8].rotate_left(0) & 0x7ff) | (c.r[7] & 0xfffff800);
                c.r[7] = c.r[7].rotate_left(0) & 0xff3fffff;
                c.r[7] = (c.r[11].rotate_left(20) & 0xc00000) | (c.r[7] & 0xff3fffff);
                c.r[7] = c.r[7].rotate_left(0) & 0xffffff;
                c.r[7] = (c.r[6].rotate_left(24) & 0xff000000) | (c.r[7] & 0xffffff);
                c.stw(c.r[9].wrapping_add(0xffff8000), c.r[7]);
                { let ea = c.r[4].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[2] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.stb(c.r[9].wrapping_add(0xffff8000), c.r[10]);
                c.f[1] = fctiwz(c.f[1]);
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[1]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let v = c.f[0]; c.stfd(ea, v); }
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x4c));
                c.r[4] = c.r[4].rotate_left(11) & 0x3ff800;
                c.r[4] = (c.r[6].rotate_left(0) & 0x7ff) | (c.r[4] & 0xfffff800);
                c.r[4] = c.r[4].rotate_left(0) & 0xff3fffff;
                c.r[4] = (c.r[11].rotate_left(18) & 0xc00000) | (c.r[4] & 0xff3fffff);
                c.r[4] = c.r[4].rotate_left(0) & 0xffffff;
                c.r[4] = (c.r[5].rotate_left(24) & 0xff000000) | (c.r[4] & 0xffffff);
                c.stw(c.r[9].wrapping_add(0xffff8000), c.r[4]);
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x78);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb81c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetIndTexCoordScale (0x800bb97c, 380 bytes)
#[inline(never)]
pub fn f_800bb97c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bb97c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bb97c); p = true; }
    }
    f_800bb97c_b(c, 0x800bb97c);
    if p { crate::prof::exit(c, 0x800bb97c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bb97c_at(c: &mut Cpu, pc: u32) { f_800bb97c_b(c, pc) }
#[inline(always)]
fn f_800bb97c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bb97c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bb97c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bb97c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x2);
                if cr_bit(cr, 2) { pc = 0x800bba48; continue; }
                pc = 0x800bb984;
            }
            0x800bb984 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bb998; continue; }
                pc = 0x800bb988;
            }
            0x800bb988 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bb9a4; continue; }
                pc = 0x800bb990;
            }
            0x800bb990 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bb9f4; continue; }
                pc = 0x800bb994;
            }
            0x800bb994 => { ins += 1;
                pc = 0x800bbae8; continue;
                pc = 0x800bb998;
            }
            0x800bb998 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x4);
                if !cr_bit(cr, 0) { pc = 0x800bbae8; continue; }
                pc = 0x800bb9a0;
            }
            0x800bb9a0 => { ins += 1;
                pc = 0x800bba98; continue;
                pc = 0x800bb9a4;
            }
            0x800bb9a4 => { ins += 20;
                c.r[7] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[0] = 0x61;
                c.r[6] = c.lwz_t(ins, c.r[7].wrapping_add(0x128));
                c.r[3] = 0xcc010000;
                c.r[6] = c.r[6].rotate_left(0) & 0xfffffff0;
                c.r[4] = c.r[6] | c.r[4];
                c.stw(c.r[7].wrapping_add(0x128), c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[7].wrapping_add(0x128));
                c.r[4] = c.r[4].rotate_left(0) & 0xffffff0f;
                c.r[4] = c.r[4] | c.r[5];
                c.stw(c.r[7].wrapping_add(0x128), c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[7].wrapping_add(0x128));
                c.r[4] = c.r[4].rotate_left(0) & 0xffffff;
                c.r[4] = c.r[4] | 0x25000000;
                c.stw(c.r[7].wrapping_add(0x128), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[7].wrapping_add(0x128));
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bbae8; continue;
                pc = 0x800bb9f4;
            }
            0x800bb9f4 => { ins += 21;
                c.r[7] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[6] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[5].rotate_left(12) & 0xfffff000;
                c.r[5] = c.lwz_t(ins, c.r[7].wrapping_add(0x128));
                c.r[0] = 0x61;
                c.r[3] = 0xcc010000;
                c.r[5] = c.r[5].rotate_left(0) & 0xfffff0ff;
                c.r[5] = c.r[5] | c.r[6];
                c.stw(c.r[7].wrapping_add(0x128), c.r[5]);
                c.r[5] = c.lwz_t(ins, c.r[7].wrapping_add(0x128));
                c.r[5] = c.r[5].rotate_left(0) & 0xffff0fff;
                c.r[4] = c.r[5] | c.r[4];
                c.stw(c.r[7].wrapping_add(0x128), c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[7].wrapping_add(0x128));
                c.r[4] = c.r[4].rotate_left(0) & 0xffffff;
                c.r[4] = c.r[4] | 0x25000000;
                c.stw(c.r[7].wrapping_add(0x128), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[7].wrapping_add(0x128));
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bbae8; continue;
                pc = 0x800bba48;
            }
            0x800bba48 => { ins += 20;
                c.r[7] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[0] = 0x61;
                c.r[6] = c.lwz_t(ins, c.r[7].wrapping_add(0x12c));
                c.r[3] = 0xcc010000;
                c.r[6] = c.r[6].rotate_left(0) & 0xfffffff0;
                c.r[4] = c.r[6] | c.r[4];
                c.stw(c.r[7].wrapping_add(0x12c), c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[7].wrapping_add(0x12c));
                c.r[4] = c.r[4].rotate_left(0) & 0xffffff0f;
                c.r[4] = c.r[4] | c.r[5];
                c.stw(c.r[7].wrapping_add(0x12c), c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[7].wrapping_add(0x12c));
                c.r[4] = c.r[4].rotate_left(0) & 0xffffff;
                c.r[4] = c.r[4] | 0x26000000;
                c.stw(c.r[7].wrapping_add(0x12c), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[7].wrapping_add(0x12c));
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bbae8; continue;
                pc = 0x800bba98;
            }
            0x800bba98 => { ins += 20;
                c.r[7] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[6] = c.r[4].rotate_left(8) & 0xffffff00;
                c.r[4] = c.r[5].rotate_left(12) & 0xfffff000;
                c.r[5] = c.lwz_t(ins, c.r[7].wrapping_add(0x12c));
                c.r[0] = 0x61;
                c.r[3] = 0xcc010000;
                c.r[5] = c.r[5].rotate_left(0) & 0xfffff0ff;
                c.r[5] = c.r[5] | c.r[6];
                c.stw(c.r[7].wrapping_add(0x12c), c.r[5]);
                c.r[5] = c.lwz_t(ins, c.r[7].wrapping_add(0x12c));
                c.r[5] = c.r[5].rotate_left(0) & 0xffff0fff;
                c.r[4] = c.r[5] | c.r[4];
                c.stw(c.r[7].wrapping_add(0x12c), c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[7].wrapping_add(0x12c));
                c.r[4] = c.r[4].rotate_left(0) & 0xffffff;
                c.r[4] = c.r[4] | 0x26000000;
                c.stw(c.r[7].wrapping_add(0x12c), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[7].wrapping_add(0x12c));
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bbae8;
            }
            0x800bbae8 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x0;
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bb97c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetIndTexOrder (0x800bbaf8, 276 bytes)
#[inline(never)]
pub fn f_800bbaf8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bbaf8) { return; }
        if c.prof { crate::prof::enter(c, 0x800bbaf8); p = true; }
    }
    f_800bbaf8_b(c, 0x800bbaf8);
    if p { crate::prof::exit(c, 0x800bbaf8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bbaf8_at(c: &mut Cpu, pc: u32) { f_800bbaf8_b(c, pc) }
#[inline(always)]
fn f_800bbaf8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bbaf8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bbaf8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bbaf8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x2);
                if cr_bit(cr, 2) { pc = 0x800bbb80; continue; }
                pc = 0x800bbb00;
            }
            0x800bbb00 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bbb14; continue; }
                pc = 0x800bbb04;
            }
            0x800bbb04 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bbb20; continue; }
                pc = 0x800bbb0c;
            }
            0x800bbb0c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bbb50; continue; }
                pc = 0x800bbb10;
            }
            0x800bbb10 => { ins += 1;
                pc = 0x800bbbdc; continue;
                pc = 0x800bbb14;
            }
            0x800bbb14 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x4);
                if !cr_bit(cr, 0) { pc = 0x800bbbdc; continue; }
                pc = 0x800bbb1c;
            }
            0x800bbb1c => { ins += 1;
                pc = 0x800bbbb0; continue;
                pc = 0x800bbb20;
            }
            0x800bbb20 => { ins += 12;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[4].rotate_left(3) & 0xfffffff8;
                c.r[4] = c.r[3].wrapping_add(0x120);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x120));
                c.r[3] = c.r[3].rotate_left(0) & 0xfffffff8;
                c.r[3] = c.r[3] | c.r[5];
                c.stw(c.r[4], c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[4]);
                c.r[3] = c.r[3].rotate_left(0) & 0xffffffc7;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[4], c.r[0]);
                pc = 0x800bbbdc; continue;
                pc = 0x800bbb50;
            }
            0x800bbb50 => { ins += 12;
                c.r[6] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[4].rotate_left(9) & 0xfffffe00;
                c.r[3] = c.r[5].rotate_left(6) & 0xffffffc0;
                c.r[4] = c.lwz_t(ins, c.r[6].wrapping_add(0x120));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffe3f;
                c.r[3] = c.r[4] | c.r[3];
                c.stw(c.r[6].wrapping_add(0x120), c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[6].wrapping_add(0x120));
                c.r[3] = c.r[3].rotate_left(0) & 0xfffff1ff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[6].wrapping_add(0x120), c.r[0]);
                pc = 0x800bbbdc; continue;
                pc = 0x800bbb80;
            }
            0x800bbb80 => { ins += 12;
                c.r[6] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[4].rotate_left(15) & 0xffff8000;
                c.r[3] = c.r[5].rotate_left(12) & 0xfffff000;
                c.r[4] = c.lwz_t(ins, c.r[6].wrapping_add(0x120));
                c.r[4] = c.r[4].rotate_left(0) & 0xffff8fff;
                c.r[3] = c.r[4] | c.r[3];
                c.stw(c.r[6].wrapping_add(0x120), c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[6].wrapping_add(0x120));
                c.r[3] = c.r[3].rotate_left(0) & 0xfffc7fff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[6].wrapping_add(0x120), c.r[0]);
                pc = 0x800bbbdc; continue;
                pc = 0x800bbbb0;
            }
            0x800bbbb0 => { ins += 11;
                c.r[6] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[4].rotate_left(21) & 0xffe00000;
                c.r[3] = c.r[5].rotate_left(18) & 0xfffc0000;
                c.r[4] = c.lwz_t(ins, c.r[6].wrapping_add(0x120));
                c.r[4] = c.r[4].rotate_left(0) & 0xffe3ffff;
                c.r[3] = c.r[4] | c.r[3];
                c.stw(c.r[6].wrapping_add(0x120), c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[6].wrapping_add(0x120));
                c.r[3] = c.r[3].rotate_left(0) & 0xff1fffff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[6].wrapping_add(0x120), c.r[0]);
                pc = 0x800bbbdc;
            }
            0x800bbbdc => { ins += 12;
                c.r[0] = 0x61;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x120));
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x4f4));
                c.r[3] = c.r[3] | 0x3;
                c.stw(c.r[4].wrapping_add(0x4f4), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bbaf8: bad pc {:#x}", pc),
        }
    }
}

/// GXSetNumIndStages (0x800bbc0c, 40 bytes)
#[inline(never)]
pub fn f_800bbc0c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bbc0c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bbc0c); p = true; }
    }
    f_800bbc0c_b(c, 0x800bbc0c);
    if p { crate::prof::exit(c, 0x800bbc0c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bbc0c_at(c: &mut Cpu, pc: u32) { f_800bbc0c_b(c, pc) }
#[inline(always)]
fn f_800bbc0c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bbc0c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bbc0c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bbc0c => { ins += 10;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[3].rotate_left(16) & 0xff0000;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x204));
                c.r[3] = c.r[3].rotate_left(0) & 0xfff8ffff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[4].wrapping_add(0x204), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x4f4));
                c.r[0] = c.r[0] | 0x6;
                c.stw(c.r[4].wrapping_add(0x4f4), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bbc0c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevDirect (0x800bbc34, 72 bytes)
#[inline(never)]
pub fn f_800bbc34(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bbc34) { return; }
        if c.prof { crate::prof::enter(c, 0x800bbc34); p = true; }
    }
    f_800bbc34_b(c, 0x800bbc34);
    if p { crate::prof::exit(c, 0x800bbc34); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bbc34_at(c: &mut Cpu, pc: u32) { f_800bbc34_b(c, pc) }
#[inline(always)]
fn f_800bbc34_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bbc34;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bbc34, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bbc34 => { ins += 14;
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                c.r[5] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[8] = 0x0;
                c.r[9] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.r[10] = 0x0;
                c.lr = 0x800bbc6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bbc6c;
            }
            0x800bbc6c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bbc34: bad pc {:#x}", pc),
        }
    }
}

/// fn_800BBC7C (0x800bbc7c, 100 bytes)
#[inline(never)]
pub fn f_800bbc7c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bbc7c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bbc7c); p = true; }
    }
    f_800bbc7c_b(c, 0x800bbc7c);
    if p { crate::prof::exit(c, 0x800bbc7c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bbc7c_at(c: &mut Cpu, pc: u32) { f_800bbc7c_b(c, pc) }
#[inline(always)]
fn f_800bbc7c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bbc7c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bbc7c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bbc7c => { ins += 5;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[6].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                if cr_bit(cr, 2) { pc = 0x800bbc98; continue; }
                pc = 0x800bbc90;
            }
            0x800bbc90 => { ins += 2;
                c.r[6] = 0x6;
                pc = 0x800bbc9c; continue;
                pc = 0x800bbc98;
            }
            0x800bbc98 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x800bbc9c;
            }
            0x800bbc9c => { ins += 3;
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[8] = c.r[6].wrapping_add(0x0);
                if cr_bit(cr, 2) { pc = 0x800bbcb0; continue; }
                pc = 0x800bbca8;
            }
            0x800bbca8 => { ins += 2;
                c.r[6] = 0x7;
                pc = 0x800bbcb4; continue;
                pc = 0x800bbcb0;
            }
            0x800bbcb0 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x800bbcb4;
            }
            0x800bbcb4 => { ins += 7;
                c.r[0] = 0x0;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[9] = c.r[8].wrapping_add(0x0);
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.r[10] = 0x0;
                c.lr = 0x800bbcd0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bbcd0;
            }
            0x800bbcd0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bbc7c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800BBCE0 (0x800bbce0, 428 bytes)
#[inline(never)]
pub fn f_800bbce0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bbce0) { return; }
        if c.prof { crate::prof::enter(c, 0x800bbce0); p = true; }
    }
    f_800bbce0_b(c, 0x800bbce0);
    if p { crate::prof::exit(c, 0x800bbce0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bbce0_at(c: &mut Cpu, pc: u32) { f_800bbce0_b(c, pc) }
#[inline(never)]
fn f_800bbce0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bbce0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bbce0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bbce0 => { ins += 13;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x40);
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x50); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[24] = c.r[3].wrapping_add(0x0);
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x78));
                c.r[25] = c.r[4].wrapping_add(0x0);
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x7c));
                c.r[26] = c.r[9].wrapping_add(0x0);
                c.r[27] = c.r[10].wrapping_add(0x0);
                if cr_bit(cr, 2) { pc = 0x800bbd58; continue; }
                pc = 0x800bbd14;
            }
            0x800bbd14 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bbd30; continue; }
                pc = 0x800bbd18;
            }
            0x800bbd18 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x20);
                if cr_bit(cr, 2) { pc = 0x800bbd60; continue; }
                pc = 0x800bbd20;
            }
            0x800bbd20 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bbd70; continue; }
                pc = 0x800bbd24;
            }
            0x800bbd24 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x10);
                if cr_bit(cr, 2) { pc = 0x800bbd68; continue; }
                pc = 0x800bbd2c;
            }
            0x800bbd2c => { ins += 1;
                pc = 0x800bbd70; continue;
                pc = 0x800bbd30;
            }
            0x800bbd30 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x100);
                if cr_bit(cr, 2) { pc = 0x800bbd48; continue; }
                pc = 0x800bbd38;
            }
            0x800bbd38 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bbd70; continue; }
                pc = 0x800bbd3c;
            }
            0x800bbd3c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x80);
                if cr_bit(cr, 2) { pc = 0x800bbd50; continue; }
                pc = 0x800bbd44;
            }
            0x800bbd44 => { ins += 1;
                pc = 0x800bbd70; continue;
                pc = 0x800bbd48;
            }
            0x800bbd48 => { ins += 2;
                c.r[31] = 0x1;
                pc = 0x800bbd74; continue;
                pc = 0x800bbd50;
            }
            0x800bbd50 => { ins += 2;
                c.r[31] = 0x2;
                pc = 0x800bbd74; continue;
                pc = 0x800bbd58;
            }
            0x800bbd58 => { ins += 2;
                c.r[31] = 0x3;
                pc = 0x800bbd74; continue;
                pc = 0x800bbd60;
            }
            0x800bbd60 => { ins += 2;
                c.r[31] = 0x4;
                pc = 0x800bbd74; continue;
                pc = 0x800bbd68;
            }
            0x800bbd68 => { ins += 2;
                c.r[31] = 0x5;
                pc = 0x800bbd74; continue;
                pc = 0x800bbd70;
            }
            0x800bbd70 => { ins += 1;
                c.r[31] = 0x0;
                pc = 0x800bbd74;
            }
            0x800bbd74 => { ins += 3;
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x40);
                if cr_bit(cr, 2) { pc = 0x800bbdc4; continue; }
                pc = 0x800bbd80;
            }
            0x800bbd80 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bbd9c; continue; }
                pc = 0x800bbd84;
            }
            0x800bbd84 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x20);
                if cr_bit(cr, 2) { pc = 0x800bbdcc; continue; }
                pc = 0x800bbd8c;
            }
            0x800bbd8c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bbddc; continue; }
                pc = 0x800bbd90;
            }
            0x800bbd90 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x10);
                if cr_bit(cr, 2) { pc = 0x800bbdd4; continue; }
                pc = 0x800bbd98;
            }
            0x800bbd98 => { ins += 1;
                pc = 0x800bbddc; continue;
                pc = 0x800bbd9c;
            }
            0x800bbd9c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x100);
                if cr_bit(cr, 2) { pc = 0x800bbdb4; continue; }
                pc = 0x800bbda4;
            }
            0x800bbda4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bbddc; continue; }
                pc = 0x800bbda8;
            }
            0x800bbda8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x80);
                if cr_bit(cr, 2) { pc = 0x800bbdbc; continue; }
                pc = 0x800bbdb0;
            }
            0x800bbdb0 => { ins += 1;
                pc = 0x800bbddc; continue;
                pc = 0x800bbdb4;
            }
            0x800bbdb4 => { ins += 2;
                c.r[30] = 0x1;
                pc = 0x800bbde0; continue;
                pc = 0x800bbdbc;
            }
            0x800bbdbc => { ins += 2;
                c.r[30] = 0x2;
                pc = 0x800bbde0; continue;
                pc = 0x800bbdc4;
            }
            0x800bbdc4 => { ins += 2;
                c.r[30] = 0x3;
                pc = 0x800bbde0; continue;
                pc = 0x800bbdcc;
            }
            0x800bbdcc => { ins += 2;
                c.r[30] = 0x4;
                pc = 0x800bbde0; continue;
                pc = 0x800bbdd4;
            }
            0x800bbdd4 => { ins += 2;
                c.r[30] = 0x5;
                pc = 0x800bbde0; continue;
                pc = 0x800bbddc;
            }
            0x800bbddc => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x800bbde0;
            }
            0x800bbde0 => { ins += 26;
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                { let ea = c.r[2].wrapping_add(0xffff8ce0); c.f[4] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x4c), c.r[0]);
                c.r[6] = 0x43300000;
                c.r[0] = c.r[8].rotate_left(0) & 0xffff;
                { let ea = c.r[2].wrapping_add(0xffff8cd4); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.stw(c.r[1].wrapping_add(0x48), c.r[6]);
                { let ea = c.r[2].wrapping_add(0xffff8cd8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[27] | c.r[27];
                { let ea = c.r[1].wrapping_add(0x48); c.f[0] = c.lfd(ea); }
                c.r[4] = c.r[1].wrapping_add(0x28);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let v = s(c.f[0] - c.f[4]); c.f[0] = v; c.ps1[0] = v; }
                c.r[5] = 0xa;
                c.stw(c.r[1].wrapping_add(0x40), c.r[6]);
                { let v = s(c.f[0] * c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[1].wrapping_add(0x40); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[4]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[1]; c.stfs(ea, v); }
                { let v = s(c.f[0] * c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x2c); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x3c); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x34); let v = c.f[1]; c.stfs(ea, v); }
                c.lr = 0x800bbe48; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb81c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bbe48;
            }
            0x800bbe48 => { ins += 12;
                c.r[0] = 0x1;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.r[24].wrapping_add(0x0);
                c.r[4] = c.r[25].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0xc), c.r[29]);
                c.r[5] = c.r[26].wrapping_add(0x0);
                c.r[6] = c.r[28].wrapping_add(0x0);
                c.r[7] = c.r[27].wrapping_add(0x0);
                c.r[8] = c.r[31].wrapping_add(0x0);
                c.r[9] = c.r[30].wrapping_add(0x0);
                c.r[10] = 0x0;
                c.lr = 0x800bbe78; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bbe78;
            }
            0x800bbe78 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x50); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x74));
                c.r[1] = c.r[1].wrapping_add(0x70);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bbce0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800BBE8C (0x800bbe8c, 268 bytes)
#[inline(never)]
pub fn f_800bbe8c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bbe8c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bbe8c); p = true; }
    }
    f_800bbe8c_b(c, 0x800bbe8c);
    if p { crate::prof::exit(c, 0x800bbe8c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bbe8c_at(c: &mut Cpu, pc: u32) { f_800bbe8c_b(c, pc) }
#[inline(always)]
fn f_800bbe8c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bbe8c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bbe8c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bbe8c => { ins += 11;
                c.r[0] = c.mfspr(8);
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x2);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[29] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[28]);
                c.r[28] = c.r[3].wrapping_add(0x0);
                if cr_bit(cr, 2) { pc = 0x800bbee0; continue; }
                pc = 0x800bbeb8;
            }
            0x800bbeb8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bbec8; continue; }
                pc = 0x800bbebc;
            }
            0x800bbebc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x1);
                if !cr_bit(cr, 0) { pc = 0x800bbed4; continue; }
                pc = 0x800bbec4;
            }
            0x800bbec4 => { ins += 1;
                pc = 0x800bbef4; continue;
                pc = 0x800bbec8;
            }
            0x800bbec8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x4);
                if !cr_bit(cr, 0) { pc = 0x800bbef4; continue; }
                pc = 0x800bbed0;
            }
            0x800bbed0 => { ins += 1;
                pc = 0x800bbeec; continue;
                pc = 0x800bbed4;
            }
            0x800bbed4 => { ins += 3;
                c.r[7] = 0x5;
                c.r[30] = 0x9;
                pc = 0x800bbef4; continue;
                pc = 0x800bbee0;
            }
            0x800bbee0 => { ins += 3;
                c.r[7] = 0x6;
                c.r[30] = 0xa;
                pc = 0x800bbef4; continue;
                pc = 0x800bbeec;
            }
            0x800bbeec => { ins += 2;
                c.r[7] = 0x7;
                c.r[30] = 0xb;
                pc = 0x800bbef4;
            }
            0x800bbef4 => { ins += 11;
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x8), c.r[31]);
                c.r[3] = c.r[28].wrapping_add(0x0);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[5] = 0x0;
                c.r[6] = 0x3;
                c.r[8] = 0x6;
                c.r[9] = 0x6;
                c.r[10] = 0x0;
                c.lr = 0x800bbf20; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bbf20;
            }
            0x800bbf20 => { ins += 11;
                c.stw(c.r[1].wrapping_add(0x8), c.r[31]);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.r[7] = c.r[30].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[3] = c.r[28].wrapping_add(0x1);
                c.r[5] = 0x0;
                c.r[6] = 0x3;
                c.r[8] = 0x6;
                c.r[9] = 0x6;
                c.r[10] = 0x1;
                c.lr = 0x800bbf4c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bbf4c;
            }
            0x800bbf4c => { ins += 11;
                c.stw(c.r[1].wrapping_add(0x8), c.r[31]);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.r[3] = c.r[28].wrapping_add(0x2);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.r[9] = 0x0;
                c.r[10] = 0x1;
                c.lr = 0x800bbf78; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bbf78;
            }
            0x800bbf78 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x20));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bbe8c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800BBF98 (0x800bbf98, 68 bytes)
#[inline(never)]
pub fn f_800bbf98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bbf98) { return; }
        if c.prof { crate::prof::enter(c, 0x800bbf98); p = true; }
    }
    f_800bbf98_b(c, 0x800bbf98);
    if p { crate::prof::exit(c, 0x800bbf98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bbf98_at(c: &mut Cpu, pc: u32) { f_800bbf98_b(c, pc) }
#[inline(always)]
fn f_800bbf98_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bbf98;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bbf98, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bbf98 => { ins += 13;
                c.r[0] = c.mfspr(8);
                c.r[7] = c.r[5].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                c.r[5] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[6] = 0x7;
                c.r[8] = 0x0;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.lr = 0x800bbfcc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bbfcc;
            }
            0x800bbfcc => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bbf98: bad pc {:#x}", pc),
        }
    }
}

/// fn_800BBFDC (0x800bbfdc, 72 bytes)
#[inline(never)]
pub fn f_800bbfdc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bbfdc) { return; }
        if c.prof { crate::prof::enter(c, 0x800bbfdc); p = true; }
    }
    f_800bbfdc_b(c, 0x800bbfdc);
    if p { crate::prof::exit(c, 0x800bbfdc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bbfdc_at(c: &mut Cpu, pc: u32) { f_800bbfdc_b(c, pc) }
#[inline(always)]
fn f_800bbfdc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bbfdc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bbfdc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bbfdc => { ins += 14;
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x0;
                c.r[5] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[8] = 0x6;
                c.r[9] = 0x6;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.r[10] = 0x1;
                c.lr = 0x800bc014; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bb780(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bc014;
            }
            0x800bc014 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bbfdc: bad pc {:#x}", pc),
        }
    }
}

/// __GXUpdateBPMask (0x800bc024, 204 bytes)
#[inline(never)]
pub fn f_800bc024(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc024) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc024); p = true; }
    }
    f_800bc024_b(c, 0x800bc024);
    if p { crate::prof::exit(c, 0x800bc024); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc024_at(c: &mut Cpu, pc: u32) { f_800bc024_b(c, pc) }
#[inline(always)]
fn f_800bc024_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc024;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc024, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc024 => { ins += 8;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[6] = 0x0;
                c.r[4] = 0x0;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x204));
                c.r[0] = c.r[0].rotate_left(16) & 0x7;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                c.mtspr(9, c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x800bc0ac; continue; }
                pc = 0x800bc044;
            }
            0x800bc044 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x2);
                if cr_bit(cr, 2) { pc = 0x800bc084; continue; }
                pc = 0x800bc04c;
            }
            0x800bc04c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bc060; continue; }
                pc = 0x800bc050;
            }
            0x800bc050 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bc06c; continue; }
                pc = 0x800bc058;
            }
            0x800bc058 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bc078; continue; }
                pc = 0x800bc05c;
            }
            0x800bc05c => { ins += 1;
                pc = 0x800bc098; continue;
                pc = 0x800bc060;
            }
            0x800bc060 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x4);
                if !cr_bit(cr, 0) { pc = 0x800bc098; continue; }
                pc = 0x800bc068;
            }
            0x800bc068 => { ins += 1;
                pc = 0x800bc090; continue;
                pc = 0x800bc06c;
            }
            0x800bc06c => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x120));
                c.r[5] = c.r[0].rotate_left(0) & 0x7;
                pc = 0x800bc098; continue;
                pc = 0x800bc078;
            }
            0x800bc078 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x120));
                c.r[5] = c.r[0].rotate_left(26) & 0x7;
                pc = 0x800bc098; continue;
                pc = 0x800bc084;
            }
            0x800bc084 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x120));
                c.r[5] = c.r[0].rotate_left(20) & 0x7;
                pc = 0x800bc098; continue;
                pc = 0x800bc090;
            }
            0x800bc090 => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x120));
                c.r[5] = c.r[0].rotate_left(14) & 0x7;
                pc = 0x800bc098;
            }
            0x800bc098 => { ins += 5;
                c.r[0] = 0x1;
                c.r[0] = slw(c.r[0], c.r[5]);
                c.r[6] = c.r[6] | c.r[0];
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800bc044; continue; }
                pc = 0x800bc0ac;
            }
            0x800bc0ac => { ins += 5;
                c.r[4] = c.r[3].wrapping_add(0x124);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x124));
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[6]);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800bc0c0;
            }
            0x800bc0c0 => { ins += 12;
                c.r[0] = c.r[3].rotate_left(0) & 0xffffff00;
                c.r[0] = c.r[0] | c.r[6];
                c.stw(c.r[4], c.r[0]);
                c.r[0] = 0x61;
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x124));
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc024: bad pc {:#x}", pc),
        }
    }
}

/// __GXFlushTextureState (0x800bc0f0, 36 bytes)
#[inline(never)]
pub fn f_800bc0f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc0f0) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc0f0); p = true; }
    }
    f_800bc0f0_b(c, 0x800bc0f0);
    if p { crate::prof::exit(c, 0x800bc0f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc0f0_at(c: &mut Cpu, pc: u32) { f_800bc0f0_b(c, pc) }
#[inline(always)]
fn f_800bc0f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc0f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc0f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc0f0 => { ins += 9;
                c.r[0] = 0x61;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x124));
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc0f0: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevOp (0x800bc114, 140 bytes)
#[inline(never)]
pub fn f_800bc114(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc114) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc114); p = true; }
    }
    f_800bc114_b(c, 0x800bc114);
    if p { crate::prof::exit(c, 0x800bc114); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc114_at(c: &mut Cpu, pc: u32) { f_800bc114_b(c, pc) }
#[inline(always)]
fn f_800bc114_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc114;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc114, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc114 => { ins += 4;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.r[5] = 0x80310000;
                c.r[0] = c.r[5].wrapping_add(0x3590);
                if !cr_bit(cr, 2) { pc = 0x800bc134; continue; }
                pc = 0x800bc124;
            }
            0x800bc124 => { ins += 4;
                c.r[4] = c.r[4].rotate_left(2) & 0xfffffffc;
                c.r[5] = c.r[0].wrapping_add(c.r[4]);
                c.r[9] = c.r[5].wrapping_add(0x28);
                pc = 0x800bc148; continue;
                pc = 0x800bc134;
            }
            0x800bc134 => { ins += 5;
                c.r[4] = c.r[4].rotate_left(2) & 0xfffffffc;
                c.r[5] = c.r[0].wrapping_add(c.r[4]);
                c.r[9] = c.r[0].wrapping_add(c.r[4]);
                c.r[5] = c.r[5].wrapping_add(0x14);
                c.r[9] = c.r[9].wrapping_add(0x3c);
                pc = 0x800bc148;
            }
            0x800bc148 => { ins += 22;
                c.r[7] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lwz_t(ins, c.r[5]);
                c.r[5] = 0x61;
                c.r[6] = c.r[7].wrapping_add(c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[6].wrapping_add(0x130));
                c.r[4] = 0xcc010000;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[3] = c.r[3].rotate_left(0) & 0xff000000;
                c.r[3] = (c.r[0].rotate_left(0) & 0xffffff) | (c.r[3] & 0xff000000);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[3]);
                c.r[0] = 0x0;
                c.stw(c.r[6].wrapping_add(0x130), c.r[3]);
                c.r[8] = c.lwz_t(ins, c.r[6].wrapping_add(0x170));
                c.r[3] = c.lwz_t(ins, c.r[9]);
                c.r[8] = c.r[8].rotate_left(0) & 0xff00000f;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[8] = (c.r[3].rotate_left(0) & 0xfffff0) | (c.r[8] & 0xff00000f);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[8]);
                c.stw(c.r[6].wrapping_add(0x170), c.r[8]);
                c.sth(c.r[7].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc114: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevColorIn (0x800bc1a0, 68 bytes)
#[inline(never)]
pub fn f_800bc1a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc1a0) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc1a0); p = true; }
    }
    f_800bc1a0_b(c, 0x800bc1a0);
    if p { crate::prof::exit(c, 0x800bc1a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc1a0_at(c: &mut Cpu, pc: u32) { f_800bc1a0_b(c, pc) }
#[inline(always)]
fn f_800bc1a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc1a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc1a0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc1a0 => { ins += 17;
                c.r[9] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = 0x61;
                c.r[8] = c.r[9].wrapping_add(c.r[3]);
                c.r[10] = c.lwz_t(ins, c.r[8].wrapping_add(0x130));
                c.r[3] = 0xcc010000;
                c.r[10] = (c.r[4].rotate_left(12) & 0xf000) | (c.r[10] & 0xffff0fff);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[4] = c.r[10].wrapping_add(0x0);
                c.r[4] = (c.r[5].rotate_left(8) & 0xf00) | (c.r[4] & 0xfffff0ff);
                c.r[4] = (c.r[6].rotate_left(4) & 0xf0) | (c.r[4] & 0xffffff0f);
                c.r[4] = (c.r[7].rotate_left(0) & 0xf) | (c.r[4] & 0xfffffff0);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.r[0] = 0x0;
                c.stw(c.r[8].wrapping_add(0x130), c.r[4]);
                c.sth(c.r[9].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc1a0: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevAlphaIn (0x800bc1e4, 68 bytes)
#[inline(never)]
pub fn f_800bc1e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc1e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc1e4); p = true; }
    }
    f_800bc1e4_b(c, 0x800bc1e4);
    if p { crate::prof::exit(c, 0x800bc1e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc1e4_at(c: &mut Cpu, pc: u32) { f_800bc1e4_b(c, pc) }
#[inline(always)]
fn f_800bc1e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc1e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc1e4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc1e4 => { ins += 17;
                c.r[9] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = 0x61;
                c.r[8] = c.r[9].wrapping_add(c.r[3]);
                c.r[10] = c.lwz_t(ins, c.r[8].wrapping_add(0x170));
                c.r[3] = 0xcc010000;
                c.r[10] = (c.r[4].rotate_left(13) & 0xe000) | (c.r[10] & 0xffff1fff);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[4] = c.r[10].wrapping_add(0x0);
                c.r[4] = (c.r[5].rotate_left(10) & 0x1c00) | (c.r[4] & 0xffffe3ff);
                c.r[4] = (c.r[6].rotate_left(7) & 0x380) | (c.r[4] & 0xfffffc7f);
                c.r[4] = (c.r[7].rotate_left(4) & 0x70) | (c.r[4] & 0xffffff8f);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.r[0] = 0x0;
                c.stw(c.r[8].wrapping_add(0x170), c.r[4]);
                c.sth(c.r[9].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc1e4: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevColorOp (0x800bc228, 104 bytes)
#[inline(never)]
pub fn f_800bc228(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc228) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc228); p = true; }
    }
    f_800bc228_b(c, 0x800bc228);
    if p { crate::prof::exit(c, 0x800bc228); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc228_at(c: &mut Cpu, pc: u32) { f_800bc228_b(c, pc) }
#[inline(always)]
fn f_800bc228_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc228;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc228, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc228 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[9] = c.r[3].rotate_left(2) & 0xfffffffc;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x1);
                c.r[3] = c.r[0].wrapping_add(c.r[9]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x130));
                c.r[3] = (c.r[4].rotate_left(18) & 0x40000) | (c.r[3] & 0xfffbffff);
                c.r[10] = c.r[3].wrapping_add(0x0);
                if cr_bit(cr, 1) { pc = 0x800bc254; continue; }
                pc = 0x800bc248;
            }
            0x800bc248 => { ins += 3;
                c.r[10] = (c.r[6].rotate_left(20) & 0x300000) | (c.r[10] & 0xffcfffff);
                c.r[10] = (c.r[5].rotate_left(16) & 0x30000) | (c.r[10] & 0xfffcffff);
                pc = 0x800bc260; continue;
                pc = 0x800bc254;
            }
            0x800bc254 => { ins += 3;
                c.r[0] = 0x3;
                c.r[10] = (c.r[4].rotate_left(19) & 0x300000) | (c.r[10] & 0xffcfffff);
                c.r[10] = (c.r[0].rotate_left(16) & 0x30000) | (c.r[10] & 0xfffcffff);
                pc = 0x800bc260;
            }
            0x800bc260 => { ins += 12;
                c.r[0] = 0x61;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[10] = (c.r[7].rotate_left(19) & 0x80000) | (c.r[10] & 0xfff7ffff);
                c.r[10] = (c.r[8].rotate_left(22) & 0xc00000) | (c.r[10] & 0xff3fffff);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[10]);
                c.r[3] = c.r[4].wrapping_add(c.r[9]);
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x130), c.r[10]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc228: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevAlphaOp (0x800bc290, 104 bytes)
#[inline(never)]
pub fn f_800bc290(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc290) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc290); p = true; }
    }
    f_800bc290_b(c, 0x800bc290);
    if p { crate::prof::exit(c, 0x800bc290); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc290_at(c: &mut Cpu, pc: u32) { f_800bc290_b(c, pc) }
#[inline(always)]
fn f_800bc290_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc290;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc290, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc290 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[9] = c.r[3].rotate_left(2) & 0xfffffffc;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x1);
                c.r[3] = c.r[0].wrapping_add(c.r[9]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x170));
                c.r[3] = (c.r[4].rotate_left(18) & 0x40000) | (c.r[3] & 0xfffbffff);
                c.r[10] = c.r[3].wrapping_add(0x0);
                if cr_bit(cr, 1) { pc = 0x800bc2bc; continue; }
                pc = 0x800bc2b0;
            }
            0x800bc2b0 => { ins += 3;
                c.r[10] = (c.r[6].rotate_left(20) & 0x300000) | (c.r[10] & 0xffcfffff);
                c.r[10] = (c.r[5].rotate_left(16) & 0x30000) | (c.r[10] & 0xfffcffff);
                pc = 0x800bc2c8; continue;
                pc = 0x800bc2bc;
            }
            0x800bc2bc => { ins += 3;
                c.r[0] = 0x3;
                c.r[10] = (c.r[4].rotate_left(19) & 0x300000) | (c.r[10] & 0xffcfffff);
                c.r[10] = (c.r[0].rotate_left(16) & 0x30000) | (c.r[10] & 0xfffcffff);
                pc = 0x800bc2c8;
            }
            0x800bc2c8 => { ins += 12;
                c.r[0] = 0x61;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[10] = (c.r[7].rotate_left(19) & 0x80000) | (c.r[10] & 0xfff7ffff);
                c.r[10] = (c.r[8].rotate_left(22) & 0xc00000) | (c.r[10] & 0xff3fffff);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[10]);
                c.r[3] = c.r[4].wrapping_add(c.r[9]);
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x170), c.r[10]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc290: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevColor (0x800bc2f8, 116 bytes)
#[inline(never)]
pub fn f_800bc2f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc2f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc2f8); p = true; }
    }
    f_800bc2f8_b(c, 0x800bc2f8);
    if p { crate::prof::exit(c, 0x800bc2f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc2f8_at(c: &mut Cpu, pc: u32) { f_800bc2f8_b(c, pc) }
#[inline(always)]
fn f_800bc2f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc2f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc2f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc2f8 => { ins += 29;
                c.r[0] = c.lbz_t(ins, c.r[4].wrapping_add(0x3));
                c.r[8] = c.r[3].rotate_left(1) & 0xfffffffe;
                c.r[5] = c.lbz_t(ins, c.r[4]);
                c.r[7] = c.r[8].wrapping_add(0xe0);
                c.r[9] = c.r[0].rotate_left(12) & 0xfffff000;
                c.r[9] = (c.r[5].rotate_left(0) & 0xff) | (c.r[9] & 0xffffff00);
                c.r[10] = c.lbz_t(ins, c.r[4].wrapping_add(0x2));
                c.r[6] = c.lbz_t(ins, c.r[4].wrapping_add(0x1));
                c.r[7] = c.r[7].rotate_left(24) & 0xff000000;
                c.r[5] = 0x61;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[4] = 0xcc010000;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[7] = (c.r[9].rotate_left(0) & 0xfffff) | (c.r[7] & 0xfff00000);
                c.r[0] = c.r[8].wrapping_add(0xe1);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[7]);
                c.r[7] = c.r[6].rotate_left(12) & 0xfffff000;
                c.r[6] = c.r[0].rotate_left(24) & 0xff000000;
                c.r[7] = (c.r[10].rotate_left(0) & 0xff) | (c.r[7] & 0xffffff00);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[6] = (c.r[7].rotate_left(0) & 0xfffff) | (c.r[6] & 0xfff00000);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                c.r[0] = 0x0;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc2f8: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevColorS10 (0x800bc36c, 116 bytes)
#[inline(never)]
pub fn f_800bc36c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc36c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc36c); p = true; }
    }
    f_800bc36c_b(c, 0x800bc36c);
    if p { crate::prof::exit(c, 0x800bc36c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc36c_at(c: &mut Cpu, pc: u32) { f_800bc36c_b(c, pc) }
#[inline(always)]
fn f_800bc36c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc36c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc36c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc36c => { ins += 29;
                c.r[0] = c.lha_t(ins, c.r[4].wrapping_add(0x6));
                c.r[9] = c.r[3].rotate_left(1) & 0xfffffffe;
                c.r[5] = c.lha_t(ins, c.r[4]);
                c.r[8] = c.r[9].wrapping_add(0xe0);
                c.r[10] = c.r[0].rotate_left(12) & 0x7ff000;
                c.r[10] = (c.r[5].rotate_left(0) & 0x7ff) | (c.r[10] & 0xfffff800);
                c.r[7] = c.lha_t(ins, c.r[4].wrapping_add(0x4));
                c.r[6] = c.lha_t(ins, c.r[4].wrapping_add(0x2));
                c.r[8] = c.r[8].rotate_left(24) & 0xff000000;
                c.r[5] = 0x61;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[4] = 0xcc010000;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[8] = (c.r[10].rotate_left(0) & 0x7fffff) | (c.r[8] & 0xff800000);
                c.r[0] = c.r[9].wrapping_add(0xe1);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[8]);
                c.r[8] = c.r[6].rotate_left(12) & 0x7ff000;
                c.r[6] = c.r[0].rotate_left(24) & 0xff000000;
                c.r[8] = (c.r[7].rotate_left(0) & 0x7ff) | (c.r[8] & 0xfffff800);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[6] = (c.r[8].rotate_left(0) & 0x7fffff) | (c.r[6] & 0xff800000);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                c.r[0] = 0x0;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc36c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevKColor (0x800bc3e0, 116 bytes)
#[inline(never)]
pub fn f_800bc3e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc3e0) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc3e0); p = true; }
    }
    f_800bc3e0_b(c, 0x800bc3e0);
    if p { crate::prof::exit(c, 0x800bc3e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc3e0_at(c: &mut Cpu, pc: u32) { f_800bc3e0_b(c, pc) }
#[inline(always)]
fn f_800bc3e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc3e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc3e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc3e0 => { ins += 29;
                c.r[0] = c.lbz_t(ins, c.r[4].wrapping_add(0x3));
                c.r[5] = c.r[3].rotate_left(1) & 0xfffffffe;
                c.r[6] = c.lbz_t(ins, c.r[4]);
                c.r[7] = c.r[5].wrapping_add(0xe0);
                c.r[0] = c.r[0].rotate_left(12) & 0xfffff000;
                c.r[0] = (c.r[6].rotate_left(0) & 0xff) | (c.r[0] & 0xffffff00);
                c.r[6] = c.lbz_t(ins, c.r[4].wrapping_add(0x1));
                c.r[9] = c.lbz_t(ins, c.r[4].wrapping_add(0x2));
                c.r[0] = c.r[0].rotate_left(0) & 0xff0fffff;
                c.r[8] = c.r[0] | 0x800000;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[7] = c.r[7].rotate_left(24) & 0xff000000;
                c.r[0] = 0x61;
                c.r[4] = 0xcc010000;
                c.r[6] = c.r[6].rotate_left(12) & 0xfffff000;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = (c.r[8].rotate_left(0) & 0xffffff) | (c.r[7] & 0xff000000);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[7]);
                c.r[6] = (c.r[9].rotate_left(0) & 0xff) | (c.r[6] & 0xffffff00);
                c.r[5] = c.r[5].wrapping_add(0xe1);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.r[6].rotate_left(0) & 0xff0fffff;
                c.r[6] = c.r[0] | 0x800000;
                c.r[0] = c.r[5].rotate_left(24) & 0xff000000;
                c.r[0] = (c.r[6].rotate_left(0) & 0xffffff) | (c.r[0] & 0xff000000);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc3e0: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevKColorSel (0x800bc454, 108 bytes)
#[inline(never)]
pub fn f_800bc454(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc454) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc454); p = true; }
    }
    f_800bc454_b(c, 0x800bc454);
    if p { crate::prof::exit(c, 0x800bc454); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc454_at(c: &mut Cpu, pc: u32) { f_800bc454_b(c, pc) }
#[inline(always)]
fn f_800bc454_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc454;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc454, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc454 => { ins += 7;
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[5] = (v >> 1) as u32; }
                c.r[6] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[3] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[7] = c.r[3].wrapping_add(0x1b0);
                c.r[7] = c.r[6].wrapping_add(c.r[7]);
                if cr_bit(cr, 2) { pc = 0x800bc488; continue; }
                pc = 0x800bc470;
            }
            0x800bc470 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[0] = c.r[4].rotate_left(14) & 0xffffc000;
                c.r[3] = c.r[3].rotate_left(0) & 0xfff83fff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[7], c.r[0]);
                pc = 0x800bc49c; continue;
                pc = 0x800bc488;
            }
            0x800bc488 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[0] = c.r[4].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[3].rotate_left(0) & 0xfffffe0f;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[7], c.r[0]);
                pc = 0x800bc49c;
            }
            0x800bc49c => { ins += 9;
                c.r[0] = 0x61;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = c.lwz_t(ins, c.r[7]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[4]);
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc454: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevKAlphaSel (0x800bc4c0, 108 bytes)
#[inline(never)]
pub fn f_800bc4c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc4c0) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc4c0); p = true; }
    }
    f_800bc4c0_b(c, 0x800bc4c0);
    if p { crate::prof::exit(c, 0x800bc4c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc4c0_at(c: &mut Cpu, pc: u32) { f_800bc4c0_b(c, pc) }
#[inline(always)]
fn f_800bc4c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc4c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc4c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc4c0 => { ins += 7;
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[5] = (v >> 1) as u32; }
                c.r[6] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.r[3] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[7] = c.r[3].wrapping_add(0x1b0);
                c.r[7] = c.r[6].wrapping_add(c.r[7]);
                if cr_bit(cr, 2) { pc = 0x800bc4f4; continue; }
                pc = 0x800bc4dc;
            }
            0x800bc4dc => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[0] = c.r[4].rotate_left(19) & 0xfff80000;
                c.r[3] = c.r[3].rotate_left(0) & 0xff07ffff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[7], c.r[0]);
                pc = 0x800bc508; continue;
                pc = 0x800bc4f4;
            }
            0x800bc4f4 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[0] = c.r[4].rotate_left(9) & 0xfffffe00;
                c.r[3] = c.r[3].rotate_left(0) & 0xffffc1ff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[7], c.r[0]);
                pc = 0x800bc508;
            }
            0x800bc508 => { ins += 9;
                c.r[0] = 0x61;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = c.lwz_t(ins, c.r[7]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[4]);
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc4c0: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevSwapMode (0x800bc52c, 84 bytes)
#[inline(never)]
pub fn f_800bc52c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc52c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc52c); p = true; }
    }
    f_800bc52c_b(c, 0x800bc52c);
    if p { crate::prof::exit(c, 0x800bc52c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc52c_at(c: &mut Cpu, pc: u32) { f_800bc52c_b(c, pc) }
#[inline(always)]
fn f_800bc52c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc52c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc52c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc52c => { ins += 21;
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[7] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[8] = c.r[3].wrapping_add(0x170);
                c.r[8] = c.r[7].wrapping_add(c.r[8]);
                c.r[0] = c.lwz_t(ins, c.r[8]);
                c.r[5] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x61;
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffc;
                c.r[0] = c.r[0] | c.r[4];
                c.stw(c.r[8], c.r[0]);
                c.r[4] = 0xcc010000;
                c.r[0] = 0x0;
                c.r[6] = c.lwz_t(ins, c.r[8]);
                c.r[6] = c.r[6].rotate_left(0) & 0xfffffff3;
                c.r[5] = c.r[6] | c.r[5];
                c.stw(c.r[8], c.r[5]);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[8]);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[7].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc52c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevSwapModeTable (0x800bc580, 152 bytes)
#[inline(never)]
pub fn f_800bc580(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc580) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc580); p = true; }
    }
    f_800bc580_b(c, 0x800bc580);
    if p { crate::prof::exit(c, 0x800bc580); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc580_at(c: &mut Cpu, pc: u32) { f_800bc580_b(c, pc) }
#[inline(always)]
fn f_800bc580_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc580;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc580, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc580 => { ins += 38;
                c.r[10] = c.r[3].rotate_left(1) & 0xfffffffe;
                c.r[9] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[3] = c.r[3].rotate_left(3) & 0xfffffff8;
                c.r[11] = c.r[3].wrapping_add(0x1b0);
                c.r[11] = c.r[9].wrapping_add(c.r[11]);
                c.r[8] = c.lwz_t(ins, c.r[11]);
                c.r[0] = c.r[10].wrapping_add(0x1);
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[8].rotate_left(0) & 0xfffffffc;
                c.r[0] = c.r[0] | c.r[4];
                c.stw(c.r[11], c.r[0]);
                c.r[10] = c.r[3].wrapping_add(0x1b0);
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz_t(ins, c.r[11]);
                c.r[8] = 0x61;
                c.r[5] = 0xcc010000;
                c.r[3] = c.r[3].rotate_left(0) & 0xfffffff3;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[11], c.r[0]);
                c.r[10] = c.r[9].wrapping_add(c.r[10]);
                c.r[3] = c.r[7].rotate_left(2) & 0xfffffffc;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[8]);
                c.r[0] = 0x0;
                c.r[4] = c.lwz_t(ins, c.r[11]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[10]);
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffffc;
                c.r[4] = c.r[4] | c.r[6];
                c.stw(c.r[10], c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[10]);
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffff3;
                c.r[3] = c.r[4] | c.r[3];
                c.stw(c.r[10], c.r[3]);
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[8]);
                c.r[3] = c.lwz_t(ins, c.r[10]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[9].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc580: bad pc {:#x}", pc),
        }
    }
}

/// GXSetAlphaCompare (0x800bc618, 84 bytes)
#[inline(never)]
pub fn f_800bc618(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc618) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc618); p = true; }
    }
    f_800bc618_b(c, 0x800bc618);
    if p { crate::prof::exit(c, 0x800bc618); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc618_at(c: &mut Cpu, pc: u32) { f_800bc618_b(c, pc) }
#[inline(always)]
fn f_800bc618_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc618;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc618, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc618 => { ins += 21;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[0] | 0xf3000000;
                c.r[0] = c.r[0].rotate_left(0) & 0xffff00ff;
                c.r[0] = (c.r[7].rotate_left(8) & 0xff00) | (c.r[0] & 0xffff00ff);
                c.r[7] = c.r[0].rotate_left(0) & 0xfff8ffff;
                c.r[0] = c.r[3].rotate_left(16) & 0xffff0000;
                c.r[0] = c.r[7] | c.r[0];
                c.r[3] = c.r[0].rotate_left(0) & 0xffc7ffff;
                c.r[0] = c.r[6].rotate_left(19) & 0xfff80000;
                c.r[6] = c.r[3] | c.r[0];
                c.r[0] = 0x61;
                c.r[3] = 0xcc010000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[6] = c.r[6].rotate_left(0) & 0xff3fffff;
                c.r[0] = c.r[5].rotate_left(22) & 0xffc00000;
                c.r[0] = c.r[6] | c.r[0];
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc618: bad pc {:#x}", pc),
        }
    }
}

/// GXSetZTexture (0x800bc66c, 132 bytes)
#[inline(never)]
pub fn f_800bc66c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc66c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc66c); p = true; }
    }
    f_800bc66c_b(c, 0x800bc66c);
    if p { crate::prof::exit(c, 0x800bc66c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc66c_at(c: &mut Cpu, pc: u32) { f_800bc66c_b(c, pc) }
#[inline(always)]
fn f_800bc66c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc66c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc66c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc66c => { ins += 4;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x13);
                c.r[0] = c.r[5].rotate_left(0) & 0xffffff;
                c.r[7] = c.r[0] | 0xf4000000;
                if cr_bit(cr, 2) { pc = 0x800bc6a0; continue; }
                pc = 0x800bc67c;
            }
            0x800bc67c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bc68c; continue; }
                pc = 0x800bc680;
            }
            0x800bc680 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x11);
                if cr_bit(cr, 2) { pc = 0x800bc698; continue; }
                pc = 0x800bc688;
            }
            0x800bc688 => { ins += 1;
                pc = 0x800bc6b0; continue;
                pc = 0x800bc68c;
            }
            0x800bc68c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x16);
                if cr_bit(cr, 2) { pc = 0x800bc6a8; continue; }
                pc = 0x800bc694;
            }
            0x800bc694 => { ins += 1;
                pc = 0x800bc6b0; continue;
                pc = 0x800bc698;
            }
            0x800bc698 => { ins += 2;
                c.r[6] = 0x0;
                pc = 0x800bc6b4; continue;
                pc = 0x800bc6a0;
            }
            0x800bc6a0 => { ins += 2;
                c.r[6] = 0x1;
                pc = 0x800bc6b4; continue;
                pc = 0x800bc6a8;
            }
            0x800bc6a8 => { ins += 2;
                c.r[6] = 0x2;
                pc = 0x800bc6b4; continue;
                pc = 0x800bc6b0;
            }
            0x800bc6b0 => { ins += 1;
                c.r[6] = 0x2;
                pc = 0x800bc6b4;
            }
            0x800bc6b4 => { ins += 15;
                c.r[0] = 0x61;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[6] = c.r[6].rotate_left(0) & 0xfffffff3;
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[7]);
                c.r[3] = c.r[6] | c.r[3];
                c.r[3] = c.r[3].rotate_left(0) & 0xffffff;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[3] = c.r[3] | 0xf5000000;
                c.r[0] = 0x0;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc66c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetTevOrder (0x800bc6f0, 472 bytes)
#[inline(never)]
pub fn f_800bc6f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc6f0) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc6f0); p = true; }
    }
    f_800bc6f0_b(c, 0x800bc6f0);
    if p { crate::prof::exit(c, 0x800bc6f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc6f0_at(c: &mut Cpu, pc: u32) { f_800bc6f0_b(c, pc) }
#[inline(never)]
fn f_800bc6f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc6f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc6f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc6f0 => { ins += 13;
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[7] = (v >> 1) as u32; }
                c.r[9] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[10] = c.r[5].rotate_left(0) & 0xfffffeff;
                { let ci = c.ca(); c.r[7] = c.addc(c.r[7], 0, ci); }
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[8] = c.r[9].wrapping_add(c.r[0]);
                c.r[7] = c.r[7].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[8].wrapping_add(0x49c), c.r[5]);
                c.r[7] = c.r[7].wrapping_add(0x100);
                cr_cmpu(&mut cr, c.xer, 0, c.r[10], 0x8);
                c.r[0] = c.r[10].wrapping_add(0x0);
                c.r[7] = c.r[9].wrapping_add(c.r[7]);
                if cr_bit(cr, 0) { pc = 0x800bc72c; continue; }
                pc = 0x800bc724;
            }
            0x800bc724 => { ins += 2;
                c.r[9] = 0x0;
                pc = 0x800bc730; continue;
                pc = 0x800bc72c;
            }
            0x800bc72c => { ins += 1;
                c.r[9] = c.r[0] | c.r[0];
                pc = 0x800bc730;
            }
            0x800bc730 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x8);
                if cr_bit(cr, 0) { pc = 0x800bc75c; continue; }
                pc = 0x800bc738;
            }
            0x800bc738 => { ins += 9;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x1;
                c.r[0] = slw(c.r[0], c.r[3]);
                c.r[8] = c.lwz_t(ins, c.r[4].wrapping_add(0x4e0));
                c.r[10] = c.r[4].wrapping_add(0x4e0);
                c.r[4] = 0x0;
                c.r[0] = c.r[8] & !c.r[0];
                c.stw(c.r[10], c.r[0]);
                pc = 0x800bc778; continue;
                pc = 0x800bc75c;
            }
            0x800bc75c => { ins += 7;
                c.r[8] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x1;
                c.r[0] = slw(c.r[0], c.r[3]);
                c.r[10] = c.r[8].wrapping_add(0x4e0);
                c.r[8] = c.lwz_t(ins, c.r[8].wrapping_add(0x4e0));
                c.r[0] = c.r[8] | c.r[0];
                c.stw(c.r[10], c.r[0]);
                pc = 0x800bc778;
            }
            0x800bc778 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800bc810; continue; }
                pc = 0x800bc780;
            }
            0x800bc780 => { ins += 12;
                c.r[8] = c.lwz_t(ins, c.r[7]);
                c.r[3] = c.r[9].rotate_left(12) & 0xfffff000;
                c.r[0] = c.r[4].rotate_left(15) & 0xffff8000;
                c.r[4] = c.r[8].rotate_left(0) & 0xffff8fff;
                c.r[3] = c.r[4] | c.r[3];
                c.stw(c.r[7], c.r[3]);
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0xff);
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[3] = c.r[3].rotate_left(0) & 0xfffc7fff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[7], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800bc7b8; continue; }
                pc = 0x800bc7b0;
            }
            0x800bc7b0 => { ins += 2;
                c.r[0] = 0x7;
                pc = 0x800bc7cc; continue;
                pc = 0x800bc7b8;
            }
            0x800bc7b8 => { ins += 5;
                c.r[3] = 0x80310000;
                c.r[4] = c.r[6].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[3].wrapping_add(0x35e0);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[3]);
                pc = 0x800bc7cc;
            }
            0x800bc7cc => { ins += 8;
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[0] = c.r[0].rotate_left(19) & 0xfff80000;
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0xff);
                c.r[3] = c.r[3].rotate_left(0) & 0xffc7ffff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[7], c.r[0]);
                c.r[4] = 0x0;
                if cr_bit(cr, 2) { pc = 0x800bc7f8; continue; }
                pc = 0x800bc7ec;
            }
            0x800bc7ec => { ins += 2;
                c.r[0] = c.r[5].rotate_left(0) & 0x100;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800bc7f8; continue; }
                pc = 0x800bc7f4;
            }
            0x800bc7f4 => { ins += 1;
                c.r[4] = 0x1;
                pc = 0x800bc7f8;
            }
            0x800bc7f8 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[0] = c.r[4].rotate_left(18) & 0xfffc0000;
                c.r[3] = c.r[3].rotate_left(0) & 0xfffbffff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[7], c.r[0]);
                pc = 0x800bc898; continue;
                pc = 0x800bc810;
            }
            0x800bc810 => { ins += 11;
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[0] = c.r[4].rotate_left(3) & 0xfffffff8;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0xff);
                c.r[3] = c.r[3].rotate_left(0) & 0xfffffff8;
                c.r[3] = c.r[3] | c.r[9];
                c.stw(c.r[7], c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[3] = c.r[3].rotate_left(0) & 0xffffffc7;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[7], c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800bc844; continue; }
                pc = 0x800bc83c;
            }
            0x800bc83c => { ins += 2;
                c.r[0] = 0x7;
                pc = 0x800bc858; continue;
                pc = 0x800bc844;
            }
            0x800bc844 => { ins += 5;
                c.r[3] = 0x80310000;
                c.r[4] = c.r[6].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[3].wrapping_add(0x35e0);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[3]);
                pc = 0x800bc858;
            }
            0x800bc858 => { ins += 8;
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[0] = c.r[0].rotate_left(7) & 0xffffff80;
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0xff);
                c.r[3] = c.r[3].rotate_left(0) & 0xfffffc7f;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[7], c.r[0]);
                c.r[4] = 0x0;
                if cr_bit(cr, 2) { pc = 0x800bc884; continue; }
                pc = 0x800bc878;
            }
            0x800bc878 => { ins += 2;
                c.r[0] = c.r[5].rotate_left(0) & 0x100;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800bc884; continue; }
                pc = 0x800bc880;
            }
            0x800bc880 => { ins += 1;
                c.r[4] = 0x1;
                pc = 0x800bc884;
            }
            0x800bc884 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[7]);
                c.r[0] = c.r[4].rotate_left(6) & 0xffffffc0;
                c.r[3] = c.r[3].rotate_left(0) & 0xffffffbf;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[7], c.r[0]);
                pc = 0x800bc898;
            }
            0x800bc898 => { ins += 12;
                c.r[0] = 0x61;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.r[4] = c.lwz_t(ins, c.r[7]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[4]);
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4f4));
                c.r[0] = c.r[0] | 0x1;
                c.stw(c.r[3].wrapping_add(0x4f4), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc6f0: bad pc {:#x}", pc),
        }
    }
}

/// GXSetNumTevStages (0x800bc8c8, 48 bytes)
#[inline(never)]
pub fn f_800bc8c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc8c8) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc8c8); p = true; }
    }
    f_800bc8c8_b(c, 0x800bc8c8);
    if p { crate::prof::exit(c, 0x800bc8c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc8c8_at(c: &mut Cpu, pc: u32) { f_800bc8c8_b(c, pc) }
#[inline(always)]
fn f_800bc8c8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc8c8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc8c8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc8c8 => { ins += 12;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x204));
                c.r[0] = c.r[0].rotate_left(10) & 0xfffffc00;
                c.r[3] = c.r[3].rotate_left(0) & 0xffffc3ff;
                c.r[0] = c.r[3] | c.r[0];
                c.stw(c.r[4].wrapping_add(0x204), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x4f4));
                c.r[0] = c.r[0] | 0x4;
                c.stw(c.r[4].wrapping_add(0x4f4), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc8c8: bad pc {:#x}", pc),
        }
    }
}

/// GXSetFog (0x800bc8f8, 540 bytes)
#[inline(never)]
pub fn f_800bc8f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bc8f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800bc8f8); p = true; }
    }
    f_800bc8f8_b(c, 0x800bc8f8);
    if p { crate::prof::exit(c, 0x800bc8f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bc8f8_at(c: &mut Cpu, pc: u32) { f_800bc8f8_b(c, pc) }
#[inline(never)]
fn f_800bc8f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bc8f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bc8f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bc8f8 => { ins += 13;
                c.r[0] = c.mfspr(8);
                c.r[10] = 0x0;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[11] = 0x0;
                { let ea = c.r[1].wrapping_add(0xffffffb8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x44), c.r[31]);
                c.r[31] = c.r[3].rotate_left(29) & 0x1;
                cr_rc(&mut cr, c.xer, c.r[31]);
                c.stw(c.r[1].wrapping_add(0x40), c.r[30]);
                c.r[30] = c.r[3].rotate_left(0) & 0x7;
                c.stw(c.r[1].wrapping_add(0x3c), c.r[29]);
                c.r[29] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x38), c.r[28]);
                if cr_bit(cr, 2) { pc = 0x800bc974; continue; }
                pc = 0x800bc92c;
            }
            0x800bc92c => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[4], c.f[3]);
                if cr_bit(cr, 2) { pc = 0x800bc93c; continue; }
                pc = 0x800bc934;
            }
            0x800bc934 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[2], c.f[1]);
                if !cr_bit(cr, 2) { pc = 0x800bc94c; continue; }
                pc = 0x800bc93c;
            }
            0x800bc93c => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff8ce8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800bca48; continue;
                pc = 0x800bc94c;
            }
            0x800bc94c => { ins += 10;
                { let v = s(c.f[2] - c.f[1]); c.f[5] = v; c.ps1[5] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8cec); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                { let v = s(c.f[4] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] - c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[6] / c.f[5]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[3] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x2c); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800bca48; continue;
                pc = 0x800bc974;
            }
            0x800bc974 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[4], c.f[3]);
                if cr_bit(cr, 2) { pc = 0x800bc984; continue; }
                pc = 0x800bc97c;
            }
            0x800bc97c => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[2], c.f[1]);
                if !cr_bit(cr, 2) { pc = 0x800bc994; continue; }
                pc = 0x800bc984;
            }
            0x800bc984 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff8ce8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[2].wrapping_add(0xffff8cf0); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.f[5] = c.f[3];
                pc = 0x800bc9b0; continue;
                pc = 0x800bc994;
            }
            0x800bc994 => { ins += 7;
                { let v = s(c.f[4] - c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] - c.f[1]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(c.f[4] * c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[4] / c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[0] * c.f[5]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[1] / c.f[5]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(c.f[2] / c.f[0]); c.f[3] = v; c.ps1[3] = v; }
                pc = 0x800bc9b0;
            }
            0x800bc9b0 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff8cf0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                { let ea = c.r[2].wrapping_add(0xffff8cf8); c.f[0] = c.lfd(ea); }
                pc = 0x800bc9c8; continue;
                pc = 0x800bc9c0;
            }
            0x800bc9c0 => { ins += 2;
                { let v = s(c.f[4] * c.f[1]); c.f[4] = v; c.ps1[4] = v; }
                c.r[3] = c.r[3].wrapping_add(0x1);
                pc = 0x800bc9c8;
            }
            0x800bc9c8 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[4], c.f[0]);
                if cr_bit(cr, 1) { pc = 0x800bc9c0; continue; }
                pc = 0x800bc9d0;
            }
            0x800bc9d0 => { ins += 4;
                { let ea = c.r[2].wrapping_add(0xffff8d08); c.f[0] = c.lfd(ea); }
                { let ea = c.r[2].wrapping_add(0xffff8d00); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[2].wrapping_add(0xffff8ce8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800bc9e8; continue;
                pc = 0x800bc9e0;
            }
            0x800bc9e0 => { ins += 2;
                { let v = s(c.f[4] * c.f[2]); c.f[4] = v; c.ps1[4] = v; }
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x800bc9e8;
            }
            0x800bc9e8 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[4], c.f[1]);
                if !cr_bit(cr, 1) { pc = 0x800bc9f8; continue; }
                pc = 0x800bc9f0;
            }
            0x800bc9f0 => { ins += 2;
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[4], c.f[0]);
                if cr_bit(cr, 0) { pc = 0x800bc9e0; continue; }
                pc = 0x800bc9f8;
            }
            0x800bc9f8 => { ins += 16;
                c.r[28] = c.r[3].wrapping_add(0x1);
                { let ea = c.r[2].wrapping_add(0xffff8d10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[0] = 0x1;
                { let ea = c.r[2].wrapping_add(0xffff8d18); c.f[2] = c.lfd(ea); }
                c.r[0] = slw(c.r[0], c.r[28]);
                { let v = s(c.f[0] * c.f[4]); c.f[1] = v; c.ps1[1] = v; }
                c.r[3] = c.r[0] ^ 0x80000000;
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[5]; c.stfs(ea, v); }
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x34), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x30), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x30); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[3] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x800bca38; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c46b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bca38;
            }
            0x800bca38 => { ins += 4;
                c.r[3] = c.r[3].rotate_left(0) & 0xffffff;
                c.r[0] = c.r[28].rotate_left(0) & 0xffffff;
                c.r[10] = c.r[3] | 0xef000000;
                c.r[11] = c.r[0] | 0xf0000000;
                pc = 0x800bca48;
            }
            0x800bca48 => { ins += 51;
                c.r[7] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[0] = 0x61;
                c.r[6] = c.lbz_t(ins, c.r[29].wrapping_add(0x1));
                c.r[4] = 0xcc010000;
                c.r[8] = c.r[7].rotate_left(20) & 0x7f800;
                c.r[8] = (c.r[7].rotate_left(20) & 0x7ff) | (c.r[8] & 0xfffff800);
                c.r[9] = c.lbz_t(ins, c.r[29].wrapping_add(0x2));
                c.r[5] = c.lbz_t(ins, c.r[29]);
                c.r[7] = c.r[7].rotate_left(20) & 0x80000;
                c.r[7] = (c.r[8].rotate_left(0) & 0x7ffff) | (c.r[7] & 0xfff80000);
                c.r[12] = c.lwz_t(ins, c.r[1].wrapping_add(0x28));
                c.r[7] = c.r[7].rotate_left(0) & 0xffffff;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = c.r[7] | 0xee000000;
                c.r[6] = c.r[6].rotate_left(8) & 0xffffff00;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[7]);
                c.r[7] = c.r[12].rotate_left(20) & 0x7f800;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = (c.r[12].rotate_left(20) & 0x7ff) | (c.r[7] & 0xfffff800);
                c.r[8] = c.r[12].rotate_left(20) & 0x80000;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[10]);
                c.r[8] = (c.r[7].rotate_left(0) & 0x7ffff) | (c.r[8] & 0xfff80000);
                c.r[7] = c.r[31].rotate_left(20) & 0xfff00000;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = (c.r[8].rotate_left(0) & 0xfffff) | (c.r[7] & 0xfff00000);
                c.r[8] = c.r[7].rotate_left(0) & 0xff1fffff;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[11]);
                c.r[7] = c.r[30].rotate_left(21) & 0xffe00000;
                c.r[7] = c.r[8] | c.r[7];
                c.r[7] = c.r[7].rotate_left(0) & 0xffffff;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = c.r[7] | 0xf1000000;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[7]);
                c.r[6] = (c.r[9].rotate_left(0) & 0xff) | (c.r[6] & 0xffffff00);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.r[5].rotate_left(16) & 0xffff0000;
                c.r[0] = (c.r[6].rotate_left(0) & 0xffff) | (c.r[0] & 0xffff0000);
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff;
                c.r[0] = c.r[0] | 0xf2000000;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x4c));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x40));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x3c));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x38));
                c.r[1] = c.r[1].wrapping_add(0x48);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bc8f8: bad pc {:#x}", pc),
        }
    }
}

/// GXInitFogAdjTable (0x800bcb14, 456 bytes)
#[inline(never)]
pub fn f_800bcb14(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bcb14) { return; }
        if c.prof { crate::prof::enter(c, 0x800bcb14); p = true; }
    }
    f_800bcb14_b(c, 0x800bcb14);
    if p { crate::prof::exit(c, 0x800bcb14); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bcb14_at(c: &mut Cpu, pc: u32) { f_800bcb14_b(c, pc) }
#[inline(always)]
fn f_800bcb14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bcb14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bcb14, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bcb14 => { ins += 19;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffff80); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x78); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x70); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let v = c.f[27]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[26]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let v = c.f[25]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[24]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[23]; c.stfd(ea, v); }
                c.stw(c.r[1].wrapping_add(0x34), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x30), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x2c), c.r[29]);
                { let ea = c.r[2].wrapping_add(0xffff8d20); c.f[1] = c.lfd(ea); }
                { let ea = c.r[5].wrapping_add(0x3c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                if !cr_bit(cr, 2) { pc = 0x800bcb8c; continue; }
                pc = 0x800bcb60;
            }
            0x800bcb60 => { ins += 11;
                { let ea = c.r[5].wrapping_add(0x28); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[2].wrapping_add(0xffff8cec); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[5].wrapping_add(0x2c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[2] - c.f[1]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[5].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] + c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[5]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] / c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[24] = v; c.ps1[24] = v; }
                pc = 0x800bcbb4; continue;
                pc = 0x800bcb8c;
            }
            0x800bcb8c => { ins += 10;
                { let ea = c.r[2].wrapping_add(0xffff8cec); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[5].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0x2c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[0] - c.f[4]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[5].wrapping_add(0x28); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[4] + c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[5]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[1] = -c.f[1];
                { let v = s(c.f[3] / c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[24] = v; c.ps1[24] = v; }
                pc = 0x800bcbb4;
            }
            0x800bcbb4 => { ins += 17;
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                { let ea = c.r[2].wrapping_add(0xffff8d38); c.f[27] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[31] = 0x43300000;
                { let ea = c.r[2].wrapping_add(0xffff8d00); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[2] * c.f[2]); c.f[26] = v; c.ps1[26] = v; }
                c.stw(c.r[1].wrapping_add(0x20), c.r[31]);
                { let ea = c.r[2].wrapping_add(0xffff8cec); c.f[28] = c.lfs(ea); c.ps1[28] = c.f[28]; }
                c.r[30] = c.r[3] | c.r[3];
                { let ea = c.r[1].wrapping_add(0x20); c.f[0] = c.lfd(ea); }
                { let ea = c.r[2].wrapping_add(0xffff8ce8); c.f[29] = c.lfs(ea); c.ps1[29] = c.f[29]; }
                c.r[29] = 0x0;
                { let v = s(c.f[0] - c.f[27]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8d08); c.f[30] = c.lfd(ea); }
                { let ea = c.r[2].wrapping_add(0xffff8d28); c.f[31] = c.lfd(ea); }
                { let ea = c.r[2].wrapping_add(0xffff8d30); c.f[23] = c.lfs(ea); c.ps1[23] = c.f[23]; }
                { let v = s(c.f[1] / c.f[0]); c.f[25] = v; c.ps1[25] = v; }
                pc = 0x800bcbf8;
            }
            0x800bcbf8 => { ins += 13;
                c.r[0] = c.r[29].wrapping_add(0x1);
                c.r[0] = c.r[0].rotate_left(5) & 0xffffffe0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[31]);
                { let ea = c.r[1].wrapping_add(0x20); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[27]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] * c.f[25]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] * c.f[24]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] / c.f[26]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[28] + c.f[0]); c.f[2] = v; c.ps1[2] = v; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[2], c.f[29]);
                if !cr_bit(cr, 1) { pc = 0x800bcc7c; continue; }
                pc = 0x800bcc2c;
            }
            0x800bcc2c => { ins += 20;
                c.f[1] = frsqrte(c.f[2]);
                c.f[0] = c.f[1] * c.f[1];
                c.f[1] = c.f[30] * c.f[1];
                c.f[0] = c.f[2] * c.f[0];
                c.f[0] = c.f[31] - c.f[0];
                c.f[1] = c.f[1] * c.f[0];
                c.f[0] = c.f[1] * c.f[1];
                c.f[1] = c.f[30] * c.f[1];
                c.f[0] = c.f[2] * c.f[0];
                c.f[0] = c.f[31] - c.f[0];
                c.f[1] = c.f[1] * c.f[0];
                c.f[0] = c.f[1] * c.f[1];
                c.f[1] = c.f[30] * c.f[1];
                c.f[0] = c.f[2] * c.f[0];
                c.f[0] = c.f[31] - c.f[0];
                c.f[0] = c.f[1] * c.f[0];
                c.f[0] = c.f[2] * c.f[0];
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x14); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x14); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800bcc7c;
            }
            0x800bcc7c => { ins += 2;
                { let v = s(c.f[23] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x800bcc84; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c46b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bcc84;
            }
            0x800bcc84 => { ins += 6;
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[0] = c.r[3].rotate_left(0) & 0xfff;
                c.sth(c.r[30], c.r[0]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0xa);
                c.r[30] = c.r[30].wrapping_add(0x2);
                if cr_bit(cr, 0) { pc = 0x800bcbf8; continue; }
                pc = 0x800bcc9c;
            }
            0x800bcc9c => { ins += 16;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x84));
                { let ea = c.r[1].wrapping_add(0x78); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x70); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x68); c.f[29] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x60); c.f[28] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x58); c.f[27] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x50); c.f[26] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x48); c.f[25] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x40); c.f[24] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x38); c.f[23] = c.lfd(ea); }
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x30));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[1] = c.r[1].wrapping_add(0x80);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bcb14: bad pc {:#x}", pc),
        }
    }
}

/// GXSetFogRangeAdj (0x800bccdc, 256 bytes)
#[inline(never)]
pub fn f_800bccdc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bccdc) { return; }
        if c.prof { crate::prof::enter(c, 0x800bccdc); p = true; }
    }
    f_800bccdc_b(c, 0x800bccdc);
    if p { crate::prof::exit(c, 0x800bccdc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bccdc_at(c: &mut Cpu, pc: u32) { f_800bccdc_b(c, pc) }
#[inline(always)]
fn f_800bccdc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bccdc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bccdc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bccdc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800bcda0; continue; }
                pc = 0x800bcce4;
            }
            0x800bcce4 => { ins += 47;
                c.r[8] = c.lhz_t(ins, c.r[5].wrapping_add(0x2));
                c.r[0] = 0x61;
                c.r[10] = c.lhz_t(ins, c.r[5]);
                c.r[6] = 0xcc010000;
                c.r[8] = c.r[8].rotate_left(12) & 0xfffff000;
                c.r[8] = (c.r[10].rotate_left(0) & 0xfff) | (c.r[8] & 0xfffff000);
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = 0xe9;
                c.r[9] = c.r[8].rotate_left(0) & 0xffffff;
                c.r[9] = (c.r[7].rotate_left(24) & 0xff000000) | (c.r[9] & 0xffffff);
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[9]);
                c.r[7] = 0xea;
                c.r[9] = c.r[7].rotate_left(24) & 0xff000000;
                { let ea = c.r[5].wrapping_add(0x4); c.r[10] = c.lhz_t(ins, ea); c.r[5] = ea; }
                c.r[7] = 0xeb;
                c.r[8] = c.lhz_t(ins, c.r[5].wrapping_add(0x2));
                c.r[8] = c.r[8].rotate_left(12) & 0xfffff000;
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[0]);
                c.r[8] = (c.r[10].rotate_left(0) & 0xfff) | (c.r[8] & 0xfffff000);
                c.r[9] = (c.r[8].rotate_left(0) & 0xffffff) | (c.r[9] & 0xff000000);
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[9]);
                c.r[9] = c.r[7].rotate_left(24) & 0xff000000;
                c.r[7] = 0xec;
                c.r[8] = c.lhz_t(ins, c.r[5].wrapping_add(0x6));
                c.r[10] = c.lhz_t(ins, c.r[5].wrapping_add(0x4));
                c.r[8] = c.r[8].rotate_left(12) & 0xfffff000;
                c.r[8] = (c.r[10].rotate_left(0) & 0xfff) | (c.r[8] & 0xfffff000);
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[0]);
                c.r[9] = (c.r[8].rotate_left(0) & 0xffffff) | (c.r[9] & 0xff000000);
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[9]);
                c.r[9] = c.r[7].rotate_left(24) & 0xff000000;
                c.r[7] = 0xed;
                c.r[8] = c.lhz_t(ins, c.r[5].wrapping_add(0xa));
                c.r[10] = c.lhz_t(ins, c.r[5].wrapping_add(0x8));
                c.r[8] = c.r[8].rotate_left(12) & 0xfffff000;
                c.r[8] = (c.r[10].rotate_left(0) & 0xfff) | (c.r[8] & 0xfffff000);
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[0]);
                c.r[9] = (c.r[8].rotate_left(0) & 0xffffff) | (c.r[9] & 0xff000000);
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[9]);
                c.r[8] = c.lhz_t(ins, c.r[5].wrapping_add(0xe));
                c.r[10] = c.lhz_t(ins, c.r[5].wrapping_add(0xc));
                c.r[8] = c.r[8].rotate_left(12) & 0xfffff000;
                c.r[8] = (c.r[10].rotate_left(0) & 0xfff) | (c.r[8] & 0xfffff000);
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[0]);
                c.r[9] = c.r[8].rotate_left(0) & 0xffffff;
                c.r[9] = (c.r[7].rotate_left(24) & 0xff000000) | (c.r[9] & 0xffffff);
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[9]);
                pc = 0x800bcda0;
            }
            0x800bcda0 => { ins += 15;
                c.r[5] = c.r[4].rotate_left(0) & 0xffff;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[5].wrapping_add(0x156);
                c.r[5] = c.r[0].rotate_left(0) & 0xfffffbff;
                c.r[0] = c.r[3].rotate_left(10) & 0x3fc00;
                c.r[0] = c.r[5] | c.r[0];
                c.r[5] = c.r[0].rotate_left(0) & 0xffffff;
                c.r[0] = 0x61;
                c.r[3] = 0xcc010000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[5] = c.r[5] | 0xe8000000;
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bccdc: bad pc {:#x}", pc),
        }
    }
}

/// GXSetBlendMode (0x800bcddc, 84 bytes)
#[inline(never)]
pub fn f_800bcddc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bcddc) { return; }
        if c.prof { crate::prof::enter(c, 0x800bcddc); p = true; }
    }
    f_800bcddc_b(c, 0x800bcddc);
    if p { crate::prof::exit(c, 0x800bcddc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bcddc_at(c: &mut Cpu, pc: u32) { f_800bcddc_b(c, pc) }
#[inline(always)]
fn f_800bcddc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bcddc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bcddc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bcddc => { ins += 21;
                c.r[8] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.addc(!c.r[3], 0x3, 1);
                c.r[7] = c.r[0].leading_zeros();
                c.r[0] = c.addc(!c.r[3], 0x2, 1);
                c.r[9] = c.lwz_t(ins, c.r[8].wrapping_add(0x1d0));
                c.r[9] = (c.r[7].rotate_left(6) & 0x800) | (c.r[9] & 0xfffff7ff);
                c.r[7] = c.r[9].wrapping_add(0x0);
                c.r[7] = (c.r[3].rotate_left(0) & 0x1) | (c.r[7] & 0xfffffffe);
                c.r[0] = c.r[0].leading_zeros();
                c.r[7] = (c.r[0].rotate_left(28) & 0x2) | (c.r[7] & 0xfffffffd);
                c.r[7] = (c.r[6].rotate_left(12) & 0xf000) | (c.r[7] & 0xffff0fff);
                c.r[7] = (c.r[4].rotate_left(8) & 0x700) | (c.r[7] & 0xfffff8ff);
                c.r[0] = 0x61;
                c.r[3] = 0xcc010000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = (c.r[5].rotate_left(5) & 0xe0) | (c.r[7] & 0xffffff1f);
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[7]);
                c.stw(c.r[8].wrapping_add(0x1d0), c.r[7]);
                c.sth(c.r[8].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bcddc: bad pc {:#x}", pc),
        }
    }
}

/// GXSetColorUpdate (0x800bce30, 44 bytes)
#[inline(never)]
pub fn f_800bce30(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bce30) { return; }
        if c.prof { crate::prof::enter(c, 0x800bce30); p = true; }
    }
    f_800bce30_b(c, 0x800bce30);
    if p { crate::prof::exit(c, 0x800bce30); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bce30_at(c: &mut Cpu, pc: u32) { f_800bce30_b(c, pc) }
#[inline(always)]
fn f_800bce30_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bce30;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bce30, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bce30 => { ins += 11;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x61;
                c.r[4] = 0xcc010000;
                c.r[6] = c.lwz_t(ins, c.r[5].wrapping_add(0x1d0));
                c.r[6] = (c.r[3].rotate_left(3) & 0x8) | (c.r[6] & 0xfffffff7);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                c.stw(c.r[5].wrapping_add(0x1d0), c.r[6]);
                c.sth(c.r[5].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bce30: bad pc {:#x}", pc),
        }
    }
}

/// GXSetAlphaUpdate (0x800bce5c, 44 bytes)
#[inline(never)]
pub fn f_800bce5c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bce5c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bce5c); p = true; }
    }
    f_800bce5c_b(c, 0x800bce5c);
    if p { crate::prof::exit(c, 0x800bce5c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bce5c_at(c: &mut Cpu, pc: u32) { f_800bce5c_b(c, pc) }
#[inline(always)]
fn f_800bce5c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bce5c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bce5c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bce5c => { ins += 11;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x61;
                c.r[4] = 0xcc010000;
                c.r[6] = c.lwz_t(ins, c.r[5].wrapping_add(0x1d0));
                c.r[6] = (c.r[3].rotate_left(4) & 0x10) | (c.r[6] & 0xffffffef);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                c.stw(c.r[5].wrapping_add(0x1d0), c.r[6]);
                c.sth(c.r[5].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bce5c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetZMode (0x800bce88, 52 bytes)
#[inline(never)]
pub fn f_800bce88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bce88) { return; }
        if c.prof { crate::prof::enter(c, 0x800bce88); p = true; }
    }
    f_800bce88_b(c, 0x800bce88);
    if p { crate::prof::exit(c, 0x800bce88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bce88_at(c: &mut Cpu, pc: u32) { f_800bce88_b(c, pc) }
#[inline(always)]
fn f_800bce88_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bce88;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bce88, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bce88 => { ins += 13;
                c.r[6] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x61;
                c.r[7] = c.lwz_t(ins, c.r[6].wrapping_add(0x1d8));
                c.r[7] = (c.r[3].rotate_left(0) & 0x1) | (c.r[7] & 0xfffffffe);
                c.r[3] = 0xcc010000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = (c.r[4].rotate_left(1) & 0xe) | (c.r[7] & 0xfffffff1);
                c.r[7] = (c.r[5].rotate_left(4) & 0x10) | (c.r[7] & 0xffffffef);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[7]);
                c.r[0] = 0x0;
                c.stw(c.r[6].wrapping_add(0x1d8), c.r[7]);
                c.sth(c.r[6].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bce88: bad pc {:#x}", pc),
        }
    }
}

/// GXSetZCompLoc (0x800bcebc, 56 bytes)
#[inline(never)]
pub fn f_800bcebc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bcebc) { return; }
        if c.prof { crate::prof::enter(c, 0x800bcebc); p = true; }
    }
    f_800bcebc_b(c, 0x800bcebc);
    if p { crate::prof::exit(c, 0x800bcebc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bcebc_at(c: &mut Cpu, pc: u32) { f_800bcebc_b(c, pc) }
#[inline(always)]
fn f_800bcebc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bcebc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bcebc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bcebc => { ins += 14;
                c.r[6] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[3].rotate_left(6) & 0x3fc0;
                c.r[3] = 0x61;
                c.r[5] = c.lwz_t(ins, c.r[6].wrapping_add(0x1dc));
                c.r[4] = 0xcc010000;
                c.r[5] = c.r[5].rotate_left(0) & 0xffffffbf;
                c.r[0] = c.r[5] | c.r[0];
                c.stw(c.r[6].wrapping_add(0x1dc), c.r[0]);
                c.r[0] = 0x0;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[6].wrapping_add(0x1dc));
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[6].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bcebc: bad pc {:#x}", pc),
        }
    }
}

/// GXSetPixelFmt (0x800bcef4, 232 bytes)
#[inline(never)]
pub fn f_800bcef4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bcef4) { return; }
        if c.prof { crate::prof::enter(c, 0x800bcef4); p = true; }
    }
    f_800bcef4_b(c, 0x800bcef4);
    if p { crate::prof::exit(c, 0x800bcef4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bcef4_at(c: &mut Cpu, pc: u32) { f_800bcef4_b(c, pc) }
#[inline(always)]
fn f_800bcef4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bcef4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bcef4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bcef4 => { ins += 18;
                c.r[7] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0x80310000;
                c.r[6] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[5].wrapping_add(0x3608);
                c.r[8] = c.lwz_t(ins, c.r[7].wrapping_add(0x1dc));
                c.r[10] = c.r[0].wrapping_add(c.r[6]);
                c.r[5] = c.lwz_t(ins, c.r[10]);
                c.r[6] = c.r[8].rotate_left(0) & 0xfffffff8;
                c.r[0] = c.r[4].rotate_left(3) & 0xfffffff8;
                c.r[4] = c.r[6] | c.r[5];
                c.stw(c.r[7].wrapping_add(0x1dc), c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[7].wrapping_add(0x1dc));
                c.r[4] = c.r[4].rotate_left(0) & 0xffffffc7;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[7].wrapping_add(0x1dc), c.r[0]);
                c.r[5] = c.lwz_t(ins, c.r[7].wrapping_add(0x1dc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], c.r[5]);
                if cr_bit(cr, 2) { pc = 0x800bcf84; continue; }
                pc = 0x800bcf3c;
            }
            0x800bcf3c => { ins += 6;
                c.r[0] = 0x61;
                c.r[4] = 0xcc010000;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x2);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                if !cr_bit(cr, 2) { pc = 0x800bcf5c; continue; }
                pc = 0x800bcf54;
            }
            0x800bcf54 => { ins += 2;
                c.r[0] = 0x1;
                pc = 0x800bcf60; continue;
                pc = 0x800bcf5c;
            }
            0x800bcf5c => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800bcf60;
            }
            0x800bcf60 => { ins += 9;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[0].rotate_left(9) & 0x1fe00;
                c.r[4] = c.lwz_t(ins, c.r[5].wrapping_add(0x204));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffdff;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[5].wrapping_add(0x204), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0x4f4));
                c.r[0] = c.r[0] | 0x4;
                c.stw(c.r[5].wrapping_add(0x4f4), c.r[0]);
                pc = 0x800bcf84;
            }
            0x800bcf84 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[10]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 2) { pc = 0x800bcfcc; continue; }
                pc = 0x800bcf90;
            }
            0x800bcf90 => { ins += 15;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[3] = c.r[3].wrapping_add(0xfffffffc);
                c.r[0] = 0x61;
                c.r[4] = c.lwz_t(ins, c.r[5].wrapping_add(0x1d4));
                c.r[4] = c.r[4].rotate_left(0) & 0xfffff9ff;
                c.r[4] = (c.r[3].rotate_left(9) & 0x600) | (c.r[4] & 0xfffff9ff);
                c.stw(c.r[5].wrapping_add(0x1d4), c.r[4]);
                c.r[3] = 0xcc010000;
                c.r[4] = c.lwz_t(ins, c.r[5].wrapping_add(0x1d4));
                c.r[4] = c.r[4].rotate_left(0) & 0xffffff;
                c.r[4] = c.r[4] | 0x42000000;
                c.stw(c.r[5].wrapping_add(0x1d4), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0x1d4));
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bcfcc;
            }
            0x800bcfcc => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x0;
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bcef4: bad pc {:#x}", pc),
        }
    }
}

/// GXSetDither (0x800bcfdc, 44 bytes)
#[inline(never)]
pub fn f_800bcfdc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bcfdc) { return; }
        if c.prof { crate::prof::enter(c, 0x800bcfdc); p = true; }
    }
    f_800bcfdc_b(c, 0x800bcfdc);
    if p { crate::prof::exit(c, 0x800bcfdc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bcfdc_at(c: &mut Cpu, pc: u32) { f_800bcfdc_b(c, pc) }
#[inline(always)]
fn f_800bcfdc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bcfdc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bcfdc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bcfdc => { ins += 11;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x61;
                c.r[4] = 0xcc010000;
                c.r[6] = c.lwz_t(ins, c.r[5].wrapping_add(0x1d0));
                c.r[6] = (c.r[3].rotate_left(2) & 0x4) | (c.r[6] & 0xfffffffb);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[6]);
                c.stw(c.r[5].wrapping_add(0x1d0), c.r[6]);
                c.sth(c.r[5].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bcfdc: bad pc {:#x}", pc),
        }
    }
}

/// GXSetDstAlpha (0x800bd008, 60 bytes)
#[inline(never)]
pub fn f_800bd008(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd008) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd008); p = true; }
    }
    f_800bd008_b(c, 0x800bd008);
    if p { crate::prof::exit(c, 0x800bd008); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd008_at(c: &mut Cpu, pc: u32) { f_800bd008_b(c, pc) }
#[inline(always)]
fn f_800bd008_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd008;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd008, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd008 => { ins += 15;
                c.r[6] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = c.r[4].rotate_left(0) & 0xff;
                c.r[0] = 0x61;
                c.r[7] = c.lwz_t(ins, c.r[6].wrapping_add(0x1d4));
                c.r[4] = 0xcc010000;
                c.r[7] = (c.r[5].rotate_left(0) & 0xff) | (c.r[7] & 0xffffff00);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[7].wrapping_add(0x0);
                c.r[3] = (c.r[0].rotate_left(8) & 0x100) | (c.r[3] & 0xfffffeff);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[3]);
                c.r[0] = 0x0;
                c.stw(c.r[6].wrapping_add(0x1d4), c.r[3]);
                c.sth(c.r[6].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd008: bad pc {:#x}", pc),
        }
    }
}

/// GXSetFieldMask (0x800bd044, 56 bytes)
#[inline(never)]
pub fn f_800bd044(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd044) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd044); p = true; }
    }
    f_800bd044_b(c, 0x800bd044);
    if p { crate::prof::exit(c, 0x800bd044); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd044_at(c: &mut Cpu, pc: u32) { f_800bd044_b(c, pc) }
#[inline(always)]
fn f_800bd044_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd044;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd044, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd044 => { ins += 14;
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = c.r[0].rotate_left(0) & 0xfffffffd;
                c.r[0] = c.r[3].rotate_left(1) & 0x1fe;
                c.r[0] = c.r[5] | c.r[0];
                c.r[5] = c.r[0].rotate_left(0) & 0xffffff;
                c.r[0] = 0x61;
                c.r[3] = 0xcc010000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.r[5] = c.r[5] | 0x44000000;
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd044: bad pc {:#x}", pc),
        }
    }
}

/// GXSetFieldMode (0x800bd07c, 124 bytes)
#[inline(never)]
pub fn f_800bd07c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd07c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd07c); p = true; }
    }
    f_800bd07c_b(c, 0x800bd07c);
    if p { crate::prof::exit(c, 0x800bd07c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd07c_at(c: &mut Cpu, pc: u32) { f_800bd07c_b(c, pc) }
#[inline(always)]
fn f_800bd07c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd07c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd07c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd07c => { ins += 19;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[4].rotate_left(22) & 0x3fc00000;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = 0xcc010000;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = 0x61;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[4] = c.lwz_t(ins, c.r[5].wrapping_add(0x7c));
                c.r[4] = c.r[4].rotate_left(0) & 0xffbfffff;
                c.r[0] = c.r[4] | c.r[0];
                c.stw(c.r[5].wrapping_add(0x7c), c.r[0]);
                c.stb(c.r[31].wrapping_add(0xffff8000), c.r[30]);
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0x7c));
                c.stw(c.r[31].wrapping_add(0xffff8000), c.r[0]);
                c.lr = 0x800bd0c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bc0f0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bd0c8;
            }
            0x800bd0c8 => { ins += 5;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                c.stb(c.r[31].wrapping_add(0xffff8000), c.r[30]);
                c.r[0] = c.r[0] | 0x68000000;
                c.stw(c.r[31].wrapping_add(0xffff8000), c.r[0]);
                c.lr = 0x800bd0dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bc0f0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bd0dc;
            }
            0x800bd0dc => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd07c: bad pc {:#x}", pc),
        }
    }
}

/// __GXSetRange (0x800bd0f8, 4 bytes)
#[inline(never)]
pub fn f_800bd0f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd0f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd0f8); p = true; }
    }
    f_800bd0f8_b(c, 0x800bd0f8);
    if p { crate::prof::exit(c, 0x800bd0f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd0f8_at(c: &mut Cpu, pc: u32) { f_800bd0f8_b(c, pc) }
#[inline(always)]
fn f_800bd0f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd0f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd0f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd0f8 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd0f8: bad pc {:#x}", pc),
        }
    }
}

/// GXCallDisplayList (0x800bd0fc, 112 bytes)
#[inline(never)]
pub fn f_800bd0fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd0fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd0fc); p = true; }
    }
    f_800bd0fc_b(c, 0x800bd0fc);
    if p { crate::prof::exit(c, 0x800bd0fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd0fc_at(c: &mut Cpu, pc: u32) { f_800bd0fc_b(c, pc) }
#[inline(always)]
fn f_800bd0fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd0fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd0fc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd0fc => { ins += 11;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0x4f4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bd12c; continue; }
                pc = 0x800bd128;
            }
            0x800bd128 => { ins += 1;
                c.lr = 0x800bd12c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800b91ec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bd12c;
            }
            0x800bd12c => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bd140; continue; }
                pc = 0x800bd13c;
            }
            0x800bd13c => { ins += 1;
                c.lr = 0x800bd140; { c.steps = st; c.instrs = ins; c.cr = cr; f_800b937c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bd140;
            }
            0x800bd140 => { ins += 11;
                c.r[0] = 0x40;
                c.r[3] = 0xcc010000;
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[30]);
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[31]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd0fc: bad pc {:#x}", pc),
        }
    }
}

/// GXProject (0x800bd16c, 372 bytes)
#[inline(never)]
pub fn f_800bd16c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd16c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd16c); p = true; }
    }
    f_800bd16c_b(c, 0x800bd16c);
    if p { crate::prof::exit(c, 0x800bd16c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd16c_at(c: &mut Cpu, pc: u32) { f_800bd16c_b(c, pc) }
#[inline(always)]
fn f_800bd16c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd16c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd16c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd16c => { ins += 34;
                { let ea = c.r[3]; c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[3].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x10); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                { let v = s(c.f[4] * c.f[1]); c.f[10] = v; c.ps1[10] = v; }
                { let ea = c.r[3].wrapping_add(0x14); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let v = s(c.f[0] * c.f[2]); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[3].wrapping_add(0x20); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[3].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[11] = c.lfs(ea); c.ps1[11] = c.f[11]; }
                { let v = s(c.f[6] * c.f[1]); c.f[7] = v; c.ps1[7] = v; }
                { let v = s(c.f[5] * c.f[2]); c.f[6] = v; c.ps1[6] = v; }
                { let ea = c.r[3].wrapping_add(0x18); c.f[8] = c.lfs(ea); c.ps1[8] = c.f[8]; }
                { let ea = c.r[3].wrapping_add(0x28); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let v = s(c.f[4] * c.f[1]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[0] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[11] * c.f[3]); c.f[11] = v; c.ps1[11] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8d40); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[10] + c.f[9]); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[4]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[8] * c.f[3]); c.f[8] = v; c.ps1[8] = v; }
                { let v = s(c.f[7] + c.f[6]); c.f[6] = v; c.ps1[6] = v; }
                { let ea = c.r[3].wrapping_add(0xc); c.f[10] = c.lfs(ea); c.ps1[10] = c.f[10]; }
                { let v = s(c.f[11] + c.f[9]); c.f[9] = v; c.ps1[9] = v; }
                { let ea = c.r[3].wrapping_add(0x1c); c.f[7] = c.lfs(ea); c.ps1[7] = c.f[7]; }
                { let v = s(c.f[8] + c.f[6]); c.f[6] = v; c.ps1[6] = v; }
                { let v = s(c.f[5] * c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[3].wrapping_add(0x2c); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let v = s(c.f[4] + c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                cr_fcmp(&mut cr, &mut c.fpscr, 0, c.f[1], c.f[0]);
                { let v = s(c.f[10] + c.f[9]); c.f[8] = v; c.ps1[8] = v; }
                { let v = s(c.f[3] + c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[7] + c.f[6]); c.f[7] = v; c.ps1[7] = v; }
                { let v = s(c.f[5] + c.f[0]); c.f[9] = v; c.ps1[9] = v; }
                if !cr_bit(cr, 2) { pc = 0x800bd23c; continue; }
                pc = 0x800bd1f4;
            }
            0x800bd1f4 => { ins += 18;
                c.f[0] = -c.f[9];
                { let ea = c.r[2].wrapping_add(0xffff8d44); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x4); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[4].wrapping_add(0xc); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[4].wrapping_add(0x10); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[4].wrapping_add(0x14); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x18); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[9] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[8] * c.f[6]); c.f[6] = v; c.ps1[6] = v; }
                { let v = s(c.f[9] * c.f[5]); c.f[5] = v; c.ps1[5] = v; }
                { let v = s(c.f[7] * c.f[4]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[9] * c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[6] + c.f[5]); c.f[6] = v; c.ps1[6] = v; }
                { let v = s(c.f[2] + c.f[1]); c.f[7] = v; c.ps1[7] = v; }
                { let v = s(c.f[4] + c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                pc = 0x800bd270; continue;
                pc = 0x800bd23c;
            }
            0x800bd23c => { ins += 13;
                { let ea = c.r[4].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[8] * c.f[2]); c.f[5] = v; c.ps1[5] = v; }
                { let ea = c.r[4].wrapping_add(0x8); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                { let v = s(c.f[7] * c.f[1]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[4].wrapping_add(0x10); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let v = s(c.f[9] * c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[4].wrapping_add(0x18); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[6] + c.f[5]); c.f[6] = v; c.ps1[6] = v; }
                { let v = s(c.f[4] + c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[2].wrapping_add(0xffff8d44); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] + c.f[1]); c.f[7] = v; c.ps1[7] = v; }
                pc = 0x800bd270;
            }
            0x800bd270 => { ins += 28;
                { let ea = c.r[5].wrapping_add(0x8); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.f[1] = -c.f[3];
                { let ea = c.r[2].wrapping_add(0xffff8d48); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let v = s(c.f[6] * c.f[4]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[5]; c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[4] * c.f[5]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[2] * c.f[5]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[0] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[3] + c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[4] + c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[6]; let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0xc); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[5].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[1] * c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[3] * c.f[5]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[1] * c.f[5]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[2] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[3] + c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[7]; let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0x14); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[5].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[2] - c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[7] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] * c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[8]; let v = c.f[0]; c.stfs(ea, v); }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd16c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetProjection (0x800bd2e0, 180 bytes)
#[inline(never)]
pub fn f_800bd2e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd2e0) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd2e0); p = true; }
    }
    f_800bd2e0_b(c, 0x800bd2e0);
    if p { crate::prof::exit(c, 0x800bd2e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd2e0_at(c: &mut Cpu, pc: u32) { f_800bd2e0_b(c, pc) }
#[inline(always)]
fn f_800bd2e0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd2e0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd2e0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd2e0 => { ins += 12;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x1);
                c.stw(c.r[5].wrapping_add(0x420), c.r[4]);
                { let ea = c.r[3]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0x424); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0x42c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0x434); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0x438); let v = c.f[0]; c.stfs(ea, v); }
                if !cr_bit(cr, 2) { pc = 0x800bd324; continue; }
                pc = 0x800bd310;
            }
            0x800bd310 => { ins += 5;
                { let ea = c.r[3].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0x428); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x1c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0x430); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800bd334; continue;
                pc = 0x800bd324;
            }
            0x800bd324 => { ins += 4;
                { let ea = c.r[3].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0x428); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0x430); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800bd334;
            }
            0x800bd334 => { ins += 24;
                c.r[0] = 0x10;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.r[3] = 0x60000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x1020);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x1;
                { let ea = c.r[4].wrapping_add(0x424); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x428); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x42c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x430); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x434); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x438); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x420));
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd2e0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800BD394 (0x800bd394, 192 bytes)
#[inline(never)]
pub fn f_800bd394(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd394) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd394); p = true; }
    }
    f_800bd394_b(c, 0x800bd394);
    if p { crate::prof::exit(c, 0x800bd394); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd394_at(c: &mut Cpu, pc: u32) { f_800bd394_b(c, pc) }
#[inline(always)]
fn f_800bd394_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd394;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd394, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd394 => { ins += 7;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                { let ea = c.r[3]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x800bd3b0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c46b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bd3b0;
            }
            0x800bd3b0 => { ins += 41;
                c.r[7] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[4] = 0x60000;
                c.r[6] = 0x10;
                c.stw(c.r[7].wrapping_add(0x420), c.r[3]);
                c.r[5] = 0xcc010000;
                c.r[3] = c.r[4].wrapping_add(0x1020);
                { let ea = c.r[31].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[0] = 0x1;
                { let ea = c.r[7].wrapping_add(0x424); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[7].wrapping_add(0x428); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[7].wrapping_add(0x42c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[7].wrapping_add(0x430); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[7].wrapping_add(0x434); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[7].wrapping_add(0x438); let v = c.f[0]; c.stfs(ea, v); }
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[6]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                { let ea = c.r[7].wrapping_add(0x424); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[7].wrapping_add(0x428); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[7].wrapping_add(0x42c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[7].wrapping_add(0x430); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[7].wrapping_add(0x434); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[7].wrapping_add(0x438); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = c.lwz_t(ins, c.r[7].wrapping_add(0x420));
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[7].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd394: bad pc {:#x}", pc),
        }
    }
}

/// GXGetProjectionv (0x800bd454, 96 bytes)
#[inline(never)]
pub fn f_800bd454(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd454) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd454); p = true; }
    }
    f_800bd454_b(c, 0x800bd454);
    if p { crate::prof::exit(c, 0x800bd454); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd454_at(c: &mut Cpu, pc: u32) { f_800bd454_b(c, pc) }
#[inline(always)]
fn f_800bd454_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd454;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd454, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd454 => { ins += 24;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = 0x43300000;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                { let ea = c.r[2].wrapping_add(0xffff8d50); c.f[1] = c.lfd(ea); }
                c.r[4] = c.lwz_t(ins, c.r[5].wrapping_add(0x420));
                c.stw(c.r[1].wrapping_add(0x14), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3]; let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0x424); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0x428); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0x42c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0xc); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0x430); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x10); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0x434); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x14); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0x438); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x18); let v = c.f[0]; c.stfs(ea, v); }
                c.r[1] = c.r[1].wrapping_add(0x18);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd454: bad pc {:#x}", pc),
        }
    }
}

/// GXLoadPosMtxImm (0x800bd4b4, 80 bytes)
#[inline(never)]
pub fn f_800bd4b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd4b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd4b4); p = true; }
    }
    f_800bd4b4_b(c, 0x800bd4b4);
    if p { crate::prof::exit(c, 0x800bd4b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd4b4_at(c: &mut Cpu, pc: u32) { f_800bd4b4_b(c, pc) }
#[inline(always)]
fn f_800bd4b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd4b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd4b4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd4b4 => { ins += 20;
                c.r[5] = 0xcc010000;
                c.r[0] = 0x10;
                c.r[4] = c.r[4].rotate_left(2) & 0xfffffffc;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.r[4] | 0xb0000;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[4] = c.r[5].wrapping_add(0xffff8000);
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd4b4: bad pc {:#x}", pc),
        }
    }
}

/// GXLoadNrmMtxImm (0x800bd504, 80 bytes)
#[inline(never)]
pub fn f_800bd504(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd504) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd504); p = true; }
    }
    f_800bd504_b(c, 0x800bd504);
    if p { crate::prof::exit(c, 0x800bd504); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd504_at(c: &mut Cpu, pc: u32) { f_800bd504_b(c, pc) }
#[inline(always)]
fn f_800bd504_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd504;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd504, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd504 => { ins += 20;
                c.r[5] = c.r[4].wrapping_mul(0x3);
                c.r[4] = 0xcc010000;
                c.r[0] = 0x10;
                c.r[5] = c.r[5].wrapping_add(0x400);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.r[5] | 0x80000;
                { let ea = c.r[4].wrapping_add(0xffff8000); c.stw(ea, c.r[0]); c.r[4] = ea; }
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[3].wrapping_add(0x8); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[3].wrapping_add(0x18); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4]; let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4]; let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4]; let v = c.f[0]; c.stfs(ea, v); }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd504: bad pc {:#x}", pc),
        }
    }
}

/// GXSetCurrentMtx (0x800bd554, 56 bytes)
#[inline(never)]
pub fn f_800bd554(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd554) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd554); p = true; }
    }
    f_800bd554_b(c, 0x800bd554);
    if p { crate::prof::exit(c, 0x800bd554); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd554_at(c: &mut Cpu, pc: u32) { f_800bd554_b(c, pc) }
#[inline(always)]
fn f_800bd554_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd554;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd554, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd554 => { ins += 10;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                { let ea = c.r[4].wrapping_add(0x80); c.r[0] = c.lwz_t(ins, ea); c.r[4] = ea; }
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffc0;
                c.r[0] = c.r[0] | c.r[3];
                c.stw(c.r[4], c.r[0]);
                c.r[3] = 0x0;
                c.lr = 0x800bd57c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bd898(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bd57c;
            }
            0x800bd57c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd554: bad pc {:#x}", pc),
        }
    }
}

/// GXLoadTexMtxImm (0x800bd58c, 180 bytes)
#[inline(never)]
pub fn f_800bd58c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd58c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd58c); p = true; }
    }
    f_800bd58c_b(c, 0x800bd58c);
    if p { crate::prof::exit(c, 0x800bd58c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd58c_at(c: &mut Cpu, pc: u32) { f_800bd58c_b(c, pc) }
#[inline(always)]
fn f_800bd58c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd58c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd58c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd58c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x40);
                if cr_bit(cr, 0) { pc = 0x800bd5a4; continue; }
                pc = 0x800bd594;
            }
            0x800bd594 => { ins += 4;
                c.r[0] = c.r[4].wrapping_add(0xffffffc0);
                c.r[4] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[7] = c.r[4].wrapping_add(0x500);
                pc = 0x800bd5a8; continue;
                pc = 0x800bd5a4;
            }
            0x800bd5a4 => { ins += 1;
                c.r[7] = c.r[4].rotate_left(2) & 0xfffffffc;
                pc = 0x800bd5a8;
            }
            0x800bd5a8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x1);
                if !cr_bit(cr, 2) { pc = 0x800bd5b8; continue; }
                pc = 0x800bd5b0;
            }
            0x800bd5b0 => { ins += 2;
                c.r[4] = 0x8;
                pc = 0x800bd5bc; continue;
                pc = 0x800bd5b8;
            }
            0x800bd5b8 => { ins += 1;
                c.r[4] = 0xc;
                pc = 0x800bd5bc;
            }
            0x800bd5bc => { ins += 9;
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                c.r[6] = c.r[0].rotate_left(16) & 0xffff0000;
                c.r[0] = 0x10;
                c.r[4] = 0xcc010000;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.r[7] | c.r[6];
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x0);
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800bd618; continue; }
                pc = 0x800bd5e0;
            }
            0x800bd5e0 => { ins += 14;
                c.r[4] = c.r[4].wrapping_add(0xffff8000);
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[5] = x; c.ps1[5] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[4] = x; c.ps1[4] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[3].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[5], c.ps1[5]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[4], c.ps1[4]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800bd618;
            }
            0x800bd618 => { ins += 10;
                c.r[4] = c.r[4].wrapping_add(0xffff8000);
                { let ea = c.r[3].wrapping_add(0x0); let (x, y) = c.psq_load(ea, false, 0); c.f[3] = x; c.ps1[3] = y; }
                { let ea = c.r[3].wrapping_add(0x8); let (x, y) = c.psq_load(ea, false, 0); c.f[2] = x; c.ps1[2] = y; }
                { let ea = c.r[3].wrapping_add(0x10); let (x, y) = c.psq_load(ea, false, 0); c.f[1] = x; c.ps1[1] = y; }
                { let ea = c.r[3].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[0] = x; c.ps1[0] = y; }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[3], c.ps1[3]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[2], c.ps1[2]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[1], c.ps1[1]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[4].wrapping_add(0x0); let (x, y) = (c.f[0], c.ps1[0]); c.psq_store(ea, x, y, false, 0); }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd58c: bad pc {:#x}", pc),
        }
    }
}

/// GXSetViewportJitter (0x800bd640, 260 bytes)
#[inline(never)]
pub fn f_800bd640(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd640) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd640); p = true; }
    }
    f_800bd640_b(c, 0x800bd640);
    if p { crate::prof::exit(c, 0x800bd640); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd640_at(c: &mut Cpu, pc: u32) { f_800bd640_b(c, pc) }
#[inline(always)]
fn f_800bd640_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd640;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd640, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd640 => { ins += 11;
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x58); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[27]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[26]; c.stfd(ea, v); }
                if !cr_bit(cr, 2) { pc = 0x800bd674; continue; }
                pc = 0x800bd66c;
            }
            0x800bd66c => { ins += 2;
                { let ea = c.r[2].wrapping_add(0xffff8d48); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] - c.f[0]); c.f[2] = v; c.ps1[2] = v; }
                pc = 0x800bd674;
            }
            0x800bd674 => { ins += 24;
                { let ea = c.r[2].wrapping_add(0xffff8d48); c.f[11] = c.lfs(ea); c.ps1[11] = c.f[11]; }
                c.f[10] = -c.f[4];
                { let ea = c.r[2].wrapping_add(0xffff8d5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[2].wrapping_add(0xffff8d58); c.f[9] = c.lfs(ea); c.ps1[9] = c.f[9]; }
                { let v = s(c.f[3] * c.f[11]); c.f[31] = v; c.ps1[31] = v; }
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                { let v = s(c.f[4] * c.f[11]); c.f[7] = v; c.ps1[7] = v; }
                { let ea = c.r[3].wrapping_add(0x43c); let v = c.f[1]; c.stfs(ea, v); }
                { let v = s(c.f[1] + c.f[31]); c.f[8] = v; c.ps1[8] = v; }
                { let v = s(c.f[0] * c.f[6]); c.f[30] = v; c.ps1[30] = v; }
                { let ea = c.r[3].wrapping_add(0x440); let v = c.f[2]; c.stfs(ea, v); }
                { let v = s(c.f[2] + c.f[7]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[3].wrapping_add(0x444); let v = c.f[3]; c.stfs(ea, v); }
                { let v = s(c.f[0] * c.f[5]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[10] * c.f[11]); c.f[29] = v; c.ps1[29] = v; }
                { let ea = c.r[3].wrapping_add(0x448); let v = c.f[4]; c.stfs(ea, v); }
                { let v = s(c.f[9] + c.f[8]); c.f[27] = v; c.ps1[27] = v; }
                { let ea = c.r[3].wrapping_add(0x44c); let v = c.f[5]; c.stfs(ea, v); }
                { let v = s(c.f[9] + c.f[1]); c.f[26] = v; c.ps1[26] = v; }
                { let v = s(c.f[30] - c.f[0]); c.f[28] = v; c.ps1[28] = v; }
                { let ea = c.r[3].wrapping_add(0x450); let v = c.f[6]; c.stfs(ea, v); }
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x454));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bd6e0; continue; }
                pc = 0x800bd6d4;
            }
            0x800bd6d4 => { ins += 3;
                c.f[1] = c.f[5];
                { let ea = c.r[3].wrapping_add(0x458); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.lr = 0x800bd6e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bd0f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bd6e0;
            }
            0x800bd6e0 => { ins += 25;
                c.r[0] = 0x10;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.r[4] = 0x50000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.r[4].wrapping_add(0x101a);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x1;
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[31]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[29]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[28]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[27]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[26]; c.stfs(ea, v); }
                { let ea = c.r[5].wrapping_add(0xffff8000); let v = c.f[30]; c.stfs(ea, v); }
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x64));
                { let ea = c.r[1].wrapping_add(0x58); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x50); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x48); c.f[29] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x40); c.f[28] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x38); c.f[27] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x30); c.f[26] = c.lfd(ea); }
                c.r[1] = c.r[1].wrapping_add(0x60);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd640: bad pc {:#x}", pc),
        }
    }
}

/// GXSetViewport (0x800bd744, 36 bytes)
#[inline(never)]
pub fn f_800bd744(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd744) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd744); p = true; }
    }
    f_800bd744_b(c, 0x800bd744);
    if p { crate::prof::exit(c, 0x800bd744); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd744_at(c: &mut Cpu, pc: u32) { f_800bd744_b(c, pc) }
#[inline(always)]
fn f_800bd744_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd744;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd744, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd744 => { ins += 5;
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x1;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800bd758; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bd640(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bd758;
            }
            0x800bd758 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd744: bad pc {:#x}", pc),
        }
    }
}

/// GXGetViewportv (0x800bd768, 56 bytes)
#[inline(never)]
pub fn f_800bd768(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd768) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd768); p = true; }
    }
    f_800bd768_b(c, 0x800bd768);
    if p { crate::prof::exit(c, 0x800bd768); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd768_at(c: &mut Cpu, pc: u32) { f_800bd768_b(c, pc) }
#[inline(always)]
fn f_800bd768_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd768;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd768, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd768 => { ins += 14;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                { let ea = c.r[4].wrapping_add(0x43c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3]; let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x440); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x444); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x448); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0xc); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x44c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x10); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x450); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x14); let v = c.f[0]; c.stfs(ea, v); }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd768: bad pc {:#x}", pc),
        }
    }
}

/// GXSetScissor (0x800bd7a0, 144 bytes)
#[inline(never)]
pub fn f_800bd7a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd7a0) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd7a0); p = true; }
    }
    f_800bd7a0_b(c, 0x800bd7a0);
    if p { crate::prof::exit(c, 0x800bd7a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd7a0_at(c: &mut Cpu, pc: u32) { f_800bd7a0_b(c, pc) }
#[inline(always)]
fn f_800bd7a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd7a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd7a0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd7a0 => { ins += 36;
                c.r[8] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[9] = c.r[3].wrapping_add(0x156);
                c.r[10] = c.r[4].wrapping_add(0x156);
                c.r[3] = c.lwz_t(ins, c.r[8].wrapping_add(0xf8));
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                c.r[4] = c.r[9].rotate_left(12) & 0xfffff000;
                c.r[3] = c.r[3].rotate_left(0) & 0xfffff800;
                c.r[3] = c.r[3] | c.r[10];
                c.stw(c.r[8].wrapping_add(0xf8), c.r[3]);
                c.r[3] = c.r[6].wrapping_add(0xffffffff);
                c.r[6] = c.r[10].wrapping_add(c.r[3]);
                c.r[5] = c.lwz_t(ins, c.r[8].wrapping_add(0xf8));
                c.r[0] = c.r[9].wrapping_add(c.r[0]);
                c.r[3] = c.r[5].rotate_left(0) & 0xff800fff;
                c.r[3] = c.r[3] | c.r[4];
                c.stw(c.r[8].wrapping_add(0xf8), c.r[3]);
                c.r[3] = c.r[0].rotate_left(12) & 0xfffff000;
                c.r[5] = 0x61;
                c.r[7] = c.lwz_t(ins, c.r[8].wrapping_add(0xfc));
                c.r[4] = 0xcc010000;
                c.r[0] = 0x0;
                c.r[7] = c.r[7].rotate_left(0) & 0xfffff800;
                c.r[6] = c.r[7] | c.r[6];
                c.stw(c.r[8].wrapping_add(0xfc), c.r[6]);
                c.r[6] = c.lwz_t(ins, c.r[8].wrapping_add(0xfc));
                c.r[6] = c.r[6].rotate_left(0) & 0xff800fff;
                c.r[3] = c.r[6] | c.r[3];
                c.stw(c.r[8].wrapping_add(0xfc), c.r[3]);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[3] = c.lwz_t(ins, c.r[8].wrapping_add(0xf8));
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[3]);
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.r[3] = c.lwz_t(ins, c.r[8].wrapping_add(0xfc));
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[8].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd7a0: bad pc {:#x}", pc),
        }
    }
}

/// GXSetScissorBoxOffset (0x800bd830, 64 bytes)
#[inline(never)]
pub fn f_800bd830(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd830) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd830); p = true; }
    }
    f_800bd830_b(c, 0x800bd830);
    if p { crate::prof::exit(c, 0x800bd830); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd830_at(c: &mut Cpu, pc: u32) { f_800bd830_b(c, pc) }
#[inline(always)]
fn f_800bd830_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd830;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd830, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd830 => { ins += 16;
                c.r[5] = c.r[3].wrapping_add(0x156);
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = c.r[4].wrapping_add(0x156);
                c.r[4] = c.r[5].rotate_left(31) & 0x7fffffff;
                c.r[4] = c.r[4].rotate_left(0) & 0xfff003ff;
                c.r[0] = c.r[0].rotate_left(9) & 0xfffffc00;
                c.r[0] = c.r[4] | c.r[0];
                c.r[5] = c.r[0].rotate_left(0) & 0xffffff;
                c.r[0] = 0x61;
                c.r[4] = 0xcc010000;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                c.r[5] = c.r[5] | 0x59000000;
                c.r[0] = 0x0;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd830: bad pc {:#x}", pc),
        }
    }
}

/// GXSetClipMode (0x800bd870, 40 bytes)
#[inline(never)]
pub fn f_800bd870(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd870) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd870); p = true; }
    }
    f_800bd870_b(c, 0x800bd870);
    if p { crate::prof::exit(c, 0x800bd870); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd870_at(c: &mut Cpu, pc: u32) { f_800bd870_b(c, pc) }
#[inline(always)]
fn f_800bd870_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd870;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd870, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd870 => { ins += 10;
                c.r[0] = 0x10;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[6] = 0xcc010000;
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[0]);
                c.r[5] = 0x1005;
                c.r[0] = 0x1;
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[5]);
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[3]);
                c.sth(c.r[4].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd870: bad pc {:#x}", pc),
        }
    }
}

/// __GXSetMatrixIndex (0x800bd898, 132 bytes)
#[inline(never)]
pub fn f_800bd898(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd898) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd898); p = true; }
    }
    f_800bd898_b(c, 0x800bd898);
    if p { crate::prof::exit(c, 0x800bd898); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd898_at(c: &mut Cpu, pc: u32) { f_800bd898_b(c, pc) }
#[inline(always)]
fn f_800bd898_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd898;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd898, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd898 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x5);
                if !cr_bit(cr, 0) { pc = 0x800bd8d8; continue; }
                pc = 0x800bd8a0;
            }
            0x800bd8a0 => { ins += 14;
                c.r[0] = 0x8;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x30;
                c.r[3] = 0x10;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x1018;
                c.r[4] = c.lwz_t(ins, c.r[4].wrapping_add(0x80));
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[4]);
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[4]);
                pc = 0x800bd90c; continue;
                pc = 0x800bd8d8;
            }
            0x800bd8d8 => { ins += 13;
                c.r[0] = 0x8;
                c.r[4] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[5] = 0xcc010000;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x40;
                c.r[3] = 0x10;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = 0x1019;
                c.r[4] = c.lwz_t(ins, c.r[4].wrapping_add(0x84));
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[4]);
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[4]);
                pc = 0x800bd90c;
            }
            0x800bd90c => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                c.r[0] = 0x1;
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd898: bad pc {:#x}", pc),
        }
    }
}

/// fn_800BD91C (0x800bd91c, 2120 bytes)
#[inline(never)]
pub fn f_800bd91c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bd91c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bd91c); p = true; }
    }
    f_800bd91c_b(c, 0x800bd91c);
    if p { crate::prof::exit(c, 0x800bd91c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bd91c_at(c: &mut Cpu, pc: u32) { f_800bd91c_b(c, pc) }
#[inline(never)]
fn f_800bd91c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bd91c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bd91c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bd91c => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                pc = 0x800bd920;
            }
            0x800bd920 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0x4e4));
                pc = 0x800bd924;
            }
            0x800bd924 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x22);
                pc = 0x800bd928;
            }
            0x800bd928 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800bd954; continue; }
                pc = 0x800bd92c;
            }
            0x800bd92c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bd9a0; continue; }
                pc = 0x800bd930;
            }
            0x800bd930 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xb);
                pc = 0x800bd934;
            }
            0x800bd934 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bd944; continue; }
                pc = 0x800bd938;
            }
            0x800bd938 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                pc = 0x800bd93c;
            }
            0x800bd93c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bd954; continue; }
                pc = 0x800bd940;
            }
            0x800bd940 => { ins += 1;
                pc = 0x800bd9a0; continue;
                pc = 0x800bd944;
            }
            0x800bd944 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x1b);
                pc = 0x800bd948;
            }
            0x800bd948 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bd98c; continue; }
                pc = 0x800bd94c;
            }
            0x800bd94c => { ins += 1;
                pc = 0x800bd974; continue;
                pc = 0x800bd950;
            }
            0x800bd950 => { ins += 1;
                pc = 0x800bd9a0; continue;
                pc = 0x800bd954;
            }
            0x800bd954 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bd958;
            }
            0x800bd958 => { ins += 1;
                c.r[6] = 0xcc010000;
                pc = 0x800bd95c;
            }
            0x800bd95c => { ins += 1;
                c.stb(c.r[6].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bd960;
            }
            0x800bd960 => { ins += 1;
                c.r[5] = 0x1006;
                pc = 0x800bd964;
            }
            0x800bd964 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800bd968;
            }
            0x800bd968 => { ins += 1;
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[5]);
                pc = 0x800bd96c;
            }
            0x800bd96c => { ins += 1;
                c.stw(c.r[6].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bd970;
            }
            0x800bd970 => { ins += 1;
                pc = 0x800bd9a0; continue;
                pc = 0x800bd974;
            }
            0x800bd974 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bd978;
            }
            0x800bd978 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bd97c;
            }
            0x800bd97c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bd980;
            }
            0x800bd980 => { ins += 1;
                c.r[0] = 0x23000000;
                pc = 0x800bd984;
            }
            0x800bd984 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bd988;
            }
            0x800bd988 => { ins += 1;
                pc = 0x800bd9a0; continue;
                pc = 0x800bd98c;
            }
            0x800bd98c => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bd990;
            }
            0x800bd990 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bd994;
            }
            0x800bd994 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bd998;
            }
            0x800bd998 => { ins += 1;
                c.r[0] = 0x24000000;
                pc = 0x800bd99c;
            }
            0x800bd99c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bd9a0;
            }
            0x800bd9a0 => { ins += 1;
                c.r[7] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                pc = 0x800bd9a4;
            }
            0x800bd9a4 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[7].wrapping_add(0x4e8));
                pc = 0x800bd9a8;
            }
            0x800bd9a8 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x15);
                pc = 0x800bd9ac;
            }
            0x800bd9ac => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800bd9d8; continue; }
                pc = 0x800bd9b0;
            }
            0x800bd9b0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bda28; continue; }
                pc = 0x800bd9b4;
            }
            0x800bd9b4 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x9);
                pc = 0x800bd9b8;
            }
            0x800bd9b8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bd9c8; continue; }
                pc = 0x800bd9bc;
            }
            0x800bd9bc => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                pc = 0x800bd9c0;
            }
            0x800bd9c0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bd9d8; continue; }
                pc = 0x800bd9c4;
            }
            0x800bd9c4 => { ins += 1;
                pc = 0x800bda28; continue;
                pc = 0x800bd9c8;
            }
            0x800bd9c8 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x11);
                pc = 0x800bd9cc;
            }
            0x800bd9cc => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bda1c; continue; }
                pc = 0x800bd9d0;
            }
            0x800bd9d0 => { ins += 1;
                pc = 0x800bd9f0; continue;
                pc = 0x800bd9d4;
            }
            0x800bd9d4 => { ins += 1;
                pc = 0x800bda28; continue;
                pc = 0x800bd9d8;
            }
            0x800bd9d8 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bd9dc;
            }
            0x800bd9dc => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bd9e0;
            }
            0x800bd9e0 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bd9e4;
            }
            0x800bd9e4 => { ins += 1;
                c.r[0] = 0x67000000;
                pc = 0x800bd9e8;
            }
            0x800bd9e8 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bd9ec;
            }
            0x800bd9ec => { ins += 1;
                pc = 0x800bda28; continue;
                pc = 0x800bd9f0;
            }
            0x800bd9f0 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[7].wrapping_add(0x4ec));
                pc = 0x800bd9f4;
            }
            0x800bd9f4 => { ins += 1;
                c.r[6] = 0x8;
                pc = 0x800bd9f8;
            }
            0x800bd9f8 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bd9fc;
            }
            0x800bd9fc => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff0f;
                pc = 0x800bda00;
            }
            0x800bda00 => { ins += 1;
                c.stw(c.r[7].wrapping_add(0x4ec), c.r[0]);
                pc = 0x800bda04;
            }
            0x800bda04 => { ins += 1;
                c.r[0] = 0x20;
                pc = 0x800bda08;
            }
            0x800bda08 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[6]);
                pc = 0x800bda0c;
            }
            0x800bda0c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bda10;
            }
            0x800bda10 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[7].wrapping_add(0x4ec));
                pc = 0x800bda14;
            }
            0x800bda14 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bda18;
            }
            0x800bda18 => { ins += 1;
                pc = 0x800bda28; continue;
                pc = 0x800bda1c;
            }
            0x800bda1c => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa15c));
                pc = 0x800bda20;
            }
            0x800bda20 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800bda24;
            }
            0x800bda24 => { ins += 1;
                c.sth(c.r[5].wrapping_add(0x6), c.r[0]);
                pc = 0x800bda28;
            }
            0x800bda28 => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                pc = 0x800bda2c;
            }
            0x800bda2c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0x4e4), c.r[3]);
                pc = 0x800bda30;
            }
            0x800bda30 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0x4e4));
                pc = 0x800bda34;
            }
            0x800bda34 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x23);
                pc = 0x800bda38;
            }
            0x800bda38 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x800bde54; continue; }
                pc = 0x800bda3c;
            }
            0x800bda3c => { ins += 1;
                c.r[3] = 0x80310000;
                pc = 0x800bda40;
            }
            0x800bda40 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x3684);
                pc = 0x800bda44;
            }
            0x800bda44 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800bda48;
            }
            0x800bda48 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[0]));
                pc = 0x800bda4c;
            }
            0x800bda4c => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x800bda50;
            }
            0x800bda50 => { ins += 1;
                if true { let t = c.ctr; if (0x800bd91c..0x800be164).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800bda54;
            }
            0x800bda54 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bda58;
            }
            0x800bda58 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bda5c;
            }
            0x800bda5c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bda60;
            }
            0x800bda60 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bda64;
            }
            0x800bda64 => { ins += 1;
                c.r[0] = 0x273;
                pc = 0x800bda68;
            }
            0x800bda68 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bda6c;
            }
            0x800bda6c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bda70;
            }
            0x800bda70 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bda74;
            }
            0x800bda74 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bda78;
            }
            0x800bda78 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bda7c;
            }
            0x800bda7c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bda80;
            }
            0x800bda80 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bda84;
            }
            0x800bda84 => { ins += 1;
                c.r[0] = 0x14a;
                pc = 0x800bda88;
            }
            0x800bda88 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bda8c;
            }
            0x800bda8c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bda90;
            }
            0x800bda90 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bda94;
            }
            0x800bda94 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bda98;
            }
            0x800bda98 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bda9c;
            }
            0x800bda9c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdaa0;
            }
            0x800bdaa0 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bdaa4;
            }
            0x800bdaa4 => { ins += 1;
                c.r[0] = 0x16b;
                pc = 0x800bdaa8;
            }
            0x800bdaa8 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bdaac;
            }
            0x800bdaac => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdab0;
            }
            0x800bdab0 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdab4;
            }
            0x800bdab4 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bdab8;
            }
            0x800bdab8 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdabc;
            }
            0x800bdabc => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdac0;
            }
            0x800bdac0 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bdac4;
            }
            0x800bdac4 => { ins += 1;
                c.r[0] = 0x84;
                pc = 0x800bdac8;
            }
            0x800bdac8 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bdacc;
            }
            0x800bdacc => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdad0;
            }
            0x800bdad0 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdad4;
            }
            0x800bdad4 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bdad8;
            }
            0x800bdad8 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdadc;
            }
            0x800bdadc => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdae0;
            }
            0x800bdae0 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bdae4;
            }
            0x800bdae4 => { ins += 1;
                c.r[0] = 0xc6;
                pc = 0x800bdae8;
            }
            0x800bdae8 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bdaec;
            }
            0x800bdaec => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdaf0;
            }
            0x800bdaf0 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdaf4;
            }
            0x800bdaf4 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bdaf8;
            }
            0x800bdaf8 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdafc;
            }
            0x800bdafc => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdb00;
            }
            0x800bdb00 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bdb04;
            }
            0x800bdb04 => { ins += 1;
                c.r[0] = 0x210;
                pc = 0x800bdb08;
            }
            0x800bdb08 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bdb0c;
            }
            0x800bdb0c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdb10;
            }
            0x800bdb10 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdb14;
            }
            0x800bdb14 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bdb18;
            }
            0x800bdb18 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdb1c;
            }
            0x800bdb1c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdb20;
            }
            0x800bdb20 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bdb24;
            }
            0x800bdb24 => { ins += 1;
                c.r[0] = 0x252;
                pc = 0x800bdb28;
            }
            0x800bdb28 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bdb2c;
            }
            0x800bdb2c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdb30;
            }
            0x800bdb30 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdb34;
            }
            0x800bdb34 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bdb38;
            }
            0x800bdb38 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdb3c;
            }
            0x800bdb3c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdb40;
            }
            0x800bdb40 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bdb44;
            }
            0x800bdb44 => { ins += 1;
                c.r[0] = 0x231;
                pc = 0x800bdb48;
            }
            0x800bdb48 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bdb4c;
            }
            0x800bdb4c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdb50;
            }
            0x800bdb50 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdb54;
            }
            0x800bdb54 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bdb58;
            }
            0x800bdb58 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdb5c;
            }
            0x800bdb5c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdb60;
            }
            0x800bdb60 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bdb64;
            }
            0x800bdb64 => { ins += 1;
                c.r[0] = 0x1ad;
                pc = 0x800bdb68;
            }
            0x800bdb68 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bdb6c;
            }
            0x800bdb6c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdb70;
            }
            0x800bdb70 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdb74;
            }
            0x800bdb74 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bdb78;
            }
            0x800bdb78 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdb7c;
            }
            0x800bdb7c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdb80;
            }
            0x800bdb80 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bdb84;
            }
            0x800bdb84 => { ins += 1;
                c.r[0] = 0x1ce;
                pc = 0x800bdb88;
            }
            0x800bdb88 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bdb8c;
            }
            0x800bdb8c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdb90;
            }
            0x800bdb90 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdb94;
            }
            0x800bdb94 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bdb98;
            }
            0x800bdb98 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdb9c;
            }
            0x800bdb9c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdba0;
            }
            0x800bdba0 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bdba4;
            }
            0x800bdba4 => { ins += 1;
                c.r[0] = 0x21;
                pc = 0x800bdba8;
            }
            0x800bdba8 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bdbac;
            }
            0x800bdbac => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdbb0;
            }
            0x800bdbb0 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdbb4;
            }
            0x800bdbb4 => { ins += 1;
                c.r[0] = 0x10;
                pc = 0x800bdbb8;
            }
            0x800bdbb8 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdbbc;
            }
            0x800bdbbc => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdbc0;
            }
            0x800bdbc0 => { ins += 1;
                c.r[3] = 0x1006;
                pc = 0x800bdbc4;
            }
            0x800bdbc4 => { ins += 1;
                c.r[0] = 0x153;
                pc = 0x800bdbc8;
            }
            0x800bdbc8 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[3]);
                pc = 0x800bdbcc;
            }
            0x800bdbcc => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdbd0;
            }
            0x800bdbd0 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdbd4;
            }
            0x800bdbd4 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdbd8;
            }
            0x800bdbd8 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdbdc;
            }
            0x800bdbdc => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdbe0;
            }
            0x800bdbe0 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdbe4;
            }
            0x800bdbe4 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffae7f);
                pc = 0x800bdbe8;
            }
            0x800bdbe8 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdbec;
            }
            0x800bdbec => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdbf0;
            }
            0x800bdbf0 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdbf4;
            }
            0x800bdbf4 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdbf8;
            }
            0x800bdbf8 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdbfc;
            }
            0x800bdbfc => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc00;
            }
            0x800bdc00 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffff8e7f);
                pc = 0x800bdc04;
            }
            0x800bdc04 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc08;
            }
            0x800bdc08 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdc0c;
            }
            0x800bdc0c => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdc10;
            }
            0x800bdc10 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdc14;
            }
            0x800bdc14 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdc18;
            }
            0x800bdc18 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc1c;
            }
            0x800bdc1c => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffff9e7f);
                pc = 0x800bdc20;
            }
            0x800bdc20 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc24;
            }
            0x800bdc24 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdc28;
            }
            0x800bdc28 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdc2c;
            }
            0x800bdc2c => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdc30;
            }
            0x800bdc30 => { ins += 1;
                c.r[3] = 0x23000000;
                pc = 0x800bdc34;
            }
            0x800bdc34 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc38;
            }
            0x800bdc38 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x1e7f);
                pc = 0x800bdc3c;
            }
            0x800bdc3c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc40;
            }
            0x800bdc40 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdc44;
            }
            0x800bdc44 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdc48;
            }
            0x800bdc48 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdc4c;
            }
            0x800bdc4c => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdc50;
            }
            0x800bdc50 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc54;
            }
            0x800bdc54 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffac3f);
                pc = 0x800bdc58;
            }
            0x800bdc58 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc5c;
            }
            0x800bdc5c => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdc60;
            }
            0x800bdc60 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdc64;
            }
            0x800bdc64 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdc68;
            }
            0x800bdc68 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdc6c;
            }
            0x800bdc6c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc70;
            }
            0x800bdc70 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffac7f);
                pc = 0x800bdc74;
            }
            0x800bdc74 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc78;
            }
            0x800bdc78 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdc7c;
            }
            0x800bdc7c => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdc80;
            }
            0x800bdc80 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdc84;
            }
            0x800bdc84 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdc88;
            }
            0x800bdc88 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc8c;
            }
            0x800bdc8c => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffacbf);
                pc = 0x800bdc90;
            }
            0x800bdc90 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdc94;
            }
            0x800bdc94 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdc98;
            }
            0x800bdc98 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdc9c;
            }
            0x800bdc9c => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdca0;
            }
            0x800bdca0 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdca4;
            }
            0x800bdca4 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdca8;
            }
            0x800bdca8 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffacff);
                pc = 0x800bdcac;
            }
            0x800bdcac => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdcb0;
            }
            0x800bdcb0 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdcb4;
            }
            0x800bdcb4 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdcb8;
            }
            0x800bdcb8 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdcbc;
            }
            0x800bdcbc => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdcc0;
            }
            0x800bdcc0 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdcc4;
            }
            0x800bdcc4 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffad3f);
                pc = 0x800bdcc8;
            }
            0x800bdcc8 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdccc;
            }
            0x800bdccc => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdcd0;
            }
            0x800bdcd0 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdcd4;
            }
            0x800bdcd4 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdcd8;
            }
            0x800bdcd8 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdcdc;
            }
            0x800bdcdc => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdce0;
            }
            0x800bdce0 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffad7f);
                pc = 0x800bdce4;
            }
            0x800bdce4 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdce8;
            }
            0x800bdce8 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdcec;
            }
            0x800bdcec => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdcf0;
            }
            0x800bdcf0 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdcf4;
            }
            0x800bdcf4 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdcf8;
            }
            0x800bdcf8 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdcfc;
            }
            0x800bdcfc => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffadbf);
                pc = 0x800bdd00;
            }
            0x800bdd00 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd04;
            }
            0x800bdd04 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdd08;
            }
            0x800bdd08 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdd0c;
            }
            0x800bdd0c => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdd10;
            }
            0x800bdd10 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdd14;
            }
            0x800bdd14 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd18;
            }
            0x800bdd18 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffadff);
                pc = 0x800bdd1c;
            }
            0x800bdd1c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd20;
            }
            0x800bdd20 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdd24;
            }
            0x800bdd24 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdd28;
            }
            0x800bdd28 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdd2c;
            }
            0x800bdd2c => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdd30;
            }
            0x800bdd30 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd34;
            }
            0x800bdd34 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffae3f);
                pc = 0x800bdd38;
            }
            0x800bdd38 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd3c;
            }
            0x800bdd3c => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdd40;
            }
            0x800bdd40 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdd44;
            }
            0x800bdd44 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdd48;
            }
            0x800bdd48 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdd4c;
            }
            0x800bdd4c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd50;
            }
            0x800bdd50 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffa27f);
                pc = 0x800bdd54;
            }
            0x800bdd54 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd58;
            }
            0x800bdd58 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdd5c;
            }
            0x800bdd5c => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdd60;
            }
            0x800bdd60 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdd64;
            }
            0x800bdd64 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdd68;
            }
            0x800bdd68 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd6c;
            }
            0x800bdd6c => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffa67f);
                pc = 0x800bdd70;
            }
            0x800bdd70 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd74;
            }
            0x800bdd74 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdd78;
            }
            0x800bdd78 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdd7c;
            }
            0x800bdd7c => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdd80;
            }
            0x800bdd80 => { ins += 1;
                c.r[3] = 0x23010000;
                pc = 0x800bdd84;
            }
            0x800bdd84 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd88;
            }
            0x800bdd88 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffaa7f);
                pc = 0x800bdd8c;
            }
            0x800bdd8c => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdd90;
            }
            0x800bdd90 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdd94;
            }
            0x800bdd94 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdd98;
            }
            0x800bdd98 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bdd9c;
            }
            0x800bdd9c => { ins += 1;
                c.r[3] = 0x24030000;
                pc = 0x800bdda0;
            }
            0x800bdda0 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdda4;
            }
            0x800bdda4 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffc0c6);
                pc = 0x800bdda8;
            }
            0x800bdda8 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bddac;
            }
            0x800bddac => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bddb0;
            }
            0x800bddb0 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bddb4;
            }
            0x800bddb4 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bddb8;
            }
            0x800bddb8 => { ins += 1;
                c.r[3] = 0x24030000;
                pc = 0x800bddbc;
            }
            0x800bddbc => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bddc0;
            }
            0x800bddc0 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffc16b);
                pc = 0x800bddc4;
            }
            0x800bddc4 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bddc8;
            }
            0x800bddc8 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bddcc;
            }
            0x800bddcc => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bddd0;
            }
            0x800bddd0 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bddd4;
            }
            0x800bddd4 => { ins += 1;
                c.r[3] = 0x24030000;
                pc = 0x800bddd8;
            }
            0x800bddd8 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdddc;
            }
            0x800bdddc => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffc0e7);
                pc = 0x800bdde0;
            }
            0x800bdde0 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdde4;
            }
            0x800bdde4 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bdde8;
            }
            0x800bdde8 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bddec;
            }
            0x800bddec => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bddf0;
            }
            0x800bddf0 => { ins += 1;
                c.r[3] = 0x24030000;
                pc = 0x800bddf4;
            }
            0x800bddf4 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bddf8;
            }
            0x800bddf8 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffc108);
                pc = 0x800bddfc;
            }
            0x800bddfc => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bde00;
            }
            0x800bde00 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bde04;
            }
            0x800bde04 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bde08;
            }
            0x800bde08 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bde0c;
            }
            0x800bde0c => { ins += 1;
                c.r[3] = 0x24030000;
                pc = 0x800bde10;
            }
            0x800bde10 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bde14;
            }
            0x800bde14 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffc129);
                pc = 0x800bde18;
            }
            0x800bde18 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bde1c;
            }
            0x800bde1c => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bde20;
            }
            0x800bde20 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bde24;
            }
            0x800bde24 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bde28;
            }
            0x800bde28 => { ins += 1;
                c.r[3] = 0x24030000;
                pc = 0x800bde2c;
            }
            0x800bde2c => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bde30;
            }
            0x800bde30 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffc14a);
                pc = 0x800bde34;
            }
            0x800bde34 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bde38;
            }
            0x800bde38 => { ins += 1;
                pc = 0x800bde54; continue;
                pc = 0x800bde3c;
            }
            0x800bde3c => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bde40;
            }
            0x800bde40 => { ins += 1;
                c.r[5] = 0xcc010000;
                pc = 0x800bde44;
            }
            0x800bde44 => { ins += 1;
                c.r[3] = 0x24030000;
                pc = 0x800bde48;
            }
            0x800bde48 => { ins += 1;
                c.stb(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bde4c;
            }
            0x800bde4c => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0xffffc1ad);
                pc = 0x800bde50;
            }
            0x800bde50 => { ins += 1;
                c.stw(c.r[5].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bde54;
            }
            0x800bde54 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                pc = 0x800bde58;
            }
            0x800bde58 => { ins += 1;
                c.stw(c.r[3].wrapping_add(0x4e8), c.r[4]);
                pc = 0x800bde5c;
            }
            0x800bde5c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4e8));
                pc = 0x800bde60;
            }
            0x800bde60 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x16);
                pc = 0x800bde64;
            }
            0x800bde64 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x800be154; continue; }
                pc = 0x800bde68;
            }
            0x800bde68 => { ins += 1;
                c.r[4] = 0x80310000;
                pc = 0x800bde6c;
            }
            0x800bde6c => { ins += 1;
                c.r[4] = c.r[4].wrapping_add(0x3628);
                pc = 0x800bde70;
            }
            0x800bde70 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800bde74;
            }
            0x800bde74 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(c.r[0]));
                pc = 0x800bde78;
            }
            0x800bde78 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x800bde7c;
            }
            0x800bde7c => { ins += 1;
                if true { let t = c.ctr; if (0x800bd91c..0x800be164).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800bde80;
            }
            0x800bde80 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bde84;
            }
            0x800bde84 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bde88;
            }
            0x800bde88 => { ins += 1;
                c.r[3] = 0x67000000;
                pc = 0x800bde8c;
            }
            0x800bde8c => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bde90;
            }
            0x800bde90 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x42);
                pc = 0x800bde94;
            }
            0x800bde94 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bde98;
            }
            0x800bde98 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bde9c;
            }
            0x800bde9c => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdea0;
            }
            0x800bdea0 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdea4;
            }
            0x800bdea4 => { ins += 1;
                c.r[3] = 0x67000000;
                pc = 0x800bdea8;
            }
            0x800bdea8 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdeac;
            }
            0x800bdeac => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x84);
                pc = 0x800bdeb0;
            }
            0x800bdeb0 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdeb4;
            }
            0x800bdeb4 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bdeb8;
            }
            0x800bdeb8 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdebc;
            }
            0x800bdebc => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdec0;
            }
            0x800bdec0 => { ins += 1;
                c.r[3] = 0x67000000;
                pc = 0x800bdec4;
            }
            0x800bdec4 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdec8;
            }
            0x800bdec8 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x63);
                pc = 0x800bdecc;
            }
            0x800bdecc => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bded0;
            }
            0x800bded0 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bded4;
            }
            0x800bded4 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bded8;
            }
            0x800bded8 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdedc;
            }
            0x800bdedc => { ins += 1;
                c.r[3] = 0x67000000;
                pc = 0x800bdee0;
            }
            0x800bdee0 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdee4;
            }
            0x800bdee4 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x129);
                pc = 0x800bdee8;
            }
            0x800bdee8 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdeec;
            }
            0x800bdeec => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bdef0;
            }
            0x800bdef0 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdef4;
            }
            0x800bdef4 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdef8;
            }
            0x800bdef8 => { ins += 1;
                c.r[3] = 0x67000000;
                pc = 0x800bdefc;
            }
            0x800bdefc => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf00;
            }
            0x800bdf00 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x252);
                pc = 0x800bdf04;
            }
            0x800bdf04 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf08;
            }
            0x800bdf08 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bdf0c;
            }
            0x800bdf0c => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdf10;
            }
            0x800bdf10 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdf14;
            }
            0x800bdf14 => { ins += 1;
                c.r[3] = 0x67000000;
                pc = 0x800bdf18;
            }
            0x800bdf18 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf1c;
            }
            0x800bdf1c => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x21);
                pc = 0x800bdf20;
            }
            0x800bdf20 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf24;
            }
            0x800bdf24 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bdf28;
            }
            0x800bdf28 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdf2c;
            }
            0x800bdf2c => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdf30;
            }
            0x800bdf30 => { ins += 1;
                c.r[3] = 0x67000000;
                pc = 0x800bdf34;
            }
            0x800bdf34 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf38;
            }
            0x800bdf38 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x14b);
                pc = 0x800bdf3c;
            }
            0x800bdf3c => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf40;
            }
            0x800bdf40 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bdf44;
            }
            0x800bdf44 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdf48;
            }
            0x800bdf48 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdf4c;
            }
            0x800bdf4c => { ins += 1;
                c.r[3] = 0x67000000;
                pc = 0x800bdf50;
            }
            0x800bdf50 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf54;
            }
            0x800bdf54 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x18d);
                pc = 0x800bdf58;
            }
            0x800bdf58 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf5c;
            }
            0x800bdf5c => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bdf60;
            }
            0x800bdf60 => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdf64;
            }
            0x800bdf64 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdf68;
            }
            0x800bdf68 => { ins += 1;
                c.r[3] = 0x67000000;
                pc = 0x800bdf6c;
            }
            0x800bdf6c => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf70;
            }
            0x800bdf70 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x1cf);
                pc = 0x800bdf74;
            }
            0x800bdf74 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf78;
            }
            0x800bdf78 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bdf7c;
            }
            0x800bdf7c => { ins += 1;
                c.r[0] = 0x61;
                pc = 0x800bdf80;
            }
            0x800bdf80 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdf84;
            }
            0x800bdf84 => { ins += 1;
                c.r[3] = 0x67000000;
                pc = 0x800bdf88;
            }
            0x800bdf88 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf8c;
            }
            0x800bdf8c => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(0x211);
                pc = 0x800bdf90;
            }
            0x800bdf90 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdf94;
            }
            0x800bdf94 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bdf98;
            }
            0x800bdf98 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800bdf9c;
            }
            0x800bdf9c => { ins += 1;
                c.r[5] = 0x8;
                pc = 0x800bdfa0;
            }
            0x800bdfa0 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdfa4;
            }
            0x800bdfa4 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff0f;
                pc = 0x800bdfa8;
            }
            0x800bdfa8 => { ins += 1;
                c.r[0] = c.r[0] | 0x20;
                pc = 0x800bdfac;
            }
            0x800bdfac => { ins += 1;
                c.stw(c.r[3].wrapping_add(0x4ec), c.r[0]);
                pc = 0x800bdfb0;
            }
            0x800bdfb0 => { ins += 1;
                c.r[0] = 0x20;
                pc = 0x800bdfb4;
            }
            0x800bdfb4 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                pc = 0x800bdfb8;
            }
            0x800bdfb8 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdfbc;
            }
            0x800bdfbc => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800bdfc0;
            }
            0x800bdfc0 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdfc4;
            }
            0x800bdfc4 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bdfc8;
            }
            0x800bdfc8 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800bdfcc;
            }
            0x800bdfcc => { ins += 1;
                c.r[5] = 0x8;
                pc = 0x800bdfd0;
            }
            0x800bdfd0 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800bdfd4;
            }
            0x800bdfd4 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff0f;
                pc = 0x800bdfd8;
            }
            0x800bdfd8 => { ins += 1;
                c.r[0] = c.r[0] | 0x30;
                pc = 0x800bdfdc;
            }
            0x800bdfdc => { ins += 1;
                c.stw(c.r[3].wrapping_add(0x4ec), c.r[0]);
                pc = 0x800bdfe0;
            }
            0x800bdfe0 => { ins += 1;
                c.r[0] = 0x20;
                pc = 0x800bdfe4;
            }
            0x800bdfe4 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                pc = 0x800bdfe8;
            }
            0x800bdfe8 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdfec;
            }
            0x800bdfec => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800bdff0;
            }
            0x800bdff0 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800bdff4;
            }
            0x800bdff4 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800bdff8;
            }
            0x800bdff8 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800bdffc;
            }
            0x800bdffc => { ins += 1;
                c.r[5] = 0x8;
                pc = 0x800be000;
            }
            0x800be000 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800be004;
            }
            0x800be004 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff0f;
                pc = 0x800be008;
            }
            0x800be008 => { ins += 1;
                c.r[0] = c.r[0] | 0x40;
                pc = 0x800be00c;
            }
            0x800be00c => { ins += 1;
                c.stw(c.r[3].wrapping_add(0x4ec), c.r[0]);
                pc = 0x800be010;
            }
            0x800be010 => { ins += 1;
                c.r[0] = 0x20;
                pc = 0x800be014;
            }
            0x800be014 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                pc = 0x800be018;
            }
            0x800be018 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be01c;
            }
            0x800be01c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be020;
            }
            0x800be020 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be024;
            }
            0x800be024 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800be028;
            }
            0x800be028 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be02c;
            }
            0x800be02c => { ins += 1;
                c.r[5] = 0x8;
                pc = 0x800be030;
            }
            0x800be030 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800be034;
            }
            0x800be034 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff0f;
                pc = 0x800be038;
            }
            0x800be038 => { ins += 1;
                c.r[0] = c.r[0] | 0x50;
                pc = 0x800be03c;
            }
            0x800be03c => { ins += 1;
                c.stw(c.r[3].wrapping_add(0x4ec), c.r[0]);
                pc = 0x800be040;
            }
            0x800be040 => { ins += 1;
                c.r[0] = 0x20;
                pc = 0x800be044;
            }
            0x800be044 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                pc = 0x800be048;
            }
            0x800be048 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be04c;
            }
            0x800be04c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be050;
            }
            0x800be050 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be054;
            }
            0x800be054 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800be058;
            }
            0x800be058 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be05c;
            }
            0x800be05c => { ins += 1;
                c.r[5] = 0x8;
                pc = 0x800be060;
            }
            0x800be060 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800be064;
            }
            0x800be064 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff0f;
                pc = 0x800be068;
            }
            0x800be068 => { ins += 1;
                c.r[0] = c.r[0] | 0x60;
                pc = 0x800be06c;
            }
            0x800be06c => { ins += 1;
                c.stw(c.r[3].wrapping_add(0x4ec), c.r[0]);
                pc = 0x800be070;
            }
            0x800be070 => { ins += 1;
                c.r[0] = 0x20;
                pc = 0x800be074;
            }
            0x800be074 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                pc = 0x800be078;
            }
            0x800be078 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be07c;
            }
            0x800be07c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be080;
            }
            0x800be080 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be084;
            }
            0x800be084 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800be088;
            }
            0x800be088 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be08c;
            }
            0x800be08c => { ins += 1;
                c.r[5] = 0x8;
                pc = 0x800be090;
            }
            0x800be090 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800be094;
            }
            0x800be094 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff0f;
                pc = 0x800be098;
            }
            0x800be098 => { ins += 1;
                c.r[0] = c.r[0] | 0x70;
                pc = 0x800be09c;
            }
            0x800be09c => { ins += 1;
                c.stw(c.r[3].wrapping_add(0x4ec), c.r[0]);
                pc = 0x800be0a0;
            }
            0x800be0a0 => { ins += 1;
                c.r[0] = 0x20;
                pc = 0x800be0a4;
            }
            0x800be0a4 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                pc = 0x800be0a8;
            }
            0x800be0a8 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be0ac;
            }
            0x800be0ac => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be0b0;
            }
            0x800be0b0 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be0b4;
            }
            0x800be0b4 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800be0b8;
            }
            0x800be0b8 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be0bc;
            }
            0x800be0bc => { ins += 1;
                c.r[5] = 0x8;
                pc = 0x800be0c0;
            }
            0x800be0c0 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800be0c4;
            }
            0x800be0c4 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff0f;
                pc = 0x800be0c8;
            }
            0x800be0c8 => { ins += 1;
                c.r[0] = c.r[0] | 0x90;
                pc = 0x800be0cc;
            }
            0x800be0cc => { ins += 1;
                c.stw(c.r[3].wrapping_add(0x4ec), c.r[0]);
                pc = 0x800be0d0;
            }
            0x800be0d0 => { ins += 1;
                c.r[0] = 0x20;
                pc = 0x800be0d4;
            }
            0x800be0d4 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                pc = 0x800be0d8;
            }
            0x800be0d8 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be0dc;
            }
            0x800be0dc => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be0e0;
            }
            0x800be0e0 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be0e4;
            }
            0x800be0e4 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800be0e8;
            }
            0x800be0e8 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be0ec;
            }
            0x800be0ec => { ins += 1;
                c.r[5] = 0x8;
                pc = 0x800be0f0;
            }
            0x800be0f0 => { ins += 1;
                c.r[4] = 0xcc010000;
                pc = 0x800be0f4;
            }
            0x800be0f4 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff0f;
                pc = 0x800be0f8;
            }
            0x800be0f8 => { ins += 1;
                c.r[0] = c.r[0] | 0x80;
                pc = 0x800be0fc;
            }
            0x800be0fc => { ins += 1;
                c.stw(c.r[3].wrapping_add(0x4ec), c.r[0]);
                pc = 0x800be100;
            }
            0x800be100 => { ins += 1;
                c.r[0] = 0x20;
                pc = 0x800be104;
            }
            0x800be104 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[5]);
                pc = 0x800be108;
            }
            0x800be108 => { ins += 1;
                c.stb(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be10c;
            }
            0x800be10c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4ec));
                pc = 0x800be110;
            }
            0x800be110 => { ins += 1;
                c.stw(c.r[4].wrapping_add(0xffff8000), c.r[0]);
                pc = 0x800be114;
            }
            0x800be114 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800be118;
            }
            0x800be118 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa15c));
                pc = 0x800be11c;
            }
            0x800be11c => { ins += 1;
                c.r[0] = 0x2;
                pc = 0x800be120;
            }
            0x800be120 => { ins += 1;
                c.sth(c.r[3].wrapping_add(0x6), c.r[0]);
                pc = 0x800be124;
            }
            0x800be124 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800be128;
            }
            0x800be128 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa15c));
                pc = 0x800be12c;
            }
            0x800be12c => { ins += 1;
                c.r[0] = 0x3;
                pc = 0x800be130;
            }
            0x800be130 => { ins += 1;
                c.sth(c.r[3].wrapping_add(0x6), c.r[0]);
                pc = 0x800be134;
            }
            0x800be134 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800be138;
            }
            0x800be138 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa15c));
                pc = 0x800be13c;
            }
            0x800be13c => { ins += 1;
                c.r[0] = 0x4;
                pc = 0x800be140;
            }
            0x800be140 => { ins += 1;
                c.sth(c.r[3].wrapping_add(0x6), c.r[0]);
                pc = 0x800be144;
            }
            0x800be144 => { ins += 1;
                pc = 0x800be154; continue;
                pc = 0x800be148;
            }
            0x800be148 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa15c));
                pc = 0x800be14c;
            }
            0x800be14c => { ins += 1;
                c.r[0] = 0x5;
                pc = 0x800be150;
            }
            0x800be150 => { ins += 1;
                c.sth(c.r[3].wrapping_add(0x6), c.r[0]);
                pc = 0x800be154;
            }
            0x800be154 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                pc = 0x800be158;
            }
            0x800be158 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800be15c;
            }
            0x800be15c => { ins += 1;
                c.sth(c.r[3].wrapping_add(0x2), c.r[0]);
                pc = 0x800be160;
            }
            0x800be160 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bd91c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800BE164 (0x800be164, 424 bytes)
#[inline(never)]
pub fn f_800be164(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be164) { return; }
        if c.prof { crate::prof::enter(c, 0x800be164); p = true; }
    }
    f_800be164_b(c, 0x800be164);
    if p { crate::prof::exit(c, 0x800be164); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be164_at(c: &mut Cpu, pc: u32) { f_800be164_b(c, pc) }
#[inline(never)]
fn f_800be164_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be164;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be164, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be164 => { ins += 1;
                c.r[10] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa15c));
                pc = 0x800be168;
            }
            0x800be168 => { ins += 1;
                c.r[8] = c.r[10].wrapping_add(0x42);
                pc = 0x800be16c;
            }
            0x800be16c => { ins += 1;
                c.r[6] = c.lhz_t(ins, c.r[10].wrapping_add(0x42));
                pc = 0x800be170;
            }
            0x800be170 => { ins += 1;
                c.r[7] = c.r[10].wrapping_add(0x40);
                pc = 0x800be174;
            }
            0x800be174 => { ins += 1;
                c.r[0] = c.r[6] | c.r[6];
                pc = 0x800be178;
            }
            0x800be178 => { ins += 1;
                c.r[5] = c.lhz_t(ins, c.r[7]);
                pc = 0x800be17c;
            }
            0x800be17c => { ins += 1;
                c.r[6] = c.lhz_t(ins, c.r[8]);
                pc = 0x800be180;
            }
            0x800be180 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], c.r[0]);
                pc = 0x800be184;
            }
            0x800be184 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800be174; continue; }
                pc = 0x800be188;
            }
            0x800be188 => { ins += 1;
                c.r[0] = c.r[6].rotate_left(16) & 0xffff0000;
                pc = 0x800be18c;
            }
            0x800be18c => { ins += 1;
                c.r[6] = c.r[0] | c.r[5];
                pc = 0x800be190;
            }
            0x800be190 => { ins += 1;
                c.r[9] = c.r[10].wrapping_add(0x46);
                pc = 0x800be194;
            }
            0x800be194 => { ins += 1;
                c.r[7] = c.lhz_t(ins, c.r[10].wrapping_add(0x46));
                pc = 0x800be198;
            }
            0x800be198 => { ins += 1;
                c.r[8] = c.r[10].wrapping_add(0x44);
                pc = 0x800be19c;
            }
            0x800be19c => { ins += 1;
                c.r[0] = c.r[7] | c.r[7];
                pc = 0x800be1a0;
            }
            0x800be1a0 => { ins += 1;
                c.r[5] = c.lhz_t(ins, c.r[8]);
                pc = 0x800be1a4;
            }
            0x800be1a4 => { ins += 1;
                c.r[7] = c.lhz_t(ins, c.r[9]);
                pc = 0x800be1a8;
            }
            0x800be1a8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], c.r[0]);
                pc = 0x800be1ac;
            }
            0x800be1ac => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800be19c; continue; }
                pc = 0x800be1b0;
            }
            0x800be1b0 => { ins += 1;
                c.r[0] = c.r[7].rotate_left(16) & 0xffff0000;
                pc = 0x800be1b4;
            }
            0x800be1b4 => { ins += 1;
                c.r[7] = c.r[0] | c.r[5];
                pc = 0x800be1b8;
            }
            0x800be1b8 => { ins += 1;
                c.r[11] = c.r[10].wrapping_add(0x4a);
                pc = 0x800be1bc;
            }
            0x800be1bc => { ins += 1;
                c.r[8] = c.lhz_t(ins, c.r[10].wrapping_add(0x4a));
                pc = 0x800be1c0;
            }
            0x800be1c0 => { ins += 1;
                c.r[9] = c.r[10].wrapping_add(0x48);
                pc = 0x800be1c4;
            }
            0x800be1c4 => { ins += 1;
                c.r[0] = c.r[8] | c.r[8];
                pc = 0x800be1c8;
            }
            0x800be1c8 => { ins += 1;
                c.r[5] = c.lhz_t(ins, c.r[9]);
                pc = 0x800be1cc;
            }
            0x800be1cc => { ins += 1;
                c.r[8] = c.lhz_t(ins, c.r[11]);
                pc = 0x800be1d0;
            }
            0x800be1d0 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[8], c.r[0]);
                pc = 0x800be1d4;
            }
            0x800be1d4 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800be1c4; continue; }
                pc = 0x800be1d8;
            }
            0x800be1d8 => { ins += 1;
                c.r[0] = c.r[8].rotate_left(16) & 0xffff0000;
                pc = 0x800be1dc;
            }
            0x800be1dc => { ins += 1;
                c.r[8] = c.r[0] | c.r[5];
                pc = 0x800be1e0;
            }
            0x800be1e0 => { ins += 1;
                c.r[11] = c.r[10].wrapping_add(0x4e);
                pc = 0x800be1e4;
            }
            0x800be1e4 => { ins += 1;
                c.r[9] = c.lhz_t(ins, c.r[10].wrapping_add(0x4e));
                pc = 0x800be1e8;
            }
            0x800be1e8 => { ins += 1;
                c.r[10] = c.r[10].wrapping_add(0x4c);
                pc = 0x800be1ec;
            }
            0x800be1ec => { ins += 1;
                c.r[0] = c.r[9] | c.r[9];
                pc = 0x800be1f0;
            }
            0x800be1f0 => { ins += 1;
                c.r[5] = c.lhz_t(ins, c.r[10]);
                pc = 0x800be1f4;
            }
            0x800be1f4 => { ins += 1;
                c.r[9] = c.lhz_t(ins, c.r[11]);
                pc = 0x800be1f8;
            }
            0x800be1f8 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[9], c.r[0]);
                pc = 0x800be1fc;
            }
            0x800be1fc => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800be1ec; continue; }
                pc = 0x800be200;
            }
            0x800be200 => { ins += 1;
                c.r[0] = c.r[9].rotate_left(16) & 0xffff0000;
                pc = 0x800be204;
            }
            0x800be204 => { ins += 1;
                c.r[9] = c.r[0] | c.r[5];
                pc = 0x800be208;
            }
            0x800be208 => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                pc = 0x800be20c;
            }
            0x800be20c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0x4e4));
                pc = 0x800be210;
            }
            0x800be210 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xa);
                pc = 0x800be214;
            }
            0x800be214 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800be238; continue; }
                pc = 0x800be218;
            }
            0x800be218 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800be228; continue; }
                pc = 0x800be21c;
            }
            0x800be21c => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                pc = 0x800be220;
            }
            0x800be220 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800be248; continue; }
                pc = 0x800be224;
            }
            0x800be224 => { ins += 1;
                pc = 0x800be25c; continue;
                pc = 0x800be228;
            }
            0x800be228 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x23);
                pc = 0x800be22c;
            }
            0x800be22c => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800be250; continue; }
                pc = 0x800be230;
            }
            0x800be230 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800be25c; continue; }
                pc = 0x800be234;
            }
            0x800be234 => { ins += 1;
                pc = 0x800be248; continue;
                pc = 0x800be238;
            }
            0x800be238 => { ins += 1;
                c.r[0] = c.r[7].wrapping_mul(0x3e8);
                pc = 0x800be23c;
            }
            0x800be23c => { ins += 1;
                c.r[0] = divwu(c.r[0], c.r[6]);
                pc = 0x800be240;
            }
            0x800be240 => { ins += 1;
                c.stw(c.r[3], c.r[0]);
                pc = 0x800be244;
            }
            0x800be244 => { ins += 1;
                pc = 0x800be264; continue;
                pc = 0x800be248;
            }
            0x800be248 => { ins += 1;
                c.stw(c.r[3], c.r[6]);
                pc = 0x800be24c;
            }
            0x800be24c => { ins += 1;
                pc = 0x800be264; continue;
                pc = 0x800be250;
            }
            0x800be250 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800be254;
            }
            0x800be254 => { ins += 1;
                c.stw(c.r[3], c.r[0]);
                pc = 0x800be258;
            }
            0x800be258 => { ins += 1;
                pc = 0x800be264; continue;
                pc = 0x800be25c;
            }
            0x800be25c => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800be260;
            }
            0x800be260 => { ins += 1;
                c.stw(c.r[3], c.r[0]);
                pc = 0x800be264;
            }
            0x800be264 => { ins += 1;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xffff8c40));
                pc = 0x800be268;
            }
            0x800be268 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x4e8));
                pc = 0x800be26c;
            }
            0x800be26c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x16);
                pc = 0x800be270;
            }
            0x800be270 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x800be300; continue; }
                pc = 0x800be274;
            }
            0x800be274 => { ins += 1;
                c.r[3] = 0x80310000;
                pc = 0x800be278;
            }
            0x800be278 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x3714);
                pc = 0x800be27c;
            }
            0x800be27c => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800be280;
            }
            0x800be280 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[0]));
                pc = 0x800be284;
            }
            0x800be284 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x800be288;
            }
            0x800be288 => { ins += 1;
                if true { let t = c.ctr; if (0x800be164..0x800be30c).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800be28c;
            }
            0x800be28c => { ins += 1;
                c.r[0] = c.r[9].rotate_left(2) & 0xfffffffc;
                pc = 0x800be290;
            }
            0x800be290 => { ins += 1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800be294;
            }
            0x800be294 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800be298;
            }
            0x800be298 => { ins += 1;
                c.r[0] = c.r[9].rotate_left(1) & 0xfffffffe;
                pc = 0x800be29c;
            }
            0x800be29c => { ins += 1;
                c.r[0] = c.r[8].wrapping_add(c.r[0]);
                pc = 0x800be2a0;
            }
            0x800be2a0 => { ins += 1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800be2a4;
            }
            0x800be2a4 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800be2a8;
            }
            0x800be2a8 => { ins += 1;
                c.r[3] = c.r[8].wrapping_mul(0x3);
                pc = 0x800be2ac;
            }
            0x800be2ac => { ins += 1;
                c.r[0] = c.r[9].rotate_left(2) & 0xfffffffc;
                pc = 0x800be2b0;
            }
            0x800be2b0 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x800be2b4;
            }
            0x800be2b4 => { ins += 1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800be2b8;
            }
            0x800be2b8 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800be2bc;
            }
            0x800be2bc => { ins += 1;
                c.r[3] = c.r[8].wrapping_mul(0x5);
                pc = 0x800be2c0;
            }
            0x800be2c0 => { ins += 1;
                c.r[0] = c.r[9].wrapping_mul(0x6);
                pc = 0x800be2c4;
            }
            0x800be2c4 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x800be2c8;
            }
            0x800be2c8 => { ins += 1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800be2cc;
            }
            0x800be2cc => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800be2d0;
            }
            0x800be2d0 => { ins += 1;
                c.r[3] = c.r[8].wrapping_mul(0x7);
                pc = 0x800be2d4;
            }
            0x800be2d4 => { ins += 1;
                c.r[0] = c.r[9].rotate_left(3) & 0xfffffff8;
                pc = 0x800be2d8;
            }
            0x800be2d8 => { ins += 1;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x800be2dc;
            }
            0x800be2dc => { ins += 1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800be2e0;
            }
            0x800be2e0 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800be2e4;
            }
            0x800be2e4 => { ins += 1;
                c.stw(c.r[4], c.r[9]);
                pc = 0x800be2e8;
            }
            0x800be2e8 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800be2ec;
            }
            0x800be2ec => { ins += 1;
                c.stw(c.r[4], c.r[8]);
                pc = 0x800be2f0;
            }
            0x800be2f0 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800be2f4;
            }
            0x800be2f4 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800be2f8;
            }
            0x800be2f8 => { ins += 1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800be2fc;
            }
            0x800be2fc => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800be300;
            }
            0x800be300 => { ins += 1;
                c.r[0] = 0x0;
                pc = 0x800be304;
            }
            0x800be304 => { ins += 1;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800be308;
            }
            0x800be308 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be164: bad pc {:#x}", pc),
        }
    }
}

/// fn_800BE30C (0x800be30c, 16 bytes)
#[inline(never)]
pub fn f_800be30c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be30c) { return; }
        if c.prof { crate::prof::enter(c, 0x800be30c); p = true; }
    }
    f_800be30c_b(c, 0x800be30c);
    if p { crate::prof::exit(c, 0x800be30c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be30c_at(c: &mut Cpu, pc: u32) { f_800be30c_b(c, pc) }
#[inline(always)]
fn f_800be30c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be30c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be30c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be30c => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[13].wrapping_add(0xffffa15c));
                c.r[0] = 0x4;
                c.sth(c.r[3].wrapping_add(0x4), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be30c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800BE31C (0x800be31c, 44 bytes)
#[inline(never)]
pub fn f_800be31c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be31c) { return; }
        if c.prof { crate::prof::enter(c, 0x800be31c); p = true; }
    }
    f_800be31c_b(c, 0x800be31c);
    if p { crate::prof::exit(c, 0x800be31c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be31c_at(c: &mut Cpu, pc: u32) { f_800be31c_b(c, pc) }
#[inline(always)]
fn f_800be31c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be31c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be31c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be31c => { ins += 6;
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800be334; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be164(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be334;
            }
            0x800be334 => { ins += 5;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be31c: bad pc {:#x}", pc),
        }
    }
}

/// TRKNubMainLoop (0x800be348, 248 bytes)
#[inline(never)]
pub fn f_800be348(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be348) { return; }
        if c.prof { crate::prof::enter(c, 0x800be348); p = true; }
    }
    f_800be348_b(c, 0x800be348);
    if p { crate::prof::exit(c, 0x800be348); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be348_at(c: &mut Cpu, pc: u32) { f_800be348_b(c, pc) }
#[inline(always)]
fn f_800be348_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be348;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be348, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be348 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = 0x0;
                pc = 0x800be420; continue;
                pc = 0x800be368;
            }
            0x800be368 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800be370; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be55c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be370;
            }
            0x800be370 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x800be3e0; continue; }
                pc = 0x800be378;
            }
            0x800be378 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[30] = 0x0;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                if cr_bit(cr, 2) { pc = 0x800be3ac; continue; }
                pc = 0x800be388;
            }
            0x800be388 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800be39c; continue; }
                pc = 0x800be38c;
            }
            0x800be38c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800be3d4; continue; }
                pc = 0x800be394;
            }
            0x800be394 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800be3bc; continue; }
                pc = 0x800be398;
            }
            0x800be398 => { ins += 1;
                pc = 0x800be3d4; continue;
                pc = 0x800be39c;
            }
            0x800be39c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x5);
                if cr_bit(cr, 2) { pc = 0x800be3d0; continue; }
                pc = 0x800be3a4;
            }
            0x800be3a4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800be3d4; continue; }
                pc = 0x800be3a8;
            }
            0x800be3a8 => { ins += 1;
                pc = 0x800be3c4; continue;
                pc = 0x800be3ac;
            }
            0x800be3ac => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800be3b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be3b4;
            }
            0x800be3b4 => { ins += 1;
                c.lr = 0x800be3b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf3c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be3b8;
            }
            0x800be3b8 => { ins += 1;
                pc = 0x800be3d4; continue;
                pc = 0x800be3bc;
            }
            0x800be3bc => { ins += 2;
                c.r[31] = 0x1;
                pc = 0x800be3d4; continue;
                pc = 0x800be3c4;
            }
            0x800be3c4 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800be3cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c17cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be3cc;
            }
            0x800be3cc => { ins += 1;
                pc = 0x800be3d4; continue;
                pc = 0x800be3d0;
            }
            0x800be3d0 => { ins += 1;
                c.lr = 0x800be3d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1348(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be3d4;
            }
            0x800be3d4 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800be3dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be440(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be3dc;
            }
            0x800be3dc => { ins += 1;
                pc = 0x800be420; continue;
                pc = 0x800be3e0;
            }
            0x800be3e0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x800be400; continue; }
                pc = 0x800be3e8;
            }
            0x800be3e8 => { ins += 6;
                c.r[3] = 0x80480000;
                c.r[3] = c.r[3].wrapping_add(0xffffa9d8);
                c.r[3] = c.lwz_t(ins, c.r[3]);
                c.r[0] = c.lbz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800be40c; continue; }
                pc = 0x800be400;
            }
            0x800be400 => { ins += 2;
                c.r[30] = 0x1;
                c.lr = 0x800be408; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf19c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be408;
            }
            0x800be408 => { ins += 1;
                pc = 0x800be420; continue;
                pc = 0x800be40c;
            }
            0x800be40c => { ins += 1;
                c.lr = 0x800be410; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1338(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be410;
            }
            0x800be410 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be41c; continue; }
                pc = 0x800be418;
            }
            0x800be418 => { ins += 1;
                c.lr = 0x800be41c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c396c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be41c;
            }
            0x800be41c => { ins += 1;
                c.r[30] = 0x0;
                pc = 0x800be420;
            }
            0x800be420 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x800be368; continue; }
                pc = 0x800be428;
            }
            0x800be428 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be348: bad pc {:#x}", pc),
        }
    }
}

/// TRKDestructEvent (0x800be440, 36 bytes)
#[inline(never)]
pub fn f_800be440(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be440) { return; }
        if c.prof { crate::prof::enter(c, 0x800be440); p = true; }
    }
    f_800be440_b(c, 0x800be440);
    if p { crate::prof::exit(c, 0x800be440); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be440_at(c: &mut Cpu, pc: u32) { f_800be440_b(c, pc) }
#[inline(always)]
fn f_800be440_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be440;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be440, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be440 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                c.lr = 0x800be454; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be454;
            }
            0x800be454 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be440: bad pc {:#x}", pc),
        }
    }
}

/// TRKConstructEvent (0x800be464, 24 bytes)
#[inline(never)]
pub fn f_800be464(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be464) { return; }
        if c.prof { crate::prof::enter(c, 0x800be464); p = true; }
    }
    f_800be464_b(c, 0x800be464);
    if p { crate::prof::exit(c, 0x800be464); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be464_at(c: &mut Cpu, pc: u32) { f_800be464_b(c, pc) }
#[inline(always)]
fn f_800be464_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be464;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be464, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be464 => { ins += 6;
                c.stw(c.r[3], c.r[4]);
                c.r[4] = 0x0;
                c.r[0] = 0xffffffff;
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be464: bad pc {:#x}", pc),
        }
    }
}

/// TRKPostEvent (0x800be47c, 224 bytes)
#[inline(never)]
pub fn f_800be47c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be47c) { return; }
        if c.prof { crate::prof::enter(c, 0x800be47c); p = true; }
    }
    f_800be47c_b(c, 0x800be47c);
    if p { crate::prof::exit(c, 0x800be47c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be47c_at(c: &mut Cpu, pc: u32) { f_800be47c_b(c, pc) }
#[inline(always)]
fn f_800be47c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be47c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be47c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be47c => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[4].wrapping_add(0xffffcdd8);
                c.lr = 0x800be4a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be4a8;
            }
            0x800be4a8 => { ins += 5;
                c.r[3] = 0x80400000;
                c.r[30] = c.r[3].wrapping_add(0xffffcdd8);
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x2);
                if !cr_bit(cr, 2) { pc = 0x800be4c4; continue; }
                pc = 0x800be4bc;
            }
            0x800be4bc => { ins += 2;
                c.r[31] = 0x100;
                pc = 0x800be530; continue;
                pc = 0x800be4c4;
            }
            0x800be4c4 => { ins += 12;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0xc;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.r[0] = c.r[0] ^ c.r[3];
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.r[29] = c.r[0].wrapping_mul(0xc);
                c.r[3] = c.r[30].wrapping_add(c.r[29]);
                c.r[3] = c.r[3].wrapping_add(0xc);
                c.lr = 0x800be4f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be4f4;
            }
            0x800be4f4 => { ins += 10;
                c.r[3] = 0x80400000;
                c.r[4] = c.r[3].wrapping_add(0xffffcdd8);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x24));
                c.r[3] = c.r[4].wrapping_add(c.r[29]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x24));
                c.r[0] = c.r[3].wrapping_add(0x1);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x100);
                c.stw(c.r[4].wrapping_add(0x24), c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x800be524; continue; }
                pc = 0x800be51c;
            }
            0x800be51c => { ins += 2;
                c.r[0] = 0x100;
                c.stw(c.r[4].wrapping_add(0x24), c.r[0]);
                pc = 0x800be524;
            }
            0x800be524 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[30].wrapping_add(0x4), c.r[0]);
                pc = 0x800be530;
            }
            0x800be530 => { ins += 3;
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffcdd8);
                c.lr = 0x800be53c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be53c;
            }
            0x800be53c => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be47c: bad pc {:#x}", pc),
        }
    }
}

/// TRKGetNextEvent (0x800be55c, 180 bytes)
#[inline(never)]
pub fn f_800be55c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be55c) { return; }
        if c.prof { crate::prof::enter(c, 0x800be55c); p = true; }
    }
    f_800be55c_b(c, 0x800be55c);
    if p { crate::prof::exit(c, 0x800be55c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be55c_at(c: &mut Cpu, pc: u32) { f_800be55c_b(c, pc) }
#[inline(always)]
fn f_800be55c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be55c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be55c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be55c => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[4].wrapping_add(0xffffcdd8);
                c.lr = 0x800be588; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be588;
            }
            0x800be588 => { ins += 5;
                c.r[3] = 0x80400000;
                c.r[31] = c.r[3].wrapping_add(0xffffcdd8);
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 1) { pc = 0x800be5e4; continue; }
                pc = 0x800be59c;
            }
            0x800be59c => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                c.r[3] = c.r[29] | c.r[29];
                c.r[5] = 0xc;
                c.r[0] = c.r[0].wrapping_mul(0xc);
                c.r[4] = c.r[31].wrapping_add(c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0xc);
                c.lr = 0x800be5b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be5b8;
            }
            0x800be5b8 => { ins += 8;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[3] = c.r[4].wrapping_add(0xffffffff);
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x2);
                c.stw(c.r[31].wrapping_add(0x4), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x800be5e0; continue; }
                pc = 0x800be5d8;
            }
            0x800be5d8 => { ins += 2;
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                pc = 0x800be5e0;
            }
            0x800be5e0 => { ins += 1;
                c.r[30] = 0x1;
                pc = 0x800be5e4;
            }
            0x800be5e4 => { ins += 3;
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffcdd8);
                c.lr = 0x800be5f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be5f0;
            }
            0x800be5f0 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[3] = c.r[30] | c.r[30];
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be55c: bad pc {:#x}", pc),
        }
    }
}

/// TRKInitializeEventQueue (0x800be610, 88 bytes)
#[inline(never)]
pub fn f_800be610(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be610) { return; }
        if c.prof { crate::prof::enter(c, 0x800be610); p = true; }
    }
    f_800be610_b(c, 0x800be610);
    if p { crate::prof::exit(c, 0x800be610); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be610_at(c: &mut Cpu, pc: u32) { f_800be610_b(c, pc) }
#[inline(always)]
fn f_800be610_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be610;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be610, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be610 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xffffcdd8);
                c.lr = 0x800be628; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cd0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be628;
            }
            0x800be628 => { ins += 3;
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffcdd8);
                c.lr = 0x800be634; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be634;
            }
            0x800be634 => { ins += 8;
                c.r[3] = 0x80400000;
                c.r[4] = 0x0;
                c.r[3] = c.r[3].wrapping_add(0xffffcdd8);
                c.r[0] = 0x100;
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                c.lr = 0x800be654; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be654;
            }
            0x800be654 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be610: bad pc {:#x}", pc),
        }
    }
}

/// TRKNubWelcome (0x800be668, 40 bytes)
#[inline(never)]
pub fn f_800be668(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be668) { return; }
        if c.prof { crate::prof::enter(c, 0x800be668); p = true; }
    }
    f_800be668_b(c, 0x800be668);
    if p { crate::prof::exit(c, 0x800be668); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be668_at(c: &mut Cpu, pc: u32) { f_800be668_b(c, pc) }
#[inline(always)]
fn f_800be668_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be668;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be668, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be668 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xfffff610);
                c.lr = 0x800be680; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c34f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be680;
            }
            0x800be680 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be668: bad pc {:#x}", pc),
        }
    }
}

/// TRKTerminateNub (0x800be690, 36 bytes)
#[inline(never)]
pub fn f_800be690(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be690) { return; }
        if c.prof { crate::prof::enter(c, 0x800be690); p = true; }
    }
    f_800be690_b(c, 0x800be690);
    if p { crate::prof::exit(c, 0x800be690); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be690_at(c: &mut Cpu, pc: u32) { f_800be690_b(c, pc) }
#[inline(always)]
fn f_800be690_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be690;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be690, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be690 => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x800be6a0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf080(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be6a0;
            }
            0x800be6a0 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be690: bad pc {:#x}", pc),
        }
    }
}

/// TRKInitializeNub (0x800be6b4, 332 bytes)
#[inline(never)]
pub fn f_800be6b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be6b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800be6b4); p = true; }
    }
    f_800be6b4_b(c, 0x800be6b4);
    if p { crate::prof::exit(c, 0x800be6b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be6b4_at(c: &mut Cpu, pc: u32) { f_800be6b4_b(c, pc) }
#[inline(always)]
fn f_800be6b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be6b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be6b4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be6b4 => { ins += 21;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x12;
                c.r[4] = 0x34;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[3] = 0x56;
                c.r[0] = 0x78;
                c.r[6] = 0x1;
                c.stb(c.r[1].wrapping_add(0x8), c.r[5]);
                c.r[5] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stb(c.r[1].wrapping_add(0x9), c.r[4]);
                c.stb(c.r[1].wrapping_add(0xa), c.r[3]);
                c.stb(c.r[1].wrapping_add(0xb), c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                { let ea = c.r[5].wrapping_add(0xffffce00); c.stw(ea, c.r[6]); c.r[5] = ea; }
                c.r[0] = c.r[3].wrapping_add(0xedcc0000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x5678);
                if !cr_bit(cr, 2) { pc = 0x800be710; continue; }
                pc = 0x800be708;
            }
            0x800be708 => { ins += 2;
                c.stw(c.r[5], c.r[6]);
                pc = 0x800be728; continue;
                pc = 0x800be710;
            }
            0x800be710 => { ins += 3;
                c.r[0] = c.r[3].wrapping_add(0x87aa0000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x3412);
                if !cr_bit(cr, 2) { pc = 0x800be724; continue; }
                pc = 0x800be71c;
            }
            0x800be71c => { ins += 2;
                c.stw(c.r[5], c.r[31]);
                pc = 0x800be728; continue;
                pc = 0x800be724;
            }
            0x800be724 => { ins += 1;
                c.r[31] = c.r[6] | c.r[6];
                pc = 0x800be728;
            }
            0x800be728 => { ins += 5;
                c.r[3] = 0x80270000;
                c.r[4] = c.r[3].wrapping_add(0xfffff62c);
                c.r[3] = 0x1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800be73c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be73c;
            }
            0x800be73c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be748; continue; }
                pc = 0x800be744;
            }
            0x800be744 => { ins += 1;
                c.lr = 0x800be748; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf338(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be748;
            }
            0x800be748 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be758; continue; }
                pc = 0x800be750;
            }
            0x800be750 => { ins += 1;
                c.lr = 0x800be754; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be610(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be754;
            }
            0x800be754 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800be758;
            }
            0x800be758 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be768; continue; }
                pc = 0x800be760;
            }
            0x800be760 => { ins += 1;
                c.lr = 0x800be764; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf00c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be764;
            }
            0x800be764 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800be768;
            }
            0x800be768 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be778; continue; }
                pc = 0x800be770;
            }
            0x800be770 => { ins += 1;
                c.lr = 0x800be774; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf534(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be774;
            }
            0x800be774 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800be778;
            }
            0x800be778 => { ins += 1;
                c.lr = 0x800be77c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c34a0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be77c;
            }
            0x800be77c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be78c; continue; }
                pc = 0x800be784;
            }
            0x800be784 => { ins += 1;
                c.lr = 0x800be788; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf088(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be788;
            }
            0x800be788 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800be78c;
            }
            0x800be78c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be79c; continue; }
                pc = 0x800be794;
            }
            0x800be794 => { ins += 1;
                c.lr = 0x800be798; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c31cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be798;
            }
            0x800be798 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800be79c;
            }
            0x800be79c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be7e4; continue; }
                pc = 0x800be7a4;
            }
            0x800be7a4 => { ins += 7;
                c.r[3] = 0x80480000;
                c.r[5] = 0x10000;
                c.r[6] = c.r[3].wrapping_add(0xffffa9d8);
                c.r[4] = 0x1;
                c.r[3] = c.r[5].wrapping_add(0xffffe100);
                c.r[5] = 0x0;
                c.lr = 0x800be7c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3678(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be7c0;
            }
            0x800be7c0 => { ins += 6;
                c.r[4] = 0x80480000;
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[4].wrapping_add(0xffffa9d8);
                c.r[3] = c.lwz_t(ins, c.r[3]);
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x800be7d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1300(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be7d8;
            }
            0x800be7d8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x800be7e4; continue; }
                pc = 0x800be7e0;
            }
            0x800be7e0 => { ins += 1;
                c.r[31] = c.r[30] | c.r[30];
                pc = 0x800be7e4;
            }
            0x800be7e4 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be6b4: bad pc {:#x}", pc),
        }
    }
}

/// MessageSend (0x800be800, 68 bytes)
#[inline(never)]
pub fn f_800be800(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be800) { return; }
        if c.prof { crate::prof::enter(c, 0x800be800); p = true; }
    }
    f_800be800_b(c, 0x800be800);
    if p { crate::prof::exit(c, 0x800be800); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be800_at(c: &mut Cpu, pc: u32) { f_800be800_b(c, pc) }
#[inline(always)]
fn f_800be800_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be800;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be800, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be800 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.lr = 0x800be818; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be818;
            }
            0x800be818 => { ins += 6;
                c.r[4] = 0x80270000;
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.r[4] = c.r[4].wrapping_add(0xfffff640);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800be830; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be830;
            }
            0x800be830 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be800: bad pc {:#x}", pc),
        }
    }
}

/// TRKReadBuffer_ui32 (0x800be844, 240 bytes)
#[inline(never)]
pub fn f_800be844(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be844) { return; }
        if c.prof { crate::prof::enter(c, 0x800be844); p = true; }
    }
    f_800be844_b(c, 0x800be844);
    if p { crate::prof::exit(c, 0x800be844); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be844_at(c: &mut Cpu, pc: u32) { f_800be844_b(c, pc) }
#[inline(always)]
fn f_800be844_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be844;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be844, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be844 => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[5] | c.r[5];
                c.r[31] = c.r[4] | c.r[4];
                c.r[27] = c.r[6].wrapping_add(0xffffce00);
                c.r[30] = 0x0;
                c.r[3] = 0x0;
                pc = 0x800be910; continue;
                pc = 0x800be874;
            }
            0x800be874 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[27]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800be888; continue; }
                pc = 0x800be880;
            }
            0x800be880 => { ins += 2;
                c.r[25] = c.r[31] | c.r[31];
                pc = 0x800be88c; continue;
                pc = 0x800be888;
            }
            0x800be888 => { ins += 1;
                c.r[25] = c.r[1].wrapping_add(0x8);
                pc = 0x800be88c;
            }
            0x800be88c => { ins += 7;
                c.r[3] = c.lwz_t(ins, c.r[28].wrapping_add(0xc));
                c.r[24] = 0x4;
                c.r[0] = c.lwz_t(ins, c.r[28].wrapping_add(0x8));
                c.r[26] = 0x0;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[24], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x800be8b0; continue; }
                pc = 0x800be8a8;
            }
            0x800be8a8 => { ins += 2;
                c.r[26] = 0x302;
                c.r[24] = c.r[0] | c.r[0];
                pc = 0x800be8b0;
            }
            0x800be8b0 => { ins += 5;
                c.r[4] = c.r[3].wrapping_add(0x10);
                c.r[3] = c.r[25] | c.r[25];
                c.r[5] = c.r[24] | c.r[24];
                c.r[4] = c.r[28].wrapping_add(c.r[4]);
                c.lr = 0x800be8c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be8c4;
            }
            0x800be8c4 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[28].wrapping_add(0xc));
                c.r[0] = c.r[0].wrapping_add(c.r[24]);
                c.stw(c.r[28].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[27]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be904; continue; }
                pc = 0x800be8dc;
            }
            0x800be8dc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[26], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be904; continue; }
                pc = 0x800be8e4;
            }
            0x800be8e4 => { ins += 8;
                c.r[0] = c.lbz_t(ins, c.r[25].wrapping_add(0x3));
                c.stb(c.r[31], c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[25].wrapping_add(0x2));
                c.stb(c.r[31].wrapping_add(0x1), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[25].wrapping_add(0x1));
                c.stb(c.r[31].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[25]);
                c.stb(c.r[31].wrapping_add(0x3), c.r[0]);
                pc = 0x800be904;
            }
            0x800be904 => { ins += 3;
                c.r[3] = c.r[26] | c.r[26];
                c.r[31] = c.r[31].wrapping_add(0x4);
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x800be910;
            }
            0x800be910 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be920; continue; }
                pc = 0x800be918;
            }
            0x800be918 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], c.r[29]);
                if cr_bit(cr, 0) { pc = 0x800be874; continue; }
                pc = 0x800be920;
            }
            0x800be920 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be844: bad pc {:#x}", pc),
        }
    }
}

/// TRKReadBuffer_ui8 (0x800be934, 152 bytes)
#[inline(never)]
pub fn f_800be934(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be934) { return; }
        if c.prof { crate::prof::enter(c, 0x800be934); p = true; }
    }
    f_800be934_b(c, 0x800be934);
    if p { crate::prof::exit(c, 0x800be934); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be934_at(c: &mut Cpu, pc: u32) { f_800be934_b(c, pc) }
#[inline(always)]
fn f_800be934_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be934;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be934, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be934 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[29] = 0x0;
                c.r[3] = 0x0;
                pc = 0x800be9a8; continue;
                pc = 0x800be95c;
            }
            0x800be95c => { ins += 7;
                c.r[3] = c.lwz_t(ins, c.r[26].wrapping_add(0xc));
                c.r[30] = 0x1;
                c.r[0] = c.lwz_t(ins, c.r[26].wrapping_add(0x8));
                c.r[31] = 0x0;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x800be980; continue; }
                pc = 0x800be978;
            }
            0x800be978 => { ins += 2;
                c.r[31] = 0x302;
                c.r[30] = c.r[0] | c.r[0];
                pc = 0x800be980;
            }
            0x800be980 => { ins += 5;
                c.r[4] = c.r[3].wrapping_add(0x10);
                c.r[5] = c.r[30] | c.r[30];
                c.r[3] = c.r[27].wrapping_add(c.r[29]);
                c.r[4] = c.r[26].wrapping_add(c.r[4]);
                c.lr = 0x800be994; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800be994;
            }
            0x800be994 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[26].wrapping_add(0xc));
                c.r[3] = c.r[31] | c.r[31];
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[0] = c.r[0].wrapping_add(c.r[30]);
                c.stw(c.r[26].wrapping_add(0xc), c.r[0]);
                pc = 0x800be9a8;
            }
            0x800be9a8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800be9b8; continue; }
                pc = 0x800be9b0;
            }
            0x800be9b0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x800be95c; continue; }
                pc = 0x800be9b8;
            }
            0x800be9b8 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be934: bad pc {:#x}", pc),
        }
    }
}

/// TRKReadBuffer1_ui64 (0x800be9cc, 232 bytes)
#[inline(never)]
pub fn f_800be9cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800be9cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800be9cc); p = true; }
    }
    f_800be9cc_b(c, 0x800be9cc);
    if p { crate::prof::exit(c, 0x800be9cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800be9cc_at(c: &mut Cpu, pc: u32) { f_800be9cc_b(c, pc) }
#[inline(always)]
fn f_800be9cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800be9cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800be9cc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800be9cc => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[0] = c.lwz_t(ins, c.r[5].wrapping_add(0xffffce00));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800be9fc; continue; }
                pc = 0x800be9f4;
            }
            0x800be9f4 => { ins += 2;
                c.r[31] = c.r[30] | c.r[30];
                pc = 0x800bea00; continue;
                pc = 0x800be9fc;
            }
            0x800be9fc => { ins += 1;
                c.r[31] = c.r[1].wrapping_add(0x8);
                pc = 0x800bea00;
            }
            0x800bea00 => { ins += 7;
                c.r[3] = c.lwz_t(ins, c.r[27].wrapping_add(0xc));
                c.r[28] = 0x8;
                c.r[0] = c.lwz_t(ins, c.r[27].wrapping_add(0x8));
                c.r[29] = 0x0;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x800bea24; continue; }
                pc = 0x800bea1c;
            }
            0x800bea1c => { ins += 2;
                c.r[29] = 0x302;
                c.r[28] = c.r[0] | c.r[0];
                pc = 0x800bea24;
            }
            0x800bea24 => { ins += 5;
                c.r[4] = c.r[3].wrapping_add(0x10);
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[28] | c.r[28];
                c.r[4] = c.r[27].wrapping_add(c.r[4]);
                c.lr = 0x800bea38; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bea38;
            }
            0x800bea38 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[27].wrapping_add(0xc));
                c.r[3] = 0x80400000;
                c.r[0] = c.r[0].wrapping_add(c.r[28]);
                c.stw(c.r[27].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0xffffce00));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bea9c; continue; }
                pc = 0x800bea54;
            }
            0x800bea54 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bea9c; continue; }
                pc = 0x800bea5c;
            }
            0x800bea5c => { ins += 16;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x7));
                c.stb(c.r[30], c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x6));
                c.stb(c.r[30].wrapping_add(0x1), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x5));
                c.stb(c.r[30].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x4));
                c.stb(c.r[30].wrapping_add(0x3), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x3));
                c.stb(c.r[30].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x2));
                c.stb(c.r[30].wrapping_add(0x5), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0x1));
                c.stb(c.r[30].wrapping_add(0x6), c.r[0]);
                c.r[0] = c.lbz_t(ins, c.r[31]);
                c.stb(c.r[30].wrapping_add(0x7), c.r[0]);
                pc = 0x800bea9c;
            }
            0x800bea9c => { ins += 6;
                c.r[3] = c.r[29] | c.r[29];
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800be9cc: bad pc {:#x}", pc),
        }
    }
}

/// TRKAppendBuffer_ui32 (0x800beab4, 252 bytes)
#[inline(never)]
pub fn f_800beab4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800beab4) { return; }
        if c.prof { crate::prof::enter(c, 0x800beab4); p = true; }
    }
    f_800beab4_b(c, 0x800beab4);
    if p { crate::prof::exit(c, 0x800beab4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800beab4_at(c: &mut Cpu, pc: u32) { f_800beab4_b(c, pc) }
#[inline(always)]
fn f_800beab4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800beab4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800beab4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800beab4 => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[5] | c.r[5];
                c.r[30] = c.r[4] | c.r[4];
                c.r[31] = c.r[6].wrapping_add(0xffffce00);
                c.r[29] = 0x0;
                c.r[3] = 0x0;
                pc = 0x800beb8c; continue;
                pc = 0x800beae4;
            }
            0x800beae4 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                c.r[3] = c.lwz_t(ins, c.r[30]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                if cr_bit(cr, 2) { pc = 0x800beb00; continue; }
                pc = 0x800beaf8;
            }
            0x800beaf8 => { ins += 2;
                c.r[4] = c.r[1].wrapping_add(0x8);
                pc = 0x800beb24; continue;
                pc = 0x800beb00;
            }
            0x800beb00 => { ins += 9;
                c.r[6] = c.lbz_t(ins, c.r[1].wrapping_add(0xb));
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.r[5] = c.lbz_t(ins, c.r[1].wrapping_add(0xa));
                c.r[3] = c.lbz_t(ins, c.r[1].wrapping_add(0x9));
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x8));
                c.stb(c.r[1].wrapping_add(0xc), c.r[6]);
                c.stb(c.r[1].wrapping_add(0xd), c.r[5]);
                c.stb(c.r[1].wrapping_add(0xe), c.r[3]);
                c.stb(c.r[1].wrapping_add(0xf), c.r[0]);
                pc = 0x800beb24;
            }
            0x800beb24 => { ins += 6;
                c.r[5] = c.lwz_t(ins, c.r[27].wrapping_add(0xc));
                c.r[25] = 0x4;
                c.r[26] = 0x0;
                c.r[0] = c.addc(!c.r[5], 0x880, 1);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if !cr_bit(cr, 0) { pc = 0x800beb44; continue; }
                pc = 0x800beb3c;
            }
            0x800beb3c => { ins += 2;
                c.r[26] = 0x301;
                c.r[25] = c.r[0] | c.r[0];
                pc = 0x800beb44;
            }
            0x800beb44 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[25], 0x1);
                if !cr_bit(cr, 2) { pc = 0x800beb5c; continue; }
                pc = 0x800beb4c;
            }
            0x800beb4c => { ins += 4;
                c.r[3] = c.lbz_t(ins, c.r[4]);
                c.r[0] = c.r[5].wrapping_add(0x10);
                c.stb(c.r[27].wrapping_add(c.r[0]), c.r[3]);
                pc = 0x800beb6c; continue;
                pc = 0x800beb5c;
            }
            0x800beb5c => { ins += 4;
                c.r[3] = c.r[5].wrapping_add(0x10);
                c.r[5] = c.r[25] | c.r[25];
                c.r[3] = c.r[27].wrapping_add(c.r[3]);
                c.lr = 0x800beb6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800beb6c;
            }
            0x800beb6c => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[27].wrapping_add(0xc));
                c.r[3] = c.r[26] | c.r[26];
                c.r[30] = c.r[30].wrapping_add(0x4);
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[0] = c.r[0].wrapping_add(c.r[25]);
                c.stw(c.r[27].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[27].wrapping_add(0xc));
                c.stw(c.r[27].wrapping_add(0x8), c.r[0]);
                pc = 0x800beb8c;
            }
            0x800beb8c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800beb9c; continue; }
                pc = 0x800beb94;
            }
            0x800beb94 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], c.r[28]);
                if cr_bit(cr, 0) { pc = 0x800beae4; continue; }
                pc = 0x800beb9c;
            }
            0x800beb9c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800beab4: bad pc {:#x}", pc),
        }
    }
}

/// TRKAppendBuffer_ui8 (0x800bebb0, 104 bytes)
#[inline(never)]
pub fn f_800bebb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bebb0) { return; }
        if c.prof { crate::prof::enter(c, 0x800bebb0); p = true; }
    }
    f_800bebb0_b(c, 0x800bebb0);
    if p { crate::prof::exit(c, 0x800bebb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bebb0_at(c: &mut Cpu, pc: u32) { f_800bebb0_b(c, pc) }
#[inline(always)]
fn f_800bebb0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bebb0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bebb0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bebb0 => { ins += 3;
                c.r[9] = 0x0;
                c.r[0] = 0x0;
                pc = 0x800bec00; continue;
                pc = 0x800bebbc;
            }
            0x800bebbc => { ins += 4;
                c.r[7] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                c.r[8] = c.lbz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], 0x880);
                if cr_bit(cr, 0) { pc = 0x800bebd4; continue; }
                pc = 0x800bebcc;
            }
            0x800bebcc => { ins += 2;
                c.r[7] = 0x301;
                pc = 0x800bebf4; continue;
                pc = 0x800bebd4;
            }
            0x800bebd4 => { ins += 8;
                c.r[6] = c.r[7].wrapping_add(0x1);
                c.r[0] = c.r[7].wrapping_add(0x10);
                c.stw(c.r[3].wrapping_add(0xc), c.r[6]);
                c.r[7] = 0x0;
                c.stb(c.r[3].wrapping_add(c.r[0]), c.r[8]);
                c.r[6] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                c.r[0] = c.r[6].wrapping_add(0x1);
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                pc = 0x800bebf4;
            }
            0x800bebf4 => { ins += 3;
                c.r[0] = c.r[7] | c.r[7];
                c.r[9] = c.r[9].wrapping_add(0x1);
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x800bec00;
            }
            0x800bec00 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bec10; continue; }
                pc = 0x800bec08;
            }
            0x800bec08 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[9], c.r[5]);
                if cr_bit(cr, 0) { pc = 0x800bebbc; continue; }
                pc = 0x800bec10;
            }
            0x800bec10 => { ins += 2;
                c.r[3] = c.r[0] | c.r[0];
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bebb0: bad pc {:#x}", pc),
        }
    }
}

/// TRKAppendBuffer1_ui64 (0x800bec18, 252 bytes)
#[inline(never)]
pub fn f_800bec18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bec18) { return; }
        if c.prof { crate::prof::enter(c, 0x800bec18); p = true; }
    }
    f_800bec18_b(c, 0x800bec18);
    if p { crate::prof::exit(c, 0x800bec18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bec18_at(c: &mut Cpu, pc: u32) { f_800bec18_b(c, pc) }
#[inline(always)]
fn f_800bec18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bec18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bec18, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bec18 => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0xffffce00));
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                c.stw(c.r[1].wrapping_add(0xc), c.r[6]);
                if cr_bit(cr, 2) { pc = 0x800bec54; continue; }
                pc = 0x800bec4c;
            }
            0x800bec4c => { ins += 2;
                c.r[4] = c.r[1].wrapping_add(0x8);
                pc = 0x800bec98; continue;
                pc = 0x800bec54;
            }
            0x800bec54 => { ins += 17;
                c.r[10] = c.lbz_t(ins, c.r[1].wrapping_add(0xf));
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[9] = c.lbz_t(ins, c.r[1].wrapping_add(0xe));
                c.r[8] = c.lbz_t(ins, c.r[1].wrapping_add(0xd));
                c.r[7] = c.lbz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[6] = c.lbz_t(ins, c.r[1].wrapping_add(0xb));
                c.r[5] = c.lbz_t(ins, c.r[1].wrapping_add(0xa));
                c.r[3] = c.lbz_t(ins, c.r[1].wrapping_add(0x9));
                c.r[0] = c.lbz_t(ins, c.r[1].wrapping_add(0x8));
                c.stb(c.r[1].wrapping_add(0x10), c.r[10]);
                c.stb(c.r[1].wrapping_add(0x11), c.r[9]);
                c.stb(c.r[1].wrapping_add(0x12), c.r[8]);
                c.stb(c.r[1].wrapping_add(0x13), c.r[7]);
                c.stb(c.r[1].wrapping_add(0x14), c.r[6]);
                c.stb(c.r[1].wrapping_add(0x15), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x16), c.r[3]);
                c.stb(c.r[1].wrapping_add(0x17), c.r[0]);
                pc = 0x800bec98;
            }
            0x800bec98 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                c.r[29] = 0x8;
                c.r[30] = 0x0;
                c.r[0] = c.addc(!c.r[3], 0x880, 1);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x8);
                if !cr_bit(cr, 0) { pc = 0x800becb8; continue; }
                pc = 0x800becb0;
            }
            0x800becb0 => { ins += 2;
                c.r[30] = 0x301;
                c.r[29] = c.r[0] | c.r[0];
                pc = 0x800becb8;
            }
            0x800becb8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x1);
                if !cr_bit(cr, 2) { pc = 0x800becd0; continue; }
                pc = 0x800becc0;
            }
            0x800becc0 => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[4]);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.stb(c.r[3].wrapping_add(0x10), c.r[0]);
                pc = 0x800bece0; continue;
                pc = 0x800becd0;
            }
            0x800becd0 => { ins += 4;
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                c.lr = 0x800bece0; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bece0;
            }
            0x800bece0 => { ins += 13;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.r[0].wrapping_add(c.r[29]);
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bec18: bad pc {:#x}", pc),
        }
    }
}

/// TRKReadBuffer (0x800bed14, 140 bytes)
#[inline(never)]
pub fn f_800bed14(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bed14) { return; }
        if c.prof { crate::prof::enter(c, 0x800bed14); p = true; }
    }
    f_800bed14_b(c, 0x800bed14);
    if p { crate::prof::exit(c, 0x800bed14); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bed14_at(c: &mut Cpu, pc: u32) { f_800bed14_b(c, pc) }
#[inline(always)]
fn f_800bed14_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bed14;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bed14, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bed14 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                cr_rc(&mut cr, c.xer, c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[4] | c.r[4];
                if !cr_bit(cr, 2) { pc = 0x800bed48; continue; }
                pc = 0x800bed40;
            }
            0x800bed40 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bed84; continue;
                pc = 0x800bed48;
            }
            0x800bed48 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[29].wrapping_add(0xc));
                c.r[0] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x800bed64; continue; }
                pc = 0x800bed5c;
            }
            0x800bed5c => { ins += 2;
                c.r[31] = 0x302;
                c.r[30] = c.r[0] | c.r[0];
                pc = 0x800bed64;
            }
            0x800bed64 => { ins += 4;
                c.r[4] = c.r[4].wrapping_add(0x10);
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[29].wrapping_add(c.r[4]);
                c.lr = 0x800bed74; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bed74;
            }
            0x800bed74 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[29].wrapping_add(0xc));
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.r[0].wrapping_add(c.r[30]);
                c.stw(c.r[29].wrapping_add(0xc), c.r[0]);
                pc = 0x800bed84;
            }
            0x800bed84 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bed14: bad pc {:#x}", pc),
        }
    }
}

/// TRKAppendBuffer (0x800beda0, 164 bytes)
#[inline(never)]
pub fn f_800beda0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800beda0) { return; }
        if c.prof { crate::prof::enter(c, 0x800beda0); p = true; }
    }
    f_800beda0_b(c, 0x800beda0);
    if p { crate::prof::exit(c, 0x800beda0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800beda0_at(c: &mut Cpu, pc: u32) { f_800beda0_b(c, pc) }
#[inline(always)]
fn f_800beda0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800beda0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800beda0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800beda0 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                cr_rc(&mut cr, c.xer, c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                if !cr_bit(cr, 2) { pc = 0x800bedd0; continue; }
                pc = 0x800bedc8;
            }
            0x800bedc8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bee28; continue;
                pc = 0x800bedd0;
            }
            0x800bedd0 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[29].wrapping_add(0xc));
                c.r[0] = c.addc(!c.r[3], 0x880, 1);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], c.r[30]);
                if !cr_bit(cr, 0) { pc = 0x800bede8; continue; }
                pc = 0x800bede0;
            }
            0x800bede0 => { ins += 2;
                c.r[31] = 0x301;
                c.r[30] = c.r[0] | c.r[0];
                pc = 0x800bede8;
            }
            0x800bede8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x1);
                if !cr_bit(cr, 2) { pc = 0x800bee00; continue; }
                pc = 0x800bedf0;
            }
            0x800bedf0 => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[4]);
                c.r[3] = c.r[29].wrapping_add(c.r[3]);
                c.stb(c.r[3].wrapping_add(0x10), c.r[0]);
                pc = 0x800bee10; continue;
                pc = 0x800bee00;
            }
            0x800bee00 => { ins += 4;
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.r[5] = c.r[30] | c.r[30];
                c.r[3] = c.r[29].wrapping_add(c.r[3]);
                c.lr = 0x800bee10; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bee10;
            }
            0x800bee10 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[29].wrapping_add(0xc));
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.r[0].wrapping_add(c.r[30]);
                c.stw(c.r[29].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[29].wrapping_add(0xc));
                c.stw(c.r[29].wrapping_add(0x8), c.r[0]);
                pc = 0x800bee28;
            }
            0x800bee28 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800beda0: bad pc {:#x}", pc),
        }
    }
}

/// TRKSetBufferPosition (0x800bee44, 48 bytes)
#[inline(never)]
pub fn f_800bee44(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bee44) { return; }
        if c.prof { crate::prof::enter(c, 0x800bee44); p = true; }
    }
    f_800bee44_b(c, 0x800bee44);
    if p { crate::prof::exit(c, 0x800bee44); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bee44_at(c: &mut Cpu, pc: u32) { f_800bee44_b(c, pc) }
#[inline(always)]
fn f_800bee44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bee44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bee44, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bee44 => { ins += 3;
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x880);
                c.r[5] = 0x0;
                if !cr_bit(cr, 1) { pc = 0x800bee58; continue; }
                pc = 0x800bee50;
            }
            0x800bee50 => { ins += 2;
                c.r[5] = 0x301;
                pc = 0x800bee6c; continue;
                pc = 0x800bee58;
            }
            0x800bee58 => { ins += 4;
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x800bee6c; continue; }
                pc = 0x800bee68;
            }
            0x800bee68 => { ins += 1;
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                pc = 0x800bee6c;
            }
            0x800bee6c => { ins += 2;
                c.r[3] = c.r[5] | c.r[5];
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bee44: bad pc {:#x}", pc),
        }
    }
}

/// TRKResetBuffer (0x800bee74, 64 bytes)
#[inline(never)]
pub fn f_800bee74(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bee74) { return; }
        if c.prof { crate::prof::enter(c, 0x800bee74); p = true; }
    }
    f_800bee74_b(c, 0x800bee74);
    if p { crate::prof::exit(c, 0x800bee74); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bee74_at(c: &mut Cpu, pc: u32) { f_800bee74_b(c, pc) }
#[inline(always)]
fn f_800bee74_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bee74;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bee74, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bee74 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800beea4; continue; }
                pc = 0x800bee94;
            }
            0x800bee94 => { ins += 4;
                c.r[3] = c.r[3].wrapping_add(0x10);
                c.r[4] = 0x0;
                c.r[5] = 0x880;
                c.lr = 0x800beea4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003458(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800beea4;
            }
            0x800beea4 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bee74: bad pc {:#x}", pc),
        }
    }
}

/// TRKReleaseBuffer (0x800beeb4, 100 bytes)
#[inline(never)]
pub fn f_800beeb4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800beeb4) { return; }
        if c.prof { crate::prof::enter(c, 0x800beeb4); p = true; }
    }
    f_800beeb4_b(c, 0x800beeb4);
    if p { crate::prof::exit(c, 0x800beeb4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800beeb4_at(c: &mut Cpu, pc: u32) { f_800beeb4_b(c, pc) }
#[inline(always)]
fn f_800beeb4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800beeb4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800beeb4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800beeb4 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xffffffff);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                if cr_bit(cr, 2) { pc = 0x800bef04; continue; }
                pc = 0x800beecc;
            }
            0x800beecc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 0) { pc = 0x800bef04; continue; }
                pc = 0x800beed4;
            }
            0x800beed4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x3);
                if !cr_bit(cr, 0) { pc = 0x800bef04; continue; }
                pc = 0x800beedc;
            }
            0x800beedc => { ins += 6;
                c.r[4] = c.r[3].wrapping_mul(0x890);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffce08);
                c.r[31] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800beef4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800beef4;
            }
            0x800beef4 => { ins += 4;
                c.r[0] = 0x0;
                c.r[3] = c.r[31] | c.r[31];
                c.stw(c.r[31].wrapping_add(0x4), c.r[0]);
                c.lr = 0x800bef04; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bef04;
            }
            0x800bef04 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800beeb4: bad pc {:#x}", pc),
        }
    }
}

/// TRKGetBuffer (0x800bef18, 44 bytes)
#[inline(never)]
pub fn f_800bef18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bef18) { return; }
        if c.prof { crate::prof::enter(c, 0x800bef18); p = true; }
    }
    f_800bef18_b(c, 0x800bef18);
    if p { crate::prof::exit(c, 0x800bef18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bef18_at(c: &mut Cpu, pc: u32) { f_800bef18_b(c, pc) }
#[inline(always)]
fn f_800bef18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bef18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bef18, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bef18 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.r[0] = 0x0;
                if cr_bit(cr, 0) { pc = 0x800bef3c; continue; }
                pc = 0x800bef24;
            }
            0x800bef24 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x3);
                if !cr_bit(cr, 0) { pc = 0x800bef3c; continue; }
                pc = 0x800bef2c;
            }
            0x800bef2c => { ins += 4;
                c.r[4] = c.r[3].wrapping_mul(0x890);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffce08);
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x800bef3c;
            }
            0x800bef3c => { ins += 2;
                c.r[3] = c.r[0] | c.r[0];
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bef18: bad pc {:#x}", pc),
        }
    }
}

/// TRKGetFreeBuffer (0x800bef44, 200 bytes)
#[inline(never)]
pub fn f_800bef44(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bef44) { return; }
        if c.prof { crate::prof::enter(c, 0x800bef44); p = true; }
    }
    f_800bef44_b(c, 0x800bef44);
    if p { crate::prof::exit(c, 0x800bef44); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bef44_at(c: &mut Cpu, pc: u32) { f_800bef44_b(c, pc) }
#[inline(always)]
fn f_800bef44_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bef44;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bef44, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bef44 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[4] | c.r[4];
                c.r[27] = c.r[3] | c.r[3];
                c.r[30] = 0x300;
                c.r[29] = 0x0;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800befd8; continue;
                pc = 0x800bef70;
            }
            0x800bef70 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                c.r[31] = 0x0;
                if cr_bit(cr, 0) { pc = 0x800bef94; continue; }
                pc = 0x800bef7c;
            }
            0x800bef7c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x3);
                if !cr_bit(cr, 0) { pc = 0x800bef94; continue; }
                pc = 0x800bef84;
            }
            0x800bef84 => { ins += 4;
                c.r[4] = c.r[29].wrapping_mul(0x890);
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xffffce08);
                c.r[31] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x800bef94;
            }
            0x800bef94 => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800bef9c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bef9c;
            }
            0x800bef9c => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800befcc; continue; }
                pc = 0x800befa8;
            }
            0x800befa8 => { ins += 9;
                c.r[3] = 0x0;
                c.r[0] = 0x1;
                c.stw(c.r[31].wrapping_add(0x8), c.r[3]);
                c.r[30] = 0x0;
                c.stw(c.r[31].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[31].wrapping_add(0x4), c.r[0]);
                c.stw(c.r[28], c.r[31]);
                c.stw(c.r[27], c.r[29]);
                c.r[29] = 0x3;
                pc = 0x800befcc;
            }
            0x800befcc => { ins += 2;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800befd4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800befd4;
            }
            0x800befd4 => { ins += 1;
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x800befd8;
            }
            0x800befd8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x3);
                if cr_bit(cr, 0) { pc = 0x800bef70; continue; }
                pc = 0x800befe0;
            }
            0x800befe0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x300);
                if !cr_bit(cr, 2) { pc = 0x800beff4; continue; }
                pc = 0x800befe8;
            }
            0x800befe8 => { ins += 3;
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0xfffff668);
                c.lr = 0x800beff4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf33c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800beff4;
            }
            0x800beff4 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bef44: bad pc {:#x}", pc),
        }
    }
}

/// TRKInitializeMessageBuffers (0x800bf00c, 116 bytes)
#[inline(never)]
pub fn f_800bf00c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf00c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf00c); p = true; }
    }
    f_800bf00c_b(c, 0x800bf00c);
    if p { crate::prof::exit(c, 0x800bf00c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf00c_at(c: &mut Cpu, pc: u32) { f_800bf00c_b(c, pc) }
#[inline(always)]
fn f_800bf00c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf00c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf00c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf00c => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0xffffce08);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = 0x0;
                pc = 0x800bf034;
            }
            0x800bf034 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800bf03c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cd0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf03c;
            }
            0x800bf03c => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800bf044; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf044;
            }
            0x800bf044 => { ins += 3;
                c.stw(c.r[30].wrapping_add(0x4), c.r[31]);
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800bf050; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cc0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf050;
            }
            0x800bf050 => { ins += 4;
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[30] = c.r[30].wrapping_add(0x890);
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x3);
                if cr_bit(cr, 0) { pc = 0x800bf034; continue; }
                pc = 0x800bf060;
            }
            0x800bf060 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf00c: bad pc {:#x}", pc),
        }
    }
}

/// TRKTerminateSerialHandler (0x800bf080, 8 bytes)
#[inline(never)]
pub fn f_800bf080(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf080) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf080); p = true; }
    }
    f_800bf080_b(c, 0x800bf080);
    if p { crate::prof::exit(c, 0x800bf080); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf080_at(c: &mut Cpu, pc: u32) { f_800bf080_b(c, pc) }
#[inline(always)]
fn f_800bf080_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf080;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf080, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf080 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf080: bad pc {:#x}", pc),
        }
    }
}

/// TRKInitializeSerialHandler (0x800bf088, 196 bytes)
#[inline(never)]
pub fn f_800bf088(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf088) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf088); p = true; }
    }
    f_800bf088_b(c, 0x800bf088);
    if p { crate::prof::exit(c, 0x800bf088); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf088_at(c: &mut Cpu, pc: u32) { f_800bf088_b(c, pc) }
#[inline(always)]
fn f_800bf088_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf088;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf088, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf088 => { ins += 18;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.r[4] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.r[3].wrapping_add(0xffffe7b8);
                c.r[0] = 0x0;
                c.r[3] = 0xffffffff;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xfffff688);
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[5] = 0x40;
                c.stw(c.r[6], c.r[3]);
                c.r[3] = 0x1;
                c.stw(c.r[6].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[6].wrapping_add(0xc), c.r[0]);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf0d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf0d0;
            }
            0x800bf0d0 => { ins += 5;
                c.r[4] = c.r[31].wrapping_add(0x24);
                c.r[3] = 0x1;
                c.r[5] = 0x40;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf0e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf0e4;
            }
            0x800bf0e4 => { ins += 5;
                c.r[4] = c.r[31].wrapping_add(0x48);
                c.r[3] = 0x1;
                c.r[5] = 0x40;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf0f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf0f8;
            }
            0x800bf0f8 => { ins += 5;
                c.r[4] = c.r[31].wrapping_add(0x6c);
                c.r[3] = 0x1;
                c.r[5] = 0x40;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf10c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf10c;
            }
            0x800bf10c => { ins += 5;
                c.r[4] = c.r[31].wrapping_add(0x8c);
                c.r[3] = 0x1;
                c.r[5] = 0x40;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf120; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf120;
            }
            0x800bf120 => { ins += 5;
                c.r[4] = c.r[31].wrapping_add(0xac);
                c.r[3] = 0x1;
                c.r[5] = 0x40;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf134; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf134;
            }
            0x800bf134 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf088: bad pc {:#x}", pc),
        }
    }
}

/// TRKProcessInput (0x800bf14c, 80 bytes)
#[inline(never)]
pub fn f_800bf14c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf14c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf14c); p = true; }
    }
    f_800bf14c_b(c, 0x800bf14c);
    if p { crate::prof::exit(c, 0x800bf14c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf14c_at(c: &mut Cpu, pc: u32) { f_800bf14c_b(c, pc) }
#[inline(always)]
fn f_800bf14c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf14c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf14c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf14c => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x2;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800bf16c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be464(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf16c;
            }
            0x800bf16c => { ins += 7;
                c.r[3] = 0x80400000;
                c.r[0] = 0xffffffff;
                c.r[4] = c.r[3].wrapping_add(0xffffe7b8);
                c.stw(c.r[1].wrapping_add(0x10), c.r[31]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[4], c.r[0]);
                c.lr = 0x800bf188; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be47c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf188;
            }
            0x800bf188 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf14c: bad pc {:#x}", pc),
        }
    }
}

/// TRKGetInput (0x800bf19c, 96 bytes)
#[inline(never)]
pub fn f_800bf19c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf19c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf19c); p = true; }
    }
    f_800bf19c_b(c, 0x800bf19c);
    if p { crate::prof::exit(c, 0x800bf19c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf19c_at(c: &mut Cpu, pc: u32) { f_800bf19c_b(c, pc) }
#[inline(always)]
fn f_800bf19c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf19c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf19c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf19c => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.lr = 0x800bf1b0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf1fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf1b0;
            }
            0x800bf1b0 => { ins += 3;
                c.r[31] = c.r[3] | c.r[3];
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0xffffffff);
                if cr_bit(cr, 2) { pc = 0x800bf1e8; continue; }
                pc = 0x800bf1bc;
            }
            0x800bf1bc => { ins += 1;
                c.lr = 0x800bf1c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf1c0;
            }
            0x800bf1c0 => { ins += 3;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x2;
                c.lr = 0x800bf1cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be464(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf1cc;
            }
            0x800bf1cc => { ins += 7;
                c.r[3] = 0x80400000;
                c.r[0] = 0xffffffff;
                c.r[4] = c.r[3].wrapping_add(0xffffe7b8);
                c.stw(c.r[1].wrapping_add(0x10), c.r[31]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[4], c.r[0]);
                c.lr = 0x800bf1e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be47c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf1e8;
            }
            0x800bf1e8 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf19c: bad pc {:#x}", pc),
        }
    }
}

/// TRKTestForPacket (0x800bf1fc, 316 bytes)
#[inline(never)]
pub fn f_800bf1fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf1fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf1fc); p = true; }
    }
    f_800bf1fc_b(c, 0x800bf1fc);
    if p { crate::prof::exit(c, 0x800bf1fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf1fc_at(c: &mut Cpu, pc: u32) { f_800bf1fc_b(c, pc) }
#[inline(always)]
fn f_800bf1fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf1fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf1fc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf1fc => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffff720); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x8e4), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x8dc), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0xfffff688);
                c.stw(c.r[1].wrapping_add(0x8d8), c.r[30]);
                c.lr = 0x800bf21c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3600(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf21c;
            }
            0x800bf21c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 1) { pc = 0x800bf22c; continue; }
                pc = 0x800bf224;
            }
            0x800bf224 => { ins += 2;
                c.r[3] = 0xffffffff;
                pc = 0x800bf320; continue;
                pc = 0x800bf22c;
            }
            0x800bf22c => { ins += 3;
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800bf238; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf238;
            }
            0x800bf238 => { ins += 6;
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[31].wrapping_add(0xd0);
                c.r[3] = 0x4;
                c.r[5] = c.r[30] | c.r[30];
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf250; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf250;
            }
            0x800bf250 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = 0x0;
                c.lr = 0x800bf25c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf25c;
            }
            0x800bf25c => { ins += 3;
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.r[4] = 0x40;
                c.lr = 0x800bf268; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c35c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf268;
            }
            0x800bf268 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bf2ec; continue; }
                pc = 0x800bf270;
            }
            0x800bf270 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[5] = 0x40;
                c.lr = 0x800bf280; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf280;
            }
            0x800bf280 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[5] = c.addc(c.r[3], 0xffffffc0, 0);
                cr_rc(&mut cr, c.xer, c.r[5]);
                if !cr_bit(cr, 1) { pc = 0x800bf308; continue; }
                pc = 0x800bf290;
            }
            0x800bf290 => { ins += 4;
                c.r[4] = c.r[31].wrapping_add(0xf4);
                c.r[3] = 0x1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf2a0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf2a0;
            }
            0x800bf2a0 => { ins += 4;
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[3] = c.r[1].wrapping_add(0x50);
                c.r[4] = c.r[4].wrapping_add(0xffffffc0);
                c.lr = 0x800bf2b0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c35c4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf2b0;
            }
            0x800bf2b0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bf2cc; continue; }
                pc = 0x800bf2b8;
            }
            0x800bf2b8 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[1].wrapping_add(0x50);
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800bf2c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf2c8;
            }
            0x800bf2c8 => { ins += 1;
                pc = 0x800bf308; continue;
                pc = 0x800bf2cc;
            }
            0x800bf2cc => { ins += 4;
                c.r[4] = c.r[31].wrapping_add(0x110);
                c.r[3] = 0x8;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf2dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf2dc;
            }
            0x800bf2dc => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800bf2e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf2e4;
            }
            0x800bf2e4 => { ins += 2;
                c.r[30] = 0xffffffff;
                pc = 0x800bf308; continue;
                pc = 0x800bf2ec;
            }
            0x800bf2ec => { ins += 4;
                c.r[4] = c.r[31].wrapping_add(0x144);
                c.r[3] = 0x8;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf2fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf2fc;
            }
            0x800bf2fc => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800bf304; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf304;
            }
            0x800bf304 => { ins += 1;
                c.r[30] = 0xffffffff;
                pc = 0x800bf308;
            }
            0x800bf308 => { ins += 5;
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].wrapping_add(0x16c);
                c.r[3] = 0x1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf31c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf31c;
            }
            0x800bf31c => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf320;
            }
            0x800bf320 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x8e4));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x8dc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8d8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x8e0);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf1fc: bad pc {:#x}", pc),
        }
    }
}

/// usr_put_initialize (0x800bf338, 4 bytes)
#[inline(never)]
pub fn f_800bf338(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf338) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf338); p = true; }
    }
    f_800bf338_b(c, 0x800bf338);
    if p { crate::prof::exit(c, 0x800bf338); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf338_at(c: &mut Cpu, pc: u32) { f_800bf338_b(c, pc) }
#[inline(always)]
fn f_800bf338_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf338;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf338, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf338 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf338: bad pc {:#x}", pc),
        }
    }
}

/// usr_puts_serial (0x800bf33c, 136 bytes)
#[inline(never)]
pub fn f_800bf33c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf33c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf33c); p = true; }
    }
    f_800bf33c_b(c, 0x800bf33c);
    if p { crate::prof::exit(c, 0x800bf33c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf33c_at(c: &mut Cpu, pc: u32) { f_800bf33c_b(c, pc) }
#[inline(always)]
fn f_800bf33c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf33c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf33c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf33c => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                pc = 0x800bf390; continue;
                pc = 0x800bf364;
            }
            0x800bf364 => { ins += 1;
                c.lr = 0x800bf368; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c04f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf368;
            }
            0x800bf368 => { ins += 5;
                c.stb(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.stb(c.r[1].wrapping_add(0x9), c.r[31]);
                c.lr = 0x800bf37c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c04e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf37c;
            }
            0x800bf37c => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800bf384; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009c2e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf384;
            }
            0x800bf384 => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800bf38c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c04e8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf38c;
            }
            0x800bf38c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x800bf390;
            }
            0x800bf390 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bf3a8; continue; }
                pc = 0x800bf398;
            }
            0x800bf398 => { ins += 4;
                c.r[0] = c.lbz_t(ins, c.r[29]);
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[30] = c.r[0] as u8 as i8 as i32 as u32;
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x800bf364; continue; }
                pc = 0x800bf3a8;
            }
            0x800bf3a8 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf33c: bad pc {:#x}", pc),
        }
    }
}

/// TRKDispatchMessage (0x800bf3c4, 368 bytes)
#[inline(never)]
pub fn f_800bf3c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf3c4) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf3c4); p = true; }
    }
    f_800bf3c4_b(c, 0x800bf3c4);
    if p { crate::prof::exit(c, 0x800bf3c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf3c4_at(c: &mut Cpu, pc: u32) { f_800bf3c4_b(c, pc) }
#[inline(never)]
fn f_800bf3c4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf3c4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf3c4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf3c4 => { ins += 1;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800bf3c8;
            }
            0x800bf3c8 => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x800bf3cc;
            }
            0x800bf3cc => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800bf3d0;
            }
            0x800bf3d0 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                pc = 0x800bf3d4;
            }
            0x800bf3d4 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                pc = 0x800bf3d8;
            }
            0x800bf3d8 => { ins += 1;
                c.r[31] = 0x500;
                pc = 0x800bf3dc;
            }
            0x800bf3dc => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                pc = 0x800bf3e0;
            }
            0x800bf3e0 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800bf3e4;
            }
            0x800bf3e4 => { ins += 1;
                c.lr = 0x800bf3e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf3e8;
            }
            0x800bf3e8 => { ins += 1;
                c.r[3] = 0x80270000;
                pc = 0x800bf3ec;
            }
            0x800bf3ec => { ins += 1;
                c.r[5] = c.lbz_t(ins, c.r[30].wrapping_add(0x14));
                pc = 0x800bf3f0;
            }
            0x800bf3f0 => { ins += 1;
                c.r[4] = c.r[3].wrapping_add(0xfffff818);
                pc = 0x800bf3f4;
            }
            0x800bf3f4 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x800bf3f8;
            }
            0x800bf3f8 => { ins += 1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                pc = 0x800bf3fc;
            }
            0x800bf3fc => { ins += 1;
                c.lr = 0x800bf400; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf400;
            }
            0x800bf400 => { ins += 1;
                c.r[0] = c.lbz_t(ins, c.r[30].wrapping_add(0x14));
                pc = 0x800bf404;
            }
            0x800bf404 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1a);
                pc = 0x800bf408;
            }
            0x800bf408 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x800bf500; continue; }
                pc = 0x800bf40c;
            }
            0x800bf40c => { ins += 1;
                c.r[3] = 0x80310000;
                pc = 0x800bf410;
            }
            0x800bf410 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800bf414;
            }
            0x800bf414 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x3770);
                pc = 0x800bf418;
            }
            0x800bf418 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[0]));
                pc = 0x800bf41c;
            }
            0x800bf41c => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x800bf420;
            }
            0x800bf420 => { ins += 1;
                if true { let t = c.ctr; if (0x800bf3c4..0x800bf534).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800bf424;
            }
            0x800bf424 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf428;
            }
            0x800bf428 => { ins += 1;
                c.lr = 0x800bf42c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0484(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf42c;
            }
            0x800bf42c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf430;
            }
            0x800bf430 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf434;
            }
            0x800bf434 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf438;
            }
            0x800bf438 => { ins += 1;
                c.lr = 0x800bf43c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c040c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf43c;
            }
            0x800bf43c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf440;
            }
            0x800bf440 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf444;
            }
            0x800bf444 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf448;
            }
            0x800bf448 => { ins += 1;
                c.lr = 0x800bf44c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c03b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf44c;
            }
            0x800bf44c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf450;
            }
            0x800bf450 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf454;
            }
            0x800bf454 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf458;
            }
            0x800bf458 => { ins += 1;
                c.lr = 0x800bf45c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c035c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf45c;
            }
            0x800bf45c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf460;
            }
            0x800bf460 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf464;
            }
            0x800bf464 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf468;
            }
            0x800bf468 => { ins += 1;
                c.lr = 0x800bf46c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0354(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf46c;
            }
            0x800bf46c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf470;
            }
            0x800bf470 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf474;
            }
            0x800bf474 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf478;
            }
            0x800bf478 => { ins += 1;
                c.lr = 0x800bf47c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c034c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf47c;
            }
            0x800bf47c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf480;
            }
            0x800bf480 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf484;
            }
            0x800bf484 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf488;
            }
            0x800bf488 => { ins += 1;
                c.lr = 0x800bf48c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0108(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf48c;
            }
            0x800bf48c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf490;
            }
            0x800bf490 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf494;
            }
            0x800bf494 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf498;
            }
            0x800bf498 => { ins += 1;
                c.lr = 0x800bf49c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bfecc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf49c;
            }
            0x800bf49c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf4a0;
            }
            0x800bf4a0 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf4a4;
            }
            0x800bf4a4 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf4a8;
            }
            0x800bf4a8 => { ins += 1;
                c.lr = 0x800bf4ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bfbec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf4ac;
            }
            0x800bf4ac => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf4b0;
            }
            0x800bf4b0 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf4b4;
            }
            0x800bf4b4 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf4b8;
            }
            0x800bf4b8 => { ins += 1;
                c.lr = 0x800bf4bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf95c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf4bc;
            }
            0x800bf4bc => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf4c0;
            }
            0x800bf4c0 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf4c4;
            }
            0x800bf4c4 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf4c8;
            }
            0x800bf4c8 => { ins += 1;
                c.lr = 0x800bf4cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf8ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf4cc;
            }
            0x800bf4cc => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf4d0;
            }
            0x800bf4d0 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf4d4;
            }
            0x800bf4d4 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf4d8;
            }
            0x800bf4d8 => { ins += 1;
                c.lr = 0x800bf4dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf68c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf4dc;
            }
            0x800bf4dc => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf4e0;
            }
            0x800bf4e0 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf4e4;
            }
            0x800bf4e4 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf4e8;
            }
            0x800bf4e8 => { ins += 1;
                c.lr = 0x800bf4ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf5e4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf4ec;
            }
            0x800bf4ec => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf4f0;
            }
            0x800bf4f0 => { ins += 1;
                pc = 0x800bf500; continue;
                pc = 0x800bf4f4;
            }
            0x800bf4f4 => { ins += 1;
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800bf4f8;
            }
            0x800bf4f8 => { ins += 1;
                c.lr = 0x800bf4fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf53c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf4fc;
            }
            0x800bf4fc => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bf500;
            }
            0x800bf500 => { ins += 1;
                c.r[3] = 0x80270000;
                pc = 0x800bf504;
            }
            0x800bf504 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x800bf508;
            }
            0x800bf508 => { ins += 1;
                c.r[4] = c.r[3].wrapping_add(0xfffff834);
                pc = 0x800bf50c;
            }
            0x800bf50c => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x800bf510;
            }
            0x800bf510 => { ins += 1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                pc = 0x800bf514;
            }
            0x800bf514 => { ins += 1;
                c.lr = 0x800bf518; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf518;
            }
            0x800bf518 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                pc = 0x800bf51c;
            }
            0x800bf51c => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800bf520;
            }
            0x800bf520 => { ins += 1;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                pc = 0x800bf524;
            }
            0x800bf524 => { ins += 1;
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                pc = 0x800bf528;
            }
            0x800bf528 => { ins += 1;
                c.mtspr(8, c.r[0]);
                pc = 0x800bf52c;
            }
            0x800bf52c => { ins += 1;
                c.r[1] = c.r[1].wrapping_add(0x10);
                pc = 0x800bf530;
            }
            0x800bf530 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf3c4: bad pc {:#x}", pc),
        }
    }
}

/// TRKInitializeDispatcher (0x800bf534, 8 bytes)
#[inline(never)]
pub fn f_800bf534(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf534) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf534); p = true; }
    }
    f_800bf534_b(c, 0x800bf534);
    if p { crate::prof::exit(c, 0x800bf534); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf534_at(c: &mut Cpu, pc: u32) { f_800bf534_b(c, pc) }
#[inline(always)]
fn f_800bf534_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf534;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf534, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf534 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf534: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoSetOption (0x800bf53c, 168 bytes)
#[inline(never)]
pub fn f_800bf53c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf53c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf53c); p = true; }
    }
    f_800bf53c_b(c, 0x800bf53c);
    if p { crate::prof::exit(c, 0x800bf53c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf53c_at(c: &mut Cpu, pc: u32) { f_800bf53c_b(c, pc) }
#[inline(always)]
fn f_800bf53c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf53c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf53c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf53c => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x4c), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xfffff858);
                c.stw(c.r[1].wrapping_add(0x48), c.r[30]);
                c.r[0] = c.lbz_t(ins, c.r[3].wrapping_add(0x18));
                c.r[30] = c.lbz_t(ins, c.r[3].wrapping_add(0x1c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x1);
                if !cr_bit(cr, 2) { pc = 0x800bf594; continue; }
                pc = 0x800bf568;
            }
            0x800bf568 => { ins += 2;
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.lr = 0x800bf570; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf33c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf570;
            }
            0x800bf570 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bf584; continue; }
                pc = 0x800bf578;
            }
            0x800bf578 => { ins += 2;
                c.r[3] = c.r[31].wrapping_add(0x20);
                c.lr = 0x800bf580; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf33c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf580;
            }
            0x800bf580 => { ins += 1;
                pc = 0x800bf58c; continue;
                pc = 0x800bf584;
            }
            0x800bf584 => { ins += 2;
                c.r[3] = c.r[31].wrapping_add(0x28);
                c.lr = 0x800bf58c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf33c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf58c;
            }
            0x800bf58c => { ins += 2;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800bf594; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c39b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf594;
            }
            0x800bf594 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bf5a4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf5a4;
            }
            0x800bf5a4 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x800bf5c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf5c8;
            }
            0x800bf5c8 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.r[3] = 0x0;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x4c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x48));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf53c: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoStop (0x800bf5e4, 168 bytes)
#[inline(never)]
pub fn f_800bf5e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf5e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf5e4); p = true; }
    }
    f_800bf5e4_b(c, 0x800bf5e4);
    if p { crate::prof::exit(c, 0x800bf5e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf5e4_at(c: &mut Cpu, pc: u32) { f_800bf5e4_b(c, pc) }
#[inline(always)]
fn f_800bf5e4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf5e4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf5e4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf5e4 => { ins += 5;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x4c), c.r[31]);
                c.lr = 0x800bf5f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1310(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf5f8;
            }
            0x800bf5f8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x704);
                if cr_bit(cr, 2) { pc = 0x800bf628; continue; }
                pc = 0x800bf600;
            }
            0x800bf600 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bf610; continue; }
                pc = 0x800bf604;
            }
            0x800bf604 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bf620; continue; }
                pc = 0x800bf60c;
            }
            0x800bf60c => { ins += 1;
                pc = 0x800bf640; continue;
                pc = 0x800bf610;
            }
            0x800bf610 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x706);
                if cr_bit(cr, 2) { pc = 0x800bf638; continue; }
                pc = 0x800bf618;
            }
            0x800bf618 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bf640; continue; }
                pc = 0x800bf61c;
            }
            0x800bf61c => { ins += 1;
                pc = 0x800bf630; continue;
                pc = 0x800bf620;
            }
            0x800bf620 => { ins += 2;
                c.r[31] = 0x0;
                pc = 0x800bf644; continue;
                pc = 0x800bf628;
            }
            0x800bf628 => { ins += 2;
                c.r[31] = 0x21;
                pc = 0x800bf644; continue;
                pc = 0x800bf630;
            }
            0x800bf630 => { ins += 2;
                c.r[31] = 0x22;
                pc = 0x800bf644; continue;
                pc = 0x800bf638;
            }
            0x800bf638 => { ins += 2;
                c.r[31] = 0x20;
                pc = 0x800bf644; continue;
                pc = 0x800bf640;
            }
            0x800bf640 => { ins += 1;
                c.r[31] = 0x1;
                pc = 0x800bf644;
            }
            0x800bf644 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bf654; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf654;
            }
            0x800bf654 => { ins += 8;
                c.r[3] = 0x80;
                c.r[0] = 0x40;
                c.stb(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x10), c.r[31]);
                c.lr = 0x800bf674; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf674;
            }
            0x800bf674 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.r[3] = 0x0;
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x4c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf5e4: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoStep (0x800bf68c, 544 bytes)
#[inline(never)]
pub fn f_800bf68c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf68c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf68c); p = true; }
    }
    f_800bf68c_b(c, 0x800bf68c);
    if p { crate::prof::exit(c, 0x800bf68c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf68c_at(c: &mut Cpu, pc: u32) { f_800bf68c_b(c, pc) }
#[inline(never)]
fn f_800bf68c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf68c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf68c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf68c => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xfffffea0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x164), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x800bf6a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf6a8;
            }
            0x800bf6a8 => { ins += 5;
                c.r[31] = c.lbz_t(ins, c.r[27].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[27].wrapping_add(0x20));
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x10);
                c.r[28] = c.lwz_t(ins, c.r[27].wrapping_add(0x24));
                if cr_bit(cr, 2) { pc = 0x800bf6e4; continue; }
                pc = 0x800bf6bc;
            }
            0x800bf6bc => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bf6d8; continue; }
                pc = 0x800bf6c0;
            }
            0x800bf6c0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x1);
                if cr_bit(cr, 2) { pc = 0x800bf72c; continue; }
                pc = 0x800bf6c8;
            }
            0x800bf6c8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bf77c; continue; }
                pc = 0x800bf6cc;
            }
            0x800bf6cc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 0) { pc = 0x800bf6e4; continue; }
                pc = 0x800bf6d4;
            }
            0x800bf6d4 => { ins += 1;
                pc = 0x800bf77c; continue;
                pc = 0x800bf6d8;
            }
            0x800bf6d8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x12);
                if !cr_bit(cr, 0) { pc = 0x800bf77c; continue; }
                pc = 0x800bf6e0;
            }
            0x800bf6e0 => { ins += 1;
                pc = 0x800bf72c; continue;
                pc = 0x800bf6e4;
            }
            0x800bf6e4 => { ins += 3;
                c.r[30] = c.lbz_t(ins, c.r[27].wrapping_add(0x1c));
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], 0x1);
                if !cr_bit(cr, 0) { pc = 0x800bf7b8; continue; }
                pc = 0x800bf6f0;
            }
            0x800bf6f0 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x108);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bf700; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf700;
            }
            0x800bf700 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x11;
                c.stb(c.r[1].wrapping_add(0x10c), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x108);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x108), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x110), c.r[0]);
                c.lr = 0x800bf724; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf724;
            }
            0x800bf724 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bf898; continue;
                pc = 0x800bf72c;
            }
            0x800bf72c => { ins += 1;
                c.lr = 0x800bf730; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1548(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf730;
            }
            0x800bf730 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[29]);
                if cr_bit(cr, 0) { pc = 0x800bf740; continue; }
                pc = 0x800bf738;
            }
            0x800bf738 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[28]);
                if !cr_bit(cr, 1) { pc = 0x800bf7b8; continue; }
                pc = 0x800bf740;
            }
            0x800bf740 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0xc8);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bf750; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf750;
            }
            0x800bf750 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x11;
                c.stb(c.r[1].wrapping_add(0xcc), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0xc8);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0xc8), c.r[5]);
                c.stb(c.r[1].wrapping_add(0xd0), c.r[0]);
                c.lr = 0x800bf774; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf774;
            }
            0x800bf774 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bf898; continue;
                pc = 0x800bf77c;
            }
            0x800bf77c => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x88);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bf78c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf78c;
            }
            0x800bf78c => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x12;
                c.stb(c.r[1].wrapping_add(0x8c), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x88);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x88), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x90), c.r[0]);
                c.lr = 0x800bf7b0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf7b0;
            }
            0x800bf7b0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bf898; continue;
                pc = 0x800bf7b8;
            }
            0x800bf7b8 => { ins += 1;
                c.lr = 0x800bf7bc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1338(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf7bc;
            }
            0x800bf7bc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bf800; continue; }
                pc = 0x800bf7c4;
            }
            0x800bf7c4 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x48);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bf7d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf7d4;
            }
            0x800bf7d4 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x16;
                c.stb(c.r[1].wrapping_add(0x4c), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x48);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x48), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x50), c.r[0]);
                c.lr = 0x800bf7f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf7f8;
            }
            0x800bf7f8 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bf898; continue;
                pc = 0x800bf800;
            }
            0x800bf800 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bf810; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf810;
            }
            0x800bf810 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x800bf834; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf834;
            }
            0x800bf834 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x10);
                c.r[3] = 0x0;
                if cr_bit(cr, 2) { pc = 0x800bf868; continue; }
                pc = 0x800bf840;
            }
            0x800bf840 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bf85c; continue; }
                pc = 0x800bf844;
            }
            0x800bf844 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x1);
                if cr_bit(cr, 2) { pc = 0x800bf880; continue; }
                pc = 0x800bf84c;
            }
            0x800bf84c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bf898; continue; }
                pc = 0x800bf850;
            }
            0x800bf850 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 0) { pc = 0x800bf868; continue; }
                pc = 0x800bf858;
            }
            0x800bf858 => { ins += 1;
                pc = 0x800bf898; continue;
                pc = 0x800bf85c;
            }
            0x800bf85c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x12);
                if !cr_bit(cr, 0) { pc = 0x800bf898; continue; }
                pc = 0x800bf864;
            }
            0x800bf864 => { ins += 1;
                pc = 0x800bf880; continue;
                pc = 0x800bf868;
            }
            0x800bf868 => { ins += 5;
                c.r[0] = c.addc(!c.r[31], 0x10, 1);
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.r[0].leading_zeros();
                c.r[4] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.lr = 0x800bf87c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1610(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf87c;
            }
            0x800bf87c => { ins += 1;
                pc = 0x800bf898; continue;
                pc = 0x800bf880;
            }
            0x800bf880 => { ins += 6;
                c.r[0] = c.addc(!c.r[31], 0x11, 1);
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.r[0].leading_zeros();
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.lr = 0x800bf898; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1558(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf898;
            }
            0x800bf898 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x14c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x164));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x160);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf68c: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoContinue (0x800bf8ac, 176 bytes)
#[inline(never)]
pub fn f_800bf8ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf8ac) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf8ac); p = true; }
    }
    f_800bf8ac_b(c, 0x800bf8ac);
    if p { crate::prof::exit(c, 0x800bf8ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf8ac_at(c: &mut Cpu, pc: u32) { f_800bf8ac_b(c, pc) }
#[inline(always)]
fn f_800bf8ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf8ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf8ac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf8ac => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffff70); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80270000;
                c.r[3] = 0x1;
                c.stw(c.r[1].wrapping_add(0x94), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0xfffff88c);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bf8cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf8cc;
            }
            0x800bf8cc => { ins += 1;
                c.lr = 0x800bf8d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1338(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf8d0;
            }
            0x800bf8d0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bf914; continue; }
                pc = 0x800bf8d8;
            }
            0x800bf8d8 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x48);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bf8e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf8e8;
            }
            0x800bf8e8 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x16;
                c.stb(c.r[1].wrapping_add(0x4c), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x48);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x48), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x50), c.r[0]);
                c.lr = 0x800bf90c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf90c;
            }
            0x800bf90c => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bf94c; continue;
                pc = 0x800bf914;
            }
            0x800bf914 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bf924; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf924;
            }
            0x800bf924 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x800bf948; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf948;
            }
            0x800bf948 => { ins += 1;
                c.lr = 0x800bf94c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c396c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf94c;
            }
            0x800bf94c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x94));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x90);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf8ac: bad pc {:#x}", pc),
        }
    }
}

/// SendACK (0x800bf95c, 656 bytes)
#[inline(never)]
pub fn f_800bf95c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bf95c) { return; }
        if c.prof { crate::prof::enter(c, 0x800bf95c); p = true; }
    }
    f_800bf95c_b(c, 0x800bf95c);
    if p { crate::prof::exit(c, 0x800bf95c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bf95c_at(c: &mut Cpu, pc: u32) { f_800bf95c_b(c, pc) }
#[inline(never)]
fn f_800bf95c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bf95c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bf95c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bf95c => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xffffff20); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0xe4), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xdc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0xd8), c.r[30]);
                c.stw(c.r[1].wrapping_add(0xd4), c.r[29]);
                c.stw(c.r[1].wrapping_add(0xd0), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[31] = c.lbz_t(ins, c.r[3].wrapping_add(0x18));
                c.r[30] = c.lhz_t(ins, c.r[3].wrapping_add(0x1c));
                c.r[29] = c.lhz_t(ins, c.r[3].wrapping_add(0x20));
                c.lr = 0x800bf990; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf990;
            }
            0x800bf990 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[29]);
                if !cr_bit(cr, 1) { pc = 0x800bf9d4; continue; }
                pc = 0x800bf998;
            }
            0x800bf998 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x4c);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bf9a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf9a8;
            }
            0x800bf9a8 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x14;
                c.stb(c.r[1].wrapping_add(0x50), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x4c);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x4c), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x54), c.r[0]);
                c.lr = 0x800bf9cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf9cc;
            }
            0x800bf9cc => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bfbcc; continue;
                pc = 0x800bf9d4;
            }
            0x800bf9d4 => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x40;
                c.lr = 0x800bf9e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bf9e0;
            }
            0x800bf9e0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x2);
                if cr_bit(cr, 2) { pc = 0x800bfa48; continue; }
                pc = 0x800bf9e8;
            }
            0x800bf9e8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bf9fc; continue; }
                pc = 0x800bf9ec;
            }
            0x800bf9ec => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bfa08; continue; }
                pc = 0x800bf9f4;
            }
            0x800bf9f4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bfa28; continue; }
                pc = 0x800bf9f8;
            }
            0x800bf9f8 => { ins += 1;
                pc = 0x800bfa88; continue;
                pc = 0x800bf9fc;
            }
            0x800bf9fc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x4);
                if !cr_bit(cr, 0) { pc = 0x800bfa88; continue; }
                pc = 0x800bfa04;
            }
            0x800bfa04 => { ins += 1;
                pc = 0x800bfa68; continue;
                pc = 0x800bfa08;
            }
            0x800bfa08 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[7] = 0x0;
                c.lr = 0x800bfa20; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c24bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfa20;
            }
            0x800bfa20 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bfa8c; continue;
                pc = 0x800bfa28;
            }
            0x800bfa28 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[7] = 0x0;
                c.lr = 0x800bfa40; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1fb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfa40;
            }
            0x800bfa40 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bfa8c; continue;
                pc = 0x800bfa48;
            }
            0x800bfa48 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[7] = 0x0;
                c.lr = 0x800bfa60; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1e40(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfa60;
            }
            0x800bfa60 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bfa8c; continue;
                pc = 0x800bfa68;
            }
            0x800bfa68 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[7] = 0x0;
                c.lr = 0x800bfa80; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1a08(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfa80;
            }
            0x800bfa80 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bfa8c; continue;
                pc = 0x800bfa88;
            }
            0x800bfa88 => { ins += 1;
                c.r[31] = 0x703;
                pc = 0x800bfa8c;
            }
            0x800bfa8c => { ins += 3;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.lr = 0x800bfa98; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee74(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfa98;
            }
            0x800bfa98 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bfad8; continue; }
                pc = 0x800bfaa0;
            }
            0x800bfaa0 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x8c);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bfab0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfab0;
            }
            0x800bfab0 => { ins += 9;
                c.r[3] = 0x40;
                c.r[0] = 0x80;
                c.stw(c.r[1].wrapping_add(0x8c), c.r[3]);
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[1].wrapping_add(0x8c);
                c.r[5] = 0x40;
                c.stb(c.r[1].wrapping_add(0x90), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x94), c.r[31]);
                c.lr = 0x800bfad4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beda0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfad4;
            }
            0x800bfad4 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800bfad8;
            }
            0x800bfad8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bfb90; continue; }
                pc = 0x800bfae0;
            }
            0x800bfae0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x703);
                if cr_bit(cr, 2) { pc = 0x800bfb1c; continue; }
                pc = 0x800bfae8;
            }
            0x800bfae8 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bfb04; continue; }
                pc = 0x800bfaec;
            }
            0x800bfaec => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x701);
                if cr_bit(cr, 2) { pc = 0x800bfb24; continue; }
                pc = 0x800bfaf4;
            }
            0x800bfaf4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bfb34; continue; }
                pc = 0x800bfaf8;
            }
            0x800bfaf8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x302);
                if cr_bit(cr, 2) { pc = 0x800bfb2c; continue; }
                pc = 0x800bfb00;
            }
            0x800bfb00 => { ins += 1;
                pc = 0x800bfb54; continue;
                pc = 0x800bfb04;
            }
            0x800bfb04 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x706);
                if cr_bit(cr, 2) { pc = 0x800bfb4c; continue; }
                pc = 0x800bfb0c;
            }
            0x800bfb0c => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bfb54; continue; }
                pc = 0x800bfb10;
            }
            0x800bfb10 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x705);
                if !cr_bit(cr, 0) { pc = 0x800bfb44; continue; }
                pc = 0x800bfb18;
            }
            0x800bfb18 => { ins += 1;
                pc = 0x800bfb3c; continue;
                pc = 0x800bfb1c;
            }
            0x800bfb1c => { ins += 2;
                c.r[31] = 0x12;
                pc = 0x800bfb58; continue;
                pc = 0x800bfb24;
            }
            0x800bfb24 => { ins += 2;
                c.r[31] = 0x14;
                pc = 0x800bfb58; continue;
                pc = 0x800bfb2c;
            }
            0x800bfb2c => { ins += 2;
                c.r[31] = 0x2;
                pc = 0x800bfb58; continue;
                pc = 0x800bfb34;
            }
            0x800bfb34 => { ins += 2;
                c.r[31] = 0x15;
                pc = 0x800bfb58; continue;
                pc = 0x800bfb3c;
            }
            0x800bfb3c => { ins += 2;
                c.r[31] = 0x21;
                pc = 0x800bfb58; continue;
                pc = 0x800bfb44;
            }
            0x800bfb44 => { ins += 2;
                c.r[31] = 0x22;
                pc = 0x800bfb58; continue;
                pc = 0x800bfb4c;
            }
            0x800bfb4c => { ins += 2;
                c.r[31] = 0x20;
                pc = 0x800bfb58; continue;
                pc = 0x800bfb54;
            }
            0x800bfb54 => { ins += 1;
                c.r[31] = 0x3;
                pc = 0x800bfb58;
            }
            0x800bfb58 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bfb68; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfb68;
            }
            0x800bfb68 => { ins += 8;
                c.r[3] = 0x80;
                c.r[0] = 0x40;
                c.stb(c.r[1].wrapping_add(0x10), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x14), c.r[31]);
                c.lr = 0x800bfb88; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfb88;
            }
            0x800bfb88 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bfbcc; continue;
                pc = 0x800bfb90;
            }
            0x800bfb90 => { ins += 5;
                c.r[4] = 0x80270000;
                c.r[3] = 0x1;
                c.r[4] = c.r[4].wrapping_add(0xfffff8b8);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfba4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfba4;
            }
            0x800bfba4 => { ins += 2;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x800bfbac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be800(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfbac;
            }
            0x800bfbac => { ins += 7;
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = 0x80270000;
                c.r[3] = 0x1;
                c.r[4] = c.r[4].wrapping_add(0xfffff8d8);
                c.r[5] = c.r[31] | c.r[31];
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfbc8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfbc8;
            }
            0x800bfbc8 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800bfbcc;
            }
            0x800bfbcc => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xe4));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xdc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0xd8));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0xd4));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0xd0));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xe0);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bf95c: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoReadRegisters (0x800bfbec, 736 bytes)
#[inline(never)]
pub fn f_800bfbec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bfbec) { return; }
        if c.prof { crate::prof::enter(c, 0x800bfbec); p = true; }
    }
    f_800bfbec_b(c, 0x800bfbec);
    if p { crate::prof::exit(c, 0x800bfbec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bfbec_at(c: &mut Cpu, pc: u32) { f_800bfbec_b(c, pc) }
#[inline(never)]
fn f_800bfbec_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bfbec;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bfbec, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bfbec => { ins += 13;
                { let ea = c.r[1].wrapping_add(0xffffff20); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0xe4), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xdc), c.r[31]);
                c.r[31] = c.r[5].wrapping_add(0xfffff858);
                c.stw(c.r[1].wrapping_add(0xd8), c.r[30]);
                c.stw(c.r[1].wrapping_add(0xd4), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = c.lhz_t(ins, c.r[3].wrapping_add(0x1c));
                c.r[0] = c.lhz_t(ins, c.r[3].wrapping_add(0x20));
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x800bfc5c; continue; }
                pc = 0x800bfc20;
            }
            0x800bfc20 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x4c);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bfc30; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfc30;
            }
            0x800bfc30 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x14;
                c.stb(c.r[1].wrapping_add(0x50), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x4c);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x4c), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x54), c.r[0]);
                c.lr = 0x800bfc54; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfc54;
            }
            0x800bfc54 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bfeb0; continue;
                pc = 0x800bfc5c;
            }
            0x800bfc5c => { ins += 6;
                c.r[4] = 0x80;
                c.r[0] = 0x468;
                c.stb(c.r[1].wrapping_add(0x90), c.r[4]);
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x8c), c.r[0]);
                c.lr = 0x800bfc74; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee74(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfc74;
            }
            0x800bfc74 => { ins += 5;
                c.r[5] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[4] = c.r[31].wrapping_add(0x98);
                c.r[3] = 0x4;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfc88; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfc88;
            }
            0x800bfc88 => { ins += 4;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[1].wrapping_add(0x8c);
                c.r[5] = 0x40;
                c.lr = 0x800bfc98; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfc98;
            }
            0x800bfc98 => { ins += 5;
                c.r[5] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[4] = c.r[31].wrapping_add(0x98);
                c.r[3] = 0x4;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfcac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfcac;
            }
            0x800bfcac => { ins += 6;
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[4] = 0x24;
                c.r[7] = 0x1;
                c.lr = 0x800bfcc4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c24bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfcc4;
            }
            0x800bfcc4 => { ins += 6;
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[31].wrapping_add(0xc0);
                c.r[3] = 0x4;
                c.r[5] = c.r[30] | c.r[30];
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfcdc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfcdc;
            }
            0x800bfcdc => { ins += 5;
                c.r[5] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[4] = c.r[31].wrapping_add(0x98);
                c.r[3] = 0x4;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfcf0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfcf0;
            }
            0x800bfcf0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bfd14; continue; }
                pc = 0x800bfcf8;
            }
            0x800bfcf8 => { ins += 6;
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[4] = 0x21;
                c.r[7] = 0x1;
                c.lr = 0x800bfd10; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1fb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfd10;
            }
            0x800bfd10 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800bfd14;
            }
            0x800bfd14 => { ins += 5;
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].wrapping_add(0xf8);
                c.r[3] = 0x4;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfd28; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfd28;
            }
            0x800bfd28 => { ins += 5;
                c.r[5] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[4] = c.r[31].wrapping_add(0x98);
                c.r[3] = 0x4;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfd3c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfd3c;
            }
            0x800bfd3c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bfd60; continue; }
                pc = 0x800bfd44;
            }
            0x800bfd44 => { ins += 6;
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[4] = 0x60;
                c.r[7] = 0x1;
                c.lr = 0x800bfd5c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1e40(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfd5c;
            }
            0x800bfd5c => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800bfd60;
            }
            0x800bfd60 => { ins += 5;
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].wrapping_add(0x120);
                c.r[3] = 0x4;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfd74; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfd74;
            }
            0x800bfd74 => { ins += 5;
                c.r[5] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[4] = c.r[31].wrapping_add(0x98);
                c.r[3] = 0x4;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfd88; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfd88;
            }
            0x800bfd88 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800bfdac; continue; }
                pc = 0x800bfd90;
            }
            0x800bfd90 => { ins += 6;
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.r[3] = 0x0;
                c.r[4] = 0x1f;
                c.r[7] = 0x1;
                c.lr = 0x800bfda8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1a08(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfda8;
            }
            0x800bfda8 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800bfdac;
            }
            0x800bfdac => { ins += 5;
                c.r[5] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].wrapping_add(0x150);
                c.r[3] = 0x4;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfdc0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfdc0;
            }
            0x800bfdc0 => { ins += 5;
                c.r[5] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[4] = c.r[31].wrapping_add(0x98);
                c.r[3] = 0x4;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfdd4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfdd4;
            }
            0x800bfdd4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x800bfe7c; continue; }
                pc = 0x800bfddc;
            }
            0x800bfddc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x704);
                if cr_bit(cr, 2) { pc = 0x800bfe28; continue; }
                pc = 0x800bfde4;
            }
            0x800bfde4 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bfe00; continue; }
                pc = 0x800bfde8;
            }
            0x800bfde8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x702);
                if cr_bit(cr, 2) { pc = 0x800bfe20; continue; }
                pc = 0x800bfdf0;
            }
            0x800bfdf0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bfe10; continue; }
                pc = 0x800bfdf4;
            }
            0x800bfdf4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x701);
                if !cr_bit(cr, 0) { pc = 0x800bfe18; continue; }
                pc = 0x800bfdfc;
            }
            0x800bfdfc => { ins += 1;
                pc = 0x800bfe40; continue;
                pc = 0x800bfe00;
            }
            0x800bfe00 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x706);
                if cr_bit(cr, 2) { pc = 0x800bfe38; continue; }
                pc = 0x800bfe08;
            }
            0x800bfe08 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800bfe40; continue; }
                pc = 0x800bfe0c;
            }
            0x800bfe0c => { ins += 1;
                pc = 0x800bfe30; continue;
                pc = 0x800bfe10;
            }
            0x800bfe10 => { ins += 2;
                c.r[30] = 0x12;
                pc = 0x800bfe44; continue;
                pc = 0x800bfe18;
            }
            0x800bfe18 => { ins += 2;
                c.r[30] = 0x14;
                pc = 0x800bfe44; continue;
                pc = 0x800bfe20;
            }
            0x800bfe20 => { ins += 2;
                c.r[30] = 0x15;
                pc = 0x800bfe44; continue;
                pc = 0x800bfe28;
            }
            0x800bfe28 => { ins += 2;
                c.r[30] = 0x21;
                pc = 0x800bfe44; continue;
                pc = 0x800bfe30;
            }
            0x800bfe30 => { ins += 2;
                c.r[30] = 0x22;
                pc = 0x800bfe44; continue;
                pc = 0x800bfe38;
            }
            0x800bfe38 => { ins += 2;
                c.r[30] = 0x20;
                pc = 0x800bfe44; continue;
                pc = 0x800bfe40;
            }
            0x800bfe40 => { ins += 1;
                c.r[30] = 0x3;
                pc = 0x800bfe44;
            }
            0x800bfe44 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800bfe54; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfe54;
            }
            0x800bfe54 => { ins += 8;
                c.r[3] = 0x80;
                c.r[0] = 0x40;
                c.stb(c.r[1].wrapping_add(0x10), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x14), c.r[30]);
                c.lr = 0x800bfe74; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfe74;
            }
            0x800bfe74 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800bfeb0; continue;
                pc = 0x800bfe7c;
            }
            0x800bfe7c => { ins += 4;
                c.r[4] = c.r[31].wrapping_add(0x60);
                c.r[3] = 0x1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfe8c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfe8c;
            }
            0x800bfe8c => { ins += 2;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800bfe94; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be800(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfe94;
            }
            0x800bfe94 => { ins += 6;
                c.r[4] = c.r[31].wrapping_add(0x80);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.r[5] = c.r[31] | c.r[31];
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800bfeac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bfeac;
            }
            0x800bfeac => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800bfeb0;
            }
            0x800bfeb0 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xe4));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xdc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0xd8));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0xd4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xe0);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bfbec: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoWriteMemory (0x800bfecc, 572 bytes)
#[inline(never)]
pub fn f_800bfecc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800bfecc) { return; }
        if c.prof { crate::prof::enter(c, 0x800bfecc); p = true; }
    }
    f_800bfecc_b(c, 0x800bfecc);
    if p { crate::prof::exit(c, 0x800bfecc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800bfecc_at(c: &mut Cpu, pc: u32) { f_800bfecc_b(c, pc) }
#[inline(never)]
fn f_800bfecc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800bfecc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800bfecc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800bfecc => { ins += 1;
                c.r[11] = c.r[1].rotate_left(0) & 0x1f;
                pc = 0x800bfed0;
            }
            0x800bfed0 => { ins += 1;
                c.r[12] = c.r[1] | c.r[1];
                pc = 0x800bfed4;
            }
            0x800bfed4 => { ins += 1;
                c.r[11] = c.addc(!c.r[11], 0xfffff6c0, 1);
                pc = 0x800bfed8;
            }
            0x800bfed8 => { ins += 1;
                { let ea = c.r[1].wrapping_add(c.r[11]); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800bfedc;
            }
            0x800bfedc => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x800bfee0;
            }
            0x800bfee0 => { ins += 1;
                c.stw(c.r[12].wrapping_add(0x4), c.r[0]);
                pc = 0x800bfee4;
            }
            0x800bfee4 => { ins += 1;
                { let mut ea = c.r[12].wrapping_add(0xffffffec); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x800bfee8;
            }
            0x800bfee8 => { ins += 1;
                c.r[27] = c.r[3] | c.r[3];
                pc = 0x800bfeec;
            }
            0x800bfeec => { ins += 1;
                c.r[3] = 0x80270000;
                pc = 0x800bfef0;
            }
            0x800bfef0 => { ins += 1;
                c.r[31] = c.r[3].wrapping_add(0xfffff858);
                pc = 0x800bfef4;
            }
            0x800bfef4 => { ins += 1;
                c.r[4] = c.r[31].wrapping_add(0x180);
                pc = 0x800bfef8;
            }
            0x800bfef8 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x800bfefc;
            }
            0x800bfefc => { ins += 1;
                c.r[28] = c.lwz_t(ins, c.r[27].wrapping_add(0x20));
                pc = 0x800bff00;
            }
            0x800bff00 => { ins += 1;
                c.r[29] = c.lhz_t(ins, c.r[27].wrapping_add(0x1c));
                pc = 0x800bff04;
            }
            0x800bff04 => { ins += 1;
                c.r[30] = c.lbz_t(ins, c.r[27].wrapping_add(0x18));
                pc = 0x800bff08;
            }
            0x800bff08 => { ins += 1;
                c.r[6] = c.r[28] | c.r[28];
                pc = 0x800bff0c;
            }
            0x800bff0c => { ins += 1;
                c.r[5] = c.lbz_t(ins, c.r[27].wrapping_add(0x14));
                pc = 0x800bff10;
            }
            0x800bff10 => { ins += 1;
                c.r[7] = c.r[29] | c.r[29];
                pc = 0x800bff14;
            }
            0x800bff14 => { ins += 1;
                c.r[8] = c.r[30] | c.r[30];
                pc = 0x800bff18;
            }
            0x800bff18 => { ins += 1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                pc = 0x800bff1c;
            }
            0x800bff1c => { ins += 1;
                c.lr = 0x800bff20; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bff20;
            }
            0x800bff20 => { ins += 1;
                c.r[0] = c.r[30].rotate_left(0) & 0x2;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x800bff24;
            }
            0x800bff24 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800bff64; continue; }
                pc = 0x800bff28;
            }
            0x800bff28 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x64);
                pc = 0x800bff2c;
            }
            0x800bff2c => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800bff30;
            }
            0x800bff30 => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800bff34;
            }
            0x800bff34 => { ins += 1;
                c.lr = 0x800bff38; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bff38;
            }
            0x800bff38 => { ins += 1;
                c.r[3] = 0x80;
                pc = 0x800bff3c;
            }
            0x800bff3c => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800bff40;
            }
            0x800bff40 => { ins += 1;
                c.r[0] = 0x12;
                pc = 0x800bff44;
            }
            0x800bff44 => { ins += 1;
                c.stb(c.r[1].wrapping_add(0x68), c.r[3]);
                pc = 0x800bff48;
            }
            0x800bff48 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x64);
                pc = 0x800bff4c;
            }
            0x800bff4c => { ins += 1;
                c.r[4] = 0x40;
                pc = 0x800bff50;
            }
            0x800bff50 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x64), c.r[5]);
                pc = 0x800bff54;
            }
            0x800bff54 => { ins += 1;
                c.stb(c.r[1].wrapping_add(0x6c), c.r[0]);
                pc = 0x800bff58;
            }
            0x800bff58 => { ins += 1;
                c.lr = 0x800bff5c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bff5c;
            }
            0x800bff5c => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x800bff60;
            }
            0x800bff60 => { ins += 1;
                pc = 0x800c00f0; continue;
                pc = 0x800bff64;
            }
            0x800bff64 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x20), c.r[29]);
                pc = 0x800bff68;
            }
            0x800bff68 => { ins += 1;
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x800bff6c;
            }
            0x800bff6c => { ins += 1;
                c.r[4] = 0x40;
                pc = 0x800bff70;
            }
            0x800bff70 => { ins += 1;
                c.lr = 0x800bff74; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bff74;
            }
            0x800bff74 => { ins += 1;
                c.r[0] = c.r[30].rotate_left(0) & 0x40;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x800bff78;
            }
            0x800bff78 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800bffb0; continue; }
                pc = 0x800bff7c;
            }
            0x800bff7c => { ins += 1;
                c.r[0] = c.r[28].rotate_left(0) & 0x1f;
                pc = 0x800bff80;
            }
            0x800bff80 => { ins += 1;
                c.r[4] = c.r[1].wrapping_add(0x100);
                pc = 0x800bff84;
            }
            0x800bff84 => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x20));
                pc = 0x800bff88;
            }
            0x800bff88 => { ins += 1;
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x800bff8c;
            }
            0x800bff8c => { ins += 1;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                pc = 0x800bff90;
            }
            0x800bff90 => { ins += 1;
                c.lr = 0x800bff94; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bed14(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bff94;
            }
            0x800bff94 => { ins += 1;
                c.r[4] = c.r[28] | c.r[28];
                pc = 0x800bff98;
            }
            0x800bff98 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x100);
                pc = 0x800bff9c;
            }
            0x800bff9c => { ins += 1;
                c.r[5] = c.r[1].wrapping_add(0x20);
                pc = 0x800bffa0;
            }
            0x800bffa0 => { ins += 1;
                c.r[6] = 0x0;
                pc = 0x800bffa4;
            }
            0x800bffa4 => { ins += 1;
                c.lr = 0x800bffa8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c123c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bffa8;
            }
            0x800bffa8 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800bffac;
            }
            0x800bffac => { ins += 1;
                pc = 0x800bffe0; continue;
                pc = 0x800bffb0;
            }
            0x800bffb0 => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x20));
                pc = 0x800bffb4;
            }
            0x800bffb4 => { ins += 1;
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x800bffb8;
            }
            0x800bffb8 => { ins += 1;
                c.r[4] = c.r[1].wrapping_add(0x100);
                pc = 0x800bffbc;
            }
            0x800bffbc => { ins += 1;
                c.lr = 0x800bffc0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bed14(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bffc0;
            }
            0x800bffc0 => { ins += 1;
                c.r[0] = c.r[30].rotate_left(29) & 0x1;
                pc = 0x800bffc4;
            }
            0x800bffc4 => { ins += 1;
                c.r[4] = c.r[28] | c.r[28];
                pc = 0x800bffc8;
            }
            0x800bffc8 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x100);
                pc = 0x800bffcc;
            }
            0x800bffcc => { ins += 1;
                c.r[5] = c.r[1].wrapping_add(0x20);
                pc = 0x800bffd0;
            }
            0x800bffd0 => { ins += 1;
                c.r[6] = c.r[0] ^ 0x1;
                pc = 0x800bffd4;
            }
            0x800bffd4 => { ins += 1;
                c.r[7] = 0x0;
                pc = 0x800bffd8;
            }
            0x800bffd8 => { ins += 1;
                c.lr = 0x800bffdc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c25fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bffdc;
            }
            0x800bffdc => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800bffe0;
            }
            0x800bffe0 => { ins += 1;
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x800bffe4;
            }
            0x800bffe4 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800bffe8;
            }
            0x800bffe8 => { ins += 1;
                c.lr = 0x800bffec; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee74(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800bffec;
            }
            0x800bffec => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                pc = 0x800bfff0;
            }
            0x800bfff0 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800c002c; continue; }
                pc = 0x800bfff4;
            }
            0x800bfff4 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0xa4);
                pc = 0x800bfff8;
            }
            0x800bfff8 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800bfffc;
            }
            0x800bfffc => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800c0000;
            }
            0x800c0000 => { ins += 1;
                c.lr = 0x800c0004; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0004;
            }
            0x800c0004 => { ins += 1;
                c.r[3] = 0x40;
                pc = 0x800c0008;
            }
            0x800c0008 => { ins += 1;
                c.r[0] = 0x80;
                pc = 0x800c000c;
            }
            0x800c000c => { ins += 1;
                c.stw(c.r[1].wrapping_add(0xa4), c.r[3]);
                pc = 0x800c0010;
            }
            0x800c0010 => { ins += 1;
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x800c0014;
            }
            0x800c0014 => { ins += 1;
                c.r[4] = c.r[1].wrapping_add(0xa4);
                pc = 0x800c0018;
            }
            0x800c0018 => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800c001c;
            }
            0x800c001c => { ins += 1;
                c.stb(c.r[1].wrapping_add(0xa8), c.r[0]);
                pc = 0x800c0020;
            }
            0x800c0020 => { ins += 1;
                c.stb(c.r[1].wrapping_add(0xac), c.r[30]);
                pc = 0x800c0024;
            }
            0x800c0024 => { ins += 1;
                c.lr = 0x800c0028; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beda0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0028;
            }
            0x800c0028 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800c002c;
            }
            0x800c002c => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                pc = 0x800c0030;
            }
            0x800c0030 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800c00bc; continue; }
                pc = 0x800c0034;
            }
            0x800c0034 => { ins += 1;
                c.r[0] = c.r[30].wrapping_add(0xfffff900);
                pc = 0x800c0038;
            }
            0x800c0038 => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                pc = 0x800c003c;
            }
            0x800c003c => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x800c0080; continue; }
                pc = 0x800c0040;
            }
            0x800c0040 => { ins += 1;
                c.r[3] = 0x80310000;
                pc = 0x800c0044;
            }
            0x800c0044 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800c0048;
            }
            0x800c0048 => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x37e0);
                pc = 0x800c004c;
            }
            0x800c004c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[0]));
                pc = 0x800c0050;
            }
            0x800c0050 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x800c0054;
            }
            0x800c0054 => { ins += 1;
                if true { let t = c.ctr; if (0x800bfecc..0x800c0108).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800c0058;
            }
            0x800c0058 => { ins += 1;
                c.r[30] = 0x15;
                pc = 0x800c005c;
            }
            0x800c005c => { ins += 1;
                pc = 0x800c0084; continue;
                pc = 0x800c0060;
            }
            0x800c0060 => { ins += 1;
                c.r[30] = 0x13;
                pc = 0x800c0064;
            }
            0x800c0064 => { ins += 1;
                pc = 0x800c0084; continue;
                pc = 0x800c0068;
            }
            0x800c0068 => { ins += 1;
                c.r[30] = 0x21;
                pc = 0x800c006c;
            }
            0x800c006c => { ins += 1;
                pc = 0x800c0084; continue;
                pc = 0x800c0070;
            }
            0x800c0070 => { ins += 1;
                c.r[30] = 0x22;
                pc = 0x800c0074;
            }
            0x800c0074 => { ins += 1;
                pc = 0x800c0084; continue;
                pc = 0x800c0078;
            }
            0x800c0078 => { ins += 1;
                c.r[30] = 0x20;
                pc = 0x800c007c;
            }
            0x800c007c => { ins += 1;
                pc = 0x800c0084; continue;
                pc = 0x800c0080;
            }
            0x800c0080 => { ins += 1;
                c.r[30] = 0x3;
                pc = 0x800c0084;
            }
            0x800c0084 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x24);
                pc = 0x800c0088;
            }
            0x800c0088 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800c008c;
            }
            0x800c008c => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800c0090;
            }
            0x800c0090 => { ins += 1;
                c.lr = 0x800c0094; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0094;
            }
            0x800c0094 => { ins += 1;
                c.r[3] = 0x80;
                pc = 0x800c0098;
            }
            0x800c0098 => { ins += 1;
                c.r[0] = 0x40;
                pc = 0x800c009c;
            }
            0x800c009c => { ins += 1;
                c.stb(c.r[1].wrapping_add(0x28), c.r[3]);
                pc = 0x800c00a0;
            }
            0x800c00a0 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x24);
                pc = 0x800c00a4;
            }
            0x800c00a4 => { ins += 1;
                c.r[4] = 0x40;
                pc = 0x800c00a8;
            }
            0x800c00a8 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x800c00ac;
            }
            0x800c00ac => { ins += 1;
                c.stb(c.r[1].wrapping_add(0x2c), c.r[30]);
                pc = 0x800c00b0;
            }
            0x800c00b0 => { ins += 1;
                c.lr = 0x800c00b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c00b4;
            }
            0x800c00b4 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x800c00b8;
            }
            0x800c00b8 => { ins += 1;
                pc = 0x800c00f0; continue;
                pc = 0x800c00bc;
            }
            0x800c00bc => { ins += 1;
                c.r[4] = c.r[31].wrapping_add(0x60);
                pc = 0x800c00c0;
            }
            0x800c00c0 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x800c00c4;
            }
            0x800c00c4 => { ins += 1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                pc = 0x800c00c8;
            }
            0x800c00c8 => { ins += 1;
                c.lr = 0x800c00cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c00cc;
            }
            0x800c00cc => { ins += 1;
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x800c00d0;
            }
            0x800c00d0 => { ins += 1;
                c.lr = 0x800c00d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be800(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c00d4;
            }
            0x800c00d4 => { ins += 1;
                c.r[4] = c.r[31].wrapping_add(0x80);
                pc = 0x800c00d8;
            }
            0x800c00d8 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800c00dc;
            }
            0x800c00dc => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x800c00e0;
            }
            0x800c00e0 => { ins += 1;
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x800c00e4;
            }
            0x800c00e4 => { ins += 1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                pc = 0x800c00e8;
            }
            0x800c00e8 => { ins += 1;
                c.lr = 0x800c00ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c00ec;
            }
            0x800c00ec => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800c00f0;
            }
            0x800c00f0 => { ins += 1;
                c.r[10] = c.lwz_t(ins, c.r[1]);
                pc = 0x800c00f4;
            }
            0x800c00f4 => { ins += 1;
                { let mut ea = c.r[10].wrapping_add(0xffffffec); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                pc = 0x800c00f8;
            }
            0x800c00f8 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[10].wrapping_add(0x4));
                pc = 0x800c00fc;
            }
            0x800c00fc => { ins += 1;
                c.mtspr(8, c.r[0]);
                pc = 0x800c0100;
            }
            0x800c0100 => { ins += 1;
                c.r[1] = c.r[10] | c.r[10];
                pc = 0x800c0104;
            }
            0x800c0104 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800bfecc: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoReadMemory (0x800c0108, 580 bytes)
#[inline(never)]
pub fn f_800c0108(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0108) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0108); p = true; }
    }
    f_800c0108_b(c, 0x800c0108);
    if p { crate::prof::exit(c, 0x800c0108); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0108_at(c: &mut Cpu, pc: u32) { f_800c0108_b(c, pc) }
#[inline(never)]
fn f_800c0108_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0108;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0108, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0108 => { ins += 1;
                c.r[11] = c.r[1].rotate_left(0) & 0x1f;
                pc = 0x800c010c;
            }
            0x800c010c => { ins += 1;
                c.r[12] = c.r[1] | c.r[1];
                pc = 0x800c0110;
            }
            0x800c0110 => { ins += 1;
                c.r[11] = c.addc(!c.r[11], 0xfffff6c0, 1);
                pc = 0x800c0114;
            }
            0x800c0114 => { ins += 1;
                { let ea = c.r[1].wrapping_add(c.r[11]); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800c0118;
            }
            0x800c0118 => { ins += 1;
                c.r[0] = c.mfspr(8);
                pc = 0x800c011c;
            }
            0x800c011c => { ins += 1;
                c.stw(c.r[12].wrapping_add(0x4), c.r[0]);
                pc = 0x800c0120;
            }
            0x800c0120 => { ins += 1;
                { let mut ea = c.r[12].wrapping_add(0xffffffe8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x800c0124;
            }
            0x800c0124 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800c0128;
            }
            0x800c0128 => { ins += 1;
                c.r[3] = 0x80270000;
                pc = 0x800c012c;
            }
            0x800c012c => { ins += 1;
                c.r[29] = c.r[3].wrapping_add(0xfffff858);
                pc = 0x800c0130;
            }
            0x800c0130 => { ins += 1;
                c.r[4] = c.r[29].wrapping_add(0x1b0);
                pc = 0x800c0134;
            }
            0x800c0134 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x800c0138;
            }
            0x800c0138 => { ins += 1;
                c.r[26] = c.lwz_t(ins, c.r[31].wrapping_add(0x20));
                pc = 0x800c013c;
            }
            0x800c013c => { ins += 1;
                c.r[27] = c.lhz_t(ins, c.r[31].wrapping_add(0x1c));
                pc = 0x800c0140;
            }
            0x800c0140 => { ins += 1;
                c.r[30] = c.lbz_t(ins, c.r[31].wrapping_add(0x18));
                pc = 0x800c0144;
            }
            0x800c0144 => { ins += 1;
                c.r[6] = c.r[26] | c.r[26];
                pc = 0x800c0148;
            }
            0x800c0148 => { ins += 1;
                c.r[5] = c.lbz_t(ins, c.r[31].wrapping_add(0x14));
                pc = 0x800c014c;
            }
            0x800c014c => { ins += 1;
                c.r[7] = c.r[27] | c.r[27];
                pc = 0x800c0150;
            }
            0x800c0150 => { ins += 1;
                c.r[8] = c.r[30] | c.r[30];
                pc = 0x800c0154;
            }
            0x800c0154 => { ins += 1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                pc = 0x800c0158;
            }
            0x800c0158 => { ins += 1;
                c.lr = 0x800c015c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c015c;
            }
            0x800c015c => { ins += 1;
                c.r[0] = c.r[30].rotate_left(0) & 0x2;
                cr_rc(&mut cr, c.xer, c.r[0]);
                pc = 0x800c0160;
            }
            0x800c0160 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800c01a0; continue; }
                pc = 0x800c0164;
            }
            0x800c0164 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x64);
                pc = 0x800c0168;
            }
            0x800c0168 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800c016c;
            }
            0x800c016c => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800c0170;
            }
            0x800c0170 => { ins += 1;
                c.lr = 0x800c0174; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0174;
            }
            0x800c0174 => { ins += 1;
                c.r[3] = 0x80;
                pc = 0x800c0178;
            }
            0x800c0178 => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800c017c;
            }
            0x800c017c => { ins += 1;
                c.r[0] = 0x12;
                pc = 0x800c0180;
            }
            0x800c0180 => { ins += 1;
                c.stb(c.r[1].wrapping_add(0x68), c.r[3]);
                pc = 0x800c0184;
            }
            0x800c0184 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x64);
                pc = 0x800c0188;
            }
            0x800c0188 => { ins += 1;
                c.r[4] = 0x40;
                pc = 0x800c018c;
            }
            0x800c018c => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x64), c.r[5]);
                pc = 0x800c0190;
            }
            0x800c0190 => { ins += 1;
                c.stb(c.r[1].wrapping_add(0x6c), c.r[0]);
                pc = 0x800c0194;
            }
            0x800c0194 => { ins += 1;
                c.lr = 0x800c0198; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0198;
            }
            0x800c0198 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x800c019c;
            }
            0x800c019c => { ins += 1;
                pc = 0x800c0334; continue;
                pc = 0x800c01a0;
            }
            0x800c01a0 => { ins += 1;
                c.r[28] = c.r[30].rotate_left(0) & 0x40;
                cr_rc(&mut cr, c.xer, c.r[28]);
                pc = 0x800c01a4;
            }
            0x800c01a4 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x20), c.r[27]);
                pc = 0x800c01a8;
            }
            0x800c01a8 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800c01c8; continue; }
                pc = 0x800c01ac;
            }
            0x800c01ac => { ins += 1;
                c.r[4] = c.r[26] | c.r[26];
                pc = 0x800c01b0;
            }
            0x800c01b0 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x100);
                pc = 0x800c01b4;
            }
            0x800c01b4 => { ins += 1;
                c.r[5] = c.r[1].wrapping_add(0x20);
                pc = 0x800c01b8;
            }
            0x800c01b8 => { ins += 1;
                c.r[6] = 0x1;
                pc = 0x800c01bc;
            }
            0x800c01bc => { ins += 1;
                c.lr = 0x800c01c0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c123c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c01c0;
            }
            0x800c01c0 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800c01c4;
            }
            0x800c01c4 => { ins += 1;
                pc = 0x800c01e8; continue;
                pc = 0x800c01c8;
            }
            0x800c01c8 => { ins += 1;
                c.r[0] = c.r[30].rotate_left(29) & 0x1;
                pc = 0x800c01cc;
            }
            0x800c01cc => { ins += 1;
                c.r[4] = c.r[26] | c.r[26];
                pc = 0x800c01d0;
            }
            0x800c01d0 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x100);
                pc = 0x800c01d4;
            }
            0x800c01d4 => { ins += 1;
                c.r[5] = c.r[1].wrapping_add(0x20);
                pc = 0x800c01d8;
            }
            0x800c01d8 => { ins += 1;
                c.r[6] = c.r[0] ^ 0x1;
                pc = 0x800c01dc;
            }
            0x800c01dc => { ins += 1;
                c.r[7] = 0x1;
                pc = 0x800c01e0;
            }
            0x800c01e0 => { ins += 1;
                c.lr = 0x800c01e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c25fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c01e4;
            }
            0x800c01e4 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800c01e8;
            }
            0x800c01e8 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800c01ec;
            }
            0x800c01ec => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800c01f0;
            }
            0x800c01f0 => { ins += 1;
                c.lr = 0x800c01f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee74(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c01f4;
            }
            0x800c01f4 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                pc = 0x800c01f8;
            }
            0x800c01f8 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800c0270; continue; }
                pc = 0x800c01fc;
            }
            0x800c01fc => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0xa4);
                pc = 0x800c0200;
            }
            0x800c0200 => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800c0204;
            }
            0x800c0204 => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800c0208;
            }
            0x800c0208 => { ins += 1;
                c.lr = 0x800c020c; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c020c;
            }
            0x800c020c => { ins += 1;
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x20));
                pc = 0x800c0210;
            }
            0x800c0210 => { ins += 1;
                c.r[0] = 0x80;
                pc = 0x800c0214;
            }
            0x800c0214 => { ins += 1;
                c.stb(c.r[1].wrapping_add(0xac), c.r[30]);
                pc = 0x800c0218;
            }
            0x800c0218 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800c021c;
            }
            0x800c021c => { ins += 1;
                c.r[4] = c.r[4].wrapping_add(0x40);
                pc = 0x800c0220;
            }
            0x800c0220 => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800c0224;
            }
            0x800c0224 => { ins += 1;
                c.stw(c.r[1].wrapping_add(0xa4), c.r[4]);
                pc = 0x800c0228;
            }
            0x800c0228 => { ins += 1;
                c.r[4] = c.r[1].wrapping_add(0xa4);
                pc = 0x800c022c;
            }
            0x800c022c => { ins += 1;
                c.stb(c.r[1].wrapping_add(0xa8), c.r[0]);
                pc = 0x800c0230;
            }
            0x800c0230 => { ins += 1;
                c.lr = 0x800c0234; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beda0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0234;
            }
            0x800c0234 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x0);
                pc = 0x800c0238;
            }
            0x800c0238 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800c025c; continue; }
                pc = 0x800c023c;
            }
            0x800c023c => { ins += 1;
                c.r[0] = c.r[26].rotate_left(0) & 0x1f;
                pc = 0x800c0240;
            }
            0x800c0240 => { ins += 1;
                c.r[4] = c.r[1].wrapping_add(0x100);
                pc = 0x800c0244;
            }
            0x800c0244 => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x20));
                pc = 0x800c0248;
            }
            0x800c0248 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800c024c;
            }
            0x800c024c => { ins += 1;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                pc = 0x800c0250;
            }
            0x800c0250 => { ins += 1;
                c.lr = 0x800c0254; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beda0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0254;
            }
            0x800c0254 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800c0258;
            }
            0x800c0258 => { ins += 1;
                pc = 0x800c0270; continue;
                pc = 0x800c025c;
            }
            0x800c025c => { ins += 1;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x20));
                pc = 0x800c0260;
            }
            0x800c0260 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800c0264;
            }
            0x800c0264 => { ins += 1;
                c.r[4] = c.r[1].wrapping_add(0x100);
                pc = 0x800c0268;
            }
            0x800c0268 => { ins += 1;
                c.lr = 0x800c026c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beda0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c026c;
            }
            0x800c026c => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800c0270;
            }
            0x800c0270 => { ins += 1;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                pc = 0x800c0274;
            }
            0x800c0274 => { ins += 1;
                if cr_bit(cr, 2) { pc = 0x800c0300; continue; }
                pc = 0x800c0278;
            }
            0x800c0278 => { ins += 1;
                c.r[0] = c.r[30].wrapping_add(0xfffff900);
                pc = 0x800c027c;
            }
            0x800c027c => { ins += 1;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x6);
                pc = 0x800c0280;
            }
            0x800c0280 => { ins += 1;
                if cr_bit(cr, 1) { pc = 0x800c02c4; continue; }
                pc = 0x800c0284;
            }
            0x800c0284 => { ins += 1;
                c.r[3] = 0x80310000;
                pc = 0x800c0288;
            }
            0x800c0288 => { ins += 1;
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800c028c;
            }
            0x800c028c => { ins += 1;
                c.r[3] = c.r[3].wrapping_add(0x37fc);
                pc = 0x800c0290;
            }
            0x800c0290 => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(c.r[0]));
                pc = 0x800c0294;
            }
            0x800c0294 => { ins += 1;
                c.mtspr(9, c.r[0]);
                pc = 0x800c0298;
            }
            0x800c0298 => { ins += 1;
                if true { let t = c.ctr; if (0x800c0108..0x800c034c).contains(&t) { pc = t; continue; } { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800c029c;
            }
            0x800c029c => { ins += 1;
                c.r[28] = 0x15;
                pc = 0x800c02a0;
            }
            0x800c02a0 => { ins += 1;
                pc = 0x800c02c8; continue;
                pc = 0x800c02a4;
            }
            0x800c02a4 => { ins += 1;
                c.r[28] = 0x13;
                pc = 0x800c02a8;
            }
            0x800c02a8 => { ins += 1;
                pc = 0x800c02c8; continue;
                pc = 0x800c02ac;
            }
            0x800c02ac => { ins += 1;
                c.r[28] = 0x21;
                pc = 0x800c02b0;
            }
            0x800c02b0 => { ins += 1;
                pc = 0x800c02c8; continue;
                pc = 0x800c02b4;
            }
            0x800c02b4 => { ins += 1;
                c.r[28] = 0x22;
                pc = 0x800c02b8;
            }
            0x800c02b8 => { ins += 1;
                pc = 0x800c02c8; continue;
                pc = 0x800c02bc;
            }
            0x800c02bc => { ins += 1;
                c.r[28] = 0x20;
                pc = 0x800c02c0;
            }
            0x800c02c0 => { ins += 1;
                pc = 0x800c02c8; continue;
                pc = 0x800c02c4;
            }
            0x800c02c4 => { ins += 1;
                c.r[28] = 0x3;
                pc = 0x800c02c8;
            }
            0x800c02c8 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x24);
                pc = 0x800c02cc;
            }
            0x800c02cc => { ins += 1;
                c.r[4] = 0x0;
                pc = 0x800c02d0;
            }
            0x800c02d0 => { ins += 1;
                c.r[5] = 0x40;
                pc = 0x800c02d4;
            }
            0x800c02d4 => { ins += 1;
                c.lr = 0x800c02d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c02d8;
            }
            0x800c02d8 => { ins += 1;
                c.r[3] = 0x80;
                pc = 0x800c02dc;
            }
            0x800c02dc => { ins += 1;
                c.r[0] = 0x40;
                pc = 0x800c02e0;
            }
            0x800c02e0 => { ins += 1;
                c.stb(c.r[1].wrapping_add(0x28), c.r[3]);
                pc = 0x800c02e4;
            }
            0x800c02e4 => { ins += 1;
                c.r[3] = c.r[1].wrapping_add(0x24);
                pc = 0x800c02e8;
            }
            0x800c02e8 => { ins += 1;
                c.r[4] = 0x40;
                pc = 0x800c02ec;
            }
            0x800c02ec => { ins += 1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x800c02f0;
            }
            0x800c02f0 => { ins += 1;
                c.stb(c.r[1].wrapping_add(0x2c), c.r[28]);
                pc = 0x800c02f4;
            }
            0x800c02f4 => { ins += 1;
                c.lr = 0x800c02f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c02f8;
            }
            0x800c02f8 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x800c02fc;
            }
            0x800c02fc => { ins += 1;
                pc = 0x800c0334; continue;
                pc = 0x800c0300;
            }
            0x800c0300 => { ins += 1;
                c.r[4] = c.r[29].wrapping_add(0x60);
                pc = 0x800c0304;
            }
            0x800c0304 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x800c0308;
            }
            0x800c0308 => { ins += 1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                pc = 0x800c030c;
            }
            0x800c030c => { ins += 1;
                c.lr = 0x800c0310; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0310;
            }
            0x800c0310 => { ins += 1;
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800c0314;
            }
            0x800c0314 => { ins += 1;
                c.lr = 0x800c0318; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be800(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0318;
            }
            0x800c0318 => { ins += 1;
                c.r[4] = c.r[29].wrapping_add(0x80);
                pc = 0x800c031c;
            }
            0x800c031c => { ins += 1;
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x800c0320;
            }
            0x800c0320 => { ins += 1;
                c.r[3] = 0x1;
                pc = 0x800c0324;
            }
            0x800c0324 => { ins += 1;
                c.r[5] = c.r[29] | c.r[29];
                pc = 0x800c0328;
            }
            0x800c0328 => { ins += 1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                pc = 0x800c032c;
            }
            0x800c032c => { ins += 1;
                c.lr = 0x800c0330; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0330;
            }
            0x800c0330 => { ins += 1;
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x800c0334;
            }
            0x800c0334 => { ins += 1;
                c.r[10] = c.lwz_t(ins, c.r[1]);
                pc = 0x800c0338;
            }
            0x800c0338 => { ins += 1;
                { let mut ea = c.r[10].wrapping_add(0xffffffe8); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                pc = 0x800c033c;
            }
            0x800c033c => { ins += 1;
                c.r[0] = c.lwz_t(ins, c.r[10].wrapping_add(0x4));
                pc = 0x800c0340;
            }
            0x800c0340 => { ins += 1;
                c.mtspr(8, c.r[0]);
                pc = 0x800c0344;
            }
            0x800c0344 => { ins += 1;
                c.r[1] = c.r[10] | c.r[10];
                pc = 0x800c0348;
            }
            0x800c0348 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0108: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoSupportMask (0x800c034c, 8 bytes)
#[inline(never)]
pub fn f_800c034c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c034c) { return; }
        if c.prof { crate::prof::enter(c, 0x800c034c); p = true; }
    }
    f_800c034c_b(c, 0x800c034c);
    if p { crate::prof::exit(c, 0x800c034c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c034c_at(c: &mut Cpu, pc: u32) { f_800c034c_b(c, pc) }
#[inline(always)]
fn f_800c034c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c034c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c034c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c034c => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c034c: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoVersions (0x800c0354, 8 bytes)
#[inline(never)]
pub fn f_800c0354(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0354) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0354); p = true; }
    }
    f_800c0354_b(c, 0x800c0354);
    if p { crate::prof::exit(c, 0x800c0354); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0354_at(c: &mut Cpu, pc: u32) { f_800c0354_b(c, pc) }
#[inline(always)]
fn f_800c0354_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0354;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0354, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0354 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0354: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoOverride (0x800c035c, 88 bytes)
#[inline(never)]
pub fn f_800c035c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c035c) { return; }
        if c.prof { crate::prof::enter(c, 0x800c035c); p = true; }
    }
    f_800c035c_b(c, 0x800c035c);
    if p { crate::prof::exit(c, 0x800c035c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c035c_at(c: &mut Cpu, pc: u32) { f_800c035c_b(c, pc) }
#[inline(always)]
fn f_800c035c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c035c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c035c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c035c => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800c0378; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0378;
            }
            0x800c0378 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x800c039c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c039c;
            }
            0x800c039c => { ins += 1;
                c.lr = 0x800c03a0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3218(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c03a0;
            }
            0x800c03a0 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c035c: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoReset (0x800c03b4, 88 bytes)
#[inline(never)]
pub fn f_800c03b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c03b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800c03b4); p = true; }
    }
    f_800c03b4_b(c, 0x800c03b4);
    if p { crate::prof::exit(c, 0x800c03b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c03b4_at(c: &mut Cpu, pc: u32) { f_800c03b4_b(c, pc) }
#[inline(always)]
fn f_800c03b4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c03b4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c03b4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c03b4 => { ins += 7;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800c03d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c03d0;
            }
            0x800c03d0 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x800c03f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c03f4;
            }
            0x800c03f4 => { ins += 1;
                c.lr = 0x800c03f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800053e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c03f8;
            }
            0x800c03f8 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c03b4: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoDisconnect (0x800c040c, 120 bytes)
#[inline(never)]
pub fn f_800c040c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c040c) { return; }
        if c.prof { crate::prof::enter(c, 0x800c040c); p = true; }
    }
    f_800c040c_b(c, 0x800c040c);
    if p { crate::prof::exit(c, 0x800c040c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c040c_at(c: &mut Cpu, pc: u32) { f_800c040c_b(c, pc) }
#[inline(always)]
fn f_800c040c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c040c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c040c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c040c => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.r[5] = 0x40;
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xffffe7d0);
                c.r[0] = 0x0;
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.stw(c.r[4], c.r[0]);
                c.r[4] = 0x0;
                c.lr = 0x800c0438; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0438;
            }
            0x800c0438 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x18), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x14), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x1c), c.r[0]);
                c.lr = 0x800c045c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c045c;
            }
            0x800c045c => { ins += 3;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x1;
                c.lr = 0x800c0468; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be464(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0468;
            }
            0x800c0468 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800c0470; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be47c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0470;
            }
            0x800c0470 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x64));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x60);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c040c: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoConnect (0x800c0484, 100 bytes)
#[inline(never)]
pub fn f_800c0484(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0484) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0484); p = true; }
    }
    f_800c0484_b(c, 0x800c0484);
    if p { crate::prof::exit(c, 0x800c0484); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0484_at(c: &mut Cpu, pc: u32) { f_800c0484_b(c, pc) }
#[inline(always)]
fn f_800c0484_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0484;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0484, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0484 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.r[5] = 0x40;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xffffe7d0);
                c.r[0] = 0x1;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[4], c.r[0]);
                c.r[4] = 0x0;
                c.lr = 0x800c04b0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c04b0;
            }
            0x800c04b0 => { ins += 9;
                c.r[3] = 0x80;
                c.r[5] = 0x40;
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x40;
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x800c04d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3588(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c04d4;
            }
            0x800c04d4 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x54));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0484: bad pc {:#x}", pc),
        }
    }
}

/// SetTRKConnected (0x800c04e8, 12 bytes)
#[inline(never)]
pub fn f_800c04e8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c04e8) { return; }
        if c.prof { crate::prof::enter(c, 0x800c04e8); p = true; }
    }
    f_800c04e8_b(c, 0x800c04e8);
    if p { crate::prof::exit(c, 0x800c04e8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c04e8_at(c: &mut Cpu, pc: u32) { f_800c04e8_b(c, pc) }
#[inline(always)]
fn f_800c04e8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c04e8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c04e8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c04e8 => { ins += 3;
                c.r[4] = 0x80400000;
                c.stw(c.r[4].wrapping_add(0xffffe7d0), c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c04e8: bad pc {:#x}", pc),
        }
    }
}

/// GetTRKConnected (0x800c04f4, 16 bytes)
#[inline(never)]
pub fn f_800c04f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c04f4) { return; }
        if c.prof { crate::prof::enter(c, 0x800c04f4); p = true; }
    }
    f_800c04f4_b(c, 0x800c04f4);
    if p { crate::prof::exit(c, 0x800c04f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c04f4_at(c: &mut Cpu, pc: u32) { f_800c04f4_b(c, pc) }
#[inline(always)]
fn f_800c04f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c04f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c04f4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c04f4 => { ins += 4;
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffe7d0);
                c.r[3] = c.lwz_t(ins, c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c04f4: bad pc {:#x}", pc),
        }
    }
}

/// OutputData (0x800c0504, 168 bytes)
#[inline(never)]
pub fn f_800c0504(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0504) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0504); p = true; }
    }
    f_800c0504_b(c, 0x800c0504);
    if p { crate::prof::exit(c, 0x800c0504); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0504_at(c: &mut Cpu, pc: u32) { f_800c0504_b(c, pc) }
#[inline(always)]
fn f_800c0504_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0504;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0504, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0504 => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80270000;
                c.r[5] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[4] | c.r[4];
                c.r[31] = c.r[3] | c.r[3];
                c.r[29] = c.r[6].wrapping_add(0xfffffa34);
                c.r[30] = c.r[5].wrapping_add(0xfffffa3c);
                c.r[28] = 0x0;
                pc = 0x800c057c; continue;
                pc = 0x800c0534;
            }
            0x800c0534 => { ins += 5;
                c.r[5] = c.lbz_t(ins, c.r[31]);
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x8;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c0548; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0548;
            }
            0x800c0548 => { ins += 7;
                c.r[0] = c.r[28].rotate_left(28) & 0xf0000000;
                c.r[3] = c.r[28].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.r[0] = c.r[0].rotate_left(4) & 0xffffffff;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xf);
                if !cr_bit(cr, 2) { pc = 0x800c0574; continue; }
                pc = 0x800c0564;
            }
            0x800c0564 => { ins += 4;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x8;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c0574; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0574;
            }
            0x800c0574 => { ins += 2;
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x800c057c;
            }
            0x800c057c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], c.r[27]);
                if cr_bit(cr, 0) { pc = 0x800c0534; continue; }
                pc = 0x800c0584;
            }
            0x800c0584 => { ins += 5;
                c.r[4] = 0x80270000;
                c.r[3] = 0x8;
                c.r[4] = c.r[4].wrapping_add(0xfffffa3c);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c0598; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0598;
            }
            0x800c0598 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0504: bad pc {:#x}", pc),
        }
    }
}

/// HandlePositionFileSupportRequest (0x800c05ac, 272 bytes)
#[inline(never)]
pub fn f_800c05ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c05ac) { return; }
        if c.prof { crate::prof::enter(c, 0x800c05ac); p = true; }
    }
    f_800c05ac_b(c, 0x800c05ac);
    if p { crate::prof::exit(c, 0x800c05ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c05ac_at(c: &mut Cpu, pc: u32) { f_800c05ac_b(c, pc) }
#[inline(always)]
fn f_800c05ac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c05ac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c05ac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c05ac => { ins += 15;
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x6c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.r[5] = 0x40;
                c.stw(c.r[1].wrapping_add(0x68), c.r[30]);
                c.r[30] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x64), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x60), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.lr = 0x800c05e8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c05e8;
            }
            0x800c05e8 => { ins += 11;
                c.r[3] = 0xd4;
                c.r[0] = 0x40;
                c.stb(c.r[1].wrapping_add(0x18), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[28]);
                c.r[0] = c.lwz_t(ins, c.r[29]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x24), c.r[31]);
                c.lr = 0x800c0614; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0614;
            }
            0x800c0614 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x800c0630; continue; }
                pc = 0x800c061c;
            }
            0x800c061c => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[5] = 0x40;
                c.lr = 0x800c062c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c062c;
            }
            0x800c062c => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800c0630;
            }
            0x800c0630 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0690; continue; }
                pc = 0x800c0638;
            }
            0x800c0638 => { ins += 10;
                c.r[3] = 0x0;
                c.r[0] = 0xffffffff;
                c.stw(c.r[30], c.r[3]);
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[5] = 0x3;
                c.r[6] = 0x3;
                c.stw(c.r[29], c.r[0]);
                c.r[7] = 0x0;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.lr = 0x800c0660; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c08c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0660;
            }
            0x800c0660 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x800c0688; continue; }
                pc = 0x800c0668;
            }
            0x800c0668 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800c0670; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0670;
            }
            0x800c0670 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c0688; continue; }
                pc = 0x800c0678;
            }
            0x800c0678 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x20));
                c.stw(c.r[30], c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x28));
                c.stw(c.r[29], c.r[0]);
                pc = 0x800c0688;
            }
            0x800c0688 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800c0690; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0690;
            }
            0x800c0690 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.lr = 0x800c0698; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0698;
            }
            0x800c0698 => { ins += 9;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x74));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x6c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x68));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x64));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x60));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c05ac: bad pc {:#x}", pc),
        }
    }
}

/// HandleCloseFileSupportRequest (0x800c06bc, 232 bytes)
#[inline(never)]
pub fn f_800c06bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c06bc) { return; }
        if c.prof { crate::prof::enter(c, 0x800c06bc); p = true; }
    }
    f_800c06bc_b(c, 0x800c06bc);
    if p { crate::prof::exit(c, 0x800c06bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c06bc_at(c: &mut Cpu, pc: u32) { f_800c06bc_b(c, pc) }
#[inline(always)]
fn f_800c06bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c06bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c06bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c06bc => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x40;
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x6c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.stw(c.r[1].wrapping_add(0x68), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x64), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.r[4] = 0x0;
                c.lr = 0x800c06ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c06ec;
            }
            0x800c06ec => { ins += 8;
                c.r[3] = 0xd3;
                c.r[0] = 0x40;
                c.stb(c.r[1].wrapping_add(0x18), c.r[3]);
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.lr = 0x800c070c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c070c;
            }
            0x800c070c => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x800c0728; continue; }
                pc = 0x800c0714;
            }
            0x800c0714 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[5] = 0x40;
                c.lr = 0x800c0724; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0724;
            }
            0x800c0724 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800c0728;
            }
            0x800c0728 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c077c; continue; }
                pc = 0x800c0730;
            }
            0x800c0730 => { ins += 8;
                c.r[0] = 0x0;
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.stw(c.r[29], c.r[0]);
                c.r[5] = 0x3;
                c.r[6] = 0x3;
                c.r[7] = 0x0;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.lr = 0x800c0750; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c08c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0750;
            }
            0x800c0750 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x800c0764; continue; }
                pc = 0x800c0758;
            }
            0x800c0758 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800c0760; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0760;
            }
            0x800c0760 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800c0764;
            }
            0x800c0764 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0774; continue; }
                pc = 0x800c076c;
            }
            0x800c076c => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x20));
                c.stw(c.r[29], c.r[0]);
                pc = 0x800c0774;
            }
            0x800c0774 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800c077c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c077c;
            }
            0x800c077c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.lr = 0x800c0784; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0784;
            }
            0x800c0784 => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x74));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x6c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x68));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x64));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c06bc: bad pc {:#x}", pc),
        }
    }
}

/// HandleOpenFileSupportRequest (0x800c07a4, 284 bytes)
#[inline(never)]
pub fn f_800c07a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c07a4) { return; }
        if c.prof { crate::prof::enter(c, 0x800c07a4); p = true; }
    }
    f_800c07a4_b(c, 0x800c07a4);
    if p { crate::prof::exit(c, 0x800c07a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c07a4_at(c: &mut Cpu, pc: u32) { f_800c07a4_b(c, pc) }
#[inline(always)]
fn f_800c07a4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c07a4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c07a4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c07a4 => { ins += 12;
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x5c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[29] = c.r[6] | c.r[6];
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800c07d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c07d4;
            }
            0x800c07d4 => { ins += 6;
                c.r[3] = 0x0;
                c.r[0] = 0xd2;
                c.stw(c.r[28], c.r[3]);
                c.r[3] = c.r[27] | c.r[27];
                c.stb(c.r[1].wrapping_add(0x18), c.r[0]);
                c.lr = 0x800c07ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_800caa20(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c07ec;
            }
            0x800c07ec => { ins += 5;
                c.r[0] = c.r[3].wrapping_add(0x41);
                c.stb(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[3] = c.r[27] | c.r[27];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x800c0800; { c.steps = st; c.instrs = ins; c.cr = cr; f_800caa20(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0800;
            }
            0x800c0800 => { ins += 5;
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.sth(c.r[1].wrapping_add(0x20), c.r[0]);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800c0814; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0814;
            }
            0x800c0814 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[5] = 0x40;
                c.lr = 0x800c0824; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0824;
            }
            0x800c0824 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x800c084c; continue; }
                pc = 0x800c082c;
            }
            0x800c082c => { ins += 2;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x800c0834; { c.steps = st; c.instrs = ins; c.cr = cr; f_800caa20(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0834;
            }
            0x800c0834 => { ins += 5;
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.lr = 0x800c0848; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0848;
            }
            0x800c0848 => { ins += 1;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x800c084c;
            }
            0x800c084c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c08a0; continue; }
                pc = 0x800c0854;
            }
            0x800c0854 => { ins += 8;
                c.r[0] = 0x0;
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.stw(c.r[29], c.r[0]);
                c.r[5] = 0x7;
                c.r[6] = 0x3;
                c.r[7] = 0x0;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.lr = 0x800c0874; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c08c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0874;
            }
            0x800c0874 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x800c0888; continue; }
                pc = 0x800c087c;
            }
            0x800c087c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800c0884; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0884;
            }
            0x800c0884 => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800c0888;
            }
            0x800c0888 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x20));
                c.stw(c.r[29], c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[30].wrapping_add(0x18));
                c.stw(c.r[28], c.r[0]);
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800c08a0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c08a0;
            }
            0x800c08a0 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.lr = 0x800c08a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c08a8;
            }
            0x800c08a8 => { ins += 6;
                c.r[3] = c.r[31] | c.r[31];
                { let mut ea = c.r[1].wrapping_add(0x5c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x74));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c07a4: bad pc {:#x}", pc),
        }
    }
}

/// TRKRequestSend (0x800c08c0, 480 bytes)
#[inline(never)]
pub fn f_800c08c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c08c0) { return; }
        if c.prof { crate::prof::enter(c, 0x800c08c0); p = true; }
    }
    f_800c08c0_b(c, 0x800c08c0);
    if p { crate::prof::exit(c, 0x800c08c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c08c0_at(c: &mut Cpu, pc: u32) { f_800c08c0_b(c, pc) }
#[inline(never)]
fn f_800c08c0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c08c0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c08c0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c08c0 => { ins += 15;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.r[0] = 0xffffffff;
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 21..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[22] = c.r[4] | c.r[4];
                c.r[4] = 0x80270000;
                c.r[21] = c.r[3] | c.r[3];
                c.r[23] = c.r[7] | c.r[7];
                c.r[27] = c.r[6].wrapping_add(0x1);
                c.r[31] = c.r[4].wrapping_add(0xfffffa40);
                c.r[30] = 0x0;
                c.r[24] = 0x1;
                c.stw(c.r[22], c.r[0]);
                pc = 0x800c0a5c; continue;
                pc = 0x800c08fc;
            }
            0x800c08fc => { ins += 4;
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[3] = 0x1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c090c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c090c;
            }
            0x800c090c => { ins += 2;
                c.r[3] = c.r[21] | c.r[21];
                c.lr = 0x800c0914; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be800(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0914;
            }
            0x800c0914 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if !cr_bit(cr, 2) { pc = 0x800c0a58; continue; }
                pc = 0x800c091c;
            }
            0x800c091c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[23], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c0928; continue; }
                pc = 0x800c0924;
            }
            0x800c0924 => { ins += 1;
                c.r[28] = 0x0;
                pc = 0x800c0928;
            }
            0x800c0928 => { ins += 1;
                c.lr = 0x800c092c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf1fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c092c;
            }
            0x800c092c => { ins += 4;
                c.stw(c.r[22], c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[22]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x800c0958; continue; }
                pc = 0x800c093c;
            }
            0x800c093c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[23], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c0928; continue; }
                pc = 0x800c0944;
            }
            0x800c0944 => { ins += 5;
                c.r[4] = 0x4c50000;
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.r[0] = c.r[4].wrapping_add(0xffffb3ec);
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x800c0928; continue; }
                pc = 0x800c0958;
            }
            0x800c0958 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0xffffffff);
                if cr_bit(cr, 2) { pc = 0x800c09b8; continue; }
                pc = 0x800c0960;
            }
            0x800c0960 => { ins += 2;
                c.r[24] = 0x0;
                c.lr = 0x800c0968; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0968;
            }
            0x800c0968 => { ins += 3;
                c.r[4] = 0x0;
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x800c0974; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0974;
            }
            0x800c0974 => { ins += 3;
                c.r[4] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[3] = c.r[29].wrapping_add(0x10);
                c.lr = 0x800c0980; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0504(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0980;
            }
            0x800c0980 => { ins += 7;
                c.r[26] = c.lbz_t(ins, c.r[29].wrapping_add(0x14));
                c.r[4] = c.r[31].wrapping_add(0x18);
                c.r[3] = 0x1;
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[26] | c.r[26];
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c099c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c099c;
            }
            0x800c099c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], 0x80);
                if !cr_bit(cr, 0) { pc = 0x800c09b8; continue; }
                pc = 0x800c09a4;
            }
            0x800c09a4 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[22]);
                c.lr = 0x800c09ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bf14c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c09ac;
            }
            0x800c09ac => { ins += 3;
                c.r[0] = 0xffffffff;
                c.stw(c.r[22], c.r[0]);
                pc = 0x800c0928; continue;
                pc = 0x800c09b8;
            }
            0x800c09b8 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[22]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xffffffff);
                if cr_bit(cr, 2) { pc = 0x800c0a58; continue; }
                pc = 0x800c09c4;
            }
            0x800c09c4 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x40);
                if !cr_bit(cr, 0) { pc = 0x800c09d4; continue; }
                pc = 0x800c09d0;
            }
            0x800c09d0 => { ins += 1;
                c.r[24] = 0x1;
                pc = 0x800c09d4;
            }
            0x800c09d4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c09fc; continue; }
                pc = 0x800c09dc;
            }
            0x800c09dc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[24], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c09fc; continue; }
                pc = 0x800c09e4;
            }
            0x800c09e4 => { ins += 6;
                c.r[25] = c.lbz_t(ins, c.r[29].wrapping_add(0x18));
                c.r[4] = c.r[31].wrapping_add(0x40);
                c.r[3] = 0x1;
                c.r[5] = c.r[25] | c.r[25];
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c09fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c09fc;
            }
            0x800c09fc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0a38; continue; }
                pc = 0x800c0a04;
            }
            0x800c0a04 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[24], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0a38; continue; }
                pc = 0x800c0a0c;
            }
            0x800c0a0c => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[26], 0x80);
                c.r[5] = c.r[26] | c.r[26];
                if !cr_bit(cr, 2) { pc = 0x800c0a20; continue; }
                pc = 0x800c0a18;
            }
            0x800c0a18 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[25], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c0a38; continue; }
                pc = 0x800c0a20;
            }
            0x800c0a20 => { ins += 5;
                c.r[6] = c.r[25] | c.r[25];
                c.r[4] = c.r[31].wrapping_add(0x54);
                c.r[3] = 0x8;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c0a34; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0a34;
            }
            0x800c0a34 => { ins += 1;
                c.r[24] = 0x1;
                pc = 0x800c0a38;
            }
            0x800c0a38 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0a48; continue; }
                pc = 0x800c0a40;
            }
            0x800c0a40 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[24], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c0a58; continue; }
                pc = 0x800c0a48;
            }
            0x800c0a48 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[22]);
                c.lr = 0x800c0a50; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0a50;
            }
            0x800c0a50 => { ins += 2;
                c.r[0] = 0xffffffff;
                c.stw(c.r[22], c.r[0]);
                pc = 0x800c0a58;
            }
            0x800c0a58 => { ins += 1;
                c.r[27] = c.r[27].wrapping_add(0xffffffff);
                pc = 0x800c0a5c;
            }
            0x800c0a5c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c0a78; continue; }
                pc = 0x800c0a64;
            }
            0x800c0a64 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[22]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x800c0a78; continue; }
                pc = 0x800c0a70;
            }
            0x800c0a70 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c08fc; continue; }
                pc = 0x800c0a78;
            }
            0x800c0a78 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[22]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xffffffff);
                if !cr_bit(cr, 2) { pc = 0x800c0a88; continue; }
                pc = 0x800c0a84;
            }
            0x800c0a84 => { ins += 1;
                c.r[30] = 0x800;
                pc = 0x800c0a88;
            }
            0x800c0a88 => { ins += 6;
                c.r[3] = c.r[30] | c.r[30];
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 21..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c08c0: bad pc {:#x}", pc),
        }
    }
}

/// TRKSuppAccessFile (0x800c0aa0, 544 bytes)
#[inline(never)]
pub fn f_800c0aa0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0aa0) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0aa0); p = true; }
    }
    f_800c0aa0_b(c, 0x800c0aa0);
    if p { crate::prof::exit(c, 0x800c0aa0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0aa0_at(c: &mut Cpu, pc: u32) { f_800c0aa0_b(c, pc) }
#[inline(never)]
fn f_800c0aa0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0aa0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0aa0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0aa0 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffff70); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x94), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x5c); for n in 19..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[24] = c.r[4] | c.r[4];
                cr_rc(&mut cr, c.xer, c.r[24]);
                c.r[23] = c.r[3] | c.r[3];
                c.r[25] = c.r[5] | c.r[5];
                c.r[26] = c.r[6] | c.r[6];
                c.r[27] = c.r[7] | c.r[7];
                c.r[28] = c.r[8] | c.r[8];
                if cr_bit(cr, 2) { pc = 0x800c0ad8; continue; }
                pc = 0x800c0acc;
            }
            0x800c0acc => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[25]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0ae0; continue; }
                pc = 0x800c0ad8;
            }
            0x800c0ad8 => { ins += 2;
                c.r[3] = 0x2;
                pc = 0x800c0cac; continue;
                pc = 0x800c0ae0;
            }
            0x800c0ae0 => { ins += 6;
                c.r[0] = 0x0;
                c.r[29] = 0x0;
                c.stw(c.r[26], c.r[0]);
                c.r[30] = 0x0;
                c.r[21] = 0x0;
                pc = 0x800c0c7c; continue;
                pc = 0x800c0af8;
            }
            0x800c0af8 => { ins += 4;
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.lr = 0x800c0b08; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0b08;
            }
            0x800c0b08 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[25]);
                c.r[3] = 0x800;
                c.r[0] = c.r[0].wrapping_sub(c.r[30]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x800);
                if cr_bit(cr, 1) { pc = 0x800c0b20; continue; }
                pc = 0x800c0b1c;
            }
            0x800c0b1c => { ins += 1;
                c.r[3] = c.r[0] | c.r[0];
                pc = 0x800c0b20;
            }
            0x800c0b20 => { ins += 4;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x0);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = 0xd0;
                if cr_bit(cr, 2) { pc = 0x800c0b34; continue; }
                pc = 0x800c0b30;
            }
            0x800c0b30 => { ins += 1;
                c.r[0] = 0xd1;
                pc = 0x800c0b34;
            }
            0x800c0b34 => { ins += 4;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x0);
                c.stb(c.r[1].wrapping_add(0x18), c.r[0]);
                c.r[0] = 0x40;
                if !cr_bit(cr, 2) { pc = 0x800c0b48; continue; }
                pc = 0x800c0b44;
            }
            0x800c0b44 => { ins += 1;
                c.r[0] = c.r[31].wrapping_add(0x40);
                pc = 0x800c0b48;
            }
            0x800c0b48 => { ins += 6;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[23]);
                c.sth(c.r[1].wrapping_add(0x20), c.r[31]);
                c.lr = 0x800c0b60; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0b60;
            }
            0x800c0b60 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[5] = 0x40;
                c.lr = 0x800c0b70; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0b70;
            }
            0x800c0b70 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x0);
                c.r[21] = c.r[3] | c.r[3];
                if !cr_bit(cr, 2) { pc = 0x800c0b98; continue; }
                pc = 0x800c0b7c;
            }
            0x800c0b7c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[21], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0b98; continue; }
                pc = 0x800c0b84;
            }
            0x800c0b84 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = c.r[24].wrapping_add(c.r[30]);
                c.lr = 0x800c0b94; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0b94;
            }
            0x800c0b94 => { ins += 1;
                c.r[21] = c.r[3] | c.r[3];
                pc = 0x800c0b98;
            }
            0x800c0b98 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[21], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0c70; continue; }
                pc = 0x800c0ba0;
            }
            0x800c0ba0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c0c64; continue; }
                pc = 0x800c0ba8;
            }
            0x800c0ba8 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x0);
                c.r[0] = 0x0;
                if cr_bit(cr, 2) { pc = 0x800c0bc0; continue; }
                pc = 0x800c0bb4;
            }
            0x800c0bb4 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0bc0; continue; }
                pc = 0x800c0bbc;
            }
            0x800c0bbc => { ins += 1;
                c.r[0] = 0x1;
                pc = 0x800c0bc0;
            }
            0x800c0bc0 => { ins += 8;
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x0);
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[5] = 0x5;
                c.r[0] = c.r[0].leading_zeros();
                c.r[6] = 0x3;
                c.r[7] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.lr = 0x800c0be0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c08c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0be0;
            }
            0x800c0be0 => { ins += 2;
                c.r[21] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[21]);
                if !cr_bit(cr, 2) { pc = 0x800c0bf4; continue; }
                pc = 0x800c0be8;
            }
            0x800c0be8 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800c0bf0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0bf0;
            }
            0x800c0bf0 => { ins += 1;
                c.r[22] = c.r[3] | c.r[3];
                pc = 0x800c0bf4;
            }
            0x800c0bf4 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[22].wrapping_add(0x20));
                cr_cmps(&mut cr, c.xer, 0, c.r[28], 0x0);
                c.r[19] = c.lhz_t(ins, c.r[22].wrapping_add(0x24));
                c.r[20] = c.r[0].rotate_left(0) & 0xff;
                if cr_bit(cr, 2) { pc = 0x800c0c44; continue; }
                pc = 0x800c0c08;
            }
            0x800c0c08 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[21], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0c44; continue; }
                pc = 0x800c0c10;
            }
            0x800c0c10 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[19], c.r[31]);
                if cr_bit(cr, 1) { pc = 0x800c0c44; continue; }
                pc = 0x800c0c18;
            }
            0x800c0c18 => { ins += 3;
                c.r[3] = c.r[22] | c.r[22];
                c.r[4] = 0x40;
                c.lr = 0x800c0c24; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bee44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0c24;
            }
            0x800c0c24 => { ins += 4;
                c.r[3] = c.r[22] | c.r[22];
                c.r[5] = c.r[19] | c.r[19];
                c.r[4] = c.r[24].wrapping_add(c.r[30]);
                c.lr = 0x800c0c34; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be934(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0c34;
            }
            0x800c0c34 => { ins += 3;
                c.r[21] = c.r[3] | c.r[3];
                cr_cmps(&mut cr, c.xer, 0, c.r[21], 0x302);
                if !cr_bit(cr, 2) { pc = 0x800c0c44; continue; }
                pc = 0x800c0c40;
            }
            0x800c0c40 => { ins += 1;
                c.r[21] = 0x0;
                pc = 0x800c0c44;
            }
            0x800c0c44 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[19], c.r[31]);
                if cr_bit(cr, 2) { pc = 0x800c0c54; continue; }
                pc = 0x800c0c4c;
            }
            0x800c0c4c => { ins += 2;
                c.r[31] = c.r[19] | c.r[19];
                c.r[29] = 0x1;
                pc = 0x800c0c54;
            }
            0x800c0c54 => { ins += 3;
                c.stw(c.r[26], c.r[20]);
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800c0c60; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0c60;
            }
            0x800c0c60 => { ins += 1;
                pc = 0x800c0c70; continue;
                pc = 0x800c0c64;
            }
            0x800c0c64 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.lr = 0x800c0c6c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be800(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0c6c;
            }
            0x800c0c6c => { ins += 1;
                c.r[21] = c.r[3] | c.r[3];
                pc = 0x800c0c70;
            }
            0x800c0c70 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.lr = 0x800c0c78; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0c78;
            }
            0x800c0c78 => { ins += 1;
                c.r[30] = c.r[30].wrapping_add(c.r[31]);
                pc = 0x800c0c7c;
            }
            0x800c0c7c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0ca4; continue; }
                pc = 0x800c0c84;
            }
            0x800c0c84 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[25]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x800c0ca4; continue; }
                pc = 0x800c0c90;
            }
            0x800c0c90 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[21], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c0ca4; continue; }
                pc = 0x800c0c98;
            }
            0x800c0c98 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[26]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c0af8; continue; }
                pc = 0x800c0ca4;
            }
            0x800c0ca4 => { ins += 2;
                c.stw(c.r[25], c.r[30]);
                c.r[3] = c.r[21] | c.r[21];
                pc = 0x800c0cac;
            }
            0x800c0cac => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x5c); for n in 19..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x94));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x90);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0aa0: bad pc {:#x}", pc),
        }
    }
}

/// TRKReleaseMutex (0x800c0cc0, 8 bytes)
#[inline(never)]
pub fn f_800c0cc0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0cc0) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0cc0); p = true; }
    }
    f_800c0cc0_b(c, 0x800c0cc0);
    if p { crate::prof::exit(c, 0x800c0cc0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0cc0_at(c: &mut Cpu, pc: u32) { f_800c0cc0_b(c, pc) }
#[inline(always)]
fn f_800c0cc0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0cc0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0cc0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0cc0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0cc0: bad pc {:#x}", pc),
        }
    }
}

/// TRKAcquireMutex (0x800c0cc8, 8 bytes)
#[inline(never)]
pub fn f_800c0cc8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0cc8) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0cc8); p = true; }
    }
    f_800c0cc8_b(c, 0x800c0cc8);
    if p { crate::prof::exit(c, 0x800c0cc8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0cc8_at(c: &mut Cpu, pc: u32) { f_800c0cc8_b(c, pc) }
#[inline(always)]
fn f_800c0cc8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0cc8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0cc8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0cc8 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0cc8: bad pc {:#x}", pc),
        }
    }
}

/// TRKInitializeMutex (0x800c0cd0, 8 bytes)
#[inline(never)]
pub fn f_800c0cd0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0cd0) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0cd0); p = true; }
    }
    f_800c0cd0_b(c, 0x800c0cd0);
    if p { crate::prof::exit(c, 0x800c0cd0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0cd0_at(c: &mut Cpu, pc: u32) { f_800c0cd0_b(c, pc) }
#[inline(always)]
fn f_800c0cd0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0cd0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0cd0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0cd0 => { ins += 2;
                c.r[3] = 0x0;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0cd0: bad pc {:#x}", pc),
        }
    }
}

/// TRKDoNotifyStopped (0x800c0cd8, 152 bytes)
#[inline(never)]
pub fn f_800c0cd8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0cd8) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0cd8); p = true; }
    }
    f_800c0cd8_b(c, 0x800c0cd8);
    if p { crate::prof::exit(c, 0x800c0cd8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0cd8_at(c: &mut Cpu, pc: u32) { f_800c0cd8_b(c, pc) }
#[inline(always)]
fn f_800c0cd8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0cd8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0cd8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0cd8 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.lr = 0x800c0cfc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bef44(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0cfc;
            }
            0x800c0cfc => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x800c0d54; continue; }
                pc = 0x800c0d04;
            }
            0x800c0d04 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800c0d24; continue; }
                pc = 0x800c0d08;
            }
            0x800c0d08 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x90);
                if !cr_bit(cr, 2) { pc = 0x800c0d1c; continue; }
                pc = 0x800c0d10;
            }
            0x800c0d10 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.lr = 0x800c0d18; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1740(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0d18;
            }
            0x800c0d18 => { ins += 1;
                pc = 0x800c0d24; continue;
                pc = 0x800c0d1c;
            }
            0x800c0d1c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.lr = 0x800c0d24; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c16bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0d24;
            }
            0x800c0d24 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[5] = 0x2;
                c.r[6] = 0x3;
                c.r[7] = 0x1;
                c.lr = 0x800c0d3c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c08c0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0d3c;
            }
            0x800c0d3c => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[31]);
                if !cr_bit(cr, 2) { pc = 0x800c0d4c; continue; }
                pc = 0x800c0d44;
            }
            0x800c0d44 => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800c0d4c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0d4c;
            }
            0x800c0d4c => { ins += 2;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.lr = 0x800c0d54; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beeb4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0d54;
            }
            0x800c0d54 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0cd8: bad pc {:#x}", pc),
        }
    }
}

/// TRK_flush_cache (0x800c0d70, 56 bytes)
#[inline(never)]
pub fn f_800c0d70(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0d70) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0d70); p = true; }
    }
    f_800c0d70_b(c, 0x800c0d70);
    if p { crate::prof::exit(c, 0x800c0d70); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0d70_at(c: &mut Cpu, pc: u32) { f_800c0d70_b(c, pc) }
#[inline(always)]
fn f_800c0d70_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0d70;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0d70, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0d70 => { ins += 5;
                c.r[5] = 0xffff0000;
                c.r[5] = c.r[5] | 0xfff1;
                c.r[5] = c.r[5] & c.r[3];
                c.r[3] = c.r[3].wrapping_sub(c.r[5]);
                c.r[4] = c.r[4].wrapping_add(c.r[3]);
                pc = 0x800c0d84;
            }
            0x800c0d84 => { ins += 7;
                // dcbst
                // dcbf
                // sync
                // icbi
                c.r[5] = c.addc(c.r[5], 0x8, 0);
                c.r[4] = c.addc(c.r[4], 0xfffffff8, 0);
                cr_rc(&mut cr, c.xer, c.r[4]);
                if !cr_bit(cr, 0) { pc = 0x800c0d84; continue; }
                pc = 0x800c0da0;
            }
            0x800c0da0 => { ins += 2;
                // isync
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0d70: bad pc {:#x}", pc),
        }
    }
}

/// TRK_fill_mem_800D6430 (0x800c0da8, 184 bytes)
#[inline(never)]
pub fn f_800c0da8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0da8) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0da8); p = true; }
    }
    f_800c0da8_b(c, 0x800c0da8);
    if p { crate::prof::exit(c, 0x800c0da8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0da8_at(c: &mut Cpu, pc: u32) { f_800c0da8_b(c, pc) }
#[inline(always)]
fn f_800c0da8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0da8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0da8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0da8 => { ins += 5;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x20);
                c.r[4] = c.r[4].rotate_left(0) & 0xff;
                c.r[6] = c.r[3].wrapping_add(0xffffffff);
                c.r[7] = c.r[4] | c.r[4];
                if cr_bit(cr, 0) { pc = 0x800c0e48; continue; }
                pc = 0x800c0dbc;
            }
            0x800c0dbc => { ins += 3;
                c.r[0] = !(c.r[6] | c.r[6]);
                c.r[3] = c.r[0].rotate_left(0) & 0x3;
                cr_rc(&mut cr, c.xer, c.r[3]);
                if cr_bit(cr, 2) { pc = 0x800c0dd8; continue; }
                pc = 0x800c0dc8;
            }
            0x800c0dc8 => { ins += 1;
                c.r[5] = c.r[5].wrapping_sub(c.r[3]);
                pc = 0x800c0dcc;
            }
            0x800c0dcc => { ins += 3;
                c.r[3] = c.addc(c.r[3], 0xffffffff, 0);
                cr_rc(&mut cr, c.xer, c.r[3]);
                { let ea = c.r[6].wrapping_add(0x1); c.stb(ea, c.r[7]); c.r[6] = ea; }
                if !cr_bit(cr, 2) { pc = 0x800c0dcc; continue; }
                pc = 0x800c0dd8;
            }
            0x800c0dd8 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[7], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c0df8; continue; }
                pc = 0x800c0de0;
            }
            0x800c0de0 => { ins += 6;
                c.r[3] = c.r[7].rotate_left(24) & 0xff000000;
                c.r[0] = c.r[7].rotate_left(16) & 0xffff0000;
                c.r[4] = c.r[7].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[3] | c.r[0];
                c.r[0] = c.r[4] | c.r[0];
                c.r[7] = c.r[7] | c.r[0];
                pc = 0x800c0df8;
            }
            0x800c0df8 => { ins += 3;
                c.r[4] = c.r[5].rotate_left(27) & 0x7ffffff;
                cr_rc(&mut cr, c.xer, c.r[4]);
                c.r[3] = c.r[6].wrapping_add(0xfffffffd);
                if cr_bit(cr, 2) { pc = 0x800c0e2c; continue; }
                pc = 0x800c0e04;
            }
            0x800c0e04 => { ins += 10;
                c.stw(c.r[3].wrapping_add(0x4), c.r[7]);
                c.r[4] = c.addc(c.r[4], 0xffffffff, 0);
                cr_rc(&mut cr, c.xer, c.r[4]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[7]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[7]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[7]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[7]);
                c.stw(c.r[3].wrapping_add(0x18), c.r[7]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[7]);
                { let ea = c.r[3].wrapping_add(0x20); c.stw(ea, c.r[7]); c.r[3] = ea; }
                if !cr_bit(cr, 2) { pc = 0x800c0e04; continue; }
                pc = 0x800c0e2c;
            }
            0x800c0e2c => { ins += 2;
                c.r[4] = c.r[5].rotate_left(30) & 0x7;
                cr_rc(&mut cr, c.xer, c.r[4]);
                if cr_bit(cr, 2) { pc = 0x800c0e40; continue; }
                pc = 0x800c0e34;
            }
            0x800c0e34 => { ins += 3;
                c.r[4] = c.addc(c.r[4], 0xffffffff, 0);
                cr_rc(&mut cr, c.xer, c.r[4]);
                { let ea = c.r[3].wrapping_add(0x4); c.stw(ea, c.r[7]); c.r[3] = ea; }
                if !cr_bit(cr, 2) { pc = 0x800c0e34; continue; }
                pc = 0x800c0e40;
            }
            0x800c0e40 => { ins += 2;
                c.r[6] = c.r[3].wrapping_add(0x3);
                c.r[5] = c.r[5].rotate_left(0) & 0x3;
                pc = 0x800c0e48;
            }
            0x800c0e48 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[5], 0x0);
                if cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800c0e50;
            }
            0x800c0e50 => { ins += 3;
                c.r[5] = c.addc(c.r[5], 0xffffffff, 0);
                cr_rc(&mut cr, c.xer, c.r[5]);
                { let ea = c.r[6].wrapping_add(0x1); c.stb(ea, c.r[7]); c.r[6] = ea; }
                if !cr_bit(cr, 2) { pc = 0x800c0e50; continue; }
                pc = 0x800c0e5c;
            }
            0x800c0e5c => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0da8: bad pc {:#x}", pc),
        }
    }
}

/// __TRK_get_MSR (0x800c0e60, 8 bytes)
#[inline(never)]
pub fn f_800c0e60(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0e60) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0e60); p = true; }
    }
    f_800c0e60_b(c, 0x800c0e60);
    if p { crate::prof::exit(c, 0x800c0e60); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0e60_at(c: &mut Cpu, pc: u32) { f_800c0e60_b(c, pc) }
#[inline(always)]
fn f_800c0e60_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0e60;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0e60, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0e60 => { ins += 2;
                c.r[3] = 0; // mfmsr
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0e60: bad pc {:#x}", pc),
        }
    }
}

/// __TRK_set_MSR (0x800c0e68, 8 bytes)
#[inline(never)]
pub fn f_800c0e68(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0e68) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0e68); p = true; }
    }
    f_800c0e68_b(c, 0x800c0e68);
    if p { crate::prof::exit(c, 0x800c0e68); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0e68_at(c: &mut Cpu, pc: u32) { f_800c0e68_b(c, pc) }
#[inline(always)]
fn f_800c0e68_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0e68;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0e68, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0e68 => { ins += 2;
                // mtmsr
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0e68: bad pc {:#x}", pc),
        }
    }
}

/// TRK_ppc_memcpy (0x800c0e70, 60 bytes)
#[inline(never)]
pub fn f_800c0e70(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0e70) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0e70); p = true; }
    }
    f_800c0e70_b(c, 0x800c0e70);
    if p { crate::prof::exit(c, 0x800c0e70); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0e70_at(c: &mut Cpu, pc: u32) { f_800c0e70_b(c, pc) }
#[inline(always)]
fn f_800c0e70_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0e70;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0e70, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0e70 => { ins += 2;
                c.r[8] = 0; // mfmsr
                c.r[10] = 0x0;
                pc = 0x800c0e78;
            }
            0x800c0e78 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[10], c.r[5]);
                if cr_bit(cr, 2) { pc = 0x800c0ea0; continue; }
                pc = 0x800c0e80;
            }
            0x800c0e80 => { ins += 8;
                // mtmsr
                // sync
                c.r[9] = c.lbz_t(ins, c.r[10].wrapping_add(c.r[4]));
                // mtmsr
                // sync
                c.stb(c.r[10].wrapping_add(c.r[3]), c.r[9]);
                c.r[10] = c.r[10].wrapping_add(0x1);
                pc = 0x800c0e78; continue;
                pc = 0x800c0ea0;
            }
            0x800c0ea0 => { ins += 3;
                // mtmsr
                // sync
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0e70: bad pc {:#x}", pc),
        }
    }
}

/// TRKInterruptHandler (0x800c0eac, 404 bytes)
#[inline(never)]
pub fn f_800c0eac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c0eac) { return; }
        if c.prof { crate::prof::enter(c, 0x800c0eac); p = true; }
    }
    f_800c0eac_b(c, 0x800c0eac);
    if p { crate::prof::exit(c, 0x800c0eac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c0eac_at(c: &mut Cpu, pc: u32) { f_800c0eac_b(c, pc) }
#[inline(always)]
fn f_800c0eac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c0eac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c0eac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c0eac => { ins += 18;
                c.mtspr(26, c.r[2]);
                c.mtspr(27, c.r[4]);
                c.r[4] = c.mfspr(275);
                c.r[2] = cr;
                c.mtspr(275, c.r[2]);
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe7dc;
                c.r[2] = c.lwz_t(ins, c.r[2].wrapping_add(0x8c));
                c.r[2] = c.r[2] | 0x8002;
                c.r[2] = c.r[2] ^ 0x8002;
                // sync
                // mtmsr
                // sync
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe7d8;
                c.sth(c.r[2], c.r[3]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x500);
                if !cr_bit(cr, 2) { pc = 0x800c0f74; continue; }
                pc = 0x800c0ef4;
            }
            0x800c0ef4 => { ins += 5;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe880;
                c.r[3] = c.mfspr(8);
                c.stw(c.r[2].wrapping_add(0x42c), c.r[3]);
                c.lr = 0x800c0f08; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c349c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0f08;
            }
            0x800c0f08 => { ins += 10;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe880;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0x42c));
                c.mtspr(8, c.r[3]);
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe7dc;
                c.r[2] = c.lwz_t(ins, c.r[2].wrapping_add(0xa0));
                c.r[2] = c.lbz_t(ins, c.r[2]);
                cr_cmps(&mut cr, c.xer, 0, c.r[2], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c0f58; continue; }
                pc = 0x800c0f30;
            }
            0x800c0f30 => { ins += 5;
                c.r[2] = 0x80310000;
                c.r[2] = c.r[2] | 0x3824;
                c.r[2] = c.lbz_t(ins, c.r[2].wrapping_add(0xc));
                cr_cmps(&mut cr, c.xer, 0, c.r[2], 0x1);
                if cr_bit(cr, 2) { pc = 0x800c0f58; continue; }
                pc = 0x800c0f44;
            }
            0x800c0f44 => { ins += 5;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe7dc;
                c.r[3] = 0x1;
                c.stb(c.r[2].wrapping_add(0x9c), c.r[3]);
                pc = 0x800c0f74; continue;
                pc = 0x800c0f58;
            }
            0x800c0f58 => { ins += 7;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xecb0;
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0x88));
                cr = (cr & 0x0) | (c.r[3] & 0xffffffff);
                c.r[3] = c.lwz_t(ins, c.r[2].wrapping_add(0xc));
                c.r[2] = c.lwz_t(ins, c.r[2].wrapping_add(0x8));
                // rfi
                pc = 0x800c0f74;
            }
            0x800c0f74 => { ins += 8;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe7d8;
                c.r[3] = c.lhz_t(ins, c.r[2]);
                c.r[2] = 0x80310000;
                c.r[2] = c.r[2] | 0x3824;
                c.r[2] = c.lbz_t(ins, c.r[2].wrapping_add(0xc));
                cr_cmps(&mut cr, c.xer, 0, c.r[2], 0x0);
                if !cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1040(c); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800c0f94;
            }
            0x800c0f94 => { ins += 18;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe880;
                c.stw(c.r[2], c.r[0]);
                c.stw(c.r[2].wrapping_add(0x4), c.r[1]);
                c.r[0] = c.mfspr(273);
                c.stw(c.r[2].wrapping_add(0x8), c.r[0]);
                c.sth(c.r[2].wrapping_add(0x2f8), c.r[3]);
                c.sth(c.r[2].wrapping_add(0x2fa), c.r[3]);
                c.r[0] = c.mfspr(274);
                c.stw(c.r[2].wrapping_add(0xc), c.r[0]);
                { let mut ea = c.r[2].wrapping_add(0x10); for n in 4..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.mfspr(26);
                c.r[28] = c.mfspr(8);
                c.r[29] = c.mfspr(275);
                c.r[30] = c.mfspr(9);
                c.r[31] = c.mfspr(1);
                { let mut ea = c.r[2].wrapping_add(0x80); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.lr = 0x800c0fdc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c2a10(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c0fdc;
            }
            0x800c0fdc => { ins += 25;
                c.r[2] = 0x80310000;
                c.r[2] = c.r[2] | 0x3824;
                c.r[3] = 0x1;
                c.stb(c.r[2].wrapping_add(0xc), c.r[3]);
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe7dc;
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x8c));
                // sync
                // mtmsr
                // sync
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x80));
                c.mtspr(8, c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x84));
                c.mtspr(9, c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x88));
                c.mtspr(1, c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x94));
                c.mtspr(18, c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x90));
                c.mtspr(19, c.r[0]);
                { let mut ea = c.r[2].wrapping_add(0xc); for n in 3..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[2]);
                c.r[1] = c.lwz_t(ins, c.r[2].wrapping_add(0x4));
                c.r[2] = c.lwz_t(ins, c.r[2].wrapping_add(0x8));
                { c.steps = st; c.instrs = ins; c.cr = cr; f_800c195c(c); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c0eac: bad pc {:#x}", pc),
        }
    }
}

/// TRKExceptionHandler (0x800c1040, 156 bytes)
#[inline(never)]
pub fn f_800c1040(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1040) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1040); p = true; }
    }
    f_800c1040_b(c, 0x800c1040);
    if p { crate::prof::exit(c, 0x800c1040); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1040_at(c: &mut Cpu, pc: u32) { f_800c1040_b(c, pc) }
#[inline(always)]
fn f_800c1040_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1040;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1040, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1040 => { ins += 8;
                c.r[2] = 0x80310000;
                c.r[2] = c.r[2] | 0x3824;
                c.sth(c.r[2].wrapping_add(0x8), c.r[3]);
                c.r[3] = c.mfspr(26);
                c.stw(c.r[2], c.r[3]);
                c.r[3] = c.lhz_t(ins, c.r[2].wrapping_add(0x8));
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x200);
                if cr_bit(cr, 2) { pc = 0x800c10ac; continue; }
                pc = 0x800c1060;
            }
            0x800c1060 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x300);
                if cr_bit(cr, 2) { pc = 0x800c10ac; continue; }
                pc = 0x800c1068;
            }
            0x800c1068 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x400);
                if cr_bit(cr, 2) { pc = 0x800c10ac; continue; }
                pc = 0x800c1070;
            }
            0x800c1070 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x600);
                if cr_bit(cr, 2) { pc = 0x800c10ac; continue; }
                pc = 0x800c1078;
            }
            0x800c1078 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x700);
                if cr_bit(cr, 2) { pc = 0x800c10ac; continue; }
                pc = 0x800c1080;
            }
            0x800c1080 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x800);
                if cr_bit(cr, 2) { pc = 0x800c10ac; continue; }
                pc = 0x800c1088;
            }
            0x800c1088 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1000);
                if cr_bit(cr, 2) { pc = 0x800c10ac; continue; }
                pc = 0x800c1090;
            }
            0x800c1090 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1100);
                if cr_bit(cr, 2) { pc = 0x800c10ac; continue; }
                pc = 0x800c1098;
            }
            0x800c1098 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1200);
                if cr_bit(cr, 2) { pc = 0x800c10ac; continue; }
                pc = 0x800c10a0;
            }
            0x800c10a0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1300);
                if cr_bit(cr, 2) { pc = 0x800c10ac; continue; }
                pc = 0x800c10a8;
            }
            0x800c10a8 => { ins += 1;
                pc = 0x800c10b8; continue;
                pc = 0x800c10ac;
            }
            0x800c10ac => { ins += 3;
                c.r[3] = c.mfspr(26);
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.mtspr(26, c.r[3]);
                pc = 0x800c10b8;
            }
            0x800c10b8 => { ins += 9;
                c.r[2] = 0x80310000;
                c.r[2] = c.r[2] | 0x3824;
                c.r[3] = 0x1;
                c.stb(c.r[2].wrapping_add(0xd), c.r[3]);
                c.r[3] = c.mfspr(275);
                cr = (cr & 0x0) | (c.r[3] & 0xffffffff);
                c.r[2] = c.mfspr(273);
                c.r[3] = c.mfspr(274);
                // rfi
                { c.steps = st; c.instrs = ins; c.cr = cr; f_800c10dc(c); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1040: bad pc {:#x}", pc),
        }
    }
}

/// TRKSwapAndGo (0x800c10dc, 196 bytes)
#[inline(never)]
pub fn f_800c10dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c10dc) { return; }
        if c.prof { crate::prof::enter(c, 0x800c10dc); p = true; }
    }
    f_800c10dc_b(c, 0x800c10dc);
    if p { crate::prof::exit(c, 0x800c10dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c10dc_at(c: &mut Cpu, pc: u32) { f_800c10dc_b(c, pc) }
#[inline(always)]
fn f_800c10dc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c10dc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c10dc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c10dc => { ins += 26;
                c.r[3] = 0x803f0000;
                c.r[3] = c.r[3] | 0xe7dc;
                { let mut ea = c.r[3]; for n in 0..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[0] = 0; // mfmsr
                c.stw(c.r[3].wrapping_add(0x8c), c.r[0]);
                c.r[0] = c.mfspr(8);
                c.stw(c.r[3].wrapping_add(0x80), c.r[0]);
                c.r[0] = c.mfspr(9);
                c.stw(c.r[3].wrapping_add(0x84), c.r[0]);
                c.r[0] = c.mfspr(1);
                c.stw(c.r[3].wrapping_add(0x88), c.r[0]);
                c.r[0] = c.mfspr(18);
                c.stw(c.r[3].wrapping_add(0x94), c.r[0]);
                c.r[0] = c.mfspr(19);
                c.stw(c.r[3].wrapping_add(0x90), c.r[0]);
                c.r[1] = 0xffff8002;
                c.r[1] = !(c.r[1] | c.r[1]);
                c.r[3] = 0; // mfmsr
                c.r[3] = c.r[3] & c.r[1];
                // mtmsr
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe7dc;
                c.r[2] = c.lwz_t(ins, c.r[2].wrapping_add(0xa0));
                c.r[2] = c.lbz_t(ins, c.r[2]);
                cr_cmps(&mut cr, c.xer, 0, c.r[2], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c1158; continue; }
                pc = 0x800c1144;
            }
            0x800c1144 => { ins += 5;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe7dc;
                c.r[3] = 0x1;
                c.stb(c.r[2].wrapping_add(0x9c), c.r[3]);
                { c.steps = st; c.instrs = ins; c.cr = cr; f_800c11a0(c); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; }
                pc = 0x800c1158;
            }
            0x800c1158 => { ins += 5;
                c.r[2] = 0x80310000;
                c.r[2] = c.r[2] | 0x3824;
                c.r[3] = 0x0;
                c.stb(c.r[2].wrapping_add(0xc), c.r[3]);
                c.lr = 0x800c116c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c2bc8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c116c;
            }
            0x800c116c => { ins += 13;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe880;
                { let mut ea = c.r[2].wrapping_add(0x80); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.mtspr(26, c.r[27]);
                c.mtspr(8, c.r[28]);
                cr = (cr & 0x0) | (c.r[29] & 0xffffffff);
                c.mtspr(9, c.r[30]);
                c.mtspr(1, c.r[31]);
                { let mut ea = c.r[2].wrapping_add(0xc); for n in 3..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[2]);
                c.r[1] = c.lwz_t(ins, c.r[2].wrapping_add(0x4));
                c.r[2] = c.lwz_t(ins, c.r[2].wrapping_add(0x8));
                // rfi
                { c.steps = st; c.instrs = ins; c.cr = cr; f_800c11a0(c); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c10dc: bad pc {:#x}", pc),
        }
    }
}

/// TRKInterruptHandlerEnableInterrupts (0x800c11a0, 84 bytes)
#[inline(never)]
pub fn f_800c11a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c11a0) { return; }
        if c.prof { crate::prof::enter(c, 0x800c11a0); p = true; }
    }
    f_800c11a0_b(c, 0x800c11a0);
    if p { crate::prof::exit(c, 0x800c11a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c11a0_at(c: &mut Cpu, pc: u32) { f_800c11a0_b(c, pc) }
#[inline(always)]
fn f_800c11a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c11a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c11a0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c11a0 => { ins += 21;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe7dc;
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x8c));
                // sync
                // mtmsr
                // sync
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x80));
                c.mtspr(8, c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x84));
                c.mtspr(9, c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x88));
                c.mtspr(1, c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x94));
                c.mtspr(18, c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[2].wrapping_add(0x90));
                c.mtspr(19, c.r[0]);
                { let mut ea = c.r[2].wrapping_add(0xc); for n in 3..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[2]);
                c.r[1] = c.lwz_t(ins, c.r[2].wrapping_add(0x4));
                c.r[2] = c.lwz_t(ins, c.r[2].wrapping_add(0x8));
                { c.steps = st; c.instrs = ins; c.cr = cr; f_800c195c(c); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c11a0: bad pc {:#x}", pc),
        }
    }
}

/// ReadFPSCR (0x800c11f4, 36 bytes)
#[inline(never)]
pub fn f_800c11f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c11f4) { return; }
        if c.prof { crate::prof::enter(c, 0x800c11f4); p = true; }
    }
    f_800c11f4_b(c, 0x800c11f4);
    if p { crate::prof::exit(c, 0x800c11f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c11f4_at(c: &mut Cpu, pc: u32) { f_800c11f4_b(c, pc) }
#[inline(always)]
fn f_800c11f4_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c11f4;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c11f4, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c11f4 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x20); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                c.f[31] = f64::from_bits(c.fpscr as u64);
                { let ea = c.r[3]; let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[31] = c.lfd(ea); }
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c11f4: bad pc {:#x}", pc),
        }
    }
}

/// WriteFPSCR (0x800c1218, 36 bytes)
#[inline(never)]
pub fn f_800c1218(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1218) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1218); p = true; }
    }
    f_800c1218_b(c, 0x800c1218);
    if p { crate::prof::exit(c, 0x800c1218); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1218_at(c: &mut Cpu, pc: u32) { f_800c1218_b(c, pc) }
#[inline(always)]
fn f_800c1218_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1218;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1218, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1218 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x20); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[3]; c.f[31] = c.lfd(ea); }
                c.fpscr = c.f[31].to_bits() as u32;
                { let ea = c.r[1].wrapping_add(0x20); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[31] = c.lfd(ea); }
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1218: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetAccessARAM (0x800c123c, 196 bytes)
#[inline(never)]
pub fn f_800c123c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c123c) { return; }
        if c.prof { crate::prof::enter(c, 0x800c123c); p = true; }
    }
    f_800c123c_b(c, 0x800c123c);
    if p { crate::prof::exit(c, 0x800c123c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c123c_at(c: &mut Cpu, pc: u32) { f_800c123c_b(c, pc) }
#[inline(always)]
fn f_800c123c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c123c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c123c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c123c => { ins += 22;
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = 0x80310000;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x0);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.r[31] = c.r[7].wrapping_add(0x3824);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.r[30] = 0x0;
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[29] = c.r[5] | c.r[5];
                c.r[6] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                c.r[9] = c.lwz_t(ins, c.r[31]);
                c.r[8] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                c.r[7] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                c.stw(c.r[1].wrapping_add(0x8), c.r[9]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[6]);
                c.stb(c.r[31].wrapping_add(0xd), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c129c; continue; }
                pc = 0x800c1294;
            }
            0x800c1294 => { ins += 1;
                c.lr = 0x800c1298; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3098(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1298;
            }
            0x800c1298 => { ins += 1;
                pc = 0x800c12a0; continue;
                pc = 0x800c129c;
            }
            0x800c129c => { ins += 1;
                c.lr = 0x800c12a0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c2eac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c12a0;
            }
            0x800c12a0 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0xd));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c12b8; continue; }
                pc = 0x800c12ac;
            }
            0x800c12ac => { ins += 3;
                c.r[0] = 0x0;
                c.r[30] = 0x702;
                c.stw(c.r[29], c.r[0]);
                pc = 0x800c12b8;
            }
            0x800c12b8 => { ins += 18;
                c.r[3] = 0x80310000;
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[7] = c.r[3].wrapping_add(0x3824);
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.stw(c.r[7], c.r[6]);
                c.stw(c.r[7].wrapping_add(0x4), c.r[5]);
                c.stw(c.r[7].wrapping_add(0x8), c.r[4]);
                c.stw(c.r[7].wrapping_add(0xc), c.r[0]);
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c123c: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetSetInputPendingPtr (0x800c1300, 16 bytes)
#[inline(never)]
pub fn f_800c1300(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1300) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1300); p = true; }
    }
    f_800c1300_b(c, 0x800c1300);
    if p { crate::prof::exit(c, 0x800c1300); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1300_at(c: &mut Cpu, pc: u32) { f_800c1300_b(c, pc) }
#[inline(always)]
fn f_800c1300_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1300;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1300, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1300 => { ins += 4;
                c.r[4] = 0x80400000;
                c.r[4] = c.r[4].wrapping_add(0xffffe7dc);
                c.stw(c.r[4].wrapping_add(0xa0), c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1300: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetStop (0x800c1310, 24 bytes)
#[inline(never)]
pub fn f_800c1310(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1310) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1310); p = true; }
    }
    f_800c1310_b(c, 0x800c1310);
    if p { crate::prof::exit(c, 0x800c1310); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1310_at(c: &mut Cpu, pc: u32) { f_800c1310_b(c, pc) }
#[inline(always)]
fn f_800c1310_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1310;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1310, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1310 => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[0] = 0x1;
                c.r[4] = c.r[3].wrapping_add(0xffffe7dc);
                c.r[3] = 0x0;
                c.stw(c.r[4].wrapping_add(0x98), c.r[0]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1310: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetSetStopped (0x800c1328, 16 bytes)
#[inline(never)]
pub fn f_800c1328(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1328) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1328); p = true; }
    }
    f_800c1328_b(c, 0x800c1328);
    if p { crate::prof::exit(c, 0x800c1328); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1328_at(c: &mut Cpu, pc: u32) { f_800c1328_b(c, pc) }
#[inline(always)]
fn f_800c1328_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1328;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1328, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1328 => { ins += 4;
                c.r[4] = 0x80400000;
                c.r[4] = c.r[4].wrapping_add(0xffffe7dc);
                c.stw(c.r[4].wrapping_add(0x98), c.r[3]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1328: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetStopped (0x800c1338, 16 bytes)
#[inline(never)]
pub fn f_800c1338(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1338) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1338); p = true; }
    }
    f_800c1338_b(c, 0x800c1338);
    if p { crate::prof::exit(c, 0x800c1338); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1338_at(c: &mut Cpu, pc: u32) { f_800c1338_b(c, pc) }
#[inline(always)]
fn f_800c1338_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1338;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1338, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1338 => { ins += 4;
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffe7dc);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x98));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1338: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetSupportRequest (0x800c1348, 512 bytes)
#[inline(never)]
pub fn f_800c1348(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1348) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1348); p = true; }
    }
    f_800c1348_b(c, 0x800c1348);
    if p { crate::prof::exit(c, 0x800c1348); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1348_at(c: &mut Cpu, pc: u32) { f_800c1348_b(c, pc) }
#[inline(never)]
fn f_800c1348_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1348;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1348, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1348 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[3].wrapping_add(0xffffe880);
                c.r[27] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0xd1);
                if cr_bit(cr, 2) { pc = 0x800c13a8; continue; }
                pc = 0x800c136c;
            }
            0x800c136c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0xd0);
                if cr_bit(cr, 2) { pc = 0x800c13a8; continue; }
                pc = 0x800c1374;
            }
            0x800c1374 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0xd2);
                if cr_bit(cr, 2) { pc = 0x800c13a8; continue; }
                pc = 0x800c137c;
            }
            0x800c137c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0xd3);
                if cr_bit(cr, 2) { pc = 0x800c13a8; continue; }
                pc = 0x800c1384;
            }
            0x800c1384 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0xd4);
                if cr_bit(cr, 2) { pc = 0x800c13a8; continue; }
                pc = 0x800c138c;
            }
            0x800c138c => { ins += 3;
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.r[4] = 0x4;
                c.lr = 0x800c1398; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be464(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1398;
            }
            0x800c1398 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800c13a0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be47c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c13a0;
            }
            0x800c13a0 => { ins += 2;
                c.r[3] = 0x0;
                pc = 0x800c1534; continue;
                pc = 0x800c13a8;
            }
            0x800c13a8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0xd2);
                if !cr_bit(cr, 2) { pc = 0x800c13fc; continue; }
                pc = 0x800c13b0;
            }
            0x800c13b0 => { ins += 8;
                c.r[3] = 0x80400000;
                c.r[6] = c.r[1].wrapping_add(0xc);
                c.r[4] = c.r[3].wrapping_add(0xffffe880);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x14));
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x10));
                c.r[5] = c.lwz_t(ins, c.r[4].wrapping_add(0x18));
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                c.lr = 0x800c13d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c07a4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c13d0;
            }
            0x800c13d0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.r[3] | c.r[3];
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c13f0; continue; }
                pc = 0x800c13e0;
            }
            0x800c13e0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c13f0; continue; }
                pc = 0x800c13e8;
            }
            0x800c13e8 => { ins += 2;
                c.r[0] = 0x1;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                pc = 0x800c13f0;
            }
            0x800c13f0 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                pc = 0x800c151c; continue;
                pc = 0x800c13fc;
            }
            0x800c13fc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0xd3);
                if !cr_bit(cr, 2) { pc = 0x800c1444; continue; }
                pc = 0x800c1404;
            }
            0x800c1404 => { ins += 5;
                c.r[3] = 0x80400000;
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.r[3] = c.r[3].wrapping_add(0xffffe880);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x10));
                c.lr = 0x800c1418; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c06bc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1418;
            }
            0x800c1418 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.r[3] | c.r[3];
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c1438; continue; }
                pc = 0x800c1428;
            }
            0x800c1428 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c1438; continue; }
                pc = 0x800c1430;
            }
            0x800c1430 => { ins += 2;
                c.r[0] = 0x1;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                pc = 0x800c1438;
            }
            0x800c1438 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                pc = 0x800c151c; continue;
                pc = 0x800c1444;
            }
            0x800c1444 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0xd4);
                if !cr_bit(cr, 2) { pc = 0x800c14b0; continue; }
                pc = 0x800c144c;
            }
            0x800c144c => { ins += 11;
                c.r[3] = 0x80400000;
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[29] = c.r[3].wrapping_add(0xffffe880);
                c.r[6] = c.r[1].wrapping_add(0xc);
                c.r[3] = c.lwz_t(ins, c.r[29].wrapping_add(0x14));
                c.r[0] = c.lwz_t(ins, c.r[29].wrapping_add(0x18));
                c.r[7] = c.lwz_t(ins, c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[29].wrapping_add(0x10));
                c.r[5] = c.r[0].rotate_left(0) & 0xff;
                c.stw(c.r[1].wrapping_add(0x8), c.r[7]);
                c.lr = 0x800c1478; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c05ac(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1478;
            }
            0x800c1478 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.r[3] | c.r[3];
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c1498; continue; }
                pc = 0x800c1488;
            }
            0x800c1488 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c1498; continue; }
                pc = 0x800c1490;
            }
            0x800c1490 => { ins += 2;
                c.r[0] = 0x1;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                pc = 0x800c1498;
            }
            0x800c1498 => { ins += 6;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.stw(c.r[31].wrapping_add(0xc), c.r[3]);
                c.r[3] = c.lwz_t(ins, c.r[29].wrapping_add(0x14));
                c.stw(c.r[3], c.r[0]);
                pc = 0x800c151c; continue;
                pc = 0x800c14b0;
            }
            0x800c14b0 => { ins += 12;
                c.r[3] = 0x80400000;
                c.r[0] = c.addc(!c.r[27], 0xd1, 1);
                c.r[29] = c.r[3].wrapping_add(0xffffe880);
                c.r[6] = c.r[1].wrapping_add(0xc);
                c.r[28] = c.lwz_t(ins, c.r[29].wrapping_add(0x14));
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.lwz_t(ins, c.r[29].wrapping_add(0x10));
                c.r[8] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[4] = c.lwz_t(ins, c.r[29].wrapping_add(0x18));
                c.r[5] = c.r[28] | c.r[28];
                c.r[7] = 0x1;
                c.lr = 0x800c14e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0aa0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c14e0;
            }
            0x800c14e0 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.r[3] | c.r[3];
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c1500; continue; }
                pc = 0x800c14f0;
            }
            0x800c14f0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c1500; continue; }
                pc = 0x800c14f8;
            }
            0x800c14f8 => { ins += 2;
                c.r[0] = 0x1;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                pc = 0x800c1500;
            }
            0x800c1500 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                cr_cmps(&mut cr, c.xer, 0, c.r[27], 0xd1);
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800c151c; continue; }
                pc = 0x800c1510;
            }
            0x800c1510 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[29].wrapping_add(0x18));
                c.r[4] = c.lwz_t(ins, c.r[28]);
                c.lr = 0x800c151c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c151c;
            }
            0x800c151c => { ins += 6;
                c.r[4] = 0x80400000;
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[4].wrapping_add(0xffffe880);
                c.r[4] = c.lwz_t(ins, c.r[5].wrapping_add(0x80));
                c.r[0] = c.r[4].wrapping_add(0x4);
                c.stw(c.r[5].wrapping_add(0x80), c.r[0]);
                pc = 0x800c1534;
            }
            0x800c1534 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1348: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetGetPC (0x800c1548, 16 bytes)
#[inline(never)]
pub fn f_800c1548(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1548) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1548); p = true; }
    }
    f_800c1548_b(c, 0x800c1548);
    if p { crate::prof::exit(c, 0x800c1548); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1548_at(c: &mut Cpu, pc: u32) { f_800c1548_b(c, pc) }
#[inline(always)]
fn f_800c1548_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1548;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1548, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1548 => { ins += 4;
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffe880);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x80));
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1548: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetStepOutOfRange (0x800c1558, 184 bytes)
#[inline(never)]
pub fn f_800c1558(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1558) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1558); p = true; }
    }
    f_800c1558_b(c, 0x800c1558);
    if p { crate::prof::exit(c, 0x800c1558); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1558_at(c: &mut Cpu, pc: u32) { f_800c1558_b(c, pc) }
#[inline(always)]
fn f_800c1558_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1558;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1558, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1558 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                if cr_bit(cr, 2) { pc = 0x800c1578; continue; }
                pc = 0x800c1570;
            }
            0x800c1570 => { ins += 2;
                c.r[3] = 0x703;
                pc = 0x800c15fc; continue;
                pc = 0x800c1578;
            }
            0x800c1578 => { ins += 13;
                c.r[6] = 0x80310000;
                c.r[5] = 0x80270000;
                c.r[31] = c.r[6].wrapping_add(0x3834);
                c.r[6] = 0x1;
                c.stw(c.r[31].wrapping_add(0xc), c.r[3]);
                c.r[0] = c.r[5].wrapping_add(0xfffffb70);
                c.r[3] = 0x1;
                c.stw(c.r[31].wrapping_add(0x10), c.r[4]);
                c.r[4] = c.r[0] | c.r[0];
                c.stw(c.r[31].wrapping_add(0x4), c.r[6]);
                c.stw(c.r[31], c.r[6]);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c15ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c15ac;
            }
            0x800c15ac => { ins += 8;
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                c.r[3] = c.r[3].wrapping_add(0xffffe880);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x1f8));
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x0);
                c.r[0] = c.r[0] | 0x400;
                c.stw(c.r[3].wrapping_add(0x1f8), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c15d4; continue; }
                pc = 0x800c15cc;
            }
            0x800c15cc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x10);
                if !cr_bit(cr, 2) { pc = 0x800c15e8; continue; }
                pc = 0x800c15d4;
            }
            0x800c15d4 => { ins += 5;
                c.r[3] = 0x80310000;
                c.r[4] = c.r[3].wrapping_add(0x3834);
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.stw(c.r[4].wrapping_add(0x8), c.r[0]);
                pc = 0x800c15e8;
            }
            0x800c15e8 => { ins += 5;
                c.r[3] = 0x80400000;
                c.r[0] = 0x0;
                c.r[4] = c.r[3].wrapping_add(0xffffe7dc);
                c.r[3] = 0x0;
                c.stw(c.r[4].wrapping_add(0x98), c.r[0]);
                pc = 0x800c15fc;
            }
            0x800c15fc => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1558: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetSingleStep (0x800c1610, 172 bytes)
#[inline(never)]
pub fn f_800c1610(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1610) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1610); p = true; }
    }
    f_800c1610_b(c, 0x800c1610);
    if p { crate::prof::exit(c, 0x800c1610); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1610_at(c: &mut Cpu, pc: u32) { f_800c1610_b(c, pc) }
#[inline(always)]
fn f_800c1610_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1610;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1610, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1610 => { ins += 6;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                if cr_bit(cr, 2) { pc = 0x800c1630; continue; }
                pc = 0x800c1628;
            }
            0x800c1628 => { ins += 2;
                c.r[3] = 0x703;
                pc = 0x800c16a8; continue;
                pc = 0x800c1630;
            }
            0x800c1630 => { ins += 12;
                c.r[5] = 0x80310000;
                c.r[4] = 0x80270000;
                c.r[31] = c.r[5].wrapping_add(0x3834);
                c.r[0] = 0x1;
                c.r[5] = 0x0;
                c.stw(c.r[31].wrapping_add(0x8), c.r[3]);
                c.r[4] = c.r[4].wrapping_add(0xfffffb70);
                c.r[3] = 0x1;
                c.stw(c.r[31].wrapping_add(0x4), c.r[5]);
                c.stw(c.r[31], c.r[0]);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c1660; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1660;
            }
            0x800c1660 => { ins += 8;
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                c.r[3] = c.r[3].wrapping_add(0xffffe880);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x1f8));
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x0);
                c.r[0] = c.r[0] | 0x400;
                c.stw(c.r[3].wrapping_add(0x1f8), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c1688; continue; }
                pc = 0x800c1680;
            }
            0x800c1680 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[4], 0x10);
                if !cr_bit(cr, 2) { pc = 0x800c1694; continue; }
                pc = 0x800c1688;
            }
            0x800c1688 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                pc = 0x800c1694;
            }
            0x800c1694 => { ins += 5;
                c.r[3] = 0x80400000;
                c.r[0] = 0x0;
                c.r[4] = c.r[3].wrapping_add(0xffffe7dc);
                c.r[3] = 0x0;
                c.stw(c.r[4].wrapping_add(0x98), c.r[0]);
                pc = 0x800c16a8;
            }
            0x800c16a8 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1610: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetAddExceptionInfo (0x800c16bc, 132 bytes)
#[inline(never)]
pub fn f_800c16bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c16bc) { return; }
        if c.prof { crate::prof::enter(c, 0x800c16bc); p = true; }
    }
    f_800c16bc_b(c, 0x800c16bc);
    if p { crate::prof::exit(c, 0x800c16bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c16bc_at(c: &mut Cpu, pc: u32) { f_800c16bc_b(c, pc) }
#[inline(always)]
fn f_800c16bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c16bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c16bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c16bc => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x5c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.lr = 0x800c16e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c16e0;
            }
            0x800c16e0 => { ins += 9;
                c.r[3] = 0x80310000;
                c.r[5] = 0x40;
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x3824));
                c.r[0] = 0x91;
                c.stw(c.r[1].wrapping_add(0xc), c.r[5]);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.stb(c.r[1].wrapping_add(0x10), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[4]);
                c.lr = 0x800c1704; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c25b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1704;
            }
            0x800c1704 => { ins += 10;
                c.r[3] = 0x80310000;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[3].wrapping_add(0x3824);
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lhz_t(ins, c.r[4].wrapping_add(0x8));
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.stw(c.r[1].wrapping_add(0x18), c.r[5]);
                c.r[5] = 0x40;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[0]);
                c.lr = 0x800c172c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c172c;
            }
            0x800c172c => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x64));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x5c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x60);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c16bc: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetAddStopInfo (0x800c1740, 140 bytes)
#[inline(never)]
pub fn f_800c1740(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1740) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1740); p = true; }
    }
    f_800c1740_b(c, 0x800c1740);
    if p { crate::prof::exit(c, 0x800c1740); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1740_at(c: &mut Cpu, pc: u32) { f_800c1740_b(c, pc) }
#[inline(always)]
fn f_800c1740_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1740;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1740, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1740 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x5c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.lr = 0x800c1764; { c.steps = st; c.instrs = ins; c.cr = cr; f_8000540c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1764;
            }
            0x800c1764 => { ins += 10;
                c.r[3] = 0x80400000;
                c.r[5] = 0x40;
                c.r[3] = c.r[3].wrapping_add(0xffffe880);
                c.r[0] = 0x90;
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x80));
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0xc), c.r[5]);
                c.stb(c.r[1].wrapping_add(0x10), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[4]);
                c.lr = 0x800c178c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c25b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c178c;
            }
            0x800c178c => { ins += 11;
                c.r[3] = 0x80400000;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = c.r[3].wrapping_add(0xffffe880);
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x2f8));
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.stw(c.r[1].wrapping_add(0x18), c.r[5]);
                c.r[5] = 0x40;
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[0]);
                c.lr = 0x800c17b8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bebb0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c17b8;
            }
            0x800c17b8 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x64));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x5c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x60);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1740: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetInterrupt (0x800c17cc, 400 bytes)
#[inline(never)]
pub fn f_800c17cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c17cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800c17cc); p = true; }
    }
    f_800c17cc_b(c, 0x800c17cc);
    if p { crate::prof::exit(c, 0x800c17cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c17cc_at(c: &mut Cpu, pc: u32) { f_800c17cc_b(c, pc) }
#[inline(never)]
fn f_800c17cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c17cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c17cc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c17cc => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = 0x0;
                c.r[0] = c.lwz_t(ins, c.r[3]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x5);
                if !cr_bit(cr, 0) { pc = 0x800c1940; continue; }
                pc = 0x800c17f0;
            }
            0x800c17f0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x3);
                if !cr_bit(cr, 0) { pc = 0x800c17fc; continue; }
                pc = 0x800c17f8;
            }
            0x800c17f8 => { ins += 1;
                pc = 0x800c1940; continue;
                pc = 0x800c17fc;
            }
            0x800c17fc => { ins += 5;
                c.r[3] = 0x80310000;
                c.r[4] = c.r[3].wrapping_add(0x3834);
                c.r[0] = c.lwz_t(ins, c.r[4]);
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c1914; continue; }
                pc = 0x800c1810;
            }
            0x800c1810 => { ins += 7;
                c.r[3] = 0x80400000;
                c.r[5] = 0x1;
                c.r[31] = c.r[3].wrapping_add(0xffffe880);
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x1f8));
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffbff;
                c.stw(c.r[31].wrapping_add(0x1f8), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c188c; continue; }
                pc = 0x800c182c;
            }
            0x800c182c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x2f8));
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0xd00);
                if !cr_bit(cr, 2) { pc = 0x800c188c; continue; }
                pc = 0x800c183c;
            }
            0x800c183c => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x4));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x1);
                if cr_bit(cr, 2) { pc = 0x800c186c; continue; }
                pc = 0x800c1848;
            }
            0x800c1848 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800c188c; continue; }
                pc = 0x800c184c;
            }
            0x800c184c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 0) { pc = 0x800c1858; continue; }
                pc = 0x800c1854;
            }
            0x800c1854 => { ins += 1;
                pc = 0x800c188c; continue;
                pc = 0x800c1858;
            }
            0x800c1858 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c188c; continue; }
                pc = 0x800c1864;
            }
            0x800c1864 => { ins += 2;
                c.r[5] = 0x0;
                pc = 0x800c188c; continue;
                pc = 0x800c186c;
            }
            0x800c186c => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[31].wrapping_add(0x80));
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0xc));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x800c188c; continue; }
                pc = 0x800c187c;
            }
            0x800c187c => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x10));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 1) { pc = 0x800c188c; continue; }
                pc = 0x800c1888;
            }
            0x800c1888 => { ins += 1;
                c.r[5] = 0x0;
                pc = 0x800c188c;
            }
            0x800c188c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c18a4; continue; }
                pc = 0x800c1894;
            }
            0x800c1894 => { ins += 4;
                c.r[3] = 0x80310000;
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x3834), c.r[0]);
                pc = 0x800c1914; continue;
                pc = 0x800c18a4;
            }
            0x800c18a4 => { ins += 9;
                c.r[4] = 0x80310000;
                c.r[0] = 0x1;
                c.r[5] = c.r[4].wrapping_add(0x3834);
                c.r[3] = 0x80270000;
                c.stw(c.r[5], c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xfffffb70);
                c.r[3] = 0x1;
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c18c8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c18c8;
            }
            0x800c18c8 => { ins += 8;
                c.r[3] = 0x80310000;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x1f8));
                c.r[3] = c.r[3].wrapping_add(0x3834);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[0] | 0x400;
                c.stw(c.r[31].wrapping_add(0x1f8), c.r[0]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c18f0; continue; }
                pc = 0x800c18e8;
            }
            0x800c18e8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x10);
                if !cr_bit(cr, 2) { pc = 0x800c1904; continue; }
                pc = 0x800c18f0;
            }
            0x800c18f0 => { ins += 5;
                c.r[3] = 0x80310000;
                c.r[4] = c.r[3].wrapping_add(0x3834);
                c.r[3] = c.lwz_t(ins, c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.stw(c.r[4].wrapping_add(0x8), c.r[0]);
                pc = 0x800c1904;
            }
            0x800c1904 => { ins += 4;
                c.r[3] = 0x80400000;
                c.r[0] = 0x0;
                c.r[3] = c.r[3].wrapping_add(0xffffe7dc);
                c.stw(c.r[3].wrapping_add(0x98), c.r[0]);
                pc = 0x800c1914;
            }
            0x800c1914 => { ins += 4;
                c.r[3] = 0x80310000;
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x3834));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c1940; continue; }
                pc = 0x800c1924;
            }
            0x800c1924 => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[0] = 0x1;
                c.r[4] = c.r[3].wrapping_add(0xffffe7dc);
                c.r[3] = 0x90;
                c.stw(c.r[4].wrapping_add(0x98), c.r[0]);
                c.lr = 0x800c193c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0cd8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c193c;
            }
            0x800c193c => { ins += 1;
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x800c1940;
            }
            0x800c1940 => { ins += 7;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[30] | c.r[30];
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c17cc: bad pc {:#x}", pc),
        }
    }
}

/// TRKPostInterruptEvent (0x800c195c, 172 bytes)
#[inline(never)]
pub fn f_800c195c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c195c) { return; }
        if c.prof { crate::prof::enter(c, 0x800c195c); p = true; }
    }
    f_800c195c_b(c, 0x800c195c);
    if p { crate::prof::exit(c, 0x800c195c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c195c_at(c: &mut Cpu, pc: u32) { f_800c195c_b(c, pc) }
#[inline(always)]
fn f_800c195c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c195c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c195c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c195c => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xffffe7dc);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x9c));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c1988; continue; }
                pc = 0x800c197c;
            }
            0x800c197c => { ins += 3;
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x9c), c.r[0]);
                pc = 0x800c19f8; continue;
                pc = 0x800c1988;
            }
            0x800c1988 => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffe880);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x2f8));
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0xd00);
                if cr_bit(cr, 2) { pc = 0x800c19b0; continue; }
                pc = 0x800c19a0;
            }
            0x800c19a0 => { ins += 1;
                if !cr_bit(cr, 0) { pc = 0x800c19e4; continue; }
                pc = 0x800c19a4;
            }
            0x800c19a4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x700);
                if cr_bit(cr, 2) { pc = 0x800c19b0; continue; }
                pc = 0x800c19ac;
            }
            0x800c19ac => { ins += 1;
                pc = 0x800c19e4; continue;
                pc = 0x800c19b0;
            }
            0x800c19b0 => { ins += 5;
                c.r[4] = 0x80400000;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[4].wrapping_add(0xffffe880);
                c.r[4] = c.lwz_t(ins, c.r[4].wrapping_add(0x80));
                c.lr = 0x800c19c4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c25b0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c19c4;
            }
            0x800c19c4 => { ins += 4;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[0] = c.r[3].wrapping_add(0xf0200000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c19dc; continue; }
                pc = 0x800c19d4;
            }
            0x800c19d4 => { ins += 2;
                c.r[4] = 0x5;
                pc = 0x800c19e8; continue;
                pc = 0x800c19dc;
            }
            0x800c19dc => { ins += 2;
                c.r[4] = 0x3;
                pc = 0x800c19e8; continue;
                pc = 0x800c19e4;
            }
            0x800c19e4 => { ins += 1;
                c.r[4] = 0x4;
                pc = 0x800c19e8;
            }
            0x800c19e8 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.lr = 0x800c19f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be464(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c19f0;
            }
            0x800c19f0 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.lr = 0x800c19f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be47c(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c19f8;
            }
            0x800c19f8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c195c: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetAccessExtended2 (0x800c1a08, 1080 bytes)
#[inline(never)]
pub fn f_800c1a08(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1a08) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1a08); p = true; }
    }
    f_800c1a08_b(c, 0x800c1a08);
    if p { crate::prof::exit(c, 0x800c1a08); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1a08_at(c: &mut Cpu, pc: u32) { f_800c1a08_b(c, pc) }
#[inline(never)]
fn f_800c1a08_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1a08;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1a08, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1a08 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffed0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x134), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xfc); for n in 19..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[23] = c.r[4] | c.r[4];
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], 0x1f);
                c.r[27] = c.r[3] | c.r[3];
                c.r[24] = c.r[5] | c.r[5];
                c.r[25] = c.r[6] | c.r[6];
                c.r[26] = c.r[7] | c.r[7];
                if !cr_bit(cr, 1) { pc = 0x800c1a3c; continue; }
                pc = 0x800c1a34;
            }
            0x800c1a34 => { ins += 2;
                c.r[3] = 0x701;
                pc = 0x800c1e2c; continue;
                pc = 0x800c1a3c;
            }
            0x800c1a3c => { ins += 45;
                c.r[3] = 0x80270000;
                c.r[5] = 0x80310000;
                c.r[29] = c.r[3].wrapping_add(0xfffffaf8);
                c.r[4] = 0x7c990000;
                c.r[0] = c.lwz_t(ins, c.r[29]);
                c.r[31] = c.r[5].wrapping_add(0x3824);
                c.r[8] = c.lwz_t(ins, c.r[29].wrapping_add(0x4));
                c.r[3] = 0x4e800000;
                c.r[7] = c.lwz_t(ins, c.r[29].wrapping_add(0x24));
                c.r[6] = c.r[4].wrapping_add(0xffffe2a6);
                c.stw(c.r[1].wrapping_add(0xc4), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x20);
                c.r[19] = c.lwz_t(ins, c.r[31]);
                c.r[30] = 0x0;
                c.r[22] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                c.r[5] = 0x90830000;
                c.stw(c.r[1].wrapping_add(0xc8), c.r[8]);
                c.r[3] = c.r[1].wrapping_add(0xc4);
                c.r[20] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                c.r[4] = 0x28;
                c.stw(c.r[1].wrapping_add(0xe8), c.r[7]);
                c.r[21] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                c.r[28] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[12] = c.lwz_t(ins, c.r[29].wrapping_add(0xc));
                c.r[11] = c.lwz_t(ins, c.r[29].wrapping_add(0x10));
                c.r[10] = c.lwz_t(ins, c.r[29].wrapping_add(0x14));
                c.r[9] = c.lwz_t(ins, c.r[29].wrapping_add(0x18));
                c.r[8] = c.lwz_t(ins, c.r[29].wrapping_add(0x1c));
                c.r[7] = c.lwz_t(ins, c.r[29].wrapping_add(0x20));
                c.stw(c.r[1].wrapping_add(0x14), c.r[19]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[20]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[21]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[22]);
                c.stb(c.r[31].wrapping_add(0xd), c.r[30]);
                c.stw(c.r[1].wrapping_add(0xcc), c.r[28]);
                c.stw(c.r[1].wrapping_add(0xd0), c.r[12]);
                c.stw(c.r[1].wrapping_add(0xd4), c.r[11]);
                c.stw(c.r[1].wrapping_add(0xd8), c.r[10]);
                c.stw(c.r[1].wrapping_add(0xdc), c.r[9]);
                c.stw(c.r[1].wrapping_add(0xe0), c.r[8]);
                c.stw(c.r[1].wrapping_add(0xe4), c.r[7]);
                c.stw(c.r[1].wrapping_add(0xc4), c.r[6]);
                c.stw(c.r[1].wrapping_add(0xc8), c.r[5]);
                c.stw(c.r[1].wrapping_add(0xe8), c.r[0]);
                c.lr = 0x800c1af0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1af0;
            }
            0x800c1af0 => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[12] = c.r[1].wrapping_add(0xc4);
                c.r[4] = c.r[3].wrapping_add(0xffffed44);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800c1b08; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800c1b08;
            }
            0x800c1b08 => { ins += 36;
                c.r[3] = 0x80270000;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[29] = c.r[3].wrapping_add(0xfffffaf8);
                c.r[4] = 0x7c990000;
                c.r[8] = c.lwz_t(ins, c.r[29]);
                c.r[3] = 0x4e800000;
                c.r[0] = c.lwz_t(ins, c.r[29].wrapping_add(0x4));
                c.r[30] = c.r[5] | 0xa0000000;
                c.r[7] = c.lwz_t(ins, c.r[29].wrapping_add(0x24));
                c.r[5] = c.r[4].wrapping_add(0xffffe3a6);
                c.stw(c.r[1].wrapping_add(0xa0), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x20);
                c.r[28] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[6] = 0x80830000;
                c.stw(c.r[1].wrapping_add(0x9c), c.r[8]);
                c.r[3] = c.r[1].wrapping_add(0x9c);
                c.r[12] = c.lwz_t(ins, c.r[29].wrapping_add(0xc));
                c.r[4] = 0x28;
                c.stw(c.r[1].wrapping_add(0xc0), c.r[7]);
                c.r[11] = c.lwz_t(ins, c.r[29].wrapping_add(0x10));
                c.r[10] = c.lwz_t(ins, c.r[29].wrapping_add(0x14));
                c.r[9] = c.lwz_t(ins, c.r[29].wrapping_add(0x18));
                c.r[8] = c.lwz_t(ins, c.r[29].wrapping_add(0x1c));
                c.r[7] = c.lwz_t(ins, c.r[29].wrapping_add(0x20));
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.stw(c.r[1].wrapping_add(0xa4), c.r[28]);
                c.stw(c.r[1].wrapping_add(0xa8), c.r[12]);
                c.stw(c.r[1].wrapping_add(0xac), c.r[11]);
                c.stw(c.r[1].wrapping_add(0xb0), c.r[10]);
                c.stw(c.r[1].wrapping_add(0xb4), c.r[9]);
                c.stw(c.r[1].wrapping_add(0xb8), c.r[8]);
                c.stw(c.r[1].wrapping_add(0xbc), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x9c), c.r[6]);
                c.stw(c.r[1].wrapping_add(0xa0), c.r[5]);
                c.stw(c.r[1].wrapping_add(0xc0), c.r[0]);
                c.lr = 0x800c1b98; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1b98;
            }
            0x800c1b98 => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[12] = c.r[1].wrapping_add(0x9c);
                c.r[4] = c.r[3].wrapping_add(0xffffed44);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800c1bb0; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800c1bb0;
            }
            0x800c1bb0 => { ins += 35;
                c.r[3] = 0x80270000;
                c.r[4] = 0x7c910000;
                c.r[29] = c.r[3].wrapping_add(0xfffffaf8);
                c.r[3] = 0x4e800000;
                c.r[8] = c.lwz_t(ins, c.r[29]);
                c.r[30] = 0x0;
                c.r[6] = c.lwz_t(ins, c.r[29].wrapping_add(0x4));
                c.r[5] = c.r[4].wrapping_add(0xffffe3a6);
                c.r[7] = c.lwz_t(ins, c.r[29].wrapping_add(0x24));
                c.r[0] = c.r[3].wrapping_add(0x20);
                c.stw(c.r[1].wrapping_add(0x78), c.r[6]);
                c.r[6] = 0x80830000;
                c.r[28] = c.lwz_t(ins, c.r[29].wrapping_add(0x8));
                c.r[3] = c.r[1].wrapping_add(0x74);
                c.stw(c.r[1].wrapping_add(0x74), c.r[8]);
                c.r[4] = 0x28;
                c.r[12] = c.lwz_t(ins, c.r[29].wrapping_add(0xc));
                c.stw(c.r[1].wrapping_add(0x98), c.r[7]);
                c.r[11] = c.lwz_t(ins, c.r[29].wrapping_add(0x10));
                c.r[10] = c.lwz_t(ins, c.r[29].wrapping_add(0x14));
                c.r[9] = c.lwz_t(ins, c.r[29].wrapping_add(0x18));
                c.r[8] = c.lwz_t(ins, c.r[29].wrapping_add(0x1c));
                c.r[7] = c.lwz_t(ins, c.r[29].wrapping_add(0x20));
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x7c), c.r[28]);
                c.stw(c.r[1].wrapping_add(0x80), c.r[12]);
                c.stw(c.r[1].wrapping_add(0x84), c.r[11]);
                c.stw(c.r[1].wrapping_add(0x88), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x8c), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x90), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x94), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x74), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x78), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x98), c.r[0]);
                c.lr = 0x800c1c3c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1c3c;
            }
            0x800c1c3c => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[12] = c.r[1].wrapping_add(0x74);
                c.r[4] = c.r[3].wrapping_add(0xffffed44);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800c1c54; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800c1c54;
            }
            0x800c1c54 => { ins += 7;
                c.r[0] = 0x0;
                c.r[30] = c.r[27].rotate_left(21) & 0xffe00000;
                c.stw(c.r[25], c.r[0]);
                c.r[29] = c.r[1].wrapping_add(0x4c);
                c.r[28] = c.r[1].wrapping_add(0x24);
                c.r[3] = 0x0;
                pc = 0x800c1ddc; continue;
                pc = 0x800c1c70;
            }
            0x800c1c70 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[26], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c1d20; continue; }
                pc = 0x800c1c78;
            }
            0x800c1c78 => { ins += 23;
                c.r[3] = 0x80270000;
                { let ea = c.r[3].wrapping_add(0xfffffb20); c.r[12] = c.lwz_t(ins, ea); c.r[3] = ea; }
                c.r[0] = c.r[30] | 0xe0030000;
                c.r[11] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[10] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                c.r[9] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                c.r[8] = c.lwz_t(ins, c.r[3].wrapping_add(0x10));
                c.r[7] = c.lwz_t(ins, c.r[3].wrapping_add(0x14));
                c.r[6] = c.lwz_t(ins, c.r[3].wrapping_add(0x18));
                c.r[5] = c.lwz_t(ins, c.r[3].wrapping_add(0x1c));
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x20));
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x24));
                c.stw(c.r[1].wrapping_add(0x4c), c.r[12]);
                c.stw(c.r[1].wrapping_add(0x50), c.r[11]);
                c.stw(c.r[1].wrapping_add(0x54), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x58), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x5c), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x60), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x64), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x68), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x6c), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x70), c.r[3]);
                if cr_bit(cr, 2) { pc = 0x800c1cd8; continue; }
                pc = 0x800c1cd4;
            }
            0x800c1cd4 => { ins += 1;
                c.r[0] = c.r[30] | 0xf0030000;
                pc = 0x800c1cd8;
            }
            0x800c1cd8 => { ins += 7;
                c.r[3] = 0x4e800000;
                c.stw(c.r[1].wrapping_add(0x4c), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x20);
                c.r[3] = c.r[29] | c.r[29];
                c.stw(c.r[1].wrapping_add(0x70), c.r[0]);
                c.r[4] = 0x28;
                c.lr = 0x800c1cf4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1cf4;
            }
            0x800c1cf4 => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[12] = c.r[1].wrapping_add(0x4c);
                c.r[4] = c.r[3].wrapping_add(0xffffed44);
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800c1d0c; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800c1d0c;
            }
            0x800c1d0c => { ins += 4;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[3] = c.r[24] | c.r[24];
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.lr = 0x800c1d1c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bec18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1d1c;
            }
            0x800c1d1c => { ins += 1;
                pc = 0x800c1dc8; continue;
                pc = 0x800c1d20;
            }
            0x800c1d20 => { ins += 3;
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.lr = 0x800c1d2c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be9cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1d2c;
            }
            0x800c1d2c => { ins += 24;
                c.r[3] = 0x80270000;
                { let ea = c.r[3].wrapping_add(0xfffffb20); c.r[12] = c.lwz_t(ins, ea); c.r[3] = ea; }
                cr_cmps(&mut cr, c.xer, 0, c.r[26], 0x0);
                c.r[0] = c.r[30] | 0xe0030000;
                c.r[11] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[10] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                c.r[9] = c.lwz_t(ins, c.r[3].wrapping_add(0xc));
                c.r[8] = c.lwz_t(ins, c.r[3].wrapping_add(0x10));
                c.r[7] = c.lwz_t(ins, c.r[3].wrapping_add(0x14));
                c.r[6] = c.lwz_t(ins, c.r[3].wrapping_add(0x18));
                c.r[5] = c.lwz_t(ins, c.r[3].wrapping_add(0x1c));
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x20));
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x24));
                c.stw(c.r[1].wrapping_add(0x24), c.r[12]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[11]);
                c.stw(c.r[1].wrapping_add(0x2c), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x30), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x34), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x38), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x3c), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x40), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x44), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x48), c.r[3]);
                if cr_bit(cr, 2) { pc = 0x800c1d90; continue; }
                pc = 0x800c1d8c;
            }
            0x800c1d8c => { ins += 1;
                c.r[0] = c.r[30] | 0xf0030000;
                pc = 0x800c1d90;
            }
            0x800c1d90 => { ins += 7;
                c.r[3] = 0x4e800000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x20);
                c.r[3] = c.r[28] | c.r[28];
                c.stw(c.r[1].wrapping_add(0x48), c.r[0]);
                c.r[4] = 0x28;
                c.lr = 0x800c1dac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1dac;
            }
            0x800c1dac => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[12] = c.r[1].wrapping_add(0x24);
                c.r[4] = c.r[3].wrapping_add(0xffffed44);
                c.r[3] = c.r[1].wrapping_add(0xc);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800c1dc4; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800c1dc4;
            }
            0x800c1dc4 => { ins += 1;
                c.r[3] = 0x0;
                pc = 0x800c1dc8;
            }
            0x800c1dc8 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[25]);
                c.r[30] = c.r[30].wrapping_add(0x200000);
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.r[0] = c.r[4].wrapping_add(0x8);
                c.stw(c.r[25], c.r[0]);
                pc = 0x800c1ddc;
            }
            0x800c1ddc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], c.r[23]);
                if cr_bit(cr, 1) { pc = 0x800c1dec; continue; }
                pc = 0x800c1de4;
            }
            0x800c1de4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c1c70; continue; }
                pc = 0x800c1dec;
            }
            0x800c1dec => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0xd));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c1e04; continue; }
                pc = 0x800c1df8;
            }
            0x800c1df8 => { ins += 3;
                c.r[0] = 0x0;
                c.r[3] = 0x702;
                c.stw(c.r[25], c.r[0]);
                pc = 0x800c1e04;
            }
            0x800c1e04 => { ins += 10;
                c.r[4] = 0x80310000;
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[7] = c.r[4].wrapping_add(0x3824);
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x20));
                c.stw(c.r[7], c.r[6]);
                c.stw(c.r[7].wrapping_add(0x4), c.r[5]);
                c.stw(c.r[7].wrapping_add(0x8), c.r[4]);
                c.stw(c.r[7].wrapping_add(0xc), c.r[0]);
                pc = 0x800c1e2c;
            }
            0x800c1e2c => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xfc); for n in 19..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x134));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x130);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1a08: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetAccessExtended1 (0x800c1e40, 368 bytes)
#[inline(never)]
pub fn f_800c1e40(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1e40) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1e40); p = true; }
    }
    f_800c1e40_b(c, 0x800c1e40);
    if p { crate::prof::exit(c, 0x800c1e40); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1e40_at(c: &mut Cpu, pc: u32) { f_800c1e40_b(c, pc) }
#[inline(always)]
fn f_800c1e40_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1e40;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1e40, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1e40 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x60);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[6] | c.r[6];
                if !cr_bit(cr, 1) { pc = 0x800c1e68; continue; }
                pc = 0x800c1e60;
            }
            0x800c1e60 => { ins += 2;
                c.r[3] = 0x701;
                pc = 0x800c1f98; continue;
                pc = 0x800c1e68;
            }
            0x800c1e68 => { ins += 15;
                c.r[6] = 0x80310000;
                c.r[0] = 0x0;
                c.r[31] = c.r[6].wrapping_add(0x3824);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[4]);
                c.r[6] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                c.r[10] = c.lwz_t(ins, c.r[31]);
                c.r[9] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                c.r[8] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                c.stb(c.r[31].wrapping_add(0xd), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[10]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[6]);
                c.stw(c.r[30], c.r[0]);
                if cr_bit(cr, 1) { pc = 0x800c1f58; continue; }
                pc = 0x800c1ea4;
            }
            0x800c1ea4 => { ins += 13;
                c.r[4] = c.r[4].wrapping_sub(c.r[3]);
                c.r[8] = 0x80400000;
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.r[4] = c.lwz_t(ins, c.r[30]);
                c.r[6] = c.r[0].rotate_left(2) & 0xfffffffc;
                cr_cmps(&mut cr, c.xer, 0, c.r[7], 0x0);
                c.r[4] = c.r[4].wrapping_add(c.r[6]);
                c.r[7] = c.r[8].wrapping_add(0xffffe880);
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[30], c.r[4]);
                c.r[4] = c.r[7].wrapping_add(c.r[3]);
                c.r[4] = c.r[4].wrapping_add(0x1a8);
                if cr_bit(cr, 2) { pc = 0x800c1ee8; continue; }
                pc = 0x800c1ed8;
            }
            0x800c1ed8 => { ins += 3;
                c.r[3] = c.r[5] | c.r[5];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x800c1ee4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beab4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1ee4;
            }
            0x800c1ee4 => { ins += 1;
                pc = 0x800c1f58; continue;
                pc = 0x800c1ee8;
            }
            0x800c1ee8 => { ins += 3;
                c.r[3] = c.r[7].wrapping_add(0x1ec);
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], c.r[3]);
                if cr_bit(cr, 1) { pc = 0x800c1f14; continue; }
                pc = 0x800c1ef4;
            }
            0x800c1ef4 => { ins += 5;
                c.r[6] = c.r[6].wrapping_add(0xfffffffc);
                c.r[3] = c.r[7].wrapping_add(0x1e8);
                c.r[6] = c.r[4].wrapping_add(c.r[6]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], c.r[3]);
                if cr_bit(cr, 0) { pc = 0x800c1f14; continue; }
                pc = 0x800c1f08;
            }
            0x800c1f08 => { ins += 3;
                c.r[3] = 0x80310000;
                c.r[6] = 0x1;
                c.stb(c.r[3].wrapping_add(0x3818), c.r[6]);
                pc = 0x800c1f14;
            }
            0x800c1f14 => { ins += 5;
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffe880);
                c.r[6] = c.r[3].wrapping_add(0x278);
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], c.r[6]);
                if cr_bit(cr, 1) { pc = 0x800c1f4c; continue; }
                pc = 0x800c1f28;
            }
            0x800c1f28 => { ins += 5;
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(0xfffffffc);
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[6]);
                if cr_bit(cr, 0) { pc = 0x800c1f4c; continue; }
                pc = 0x800c1f3c;
            }
            0x800c1f3c => { ins += 4;
                c.r[3] = 0x80310000;
                c.r[6] = 0x1;
                c.r[3] = c.r[3].wrapping_add(0x3818);
                c.stb(c.r[3].wrapping_add(0x1), c.r[6]);
                pc = 0x800c1f4c;
            }
            0x800c1f4c => { ins += 3;
                c.r[3] = c.r[5] | c.r[5];
                c.r[5] = c.r[0] | c.r[0];
                c.lr = 0x800c1f58; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be844(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c1f58;
            }
            0x800c1f58 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0xd));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c1f70; continue; }
                pc = 0x800c1f64;
            }
            0x800c1f64 => { ins += 3;
                c.r[0] = 0x0;
                c.r[3] = 0x702;
                c.stw(c.r[30], c.r[0]);
                pc = 0x800c1f70;
            }
            0x800c1f70 => { ins += 10;
                c.r[4] = 0x80310000;
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[7] = c.r[4].wrapping_add(0x3824);
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.stw(c.r[7], c.r[6]);
                c.stw(c.r[7].wrapping_add(0x4), c.r[5]);
                c.stw(c.r[7].wrapping_add(0x8), c.r[4]);
                c.stw(c.r[7].wrapping_add(0xc), c.r[0]);
                pc = 0x800c1f98;
            }
            0x800c1f98 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1e40: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetAccessFP (0x800c1fb0, 1292 bytes)
#[inline(never)]
pub fn f_800c1fb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c1fb0) { return; }
        if c.prof { crate::prof::enter(c, 0x800c1fb0); p = true; }
    }
    f_800c1fb0_b(c, 0x800c1fb0);
    if p { crate::prof::exit(c, 0x800c1fb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c1fb0_at(c: &mut Cpu, pc: u32) { f_800c1fb0_b(c, pc) }
#[inline(never)]
fn f_800c1fb0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c1fb0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c1fb0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c1fb0 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xffffff10); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xf4), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc0); for n in 20..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[4] | c.r[4];
                cr_cmpu(&mut cr, c.xer, 0, c.r[28], 0x21);
                c.r[20] = c.r[3] | c.r[3];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = c.r[7] | c.r[7];
                if !cr_bit(cr, 1) { pc = 0x800c1fe4; continue; }
                pc = 0x800c1fdc;
            }
            0x800c1fdc => { ins += 2;
                c.r[3] = 0x701;
                pc = 0x800c24a8; continue;
                pc = 0x800c1fe4;
            }
            0x800c1fe4 => { ins += 13;
                c.r[3] = 0x80310000;
                c.r[0] = 0x0;
                c.r[27] = c.r[3].wrapping_add(0x3824);
                c.r[3] = c.lwz_t(ins, c.r[27].wrapping_add(0xc));
                c.r[6] = c.lwz_t(ins, c.r[27]);
                c.r[5] = c.lwz_t(ins, c.r[27].wrapping_add(0x4));
                c.r[4] = c.lwz_t(ins, c.r[27].wrapping_add(0x8));
                c.stw(c.r[1].wrapping_add(0x10), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[3]);
                c.stb(c.r[27].wrapping_add(0xd), c.r[0]);
                c.lr = 0x800c2018; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0e60(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2018;
            }
            0x800c2018 => { ins += 2;
                c.r[3] = c.r[3] | 0x2000;
                c.lr = 0x800c2020; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0e68(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2020;
            }
            0x800c2020 => { ins += 10;
                c.r[0] = 0x0;
                c.r[21] = c.r[20] | c.r[20];
                c.stw(c.r[30], c.r[0]);
                c.r[26] = c.r[20].rotate_left(21) & 0xffe00000;
                c.r[25] = c.r[1].wrapping_add(0x98);
                c.r[24] = c.r[1].wrapping_add(0x48);
                c.r[23] = c.r[1].wrapping_add(0x70);
                c.r[22] = c.r[1].wrapping_add(0x20);
                c.r[3] = 0x0;
                pc = 0x800c2458; continue;
                pc = 0x800c2048;
            }
            0x800c2048 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2248; continue; }
                pc = 0x800c2050;
            }
            0x800c2050 => { ins += 24;
                c.r[3] = 0x80270000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[21], 0x20);
                c.r[12] = c.r[3].wrapping_add(0xfffffb48);
                c.r[11] = c.lwz_t(ins, c.r[12]);
                c.r[10] = c.lwz_t(ins, c.r[12].wrapping_add(0x4));
                c.r[9] = c.lwz_t(ins, c.r[12].wrapping_add(0x8));
                c.r[8] = c.lwz_t(ins, c.r[12].wrapping_add(0xc));
                c.r[7] = c.lwz_t(ins, c.r[12].wrapping_add(0x10));
                c.r[6] = c.lwz_t(ins, c.r[12].wrapping_add(0x14));
                c.r[5] = c.lwz_t(ins, c.r[12].wrapping_add(0x18));
                c.r[4] = c.lwz_t(ins, c.r[12].wrapping_add(0x1c));
                c.r[3] = c.lwz_t(ins, c.r[12].wrapping_add(0x20));
                c.r[0] = c.lwz_t(ins, c.r[12].wrapping_add(0x24));
                c.stw(c.r[1].wrapping_add(0x98), c.r[11]);
                c.stw(c.r[1].wrapping_add(0x9c), c.r[10]);
                c.stw(c.r[1].wrapping_add(0xa0), c.r[9]);
                c.stw(c.r[1].wrapping_add(0xa4), c.r[8]);
                c.stw(c.r[1].wrapping_add(0xa8), c.r[7]);
                c.stw(c.r[1].wrapping_add(0xac), c.r[6]);
                c.stw(c.r[1].wrapping_add(0xb0), c.r[5]);
                c.stw(c.r[1].wrapping_add(0xb4), c.r[4]);
                c.stw(c.r[1].wrapping_add(0xb8), c.r[3]);
                c.stw(c.r[1].wrapping_add(0xbc), c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x800c20f8; continue; }
                pc = 0x800c20b0;
            }
            0x800c20b0 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                c.r[0] = c.r[26] | 0xc8030000;
                if cr_bit(cr, 2) { pc = 0x800c20c0; continue; }
                pc = 0x800c20bc;
            }
            0x800c20bc => { ins += 1;
                c.r[0] = c.r[26] | 0xd8030000;
                pc = 0x800c20c0;
            }
            0x800c20c0 => { ins += 7;
                c.r[3] = 0x4e800000;
                c.stw(c.r[1].wrapping_add(0x98), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x20);
                c.r[3] = c.r[25] | c.r[25];
                c.stw(c.r[1].wrapping_add(0xbc), c.r[0]);
                c.r[4] = 0x28;
                c.lr = 0x800c20dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c20dc;
            }
            0x800c20dc => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[12] = c.r[1].wrapping_add(0x98);
                c.r[4] = c.r[3].wrapping_add(0xffffed44);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800c20f4; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800c20f4;
            }
            0x800c20f4 => { ins += 1;
                pc = 0x800c2234; continue;
                pc = 0x800c20f8;
            }
            0x800c20f8 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800c213c; continue; }
                pc = 0x800c20fc;
            }
            0x800c20fc => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2110; continue; }
                pc = 0x800c2104;
            }
            0x800c2104 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800c210c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c11f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c210c;
            }
            0x800c210c => { ins += 1;
                pc = 0x800c2118; continue;
                pc = 0x800c2110;
            }
            0x800c2110 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800c2118; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1218(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2118;
            }
            0x800c2118 => { ins += 9;
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[0] = 0x0;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[3] = 0xffffffff;
                c.r[0] = c.r[4] & c.r[0];
                c.r[3] = c.r[5] & c.r[3];
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                pc = 0x800c2234; continue;
                pc = 0x800c213c;
            }
            0x800c213c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[21], 0x21);
                if !cr_bit(cr, 2) { pc = 0x800c2234; continue; }
                pc = 0x800c2144;
            }
            0x800c2144 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c2154; continue; }
                pc = 0x800c214c;
            }
            0x800c214c => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x800c2154;
            }
            0x800c2154 => { ins += 24;
                c.r[3] = 0x80270000;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                c.r[12] = c.r[3].wrapping_add(0xfffffaf8);
                c.r[11] = c.lwz_t(ins, c.r[12]);
                c.r[10] = c.lwz_t(ins, c.r[12].wrapping_add(0x4));
                c.r[9] = c.lwz_t(ins, c.r[12].wrapping_add(0x8));
                c.r[8] = c.lwz_t(ins, c.r[12].wrapping_add(0xc));
                c.r[7] = c.lwz_t(ins, c.r[12].wrapping_add(0x10));
                c.r[6] = c.lwz_t(ins, c.r[12].wrapping_add(0x14));
                c.r[5] = c.lwz_t(ins, c.r[12].wrapping_add(0x18));
                c.r[4] = c.lwz_t(ins, c.r[12].wrapping_add(0x1c));
                c.r[3] = c.lwz_t(ins, c.r[12].wrapping_add(0x20));
                c.r[0] = c.lwz_t(ins, c.r[12].wrapping_add(0x24));
                c.stw(c.r[1].wrapping_add(0x48), c.r[11]);
                c.stw(c.r[1].wrapping_add(0x4c), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x50), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x54), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x58), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x5c), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x60), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x64), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x68), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x6c), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c21cc; continue; }
                pc = 0x800c21b4;
            }
            0x800c21b4 => { ins += 6;
                c.r[3] = 0x7c9f0000;
                c.r[0] = 0x90830000;
                c.r[3] = c.r[3].wrapping_add(0xfffffaa6);
                c.stw(c.r[1].wrapping_add(0x4c), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x48), c.r[3]);
                pc = 0x800c21e0; continue;
                pc = 0x800c21cc;
            }
            0x800c21cc => { ins += 5;
                c.r[3] = 0x7c9f0000;
                c.r[4] = 0x80830000;
                c.r[0] = c.r[3].wrapping_add(0xfffffba6);
                c.stw(c.r[1].wrapping_add(0x48), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x4c), c.r[0]);
                pc = 0x800c21e0;
            }
            0x800c21e0 => { ins += 6;
                c.r[4] = 0x4e800000;
                c.r[3] = c.r[24] | c.r[24];
                c.r[0] = c.r[4].wrapping_add(0x20);
                c.r[4] = 0x28;
                c.stw(c.r[1].wrapping_add(0x6c), c.r[0]);
                c.lr = 0x800c21f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c21f8;
            }
            0x800c21f8 => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[12] = c.r[1].wrapping_add(0x48);
                c.r[4] = c.r[3].wrapping_add(0xffffed44);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800c2210; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800c2210;
            }
            0x800c2210 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2234; continue; }
                pc = 0x800c2218;
            }
            0x800c2218 => { ins += 7;
                c.r[3] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[4] = 0x0;
                c.r[0] = 0xffffffff;
                c.r[3] = c.r[3] & c.r[0];
                c.r[0] = c.r[4] & c.r[4];
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x800c2234;
            }
            0x800c2234 => { ins += 4;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[29] | c.r[29];
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.lr = 0x800c2244; { c.steps = st; c.instrs = ins; c.cr = cr; f_800bec18(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2244;
            }
            0x800c2244 => { ins += 1;
                pc = 0x800c2444; continue;
                pc = 0x800c2248;
            }
            0x800c2248 => { ins += 3;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800c2254; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be9cc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2254;
            }
            0x800c2254 => { ins += 25;
                c.r[3] = 0x80270000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[21], 0x20);
                c.r[12] = c.r[3].wrapping_add(0xfffffb48);
                c.r[20] = 0x0;
                c.r[11] = c.lwz_t(ins, c.r[12]);
                c.r[10] = c.lwz_t(ins, c.r[12].wrapping_add(0x4));
                c.r[9] = c.lwz_t(ins, c.r[12].wrapping_add(0x8));
                c.r[8] = c.lwz_t(ins, c.r[12].wrapping_add(0xc));
                c.r[7] = c.lwz_t(ins, c.r[12].wrapping_add(0x10));
                c.r[6] = c.lwz_t(ins, c.r[12].wrapping_add(0x14));
                c.r[5] = c.lwz_t(ins, c.r[12].wrapping_add(0x18));
                c.r[4] = c.lwz_t(ins, c.r[12].wrapping_add(0x1c));
                c.r[3] = c.lwz_t(ins, c.r[12].wrapping_add(0x20));
                c.r[0] = c.lwz_t(ins, c.r[12].wrapping_add(0x24));
                c.stw(c.r[1].wrapping_add(0x70), c.r[11]);
                c.stw(c.r[1].wrapping_add(0x74), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x78), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x7c), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x80), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x84), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x88), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x8c), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x90), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x94), c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x800c2304; continue; }
                pc = 0x800c22b8;
            }
            0x800c22b8 => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                c.r[0] = c.r[26] | 0xc8030000;
                if cr_bit(cr, 2) { pc = 0x800c22c8; continue; }
                pc = 0x800c22c4;
            }
            0x800c22c4 => { ins += 1;
                c.r[0] = c.r[26] | 0xd8030000;
                pc = 0x800c22c8;
            }
            0x800c22c8 => { ins += 7;
                c.r[3] = 0x4e800000;
                c.stw(c.r[1].wrapping_add(0x70), c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0x20);
                c.r[3] = c.r[23] | c.r[23];
                c.stw(c.r[1].wrapping_add(0x94), c.r[0]);
                c.r[4] = 0x28;
                c.lr = 0x800c22e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c22e4;
            }
            0x800c22e4 => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[12] = c.r[1].wrapping_add(0x70);
                c.r[4] = c.r[3].wrapping_add(0xffffed44);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800c22fc; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800c22fc;
            }
            0x800c22fc => { ins += 2;
                c.r[20] = 0x0;
                pc = 0x800c2440; continue;
                pc = 0x800c2304;
            }
            0x800c2304 => { ins += 1;
                if !cr_bit(cr, 2) { pc = 0x800c2348; continue; }
                pc = 0x800c2308;
            }
            0x800c2308 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c231c; continue; }
                pc = 0x800c2310;
            }
            0x800c2310 => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800c2318; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c11f4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2318;
            }
            0x800c2318 => { ins += 1;
                pc = 0x800c2324; continue;
                pc = 0x800c231c;
            }
            0x800c231c => { ins += 2;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800c2324; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c1218(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2324;
            }
            0x800c2324 => { ins += 9;
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[0] = 0x0;
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[3] = 0xffffffff;
                c.r[0] = c.r[4] & c.r[0];
                c.r[3] = c.r[5] & c.r[3];
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                pc = 0x800c2440; continue;
                pc = 0x800c2348;
            }
            0x800c2348 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[21], 0x21);
                if !cr_bit(cr, 2) { pc = 0x800c2440; continue; }
                pc = 0x800c2350;
            }
            0x800c2350 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c2360; continue; }
                pc = 0x800c2358;
            }
            0x800c2358 => { ins += 2;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x800c2360;
            }
            0x800c2360 => { ins += 24;
                c.r[3] = 0x80270000;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                c.r[12] = c.r[3].wrapping_add(0xfffffaf8);
                c.r[11] = c.lwz_t(ins, c.r[12]);
                c.r[10] = c.lwz_t(ins, c.r[12].wrapping_add(0x4));
                c.r[9] = c.lwz_t(ins, c.r[12].wrapping_add(0x8));
                c.r[8] = c.lwz_t(ins, c.r[12].wrapping_add(0xc));
                c.r[7] = c.lwz_t(ins, c.r[12].wrapping_add(0x10));
                c.r[6] = c.lwz_t(ins, c.r[12].wrapping_add(0x14));
                c.r[5] = c.lwz_t(ins, c.r[12].wrapping_add(0x18));
                c.r[4] = c.lwz_t(ins, c.r[12].wrapping_add(0x1c));
                c.r[3] = c.lwz_t(ins, c.r[12].wrapping_add(0x20));
                c.r[0] = c.lwz_t(ins, c.r[12].wrapping_add(0x24));
                c.stw(c.r[1].wrapping_add(0x20), c.r[11]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[9]);
                c.stw(c.r[1].wrapping_add(0x2c), c.r[8]);
                c.stw(c.r[1].wrapping_add(0x30), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x34), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x38), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x3c), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x40), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c23d8; continue; }
                pc = 0x800c23c0;
            }
            0x800c23c0 => { ins += 6;
                c.r[3] = 0x7c9f0000;
                c.r[0] = 0x90830000;
                c.r[3] = c.r[3].wrapping_add(0xfffffaa6);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[3]);
                pc = 0x800c23ec; continue;
                pc = 0x800c23d8;
            }
            0x800c23d8 => { ins += 5;
                c.r[3] = 0x7c9f0000;
                c.r[4] = 0x80830000;
                c.r[0] = c.r[3].wrapping_add(0xfffffba6);
                c.stw(c.r[1].wrapping_add(0x20), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x800c23ec;
            }
            0x800c23ec => { ins += 6;
                c.r[4] = 0x4e800000;
                c.r[3] = c.r[22] | c.r[22];
                c.r[0] = c.r[4].wrapping_add(0x20);
                c.r[4] = 0x28;
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.lr = 0x800c2404; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2404;
            }
            0x800c2404 => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[12] = c.r[1].wrapping_add(0x20);
                c.r[4] = c.r[3].wrapping_add(0xffffed44);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800c241c; { c.steps = st; c.instrs = ins; c.cr = cr; (c.dispatch)(c, t); st = c.steps; ins = c.instrs; cr = c.cr; }; }
                pc = 0x800c241c;
            }
            0x800c241c => { ins += 3;
                cr_cmps(&mut cr, c.xer, 0, c.r[31], 0x0);
                c.r[20] = 0x0;
                if cr_bit(cr, 2) { pc = 0x800c2440; continue; }
                pc = 0x800c2428;
            }
            0x800c2428 => { ins += 6;
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[3] = 0xffffffff;
                c.r[0] = c.r[20] & c.r[20];
                c.r[3] = c.r[4] & c.r[3];
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                pc = 0x800c2440;
            }
            0x800c2440 => { ins += 1;
                c.r[3] = c.r[20] | c.r[20];
                pc = 0x800c2444;
            }
            0x800c2444 => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[30]);
                c.r[26] = c.r[26].wrapping_add(0x200000);
                c.r[21] = c.r[21].wrapping_add(0x1);
                c.r[0] = c.r[4].wrapping_add(0x8);
                c.stw(c.r[30], c.r[0]);
                pc = 0x800c2458;
            }
            0x800c2458 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[21], c.r[28]);
                if cr_bit(cr, 1) { pc = 0x800c2468; continue; }
                pc = 0x800c2460;
            }
            0x800c2460 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2048; continue; }
                pc = 0x800c2468;
            }
            0x800c2468 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[27].wrapping_add(0xd));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2480; continue; }
                pc = 0x800c2474;
            }
            0x800c2474 => { ins += 3;
                c.r[0] = 0x0;
                c.r[3] = 0x702;
                c.stw(c.r[30], c.r[0]);
                pc = 0x800c2480;
            }
            0x800c2480 => { ins += 10;
                c.r[4] = 0x80310000;
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[7] = c.r[4].wrapping_add(0x3824);
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.stw(c.r[7], c.r[6]);
                c.stw(c.r[7].wrapping_add(0x4), c.r[5]);
                c.stw(c.r[7].wrapping_add(0x8), c.r[4]);
                c.stw(c.r[7].wrapping_add(0xc), c.r[0]);
                pc = 0x800c24a8;
            }
            0x800c24a8 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xc0); for n in 20..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0xf4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xf0);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c1fb0: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetAccessDefault (0x800c24bc, 244 bytes)
#[inline(never)]
pub fn f_800c24bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c24bc) { return; }
        if c.prof { crate::prof::enter(c, 0x800c24bc); p = true; }
    }
    f_800c24bc_b(c, 0x800c24bc);
    if p { crate::prof::exit(c, 0x800c24bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c24bc_at(c: &mut Cpu, pc: u32) { f_800c24bc_b(c, pc) }
#[inline(always)]
fn f_800c24bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c24bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c24bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c24bc => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x24);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                if !cr_bit(cr, 1) { pc = 0x800c24e4; continue; }
                pc = 0x800c24dc;
            }
            0x800c24dc => { ins += 2;
                c.r[3] = 0x701;
                pc = 0x800c2598; continue;
                pc = 0x800c24e4;
            }
            0x800c24e4 => { ins += 22;
                c.r[6] = 0x80310000;
                c.r[4] = c.r[4].wrapping_sub(c.r[3]);
                c.r[30] = c.r[6].wrapping_add(0x3824);
                c.r[6] = 0x80400000;
                c.r[9] = c.lwz_t(ins, c.r[30].wrapping_add(0xc));
                c.r[8] = 0x0;
                c.r[12] = c.r[4].wrapping_add(0x1);
                c.r[11] = c.lwz_t(ins, c.r[30]);
                c.r[10] = c.lwz_t(ins, c.r[30].wrapping_add(0x4));
                cr_cmps(&mut cr, c.xer, 0, c.r[7], 0x0);
                c.r[7] = c.lwz_t(ins, c.r[30].wrapping_add(0x8));
                c.r[0] = c.r[12].rotate_left(2) & 0xfffffffc;
                c.stb(c.r[30].wrapping_add(0xd), c.r[8]);
                c.r[4] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[6].wrapping_add(0xffffe880);
                c.stw(c.r[1].wrapping_add(0x8), c.r[11]);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[10]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[7]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[9]);
                c.stw(c.r[31], c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c254c; continue; }
                pc = 0x800c253c;
            }
            0x800c253c => { ins += 3;
                c.r[3] = c.r[5] | c.r[5];
                c.r[5] = c.r[12] | c.r[12];
                c.lr = 0x800c2548; { c.steps = st; c.instrs = ins; c.cr = cr; f_800beab4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2548;
            }
            0x800c2548 => { ins += 1;
                pc = 0x800c2558; continue;
                pc = 0x800c254c;
            }
            0x800c254c => { ins += 3;
                c.r[3] = c.r[5] | c.r[5];
                c.r[5] = c.r[12] | c.r[12];
                c.lr = 0x800c2558; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be844(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2558;
            }
            0x800c2558 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[30].wrapping_add(0xd));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2570; continue; }
                pc = 0x800c2564;
            }
            0x800c2564 => { ins += 3;
                c.r[0] = 0x0;
                c.r[3] = 0x702;
                c.stw(c.r[31], c.r[0]);
                pc = 0x800c2570;
            }
            0x800c2570 => { ins += 10;
                c.r[4] = 0x80310000;
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[7] = c.r[4].wrapping_add(0x3824);
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.stw(c.r[7], c.r[6]);
                c.stw(c.r[7].wrapping_add(0x4), c.r[5]);
                c.stw(c.r[7].wrapping_add(0x8), c.r[4]);
                c.stw(c.r[7].wrapping_add(0xc), c.r[0]);
                pc = 0x800c2598;
            }
            0x800c2598 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c24bc: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetReadInstruction (0x800c25b0, 76 bytes)
#[inline(never)]
pub fn f_800c25b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c25b0) { return; }
        if c.prof { crate::prof::enter(c, 0x800c25b0); p = true; }
    }
    f_800c25b0_b(c, 0x800c25b0);
    if p { crate::prof::exit(c, 0x800c25b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c25b0_at(c: &mut Cpu, pc: u32) { f_800c25b0_b(c, pc) }
#[inline(always)]
fn f_800c25b0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c25b0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c25b0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c25b0 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x4;
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.lr = 0x800c25d4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c25fc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c25d4;
            }
            0x800c25d4 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c25ec; continue; }
                pc = 0x800c25dc;
            }
            0x800c25dc => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x4);
                if cr_bit(cr, 2) { pc = 0x800c25ec; continue; }
                pc = 0x800c25e8;
            }
            0x800c25e8 => { ins += 1;
                c.r[3] = 0x700;
                pc = 0x800c25ec;
            }
            0x800c25ec => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c25b0: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetAccessMemory (0x800c25fc, 332 bytes)
#[inline(never)]
pub fn f_800c25fc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c25fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800c25fc); p = true; }
    }
    f_800c25fc_b(c, 0x800c25fc);
    if p { crate::prof::exit(c, 0x800c25fc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c25fc_at(c: &mut Cpu, pc: u32) { f_800c25fc_b(c, pc) }
#[inline(always)]
fn f_800c25fc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c25fc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c25fc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c25fc => { ins += 22;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80310000;
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.r[0] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[6].wrapping_add(0x3824);
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                c.r[26] = c.r[3] | c.r[3];
                c.r[29] = c.r[7] | c.r[7];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                c.r[8] = c.lwz_t(ins, c.r[31]);
                c.r[6] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                c.r[5] = c.lwz_t(ins, c.r[31].wrapping_add(0x8));
                c.stw(c.r[1].wrapping_add(0x8), c.r[8]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[6]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[4]);
                c.stb(c.r[31].wrapping_add(0xd), c.r[0]);
                c.lr = 0x800c2654; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3344(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2654;
            }
            0x800c2654 => { ins += 5;
                c.r[0] = c.r[29].leading_zeros();
                c.r[4] = c.lwz_t(ins, c.r[28]);
                c.r[25] = c.r[3] | c.r[3];
                c.r[5] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.lr = 0x800c2668; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c2748(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2668;
            }
            0x800c2668 => { ins += 2;
                c.r[30] = c.r[3] | c.r[3];
                cr_rc(&mut cr, c.xer, c.r[30]);
                if cr_bit(cr, 2) { pc = 0x800c267c; continue; }
                pc = 0x800c2670;
            }
            0x800c2670 => { ins += 3;
                c.r[0] = 0x0;
                c.stw(c.r[28], c.r[0]);
                pc = 0x800c26f0; continue;
                pc = 0x800c267c;
            }
            0x800c267c => { ins += 1;
                c.lr = 0x800c2680; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0e60(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2680;
            }
            0x800c2680 => { ins += 8;
                c.r[4] = 0x80400000;
                cr_cmps(&mut cr, c.xer, 0, c.r[29], 0x0);
                c.r[4] = c.r[4].wrapping_add(0xffffe880);
                c.r[8] = c.r[3] | c.r[3];
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x1f8));
                c.r[0] = c.r[0].rotate_left(0) & 0x10;
                c.r[7] = c.r[8] | c.r[0];
                if cr_bit(cr, 2) { pc = 0x800c26b8; continue; }
                pc = 0x800c26a0;
            }
            0x800c26a0 => { ins += 5;
                c.r[5] = c.lwz_t(ins, c.r[28]);
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[25] | c.r[25];
                c.r[6] = c.r[8] | c.r[8];
                c.lr = 0x800c26b4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0e70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c26b4;
            }
            0x800c26b4 => { ins += 1;
                pc = 0x800c26f0; continue;
                pc = 0x800c26b8;
            }
            0x800c26b8 => { ins += 6;
                c.r[5] = c.lwz_t(ins, c.r[28]);
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[26] | c.r[26];
                c.r[6] = c.r[7] | c.r[7];
                c.r[7] = c.r[8] | c.r[8];
                c.lr = 0x800c26d0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0e70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c26d0;
            }
            0x800c26d0 => { ins += 3;
                c.r[4] = c.lwz_t(ins, c.r[28]);
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x800c26dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c26dc;
            }
            0x800c26dc => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], c.r[25]);
                if cr_bit(cr, 2) { pc = 0x800c26f0; continue; }
                pc = 0x800c26e4;
            }
            0x800c26e4 => { ins += 3;
                c.r[4] = c.lwz_t(ins, c.r[28]);
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x800c26f0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c26f0;
            }
            0x800c26f0 => { ins += 3;
                c.r[0] = c.lbz_t(ins, c.r[31].wrapping_add(0xd));
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2708; continue; }
                pc = 0x800c26fc;
            }
            0x800c26fc => { ins += 3;
                c.r[0] = 0x0;
                c.r[30] = 0x702;
                c.stw(c.r[28], c.r[0]);
                pc = 0x800c2708;
            }
            0x800c2708 => { ins += 16;
                c.r[3] = 0x80310000;
                c.r[6] = c.lwz_t(ins, c.r[1].wrapping_add(0x8));
                c.r[7] = c.r[3].wrapping_add(0x3824);
                c.r[5] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.r[4] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.stw(c.r[7], c.r[6]);
                c.stw(c.r[7].wrapping_add(0x4), c.r[5]);
                c.stw(c.r[7].wrapping_add(0x8), c.r[4]);
                c.stw(c.r[7].wrapping_add(0xc), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c25fc: bad pc {:#x}", pc),
        }
    }
}

/// TRKValidMemory32 (0x800c2748, 676 bytes)
#[inline(never)]
pub fn f_800c2748(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c2748) { return; }
        if c.prof { crate::prof::enter(c, 0x800c2748); p = true; }
    }
    f_800c2748_b(c, 0x800c2748);
    if p { crate::prof::exit(c, 0x800c2748); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c2748_at(c: &mut Cpu, pc: u32) { f_800c2748_b(c, pc) }
#[inline(never)]
fn f_800c2748_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c2748;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c2748, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c2748 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[4].wrapping_add(c.r[3]);
                c.r[27] = c.r[27].wrapping_add(0xffffffff);
                c.r[26] = c.r[5] | c.r[5];
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], c.r[3]);
                c.r[5] = 0x700;
                if !cr_bit(cr, 0) { pc = 0x800c2778; continue; }
                pc = 0x800c2770;
            }
            0x800c2770 => { ins += 2;
                c.r[3] = 0x700;
                pc = 0x800c29d8; continue;
                pc = 0x800c2778;
            }
            0x800c2778 => { ins += 6;
                c.r[4] = 0x80270000;
                c.r[6] = 0x0;
                c.r[31] = c.r[4].wrapping_add(0xfffffae8);
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 1) { pc = 0x800c29d4; continue; }
                pc = 0x800c2790;
            }
            0x800c2790 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x800c29d4; continue; }
                pc = 0x800c279c;
            }
            0x800c279c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[26], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c27b8; continue; }
                pc = 0x800c27a4;
            }
            0x800c27a4 => { ins += 5;
                c.r[0] = c.r[6].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.r[31].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x8));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c27dc; continue; }
                pc = 0x800c27b8;
            }
            0x800c27b8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[26], 0x1);
                if !cr_bit(cr, 2) { pc = 0x800c27e4; continue; }
                pc = 0x800c27c0;
            }
            0x800c27c0 => { ins += 7;
                c.r[4] = 0x80270000;
                c.r[0] = c.r[6].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.r[4].wrapping_add(0xfffffae8);
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0xc));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c27e4; continue; }
                pc = 0x800c27dc;
            }
            0x800c27dc => { ins += 2;
                c.r[5] = 0x700;
                pc = 0x800c29d4; continue;
                pc = 0x800c27e4;
            }
            0x800c27e4 => { ins += 7;
                c.r[4] = 0x80270000;
                c.r[29] = c.r[6].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.r[4].wrapping_add(0xfffffae8);
                c.r[5] = 0x0;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(c.r[29]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x800c28e0; continue; }
                pc = 0x800c2800;
            }
            0x800c2800 => { ins += 6;
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.r[6] = 0x700;
                c.r[30] = c.r[0].wrapping_add(c.r[3]);
                c.r[30] = c.r[30].wrapping_add(0xffffffff);
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[3]);
                if !cr_bit(cr, 0) { pc = 0x800c281c; continue; }
                pc = 0x800c2818;
            }
            0x800c2818 => { ins += 1;
                pc = 0x800c28dc; continue;
                pc = 0x800c281c;
            }
            0x800c281c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                c.r[5] = 0x0;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 1) { pc = 0x800c28dc; continue; }
                pc = 0x800c282c;
            }
            0x800c282c => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x800c28dc; continue; }
                pc = 0x800c2838;
            }
            0x800c2838 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[26], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c2854; continue; }
                pc = 0x800c2840;
            }
            0x800c2840 => { ins += 5;
                c.r[0] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x8));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2878; continue; }
                pc = 0x800c2854;
            }
            0x800c2854 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[26], 0x1);
                if !cr_bit(cr, 2) { pc = 0x800c2880; continue; }
                pc = 0x800c285c;
            }
            0x800c285c => { ins += 7;
                c.r[4] = 0x80270000;
                c.r[0] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.r[4].wrapping_add(0xfffffae8);
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0xc));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c2880; continue; }
                pc = 0x800c2878;
            }
            0x800c2878 => { ins += 2;
                c.r[6] = 0x700;
                pc = 0x800c28dc; continue;
                pc = 0x800c2880;
            }
            0x800c2880 => { ins += 7;
                c.r[4] = 0x80270000;
                c.r[28] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.r[4].wrapping_add(0xfffffae8);
                c.r[6] = 0x0;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(c.r[28]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x800c28ac; continue; }
                pc = 0x800c289c;
            }
            0x800c289c => { ins += 3;
                c.r[5] = c.r[26] | c.r[26];
                c.r[4] = c.r[0].wrapping_sub(c.r[3]);
                c.lr = 0x800c28a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c2748(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c28a8;
            }
            0x800c28a8 => { ins += 1;
                c.r[6] = c.r[3] | c.r[3];
                pc = 0x800c28ac;
            }
            0x800c28ac => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c28dc; continue; }
                pc = 0x800c28b4;
            }
            0x800c28b4 => { ins += 6;
                c.r[3] = 0x80270000;
                c.r[0] = c.r[3].wrapping_add(0xfffffae8);
                c.r[3] = c.r[0].wrapping_add(c.r[28]);
                c.r[3] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[3]);
                if !cr_bit(cr, 1) { pc = 0x800c28dc; continue; }
                pc = 0x800c28cc;
            }
            0x800c28cc => { ins += 3;
                c.r[5] = c.r[26] | c.r[26];
                c.r[4] = c.r[30].wrapping_sub(c.r[3]);
                c.lr = 0x800c28d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c2748(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c28d8;
            }
            0x800c28d8 => { ins += 1;
                c.r[6] = c.r[3] | c.r[3];
                pc = 0x800c28dc;
            }
            0x800c28dc => { ins += 1;
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x800c28e0;
            }
            0x800c28e0 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[5], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c29d4; continue; }
                pc = 0x800c28e8;
            }
            0x800c28e8 => { ins += 6;
                c.r[3] = 0x80270000;
                c.r[4] = c.r[3].wrapping_add(0xfffffae8);
                c.r[28] = c.r[4].wrapping_add(0x4);
                c.r[3] = c.lwz_t(ins, c.r[28].wrapping_add(c.r[29]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[27], c.r[3]);
                if !cr_bit(cr, 1) { pc = 0x800c29d4; continue; }
                pc = 0x800c2900;
            }
            0x800c2900 => { ins += 6;
                c.r[0] = c.r[27].wrapping_sub(c.r[3]);
                c.r[6] = 0x700;
                c.r[30] = c.r[0].wrapping_add(c.r[3]);
                c.r[30] = c.r[30].wrapping_add(0xffffffff);
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[3]);
                if !cr_bit(cr, 0) { pc = 0x800c291c; continue; }
                pc = 0x800c2918;
            }
            0x800c2918 => { ins += 1;
                pc = 0x800c29d0; continue;
                pc = 0x800c291c;
            }
            0x800c291c => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[31].wrapping_add(0x4));
                c.r[5] = 0x0;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 1) { pc = 0x800c29d0; continue; }
                pc = 0x800c292c;
            }
            0x800c292c => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[31]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x800c29d0; continue; }
                pc = 0x800c2938;
            }
            0x800c2938 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[26], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c2954; continue; }
                pc = 0x800c2940;
            }
            0x800c2940 => { ins += 5;
                c.r[0] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x8));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2978; continue; }
                pc = 0x800c2954;
            }
            0x800c2954 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[26], 0x1);
                if !cr_bit(cr, 2) { pc = 0x800c2980; continue; }
                pc = 0x800c295c;
            }
            0x800c295c => { ins += 7;
                c.r[4] = 0x80270000;
                c.r[0] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.r[4].wrapping_add(0xfffffae8);
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0xc));
                cr_cmps(&mut cr, c.xer, 0, c.r[0], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c2980; continue; }
                pc = 0x800c2978;
            }
            0x800c2978 => { ins += 2;
                c.r[6] = 0x700;
                pc = 0x800c29d0; continue;
                pc = 0x800c2980;
            }
            0x800c2980 => { ins += 7;
                c.r[4] = 0x80270000;
                c.r[27] = c.r[5].rotate_left(4) & 0xfffffff0;
                c.r[4] = c.r[4].wrapping_add(0xfffffae8);
                c.r[6] = 0x0;
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(c.r[27]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x800c29ac; continue; }
                pc = 0x800c299c;
            }
            0x800c299c => { ins += 3;
                c.r[5] = c.r[26] | c.r[26];
                c.r[4] = c.r[0].wrapping_sub(c.r[3]);
                c.lr = 0x800c29a8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c2748(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c29a8;
            }
            0x800c29a8 => { ins += 1;
                c.r[6] = c.r[3] | c.r[3];
                pc = 0x800c29ac;
            }
            0x800c29ac => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c29d0; continue; }
                pc = 0x800c29b4;
            }
            0x800c29b4 => { ins += 3;
                c.r[3] = c.lwz_t(ins, c.r[28].wrapping_add(c.r[27]));
                cr_cmpu(&mut cr, c.xer, 0, c.r[30], c.r[3]);
                if !cr_bit(cr, 1) { pc = 0x800c29d0; continue; }
                pc = 0x800c29c0;
            }
            0x800c29c0 => { ins += 3;
                c.r[5] = c.r[26] | c.r[26];
                c.r[4] = c.r[30].wrapping_sub(c.r[3]);
                c.lr = 0x800c29cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c2748(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c29cc;
            }
            0x800c29cc => { ins += 1;
                c.r[6] = c.r[3] | c.r[3];
                pc = 0x800c29d0;
            }
            0x800c29d0 => { ins += 1;
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x800c29d4;
            }
            0x800c29d4 => { ins += 1;
                c.r[3] = c.r[5] | c.r[5];
                pc = 0x800c29d8;
            }
            0x800c29d8 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c2748: bad pc {:#x}", pc),
        }
    }
}

/// TRKAccessFile (0x800c29f0, 8 bytes)
#[inline(never)]
pub fn f_800c29f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c29f0) { return; }
        if c.prof { crate::prof::enter(c, 0x800c29f0); p = true; }
    }
    f_800c29f0_b(c, 0x800c29f0);
    if p { crate::prof::exit(c, 0x800c29f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c29f0_at(c: &mut Cpu, pc: u32) { f_800c29f0_b(c, pc) }
#[inline(always)]
fn f_800c29f0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c29f0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c29f0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c29f0 => { ins += 2;
                // twi
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c29f0: bad pc {:#x}", pc),
        }
    }
}

/// TRKOpenFile (0x800c29f8, 8 bytes)
#[inline(never)]
pub fn f_800c29f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c29f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800c29f8); p = true; }
    }
    f_800c29f8_b(c, 0x800c29f8);
    if p { crate::prof::exit(c, 0x800c29f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c29f8_at(c: &mut Cpu, pc: u32) { f_800c29f8_b(c, pc) }
#[inline(always)]
fn f_800c29f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c29f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c29f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c29f8 => { ins += 2;
                // twi
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c29f8: bad pc {:#x}", pc),
        }
    }
}

/// TRKCloseFile (0x800c2a00, 8 bytes)
#[inline(never)]
pub fn f_800c2a00(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c2a00) { return; }
        if c.prof { crate::prof::enter(c, 0x800c2a00); p = true; }
    }
    f_800c2a00_b(c, 0x800c2a00);
    if p { crate::prof::exit(c, 0x800c2a00); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c2a00_at(c: &mut Cpu, pc: u32) { f_800c2a00_b(c, pc) }
#[inline(always)]
fn f_800c2a00_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c2a00;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c2a00, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c2a00 => { ins += 2;
                // twi
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c2a00: bad pc {:#x}", pc),
        }
    }
}

/// TRKPositionFile (0x800c2a08, 8 bytes)
#[inline(never)]
pub fn f_800c2a08(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c2a08) { return; }
        if c.prof { crate::prof::enter(c, 0x800c2a08); p = true; }
    }
    f_800c2a08_b(c, 0x800c2a08);
    if p { crate::prof::exit(c, 0x800c2a08); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c2a08_at(c: &mut Cpu, pc: u32) { f_800c2a08_b(c, pc) }
#[inline(always)]
fn f_800c2a08_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c2a08;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c2a08, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c2a08 => { ins += 2;
                // twi
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c2a08: bad pc {:#x}", pc),
        }
    }
}

/// TRKSaveExtended1Block (0x800c2a10, 440 bytes)
#[inline(never)]
pub fn f_800c2a10(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c2a10) { return; }
        if c.prof { crate::prof::enter(c, 0x800c2a10); p = true; }
    }
    f_800c2a10_b(c, 0x800c2a10);
    if p { crate::prof::exit(c, 0x800c2a10); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c2a10_at(c: &mut Cpu, pc: u32) { f_800c2a10_b(c, pc) }
#[inline(always)]
fn f_800c2a10_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c2a10;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c2a10, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c2a10 => { ins += 67;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe880;
                c.r[16] = 0; // mfsr
                c.r[17] = 0; // mfsr
                c.r[18] = 0; // mfsr
                c.r[19] = 0; // mfsr
                c.r[20] = 0; // mfsr
                c.r[21] = 0; // mfsr
                c.r[22] = 0; // mfsr
                c.r[23] = 0; // mfsr
                c.r[24] = 0; // mfsr
                c.r[25] = 0; // mfsr
                c.r[26] = 0; // mfsr
                c.r[27] = 0; // mfsr
                c.r[28] = 0; // mfsr
                c.r[29] = 0; // mfsr
                c.r[30] = 0; // mfsr
                c.r[31] = 0; // mfsr
                { let mut ea = c.r[2].wrapping_add(0x1a8); for n in 16..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[10] = 0; // mftb
                c.r[11] = 0; // mftb
                c.r[12] = c.mfspr(1008);
                c.r[13] = c.mfspr(1009);
                c.r[14] = c.mfspr(27);
                c.r[15] = c.mfspr(287);
                c.r[16] = c.mfspr(528);
                c.r[17] = c.mfspr(529);
                c.r[18] = c.mfspr(530);
                c.r[19] = c.mfspr(531);
                c.r[20] = c.mfspr(532);
                c.r[21] = c.mfspr(533);
                c.r[22] = c.mfspr(534);
                c.r[23] = c.mfspr(535);
                c.r[24] = c.mfspr(536);
                c.r[25] = c.mfspr(537);
                c.r[26] = c.mfspr(538);
                c.r[27] = c.mfspr(539);
                c.r[28] = c.mfspr(540);
                c.r[29] = c.mfspr(541);
                c.r[30] = c.mfspr(542);
                c.r[31] = c.mfspr(543);
                { let mut ea = c.r[2].wrapping_add(0x1e8); for n in 10..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[22] = c.mfspr(25);
                c.r[23] = c.mfspr(19);
                c.r[24] = c.mfspr(18);
                c.r[25] = c.mfspr(272);
                c.r[26] = c.mfspr(273);
                c.r[27] = c.mfspr(274);
                c.r[28] = c.mfspr(275);
                c.r[29] = 0x0;
                c.r[30] = c.mfspr(1010);
                c.r[31] = c.mfspr(282);
                { let mut ea = c.r[2].wrapping_add(0x25c); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[20] = c.mfspr(912);
                c.r[21] = c.mfspr(913);
                c.r[22] = c.mfspr(914);
                c.r[23] = c.mfspr(915);
                c.r[24] = c.mfspr(916);
                c.r[25] = c.mfspr(917);
                c.r[26] = c.mfspr(918);
                c.r[27] = c.mfspr(919);
                c.r[28] = c.mfspr(920);
                c.r[29] = c.mfspr(921);
                c.r[30] = c.mfspr(922);
                c.r[31] = c.mfspr(923);
                { let mut ea = c.r[2].wrapping_add(0x2fc); for n in 20..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x800c2b60; continue;
                pc = 0x800c2b1c;
            }
            0x800c2b1c => { ins += 17;
                c.r[16] = c.mfspr(928);
                c.r[17] = c.mfspr(935);
                c.r[18] = c.mfspr(936);
                c.r[19] = c.mfspr(937);
                c.r[20] = c.mfspr(938);
                c.r[21] = c.mfspr(939);
                c.r[22] = c.mfspr(940);
                c.r[23] = c.mfspr(941);
                c.r[24] = c.mfspr(942);
                c.r[25] = c.mfspr(943);
                c.r[26] = c.mfspr(944);
                c.r[27] = c.mfspr(951);
                c.r[28] = c.mfspr(959);
                c.r[29] = c.mfspr(1014);
                c.r[30] = c.mfspr(1015);
                c.r[31] = c.mfspr(1023);
                { let mut ea = c.r[2].wrapping_add(0x2b8); for n in 16..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x800c2b60;
            }
            0x800c2b60 => { ins += 15;
                c.r[19] = c.mfspr(1013);
                c.r[20] = c.mfspr(953);
                c.r[21] = c.mfspr(954);
                c.r[22] = c.mfspr(957);
                c.r[23] = c.mfspr(958);
                c.r[24] = c.mfspr(955);
                c.r[25] = c.mfspr(952);
                c.r[26] = c.mfspr(956);
                c.r[27] = c.mfspr(1020);
                c.r[28] = c.mfspr(1021);
                c.r[29] = c.mfspr(1022);
                c.r[30] = c.mfspr(1019);
                c.r[31] = c.mfspr(1017);
                { let mut ea = c.r[2].wrapping_add(0x284); for n in 19..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800c2b9c;
            }
            0x800c2b9c => { ins += 11;
                c.r[25] = c.mfspr(976);
                c.r[26] = c.mfspr(977);
                c.r[27] = c.mfspr(978);
                c.r[28] = c.mfspr(979);
                c.r[29] = c.mfspr(980);
                c.r[30] = c.mfspr(981);
                c.r[31] = c.mfspr(982);
                { let mut ea = c.r[2].wrapping_add(0x240); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.mfspr(22);
                c.stw(c.r[2].wrapping_add(0x278), c.r[31]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c2a10: bad pc {:#x}", pc),
        }
    }
}

/// TRKRestoreExtended1Block (0x800c2bc8, 440 bytes)
#[inline(never)]
pub fn f_800c2bc8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c2bc8) { return; }
        if c.prof { crate::prof::enter(c, 0x800c2bc8); p = true; }
    }
    f_800c2bc8_b(c, 0x800c2bc8);
    if p { crate::prof::exit(c, 0x800c2bc8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c2bc8_at(c: &mut Cpu, pc: u32) { f_800c2bc8_b(c, pc) }
#[inline(always)]
fn f_800c2bc8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c2bc8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c2bc8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c2bc8 => { ins += 11;
                c.r[2] = 0x803f0000;
                c.r[2] = c.r[2] | 0xe880;
                c.r[5] = 0x80310000;
                c.r[5] = c.r[5] | 0x3818;
                c.r[3] = c.lbz_t(ins, c.r[5]);
                c.r[6] = c.lbz_t(ins, c.r[5].wrapping_add(0x1));
                c.r[0] = 0x0;
                c.stb(c.r[5], c.r[0]);
                c.stb(c.r[5].wrapping_add(0x1), c.r[0]);
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2c04; continue; }
                pc = 0x800c2bf4;
            }
            0x800c2bf4 => { ins += 4;
                c.r[24] = c.lwz_t(ins, c.r[2].wrapping_add(0x1e8));
                c.r[25] = c.lwz_t(ins, c.r[2].wrapping_add(0x1ec));
                c.mtspr(284, c.r[24]);
                c.mtspr(285, c.r[25]);
                pc = 0x800c2c04;
            }
            0x800c2c04 => { ins += 13;
                { let mut ea = c.r[2].wrapping_add(0x2fc); for n in 20..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.mtspr(912, c.r[20]);
                c.mtspr(913, c.r[21]);
                c.mtspr(914, c.r[22]);
                c.mtspr(915, c.r[23]);
                c.mtspr(916, c.r[24]);
                c.mtspr(917, c.r[25]);
                c.mtspr(918, c.r[26]);
                c.mtspr(919, c.r[27]);
                c.mtspr(920, c.r[28]);
                c.mtspr(922, c.r[30]);
                c.mtspr(923, c.r[31]);
                pc = 0x800c2c50; continue;
                pc = 0x800c2c38;
            }
            0x800c2c38 => { ins += 6;
                { let mut ea = c.r[2].wrapping_add(0x2e0); for n in 26..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.mtspr(944, c.r[26]);
                c.mtspr(951, c.r[27]);
                c.mtspr(1014, c.r[29]);
                c.mtspr(1015, c.r[30]);
                c.mtspr(1023, c.r[31]);
                pc = 0x800c2c50;
            }
            0x800c2c50 => { ins += 15;
                { let mut ea = c.r[2].wrapping_add(0x284); for n in 19..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.mtspr(1013, c.r[19]);
                c.mtspr(953, c.r[20]);
                c.mtspr(954, c.r[21]);
                c.mtspr(957, c.r[22]);
                c.mtspr(958, c.r[23]);
                c.mtspr(955, c.r[24]);
                c.mtspr(952, c.r[25]);
                c.mtspr(956, c.r[26]);
                c.mtspr(1020, c.r[27]);
                c.mtspr(1021, c.r[28]);
                c.mtspr(1022, c.r[29]);
                c.mtspr(1019, c.r[30]);
                c.mtspr(1017, c.r[31]);
                pc = 0x800c2cbc; continue;
                pc = 0x800c2c8c;
            }
            0x800c2c8c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[6], 0x0);
                if cr_bit(cr, 2) { pc = 0x800c2c9c; continue; }
                pc = 0x800c2c94;
            }
            0x800c2c94 => { ins += 2;
                c.r[26] = c.lwz_t(ins, c.r[2].wrapping_add(0x278));
                c.mtspr(22, c.r[26]);
                pc = 0x800c2c9c;
            }
            0x800c2c9c => { ins += 8;
                { let mut ea = c.r[2].wrapping_add(0x240); for n in 25..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.mtspr(976, c.r[25]);
                c.mtspr(977, c.r[26]);
                c.mtspr(978, c.r[27]);
                c.mtspr(979, c.r[28]);
                c.mtspr(980, c.r[29]);
                c.mtspr(981, c.r[30]);
                c.mtspr(982, c.r[31]);
                pc = 0x800c2cbc;
            }
            0x800c2cbc => { ins += 49;
                { let mut ea = c.r[2].wrapping_add(0x1a8); for n in 16..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                // mtsr
                { let mut ea = c.r[2].wrapping_add(0x1f0); for n in 12..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.mtspr(1008, c.r[12]);
                c.mtspr(1009, c.r[13]);
                c.mtspr(27, c.r[14]);
                c.mtspr(287, c.r[15]);
                c.mtspr(528, c.r[16]);
                c.mtspr(529, c.r[17]);
                c.mtspr(530, c.r[18]);
                c.mtspr(531, c.r[19]);
                c.mtspr(532, c.r[20]);
                c.mtspr(533, c.r[21]);
                c.mtspr(534, c.r[22]);
                c.mtspr(535, c.r[23]);
                c.mtspr(536, c.r[24]);
                c.mtspr(537, c.r[25]);
                c.mtspr(538, c.r[26]);
                c.mtspr(539, c.r[27]);
                c.mtspr(540, c.r[28]);
                c.mtspr(541, c.r[29]);
                c.mtspr(542, c.r[30]);
                c.mtspr(543, c.r[31]);
                { let mut ea = c.r[2].wrapping_add(0x25c); for n in 22..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.mtspr(25, c.r[22]);
                c.mtspr(19, c.r[23]);
                c.mtspr(18, c.r[24]);
                c.mtspr(272, c.r[25]);
                c.mtspr(273, c.r[26]);
                c.mtspr(274, c.r[27]);
                c.mtspr(275, c.r[28]);
                c.mtspr(1010, c.r[30]);
                c.mtspr(282, c.r[31]);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c2bc8: bad pc {:#x}", pc),
        }
    }
}

/// InitMetroTRK (0x800c2d80, 148 bytes)
#[inline(never)]
pub fn f_800c2d80(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c2d80) { return; }
        if c.prof { crate::prof::enter(c, 0x800c2d80); p = true; }
    }
    f_800c2d80_b(c, 0x800c2d80);
    if p { crate::prof::exit(c, 0x800c2d80); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c2d80_at(c: &mut Cpu, pc: u32) { f_800c2d80_b(c, pc) }
#[inline(always)]
fn f_800c2d80_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c2d80;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c2d80, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c2d80 => { ins += 20;
                c.r[1] = c.r[1].wrapping_add(0xfffffffc);
                c.stw(c.r[1], c.r[3]);
                c.r[3] = 0x803f0000;
                c.r[3] = c.r[3] | 0xe880;
                { let mut ea = c.r[3]; for n in 0..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[4] = c.lwz_t(ins, c.r[1]);
                c.r[1] = c.r[1].wrapping_add(0x4);
                c.stw(c.r[3].wrapping_add(0x4), c.r[1]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                c.r[4] = c.mfspr(8);
                c.stw(c.r[3].wrapping_add(0x84), c.r[4]);
                c.stw(c.r[3].wrapping_add(0x80), c.r[4]);
                c.r[4] = cr;
                c.stw(c.r[3].wrapping_add(0x88), c.r[4]);
                c.r[4] = 0; // mfmsr
                c.r[3] = c.r[4] | 0x8000;
                c.r[3] = c.r[3] ^ 0x8000;
                // mtmsr
                c.mtspr(27, c.r[4]);
                c.lr = 0x800c2dd0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c2a10(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2dd0;
            }
            0x800c2dd0 => { ins += 10;
                c.r[3] = 0x803f0000;
                c.r[3] = c.r[3] | 0xe880;
                { let mut ea = c.r[3]; for n in 0..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = 0x0;
                c.mtspr(1010, c.r[0]);
                c.mtspr(1013, c.r[0]);
                c.r[1] = 0x80490000;
                c.r[1] = c.r[1] | 0x6738;
                c.r[3] = c.r[5] | c.r[5];
                c.lr = 0x800c2df8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c36c8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2df8;
            }
            0x800c2df8 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                if !cr_bit(cr, 2) { pc = 0x800c2e10; continue; }
                pc = 0x800c2e00;
            }
            0x800c2e00 => { ins += 4;
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x84));
                c.mtspr(8, c.r[4]);
                { let mut ea = c.r[3]; for n in 0..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800c2e10;
            }
            0x800c2e10 => { ins += 1;
                { c.steps = st; c.instrs = ins; c.cr = cr; f_800c33bc(c); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c2d80: bad pc {:#x}", pc),
        }
    }
}

/// InitMetroTRK_BBA (0x800c2e18, 148 bytes)
#[inline(never)]
pub fn f_800c2e18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c2e18) { return; }
        if c.prof { crate::prof::enter(c, 0x800c2e18); p = true; }
    }
    f_800c2e18_b(c, 0x800c2e18);
    if p { crate::prof::exit(c, 0x800c2e18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c2e18_at(c: &mut Cpu, pc: u32) { f_800c2e18_b(c, pc) }
#[inline(always)]
fn f_800c2e18_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c2e18;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c2e18, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c2e18 => { ins += 19;
                c.r[1] = c.r[1].wrapping_add(0xfffffffc);
                c.stw(c.r[1], c.r[3]);
                c.r[3] = 0x803f0000;
                c.r[3] = c.r[3] | 0xe880;
                { let mut ea = c.r[3]; for n in 0..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[4] = c.lwz_t(ins, c.r[1]);
                c.r[1] = c.r[1].wrapping_add(0x4);
                c.stw(c.r[3].wrapping_add(0x4), c.r[1]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                c.r[4] = c.mfspr(8);
                c.stw(c.r[3].wrapping_add(0x84), c.r[4]);
                c.stw(c.r[3].wrapping_add(0x80), c.r[4]);
                c.r[4] = cr;
                c.stw(c.r[3].wrapping_add(0x88), c.r[4]);
                c.r[4] = 0; // mfmsr
                c.r[3] = c.r[4] | 0x8000;
                // mtmsr
                c.mtspr(27, c.r[4]);
                c.lr = 0x800c2e64; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c2a10(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2e64;
            }
            0x800c2e64 => { ins += 10;
                c.r[3] = 0x803f0000;
                c.r[3] = c.r[3] | 0xe880;
                { let mut ea = c.r[3]; for n in 0..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = 0x0;
                c.mtspr(1010, c.r[0]);
                c.mtspr(1013, c.r[0]);
                c.r[1] = 0x80490000;
                c.r[1] = c.r[1] | 0x6738;
                c.r[3] = 0x2;
                c.lr = 0x800c2e8c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c36c8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2e8c;
            }
            0x800c2e8c => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x1);
                if !cr_bit(cr, 2) { pc = 0x800c2ea4; continue; }
                pc = 0x800c2e94;
            }
            0x800c2e94 => { ins += 4;
                c.r[4] = c.lwz_t(ins, c.r[3].wrapping_add(0x84));
                c.mtspr(8, c.r[4]);
                { let mut ea = c.r[3]; for n in 0..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800c2ea4;
            }
            0x800c2ea4 => { ins += 1;
                { c.steps = st; c.instrs = ins; c.cr = cr; f_800c33bc(c); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; }
                pc = 0x800c2ea8;
            }
            0x800c2ea8 => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c2e18: bad pc {:#x}", pc),
        }
    }
}

/// TRK__write_aram (0x800c2eac, 492 bytes)
#[inline(never)]
pub fn f_800c2eac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c2eac) { return; }
        if c.prof { crate::prof::enter(c, 0x800c2eac); p = true; }
    }
    f_800c2eac_b(c, 0x800c2eac);
    if p { crate::prof::exit(c, 0x800c2eac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c2eac_at(c: &mut Cpu, pc: u32) { f_800c2eac_b(c, pc) }
#[inline(never)]
fn f_800c2eac_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c2eac;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c2eac, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c2eac => { ins += 12;
                c.r[11] = c.r[1].rotate_left(0) & 0x1f;
                c.r[12] = c.r[1] | c.r[1];
                c.r[11] = c.addc(!c.r[11], 0xffffff80, 1);
                { let ea = c.r[1].wrapping_add(c.r[11]); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[12].wrapping_add(0x4), c.r[0]);
                { let mut ea = c.r[12].wrapping_add(0xffffffdc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[31] = c.r[4] | c.r[4];
                cr_cmpu(&mut cr, c.xer, 0, c.r[31], 0x4000);
                c.r[30] = c.r[3] | c.r[3];
                c.r[23] = c.r[5] | c.r[5];
                if cr_bit(cr, 0) { pc = 0x800c3080; continue; }
                pc = 0x800c2edc;
            }
            0x800c2edc => { ins += 5;
                c.r[4] = c.lwz_t(ins, c.r[23]);
                c.r[0] = 0x8000000;
                c.r[3] = c.r[31].wrapping_add(c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x800c2ef4; continue; }
                pc = 0x800c2ef0;
            }
            0x800c2ef0 => { ins += 1;
                pc = 0x800c3080; continue;
                pc = 0x800c2ef4;
            }
            0x800c2ef4 => { ins += 10;
                c.r[0] = c.r[31].rotate_left(0) & 0x1f;
                c.r[27] = c.r[31].rotate_left(0) & 0xffffffe0;
                c.r[26] = c.r[4].wrapping_add(c.r[0]);
                c.r[24] = 0x0;
                c.r[0] = c.r[26].wrapping_add(0x1f);
                c.r[26] = c.r[0].rotate_left(0) & 0xffffffe0;
                cr_cmpu(&mut cr, c.xer, 0, c.r[26], 0x0);
                c.r[3] = c.r[26].wrapping_add(0x1f);
                c.r[3] = c.r[3].rotate_left(27) & 0x7ffffff;
                if !cr_bit(cr, 1) { pc = 0x800c2f84; continue; }
                pc = 0x800c2f1c;
            }
            0x800c2f1c => { ins += 3;
                c.r[0] = c.r[3].rotate_left(29) & 0x1fffffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.mtspr(9, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c2f74; continue; }
                pc = 0x800c2f28;
            }
            0x800c2f28 => { ins += 17;
                // dcbf
                c.r[24] = c.r[24].wrapping_add(0x20);
                // dcbf
                c.r[24] = c.r[24].wrapping_add(0x20);
                // dcbf
                c.r[24] = c.r[24].wrapping_add(0x20);
                // dcbf
                c.r[24] = c.r[24].wrapping_add(0x20);
                // dcbf
                c.r[24] = c.r[24].wrapping_add(0x20);
                // dcbf
                c.r[24] = c.r[24].wrapping_add(0x20);
                // dcbf
                c.r[24] = c.r[24].wrapping_add(0x20);
                // dcbf
                c.r[24] = c.r[24].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800c2f28; continue; }
                pc = 0x800c2f6c;
            }
            0x800c2f6c => { ins += 2;
                c.r[3] = c.r[3] & 0x7;
                cr_rc(&mut cr, c.xer, c.r[3]);
                if cr_bit(cr, 2) { pc = 0x800c2f84; continue; }
                pc = 0x800c2f74;
            }
            0x800c2f74 => { ins += 1;
                c.mtspr(9, c.r[3]);
                pc = 0x800c2f78;
            }
            0x800c2f78 => { ins += 3;
                // dcbf
                c.r[24] = c.r[24].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800c2f78; continue; }
                pc = 0x800c2f84;
            }
            0x800c2f84 => { ins += 1;
                c.lr = 0x800c2f88; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac954(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2f88;
            }
            0x800c2f88 => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c2f84; continue; }
                pc = 0x800c2f90;
            }
            0x800c2f90 => { ins += 1;
                c.lr = 0x800c2f94; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2f94;
            }
            0x800c2f94 => { ins += 4;
                c.r[24] = c.r[31].rotate_left(0) & 0x1f;
                cr_rc(&mut cr, c.xer, c.r[24]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[25] = 0x8000000;
                if cr_bit(cr, 2) { pc = 0x800c2fe8; continue; }
                pc = 0x800c2fa4;
            }
            0x800c2fa4 => { ins += 4;
                c.r[28] = c.r[1].wrapping_add(0x20);
                c.r[25] = c.r[27] | c.r[27];
                // dcbi
                c.lr = 0x800c2fb4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbcc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2fb4;
            }
            0x800c2fb4 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[27] | c.r[27];
                c.r[3] = 0x1;
                c.r[6] = 0x20;
                c.lr = 0x800c2fc8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac990(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2fc8;
            }
            0x800c2fc8 => { ins += 1;
                c.lr = 0x800c2fcc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2fcc;
            }
            0x800c2fcc => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c2fc8; continue; }
                pc = 0x800c2fd4;
            }
            0x800c2fd4 => { ins += 4;
                c.r[3] = c.r[30] | c.r[30];
                c.r[5] = c.r[24] | c.r[24];
                c.r[4] = c.r[1].wrapping_add(0x20);
                c.lr = 0x800c2fe4; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c2fe4;
            }
            0x800c2fe4 => { ins += 1;
                // dcbf
                pc = 0x800c2fe8;
            }
            0x800c2fe8 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[23]);
                c.r[31] = c.r[31].wrapping_add(c.r[0]);
                c.r[24] = c.r[31].rotate_left(0) & 0x1f;
                cr_rc(&mut cr, c.xer, c.r[24]);
                if cr_bit(cr, 2) { pc = 0x800c304c; continue; }
                pc = 0x800c2ff8;
            }
            0x800c2ff8 => { ins += 3;
                c.r[23] = c.r[31].rotate_left(0) & 0xffffffe0;
                cr_cmpu(&mut cr, c.xer, 0, c.r[23], c.r[25]);
                if cr_bit(cr, 2) { pc = 0x800c3030; continue; }
                pc = 0x800c3004;
            }
            0x800c3004 => { ins += 3;
                c.r[28] = c.r[1].wrapping_add(0x20);
                // dcbi
                c.lr = 0x800c3010; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbcc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3010;
            }
            0x800c3010 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[23] | c.r[23];
                c.r[3] = 0x1;
                c.r[6] = 0x20;
                c.lr = 0x800c3024; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac990(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3024;
            }
            0x800c3024 => { ins += 1;
                c.lr = 0x800c3028; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3028;
            }
            0x800c3028 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c3024; continue; }
                pc = 0x800c3030;
            }
            0x800c3030 => { ins += 6;
                c.r[25] = c.r[30].wrapping_add(c.r[31]);
                c.r[4] = c.r[1].wrapping_add(0x20);
                c.r[3] = c.r[25] | c.r[25];
                c.r[5] = c.addc(!c.r[24], 0x20, 1);
                c.r[4] = c.r[4].wrapping_add(c.r[24]);
                c.lr = 0x800c3048; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3048;
            }
            0x800c3048 => { ins += 1;
                // dcbf
                pc = 0x800c304c;
            }
            0x800c304c => { ins += 2;
                // sync
                c.lr = 0x800c3054; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbcc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3054;
            }
            0x800c3054 => { ins += 5;
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[27] | c.r[27];
                c.r[6] = c.r[26] | c.r[26];
                c.r[3] = 0x0;
                c.lr = 0x800c3068; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac990(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3068;
            }
            0x800c3068 => { ins += 2;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800c3080; continue; }
                pc = 0x800c3070;
            }
            0x800c3070 => { ins += 1;
                c.lr = 0x800c3074; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3074;
            }
            0x800c3074 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c3070; continue; }
                pc = 0x800c307c;
            }
            0x800c307c => { ins += 1;
                c.lr = 0x800c3080; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbcc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3080;
            }
            0x800c3080 => { ins += 6;
                c.r[10] = c.lwz_t(ins, c.r[1]);
                { let mut ea = c.r[10].wrapping_add(0xffffffdc); for n in 23..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[10].wrapping_add(0x4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[10] | c.r[10];
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c2eac: bad pc {:#x}", pc),
        }
    }
}

/// TRK__read_aram (0x800c3098, 308 bytes)
#[inline(never)]
pub fn f_800c3098(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c3098) { return; }
        if c.prof { crate::prof::enter(c, 0x800c3098); p = true; }
    }
    f_800c3098_b(c, 0x800c3098);
    if p { crate::prof::exit(c, 0x800c3098); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c3098_at(c: &mut Cpu, pc: u32) { f_800c3098_b(c, pc) }
#[inline(always)]
fn f_800c3098_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c3098;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c3098, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c3098 => { ins += 10;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                cr_cmpu(&mut cr, c.xer, 0, c.r[4], 0x4000);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                if cr_bit(cr, 0) { pc = 0x800c31ac; continue; }
                pc = 0x800c30c0;
            }
            0x800c30c0 => { ins += 5;
                c.r[5] = c.lwz_t(ins, c.r[5]);
                c.r[0] = 0x8000000;
                c.r[3] = c.r[4].wrapping_add(c.r[5]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 1) { pc = 0x800c30d8; continue; }
                pc = 0x800c30d4;
            }
            0x800c30d4 => { ins += 1;
                pc = 0x800c31ac; continue;
                pc = 0x800c30d8;
            }
            0x800c30d8 => { ins += 10;
                c.r[0] = c.r[4].rotate_left(0) & 0x1f;
                c.r[30] = c.r[4].rotate_left(0) & 0xffffffe0;
                c.r[29] = c.r[5].wrapping_add(c.r[0]);
                c.r[4] = 0x0;
                c.r[0] = c.r[29].wrapping_add(0x1f);
                c.r[29] = c.r[0].rotate_left(0) & 0xffffffe0;
                cr_cmpu(&mut cr, c.xer, 0, c.r[29], 0x0);
                c.r[3] = c.r[29].wrapping_add(0x1f);
                c.r[3] = c.r[3].rotate_left(27) & 0x7ffffff;
                if !cr_bit(cr, 1) { pc = 0x800c3168; continue; }
                pc = 0x800c3100;
            }
            0x800c3100 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(29) & 0x1fffffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                c.mtspr(9, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c3158; continue; }
                pc = 0x800c310c;
            }
            0x800c310c => { ins += 17;
                // dcbi
                c.r[4] = c.r[4].wrapping_add(0x20);
                // dcbi
                c.r[4] = c.r[4].wrapping_add(0x20);
                // dcbi
                c.r[4] = c.r[4].wrapping_add(0x20);
                // dcbi
                c.r[4] = c.r[4].wrapping_add(0x20);
                // dcbi
                c.r[4] = c.r[4].wrapping_add(0x20);
                // dcbi
                c.r[4] = c.r[4].wrapping_add(0x20);
                // dcbi
                c.r[4] = c.r[4].wrapping_add(0x20);
                // dcbi
                c.r[4] = c.r[4].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800c310c; continue; }
                pc = 0x800c3150;
            }
            0x800c3150 => { ins += 2;
                c.r[3] = c.r[3] & 0x7;
                cr_rc(&mut cr, c.xer, c.r[3]);
                if cr_bit(cr, 2) { pc = 0x800c3168; continue; }
                pc = 0x800c3158;
            }
            0x800c3158 => { ins += 1;
                c.mtspr(9, c.r[3]);
                pc = 0x800c315c;
            }
            0x800c315c => { ins += 3;
                // dcbi
                c.r[4] = c.r[4].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800c315c; continue; }
                pc = 0x800c3168;
            }
            0x800c3168 => { ins += 1;
                c.lr = 0x800c316c; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac954(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c316c;
            }
            0x800c316c => { ins += 2;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x0);
                if !cr_bit(cr, 2) { pc = 0x800c3168; continue; }
                pc = 0x800c3174;
            }
            0x800c3174 => { ins += 1;
                c.lr = 0x800c3178; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3178;
            }
            0x800c3178 => { ins += 2;
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800c3180; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbcc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3180;
            }
            0x800c3180 => { ins += 5;
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[30] | c.r[30];
                c.r[6] = c.r[29] | c.r[29];
                c.r[3] = 0x1;
                c.lr = 0x800c3194; { c.steps = st; c.instrs = ins; c.cr = cr; f_800ac990(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3194;
            }
            0x800c3194 => { ins += 1;
                c.lr = 0x800c3198; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbec(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3198;
            }
            0x800c3198 => { ins += 2;
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c3194; continue; }
                pc = 0x800c31a0;
            }
            0x800c31a0 => { ins += 2;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { pc = 0x800c31ac; continue; }
                pc = 0x800c31a8;
            }
            0x800c31a8 => { ins += 1;
                c.lr = 0x800c31ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800acbcc(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c31ac;
            }
            0x800c31ac => { ins += 8;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz_t(ins, c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz_t(ins, c.r[1].wrapping_add(0x10));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c3098: bad pc {:#x}", pc),
        }
    }
}

/// TRKInitializeTarget (0x800c31cc, 76 bytes)
#[inline(never)]
pub fn f_800c31cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c31cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800c31cc); p = true; }
    }
    f_800c31cc_b(c, 0x800c31cc);
    if p { crate::prof::exit(c, 0x800c31cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c31cc_at(c: &mut Cpu, pc: u32) { f_800c31cc_b(c, pc) }
#[inline(always)]
fn f_800c31cc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c31cc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c31cc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c31cc => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x1;
                c.r[3] = c.r[3].wrapping_add(0xffffe7dc);
                c.stw(c.r[3].wrapping_add(0x98), c.r[0]);
                c.lr = 0x800c31ec; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0e60(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c31ec;
            }
            0x800c31ec => { ins += 11;
                c.r[5] = 0x80400000;
                c.r[4] = 0x80400000;
                c.r[5] = c.r[5].wrapping_add(0xffffe7dc);
                c.r[0] = 0xe0000000;
                c.stw(c.r[5].wrapping_add(0x8c), c.r[3]);
                c.r[3] = 0x0;
                c.stw(c.r[4].wrapping_add(0xffffed58), c.r[0]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c31cc: bad pc {:#x}", pc),
        }
    }
}

/// __TRK_copy_vectors (0x800c3218, 300 bytes)
#[inline(never)]
pub fn f_800c3218(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c3218) { return; }
        if c.prof { crate::prof::enter(c, 0x800c3218); p = true; }
    }
    f_800c3218_b(c, 0x800c3218);
    if p { crate::prof::exit(c, 0x800c3218); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c3218_at(c: &mut Cpu, pc: u32) { f_800c3218_b(c, pc) }
#[inline(always)]
fn f_800c3218_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c3218;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c3218, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c3218 => { ins += 9;
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xffffed58);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[3] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], 0x44);
                if cr_bit(cr, 1) { pc = 0x800c3264; continue; }
                pc = 0x800c323c;
            }
            0x800c323c => { ins += 3;
                c.r[0] = c.r[3].wrapping_add(0x4000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[0], 0x44);
                if !cr_bit(cr, 1) { pc = 0x800c3264; continue; }
                pc = 0x800c3248;
            }
            0x800c3248 => { ins += 5;
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0xffffe880);
                c.r[0] = c.lwz_t(ins, c.r[3].wrapping_add(0x238));
                c.r[0] = c.r[0].rotate_left(0) & 0x3;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c3264; continue; }
                pc = 0x800c325c;
            }
            0x800c325c => { ins += 2;
                c.r[5] = 0x44;
                pc = 0x800c326c; continue;
                pc = 0x800c3264;
            }
            0x800c3264 => { ins += 2;
                c.r[3] = 0x80000000;
                c.r[5] = c.r[3].wrapping_add(0x44);
                pc = 0x800c326c;
            }
            0x800c326c => { ins += 6;
                c.r[4] = 0x80310000;
                c.r[3] = 0x80400000;
                c.r[29] = c.lwz_t(ins, c.r[5]);
                c.r[31] = c.r[4].wrapping_add(0x3848);
                c.r[28] = c.r[3].wrapping_add(0xffffe880);
                c.r[30] = 0x0;
                pc = 0x800c3284;
            }
            0x800c3284 => { ins += 4;
                c.r[0] = 0x1;
                c.r[0] = slw(c.r[0], c.r[30]);
                c.r[0] = c.r[29] & c.r[0];
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c3320; continue; }
                pc = 0x800c3294;
            }
            0x800c3294 => { ins += 2;
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0x4);
                if cr_bit(cr, 2) { pc = 0x800c3320; continue; }
                pc = 0x800c329c;
            }
            0x800c329c => { ins += 6;
                c.r[3] = 0x80400000;
                c.r[6] = c.lwz_t(ins, c.r[31]);
                c.r[3] = c.r[3].wrapping_add(0xffffed58);
                c.r[3] = c.lwz_t(ins, c.r[3]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], c.r[3]);
                if cr_bit(cr, 0) { pc = 0x800c32d4; continue; }
                pc = 0x800c32b4;
            }
            0x800c32b4 => { ins += 3;
                c.r[0] = c.r[3].wrapping_add(0x4000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x800c32d4; continue; }
                pc = 0x800c32c0;
            }
            0x800c32c0 => { ins += 3;
                c.r[0] = c.lwz_t(ins, c.r[28].wrapping_add(0x238));
                c.r[0] = c.r[0].rotate_left(0) & 0x3;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if cr_bit(cr, 2) { pc = 0x800c32d4; continue; }
                pc = 0x800c32cc;
            }
            0x800c32cc => { ins += 2;
                c.r[27] = c.r[6] | c.r[6];
                pc = 0x800c32fc; continue;
                pc = 0x800c32d4;
            }
            0x800c32d4 => { ins += 3;
                c.r[0] = 0x7e000000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x800c32f4; continue; }
                pc = 0x800c32e0;
            }
            0x800c32e0 => { ins += 3;
                c.r[0] = 0x80000000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[6], c.r[0]);
                if cr_bit(cr, 1) { pc = 0x800c32f4; continue; }
                pc = 0x800c32ec;
            }
            0x800c32ec => { ins += 2;
                c.r[27] = c.r[6] | c.r[6];
                pc = 0x800c32fc; continue;
                pc = 0x800c32f4;
            }
            0x800c32f4 => { ins += 2;
                c.r[0] = c.r[6].rotate_left(0) & 0x3fffffff;
                c.r[27] = c.r[0] | 0x80000000;
                pc = 0x800c32fc;
            }
            0x800c32fc => { ins += 6;
                c.r[4] = 0x80000000;
                c.r[3] = c.r[27] | c.r[27];
                c.r[0] = c.r[4].wrapping_add(0x34ac);
                c.r[5] = 0x100;
                c.r[4] = c.r[0].wrapping_add(c.r[6]);
                c.lr = 0x800c3314; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3314;
            }
            0x800c3314 => { ins += 3;
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = 0x100;
                c.lr = 0x800c3320; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0d70(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3320;
            }
            0x800c3320 => { ins += 4;
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x4);
                cr_cmps(&mut cr, c.xer, 0, c.r[30], 0xe);
                if !cr_bit(cr, 1) { pc = 0x800c3284; continue; }
                pc = 0x800c3330;
            }
            0x800c3330 => { ins += 5;
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c3218: bad pc {:#x}", pc),
        }
    }
}

/// TRKTargetTranslate (0x800c3344, 88 bytes)
#[inline(never)]
pub fn f_800c3344(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c3344) { return; }
        if c.prof { crate::prof::enter(c, 0x800c3344); p = true; }
    }
    f_800c3344_b(c, 0x800c3344);
    if p { crate::prof::exit(c, 0x800c3344); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c3344_at(c: &mut Cpu, pc: u32) { f_800c3344_b(c, pc) }
#[inline(always)]
fn f_800c3344_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c3344;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c3344, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c3344 => { ins += 5;
                c.r[4] = 0x80400000;
                c.r[4] = c.r[4].wrapping_add(0xffffed58);
                c.r[4] = c.lwz_t(ins, c.r[4]);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[4]);
                if cr_bit(cr, 0) { pc = 0x800c3378; continue; }
                pc = 0x800c3358;
            }
            0x800c3358 => { ins += 3;
                c.r[0] = c.r[4].wrapping_add(0x4000);
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 0) { pc = 0x800c3378; continue; }
                pc = 0x800c3364;
            }
            0x800c3364 => { ins += 5;
                c.r[4] = 0x80400000;
                c.r[4] = c.r[4].wrapping_add(0xffffe880);
                c.r[0] = c.lwz_t(ins, c.r[4].wrapping_add(0x238));
                c.r[0] = c.r[0].rotate_left(0) & 0x3;
                cr_rc(&mut cr, c.xer, c.r[0]);
                if !cr_bit(cr, 2) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800c3378;
            }
            0x800c3378 => { ins += 3;
                c.r[0] = 0x7e000000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if cr_bit(cr, 0) { pc = 0x800c3390; continue; }
                pc = 0x800c3384;
            }
            0x800c3384 => { ins += 3;
                c.r[0] = 0x80000000;
                cr_cmpu(&mut cr, c.xer, 0, c.r[3], c.r[0]);
                if !cr_bit(cr, 1) { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                pc = 0x800c3390;
            }
            0x800c3390 => { ins += 3;
                c.r[0] = c.r[3].rotate_left(0) & 0x3fffffff;
                c.r[3] = c.r[0] | 0x80000000;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c3344: bad pc {:#x}", pc),
        }
    }
}

/// EnableMetroTRKInterrupts (0x800c339c, 32 bytes)
#[inline(never)]
pub fn f_800c339c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c339c) { return; }
        if c.prof { crate::prof::enter(c, 0x800c339c); p = true; }
    }
    f_800c339c_b(c, 0x800c339c);
    if p { crate::prof::exit(c, 0x800c339c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c339c_at(c: &mut Cpu, pc: u32) { f_800c339c_b(c, pc) }
#[inline(always)]
fn f_800c339c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c339c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c339c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c339c => { ins += 4;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x800c33ac; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c3630(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c33ac;
            }
            0x800c33ac => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c339c: bad pc {:#x}", pc),
        }
    }
}

/// TRK_main (0x800c33bc, 88 bytes)
#[inline(never)]
pub fn f_800c33bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c33bc) { return; }
        if c.prof { crate::prof::enter(c, 0x800c33bc); p = true; }
    }
    f_800c33bc_b(c, 0x800c33bc);
    if p { crate::prof::exit(c, 0x800c33bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c33bc_at(c: &mut Cpu, pc: u32) { f_800c33bc_b(c, pc) }
#[inline(always)]
fn f_800c33bc_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c33bc;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c33bc, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c33bc => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80270000;
                c.r[3] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0xfffffb80);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c33dc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800c44f8(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c33dc;
            }
            0x800c33dc => { ins += 1;
                c.lr = 0x800c33e0; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be6b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c33e0;
            }
            0x800c33e0 => { ins += 4;
                c.r[4] = 0x80400000;
                cr_cmps(&mut cr, c.xer, 0, c.r[3], 0x0);
                c.stw(c.r[4].wrapping_add(0xffffed60), c.r[3]);
                if !cr_bit(cr, 2) { pc = 0x800c33f8; continue; }
                pc = 0x800c33f0;
            }
            0x800c33f0 => { ins += 1;
                c.lr = 0x800c33f4; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be668(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c33f4;
            }
            0x800c33f4 => { ins += 1;
                c.lr = 0x800c33f8; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be348(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c33f8;
            }
            0x800c33f8 => { ins += 1;
                c.lr = 0x800c33fc; { c.steps = st; c.instrs = ins; c.cr = cr; f_800be690(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c33fc;
            }
            0x800c33fc => { ins += 6;
                c.r[4] = 0x80400000;
                c.stw(c.r[4].wrapping_add(0xffffed60), c.r[3]);
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c33bc: bad pc {:#x}", pc),
        }
    }
}

/// TRKLoadContext (0x800c3414, 136 bytes)
#[inline(never)]
pub fn f_800c3414(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c3414) { return; }
        if c.prof { crate::prof::enter(c, 0x800c3414); p = true; }
    }
    f_800c3414_b(c, 0x800c3414);
    if p { crate::prof::exit(c, 0x800c3414); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c3414_at(c: &mut Cpu, pc: u32) { f_800c3414_b(c, pc) }
#[inline(always)]
fn f_800c3414_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c3414;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c3414, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c3414 => { ins += 6;
                c.r[0] = c.lwz_t(ins, c.r[3]);
                c.r[1] = c.lwz_t(ins, c.r[3].wrapping_add(0x4));
                c.r[2] = c.lwz_t(ins, c.r[3].wrapping_add(0x8));
                c.r[5] = c.lhz_t(ins, c.r[3].wrapping_add(0x1a2));
                c.r[6] = c.r[5].rotate_left(0) & 0x2;
                cr_rc(&mut cr, c.xer, c.r[6]);
                if cr_bit(cr, 2) { pc = 0x800c343c; continue; }
                pc = 0x800c342c;
            }
            0x800c342c => { ins += 4;
                c.r[5] = c.r[5].rotate_left(0) & 0xfffffffd;
                c.sth(c.r[3].wrapping_add(0x1a2), c.r[5]);
                { let mut ea = c.r[3].wrapping_add(0x14); for n in 5..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                pc = 0x800c3440; continue;
                pc = 0x800c343c;
            }
            0x800c343c => { ins += 1;
                { let mut ea = c.r[3].wrapping_add(0x34); for n in 13..32 { c.r[n] = c.lwz_t(ins, ea); ea = ea.wrapping_add(4); } }
                pc = 0x800c3440;
            }
            0x800c3440 => { ins += 23;
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x80));
                cr = (cr & 0x0) | (c.r[4] & 0xffffffff);
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x84));
                c.mtspr(8, c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x88));
                c.mtspr(9, c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x8c));
                c.mtspr(1, c.r[4]);
                c.r[4] = 0; // mfmsr
                c.r[4] = c.r[4].rotate_left(0) & 0xffff7fff;
                c.r[4] = c.r[4].rotate_left(0) & 0xfffffffd;
                // mtmsr
                c.mtspr(273, c.r[2]);
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0xc));
                c.mtspr(274, c.r[4]);
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x10));
                c.mtspr(275, c.r[4]);
                c.r[2] = c.lwz_t(ins, c.r[31].wrapping_add(0x198));
                c.r[4] = c.lwz_t(ins, c.r[31].wrapping_add(0x19c));
                c.r[31] = c.lwz_t(ins, c.r[31].wrapping_add(0x7c));
                { c.steps = st; c.instrs = ins; c.cr = cr; f_800c0eac(c); st = c.steps; ins = c.instrs; cr = c.cr; }; { c.steps = st; c.instrs = ins; c.cr = cr; return; }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c3414: bad pc {:#x}", pc),
        }
    }
}

/// TRKUARTInterruptHandler (0x800c349c, 4 bytes)
#[inline(never)]
pub fn f_800c349c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c349c) { return; }
        if c.prof { crate::prof::enter(c, 0x800c349c); p = true; }
    }
    f_800c349c_b(c, 0x800c349c);
    if p { crate::prof::exit(c, 0x800c349c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c349c_at(c: &mut Cpu, pc: u32) { f_800c349c_b(c, pc) }
#[inline(always)]
fn f_800c349c_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c349c;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c349c, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c349c => { ins += 1;
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c349c: bad pc {:#x}", pc),
        }
    }
}

/// InitializeProgramEndTrap (0x800c34a0, 88 bytes)
#[inline(never)]
pub fn f_800c34a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c34a0) { return; }
        if c.prof { crate::prof::enter(c, 0x800c34a0); p = true; }
    }
    f_800c34a0_b(c, 0x800c34a0);
    if p { crate::prof::exit(c, 0x800c34a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c34a0_at(c: &mut Cpu, pc: u32) { f_800c34a0_b(c, pc) }
#[inline(always)]
fn f_800c34a0_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c34a0;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c34a0, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c34a0 => { ins += 11;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x800a0000;
                c.r[3] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0x4;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffff803c);
                c.r[4] = c.r[3].wrapping_add(0xfffffb90);
                c.r[3] = c.r[31].wrapping_add(0x4);
                c.lr = 0x800c34cc; { c.steps = st; c.instrs = ins; c.cr = cr; f_80003488(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c34cc;
            }
            0x800c34cc => { ins += 3;
                c.r[3] = c.r[31].wrapping_add(0x4);
                c.r[4] = 0x4;
                c.lr = 0x800c34d8; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009b3b4(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c34d8;
            }
            0x800c34d8 => { ins += 3;
                c.r[3] = c.r[31].wrapping_add(0x4);
                c.r[4] = 0x4;
                c.lr = 0x800c34e4; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009b2d0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c34e4;
            }
            0x800c34e4 => { ins += 5;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz_t(ins, c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c34a0: bad pc {:#x}", pc),
        }
    }
}

/// TRK_board_display (0x800c34f8, 48 bytes)
#[inline(never)]
pub fn f_800c34f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x800c34f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800c34f8); p = true; }
    }
    f_800c34f8_b(c, 0x800c34f8);
    if p { crate::prof::exit(c, 0x800c34f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800c34f8_at(c: &mut Cpu, pc: u32) { f_800c34f8_b(c, pc) }
#[inline(always)]
fn f_800c34f8_b(c: &mut Cpu, mut pc: u32) {
    let mut st = c.steps;
    let mut ins = c.instrs;
    let mut cr = c.cr;
    c.last_fn = 0x800c34f8;
    loop {
        st += 1;
        if st > c.step_limit { c.steps = st; c.instrs = ins; c.cr = cr; c.step_event(0x800c34f8, pc); st = c.steps; ins = c.instrs; cr = c.cr; }
        match pc {
            0x800c34f8 => { ins += 8;
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x80270000;
                c.r[4] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.r[5].wrapping_add(0xfffffb94);
                { let (a, b) = (cr_bit(cr, 6), cr_bit(cr, 6)); cr_set_bit(&mut cr, 6, a ^ b); }
                c.lr = 0x800c3518; { c.steps = st; c.instrs = ins; c.cr = cr; f_8009c2e0(c); st = c.steps; ins = c.instrs; cr = c.cr; };
                pc = 0x800c3518;
            }
            0x800c3518 => { ins += 4;
                c.r[0] = c.lwz_t(ins, c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { { c.steps = st; c.instrs = ins; c.cr = cr; return; } }
                { c.steps = st; c.instrs = ins; c.cr = cr; return; }
            }
            _ => panic!("f_800c34f8: bad pc {:#x}", pc),
        }
    }
}

