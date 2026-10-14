#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// std246_saveSetStatus578 (0x801140dc, 144 bytes)
#[inline(never)]
pub fn f_801140dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801140dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801140dc); f_801140dc_b(c, 0x801140dc); crate::prof::exit(c, 0x801140dc); return; }
    }
    f_801140dc_b(c, 0x801140dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801140dc_at(c: &mut Cpu, pc: u32) { f_801140dc_b(c, pc) }
#[inline(always)]
fn f_801140dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801140dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801140dc, pc); }
        match pc {
            0x801140dc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.lr = 0x80114108; f_80128e24(c);
                pc = 0x80114108;
            }
            0x80114108 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8011414c; continue; }
                pc = 0x80114110;
            }
            0x80114110 => {
                c.lr = 0x80114114; f_80128e04(c);
                pc = 0x80114114;
            }
            0x80114114 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8011414c; continue; }
                pc = 0x80114120;
            }
            0x80114120 => {
                c.r[5] = c.r[28] | c.r[28];
                c.r[4] = 0x5;
                c.lr = 0x8011412c; f_80135030(c);
                pc = 0x8011412c;
            }
            0x8011412c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[29] | c.r[29];
                c.r[4] = 0x7;
                c.lr = 0x8011413c; f_80135030(c);
                pc = 0x8011413c;
            }
            0x8011413c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[5] = c.r[30].rotate_left(0) & 0xff;
                c.r[4] = 0x8;
                c.lr = 0x8011414c; f_80135030(c);
                pc = 0x8011414c;
            }
            0x8011414c => {
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
            _ => panic!("f_801140dc: bad pc {:#x}", pc),
        }
    }
}

/// std161_fieldSetPending (0x8011416c, 32 bytes)
#[inline(never)]
pub fn f_8011416c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011416c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011416c); f_8011416c_b(c, 0x8011416c); crate::prof::exit(c, 0x8011416c); return; }
    }
    f_8011416c_b(c, 0x8011416c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011416c_at(c: &mut Cpu, pc: u32) { f_8011416c_b(c, pc) }
#[inline(always)]
fn f_8011416c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011416c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011416c, pc); }
        match pc {
            0x8011416c => {
                c.r[6] = 0x80410000;
                c.r[0] = 0x1;
                c.r[6] = c.r[6].wrapping_add(0xffff8378);
                c.stw(c.r[6].wrapping_add(0x48), c.r[3]);
                c.stw(c.r[6].wrapping_add(0x4c), c.r[4]);
                c.stb(c.r[6].wrapping_add(0x50), c.r[5]);
                c.stb(c.r[6].wrapping_add(0x51), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011416c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011418C (0x8011418c, 76 bytes)
#[inline(never)]
pub fn f_8011418c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011418c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011418c); f_8011418c_b(c, 0x8011418c); crate::prof::exit(c, 0x8011418c); return; }
    }
    f_8011418c_b(c, 0x8011418c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011418c_at(c: &mut Cpu, pc: u32) { f_8011418c_b(c, pc) }
#[inline(always)]
fn f_8011418c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011418c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011418c, pc); }
        match pc {
            0x8011418c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801141a4; continue; }
                pc = 0x80114194;
            }
            0x80114194 => {
                c.r[6] = 0x80410000;
                c.r[6] = c.r[6].wrapping_add(0xffff8378);
                c.r[0] = c.lwz(c.r[6].wrapping_add(0x3c));
                c.stw(c.r[3], c.r[0]);
                pc = 0x801141a4;
            }
            0x801141a4 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x801141bc; continue; }
                pc = 0x801141ac;
            }
            0x801141ac => {
                c.r[3] = 0x80410000;
                c.r[3] = c.r[3].wrapping_add(0xffff8378);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x40));
                c.stw(c.r[4], c.r[0]);
                pc = 0x801141bc;
            }
            0x801141bc => {
                c.cmpu(0, c.r[5], 0x0);
                if c.crbit(2) { return; }
                pc = 0x801141c4;
            }
            0x801141c4 => {
                c.r[3] = 0x80410000;
                c.r[3] = c.r[3].wrapping_add(0xffff8378);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x44));
                c.stb(c.r[5], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011418c: bad pc {:#x}", pc),
        }
    }
}

/// std128_mapSetReturnMap (0x801141d8, 32 bytes)
#[inline(never)]
pub fn f_801141d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801141d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801141d8); f_801141d8_b(c, 0x801141d8); crate::prof::exit(c, 0x801141d8); return; }
    }
    f_801141d8_b(c, 0x801141d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801141d8_at(c: &mut Cpu, pc: u32) { f_801141d8_b(c, pc) }
#[inline(always)]
fn f_801141d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801141d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801141d8, pc); }
        match pc {
            0x801141d8 => {
                c.r[6] = 0x80410000;
                c.r[0] = 0x0;
                c.r[6] = c.r[6].wrapping_add(0xffff8378);
                c.stw(c.r[6].wrapping_add(0x3c), c.r[3]);
                c.stw(c.r[6].wrapping_add(0x40), c.r[4]);
                c.stb(c.r[6].wrapping_add(0x44), c.r[5]);
                c.stb(c.r[6].wrapping_add(0x51), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_801141d8: bad pc {:#x}", pc),
        }
    }
}

/// EvlogSet__FScUl (0x801141f8, 92 bytes)
#[inline(never)]
pub fn f_801141f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801141f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801141f8); f_801141f8_b(c, 0x801141f8); crate::prof::exit(c, 0x801141f8); return; }
    }
    f_801141f8_b(c, 0x801141f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801141f8_at(c: &mut Cpu, pc: u32) { f_801141f8_b(c, pc) }
#[inline(always)]
fn f_801141f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801141f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801141f8, pc); }
        match pc {
            0x801141f8 => {
                c.r[6] = c.lbz(c.r[13].wrapping_add(0xffffa540));
                c.r[5] = 0x66660000;
                c.r[8] = 0x80360000;
                c.r[7] = c.r[6].wrapping_add(0x1);
                c.r[9] = c.r[6] as u8 as i8 as i32 as u32;
                c.r[6] = c.r[7].rotate_left(0) & 0xff;
                c.r[0] = c.r[5].wrapping_add(0x6667);
                c.r[5] = c.r[6] as u8 as i8 as i32 as u32;
                c.r[9] = c.r[9].rotate_left(3) & 0xfffffff8;
                c.r[0] = mulhw(c.r[0], c.r[5]);
                c.r[6] = c.r[8].wrapping_add(0xffffb818);
                c.stb(c.r[6].wrapping_add(c.r[9]), c.r[3]);
                c.r[6] = c.r[6].wrapping_add(c.r[9]);
                c.stb(c.r[13].wrapping_add(0xffffa540), c.r[7]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 30 != 0); c.r[0] = (v >> 2) as u32; }
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.stw(c.r[6].wrapping_add(0x4), c.r[4]);
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.r[0].wrapping_mul(0xa);
                c.r[0] = c.r[5].wrapping_sub(c.r[0]);
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.stb(c.r[13].wrapping_add(0xffffa540), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_801141f8: bad pc {:#x}", pc),
        }
    }
}

/// floorReadRelModulePostFunc (0x80114254, 96 bytes)
#[inline(never)]
pub fn f_80114254(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114254) { return; }
        if c.prof { crate::prof::enter(c, 0x80114254); f_80114254_b(c, 0x80114254); crate::prof::exit(c, 0x80114254); return; }
    }
    f_80114254_b(c, 0x80114254)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114254_at(c: &mut Cpu, pc: u32) { f_80114254_b(c, pc) }
#[inline(always)]
fn f_80114254_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114254;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114254, pc); }
        match pc {
            0x80114254 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.lr = 0x8011427c; f_800f9318(c);
                pc = 0x8011427c;
            }
            0x8011427c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.r[0] | c.r[0];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x80114294; f_8017f484(c);
                pc = 0x80114294;
            }
            0x80114294 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80114254: bad pc {:#x}", pc),
        }
    }
}

/// floorReadRelModulePreFunc (0x801142b4, 68 bytes)
#[inline(never)]
pub fn f_801142b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801142b4) { return; }
        if c.prof { crate::prof::enter(c, 0x801142b4); f_801142b4_b(c, 0x801142b4); crate::prof::exit(c, 0x801142b4); return; }
    }
    f_801142b4_b(c, 0x801142b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801142b4_at(c: &mut Cpu, pc: u32) { f_801142b4_b(c, pc) }
#[inline(always)]
fn f_801142b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801142b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801142b4, pc); }
        match pc {
            0x801142b4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4] | c.r[4];
                c.r[4] = 0x20;
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[7] = 0x0;
                c.lr = 0x801142dc; f_800f9418(c);
                pc = 0x801142dc;
            }
            0x801142dc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801142e8; continue; }
                pc = 0x801142e4;
            }
            0x801142e4 => {
                c.r[3] = 0x0;
                pc = 0x801142e8;
            }
            0x801142e8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801142b4: bad pc {:#x}", pc),
        }
    }
}

/// floorReadGFLPostFunc (0x801142f8, 52 bytes)
#[inline(never)]
pub fn f_801142f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801142f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801142f8); f_801142f8_b(c, 0x801142f8); crate::prof::exit(c, 0x801142f8); return; }
    }
    f_801142f8_b(c, 0x801142f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801142f8_at(c: &mut Cpu, pc: u32) { f_801142f8_b(c, pc) }
#[inline(always)]
fn f_801142f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801142f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801142f8, pc); }
        match pc {
            0x801142f8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x8011430c; f_800f9318(c);
                pc = 0x8011430c;
            }
            0x8011430c => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80114314; f_801ed680(c);
                pc = 0x80114314;
            }
            0x80114314 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801142f8: bad pc {:#x}", pc),
        }
    }
}

/// floorReadGFLPreFunc (0x8011432c, 116 bytes)
#[inline(never)]
pub fn f_8011432c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011432c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011432c); f_8011432c_b(c, 0x8011432c); crate::prof::exit(c, 0x8011432c); return; }
    }
    f_8011432c_b(c, 0x8011432c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011432c_at(c: &mut Cpu, pc: u32) { f_8011432c_b(c, pc) }
#[inline(always)]
fn f_8011432c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011432c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011432c, pc); }
        match pc {
            0x8011432c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.r[5] = 0x80110000;
                c.r[0] = c.r[30].wrapping_add(0x1f);
                c.r[7] = c.r[5].wrapping_add(0x5094);
                c.r[6] = c.r[4] | c.r[4];
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[4] = 0x20;
                c.lr = 0x80114364; f_800f9418(c);
                pc = 0x80114364;
            }
            0x80114364 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80114384; continue; }
                pc = 0x80114370;
            }
            0x80114370 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x2200);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114384; stubs_gen::f_800dd970(c);
                pc = 0x80114384;
            }
            0x80114384 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011432c: bad pc {:#x}", pc),
        }
    }
}

/// floorReadBGMPostFunc (0x801143a0, 8 bytes)
#[inline(never)]
pub fn f_801143a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801143a0) { return; }
        if c.prof { crate::prof::enter(c, 0x801143a0); f_801143a0_b(c, 0x801143a0); crate::prof::exit(c, 0x801143a0); return; }
    }
    f_801143a0_b(c, 0x801143a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801143a0_at(c: &mut Cpu, pc: u32) { f_801143a0_b(c, pc) }
#[inline(always)]
fn f_801143a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801143a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801143a0, pc); }
        match pc {
            0x801143a0 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_801143a0: bad pc {:#x}", pc),
        }
    }
}

/// floorReadBGMPreFunc (0x801143a8, 68 bytes)
#[inline(never)]
pub fn f_801143a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801143a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801143a8); f_801143a8_b(c, 0x801143a8); crate::prof::exit(c, 0x801143a8); return; }
    }
    f_801143a8_b(c, 0x801143a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801143a8_at(c: &mut Cpu, pc: u32) { f_801143a8_b(c, pc) }
#[inline(always)]
fn f_801143a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801143a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801143a8, pc); }
        match pc {
            0x801143a8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa890));
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.cmpu(0, c.r[0], c.r[5]);
                if !c.crbit(1) { pc = 0x801143d8; continue; }
                pc = 0x801143c8;
            }
            0x801143c8 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x2238);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801143d8; stubs_gen::f_800dd970(c);
                pc = 0x801143d8;
            }
            0x801143d8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa894));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801143a8: bad pc {:#x}", pc),
        }
    }
}

/// floorReadNotLinkedParticlePostFunc (0x801143ec, 112 bytes)
#[inline(never)]
pub fn f_801143ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801143ec) { return; }
        if c.prof { crate::prof::enter(c, 0x801143ec); f_801143ec_b(c, 0x801143ec); crate::prof::exit(c, 0x801143ec); return; }
    }
    f_801143ec_b(c, 0x801143ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801143ec_at(c: &mut Cpu, pc: u32) { f_801143ec_b(c, pc) }
#[inline(always)]
fn f_801143ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801143ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801143ec, pc); }
        match pc {
            0x801143ec => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[0] = c.r[30].rotate_left(0) & 0x7fff0000;
                c.r[4] = c.r[0] | 0x400;
                c.lr = 0x80114418; f_800f9318(c);
                pc = 0x80114418;
            }
            0x80114418 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80114420; f_801195ac(c);
                pc = 0x80114420;
            }
            0x80114420 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8011443c; continue; }
                pc = 0x80114428;
            }
            0x80114428 => {
                c.r[5] = 0x80110000;
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[5].wrapping_add(0x50b8);
                c.r[5] = c.r[30] | c.r[30];
                c.lr = 0x8011443c; f_800f9378(c);
                pc = 0x8011443c;
            }
            0x8011443c => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801143ec: bad pc {:#x}", pc),
        }
    }
}

/// floorReadNotLinkedParticlePreFunc (0x8011445c, 116 bytes)
#[inline(never)]
pub fn f_8011445c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011445c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011445c); f_8011445c_b(c, 0x8011445c); crate::prof::exit(c, 0x8011445c); return; }
    }
    f_8011445c_b(c, 0x8011445c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011445c_at(c: &mut Cpu, pc: u32) { f_8011445c_b(c, pc) }
#[inline(always)]
fn f_8011445c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011445c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011445c, pc); }
        match pc {
            0x8011445c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[5] = c.r[5].wrapping_add(0x1f);
                c.r[0] = c.r[4].rotate_left(0) & 0x7fff0000;
                c.r[30] = c.r[5].rotate_left(0) & 0xffffffe0;
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[0] | 0x400;
                c.r[4] = 0x20;
                c.r[7] = 0x0;
                c.lr = 0x80114494; f_800f9418(c);
                pc = 0x80114494;
            }
            0x80114494 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x801144b4; continue; }
                pc = 0x801144a0;
            }
            0x801144a0 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x2270);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801144b4; stubs_gen::f_800dd970(c);
                pc = 0x801144b4;
            }
            0x801144b4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011445c: bad pc {:#x}", pc),
        }
    }
}

/// floorReadParticlePostFunc (0x801144d0, 240 bytes)
#[inline(never)]
pub fn f_801144d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801144d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801144d0); f_801144d0_b(c, 0x801144d0); crate::prof::exit(c, 0x801144d0); return; }
    }
    f_801144d0_b(c, 0x801144d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801144d0_at(c: &mut Cpu, pc: u32) { f_801144d0_b(c, pc) }
#[inline(always)]
fn f_801144d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801144d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801144d0, pc); }
        match pc {
            0x801144d0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[0] = c.r[28].rotate_left(0) & 0x7fff0000;
                c.r[31] = 0x0;
                c.r[4] = c.r[0] | 0x400;
                c.lr = 0x801144f8; f_800f9318(c);
                pc = 0x801144f8;
            }
            0x801144f8 => {
                c.r[27] = c.r[3] | c.r[3];
                c.lr = 0x80114500; f_801195ac(c);
                pc = 0x80114500;
            }
            0x80114500 => {
                c.r[26] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[26], 0x0);
                if c.crbit(2) { pc = 0x801145a8; continue; }
                pc = 0x8011450c;
            }
            0x8011450c => {
                c.r[5] = 0x80110000;
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[5].wrapping_add(0x50b8);
                c.r[5] = c.r[28] | c.r[28];
                c.lr = 0x80114520; f_800f9378(c);
                pc = 0x80114520;
            }
            0x80114520 => {
                c.lr = 0x80114524; f_80115bd8(c);
                pc = 0x80114524;
            }
            0x80114524 => {
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x8011452c; f_80113f48(c);
                pc = 0x8011452c;
            }
            0x8011452c => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x8));
                c.lr = 0x80114534; f_800f9318(c);
                pc = 0x80114534;
            }
            0x80114534 => {
                c.r[4] = 0x80270000;
                c.r[4] = c.r[4].wrapping_add(0x22ac);
                c.lr = 0x80114540; f_80191ecc(c);
                pc = 0x80114540;
            }
            0x80114540 => {
                c.r[30] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80114554; continue; }
                pc = 0x8011454c;
            }
            0x8011454c => {
                c.r[3] = 0x0;
                pc = 0x801145ac; continue;
                pc = 0x80114554;
            }
            0x80114554 => {
                c.r[0] = c.lwz(c.r[30]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801145a8; continue; }
                pc = 0x80114560;
            }
            0x80114560 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                c.r[28] = 0x0;
                c.r[0] = c.r[0].rotate_left(0) & 0x7fff0000;
                c.r[29] = c.r[0] | 0x1000;
                pc = 0x80114598; continue;
                pc = 0x80114574;
            }
            0x80114574 => {
                c.lr = 0x80114578; f_80113f48(c);
                pc = 0x80114578;
            }
            0x80114578 => {
                c.r[4] = c.r[29] | c.r[31];
                c.lr = 0x80114580; f_800f9318(c);
                pc = 0x80114580;
            }
            0x80114580 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80114590; continue; }
                pc = 0x80114588;
            }
            0x80114588 => {
                c.r[4] = c.r[26] | c.r[26];
                c.lr = 0x80114590; f_800e3c08(c);
                pc = 0x80114590;
            }
            0x80114590 => {
                c.r[28] = c.r[28].wrapping_add(0x4);
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80114598;
            }
            0x80114598 => {
                c.r[3] = c.lwz(c.r[30]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[28]));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80114574; continue; }
                pc = 0x801145a8;
            }
            0x801145a8 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x801145ac;
            }
            0x801145ac => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801144d0: bad pc {:#x}", pc),
        }
    }
}

/// floorReadParticlePreFunc (0x801145c0, 116 bytes)
#[inline(never)]
pub fn f_801145c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801145c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801145c0); f_801145c0_b(c, 0x801145c0); crate::prof::exit(c, 0x801145c0); return; }
    }
    f_801145c0_b(c, 0x801145c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801145c0_at(c: &mut Cpu, pc: u32) { f_801145c0_b(c, pc) }
#[inline(always)]
fn f_801145c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801145c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801145c0, pc); }
        match pc {
            0x801145c0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[5] = c.r[5].wrapping_add(0x1f);
                c.r[0] = c.r[4].rotate_left(0) & 0x7fff0000;
                c.r[30] = c.r[5].rotate_left(0) & 0xffffffe0;
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[6] = c.r[0] | 0x400;
                c.r[4] = 0x20;
                c.r[7] = 0x0;
                c.lr = 0x801145f8; f_800f9418(c);
                pc = 0x801145f8;
            }
            0x801145f8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80114618; continue; }
                pc = 0x80114604;
            }
            0x80114604 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x2270);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114618; stubs_gen::f_800dd970(c);
                pc = 0x80114618;
            }
            0x80114618 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801145c0: bad pc {:#x}", pc),
        }
    }
}

/// floorReadWZXPreFunc (0x80114634, 112 bytes)
#[inline(never)]
pub fn f_80114634(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114634) { return; }
        if c.prof { crate::prof::enter(c, 0x80114634); f_80114634_b(c, 0x80114634); crate::prof::exit(c, 0x80114634); return; }
    }
    f_80114634_b(c, 0x80114634)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114634_at(c: &mut Cpu, pc: u32) { f_80114634_b(c, pc) }
#[inline(always)]
fn f_80114634_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114634;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114634, pc); }
        match pc {
            0x80114634 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[5] = c.r[3] | c.r[3];
                c.r[30] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[6] = c.r[4] | c.r[4];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x20;
                c.r[7] = 0x0;
                c.lr = 0x80114668; f_800f9418(c);
                pc = 0x80114668;
            }
            0x80114668 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80114688; continue; }
                pc = 0x80114674;
            }
            0x80114674 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x22b8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114688; stubs_gen::f_800dd970(c);
                pc = 0x80114688;
            }
            0x80114688 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80114634: bad pc {:#x}", pc),
        }
    }
}

/// floorReadPKXPreFunc (0x801146a4, 112 bytes)
#[inline(never)]
pub fn f_801146a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801146a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801146a4); f_801146a4_b(c, 0x801146a4); crate::prof::exit(c, 0x801146a4); return; }
    }
    f_801146a4_b(c, 0x801146a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801146a4_at(c: &mut Cpu, pc: u32) { f_801146a4_b(c, pc) }
#[inline(always)]
fn f_801146a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801146a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801146a4, pc); }
        match pc {
            0x801146a4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[5] = c.r[3] | c.r[3];
                c.r[30] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[6] = c.r[4] | c.r[4];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x20;
                c.r[7] = 0x0;
                c.lr = 0x801146d8; f_800f9418(c);
                pc = 0x801146d8;
            }
            0x801146d8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x801146f8; continue; }
                pc = 0x801146e4;
            }
            0x801146e4 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x22f0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801146f8; stubs_gen::f_800dd970(c);
                pc = 0x801146f8;
            }
            0x801146f8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801146a4: bad pc {:#x}", pc),
        }
    }
}

/// floorReadTexPostFunc (0x80114714, 76 bytes)
#[inline(never)]
pub fn f_80114714(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114714) { return; }
        if c.prof { crate::prof::enter(c, 0x80114714); f_80114714_b(c, 0x80114714); crate::prof::exit(c, 0x80114714); return; }
    }
    f_80114714_b(c, 0x80114714)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114714_at(c: &mut Cpu, pc: u32) { f_80114714_b(c, pc) }
#[inline(always)]
fn f_80114714_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114714;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114714, pc); }
        match pc {
            0x80114714 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80114728; f_800f9318(c);
                pc = 0x80114728;
            }
            0x80114728 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80114730; f_800efd3c(c);
                pc = 0x80114730;
            }
            0x80114730 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80114748; continue; }
                pc = 0x80114738;
            }
            0x80114738 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x2328);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114748; stubs_gen::f_800dd970(c);
                pc = 0x80114748;
            }
            0x80114748 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80114714: bad pc {:#x}", pc),
        }
    }
}

/// floorReadTexPreFunc (0x80114760, 116 bytes)
#[inline(never)]
pub fn f_80114760(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114760) { return; }
        if c.prof { crate::prof::enter(c, 0x80114760); f_80114760_b(c, 0x80114760); crate::prof::exit(c, 0x80114760); return; }
    }
    f_80114760_b(c, 0x80114760)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114760_at(c: &mut Cpu, pc: u32) { f_80114760_b(c, pc) }
#[inline(always)]
fn f_80114760_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114760;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114760, pc); }
        match pc {
            0x80114760 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[6] = 0x80110000;
                c.r[30] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[5] = c.r[3] | c.r[3];
                c.r[7] = c.r[6].wrapping_add(0x522c);
                c.r[6] = c.r[4] | c.r[4];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x20;
                c.lr = 0x80114798; f_800f9418(c);
                pc = 0x80114798;
            }
            0x80114798 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x801147b8; continue; }
                pc = 0x801147a4;
            }
            0x801147a4 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x235c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801147b8; stubs_gen::f_800dd970(c);
                pc = 0x801147b8;
            }
            0x801147b8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80114760: bad pc {:#x}", pc),
        }
    }
}

/// floorReadColPostFunc (0x801147d4, 52 bytes)
#[inline(never)]
pub fn f_801147d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801147d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801147d4); f_801147d4_b(c, 0x801147d4); crate::prof::exit(c, 0x801147d4); return; }
    }
    f_801147d4_b(c, 0x801147d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801147d4_at(c: &mut Cpu, pc: u32) { f_801147d4_b(c, pc) }
#[inline(always)]
fn f_801147d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801147d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801147d4, pc); }
        match pc {
            0x801147d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x801147e8; f_800f9318(c);
                pc = 0x801147e8;
            }
            0x801147e8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801147f0; f_8010cfe4(c);
                pc = 0x801147f0;
            }
            0x801147f0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801147d4: bad pc {:#x}", pc),
        }
    }
}

/// floorReadColPreFunc (0x80114808, 116 bytes)
#[inline(never)]
pub fn f_80114808(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114808) { return; }
        if c.prof { crate::prof::enter(c, 0x80114808); f_80114808_b(c, 0x80114808); crate::prof::exit(c, 0x80114808); return; }
    }
    f_80114808_b(c, 0x80114808)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114808_at(c: &mut Cpu, pc: u32) { f_80114808_b(c, pc) }
#[inline(always)]
fn f_80114808_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114808;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114808, pc); }
        match pc {
            0x80114808 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[6] = 0x80110000;
                c.r[30] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[5] = c.r[3] | c.r[3];
                c.r[7] = c.r[6].wrapping_add(0x5208);
                c.r[6] = c.r[4] | c.r[4];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x20;
                c.lr = 0x80114840; f_800f9418(c);
                pc = 0x80114840;
            }
            0x80114840 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80114860; continue; }
                pc = 0x8011484c;
            }
            0x8011484c => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x2394);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114860; stubs_gen::f_800dd970(c);
                pc = 0x80114860;
            }
            0x80114860 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80114808: bad pc {:#x}", pc),
        }
    }
}

/// floorReadCameraPostFunc (0x8011487c, 204 bytes)
#[inline(never)]
pub fn f_8011487c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011487c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011487c); f_8011487c_b(c, 0x8011487c); crate::prof::exit(c, 0x8011487c); return; }
    }
    f_8011487c_b(c, 0x8011487c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011487c_at(c: &mut Cpu, pc: u32) { f_8011487c_b(c, pc) }
#[inline(always)]
fn f_8011487c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011487c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011487c, pc); }
        match pc {
            0x8011487c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[0] = c.r[29].rotate_left(0) & 0x7fff0000;
                c.r[4] = 0x80270000;
                c.r[31] = c.r[4].wrapping_add(0x2200);
                c.r[4] = c.r[0] | 0x400;
                c.lr = 0x801148b8; f_800f9318(c);
                pc = 0x801148b8;
            }
            0x801148b8 => {
                c.r[5] = c.r[30] | c.r[30];
                c.r[30] = c.r[3] | c.r[3];
                c.r[4] = c.r[30].wrapping_add(0x60);
                c.lr = 0x801148c8; f_80191f64(c);
                pc = 0x801148c8;
            }
            0x801148c8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31].wrapping_add(0xac);
                c.lr = 0x801148d4; f_80191ecc(c);
                pc = 0x801148d4;
            }
            0x801148d4 => {
                c.r[30] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x801148f0; continue; }
                pc = 0x801148e0;
            }
            0x801148e0 => {
                c.r[3] = c.r[31].wrapping_add(0x1cc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801148ec; stubs_gen::f_800dd970(c);
                pc = 0x801148ec;
            }
            0x801148ec => {
                pc = 0x80114924; continue;
                pc = 0x801148f0;
            }
            0x801148f0 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x4));
                c.lr = 0x801148f8; f_800d27fc(c);
                pc = 0x801148f8;
            }
            0x801148f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80114910; continue; }
                pc = 0x80114900;
            }
            0x80114900 => {
                c.r[3] = c.r[31].wrapping_add(0x200);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8011490c; stubs_gen::f_800dd970(c);
                pc = 0x8011490c;
            }
            0x8011490c => {
                pc = 0x80114924; continue;
                pc = 0x80114910;
            }
            0x80114910 => {
                c.r[5] = 0x80110000;
                c.r[4] = c.r[28] | c.r[28];
                c.r[6] = c.r[5].wrapping_add(0x50dc);
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x80114924; f_800f9378(c);
                pc = 0x80114924;
            }
            0x80114924 => {
                c.r[3] = c.r[30] | c.r[30];
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
            _ => panic!("f_8011487c: bad pc {:#x}", pc),
        }
    }
}

/// floorReadCameraPreFunc (0x80114948, 116 bytes)
#[inline(never)]
pub fn f_80114948(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114948) { return; }
        if c.prof { crate::prof::enter(c, 0x80114948); f_80114948_b(c, 0x80114948); crate::prof::exit(c, 0x80114948); return; }
    }
    f_80114948_b(c, 0x80114948)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114948_at(c: &mut Cpu, pc: u32) { f_80114948_b(c, pc) }
#[inline(always)]
fn f_80114948_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114948;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114948, pc); }
        match pc {
            0x80114948 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[5] = c.r[5].wrapping_add(0x1f);
                c.r[0] = c.r[4].rotate_left(0) & 0x7fff0000;
                c.r[4] = c.r[5].rotate_left(0) & 0xffffffe0;
                c.r[5] = c.r[3] | c.r[3];
                c.r[31] = c.r[4].wrapping_add(0x60);
                c.r[6] = c.r[0] | 0x400;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x20;
                c.r[7] = 0x0;
                c.lr = 0x80114980; f_800f9418(c);
                pc = 0x80114980;
            }
            0x80114980 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801149a4; continue; }
                pc = 0x80114988;
            }
            0x80114988 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[3].wrapping_add(0x2428);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8011499c; stubs_gen::f_800dd970(c);
                pc = 0x8011499c;
            }
            0x8011499c => {
                c.r[3] = 0x0;
                pc = 0x801149a8; continue;
                pc = 0x801149a4;
            }
            0x801149a4 => {
                c.r[3] = c.r[3].wrapping_add(0x60);
                pc = 0x801149a8;
            }
            0x801149a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80114948: bad pc {:#x}", pc),
        }
    }
}

/// floorReadObjPostFunc (0x801149bc, 180 bytes)
#[inline(never)]
pub fn f_801149bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801149bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801149bc); f_801149bc_b(c, 0x801149bc); crate::prof::exit(c, 0x801149bc); return; }
    }
    f_801149bc_b(c, 0x801149bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801149bc_at(c: &mut Cpu, pc: u32) { f_801149bc_b(c, pc) }
#[inline(always)]
fn f_801149bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801149bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801149bc, pc); }
        match pc {
            0x801149bc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[28] = 0x0;
                c.lr = 0x801149e0; f_800f9318(c);
                pc = 0x801149e0;
            }
            0x801149e0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = c.r[29] | c.r[29];
                c.r[4] = c.r[31].wrapping_add(0x60);
                c.lr = 0x801149f0; f_80191f64(c);
                pc = 0x801149f0;
            }
            0x801149f0 => {
                c.r[4] = 0x80270000;
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[4].wrapping_add(0x22ac);
                c.lr = 0x80114a00; f_80191ecc(c);
                pc = 0x80114a00;
            }
            0x80114a00 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80114a14; continue; }
                pc = 0x80114a0c;
            }
            0x80114a0c => {
                c.r[3] = 0x0;
                pc = 0x80114a5c; continue;
                pc = 0x80114a14;
            }
            0x80114a14 => {
                c.r[0] = c.lwz(c.r[31]);
                c.r[3] = c.r[30].rotate_left(0) & 0x7fff0000;
                c.r[30] = c.r[3] | 0x1000;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80114a58; continue; }
                pc = 0x80114a28;
            }
            0x80114a28 => {
                c.r[29] = 0x0;
                pc = 0x80114a48; continue;
                pc = 0x80114a30;
            }
            0x80114a30 => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[30] | c.r[28];
                c.r[6] = 0x0;
                c.lr = 0x80114a40; f_800f9378(c);
                pc = 0x80114a40;
            }
            0x80114a40 => {
                c.r[29] = c.r[29].wrapping_add(0x4);
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x80114a48;
            }
            0x80114a48 => {
                c.r[3] = c.lwz(c.r[31]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[29]));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80114a30; continue; }
                pc = 0x80114a58;
            }
            0x80114a58 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80114a5c;
            }
            0x80114a5c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801149bc: bad pc {:#x}", pc),
        }
    }
}

/// floorReadObjPreFunc (0x80114a70, 112 bytes)
#[inline(never)]
pub fn f_80114a70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114a70) { return; }
        if c.prof { crate::prof::enter(c, 0x80114a70); f_80114a70_b(c, 0x80114a70); crate::prof::exit(c, 0x80114a70); return; }
    }
    f_80114a70_b(c, 0x80114a70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114a70_at(c: &mut Cpu, pc: u32) { f_80114a70_b(c, pc) }
#[inline(always)]
fn f_80114a70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114a70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114a70, pc); }
        match pc {
            0x80114a70 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[6] = c.r[4] | c.r[4];
                c.r[31] = c.r[3].wrapping_add(0x60);
                c.r[4] = 0x20;
                c.r[3] = c.r[31] | c.r[31];
                c.r[7] = 0x0;
                c.lr = 0x80114aa4; f_800f9418(c);
                pc = 0x80114aa4;
            }
            0x80114aa4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80114ac8; continue; }
                pc = 0x80114aac;
            }
            0x80114aac => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[3].wrapping_add(0x2460);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114ac0; stubs_gen::f_800dd970(c);
                pc = 0x80114ac0;
            }
            0x80114ac0 => {
                c.r[3] = 0x0;
                pc = 0x80114acc; continue;
                pc = 0x80114ac8;
            }
            0x80114ac8 => {
                c.r[3] = c.r[3].wrapping_add(0x60);
                pc = 0x80114acc;
            }
            0x80114acc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80114a70: bad pc {:#x}", pc),
        }
    }
}

/// floorReadMapPostFunc (0x80114ae0, 456 bytes)
#[inline(never)]
pub fn f_80114ae0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114ae0) { return; }
        if c.prof { crate::prof::enter(c, 0x80114ae0); f_80114ae0_b(c, 0x80114ae0); crate::prof::exit(c, 0x80114ae0); return; }
    }
    f_80114ae0_b(c, 0x80114ae0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114ae0_at(c: &mut Cpu, pc: u32) { f_80114ae0_b(c, pc) }
#[inline(always)]
fn f_80114ae0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114ae0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114ae0, pc); }
        match pc {
            0x80114ae0 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.r[25] = c.r[5] | c.r[5];
                c.r[5] = 0x80270000;
                c.r[24] = 0x0;
                c.r[27] = c.r[5].wrapping_add(0x2200);
                c.r[31] = 0x0;
                c.lr = 0x80114b10; f_800f9318(c);
                pc = 0x80114b10;
            }
            0x80114b10 => {
                c.r[26] = c.r[3] | c.r[3];
                c.r[5] = c.r[25] | c.r[25];
                c.r[4] = c.r[26].wrapping_add(0x60);
                c.lr = 0x80114b20; f_80191f64(c);
                pc = 0x80114b20;
            }
            0x80114b20 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27].wrapping_add(0xac);
                c.lr = 0x80114b2c; f_80191ecc(c);
                pc = 0x80114b2c;
            }
            0x80114b2c => {
                c.r[26] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[26], 0x0);
                if !c.crbit(2) { pc = 0x80114b40; continue; }
                pc = 0x80114b38;
            }
            0x80114b38 => {
                c.r[3] = 0x0;
                pc = 0x80114c94; continue;
                pc = 0x80114b40;
            }
            0x80114b40 => {
                c.lr = 0x80114b44; f_80115bd8(c);
                pc = 0x80114b44;
            }
            0x80114b44 => {
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80114b4c; f_80115584(c);
                pc = 0x80114b4c;
            }
            0x80114b4c => {
                c.r[0] = c.lwz(c.r[26]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80114b90; continue; }
                pc = 0x80114b58;
            }
            0x80114b58 => {
                c.r[0] = c.r[30].rotate_left(0) & 0x7fff0000;
                c.r[25] = 0x0;
                c.r[28] = c.r[0] | 0x1000;
                pc = 0x80114b80; continue;
                pc = 0x80114b68;
            }
            0x80114b68 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[28] | c.r[24];
                c.r[6] = 0x0;
                c.lr = 0x80114b78; f_800f9378(c);
                pc = 0x80114b78;
            }
            0x80114b78 => {
                c.r[25] = c.r[25].wrapping_add(0x4);
                c.r[24] = c.r[24].wrapping_add(0x1);
                pc = 0x80114b80;
            }
            0x80114b80 => {
                c.r[3] = c.lwz(c.r[26]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[25]));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80114b68; continue; }
                pc = 0x80114b90;
            }
            0x80114b90 => {
                c.r[0] = c.lwz(c.r[26].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80114c08; continue; }
                pc = 0x80114b9c;
            }
            0x80114b9c => {
                c.r[0] = c.r[30].rotate_left(0) & 0x7fff0000;
                c.r[24] = 0x0;
                c.r[25] = c.r[0] | 0x1600;
                c.r[3] = 0x80110000;
                c.r[28] = c.r[3].wrapping_add(0x5100);
                pc = 0x80114bf8; continue;
                pc = 0x80114bb4;
            }
            0x80114bb4 => {
                c.lr = 0x80114bb8; f_800dce4c(c);
                pc = 0x80114bb8;
            }
            0x80114bb8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80114be0; continue; }
                pc = 0x80114bc0;
            }
            0x80114bc0 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[27].wrapping_add(0x298);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114bd0; stubs_gen::f_800dd970(c);
                pc = 0x80114bd0;
            }
            0x80114bd0 => {
                c.r[3] = c.r[27].wrapping_add(0x2d0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114bdc; stubs_gen::f_800dd970(c);
                pc = 0x80114bdc;
            }
            0x80114bdc => {
                pc = 0x80114bf0; continue;
                pc = 0x80114be0;
            }
            0x80114be0 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[28] | c.r[28];
                c.r[5] = c.r[25] | c.r[31];
                c.lr = 0x80114bf0; f_800f9378(c);
                pc = 0x80114bf0;
            }
            0x80114bf0 => {
                c.r[24] = c.r[24].wrapping_add(0x4);
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80114bf8;
            }
            0x80114bf8 => {
                c.r[3] = c.lwz(c.r[26].wrapping_add(0x8));
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[24]));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80114bb4; continue; }
                pc = 0x80114c08;
            }
            0x80114c08 => {
                c.cmpu(0, c.r[26], 0x0);
                c.r[24] = c.r[30].rotate_left(0) & 0x7fff0000;
                c.r[25] = c.r[24] | 0x1800;
                if !c.crbit(2) { pc = 0x80114c28; continue; }
                pc = 0x80114c18;
            }
            0x80114c18 => {
                c.r[3] = c.r[27].wrapping_add(0x1cc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114c24; stubs_gen::f_800dd970(c);
                pc = 0x80114c24;
            }
            0x80114c24 => {
                pc = 0x80114c5c; continue;
                pc = 0x80114c28;
            }
            0x80114c28 => {
                c.r[3] = c.lwz(c.r[26].wrapping_add(0x4));
                c.lr = 0x80114c30; f_800d27fc(c);
                pc = 0x80114c30;
            }
            0x80114c30 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80114c48; continue; }
                pc = 0x80114c38;
            }
            0x80114c38 => {
                c.r[3] = c.r[27].wrapping_add(0x200);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114c44; stubs_gen::f_800dd970(c);
                pc = 0x80114c44;
            }
            0x80114c44 => {
                pc = 0x80114c5c; continue;
                pc = 0x80114c48;
            }
            0x80114c48 => {
                c.r[5] = 0x80110000;
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[5].wrapping_add(0x50dc);
                c.r[5] = c.r[25] | c.r[25];
                c.lr = 0x80114c5c; f_800f9378(c);
                pc = 0x80114c5c;
            }
            0x80114c5c => {
                c.r[3] = c.lwz(c.r[26].wrapping_add(0xc));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80114c84; continue; }
                pc = 0x80114c68;
            }
            0x80114c68 => {
                c.r[3] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80114c84; continue; }
                pc = 0x80114c74;
            }
            0x80114c74 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[24] | 0x1a00;
                c.r[6] = 0x0;
                c.lr = 0x80114c84; f_800f9378(c);
                pc = 0x80114c84;
            }
            0x80114c84 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80114c90; f_801134e4(c);
                pc = 0x80114c90;
            }
            0x80114c90 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x80114c94;
            }
            0x80114c94 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80114ae0: bad pc {:#x}", pc),
        }
    }
}

/// floorReadMapPreFunc (0x80114ca8, 112 bytes)
#[inline(never)]
pub fn f_80114ca8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114ca8) { return; }
        if c.prof { crate::prof::enter(c, 0x80114ca8); f_80114ca8_b(c, 0x80114ca8); crate::prof::exit(c, 0x80114ca8); return; }
    }
    f_80114ca8_b(c, 0x80114ca8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114ca8_at(c: &mut Cpu, pc: u32) { f_80114ca8_b(c, pc) }
#[inline(always)]
fn f_80114ca8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114ca8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114ca8, pc); }
        match pc {
            0x80114ca8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[6] = c.r[4] | c.r[4];
                c.r[31] = c.r[3].wrapping_add(0x60);
                c.r[4] = 0x20;
                c.r[3] = c.r[31] | c.r[31];
                c.r[7] = 0x0;
                c.lr = 0x80114cdc; f_800f9418(c);
                pc = 0x80114cdc;
            }
            0x80114cdc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80114d00; continue; }
                pc = 0x80114ce4;
            }
            0x80114ce4 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[3].wrapping_add(0x24e8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114cf8; stubs_gen::f_800dd970(c);
                pc = 0x80114cf8;
            }
            0x80114cf8 => {
                c.r[3] = 0x0;
                pc = 0x80114d04; continue;
                pc = 0x80114d00;
            }
            0x80114d00 => {
                c.r[3] = c.r[3].wrapping_add(0x60);
                pc = 0x80114d04;
            }
            0x80114d04 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80114ca8: bad pc {:#x}", pc),
        }
    }
}

/// floorReadScriptPostFunc (0x80114d18, 84 bytes)
#[inline(never)]
pub fn f_80114d18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114d18) { return; }
        if c.prof { crate::prof::enter(c, 0x80114d18); f_80114d18_b(c, 0x80114d18); crate::prof::exit(c, 0x80114d18); return; }
    }
    f_80114d18_b(c, 0x80114d18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114d18_at(c: &mut Cpu, pc: u32) { f_80114d18_b(c, pc) }
#[inline(always)]
fn f_80114d18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114d18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114d18, pc); }
        match pc {
            0x80114d18 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80114d2c; f_800f9318(c);
                pc = 0x80114d2c;
            }
            0x80114d2c => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80114d34; f_800ff548(c);
                pc = 0x80114d34;
            }
            0x80114d34 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80114d54; continue; }
                pc = 0x80114d40;
            }
            0x80114d40 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80114d54; continue; }
                pc = 0x80114d48;
            }
            0x80114d48 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80114d50; f_800f76e4(c);
                pc = 0x80114d50;
            }
            0x80114d50 => {
                c.lr = 0x80114d54; f_80112700(c);
                pc = 0x80114d54;
            }
            0x80114d54 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80114d18: bad pc {:#x}", pc),
        }
    }
}

/// floorReadScriptPreFunc (0x80114d6c, 160 bytes)
#[inline(never)]
pub fn f_80114d6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114d6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80114d6c); f_80114d6c_b(c, 0x80114d6c); crate::prof::exit(c, 0x80114d6c); return; }
    }
    f_80114d6c_b(c, 0x80114d6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114d6c_at(c: &mut Cpu, pc: u32) { f_80114d6c_b(c, pc) }
#[inline(always)]
fn f_80114d6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114d6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114d6c, pc); }
        match pc {
            0x80114d6c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.lr = 0x80114d94; f_800ff548(c);
                pc = 0x80114d94;
            }
            0x80114d94 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80114da8; continue; }
                pc = 0x80114da0;
            }
            0x80114da0 => {
                c.r[3] = 0x0;
                pc = 0x80114df0; continue;
                pc = 0x80114da8;
            }
            0x80114da8 => {
                c.r[0] = c.r[30].wrapping_add(0x1f);
                c.r[3] = 0x80110000;
                c.r[30] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[5] = c.r[29] | c.r[29];
                c.r[7] = c.r[3].wrapping_add(0x5124);
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x20;
                c.lr = 0x80114dcc; f_800f9418(c);
                pc = 0x80114dcc;
            }
            0x80114dcc => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80114dec; continue; }
                pc = 0x80114dd8;
            }
            0x80114dd8 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x2520);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114dec; stubs_gen::f_800dd970(c);
                pc = 0x80114dec;
            }
            0x80114dec => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80114df0;
            }
            0x80114df0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80114d6c: bad pc {:#x}", pc),
        }
    }
}

/// floorReadFontPostFunc (0x80114e0c, 108 bytes)
#[inline(never)]
pub fn f_80114e0c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114e0c) { return; }
        if c.prof { crate::prof::enter(c, 0x80114e0c); f_80114e0c_b(c, 0x80114e0c); crate::prof::exit(c, 0x80114e0c); return; }
    }
    f_80114e0c_b(c, 0x80114e0c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114e0c_at(c: &mut Cpu, pc: u32) { f_80114e0c_b(c, pc) }
#[inline(always)]
fn f_80114e0c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114e0c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114e0c, pc); }
        match pc {
            0x80114e0c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x80114e2c; f_800ff548(c);
                pc = 0x80114e2c;
            }
            0x80114e2c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80114e40; continue; }
                pc = 0x80114e38;
            }
            0x80114e38 => {
                c.r[3] = 0x0;
                pc = 0x80114e60; continue;
                pc = 0x80114e40;
            }
            0x80114e40 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80114e4c; f_800f9318(c);
                pc = 0x80114e4c;
            }
            0x80114e4c => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80114e5c; continue; }
                pc = 0x80114e58;
            }
            0x80114e58 => {
                c.lr = 0x80114e5c; f_800fc39c(c);
                pc = 0x80114e5c;
            }
            0x80114e5c => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80114e60;
            }
            0x80114e60 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80114e0c: bad pc {:#x}", pc),
        }
    }
}

/// floorReadFontPreFunc (0x80114e78, 160 bytes)
#[inline(never)]
pub fn f_80114e78(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114e78) { return; }
        if c.prof { crate::prof::enter(c, 0x80114e78); f_80114e78_b(c, 0x80114e78); crate::prof::exit(c, 0x80114e78); return; }
    }
    f_80114e78_b(c, 0x80114e78)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114e78_at(c: &mut Cpu, pc: u32) { f_80114e78_b(c, pc) }
#[inline(always)]
fn f_80114e78_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114e78;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114e78, pc); }
        match pc {
            0x80114e78 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.lr = 0x80114ea0; f_800ff548(c);
                pc = 0x80114ea0;
            }
            0x80114ea0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80114eb4; continue; }
                pc = 0x80114eac;
            }
            0x80114eac => {
                c.r[3] = 0x0;
                pc = 0x80114efc; continue;
                pc = 0x80114eb4;
            }
            0x80114eb4 => {
                c.r[0] = c.r[30].wrapping_add(0x1f);
                c.r[3] = 0x80110000;
                c.r[30] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[5] = c.r[29] | c.r[29];
                c.r[7] = c.r[3].wrapping_add(0x5170);
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x20;
                c.lr = 0x80114ed8; f_800f9418(c);
                pc = 0x80114ed8;
            }
            0x80114ed8 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80114ef8; continue; }
                pc = 0x80114ee4;
            }
            0x80114ee4 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x255c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80114ef8; stubs_gen::f_800dd970(c);
                pc = 0x80114ef8;
            }
            0x80114ef8 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80114efc;
            }
            0x80114efc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80114e78: bad pc {:#x}", pc),
        }
    }
}

/// floorReadMsgPostFunc (0x80114f18, 108 bytes)
#[inline(never)]
pub fn f_80114f18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114f18) { return; }
        if c.prof { crate::prof::enter(c, 0x80114f18); f_80114f18_b(c, 0x80114f18); crate::prof::exit(c, 0x80114f18); return; }
    }
    f_80114f18_b(c, 0x80114f18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114f18_at(c: &mut Cpu, pc: u32) { f_80114f18_b(c, pc) }
#[inline(always)]
fn f_80114f18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114f18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114f18, pc); }
        match pc {
            0x80114f18 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.lr = 0x80114f38; f_800ff548(c);
                pc = 0x80114f38;
            }
            0x80114f38 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80114f4c; continue; }
                pc = 0x80114f44;
            }
            0x80114f44 => {
                c.r[3] = 0x0;
                pc = 0x80114f6c; continue;
                pc = 0x80114f4c;
            }
            0x80114f4c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80114f58; f_800f9318(c);
                pc = 0x80114f58;
            }
            0x80114f58 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80114f68; continue; }
                pc = 0x80114f64;
            }
            0x80114f64 => {
                c.lr = 0x80114f68; f_800fc244(c);
                pc = 0x80114f68;
            }
            0x80114f68 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80114f6c;
            }
            0x80114f6c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80114f18: bad pc {:#x}", pc),
        }
    }
}

/// floorReadMsgPreFunc (0x80114f84, 160 bytes)
#[inline(never)]
pub fn f_80114f84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80114f84) { return; }
        if c.prof { crate::prof::enter(c, 0x80114f84); f_80114f84_b(c, 0x80114f84); crate::prof::exit(c, 0x80114f84); return; }
    }
    f_80114f84_b(c, 0x80114f84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80114f84_at(c: &mut Cpu, pc: u32) { f_80114f84_b(c, pc) }
#[inline(always)]
fn f_80114f84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80114f84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80114f84, pc); }
        match pc {
            0x80114f84 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.lr = 0x80114fac; f_800ff548(c);
                pc = 0x80114fac;
            }
            0x80114fac => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80114fc0; continue; }
                pc = 0x80114fb8;
            }
            0x80114fb8 => {
                c.r[3] = 0x0;
                pc = 0x80115008; continue;
                pc = 0x80114fc0;
            }
            0x80114fc0 => {
                c.r[0] = c.r[30].wrapping_add(0x1f);
                c.r[3] = 0x80110000;
                c.r[30] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[5] = c.r[29] | c.r[29];
                c.r[7] = c.r[3].wrapping_add(0x51bc);
                c.r[6] = c.r[31] | c.r[31];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x20;
                c.lr = 0x80114fe4; f_800f9418(c);
                pc = 0x80114fe4;
            }
            0x80114fe4 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80115004; continue; }
                pc = 0x80114ff0;
            }
            0x80114ff0 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x2594);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115004; stubs_gen::f_800dd970(c);
                pc = 0x80115004;
            }
            0x80115004 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80115008;
            }
            0x80115008 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80114f84: bad pc {:#x}", pc),
        }
    }
}

/// floorReadNormalPreFunc (0x80115024, 112 bytes)
#[inline(never)]
pub fn f_80115024(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115024) { return; }
        if c.prof { crate::prof::enter(c, 0x80115024); f_80115024_b(c, 0x80115024); crate::prof::exit(c, 0x80115024); return; }
    }
    f_80115024_b(c, 0x80115024)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115024_at(c: &mut Cpu, pc: u32) { f_80115024_b(c, pc) }
#[inline(always)]
fn f_80115024_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115024;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115024, pc); }
        match pc {
            0x80115024 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.r[0] = c.r[30].wrapping_add(0x1f);
                c.r[5] = c.r[3] | c.r[3];
                c.r[6] = c.r[4] | c.r[4];
                c.r[4] = 0x20;
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[7] = 0x0;
                c.lr = 0x80115058; f_800f9418(c);
                pc = 0x80115058;
            }
            0x80115058 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80115078; continue; }
                pc = 0x80115064;
            }
            0x80115064 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = c.r[3].wrapping_add(0x25cc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115078; stubs_gen::f_800dd970(c);
                pc = 0x80115078;
            }
            0x80115078 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115024: bad pc {:#x}", pc),
        }
    }
}

/// _unloadFlare__FPvUlUl (0x80115094, 36 bytes)
#[inline(never)]
pub fn f_80115094(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115094) { return; }
        if c.prof { crate::prof::enter(c, 0x80115094); f_80115094_b(c, 0x80115094); crate::prof::exit(c, 0x80115094); return; }
    }
    f_80115094_b(c, 0x80115094)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115094_at(c: &mut Cpu, pc: u32) { f_80115094_b(c, pc) }
#[inline(always)]
fn f_80115094_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115094;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115094, pc); }
        match pc {
            0x80115094 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801150a4; f_801ed674(c);
                pc = 0x801150a4;
            }
            0x801150a4 => {
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115094: bad pc {:#x}", pc),
        }
    }
}

/// _unloadParticles__FPvUlUl (0x801150b8, 36 bytes)
#[inline(never)]
pub fn f_801150b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801150b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801150b8); f_801150b8_b(c, 0x801150b8); crate::prof::exit(c, 0x801150b8); return; }
    }
    f_801150b8_b(c, 0x801150b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801150b8_at(c: &mut Cpu, pc: u32) { f_801150b8_b(c, pc) }
#[inline(always)]
fn f_801150b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801150b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801150b8, pc); }
        match pc {
            0x801150b8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801150c8; f_801193bc(c);
                pc = 0x801150c8;
            }
            0x801150c8 => {
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801150b8: bad pc {:#x}", pc),
        }
    }
}

/// _unloadCamera__FPvUlUl (0x801150dc, 36 bytes)
#[inline(never)]
pub fn f_801150dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801150dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801150dc); f_801150dc_b(c, 0x801150dc); crate::prof::exit(c, 0x801150dc); return; }
    }
    f_801150dc_b(c, 0x801150dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801150dc_at(c: &mut Cpu, pc: u32) { f_801150dc_b(c, pc) }
#[inline(always)]
fn f_801150dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801150dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801150dc, pc); }
        match pc {
            0x801150dc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801150ec; f_800d2738(c);
                pc = 0x801150ec;
            }
            0x801150ec => {
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801150dc: bad pc {:#x}", pc),
        }
    }
}

/// _unloadLight__FPvUlUl (0x80115100, 36 bytes)
#[inline(never)]
pub fn f_80115100(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115100) { return; }
        if c.prof { crate::prof::enter(c, 0x80115100); f_80115100_b(c, 0x80115100); crate::prof::exit(c, 0x80115100); return; }
    }
    f_80115100_b(c, 0x80115100)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115100_at(c: &mut Cpu, pc: u32) { f_80115100_b(c, pc) }
#[inline(always)]
fn f_80115100_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115100;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115100, pc); }
        match pc {
            0x80115100 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80115110; f_800dcd98(c);
                pc = 0x80115110;
            }
            0x80115110 => {
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115100: bad pc {:#x}", pc),
        }
    }
}

/// _unloadScript__FPvUlUl (0x80115124, 76 bytes)
#[inline(never)]
pub fn f_80115124(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115124) { return; }
        if c.prof { crate::prof::enter(c, 0x80115124); f_80115124_b(c, 0x80115124); crate::prof::exit(c, 0x80115124); return; }
    }
    f_80115124_b(c, 0x80115124)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115124_at(c: &mut Cpu, pc: u32) { f_80115124_b(c, pc) }
#[inline(always)]
fn f_80115124_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115124;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115124, pc); }
        match pc {
            0x80115124 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x8011513c; f_800ff554(c);
                pc = 0x8011513c;
            }
            0x8011513c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115150; continue; }
                pc = 0x80115148;
            }
            0x80115148 => {
                c.r[3] = 0x0;
                pc = 0x8011515c; continue;
                pc = 0x80115150;
            }
            0x80115150 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80115158; f_800f760c(c);
                pc = 0x80115158;
            }
            0x80115158 => {
                c.r[3] = 0x1;
                pc = 0x8011515c;
            }
            0x8011515c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115124: bad pc {:#x}", pc),
        }
    }
}

/// _unloadFont__FPvUlUl (0x80115170, 76 bytes)
#[inline(never)]
pub fn f_80115170(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115170) { return; }
        if c.prof { crate::prof::enter(c, 0x80115170); f_80115170_b(c, 0x80115170); crate::prof::exit(c, 0x80115170); return; }
    }
    f_80115170_b(c, 0x80115170)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115170_at(c: &mut Cpu, pc: u32) { f_80115170_b(c, pc) }
#[inline(always)]
fn f_80115170_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115170;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115170, pc); }
        match pc {
            0x80115170 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80115188; f_800ff554(c);
                pc = 0x80115188;
            }
            0x80115188 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8011519c; continue; }
                pc = 0x80115194;
            }
            0x80115194 => {
                c.r[3] = 0x0;
                pc = 0x801151a8; continue;
                pc = 0x8011519c;
            }
            0x8011519c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801151a4; f_800fc2a8(c);
                pc = 0x801151a4;
            }
            0x801151a4 => {
                c.r[3] = 0x1;
                pc = 0x801151a8;
            }
            0x801151a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115170: bad pc {:#x}", pc),
        }
    }
}

/// _unloadMsg__FPvUlUl (0x801151bc, 76 bytes)
#[inline(never)]
pub fn f_801151bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801151bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801151bc); f_801151bc_b(c, 0x801151bc); crate::prof::exit(c, 0x801151bc); return; }
    }
    f_801151bc_b(c, 0x801151bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801151bc_at(c: &mut Cpu, pc: u32) { f_801151bc_b(c, pc) }
#[inline(always)]
fn f_801151bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801151bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801151bc, pc); }
        match pc {
            0x801151bc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801151d4; f_800ff554(c);
                pc = 0x801151d4;
            }
            0x801151d4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801151e8; continue; }
                pc = 0x801151e0;
            }
            0x801151e0 => {
                c.r[3] = 0x0;
                pc = 0x801151f4; continue;
                pc = 0x801151e8;
            }
            0x801151e8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801151f0; f_800fc1d0(c);
                pc = 0x801151f0;
            }
            0x801151f0 => {
                c.r[3] = 0x1;
                pc = 0x801151f4;
            }
            0x801151f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801151bc: bad pc {:#x}", pc),
        }
    }
}

/// _unloadColsys__FPvUlUl (0x80115208, 36 bytes)
#[inline(never)]
pub fn f_80115208(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115208) { return; }
        if c.prof { crate::prof::enter(c, 0x80115208); f_80115208_b(c, 0x80115208); crate::prof::exit(c, 0x80115208); return; }
    }
    f_80115208_b(c, 0x80115208)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115208_at(c: &mut Cpu, pc: u32) { f_80115208_b(c, pc) }
#[inline(always)]
fn f_80115208_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115208;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115208, pc); }
        match pc {
            0x80115208 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80115218; f_8010cc04(c);
                pc = 0x80115218;
            }
            0x80115218 => {
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115208: bad pc {:#x}", pc),
        }
    }
}

/// _unloadTexture__FPvUlUl (0x8011522c, 36 bytes)
#[inline(never)]
pub fn f_8011522c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011522c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011522c); f_8011522c_b(c, 0x8011522c); crate::prof::exit(c, 0x8011522c); return; }
    }
    f_8011522c_b(c, 0x8011522c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011522c_at(c: &mut Cpu, pc: u32) { f_8011522c_b(c, pc) }
#[inline(always)]
fn f_8011522c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011522c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011522c, pc); }
        match pc {
            0x8011522c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x8011523c; f_800ef5a4(c);
                pc = 0x8011523c;
            }
            0x8011523c => {
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011522c: bad pc {:#x}", pc),
        }
    }
}

/// floorReadMakeFogResID (0x80115250, 12 bytes)
#[inline(never)]
pub fn f_80115250(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115250) { return; }
        if c.prof { crate::prof::enter(c, 0x80115250); f_80115250_b(c, 0x80115250); crate::prof::exit(c, 0x80115250); return; }
    }
    f_80115250_b(c, 0x80115250)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115250_at(c: &mut Cpu, pc: u32) { f_80115250_b(c, pc) }
#[inline(always)]
fn f_80115250_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115250;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115250, pc); }
        match pc {
            0x80115250 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x7fff0000;
                c.r[3] = c.r[0] | 0x1a00;
                if true { return; }
                return;
            }
            _ => panic!("f_80115250: bad pc {:#x}", pc),
        }
    }
}

/// floorReadMakeCameraResID (0x8011525c, 12 bytes)
#[inline(never)]
pub fn f_8011525c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011525c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011525c); f_8011525c_b(c, 0x8011525c); crate::prof::exit(c, 0x8011525c); return; }
    }
    f_8011525c_b(c, 0x8011525c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011525c_at(c: &mut Cpu, pc: u32) { f_8011525c_b(c, pc) }
#[inline(always)]
fn f_8011525c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011525c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011525c, pc); }
        match pc {
            0x8011525c => {
                c.r[0] = c.r[3].rotate_left(0) & 0x7fff0000;
                c.r[3] = c.r[0] | 0x1800;
                if true { return; }
                return;
            }
            _ => panic!("f_8011525c: bad pc {:#x}", pc),
        }
    }
}

/// floorReadMakeLightResID (0x80115268, 12 bytes)
#[inline(never)]
pub fn f_80115268(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115268) { return; }
        if c.prof { crate::prof::enter(c, 0x80115268); f_80115268_b(c, 0x80115268); crate::prof::exit(c, 0x80115268); return; }
    }
    f_80115268_b(c, 0x80115268)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115268_at(c: &mut Cpu, pc: u32) { f_80115268_b(c, pc) }
#[inline(always)]
fn f_80115268_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115268;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115268, pc); }
        match pc {
            0x80115268 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x7fff0000;
                c.r[3] = c.r[0] | 0x1600;
                if true { return; }
                return;
            }
            _ => panic!("f_80115268: bad pc {:#x}", pc),
        }
    }
}

/// floorReadMakeModelResID (0x80115274, 12 bytes)
#[inline(never)]
pub fn f_80115274(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115274) { return; }
        if c.prof { crate::prof::enter(c, 0x80115274); f_80115274_b(c, 0x80115274); crate::prof::exit(c, 0x80115274); return; }
    }
    f_80115274_b(c, 0x80115274)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115274_at(c: &mut Cpu, pc: u32) { f_80115274_b(c, pc) }
#[inline(always)]
fn f_80115274_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115274;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115274, pc); }
        match pc {
            0x80115274 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x7fff0000;
                c.r[3] = c.r[0] | 0x1000;
                if true { return; }
                return;
            }
            _ => panic!("f_80115274: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetShadowReciveNum (0x80115280, 268 bytes)
#[inline(never)]
pub fn f_80115280(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115280) { return; }
        if c.prof { crate::prof::enter(c, 0x80115280); f_80115280_b(c, 0x80115280); crate::prof::exit(c, 0x80115280); return; }
    }
    f_80115280_b(c, 0x80115280)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115280_at(c: &mut Cpu, pc: u32) { f_80115280_b(c, pc) }
#[inline(always)]
fn f_80115280_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115280;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115280, pc); }
        match pc {
            0x80115280 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                c.r[4] = 0x0;
                if !c.crbit(2) { pc = 0x801152b8; continue; }
                pc = 0x80115298;
            }
            0x80115298 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffbb50);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801152b0; stubs_gen::f_800dd970(c);
                pc = 0x801152b0;
            }
            0x801152b0 => {
                c.r[3] = 0x0;
                pc = 0x8011537c; continue;
                pc = 0x801152b8;
            }
            0x801152b8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801152cc; continue; }
                pc = 0x801152c4;
            }
            0x801152c4 => {
                c.r[3] = 0x0;
                pc = 0x8011537c; continue;
                pc = 0x801152cc;
            }
            0x801152cc => {
                c.r[3] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801152f8; continue; }
                pc = 0x801152d8;
            }
            0x801152d8 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x262c);
                c.r[4] = c.r[4].wrapping_add(0xffffbb50);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801152f0; stubs_gen::f_800dd970(c);
                pc = 0x801152f0;
            }
            0x801152f0 => {
                c.r[3] = 0x0;
                pc = 0x8011537c; continue;
                pc = 0x801152f8;
            }
            0x801152f8 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115308; continue; }
                pc = 0x80115304;
            }
            0x80115304 => {
                c.r[4] = 0x1;
                pc = 0x80115308;
            }
            0x80115308 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0xc));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115318; continue; }
                pc = 0x80115314;
            }
            0x80115314 => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x80115318;
            }
            0x80115318 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x10));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115328; continue; }
                pc = 0x80115324;
            }
            0x80115324 => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x80115328;
            }
            0x80115328 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x14));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115338; continue; }
                pc = 0x80115334;
            }
            0x80115334 => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x80115338;
            }
            0x80115338 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x18));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115348; continue; }
                pc = 0x80115344;
            }
            0x80115344 => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x80115348;
            }
            0x80115348 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115358; continue; }
                pc = 0x80115354;
            }
            0x80115354 => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x80115358;
            }
            0x80115358 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x20));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115368; continue; }
                pc = 0x80115364;
            }
            0x80115364 => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x80115368;
            }
            0x80115368 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x24));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115378; continue; }
                pc = 0x80115374;
            }
            0x80115374 => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x80115378;
            }
            0x80115378 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x8011537c;
            }
            0x8011537c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115280: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetShadowReciveID (0x8011538c, 160 bytes)
#[inline(never)]
pub fn f_8011538c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011538c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011538c); f_8011538c_b(c, 0x8011538c); crate::prof::exit(c, 0x8011538c); return; }
    }
    f_8011538c_b(c, 0x8011538c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011538c_at(c: &mut Cpu, pc: u32) { f_8011538c_b(c, pc) }
#[inline(always)]
fn f_8011538c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011538c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011538c, pc); }
        match pc {
            0x8011538c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801153c0; continue; }
                pc = 0x801153a0;
            }
            0x801153a0 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffbb30);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801153b8; stubs_gen::f_800dd970(c);
                pc = 0x801153b8;
            }
            0x801153b8 => {
                c.r[3] = 0x0;
                pc = 0x8011541c; continue;
                pc = 0x801153c0;
            }
            0x801153c0 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801153d4; continue; }
                pc = 0x801153cc;
            }
            0x801153cc => {
                c.r[3] = 0x0;
                pc = 0x8011541c; continue;
                pc = 0x801153d4;
            }
            0x801153d4 => {
                c.cmpu(0, c.r[4], 0x8);
                if c.crbit(0) { pc = 0x801153e4; continue; }
                pc = 0x801153dc;
            }
            0x801153dc => {
                c.r[3] = 0x0;
                pc = 0x8011541c; continue;
                pc = 0x801153e4;
            }
            0x801153e4 => {
                c.r[3] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115410; continue; }
                pc = 0x801153f0;
            }
            0x801153f0 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x262c);
                c.r[4] = c.r[4].wrapping_add(0xffffbb30);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115408; stubs_gen::f_800dd970(c);
                pc = 0x80115408;
            }
            0x80115408 => {
                c.r[3] = 0x0;
                pc = 0x8011541c; continue;
                pc = 0x80115410;
            }
            0x80115410 => {
                c.r[0] = c.r[4].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x8011541c;
            }
            0x8011541c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011538c: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetShadowLightID (0x8011542c, 136 bytes)
#[inline(never)]
pub fn f_8011542c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011542c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011542c); f_8011542c_b(c, 0x8011542c); crate::prof::exit(c, 0x8011542c); return; }
    }
    f_8011542c_b(c, 0x8011542c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011542c_at(c: &mut Cpu, pc: u32) { f_8011542c_b(c, pc) }
#[inline(always)]
fn f_8011542c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011542c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011542c, pc); }
        match pc {
            0x8011542c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115460; continue; }
                pc = 0x80115440;
            }
            0x80115440 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffbb10);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115458; stubs_gen::f_800dd970(c);
                pc = 0x80115458;
            }
            0x80115458 => {
                c.r[3] = 0x0;
                pc = 0x801154a4; continue;
                pc = 0x80115460;
            }
            0x80115460 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115474; continue; }
                pc = 0x8011546c;
            }
            0x8011546c => {
                c.r[3] = 0x0;
                pc = 0x801154a4; continue;
                pc = 0x80115474;
            }
            0x80115474 => {
                c.r[3] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801154a0; continue; }
                pc = 0x80115480;
            }
            0x80115480 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x262c);
                c.r[4] = c.r[4].wrapping_add(0xffffbb10);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115498; stubs_gen::f_800dd970(c);
                pc = 0x80115498;
            }
            0x80115498 => {
                c.r[3] = 0x0;
                pc = 0x801154a4; continue;
                pc = 0x801154a0;
            }
            0x801154a0 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x801154a4;
            }
            0x801154a4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011542c: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetSunResID (0x801154b4, 136 bytes)
#[inline(never)]
pub fn f_801154b4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801154b4) { return; }
        if c.prof { crate::prof::enter(c, 0x801154b4); f_801154b4_b(c, 0x801154b4); crate::prof::exit(c, 0x801154b4); return; }
    }
    f_801154b4_b(c, 0x801154b4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801154b4_at(c: &mut Cpu, pc: u32) { f_801154b4_b(c, pc) }
#[inline(always)]
fn f_801154b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801154b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801154b4, pc); }
        match pc {
            0x801154b4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801154e8; continue; }
                pc = 0x801154c8;
            }
            0x801154c8 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffbaf4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801154e0; stubs_gen::f_800dd970(c);
                pc = 0x801154e0;
            }
            0x801154e0 => {
                c.r[3] = 0x0;
                pc = 0x8011552c; continue;
                pc = 0x801154e8;
            }
            0x801154e8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801154fc; continue; }
                pc = 0x801154f4;
            }
            0x801154f4 => {
                c.r[3] = 0x0;
                pc = 0x8011552c; continue;
                pc = 0x801154fc;
            }
            0x801154fc => {
                c.r[3] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115528; continue; }
                pc = 0x80115508;
            }
            0x80115508 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x262c);
                c.r[4] = c.r[4].wrapping_add(0xffffbaf4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115520; stubs_gen::f_800dd970(c);
                pc = 0x80115520;
            }
            0x80115520 => {
                c.r[3] = 0x0;
                pc = 0x8011552c; continue;
                pc = 0x80115528;
            }
            0x80115528 => {
                c.r[3] = c.lwz(c.r[3]);
                pc = 0x8011552c;
            }
            0x8011552c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801154b4: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011553C (0x8011553c, 72 bytes)
#[inline(never)]
pub fn f_8011553c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011553c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011553c); f_8011553c_b(c, 0x8011553c); crate::prof::exit(c, 0x8011553c); return; }
    }
    f_8011553c_b(c, 0x8011553c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011553c_at(c: &mut Cpu, pc: u32) { f_8011553c_b(c, pc) }
#[inline(always)]
fn f_8011553c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011553c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011553c, pc); }
        match pc {
            0x8011553c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115570; continue; }
                pc = 0x80115550;
            }
            0x80115550 => {
                c.r[3] = 0x80270000;
                c.r[6] = 0x80360000;
                c.r[5] = c.r[4] | c.r[4];
                c.r[3] = c.r[3].wrapping_add(0x2658);
                c.r[4] = c.r[6].wrapping_add(0xffffbad8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8011556c; stubs_gen::f_800dd970(c);
                pc = 0x8011556c;
            }
            0x8011556c => {
                pc = 0x80115574; continue;
                pc = 0x80115570;
            }
            0x80115570 => {
                c.stw(c.r[3].wrapping_add(0x34), c.r[4]);
                pc = 0x80115574;
            }
            0x80115574 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011553c: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosSetMapResID (0x80115584, 72 bytes)
#[inline(never)]
pub fn f_80115584(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115584) { return; }
        if c.prof { crate::prof::enter(c, 0x80115584); f_80115584_b(c, 0x80115584); crate::prof::exit(c, 0x80115584); return; }
    }
    f_80115584_b(c, 0x80115584)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115584_at(c: &mut Cpu, pc: u32) { f_80115584_b(c, pc) }
#[inline(always)]
fn f_80115584_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115584;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115584, pc); }
        match pc {
            0x80115584 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801155b8; continue; }
                pc = 0x80115598;
            }
            0x80115598 => {
                c.r[3] = 0x80270000;
                c.r[6] = 0x80360000;
                c.r[5] = c.r[4] | c.r[4];
                c.r[3] = c.r[3].wrapping_add(0x2680);
                c.r[4] = c.r[6].wrapping_add(0xffffbabc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801155b4; stubs_gen::f_800dd970(c);
                pc = 0x801155b4;
            }
            0x801155b4 => {
                pc = 0x801155bc; continue;
                pc = 0x801155b8;
            }
            0x801155b8 => {
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                pc = 0x801155bc;
            }
            0x801155bc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115584: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetFieldCameraListPtr (0x801155cc, 92 bytes)
#[inline(never)]
pub fn f_801155cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801155cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801155cc); f_801155cc_b(c, 0x801155cc); crate::prof::exit(c, 0x801155cc); return; }
    }
    f_801155cc_b(c, 0x801155cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801155cc_at(c: &mut Cpu, pc: u32) { f_801155cc_b(c, pc) }
#[inline(always)]
fn f_801155cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801155cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801155cc, pc); }
        match pc {
            0x801155cc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115600; continue; }
                pc = 0x801155e0;
            }
            0x801155e0 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffba98);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801155f8; stubs_gen::f_800dd970(c);
                pc = 0x801155f8;
            }
            0x801155f8 => {
                c.r[3] = 0x0;
                pc = 0x80115618; continue;
                pc = 0x80115600;
            }
            0x80115600 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115614; continue; }
                pc = 0x8011560c;
            }
            0x8011560c => {
                c.r[3] = 0x0;
                pc = 0x80115618; continue;
                pc = 0x80115614;
            }
            0x80115614 => {
                c.r[3] = c.lwz(c.r[3]);
                pc = 0x80115618;
            }
            0x80115618 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801155cc: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetPosListPtr (0x80115628, 92 bytes)
#[inline(never)]
pub fn f_80115628(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115628) { return; }
        if c.prof { crate::prof::enter(c, 0x80115628); f_80115628_b(c, 0x80115628); crate::prof::exit(c, 0x80115628); return; }
    }
    f_80115628_b(c, 0x80115628)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115628_at(c: &mut Cpu, pc: u32) { f_80115628_b(c, pc) }
#[inline(always)]
fn f_80115628_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115628;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115628, pc); }
        match pc {
            0x80115628 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011565c; continue; }
                pc = 0x8011563c;
            }
            0x8011563c => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffba7c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115654; stubs_gen::f_800dd970(c);
                pc = 0x80115654;
            }
            0x80115654 => {
                c.r[3] = 0x0;
                pc = 0x80115674; continue;
                pc = 0x8011565c;
            }
            0x8011565c => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x18));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115670; continue; }
                pc = 0x80115668;
            }
            0x80115668 => {
                c.r[3] = 0x0;
                pc = 0x80115674; continue;
                pc = 0x80115670;
            }
            0x80115670 => {
                c.r[3] = c.lwz(c.r[3]);
                pc = 0x80115674;
            }
            0x80115674 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115628: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetCharInfo (0x80115684, 128 bytes)
#[inline(never)]
pub fn f_80115684(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115684) { return; }
        if c.prof { crate::prof::enter(c, 0x80115684); f_80115684_b(c, 0x80115684); crate::prof::exit(c, 0x80115684); return; }
    }
    f_80115684_b(c, 0x80115684)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115684_at(c: &mut Cpu, pc: u32) { f_80115684_b(c, pc) }
#[inline(always)]
fn f_80115684_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115684;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115684, pc); }
        match pc {
            0x80115684 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801156b8; continue; }
                pc = 0x80115698;
            }
            0x80115698 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffba60);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801156b0; stubs_gen::f_800dd970(c);
                pc = 0x801156b0;
            }
            0x801156b0 => {
                c.r[3] = 0x0;
                pc = 0x801156f4; continue;
                pc = 0x801156b8;
            }
            0x801156b8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801156cc; continue; }
                pc = 0x801156c4;
            }
            0x801156c4 => {
                c.r[3] = 0x0;
                pc = 0x801156f4; continue;
                pc = 0x801156cc;
            }
            0x801156cc => {
                c.r[5] = c.lwz(c.r[3]);
                c.r[3] = c.lwz(c.r[5]);
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x801156e8; continue; }
                pc = 0x801156e0;
            }
            0x801156e0 => {
                c.r[3] = 0x0;
                pc = 0x801156f4; continue;
                pc = 0x801156e8;
            }
            0x801156e8 => {
                c.r[0] = c.r[4].wrapping_mul(0x24);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x801156f4;
            }
            0x801156f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115684: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetCharNum (0x80115704, 100 bytes)
#[inline(never)]
pub fn f_80115704(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115704) { return; }
        if c.prof { crate::prof::enter(c, 0x80115704); f_80115704_b(c, 0x80115704); crate::prof::exit(c, 0x80115704); return; }
    }
    f_80115704_b(c, 0x80115704)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115704_at(c: &mut Cpu, pc: u32) { f_80115704_b(c, pc) }
#[inline(always)]
fn f_80115704_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115704;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115704, pc); }
        match pc {
            0x80115704 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115738; continue; }
                pc = 0x80115718;
            }
            0x80115718 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffba48);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115730; stubs_gen::f_800dd970(c);
                pc = 0x80115730;
            }
            0x80115730 => {
                c.r[3] = 0x0;
                pc = 0x80115758; continue;
                pc = 0x80115738;
            }
            0x80115738 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011574c; continue; }
                pc = 0x80115744;
            }
            0x80115744 => {
                c.r[3] = 0x0;
                pc = 0x80115758; continue;
                pc = 0x8011574c;
            }
            0x8011574c => {
                c.r[3] = c.lwz(c.r[3]);
                c.r[3] = c.lwz(c.r[3]);
                c.r[3] = c.lwz(c.r[3]);
                pc = 0x80115758;
            }
            0x80115758 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115704: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetPostFunc (0x80115768, 72 bytes)
#[inline(never)]
pub fn f_80115768(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115768) { return; }
        if c.prof { crate::prof::enter(c, 0x80115768); f_80115768_b(c, 0x80115768); crate::prof::exit(c, 0x80115768); return; }
    }
    f_80115768_b(c, 0x80115768)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115768_at(c: &mut Cpu, pc: u32) { f_80115768_b(c, pc) }
#[inline(always)]
fn f_80115768_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115768;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115768, pc); }
        match pc {
            0x80115768 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011579c; continue; }
                pc = 0x8011577c;
            }
            0x8011577c => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffba2c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115794; stubs_gen::f_800dd970(c);
                pc = 0x80115794;
            }
            0x80115794 => {
                c.r[3] = 0x0;
                pc = 0x801157a0; continue;
                pc = 0x8011579c;
            }
            0x8011579c => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x44));
                pc = 0x801157a0;
            }
            0x801157a0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115768: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetMainFunc (0x801157b0, 72 bytes)
#[inline(never)]
pub fn f_801157b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801157b0) { return; }
        if c.prof { crate::prof::enter(c, 0x801157b0); f_801157b0_b(c, 0x801157b0); crate::prof::exit(c, 0x801157b0); return; }
    }
    f_801157b0_b(c, 0x801157b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801157b0_at(c: &mut Cpu, pc: u32) { f_801157b0_b(c, pc) }
#[inline(always)]
fn f_801157b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801157b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801157b0, pc); }
        match pc {
            0x801157b0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801157e4; continue; }
                pc = 0x801157c4;
            }
            0x801157c4 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffba10);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801157dc; stubs_gen::f_800dd970(c);
                pc = 0x801157dc;
            }
            0x801157dc => {
                c.r[3] = 0x0;
                pc = 0x801157e8; continue;
                pc = 0x801157e4;
            }
            0x801157e4 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x40));
                pc = 0x801157e8;
            }
            0x801157e8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801157b0: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetPreFunc (0x801157f8, 72 bytes)
#[inline(never)]
pub fn f_801157f8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801157f8) { return; }
        if c.prof { crate::prof::enter(c, 0x801157f8); f_801157f8_b(c, 0x801157f8); crate::prof::exit(c, 0x801157f8); return; }
    }
    f_801157f8_b(c, 0x801157f8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801157f8_at(c: &mut Cpu, pc: u32) { f_801157f8_b(c, pc) }
#[inline(always)]
fn f_801157f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801157f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801157f8, pc); }
        match pc {
            0x801157f8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011582c; continue; }
                pc = 0x8011580c;
            }
            0x8011580c => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb9f8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115824; stubs_gen::f_800dd970(c);
                pc = 0x80115824;
            }
            0x80115824 => {
                c.r[3] = 0x0;
                pc = 0x80115830; continue;
                pc = 0x8011582c;
            }
            0x8011582c => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x3c));
                pc = 0x80115830;
            }
            0x80115830 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801157f8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80115840 (0x80115840, 72 bytes)
#[inline(never)]
pub fn f_80115840(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115840) { return; }
        if c.prof { crate::prof::enter(c, 0x80115840); f_80115840_b(c, 0x80115840); crate::prof::exit(c, 0x80115840); return; }
    }
    f_80115840_b(c, 0x80115840)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115840_at(c: &mut Cpu, pc: u32) { f_80115840_b(c, pc) }
#[inline(always)]
fn f_80115840_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115840;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115840, pc); }
        match pc {
            0x80115840 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115874; continue; }
                pc = 0x80115854;
            }
            0x80115854 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb9dc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8011586c; stubs_gen::f_800dd970(c);
                pc = 0x8011586c;
            }
            0x8011586c => {
                c.r[3] = 0x0;
                pc = 0x80115878; continue;
                pc = 0x80115874;
            }
            0x80115874 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x24));
                pc = 0x80115878;
            }
            0x80115878 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115840: bad pc {:#x}", pc),
        }
    }
}

/// fn_80115888 (0x80115888, 72 bytes)
#[inline(never)]
pub fn f_80115888(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115888) { return; }
        if c.prof { crate::prof::enter(c, 0x80115888); f_80115888_b(c, 0x80115888); crate::prof::exit(c, 0x80115888); return; }
    }
    f_80115888_b(c, 0x80115888)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115888_at(c: &mut Cpu, pc: u32) { f_80115888_b(c, pc) }
#[inline(always)]
fn f_80115888_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115888;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115888, pc); }
        match pc {
            0x80115888 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801158bc; continue; }
                pc = 0x8011589c;
            }
            0x8011589c => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb9c0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801158b4; stubs_gen::f_800dd970(c);
                pc = 0x801158b4;
            }
            0x801158b4 => {
                c.r[3] = 0x0;
                pc = 0x801158c0; continue;
                pc = 0x801158bc;
            }
            0x801158bc => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x38));
                pc = 0x801158c0;
            }
            0x801158c0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115888: bad pc {:#x}", pc),
        }
    }
}

/// fn_801158D0 (0x801158d0, 72 bytes)
#[inline(never)]
pub fn f_801158d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801158d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801158d0); f_801158d0_b(c, 0x801158d0); crate::prof::exit(c, 0x801158d0); return; }
    }
    f_801158d0_b(c, 0x801158d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801158d0_at(c: &mut Cpu, pc: u32) { f_801158d0_b(c, pc) }
#[inline(always)]
fn f_801158d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801158d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801158d0, pc); }
        match pc {
            0x801158d0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115904; continue; }
                pc = 0x801158e4;
            }
            0x801158e4 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb9a4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801158fc; stubs_gen::f_800dd970(c);
                pc = 0x801158fc;
            }
            0x801158fc => {
                c.r[3] = 0x0;
                pc = 0x80115908; continue;
                pc = 0x80115904;
            }
            0x80115904 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x34));
                pc = 0x80115908;
            }
            0x80115908 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801158d0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80115918 (0x80115918, 72 bytes)
#[inline(never)]
pub fn f_80115918(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115918) { return; }
        if c.prof { crate::prof::enter(c, 0x80115918); f_80115918_b(c, 0x80115918); crate::prof::exit(c, 0x80115918); return; }
    }
    f_80115918_b(c, 0x80115918)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115918_at(c: &mut Cpu, pc: u32) { f_80115918_b(c, pc) }
#[inline(always)]
fn f_80115918_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115918;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115918, pc); }
        match pc {
            0x80115918 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011594c; continue; }
                pc = 0x8011592c;
            }
            0x8011592c => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb988);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115944; stubs_gen::f_800dd970(c);
                pc = 0x80115944;
            }
            0x80115944 => {
                c.r[3] = 0x0;
                pc = 0x80115950; continue;
                pc = 0x8011594c;
            }
            0x8011594c => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x30));
                pc = 0x80115950;
            }
            0x80115950 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115918: bad pc {:#x}", pc),
        }
    }
}

/// fn_80115960 (0x80115960, 72 bytes)
#[inline(never)]
pub fn f_80115960(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115960) { return; }
        if c.prof { crate::prof::enter(c, 0x80115960); f_80115960_b(c, 0x80115960); crate::prof::exit(c, 0x80115960); return; }
    }
    f_80115960_b(c, 0x80115960)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115960_at(c: &mut Cpu, pc: u32) { f_80115960_b(c, pc) }
#[inline(always)]
fn f_80115960_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115960;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115960, pc); }
        match pc {
            0x80115960 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115994; continue; }
                pc = 0x80115974;
            }
            0x80115974 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb96c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8011598c; stubs_gen::f_800dd970(c);
                pc = 0x8011598c;
            }
            0x8011598c => {
                c.r[3] = 0x0;
                pc = 0x80115998; continue;
                pc = 0x80115994;
            }
            0x80115994 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x2c));
                pc = 0x80115998;
            }
            0x80115998 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115960: bad pc {:#x}", pc),
        }
    }
}

/// fn_801159A8 (0x801159a8, 72 bytes)
#[inline(never)]
pub fn f_801159a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801159a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801159a8); f_801159a8_b(c, 0x801159a8); crate::prof::exit(c, 0x801159a8); return; }
    }
    f_801159a8_b(c, 0x801159a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801159a8_at(c: &mut Cpu, pc: u32) { f_801159a8_b(c, pc) }
#[inline(always)]
fn f_801159a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801159a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801159a8, pc); }
        match pc {
            0x801159a8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801159dc; continue; }
                pc = 0x801159bc;
            }
            0x801159bc => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb950);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801159d4; stubs_gen::f_800dd970(c);
                pc = 0x801159d4;
            }
            0x801159d4 => {
                c.r[3] = 0x0;
                pc = 0x801159e0; continue;
                pc = 0x801159dc;
            }
            0x801159dc => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x28));
                pc = 0x801159e0;
            }
            0x801159e0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801159a8: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetFloorID (0x801159f0, 72 bytes)
#[inline(never)]
pub fn f_801159f0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801159f0) { return; }
        if c.prof { crate::prof::enter(c, 0x801159f0); f_801159f0_b(c, 0x801159f0); crate::prof::exit(c, 0x801159f0); return; }
    }
    f_801159f0_b(c, 0x801159f0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801159f0_at(c: &mut Cpu, pc: u32) { f_801159f0_b(c, pc) }
#[inline(always)]
fn f_801159f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801159f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801159f0, pc); }
        match pc {
            0x801159f0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115a24; continue; }
                pc = 0x80115a04;
            }
            0x80115a04 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb938);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115a1c; stubs_gen::f_800dd970(c);
                pc = 0x80115a1c;
            }
            0x80115a1c => {
                c.r[3] = 0x0;
                pc = 0x80115a28; continue;
                pc = 0x80115a24;
            }
            0x80115a24 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                pc = 0x80115a28;
            }
            0x80115a28 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801159f0: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetFileGroupID (0x80115a38, 72 bytes)
#[inline(never)]
pub fn f_80115a38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115a38) { return; }
        if c.prof { crate::prof::enter(c, 0x80115a38); f_80115a38_b(c, 0x80115a38); crate::prof::exit(c, 0x80115a38); return; }
    }
    f_80115a38_b(c, 0x80115a38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115a38_at(c: &mut Cpu, pc: u32) { f_80115a38_b(c, pc) }
#[inline(always)]
fn f_80115a38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115a38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115a38, pc); }
        match pc {
            0x80115a38 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115a6c; continue; }
                pc = 0x80115a4c;
            }
            0x80115a4c => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb91c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115a64; stubs_gen::f_800dd970(c);
                pc = 0x80115a64;
            }
            0x80115a64 => {
                c.r[3] = 0x0;
                pc = 0x80115a70; continue;
                pc = 0x80115a6c;
            }
            0x80115a6c => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x80115a70;
            }
            0x80115a70 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115a38: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetGroupID (0x80115a80, 72 bytes)
#[inline(never)]
pub fn f_80115a80(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115a80) { return; }
        if c.prof { crate::prof::enter(c, 0x80115a80); f_80115a80_b(c, 0x80115a80); crate::prof::exit(c, 0x80115a80); return; }
    }
    f_80115a80_b(c, 0x80115a80)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115a80_at(c: &mut Cpu, pc: u32) { f_80115a80_b(c, pc) }
#[inline(always)]
fn f_80115a80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115a80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115a80, pc); }
        match pc {
            0x80115a80 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115ab4; continue; }
                pc = 0x80115a94;
            }
            0x80115a94 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb904);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115aac; stubs_gen::f_800dd970(c);
                pc = 0x80115aac;
            }
            0x80115aac => {
                c.r[3] = 0x0;
                pc = 0x80115ab8; continue;
                pc = 0x80115ab4;
            }
            0x80115ab4 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x80115ab8;
            }
            0x80115ab8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115a80: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetFloorKind (0x80115ac8, 128 bytes)
#[inline(never)]
pub fn f_80115ac8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115ac8) { return; }
        if c.prof { crate::prof::enter(c, 0x80115ac8); f_80115ac8_b(c, 0x80115ac8); crate::prof::exit(c, 0x80115ac8); return; }
    }
    f_80115ac8_b(c, 0x80115ac8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115ac8_at(c: &mut Cpu, pc: u32) { f_80115ac8_b(c, pc) }
#[inline(always)]
fn f_80115ac8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115ac8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115ac8, pc); }
        match pc {
            0x80115ac8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80115b04; continue; }
                pc = 0x80115ae4;
            }
            0x80115ae4 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb8e8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115afc; stubs_gen::f_800dd970(c);
                pc = 0x80115afc;
            }
            0x80115afc => {
                c.r[3] = 0x0;
                pc = 0x80115b34; continue;
                pc = 0x80115b04;
            }
            0x80115b04 => {
                c.r[0] = c.lbz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(2) { pc = 0x80115b2c; continue; }
                pc = 0x80115b14;
            }
            0x80115b14 => {
                c.lr = 0x80115b18; f_801ef624(c);
                pc = 0x80115b18;
            }
            0x80115b18 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80115b2c; continue; }
                pc = 0x80115b24;
            }
            0x80115b24 => {
                c.r[3] = 0x1;
                pc = 0x80115b34; continue;
                pc = 0x80115b2c;
            }
            0x80115b2c => {
                c.r[0] = c.lbz(c.r[31]);
                c.r[3] = c.r[0].rotate_left(27) & 0x7;
                pc = 0x80115b34;
            }
            0x80115b34 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115ac8: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetMapResID (0x80115b48, 72 bytes)
#[inline(never)]
pub fn f_80115b48(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115b48) { return; }
        if c.prof { crate::prof::enter(c, 0x80115b48); f_80115b48_b(c, 0x80115b48); crate::prof::exit(c, 0x80115b48); return; }
    }
    f_80115b48_b(c, 0x80115b48)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115b48_at(c: &mut Cpu, pc: u32) { f_80115b48_b(c, pc) }
#[inline(always)]
fn f_80115b48_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115b48;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115b48, pc); }
        match pc {
            0x80115b48 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115b7c; continue; }
                pc = 0x80115b5c;
            }
            0x80115b5c => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x26ac);
                c.r[4] = c.r[4].wrapping_add(0xffffb8cc);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115b74; stubs_gen::f_800dd970(c);
                pc = 0x80115b74;
            }
            0x80115b74 => {
                c.r[3] = 0x0;
                pc = 0x80115b80; continue;
                pc = 0x80115b7c;
            }
            0x80115b7c => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x80115b80;
            }
            0x80115b80 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115b48: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetArea (0x80115b90, 72 bytes)
#[inline(never)]
pub fn f_80115b90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115b90) { return; }
        if c.prof { crate::prof::enter(c, 0x80115b90); f_80115b90_b(c, 0x80115b90); crate::prof::exit(c, 0x80115b90); return; }
    }
    f_80115b90_b(c, 0x80115b90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115b90_at(c: &mut Cpu, pc: u32) { f_80115b90_b(c, pc) }
#[inline(always)]
fn f_80115b90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115b90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115b90, pc); }
        match pc {
            0x80115b90 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80115bc4; continue; }
                pc = 0x80115ba4;
            }
            0x80115ba4 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2608);
                c.r[4] = c.r[4].wrapping_add(0xffffb8b4);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115bbc; stubs_gen::f_800dd970(c);
                pc = 0x80115bbc;
            }
            0x80115bbc => {
                c.r[3] = 0x0;
                pc = 0x80115bc8; continue;
                pc = 0x80115bc4;
            }
            0x80115bc4 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                pc = 0x80115bc8;
            }
            0x80115bc8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115b90: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetCurrentPtr (0x80115bd8, 112 bytes)
#[inline(never)]
pub fn f_80115bd8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115bd8) { return; }
        if c.prof { crate::prof::enter(c, 0x80115bd8); f_80115bd8_b(c, 0x80115bd8); crate::prof::exit(c, 0x80115bd8); return; }
    }
    f_80115bd8_b(c, 0x80115bd8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115bd8_at(c: &mut Cpu, pc: u32) { f_80115bd8_b(c, pc) }
#[inline(always)]
fn f_80115bd8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115bd8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115bd8, pc); }
        match pc {
            0x80115bd8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80115be8; f_800ff56c(c);
                pc = 0x80115be8;
            }
            0x80115be8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8798));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff879c));
                c.r[0] = c.lwz(c.r[4]);
                c.mtspr(9, c.r[0]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115c18; continue; }
                pc = 0x80115c00;
            }
            0x80115c00 => {
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xc));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x80115c10; continue; }
                pc = 0x80115c0c;
            }
            0x80115c0c => {
                pc = 0x80115c34; continue;
                pc = 0x80115c10;
            }
            0x80115c10 => {
                c.r[5] = c.r[5].wrapping_add(0x4c);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80115c00; continue; }
                pc = 0x80115c18;
            }
            0x80115c18 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x26d4);
                c.r[4] = c.r[4].wrapping_add(0xffffb8a0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115c30; stubs_gen::f_800dd970(c);
                pc = 0x80115c30;
            }
            0x80115c30 => {
                c.r[5] = 0x0;
                pc = 0x80115c34;
            }
            0x80115c34 => {
                c.r[3] = c.r[5] | c.r[5];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115bd8: bad pc {:#x}", pc),
        }
    }
}

/// floorDataBiosGetPtr (0x80115c48, 108 bytes)
#[inline(never)]
pub fn f_80115c48(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115c48) { return; }
        if c.prof { crate::prof::enter(c, 0x80115c48); f_80115c48_b(c, 0x80115c48); crate::prof::exit(c, 0x80115c48); return; }
    }
    f_80115c48_b(c, 0x80115c48)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115c48_at(c: &mut Cpu, pc: u32) { f_80115c48_b(c, pc) }
#[inline(always)]
fn f_80115c48_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115c48;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115c48, pc); }
        match pc {
            0x80115c48 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8798));
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff879c));
                c.r[0] = c.lwz(c.r[4]);
                c.mtspr(9, c.r[0]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115c88; continue; }
                pc = 0x80115c6c;
            }
            0x80115c6c => {
                c.r[0] = c.lwz(c.r[5].wrapping_add(0xc));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x80115c80; continue; }
                pc = 0x80115c78;
            }
            0x80115c78 => {
                c.r[3] = c.r[5] | c.r[5];
                pc = 0x80115ca4; continue;
                pc = 0x80115c80;
            }
            0x80115c80 => {
                c.r[5] = c.r[5].wrapping_add(0x4c);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80115c6c; continue; }
                pc = 0x80115c88;
            }
            0x80115c88 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x26d4);
                c.r[4] = c.r[4].wrapping_add(0xffffb8a0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115ca0; stubs_gen::f_800dd970(c);
                pc = 0x80115ca0;
            }
            0x80115ca0 => {
                c.r[3] = 0x0;
                pc = 0x80115ca4;
            }
            0x80115ca4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115c48: bad pc {:#x}", pc),
        }
    }
}

/// floorEventGetTresureList (0x80115cb4, 176 bytes)
#[inline(never)]
pub fn f_80115cb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115cb4) { return; }
        if c.prof { crate::prof::enter(c, 0x80115cb4); f_80115cb4_b(c, 0x80115cb4); crate::prof::exit(c, 0x80115cb4); return; }
    }
    f_80115cb4_b(c, 0x80115cb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115cb4_at(c: &mut Cpu, pc: u32) { f_80115cb4_b(c, pc) }
#[inline(always)]
fn f_80115cb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115cb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115cb4, pc); }
        match pc {
            0x80115cb4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[4] = c.r[3].rotate_left(0) & 0x7fff0000;
                c.r[30] = 0x0;
                c.r[0] = c.r[4].wrapping_add(0x80010000);
                c.r[28] = 0x0;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80115ce4; continue; }
                pc = 0x80115cdc;
            }
            0x80115cdc => {
                c.r[3] = 0x0;
                pc = 0x80115d50; continue;
                pc = 0x80115ce4;
            }
            0x80115ce4 => {
                c.r[27] = c.r[3].rotate_left(0) & 0x1ff;
                c.r[29] = 0x0;
                c.r[31] = 0x0;
                pc = 0x80115d24; continue;
                pc = 0x80115cf4;
            }
            0x80115cf4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff869c));
                c.r[30] = c.r[0].wrapping_add(c.r[31]);
                c.lr = 0x80115d00; f_800ff56c(c);
                pc = 0x80115d00;
            }
            0x80115d00 => {
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x4));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x80115d1c; continue; }
                pc = 0x80115d0c;
            }
            0x80115d0c => {
                c.r[0] = c.r[28] | c.r[28];
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.cmpu(0, c.r[27], c.r[0]);
                if c.crbit(2) { pc = 0x80115d34; continue; }
                pc = 0x80115d1c;
            }
            0x80115d1c => {
                c.r[31] = c.r[31].wrapping_add(0x1c);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x80115d24;
            }
            0x80115d24 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8698));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x80115cf4; continue; }
                pc = 0x80115d34;
            }
            0x80115d34 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8698));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[29], c.r[0]);
                if !c.crbit(2) { pc = 0x80115d4c; continue; }
                pc = 0x80115d44;
            }
            0x80115d44 => {
                c.r[3] = 0x0;
                pc = 0x80115d50; continue;
                pc = 0x80115d4c;
            }
            0x80115d4c => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x80115d50;
            }
            0x80115d50 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80115cb4: bad pc {:#x}", pc),
        }
    }
}

/// floorEventSetTresureDisp (0x80115d64, 160 bytes)
#[inline(never)]
pub fn f_80115d64(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115d64) { return; }
        if c.prof { crate::prof::enter(c, 0x80115d64); f_80115d64_b(c, 0x80115d64); crate::prof::exit(c, 0x80115d64); return; }
    }
    f_80115d64_b(c, 0x80115d64)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115d64_at(c: &mut Cpu, pc: u32) { f_80115d64_b(c, pc) }
#[inline(always)]
fn f_80115d64_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115d64;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115d64, pc); }
        match pc {
            0x80115d64 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.r[26] = c.r[4] | c.r[4];
                c.r[28] = 0x0;
                c.r[29] = 0x0;
                c.r[31] = 0x0;
                pc = 0x80115dc0; continue;
                pc = 0x80115d8c;
            }
            0x80115d8c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff869c));
                c.r[30] = c.r[0].wrapping_add(c.r[31]);
                c.lr = 0x80115d98; f_800ff56c(c);
                pc = 0x80115d98;
            }
            0x80115d98 => {
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x4));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x80115db8; continue; }
                pc = 0x80115da4;
            }
            0x80115da4 => {
                c.cmpu(0, c.r[29], c.r[25]);
                if !c.crbit(2) { pc = 0x80115db4; continue; }
                pc = 0x80115dac;
            }
            0x80115dac => {
                c.r[27] = c.r[28] | 0x7fff0000;
                pc = 0x80115dd0; continue;
                pc = 0x80115db4;
            }
            0x80115db4 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x80115db8;
            }
            0x80115db8 => {
                c.r[31] = c.r[31].wrapping_add(0x1c);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x80115dc0;
            }
            0x80115dc0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8698));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x80115d8c; continue; }
                pc = 0x80115dd0;
            }
            0x80115dd0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8698));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(2) { pc = 0x80115df0; continue; }
                pc = 0x80115de0;
            }
            0x80115de0 => {
                c.lr = 0x80115de4; f_80113f48(c);
                pc = 0x80115de4;
            }
            0x80115de4 => {
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[26] | c.r[26];
                c.lr = 0x80115df0; f_8018c1e8(c);
                pc = 0x80115df0;
            }
            0x80115df0 => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80115d64: bad pc {:#x}", pc),
        }
    }
}

/// floorEventChangeTresure (0x80115e04, 104 bytes)
#[inline(never)]
pub fn f_80115e04(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115e04) { return; }
        if c.prof { crate::prof::enter(c, 0x80115e04); f_80115e04_b(c, 0x80115e04); crate::prof::exit(c, 0x80115e04); return; }
    }
    f_80115e04_b(c, 0x80115e04)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115e04_at(c: &mut Cpu, pc: u32) { f_80115e04_b(c, pc) }
#[inline(always)]
fn f_80115e04_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115e04;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115e04, pc); }
        match pc {
            0x80115e04 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff8698));
                c.r[0] = c.lwz(c.r[6]);
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80115e40; continue; }
                pc = 0x80115e20;
            }
            0x80115e20 => {
                c.r[3] = 0x80270000;
                c.r[4] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x2708);
                c.r[4] = c.r[4].wrapping_add(0xffffbb70);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80115e38; stubs_gen::f_800dd970(c);
                pc = 0x80115e38;
            }
            0x80115e38 => {
                c.r[3] = 0xffffffff;
                pc = 0x80115e5c; continue;
                pc = 0x80115e40;
            }
            0x80115e40 => {
                c.r[6] = c.r[3].wrapping_mul(0x1c);
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffff869c));
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[4] = c.r[7].wrapping_add(c.r[6]);
                c.stw(c.r[4].wrapping_add(0xc), c.r[0]);
                c.stb(c.r[4].wrapping_add(0x1), c.r[5]);
                pc = 0x80115e5c;
            }
            0x80115e5c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80115e04: bad pc {:#x}", pc),
        }
    }
}

/// floorEventGetTresure (0x80115e6c, 760 bytes)
#[inline(never)]
pub fn f_80115e6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80115e6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80115e6c); f_80115e6c_b(c, 0x80115e6c); crate::prof::exit(c, 0x80115e6c); return; }
    }
    f_80115e6c_b(c, 0x80115e6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80115e6c_at(c: &mut Cpu, pc: u32) { f_80115e6c_b(c, pc) }
#[inline(always)]
fn f_80115e6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80115e6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80115e6c, pc); }
        match pc {
            0x80115e6c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = 0x0;
                c.r[3] = 0x3ca;
                c.r[4] = 0x0;
                c.r[5] = 0xff;
                c.lr = 0x80115e9c; f_801653cc(c);
                pc = 0x80115e9c;
            }
            0x80115e9c => {
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x80115ffc; continue; }
                pc = 0x80115ea8;
            }
            0x80115ea8 => {
                if !c.crbit(0) { pc = 0x80115ebc; continue; }
                pc = 0x80115eac;
            }
            0x80115eac => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80115f1c; continue; }
                pc = 0x80115eb4;
            }
            0x80115eb4 => {
                if !c.crbit(0) { pc = 0x80115ec8; continue; }
                pc = 0x80115eb8;
            }
            0x80115eb8 => {
                pc = 0x8011614c; continue;
                pc = 0x80115ebc;
            }
            0x80115ebc => {
                c.cmps(0, c.r[0], 0x5);
                if !c.crbit(0) { pc = 0x8011614c; continue; }
                pc = 0x80115ec4;
            }
            0x80115ec4 => {
                pc = 0x80115f1c; continue;
                pc = 0x80115ec8;
            }
            0x80115ec8 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x4b;
                c.lr = 0x80115ed4; f_80132a38(c);
                pc = 0x80115ed4;
            }
            0x80115ed4 => {
                c.r[3] = 0x3;
                c.r[4] = 0x3cb5;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80115ee8; f_80106d3c(c);
                pc = 0x80115ee8;
            }
            0x80115ee8 => {
                c.r[3] = 0x1;
                c.lr = 0x80115ef0; f_801069fc(c);
                pc = 0x80115ef0;
            }
            0x80115ef0 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.lr = 0x80115efc; f_801294c4(c);
                pc = 0x80115efc;
            }
            0x80115efc => {
                c.r[3] = 0x3;
                c.r[4] = 0x3cb7;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80115f10; f_80106d3c(c);
                pc = 0x80115f10;
            }
            0x80115f10 => {
                c.r[3] = 0x1;
                c.lr = 0x80115f18; f_801069fc(c);
                pc = 0x80115f18;
            }
            0x80115f18 => {
                pc = 0x8011614c; continue;
                pc = 0x80115f1c;
            }
            0x80115f1c => {
                c.cmps(0, c.r[29], 0x0);
                if !c.crbit(1) { pc = 0x8011614c; continue; }
                pc = 0x80115f24;
            }
            0x80115f24 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x2d;
                c.lr = 0x80115f30; f_80132a38(c);
                pc = 0x80115f30;
            }
            0x80115f30 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = 0x2f;
                c.lr = 0x80115f3c; f_80132a38(c);
                pc = 0x80115f3c;
            }
            0x80115f3c => {
                c.r[0] = c.r[27].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80115f78; continue; }
                pc = 0x80115f48;
            }
            0x80115f48 => {
                c.cmps(0, c.r[29], 0x1);
                if !c.crbit(2) { pc = 0x80115f58; continue; }
                pc = 0x80115f50;
            }
            0x80115f50 => {
                c.r[30] = 0x3cb4;
                pc = 0x80115f5c; continue;
                pc = 0x80115f58;
            }
            0x80115f58 => {
                c.r[30] = 0x3cb9;
                pc = 0x80115f5c;
            }
            0x80115f5c => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x3;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80115f70; f_80106d3c(c);
                pc = 0x80115f70;
            }
            0x80115f70 => {
                c.r[3] = 0x1;
                c.lr = 0x80115f78; f_801069fc(c);
                pc = 0x80115f78;
            }
            0x80115f78 => {
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = c.r[29].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[6] = 0xffffffff;
                c.lr = 0x80115f8c; f_80129a78(c);
                pc = 0x80115f8c;
            }
            0x80115f8c => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80115fb0; continue; }
                pc = 0x80115f98;
            }
            0x80115f98 => {
                c.cmps(0, c.r[29], 0x1);
                if !c.crbit(2) { pc = 0x80115fa8; continue; }
                pc = 0x80115fa0;
            }
            0x80115fa0 => {
                c.r[30] = 0x3cb8;
                pc = 0x80115fdc; continue;
                pc = 0x80115fa8;
            }
            0x80115fa8 => {
                c.r[30] = 0x3cbd;
                pc = 0x80115fdc; continue;
                pc = 0x80115fb0;
            }
            0x80115fb0 => {
                if !c.crbit(1) { pc = 0x80115fdc; continue; }
                pc = 0x80115fb4;
            }
            0x80115fb4 => {
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[5] = c.r[31].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.lr = 0x80115fc4; f_8013467c(c);
                pc = 0x80115fc4;
            }
            0x80115fc4 => {
                c.cmps(0, c.r[29], 0x1);
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                if !c.crbit(2) { pc = 0x80115fd8; continue; }
                pc = 0x80115fd0;
            }
            0x80115fd0 => {
                c.r[30] = 0x3cba;
                pc = 0x80115fdc; continue;
                pc = 0x80115fd8;
            }
            0x80115fd8 => {
                c.r[30] = 0x3cbb;
                pc = 0x80115fdc;
            }
            0x80115fdc => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x3;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80115ff0; f_80106d3c(c);
                pc = 0x80115ff0;
            }
            0x80115ff0 => {
                c.r[3] = 0x1;
                c.lr = 0x80115ff8; f_801069fc(c);
                pc = 0x80115ff8;
            }
            0x80115ff8 => {
                pc = 0x8011614c; continue;
                pc = 0x80115ffc;
            }
            0x80115ffc => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x2d;
                c.lr = 0x80116008; f_80132a38(c);
                pc = 0x80116008;
            }
            0x80116008 => {
                c.r[3] = 0x3;
                c.r[4] = 0x3cbc;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x8011601c; f_80106d3c(c);
                pc = 0x8011601c;
            }
            0x8011601c => {
                c.r[3] = 0x1;
                c.lr = 0x80116024; f_801069fc(c);
                pc = 0x80116024;
            }
            0x80116024 => {
                c.r[4] = c.r[28].rotate_left(0) & 0xffff;
                c.r[3] = 0x0;
                c.r[5] = 0x1;
                c.r[6] = 0xffffffff;
                c.lr = 0x80116038; f_80129a78(c);
                pc = 0x80116038;
            }
            0x80116038 => {
                c.r[3] = 0x3;
                c.r[4] = 0x3cb6;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x8011604c; f_80106d3c(c);
                pc = 0x8011604c;
            }
            0x8011604c => {
                c.r[3] = 0x1;
                c.lr = 0x80116054; f_801069fc(c);
                pc = 0x80116054;
            }
            0x80116054 => {
                c.cmps(0, c.r[28], 0x21d);
                if c.crbit(2) { pc = 0x8011608c; continue; }
                pc = 0x8011605c;
            }
            0x8011605c => {
                if !c.crbit(0) { pc = 0x80116078; continue; }
                pc = 0x80116060;
            }
            0x80116060 => {
                c.cmps(0, c.r[28], 0x21b);
                if c.crbit(2) { pc = 0x80116094; continue; }
                pc = 0x80116068;
            }
            0x80116068 => {
                if !c.crbit(0) { pc = 0x8011609c; continue; }
                pc = 0x8011606c;
            }
            0x8011606c => {
                c.cmps(0, c.r[28], 0x21a);
                if !c.crbit(0) { pc = 0x80116084; continue; }
                pc = 0x80116074;
            }
            0x80116074 => {
                pc = 0x801160a8; continue;
                pc = 0x80116078;
            }
            0x80116078 => {
                c.cmps(0, c.r[28], 0x223);
                if c.crbit(2) { pc = 0x801160a4; continue; }
                pc = 0x80116080;
            }
            0x80116080 => {
                pc = 0x801160a8; continue;
                pc = 0x80116084;
            }
            0x80116084 => {
                c.r[30] = 0x3b33;
                pc = 0x801160a8; continue;
                pc = 0x8011608c;
            }
            0x8011608c => {
                c.r[30] = 0x3b35;
                pc = 0x801160a8; continue;
                pc = 0x80116094;
            }
            0x80116094 => {
                c.r[30] = 0x3b39;
                pc = 0x801160a8; continue;
                pc = 0x8011609c;
            }
            0x8011609c => {
                c.r[30] = 0x3b37;
                pc = 0x801160a8; continue;
                pc = 0x801160a4;
            }
            0x801160a4 => {
                c.r[30] = 0x44c4;
                pc = 0x801160a8;
            }
            0x801160a8 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x3;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x801160bc; f_80106d3c(c);
                pc = 0x801160bc;
            }
            0x801160bc => {
                c.lr = 0x801160c0; f_8001e184(c);
                pc = 0x801160c0;
            }
            0x801160c0 => {
                c.r[31] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[3] = 0x1;
                c.lr = 0x801160cc; f_801069fc(c);
                pc = 0x801160cc;
            }
            0x801160cc => {
                c.cmps(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x801160dc; continue; }
                pc = 0x801160d4;
            }
            0x801160d4 => {
                c.r[3] = 0x0;
                pc = 0x80116150; continue;
                pc = 0x801160dc;
            }
            0x801160dc => {
                c.cmps(0, c.r[28], 0x21d);
                if c.crbit(2) { pc = 0x80116114; continue; }
                pc = 0x801160e4;
            }
            0x801160e4 => {
                if !c.crbit(0) { pc = 0x80116100; continue; }
                pc = 0x801160e8;
            }
            0x801160e8 => {
                c.cmps(0, c.r[28], 0x21b);
                if c.crbit(2) { pc = 0x8011611c; continue; }
                pc = 0x801160f0;
            }
            0x801160f0 => {
                if !c.crbit(0) { pc = 0x80116124; continue; }
                pc = 0x801160f4;
            }
            0x801160f4 => {
                c.cmps(0, c.r[28], 0x21a);
                if !c.crbit(0) { pc = 0x8011610c; continue; }
                pc = 0x801160fc;
            }
            0x801160fc => {
                pc = 0x80116130; continue;
                pc = 0x80116100;
            }
            0x80116100 => {
                c.cmps(0, c.r[28], 0x223);
                if c.crbit(2) { pc = 0x8011612c; continue; }
                pc = 0x80116108;
            }
            0x80116108 => {
                pc = 0x80116130; continue;
                pc = 0x8011610c;
            }
            0x8011610c => {
                c.r[30] = 0x3b34;
                pc = 0x80116130; continue;
                pc = 0x80116114;
            }
            0x80116114 => {
                c.r[30] = 0x3b36;
                pc = 0x80116130; continue;
                pc = 0x8011611c;
            }
            0x8011611c => {
                c.r[30] = 0x3b30;
                pc = 0x80116130; continue;
                pc = 0x80116124;
            }
            0x80116124 => {
                c.r[30] = 0x3b38;
                pc = 0x80116130; continue;
                pc = 0x8011612c;
            }
            0x8011612c => {
                c.r[30] = 0x44c5;
                pc = 0x80116130;
            }
            0x80116130 => {
                c.r[4] = c.r[30] | c.r[30];
                c.r[3] = 0x3;
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.lr = 0x80116144; f_80106d3c(c);
                pc = 0x80116144;
            }
            0x80116144 => {
                c.r[3] = 0x1;
                c.lr = 0x8011614c; f_801069fc(c);
                pc = 0x8011614c;
            }
            0x8011614c => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80116150;
            }
            0x80116150 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80115e6c: bad pc {:#x}", pc),
        }
    }
}

/// floorEventCtrlTresure (0x80116164, 780 bytes)
#[inline(never)]
pub fn f_80116164(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116164) { return; }
        if c.prof { crate::prof::enter(c, 0x80116164); f_80116164_b(c, 0x80116164); crate::prof::exit(c, 0x80116164); return; }
    }
    f_80116164_b(c, 0x80116164)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116164_at(c: &mut Cpu, pc: u32) { f_80116164_b(c, pc) }
#[inline(always)]
fn f_80116164_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116164;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116164, pc); }
        match pc {
            0x80116164 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[23] = c.r[5] | c.r[5];
                c.r[30] = c.r[29].rotate_left(0) & 0x7fff0000;
                c.r[31] = 0x0;
                c.r[0] = c.r[30].wrapping_add(0x80010000);
                c.cmpu(0, c.r[0], 0x0);
                c.r[26] = c.r[31] | c.r[31];
                if c.crbit(2) { pc = 0x8011619c; continue; }
                pc = 0x80116198;
            }
            0x80116198 => {
                pc = 0x80116200; continue;
                pc = 0x8011619c;
            }
            0x8011619c => {
                c.r[25] = c.r[31] | c.r[31];
                c.r[24] = c.r[31] | c.r[31];
                c.r[27] = c.r[29].rotate_left(0) & 0x1ff;
                pc = 0x801161dc; continue;
                pc = 0x801161ac;
            }
            0x801161ac => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff869c));
                c.r[31] = c.r[0].wrapping_add(c.r[24]);
                c.lr = 0x801161b8; f_800ff56c(c);
                pc = 0x801161b8;
            }
            0x801161b8 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x4));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x801161d4; continue; }
                pc = 0x801161c4;
            }
            0x801161c4 => {
                c.r[0] = c.r[26] | c.r[26];
                c.r[26] = c.r[26].wrapping_add(0x1);
                c.cmpu(0, c.r[27], c.r[0]);
                if c.crbit(2) { pc = 0x801161ec; continue; }
                pc = 0x801161d4;
            }
            0x801161d4 => {
                c.r[24] = c.r[24].wrapping_add(0x1c);
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x801161dc;
            }
            0x801161dc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8698));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[25], c.r[0]);
                if c.crbit(0) { pc = 0x801161ac; continue; }
                pc = 0x801161ec;
            }
            0x801161ec => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8698));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[25], c.r[0]);
                if !c.crbit(2) { pc = 0x80116200; continue; }
                pc = 0x801161fc;
            }
            0x801161fc => {
                c.r[31] = 0x0;
                pc = 0x80116200;
            }
            0x80116200 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80116210; continue; }
                pc = 0x80116208;
            }
            0x80116208 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011645c; continue;
                pc = 0x80116210;
            }
            0x80116210 => {
                c.r[0] = c.r[23].rotate_left(0) & 0xff;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80116298; continue; }
                pc = 0x8011621c;
            }
            0x8011621c => {
                if !c.crbit(0) { pc = 0x8011622c; continue; }
                pc = 0x80116220;
            }
            0x80116220 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x80116238; continue; }
                pc = 0x80116228;
            }
            0x80116228 => {
                pc = 0x80116458; continue;
                pc = 0x8011622c;
            }
            0x8011622c => {
                c.cmps(0, c.r[0], 0x3);
                if !c.crbit(0) { pc = 0x80116458; continue; }
                pc = 0x80116234;
            }
            0x80116234 => {
                pc = 0x801162ec; continue;
                pc = 0x80116238;
            }
            0x80116238 => {
                c.r[0] = c.lbz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80116258; continue; }
                pc = 0x80116248;
            }
            0x80116248 => {
                if c.crbit(0) { pc = 0x80116458; continue; }
                pc = 0x8011624c;
            }
            0x8011624c => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x80116458; continue; }
                pc = 0x80116254;
            }
            0x80116254 => {
                pc = 0x80116284; continue;
                pc = 0x80116258;
            }
            0x80116258 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80116270; f_8018b76c(c);
                pc = 0x80116270;
            }
            0x80116270 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.lr = 0x80116280; f_8018c7c8(c);
                pc = 0x80116280;
            }
            0x80116280 => {
                pc = 0x80116458; continue;
                pc = 0x80116284;
            }
            0x80116284 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.lr = 0x80116294; f_8018c1e8(c);
                pc = 0x80116294;
            }
            0x80116294 => {
                pc = 0x80116458; continue;
                pc = 0x80116298;
            }
            0x80116298 => {
                c.r[0] = c.lbz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x801162b8; continue; }
                pc = 0x801162a8;
            }
            0x801162a8 => {
                if c.crbit(0) { pc = 0x80116458; continue; }
                pc = 0x801162ac;
            }
            0x801162ac => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x80116458; continue; }
                pc = 0x801162b4;
            }
            0x801162b4 => {
                pc = 0x801162d0; continue;
                pc = 0x801162b8;
            }
            0x801162b8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x801162d0; f_8018b76c(c);
                pc = 0x801162d0;
            }
            0x801162d0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x801162e8; f_8018b76c(c);
                pc = 0x801162e8;
            }
            0x801162e8 => {
                pc = 0x80116458; continue;
                pc = 0x801162ec;
            }
            0x801162ec => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x6));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116310; continue; }
                pc = 0x801162f8;
            }
            0x801162f8 => {
                c.lr = 0x801162fc; f_801902e0(c);
                pc = 0x801162fc;
            }
            0x801162fc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80116310; continue; }
                pc = 0x80116308;
            }
            0x80116308 => {
                c.r[3] = 0x0;
                pc = 0x8011645c; continue;
                pc = 0x80116310;
            }
            0x80116310 => {
                c.r[0] = c.lbz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x8011632c; continue; }
                pc = 0x80116320;
            }
            0x80116320 => {
                if c.crbit(0) { pc = 0x8011635c; continue; }
                pc = 0x80116324;
            }
            0x80116324 => {
                c.cmps(0, c.r[0], 0x4);
                pc = 0x8011635c; continue;
                pc = 0x8011632c;
            }
            0x8011632c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80116344; f_8018b76c(c);
                pc = 0x80116344;
            }
            0x80116344 => {
                c.r[3] = 0x3c2;
                c.lr = 0x8011634c; f_80166a28(c);
                pc = 0x8011634c;
            }
            0x8011634c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.lr = 0x8011635c; f_8018b07c(c);
                pc = 0x8011635c;
            }
            0x8011635c => {
                c.r[0] = c.r[30].wrapping_add(0x80010000);
                c.r[30] = 0x0;
                c.cmpu(0, c.r[0], 0x0);
                c.r[26] = c.r[30] | c.r[30];
                if c.crbit(2) { pc = 0x80116374; continue; }
                pc = 0x80116370;
            }
            0x80116370 => {
                pc = 0x801163d8; continue;
                pc = 0x80116374;
            }
            0x80116374 => {
                c.r[27] = c.r[30] | c.r[30];
                c.r[24] = c.r[30] | c.r[30];
                c.r[25] = c.r[29].rotate_left(0) & 0x1ff;
                pc = 0x801163b4; continue;
                pc = 0x80116384;
            }
            0x80116384 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff869c));
                c.r[30] = c.r[0].wrapping_add(c.r[24]);
                c.lr = 0x80116390; f_800ff56c(c);
                pc = 0x80116390;
            }
            0x80116390 => {
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x4));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x801163ac; continue; }
                pc = 0x8011639c;
            }
            0x8011639c => {
                c.r[0] = c.r[26] | c.r[26];
                c.r[26] = c.r[26].wrapping_add(0x1);
                c.cmpu(0, c.r[25], c.r[0]);
                if c.crbit(2) { pc = 0x801163c4; continue; }
                pc = 0x801163ac;
            }
            0x801163ac => {
                c.r[24] = c.r[24].wrapping_add(0x1c);
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x801163b4;
            }
            0x801163b4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8698));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[27], c.r[0]);
                if c.crbit(0) { pc = 0x80116384; continue; }
                pc = 0x801163c4;
            }
            0x801163c4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8698));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[27], c.r[0]);
                if !c.crbit(2) { pc = 0x801163d8; continue; }
                pc = 0x801163d4;
            }
            0x801163d4 => {
                c.r[30] = 0x0;
                pc = 0x801163d8;
            }
            0x801163d8 => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8011643c; continue; }
                pc = 0x801163e0;
            }
            0x801163e0 => {
                c.r[0] = c.lbz(c.r[30]);
                c.r[0] = c.r[0].rotate_left(27) & 0x7;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80116400; continue; }
                pc = 0x801163f0;
            }
            0x801163f0 => {
                if c.crbit(0) { pc = 0x8011643c; continue; }
                pc = 0x801163f4;
            }
            0x801163f4 => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x8011643c; continue; }
                pc = 0x801163fc;
            }
            0x801163fc => {
                pc = 0x8011642c; continue;
                pc = 0x80116400;
            }
            0x80116400 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.r[7] = 0x1;
                c.lr = 0x80116418; f_8018b76c(c);
                pc = 0x80116418;
            }
            0x80116418 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.lr = 0x80116428; f_8018c7c8(c);
                pc = 0x80116428;
            }
            0x80116428 => {
                pc = 0x8011643c; continue;
                pc = 0x8011642c;
            }
            0x8011642c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.lr = 0x8011643c; f_8018c1e8(c);
                pc = 0x8011643c;
            }
            0x8011643c => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x6));
                c.lr = 0x80116444; f_80190528(c);
                pc = 0x80116444;
            }
            0x80116444 => {
                c.r[0] = c.lbz(c.r[31]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[5] = c.lbz(c.r[31].wrapping_add(0x1));
                c.r[3] = c.r[0].rotate_left(30) & 0x7;
                c.lr = 0x80116458; f_80115e6c(c);
                pc = 0x80116458;
            }
            0x80116458 => {
                c.r[3] = 0x0;
                pc = 0x8011645c;
            }
            0x8011645c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80116164: bad pc {:#x}", pc),
        }
    }
}

/// floorEventCtrlElevator (0x80116470, 1256 bytes)
#[inline(never)]
pub fn f_80116470(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116470) { return; }
        if c.prof { crate::prof::enter(c, 0x80116470); f_80116470_b(c, 0x80116470); crate::prof::exit(c, 0x80116470); return; }
    }
    f_80116470_b(c, 0x80116470)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116470_at(c: &mut Cpu, pc: u32) { f_80116470_b(c, pc) }
#[inline(always)]
fn f_80116470_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116470;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116470, pc); }
        match pc {
            0x80116470 => {
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[24] = c.r[3] | c.r[3];
                c.r[25] = c.r[4] | c.r[4];
                c.r[23] = c.r[5] | c.r[5];
                c.r[26] = c.r[6] | c.r[6];
                c.r[27] = c.r[7] | c.r[7];
                { let ea = c.r[2].wrapping_add(0xffff9900); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[28] = 0xffffffff;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff86a8));
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[25], c.r[0]);
                if c.crbit(0) { pc = 0x801164c0; continue; }
                pc = 0x801164b8;
            }
            0x801164b8 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011693c; continue;
                pc = 0x801164c0;
            }
            0x801164c0 => {
                c.r[0] = c.r[25].wrapping_mul(0x18);
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff86ac));
                c.r[30] = c.r[4].wrapping_add(c.r[0]);
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x14));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x801164e0; continue; }
                pc = 0x801164d8;
            }
            0x801164d8 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011693c; continue;
                pc = 0x801164e0;
            }
            0x801164e0 => {
                c.lr = 0x801164e4; f_80113f6c(c);
                pc = 0x801164e4;
            }
            0x801164e4 => {
                c.r[29] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x801164f8; continue; }
                pc = 0x801164f0;
            }
            0x801164f0 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011693c; continue;
                pc = 0x801164f8;
            }
            0x801164f8 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x80116500; f_80115c48(c);
                pc = 0x80116500;
            }
            0x80116500 => {
                c.lr = 0x80116504; f_80115a80(c);
                pc = 0x80116504;
            }
            0x80116504 => {
                c.r[31] = c.r[23].rotate_left(0) & 0xffff;
                c.r[23] = c.r[3] | c.r[3];
                c.cmps(0, c.r[31], 0x83);
                if !c.crbit(0) { pc = 0x80116534; continue; }
                pc = 0x80116514;
            }
            0x80116514 => {
                c.cmps(0, c.r[31], 0x3);
                if !c.crbit(0) { pc = 0x80116528; continue; }
                pc = 0x8011651c;
            }
            0x8011651c => {
                c.cmps(0, c.r[31], 0x1);
                if !c.crbit(0) { pc = 0x80116540; continue; }
                pc = 0x80116524;
            }
            0x80116524 => {
                pc = 0x80116798; continue;
                pc = 0x80116528;
            }
            0x80116528 => {
                c.cmps(0, c.r[31], 0x81);
                if !c.crbit(0) { pc = 0x80116540; continue; }
                pc = 0x80116530;
            }
            0x80116530 => {
                pc = 0x80116798; continue;
                pc = 0x80116534;
            }
            0x80116534 => {
                c.cmps(0, c.r[31], 0xc0);
                if c.crbit(2) { pc = 0x80116750; continue; }
                pc = 0x8011653c;
            }
            0x8011653c => {
                pc = 0x80116798; continue;
                pc = 0x80116540;
            }
            0x80116540 => {
                c.r[0] = c.lbz(c.r[30]);
                c.r[4] = c.r[0] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[4], 0x0);
                if c.crbit(0) { pc = 0x80116590; continue; }
                pc = 0x80116550;
            }
            0x80116550 => {
                c.cmpu(0, c.r[29], 0x0);
                { let ea = c.r[1].wrapping_add(0x8); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                if c.crbit(2) { pc = 0x80116590; continue; }
                pc = 0x8011655c;
            }
            0x8011655c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80116564; f_800ecca8(c);
                pc = 0x80116564;
            }
            0x80116564 => {
                c.f[1] = c.f[31];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80116570; f_800eca78(c);
                pc = 0x80116570;
            }
            0x80116570 => {
                { let ea = c.r[2].wrapping_add(0xffff9904); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8011657c; f_800ec9dc(c);
                pc = 0x8011657c;
            }
            0x8011657c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x80116588; f_800ecb74(c);
                pc = 0x80116588;
            }
            0x80116588 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80116590; f_800ec990(c);
                pc = 0x80116590;
            }
            0x80116590 => {
                c.r[24] = c.r[31].rotate_left(0) & 0x80;
                c.cmps(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x801165a4; continue; }
                pc = 0x8011659c;
            }
            0x8011659c => {
                c.r[3] = 0x44;
                c.lr = 0x801165a4; f_80166a28(c);
                pc = 0x801165a4;
            }
            0x801165a4 => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x801165b8; continue; }
                pc = 0x801165ac;
            }
            0x801165ac => {
                pc = 0x801165cc; continue;
                pc = 0x801165b0;
            }
            0x801165b0 => {
                pc = 0x801165b8; continue;
                pc = 0x801165b4;
            }
            0x801165b4 => {
                c.lr = 0x801165b8; crate::stubs::f_800f0308(c);
                pc = 0x801165b8;
            }
            0x801165b8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801165c0; f_800ec960(c);
                pc = 0x801165c0;
            }
            0x801165c0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801165b4; continue; }
                pc = 0x801165cc;
            }
            0x801165cc => {
                c.r[4] = c.lbz(c.r[30].wrapping_add(0x8));
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801165d8; f_800ee150(c);
                pc = 0x801165d8;
            }
            0x801165d8 => {
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.r[25] = c.r[3] | c.r[3];
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x801165ec; f_800ee3bc(c);
                pc = 0x801165ec;
            }
            0x801165ec => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x801165f4; f_800ee828(c);
                pc = 0x801165f4;
            }
            0x801165f4 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x1;
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8011664c; continue; }
                pc = 0x80116600;
            }
            0x80116600 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[1].wrapping_add(0xc);
                c.r[5] = 0x1;
                c.lr = 0x80116614; f_8018aacc(c);
                pc = 0x80116614;
            }
            0x80116614 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.lr = 0x80116624; f_8018a280(c);
                pc = 0x80116624;
            }
            0x80116624 => {
                { let ea = c.r[2].wrapping_add(0xffff9900); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[26] | c.r[26];
                { let ea = c.r[2].wrapping_add(0xffff9904); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x80116638; f_8018805c(c);
                pc = 0x80116638;
            }
            0x80116638 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.lr = 0x80116648; f_8018a280(c);
                pc = 0x80116648;
            }
            0x80116648 => {
                pc = 0x801166c4; continue;
                pc = 0x8011664c;
            }
            0x8011664c => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x80116658; f_80184470(c);
                pc = 0x80116658;
            }
            0x80116658 => {
                c.r[4] = c.lbz(c.r[30].wrapping_add(0x8));
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80116664; f_800ee150(c);
                pc = 0x80116664;
            }
            0x80116664 => {
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.r[25] = c.r[3] | c.r[3];
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80116678; f_800ee3bc(c);
                pc = 0x80116678;
            }
            0x80116678 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80116680; f_800ee828(c);
                pc = 0x80116680;
            }
            0x80116680 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[1].wrapping_add(0xc);
                c.lr = 0x80116690; f_8018c0a8(c);
                pc = 0x80116690;
            }
            0x80116690 => {
                { let ea = c.r[1].wrapping_add(0x14); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[26] | c.r[26];
                { let ea = c.r[2].wrapping_add(0xffff9908); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.r[27] | c.r[27];
                c.r[6] = c.r[1].wrapping_add(0xc);
                c.r[5] = 0x1;
                { let v = s(c.f[1] + c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x14); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x801166b4; f_8018aacc(c);
                pc = 0x801166b4;
            }
            0x801166b4 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x1;
                c.lr = 0x801166c4; f_8018a280(c);
                pc = 0x801166c4;
            }
            0x801166c4 => {
                c.r[0] = c.lbz(c.r[30].wrapping_add(0x1));
                c.r[4] = c.r[0] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[4], 0x0);
                if c.crbit(0) { pc = 0x80116714; continue; }
                pc = 0x801166d4;
            }
            0x801166d4 => {
                c.cmpu(0, c.r[29], 0x0);
                { let ea = c.r[1].wrapping_add(0x8); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                if c.crbit(2) { pc = 0x80116714; continue; }
                pc = 0x801166e0;
            }
            0x801166e0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801166e8; f_800ecca8(c);
                pc = 0x801166e8;
            }
            0x801166e8 => {
                c.f[1] = c.f[31];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801166f4; f_800eca78(c);
                pc = 0x801166f4;
            }
            0x801166f4 => {
                { let ea = c.r[2].wrapping_add(0xffff9904); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80116700; f_800ec9dc(c);
                pc = 0x80116700;
            }
            0x80116700 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8011670c; f_800ecb74(c);
                pc = 0x8011670c;
            }
            0x8011670c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80116714; f_800ec990(c);
                pc = 0x80116714;
            }
            0x80116714 => {
                c.cmps(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x80116724; continue; }
                pc = 0x8011671c;
            }
            0x8011671c => {
                c.r[3] = 0x44;
                c.lr = 0x80116724; f_80166a28(c);
                pc = 0x80116724;
            }
            0x80116724 => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x80116738; continue; }
                pc = 0x8011672c;
            }
            0x8011672c => {
                pc = 0x801168e8; continue;
                pc = 0x80116730;
            }
            0x80116730 => {
                pc = 0x80116738; continue;
                pc = 0x80116734;
            }
            0x80116734 => {
                c.lr = 0x80116738; crate::stubs::f_800f0308(c);
                pc = 0x80116738;
            }
            0x80116738 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80116740; f_800ec960(c);
                pc = 0x80116740;
            }
            0x80116740 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80116734; continue; }
                pc = 0x8011674c;
            }
            0x8011674c => {
                pc = 0x801168e8; continue;
                pc = 0x80116750;
            }
            0x80116750 => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x80116764; continue; }
                pc = 0x80116758;
            }
            0x80116758 => {
                pc = 0x80116778; continue;
                pc = 0x8011675c;
            }
            0x8011675c => {
                pc = 0x80116764; continue;
                pc = 0x80116760;
            }
            0x80116760 => {
                c.lr = 0x80116764; crate::stubs::f_800f0308(c);
                pc = 0x80116764;
            }
            0x80116764 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8011676c; f_800ec960(c);
                pc = 0x8011676c;
            }
            0x8011676c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80116760; continue; }
                pc = 0x80116778;
            }
            0x80116778 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x80;
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801168e8; continue; }
                pc = 0x80116784;
            }
            0x80116784 => {
                c.r[3] = 0x45;
                c.lr = 0x8011678c; f_801669bc(c);
                pc = 0x8011678c;
            }
            0x8011678c => {
                c.r[3] = 0x46;
                c.lr = 0x80116794; f_80166a28(c);
                pc = 0x80116794;
            }
            0x80116794 => {
                pc = 0x801168e8; continue;
                pc = 0x80116798;
            }
            0x80116798 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x4;
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801167b0; continue; }
                pc = 0x801167a4;
            }
            0x801167a4 => {
                c.r[28] = c.lbz(c.r[30].wrapping_add(0x5));
                c.r[28] = c.r[28] as u8 as i8 as i32 as u32;
                pc = 0x801167c4; continue;
                pc = 0x801167b0;
            }
            0x801167b0 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x8;
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801167c4; continue; }
                pc = 0x801167bc;
            }
            0x801167bc => {
                c.r[28] = c.lbz(c.r[30].wrapping_add(0x6));
                c.r[28] = c.r[28] as u8 as i8 as i32 as u32;
                pc = 0x801167c4;
            }
            0x801167c4 => {
                c.r[0] = c.r[28] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x801167d8; continue; }
                pc = 0x801167d0;
            }
            0x801167d0 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011693c; continue;
                pc = 0x801167d8;
            }
            0x801167d8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801167e0; f_800ec1bc(c);
                pc = 0x801167e0;
            }
            0x801167e0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801167f4; continue; }
                pc = 0x801167ec;
            }
            0x801167ec => {
                c.r[3] = 0xffffffff;
                pc = 0x8011693c; continue;
                pc = 0x801167f4;
            }
            0x801167f4 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x20;
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80116830; continue; }
                pc = 0x80116800;
            }
            0x80116800 => {
                c.r[6] = c.lwz(c.r[30].wrapping_add(0x14));
                c.r[3] = c.r[26] | c.r[26];
                c.r[7] = c.lbz(c.r[30].wrapping_add(0x8));
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[23] | c.r[23];
                c.lr = 0x80116818; f_801845e4(c);
                pc = 0x80116818;
            }
            0x80116818 => {
                { let ea = c.r[2].wrapping_add(0xffff9900); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80116830; f_801860f8(c);
                pc = 0x80116830;
            }
            0x80116830 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x10;
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80116868; continue; }
                pc = 0x8011683c;
            }
            0x8011683c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] as u16 as i16 as i32 as u32;
                c.lr = 0x80116848; f_800ecca8(c);
                pc = 0x80116848;
            }
            0x80116848 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x0;
                c.lr = 0x80116858; f_800ec4d0(c);
                pc = 0x80116858;
            }
            0x80116858 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff990c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80116868;
            }
            0x80116868 => {
                c.cmpu(0, c.r[29], 0x0);
                { let ea = c.r[1].wrapping_add(0x8); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                if c.crbit(2) { pc = 0x801168ac; continue; }
                pc = 0x80116874;
            }
            0x80116874 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] as u16 as i16 as i32 as u32;
                c.lr = 0x80116880; f_800ecca8(c);
                pc = 0x80116880;
            }
            0x80116880 => {
                c.f[1] = c.f[31];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8011688c; f_800eca78(c);
                pc = 0x8011688c;
            }
            0x8011688c => {
                { let ea = c.r[2].wrapping_add(0xffff9904); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80116898; f_800ec9dc(c);
                pc = 0x80116898;
            }
            0x80116898 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x801168a4; f_800ecb74(c);
                pc = 0x801168a4;
            }
            0x801168a4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801168ac; f_800ec990(c);
                pc = 0x801168ac;
            }
            0x801168ac => {
                c.r[0] = c.r[31].rotate_left(0) & 0x40;
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801168d4; continue; }
                pc = 0x801168b8;
            }
            0x801168b8 => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[25] | c.r[25];
                c.r[6] = c.r[26] | c.r[26];
                c.r[7] = c.r[27] | c.r[27];
                c.r[5] = 0xc0;
                c.lr = 0x801168d0; f_80116470(c);
                pc = 0x801168d0;
            }
            0x801168d0 => {
                pc = 0x801168e8; continue;
                pc = 0x801168d4;
            }
            0x801168d4 => {
                c.r[0] = c.r[31].rotate_left(0) & 0x80;
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801168e8; continue; }
                pc = 0x801168e0;
            }
            0x801168e0 => {
                c.r[3] = 0x45;
                c.lr = 0x801168e8; f_80166a28(c);
                pc = 0x801168e8;
            }
            0x801168e8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x0;
                c.lr = 0x801168f8; f_800ec4d0(c);
                pc = 0x801168f8;
            }
            0x801168f8 => {
                c.lr = 0x801168fc; f_800d37cc(c);
                pc = 0x801168fc;
            }
            0x801168fc => {
                c.r[3] = c.r[3] ^ 0x80000000;
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x1c), c.r[3]);
                { let ea = c.r[2].wrapping_add(0xffff9918); c.f[3] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x18), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9914); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x18); c.f[0] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[0] - c.f[3]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9910); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] * c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x8011693c;
            }
            0x8011693c => {
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[31] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x60);
                if true { return; }
                return;
            }
            _ => panic!("f_80116470: bad pc {:#x}", pc),
        }
    }
}

/// floorEventCtrlDoor (0x80116958, 984 bytes)
#[inline(never)]
pub fn f_80116958(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116958) { return; }
        if c.prof { crate::prof::enter(c, 0x80116958); f_80116958_b(c, 0x80116958); crate::prof::exit(c, 0x80116958); return; }
    }
    f_80116958_b(c, 0x80116958)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116958_at(c: &mut Cpu, pc: u32) { f_80116958_b(c, pc) }
#[inline(always)]
fn f_80116958_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116958;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116958, pc); }
        match pc {
            0x80116958 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                { let ea = c.r[2].wrapping_add(0xffff9900); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[30] = 0xffffffff;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff86a8));
                c.r[29] = 0x0;
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x8011699c; continue; }
                pc = 0x80116994;
            }
            0x80116994 => {
                c.r[3] = 0xffffffff;
                pc = 0x80116d14; continue;
                pc = 0x8011699c;
            }
            0x8011699c => {
                c.r[31] = c.r[5].rotate_left(0) & 0xff;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff86ac));
                c.r[0] = c.r[4].wrapping_mul(0x18);
                c.cmps(0, c.r[31], 0x2);
                c.r[27] = c.r[3].wrapping_add(c.r[0]);
                if c.crbit(2) { pc = 0x80116a3c; continue; }
                pc = 0x801169b4;
            }
            0x801169b4 => {
                if !c.crbit(0) { pc = 0x801169c8; continue; }
                pc = 0x801169b8;
            }
            0x801169b8 => {
                c.cmps(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x801169d4; continue; }
                pc = 0x801169c0;
            }
            0x801169c0 => {
                if !c.crbit(0) { pc = 0x80116a1c; continue; }
                pc = 0x801169c4;
            }
            0x801169c4 => {
                pc = 0x80116a7c; continue;
                pc = 0x801169c8;
            }
            0x801169c8 => {
                c.cmps(0, c.r[31], 0x4);
                if !c.crbit(0) { pc = 0x80116a7c; continue; }
                pc = 0x801169d0;
            }
            0x801169d0 => {
                pc = 0x80116a60; continue;
                pc = 0x801169d4;
            }
            0x801169d4 => {
                c.r[3] = c.lhz(c.r[27].wrapping_add(0xe));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801169f8; continue; }
                pc = 0x801169e0;
            }
            0x801169e0 => {
                c.lr = 0x801169e4; f_801902e0(c);
                pc = 0x801169e4;
            }
            0x801169e4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801169f8; continue; }
                pc = 0x801169f0;
            }
            0x801169f0 => {
                c.r[3] = 0x0;
                pc = 0x80116d14; continue;
                pc = 0x801169f8;
            }
            0x801169f8 => {
                c.r[3] = c.lhz(c.r[27].wrapping_add(0x10));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116a1c; continue; }
                pc = 0x80116a04;
            }
            0x80116a04 => {
                c.lr = 0x80116a08; f_801902e0(c);
                pc = 0x80116a08;
            }
            0x80116a08 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80116a1c; continue; }
                pc = 0x80116a14;
            }
            0x80116a14 => {
                c.r[3] = 0x1;
                pc = 0x80116d14; continue;
                pc = 0x80116a1c;
            }
            0x80116a1c => {
                c.r[3] = c.lhz(c.r[27].wrapping_add(0xe));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116a2c; continue; }
                pc = 0x80116a28;
            }
            0x80116a28 => {
                c.lr = 0x80116a2c; f_80190528(c);
                pc = 0x80116a2c;
            }
            0x80116a2c => {
                c.r[3] = c.lha(c.r[27].wrapping_add(0xa));
                c.r[4] = 0x0;
                c.lr = 0x80116a38; f_801caa08(c);
                pc = 0x80116a38;
            }
            0x80116a38 => {
                pc = 0x80116a7c; continue;
                pc = 0x80116a3c;
            }
            0x80116a3c => {
                c.r[3] = c.lhz(c.r[27].wrapping_add(0xe));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116a60; continue; }
                pc = 0x80116a48;
            }
            0x80116a48 => {
                c.lr = 0x80116a4c; f_801902e0(c);
                pc = 0x80116a4c;
            }
            0x80116a4c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80116a60; continue; }
                pc = 0x80116a58;
            }
            0x80116a58 => {
                c.r[3] = 0x0;
                pc = 0x80116d14; continue;
                pc = 0x80116a60;
            }
            0x80116a60 => {
                c.r[3] = c.lhz(c.r[27].wrapping_add(0xe));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116a70; continue; }
                pc = 0x80116a6c;
            }
            0x80116a6c => {
                c.lr = 0x80116a70; f_801903b0(c);
                pc = 0x80116a70;
            }
            0x80116a70 => {
                c.r[3] = c.lha(c.r[27].wrapping_add(0xa));
                c.r[4] = 0x1;
                c.lr = 0x80116a7c; f_801caa08(c);
                pc = 0x80116a7c;
            }
            0x80116a7c => {
                c.r[4] = c.lwz(c.r[27].wrapping_add(0x14));
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x80116a90; continue; }
                pc = 0x80116a88;
            }
            0x80116a88 => {
                c.r[3] = 0xffffffff;
                pc = 0x80116d14; continue;
                pc = 0x80116a90;
            }
            0x80116a90 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80116a98; f_80113f6c(c);
                pc = 0x80116a98;
            }
            0x80116a98 => {
                c.r[28] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x80116aac; continue; }
                pc = 0x80116aa4;
            }
            0x80116aa4 => {
                c.r[3] = 0xffffffff;
                pc = 0x80116d14; continue;
                pc = 0x80116aac;
            }
            0x80116aac => {
                c.cmps(0, c.r[31], 0x2);
                if c.crbit(2) { pc = 0x80116b68; continue; }
                pc = 0x80116ab4;
            }
            0x80116ab4 => {
                if !c.crbit(0) { pc = 0x80116ac8; continue; }
                pc = 0x80116ab8;
            }
            0x80116ab8 => {
                c.cmps(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80116ad4; continue; }
                pc = 0x80116ac0;
            }
            0x80116ac0 => {
                if !c.crbit(0) { pc = 0x80116b1c; continue; }
                pc = 0x80116ac4;
            }
            0x80116ac4 => {
                pc = 0x80116bf8; continue;
                pc = 0x80116ac8;
            }
            0x80116ac8 => {
                c.cmps(0, c.r[31], 0x4);
                if !c.crbit(0) { pc = 0x80116bf8; continue; }
                pc = 0x80116ad0;
            }
            0x80116ad0 => {
                pc = 0x80116bb0; continue;
                pc = 0x80116ad4;
            }
            0x80116ad4 => {
                c.r[0] = c.lbz(c.r[27].wrapping_add(0x7));
                c.r[30] = c.lbz(c.r[27]);
                c.cmps(0, c.r[0], 0x2);
                c.r[30] = c.r[30] as u8 as i8 as i32 as u32;
                if c.crbit(2) { pc = 0x80116b0c; continue; }
                pc = 0x80116ae8;
            }
            0x80116ae8 => {
                if !c.crbit(0) { pc = 0x80116af8; continue; }
                pc = 0x80116aec;
            }
            0x80116aec => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(0) { pc = 0x80116b04; continue; }
                pc = 0x80116af4;
            }
            0x80116af4 => {
                pc = 0x80116bf8; continue;
                pc = 0x80116af8;
            }
            0x80116af8 => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x80116bf8; continue; }
                pc = 0x80116b00;
            }
            0x80116b00 => {
                pc = 0x80116b14; continue;
                pc = 0x80116b04;
            }
            0x80116b04 => {
                c.r[29] = 0x44;
                pc = 0x80116bf8; continue;
                pc = 0x80116b0c;
            }
            0x80116b0c => {
                c.r[29] = 0x44;
                pc = 0x80116bf8; continue;
                pc = 0x80116b14;
            }
            0x80116b14 => {
                c.r[29] = 0x4be;
                pc = 0x80116bf8; continue;
                pc = 0x80116b1c;
            }
            0x80116b1c => {
                c.r[30] = c.lbz(c.r[27].wrapping_add(0x2));
                c.r[30] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(0) { pc = 0x80116bf8; continue; }
                pc = 0x80116b2c;
            }
            0x80116b2c => {
                c.r[30] = c.lbz(c.r[27]);
                c.r[30] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(0) { pc = 0x80116bf8; continue; }
                pc = 0x80116b3c;
            }
            0x80116b3c => {
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80116b44; f_800ecca8(c);
                pc = 0x80116b44;
            }
            0x80116b44 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x0;
                c.lr = 0x80116b54; f_800ec4d0(c);
                pc = 0x80116b54;
            }
            0x80116b54 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff990c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80116bf8; continue;
                pc = 0x80116b68;
            }
            0x80116b68 => {
                c.r[0] = c.lbz(c.r[27].wrapping_add(0x7));
                c.r[30] = c.lbz(c.r[27].wrapping_add(0x1));
                c.cmps(0, c.r[0], 0x2);
                c.r[30] = c.r[30] as u8 as i8 as i32 as u32;
                if c.crbit(2) { pc = 0x80116ba0; continue; }
                pc = 0x80116b7c;
            }
            0x80116b7c => {
                if !c.crbit(0) { pc = 0x80116b8c; continue; }
                pc = 0x80116b80;
            }
            0x80116b80 => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(0) { pc = 0x80116b98; continue; }
                pc = 0x80116b88;
            }
            0x80116b88 => {
                pc = 0x80116bf8; continue;
                pc = 0x80116b8c;
            }
            0x80116b8c => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x80116bf8; continue; }
                pc = 0x80116b94;
            }
            0x80116b94 => {
                pc = 0x80116ba8; continue;
                pc = 0x80116b98;
            }
            0x80116b98 => {
                c.r[29] = 0x44;
                pc = 0x80116bf8; continue;
                pc = 0x80116ba0;
            }
            0x80116ba0 => {
                c.r[29] = 0x44;
                pc = 0x80116bf8; continue;
                pc = 0x80116ba8;
            }
            0x80116ba8 => {
                c.r[29] = 0x4be;
                pc = 0x80116bf8; continue;
                pc = 0x80116bb0;
            }
            0x80116bb0 => {
                c.r[30] = c.lbz(c.r[27].wrapping_add(0x3));
                c.r[30] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[30], 0x0);
                if !c.crbit(0) { pc = 0x80116bf8; continue; }
                pc = 0x80116bc0;
            }
            0x80116bc0 => {
                c.r[30] = c.lbz(c.r[27].wrapping_add(0x1));
                c.r[30] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[30], 0x0);
                if c.crbit(0) { pc = 0x80116bf8; continue; }
                pc = 0x80116bd0;
            }
            0x80116bd0 => {
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80116bd8; f_800ecca8(c);
                pc = 0x80116bd8;
            }
            0x80116bd8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = 0x0;
                c.lr = 0x80116be8; f_800ec4d0(c);
                pc = 0x80116be8;
            }
            0x80116be8 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff990c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] - c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80116bf8;
            }
            0x80116bf8 => {
                c.r[0] = c.r[30] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x80116c0c; continue; }
                pc = 0x80116c04;
            }
            0x80116c04 => {
                c.r[3] = 0xffffffff;
                pc = 0x80116d14; continue;
                pc = 0x80116c0c;
            }
            0x80116c0c => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80116c14; f_800ec1bc(c);
                pc = 0x80116c14;
            }
            0x80116c14 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80116c28; continue; }
                pc = 0x80116c20;
            }
            0x80116c20 => {
                c.r[3] = 0xffffffff;
                pc = 0x80116d14; continue;
                pc = 0x80116c28;
            }
            0x80116c28 => {
                c.cmpu(0, c.r[28], 0x0);
                { let ea = c.r[1].wrapping_add(0x8); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                if c.crbit(2) { pc = 0x80116c6c; continue; }
                pc = 0x80116c34;
            }
            0x80116c34 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] as u16 as i16 as i32 as u32;
                c.lr = 0x80116c40; f_800ecca8(c);
                pc = 0x80116c40;
            }
            0x80116c40 => {
                c.f[1] = c.f[31];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80116c4c; f_800eca78(c);
                pc = 0x80116c4c;
            }
            0x80116c4c => {
                { let ea = c.r[2].wrapping_add(0xffff9904); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80116c58; f_800ec9dc(c);
                pc = 0x80116c58;
            }
            0x80116c58 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = 0x0;
                c.lr = 0x80116c64; f_800ecb74(c);
                pc = 0x80116c64;
            }
            0x80116c64 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80116c6c; f_800ec990(c);
                pc = 0x80116c6c;
            }
            0x80116c6c => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x80116c7c; continue; }
                pc = 0x80116c74;
            }
            0x80116c74 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80116c7c; f_80166a28(c);
                pc = 0x80116c7c;
            }
            0x80116c7c => {
                c.cmps(0, c.r[31], 0x1);
                if c.crbit(2) { pc = 0x80116d10; continue; }
                pc = 0x80116c84;
            }
            0x80116c84 => {
                if !c.crbit(0) { pc = 0x80116c94; continue; }
                pc = 0x80116c88;
            }
            0x80116c88 => {
                c.cmps(0, c.r[31], 0x0);
                if !c.crbit(0) { pc = 0x80116c9c; continue; }
                pc = 0x80116c90;
            }
            0x80116c90 => {
                pc = 0x80116d10; continue;
                pc = 0x80116c94;
            }
            0x80116c94 => {
                c.cmps(0, c.r[31], 0x3);
                if !c.crbit(0) { pc = 0x80116d10; continue; }
                pc = 0x80116c9c;
            }
            0x80116c9c => {
                c.r[3] = 0x0;
                c.r[4] = 0x64;
                c.lr = 0x80116ca8; f_8018c558(c);
                pc = 0x80116ca8;
            }
            0x80116ca8 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.r[27] = c.r[0] | c.r[0];
                c.r[4] = 0x65;
                c.lr = 0x80116cbc; f_8018c558(c);
                pc = 0x80116cbc;
            }
            0x80116cbc => {
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x80116cc4; f_8012bca4(c);
                pc = 0x80116cc4;
            }
            0x80116cc4 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x80116cd8; continue; }
                pc = 0x80116ccc;
            }
            0x80116ccc => {
                pc = 0x80116cec; continue;
                pc = 0x80116cd0;
            }
            0x80116cd0 => {
                pc = 0x80116cd8; continue;
                pc = 0x80116cd4;
            }
            0x80116cd4 => {
                c.lr = 0x80116cd8; crate::stubs::f_800f0308(c);
                pc = 0x80116cd8;
            }
            0x80116cd8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80116ce0; f_800ec960(c);
                pc = 0x80116ce0;
            }
            0x80116ce0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80116cd4; continue; }
                pc = 0x80116cec;
            }
            0x80116cec => {
                c.lr = 0x80116cf0; f_8012bba8(c);
                pc = 0x80116cf0;
            }
            0x80116cf0 => {
                c.r[5] = c.r[27] | c.r[27];
                c.r[3] = 0x0;
                c.r[4] = 0x64;
                c.lr = 0x80116d00; f_8018c8f4(c);
                pc = 0x80116d00;
            }
            0x80116d00 => {
                c.r[5] = c.r[29] | c.r[29];
                c.r[3] = 0x0;
                c.r[4] = 0x65;
                c.lr = 0x80116d10; f_8018c8f4(c);
                pc = 0x80116d10;
            }
            0x80116d10 => {
                c.r[3] = 0x0;
                pc = 0x80116d14;
            }
            0x80116d14 => {
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x30); c.f[31] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_80116958: bad pc {:#x}", pc),
        }
    }
}

/// heroDispatchEvent (0x80116d30, 316 bytes)
#[inline(never)]
pub fn f_80116d30(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116d30) { return; }
        if c.prof { crate::prof::enter(c, 0x80116d30); f_80116d30_b(c, 0x80116d30); crate::prof::exit(c, 0x80116d30); return; }
    }
    f_80116d30_b(c, 0x80116d30)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116d30_at(c: &mut Cpu, pc: u32) { f_80116d30_b(c, pc) }
#[inline(always)]
fn f_80116d30_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116d30;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116d30, pc); }
        match pc {
            0x80116d30 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[5] = 0x80270000;
                c.r[31] = c.r[5].wrapping_add(0x2708);
                c.lr = 0x80116d54; f_801141f8(c);
                pc = 0x80116d54;
            }
            0x80116d54 => {
                c.r[28] = c.r[26] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[28], 0x3);
                if c.crbit(2) { pc = 0x80116da8; continue; }
                pc = 0x80116d60;
            }
            0x80116d60 => {
                if !c.crbit(0) { pc = 0x80116d74; continue; }
                pc = 0x80116d64;
            }
            0x80116d64 => {
                c.cmps(0, c.r[28], 0x1);
                if c.crbit(2) { pc = 0x80116d80; continue; }
                pc = 0x80116d6c;
            }
            0x80116d6c => {
                if !c.crbit(0) { pc = 0x80116d94; continue; }
                pc = 0x80116d70;
            }
            0x80116d70 => {
                pc = 0x80116dcc; continue;
                pc = 0x80116d74;
            }
            0x80116d74 => {
                c.cmps(0, c.r[28], 0x5);
                if !c.crbit(0) { pc = 0x80116dcc; continue; }
                pc = 0x80116d7c;
            }
            0x80116d7c => {
                pc = 0x80116dbc; continue;
                pc = 0x80116d80;
            }
            0x80116d80 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[31].wrapping_add(0x14);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80116d90; stubs_gen::f_800dd970(c);
                pc = 0x80116d90;
            }
            0x80116d90 => {
                pc = 0x80116dcc; continue;
                pc = 0x80116d94;
            }
            0x80116d94 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[31].wrapping_add(0x28);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80116da4; stubs_gen::f_800dd970(c);
                pc = 0x80116da4;
            }
            0x80116da4 => {
                pc = 0x80116dcc; continue;
                pc = 0x80116da8;
            }
            0x80116da8 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[31].wrapping_add(0x3c);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80116db8; stubs_gen::f_800dd970(c);
                pc = 0x80116db8;
            }
            0x80116db8 => {
                pc = 0x80116dcc; continue;
                pc = 0x80116dbc;
            }
            0x80116dbc => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[31].wrapping_add(0x50);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80116dcc; stubs_gen::f_800dd970(c);
                pc = 0x80116dcc;
            }
            0x80116dcc => {
                c.lr = 0x80116dd0; f_800ff56c(c);
                pc = 0x80116dd0;
            }
            0x80116dd0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[26] = 0x0;
                c.r[30] = 0x0;
                pc = 0x80116e48; continue;
                pc = 0x80116de0;
            }
            0x80116de0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff86a4));
                c.r[27] = c.r[0].wrapping_add(c.r[30]);
                c.r[0] = c.lhz(c.r[27].wrapping_add(0x2));
                c.cmpu(0, c.r[0], c.r[31]);
                if !c.crbit(2) { pc = 0x80116e40; continue; }
                pc = 0x80116df4;
            }
            0x80116df4 => {
                c.r[0] = c.lbz(c.r[27]);
                c.cmps(0, c.r[0], c.r[28]);
                if !c.crbit(2) { pc = 0x80116e40; continue; }
                pc = 0x80116e00;
            }
            0x80116e00 => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x4));
                c.cmpu(0, c.r[0], c.r[29]);
                if !c.crbit(2) { pc = 0x80116e40; continue; }
                pc = 0x80116e0c;
            }
            0x80116e0c => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80116e40; continue; }
                pc = 0x80116e18;
            }
            0x80116e18 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80116e20; f_801ca9f8(c);
                pc = 0x80116e20;
            }
            0x80116e20 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x8));
                c.r[4] = 0x4;
                c.r[5] = c.lwz(c.r[27].wrapping_add(0xc));
                c.r[6] = c.lwz(c.r[27].wrapping_add(0x10));
                c.r[7] = c.lwz(c.r[27].wrapping_add(0x14));
                c.r[8] = c.lwz(c.r[27].wrapping_add(0x18));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80116e40; f_800f7434(c);
                pc = 0x80116e40;
            }
            0x80116e40 => {
                c.r[30] = c.r[30].wrapping_add(0x1c);
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x80116e48;
            }
            0x80116e48 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff86a0));
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[26], c.r[0]);
                if c.crbit(0) { pc = 0x80116de0; continue; }
                pc = 0x80116e58;
            }
            0x80116e58 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80116d30: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosSetVisibility (0x80116e6c, 24 bytes)
#[inline(never)]
pub fn f_80116e6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116e6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80116e6c); f_80116e6c_b(c, 0x80116e6c); crate::prof::exit(c, 0x80116e6c); return; }
    }
    f_80116e6c_b(c, 0x80116e6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116e6c_at(c: &mut Cpu, pc: u32) { f_80116e6c_b(c, pc) }
#[inline(always)]
fn f_80116e6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116e6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116e6c, pc); }
        match pc {
            0x80116e6c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80116e74;
            }
            0x80116e74 => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[0] = (c.r[4].rotate_left(7) & 0x80) | (c.r[0] & 0xffffff7f);
                c.stb(c.r[3], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80116e6c: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosSetPos (0x80116e84, 44 bytes)
#[inline(never)]
pub fn f_80116e84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116e84) { return; }
        if c.prof { crate::prof::enter(c, 0x80116e84); f_80116e84_b(c, 0x80116e84); crate::prof::exit(c, 0x80116e84); return; }
    }
    f_80116e84_b(c, 0x80116e84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116e84_at(c: &mut Cpu, pc: u32) { f_80116e84_b(c, pc) }
#[inline(always)]
fn f_80116e84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116e84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116e84, pc); }
        match pc {
            0x80116e84 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80116e8c;
            }
            0x80116e8c => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80116e94;
            }
            0x80116e94 => {
                { let ea = c.r[4]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x18); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x1c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[4].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_80116e84: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetTalkSctID (0x80116eb0, 24 bytes)
#[inline(never)]
pub fn f_80116eb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116eb0) { return; }
        if c.prof { crate::prof::enter(c, 0x80116eb0); f_80116eb0_b(c, 0x80116eb0); crate::prof::exit(c, 0x80116eb0); return; }
    }
    f_80116eb0_b(c, 0x80116eb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116eb0_at(c: &mut Cpu, pc: u32) { f_80116eb0_b(c, pc) }
#[inline(always)]
fn f_80116eb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116eb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116eb0, pc); }
        match pc {
            0x80116eb0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116ec0; continue; }
                pc = 0x80116eb8;
            }
            0x80116eb8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                if true { return; }
                pc = 0x80116ec0;
            }
            0x80116ec0 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80116eb0: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetMoveSctID (0x80116ec8, 24 bytes)
#[inline(never)]
pub fn f_80116ec8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116ec8) { return; }
        if c.prof { crate::prof::enter(c, 0x80116ec8); f_80116ec8_b(c, 0x80116ec8); crate::prof::exit(c, 0x80116ec8); return; }
    }
    f_80116ec8_b(c, 0x80116ec8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116ec8_at(c: &mut Cpu, pc: u32) { f_80116ec8_b(c, pc) }
#[inline(always)]
fn f_80116ec8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116ec8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116ec8, pc); }
        match pc {
            0x80116ec8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116ed8; continue; }
                pc = 0x80116ed0;
            }
            0x80116ed0 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                if true { return; }
                pc = 0x80116ed8;
            }
            0x80116ed8 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80116ec8: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetNameID (0x80116ee0, 24 bytes)
#[inline(never)]
pub fn f_80116ee0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116ee0) { return; }
        if c.prof { crate::prof::enter(c, 0x80116ee0); f_80116ee0_b(c, 0x80116ee0); crate::prof::exit(c, 0x80116ee0); return; }
    }
    f_80116ee0_b(c, 0x80116ee0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116ee0_at(c: &mut Cpu, pc: u32) { f_80116ee0_b(c, pc) }
#[inline(always)]
fn f_80116ee0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116ee0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116ee0, pc); }
        match pc {
            0x80116ee0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116ef0; continue; }
                pc = 0x80116ee8;
            }
            0x80116ee8 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x8));
                if true { return; }
                pc = 0x80116ef0;
            }
            0x80116ef0 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80116ee0: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetTalkWallThrough (0x80116ef8, 28 bytes)
#[inline(never)]
pub fn f_80116ef8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116ef8) { return; }
        if c.prof { crate::prof::enter(c, 0x80116ef8); f_80116ef8_b(c, 0x80116ef8); crate::prof::exit(c, 0x80116ef8); return; }
    }
    f_80116ef8_b(c, 0x80116ef8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116ef8_at(c: &mut Cpu, pc: u32) { f_80116ef8_b(c, pc) }
#[inline(always)]
fn f_80116ef8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116ef8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116ef8, pc); }
        match pc {
            0x80116ef8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116f0c; continue; }
                pc = 0x80116f00;
            }
            0x80116f00 => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[3] = c.r[0].rotate_left(28) & 0x1;
                if true { return; }
                pc = 0x80116f0c;
            }
            0x80116f0c => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80116ef8: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetTalkEndType (0x80116f14, 28 bytes)
#[inline(never)]
pub fn f_80116f14(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116f14) { return; }
        if c.prof { crate::prof::enter(c, 0x80116f14); f_80116f14_b(c, 0x80116f14); crate::prof::exit(c, 0x80116f14); return; }
    }
    f_80116f14_b(c, 0x80116f14)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116f14_at(c: &mut Cpu, pc: u32) { f_80116f14_b(c, pc) }
#[inline(always)]
fn f_80116f14_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116f14;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116f14, pc); }
        match pc {
            0x80116f14 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116f28; continue; }
                pc = 0x80116f1c;
            }
            0x80116f1c => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[0].rotate_left(28) & 0x3;
                if true { return; }
                pc = 0x80116f28;
            }
            0x80116f28 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80116f14: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetTalkStartType (0x80116f30, 28 bytes)
#[inline(never)]
pub fn f_80116f30(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116f30) { return; }
        if c.prof { crate::prof::enter(c, 0x80116f30); f_80116f30_b(c, 0x80116f30); crate::prof::exit(c, 0x80116f30); return; }
    }
    f_80116f30_b(c, 0x80116f30)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116f30_at(c: &mut Cpu, pc: u32) { f_80116f30_b(c, pc) }
#[inline(always)]
fn f_80116f30_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116f30;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116f30, pc); }
        match pc {
            0x80116f30 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116f44; continue; }
                pc = 0x80116f38;
            }
            0x80116f38 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[0].rotate_left(26) & 0x3;
                if true { return; }
                pc = 0x80116f44;
            }
            0x80116f44 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80116f30: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetMoveType (0x80116f4c, 28 bytes)
#[inline(never)]
pub fn f_80116f4c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116f4c) { return; }
        if c.prof { crate::prof::enter(c, 0x80116f4c); f_80116f4c_b(c, 0x80116f4c); crate::prof::exit(c, 0x80116f4c); return; }
    }
    f_80116f4c_b(c, 0x80116f4c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116f4c_at(c: &mut Cpu, pc: u32) { f_80116f4c_b(c, pc) }
#[inline(always)]
fn f_80116f4c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116f4c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116f4c, pc); }
        match pc {
            0x80116f4c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80116f60; continue; }
                pc = 0x80116f54;
            }
            0x80116f54 => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[3] = c.r[0].rotate_left(0) & 0x7;
                if true { return; }
                pc = 0x80116f60;
            }
            0x80116f60 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80116f4c: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetRot (0x80116f68, 120 bytes)
#[inline(never)]
pub fn f_80116f68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116f68) { return; }
        if c.prof { crate::prof::enter(c, 0x80116f68); f_80116f68_b(c, 0x80116f68); crate::prof::exit(c, 0x80116f68); return; }
    }
    f_80116f68_b(c, 0x80116f68)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116f68_at(c: &mut Cpu, pc: u32) { f_80116f68_b(c, pc) }
#[inline(always)]
fn f_80116f68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116f68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116f68, pc); }
        match pc {
            0x80116f68 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80116f84; continue; }
                pc = 0x80116f7c;
            }
            0x80116f7c => {
                c.r[3] = 0x0;
                pc = 0x80116fd0; continue;
                pc = 0x80116f84;
            }
            0x80116f84 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x80116f94; continue; }
                pc = 0x80116f8c;
            }
            0x80116f8c => {
                c.r[3] = 0x0;
                pc = 0x80116fd0; continue;
                pc = 0x80116f94;
            }
            0x80116f94 => {
                c.r[5] = c.lha(c.r[3].wrapping_add(0x4));
                c.r[0] = 0x43300000;
                { let ea = c.r[2].wrapping_add(0xffff9920); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[4] | c.r[4];
                c.r[4] = c.r[5] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9928); c.f[2] = c.lfd(ea); }
                c.f[3] = c.f[1];
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                { let ea = c.r[2].wrapping_add(0xffff9924); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[2]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[4] * c.f[0]); c.f[2] = v; c.ps1[2] = v; }
                c.lr = 0x80116fcc; f_800e01f4(c);
                pc = 0x80116fcc;
            }
            0x80116fcc => {
                c.r[3] = 0x1;
                pc = 0x80116fd0;
            }
            0x80116fd0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80116f68: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetPos (0x80116fe0, 88 bytes)
#[inline(never)]
pub fn f_80116fe0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80116fe0) { return; }
        if c.prof { crate::prof::enter(c, 0x80116fe0); f_80116fe0_b(c, 0x80116fe0); crate::prof::exit(c, 0x80116fe0); return; }
    }
    f_80116fe0_b(c, 0x80116fe0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80116fe0_at(c: &mut Cpu, pc: u32) { f_80116fe0_b(c, pc) }
#[inline(always)]
fn f_80116fe0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80116fe0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80116fe0, pc); }
        match pc {
            0x80116fe0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x80117000; continue; }
                pc = 0x80116ff8;
            }
            0x80116ff8 => {
                c.r[3] = 0x0;
                pc = 0x80117028; continue;
                pc = 0x80117000;
            }
            0x80117000 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x80117010; continue; }
                pc = 0x80117008;
            }
            0x80117008 => {
                c.r[3] = 0x0;
                pc = 0x80117028; continue;
                pc = 0x80117010;
            }
            0x80117010 => {
                { let ea = c.r[5].wrapping_add(0x18); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[4] | c.r[4];
                { let ea = c.r[5].wrapping_add(0x1c); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[5].wrapping_add(0x20); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x80117024; f_800e01f4(c);
                pc = 0x80117024;
            }
            0x80117024 => {
                c.r[3] = 0x1;
                pc = 0x80117028;
            }
            0x80117028 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80116fe0: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetLoadInit (0x80117038, 28 bytes)
#[inline(never)]
pub fn f_80117038(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117038) { return; }
        if c.prof { crate::prof::enter(c, 0x80117038); f_80117038_b(c, 0x80117038); crate::prof::exit(c, 0x80117038); return; }
    }
    f_80117038_b(c, 0x80117038)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117038_at(c: &mut Cpu, pc: u32) { f_80117038_b(c, pc) }
#[inline(always)]
fn f_80117038_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117038;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117038, pc); }
        match pc {
            0x80117038 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8011704c; continue; }
                pc = 0x80117040;
            }
            0x80117040 => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[3] = c.r[0].rotate_left(26) & 0x1;
                if true { return; }
                pc = 0x8011704c;
            }
            0x8011704c => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80117038: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetVisibility (0x80117054, 28 bytes)
#[inline(never)]
pub fn f_80117054(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117054) { return; }
        if c.prof { crate::prof::enter(c, 0x80117054); f_80117054_b(c, 0x80117054); crate::prof::exit(c, 0x80117054); return; }
    }
    f_80117054_b(c, 0x80117054)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117054_at(c: &mut Cpu, pc: u32) { f_80117054_b(c, pc) }
#[inline(always)]
fn f_80117054_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117054;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117054, pc); }
        match pc {
            0x80117054 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117068; continue; }
                pc = 0x8011705c;
            }
            0x8011705c => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[3] = c.r[0].rotate_left(25) & 0x1;
                if true { return; }
                pc = 0x80117068;
            }
            0x80117068 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80117054: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetPeopleInfoPtr (0x80117070, 52 bytes)
#[inline(never)]
pub fn f_80117070(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117070) { return; }
        if c.prof { crate::prof::enter(c, 0x80117070); f_80117070_b(c, 0x80117070); crate::prof::exit(c, 0x80117070); return; }
    }
    f_80117070_b(c, 0x80117070)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117070_at(c: &mut Cpu, pc: u32) { f_80117070_b(c, pc) }
#[inline(always)]
fn f_80117070_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117070;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117070, pc); }
        match pc {
            0x80117070 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117090; continue; }
                pc = 0x80117084;
            }
            0x80117084 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x6));
                c.lr = 0x8011708c; f_8018f6cc(c);
                pc = 0x8011708c;
            }
            0x8011708c => {
                pc = 0x80117094; continue;
                pc = 0x80117090;
            }
            0x80117090 => {
                c.r[3] = 0x0;
                pc = 0x80117094;
            }
            0x80117094 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80117070: bad pc {:#x}", pc),
        }
    }
}

/// fn_801170A4 (0x801170a4, 120 bytes)
#[inline(never)]
pub fn f_801170a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801170a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801170a4); f_801170a4_b(c, 0x801170a4); crate::prof::exit(c, 0x801170a4); return; }
    }
    f_801170a4_b(c, 0x801170a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801170a4_at(c: &mut Cpu, pc: u32) { f_801170a4_b(c, pc) }
#[inline(always)]
fn f_801170a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801170a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801170a4, pc); }
        match pc {
            0x801170a4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[30] = c.r[4] | c.r[4];
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x801170d4; continue; }
                pc = 0x801170cc;
            }
            0x801170cc => {
                c.r[3] = 0x0;
                pc = 0x80117100; continue;
                pc = 0x801170d4;
            }
            0x801170d4 => {
                c.lr = 0x801170d8; f_800ff56c(c);
                pc = 0x801170d8;
            }
            0x801170d8 => {
                c.lr = 0x801170dc; f_80115c48(c);
                pc = 0x801170dc;
            }
            0x801170dc => {
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x801170e4; f_80115a80(c);
                pc = 0x801170e4;
            }
            0x801170e4 => {
                c.cmpu(0, c.r[29], c.r[3]);
                if c.crbit(2) { pc = 0x801170f4; continue; }
                pc = 0x801170ec;
            }
            0x801170ec => {
                c.r[3] = 0x0;
                pc = 0x80117100; continue;
                pc = 0x801170f4;
            }
            0x801170f4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80117100; f_80115684(c);
                pc = 0x80117100;
            }
            0x80117100 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801170a4: bad pc {:#x}", pc),
        }
    }
}

/// floorCharacterBiosGetByIndex (0x8011711c, 56 bytes)
#[inline(never)]
pub fn f_8011711c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011711c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011711c); f_8011711c_b(c, 0x8011711c); crate::prof::exit(c, 0x8011711c); return; }
    }
    f_8011711c_b(c, 0x8011711c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011711c_at(c: &mut Cpu, pc: u32) { f_8011711c_b(c, pc) }
#[inline(always)]
fn f_8011711c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011711c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011711c, pc); }
        match pc {
            0x8011711c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.lr = 0x80117134; f_800ff56c(c);
                pc = 0x80117134;
            }
            0x80117134 => {
                c.lr = 0x80117138; f_80115c48(c);
                pc = 0x80117138;
            }
            0x80117138 => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80117140; f_80115684(c);
                pc = 0x80117140;
            }
            0x80117140 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011711c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80117154 (0x80117154, 16 bytes)
#[inline(never)]
pub fn f_80117154(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117154) { return; }
        if c.prof { crate::prof::enter(c, 0x80117154); f_80117154_b(c, 0x80117154); crate::prof::exit(c, 0x80117154); return; }
    }
    f_80117154_b(c, 0x80117154)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117154_at(c: &mut Cpu, pc: u32) { f_80117154_b(c, pc) }
#[inline(always)]
fn f_80117154_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117154;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117154, pc); }
        match pc {
            0x80117154 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa548), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa54c), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80117154: bad pc {:#x}", pc),
        }
    }
}

/// fn_80117164 (0x80117164, 100 bytes)
#[inline(never)]
pub fn f_80117164(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117164) { return; }
        if c.prof { crate::prof::enter(c, 0x80117164); f_80117164_b(c, 0x80117164); crate::prof::exit(c, 0x80117164); return; }
    }
    f_80117164_b(c, 0x80117164)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117164_at(c: &mut Cpu, pc: u32) { f_80117164_b(c, pc) }
#[inline(always)]
fn f_80117164_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117164;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117164, pc); }
        match pc {
            0x80117164 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80117174; f_801155cc(c);
                pc = 0x80117174;
            }
            0x80117174 => {
                { let ea = c.r[2].wrapping_add(0xffff9930); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = 0x0;
                c.r[0] = 0x1;
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa548), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa54c), c.r[4]);
                c.stb(c.r[13].wrapping_add(0xffffa550), c.r[4]);
                c.stb(c.r[13].wrapping_add(0xffffa551), c.r[0]);
                { let ea = c.r[13].wrapping_add(0xffffa554); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[13].wrapping_add(0xffffa558); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[13].wrapping_add(0xffffa55c); let v = c.f[0]; c.stfs(ea, v); }
                if c.crbit(2) { pc = 0x801171b8; continue; }
                pc = 0x801171a4;
            }
            0x801171a4 => {
                c.r[4] = c.lwz(c.r[3]);
                c.r[0] = c.lwz(c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa548), c.r[0]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.stw(c.r[13].wrapping_add(0xffffa54c), c.r[0]);
                pc = 0x801171b8;
            }
            0x801171b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80117164: bad pc {:#x}", pc),
        }
    }
}

/// fn_801171C8 (0x801171c8, 360 bytes)
#[inline(never)]
pub fn f_801171c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801171c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801171c8); f_801171c8_b(c, 0x801171c8); crate::prof::exit(c, 0x801171c8); return; }
    }
    f_801171c8_b(c, 0x801171c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801171c8_at(c: &mut Cpu, pc: u32) { f_801171c8_b(c, pc) }
#[inline(always)]
fn f_801171c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801171c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801171c8, pc); }
        match pc {
            0x801171c8 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa551));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80117310; continue; }
                pc = 0x801171f0;
            }
            0x801171f0 => {
                c.lr = 0x801171f4; f_80177a38(c);
                pc = 0x801171f4;
            }
            0x801171f4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80117310; continue; }
                pc = 0x80117200;
            }
            0x80117200 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa548));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80117310; continue; }
                pc = 0x8011720c;
            }
            0x8011720c => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011723c; continue; }
                pc = 0x80117214;
            }
            0x80117214 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa54c));
                { let ea = c.r[3].wrapping_add(0x14); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                { let ea = c.r[3].wrapping_add(0xc); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                { let ea = c.r[3].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x80117228; f_80176868(c);
                pc = 0x80117228;
            }
            0x80117228 => {
                c.f[1] = c.f[31];
                c.lr = 0x80117230; f_801767e0(c);
                pc = 0x80117230;
            }
            0x80117230 => {
                c.f[1] = c.f[30];
                c.lr = 0x80117238; f_80176758(c);
                pc = 0x80117238;
            }
            0x80117238 => {
                pc = 0x80117310; continue;
                pc = 0x8011723c;
            }
            0x8011723c => {
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.lr = 0x80117244; f_80177908(c);
                pc = 0x80117244;
            }
            0x80117244 => {
                { let ea = c.r[13].wrapping_add(0xffffa554); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[3] = c.r[1].wrapping_add(0x14);
                { let ea = c.r[13].wrapping_add(0xffffa558); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = c.r[1].wrapping_add(0xc);
                { let ea = c.r[13].wrapping_add(0xffffa55c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[5] = c.r[1].wrapping_add(0x10);
                { let ea = c.r[1].wrapping_add(0xc); let v = c.f[2]; c.stfs(ea, v); }
                c.r[6] = c.r[1].wrapping_add(0x8);
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x80117270; f_80117514(c);
                pc = 0x80117270;
            }
            0x80117270 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80117310; continue; }
                pc = 0x8011727c;
            }
            0x8011727c => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa550));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801172d0; continue; }
                pc = 0x80117288;
            }
            0x80117288 => {
                { let ea = c.r[2].wrapping_add(0xffff9938); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[3] * c.f[2]); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[3] * c.f[1]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9934); c.f[6] = c.lfs(ea); c.ps1[6] = c.f[6]; }
                { let ea = c.r[13].wrapping_add(0xffffa554); c.f[5] = c.lfs(ea); c.ps1[5] = c.f[5]; }
                { let v = s(c.f[3] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[13].wrapping_add(0xffffa558); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[13].wrapping_add(0xffffa55c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let v = s(c.f[6].mul_add(c.f[5], c.f[4])); c.f[4] = v; c.ps1[4] = v; }
                { let v = s(c.f[6].mul_add(c.f[3], c.f[2])); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[6].mul_add(c.f[1], c.f[0])); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[13].wrapping_add(0xffffa554); let v = c.f[4]; c.stfs(ea, v); }
                { let ea = c.r[13].wrapping_add(0xffffa558); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[13].wrapping_add(0xffffa55c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801172f0; continue;
                pc = 0x801172d0;
            }
            0x801172d0 => {
                { let ea = c.r[1].wrapping_add(0xc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[0] = 0x1;
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[13].wrapping_add(0xffffa554); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[13].wrapping_add(0xffffa558); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[13].wrapping_add(0xffffa55c); let v = c.f[0]; c.stfs(ea, v); }
                c.stb(c.r[13].wrapping_add(0xffffa550), c.r[0]);
                pc = 0x801172f0;
            }
            0x801172f0 => {
                { let ea = c.r[13].wrapping_add(0xffffa55c); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                { let ea = c.r[13].wrapping_add(0xffffa558); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                { let ea = c.r[13].wrapping_add(0xffffa554); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x80117300; f_80176868(c);
                pc = 0x80117300;
            }
            0x80117300 => {
                c.f[1] = c.f[30];
                c.lr = 0x80117308; f_801767e0(c);
                pc = 0x80117308;
            }
            0x80117308 => {
                c.f[1] = c.f[31];
                c.lr = 0x80117310; f_80176758(c);
                pc = 0x80117310;
            }
            0x80117310 => {
                { let ea = c.r[1].wrapping_add(0x38); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x30); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x28); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                { let ea = c.r[1].wrapping_add(0x20); c.f[30] = c.lfd(ea); }
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_801171c8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80117330 (0x80117330, 404 bytes)
#[inline(never)]
pub fn f_80117330(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117330) { return; }
        if c.prof { crate::prof::enter(c, 0x80117330); f_80117330_b(c, 0x80117330); crate::prof::exit(c, 0x80117330); return; }
    }
    f_80117330_b(c, 0x80117330)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117330_at(c: &mut Cpu, pc: u32) { f_80117330_b(c, pc) }
#[inline(always)]
fn f_80117330_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117330;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117330, pc); }
        match pc {
            0x80117330 => {
                { let ea = c.r[1].wrapping_add(0xffffff70); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x94), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x80); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                c.f[31] = c.f[1];
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa551));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801174ac; continue; }
                pc = 0x80117354;
            }
            0x80117354 => {
                c.lr = 0x80117358; f_80177a38(c);
                pc = 0x80117358;
            }
            0x80117358 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801174ac; continue; }
                pc = 0x80117364;
            }
            0x80117364 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa548));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801174ac; continue; }
                pc = 0x80117370;
            }
            0x80117370 => {
                c.r[3] = 0x0;
                c.r[4] = 0x64;
                c.lr = 0x8011737c; f_800f9318(c);
                pc = 0x8011737c;
            }
            0x8011737c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117390; continue; }
                pc = 0x80117384;
            }
            0x80117384 => {
                c.r[4] = c.r[1].wrapping_add(0x44);
                c.lr = 0x8011738c; f_800e3d98(c);
                pc = 0x8011738c;
            }
            0x8011738c => {
                pc = 0x80117398; continue;
                pc = 0x80117390;
            }
            0x80117390 => {
                c.r[3] = c.r[1].wrapping_add(0x44);
                c.lr = 0x80117398; f_80177908(c);
                pc = 0x80117398;
            }
            0x80117398 => {
                c.r[3] = c.r[1].wrapping_add(0x38);
                c.lr = 0x801173a0; f_8017795c(c);
                pc = 0x801173a0;
            }
            0x801173a0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa548));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x801173cc; continue; }
                pc = 0x801173ac;
            }
            0x801173ac => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa54c));
                { let ea = c.r[3].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0xc); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[3].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80117404; continue;
                pc = 0x801173cc;
            }
            0x801173cc => {
                c.lr = 0x801173d0; f_8017669c(c);
                pc = 0x801173d0;
            }
            0x801173d0 => {
                { let ea = c.r[1].wrapping_add(0xc); let v = c.f[1]; c.stfs(ea, v); }
                c.lr = 0x801173d8; f_80176690(c);
                pc = 0x801173d8;
            }
            0x801173d8 => {
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[1]; c.stfs(ea, v); }
                c.lr = 0x801173e0; f_80176684(c);
                pc = 0x801173e0;
            }
            0x801173e0 => {
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[1]; c.stfs(ea, v); }
                c.r[3] = c.r[1].wrapping_add(0x44);
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.r[5] = c.r[1].wrapping_add(0x10);
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.lr = 0x801173f8; f_80117514(c);
                pc = 0x801173f8;
            }
            0x801173f8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801174ac; continue; }
                pc = 0x80117404;
            }
            0x80117404 => {
                { let ea = c.r[2].wrapping_add(0xffff9930); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x14);
                { let ea = c.r[1].wrapping_add(0xc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x80117418; f_800e01f4(c);
                pc = 0x80117418;
            }
            0x80117418 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x50);
                c.lr = 0x80117424; f_800e0518(c);
                pc = 0x80117424;
            }
            0x80117424 => {
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.r[4] = c.r[1].wrapping_add(0x50);
                c.r[5] = c.r[3] | c.r[3];
                c.lr = 0x80117434; f_800dff98(c);
                pc = 0x80117434;
            }
            0x80117434 => {
                c.r[3] = c.r[1].wrapping_add(0x2c);
                c.r[4] = c.r[1].wrapping_add(0x44);
                c.r[5] = c.r[1].wrapping_add(0x38);
                c.lr = 0x80117444; f_800e019c(c);
                pc = 0x80117444;
            }
            0x80117444 => {
                c.r[3] = c.r[1].wrapping_add(0x2c);
                c.r[5] = c.r[1].wrapping_add(0x14);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80117454; f_800e019c(c);
                pc = 0x80117454;
            }
            0x80117454 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[1].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x10); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.lr = 0x80117468; f_800ce2d8(c);
                pc = 0x80117468;
            }
            0x80117468 => {
                { let v = s(c.f[1]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9930); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[1] = c.f[31];
                c.r[4] = c.r[1].wrapping_add(0x44);
                { let ea = c.r[1].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = 0x0;
                c.f[0] = -c.f[2];
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x8011748c; f_801776e8(c);
                pc = 0x8011748c;
            }
            0x8011748c => {
                c.f[1] = c.f[31];
                c.r[4] = c.r[1].wrapping_add(0x2c);
                c.r[3] = 0x0;
                c.lr = 0x8011749c; f_80177574(c);
                pc = 0x8011749c;
            }
            0x8011749c => {
                c.f[1] = c.f[31];
                c.r[4] = c.r[1].wrapping_add(0x20);
                c.r[3] = 0x0;
                c.lr = 0x801174ac; f_80177478(c);
                pc = 0x801174ac;
            }
            0x801174ac => {
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x94));
                { let ea = c.r[1].wrapping_add(0x80); c.f[31] = c.lfd(ea); }
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x90);
                if true { return; }
                return;
            }
            _ => panic!("f_80117330: bad pc {:#x}", pc),
        }
    }
}

/// fn_801174C4 (0x801174c4, 40 bytes)
#[inline(never)]
pub fn f_801174c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801174c4) { return; }
        if c.prof { crate::prof::enter(c, 0x801174c4); f_801174c4_b(c, 0x801174c4); crate::prof::exit(c, 0x801174c4); return; }
    }
    f_801174c4_b(c, 0x801174c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801174c4_at(c: &mut Cpu, pc: u32) { f_801174c4_b(c, pc) }
#[inline(always)]
fn f_801174c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801174c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801174c4, pc); }
        match pc {
            0x801174c4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa548));
                c.r[3] = 0x0;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801174e4; continue; }
                pc = 0x801174d4;
            }
            0x801174d4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa54c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801174e4; continue; }
                pc = 0x801174e0;
            }
            0x801174e0 => {
                c.r[3] = 0x1;
                pc = 0x801174e4;
            }
            0x801174e4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                if true { return; }
                return;
            }
            _ => panic!("f_801174c4: bad pc {:#x}", pc),
        }
    }
}

/// fn_801174EC (0x801174ec, 8 bytes)
#[inline(never)]
pub fn f_801174ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801174ec) { return; }
        if c.prof { crate::prof::enter(c, 0x801174ec); f_801174ec_b(c, 0x801174ec); crate::prof::exit(c, 0x801174ec); return; }
    }
    f_801174ec_b(c, 0x801174ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801174ec_at(c: &mut Cpu, pc: u32) { f_801174ec_b(c, pc) }
#[inline(always)]
fn f_801174ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801174ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801174ec, pc); }
        match pc {
            0x801174ec => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffffa551));
                if true { return; }
                return;
            }
            _ => panic!("f_801174ec: bad pc {:#x}", pc),
        }
    }
}

/// std136_fieldClearFlag_8047ad71 (0x801174f4, 12 bytes)
#[inline(never)]
pub fn f_801174f4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801174f4) { return; }
        if c.prof { crate::prof::enter(c, 0x801174f4); f_801174f4_b(c, 0x801174f4); crate::prof::exit(c, 0x801174f4); return; }
    }
    f_801174f4_b(c, 0x801174f4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801174f4_at(c: &mut Cpu, pc: u32) { f_801174f4_b(c, pc) }
#[inline(always)]
fn f_801174f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801174f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801174f4, pc); }
        match pc {
            0x801174f4 => {
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa551), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_801174f4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80117500 (0x80117500, 20 bytes)
#[inline(never)]
pub fn f_80117500(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117500) { return; }
        if c.prof { crate::prof::enter(c, 0x80117500); f_80117500_b(c, 0x80117500); crate::prof::exit(c, 0x80117500); return; }
    }
    f_80117500_b(c, 0x80117500)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117500_at(c: &mut Cpu, pc: u32) { f_80117500_b(c, pc) }
#[inline(always)]
fn f_80117500_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117500;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117500, pc); }
        match pc {
            0x80117500 => {
                c.r[3] = 0x1;
                c.r[0] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffffa551), c.r[3]);
                c.stb(c.r[13].wrapping_add(0xffffa550), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80117500: bad pc {:#x}", pc),
        }
    }
}

/// floorUpdateFieldCamera (0x80117514, 436 bytes)
#[inline(never)]
pub fn f_80117514(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117514) { return; }
        if c.prof { crate::prof::enter(c, 0x80117514); f_80117514_b(c, 0x80117514); crate::prof::exit(c, 0x80117514); return; }
    }
    f_80117514_b(c, 0x80117514)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117514_at(c: &mut Cpu, pc: u32) { f_80117514_b(c, pc) }
#[inline(always)]
fn f_80117514_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117514;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117514, pc); }
        match pc {
            0x80117514 => {
                { let ea = c.r[1].wrapping_add(0xffffff60); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xa4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x90); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x98); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x80); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x70); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[28]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = (c.f[28], c.ps1[28]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[27]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[27], c.ps1[27]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[26]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = (c.f[26], c.ps1[26]); c.psq_store(ea, x, y, false, 0); }
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[28] = c.r[6] | c.r[6];
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa548));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011759c; continue; }
                pc = 0x80117570;
            }
            0x80117570 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa54c));
                c.r[3] = 0x1;
                { let ea = c.r[4].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[26]; let v = c.f[0]; c.stfs(ea, v); }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa54c));
                { let ea = c.r[4].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[27]; let v = c.f[0]; c.stfs(ea, v); }
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa54c));
                { let ea = c.r[4].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28]; let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80117684; continue;
                pc = 0x8011759c;
            }
            0x8011759c => {
                { let ea = c.r[2].wrapping_add(0xffff9930); c.f[29] = c.lfs(ea); c.ps1[29] = c.f[29]; }
                c.r[30] = 0x0;
                c.r[31] = 0x0;
                c.f[28] = c.f[29];
                c.f[27] = c.f[29];
                c.f[26] = c.f[29];
                { let ea = c.r[2].wrapping_add(0xffff993c); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                { let ea = c.r[2].wrapping_add(0xffff9940); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                pc = 0x80117630; continue;
                pc = 0x801175c0;
            }
            0x801175c0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa54c));
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[29] = c.r[0].wrapping_add(c.r[31]);
                { let ea = c.r[29]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[29].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[29].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x801175dc; f_800e01f4(c);
                pc = 0x801175dc;
            }
            0x801175dc => {
                c.r[4] = c.r[25] | c.r[25];
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.lr = 0x801175e8; f_800e0020(c);
                pc = 0x801175e8;
            }
            0x801175e8 => {
                c.fcmp(0, c.f[1], c.f[30]);
                if !c.crbit(1) { pc = 0x80117614; continue; }
                pc = 0x801175f0;
            }
            0x801175f0 => {
                { let v = s(c.f[31] / c.f[1]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[29].wrapping_add(0xc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[29].wrapping_add(0x10); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[29].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[29] + c.f[3]); c.f[29] = v; c.ps1[29] = v; }
                { let v = s(c.f[2].mul_add(c.f[3], c.f[28])); c.f[28] = v; c.ps1[28] = v; }
                { let v = s(c.f[1].mul_add(c.f[3], c.f[27])); c.f[27] = v; c.ps1[27] = v; }
                { let v = s(c.f[0].mul_add(c.f[3], c.f[26])); c.f[26] = v; c.ps1[26] = v; }
                pc = 0x80117628; continue;
                pc = 0x80117614;
            }
            0x80117614 => {
                c.f[29] = c.f[30];
                { let ea = c.r[29].wrapping_add(0x10); c.f[27] = c.lfs(ea); c.ps1[27] = c.f[27]; }
                { let ea = c.r[29].wrapping_add(0xc); c.f[28] = c.lfs(ea); c.ps1[28] = c.f[28]; }
                { let ea = c.r[29].wrapping_add(0x14); c.f[26] = c.lfs(ea); c.ps1[26] = c.f[26]; }
                pc = 0x8011763c; continue;
                pc = 0x80117628;
            }
            0x80117628 => {
                c.r[31] = c.r[31].wrapping_add(0x18);
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x80117630;
            }
            0x80117630 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa548));
                c.cmpu(0, c.r[30], c.r[0]);
                if c.crbit(0) { pc = 0x801175c0; continue; }
                pc = 0x8011763c;
            }
            0x8011763c => {
                { let ea = c.r[2].wrapping_add(0xffff9930); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[0], c.f[29]);
                if !c.crbit(2) { pc = 0x80117660; continue; }
                pc = 0x80117648;
            }
            0x80117648 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x2770);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80117658; stubs_gen::f_800dd970(c);
                pc = 0x80117658;
            }
            0x80117658 => {
                c.r[3] = 0x0;
                pc = 0x80117684; continue;
                pc = 0x80117660;
            }
            0x80117660 => {
                { let ea = c.r[2].wrapping_add(0xffff993c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = 0x1;
                { let v = s(c.f[0] / c.f[29]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[27] * c.f[0]); c.f[2] = v; c.ps1[2] = v; }
                { let v = s(c.f[28] * c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[26] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[26]; let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[27]; let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[28]; let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80117684;
            }
            0x80117684 => {
                { let ea = c.r[1].wrapping_add(0x98); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x90); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x80); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x78); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                { let ea = c.r[1].wrapping_add(0x70); c.f[29] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[28] = x; c.ps1[28] = y; }
                { let ea = c.r[1].wrapping_add(0x60); c.f[28] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[27] = x; c.ps1[27] = y; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[27] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = c.psq_load(ea, false, 0); c.f[26] = x; c.ps1[26] = y; }
                { let ea = c.r[1].wrapping_add(0x40); c.f[26] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x24); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xa4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xa0);
                if true { return; }
                return;
            }
            _ => panic!("f_80117514: bad pc {:#x}", pc),
        }
    }
}

/// fn_801176C8 (0x801176c8, 596 bytes)
#[inline(never)]
pub fn f_801176c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801176c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801176c8); f_801176c8_b(c, 0x801176c8); crate::prof::exit(c, 0x801176c8); return; }
    }
    f_801176c8_b(c, 0x801176c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801176c8_at(c: &mut Cpu, pc: u32) { f_801176c8_b(c, pc) }
#[inline(always)]
fn f_801176c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801176c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801176c8, pc); }
        match pc {
            0x801176c8 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x801176e0; f_80115c48(c);
                pc = 0x801176e0;
            }
            0x801176e0 => {
                c.r[4] = 0x80410000;
                c.r[0] = 0x0;
                c.r[28] = c.r[4].wrapping_add(0xffff83d0);
                c.r[27] = c.r[3] | c.r[3];
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x10));
                c.stb(c.r[28].wrapping_add(0x1c), c.r[0]);
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x80117724; continue; }
                pc = 0x80117700;
            }
            0x80117700 => {
                c.r[3] = c.r[4] | c.r[4];
                c.lr = 0x80117708; f_80115c48(c);
                pc = 0x80117708;
            }
            0x80117708 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117724; continue; }
                pc = 0x80117710;
            }
            0x80117710 => {
                c.r[4] = 0x80410000;
                c.r[4] = c.r[4].wrapping_add(0xffff83d0);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x14));
                c.stw(c.r[3].wrapping_add(0x1c), c.r[0]);
                c.lr = 0x80117724; f_80117164(c);
                pc = 0x80117724;
            }
            0x80117724 => {
                c.r[3] = 0x80410000;
                c.r[29] = c.r[3].wrapping_add(0xffff83d0);
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x18));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8011773c; continue; }
                pc = 0x80117738;
            }
            0x80117738 => {
                c.lr = 0x8011773c; f_800fe714(c);
                pc = 0x8011773c;
            }
            0x8011773c => {
                c.r[3] = 0x80410000;
                c.r[30] = c.r[3].wrapping_add(0xffff83d0);
                c.r[25] = c.lhz(c.r[30].wrapping_add(0xe));
                c.cmpu(0, c.r[25], 0x0);
                if c.crbit(2) { pc = 0x80117760; continue; }
                pc = 0x80117750;
            }
            0x80117750 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80117758; f_800e24b0(c);
                pc = 0x80117758;
            }
            0x80117758 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80117760; f_800e209c(c);
                pc = 0x80117760;
            }
            0x80117760 => {
                c.r[3] = 0x80410000;
                c.r[31] = c.r[3].wrapping_add(0xffff83d0);
                c.r[24] = c.lhz(c.r[31].wrapping_add(0xc));
                c.cmpu(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x80117784; continue; }
                pc = 0x80117774;
            }
            0x80117774 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x8011777c; f_800e24b0(c);
                pc = 0x8011777c;
            }
            0x8011777c => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x80117784; f_800e209c(c);
                pc = 0x80117784;
            }
            0x80117784 => {
                c.r[3] = 0x80410000;
                c.r[4] = 0x0;
                c.r[3] = c.r[3].wrapping_add(0xffff83d0);
                c.r[5] = 0x20;
                c.lr = 0x80117798; f_8000540c(c);
                pc = 0x80117798;
            }
            0x80117798 => {
                c.cmpu(0, c.r[27], 0x0);
                if c.crbit(2) { pc = 0x80117878; continue; }
                pc = 0x801177a0;
            }
            0x801177a0 => {
                c.r[4] = 0x80110000;
                c.r[3] = 0x1;
                c.r[6] = c.r[4].wrapping_add(0x791c);
                c.r[5] = 0x0;
                c.r[4] = 0x7f;
                c.lr = 0x801177b8; f_800fe834(c);
                pc = 0x801177b8;
            }
            0x801177b8 => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[29].wrapping_add(0x18), c.r[3]);
                if c.crbit(2) { pc = 0x80117878; continue; }
                pc = 0x801177c4;
            }
            0x801177c4 => {
                c.r[3] = 0x8;
                c.lr = 0x801177cc; f_800e3534(c);
                pc = 0x801177cc;
            }
            0x801177cc => {
                c.r[25] = c.r[3] | c.r[3];
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80117878; continue; }
                pc = 0x801177dc;
            }
            0x801177dc => {
                c.lr = 0x801177e0; f_800e27b0(c);
                pc = 0x801177e0;
            }
            0x801177e0 => {
                c.r[4] = 0x80410000;
                c.sth(c.r[31].wrapping_add(0xc), c.r[25]);
                c.r[25] = c.r[4].wrapping_add(0xffff83d0);
                c.stw(c.r[25].wrapping_add(0x4), c.r[3]);
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x801177f8; f_801155cc(c);
                pc = 0x801177f8;
            }
            0x801177f8 => {
                c.stw(c.r[28].wrapping_add(0x10), c.r[26]);
                c.r[4] = 0x80410000;
                c.r[4] = c.r[4].wrapping_add(0xffff83d0);
                c.r[26] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x1c));
                c.cmpu(0, c.r[26], 0x0);
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x4));
                c.r[27] = c.r[4].wrapping_add(0x8);
                c.stw(c.r[4].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[3], c.r[27]);
                if c.crbit(2) { pc = 0x80117908; continue; }
                pc = 0x80117824;
            }
            0x80117824 => {
                c.r[3] = c.lwz(c.r[26]);
                c.r[23] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[23], 0x0);
                if c.crbit(2) { pc = 0x80117908; continue; }
                pc = 0x80117834;
            }
            0x80117834 => {
                c.r[24] = c.r[23].wrapping_mul(0x18);
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x80117840; f_800e3534(c);
                pc = 0x80117840;
            }
            0x80117840 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80117878; continue; }
                pc = 0x8011784c;
            }
            0x8011784c => {
                c.sth(c.r[30].wrapping_add(0xe), c.r[3]);
                c.stw(c.r[27], c.r[23]);
                c.lr = 0x80117858; f_800e27b0(c);
                pc = 0x80117858;
            }
            0x80117858 => {
                c.r[4] = c.lwz(c.r[25].wrapping_add(0x4));
                c.r[5] = c.r[24] | c.r[24];
                c.stw(c.r[4].wrapping_add(0x4), c.r[3]);
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x4));
                c.r[4] = c.lwz(c.r[26].wrapping_add(0x4));
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                c.lr = 0x80117874; f_800054f4(c);
                pc = 0x80117874;
            }
            0x80117874 => {
                pc = 0x80117908; continue;
                pc = 0x80117878;
            }
            0x80117878 => {
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x10));
                c.r[0] = 0x0;
                c.stb(c.r[28].wrapping_add(0x1c), c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801178ac; continue; }
                pc = 0x8011788c;
            }
            0x8011788c => {
                c.lr = 0x80117890; f_80115c48(c);
                pc = 0x80117890;
            }
            0x80117890 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801178ac; continue; }
                pc = 0x80117898;
            }
            0x80117898 => {
                c.r[4] = 0x80410000;
                c.r[4] = c.r[4].wrapping_add(0xffff83d0);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x14));
                c.stw(c.r[3].wrapping_add(0x1c), c.r[0]);
                c.lr = 0x801178ac; f_80117164(c);
                pc = 0x801178ac;
            }
            0x801178ac => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x18));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801178bc; continue; }
                pc = 0x801178b8;
            }
            0x801178b8 => {
                c.lr = 0x801178bc; f_800fe714(c);
                pc = 0x801178bc;
            }
            0x801178bc => {
                c.r[25] = c.lhz(c.r[30].wrapping_add(0xe));
                c.cmpu(0, c.r[25], 0x0);
                if c.crbit(2) { pc = 0x801178d8; continue; }
                pc = 0x801178c8;
            }
            0x801178c8 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x801178d0; f_800e24b0(c);
                pc = 0x801178d0;
            }
            0x801178d0 => {
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x801178d8; f_800e209c(c);
                pc = 0x801178d8;
            }
            0x801178d8 => {
                c.r[24] = c.lhz(c.r[31].wrapping_add(0xc));
                c.cmpu(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x801178f4; continue; }
                pc = 0x801178e4;
            }
            0x801178e4 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x801178ec; f_800e24b0(c);
                pc = 0x801178ec;
            }
            0x801178ec => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x801178f4; f_800e209c(c);
                pc = 0x801178f4;
            }
            0x801178f4 => {
                c.r[3] = 0x80410000;
                c.r[4] = 0x0;
                c.r[3] = c.r[3].wrapping_add(0xffff83d0);
                c.r[5] = 0x20;
                c.lr = 0x80117908; f_8000540c(c);
                pc = 0x80117908;
            }
            0x80117908 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_801176c8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011791C (0x8011791c, 440 bytes)
#[inline(never)]
pub fn f_8011791c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011791c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011791c); f_8011791c_b(c, 0x8011791c); crate::prof::exit(c, 0x8011791c); return; }
    }
    f_8011791c_b(c, 0x8011791c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011791c_at(c: &mut Cpu, pc: u32) { f_8011791c_b(c, pc) }
#[inline(always)]
fn f_8011791c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011791c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011791c, pc); }
        match pc {
            0x8011791c => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x2c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x28), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x24), c.r[29]);
                c.r[3] = 0x80410000;
                c.r[3] = c.r[3].wrapping_add(0xffff83d0);
                c.r[29] = c.lwz(c.r[3]);
                c.lr = 0x80117944; f_800ff56c(c);
                pc = 0x80117944;
            }
            0x80117944 => {
                c.lr = 0x80117948; f_80115c48(c);
                pc = 0x80117948;
            }
            0x80117948 => {
                c.r[4] = 0x80410000;
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4].wrapping_add(0xffff83d0);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.lr = 0x8011795c; f_80115c48(c);
                pc = 0x8011795c;
            }
            0x8011795c => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8011796c; continue; }
                pc = 0x80117964;
            }
            0x80117964 => {
                c.cmpu(0, c.r[30], c.r[3]);
                if c.crbit(2) { pc = 0x80117a20; continue; }
                pc = 0x8011796c;
            }
            0x8011796c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[4] = 0x80410000;
                c.r[4] = c.r[4].wrapping_add(0xffff83d0);
                c.r[0] = 0x0;
                c.cmpu(0, c.r[3], 0x0);
                c.stb(c.r[4].wrapping_add(0x1c), c.r[0]);
                if c.crbit(2) { pc = 0x801179a8; continue; }
                pc = 0x80117988;
            }
            0x80117988 => {
                c.lr = 0x8011798c; f_80115c48(c);
                pc = 0x8011798c;
            }
            0x8011798c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801179a8; continue; }
                pc = 0x80117994;
            }
            0x80117994 => {
                c.r[4] = 0x80410000;
                c.r[4] = c.r[4].wrapping_add(0xffff83d0);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x14));
                c.stw(c.r[3].wrapping_add(0x1c), c.r[0]);
                c.lr = 0x801179a8; f_80117164(c);
                pc = 0x801179a8;
            }
            0x801179a8 => {
                c.r[3] = 0x80410000;
                c.r[3] = c.r[3].wrapping_add(0xffff83d0);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x18));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801179c0; continue; }
                pc = 0x801179bc;
            }
            0x801179bc => {
                c.lr = 0x801179c0; f_800fe714(c);
                pc = 0x801179c0;
            }
            0x801179c0 => {
                c.r[3] = 0x80410000;
                c.r[3] = c.r[3].wrapping_add(0xffff83d0);
                c.r[30] = c.lhz(c.r[3].wrapping_add(0xe));
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x801179e4; continue; }
                pc = 0x801179d4;
            }
            0x801179d4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801179dc; f_800e24b0(c);
                pc = 0x801179dc;
            }
            0x801179dc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801179e4; f_800e209c(c);
                pc = 0x801179e4;
            }
            0x801179e4 => {
                c.r[3] = 0x80410000;
                c.r[3] = c.r[3].wrapping_add(0xffff83d0);
                c.r[30] = c.lhz(c.r[3].wrapping_add(0xc));
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x80117a08; continue; }
                pc = 0x801179f8;
            }
            0x801179f8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80117a00; f_800e24b0(c);
                pc = 0x80117a00;
            }
            0x80117a00 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80117a08; f_800e209c(c);
                pc = 0x80117a08;
            }
            0x80117a08 => {
                c.r[3] = 0x80410000;
                c.r[4] = 0x0;
                c.r[3] = c.r[3].wrapping_add(0xffff83d0);
                c.r[5] = 0x20;
                c.lr = 0x80117a1c; f_8000540c(c);
                pc = 0x80117a1c;
            }
            0x80117a1c => {
                pc = 0x80117ab8; continue;
                pc = 0x80117a20;
            }
            0x80117a20 => {
                c.r[3] = 0x80410000;
                c.r[3] = c.r[3].wrapping_add(0xffff83d0);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80117ab8; continue; }
                pc = 0x80117a34;
            }
            0x80117a34 => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x80117ab8; continue; }
                pc = 0x80117a3c;
            }
            0x80117a3c => {
                c.r[3] = 0x80270000;
                c.r[5] = c.r[3].wrapping_add(0x27b8);
                c.r[4] = c.lwz(c.r[5]);
                c.r[3] = c.lwz(c.r[5].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[5].wrapping_add(0x8));
                c.stw(c.r[1].wrapping_add(0x8), c.r[4]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x80117a60; f_80177a38(c);
                pc = 0x80117a60;
            }
            0x80117a60 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x5);
                if !c.crbit(2) { pc = 0x80117ab8; continue; }
                pc = 0x80117a6c;
            }
            0x80117a6c => {
                c.lr = 0x80117a70; f_800d2584(c);
                pc = 0x80117a70;
            }
            0x80117a70 => {
                c.r[4] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x80117ab8; continue; }
                pc = 0x80117a7c;
            }
            0x80117a7c => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[4].wrapping_add(0x100);
                c.lr = 0x80117a88; f_800e01d0(c);
                pc = 0x80117a88;
            }
            0x80117a88 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29]; let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x8); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x80117aa4; f_80176690(c);
                pc = 0x80117aa4;
            }
            0x80117aa4 => {
                { let ea = c.r[29].wrapping_add(0xc); let v = c.f[1]; c.stfs(ea, v); }
                c.lr = 0x80117aac; f_8017669c(c);
                pc = 0x80117aac;
            }
            0x80117aac => {
                { let ea = c.r[29].wrapping_add(0x10); let v = c.f[1]; c.stfs(ea, v); }
                c.lr = 0x80117ab4; f_80176684(c);
                pc = 0x80117ab4;
            }
            0x80117ab4 => {
                { let ea = c.r[29].wrapping_add(0x14); let v = c.f[1]; c.stfs(ea, v); }
                pc = 0x80117ab8;
            }
            0x80117ab8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x2c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x28));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8011791c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80117AD4 (0x80117ad4, 16 bytes)
#[inline(never)]
pub fn f_80117ad4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117ad4) { return; }
        if c.prof { crate::prof::enter(c, 0x80117ad4); f_80117ad4_b(c, 0x80117ad4); crate::prof::exit(c, 0x80117ad4); return; }
    }
    f_80117ad4_b(c, 0x80117ad4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117ad4_at(c: &mut Cpu, pc: u32) { f_80117ad4_b(c, pc) }
#[inline(always)]
fn f_80117ad4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117ad4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117ad4, pc); }
        match pc {
            0x80117ad4 => {
                c.r[3] = 0x80410000;
                c.r[3] = c.r[3].wrapping_add(0xffff83d0);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                if true { return; }
                return;
            }
            _ => panic!("f_80117ad4: bad pc {:#x}", pc),
        }
    }
}

/// std171_mapSetTextureChange (0x80117ae4, 416 bytes)
#[inline(never)]
pub fn f_80117ae4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117ae4) { return; }
        if c.prof { crate::prof::enter(c, 0x80117ae4); f_80117ae4_b(c, 0x80117ae4); crate::prof::exit(c, 0x80117ae4); return; }
    }
    f_80117ae4_b(c, 0x80117ae4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117ae4_at(c: &mut Cpu, pc: u32) { f_80117ae4_b(c, pc) }
#[inline(always)]
fn f_80117ae4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117ae4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117ae4, pc); }
        match pc {
            0x80117ae4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8320));
                c.cmps(0, c.r[0], c.r[31]);
                if c.crbit(2) { pc = 0x80117c6c; continue; }
                pc = 0x80117b08;
            }
            0x80117b08 => {
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x80117b64; continue; }
                pc = 0x80117b14;
            }
            0x80117b14 => {
                c.lr = 0x80117b18; f_80113f48(c);
                pc = 0x80117b18;
            }
            0x80117b18 => {
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x8));
                c.lr = 0x80117b20; f_800f9318(c);
                pc = 0x80117b20;
            }
            0x80117b20 => {
                c.lr = 0x80117b24; f_800e5550(c);
                pc = 0x80117b24;
            }
            0x80117b24 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa56c));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117b3c; continue; }
                pc = 0x80117b30;
            }
            0x80117b30 => {
                c.lr = 0x80117b34; f_800ef5a4(c);
                pc = 0x80117b34;
            }
            0x80117b34 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa56c), c.r[0]);
                pc = 0x80117b3c;
            }
            0x80117b3c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117b54; continue; }
                pc = 0x80117b48;
            }
            0x80117b48 => {
                c.lr = 0x80117b4c; f_800e4bf4(c);
                pc = 0x80117b4c;
            }
            0x80117b4c => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa570), c.r[0]);
                pc = 0x80117b54;
            }
            0x80117b54 => {
                c.r[3] = 0x0;
                c.r[0] = 0xffffffff;
                c.stw(c.r[13].wrapping_add(0xffffa574), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffff8320), c.r[0]);
                pc = 0x80117b64;
            }
            0x80117b64 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa560));
                c.r[4] = 0x0;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa564));
                c.stw(c.r[13].wrapping_add(0xffffa568), c.r[0]);
                c.mtspr(9, c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117ba4; continue; }
                pc = 0x80117b80;
            }
            0x80117b80 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.cmpu(0, c.r[0], c.r[31]);
                if !c.crbit(2) { pc = 0x80117b98; continue; }
                pc = 0x80117b90;
            }
            0x80117b90 => {
                c.r[4] = 0x1;
                pc = 0x80117ba4; continue;
                pc = 0x80117b98;
            }
            0x80117b98 => {
                c.r[0] = c.r[3].wrapping_add(0x18);
                c.stw(c.r[13].wrapping_add(0xffffa568), c.r[0]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80117b80; continue; }
                pc = 0x80117ba4;
            }
            0x80117ba4 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80117bc0; continue; }
                pc = 0x80117bb0;
            }
            0x80117bb0 => {
                c.r[0] = 0x0;
                c.r[3] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa568), c.r[0]);
                pc = 0x80117c6c; continue;
                pc = 0x80117bc0;
            }
            0x80117bc0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[3] = c.lhz(c.r[4]);
                c.r[4] = c.lhz(c.r[4].wrapping_add(0x2));
                c.lr = 0x80117bdc; f_800ef5fc(c);
                pc = 0x80117bdc;
            }
            0x80117bdc => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa56c), c.r[3]);
                if !c.crbit(2) { pc = 0x80117bf8; continue; }
                pc = 0x80117be8;
            }
            0x80117be8 => {
                c.r[0] = 0x0;
                c.r[3] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa568), c.r[0]);
                pc = 0x80117c6c; continue;
                pc = 0x80117bf8;
            }
            0x80117bf8 => {
                c.lr = 0x80117bfc; f_80113f48(c);
                pc = 0x80117bfc;
            }
            0x80117bfc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0xc));
                c.lr = 0x80117c08; f_80113d34(c);
                pc = 0x80117c08;
            }
            0x80117c08 => {
                c.stw(c.r[13].wrapping_add(0xffffa570), c.r[3]);
                c.r[4] = 0x0;
                c.lr = 0x80117c14; f_800e4014(c);
                pc = 0x80117c14;
            }
            0x80117c14 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.r[4] = 0x1;
                c.lr = 0x80117c20; f_800ec188(c);
                pc = 0x80117c20;
            }
            0x80117c20 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x10));
                c.lr = 0x80117c30; f_800ecca8(c);
                pc = 0x80117c30;
            }
            0x80117c30 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.lr = 0x80117c38; f_800ec990(c);
                pc = 0x80117c38;
            }
            0x80117c38 => {
                c.lr = 0x80117c3c; f_80113f48(c);
                pc = 0x80117c3c;
            }
            0x80117c3c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x14));
                c.lr = 0x80117c48; f_800f9318(c);
                pc = 0x80117c48;
            }
            0x80117c48 => {
                c.stw(c.r[13].wrapping_add(0xffffa574), c.r[3]);
                c.lr = 0x80117c50; f_80113f48(c);
                pc = 0x80117c50;
            }
            0x80117c50 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x8));
                c.lr = 0x80117c5c; f_800f9318(c);
                pc = 0x80117c5c;
            }
            0x80117c5c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa56c));
                c.lr = 0x80117c64; f_800e563c(c);
                pc = 0x80117c64;
            }
            0x80117c64 => {
                c.stw(c.r[13].wrapping_add(0xffff8320), c.r[31]);
                c.r[3] = 0x1;
                pc = 0x80117c6c;
            }
            0x80117c6c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80117ae4: bad pc {:#x}", pc),
        }
    }
}

/// tvRelease (0x80117c84, 144 bytes)
#[inline(never)]
pub fn f_80117c84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117c84) { return; }
        if c.prof { crate::prof::enter(c, 0x80117c84); f_80117c84_b(c, 0x80117c84); crate::prof::exit(c, 0x80117c84); return; }
    }
    f_80117c84_b(c, 0x80117c84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117c84_at(c: &mut Cpu, pc: u32) { f_80117c84_b(c, pc) }
#[inline(always)]
fn f_80117c84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117c84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117c84, pc); }
        match pc {
            0x80117c84 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80117cf4; continue; }
                pc = 0x80117ca0;
            }
            0x80117ca0 => {
                c.lr = 0x80117ca4; f_80113f48(c);
                pc = 0x80117ca4;
            }
            0x80117ca4 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x8));
                c.lr = 0x80117cac; f_800f9318(c);
                pc = 0x80117cac;
            }
            0x80117cac => {
                c.lr = 0x80117cb0; f_800e5550(c);
                pc = 0x80117cb0;
            }
            0x80117cb0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa56c));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117cc8; continue; }
                pc = 0x80117cbc;
            }
            0x80117cbc => {
                c.lr = 0x80117cc0; f_800ef5a4(c);
                pc = 0x80117cc0;
            }
            0x80117cc0 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa56c), c.r[0]);
                pc = 0x80117cc8;
            }
            0x80117cc8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117ce0; continue; }
                pc = 0x80117cd4;
            }
            0x80117cd4 => {
                c.lr = 0x80117cd8; f_800e4bf4(c);
                pc = 0x80117cd8;
            }
            0x80117cd8 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa570), c.r[0]);
                pc = 0x80117ce0;
            }
            0x80117ce0 => {
                c.r[3] = 0x0;
                c.r[0] = 0xffffffff;
                c.stw(c.r[13].wrapping_add(0xffffa574), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa568), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffff8320), c.r[0]);
                pc = 0x80117cf4;
            }
            0x80117cf4 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa560), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa564), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80117c84: bad pc {:#x}", pc),
        }
    }
}

/// tvRender (0x80117d14, 324 bytes)
#[inline(never)]
pub fn f_80117d14(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117d14) { return; }
        if c.prof { crate::prof::enter(c, 0x80117d14); f_80117d14_b(c, 0x80117d14); crate::prof::exit(c, 0x80117d14); return; }
    }
    f_80117d14_b(c, 0x80117d14)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117d14_at(c: &mut Cpu, pc: u32) { f_80117d14_b(c, pc) }
#[inline(always)]
fn f_80117d14_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117d14;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117d14, pc); }
        match pc {
            0x80117d14 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80117e44; continue; }
                pc = 0x80117d30;
            }
            0x80117d30 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80117d40; continue; }
                pc = 0x80117d3c;
            }
            0x80117d3c => {
                pc = 0x80117e44; continue;
                pc = 0x80117d40;
            }
            0x80117d40 => {
                c.lr = 0x80117d44; f_800ec134(c);
                pc = 0x80117d44;
            }
            0x80117d44 => {
                c.lr = 0x80117d48; f_800d2584(c);
                pc = 0x80117d48;
            }
            0x80117d48 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = 0x2;
                c.r[31] = c.r[0] | c.r[0];
                c.lr = 0x80117d58; f_800d4604(c);
                pc = 0x80117d58;
            }
            0x80117d58 => {
                c.r[3] = 0x1;
                c.lr = 0x80117d60; f_800d377c(c);
                pc = 0x80117d60;
            }
            0x80117d60 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa56c));
                c.r[4] = 0x0;
                c.lr = 0x80117d6c; f_800d3410(c);
                pc = 0x80117d6c;
            }
            0x80117d6c => {
                c.r[3] = c.r[1].wrapping_add(0x16);
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.r[5] = c.r[1].wrapping_add(0x12);
                c.r[6] = c.r[1].wrapping_add(0x10);
                c.lr = 0x80117d80; f_800d9b24(c);
                pc = 0x80117d80;
            }
            0x80117d80 => {
                c.r[3] = c.r[1].wrapping_add(0xe);
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.r[5] = c.r[1].wrapping_add(0xa);
                c.r[6] = c.r[1].wrapping_add(0x8);
                c.lr = 0x80117d94; f_800d9af0(c);
                pc = 0x80117d94;
            }
            0x80117d94 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa574));
                c.lr = 0x80117d9c; f_800d258c(c);
                pc = 0x80117d9c;
            }
            0x80117d9c => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[6] = c.lhz(c.r[5]);
                c.r[5] = c.lhz(c.r[5].wrapping_add(0x2));
                c.r[6] = c.r[6].wrapping_add(0xffffffff);
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                c.r[5] = c.r[6].rotate_left(0) & 0xffff;
                c.r[6] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x80117dc4; f_800d9d68(c);
                pc = 0x80117dc4;
            }
            0x80117dc4 => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[6] = c.lhz(c.r[5]);
                c.r[5] = c.lhz(c.r[5].wrapping_add(0x2));
                c.r[6] = c.r[6].wrapping_add(0xffffffff);
                c.r[0] = c.r[5].wrapping_add(0xffffffff);
                c.r[5] = c.r[6].rotate_left(0) & 0xffff;
                c.r[6] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x80117dec; f_800d9c24(c);
                pc = 0x80117dec;
            }
            0x80117dec => {
                c.lr = 0x80117df0; f_800d2248(c);
                pc = 0x80117df0;
            }
            0x80117df0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.r[4] = 0x3010;
                c.lr = 0x80117dfc; f_800e3760(c);
                pc = 0x80117dfc;
            }
            0x80117dfc => {
                c.lr = 0x80117e00; f_800d3190(c);
                pc = 0x80117e00;
            }
            0x80117e00 => {
                c.r[3] = 0x1;
                c.lr = 0x80117e08; f_800d377c(c);
                pc = 0x80117e08;
            }
            0x80117e08 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80117e10; f_800d258c(c);
                pc = 0x80117e10;
            }
            0x80117e10 => {
                c.r[3] = c.lhz(c.r[1].wrapping_add(0x16));
                c.r[4] = c.lhz(c.r[1].wrapping_add(0x14));
                c.r[5] = c.lhz(c.r[1].wrapping_add(0x12));
                c.r[6] = c.lhz(c.r[1].wrapping_add(0x10));
                c.lr = 0x80117e24; f_800d9d68(c);
                pc = 0x80117e24;
            }
            0x80117e24 => {
                c.r[3] = c.lhz(c.r[1].wrapping_add(0xe));
                c.r[4] = c.lhz(c.r[1].wrapping_add(0xc));
                c.r[5] = c.lhz(c.r[1].wrapping_add(0xa));
                c.r[6] = c.lhz(c.r[1].wrapping_add(0x8));
                c.lr = 0x80117e38; f_800d9c24(c);
                pc = 0x80117e38;
            }
            0x80117e38 => {
                c.lr = 0x80117e3c; f_800d2248(c);
                pc = 0x80117e3c;
            }
            0x80117e3c => {
                c.r[3] = 0x1;
                c.lr = 0x80117e44; f_800d4604(c);
                pc = 0x80117e44;
            }
            0x80117e44 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80117d14: bad pc {:#x}", pc),
        }
    }
}

/// tvInitFloor (0x80117e58, 456 bytes)
#[inline(never)]
pub fn f_80117e58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80117e58) { return; }
        if c.prof { crate::prof::enter(c, 0x80117e58); f_80117e58_b(c, 0x80117e58); crate::prof::exit(c, 0x80117e58); return; }
    }
    f_80117e58_b(c, 0x80117e58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80117e58_at(c: &mut Cpu, pc: u32) { f_80117e58_b(c, pc) }
#[inline(always)]
fn f_80117e58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80117e58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80117e58, pc); }
        match pc {
            0x80117e58 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x48));
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x8011800c; continue; }
                pc = 0x80117e74;
            }
            0x80117e74 => {
                c.r[4] = c.lwz(c.r[4]);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.cmpu(0, c.r[0], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa560), c.r[0]);
                if c.crbit(2) { pc = 0x8011800c; continue; }
                pc = 0x80117e88;
            }
            0x80117e88 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x48));
                c.r[3] = c.lwz(c.r[3]);
                c.r[3] = c.lwz(c.r[3]);
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[0], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa564), c.r[0]);
                if c.crbit(2) { pc = 0x8011800c; continue; }
                pc = 0x80117ea4;
            }
            0x80117ea4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8320));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8011800c; continue; }
                pc = 0x80117eb0;
            }
            0x80117eb0 => {
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80117f0c; continue; }
                pc = 0x80117ebc;
            }
            0x80117ebc => {
                c.lr = 0x80117ec0; f_80113f48(c);
                pc = 0x80117ec0;
            }
            0x80117ec0 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x8));
                c.lr = 0x80117ec8; f_800f9318(c);
                pc = 0x80117ec8;
            }
            0x80117ec8 => {
                c.lr = 0x80117ecc; f_800e5550(c);
                pc = 0x80117ecc;
            }
            0x80117ecc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa56c));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117ee4; continue; }
                pc = 0x80117ed8;
            }
            0x80117ed8 => {
                c.lr = 0x80117edc; f_800ef5a4(c);
                pc = 0x80117edc;
            }
            0x80117edc => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa56c), c.r[0]);
                pc = 0x80117ee4;
            }
            0x80117ee4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117efc; continue; }
                pc = 0x80117ef0;
            }
            0x80117ef0 => {
                c.lr = 0x80117ef4; f_800e4bf4(c);
                pc = 0x80117ef4;
            }
            0x80117ef4 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa570), c.r[0]);
                pc = 0x80117efc;
            }
            0x80117efc => {
                c.r[3] = 0x0;
                c.r[0] = 0xffffffff;
                c.stw(c.r[13].wrapping_add(0xffffa574), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffff8320), c.r[0]);
                pc = 0x80117f0c;
            }
            0x80117f0c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa560));
                c.r[4] = 0x0;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa564));
                c.stw(c.r[13].wrapping_add(0xffffa568), c.r[0]);
                c.mtspr(9, c.r[3]);
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80117f4c; continue; }
                pc = 0x80117f28;
            }
            0x80117f28 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80117f40; continue; }
                pc = 0x80117f38;
            }
            0x80117f38 => {
                c.r[4] = 0x1;
                pc = 0x80117f4c; continue;
                pc = 0x80117f40;
            }
            0x80117f40 => {
                c.r[0] = c.r[3].wrapping_add(0x18);
                c.stw(c.r[13].wrapping_add(0xffffa568), c.r[0]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80117f28; continue; }
                pc = 0x80117f4c;
            }
            0x80117f4c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80117f64; continue; }
                pc = 0x80117f58;
            }
            0x80117f58 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa568), c.r[0]);
                pc = 0x8011800c; continue;
                pc = 0x80117f64;
            }
            0x80117f64 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[5] = 0x44;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[3] = c.lhz(c.r[4]);
                c.r[4] = c.lhz(c.r[4].wrapping_add(0x2));
                c.lr = 0x80117f80; f_800ef5fc(c);
                pc = 0x80117f80;
            }
            0x80117f80 => {
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa56c), c.r[3]);
                if !c.crbit(2) { pc = 0x80117f98; continue; }
                pc = 0x80117f8c;
            }
            0x80117f8c => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa568), c.r[0]);
                pc = 0x8011800c; continue;
                pc = 0x80117f98;
            }
            0x80117f98 => {
                c.lr = 0x80117f9c; f_80113f48(c);
                pc = 0x80117f9c;
            }
            0x80117f9c => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0xc));
                c.lr = 0x80117fa8; f_80113d34(c);
                pc = 0x80117fa8;
            }
            0x80117fa8 => {
                c.stw(c.r[13].wrapping_add(0xffffa570), c.r[3]);
                c.r[4] = 0x0;
                c.lr = 0x80117fb4; f_800e4014(c);
                pc = 0x80117fb4;
            }
            0x80117fb4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.r[4] = 0x1;
                c.lr = 0x80117fc0; f_800ec188(c);
                pc = 0x80117fc0;
            }
            0x80117fc0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x10));
                c.lr = 0x80117fd0; f_800ecca8(c);
                pc = 0x80117fd0;
            }
            0x80117fd0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.lr = 0x80117fd8; f_800ec990(c);
                pc = 0x80117fd8;
            }
            0x80117fd8 => {
                c.lr = 0x80117fdc; f_80113f48(c);
                pc = 0x80117fdc;
            }
            0x80117fdc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x14));
                c.lr = 0x80117fe8; f_800f9318(c);
                pc = 0x80117fe8;
            }
            0x80117fe8 => {
                c.stw(c.r[13].wrapping_add(0xffffa574), c.r[3]);
                c.lr = 0x80117ff0; f_80113f48(c);
                pc = 0x80117ff0;
            }
            0x80117ff0 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x8));
                c.lr = 0x80117ffc; f_800f9318(c);
                pc = 0x80117ffc;
            }
            0x80117ffc => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffffa56c));
                c.lr = 0x80118004; f_800e563c(c);
                pc = 0x80118004;
            }
            0x80118004 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffff8320), c.r[0]);
                pc = 0x8011800c;
            }
            0x8011800c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80117e58: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118020 (0x80118020, 72 bytes)
#[inline(never)]
pub fn f_80118020(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118020) { return; }
        if c.prof { crate::prof::enter(c, 0x80118020); f_80118020_b(c, 0x80118020); crate::prof::exit(c, 0x80118020); return; }
    }
    f_80118020_b(c, 0x80118020)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118020_at(c: &mut Cpu, pc: u32) { f_80118020_b(c, pc) }
#[inline(always)]
fn f_80118020_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118020;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118020, pc); }
        match pc {
            0x80118020 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[4] = 0x80270000;
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[7] = c.r[4].wrapping_add(0x27c8);
                c.r[4] = 0x1;
                c.r[6] = c.lwz(c.r[7]);
                c.r[5] = c.lwz(c.r[7].wrapping_add(0x4));
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x8));
                c.stw(c.r[1].wrapping_add(0x8), c.r[6]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[5]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[0]);
                c.lr = 0x80118058; f_800ff4d4(c);
                pc = 0x80118058;
            }
            0x80118058 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80118020: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118068 (0x80118068, 8 bytes)
#[inline(never)]
pub fn f_80118068(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118068) { return; }
        if c.prof { crate::prof::enter(c, 0x80118068); f_80118068_b(c, 0x80118068); crate::prof::exit(c, 0x80118068); return; }
    }
    f_80118068_b(c, 0x80118068)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118068_at(c: &mut Cpu, pc: u32) { f_80118068_b(c, pc) }
#[inline(always)]
fn f_80118068_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118068;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118068, pc); }
        match pc {
            0x80118068 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80118068: bad pc {:#x}", pc),
        }
    }
}

/// tvRelease2 (0x80118070, 144 bytes)
#[inline(never)]
pub fn f_80118070(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118070) { return; }
        if c.prof { crate::prof::enter(c, 0x80118070); f_80118070_b(c, 0x80118070); crate::prof::exit(c, 0x80118070); return; }
    }
    f_80118070_b(c, 0x80118070)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118070_at(c: &mut Cpu, pc: u32) { f_80118070_b(c, pc) }
#[inline(always)]
fn f_80118070_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118070;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118070, pc); }
        match pc {
            0x80118070 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa568));
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x801180e0; continue; }
                pc = 0x8011808c;
            }
            0x8011808c => {
                c.lr = 0x80118090; f_80113f48(c);
                pc = 0x80118090;
            }
            0x80118090 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x8));
                c.lr = 0x80118098; f_800f9318(c);
                pc = 0x80118098;
            }
            0x80118098 => {
                c.lr = 0x8011809c; f_800e5550(c);
                pc = 0x8011809c;
            }
            0x8011809c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa56c));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801180b4; continue; }
                pc = 0x801180a8;
            }
            0x801180a8 => {
                c.lr = 0x801180ac; f_800ef5a4(c);
                pc = 0x801180ac;
            }
            0x801180ac => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa56c), c.r[0]);
                pc = 0x801180b4;
            }
            0x801180b4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa570));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801180cc; continue; }
                pc = 0x801180c0;
            }
            0x801180c0 => {
                c.lr = 0x801180c4; f_800e4bf4(c);
                pc = 0x801180c4;
            }
            0x801180c4 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa570), c.r[0]);
                pc = 0x801180cc;
            }
            0x801180cc => {
                c.r[3] = 0x0;
                c.r[0] = 0xffffffff;
                c.stw(c.r[13].wrapping_add(0xffffa574), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffffa568), c.r[3]);
                c.stw(c.r[13].wrapping_add(0xffff8320), c.r[0]);
                pc = 0x801180e0;
            }
            0x801180e0 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa560), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa564), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118070: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118100 (0x80118100, 4 bytes)
#[inline(never)]
pub fn f_80118100(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118100) { return; }
        if c.prof { crate::prof::enter(c, 0x80118100); f_80118100_b(c, 0x80118100); crate::prof::exit(c, 0x80118100); return; }
    }
    f_80118100_b(c, 0x80118100)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118100_at(c: &mut Cpu, pc: u32) { f_80118100_b(c, pc) }
#[inline(always)]
fn f_80118100_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118100;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118100, pc); }
        match pc {
            0x80118100 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80118100: bad pc {:#x}", pc),
        }
    }
}

/// gsParticleDraw (0x80118104, 172 bytes)
#[inline(never)]
pub fn f_80118104(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118104) { return; }
        if c.prof { crate::prof::enter(c, 0x80118104); f_80118104_b(c, 0x80118104); crate::prof::exit(c, 0x80118104); return; }
    }
    f_80118104_b(c, 0x80118104)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118104_at(c: &mut Cpu, pc: u32) { f_80118104_b(c, pc) }
#[inline(always)]
fn f_80118104_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118104;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118104, pc); }
        match pc {
            0x80118104 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.lr = 0x80118128; f_800d2584(c);
                pc = 0x80118128;
            }
            0x80118128 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80118194; continue; }
                pc = 0x80118130;
            }
            0x80118130 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                c.lr = 0x80118138; f_80173624(c);
                pc = 0x80118138;
            }
            0x80118138 => {
                c.cmps(0, c.r[29], 0x1000);
                if c.crbit(2) { pc = 0x80118164; continue; }
                pc = 0x80118140;
            }
            0x80118140 => {
                if !c.crbit(0) { pc = 0x80118150; continue; }
                pc = 0x80118144;
            }
            0x80118144 => {
                c.cmps(0, c.r[29], 0x10);
                if c.crbit(2) { pc = 0x8011815c; continue; }
                pc = 0x8011814c;
            }
            0x8011814c => {
                pc = 0x80118170; continue;
                pc = 0x80118150;
            }
            0x80118150 => {
                c.cmps(0, c.r[29], 0x2000);
                if c.crbit(2) { pc = 0x8011816c; continue; }
                pc = 0x80118158;
            }
            0x80118158 => {
                pc = 0x80118170; continue;
                pc = 0x8011815c;
            }
            0x8011815c => {
                c.r[31] = 0x0;
                pc = 0x80118170; continue;
                pc = 0x80118164;
            }
            0x80118164 => {
                c.r[31] = 0x1;
                pc = 0x80118170; continue;
                pc = 0x8011816c;
            }
            0x8011816c => {
                c.r[31] = 0x2;
                pc = 0x80118170;
            }
            0x80118170 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80118188; continue; }
                pc = 0x80118178;
            }
            0x80118178 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x1;
                c.lr = 0x80118184; f_8016ab94(c);
                pc = 0x80118184;
            }
            0x80118184 => {
                pc = 0x80118194; continue;
                pc = 0x80118188;
            }
            0x80118188 => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = 0x2;
                c.lr = 0x80118194; f_8016ab94(c);
                pc = 0x80118194;
            }
            0x80118194 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80118104: bad pc {:#x}", pc),
        }
    }
}

/// fn_801181B0 (0x801181b0, 572 bytes)
#[inline(never)]
pub fn f_801181b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801181b0) { return; }
        if c.prof { crate::prof::enter(c, 0x801181b0); f_801181b0_b(c, 0x801181b0); crate::prof::exit(c, 0x801181b0); return; }
    }
    f_801181b0_b(c, 0x801181b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801181b0_at(c: &mut Cpu, pc: u32) { f_801181b0_b(c, pc) }
#[inline(always)]
fn f_801181b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801181b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801181b0, pc); }
        match pc {
            0x801181b0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = 0x0;
                c.r[30] = 0x0;
                pc = 0x801183cc; continue;
                pc = 0x801181cc;
            }
            0x801181cc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa57c));
                c.r[3] = c.r[0].wrapping_add(c.r[30]);
                c.r[0] = c.lbz(c.r[3]);
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x801183c4; continue; }
                pc = 0x801181e0;
            }
            0x801181e0 => {
                c.r[28] = 0x0;
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x801181e8;
            }
            0x801181e8 => {
                c.r[27] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[27], 0x0);
                if c.crbit(2) { pc = 0x801183b4; continue; }
                pc = 0x801181f4;
            }
            0x801181f4 => {
                c.r[0] = c.lbz(c.r[27].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801183b4; continue; }
                pc = 0x80118200;
            }
            0x80118200 => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80118210; continue; }
                pc = 0x80118208;
            }
            0x80118208 => {
                c.r[3] = 0xffffffff;
                pc = 0x80118218; continue;
                pc = 0x80118210;
            }
            0x80118210 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                c.lr = 0x80118218; f_801694e0(c);
                pc = 0x80118218;
            }
            0x80118218 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801183b4; continue; }
                pc = 0x80118220;
            }
            0x80118220 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                c.lr = 0x80118228; f_801694a8(c);
                pc = 0x80118228;
            }
            0x80118228 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801183b4; continue; }
                pc = 0x80118230;
            }
            0x80118230 => {
                c.r[4] = c.lwz(c.r[27].wrapping_add(0x10));
                c.r[3] = c.lhz(c.r[4].wrapping_add(0x18));
                c.r[4] = c.lbz(c.r[4].wrapping_add(0x15));
                c.lr = 0x80118240; f_801695fc(c);
                pc = 0x80118240;
            }
            0x80118240 => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80118364; continue; }
                pc = 0x8011824c;
            }
            0x8011824c => {
                if c.crbit(2) { pc = 0x80118364; continue; }
                pc = 0x80118250;
            }
            0x80118250 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x48));
                c.r[4] = 0x0;
                c.lr = 0x8011825c; f_800ec160(c);
                pc = 0x8011825c;
            }
            0x8011825c => {
                c.r[0] = 0x0;
                c.stw(c.r[27].wrapping_add(0x48), c.r[0]);
                c.stw(c.r[27].wrapping_add(0x4c), c.r[0]);
                c.stb(c.r[27].wrapping_add(0x6), c.r[0]);
                c.stb(c.r[27].wrapping_add(0x5), c.r[0]);
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                c.lr = 0x80118278; f_80169484(c);
                pc = 0x80118278;
            }
            0x80118278 => {
                c.r[0] = 0x0;
                c.stw(c.r[27].wrapping_add(0x44), c.r[0]);
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801182c0; continue; }
                pc = 0x8011828c;
            }
            0x8011828c => {
                c.r[3] = c.r[27].wrapping_add(0x14);
                c.r[4] = c.r[27].wrapping_add(0x50);
                c.lr = 0x80118298; f_800e01d0(c);
                pc = 0x80118298;
            }
            0x80118298 => {
                { let ea = c.r[27].wrapping_add(0x50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[27].wrapping_add(0x54); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[27].wrapping_add(0x58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801182cc; continue;
                pc = 0x801182c0;
            }
            0x801182c0 => {
                c.r[3] = c.r[27].wrapping_add(0x50);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x801182cc; f_800e01d0(c);
                pc = 0x801182cc;
            }
            0x801182cc => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8011830c; continue; }
                pc = 0x801182d8;
            }
            0x801182d8 => {
                c.r[3] = c.r[27].wrapping_add(0x20);
                c.r[4] = c.r[27].wrapping_add(0x5c);
                c.lr = 0x801182e4; f_800e01d0(c);
                pc = 0x801182e4;
            }
            0x801182e4 => {
                { let ea = c.r[27].wrapping_add(0x5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[27].wrapping_add(0x60); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[27].wrapping_add(0x64); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118318; continue;
                pc = 0x8011830c;
            }
            0x8011830c => {
                c.r[3] = c.r[27].wrapping_add(0x5c);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80118318; f_800e01d0(c);
                pc = 0x80118318;
            }
            0x80118318 => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80118358; continue; }
                pc = 0x80118324;
            }
            0x80118324 => {
                c.r[3] = c.r[27].wrapping_add(0x2c);
                c.r[4] = c.r[27].wrapping_add(0x68);
                c.lr = 0x80118330; f_800e01d0(c);
                pc = 0x80118330;
            }
            0x80118330 => {
                { let ea = c.r[27].wrapping_add(0x68); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[27].wrapping_add(0x6c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[27].wrapping_add(0x70); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118364; continue;
                pc = 0x80118358;
            }
            0x80118358 => {
                c.r[3] = c.r[27].wrapping_add(0x68);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80118364; f_800e01d0(c);
                pc = 0x80118364;
            }
            0x80118364 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                c.lr = 0x8011836c; f_80175a1c(c);
                pc = 0x8011836c;
            }
            0x8011836c => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0xc));
                c.r[0] = 0x40;
                c.r[3] = 0x0;
                c.r[4] = c.r[5] | c.r[5];
                c.mtspr(9, c.r[0]);
                pc = 0x80118380;
            }
            0x80118380 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.cmpu(0, c.r[0], c.r[27]);
                if !c.crbit(2) { pc = 0x801183a0; continue; }
                pc = 0x8011838c;
            }
            0x8011838c => {
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[4] = 0x0;
                c.r[0] = c.r[3].wrapping_add(0x8);
                c.stw(c.r[5].wrapping_add(c.r[0]), c.r[4]);
                pc = 0x801183ac; continue;
                pc = 0x801183a0;
            }
            0x801183a0 => {
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80118380; continue; }
                pc = 0x801183ac;
            }
            0x801183ac => {
                c.r[0] = 0x0;
                c.stb(c.r[27], c.r[0]);
                pc = 0x801183b4;
            }
            0x801183b4 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x4);
                c.cmpu(0, c.r[28], 0x40);
                if c.crbit(0) { pc = 0x801181e8; continue; }
                pc = 0x801183c4;
            }
            0x801183c4 => {
                c.r[30] = c.r[30].wrapping_add(0x108);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801183cc;
            }
            0x801183cc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa580));
                c.cmpu(0, c.r[29], c.r[0]);
                if c.crbit(0) { pc = 0x801181cc; continue; }
                pc = 0x801183d8;
            }
            0x801183d8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801181b0: bad pc {:#x}", pc),
        }
    }
}

/// gsParticleUpdate (0x801183ec, 1160 bytes)
#[inline(never)]
pub fn f_801183ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801183ec) { return; }
        if c.prof { crate::prof::enter(c, 0x801183ec); f_801183ec_b(c, 0x801183ec); crate::prof::exit(c, 0x801183ec); return; }
    }
    f_801183ec_b(c, 0x801183ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801183ec_at(c: &mut Cpu, pc: u32) { f_801183ec_b(c, pc) }
#[inline(always)]
fn f_801183ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801183ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801183ec, pc); }
        match pc {
            0x801183ec => {
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x801183f0;
            }
            0x801183f0 => {
                c.r[0] = c.mfspr(8);
                pc = 0x801183f4;
            }
            0x801183f4 => {
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                pc = 0x801183f8;
            }
            0x801183f8 => {
                { let mut ea = c.r[1].wrapping_add(0x54); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x801183fc;
            }
            0x801183fc => {
                c.r[26] = c.r[3] | c.r[3];
                pc = 0x80118400;
            }
            0x80118400 => {
                c.r[29] = 0x0;
                pc = 0x80118404;
            }
            0x80118404 => {
                c.r[30] = 0x0;
                pc = 0x80118408;
            }
            0x80118408 => {
                pc = 0x801187f4; continue;
                pc = 0x8011840c;
            }
            0x8011840c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa57c));
                pc = 0x80118410;
            }
            0x80118410 => {
                c.r[3] = c.r[0].wrapping_add(c.r[30]);
                pc = 0x80118414;
            }
            0x80118414 => {
                c.r[0] = c.lbz(c.r[3]);
                pc = 0x80118418;
            }
            0x80118418 => {
                c.cmpu(0, c.r[0], 0x1);
                pc = 0x8011841c;
            }
            0x8011841c => {
                if !c.crbit(2) { pc = 0x801187ec; continue; }
                pc = 0x80118420;
            }
            0x80118420 => {
                c.r[28] = 0x0;
                pc = 0x80118424;
            }
            0x80118424 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80118428;
            }
            0x80118428 => {
                c.r[27] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x8011842c;
            }
            0x8011842c => {
                c.cmpu(0, c.r[27], 0x0);
                pc = 0x80118430;
            }
            0x80118430 => {
                if c.crbit(2) { pc = 0x801187dc; continue; }
                pc = 0x80118434;
            }
            0x80118434 => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x44));
                pc = 0x80118438;
            }
            0x80118438 => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x8011843c;
            }
            0x8011843c => {
                if c.crbit(2) { pc = 0x801187dc; continue; }
                pc = 0x80118440;
            }
            0x80118440 => {
                c.r[0] = c.lbz(c.r[27].wrapping_add(0x6));
                pc = 0x80118444;
            }
            0x80118444 => {
                c.cmpu(0, c.r[0], 0x0);
                pc = 0x80118448;
            }
            0x80118448 => {
                if c.crbit(2) { pc = 0x801187dc; continue; }
                pc = 0x8011844c;
            }
            0x8011844c => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x48));
                pc = 0x80118450;
            }
            0x80118450 => {
                c.r[4] = c.lwz(c.r[27].wrapping_add(0x4c));
                pc = 0x80118454;
            }
            0x80118454 => {
                c.lr = 0x80118458; f_800ee150(c);
                pc = 0x80118458;
            }
            0x80118458 => {
                c.r[4] = c.r[1].wrapping_add(0x3c);
                pc = 0x8011845c;
            }
            0x8011845c => {
                c.r[25] = c.r[3] | c.r[3];
                pc = 0x80118460;
            }
            0x80118460 => {
                c.r[5] = c.r[1].wrapping_add(0x30);
                pc = 0x80118464;
            }
            0x80118464 => {
                c.r[6] = c.r[1].wrapping_add(0x24);
                pc = 0x80118468;
            }
            0x80118468 => {
                c.lr = 0x8011846c; f_800ee3bc(c);
                pc = 0x8011846c;
            }
            0x8011846c => {
                c.r[3] = c.r[25] | c.r[25];
                pc = 0x80118470;
            }
            0x80118470 => {
                c.lr = 0x80118474; f_800ee828(c);
                pc = 0x80118474;
            }
            0x80118474 => {
                c.r[3] = c.r[1].wrapping_add(0x14);
                pc = 0x80118478;
            }
            0x80118478 => {
                c.r[4] = c.r[1].wrapping_add(0x30);
                pc = 0x8011847c;
            }
            0x8011847c => {
                c.lr = 0x80118480; f_800e06ec(c);
                pc = 0x80118480;
            }
            0x80118480 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                pc = 0x80118484;
            }
            0x80118484 => {
                c.r[4] = c.r[1].wrapping_add(0x14);
                pc = 0x80118488;
            }
            0x80118488 => {
                c.r[5] = c.r[27].wrapping_add(0x50);
                pc = 0x8011848c;
            }
            0x8011848c => {
                c.lr = 0x80118490; f_800dfeec(c);
                pc = 0x80118490;
            }
            0x80118490 => {
                c.r[3] = c.r[1].wrapping_add(0x3c);
                pc = 0x80118494;
            }
            0x80118494 => {
                c.r[5] = c.r[1].wrapping_add(0x8);
                pc = 0x80118498;
            }
            0x80118498 => {
                c.r[4] = c.r[3] | c.r[3];
                pc = 0x8011849c;
            }
            0x8011849c => {
                c.lr = 0x801184a0; f_800e019c(c);
                pc = 0x801184a0;
            }
            0x801184a0 => {
                c.r[3] = c.r[1].wrapping_add(0x30);
                pc = 0x801184a4;
            }
            0x801184a4 => {
                c.r[5] = c.r[27].wrapping_add(0x5c);
                pc = 0x801184a8;
            }
            0x801184a8 => {
                c.r[4] = c.r[3] | c.r[3];
                pc = 0x801184ac;
            }
            0x801184ac => {
                c.lr = 0x801184b0; f_800e019c(c);
                pc = 0x801184b0;
            }
            0x801184b0 => {
                c.r[3] = c.r[1].wrapping_add(0x24);
                pc = 0x801184b4;
            }
            0x801184b4 => {
                c.r[5] = c.r[27].wrapping_add(0x68);
                pc = 0x801184b8;
            }
            0x801184b8 => {
                c.r[4] = c.r[3] | c.r[3];
                pc = 0x801184bc;
            }
            0x801184bc => {
                c.lr = 0x801184c0; f_800e0108(c);
                pc = 0x801184c0;
            }
            0x801184c0 => {
                c.r[3] = c.r[27].wrapping_add(0x14);
                pc = 0x801184c4;
            }
            0x801184c4 => {
                c.r[4] = c.r[27].wrapping_add(0x50);
                pc = 0x801184c8;
            }
            0x801184c8 => {
                c.lr = 0x801184cc; f_800e01d0(c);
                pc = 0x801184cc;
            }
            0x801184cc => {
                { let ea = c.r[27].wrapping_add(0x50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801184d0;
            }
            0x801184d0 => {
                c.r[3] = c.r[27].wrapping_add(0x20);
                pc = 0x801184d4;
            }
            0x801184d4 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801184d8;
            }
            0x801184d8 => {
                c.r[4] = c.r[27].wrapping_add(0x5c);
                pc = 0x801184dc;
            }
            0x801184dc => {
                { let ea = c.r[5].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801184e0;
            }
            0x801184e0 => {
                { let ea = c.r[27].wrapping_add(0x54); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801184e4;
            }
            0x801184e4 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801184e8;
            }
            0x801184e8 => {
                { let ea = c.r[5].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801184ec;
            }
            0x801184ec => {
                { let ea = c.r[27].wrapping_add(0x58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801184f0;
            }
            0x801184f0 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801184f4;
            }
            0x801184f4 => {
                { let ea = c.r[5].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801184f8;
            }
            0x801184f8 => {
                c.lr = 0x801184fc; f_800e01d0(c);
                pc = 0x801184fc;
            }
            0x801184fc => {
                { let ea = c.r[27].wrapping_add(0x5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118500;
            }
            0x80118500 => {
                c.r[3] = c.r[27].wrapping_add(0x2c);
                pc = 0x80118504;
            }
            0x80118504 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118508;
            }
            0x80118508 => {
                c.r[4] = c.r[27].wrapping_add(0x68);
                pc = 0x8011850c;
            }
            0x8011850c => {
                { let ea = c.r[5].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118510;
            }
            0x80118510 => {
                { let ea = c.r[27].wrapping_add(0x60); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118514;
            }
            0x80118514 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118518;
            }
            0x80118518 => {
                { let ea = c.r[5].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8011851c;
            }
            0x8011851c => {
                { let ea = c.r[27].wrapping_add(0x64); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118520;
            }
            0x80118520 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118524;
            }
            0x80118524 => {
                { let ea = c.r[5].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118528;
            }
            0x80118528 => {
                c.lr = 0x8011852c; f_800e01d0(c);
                pc = 0x8011852c;
            }
            0x8011852c => {
                { let ea = c.r[27].wrapping_add(0x68); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118530;
            }
            0x80118530 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118534;
            }
            0x80118534 => {
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118538;
            }
            0x80118538 => {
                { let ea = c.r[27].wrapping_add(0x6c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8011853c;
            }
            0x8011853c => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118540;
            }
            0x80118540 => {
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118544;
            }
            0x80118544 => {
                { let ea = c.r[27].wrapping_add(0x70); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118548;
            }
            0x80118548 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x8011854c;
            }
            0x8011854c => {
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118550;
            }
            0x80118550 => {
                c.r[0] = c.lwz(c.r[27].wrapping_add(0x44));
                pc = 0x80118554;
            }
            0x80118554 => {
                c.cmpu(0, c.r[0], 0x7);
                pc = 0x80118558;
            }
            0x80118558 => {
                if c.crbit(1) { pc = 0x801187cc; continue; }
                pc = 0x8011855c;
            }
            0x8011855c => {
                c.r[3] = 0x80360000;
                pc = 0x80118560;
            }
            0x80118560 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80118564;
            }
            0x80118564 => {
                c.r[3] = c.r[3].wrapping_add(0xffffbb88);
                pc = 0x80118568;
            }
            0x80118568 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x8011856c;
            }
            0x8011856c => {
                c.mtspr(9, c.r[0]);
                pc = 0x80118570;
            }
            0x80118570 => {
                if true { let t = c.ctr; if (0x801183ec..0x80118874).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80118574;
            }
            0x80118574 => {
                c.r[3] = c.r[27].wrapping_add(0x14);
                pc = 0x80118578;
            }
            0x80118578 => {
                c.r[4] = c.r[1].wrapping_add(0x3c);
                pc = 0x8011857c;
            }
            0x8011857c => {
                c.lr = 0x80118580; f_800e01d0(c);
                pc = 0x80118580;
            }
            0x80118580 => {
                { let ea = c.r[1].wrapping_add(0x3c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118584;
            }
            0x80118584 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118588;
            }
            0x80118588 => {
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8011858c;
            }
            0x8011858c => {
                { let ea = c.r[1].wrapping_add(0x40); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118590;
            }
            0x80118590 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118594;
            }
            0x80118594 => {
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118598;
            }
            0x80118598 => {
                { let ea = c.r[1].wrapping_add(0x44); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8011859c;
            }
            0x8011859c => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801185a0;
            }
            0x801185a0 => {
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801185a4;
            }
            0x801185a4 => {
                pc = 0x801187cc; continue;
                pc = 0x801185a8;
            }
            0x801185a8 => {
                c.r[3] = c.r[27].wrapping_add(0x20);
                pc = 0x801185ac;
            }
            0x801185ac => {
                c.r[4] = c.r[1].wrapping_add(0x30);
                pc = 0x801185b0;
            }
            0x801185b0 => {
                c.lr = 0x801185b4; f_800e01d0(c);
                pc = 0x801185b4;
            }
            0x801185b4 => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801185b8;
            }
            0x801185b8 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801185bc;
            }
            0x801185bc => {
                { let ea = c.r[3].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801185c0;
            }
            0x801185c0 => {
                { let ea = c.r[1].wrapping_add(0x34); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801185c4;
            }
            0x801185c4 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801185c8;
            }
            0x801185c8 => {
                { let ea = c.r[3].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801185cc;
            }
            0x801185cc => {
                { let ea = c.r[1].wrapping_add(0x38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801185d0;
            }
            0x801185d0 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801185d4;
            }
            0x801185d4 => {
                { let ea = c.r[3].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801185d8;
            }
            0x801185d8 => {
                pc = 0x801187cc; continue;
                pc = 0x801185dc;
            }
            0x801185dc => {
                c.r[3] = c.r[27].wrapping_add(0x2c);
                pc = 0x801185e0;
            }
            0x801185e0 => {
                c.r[4] = c.r[1].wrapping_add(0x24);
                pc = 0x801185e4;
            }
            0x801185e4 => {
                c.lr = 0x801185e8; f_800e01d0(c);
                pc = 0x801185e8;
            }
            0x801185e8 => {
                { let ea = c.r[1].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801185ec;
            }
            0x801185ec => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801185f0;
            }
            0x801185f0 => {
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801185f4;
            }
            0x801185f4 => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801185f8;
            }
            0x801185f8 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801185fc;
            }
            0x801185fc => {
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118600;
            }
            0x80118600 => {
                { let ea = c.r[1].wrapping_add(0x2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118604;
            }
            0x80118604 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118608;
            }
            0x80118608 => {
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8011860c;
            }
            0x8011860c => {
                pc = 0x801187cc; continue;
                pc = 0x80118610;
            }
            0x80118610 => {
                c.r[3] = c.r[27].wrapping_add(0x14);
                pc = 0x80118614;
            }
            0x80118614 => {
                c.r[4] = c.r[1].wrapping_add(0x3c);
                pc = 0x80118618;
            }
            0x80118618 => {
                c.lr = 0x8011861c; f_800e01d0(c);
                pc = 0x8011861c;
            }
            0x8011861c => {
                { let ea = c.r[1].wrapping_add(0x3c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118620;
            }
            0x80118620 => {
                c.r[3] = c.r[27].wrapping_add(0x20);
                pc = 0x80118624;
            }
            0x80118624 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118628;
            }
            0x80118628 => {
                c.r[4] = c.r[1].wrapping_add(0x30);
                pc = 0x8011862c;
            }
            0x8011862c => {
                { let ea = c.r[5].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118630;
            }
            0x80118630 => {
                { let ea = c.r[1].wrapping_add(0x40); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118634;
            }
            0x80118634 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118638;
            }
            0x80118638 => {
                { let ea = c.r[5].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8011863c;
            }
            0x8011863c => {
                { let ea = c.r[1].wrapping_add(0x44); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118640;
            }
            0x80118640 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118644;
            }
            0x80118644 => {
                { let ea = c.r[5].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118648;
            }
            0x80118648 => {
                c.lr = 0x8011864c; f_800e01d0(c);
                pc = 0x8011864c;
            }
            0x8011864c => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118650;
            }
            0x80118650 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118654;
            }
            0x80118654 => {
                { let ea = c.r[3].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118658;
            }
            0x80118658 => {
                { let ea = c.r[1].wrapping_add(0x34); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8011865c;
            }
            0x8011865c => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118660;
            }
            0x80118660 => {
                { let ea = c.r[3].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118664;
            }
            0x80118664 => {
                { let ea = c.r[1].wrapping_add(0x38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118668;
            }
            0x80118668 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x8011866c;
            }
            0x8011866c => {
                { let ea = c.r[3].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118670;
            }
            0x80118670 => {
                pc = 0x801187cc; continue;
                pc = 0x80118674;
            }
            0x80118674 => {
                c.r[3] = c.r[27].wrapping_add(0x20);
                pc = 0x80118678;
            }
            0x80118678 => {
                c.r[4] = c.r[1].wrapping_add(0x30);
                pc = 0x8011867c;
            }
            0x8011867c => {
                c.lr = 0x80118680; f_800e01d0(c);
                pc = 0x80118680;
            }
            0x80118680 => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118684;
            }
            0x80118684 => {
                c.r[3] = c.r[27].wrapping_add(0x2c);
                pc = 0x80118688;
            }
            0x80118688 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x8011868c;
            }
            0x8011868c => {
                c.r[4] = c.r[1].wrapping_add(0x24);
                pc = 0x80118690;
            }
            0x80118690 => {
                { let ea = c.r[5].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118694;
            }
            0x80118694 => {
                { let ea = c.r[1].wrapping_add(0x34); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118698;
            }
            0x80118698 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x8011869c;
            }
            0x8011869c => {
                { let ea = c.r[5].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801186a0;
            }
            0x801186a0 => {
                { let ea = c.r[1].wrapping_add(0x38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801186a4;
            }
            0x801186a4 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801186a8;
            }
            0x801186a8 => {
                { let ea = c.r[5].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801186ac;
            }
            0x801186ac => {
                c.lr = 0x801186b0; f_800e01d0(c);
                pc = 0x801186b0;
            }
            0x801186b0 => {
                { let ea = c.r[1].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801186b4;
            }
            0x801186b4 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801186b8;
            }
            0x801186b8 => {
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801186bc;
            }
            0x801186bc => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801186c0;
            }
            0x801186c0 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801186c4;
            }
            0x801186c4 => {
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801186c8;
            }
            0x801186c8 => {
                { let ea = c.r[1].wrapping_add(0x2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801186cc;
            }
            0x801186cc => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801186d0;
            }
            0x801186d0 => {
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801186d4;
            }
            0x801186d4 => {
                pc = 0x801187cc; continue;
                pc = 0x801186d8;
            }
            0x801186d8 => {
                c.r[3] = c.r[27].wrapping_add(0x14);
                pc = 0x801186dc;
            }
            0x801186dc => {
                c.r[4] = c.r[1].wrapping_add(0x3c);
                pc = 0x801186e0;
            }
            0x801186e0 => {
                c.lr = 0x801186e4; f_800e01d0(c);
                pc = 0x801186e4;
            }
            0x801186e4 => {
                { let ea = c.r[1].wrapping_add(0x3c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801186e8;
            }
            0x801186e8 => {
                c.r[3] = c.r[27].wrapping_add(0x2c);
                pc = 0x801186ec;
            }
            0x801186ec => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801186f0;
            }
            0x801186f0 => {
                c.r[4] = c.r[1].wrapping_add(0x24);
                pc = 0x801186f4;
            }
            0x801186f4 => {
                { let ea = c.r[5].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801186f8;
            }
            0x801186f8 => {
                { let ea = c.r[1].wrapping_add(0x40); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801186fc;
            }
            0x801186fc => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118700;
            }
            0x80118700 => {
                { let ea = c.r[5].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118704;
            }
            0x80118704 => {
                { let ea = c.r[1].wrapping_add(0x44); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118708;
            }
            0x80118708 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x8011870c;
            }
            0x8011870c => {
                { let ea = c.r[5].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118710;
            }
            0x80118710 => {
                c.lr = 0x80118714; f_800e01d0(c);
                pc = 0x80118714;
            }
            0x80118714 => {
                { let ea = c.r[1].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118718;
            }
            0x80118718 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x8011871c;
            }
            0x8011871c => {
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118720;
            }
            0x80118720 => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118724;
            }
            0x80118724 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118728;
            }
            0x80118728 => {
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8011872c;
            }
            0x8011872c => {
                { let ea = c.r[1].wrapping_add(0x2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118730;
            }
            0x80118730 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118734;
            }
            0x80118734 => {
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118738;
            }
            0x80118738 => {
                pc = 0x801187cc; continue;
                pc = 0x8011873c;
            }
            0x8011873c => {
                c.r[3] = c.r[27].wrapping_add(0x14);
                pc = 0x80118740;
            }
            0x80118740 => {
                c.r[4] = c.r[1].wrapping_add(0x3c);
                pc = 0x80118744;
            }
            0x80118744 => {
                c.lr = 0x80118748; f_800e01d0(c);
                pc = 0x80118748;
            }
            0x80118748 => {
                { let ea = c.r[1].wrapping_add(0x3c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8011874c;
            }
            0x8011874c => {
                c.r[3] = c.r[27].wrapping_add(0x20);
                pc = 0x80118750;
            }
            0x80118750 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118754;
            }
            0x80118754 => {
                c.r[4] = c.r[1].wrapping_add(0x30);
                pc = 0x80118758;
            }
            0x80118758 => {
                { let ea = c.r[5].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8011875c;
            }
            0x8011875c => {
                { let ea = c.r[1].wrapping_add(0x40); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118760;
            }
            0x80118760 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118764;
            }
            0x80118764 => {
                { let ea = c.r[5].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118768;
            }
            0x80118768 => {
                { let ea = c.r[1].wrapping_add(0x44); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8011876c;
            }
            0x8011876c => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118770;
            }
            0x80118770 => {
                { let ea = c.r[5].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118774;
            }
            0x80118774 => {
                c.lr = 0x80118778; f_800e01d0(c);
                pc = 0x80118778;
            }
            0x80118778 => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8011877c;
            }
            0x8011877c => {
                c.r[3] = c.r[27].wrapping_add(0x2c);
                pc = 0x80118780;
            }
            0x80118780 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118784;
            }
            0x80118784 => {
                c.r[4] = c.r[1].wrapping_add(0x24);
                pc = 0x80118788;
            }
            0x80118788 => {
                { let ea = c.r[5].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8011878c;
            }
            0x8011878c => {
                { let ea = c.r[1].wrapping_add(0x34); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80118790;
            }
            0x80118790 => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x80118794;
            }
            0x80118794 => {
                { let ea = c.r[5].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118798;
            }
            0x80118798 => {
                { let ea = c.r[1].wrapping_add(0x38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8011879c;
            }
            0x8011879c => {
                c.r[5] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801187a0;
            }
            0x801187a0 => {
                { let ea = c.r[5].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801187a4;
            }
            0x801187a4 => {
                c.lr = 0x801187a8; f_800e01d0(c);
                pc = 0x801187a8;
            }
            0x801187a8 => {
                { let ea = c.r[1].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801187ac;
            }
            0x801187ac => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801187b0;
            }
            0x801187b0 => {
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801187b4;
            }
            0x801187b4 => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801187b8;
            }
            0x801187b8 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801187bc;
            }
            0x801187bc => {
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801187c0;
            }
            0x801187c0 => {
                { let ea = c.r[1].wrapping_add(0x2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801187c4;
            }
            0x801187c4 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                pc = 0x801187c8;
            }
            0x801187c8 => {
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801187cc;
            }
            0x801187cc => {
                c.r[0] = c.lbz(c.r[27].wrapping_add(0x5));
                pc = 0x801187d0;
            }
            0x801187d0 => {
                c.r[0] = c.r[0].leading_zeros();
                pc = 0x801187d4;
            }
            0x801187d4 => {
                c.r[0] = c.r[0].rotate_left(27) & 0xff;
                pc = 0x801187d8;
            }
            0x801187d8 => {
                c.stb(c.r[27].wrapping_add(0x6), c.r[0]);
                pc = 0x801187dc;
            }
            0x801187dc => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x801187e0;
            }
            0x801187e0 => {
                c.r[31] = c.r[31].wrapping_add(0x4);
                pc = 0x801187e4;
            }
            0x801187e4 => {
                c.cmpu(0, c.r[28], 0x40);
                pc = 0x801187e8;
            }
            0x801187e8 => {
                if c.crbit(0) { pc = 0x80118428; continue; }
                pc = 0x801187ec;
            }
            0x801187ec => {
                c.r[30] = c.r[30].wrapping_add(0x108);
                pc = 0x801187f0;
            }
            0x801187f0 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801187f4;
            }
            0x801187f4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa580));
                pc = 0x801187f8;
            }
            0x801187f8 => {
                c.cmpu(0, c.r[29], c.r[0]);
                pc = 0x801187fc;
            }
            0x801187fc => {
                if c.crbit(0) { pc = 0x8011840c; continue; }
                pc = 0x80118800;
            }
            0x80118800 => {
                c.r[27] = 0x0;
                pc = 0x80118804;
            }
            0x80118804 => {
                pc = 0x8011881c; continue;
                pc = 0x80118808;
            }
            0x80118808 => {
                c.r[3] = 0x0;
                pc = 0x8011880c;
            }
            0x8011880c => {
                c.lr = 0x80118810; f_8016f430(c);
                pc = 0x80118810;
            }
            0x80118810 => {
                c.r[3] = 0x0;
                pc = 0x80118814;
            }
            0x80118814 => {
                c.lr = 0x80118818; f_80173f98(c);
                pc = 0x80118818;
            }
            0x80118818 => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                pc = 0x8011881c;
            }
            0x8011881c => {
                c.cmpu(0, c.r[27], c.r[26]);
                pc = 0x80118820;
            }
            0x80118820 => {
                if c.crbit(0) { pc = 0x80118808; continue; }
                pc = 0x80118824;
            }
            0x80118824 => {
                c.lr = 0x80118828; f_800057a0(c);
                pc = 0x80118828;
            }
            0x80118828 => {
                c.cmps(0, c.r[3], 0x2);
                pc = 0x8011882c;
            }
            0x8011882c => {
                if !c.crbit(2) { pc = 0x80118860; continue; }
                pc = 0x80118830;
            }
            0x80118830 => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffffa590));
                pc = 0x80118834;
            }
            0x80118834 => {
                c.r[3] = c.r[3].wrapping_add(0x1);
                pc = 0x80118838;
            }
            0x80118838 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011883c;
            }
            0x8011883c => {
                c.stb(c.r[13].wrapping_add(0xffffa590), c.r[3]);
                pc = 0x80118840;
            }
            0x80118840 => {
                c.cmpu(0, c.r[0], 0x5);
                pc = 0x80118844;
            }
            0x80118844 => {
                if c.crbit(0) { pc = 0x80118860; continue; }
                pc = 0x80118848;
            }
            0x80118848 => {
                c.r[3] = 0x0;
                pc = 0x8011884c;
            }
            0x8011884c => {
                c.lr = 0x80118850; f_8016f430(c);
                pc = 0x80118850;
            }
            0x80118850 => {
                c.r[3] = 0x0;
                pc = 0x80118854;
            }
            0x80118854 => {
                c.lr = 0x80118858; f_80173f98(c);
                pc = 0x80118858;
            }
            0x80118858 => {
                c.r[0] = 0x0;
                pc = 0x8011885c;
            }
            0x8011885c => {
                c.stb(c.r[13].wrapping_add(0xffffa590), c.r[0]);
                pc = 0x80118860;
            }
            0x80118860 => {
                { let mut ea = c.r[1].wrapping_add(0x54); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                pc = 0x80118864;
            }
            0x80118864 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x74));
                pc = 0x80118868;
            }
            0x80118868 => {
                c.mtspr(8, c.r[0]);
                pc = 0x8011886c;
            }
            0x8011886c => {
                c.r[1] = c.r[1].wrapping_add(0x70);
                pc = 0x80118870;
            }
            0x80118870 => {
                if true { return; }
                return;
            }
            _ => panic!("f_801183ec: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118874 (0x80118874, 500 bytes)
#[inline(never)]
pub fn f_80118874(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118874) { return; }
        if c.prof { crate::prof::enter(c, 0x80118874); f_80118874_b(c, 0x80118874); crate::prof::exit(c, 0x80118874); return; }
    }
    f_80118874_b(c, 0x80118874)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118874_at(c: &mut Cpu, pc: u32) { f_80118874_b(c, pc) }
#[inline(always)]
fn f_80118874_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118874;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118874, pc); }
        match pc {
            0x80118874 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = 0x0;
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[4] | c.r[4];
                pc = 0x8011889c;
            }
            0x8011889c => {
                c.r[30] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x80118a38; continue; }
                pc = 0x801188a8;
            }
            0x801188a8 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x801188c4; continue; }
                pc = 0x801188b4;
            }
            0x801188b4 => {
                c.r[4] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[3] = c.lhz(c.r[4].wrapping_add(0x18));
                c.r[4] = c.lbz(c.r[4].wrapping_add(0x15));
                c.lr = 0x801188c4; f_801695fc(c);
                pc = 0x801188c4;
            }
            0x801188c4 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801189e8; continue; }
                pc = 0x801188d0;
            }
            0x801188d0 => {
                if c.crbit(2) { pc = 0x801189e8; continue; }
                pc = 0x801188d4;
            }
            0x801188d4 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x48));
                c.r[4] = 0x0;
                c.lr = 0x801188e0; f_800ec160(c);
                pc = 0x801188e0;
            }
            0x801188e0 => {
                c.r[0] = 0x0;
                c.stw(c.r[30].wrapping_add(0x48), c.r[0]);
                c.stw(c.r[30].wrapping_add(0x4c), c.r[0]);
                c.stb(c.r[30].wrapping_add(0x6), c.r[0]);
                c.stb(c.r[30].wrapping_add(0x5), c.r[0]);
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                c.lr = 0x801188fc; f_80169484(c);
                pc = 0x801188fc;
            }
            0x801188fc => {
                c.r[0] = 0x0;
                c.stw(c.r[30].wrapping_add(0x44), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80118944; continue; }
                pc = 0x80118910;
            }
            0x80118910 => {
                c.r[3] = c.r[30].wrapping_add(0x14);
                c.r[4] = c.r[30].wrapping_add(0x50);
                c.lr = 0x8011891c; f_800e01d0(c);
                pc = 0x8011891c;
            }
            0x8011891c => {
                { let ea = c.r[30].wrapping_add(0x50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x54); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118950; continue;
                pc = 0x80118944;
            }
            0x80118944 => {
                c.r[3] = c.r[30].wrapping_add(0x50);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80118950; f_800e01d0(c);
                pc = 0x80118950;
            }
            0x80118950 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80118990; continue; }
                pc = 0x8011895c;
            }
            0x8011895c => {
                c.r[3] = c.r[30].wrapping_add(0x20);
                c.r[4] = c.r[30].wrapping_add(0x5c);
                c.lr = 0x80118968; f_800e01d0(c);
                pc = 0x80118968;
            }
            0x80118968 => {
                { let ea = c.r[30].wrapping_add(0x5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x60); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x64); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8011899c; continue;
                pc = 0x80118990;
            }
            0x80118990 => {
                c.r[3] = c.r[30].wrapping_add(0x5c);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x8011899c; f_800e01d0(c);
                pc = 0x8011899c;
            }
            0x8011899c => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801189dc; continue; }
                pc = 0x801189a8;
            }
            0x801189a8 => {
                c.r[3] = c.r[30].wrapping_add(0x2c);
                c.r[4] = c.r[30].wrapping_add(0x68);
                c.lr = 0x801189b4; f_800e01d0(c);
                pc = 0x801189b4;
            }
            0x801189b4 => {
                { let ea = c.r[30].wrapping_add(0x68); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x6c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x70); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801189e8; continue;
                pc = 0x801189dc;
            }
            0x801189dc => {
                c.r[3] = c.r[30].wrapping_add(0x68);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x801189e8; f_800e01d0(c);
                pc = 0x801189e8;
            }
            0x801189e8 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                c.lr = 0x801189f0; f_80175a1c(c);
                pc = 0x801189f0;
            }
            0x801189f0 => {
                c.r[5] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = 0x40;
                c.r[3] = 0x0;
                c.r[4] = c.r[5] | c.r[5];
                c.mtspr(9, c.r[0]);
                pc = 0x80118a04;
            }
            0x80118a04 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.cmpu(0, c.r[0], c.r[30]);
                if !c.crbit(2) { pc = 0x80118a24; continue; }
                pc = 0x80118a10;
            }
            0x80118a10 => {
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[4] = 0x0;
                c.r[0] = c.r[3].wrapping_add(0x8);
                c.stw(c.r[5].wrapping_add(c.r[0]), c.r[4]);
                pc = 0x80118a30; continue;
                pc = 0x80118a24;
            }
            0x80118a24 => {
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80118a04; continue; }
                pc = 0x80118a30;
            }
            0x80118a30 => {
                c.r[0] = 0x0;
                c.stb(c.r[30], c.r[0]);
                pc = 0x80118a38;
            }
            0x80118a38 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x4);
                c.cmpu(0, c.r[29], 0x40);
                if c.crbit(0) { pc = 0x8011889c; continue; }
                pc = 0x80118a48;
            }
            0x80118a48 => {
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
            _ => panic!("f_80118874: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118A68 (0x80118a68, 440 bytes)
#[inline(never)]
pub fn f_80118a68(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118a68) { return; }
        if c.prof { crate::prof::enter(c, 0x80118a68); f_80118a68_b(c, 0x80118a68); crate::prof::exit(c, 0x80118a68); return; }
    }
    f_80118a68_b(c, 0x80118a68)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118a68_at(c: &mut Cpu, pc: u32) { f_80118a68_b(c, pc) }
#[inline(always)]
fn f_80118a68_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118a68;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118a68, pc); }
        match pc {
            0x80118a68 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x80118a98; continue; }
                pc = 0x80118a88;
            }
            0x80118a88 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[3] = c.lhz(c.r[4].wrapping_add(0x18));
                c.r[4] = c.lbz(c.r[4].wrapping_add(0x15));
                c.lr = 0x80118a98; f_801695fc(c);
                pc = 0x80118a98;
            }
            0x80118a98 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80118bbc; continue; }
                pc = 0x80118aa4;
            }
            0x80118aa4 => {
                if c.crbit(2) { pc = 0x80118bbc; continue; }
                pc = 0x80118aa8;
            }
            0x80118aa8 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = 0x0;
                c.lr = 0x80118ab4; f_800ec160(c);
                pc = 0x80118ab4;
            }
            0x80118ab4 => {
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x48), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x4c), c.r[0]);
                c.stb(c.r[31].wrapping_add(0x6), c.r[0]);
                c.stb(c.r[31].wrapping_add(0x5), c.r[0]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.lr = 0x80118ad0; f_80169484(c);
                pc = 0x80118ad0;
            }
            0x80118ad0 => {
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x44), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80118b18; continue; }
                pc = 0x80118ae4;
            }
            0x80118ae4 => {
                c.r[3] = c.r[31].wrapping_add(0x14);
                c.r[4] = c.r[31].wrapping_add(0x50);
                c.lr = 0x80118af0; f_800e01d0(c);
                pc = 0x80118af0;
            }
            0x80118af0 => {
                { let ea = c.r[31].wrapping_add(0x50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x54); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118b24; continue;
                pc = 0x80118b18;
            }
            0x80118b18 => {
                c.r[3] = c.r[31].wrapping_add(0x50);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80118b24; f_800e01d0(c);
                pc = 0x80118b24;
            }
            0x80118b24 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80118b64; continue; }
                pc = 0x80118b30;
            }
            0x80118b30 => {
                c.r[3] = c.r[31].wrapping_add(0x20);
                c.r[4] = c.r[31].wrapping_add(0x5c);
                c.lr = 0x80118b3c; f_800e01d0(c);
                pc = 0x80118b3c;
            }
            0x80118b3c => {
                { let ea = c.r[31].wrapping_add(0x5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x60); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x64); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118b70; continue;
                pc = 0x80118b64;
            }
            0x80118b64 => {
                c.r[3] = c.r[31].wrapping_add(0x5c);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80118b70; f_800e01d0(c);
                pc = 0x80118b70;
            }
            0x80118b70 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80118bb0; continue; }
                pc = 0x80118b7c;
            }
            0x80118b7c => {
                c.r[3] = c.r[31].wrapping_add(0x2c);
                c.r[4] = c.r[31].wrapping_add(0x68);
                c.lr = 0x80118b88; f_800e01d0(c);
                pc = 0x80118b88;
            }
            0x80118b88 => {
                { let ea = c.r[31].wrapping_add(0x68); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x6c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x70); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118bbc; continue;
                pc = 0x80118bb0;
            }
            0x80118bb0 => {
                c.r[3] = c.r[31].wrapping_add(0x68);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80118bbc; f_800e01d0(c);
                pc = 0x80118bbc;
            }
            0x80118bbc => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.lr = 0x80118bc4; f_80175a1c(c);
                pc = 0x80118bc4;
            }
            0x80118bc4 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = 0x40;
                c.r[3] = 0x0;
                c.r[4] = c.r[5] | c.r[5];
                c.mtspr(9, c.r[0]);
                pc = 0x80118bd8;
            }
            0x80118bd8 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.cmpu(0, c.r[0], c.r[31]);
                if !c.crbit(2) { pc = 0x80118bf8; continue; }
                pc = 0x80118be4;
            }
            0x80118be4 => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[4] = 0x0;
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                pc = 0x80118c04; continue;
                pc = 0x80118bf8;
            }
            0x80118bf8 => {
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80118bd8; continue; }
                pc = 0x80118c04;
            }
            0x80118c04 => {
                c.r[0] = 0x0;
                c.stb(c.r[31], c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118a68: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118C20 (0x80118c20, 104 bytes)
#[inline(never)]
pub fn f_80118c20(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118c20) { return; }
        if c.prof { crate::prof::enter(c, 0x80118c20); f_80118c20_b(c, 0x80118c20); crate::prof::exit(c, 0x80118c20); return; }
    }
    f_80118c20_b(c, 0x80118c20)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118c20_at(c: &mut Cpu, pc: u32) { f_80118c20_b(c, pc) }
#[inline(always)]
fn f_80118c20_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118c20;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118c20, pc); }
        match pc {
            0x80118c20 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                pc = 0x80118c44;
            }
            0x80118c44 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80118c5c; continue; }
                pc = 0x80118c50;
            }
            0x80118c50 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80118c5c; f_80169034(c);
                pc = 0x80118c5c;
            }
            0x80118c5c => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x4);
                c.cmpu(0, c.r[30], 0x40);
                if c.crbit(0) { pc = 0x80118c44; continue; }
                pc = 0x80118c6c;
            }
            0x80118c6c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80118c20: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118C88 (0x80118c88, 36 bytes)
#[inline(never)]
pub fn f_80118c88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118c88) { return; }
        if c.prof { crate::prof::enter(c, 0x80118c88); f_80118c88_b(c, 0x80118c88); crate::prof::exit(c, 0x80118c88); return; }
    }
    f_80118c88_b(c, 0x80118c88)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118c88_at(c: &mut Cpu, pc: u32) { f_80118c88_b(c, pc) }
#[inline(always)]
fn f_80118c88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118c88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118c88, pc); }
        match pc {
            0x80118c88 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x80118c9c; f_80169034(c);
                pc = 0x80118c9c;
            }
            0x80118c9c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118c88: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118CAC (0x80118cac, 36 bytes)
#[inline(never)]
pub fn f_80118cac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118cac) { return; }
        if c.prof { crate::prof::enter(c, 0x80118cac); f_80118cac_b(c, 0x80118cac); crate::prof::exit(c, 0x80118cac); return; }
    }
    f_80118cac_b(c, 0x80118cac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118cac_at(c: &mut Cpu, pc: u32) { f_80118cac_b(c, pc) }
#[inline(always)]
fn f_80118cac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118cac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118cac, pc); }
        match pc {
            0x80118cac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x80118cc0; f_80169340(c);
                pc = 0x80118cc0;
            }
            0x80118cc0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118cac: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118CD0 (0x80118cd0, 36 bytes)
#[inline(never)]
pub fn f_80118cd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118cd0) { return; }
        if c.prof { crate::prof::enter(c, 0x80118cd0); f_80118cd0_b(c, 0x80118cd0); crate::prof::exit(c, 0x80118cd0); return; }
    }
    f_80118cd0_b(c, 0x80118cd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118cd0_at(c: &mut Cpu, pc: u32) { f_80118cd0_b(c, pc) }
#[inline(always)]
fn f_80118cd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118cd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118cd0, pc); }
        match pc {
            0x80118cd0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x80118ce4; f_80169368(c);
                pc = 0x80118ce4;
            }
            0x80118ce4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118cd0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118CF4 (0x80118cf4, 36 bytes)
#[inline(never)]
pub fn f_80118cf4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118cf4) { return; }
        if c.prof { crate::prof::enter(c, 0x80118cf4); f_80118cf4_b(c, 0x80118cf4); crate::prof::exit(c, 0x80118cf4); return; }
    }
    f_80118cf4_b(c, 0x80118cf4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118cf4_at(c: &mut Cpu, pc: u32) { f_80118cf4_b(c, pc) }
#[inline(always)]
fn f_80118cf4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118cf4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118cf4, pc); }
        match pc {
            0x80118cf4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x80118d08; f_80169390(c);
                pc = 0x80118d08;
            }
            0x80118d08 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118cf4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118D18 (0x80118d18, 36 bytes)
#[inline(never)]
pub fn f_80118d18(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118d18) { return; }
        if c.prof { crate::prof::enter(c, 0x80118d18); f_80118d18_b(c, 0x80118d18); crate::prof::exit(c, 0x80118d18); return; }
    }
    f_80118d18_b(c, 0x80118d18)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118d18_at(c: &mut Cpu, pc: u32) { f_80118d18_b(c, pc) }
#[inline(always)]
fn f_80118d18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118d18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118d18, pc); }
        match pc {
            0x80118d18 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x80118d2c; f_801693dc(c);
                pc = 0x80118d2c;
            }
            0x80118d2c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118d18: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118D3C (0x80118d3c, 36 bytes)
#[inline(never)]
pub fn f_80118d3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118d3c) { return; }
        if c.prof { crate::prof::enter(c, 0x80118d3c); f_80118d3c_b(c, 0x80118d3c); crate::prof::exit(c, 0x80118d3c); return; }
    }
    f_80118d3c_b(c, 0x80118d3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118d3c_at(c: &mut Cpu, pc: u32) { f_80118d3c_b(c, pc) }
#[inline(always)]
fn f_80118d3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118d3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118d3c, pc); }
        match pc {
            0x80118d3c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x80118d50; f_80169404(c);
                pc = 0x80118d50;
            }
            0x80118d50 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118d3c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118D60 (0x80118d60, 36 bytes)
#[inline(never)]
pub fn f_80118d60(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118d60) { return; }
        if c.prof { crate::prof::enter(c, 0x80118d60); f_80118d60_b(c, 0x80118d60); crate::prof::exit(c, 0x80118d60); return; }
    }
    f_80118d60_b(c, 0x80118d60)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118d60_at(c: &mut Cpu, pc: u32) { f_80118d60_b(c, pc) }
#[inline(always)]
fn f_80118d60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118d60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118d60, pc); }
        match pc {
            0x80118d60 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x80118d74; f_8016945c(c);
                pc = 0x80118d74;
            }
            0x80118d74 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118d60: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118D84 (0x80118d84, 36 bytes)
#[inline(never)]
pub fn f_80118d84(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118d84) { return; }
        if c.prof { crate::prof::enter(c, 0x80118d84); f_80118d84_b(c, 0x80118d84); crate::prof::exit(c, 0x80118d84); return; }
    }
    f_80118d84_b(c, 0x80118d84)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118d84_at(c: &mut Cpu, pc: u32) { f_80118d84_b(c, pc) }
#[inline(always)]
fn f_80118d84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118d84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118d84, pc); }
        match pc {
            0x80118d84 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x80118d98; f_801694a8(c);
                pc = 0x80118d98;
            }
            0x80118d98 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118d84: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118DA8 (0x80118da8, 56 bytes)
#[inline(never)]
pub fn f_80118da8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118da8) { return; }
        if c.prof { crate::prof::enter(c, 0x80118da8); f_80118da8_b(c, 0x80118da8); crate::prof::exit(c, 0x80118da8); return; }
    }
    f_80118da8_b(c, 0x80118da8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118da8_at(c: &mut Cpu, pc: u32) { f_80118da8_b(c, pc) }
#[inline(always)]
fn f_80118da8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118da8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118da8, pc); }
        match pc {
            0x80118da8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80118dc8; continue; }
                pc = 0x80118dc0;
            }
            0x80118dc0 => {
                c.r[3] = 0xffffffff;
                pc = 0x80118dd0; continue;
                pc = 0x80118dc8;
            }
            0x80118dc8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                c.lr = 0x80118dd0; f_801694e0(c);
                pc = 0x80118dd0;
            }
            0x80118dd0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118da8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118DE0 (0x80118de0, 172 bytes)
#[inline(never)]
pub fn f_80118de0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118de0) { return; }
        if c.prof { crate::prof::enter(c, 0x80118de0); f_80118de0_b(c, 0x80118de0); crate::prof::exit(c, 0x80118de0); return; }
    }
    f_80118de0_b(c, 0x80118de0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118de0_at(c: &mut Cpu, pc: u32) { f_80118de0_b(c, pc) }
#[inline(always)]
fn f_80118de0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118de0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118de0, pc); }
        match pc {
            0x80118de0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80118e48; continue; }
                pc = 0x80118e18;
            }
            0x80118e18 => {
                c.r[3] = c.r[28].wrapping_add(0x2c);
                c.lr = 0x80118e20; f_800e01d0(c);
                pc = 0x80118e20;
            }
            0x80118e20 => {
                { let ea = c.r[29]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118e50; continue;
                pc = 0x80118e48;
            }
            0x80118e48 => {
                c.r[3] = c.r[28].wrapping_add(0x68);
                c.lr = 0x80118e50; f_800e01d0(c);
                pc = 0x80118e50;
            }
            0x80118e50 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80118e6c; continue; }
                pc = 0x80118e5c;
            }
            0x80118e5c => {
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x10));
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = c.r[31] | c.r[31];
                c.lr = 0x80118e6c; f_80169104(c);
                pc = 0x80118e6c;
            }
            0x80118e6c => {
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
            _ => panic!("f_80118de0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118E8C (0x80118e8c, 120 bytes)
#[inline(never)]
pub fn f_80118e8c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118e8c) { return; }
        if c.prof { crate::prof::enter(c, 0x80118e8c); f_80118e8c_b(c, 0x80118e8c); crate::prof::exit(c, 0x80118e8c); return; }
    }
    f_80118e8c_b(c, 0x80118e8c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118e8c_at(c: &mut Cpu, pc: u32) { f_80118e8c_b(c, pc) }
#[inline(always)]
fn f_80118e8c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118e8c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118e8c, pc); }
        match pc {
            0x80118e8c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80118ee4; continue; }
                pc = 0x80118eb4;
            }
            0x80118eb4 => {
                c.r[3] = c.r[30].wrapping_add(0x20);
                c.lr = 0x80118ebc; f_800e01d0(c);
                pc = 0x80118ebc;
            }
            0x80118ebc => {
                { let ea = c.r[31]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118eec; continue;
                pc = 0x80118ee4;
            }
            0x80118ee4 => {
                c.r[3] = c.r[30].wrapping_add(0x5c);
                c.lr = 0x80118eec; f_800e01d0(c);
                pc = 0x80118eec;
            }
            0x80118eec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118e8c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118F04 (0x80118f04, 120 bytes)
#[inline(never)]
pub fn f_80118f04(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118f04) { return; }
        if c.prof { crate::prof::enter(c, 0x80118f04); f_80118f04_b(c, 0x80118f04); crate::prof::exit(c, 0x80118f04); return; }
    }
    f_80118f04_b(c, 0x80118f04)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118f04_at(c: &mut Cpu, pc: u32) { f_80118f04_b(c, pc) }
#[inline(always)]
fn f_80118f04_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118f04;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118f04, pc); }
        match pc {
            0x80118f04 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80118f5c; continue; }
                pc = 0x80118f2c;
            }
            0x80118f2c => {
                c.r[3] = c.r[30].wrapping_add(0x14);
                c.lr = 0x80118f34; f_800e01d0(c);
                pc = 0x80118f34;
            }
            0x80118f34 => {
                { let ea = c.r[31]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80118f64; continue;
                pc = 0x80118f5c;
            }
            0x80118f5c => {
                c.r[3] = c.r[30].wrapping_add(0x50);
                c.lr = 0x80118f64; f_800e01d0(c);
                pc = 0x80118f64;
            }
            0x80118f64 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118f04: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118F7C (0x80118f7c, 52 bytes)
#[inline(never)]
pub fn f_80118f7c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118f7c) { return; }
        if c.prof { crate::prof::enter(c, 0x80118f7c); f_80118f7c_b(c, 0x80118f7c); crate::prof::exit(c, 0x80118f7c); return; }
    }
    f_80118f7c_b(c, 0x80118f7c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118f7c_at(c: &mut Cpu, pc: u32) { f_80118f7c_b(c, pc) }
#[inline(always)]
fn f_80118f7c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118f7c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118f7c, pc); }
        match pc {
            0x80118f7c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let ea = c.r[3].wrapping_add(0x38); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[4] | c.r[4];
                { let ea = c.r[5].wrapping_add(0x3c); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[5].wrapping_add(0x40); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x80118fa0; f_800e01f4(c);
                pc = 0x80118fa0;
            }
            0x80118fa0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80118f7c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80118FB0 (0x80118fb0, 300 bytes)
#[inline(never)]
pub fn f_80118fb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80118fb0) { return; }
        if c.prof { crate::prof::enter(c, 0x80118fb0); f_80118fb0_b(c, 0x80118fb0); crate::prof::exit(c, 0x80118fb0); return; }
    }
    f_80118fb0_b(c, 0x80118fb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80118fb0_at(c: &mut Cpu, pc: u32) { f_80118fb0_b(c, pc) }
#[inline(always)]
fn f_80118fb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80118fb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80118fb0, pc); }
        match pc {
            0x80118fb0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = c.r[8] | c.r[8];
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801190c8; continue; }
                pc = 0x80118fe0;
            }
            0x80118fe0 => {
                c.cmps(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x801190c8; continue; }
                pc = 0x80118fe8;
            }
            0x80118fe8 => {
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x4));
                c.r[0] = c.r[7].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                c.stw(c.r[27].wrapping_add(0x48), c.r[3]);
                c.r[0] = c.lhz(c.r[28].wrapping_add(0x2));
                c.stw(c.r[27].wrapping_add(0x4c), c.r[0]);
                if !c.crbit(2) { pc = 0x80119044; continue; }
                pc = 0x80119004;
            }
            0x80119004 => {
                { let ea = c.r[2].wrapping_add(0xffff9948); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[27].wrapping_add(0x50);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80119018; f_800e01f4(c);
                pc = 0x80119018;
            }
            0x80119018 => {
                { let ea = c.r[2].wrapping_add(0xffff9948); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[27].wrapping_add(0x5c);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x8011902c; f_800e01f4(c);
                pc = 0x8011902c;
            }
            0x8011902c => {
                { let ea = c.r[2].wrapping_add(0xffff994c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[27].wrapping_add(0x68);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80119040; f_800e01f4(c);
                pc = 0x80119040;
            }
            0x80119040 => {
                pc = 0x80119068; continue;
                pc = 0x80119044;
            }
            0x80119044 => {
                c.r[3] = c.r[27].wrapping_add(0x50);
                c.r[4] = c.r[27].wrapping_add(0x14);
                c.lr = 0x80119050; f_800e01d0(c);
                pc = 0x80119050;
            }
            0x80119050 => {
                c.r[3] = c.r[27].wrapping_add(0x5c);
                c.r[4] = c.r[27].wrapping_add(0x20);
                c.lr = 0x8011905c; f_800e01d0(c);
                pc = 0x8011905c;
            }
            0x8011905c => {
                c.r[3] = c.r[27].wrapping_add(0x68);
                c.r[4] = c.r[27].wrapping_add(0x2c);
                c.lr = 0x80119068; f_800e01d0(c);
                pc = 0x80119068;
            }
            0x80119068 => {
                { let ea = c.r[2].wrapping_add(0xffff9948); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[27].wrapping_add(0x14);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x8011907c; f_800e01f4(c);
                pc = 0x8011907c;
            }
            0x8011907c => {
                { let ea = c.r[2].wrapping_add(0xffff9948); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[27].wrapping_add(0x20);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80119090; f_800e01f4(c);
                pc = 0x80119090;
            }
            0x80119090 => {
                { let ea = c.r[2].wrapping_add(0xffff994c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[27].wrapping_add(0x2c);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x801190a4; f_800e01f4(c);
                pc = 0x801190a4;
            }
            0x801190a4 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x801190b8; continue; }
                pc = 0x801190ac;
            }
            0x801190ac => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x8));
                c.lr = 0x801190b8; f_80169494(c);
                pc = 0x801190b8;
            }
            0x801190b8 => {
                c.stw(c.r[27].wrapping_add(0x44), c.r[29]);
                c.r[0] = 0x1;
                c.stb(c.r[27].wrapping_add(0x5), c.r[30]);
                c.stb(c.r[27].wrapping_add(0x6), c.r[0]);
                pc = 0x801190c8;
            }
            0x801190c8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80118fb0: bad pc {:#x}", pc),
        }
    }
}

/// gsParticleCreate (0x801190dc, 736 bytes)
#[inline(never)]
pub fn f_801190dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801190dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801190dc); f_801190dc_b(c, 0x801190dc); crate::prof::exit(c, 0x801190dc); return; }
    }
    f_801190dc_b(c, 0x801190dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801190dc_at(c: &mut Cpu, pc: u32) { f_801190dc_b(c, pc) }
#[inline(always)]
fn f_801190dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801190dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801190dc, pc); }
        match pc {
            0x801190dc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[0] = c.lbz(c.r[3]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8011910c; continue; }
                pc = 0x80119104;
            }
            0x80119104 => {
                c.r[3] = 0x0;
                pc = 0x801193a8; continue;
                pc = 0x8011910c;
            }
            0x8011910c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa58c));
                c.r[31] = c.lwz(c.r[13].wrapping_add(0xffffa588));
                c.mtspr(9, c.r[0]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(1) { pc = 0x80119138; continue; }
                pc = 0x80119120;
            }
            0x80119120 => {
                c.r[0] = c.lbz(c.r[31]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119130; continue; }
                pc = 0x8011912c;
            }
            0x8011912c => {
                pc = 0x8011913c; continue;
                pc = 0x80119130;
            }
            0x80119130 => {
                c.r[31] = c.r[31].wrapping_add(0x74);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80119120; continue; }
                pc = 0x80119138;
            }
            0x80119138 => {
                c.r[31] = 0x0;
                pc = 0x8011913c;
            }
            0x8011913c => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8011914c; continue; }
                pc = 0x80119144;
            }
            0x80119144 => {
                c.r[3] = 0x0;
                pc = 0x801193a8; continue;
                pc = 0x8011914c;
            }
            0x8011914c => {
                c.r[0] = 0x8;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x8011915c;
            }
            0x8011915c => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8011916c; continue; }
                pc = 0x80119168;
            }
            0x80119168 => {
                pc = 0x80119208; continue;
                pc = 0x8011916c;
            }
            0x8011916c => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119180; continue; }
                pc = 0x8011917c;
            }
            0x8011917c => {
                pc = 0x80119208; continue;
                pc = 0x80119180;
            }
            0x80119180 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x10));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119194; continue; }
                pc = 0x80119190;
            }
            0x80119190 => {
                pc = 0x80119208; continue;
                pc = 0x80119194;
            }
            0x80119194 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x14));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801191a8; continue; }
                pc = 0x801191a4;
            }
            0x801191a4 => {
                pc = 0x80119208; continue;
                pc = 0x801191a8;
            }
            0x801191a8 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x18));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801191bc; continue; }
                pc = 0x801191b8;
            }
            0x801191b8 => {
                pc = 0x80119208; continue;
                pc = 0x801191bc;
            }
            0x801191bc => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x1c));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801191d0; continue; }
                pc = 0x801191cc;
            }
            0x801191cc => {
                pc = 0x80119208; continue;
                pc = 0x801191d0;
            }
            0x801191d0 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x20));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801191e4; continue; }
                pc = 0x801191e0;
            }
            0x801191e0 => {
                pc = 0x80119208; continue;
                pc = 0x801191e4;
            }
            0x801191e4 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801191f8; continue; }
                pc = 0x801191f4;
            }
            0x801191f4 => {
                pc = 0x80119208; continue;
                pc = 0x801191f8;
            }
            0x801191f8 => {
                c.r[4] = c.r[4].wrapping_add(0x20);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8011915c; continue; }
                pc = 0x80119204;
            }
            0x80119204 => {
                c.r[3] = 0xffffffff;
                pc = 0x80119208;
            }
            0x80119208 => {
                c.cmps(0, c.r[3], 0xffffffff);
                c.r[27] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x801192ec; continue; }
                pc = 0x80119214;
            }
            0x80119214 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011921c; f_80119930(c);
                pc = 0x8011921c;
            }
            0x8011921c => {
                c.r[0] = 0x8;
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x8011922c;
            }
            0x8011922c => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8011923c; continue; }
                pc = 0x80119238;
            }
            0x80119238 => {
                pc = 0x801192d8; continue;
                pc = 0x8011923c;
            }
            0x8011923c => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xc));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119250; continue; }
                pc = 0x8011924c;
            }
            0x8011924c => {
                pc = 0x801192d8; continue;
                pc = 0x80119250;
            }
            0x80119250 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x10));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119264; continue; }
                pc = 0x80119260;
            }
            0x80119260 => {
                pc = 0x801192d8; continue;
                pc = 0x80119264;
            }
            0x80119264 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x14));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119278; continue; }
                pc = 0x80119274;
            }
            0x80119274 => {
                pc = 0x801192d8; continue;
                pc = 0x80119278;
            }
            0x80119278 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x18));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8011928c; continue; }
                pc = 0x80119288;
            }
            0x80119288 => {
                pc = 0x801192d8; continue;
                pc = 0x8011928c;
            }
            0x8011928c => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x1c));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801192a0; continue; }
                pc = 0x8011929c;
            }
            0x8011929c => {
                pc = 0x801192d8; continue;
                pc = 0x801192a0;
            }
            0x801192a0 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x20));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801192b4; continue; }
                pc = 0x801192b0;
            }
            0x801192b0 => {
                pc = 0x801192d8; continue;
                pc = 0x801192b4;
            }
            0x801192b4 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x24));
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801192c8; continue; }
                pc = 0x801192c4;
            }
            0x801192c4 => {
                pc = 0x801192d8; continue;
                pc = 0x801192c8;
            }
            0x801192c8 => {
                c.r[4] = c.r[4].wrapping_add(0x20);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8011922c; continue; }
                pc = 0x801192d4;
            }
            0x801192d4 => {
                c.r[3] = 0xffffffff;
                pc = 0x801192d8;
            }
            0x801192d8 => {
                c.cmps(0, c.r[3], 0xffffffff);
                c.r[27] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x801192ec; continue; }
                pc = 0x801192e4;
            }
            0x801192e4 => {
                c.r[3] = 0x0;
                pc = 0x801193a8; continue;
                pc = 0x801192ec;
            }
            0x801192ec => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.r[3] = 0x0;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80119300; continue; }
                pc = 0x801192fc;
            }
            0x801192fc => {
                c.r[3] = 0x1;
                pc = 0x80119300;
            }
            0x80119300 => {
                c.r[4] = c.lbz(c.r[28].wrapping_add(0x1));
                c.r[5] = c.r[29] | c.r[29];
                c.lr = 0x8011930c; f_80173718(c);
                pc = 0x8011930c;
            }
            0x8011930c => {
                c.stw(c.r[31].wrapping_add(0x10), c.r[3]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80119324; continue; }
                pc = 0x8011931c;
            }
            0x8011931c => {
                c.r[3] = 0x0;
                pc = 0x801193a8; continue;
                pc = 0x80119324;
            }
            0x80119324 => {
                c.r[4] = 0x1;
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x17), c.r[4]);
                c.r[3] = c.r[31].wrapping_add(0x38);
                c.stb(c.r[31], c.r[4]);
                c.stw(c.r[31].wrapping_add(0xc), c.r[28]);
                c.sth(c.r[31].wrapping_add(0x2), c.r[29]);
                c.stw(c.r[31].wrapping_add(0x44), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x48), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x4c), c.r[0]);
                c.stb(c.r[31].wrapping_add(0x5), c.r[0]);
                c.stb(c.r[31].wrapping_add(0x6), c.r[0]);
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[4].wrapping_add(0x20); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[4].wrapping_add(0x24); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[4].wrapping_add(0x28); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x80119368; f_800e01f4(c);
                pc = 0x80119368;
            }
            0x80119368 => {
                c.stb(c.r[31].wrapping_add(0x4), c.r[30]);
                c.lr = 0x80119370; f_800d3094(c);
                pc = 0x80119370;
            }
            0x80119370 => {
                c.r[0] = c.r[27].rotate_left(2) & 0xfffffffc;
                c.stw(c.r[31].wrapping_add(0x8), c.r[3]);
                c.r[3] = c.r[28].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[31]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x10));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8011939c; continue; }
                pc = 0x80119390;
            }
            0x80119390 => {
                c.r[0] = 0x1;
                c.stb(c.r[31].wrapping_add(0x1), c.r[0]);
                pc = 0x801193a4; continue;
                pc = 0x8011939c;
            }
            0x8011939c => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x1), c.r[0]);
                pc = 0x801193a4;
            }
            0x801193a4 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x801193a8;
            }
            0x801193a8 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801190dc: bad pc {:#x}", pc),
        }
    }
}

/// fn_801193BC (0x801193bc, 496 bytes)
#[inline(never)]
pub fn f_801193bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801193bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801193bc); f_801193bc_b(c, 0x801193bc); crate::prof::exit(c, 0x801193bc); return; }
    }
    f_801193bc_b(c, 0x801193bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801193bc_at(c: &mut Cpu, pc: u32) { f_801193bc_b(c, pc) }
#[inline(always)]
fn f_801193bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801193bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801193bc, pc); }
        match pc {
            0x801193bc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = 0x0;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = c.r[28] | c.r[28];
                pc = 0x801193e4;
            }
            0x801193e4 => {
                c.r[29] = c.lwz(c.r[30].wrapping_add(0x8));
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x80119574; continue; }
                pc = 0x801193f0;
            }
            0x801193f0 => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x10));
                c.r[3] = c.lhz(c.r[4].wrapping_add(0x18));
                c.r[4] = c.lbz(c.r[4].wrapping_add(0x15));
                c.lr = 0x80119400; f_801695fc(c);
                pc = 0x80119400;
            }
            0x80119400 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80119524; continue; }
                pc = 0x8011940c;
            }
            0x8011940c => {
                if c.crbit(2) { pc = 0x80119524; continue; }
                pc = 0x80119410;
            }
            0x80119410 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x48));
                c.r[4] = 0x0;
                c.lr = 0x8011941c; f_800ec160(c);
                pc = 0x8011941c;
            }
            0x8011941c => {
                c.r[0] = 0x0;
                c.stw(c.r[29].wrapping_add(0x48), c.r[0]);
                c.stw(c.r[29].wrapping_add(0x4c), c.r[0]);
                c.stb(c.r[29].wrapping_add(0x6), c.r[0]);
                c.stb(c.r[29].wrapping_add(0x5), c.r[0]);
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                c.lr = 0x80119438; f_80169484(c);
                pc = 0x80119438;
            }
            0x80119438 => {
                c.r[0] = 0x0;
                c.stw(c.r[29].wrapping_add(0x44), c.r[0]);
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119480; continue; }
                pc = 0x8011944c;
            }
            0x8011944c => {
                c.r[3] = c.r[29].wrapping_add(0x14);
                c.r[4] = c.r[29].wrapping_add(0x50);
                c.lr = 0x80119458; f_800e01d0(c);
                pc = 0x80119458;
            }
            0x80119458 => {
                { let ea = c.r[29].wrapping_add(0x50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x54); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8011948c; continue;
                pc = 0x80119480;
            }
            0x80119480 => {
                c.r[3] = c.r[29].wrapping_add(0x50);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x8011948c; f_800e01d0(c);
                pc = 0x8011948c;
            }
            0x8011948c => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801194cc; continue; }
                pc = 0x80119498;
            }
            0x80119498 => {
                c.r[3] = c.r[29].wrapping_add(0x20);
                c.r[4] = c.r[29].wrapping_add(0x5c);
                c.lr = 0x801194a4; f_800e01d0(c);
                pc = 0x801194a4;
            }
            0x801194a4 => {
                { let ea = c.r[29].wrapping_add(0x5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x60); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x64); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801194d8; continue;
                pc = 0x801194cc;
            }
            0x801194cc => {
                c.r[3] = c.r[29].wrapping_add(0x5c);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x801194d8; f_800e01d0(c);
                pc = 0x801194d8;
            }
            0x801194d8 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119518; continue; }
                pc = 0x801194e4;
            }
            0x801194e4 => {
                c.r[3] = c.r[29].wrapping_add(0x2c);
                c.r[4] = c.r[29].wrapping_add(0x68);
                c.lr = 0x801194f0; f_800e01d0(c);
                pc = 0x801194f0;
            }
            0x801194f0 => {
                { let ea = c.r[29].wrapping_add(0x68); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x6c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[29].wrapping_add(0x70); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80119524; continue;
                pc = 0x80119518;
            }
            0x80119518 => {
                c.r[3] = c.r[29].wrapping_add(0x68);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80119524; f_800e01d0(c);
                pc = 0x80119524;
            }
            0x80119524 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                c.lr = 0x8011952c; f_80175a1c(c);
                pc = 0x8011952c;
            }
            0x8011952c => {
                c.r[5] = c.lwz(c.r[29].wrapping_add(0xc));
                c.r[0] = 0x40;
                c.r[3] = 0x0;
                c.r[4] = c.r[5] | c.r[5];
                c.mtspr(9, c.r[0]);
                pc = 0x80119540;
            }
            0x80119540 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.cmpu(0, c.r[0], c.r[29]);
                if !c.crbit(2) { pc = 0x80119560; continue; }
                pc = 0x8011954c;
            }
            0x8011954c => {
                c.r[3] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[4] = 0x0;
                c.r[0] = c.r[3].wrapping_add(0x8);
                c.stw(c.r[5].wrapping_add(c.r[0]), c.r[4]);
                pc = 0x8011956c; continue;
                pc = 0x80119560;
            }
            0x80119560 => {
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80119540; continue; }
                pc = 0x8011956c;
            }
            0x8011956c => {
                c.r[0] = 0x0;
                c.stb(c.r[29], c.r[0]);
                pc = 0x80119574;
            }
            0x80119574 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.r[30] = c.r[30].wrapping_add(0x4);
                c.cmpu(0, c.r[31], 0x40);
                if c.crbit(0) { pc = 0x801193e4; continue; }
                pc = 0x80119584;
            }
            0x80119584 => {
                c.r[0] = 0x0;
                c.stb(c.r[28], c.r[0]);
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
            _ => panic!("f_801193bc: bad pc {:#x}", pc),
        }
    }
}

/// gsParticleBankLoad (0x801195ac, 632 bytes)
#[inline(never)]
pub fn f_801195ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801195ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801195ac); f_801195ac_b(c, 0x801195ac); crate::prof::exit(c, 0x801195ac); return; }
    }
    f_801195ac_b(c, 0x801195ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801195ac_at(c: &mut Cpu, pc: u32) { f_801195ac_b(c, pc) }
#[inline(always)]
fn f_801195ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801195ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801195ac, pc); }
        match pc {
            0x801195ac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x47500000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[0] = c.r[3].wrapping_add(0x5431);
                c.r[30] = c.r[5].wrapping_add(0x27d8);
                c.r[4] = c.lwz(c.r[31]);
                c.cmps(0, c.r[4], c.r[0]);
                if c.crbit(2) { pc = 0x80119634; continue; }
                pc = 0x801195e0;
            }
            0x801195e0 => {
                if !c.crbit(0) { pc = 0x80119620; continue; }
                pc = 0x801195e4;
            }
            0x801195e4 => {
                c.r[0] = c.r[3].wrapping_add(0x5430);
                c.cmps(0, c.r[4], c.r[0]);
                if !c.crbit(0) { pc = 0x801195f4; continue; }
                pc = 0x801195f0;
            }
            0x801195f0 => {
                pc = 0x80119620; continue;
                pc = 0x801195f4;
            }
            0x801195f4 => {
                c.r[3] = c.r[30].wrapping_add(0x50);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80119600; stubs_gen::f_800dd970(c);
                pc = 0x80119600;
            }
            0x80119600 => {
                c.r[3] = c.r[30].wrapping_add(0xac);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8011960c; stubs_gen::f_800dd970(c);
                pc = 0x8011960c;
            }
            0x8011960c => {
                c.r[3] = c.r[30].wrapping_add(0x50);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80119618; stubs_gen::f_800dd970(c);
                pc = 0x80119618;
            }
            0x80119618 => {
                c.r[3] = 0x0;
                pc = 0x8011980c; continue;
                pc = 0x80119620;
            }
            0x80119620 => {
                c.r[3] = c.r[30].wrapping_add(0x108);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8011962c; stubs_gen::f_800dd970(c);
                pc = 0x8011962c;
            }
            0x8011962c => {
                c.r[3] = 0x0;
                pc = 0x8011980c; continue;
                pc = 0x80119634;
            }
            0x80119634 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[31].wrapping_add(0x4), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[31].wrapping_add(0x8), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.stw(c.r[31].wrapping_add(0x10), c.r[0]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[5] = c.lhz(c.r[3]);
                c.cmps(0, c.r[5], 0x43);
                if c.crbit(2) { pc = 0x80119680; continue; }
                pc = 0x80119668;
            }
            0x80119668 => {
                c.r[3] = c.r[30].wrapping_add(0x128);
                c.r[4] = 0x43;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80119678; stubs_gen::f_800dd970(c);
                pc = 0x80119678;
            }
            0x80119678 => {
                c.r[3] = 0x0;
                pc = 0x8011980c; continue;
                pc = 0x80119680;
            }
            0x80119680 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[0] = c.r[4].wrapping_add(0x1f);
                c.r[4] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.lr = 0x80119694; f_8009b2d0(c);
                pc = 0x80119694;
            }
            0x80119694 => {
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffffa57c));
                c.r[7] = c.lwz(c.r[13].wrapping_add(0xffffa580));
                c.r[30] = c.r[6] | c.r[6];
                c.mtspr(9, c.r[7]);
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(1) { pc = 0x801196c4; continue; }
                pc = 0x801196ac;
            }
            0x801196ac => {
                c.r[0] = c.lbz(c.r[30]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801196bc; continue; }
                pc = 0x801196b8;
            }
            0x801196b8 => {
                pc = 0x801196c8; continue;
                pc = 0x801196bc;
            }
            0x801196bc => {
                c.r[30] = c.r[30].wrapping_add(0x108);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x801196ac; continue; }
                pc = 0x801196c4;
            }
            0x801196c4 => {
                c.r[30] = 0x0;
                pc = 0x801196c8;
            }
            0x801196c8 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x801196d8; continue; }
                pc = 0x801196d0;
            }
            0x801196d0 => {
                c.r[3] = 0x0;
                pc = 0x8011980c; continue;
                pc = 0x801196d8;
            }
            0x801196d8 => {
                c.r[4] = 0x0;
                pc = 0x80119724; continue;
                pc = 0x801196e0;
            }
            0x801196e0 => {
                c.r[5] = c.r[6] | c.r[6];
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.mtspr(9, c.r[7]);
                c.cmpu(0, c.r[7], 0x0);
                if !c.crbit(1) { pc = 0x80119710; continue; }
                pc = 0x801196f4;
            }
            0x801196f4 => {
                c.r[3] = c.lbz(c.r[5].wrapping_add(0x1));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(2) { pc = 0x80119708; continue; }
                pc = 0x80119700;
            }
            0x80119700 => {
                c.r[0] = 0x1;
                pc = 0x80119714; continue;
                pc = 0x80119708;
            }
            0x80119708 => {
                c.r[5] = c.r[5].wrapping_add(0x108);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x801196f4; continue; }
                pc = 0x80119710;
            }
            0x80119710 => {
                c.r[0] = 0x0;
                pc = 0x80119714;
            }
            0x80119714 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80119720; continue; }
                pc = 0x8011971c;
            }
            0x8011971c => {
                pc = 0x80119734; continue;
                pc = 0x80119720;
            }
            0x80119720 => {
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x80119724;
            }
            0x80119724 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x40);
                if c.crbit(0) { pc = 0x801196e0; continue; }
                pc = 0x80119730;
            }
            0x80119730 => {
                c.r[4] = 0xff;
                pc = 0x80119734;
            }
            0x80119734 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0xff);
                if !c.crbit(2) { pc = 0x80119748; continue; }
                pc = 0x80119740;
            }
            0x80119740 => {
                c.r[3] = 0x0;
                pc = 0x8011980c; continue;
                pc = 0x80119748;
            }
            0x80119748 => {
                c.r[3] = 0x1;
                c.r[0] = 0x2;
                c.stb(c.r[30], c.r[3]);
                c.r[5] = c.r[30] | c.r[30];
                c.r[3] = 0x0;
                c.stb(c.r[30].wrapping_add(0x1), c.r[4]);
                c.stw(c.r[30].wrapping_add(0x4), c.r[31]);
                c.mtspr(9, c.r[0]);
                pc = 0x80119768;
            }
            0x80119768 => {
                c.stw(c.r[5].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[5].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x10), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x1c), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x20), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x24), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x28), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x2c), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x30), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x34), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x38), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x3c), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x40), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x44), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x48), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x4c), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x50), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x54), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x58), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x5c), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x60), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x64), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x68), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x6c), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x70), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x74), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x78), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x7c), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x80), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x84), c.r[3]);
                c.r[5] = c.r[5].wrapping_add(0x80);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80119768; continue; }
                pc = 0x801197f0;
            }
            0x801197f0 => {
                c.r[3] = c.lbz(c.r[30].wrapping_add(0x1));
                c.r[7] = 0x0;
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[6] = c.lwz(c.r[31].wrapping_add(0x10));
                c.lr = 0x80119808; f_8016a17c(c);
                pc = 0x80119808;
            }
            0x80119808 => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x8011980c;
            }
            0x8011980c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801195ac: bad pc {:#x}", pc),
        }
    }
}

/// gsParticleInit (0x80119824, 268 bytes)
#[inline(never)]
pub fn f_80119824(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119824) { return; }
        if c.prof { crate::prof::enter(c, 0x80119824); f_80119824_b(c, 0x80119824); crate::prof::exit(c, 0x80119824); return; }
    }
    f_80119824_b(c, 0x80119824)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119824_at(c: &mut Cpu, pc: u32) { f_80119824_b(c, pc) }
#[inline(always)]
fn f_80119824_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119824;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119824, pc); }
        match pc {
            0x80119824 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[0].wrapping_mul(0x108);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[13].wrapping_add(0xffffa580), c.r[0]);
                c.lr = 0x80119848; f_800e3534(c);
                pc = 0x80119848;
            }
            0x80119848 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.sth(c.r[13].wrapping_add(0xffffa578), c.r[3]);
                if c.crbit(2) { pc = 0x8011991c; continue; }
                pc = 0x80119854;
            }
            0x80119854 => {
                c.r[3] = c.r[0] | c.r[0];
                c.lr = 0x8011985c; f_800e27b0(c);
                pc = 0x8011985c;
            }
            0x8011985c => {
                c.r[5] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa57c), c.r[3]);
                c.r[4] = c.r[5] | c.r[5];
                c.r[6] = 0x0;
                pc = 0x80119880; continue;
                pc = 0x80119870;
            }
            0x80119870 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa57c));
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.stb(c.r[3].wrapping_add(c.r[5]), c.r[4]);
                c.r[5] = c.r[5].wrapping_add(0x108);
                pc = 0x80119880;
            }
            0x80119880 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa580));
                c.cmpu(0, c.r[6], c.r[0]);
                if c.crbit(0) { pc = 0x80119870; continue; }
                pc = 0x8011988c;
            }
            0x8011988c => {
                c.r[3] = c.r[31].wrapping_mul(0x74);
                c.stw(c.r[13].wrapping_add(0xffffa58c), c.r[31]);
                c.lr = 0x80119898; f_800e3534(c);
                pc = 0x80119898;
            }
            0x80119898 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.sth(c.r[13].wrapping_add(0xffffa584), c.r[3]);
                if c.crbit(2) { pc = 0x8011991c; continue; }
                pc = 0x801198a4;
            }
            0x801198a4 => {
                c.r[3] = c.r[0] | c.r[0];
                c.lr = 0x801198ac; f_800e27b0(c);
                pc = 0x801198ac;
            }
            0x801198ac => {
                c.r[6] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa588), c.r[3]);
                c.r[5] = c.r[6] | c.r[6];
                c.r[4] = c.r[6] | c.r[6];
                pc = 0x801198d0; continue;
                pc = 0x801198c0;
            }
            0x801198c0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa588));
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.stb(c.r[3].wrapping_add(c.r[5]), c.r[4]);
                c.r[5] = c.r[5].wrapping_add(0x74);
                pc = 0x801198d0;
            }
            0x801198d0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa58c));
                c.cmpu(0, c.r[6], c.r[0]);
                if c.crbit(0) { pc = 0x801198c0; continue; }
                pc = 0x801198dc;
            }
            0x801198dc => {
                c.r[3] = 0x0;
                c.lr = 0x801198e4; f_8016a01c(c);
                pc = 0x801198e4;
            }
            0x801198e4 => {
                c.r[3] = 0x0;
                c.lr = 0x801198ec; f_80175df0(c);
                pc = 0x801198ec;
            }
            0x801198ec => {
                c.r[3] = 0x0;
                c.r[4] = 0x74;
                c.lr = 0x801198f8; f_8016aaf4(c);
                pc = 0x801198f8;
            }
            0x801198f8 => {
                c.r[3] = 0x80120000;
                c.r[3] = c.r[3].wrapping_add(0xffff9bd0);
                c.lr = 0x80119904; f_8019733c(c);
                pc = 0x80119904;
            }
            0x80119904 => {
                c.r[3] = 0x80120000;
                c.r[3] = c.r[3].wrapping_add(0xffff9bd0);
                c.lr = 0x80119910; f_8019d618(c);
                pc = 0x80119910;
            }
            0x80119910 => {
                c.r[3] = 0x80170000;
                c.r[3] = c.r[3].wrapping_add(0xffff972c);
                c.lr = 0x8011991c; f_8019d610(c);
                pc = 0x8011991c;
            }
            0x8011991c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80119824: bad pc {:#x}", pc),
        }
    }
}

/// gsParticleKillOldest (0x80119930, 672 bytes)
#[inline(never)]
pub fn f_80119930(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119930) { return; }
        if c.prof { crate::prof::enter(c, 0x80119930); f_80119930_b(c, 0x80119930); crate::prof::exit(c, 0x80119930); return; }
    }
    f_80119930_b(c, 0x80119930)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119930_at(c: &mut Cpu, pc: u32) { f_80119930_b(c, pc) }
#[inline(always)]
fn f_80119930_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119930;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119930, pc); }
        match pc {
            0x80119930 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = c.r[3] | c.r[3];
                c.r[8] = 0xffffffff;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x10;
                c.r[9] = 0xffffffff;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[4] = 0x0;
                c.mtspr(9, c.r[0]);
                pc = 0x8011995c;
            }
            0x8011995c => {
                c.r[7] = c.lwz(c.r[5].wrapping_add(0x8));
                c.r[0] = c.lbz(c.r[7]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119974; continue; }
                pc = 0x8011996c;
            }
            0x8011996c => {
                c.stw(c.r[5].wrapping_add(0x8), c.r[4]);
                pc = 0x80119988; continue;
                pc = 0x80119974;
            }
            0x80119974 => {
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x8));
                c.cmpu(0, c.r[0], c.r[8]);
                if !c.crbit(0) { pc = 0x80119988; continue; }
                pc = 0x80119980;
            }
            0x80119980 => {
                c.r[8] = c.r[0] | c.r[0];
                c.r[9] = c.r[6] | c.r[6];
                pc = 0x80119988;
            }
            0x80119988 => {
                c.r[7] = c.lwz(c.r[5].wrapping_add(0xc));
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.r[0] = c.lbz(c.r[7]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801199a4; continue; }
                pc = 0x8011999c;
            }
            0x8011999c => {
                c.stw(c.r[5].wrapping_add(0xc), c.r[4]);
                pc = 0x801199b8; continue;
                pc = 0x801199a4;
            }
            0x801199a4 => {
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x8));
                c.cmpu(0, c.r[0], c.r[8]);
                if !c.crbit(0) { pc = 0x801199b8; continue; }
                pc = 0x801199b0;
            }
            0x801199b0 => {
                c.r[8] = c.r[0] | c.r[0];
                c.r[9] = c.r[6] | c.r[6];
                pc = 0x801199b8;
            }
            0x801199b8 => {
                c.r[7] = c.lwz(c.r[5].wrapping_add(0x10));
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.r[0] = c.lbz(c.r[7]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801199d4; continue; }
                pc = 0x801199cc;
            }
            0x801199cc => {
                c.stw(c.r[5].wrapping_add(0x10), c.r[4]);
                pc = 0x801199e8; continue;
                pc = 0x801199d4;
            }
            0x801199d4 => {
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x8));
                c.cmpu(0, c.r[0], c.r[8]);
                if !c.crbit(0) { pc = 0x801199e8; continue; }
                pc = 0x801199e0;
            }
            0x801199e0 => {
                c.r[8] = c.r[0] | c.r[0];
                c.r[9] = c.r[6] | c.r[6];
                pc = 0x801199e8;
            }
            0x801199e8 => {
                c.r[7] = c.lwz(c.r[5].wrapping_add(0x14));
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.r[0] = c.lbz(c.r[7]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119a04; continue; }
                pc = 0x801199fc;
            }
            0x801199fc => {
                c.stw(c.r[5].wrapping_add(0x14), c.r[4]);
                pc = 0x80119a18; continue;
                pc = 0x80119a04;
            }
            0x80119a04 => {
                c.r[0] = c.lwz(c.r[7].wrapping_add(0x8));
                c.cmpu(0, c.r[0], c.r[8]);
                if !c.crbit(0) { pc = 0x80119a18; continue; }
                pc = 0x80119a10;
            }
            0x80119a10 => {
                c.r[8] = c.r[0] | c.r[0];
                c.r[9] = c.r[6] | c.r[6];
                pc = 0x80119a18;
            }
            0x80119a18 => {
                c.r[5] = c.r[5].wrapping_add(0x10);
                c.r[6] = c.r[6].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x8011995c; continue; }
                pc = 0x80119a24;
            }
            0x80119a24 => {
                c.cmps(0, c.r[9], 0xffffffff);
                if c.crbit(2) { pc = 0x80119bbc; continue; }
                pc = 0x80119a2c;
            }
            0x80119a2c => {
                c.r[0] = c.r[9].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[31] = c.lwz(c.r[3].wrapping_add(0x8));
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x10));
                c.r[3] = c.lhz(c.r[4].wrapping_add(0x18));
                c.r[4] = c.lbz(c.r[4].wrapping_add(0x15));
                c.lr = 0x80119a48; f_801695fc(c);
                pc = 0x80119a48;
            }
            0x80119a48 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80119b6c; continue; }
                pc = 0x80119a54;
            }
            0x80119a54 => {
                if c.crbit(2) { pc = 0x80119b6c; continue; }
                pc = 0x80119a58;
            }
            0x80119a58 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x48));
                c.r[4] = 0x0;
                c.lr = 0x80119a64; f_800ec160(c);
                pc = 0x80119a64;
            }
            0x80119a64 => {
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x48), c.r[0]);
                c.stw(c.r[31].wrapping_add(0x4c), c.r[0]);
                c.stb(c.r[31].wrapping_add(0x6), c.r[0]);
                c.stb(c.r[31].wrapping_add(0x5), c.r[0]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.lr = 0x80119a80; f_80169484(c);
                pc = 0x80119a80;
            }
            0x80119a80 => {
                c.r[0] = 0x0;
                c.stw(c.r[31].wrapping_add(0x44), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119ac8; continue; }
                pc = 0x80119a94;
            }
            0x80119a94 => {
                c.r[3] = c.r[31].wrapping_add(0x14);
                c.r[4] = c.r[31].wrapping_add(0x50);
                c.lr = 0x80119aa0; f_800e01d0(c);
                pc = 0x80119aa0;
            }
            0x80119aa0 => {
                { let ea = c.r[31].wrapping_add(0x50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x54); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x58); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80119ad4; continue;
                pc = 0x80119ac8;
            }
            0x80119ac8 => {
                c.r[3] = c.r[31].wrapping_add(0x50);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80119ad4; f_800e01d0(c);
                pc = 0x80119ad4;
            }
            0x80119ad4 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119b14; continue; }
                pc = 0x80119ae0;
            }
            0x80119ae0 => {
                c.r[3] = c.r[31].wrapping_add(0x20);
                c.r[4] = c.r[31].wrapping_add(0x5c);
                c.lr = 0x80119aec; f_800e01d0(c);
                pc = 0x80119aec;
            }
            0x80119aec => {
                { let ea = c.r[31].wrapping_add(0x5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x8c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x60); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x64); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80119b20; continue;
                pc = 0x80119b14;
            }
            0x80119b14 => {
                c.r[3] = c.r[31].wrapping_add(0x5c);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80119b20; f_800e01d0(c);
                pc = 0x80119b20;
            }
            0x80119b20 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x44));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119b60; continue; }
                pc = 0x80119b2c;
            }
            0x80119b2c => {
                c.r[3] = c.r[31].wrapping_add(0x2c);
                c.r[4] = c.r[31].wrapping_add(0x68);
                c.lr = 0x80119b38; f_800e01d0(c);
                pc = 0x80119b38;
            }
            0x80119b38 => {
                { let ea = c.r[31].wrapping_add(0x68); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x6c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x70); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                { let ea = c.r[3].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80119b6c; continue;
                pc = 0x80119b60;
            }
            0x80119b60 => {
                c.r[3] = c.r[31].wrapping_add(0x68);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80119b6c; f_800e01d0(c);
                pc = 0x80119b6c;
            }
            0x80119b6c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x10));
                c.lr = 0x80119b74; f_80175a1c(c);
                pc = 0x80119b74;
            }
            0x80119b74 => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = 0x40;
                c.r[3] = 0x0;
                c.r[4] = c.r[5] | c.r[5];
                c.mtspr(9, c.r[0]);
                pc = 0x80119b88;
            }
            0x80119b88 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x8));
                c.cmpu(0, c.r[0], c.r[31]);
                if !c.crbit(2) { pc = 0x80119ba8; continue; }
                pc = 0x80119b94;
            }
            0x80119b94 => {
                c.r[0] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[4] = 0x0;
                c.r[3] = c.r[5].wrapping_add(c.r[0]);
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                pc = 0x80119bb4; continue;
                pc = 0x80119ba8;
            }
            0x80119ba8 => {
                c.r[4] = c.r[4].wrapping_add(0x4);
                c.r[3] = c.r[3].wrapping_add(0x1);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80119b88; continue; }
                pc = 0x80119bb4;
            }
            0x80119bb4 => {
                c.r[0] = 0x0;
                c.stb(c.r[31], c.r[0]);
                pc = 0x80119bbc;
            }
            0x80119bbc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80119930: bad pc {:#x}", pc),
        }
    }
}

/// gsParticlePtclHook (0x80119bd0, 448 bytes)
#[inline(never)]
pub fn f_80119bd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119bd0) { return; }
        if c.prof { crate::prof::enter(c, 0x80119bd0); f_80119bd0_b(c, 0x80119bd0); crate::prof::exit(c, 0x80119bd0); return; }
    }
    f_80119bd0_b(c, 0x80119bd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119bd0_at(c: &mut Cpu, pc: u32) { f_80119bd0_b(c, pc) }
#[inline(always)]
fn f_80119bd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119bd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119bd0, pc); }
        match pc {
            0x80119bd0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[6] | c.r[6];
                c.r[27] = c.r[5] | c.r[5];
                c.r[30] = c.r[28] | c.r[28];
                pc = 0x80119c04; continue;
                pc = 0x80119bf0;
            }
            0x80119bf0 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80119c00; continue; }
                pc = 0x80119bf8;
            }
            0x80119bf8 => {
                c.r[30] = 0x0;
                pc = 0x80119c04; continue;
                pc = 0x80119c00;
            }
            0x80119c00 => {
                c.r[30] = c.lwz(c.r[30].wrapping_add(0xc));
                pc = 0x80119c04;
            }
            0x80119c04 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80119c14; continue; }
                pc = 0x80119c0c;
            }
            0x80119c0c => {
                c.r[0] = 0x0;
                pc = 0x80119c18; continue;
                pc = 0x80119c14;
            }
            0x80119c14 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0xc));
                pc = 0x80119c18;
            }
            0x80119c18 => {
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119bf0; continue; }
                pc = 0x80119c20;
            }
            0x80119c20 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80119c28; f_800e3b6c(c);
                pc = 0x80119c28;
            }
            0x80119c28 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x80119d7c; continue; }
                pc = 0x80119c30;
            }
            0x80119c30 => {
                c.lr = 0x80119c34; f_800e3bf8(c);
                pc = 0x80119c34;
            }
            0x80119c34 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x80119d7c; continue; }
                pc = 0x80119c3c;
            }
            0x80119c3c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80119c44; f_800e6dc0(c);
                pc = 0x80119c44;
            }
            0x80119c44 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80119c50; continue; }
                pc = 0x80119c4c;
            }
            0x80119c4c => {
                c.r[30] = c.lwz(c.r[30].wrapping_add(0x10));
                pc = 0x80119c50;
            }
            0x80119c50 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80119c5c; f_800ee22c(c);
                pc = 0x80119c5c;
            }
            0x80119c5c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[0] = c.r[4].wrapping_add(0x10000);
                c.cmpu(0, c.r[0], 0xffff);
                if c.crbit(2) { pc = 0x80119d7c; continue; }
                pc = 0x80119c6c;
            }
            0x80119c6c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80119c74; f_800ee150(c);
                pc = 0x80119c74;
            }
            0x80119c74 => {
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                if c.crbit(2) { pc = 0x80119d7c; continue; }
                pc = 0x80119c7c;
            }
            0x80119c7c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80119c84; f_800e3cbc(c);
                pc = 0x80119c84;
            }
            0x80119c84 => {
                c.r[5] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x80119c94; f_801190dc(c);
                pc = 0x80119c94;
            }
            0x80119c94 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x80119d58; continue; }
                pc = 0x80119c9c;
            }
            0x80119c9c => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80119ca4; f_800e3bf0(c);
                pc = 0x80119ca4;
            }
            0x80119ca4 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x44));
                c.r[31] = c.r[3] | c.r[3];
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80119d58; continue; }
                pc = 0x80119cb4;
            }
            0x80119cb4 => {
                c.cmps(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80119d58; continue; }
                pc = 0x80119cbc;
            }
            0x80119cbc => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x4));
                c.r[3] = c.r[30].wrapping_add(0x50);
                { let ea = c.r[2].wrapping_add(0xffff9948); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[30].wrapping_add(0x48), c.r[0]);
                c.f[2] = c.f[1];
                c.r[0] = c.lhz(c.r[28].wrapping_add(0x2));
                c.f[3] = c.f[1];
                c.stw(c.r[30].wrapping_add(0x4c), c.r[0]);
                c.lr = 0x80119ce0; f_800e01f4(c);
                pc = 0x80119ce0;
            }
            0x80119ce0 => {
                { let ea = c.r[2].wrapping_add(0xffff9948); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30].wrapping_add(0x5c);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80119cf4; f_800e01f4(c);
                pc = 0x80119cf4;
            }
            0x80119cf4 => {
                { let ea = c.r[2].wrapping_add(0xffff994c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30].wrapping_add(0x68);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80119d08; f_800e01f4(c);
                pc = 0x80119d08;
            }
            0x80119d08 => {
                { let ea = c.r[2].wrapping_add(0xffff9948); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30].wrapping_add(0x14);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80119d1c; f_800e01f4(c);
                pc = 0x80119d1c;
            }
            0x80119d1c => {
                { let ea = c.r[2].wrapping_add(0xffff9948); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30].wrapping_add(0x20);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80119d30; f_800e01f4(c);
                pc = 0x80119d30;
            }
            0x80119d30 => {
                { let ea = c.r[2].wrapping_add(0xffff994c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[30].wrapping_add(0x2c);
                c.f[2] = c.f[1];
                c.f[3] = c.f[1];
                c.lr = 0x80119d44; f_800e01f4(c);
                pc = 0x80119d44;
            }
            0x80119d44 => {
                c.stw(c.r[30].wrapping_add(0x44), c.r[31]);
                c.r[3] = 0x0;
                c.r[0] = 0x1;
                c.stb(c.r[30].wrapping_add(0x5), c.r[3]);
                c.stb(c.r[30].wrapping_add(0x6), c.r[0]);
                pc = 0x80119d58;
            }
            0x80119d58 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80119d60; f_800ee828(c);
                pc = 0x80119d60;
            }
            0x80119d60 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80119d68; f_800e3d08(c);
                pc = 0x80119d68;
            }
            0x80119d68 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80119d7c; continue; }
                pc = 0x80119d70;
            }
            0x80119d70 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x10));
                c.r[4] = 0x0;
                c.lr = 0x80119d7c; f_80169034(c);
                pc = 0x80119d7c;
            }
            0x80119d7c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80119bd0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119D90 (0x80119d90, 64 bytes)
#[inline(never)]
pub fn f_80119d90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119d90) { return; }
        if c.prof { crate::prof::enter(c, 0x80119d90); f_80119d90_b(c, 0x80119d90); crate::prof::exit(c, 0x80119d90); return; }
    }
    f_80119d90_b(c, 0x80119d90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119d90_at(c: &mut Cpu, pc: u32) { f_80119d90_b(c, pc) }
#[inline(always)]
fn f_80119d90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119d90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119d90, pc); }
        match pc {
            0x80119d90 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8328));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80119da8; continue; }
                pc = 0x80119da0;
            }
            0x80119da0 => {
                c.r[3] = 0x0;
                pc = 0x80119db8; continue;
                pc = 0x80119da8;
            }
            0x80119da8 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffbba8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80119db8;
            }
            0x80119db8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80119dc8; continue; }
                pc = 0x80119dc0;
            }
            0x80119dc0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80119dc8;
            }
            0x80119dc8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_80119d90: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119DD0 (0x80119dd0, 64 bytes)
#[inline(never)]
pub fn f_80119dd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119dd0) { return; }
        if c.prof { crate::prof::enter(c, 0x80119dd0); f_80119dd0_b(c, 0x80119dd0); crate::prof::exit(c, 0x80119dd0); return; }
    }
    f_80119dd0_b(c, 0x80119dd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119dd0_at(c: &mut Cpu, pc: u32) { f_80119dd0_b(c, pc) }
#[inline(always)]
fn f_80119dd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119dd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119dd0, pc); }
        match pc {
            0x80119dd0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8328));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80119de8; continue; }
                pc = 0x80119de0;
            }
            0x80119de0 => {
                c.r[3] = 0x0;
                pc = 0x80119df8; continue;
                pc = 0x80119de8;
            }
            0x80119de8 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffbba8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80119df8;
            }
            0x80119df8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80119e08; continue; }
                pc = 0x80119e00;
            }
            0x80119e00 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80119e08;
            }
            0x80119e08 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x3));
                if true { return; }
                return;
            }
            _ => panic!("f_80119dd0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119E10 (0x80119e10, 64 bytes)
#[inline(never)]
pub fn f_80119e10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119e10) { return; }
        if c.prof { crate::prof::enter(c, 0x80119e10); f_80119e10_b(c, 0x80119e10); crate::prof::exit(c, 0x80119e10); return; }
    }
    f_80119e10_b(c, 0x80119e10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119e10_at(c: &mut Cpu, pc: u32) { f_80119e10_b(c, pc) }
#[inline(always)]
fn f_80119e10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119e10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119e10, pc); }
        match pc {
            0x80119e10 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8328));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80119e28; continue; }
                pc = 0x80119e20;
            }
            0x80119e20 => {
                c.r[3] = 0x0;
                pc = 0x80119e38; continue;
                pc = 0x80119e28;
            }
            0x80119e28 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffbba8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80119e38;
            }
            0x80119e38 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80119e48; continue; }
                pc = 0x80119e40;
            }
            0x80119e40 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80119e48;
            }
            0x80119e48 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0xa));
                if true { return; }
                return;
            }
            _ => panic!("f_80119e10: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119E50 (0x80119e50, 64 bytes)
#[inline(never)]
pub fn f_80119e50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119e50) { return; }
        if c.prof { crate::prof::enter(c, 0x80119e50); f_80119e50_b(c, 0x80119e50); crate::prof::exit(c, 0x80119e50); return; }
    }
    f_80119e50_b(c, 0x80119e50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119e50_at(c: &mut Cpu, pc: u32) { f_80119e50_b(c, pc) }
#[inline(always)]
fn f_80119e50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119e50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119e50, pc); }
        match pc {
            0x80119e50 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8328));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80119e68; continue; }
                pc = 0x80119e60;
            }
            0x80119e60 => {
                c.r[3] = 0x0;
                pc = 0x80119e78; continue;
                pc = 0x80119e68;
            }
            0x80119e68 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffbba8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80119e78;
            }
            0x80119e78 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80119e88; continue; }
                pc = 0x80119e80;
            }
            0x80119e80 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80119e88;
            }
            0x80119e88 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_80119e50: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119E90 (0x80119e90, 64 bytes)
#[inline(never)]
pub fn f_80119e90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119e90) { return; }
        if c.prof { crate::prof::enter(c, 0x80119e90); f_80119e90_b(c, 0x80119e90); crate::prof::exit(c, 0x80119e90); return; }
    }
    f_80119e90_b(c, 0x80119e90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119e90_at(c: &mut Cpu, pc: u32) { f_80119e90_b(c, pc) }
#[inline(always)]
fn f_80119e90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119e90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119e90, pc); }
        match pc {
            0x80119e90 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8328));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80119ea8; continue; }
                pc = 0x80119ea0;
            }
            0x80119ea0 => {
                c.r[3] = 0x0;
                pc = 0x80119eb8; continue;
                pc = 0x80119ea8;
            }
            0x80119ea8 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffbba8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80119eb8;
            }
            0x80119eb8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80119ec8; continue; }
                pc = 0x80119ec0;
            }
            0x80119ec0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80119ec8;
            }
            0x80119ec8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_80119e90: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119ED0 (0x80119ed0, 64 bytes)
#[inline(never)]
pub fn f_80119ed0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119ed0) { return; }
        if c.prof { crate::prof::enter(c, 0x80119ed0); f_80119ed0_b(c, 0x80119ed0); crate::prof::exit(c, 0x80119ed0); return; }
    }
    f_80119ed0_b(c, 0x80119ed0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119ed0_at(c: &mut Cpu, pc: u32) { f_80119ed0_b(c, pc) }
#[inline(always)]
fn f_80119ed0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119ed0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119ed0, pc); }
        match pc {
            0x80119ed0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8328));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80119ee8; continue; }
                pc = 0x80119ee0;
            }
            0x80119ee0 => {
                c.r[3] = 0x0;
                pc = 0x80119ef8; continue;
                pc = 0x80119ee8;
            }
            0x80119ee8 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffbba8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80119ef8;
            }
            0x80119ef8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80119f08; continue; }
                pc = 0x80119f00;
            }
            0x80119f00 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80119f08;
            }
            0x80119f08 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_80119ed0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119F10 (0x80119f10, 64 bytes)
#[inline(never)]
pub fn f_80119f10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119f10) { return; }
        if c.prof { crate::prof::enter(c, 0x80119f10); f_80119f10_b(c, 0x80119f10); crate::prof::exit(c, 0x80119f10); return; }
    }
    f_80119f10_b(c, 0x80119f10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119f10_at(c: &mut Cpu, pc: u32) { f_80119f10_b(c, pc) }
#[inline(always)]
fn f_80119f10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119f10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119f10, pc); }
        match pc {
            0x80119f10 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8328));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80119f28; continue; }
                pc = 0x80119f20;
            }
            0x80119f20 => {
                c.r[3] = 0x0;
                pc = 0x80119f38; continue;
                pc = 0x80119f28;
            }
            0x80119f28 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffbba8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80119f38;
            }
            0x80119f38 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80119f48; continue; }
                pc = 0x80119f40;
            }
            0x80119f40 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80119f48;
            }
            0x80119f48 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_80119f10: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119F50 (0x80119f50, 64 bytes)
#[inline(never)]
pub fn f_80119f50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119f50) { return; }
        if c.prof { crate::prof::enter(c, 0x80119f50); f_80119f50_b(c, 0x80119f50); crate::prof::exit(c, 0x80119f50); return; }
    }
    f_80119f50_b(c, 0x80119f50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119f50_at(c: &mut Cpu, pc: u32) { f_80119f50_b(c, pc) }
#[inline(always)]
fn f_80119f50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119f50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119f50, pc); }
        match pc {
            0x80119f50 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8328));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80119f68; continue; }
                pc = 0x80119f60;
            }
            0x80119f60 => {
                c.r[3] = 0x0;
                pc = 0x80119f78; continue;
                pc = 0x80119f68;
            }
            0x80119f68 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffbba8);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80119f78;
            }
            0x80119f78 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80119f88; continue; }
                pc = 0x80119f80;
            }
            0x80119f80 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80119f88;
            }
            0x80119f88 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                if true { return; }
                return;
            }
            _ => panic!("f_80119f50: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119F90 (0x80119f90, 16 bytes)
#[inline(never)]
pub fn f_80119f90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119f90) { return; }
        if c.prof { crate::prof::enter(c, 0x80119f90); f_80119f90_b(c, 0x80119f90); crate::prof::exit(c, 0x80119f90); return; }
    }
    f_80119f90_b(c, 0x80119f90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119f90_at(c: &mut Cpu, pc: u32) { f_80119f90_b(c, pc) }
#[inline(always)]
fn f_80119f90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119f90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119f90, pc); }
        match pc {
            0x80119f90 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80119f98;
            }
            0x80119f98 => {
                c.sth(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80119f90: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119FA0 (0x80119fa0, 16 bytes)
#[inline(never)]
pub fn f_80119fa0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119fa0) { return; }
        if c.prof { crate::prof::enter(c, 0x80119fa0); f_80119fa0_b(c, 0x80119fa0); crate::prof::exit(c, 0x80119fa0); return; }
    }
    f_80119fa0_b(c, 0x80119fa0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119fa0_at(c: &mut Cpu, pc: u32) { f_80119fa0_b(c, pc) }
#[inline(always)]
fn f_80119fa0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119fa0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119fa0, pc); }
        match pc {
            0x80119fa0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80119fa8;
            }
            0x80119fa8 => {
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80119fa0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119FB0 (0x80119fb0, 16 bytes)
#[inline(never)]
pub fn f_80119fb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119fb0) { return; }
        if c.prof { crate::prof::enter(c, 0x80119fb0); f_80119fb0_b(c, 0x80119fb0); crate::prof::exit(c, 0x80119fb0); return; }
    }
    f_80119fb0_b(c, 0x80119fb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119fb0_at(c: &mut Cpu, pc: u32) { f_80119fb0_b(c, pc) }
#[inline(always)]
fn f_80119fb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119fb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119fb0, pc); }
        match pc {
            0x80119fb0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80119fb8;
            }
            0x80119fb8 => {
                c.stb(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80119fb0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119FC0 (0x80119fc0, 16 bytes)
#[inline(never)]
pub fn f_80119fc0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119fc0) { return; }
        if c.prof { crate::prof::enter(c, 0x80119fc0); f_80119fc0_b(c, 0x80119fc0); crate::prof::exit(c, 0x80119fc0); return; }
    }
    f_80119fc0_b(c, 0x80119fc0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119fc0_at(c: &mut Cpu, pc: u32) { f_80119fc0_b(c, pc) }
#[inline(always)]
fn f_80119fc0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119fc0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119fc0, pc); }
        match pc {
            0x80119fc0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80119fc8;
            }
            0x80119fc8 => {
                c.stb(c.r[3].wrapping_add(0x5), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80119fc0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119FD0 (0x80119fd0, 16 bytes)
#[inline(never)]
pub fn f_80119fd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119fd0) { return; }
        if c.prof { crate::prof::enter(c, 0x80119fd0); f_80119fd0_b(c, 0x80119fd0); crate::prof::exit(c, 0x80119fd0); return; }
    }
    f_80119fd0_b(c, 0x80119fd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119fd0_at(c: &mut Cpu, pc: u32) { f_80119fd0_b(c, pc) }
#[inline(always)]
fn f_80119fd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119fd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119fd0, pc); }
        match pc {
            0x80119fd0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80119fd8;
            }
            0x80119fd8 => {
                c.stb(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80119fd0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119FE0 (0x80119fe0, 16 bytes)
#[inline(never)]
pub fn f_80119fe0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119fe0) { return; }
        if c.prof { crate::prof::enter(c, 0x80119fe0); f_80119fe0_b(c, 0x80119fe0); crate::prof::exit(c, 0x80119fe0); return; }
    }
    f_80119fe0_b(c, 0x80119fe0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119fe0_at(c: &mut Cpu, pc: u32) { f_80119fe0_b(c, pc) }
#[inline(always)]
fn f_80119fe0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119fe0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119fe0, pc); }
        match pc {
            0x80119fe0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80119fe8;
            }
            0x80119fe8 => {
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80119fe0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80119FF0 (0x80119ff0, 16 bytes)
#[inline(never)]
pub fn f_80119ff0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80119ff0) { return; }
        if c.prof { crate::prof::enter(c, 0x80119ff0); f_80119ff0_b(c, 0x80119ff0); crate::prof::exit(c, 0x80119ff0); return; }
    }
    f_80119ff0_b(c, 0x80119ff0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80119ff0_at(c: &mut Cpu, pc: u32) { f_80119ff0_b(c, pc) }
#[inline(always)]
fn f_80119ff0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80119ff0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80119ff0, pc); }
        match pc {
            0x80119ff0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80119ff8;
            }
            0x80119ff8 => {
                c.sth(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80119ff0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A000 (0x8011a000, 24 bytes)
#[inline(never)]
pub fn f_8011a000(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a000) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a000); f_8011a000_b(c, 0x8011a000); crate::prof::exit(c, 0x8011a000); return; }
    }
    f_8011a000_b(c, 0x8011a000)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a000_at(c: &mut Cpu, pc: u32) { f_8011a000_b(c, pc) }
#[inline(always)]
fn f_8011a000_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a000;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a000, pc); }
        match pc {
            0x8011a000 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a010; continue; }
                pc = 0x8011a008;
            }
            0x8011a008 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011a010;
            }
            0x8011a010 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0xc));
                if true { return; }
                return;
            }
            _ => panic!("f_8011a000: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A018 (0x8011a018, 24 bytes)
#[inline(never)]
pub fn f_8011a018(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a018) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a018); f_8011a018_b(c, 0x8011a018); crate::prof::exit(c, 0x8011a018); return; }
    }
    f_8011a018_b(c, 0x8011a018)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a018_at(c: &mut Cpu, pc: u32) { f_8011a018_b(c, pc) }
#[inline(always)]
fn f_8011a018_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a018;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a018, pc); }
        match pc {
            0x8011a018 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a028; continue; }
                pc = 0x8011a020;
            }
            0x8011a020 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011a028;
            }
            0x8011a028 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8011a018: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A030 (0x8011a030, 24 bytes)
#[inline(never)]
pub fn f_8011a030(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a030) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a030); f_8011a030_b(c, 0x8011a030); crate::prof::exit(c, 0x8011a030); return; }
    }
    f_8011a030_b(c, 0x8011a030)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a030_at(c: &mut Cpu, pc: u32) { f_8011a030_b(c, pc) }
#[inline(always)]
fn f_8011a030_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a030;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a030, pc); }
        match pc {
            0x8011a030 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a040; continue; }
                pc = 0x8011a038;
            }
            0x8011a038 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011a040;
            }
            0x8011a040 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x6));
                if true { return; }
                return;
            }
            _ => panic!("f_8011a030: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A048 (0x8011a048, 24 bytes)
#[inline(never)]
pub fn f_8011a048(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a048) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a048); f_8011a048_b(c, 0x8011a048); crate::prof::exit(c, 0x8011a048); return; }
    }
    f_8011a048_b(c, 0x8011a048)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a048_at(c: &mut Cpu, pc: u32) { f_8011a048_b(c, pc) }
#[inline(always)]
fn f_8011a048_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a048;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a048, pc); }
        match pc {
            0x8011a048 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a058; continue; }
                pc = 0x8011a050;
            }
            0x8011a050 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011a058;
            }
            0x8011a058 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x5));
                if true { return; }
                return;
            }
            _ => panic!("f_8011a048: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A060 (0x8011a060, 24 bytes)
#[inline(never)]
pub fn f_8011a060(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a060) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a060); f_8011a060_b(c, 0x8011a060); crate::prof::exit(c, 0x8011a060); return; }
    }
    f_8011a060_b(c, 0x8011a060)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a060_at(c: &mut Cpu, pc: u32) { f_8011a060_b(c, pc) }
#[inline(always)]
fn f_8011a060_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a060;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a060, pc); }
        match pc {
            0x8011a060 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a070; continue; }
                pc = 0x8011a068;
            }
            0x8011a068 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011a070;
            }
            0x8011a070 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8011a060: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A078 (0x8011a078, 24 bytes)
#[inline(never)]
pub fn f_8011a078(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a078) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a078); f_8011a078_b(c, 0x8011a078); crate::prof::exit(c, 0x8011a078); return; }
    }
    f_8011a078_b(c, 0x8011a078)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a078_at(c: &mut Cpu, pc: u32) { f_8011a078_b(c, pc) }
#[inline(always)]
fn f_8011a078_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a078;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a078, pc); }
        match pc {
            0x8011a078 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a088; continue; }
                pc = 0x8011a080;
            }
            0x8011a080 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011a088;
            }
            0x8011a088 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8011a078: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A090 (0x8011a090, 24 bytes)
#[inline(never)]
pub fn f_8011a090(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a090) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a090); f_8011a090_b(c, 0x8011a090); crate::prof::exit(c, 0x8011a090); return; }
    }
    f_8011a090_b(c, 0x8011a090)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a090_at(c: &mut Cpu, pc: u32) { f_8011a090_b(c, pc) }
#[inline(always)]
fn f_8011a090_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a090;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a090, pc); }
        match pc {
            0x8011a090 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a0a0; continue; }
                pc = 0x8011a098;
            }
            0x8011a098 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011a0a0;
            }
            0x8011a0a0 => {
                c.r[3] = c.lhz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011a090: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A0A8 (0x8011a0a8, 472 bytes)
#[inline(never)]
pub fn f_8011a0a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a0a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a0a8); f_8011a0a8_b(c, 0x8011a0a8); crate::prof::exit(c, 0x8011a0a8); return; }
    }
    f_8011a0a8_b(c, 0x8011a0a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a0a8_at(c: &mut Cpu, pc: u32) { f_8011a0a8_b(c, pc) }
#[inline(always)]
fn f_8011a0a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a0a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a0a8, pc); }
        match pc {
            0x8011a0a8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[5] | c.r[5];
                c.r[27] = c.r[3] | c.r[3];
                c.r[29] = c.r[4] | c.r[4];
                if c.crbit(2) { pc = 0x8011a26c; continue; }
                pc = 0x8011a0cc;
            }
            0x8011a0cc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a0d4; f_80119e90(c);
                pc = 0x8011a0d4;
            }
            0x8011a0d4 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a0e0; f_80119f10(c);
                pc = 0x8011a0e0;
            }
            0x8011a0e0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a0ec; f_80119ed0(c);
                pc = 0x8011a0ec;
            }
            0x8011a0ec => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a104; f_80135e44(c);
                pc = 0x8011a104;
            }
            0x8011a104 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a114; continue; }
                pc = 0x8011a10c;
            }
            0x8011a10c => {
                c.r[31] = 0x0;
                pc = 0x8011a11c; continue;
                pc = 0x8011a114;
            }
            0x8011a114 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a11c;
            }
            0x8011a11c => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8011a26c; continue; }
                pc = 0x8011a124;
            }
            0x8011a124 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011a134; continue; }
                pc = 0x8011a12c;
            }
            0x8011a12c => {
                c.r[0] = 0x0;
                pc = 0x8011a1ec; continue;
                pc = 0x8011a134;
            }
            0x8011a134 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a13c; f_80119e90(c);
                pc = 0x8011a13c;
            }
            0x8011a13c => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a148; f_80119f10(c);
                pc = 0x8011a148;
            }
            0x8011a148 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a154; f_80119ed0(c);
                pc = 0x8011a154;
            }
            0x8011a154 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a16c; f_80135e44(c);
                pc = 0x8011a16c;
            }
            0x8011a16c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a17c; continue; }
                pc = 0x8011a174;
            }
            0x8011a174 => {
                c.r[27] = 0x0;
                pc = 0x8011a184; continue;
                pc = 0x8011a17c;
            }
            0x8011a17c => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[27] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a184;
            }
            0x8011a184 => {
                c.cmpu(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x8011a194; continue; }
                pc = 0x8011a18c;
            }
            0x8011a18c => {
                c.r[0] = 0x0;
                pc = 0x8011a1ec; continue;
                pc = 0x8011a194;
            }
            0x8011a194 => {
                if !c.crbit(2) { pc = 0x8011a1a0; continue; }
                pc = 0x8011a198;
            }
            0x8011a198 => {
                c.r[0] = 0x0;
                pc = 0x8011a1bc; continue;
                pc = 0x8011a1a0;
            }
            0x8011a1a0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011a1a8; f_8011a090(c);
                pc = 0x8011a1a8;
            }
            0x8011a1a8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011a1b8; continue; }
                pc = 0x8011a1b0;
            }
            0x8011a1b0 => {
                c.r[0] = 0x0;
                pc = 0x8011a1bc; continue;
                pc = 0x8011a1b8;
            }
            0x8011a1b8 => {
                c.r[0] = 0x1;
                pc = 0x8011a1bc;
            }
            0x8011a1bc => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011a1e8; continue; }
                pc = 0x8011a1c8;
            }
            0x8011a1c8 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011a1d0; f_8011a090(c);
                pc = 0x8011a1d0;
            }
            0x8011a1d0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011a1e8; continue; }
                pc = 0x8011a1e0;
            }
            0x8011a1e0 => {
                c.r[0] = 0x1;
                pc = 0x8011a1ec; continue;
                pc = 0x8011a1e8;
            }
            0x8011a1e8 => {
                c.r[0] = 0x0;
                pc = 0x8011a1ec;
            }
            0x8011a1ec => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8011a26c; continue; }
                pc = 0x8011a1f4;
            }
            0x8011a1f4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a1fc; f_80119e90(c);
                pc = 0x8011a1fc;
            }
            0x8011a1fc => {
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a208; f_80119f10(c);
                pc = 0x8011a208;
            }
            0x8011a208 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a214; f_80119ed0(c);
                pc = 0x8011a214;
            }
            0x8011a214 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a22c; f_80135e44(c);
                pc = 0x8011a22c;
            }
            0x8011a22c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a23c; continue; }
                pc = 0x8011a234;
            }
            0x8011a234 => {
                c.r[4] = 0x0;
                pc = 0x8011a244; continue;
                pc = 0x8011a23c;
            }
            0x8011a23c => {
                c.r[0] = c.r[27].rotate_left(4) & 0xffff0;
                c.r[4] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a244;
            }
            0x8011a244 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { pc = 0x8011a26c; continue; }
                pc = 0x8011a24c;
            }
            0x8011a24c => {
                c.r[3] = c.lwz(c.r[4]);
                c.r[0] = c.lwz(c.r[4].wrapping_add(0x4));
                c.stw(c.r[31], c.r[3]);
                c.stw(c.r[31].wrapping_add(0x4), c.r[0]);
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x8));
                c.r[0] = c.lwz(c.r[4].wrapping_add(0xc));
                c.stw(c.r[31].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[31].wrapping_add(0xc), c.r[0]);
                pc = 0x8011a26c;
            }
            0x8011a26c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8011a0a8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A280 (0x8011a280, 356 bytes)
#[inline(never)]
pub fn f_8011a280(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a280) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a280); f_8011a280_b(c, 0x8011a280); crate::prof::exit(c, 0x8011a280); return; }
    }
    f_8011a280_b(c, 0x8011a280)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a280_at(c: &mut Cpu, pc: u32) { f_8011a280_b(c, pc) }
#[inline(always)]
fn f_8011a280_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a280;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a280, pc); }
        match pc {
            0x8011a280 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                if c.crbit(2) { pc = 0x8011a3d0; continue; }
                pc = 0x8011a2a4;
            }
            0x8011a2a4 => {
                if !c.crbit(2) { pc = 0x8011a2b0; continue; }
                pc = 0x8011a2a8;
            }
            0x8011a2a8 => {
                c.r[0] = 0x0;
                pc = 0x8011a368; continue;
                pc = 0x8011a2b0;
            }
            0x8011a2b0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a2b8; f_80119e90(c);
                pc = 0x8011a2b8;
            }
            0x8011a2b8 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a2c4; f_80119f10(c);
                pc = 0x8011a2c4;
            }
            0x8011a2c4 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a2d0; f_80119ed0(c);
                pc = 0x8011a2d0;
            }
            0x8011a2d0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a2e8; f_80135e44(c);
                pc = 0x8011a2e8;
            }
            0x8011a2e8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a2f8; continue; }
                pc = 0x8011a2f0;
            }
            0x8011a2f0 => {
                c.r[27] = 0x0;
                pc = 0x8011a300; continue;
                pc = 0x8011a2f8;
            }
            0x8011a2f8 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[27] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a300;
            }
            0x8011a300 => {
                c.cmpu(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x8011a310; continue; }
                pc = 0x8011a308;
            }
            0x8011a308 => {
                c.r[0] = 0x0;
                pc = 0x8011a368; continue;
                pc = 0x8011a310;
            }
            0x8011a310 => {
                if !c.crbit(2) { pc = 0x8011a31c; continue; }
                pc = 0x8011a314;
            }
            0x8011a314 => {
                c.r[0] = 0x0;
                pc = 0x8011a338; continue;
                pc = 0x8011a31c;
            }
            0x8011a31c => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011a324; f_8011a090(c);
                pc = 0x8011a324;
            }
            0x8011a324 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011a334; continue; }
                pc = 0x8011a32c;
            }
            0x8011a32c => {
                c.r[0] = 0x0;
                pc = 0x8011a338; continue;
                pc = 0x8011a334;
            }
            0x8011a334 => {
                c.r[0] = 0x1;
                pc = 0x8011a338;
            }
            0x8011a338 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011a364; continue; }
                pc = 0x8011a344;
            }
            0x8011a344 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011a34c; f_8011a090(c);
                pc = 0x8011a34c;
            }
            0x8011a34c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011a364; continue; }
                pc = 0x8011a35c;
            }
            0x8011a35c => {
                c.r[0] = 0x1;
                pc = 0x8011a368; continue;
                pc = 0x8011a364;
            }
            0x8011a364 => {
                c.r[0] = 0x0;
                pc = 0x8011a368;
            }
            0x8011a368 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8011a3d0; continue; }
                pc = 0x8011a370;
            }
            0x8011a370 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a378; f_80119e90(c);
                pc = 0x8011a378;
            }
            0x8011a378 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a384; f_80119f10(c);
                pc = 0x8011a384;
            }
            0x8011a384 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a390; f_80119ed0(c);
                pc = 0x8011a390;
            }
            0x8011a390 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a3a8; f_80135e44(c);
                pc = 0x8011a3a8;
            }
            0x8011a3a8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a3b8; continue; }
                pc = 0x8011a3b0;
            }
            0x8011a3b0 => {
                c.r[3] = 0x0;
                pc = 0x8011a3c0; continue;
                pc = 0x8011a3b8;
            }
            0x8011a3b8 => {
                c.r[0] = c.r[27].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a3c0;
            }
            0x8011a3c0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8011a3d0; continue; }
                pc = 0x8011a3c8;
            }
            0x8011a3c8 => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8011a3d0; f_80119fa0(c);
                pc = 0x8011a3d0;
            }
            0x8011a3d0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8011a280: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A3E4 (0x8011a3e4, 396 bytes)
#[inline(never)]
pub fn f_8011a3e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a3e4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a3e4); f_8011a3e4_b(c, 0x8011a3e4); crate::prof::exit(c, 0x8011a3e4); return; }
    }
    f_8011a3e4_b(c, 0x8011a3e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a3e4_at(c: &mut Cpu, pc: u32) { f_8011a3e4_b(c, pc) }
#[inline(always)]
fn f_8011a3e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a3e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a3e4, pc); }
        match pc {
            0x8011a3e4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                if !c.crbit(2) { pc = 0x8011a418; continue; }
                pc = 0x8011a410;
            }
            0x8011a410 => {
                c.r[3] = 0x0;
                pc = 0x8011a550; continue;
                pc = 0x8011a418;
            }
            0x8011a418 => {
                if !c.crbit(2) { pc = 0x8011a424; continue; }
                pc = 0x8011a41c;
            }
            0x8011a41c => {
                c.r[0] = 0x0;
                pc = 0x8011a4dc; continue;
                pc = 0x8011a424;
            }
            0x8011a424 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a42c; f_80119e90(c);
                pc = 0x8011a42c;
            }
            0x8011a42c => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a438; f_80119f10(c);
                pc = 0x8011a438;
            }
            0x8011a438 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a444; f_80119ed0(c);
                pc = 0x8011a444;
            }
            0x8011a444 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a45c; f_80135e44(c);
                pc = 0x8011a45c;
            }
            0x8011a45c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a46c; continue; }
                pc = 0x8011a464;
            }
            0x8011a464 => {
                c.r[28] = 0x0;
                pc = 0x8011a474; continue;
                pc = 0x8011a46c;
            }
            0x8011a46c => {
                c.r[0] = c.r[29].rotate_left(4) & 0xffff0;
                c.r[28] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a474;
            }
            0x8011a474 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8011a484; continue; }
                pc = 0x8011a47c;
            }
            0x8011a47c => {
                c.r[0] = 0x0;
                pc = 0x8011a4dc; continue;
                pc = 0x8011a484;
            }
            0x8011a484 => {
                if !c.crbit(2) { pc = 0x8011a490; continue; }
                pc = 0x8011a488;
            }
            0x8011a488 => {
                c.r[0] = 0x0;
                pc = 0x8011a4ac; continue;
                pc = 0x8011a490;
            }
            0x8011a490 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011a498; f_8011a090(c);
                pc = 0x8011a498;
            }
            0x8011a498 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011a4a8; continue; }
                pc = 0x8011a4a0;
            }
            0x8011a4a0 => {
                c.r[0] = 0x0;
                pc = 0x8011a4ac; continue;
                pc = 0x8011a4a8;
            }
            0x8011a4a8 => {
                c.r[0] = 0x1;
                pc = 0x8011a4ac;
            }
            0x8011a4ac => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011a4d8; continue; }
                pc = 0x8011a4b8;
            }
            0x8011a4b8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011a4c0; f_8011a090(c);
                pc = 0x8011a4c0;
            }
            0x8011a4c0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011a4d8; continue; }
                pc = 0x8011a4d0;
            }
            0x8011a4d0 => {
                c.r[0] = 0x1;
                pc = 0x8011a4dc; continue;
                pc = 0x8011a4d8;
            }
            0x8011a4d8 => {
                c.r[0] = 0x0;
                pc = 0x8011a4dc;
            }
            0x8011a4dc => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011a4ec; continue; }
                pc = 0x8011a4e4;
            }
            0x8011a4e4 => {
                c.r[3] = 0x0;
                pc = 0x8011a550; continue;
                pc = 0x8011a4ec;
            }
            0x8011a4ec => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a4f4; f_80119e90(c);
                pc = 0x8011a4f4;
            }
            0x8011a4f4 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a500; f_80119f10(c);
                pc = 0x8011a500;
            }
            0x8011a500 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a50c; f_80119ed0(c);
                pc = 0x8011a50c;
            }
            0x8011a50c => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a524; f_80135e44(c);
                pc = 0x8011a524;
            }
            0x8011a524 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a534; continue; }
                pc = 0x8011a52c;
            }
            0x8011a52c => {
                c.r[3] = 0x0;
                pc = 0x8011a53c; continue;
                pc = 0x8011a534;
            }
            0x8011a534 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a53c;
            }
            0x8011a53c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a54c; continue; }
                pc = 0x8011a544;
            }
            0x8011a544 => {
                c.r[3] = 0x0;
                pc = 0x8011a550; continue;
                pc = 0x8011a54c;
            }
            0x8011a54c => {
                c.lr = 0x8011a550; f_8011a018(c);
                pc = 0x8011a550;
            }
            0x8011a550 => {
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
            _ => panic!("f_8011a3e4: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A570 (0x8011a570, 356 bytes)
#[inline(never)]
pub fn f_8011a570(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a570) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a570); f_8011a570_b(c, 0x8011a570); crate::prof::exit(c, 0x8011a570); return; }
    }
    f_8011a570_b(c, 0x8011a570)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a570_at(c: &mut Cpu, pc: u32) { f_8011a570_b(c, pc) }
#[inline(always)]
fn f_8011a570_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a570;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a570, pc); }
        match pc {
            0x8011a570 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                if c.crbit(2) { pc = 0x8011a6c0; continue; }
                pc = 0x8011a594;
            }
            0x8011a594 => {
                if !c.crbit(2) { pc = 0x8011a5a0; continue; }
                pc = 0x8011a598;
            }
            0x8011a598 => {
                c.r[0] = 0x0;
                pc = 0x8011a658; continue;
                pc = 0x8011a5a0;
            }
            0x8011a5a0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a5a8; f_80119e90(c);
                pc = 0x8011a5a8;
            }
            0x8011a5a8 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a5b4; f_80119f10(c);
                pc = 0x8011a5b4;
            }
            0x8011a5b4 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a5c0; f_80119ed0(c);
                pc = 0x8011a5c0;
            }
            0x8011a5c0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a5d8; f_80135e44(c);
                pc = 0x8011a5d8;
            }
            0x8011a5d8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a5e8; continue; }
                pc = 0x8011a5e0;
            }
            0x8011a5e0 => {
                c.r[27] = 0x0;
                pc = 0x8011a5f0; continue;
                pc = 0x8011a5e8;
            }
            0x8011a5e8 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[27] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a5f0;
            }
            0x8011a5f0 => {
                c.cmpu(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x8011a600; continue; }
                pc = 0x8011a5f8;
            }
            0x8011a5f8 => {
                c.r[0] = 0x0;
                pc = 0x8011a658; continue;
                pc = 0x8011a600;
            }
            0x8011a600 => {
                if !c.crbit(2) { pc = 0x8011a60c; continue; }
                pc = 0x8011a604;
            }
            0x8011a604 => {
                c.r[0] = 0x0;
                pc = 0x8011a628; continue;
                pc = 0x8011a60c;
            }
            0x8011a60c => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011a614; f_8011a090(c);
                pc = 0x8011a614;
            }
            0x8011a614 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011a624; continue; }
                pc = 0x8011a61c;
            }
            0x8011a61c => {
                c.r[0] = 0x0;
                pc = 0x8011a628; continue;
                pc = 0x8011a624;
            }
            0x8011a624 => {
                c.r[0] = 0x1;
                pc = 0x8011a628;
            }
            0x8011a628 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011a654; continue; }
                pc = 0x8011a634;
            }
            0x8011a634 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011a63c; f_8011a090(c);
                pc = 0x8011a63c;
            }
            0x8011a63c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011a654; continue; }
                pc = 0x8011a64c;
            }
            0x8011a64c => {
                c.r[0] = 0x1;
                pc = 0x8011a658; continue;
                pc = 0x8011a654;
            }
            0x8011a654 => {
                c.r[0] = 0x0;
                pc = 0x8011a658;
            }
            0x8011a658 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8011a6c0; continue; }
                pc = 0x8011a660;
            }
            0x8011a660 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a668; f_80119e90(c);
                pc = 0x8011a668;
            }
            0x8011a668 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a674; f_80119f10(c);
                pc = 0x8011a674;
            }
            0x8011a674 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011a680; f_80119ed0(c);
                pc = 0x8011a680;
            }
            0x8011a680 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a698; f_80135e44(c);
                pc = 0x8011a698;
            }
            0x8011a698 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a6a8; continue; }
                pc = 0x8011a6a0;
            }
            0x8011a6a0 => {
                c.r[3] = 0x0;
                pc = 0x8011a6b0; continue;
                pc = 0x8011a6a8;
            }
            0x8011a6a8 => {
                c.r[0] = c.r[27].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a6b0;
            }
            0x8011a6b0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8011a6c0; continue; }
                pc = 0x8011a6b8;
            }
            0x8011a6b8 => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8011a6c0; f_80119f90(c);
                pc = 0x8011a6c0;
            }
            0x8011a6c0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8011a570: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A6D4 (0x8011a6d4, 396 bytes)
#[inline(never)]
pub fn f_8011a6d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a6d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a6d4); f_8011a6d4_b(c, 0x8011a6d4); crate::prof::exit(c, 0x8011a6d4); return; }
    }
    f_8011a6d4_b(c, 0x8011a6d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a6d4_at(c: &mut Cpu, pc: u32) { f_8011a6d4_b(c, pc) }
#[inline(always)]
fn f_8011a6d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a6d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a6d4, pc); }
        match pc {
            0x8011a6d4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                if !c.crbit(2) { pc = 0x8011a708; continue; }
                pc = 0x8011a700;
            }
            0x8011a700 => {
                c.r[3] = 0x0;
                pc = 0x8011a840; continue;
                pc = 0x8011a708;
            }
            0x8011a708 => {
                if !c.crbit(2) { pc = 0x8011a714; continue; }
                pc = 0x8011a70c;
            }
            0x8011a70c => {
                c.r[0] = 0x0;
                pc = 0x8011a7cc; continue;
                pc = 0x8011a714;
            }
            0x8011a714 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a71c; f_80119e90(c);
                pc = 0x8011a71c;
            }
            0x8011a71c => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a728; f_80119f10(c);
                pc = 0x8011a728;
            }
            0x8011a728 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a734; f_80119ed0(c);
                pc = 0x8011a734;
            }
            0x8011a734 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a74c; f_80135e44(c);
                pc = 0x8011a74c;
            }
            0x8011a74c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a75c; continue; }
                pc = 0x8011a754;
            }
            0x8011a754 => {
                c.r[28] = 0x0;
                pc = 0x8011a764; continue;
                pc = 0x8011a75c;
            }
            0x8011a75c => {
                c.r[0] = c.r[29].rotate_left(4) & 0xffff0;
                c.r[28] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a764;
            }
            0x8011a764 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8011a774; continue; }
                pc = 0x8011a76c;
            }
            0x8011a76c => {
                c.r[0] = 0x0;
                pc = 0x8011a7cc; continue;
                pc = 0x8011a774;
            }
            0x8011a774 => {
                if !c.crbit(2) { pc = 0x8011a780; continue; }
                pc = 0x8011a778;
            }
            0x8011a778 => {
                c.r[0] = 0x0;
                pc = 0x8011a79c; continue;
                pc = 0x8011a780;
            }
            0x8011a780 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011a788; f_8011a090(c);
                pc = 0x8011a788;
            }
            0x8011a788 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011a798; continue; }
                pc = 0x8011a790;
            }
            0x8011a790 => {
                c.r[0] = 0x0;
                pc = 0x8011a79c; continue;
                pc = 0x8011a798;
            }
            0x8011a798 => {
                c.r[0] = 0x1;
                pc = 0x8011a79c;
            }
            0x8011a79c => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011a7c8; continue; }
                pc = 0x8011a7a8;
            }
            0x8011a7a8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011a7b0; f_8011a090(c);
                pc = 0x8011a7b0;
            }
            0x8011a7b0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011a7c8; continue; }
                pc = 0x8011a7c0;
            }
            0x8011a7c0 => {
                c.r[0] = 0x1;
                pc = 0x8011a7cc; continue;
                pc = 0x8011a7c8;
            }
            0x8011a7c8 => {
                c.r[0] = 0x0;
                pc = 0x8011a7cc;
            }
            0x8011a7cc => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011a7dc; continue; }
                pc = 0x8011a7d4;
            }
            0x8011a7d4 => {
                c.r[3] = 0x0;
                pc = 0x8011a840; continue;
                pc = 0x8011a7dc;
            }
            0x8011a7dc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a7e4; f_80119e90(c);
                pc = 0x8011a7e4;
            }
            0x8011a7e4 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a7f0; f_80119f10(c);
                pc = 0x8011a7f0;
            }
            0x8011a7f0 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a7fc; f_80119ed0(c);
                pc = 0x8011a7fc;
            }
            0x8011a7fc => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a814; f_80135e44(c);
                pc = 0x8011a814;
            }
            0x8011a814 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a824; continue; }
                pc = 0x8011a81c;
            }
            0x8011a81c => {
                c.r[3] = 0x0;
                pc = 0x8011a82c; continue;
                pc = 0x8011a824;
            }
            0x8011a824 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a82c;
            }
            0x8011a82c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a83c; continue; }
                pc = 0x8011a834;
            }
            0x8011a834 => {
                c.r[3] = 0x0;
                pc = 0x8011a840; continue;
                pc = 0x8011a83c;
            }
            0x8011a83c => {
                c.lr = 0x8011a840; f_8011a000(c);
                pc = 0x8011a840;
            }
            0x8011a840 => {
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
            _ => panic!("f_8011a6d4: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A860 (0x8011a860, 396 bytes)
#[inline(never)]
pub fn f_8011a860(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a860) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a860); f_8011a860_b(c, 0x8011a860); crate::prof::exit(c, 0x8011a860); return; }
    }
    f_8011a860_b(c, 0x8011a860)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a860_at(c: &mut Cpu, pc: u32) { f_8011a860_b(c, pc) }
#[inline(always)]
fn f_8011a860_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a860;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a860, pc); }
        match pc {
            0x8011a860 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                if !c.crbit(2) { pc = 0x8011a894; continue; }
                pc = 0x8011a88c;
            }
            0x8011a88c => {
                c.r[3] = 0x0;
                pc = 0x8011a9cc; continue;
                pc = 0x8011a894;
            }
            0x8011a894 => {
                if !c.crbit(2) { pc = 0x8011a8a0; continue; }
                pc = 0x8011a898;
            }
            0x8011a898 => {
                c.r[0] = 0x0;
                pc = 0x8011a958; continue;
                pc = 0x8011a8a0;
            }
            0x8011a8a0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a8a8; f_80119e90(c);
                pc = 0x8011a8a8;
            }
            0x8011a8a8 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a8b4; f_80119f10(c);
                pc = 0x8011a8b4;
            }
            0x8011a8b4 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a8c0; f_80119ed0(c);
                pc = 0x8011a8c0;
            }
            0x8011a8c0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a8d8; f_80135e44(c);
                pc = 0x8011a8d8;
            }
            0x8011a8d8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a8e8; continue; }
                pc = 0x8011a8e0;
            }
            0x8011a8e0 => {
                c.r[28] = 0x0;
                pc = 0x8011a8f0; continue;
                pc = 0x8011a8e8;
            }
            0x8011a8e8 => {
                c.r[0] = c.r[29].rotate_left(4) & 0xffff0;
                c.r[28] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a8f0;
            }
            0x8011a8f0 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8011a900; continue; }
                pc = 0x8011a8f8;
            }
            0x8011a8f8 => {
                c.r[0] = 0x0;
                pc = 0x8011a958; continue;
                pc = 0x8011a900;
            }
            0x8011a900 => {
                if !c.crbit(2) { pc = 0x8011a90c; continue; }
                pc = 0x8011a904;
            }
            0x8011a904 => {
                c.r[0] = 0x0;
                pc = 0x8011a928; continue;
                pc = 0x8011a90c;
            }
            0x8011a90c => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011a914; f_8011a090(c);
                pc = 0x8011a914;
            }
            0x8011a914 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011a924; continue; }
                pc = 0x8011a91c;
            }
            0x8011a91c => {
                c.r[0] = 0x0;
                pc = 0x8011a928; continue;
                pc = 0x8011a924;
            }
            0x8011a924 => {
                c.r[0] = 0x1;
                pc = 0x8011a928;
            }
            0x8011a928 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011a954; continue; }
                pc = 0x8011a934;
            }
            0x8011a934 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011a93c; f_8011a090(c);
                pc = 0x8011a93c;
            }
            0x8011a93c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011a954; continue; }
                pc = 0x8011a94c;
            }
            0x8011a94c => {
                c.r[0] = 0x1;
                pc = 0x8011a958; continue;
                pc = 0x8011a954;
            }
            0x8011a954 => {
                c.r[0] = 0x0;
                pc = 0x8011a958;
            }
            0x8011a958 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011a968; continue; }
                pc = 0x8011a960;
            }
            0x8011a960 => {
                c.r[3] = 0x0;
                pc = 0x8011a9cc; continue;
                pc = 0x8011a968;
            }
            0x8011a968 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a970; f_80119e90(c);
                pc = 0x8011a970;
            }
            0x8011a970 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a97c; f_80119f10(c);
                pc = 0x8011a97c;
            }
            0x8011a97c => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011a988; f_80119ed0(c);
                pc = 0x8011a988;
            }
            0x8011a988 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011a9a0; f_80135e44(c);
                pc = 0x8011a9a0;
            }
            0x8011a9a0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a9b0; continue; }
                pc = 0x8011a9a8;
            }
            0x8011a9a8 => {
                c.r[3] = 0x0;
                pc = 0x8011a9b8; continue;
                pc = 0x8011a9b0;
            }
            0x8011a9b0 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011a9b8;
            }
            0x8011a9b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011a9c8; continue; }
                pc = 0x8011a9c0;
            }
            0x8011a9c0 => {
                c.r[3] = 0x0;
                pc = 0x8011a9cc; continue;
                pc = 0x8011a9c8;
            }
            0x8011a9c8 => {
                c.lr = 0x8011a9cc; f_8011a078(c);
                pc = 0x8011a9cc;
            }
            0x8011a9cc => {
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
            _ => panic!("f_8011a860: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011A9EC (0x8011a9ec, 356 bytes)
#[inline(never)]
pub fn f_8011a9ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011a9ec) { return; }
        if c.prof { crate::prof::enter(c, 0x8011a9ec); f_8011a9ec_b(c, 0x8011a9ec); crate::prof::exit(c, 0x8011a9ec); return; }
    }
    f_8011a9ec_b(c, 0x8011a9ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011a9ec_at(c: &mut Cpu, pc: u32) { f_8011a9ec_b(c, pc) }
#[inline(always)]
fn f_8011a9ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011a9ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011a9ec, pc); }
        match pc {
            0x8011a9ec => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                if c.crbit(2) { pc = 0x8011ab3c; continue; }
                pc = 0x8011aa10;
            }
            0x8011aa10 => {
                if !c.crbit(2) { pc = 0x8011aa1c; continue; }
                pc = 0x8011aa14;
            }
            0x8011aa14 => {
                c.r[0] = 0x0;
                pc = 0x8011aad4; continue;
                pc = 0x8011aa1c;
            }
            0x8011aa1c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011aa24; f_80119e90(c);
                pc = 0x8011aa24;
            }
            0x8011aa24 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011aa30; f_80119f10(c);
                pc = 0x8011aa30;
            }
            0x8011aa30 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011aa3c; f_80119ed0(c);
                pc = 0x8011aa3c;
            }
            0x8011aa3c => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011aa54; f_80135e44(c);
                pc = 0x8011aa54;
            }
            0x8011aa54 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011aa64; continue; }
                pc = 0x8011aa5c;
            }
            0x8011aa5c => {
                c.r[27] = 0x0;
                pc = 0x8011aa6c; continue;
                pc = 0x8011aa64;
            }
            0x8011aa64 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[27] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011aa6c;
            }
            0x8011aa6c => {
                c.cmpu(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x8011aa7c; continue; }
                pc = 0x8011aa74;
            }
            0x8011aa74 => {
                c.r[0] = 0x0;
                pc = 0x8011aad4; continue;
                pc = 0x8011aa7c;
            }
            0x8011aa7c => {
                if !c.crbit(2) { pc = 0x8011aa88; continue; }
                pc = 0x8011aa80;
            }
            0x8011aa80 => {
                c.r[0] = 0x0;
                pc = 0x8011aaa4; continue;
                pc = 0x8011aa88;
            }
            0x8011aa88 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011aa90; f_8011a090(c);
                pc = 0x8011aa90;
            }
            0x8011aa90 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011aaa0; continue; }
                pc = 0x8011aa98;
            }
            0x8011aa98 => {
                c.r[0] = 0x0;
                pc = 0x8011aaa4; continue;
                pc = 0x8011aaa0;
            }
            0x8011aaa0 => {
                c.r[0] = 0x1;
                pc = 0x8011aaa4;
            }
            0x8011aaa4 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011aad0; continue; }
                pc = 0x8011aab0;
            }
            0x8011aab0 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011aab8; f_8011a090(c);
                pc = 0x8011aab8;
            }
            0x8011aab8 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011aad0; continue; }
                pc = 0x8011aac8;
            }
            0x8011aac8 => {
                c.r[0] = 0x1;
                pc = 0x8011aad4; continue;
                pc = 0x8011aad0;
            }
            0x8011aad0 => {
                c.r[0] = 0x0;
                pc = 0x8011aad4;
            }
            0x8011aad4 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8011ab3c; continue; }
                pc = 0x8011aadc;
            }
            0x8011aadc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011aae4; f_80119e90(c);
                pc = 0x8011aae4;
            }
            0x8011aae4 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011aaf0; f_80119f10(c);
                pc = 0x8011aaf0;
            }
            0x8011aaf0 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011aafc; f_80119ed0(c);
                pc = 0x8011aafc;
            }
            0x8011aafc => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011ab14; f_80135e44(c);
                pc = 0x8011ab14;
            }
            0x8011ab14 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ab24; continue; }
                pc = 0x8011ab1c;
            }
            0x8011ab1c => {
                c.r[3] = 0x0;
                pc = 0x8011ab2c; continue;
                pc = 0x8011ab24;
            }
            0x8011ab24 => {
                c.r[0] = c.r[27].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011ab2c;
            }
            0x8011ab2c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8011ab3c; continue; }
                pc = 0x8011ab34;
            }
            0x8011ab34 => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8011ab3c; f_80119fd0(c);
                pc = 0x8011ab3c;
            }
            0x8011ab3c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8011a9ec: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011AB50 (0x8011ab50, 356 bytes)
#[inline(never)]
pub fn f_8011ab50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011ab50) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ab50); f_8011ab50_b(c, 0x8011ab50); crate::prof::exit(c, 0x8011ab50); return; }
    }
    f_8011ab50_b(c, 0x8011ab50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ab50_at(c: &mut Cpu, pc: u32) { f_8011ab50_b(c, pc) }
#[inline(always)]
fn f_8011ab50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ab50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ab50, pc); }
        match pc {
            0x8011ab50 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                if c.crbit(2) { pc = 0x8011aca0; continue; }
                pc = 0x8011ab74;
            }
            0x8011ab74 => {
                if !c.crbit(2) { pc = 0x8011ab80; continue; }
                pc = 0x8011ab78;
            }
            0x8011ab78 => {
                c.r[0] = 0x0;
                pc = 0x8011ac38; continue;
                pc = 0x8011ab80;
            }
            0x8011ab80 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011ab88; f_80119e90(c);
                pc = 0x8011ab88;
            }
            0x8011ab88 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011ab94; f_80119f10(c);
                pc = 0x8011ab94;
            }
            0x8011ab94 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011aba0; f_80119ed0(c);
                pc = 0x8011aba0;
            }
            0x8011aba0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011abb8; f_80135e44(c);
                pc = 0x8011abb8;
            }
            0x8011abb8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011abc8; continue; }
                pc = 0x8011abc0;
            }
            0x8011abc0 => {
                c.r[27] = 0x0;
                pc = 0x8011abd0; continue;
                pc = 0x8011abc8;
            }
            0x8011abc8 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[27] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011abd0;
            }
            0x8011abd0 => {
                c.cmpu(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x8011abe0; continue; }
                pc = 0x8011abd8;
            }
            0x8011abd8 => {
                c.r[0] = 0x0;
                pc = 0x8011ac38; continue;
                pc = 0x8011abe0;
            }
            0x8011abe0 => {
                if !c.crbit(2) { pc = 0x8011abec; continue; }
                pc = 0x8011abe4;
            }
            0x8011abe4 => {
                c.r[0] = 0x0;
                pc = 0x8011ac08; continue;
                pc = 0x8011abec;
            }
            0x8011abec => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011abf4; f_8011a090(c);
                pc = 0x8011abf4;
            }
            0x8011abf4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011ac04; continue; }
                pc = 0x8011abfc;
            }
            0x8011abfc => {
                c.r[0] = 0x0;
                pc = 0x8011ac08; continue;
                pc = 0x8011ac04;
            }
            0x8011ac04 => {
                c.r[0] = 0x1;
                pc = 0x8011ac08;
            }
            0x8011ac08 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011ac34; continue; }
                pc = 0x8011ac14;
            }
            0x8011ac14 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011ac1c; f_8011a090(c);
                pc = 0x8011ac1c;
            }
            0x8011ac1c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011ac34; continue; }
                pc = 0x8011ac2c;
            }
            0x8011ac2c => {
                c.r[0] = 0x1;
                pc = 0x8011ac38; continue;
                pc = 0x8011ac34;
            }
            0x8011ac34 => {
                c.r[0] = 0x0;
                pc = 0x8011ac38;
            }
            0x8011ac38 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8011aca0; continue; }
                pc = 0x8011ac40;
            }
            0x8011ac40 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011ac48; f_80119e90(c);
                pc = 0x8011ac48;
            }
            0x8011ac48 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011ac54; f_80119f10(c);
                pc = 0x8011ac54;
            }
            0x8011ac54 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011ac60; f_80119ed0(c);
                pc = 0x8011ac60;
            }
            0x8011ac60 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011ac78; f_80135e44(c);
                pc = 0x8011ac78;
            }
            0x8011ac78 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ac88; continue; }
                pc = 0x8011ac80;
            }
            0x8011ac80 => {
                c.r[3] = 0x0;
                pc = 0x8011ac90; continue;
                pc = 0x8011ac88;
            }
            0x8011ac88 => {
                c.r[0] = c.r[27].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011ac90;
            }
            0x8011ac90 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8011aca0; continue; }
                pc = 0x8011ac98;
            }
            0x8011ac98 => {
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x8011aca0; f_80119fc0(c);
                pc = 0x8011aca0;
            }
            0x8011aca0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8011ab50: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011ACB4 (0x8011acb4, 396 bytes)
#[inline(never)]
pub fn f_8011acb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011acb4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011acb4); f_8011acb4_b(c, 0x8011acb4); crate::prof::exit(c, 0x8011acb4); return; }
    }
    f_8011acb4_b(c, 0x8011acb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011acb4_at(c: &mut Cpu, pc: u32) { f_8011acb4_b(c, pc) }
#[inline(always)]
fn f_8011acb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011acb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011acb4, pc); }
        match pc {
            0x8011acb4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                if !c.crbit(2) { pc = 0x8011ace8; continue; }
                pc = 0x8011ace0;
            }
            0x8011ace0 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011ae20; continue;
                pc = 0x8011ace8;
            }
            0x8011ace8 => {
                if !c.crbit(2) { pc = 0x8011acf4; continue; }
                pc = 0x8011acec;
            }
            0x8011acec => {
                c.r[0] = 0x0;
                pc = 0x8011adac; continue;
                pc = 0x8011acf4;
            }
            0x8011acf4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011acfc; f_80119e90(c);
                pc = 0x8011acfc;
            }
            0x8011acfc => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011ad08; f_80119f10(c);
                pc = 0x8011ad08;
            }
            0x8011ad08 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011ad14; f_80119ed0(c);
                pc = 0x8011ad14;
            }
            0x8011ad14 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011ad2c; f_80135e44(c);
                pc = 0x8011ad2c;
            }
            0x8011ad2c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ad3c; continue; }
                pc = 0x8011ad34;
            }
            0x8011ad34 => {
                c.r[28] = 0x0;
                pc = 0x8011ad44; continue;
                pc = 0x8011ad3c;
            }
            0x8011ad3c => {
                c.r[0] = c.r[29].rotate_left(4) & 0xffff0;
                c.r[28] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011ad44;
            }
            0x8011ad44 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8011ad54; continue; }
                pc = 0x8011ad4c;
            }
            0x8011ad4c => {
                c.r[0] = 0x0;
                pc = 0x8011adac; continue;
                pc = 0x8011ad54;
            }
            0x8011ad54 => {
                if !c.crbit(2) { pc = 0x8011ad60; continue; }
                pc = 0x8011ad58;
            }
            0x8011ad58 => {
                c.r[0] = 0x0;
                pc = 0x8011ad7c; continue;
                pc = 0x8011ad60;
            }
            0x8011ad60 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011ad68; f_8011a090(c);
                pc = 0x8011ad68;
            }
            0x8011ad68 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011ad78; continue; }
                pc = 0x8011ad70;
            }
            0x8011ad70 => {
                c.r[0] = 0x0;
                pc = 0x8011ad7c; continue;
                pc = 0x8011ad78;
            }
            0x8011ad78 => {
                c.r[0] = 0x1;
                pc = 0x8011ad7c;
            }
            0x8011ad7c => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011ada8; continue; }
                pc = 0x8011ad88;
            }
            0x8011ad88 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011ad90; f_8011a090(c);
                pc = 0x8011ad90;
            }
            0x8011ad90 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011ada8; continue; }
                pc = 0x8011ada0;
            }
            0x8011ada0 => {
                c.r[0] = 0x1;
                pc = 0x8011adac; continue;
                pc = 0x8011ada8;
            }
            0x8011ada8 => {
                c.r[0] = 0x0;
                pc = 0x8011adac;
            }
            0x8011adac => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011adbc; continue; }
                pc = 0x8011adb4;
            }
            0x8011adb4 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011ae20; continue;
                pc = 0x8011adbc;
            }
            0x8011adbc => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011adc4; f_80119e90(c);
                pc = 0x8011adc4;
            }
            0x8011adc4 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011add0; f_80119f10(c);
                pc = 0x8011add0;
            }
            0x8011add0 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011addc; f_80119ed0(c);
                pc = 0x8011addc;
            }
            0x8011addc => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011adf4; f_80135e44(c);
                pc = 0x8011adf4;
            }
            0x8011adf4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ae04; continue; }
                pc = 0x8011adfc;
            }
            0x8011adfc => {
                c.r[3] = 0x0;
                pc = 0x8011ae0c; continue;
                pc = 0x8011ae04;
            }
            0x8011ae04 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011ae0c;
            }
            0x8011ae0c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ae1c; continue; }
                pc = 0x8011ae14;
            }
            0x8011ae14 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011ae20; continue;
                pc = 0x8011ae1c;
            }
            0x8011ae1c => {
                c.lr = 0x8011ae20; f_8011a048(c);
                pc = 0x8011ae20;
            }
            0x8011ae20 => {
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
            _ => panic!("f_8011acb4: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011AE40 (0x8011ae40, 396 bytes)
#[inline(never)]
pub fn f_8011ae40(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011ae40) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ae40); f_8011ae40_b(c, 0x8011ae40); crate::prof::exit(c, 0x8011ae40); return; }
    }
    f_8011ae40_b(c, 0x8011ae40)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ae40_at(c: &mut Cpu, pc: u32) { f_8011ae40_b(c, pc) }
#[inline(always)]
fn f_8011ae40_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ae40;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ae40, pc); }
        match pc {
            0x8011ae40 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                if !c.crbit(2) { pc = 0x8011ae74; continue; }
                pc = 0x8011ae6c;
            }
            0x8011ae6c => {
                c.r[3] = 0xffffffff;
                pc = 0x8011afac; continue;
                pc = 0x8011ae74;
            }
            0x8011ae74 => {
                if !c.crbit(2) { pc = 0x8011ae80; continue; }
                pc = 0x8011ae78;
            }
            0x8011ae78 => {
                c.r[0] = 0x0;
                pc = 0x8011af38; continue;
                pc = 0x8011ae80;
            }
            0x8011ae80 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011ae88; f_80119e90(c);
                pc = 0x8011ae88;
            }
            0x8011ae88 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011ae94; f_80119f10(c);
                pc = 0x8011ae94;
            }
            0x8011ae94 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011aea0; f_80119ed0(c);
                pc = 0x8011aea0;
            }
            0x8011aea0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011aeb8; f_80135e44(c);
                pc = 0x8011aeb8;
            }
            0x8011aeb8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011aec8; continue; }
                pc = 0x8011aec0;
            }
            0x8011aec0 => {
                c.r[28] = 0x0;
                pc = 0x8011aed0; continue;
                pc = 0x8011aec8;
            }
            0x8011aec8 => {
                c.r[0] = c.r[29].rotate_left(4) & 0xffff0;
                c.r[28] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011aed0;
            }
            0x8011aed0 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8011aee0; continue; }
                pc = 0x8011aed8;
            }
            0x8011aed8 => {
                c.r[0] = 0x0;
                pc = 0x8011af38; continue;
                pc = 0x8011aee0;
            }
            0x8011aee0 => {
                if !c.crbit(2) { pc = 0x8011aeec; continue; }
                pc = 0x8011aee4;
            }
            0x8011aee4 => {
                c.r[0] = 0x0;
                pc = 0x8011af08; continue;
                pc = 0x8011aeec;
            }
            0x8011aeec => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011aef4; f_8011a090(c);
                pc = 0x8011aef4;
            }
            0x8011aef4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011af04; continue; }
                pc = 0x8011aefc;
            }
            0x8011aefc => {
                c.r[0] = 0x0;
                pc = 0x8011af08; continue;
                pc = 0x8011af04;
            }
            0x8011af04 => {
                c.r[0] = 0x1;
                pc = 0x8011af08;
            }
            0x8011af08 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011af34; continue; }
                pc = 0x8011af14;
            }
            0x8011af14 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011af1c; f_8011a090(c);
                pc = 0x8011af1c;
            }
            0x8011af1c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011af34; continue; }
                pc = 0x8011af2c;
            }
            0x8011af2c => {
                c.r[0] = 0x1;
                pc = 0x8011af38; continue;
                pc = 0x8011af34;
            }
            0x8011af34 => {
                c.r[0] = 0x0;
                pc = 0x8011af38;
            }
            0x8011af38 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011af48; continue; }
                pc = 0x8011af40;
            }
            0x8011af40 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011afac; continue;
                pc = 0x8011af48;
            }
            0x8011af48 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011af50; f_80119e90(c);
                pc = 0x8011af50;
            }
            0x8011af50 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011af5c; f_80119f10(c);
                pc = 0x8011af5c;
            }
            0x8011af5c => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011af68; f_80119ed0(c);
                pc = 0x8011af68;
            }
            0x8011af68 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011af80; f_80135e44(c);
                pc = 0x8011af80;
            }
            0x8011af80 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011af90; continue; }
                pc = 0x8011af88;
            }
            0x8011af88 => {
                c.r[3] = 0x0;
                pc = 0x8011af98; continue;
                pc = 0x8011af90;
            }
            0x8011af90 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011af98;
            }
            0x8011af98 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011afa8; continue; }
                pc = 0x8011afa0;
            }
            0x8011afa0 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011afac; continue;
                pc = 0x8011afa8;
            }
            0x8011afa8 => {
                c.lr = 0x8011afac; f_8011a060(c);
                pc = 0x8011afac;
            }
            0x8011afac => {
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
            _ => panic!("f_8011ae40: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011AFCC (0x8011afcc, 356 bytes)
#[inline(never)]
pub fn f_8011afcc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011afcc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011afcc); f_8011afcc_b(c, 0x8011afcc); crate::prof::exit(c, 0x8011afcc); return; }
    }
    f_8011afcc_b(c, 0x8011afcc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011afcc_at(c: &mut Cpu, pc: u32) { f_8011afcc_b(c, pc) }
#[inline(always)]
fn f_8011afcc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011afcc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011afcc, pc); }
        match pc {
            0x8011afcc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[29] = c.r[3] | c.r[3];
                c.r[31] = c.r[5] | c.r[5];
                if c.crbit(2) { pc = 0x8011b11c; continue; }
                pc = 0x8011aff0;
            }
            0x8011aff0 => {
                if !c.crbit(2) { pc = 0x8011affc; continue; }
                pc = 0x8011aff4;
            }
            0x8011aff4 => {
                c.r[0] = 0x0;
                pc = 0x8011b0b4; continue;
                pc = 0x8011affc;
            }
            0x8011affc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011b004; f_80119e90(c);
                pc = 0x8011b004;
            }
            0x8011b004 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011b010; f_80119f10(c);
                pc = 0x8011b010;
            }
            0x8011b010 => {
                c.r[27] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011b01c; f_80119ed0(c);
                pc = 0x8011b01c;
            }
            0x8011b01c => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011b034; f_80135e44(c);
                pc = 0x8011b034;
            }
            0x8011b034 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011b044; continue; }
                pc = 0x8011b03c;
            }
            0x8011b03c => {
                c.r[27] = 0x0;
                pc = 0x8011b04c; continue;
                pc = 0x8011b044;
            }
            0x8011b044 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[27] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011b04c;
            }
            0x8011b04c => {
                c.cmpu(0, c.r[27], 0x0);
                if !c.crbit(2) { pc = 0x8011b05c; continue; }
                pc = 0x8011b054;
            }
            0x8011b054 => {
                c.r[0] = 0x0;
                pc = 0x8011b0b4; continue;
                pc = 0x8011b05c;
            }
            0x8011b05c => {
                if !c.crbit(2) { pc = 0x8011b068; continue; }
                pc = 0x8011b060;
            }
            0x8011b060 => {
                c.r[0] = 0x0;
                pc = 0x8011b084; continue;
                pc = 0x8011b068;
            }
            0x8011b068 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011b070; f_8011a090(c);
                pc = 0x8011b070;
            }
            0x8011b070 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011b080; continue; }
                pc = 0x8011b078;
            }
            0x8011b078 => {
                c.r[0] = 0x0;
                pc = 0x8011b084; continue;
                pc = 0x8011b080;
            }
            0x8011b080 => {
                c.r[0] = 0x1;
                pc = 0x8011b084;
            }
            0x8011b084 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011b0b0; continue; }
                pc = 0x8011b090;
            }
            0x8011b090 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011b098; f_8011a090(c);
                pc = 0x8011b098;
            }
            0x8011b098 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011b0b0; continue; }
                pc = 0x8011b0a8;
            }
            0x8011b0a8 => {
                c.r[0] = 0x1;
                pc = 0x8011b0b4; continue;
                pc = 0x8011b0b0;
            }
            0x8011b0b0 => {
                c.r[0] = 0x0;
                pc = 0x8011b0b4;
            }
            0x8011b0b4 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8011b11c; continue; }
                pc = 0x8011b0bc;
            }
            0x8011b0bc => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011b0c4; f_80119e90(c);
                pc = 0x8011b0c4;
            }
            0x8011b0c4 => {
                c.r[27] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011b0d0; f_80119f10(c);
                pc = 0x8011b0d0;
            }
            0x8011b0d0 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011b0dc; f_80119ed0(c);
                pc = 0x8011b0dc;
            }
            0x8011b0dc => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011b0f4; f_80135e44(c);
                pc = 0x8011b0f4;
            }
            0x8011b0f4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011b104; continue; }
                pc = 0x8011b0fc;
            }
            0x8011b0fc => {
                c.r[3] = 0x0;
                pc = 0x8011b10c; continue;
                pc = 0x8011b104;
            }
            0x8011b104 => {
                c.r[0] = c.r[27].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011b10c;
            }
            0x8011b10c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8011b11c; continue; }
                pc = 0x8011b114;
            }
            0x8011b114 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.lr = 0x8011b11c; f_80119fb0(c);
                pc = 0x8011b11c;
            }
            0x8011b11c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8011afcc: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011B130 (0x8011b130, 400 bytes)
#[inline(never)]
pub fn f_8011b130(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011b130) { return; }
        if c.prof { crate::prof::enter(c, 0x8011b130); f_8011b130_b(c, 0x8011b130); crate::prof::exit(c, 0x8011b130); return; }
    }
    f_8011b130_b(c, 0x8011b130)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011b130_at(c: &mut Cpu, pc: u32) { f_8011b130_b(c, pc) }
#[inline(always)]
fn f_8011b130_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011b130;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011b130, pc); }
        match pc {
            0x8011b130 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                if !c.crbit(2) { pc = 0x8011b164; continue; }
                pc = 0x8011b15c;
            }
            0x8011b15c => {
                c.r[3] = 0xffffffff;
                pc = 0x8011b2a0; continue;
                pc = 0x8011b164;
            }
            0x8011b164 => {
                if !c.crbit(2) { pc = 0x8011b170; continue; }
                pc = 0x8011b168;
            }
            0x8011b168 => {
                c.r[0] = 0x0;
                pc = 0x8011b228; continue;
                pc = 0x8011b170;
            }
            0x8011b170 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b178; f_80119e90(c);
                pc = 0x8011b178;
            }
            0x8011b178 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b184; f_80119f10(c);
                pc = 0x8011b184;
            }
            0x8011b184 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b190; f_80119ed0(c);
                pc = 0x8011b190;
            }
            0x8011b190 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011b1a8; f_80135e44(c);
                pc = 0x8011b1a8;
            }
            0x8011b1a8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011b1b8; continue; }
                pc = 0x8011b1b0;
            }
            0x8011b1b0 => {
                c.r[28] = 0x0;
                pc = 0x8011b1c0; continue;
                pc = 0x8011b1b8;
            }
            0x8011b1b8 => {
                c.r[0] = c.r[29].rotate_left(4) & 0xffff0;
                c.r[28] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011b1c0;
            }
            0x8011b1c0 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8011b1d0; continue; }
                pc = 0x8011b1c8;
            }
            0x8011b1c8 => {
                c.r[0] = 0x0;
                pc = 0x8011b228; continue;
                pc = 0x8011b1d0;
            }
            0x8011b1d0 => {
                if !c.crbit(2) { pc = 0x8011b1dc; continue; }
                pc = 0x8011b1d4;
            }
            0x8011b1d4 => {
                c.r[0] = 0x0;
                pc = 0x8011b1f8; continue;
                pc = 0x8011b1dc;
            }
            0x8011b1dc => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011b1e4; f_8011a090(c);
                pc = 0x8011b1e4;
            }
            0x8011b1e4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011b1f4; continue; }
                pc = 0x8011b1ec;
            }
            0x8011b1ec => {
                c.r[0] = 0x0;
                pc = 0x8011b1f8; continue;
                pc = 0x8011b1f4;
            }
            0x8011b1f4 => {
                c.r[0] = 0x1;
                pc = 0x8011b1f8;
            }
            0x8011b1f8 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011b224; continue; }
                pc = 0x8011b204;
            }
            0x8011b204 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011b20c; f_8011a090(c);
                pc = 0x8011b20c;
            }
            0x8011b20c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011b224; continue; }
                pc = 0x8011b21c;
            }
            0x8011b21c => {
                c.r[0] = 0x1;
                pc = 0x8011b228; continue;
                pc = 0x8011b224;
            }
            0x8011b224 => {
                c.r[0] = 0x0;
                pc = 0x8011b228;
            }
            0x8011b228 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011b238; continue; }
                pc = 0x8011b230;
            }
            0x8011b230 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011b2a0; continue;
                pc = 0x8011b238;
            }
            0x8011b238 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b240; f_80119e90(c);
                pc = 0x8011b240;
            }
            0x8011b240 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b24c; f_80119f10(c);
                pc = 0x8011b24c;
            }
            0x8011b24c => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b258; f_80119ed0(c);
                pc = 0x8011b258;
            }
            0x8011b258 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011b270; f_80135e44(c);
                pc = 0x8011b270;
            }
            0x8011b270 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011b280; continue; }
                pc = 0x8011b278;
            }
            0x8011b278 => {
                c.r[3] = 0x0;
                pc = 0x8011b288; continue;
                pc = 0x8011b280;
            }
            0x8011b280 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011b288;
            }
            0x8011b288 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011b298; continue; }
                pc = 0x8011b290;
            }
            0x8011b290 => {
                c.r[3] = 0xffffffff;
                pc = 0x8011b2a0; continue;
                pc = 0x8011b298;
            }
            0x8011b298 => {
                c.lr = 0x8011b29c; f_8011a030(c);
                pc = 0x8011b29c;
            }
            0x8011b29c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011b2a0;
            }
            0x8011b2a0 => {
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
            _ => panic!("f_8011b130: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011B2C0 (0x8011b2c0, 388 bytes)
#[inline(never)]
pub fn f_8011b2c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011b2c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011b2c0); f_8011b2c0_b(c, 0x8011b2c0); crate::prof::exit(c, 0x8011b2c0); return; }
    }
    f_8011b2c0_b(c, 0x8011b2c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011b2c0_at(c: &mut Cpu, pc: u32) { f_8011b2c0_b(c, pc) }
#[inline(always)]
fn f_8011b2c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011b2c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011b2c0, pc); }
        match pc {
            0x8011b2c0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[4] | c.r[4];
                c.r[31] = c.r[3] | c.r[3];
                c.r[28] = c.r[5] | c.r[5];
                if c.crbit(2) { pc = 0x8011b430; continue; }
                pc = 0x8011b2e4;
            }
            0x8011b2e4 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011b2ec; f_80119e90(c);
                pc = 0x8011b2ec;
            }
            0x8011b2ec => {
                c.r[30] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011b2f8; f_80119f10(c);
                pc = 0x8011b2f8;
            }
            0x8011b2f8 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011b304; f_80119ed0(c);
                pc = 0x8011b304;
            }
            0x8011b304 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011b31c; f_80135e44(c);
                pc = 0x8011b31c;
            }
            0x8011b31c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011b32c; continue; }
                pc = 0x8011b324;
            }
            0x8011b324 => {
                c.r[29] = 0x0;
                pc = 0x8011b334; continue;
                pc = 0x8011b32c;
            }
            0x8011b32c => {
                c.r[0] = c.r[30].rotate_left(4) & 0xffff0;
                c.r[29] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011b334;
            }
            0x8011b334 => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x8011b430; continue; }
                pc = 0x8011b33c;
            }
            0x8011b33c => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011b344; f_80119e50(c);
                pc = 0x8011b344;
            }
            0x8011b344 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if !c.crbit(2) { pc = 0x8011b364; continue; }
                pc = 0x8011b354;
            }
            0x8011b354 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8011b35c; f_8011a030(c);
                pc = 0x8011b35c;
            }
            0x8011b35c => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8011b368; continue;
                pc = 0x8011b364;
            }
            0x8011b364 => {
                c.r[31] = 0x0;
                pc = 0x8011b368;
            }
            0x8011b368 => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x8011b3c4; continue; }
                pc = 0x8011b370;
            }
            0x8011b370 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8011b37c; f_80119ff0(c);
                pc = 0x8011b37c;
            }
            0x8011b37c => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8011b388; f_80119fe0(c);
                pc = 0x8011b388;
            }
            0x8011b388 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8011b394; f_80119fd0(c);
                pc = 0x8011b394;
            }
            0x8011b394 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8011b3a0; f_80119fc0(c);
                pc = 0x8011b3a0;
            }
            0x8011b3a0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8011b3ac; f_80119fb0(c);
                pc = 0x8011b3ac;
            }
            0x8011b3ac => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8011b3b8; f_80119fa0(c);
                pc = 0x8011b3b8;
            }
            0x8011b3b8 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.lr = 0x8011b3c4; f_80119f90(c);
                pc = 0x8011b3c4;
            }
            0x8011b3c4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[27] | c.r[27];
                c.lr = 0x8011b3d0; f_80119ff0(c);
                pc = 0x8011b3d0;
            }
            0x8011b3d0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8011b3dc; f_80119fe0(c);
                pc = 0x8011b3dc;
            }
            0x8011b3dc => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8011b3e4; f_80119d90(c);
                pc = 0x8011b3e4;
            }
            0x8011b3e4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.lr = 0x8011b3ec; f_80101ac4(c);
                pc = 0x8011b3ec;
            }
            0x8011b3ec => {
                c.r[4] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8011b3f8; f_80119fd0(c);
                pc = 0x8011b3f8;
            }
            0x8011b3f8 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if !c.crbit(2) { pc = 0x8011b430; continue; }
                pc = 0x8011b404;
            }
            0x8011b404 => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[30] = c.r[31].wrapping_add(0x1);
                c.lr = 0x8011b410; f_80119dd0(c);
                pc = 0x8011b410;
            }
            0x8011b410 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(0) { pc = 0x8011b424; continue; }
                pc = 0x8011b420;
            }
            0x8011b420 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x8011b424;
            }
            0x8011b424 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x8011b430; f_80119fb0(c);
                pc = 0x8011b430;
            }
            0x8011b430 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8011b2c0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011B444 (0x8011b444, 568 bytes)
#[inline(never)]
pub fn f_8011b444(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011b444) { return; }
        if c.prof { crate::prof::enter(c, 0x8011b444); f_8011b444_b(c, 0x8011b444); crate::prof::exit(c, 0x8011b444); return; }
    }
    f_8011b444_b(c, 0x8011b444)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011b444_at(c: &mut Cpu, pc: u32) { f_8011b444_b(c, pc) }
#[inline(always)]
fn f_8011b444_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011b444;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011b444, pc); }
        match pc {
            0x8011b444 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x8011b478; continue; }
                pc = 0x8011b470;
            }
            0x8011b470 => {
                c.r[3] = 0x1;
                pc = 0x8011b65c; continue;
                pc = 0x8011b478;
            }
            0x8011b478 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b480; f_80119e90(c);
                pc = 0x8011b480;
            }
            0x8011b480 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b48c; f_80119f10(c);
                pc = 0x8011b48c;
            }
            0x8011b48c => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b498; f_80119ed0(c);
                pc = 0x8011b498;
            }
            0x8011b498 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011b4b0; f_80135e44(c);
                pc = 0x8011b4b0;
            }
            0x8011b4b0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011b4c0; continue; }
                pc = 0x8011b4b8;
            }
            0x8011b4b8 => {
                c.r[30] = 0x0;
                pc = 0x8011b4c8; continue;
                pc = 0x8011b4c0;
            }
            0x8011b4c0 => {
                c.r[0] = c.r[29].rotate_left(4) & 0xffff0;
                c.r[30] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011b4c8;
            }
            0x8011b4c8 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8011b4d8; continue; }
                pc = 0x8011b4d0;
            }
            0x8011b4d0 => {
                c.r[3] = 0x0;
                pc = 0x8011b65c; continue;
                pc = 0x8011b4d8;
            }
            0x8011b4d8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011b4e0; f_8011a090(c);
                pc = 0x8011b4e0;
            }
            0x8011b4e0 => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[29] = c.r[0] | c.r[0];
                c.lr = 0x8011b4f0; f_80119e50(c);
                pc = 0x8011b4f0;
            }
            0x8011b4f0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x8011b530; continue; }
                pc = 0x8011b4fc;
            }
            0x8011b4fc => {
                if !c.crbit(0) { pc = 0x8011b510; continue; }
                pc = 0x8011b500;
            }
            0x8011b500 => {
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8011b520; continue; }
                pc = 0x8011b508;
            }
            0x8011b508 => {
                if !c.crbit(0) { pc = 0x8011b528; continue; }
                pc = 0x8011b50c;
            }
            0x8011b50c => {
                pc = 0x8011b658; continue;
                pc = 0x8011b510;
            }
            0x8011b510 => {
                c.cmps(0, c.r[0], 0x4);
                if c.crbit(2) { pc = 0x8011b5e4; continue; }
                pc = 0x8011b518;
            }
            0x8011b518 => {
                if !c.crbit(0) { pc = 0x8011b658; continue; }
                pc = 0x8011b51c;
            }
            0x8011b51c => {
                pc = 0x8011b570; continue;
                pc = 0x8011b520;
            }
            0x8011b520 => {
                c.r[3] = 0x2;
                pc = 0x8011b65c; continue;
                pc = 0x8011b528;
            }
            0x8011b528 => {
                c.r[3] = 0x2;
                pc = 0x8011b65c; continue;
                pc = 0x8011b530;
            }
            0x8011b530 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8011b540; continue; }
                pc = 0x8011b538;
            }
            0x8011b538 => {
                c.r[0] = 0x0;
                pc = 0x8011b55c; continue;
                pc = 0x8011b540;
            }
            0x8011b540 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011b548; f_8011a090(c);
                pc = 0x8011b548;
            }
            0x8011b548 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011b558; continue; }
                pc = 0x8011b550;
            }
            0x8011b550 => {
                c.r[0] = 0x0;
                pc = 0x8011b55c; continue;
                pc = 0x8011b558;
            }
            0x8011b558 => {
                c.r[0] = 0x1;
                pc = 0x8011b55c;
            }
            0x8011b55c => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.r[3] = c.r[3].wrapping_add(0x1);
                pc = 0x8011b65c; continue;
                pc = 0x8011b570;
            }
            0x8011b570 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8011b580; continue; }
                pc = 0x8011b578;
            }
            0x8011b578 => {
                c.r[0] = 0x0;
                pc = 0x8011b59c; continue;
                pc = 0x8011b580;
            }
            0x8011b580 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011b588; f_8011a090(c);
                pc = 0x8011b588;
            }
            0x8011b588 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011b598; continue; }
                pc = 0x8011b590;
            }
            0x8011b590 => {
                c.r[0] = 0x0;
                pc = 0x8011b59c; continue;
                pc = 0x8011b598;
            }
            0x8011b598 => {
                c.r[0] = 0x1;
                pc = 0x8011b59c;
            }
            0x8011b59c => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011b5ac; continue; }
                pc = 0x8011b5a4;
            }
            0x8011b5a4 => {
                c.r[3] = 0x2;
                pc = 0x8011b65c; continue;
                pc = 0x8011b5ac;
            }
            0x8011b5ac => {
                c.r[3] = c.r[29].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x8011b5dc; continue; }
                pc = 0x8011b5bc;
            }
            0x8011b5bc => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8011b5c4; f_80119e10(c);
                pc = 0x8011b5c4;
            }
            0x8011b5c4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if c.crbit(2) { pc = 0x8011b5dc; continue; }
                pc = 0x8011b5d4;
            }
            0x8011b5d4 => {
                c.r[3] = 0x2;
                pc = 0x8011b65c; continue;
                pc = 0x8011b5dc;
            }
            0x8011b5dc => {
                c.r[3] = 0x1;
                pc = 0x8011b65c; continue;
                pc = 0x8011b5e4;
            }
            0x8011b5e4 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x8011b5f4; continue; }
                pc = 0x8011b5ec;
            }
            0x8011b5ec => {
                c.r[0] = 0x0;
                pc = 0x8011b610; continue;
                pc = 0x8011b5f4;
            }
            0x8011b5f4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8011b5fc; f_8011a090(c);
                pc = 0x8011b5fc;
            }
            0x8011b5fc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011b60c; continue; }
                pc = 0x8011b604;
            }
            0x8011b604 => {
                c.r[0] = 0x0;
                pc = 0x8011b610; continue;
                pc = 0x8011b60c;
            }
            0x8011b60c => {
                c.r[0] = 0x1;
                pc = 0x8011b610;
            }
            0x8011b610 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011b620; continue; }
                pc = 0x8011b618;
            }
            0x8011b618 => {
                c.r[3] = 0x2;
                pc = 0x8011b65c; continue;
                pc = 0x8011b620;
            }
            0x8011b620 => {
                c.r[3] = c.r[29].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(2) { pc = 0x8011b648; continue; }
                pc = 0x8011b630;
            }
            0x8011b630 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8011b638; f_80119e10(c);
                pc = 0x8011b638;
            }
            0x8011b638 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011b650; continue; }
                pc = 0x8011b648;
            }
            0x8011b648 => {
                c.r[3] = 0x2;
                pc = 0x8011b65c; continue;
                pc = 0x8011b650;
            }
            0x8011b650 => {
                c.r[3] = 0x1;
                pc = 0x8011b65c; continue;
                pc = 0x8011b658;
            }
            0x8011b658 => {
                c.r[3] = 0x0;
                pc = 0x8011b65c;
            }
            0x8011b65c => {
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
            _ => panic!("f_8011b444: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011B67C (0x8011b67c, 268 bytes)
#[inline(never)]
pub fn f_8011b67c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011b67c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011b67c); f_8011b67c_b(c, 0x8011b67c); crate::prof::exit(c, 0x8011b67c); return; }
    }
    f_8011b67c_b(c, 0x8011b67c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011b67c_at(c: &mut Cpu, pc: u32) { f_8011b67c_b(c, pc) }
#[inline(always)]
fn f_8011b67c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011b67c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011b67c, pc); }
        match pc {
            0x8011b67c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x8011b6b0; continue; }
                pc = 0x8011b6a8;
            }
            0x8011b6a8 => {
                c.r[3] = 0x0;
                pc = 0x8011b768; continue;
                pc = 0x8011b6b0;
            }
            0x8011b6b0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8011b6b8; f_80119e90(c);
                pc = 0x8011b6b8;
            }
            0x8011b6b8 => {
                c.r[30] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8011b6c4; f_80119f10(c);
                pc = 0x8011b6c4;
            }
            0x8011b6c4 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x8011b6d0; f_80119ed0(c);
                pc = 0x8011b6d0;
            }
            0x8011b6d0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011b6e8; f_80135e44(c);
                pc = 0x8011b6e8;
            }
            0x8011b6e8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011b6f8; continue; }
                pc = 0x8011b6f0;
            }
            0x8011b6f0 => {
                c.r[31] = 0x0;
                pc = 0x8011b700; continue;
                pc = 0x8011b6f8;
            }
            0x8011b6f8 => {
                c.r[0] = c.r[30].rotate_left(4) & 0xffff0;
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011b700;
            }
            0x8011b700 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x8011b710; continue; }
                pc = 0x8011b708;
            }
            0x8011b708 => {
                c.r[3] = 0x0;
                pc = 0x8011b768; continue;
                pc = 0x8011b710;
            }
            0x8011b710 => {
                if !c.crbit(2) { pc = 0x8011b71c; continue; }
                pc = 0x8011b714;
            }
            0x8011b714 => {
                c.r[0] = 0x0;
                pc = 0x8011b738; continue;
                pc = 0x8011b71c;
            }
            0x8011b71c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b724; f_8011a090(c);
                pc = 0x8011b724;
            }
            0x8011b724 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011b734; continue; }
                pc = 0x8011b72c;
            }
            0x8011b72c => {
                c.r[0] = 0x0;
                pc = 0x8011b738; continue;
                pc = 0x8011b734;
            }
            0x8011b734 => {
                c.r[0] = 0x1;
                pc = 0x8011b738;
            }
            0x8011b738 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011b764; continue; }
                pc = 0x8011b744;
            }
            0x8011b744 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b74c; f_8011a090(c);
                pc = 0x8011b74c;
            }
            0x8011b74c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011b764; continue; }
                pc = 0x8011b75c;
            }
            0x8011b75c => {
                c.r[3] = 0x1;
                pc = 0x8011b768; continue;
                pc = 0x8011b764;
            }
            0x8011b764 => {
                c.r[3] = 0x0;
                pc = 0x8011b768;
            }
            0x8011b768 => {
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
            _ => panic!("f_8011b67c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011B788 (0x8011b788, 456 bytes)
#[inline(never)]
pub fn f_8011b788(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011b788) { return; }
        if c.prof { crate::prof::enter(c, 0x8011b788); f_8011b788_b(c, 0x8011b788); crate::prof::exit(c, 0x8011b788); return; }
    }
    f_8011b788_b(c, 0x8011b788)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011b788_at(c: &mut Cpu, pc: u32) { f_8011b788_b(c, pc) }
#[inline(always)]
fn f_8011b788_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011b788;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011b788, pc); }
        match pc {
            0x8011b788 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                if c.crbit(2) { pc = 0x8011b930; continue; }
                pc = 0x8011b7b4;
            }
            0x8011b7b4 => {
                if !c.crbit(2) { pc = 0x8011b7c0; continue; }
                pc = 0x8011b7b8;
            }
            0x8011b7b8 => {
                c.r[0] = 0x0;
                pc = 0x8011b878; continue;
                pc = 0x8011b7c0;
            }
            0x8011b7c0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b7c8; f_80119e90(c);
                pc = 0x8011b7c8;
            }
            0x8011b7c8 => {
                c.r[29] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b7d4; f_80119f10(c);
                pc = 0x8011b7d4;
            }
            0x8011b7d4 => {
                c.r[28] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b7e0; f_80119ed0(c);
                pc = 0x8011b7e0;
            }
            0x8011b7e0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011b7f8; f_80135e44(c);
                pc = 0x8011b7f8;
            }
            0x8011b7f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011b808; continue; }
                pc = 0x8011b800;
            }
            0x8011b800 => {
                c.r[28] = 0x0;
                pc = 0x8011b810; continue;
                pc = 0x8011b808;
            }
            0x8011b808 => {
                c.r[0] = c.r[29].rotate_left(4) & 0xffff0;
                c.r[28] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011b810;
            }
            0x8011b810 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x8011b820; continue; }
                pc = 0x8011b818;
            }
            0x8011b818 => {
                c.r[0] = 0x0;
                pc = 0x8011b878; continue;
                pc = 0x8011b820;
            }
            0x8011b820 => {
                if !c.crbit(2) { pc = 0x8011b82c; continue; }
                pc = 0x8011b824;
            }
            0x8011b824 => {
                c.r[0] = 0x0;
                pc = 0x8011b848; continue;
                pc = 0x8011b82c;
            }
            0x8011b82c => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011b834; f_8011a090(c);
                pc = 0x8011b834;
            }
            0x8011b834 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011b844; continue; }
                pc = 0x8011b83c;
            }
            0x8011b83c => {
                c.r[0] = 0x0;
                pc = 0x8011b848; continue;
                pc = 0x8011b844;
            }
            0x8011b844 => {
                c.r[0] = 0x1;
                pc = 0x8011b848;
            }
            0x8011b848 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011b874; continue; }
                pc = 0x8011b854;
            }
            0x8011b854 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x8011b85c; f_8011a090(c);
                pc = 0x8011b85c;
            }
            0x8011b85c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(2) { pc = 0x8011b874; continue; }
                pc = 0x8011b86c;
            }
            0x8011b86c => {
                c.r[0] = 0x1;
                pc = 0x8011b878; continue;
                pc = 0x8011b874;
            }
            0x8011b874 => {
                c.r[0] = 0x0;
                pc = 0x8011b878;
            }
            0x8011b878 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8011b930; continue; }
                pc = 0x8011b880;
            }
            0x8011b880 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b888; f_80119e90(c);
                pc = 0x8011b888;
            }
            0x8011b888 => {
                c.r[28] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b894; f_80119f10(c);
                pc = 0x8011b894;
            }
            0x8011b894 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x8011b8a0; f_80119ed0(c);
                pc = 0x8011b8a0;
            }
            0x8011b8a0 => {
                c.r[6] = c.r[3] | c.r[3];
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[30] | c.r[30];
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8011b8b8; f_80135e44(c);
                pc = 0x8011b8b8;
            }
            0x8011b8b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011b8c8; continue; }
                pc = 0x8011b8c0;
            }
            0x8011b8c0 => {
                c.r[30] = 0x0;
                pc = 0x8011b8d0; continue;
                pc = 0x8011b8c8;
            }
            0x8011b8c8 => {
                c.r[0] = c.r[28].rotate_left(4) & 0xffff0;
                c.r[30] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x8011b8d0;
            }
            0x8011b8d0 => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x8011b930; continue; }
                pc = 0x8011b8d8;
            }
            0x8011b8d8 => {
                if c.crbit(2) { pc = 0x8011b930; continue; }
                pc = 0x8011b8dc;
            }
            0x8011b8dc => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b8e8; f_80119ff0(c);
                pc = 0x8011b8e8;
            }
            0x8011b8e8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b8f4; f_80119fe0(c);
                pc = 0x8011b8f4;
            }
            0x8011b8f4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b900; f_80119fd0(c);
                pc = 0x8011b900;
            }
            0x8011b900 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b90c; f_80119fc0(c);
                pc = 0x8011b90c;
            }
            0x8011b90c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b918; f_80119fb0(c);
                pc = 0x8011b918;
            }
            0x8011b918 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b924; f_80119fa0(c);
                pc = 0x8011b924;
            }
            0x8011b924 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b930; f_80119f90(c);
                pc = 0x8011b930;
            }
            0x8011b930 => {
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
            _ => panic!("f_8011b788: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011B950 (0x8011b950, 188 bytes)
#[inline(never)]
pub fn f_8011b950(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011b950) { return; }
        if c.prof { crate::prof::enter(c, 0x8011b950); f_8011b950_b(c, 0x8011b950); crate::prof::exit(c, 0x8011b950); return; }
    }
    f_8011b950_b(c, 0x8011b950)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011b950_at(c: &mut Cpu, pc: u32) { f_8011b950_b(c, pc) }
#[inline(always)]
fn f_8011b950_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011b950;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011b950, pc); }
        match pc {
            0x8011b950 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4].rotate_left(0) & 0xffff;
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = 0x0;
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x8011b9e0; continue;
                pc = 0x8011b97c;
            }
            0x8011b97c => {
                c.r[0] = c.r[29].rotate_left(4) & 0xffff0;
                c.r[30] = c.r[28].wrapping_add(c.r[0]);
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x8011b9dc; continue; }
                pc = 0x8011b988;
            }
            0x8011b988 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b994; f_80119ff0(c);
                pc = 0x8011b994;
            }
            0x8011b994 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b9a0; f_80119fe0(c);
                pc = 0x8011b9a0;
            }
            0x8011b9a0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b9ac; f_80119fd0(c);
                pc = 0x8011b9ac;
            }
            0x8011b9ac => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b9b8; f_80119fc0(c);
                pc = 0x8011b9b8;
            }
            0x8011b9b8 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b9c4; f_80119fb0(c);
                pc = 0x8011b9c4;
            }
            0x8011b9c4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b9d0; f_80119fa0(c);
                pc = 0x8011b9d0;
            }
            0x8011b9d0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x8011b9dc; f_80119f90(c);
                pc = 0x8011b9dc;
            }
            0x8011b9dc => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x8011b9e0;
            }
            0x8011b9e0 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8011b97c; continue; }
                pc = 0x8011b9ec;
            }
            0x8011b9ec => {
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
            _ => panic!("f_8011b950: bad pc {:#x}", pc),
        }
    }
}

/// wazaGetWazaTypeIdName (0x8011ba0c, 180 bytes)
#[inline(never)]
pub fn f_8011ba0c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011ba0c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ba0c); f_8011ba0c_b(c, 0x8011ba0c); crate::prof::exit(c, 0x8011ba0c); return; }
    }
    f_8011ba0c_b(c, 0x8011ba0c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ba0c_at(c: &mut Cpu, pc: u32) { f_8011ba0c_b(c, pc) }
#[inline(always)]
fn f_8011ba0c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ba0c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ba0c, pc); }
        match pc {
            0x8011ba0c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011ba10;
            }
            0x8011ba10 => {
                c.r[3] = 0x0;
                pc = 0x8011ba14;
            }
            0x8011ba14 => {
                c.cmpu(0, c.r[0], 0xb);
                pc = 0x8011ba18;
            }
            0x8011ba18 => {
                if c.crbit(1) { return; }
                pc = 0x8011ba1c;
            }
            0x8011ba1c => {
                c.r[4] = 0x80360000;
                pc = 0x8011ba20;
            }
            0x8011ba20 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x8011ba24;
            }
            0x8011ba24 => {
                c.r[4] = c.r[4].wrapping_add(0xffffc260);
                pc = 0x8011ba28;
            }
            0x8011ba28 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                pc = 0x8011ba2c;
            }
            0x8011ba2c => {
                c.mtspr(9, c.r[0]);
                pc = 0x8011ba30;
            }
            0x8011ba30 => {
                if true { let t = c.ctr; if (0x8011ba0c..0x8011bac0).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x8011ba34;
            }
            0x8011ba34 => {
                c.r[3] = 0x0;
                pc = 0x8011ba38;
            }
            0x8011ba38 => {
                if true { return; }
                pc = 0x8011ba3c;
            }
            0x8011ba3c => {
                c.r[3] = 0x10000;
                pc = 0x8011ba40;
            }
            0x8011ba40 => {
                c.r[3] = c.r[3].wrapping_add(0xffffec50);
                pc = 0x8011ba44;
            }
            0x8011ba44 => {
                if true { return; }
                pc = 0x8011ba48;
            }
            0x8011ba48 => {
                c.r[3] = 0x10000;
                pc = 0x8011ba4c;
            }
            0x8011ba4c => {
                c.r[3] = c.r[3].wrapping_add(0xffffec51);
                pc = 0x8011ba50;
            }
            0x8011ba50 => {
                if true { return; }
                pc = 0x8011ba54;
            }
            0x8011ba54 => {
                c.r[3] = 0x10000;
                pc = 0x8011ba58;
            }
            0x8011ba58 => {
                c.r[3] = c.r[3].wrapping_add(0xffffec52);
                pc = 0x8011ba5c;
            }
            0x8011ba5c => {
                if true { return; }
                pc = 0x8011ba60;
            }
            0x8011ba60 => {
                c.r[3] = 0x10000;
                pc = 0x8011ba64;
            }
            0x8011ba64 => {
                c.r[3] = c.r[3].wrapping_add(0xffffed09);
                pc = 0x8011ba68;
            }
            0x8011ba68 => {
                if true { return; }
                pc = 0x8011ba6c;
            }
            0x8011ba6c => {
                c.r[3] = 0x10000;
                pc = 0x8011ba70;
            }
            0x8011ba70 => {
                c.r[3] = c.r[3].wrapping_add(0xffffec53);
                pc = 0x8011ba74;
            }
            0x8011ba74 => {
                if true { return; }
                pc = 0x8011ba78;
            }
            0x8011ba78 => {
                c.r[3] = 0x10000;
                pc = 0x8011ba7c;
            }
            0x8011ba7c => {
                c.r[3] = c.r[3].wrapping_add(0xffffec54);
                pc = 0x8011ba80;
            }
            0x8011ba80 => {
                if true { return; }
                pc = 0x8011ba84;
            }
            0x8011ba84 => {
                c.r[3] = 0x10000;
                pc = 0x8011ba88;
            }
            0x8011ba88 => {
                c.r[3] = c.r[3].wrapping_add(0xffffec55);
                pc = 0x8011ba8c;
            }
            0x8011ba8c => {
                if true { return; }
                pc = 0x8011ba90;
            }
            0x8011ba90 => {
                c.r[3] = 0x10000;
                pc = 0x8011ba94;
            }
            0x8011ba94 => {
                c.r[3] = c.r[3].wrapping_add(0xffffec56);
                pc = 0x8011ba98;
            }
            0x8011ba98 => {
                if true { return; }
                pc = 0x8011ba9c;
            }
            0x8011ba9c => {
                c.r[3] = 0x10000;
                pc = 0x8011baa0;
            }
            0x8011baa0 => {
                c.r[3] = c.r[3].wrapping_add(0xffffec57);
                pc = 0x8011baa4;
            }
            0x8011baa4 => {
                if true { return; }
                pc = 0x8011baa8;
            }
            0x8011baa8 => {
                c.r[3] = 0x10000;
                pc = 0x8011baac;
            }
            0x8011baac => {
                c.r[3] = c.r[3].wrapping_add(0xffffec58);
                pc = 0x8011bab0;
            }
            0x8011bab0 => {
                if true { return; }
                pc = 0x8011bab4;
            }
            0x8011bab4 => {
                c.r[3] = 0x10000;
                pc = 0x8011bab8;
            }
            0x8011bab8 => {
                c.r[3] = c.r[3].wrapping_add(0xffffec59);
                pc = 0x8011babc;
            }
            0x8011babc => {
                if true { return; }
                return;
            }
            _ => panic!("f_8011ba0c: bad pc {:#x}", pc),
        }
    }
}

/// wazaIsWazaTypeId (0x8011bac0, 172 bytes)
#[inline(never)]
pub fn f_8011bac0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011bac0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011bac0); f_8011bac0_b(c, 0x8011bac0); crate::prof::exit(c, 0x8011bac0); return; }
    }
    f_8011bac0_b(c, 0x8011bac0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011bac0_at(c: &mut Cpu, pc: u32) { f_8011bac0_b(c, pc) }
#[inline(always)]
fn f_8011bac0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011bac0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011bac0, pc); }
        match pc {
            0x8011bac0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x8011bafc; continue; }
                pc = 0x8011bae4;
            }
            0x8011bae4 => {
                c.cmpu(0, c.r[0], 0x163);
                if !c.crbit(2) { pc = 0x8011bafc; continue; }
                pc = 0x8011baec;
            }
            0x8011baec => {
                c.cmpu(0, c.r[0], 0x165);
                if !c.crbit(2) { pc = 0x8011bafc; continue; }
                pc = 0x8011baf4;
            }
            0x8011baf4 => {
                c.r[3] = 0x0;
                pc = 0x8011bb50; continue;
                pc = 0x8011bafc;
            }
            0x8011bafc => {
                c.r[31] = c.r[4].rotate_left(0) & 0xff;
                c.rc(c.r[31]);
                if !c.crbit(2) { pc = 0x8011bb0c; continue; }
                pc = 0x8011bb04;
            }
            0x8011bb04 => {
                c.r[3] = 0x0;
                pc = 0x8011bb50; continue;
                pc = 0x8011bb0c;
            }
            0x8011bb0c => {
                c.r[30] = 0x0;
                pc = 0x8011bb40; continue;
                pc = 0x8011bb14;
            }
            0x8011bb14 => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[6] = c.r[30].rotate_left(0) & 0xff;
                c.r[3] = 0x0;
                c.r[5] = 0x1a;
                c.lr = 0x8011bb28; f_8011beb4(c);
                pc = 0x8011bb28;
            }
            0x8011bb28 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[31], c.r[0]);
                if !c.crbit(2) { pc = 0x8011bb3c; continue; }
                pc = 0x8011bb34;
            }
            0x8011bb34 => {
                c.r[3] = 0x1;
                pc = 0x8011bb50; continue;
                pc = 0x8011bb3c;
            }
            0x8011bb3c => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x8011bb40;
            }
            0x8011bb40 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x8011bb14; continue; }
                pc = 0x8011bb4c;
            }
            0x8011bb4c => {
                c.r[3] = 0x0;
                pc = 0x8011bb50;
            }
            0x8011bb50 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8011bac0: bad pc {:#x}", pc),
        }
    }
}

/// wazaGetMaxPP (0x8011bb6c, 108 bytes)
#[inline(never)]
pub fn f_8011bb6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011bb6c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011bb6c); f_8011bb6c_b(c, 0x8011bb6c); crate::prof::exit(c, 0x8011bb6c); return; }
    }
    f_8011bb6c_b(c, 0x8011bb6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011bb6c_at(c: &mut Cpu, pc: u32) { f_8011bb6c_b(c, pc) }
#[inline(always)]
fn f_8011bb6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011bb6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011bb6c, pc); }
        match pc {
            0x8011bb6c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x2;
                c.r[6] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x8011bb94; f_8011beb4(c);
                pc = 0x8011bb94;
            }
            0x8011bb94 => {
                c.r[5] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = 0x51ec0000;
                c.r[0] = c.r[5].wrapping_mul(0x14);
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.r[3] = c.r[3].wrapping_add(0xffff851f);
                c.r[0] = c.r[4].wrapping_mul(c.r[0]);
                c.r[0] = mulhw(c.r[3], c.r[0]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 27 != 0); c.r[0] = (v >> 5) as u32; }
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[0] = c.r[5].wrapping_add(c.r[0]);
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011bb6c: bad pc {:#x}", pc),
        }
    }
}

/// wazaSetStatus (0x8011bbd8, 732 bytes)
#[inline(never)]
pub fn f_8011bbd8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011bbd8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011bbd8); f_8011bbd8_b(c, 0x8011bbd8); crate::prof::exit(c, 0x8011bbd8); return; }
    }
    f_8011bbd8_b(c, 0x8011bbd8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011bbd8_at(c: &mut Cpu, pc: u32) { f_8011bbd8_b(c, pc) }
#[inline(always)]
fn f_8011bbd8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011bbd8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011bbd8, pc); }
        match pc {
            0x8011bbd8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x8011bbdc;
            }
            0x8011bbdc => {
                c.r[0] = c.mfspr(8);
                pc = 0x8011bbe0;
            }
            0x8011bbe0 => {
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x8011bbe4;
            }
            0x8011bbe4 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                pc = 0x8011bbe8;
            }
            0x8011bbe8 => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                pc = 0x8011bbec;
            }
            0x8011bbec => {
                c.r[31] = c.r[7] | c.r[7];
                pc = 0x8011bbf0;
            }
            0x8011bbf0 => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                pc = 0x8011bbf4;
            }
            0x8011bbf4 => {
                c.r[30] = c.r[6] | c.r[6];
                pc = 0x8011bbf8;
            }
            0x8011bbf8 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                pc = 0x8011bbfc;
            }
            0x8011bbfc => {
                c.r[29] = c.r[5] | c.r[5];
                pc = 0x8011bc00;
            }
            0x8011bc00 => {
                if c.crbit(2) { pc = 0x8011be98; continue; }
                pc = 0x8011bc04;
            }
            0x8011bc04 => {
                c.cmpu(0, c.r[0], 0x35);
                pc = 0x8011bc08;
            }
            0x8011bc08 => {
                if c.crbit(0) { pc = 0x8011bc10; continue; }
                pc = 0x8011bc0c;
            }
            0x8011bc0c => {
                pc = 0x8011be98; continue;
                pc = 0x8011bc10;
            }
            0x8011bc10 => {
                c.cmpu(0, c.r[0], 0x25);
                pc = 0x8011bc14;
            }
            0x8011bc14 => {
                if !c.crbit(0) { pc = 0x8011bc2c; continue; }
                pc = 0x8011bc18;
            }
            0x8011bc18 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x8011bc1c;
            }
            0x8011bc1c => {
                c.lr = 0x8011bc20; f_8011ca34(c);
                pc = 0x8011bc20;
            }
            0x8011bc20 => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x8011bc24;
            }
            0x8011bc24 => {
                if !c.crbit(2) { pc = 0x8011bc34; continue; }
                pc = 0x8011bc28;
            }
            0x8011bc28 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bc2c;
            }
            0x8011bc2c => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x8011bc30;
            }
            0x8011bc30 => {
                if c.crbit(2) { pc = 0x8011be98; continue; }
                pc = 0x8011bc34;
            }
            0x8011bc34 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                pc = 0x8011bc38;
            }
            0x8011bc38 => {
                c.cmpu(0, c.r[0], 0x32);
                pc = 0x8011bc3c;
            }
            0x8011bc3c => {
                if c.crbit(1) { pc = 0x8011be98; continue; }
                pc = 0x8011bc40;
            }
            0x8011bc40 => {
                c.r[4] = 0x80360000;
                pc = 0x8011bc44;
            }
            0x8011bc44 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x8011bc48;
            }
            0x8011bc48 => {
                c.r[4] = c.r[4].wrapping_add(0xffffc290);
                pc = 0x8011bc4c;
            }
            0x8011bc4c => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                pc = 0x8011bc50;
            }
            0x8011bc50 => {
                c.mtspr(9, c.r[0]);
                pc = 0x8011bc54;
            }
            0x8011bc54 => {
                if true { let t = c.ctr; if (0x8011bbd8..0x8011beb4).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x8011bc58;
            }
            0x8011bc58 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011bc5c;
            }
            0x8011bc5c => {
                c.lr = 0x8011bc60; f_8011c780(c);
                pc = 0x8011bc60;
            }
            0x8011bc60 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bc64;
            }
            0x8011bc64 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bc68;
            }
            0x8011bc68 => {
                c.lr = 0x8011bc6c; f_8011c770(c);
                pc = 0x8011bc6c;
            }
            0x8011bc6c => {
                pc = 0x8011be98; continue;
                pc = 0x8011bc70;
            }
            0x8011bc70 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bc74;
            }
            0x8011bc74 => {
                c.lr = 0x8011bc78; f_8011c760(c);
                pc = 0x8011bc78;
            }
            0x8011bc78 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bc7c;
            }
            0x8011bc7c => {
                c.r[4] = c.r[31] as u8 as i8 as i32 as u32;
                pc = 0x8011bc80;
            }
            0x8011bc80 => {
                c.lr = 0x8011bc84; f_8011c750(c);
                pc = 0x8011bc84;
            }
            0x8011bc84 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bc88;
            }
            0x8011bc88 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bc8c;
            }
            0x8011bc8c => {
                c.lr = 0x8011bc90; f_8011c740(c);
                pc = 0x8011bc90;
            }
            0x8011bc90 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bc94;
            }
            0x8011bc94 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bc98;
            }
            0x8011bc98 => {
                c.lr = 0x8011bc9c; f_8011c730(c);
                pc = 0x8011bc9c;
            }
            0x8011bc9c => {
                pc = 0x8011be98; continue;
                pc = 0x8011bca0;
            }
            0x8011bca0 => {
                c.r[4] = c.r[31] as u16 as i16 as i32 as u32;
                pc = 0x8011bca4;
            }
            0x8011bca4 => {
                c.lr = 0x8011bca8; f_8011c720(c);
                pc = 0x8011bca8;
            }
            0x8011bca8 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bcac;
            }
            0x8011bcac => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x8011bcb0;
            }
            0x8011bcb0 => {
                c.lr = 0x8011bcb4; f_8011c710(c);
                pc = 0x8011bcb4;
            }
            0x8011bcb4 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bcb8;
            }
            0x8011bcb8 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x8011bcbc;
            }
            0x8011bcbc => {
                c.lr = 0x8011bcc0; f_8011c700(c);
                pc = 0x8011bcc0;
            }
            0x8011bcc0 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bcc4;
            }
            0x8011bcc4 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x8011bcc8;
            }
            0x8011bcc8 => {
                c.lr = 0x8011bccc; f_8011c6f0(c);
                pc = 0x8011bccc;
            }
            0x8011bccc => {
                pc = 0x8011be98; continue;
                pc = 0x8011bcd0;
            }
            0x8011bcd0 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x8011bcd4;
            }
            0x8011bcd4 => {
                c.lr = 0x8011bcd8; f_8011c6f0(c);
                pc = 0x8011bcd8;
            }
            0x8011bcd8 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bcdc;
            }
            0x8011bcdc => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bce0;
            }
            0x8011bce0 => {
                c.lr = 0x8011bce4; f_8011c6e0(c);
                pc = 0x8011bce4;
            }
            0x8011bce4 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bce8;
            }
            0x8011bce8 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bcec;
            }
            0x8011bcec => {
                c.lr = 0x8011bcf0; f_8011c6d0(c);
                pc = 0x8011bcf0;
            }
            0x8011bcf0 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bcf4;
            }
            0x8011bcf4 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bcf8;
            }
            0x8011bcf8 => {
                c.lr = 0x8011bcfc; f_8011c6c0(c);
                pc = 0x8011bcfc;
            }
            0x8011bcfc => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd00;
            }
            0x8011bd00 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd04;
            }
            0x8011bd04 => {
                c.lr = 0x8011bd08; f_8011c6b0(c);
                pc = 0x8011bd08;
            }
            0x8011bd08 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd0c;
            }
            0x8011bd0c => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd10;
            }
            0x8011bd10 => {
                c.lr = 0x8011bd14; f_8011c6a0(c);
                pc = 0x8011bd14;
            }
            0x8011bd14 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd18;
            }
            0x8011bd18 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd1c;
            }
            0x8011bd1c => {
                c.lr = 0x8011bd20; f_8011c690(c);
                pc = 0x8011bd20;
            }
            0x8011bd20 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd24;
            }
            0x8011bd24 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd28;
            }
            0x8011bd28 => {
                c.lr = 0x8011bd2c; f_8011c680(c);
                pc = 0x8011bd2c;
            }
            0x8011bd2c => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd30;
            }
            0x8011bd30 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd34;
            }
            0x8011bd34 => {
                c.lr = 0x8011bd38; f_8011c670(c);
                pc = 0x8011bd38;
            }
            0x8011bd38 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd3c;
            }
            0x8011bd3c => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd40;
            }
            0x8011bd40 => {
                c.lr = 0x8011bd44; f_8011c660(c);
                pc = 0x8011bd44;
            }
            0x8011bd44 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd48;
            }
            0x8011bd48 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd4c;
            }
            0x8011bd4c => {
                c.lr = 0x8011bd50; f_8011c650(c);
                pc = 0x8011bd50;
            }
            0x8011bd50 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd54;
            }
            0x8011bd54 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd58;
            }
            0x8011bd58 => {
                c.lr = 0x8011bd5c; f_8011c640(c);
                pc = 0x8011bd5c;
            }
            0x8011bd5c => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd60;
            }
            0x8011bd60 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd64;
            }
            0x8011bd64 => {
                c.lr = 0x8011bd68; f_8011c630(c);
                pc = 0x8011bd68;
            }
            0x8011bd68 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd6c;
            }
            0x8011bd6c => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd70;
            }
            0x8011bd70 => {
                c.lr = 0x8011bd74; f_8011c620(c);
                pc = 0x8011bd74;
            }
            0x8011bd74 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd78;
            }
            0x8011bd78 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd7c;
            }
            0x8011bd7c => {
                c.lr = 0x8011bd80; f_8011c5b8(c);
                pc = 0x8011bd80;
            }
            0x8011bd80 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd84;
            }
            0x8011bd84 => {
                c.r[4] = c.r[30] | c.r[30];
                pc = 0x8011bd88;
            }
            0x8011bd88 => {
                c.r[5] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd8c;
            }
            0x8011bd8c => {
                c.lr = 0x8011bd90; f_8011c430(c);
                pc = 0x8011bd90;
            }
            0x8011bd90 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bd94;
            }
            0x8011bd94 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bd98;
            }
            0x8011bd98 => {
                c.lr = 0x8011bd9c; f_8011c420(c);
                pc = 0x8011bd9c;
            }
            0x8011bd9c => {
                pc = 0x8011be98; continue;
                pc = 0x8011bda0;
            }
            0x8011bda0 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011bda4;
            }
            0x8011bda4 => {
                c.lr = 0x8011bda8; f_8011c3d0(c);
                pc = 0x8011bda8;
            }
            0x8011bda8 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bdac;
            }
            0x8011bdac => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011bdb0;
            }
            0x8011bdb0 => {
                c.lr = 0x8011bdb4; f_8011c380(c);
                pc = 0x8011bdb4;
            }
            0x8011bdb4 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bdb8;
            }
            0x8011bdb8 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011bdbc;
            }
            0x8011bdbc => {
                c.lr = 0x8011bdc0; f_8011c330(c);
                pc = 0x8011bdc0;
            }
            0x8011bdc0 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bdc4;
            }
            0x8011bdc4 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011bdc8;
            }
            0x8011bdc8 => {
                c.lr = 0x8011bdcc; f_8011c220(c);
                pc = 0x8011bdcc;
            }
            0x8011bdcc => {
                pc = 0x8011be98; continue;
                pc = 0x8011bdd0;
            }
            0x8011bdd0 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011bdd4;
            }
            0x8011bdd4 => {
                c.lr = 0x8011bdd8; f_8011c1d0(c);
                pc = 0x8011bdd8;
            }
            0x8011bdd8 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bddc;
            }
            0x8011bddc => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011bde0;
            }
            0x8011bde0 => {
                c.lr = 0x8011bde4; f_8011c610(c);
                pc = 0x8011bde4;
            }
            0x8011bde4 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bde8;
            }
            0x8011bde8 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011bdec;
            }
            0x8011bdec => {
                c.lr = 0x8011bdf0; f_8011c600(c);
                pc = 0x8011bdf0;
            }
            0x8011bdf0 => {
                pc = 0x8011be98; continue;
                pc = 0x8011bdf4;
            }
            0x8011bdf4 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011bdf8;
            }
            0x8011bdf8 => {
                c.lr = 0x8011bdfc; f_8011c5f0(c);
                pc = 0x8011bdfc;
            }
            0x8011bdfc => {
                pc = 0x8011be98; continue;
                pc = 0x8011be00;
            }
            0x8011be00 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011be04;
            }
            0x8011be04 => {
                c.lr = 0x8011be08; f_8011c5e0(c);
                pc = 0x8011be08;
            }
            0x8011be08 => {
                pc = 0x8011be98; continue;
                pc = 0x8011be0c;
            }
            0x8011be0c => {
                c.r[4] = c.r[31] as u8 as i8 as i32 as u32;
                pc = 0x8011be10;
            }
            0x8011be10 => {
                c.lr = 0x8011be14; f_8020a154(c);
                pc = 0x8011be14;
            }
            0x8011be14 => {
                pc = 0x8011be98; continue;
                pc = 0x8011be18;
            }
            0x8011be18 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x8011be1c;
            }
            0x8011be1c => {
                c.lr = 0x8011be20; f_8020a144(c);
                pc = 0x8011be20;
            }
            0x8011be20 => {
                pc = 0x8011be98; continue;
                pc = 0x8011be24;
            }
            0x8011be24 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x8011be28;
            }
            0x8011be28 => {
                c.lr = 0x8011be2c; f_8020a134(c);
                pc = 0x8011be2c;
            }
            0x8011be2c => {
                pc = 0x8011be98; continue;
                pc = 0x8011be30;
            }
            0x8011be30 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x8011be34;
            }
            0x8011be34 => {
                c.lr = 0x8011be38; f_8020a124(c);
                pc = 0x8011be38;
            }
            0x8011be38 => {
                pc = 0x8011be98; continue;
                pc = 0x8011be3c;
            }
            0x8011be3c => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011be40;
            }
            0x8011be40 => {
                c.lr = 0x8011be44; f_8020a114(c);
                pc = 0x8011be44;
            }
            0x8011be44 => {
                pc = 0x8011be98; continue;
                pc = 0x8011be48;
            }
            0x8011be48 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011be4c;
            }
            0x8011be4c => {
                c.lr = 0x8011be50; f_8020a104(c);
                pc = 0x8011be50;
            }
            0x8011be50 => {
                pc = 0x8011be98; continue;
                pc = 0x8011be54;
            }
            0x8011be54 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011be58;
            }
            0x8011be58 => {
                c.lr = 0x8011be5c; f_8020a0f4(c);
                pc = 0x8011be5c;
            }
            0x8011be5c => {
                pc = 0x8011be98; continue;
                pc = 0x8011be60;
            }
            0x8011be60 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011be64;
            }
            0x8011be64 => {
                c.lr = 0x8011be68; f_8020a0e4(c);
                pc = 0x8011be68;
            }
            0x8011be68 => {
                pc = 0x8011be98; continue;
                pc = 0x8011be6c;
            }
            0x8011be6c => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x8011be70;
            }
            0x8011be70 => {
                c.lr = 0x8011be74; f_8020a0d4(c);
                pc = 0x8011be74;
            }
            0x8011be74 => {
                pc = 0x8011be98; continue;
                pc = 0x8011be78;
            }
            0x8011be78 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xffff;
                pc = 0x8011be7c;
            }
            0x8011be7c => {
                c.lr = 0x8011be80; f_8020a0c4(c);
                pc = 0x8011be80;
            }
            0x8011be80 => {
                pc = 0x8011be98; continue;
                pc = 0x8011be84;
            }
            0x8011be84 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011be88;
            }
            0x8011be88 => {
                c.lr = 0x8011be8c; f_8020a0b4(c);
                pc = 0x8011be8c;
            }
            0x8011be8c => {
                pc = 0x8011be98; continue;
                pc = 0x8011be90;
            }
            0x8011be90 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8011be94;
            }
            0x8011be94 => {
                c.lr = 0x8011be98; f_8020a0a4(c);
                pc = 0x8011be98;
            }
            0x8011be98 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x8011be9c;
            }
            0x8011be9c => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                pc = 0x8011bea0;
            }
            0x8011bea0 => {
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                pc = 0x8011bea4;
            }
            0x8011bea4 => {
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x8011bea8;
            }
            0x8011bea8 => {
                c.mtspr(8, c.r[0]);
                pc = 0x8011beac;
            }
            0x8011beac => {
                c.r[1] = c.r[1].wrapping_add(0x20);
                pc = 0x8011beb0;
            }
            0x8011beb0 => {
                if true { return; }
                return;
            }
            _ => panic!("f_8011bbd8: bad pc {:#x}", pc),
        }
    }
}

/// wazaGetStatus (0x8011beb4, 796 bytes)
#[inline(never)]
pub fn f_8011beb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011beb4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011beb4); f_8011beb4_b(c, 0x8011beb4); crate::prof::exit(c, 0x8011beb4); return; }
    }
    f_8011beb4_b(c, 0x8011beb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011beb4_at(c: &mut Cpu, pc: u32) { f_8011beb4_b(c, pc) }
#[inline(always)]
fn f_8011beb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011beb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011beb4, pc); }
        match pc {
            0x8011beb4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x8011beb8;
            }
            0x8011beb8 => {
                c.r[0] = c.mfspr(8);
                pc = 0x8011bebc;
            }
            0x8011bebc => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                pc = 0x8011bec0;
            }
            0x8011bec0 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                pc = 0x8011bec4;
            }
            0x8011bec4 => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                pc = 0x8011bec8;
            }
            0x8011bec8 => {
                c.r[31] = c.r[6] | c.r[6];
                pc = 0x8011becc;
            }
            0x8011becc => {
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                pc = 0x8011bed0;
            }
            0x8011bed0 => {
                c.r[30] = c.r[5] | c.r[5];
                pc = 0x8011bed4;
            }
            0x8011bed4 => {
                if c.crbit(2) { pc = 0x8011bee0; continue; }
                pc = 0x8011bed8;
            }
            0x8011bed8 => {
                c.cmpu(0, c.r[0], 0x35);
                pc = 0x8011bedc;
            }
            0x8011bedc => {
                if c.crbit(0) { pc = 0x8011bee8; continue; }
                pc = 0x8011bee0;
            }
            0x8011bee0 => {
                c.r[3] = 0x0;
                pc = 0x8011bee4;
            }
            0x8011bee4 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bee8;
            }
            0x8011bee8 => {
                c.cmpu(0, c.r[0], 0x25);
                pc = 0x8011beec;
            }
            0x8011beec => {
                if !c.crbit(0) { pc = 0x8011bf08; continue; }
                pc = 0x8011bef0;
            }
            0x8011bef0 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x8011bef4;
            }
            0x8011bef4 => {
                c.lr = 0x8011bef8; f_8011ca34(c);
                pc = 0x8011bef8;
            }
            0x8011bef8 => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x8011befc;
            }
            0x8011befc => {
                if !c.crbit(2) { pc = 0x8011bf18; continue; }
                pc = 0x8011bf00;
            }
            0x8011bf00 => {
                c.r[3] = 0x0;
                pc = 0x8011bf04;
            }
            0x8011bf04 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bf08;
            }
            0x8011bf08 => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x8011bf0c;
            }
            0x8011bf0c => {
                if !c.crbit(2) { pc = 0x8011bf18; continue; }
                pc = 0x8011bf10;
            }
            0x8011bf10 => {
                c.r[3] = 0x0;
                pc = 0x8011bf14;
            }
            0x8011bf14 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bf18;
            }
            0x8011bf18 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                pc = 0x8011bf1c;
            }
            0x8011bf1c => {
                c.cmpu(0, c.r[0], 0x34);
                pc = 0x8011bf20;
            }
            0x8011bf20 => {
                if c.crbit(1) { pc = 0x8011c1b4; continue; }
                pc = 0x8011bf24;
            }
            0x8011bf24 => {
                c.r[4] = 0x80360000;
                pc = 0x8011bf28;
            }
            0x8011bf28 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x8011bf2c;
            }
            0x8011bf2c => {
                c.r[4] = c.r[4].wrapping_add(0xffffc35c);
                pc = 0x8011bf30;
            }
            0x8011bf30 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                pc = 0x8011bf34;
            }
            0x8011bf34 => {
                c.mtspr(9, c.r[0]);
                pc = 0x8011bf38;
            }
            0x8011bf38 => {
                if true { let t = c.ctr; if (0x8011beb4..0x8011c1d0).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x8011bf3c;
            }
            0x8011bf3c => {
                c.lr = 0x8011bf40; f_8011ca1c(c);
                pc = 0x8011bf40;
            }
            0x8011bf40 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bf44;
            }
            0x8011bf44 => {
                c.lr = 0x8011bf48; f_8011ca04(c);
                pc = 0x8011bf48;
            }
            0x8011bf48 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011bf4c;
            }
            0x8011bf4c => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bf50;
            }
            0x8011bf50 => {
                c.lr = 0x8011bf54; f_8011c9ec(c);
                pc = 0x8011bf54;
            }
            0x8011bf54 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011bf58;
            }
            0x8011bf58 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bf5c;
            }
            0x8011bf5c => {
                c.lr = 0x8011bf60; f_8011c9d0(c);
                pc = 0x8011bf60;
            }
            0x8011bf60 => {
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x8011bf64;
            }
            0x8011bf64 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bf68;
            }
            0x8011bf68 => {
                c.lr = 0x8011bf6c; f_8011c9b8(c);
                pc = 0x8011bf6c;
            }
            0x8011bf6c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011bf70;
            }
            0x8011bf70 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bf74;
            }
            0x8011bf74 => {
                c.lr = 0x8011bf78; f_8011c9a0(c);
                pc = 0x8011bf78;
            }
            0x8011bf78 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011bf7c;
            }
            0x8011bf7c => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bf80;
            }
            0x8011bf80 => {
                c.lr = 0x8011bf84; f_8011c988(c);
                pc = 0x8011bf84;
            }
            0x8011bf84 => {
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x8011bf88;
            }
            0x8011bf88 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bf8c;
            }
            0x8011bf8c => {
                c.lr = 0x8011bf90; f_8011c970(c);
                pc = 0x8011bf90;
            }
            0x8011bf90 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8011bf94;
            }
            0x8011bf94 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bf98;
            }
            0x8011bf98 => {
                c.lr = 0x8011bf9c; f_8011c958(c);
                pc = 0x8011bf9c;
            }
            0x8011bf9c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8011bfa0;
            }
            0x8011bfa0 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bfa4;
            }
            0x8011bfa4 => {
                c.lr = 0x8011bfa8; f_8011c940(c);
                pc = 0x8011bfa8;
            }
            0x8011bfa8 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bfac;
            }
            0x8011bfac => {
                c.lr = 0x8011bfb0; f_8011c928(c);
                pc = 0x8011bfb0;
            }
            0x8011bfb0 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bfb4;
            }
            0x8011bfb4 => {
                c.lr = 0x8011bfb8; f_8011c910(c);
                pc = 0x8011bfb8;
            }
            0x8011bfb8 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011bfbc;
            }
            0x8011bfbc => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bfc0;
            }
            0x8011bfc0 => {
                c.lr = 0x8011bfc4; f_8011c8f8(c);
                pc = 0x8011bfc4;
            }
            0x8011bfc4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011bfc8;
            }
            0x8011bfc8 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bfcc;
            }
            0x8011bfcc => {
                c.lr = 0x8011bfd0; f_8011c8e0(c);
                pc = 0x8011bfd0;
            }
            0x8011bfd0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011bfd4;
            }
            0x8011bfd4 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bfd8;
            }
            0x8011bfd8 => {
                c.lr = 0x8011bfdc; f_8011c8c8(c);
                pc = 0x8011bfdc;
            }
            0x8011bfdc => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011bfe0;
            }
            0x8011bfe0 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bfe4;
            }
            0x8011bfe4 => {
                c.lr = 0x8011bfe8; f_8011c8b0(c);
                pc = 0x8011bfe8;
            }
            0x8011bfe8 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011bfec;
            }
            0x8011bfec => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bff0;
            }
            0x8011bff0 => {
                c.lr = 0x8011bff4; f_8011c898(c);
                pc = 0x8011bff4;
            }
            0x8011bff4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011bff8;
            }
            0x8011bff8 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011bffc;
            }
            0x8011bffc => {
                c.lr = 0x8011c000; f_8011c880(c);
                pc = 0x8011c000;
            }
            0x8011c000 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c004;
            }
            0x8011c004 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c008;
            }
            0x8011c008 => {
                c.lr = 0x8011c00c; f_8011c868(c);
                pc = 0x8011c00c;
            }
            0x8011c00c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c010;
            }
            0x8011c010 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c014;
            }
            0x8011c014 => {
                c.lr = 0x8011c018; f_8011c850(c);
                pc = 0x8011c018;
            }
            0x8011c018 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c01c;
            }
            0x8011c01c => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c020;
            }
            0x8011c020 => {
                c.lr = 0x8011c024; f_8011c838(c);
                pc = 0x8011c024;
            }
            0x8011c024 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c028;
            }
            0x8011c028 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c02c;
            }
            0x8011c02c => {
                c.lr = 0x8011c030; f_8011c820(c);
                pc = 0x8011c030;
            }
            0x8011c030 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c034;
            }
            0x8011c034 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c038;
            }
            0x8011c038 => {
                c.lr = 0x8011c03c; f_8011c808(c);
                pc = 0x8011c03c;
            }
            0x8011c03c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c040;
            }
            0x8011c040 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c044;
            }
            0x8011c044 => {
                c.lr = 0x8011c048; f_8011c7f0(c);
                pc = 0x8011c048;
            }
            0x8011c048 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c04c;
            }
            0x8011c04c => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c050;
            }
            0x8011c050 => {
                c.lr = 0x8011c054; f_8011c5c8(c);
                pc = 0x8011c054;
            }
            0x8011c054 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c058;
            }
            0x8011c058 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c05c;
            }
            0x8011c05c => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011c060;
            }
            0x8011c060 => {
                c.lr = 0x8011c064; f_8011c588(c);
                pc = 0x8011c064;
            }
            0x8011c064 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c068;
            }
            0x8011c068 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c06c;
            }
            0x8011c06c => {
                c.lr = 0x8011c070; f_8011c570(c);
                pc = 0x8011c070;
            }
            0x8011c070 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c074;
            }
            0x8011c074 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c078;
            }
            0x8011c078 => {
                c.lr = 0x8011c07c; f_8011c510(c);
                pc = 0x8011c07c;
            }
            0x8011c07c => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c080;
            }
            0x8011c080 => {
                c.lr = 0x8011c084; f_8011c4b0(c);
                pc = 0x8011c084;
            }
            0x8011c084 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c088;
            }
            0x8011c088 => {
                c.lr = 0x8011c08c; f_8011c450(c);
                pc = 0x8011c08c;
            }
            0x8011c08c => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c090;
            }
            0x8011c090 => {
                c.lr = 0x8011c094; f_8011c2d0(c);
                pc = 0x8011c094;
            }
            0x8011c094 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c098;
            }
            0x8011c098 => {
                c.lr = 0x8011c09c; f_8011c270(c);
                pc = 0x8011c09c;
            }
            0x8011c09c => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c0a0;
            }
            0x8011c0a0 => {
                c.lr = 0x8011c0a4; f_8011c7d8(c);
                pc = 0x8011c0a4;
            }
            0x8011c0a4 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c0a8;
            }
            0x8011c0a8 => {
                c.lr = 0x8011c0ac; f_8011c7c0(c);
                pc = 0x8011c0ac;
            }
            0x8011c0ac => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c0b0;
            }
            0x8011c0b0 => {
                c.lr = 0x8011c0b4; f_8011c7a8(c);
                pc = 0x8011c0b4;
            }
            0x8011c0b4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c0b8;
            }
            0x8011c0b8 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c0bc;
            }
            0x8011c0bc => {
                c.lr = 0x8011c0c0; f_8011c790(c);
                pc = 0x8011c0c0;
            }
            0x8011c0c0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c0c4;
            }
            0x8011c0c4 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c0c8;
            }
            0x8011c0c8 => {
                c.lr = 0x8011c0cc; f_8020a2a0(c);
                pc = 0x8011c0cc;
            }
            0x8011c0cc => {
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                pc = 0x8011c0d0;
            }
            0x8011c0d0 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c0d4;
            }
            0x8011c0d4 => {
                c.lr = 0x8011c0d8; f_8020a288(c);
                pc = 0x8011c0d8;
            }
            0x8011c0d8 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8011c0dc;
            }
            0x8011c0dc => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c0e0;
            }
            0x8011c0e0 => {
                c.lr = 0x8011c0e4; f_8020a270(c);
                pc = 0x8011c0e4;
            }
            0x8011c0e4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8011c0e8;
            }
            0x8011c0e8 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c0ec;
            }
            0x8011c0ec => {
                c.lr = 0x8011c0f0; f_8020a258(c);
                pc = 0x8011c0f0;
            }
            0x8011c0f0 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8011c0f4;
            }
            0x8011c0f4 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c0f8;
            }
            0x8011c0f8 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x8011c0fc;
            }
            0x8011c0fc => {
                c.lr = 0x8011c100; f_8020a224(c);
                pc = 0x8011c100;
            }
            0x8011c100 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c104;
            }
            0x8011c104 => {
                c.lr = 0x8011c108; f_8020a20c(c);
                pc = 0x8011c108;
            }
            0x8011c108 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c10c;
            }
            0x8011c10c => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c110;
            }
            0x8011c110 => {
                c.lr = 0x8011c114; f_8020a1f4(c);
                pc = 0x8011c114;
            }
            0x8011c114 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c118;
            }
            0x8011c118 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c11c;
            }
            0x8011c11c => {
                c.lr = 0x8011c120; f_8020a1dc(c);
                pc = 0x8011c120;
            }
            0x8011c120 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c124;
            }
            0x8011c124 => {
                c.lr = 0x8011c128; f_8020a1c4(c);
                pc = 0x8011c128;
            }
            0x8011c128 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c12c;
            }
            0x8011c12c => {
                c.lr = 0x8011c130; f_8020a1ac(c);
                pc = 0x8011c130;
            }
            0x8011c130 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8011c134;
            }
            0x8011c134 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c138;
            }
            0x8011c138 => {
                c.lr = 0x8011c13c; f_8020a194(c);
                pc = 0x8011c13c;
            }
            0x8011c13c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x8011c140;
            }
            0x8011c140 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c144;
            }
            0x8011c144 => {
                c.lr = 0x8011c148; f_8020a17c(c);
                pc = 0x8011c148;
            }
            0x8011c148 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c14c;
            }
            0x8011c14c => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c150;
            }
            0x8011c150 => {
                c.lr = 0x8011c154; f_8020a164(c);
                pc = 0x8011c154;
            }
            0x8011c154 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8011c158;
            }
            0x8011c158 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c15c;
            }
            0x8011c15c => {
                c.r[4] = 0x0;
                pc = 0x8011c160;
            }
            0x8011c160 => {
                c.r[5] = 0x27;
                pc = 0x8011c164;
            }
            0x8011c164 => {
                c.r[6] = 0x0;
                pc = 0x8011c168;
            }
            0x8011c168 => {
                c.lr = 0x8011c16c; f_8011beb4(c);
                pc = 0x8011c16c;
            }
            0x8011c16c => {
                c.r[0] = c.r[3] | c.r[3];
                pc = 0x8011c170;
            }
            0x8011c170 => {
                c.r[3] = 0x0;
                pc = 0x8011c174;
            }
            0x8011c174 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x8011c178;
            }
            0x8011c178 => {
                c.r[5] = 0x3;
                pc = 0x8011c17c;
            }
            0x8011c17c => {
                c.r[6] = 0x0;
                pc = 0x8011c180;
            }
            0x8011c180 => {
                c.lr = 0x8011c184; f_8011beb4(c);
                pc = 0x8011c184;
            }
            0x8011c184 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c188;
            }
            0x8011c188 => {
                c.r[4] = 0x0;
                pc = 0x8011c18c;
            }
            0x8011c18c => {
                c.r[5] = 0x28;
                pc = 0x8011c190;
            }
            0x8011c190 => {
                c.r[6] = 0x0;
                pc = 0x8011c194;
            }
            0x8011c194 => {
                c.lr = 0x8011c198; f_8011beb4(c);
                pc = 0x8011c198;
            }
            0x8011c198 => {
                c.r[0] = c.r[3] | c.r[3];
                pc = 0x8011c19c;
            }
            0x8011c19c => {
                c.r[3] = 0x0;
                pc = 0x8011c1a0;
            }
            0x8011c1a0 => {
                c.r[4] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x8011c1a4;
            }
            0x8011c1a4 => {
                c.r[5] = 0x3;
                pc = 0x8011c1a8;
            }
            0x8011c1a8 => {
                c.r[6] = 0x0;
                pc = 0x8011c1ac;
            }
            0x8011c1ac => {
                c.lr = 0x8011c1b0; f_8011beb4(c);
                pc = 0x8011c1b0;
            }
            0x8011c1b0 => {
                pc = 0x8011c1b8; continue;
                pc = 0x8011c1b4;
            }
            0x8011c1b4 => {
                c.r[3] = 0x0;
                pc = 0x8011c1b8;
            }
            0x8011c1b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x8011c1bc;
            }
            0x8011c1bc => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x8011c1c0;
            }
            0x8011c1c0 => {
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                pc = 0x8011c1c4;
            }
            0x8011c1c4 => {
                c.mtspr(8, c.r[0]);
                pc = 0x8011c1c8;
            }
            0x8011c1c8 => {
                c.r[1] = c.r[1].wrapping_add(0x10);
                pc = 0x8011c1cc;
            }
            0x8011c1cc => {
                if true { return; }
                return;
            }
            _ => panic!("f_8011beb4: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetFightWazaWzxVariationFuncPtr (0x8011c1d0, 80 bytes)
#[inline(never)]
pub fn f_8011c1d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c1d0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c1d0); f_8011c1d0_b(c, 0x8011c1d0); crate::prof::exit(c, 0x8011c1d0); return; }
    }
    f_8011c1d0_b(c, 0x8011c1d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c1d0_at(c: &mut Cpu, pc: u32) { f_8011c1d0_b(c, pc) }
#[inline(always)]
fn f_8011c1d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c1d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c1d0, pc); }
        match pc {
            0x8011c1d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c1d8;
            }
            0x8011c1d8 => {
                if !c.crbit(2) { pc = 0x8011c1e4; continue; }
                pc = 0x8011c1dc;
            }
            0x8011c1dc => {
                c.r[3] = 0x0;
                pc = 0x8011c1e8; continue;
                pc = 0x8011c1e4;
            }
            0x8011c1e4 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                pc = 0x8011c1e8;
            }
            0x8011c1e8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8330));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011c200; continue; }
                pc = 0x8011c1f8;
            }
            0x8011c1f8 => {
                c.r[3] = 0x0;
                pc = 0x8011c210; continue;
                pc = 0x8011c200;
            }
            0x8011c200 => {
                c.r[5] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffc430);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8011c210;
            }
            0x8011c210 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c218;
            }
            0x8011c218 => {
                c.stw(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c1d0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetFightWazaWzxTypeFuncPtr (0x8011c220, 80 bytes)
#[inline(never)]
pub fn f_8011c220(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c220) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c220); f_8011c220_b(c, 0x8011c220); crate::prof::exit(c, 0x8011c220); return; }
    }
    f_8011c220_b(c, 0x8011c220)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c220_at(c: &mut Cpu, pc: u32) { f_8011c220_b(c, pc) }
#[inline(always)]
fn f_8011c220_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c220;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c220, pc); }
        match pc {
            0x8011c220 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c228;
            }
            0x8011c228 => {
                if !c.crbit(2) { pc = 0x8011c234; continue; }
                pc = 0x8011c22c;
            }
            0x8011c22c => {
                c.r[3] = 0x0;
                pc = 0x8011c238; continue;
                pc = 0x8011c234;
            }
            0x8011c234 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                pc = 0x8011c238;
            }
            0x8011c238 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8330));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011c250; continue; }
                pc = 0x8011c248;
            }
            0x8011c248 => {
                c.r[3] = 0x0;
                pc = 0x8011c260; continue;
                pc = 0x8011c250;
            }
            0x8011c250 => {
                c.r[5] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffc430);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8011c260;
            }
            0x8011c260 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c268;
            }
            0x8011c268 => {
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c220: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetFightWazaWzxVariationFuncPtr (0x8011c270, 96 bytes)
#[inline(never)]
pub fn f_8011c270(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c270) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c270); f_8011c270_b(c, 0x8011c270); crate::prof::exit(c, 0x8011c270); return; }
    }
    f_8011c270_b(c, 0x8011c270)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c270_at(c: &mut Cpu, pc: u32) { f_8011c270_b(c, pc) }
#[inline(always)]
fn f_8011c270_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c270;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c270, pc); }
        match pc {
            0x8011c270 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c280; continue; }
                pc = 0x8011c278;
            }
            0x8011c278 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c280;
            }
            0x8011c280 => {
                if !c.crbit(2) { pc = 0x8011c28c; continue; }
                pc = 0x8011c284;
            }
            0x8011c284 => {
                c.r[3] = 0x0;
                pc = 0x8011c290; continue;
                pc = 0x8011c28c;
            }
            0x8011c28c => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                pc = 0x8011c290;
            }
            0x8011c290 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8330));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011c2a8; continue; }
                pc = 0x8011c2a0;
            }
            0x8011c2a0 => {
                c.r[3] = 0x0;
                pc = 0x8011c2b8; continue;
                pc = 0x8011c2a8;
            }
            0x8011c2a8 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffc430);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8011c2b8;
            }
            0x8011c2b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c2c8; continue; }
                pc = 0x8011c2c0;
            }
            0x8011c2c0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c2c8;
            }
            0x8011c2c8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x10));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c270: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetFightWazaWzxTypeFuncPtr (0x8011c2d0, 96 bytes)
#[inline(never)]
pub fn f_8011c2d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c2d0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c2d0); f_8011c2d0_b(c, 0x8011c2d0); crate::prof::exit(c, 0x8011c2d0); return; }
    }
    f_8011c2d0_b(c, 0x8011c2d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c2d0_at(c: &mut Cpu, pc: u32) { f_8011c2d0_b(c, pc) }
#[inline(always)]
fn f_8011c2d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c2d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c2d0, pc); }
        match pc {
            0x8011c2d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c2e0; continue; }
                pc = 0x8011c2d8;
            }
            0x8011c2d8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c2e0;
            }
            0x8011c2e0 => {
                if !c.crbit(2) { pc = 0x8011c2ec; continue; }
                pc = 0x8011c2e4;
            }
            0x8011c2e4 => {
                c.r[3] = 0x0;
                pc = 0x8011c2f0; continue;
                pc = 0x8011c2ec;
            }
            0x8011c2ec => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                pc = 0x8011c2f0;
            }
            0x8011c2f0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8330));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011c308; continue; }
                pc = 0x8011c300;
            }
            0x8011c300 => {
                c.r[3] = 0x0;
                pc = 0x8011c318; continue;
                pc = 0x8011c308;
            }
            0x8011c308 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffc430);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8011c318;
            }
            0x8011c318 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c328; continue; }
                pc = 0x8011c320;
            }
            0x8011c320 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c328;
            }
            0x8011c328 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c2d0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetFightTrainerAiWazaDamageFuncPtr (0x8011c330, 80 bytes)
#[inline(never)]
pub fn f_8011c330(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c330) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c330); f_8011c330_b(c, 0x8011c330); crate::prof::exit(c, 0x8011c330); return; }
    }
    f_8011c330_b(c, 0x8011c330)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c330_at(c: &mut Cpu, pc: u32) { f_8011c330_b(c, pc) }
#[inline(always)]
fn f_8011c330_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c330;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c330, pc); }
        match pc {
            0x8011c330 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c338;
            }
            0x8011c338 => {
                if !c.crbit(2) { pc = 0x8011c344; continue; }
                pc = 0x8011c33c;
            }
            0x8011c33c => {
                c.r[3] = 0x0;
                pc = 0x8011c348; continue;
                pc = 0x8011c344;
            }
            0x8011c344 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                pc = 0x8011c348;
            }
            0x8011c348 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8330));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011c360; continue; }
                pc = 0x8011c358;
            }
            0x8011c358 => {
                c.r[3] = 0x0;
                pc = 0x8011c370; continue;
                pc = 0x8011c360;
            }
            0x8011c360 => {
                c.r[5] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffc430);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8011c370;
            }
            0x8011c370 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c378;
            }
            0x8011c378 => {
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c330: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetFightTrainerAiWazaHitFuncPtr (0x8011c380, 80 bytes)
#[inline(never)]
pub fn f_8011c380(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c380) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c380); f_8011c380_b(c, 0x8011c380); crate::prof::exit(c, 0x8011c380); return; }
    }
    f_8011c380_b(c, 0x8011c380)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c380_at(c: &mut Cpu, pc: u32) { f_8011c380_b(c, pc) }
#[inline(always)]
fn f_8011c380_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c380;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c380, pc); }
        match pc {
            0x8011c380 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c388;
            }
            0x8011c388 => {
                if !c.crbit(2) { pc = 0x8011c394; continue; }
                pc = 0x8011c38c;
            }
            0x8011c38c => {
                c.r[3] = 0x0;
                pc = 0x8011c398; continue;
                pc = 0x8011c394;
            }
            0x8011c394 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                pc = 0x8011c398;
            }
            0x8011c398 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8330));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011c3b0; continue; }
                pc = 0x8011c3a8;
            }
            0x8011c3a8 => {
                c.r[3] = 0x0;
                pc = 0x8011c3c0; continue;
                pc = 0x8011c3b0;
            }
            0x8011c3b0 => {
                c.r[5] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffc430);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8011c3c0;
            }
            0x8011c3c0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c3c8;
            }
            0x8011c3c8 => {
                c.stw(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c380: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetFightTrainerAiWazaValueFuncPtr (0x8011c3d0, 80 bytes)
#[inline(never)]
pub fn f_8011c3d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c3d0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c3d0); f_8011c3d0_b(c, 0x8011c3d0); crate::prof::exit(c, 0x8011c3d0); return; }
    }
    f_8011c3d0_b(c, 0x8011c3d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c3d0_at(c: &mut Cpu, pc: u32) { f_8011c3d0_b(c, pc) }
#[inline(always)]
fn f_8011c3d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c3d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c3d0, pc); }
        match pc {
            0x8011c3d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c3d8;
            }
            0x8011c3d8 => {
                if !c.crbit(2) { pc = 0x8011c3e4; continue; }
                pc = 0x8011c3dc;
            }
            0x8011c3dc => {
                c.r[3] = 0x0;
                pc = 0x8011c3e8; continue;
                pc = 0x8011c3e4;
            }
            0x8011c3e4 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                pc = 0x8011c3e8;
            }
            0x8011c3e8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8330));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011c400; continue; }
                pc = 0x8011c3f8;
            }
            0x8011c3f8 => {
                c.r[3] = 0x0;
                pc = 0x8011c410; continue;
                pc = 0x8011c400;
            }
            0x8011c400 => {
                c.r[5] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffc430);
                c.r[3] = c.r[0].wrapping_add(c.r[5]);
                pc = 0x8011c410;
            }
            0x8011c410 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c418;
            }
            0x8011c418 => {
                c.stw(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c3d0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetRiskFlag (0x8011c420, 16 bytes)
#[inline(never)]
pub fn f_8011c420(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c420) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c420); f_8011c420_b(c, 0x8011c420); crate::prof::exit(c, 0x8011c420); return; }
    }
    f_8011c420_b(c, 0x8011c420)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c420_at(c: &mut Cpu, pc: u32) { f_8011c420_b(c, pc) }
#[inline(always)]
fn f_8011c420_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c420;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c420, pc); }
        match pc {
            0x8011c420 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c428;
            }
            0x8011c428 => {
                c.stb(c.r[3].wrapping_add(0x13), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c420: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetTypeId (0x8011c430, 32 bytes)
#[inline(never)]
pub fn f_8011c430(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c430) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c430); f_8011c430_b(c, 0x8011c430); crate::prof::exit(c, 0x8011c430); return; }
    }
    f_8011c430_b(c, 0x8011c430)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c430_at(c: &mut Cpu, pc: u32) { f_8011c430_b(c, pc) }
#[inline(always)]
fn f_8011c430_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c430;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c430, pc); }
        match pc {
            0x8011c430 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c438;
            }
            0x8011c438 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x3);
                if !c.crbit(0) { return; }
                pc = 0x8011c444;
            }
            0x8011c444 => {
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x34), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c430: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetFightTrainerAiWazaDamageFuncPtr (0x8011c450, 96 bytes)
#[inline(never)]
pub fn f_8011c450(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c450) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c450); f_8011c450_b(c, 0x8011c450); crate::prof::exit(c, 0x8011c450); return; }
    }
    f_8011c450_b(c, 0x8011c450)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c450_at(c: &mut Cpu, pc: u32) { f_8011c450_b(c, pc) }
#[inline(always)]
fn f_8011c450_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c450;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c450, pc); }
        match pc {
            0x8011c450 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c460; continue; }
                pc = 0x8011c458;
            }
            0x8011c458 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c460;
            }
            0x8011c460 => {
                if !c.crbit(2) { pc = 0x8011c46c; continue; }
                pc = 0x8011c464;
            }
            0x8011c464 => {
                c.r[3] = 0x0;
                pc = 0x8011c470; continue;
                pc = 0x8011c46c;
            }
            0x8011c46c => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                pc = 0x8011c470;
            }
            0x8011c470 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8330));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011c488; continue; }
                pc = 0x8011c480;
            }
            0x8011c480 => {
                c.r[3] = 0x0;
                pc = 0x8011c498; continue;
                pc = 0x8011c488;
            }
            0x8011c488 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffc430);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8011c498;
            }
            0x8011c498 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c4a8; continue; }
                pc = 0x8011c4a0;
            }
            0x8011c4a0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c4a8;
            }
            0x8011c4a8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c450: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetFightTrainerAiWazaHitFuncPtr (0x8011c4b0, 96 bytes)
#[inline(never)]
pub fn f_8011c4b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c4b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c4b0); f_8011c4b0_b(c, 0x8011c4b0); crate::prof::exit(c, 0x8011c4b0); return; }
    }
    f_8011c4b0_b(c, 0x8011c4b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c4b0_at(c: &mut Cpu, pc: u32) { f_8011c4b0_b(c, pc) }
#[inline(always)]
fn f_8011c4b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c4b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c4b0, pc); }
        match pc {
            0x8011c4b0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c4c0; continue; }
                pc = 0x8011c4b8;
            }
            0x8011c4b8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c4c0;
            }
            0x8011c4c0 => {
                if !c.crbit(2) { pc = 0x8011c4cc; continue; }
                pc = 0x8011c4c4;
            }
            0x8011c4c4 => {
                c.r[3] = 0x0;
                pc = 0x8011c4d0; continue;
                pc = 0x8011c4cc;
            }
            0x8011c4cc => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                pc = 0x8011c4d0;
            }
            0x8011c4d0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8330));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011c4e8; continue; }
                pc = 0x8011c4e0;
            }
            0x8011c4e0 => {
                c.r[3] = 0x0;
                pc = 0x8011c4f8; continue;
                pc = 0x8011c4e8;
            }
            0x8011c4e8 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffc430);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8011c4f8;
            }
            0x8011c4f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c508; continue; }
                pc = 0x8011c500;
            }
            0x8011c500 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c508;
            }
            0x8011c508 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c4b0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetFightTrainerAiWazaValueFuncPtr (0x8011c510, 96 bytes)
#[inline(never)]
pub fn f_8011c510(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8011c510) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c510); f_8011c510_b(c, 0x8011c510); crate::prof::exit(c, 0x8011c510); return; }
    }
    f_8011c510_b(c, 0x8011c510)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c510_at(c: &mut Cpu, pc: u32) { f_8011c510_b(c, pc) }
#[inline(always)]
fn f_8011c510_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c510;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c510, pc); }
        match pc {
            0x8011c510 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c520; continue; }
                pc = 0x8011c518;
            }
            0x8011c518 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c520;
            }
            0x8011c520 => {
                if !c.crbit(2) { pc = 0x8011c52c; continue; }
                pc = 0x8011c524;
            }
            0x8011c524 => {
                c.r[3] = 0x0;
                pc = 0x8011c530; continue;
                pc = 0x8011c52c;
            }
            0x8011c52c => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                pc = 0x8011c530;
            }
            0x8011c530 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8330));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011c548; continue; }
                pc = 0x8011c540;
            }
            0x8011c540 => {
                c.r[3] = 0x0;
                pc = 0x8011c558; continue;
                pc = 0x8011c548;
            }
            0x8011c548 => {
                c.r[4] = c.r[3].wrapping_mul(0x14);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffc430);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x8011c558;
            }
            0x8011c558 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c568; continue; }
                pc = 0x8011c560;
            }
            0x8011c560 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c568;
            }
            0x8011c568 => {
                c.r[3] = c.lwz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c510: bad pc {:#x}", pc),
        }
    }
}

