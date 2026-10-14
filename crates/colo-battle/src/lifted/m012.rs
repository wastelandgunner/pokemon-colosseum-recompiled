#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// AMC_IsStub (0x800ce7c4, 8 bytes)
#[inline(never)]
pub fn f_800ce7c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800ce7c4) { return; }
        if c.prof { crate::prof::enter(c, 0x800ce7c4); f_800ce7c4_b(c, 0x800ce7c4); crate::prof::exit(c, 0x800ce7c4); return; }
    }
    f_800ce7c4_b(c, 0x800ce7c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ce7c4_at(c: &mut Cpu, pc: u32) { f_800ce7c4_b(c, pc) }
#[inline(always)]
fn f_800ce7c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ce7c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ce7c4, pc); }
        match pc {
            0x800ce7c4 => {
                c.r[3] = 0x1;
                if true { return; }
                return;
            }
            _ => panic!("f_800ce7c4: bad pc {:#x}", pc),
        }
    }
}

/// Hu_IsStub (0x800ce7cc, 8 bytes)
#[inline(never)]
pub fn f_800ce7cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800ce7cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800ce7cc); f_800ce7cc_b(c, 0x800ce7cc); crate::prof::exit(c, 0x800ce7cc); return; }
    }
    f_800ce7cc_b(c, 0x800ce7cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ce7cc_at(c: &mut Cpu, pc: u32) { f_800ce7cc_b(c, pc) }
#[inline(always)]
fn f_800ce7cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ce7cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ce7cc, pc); }
        match pc {
            0x800ce7cc => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_800ce7cc: bad pc {:#x}", pc),
        }
    }
}

/// fn_800CE7D4 (0x800ce7d4, 4 bytes)
#[inline(never)]
pub fn f_800ce7d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800ce7d4) { return; }
        if c.prof { crate::prof::enter(c, 0x800ce7d4); f_800ce7d4_b(c, 0x800ce7d4); crate::prof::exit(c, 0x800ce7d4); return; }
    }
    f_800ce7d4_b(c, 0x800ce7d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ce7d4_at(c: &mut Cpu, pc: u32) { f_800ce7d4_b(c, pc) }
#[inline(always)]
fn f_800ce7d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ce7d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ce7d4, pc); }
        match pc {
            0x800ce7d4 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800ce7d4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800CE7D8 (0x800ce7d8, 4 bytes)
#[inline(never)]
pub fn f_800ce7d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800ce7d8) { return; }
        if c.prof { crate::prof::enter(c, 0x800ce7d8); f_800ce7d8_b(c, 0x800ce7d8); crate::prof::exit(c, 0x800ce7d8); return; }
    }
    f_800ce7d8_b(c, 0x800ce7d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ce7d8_at(c: &mut Cpu, pc: u32) { f_800ce7d8_b(c, pc) }
#[inline(always)]
fn f_800ce7d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ce7d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ce7d8, pc); }
        match pc {
            0x800ce7d8 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800ce7d8: bad pc {:#x}", pc),
        }
    }
}

/// DBWrite (0x800ce7dc, 608 bytes)
#[inline(never)]
pub fn f_800ce7dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800ce7dc) { return; }
        if c.prof { crate::prof::enter(c, 0x800ce7dc); f_800ce7dc_b(c, 0x800ce7dc); crate::prof::exit(c, 0x800ce7dc); return; }
    }
    f_800ce7dc_b(c, 0x800ce7dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ce7dc_at(c: &mut Cpu, pc: u32) { f_800ce7dc_b(c, pc) }
#[inline(always)]
fn f_800ce7dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ce7dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ce7dc, pc); }
        match pc {
            0x800ce7dc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffff88); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x58); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3].wrapping_add(0x0);
                c.r[27] = c.r[4].wrapping_add(0x0);
                c.lr = 0x800ce7f8; f_8009df3c(c);
                pc = 0x800ce7f8;
            }
            0x800ce7f8 => {
                c.r[28] = c.r[3].wrapping_add(0x0);
                c.r[25] = 0xcc000000;
                c.r[31] = 0x40000000;
                pc = 0x800ce804;
            }
            0x800ce804 => {
                c.r[0] = c.lwz(c.r[25].wrapping_add(0x6828));
                c.r[30] = c.r[25].wrapping_add(0x6800);
                c.r[3] = c.r[1].wrapping_add(0x50);
                c.r[0] = c.r[0] & 0x405;
                c.rc(c.r[0]);
                c.r[4] = 0x2;
                c.r[5] = 0x1;
                c.r[0] = c.r[0] | 0xc0;
                { let ea = c.r[30].wrapping_add(0x28); c.stw(ea, c.r[0]); c.r[30] = ea; }
                c.stw(c.r[1].wrapping_add(0x50), c.r[31]);
                c.lr = 0x800ce82c; f_800cefbc(c);
                pc = 0x800ce82c;
            }
            0x800ce82c => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[24] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800ce834;
            }
            0x800ce834 => {
                c.r[29] = c.r[25].wrapping_add(0x6800);
                { let ea = c.r[29].wrapping_add(0x34); c.r[0] = c.lwz(ea); c.r[29] = ea; }
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ce834; continue; }
                pc = 0x800ce844;
            }
            0x800ce844 => {
                c.r[3] = c.r[1].wrapping_add(0x54);
                c.r[4] = 0x4;
                c.r[5] = 0x0;
                c.lr = 0x800ce854; f_800cefbc(c);
                pc = 0x800ce854;
            }
            0x800ce854 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ce854; continue; }
                pc = 0x800ce860;
            }
            0x800ce860 => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[0] = c.r[0] & 0x405;
                c.rc(c.r[0]);
                c.stw(c.r[30], c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ce804; continue; }
                pc = 0x800ce878;
            }
            0x800ce878 => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffff82b0));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stb(c.r[13].wrapping_add(0xffff82b0), c.r[0]);
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffff82b0));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ce898; continue; }
                pc = 0x800ce890;
            }
            0x800ce890 => {
                c.r[3] = 0x1000;
                pc = 0x800ce89c; continue;
                pc = 0x800ce898;
            }
            0x800ce898 => {
                c.r[3] = 0x0;
                pc = 0x800ce89c;
            }
            0x800ce89c => {
                c.r[0] = c.r[27].wrapping_add(0x3);
                c.r[25] = c.r[3] | 0x10000;
                c.r[24] = c.r[0].rotate_left(0) & 0xfffffffc;
                c.r[25] = c.r[25] | 0xc000;
                pc = 0x800ce8ac;
            }
            0x800ce8ac => {
                c.r[3] = c.r[25].wrapping_add(0x0);
                c.r[4] = c.r[26].wrapping_add(0x0);
                c.r[5] = c.r[24].wrapping_add(0x0);
                c.lr = 0x800ce8bc; f_800ced58(c);
                pc = 0x800ce8bc;
            }
            0x800ce8bc => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800ce8ac; continue; }
                pc = 0x800ce8c4;
            }
            0x800ce8c4 => {
                c.r[31] = 0x40000000;
                pc = 0x800ce8c8;
            }
            0x800ce8c8 => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[3] = c.r[1].wrapping_add(0x4c);
                c.r[4] = 0x2;
                c.r[0] = c.r[0] & 0x405;
                c.rc(c.r[0]);
                c.r[5] = 0x1;
                c.r[0] = c.r[0] | 0xc0;
                c.stw(c.r[30], c.r[0]);
                c.stw(c.r[1].wrapping_add(0x4c), c.r[31]);
                c.lr = 0x800ce8ec; f_800cefbc(c);
                pc = 0x800ce8ec;
            }
            0x800ce8ec => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[26] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800ce8f4;
            }
            0x800ce8f4 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ce8f4; continue; }
                pc = 0x800ce900;
            }
            0x800ce900 => {
                c.r[3] = c.r[1].wrapping_add(0x54);
                c.r[4] = 0x4;
                c.r[5] = 0x0;
                c.lr = 0x800ce910; f_800cefbc(c);
                pc = 0x800ce910;
            }
            0x800ce910 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ce910; continue; }
                pc = 0x800ce91c;
            }
            0x800ce91c => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[0] = c.r[0] & 0x405;
                c.rc(c.r[0]);
                c.stw(c.r[30], c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ce8c8; continue; }
                pc = 0x800ce934;
            }
            0x800ce934 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffff82b0));
                c.r[0] = c.r[0].rotate_left(16) & 0xffff0000;
                c.r[0] = c.r[0] | 0x1f000000;
                c.r[0] = c.r[0] | c.r[27];
                c.r[0] = c.r[0].rotate_left(0) & 0x1fffffff;
                c.r[24] = c.r[0] | 0xc0000000;
                pc = 0x800ce94c;
            }
            0x800ce94c => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[3] = c.r[1].wrapping_add(0x44);
                c.r[4] = 0x4;
                c.r[0] = c.r[0] & 0x405;
                c.rc(c.r[0]);
                c.r[5] = 0x1;
                c.r[0] = c.r[0] | 0xc0;
                c.stw(c.r[30], c.r[0]);
                c.stw(c.r[1].wrapping_add(0x44), c.r[24]);
                c.lr = 0x800ce970; f_800cefbc(c);
                pc = 0x800ce970;
            }
            0x800ce970 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800ce978;
            }
            0x800ce978 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ce978; continue; }
                pc = 0x800ce984;
            }
            0x800ce984 => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[0] = c.r[0] & 0x405;
                c.rc(c.r[0]);
                c.cmps(0, c.r[3], 0x0);
                c.stw(c.r[30], c.r[0]);
                if !c.crbit(2) { pc = 0x800ce94c; continue; }
                pc = 0x800ce998;
            }
            0x800ce998 => {
                c.r[27] = 0x40000000;
                pc = 0x800ce99c;
            }
            0x800ce99c => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[4] = 0x2;
                c.r[0] = c.r[0] & 0x405;
                c.rc(c.r[0]);
                c.r[5] = 0x1;
                c.r[0] = c.r[0] | 0xc0;
                c.stw(c.r[30], c.r[0]);
                c.stw(c.r[1].wrapping_add(0x3c), c.r[27]);
                c.lr = 0x800ce9c0; f_800cefbc(c);
                pc = 0x800ce9c0;
            }
            0x800ce9c0 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[26] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800ce9c8;
            }
            0x800ce9c8 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ce9c8; continue; }
                pc = 0x800ce9d4;
            }
            0x800ce9d4 => {
                c.r[3] = c.r[1].wrapping_add(0x54);
                c.r[4] = 0x4;
                c.r[5] = 0x0;
                c.lr = 0x800ce9e4; f_800cefbc(c);
                pc = 0x800ce9e4;
            }
            0x800ce9e4 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[3] = c.r[26] | c.r[0];
                pc = 0x800ce9f0;
            }
            0x800ce9f0 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ce9f0; continue; }
                pc = 0x800ce9fc;
            }
            0x800ce9fc => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[0] = c.r[0] & 0x405;
                c.rc(c.r[0]);
                c.cmps(0, c.r[3], 0x0);
                c.stw(c.r[30], c.r[0]);
                if !c.crbit(2) { pc = 0x800ce99c; continue; }
                pc = 0x800cea10;
            }
            0x800cea10 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ce99c; continue; }
                pc = 0x800cea1c;
            }
            0x800cea1c => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x800cea24; f_8009df64(c);
                pc = 0x800cea24;
            }
            0x800cea24 => {
                { let mut ea = c.r[1].wrapping_add(0x58); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x0;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x7c));
                c.r[1] = c.r[1].wrapping_add(0x78);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ce7dc: bad pc {:#x}", pc),
        }
    }
}

/// DBRead (0x800cea3c, 140 bytes)
#[inline(never)]
pub fn f_800cea3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cea3c) { return; }
        if c.prof { crate::prof::enter(c, 0x800cea3c); f_800cea3c_b(c, 0x800cea3c); crate::prof::exit(c, 0x800cea3c); return; }
    }
    f_800cea3c_b(c, 0x800cea3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cea3c_at(c: &mut Cpu, pc: u32) { f_800cea3c_b(c, pc) }
#[inline(always)]
fn f_800cea3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cea3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cea3c, pc); }
        match pc {
            0x800cea3c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800cea60; f_8009df3c(c);
                pc = 0x800cea60;
            }
            0x800cea60 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa210));
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.r[0].rotate_left(0) & 0x10000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800cea78; continue; }
                pc = 0x800cea70;
            }
            0x800cea70 => {
                c.r[3] = 0x1000;
                pc = 0x800cea7c; continue;
                pc = 0x800cea78;
            }
            0x800cea78 => {
                c.r[3] = 0x0;
                pc = 0x800cea7c;
            }
            0x800cea7c => {
                c.r[3] = c.r[3].wrapping_add(0x20000);
                c.r[0] = c.r[30].wrapping_add(0x3);
                c.r[4] = c.r[29].wrapping_add(0x0);
                c.r[5] = c.r[0].rotate_left(0) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(0xffffe000);
                c.lr = 0x800cea94; f_800cee34(c);
                pc = 0x800cea94;
            }
            0x800cea94 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa214), c.r[0]);
                c.r[3] = c.r[31] | c.r[31];
                c.stb(c.r[13].wrapping_add(0xffffa21c), c.r[0]);
                c.lr = 0x800ceaa8; f_8009df64(c);
                pc = 0x800ceaa8;
            }
            0x800ceaa8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cea3c: bad pc {:#x}", pc),
        }
    }
}

/// DBQueryData (0x800ceac8, 156 bytes)
#[inline(never)]
pub fn f_800ceac8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800ceac8) { return; }
        if c.prof { crate::prof::enter(c, 0x800ceac8); f_800ceac8_b(c, 0x800ceac8); crate::prof::exit(c, 0x800ceac8); return; }
    }
    f_800ceac8_b(c, 0x800ceac8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ceac8_at(c: &mut Cpu, pc: u32) { f_800ceac8_b(c, pc) }
#[inline(always)]
fn f_800ceac8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ceac8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ceac8, pc); }
        match pc {
            0x800ceac8 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x0;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa214));
                c.stb(c.r[13].wrapping_add(0xffffa21c), c.r[3]);
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800ceb4c; continue; }
                pc = 0x800ceaec;
            }
            0x800ceaec => {
                c.lr = 0x800ceaf0; f_8009df3c(c);
                pc = 0x800ceaf0;
            }
            0x800ceaf0 => {
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800ceafc; f_800cecac(c);
                pc = 0x800ceafc;
            }
            0x800ceafc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800ceb44; continue; }
                pc = 0x800ceb08;
            }
            0x800ceb08 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800ceb10; f_800cef10(c);
                pc = 0x800ceb10;
            }
            0x800ceb10 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[0] = c.r[0].rotate_left(0) & 0x1fffffff;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[3] = c.r[4].rotate_left(0) & 0x1f000000;
                c.r[0] = c.r[3].wrapping_add(0xe1000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800ceb44; continue; }
                pc = 0x800ceb30;
            }
            0x800ceb30 => {
                c.r[3] = c.r[4].rotate_left(0) & 0x7fff;
                c.stw(c.r[13].wrapping_add(0xffffa210), c.r[4]);
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa214), c.r[3]);
                c.stb(c.r[13].wrapping_add(0xffffa21c), c.r[0]);
                pc = 0x800ceb44;
            }
            0x800ceb44 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800ceb4c; f_8009df64(c);
                pc = 0x800ceb4c;
            }
            0x800ceb4c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa214));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ceac8: bad pc {:#x}", pc),
        }
    }
}

/// DBInitInterrupts (0x800ceb64, 84 bytes)
#[inline(never)]
pub fn f_800ceb64(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800ceb64) { return; }
        if c.prof { crate::prof::enter(c, 0x800ceb64); f_800ceb64_b(c, 0x800ceb64); crate::prof::exit(c, 0x800ceb64); return; }
    }
    f_800ceb64_b(c, 0x800ceb64)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ceb64_at(c: &mut Cpu, pc: u32) { f_800ceb64_b(c, pc) }
#[inline(always)]
fn f_800ceb64_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ceb64;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ceb64, pc); }
        match pc {
            0x800ceb64 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x20000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0xffff8000);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800ceb7c; f_8009e304(c);
                pc = 0x800ceb7c;
            }
            0x800ceb7c => {
                c.r[3] = 0x40;
                c.lr = 0x800ceb84; f_8009e304(c);
                pc = 0x800ceb84;
            }
            0x800ceb84 => {
                c.r[3] = 0x800d0000;
                c.r[0] = c.r[3].wrapping_add(0xffffec70);
                c.r[3] = 0x800d0000;
                c.stw(c.r[13].wrapping_add(0xffffa20c), c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xffffec30);
                c.r[3] = 0x19;
                c.lr = 0x800ceba0; f_8009df88(c);
                pc = 0x800ceba0;
            }
            0x800ceba0 => {
                c.r[3] = 0x40;
                c.lr = 0x800ceba8; f_8009e38c(c);
                pc = 0x800ceba8;
            }
            0x800ceba8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ceb64: bad pc {:#x}", pc),
        }
    }
}

/// DBInitComm (0x800cebb8, 120 bytes)
#[inline(never)]
pub fn f_800cebb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cebb8) { return; }
        if c.prof { crate::prof::enter(c, 0x800cebb8); f_800cebb8_b(c, 0x800cebb8); crate::prof::exit(c, 0x800cebb8); return; }
    }
    f_800cebb8_b(c, 0x800cebb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cebb8_at(c: &mut Cpu, pc: u32) { f_800cebb8_b(c, pc) }
#[inline(always)]
fn f_800cebb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cebb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cebb8, pc); }
        match pc {
            0x800cebb8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800cebdc; f_8009df3c(c);
                pc = 0x800cebdc;
            }
            0x800cebdc => {
                c.r[0] = c.r[13].wrapping_add(0xffffa21c);
                c.stw(c.r[13].wrapping_add(0xffffa218), c.r[0]);
                c.r[4] = 0x20000;
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa218));
                c.r[3] = c.r[4].wrapping_add(0xffff8000);
                c.stw(c.r[29], c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa208), c.r[30]);
                c.lr = 0x800cec00; f_8009e304(c);
                pc = 0x800cec00;
            }
            0x800cec00 => {
                c.r[3] = 0xcc000000;
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x6828), c.r[0]);
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800cec14; f_8009df64(c);
                pc = 0x800cec14;
            }
            0x800cec14 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cebb8: bad pc {:#x}", pc),
        }
    }
}

/// DBGHandler (0x800cec30, 64 bytes)
#[inline(never)]
pub fn f_800cec30(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cec30) { return; }
        if c.prof { crate::prof::enter(c, 0x800cec30); f_800cec30_b(c, 0x800cec30); crate::prof::exit(c, 0x800cec30); return; }
    }
    f_800cec30_b(c, 0x800cec30)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cec30_at(c: &mut Cpu, pc: u32) { f_800cec30_b(c, pc) }
#[inline(always)]
fn f_800cec30_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cec30;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cec30, pc); }
        match pc {
            0x800cec30 => {
                c.r[0] = c.mfspr(8);
                c.r[5] = 0xcc000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x1000;
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa20c));
                c.stw(c.r[5].wrapping_add(0x3000), c.r[0]);
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800cec60; continue; }
                pc = 0x800cec54;
            }
            0x800cec54 => {
                c.mtspr(8, c.r[12]);
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                if true { let t = c.lr; c.lr = 0x800cec60; (c.dispatch)(c, t); }
                pc = 0x800cec60;
            }
            0x800cec60 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cec30: bad pc {:#x}", pc),
        }
    }
}

/// MWCallback (0x800cec70, 60 bytes)
#[inline(never)]
pub fn f_800cec70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cec70) { return; }
        if c.prof { crate::prof::enter(c, 0x800cec70); f_800cec70_b(c, 0x800cec70); crate::prof::exit(c, 0x800cec70); return; }
    }
    f_800cec70_b(c, 0x800cec70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cec70_at(c: &mut Cpu, pc: u32) { f_800cec70_b(c, pc) }
#[inline(always)]
fn f_800cec70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cec70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cec70, pc); }
        match pc {
            0x800cec70 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x1;
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa208));
                c.stb(c.r[13].wrapping_add(0xffffa21c), c.r[0]);
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800cec9c; continue; }
                pc = 0x800cec90;
            }
            0x800cec90 => {
                c.mtspr(8, c.r[12]);
                c.r[3] = 0x0;
                if true { let t = c.lr; c.lr = 0x800cec9c; (c.dispatch)(c, t); }
                pc = 0x800cec9c;
            }
            0x800cec9c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cec70: bad pc {:#x}", pc),
        }
    }
}

/// fn_800CECAC (0x800cecac, 172 bytes)
#[inline(never)]
pub fn f_800cecac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cecac) { return; }
        if c.prof { crate::prof::enter(c, 0x800cecac); f_800cecac_b(c, 0x800cecac); crate::prof::exit(c, 0x800cecac); return; }
    }
    f_800cecac_b(c, 0x800cecac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cecac_at(c: &mut Cpu, pc: u32) { f_800cecac_b(c, pc) }
#[inline(always)]
fn f_800cecac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cecac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cecac, pc); }
        match pc {
            0x800cecac => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x2;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x40000000;
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = 0xcc000000;
                c.r[27] = c.r[3].wrapping_add(0x0);
                c.r[29] = c.r[30].wrapping_add(0x6800);
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.r[5] = c.lwz(c.r[30].wrapping_add(0x6828));
                c.r[5] = c.r[5] & 0x405;
                c.rc(c.r[5]);
                c.r[5] = c.r[5] | 0xc0;
                { let ea = c.r[29].wrapping_add(0x28); c.stw(ea, c.r[5]); c.r[29] = ea; }
                c.r[5] = 0x1;
                c.stw(c.r[1].wrapping_add(0x18), c.r[0]);
                c.lr = 0x800cecf0; f_800cefbc(c);
                pc = 0x800cecf0;
            }
            0x800cecf0 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[31] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800cecf8;
            }
            0x800cecf8 => {
                c.r[28] = c.r[30].wrapping_add(0x6800);
                { let ea = c.r[28].wrapping_add(0x34); c.r[0] = c.lwz(ea); c.r[28] = ea; }
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800cecf8; continue; }
                pc = 0x800ced08;
            }
            0x800ced08 => {
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[4] = 0x4;
                c.r[5] = 0x0;
                c.lr = 0x800ced18; f_800cefbc(c);
                pc = 0x800ced18;
            }
            0x800ced18 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[3] = c.r[31] | c.r[0];
                pc = 0x800ced24;
            }
            0x800ced24 => {
                c.r[0] = c.lwz(c.r[28]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ced24; continue; }
                pc = 0x800ced30;
            }
            0x800ced30 => {
                c.r[4] = c.lwz(c.r[29]);
                c.r[0] = c.r[3].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[0] = c.r[4] & 0x405;
                c.rc(c.r[0]);
                c.stw(c.r[29], c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cecac: bad pc {:#x}", pc),
        }
    }
}

/// DBGWrite (0x800ced58, 220 bytes)
#[inline(never)]
pub fn f_800ced58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800ced58) { return; }
        if c.prof { crate::prof::enter(c, 0x800ced58); f_800ced58_b(c, 0x800ced58); crate::prof::exit(c, 0x800ced58); return; }
    }
    f_800ced58_b(c, 0x800ced58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800ced58_at(c: &mut Cpu, pc: u32) { f_800ced58_b(c, pc) }
#[inline(always)]
fn f_800ced58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800ced58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800ced58, pc); }
        match pc {
            0x800ced58 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[3].rotate_left(8) & 0x1fffc00;
                c.r[0] = c.r[0] | 0xa0000000;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x28); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = 0xcc000000;
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.r[26] = c.r[4].wrapping_add(0x0);
                c.r[31] = c.r[29].wrapping_add(0x6800);
                c.r[3] = c.r[1].wrapping_add(0x24);
                c.r[4] = 0x4;
                c.r[5] = 0x1;
                c.r[6] = c.lwz(c.r[29].wrapping_add(0x6828));
                c.r[6] = c.r[6] & 0x405;
                c.rc(c.r[6]);
                c.r[6] = c.r[6] | 0xc0;
                { let ea = c.r[31].wrapping_add(0x28); c.stw(ea, c.r[6]); c.r[31] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.lr = 0x800ceda4; f_800cefbc(c);
                pc = 0x800ceda4;
            }
            0x800ceda4 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[27] = c.r[0] | c.r[0];
                pc = 0x800cedb0;
            }
            0x800cedb0 => {
                c.r[28] = c.r[29].wrapping_add(0x6800);
                { let ea = c.r[28].wrapping_add(0x34); c.r[0] = c.lwz(ea); c.r[28] = ea; }
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800cedb0; continue; }
                pc = 0x800cedc0;
            }
            0x800cedc0 => {
                pc = 0x800cee04; continue;
                pc = 0x800cedc4;
            }
            0x800cedc4 => {
                c.r[0] = c.lwz(c.r[26]);
                c.r[3] = c.r[1].wrapping_add(0x20);
                c.r[4] = 0x4;
                c.stw(c.r[1].wrapping_add(0x20), c.r[0]);
                c.r[5] = 0x1;
                c.r[26] = c.r[26].wrapping_add(0x4);
                c.lr = 0x800cede0; f_800cefbc(c);
                pc = 0x800cede0;
            }
            0x800cede0 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[27] = c.r[27] | c.r[0];
                pc = 0x800cedec;
            }
            0x800cedec => {
                c.r[0] = c.lwz(c.r[28]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800cedec; continue; }
                pc = 0x800cedf8;
            }
            0x800cedf8 => {
                c.r[30] = c.addc(c.r[30], 0xfffffffc, 0);
                c.rc(c.r[30]);
                if !c.crbit(0) { pc = 0x800cee04; continue; }
                pc = 0x800cee00;
            }
            0x800cee00 => {
                c.r[30] = 0x0;
                pc = 0x800cee04;
            }
            0x800cee04 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x800cedc4; continue; }
                pc = 0x800cee0c;
            }
            0x800cee0c => {
                c.r[4] = c.lwz(c.r[31]);
                c.r[0] = c.r[27].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[0] = c.r[4] & 0x405;
                c.rc(c.r[0]);
                c.stw(c.r[31], c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x28); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800ced58: bad pc {:#x}", pc),
        }
    }
}

/// DBGRead (0x800cee34, 220 bytes)
#[inline(never)]
pub fn f_800cee34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cee34) { return; }
        if c.prof { crate::prof::enter(c, 0x800cee34); f_800cee34_b(c, 0x800cee34); crate::prof::exit(c, 0x800cee34); return; }
    }
    f_800cee34_b(c, 0x800cee34)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cee34_at(c: &mut Cpu, pc: u32) { f_800cee34_b(c, pc) }
#[inline(always)]
fn f_800cee34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cee34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cee34, pc); }
        match pc {
            0x800cee34 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[3].rotate_left(8) & 0x1fffc00;
                c.r[0] = c.r[0] | 0x20000000;
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x28); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = 0xcc000000;
                c.r[30] = c.r[5].wrapping_add(0x0);
                c.r[26] = c.r[4].wrapping_add(0x0);
                c.r[31] = c.r[29].wrapping_add(0x6800);
                c.r[3] = c.r[1].wrapping_add(0x24);
                c.r[4] = 0x4;
                c.r[5] = 0x1;
                c.r[6] = c.lwz(c.r[29].wrapping_add(0x6828));
                c.r[6] = c.r[6] & 0x405;
                c.rc(c.r[6]);
                c.r[6] = c.r[6] | 0xc0;
                { let ea = c.r[31].wrapping_add(0x28); c.stw(ea, c.r[6]); c.r[31] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.lr = 0x800cee80; f_800cefbc(c);
                pc = 0x800cee80;
            }
            0x800cee80 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[27] = c.r[0] | c.r[0];
                pc = 0x800cee8c;
            }
            0x800cee8c => {
                c.r[28] = c.r[29].wrapping_add(0x6800);
                { let ea = c.r[28].wrapping_add(0x34); c.r[0] = c.lwz(ea); c.r[28] = ea; }
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800cee8c; continue; }
                pc = 0x800cee9c;
            }
            0x800cee9c => {
                pc = 0x800ceee0; continue;
                pc = 0x800ceea0;
            }
            0x800ceea0 => {
                c.r[3] = c.r[1].wrapping_add(0x20);
                c.r[4] = 0x4;
                c.r[5] = 0x0;
                c.lr = 0x800ceeb0; f_800cefbc(c);
                pc = 0x800ceeb0;
            }
            0x800ceeb0 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[27] = c.r[27] | c.r[0];
                pc = 0x800ceebc;
            }
            0x800ceebc => {
                c.r[0] = c.lwz(c.r[28]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800ceebc; continue; }
                pc = 0x800ceec8;
            }
            0x800ceec8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[30] = c.addc(c.r[30], 0xfffffffc, 0);
                c.rc(c.r[30]);
                c.stw(c.r[26], c.r[0]);
                c.r[26] = c.r[26].wrapping_add(0x4);
                if !c.crbit(0) { pc = 0x800ceee0; continue; }
                pc = 0x800ceedc;
            }
            0x800ceedc => {
                c.r[30] = 0x0;
                pc = 0x800ceee0;
            }
            0x800ceee0 => {
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x800ceea0; continue; }
                pc = 0x800ceee8;
            }
            0x800ceee8 => {
                c.r[4] = c.lwz(c.r[31]);
                c.r[0] = c.r[27].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[0] = c.r[4] & 0x405;
                c.rc(c.r[0]);
                c.stw(c.r[31], c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x28); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[1] = c.r[1].wrapping_add(0x40);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cee34: bad pc {:#x}", pc),
        }
    }
}

/// fn_800CEF10 (0x800cef10, 172 bytes)
#[inline(never)]
pub fn f_800cef10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cef10) { return; }
        if c.prof { crate::prof::enter(c, 0x800cef10); f_800cef10_b(c, 0x800cef10); crate::prof::exit(c, 0x800cef10); return; }
    }
    f_800cef10_b(c, 0x800cef10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cef10_at(c: &mut Cpu, pc: u32) { f_800cef10_b(c, pc) }
#[inline(always)]
fn f_800cef10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cef10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cef10, pc); }
        match pc {
            0x800cef10 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x2;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x60000000;
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = 0xcc000000;
                c.r[27] = c.r[3].wrapping_add(0x0);
                c.r[29] = c.r[30].wrapping_add(0x6800);
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.r[5] = c.lwz(c.r[30].wrapping_add(0x6828));
                c.r[5] = c.r[5] & 0x405;
                c.rc(c.r[5]);
                c.r[5] = c.r[5] | 0xc0;
                { let ea = c.r[29].wrapping_add(0x28); c.stw(ea, c.r[5]); c.r[29] = ea; }
                c.r[5] = 0x1;
                c.stw(c.r[1].wrapping_add(0x18), c.r[0]);
                c.lr = 0x800cef54; f_800cefbc(c);
                pc = 0x800cef54;
            }
            0x800cef54 => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[31] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800cef5c;
            }
            0x800cef5c => {
                c.r[28] = c.r[30].wrapping_add(0x6800);
                { let ea = c.r[28].wrapping_add(0x34); c.r[0] = c.lwz(ea); c.r[28] = ea; }
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800cef5c; continue; }
                pc = 0x800cef6c;
            }
            0x800cef6c => {
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[4] = 0x4;
                c.r[5] = 0x0;
                c.lr = 0x800cef7c; f_800cefbc(c);
                pc = 0x800cef7c;
            }
            0x800cef7c => {
                c.r[0] = c.r[3].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[3] = c.r[31] | c.r[0];
                pc = 0x800cef88;
            }
            0x800cef88 => {
                c.r[0] = c.lwz(c.r[28]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800cef88; continue; }
                pc = 0x800cef94;
            }
            0x800cef94 => {
                c.r[4] = c.lwz(c.r[29]);
                c.r[0] = c.r[3].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[0] = c.r[4] & 0x405;
                c.rc(c.r[0]);
                c.stw(c.r[29], c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cef10: bad pc {:#x}", pc),
        }
    }
}

/// DBGEXIImm (0x800cefbc, 664 bytes)
#[inline(never)]
pub fn f_800cefbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cefbc) { return; }
        if c.prof { crate::prof::enter(c, 0x800cefbc); f_800cefbc_b(c, 0x800cefbc); crate::prof::exit(c, 0x800cefbc); return; }
    }
    f_800cefbc_b(c, 0x800cefbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cefbc_at(c: &mut Cpu, pc: u32) { f_800cefbc_b(c, pc) }
#[inline(always)]
fn f_800cefbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cefbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cefbc, pc); }
        match pc {
            0x800cefbc => {
                { let ea = c.r[1].wrapping_add(0xffffffb8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.cmpu(0, c.r[5], 0x0);
                { let mut ea = c.r[1].wrapping_add(0x20); for n in 22..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                if c.crbit(2) { pc = 0x800cf100; continue; }
                pc = 0x800cefcc;
            }
            0x800cefcc => {
                c.r[29] = 0x0;
                c.cmps(0, c.r[29], c.r[4]);
                c.r[30] = 0x0;
                if !c.crbit(0) { pc = 0x800cf0f8; continue; }
                pc = 0x800cefdc;
            }
            0x800cefdc => {
                c.cmps(0, c.r[4], 0x8);
                c.r[6] = c.r[4].wrapping_add(0xfffffff8);
                if !c.crbit(1) { pc = 0x800cf240; continue; }
                pc = 0x800cefe8;
            }
            0x800cefe8 => {
                c.r[0] = c.r[6].wrapping_add(0x7);
                c.r[0] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.cmps(0, c.r[6], 0x0);
                c.mtspr(9, c.r[0]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                if !c.crbit(1) { pc = 0x800cf240; continue; }
                pc = 0x800cf000;
            }
            0x800cf000 => {
                c.r[6] = c.addc(!c.r[29], 0x3, 1);
                c.r[12] = c.lbz(c.r[31]);
                c.r[0] = c.r[29].wrapping_add(0x1);
                c.r[11] = c.lbz(c.r[31].wrapping_add(0x1));
                c.r[10] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[9] = c.lbz(c.r[31].wrapping_add(0x2));
                c.r[8] = c.addc(!c.r[0], 0x3, 1);
                c.r[7] = c.lbz(c.r[31].wrapping_add(0x3));
                c.r[6] = c.r[29].wrapping_add(0x2);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x4));
                c.r[12] = slw(c.r[12], c.r[10]);
                c.r[23] = c.lbz(c.r[31].wrapping_add(0x5));
                c.r[10] = c.r[8].rotate_left(3) & 0xfffffff8;
                c.r[25] = c.lbz(c.r[31].wrapping_add(0x6));
                c.r[6] = c.addc(!c.r[6], 0x3, 1);
                c.r[27] = c.lbz(c.r[31].wrapping_add(0x7));
                c.r[8] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[6] = c.r[29].wrapping_neg();
                c.r[22] = c.r[29].wrapping_add(0x4);
                c.r[6] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[22] = c.addc(!c.r[22], 0x3, 1);
                c.r[24] = c.r[29].wrapping_add(0x5);
                c.r[22] = c.r[22].rotate_left(3) & 0xfffffff8;
                c.r[24] = c.addc(!c.r[24], 0x3, 1);
                c.r[26] = c.r[29].wrapping_add(0x6);
                c.r[24] = c.r[24].rotate_left(3) & 0xfffffff8;
                c.r[26] = c.addc(!c.r[26], 0x3, 1);
                c.r[28] = c.r[29].wrapping_add(0x7);
                c.r[26] = c.r[26].rotate_left(3) & 0xfffffff8;
                c.r[28] = c.addc(!c.r[28], 0x3, 1);
                c.r[28] = c.r[28].rotate_left(3) & 0xfffffff8;
                c.r[30] = c.r[30] | c.r[12];
                c.r[10] = slw(c.r[11], c.r[10]);
                c.r[30] = c.r[30] | c.r[10];
                c.r[8] = slw(c.r[9], c.r[8]);
                c.r[30] = c.r[30] | c.r[8];
                c.r[6] = slw(c.r[7], c.r[6]);
                c.r[30] = c.r[30] | c.r[6];
                c.r[0] = slw(c.r[0], c.r[22]);
                c.r[30] = c.r[30] | c.r[0];
                c.r[0] = slw(c.r[23], c.r[24]);
                c.r[30] = c.r[30] | c.r[0];
                c.r[0] = slw(c.r[25], c.r[26]);
                c.r[30] = c.r[30] | c.r[0];
                c.r[0] = slw(c.r[27], c.r[28]);
                c.r[30] = c.r[30] | c.r[0];
                c.r[31] = c.r[31].wrapping_add(0x8);
                c.r[29] = c.r[29].wrapping_add(0x8);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cf000; continue; }
                pc = 0x800cf0c4;
            }
            0x800cf0c4 => {
                pc = 0x800cf240; continue;
                pc = 0x800cf0c8;
            }
            0x800cf0c8 => {
                c.r[0] = c.r[4].wrapping_sub(c.r[29]);
                c.cmps(0, c.r[29], c.r[4]);
                c.mtspr(9, c.r[0]);
                if !c.crbit(0) { pc = 0x800cf0f8; continue; }
                pc = 0x800cf0d8;
            }
            0x800cf0d8 => {
                c.r[0] = c.addc(!c.r[29], 0x3, 1);
                c.r[6] = c.lbz(c.r[7]);
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = slw(c.r[6], c.r[0]);
                c.r[30] = c.r[30] | c.r[0];
                c.r[7] = c.r[7].wrapping_add(0x1);
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cf0d8; continue; }
                pc = 0x800cf0f8;
            }
            0x800cf0f8 => {
                c.r[6] = 0xcc000000;
                c.stw(c.r[6].wrapping_add(0x6838), c.r[30]);
                pc = 0x800cf100;
            }
            0x800cf100 => {
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                c.r[6] = 0xcc000000;
                c.r[7] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[8] = c.r[6].wrapping_add(0x6800);
                c.r[6] = c.r[7] | 0x1;
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[0] = c.r[6] | c.r[0];
                { let ea = c.r[8].wrapping_add(0x34); c.stw(ea, c.r[0]); c.r[8] = ea; }
                pc = 0x800cf120;
            }
            0x800cf120 => {
                c.r[0] = c.lwz(c.r[8]);
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800cf120; continue; }
                pc = 0x800cf12c;
            }
            0x800cf12c => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x800cf238; continue; }
                pc = 0x800cf134;
            }
            0x800cf134 => {
                c.r[5] = 0x0;
                c.r[6] = 0xcc000000;
                c.cmps(0, c.r[5], c.r[4]);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x6838));
                if !c.crbit(0) { pc = 0x800cf238; continue; }
                pc = 0x800cf148;
            }
            0x800cf148 => {
                c.cmps(0, c.r[4], 0x8);
                c.r[7] = c.r[4].wrapping_add(0xfffffff8);
                if !c.crbit(1) { pc = 0x800cf20c; continue; }
                pc = 0x800cf154;
            }
            0x800cf154 => {
                c.r[6] = c.r[7].wrapping_add(0x7);
                c.r[6] = c.r[6].rotate_left(29) & 0x1fffffff;
                c.cmps(0, c.r[7], 0x0);
                c.mtspr(9, c.r[6]);
                if !c.crbit(1) { pc = 0x800cf20c; continue; }
                pc = 0x800cf168;
            }
            0x800cf168 => {
                c.r[6] = c.addc(!c.r[5], 0x3, 1);
                c.r[7] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[6] = c.r[5].wrapping_add(0x1);
                c.r[8] = srw(c.r[0], c.r[7]);
                c.r[6] = c.addc(!c.r[6], 0x3, 1);
                c.stb(c.r[3], c.r[8]);
                c.r[7] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[6] = c.r[5].wrapping_add(0x2);
                c.r[12] = srw(c.r[0], c.r[7]);
                c.r[6] = c.addc(!c.r[6], 0x3, 1);
                c.stb(c.r[3].wrapping_add(0x1), c.r[12]);
                c.r[6] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[11] = srw(c.r[0], c.r[6]);
                c.r[6] = c.r[5].wrapping_neg();
                c.stb(c.r[3].wrapping_add(0x2), c.r[11]);
                c.r[7] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[6] = c.r[5].wrapping_add(0x4);
                c.r[10] = srw(c.r[0], c.r[7]);
                c.r[6] = c.addc(!c.r[6], 0x3, 1);
                c.stb(c.r[3].wrapping_add(0x3), c.r[10]);
                c.r[7] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[6] = c.r[5].wrapping_add(0x5);
                c.r[9] = srw(c.r[0], c.r[7]);
                c.r[6] = c.addc(!c.r[6], 0x3, 1);
                c.stb(c.r[3].wrapping_add(0x4), c.r[9]);
                c.r[7] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[8] = srw(c.r[0], c.r[7]);
                c.r[6] = c.r[5].wrapping_add(0x6);
                c.stb(c.r[3].wrapping_add(0x5), c.r[8]);
                c.r[7] = c.addc(!c.r[6], 0x3, 1);
                c.r[6] = c.r[5].wrapping_add(0x7);
                c.r[7] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[7] = srw(c.r[0], c.r[7]);
                c.r[6] = c.addc(!c.r[6], 0x3, 1);
                c.stb(c.r[3].wrapping_add(0x6), c.r[7]);
                c.r[6] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[6] = srw(c.r[0], c.r[6]);
                c.stb(c.r[3].wrapping_add(0x7), c.r[6]);
                c.r[3] = c.r[3].wrapping_add(0x8);
                c.r[5] = c.r[5].wrapping_add(0x8);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cf168; continue; }
                pc = 0x800cf20c;
            }
            0x800cf20c => {
                c.r[6] = c.r[4].wrapping_sub(c.r[5]);
                c.cmps(0, c.r[5], c.r[4]);
                c.mtspr(9, c.r[6]);
                if !c.crbit(0) { pc = 0x800cf238; continue; }
                pc = 0x800cf21c;
            }
            0x800cf21c => {
                c.r[4] = c.addc(!c.r[5], 0x3, 1);
                c.r[4] = c.r[4].rotate_left(3) & 0xfffffff8;
                c.r[4] = srw(c.r[0], c.r[4]);
                c.stb(c.r[3], c.r[4]);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cf21c; continue; }
                pc = 0x800cf238;
            }
            0x800cf238 => {
                c.r[3] = 0x1;
                pc = 0x800cf248; continue;
                pc = 0x800cf240;
            }
            0x800cf240 => {
                c.r[7] = c.r[3].wrapping_add(c.r[29]);
                pc = 0x800cf0c8; continue;
                pc = 0x800cf248;
            }
            0x800cf248 => {
                { let mut ea = c.r[1].wrapping_add(0x20); for n in 22..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x48);
                if true { return; }
                return;
            }
            _ => panic!("f_800cefbc: bad pc {:#x}", pc),
        }
    }
}

/// __EXIProbe (0x800cf254, 372 bytes)
#[inline(never)]
pub fn f_800cf254(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cf254) { return; }
        if c.prof { crate::prof::enter(c, 0x800cf254); f_800cf254_b(c, 0x800cf254); crate::prof::exit(c, 0x800cf254); return; }
    }
    f_800cf254_b(c, 0x800cf254)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cf254_at(c: &mut Cpu, pc: u32) { f_800cf254_b(c, pc) }
#[inline(always)]
fn f_800cf254_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cf254;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cf254, pc); }
        match pc {
            0x800cf254 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[3].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xfffffef0);
                c.r[31] = c.r[0].wrapping_add(c.r[4]);
                c.cmps(0, c.r[28], 0x2);
                if !c.crbit(2) { pc = 0x800cf288; continue; }
                pc = 0x800cf280;
            }
            0x800cf280 => {
                c.r[3] = 0x1;
                pc = 0x800cf3b4; continue;
                pc = 0x800cf288;
            }
            0x800cf288 => {
                c.r[29] = 0x1;
                c.lr = 0x800cf290; f_8009df3c(c);
                pc = 0x800cf290;
            }
            0x800cf290 => {
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.r[28].wrapping_mul(0x14);
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6800);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[5] = c.lwz(c.r[3]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800cf37c; continue; }
                pc = 0x800cf2b4;
            }
            0x800cf2b4 => {
                c.r[0] = c.r[5].rotate_left(0) & 0x800;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800cf2e4; continue; }
                pc = 0x800cf2bc;
            }
            0x800cf2bc => {
                c.r[0] = c.lwz(c.r[3]);
                c.r[0] = c.r[0] & 0x7f5;
                c.rc(c.r[0]);
                c.r[0] = c.r[0] | 0x800;
                c.stw(c.r[3], c.r[0]);
                c.r[4] = 0x0;
                c.stw(c.r[31].wrapping_add(0x20), c.r[4]);
                c.r[0] = c.r[28].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                pc = 0x800cf2e4;
            }
            0x800cf2e4 => {
                c.r[0] = c.r[5].rotate_left(0) & 0x1000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800cf35c; continue; }
                pc = 0x800cf2ec;
            }
            0x800cf2ec => {
                c.r[31] = 0x80000000;
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xf8));
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[3] = 0x10620000;
                c.r[3] = c.r[3].wrapping_add(0x4dd3);
                c.r[0] = mulhwu(c.r[3], c.r[0]);
                c.r[27] = c.r[0].rotate_left(26) & 0x3ffffff;
                c.lr = 0x800cf30c; crate::stubs::f_800a2778(c);
                pc = 0x800cf30c;
            }
            0x800cf30c => {
                c.r[5] = 0x0;
                c.r[6] = c.r[27].wrapping_add(0x0);
                c.lr = 0x800cf318; f_800c4928(c);
                pc = 0x800cf318;
            }
            0x800cf318 => {
                c.r[5] = 0x0;
                c.r[6] = 0x64;
                c.lr = 0x800cf324; f_800c4928(c);
                pc = 0x800cf324;
            }
            0x800cf324 => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[28].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[31].wrapping_add(0x30c0);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800cf344; continue; }
                pc = 0x800cf340;
            }
            0x800cf340 => {
                c.stw(c.r[3], c.r[4]);
                pc = 0x800cf344;
            }
            0x800cf344 => {
                c.r[0] = c.lwz(c.r[3]);
                c.r[0] = c.r[4].wrapping_sub(c.r[0]);
                c.cmps(0, c.r[0], 0x3);
                if !c.crbit(0) { pc = 0x800cf3a8; continue; }
                pc = 0x800cf354;
            }
            0x800cf354 => {
                c.r[29] = 0x0;
                pc = 0x800cf3a8; continue;
                pc = 0x800cf35c;
            }
            0x800cf35c => {
                c.r[4] = 0x0;
                c.stw(c.r[31].wrapping_add(0x20), c.r[4]);
                c.r[0] = c.r[28].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                c.r[29] = 0x0;
                pc = 0x800cf3a8; continue;
                pc = 0x800cf37c;
            }
            0x800cf37c => {
                c.r[0] = c.r[5].rotate_left(0) & 0x1000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800cf38c; continue; }
                pc = 0x800cf384;
            }
            0x800cf384 => {
                c.r[0] = c.r[5].rotate_left(0) & 0x800;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800cf3a8; continue; }
                pc = 0x800cf38c;
            }
            0x800cf38c => {
                c.r[4] = 0x0;
                c.stw(c.r[31].wrapping_add(0x20), c.r[4]);
                c.r[0] = c.r[28].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                c.r[29] = 0x0;
                pc = 0x800cf3a8;
            }
            0x800cf3a8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800cf3b0; f_8009df64(c);
                pc = 0x800cf3b0;
            }
            0x800cf3b0 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x800cf3b4;
            }
            0x800cf3b4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cf254: bad pc {:#x}", pc),
        }
    }
}

/// EXIProbeEx (0x800cf3c8, 180 bytes)
#[inline(never)]
pub fn f_800cf3c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cf3c8) { return; }
        if c.prof { crate::prof::enter(c, 0x800cf3c8); f_800cf3c8_b(c, 0x800cf3c8); crate::prof::exit(c, 0x800cf3c8); return; }
    }
    f_800cf3c8_b(c, 0x800cf3c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cf3c8_at(c: &mut Cpu, pc: u32) { f_800cf3c8_b(c, pc) }
#[inline(always)]
fn f_800cf3c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cf3c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cf3c8, pc); }
        match pc {
            0x800cf3c8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.r[3].rotate_left(6) & 0xffffffc0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0xfffffef0);
                c.r[31] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.lr = 0x800cf3f8; f_800cf254(c);
                pc = 0x800cf3f8;
            }
            0x800cf3f8 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800cf430; continue; }
                pc = 0x800cf400;
            }
            0x800cf400 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x20));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800cf430; continue; }
                pc = 0x800cf40c;
            }
            0x800cf40c => {
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.r[5] = c.r[1].wrapping_add(0xc);
                c.lr = 0x800cf41c; f_80099400(c);
                pc = 0x800cf41c;
            }
            0x800cf41c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800cf42c; continue; }
                pc = 0x800cf424;
            }
            0x800cf424 => {
                c.r[3] = 0x1;
                pc = 0x800cf430; continue;
                pc = 0x800cf42c;
            }
            0x800cf42c => {
                c.r[3] = 0x0;
                pc = 0x800cf430;
            }
            0x800cf430 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800cf440; continue; }
                pc = 0x800cf438;
            }
            0x800cf438 => {
                c.r[3] = 0x1;
                pc = 0x800cf464; continue;
                pc = 0x800cf440;
            }
            0x800cf440 => {
                c.r[0] = c.r[30].rotate_left(2) & 0xfffffffc;
                c.r[3] = 0x80000000;
                c.r[3] = c.r[3].wrapping_add(0x30c0);
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800cf460; continue; }
                pc = 0x800cf458;
            }
            0x800cf458 => {
                c.r[3] = 0x0;
                pc = 0x800cf464; continue;
                pc = 0x800cf460;
            }
            0x800cf460 => {
                c.r[3] = 0xffffffff;
                pc = 0x800cf464;
            }
            0x800cf464 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cf3c8: bad pc {:#x}", pc),
        }
    }
}

/// InitializeUART (0x800cf47c, 112 bytes)
#[inline(never)]
pub fn f_800cf47c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cf47c) { return; }
        if c.prof { crate::prof::enter(c, 0x800cf47c); f_800cf47c_b(c, 0x800cf47c); crate::prof::exit(c, 0x800cf47c); return; }
    }
    f_800cf47c_b(c, 0x800cf47c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cf47c_at(c: &mut Cpu, pc: u32) { f_800cf47c_b(c, pc) }
#[inline(always)]
fn f_800cf47c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cf47c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cf47c, pc); }
        match pc {
            0x800cf47c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa22c));
                c.r[0] = c.r[3].wrapping_add(0x5a010000);
                c.cmpu(0, c.r[0], 0x5a);
                if !c.crbit(2) { pc = 0x800cf4a0; continue; }
                pc = 0x800cf498;
            }
            0x800cf498 => {
                c.r[3] = 0x0;
                pc = 0x800cf4dc; continue;
                pc = 0x800cf4a0;
            }
            0x800cf4a0 => {
                c.lr = 0x800cf4a4; f_800998b8(c);
                pc = 0x800cf4a4;
            }
            0x800cf4a4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x10000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800cf4bc; continue; }
                pc = 0x800cf4ac;
            }
            0x800cf4ac => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa228), c.r[0]);
                c.r[3] = 0x2;
                pc = 0x800cf4dc; continue;
                pc = 0x800cf4bc;
            }
            0x800cf4bc => {
                c.r[3] = 0xa5ff0000;
                c.r[0] = c.r[3].wrapping_add(0x5a);
                c.r[3] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa228), c.r[0]);
                c.r[0] = 0x1;
                c.stw(c.r[13].wrapping_add(0xffffa220), c.r[3]);
                c.r[3] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa224), c.r[0]);
                pc = 0x800cf4dc;
            }
            0x800cf4dc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cf47c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800CF4EC (0x800cf4ec, 540 bytes)
#[inline(never)]
pub fn f_800cf4ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cf4ec) { return; }
        if c.prof { crate::prof::enter(c, 0x800cf4ec); f_800cf4ec_b(c, 0x800cf4ec); crate::prof::exit(c, 0x800cf4ec); return; }
    }
    f_800cf4ec_b(c, 0x800cf4ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cf4ec_at(c: &mut Cpu, pc: u32) { f_800cf4ec_b(c, pc) }
#[inline(always)]
fn f_800cf4ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cf4ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cf4ec, pc); }
        match pc {
            0x800cf4ec => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffc8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa228));
                c.r[0] = c.r[5].wrapping_add(0x5a010000);
                c.cmpu(0, c.r[0], 0x5a);
                if c.crbit(2) { pc = 0x800cf51c; continue; }
                pc = 0x800cf514;
            }
            0x800cf514 => {
                c.r[3] = 0x2;
                pc = 0x800cf6f4; continue;
                pc = 0x800cf51c;
            }
            0x800cf51c => {
                c.lr = 0x800cf520; f_8009df3c(c);
                pc = 0x800cf520;
            }
            0x800cf520 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa224));
                c.r[27] = c.r[0] | c.r[0];
                c.r[5] = 0x0;
                c.lr = 0x800cf538; f_800991e4(c);
                pc = 0x800cf538;
            }
            0x800cf538 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800cf550; continue; }
                pc = 0x800cf540;
            }
            0x800cf540 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x800cf548; f_8009df64(c);
                pc = 0x800cf548;
            }
            0x800cf548 => {
                c.r[3] = 0x0;
                pc = 0x800cf6f4; continue;
                pc = 0x800cf550;
            }
            0x800cf550 => {
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[3] = 0xd;
                pc = 0x800cf570; continue;
                pc = 0x800cf55c;
            }
            0x800cf55c => {
                c.r[0] = c.lbz(c.r[4]);
                c.cmps(0, c.r[0], 0xa);
                if !c.crbit(2) { pc = 0x800cf56c; continue; }
                pc = 0x800cf568;
            }
            0x800cf568 => {
                c.stb(c.r[4], c.r[3]);
                pc = 0x800cf56c;
            }
            0x800cf56c => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x800cf570;
            }
            0x800cf570 => {
                c.r[0] = c.r[4].wrapping_sub(c.r[30]);
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x800cf55c; continue; }
                pc = 0x800cf57c;
            }
            0x800cf57c => {
                c.r[0] = 0xa0010000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[25] = 0x0;
                c.r[29] = 0x20010000;
                pc = 0x800cf6d8; continue;
                pc = 0x800cf590;
            }
            0x800cf590 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.r[5] = 0x3;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa224));
                c.lr = 0x800cf5a0; f_80098b94(c);
                pc = 0x800cf5a0;
            }
            0x800cf5a0 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800cf5b0; continue; }
                pc = 0x800cf5a8;
            }
            0x800cf5a8 => {
                c.r[0] = 0xffffffff;
                pc = 0x800cf608; continue;
                pc = 0x800cf5b0;
            }
            0x800cf5b0 => {
                c.stw(c.r[1].wrapping_add(0x10), c.r[29]);
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.r[5] = 0x4;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x800cf5cc; f_8009820c(c);
                pc = 0x800cf5cc;
            }
            0x800cf5cc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.lr = 0x800cf5d4; f_80098530(c);
                pc = 0x800cf5d4;
            }
            0x800cf5d4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x800cf5ec; f_8009820c(c);
                pc = 0x800cf5ec;
            }
            0x800cf5ec => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.lr = 0x800cf5f4; f_80098530(c);
                pc = 0x800cf5f4;
            }
            0x800cf5f4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.lr = 0x800cf5fc; f_80098cd4(c);
                pc = 0x800cf5fc;
            }
            0x800cf5fc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(8) & 0xff;
                c.r[0] = c.addc(!c.r[0], 0x10, 1);
                pc = 0x800cf608;
            }
            0x800cf608 => {
                c.cmps(0, c.r[0], 0x0);
                c.r[26] = c.r[0] | c.r[0];
                if !c.crbit(0) { pc = 0x800cf61c; continue; }
                pc = 0x800cf614;
            }
            0x800cf614 => {
                c.r[25] = 0x3;
                pc = 0x800cf6e0; continue;
                pc = 0x800cf61c;
            }
            0x800cf61c => {
                c.cmps(0, c.r[0], 0xc);
                if !c.crbit(0) { pc = 0x800cf62c; continue; }
                pc = 0x800cf624;
            }
            0x800cf624 => {
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x800cf6d8; continue; }
                pc = 0x800cf62c;
            }
            0x800cf62c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.r[5] = 0x3;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa224));
                c.lr = 0x800cf63c; f_80098b94(c);
                pc = 0x800cf63c;
            }
            0x800cf63c => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800cf64c; continue; }
                pc = 0x800cf644;
            }
            0x800cf644 => {
                c.r[25] = 0x3;
                pc = 0x800cf6e0; continue;
                pc = 0x800cf64c;
            }
            0x800cf64c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[5] = 0x4;
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x800cf664; f_8009820c(c);
                pc = 0x800cf664;
            }
            0x800cf664 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.lr = 0x800cf66c; f_80098530(c);
                pc = 0x800cf66c;
            }
            0x800cf66c => {
                pc = 0x800cf6c0; continue;
                pc = 0x800cf670;
            }
            0x800cf670 => {
                c.cmps(0, c.r[26], 0x4);
                if !c.crbit(0) { pc = 0x800cf680; continue; }
                pc = 0x800cf678;
            }
            0x800cf678 => {
                c.cmpu(0, c.r[26], c.r[31]);
                if c.crbit(0) { pc = 0x800cf6d0; continue; }
                pc = 0x800cf680;
            }
            0x800cf680 => {
                c.cmpu(0, c.r[31], 0x4);
                if !c.crbit(0) { pc = 0x800cf690; continue; }
                pc = 0x800cf688;
            }
            0x800cf688 => {
                c.r[28] = c.r[31] | c.r[31];
                pc = 0x800cf694; continue;
                pc = 0x800cf690;
            }
            0x800cf690 => {
                c.r[28] = 0x4;
                pc = 0x800cf694;
            }
            0x800cf694 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.r[5] = c.r[28] | c.r[28];
                c.r[4] = c.r[30].wrapping_add(0x0);
                c.r[6] = 0x1;
                c.r[7] = 0x0;
                c.lr = 0x800cf6ac; f_8009820c(c);
                pc = 0x800cf6ac;
            }
            0x800cf6ac => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.r[30] = c.r[30].wrapping_add(c.r[28]);
                c.r[31] = c.r[31].wrapping_sub(c.r[28]);
                c.r[26] = c.r[26].wrapping_sub(c.r[28]);
                c.lr = 0x800cf6c0; f_80098530(c);
                pc = 0x800cf6c0;
            }
            0x800cf6c0 => {
                c.cmps(0, c.r[26], 0x0);
                if c.crbit(2) { pc = 0x800cf6d0; continue; }
                pc = 0x800cf6c8;
            }
            0x800cf6c8 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x800cf670; continue; }
                pc = 0x800cf6d0;
            }
            0x800cf6d0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.lr = 0x800cf6d8; f_80098cd4(c);
                pc = 0x800cf6d8;
            }
            0x800cf6d8 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x800cf590; continue; }
                pc = 0x800cf6e0;
            }
            0x800cf6e0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa220));
                c.lr = 0x800cf6e8; f_800992e0(c);
                pc = 0x800cf6e8;
            }
            0x800cf6e8 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x800cf6f0; f_8009df64(c);
                pc = 0x800cf6f0;
            }
            0x800cf6f0 => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x800cf6f4;
            }
            0x800cf6f4 => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x3c));
                c.r[1] = c.r[1].wrapping_add(0x38);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cf4ec: bad pc {:#x}", pc),
        }
    }
}

/// fn_800CF708 (0x800cf708, 32 bytes)
#[inline(never)]
pub fn f_800cf708(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cf708) { return; }
        if c.prof { crate::prof::enter(c, 0x800cf708); f_800cf708_b(c, 0x800cf708); crate::prof::exit(c, 0x800cf708); return; }
    }
    f_800cf708_b(c, 0x800cf708)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cf708_at(c: &mut Cpu, pc: u32) { f_800cf708_b(c, pc) }
#[inline(always)]
fn f_800cf708_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cf708;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cf708, pc); }
        match pc {
            0x800cf708 => {
                c.r[3] = 0x80310000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x3f8c));
                c.cmps(0, c.r[0], 0xffffffff);
                if c.crbit(2) { pc = 0x800cf720; continue; }
                pc = 0x800cf718;
            }
            0x800cf718 => {
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x800cf720;
            }
            0x800cf720 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_800cf708: bad pc {:#x}", pc),
        }
    }
}

/// SIIsChanBusy (0x800cf728, 60 bytes)
#[inline(never)]
pub fn f_800cf728(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cf728) { return; }
        if c.prof { crate::prof::enter(c, 0x800cf728); f_800cf728_b(c, 0x800cf728); crate::prof::exit(c, 0x800cf728); return; }
    }
    f_800cf728_b(c, 0x800cf728)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cf728_at(c: &mut Cpu, pc: u32) { f_800cf728_b(c, pc) }
#[inline(always)]
fn f_800cf728_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cf728;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cf728, pc); }
        match pc {
            0x800cf728 => {
                c.r[4] = 0x80400000;
                c.r[5] = c.r[3].rotate_left(5) & 0xffffffe0;
                c.r[0] = c.r[4].wrapping_add(0xffffffb0);
                c.r[4] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = c.lwz(c.r[4]);
                c.r[5] = 0x1;
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(2) { pc = 0x800cf75c; continue; }
                pc = 0x800cf748;
            }
            0x800cf748 => {
                c.r[4] = 0x80310000;
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x3f8c));
                c.cmps(0, c.r[0], c.r[3]);
                if c.crbit(2) { pc = 0x800cf75c; continue; }
                pc = 0x800cf758;
            }
            0x800cf758 => {
                c.r[5] = 0x0;
                pc = 0x800cf75c;
            }
            0x800cf75c => {
                c.r[3] = c.r[5] | c.r[5];
                if true { return; }
                return;
            }
            _ => panic!("f_800cf728: bad pc {:#x}", pc),
        }
    }
}

/// CompleteTransfer (0x800cf764, 764 bytes)
#[inline(never)]
pub fn f_800cf764(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cf764) { return; }
        if c.prof { crate::prof::enter(c, 0x800cf764); f_800cf764_b(c, 0x800cf764); crate::prof::exit(c, 0x800cf764); return; }
    }
    f_800cf764_b(c, 0x800cf764)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cf764_at(c: &mut Cpu, pc: u32) { f_800cf764_b(c, pc) }
#[inline(always)]
fn f_800cf764_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cf764;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cf764, pc); }
        match pc {
            0x800cf764 => {
                c.r[0] = c.mfspr(8);
                c.r[3] = 0xcc000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x6400);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[30].wrapping_add(0x34);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x6434));
                c.r[28] = c.lwz(c.r[3].wrapping_add(0x6438));
                c.r[3] = 0x80310000;
                c.r[0] = c.r[0] | 0x80000000;
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffe;
                c.stw(c.r[30].wrapping_add(0x34), c.r[0]);
                c.r[31] = c.r[3].wrapping_add(0x3f48);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0xffffffff);
                if c.crbit(2) { pc = 0x800cfa24; continue; }
                pc = 0x800cf7b4;
            }
            0x800cf7b4 => {
                c.lr = 0x800cf7b8; f_800a2798(c);
                pc = 0x800cf7b8;
            }
            0x800cf7b8 => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0x44));
                c.r[5] = 0x80400000;
                c.r[0] = c.r[5].wrapping_add(0xf0);
                c.r[5] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                c.stw(c.r[5].wrapping_add(0x4), c.r[4]);
                c.r[6] = c.r[31].wrapping_add(0x4c);
                c.r[4] = 0x0;
                c.stw(c.r[5], c.r[3]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4c));
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x50));
                c.r[7] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(1) { pc = 0x800cf88c; continue; }
                pc = 0x800cf7f0;
            }
            0x800cf7f0 => {
                c.cmpu(0, c.r[7], 0x8);
                c.r[3] = c.r[7].wrapping_add(0xfffffff8);
                if !c.crbit(1) { pc = 0x800cfa2c; continue; }
                pc = 0x800cf7fc;
            }
            0x800cf7fc => {
                c.r[0] = c.r[3].wrapping_add(0x7);
                c.r[0] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.cmpu(0, c.r[3], 0x0);
                c.mtspr(9, c.r[0]);
                if !c.crbit(1) { pc = 0x800cfa2c; continue; }
                pc = 0x800cf810;
            }
            0x800cf810 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x80));
                c.r[4] = c.r[4].wrapping_add(0x8);
                c.stw(c.r[5], c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x84));
                c.stw(c.r[5].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x88));
                c.stw(c.r[5].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x8c));
                c.stw(c.r[5].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x90));
                c.stw(c.r[5].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x94));
                c.stw(c.r[5].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x98));
                c.stw(c.r[5].wrapping_add(0x18), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x9c));
                c.r[30] = c.r[30].wrapping_add(0x20);
                c.stw(c.r[5].wrapping_add(0x1c), c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cf810; continue; }
                pc = 0x800cf860;
            }
            0x800cf860 => {
                pc = 0x800cfa2c; continue;
                pc = 0x800cf864;
            }
            0x800cf864 => {
                c.r[0] = c.r[7].wrapping_sub(c.r[4]);
                c.cmpu(0, c.r[4], c.r[7]);
                c.mtspr(9, c.r[0]);
                if !c.crbit(0) { pc = 0x800cf88c; continue; }
                pc = 0x800cf874;
            }
            0x800cf874 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x80));
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[5], c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0x4);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cf874; continue; }
                pc = 0x800cf88c;
            }
            0x800cf88c => {
                c.r[0] = c.lwz(c.r[6]);
                c.r[3] = c.r[0].rotate_left(0) & 0x3;
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x800cf9a4; continue; }
                pc = 0x800cf898;
            }
            0x800cf898 => {
                c.r[6] = 0xcc000000;
                c.r[6] = c.r[6].wrapping_add(0x6400);
                c.r[0] = c.r[4].rotate_left(2) & 0xfffffffc;
                c.r[4] = c.r[6].wrapping_add(c.r[0]);
                c.r[6] = c.lwz(c.r[4].wrapping_add(0x80));
                c.r[4] = 0x0;
                if !c.crbit(1) { pc = 0x800cf9a4; continue; }
                pc = 0x800cf8b4;
            }
            0x800cf8b4 => {
                c.cmpu(0, c.r[3], 0x8);
                c.r[7] = c.r[3].wrapping_add(0xfffffff8);
                if !c.crbit(1) { pc = 0x800cf978; continue; }
                pc = 0x800cf8c0;
            }
            0x800cf8c0 => {
                c.r[0] = c.r[7].wrapping_add(0x7);
                c.r[0] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.cmpu(0, c.r[7], 0x0);
                c.mtspr(9, c.r[0]);
                if !c.crbit(1) { pc = 0x800cf978; continue; }
                pc = 0x800cf8d4;
            }
            0x800cf8d4 => {
                c.r[0] = c.addc(!c.r[4], 0x3, 1);
                c.r[7] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.r[7] = srw(c.r[6], c.r[7]);
                c.r[0] = c.addc(!c.r[0], 0x3, 1);
                c.stb(c.r[5], c.r[7]);
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[7] = c.r[4].wrapping_add(0x2);
                c.r[0] = srw(c.r[6], c.r[0]);
                c.r[7] = c.addc(!c.r[7], 0x3, 1);
                c.stb(c.r[5].wrapping_add(0x1), c.r[0]);
                c.r[7] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[12] = srw(c.r[6], c.r[7]);
                c.r[7] = c.r[4].wrapping_neg();
                c.stb(c.r[5].wrapping_add(0x2), c.r[12]);
                c.r[8] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[7] = c.r[4].wrapping_add(0x4);
                c.r[11] = srw(c.r[6], c.r[8]);
                c.r[7] = c.addc(!c.r[7], 0x3, 1);
                c.stb(c.r[5].wrapping_add(0x3), c.r[11]);
                c.r[8] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[7] = c.r[4].wrapping_add(0x5);
                c.r[10] = srw(c.r[6], c.r[8]);
                c.r[7] = c.addc(!c.r[7], 0x3, 1);
                c.stb(c.r[5].wrapping_add(0x4), c.r[10]);
                c.r[8] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[9] = srw(c.r[6], c.r[8]);
                c.r[7] = c.r[4].wrapping_add(0x6);
                c.stb(c.r[5].wrapping_add(0x5), c.r[9]);
                c.r[8] = c.addc(!c.r[7], 0x3, 1);
                c.r[7] = c.r[4].wrapping_add(0x7);
                c.r[8] = c.r[8].rotate_left(3) & 0xfffffff8;
                c.r[8] = srw(c.r[6], c.r[8]);
                c.r[7] = c.addc(!c.r[7], 0x3, 1);
                c.stb(c.r[5].wrapping_add(0x6), c.r[8]);
                c.r[7] = c.r[7].rotate_left(3) & 0xfffffff8;
                c.r[7] = srw(c.r[6], c.r[7]);
                c.stb(c.r[5].wrapping_add(0x7), c.r[7]);
                c.r[5] = c.r[5].wrapping_add(0x8);
                c.r[4] = c.r[4].wrapping_add(0x8);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cf8d4; continue; }
                pc = 0x800cf978;
            }
            0x800cf978 => {
                c.r[0] = c.r[3].wrapping_sub(c.r[4]);
                c.cmpu(0, c.r[4], c.r[3]);
                c.mtspr(9, c.r[0]);
                if !c.crbit(0) { pc = 0x800cf9a4; continue; }
                pc = 0x800cf988;
            }
            0x800cf988 => {
                c.r[0] = c.addc(!c.r[4], 0x3, 1);
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[0] = srw(c.r[6], c.r[0]);
                c.stb(c.r[5], c.r[0]);
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cf988; continue; }
                pc = 0x800cf9a4;
            }
            0x800cf9a4 => {
                c.r[0] = c.lwz(c.r[29]);
                c.r[0] = c.r[0].rotate_left(0) & 0x20000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800cf9f8; continue; }
                pc = 0x800cf9b0;
            }
            0x800cf9b0 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x44));
                c.r[0] = c.addc(!c.r[3], 0x3, 1);
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[28] = srw(c.r[28], c.r[0]);
                c.r[28] = c.r[28].rotate_left(0) & 0xf;
                c.r[0] = c.r[28].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800cf9e8; continue; }
                pc = 0x800cf9cc;
            }
            0x800cf9cc => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[31].wrapping_add(c.r[0]);
                { let ea = c.r[3].wrapping_add(0x58); c.r[0] = c.lwz(ea); c.r[3] = ea; }
                c.r[0] = c.r[0].rotate_left(0) & 0x80;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800cf9e8; continue; }
                pc = 0x800cf9e0;
            }
            0x800cf9e0 => {
                c.r[0] = 0x8;
                c.stw(c.r[3], c.r[0]);
                pc = 0x800cf9e8;
            }
            0x800cf9e8 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x800cfa1c; continue; }
                pc = 0x800cf9f0;
            }
            0x800cf9f0 => {
                c.r[28] = 0x4;
                pc = 0x800cfa1c; continue;
                pc = 0x800cf9f8;
            }
            0x800cf9f8 => {
                c.lr = 0x800cf9fc; f_800a2798(c);
                pc = 0x800cf9fc;
            }
            0x800cf9fc => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0x44));
                c.r[5] = 0x80400000;
                c.r[0] = c.r[5].wrapping_add(0xd0);
                c.r[5] = c.r[6].rotate_left(3) & 0xfffffff8;
                c.r[5] = c.r[0].wrapping_add(c.r[5]);
                c.stw(c.r[5].wrapping_add(0x4), c.r[4]);
                c.r[28] = 0x0;
                c.stw(c.r[5], c.r[3]);
                pc = 0x800cfa1c;
            }
            0x800cfa1c => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[31].wrapping_add(0x44), c.r[0]);
                pc = 0x800cfa24;
            }
            0x800cfa24 => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x800cfa40; continue;
                pc = 0x800cfa2c;
            }
            0x800cfa2c => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.r[4].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(0x6400);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x800cf864; continue;
                pc = 0x800cfa40;
            }
            0x800cfa40 => {
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
            _ => panic!("f_800cf764: bad pc {:#x}", pc),
        }
    }
}

/// SIInterruptHandler (0x800cfa60, 836 bytes)
#[inline(never)]
pub fn f_800cfa60(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cfa60) { return; }
        if c.prof { crate::prof::enter(c, 0x800cfa60); f_800cfa60_b(c, 0x800cfa60); crate::prof::exit(c, 0x800cfa60); return; }
    }
    f_800cfa60_b(c, 0x800cfa60)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cfa60_at(c: &mut Cpu, pc: u32) { f_800cfa60_b(c, pc) }
#[inline(always)]
fn f_800cfa60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cfa60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cfa60, pc); }
        match pc {
            0x800cfa60 => {
                c.r[0] = c.mfspr(8);
                c.r[5] = 0xcc000000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[7] = 0x80400000;
                c.r[6] = 0x80310000;
                { let ea = c.r[1].wrapping_add(0xffffffb8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 21..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.r[28] = c.r[7].wrapping_add(0xffffffb0);
                c.r[29] = c.r[6].wrapping_add(0x3f48);
                c.r[24] = c.lwz(c.r[5].wrapping_add(0x6434));
                c.r[5] = c.r[24].rotate_left(0) & 0xc0000000;
                c.r[0] = c.r[5].wrapping_add(0x40000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800cfc20; continue; }
                pc = 0x800cfaa0;
            }
            0x800cfaa0 => {
                c.r[23] = c.lwz(c.r[29].wrapping_add(0x44));
                c.lr = 0x800cfaa8; f_800cf764(c);
                pc = 0x800cfaa8;
            }
            0x800cfaa8 => {
                c.r[21] = c.lwz(c.r[29].wrapping_add(0x54));
                c.r[27] = 0x0;
                c.r[22] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[29].wrapping_add(0x54), c.r[27]);
                c.r[26] = c.r[23].wrapping_add(0x0);
                pc = 0x800cfabc;
            }
            0x800cfabc => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                { let v = c.r[26] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[0] = (v >> 2) as u32; }
                { let ci = c.ca(); c.r[0] = c.addc(c.r[0], 0, ci); }
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.addc(!c.r[0], c.r[26], 1);
                c.r[26] = c.r[0] | c.r[0];
                c.r[0] = c.r[0].rotate_left(5) & 0xffffffe0;
                c.r[25] = c.r[28].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[25]);
                c.cmps(0, c.r[0], 0xffffffff);
                if c.crbit(2) { pc = 0x800cfb50; continue; }
                pc = 0x800cfae8;
            }
            0x800cfae8 => {
                c.lr = 0x800cfaec; f_800a2798(c);
                pc = 0x800cfaec;
            }
            0x800cfaec => {
                c.r[0] = c.lwz(c.r[25].wrapping_add(0x18));
                c.r[5] = c.r[3] ^ 0x80000000;
                c.r[6] = c.lwz(c.r[25].wrapping_add(0x1c));
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[6], c.r[4], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[5], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[5], c.r[5], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if !c.crbit(2) { pc = 0x800cfb50; continue; }
                pc = 0x800cfb10;
            }
            0x800cfb10 => {
                c.r[3] = c.lwz(c.r[25]);
                c.r[4] = c.lwz(c.r[25].wrapping_add(0x4));
                c.r[5] = c.lwz(c.r[25].wrapping_add(0x8));
                c.r[6] = c.lwz(c.r[25].wrapping_add(0xc));
                c.r[7] = c.lwz(c.r[25].wrapping_add(0x10));
                c.r[8] = c.lwz(c.r[25].wrapping_add(0x14));
                c.lr = 0x800cfb2c; f_800d00b0(c);
                pc = 0x800cfb2c;
            }
            0x800cfb2c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800cfb5c; continue; }
                pc = 0x800cfb34;
            }
            0x800cfb34 => {
                c.r[0] = c.r[26].wrapping_mul(0x28);
                c.r[3] = c.r[28].wrapping_add(c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x80);
                c.lr = 0x800cfb44; f_8009a590(c);
                pc = 0x800cfb44;
            }
            0x800cfb44 => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[25], c.r[0]);
                pc = 0x800cfb5c; continue;
                pc = 0x800cfb50;
            }
            0x800cfb50 => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.cmps(0, c.r[27], 0x4);
                if c.crbit(0) { pc = 0x800cfabc; continue; }
                pc = 0x800cfb5c;
            }
            0x800cfb5c => {
                c.cmpu(0, c.r[21], 0x0);
                if c.crbit(2) { pc = 0x800cfb7c; continue; }
                pc = 0x800cfb64;
            }
            0x800cfb64 => {
                c.r[12] = c.r[21].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                c.r[3] = c.r[23].wrapping_add(0x0);
                c.r[4] = c.r[22].wrapping_add(0x0);
                c.r[5] = c.r[31].wrapping_add(0x0);
                if true { let t = c.lr; c.lr = 0x800cfb7c; (c.dispatch)(c, t); }
                pc = 0x800cfb7c;
            }
            0x800cfb7c => {
                c.r[3] = 0xcc000000;
                c.r[4] = c.r[3].wrapping_add(0x6400);
                c.r[5] = c.lwz(c.r[3].wrapping_add(0x6438));
                c.r[0] = c.r[23].rotate_left(3) & 0xfffffff8;
                c.r[3] = 0xf000000;
                { let v = c.r[3] as i32; let n = c.r[0] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[0] = r as u32; }
                c.r[5] = c.r[5] & c.r[0];
                c.r[0] = c.r[23].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[4].wrapping_add(0x38), c.r[5]);
                c.r[6] = c.r[29].wrapping_add(c.r[0]);
                { let ea = c.r[6].wrapping_add(0x58); c.r[0] = c.lwz(ea); c.r[6] = ea; }
                c.cmpu(0, c.r[0], 0x80);
                if !c.crbit(2) { pc = 0x800cfc20; continue; }
                pc = 0x800cfbb0;
            }
            0x800cfbb0 => {
                c.r[0] = c.r[23].rotate_left(5) & 0xffffffe0;
                c.r[0] = c.lwz(c.r[28].wrapping_add(c.r[0]));
                c.r[3] = 0x1;
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(2) { pc = 0x800cfbd4; continue; }
                pc = 0x800cfbc4;
            }
            0x800cfbc4 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x44));
                c.cmps(0, c.r[0], c.r[23]);
                if c.crbit(2) { pc = 0x800cfbd4; continue; }
                pc = 0x800cfbd0;
            }
            0x800cfbd0 => {
                c.r[3] = 0x0;
                pc = 0x800cfbd4;
            }
            0x800cfbd4 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800cfc20; continue; }
                pc = 0x800cfbdc;
            }
            0x800cfbdc => {
                c.r[3] = 0x80000000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[3] = 0x431c0000;
                c.r[4] = 0x800d0000;
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[3] = c.r[3].wrapping_add(0xffffde83);
                c.r[0] = mulhwu(c.r[3], c.r[0]);
                c.r[0] = c.r[0].rotate_left(17) & 0x1ffff;
                c.r[0] = c.r[0].wrapping_mul(0x41);
                c.r[8] = c.r[4].wrapping_add(0x860);
                c.r[10] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.r[3] = c.r[23].wrapping_add(0x0);
                c.r[4] = c.r[13].wrapping_add(0xffffa230);
                c.r[5] = 0x1;
                c.r[7] = 0x3;
                c.r[9] = 0x0;
                c.lr = 0x800cfc20; f_800d06f4(c);
                pc = 0x800cfc20;
            }
            0x800cfc20 => {
                c.r[3] = c.r[24].rotate_left(0) & 0x18000000;
                c.r[0] = c.r[3].wrapping_add(0xe8000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800cfd90; continue; }
                pc = 0x800cfc30;
            }
            0x800cfc30 => {
                c.lr = 0x800cfc34; f_800aa398(c);
                pc = 0x800cfc34;
            }
            0x800cfc34 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x48));
                c.r[23] = c.r[28].wrapping_add(0x1e0);
                c.r[24] = c.r[29].wrapping_add(0x48);
                c.r[22] = c.r[23].wrapping_add(0x0);
                c.r[21] = c.r[3].wrapping_add(0x1);
                c.r[26] = c.r[0].rotate_left(16) & 0x3ff;
                c.r[25] = 0x0;
                pc = 0x800cfc50;
            }
            0x800cfc50 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x800cfc58; f_800d04d0(c);
                pc = 0x800cfc58;
            }
            0x800cfc58 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800cfc64; continue; }
                pc = 0x800cfc60;
            }
            0x800cfc60 => {
                c.stw(c.r[22], c.r[21]);
                pc = 0x800cfc64;
            }
            0x800cfc64 => {
                c.r[25] = c.r[25].wrapping_add(0x1);
                c.cmps(0, c.r[25], 0x4);
                c.r[22] = c.r[22].wrapping_add(0x4);
                if c.crbit(0) { pc = 0x800cfc50; continue; }
                pc = 0x800cfc74;
            }
            0x800cfc74 => {
                c.r[5] = c.lwz(c.r[24]);
                c.r[3] = 0x80000000;
                c.r[0] = 0x18;
                c.r[0] = srw(c.r[3], c.r[0]);
                c.r[0] = c.r[5] & c.r[0];
                c.rc(c.r[0]);
                c.r[6] = c.r[26].rotate_left(31) & 0x7fffffff;
                if c.crbit(2) { pc = 0x800cfcb0; continue; }
                pc = 0x800cfc90;
            }
            0x800cfc90 => {
                c.r[0] = c.lwz(c.r[23]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800cfd90; continue; }
                pc = 0x800cfc9c;
            }
            0x800cfc9c => {
                c.r[0] = c.lwz(c.r[23]);
                c.r[0] = c.r[6].wrapping_add(c.r[0]);
                c.cmpu(0, c.r[0], c.r[21]);
                if !c.crbit(0) { pc = 0x800cfcb0; continue; }
                pc = 0x800cfcac;
            }
            0x800cfcac => {
                pc = 0x800cfd90; continue;
                pc = 0x800cfcb0;
            }
            0x800cfcb0 => {
                c.r[0] = 0x19;
                c.r[0] = srw(c.r[3], c.r[0]);
                c.r[0] = c.r[5] & c.r[0];
                c.rc(c.r[0]);
                c.r[4] = c.r[23].wrapping_add(0x4);
                if c.crbit(2) { pc = 0x800cfce4; continue; }
                pc = 0x800cfcc4;
            }
            0x800cfcc4 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800cfd90; continue; }
                pc = 0x800cfcd0;
            }
            0x800cfcd0 => {
                c.r[0] = c.lwz(c.r[4]);
                c.r[0] = c.r[6].wrapping_add(c.r[0]);
                c.cmpu(0, c.r[0], c.r[21]);
                if !c.crbit(0) { pc = 0x800cfce4; continue; }
                pc = 0x800cfce0;
            }
            0x800cfce0 => {
                pc = 0x800cfd90; continue;
                pc = 0x800cfce4;
            }
            0x800cfce4 => {
                c.r[0] = 0x1a;
                c.r[0] = srw(c.r[3], c.r[0]);
                c.r[0] = c.r[5] & c.r[0];
                c.rc(c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                if c.crbit(2) { pc = 0x800cfd18; continue; }
                pc = 0x800cfcf8;
            }
            0x800cfcf8 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800cfd90; continue; }
                pc = 0x800cfd04;
            }
            0x800cfd04 => {
                c.r[0] = c.lwz(c.r[4]);
                c.r[0] = c.r[6].wrapping_add(c.r[0]);
                c.cmpu(0, c.r[0], c.r[21]);
                if !c.crbit(0) { pc = 0x800cfd18; continue; }
                pc = 0x800cfd14;
            }
            0x800cfd14 => {
                pc = 0x800cfd90; continue;
                pc = 0x800cfd18;
            }
            0x800cfd18 => {
                c.r[0] = 0x1b;
                c.r[0] = srw(c.r[3], c.r[0]);
                c.r[0] = c.r[5] & c.r[0];
                c.rc(c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x4);
                if c.crbit(2) { pc = 0x800cfd4c; continue; }
                pc = 0x800cfd2c;
            }
            0x800cfd2c => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800cfd90; continue; }
                pc = 0x800cfd38;
            }
            0x800cfd38 => {
                c.r[0] = c.lwz(c.r[4]);
                c.r[0] = c.r[6].wrapping_add(c.r[0]);
                c.cmpu(0, c.r[0], c.r[21]);
                if !c.crbit(0) { pc = 0x800cfd4c; continue; }
                pc = 0x800cfd48;
            }
            0x800cfd48 => {
                pc = 0x800cfd90; continue;
                pc = 0x800cfd4c;
            }
            0x800cfd4c => {
                c.r[22] = 0x0;
                c.stw(c.r[23], c.r[22]);
                c.r[21] = c.r[28].wrapping_add(0x1a0);
                c.stw(c.r[28].wrapping_add(0x1e4), c.r[22]);
                c.stw(c.r[28].wrapping_add(0x1e8), c.r[22]);
                c.stw(c.r[28].wrapping_add(0x1ec), c.r[22]);
                pc = 0x800cfd64;
            }
            0x800cfd64 => {
                c.r[12] = c.lwz(c.r[21]);
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800cfd80; continue; }
                pc = 0x800cfd70;
            }
            0x800cfd70 => {
                c.mtspr(8, c.r[12]);
                c.r[3] = c.r[30].wrapping_add(0x0);
                c.r[4] = c.r[31].wrapping_add(0x0);
                if true { let t = c.lr; c.lr = 0x800cfd80; (c.dispatch)(c, t); }
                pc = 0x800cfd80;
            }
            0x800cfd80 => {
                c.r[22] = c.r[22].wrapping_add(0x1);
                c.cmps(0, c.r[22], 0x4);
                c.r[21] = c.r[21].wrapping_add(0x4);
                if c.crbit(0) { pc = 0x800cfd64; continue; }
                pc = 0x800cfd90;
            }
            0x800cfd90 => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 21..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x4c));
                c.r[1] = c.r[1].wrapping_add(0x48);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cfa60: bad pc {:#x}", pc),
        }
    }
}

/// SIEnablePollingInterrupt (0x800cfda4, 152 bytes)
#[inline(never)]
pub fn f_800cfda4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cfda4) { return; }
        if c.prof { crate::prof::enter(c, 0x800cfda4); f_800cfda4_b(c, 0x800cfda4); crate::prof::exit(c, 0x800cfda4); return; }
    }
    f_800cfda4_b(c, 0x800cfda4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cfda4_at(c: &mut Cpu, pc: u32) { f_800cfda4_b(c, pc) }
#[inline(always)]
fn f_800cfda4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cfda4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cfda4, pc); }
        match pc {
            0x800cfda4 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffffb0);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800cfdcc; f_8009df3c(c);
                pc = 0x800cfdcc;
            }
            0x800cfdcc => {
                c.r[4] = 0xcc000000;
                { let ea = c.r[4].wrapping_add(0x6434); c.r[5] = c.lwz(ea); c.r[4] = ea; }
                c.r[0] = c.r[5].rotate_left(0) & 0x8000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800cfde4; continue; }
                pc = 0x800cfddc;
            }
            0x800cfddc => {
                c.r[30] = 0x1;
                pc = 0x800cfde8; continue;
                pc = 0x800cfde4;
            }
            0x800cfde4 => {
                c.r[30] = 0x0;
                pc = 0x800cfde8;
            }
            0x800cfde8 => {
                c.cmps(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x800cfe0c; continue; }
                pc = 0x800cfdf0;
            }
            0x800cfdf0 => {
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x1e0), c.r[0]);
                c.r[5] = c.r[5] | 0x8000000;
                c.stw(c.r[31].wrapping_add(0x1e4), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x1e8), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x1ec), c.r[0]);
                pc = 0x800cfe10; continue;
                pc = 0x800cfe0c;
            }
            0x800cfe0c => {
                c.r[5] = c.r[5].rotate_left(0) & 0xf7ffffff;
                pc = 0x800cfe10;
            }
            0x800cfe10 => {
                c.r[5] = c.r[5].rotate_left(0) & 0x7ffffffe;
                c.stw(c.r[4], c.r[5]);
                c.lr = 0x800cfe1c; f_8009df64(c);
                pc = 0x800cfe1c;
            }
            0x800cfe1c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cfda4: bad pc {:#x}", pc),
        }
    }
}

/// SIRegisterPollingHandler (0x800cfe3c, 204 bytes)
#[inline(never)]
pub fn f_800cfe3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cfe3c) { return; }
        if c.prof { crate::prof::enter(c, 0x800cfe3c); f_800cfe3c_b(c, 0x800cfe3c); crate::prof::exit(c, 0x800cfe3c); return; }
    }
    f_800cfe3c_b(c, 0x800cfe3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cfe3c_at(c: &mut Cpu, pc: u32) { f_800cfe3c_b(c, pc) }
#[inline(always)]
fn f_800cfe3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cfe3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cfe3c, pc); }
        match pc {
            0x800cfe3c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800cfe58; f_8009df3c(c);
                pc = 0x800cfe58;
            }
            0x800cfe58 => {
                c.r[4] = 0x80400000;
                c.r[0] = 0x4;
                c.r[4] = c.r[4].wrapping_add(0x150);
                c.mtspr(9, c.r[0]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[4].wrapping_add(0x0);
                pc = 0x800cfe70;
            }
            0x800cfe70 => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[0], c.r[30]);
                if !c.crbit(2) { pc = 0x800cfe8c; continue; }
                pc = 0x800cfe7c;
            }
            0x800cfe7c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800cfe84; f_8009df64(c);
                pc = 0x800cfe84;
            }
            0x800cfe84 => {
                c.r[3] = 0x1;
                pc = 0x800cfef0; continue;
                pc = 0x800cfe8c;
            }
            0x800cfe8c => {
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cfe70; continue; }
                pc = 0x800cfe94;
            }
            0x800cfe94 => {
                c.r[0] = 0x4;
                c.mtspr(9, c.r[0]);
                c.r[5] = 0x0;
                pc = 0x800cfea0;
            }
            0x800cfea0 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800cfed8; continue; }
                pc = 0x800cfeac;
            }
            0x800cfeac => {
                c.r[3] = 0x80400000;
                c.r[4] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[3].wrapping_add(0x150);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.stw(c.r[3], c.r[30]);
                c.r[3] = 0x1;
                c.lr = 0x800cfec8; f_800cfda4(c);
                pc = 0x800cfec8;
            }
            0x800cfec8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800cfed0; f_8009df64(c);
                pc = 0x800cfed0;
            }
            0x800cfed0 => {
                c.r[3] = 0x1;
                pc = 0x800cfef0; continue;
                pc = 0x800cfed8;
            }
            0x800cfed8 => {
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cfea0; continue; }
                pc = 0x800cfee4;
            }
            0x800cfee4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800cfeec; f_8009df64(c);
                pc = 0x800cfeec;
            }
            0x800cfeec => {
                c.r[3] = 0x0;
                pc = 0x800cfef0;
            }
            0x800cfef0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cfe3c: bad pc {:#x}", pc),
        }
    }
}

/// SIUnregisterPollingHandler (0x800cff08, 244 bytes)
#[inline(never)]
pub fn f_800cff08(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cff08) { return; }
        if c.prof { crate::prof::enter(c, 0x800cff08); f_800cff08_b(c, 0x800cff08); crate::prof::exit(c, 0x800cff08); return; }
    }
    f_800cff08_b(c, 0x800cff08)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cff08_at(c: &mut Cpu, pc: u32) { f_800cff08_b(c, pc) }
#[inline(always)]
fn f_800cff08_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cff08;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cff08, pc); }
        match pc {
            0x800cff08 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800cff24; f_8009df3c(c);
                pc = 0x800cff24;
            }
            0x800cff24 => {
                c.r[4] = 0x80400000;
                c.r[0] = 0x4;
                c.r[5] = c.r[4].wrapping_add(0x150);
                c.mtspr(9, c.r[0]);
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[5].wrapping_add(0x0);
                c.r[4] = 0x0;
                pc = 0x800cff40;
            }
            0x800cff40 => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[0], c.r[30]);
                if !c.crbit(2) { pc = 0x800cffcc; continue; }
                pc = 0x800cff4c;
            }
            0x800cff4c => {
                c.r[3] = 0x80400000;
                c.r[4] = c.r[4].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[3].wrapping_add(0x150);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = 0x0;
                c.stw(c.r[3], c.r[0]);
                c.r[4] = 0x0;
                c.r[0] = c.lwz(c.r[5]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800cffac; continue; }
                pc = 0x800cff74;
            }
            0x800cff74 => {
                c.r[3] = c.r[5].wrapping_add(0x4);
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[4] = 0x1;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800cffac; continue; }
                pc = 0x800cff88;
            }
            0x800cff88 => {
                { let ea = c.r[3].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[3] = ea; }
                c.r[4] = 0x2;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800cffac; continue; }
                pc = 0x800cff98;
            }
            0x800cff98 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[4] = 0x3;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800cffac; continue; }
                pc = 0x800cffa8;
            }
            0x800cffa8 => {
                c.r[4] = 0x4;
                pc = 0x800cffac;
            }
            0x800cffac => {
                c.cmps(0, c.r[4], 0x4);
                if !c.crbit(2) { pc = 0x800cffbc; continue; }
                pc = 0x800cffb4;
            }
            0x800cffb4 => {
                c.r[3] = 0x0;
                c.lr = 0x800cffbc; f_800cfda4(c);
                pc = 0x800cffbc;
            }
            0x800cffbc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800cffc4; f_8009df64(c);
                pc = 0x800cffc4;
            }
            0x800cffc4 => {
                c.r[3] = 0x1;
                pc = 0x800cffe4; continue;
                pc = 0x800cffcc;
            }
            0x800cffcc => {
                c.r[3] = c.r[3].wrapping_add(0x4);
                c.r[4] = c.r[4].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800cff40; continue; }
                pc = 0x800cffd8;
            }
            0x800cffd8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800cffe0; f_8009df64(c);
                pc = 0x800cffe0;
            }
            0x800cffe0 => {
                c.r[3] = 0x0;
                pc = 0x800cffe4;
            }
            0x800cffe4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cff08: bad pc {:#x}", pc),
        }
    }
}

/// SIInit (0x800cfffc, 180 bytes)
#[inline(never)]
pub fn f_800cfffc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800cfffc) { return; }
        if c.prof { crate::prof::enter(c, 0x800cfffc); f_800cfffc_b(c, 0x800cfffc); crate::prof::exit(c, 0x800cfffc); return; }
    }
    f_800cfffc_b(c, 0x800cfffc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800cfffc_at(c: &mut Cpu, pc: u32) { f_800cfffc_b(c, pc) }
#[inline(always)]
fn f_800cfffc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800cfffc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800cfffc, pc); }
        match pc {
            0x800cfffc => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffffb0);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff82b8));
                c.lr = 0x800d001c; f_8009a250(c);
                pc = 0x800d001c;
            }
            0x800d001c => {
                c.r[5] = 0xffffffff;
                c.stw(c.r[31].wrapping_add(0x60), c.r[5]);
                c.r[3] = 0x80310000;
                c.r[4] = c.r[3].wrapping_add(0x3f8c);
                c.stw(c.r[31].wrapping_add(0x40), c.r[5]);
                c.r[0] = 0x0;
                c.r[3] = 0x0;
                c.stw(c.r[31].wrapping_add(0x20), c.r[5]);
                c.stw(c.r[31], c.r[5]);
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                c.lr = 0x800d0048; f_800d0f68(c);
                pc = 0x800d0048;
            }
            0x800d0048 => {
                c.r[3] = 0xcc000000;
                pc = 0x800d004c;
            }
            0x800d004c => {
                c.r[4] = c.r[3].wrapping_add(0x6400);
                { let ea = c.r[4].wrapping_add(0x34); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800d004c; continue; }
                pc = 0x800d005c;
            }
            0x800d005c => {
                c.r[0] = 0x80000000;
                c.r[3] = 0x800d0000;
                c.stw(c.r[4], c.r[0]);
                c.r[4] = c.r[3].wrapping_add(0xfffffa60);
                c.r[3] = 0x14;
                c.lr = 0x800d0074; f_8009df88(c);
                pc = 0x800d0074;
            }
            0x800d0074 => {
                c.r[3] = 0x800;
                c.lr = 0x800d007c; f_8009e38c(c);
                pc = 0x800d007c;
            }
            0x800d007c => {
                c.r[3] = 0x0;
                c.lr = 0x800d0084; f_800d0af8(c);
                pc = 0x800d0084;
            }
            0x800d0084 => {
                c.r[3] = 0x1;
                c.lr = 0x800d008c; f_800d0af8(c);
                pc = 0x800d008c;
            }
            0x800d008c => {
                c.r[3] = 0x2;
                c.lr = 0x800d0094; f_800d0af8(c);
                pc = 0x800d0094;
            }
            0x800d0094 => {
                c.r[3] = 0x3;
                c.lr = 0x800d009c; f_800d0af8(c);
                pc = 0x800d009c;
            }
            0x800d009c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x10);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800cfffc: bad pc {:#x}", pc),
        }
    }
}

/// __SITransfer (0x800d00b0, 524 bytes)
#[inline(never)]
pub fn f_800d00b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d00b0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d00b0); f_800d00b0_b(c, 0x800d00b0); crate::prof::exit(c, 0x800d00b0); return; }
    }
    f_800d00b0_b(c, 0x800d00b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d00b0_at(c: &mut Cpu, pc: u32) { f_800d00b0_b(c, pc) }
#[inline(always)]
fn f_800d00b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d00b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d00b0, pc); }
        match pc {
            0x800d00b0 => {
                c.r[0] = c.mfspr(8);
                c.r[9] = 0x80310000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffb8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3].wrapping_add(0x0);
                c.r[27] = c.r[4].wrapping_add(0x0);
                c.r[28] = c.r[5].wrapping_add(0x0);
                c.r[29] = c.r[6].wrapping_add(0x0);
                c.r[30] = c.r[7].wrapping_add(0x0);
                c.r[31] = c.r[8].wrapping_add(0x0);
                c.r[25] = c.r[9].wrapping_add(0x3f48);
                c.lr = 0x800d00e4; f_8009df3c(c);
                pc = 0x800d00e4;
            }
            0x800d00e4 => {
                c.r[0] = c.lwz(c.r[25].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0xffffffff);
                if c.crbit(2) { pc = 0x800d00fc; continue; }
                pc = 0x800d00f0;
            }
            0x800d00f0 => {
                c.lr = 0x800d00f4; f_8009df64(c);
                pc = 0x800d00f4;
            }
            0x800d00f4 => {
                c.r[3] = 0x0;
                pc = 0x800d02a8; continue;
                pc = 0x800d00fc;
            }
            0x800d00fc => {
                c.r[6] = 0xcc000000;
                c.r[0] = c.r[26].rotate_left(3) & 0xfffffff8;
                c.r[7] = c.lwz(c.r[6].wrapping_add(0x6438));
                c.r[4] = 0xf000000;
                { let v = c.r[4] as i32; let n = c.r[0] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[0] = r as u32; }
                c.r[7] = c.r[7] & c.r[0];
                c.stw(c.r[6].wrapping_add(0x6438), c.r[7]);
                c.r[0] = c.r[28].wrapping_add(0x3);
                c.r[4] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.stw(c.r[25].wrapping_add(0x44), c.r[26]);
                c.cmpu(0, c.r[4], 0x0);
                c.r[5] = 0x0;
                c.stw(c.r[25].wrapping_add(0x54), c.r[31]);
                c.stw(c.r[25].wrapping_add(0x4c), c.r[30]);
                c.stw(c.r[25].wrapping_add(0x50), c.r[29]);
                if !c.crbit(1) { pc = 0x800d01dc; continue; }
                pc = 0x800d013c;
            }
            0x800d013c => {
                c.cmpu(0, c.r[4], 0x8);
                c.r[7] = c.r[4].wrapping_add(0xfffffff8);
                if !c.crbit(1) { pc = 0x800d0290; continue; }
                pc = 0x800d0148;
            }
            0x800d0148 => {
                c.r[0] = c.r[7].wrapping_add(0x7);
                c.r[0] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.cmpu(0, c.r[7], 0x0);
                c.mtspr(9, c.r[0]);
                c.r[7] = c.r[27].wrapping_add(0x0);
                c.r[6] = c.r[6].wrapping_add(0x6400);
                if !c.crbit(1) { pc = 0x800d0290; continue; }
                pc = 0x800d0164;
            }
            0x800d0164 => {
                c.r[0] = c.lwz(c.r[7]);
                c.r[5] = c.r[5].wrapping_add(0x8);
                c.stw(c.r[6].wrapping_add(0x80), c.r[0]);
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x4));
                c.stw(c.r[6].wrapping_add(0x84), c.r[0]);
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x8));
                c.stw(c.r[6].wrapping_add(0x88), c.r[0]);
                c.r[0] = c.lwz(c.r[7].wrapping_add(0xc));
                c.stw(c.r[6].wrapping_add(0x8c), c.r[0]);
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x10));
                c.stw(c.r[6].wrapping_add(0x90), c.r[0]);
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x14));
                c.stw(c.r[6].wrapping_add(0x94), c.r[0]);
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x18));
                c.stw(c.r[6].wrapping_add(0x98), c.r[0]);
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x1c));
                c.r[7] = c.r[7].wrapping_add(0x20);
                c.stw(c.r[6].wrapping_add(0x9c), c.r[0]);
                c.r[6] = c.r[6].wrapping_add(0x20);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800d0164; continue; }
                pc = 0x800d01b4;
            }
            0x800d01b4 => {
                pc = 0x800d0290; continue;
                pc = 0x800d01b8;
            }
            0x800d01b8 => {
                c.r[0] = c.r[4].wrapping_sub(c.r[5]);
                c.cmpu(0, c.r[5], c.r[4]);
                c.mtspr(9, c.r[0]);
                if !c.crbit(0) { pc = 0x800d01dc; continue; }
                pc = 0x800d01c8;
            }
            0x800d01c8 => {
                c.r[0] = c.lwz(c.r[6]);
                c.r[6] = c.r[6].wrapping_add(0x4);
                c.stw(c.r[7].wrapping_add(0x80), c.r[0]);
                c.r[7] = c.r[7].wrapping_add(0x4);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800d01c8; continue; }
                pc = 0x800d01dc;
            }
            0x800d01dc => {
                c.r[4] = 0xcc000000;
                c.r[6] = c.r[4].wrapping_add(0x6400);
                { let ea = c.r[6].wrapping_add(0x34); c.r[0] = c.lwz(ea); c.r[6] = ea; }
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.cmpu(0, c.r[31], 0x0);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x24));
                c.r[0] = (c.r[4].rotate_left(7) & 0x80) | (c.r[0] & 0xffffff7f);
                c.stb(c.r[1].wrapping_add(0x24), c.r[0]);
                if c.crbit(2) { pc = 0x800d0208; continue; }
                pc = 0x800d0204;
            }
            0x800d0204 => {
                pc = 0x800d020c; continue;
                pc = 0x800d0208;
            }
            0x800d0208 => {
                c.r[4] = 0x0;
                pc = 0x800d020c;
            }
            0x800d020c => {
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x24));
                c.r[0] = (c.r[4].rotate_left(6) & 0x40) | (c.r[0] & 0xffffffbf);
                c.cmpu(0, c.r[28], 0x80);
                c.stb(c.r[1].wrapping_add(0x24), c.r[0]);
                if !c.crbit(2) { pc = 0x800d0228; continue; }
                pc = 0x800d0220;
            }
            0x800d0220 => {
                c.r[0] = 0x0;
                pc = 0x800d022c; continue;
                pc = 0x800d0228;
            }
            0x800d0228 => {
                c.r[0] = c.r[28] | c.r[28];
                pc = 0x800d022c;
            }
            0x800d022c => {
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x25));
                c.r[0] = (c.r[4].rotate_left(0) & 0x7f) | (c.r[0] & 0xffffff80);
                c.cmpu(0, c.r[30], 0x80);
                c.stb(c.r[1].wrapping_add(0x25), c.r[0]);
                if !c.crbit(2) { pc = 0x800d024c; continue; }
                pc = 0x800d0244;
            }
            0x800d0244 => {
                c.r[0] = 0x0;
                pc = 0x800d0250; continue;
                pc = 0x800d024c;
            }
            0x800d024c => {
                c.r[0] = c.r[30] | c.r[30];
                pc = 0x800d0250;
            }
            0x800d0250 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x26));
                c.r[0] = (c.r[4].rotate_left(0) & 0x7f) | (c.r[0] & 0xffffff80);
                c.stb(c.r[1].wrapping_add(0x26), c.r[0]);
                c.r[4] = 0x1;
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x27));
                c.r[0] = (c.r[26].rotate_left(1) & 0x6) | (c.r[0] & 0xfffffff9);
                c.stb(c.r[1].wrapping_add(0x27), c.r[0]);
                c.r[0] = c.lbz(c.r[1].wrapping_add(0x27));
                c.r[0] = (c.r[4].rotate_left(0) & 0x1) | (c.r[0] & 0xfffffffe);
                c.stb(c.r[1].wrapping_add(0x27), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.stw(c.r[6], c.r[0]);
                c.lr = 0x800d0288; f_8009df64(c);
                pc = 0x800d0288;
            }
            0x800d0288 => {
                c.r[3] = 0x1;
                pc = 0x800d02a8; continue;
                pc = 0x800d0290;
            }
            0x800d0290 => {
                c.r[6] = 0xcc000000;
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[7] = c.r[6].wrapping_add(0x6400);
                c.r[6] = c.r[27].wrapping_add(c.r[0]);
                c.r[7] = c.r[7].wrapping_add(c.r[0]);
                pc = 0x800d01b8; continue;
                pc = 0x800d02a8;
            }
            0x800d02a8 => {
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x4c));
                c.r[1] = c.r[1].wrapping_add(0x48);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d00b0: bad pc {:#x}", pc),
        }
    }
}

/// SIGetStatus (0x800d02bc, 124 bytes)
#[inline(never)]
pub fn f_800d02bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d02bc) { return; }
        if c.prof { crate::prof::enter(c, 0x800d02bc); f_800d02bc_b(c, 0x800d02bc); crate::prof::exit(c, 0x800d02bc); return; }
    }
    f_800d02bc_b(c, 0x800d02bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d02bc_at(c: &mut Cpu, pc: u32) { f_800d02bc_b(c, pc) }
#[inline(always)]
fn f_800d02bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d02bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d02bc, pc); }
        match pc {
            0x800d02bc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800d02d8; f_8009df3c(c);
                pc = 0x800d02d8;
            }
            0x800d02d8 => {
                c.r[4] = 0xcc000000;
                c.r[0] = c.addc(!c.r[30], 0x3, 1);
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x6438));
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[31] = srw(c.r[31], c.r[0]);
                c.r[0] = c.r[31].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0318; continue; }
                pc = 0x800d02f4;
            }
            0x800d02f4 => {
                c.r[4] = 0x80310000;
                c.r[5] = c.r[30].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x3fa0);
                c.r[4] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = c.lwz(c.r[4]);
                c.r[0] = c.r[0].rotate_left(0) & 0x80;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800d0318; continue; }
                pc = 0x800d0310;
            }
            0x800d0310 => {
                c.r[0] = 0x8;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800d0318;
            }
            0x800d0318 => {
                c.lr = 0x800d031c; f_8009df64(c);
                pc = 0x800d031c;
            }
            0x800d031c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x10));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d02bc: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D0338 (0x800d0338, 20 bytes)
#[inline(never)]
pub fn f_800d0338(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d0338) { return; }
        if c.prof { crate::prof::enter(c, 0x800d0338); f_800d0338_b(c, 0x800d0338); crate::prof::exit(c, 0x800d0338); return; }
    }
    f_800d0338_b(c, 0x800d0338)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d0338_at(c: &mut Cpu, pc: u32) { f_800d0338_b(c, pc) }
#[inline(always)]
fn f_800d0338_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d0338;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d0338, pc); }
        match pc {
            0x800d0338 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = 0xcc000000;
                c.r[3] = c.r[3].wrapping_add(0x6400);
                c.stw(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d0338: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D034C (0x800d034c, 16 bytes)
#[inline(never)]
pub fn f_800d034c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d034c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d034c); f_800d034c_b(c, 0x800d034c); crate::prof::exit(c, 0x800d034c); return; }
    }
    f_800d034c_b(c, 0x800d034c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d034c_at(c: &mut Cpu, pc: u32) { f_800d034c_b(c, pc) }
#[inline(always)]
fn f_800d034c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d034c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d034c, pc); }
        match pc {
            0x800d034c => {
                c.r[3] = 0xcc000000;
                c.r[0] = 0x80000000;
                c.stw(c.r[3].wrapping_add(0x6438), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d034c: bad pc {:#x}", pc),
        }
    }
}

/// SISetXY (0x800d035c, 108 bytes)
#[inline(never)]
pub fn f_800d035c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d035c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d035c); f_800d035c_b(c, 0x800d035c); crate::prof::exit(c, 0x800d035c); return; }
    }
    f_800d035c_b(c, 0x800d035c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d035c_at(c: &mut Cpu, pc: u32) { f_800d035c_b(c, pc) }
#[inline(always)]
fn f_800d035c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d035c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d035c, pc); }
        match pc {
            0x800d035c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[4].rotate_left(8) & 0xffffff00;
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3].rotate_left(16) & 0xffff0000;
                c.r[31] = c.r[31] | c.r[0];
                c.lr = 0x800d037c; f_8009df3c(c);
                pc = 0x800d037c;
            }
            0x800d037c => {
                c.r[4] = 0x80310000;
                c.r[4] = c.r[4].wrapping_add(0x3f8c);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[5] = c.r[4].wrapping_add(0x4);
                c.r[4] = 0xcc000000;
                c.r[0] = c.r[0].rotate_left(0) & 0xfc0000ff;
                c.stw(c.r[5], c.r[0]);
                c.r[0] = c.lwz(c.r[5]);
                c.r[0] = c.r[0] | c.r[31];
                c.stw(c.r[5], c.r[0]);
                c.r[31] = c.lwz(c.r[5]);
                c.stw(c.r[4].wrapping_add(0x6430), c.r[31]);
                c.lr = 0x800d03b0; f_8009df64(c);
                pc = 0x800d03b0;
            }
            0x800d03b0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d035c: bad pc {:#x}", pc),
        }
    }
}

/// SIEnablePolling (0x800d03c8, 156 bytes)
#[inline(never)]
pub fn f_800d03c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d03c8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d03c8); f_800d03c8_b(c, 0x800d03c8); crate::prof::exit(c, 0x800d03c8); return; }
    }
    f_800d03c8_b(c, 0x800d03c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d03c8_at(c: &mut Cpu, pc: u32) { f_800d03c8_b(c, pc) }
#[inline(always)]
fn f_800d03c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d03c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d03c8, pc); }
        match pc {
            0x800d03c8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if !c.crbit(2) { pc = 0x800d03f0; continue; }
                pc = 0x800d03e0;
            }
            0x800d03e0 => {
                c.r[3] = 0x80310000;
                c.r[3] = c.r[3].wrapping_add(0x3f8c);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x800d0450; continue;
                pc = 0x800d03f0;
            }
            0x800d03f0 => {
                c.lr = 0x800d03f4; f_8009df3c(c);
                pc = 0x800d03f4;
            }
            0x800d03f4 => {
                c.r[4] = 0x80310000;
                c.r[4] = c.r[4].wrapping_add(0x3f8c);
                c.r[31] = c.r[31].rotate_left(8) & 0xff;
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[5] = c.r[4].wrapping_add(0x4);
                c.r[4] = c.r[31].rotate_left(28) & 0xf;
                c.r[0] = c.r[0] & !c.r[4];
                c.stw(c.r[5], c.r[0]);
                c.r[0] = c.r[4] | 0x3ff0000;
                c.r[0] = c.r[0] | 0xfff0;
                c.r[31] = c.r[31] & c.r[0];
                c.r[0] = c.lwz(c.r[5]);
                c.r[31] = c.r[31].rotate_left(0) & 0xfc0000ff;
                c.r[0] = c.r[0] | c.r[31];
                c.stw(c.r[5], c.r[0]);
                c.r[4] = 0xcc000000;
                c.r[4] = c.r[4].wrapping_add(0x6400);
                c.r[31] = c.lwz(c.r[5]);
                c.r[0] = 0x80000000;
                c.stw(c.r[4].wrapping_add(0x38), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x30), c.r[31]);
                c.lr = 0x800d044c; f_8009df64(c);
                pc = 0x800d044c;
            }
            0x800d044c => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d0450;
            }
            0x800d0450 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d03c8: bad pc {:#x}", pc),
        }
    }
}

/// SIDisablePolling (0x800d0464, 108 bytes)
#[inline(never)]
pub fn f_800d0464(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d0464) { return; }
        if c.prof { crate::prof::enter(c, 0x800d0464); f_800d0464_b(c, 0x800d0464); crate::prof::exit(c, 0x800d0464); return; }
    }
    f_800d0464_b(c, 0x800d0464)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d0464_at(c: &mut Cpu, pc: u32) { f_800d0464_b(c, pc) }
#[inline(always)]
fn f_800d0464_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d0464;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d0464, pc); }
        match pc {
            0x800d0464 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if !c.crbit(2) { pc = 0x800d048c; continue; }
                pc = 0x800d047c;
            }
            0x800d047c => {
                c.r[3] = 0x80310000;
                c.r[3] = c.r[3].wrapping_add(0x3f8c);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x800d04bc; continue;
                pc = 0x800d048c;
            }
            0x800d048c => {
                c.lr = 0x800d0490; f_8009df3c(c);
                pc = 0x800d0490;
            }
            0x800d0490 => {
                c.r[4] = 0x80310000;
                c.r[4] = c.r[4].wrapping_add(0x3f8c);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[31] = c.r[31].rotate_left(8) & 0xf0;
                c.r[5] = c.r[4].wrapping_add(0x4);
                c.r[4] = 0xcc000000;
                c.r[31] = c.r[0] & !c.r[31];
                c.stw(c.r[4].wrapping_add(0x6430), c.r[31]);
                c.stw(c.r[5], c.r[31]);
                c.lr = 0x800d04b8; f_8009df64(c);
                pc = 0x800d04b8;
            }
            0x800d04b8 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d04bc;
            }
            0x800d04bc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x18);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d0464: bad pc {:#x}", pc),
        }
    }
}

/// SIGetResponseRaw (0x800d04d0, 212 bytes)
#[inline(never)]
pub fn f_800d04d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d04d0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d04d0); f_800d04d0_b(c, 0x800d04d0); crate::prof::exit(c, 0x800d04d0); return; }
    }
    f_800d04d0_b(c, 0x800d04d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d04d0_at(c: &mut Cpu, pc: u32) { f_800d04d0_b(c, pc) }
#[inline(always)]
fn f_800d04d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d04d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d04d0, pc); }
        match pc {
            0x800d04d0 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x24), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0xffffffb0);
                c.stw(c.r[1].wrapping_add(0x20), c.r[30]);
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[29]);
                c.lr = 0x800d04f8; f_8009df3c(c);
                pc = 0x800d04f8;
            }
            0x800d04f8 => {
                c.r[4] = 0xcc000000;
                c.r[0] = c.addc(!c.r[30], 0x3, 1);
                c.r[29] = c.lwz(c.r[4].wrapping_add(0x6438));
                c.r[0] = c.r[0].rotate_left(3) & 0xfffffff8;
                c.r[29] = srw(c.r[29], c.r[0]);
                c.r[0] = c.r[29].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0538; continue; }
                pc = 0x800d0514;
            }
            0x800d0514 => {
                c.r[4] = 0x80310000;
                c.r[5] = c.r[30].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x3fa0);
                c.r[4] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = c.lwz(c.r[4]);
                c.r[0] = c.r[0].rotate_left(0) & 0x80;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800d0538; continue; }
                pc = 0x800d0530;
            }
            0x800d0530 => {
                c.r[0] = 0x8;
                c.stw(c.r[4], c.r[0]);
                pc = 0x800d0538;
            }
            0x800d0538 => {
                c.lr = 0x800d053c; f_8009df64(c);
                pc = 0x800d053c;
            }
            0x800d053c => {
                c.r[0] = c.r[29].rotate_left(0) & 0x20;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0584; continue; }
                pc = 0x800d0544;
            }
            0x800d0544 => {
                c.r[4] = c.r[30].wrapping_mul(0xc);
                c.r[3] = 0xcc000000;
                c.r[0] = c.r[3].wrapping_add(0x6400);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                c.r[4] = c.r[30].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[6] = c.r[31].wrapping_add(c.r[4]);
                c.stw(c.r[6].wrapping_add(0x1c0), c.r[0]);
                c.r[0] = c.r[30].rotate_left(2) & 0xfffffffc;
                c.r[4] = c.r[31].wrapping_add(c.r[0]);
                c.r[5] = c.lwz(c.r[3].wrapping_add(0x8));
                c.r[0] = 0x1;
                c.r[3] = 0x1;
                c.stw(c.r[6].wrapping_add(0x1c4), c.r[5]);
                c.stw(c.r[4].wrapping_add(0x1b0), c.r[0]);
                pc = 0x800d0588; continue;
                pc = 0x800d0584;
            }
            0x800d0584 => {
                c.r[3] = 0x0;
                pc = 0x800d0588;
            }
            0x800d0588 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x20));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d04d0: bad pc {:#x}", pc),
        }
    }
}

/// SIGetResponse (0x800d05a4, 196 bytes)
#[inline(never)]
pub fn f_800d05a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d05a4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d05a4); f_800d05a4_b(c, 0x800d05a4); crate::prof::exit(c, 0x800d05a4); return; }
    }
    f_800d05a4_b(c, 0x800d05a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d05a4_at(c: &mut Cpu, pc: u32) { f_800d05a4_b(c, pc) }
#[inline(always)]
fn f_800d05a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d05a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d05a4, pc); }
        match pc {
            0x800d05a4 => {
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[30] = c.r[4].wrapping_add(0x0);
                c.r[28] = c.r[5].wrapping_add(0xffffffb0);
                c.lr = 0x800d05c8; f_8009df3c(c);
                pc = 0x800d05c8;
            }
            0x800d05c8 => {
                c.r[31] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[29].wrapping_add(0x0);
                c.lr = 0x800d05d4; f_800d02bc(c);
                pc = 0x800d05d4;
            }
            0x800d05d4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x20;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0614; continue; }
                pc = 0x800d05dc;
            }
            0x800d05dc => {
                c.r[4] = c.r[29].wrapping_mul(0xc);
                c.r[3] = 0xcc000000;
                c.r[0] = c.r[3].wrapping_add(0x6400);
                c.r[4] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.r[29].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[5] = c.r[28].wrapping_add(c.r[3]);
                c.stw(c.r[5].wrapping_add(0x1c0), c.r[0]);
                c.r[0] = c.r[29].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[28].wrapping_add(c.r[0]);
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x8));
                c.r[0] = 0x1;
                c.stw(c.r[5].wrapping_add(0x1c4), c.r[4]);
                c.stw(c.r[3].wrapping_add(0x1b0), c.r[0]);
                pc = 0x800d0614;
            }
            0x800d0614 => {
                c.r[0] = c.r[29].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[28].wrapping_add(c.r[0]);
                c.r[27] = c.lwz(c.r[3].wrapping_add(0x1b0));
                c.r[0] = 0x0;
                c.cmps(0, c.r[27], 0x0);
                c.stw(c.r[3].wrapping_add(0x1b0), c.r[0]);
                if c.crbit(2) { pc = 0x800d0648; continue; }
                pc = 0x800d0630;
            }
            0x800d0630 => {
                c.r[0] = c.r[29].rotate_left(3) & 0xfffffff8;
                c.r[3] = c.r[28].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1c0));
                c.stw(c.r[30], c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1c4));
                c.stw(c.r[30].wrapping_add(0x4), c.r[0]);
                pc = 0x800d0648;
            }
            0x800d0648 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800d0650; f_8009df64(c);
                pc = 0x800d0650;
            }
            0x800d0650 => {
                c.r[3] = c.r[27] | c.r[27];
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d05a4: bad pc {:#x}", pc),
        }
    }
}

/// AlarmHandler_800D0668 (0x800d0668, 140 bytes)
#[inline(never)]
pub fn f_800d0668(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d0668) { return; }
        if c.prof { crate::prof::enter(c, 0x800d0668); f_800d0668_b(c, 0x800d0668); crate::prof::exit(c, 0x800d0668); return; }
    }
    f_800d0668_b(c, 0x800d0668)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d0668_at(c: &mut Cpu, pc: u32) { f_800d0668_b(c, pc) }
#[inline(always)]
fn f_800d0668_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d0668;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d0668, pc); }
        match pc {
            0x800d0668 => {
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.r[4].wrapping_add(0x30);
                c.r[4] = 0x66660000;
                c.r[0] = c.r[3].wrapping_sub(c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[3] = c.r[4].wrapping_add(0x6667);
                c.r[0] = mulhw(c.r[3], c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 28 != 0); c.r[0] = (v >> 4) as u32; }
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[3] = 0x80400000;
                c.r[4] = c.r[0].rotate_left(5) & 0xffffffe0;
                c.r[0] = c.r[3].wrapping_add(0xffffffb0);
                c.r[31] = c.r[0].wrapping_add(c.r[4]);
                c.r[3] = c.lwz(c.r[31]);
                c.cmps(0, c.r[3], 0xffffffff);
                if c.crbit(2) { pc = 0x800d06e0; continue; }
                pc = 0x800d06b8;
            }
            0x800d06b8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[6] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[7] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[8] = c.lwz(c.r[31].wrapping_add(0x14));
                c.lr = 0x800d06d0; f_800d00b0(c);
                pc = 0x800d06d0;
            }
            0x800d06d0 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d06e0; continue; }
                pc = 0x800d06d8;
            }
            0x800d06d8 => {
                c.r[0] = 0xffffffff;
                c.stw(c.r[31], c.r[0]);
                pc = 0x800d06e0;
            }
            0x800d06e0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d0668: bad pc {:#x}", pc),
        }
    }
}

/// SITransfer (0x800d06f4, 364 bytes)
#[inline(never)]
pub fn f_800d06f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d06f4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d06f4); f_800d06f4_b(c, 0x800d06f4); crate::prof::exit(c, 0x800d06f4); return; }
    }
    f_800d06f4_b(c, 0x800d06f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d06f4_at(c: &mut Cpu, pc: u32) { f_800d06f4_b(c, pc) }
#[inline(always)]
fn f_800d06f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d06f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d06f4, pc); }
        match pc {
            0x800d06f4 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 19..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[24] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x80400000;
                c.r[23] = c.r[3].wrapping_add(0xffffffb0);
                c.r[0] = c.r[24].rotate_left(5) & 0xffffffe0;
                c.r[25] = c.r[4].wrapping_add(0x0);
                c.r[26] = c.r[5].wrapping_add(0x0);
                c.r[27] = c.r[6].wrapping_add(0x0);
                c.r[28] = c.r[7].wrapping_add(0x0);
                c.r[29] = c.r[8].wrapping_add(0x0);
                c.r[31] = c.r[9].wrapping_add(0x0);
                c.r[30] = c.r[10].wrapping_add(0x0);
                c.r[22] = c.r[23].wrapping_add(c.r[0]);
                c.lr = 0x800d0738; f_8009df3c(c);
                pc = 0x800d0738;
            }
            0x800d0738 => {
                c.r[0] = c.lwz(c.r[22]);
                c.r[21] = c.r[3].wrapping_add(0x0);
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(2) { pc = 0x800d0758; continue; }
                pc = 0x800d0748;
            }
            0x800d0748 => {
                c.r[3] = 0x80310000;
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x3f8c));
                c.cmps(0, c.r[0], c.r[24]);
                if !c.crbit(2) { pc = 0x800d0768; continue; }
                pc = 0x800d0758;
            }
            0x800d0758 => {
                c.r[3] = c.r[21] | c.r[21];
                c.lr = 0x800d0760; f_8009df64(c);
                pc = 0x800d0760;
            }
            0x800d0760 => {
                c.r[3] = 0x0;
                pc = 0x800d084c; continue;
                pc = 0x800d0768;
            }
            0x800d0768 => {
                c.lr = 0x800d076c; f_800a2798(c);
                pc = 0x800d076c;
            }
            0x800d076c => {
                c.r[0] = 0x0;
                c.r[5] = c.r[30] ^ c.r[0];
                c.r[0] = c.r[31] ^ c.r[0];
                c.r[0] = c.r[5] | c.r[0];
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800d078c; continue; }
                pc = 0x800d0780;
            }
            0x800d0780 => {
                c.r[19] = c.r[4].wrapping_add(0x0);
                c.r[20] = c.r[3].wrapping_add(0x0);
                pc = 0x800d07a4; continue;
                pc = 0x800d078c;
            }
            0x800d078c => {
                c.r[0] = c.r[24].rotate_left(3) & 0xfffffff8;
                c.r[6] = c.r[23].wrapping_add(c.r[0]);
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x144));
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x140));
                c.r[19] = c.addc(c.r[30], c.r[5], 0);
                { let ci = c.ca(); c.r[20] = c.addc(c.r[31], c.r[0], ci); }
                pc = 0x800d07a4;
            }
            0x800d07a4 => {
                c.r[6] = c.r[3] ^ 0x80000000;
                c.r[5] = c.r[20] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[19], c.r[4], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[6], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x800d07ec; continue; }
                pc = 0x800d07c0;
            }
            0x800d07c0 => {
                c.r[0] = c.r[24].wrapping_mul(0x28);
                c.r[30] = c.addc(!c.r[4], c.r[19], 1);
                { let ci = c.ca(); c.r[31] = c.addc(!c.r[3], c.r[20], ci); }
                c.r[4] = 0x800d0000;
                c.r[3] = c.r[23].wrapping_add(c.r[0]);
                c.r[7] = c.r[4].wrapping_add(0x668);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = c.r[31].wrapping_add(0x0);
                c.r[3] = c.r[3].wrapping_add(0x80);
                c.lr = 0x800d07e8; f_8009a528(c);
                pc = 0x800d07e8;
            }
            0x800d07e8 => {
                pc = 0x800d0820; continue;
                pc = 0x800d07ec;
            }
            0x800d07ec => {
                c.r[3] = c.r[24].wrapping_add(0x0);
                c.r[4] = c.r[25].wrapping_add(0x0);
                c.r[5] = c.r[26].wrapping_add(0x0);
                c.r[6] = c.r[27].wrapping_add(0x0);
                c.r[7] = c.r[28].wrapping_add(0x0);
                c.r[8] = c.r[29].wrapping_add(0x0);
                c.lr = 0x800d0808; f_800d00b0(c);
                pc = 0x800d0808;
            }
            0x800d0808 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d0820; continue; }
                pc = 0x800d0810;
            }
            0x800d0810 => {
                c.r[3] = c.r[21] | c.r[21];
                c.lr = 0x800d0818; f_8009df64(c);
                pc = 0x800d0818;
            }
            0x800d0818 => {
                c.r[3] = 0x1;
                pc = 0x800d084c; continue;
                pc = 0x800d0820;
            }
            0x800d0820 => {
                c.stw(c.r[22], c.r[24]);
                c.r[3] = c.r[21] | c.r[21];
                c.stw(c.r[22].wrapping_add(0x4), c.r[25]);
                c.stw(c.r[22].wrapping_add(0x8), c.r[26]);
                c.stw(c.r[22].wrapping_add(0xc), c.r[27]);
                c.stw(c.r[22].wrapping_add(0x10), c.r[28]);
                c.stw(c.r[22].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[22].wrapping_add(0x1c), c.r[19]);
                c.stw(c.r[22].wrapping_add(0x18), c.r[20]);
                c.lr = 0x800d0848; f_8009df64(c);
                pc = 0x800d0848;
            }
            0x800d0848 => {
                c.r[3] = 0x1;
                pc = 0x800d084c;
            }
            0x800d084c => {
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 19..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                c.r[1] = c.r[1].wrapping_add(0x60);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d06f4: bad pc {:#x}", pc),
        }
    }
}

/// GetTypeCallback (0x800d0860, 664 bytes)
#[inline(never)]
pub fn f_800d0860(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d0860) { return; }
        if c.prof { crate::prof::enter(c, 0x800d0860); f_800d0860_b(c, 0x800d0860); crate::prof::exit(c, 0x800d0860); return; }
    }
    f_800d0860_b(c, 0x800d0860)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d0860_at(c: &mut Cpu, pc: u32) { f_800d0860_b(c, pc) }
#[inline(always)]
fn f_800d0860_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d0860;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d0860, pc); }
        match pc {
            0x800d0860 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3].wrapping_add(0x0);
                c.r[3] = 0x80310000;
                c.r[0] = c.r[3].wrapping_add(0x3fa0);
                c.r[29] = c.r[27].rotate_left(2) & 0xfffffffc;
                c.r[30] = c.r[0].wrapping_add(c.r[29]);
                c.r[3] = 0x80400000;
                c.r[26] = c.r[4].wrapping_add(0x0);
                c.r[31] = c.r[3].wrapping_add(0xffffffb0);
                c.r[0] = c.lwz(c.r[30]);
                c.r[0] = c.r[0].rotate_left(0) & 0xffffff7f;
                c.stw(c.r[30], c.r[0]);
                c.r[0] = c.lwz(c.r[30]);
                c.r[0] = c.r[0] | c.r[26];
                c.stw(c.r[30], c.r[0]);
                c.lr = 0x800d08ac; f_800a2798(c);
                pc = 0x800d08ac;
            }
            0x800d08ac => {
                c.r[0] = c.r[27].rotate_left(3) & 0xfffffff8;
                c.r[5] = c.r[31].wrapping_add(c.r[0]);
                c.stw(c.r[5].wrapping_add(0x124), c.r[4]);
                c.r[4] = 0x80000000;
                c.r[0] = c.r[26].rotate_left(0) & 0xf;
                c.rc(c.r[0]);
                c.stw(c.r[5].wrapping_add(0x120), c.r[3]);
                c.r[4] = srw(c.r[4], c.r[27]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa238));
                c.r[28] = c.lwz(c.r[30]);
                c.r[0] = c.r[3] & !c.r[4];
                c.stw(c.r[13].wrapping_add(0xffffa238), c.r[0]);
                c.r[26] = c.r[3] & c.r[4];
                if !c.crbit(2) { pc = 0x800d0900; continue; }
                pc = 0x800d08e0;
            }
            0x800d08e0 => {
                c.r[3] = c.r[28].rotate_left(0) & 0x18000000;
                c.r[0] = c.r[3].wrapping_add(0xf8000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d0900; continue; }
                pc = 0x800d08f0;
            }
            0x800d08f0 => {
                c.r[0] = c.r[28].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0900; continue; }
                pc = 0x800d08f8;
            }
            0x800d08f8 => {
                c.r[0] = c.r[28].rotate_left(0) & 0x4000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0958; continue; }
                pc = 0x800d0900;
            }
            0x800d0900 => {
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.lr = 0x800d090c; f_800a115c(c);
                pc = 0x800d090c;
            }
            0x800d090c => {
                c.r[0] = c.r[27].rotate_left(4) & 0xfffffff0;
                c.r[30] = c.lwz(c.r[30]);
                c.r[31] = c.r[31].wrapping_add(c.r[0]);
                c.r[29] = 0x0;
                c.r[28] = c.r[29].wrapping_add(0x0);
                c.r[31] = c.r[31].wrapping_add(0x160);
                pc = 0x800d0924;
            }
            0x800d0924 => {
                c.r[12] = c.lwz(c.r[31]);
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800d0944; continue; }
                pc = 0x800d0930;
            }
            0x800d0930 => {
                c.stw(c.r[31], c.r[28]);
                c.mtspr(8, c.r[12]);
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[4] = c.r[30].wrapping_add(0x0);
                if true { let t = c.lr; c.lr = 0x800d0944; (c.dispatch)(c, t); }
                pc = 0x800d0944;
            }
            0x800d0944 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.cmps(0, c.r[29], 0x4);
                c.r[31] = c.r[31].wrapping_add(0x4);
                if c.crbit(0) { pc = 0x800d0924; continue; }
                pc = 0x800d0954;
            }
            0x800d0954 => {
                pc = 0x800d0ae4; continue;
                pc = 0x800d0958;
            }
            0x800d0958 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x800d0960; f_800a10d8(c);
                pc = 0x800d0960;
            }
            0x800d0960 => {
                c.cmps(0, c.r[26], 0x0);
                c.r[26] = c.r[3].rotate_left(8) & 0xffff00;
                if c.crbit(2) { pc = 0x800d09bc; continue; }
                pc = 0x800d096c;
            }
            0x800d096c => {
                c.r[0] = c.r[26].rotate_left(0) & 0x100000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d09bc; continue; }
                pc = 0x800d0974;
            }
            0x800d0974 => {
                c.r[3] = 0xd00000;
                c.r[0] = c.r[3].wrapping_add(0xffffff00);
                c.r[0] = c.r[26] & c.r[0];
                c.r[0] = c.r[0] | 0x4e100000;
                c.r[4] = c.r[31].wrapping_add(c.r[29]);
                { let ea = c.r[4].wrapping_add(0x1f0); c.stw(ea, c.r[0]); c.r[4] = ea; }
                c.r[0] = 0x80;
                c.r[3] = 0x800d0000;
                c.stw(c.r[30], c.r[0]);
                c.r[8] = c.r[3].wrapping_add(0x860);
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x3;
                c.r[7] = 0x3;
                c.r[10] = 0x0;
                c.r[9] = 0x0;
                c.lr = 0x800d09b8; f_800d06f4(c);
                pc = 0x800d09b8;
            }
            0x800d09b8 => {
                pc = 0x800d0ae4; continue;
                pc = 0x800d09bc;
            }
            0x800d09bc => {
                c.r[0] = c.r[28].rotate_left(0) & 0x100000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0a30; continue; }
                pc = 0x800d09c4;
            }
            0x800d09c4 => {
                c.r[3] = 0xd00000;
                c.r[3] = c.r[3].wrapping_add(0xffffff00);
                c.r[0] = c.r[26] & c.r[3];
                c.r[3] = c.r[28] & c.r[3];
                c.cmpu(0, c.r[0], c.r[3]);
                if c.crbit(2) { pc = 0x800d0a9c; continue; }
                pc = 0x800d09dc;
            }
            0x800d09dc => {
                c.r[0] = c.r[26].rotate_left(0) & 0x100000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800d09f4; continue; }
                pc = 0x800d09e4;
            }
            0x800d09e4 => {
                c.r[26] = c.r[3] | 0x100000;
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[4] = c.r[26].rotate_left(24) & 0xffff;
                c.lr = 0x800d09f4; f_800a115c(c);
                pc = 0x800d09f4;
            }
            0x800d09f4 => {
                c.r[0] = c.r[26] | 0x4e000000;
                c.r[4] = c.r[31].wrapping_add(c.r[29]);
                { let ea = c.r[4].wrapping_add(0x1f0); c.stw(ea, c.r[0]); c.r[4] = ea; }
                c.r[0] = 0x80;
                c.r[3] = 0x800d0000;
                c.stw(c.r[30], c.r[0]);
                c.r[8] = c.r[3].wrapping_add(0x860);
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x3;
                c.r[7] = 0x3;
                c.r[10] = 0x0;
                c.r[9] = 0x0;
                c.lr = 0x800d0a2c; f_800d06f4(c);
                pc = 0x800d0a2c;
            }
            0x800d0a2c => {
                pc = 0x800d0ae4; continue;
                pc = 0x800d0a30;
            }
            0x800d0a30 => {
                c.r[0] = c.r[28].rotate_left(0) & 0x40000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0a90; continue; }
                pc = 0x800d0a38;
            }
            0x800d0a38 => {
                c.r[3] = 0xd00000;
                c.r[0] = c.r[3].wrapping_add(0xffffff00);
                c.r[28] = c.r[28] & c.r[0];
                c.r[28] = c.r[28] | 0x100000;
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[4] = c.r[28].rotate_left(24) & 0xffff;
                c.lr = 0x800d0a54; f_800a115c(c);
                pc = 0x800d0a54;
            }
            0x800d0a54 => {
                c.r[0] = c.r[28] | 0x4e000000;
                c.r[4] = c.r[31].wrapping_add(c.r[29]);
                { let ea = c.r[4].wrapping_add(0x1f0); c.stw(ea, c.r[0]); c.r[4] = ea; }
                c.r[0] = 0x80;
                c.r[3] = 0x800d0000;
                c.stw(c.r[30], c.r[0]);
                c.r[8] = c.r[3].wrapping_add(0x860);
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[5] = 0x3;
                c.r[7] = 0x3;
                c.r[10] = 0x0;
                c.r[9] = 0x0;
                c.lr = 0x800d0a8c; f_800d06f4(c);
                pc = 0x800d0a8c;
            }
            0x800d0a8c => {
                pc = 0x800d0ae4; continue;
                pc = 0x800d0a90;
            }
            0x800d0a90 => {
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[4] = 0x0;
                c.lr = 0x800d0a9c; f_800a115c(c);
                pc = 0x800d0a9c;
            }
            0x800d0a9c => {
                c.r[0] = c.r[27].rotate_left(4) & 0xfffffff0;
                c.r[30] = c.lwz(c.r[30]);
                c.r[31] = c.r[31].wrapping_add(c.r[0]);
                c.r[29] = 0x0;
                c.r[28] = c.r[29].wrapping_add(0x0);
                c.r[31] = c.r[31].wrapping_add(0x160);
                pc = 0x800d0ab4;
            }
            0x800d0ab4 => {
                c.r[12] = c.lwz(c.r[31]);
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800d0ad4; continue; }
                pc = 0x800d0ac0;
            }
            0x800d0ac0 => {
                c.stw(c.r[31], c.r[28]);
                c.mtspr(8, c.r[12]);
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[4] = c.r[30].wrapping_add(0x0);
                if true { let t = c.lr; c.lr = 0x800d0ad4; (c.dispatch)(c, t); }
                pc = 0x800d0ad4;
            }
            0x800d0ad4 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.cmps(0, c.r[29], 0x4);
                c.r[31] = c.r[31].wrapping_add(0x4);
                if c.crbit(0) { pc = 0x800d0ab4; continue; }
                pc = 0x800d0ae4;
            }
            0x800d0ae4 => {
                { let mut ea = c.r[1].wrapping_add(0x18); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[1] = c.r[1].wrapping_add(0x30);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d0860: bad pc {:#x}", pc),
        }
    }
}

/// SIGetType (0x800d0af8, 452 bytes)
#[inline(never)]
pub fn f_800d0af8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d0af8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d0af8); f_800d0af8_b(c, 0x800d0af8); crate::prof::exit(c, 0x800d0af8); return; }
    }
    f_800d0af8_b(c, 0x800d0af8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d0af8_at(c: &mut Cpu, pc: u32) { f_800d0af8_b(c, pc) }
#[inline(always)]
fn f_800d0af8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d0af8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d0af8, pc); }
        match pc {
            0x800d0af8 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffd8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x800d0b10; f_8009df3c(c);
                pc = 0x800d0b10;
            }
            0x800d0b10 => {
                c.r[4] = 0x80310000;
                c.r[5] = c.r[27].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x3fa0);
                c.r[30] = c.r[0].wrapping_add(c.r[5]);
                c.r[28] = c.lwz(c.r[30]);
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x800d0b2c; f_800a2798(c);
                pc = 0x800d0b2c;
            }
            0x800d0b2c => {
                c.r[5] = 0x80400000;
                c.r[6] = c.r[27].rotate_left(3) & 0xfffffff8;
                c.r[0] = c.r[5].wrapping_add(0xd0);
                c.r[31] = c.r[0].wrapping_add(c.r[6]);
                c.r[6] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[5] = 0x80310000;
                c.r[5] = c.r[5].wrapping_add(0x3f8c);
                c.r[0] = c.lwz(c.r[31]);
                c.r[4] = c.addc(!c.r[6], c.r[4], 1);
                { let ci = c.ca(); c.r[0] = c.addc(!c.r[0], c.r[3], ci); }
                c.r[6] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[5] = 0x80;
                { let v = c.r[5] as i32; let n = c.r[27] & 0x3f; let r = if n > 31 { v >> 31 } else { v >> n }; c.set_ca(v < 0 && (n > 31 || (v as u32) & ((1u32 << n) - 1) != 0)); c.r[3] = r as u32; }
                c.r[3] = c.r[6] & c.r[3];
                c.rc(c.r[3]);
                if c.crbit(2) { pc = 0x800d0b98; continue; }
                pc = 0x800d0b68;
            }
            0x800d0b68 => {
                c.cmpu(0, c.r[28], 0x8);
                if c.crbit(2) { pc = 0x800d0b8c; continue; }
                pc = 0x800d0b70;
            }
            0x800d0b70 => {
                c.lr = 0x800d0b74; f_800a2798(c);
                pc = 0x800d0b74;
            }
            0x800d0b74 => {
                c.stw(c.r[31].wrapping_add(0x4), c.r[4]);
                c.stw(c.r[31], c.r[3]);
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800d0b84; f_8009df64(c);
                pc = 0x800d0b84;
            }
            0x800d0b84 => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x800d0ca8; continue;
                pc = 0x800d0b8c;
            }
            0x800d0b8c => {
                c.stw(c.r[30], c.r[5]);
                c.r[28] = 0x80;
                pc = 0x800d0c48; continue;
                pc = 0x800d0b98;
            }
            0x800d0b98 => {
                c.r[3] = 0x80000000;
                c.r[6] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[5] = 0x10620000;
                c.r[5] = c.r[5].wrapping_add(0x4dd3);
                c.r[6] = c.r[6].rotate_left(30) & 0x3fffffff;
                c.r[5] = mulhwu(c.r[5], c.r[6]);
                c.r[5] = c.r[5].rotate_left(26) & 0x3ffffff;
                c.r[7] = c.r[5].wrapping_mul(0x32);
                c.r[3] = 0x0;
                c.r[6] = c.r[3] ^ 0x80000000;
                c.r[5] = c.r[0] ^ 0x80000000;
                c.r[3] = c.addc(!c.r[4], c.r[7], 1);
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[5], c.r[6], ci); }
                { let ci = c.ca(); c.r[5] = c.addc(!c.r[6], c.r[6], ci); }
                c.r[5] = c.r[5].wrapping_neg();
                c.rc(c.r[5]);
                if !c.crbit(2) { pc = 0x800d0bf0; continue; }
                pc = 0x800d0bd8;
            }
            0x800d0bd8 => {
                c.cmpu(0, c.r[28], 0x8);
                if c.crbit(2) { pc = 0x800d0bf0; continue; }
                pc = 0x800d0be0;
            }
            0x800d0be0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800d0be8; f_8009df64(c);
                pc = 0x800d0be8;
            }
            0x800d0be8 => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x800d0ca8; continue;
                pc = 0x800d0bf0;
            }
            0x800d0bf0 => {
                c.r[3] = 0x80000000;
                c.r[6] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[5] = 0x10620000;
                c.r[5] = c.r[5].wrapping_add(0x4dd3);
                c.r[6] = c.r[6].rotate_left(30) & 0x3fffffff;
                c.r[5] = mulhwu(c.r[5], c.r[6]);
                c.r[5] = c.r[5].rotate_left(26) & 0x3ffffff;
                c.r[6] = c.r[5].wrapping_mul(0x4b);
                c.r[3] = 0x0;
                c.r[5] = c.r[3] ^ 0x80000000;
                c.r[3] = c.r[0] ^ 0x80000000;
                c.r[0] = c.addc(!c.r[4], c.r[6], 1);
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[3], c.r[5], ci); }
                { let ci = c.ca(); c.r[3] = c.addc(!c.r[5], c.r[5], ci); }
                c.r[3] = c.r[3].wrapping_neg();
                c.rc(c.r[3]);
                if !c.crbit(2) { pc = 0x800d0c3c; continue; }
                pc = 0x800d0c30;
            }
            0x800d0c30 => {
                c.r[0] = 0x80;
                c.stw(c.r[30], c.r[0]);
                pc = 0x800d0c48; continue;
                pc = 0x800d0c3c;
            }
            0x800d0c3c => {
                c.r[0] = 0x80;
                c.stw(c.r[30], c.r[0]);
                c.r[28] = 0x80;
                pc = 0x800d0c48;
            }
            0x800d0c48 => {
                c.lr = 0x800d0c4c; f_800a2798(c);
                pc = 0x800d0c4c;
            }
            0x800d0c4c => {
                c.stw(c.r[31].wrapping_add(0x4), c.r[4]);
                c.r[4] = 0x431c0000;
                c.r[5] = 0x800d0000;
                c.stw(c.r[31], c.r[3]);
                c.r[3] = 0x80000000;
                c.r[8] = c.r[5].wrapping_add(0x860);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xf8));
                c.r[4] = c.r[4].wrapping_add(0xffffde83);
                c.r[3] = c.r[27].wrapping_add(0x0);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = mulhwu(c.r[4], c.r[0]);
                c.r[0] = c.r[0].rotate_left(17) & 0x1ffff;
                c.r[0] = c.r[0].wrapping_mul(0x41);
                c.r[10] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.r[6] = c.r[30].wrapping_add(0x0);
                c.r[4] = c.r[13].wrapping_add(0xffffa234);
                c.r[5] = 0x1;
                c.r[7] = 0x3;
                c.r[9] = 0x0;
                c.lr = 0x800d0c9c; f_800d06f4(c);
                pc = 0x800d0c9c;
            }
            0x800d0c9c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x800d0ca4; f_8009df64(c);
                pc = 0x800d0ca4;
            }
            0x800d0ca4 => {
                c.r[3] = c.r[28] | c.r[28];
                pc = 0x800d0ca8;
            }
            0x800d0ca8 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[1] = c.r[1].wrapping_add(0x28);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d0af8: bad pc {:#x}", pc),
        }
    }
}

/// SIGetTypeAsync (0x800d0cbc, 316 bytes)
#[inline(never)]
pub fn f_800d0cbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d0cbc) { return; }
        if c.prof { crate::prof::enter(c, 0x800d0cbc); f_800d0cbc_b(c, 0x800d0cbc); crate::prof::exit(c, 0x800d0cbc); return; }
    }
    f_800d0cbc_b(c, 0x800d0cbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d0cbc_at(c: &mut Cpu, pc: u32) { f_800d0cbc_b(c, pc) }
#[inline(always)]
fn f_800d0cbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d0cbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d0cbc, pc); }
        match pc {
            0x800d0cbc => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x0);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3].wrapping_add(0x0);
                c.lr = 0x800d0ce4; f_8009df3c(c);
                pc = 0x800d0ce4;
            }
            0x800d0ce4 => {
                c.r[30] = c.r[3].wrapping_add(0x0);
                c.r[3] = c.r[28].wrapping_add(0x0);
                c.lr = 0x800d0cf0; f_800d0af8(c);
                pc = 0x800d0cf0;
            }
            0x800d0cf0 => {
                c.r[4] = 0x80310000;
                c.r[5] = c.r[28].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x3fa0);
                c.r[4] = c.r[0].wrapping_add(c.r[5]);
                c.r[0] = c.lwz(c.r[4]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.r[0] = c.r[0].rotate_left(0) & 0x80;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0db8; continue; }
                pc = 0x800d0d10;
            }
            0x800d0d10 => {
                c.r[3] = 0x80400000;
                c.r[4] = c.r[28].rotate_left(4) & 0xfffffff0;
                c.r[0] = c.r[3].wrapping_add(0x110);
                c.r[4] = c.r[0].wrapping_add(c.r[4]);
                c.r[0] = c.lwz(c.r[4]);
                c.r[5] = 0x0;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(2) { pc = 0x800d0dcc; continue; }
                pc = 0x800d0d30;
            }
            0x800d0d30 => {
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d0d44; continue; }
                pc = 0x800d0d38;
            }
            0x800d0d38 => {
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[4].wrapping_add(c.r[0]), c.r[31]);
                pc = 0x800d0dcc; continue;
                pc = 0x800d0d44;
            }
            0x800d0d44 => {
                c.r[3] = c.r[4].wrapping_add(0x4);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[5] = 0x1;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(2) { pc = 0x800d0dcc; continue; }
                pc = 0x800d0d58;
            }
            0x800d0d58 => {
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d0d6c; continue; }
                pc = 0x800d0d60;
            }
            0x800d0d60 => {
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[4].wrapping_add(c.r[0]), c.r[31]);
                pc = 0x800d0dcc; continue;
                pc = 0x800d0d6c;
            }
            0x800d0d6c => {
                { let ea = c.r[3].wrapping_add(0x4); c.r[0] = c.lwz(ea); c.r[3] = ea; }
                c.r[5] = 0x2;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(2) { pc = 0x800d0dcc; continue; }
                pc = 0x800d0d7c;
            }
            0x800d0d7c => {
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d0d90; continue; }
                pc = 0x800d0d84;
            }
            0x800d0d84 => {
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[4].wrapping_add(c.r[0]), c.r[31]);
                pc = 0x800d0dcc; continue;
                pc = 0x800d0d90;
            }
            0x800d0d90 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[5] = 0x3;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(2) { pc = 0x800d0dcc; continue; }
                pc = 0x800d0da0;
            }
            0x800d0da0 => {
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d0dcc; continue; }
                pc = 0x800d0da8;
            }
            0x800d0da8 => {
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[4].wrapping_add(c.r[0]), c.r[31]);
                pc = 0x800d0dcc; continue;
                pc = 0x800d0db4;
            }
            0x800d0db4 => {
                pc = 0x800d0dcc; continue;
                pc = 0x800d0db8;
            }
            0x800d0db8 => {
                c.r[12] = c.r[31].wrapping_add(0x0);
                c.mtspr(8, c.r[12]);
                c.r[3] = c.r[28].wrapping_add(0x0);
                c.r[4] = c.r[29].wrapping_add(0x0);
                if true { let t = c.lr; c.lr = 0x800d0dcc; (c.dispatch)(c, t); }
                pc = 0x800d0dcc;
            }
            0x800d0dcc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800d0dd4; f_8009df64(c);
                pc = 0x800d0dd4;
            }
            0x800d0dd4 => {
                c.r[3] = c.r[29] | c.r[29];
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
            _ => panic!("f_800d0cbc: bad pc {:#x}", pc),
        }
    }
}

/// SIDecodeType (0x800d0df8, 332 bytes)
#[inline(never)]
pub fn f_800d0df8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d0df8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d0df8); f_800d0df8_b(c, 0x800d0df8); crate::prof::exit(c, 0x800d0df8); return; }
    }
    f_800d0df8_b(c, 0x800d0df8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d0df8_at(c: &mut Cpu, pc: u32) { f_800d0df8_b(c, pc) }
#[inline(always)]
fn f_800d0df8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d0df8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d0df8, pc); }
        match pc {
            0x800d0df8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x8;
                c.rc(c.r[0]);
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                c.r[5] = c.r[3].rotate_left(0) & 0xffffff00;
                if c.crbit(2) { pc = 0x800d0e10; continue; }
                pc = 0x800d0e08;
            }
            0x800d0e08 => {
                c.r[3] = 0x8;
                if true { return; }
                pc = 0x800d0e10;
            }
            0x800d0e10 => {
                c.r[0] = c.r[4] & 0x47;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0e20; continue; }
                pc = 0x800d0e18;
            }
            0x800d0e18 => {
                c.r[3] = 0x40;
                if true { return; }
                pc = 0x800d0e20;
            }
            0x800d0e20 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800d0e30; continue; }
                pc = 0x800d0e28;
            }
            0x800d0e28 => {
                c.r[3] = 0x80;
                if true { return; }
                pc = 0x800d0e30;
            }
            0x800d0e30 => {
                c.r[3] = c.r[5].rotate_left(0) & 0x18000000;
                c.rc(c.r[3]);
                if !c.crbit(2) { pc = 0x800d0e98; continue; }
                pc = 0x800d0e38;
            }
            0x800d0e38 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xffff0000;
                c.r[0] = 0x40000;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(2) { return; }
                pc = 0x800d0e48;
            }
            0x800d0e48 => {
                if !c.crbit(0) { pc = 0x800d0e6c; continue; }
                pc = 0x800d0e4c;
            }
            0x800d0e4c => {
                c.r[0] = 0x20000;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(2) { return; }
                pc = 0x800d0e58;
            }
            0x800d0e58 => {
                if !c.crbit(0) { pc = 0x800d0e90; continue; }
                pc = 0x800d0e5c;
            }
            0x800d0e5c => {
                c.r[0] = 0x10000;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(2) { return; }
                pc = 0x800d0e68;
            }
            0x800d0e68 => {
                pc = 0x800d0e90; continue;
                pc = 0x800d0e6c;
            }
            0x800d0e6c => {
                c.r[0] = 0x5000000;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(2) { return; }
                pc = 0x800d0e78;
            }
            0x800d0e78 => {
                if !c.crbit(0) { pc = 0x800d0e90; continue; }
                pc = 0x800d0e7c;
            }
            0x800d0e7c => {
                c.r[0] = 0x2000000;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(2) { return; }
                pc = 0x800d0e88;
            }
            0x800d0e88 => {
                pc = 0x800d0e90; continue;
                pc = 0x800d0e8c;
            }
            0x800d0e8c => {
                if true { return; }
                pc = 0x800d0e90;
            }
            0x800d0e90 => {
                c.r[3] = 0x40;
                if true { return; }
                pc = 0x800d0e98;
            }
            0x800d0e98 => {
                c.r[0] = c.r[3].wrapping_add(0xf8000000);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d0eac; continue; }
                pc = 0x800d0ea4;
            }
            0x800d0ea4 => {
                c.r[3] = 0x40;
                if true { return; }
                pc = 0x800d0eac;
            }
            0x800d0eac => {
                c.r[3] = c.r[5].rotate_left(0) & 0xffff0000;
                c.r[0] = 0x9000000;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(2) { return; }
                pc = 0x800d0ebc;
            }
            0x800d0ebc => {
                if !c.crbit(0) { pc = 0x800d0ed4; continue; }
                pc = 0x800d0ec0;
            }
            0x800d0ec0 => {
                c.r[0] = 0x8000000;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(2) { return; }
                pc = 0x800d0ecc;
            }
            0x800d0ecc => {
                pc = 0x800d0ed4; continue;
                pc = 0x800d0ed0;
            }
            0x800d0ed0 => {
                if true { return; }
                pc = 0x800d0ed4;
            }
            0x800d0ed4 => {
                c.r[3] = c.r[5].rotate_left(0) & 0xffe00000;
                c.r[0] = c.r[3].wrapping_add(0xf7e00000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d0eec; continue; }
                pc = 0x800d0ee4;
            }
            0x800d0ee4 => {
                c.r[3] = 0x8200000;
                if true { return; }
                pc = 0x800d0eec;
            }
            0x800d0eec => {
                c.r[0] = c.r[5].rotate_left(0) & 0x80000000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d0f24; continue; }
                pc = 0x800d0ef4;
            }
            0x800d0ef4 => {
                c.r[0] = c.r[5].rotate_left(0) & 0x4000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800d0f24; continue; }
                pc = 0x800d0efc;
            }
            0x800d0efc => {
                c.r[3] = c.r[5] & 0x8b100000;
                c.rc(c.r[3]);
                c.r[0] = c.r[3].wrapping_add(0x74f00000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d0f14; continue; }
                pc = 0x800d0f0c;
            }
            0x800d0f0c => {
                c.r[3] = 0x8b100000;
                if true { return; }
                pc = 0x800d0f14;
            }
            0x800d0f14 => {
                c.r[0] = c.r[5].rotate_left(0) & 0x2000000;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800d0f24; continue; }
                pc = 0x800d0f1c;
            }
            0x800d0f1c => {
                c.r[3] = 0x88000000;
                if true { return; }
                pc = 0x800d0f24;
            }
            0x800d0f24 => {
                c.r[3] = c.r[5] & 0x9000000;
                c.rc(c.r[3]);
                c.r[0] = c.r[3].wrapping_add(0xf7000000);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d0f3c; continue; }
                pc = 0x800d0f34;
            }
            0x800d0f34 => {
                c.r[3] = 0x9000000;
                if true { return; }
                pc = 0x800d0f3c;
            }
            0x800d0f3c => {
                c.r[3] = 0x40;
                if true { return; }
                return;
            }
            _ => panic!("f_800d0df8: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D0F44 (0x800d0f44, 36 bytes)
#[inline(never)]
pub fn f_800d0f44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d0f44) { return; }
        if c.prof { crate::prof::enter(c, 0x800d0f44); f_800d0f44_b(c, 0x800d0f44); crate::prof::exit(c, 0x800d0f44); return; }
    }
    f_800d0f44_b(c, 0x800d0f44)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d0f44_at(c: &mut Cpu, pc: u32) { f_800d0f44_b(c, pc) }
#[inline(always)]
fn f_800d0f44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d0f44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d0f44, pc); }
        match pc {
            0x800d0f44 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.lr = 0x800d0f54; f_800d0af8(c);
                pc = 0x800d0f54;
            }
            0x800d0f54 => {
                c.lr = 0x800d0f58; f_800d0df8(c);
                pc = 0x800d0f58;
            }
            0x800d0f58 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d0f44: bad pc {:#x}", pc),
        }
    }
}

/// SISetSamplingRate (0x800d0f68, 228 bytes)
#[inline(never)]
pub fn f_800d0f68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d0f68) { return; }
        if c.prof { crate::prof::enter(c, 0x800d0f68); f_800d0f68_b(c, 0x800d0f68); crate::prof::exit(c, 0x800d0f68); return; }
    }
    f_800d0f68_b(c, 0x800d0f68)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d0f68_at(c: &mut Cpu, pc: u32) { f_800d0f68_b(c, pc) }
#[inline(always)]
fn f_800d0f68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d0f68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d0f68, pc); }
        match pc {
            0x800d0f68 => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3].wrapping_add(0x0);
                c.cmpu(0, c.r[29], 0xb);
                c.r[3] = 0x80310000;
                c.r[31] = c.r[3].wrapping_add(0x4060);
                if !c.crbit(1) { pc = 0x800d0f98; continue; }
                pc = 0x800d0f94;
            }
            0x800d0f94 => {
                c.r[29] = 0xb;
                pc = 0x800d0f98;
            }
            0x800d0f98 => {
                c.lr = 0x800d0f9c; f_8009df3c(c);
                pc = 0x800d0f9c;
            }
            0x800d0f9c => {
                c.stw(c.r[13].wrapping_add(0xffffa240), c.r[29]);
                c.r[30] = c.r[3] | c.r[3];
                c.lr = 0x800d0fa8; f_800aa430(c);
                pc = 0x800d0fa8;
            }
            0x800d0fa8 => {
                c.cmps(0, c.r[3], 0x2);
                if c.crbit(2) { pc = 0x800d0fd0; continue; }
                pc = 0x800d0fb0;
            }
            0x800d0fb0 => {
                if !c.crbit(0) { pc = 0x800d0fc4; continue; }
                pc = 0x800d0fb4;
            }
            0x800d0fb4 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d0fd0; continue; }
                pc = 0x800d0fbc;
            }
            0x800d0fbc => {
                if !c.crbit(0) { pc = 0x800d0fd8; continue; }
                pc = 0x800d0fc0;
            }
            0x800d0fc0 => {
                pc = 0x800d0fe0; continue;
                pc = 0x800d0fc4;
            }
            0x800d0fc4 => {
                c.cmps(0, c.r[3], 0x5);
                if c.crbit(2) { pc = 0x800d0fd0; continue; }
                pc = 0x800d0fcc;
            }
            0x800d0fcc => {
                pc = 0x800d0fe0; continue;
                pc = 0x800d0fd0;
            }
            0x800d0fd0 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x800d0ff4; continue;
                pc = 0x800d0fd8;
            }
            0x800d0fd8 => {
                c.r[4] = c.r[31].wrapping_add(0x30);
                pc = 0x800d0ff4; continue;
                pc = 0x800d0fe0;
            }
            0x800d0fe0 => {
                c.r[3] = c.r[31].wrapping_add(0x60);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d0fec; f_8009c2e0(c);
                pc = 0x800d0fec;
            }
            0x800d0fec => {
                c.r[29] = 0x0;
                c.r[4] = c.r[31].wrapping_add(0x0);
                pc = 0x800d0ff4;
            }
            0x800d0ff4 => {
                c.r[3] = 0xcc000000;
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x206c));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d100c; continue; }
                pc = 0x800d1004;
            }
            0x800d1004 => {
                c.r[5] = 0x2;
                pc = 0x800d1010; continue;
                pc = 0x800d100c;
            }
            0x800d100c => {
                c.r[5] = 0x1;
                pc = 0x800d1010;
            }
            0x800d1010 => {
                c.r[3] = c.r[29].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lhz(c.r[4].wrapping_add(c.r[3]));
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[4] = c.lbz(c.r[3].wrapping_add(0x2));
                c.r[3] = c.r[5].wrapping_mul(c.r[0]);
                c.lr = 0x800d1028; f_800d035c(c);
                pc = 0x800d1028;
            }
            0x800d1028 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800d1030; f_8009df64(c);
                pc = 0x800d1030;
            }
            0x800d1030 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[1] = c.r[1].wrapping_add(0x20);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d0f68: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D104C (0x800d104c, 36 bytes)
#[inline(never)]
pub fn f_800d104c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d104c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d104c); f_800d104c_b(c, 0x800d104c); crate::prof::exit(c, 0x800d104c); return; }
    }
    f_800d104c_b(c, 0x800d104c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d104c_at(c: &mut Cpu, pc: u32) { f_800d104c_b(c, pc) }
#[inline(always)]
fn f_800d104c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d104c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d104c, pc); }
        match pc {
            0x800d104c => {
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xfffffff8); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa240));
                c.lr = 0x800d1060; f_800d0f68(c);
                pc = 0x800d1060;
            }
            0x800d1060 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[1] = c.r[1].wrapping_add(0x8);
                c.mtspr(8, c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d104c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D1070 (0x800d1070, 852 bytes)
#[inline(never)]
pub fn f_800d1070(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1070) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1070); f_800d1070_b(c, 0x800d1070); crate::prof::exit(c, 0x800d1070); return; }
    }
    f_800d1070_b(c, 0x800d1070)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1070_at(c: &mut Cpu, pc: u32) { f_800d1070_b(c, pc) }
#[inline(always)]
fn f_800d1070_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1070;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1070, pc); }
        match pc {
            0x800d1070 => {
                { let ea = c.r[1].wrapping_add(0xffffff40); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xc4), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xbc), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0xb8), c.r[30]);
                c.r[30] = 0x0;
                c.stw(c.r[1].wrapping_add(0xb4), c.r[29]);
                c.stw(c.r[1].wrapping_add(0xb0), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x800d1398; continue;
                pc = 0x800d109c;
            }
            0x800d109c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa24c));
                c.r[29] = c.r[0].wrapping_add(c.r[31]);
                c.r[0] = c.lbz(c.r[29]);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d1390; continue; }
                pc = 0x800d10b0;
            }
            0x800d10b0 => {
                c.r[0] = c.lbz(c.r[29].wrapping_add(0x4));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d1210; continue; }
                pc = 0x800d10bc;
            }
            0x800d10bc => {
                c.r[3] = c.r[29].wrapping_add(0x7c);
                c.r[4] = c.r[29].wrapping_add(0x70);
                c.lr = 0x800d10c8; f_800e01d0(c);
                pc = 0x800d10c8;
            }
            0x800d10c8 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                { let ea = c.r[29].wrapping_add(0x11c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x800d10d4; f_80196698(c);
                pc = 0x800d10d4;
            }
            0x800d10d4 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.lr = 0x800d10dc; f_801966fc(c);
                pc = 0x800d10dc;
            }
            0x800d10dc => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[4] = c.r[29].wrapping_add(0x70);
                c.lr = 0x800d10e8; f_80195904(c);
                pc = 0x800d10e8;
            }
            0x800d10e8 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[4] = c.r[29].wrapping_add(0xf4);
                c.lr = 0x800d10f4; f_801950d0(c);
                pc = 0x800d10f4;
            }
            0x800d10f4 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[4] = c.r[29].wrapping_add(0x100);
                c.lr = 0x800d1100; f_801959dc(c);
                pc = 0x800d1100;
            }
            0x800d1100 => {
                c.r[3] = 0x43300000;
                c.r[0] = c.lbz(c.r[29].wrapping_add(0x125));
                c.stw(c.r[1].wrapping_add(0xac), c.r[28]);
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                { let ea = c.r[2].wrapping_add(0xffff9308); c.f[2] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0xa8), c.r[3]);
                c.cmps(0, c.r[0], 0xffffffff);
                { let ea = c.r[29].wrapping_add(0x118); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0xa8); c.f[0] = c.lfd(ea); }
                { let ea = c.r[29].wrapping_add(0x120); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[0] - c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[2].wrapping_add(0xffff92f0); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10c));
                { let v = s(c.f[1] - c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                if !c.crbit(2) { pc = 0x800d1150; continue; }
                pc = 0x800d1140;
            }
            0x800d1140 => {
                { let ea = c.r[29].wrapping_add(0x11c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[29].wrapping_add(0x11c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d1164; continue;
                pc = 0x800d1150;
            }
            0x800d1150 => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d1164; continue; }
                pc = 0x800d1158;
            }
            0x800d1158 => {
                { let ea = c.r[29].wrapping_add(0x11c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] + c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[29].wrapping_add(0x11c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d1164;
            }
            0x800d1164 => {
                c.cmps(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x800d11b8; continue; }
                pc = 0x800d116c;
            }
            0x800d116c => {
                if !c.crbit(0) { pc = 0x800d117c; continue; }
                pc = 0x800d1170;
            }
            0x800d1170 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x800d1188; continue; }
                pc = 0x800d1178;
            }
            0x800d1178 => {
                pc = 0x800d1208; continue;
                pc = 0x800d117c;
            }
            0x800d117c => {
                c.cmps(0, c.r[3], 0x3);
                if !c.crbit(0) { pc = 0x800d1208; continue; }
                pc = 0x800d1184;
            }
            0x800d1184 => {
                pc = 0x800d11d4; continue;
                pc = 0x800d1188;
            }
            0x800d1188 => {
                { let ea = c.r[2].wrapping_add(0xffff92f4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x11c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[4] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.fcmp(0, c.f[1], c.f[0]);
                { let (a, b) = (c.crbit(1), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x800d1208; continue; }
                pc = 0x800d11a0;
            }
            0x800d11a0 => {
                c.r[3] = 0x1;
                c.r[0] = 0x0;
                c.stb(c.r[29].wrapping_add(0x124), c.r[3]);
                c.stb(c.r[29].wrapping_add(0x125), c.r[0]);
                { let ea = c.r[29].wrapping_add(0x11c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d1208; continue;
                pc = 0x800d11b8;
            }
            0x800d11b8 => {
                { let ea = c.r[29].wrapping_add(0x11c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[0], c.f[4]);
                { let (a, b) = (c.crbit(1), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x800d1208; continue; }
                pc = 0x800d11c8;
            }
            0x800d11c8 => {
                { let v = s(c.f[0] - c.f[4]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[29].wrapping_add(0x11c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d1208; continue;
                pc = 0x800d11d4;
            }
            0x800d11d4 => {
                { let ea = c.r[29].wrapping_add(0x11c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.fcmp(0, c.f[1], c.f[4]);
                { let (a, b) = (c.crbit(1), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x800d11f0; continue; }
                pc = 0x800d11e4;
            }
            0x800d11e4 => {
                c.r[0] = 0xffffffff;
                c.stb(c.r[29].wrapping_add(0x125), c.r[0]);
                pc = 0x800d1208; continue;
                pc = 0x800d11f0;
            }
            0x800d11f0 => {
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                { let (a, b) = (c.crbit(0), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x800d1208; continue; }
                pc = 0x800d1200;
            }
            0x800d1200 => {
                c.r[0] = 0x1;
                c.stb(c.r[29].wrapping_add(0x125), c.r[0]);
                pc = 0x800d1208;
            }
            0x800d1208 => {
                c.r[0] = 0x1;
                c.stb(c.r[29].wrapping_add(0x2), c.r[0]);
                pc = 0x800d1210;
            }
            0x800d1210 => {
                c.r[0] = c.lbz(c.r[29].wrapping_add(0x4));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1258; continue; }
                pc = 0x800d121c;
            }
            0x800d121c => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.lr = 0x800d1224; f_80194cc4(c);
                pc = 0x800d1224;
            }
            0x800d1224 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[29].wrapping_add(0x94);
                c.lr = 0x800d1230; f_800e0628(c);
                pc = 0x800d1230;
            }
            0x800d1230 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[4] = c.r[29].wrapping_add(0x70);
                c.lr = 0x800d123c; f_80195904(c);
                pc = 0x800d123c;
            }
            0x800d123c => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[4] = c.r[29].wrapping_add(0xf4);
                c.lr = 0x800d1248; f_801950d0(c);
                pc = 0x800d1248;
            }
            0x800d1248 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[4] = c.r[29].wrapping_add(0x100);
                c.lr = 0x800d1254; f_801959dc(c);
                pc = 0x800d1254;
            }
            0x800d1254 => {
                pc = 0x800d1388; continue;
                pc = 0x800d1258;
            }
            0x800d1258 => {
                c.r[0] = c.lbz(c.r[29].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d1308; continue; }
                pc = 0x800d1264;
            }
            0x800d1264 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[29].wrapping_add(0x70);
                c.r[5] = c.r[29].wrapping_add(0x100);
                c.lr = 0x800d1274; f_800e0168(c);
                pc = 0x800d1274;
            }
            0x800d1274 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d1288; continue; }
                pc = 0x800d1284;
            }
            0x800d1284 => {
                pc = 0x800d128c; continue;
                pc = 0x800d1288;
            }
            0x800d1288 => {
                c.f[1] = -c.f[1];
                pc = 0x800d128c;
            }
            0x800d128c => {
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8acc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d12f0; continue; }
                pc = 0x800d129c;
            }
            0x800d129c => {
                { let ea = c.r[1].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d12b0; continue; }
                pc = 0x800d12ac;
            }
            0x800d12ac => {
                pc = 0x800d12b4; continue;
                pc = 0x800d12b0;
            }
            0x800d12b0 => {
                c.f[1] = -c.f[1];
                pc = 0x800d12b4;
            }
            0x800d12b4 => {
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d12f0; continue; }
                pc = 0x800d12bc;
            }
            0x800d12bc => {
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d12d0; continue; }
                pc = 0x800d12cc;
            }
            0x800d12cc => {
                pc = 0x800d12d4; continue;
                pc = 0x800d12d0;
            }
            0x800d12d0 => {
                c.f[1] = -c.f[1];
                pc = 0x800d12d4;
            }
            0x800d12d4 => {
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d12f0; continue; }
                pc = 0x800d12dc;
            }
            0x800d12dc => {
                { let ea = c.r[29].wrapping_add(0x100); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9300); c.f[0] = c.lfd(ea); }
                c.f[0] = c.f[1] + c.f[0];
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[29].wrapping_add(0x100); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d12f0;
            }
            0x800d12f0 => {
                c.r[3] = c.r[29].wrapping_add(0x94);
                c.r[4] = c.r[29].wrapping_add(0x70);
                c.r[5] = c.r[29].wrapping_add(0xf4);
                c.r[6] = c.r[29].wrapping_add(0x100);
                c.lr = 0x800d1304; f_800e0218(c);
                pc = 0x800d1304;
            }
            0x800d1304 => {
                pc = 0x800d1388; continue;
                pc = 0x800d1308;
            }
            0x800d1308 => {
                { let ea = c.r[29].wrapping_add(0x88); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[29].wrapping_add(0x94);
                c.f[1] = -c.f[0];
                c.lr = 0x800d1318; f_800e053c(c);
                pc = 0x800d1318;
            }
            0x800d1318 => {
                { let ea = c.r[29].wrapping_add(0x8c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x44);
                c.f[1] = -c.f[0];
                c.lr = 0x800d1328; f_800e0518(c);
                pc = 0x800d1328;
            }
            0x800d1328 => {
                { let ea = c.r[29].wrapping_add(0x90); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x74);
                c.f[1] = -c.f[0];
                c.lr = 0x800d1338; f_800e04f4(c);
                pc = 0x800d1338;
            }
            0x800d1338 => {
                { let ea = c.r[29].wrapping_add(0x70); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x14);
                { let ea = c.r[29].wrapping_add(0x74); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[29].wrapping_add(0x78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[1] = -c.f[1];
                c.f[2] = -c.f[2];
                c.f[3] = -c.f[0];
                c.lr = 0x800d1358; f_800e05c0(c);
                pc = 0x800d1358;
            }
            0x800d1358 => {
                c.r[3] = c.r[29].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x44);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d1368; f_800e0290(c);
                pc = 0x800d1368;
            }
            0x800d1368 => {
                c.r[3] = c.r[29].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x74);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d1378; f_800e0290(c);
                pc = 0x800d1378;
            }
            0x800d1378 => {
                c.r[3] = c.r[29].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x14);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d1388; f_800e0290(c);
                pc = 0x800d1388;
            }
            0x800d1388 => {
                c.r[0] = 0x0;
                c.stb(c.r[29].wrapping_add(0x2), c.r[0]);
                pc = 0x800d1390;
            }
            0x800d1390 => {
                c.r[31] = c.r[31].wrapping_add(0x128);
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x800d1398;
            }
            0x800d1398 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa250));
                c.cmpu(0, c.r[30], c.r[0]);
                if c.crbit(0) { pc = 0x800d109c; continue; }
                pc = 0x800d13a4;
            }
            0x800d13a4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc4));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xbc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0xb8));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0xb4));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0xb0));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xc0);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1070: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D13C4 (0x800d13c4, 4 bytes)
#[inline(never)]
pub fn f_800d13c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d13c4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d13c4); f_800d13c4_b(c, 0x800d13c4); crate::prof::exit(c, 0x800d13c4); return; }
    }
    f_800d13c4_b(c, 0x800d13c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d13c4_at(c: &mut Cpu, pc: u32) { f_800d13c4_b(c, pc) }
#[inline(always)]
fn f_800d13c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d13c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d13c4, pc); }
        match pc {
            0x800d13c4 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800d13c4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D13C8 (0x800d13c8, 684 bytes)
#[inline(never)]
pub fn f_800d13c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d13c8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d13c8); f_800d13c8_b(c, 0x800d13c8); crate::prof::exit(c, 0x800d13c8); return; }
    }
    f_800d13c8_b(c, 0x800d13c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d13c8_at(c: &mut Cpu, pc: u32) { f_800d13c8_b(c, pc) }
#[inline(always)]
fn f_800d13c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d13c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d13c8, pc); }
        match pc {
            0x800d13c8 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = c.r[30].wrapping_add(0x70);
                c.r[4] = c.r[31].wrapping_add(0x4);
                c.lr = 0x800d13fc; f_800e01d0(c);
                pc = 0x800d13fc;
            }
            0x800d13fc => {
                c.r[3] = c.r[30].wrapping_add(0x7c);
                c.r[4] = c.r[31].wrapping_add(0x10);
                c.lr = 0x800d1408; f_800e01d0(c);
                pc = 0x800d1408;
            }
            0x800d1408 => {
                c.r[3] = c.r[30].wrapping_add(0x88);
                c.r[4] = c.r[31].wrapping_add(0x1c);
                c.lr = 0x800d1414; f_800e01d0(c);
                pc = 0x800d1414;
            }
            0x800d1414 => {
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.r[4] = c.r[31].wrapping_add(0x28);
                c.lr = 0x800d1420; f_800e0628(c);
                pc = 0x800d1420;
            }
            0x800d1420 => {
                c.r[3] = c.r[30].wrapping_add(0xc4);
                c.r[4] = c.r[31].wrapping_add(0x58);
                c.lr = 0x800d142c; f_800e0628(c);
                pc = 0x800d142c;
            }
            0x800d142c => {
                c.r[3] = c.r[30].wrapping_add(0xf4);
                c.r[4] = c.r[31].wrapping_add(0x88);
                c.lr = 0x800d1438; f_800e01d0(c);
                pc = 0x800d1438;
            }
            0x800d1438 => {
                c.r[3] = c.r[30].wrapping_add(0x100);
                c.r[4] = c.r[31].wrapping_add(0x94);
                c.lr = 0x800d1444; f_800e01d0(c);
                pc = 0x800d1444;
            }
            0x800d1444 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.stb(c.r[30].wrapping_add(0x124), c.r[0]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d1650; continue; }
                pc = 0x800d1458;
            }
            0x800d1458 => {
                c.r[0] = c.lbz(c.r[31]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1650; continue; }
                pc = 0x800d1464;
            }
            0x800d1464 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x3));
                c.r[29] = c.lwz(c.r[31].wrapping_add(0xa4));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d14e8; continue; }
                pc = 0x800d1474;
            }
            0x800d1474 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.lr = 0x800d147c; f_80196bb8(c);
                pc = 0x800d147c;
            }
            0x800d147c => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x110));
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(1) { pc = 0x800d14e8; continue; }
                pc = 0x800d1488;
            }
            0x800d1488 => {
                c.stw(c.r[30].wrapping_add(0x114), c.r[29]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x8));
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x114));
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                c.lr = 0x800d14a8; f_80196b10(c);
                pc = 0x800d14a8;
            }
            0x800d14a8 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x800d14b4; f_80196698(c);
                pc = 0x800d14b4;
            }
            0x800d14b4 => {
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = 0x800d0000;
                c.r[6] = c.r[3].wrapping_add(0x2b44);
                c.r[4] = 0x2;
                { let ea = c.r[13].wrapping_add(0xffffa258); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = 0x10000;
                c.r[5] = c.r[3].wrapping_add(0xffffffff);
                c.r[7] = 0x0;
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d14e0; f_801c028c(c);
                pc = 0x800d14e0;
            }
            0x800d14e0 => {
                { let ea = c.r[13].wrapping_add(0xffffa258); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[30].wrapping_add(0x120); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d14e8;
            }
            0x800d14e8 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xa0));
                c.stw(c.r[30].wrapping_add(0x10c), c.r[0]);
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x3));
                { let ea = c.r[31].wrapping_add(0xa8); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1540; continue; }
                pc = 0x800d1500;
            }
            0x800d1500 => {
                c.lr = 0x800d1504; f_800d37cc(c);
                pc = 0x800d1504;
            }
            0x800d1504 => {
                c.cmps(0, c.r[3], 0x32);
                if !c.crbit(2) { pc = 0x800d1514; continue; }
                pc = 0x800d150c;
            }
            0x800d150c => {
                { let ea = c.r[2].wrapping_add(0xffff9310); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[31] * c.f[0]); c.f[31] = v; c.ps1[31] = v; }
                pc = 0x800d1514;
            }
            0x800d1514 => {
                { let ea = c.r[30].wrapping_add(0x118); let v = c.f[31]; c.stfs(ea, v); }
                c.r[3] = 0x801c0000;
                c.r[6] = c.r[3].wrapping_add(0x27c);
                c.r[4] = 0x10000;
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                { let ea = c.r[30].wrapping_add(0x118); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = 0x2;
                c.r[7] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d1540; f_801c028c(c);
                pc = 0x800d1540;
            }
            0x800d1540 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x3));
                { let ea = c.r[31].wrapping_add(0xac); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d15b8; continue; }
                pc = 0x800d1550;
            }
            0x800d1550 => {
                { let ea = c.r[30].wrapping_add(0x11c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = 0x801c0000;
                c.r[6] = c.r[3].wrapping_add(0x27c);
                c.r[4] = 0x10000;
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                { let ea = c.r[2].wrapping_add(0xffff9318); c.f[1] = c.lfd(ea); }
                c.r[4] = 0x2;
                c.r[7] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d157c; f_801c028c(c);
                pc = 0x800d157c;
            }
            0x800d157c => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                { let ea = c.r[30].wrapping_add(0x11c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x800d1588; f_80196698(c);
                pc = 0x800d1588;
            }
            0x800d1588 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.lr = 0x800d1590; f_801966fc(c);
                pc = 0x800d1590;
            }
            0x800d1590 => {
                c.r[3] = 0x801c0000;
                c.r[4] = 0x10000;
                c.r[6] = c.r[3].wrapping_add(0x27c);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                { let ea = c.r[30].wrapping_add(0x118); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = 0x2;
                c.r[7] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d15b8; f_801c028c(c);
                pc = 0x800d15b8;
            }
            0x800d15b8 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x3));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1650; continue; }
                pc = 0x800d15c4;
            }
            0x800d15c4 => {
                c.r[3] = 0x1;
                c.r[0] = 0x0;
                c.stb(c.r[30].wrapping_add(0x4), c.r[3]);
                c.stb(c.r[30].wrapping_add(0x124), c.r[0]);
                c.stb(c.r[30].wrapping_add(0x125), c.r[3]);
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x3));
                { let ea = c.r[30].wrapping_add(0x11c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1650; continue; }
                pc = 0x800d15e8;
            }
            0x800d15e8 => {
                { let ea = c.r[30].wrapping_add(0x11c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = 0x801c0000;
                c.r[6] = c.r[3].wrapping_add(0x27c);
                c.r[4] = 0x10000;
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                { let ea = c.r[2].wrapping_add(0xffff9318); c.f[1] = c.lfd(ea); }
                c.r[4] = 0x2;
                c.r[7] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d1614; f_801c028c(c);
                pc = 0x800d1614;
            }
            0x800d1614 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                { let ea = c.r[30].wrapping_add(0x11c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x800d1620; f_80196698(c);
                pc = 0x800d1620;
            }
            0x800d1620 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.lr = 0x800d1628; f_801966fc(c);
                pc = 0x800d1628;
            }
            0x800d1628 => {
                c.r[3] = 0x801c0000;
                c.r[4] = 0x10000;
                c.r[6] = c.r[3].wrapping_add(0x27c);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                { let ea = c.r[30].wrapping_add(0x118); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = 0x2;
                c.r[7] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d1650; f_801c028c(c);
                pc = 0x800d1650;
            }
            0x800d1650 => {
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                { let ea = c.r[1].wrapping_add(0x20); c.f[31] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_800d13c8: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D1674 (0x800d1674, 184 bytes)
#[inline(never)]
pub fn f_800d1674(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1674) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1674); f_800d1674_b(c, 0x800d1674); crate::prof::exit(c, 0x800d1674); return; }
    }
    f_800d1674_b(c, 0x800d1674)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1674_at(c: &mut Cpu, pc: u32) { f_800d1674_b(c, pc) }
#[inline(always)]
fn f_800d1674_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1674;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1674, pc); }
        match pc {
            0x800d1674 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[30].wrapping_add(0x70);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x4));
                c.r[3] = c.r[31].wrapping_add(0x4);
                c.stb(c.r[31], c.r[0]);
                c.lr = 0x800d16a4; f_800e01d0(c);
                pc = 0x800d16a4;
            }
            0x800d16a4 => {
                c.r[3] = c.r[31].wrapping_add(0x10);
                c.r[4] = c.r[30].wrapping_add(0x7c);
                c.lr = 0x800d16b0; f_800e01d0(c);
                pc = 0x800d16b0;
            }
            0x800d16b0 => {
                c.r[3] = c.r[31].wrapping_add(0x1c);
                c.r[4] = c.r[30].wrapping_add(0x88);
                c.lr = 0x800d16bc; f_800e01d0(c);
                pc = 0x800d16bc;
            }
            0x800d16bc => {
                c.r[3] = c.r[31].wrapping_add(0x28);
                c.r[4] = c.r[30].wrapping_add(0x94);
                c.lr = 0x800d16c8; f_800e0628(c);
                pc = 0x800d16c8;
            }
            0x800d16c8 => {
                c.r[3] = c.r[31].wrapping_add(0x58);
                c.r[4] = c.r[30].wrapping_add(0xc4);
                c.lr = 0x800d16d4; f_800e0628(c);
                pc = 0x800d16d4;
            }
            0x800d16d4 => {
                c.r[3] = c.r[31].wrapping_add(0x88);
                c.r[4] = c.r[30].wrapping_add(0xf4);
                c.lr = 0x800d16e0; f_800e01d0(c);
                pc = 0x800d16e0;
            }
            0x800d16e0 => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[4] = c.r[30].wrapping_add(0x100);
                c.lr = 0x800d16ec; f_800e01d0(c);
                pc = 0x800d16ec;
            }
            0x800d16ec => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x10c));
                c.stw(c.r[31].wrapping_add(0xa0), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x114));
                c.stw(c.r[31].wrapping_add(0xa4), c.r[0]);
                { let ea = c.r[30].wrapping_add(0x118); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0xa8); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x11c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0xac); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x124));
                c.stb(c.r[31].wrapping_add(0x1), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1674: bad pc {:#x}", pc),
        }
    }
}

/// GScameraGetAnimFrame (0x800d172c, 8 bytes)
#[inline(never)]
pub fn f_800d172c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d172c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d172c); f_800d172c_b(c, 0x800d172c); crate::prof::exit(c, 0x800d172c); return; }
    }
    f_800d172c_b(c, 0x800d172c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d172c_at(c: &mut Cpu, pc: u32) { f_800d172c_b(c, pc) }
#[inline(always)]
fn f_800d172c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d172c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d172c, pc); }
        match pc {
            0x800d172c => {
                { let ea = c.r[3].wrapping_add(0x11c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { return; }
                return;
            }
            _ => panic!("f_800d172c: bad pc {:#x}", pc),
        }
    }
}

/// GScameraHasAnimationEnded (0x800d1734, 8 bytes)
#[inline(never)]
pub fn f_800d1734(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1734) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1734); f_800d1734_b(c, 0x800d1734); crate::prof::exit(c, 0x800d1734); return; }
    }
    f_800d1734_b(c, 0x800d1734)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1734_at(c: &mut Cpu, pc: u32) { f_800d1734_b(c, pc) }
#[inline(always)]
fn f_800d1734_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1734;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1734, pc); }
        match pc {
            0x800d1734 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x124));
                if true { return; }
                return;
            }
            _ => panic!("f_800d1734: bad pc {:#x}", pc),
        }
    }
}

/// GScameraStopAnimation (0x800d173c, 92 bytes)
#[inline(never)]
pub fn f_800d173c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d173c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d173c); f_800d173c_b(c, 0x800d173c); crate::prof::exit(c, 0x800d173c); return; }
    }
    f_800d173c_b(c, 0x800d173c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d173c_at(c: &mut Cpu, pc: u32) { f_800d173c_b(c, pc) }
#[inline(always)]
fn f_800d173c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d173c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d173c, pc); }
        match pc {
            0x800d173c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.r[31].wrapping_add(0x70);
                c.stb(c.r[3].wrapping_add(0x4), c.r[0]);
                c.r[0] = 0x1;
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.lr = 0x800d176c; f_80195904(c);
                pc = 0x800d176c;
            }
            0x800d176c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0xf4);
                c.lr = 0x800d1778; f_801950d0(c);
                pc = 0x800d1778;
            }
            0x800d1778 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800d1784; f_801959dc(c);
                pc = 0x800d1784;
            }
            0x800d1784 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d173c: bad pc {:#x}", pc),
        }
    }
}

/// GScameraStartAnimation (0x800d1798, 192 bytes)
#[inline(never)]
pub fn f_800d1798(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1798) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1798); f_800d1798_b(c, 0x800d1798); crate::prof::exit(c, 0x800d1798); return; }
    }
    f_800d1798_b(c, 0x800d1798)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1798_at(c: &mut Cpu, pc: u32) { f_800d1798_b(c, pc) }
#[inline(always)]
fn f_800d1798_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1798;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1798, pc); }
        match pc {
            0x800d1798 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x3));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1844; continue; }
                pc = 0x800d17b8;
            }
            0x800d17b8 => {
                c.r[3] = 0x1;
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x4), c.r[3]);
                c.stb(c.r[31].wrapping_add(0x124), c.r[0]);
                c.stb(c.r[31].wrapping_add(0x125), c.r[3]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x3));
                { let ea = c.r[31].wrapping_add(0x11c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1844; continue; }
                pc = 0x800d17dc;
            }
            0x800d17dc => {
                { let ea = c.r[31].wrapping_add(0x11c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = 0x801c0000;
                c.r[6] = c.r[3].wrapping_add(0x27c);
                c.r[4] = 0x10000;
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                { let ea = c.r[2].wrapping_add(0xffff9318); c.f[1] = c.lfd(ea); }
                c.r[4] = 0x2;
                c.r[7] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d1808; f_801c028c(c);
                pc = 0x800d1808;
            }
            0x800d1808 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                { let ea = c.r[31].wrapping_add(0x11c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x800d1814; f_80196698(c);
                pc = 0x800d1814;
            }
            0x800d1814 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.lr = 0x800d181c; f_801966fc(c);
                pc = 0x800d181c;
            }
            0x800d181c => {
                c.r[3] = 0x801c0000;
                c.r[4] = 0x10000;
                c.r[6] = c.r[3].wrapping_add(0x27c);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                { let ea = c.r[31].wrapping_add(0x118); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = 0x2;
                c.r[7] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d1844; f_801c028c(c);
                pc = 0x800d1844;
            }
            0x800d1844 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1798: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D1858 (0x800d1858, 8 bytes)
#[inline(never)]
pub fn f_800d1858(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1858) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1858); f_800d1858_b(c, 0x800d1858); crate::prof::exit(c, 0x800d1858); return; }
    }
    f_800d1858_b(c, 0x800d1858)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1858_at(c: &mut Cpu, pc: u32) { f_800d1858_b(c, pc) }
#[inline(always)]
fn f_800d1858_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1858;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1858, pc); }
        match pc {
            0x800d1858 => {
                c.stw(c.r[3].wrapping_add(0x10c), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1858: bad pc {:#x}", pc),
        }
    }
}

/// GScameraSetAnimFrame (0x800d1860, 156 bytes)
#[inline(never)]
pub fn f_800d1860(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1860) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1860); f_800d1860_b(c, 0x800d1860); crate::prof::exit(c, 0x800d1860); return; }
    }
    f_800d1860_b(c, 0x800d1860)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1860_at(c: &mut Cpu, pc: u32) { f_800d1860_b(c, pc) }
#[inline(always)]
fn f_800d1860_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1860;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1860, pc); }
        match pc {
            0x800d1860 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x3));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d18e8; continue; }
                pc = 0x800d1880;
            }
            0x800d1880 => {
                { let ea = c.r[31].wrapping_add(0x11c); let v = c.f[1]; c.stfs(ea, v); }
                c.r[3] = 0x801c0000;
                c.r[6] = c.r[3].wrapping_add(0x27c);
                c.r[4] = 0x10000;
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                { let ea = c.r[2].wrapping_add(0xffff9318); c.f[1] = c.lfd(ea); }
                c.r[4] = 0x2;
                c.r[7] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d18ac; f_801c028c(c);
                pc = 0x800d18ac;
            }
            0x800d18ac => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                { let ea = c.r[31].wrapping_add(0x11c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x800d18b8; f_80196698(c);
                pc = 0x800d18b8;
            }
            0x800d18b8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.lr = 0x800d18c0; f_801966fc(c);
                pc = 0x800d18c0;
            }
            0x800d18c0 => {
                c.r[3] = 0x801c0000;
                c.r[4] = 0x10000;
                c.r[6] = c.r[3].wrapping_add(0x27c);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                { let ea = c.r[31].wrapping_add(0x118); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = 0x2;
                c.r[7] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d18e8; f_801c028c(c);
                pc = 0x800d18e8;
            }
            0x800d18e8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1860: bad pc {:#x}", pc),
        }
    }
}

/// GScameraSetAnimRate (0x800d18fc, 136 bytes)
#[inline(never)]
pub fn f_800d18fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d18fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800d18fc); f_800d18fc_b(c, 0x800d18fc); crate::prof::exit(c, 0x800d18fc); return; }
    }
    f_800d18fc_b(c, 0x800d18fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d18fc_at(c: &mut Cpu, pc: u32) { f_800d18fc_b(c, pc) }
#[inline(always)]
fn f_800d18fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d18fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d18fc, pc); }
        match pc {
            0x800d18fc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x18); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.f[31] = c.f[1];
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x3));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1968; continue; }
                pc = 0x800d1928;
            }
            0x800d1928 => {
                c.lr = 0x800d192c; f_800d37cc(c);
                pc = 0x800d192c;
            }
            0x800d192c => {
                c.cmps(0, c.r[3], 0x32);
                if !c.crbit(2) { pc = 0x800d193c; continue; }
                pc = 0x800d1934;
            }
            0x800d1934 => {
                { let ea = c.r[2].wrapping_add(0xffff9310); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[31] * c.f[0]); c.f[31] = v; c.ps1[31] = v; }
                pc = 0x800d193c;
            }
            0x800d193c => {
                { let ea = c.r[31].wrapping_add(0x118); let v = c.f[31]; c.stfs(ea, v); }
                c.r[3] = 0x801c0000;
                c.r[6] = c.r[3].wrapping_add(0x27c);
                c.r[4] = 0x10000;
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[5] = c.r[4].wrapping_add(0xffffffff);
                { let ea = c.r[31].wrapping_add(0x118); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = 0x2;
                c.r[7] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d1968; f_801c028c(c);
                pc = 0x800d1968;
            }
            0x800d1968 => {
                { let ea = c.r[1].wrapping_add(0x18); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                { let ea = c.r[1].wrapping_add(0x10); c.f[31] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d18fc: bad pc {:#x}", pc),
        }
    }
}

/// GScameraSetAnimIndex (0x800d1984, 180 bytes)
#[inline(never)]
pub fn f_800d1984(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1984) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1984); f_800d1984_b(c, 0x800d1984); crate::prof::exit(c, 0x800d1984); return; }
    }
    f_800d1984_b(c, 0x800d1984)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1984_at(c: &mut Cpu, pc: u32) { f_800d1984_b(c, pc) }
#[inline(always)]
fn f_800d1984_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1984;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1984, pc); }
        match pc {
            0x800d1984 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x3));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1a20; continue; }
                pc = 0x800d19ac;
            }
            0x800d19ac => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.lr = 0x800d19b4; f_80196bb8(c);
                pc = 0x800d19b4;
            }
            0x800d19b4 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x110));
                c.cmpu(0, c.r[31], c.r[0]);
                if c.crbit(1) { pc = 0x800d1a20; continue; }
                pc = 0x800d19c0;
            }
            0x800d19c0 => {
                c.stw(c.r[30].wrapping_add(0x114), c.r[31]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x8));
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x114));
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                c.lr = 0x800d19e0; f_80196b10(c);
                pc = 0x800d19e0;
            }
            0x800d19e0 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x800d19ec; f_80196698(c);
                pc = 0x800d19ec;
            }
            0x800d19ec => {
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = 0x800d0000;
                c.r[6] = c.r[3].wrapping_add(0x2b44);
                c.r[4] = 0x2;
                { let ea = c.r[13].wrapping_add(0xffffa258); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = 0x10000;
                c.r[5] = c.r[3].wrapping_add(0xffffffff);
                c.r[7] = 0x0;
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d1a18; f_801c028c(c);
                pc = 0x800d1a18;
            }
            0x800d1a18 => {
                { let ea = c.r[13].wrapping_add(0xffffa258); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[30].wrapping_add(0x120); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d1a20;
            }
            0x800d1a20 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1984: bad pc {:#x}", pc),
        }
    }
}

/// GScameraIsAnimating (0x800d1a38, 8 bytes)
#[inline(never)]
pub fn f_800d1a38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1a38) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1a38); f_800d1a38_b(c, 0x800d1a38); crate::prof::exit(c, 0x800d1a38); return; }
    }
    f_800d1a38_b(c, 0x800d1a38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1a38_at(c: &mut Cpu, pc: u32) { f_800d1a38_b(c, pc) }
#[inline(always)]
fn f_800d1a38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1a38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1a38, pc); }
        match pc {
            0x800d1a38 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_800d1a38: bad pc {:#x}", pc),
        }
    }
}

/// GScameraGetDistanceVector (0x800d1a40, 48 bytes)
#[inline(never)]
pub fn f_800d1a40(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1a40) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1a40); f_800d1a40_b(c, 0x800d1a40); crate::prof::exit(c, 0x800d1a40); return; }
    }
    f_800d1a40_b(c, 0x800d1a40)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1a40_at(c: &mut Cpu, pc: u32) { f_800d1a40_b(c, pc) }
#[inline(always)]
fn f_800d1a40_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1a40;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1a40, pc); }
        match pc {
            0x800d1a40 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.r[5].wrapping_add(0x70);
                c.r[5] = c.r[5].wrapping_add(0x100);
                c.lr = 0x800d1a60; f_800e0168(c);
                pc = 0x800d1a60;
            }
            0x800d1a60 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1a40: bad pc {:#x}", pc),
        }
    }
}

/// GScameraGetProjMatrixPtr (0x800d1a70, 204 bytes)
#[inline(never)]
pub fn f_800d1a70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1a70) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1a70); f_800d1a70_b(c, 0x800d1a70); crate::prof::exit(c, 0x800d1a70); return; }
    }
    f_800d1a70_b(c, 0x800d1a70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1a70_at(c: &mut Cpu, pc: u32) { f_800d1a70_b(c, pc) }
#[inline(always)]
fn f_800d1a70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1a70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1a70, pc); }
        match pc {
            0x800d1a70 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x50));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d1ae0; continue; }
                pc = 0x800d1a9c;
            }
            0x800d1a9c => {
                c.r[4] = c.r[1].wrapping_add(0x1c);
                c.r[5] = c.r[1].wrapping_add(0x18);
                c.lr = 0x800d1aa8; f_8019431c(c);
                pc = 0x800d1aa8;
            }
            0x800d1aa8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.lr = 0x800d1ab0; f_801944f8(c);
                pc = 0x800d1ab0;
            }
            0x800d1ab0 => {
                c.f[31] = c.f[1];
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.lr = 0x800d1abc; f_801944d0(c);
                pc = 0x800d1abc;
            }
            0x800d1abc => {
                c.f[0] = c.f[1];
                c.r[3] = 0x80400000;
                c.f[3] = c.f[31];
                { let ea = c.r[1].wrapping_add(0x1c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                c.f[4] = c.f[0];
                c.lr = 0x800d1adc; f_800e0678(c);
                pc = 0x800d1adc;
            }
            0x800d1adc => {
                pc = 0x800d1b18; continue;
                pc = 0x800d1ae0;
            }
            0x800d1ae0 => {
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = c.r[1].wrapping_add(0x14);
                c.r[7] = c.r[1].wrapping_add(0xc);
                c.lr = 0x800d1af4; f_801942c0(c);
                pc = 0x800d1af4;
            }
            0x800d1af4 => {
                c.r[3] = 0x80400000;
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                { let ea = c.r[1].wrapping_add(0x14); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[2].wrapping_add(0xffff92f4); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let ea = c.r[2].wrapping_add(0xffff9320); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                c.lr = 0x800d1b18; f_800e0698(c);
                pc = 0x800d1b18;
            }
            0x800d1b18 => {
                c.r[3] = 0x80400000;
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                { let ea = c.r[1].wrapping_add(0x30); c.f[31] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1a70: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D1B3C (0x800d1b3c, 452 bytes)
#[inline(never)]
pub fn f_800d1b3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1b3c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1b3c); f_800d1b3c_b(c, 0x800d1b3c); crate::prof::exit(c, 0x800d1b3c); return; }
    }
    f_800d1b3c_b(c, 0x800d1b3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1b3c_at(c: &mut Cpu, pc: u32) { f_800d1b3c_b(c, pc) }
#[inline(always)]
fn f_800d1b3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1b3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1b3c, pc); }
        match pc {
            0x800d1b3c => {
                { let ea = c.r[1].wrapping_add(0xffffff50); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xb4), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xac), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x2));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1cdc; continue; }
                pc = 0x800d1b5c;
            }
            0x800d1b5c => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x4));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1ba4; continue; }
                pc = 0x800d1b68;
            }
            0x800d1b68 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.lr = 0x800d1b70; f_80194cc4(c);
                pc = 0x800d1b70;
            }
            0x800d1b70 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.lr = 0x800d1b7c; f_800e0628(c);
                pc = 0x800d1b7c;
            }
            0x800d1b7c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0x70);
                c.lr = 0x800d1b88; f_80195904(c);
                pc = 0x800d1b88;
            }
            0x800d1b88 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0xf4);
                c.lr = 0x800d1b94; f_801950d0(c);
                pc = 0x800d1b94;
            }
            0x800d1b94 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800d1ba0; f_801959dc(c);
                pc = 0x800d1ba0;
            }
            0x800d1ba0 => {
                pc = 0x800d1cd4; continue;
                pc = 0x800d1ba4;
            }
            0x800d1ba4 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d1c54; continue; }
                pc = 0x800d1bb0;
            }
            0x800d1bb0 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[31].wrapping_add(0x70);
                c.r[5] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800d1bc0; f_800e0168(c);
                pc = 0x800d1bc0;
            }
            0x800d1bc0 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d1bd4; continue; }
                pc = 0x800d1bd0;
            }
            0x800d1bd0 => {
                pc = 0x800d1bd8; continue;
                pc = 0x800d1bd4;
            }
            0x800d1bd4 => {
                c.f[1] = -c.f[1];
                pc = 0x800d1bd8;
            }
            0x800d1bd8 => {
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8acc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d1c3c; continue; }
                pc = 0x800d1be8;
            }
            0x800d1be8 => {
                { let ea = c.r[1].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d1bfc; continue; }
                pc = 0x800d1bf8;
            }
            0x800d1bf8 => {
                pc = 0x800d1c00; continue;
                pc = 0x800d1bfc;
            }
            0x800d1bfc => {
                c.f[1] = -c.f[1];
                pc = 0x800d1c00;
            }
            0x800d1c00 => {
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d1c3c; continue; }
                pc = 0x800d1c08;
            }
            0x800d1c08 => {
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d1c1c; continue; }
                pc = 0x800d1c18;
            }
            0x800d1c18 => {
                pc = 0x800d1c20; continue;
                pc = 0x800d1c1c;
            }
            0x800d1c1c => {
                c.f[1] = -c.f[1];
                pc = 0x800d1c20;
            }
            0x800d1c20 => {
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d1c3c; continue; }
                pc = 0x800d1c28;
            }
            0x800d1c28 => {
                { let ea = c.r[31].wrapping_add(0x100); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9300); c.f[0] = c.lfd(ea); }
                c.f[0] = c.f[1] + c.f[0];
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31].wrapping_add(0x100); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d1c3c;
            }
            0x800d1c3c => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[4] = c.r[31].wrapping_add(0x70);
                c.r[5] = c.r[31].wrapping_add(0xf4);
                c.r[6] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800d1c50; f_800e0218(c);
                pc = 0x800d1c50;
            }
            0x800d1c50 => {
                pc = 0x800d1cd4; continue;
                pc = 0x800d1c54;
            }
            0x800d1c54 => {
                { let ea = c.r[31].wrapping_add(0x88); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.f[1] = -c.f[0];
                c.lr = 0x800d1c64; f_800e053c(c);
                pc = 0x800d1c64;
            }
            0x800d1c64 => {
                { let ea = c.r[31].wrapping_add(0x8c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x44);
                c.f[1] = -c.f[0];
                c.lr = 0x800d1c74; f_800e0518(c);
                pc = 0x800d1c74;
            }
            0x800d1c74 => {
                { let ea = c.r[31].wrapping_add(0x90); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x74);
                c.f[1] = -c.f[0];
                c.lr = 0x800d1c84; f_800e04f4(c);
                pc = 0x800d1c84;
            }
            0x800d1c84 => {
                { let ea = c.r[31].wrapping_add(0x70); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x14);
                { let ea = c.r[31].wrapping_add(0x74); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[31].wrapping_add(0x78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[1] = -c.f[1];
                c.f[2] = -c.f[2];
                c.f[3] = -c.f[0];
                c.lr = 0x800d1ca4; f_800e05c0(c);
                pc = 0x800d1ca4;
            }
            0x800d1ca4 => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x44);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d1cb4; f_800e0290(c);
                pc = 0x800d1cb4;
            }
            0x800d1cb4 => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x74);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d1cc4; f_800e0290(c);
                pc = 0x800d1cc4;
            }
            0x800d1cc4 => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x14);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d1cd4; f_800e0290(c);
                pc = 0x800d1cd4;
            }
            0x800d1cd4 => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x2), c.r[0]);
                pc = 0x800d1cdc;
            }
            0x800d1cdc => {
                c.r[3] = c.r[31].wrapping_add(0xc4);
                c.r[4] = c.r[31].wrapping_add(0x94);
                c.lr = 0x800d1ce8; f_800e0264(c);
                pc = 0x800d1ce8;
            }
            0x800d1ce8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xb4));
                c.r[3] = c.r[31].wrapping_add(0xc4);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xac));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xb0);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1b3c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D1D00 (0x800d1d00, 440 bytes)
#[inline(never)]
pub fn f_800d1d00(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1d00) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1d00); f_800d1d00_b(c, 0x800d1d00); crate::prof::exit(c, 0x800d1d00); return; }
    }
    f_800d1d00_b(c, 0x800d1d00)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1d00_at(c: &mut Cpu, pc: u32) { f_800d1d00_b(c, pc) }
#[inline(always)]
fn f_800d1d00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1d00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1d00, pc); }
        match pc {
            0x800d1d00 => {
                { let ea = c.r[1].wrapping_add(0xffffff50); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xb4), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xac), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x2));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1ea0; continue; }
                pc = 0x800d1d20;
            }
            0x800d1d20 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x4));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1d68; continue; }
                pc = 0x800d1d2c;
            }
            0x800d1d2c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.lr = 0x800d1d34; f_80194cc4(c);
                pc = 0x800d1d34;
            }
            0x800d1d34 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.lr = 0x800d1d40; f_800e0628(c);
                pc = 0x800d1d40;
            }
            0x800d1d40 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0x70);
                c.lr = 0x800d1d4c; f_80195904(c);
                pc = 0x800d1d4c;
            }
            0x800d1d4c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0xf4);
                c.lr = 0x800d1d58; f_801950d0(c);
                pc = 0x800d1d58;
            }
            0x800d1d58 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800d1d64; f_801959dc(c);
                pc = 0x800d1d64;
            }
            0x800d1d64 => {
                pc = 0x800d1e98; continue;
                pc = 0x800d1d68;
            }
            0x800d1d68 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d1e18; continue; }
                pc = 0x800d1d74;
            }
            0x800d1d74 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[31].wrapping_add(0x70);
                c.r[5] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800d1d84; f_800e0168(c);
                pc = 0x800d1d84;
            }
            0x800d1d84 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d1d98; continue; }
                pc = 0x800d1d94;
            }
            0x800d1d94 => {
                pc = 0x800d1d9c; continue;
                pc = 0x800d1d98;
            }
            0x800d1d98 => {
                c.f[1] = -c.f[1];
                pc = 0x800d1d9c;
            }
            0x800d1d9c => {
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8acc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d1e00; continue; }
                pc = 0x800d1dac;
            }
            0x800d1dac => {
                { let ea = c.r[1].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d1dc0; continue; }
                pc = 0x800d1dbc;
            }
            0x800d1dbc => {
                pc = 0x800d1dc4; continue;
                pc = 0x800d1dc0;
            }
            0x800d1dc0 => {
                c.f[1] = -c.f[1];
                pc = 0x800d1dc4;
            }
            0x800d1dc4 => {
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d1e00; continue; }
                pc = 0x800d1dcc;
            }
            0x800d1dcc => {
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d1de0; continue; }
                pc = 0x800d1ddc;
            }
            0x800d1ddc => {
                pc = 0x800d1de4; continue;
                pc = 0x800d1de0;
            }
            0x800d1de0 => {
                c.f[1] = -c.f[1];
                pc = 0x800d1de4;
            }
            0x800d1de4 => {
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d1e00; continue; }
                pc = 0x800d1dec;
            }
            0x800d1dec => {
                { let ea = c.r[31].wrapping_add(0x100); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9300); c.f[0] = c.lfd(ea); }
                c.f[0] = c.f[1] + c.f[0];
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31].wrapping_add(0x100); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d1e00;
            }
            0x800d1e00 => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[4] = c.r[31].wrapping_add(0x70);
                c.r[5] = c.r[31].wrapping_add(0xf4);
                c.r[6] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800d1e14; f_800e0218(c);
                pc = 0x800d1e14;
            }
            0x800d1e14 => {
                pc = 0x800d1e98; continue;
                pc = 0x800d1e18;
            }
            0x800d1e18 => {
                { let ea = c.r[31].wrapping_add(0x88); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.f[1] = -c.f[0];
                c.lr = 0x800d1e28; f_800e053c(c);
                pc = 0x800d1e28;
            }
            0x800d1e28 => {
                { let ea = c.r[31].wrapping_add(0x8c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x44);
                c.f[1] = -c.f[0];
                c.lr = 0x800d1e38; f_800e0518(c);
                pc = 0x800d1e38;
            }
            0x800d1e38 => {
                { let ea = c.r[31].wrapping_add(0x90); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x74);
                c.f[1] = -c.f[0];
                c.lr = 0x800d1e48; f_800e04f4(c);
                pc = 0x800d1e48;
            }
            0x800d1e48 => {
                { let ea = c.r[31].wrapping_add(0x70); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x14);
                { let ea = c.r[31].wrapping_add(0x74); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[31].wrapping_add(0x78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[1] = -c.f[1];
                c.f[2] = -c.f[2];
                c.f[3] = -c.f[0];
                c.lr = 0x800d1e68; f_800e05c0(c);
                pc = 0x800d1e68;
            }
            0x800d1e68 => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x44);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d1e78; f_800e0290(c);
                pc = 0x800d1e78;
            }
            0x800d1e78 => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x74);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d1e88; f_800e0290(c);
                pc = 0x800d1e88;
            }
            0x800d1e88 => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x14);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d1e98; f_800e0290(c);
                pc = 0x800d1e98;
            }
            0x800d1e98 => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x2), c.r[0]);
                pc = 0x800d1ea0;
            }
            0x800d1ea0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xb4));
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xac));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xb0);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1d00: bad pc {:#x}", pc),
        }
    }
}

/// GScameraGetLookAt (0x800d1eb8, 76 bytes)
#[inline(never)]
pub fn f_800d1eb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1eb8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1eb8); f_800d1eb8_b(c, 0x800d1eb8); crate::prof::exit(c, 0x800d1eb8); return; }
    }
    f_800d1eb8_b(c, 0x800d1eb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1eb8_at(c: &mut Cpu, pc: u32) { f_800d1eb8_b(c, pc) }
#[inline(always)]
fn f_800d1eb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1eb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1eb8, pc); }
        match pc {
            0x800d1eb8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = c.r[30].wrapping_add(0xf4);
                c.lr = 0x800d1ee0; f_800e01d0(c);
                pc = 0x800d1ee0;
            }
            0x800d1ee0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30].wrapping_add(0x100);
                c.lr = 0x800d1eec; f_800e01d0(c);
                pc = 0x800d1eec;
            }
            0x800d1eec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1eb8: bad pc {:#x}", pc),
        }
    }
}

/// GScameraLookAt (0x800d1f04, 84 bytes)
#[inline(never)]
pub fn f_800d1f04(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1f04) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1f04); f_800d1f04_b(c, 0x800d1f04); crate::prof::exit(c, 0x800d1f04); return; }
    }
    f_800d1f04_b(c, 0x800d1f04)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1f04_at(c: &mut Cpu, pc: u32) { f_800d1f04_b(c, pc) }
#[inline(always)]
fn f_800d1f04_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1f04;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1f04, pc); }
        match pc {
            0x800d1f04 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[30].wrapping_add(0xf4);
                c.lr = 0x800d1f28; f_800e01d0(c);
                pc = 0x800d1f28;
            }
            0x800d1f28 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[30].wrapping_add(0x100);
                c.lr = 0x800d1f34; f_800e01d0(c);
                pc = 0x800d1f34;
            }
            0x800d1f34 => {
                c.r[0] = 0x1;
                c.stb(c.r[30].wrapping_add(0x2), c.r[0]);
                c.stb(c.r[30].wrapping_add(0x1), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1f04: bad pc {:#x}", pc),
        }
    }
}

/// GScameraGetRotation (0x800d1f58, 44 bytes)
#[inline(never)]
pub fn f_800d1f58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1f58) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1f58); f_800d1f58_b(c, 0x800d1f58); crate::prof::exit(c, 0x800d1f58); return; }
    }
    f_800d1f58_b(c, 0x800d1f58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1f58_at(c: &mut Cpu, pc: u32) { f_800d1f58_b(c, pc) }
#[inline(always)]
fn f_800d1f58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1f58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1f58, pc); }
        match pc {
            0x800d1f58 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.r[5].wrapping_add(0x88);
                c.lr = 0x800d1f74; f_800e01d0(c);
                pc = 0x800d1f74;
            }
            0x800d1f74 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1f58: bad pc {:#x}", pc),
        }
    }
}

/// GScameraGetPosition (0x800d1f84, 88 bytes)
#[inline(never)]
pub fn f_800d1f84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1f84) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1f84); f_800d1f84_b(c, 0x800d1f84); crate::prof::exit(c, 0x800d1f84); return; }
    }
    f_800d1f84_b(c, 0x800d1f84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1f84_at(c: &mut Cpu, pc: u32) { f_800d1f84_b(c, pc) }
#[inline(always)]
fn f_800d1f84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1f84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1f84, pc); }
        match pc {
            0x800d1f84 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x4));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d1fb8; continue; }
                pc = 0x800d1fac;
            }
            0x800d1fac => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.r[30].wrapping_add(0x70);
                c.lr = 0x800d1fb8; f_80195904(c);
                pc = 0x800d1fb8;
            }
            0x800d1fb8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30].wrapping_add(0x70);
                c.lr = 0x800d1fc4; f_800e01d0(c);
                pc = 0x800d1fc4;
            }
            0x800d1fc4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1f84: bad pc {:#x}", pc),
        }
    }
}

/// GScameraGetPerspective (0x800d1fdc, 96 bytes)
#[inline(never)]
pub fn f_800d1fdc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d1fdc) { return; }
        if c.prof { crate::prof::enter(c, 0x800d1fdc); f_800d1fdc_b(c, 0x800d1fdc); crate::prof::exit(c, 0x800d1fdc); return; }
    }
    f_800d1fdc_b(c, 0x800d1fdc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d1fdc_at(c: &mut Cpu, pc: u32) { f_800d1fdc_b(c, pc) }
#[inline(always)]
fn f_800d1fdc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d1fdc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d1fdc, pc); }
        match pc {
            0x800d1fdc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[7] | c.r[7];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.lr = 0x800d2008; f_8019431c(c);
                pc = 0x800d2008;
            }
            0x800d2008 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.lr = 0x800d2010; f_801944f8(c);
                pc = 0x800d2010;
            }
            0x800d2010 => {
                { let ea = c.r[30]; let v = c.f[1]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                c.lr = 0x800d201c; f_801944d0(c);
                pc = 0x800d201c;
            }
            0x800d201c => {
                { let ea = c.r[31]; let v = c.f[1]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d1fdc: bad pc {:#x}", pc),
        }
    }
}

/// GScameraSetRotation (0x800d203c, 64 bytes)
#[inline(never)]
pub fn f_800d203c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d203c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d203c); f_800d203c_b(c, 0x800d203c); crate::prof::exit(c, 0x800d203c); return; }
    }
    f_800d203c_b(c, 0x800d203c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d203c_at(c: &mut Cpu, pc: u32) { f_800d203c_b(c, pc) }
#[inline(always)]
fn f_800d203c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d203c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d203c, pc); }
        match pc {
            0x800d203c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[31].wrapping_add(0x88);
                c.lr = 0x800d2058; f_800e01d0(c);
                pc = 0x800d2058;
            }
            0x800d2058 => {
                c.r[3] = 0x1;
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x2), c.r[3]);
                c.stb(c.r[31].wrapping_add(0x1), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d203c: bad pc {:#x}", pc),
        }
    }
}

/// GScameraSetPosition (0x800d207c, 80 bytes)
#[inline(never)]
pub fn f_800d207c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d207c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d207c); f_800d207c_b(c, 0x800d207c); crate::prof::exit(c, 0x800d207c); return; }
    }
    f_800d207c_b(c, 0x800d207c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d207c_at(c: &mut Cpu, pc: u32) { f_800d207c_b(c, pc) }
#[inline(always)]
fn f_800d207c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d207c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d207c, pc); }
        match pc {
            0x800d207c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.lr = 0x800d20a0; f_80195898(c);
                pc = 0x800d20a0;
            }
            0x800d20a0 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[30].wrapping_add(0x70);
                c.lr = 0x800d20ac; f_800e01d0(c);
                pc = 0x800d20ac;
            }
            0x800d20ac => {
                c.r[0] = 0x1;
                c.stb(c.r[30].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d207c: bad pc {:#x}", pc),
        }
    }
}

/// GScameraSetPerspective (0x800d20cc, 132 bytes)
#[inline(never)]
pub fn f_800d20cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d20cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800d20cc); f_800d20cc_b(c, 0x800d20cc); crate::prof::exit(c, 0x800d20cc); return; }
    }
    f_800d20cc_b(c, 0x800d20cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d20cc_at(c: &mut Cpu, pc: u32) { f_800d20cc_b(c, pc) }
#[inline(always)]
fn f_800d20cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d20cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d20cc, pc); }
        match pc {
            0x800d20cc => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.lr = 0x800d20e0; f_800c4744(c);
                pc = 0x800d20e0;
            }
            0x800d20e0 => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.f[28] = c.f[1];
                c.f[29] = c.f[2];
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.f[30] = c.f[3];
                c.r[4] = 0x1;
                c.f[31] = c.f[4];
                c.lr = 0x800d2104; f_801943bc(c);
                pc = 0x800d2104;
            }
            0x800d2104 => {
                c.f[1] = c.f[28];
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.f[2] = c.f[29];
                c.lr = 0x800d2114; f_801943a0(c);
                pc = 0x800d2114;
            }
            0x800d2114 => {
                c.f[1] = c.f[30];
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.lr = 0x800d2120; f_801944e8(c);
                pc = 0x800d2120;
            }
            0x800d2120 => {
                c.f[1] = c.f[31];
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.lr = 0x800d212c; f_801944c0(c);
                pc = 0x800d212c;
            }
            0x800d212c => {
                c.r[0] = 0x1;
                c.r[11] = c.r[1].wrapping_add(0x30);
                c.stb(c.r[31].wrapping_add(0x2), c.r[0]);
                c.lr = 0x800d213c; f_800c4790(c);
                pc = 0x800d213c;
            }
            0x800d213c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_800d20cc: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D2150 (0x800d2150, 120 bytes)
#[inline(never)]
pub fn f_800d2150(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d2150) { return; }
        if c.prof { crate::prof::enter(c, 0x800d2150); f_800d2150_b(c, 0x800d2150); crate::prof::exit(c, 0x800d2150); return; }
    }
    f_800d2150_b(c, 0x800d2150)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d2150_at(c: &mut Cpu, pc: u32) { f_800d2150_b(c, pc) }
#[inline(always)]
fn f_800d2150_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d2150;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d2150, pc); }
        match pc {
            0x800d2150 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x27e);
                if !c.crbit(1) { pc = 0x800d216c; continue; }
                pc = 0x800d2168;
            }
            0x800d2168 => {
                c.r[4] = 0x27e;
                pc = 0x800d216c;
            }
            0x800d216c => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1de);
                if !c.crbit(1) { pc = 0x800d217c; continue; }
                pc = 0x800d2178;
            }
            0x800d2178 => {
                c.r[5] = 0x1de;
                pc = 0x800d217c;
            }
            0x800d217c => {
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x27f);
                if !c.crbit(1) { pc = 0x800d218c; continue; }
                pc = 0x800d2188;
            }
            0x800d2188 => {
                c.r[6] = 0x27f;
                pc = 0x800d218c;
            }
            0x800d218c => {
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1df);
                if !c.crbit(1) { pc = 0x800d219c; continue; }
                pc = 0x800d2198;
            }
            0x800d2198 => {
                c.r[7] = 0x1df;
                pc = 0x800d219c;
            }
            0x800d219c => {
                c.r[8] = c.r[6].wrapping_add(0x1);
                c.r[0] = c.r[7].wrapping_add(0x1);
                c.r[6] = c.r[5] | c.r[5];
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.r[5] = c.r[8].rotate_left(0) & 0xffff;
                c.r[7] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x800d21b8; f_801944a4(c);
                pc = 0x800d21b8;
            }
            0x800d21b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d2150: bad pc {:#x}", pc),
        }
    }
}

/// GScameraSetViewport (0x800d21c8, 128 bytes)
#[inline(never)]
pub fn f_800d21c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d21c8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d21c8); f_800d21c8_b(c, 0x800d21c8); crate::prof::exit(c, 0x800d21c8); return; }
    }
    f_800d21c8_b(c, 0x800d21c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d21c8_at(c: &mut Cpu, pc: u32) { f_800d21c8_b(c, pc) }
#[inline(always)]
fn f_800d21c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d21c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d21c8, pc); }
        match pc {
            0x800d21c8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x27e);
                if !c.crbit(1) { pc = 0x800d21e4; continue; }
                pc = 0x800d21e0;
            }
            0x800d21e0 => {
                c.r[4] = 0x27e;
                pc = 0x800d21e4;
            }
            0x800d21e4 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1de);
                if !c.crbit(1) { pc = 0x800d21f4; continue; }
                pc = 0x800d21f0;
            }
            0x800d21f0 => {
                c.r[5] = 0x1de;
                pc = 0x800d21f4;
            }
            0x800d21f4 => {
                c.r[0] = c.r[6].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x27f);
                if !c.crbit(1) { pc = 0x800d2204; continue; }
                pc = 0x800d2200;
            }
            0x800d2200 => {
                c.r[6] = 0x27f;
                pc = 0x800d2204;
            }
            0x800d2204 => {
                c.r[0] = c.r[7].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1df);
                if !c.crbit(1) { pc = 0x800d2214; continue; }
                pc = 0x800d2210;
            }
            0x800d2210 => {
                c.r[7] = 0x1df;
                pc = 0x800d2214;
            }
            0x800d2214 => {
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.r[0] = c.r[7].wrapping_add(0x1);
                c.sth(c.r[1].wrapping_add(0x8), c.r[4]);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.sth(c.r[1].wrapping_add(0xc), c.r[5]);
                c.sth(c.r[1].wrapping_add(0xa), c.r[6]);
                c.sth(c.r[1].wrapping_add(0xe), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.lr = 0x800d2238; f_80194400(c);
                pc = 0x800d2238;
            }
            0x800d2238 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d21c8: bad pc {:#x}", pc),
        }
    }
}

/// _cameraLoadCameraMatrix__FP9_GScamera12GSgfxLayerID (0x800d2248, 828 bytes)
#[inline(never)]
pub fn f_800d2248(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d2248) { return; }
        if c.prof { crate::prof::enter(c, 0x800d2248); f_800d2248_b(c, 0x800d2248); crate::prof::exit(c, 0x800d2248); return; }
    }
    f_800d2248_b(c, 0x800d2248)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d2248_at(c: &mut Cpu, pc: u32) { f_800d2248_b(c, pc) }
#[inline(always)]
fn f_800d2248_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d2248;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d2248, pc); }
        match pc {
            0x800d2248 => {
                { let ea = c.r[1].wrapping_add(0xffffff40); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xc4), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xbc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0xb8), c.r[30]);
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa254));
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x800d256c; continue; }
                pc = 0x800d2268;
            }
            0x800d2268 => {
                c.r[4] = c.lbz(c.r[30].wrapping_add(0x4));
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800d2324; continue; }
                pc = 0x800d2274;
            }
            0x800d2274 => {
                c.r[31] = c.lwz(c.r[30].wrapping_add(0xc));
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x800d2290; continue; }
                pc = 0x800d2280;
            }
            0x800d2280 => {
                c.r[3] = c.r[2].wrapping_add(0xffff9324);
                c.r[4] = 0x1ae;
                c.r[5] = c.r[2].wrapping_add(0xffff932c);
                c.lr = 0x800d2290; f_80196e10(c);
                pc = 0x800d2290;
            }
            0x800d2290 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[5] = c.r[1].wrapping_add(0x10);
                c.r[0] = c.r[0].rotate_left(0) & 0xfffffffd;
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.lr = 0x800d22ac; f_8019431c(c);
                pc = 0x800d22ac;
            }
            0x800d22ac => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.lr = 0x800d22b4; f_801944f8(c);
                pc = 0x800d22b4;
            }
            0x800d22b4 => {
                { let ea = c.r[1].wrapping_add(0xc); let v = c.f[1]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.lr = 0x800d22c0; f_801944d0(c);
                pc = 0x800d22c0;
            }
            0x800d22c0 => {
                c.f[0] = c.f[1];
                { let ea = c.r[1].wrapping_add(0x14); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x800d22dc; f_800d9bd0(c);
                pc = 0x800d22dc;
            }
            0x800d22dc => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.lr = 0x800d22e4; f_80194cc4(c);
                pc = 0x800d22e4;
            }
            0x800d22e4 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.lr = 0x800d22f0; f_800e0628(c);
                pc = 0x800d22f0;
            }
            0x800d22f0 => {
                c.lr = 0x800d22f4; f_800d834c(c);
                pc = 0x800d22f4;
            }
            0x800d22f4 => {
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.lr = 0x800d22fc; f_800d7fe4(c);
                pc = 0x800d22fc;
            }
            0x800d22fc => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.r[30].wrapping_add(0x70);
                c.lr = 0x800d2308; f_80195904(c);
                pc = 0x800d2308;
            }
            0x800d2308 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.r[30].wrapping_add(0xf4);
                c.lr = 0x800d2314; f_801950d0(c);
                pc = 0x800d2314;
            }
            0x800d2314 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.r[30].wrapping_add(0x100);
                c.lr = 0x800d2320; f_801959dc(c);
                pc = 0x800d2320;
            }
            0x800d2320 => {
                pc = 0x800d256c; continue;
                pc = 0x800d2324;
            }
            0x800d2324 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x50));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d2540; continue; }
                pc = 0x800d2334;
            }
            0x800d2334 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x2));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d24b8; continue; }
                pc = 0x800d2340;
            }
            0x800d2340 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800d2380; continue; }
                pc = 0x800d2348;
            }
            0x800d2348 => {
                c.lr = 0x800d234c; f_80194cc4(c);
                pc = 0x800d234c;
            }
            0x800d234c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.lr = 0x800d2358; f_800e0628(c);
                pc = 0x800d2358;
            }
            0x800d2358 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.r[30].wrapping_add(0x70);
                c.lr = 0x800d2364; f_80195904(c);
                pc = 0x800d2364;
            }
            0x800d2364 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.r[30].wrapping_add(0xf4);
                c.lr = 0x800d2370; f_801950d0(c);
                pc = 0x800d2370;
            }
            0x800d2370 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.r[30].wrapping_add(0x100);
                c.lr = 0x800d237c; f_801959dc(c);
                pc = 0x800d237c;
            }
            0x800d237c => {
                pc = 0x800d24b0; continue;
                pc = 0x800d2380;
            }
            0x800d2380 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d2430; continue; }
                pc = 0x800d238c;
            }
            0x800d238c => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.r[4] = c.r[30].wrapping_add(0x70);
                c.r[5] = c.r[30].wrapping_add(0x100);
                c.lr = 0x800d239c; f_800e0168(c);
                pc = 0x800d239c;
            }
            0x800d239c => {
                { let ea = c.r[1].wrapping_add(0x18); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d23b0; continue; }
                pc = 0x800d23ac;
            }
            0x800d23ac => {
                pc = 0x800d23b4; continue;
                pc = 0x800d23b0;
            }
            0x800d23b0 => {
                c.f[1] = -c.f[1];
                pc = 0x800d23b4;
            }
            0x800d23b4 => {
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8acc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d2418; continue; }
                pc = 0x800d23c4;
            }
            0x800d23c4 => {
                { let ea = c.r[1].wrapping_add(0x1c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d23d8; continue; }
                pc = 0x800d23d4;
            }
            0x800d23d4 => {
                pc = 0x800d23dc; continue;
                pc = 0x800d23d8;
            }
            0x800d23d8 => {
                c.f[1] = -c.f[1];
                pc = 0x800d23dc;
            }
            0x800d23dc => {
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d2418; continue; }
                pc = 0x800d23e4;
            }
            0x800d23e4 => {
                { let ea = c.r[1].wrapping_add(0x20); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d23f8; continue; }
                pc = 0x800d23f4;
            }
            0x800d23f4 => {
                pc = 0x800d23fc; continue;
                pc = 0x800d23f8;
            }
            0x800d23f8 => {
                c.f[1] = -c.f[1];
                pc = 0x800d23fc;
            }
            0x800d23fc => {
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d2418; continue; }
                pc = 0x800d2404;
            }
            0x800d2404 => {
                { let ea = c.r[30].wrapping_add(0x100); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9300); c.f[0] = c.lfd(ea); }
                c.f[0] = c.f[1] + c.f[0];
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[30].wrapping_add(0x100); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d2418;
            }
            0x800d2418 => {
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.r[4] = c.r[30].wrapping_add(0x70);
                c.r[5] = c.r[30].wrapping_add(0xf4);
                c.r[6] = c.r[30].wrapping_add(0x100);
                c.lr = 0x800d242c; f_800e0218(c);
                pc = 0x800d242c;
            }
            0x800d242c => {
                pc = 0x800d24b0; continue;
                pc = 0x800d2430;
            }
            0x800d2430 => {
                { let ea = c.r[30].wrapping_add(0x88); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.f[1] = -c.f[0];
                c.lr = 0x800d2440; f_800e053c(c);
                pc = 0x800d2440;
            }
            0x800d2440 => {
                { let ea = c.r[30].wrapping_add(0x8c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x54);
                c.f[1] = -c.f[0];
                c.lr = 0x800d2450; f_800e0518(c);
                pc = 0x800d2450;
            }
            0x800d2450 => {
                { let ea = c.r[30].wrapping_add(0x90); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x84);
                c.f[1] = -c.f[0];
                c.lr = 0x800d2460; f_800e04f4(c);
                pc = 0x800d2460;
            }
            0x800d2460 => {
                { let ea = c.r[30].wrapping_add(0x70); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x24);
                { let ea = c.r[30].wrapping_add(0x74); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[30].wrapping_add(0x78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[1] = -c.f[1];
                c.f[2] = -c.f[2];
                c.f[3] = -c.f[0];
                c.lr = 0x800d2480; f_800e05c0(c);
                pc = 0x800d2480;
            }
            0x800d2480 => {
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x54);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d2490; f_800e0290(c);
                pc = 0x800d2490;
            }
            0x800d2490 => {
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x84);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d24a0; f_800e0290(c);
                pc = 0x800d24a0;
            }
            0x800d24a0 => {
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x24);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d24b0; f_800e0290(c);
                pc = 0x800d24b0;
            }
            0x800d24b0 => {
                c.r[0] = 0x0;
                c.stb(c.r[30].wrapping_add(0x2), c.r[0]);
                pc = 0x800d24b8;
            }
            0x800d24b8 => {
                c.r[31] = c.lwz(c.r[30].wrapping_add(0xc));
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x800d24d4; continue; }
                pc = 0x800d24c4;
            }
            0x800d24c4 => {
                c.r[3] = c.r[2].wrapping_add(0xffff9324);
                c.r[4] = 0x1a2;
                c.r[5] = c.r[2].wrapping_add(0xffff932c);
                c.lr = 0x800d24d4; f_80196e10(c);
                pc = 0x800d24d4;
            }
            0x800d24d4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.r[4] = c.r[31].wrapping_add(0x54);
                c.r[0] = c.r[0] | 0x80000000;
                c.r[0] = c.r[0] | 0x2;
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                c.lr = 0x800d24f0; f_800a2d64(c);
                pc = 0x800d24f0;
            }
            0x800d24f0 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[5] = c.r[1].wrapping_add(0x10);
                c.lr = 0x800d2500; f_8019431c(c);
                pc = 0x800d2500;
            }
            0x800d2500 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.lr = 0x800d2508; f_801944f8(c);
                pc = 0x800d2508;
            }
            0x800d2508 => {
                { let ea = c.r[1].wrapping_add(0xc); let v = c.f[1]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.lr = 0x800d2514; f_801944d0(c);
                pc = 0x800d2514;
            }
            0x800d2514 => {
                c.f[0] = c.f[1];
                { let ea = c.r[1].wrapping_add(0x14); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[0]); c.f[4] = v; c.ps1[4] = v; }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x800d2530; f_800d9bd0(c);
                pc = 0x800d2530;
            }
            0x800d2530 => {
                c.lr = 0x800d2534; f_800d834c(c);
                pc = 0x800d2534;
            }
            0x800d2534 => {
                c.r[3] = c.r[30].wrapping_add(0x94);
                c.lr = 0x800d253c; f_800d7fe4(c);
                pc = 0x800d253c;
            }
            0x800d253c => {
                pc = 0x800d256c; continue;
                pc = 0x800d2540;
            }
            0x800d2540 => {
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = c.r[1].wrapping_add(0x14);
                c.r[7] = c.r[1].wrapping_add(0xc);
                c.lr = 0x800d2554; f_801942c0(c);
                pc = 0x800d2554;
            }
            0x800d2554 => {
                { let ea = c.r[1].wrapping_add(0x14); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.lr = 0x800d2568; f_800d9b58(c);
                pc = 0x800d2568;
            }
            0x800d2568 => {
                c.lr = 0x800d256c; f_800d834c(c);
                pc = 0x800d256c;
            }
            0x800d256c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc4));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xbc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0xb8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xc0);
                if true { return; }
                return;
            }
            _ => panic!("f_800d2248: bad pc {:#x}", pc),
        }
    }
}

/// GScameraGetActiveCamera (0x800d2584, 8 bytes)
#[inline(never)]
pub fn f_800d2584(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d2584) { return; }
        if c.prof { crate::prof::enter(c, 0x800d2584); f_800d2584_b(c, 0x800d2584); crate::prof::exit(c, 0x800d2584); return; }
    }
    f_800d2584_b(c, 0x800d2584)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d2584_at(c: &mut Cpu, pc: u32) { f_800d2584_b(c, pc) }
#[inline(always)]
fn f_800d2584_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d2584;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d2584, pc); }
        match pc {
            0x800d2584 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa254));
                if true { return; }
                return;
            }
            _ => panic!("f_800d2584: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D258C (0x800d258c, 428 bytes)
#[inline(never)]
pub fn f_800d258c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d258c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d258c); f_800d258c_b(c, 0x800d258c); crate::prof::exit(c, 0x800d258c); return; }
    }
    f_800d258c_b(c, 0x800d258c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d258c_at(c: &mut Cpu, pc: u32) { f_800d258c_b(c, pc) }
#[inline(always)]
fn f_800d258c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d258c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d258c, pc); }
        match pc {
            0x800d258c => {
                { let ea = c.r[1].wrapping_add(0xffffff50); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xb4), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xac), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[13].wrapping_add(0xffffa254), c.r[31]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x4));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d25ec; continue; }
                pc = 0x800d25b0;
            }
            0x800d25b0 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.lr = 0x800d25b8; f_80194cc4(c);
                pc = 0x800d25b8;
            }
            0x800d25b8 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.lr = 0x800d25c4; f_800e0628(c);
                pc = 0x800d25c4;
            }
            0x800d25c4 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0x70);
                c.lr = 0x800d25d0; f_80195904(c);
                pc = 0x800d25d0;
            }
            0x800d25d0 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0xf4);
                c.lr = 0x800d25dc; f_801950d0(c);
                pc = 0x800d25dc;
            }
            0x800d25dc => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800d25e8; f_801959dc(c);
                pc = 0x800d25e8;
            }
            0x800d25e8 => {
                pc = 0x800d271c; continue;
                pc = 0x800d25ec;
            }
            0x800d25ec => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d269c; continue; }
                pc = 0x800d25f8;
            }
            0x800d25f8 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[31].wrapping_add(0x70);
                c.r[5] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800d2608; f_800e0168(c);
                pc = 0x800d2608;
            }
            0x800d2608 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d261c; continue; }
                pc = 0x800d2618;
            }
            0x800d2618 => {
                pc = 0x800d2620; continue;
                pc = 0x800d261c;
            }
            0x800d261c => {
                c.f[1] = -c.f[1];
                pc = 0x800d2620;
            }
            0x800d2620 => {
                c.r[3] = 0x80480000;
                { let ea = c.r[3].wrapping_add(0xffff8acc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d2684; continue; }
                pc = 0x800d2630;
            }
            0x800d2630 => {
                { let ea = c.r[1].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d2644; continue; }
                pc = 0x800d2640;
            }
            0x800d2640 => {
                pc = 0x800d2648; continue;
                pc = 0x800d2644;
            }
            0x800d2644 => {
                c.f[1] = -c.f[1];
                pc = 0x800d2648;
            }
            0x800d2648 => {
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d2684; continue; }
                pc = 0x800d2650;
            }
            0x800d2650 => {
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d2664; continue; }
                pc = 0x800d2660;
            }
            0x800d2660 => {
                pc = 0x800d2668; continue;
                pc = 0x800d2664;
            }
            0x800d2664 => {
                c.f[1] = -c.f[1];
                pc = 0x800d2668;
            }
            0x800d2668 => {
                c.fcmp(0, c.f[1], c.f[2]);
                if !c.crbit(0) { pc = 0x800d2684; continue; }
                pc = 0x800d2670;
            }
            0x800d2670 => {
                { let ea = c.r[31].wrapping_add(0x100); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9300); c.f[0] = c.lfd(ea); }
                c.f[0] = c.f[1] + c.f[0];
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[31].wrapping_add(0x100); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d2684;
            }
            0x800d2684 => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[4] = c.r[31].wrapping_add(0x70);
                c.r[5] = c.r[31].wrapping_add(0xf4);
                c.r[6] = c.r[31].wrapping_add(0x100);
                c.lr = 0x800d2698; f_800e0218(c);
                pc = 0x800d2698;
            }
            0x800d2698 => {
                pc = 0x800d271c; continue;
                pc = 0x800d269c;
            }
            0x800d269c => {
                { let ea = c.r[31].wrapping_add(0x88); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.f[1] = -c.f[0];
                c.lr = 0x800d26ac; f_800e053c(c);
                pc = 0x800d26ac;
            }
            0x800d26ac => {
                { let ea = c.r[31].wrapping_add(0x8c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x44);
                c.f[1] = -c.f[0];
                c.lr = 0x800d26bc; f_800e0518(c);
                pc = 0x800d26bc;
            }
            0x800d26bc => {
                { let ea = c.r[31].wrapping_add(0x90); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x74);
                c.f[1] = -c.f[0];
                c.lr = 0x800d26cc; f_800e04f4(c);
                pc = 0x800d26cc;
            }
            0x800d26cc => {
                { let ea = c.r[31].wrapping_add(0x70); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x14);
                { let ea = c.r[31].wrapping_add(0x74); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[31].wrapping_add(0x78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[1] = -c.f[1];
                c.f[2] = -c.f[2];
                c.f[3] = -c.f[0];
                c.lr = 0x800d26ec; f_800e05c0(c);
                pc = 0x800d26ec;
            }
            0x800d26ec => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x44);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d26fc; f_800e0290(c);
                pc = 0x800d26fc;
            }
            0x800d26fc => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x74);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d270c; f_800e0290(c);
                pc = 0x800d270c;
            }
            0x800d270c => {
                c.r[3] = c.r[31].wrapping_add(0x94);
                c.r[5] = c.r[1].wrapping_add(0x14);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x800d271c; f_800e0290(c);
                pc = 0x800d271c;
            }
            0x800d271c => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x2), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xb4));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xac));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xb0);
                if true { return; }
                return;
            }
            _ => panic!("f_800d258c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D2738 (0x800d2738, 196 bytes)
#[inline(never)]
pub fn f_800d2738(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d2738) { return; }
        if c.prof { crate::prof::enter(c, 0x800d2738); f_800d2738_b(c, 0x800d2738); crate::prof::exit(c, 0x800d2738); return; }
    }
    f_800d2738_b(c, 0x800d2738)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d2738_at(c: &mut Cpu, pc: u32) { f_800d2738_b(c, pc) }
#[inline(always)]
fn f_800d2738_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d2738;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d2738, pc); }
        match pc {
            0x800d2738 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa254));
                c.cmpu(0, c.r[30], c.r[0]);
                if !c.crbit(2) { pc = 0x800d2764; continue; }
                pc = 0x800d275c;
            }
            0x800d275c => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa254), c.r[0]);
                pc = 0x800d2764;
            }
            0x800d2764 => {
                c.r[31] = c.lwz(c.r[30].wrapping_add(0xc));
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x800d27dc; continue; }
                pc = 0x800d2770;
            }
            0x800d2770 => {
                c.r[3] = 0x10000;
                c.r[4] = c.lhz(c.r[31].wrapping_add(0x4));
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d2794; continue; }
                pc = 0x800d2790;
            }
            0x800d2790 => {
                pc = 0x800d27a4; continue;
                pc = 0x800d2794;
            }
            0x800d2794 => {
                c.r[3] = c.r[4].wrapping_add(0xffffffff);
                c.r[0] = c.r[4].leading_zeros();
                c.sth(c.r[31].wrapping_add(0x4), c.r[3]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800d27a4;
            }
            0x800d27a4 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d27dc; continue; }
                pc = 0x800d27ac;
            }
            0x800d27ac => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x800d27dc; continue; }
                pc = 0x800d27b4;
            }
            0x800d27b4 => {
                c.r[4] = c.lwz(c.r[31]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x30));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d27c8; (c.dispatch)(c, t); }
                pc = 0x800d27c8;
            }
            0x800d27c8 => {
                c.r[4] = c.lwz(c.r[31]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x34));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d27dc; (c.dispatch)(c, t); }
                pc = 0x800d27dc;
            }
            0x800d27dc => {
                c.r[0] = 0x0;
                c.stb(c.r[30], c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d2738: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D27FC (0x800d27fc, 420 bytes)
#[inline(never)]
pub fn f_800d27fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d27fc) { return; }
        if c.prof { crate::prof::enter(c, 0x800d27fc); f_800d27fc_b(c, 0x800d27fc); crate::prof::exit(c, 0x800d27fc); return; }
    }
    f_800d27fc_b(c, 0x800d27fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d27fc_at(c: &mut Cpu, pc: u32) { f_800d27fc_b(c, pc) }
#[inline(always)]
fn f_800d27fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d27fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d27fc, pc); }
        match pc {
            0x800d27fc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa250));
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa24c));
                c.mtspr(9, c.r[0]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(1) { pc = 0x800d2838; continue; }
                pc = 0x800d2820;
            }
            0x800d2820 => {
                c.r[0] = c.lbz(c.r[31]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d2830; continue; }
                pc = 0x800d282c;
            }
            0x800d282c => {
                pc = 0x800d283c; continue;
                pc = 0x800d2830;
            }
            0x800d2830 => {
                c.r[31] = c.r[31].wrapping_add(0x128);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800d2820; continue; }
                pc = 0x800d2838;
            }
            0x800d2838 => {
                c.r[31] = 0x0;
                pc = 0x800d283c;
            }
            0x800d283c => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x800d284c; continue; }
                pc = 0x800d2844;
            }
            0x800d2844 => {
                c.r[3] = 0x0;
                pc = 0x800d298c; continue;
                pc = 0x800d284c;
            }
            0x800d284c => {
                c.stw(c.r[31].wrapping_add(0x8), c.r[3]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[3] = c.lwz(c.r[3]);
                c.lr = 0x800d285c; f_80193f44(c);
                pc = 0x800d285c;
            }
            0x800d285c => {
                c.stw(c.r[31].wrapping_add(0xc), c.r[3]);
                c.r[5] = 0x1;
                c.r[4] = 0x0;
                c.stb(c.r[31], c.r[5]);
                c.stb(c.r[31].wrapping_add(0x4), c.r[4]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d294c; continue; }
                pc = 0x800d2880;
            }
            0x800d2880 => {
                c.stb(c.r[31].wrapping_add(0x3), c.r[5]);
                { let ea = c.r[2].wrapping_add(0xffff92f0); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0x118); let v = c.f[0]; c.stfs(ea, v); }
                c.stw(c.r[31].wrapping_add(0x10c), c.r[5]);
                c.stb(c.r[31].wrapping_add(0x124), c.r[4]);
                c.stw(c.r[31].wrapping_add(0x110), c.r[4]);
                pc = 0x800d28a8; continue;
                pc = 0x800d289c;
            }
            0x800d289c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x110));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[31].wrapping_add(0x110), c.r[0]);
                pc = 0x800d28a8;
            }
            0x800d28a8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x110));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d289c; continue; }
                pc = 0x800d28c4;
            }
            0x800d28c4 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x3));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d2950; continue; }
                pc = 0x800d28d0;
            }
            0x800d28d0 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.lr = 0x800d28d8; f_80196bb8(c);
                pc = 0x800d28d8;
            }
            0x800d28d8 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x110));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(0) { pc = 0x800d2950; continue; }
                pc = 0x800d28e4;
            }
            0x800d28e4 => {
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x114), c.r[0]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x114));
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                c.lr = 0x800d2908; f_80196b10(c);
                pc = 0x800d2908;
            }
            0x800d2908 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x800d2914; f_80196698(c);
                pc = 0x800d2914;
            }
            0x800d2914 => {
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = 0x800d0000;
                c.r[6] = c.r[3].wrapping_add(0x2b44);
                c.r[4] = 0x2;
                { let ea = c.r[13].wrapping_add(0xffffa258); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = 0x10000;
                c.r[5] = c.r[3].wrapping_add(0xffffffff);
                c.r[7] = 0x0;
                c.r[3] = c.lwz(c.r[31].wrapping_add(0xc));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d2940; f_801c028c(c);
                pc = 0x800d2940;
            }
            0x800d2940 => {
                { let ea = c.r[13].wrapping_add(0xffffa258); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0x120); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d2950; continue;
                pc = 0x800d294c;
            }
            0x800d294c => {
                c.stb(c.r[31].wrapping_add(0x3), c.r[4]);
                pc = 0x800d2950;
            }
            0x800d2950 => {
                c.r[3] = 0x1;
                c.r[0] = 0x0;
                c.stb(c.r[31], c.r[3]);
                c.r[3] = c.r[31].wrapping_add(0x70);
                c.stb(c.r[31].wrapping_add(0x1), c.r[0]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[4] = c.r[4].wrapping_add(0xc);
                c.lr = 0x800d2974; f_800e01d0(c);
                pc = 0x800d2974;
            }
            0x800d2974 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[3] = c.r[31].wrapping_add(0x100);
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x28));
                c.r[4] = c.r[4].wrapping_add(0xc);
                c.lr = 0x800d2988; f_800e01d0(c);
                pc = 0x800d2988;
            }
            0x800d2988 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d298c;
            }
            0x800d298c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d27fc: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D29A0 (0x800d29a0, 308 bytes)
#[inline(never)]
pub fn f_800d29a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d29a0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d29a0); f_800d29a0_b(c, 0x800d29a0); crate::prof::exit(c, 0x800d29a0); return; }
    }
    f_800d29a0_b(c, 0x800d29a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d29a0_at(c: &mut Cpu, pc: u32) { f_800d29a0_b(c, pc) }
#[inline(always)]
fn f_800d29a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d29a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d29a0, pc); }
        match pc {
            0x800d29a0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa250));
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa24c));
                c.mtspr(9, c.r[0]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(1) { pc = 0x800d29dc; continue; }
                pc = 0x800d29c4;
            }
            0x800d29c4 => {
                c.r[0] = c.lbz(c.r[31]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d29d4; continue; }
                pc = 0x800d29d0;
            }
            0x800d29d0 => {
                pc = 0x800d29e0; continue;
                pc = 0x800d29d4;
            }
            0x800d29d4 => {
                c.r[31] = c.r[31].wrapping_add(0x128);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800d29c4; continue; }
                pc = 0x800d29dc;
            }
            0x800d29dc => {
                c.r[31] = 0x0;
                pc = 0x800d29e0;
            }
            0x800d29e0 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x800d29f0; continue; }
                pc = 0x800d29e8;
            }
            0x800d29e8 => {
                c.r[3] = 0x0;
                pc = 0x800d2ac0; continue;
                pc = 0x800d29f0;
            }
            0x800d29f0 => {
                c.r[8] = 0x0;
                { let ea = c.r[2].wrapping_add(0xffff92f8); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                c.stw(c.r[31].wrapping_add(0x10), c.r[8]);
                c.r[7] = 0x1;
                { let ea = c.r[2].wrapping_add(0xffff92f0); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.r[6] = 0x280;
                { let ea = c.r[31].wrapping_add(0x14); let v = c.f[5]; c.stfs(ea, v); }
                c.r[5] = 0x1e0;
                c.r[4] = c.r[31].wrapping_add(0x10);
                c.r[0] = c.r[31].wrapping_add(0x24);
                { let ea = c.r[31].wrapping_add(0x18); let v = c.f[5]; c.stfs(ea, v); }
                c.r[3] = c.r[31].wrapping_add(0x38);
                { let ea = c.r[2].wrapping_add(0xffff92f4); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[31].wrapping_add(0x1c); let v = c.f[5]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff9320); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.stw(c.r[31].wrapping_add(0x20), c.r[8]);
                { let ea = c.r[2].wrapping_add(0xffff9334); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[31].wrapping_add(0x24), c.r[8]);
                { let ea = c.r[2].wrapping_add(0xffff9338); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0x28); let v = c.f[5]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x2c); let v = c.f[5]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x30); let v = c.f[4]; c.stfs(ea, v); }
                c.stw(c.r[31].wrapping_add(0x34), c.r[8]);
                c.stw(c.r[31].wrapping_add(0x38), c.r[8]);
                c.sth(c.r[31].wrapping_add(0x3c), c.r[8]);
                c.sth(c.r[31].wrapping_add(0x3e), c.r[7]);
                c.sth(c.r[31].wrapping_add(0x40), c.r[8]);
                c.sth(c.r[31].wrapping_add(0x42), c.r[6]);
                c.sth(c.r[31].wrapping_add(0x44), c.r[8]);
                c.sth(c.r[31].wrapping_add(0x46), c.r[5]);
                c.sth(c.r[31].wrapping_add(0x48), c.r[8]);
                c.sth(c.r[31].wrapping_add(0x4a), c.r[6]);
                c.sth(c.r[31].wrapping_add(0x4c), c.r[8]);
                c.sth(c.r[31].wrapping_add(0x4e), c.r[5]);
                c.stw(c.r[31].wrapping_add(0x50), c.r[4]);
                c.stw(c.r[31].wrapping_add(0x54), c.r[0]);
                { let ea = c.r[31].wrapping_add(0x58); let v = c.f[5]; c.stfs(ea, v); }
                c.stw(c.r[31].wrapping_add(0x5c), c.r[8]);
                { let ea = c.r[31].wrapping_add(0x60); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x64); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x68); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x6c); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x800d2a9c; f_80193f44(c);
                pc = 0x800d2a9c;
            }
            0x800d2a9c => {
                c.stw(c.r[31].wrapping_add(0xc), c.r[3]);
                c.r[4] = 0x1;
                c.r[0] = 0x0;
                c.r[3] = c.r[31] | c.r[31];
                c.stb(c.r[31], c.r[4]);
                c.stb(c.r[31].wrapping_add(0x1), c.r[0]);
                c.stb(c.r[31].wrapping_add(0x4), c.r[0]);
                c.stb(c.r[31].wrapping_add(0x3), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                pc = 0x800d2ac0;
            }
            0x800d2ac0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d29a0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D2AD4 (0x800d2ad4, 112 bytes)
#[inline(never)]
pub fn f_800d2ad4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d2ad4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d2ad4); f_800d2ad4_b(c, 0x800d2ad4); crate::prof::exit(c, 0x800d2ad4); return; }
    }
    f_800d2ad4_b(c, 0x800d2ad4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d2ad4_at(c: &mut Cpu, pc: u32) { f_800d2ad4_b(c, pc) }
#[inline(always)]
fn f_800d2ad4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d2ad4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d2ad4, pc); }
        match pc {
            0x800d2ad4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[0].wrapping_mul(0x128);
                c.stw(c.r[13].wrapping_add(0xffffa250), c.r[0]);
                c.lr = 0x800d2af0; f_800e3534(c);
                pc = 0x800d2af0;
            }
            0x800d2af0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.sth(c.r[13].wrapping_add(0xffffa248), c.r[3]);
                if c.crbit(2) { pc = 0x800d2b34; continue; }
                pc = 0x800d2afc;
            }
            0x800d2afc => {
                c.r[3] = c.r[0] | c.r[0];
                c.lr = 0x800d2b04; f_800e27b0(c);
                pc = 0x800d2b04;
            }
            0x800d2b04 => {
                c.r[5] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa24c), c.r[3]);
                c.r[4] = c.r[5] | c.r[5];
                c.r[6] = 0x0;
                pc = 0x800d2b28; continue;
                pc = 0x800d2b18;
            }
            0x800d2b18 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa24c));
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.stb(c.r[3].wrapping_add(c.r[5]), c.r[4]);
                c.r[5] = c.r[5].wrapping_add(0x128);
                pc = 0x800d2b28;
            }
            0x800d2b28 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa250));
                c.cmpu(0, c.r[6], c.r[0]);
                if c.crbit(0) { pc = 0x800d2b18; continue; }
                pc = 0x800d2b34;
            }
            0x800d2b34 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d2ad4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D2B44 (0x800d2b44, 76 bytes)
#[inline(never)]
pub fn f_800d2b44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d2b44) { return; }
        if c.prof { crate::prof::enter(c, 0x800d2b44); f_800d2b44_b(c, 0x800d2b44); crate::prof::exit(c, 0x800d2b44); return; }
    }
    f_800d2b44_b(c, 0x800d2b44)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d2b44_at(c: &mut Cpu, pc: u32) { f_800d2b44_b(c, pc) }
#[inline(always)]
fn f_800d2b44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d2b44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d2b44, pc); }
        match pc {
            0x800d2b44 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if !c.crbit(2) { pc = 0x800d2b6c; continue; }
                pc = 0x800d2b5c;
            }
            0x800d2b5c => {
                c.r[3] = c.r[2].wrapping_add(0xffff933c);
                c.r[4] = 0xab;
                c.r[5] = c.r[2].wrapping_add(0xffff9344);
                c.lr = 0x800d2b6c; f_80196e10(c);
                pc = 0x800d2b6c;
            }
            0x800d2b6c => {
                { let ea = c.r[2].wrapping_add(0xffff92f0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[31].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[13].wrapping_add(0xffffa258); let v = c.f[0]; c.stfs(ea, v); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d2b44: bad pc {:#x}", pc),
        }
    }
}

/// gfxSetFog (0x800d2b90, 600 bytes)
#[inline(never)]
pub fn f_800d2b90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d2b90) { return; }
        if c.prof { crate::prof::enter(c, 0x800d2b90); f_800d2b90_b(c, 0x800d2b90); crate::prof::exit(c, 0x800d2b90); return; }
    }
    f_800d2b90_b(c, 0x800d2b90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d2b90_at(c: &mut Cpu, pc: u32) { f_800d2b90_b(c, pc) }
#[inline(always)]
fn f_800d2b90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d2b90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d2b90, pc); }
        match pc {
            0x800d2b90 => {
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                c.r[6] = c.r[4].wrapping_add(0x350);
                c.stw(c.r[1].wrapping_add(0x6c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x68), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa26c));
                c.r[5] = c.lwz(c.r[6]);
                c.r[4] = c.lwz(c.r[6].wrapping_add(0x4));
                c.cmpu(0, c.r[31], 0x0);
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x8));
                c.r[0] = c.lwz(c.r[6].wrapping_add(0xc));
                c.stw(c.r[1].wrapping_add(0x18), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[4]);
                c.stw(c.r[1].wrapping_add(0x20), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                if c.crbit(2) { pc = 0x800d2c54; continue; }
                pc = 0x800d2bdc;
            }
            0x800d2bdc => {
                if c.crbit(2) { pc = 0x800d2c4c; continue; }
                pc = 0x800d2be0;
            }
            0x800d2be0 => {
                c.r[3] = 0x10000;
                c.r[4] = c.lhz(c.r[31].wrapping_add(0x4));
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.r[0] = c.r[0].rotate_left(0) & 0xffff;
                c.r[0] = c.r[0].wrapping_sub(c.r[4]);
                c.r[0] = c.r[0].leading_zeros();
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d2c04; continue; }
                pc = 0x800d2c00;
            }
            0x800d2c00 => {
                pc = 0x800d2c14; continue;
                pc = 0x800d2c04;
            }
            0x800d2c04 => {
                c.r[3] = c.r[4].wrapping_add(0xffffffff);
                c.r[0] = c.r[4].leading_zeros();
                c.sth(c.r[31].wrapping_add(0x4), c.r[3]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7ffffff;
                pc = 0x800d2c14;
            }
            0x800d2c14 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d2c4c; continue; }
                pc = 0x800d2c1c;
            }
            0x800d2c1c => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x800d2c4c; continue; }
                pc = 0x800d2c24;
            }
            0x800d2c24 => {
                c.r[4] = c.lwz(c.r[31]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x30));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d2c38; (c.dispatch)(c, t); }
                pc = 0x800d2c38;
            }
            0x800d2c38 => {
                c.r[4] = c.lwz(c.r[31]);
                c.r[3] = c.r[31] | c.r[31];
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x34));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d2c4c; (c.dispatch)(c, t); }
                pc = 0x800d2c4c;
            }
            0x800d2c4c => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa26c), c.r[0]);
                pc = 0x800d2c54;
            }
            0x800d2c54 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x800d2c6c; continue; }
                pc = 0x800d2c5c;
            }
            0x800d2c5c => {
                c.r[3] = 0x0;
                c.lr = 0x800d2c64; f_8019bd18(c);
                pc = 0x800d2c64;
            }
            0x800d2c64 => {
                c.lr = 0x800d2c68; f_8016ea88(c);
                pc = 0x800d2c68;
            }
            0x800d2c68 => {
                pc = 0x800d2dd0; continue;
                pc = 0x800d2c6c;
            }
            0x800d2c6c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x800d2c74; f_8019bb78(c);
                pc = 0x800d2c74;
            }
            0x800d2c74 => {
                c.stw(c.r[13].wrapping_add(0xffffa26c), c.r[3]);
                c.lr = 0x800d2c7c; f_8016eb30(c);
                pc = 0x800d2c7c;
            }
            0x800d2c7c => {
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x10));
                c.r[5] = 0x43300000;
                c.r[4] = c.lbz(c.r[30].wrapping_add(0x11));
                c.r[0] = 0x1;
                c.stw(c.r[1].wrapping_add(0x2c), c.r[3]);
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x12));
                c.stw(c.r[1].wrapping_add(0x28), c.r[5]);
                { let ea = c.r[2].wrapping_add(0xffff9358); c.f[6] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x34), c.r[4]);
                { let v = s(c.f[0] - c.f[6]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9350); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                c.stw(c.r[1].wrapping_add(0x30), c.r[5]);
                c.r[4] = c.lbz(c.r[30].wrapping_add(0x13));
                { let ea = c.r[1].wrapping_add(0x30); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[0] / c.f[5]); c.f[3] = v; c.ps1[3] = v; }
                c.stw(c.r[1].wrapping_add(0x3c), c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[1].wrapping_add(0x38), c.r[5]);
                { let ea = c.r[1].wrapping_add(0x38); c.f[0] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x44), c.r[4]);
                { let v = s(c.f[1] - c.f[6]); c.f[1] = v; c.ps1[1] = v; }
                c.stw(c.r[1].wrapping_add(0x40), c.r[5]);
                { let v = s(c.f[0] - c.f[6]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x40); c.f[2] = c.lfd(ea); }
                { let v = s(c.f[1] / c.f[5]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[3]; c.stfs(ea, v); }
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x18));
                c.stb(c.r[3].wrapping_add(0x19), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let v = s(c.f[0] / c.f[5]); c.f[4] = v; c.ps1[4] = v; }
                { let ea = c.r[1].wrapping_add(0x1c); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] - c.f[6]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[4]; c.stfs(ea, v); }
                { let v = s(c.f[5] * c.f[1]); c.f[2] = v; c.ps1[2] = v; }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x20));
                { let v = s(c.f[5] * c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[3] / c.f[5]); c.f[4] = v; c.ps1[4] = v; }
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[3] = fctiwz(c.f[2]);
                { let ea = c.r[1].wrapping_add(0x24); let v = c.f[4]; c.stfs(ea, v); }
                c.f[2] = fctiwz(c.f[1]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                { let v = s(c.f[5] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x48); let v = c.f[3]; c.stfd(ea, v); }
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.f[1] = fctiwz(c.f[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x4c));
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[2]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stb(c.r[3].wrapping_add(0x1c), c.r[0]);
                { let v = s(c.f[5] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[1].wrapping_add(0x58); let v = c.f[1]; c.stfd(ea, v); }
                c.f[0] = fctiwz(c.f[0]);
                c.stb(c.r[3].wrapping_add(0x1d), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x5c));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[0]; c.stfd(ea, v); }
                c.stb(c.r[3].wrapping_add(0x1e), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x1f), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d2dd0; continue; }
                pc = 0x800d2da4;
            }
            0x800d2da4 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1d));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d2dd0; continue; }
                pc = 0x800d2db0;
            }
            0x800d2db0 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d2dd0; continue; }
                pc = 0x800d2dbc;
            }
            0x800d2dbc => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1f));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d2dd0; continue; }
                pc = 0x800d2dc8;
            }
            0x800d2dc8 => {
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x19), c.r[0]);
                pc = 0x800d2dd0;
            }
            0x800d2dd0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x74));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x6c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x68));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { return; }
                return;
            }
            _ => panic!("f_800d2b90: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D2DE8 (0x800d2de8, 332 bytes)
#[inline(never)]
pub fn f_800d2de8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d2de8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d2de8); f_800d2de8_b(c, 0x800d2de8); crate::prof::exit(c, 0x800d2de8); return; }
    }
    f_800d2de8_b(c, 0x800d2de8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d2de8_at(c: &mut Cpu, pc: u32) { f_800d2de8_b(c, pc) }
#[inline(always)]
fn f_800d2de8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d2de8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d2de8, pc); }
        match pc {
            0x800d2de8 => {
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                { let mut ea = c.r[1].wrapping_add(0x54); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[5] | c.r[5];
                c.r[28] = c.r[0] | c.r[0];
                c.r[27] = c.r[4] | c.r[4];
                c.lr = 0x800d2e10; f_800d7bf8(c);
                pc = 0x800d2e10;
            }
            0x800d2e10 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x800d2e20; f_800d7bf8(c);
                pc = 0x800d2e20;
            }
            0x800d2e20 => {
                c.cmpu(0, c.r[30], 0x0);
                c.r[31] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x800d2e34; continue; }
                pc = 0x800d2e2c;
            }
            0x800d2e2c => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x800d2e3c; continue; }
                pc = 0x800d2e34;
            }
            0x800d2e34 => {
                c.r[3] = 0x0;
                pc = 0x800d2f20; continue;
                pc = 0x800d2e3c;
            }
            0x800d2e3c => {
                { let ea = c.r[2].wrapping_add(0xffff9360); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.r[26] = 0x2;
                { let ea = c.r[2].wrapping_add(0xffff9364); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[29] = 0x0;
                { let ea = c.r[1].wrapping_add(0x2c); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[2].wrapping_add(0xffff9368); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[31]; c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[2].wrapping_add(0xffff936c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x34); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x14); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x18); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x3c); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x28); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x2c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x44); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x14); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x1c); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x24); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d2f14; continue;
                pc = 0x800d2ea4;
            }
            0x800d2ea4 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800d2eb4; f_800dff98(c);
                pc = 0x800d2eb4;
            }
            0x800d2eb4 => {
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[31].wrapping_add(0x2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                { let (a, b) = (c.crbit(1), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x800d2ee0; continue; }
                pc = 0x800d2ec8;
            }
            0x800d2ec8 => {
                { let ea = c.r[2].wrapping_add(0xffff936c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[26] = 0x3;
                { let ea = c.r[27].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[27].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[27]; let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d2f08; continue;
                pc = 0x800d2ee0;
            }
            0x800d2ee0 => {
                { let ea = c.r[28]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30] | c.r[30];
                { let ea = c.r[28].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[6] = c.r[27] | c.r[27];
                { let ea = c.r[28].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.r[4] = c.r[1].wrapping_add(0x2c);
                c.r[5] = c.r[1].wrapping_add(0x14);
                c.r[7] = c.r[27].wrapping_add(0x4);
                c.r[8] = c.r[27].wrapping_add(0x8);
                c.lr = 0x800d2f08; f_800bd16c(c);
                pc = 0x800d2f08;
            }
            0x800d2f08 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[28] = c.r[28].wrapping_add(0xc);
                c.r[27] = c.r[27].wrapping_add(0xc);
                pc = 0x800d2f14;
            }
            0x800d2f14 => {
                c.cmpu(0, c.r[29], c.r[25]);
                if c.crbit(0) { pc = 0x800d2ea4; continue; }
                pc = 0x800d2f1c;
            }
            0x800d2f1c => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x800d2f20;
            }
            0x800d2f20 => {
                { let mut ea = c.r[1].wrapping_add(0x54); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x74));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { return; }
                return;
            }
            _ => panic!("f_800d2de8: bad pc {:#x}", pc),
        }
    }
}

/// gfxWorldToScreen (0x800d2f34, 296 bytes)
#[inline(never)]
pub fn f_800d2f34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d2f34) { return; }
        if c.prof { crate::prof::enter(c, 0x800d2f34); f_800d2f34_b(c, 0x800d2f34); crate::prof::exit(c, 0x800d2f34); return; }
    }
    f_800d2f34_b(c, 0x800d2f34)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d2f34_at(c: &mut Cpu, pc: u32) { f_800d2f34_b(c, pc) }
#[inline(always)]
fn f_800d2f34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d2f34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d2f34, pc); }
        match pc {
            0x800d2f34 => {
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x5c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x58), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x54), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x50), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x800d2f60; f_800d7bf8(c);
                pc = 0x800d2f60;
            }
            0x800d2f60 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[30] = c.r[0] | c.r[0];
                c.lr = 0x800d2f70; f_800d7bf8(c);
                pc = 0x800d2f70;
            }
            0x800d2f70 => {
                c.cmpu(0, c.r[30], 0x0);
                c.r[31] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x800d2f84; continue; }
                pc = 0x800d2f7c;
            }
            0x800d2f7c => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x800d2f8c; continue; }
                pc = 0x800d2f84;
            }
            0x800d2f84 => {
                c.r[3] = 0x0;
                pc = 0x800d303c; continue;
                pc = 0x800d2f8c;
            }
            0x800d2f8c => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = c.r[28] | c.r[28];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800d2f9c; f_800dff98(c);
                pc = 0x800d2f9c;
            }
            0x800d2f9c => {
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0x2c); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                c.fcmp(0, c.f[0], c.f[5]);
                { let (a, b) = (c.crbit(1), c.crbit(2)); c.set_crbit(2, a | b); }
                if !c.crbit(2) { pc = 0x800d2fb8; continue; }
                pc = 0x800d2fb0;
            }
            0x800d2fb0 => {
                c.r[3] = 0x1;
                pc = 0x800d303c; continue;
                pc = 0x800d2fb8;
            }
            0x800d2fb8 => {
                { let ea = c.r[2].wrapping_add(0xffff9360); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.r[3] = c.r[30] | c.r[30];
                { let ea = c.r[2].wrapping_add(0xffff9364); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[6] = c.r[29] | c.r[29];
                { let ea = c.r[1].wrapping_add(0x2c); let v = c.f[4]; c.stfs(ea, v); }
                c.r[4] = c.r[1].wrapping_add(0x2c);
                { let ea = c.r[2].wrapping_add(0xffff9368); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[5] = c.r[1].wrapping_add(0x14);
                { let ea = c.r[31]; c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.r[7] = c.r[29].wrapping_add(0x4);
                { let ea = c.r[2].wrapping_add(0xffff936c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[8] = c.r[29].wrapping_add(0x8);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x34); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x14); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x18); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x3c); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x28); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[3]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x44); let v = c.f[5]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x14); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x1c); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x24); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[28]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[28].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[28].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x800d3038; f_800bd16c(c);
                pc = 0x800d3038;
            }
            0x800d3038 => {
                c.r[3] = 0x2;
                pc = 0x800d303c;
            }
            0x800d303c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x5c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x58));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x50));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x60);
                if true { return; }
                return;
            }
            _ => panic!("f_800d2f34: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D305C (0x800d305c, 12 bytes)
#[inline(never)]
pub fn f_800d305c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d305c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d305c); f_800d305c_b(c, 0x800d305c); crate::prof::exit(c, 0x800d305c); return; }
    }
    f_800d305c_b(c, 0x800d305c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d305c_at(c: &mut Cpu, pc: u32) { f_800d305c_b(c, pc) }
#[inline(always)]
fn f_800d305c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d305c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d305c, pc); }
        match pc {
            0x800d305c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x5c), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d305c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D3068 (0x800d3068, 12 bytes)
#[inline(never)]
pub fn f_800d3068(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3068) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3068); f_800d3068_b(c, 0x800d3068); crate::prof::exit(c, 0x800d3068); return; }
    }
    f_800d3068_b(c, 0x800d3068)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3068_at(c: &mut Cpu, pc: u32) { f_800d3068_b(c, pc) }
#[inline(always)]
fn f_800d3068_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3068;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3068, pc); }
        match pc {
            0x800d3068 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x58));
                if true { return; }
                return;
            }
            _ => panic!("f_800d3068: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D3074 (0x800d3074, 20 bytes)
#[inline(never)]
pub fn f_800d3074(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3074) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3074); f_800d3074_b(c, 0x800d3074); crate::prof::exit(c, 0x800d3074); return; }
    }
    f_800d3074_b(c, 0x800d3074)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3074_at(c: &mut Cpu, pc: u32) { f_800d3074_b(c, pc) }
#[inline(always)]
fn f_800d3074_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3074;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3074, pc); }
        match pc {
            0x800d3074 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x800d307c;
            }
            0x800d307c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[4].wrapping_add(0x58), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d3074: bad pc {:#x}", pc),
        }
    }
}

/// msgGetCharsPerFrame (0x800d3088, 12 bytes)
#[inline(never)]
pub fn f_800d3088(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3088) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3088); f_800d3088_b(c, 0x800d3088); crate::prof::exit(c, 0x800d3088); return; }
    }
    f_800d3088_b(c, 0x800d3088)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3088_at(c: &mut Cpu, pc: u32) { f_800d3088_b(c, pc) }
#[inline(always)]
fn f_800d3088_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3088;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3088, pc); }
        match pc {
            0x800d3088 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x54));
                if true { return; }
                return;
            }
            _ => panic!("f_800d3088: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D3094 (0x800d3094, 12 bytes)
#[inline(never)]
pub fn f_800d3094(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3094) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3094); f_800d3094_b(c, 0x800d3094); crate::prof::exit(c, 0x800d3094); return; }
    }
    f_800d3094_b(c, 0x800d3094)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3094_at(c: &mut Cpu, pc: u32) { f_800d3094_b(c, pc) }
#[inline(always)]
fn f_800d3094_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3094;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3094, pc); }
        match pc {
            0x800d3094 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4c));
                if true { return; }
                return;
            }
            _ => panic!("f_800d3094: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D30A0 (0x800d30a0, 12 bytes)
#[inline(never)]
pub fn f_800d30a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d30a0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d30a0); f_800d30a0_b(c, 0x800d30a0); crate::prof::exit(c, 0x800d30a0); return; }
    }
    f_800d30a0_b(c, 0x800d30a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d30a0_at(c: &mut Cpu, pc: u32) { f_800d30a0_b(c, pc) }
#[inline(always)]
fn f_800d30a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d30a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d30a0, pc); }
        match pc {
            0x800d30a0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[4].wrapping_add(0x48), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d30a0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D30AC (0x800d30ac, 68 bytes)
#[inline(never)]
pub fn f_800d30ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d30ac) { return; }
        if c.prof { crate::prof::enter(c, 0x800d30ac); f_800d30ac_b(c, 0x800d30ac); crate::prof::exit(c, 0x800d30ac); return; }
    }
    f_800d30ac_b(c, 0x800d30ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d30ac_at(c: &mut Cpu, pc: u32) { f_800d30ac_b(c, pc) }
#[inline(always)]
fn f_800d30ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d30ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d30ac, pc); }
        match pc {
            0x800d30ac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d30dc; continue; }
                pc = 0x800d30c8;
            }
            0x800d30c8 => {
                c.r[3] = 0x4;
                c.r[4] = 0x0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d30d8; f_800d4f98(c);
                pc = 0x800d30d8;
            }
            0x800d30d8 => {
                pc = 0x800d30e0; continue;
                pc = 0x800d30dc;
            }
            0x800d30dc => {
                c.lr = 0x800d30e0; f_800b8920(c);
                pc = 0x800d30e0;
            }
            0x800d30e0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d30ac: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D30F0 (0x800d30f0, 160 bytes)
#[inline(never)]
pub fn f_800d30f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d30f0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d30f0); f_800d30f0_b(c, 0x800d30f0); crate::prof::exit(c, 0x800d30f0); return; }
    }
    f_800d30f0_b(c, 0x800d30f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d30f0_at(c: &mut Cpu, pc: u32) { f_800d30f0_b(c, pc) }
#[inline(always)]
fn f_800d30f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d30f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d30f0, pc); }
        match pc {
            0x800d30f0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[0] = c.r[3].wrapping_add(0x1020000);
                c.cmpu(0, c.r[0], 0xfefe);
                if c.crbit(2) { pc = 0x800d317c; continue; }
                pc = 0x800d3118;
            }
            0x800d3118 => {
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x49d));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d313c; continue; }
                pc = 0x800d3124;
            }
            0x800d3124 => {
                c.lr = 0x800d3128; f_800dc560(c);
                pc = 0x800d3128;
            }
            0x800d3128 => {
                c.r[3] = 0x0;
                c.lr = 0x800d3130; f_801bf8a0(c);
                pc = 0x800d3130;
            }
            0x800d3130 => {
                c.lr = 0x800d3134; f_801e16f0(c);
                pc = 0x800d3134;
            }
            0x800d3134 => {
                c.lr = 0x800d3138; f_801bf6ac(c);
                pc = 0x800d3138;
            }
            0x800d3138 => {
                pc = 0x800d3150; continue;
                pc = 0x800d313c;
            }
            0x800d313c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d3150; continue; }
                pc = 0x800d3144;
            }
            0x800d3144 => {
                c.r[4] = 0x1;
                c.lr = 0x800d314c; f_800ef1e8(c);
                pc = 0x800d314c;
            }
            0x800d314c => {
                c.r[31] = 0x0;
                pc = 0x800d3150;
            }
            0x800d3150 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = 0x1;
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x49d), c.r[4]);
                if c.crbit(2) { pc = 0x800d317c; continue; }
                pc = 0x800d3164;
            }
            0x800d3164 => {
                c.lr = 0x800d3168; f_800b8e74(c);
                pc = 0x800d3168;
            }
            0x800d3168 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xc));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d317c; continue; }
                pc = 0x800d3178;
            }
            0x800d3178 => {
                c.lr = 0x800d317c; f_800bb29c(c);
                pc = 0x800d317c;
            }
            0x800d317c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d30f0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D3190 (0x800d3190, 640 bytes)
#[inline(never)]
pub fn f_800d3190(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3190) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3190); f_800d3190_b(c, 0x800d3190); crate::prof::exit(c, 0x800d3190); return; }
    }
    f_800d3190_b(c, 0x800d3190)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3190_at(c: &mut Cpu, pc: u32) { f_800d3190_b(c, pc) }
#[inline(always)]
fn f_800d3190_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3190;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3190, pc); }
        match pc {
            0x800d3190 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.r[0] = c.r[3].wrapping_add(0x1020000);
                c.cmpu(0, c.r[0], 0xfefe);
                if c.crbit(2) { pc = 0x800d33f4; continue; }
                pc = 0x800d31bc;
            }
            0x800d31bc => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa271), c.r[0]);
                c.lr = 0x800d31c8; f_800b8920(c);
                pc = 0x800d31c8;
            }
            0x800d31c8 => {
                c.lr = 0x800d31cc; f_800b8d10(c);
                pc = 0x800d31cc;
            }
            0x800d31cc => {
                c.lr = 0x800d31d0; f_800a2790(c);
                pc = 0x800d31d0;
            }
            0x800d31d0 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = 0x80000000;
                c.r[30] = 0x1;
                pc = 0x800d3210; continue;
                pc = 0x800d31e0;
            }
            0x800d31e0 => {
                c.lr = 0x800d31e4; f_800a1990(c);
                pc = 0x800d31e4;
            }
            0x800d31e4 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa271));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d3210; continue; }
                pc = 0x800d31f0;
            }
            0x800d31f0 => {
                c.lr = 0x800d31f4; f_800a2790(c);
                pc = 0x800d31f4;
            }
            0x800d31f4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xf8));
                c.r[3] = c.r[3].wrapping_sub(c.r[29]);
                c.r[0] = c.r[0].rotate_left(30) & 0x3fffffff;
                c.r[0] = divwu(c.r[3], c.r[0]);
                c.cmpu(0, c.r[0], 0x3);
                if !c.crbit(1) { pc = 0x800d3210; continue; }
                pc = 0x800d320c;
            }
            0x800d320c => {
                c.stb(c.r[13].wrapping_add(0xffffa271), c.r[30]);
                pc = 0x800d3210;
            }
            0x800d3210 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa271));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d31e0; continue; }
                pc = 0x800d321c;
            }
            0x800d321c => {
                c.lr = 0x800d3220; f_8019c6fc(c);
                pc = 0x800d3220;
            }
            0x800d3220 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[4].wrapping_add(0xc));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d32a8; continue; }
                pc = 0x800d3230;
            }
            0x800d3230 => {
                c.r[0] = c.r[3].wrapping_add(0x1020000);
                c.r[29] = 0x1;
                c.cmpu(0, c.r[0], 0xfefe);
                if c.crbit(2) { pc = 0x800d32d8; continue; }
                pc = 0x800d3240;
            }
            0x800d3240 => {
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x49d));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d3264; continue; }
                pc = 0x800d324c;
            }
            0x800d324c => {
                c.lr = 0x800d3250; f_800dc560(c);
                pc = 0x800d3250;
            }
            0x800d3250 => {
                c.r[3] = 0x0;
                c.lr = 0x800d3258; f_801bf8a0(c);
                pc = 0x800d3258;
            }
            0x800d3258 => {
                c.lr = 0x800d325c; f_801e16f0(c);
                pc = 0x800d325c;
            }
            0x800d325c => {
                c.lr = 0x800d3260; f_801bf6ac(c);
                pc = 0x800d3260;
            }
            0x800d3260 => {
                pc = 0x800d3278; continue;
                pc = 0x800d3264;
            }
            0x800d3264 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d3278; continue; }
                pc = 0x800d326c;
            }
            0x800d326c => {
                c.r[4] = 0x1;
                c.lr = 0x800d3274; f_800ef1e8(c);
                pc = 0x800d3274;
            }
            0x800d3274 => {
                c.r[29] = 0x0;
                pc = 0x800d3278;
            }
            0x800d3278 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = 0x1;
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x49d), c.r[4]);
                if c.crbit(2) { pc = 0x800d32d8; continue; }
                pc = 0x800d328c;
            }
            0x800d328c => {
                c.lr = 0x800d3290; f_800b8e74(c);
                pc = 0x800d3290;
            }
            0x800d3290 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xc));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d32d8; continue; }
                pc = 0x800d32a0;
            }
            0x800d32a0 => {
                c.lr = 0x800d32a4; f_800bb29c(c);
                pc = 0x800d32a4;
            }
            0x800d32a4 => {
                pc = 0x800d32d8; continue;
                pc = 0x800d32a8;
            }
            0x800d32a8 => {
                c.r[3] = 0x80400000;
                c.r[0] = 0x0;
                c.r[6] = c.r[3].wrapping_add(0x1f0);
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x10));
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x4));
                c.r[4] = c.lwz(c.r[6].wrapping_add(0xc));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[6], c.r[5]);
                c.stw(c.r[6].wrapping_add(0x8), c.r[4]);
                c.stw(c.r[6].wrapping_add(0x10), c.r[3]);
                c.stw(c.r[6].wrapping_add(0x4), c.r[0]);
                c.stw(c.r[6].wrapping_add(0xc), c.r[0]);
                pc = 0x800d32d8;
            }
            0x800d32d8 => {
                c.r[4] = 0xfeff0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.r[4].wrapping_add(0xfffffefe);
                c.stw(c.r[3].wrapping_add(0xc), c.r[0]);
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa270));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d33f4; continue; }
                pc = 0x800d32f4;
            }
            0x800d32f4 => {
                c.lr = 0x800d32f8; f_800a2790(c);
                pc = 0x800d32f8;
            }
            0x800d32f8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x54));
                c.lr = 0x800d3308; f_800d1070(c);
                pc = 0x800d3308;
            }
            0x800d3308 => {
                c.lr = 0x800d330c; f_800a2790(c);
                pc = 0x800d330c;
            }
            0x800d330c => {
                c.r[4] = 0x80400000;
                c.r[0] = c.r[3].wrapping_sub(c.r[30]);
                c.r[4] = c.r[4].wrapping_add(0x1f0);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x2c));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[4].wrapping_add(0x2c), c.r[0]);
                c.lr = 0x800d3328; f_800a2790(c);
                pc = 0x800d3328;
            }
            0x800d3328 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x54));
                c.lr = 0x800d3338; f_800dc6d8(c);
                pc = 0x800d3338;
            }
            0x800d3338 => {
                c.lr = 0x800d333c; f_800a2790(c);
                pc = 0x800d333c;
            }
            0x800d333c => {
                c.r[4] = 0x80400000;
                c.r[0] = c.r[3].wrapping_sub(c.r[30]);
                c.r[4] = c.r[4].wrapping_add(0x1f0);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x30));
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[4].wrapping_add(0x30), c.r[0]);
                c.lr = 0x800d3358; f_800a2790(c);
                pc = 0x800d3358;
            }
            0x800d3358 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x54));
                c.lr = 0x800d336c; f_800e3884(c);
                pc = 0x800d336c;
            }
            0x800d336c => {
                c.lr = 0x800d3370; f_800a2790(c);
                pc = 0x800d3370;
            }
            0x800d3370 => {
                c.r[5] = 0x80400000;
                c.r[4] = c.r[3].wrapping_sub(c.r[30]);
                c.r[30] = c.r[5].wrapping_add(0x1f0);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = c.lwz(c.r[30].wrapping_add(0x34));
                c.r[0] = 0x0;
                c.r[4] = c.r[5].wrapping_add(c.r[4]);
                c.stw(c.r[30].wrapping_add(0x34), c.r[4]);
                c.r[29] = c.lwz(c.r[3].wrapping_add(0x54));
                c.stw(c.r[30].wrapping_add(0x38), c.r[0]);
                pc = 0x800d33e8; continue;
                pc = 0x800d339c;
            }
            0x800d339c => {
                c.lr = 0x800d33a0; f_800a2790(c);
                pc = 0x800d33a0;
            }
            0x800d33a0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.lr = 0x800d33b0; f_800e3884(c);
                pc = 0x800d33b0;
            }
            0x800d33b0 => {
                c.lr = 0x800d33b4; f_800a2790(c);
                pc = 0x800d33b4;
            }
            0x800d33b4 => {
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x34));
                c.r[0] = c.r[3].wrapping_sub(c.r[31]);
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[30].wrapping_add(0x34), c.r[0]);
                c.lr = 0x800d33c8; f_800a2790(c);
                pc = 0x800d33c8;
            }
            0x800d33c8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.lr = 0x800d33d4; f_801181b0(c);
                pc = 0x800d33d4;
            }
            0x800d33d4 => {
                c.lr = 0x800d33d8; f_800a2790(c);
                pc = 0x800d33d8;
            }
            0x800d33d8 => {
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x38));
                c.r[0] = c.r[3].wrapping_sub(c.r[31]);
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[30].wrapping_add(0x38), c.r[0]);
                pc = 0x800d33e8;
            }
            0x800d33e8 => {
                c.cmps(0, c.r[29], 0x0);
                c.r[29] = c.r[29].wrapping_add(0xffffffff);
                if !c.crbit(2) { pc = 0x800d339c; continue; }
                pc = 0x800d33f4;
            }
            0x800d33f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d3190: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D3410 (0x800d3410, 524 bytes)
#[inline(never)]
pub fn f_800d3410(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3410) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3410); f_800d3410_b(c, 0x800d3410); crate::prof::exit(c, 0x800d3410); return; }
    }
    f_800d3410_b(c, 0x800d3410)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3410_at(c: &mut Cpu, pc: u32) { f_800d3410_b(c, pc) }
#[inline(always)]
fn f_800d3410_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3410;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3410, pc); }
        match pc {
            0x800d3410 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[5].wrapping_add(0xc));
                c.r[0] = c.r[3].wrapping_add(0x1020000);
                c.cmpu(0, c.r[0], 0xfefe);
                if !c.crbit(2) { pc = 0x800d3600; continue; }
                pc = 0x800d3444;
            }
            0x800d3444 => {
                c.stw(c.r[5].wrapping_add(0xc), c.r[31]);
                c.r[30] = 0x0;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[0] = c.r[3].wrapping_add(0x1020000);
                c.cmpu(0, c.r[0], 0xfefe);
                if c.crbit(2) { pc = 0x800d34c4; continue; }
                pc = 0x800d3460;
            }
            0x800d3460 => {
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x49d));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d3484; continue; }
                pc = 0x800d346c;
            }
            0x800d346c => {
                c.lr = 0x800d3470; f_800dc560(c);
                pc = 0x800d3470;
            }
            0x800d3470 => {
                c.r[3] = 0x0;
                c.lr = 0x800d3478; f_801bf8a0(c);
                pc = 0x800d3478;
            }
            0x800d3478 => {
                c.lr = 0x800d347c; f_801e16f0(c);
                pc = 0x800d347c;
            }
            0x800d347c => {
                c.lr = 0x800d3480; f_801bf6ac(c);
                pc = 0x800d3480;
            }
            0x800d3480 => {
                pc = 0x800d3498; continue;
                pc = 0x800d3484;
            }
            0x800d3484 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d3498; continue; }
                pc = 0x800d348c;
            }
            0x800d348c => {
                c.r[4] = 0x1;
                c.lr = 0x800d3494; f_800ef1e8(c);
                pc = 0x800d3494;
            }
            0x800d3494 => {
                c.r[30] = 0x0;
                pc = 0x800d3498;
            }
            0x800d3498 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = 0x1;
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x49d), c.r[4]);
                if c.crbit(2) { pc = 0x800d34c4; continue; }
                pc = 0x800d34ac;
            }
            0x800d34ac => {
                c.lr = 0x800d34b0; f_800b8e74(c);
                pc = 0x800d34b0;
            }
            0x800d34b0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xc));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d34c4; continue; }
                pc = 0x800d34c0;
            }
            0x800d34c0 => {
                c.lr = 0x800d34c4; f_800bb29c(c);
                pc = 0x800d34c4;
            }
            0x800d34c4 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.stb(c.r[13].wrapping_add(0xffffa270), c.r[29]);
                if c.crbit(2) { pc = 0x800d3570; continue; }
                pc = 0x800d34d0;
            }
            0x800d34d0 => {
                c.lr = 0x800d34d4; f_800a2790(c);
                pc = 0x800d34d4;
            }
            0x800d34d4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x54));
                c.lr = 0x800d34e4; f_800d13c4(c);
                pc = 0x800d34e4;
            }
            0x800d34e4 => {
                c.lr = 0x800d34e8; f_800a2790(c);
                pc = 0x800d34e8;
            }
            0x800d34e8 => {
                c.r[4] = 0x80400000;
                c.r[0] = c.r[3].wrapping_sub(c.r[30]);
                c.r[3] = c.r[4].wrapping_add(0x1f0);
                c.stw(c.r[3].wrapping_add(0x2c), c.r[0]);
                c.lr = 0x800d34fc; f_800a2790(c);
                pc = 0x800d34fc;
            }
            0x800d34fc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x54));
                c.lr = 0x800d350c; f_800dc874(c);
                pc = 0x800d350c;
            }
            0x800d350c => {
                c.lr = 0x800d3510; f_800a2790(c);
                pc = 0x800d3510;
            }
            0x800d3510 => {
                c.r[4] = 0x80400000;
                c.r[0] = c.r[3].wrapping_sub(c.r[30]);
                c.r[3] = c.r[4].wrapping_add(0x1f0);
                c.stw(c.r[3].wrapping_add(0x30), c.r[0]);
                c.lr = 0x800d3524; f_800a2790(c);
                pc = 0x800d3524;
            }
            0x800d3524 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x54));
                c.lr = 0x800d3534; f_800e3928(c);
                pc = 0x800d3534;
            }
            0x800d3534 => {
                c.lr = 0x800d3538; f_800a2790(c);
                pc = 0x800d3538;
            }
            0x800d3538 => {
                c.r[4] = 0x80400000;
                c.r[0] = c.r[3].wrapping_sub(c.r[30]);
                c.r[3] = c.r[4].wrapping_add(0x1f0);
                c.stw(c.r[3].wrapping_add(0x34), c.r[0]);
                c.lr = 0x800d354c; f_800a2790(c);
                pc = 0x800d354c;
            }
            0x800d354c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x54));
                c.lr = 0x800d355c; f_801183ec(c);
                pc = 0x800d355c;
            }
            0x800d355c => {
                c.lr = 0x800d3560; f_800a2790(c);
                pc = 0x800d3560;
            }
            0x800d3560 => {
                c.r[4] = 0x80400000;
                c.r[0] = c.r[3].wrapping_sub(c.r[30]);
                c.r[3] = c.r[4].wrapping_add(0x1f0);
                c.stw(c.r[3].wrapping_add(0x38), c.r[0]);
                pc = 0x800d3570;
            }
            0x800d3570 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x800d35f8; continue; }
                pc = 0x800d3578;
            }
            0x800d3578 => {
                c.lr = 0x800d357c; f_800a2790(c);
                pc = 0x800d357c;
            }
            0x800d357c => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800d3584; f_800e8684(c);
                pc = 0x800d3584;
            }
            0x800d3584 => {
                c.lr = 0x800d3588; f_800a2790(c);
                pc = 0x800d3588;
            }
            0x800d3588 => {
                c.r[4] = 0x80400000;
                c.r[0] = c.r[3].wrapping_sub(c.r[31]);
                c.r[4] = c.r[4].wrapping_add(0x1f0);
                c.r[3] = 0x0;
                c.stw(c.r[4].wrapping_add(0x3c), c.r[0]);
                c.lr = 0x800d35a0; f_8019c708(c);
                pc = 0x800d35a0;
            }
            0x800d35a0 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x19));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d35e8; continue; }
                pc = 0x800d35b0;
            }
            0x800d35b0 => {
                c.r[3] = c.lbz(c.r[6].wrapping_add(0x1c));
                c.r[4] = c.lbz(c.r[6].wrapping_add(0x1d));
                c.r[5] = c.lbz(c.r[6].wrapping_add(0x1e));
                c.r[6] = c.lbz(c.r[6].wrapping_add(0x1f));
                c.lr = 0x800d35c4; f_8019731c(c);
                pc = 0x800d35c4;
            }
            0x800d35c4 => {
                { let ea = c.r[2].wrapping_add(0xffff9360); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x1;
                { let ea = c.r[2].wrapping_add(0xffff9368); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[4] = 0x1;
                c.f[3] = c.f[1];
                { let ea = c.r[2].wrapping_add(0xffff9364); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.f[5] = c.f[1];
                c.r[5] = 0x0;
                c.lr = 0x800d35e8; f_80196ef8(c);
                pc = 0x800d35e8;
            }
            0x800d35e8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x49d), c.r[0]);
                pc = 0x800d3600; continue;
                pc = 0x800d35f8;
            }
            0x800d35f8 => {
                c.r[3] = 0x3;
                c.lr = 0x800d3600; f_8019c708(c);
                pc = 0x800d3600;
            }
            0x800d3600 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d3410: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D361C (0x800d361c, 152 bytes)
#[inline(never)]
pub fn f_800d361c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d361c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d361c); f_800d361c_b(c, 0x800d361c); crate::prof::exit(c, 0x800d361c); return; }
    }
    f_800d361c_b(c, 0x800d361c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d361c_at(c: &mut Cpu, pc: u32) { f_800d361c_b(c, pc) }
#[inline(always)]
fn f_800d361c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d361c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d361c, pc); }
        match pc {
            0x800d361c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800d3634; f_800a8fe4(c);
                pc = 0x800d3634;
            }
            0x800d3634 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x50));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4c));
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[4].wrapping_add(0x54), c.r[0]);
                if !c.crbit(2) { pc = 0x800d3694; continue; }
                pc = 0x800d3654;
            }
            0x800d3654 => {
                pc = 0x800d3670; continue;
                pc = 0x800d3658;
            }
            0x800d3658 => {
                c.lr = 0x800d365c; f_800a8fe4(c);
                pc = 0x800d365c;
            }
            0x800d365c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x50));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4c));
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.stw(c.r[4].wrapping_add(0x54), c.r[0]);
                pc = 0x800d3670;
            }
            0x800d3670 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x54));
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x58));
                c.cmpu(0, c.r[0], c.r[4]);
                if c.crbit(0) { pc = 0x800d3658; continue; }
                pc = 0x800d3684;
            }
            0x800d3684 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x5c));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d3694; continue; }
                pc = 0x800d3690;
            }
            0x800d3690 => {
                c.stw(c.r[3].wrapping_add(0x54), c.r[4]);
                pc = 0x800d3694;
            }
            0x800d3694 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4c));
                c.stw(c.r[3].wrapping_add(0x50), c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d361c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D36B4 (0x800d36b4, 200 bytes)
#[inline(never)]
pub fn f_800d36b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d36b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d36b4); f_800d36b4_b(c, 0x800d36b4); crate::prof::exit(c, 0x800d36b4); return; }
    }
    f_800d36b4_b(c, 0x800d36b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d36b4_at(c: &mut Cpu, pc: u32) { f_800d36b4_b(c, pc) }
#[inline(always)]
fn f_800d36b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d36b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d36b4, pc); }
        match pc {
            0x800d36b4 => {
                { let ea = c.r[2].wrapping_add(0xffff9350); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.r[0] = 0x1;
                { let ea = c.r[3]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let v = s(c.f[4] * c.f[1]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[3].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[4] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[3] = fctiwz(c.f[2]);
                { let v = s(c.f[4] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.stb(c.r[4].wrapping_add(0x19), c.r[0]);
                c.f[2] = fctiwz(c.f[0]);
                { let ea = c.r[3].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[3]; c.stfd(ea, v); }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xc));
                c.f[1] = fctiwz(c.f[1]);
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[2]; c.stfd(ea, v); }
                { let v = s(c.f[4] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.stb(c.r[3].wrapping_add(0x1c), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.f[0] = fctiwz(c.f[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[1]; c.stfd(ea, v); }
                c.stb(c.r[3].wrapping_add(0x1d), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.stb(c.r[3].wrapping_add(0x1e), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x1f), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d3774; continue; }
                pc = 0x800d3748;
            }
            0x800d3748 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1d));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d3774; continue; }
                pc = 0x800d3754;
            }
            0x800d3754 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d3774; continue; }
                pc = 0x800d3760;
            }
            0x800d3760 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1f));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d3774; continue; }
                pc = 0x800d376c;
            }
            0x800d376c => {
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x19), c.r[0]);
                pc = 0x800d3774;
            }
            0x800d3774 => {
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_800d36b4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D377C (0x800d377c, 80 bytes)
#[inline(never)]
pub fn f_800d377c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d377c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d377c); f_800d377c_b(c, 0x800d377c); crate::prof::exit(c, 0x800d377c); return; }
    }
    f_800d377c_b(c, 0x800d377c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d377c_at(c: &mut Cpu, pc: u32) { f_800d377c_b(c, pc) }
#[inline(always)]
fn f_800d377c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d377c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d377c, pc); }
        match pc {
            0x800d377c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[3], 0x2);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if c.crbit(2) { pc = 0x800d37b0; continue; }
                pc = 0x800d3790;
            }
            0x800d3790 => {
                if !c.crbit(0) { pc = 0x800d37bc; continue; }
                pc = 0x800d3794;
            }
            0x800d3794 => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(0) { pc = 0x800d37a0; continue; }
                pc = 0x800d379c;
            }
            0x800d379c => {
                pc = 0x800d37bc; continue;
                pc = 0x800d37a0;
            }
            0x800d37a0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x800d37ac; f_8019c690(c);
                pc = 0x800d37ac;
            }
            0x800d37ac => {
                pc = 0x800d37bc; continue;
                pc = 0x800d37b0;
            }
            0x800d37b0 => {
                c.r[3] = 0x1;
                c.r[4] = 0x0;
                c.lr = 0x800d37bc; f_8019c690(c);
                pc = 0x800d37bc;
            }
            0x800d37bc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d377c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D37CC (0x800d37cc, 8 bytes)
#[inline(never)]
pub fn f_800d37cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d37cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800d37cc); f_800d37cc_b(c, 0x800d37cc); crate::prof::exit(c, 0x800d37cc); return; }
    }
    f_800d37cc_b(c, 0x800d37cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d37cc_at(c: &mut Cpu, pc: u32) { f_800d37cc_b(c, pc) }
#[inline(always)]
fn f_800d37cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d37cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d37cc, pc); }
        match pc {
            0x800d37cc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa274));
                if true { return; }
                return;
            }
            _ => panic!("f_800d37cc: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D37D4 (0x800d37d4, 524 bytes)
#[inline(never)]
pub fn f_800d37d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d37d4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d37d4); f_800d37d4_b(c, 0x800d37d4); crate::prof::exit(c, 0x800d37d4); return; }
    }
    f_800d37d4_b(c, 0x800d37d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d37d4_at(c: &mut Cpu, pc: u32) { f_800d37d4_b(c, pc) }
#[inline(always)]
fn f_800d37d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d37d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d37d4, pc); }
        match pc {
            0x800d37d4 => {
                { let ea = c.r[1].wrapping_add(0xffffffb0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[9] = 0x80310000;
                c.stw(c.r[1].wrapping_add(0x54), c.r[0]);
                c.r[0] = c.r[7].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                c.stw(c.r[1].wrapping_add(0x4c), c.r[31]);
                c.r[31] = c.r[8] | c.r[8];
                c.stw(c.r[1].wrapping_add(0x48), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[9].wrapping_add(0x40f8);
                if !c.crbit(2) { pc = 0x800d3824; continue; }
                pc = 0x800d3804;
            }
            0x800d3804 => {
                c.cmps(0, c.r[30], 0x1);
                if c.crbit(2) { pc = 0x800d3814; continue; }
                pc = 0x800d380c;
            }
            0x800d380c => {
                c.r[3] = 0x0;
                pc = 0x800d39c8; continue;
                pc = 0x800d3814;
            }
            0x800d3814 => {
                c.cmps(0, c.r[4], 0x2);
                if c.crbit(2) { pc = 0x800d3824; continue; }
                pc = 0x800d381c;
            }
            0x800d381c => {
                c.r[3] = 0x0;
                pc = 0x800d39c8; continue;
                pc = 0x800d3824;
            }
            0x800d3824 => {
                c.cmps(0, c.r[30], 0x2);
                if !c.crbit(2) { pc = 0x800d3840; continue; }
                pc = 0x800d382c;
            }
            0x800d382c => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa27c));
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x800d3840; continue; }
                pc = 0x800d3838;
            }
            0x800d3838 => {
                c.r[3] = 0x0;
                pc = 0x800d39c8; continue;
                pc = 0x800d3840;
            }
            0x800d3840 => {
                c.cmps(0, c.r[4], 0x1);
                if !c.crbit(2) { pc = 0x800d3880; continue; }
                pc = 0x800d3848;
            }
            0x800d3848 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800d3868; continue; }
                pc = 0x800d3850;
            }
            0x800d3850 => {
                c.cmps(0, c.r[6], 0x3);
                if c.crbit(2) { pc = 0x800d3860; continue; }
                pc = 0x800d3858;
            }
            0x800d3858 => {
                c.r[5] = 0x0;
                pc = 0x800d38d8; continue;
                pc = 0x800d3860;
            }
            0x800d3860 => {
                c.r[5] = 0x1;
                pc = 0x800d38d8; continue;
                pc = 0x800d3868;
            }
            0x800d3868 => {
                c.cmps(0, c.r[6], 0x3);
                if c.crbit(2) { pc = 0x800d3878; continue; }
                pc = 0x800d3870;
            }
            0x800d3870 => {
                c.r[5] = 0x2;
                pc = 0x800d38d8; continue;
                pc = 0x800d3878;
            }
            0x800d3878 => {
                c.r[5] = 0x3;
                pc = 0x800d38d8; continue;
                pc = 0x800d3880;
            }
            0x800d3880 => {
                c.r[0] = c.r[7].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800d38c4; continue; }
                pc = 0x800d3888;
            }
            0x800d3888 => {
                c.cmps(0, c.r[6], 0x2);
                if c.crbit(2) { pc = 0x800d38b4; continue; }
                pc = 0x800d3890;
            }
            0x800d3890 => {
                if !c.crbit(0) { pc = 0x800d38a0; continue; }
                pc = 0x800d3894;
            }
            0x800d3894 => {
                c.cmps(0, c.r[6], 0x1);
                if !c.crbit(0) { pc = 0x800d38ac; continue; }
                pc = 0x800d389c;
            }
            0x800d389c => {
                pc = 0x800d38d8; continue;
                pc = 0x800d38a0;
            }
            0x800d38a0 => {
                c.cmps(0, c.r[6], 0x4);
                if !c.crbit(0) { pc = 0x800d38d8; continue; }
                pc = 0x800d38a8;
            }
            0x800d38a8 => {
                pc = 0x800d38bc; continue;
                pc = 0x800d38ac;
            }
            0x800d38ac => {
                c.r[5] = 0x4;
                pc = 0x800d38d8; continue;
                pc = 0x800d38b4;
            }
            0x800d38b4 => {
                c.r[5] = 0x5;
                pc = 0x800d38d8; continue;
                pc = 0x800d38bc;
            }
            0x800d38bc => {
                c.r[5] = 0x6;
                pc = 0x800d38d8; continue;
                pc = 0x800d38c4;
            }
            0x800d38c4 => {
                c.cmps(0, c.r[6], 0x3);
                if c.crbit(2) { pc = 0x800d38d4; continue; }
                pc = 0x800d38cc;
            }
            0x800d38cc => {
                c.r[5] = 0x7;
                pc = 0x800d38d8; continue;
                pc = 0x800d38d4;
            }
            0x800d38d4 => {
                c.r[5] = 0x8;
                pc = 0x800d38d8;
            }
            0x800d38d8 => {
                c.cmps(0, c.r[30], 0x3);
                if c.crbit(2) { pc = 0x800d3920; continue; }
                pc = 0x800d38e0;
            }
            0x800d38e0 => {
                if !c.crbit(0) { pc = 0x800d38f4; continue; }
                pc = 0x800d38e4;
            }
            0x800d38e4 => {
                c.cmps(0, c.r[30], 0x1);
                if c.crbit(2) { pc = 0x800d3900; continue; }
                pc = 0x800d38ec;
            }
            0x800d38ec => {
                if !c.crbit(0) { pc = 0x800d3910; continue; }
                pc = 0x800d38f0;
            }
            0x800d38f0 => {
                pc = 0x800d393c; continue;
                pc = 0x800d38f4;
            }
            0x800d38f4 => {
                c.cmps(0, c.r[30], 0x5);
                if !c.crbit(0) { pc = 0x800d393c; continue; }
                pc = 0x800d38fc;
            }
            0x800d38fc => {
                pc = 0x800d3930; continue;
                pc = 0x800d3900;
            }
            0x800d3900 => {
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(0x0);
                c.r[4] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x800d393c; continue;
                pc = 0x800d3910;
            }
            0x800d3910 => {
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(0x24);
                c.r[4] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x800d393c; continue;
                pc = 0x800d3920;
            }
            0x800d3920 => {
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(0x48);
                c.r[4] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x800d393c; continue;
                pc = 0x800d3930;
            }
            0x800d3930 => {
                c.r[0] = c.r[5].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(0x6c);
                c.r[4] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x800d393c;
            }
            0x800d393c => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x800d394c; continue; }
                pc = 0x800d3944;
            }
            0x800d3944 => {
                c.r[3] = 0x0;
                pc = 0x800d39c8; continue;
                pc = 0x800d394c;
            }
            0x800d394c => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x3c;
                c.lr = 0x800d3958; f_800054f4(c);
                pc = 0x800d3958;
            }
            0x800d3958 => {
                c.cmps(0, c.r[30], 0x2);
                c.r[0] = 0x3c;
                if !c.crbit(2) { pc = 0x800d3968; continue; }
                pc = 0x800d3964;
            }
            0x800d3964 => {
                c.r[0] = 0x32;
                pc = 0x800d3968;
            }
            0x800d3968 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa280));
                c.stw(c.r[13].wrapping_add(0xffffa274), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa278), c.r[30]);
                if c.crbit(2) { pc = 0x800d3980; continue; }
                pc = 0x800d397c;
            }
            0x800d397c => {
                c.sth(c.r[1].wrapping_add(0xe), c.r[3]);
                pc = 0x800d3980;
            }
            0x800d3980 => {
                c.r[3] = 0x1e;
                c.r[0] = 0x294;
                c.sth(c.r[1].wrapping_add(0x12), c.r[3]);
                c.r[3] = 0x1;
                c.sth(c.r[1].wrapping_add(0x16), c.r[0]);
                c.lr = 0x800d3998; f_801bf4c4(c);
                pc = 0x800d3998;
            }
            0x800d3998 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x800d39a0; f_801bf4e4(c);
                pc = 0x800d39a0;
            }
            0x800d39a0 => {
                c.r[30] = c.r[31].rotate_left(0) & 0xffff;
                c.r[31] = 0x0;
                pc = 0x800d39b4; continue;
                pc = 0x800d39ac;
            }
            0x800d39ac => {
                c.lr = 0x800d39b0; f_800a8fe4(c);
                pc = 0x800d39b0;
            }
            0x800d39b0 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x800d39b4;
            }
            0x800d39b4 => {
                c.cmps(0, c.r[31], c.r[30]);
                if c.crbit(0) { pc = 0x800d39ac; continue; }
                pc = 0x800d39bc;
            }
            0x800d39bc => {
                c.r[3] = 0x0;
                c.lr = 0x800d39c4; f_801bf4c4(c);
                pc = 0x800d39c4;
            }
            0x800d39c4 => {
                c.r[3] = 0x1;
                pc = 0x800d39c8;
            }
            0x800d39c8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x54));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x4c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x48));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x50);
                if true { return; }
                return;
            }
            _ => panic!("f_800d37d4: bad pc {:#x}", pc),
        }
    }
}

/// GSgfxInit__FP15_GSgfxInitParms (0x800d39e0, 1132 bytes)
#[inline(never)]
pub fn f_800d39e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d39e0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d39e0); f_800d39e0_b(c, 0x800d39e0); crate::prof::exit(c, 0x800d39e0); return; }
    }
    f_800d39e0_b(c, 0x800d39e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d39e0_at(c: &mut Cpu, pc: u32) { f_800d39e0_b(c, pc) }
#[inline(always)]
fn f_800d39e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d39e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d39e0, pc); }
        match pc {
            0x800d39e0 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[28] = c.r[6] | c.r[6];
                c.r[29] = c.r[7] | c.r[7];
                c.r[30] = c.r[8] | c.r[8];
                c.lr = 0x800d3a0c; f_800e0790(c);
                pc = 0x800d3a0c;
            }
            0x800d3a0c => {
                c.r[4] = 0x800d0000;
                c.r[3] = 0x3;
                c.r[4] = c.r[4].wrapping_add(0x3e4c);
                c.lr = 0x800d3a1c; f_800eec38(c);
                pc = 0x800d3a1c;
            }
            0x800d3a1c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800d3a4c; continue; }
                pc = 0x800d3a24;
            }
            0x800d3a24 => {
                c.r[3] = 0x5a0;
                c.lr = 0x800d3a2c; f_800e3534(c);
                pc = 0x800d3a2c;
            }
            0x800d3a2c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x800d3a48; continue; }
                pc = 0x800d3a34;
            }
            0x800d3a34 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x360);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d3a44; stubs_gen::f_800dd970(c);
                pc = 0x800d3a44;
            }
            0x800d3a44 => {
                pc = 0x800d3e38; continue;
                pc = 0x800d3a48;
            }
            0x800d3a48 => {
                c.lr = 0x800d3a4c; f_800e27b0(c);
                pc = 0x800d3a4c;
            }
            0x800d3a4c => {
                c.stw(c.r[13].wrapping_add(0xffffa260), c.r[3]);
                c.r[0] = 0x2;
                c.r[5] = 0xfeff0000;
                c.r[8] = 0x10;
                c.stw(c.r[3], c.r[0]);
                c.r[7] = c.r[5].wrapping_add(0xfffffefe);
                c.r[3] = 0xffffffff;
                c.r[0] = 0x0;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[6] = 0x3;
                c.r[4] = 0x80400000;
                c.cmps(0, c.r[29], 0x4);
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x8), c.r[8]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0xc), c.r[7]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x10), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x14), c.r[6]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[5].wrapping_add(0x18), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[5].wrapping_add(0x19), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[5].wrapping_add(0x1a), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[5].wrapping_add(0x1b), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[5].wrapping_add(0x49c), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[5].wrapping_add(0x49d), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x20), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x24), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x28), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x2c), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x30), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x34), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x38), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x3c), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x40), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x44), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[5].wrapping_add(0x47e), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x480), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x484), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x488), c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                { let ea = c.r[4].wrapping_add(0x1f0); c.stw(ea, c.r[0]); c.r[4] = ea; }
                c.stw(c.r[4].wrapping_add(0x4), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x10), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x18), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x1c), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x20), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x28), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x2c), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x30), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x34), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x38), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x3c), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x40), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x44), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x48), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x4c), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x50), c.r[0]);
                c.stw(c.r[4].wrapping_add(0x54), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa280), c.r[30]);
                if c.crbit(2) { pc = 0x800d3bdc; continue; }
                pc = 0x800d3ba8;
            }
            0x800d3ba8 => {
                if !c.crbit(0) { pc = 0x800d3be8; continue; }
                pc = 0x800d3bac;
            }
            0x800d3bac => {
                c.cmps(0, c.r[29], 0x1);
                if c.crbit(2) { pc = 0x800d3bbc; continue; }
                pc = 0x800d3bb4;
            }
            0x800d3bb4 => {
                if !c.crbit(0) { pc = 0x800d3bcc; continue; }
                pc = 0x800d3bb8;
            }
            0x800d3bb8 => {
                pc = 0x800d3be8; continue;
                pc = 0x800d3bbc;
            }
            0x800d3bbc => {
                c.r[3] = 0x80310000;
                c.r[0] = c.r[3].wrapping_add(0x2d30);
                c.r[31] = c.r[0] | c.r[0];
                pc = 0x800d3be8; continue;
                pc = 0x800d3bcc;
            }
            0x800d3bcc => {
                c.r[3] = 0x80310000;
                c.r[0] = c.r[3].wrapping_add(0x30f0);
                c.r[31] = c.r[0] | c.r[0];
                pc = 0x800d3be8; continue;
                pc = 0x800d3bdc;
            }
            0x800d3bdc => {
                c.r[3] = 0x80310000;
                c.r[0] = c.r[3].wrapping_add(0x2f4c);
                c.r[31] = c.r[0] | c.r[0];
                pc = 0x800d3be8;
            }
            0x800d3be8 => {
                c.cmps(0, c.r[29], 0x2);
                if c.crbit(2) { pc = 0x800d3bf8; continue; }
                pc = 0x800d3bf0;
            }
            0x800d3bf0 => {
                c.cmps(0, c.r[29], 0x3);
                if !c.crbit(2) { pc = 0x800d3c04; continue; }
                pc = 0x800d3bf8;
            }
            0x800d3bf8 => {
                c.r[0] = 0x1;
                c.stb(c.r[13].wrapping_add(0xffffa27c), c.r[0]);
                pc = 0x800d3c0c; continue;
                pc = 0x800d3c04;
            }
            0x800d3c04 => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa27c), c.r[0]);
                pc = 0x800d3c0c;
            }
            0x800d3c0c => {
                c.r[3] = 0x1;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d3c1c; f_8019c3c4(c);
                pc = 0x800d3c1c;
            }
            0x800d3c1c => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x4;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d3c2c; f_8019c3c4(c);
                pc = 0x800d3c2c;
            }
            0x800d3c2c => {
                c.lr = 0x800d3c30; f_8019cb70(c);
                pc = 0x800d3c30;
            }
            0x800d3c30 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x2;
                c.r[5] = 0x0;
                c.r[6] = 0x2;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x800d3c4c; f_800d37d4(c);
                pc = 0x800d3c4c;
            }
            0x800d3c4c => {
                c.r[3] = 0x1;
                c.lr = 0x800d3c54; f_801bf4c4(c);
                pc = 0x800d3c54;
            }
            0x800d3c54 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x800d3c60; f_8019c690(c);
                pc = 0x800d3c60;
            }
            0x800d3c60 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = 0x0;
                c.r[3] = 0x800d0000;
                c.r[0] = 0x1;
                c.stw(c.r[4].wrapping_add(0x48), c.r[5]);
                c.r[3] = c.r[3].wrapping_add(0x3f5c);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[4].wrapping_add(0x4c), c.r[5]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[4].wrapping_add(0x50), c.r[5]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[4].wrapping_add(0x54), c.r[5]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[4].wrapping_add(0x58), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x5c), c.r[0]);
                c.lr = 0x800d3ca4; f_801c021c(c);
                pc = 0x800d3ca4;
            }
            0x800d3ca4 => {
                c.r[3] = 0x800d0000;
                c.r[3] = c.r[3].wrapping_add(0x3f50);
                c.lr = 0x800d3cb0; f_801c01c8(c);
                pc = 0x800d3cb0;
            }
            0x800d3cb0 => {
                c.r[3] = 0x800d0000;
                c.r[3] = c.r[3].wrapping_add(0x3ec4);
                c.lr = 0x800d3cbc; f_80196c3c(c);
                pc = 0x800d3cbc;
            }
            0x800d3cbc => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x800d3cc4; f_800d5504(c);
                pc = 0x800d3cc4;
            }
            0x800d3cc4 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x800d3ccc; f_800d83e4(c);
                pc = 0x800d3ccc;
            }
            0x800d3ccc => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x800d3cd4; f_800d7b80(c);
                pc = 0x800d3cd4;
            }
            0x800d3cd4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x800d3cdc; f_800db890(c);
                pc = 0x800d3cdc;
            }
            0x800d3cdc => {
                c.lr = 0x800d3ce0; f_800d7894(c);
                pc = 0x800d3ce0;
            }
            0x800d3ce0 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = 0x80460000;
                c.r[0] = 0x2;
                c.r[4] = 0x0;
                c.stw(c.r[6].wrapping_add(0x20), c.r[3]);
                c.r[25] = c.r[5].wrapping_add(0x6bc0);
                c.r[3] = 0x0;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[26] = c.lwz(c.r[5]);
                c.stw(c.r[5], c.r[0]);
                c.r[6] = c.lhz(c.r[25].wrapping_add(0x4));
                c.r[5] = c.lhz(c.r[25].wrapping_add(0x6));
                c.r[6] = c.r[6].wrapping_add(0xffffffff);
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                c.r[5] = c.r[6].rotate_left(0) & 0xffff;
                c.r[6] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x800d3d24; f_800d9d68(c);
                pc = 0x800d3d24;
            }
            0x800d3d24 => {
                c.r[6] = c.lhz(c.r[25].wrapping_add(0x4));
                c.r[3] = 0x0;
                c.r[5] = c.lhz(c.r[25].wrapping_add(0x6));
                c.r[4] = 0x0;
                c.r[6] = c.r[6].wrapping_add(0xffffffff);
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                c.r[5] = c.r[6].rotate_left(0) & 0xffff;
                c.r[6] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x800d3d48; f_800d9c24(c);
                pc = 0x800d3d48;
            }
            0x800d3d48 => {
                c.r[3] = 0xffffffff;
                c.lr = 0x800d3d50; f_800d87ac(c);
                pc = 0x800d3d50;
            }
            0x800d3d50 => {
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x800d3d60; f_800da2bc(c);
                pc = 0x800d3d60;
            }
            0x800d3d60 => {
                c.r[3] = 0x1;
                c.r[4] = 0x2;
                c.r[5] = 0x1;
                c.lr = 0x800d3d70; f_800da1e8(c);
                pc = 0x800d3d70;
            }
            0x800d3d70 => {
                c.r[3] = 0x0;
                c.r[4] = 0x7;
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.r[7] = 0x7;
                c.r[8] = 0x0;
                c.lr = 0x800d3d8c; f_800da100(c);
                pc = 0x800d3d8c;
            }
            0x800d3d8c => {
                c.r[3] = 0x2;
                c.lr = 0x800d3d94; f_800da028(c);
                pc = 0x800d3d94;
            }
            0x800d3d94 => {
                c.r[3] = 0x0;
                c.lr = 0x800d3d9c; f_800d9f40(c);
                pc = 0x800d3d9c;
            }
            0x800d3d9c => {
                c.r[3] = 0x1;
                c.lr = 0x800d3da4; f_800da08c(c);
                pc = 0x800d3da4;
            }
            0x800d3da4 => {
                c.r[3] = 0x0;
                c.lr = 0x800d3dac; f_800d9ed8(c);
                pc = 0x800d3dac;
            }
            0x800d3dac => {
                c.r[25] = 0x0;
                pc = 0x800d3db0;
            }
            0x800d3db0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[5] = c.r[25] | c.r[25];
                c.r[6] = c.r[25] | c.r[25];
                c.r[4] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x800d3dc8; f_800dc224(c);
                pc = 0x800d3dc8;
            }
            0x800d3dc8 => {
                c.r[25] = c.r[25].wrapping_add(0x1);
                c.cmps(0, c.r[25], 0x10);
                if c.crbit(0) { pc = 0x800d3db0; continue; }
                pc = 0x800d3dd4;
            }
            0x800d3dd4 => {
                { let ea = c.r[2].wrapping_add(0xffff9370); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9374); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[2].wrapping_add(0xffff9378); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[2].wrapping_add(0xffff937c); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.lr = 0x800d3de8; f_800d9bd0(c);
                pc = 0x800d3de8;
            }
            0x800d3de8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0x0;
                c.r[0] = 0x1;
                c.stw(c.r[4], c.r[26]);
                c.stw(c.r[13].wrapping_add(0xffffa26c), c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[3], c.r[0]);
                c.lr = 0x800d3e08; f_800dc540(c);
                pc = 0x800d3e08;
            }
            0x800d3e08 => {
                c.r[3] = 0x0;
                c.lr = 0x800d3e10; f_801bf4c4(c);
                pc = 0x800d3e10;
            }
            0x800d3e10 => {
                c.r[4] = 0x1;
                c.r[0] = 0x0;
                c.r[3] = 0x80270000;
                c.stb(c.r[13].wrapping_add(0xffffa264), c.r[4]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[3].wrapping_add(0x388);
                c.stb(c.r[13].wrapping_add(0xffffa265), c.r[0]);
                c.r[5] = 0x5a0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d3e38; stubs_gen::f_800dd970(c);
                pc = 0x800d3e38;
            }
            0x800d3e38 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_800d39e0: bad pc {:#x}", pc),
        }
    }
}

/// _gfxScratchNotify__F15GSscratchNotifyPvUc (0x800d3e4c, 120 bytes)
#[inline(never)]
pub fn f_800d3e4c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3e4c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3e4c); f_800d3e4c_b(c, 0x800d3e4c); crate::prof::exit(c, 0x800d3e4c); return; }
    }
    f_800d3e4c_b(c, 0x800d3e4c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3e4c_at(c: &mut Cpu, pc: u32) { f_800d3e4c_b(c, pc) }
#[inline(always)]
fn f_800d3e4c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3e4c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3e4c, pc); }
        match pc {
            0x800d3e4c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[3], 0x1);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                if c.crbit(2) { pc = 0x800d3e98; continue; }
                pc = 0x800d3e68;
            }
            0x800d3e68 => {
                if !c.crbit(0) { pc = 0x800d3eb0; continue; }
                pc = 0x800d3e6c;
            }
            0x800d3e6c => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x800d3e78; continue; }
                pc = 0x800d3e74;
            }
            0x800d3e74 => {
                pc = 0x800d3eb0; continue;
                pc = 0x800d3e78;
            }
            0x800d3e78 => {
                c.r[3] = 0x80400000;
                c.r[5] = 0x5a0;
                c.r[3] = c.r[3].wrapping_add(0x248);
                c.lr = 0x800d3e88; f_800054f4(c);
                pc = 0x800d3e88;
            }
            0x800d3e88 => {
                c.r[3] = 0x80400000;
                c.r[0] = c.r[3].wrapping_add(0x248);
                c.stw(c.r[13].wrapping_add(0xffffa260), c.r[0]);
                pc = 0x800d3eb0; continue;
                pc = 0x800d3e98;
            }
            0x800d3e98 => {
                c.r[4] = 0x80400000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[4].wrapping_add(0x248);
                c.r[5] = 0x5a0;
                c.lr = 0x800d3eac; f_800054f4(c);
                pc = 0x800d3eac;
            }
            0x800d3eac => {
                c.stw(c.r[13].wrapping_add(0xffffa260), c.r[31]);
                pc = 0x800d3eb0;
            }
            0x800d3eb0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d3e4c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D3EC4 (0x800d3ec4, 140 bytes)
#[inline(never)]
pub fn f_800d3ec4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3ec4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3ec4); f_800d3ec4_b(c, 0x800d3ec4); crate::prof::exit(c, 0x800d3ec4); return; }
    }
    f_800d3ec4_b(c, 0x800d3ec4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3ec4_at(c: &mut Cpu, pc: u32) { f_800d3ec4_b(c, pc) }
#[inline(always)]
fn f_800d3ec4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3ec4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3ec4, pc); }
        match pc {
            0x800d3ec4 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                c.r[11] = c.r[1].wrapping_add(0x40);
                c.lr = 0x800d3ed8; f_800c473c(c);
                pc = 0x800d3ed8;
            }
            0x800d3ed8 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffffa268));
                c.f[26] = c.f[1];
                c.f[27] = c.f[2];
                c.f[28] = c.f[3];
                c.cmpu(0, c.r[12], 0x0);
                c.f[29] = c.f[4];
                c.f[30] = c.f[5];
                c.f[31] = c.f[6];
                if c.crbit(2) { pc = 0x800d3f08; continue; }
                pc = 0x800d3efc;
            }
            0x800d3efc => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d3f04; (c.dispatch)(c, t); }
                pc = 0x800d3f04;
            }
            0x800d3f04 => {
                pc = 0x800d3f38; continue;
                pc = 0x800d3f08;
            }
            0x800d3f08 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d3f34; continue; }
                pc = 0x800d3f10;
            }
            0x800d3f10 => {
                c.lr = 0x800d3f14; f_800aa2f0(c);
                pc = 0x800d3f14;
            }
            0x800d3f14 => {
                c.f[1] = c.f[26];
                c.f[2] = c.f[27];
                c.f[3] = c.f[28];
                c.f[4] = c.f[29];
                c.f[5] = c.f[30];
                c.f[6] = c.f[31];
                c.lr = 0x800d3f30; f_800bd640(c);
                pc = 0x800d3f30;
            }
            0x800d3f30 => {
                pc = 0x800d3f38; continue;
                pc = 0x800d3f34;
            }
            0x800d3f34 => {
                c.lr = 0x800d3f38; f_800bd744(c);
                pc = 0x800d3f38;
            }
            0x800d3f38 => {
                c.r[11] = c.r[1].wrapping_add(0x40);
                c.lr = 0x800d3f40; f_800c4788(c);
                pc = 0x800d3f40;
            }
            0x800d3f40 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_800d3ec4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D3F50 (0x800d3f50, 12 bytes)
#[inline(never)]
pub fn f_800d3f50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3f50) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3f50); f_800d3f50_b(c, 0x800d3f50); crate::prof::exit(c, 0x800d3f50); return; }
    }
    f_800d3f50_b(c, 0x800d3f50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3f50_at(c: &mut Cpu, pc: u32) { f_800d3f50_b(c, pc) }
#[inline(always)]
fn f_800d3f50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3f50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3f50, pc); }
        match pc {
            0x800d3f50 => {
                c.r[0] = 0x1;
                c.stb(c.r[13].wrapping_add(0xffffa271), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d3f50: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D3F5C (0x800d3f5c, 72 bytes)
#[inline(never)]
pub fn f_800d3f5c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3f5c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3f5c); f_800d3f5c_b(c, 0x800d3f5c); crate::prof::exit(c, 0x800d3f5c); return; }
    }
    f_800d3f5c_b(c, 0x800d3f5c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3f5c_at(c: &mut Cpu, pc: u32) { f_800d3f5c_b(c, pc) }
#[inline(always)]
fn f_800d3f5c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3f5c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3f5c, pc); }
        match pc {
            0x800d3f5c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4c));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[4].wrapping_add(0x4c), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[12] = c.lwz(c.r[3].wrapping_add(0x48));
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x800d3f94; continue; }
                pc = 0x800d3f88;
            }
            0x800d3f88 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4c));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d3f94; (c.dispatch)(c, t); }
                pc = 0x800d3f94;
            }
            0x800d3f94 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d3f5c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D3FA4 (0x800d3fa4, 1620 bytes)
#[inline(never)]
pub fn f_800d3fa4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d3fa4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d3fa4); f_800d3fa4_b(c, 0x800d3fa4); crate::prof::exit(c, 0x800d3fa4); return; }
    }
    f_800d3fa4_b(c, 0x800d3fa4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d3fa4_at(c: &mut Cpu, pc: u32) { f_800d3fa4_b(c, pc) }
#[inline(always)]
fn f_800d3fa4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d3fa4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d3fa4, pc); }
        match pc {
            0x800d3fa4 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[6] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.r[6] = c.r[6].wrapping_add(0x1f0);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[3] | c.r[3];
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[8] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x28));
                c.r[7] = c.lwz(c.r[8].wrapping_add(0x490));
                c.r[3] = c.lwz(c.r[8].wrapping_add(0x494));
                c.r[3] = c.r[3].wrapping_sub(c.r[7]);
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x800d3fe8; continue; }
                pc = 0x800d3fe4;
            }
            0x800d3fe4 => {
                c.stw(c.r[6].wrapping_add(0x28), c.r[3]);
                pc = 0x800d3fe8;
            }
            0x800d3fe8 => {
                c.r[29] = c.lwz(c.r[8]);
                c.r[0] = 0x2;
                c.r[28] = c.lwz(c.r[8].wrapping_add(0x4));
                c.stw(c.r[8], c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa26c));
                c.lr = 0x800d4000; f_8019bd18(c);
                pc = 0x800d4000;
            }
            0x800d4000 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = 0x0;
                c.r[3] = 0x80400000;
                c.r[0] = c.r[30].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                c.stb(c.r[4].wrapping_add(0x1b), c.r[5]);
                c.r[31] = c.r[3].wrapping_add(0x1f0);
                c.stw(c.r[31].wrapping_add(0x40), c.r[5]);
                c.stw(c.r[31].wrapping_add(0x44), c.r[5]);
                c.stw(c.r[31].wrapping_add(0x48), c.r[5]);
                if c.crbit(2) { pc = 0x800d402c; continue; }
                pc = 0x800d4028;
            }
            0x800d4028 => {
                c.lr = 0x800d402c; f_801e17a8(c);
                pc = 0x800d402c;
            }
            0x800d402c => {
                c.r[0] = c.r[30].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d40d4; continue; }
                pc = 0x800d4034;
            }
            0x800d4034 => {
                c.lr = 0x800d4038; f_800a2790(c);
                pc = 0x800d4038;
            }
            0x800d4038 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = 0x10;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.r[25] = c.r[3] | c.r[3];
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                if c.crbit(2) { pc = 0x800d40a8; continue; }
                pc = 0x800d4050;
            }
            0x800d4050 => {
                c.lr = 0x800d4054; f_800d2584(c);
                pc = 0x800d4054;
            }
            0x800d4054 => {
                c.r[24] = c.r[3] | c.r[3];
                c.rc(c.r[24]);
                if c.crbit(2) { pc = 0x800d40a8; continue; }
                pc = 0x800d405c;
            }
            0x800d405c => {
                c.r[3] = 0x7f;
                c.lr = 0x800d4064; f_801b25c4(c);
                pc = 0x800d4064;
            }
            0x800d4064 => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0xc));
                c.lr = 0x800d406c; f_80195a6c(c);
                pc = 0x800d406c;
            }
            0x800d406c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d40a0; continue; }
                pc = 0x800d4074;
            }
            0x800d4074 => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0xc));
                c.lr = 0x800d407c; f_800dd174(c);
                pc = 0x800d407c;
            }
            0x800d407c => {
                c.r[3] = 0x10;
                c.r[4] = 0x0;
                c.lr = 0x800d4088; f_800e3604(c);
                pc = 0x800d4088;
            }
            0x800d4088 => {
                c.r[3] = 0x10;
                c.r[4] = 0x0;
                c.lr = 0x800d4094; f_80118104(c);
                pc = 0x800d4094;
            }
            0x800d4094 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa26c));
                c.lr = 0x800d409c; f_8019bd18(c);
                pc = 0x800d409c;
            }
            0x800d409c => {
                c.lr = 0x800d40a0; f_80195a48(c);
                pc = 0x800d40a0;
            }
            0x800d40a0 => {
                c.r[3] = 0xffffffff;
                c.lr = 0x800d40a8; f_800d87ac(c);
                pc = 0x800d40a8;
            }
            0x800d40a8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x490));
                pc = 0x800d40b8; continue;
                pc = 0x800d40b4;
            }
            0x800d40b4 => {
                c.lr = 0x800d40b8; f_800d461c(c);
                pc = 0x800d40b8;
            }
            0x800d40b8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x494));
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x800d40b4; continue; }
                pc = 0x800d40c8;
            }
            0x800d40c8 => {
                c.lr = 0x800d40cc; f_800a2790(c);
                pc = 0x800d40cc;
            }
            0x800d40cc => {
                c.r[0] = c.r[3].wrapping_sub(c.r[25]);
                c.stw(c.r[31].wrapping_add(0x40), c.r[0]);
                pc = 0x800d40d4;
            }
            0x800d40d4 => {
                c.r[0] = c.r[30].rotate_left(0) & 0x1000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d415c; continue; }
                pc = 0x800d40dc;
            }
            0x800d40dc => {
                c.lr = 0x800d40e0; f_800a2790(c);
                pc = 0x800d40e0;
            }
            0x800d40e0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = 0x1000;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.r[25] = c.r[3] | c.r[3];
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                if c.crbit(2) { pc = 0x800d4150; continue; }
                pc = 0x800d40f8;
            }
            0x800d40f8 => {
                c.lr = 0x800d40fc; f_800d2584(c);
                pc = 0x800d40fc;
            }
            0x800d40fc => {
                c.r[24] = c.r[3] | c.r[3];
                c.rc(c.r[24]);
                if c.crbit(2) { pc = 0x800d4150; continue; }
                pc = 0x800d4104;
            }
            0x800d4104 => {
                c.r[3] = 0x7f;
                c.lr = 0x800d410c; f_801b25c4(c);
                pc = 0x800d410c;
            }
            0x800d410c => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0xc));
                c.lr = 0x800d4114; f_80195a6c(c);
                pc = 0x800d4114;
            }
            0x800d4114 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d4148; continue; }
                pc = 0x800d411c;
            }
            0x800d411c => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0xc));
                c.lr = 0x800d4124; f_800dd174(c);
                pc = 0x800d4124;
            }
            0x800d4124 => {
                c.r[3] = 0x1000;
                c.r[4] = 0x0;
                c.lr = 0x800d4130; f_800e3604(c);
                pc = 0x800d4130;
            }
            0x800d4130 => {
                c.r[3] = 0x1000;
                c.r[4] = 0x0;
                c.lr = 0x800d413c; f_80118104(c);
                pc = 0x800d413c;
            }
            0x800d413c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa26c));
                c.lr = 0x800d4144; f_8019bd18(c);
                pc = 0x800d4144;
            }
            0x800d4144 => {
                c.lr = 0x800d4148; f_80195a48(c);
                pc = 0x800d4148;
            }
            0x800d4148 => {
                c.r[3] = 0xffffffff;
                c.lr = 0x800d4150; f_800d87ac(c);
                pc = 0x800d4150;
            }
            0x800d4150 => {
                c.lr = 0x800d4154; f_800a2790(c);
                pc = 0x800d4154;
            }
            0x800d4154 => {
                c.r[0] = c.r[3].wrapping_sub(c.r[25]);
                c.stw(c.r[31].wrapping_add(0x44), c.r[0]);
                pc = 0x800d415c;
            }
            0x800d415c => {
                c.r[0] = c.r[30].rotate_left(0) & 0x2000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d4204; continue; }
                pc = 0x800d4164;
            }
            0x800d4164 => {
                c.lr = 0x800d4168; f_800a2790(c);
                pc = 0x800d4168;
            }
            0x800d4168 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = 0x2000;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.r[25] = c.r[3] | c.r[3];
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                if c.crbit(2) { pc = 0x800d41d8; continue; }
                pc = 0x800d4180;
            }
            0x800d4180 => {
                c.lr = 0x800d4184; f_800d2584(c);
                pc = 0x800d4184;
            }
            0x800d4184 => {
                c.r[24] = c.r[3] | c.r[3];
                c.rc(c.r[24]);
                if c.crbit(2) { pc = 0x800d41d8; continue; }
                pc = 0x800d418c;
            }
            0x800d418c => {
                c.r[3] = 0x7f;
                c.lr = 0x800d4194; f_801b25c4(c);
                pc = 0x800d4194;
            }
            0x800d4194 => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0xc));
                c.lr = 0x800d419c; f_80195a6c(c);
                pc = 0x800d419c;
            }
            0x800d419c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d41d0; continue; }
                pc = 0x800d41a4;
            }
            0x800d41a4 => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0xc));
                c.lr = 0x800d41ac; f_800dd174(c);
                pc = 0x800d41ac;
            }
            0x800d41ac => {
                c.r[3] = 0x2000;
                c.r[4] = 0x0;
                c.lr = 0x800d41b8; f_800e3604(c);
                pc = 0x800d41b8;
            }
            0x800d41b8 => {
                c.r[3] = 0x2000;
                c.r[4] = 0x0;
                c.lr = 0x800d41c4; f_80118104(c);
                pc = 0x800d41c4;
            }
            0x800d41c4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa26c));
                c.lr = 0x800d41cc; f_8019bd18(c);
                pc = 0x800d41cc;
            }
            0x800d41cc => {
                c.lr = 0x800d41d0; f_80195a48(c);
                pc = 0x800d41d0;
            }
            0x800d41d0 => {
                c.r[3] = 0xffffffff;
                c.lr = 0x800d41d8; f_800d87ac(c);
                pc = 0x800d41d8;
            }
            0x800d41d8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x490));
                pc = 0x800d41e8; continue;
                pc = 0x800d41e4;
            }
            0x800d41e4 => {
                c.lr = 0x800d41e8; f_800d461c(c);
                pc = 0x800d41e8;
            }
            0x800d41e8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x494));
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x800d41e4; continue; }
                pc = 0x800d41f8;
            }
            0x800d41f8 => {
                c.lr = 0x800d41fc; f_800a2790(c);
                pc = 0x800d41fc;
            }
            0x800d41fc => {
                c.r[0] = c.r[3].wrapping_sub(c.r[25]);
                c.stw(c.r[31].wrapping_add(0x48), c.r[0]);
                pc = 0x800d4204;
            }
            0x800d4204 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = 0xffffffff;
                c.r[4] = 0x0;
                c.r[5] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[24] = c.lbz(c.r[3].wrapping_add(0x1a));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1b));
                c.stb(c.r[3].wrapping_add(0x1a), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x49c));
                c.lr = 0x800d4234; f_800da2bc(c);
                pc = 0x800d4234;
            }
            0x800d4234 => {
                c.r[3] = 0x1;
                c.r[4] = 0x7;
                c.r[5] = 0x2;
                c.lr = 0x800d4244; f_800da1e8(c);
                pc = 0x800d4244;
            }
            0x800d4244 => {
                c.r[3] = 0x0;
                c.r[4] = 0x7;
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.r[7] = 0x7;
                c.r[8] = 0x0;
                c.lr = 0x800d4260; f_800da100(c);
                pc = 0x800d4260;
            }
            0x800d4260 => {
                c.r[3] = 0x1;
                c.lr = 0x800d4268; f_800d88dc(c);
                pc = 0x800d4268;
            }
            0x800d4268 => {
                c.r[3] = 0x6;
                c.lr = 0x800d4270; f_800d888c(c);
                pc = 0x800d4270;
            }
            0x800d4270 => {
                { let ea = c.r[2].wrapping_add(0xffff9380); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9384); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.f[2] = c.f[1];
                { let ea = c.r[2].wrapping_add(0xffff9388); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.lr = 0x800d4284; f_800d9b58(c);
                pc = 0x800d4284;
            }
            0x800d4284 => {
                c.r[3] = 0x0;
                c.lr = 0x800d428c; f_800d9fb4(c);
                pc = 0x800d428c;
            }
            0x800d428c => {
                c.r[3] = 0x0;
                c.lr = 0x800d4294; f_800da028(c);
                pc = 0x800d4294;
            }
            0x800d4294 => {
                c.r[3] = 0x80310000;
                c.r[3] = c.r[3].wrapping_add(0x47c8);
                c.lr = 0x800d42a0; f_800d7820(c);
                pc = 0x800d42a0;
            }
            0x800d42a0 => {
                c.r[3] = 0x4;
                c.lr = 0x800d42a8; f_800d6a00(c);
                pc = 0x800d42a8;
            }
            0x800d42a8 => {
                c.r[3] = 0x4;
                c.lr = 0x800d42b0; f_800d67bc(c);
                pc = 0x800d42b0;
            }
            0x800d42b0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x752f;
                c.lr = 0x800d42c0; f_800d65cc(c);
                pc = 0x800d42c0;
            }
            0x800d42c0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0xc4;
                c.lr = 0x800d42d8; f_800d5cb8(c);
                pc = 0x800d42d8;
            }
            0x800d42d8 => {
                c.r[3] = 0x280;
                c.r[4] = 0x0;
                c.r[5] = 0x752f;
                c.lr = 0x800d42e8; f_800d65cc(c);
                pc = 0x800d42e8;
            }
            0x800d42e8 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0xc4;
                c.lr = 0x800d4300; f_800d5cb8(c);
                pc = 0x800d4300;
            }
            0x800d4300 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1e0;
                c.r[5] = 0x752f;
                c.lr = 0x800d4310; f_800d65cc(c);
                pc = 0x800d4310;
            }
            0x800d4310 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0xc4;
                c.lr = 0x800d4328; f_800d5cb8(c);
                pc = 0x800d4328;
            }
            0x800d4328 => {
                c.r[3] = 0x280;
                c.r[4] = 0x1e0;
                c.r[5] = 0x752f;
                c.lr = 0x800d4338; f_800d65cc(c);
                pc = 0x800d4338;
            }
            0x800d4338 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0xc4;
                c.lr = 0x800d4350; f_800d5cb8(c);
                pc = 0x800d4350;
            }
            0x800d4350 => {
                c.lr = 0x800d4354; f_800d6728(c);
                pc = 0x800d4354;
            }
            0x800d4354 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.stb(c.r[6].wrapping_add(0x1a), c.r[24]);
                c.lr = 0x800d436c; f_800da2bc(c);
                pc = 0x800d436c;
            }
            0x800d436c => {
                c.r[3] = 0x1;
                c.r[4] = 0x7;
                c.r[5] = 0x2;
                c.lr = 0x800d437c; f_800da1e8(c);
                pc = 0x800d437c;
            }
            0x800d437c => {
                c.r[3] = 0x1;
                c.lr = 0x800d4384; f_800d9fb4(c);
                pc = 0x800d4384;
            }
            0x800d4384 => {
                c.r[0] = 0xfffffefe;
                c.r[3] = 0x80400000;
                c.r[30] = c.r[30] & c.r[0];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = 0x1;
                c.r[31] = c.r[3].wrapping_add(0x1f0);
                c.r[0] = c.r[30].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                c.stb(c.r[4].wrapping_add(0x1b), c.r[5]);
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x4c), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x50), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x54), c.r[0]);
                if c.crbit(2) { pc = 0x800d43bc; continue; }
                pc = 0x800d43b8;
            }
            0x800d43b8 => {
                c.lr = 0x800d43bc; f_801e17a8(c);
                pc = 0x800d43bc;
            }
            0x800d43bc => {
                c.r[0] = c.r[30].rotate_left(0) & 0x10;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d4464; continue; }
                pc = 0x800d43c4;
            }
            0x800d43c4 => {
                c.lr = 0x800d43c8; f_800a2790(c);
                pc = 0x800d43c8;
            }
            0x800d43c8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = 0x10;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.r[25] = c.r[3] | c.r[3];
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                if c.crbit(2) { pc = 0x800d4438; continue; }
                pc = 0x800d43e0;
            }
            0x800d43e0 => {
                c.lr = 0x800d43e4; f_800d2584(c);
                pc = 0x800d43e4;
            }
            0x800d43e4 => {
                c.r[24] = c.r[3] | c.r[3];
                c.rc(c.r[24]);
                if c.crbit(2) { pc = 0x800d4438; continue; }
                pc = 0x800d43ec;
            }
            0x800d43ec => {
                c.r[3] = 0x7f;
                c.lr = 0x800d43f4; f_801b25c4(c);
                pc = 0x800d43f4;
            }
            0x800d43f4 => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0xc));
                c.lr = 0x800d43fc; f_80195a6c(c);
                pc = 0x800d43fc;
            }
            0x800d43fc => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d4430; continue; }
                pc = 0x800d4404;
            }
            0x800d4404 => {
                c.r[3] = c.lwz(c.r[24].wrapping_add(0xc));
                c.lr = 0x800d440c; f_800dd174(c);
                pc = 0x800d440c;
            }
            0x800d440c => {
                c.r[3] = 0x10;
                c.r[4] = 0x1;
                c.lr = 0x800d4418; f_800e3604(c);
                pc = 0x800d4418;
            }
            0x800d4418 => {
                c.r[3] = 0x10;
                c.r[4] = 0x1;
                c.lr = 0x800d4424; f_80118104(c);
                pc = 0x800d4424;
            }
            0x800d4424 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa26c));
                c.lr = 0x800d442c; f_8019bd18(c);
                pc = 0x800d442c;
            }
            0x800d442c => {
                c.lr = 0x800d4430; f_80195a48(c);
                pc = 0x800d4430;
            }
            0x800d4430 => {
                c.r[3] = 0xffffffff;
                c.lr = 0x800d4438; f_800d87ac(c);
                pc = 0x800d4438;
            }
            0x800d4438 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x490));
                pc = 0x800d4448; continue;
                pc = 0x800d4444;
            }
            0x800d4444 => {
                c.lr = 0x800d4448; f_800d461c(c);
                pc = 0x800d4448;
            }
            0x800d4448 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x494));
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x800d4444; continue; }
                pc = 0x800d4458;
            }
            0x800d4458 => {
                c.lr = 0x800d445c; f_800a2790(c);
                pc = 0x800d445c;
            }
            0x800d445c => {
                c.r[0] = c.r[3].wrapping_sub(c.r[25]);
                c.stw(c.r[31].wrapping_add(0x4c), c.r[0]);
                pc = 0x800d4464;
            }
            0x800d4464 => {
                c.r[0] = c.r[30].rotate_left(0) & 0x1000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d44ec; continue; }
                pc = 0x800d446c;
            }
            0x800d446c => {
                c.lr = 0x800d4470; f_800a2790(c);
                pc = 0x800d4470;
            }
            0x800d4470 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = 0x1000;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.r[24] = c.r[3] | c.r[3];
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                if c.crbit(2) { pc = 0x800d44e0; continue; }
                pc = 0x800d4488;
            }
            0x800d4488 => {
                c.lr = 0x800d448c; f_800d2584(c);
                pc = 0x800d448c;
            }
            0x800d448c => {
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if c.crbit(2) { pc = 0x800d44e0; continue; }
                pc = 0x800d4494;
            }
            0x800d4494 => {
                c.r[3] = 0x7f;
                c.lr = 0x800d449c; f_801b25c4(c);
                pc = 0x800d449c;
            }
            0x800d449c => {
                c.r[3] = c.lwz(c.r[25].wrapping_add(0xc));
                c.lr = 0x800d44a4; f_80195a6c(c);
                pc = 0x800d44a4;
            }
            0x800d44a4 => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d44d8; continue; }
                pc = 0x800d44ac;
            }
            0x800d44ac => {
                c.r[3] = c.lwz(c.r[25].wrapping_add(0xc));
                c.lr = 0x800d44b4; f_800dd174(c);
                pc = 0x800d44b4;
            }
            0x800d44b4 => {
                c.r[3] = 0x1000;
                c.r[4] = 0x1;
                c.lr = 0x800d44c0; f_800e3604(c);
                pc = 0x800d44c0;
            }
            0x800d44c0 => {
                c.r[3] = 0x1000;
                c.r[4] = 0x1;
                c.lr = 0x800d44cc; f_80118104(c);
                pc = 0x800d44cc;
            }
            0x800d44cc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa26c));
                c.lr = 0x800d44d4; f_8019bd18(c);
                pc = 0x800d44d4;
            }
            0x800d44d4 => {
                c.lr = 0x800d44d8; f_80195a48(c);
                pc = 0x800d44d8;
            }
            0x800d44d8 => {
                c.r[3] = 0xffffffff;
                c.lr = 0x800d44e0; f_800d87ac(c);
                pc = 0x800d44e0;
            }
            0x800d44e0 => {
                c.lr = 0x800d44e4; f_800a2790(c);
                pc = 0x800d44e4;
            }
            0x800d44e4 => {
                c.r[0] = c.r[3].wrapping_sub(c.r[24]);
                c.stw(c.r[31].wrapping_add(0x50), c.r[0]);
                pc = 0x800d44ec;
            }
            0x800d44ec => {
                c.r[0] = c.r[30].rotate_left(0) & 0x2000;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d4594; continue; }
                pc = 0x800d44f4;
            }
            0x800d44f4 => {
                c.lr = 0x800d44f8; f_800a2790(c);
                pc = 0x800d44f8;
            }
            0x800d44f8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = 0x2000;
                c.r[0] = c.r[26].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                if c.crbit(2) { pc = 0x800d4568; continue; }
                pc = 0x800d4510;
            }
            0x800d4510 => {
                c.lr = 0x800d4514; f_800d2584(c);
                pc = 0x800d4514;
            }
            0x800d4514 => {
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                if c.crbit(2) { pc = 0x800d4568; continue; }
                pc = 0x800d451c;
            }
            0x800d451c => {
                c.r[3] = 0x7f;
                c.lr = 0x800d4524; f_801b25c4(c);
                pc = 0x800d4524;
            }
            0x800d4524 => {
                c.r[3] = c.lwz(c.r[26].wrapping_add(0xc));
                c.lr = 0x800d452c; f_80195a6c(c);
                pc = 0x800d452c;
            }
            0x800d452c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x800d4560; continue; }
                pc = 0x800d4534;
            }
            0x800d4534 => {
                c.r[3] = c.lwz(c.r[26].wrapping_add(0xc));
                c.lr = 0x800d453c; f_800dd174(c);
                pc = 0x800d453c;
            }
            0x800d453c => {
                c.r[3] = 0x2000;
                c.r[4] = 0x1;
                c.lr = 0x800d4548; f_800e3604(c);
                pc = 0x800d4548;
            }
            0x800d4548 => {
                c.r[3] = 0x2000;
                c.r[4] = 0x1;
                c.lr = 0x800d4554; f_80118104(c);
                pc = 0x800d4554;
            }
            0x800d4554 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa26c));
                c.lr = 0x800d455c; f_8019bd18(c);
                pc = 0x800d455c;
            }
            0x800d455c => {
                c.lr = 0x800d4560; f_80195a48(c);
                pc = 0x800d4560;
            }
            0x800d4560 => {
                c.r[3] = 0xffffffff;
                c.lr = 0x800d4568; f_800d87ac(c);
                pc = 0x800d4568;
            }
            0x800d4568 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x490));
                pc = 0x800d4578; continue;
                pc = 0x800d4574;
            }
            0x800d4574 => {
                c.lr = 0x800d4578; f_800d461c(c);
                pc = 0x800d4578;
            }
            0x800d4578 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x494));
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x800d4574; continue; }
                pc = 0x800d4588;
            }
            0x800d4588 => {
                c.lr = 0x800d458c; f_800a2790(c);
                pc = 0x800d458c;
            }
            0x800d458c => {
                c.r[0] = c.r[3].wrapping_sub(c.r[30]);
                c.stw(c.r[31].wrapping_add(0x54), c.r[0]);
                pc = 0x800d4594;
            }
            0x800d4594 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = 0x0;
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.stb(c.r[5].wrapping_add(0x1b), c.r[0]);
                c.r[5] = 0x1;
                c.lr = 0x800d45b0; f_800da2bc(c);
                pc = 0x800d45b0;
            }
            0x800d45b0 => {
                c.r[3] = 0x1;
                c.r[4] = 0x2;
                c.r[5] = 0x2;
                c.lr = 0x800d45c0; f_800da1e8(c);
                pc = 0x800d45c0;
            }
            0x800d45c0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                c.stw(c.r[3], c.r[29]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[3].wrapping_add(0x4), c.r[28]);
                if c.crbit(2) { pc = 0x800d45e4; continue; }
                pc = 0x800d45d8;
            }
            0x800d45d8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x490));
                c.stw(c.r[3].wrapping_add(0x494), c.r[0]);
                pc = 0x800d45e4;
            }
            0x800d45e4 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_800d3fa4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D45F8 (0x800d45f8, 12 bytes)
#[inline(never)]
pub fn f_800d45f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d45f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d45f8); f_800d45f8_b(c, 0x800d45f8); crate::prof::exit(c, 0x800d45f8); return; }
    }
    f_800d45f8_b(c, 0x800d45f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d45f8_at(c: &mut Cpu, pc: u32) { f_800d45f8_b(c, pc) }
#[inline(always)]
fn f_800d45f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d45f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d45f8, pc); }
        match pc {
            0x800d45f8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d45f8: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D4604 (0x800d4604, 12 bytes)
#[inline(never)]
pub fn f_800d4604(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d4604) { return; }
        if c.prof { crate::prof::enter(c, 0x800d4604); f_800d4604_b(c, 0x800d4604); crate::prof::exit(c, 0x800d4604); return; }
    }
    f_800d4604_b(c, 0x800d4604)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d4604_at(c: &mut Cpu, pc: u32) { f_800d4604_b(c, pc) }
#[inline(always)]
fn f_800d4604_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d4604;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d4604, pc); }
        match pc {
            0x800d4604 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[4], c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d4604: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D4610 (0x800d4610, 12 bytes)
#[inline(never)]
pub fn f_800d4610(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d4610) { return; }
        if c.prof { crate::prof::enter(c, 0x800d4610); f_800d4610_b(c, 0x800d4610); crate::prof::exit(c, 0x800d4610); return; }
    }
    f_800d4610_b(c, 0x800d4610)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d4610_at(c: &mut Cpu, pc: u32) { f_800d4610_b(c, pc) }
#[inline(always)]
fn f_800d4610_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d4610;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d4610, pc); }
        match pc {
            0x800d4610 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x49c), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d4610: bad pc {:#x}", pc),
        }
    }
}

/// gfxPlayCommands (0x800d461c, 2428 bytes)
#[inline(never)]
pub fn f_800d461c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d461c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d461c); f_800d461c_b(c, 0x800d461c); crate::prof::exit(c, 0x800d461c); return; }
    }
    f_800d461c_b(c, 0x800d461c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d461c_at(c: &mut Cpu, pc: u32) { f_800d461c_b(c, pc) }
#[inline(always)]
fn f_800d461c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d461c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d461c, pc); }
        match pc {
            0x800d461c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800d4620;
            }
            0x800d4620 => {
                c.r[0] = c.mfspr(8);
                pc = 0x800d4624;
            }
            0x800d4624 => {
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x800d4628;
            }
            0x800d4628 => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                pc = 0x800d462c;
            }
            0x800d462c => {
                c.r[31] = c.r[3].wrapping_add(0x4);
                pc = 0x800d4630;
            }
            0x800d4630 => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                pc = 0x800d4634;
            }
            0x800d4634 => {
                c.r[0] = c.lwz(c.r[3]);
                pc = 0x800d4638;
            }
            0x800d4638 => {
                c.cmpu(0, c.r[0], 0x5b);
                pc = 0x800d463c;
            }
            0x800d463c => {
                if c.crbit(1) { pc = 0x800d4f7c; continue; }
                pc = 0x800d4640;
            }
            0x800d4640 => {
                c.r[3] = 0x80310000;
                pc = 0x800d4644;
            }
            0x800d4644 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x800d4648;
            }
            0x800d4648 => {
                c.r[3] = c.r[3].wrapping_add(0x4188);
                pc = 0x800d464c;
            }
            0x800d464c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x800d4650;
            }
            0x800d4650 => {
                c.mtspr(9, c.r[0]);
                pc = 0x800d4654;
            }
            0x800d4654 => {
                if true { let t = c.ctr; if (0x800d461c..0x800d4f98).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x800d4658;
            }
            0x800d4658 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d465c;
            }
            0x800d465c => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4660;
            }
            0x800d4660 => {
                c.lr = 0x800d4664; f_800d6a00(c);
                pc = 0x800d4664;
            }
            0x800d4664 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4668;
            }
            0x800d4668 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d466c;
            }
            0x800d466c => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4670;
            }
            0x800d4670 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d4674;
            }
            0x800d4674 => {
                c.lr = 0x800d4678; f_800d67bc(c);
                pc = 0x800d4678;
            }
            0x800d4678 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d467c;
            }
            0x800d467c => {
                c.lr = 0x800d4680; f_800d6728(c);
                pc = 0x800d4680;
            }
            0x800d4680 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4684;
            }
            0x800d4684 => {
                c.lr = 0x800d4688; f_800d30ac(c);
                pc = 0x800d4688;
            }
            0x800d4688 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d468c;
            }
            0x800d468c => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d4690;
            }
            0x800d4690 => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800d4694;
            }
            0x800d4694 => {
                { let ea = c.r[31].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                pc = 0x800d4698;
            }
            0x800d4698 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d469c;
            }
            0x800d469c => {
                c.lr = 0x800d46a0; f_800d6680(c);
                pc = 0x800d46a0;
            }
            0x800d46a0 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d46a4;
            }
            0x800d46a4 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d46a8;
            }
            0x800d46a8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d46ac;
            }
            0x800d46ac => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d46b0;
            }
            0x800d46b0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x800d46b4;
            }
            0x800d46b4 => {
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x800d46b8;
            }
            0x800d46b8 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d46bc;
            }
            0x800d46bc => {
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d46c0;
            }
            0x800d46c0 => {
                c.lr = 0x800d46c4; f_800d65cc(c);
                pc = 0x800d46c4;
            }
            0x800d46c4 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d46c8;
            }
            0x800d46c8 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d46cc;
            }
            0x800d46cc => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d46d0;
            }
            0x800d46d0 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d46d4;
            }
            0x800d46d4 => {
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x800d46d8;
            }
            0x800d46d8 => {
                c.r[4] = c.r[4] as u16 as i16 as i32 as u32;
                pc = 0x800d46dc;
            }
            0x800d46dc => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d46e0;
            }
            0x800d46e0 => {
                c.r[5] = c.r[0] as u16 as i16 as i32 as u32;
                pc = 0x800d46e4;
            }
            0x800d46e4 => {
                c.lr = 0x800d46e8; f_800d6518(c);
                pc = 0x800d46e8;
            }
            0x800d46e8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d46ec;
            }
            0x800d46ec => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d46f0;
            }
            0x800d46f0 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d46f4;
            }
            0x800d46f4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d46f8;
            }
            0x800d46f8 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x800d46fc;
            }
            0x800d46fc => {
                c.r[4] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x800d4700;
            }
            0x800d4700 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4704;
            }
            0x800d4704 => {
                c.r[5] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4708;
            }
            0x800d4708 => {
                c.lr = 0x800d470c; f_800d6464(c);
                pc = 0x800d470c;
            }
            0x800d470c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4710;
            }
            0x800d4710 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4714;
            }
            0x800d4714 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4718;
            }
            0x800d4718 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d471c;
            }
            0x800d471c => {
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x800d4720;
            }
            0x800d4720 => {
                c.r[4] = c.r[4] as u8 as i8 as i32 as u32;
                pc = 0x800d4724;
            }
            0x800d4724 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4728;
            }
            0x800d4728 => {
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                pc = 0x800d472c;
            }
            0x800d472c => {
                c.lr = 0x800d4730; f_800d63b0(c);
                pc = 0x800d4730;
            }
            0x800d4730 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4734;
            }
            0x800d4734 => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d4738;
            }
            0x800d4738 => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800d473c;
            }
            0x800d473c => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d4740;
            }
            0x800d4740 => {
                c.lr = 0x800d4744; f_800d631c(c);
                pc = 0x800d4744;
            }
            0x800d4744 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4748;
            }
            0x800d4748 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d474c;
            }
            0x800d474c => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4750;
            }
            0x800d4750 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d4754;
            }
            0x800d4754 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x800d4758;
            }
            0x800d4758 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d475c;
            }
            0x800d475c => {
                c.lr = 0x800d4760; f_800d6280(c);
                pc = 0x800d4760;
            }
            0x800d4760 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4764;
            }
            0x800d4764 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4768;
            }
            0x800d4768 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d476c;
            }
            0x800d476c => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d4770;
            }
            0x800d4770 => {
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x800d4774;
            }
            0x800d4774 => {
                c.r[4] = c.r[0] as u16 as i16 as i32 as u32;
                pc = 0x800d4778;
            }
            0x800d4778 => {
                c.lr = 0x800d477c; f_800d61e4(c);
                pc = 0x800d477c;
            }
            0x800d477c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4780;
            }
            0x800d4780 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4784;
            }
            0x800d4784 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4788;
            }
            0x800d4788 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d478c;
            }
            0x800d478c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x800d4790;
            }
            0x800d4790 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4794;
            }
            0x800d4794 => {
                c.lr = 0x800d4798; f_800d6148(c);
                pc = 0x800d4798;
            }
            0x800d4798 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d479c;
            }
            0x800d479c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d47a0;
            }
            0x800d47a0 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d47a4;
            }
            0x800d47a4 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d47a8;
            }
            0x800d47a8 => {
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x800d47ac;
            }
            0x800d47ac => {
                c.r[4] = c.r[0] as u8 as i8 as i32 as u32;
                pc = 0x800d47b0;
            }
            0x800d47b0 => {
                c.lr = 0x800d47b4; f_800d60ac(c);
                pc = 0x800d47b4;
            }
            0x800d47b4 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d47b8;
            }
            0x800d47b8 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d47bc;
            }
            0x800d47bc => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d47c0;
            }
            0x800d47c0 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d47c4;
            }
            0x800d47c4 => {
                c.lr = 0x800d47c8; f_800d6028(c);
                pc = 0x800d47c8;
            }
            0x800d47c8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d47cc;
            }
            0x800d47cc => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d47d0;
            }
            0x800d47d0 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d47d4;
            }
            0x800d47d4 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d47d8;
            }
            0x800d47d8 => {
                c.lr = 0x800d47dc; f_800d5fa4(c);
                pc = 0x800d47dc;
            }
            0x800d47dc => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d47e0;
            }
            0x800d47e0 => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d47e4;
            }
            0x800d47e4 => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800d47e8;
            }
            0x800d47e8 => {
                { let ea = c.r[31].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                pc = 0x800d47ec;
            }
            0x800d47ec => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d47f0;
            }
            0x800d47f0 => {
                c.lr = 0x800d47f4; f_800d5f34(c);
                pc = 0x800d47f4;
            }
            0x800d47f4 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d47f8;
            }
            0x800d47f8 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d47fc;
            }
            0x800d47fc => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4800;
            }
            0x800d4800 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4804;
            }
            0x800d4804 => {
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x800d4808;
            }
            0x800d4808 => {
                c.r[4] = c.r[4] as u16 as i16 as i32 as u32;
                pc = 0x800d480c;
            }
            0x800d480c => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4810;
            }
            0x800d4810 => {
                c.r[5] = c.r[0] as u16 as i16 as i32 as u32;
                pc = 0x800d4814;
            }
            0x800d4814 => {
                c.lr = 0x800d4818; f_800d5eb4(c);
                pc = 0x800d4818;
            }
            0x800d4818 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d481c;
            }
            0x800d481c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4820;
            }
            0x800d4820 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4824;
            }
            0x800d4824 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4828;
            }
            0x800d4828 => {
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x800d482c;
            }
            0x800d482c => {
                c.r[4] = c.r[4] as u8 as i8 as i32 as u32;
                pc = 0x800d4830;
            }
            0x800d4830 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4834;
            }
            0x800d4834 => {
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                pc = 0x800d4838;
            }
            0x800d4838 => {
                c.lr = 0x800d483c; f_800d5e34(c);
                pc = 0x800d483c;
            }
            0x800d483c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4840;
            }
            0x800d4840 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d4844;
            }
            0x800d4844 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4848;
            }
            0x800d4848 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d484c;
            }
            0x800d484c => {
                c.lr = 0x800d4850; f_800d5dd0(c);
                pc = 0x800d4850;
            }
            0x800d4850 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4854;
            }
            0x800d4854 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d4858;
            }
            0x800d4858 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d485c;
            }
            0x800d485c => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4860;
            }
            0x800d4860 => {
                c.lr = 0x800d4864; f_800d5d6c(c);
                pc = 0x800d4864;
            }
            0x800d4864 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4868;
            }
            0x800d4868 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d486c;
            }
            0x800d486c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4870;
            }
            0x800d4870 => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4874;
            }
            0x800d4874 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4878;
            }
            0x800d4878 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x10));
                pc = 0x800d487c;
            }
            0x800d487c => {
                c.r[5] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x800d4880;
            }
            0x800d4880 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4884;
            }
            0x800d4884 => {
                c.r[6] = c.r[6].rotate_left(0) & 0xff;
                pc = 0x800d4888;
            }
            0x800d4888 => {
                c.r[7] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d488c;
            }
            0x800d488c => {
                c.r[31] = c.r[31].wrapping_add(0x14);
                pc = 0x800d4890;
            }
            0x800d4890 => {
                c.lr = 0x800d4894; f_800d5cb8(c);
                pc = 0x800d4894;
            }
            0x800d4894 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4898;
            }
            0x800d4898 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d489c;
            }
            0x800d489c => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d48a0;
            }
            0x800d48a0 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d48a4;
            }
            0x800d48a4 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x800d48a8;
            }
            0x800d48a8 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d48ac;
            }
            0x800d48ac => {
                c.r[5] = c.r[5].rotate_left(0) & 0xff;
                pc = 0x800d48b0;
            }
            0x800d48b0 => {
                c.r[6] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d48b4;
            }
            0x800d48b4 => {
                c.r[31] = c.r[31].wrapping_add(0x10);
                pc = 0x800d48b8;
            }
            0x800d48b8 => {
                c.lr = 0x800d48bc; f_800d5c18(c);
                pc = 0x800d48bc;
            }
            0x800d48bc => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d48c0;
            }
            0x800d48c0 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d48c4;
            }
            0x800d48c4 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d48c8;
            }
            0x800d48c8 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d48cc;
            }
            0x800d48cc => {
                c.lr = 0x800d48d0; f_800d5ba0(c);
                pc = 0x800d48d0;
            }
            0x800d48d0 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d48d4;
            }
            0x800d48d4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d48d8;
            }
            0x800d48d8 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d48dc;
            }
            0x800d48dc => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d48e0;
            }
            0x800d48e0 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d48e4;
            }
            0x800d48e4 => {
                c.lr = 0x800d48e8; f_800d5b28(c);
                pc = 0x800d48e8;
            }
            0x800d48e8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d48ec;
            }
            0x800d48ec => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d48f0;
            }
            0x800d48f0 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d48f4;
            }
            0x800d48f4 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d48f8;
            }
            0x800d48f8 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d48fc;
            }
            0x800d48fc => {
                c.lr = 0x800d4900; f_800d5ab0(c);
                pc = 0x800d4900;
            }
            0x800d4900 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4904;
            }
            0x800d4904 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4908;
            }
            0x800d4908 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d490c;
            }
            0x800d490c => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d4910;
            }
            0x800d4910 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4914;
            }
            0x800d4914 => {
                c.lr = 0x800d4918; f_800d5a38(c);
                pc = 0x800d4918;
            }
            0x800d4918 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d491c;
            }
            0x800d491c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4920;
            }
            0x800d4920 => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d4924;
            }
            0x800d4924 => {
                { let ea = c.r[31].wrapping_add(0x8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800d4928;
            }
            0x800d4928 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d492c;
            }
            0x800d492c => {
                c.lr = 0x800d4930; f_800d59b8(c);
                pc = 0x800d4930;
            }
            0x800d4930 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4934;
            }
            0x800d4934 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4938;
            }
            0x800d4938 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d493c;
            }
            0x800d493c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4940;
            }
            0x800d4940 => {
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x800d4944;
            }
            0x800d4944 => {
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d4948;
            }
            0x800d4948 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d494c;
            }
            0x800d494c => {
                c.lr = 0x800d4950; f_800d592c(c);
                pc = 0x800d4950;
            }
            0x800d4950 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4954;
            }
            0x800d4954 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4958;
            }
            0x800d4958 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d495c;
            }
            0x800d495c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4960;
            }
            0x800d4960 => {
                c.r[4] = c.r[4] as u16 as i16 as i32 as u32;
                pc = 0x800d4964;
            }
            0x800d4964 => {
                c.r[5] = c.r[0] as u16 as i16 as i32 as u32;
                pc = 0x800d4968;
            }
            0x800d4968 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d496c;
            }
            0x800d496c => {
                c.lr = 0x800d4970; f_800d58a0(c);
                pc = 0x800d4970;
            }
            0x800d4970 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4974;
            }
            0x800d4974 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4978;
            }
            0x800d4978 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d497c;
            }
            0x800d497c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4980;
            }
            0x800d4980 => {
                c.r[4] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x800d4984;
            }
            0x800d4984 => {
                c.r[5] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4988;
            }
            0x800d4988 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d498c;
            }
            0x800d498c => {
                c.lr = 0x800d4990; f_800d5814(c);
                pc = 0x800d4990;
            }
            0x800d4990 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4994;
            }
            0x800d4994 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4998;
            }
            0x800d4998 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d499c;
            }
            0x800d499c => {
                c.r[4] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x800d49a0;
            }
            0x800d49a0 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d49a4;
            }
            0x800d49a4 => {
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                pc = 0x800d49a8;
            }
            0x800d49a8 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d49ac;
            }
            0x800d49ac => {
                c.lr = 0x800d49b0; f_800d58a0(c);
                pc = 0x800d49b0;
            }
            0x800d49b0 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d49b4;
            }
            0x800d49b4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d49b8;
            }
            0x800d49b8 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d49bc;
            }
            0x800d49bc => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d49c0;
            }
            0x800d49c0 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d49c4;
            }
            0x800d49c4 => {
                c.lr = 0x800d49c8; f_800d579c(c);
                pc = 0x800d49c8;
            }
            0x800d49c8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d49cc;
            }
            0x800d49cc => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d49d0;
            }
            0x800d49d0 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d49d4;
            }
            0x800d49d4 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d49d8;
            }
            0x800d49d8 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d49dc;
            }
            0x800d49dc => {
                c.lr = 0x800d49e0; f_800d5724(c);
                pc = 0x800d49e0;
            }
            0x800d49e0 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d49e4;
            }
            0x800d49e4 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d49e8;
            }
            0x800d49e8 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d49ec;
            }
            0x800d49ec => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d49f0;
            }
            0x800d49f0 => {
                c.lr = 0x800d49f4; f_800d56c0(c);
                pc = 0x800d49f4;
            }
            0x800d49f4 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d49f8;
            }
            0x800d49f8 => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d49fc;
            }
            0x800d49fc => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4a00;
            }
            0x800d4a00 => {
                c.lr = 0x800d4a04; f_800d5648(c);
                pc = 0x800d4a04;
            }
            0x800d4a04 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4a08;
            }
            0x800d4a08 => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d4a0c;
            }
            0x800d4a0c => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4a10;
            }
            0x800d4a10 => {
                c.lr = 0x800d4a14; f_800d55d0(c);
                pc = 0x800d4a14;
            }
            0x800d4a14 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4a18;
            }
            0x800d4a18 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4a1c;
            }
            0x800d4a1c => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4a20;
            }
            0x800d4a20 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d4a24;
            }
            0x800d4a24 => {
                c.lr = 0x800d4a28; f_800d85d4(c);
                pc = 0x800d4a28;
            }
            0x800d4a28 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4a2c;
            }
            0x800d4a2c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4a30;
            }
            0x800d4a30 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4a34;
            }
            0x800d4a34 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4a38;
            }
            0x800d4a38 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4a3c;
            }
            0x800d4a3c => {
                c.r[6] = c.r[31] | c.r[31];
                pc = 0x800d4a40;
            }
            0x800d4a40 => {
                c.lr = 0x800d4a44; f_800d848c(c);
                pc = 0x800d4a44;
            }
            0x800d4a44 => {
                c.r[31] = c.r[31].wrapping_add(0x30);
                pc = 0x800d4a48;
            }
            0x800d4a48 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4a4c;
            }
            0x800d4a4c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4a50;
            }
            0x800d4a50 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4a54;
            }
            0x800d4a54 => {
                c.lr = 0x800d4a58; f_800d88dc(c);
                pc = 0x800d4a58;
            }
            0x800d4a58 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4a5c;
            }
            0x800d4a5c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4a60;
            }
            0x800d4a60 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4a64;
            }
            0x800d4a64 => {
                c.lr = 0x800d4a68; f_800d888c(c);
                pc = 0x800d4a68;
            }
            0x800d4a68 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4a6c;
            }
            0x800d4a6c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4a70;
            }
            0x800d4a70 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4a74;
            }
            0x800d4a74 => {
                c.lr = 0x800d4a78; f_800dad10(c);
                pc = 0x800d4a78;
            }
            0x800d4a78 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4a7c;
            }
            0x800d4a7c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4a80;
            }
            0x800d4a80 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4a84;
            }
            0x800d4a84 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4a88;
            }
            0x800d4a88 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4a8c;
            }
            0x800d4a8c => {
                c.lr = 0x800d4a90; f_800da4c4(c);
                pc = 0x800d4a90;
            }
            0x800d4a90 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4a94;
            }
            0x800d4a94 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4a98;
            }
            0x800d4a98 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4a9c;
            }
            0x800d4a9c => {
                c.lr = 0x800d4aa0; f_800da428(c);
                pc = 0x800d4aa0;
            }
            0x800d4aa0 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4aa4;
            }
            0x800d4aa4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4aa8;
            }
            0x800d4aa8 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4aac;
            }
            0x800d4aac => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d4ab0;
            }
            0x800d4ab0 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4ab4;
            }
            0x800d4ab4 => {
                c.lr = 0x800d4ab8; f_800da3b0(c);
                pc = 0x800d4ab8;
            }
            0x800d4ab8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4abc;
            }
            0x800d4abc => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4ac0;
            }
            0x800d4ac0 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4ac4;
            }
            0x800d4ac4 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4ac8;
            }
            0x800d4ac8 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4acc;
            }
            0x800d4acc => {
                c.lr = 0x800d4ad0; f_800da2bc(c);
                pc = 0x800d4ad0;
            }
            0x800d4ad0 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4ad4;
            }
            0x800d4ad4 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4ad8;
            }
            0x800d4ad8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4adc;
            }
            0x800d4adc => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4ae0;
            }
            0x800d4ae0 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4ae4;
            }
            0x800d4ae4 => {
                c.lr = 0x800d4ae8; f_800da1e8(c);
                pc = 0x800d4ae8;
            }
            0x800d4ae8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4aec;
            }
            0x800d4aec => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4af0;
            }
            0x800d4af0 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x14));
                pc = 0x800d4af4;
            }
            0x800d4af4 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4af8;
            }
            0x800d4af8 => {
                c.r[5] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x800d4afc;
            }
            0x800d4afc => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4b00;
            }
            0x800d4b00 => {
                c.r[8] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4b04;
            }
            0x800d4b04 => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4b08;
            }
            0x800d4b08 => {
                c.r[7] = c.lwz(c.r[31].wrapping_add(0x10));
                pc = 0x800d4b0c;
            }
            0x800d4b0c => {
                c.r[31] = c.r[31].wrapping_add(0x18);
                pc = 0x800d4b10;
            }
            0x800d4b10 => {
                c.lr = 0x800d4b14; f_800da100(c);
                pc = 0x800d4b14;
            }
            0x800d4b14 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4b18;
            }
            0x800d4b18 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4b1c;
            }
            0x800d4b1c => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4b20;
            }
            0x800d4b20 => {
                c.lr = 0x800d4b24; f_800da08c(c);
                pc = 0x800d4b24;
            }
            0x800d4b24 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4b28;
            }
            0x800d4b28 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4b2c;
            }
            0x800d4b2c => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4b30;
            }
            0x800d4b30 => {
                c.lr = 0x800d4b34; f_800da028(c);
                pc = 0x800d4b34;
            }
            0x800d4b34 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4b38;
            }
            0x800d4b38 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4b3c;
            }
            0x800d4b3c => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4b40;
            }
            0x800d4b40 => {
                c.lr = 0x800d4b44; f_800d9fb4(c);
                pc = 0x800d4b44;
            }
            0x800d4b44 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4b48;
            }
            0x800d4b48 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4b4c;
            }
            0x800d4b4c => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4b50;
            }
            0x800d4b50 => {
                c.lr = 0x800d4b54; f_800d9f40(c);
                pc = 0x800d4b54;
            }
            0x800d4b54 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4b58;
            }
            0x800d4b58 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4b5c;
            }
            0x800d4b5c => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4b60;
            }
            0x800d4b60 => {
                c.lr = 0x800d4b64; f_800d9ed8(c);
                pc = 0x800d4b64;
            }
            0x800d4b64 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4b68;
            }
            0x800d4b68 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4b6c;
            }
            0x800d4b6c => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4b70;
            }
            0x800d4b70 => {
                c.lr = 0x800d4b74; f_800d9e4c(c);
                pc = 0x800d4b74;
            }
            0x800d4b74 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4b78;
            }
            0x800d4b78 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d4b7c;
            }
            0x800d4b7c => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4b80;
            }
            0x800d4b80 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4b84;
            }
            0x800d4b84 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d4b88;
            }
            0x800d4b88 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4b8c;
            }
            0x800d4b8c => {
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x800d4b90;
            }
            0x800d4b90 => {
                c.r[5] = c.r[5].rotate_left(0) & 0xffff;
                pc = 0x800d4b94;
            }
            0x800d4b94 => {
                c.r[31] = c.r[31].wrapping_add(0x10);
                pc = 0x800d4b98;
            }
            0x800d4b98 => {
                c.r[6] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d4b9c;
            }
            0x800d4b9c => {
                c.lr = 0x800d4ba0; f_800d9d68(c);
                pc = 0x800d4ba0;
            }
            0x800d4ba0 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4ba4;
            }
            0x800d4ba4 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d4ba8;
            }
            0x800d4ba8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4bac;
            }
            0x800d4bac => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4bb0;
            }
            0x800d4bb0 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d4bb4;
            }
            0x800d4bb4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4bb8;
            }
            0x800d4bb8 => {
                c.r[4] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x800d4bbc;
            }
            0x800d4bbc => {
                c.r[5] = c.r[5].rotate_left(0) & 0xffff;
                pc = 0x800d4bc0;
            }
            0x800d4bc0 => {
                c.r[31] = c.r[31].wrapping_add(0x10);
                pc = 0x800d4bc4;
            }
            0x800d4bc4 => {
                c.r[6] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d4bc8;
            }
            0x800d4bc8 => {
                c.lr = 0x800d4bcc; f_800d9c24(c);
                pc = 0x800d4bcc;
            }
            0x800d4bcc => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4bd0;
            }
            0x800d4bd0 => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d4bd4;
            }
            0x800d4bd4 => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800d4bd8;
            }
            0x800d4bd8 => {
                { let ea = c.r[31].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                pc = 0x800d4bdc;
            }
            0x800d4bdc => {
                { let ea = c.r[31].wrapping_add(0xc); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                pc = 0x800d4be0;
            }
            0x800d4be0 => {
                c.r[31] = c.r[31].wrapping_add(0x10);
                pc = 0x800d4be4;
            }
            0x800d4be4 => {
                c.lr = 0x800d4be8; f_800d9bd0(c);
                pc = 0x800d4be8;
            }
            0x800d4be8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4bec;
            }
            0x800d4bec => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d4bf0;
            }
            0x800d4bf0 => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800d4bf4;
            }
            0x800d4bf4 => {
                { let ea = c.r[31].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                pc = 0x800d4bf8;
            }
            0x800d4bf8 => {
                { let ea = c.r[31].wrapping_add(0xc); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                pc = 0x800d4bfc;
            }
            0x800d4bfc => {
                c.r[31] = c.r[31].wrapping_add(0x10);
                pc = 0x800d4c00;
            }
            0x800d4c00 => {
                c.lr = 0x800d4c04; f_800d9b58(c);
                pc = 0x800d4c04;
            }
            0x800d4c04 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4c08;
            }
            0x800d4c08 => {
                c.lr = 0x800d4c0c; f_800d834c(c);
                pc = 0x800d4c0c;
            }
            0x800d4c0c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4c10;
            }
            0x800d4c10 => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d4c14;
            }
            0x800d4c14 => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800d4c18;
            }
            0x800d4c18 => {
                { let ea = c.r[31].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                pc = 0x800d4c1c;
            }
            0x800d4c1c => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4c20;
            }
            0x800d4c20 => {
                c.lr = 0x800d4c24; f_800d8284(c);
                pc = 0x800d4c24;
            }
            0x800d4c24 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4c28;
            }
            0x800d4c28 => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d4c2c;
            }
            0x800d4c2c => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800d4c30;
            }
            0x800d4c30 => {
                { let ea = c.r[31].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                pc = 0x800d4c34;
            }
            0x800d4c34 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4c38;
            }
            0x800d4c38 => {
                c.lr = 0x800d4c3c; f_800d81ec(c);
                pc = 0x800d4c3c;
            }
            0x800d4c3c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4c40;
            }
            0x800d4c40 => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x800d4c44;
            }
            0x800d4c44 => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x800d4c48;
            }
            0x800d4c48 => {
                { let ea = c.r[31].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                pc = 0x800d4c4c;
            }
            0x800d4c4c => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4c50;
            }
            0x800d4c50 => {
                c.lr = 0x800d4c54; f_800d8154(c);
                pc = 0x800d4c54;
            }
            0x800d4c54 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4c58;
            }
            0x800d4c58 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d4c5c;
            }
            0x800d4c5c => {
                c.lr = 0x800d4c60; f_800d8088(c);
                pc = 0x800d4c60;
            }
            0x800d4c60 => {
                c.r[31] = c.r[31].wrapping_add(0x30);
                pc = 0x800d4c64;
            }
            0x800d4c64 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4c68;
            }
            0x800d4c68 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d4c6c;
            }
            0x800d4c6c => {
                c.lr = 0x800d4c70; f_800d7fe4(c);
                pc = 0x800d4c70;
            }
            0x800d4c70 => {
                c.r[31] = c.r[31].wrapping_add(0x30);
                pc = 0x800d4c74;
            }
            0x800d4c74 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4c78;
            }
            0x800d4c78 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d4c7c;
            }
            0x800d4c7c => {
                c.lr = 0x800d4c80; f_800d7f14(c);
                pc = 0x800d4c80;
            }
            0x800d4c80 => {
                c.r[31] = c.r[31].wrapping_add(0x30);
                pc = 0x800d4c84;
            }
            0x800d4c84 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4c88;
            }
            0x800d4c88 => {
                c.lr = 0x800d4c8c; f_800d7e5c(c);
                pc = 0x800d4c8c;
            }
            0x800d4c8c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4c90;
            }
            0x800d4c90 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d4c94;
            }
            0x800d4c94 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4c98;
            }
            0x800d4c98 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x800d4c9c;
            }
            0x800d4c9c => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4ca0;
            }
            0x800d4ca0 => {
                c.lr = 0x800d4ca4; f_800d7d90(c);
                pc = 0x800d4ca4;
            }
            0x800d4ca4 => {
                c.r[31] = c.r[31].wrapping_add(0x30);
                pc = 0x800d4ca8;
            }
            0x800d4ca8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4cac;
            }
            0x800d4cac => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d4cb0;
            }
            0x800d4cb0 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4cb4;
            }
            0x800d4cb4 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d4cb8;
            }
            0x800d4cb8 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4cbc;
            }
            0x800d4cbc => {
                c.lr = 0x800d4cc0; f_800d7d10(c);
                pc = 0x800d4cc0;
            }
            0x800d4cc0 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4cc4;
            }
            0x800d4cc4 => {
                c.lr = 0x800d4cc8; f_800d7c74(c);
                pc = 0x800d4cc8;
            }
            0x800d4cc8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4ccc;
            }
            0x800d4ccc => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4cd0;
            }
            0x800d4cd0 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4cd4;
            }
            0x800d4cd4 => {
                c.lr = 0x800d4cd8; f_800d7820(c);
                pc = 0x800d4cd8;
            }
            0x800d4cd8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4cdc;
            }
            0x800d4cdc => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4ce0;
            }
            0x800d4ce0 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4ce4;
            }
            0x800d4ce4 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d4ce8;
            }
            0x800d4ce8 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d4cec;
            }
            0x800d4cec => {
                c.lr = 0x800d4cf0; f_800d76a8(c);
                pc = 0x800d4cf0;
            }
            0x800d4cf0 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4cf4;
            }
            0x800d4cf4 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4cf8;
            }
            0x800d4cf8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4cfc;
            }
            0x800d4cfc => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4d00;
            }
            0x800d4d00 => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4d04;
            }
            0x800d4d04 => {
                c.r[7] = c.lwz(c.r[31].wrapping_add(0x10));
                pc = 0x800d4d08;
            }
            0x800d4d08 => {
                c.r[31] = c.r[31].wrapping_add(0x14);
                pc = 0x800d4d0c;
            }
            0x800d4d0c => {
                c.lr = 0x800d4d10; f_800dc224(c);
                pc = 0x800d4d10;
            }
            0x800d4d10 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4d14;
            }
            0x800d4d14 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d4d18;
            }
            0x800d4d18 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4d1c;
            }
            0x800d4d1c => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4d20;
            }
            0x800d4d20 => {
                c.lr = 0x800d4d24; f_800dc1d4(c);
                pc = 0x800d4d24;
            }
            0x800d4d24 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4d28;
            }
            0x800d4d28 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x10));
                pc = 0x800d4d2c;
            }
            0x800d4d2c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4d30;
            }
            0x800d4d30 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4d34;
            }
            0x800d4d34 => {
                c.r[7] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4d38;
            }
            0x800d4d38 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4d3c;
            }
            0x800d4d3c => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4d40;
            }
            0x800d4d40 => {
                c.r[8] = c.lwz(c.r[31].wrapping_add(0x14));
                pc = 0x800d4d44;
            }
            0x800d4d44 => {
                c.r[31] = c.r[31].wrapping_add(0x18);
                pc = 0x800d4d48;
            }
            0x800d4d48 => {
                c.lr = 0x800d4d4c; f_800dc14c(c);
                pc = 0x800d4d4c;
            }
            0x800d4d4c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4d50;
            }
            0x800d4d50 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4d54;
            }
            0x800d4d54 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4d58;
            }
            0x800d4d58 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4d5c;
            }
            0x800d4d5c => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4d60;
            }
            0x800d4d60 => {
                c.r[7] = c.lwz(c.r[31].wrapping_add(0x10));
                pc = 0x800d4d64;
            }
            0x800d4d64 => {
                c.r[31] = c.r[31].wrapping_add(0x14);
                pc = 0x800d4d68;
            }
            0x800d4d68 => {
                c.lr = 0x800d4d6c; f_800dc0d4(c);
                pc = 0x800d4d6c;
            }
            0x800d4d6c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4d70;
            }
            0x800d4d70 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x10));
                pc = 0x800d4d74;
            }
            0x800d4d74 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4d78;
            }
            0x800d4d78 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4d7c;
            }
            0x800d4d7c => {
                c.r[7] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4d80;
            }
            0x800d4d80 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4d84;
            }
            0x800d4d84 => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4d88;
            }
            0x800d4d88 => {
                c.r[8] = c.lwz(c.r[31].wrapping_add(0x14));
                pc = 0x800d4d8c;
            }
            0x800d4d8c => {
                c.r[31] = c.r[31].wrapping_add(0x18);
                pc = 0x800d4d90;
            }
            0x800d4d90 => {
                c.lr = 0x800d4d94; f_800dc04c(c);
                pc = 0x800d4d94;
            }
            0x800d4d94 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4d98;
            }
            0x800d4d98 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4d9c;
            }
            0x800d4d9c => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4da0;
            }
            0x800d4da0 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4da4;
            }
            0x800d4da4 => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4da8;
            }
            0x800d4da8 => {
                c.r[7] = c.lwz(c.r[31].wrapping_add(0x10));
                pc = 0x800d4dac;
            }
            0x800d4dac => {
                c.r[31] = c.r[31].wrapping_add(0x14);
                pc = 0x800d4db0;
            }
            0x800d4db0 => {
                c.lr = 0x800d4db4; f_800dbfd4(c);
                pc = 0x800d4db4;
            }
            0x800d4db4 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4db8;
            }
            0x800d4db8 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4dbc;
            }
            0x800d4dbc => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4dc0;
            }
            0x800d4dc0 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d4dc4;
            }
            0x800d4dc4 => {
                c.lr = 0x800d4dc8; f_800dbf78(c);
                pc = 0x800d4dc8;
            }
            0x800d4dc8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4dcc;
            }
            0x800d4dcc => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4dd0;
            }
            0x800d4dd0 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4dd4;
            }
            0x800d4dd4 => {
                c.r[31] = c.r[31].wrapping_add(0x8);
                pc = 0x800d4dd8;
            }
            0x800d4dd8 => {
                c.lr = 0x800d4ddc; f_800dbf1c(c);
                pc = 0x800d4ddc;
            }
            0x800d4ddc => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4de0;
            }
            0x800d4de0 => {
                c.r[30] = c.lwz(c.r[31]);
                pc = 0x800d4de4;
            }
            0x800d4de4 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4de8;
            }
            0x800d4de8 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x800d4dec;
            }
            0x800d4dec => {
                c.r[3] = c.r[1].wrapping_add(0x14);
                pc = 0x800d4df0;
            }
            0x800d4df0 => {
                c.r[5] = 0x4;
                pc = 0x800d4df4;
            }
            0x800d4df4 => {
                c.lr = 0x800d4df8; f_800054f4(c);
                pc = 0x800d4df8;
            }
            0x800d4df8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x800d4dfc;
            }
            0x800d4dfc => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x800d4e00;
            }
            0x800d4e00 => {
                c.r[4] = c.r[1].wrapping_add(0x10);
                pc = 0x800d4e04;
            }
            0x800d4e04 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4e08;
            }
            0x800d4e08 => {
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                pc = 0x800d4e0c;
            }
            0x800d4e0c => {
                c.lr = 0x800d4e10; f_800dbeb4(c);
                pc = 0x800d4e10;
            }
            0x800d4e10 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4e14;
            }
            0x800d4e14 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4e18;
            }
            0x800d4e18 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4e1c;
            }
            0x800d4e1c => {
                c.lr = 0x800d4e20; f_800dbe5c(c);
                pc = 0x800d4e20;
            }
            0x800d4e20 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4e24;
            }
            0x800d4e24 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d4e28;
            }
            0x800d4e28 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4e2c;
            }
            0x800d4e2c => {
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x800d4e30;
            }
            0x800d4e30 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4e34;
            }
            0x800d4e34 => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                pc = 0x800d4e38;
            }
            0x800d4e38 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x24));
                pc = 0x800d4e3c;
            }
            0x800d4e3c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4e40;
            }
            0x800d4e40 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4e44;
            }
            0x800d4e44 => {
                c.r[10] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4e48;
            }
            0x800d4e48 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x10));
                pc = 0x800d4e4c;
            }
            0x800d4e4c => {
                c.r[6] = c.lwz(c.r[31].wrapping_add(0x14));
                pc = 0x800d4e50;
            }
            0x800d4e50 => {
                c.r[7] = c.lwz(c.r[31].wrapping_add(0x18));
                pc = 0x800d4e54;
            }
            0x800d4e54 => {
                c.r[8] = c.lwz(c.r[31].wrapping_add(0x1c));
                pc = 0x800d4e58;
            }
            0x800d4e58 => {
                c.r[9] = c.lwz(c.r[31].wrapping_add(0x20));
                pc = 0x800d4e5c;
            }
            0x800d4e5c => {
                c.r[31] = c.r[31].wrapping_add(0x28);
                pc = 0x800d4e60;
            }
            0x800d4e60 => {
                c.lr = 0x800d4e64; f_800dbd70(c);
                pc = 0x800d4e64;
            }
            0x800d4e64 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4e68;
            }
            0x800d4e68 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4e6c;
            }
            0x800d4e6c => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4e70;
            }
            0x800d4e70 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4e74;
            }
            0x800d4e74 => {
                c.r[5] = c.r[4].rotate_left(0) & 0xff;
                pc = 0x800d4e78;
            }
            0x800d4e78 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4e7c;
            }
            0x800d4e7c => {
                c.r[6] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4e80;
            }
            0x800d4e80 => {
                c.r[7] = c.lwz(c.r[31].wrapping_add(0x10));
                pc = 0x800d4e84;
            }
            0x800d4e84 => {
                c.r[31] = c.r[31].wrapping_add(0x14);
                pc = 0x800d4e88;
            }
            0x800d4e88 => {
                c.lr = 0x800d4e8c; f_800dbce4(c);
                pc = 0x800d4e8c;
            }
            0x800d4e8c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4e90;
            }
            0x800d4e90 => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d4e94;
            }
            0x800d4e94 => {
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x800d4e98;
            }
            0x800d4e98 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4e9c;
            }
            0x800d4e9c => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                pc = 0x800d4ea0;
            }
            0x800d4ea0 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x10));
                pc = 0x800d4ea4;
            }
            0x800d4ea4 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x14));
                pc = 0x800d4ea8;
            }
            0x800d4ea8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x18));
                pc = 0x800d4eac;
            }
            0x800d4eac => {
                c.r[5] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d4eb0;
            }
            0x800d4eb0 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x1c));
                pc = 0x800d4eb4;
            }
            0x800d4eb4 => {
                c.r[6] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x800d4eb8;
            }
            0x800d4eb8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4ebc;
            }
            0x800d4ebc => {
                c.r[7] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x800d4ec0;
            }
            0x800d4ec0 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0xc));
                pc = 0x800d4ec4;
            }
            0x800d4ec4 => {
                c.r[8] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x800d4ec8;
            }
            0x800d4ec8 => {
                c.r[9] = c.lwz(c.r[31].wrapping_add(0x20));
                pc = 0x800d4ecc;
            }
            0x800d4ecc => {
                c.r[10] = c.lwz(c.r[31].wrapping_add(0x24));
                pc = 0x800d4ed0;
            }
            0x800d4ed0 => {
                c.r[31] = c.r[31].wrapping_add(0x28);
                pc = 0x800d4ed4;
            }
            0x800d4ed4 => {
                c.lr = 0x800d4ed8; f_800dbbfc(c);
                pc = 0x800d4ed8;
            }
            0x800d4ed8 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4edc;
            }
            0x800d4edc => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4ee0;
            }
            0x800d4ee0 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4ee4;
            }
            0x800d4ee4 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4ee8;
            }
            0x800d4ee8 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4eec;
            }
            0x800d4eec => {
                c.lr = 0x800d4ef0; f_800dbb84(c);
                pc = 0x800d4ef0;
            }
            0x800d4ef0 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4ef4;
            }
            0x800d4ef4 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4ef8;
            }
            0x800d4ef8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4efc;
            }
            0x800d4efc => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4f00;
            }
            0x800d4f00 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4f04;
            }
            0x800d4f04 => {
                c.lr = 0x800d4f08; f_800dbb0c(c);
                pc = 0x800d4f08;
            }
            0x800d4f08 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4f0c;
            }
            0x800d4f0c => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4f10;
            }
            0x800d4f10 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4f14;
            }
            0x800d4f14 => {
                c.lr = 0x800d4f18; f_800dbaa4(c);
                pc = 0x800d4f18;
            }
            0x800d4f18 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4f1c;
            }
            0x800d4f1c => {
                c.r[0] = c.lwz(c.r[31]);
                pc = 0x800d4f20;
            }
            0x800d4f20 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4f24;
            }
            0x800d4f24 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                pc = 0x800d4f28;
            }
            0x800d4f28 => {
                c.lr = 0x800d4f2c; f_800dba54(c);
                pc = 0x800d4f2c;
            }
            0x800d4f2c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4f30;
            }
            0x800d4f30 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4f34;
            }
            0x800d4f34 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4f38;
            }
            0x800d4f38 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4f3c;
            }
            0x800d4f3c => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4f40;
            }
            0x800d4f40 => {
                c.lr = 0x800d4f44; f_800db9f0(c);
                pc = 0x800d4f44;
            }
            0x800d4f44 => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4f48;
            }
            0x800d4f48 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4f4c;
            }
            0x800d4f4c => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                pc = 0x800d4f50;
            }
            0x800d4f50 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x800d4f54;
            }
            0x800d4f54 => {
                c.r[31] = c.r[31].wrapping_add(0xc);
                pc = 0x800d4f58;
            }
            0x800d4f58 => {
                c.lr = 0x800d4f5c; f_800db988(c);
                pc = 0x800d4f5c;
            }
            0x800d4f5c => {
                pc = 0x800d4f7c; continue;
                pc = 0x800d4f60;
            }
            0x800d4f60 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x64));
                pc = 0x800d4f64;
            }
            0x800d4f64 => {
                c.r[3] = c.lwz(c.r[31]);
                pc = 0x800d4f68;
            }
            0x800d4f68 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x800d4f6c;
            }
            0x800d4f6c => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x800d4f70;
            }
            0x800d4f70 => {
                c.r[5] = c.r[0] as u8 as i8 as i32 as u32;
                pc = 0x800d4f74;
            }
            0x800d4f74 => {
                c.r[31] = c.r[31].wrapping_add(0x64);
                pc = 0x800d4f78;
            }
            0x800d4f78 => {
                c.lr = 0x800d4f7c; f_800db900(c);
                pc = 0x800d4f7c;
            }
            0x800d4f7c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x800d4f80;
            }
            0x800d4f80 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d4f84;
            }
            0x800d4f84 => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                pc = 0x800d4f88;
            }
            0x800d4f88 => {
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                pc = 0x800d4f8c;
            }
            0x800d4f8c => {
                c.mtspr(8, c.r[0]);
                pc = 0x800d4f90;
            }
            0x800d4f90 => {
                c.r[1] = c.r[1].wrapping_add(0x20);
                pc = 0x800d4f94;
            }
            0x800d4f94 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800d461c: bad pc {:#x}", pc),
        }
    }
}

/// gfxRecordCommand (0x800d4f98, 1388 bytes)
#[inline(never)]
pub fn f_800d4f98(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d4f98) { return; }
        if c.prof { crate::prof::enter(c, 0x800d4f98); f_800d4f98_b(c, 0x800d4f98); crate::prof::exit(c, 0x800d4f98); return; }
    }
    f_800d4f98_b(c, 0x800d4f98)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d4f98_at(c: &mut Cpu, pc: u32) { f_800d4f98_b(c, pc) }
#[inline(always)]
fn f_800d4f98_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d4f98;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d4f98, pc); }
        match pc {
            0x800d4f98 => {
                { let ea = c.r[1].wrapping_add(0xffffff70); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x800d4f9c;
            }
            0x800d4f9c => {
                c.r[0] = c.mfspr(8);
                pc = 0x800d4fa0;
            }
            0x800d4fa0 => {
                c.stw(c.r[1].wrapping_add(0x94), c.r[0]);
                pc = 0x800d4fa4;
            }
            0x800d4fa4 => {
                c.stw(c.r[1].wrapping_add(0x8c), c.r[31]);
                pc = 0x800d4fa8;
            }
            0x800d4fa8 => {
                c.stw(c.r[1].wrapping_add(0x88), c.r[30]);
                pc = 0x800d4fac;
            }
            0x800d4fac => {
                if !c.crbit(6) { pc = 0x800d4fd0; continue; }
                pc = 0x800d4fb0;
            }
            0x800d4fb0 => {
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[1]; c.stfd(ea, v); }
                pc = 0x800d4fb4;
            }
            0x800d4fb4 => {
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[2]; c.stfd(ea, v); }
                pc = 0x800d4fb8;
            }
            0x800d4fb8 => {
                { let ea = c.r[1].wrapping_add(0x38); let v = c.f[3]; c.stfd(ea, v); }
                pc = 0x800d4fbc;
            }
            0x800d4fbc => {
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[4]; c.stfd(ea, v); }
                pc = 0x800d4fc0;
            }
            0x800d4fc0 => {
                { let ea = c.r[1].wrapping_add(0x48); let v = c.f[5]; c.stfd(ea, v); }
                pc = 0x800d4fc4;
            }
            0x800d4fc4 => {
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[6]; c.stfd(ea, v); }
                pc = 0x800d4fc8;
            }
            0x800d4fc8 => {
                { let ea = c.r[1].wrapping_add(0x58); let v = c.f[7]; c.stfd(ea, v); }
                pc = 0x800d4fcc;
            }
            0x800d4fcc => {
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[8]; c.stfd(ea, v); }
                pc = 0x800d4fd0;
            }
            0x800d4fd0 => {
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                pc = 0x800d4fd4;
            }
            0x800d4fd4 => {
                c.cmpu(0, c.r[4], 0x14);
                pc = 0x800d4fd8;
            }
            0x800d4fd8 => {
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d4fdc;
            }
            0x800d4fdc => {
                c.r[11] = c.r[1].wrapping_add(0x98);
                pc = 0x800d4fe0;
            }
            0x800d4fe0 => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                pc = 0x800d4fe4;
            }
            0x800d4fe4 => {
                c.r[0] = c.r[1].wrapping_add(0x8);
                pc = 0x800d4fe8;
            }
            0x800d4fe8 => {
                c.r[12] = 0x2000000;
                pc = 0x800d4fec;
            }
            0x800d4fec => {
                c.r[31] = c.r[1].wrapping_add(0x70);
                pc = 0x800d4ff0;
            }
            0x800d4ff0 => {
                c.stw(c.r[1].wrapping_add(0x10), c.r[5]);
                pc = 0x800d4ff4;
            }
            0x800d4ff4 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[6]);
                pc = 0x800d4ff8;
            }
            0x800d4ff8 => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[7]);
                pc = 0x800d4ffc;
            }
            0x800d4ffc => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[8]);
                pc = 0x800d5000;
            }
            0x800d5000 => {
                c.stw(c.r[1].wrapping_add(0x20), c.r[9]);
                pc = 0x800d5004;
            }
            0x800d5004 => {
                c.stw(c.r[1].wrapping_add(0x24), c.r[10]);
                pc = 0x800d5008;
            }
            0x800d5008 => {
                c.r[6] = c.lwz(c.r[30].wrapping_add(0x494));
                pc = 0x800d500c;
            }
            0x800d500c => {
                c.r[5] = c.r[6].wrapping_add(0x4);
                pc = 0x800d5010;
            }
            0x800d5010 => {
                c.stw(c.r[30].wrapping_add(0x494), c.r[5]);
                pc = 0x800d5014;
            }
            0x800d5014 => {
                c.stw(c.r[6], c.r[3]);
                pc = 0x800d5018;
            }
            0x800d5018 => {
                c.stw(c.r[1].wrapping_add(0x70), c.r[12]);
                pc = 0x800d501c;
            }
            0x800d501c => {
                c.stw(c.r[1].wrapping_add(0x74), c.r[11]);
                pc = 0x800d5020;
            }
            0x800d5020 => {
                c.stw(c.r[1].wrapping_add(0x78), c.r[0]);
                pc = 0x800d5024;
            }
            0x800d5024 => {
                if c.crbit(1) { pc = 0x800d54ec; continue; }
                pc = 0x800d5028;
            }
            0x800d5028 => {
                c.r[3] = 0x80310000;
                pc = 0x800d502c;
            }
            0x800d502c => {
                c.r[0] = c.r[4].rotate_left(2) & 0xfffffffc;
                pc = 0x800d5030;
            }
            0x800d5030 => {
                c.r[3] = c.r[3].wrapping_add(0x42f8);
                pc = 0x800d5034;
            }
            0x800d5034 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x800d5038;
            }
            0x800d5038 => {
                c.mtspr(9, c.r[0]);
                pc = 0x800d503c;
            }
            0x800d503c => {
                if true { let t = c.ctr; if (0x800d4f98..0x800d5504).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x800d5040;
            }
            0x800d5040 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d5044;
            }
            0x800d5044 => {
                c.r[4] = 0x1;
                pc = 0x800d5048;
            }
            0x800d5048 => {
                c.lr = 0x800d504c; f_800c45a0(c);
                pc = 0x800d504c;
            }
            0x800d504c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5050;
            }
            0x800d5050 => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d5054;
            }
            0x800d5054 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d5058;
            }
            0x800d5058 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d505c;
            }
            0x800d505c => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5060;
            }
            0x800d5060 => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d5064;
            }
            0x800d5064 => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d5068;
            }
            0x800d5068 => {
                c.r[4] = 0x1;
                pc = 0x800d506c;
            }
            0x800d506c => {
                c.lr = 0x800d5070; f_800c45a0(c);
                pc = 0x800d5070;
            }
            0x800d5070 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5074;
            }
            0x800d5074 => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d5078;
            }
            0x800d5078 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d507c;
            }
            0x800d507c => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d5080;
            }
            0x800d5080 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5084;
            }
            0x800d5084 => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d5088;
            }
            0x800d5088 => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d508c;
            }
            0x800d508c => {
                c.r[4] = 0x1;
                pc = 0x800d5090;
            }
            0x800d5090 => {
                c.lr = 0x800d5094; f_800c45a0(c);
                pc = 0x800d5094;
            }
            0x800d5094 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5098;
            }
            0x800d5098 => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d509c;
            }
            0x800d509c => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d50a0;
            }
            0x800d50a0 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d50a4;
            }
            0x800d50a4 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d50a8;
            }
            0x800d50a8 => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d50ac;
            }
            0x800d50ac => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d50b0;
            }
            0x800d50b0 => {
                c.r[4] = 0x1;
                pc = 0x800d50b4;
            }
            0x800d50b4 => {
                c.lr = 0x800d50b8; f_800c45a0(c);
                pc = 0x800d50b8;
            }
            0x800d50b8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d50bc;
            }
            0x800d50bc => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d50c0;
            }
            0x800d50c0 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d50c4;
            }
            0x800d50c4 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d50c8;
            }
            0x800d50c8 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d50cc;
            }
            0x800d50cc => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d50d0;
            }
            0x800d50d0 => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d50d4;
            }
            0x800d50d4 => {
                c.r[4] = 0x1;
                pc = 0x800d50d8;
            }
            0x800d50d8 => {
                c.lr = 0x800d50dc; f_800c45a0(c);
                pc = 0x800d50dc;
            }
            0x800d50dc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d50e0;
            }
            0x800d50e0 => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d50e4;
            }
            0x800d50e4 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d50e8;
            }
            0x800d50e8 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d50ec;
            }
            0x800d50ec => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d50f0;
            }
            0x800d50f0 => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d50f4;
            }
            0x800d50f4 => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d50f8;
            }
            0x800d50f8 => {
                c.r[4] = 0x1;
                pc = 0x800d50fc;
            }
            0x800d50fc => {
                c.lr = 0x800d5100; f_800c45a0(c);
                pc = 0x800d5100;
            }
            0x800d5100 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5104;
            }
            0x800d5104 => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d5108;
            }
            0x800d5108 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d510c;
            }
            0x800d510c => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d5110;
            }
            0x800d5110 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5114;
            }
            0x800d5114 => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d5118;
            }
            0x800d5118 => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d511c;
            }
            0x800d511c => {
                c.r[4] = 0x1;
                pc = 0x800d5120;
            }
            0x800d5120 => {
                c.lr = 0x800d5124; f_800c45a0(c);
                pc = 0x800d5124;
            }
            0x800d5124 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5128;
            }
            0x800d5128 => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d512c;
            }
            0x800d512c => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d5130;
            }
            0x800d5130 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d5134;
            }
            0x800d5134 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5138;
            }
            0x800d5138 => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d513c;
            }
            0x800d513c => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d5140;
            }
            0x800d5140 => {
                c.r[4] = 0x1;
                pc = 0x800d5144;
            }
            0x800d5144 => {
                c.lr = 0x800d5148; f_800c45a0(c);
                pc = 0x800d5148;
            }
            0x800d5148 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d514c;
            }
            0x800d514c => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d5150;
            }
            0x800d5150 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d5154;
            }
            0x800d5154 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d5158;
            }
            0x800d5158 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d515c;
            }
            0x800d515c => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d5160;
            }
            0x800d5160 => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d5164;
            }
            0x800d5164 => {
                c.r[4] = 0x1;
                pc = 0x800d5168;
            }
            0x800d5168 => {
                c.lr = 0x800d516c; f_800c45a0(c);
                pc = 0x800d516c;
            }
            0x800d516c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5170;
            }
            0x800d5170 => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d5174;
            }
            0x800d5174 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d5178;
            }
            0x800d5178 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d517c;
            }
            0x800d517c => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5180;
            }
            0x800d5180 => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d5184;
            }
            0x800d5184 => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d5188;
            }
            0x800d5188 => {
                c.r[4] = 0x1;
                pc = 0x800d518c;
            }
            0x800d518c => {
                c.lr = 0x800d5190; f_800c45a0(c);
                pc = 0x800d5190;
            }
            0x800d5190 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5194;
            }
            0x800d5194 => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d5198;
            }
            0x800d5198 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d519c;
            }
            0x800d519c => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d51a0;
            }
            0x800d51a0 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d51a4;
            }
            0x800d51a4 => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d51a8;
            }
            0x800d51a8 => {
                pc = 0x800d54ec; continue;
                pc = 0x800d51ac;
            }
            0x800d51ac => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d51b0;
            }
            0x800d51b0 => {
                c.r[4] = 0x3;
                pc = 0x800d51b4;
            }
            0x800d51b4 => {
                c.lr = 0x800d51b8; f_800c45a0(c);
                pc = 0x800d51b8;
            }
            0x800d51b8 => {
                { let ea = c.r[3]; c.f[0] = c.lfd(ea); }
                pc = 0x800d51bc;
            }
            0x800d51bc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d51c0;
            }
            0x800d51c0 => {
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x800d51c4;
            }
            0x800d51c4 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d51c8;
            }
            0x800d51c8 => {
                { let ea = c.r[1].wrapping_add(0x6c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d51cc;
            }
            0x800d51cc => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d51d0;
            }
            0x800d51d0 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d51d4;
            }
            0x800d51d4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x6c));
                pc = 0x800d51d8;
            }
            0x800d51d8 => {
                c.stw(c.r[3], c.r[0]);
                pc = 0x800d51dc;
            }
            0x800d51dc => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d51e0;
            }
            0x800d51e0 => {
                c.r[4] = 0x3;
                pc = 0x800d51e4;
            }
            0x800d51e4 => {
                c.lr = 0x800d51e8; f_800c45a0(c);
                pc = 0x800d51e8;
            }
            0x800d51e8 => {
                { let ea = c.r[3]; c.f[0] = c.lfd(ea); }
                pc = 0x800d51ec;
            }
            0x800d51ec => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d51f0;
            }
            0x800d51f0 => {
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x800d51f4;
            }
            0x800d51f4 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d51f8;
            }
            0x800d51f8 => {
                { let ea = c.r[1].wrapping_add(0x6c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d51fc;
            }
            0x800d51fc => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d5200;
            }
            0x800d5200 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5204;
            }
            0x800d5204 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x6c));
                pc = 0x800d5208;
            }
            0x800d5208 => {
                c.stw(c.r[3], c.r[0]);
                pc = 0x800d520c;
            }
            0x800d520c => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d5210;
            }
            0x800d5210 => {
                c.r[4] = 0x3;
                pc = 0x800d5214;
            }
            0x800d5214 => {
                c.lr = 0x800d5218; f_800c45a0(c);
                pc = 0x800d5218;
            }
            0x800d5218 => {
                { let ea = c.r[3]; c.f[0] = c.lfd(ea); }
                pc = 0x800d521c;
            }
            0x800d521c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5220;
            }
            0x800d5220 => {
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x800d5224;
            }
            0x800d5224 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d5228;
            }
            0x800d5228 => {
                { let ea = c.r[1].wrapping_add(0x6c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d522c;
            }
            0x800d522c => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d5230;
            }
            0x800d5230 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5234;
            }
            0x800d5234 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x6c));
                pc = 0x800d5238;
            }
            0x800d5238 => {
                c.stw(c.r[3], c.r[0]);
                pc = 0x800d523c;
            }
            0x800d523c => {
                c.r[3] = c.r[1].wrapping_add(0x70);
                pc = 0x800d5240;
            }
            0x800d5240 => {
                c.r[4] = 0x3;
                pc = 0x800d5244;
            }
            0x800d5244 => {
                c.lr = 0x800d5248; f_800c45a0(c);
                pc = 0x800d5248;
            }
            0x800d5248 => {
                { let ea = c.r[3]; c.f[0] = c.lfd(ea); }
                pc = 0x800d524c;
            }
            0x800d524c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5250;
            }
            0x800d5250 => {
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x800d5254;
            }
            0x800d5254 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d5258;
            }
            0x800d5258 => {
                { let ea = c.r[1].wrapping_add(0x6c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d525c;
            }
            0x800d525c => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d5260;
            }
            0x800d5260 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5264;
            }
            0x800d5264 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x6c));
                pc = 0x800d5268;
            }
            0x800d5268 => {
                c.stw(c.r[3], c.r[0]);
                pc = 0x800d526c;
            }
            0x800d526c => {
                pc = 0x800d54ec; continue;
                pc = 0x800d5270;
            }
            0x800d5270 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d5274;
            }
            0x800d5274 => {
                c.r[4] = 0x1;
                pc = 0x800d5278;
            }
            0x800d5278 => {
                c.lr = 0x800d527c; f_800c45a0(c);
                pc = 0x800d527c;
            }
            0x800d527c => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5280;
            }
            0x800d5280 => {
                c.r[4] = 0x3;
                pc = 0x800d5284;
            }
            0x800d5284 => {
                c.r[7] = c.lwz(c.r[3]);
                pc = 0x800d5288;
            }
            0x800d5288 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d528c;
            }
            0x800d528c => {
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d5290;
            }
            0x800d5290 => {
                c.r[0] = c.r[5].wrapping_add(0x4);
                pc = 0x800d5294;
            }
            0x800d5294 => {
                c.stw(c.r[6].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5298;
            }
            0x800d5298 => {
                c.stw(c.r[5], c.r[7]);
                pc = 0x800d529c;
            }
            0x800d529c => {
                c.lr = 0x800d52a0; f_800c45a0(c);
                pc = 0x800d52a0;
            }
            0x800d52a0 => {
                { let ea = c.r[3]; c.f[0] = c.lfd(ea); }
                pc = 0x800d52a4;
            }
            0x800d52a4 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d52a8;
            }
            0x800d52a8 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d52ac;
            }
            0x800d52ac => {
                c.r[4] = 0x3;
                pc = 0x800d52b0;
            }
            0x800d52b0 => {
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x800d52b4;
            }
            0x800d52b4 => {
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d52b8;
            }
            0x800d52b8 => {
                { let ea = c.r[1].wrapping_add(0x6c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d52bc;
            }
            0x800d52bc => {
                c.r[0] = c.r[5].wrapping_add(0x4);
                pc = 0x800d52c0;
            }
            0x800d52c0 => {
                c.stw(c.r[6].wrapping_add(0x494), c.r[0]);
                pc = 0x800d52c4;
            }
            0x800d52c4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x6c));
                pc = 0x800d52c8;
            }
            0x800d52c8 => {
                c.stw(c.r[5], c.r[0]);
                pc = 0x800d52cc;
            }
            0x800d52cc => {
                c.lr = 0x800d52d0; f_800c45a0(c);
                pc = 0x800d52d0;
            }
            0x800d52d0 => {
                { let ea = c.r[3]; c.f[0] = c.lfd(ea); }
                pc = 0x800d52d4;
            }
            0x800d52d4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d52d8;
            }
            0x800d52d8 => {
                { let v = s(c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x800d52dc;
            }
            0x800d52dc => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d52e0;
            }
            0x800d52e0 => {
                { let ea = c.r[1].wrapping_add(0x6c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x800d52e4;
            }
            0x800d52e4 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d52e8;
            }
            0x800d52e8 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d52ec;
            }
            0x800d52ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x6c));
                pc = 0x800d52f0;
            }
            0x800d52f0 => {
                c.stw(c.r[3], c.r[0]);
                pc = 0x800d52f4;
            }
            0x800d52f4 => {
                pc = 0x800d54ec; continue;
                pc = 0x800d52f8;
            }
            0x800d52f8 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d52fc;
            }
            0x800d52fc => {
                c.r[4] = 0x1;
                pc = 0x800d5300;
            }
            0x800d5300 => {
                c.lr = 0x800d5304; f_800c45a0(c);
                pc = 0x800d5304;
            }
            0x800d5304 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5308;
            }
            0x800d5308 => {
                c.r[5] = 0x30;
                pc = 0x800d530c;
            }
            0x800d530c => {
                c.r[4] = c.lwz(c.r[3]);
                pc = 0x800d5310;
            }
            0x800d5310 => {
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d5314;
            }
            0x800d5314 => {
                c.lr = 0x800d5318; f_800054f4(c);
                pc = 0x800d5318;
            }
            0x800d5318 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d531c;
            }
            0x800d531c => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d5320;
            }
            0x800d5320 => {
                c.r[0] = c.r[3].wrapping_add(0x30);
                pc = 0x800d5324;
            }
            0x800d5324 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5328;
            }
            0x800d5328 => {
                pc = 0x800d54ec; continue;
                pc = 0x800d532c;
            }
            0x800d532c => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d5330;
            }
            0x800d5330 => {
                c.r[4] = 0x1;
                pc = 0x800d5334;
            }
            0x800d5334 => {
                c.lr = 0x800d5338; f_800c45a0(c);
                pc = 0x800d5338;
            }
            0x800d5338 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d533c;
            }
            0x800d533c => {
                c.r[4] = 0x1;
                pc = 0x800d5340;
            }
            0x800d5340 => {
                c.r[7] = c.lwz(c.r[3]);
                pc = 0x800d5344;
            }
            0x800d5344 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d5348;
            }
            0x800d5348 => {
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d534c;
            }
            0x800d534c => {
                c.r[0] = c.r[5].wrapping_add(0x4);
                pc = 0x800d5350;
            }
            0x800d5350 => {
                c.stw(c.r[6].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5354;
            }
            0x800d5354 => {
                c.stw(c.r[5], c.r[7]);
                pc = 0x800d5358;
            }
            0x800d5358 => {
                c.lr = 0x800d535c; f_800c45a0(c);
                pc = 0x800d535c;
            }
            0x800d535c => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5360;
            }
            0x800d5360 => {
                c.r[5] = 0x30;
                pc = 0x800d5364;
            }
            0x800d5364 => {
                c.r[4] = c.lwz(c.r[3]);
                pc = 0x800d5368;
            }
            0x800d5368 => {
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d536c;
            }
            0x800d536c => {
                c.lr = 0x800d5370; f_800054f4(c);
                pc = 0x800d5370;
            }
            0x800d5370 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5374;
            }
            0x800d5374 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d5378;
            }
            0x800d5378 => {
                c.r[0] = c.r[3].wrapping_add(0x30);
                pc = 0x800d537c;
            }
            0x800d537c => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5380;
            }
            0x800d5380 => {
                pc = 0x800d54ec; continue;
                pc = 0x800d5384;
            }
            0x800d5384 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d5388;
            }
            0x800d5388 => {
                c.r[4] = 0x1;
                pc = 0x800d538c;
            }
            0x800d538c => {
                c.lr = 0x800d5390; f_800c45a0(c);
                pc = 0x800d5390;
            }
            0x800d5390 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5394;
            }
            0x800d5394 => {
                c.r[4] = 0x1;
                pc = 0x800d5398;
            }
            0x800d5398 => {
                c.r[7] = c.lwz(c.r[3]);
                pc = 0x800d539c;
            }
            0x800d539c => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d53a0;
            }
            0x800d53a0 => {
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d53a4;
            }
            0x800d53a4 => {
                c.r[0] = c.r[5].wrapping_add(0x4);
                pc = 0x800d53a8;
            }
            0x800d53a8 => {
                c.stw(c.r[6].wrapping_add(0x494), c.r[0]);
                pc = 0x800d53ac;
            }
            0x800d53ac => {
                c.stw(c.r[5], c.r[7]);
                pc = 0x800d53b0;
            }
            0x800d53b0 => {
                c.lr = 0x800d53b4; f_800c45a0(c);
                pc = 0x800d53b4;
            }
            0x800d53b4 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d53b8;
            }
            0x800d53b8 => {
                c.r[5] = 0x18;
                pc = 0x800d53bc;
            }
            0x800d53bc => {
                c.r[4] = c.lwz(c.r[3]);
                pc = 0x800d53c0;
            }
            0x800d53c0 => {
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d53c4;
            }
            0x800d53c4 => {
                c.lr = 0x800d53c8; f_800054f4(c);
                pc = 0x800d53c8;
            }
            0x800d53c8 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d53cc;
            }
            0x800d53cc => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d53d0;
            }
            0x800d53d0 => {
                c.r[4] = 0x1;
                pc = 0x800d53d4;
            }
            0x800d53d4 => {
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d53d8;
            }
            0x800d53d8 => {
                c.r[0] = c.r[5].wrapping_add(0x60);
                pc = 0x800d53dc;
            }
            0x800d53dc => {
                c.stw(c.r[6].wrapping_add(0x494), c.r[0]);
                pc = 0x800d53e0;
            }
            0x800d53e0 => {
                c.lr = 0x800d53e4; f_800c45a0(c);
                pc = 0x800d53e4;
            }
            0x800d53e4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d53e8;
            }
            0x800d53e8 => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d53ec;
            }
            0x800d53ec => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d53f0;
            }
            0x800d53f0 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d53f4;
            }
            0x800d53f4 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d53f8;
            }
            0x800d53f8 => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d53fc;
            }
            0x800d53fc => {
                pc = 0x800d54ec; continue;
                pc = 0x800d5400;
            }
            0x800d5400 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d5404;
            }
            0x800d5404 => {
                c.r[4] = 0x1;
                pc = 0x800d5408;
            }
            0x800d5408 => {
                c.lr = 0x800d540c; f_800c45a0(c);
                pc = 0x800d540c;
            }
            0x800d540c => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5410;
            }
            0x800d5410 => {
                c.r[4] = 0x1;
                pc = 0x800d5414;
            }
            0x800d5414 => {
                c.r[7] = c.lwz(c.r[3]);
                pc = 0x800d5418;
            }
            0x800d5418 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d541c;
            }
            0x800d541c => {
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d5420;
            }
            0x800d5420 => {
                c.r[0] = c.r[5].wrapping_add(0x4);
                pc = 0x800d5424;
            }
            0x800d5424 => {
                c.stw(c.r[6].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5428;
            }
            0x800d5428 => {
                c.stw(c.r[5], c.r[7]);
                pc = 0x800d542c;
            }
            0x800d542c => {
                c.lr = 0x800d5430; f_800c45a0(c);
                pc = 0x800d5430;
            }
            0x800d5430 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5434;
            }
            0x800d5434 => {
                c.r[4] = 0x1;
                pc = 0x800d5438;
            }
            0x800d5438 => {
                c.r[7] = c.lwz(c.r[3]);
                pc = 0x800d543c;
            }
            0x800d543c => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d5440;
            }
            0x800d5440 => {
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d5444;
            }
            0x800d5444 => {
                c.r[0] = c.r[5].wrapping_add(0x4);
                pc = 0x800d5448;
            }
            0x800d5448 => {
                c.stw(c.r[6].wrapping_add(0x494), c.r[0]);
                pc = 0x800d544c;
            }
            0x800d544c => {
                c.stw(c.r[5], c.r[7]);
                pc = 0x800d5450;
            }
            0x800d5450 => {
                c.lr = 0x800d5454; f_800c45a0(c);
                pc = 0x800d5454;
            }
            0x800d5454 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5458;
            }
            0x800d5458 => {
                c.r[4] = 0x1;
                pc = 0x800d545c;
            }
            0x800d545c => {
                c.r[7] = c.lwz(c.r[3]);
                pc = 0x800d5460;
            }
            0x800d5460 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d5464;
            }
            0x800d5464 => {
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d5468;
            }
            0x800d5468 => {
                c.r[0] = c.r[5].wrapping_add(0x4);
                pc = 0x800d546c;
            }
            0x800d546c => {
                c.stw(c.r[6].wrapping_add(0x494), c.r[0]);
                pc = 0x800d5470;
            }
            0x800d5470 => {
                c.stw(c.r[5], c.r[7]);
                pc = 0x800d5474;
            }
            0x800d5474 => {
                c.lr = 0x800d5478; f_800c45a0(c);
                pc = 0x800d5478;
            }
            0x800d5478 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d547c;
            }
            0x800d547c => {
                c.r[5] = 0x30;
                pc = 0x800d5480;
            }
            0x800d5480 => {
                c.r[4] = c.lwz(c.r[3]);
                pc = 0x800d5484;
            }
            0x800d5484 => {
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d5488;
            }
            0x800d5488 => {
                c.lr = 0x800d548c; f_800054f4(c);
                pc = 0x800d548c;
            }
            0x800d548c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d5490;
            }
            0x800d5490 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d5494;
            }
            0x800d5494 => {
                c.r[0] = c.r[3].wrapping_add(0x30);
                pc = 0x800d5498;
            }
            0x800d5498 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d549c;
            }
            0x800d549c => {
                pc = 0x800d54ec; continue;
                pc = 0x800d54a0;
            }
            0x800d54a0 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d54a4;
            }
            0x800d54a4 => {
                c.r[4] = 0x1;
                pc = 0x800d54a8;
            }
            0x800d54a8 => {
                c.lr = 0x800d54ac; f_800c45a0(c);
                pc = 0x800d54ac;
            }
            0x800d54ac => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d54b0;
            }
            0x800d54b0 => {
                c.r[4] = 0x0;
                pc = 0x800d54b4;
            }
            0x800d54b4 => {
                c.r[7] = c.lwz(c.r[3]);
                pc = 0x800d54b8;
            }
            0x800d54b8 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x800d54bc;
            }
            0x800d54bc => {
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x494));
                pc = 0x800d54c0;
            }
            0x800d54c0 => {
                c.r[0] = c.r[5].wrapping_add(0x4);
                pc = 0x800d54c4;
            }
            0x800d54c4 => {
                c.stw(c.r[6].wrapping_add(0x494), c.r[0]);
                pc = 0x800d54c8;
            }
            0x800d54c8 => {
                c.stw(c.r[5], c.r[7]);
                pc = 0x800d54cc;
            }
            0x800d54cc => {
                c.lr = 0x800d54d0; f_800c45a0(c);
                pc = 0x800d54d0;
            }
            0x800d54d0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                pc = 0x800d54d4;
            }
            0x800d54d4 => {
                c.r[5] = c.lwz(c.r[3]);
                pc = 0x800d54d8;
            }
            0x800d54d8 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x494));
                pc = 0x800d54dc;
            }
            0x800d54dc => {
                c.stw(c.r[1].wrapping_add(0x68), c.r[5]);
                pc = 0x800d54e0;
            }
            0x800d54e0 => {
                c.r[0] = c.r[3].wrapping_add(0x4);
                pc = 0x800d54e4;
            }
            0x800d54e4 => {
                c.stw(c.r[4].wrapping_add(0x494), c.r[0]);
                pc = 0x800d54e8;
            }
            0x800d54e8 => {
                c.stw(c.r[3], c.r[5]);
                pc = 0x800d54ec;
            }
            0x800d54ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x94));
                pc = 0x800d54f0;
            }
            0x800d54f0 => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x8c));
                pc = 0x800d54f4;
            }
            0x800d54f4 => {
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x88));
                pc = 0x800d54f8;
            }
            0x800d54f8 => {
                c.mtspr(8, c.r[0]);
                pc = 0x800d54fc;
            }
            0x800d54fc => {
                c.r[1] = c.r[1].wrapping_add(0x90);
                pc = 0x800d5500;
            }
            0x800d5500 => {
                if true { return; }
                return;
            }
            _ => panic!("f_800d4f98: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5504 (0x800d5504, 204 bytes)
#[inline(never)]
pub fn f_800d5504(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5504) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5504); f_800d5504_b(c, 0x800d5504); crate::prof::exit(c, 0x800d5504); return; }
    }
    f_800d5504_b(c, 0x800d5504)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5504_at(c: &mut Cpu, pc: u32) { f_800d5504_b(c, pc) }
#[inline(always)]
fn f_800d5504_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5504;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5504, pc); }
        match pc {
            0x800d5504 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[5].wrapping_add(0x3c0);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x490));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d5544; continue; }
                pc = 0x800d5534;
            }
            0x800d5534 => {
                c.r[3] = c.r[31].wrapping_add(0x0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5540; stubs_gen::f_800dd970(c);
                pc = 0x800d5540;
            }
            0x800d5540 => {
                pc = 0x800d55b8; continue;
                pc = 0x800d5544;
            }
            0x800d5544 => {
                c.lr = 0x800d5548; f_800e3534(c);
                pc = 0x800d5548;
            }
            0x800d5548 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x48c), c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x48c));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800d5570; continue; }
                pc = 0x800d5560;
            }
            0x800d5560 => {
                c.r[3] = c.r[31].wrapping_add(0x28);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d556c; stubs_gen::f_800dd970(c);
                pc = 0x800d556c;
            }
            0x800d556c => {
                pc = 0x800d55b8; continue;
                pc = 0x800d5570;
            }
            0x800d5570 => {
                c.lr = 0x800d5574; f_800e27b0(c);
                pc = 0x800d5574;
            }
            0x800d5574 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = 0x80400000;
                c.r[4] = c.r[4].wrapping_add(0x1f0);
                c.r[0] = 0x0;
                c.stw(c.r[5].wrapping_add(0x490), c.r[3]);
                c.r[3] = c.r[31].wrapping_add(0x4c);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = c.lwz(c.r[6].wrapping_add(0x490));
                c.stw(c.r[6].wrapping_add(0x494), c.r[5]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[5].wrapping_add(0x498), c.r[30]);
                c.stw(c.r[4].wrapping_add(0x28), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.lwz(c.r[5].wrapping_add(0x498));
                c.r[5] = c.lwz(c.r[5].wrapping_add(0x490));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d55b8; stubs_gen::f_800dd970(c);
                pc = 0x800d55b8;
            }
            0x800d55b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5504: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D55D0 (0x800d55d0, 120 bytes)
#[inline(never)]
pub fn f_800d55d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d55d0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d55d0); f_800d55d0_b(c, 0x800d55d0); crate::prof::exit(c, 0x800d55d0); return; }
    }
    f_800d55d0_b(c, 0x800d55d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d55d0_at(c: &mut Cpu, pc: u32) { f_800d55d0_b(c, pc) }
#[inline(always)]
fn f_800d55d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d55d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d55d0, pc); }
        match pc {
            0x800d55d0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffff9390); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.fcmp(0, c.f[1], c.f[0]);
                if c.crbit(0) { pc = 0x800d5638; continue; }
                pc = 0x800d55e8;
            }
            0x800d55e8 => {
                { let ea = c.r[2].wrapping_add(0xffff9394); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d55f8; continue; }
                pc = 0x800d55f4;
            }
            0x800d55f4 => {
                pc = 0x800d5638; continue;
                pc = 0x800d55f8;
            }
            0x800d55f8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d561c; continue; }
                pc = 0x800d5608;
            }
            0x800d5608 => {
                c.r[3] = 0x25;
                c.r[4] = 0xb;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d5618; f_800d4f98(c);
                pc = 0x800d5618;
            }
            0x800d5618 => {
                pc = 0x800d5638; continue;
                pc = 0x800d561c;
            }
            0x800d561c => {
                { let ea = c.r[2].wrapping_add(0xffff9398); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = 0x0;
                { let v = s(c.f[0] * c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[3] = c.lwz(c.r[1].wrapping_add(0xc));
                c.lr = 0x800d5638; f_800b944c(c);
                pc = 0x800d5638;
            }
            0x800d5638 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d55d0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5648 (0x800d5648, 120 bytes)
#[inline(never)]
pub fn f_800d5648(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5648) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5648); f_800d5648_b(c, 0x800d5648); crate::prof::exit(c, 0x800d5648); return; }
    }
    f_800d5648_b(c, 0x800d5648)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5648_at(c: &mut Cpu, pc: u32) { f_800d5648_b(c, pc) }
#[inline(always)]
fn f_800d5648_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5648;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5648, pc); }
        match pc {
            0x800d5648 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let ea = c.r[2].wrapping_add(0xffff9390); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.fcmp(0, c.f[1], c.f[0]);
                if c.crbit(0) { pc = 0x800d56b0; continue; }
                pc = 0x800d5660;
            }
            0x800d5660 => {
                { let ea = c.r[2].wrapping_add(0xffff9394); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x800d5670; continue; }
                pc = 0x800d566c;
            }
            0x800d566c => {
                pc = 0x800d56b0; continue;
                pc = 0x800d5670;
            }
            0x800d5670 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5694; continue; }
                pc = 0x800d5680;
            }
            0x800d5680 => {
                c.r[3] = 0x24;
                c.r[4] = 0xb;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d5690; f_800d4f98(c);
                pc = 0x800d5690;
            }
            0x800d5690 => {
                pc = 0x800d56b0; continue;
                pc = 0x800d5694;
            }
            0x800d5694 => {
                { let ea = c.r[2].wrapping_add(0xffff9398); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = 0x0;
                { let v = s(c.f[0] * c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                c.r[3] = c.lwz(c.r[1].wrapping_add(0xc));
                c.lr = 0x800d56b0; f_800b9404(c);
                pc = 0x800d56b0;
            }
            0x800d56b0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5648: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D56C0 (0x800d56c0, 100 bytes)
#[inline(never)]
pub fn f_800d56c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d56c0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d56c0); f_800d56c0_b(c, 0x800d56c0); crate::prof::exit(c, 0x800d56c0); return; }
    }
    f_800d56c0_b(c, 0x800d56c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d56c0_at(c: &mut Cpu, pc: u32) { f_800d56c0_b(c, pc) }
#[inline(always)]
fn f_800d56c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d56c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d56c0, pc); }
        match pc {
            0x800d56c0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5700; continue; }
                pc = 0x800d56dc;
            }
            0x800d56dc => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5700; continue; }
                pc = 0x800d56e8;
            }
            0x800d56e8 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x23;
                c.r[4] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d56fc; f_800d4f98(c);
                pc = 0x800d56fc;
            }
            0x800d56fc => {
                pc = 0x800d5714; continue;
                pc = 0x800d5700;
            }
            0x800d5700 => {
                c.r[4] = 0x800d0000;
                c.r[0] = c.r[4].wrapping_add(0x7230);
                c.stw(c.r[5].wrapping_add(0x4a0), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x4a4), c.r[3]);
                pc = 0x800d5714;
            }
            0x800d5714 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d56c0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5724 (0x800d5724, 120 bytes)
#[inline(never)]
pub fn f_800d5724(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5724) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5724); f_800d5724_b(c, 0x800d5724); crate::prof::exit(c, 0x800d5724); return; }
    }
    f_800d5724_b(c, 0x800d5724)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5724_at(c: &mut Cpu, pc: u32) { f_800d5724_b(c, pc) }
#[inline(always)]
fn f_800d5724_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5724;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5724, pc); }
        match pc {
            0x800d5724 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5768; continue; }
                pc = 0x800d5740;
            }
            0x800d5740 => {
                c.r[0] = c.lwz(c.r[7]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5768; continue; }
                pc = 0x800d574c;
            }
            0x800d574c => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4].rotate_left(0) & 0xff;
                c.r[3] = 0x22;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5764; f_800d4f98(c);
                pc = 0x800d5764;
            }
            0x800d5764 => {
                pc = 0x800d578c; continue;
                pc = 0x800d5768;
            }
            0x800d5768 => {
                c.r[5] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[6] = 0x800d0000;
                c.r[6] = c.r[6].wrapping_add(0x724c);
                c.r[0] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[7].wrapping_add(c.r[5]);
                c.stw(c.r[3].wrapping_add(0x500), c.r[6]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x520), c.r[4]);
                pc = 0x800d578c;
            }
            0x800d578c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5724: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D579C (0x800d579c, 120 bytes)
#[inline(never)]
pub fn f_800d579c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d579c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d579c); f_800d579c_b(c, 0x800d579c); crate::prof::exit(c, 0x800d579c); return; }
    }
    f_800d579c_b(c, 0x800d579c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d579c_at(c: &mut Cpu, pc: u32) { f_800d579c_b(c, pc) }
#[inline(always)]
fn f_800d579c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d579c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d579c, pc); }
        match pc {
            0x800d579c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d57e0; continue; }
                pc = 0x800d57b8;
            }
            0x800d57b8 => {
                c.r[0] = c.lwz(c.r[7]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d57e0; continue; }
                pc = 0x800d57c4;
            }
            0x800d57c4 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4].rotate_left(0) & 0xffff;
                c.r[3] = 0x21;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d57dc; f_800d4f98(c);
                pc = 0x800d57dc;
            }
            0x800d57dc => {
                pc = 0x800d5804; continue;
                pc = 0x800d57e0;
            }
            0x800d57e0 => {
                c.r[5] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[6] = 0x800d0000;
                c.r[6] = c.r[6].wrapping_add(0x7268);
                c.r[0] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[7].wrapping_add(c.r[5]);
                c.stw(c.r[3].wrapping_add(0x500), c.r[6]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.sth(c.r[3].wrapping_add(0x522), c.r[4]);
                pc = 0x800d5804;
            }
            0x800d5804 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d579c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5814 (0x800d5814, 140 bytes)
#[inline(never)]
pub fn f_800d5814(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5814) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5814); f_800d5814_b(c, 0x800d5814); crate::prof::exit(c, 0x800d5814); return; }
    }
    f_800d5814_b(c, 0x800d5814)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5814_at(c: &mut Cpu, pc: u32) { f_800d5814_b(c, pc) }
#[inline(always)]
fn f_800d5814_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5814;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5814, pc); }
        match pc {
            0x800d5814 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[8] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5860; continue; }
                pc = 0x800d5834;
            }
            0x800d5834 => {
                c.r[0] = c.lwz(c.r[6]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5860; continue; }
                pc = 0x800d5840;
            }
            0x800d5840 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4].rotate_left(0) & 0xff;
                c.r[7] = c.r[8].rotate_left(0) & 0xff;
                c.r[3] = 0x1f;
                c.r[4] = 0x3;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d585c; f_800d4f98(c);
                pc = 0x800d585c;
            }
            0x800d585c => {
                pc = 0x800d5890; continue;
                pc = 0x800d5860;
            }
            0x800d5860 => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[5] = 0x800d0000;
                c.r[5] = c.r[5].wrapping_add(0x72a4);
                c.r[7] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[6].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x500), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[7]);
                c.stb(c.r[3].wrapping_add(0x520), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[7]);
                c.stb(c.r[3].wrapping_add(0x521), c.r[8]);
                pc = 0x800d5890;
            }
            0x800d5890 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5814: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D58A0 (0x800d58a0, 140 bytes)
#[inline(never)]
pub fn f_800d58a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d58a0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d58a0); f_800d58a0_b(c, 0x800d58a0); crate::prof::exit(c, 0x800d58a0); return; }
    }
    f_800d58a0_b(c, 0x800d58a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d58a0_at(c: &mut Cpu, pc: u32) { f_800d58a0_b(c, pc) }
#[inline(always)]
fn f_800d58a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d58a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d58a0, pc); }
        match pc {
            0x800d58a0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[8] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d58ec; continue; }
                pc = 0x800d58c0;
            }
            0x800d58c0 => {
                c.r[0] = c.lwz(c.r[6]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d58ec; continue; }
                pc = 0x800d58cc;
            }
            0x800d58cc => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4] as u16 as i16 as i32 as u32;
                c.r[7] = c.r[8] as u16 as i16 as i32 as u32;
                c.r[3] = 0x1e;
                c.r[4] = 0x3;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d58e8; f_800d4f98(c);
                pc = 0x800d58e8;
            }
            0x800d58e8 => {
                pc = 0x800d591c; continue;
                pc = 0x800d58ec;
            }
            0x800d58ec => {
                c.r[5] = 0x800d0000;
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[7] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[5] = c.r[5].wrapping_add(0x72c4);
                c.r[3] = c.r[6].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x500), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[7]);
                c.sth(c.r[3].wrapping_add(0x522), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[7]);
                c.sth(c.r[3].wrapping_add(0x524), c.r[8]);
                pc = 0x800d591c;
            }
            0x800d591c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d58a0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D592C (0x800d592c, 140 bytes)
#[inline(never)]
pub fn f_800d592c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d592c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d592c); f_800d592c_b(c, 0x800d592c); crate::prof::exit(c, 0x800d592c); return; }
    }
    f_800d592c_b(c, 0x800d592c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d592c_at(c: &mut Cpu, pc: u32) { f_800d592c_b(c, pc) }
#[inline(always)]
fn f_800d592c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d592c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d592c, pc); }
        match pc {
            0x800d592c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[8] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5978; continue; }
                pc = 0x800d594c;
            }
            0x800d594c => {
                c.r[0] = c.lwz(c.r[6]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5978; continue; }
                pc = 0x800d5958;
            }
            0x800d5958 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4].rotate_left(0) & 0xffff;
                c.r[7] = c.r[8].rotate_left(0) & 0xffff;
                c.r[3] = 0x1d;
                c.r[4] = 0x3;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5974; f_800d4f98(c);
                pc = 0x800d5974;
            }
            0x800d5974 => {
                pc = 0x800d59a8; continue;
                pc = 0x800d5978;
            }
            0x800d5978 => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[5] = 0x800d0000;
                c.r[5] = c.r[5].wrapping_add(0x72e4);
                c.r[7] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[6].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x500), c.r[5]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[7]);
                c.sth(c.r[3].wrapping_add(0x522), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[7]);
                c.sth(c.r[3].wrapping_add(0x524), c.r[8]);
                pc = 0x800d59a8;
            }
            0x800d59a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d592c: bad pc {:#x}", pc),
        }
    }
}

/// gfxVtxTexCoord (0x800d59b8, 128 bytes)
#[inline(never)]
pub fn f_800d59b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d59b8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d59b8); f_800d59b8_b(c, 0x800d59b8); crate::prof::exit(c, 0x800d59b8); return; }
    }
    f_800d59b8_b(c, 0x800d59b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d59b8_at(c: &mut Cpu, pc: u32) { f_800d59b8_b(c, pc) }
#[inline(always)]
fn f_800d59b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d59b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d59b8, pc); }
        match pc {
            0x800d59b8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d59f8; continue; }
                pc = 0x800d59d4;
            }
            0x800d59d4 => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d59f8; continue; }
                pc = 0x800d59e0;
            }
            0x800d59e0 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = 0x1c;
                c.r[4] = 0xf;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d59f4; f_800d4f98(c);
                pc = 0x800d59f4;
            }
            0x800d59f4 => {
                pc = 0x800d5a28; continue;
                pc = 0x800d59f8;
            }
            0x800d59f8 => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[4] = 0x800d0000;
                c.r[4] = c.r[4].wrapping_add(0x7304);
                c.r[6] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x500), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                { let ea = c.r[3].wrapping_add(0x528); let v = c.f[1]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[6]);
                { let ea = c.r[3].wrapping_add(0x52c); let v = c.f[2]; c.stfs(ea, v); }
                pc = 0x800d5a28;
            }
            0x800d5a28 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d59b8: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5A38 (0x800d5a38, 120 bytes)
#[inline(never)]
pub fn f_800d5a38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5a38) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5a38); f_800d5a38_b(c, 0x800d5a38); crate::prof::exit(c, 0x800d5a38); return; }
    }
    f_800d5a38_b(c, 0x800d5a38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5a38_at(c: &mut Cpu, pc: u32) { f_800d5a38_b(c, pc) }
#[inline(always)]
fn f_800d5a38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5a38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5a38, pc); }
        match pc {
            0x800d5a38 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5a7c; continue; }
                pc = 0x800d5a54;
            }
            0x800d5a54 => {
                c.r[0] = c.lwz(c.r[7]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5a7c; continue; }
                pc = 0x800d5a60;
            }
            0x800d5a60 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4].rotate_left(0) & 0xff;
                c.r[3] = 0x1b;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5a78; f_800d4f98(c);
                pc = 0x800d5a78;
            }
            0x800d5a78 => {
                pc = 0x800d5aa0; continue;
                pc = 0x800d5a7c;
            }
            0x800d5a7c => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[5] = 0x800d0000;
                c.r[6] = c.r[5].wrapping_add(0x7328);
                c.r[5] = c.r[7].wrapping_add(c.r[0]);
                c.stw(c.r[5].wrapping_add(0x4e0), c.r[6]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x4e8), c.r[4]);
                pc = 0x800d5aa0;
            }
            0x800d5aa0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5a38: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5AB0 (0x800d5ab0, 120 bytes)
#[inline(never)]
pub fn f_800d5ab0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5ab0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5ab0); f_800d5ab0_b(c, 0x800d5ab0); crate::prof::exit(c, 0x800d5ab0); return; }
    }
    f_800d5ab0_b(c, 0x800d5ab0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5ab0_at(c: &mut Cpu, pc: u32) { f_800d5ab0_b(c, pc) }
#[inline(always)]
fn f_800d5ab0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5ab0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5ab0, pc); }
        match pc {
            0x800d5ab0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5af4; continue; }
                pc = 0x800d5acc;
            }
            0x800d5acc => {
                c.r[0] = c.lwz(c.r[7]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5af4; continue; }
                pc = 0x800d5ad8;
            }
            0x800d5ad8 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4].rotate_left(0) & 0xffff;
                c.r[3] = 0x1a;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5af0; f_800d4f98(c);
                pc = 0x800d5af0;
            }
            0x800d5af0 => {
                pc = 0x800d5b18; continue;
                pc = 0x800d5af4;
            }
            0x800d5af4 => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[5] = 0x800d0000;
                c.r[6] = c.r[5].wrapping_add(0x7344);
                c.r[5] = c.r[7].wrapping_add(c.r[0]);
                c.stw(c.r[5].wrapping_add(0x4e0), c.r[6]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.sth(c.r[3].wrapping_add(0x4ec), c.r[4]);
                pc = 0x800d5b18;
            }
            0x800d5b18 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5ab0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5B28 (0x800d5b28, 120 bytes)
#[inline(never)]
pub fn f_800d5b28(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5b28) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5b28); f_800d5b28_b(c, 0x800d5b28); crate::prof::exit(c, 0x800d5b28); return; }
    }
    f_800d5b28_b(c, 0x800d5b28)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5b28_at(c: &mut Cpu, pc: u32) { f_800d5b28_b(c, pc) }
#[inline(always)]
fn f_800d5b28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5b28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5b28, pc); }
        match pc {
            0x800d5b28 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5b6c; continue; }
                pc = 0x800d5b44;
            }
            0x800d5b44 => {
                c.r[0] = c.lwz(c.r[7]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5b6c; continue; }
                pc = 0x800d5b50;
            }
            0x800d5b50 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4].rotate_left(0) & 0xffff;
                c.r[3] = 0x19;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5b68; f_800d4f98(c);
                pc = 0x800d5b68;
            }
            0x800d5b68 => {
                pc = 0x800d5b90; continue;
                pc = 0x800d5b6c;
            }
            0x800d5b6c => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[5] = 0x800d0000;
                c.r[6] = c.r[5].wrapping_add(0x7360);
                c.r[5] = c.r[7].wrapping_add(c.r[0]);
                c.stw(c.r[5].wrapping_add(0x4e0), c.r[6]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.sth(c.r[3].wrapping_add(0x4ec), c.r[4]);
                pc = 0x800d5b90;
            }
            0x800d5b90 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5b28: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5BA0 (0x800d5ba0, 120 bytes)
#[inline(never)]
pub fn f_800d5ba0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5ba0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5ba0); f_800d5ba0_b(c, 0x800d5ba0); crate::prof::exit(c, 0x800d5ba0); return; }
    }
    f_800d5ba0_b(c, 0x800d5ba0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5ba0_at(c: &mut Cpu, pc: u32) { f_800d5ba0_b(c, pc) }
#[inline(always)]
fn f_800d5ba0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5ba0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5ba0, pc); }
        match pc {
            0x800d5ba0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5be4; continue; }
                pc = 0x800d5bbc;
            }
            0x800d5bbc => {
                c.r[0] = c.lwz(c.r[7]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5be4; continue; }
                pc = 0x800d5bc8;
            }
            0x800d5bc8 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4] | c.r[4];
                c.r[3] = 0x18;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5be0; f_800d4f98(c);
                pc = 0x800d5be0;
            }
            0x800d5be0 => {
                pc = 0x800d5c08; continue;
                pc = 0x800d5be4;
            }
            0x800d5be4 => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[5] = 0x800d0000;
                c.r[6] = c.r[5].wrapping_add(0x737c);
                c.r[5] = c.r[7].wrapping_add(c.r[0]);
                c.stw(c.r[5].wrapping_add(0x4e0), c.r[6]);
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x4f0), c.r[4]);
                pc = 0x800d5c08;
            }
            0x800d5c08 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5ba0: bad pc {:#x}", pc),
        }
    }
}

/// gfxVtxColor (0x800d5c18, 160 bytes)
#[inline(never)]
pub fn f_800d5c18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5c18) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5c18); f_800d5c18_b(c, 0x800d5c18); crate::prof::exit(c, 0x800d5c18); return; }
    }
    f_800d5c18_b(c, 0x800d5c18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5c18_at(c: &mut Cpu, pc: u32) { f_800d5c18_b(c, pc) }
#[inline(always)]
fn f_800d5c18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5c18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5c18, pc); }
        match pc {
            0x800d5c18 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[9] = c.r[5] | c.r[5];
                c.r[8] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5c6c; continue; }
                pc = 0x800d5c3c;
            }
            0x800d5c3c => {
                c.r[0] = c.lwz(c.r[7]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5c6c; continue; }
                pc = 0x800d5c48;
            }
            0x800d5c48 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4].rotate_left(0) & 0xff;
                c.r[7] = c.r[9].rotate_left(0) & 0xff;
                c.r[8] = c.r[8].rotate_left(0) & 0xff;
                c.r[3] = 0x17;
                c.r[4] = 0x4;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5c68; f_800d4f98(c);
                pc = 0x800d5c68;
            }
            0x800d5c68 => {
                pc = 0x800d5ca8; continue;
                pc = 0x800d5c6c;
            }
            0x800d5c6c => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[5] = 0x800d0000;
                c.r[6] = c.r[5].wrapping_add(0x7398);
                c.r[5] = c.r[7].wrapping_add(c.r[0]);
                c.stw(c.r[5].wrapping_add(0x4e0), c.r[6]);
                c.r[5] = c.r[3].wrapping_mul(0xc);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stb(c.r[3].wrapping_add(0x4e8), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stb(c.r[3].wrapping_add(0x4e9), c.r[9]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stb(c.r[3].wrapping_add(0x4ea), c.r[8]);
                pc = 0x800d5ca8;
            }
            0x800d5ca8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5c18: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5CB8 (0x800d5cb8, 180 bytes)
#[inline(never)]
pub fn f_800d5cb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5cb8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5cb8); f_800d5cb8_b(c, 0x800d5cb8); crate::prof::exit(c, 0x800d5cb8); return; }
    }
    f_800d5cb8_b(c, 0x800d5cb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5cb8_at(c: &mut Cpu, pc: u32) { f_800d5cb8_b(c, pc) }
#[inline(always)]
fn f_800d5cb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5cb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5cb8, pc); }
        match pc {
            0x800d5cb8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[11] = c.r[5] | c.r[5];
                c.r[10] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[9] = c.r[7] | c.r[7];
                c.r[8] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[8].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5d14; continue; }
                pc = 0x800d5ce0;
            }
            0x800d5ce0 => {
                c.r[0] = c.lwz(c.r[8]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5d14; continue; }
                pc = 0x800d5cec;
            }
            0x800d5cec => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4].rotate_left(0) & 0xff;
                c.r[7] = c.r[11].rotate_left(0) & 0xff;
                c.r[8] = c.r[10].rotate_left(0) & 0xff;
                c.r[9] = c.r[9].rotate_left(0) & 0xff;
                c.r[3] = 0x16;
                c.r[4] = 0x5;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5d10; f_800d4f98(c);
                pc = 0x800d5d10;
            }
            0x800d5d10 => {
                pc = 0x800d5d5c; continue;
                pc = 0x800d5d14;
            }
            0x800d5d14 => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[5] = 0x800d0000;
                c.r[6] = c.r[5].wrapping_add(0x73c4);
                c.r[5] = c.r[8].wrapping_add(c.r[0]);
                c.stw(c.r[5].wrapping_add(0x4e0), c.r[6]);
                c.r[5] = c.r[3].wrapping_mul(0xc);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stb(c.r[3].wrapping_add(0x4e8), c.r[4]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stb(c.r[3].wrapping_add(0x4e9), c.r[11]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stb(c.r[3].wrapping_add(0x4ea), c.r[10]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.stb(c.r[3].wrapping_add(0x4eb), c.r[9]);
                pc = 0x800d5d5c;
            }
            0x800d5d5c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5cb8: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5D6C (0x800d5d6c, 100 bytes)
#[inline(never)]
pub fn f_800d5d6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5d6c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5d6c); f_800d5d6c_b(c, 0x800d5d6c); crate::prof::exit(c, 0x800d5d6c); return; }
    }
    f_800d5d6c_b(c, 0x800d5d6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5d6c_at(c: &mut Cpu, pc: u32) { f_800d5d6c_b(c, pc) }
#[inline(always)]
fn f_800d5d6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5d6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5d6c, pc); }
        match pc {
            0x800d5d6c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5dac; continue; }
                pc = 0x800d5d88;
            }
            0x800d5d88 => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5dac; continue; }
                pc = 0x800d5d94;
            }
            0x800d5d94 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x15;
                c.r[4] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5da8; f_800d4f98(c);
                pc = 0x800d5da8;
            }
            0x800d5da8 => {
                pc = 0x800d5dc0; continue;
                pc = 0x800d5dac;
            }
            0x800d5dac => {
                c.r[4] = 0x800d0000;
                c.r[0] = c.r[4].wrapping_add(0x73f8);
                c.stw(c.r[5].wrapping_add(0x4c4), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x4c8), c.r[3]);
                pc = 0x800d5dc0;
            }
            0x800d5dc0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5d6c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5DD0 (0x800d5dd0, 100 bytes)
#[inline(never)]
pub fn f_800d5dd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5dd0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5dd0); f_800d5dd0_b(c, 0x800d5dd0); crate::prof::exit(c, 0x800d5dd0); return; }
    }
    f_800d5dd0_b(c, 0x800d5dd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5dd0_at(c: &mut Cpu, pc: u32) { f_800d5dd0_b(c, pc) }
#[inline(always)]
fn f_800d5dd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5dd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5dd0, pc); }
        match pc {
            0x800d5dd0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5e10; continue; }
                pc = 0x800d5dec;
            }
            0x800d5dec => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5e10; continue; }
                pc = 0x800d5df8;
            }
            0x800d5df8 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = 0x14;
                c.r[4] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5e0c; f_800d4f98(c);
                pc = 0x800d5e0c;
            }
            0x800d5e0c => {
                pc = 0x800d5e24; continue;
                pc = 0x800d5e10;
            }
            0x800d5e10 => {
                c.r[4] = 0x800d0000;
                c.r[0] = c.r[4].wrapping_add(0x740c);
                c.stw(c.r[5].wrapping_add(0x4c4), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x4cc), c.r[3]);
                pc = 0x800d5e24;
            }
            0x800d5e24 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5dd0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5E34 (0x800d5e34, 128 bytes)
#[inline(never)]
pub fn f_800d5e34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5e34) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5e34); f_800d5e34_b(c, 0x800d5e34); crate::prof::exit(c, 0x800d5e34); return; }
    }
    f_800d5e34_b(c, 0x800d5e34)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5e34_at(c: &mut Cpu, pc: u32) { f_800d5e34_b(c, pc) }
#[inline(always)]
fn f_800d5e34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5e34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5e34, pc); }
        match pc {
            0x800d5e34 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5e80; continue; }
                pc = 0x800d5e54;
            }
            0x800d5e54 => {
                c.r[0] = c.lwz(c.r[6]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5e80; continue; }
                pc = 0x800d5e60;
            }
            0x800d5e60 => {
                c.r[5] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[6] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[7] = c.r[7] as u8 as i8 as i32 as u32;
                c.r[3] = 0x13;
                c.r[4] = 0x3;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5e7c; f_800d4f98(c);
                pc = 0x800d5e7c;
            }
            0x800d5e7c => {
                pc = 0x800d5ea4; continue;
                pc = 0x800d5e80;
            }
            0x800d5e80 => {
                c.r[5] = 0x800d0000;
                c.r[0] = c.r[5].wrapping_add(0x7420);
                c.stw(c.r[6].wrapping_add(0x4c4), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[5].wrapping_add(0x4c8), c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4c9), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ca), c.r[7]);
                pc = 0x800d5ea4;
            }
            0x800d5ea4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5e34: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5EB4 (0x800d5eb4, 128 bytes)
#[inline(never)]
pub fn f_800d5eb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5eb4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5eb4); f_800d5eb4_b(c, 0x800d5eb4); crate::prof::exit(c, 0x800d5eb4); return; }
    }
    f_800d5eb4_b(c, 0x800d5eb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5eb4_at(c: &mut Cpu, pc: u32) { f_800d5eb4_b(c, pc) }
#[inline(always)]
fn f_800d5eb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5eb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5eb4, pc); }
        match pc {
            0x800d5eb4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[7] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5f00; continue; }
                pc = 0x800d5ed4;
            }
            0x800d5ed4 => {
                c.r[0] = c.lwz(c.r[6]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5f00; continue; }
                pc = 0x800d5ee0;
            }
            0x800d5ee0 => {
                c.r[5] = c.r[3] as u16 as i16 as i32 as u32;
                c.r[6] = c.r[4] as u16 as i16 as i32 as u32;
                c.r[7] = c.r[7] as u16 as i16 as i32 as u32;
                c.r[3] = 0x12;
                c.r[4] = 0x3;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5efc; f_800d4f98(c);
                pc = 0x800d5efc;
            }
            0x800d5efc => {
                pc = 0x800d5f24; continue;
                pc = 0x800d5f00;
            }
            0x800d5f00 => {
                c.r[5] = 0x800d0000;
                c.r[0] = c.r[5].wrapping_add(0x7444);
                c.stw(c.r[6].wrapping_add(0x4c4), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[5].wrapping_add(0x4cc), c.r[3]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4ce), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4d0), c.r[7]);
                pc = 0x800d5f24;
            }
            0x800d5f24 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5eb4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5F34 (0x800d5f34, 112 bytes)
#[inline(never)]
pub fn f_800d5f34(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5f34) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5f34); f_800d5f34_b(c, 0x800d5f34); crate::prof::exit(c, 0x800d5f34); return; }
    }
    f_800d5f34_b(c, 0x800d5f34)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5f34_at(c: &mut Cpu, pc: u32) { f_800d5f34_b(c, pc) }
#[inline(always)]
fn f_800d5f34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5f34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5f34, pc); }
        match pc {
            0x800d5f34 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5f70; continue; }
                pc = 0x800d5f50;
            }
            0x800d5f50 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5f70; continue; }
                pc = 0x800d5f5c;
            }
            0x800d5f5c => {
                c.r[3] = 0x11;
                c.r[4] = 0xd;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d5f6c; f_800d4f98(c);
                pc = 0x800d5f6c;
            }
            0x800d5f6c => {
                pc = 0x800d5f94; continue;
                pc = 0x800d5f70;
            }
            0x800d5f70 => {
                c.r[3] = 0x800d0000;
                c.r[0] = c.r[3].wrapping_add(0x7468);
                c.stw(c.r[4].wrapping_add(0x4c4), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x4d4); let v = c.f[1]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x4d8); let v = c.f[2]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x4dc); let v = c.f[3]; c.stfs(ea, v); }
                pc = 0x800d5f94;
            }
            0x800d5f94 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5f34: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D5FA4 (0x800d5fa4, 132 bytes)
#[inline(never)]
pub fn f_800d5fa4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d5fa4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d5fa4); f_800d5fa4_b(c, 0x800d5fa4); crate::prof::exit(c, 0x800d5fa4); return; }
    }
    f_800d5fa4_b(c, 0x800d5fa4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d5fa4_at(c: &mut Cpu, pc: u32) { f_800d5fa4_b(c, pc) }
#[inline(always)]
fn f_800d5fa4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d5fa4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d5fa4, pc); }
        match pc {
            0x800d5fa4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d5fec; continue; }
                pc = 0x800d5fc8;
            }
            0x800d5fc8 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d5fec; continue; }
                pc = 0x800d5fd4;
            }
            0x800d5fd4 => {
                c.r[5] = c.r[31].rotate_left(0) & 0xff;
                c.r[3] = 0x10;
                c.r[4] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d5fe8; f_800d4f98(c);
                pc = 0x800d5fe8;
            }
            0x800d5fe8 => {
                pc = 0x800d6014; continue;
                pc = 0x800d5fec;
            }
            0x800d5fec => {
                c.lr = 0x800d5ff0; f_800d6b00(c);
                pc = 0x800d5ff0;
            }
            0x800d5ff0 => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x748c);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ac), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d6014;
            }
            0x800d6014 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d5fa4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D6028 (0x800d6028, 132 bytes)
#[inline(never)]
pub fn f_800d6028(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6028) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6028); f_800d6028_b(c, 0x800d6028); crate::prof::exit(c, 0x800d6028); return; }
    }
    f_800d6028_b(c, 0x800d6028)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6028_at(c: &mut Cpu, pc: u32) { f_800d6028_b(c, pc) }
#[inline(always)]
fn f_800d6028_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6028;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6028, pc); }
        match pc {
            0x800d6028 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d6070; continue; }
                pc = 0x800d604c;
            }
            0x800d604c => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d6070; continue; }
                pc = 0x800d6058;
            }
            0x800d6058 => {
                c.r[5] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = 0xf;
                c.r[4] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d606c; f_800d4f98(c);
                pc = 0x800d606c;
            }
            0x800d606c => {
                pc = 0x800d6098; continue;
                pc = 0x800d6070;
            }
            0x800d6070 => {
                c.lr = 0x800d6074; f_800d6b00(c);
                pc = 0x800d6074;
            }
            0x800d6074 => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x74a0);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b0), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d6098;
            }
            0x800d6098 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6028: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D60AC (0x800d60ac, 156 bytes)
#[inline(never)]
pub fn f_800d60ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d60ac) { return; }
        if c.prof { crate::prof::enter(c, 0x800d60ac); f_800d60ac_b(c, 0x800d60ac); crate::prof::exit(c, 0x800d60ac); return; }
    }
    f_800d60ac_b(c, 0x800d60ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d60ac_at(c: &mut Cpu, pc: u32) { f_800d60ac_b(c, pc) }
#[inline(always)]
fn f_800d60ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d60ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d60ac, pc); }
        match pc {
            0x800d60ac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d6100; continue; }
                pc = 0x800d60d8;
            }
            0x800d60d8 => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d6100; continue; }
                pc = 0x800d60e4;
            }
            0x800d60e4 => {
                c.r[5] = c.r[30] as u8 as i8 as i32 as u32;
                c.r[6] = c.r[31] as u8 as i8 as i32 as u32;
                c.r[3] = 0xe;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d60fc; f_800d4f98(c);
                pc = 0x800d60fc;
            }
            0x800d60fc => {
                pc = 0x800d6130; continue;
                pc = 0x800d6100;
            }
            0x800d6100 => {
                c.lr = 0x800d6104; f_800d6b00(c);
                pc = 0x800d6104;
            }
            0x800d6104 => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x74b4);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ac), c.r[30]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ad), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d6130;
            }
            0x800d6130 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d60ac: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D6148 (0x800d6148, 156 bytes)
#[inline(never)]
pub fn f_800d6148(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6148) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6148); f_800d6148_b(c, 0x800d6148); crate::prof::exit(c, 0x800d6148); return; }
    }
    f_800d6148_b(c, 0x800d6148)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6148_at(c: &mut Cpu, pc: u32) { f_800d6148_b(c, pc) }
#[inline(always)]
fn f_800d6148_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6148;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6148, pc); }
        match pc {
            0x800d6148 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d619c; continue; }
                pc = 0x800d6174;
            }
            0x800d6174 => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d619c; continue; }
                pc = 0x800d6180;
            }
            0x800d6180 => {
                c.r[5] = c.r[30].rotate_left(0) & 0xff;
                c.r[6] = c.r[31].rotate_left(0) & 0xff;
                c.r[3] = 0xd;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d6198; f_800d4f98(c);
                pc = 0x800d6198;
            }
            0x800d6198 => {
                pc = 0x800d61cc; continue;
                pc = 0x800d619c;
            }
            0x800d619c => {
                c.lr = 0x800d61a0; f_800d6b00(c);
                pc = 0x800d61a0;
            }
            0x800d61a0 => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x74d0);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ac), c.r[30]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ad), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d61cc;
            }
            0x800d61cc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6148: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D61E4 (0x800d61e4, 156 bytes)
#[inline(never)]
pub fn f_800d61e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d61e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d61e4); f_800d61e4_b(c, 0x800d61e4); crate::prof::exit(c, 0x800d61e4); return; }
    }
    f_800d61e4_b(c, 0x800d61e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d61e4_at(c: &mut Cpu, pc: u32) { f_800d61e4_b(c, pc) }
#[inline(always)]
fn f_800d61e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d61e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d61e4, pc); }
        match pc {
            0x800d61e4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d6238; continue; }
                pc = 0x800d6210;
            }
            0x800d6210 => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d6238; continue; }
                pc = 0x800d621c;
            }
            0x800d621c => {
                c.r[5] = c.r[30] as u16 as i16 as i32 as u32;
                c.r[6] = c.r[31] as u16 as i16 as i32 as u32;
                c.r[3] = 0xc;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d6234; f_800d4f98(c);
                pc = 0x800d6234;
            }
            0x800d6234 => {
                pc = 0x800d6268; continue;
                pc = 0x800d6238;
            }
            0x800d6238 => {
                c.lr = 0x800d623c; f_800d6b00(c);
                pc = 0x800d623c;
            }
            0x800d623c => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x74ec);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b0), c.r[30]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b2), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d6268;
            }
            0x800d6268 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d61e4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D6280 (0x800d6280, 156 bytes)
#[inline(never)]
pub fn f_800d6280(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6280) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6280); f_800d6280_b(c, 0x800d6280); crate::prof::exit(c, 0x800d6280); return; }
    }
    f_800d6280_b(c, 0x800d6280)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6280_at(c: &mut Cpu, pc: u32) { f_800d6280_b(c, pc) }
#[inline(always)]
fn f_800d6280_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6280;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6280, pc); }
        match pc {
            0x800d6280 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d62d4; continue; }
                pc = 0x800d62ac;
            }
            0x800d62ac => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d62d4; continue; }
                pc = 0x800d62b8;
            }
            0x800d62b8 => {
                c.r[5] = c.r[30].rotate_left(0) & 0xffff;
                c.r[6] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = 0xb;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d62d0; f_800d4f98(c);
                pc = 0x800d62d0;
            }
            0x800d62d0 => {
                pc = 0x800d6304; continue;
                pc = 0x800d62d4;
            }
            0x800d62d4 => {
                c.lr = 0x800d62d8; f_800d6b00(c);
                pc = 0x800d62d8;
            }
            0x800d62d8 => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x7508);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b0), c.r[30]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b2), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d6304;
            }
            0x800d6304 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6280: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D631C (0x800d631c, 148 bytes)
#[inline(never)]
pub fn f_800d631c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d631c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d631c); f_800d631c_b(c, 0x800d631c); crate::prof::exit(c, 0x800d631c); return; }
    }
    f_800d631c_b(c, 0x800d631c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d631c_at(c: &mut Cpu, pc: u32) { f_800d631c_b(c, pc) }
#[inline(always)]
fn f_800d631c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d631c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d631c, pc); }
        match pc {
            0x800d631c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[31]; c.stfd(ea, v); }
                c.f[31] = c.f[2];
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[30]; c.stfd(ea, v); }
                c.f[30] = c.f[1];
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d6368; continue; }
                pc = 0x800d6348;
            }
            0x800d6348 => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d6368; continue; }
                pc = 0x800d6354;
            }
            0x800d6354 => {
                c.r[3] = 0xa;
                c.r[4] = 0xc;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d6364; f_800d4f98(c);
                pc = 0x800d6364;
            }
            0x800d6364 => {
                pc = 0x800d6398; continue;
                pc = 0x800d6368;
            }
            0x800d6368 => {
                c.lr = 0x800d636c; f_800d6b00(c);
                pc = 0x800d636c;
            }
            0x800d636c => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x7524);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x4b8); let v = c.f[30]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x4bc); let v = c.f[31]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d6398;
            }
            0x800d6398 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                { let ea = c.r[1].wrapping_add(0x18); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x10); c.f[30] = c.lfd(ea); }
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d631c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D63B0 (0x800d63b0, 180 bytes)
#[inline(never)]
pub fn f_800d63b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d63b0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d63b0); f_800d63b0_b(c, 0x800d63b0); crate::prof::exit(c, 0x800d63b0); return; }
    }
    f_800d63b0_b(c, 0x800d63b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d63b0_at(c: &mut Cpu, pc: u32) { f_800d63b0_b(c, pc) }
#[inline(always)]
fn f_800d63b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d63b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d63b0, pc); }
        match pc {
            0x800d63b0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d6410; continue; }
                pc = 0x800d63e4;
            }
            0x800d63e4 => {
                c.r[0] = c.lwz(c.r[6]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d6410; continue; }
                pc = 0x800d63f0;
            }
            0x800d63f0 => {
                c.r[5] = c.r[29] as u8 as i8 as i32 as u32;
                c.r[6] = c.r[30] as u8 as i8 as i32 as u32;
                c.r[7] = c.r[31] as u8 as i8 as i32 as u32;
                c.r[3] = 0x9;
                c.r[4] = 0x3;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d640c; f_800d4f98(c);
                pc = 0x800d640c;
            }
            0x800d640c => {
                pc = 0x800d6448; continue;
                pc = 0x800d6410;
            }
            0x800d6410 => {
                c.lr = 0x800d6414; f_800d6b00(c);
                pc = 0x800d6414;
            }
            0x800d6414 => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x7540);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ac), c.r[29]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ad), c.r[30]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ae), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d6448;
            }
            0x800d6448 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d63b0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D6464 (0x800d6464, 180 bytes)
#[inline(never)]
pub fn f_800d6464(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6464) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6464); f_800d6464_b(c, 0x800d6464); crate::prof::exit(c, 0x800d6464); return; }
    }
    f_800d6464_b(c, 0x800d6464)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6464_at(c: &mut Cpu, pc: u32) { f_800d6464_b(c, pc) }
#[inline(always)]
fn f_800d6464_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6464;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6464, pc); }
        match pc {
            0x800d6464 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d64c4; continue; }
                pc = 0x800d6498;
            }
            0x800d6498 => {
                c.r[0] = c.lwz(c.r[6]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d64c4; continue; }
                pc = 0x800d64a4;
            }
            0x800d64a4 => {
                c.r[5] = c.r[29].rotate_left(0) & 0xff;
                c.r[6] = c.r[30].rotate_left(0) & 0xff;
                c.r[7] = c.r[31].rotate_left(0) & 0xff;
                c.r[3] = 0x8;
                c.r[4] = 0x3;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d64c0; f_800d4f98(c);
                pc = 0x800d64c0;
            }
            0x800d64c0 => {
                pc = 0x800d64fc; continue;
                pc = 0x800d64c4;
            }
            0x800d64c4 => {
                c.lr = 0x800d64c8; f_800d6b00(c);
                pc = 0x800d64c8;
            }
            0x800d64c8 => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x7564);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ac), c.r[29]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ad), c.r[30]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x4ae), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d64fc;
            }
            0x800d64fc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6464: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D6518 (0x800d6518, 180 bytes)
#[inline(never)]
pub fn f_800d6518(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6518) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6518); f_800d6518_b(c, 0x800d6518); crate::prof::exit(c, 0x800d6518); return; }
    }
    f_800d6518_b(c, 0x800d6518)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6518_at(c: &mut Cpu, pc: u32) { f_800d6518_b(c, pc) }
#[inline(always)]
fn f_800d6518_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6518;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6518, pc); }
        match pc {
            0x800d6518 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d6578; continue; }
                pc = 0x800d654c;
            }
            0x800d654c => {
                c.r[0] = c.lwz(c.r[6]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d6578; continue; }
                pc = 0x800d6558;
            }
            0x800d6558 => {
                c.r[5] = c.r[29] as u16 as i16 as i32 as u32;
                c.r[6] = c.r[30] as u16 as i16 as i32 as u32;
                c.r[7] = c.r[31] as u16 as i16 as i32 as u32;
                c.r[3] = 0x7;
                c.r[4] = 0x3;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d6574; f_800d4f98(c);
                pc = 0x800d6574;
            }
            0x800d6574 => {
                pc = 0x800d65b0; continue;
                pc = 0x800d6578;
            }
            0x800d6578 => {
                c.lr = 0x800d657c; f_800d6b00(c);
                pc = 0x800d657c;
            }
            0x800d657c => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x7588);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b0), c.r[29]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b2), c.r[30]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b4), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d65b0;
            }
            0x800d65b0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6518: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D65CC (0x800d65cc, 180 bytes)
#[inline(never)]
pub fn f_800d65cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d65cc) { return; }
        if c.prof { crate::prof::enter(c, 0x800d65cc); f_800d65cc_b(c, 0x800d65cc); crate::prof::exit(c, 0x800d65cc); return; }
    }
    f_800d65cc_b(c, 0x800d65cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d65cc_at(c: &mut Cpu, pc: u32) { f_800d65cc_b(c, pc) }
#[inline(always)]
fn f_800d65cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d65cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d65cc, pc); }
        match pc {
            0x800d65cc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d662c; continue; }
                pc = 0x800d6600;
            }
            0x800d6600 => {
                c.r[0] = c.lwz(c.r[6]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d662c; continue; }
                pc = 0x800d660c;
            }
            0x800d660c => {
                c.r[5] = c.r[29].rotate_left(0) & 0xffff;
                c.r[6] = c.r[30].rotate_left(0) & 0xffff;
                c.r[7] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = 0x6;
                c.r[4] = 0x3;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d6628; f_800d4f98(c);
                pc = 0x800d6628;
            }
            0x800d6628 => {
                pc = 0x800d6664; continue;
                pc = 0x800d662c;
            }
            0x800d662c => {
                c.lr = 0x800d6630; f_800d6b00(c);
                pc = 0x800d6630;
            }
            0x800d6630 => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x75ac);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b0), c.r[29]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b2), c.r[30]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b4), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d6664;
            }
            0x800d6664 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d65cc: bad pc {:#x}", pc),
        }
    }
}

/// gfxVtxPos (0x800d6680, 168 bytes)
#[inline(never)]
pub fn f_800d6680(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6680) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6680); f_800d6680_b(c, 0x800d6680); crate::prof::exit(c, 0x800d6680); return; }
    }
    f_800d6680_b(c, 0x800d6680)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6680_at(c: &mut Cpu, pc: u32) { f_800d6680_b(c, pc) }
#[inline(always)]
fn f_800d6680_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6680;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6680, pc); }
        match pc {
            0x800d6680 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[31]; c.stfd(ea, v); }
                c.f[31] = c.f[3];
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[30]; c.stfd(ea, v); }
                c.f[30] = c.f[2];
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[29]; c.stfd(ea, v); }
                c.f[29] = c.f[1];
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d66d4; continue; }
                pc = 0x800d66b4;
            }
            0x800d66b4 => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d66d4; continue; }
                pc = 0x800d66c0;
            }
            0x800d66c0 => {
                c.r[3] = 0x5;
                c.r[4] = 0xd;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d66d0; f_800d4f98(c);
                pc = 0x800d66d0;
            }
            0x800d66d0 => {
                pc = 0x800d670c; continue;
                pc = 0x800d66d4;
            }
            0x800d66d4 => {
                c.lr = 0x800d66d8; f_800d6b00(c);
                pc = 0x800d66d8;
            }
            0x800d66d8 => {
                c.r[4] = 0x800d0000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[4].wrapping_add(0x75d0);
                c.r[0] = 0x1;
                c.stw(c.r[3].wrapping_add(0x4a8), c.r[4]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x4b8); let v = c.f[29]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x4bc); let v = c.f[30]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x4c0); let v = c.f[31]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d670c;
            }
            0x800d670c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                { let ea = c.r[1].wrapping_add(0x18); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x10); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x8); c.f[29] = c.lfd(ea); }
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6680: bad pc {:#x}", pc),
        }
    }
}

/// gfxEnd (0x800d6728, 148 bytes)
#[inline(never)]
pub fn f_800d6728(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6728) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6728); f_800d6728_b(c, 0x800d6728); crate::prof::exit(c, 0x800d6728); return; }
    }
    f_800d6728_b(c, 0x800d6728)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6728_at(c: &mut Cpu, pc: u32) { f_800d6728_b(c, pc) }
#[inline(always)]
fn f_800d6728_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6728;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6728, pc); }
        match pc {
            0x800d6728 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d674c; continue; }
                pc = 0x800d6744;
            }
            0x800d6744 => {
                c.lr = 0x800d6748; f_800d6b00(c);
                pc = 0x800d6748;
            }
            0x800d6748 => {
                pc = 0x800d67ac; continue;
                pc = 0x800d674c;
            }
            0x800d674c => {
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d676c; continue; }
                pc = 0x800d6758;
            }
            0x800d6758 => {
                c.r[3] = 0x3;
                c.r[4] = 0x0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d6768; f_800d4f98(c);
                pc = 0x800d6768;
            }
            0x800d6768 => {
                pc = 0x800d67ac; continue;
                pc = 0x800d676c;
            }
            0x800d676c => {
                c.lr = 0x800d6770; f_800d6b00(c);
                pc = 0x800d6770;
            }
            0x800d6770 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1b));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x1a));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800d67ac; continue; }
                pc = 0x800d6784;
            }
            0x800d6784 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3] & c.r[0];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d67ac; continue; }
                pc = 0x800d6794;
            }
            0x800d6794 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x20));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800d67ac; continue; }
                pc = 0x800d67a4;
            }
            0x800d67a4 => {
                c.r[0] = 0x0;
                c.stw(c.r[4].wrapping_add(0x24), c.r[0]);
                pc = 0x800d67ac;
            }
            0x800d67ac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6728: bad pc {:#x}", pc),
        }
    }
}

/// gfxBegin (0x800d67bc, 580 bytes)
#[inline(never)]
pub fn f_800d67bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d67bc) { return; }
        if c.prof { crate::prof::enter(c, 0x800d67bc); f_800d67bc_b(c, 0x800d67bc); crate::prof::exit(c, 0x800d67bc); return; }
    }
    f_800d67bc_b(c, 0x800d67bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d67bc_at(c: &mut Cpu, pc: u32) { f_800d67bc_b(c, pc) }
#[inline(always)]
fn f_800d67bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d67bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d67bc, pc); }
        match pc {
            0x800d67bc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d67e8; continue; }
                pc = 0x800d67e0;
            }
            0x800d67e0 => {
                c.lr = 0x800d67e4; f_800db758(c);
                pc = 0x800d67e4;
            }
            0x800d67e4 => {
                pc = 0x800d69ec; continue;
                pc = 0x800d67e8;
            }
            0x800d67e8 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d680c; continue; }
                pc = 0x800d67f4;
            }
            0x800d67f4 => {
                c.r[5] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = 0x2;
                c.r[4] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d6808; f_800d4f98(c);
                pc = 0x800d6808;
            }
            0x800d6808 => {
                pc = 0x800d69ec; continue;
                pc = 0x800d680c;
            }
            0x800d680c => {
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1b));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x1a));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800d69ec; continue; }
                pc = 0x800d681c;
            }
            0x800d681c => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3] & c.r[0];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d69ec; continue; }
                pc = 0x800d682c;
            }
            0x800d682c => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x24));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d6910; continue; }
                pc = 0x800d6838;
            }
            0x800d6838 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x20));
                c.lr = 0x800d6840; f_800d7650(c);
                pc = 0x800d6840;
            }
            0x800d6840 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.r[6] = 0x1;
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x20));
                c.r[7] = 0x4;
                c.r[8] = 0x0;
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                c.lr = 0x800d6868; f_800d7868(c);
                pc = 0x800d6868;
            }
            0x800d6868 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0x4;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d689c; continue; }
                pc = 0x800d6878;
            }
            0x800d6878 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x20));
                c.r[4] = 0x2;
                c.r[5] = 0x0;
                c.r[6] = 0x2;
                c.r[7] = 0x4;
                c.r[8] = 0x0;
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                c.lr = 0x800d689c; f_800d7868(c);
                pc = 0x800d689c;
            }
            0x800d689c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d68d0; continue; }
                pc = 0x800d68ac;
            }
            0x800d68ac => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x20));
                c.r[4] = 0x4;
                c.r[5] = 0x0;
                c.r[6] = 0x6;
                c.r[7] = 0xa;
                c.r[8] = 0x0;
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                c.lr = 0x800d68d0; f_800d7868(c);
                pc = 0x800d68d0;
            }
            0x800d68d0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x10));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d6904; continue; }
                pc = 0x800d68e0;
            }
            0x800d68e0 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x20));
                c.r[4] = 0x6;
                c.r[5] = 0x0;
                c.r[6] = 0x8;
                c.r[7] = 0x4;
                c.r[8] = 0x0;
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                c.lr = 0x800d6904; f_800d7868(c);
                pc = 0x800d6904;
            }
            0x800d6904 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x20));
                c.stw(c.r[3].wrapping_add(0x24), c.r[0]);
                pc = 0x800d6910;
            }
            0x800d6910 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x14));
                c.cmps(0, c.r[0], 0x7);
                if !c.crbit(2) { pc = 0x800d6924; continue; }
                pc = 0x800d6920;
            }
            0x800d6920 => {
                c.r[31] = c.r[31].rotate_left(1) & 0xfffe;
                pc = 0x800d6924;
            }
            0x800d6924 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x24));
                c.lr = 0x800d692c; f_800d7a70(c);
                pc = 0x800d692c;
            }
            0x800d692c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x24));
                c.lr = 0x800d6938; f_800d892c(c);
                pc = 0x800d6938;
            }
            0x800d6938 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0x80310000;
                c.r[3] = c.r[3].wrapping_add(0x4350);
                c.r[5] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x14));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x4));
                c.lr = 0x800d6960; f_800b928c(c);
                pc = 0x800d6960;
            }
            0x800d6960 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0x80400000;
                c.r[6] = c.r[3].wrapping_add(0x1f0);
                c.r[5] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x14));
                c.r[0] = c.lwz(c.r[6].wrapping_add(0xc));
                c.cmps(0, c.r[3], 0x6);
                c.r[0] = c.r[0].wrapping_add(c.r[5]);
                c.stw(c.r[6].wrapping_add(0xc), c.r[0]);
                if !c.crbit(0) { pc = 0x800d6998; continue; }
                pc = 0x800d6988;
            }
            0x800d6988 => {
                c.cmps(0, c.r[3], 0x3);
                if c.crbit(2) { pc = 0x800d69a4; continue; }
                pc = 0x800d6990;
            }
            0x800d6990 => {
                if !c.crbit(0) { pc = 0x800d69c8; continue; }
                pc = 0x800d6994;
            }
            0x800d6994 => {
                pc = 0x800d69ec; continue;
                pc = 0x800d6998;
            }
            0x800d6998 => {
                c.cmps(0, c.r[3], 0x8);
                if !c.crbit(0) { pc = 0x800d69ec; continue; }
                pc = 0x800d69a0;
            }
            0x800d69a0 => {
                pc = 0x800d69dc; continue;
                pc = 0x800d69a4;
            }
            0x800d69a4 => {
                c.r[3] = 0x55550000;
                c.r[4] = c.lwz(c.r[6].wrapping_add(0x4));
                c.r[0] = c.r[3].wrapping_add(0x5556);
                c.r[3] = mulhw(c.r[0], c.r[5]);
                c.r[0] = c.r[3].rotate_left(1) & 0x1;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[6].wrapping_add(0x4), c.r[0]);
                pc = 0x800d69ec; continue;
                pc = 0x800d69c8;
            }
            0x800d69c8 => {
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x4));
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0xfffffffe);
                c.stw(c.r[6].wrapping_add(0x4), c.r[0]);
                pc = 0x800d69ec; continue;
                pc = 0x800d69dc;
            }
            0x800d69dc => {
                c.r[3] = c.lwz(c.r[6].wrapping_add(0x4));
                c.r[0] = c.r[31].rotate_left(31) & 0x7fff;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[6].wrapping_add(0x4), c.r[0]);
                pc = 0x800d69ec;
            }
            0x800d69ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d67bc: bad pc {:#x}", pc),
        }
    }
}

/// gfxSetPrimitive (0x800d6a00, 92 bytes)
#[inline(never)]
pub fn f_800d6a00(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6a00) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6a00); f_800d6a00_b(c, 0x800d6a00); crate::prof::exit(c, 0x800d6a00); return; }
    }
    f_800d6a00_b(c, 0x800d6a00)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6a00_at(c: &mut Cpu, pc: u32) { f_800d6a00_b(c, pc) }
#[inline(always)]
fn f_800d6a00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6a00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6a00, pc); }
        match pc {
            0x800d6a00 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d6a24; continue; }
                pc = 0x800d6a1c;
            }
            0x800d6a1c => {
                c.stw(c.r[4].wrapping_add(0x488), c.r[3]);
                pc = 0x800d6a4c; continue;
                pc = 0x800d6a24;
            }
            0x800d6a24 => {
                c.r[0] = c.lwz(c.r[4]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d6a48; continue; }
                pc = 0x800d6a30;
            }
            0x800d6a30 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d6a44; f_800d4f98(c);
                pc = 0x800d6a44;
            }
            0x800d6a44 => {
                pc = 0x800d6a4c; continue;
                pc = 0x800d6a48;
            }
            0x800d6a48 => {
                c.stw(c.r[4].wrapping_add(0x14), c.r[3]);
                pc = 0x800d6a4c;
            }
            0x800d6a4c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6a00: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D6A5C (0x800d6a5c, 36 bytes)
#[inline(never)]
pub fn f_800d6a5c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6a5c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6a5c); f_800d6a5c_b(c, 0x800d6a5c); crate::prof::exit(c, 0x800d6a5c); return; }
    }
    f_800d6a5c_b(c, 0x800d6a5c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6a5c_at(c: &mut Cpu, pc: u32) { f_800d6a5c_b(c, pc) }
#[inline(always)]
fn f_800d6a5c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6a5c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6a5c, pc); }
        match pc {
            0x800d6a5c => {
                c.r[5] = 0x80400000;
                c.r[6] = c.r[5].wrapping_add(0x1f0);
                c.r[5] = c.lwz(c.r[6].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x4));
                c.r[3] = c.r[5].wrapping_add(c.r[3]);
                c.r[0] = c.r[0].wrapping_add(c.r[4]);
                c.stw(c.r[6].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[6].wrapping_add(0x4), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6a5c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D6A80 (0x800d6a80, 128 bytes)
#[inline(never)]
pub fn f_800d6a80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6a80) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6a80); f_800d6a80_b(c, 0x800d6a80); crate::prof::exit(c, 0x800d6a80); return; }
    }
    f_800d6a80_b(c, 0x800d6a80)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6a80_at(c: &mut Cpu, pc: u32) { f_800d6a80_b(c, pc) }
#[inline(always)]
fn f_800d6a80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6a80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6a80, pc); }
        match pc {
            0x800d6a80 => {
                c.r[0] = c.lwz(c.r[5]);
                c.r[7] = c.r[3].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[4], 0x6);
                c.r[0] = c.r[0].wrapping_add(c.r[7]);
                c.stw(c.r[5], c.r[0]);
                if !c.crbit(0) { pc = 0x800d6aa8; continue; }
                pc = 0x800d6a98;
            }
            0x800d6a98 => {
                c.cmps(0, c.r[4], 0x3);
                if c.crbit(2) { pc = 0x800d6ab4; continue; }
                pc = 0x800d6aa0;
            }
            0x800d6aa0 => {
                if !c.crbit(0) { pc = 0x800d6ad8; continue; }
                pc = 0x800d6aa4;
            }
            0x800d6aa4 => {
                if true { return; }
                pc = 0x800d6aa8;
            }
            0x800d6aa8 => {
                c.cmps(0, c.r[4], 0x8);
                if !c.crbit(0) { return; }
                pc = 0x800d6ab0;
            }
            0x800d6ab0 => {
                pc = 0x800d6aec; continue;
                pc = 0x800d6ab4;
            }
            0x800d6ab4 => {
                c.r[3] = 0x55550000;
                c.r[4] = c.lwz(c.r[6]);
                c.r[0] = c.r[3].wrapping_add(0x5556);
                c.r[3] = mulhw(c.r[0], c.r[7]);
                c.r[0] = c.r[3].rotate_left(1) & 0x1;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[6], c.r[0]);
                if true { return; }
                pc = 0x800d6ad8;
            }
            0x800d6ad8 => {
                c.r[0] = c.lwz(c.r[6]);
                c.r[3] = c.r[7].wrapping_add(c.r[0]);
                c.r[0] = c.r[3].wrapping_add(0xfffffffe);
                c.stw(c.r[6], c.r[0]);
                if true { return; }
                pc = 0x800d6aec;
            }
            0x800d6aec => {
                c.r[4] = c.lwz(c.r[6]);
                c.r[0] = c.r[3].rotate_left(31) & 0x7fff;
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stw(c.r[6], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6a80: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D6B00 (0x800d6b00, 1840 bytes)
#[inline(never)]
pub fn f_800d6b00(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d6b00) { return; }
        if c.prof { crate::prof::enter(c, 0x800d6b00); f_800d6b00_b(c, 0x800d6b00); crate::prof::exit(c, 0x800d6b00); return; }
    }
    f_800d6b00_b(c, 0x800d6b00)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d6b00_at(c: &mut Cpu, pc: u32) { f_800d6b00_b(c, pc) }
#[inline(always)]
fn f_800d6b00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d6b00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d6b00, pc); }
        match pc {
            0x800d6b00 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x80400000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3].wrapping_add(0x7e8);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x49f));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d7210; continue; }
                pc = 0x800d6b34;
            }
            0x800d6b34 => {
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x47e));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800d6b4c; continue; }
                pc = 0x800d6b40;
            }
            0x800d6b40 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x14));
                c.cmps(0, c.r[0], 0x7);
                if c.crbit(2) { pc = 0x800d6b60; continue; }
                pc = 0x800d6b4c;
            }
            0x800d6b4c => {
                c.cmpu(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x800d70f8; continue; }
                pc = 0x800d6b54;
            }
            0x800d6b54 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x488));
                c.cmps(0, c.r[0], 0x7);
                if !c.crbit(2) { pc = 0x800d70f8; continue; }
                pc = 0x800d6b60;
            }
            0x800d6b60 => {
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x18));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d70b4; continue; }
                pc = 0x800d6b6c;
            }
            0x800d6b6c => {
                c.r[3] = c.r[31].wrapping_add(0x98);
                c.r[4] = c.r[4].wrapping_add(0x4ac);
                c.r[5] = 0x18;
                c.lr = 0x800d6b7c; f_800054f4(c);
                pc = 0x800d6b7c;
            }
            0x800d6b7c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[31].wrapping_add(0x80);
                c.r[5] = 0x18;
                c.r[4] = c.r[4].wrapping_add(0x4e8);
                c.lr = 0x800d6b90; f_800054f4(c);
                pc = 0x800d6b90;
            }
            0x800d6b90 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[5] = 0x80;
                c.r[4] = c.r[4].wrapping_add(0x520);
                c.lr = 0x800d6ba4; f_800054f4(c);
                pc = 0x800d6ba4;
            }
            0x800d6ba4 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x148));
                c.r[6] = c.r[31].wrapping_add(0x148);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[31].wrapping_add(0x130);
                c.r[5] = 0x18;
                c.stb(c.r[3].wrapping_add(0x4ac), c.r[0]);
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x4));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[3].wrapping_add(0x4b0), c.r[0]);
                { let ea = c.r[6].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x4b8); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[3].wrapping_add(0x4e8);
                c.lr = 0x800d6be0; f_800054f4(c);
                pc = 0x800d6be0;
            }
            0x800d6be0 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0xb0));
                c.r[3] = c.r[31].wrapping_add(0xb0);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x520), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x2));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x522), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x528); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x10));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x530), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x12));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x532), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x538); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x20));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x540), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x22));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x542), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x548); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x30));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x550), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x32));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x552), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x558); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x40));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x560), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x42));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x562), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x48); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x568); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x50));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x570), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x52));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x572), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x578); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x60));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x580), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x62));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x582), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x68); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x588); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x70));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x590), c.r[0]);
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x72));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x592), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x598); let v = c.f[0]; c.stfs(ea, v); }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d6d1c; continue; }
                pc = 0x800d6d14;
            }
            0x800d6d14 => {
                c.lr = 0x800d6d18; f_800db098(c);
                pc = 0x800d6d18;
            }
            0x800d6d18 => {
                pc = 0x800d6e10; continue;
                pc = 0x800d6d1c;
            }
            0x800d6d1c => {
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1b));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x1a));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800d6e10; continue; }
                pc = 0x800d6d2c;
            }
            0x800d6d2c => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3] & c.r[0];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d6e10; continue; }
                pc = 0x800d6d3c;
            }
            0x800d6d3c => {
                c.r[29] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[0] = c.lbz(c.r[29].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d6d5c; continue; }
                pc = 0x800d6d4c;
            }
            0x800d6d4c => {
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x4a0));
                c.r[3] = 0x0;
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d6d5c; (c.dispatch)(c, t); }
                pc = 0x800d6d5c;
            }
            0x800d6d5c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0x0;
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x4a8));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d6d70; (c.dispatch)(c, t); }
                pc = 0x800d6d70;
            }
            0x800d6d70 => {
                c.r[0] = c.lbz(c.r[29].wrapping_add(0x40));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d6d90; continue; }
                pc = 0x800d6d7c;
            }
            0x800d6d7c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0x0;
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x4c4));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d6d90; (c.dispatch)(c, t); }
                pc = 0x800d6d90;
            }
            0x800d6d90 => {
                c.r[30] = 0x4;
                c.r[28] = c.r[29].wrapping_add(0x70);
                pc = 0x800d6d98;
            }
            0x800d6d98 => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d6dc0; continue; }
                pc = 0x800d6da4;
            }
            0x800d6da4 => {
                c.r[3] = c.r[30].wrapping_add(0xfffffffc);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x4e0);
                c.r[12] = c.lwz(c.r[5].wrapping_add(c.r[0]));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d6dc0; (c.dispatch)(c, t); }
                pc = 0x800d6dc0;
            }
            0x800d6dc0 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[28] = c.r[28].wrapping_add(0x1c);
                c.cmps(0, c.r[30], 0x5);
                if !c.crbit(1) { pc = 0x800d6d98; continue; }
                pc = 0x800d6dd0;
            }
            0x800d6dd0 => {
                c.r[30] = 0x6;
                c.r[28] = c.r[29].wrapping_add(0xa8);
                pc = 0x800d6dd8;
            }
            0x800d6dd8 => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d6e00; continue; }
                pc = 0x800d6de4;
            }
            0x800d6de4 => {
                c.r[3] = c.r[30].wrapping_add(0xfffffffa);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x500);
                c.r[12] = c.lwz(c.r[5].wrapping_add(c.r[0]));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d6e00; (c.dispatch)(c, t); }
                pc = 0x800d6e00;
            }
            0x800d6e00 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[28] = c.r[28].wrapping_add(0x1c);
                c.cmps(0, c.r[30], 0xd);
                if !c.crbit(1) { pc = 0x800d6dd8; continue; }
                pc = 0x800d6e10;
            }
            0x800d6e10 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[31].wrapping_add(0x98);
                c.r[5] = 0x18;
                c.r[3] = c.r[3].wrapping_add(0x4ac);
                c.lr = 0x800d6e24; f_800054f4(c);
                pc = 0x800d6e24;
            }
            0x800d6e24 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[31].wrapping_add(0x80);
                c.r[5] = 0x18;
                c.r[3] = c.r[3].wrapping_add(0x4e8);
                c.lr = 0x800d6e38; f_800054f4(c);
                pc = 0x800d6e38;
            }
            0x800d6e38 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[31].wrapping_add(0x0);
                c.r[5] = 0x80;
                c.r[3] = c.r[3].wrapping_add(0x520);
                c.lr = 0x800d6e4c; f_800054f4(c);
                pc = 0x800d6e4c;
            }
            0x800d6e4c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d6e64; continue; }
                pc = 0x800d6e5c;
            }
            0x800d6e5c => {
                c.lr = 0x800d6e60; f_800db098(c);
                pc = 0x800d6e60;
            }
            0x800d6e60 => {
                pc = 0x800d6f58; continue;
                pc = 0x800d6e64;
            }
            0x800d6e64 => {
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1b));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x1a));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800d6f58; continue; }
                pc = 0x800d6e74;
            }
            0x800d6e74 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3] & c.r[0];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d6f58; continue; }
                pc = 0x800d6e84;
            }
            0x800d6e84 => {
                c.r[30] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d6ea4; continue; }
                pc = 0x800d6e94;
            }
            0x800d6e94 => {
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x4a0));
                c.r[3] = 0x0;
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d6ea4; (c.dispatch)(c, t); }
                pc = 0x800d6ea4;
            }
            0x800d6ea4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0x0;
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x4a8));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d6eb8; (c.dispatch)(c, t); }
                pc = 0x800d6eb8;
            }
            0x800d6eb8 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x40));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d6ed8; continue; }
                pc = 0x800d6ec4;
            }
            0x800d6ec4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0x0;
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x4c4));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d6ed8; (c.dispatch)(c, t); }
                pc = 0x800d6ed8;
            }
            0x800d6ed8 => {
                c.r[29] = 0x4;
                c.r[28] = c.r[30].wrapping_add(0x70);
                pc = 0x800d6ee0;
            }
            0x800d6ee0 => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d6f08; continue; }
                pc = 0x800d6eec;
            }
            0x800d6eec => {
                c.r[3] = c.r[29].wrapping_add(0xfffffffc);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x4e0);
                c.r[12] = c.lwz(c.r[5].wrapping_add(c.r[0]));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d6f08; (c.dispatch)(c, t); }
                pc = 0x800d6f08;
            }
            0x800d6f08 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[28] = c.r[28].wrapping_add(0x1c);
                c.cmps(0, c.r[29], 0x5);
                if !c.crbit(1) { pc = 0x800d6ee0; continue; }
                pc = 0x800d6f18;
            }
            0x800d6f18 => {
                c.r[29] = 0x6;
                c.r[28] = c.r[30].wrapping_add(0xa8);
                pc = 0x800d6f20;
            }
            0x800d6f20 => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d6f48; continue; }
                pc = 0x800d6f2c;
            }
            0x800d6f2c => {
                c.r[3] = c.r[29].wrapping_add(0xfffffffa);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x500);
                c.r[12] = c.lwz(c.r[5].wrapping_add(c.r[0]));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d6f48; (c.dispatch)(c, t); }
                pc = 0x800d6f48;
            }
            0x800d6f48 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[28] = c.r[28].wrapping_add(0x1c);
                c.cmps(0, c.r[29], 0xd);
                if !c.crbit(1) { pc = 0x800d6f20; continue; }
                pc = 0x800d6f58;
            }
            0x800d6f58 => {
                c.r[6] = c.r[31].wrapping_add(0x148);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[5] = c.lbz(c.r[6].wrapping_add(0x1));
                c.r[3] = c.r[31].wrapping_add(0xb0);
                c.r[0] = 0x0;
                c.stb(c.r[4].wrapping_add(0x4ad), c.r[5]);
                c.r[5] = c.lhz(c.r[6].wrapping_add(0x6));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x4b2), c.r[5]);
                { let ea = c.r[6].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x4bc); let v = c.f[0]; c.stfs(ea, v); }
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x1));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x521), c.r[5]);
                c.r[5] = c.lhz(c.r[3].wrapping_add(0x4));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x524), c.r[5]);
                { let ea = c.r[3].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x52c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x11));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x531), c.r[5]);
                c.r[5] = c.lhz(c.r[3].wrapping_add(0x14));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x534), c.r[5]);
                { let ea = c.r[3].wrapping_add(0x1c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x53c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x21));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x541), c.r[5]);
                c.r[5] = c.lhz(c.r[3].wrapping_add(0x24));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x544), c.r[5]);
                { let ea = c.r[3].wrapping_add(0x2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x54c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x31));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x551), c.r[5]);
                c.r[5] = c.lhz(c.r[3].wrapping_add(0x34));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x554), c.r[5]);
                { let ea = c.r[3].wrapping_add(0x3c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x55c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x41));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x561), c.r[5]);
                c.r[5] = c.lhz(c.r[3].wrapping_add(0x44));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x564), c.r[5]);
                { let ea = c.r[3].wrapping_add(0x4c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x56c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x51));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x571), c.r[5]);
                c.r[5] = c.lhz(c.r[3].wrapping_add(0x54));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x574), c.r[5]);
                { let ea = c.r[3].wrapping_add(0x5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x57c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x61));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x581), c.r[5]);
                c.r[5] = c.lhz(c.r[3].wrapping_add(0x64));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x584), c.r[5]);
                { let ea = c.r[3].wrapping_add(0x6c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[4].wrapping_add(0x58c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x71));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[4].wrapping_add(0x591), c.r[5]);
                c.r[5] = c.lhz(c.r[3].wrapping_add(0x74));
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.sth(c.r[4].wrapping_add(0x594), c.r[5]);
                { let ea = c.r[3].wrapping_add(0x7c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                { let ea = c.r[3].wrapping_add(0x59c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stb(c.r[3].wrapping_add(0x18), c.r[0]);
                pc = 0x800d70f8; continue;
                pc = 0x800d70b4;
            }
            0x800d70b4 => {
                c.r[3] = c.r[31].wrapping_add(0x148);
                c.r[4] = c.r[4].wrapping_add(0x4ac);
                c.r[5] = 0x18;
                c.lr = 0x800d70c4; f_800054f4(c);
                pc = 0x800d70c4;
            }
            0x800d70c4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[31].wrapping_add(0x130);
                c.r[5] = 0x18;
                c.r[4] = c.r[4].wrapping_add(0x4e8);
                c.lr = 0x800d70d8; f_800054f4(c);
                pc = 0x800d70d8;
            }
            0x800d70d8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = c.r[31].wrapping_add(0xb0);
                c.r[5] = 0x80;
                c.r[4] = c.r[4].wrapping_add(0x520);
                c.lr = 0x800d70ec; f_800054f4(c);
                pc = 0x800d70ec;
            }
            0x800d70ec => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = 0x1;
                c.stb(c.r[3].wrapping_add(0x18), c.r[0]);
                pc = 0x800d70f8;
            }
            0x800d70f8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d7110; continue; }
                pc = 0x800d7108;
            }
            0x800d7108 => {
                c.lr = 0x800d710c; f_800db098(c);
                pc = 0x800d710c;
            }
            0x800d710c => {
                pc = 0x800d7204; continue;
                pc = 0x800d7110;
            }
            0x800d7110 => {
                c.r[3] = c.lbz(c.r[4].wrapping_add(0x1b));
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x1a));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x800d7204; continue; }
                pc = 0x800d7120;
            }
            0x800d7120 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.r[0] = c.r[3] & c.r[0];
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x800d7204; continue; }
                pc = 0x800d7130;
            }
            0x800d7130 => {
                c.r[30] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d7150; continue; }
                pc = 0x800d7140;
            }
            0x800d7140 => {
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x4a0));
                c.r[3] = 0x0;
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d7150; (c.dispatch)(c, t); }
                pc = 0x800d7150;
            }
            0x800d7150 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0x0;
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x4a8));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d7164; (c.dispatch)(c, t); }
                pc = 0x800d7164;
            }
            0x800d7164 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x40));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d7184; continue; }
                pc = 0x800d7170;
            }
            0x800d7170 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0x0;
                c.r[12] = c.lwz(c.r[4].wrapping_add(0x4c4));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d7184; (c.dispatch)(c, t); }
                pc = 0x800d7184;
            }
            0x800d7184 => {
                c.r[29] = 0x4;
                c.r[28] = c.r[30].wrapping_add(0x70);
                pc = 0x800d718c;
            }
            0x800d718c => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d71b4; continue; }
                pc = 0x800d7198;
            }
            0x800d7198 => {
                c.r[3] = c.r[29].wrapping_add(0xfffffffc);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x4e0);
                c.r[12] = c.lwz(c.r[5].wrapping_add(c.r[0]));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d71b4; (c.dispatch)(c, t); }
                pc = 0x800d71b4;
            }
            0x800d71b4 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[28] = c.r[28].wrapping_add(0x1c);
                c.cmps(0, c.r[29], 0x5);
                if !c.crbit(1) { pc = 0x800d718c; continue; }
                pc = 0x800d71c4;
            }
            0x800d71c4 => {
                c.r[29] = 0x6;
                c.r[28] = c.r[30].wrapping_add(0xa8);
                pc = 0x800d71cc;
            }
            0x800d71cc => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d71f4; continue; }
                pc = 0x800d71d8;
            }
            0x800d71d8 => {
                c.r[3] = c.r[29].wrapping_add(0xfffffffa);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[4] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x500);
                c.r[12] = c.lwz(c.r[5].wrapping_add(c.r[0]));
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x800d71f4; (c.dispatch)(c, t); }
                pc = 0x800d71f4;
            }
            0x800d71f4 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[28] = c.r[28].wrapping_add(0x1c);
                c.cmps(0, c.r[29], 0xd);
                if !c.crbit(1) { pc = 0x800d71cc; continue; }
                pc = 0x800d7204;
            }
            0x800d7204 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x49f), c.r[0]);
                pc = 0x800d7210;
            }
            0x800d7210 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x10));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d6b00: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7230 (0x800d7230, 28 bytes)
#[inline(never)]
pub fn f_800d7230(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7230) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7230); f_800d7230_b(c, 0x800d7230); crate::prof::exit(c, 0x800d7230); return; }
    }
    f_800d7230_b(c, 0x800d7230)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7230_at(c: &mut Cpu, pc: u32) { f_800d7230_b(c, pc) }
#[inline(always)]
fn f_800d7230_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7230;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7230, pc); }
        match pc {
            0x800d7230 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[4] = c.lbz(c.r[4].wrapping_add(0x4a4));
                c.r[0] = c.r[4].rotate_left(1) & 0xfffffffe;
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7230: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D724C (0x800d724c, 28 bytes)
#[inline(never)]
pub fn f_800d724c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d724c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d724c); f_800d724c_b(c, 0x800d724c); crate::prof::exit(c, 0x800d724c); return; }
    }
    f_800d724c_b(c, 0x800d724c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d724c_at(c: &mut Cpu, pc: u32) { f_800d724c_b(c, pc) }
#[inline(always)]
fn f_800d724c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d724c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d724c, pc); }
        match pc {
            0x800d724c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x520));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d724c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7268 (0x800d7268, 28 bytes)
#[inline(never)]
pub fn f_800d7268(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7268) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7268); f_800d7268_b(c, 0x800d7268); crate::prof::exit(c, 0x800d7268); return; }
    }
    f_800d7268_b(c, 0x800d7268)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7268_at(c: &mut Cpu, pc: u32) { f_800d7268_b(c, pc) }
#[inline(always)]
fn f_800d7268_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7268;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7268, pc); }
        match pc {
            0x800d7268 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x522));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7268: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7284 (0x800d7284, 32 bytes)
#[inline(never)]
pub fn f_800d7284(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7284) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7284); f_800d7284_b(c, 0x800d7284); crate::prof::exit(c, 0x800d7284); return; }
    }
    f_800d7284_b(c, 0x800d7284)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7284_at(c: &mut Cpu, pc: u32) { f_800d7284_b(c, pc) }
#[inline(always)]
fn f_800d7284_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7284;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7284, pc); }
        match pc {
            0x800d7284 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x520));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7284: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D72A4 (0x800d72a4, 32 bytes)
#[inline(never)]
pub fn f_800d72a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d72a4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d72a4); f_800d72a4_b(c, 0x800d72a4); crate::prof::exit(c, 0x800d72a4); return; }
    }
    f_800d72a4_b(c, 0x800d72a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d72a4_at(c: &mut Cpu, pc: u32) { f_800d72a4_b(c, pc) }
#[inline(always)]
fn f_800d72a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d72a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d72a4, pc); }
        match pc {
            0x800d72a4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x520));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d72a4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D72C4 (0x800d72c4, 32 bytes)
#[inline(never)]
pub fn f_800d72c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d72c4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d72c4); f_800d72c4_b(c, 0x800d72c4); crate::prof::exit(c, 0x800d72c4); return; }
    }
    f_800d72c4_b(c, 0x800d72c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d72c4_at(c: &mut Cpu, pc: u32) { f_800d72c4_b(c, pc) }
#[inline(always)]
fn f_800d72c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d72c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d72c4, pc); }
        match pc {
            0x800d72c4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x522));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d72c4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D72E4 (0x800d72e4, 32 bytes)
#[inline(never)]
pub fn f_800d72e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d72e4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d72e4); f_800d72e4_b(c, 0x800d72e4); crate::prof::exit(c, 0x800d72e4); return; }
    }
    f_800d72e4_b(c, 0x800d72e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d72e4_at(c: &mut Cpu, pc: u32) { f_800d72e4_b(c, pc) }
#[inline(always)]
fn f_800d72e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d72e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d72e4, pc); }
        match pc {
            0x800d72e4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x522));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d72e4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7304 (0x800d7304, 36 bytes)
#[inline(never)]
pub fn f_800d7304(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7304) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7304); f_800d7304_b(c, 0x800d7304); crate::prof::exit(c, 0x800d7304); return; }
    }
    f_800d7304_b(c, 0x800d7304)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7304_at(c: &mut Cpu, pc: u32) { f_800d7304_b(c, pc) }
#[inline(always)]
fn f_800d7304_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7304;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7304, pc); }
        match pc {
            0x800d7304 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                { let ea = c.r[4].wrapping_add(0x52c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x528); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0xffff8000); let v = c.f[1]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_800d7304: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7328 (0x800d7328, 28 bytes)
#[inline(never)]
pub fn f_800d7328(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7328) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7328); f_800d7328_b(c, 0x800d7328); crate::prof::exit(c, 0x800d7328); return; }
    }
    f_800d7328_b(c, 0x800d7328)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7328_at(c: &mut Cpu, pc: u32) { f_800d7328_b(c, pc) }
#[inline(always)]
fn f_800d7328_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7328;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7328, pc); }
        match pc {
            0x800d7328 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x4e8));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7328: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7344 (0x800d7344, 28 bytes)
#[inline(never)]
pub fn f_800d7344(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7344) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7344); f_800d7344_b(c, 0x800d7344); crate::prof::exit(c, 0x800d7344); return; }
    }
    f_800d7344_b(c, 0x800d7344)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7344_at(c: &mut Cpu, pc: u32) { f_800d7344_b(c, pc) }
#[inline(always)]
fn f_800d7344_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7344;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7344, pc); }
        match pc {
            0x800d7344 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x4ec));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7344: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7360 (0x800d7360, 28 bytes)
#[inline(never)]
pub fn f_800d7360(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7360) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7360); f_800d7360_b(c, 0x800d7360); crate::prof::exit(c, 0x800d7360); return; }
    }
    f_800d7360_b(c, 0x800d7360)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7360_at(c: &mut Cpu, pc: u32) { f_800d7360_b(c, pc) }
#[inline(always)]
fn f_800d7360_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7360;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7360, pc); }
        match pc {
            0x800d7360 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x4ec));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7360: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D737C (0x800d737c, 28 bytes)
#[inline(never)]
pub fn f_800d737c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d737c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d737c); f_800d737c_b(c, 0x800d737c); crate::prof::exit(c, 0x800d737c); return; }
    }
    f_800d737c_b(c, 0x800d737c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d737c_at(c: &mut Cpu, pc: u32) { f_800d737c_b(c, pc) }
#[inline(always)]
fn f_800d737c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d737c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d737c, pc); }
        match pc {
            0x800d737c => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4f0));
                c.stw(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d737c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7398 (0x800d7398, 44 bytes)
#[inline(never)]
pub fn f_800d7398(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7398) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7398); f_800d7398_b(c, 0x800d7398); crate::prof::exit(c, 0x800d7398); return; }
    }
    f_800d7398_b(c, 0x800d7398)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7398_at(c: &mut Cpu, pc: u32) { f_800d7398_b(c, pc) }
#[inline(always)]
fn f_800d7398_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7398;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7398, pc); }
        match pc {
            0x800d7398 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[6] = c.r[4].wrapping_add(c.r[0]);
                c.r[5] = c.lbz(c.r[6].wrapping_add(0x4ea));
                c.r[4] = c.lbz(c.r[6].wrapping_add(0x4e9));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x4e8));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7398: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D73C4 (0x800d73c4, 52 bytes)
#[inline(never)]
pub fn f_800d73c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d73c4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d73c4); f_800d73c4_b(c, 0x800d73c4); crate::prof::exit(c, 0x800d73c4); return; }
    }
    f_800d73c4_b(c, 0x800d73c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d73c4_at(c: &mut Cpu, pc: u32) { f_800d73c4_b(c, pc) }
#[inline(always)]
fn f_800d73c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d73c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d73c4, pc); }
        match pc {
            0x800d73c4 => {
                c.r[0] = c.r[3].wrapping_mul(0xc);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[7] = c.r[4].wrapping_add(c.r[0]);
                c.r[6] = c.lbz(c.r[7].wrapping_add(0x4eb));
                c.r[5] = c.lbz(c.r[7].wrapping_add(0x4ea));
                c.r[4] = c.lbz(c.r[7].wrapping_add(0x4e9));
                c.r[0] = c.lbz(c.r[7].wrapping_add(0x4e8));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[6]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d73c4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D73F8 (0x800d73f8, 20 bytes)
#[inline(never)]
pub fn f_800d73f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d73f8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d73f8); f_800d73f8_b(c, 0x800d73f8); crate::prof::exit(c, 0x800d73f8); return; }
    }
    f_800d73f8_b(c, 0x800d73f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d73f8_at(c: &mut Cpu, pc: u32) { f_800d73f8_b(c, pc) }
#[inline(always)]
fn f_800d73f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d73f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d73f8, pc); }
        match pc {
            0x800d73f8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x4c8));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d73f8: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D740C (0x800d740c, 20 bytes)
#[inline(never)]
pub fn f_800d740c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d740c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d740c); f_800d740c_b(c, 0x800d740c); crate::prof::exit(c, 0x800d740c); return; }
    }
    f_800d740c_b(c, 0x800d740c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d740c_at(c: &mut Cpu, pc: u32) { f_800d740c_b(c, pc) }
#[inline(always)]
fn f_800d740c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d740c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d740c, pc); }
        match pc {
            0x800d740c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x4cc));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d740c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7420 (0x800d7420, 36 bytes)
#[inline(never)]
pub fn f_800d7420(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7420) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7420); f_800d7420_b(c, 0x800d7420); crate::prof::exit(c, 0x800d7420); return; }
    }
    f_800d7420_b(c, 0x800d7420)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7420_at(c: &mut Cpu, pc: u32) { f_800d7420_b(c, pc) }
#[inline(always)]
fn f_800d7420_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7420;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7420, pc); }
        match pc {
            0x800d7420 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[5] = c.lbz(c.r[6].wrapping_add(0x4ca));
                c.r[4] = c.lbz(c.r[6].wrapping_add(0x4c9));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x4c8));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7420: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7444 (0x800d7444, 36 bytes)
#[inline(never)]
pub fn f_800d7444(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7444) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7444); f_800d7444_b(c, 0x800d7444); crate::prof::exit(c, 0x800d7444); return; }
    }
    f_800d7444_b(c, 0x800d7444)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7444_at(c: &mut Cpu, pc: u32) { f_800d7444_b(c, pc) }
#[inline(always)]
fn f_800d7444_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7444;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7444, pc); }
        match pc {
            0x800d7444 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[5] = c.lhz(c.r[6].wrapping_add(0x4d0));
                c.r[4] = c.lhz(c.r[6].wrapping_add(0x4ce));
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x4cc));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7444: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7468 (0x800d7468, 36 bytes)
#[inline(never)]
pub fn f_800d7468(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7468) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7468); f_800d7468_b(c, 0x800d7468); crate::prof::exit(c, 0x800d7468); return; }
    }
    f_800d7468_b(c, 0x800d7468)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7468_at(c: &mut Cpu, pc: u32) { f_800d7468_b(c, pc) }
#[inline(always)]
fn f_800d7468_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7468;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7468, pc); }
        match pc {
            0x800d7468 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                { let ea = c.r[4].wrapping_add(0x4dc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x4d8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x4d4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0xffff8000); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0xffff8000); let v = c.f[2]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_800d7468: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D748C (0x800d748c, 20 bytes)
#[inline(never)]
pub fn f_800d748c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d748c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d748c); f_800d748c_b(c, 0x800d748c); crate::prof::exit(c, 0x800d748c); return; }
    }
    f_800d748c_b(c, 0x800d748c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d748c_at(c: &mut Cpu, pc: u32) { f_800d748c_b(c, pc) }
#[inline(always)]
fn f_800d748c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d748c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d748c, pc); }
        match pc {
            0x800d748c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[0] = c.lbz(c.r[4].wrapping_add(0x4ac));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d748c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D74A0 (0x800d74a0, 20 bytes)
#[inline(never)]
pub fn f_800d74a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d74a0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d74a0); f_800d74a0_b(c, 0x800d74a0); crate::prof::exit(c, 0x800d74a0); return; }
    }
    f_800d74a0_b(c, 0x800d74a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d74a0_at(c: &mut Cpu, pc: u32) { f_800d74a0_b(c, pc) }
#[inline(always)]
fn f_800d74a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d74a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d74a0, pc); }
        match pc {
            0x800d74a0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[0] = c.lhz(c.r[4].wrapping_add(0x4b0));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d74a0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D74B4 (0x800d74b4, 28 bytes)
#[inline(never)]
pub fn f_800d74b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d74b4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d74b4); f_800d74b4_b(c, 0x800d74b4); crate::prof::exit(c, 0x800d74b4); return; }
    }
    f_800d74b4_b(c, 0x800d74b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d74b4_at(c: &mut Cpu, pc: u32) { f_800d74b4_b(c, pc) }
#[inline(always)]
fn f_800d74b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d74b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d74b4, pc); }
        match pc {
            0x800d74b4 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[4] = c.lbz(c.r[5].wrapping_add(0x4ad));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x4ac));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d74b4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D74D0 (0x800d74d0, 28 bytes)
#[inline(never)]
pub fn f_800d74d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d74d0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d74d0); f_800d74d0_b(c, 0x800d74d0); crate::prof::exit(c, 0x800d74d0); return; }
    }
    f_800d74d0_b(c, 0x800d74d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d74d0_at(c: &mut Cpu, pc: u32) { f_800d74d0_b(c, pc) }
#[inline(always)]
fn f_800d74d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d74d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d74d0, pc); }
        match pc {
            0x800d74d0 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[4] = c.lbz(c.r[5].wrapping_add(0x4ad));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x4ac));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d74d0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D74EC (0x800d74ec, 28 bytes)
#[inline(never)]
pub fn f_800d74ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d74ec) { return; }
        if c.prof { crate::prof::enter(c, 0x800d74ec); f_800d74ec_b(c, 0x800d74ec); crate::prof::exit(c, 0x800d74ec); return; }
    }
    f_800d74ec_b(c, 0x800d74ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d74ec_at(c: &mut Cpu, pc: u32) { f_800d74ec_b(c, pc) }
#[inline(always)]
fn f_800d74ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d74ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d74ec, pc); }
        match pc {
            0x800d74ec => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[4] = c.lhz(c.r[5].wrapping_add(0x4b2));
                c.r[0] = c.lhz(c.r[5].wrapping_add(0x4b0));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d74ec: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7508 (0x800d7508, 28 bytes)
#[inline(never)]
pub fn f_800d7508(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7508) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7508); f_800d7508_b(c, 0x800d7508); crate::prof::exit(c, 0x800d7508); return; }
    }
    f_800d7508_b(c, 0x800d7508)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7508_at(c: &mut Cpu, pc: u32) { f_800d7508_b(c, pc) }
#[inline(always)]
fn f_800d7508_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7508;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7508, pc); }
        match pc {
            0x800d7508 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[4] = c.lhz(c.r[5].wrapping_add(0x4b2));
                c.r[0] = c.lhz(c.r[5].wrapping_add(0x4b0));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7508: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7524 (0x800d7524, 28 bytes)
#[inline(never)]
pub fn f_800d7524(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7524) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7524); f_800d7524_b(c, 0x800d7524); crate::prof::exit(c, 0x800d7524); return; }
    }
    f_800d7524_b(c, 0x800d7524)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7524_at(c: &mut Cpu, pc: u32) { f_800d7524_b(c, pc) }
#[inline(always)]
fn f_800d7524_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7524;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7524, pc); }
        match pc {
            0x800d7524 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                { let ea = c.r[4].wrapping_add(0x4bc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x4b8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0xffff8000); let v = c.f[1]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_800d7524: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7540 (0x800d7540, 36 bytes)
#[inline(never)]
pub fn f_800d7540(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7540) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7540); f_800d7540_b(c, 0x800d7540); crate::prof::exit(c, 0x800d7540); return; }
    }
    f_800d7540_b(c, 0x800d7540)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7540_at(c: &mut Cpu, pc: u32) { f_800d7540_b(c, pc) }
#[inline(always)]
fn f_800d7540_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7540;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7540, pc); }
        match pc {
            0x800d7540 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[5] = c.lbz(c.r[6].wrapping_add(0x4ae));
                c.r[4] = c.lbz(c.r[6].wrapping_add(0x4ad));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x4ac));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7540: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7564 (0x800d7564, 36 bytes)
#[inline(never)]
pub fn f_800d7564(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7564) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7564); f_800d7564_b(c, 0x800d7564); crate::prof::exit(c, 0x800d7564); return; }
    }
    f_800d7564_b(c, 0x800d7564)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7564_at(c: &mut Cpu, pc: u32) { f_800d7564_b(c, pc) }
#[inline(always)]
fn f_800d7564_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7564;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7564, pc); }
        match pc {
            0x800d7564 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[5] = c.lbz(c.r[6].wrapping_add(0x4ae));
                c.r[4] = c.lbz(c.r[6].wrapping_add(0x4ad));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x4ac));
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.stb(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7564: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7588 (0x800d7588, 36 bytes)
#[inline(never)]
pub fn f_800d7588(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7588) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7588); f_800d7588_b(c, 0x800d7588); crate::prof::exit(c, 0x800d7588); return; }
    }
    f_800d7588_b(c, 0x800d7588)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7588_at(c: &mut Cpu, pc: u32) { f_800d7588_b(c, pc) }
#[inline(always)]
fn f_800d7588_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7588;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7588, pc); }
        match pc {
            0x800d7588 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[5] = c.lhz(c.r[6].wrapping_add(0x4b4));
                c.r[4] = c.lhz(c.r[6].wrapping_add(0x4b2));
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x4b0));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7588: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D75AC (0x800d75ac, 36 bytes)
#[inline(never)]
pub fn f_800d75ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d75ac) { return; }
        if c.prof { crate::prof::enter(c, 0x800d75ac); f_800d75ac_b(c, 0x800d75ac); crate::prof::exit(c, 0x800d75ac); return; }
    }
    f_800d75ac_b(c, 0x800d75ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d75ac_at(c: &mut Cpu, pc: u32) { f_800d75ac_b(c, pc) }
#[inline(always)]
fn f_800d75ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d75ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d75ac, pc); }
        match pc {
            0x800d75ac => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                c.r[5] = c.lhz(c.r[6].wrapping_add(0x4b4));
                c.r[4] = c.lhz(c.r[6].wrapping_add(0x4b2));
                c.r[0] = c.lhz(c.r[6].wrapping_add(0x4b0));
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[0]);
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[4]);
                c.sth(c.r[3].wrapping_add(0xffff8000), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d75ac: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D75D0 (0x800d75d0, 36 bytes)
#[inline(never)]
pub fn f_800d75d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d75d0) { return; }
        if c.prof { crate::prof::enter(c, 0x800d75d0); f_800d75d0_b(c, 0x800d75d0); crate::prof::exit(c, 0x800d75d0); return; }
    }
    f_800d75d0_b(c, 0x800d75d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d75d0_at(c: &mut Cpu, pc: u32) { f_800d75d0_b(c, pc) }
#[inline(always)]
fn f_800d75d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d75d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d75d0, pc); }
        match pc {
            0x800d75d0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[3] = 0xcc010000;
                { let ea = c.r[4].wrapping_add(0x4c0); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x4bc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x4b8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0xffff8000); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0xffff8000); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0xffff8000); let v = c.f[2]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_800d75d0: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D75F4 (0x800d75f4, 92 bytes)
#[inline(never)]
pub fn f_800d75f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d75f4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d75f4); f_800d75f4_b(c, 0x800d75f4); crate::prof::exit(c, 0x800d75f4); return; }
    }
    f_800d75f4_b(c, 0x800d75f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d75f4_at(c: &mut Cpu, pc: u32) { f_800d75f4_b(c, pc) }
#[inline(always)]
fn f_800d75f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d75f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d75f4, pc); }
        match pc {
            0x800d75f4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x24));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x800d760c; continue; }
                pc = 0x800d7604;
            }
            0x800d7604 => {
                c.r[0] = 0x0;
                c.stw(c.r[4].wrapping_add(0x24), c.r[0]);
                pc = 0x800d760c;
            }
            0x800d760c => {
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x8), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x24), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x40), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x5c), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x78), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x94), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xb0), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xcc), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xe8), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x104), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x120), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x13c), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x158), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x174), c.r[0]);
                c.stb(c.r[3], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d75f4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7650 (0x800d7650, 88 bytes)
#[inline(never)]
pub fn f_800d7650(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7650) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7650); f_800d7650_b(c, 0x800d7650); crate::prof::exit(c, 0x800d7650); return; }
    }
    f_800d7650_b(c, 0x800d7650)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7650_at(c: &mut Cpu, pc: u32) { f_800d7650_b(c, pc) }
#[inline(always)]
fn f_800d7650_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7650;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7650, pc); }
        match pc {
            0x800d7650 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x24));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x800d7668; continue; }
                pc = 0x800d7660;
            }
            0x800d7660 => {
                c.r[0] = 0x0;
                c.stw(c.r[4].wrapping_add(0x24), c.r[0]);
                pc = 0x800d7668;
            }
            0x800d7668 => {
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x8), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x24), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x40), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x5c), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x78), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x94), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xb0), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xcc), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xe8), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x104), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x120), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x13c), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x158), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x174), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7650: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D76A8 (0x800d76a8, 376 bytes)
#[inline(never)]
pub fn f_800d76a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d76a8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d76a8); f_800d76a8_b(c, 0x800d76a8); crate::prof::exit(c, 0x800d76a8); return; }
    }
    f_800d76a8_b(c, 0x800d76a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d76a8_at(c: &mut Cpu, pc: u32) { f_800d76a8_b(c, pc) }
#[inline(always)]
fn f_800d76a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d76a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d76a8, pc); }
        match pc {
            0x800d76a8 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x47e));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d76e8; continue; }
                pc = 0x800d76d0;
            }
            0x800d76d0 => {
                c.r[24] = c.lwz(c.r[5].wrapping_add(0x24));
                c.stw(c.r[5].wrapping_add(0x24), c.r[29]);
                c.lr = 0x800d76dc; f_800d7940(c);
                pc = 0x800d76dc;
            }
            0x800d76dc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.stw(c.r[3].wrapping_add(0x24), c.r[24]);
                pc = 0x800d780c; continue;
                pc = 0x800d76e8;
            }
            0x800d76e8 => {
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d7710; continue; }
                pc = 0x800d76f4;
            }
            0x800d76f4 => {
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30].rotate_left(0) & 0xffff;
                c.r[3] = 0x47;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d770c; f_800d4f98(c);
                pc = 0x800d770c;
            }
            0x800d770c => {
                pc = 0x800d780c; continue;
                pc = 0x800d7710;
            }
            0x800d7710 => {
                c.r[3] = 0x80310000;
                c.r[4] = 0x80310000;
                c.r[31] = c.r[3].wrapping_add(0x4370);
                c.r[25] = c.r[29] | c.r[29];
                c.r[3] = 0x80310000;
                c.r[27] = c.r[4].wrapping_add(0x43b4);
                c.r[24] = c.r[31] | c.r[31];
                c.r[26] = 0x0;
                c.r[28] = c.r[3].wrapping_add(0x43d8);
                pc = 0x800d7734;
            }
            0x800d7734 => {
                c.r[0] = c.lbz(c.r[25].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d7770; continue; }
                pc = 0x800d7740;
            }
            0x800d7740 => {
                c.cmps(0, c.r[26], 0x0);
                if c.crbit(2) { pc = 0x800d7770; continue; }
                pc = 0x800d7748;
            }
            0x800d7748 => {
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x10));
                c.r[0] = c.lwz(c.r[25].wrapping_add(0x14));
                c.r[5] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[4] = c.lwz(c.r[24]);
                c.r[5] = c.lwz(c.r[27].wrapping_add(c.r[5]));
                c.r[6] = c.lwz(c.r[28].wrapping_add(c.r[0]));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x18));
                c.lr = 0x800d7770; f_800b7d74(c);
                pc = 0x800d7770;
            }
            0x800d7770 => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                c.r[24] = c.r[24].wrapping_add(0x4);
                c.cmps(0, c.r[26], 0xe);
                c.r[25] = c.r[25].wrapping_add(0x1c);
                if c.crbit(0) { pc = 0x800d7734; continue; }
                pc = 0x800d7784;
            }
            0x800d7784 => {
                c.lr = 0x800d7788; f_800b7d3c(c);
                pc = 0x800d7788;
            }
            0x800d7788 => {
                c.r[3] = 0x80310000;
                c.r[24] = c.r[29] | c.r[29];
                c.r[25] = 0x0;
                c.r[28] = c.r[3].wrapping_add(0x43a8);
                pc = 0x800d7798;
            }
            0x800d7798 => {
                c.r[0] = c.lbz(c.r[24].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d77d0; continue; }
                pc = 0x800d77a4;
            }
            0x800d77a4 => {
                c.r[0] = c.lwz(c.r[24].wrapping_add(0xc));
                c.r[3] = c.lwz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[4] = c.lwz(c.r[28].wrapping_add(c.r[0]));
                c.lr = 0x800d77b8; f_800b7874(c);
                pc = 0x800d77b8;
            }
            0x800d77b8 => {
                c.r[4] = c.lwz(c.r[24].wrapping_add(0x1c));
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800d77d0; continue; }
                pc = 0x800d77c4;
            }
            0x800d77c4 => {
                c.r[3] = c.lwz(c.r[31]);
                c.r[5] = c.lbz(c.r[24].wrapping_add(0x20));
                c.lr = 0x800d77d0; f_800b84e0(c);
                pc = 0x800d77d0;
            }
            0x800d77d0 => {
                c.r[25] = c.r[25].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x4);
                c.cmps(0, c.r[25], 0xe);
                c.r[24] = c.r[24].wrapping_add(0x1c);
                if c.crbit(0) { pc = 0x800d7798; continue; }
                pc = 0x800d77e4;
            }
            0x800d77e4 => {
                c.r[4] = 0x80400000;
                c.r[3] = c.r[29] | c.r[29];
                c.r[5] = c.r[4].wrapping_add(0x1f0);
                c.r[4] = c.lwz(c.r[5].wrapping_add(0x14));
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[5].wrapping_add(0x14), c.r[0]);
                c.lr = 0x800d7800; f_800d892c(c);
                pc = 0x800d7800;
            }
            0x800d7800 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x800d780c; f_800d7940(c);
                pc = 0x800d780c;
            }
            0x800d780c => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_800d76a8: bad pc {:#x}", pc),
        }
    }
}

/// gfxSetVtxDesc (0x800d7820, 72 bytes)
#[inline(never)]
pub fn f_800d7820(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7820) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7820); f_800d7820_b(c, 0x800d7820); crate::prof::exit(c, 0x800d7820); return; }
    }
    f_800d7820_b(c, 0x800d7820)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7820_at(c: &mut Cpu, pc: u32) { f_800d7820_b(c, pc) }
#[inline(always)]
fn f_800d7820_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7820;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7820, pc); }
        match pc {
            0x800d7820 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d7854; continue; }
                pc = 0x800d783c;
            }
            0x800d783c => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = 0x46;
                c.r[4] = 0x1;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d7850; f_800d4f98(c);
                pc = 0x800d7850;
            }
            0x800d7850 => {
                pc = 0x800d7858; continue;
                pc = 0x800d7854;
            }
            0x800d7854 => {
                c.stw(c.r[4].wrapping_add(0x24), c.r[3]);
                pc = 0x800d7858;
            }
            0x800d7858 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7820: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7868 (0x800d7868, 44 bytes)
#[inline(never)]
pub fn f_800d7868(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7868) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7868); f_800d7868_b(c, 0x800d7868); crate::prof::exit(c, 0x800d7868); return; }
    }
    f_800d7868_b(c, 0x800d7868)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7868_at(c: &mut Cpu, pc: u32) { f_800d7868_b(c, pc) }
#[inline(always)]
fn f_800d7868_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7868;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7868, pc); }
        match pc {
            0x800d7868 => {
                c.r[4] = c.r[4].wrapping_mul(0x1c);
                c.r[0] = 0x1;
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.stb(c.r[3].wrapping_add(0x8), c.r[0]);
                c.stw(c.r[3].wrapping_add(0xc), c.r[5]);
                c.stw(c.r[3].wrapping_add(0x10), c.r[6]);
                c.stw(c.r[3].wrapping_add(0x14), c.r[7]);
                c.stb(c.r[3].wrapping_add(0x18), c.r[8]);
                c.stw(c.r[3].wrapping_add(0x1c), c.r[9]);
                c.stb(c.r[3].wrapping_add(0x20), c.r[10]);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7868: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7894 (0x800d7894, 172 bytes)
#[inline(never)]
pub fn f_800d7894(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7894) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7894); f_800d7894_b(c, 0x800d7894); crate::prof::exit(c, 0x800d7894); return; }
    }
    f_800d7894_b(c, 0x800d7894)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7894_at(c: &mut Cpu, pc: u32) { f_800d7894_b(c, pc) }
#[inline(always)]
fn f_800d7894_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7894;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7894, pc); }
        match pc {
            0x800d7894 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa290));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa28c));
                c.mtspr(9, c.r[0]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(1) { pc = 0x800d7938; continue; }
                pc = 0x800d78a8;
            }
            0x800d78a8 => {
                c.r[0] = c.lbz(c.r[3]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x800d7930; continue; }
                pc = 0x800d78b4;
            }
            0x800d78b4 => {
                c.r[0] = 0x1;
                c.r[4] = 0x80310000;
                c.stb(c.r[3], c.r[0]);
                c.r[4] = c.r[4].wrapping_add(0x44d0);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa294));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                c.stw(c.r[3].wrapping_add(0x4), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa294));
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x8);
                c.stw(c.r[13].wrapping_add(0xffffa294), c.r[0]);
                if c.crbit(0) { pc = 0x800d78f0; continue; }
                pc = 0x800d78e8;
            }
            0x800d78e8 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa294), c.r[0]);
                pc = 0x800d78f0;
            }
            0x800d78f0 => {
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x8), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x24), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x40), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x5c), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x78), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x94), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xb0), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xcc), c.r[0]);
                c.stb(c.r[3].wrapping_add(0xe8), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x104), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x120), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x13c), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x158), c.r[0]);
                c.stb(c.r[3].wrapping_add(0x174), c.r[0]);
                if true { return; }
                pc = 0x800d7930;
            }
            0x800d7930 => {
                c.r[3] = c.r[3].wrapping_add(0x190);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x800d78a8; continue; }
                pc = 0x800d7938;
            }
            0x800d7938 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_800d7894: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7940 (0x800d7940, 304 bytes)
#[inline(never)]
pub fn f_800d7940(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7940) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7940); f_800d7940_b(c, 0x800d7940); crate::prof::exit(c, 0x800d7940); return; }
    }
    f_800d7940_b(c, 0x800d7940)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7940_at(c: &mut Cpu, pc: u32) { f_800d7940_b(c, pc) }
#[inline(always)]
fn f_800d7940_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7940;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7940, pc); }
        match pc {
            0x800d7940 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[4] | c.r[4];
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x800d7960; f_800d67bc(c);
                pc = 0x800d7960;
            }
            0x800d7960 => {
                c.r[31] = c.r[28].rotate_left(0) & 0xffff;
                c.r[28] = 0x0;
                pc = 0x800d7a4c; continue;
                pc = 0x800d796c;
            }
            0x800d796c => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x28));
                c.cmps(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x800d7984; continue; }
                pc = 0x800d7978;
            }
            0x800d7978 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x800d7980; f_800d6028(c);
                pc = 0x800d7980;
            }
            0x800d7980 => {
                pc = 0x800d798c; continue;
                pc = 0x800d7984;
            }
            0x800d7984 => {
                c.r[3] = c.r[28].rotate_left(0) & 0xff;
                c.lr = 0x800d798c; f_800d5fa4(c);
                pc = 0x800d798c;
            }
            0x800d798c => {
                c.r[0] = c.lbz(c.r[27].wrapping_add(0x40));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d79b8; continue; }
                pc = 0x800d7998;
            }
            0x800d7998 => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x800d79b0; continue; }
                pc = 0x800d79a4;
            }
            0x800d79a4 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x800d79ac; f_800d5dd0(c);
                pc = 0x800d79ac;
            }
            0x800d79ac => {
                pc = 0x800d79b8; continue;
                pc = 0x800d79b0;
            }
            0x800d79b0 => {
                c.r[3] = c.r[28].rotate_left(0) & 0xff;
                c.lr = 0x800d79b8; f_800d5d6c(c);
                pc = 0x800d79b8;
            }
            0x800d79b8 => {
                c.r[29] = 0x4;
                c.r[30] = c.r[27].wrapping_add(0x70);
                pc = 0x800d79c0;
            }
            0x800d79c0 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d79f0; continue; }
                pc = 0x800d79cc;
            }
            0x800d79cc => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[3] = c.r[29].wrapping_add(0xfffffffc);
                c.cmps(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x800d79e8; continue; }
                pc = 0x800d79dc;
            }
            0x800d79dc => {
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x800d79e4; f_800d5ab0(c);
                pc = 0x800d79e4;
            }
            0x800d79e4 => {
                pc = 0x800d79f0; continue;
                pc = 0x800d79e8;
            }
            0x800d79e8 => {
                c.r[4] = c.r[28].rotate_left(0) & 0xff;
                c.lr = 0x800d79f0; f_800d5a38(c);
                pc = 0x800d79f0;
            }
            0x800d79f0 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[30] = c.r[30].wrapping_add(0x1c);
                c.cmps(0, c.r[29], 0x5);
                if !c.crbit(1) { pc = 0x800d79c0; continue; }
                pc = 0x800d7a00;
            }
            0x800d7a00 => {
                c.r[29] = 0x6;
                c.r[30] = c.r[27].wrapping_add(0xa8);
                pc = 0x800d7a08;
            }
            0x800d7a08 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d7a38; continue; }
                pc = 0x800d7a14;
            }
            0x800d7a14 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[3] = c.r[29].wrapping_add(0xfffffffa);
                c.cmps(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x800d7a30; continue; }
                pc = 0x800d7a24;
            }
            0x800d7a24 => {
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x800d7a2c; f_800d579c(c);
                pc = 0x800d7a2c;
            }
            0x800d7a2c => {
                pc = 0x800d7a38; continue;
                pc = 0x800d7a30;
            }
            0x800d7a30 => {
                c.r[4] = c.r[28].rotate_left(0) & 0xff;
                c.lr = 0x800d7a38; f_800d5724(c);
                pc = 0x800d7a38;
            }
            0x800d7a38 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[30] = c.r[30].wrapping_add(0x1c);
                c.cmps(0, c.r[29], 0xd);
                if !c.crbit(1) { pc = 0x800d7a08; continue; }
                pc = 0x800d7a48;
            }
            0x800d7a48 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x800d7a4c;
            }
            0x800d7a4c => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x800d796c; continue; }
                pc = 0x800d7a58;
            }
            0x800d7a58 => {
                c.lr = 0x800d7a5c; f_800d6728(c);
                pc = 0x800d7a5c;
            }
            0x800d7a5c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7940: bad pc {:#x}", pc),
        }
    }
}

/// gfxLoadVtxDesc (0x800d7a70, 272 bytes)
#[inline(never)]
pub fn f_800d7a70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7a70) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7a70); f_800d7a70_b(c, 0x800d7a70); crate::prof::exit(c, 0x800d7a70); return; }
    }
    f_800d7a70_b(c, 0x800d7a70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7a70_at(c: &mut Cpu, pc: u32) { f_800d7a70_b(c, pc) }
#[inline(always)]
fn f_800d7a70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7a70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7a70, pc); }
        match pc {
            0x800d7a70 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x80310000;
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.r[31] = c.r[4].wrapping_add(0x4370);
                c.r[4] = 0x80310000;
                c.r[3] = 0x80310000;
                c.r[28] = c.r[25] | c.r[25];
                c.r[27] = c.r[31] | c.r[31];
                c.r[29] = c.r[4].wrapping_add(0x43b4);
                c.r[30] = c.r[3].wrapping_add(0x43d8);
                c.r[26] = 0x0;
                pc = 0x800d7aa8;
            }
            0x800d7aa8 => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d7ae4; continue; }
                pc = 0x800d7ab4;
            }
            0x800d7ab4 => {
                c.cmps(0, c.r[26], 0x0);
                if c.crbit(2) { pc = 0x800d7ae4; continue; }
                pc = 0x800d7abc;
            }
            0x800d7abc => {
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x10));
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x14));
                c.r[5] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[4] = c.lwz(c.r[27]);
                c.r[5] = c.lwz(c.r[29].wrapping_add(c.r[5]));
                c.r[6] = c.lwz(c.r[30].wrapping_add(c.r[0]));
                c.r[7] = c.lbz(c.r[28].wrapping_add(0x18));
                c.lr = 0x800d7ae4; f_800b7d74(c);
                pc = 0x800d7ae4;
            }
            0x800d7ae4 => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                c.r[27] = c.r[27].wrapping_add(0x4);
                c.cmps(0, c.r[26], 0xe);
                c.r[28] = c.r[28].wrapping_add(0x1c);
                if c.crbit(0) { pc = 0x800d7aa8; continue; }
                pc = 0x800d7af8;
            }
            0x800d7af8 => {
                c.lr = 0x800d7afc; f_800b7d3c(c);
                pc = 0x800d7afc;
            }
            0x800d7afc => {
                c.r[3] = 0x80310000;
                c.r[27] = c.r[25] | c.r[25];
                c.r[28] = 0x0;
                c.r[30] = c.r[3].wrapping_add(0x43a8);
                pc = 0x800d7b0c;
            }
            0x800d7b0c => {
                c.r[0] = c.lbz(c.r[27].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d7b44; continue; }
                pc = 0x800d7b18;
            }
            0x800d7b18 => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0xc));
                c.r[3] = c.lwz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[4] = c.lwz(c.r[30].wrapping_add(c.r[0]));
                c.lr = 0x800d7b2c; f_800b7874(c);
                pc = 0x800d7b2c;
            }
            0x800d7b2c => {
                c.r[4] = c.lwz(c.r[27].wrapping_add(0x1c));
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x800d7b44; continue; }
                pc = 0x800d7b38;
            }
            0x800d7b38 => {
                c.r[3] = c.lwz(c.r[31]);
                c.r[5] = c.lbz(c.r[27].wrapping_add(0x20));
                c.lr = 0x800d7b44; f_800b84e0(c);
                pc = 0x800d7b44;
            }
            0x800d7b44 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x4);
                c.cmps(0, c.r[28], 0xe);
                c.r[27] = c.r[27].wrapping_add(0x1c);
                if c.crbit(0) { pc = 0x800d7b0c; continue; }
                pc = 0x800d7b58;
            }
            0x800d7b58 => {
                c.r[3] = 0x80400000;
                c.r[4] = c.r[3].wrapping_add(0x1f0);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x14));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[4].wrapping_add(0x14), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7a70: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7B80 (0x800d7b80, 120 bytes)
#[inline(never)]
pub fn f_800d7b80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7b80) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7b80); f_800d7b80_b(c, 0x800d7b80); crate::prof::exit(c, 0x800d7b80); return; }
    }
    f_800d7b80_b(c, 0x800d7b80)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7b80_at(c: &mut Cpu, pc: u32) { f_800d7b80_b(c, pc) }
#[inline(always)]
fn f_800d7b80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7b80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7b80, pc); }
        match pc {
            0x800d7b80 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[0].wrapping_mul(0x190);
                c.stw(c.r[13].wrapping_add(0xffffa290), c.r[0]);
                c.lr = 0x800d7b9c; f_800e3534(c);
                pc = 0x800d7b9c;
            }
            0x800d7b9c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.sth(c.r[13].wrapping_add(0xffffa288), c.r[3]);
                if c.crbit(2) { pc = 0x800d7be8; continue; }
                pc = 0x800d7ba8;
            }
            0x800d7ba8 => {
                c.r[3] = c.r[0] | c.r[0];
                c.lr = 0x800d7bb0; f_800e27b0(c);
                pc = 0x800d7bb0;
            }
            0x800d7bb0 => {
                c.r[5] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa28c), c.r[3]);
                c.r[4] = c.r[5] | c.r[5];
                c.r[6] = 0x0;
                pc = 0x800d7bd4; continue;
                pc = 0x800d7bc4;
            }
            0x800d7bc4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa28c));
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.stb(c.r[3].wrapping_add(c.r[5]), c.r[4]);
                c.r[5] = c.r[5].wrapping_add(0x190);
                pc = 0x800d7bd4;
            }
            0x800d7bd4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa290));
                c.cmpu(0, c.r[6], c.r[0]);
                if c.crbit(0) { pc = 0x800d7bc4; continue; }
                pc = 0x800d7be0;
            }
            0x800d7be0 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa294), c.r[0]);
                pc = 0x800d7be8;
            }
            0x800d7be8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7b80: bad pc {:#x}", pc),
        }
    }
}

/// gfxGetCameraMatrix (0x800d7bf8, 124 bytes)
#[inline(never)]
pub fn f_800d7bf8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7bf8) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7bf8); f_800d7bf8_b(c, 0x800d7bf8); crate::prof::exit(c, 0x800d7bf8); return; }
    }
    f_800d7bf8_b(c, 0x800d7bf8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7bf8_at(c: &mut Cpu, pc: u32) { f_800d7bf8_b(c, pc) }
#[inline(always)]
fn f_800d7bf8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7bf8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7bf8, pc); }
        match pc {
            0x800d7bf8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x800d7c10; f_800d2584(c);
                pc = 0x800d7c10;
            }
            0x800d7c10 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x800d7c20; continue; }
                pc = 0x800d7c18;
            }
            0x800d7c18 => {
                c.r[3] = 0x0;
                pc = 0x800d7c60; continue;
                pc = 0x800d7c20;
            }
            0x800d7c20 => {
                c.cmps(0, c.r[31], 0x1);
                if c.crbit(2) { pc = 0x800d7c4c; continue; }
                pc = 0x800d7c28;
            }
            0x800d7c28 => {
                if !c.crbit(0) { pc = 0x800d7c38; continue; }
                pc = 0x800d7c2c;
            }
            0x800d7c2c => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(0) { pc = 0x800d7c44; continue; }
                pc = 0x800d7c34;
            }
            0x800d7c34 => {
                pc = 0x800d7c5c; continue;
                pc = 0x800d7c38;
            }
            0x800d7c38 => {
                c.cmps(0, c.r[31], 0x3);
                if !c.crbit(0) { pc = 0x800d7c5c; continue; }
                pc = 0x800d7c40;
            }
            0x800d7c40 => {
                pc = 0x800d7c54; continue;
                pc = 0x800d7c44;
            }
            0x800d7c44 => {
                c.lr = 0x800d7c48; f_800d1d00(c);
                pc = 0x800d7c48;
            }
            0x800d7c48 => {
                pc = 0x800d7c60; continue;
                pc = 0x800d7c4c;
            }
            0x800d7c4c => {
                c.lr = 0x800d7c50; f_800d1b3c(c);
                pc = 0x800d7c50;
            }
            0x800d7c50 => {
                pc = 0x800d7c60; continue;
                pc = 0x800d7c54;
            }
            0x800d7c54 => {
                c.lr = 0x800d7c58; f_800d1a70(c);
                pc = 0x800d7c58;
            }
            0x800d7c58 => {
                pc = 0x800d7c60; continue;
                pc = 0x800d7c5c;
            }
            0x800d7c5c => {
                c.r[3] = 0x0;
                pc = 0x800d7c60;
            }
            0x800d7c60 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7bf8: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7C74 (0x800d7c74, 156 bytes)
#[inline(never)]
pub fn f_800d7c74(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7c74) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7c74); f_800d7c74_b(c, 0x800d7c74); crate::prof::exit(c, 0x800d7c74); return; }
    }
    f_800d7c74_b(c, 0x800d7c74)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7c74_at(c: &mut Cpu, pc: u32) { f_800d7c74_b(c, pc) }
#[inline(always)]
fn f_800d7c74_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7c74;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7c74, pc); }
        match pc {
            0x800d7c74 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d7ca8; continue; }
                pc = 0x800d7c94;
            }
            0x800d7c94 => {
                c.r[3] = 0x45;
                c.r[4] = 0x0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d7ca4; f_800d4f98(c);
                pc = 0x800d7ca4;
            }
            0x800d7ca4 => {
                pc = 0x800d7cfc; continue;
                pc = 0x800d7ca8;
            }
            0x800d7ca8 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa2a8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x800d7cfc; continue; }
                pc = 0x800d7cb4;
            }
            0x800d7cb4 => {
                c.r[4] = 0x80310000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[4].wrapping_add(0x4610);
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d7ccc; f_800bd4b4(c);
                pc = 0x800d7ccc;
            }
            0x800d7ccc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d7cd8; f_800bd504(c);
                pc = 0x800d7cd8;
            }
            0x800d7cd8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800d7ce0; f_800bd554(c);
                pc = 0x800d7ce0;
            }
            0x800d7ce0 => {
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[3] = c.r[3].wrapping_add(0x948);
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                c.lr = 0x800d7cf4; f_800e0628(c);
                pc = 0x800d7cf4;
            }
            0x800d7cf4 => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa2a8), c.r[0]);
                pc = 0x800d7cfc;
            }
            0x800d7cfc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7c74: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7D10 (0x800d7d10, 128 bytes)
#[inline(never)]
pub fn f_800d7d10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7d10) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7d10); f_800d7d10_b(c, 0x800d7d10); crate::prof::exit(c, 0x800d7d10); return; }
    }
    f_800d7d10_b(c, 0x800d7d10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7d10_at(c: &mut Cpu, pc: u32) { f_800d7d10_b(c, pc) }
#[inline(always)]
fn f_800d7d10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7d10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7d10, pc); }
        match pc {
            0x800d7d10 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d7d48; continue; }
                pc = 0x800d7d2c;
            }
            0x800d7d2c => {
                c.r[6] = c.r[4] | c.r[4];
                c.r[5] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x44;
                c.r[4] = 0x2;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d7d44; f_800d4f98(c);
                pc = 0x800d7d44;
            }
            0x800d7d44 => {
                pc = 0x800d7d80; continue;
                pc = 0x800d7d48;
            }
            0x800d7d48 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x9);
                if !c.crbit(1) { pc = 0x800d7d68; continue; }
                pc = 0x800d7d54;
            }
            0x800d7d54 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x440);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d7d64; stubs_gen::f_800dd970(c);
                pc = 0x800d7d64;
            }
            0x800d7d64 => {
                pc = 0x800d7d80; continue;
                pc = 0x800d7d68;
            }
            0x800d7d68 => {
                c.r[6] = c.r[0].wrapping_mul(0x30);
                c.r[5] = 0x80400000;
                c.r[3] = c.r[4] | c.r[4];
                c.r[0] = c.r[5].wrapping_add(0x948);
                c.r[4] = c.r[0].wrapping_add(c.r[6]);
                c.lr = 0x800d7d80; f_800e0628(c);
                pc = 0x800d7d80;
            }
            0x800d7d80 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7d10: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7D90 (0x800d7d90, 204 bytes)
#[inline(never)]
pub fn f_800d7d90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7d90) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7d90); f_800d7d90_b(c, 0x800d7d90); crate::prof::exit(c, 0x800d7d90); return; }
    }
    f_800d7d90_b(c, 0x800d7d90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7d90_at(c: &mut Cpu, pc: u32) { f_800d7d90_b(c, pc) }
#[inline(always)]
fn f_800d7d90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7d90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7d90, pc); }
        match pc {
            0x800d7d90 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[5]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d7ddc; continue; }
                pc = 0x800d7dc0;
            }
            0x800d7dc0 => {
                c.r[6] = c.r[30] | c.r[30];
                c.r[5] = c.r[29].rotate_left(0) & 0xff;
                c.r[3] = 0x43;
                c.r[4] = 0x11;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d7dd8; f_800d4f98(c);
                pc = 0x800d7dd8;
            }
            0x800d7dd8 => {
                pc = 0x800d7e40; continue;
                pc = 0x800d7ddc;
            }
            0x800d7ddc => {
                c.r[31] = c.r[29].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[31], 0x9);
                if !c.crbit(1) { pc = 0x800d7dfc; continue; }
                pc = 0x800d7de8;
            }
            0x800d7de8 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x440);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d7df8; stubs_gen::f_800dd970(c);
                pc = 0x800d7df8;
            }
            0x800d7df8 => {
                pc = 0x800d7e40; continue;
                pc = 0x800d7dfc;
            }
            0x800d7dfc => {
                c.r[3] = 0x80310000;
                c.r[0] = c.r[29].rotate_left(2) & 0x3fc;
                c.r[4] = c.r[3].wrapping_add(0x4610);
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                c.lr = 0x800d7e14; f_800bd4b4(c);
                pc = 0x800d7e14;
            }
            0x800d7e14 => {
                c.r[5] = c.r[31].wrapping_mul(0x30);
                c.r[3] = 0x80400000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[0] = c.r[3].wrapping_add(0x948);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                c.lr = 0x800d7e2c; f_800e0628(c);
                pc = 0x800d7e2c;
            }
            0x800d7e2c => {
                c.r[0] = c.r[31] | c.r[31];
                c.cmpu(0, c.r[0], 0x9);
                if !c.crbit(2) { pc = 0x800d7e40; continue; }
                pc = 0x800d7e38;
            }
            0x800d7e38 => {
                c.r[0] = 0x1;
                c.stb(c.r[13].wrapping_add(0xffffa2a8), c.r[0]);
                pc = 0x800d7e40;
            }
            0x800d7e40 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7d90: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7E5C (0x800d7e5c, 184 bytes)
#[inline(never)]
pub fn f_800d7e5c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7e5c) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7e5c); f_800d7e5c_b(c, 0x800d7e5c); crate::prof::exit(c, 0x800d7e5c); return; }
    }
    f_800d7e5c_b(c, 0x800d7e5c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7e5c_at(c: &mut Cpu, pc: u32) { f_800d7e5c_b(c, pc) }
#[inline(always)]
fn f_800d7e5c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7e5c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7e5c, pc); }
        match pc {
            0x800d7e5c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d7e90; continue; }
                pc = 0x800d7e7c;
            }
            0x800d7e7c => {
                c.r[3] = 0x42;
                c.r[4] = 0x0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d7e8c; f_800d4f98(c);
                pc = 0x800d7e8c;
            }
            0x800d7e8c => {
                pc = 0x800d7f00; continue;
                pc = 0x800d7e90;
            }
            0x800d7e90 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa2a4));
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x800d7eb4; continue; }
                pc = 0x800d7ea0;
            }
            0x800d7ea0 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x460);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d7eb0; stubs_gen::f_800dd970(c);
                pc = 0x800d7eb0;
            }
            0x800d7eb0 => {
                pc = 0x800d7f00; continue;
                pc = 0x800d7eb4;
            }
            0x800d7eb4 => {
                c.r[4] = 0x80310000;
                c.r[3] = c.r[3].wrapping_add(0x30);
                c.r[4] = c.r[4].wrapping_add(0x4610);
                c.stw(c.r[13].wrapping_add(0xffffa2a0), c.r[3]);
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d7ed0; f_800bd4b4(c);
                pc = 0x800d7ed0;
            }
            0x800d7ed0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d7edc; f_800bd504(c);
                pc = 0x800d7edc;
            }
            0x800d7edc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800d7ee4; f_800bd554(c);
                pc = 0x800d7ee4;
            }
            0x800d7ee4 => {
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[3] = c.r[3].wrapping_add(0x948);
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                c.lr = 0x800d7ef8; f_800e0628(c);
                pc = 0x800d7ef8;
            }
            0x800d7ef8 => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa2a8), c.r[0]);
                pc = 0x800d7f00;
            }
            0x800d7f00 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7e5c: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7F14 (0x800d7f14, 208 bytes)
#[inline(never)]
pub fn f_800d7f14(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7f14) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7f14); f_800d7f14_b(c, 0x800d7f14); crate::prof::exit(c, 0x800d7f14); return; }
    }
    f_800d7f14_b(c, 0x800d7f14)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7f14_at(c: &mut Cpu, pc: u32) { f_800d7f14_b(c, pc) }
#[inline(always)]
fn f_800d7f14_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7f14;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7f14, pc); }
        match pc {
            0x800d7f14 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d7f4c; continue; }
                pc = 0x800d7f34;
            }
            0x800d7f34 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = 0x41;
                c.r[4] = 0x10;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d7f48; f_800d4f98(c);
                pc = 0x800d7f48;
            }
            0x800d7f48 => {
                pc = 0x800d7fd0; continue;
                pc = 0x800d7f4c;
            }
            0x800d7f4c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa29c));
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(1) { pc = 0x800d7f70; continue; }
                pc = 0x800d7f5c;
            }
            0x800d7f5c => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x480);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d7f6c; stubs_gen::f_800dd970(c);
                pc = 0x800d7f6c;
            }
            0x800d7f6c => {
                pc = 0x800d7fd0; continue;
                pc = 0x800d7f70;
            }
            0x800d7f70 => {
                c.r[4] = c.r[4].wrapping_add(0xffffffd0);
                c.r[5] = c.r[3] | c.r[3];
                c.stw(c.r[13].wrapping_add(0xffffa2a0), c.r[4]);
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = c.r[4].wrapping_add(0x30);
                c.lr = 0x800d7f88; f_800e0290(c);
                pc = 0x800d7f88;
            }
            0x800d7f88 => {
                c.r[4] = 0x80310000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[4].wrapping_add(0x4610);
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d7fa0; f_800bd4b4(c);
                pc = 0x800d7fa0;
            }
            0x800d7fa0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d7fac; f_800bd504(c);
                pc = 0x800d7fac;
            }
            0x800d7fac => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800d7fb4; f_800bd554(c);
                pc = 0x800d7fb4;
            }
            0x800d7fb4 => {
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[3] = c.r[3].wrapping_add(0x948);
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                c.lr = 0x800d7fc8; f_800e0628(c);
                pc = 0x800d7fc8;
            }
            0x800d7fc8 => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa2a8), c.r[0]);
                pc = 0x800d7fd0;
            }
            0x800d7fd0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7f14: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D7FE4 (0x800d7fe4, 164 bytes)
#[inline(never)]
pub fn f_800d7fe4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d7fe4) { return; }
        if c.prof { crate::prof::enter(c, 0x800d7fe4); f_800d7fe4_b(c, 0x800d7fe4); crate::prof::exit(c, 0x800d7fe4); return; }
    }
    f_800d7fe4_b(c, 0x800d7fe4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d7fe4_at(c: &mut Cpu, pc: u32) { f_800d7fe4_b(c, pc) }
#[inline(always)]
fn f_800d7fe4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d7fe4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d7fe4, pc); }
        match pc {
            0x800d7fe4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d801c; continue; }
                pc = 0x800d8004;
            }
            0x800d8004 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = 0x40;
                c.r[4] = 0x10;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d8018; f_800d4f98(c);
                pc = 0x800d8018;
            }
            0x800d8018 => {
                pc = 0x800d8074; continue;
                pc = 0x800d801c;
            }
            0x800d801c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[4] | c.r[4];
                c.lr = 0x800d802c; f_800e0290(c);
                pc = 0x800d802c;
            }
            0x800d802c => {
                c.r[4] = 0x80310000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[4].wrapping_add(0x4610);
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d8044; f_800bd4b4(c);
                pc = 0x800d8044;
            }
            0x800d8044 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d8050; f_800bd504(c);
                pc = 0x800d8050;
            }
            0x800d8050 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800d8058; f_800bd554(c);
                pc = 0x800d8058;
            }
            0x800d8058 => {
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[3] = c.r[3].wrapping_add(0x948);
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                c.lr = 0x800d806c; f_800e0628(c);
                pc = 0x800d806c;
            }
            0x800d806c => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa2a8), c.r[0]);
                pc = 0x800d8074;
            }
            0x800d8074 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d7fe4: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D8088 (0x800d8088, 204 bytes)
#[inline(never)]
pub fn f_800d8088(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d8088) { return; }
        if c.prof { crate::prof::enter(c, 0x800d8088); f_800d8088_b(c, 0x800d8088); crate::prof::exit(c, 0x800d8088); return; }
    }
    f_800d8088_b(c, 0x800d8088)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d8088_at(c: &mut Cpu, pc: u32) { f_800d8088_b(c, pc) }
#[inline(always)]
fn f_800d8088_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d8088;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d8088, pc); }
        match pc {
            0x800d8088 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[4]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d80c0; continue; }
                pc = 0x800d80a8;
            }
            0x800d80a8 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = 0x3f;
                c.r[4] = 0x10;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d80bc; f_800d4f98(c);
                pc = 0x800d80bc;
            }
            0x800d80bc => {
                pc = 0x800d8140; continue;
                pc = 0x800d80c0;
            }
            0x800d80c0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa29c));
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(1) { pc = 0x800d80e4; continue; }
                pc = 0x800d80d0;
            }
            0x800d80d0 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x480);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x800d80e0; stubs_gen::f_800dd970(c);
                pc = 0x800d80e0;
            }
            0x800d80e0 => {
                pc = 0x800d8140; continue;
                pc = 0x800d80e4;
            }
            0x800d80e4 => {
                c.r[0] = c.r[4].wrapping_add(0xffffffd0);
                c.r[4] = c.r[3] | c.r[3];
                c.stw(c.r[13].wrapping_add(0xffffa2a0), c.r[0]);
                c.r[3] = c.r[0] | c.r[0];
                c.lr = 0x800d80f8; f_800e0628(c);
                pc = 0x800d80f8;
            }
            0x800d80f8 => {
                c.r[4] = 0x80310000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[4].wrapping_add(0x4610);
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d8110; f_800bd4b4(c);
                pc = 0x800d8110;
            }
            0x800d8110 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d811c; f_800bd504(c);
                pc = 0x800d811c;
            }
            0x800d811c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800d8124; f_800bd554(c);
                pc = 0x800d8124;
            }
            0x800d8124 => {
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[3] = c.r[3].wrapping_add(0x948);
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                c.lr = 0x800d8138; f_800e0628(c);
                pc = 0x800d8138;
            }
            0x800d8138 => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa2a8), c.r[0]);
                pc = 0x800d8140;
            }
            0x800d8140 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d8088: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D8154 (0x800d8154, 152 bytes)
#[inline(never)]
pub fn f_800d8154(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d8154) { return; }
        if c.prof { crate::prof::enter(c, 0x800d8154); f_800d8154_b(c, 0x800d8154); crate::prof::exit(c, 0x800d8154); return; }
    }
    f_800d8154_b(c, 0x800d8154)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d8154_at(c: &mut Cpu, pc: u32) { f_800d8154_b(c, pc) }
#[inline(always)]
fn f_800d8154_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d8154;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d8154, pc); }
        match pc {
            0x800d8154 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d8188; continue; }
                pc = 0x800d8174;
            }
            0x800d8174 => {
                c.r[3] = 0x3e;
                c.r[4] = 0xd;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d8184; f_800d4f98(c);
                pc = 0x800d8184;
            }
            0x800d8184 => {
                pc = 0x800d81d8; continue;
                pc = 0x800d8188;
            }
            0x800d8188 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.lr = 0x800d8190; f_800e02c4(c);
                pc = 0x800d8190;
            }
            0x800d8190 => {
                c.r[4] = 0x80310000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[4].wrapping_add(0x4610);
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d81a8; f_800bd4b4(c);
                pc = 0x800d81a8;
            }
            0x800d81a8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d81b4; f_800bd504(c);
                pc = 0x800d81b4;
            }
            0x800d81b4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800d81bc; f_800bd554(c);
                pc = 0x800d81bc;
            }
            0x800d81bc => {
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[3] = c.r[3].wrapping_add(0x948);
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                c.lr = 0x800d81d0; f_800e0628(c);
                pc = 0x800d81d0;
            }
            0x800d81d0 => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa2a8), c.r[0]);
                pc = 0x800d81d8;
            }
            0x800d81d8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d8154: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D81EC (0x800d81ec, 152 bytes)
#[inline(never)]
pub fn f_800d81ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d81ec) { return; }
        if c.prof { crate::prof::enter(c, 0x800d81ec); f_800d81ec_b(c, 0x800d81ec); crate::prof::exit(c, 0x800d81ec); return; }
    }
    f_800d81ec_b(c, 0x800d81ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d81ec_at(c: &mut Cpu, pc: u32) { f_800d81ec_b(c, pc) }
#[inline(always)]
fn f_800d81ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d81ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d81ec, pc); }
        match pc {
            0x800d81ec => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d8220; continue; }
                pc = 0x800d820c;
            }
            0x800d820c => {
                c.r[3] = 0x3d;
                c.r[4] = 0xd;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d821c; f_800d4f98(c);
                pc = 0x800d821c;
            }
            0x800d821c => {
                pc = 0x800d8270; continue;
                pc = 0x800d8220;
            }
            0x800d8220 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.lr = 0x800d8228; f_800e03e8(c);
                pc = 0x800d8228;
            }
            0x800d8228 => {
                c.r[4] = 0x80310000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[4].wrapping_add(0x4610);
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d8240; f_800bd4b4(c);
                pc = 0x800d8240;
            }
            0x800d8240 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d824c; f_800bd504(c);
                pc = 0x800d824c;
            }
            0x800d824c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800d8254; f_800bd554(c);
                pc = 0x800d8254;
            }
            0x800d8254 => {
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[3] = c.r[3].wrapping_add(0x948);
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                c.lr = 0x800d8268; f_800e0628(c);
                pc = 0x800d8268;
            }
            0x800d8268 => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa2a8), c.r[0]);
                pc = 0x800d8270;
            }
            0x800d8270 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_800d81ec: bad pc {:#x}", pc),
        }
    }
}

/// fn_800D8284 (0x800d8284, 200 bytes)
#[inline(never)]
pub fn f_800d8284(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x800d8284) { return; }
        if c.prof { crate::prof::enter(c, 0x800d8284); f_800d8284_b(c, 0x800d8284); crate::prof::exit(c, 0x800d8284); return; }
    }
    f_800d8284_b(c, 0x800d8284)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_800d8284_at(c: &mut Cpu, pc: u32) { f_800d8284_b(c, pc) }
#[inline(always)]
fn f_800d8284_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x800d8284;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x800d8284, pc); }
        match pc {
            0x800d8284 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[31]; c.stfd(ea, v); }
                c.f[31] = c.f[3];
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[30]; c.stfd(ea, v); }
                c.f[30] = c.f[2];
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa260));
                c.r[0] = c.lwz(c.r[3]);
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x800d82c8; continue; }
                pc = 0x800d82b4;
            }
            0x800d82b4 => {
                c.r[3] = 0x3c;
                c.r[4] = 0xd;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, !(a ^ b)); }
                c.lr = 0x800d82c4; f_800d4f98(c);
                pc = 0x800d82c4;
            }
            0x800d82c4 => {
                pc = 0x800d8330; continue;
                pc = 0x800d82c8;
            }
            0x800d82c8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.lr = 0x800d82d0; f_800e0370(c);
                pc = 0x800d82d0;
            }
            0x800d82d0 => {
                c.f[1] = c.f[30];
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.lr = 0x800d82dc; f_800e032c(c);
                pc = 0x800d82dc;
            }
            0x800d82dc => {
                c.f[1] = c.f[31];
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.lr = 0x800d82e8; f_800e02e8(c);
                pc = 0x800d82e8;
            }
            0x800d82e8 => {
                c.r[4] = 0x80310000;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[4].wrapping_add(0x4610);
                c.r[31] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d8300; f_800bd4b4(c);
                pc = 0x800d8300;
            }
            0x800d8300 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x800d830c; f_800bd504(c);
                pc = 0x800d830c;
            }
            0x800d830c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x800d8314; f_800bd554(c);
                pc = 0x800d8314;
            }
            0x800d8314 => {
                c.r[3] = 0x80400000;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa2a0));
                c.r[3] = c.r[3].wrapping_add(0x948);
                c.r[3] = c.r[3].wrapping_add(0x1b0);
                c.lr = 0x800d8328; f_800e0628(c);
                pc = 0x800d8328;
            }
            0x800d8328 => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa2a8), c.r[0]);
                pc = 0x800d8330;
            }
            0x800d8330 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                { let ea = c.r[1].wrapping_add(0x18); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x10); c.f[30] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_800d8284: bad pc {:#x}", pc),
        }
    }
}

