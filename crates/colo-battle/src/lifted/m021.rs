#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// msgctrlSeOff (0x80131ba8, 8 bytes)
#[inline(never)]
pub fn f_80131ba8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131ba8) { return; }
        if c.prof { crate::prof::enter(c, 0x80131ba8); f_80131ba8_b(c, 0x80131ba8); crate::prof::exit(c, 0x80131ba8); return; }
    }
    f_80131ba8_b(c, 0x80131ba8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131ba8_at(c: &mut Cpu, pc: u32) { f_80131ba8_b(c, pc) }
#[inline(always)]
fn f_80131ba8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131ba8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131ba8, pc); }
        match pc {
            0x80131ba8 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80131ba8: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlPasoName (0x80131bb0, 8 bytes)
#[inline(never)]
pub fn f_80131bb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131bb0) { return; }
        if c.prof { crate::prof::enter(c, 0x80131bb0); f_80131bb0_b(c, 0x80131bb0); crate::prof::exit(c, 0x80131bb0); return; }
    }
    f_80131bb0_b(c, 0x80131bb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131bb0_at(c: &mut Cpu, pc: u32) { f_80131bb0_b(c, pc) }
#[inline(always)]
fn f_80131bb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131bb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131bb0, pc); }
        match pc {
            0x80131bb0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa620));
                if true { return; }
                return;
            }
            _ => panic!("f_80131bb0: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlItemName (0x80131bb8, 8 bytes)
#[inline(never)]
pub fn f_80131bb8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131bb8) { return; }
        if c.prof { crate::prof::enter(c, 0x80131bb8); f_80131bb8_b(c, 0x80131bb8); crate::prof::exit(c, 0x80131bb8); return; }
    }
    f_80131bb8_b(c, 0x80131bb8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131bb8_at(c: &mut Cpu, pc: u32) { f_80131bb8_b(c, pc) }
#[inline(always)]
fn f_80131bb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131bb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131bb8, pc); }
        match pc {
            0x80131bb8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa61c));
                if true { return; }
                return;
            }
            _ => panic!("f_80131bb8: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlWazaName (0x80131bc0, 8 bytes)
#[inline(never)]
pub fn f_80131bc0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131bc0) { return; }
        if c.prof { crate::prof::enter(c, 0x80131bc0); f_80131bc0_b(c, 0x80131bc0); crate::prof::exit(c, 0x80131bc0); return; }
    }
    f_80131bc0_b(c, 0x80131bc0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131bc0_at(c: &mut Cpu, pc: u32) { f_80131bc0_b(c, pc) }
#[inline(always)]
fn f_80131bc0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131bc0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131bc0, pc); }
        match pc {
            0x80131bc0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa618));
                if true { return; }
                return;
            }
            _ => panic!("f_80131bc0: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlTrainerClientno (0x80131bc8, 8 bytes)
#[inline(never)]
pub fn f_80131bc8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131bc8) { return; }
        if c.prof { crate::prof::enter(c, 0x80131bc8); f_80131bc8_b(c, 0x80131bc8); crate::prof::exit(c, 0x80131bc8); return; }
    }
    f_80131bc8_b(c, 0x80131bc8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131bc8_at(c: &mut Cpu, pc: u32) { f_80131bc8_b(c, pc) }
#[inline(always)]
fn f_80131bc8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131bc8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131bc8, pc); }
        match pc {
            0x80131bc8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa614));
                if true { return; }
                return;
            }
            _ => panic!("f_80131bc8: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlTrainerEnename2 (0x80131bd0, 8 bytes)
#[inline(never)]
pub fn f_80131bd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131bd0) { return; }
        if c.prof { crate::prof::enter(c, 0x80131bd0); f_80131bd0_b(c, 0x80131bd0); crate::prof::exit(c, 0x80131bd0); return; }
    }
    f_80131bd0_b(c, 0x80131bd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131bd0_at(c: &mut Cpu, pc: u32) { f_80131bd0_b(c, pc) }
#[inline(always)]
fn f_80131bd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131bd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131bd0, pc); }
        match pc {
            0x80131bd0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa610));
                if true { return; }
                return;
            }
            _ => panic!("f_80131bd0: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlTrainerEnename (0x80131bd8, 8 bytes)
#[inline(never)]
pub fn f_80131bd8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131bd8) { return; }
        if c.prof { crate::prof::enter(c, 0x80131bd8); f_80131bd8_b(c, 0x80131bd8); crate::prof::exit(c, 0x80131bd8); return; }
    }
    f_80131bd8_b(c, 0x80131bd8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131bd8_at(c: &mut Cpu, pc: u32) { f_80131bd8_b(c, pc) }
#[inline(always)]
fn f_80131bd8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131bd8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131bd8, pc); }
        match pc {
            0x80131bd8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa60c));
                if true { return; }
                return;
            }
            _ => panic!("f_80131bd8: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlTrainerLose (0x80131be0, 8 bytes)
#[inline(never)]
pub fn f_80131be0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131be0) { return; }
        if c.prof { crate::prof::enter(c, 0x80131be0); f_80131be0_b(c, 0x80131be0); crate::prof::exit(c, 0x80131be0); return; }
    }
    f_80131be0_b(c, 0x80131be0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131be0_at(c: &mut Cpu, pc: u32) { f_80131be0_b(c, pc) }
#[inline(always)]
fn f_80131be0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131be0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131be0, pc); }
        match pc {
            0x80131be0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa608));
                if true { return; }
                return;
            }
            _ => panic!("f_80131be0: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlTrainerName (0x80131be8, 8 bytes)
#[inline(never)]
pub fn f_80131be8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131be8) { return; }
        if c.prof { crate::prof::enter(c, 0x80131be8); f_80131be8_b(c, 0x80131be8); crate::prof::exit(c, 0x80131be8); return; }
    }
    f_80131be8_b(c, 0x80131be8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131be8_at(c: &mut Cpu, pc: u32) { f_80131be8_b(c, pc) }
#[inline(always)]
fn f_80131be8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131be8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131be8, pc); }
        match pc {
            0x80131be8 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa604));
                if true { return; }
                return;
            }
            _ => panic!("f_80131be8: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlTrainerType (0x80131bf0, 8 bytes)
#[inline(never)]
pub fn f_80131bf0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131bf0) { return; }
        if c.prof { crate::prof::enter(c, 0x80131bf0); f_80131bf0_b(c, 0x80131bf0); crate::prof::exit(c, 0x80131bf0); return; }
    }
    f_80131bf0_b(c, 0x80131bf0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131bf0_at(c: &mut Cpu, pc: u32) { f_80131bf0_b(c, pc) }
#[inline(always)]
fn f_80131bf0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131bf0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131bf0, pc); }
        match pc {
            0x80131bf0 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa600));
                if true { return; }
                return;
            }
            _ => panic!("f_80131bf0: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSideDefenceNameno (0x80131bf8, 40 bytes)
#[inline(never)]
pub fn f_80131bf8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131bf8) { return; }
        if c.prof { crate::prof::enter(c, 0x80131bf8); f_80131bf8_b(c, 0x80131bf8); crate::prof::exit(c, 0x80131bf8); return; }
    }
    f_80131bf8_b(c, 0x80131bf8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131bf8_at(c: &mut Cpu, pc: u32) { f_80131bf8_b(c, pc) }
#[inline(always)]
fn f_80131bf8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131bf8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131bf8, pc); }
        match pc {
            0x80131bf8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa62c));
                c.r[4] = 0x2;
                c.lr = 0x80131c10; f_80131ce8(c);
                pc = 0x80131c10;
            }
            0x80131c10 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80131bf8: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSideDefenceNamewo (0x80131c20, 40 bytes)
#[inline(never)]
pub fn f_80131c20(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131c20) { return; }
        if c.prof { crate::prof::enter(c, 0x80131c20); f_80131c20_b(c, 0x80131c20); crate::prof::exit(c, 0x80131c20); return; }
    }
    f_80131c20_b(c, 0x80131c20)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131c20_at(c: &mut Cpu, pc: u32) { f_80131c20_b(c, pc) }
#[inline(always)]
fn f_80131c20_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131c20;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131c20, pc); }
        match pc {
            0x80131c20 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa628));
                c.r[4] = 0x1;
                c.lr = 0x80131c38; f_80131ce8(c);
                pc = 0x80131c38;
            }
            0x80131c38 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80131c20: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSideDefenceNameha (0x80131c48, 40 bytes)
#[inline(never)]
pub fn f_80131c48(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131c48) { return; }
        if c.prof { crate::prof::enter(c, 0x80131c48); f_80131c48_b(c, 0x80131c48); crate::prof::exit(c, 0x80131c48); return; }
    }
    f_80131c48_b(c, 0x80131c48)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131c48_at(c: &mut Cpu, pc: u32) { f_80131c48_b(c, pc) }
#[inline(always)]
fn f_80131c48_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131c48;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131c48, pc); }
        match pc {
            0x80131c48 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa624));
                c.r[4] = 0x0;
                c.lr = 0x80131c60; f_80131ce8(c);
                pc = 0x80131c60;
            }
            0x80131c60 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80131c48: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSideAttackNameno (0x80131c70, 40 bytes)
#[inline(never)]
pub fn f_80131c70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131c70) { return; }
        if c.prof { crate::prof::enter(c, 0x80131c70); f_80131c70_b(c, 0x80131c70); crate::prof::exit(c, 0x80131c70); return; }
    }
    f_80131c70_b(c, 0x80131c70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131c70_at(c: &mut Cpu, pc: u32) { f_80131c70_b(c, pc) }
#[inline(always)]
fn f_80131c70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131c70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131c70, pc); }
        match pc {
            0x80131c70 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5fc));
                c.r[4] = 0x2;
                c.lr = 0x80131c88; f_80131ce8(c);
                pc = 0x80131c88;
            }
            0x80131c88 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80131c70: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSideAttackNamewo (0x80131c98, 40 bytes)
#[inline(never)]
pub fn f_80131c98(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131c98) { return; }
        if c.prof { crate::prof::enter(c, 0x80131c98); f_80131c98_b(c, 0x80131c98); crate::prof::exit(c, 0x80131c98); return; }
    }
    f_80131c98_b(c, 0x80131c98)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131c98_at(c: &mut Cpu, pc: u32) { f_80131c98_b(c, pc) }
#[inline(always)]
fn f_80131c98_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131c98;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131c98, pc); }
        match pc {
            0x80131c98 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5f8));
                c.r[4] = 0x1;
                c.lr = 0x80131cb0; f_80131ce8(c);
                pc = 0x80131cb0;
            }
            0x80131cb0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80131c98: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSideAttackNameha (0x80131cc0, 40 bytes)
#[inline(never)]
pub fn f_80131cc0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131cc0) { return; }
        if c.prof { crate::prof::enter(c, 0x80131cc0); f_80131cc0_b(c, 0x80131cc0); crate::prof::exit(c, 0x80131cc0); return; }
    }
    f_80131cc0_b(c, 0x80131cc0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131cc0_at(c: &mut Cpu, pc: u32) { f_80131cc0_b(c, pc) }
#[inline(always)]
fn f_80131cc0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131cc0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131cc0, pc); }
        match pc {
            0x80131cc0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5f4));
                c.r[4] = 0x0;
                c.lr = 0x80131cd8; f_80131ce8(c);
                pc = 0x80131cd8;
            }
            0x80131cd8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80131cc0: bad pc {:#x}", pc),
        }
    }
}

/// _msgctrlSideName__FP15FightOutPokemonUc (0x80131ce8, 540 bytes)
#[inline(never)]
pub fn f_80131ce8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131ce8) { return; }
        if c.prof { crate::prof::enter(c, 0x80131ce8); f_80131ce8_b(c, 0x80131ce8); crate::prof::exit(c, 0x80131ce8); return; }
    }
    f_80131ce8_b(c, 0x80131ce8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131ce8_at(c: &mut Cpu, pc: u32) { f_80131ce8_b(c, pc) }
#[inline(always)]
fn f_80131ce8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131ce8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131ce8, pc); }
        match pc {
            0x80131ce8 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[24] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x80131d14; f_801f54a4(c);
                pc = 0x80131d14;
            }
            0x80131d14 => {
                c.r[30] = c.r[3].rotate_left(0) & 0xffff;
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x2;
                c.lr = 0x80131d24; f_801f025c(c);
                pc = 0x80131d24;
            }
            0x80131d24 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = c.r[24] | c.r[24];
                c.r[3] = 0x0;
                c.lr = 0x80131d34; f_801f4354(c);
                pc = 0x80131d34;
            }
            0x80131d34 => {
                c.r[26] = 0x0;
                c.r[28] = c.r[3] | c.r[3];
                c.r[25] = 0x0;
                pc = 0x80131db4; continue;
                pc = 0x80131d44;
            }
            0x80131d44 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[25] | c.r[25];
                c.lr = 0x80131d50; f_801f7258(c);
                pc = 0x80131d50;
            }
            0x80131d50 => {
                c.r[27] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[27], 0x0);
                if c.crbit(2) { pc = 0x80131db0; continue; }
                pc = 0x80131d5c;
            }
            0x80131d5c => {
                c.lr = 0x80131d60; f_801fa524(c);
                pc = 0x80131d60;
            }
            0x80131d60 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80131db0; continue; }
                pc = 0x80131d6c;
            }
            0x80131d6c => {
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80131d90; continue; }
                pc = 0x80131d78;
            }
            0x80131d78 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x80131d80; f_801f8100(c);
                pc = 0x80131d80;
            }
            0x80131d80 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x4d;
                c.lr = 0x80131d8c; f_80132a38(c);
                pc = 0x80131d8c;
            }
            0x80131d8c => {
                pc = 0x80131dac; continue;
                pc = 0x80131d90;
            }
            0x80131d90 => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80131dac; continue; }
                pc = 0x80131d98;
            }
            0x80131d98 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x80131da0; f_801f8100(c);
                pc = 0x80131da0;
            }
            0x80131da0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x57;
                c.lr = 0x80131dac; f_80132a38(c);
                pc = 0x80131dac;
            }
            0x80131dac => {
                c.r[26] = c.r[26].wrapping_add(0x1);
                pc = 0x80131db0;
            }
            0x80131db0 => {
                c.r[25] = c.r[25].wrapping_add(0x1);
                pc = 0x80131db4;
            }
            0x80131db4 => {
                c.r[0] = c.r[25].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x80131d44; continue; }
                pc = 0x80131dc0;
            }
            0x80131dc0 => {
                c.r[3] = 0x0;
                c.lr = 0x80131dc8; f_801f18dc(c);
                pc = 0x80131dc8;
            }
            0x80131dc8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80131e6c; continue; }
                pc = 0x80131dd4;
            }
            0x80131dd4 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x80131e6c; continue; }
                pc = 0x80131ddc;
            }
            0x80131ddc => {
                c.r[0] = c.r[26].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(1) { pc = 0x80131e34; continue; }
                pc = 0x80131de8;
            }
            0x80131de8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80131df0; f_801f8100(c);
                pc = 0x80131df0;
            }
            0x80131df0 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x4d;
                c.lr = 0x80131dfc; f_80132a38(c);
                pc = 0x80131dfc;
            }
            0x80131dfc => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80131e14; continue; }
                pc = 0x80131e08;
            }
            0x80131e08 => {
                c.r[3] = 0x7722;
                c.lr = 0x80131e10; f_800fa280(c);
                pc = 0x80131e10;
            }
            0x80131e10 => {
                pc = 0x80131ef0; continue;
                pc = 0x80131e14;
            }
            0x80131e14 => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80131e28; continue; }
                pc = 0x80131e1c;
            }
            0x80131e1c => {
                c.r[3] = 0x7725;
                c.lr = 0x80131e24; f_800fa280(c);
                pc = 0x80131e24;
            }
            0x80131e24 => {
                pc = 0x80131ef0; continue;
                pc = 0x80131e28;
            }
            0x80131e28 => {
                c.r[3] = 0x7727;
                c.lr = 0x80131e30; f_800fa280(c);
                pc = 0x80131e30;
            }
            0x80131e30 => {
                pc = 0x80131ef0; continue;
                pc = 0x80131e34;
            }
            0x80131e34 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80131e4c; continue; }
                pc = 0x80131e40;
            }
            0x80131e40 => {
                c.r[3] = 0x7724;
                c.lr = 0x80131e48; f_800fa280(c);
                pc = 0x80131e48;
            }
            0x80131e48 => {
                pc = 0x80131ef0; continue;
                pc = 0x80131e4c;
            }
            0x80131e4c => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80131e60; continue; }
                pc = 0x80131e54;
            }
            0x80131e54 => {
                c.r[3] = 0x7726;
                c.lr = 0x80131e5c; f_800fa280(c);
                pc = 0x80131e5c;
            }
            0x80131e5c => {
                pc = 0x80131ef0; continue;
                pc = 0x80131e60;
            }
            0x80131e60 => {
                c.r[3] = 0x7728;
                c.lr = 0x80131e68; f_800fa280(c);
                pc = 0x80131e68;
            }
            0x80131e68 => {
                pc = 0x80131ef0; continue;
                pc = 0x80131e6c;
            }
            0x80131e6c => {
                c.r[3] = c.r[24] | c.r[24];
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80131e78; f_801f0058(c);
                pc = 0x80131e78;
            }
            0x80131e78 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80131ebc; continue; }
                pc = 0x80131e84;
            }
            0x80131e84 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80131e9c; continue; }
                pc = 0x80131e90;
            }
            0x80131e90 => {
                c.r[3] = 0x768a;
                c.lr = 0x80131e98; f_800fa280(c);
                pc = 0x80131e98;
            }
            0x80131e98 => {
                pc = 0x80131ef0; continue;
                pc = 0x80131e9c;
            }
            0x80131e9c => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80131eb0; continue; }
                pc = 0x80131ea4;
            }
            0x80131ea4 => {
                c.r[3] = 0x768c;
                c.lr = 0x80131eac; f_800fa280(c);
                pc = 0x80131eac;
            }
            0x80131eac => {
                pc = 0x80131ef0; continue;
                pc = 0x80131eb0;
            }
            0x80131eb0 => {
                c.r[3] = 0x7688;
                c.lr = 0x80131eb8; f_800fa280(c);
                pc = 0x80131eb8;
            }
            0x80131eb8 => {
                pc = 0x80131ef0; continue;
                pc = 0x80131ebc;
            }
            0x80131ebc => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80131ed4; continue; }
                pc = 0x80131ec8;
            }
            0x80131ec8 => {
                c.r[3] = 0x7689;
                c.lr = 0x80131ed0; f_800fa280(c);
                pc = 0x80131ed0;
            }
            0x80131ed0 => {
                pc = 0x80131ef0; continue;
                pc = 0x80131ed4;
            }
            0x80131ed4 => {
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80131ee8; continue; }
                pc = 0x80131edc;
            }
            0x80131edc => {
                c.r[3] = 0x768b;
                c.lr = 0x80131ee4; f_800fa280(c);
                pc = 0x80131ee4;
            }
            0x80131ee4 => {
                pc = 0x80131ef0; continue;
                pc = 0x80131ee8;
            }
            0x80131ee8 => {
                c.r[3] = 0x7687;
                c.lr = 0x80131ef0; f_800fa280(c);
                pc = 0x80131ef0;
            }
            0x80131ef0 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80131ce8: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlClientnowork (0x80131f04, 152 bytes)
#[inline(never)]
pub fn f_80131f04(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131f04) { return; }
        if c.prof { crate::prof::enter(c, 0x80131f04); f_80131f04_b(c, 0x80131f04); crate::prof::exit(c, 0x80131f04); return; }
    }
    f_80131f04_b(c, 0x80131f04)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131f04_at(c: &mut Cpu, pc: u32) { f_80131f04_b(c, pc) }
#[inline(always)]
fn f_80131f04_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131f04;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131f04, pc); }
        match pc {
            0x80131f04 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa5f0));
                c.r[3] = 0x0;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80131f28; f_801f4354(c);
                pc = 0x80131f28;
            }
            0x80131f28 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80131f34; f_801f18dc(c);
                pc = 0x80131f34;
            }
            0x80131f34 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80131f7c; continue; }
                pc = 0x80131f40;
            }
            0x80131f40 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80131f7c; continue; }
                pc = 0x80131f48;
            }
            0x80131f48 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80131f50; f_801f8100(c);
                pc = 0x80131f50;
            }
            0x80131f50 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x4d;
                c.lr = 0x80131f5c; f_80132a38(c);
                pc = 0x80131f5c;
            }
            0x80131f5c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80131f64; f_802037dc(c);
                pc = 0x80131f64;
            }
            0x80131f64 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x57;
                c.lr = 0x80131f70; f_80132a38(c);
                pc = 0x80131f70;
            }
            0x80131f70 => {
                c.r[3] = 0x7721;
                c.lr = 0x80131f78; f_800fa280(c);
                pc = 0x80131f78;
            }
            0x80131f78 => {
                pc = 0x80131f84; continue;
                pc = 0x80131f7c;
            }
            0x80131f7c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80131f84; f_802037dc(c);
                pc = 0x80131f84;
            }
            0x80131f84 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80131f04: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSpeabiNamet (0x80131f9c, 8 bytes)
#[inline(never)]
pub fn f_80131f9c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131f9c) { return; }
        if c.prof { crate::prof::enter(c, 0x80131f9c); f_80131f9c_b(c, 0x80131f9c); crate::prof::exit(c, 0x80131f9c); return; }
    }
    f_80131f9c_b(c, 0x80131f9c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131f9c_at(c: &mut Cpu, pc: u32) { f_80131f9c_b(c, pc) }
#[inline(always)]
fn f_80131f9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131f9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131f9c, pc); }
        match pc {
            0x80131f9c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5ec));
                if true { return; }
                return;
            }
            _ => panic!("f_80131f9c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSpeabiNamec (0x80131fa4, 8 bytes)
#[inline(never)]
pub fn f_80131fa4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131fa4) { return; }
        if c.prof { crate::prof::enter(c, 0x80131fa4); f_80131fa4_b(c, 0x80131fa4); crate::prof::exit(c, 0x80131fa4); return; }
    }
    f_80131fa4_b(c, 0x80131fa4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131fa4_at(c: &mut Cpu, pc: u32) { f_80131fa4_b(c, pc) }
#[inline(always)]
fn f_80131fa4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131fa4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131fa4, pc); }
        match pc {
            0x80131fa4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5e8));
                if true { return; }
                return;
            }
            _ => panic!("f_80131fa4: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSpeabiNamed (0x80131fac, 8 bytes)
#[inline(never)]
pub fn f_80131fac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131fac) { return; }
        if c.prof { crate::prof::enter(c, 0x80131fac); f_80131fac_b(c, 0x80131fac); crate::prof::exit(c, 0x80131fac); return; }
    }
    f_80131fac_b(c, 0x80131fac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131fac_at(c: &mut Cpu, pc: u32) { f_80131fac_b(c, pc) }
#[inline(always)]
fn f_80131fac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131fac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131fac, pc); }
        match pc {
            0x80131fac => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5e4));
                if true { return; }
                return;
            }
            _ => panic!("f_80131fac: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSpeabiNamea (0x80131fb4, 8 bytes)
#[inline(never)]
pub fn f_80131fb4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131fb4) { return; }
        if c.prof { crate::prof::enter(c, 0x80131fb4); f_80131fb4_b(c, 0x80131fb4); crate::prof::exit(c, 0x80131fb4); return; }
    }
    f_80131fb4_b(c, 0x80131fb4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131fb4_at(c: &mut Cpu, pc: u32) { f_80131fb4_b(c, pc) }
#[inline(always)]
fn f_80131fb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131fb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131fb4, pc); }
        match pc {
            0x80131fb4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5e0));
                if true { return; }
                return;
            }
            _ => panic!("f_80131fb4: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlEnemyTmons2 (0x80131fbc, 8 bytes)
#[inline(never)]
pub fn f_80131fbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131fbc) { return; }
        if c.prof { crate::prof::enter(c, 0x80131fbc); f_80131fbc_b(c, 0x80131fbc); crate::prof::exit(c, 0x80131fbc); return; }
    }
    f_80131fbc_b(c, 0x80131fbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131fbc_at(c: &mut Cpu, pc: u32) { f_80131fbc_b(c, pc) }
#[inline(always)]
fn f_80131fbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131fbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131fbc, pc); }
        match pc {
            0x80131fbc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5dc));
                if true { return; }
                return;
            }
            _ => panic!("f_80131fbc: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlEnemyTmons (0x80131fc4, 8 bytes)
#[inline(never)]
pub fn f_80131fc4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131fc4) { return; }
        if c.prof { crate::prof::enter(c, 0x80131fc4); f_80131fc4_b(c, 0x80131fc4); crate::prof::exit(c, 0x80131fc4); return; }
    }
    f_80131fc4_b(c, 0x80131fc4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131fc4_at(c: &mut Cpu, pc: u32) { f_80131fc4_b(c, pc) }
#[inline(always)]
fn f_80131fc4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131fc4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131fc4, pc); }
        match pc {
            0x80131fc4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5d8));
                if true { return; }
                return;
            }
            _ => panic!("f_80131fc4: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlEnemyMons2 (0x80131fcc, 8 bytes)
#[inline(never)]
pub fn f_80131fcc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131fcc) { return; }
        if c.prof { crate::prof::enter(c, 0x80131fcc); f_80131fcc_b(c, 0x80131fcc); crate::prof::exit(c, 0x80131fcc); return; }
    }
    f_80131fcc_b(c, 0x80131fcc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131fcc_at(c: &mut Cpu, pc: u32) { f_80131fcc_b(c, pc) }
#[inline(always)]
fn f_80131fcc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131fcc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131fcc, pc); }
        match pc {
            0x80131fcc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5d4));
                if true { return; }
                return;
            }
            _ => panic!("f_80131fcc: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlEnemyMons (0x80131fd4, 8 bytes)
#[inline(never)]
pub fn f_80131fd4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131fd4) { return; }
        if c.prof { crate::prof::enter(c, 0x80131fd4); f_80131fd4_b(c, 0x80131fd4); crate::prof::exit(c, 0x80131fd4); return; }
    }
    f_80131fd4_b(c, 0x80131fd4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131fd4_at(c: &mut Cpu, pc: u32) { f_80131fd4_b(c, pc) }
#[inline(always)]
fn f_80131fd4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131fd4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131fd4, pc); }
        match pc {
            0x80131fd4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5d0));
                if true { return; }
                return;
            }
            _ => panic!("f_80131fd4: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlMyMons2 (0x80131fdc, 8 bytes)
#[inline(never)]
pub fn f_80131fdc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131fdc) { return; }
        if c.prof { crate::prof::enter(c, 0x80131fdc); f_80131fdc_b(c, 0x80131fdc); crate::prof::exit(c, 0x80131fdc); return; }
    }
    f_80131fdc_b(c, 0x80131fdc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131fdc_at(c: &mut Cpu, pc: u32) { f_80131fdc_b(c, pc) }
#[inline(always)]
fn f_80131fdc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131fdc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131fdc, pc); }
        match pc {
            0x80131fdc => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5cc));
                if true { return; }
                return;
            }
            _ => panic!("f_80131fdc: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlMyMons (0x80131fe4, 8 bytes)
#[inline(never)]
pub fn f_80131fe4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131fe4) { return; }
        if c.prof { crate::prof::enter(c, 0x80131fe4); f_80131fe4_b(c, 0x80131fe4); crate::prof::exit(c, 0x80131fe4); return; }
    }
    f_80131fe4_b(c, 0x80131fe4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131fe4_at(c: &mut Cpu, pc: u32) { f_80131fe4_b(c, pc) }
#[inline(always)]
fn f_80131fe4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131fe4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131fe4, pc); }
        match pc {
            0x80131fe4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5c8));
                if true { return; }
                return;
            }
            _ => panic!("f_80131fe4: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlMyName (0x80131fec, 8 bytes)
#[inline(never)]
pub fn f_80131fec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131fec) { return; }
        if c.prof { crate::prof::enter(c, 0x80131fec); f_80131fec_b(c, 0x80131fec); crate::prof::exit(c, 0x80131fec); return; }
    }
    f_80131fec_b(c, 0x80131fec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131fec_at(c: &mut Cpu, pc: u32) { f_80131fec_b(c, pc) }
#[inline(always)]
fn f_80131fec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131fec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131fec, pc); }
        match pc {
            0x80131fec => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5c4));
                if true { return; }
                return;
            }
            _ => panic!("f_80131fec: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlTsuikaMons (0x80131ff4, 152 bytes)
#[inline(never)]
pub fn f_80131ff4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80131ff4) { return; }
        if c.prof { crate::prof::enter(c, 0x80131ff4); f_80131ff4_b(c, 0x80131ff4); crate::prof::exit(c, 0x80131ff4); return; }
    }
    f_80131ff4_b(c, 0x80131ff4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80131ff4_at(c: &mut Cpu, pc: u32) { f_80131ff4_b(c, pc) }
#[inline(always)]
fn f_80131ff4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80131ff4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80131ff4, pc); }
        match pc {
            0x80131ff4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa5c0));
                c.r[3] = 0x0;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80132018; f_801f4354(c);
                pc = 0x80132018;
            }
            0x80132018 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80132024; f_801f18dc(c);
                pc = 0x80132024;
            }
            0x80132024 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8013206c; continue; }
                pc = 0x80132030;
            }
            0x80132030 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8013206c; continue; }
                pc = 0x80132038;
            }
            0x80132038 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80132040; f_801f8100(c);
                pc = 0x80132040;
            }
            0x80132040 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x4d;
                c.lr = 0x8013204c; f_80132a38(c);
                pc = 0x8013204c;
            }
            0x8013204c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80132054; f_802037dc(c);
                pc = 0x80132054;
            }
            0x80132054 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x57;
                c.lr = 0x80132060; f_80132a38(c);
                pc = 0x80132060;
            }
            0x80132060 => {
                c.r[3] = 0x7721;
                c.lr = 0x80132068; f_800fa280(c);
                pc = 0x80132068;
            }
            0x80132068 => {
                pc = 0x80132074; continue;
                pc = 0x8013206c;
            }
            0x8013206c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80132074; f_802037dc(c);
                pc = 0x80132074;
            }
            0x80132074 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80131ff4: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlClientMos (0x8013208c, 152 bytes)
#[inline(never)]
pub fn f_8013208c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013208c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013208c); f_8013208c_b(c, 0x8013208c); crate::prof::exit(c, 0x8013208c); return; }
    }
    f_8013208c_b(c, 0x8013208c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013208c_at(c: &mut Cpu, pc: u32) { f_8013208c_b(c, pc) }
#[inline(always)]
fn f_8013208c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013208c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013208c, pc); }
        match pc {
            0x8013208c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa5bc));
                c.r[3] = 0x0;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x801320b0; f_801f4354(c);
                pc = 0x801320b0;
            }
            0x801320b0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x801320bc; f_801f18dc(c);
                pc = 0x801320bc;
            }
            0x801320bc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80132104; continue; }
                pc = 0x801320c8;
            }
            0x801320c8 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80132104; continue; }
                pc = 0x801320d0;
            }
            0x801320d0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801320d8; f_801f8100(c);
                pc = 0x801320d8;
            }
            0x801320d8 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x4d;
                c.lr = 0x801320e4; f_80132a38(c);
                pc = 0x801320e4;
            }
            0x801320e4 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801320ec; f_802037dc(c);
                pc = 0x801320ec;
            }
            0x801320ec => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x57;
                c.lr = 0x801320f8; f_80132a38(c);
                pc = 0x801320f8;
            }
            0x801320f8 => {
                c.r[3] = 0x7721;
                c.lr = 0x80132100; f_800fa280(c);
                pc = 0x80132100;
            }
            0x80132100 => {
                pc = 0x8013210c; continue;
                pc = 0x80132104;
            }
            0x80132104 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8013210c; f_802037dc(c);
                pc = 0x8013210c;
            }
            0x8013210c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8013208c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlDeffenceMons (0x80132124, 152 bytes)
#[inline(never)]
pub fn f_80132124(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132124) { return; }
        if c.prof { crate::prof::enter(c, 0x80132124); f_80132124_b(c, 0x80132124); crate::prof::exit(c, 0x80132124); return; }
    }
    f_80132124_b(c, 0x80132124)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132124_at(c: &mut Cpu, pc: u32) { f_80132124_b(c, pc) }
#[inline(always)]
fn f_80132124_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132124;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132124, pc); }
        match pc {
            0x80132124 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa5b8));
                c.r[3] = 0x0;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x80132148; f_801f4354(c);
                pc = 0x80132148;
            }
            0x80132148 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x80132154; f_801f18dc(c);
                pc = 0x80132154;
            }
            0x80132154 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8013219c; continue; }
                pc = 0x80132160;
            }
            0x80132160 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x8013219c; continue; }
                pc = 0x80132168;
            }
            0x80132168 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80132170; f_801f8100(c);
                pc = 0x80132170;
            }
            0x80132170 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x4d;
                c.lr = 0x8013217c; f_80132a38(c);
                pc = 0x8013217c;
            }
            0x8013217c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80132184; f_802037dc(c);
                pc = 0x80132184;
            }
            0x80132184 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x57;
                c.lr = 0x80132190; f_80132a38(c);
                pc = 0x80132190;
            }
            0x80132190 => {
                c.r[3] = 0x7721;
                c.lr = 0x80132198; f_800fa280(c);
                pc = 0x80132198;
            }
            0x80132198 => {
                pc = 0x801321a4; continue;
                pc = 0x8013219c;
            }
            0x8013219c => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x801321a4; f_802037dc(c);
                pc = 0x801321a4;
            }
            0x801321a4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80132124: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlAttackMons (0x801321bc, 152 bytes)
#[inline(never)]
pub fn f_801321bc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801321bc) { return; }
        if c.prof { crate::prof::enter(c, 0x801321bc); f_801321bc_b(c, 0x801321bc); crate::prof::exit(c, 0x801321bc); return; }
    }
    f_801321bc_b(c, 0x801321bc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801321bc_at(c: &mut Cpu, pc: u32) { f_801321bc_b(c, pc) }
#[inline(always)]
fn f_801321bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801321bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801321bc, pc); }
        match pc {
            0x801321bc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.lwz(c.r[13].wrapping_add(0xffffa5b4));
                c.r[3] = 0x0;
                c.r[4] = c.r[30] | c.r[30];
                c.lr = 0x801321e0; f_801f4354(c);
                pc = 0x801321e0;
            }
            0x801321e0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x801321ec; f_801f18dc(c);
                pc = 0x801321ec;
            }
            0x801321ec => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80132234; continue; }
                pc = 0x801321f8;
            }
            0x801321f8 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80132234; continue; }
                pc = 0x80132200;
            }
            0x80132200 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80132208; f_801f8100(c);
                pc = 0x80132208;
            }
            0x80132208 => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x4d;
                c.lr = 0x80132214; f_80132a38(c);
                pc = 0x80132214;
            }
            0x80132214 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8013221c; f_802037dc(c);
                pc = 0x8013221c;
            }
            0x8013221c => {
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = 0x57;
                c.lr = 0x80132228; f_80132a38(c);
                pc = 0x80132228;
            }
            0x80132228 => {
                c.r[3] = 0x7721;
                c.lr = 0x80132230; f_800fa280(c);
                pc = 0x80132230;
            }
            0x80132230 => {
                pc = 0x8013223c; continue;
                pc = 0x80132234;
            }
            0x80132234 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x8013223c; f_802037dc(c);
                pc = 0x8013223c;
            }
            0x8013223c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801321bc: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlEvStrBuf2 (0x80132254, 8 bytes)
#[inline(never)]
pub fn f_80132254(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132254) { return; }
        if c.prof { crate::prof::enter(c, 0x80132254); f_80132254_b(c, 0x80132254); crate::prof::exit(c, 0x80132254); return; }
    }
    f_80132254_b(c, 0x80132254)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132254_at(c: &mut Cpu, pc: u32) { f_80132254_b(c, pc) }
#[inline(always)]
fn f_80132254_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132254;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132254, pc); }
        match pc {
            0x80132254 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5b0));
                if true { return; }
                return;
            }
            _ => panic!("f_80132254: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlEvStrBuf1 (0x8013225c, 8 bytes)
#[inline(never)]
pub fn f_8013225c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013225c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013225c); f_8013225c_b(c, 0x8013225c); crate::prof::exit(c, 0x8013225c); return; }
    }
    f_8013225c_b(c, 0x8013225c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013225c_at(c: &mut Cpu, pc: u32) { f_8013225c_b(c, pc) }
#[inline(always)]
fn f_8013225c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013225c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013225c, pc); }
        match pc {
            0x8013225c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5ac));
                if true { return; }
                return;
            }
            _ => panic!("f_8013225c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlEvStrBuf0 (0x80132264, 8 bytes)
#[inline(never)]
pub fn f_80132264(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132264) { return; }
        if c.prof { crate::prof::enter(c, 0x80132264); f_80132264_b(c, 0x80132264); crate::prof::exit(c, 0x80132264); return; }
    }
    f_80132264_b(c, 0x80132264)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132264_at(c: &mut Cpu, pc: u32) { f_80132264_b(c, pc) }
#[inline(always)]
fn f_80132264_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132264;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132264, pc); }
        match pc {
            0x80132264 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa5a8));
                if true { return; }
                return;
            }
            _ => panic!("f_80132264: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlWaza (0x8013226c, 40 bytes)
#[inline(never)]
pub fn f_8013226c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013226c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013226c); f_8013226c_b(c, 0x8013226c); crate::prof::exit(c, 0x8013226c); return; }
    }
    f_8013226c_b(c, 0x8013226c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013226c_at(c: &mut Cpu, pc: u32) { f_8013226c_b(c, pc) }
#[inline(always)]
fn f_8013226c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013226c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013226c, pc); }
        match pc {
            0x8013226c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lhz(c.r[13].wrapping_add(0xffffa65c));
                c.lr = 0x80132280; f_8011ca34(c);
                pc = 0x80132280;
            }
            0x80132280 => {
                c.lr = 0x80132284; f_8011ca1c(c);
                pc = 0x80132284;
            }
            0x80132284 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8013226c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlMenuMsg2 (0x80132294, 8 bytes)
#[inline(never)]
pub fn f_80132294(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132294) { return; }
        if c.prof { crate::prof::enter(c, 0x80132294); f_80132294_b(c, 0x80132294); crate::prof::exit(c, 0x80132294); return; }
    }
    f_80132294_b(c, 0x80132294)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132294_at(c: &mut Cpu, pc: u32) { f_80132294_b(c, pc) }
#[inline(always)]
fn f_80132294_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132294;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132294, pc); }
        match pc {
            0x80132294 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa658));
                if true { return; }
                return;
            }
            _ => panic!("f_80132294: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlMenuMsg (0x8013229c, 8 bytes)
#[inline(never)]
pub fn f_8013229c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013229c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013229c); f_8013229c_b(c, 0x8013229c); crate::prof::exit(c, 0x8013229c); return; }
    }
    f_8013229c_b(c, 0x8013229c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013229c_at(c: &mut Cpu, pc: u32) { f_8013229c_b(c, pc) }
#[inline(always)]
fn f_8013229c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013229c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013229c, pc); }
        match pc {
            0x8013229c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa654));
                if true { return; }
                return;
            }
            _ => panic!("f_8013229c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlMenuPokemon (0x801322a4, 8 bytes)
#[inline(never)]
pub fn f_801322a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801322a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801322a4); f_801322a4_b(c, 0x801322a4); crate::prof::exit(c, 0x801322a4); return; }
    }
    f_801322a4_b(c, 0x801322a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801322a4_at(c: &mut Cpu, pc: u32) { f_801322a4_b(c, pc) }
#[inline(always)]
fn f_801322a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801322a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801322a4, pc); }
        match pc {
            0x801322a4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa650));
                if true { return; }
                return;
            }
            _ => panic!("f_801322a4: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlMenuDigit2 (0x801322ac, 52 bytes)
#[inline(never)]
pub fn f_801322ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801322ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801322ac); f_801322ac_b(c, 0x801322ac); crate::prof::exit(c, 0x801322ac); return; }
    }
    f_801322ac_b(c, 0x801322ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801322ac_at(c: &mut Cpu, pc: u32) { f_801322ac_b(c, pc) }
#[inline(always)]
fn f_801322ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801322ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801322ac, pc); }
        match pc {
            0x801322ac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = 0x80420000;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa64c));
                c.r[3] = c.r[3].wrapping_add(0x7150);
                c.r[4] = 0x10;
                c.r[6] = 0x0;
                c.lr = 0x801322d0; f_80132834(c);
                pc = 0x801322d0;
            }
            0x801322d0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801322ac: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlMenuDigit (0x801322e0, 52 bytes)
#[inline(never)]
pub fn f_801322e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801322e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801322e0); f_801322e0_b(c, 0x801322e0); crate::prof::exit(c, 0x801322e0); return; }
    }
    f_801322e0_b(c, 0x801322e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801322e0_at(c: &mut Cpu, pc: u32) { f_801322e0_b(c, pc) }
#[inline(always)]
fn f_801322e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801322e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801322e0, pc); }
        match pc {
            0x801322e0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = 0x80420000;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa648));
                c.r[3] = c.r[3].wrapping_add(0x7170);
                c.r[4] = 0x10;
                c.r[6] = 0x0;
                c.lr = 0x80132304; f_80132834(c);
                pc = 0x80132304;
            }
            0x80132304 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801322e0: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlPokemon2 (0x80132314, 8 bytes)
#[inline(never)]
pub fn f_80132314(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132314) { return; }
        if c.prof { crate::prof::enter(c, 0x80132314); f_80132314_b(c, 0x80132314); crate::prof::exit(c, 0x80132314); return; }
    }
    f_80132314_b(c, 0x80132314)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132314_at(c: &mut Cpu, pc: u32) { f_80132314_b(c, pc) }
#[inline(always)]
fn f_80132314_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132314;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132314, pc); }
        match pc {
            0x80132314 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa644));
                if true { return; }
                return;
            }
            _ => panic!("f_80132314: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlPokemon (0x8013231c, 8 bytes)
#[inline(never)]
pub fn f_8013231c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013231c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013231c); f_8013231c_b(c, 0x8013231c); crate::prof::exit(c, 0x8013231c); return; }
    }
    f_8013231c_b(c, 0x8013231c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013231c_at(c: &mut Cpu, pc: u32) { f_8013231c_b(c, pc) }
#[inline(always)]
fn f_8013231c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013231c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013231c, pc); }
        match pc {
            0x8013231c => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa640));
                if true { return; }
                return;
            }
            _ => panic!("f_8013231c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlMsgID (0x80132324, 8 bytes)
#[inline(never)]
pub fn f_80132324(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132324) { return; }
        if c.prof { crate::prof::enter(c, 0x80132324); f_80132324_b(c, 0x80132324); crate::prof::exit(c, 0x80132324); return; }
    }
    f_80132324_b(c, 0x80132324)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132324_at(c: &mut Cpu, pc: u32) { f_80132324_b(c, pc) }
#[inline(always)]
fn f_80132324_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132324;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132324, pc); }
        match pc {
            0x80132324 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffffa63c));
                if true { return; }
                return;
            }
            _ => panic!("f_80132324: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlDigit2 (0x8013232c, 52 bytes)
#[inline(never)]
pub fn f_8013232c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013232c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013232c); f_8013232c_b(c, 0x8013232c); crate::prof::exit(c, 0x8013232c); return; }
    }
    f_8013232c_b(c, 0x8013232c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013232c_at(c: &mut Cpu, pc: u32) { f_8013232c_b(c, pc) }
#[inline(always)]
fn f_8013232c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013232c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013232c, pc); }
        match pc {
            0x8013232c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = 0x80420000;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa638));
                c.r[3] = c.r[3].wrapping_add(0x7190);
                c.r[4] = 0x10;
                c.r[6] = 0x0;
                c.lr = 0x80132350; f_80132834(c);
                pc = 0x80132350;
            }
            0x80132350 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8013232c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlDigit (0x80132360, 52 bytes)
#[inline(never)]
pub fn f_80132360(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132360) { return; }
        if c.prof { crate::prof::enter(c, 0x80132360); f_80132360_b(c, 0x80132360); crate::prof::exit(c, 0x80132360); return; }
    }
    f_80132360_b(c, 0x80132360)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132360_at(c: &mut Cpu, pc: u32) { f_80132360_b(c, pc) }
#[inline(always)]
fn f_80132360_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132360;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132360, pc); }
        match pc {
            0x80132360 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = 0x80420000;
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffffa634));
                c.r[3] = c.r[3].wrapping_add(0x71b0);
                c.r[4] = 0x10;
                c.r[6] = 0x0;
                c.lr = 0x80132384; f_80132834(c);
                pc = 0x80132384;
            }
            0x80132384 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80132360: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlItem2 (0x80132394, 52 bytes)
#[inline(never)]
pub fn f_80132394(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132394) { return; }
        if c.prof { crate::prof::enter(c, 0x80132394); f_80132394_b(c, 0x80132394); crate::prof::exit(c, 0x80132394); return; }
    }
    f_80132394_b(c, 0x80132394)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132394_at(c: &mut Cpu, pc: u32) { f_80132394_b(c, pc) }
#[inline(always)]
fn f_80132394_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132394;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132394, pc); }
        match pc {
            0x80132394 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lhz(c.r[13].wrapping_add(0xffffa632));
                c.lr = 0x801323a8; f_801440a0(c);
                pc = 0x801323a8;
            }
            0x801323a8 => {
                c.lr = 0x801323ac; f_80144088(c);
                pc = 0x801323ac;
            }
            0x801323ac => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801323b8; continue; }
                pc = 0x801323b4;
            }
            0x801323b4 => {
                c.r[3] = 0x2b6e;
                pc = 0x801323b8;
            }
            0x801323b8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80132394: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlItem (0x801323c8, 52 bytes)
#[inline(never)]
pub fn f_801323c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801323c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801323c8); f_801323c8_b(c, 0x801323c8); crate::prof::exit(c, 0x801323c8); return; }
    }
    f_801323c8_b(c, 0x801323c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801323c8_at(c: &mut Cpu, pc: u32) { f_801323c8_b(c, pc) }
#[inline(always)]
fn f_801323c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801323c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801323c8, pc); }
        match pc {
            0x801323c8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lhz(c.r[13].wrapping_add(0xffffa630));
                c.lr = 0x801323dc; f_801440a0(c);
                pc = 0x801323dc;
            }
            0x801323dc => {
                c.lr = 0x801323e0; f_80144088(c);
                pc = 0x801323e0;
            }
            0x801323e0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801323ec; continue; }
                pc = 0x801323e8;
            }
            0x801323e8 => {
                c.r[3] = 0x2b6e;
                pc = 0x801323ec;
            }
            0x801323ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801323c8: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlHizuki (0x801323fc, 44 bytes)
#[inline(never)]
pub fn f_801323fc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801323fc) { return; }
        if c.prof { crate::prof::enter(c, 0x801323fc); f_801323fc_b(c, 0x801323fc); crate::prof::exit(c, 0x801323fc); return; }
    }
    f_801323fc_b(c, 0x801323fc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801323fc_at(c: &mut Cpu, pc: u32) { f_801323fc_b(c, pc) }
#[inline(always)]
fn f_801323fc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801323fc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801323fc, pc); }
        match pc {
            0x801323fc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = 0x0;
                c.r[4] = 0x2;
                c.lr = 0x80132414; f_80129280(c);
                pc = 0x80132414;
            }
            0x80132414 => {
                c.lr = 0x80132418; f_8012a8d4(c);
                pc = 0x80132418;
            }
            0x80132418 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801323fc: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlHero (0x80132428, 44 bytes)
#[inline(never)]
pub fn f_80132428(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132428) { return; }
        if c.prof { crate::prof::enter(c, 0x80132428); f_80132428_b(c, 0x80132428); crate::prof::exit(c, 0x80132428); return; }
    }
    f_80132428_b(c, 0x80132428)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132428_at(c: &mut Cpu, pc: u32) { f_80132428_b(c, pc) }
#[inline(always)]
fn f_80132428_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132428;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132428, pc); }
        match pc {
            0x80132428 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = 0x0;
                c.r[4] = 0x2;
                c.lr = 0x80132440; f_80129280(c);
                pc = 0x80132440;
            }
            0x80132440 => {
                c.lr = 0x80132444; f_8012ac54(c);
                pc = 0x80132444;
            }
            0x80132444 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80132428: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlWait (0x80132454, 120 bytes)
#[inline(never)]
pub fn f_80132454(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132454) { return; }
        if c.prof { crate::prof::enter(c, 0x80132454); f_80132454_b(c, 0x80132454); crate::prof::exit(c, 0x80132454); return; }
    }
    f_80132454_b(c, 0x80132454)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132454_at(c: &mut Cpu, pc: u32) { f_80132454_b(c, pc) }
#[inline(always)]
fn f_80132454_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132454;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132454, pc); }
        match pc {
            0x80132454 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801324b8; continue; }
                pc = 0x80132460;
            }
            0x80132460 => {
                c.r[0] = c.lha(c.r[3].wrapping_add(0x48));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80132480; continue; }
                pc = 0x8013246c;
            }
            0x8013246c => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x30));
                c.r[4] = c.lbz(c.r[4]);
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.r[0] = c.r[0] as u16 as i16 as i32 as u32;
                c.sth(c.r[3].wrapping_add(0x48), c.r[0]);
                pc = 0x80132480;
            }
            0x80132480 => {
                c.r[4] = c.lha(c.r[3].wrapping_add(0x48));
                c.r[4] = c.r[4].wrapping_add(0xffffffff);
                c.r[0] = c.r[4] as u16 as i16 as i32 as u32;
                c.sth(c.r[3].wrapping_add(0x48), c.r[4]);
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(1) { pc = 0x801324a4; continue; }
                pc = 0x80132498;
            }
            0x80132498 => {
                c.r[0] = 0x0;
                c.sth(c.r[3].wrapping_add(0x48), c.r[0]);
                pc = 0x801324b8; continue;
                pc = 0x801324a4;
            }
            0x801324a4 => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x30));
                c.r[0] = c.r[4].wrapping_add(0xfffffffd);
                c.stw(c.r[3].wrapping_add(0x30), c.r[0]);
                c.r[3] = 0x1;
                if true { return; }
                pc = 0x801324b8;
            }
            0x801324b8 => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x30));
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[3].wrapping_add(0x30), c.r[0]);
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80132454: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlPalette (0x801324cc, 164 bytes)
#[inline(never)]
pub fn f_801324cc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801324cc) { return; }
        if c.prof { crate::prof::enter(c, 0x801324cc); f_801324cc_b(c, 0x801324cc); crate::prof::exit(c, 0x801324cc); return; }
    }
    f_801324cc_b(c, 0x801324cc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801324cc_at(c: &mut Cpu, pc: u32) { f_801324cc_b(c, pc) }
#[inline(always)]
fn f_801324cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801324cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801324cc, pc); }
        match pc {
            0x801324cc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8013254c; continue; }
                pc = 0x801324ec;
            }
            0x801324ec => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x30));
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8668));
                c.r[4] = c.lbz(c.r[4]);
                c.r[0] = c.lwz(c.r[3]);
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x80132508; continue; }
                pc = 0x80132504;
            }
            0x80132504 => {
                c.r[4] = 0x0;
                pc = 0x80132508;
            }
            0x80132508 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff866c));
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[31] | c.r[31];
                c.r[6] = c.r[4].wrapping_add(c.r[0]);
                c.r[4] = c.lbz(c.r[4].wrapping_add(c.r[0]));
                c.r[0] = c.lbz(c.r[6].wrapping_add(0x1));
                c.r[5] = c.lbz(c.r[6].wrapping_add(0x2));
                c.r[4] = c.r[4].rotate_left(24) & 0xff000000;
                c.r[0] = c.r[0].rotate_left(16) & 0xffff0000;
                c.r[6] = c.lbz(c.r[6].wrapping_add(0x3));
                c.r[5] = c.r[5].rotate_left(8) & 0xffffff00;
                c.r[0] = c.r[4] | c.r[0];
                c.r[0] = c.r[5] | c.r[0];
                c.r[0] = c.r[6] | c.r[0];
                c.stw(c.r[31].wrapping_add(0x24), c.r[0]);
                c.lr = 0x8013254c; f_800fa160(c);
                pc = 0x8013254c;
            }
            0x8013254c => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x30));
                c.r[3] = 0x0;
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[31].wrapping_add(0x30), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801324cc: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlColor (0x80132570, 84 bytes)
#[inline(never)]
pub fn f_80132570(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132570) { return; }
        if c.prof { crate::prof::enter(c, 0x80132570); f_80132570_b(c, 0x80132570); crate::prof::exit(c, 0x80132570); return; }
    }
    f_80132570_b(c, 0x80132570)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132570_at(c: &mut Cpu, pc: u32) { f_80132570_b(c, pc) }
#[inline(always)]
fn f_80132570_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132570;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132570, pc); }
        match pc {
            0x80132570 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801325a0; continue; }
                pc = 0x80132590;
            }
            0x80132590 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x30));
                c.r[0] = c.lwz(c.r[4]);
                c.stw(c.r[31].wrapping_add(0x24), c.r[0]);
                c.lr = 0x801325a0; f_800fa160(c);
                pc = 0x801325a0;
            }
            0x801325a0 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x30));
                c.r[3] = 0x0;
                c.r[0] = c.r[4].wrapping_add(0x4);
                c.stw(c.r[31].wrapping_add(0x30), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80132570: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlFont (0x801325c4, 104 bytes)
#[inline(never)]
pub fn f_801325c4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801325c4) { return; }
        if c.prof { crate::prof::enter(c, 0x801325c4); f_801325c4_b(c, 0x801325c4); crate::prof::exit(c, 0x801325c4); return; }
    }
    f_801325c4_b(c, 0x801325c4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801325c4_at(c: &mut Cpu, pc: u32) { f_801325c4_b(c, pc) }
#[inline(always)]
fn f_801325c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801325c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801325c4, pc); }
        match pc {
            0x801325c4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801325f8; continue; }
                pc = 0x801325e4;
            }
            0x801325e4 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x30));
                c.r[0] = c.lbz(c.r[4]);
                c.sth(c.r[31].wrapping_add(0x20), c.r[0]);
                c.lr = 0x801325f4; f_800fa1bc(c);
                pc = 0x801325f4;
            }
            0x801325f4 => {
                pc = 0x80132608; continue;
                pc = 0x801325f8;
            }
            0x801325f8 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x30));
                c.r[0] = c.lbz(c.r[4]);
                c.sth(c.r[31].wrapping_add(0x20), c.r[0]);
                c.lr = 0x80132608; f_800fa1bc(c);
                pc = 0x80132608;
            }
            0x80132608 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x30));
                c.r[3] = 0x0;
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.stw(c.r[31].wrapping_add(0x30), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801325c4: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlRubyEnd (0x8013262c, 16 bytes)
#[inline(never)]
pub fn f_8013262c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013262c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013262c); f_8013262c_b(c, 0x8013262c); crate::prof::exit(c, 0x8013262c); return; }
    }
    f_8013262c_b(c, 0x8013262c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013262c_at(c: &mut Cpu, pc: u32) { f_8013262c_b(c, pc) }
#[inline(always)]
fn f_8013262c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013262c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013262c, pc); }
        match pc {
            0x8013262c => {
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x4b), c.r[0]);
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8013262c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlRubyTop (0x8013263c, 16 bytes)
#[inline(never)]
pub fn f_8013263c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013263c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013263c); f_8013263c_b(c, 0x8013263c); crate::prof::exit(c, 0x8013263c); return; }
    }
    f_8013263c_b(c, 0x8013263c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013263c_at(c: &mut Cpu, pc: u32) { f_8013263c_b(c, pc) }
#[inline(always)]
fn f_8013263c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013263c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013263c, pc); }
        match pc {
            0x8013263c => {
                c.r[0] = 0x2;
                c.stb(c.r[3].wrapping_add(0x4b), c.r[0]);
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_8013263c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlRubyStart (0x8013264c, 68 bytes)
#[inline(never)]
pub fn f_8013264c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013264c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013264c); f_8013264c_b(c, 0x8013264c); crate::prof::exit(c, 0x8013264c); return; }
    }
    f_8013264c_b(c, 0x8013264c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013264c_at(c: &mut Cpu, pc: u32) { f_8013264c_b(c, pc) }
#[inline(always)]
fn f_8013264c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013264c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013264c, pc); }
        match pc {
            0x8013264c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80132670; continue; }
                pc = 0x8013266c;
            }
            0x8013266c => {
                c.lr = 0x80132670; f_800faa98(c);
                pc = 0x80132670;
            }
            0x80132670 => {
                c.r[0] = 0x1;
                c.r[3] = 0x0;
                c.stb(c.r[31].wrapping_add(0x4b), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8013264c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlKeyWait (0x80132690, 204 bytes)
#[inline(never)]
pub fn f_80132690(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132690) { return; }
        if c.prof { crate::prof::enter(c, 0x80132690); f_80132690_b(c, 0x80132690); crate::prof::exit(c, 0x80132690); return; }
    }
    f_80132690_b(c, 0x80132690)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132690_at(c: &mut Cpu, pc: u32) { f_80132690_b(c, pc) }
#[inline(always)]
fn f_80132690_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132690;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132690, pc); }
        match pc {
            0x80132690 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x44));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801326bc; continue; }
                pc = 0x801326b4;
            }
            0x801326b4 => {
                c.r[0] = 0x1;
                c.stb(c.r[31].wrapping_add(0x45), c.r[0]);
                pc = 0x801326bc;
            }
            0x801326bc => {
                c.r[3] = 0xa;
                c.lr = 0x801326c4; f_80102620(c);
                pc = 0x801326c4;
            }
            0x801326c4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801326d8; continue; }
                pc = 0x801326d0;
            }
            0x801326d0 => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x45), c.r[0]);
                pc = 0x801326d8;
            }
            0x801326d8 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80132734; continue; }
                pc = 0x801326e4;
            }
            0x801326e4 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x45));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8013271c; continue; }
                pc = 0x801326f0;
            }
            0x801326f0 => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x45), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x30));
                c.stw(c.r[31].wrapping_add(0x2c), c.r[0]);
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x44));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80132744; continue; }
                pc = 0x80132710;
            }
            0x80132710 => {
                c.r[3] = 0x24;
                c.lr = 0x80132718; f_80166a28(c);
                pc = 0x80132718;
            }
            0x80132718 => {
                pc = 0x80132744; continue;
                pc = 0x8013271c;
            }
            0x8013271c => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x30));
                c.r[0] = 0x1;
                c.r[3] = c.r[3].wrapping_add(0xfffffffd);
                c.stw(c.r[31].wrapping_add(0x30), c.r[3]);
                c.stb(c.r[31].wrapping_add(0x46), c.r[0]);
                pc = 0x80132744; continue;
                pc = 0x80132734;
            }
            0x80132734 => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0xc); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0x10); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80132744;
            }
            0x80132744 => {
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80132690: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlKeyEnd (0x8013275c, 132 bytes)
#[inline(never)]
pub fn f_8013275c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013275c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013275c); f_8013275c_b(c, 0x8013275c); crate::prof::exit(c, 0x8013275c); return; }
    }
    f_8013275c_b(c, 0x8013275c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013275c_at(c: &mut Cpu, pc: u32) { f_8013275c_b(c, pc) }
#[inline(always)]
fn f_8013275c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013275c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013275c, pc); }
        match pc {
            0x8013275c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x44));
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.cmps(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80132788; continue; }
                pc = 0x80132780;
            }
            0x80132780 => {
                c.r[0] = 0x1;
                c.stb(c.r[31].wrapping_add(0x45), c.r[0]);
                pc = 0x80132788;
            }
            0x80132788 => {
                c.r[3] = 0xa;
                c.lr = 0x80132790; f_80102620(c);
                pc = 0x80132790;
            }
            0x80132790 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801327a4; continue; }
                pc = 0x8013279c;
            }
            0x8013279c => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x45), c.r[0]);
                pc = 0x801327a4;
            }
            0x801327a4 => {
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x45));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801327bc; continue; }
                pc = 0x801327b0;
            }
            0x801327b0 => {
                c.r[0] = 0x0;
                c.stb(c.r[31].wrapping_add(0x45), c.r[0]);
                pc = 0x801327c8; continue;
                pc = 0x801327bc;
            }
            0x801327bc => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x30));
                c.r[0] = c.r[3].wrapping_add(0xfffffffd);
                c.stw(c.r[31].wrapping_add(0x30), c.r[0]);
                pc = 0x801327c8;
            }
            0x801327c8 => {
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8013275c: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlCR (0x801327e0, 84 bytes)
#[inline(never)]
pub fn f_801327e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801327e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801327e0); f_801327e0_b(c, 0x801327e0); crate::prof::exit(c, 0x801327e0); return; }
    }
    f_801327e0_b(c, 0x801327e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801327e0_at(c: &mut Cpu, pc: u32) { f_801327e0_b(c, pc) }
#[inline(always)]
fn f_801327e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801327e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801327e0, pc); }
        match pc {
            0x801327e0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                { let ea = c.r[3].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9a40); c.f[2] = c.lfd(ea); }
                { let ea = c.r[3].wrapping_add(0xc); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x42));
                c.r[4] = c.lbz(c.r[3].wrapping_add(0x23));
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                { let ea = c.r[3].wrapping_add(0x64); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.r[0] = c.r[4].wrapping_add(c.r[0]);
                { let ea = c.r[3].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[0] = c.r[0] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[3].mul_add(c.f[1], c.f[0])); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[3].wrapping_add(0x10); let v = c.f[0]; c.stfs(ea, v); }
                c.r[3] = 0x0;
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801327e0: bad pc {:#x}", pc),
        }
    }
}

/// _msgctrlMakeDigit__FPUslUll (0x80132834, 516 bytes)
#[inline(never)]
pub fn f_80132834(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132834) { return; }
        if c.prof { crate::prof::enter(c, 0x80132834); f_80132834_b(c, 0x80132834); crate::prof::exit(c, 0x80132834); return; }
    }
    f_80132834_b(c, 0x80132834)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132834_at(c: &mut Cpu, pc: u32) { f_80132834_b(c, pc) }
#[inline(always)]
fn f_80132834_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132834;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132834, pc); }
        match pc {
            0x80132834 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                c.r[9] = c.r[4].wrapping_add(0xffffffff);
                c.cmps(0, c.r[6], 0x3);
                c.r[0] = c.r[9].rotate_left(1) & 0xfffffffe;
                c.r[4] = 0x0;
                c.sth(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                c.r[0] = 0x0;
                c.r[4] = 0xa;
                c.r[5] = 0x0;
                c.r[7] = 0x1;
                if c.crbit(2) { pc = 0x801328b4; continue; }
                pc = 0x80132868;
            }
            0x80132868 => {
                if !c.crbit(0) { pc = 0x80132884; continue; }
                pc = 0x8013286c;
            }
            0x8013286c => {
                c.cmps(0, c.r[6], 0x1);
                if c.crbit(2) { pc = 0x801328bc; continue; }
                pc = 0x80132874;
            }
            0x80132874 => {
                if !c.crbit(0) { pc = 0x801328ac; continue; }
                pc = 0x80132878;
            }
            0x80132878 => {
                c.cmps(0, c.r[6], 0x0);
                if !c.crbit(0) { pc = 0x80132890; continue; }
                pc = 0x80132880;
            }
            0x80132880 => {
                pc = 0x801328bc; continue;
                pc = 0x80132884;
            }
            0x80132884 => {
                c.cmps(0, c.r[6], 0x5);
                if c.crbit(2) { pc = 0x80132890; continue; }
                pc = 0x8013288c;
            }
            0x8013288c => {
                pc = 0x801328bc; continue;
                pc = 0x80132890;
            }
            0x80132890 => {
                c.r[8] = c.lwz(c.r[1].wrapping_add(0x8));
                c.cmps(0, c.r[8], 0x0);
                if !c.crbit(0) { pc = 0x801328bc; continue; }
                pc = 0x8013289c;
            }
            0x8013289c => {
                c.r[8] = c.r[8].wrapping_neg();
                c.r[0] = 0x1;
                c.stw(c.r[1].wrapping_add(0x8), c.r[8]);
                pc = 0x801328bc; continue;
                pc = 0x801328ac;
            }
            0x801328ac => {
                c.r[7] = 0xa;
                pc = 0x801328bc; continue;
                pc = 0x801328b4;
            }
            0x801328b4 => {
                c.r[4] = 0x10;
                c.r[7] = 0x8;
                pc = 0x801328bc;
            }
            0x801328bc => {
                c.cmps(0, c.r[6], 0x5);
                if c.crbit(2) { pc = 0x801328c8; continue; }
                pc = 0x801328c4;
            }
            0x801328c4 => {
                pc = 0x801328d4; continue;
                pc = 0x801328c8;
            }
            0x801328c8 => {
                c.r[8] = 0x80360000;
                c.r[8] = c.r[8].wrapping_add(0x3610);
                pc = 0x801328dc; continue;
                pc = 0x801328d4;
            }
            0x801328d4 => {
                c.r[8] = 0x80360000;
                c.r[8] = c.r[8].wrapping_add(0x35f0);
                pc = 0x801328dc;
            }
            0x801328dc => {
                c.r[10] = c.r[9].rotate_left(1) & 0xfffffffe;
                pc = 0x80132954; continue;
                pc = 0x801328e4;
            }
            0x801328e4 => {
                c.cmps(0, c.r[6], 0x4);
                if !c.crbit(2) { pc = 0x80132928; continue; }
                pc = 0x801328ec;
            }
            0x801328ec => {
                c.cmps(0, c.r[5], 0x0);
                if c.crbit(2) { pc = 0x80132928; continue; }
                pc = 0x801328f4;
            }
            0x801328f4 => {
                c.r[11] = 0x55550000;
                c.r[11] = c.r[11].wrapping_add(0x5556);
                c.r[12] = mulhw(c.r[11], c.r[5]);
                c.r[11] = c.r[12].rotate_left(1) & 0x1;
                c.r[11] = c.r[12].wrapping_add(c.r[11]);
                c.r[11] = c.r[11].wrapping_mul(0x3);
                c.r[11] = c.r[5].wrapping_sub(c.r[11]);
                c.cmps(0, c.r[11], 0x0);
                if !c.crbit(2) { pc = 0x80132928; continue; }
                pc = 0x80132918;
            }
            0x80132918 => {
                c.r[10] = c.r[10].wrapping_add(0xfffffffe);
                c.r[11] = 0x2c;
                c.sth(c.r[3].wrapping_add(c.r[10]), c.r[11]);
                c.r[9] = c.r[9].wrapping_add(0xffffffff);
                pc = 0x80132928;
            }
            0x80132928 => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x8));
                c.r[10] = c.r[10].wrapping_add(0xfffffffe);
                c.r[9] = c.r[9].wrapping_add(0xffffffff);
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.r[12] = divwu(c.r[31], c.r[4]);
                c.r[11] = c.r[12].wrapping_mul(c.r[4]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[12]);
                c.r[11] = c.r[31].wrapping_sub(c.r[11]);
                c.r[11] = c.r[11].rotate_left(1) & 0xfffffffe;
                c.r[11] = c.lhz(c.r[8].wrapping_add(c.r[11]));
                c.sth(c.r[3].wrapping_add(c.r[10]), c.r[11]);
                pc = 0x80132954;
            }
            0x80132954 => {
                c.r[11] = c.lwz(c.r[1].wrapping_add(0x8));
                c.cmpu(0, c.r[11], 0x0);
                if !c.crbit(2) { pc = 0x801328e4; continue; }
                pc = 0x80132960;
            }
            0x80132960 => {
                c.cmps(0, c.r[5], c.r[7]);
                c.r[6] = c.r[9].rotate_left(1) & 0xfffffffe;
                c.r[5] = c.r[7].wrapping_sub(c.r[5]);
                if !c.crbit(0) { pc = 0x80132a08; continue; }
                pc = 0x80132970;
            }
            0x80132970 => {
                c.r[4] = c.r[5].rotate_left(29) & 0x1fffffff;
                c.cmpu(0, c.r[4], 0x0);
                c.mtspr(9, c.r[4]);
                if c.crbit(2) { pc = 0x801329f0; continue; }
                pc = 0x80132980;
            }
            0x80132980 => {
                c.r[4] = c.lhz(c.r[8]);
                c.r[6] = c.r[6].wrapping_add(0xfffffffe);
                c.r[9] = c.r[9].wrapping_add(0xfffffff8);
                c.sth(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                c.r[6] = c.r[6].wrapping_add(0xfffffffe);
                c.r[4] = c.lhz(c.r[8]);
                c.sth(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                c.r[6] = c.r[6].wrapping_add(0xfffffffe);
                c.r[4] = c.lhz(c.r[8]);
                c.sth(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                c.r[6] = c.r[6].wrapping_add(0xfffffffe);
                c.r[4] = c.lhz(c.r[8]);
                c.sth(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                c.r[6] = c.r[6].wrapping_add(0xfffffffe);
                c.r[4] = c.lhz(c.r[8]);
                c.sth(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                c.r[6] = c.r[6].wrapping_add(0xfffffffe);
                c.r[4] = c.lhz(c.r[8]);
                c.sth(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                c.r[6] = c.r[6].wrapping_add(0xfffffffe);
                c.r[4] = c.lhz(c.r[8]);
                c.sth(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                c.r[6] = c.r[6].wrapping_add(0xfffffffe);
                c.r[4] = c.lhz(c.r[8]);
                c.sth(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80132980; continue; }
                pc = 0x801329e8;
            }
            0x801329e8 => {
                c.r[5] = c.r[5] & 0x7;
                c.rc(c.r[5]);
                if c.crbit(2) { pc = 0x80132a08; continue; }
                pc = 0x801329f0;
            }
            0x801329f0 => {
                c.mtspr(9, c.r[5]);
                pc = 0x801329f4;
            }
            0x801329f4 => {
                c.r[4] = c.lhz(c.r[8]);
                c.r[6] = c.r[6].wrapping_add(0xfffffffe);
                c.r[9] = c.r[9].wrapping_add(0xffffffff);
                c.sth(c.r[3].wrapping_add(c.r[6]), c.r[4]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x801329f4; continue; }
                pc = 0x80132a08;
            }
            0x80132a08 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80132a24; continue; }
                pc = 0x80132a14;
            }
            0x80132a14 => {
                c.r[9] = c.r[9].wrapping_add(0xffffffff);
                c.r[4] = 0x2d;
                c.r[0] = c.r[9].rotate_left(1) & 0xfffffffe;
                c.sth(c.r[3].wrapping_add(c.r[0]), c.r[4]);
                pc = 0x80132a24;
            }
            0x80132a24 => {
                c.r[0] = c.r[9].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80132834: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlSetValue (0x80132a38, 528 bytes)
#[inline(never)]
pub fn f_80132a38(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132a38) { return; }
        if c.prof { crate::prof::enter(c, 0x80132a38); f_80132a38_b(c, 0x80132a38); crate::prof::exit(c, 0x80132a38); return; }
    }
    f_80132a38_b(c, 0x80132a38)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132a38_at(c: &mut Cpu, pc: u32) { f_80132a38_b(c, pc) }
#[inline(always)]
fn f_80132a38_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132a38;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132a38, pc); }
        match pc {
            0x80132a38 => {
                c.r[0] = c.r[3].wrapping_add(0xfffffff3);
                pc = 0x80132a3c;
            }
            0x80132a3c => {
                c.cmpu(0, c.r[0], 0x50);
                pc = 0x80132a40;
            }
            0x80132a40 => {
                if c.crbit(1) { return; }
                pc = 0x80132a44;
            }
            0x80132a44 => {
                c.r[3] = 0x80360000;
                pc = 0x80132a48;
            }
            0x80132a48 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80132a4c;
            }
            0x80132a4c => {
                c.r[3] = c.r[3].wrapping_add(0x3630);
                pc = 0x80132a50;
            }
            0x80132a50 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x80132a54;
            }
            0x80132a54 => {
                c.mtspr(9, c.r[0]);
                pc = 0x80132a58;
            }
            0x80132a58 => {
                if true { let t = c.ctr; if (0x80132a38..0x80132c48).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80132a5c;
            }
            0x80132a5c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x80132a60;
            }
            0x80132a60 => {
                c.sth(c.r[13].wrapping_add(0xffffa630), c.r[0]);
                pc = 0x80132a64;
            }
            0x80132a64 => {
                if true { return; }
                pc = 0x80132a68;
            }
            0x80132a68 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x80132a6c;
            }
            0x80132a6c => {
                c.sth(c.r[13].wrapping_add(0xffffa632), c.r[0]);
                pc = 0x80132a70;
            }
            0x80132a70 => {
                if true { return; }
                pc = 0x80132a74;
            }
            0x80132a74 => {
                c.stw(c.r[13].wrapping_add(0xffffa634), c.r[4]);
                pc = 0x80132a78;
            }
            0x80132a78 => {
                if true { return; }
                pc = 0x80132a7c;
            }
            0x80132a7c => {
                c.stw(c.r[13].wrapping_add(0xffffa638), c.r[4]);
                pc = 0x80132a80;
            }
            0x80132a80 => {
                if true { return; }
                pc = 0x80132a84;
            }
            0x80132a84 => {
                c.stw(c.r[13].wrapping_add(0xffffa63c), c.r[4]);
                pc = 0x80132a88;
            }
            0x80132a88 => {
                if true { return; }
                pc = 0x80132a8c;
            }
            0x80132a8c => {
                c.stw(c.r[13].wrapping_add(0xffffa640), c.r[4]);
                pc = 0x80132a90;
            }
            0x80132a90 => {
                if true { return; }
                pc = 0x80132a94;
            }
            0x80132a94 => {
                c.stw(c.r[13].wrapping_add(0xffffa644), c.r[4]);
                pc = 0x80132a98;
            }
            0x80132a98 => {
                if true { return; }
                pc = 0x80132a9c;
            }
            0x80132a9c => {
                c.stw(c.r[13].wrapping_add(0xffffa648), c.r[4]);
                pc = 0x80132aa0;
            }
            0x80132aa0 => {
                if true { return; }
                pc = 0x80132aa4;
            }
            0x80132aa4 => {
                c.stw(c.r[13].wrapping_add(0xffffa64c), c.r[4]);
                pc = 0x80132aa8;
            }
            0x80132aa8 => {
                if true { return; }
                pc = 0x80132aac;
            }
            0x80132aac => {
                c.stw(c.r[13].wrapping_add(0xffffa650), c.r[4]);
                pc = 0x80132ab0;
            }
            0x80132ab0 => {
                if true { return; }
                pc = 0x80132ab4;
            }
            0x80132ab4 => {
                c.stw(c.r[13].wrapping_add(0xffffa654), c.r[4]);
                pc = 0x80132ab8;
            }
            0x80132ab8 => {
                if true { return; }
                pc = 0x80132abc;
            }
            0x80132abc => {
                c.stw(c.r[13].wrapping_add(0xffffa658), c.r[4]);
                pc = 0x80132ac0;
            }
            0x80132ac0 => {
                if true { return; }
                pc = 0x80132ac4;
            }
            0x80132ac4 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x80132ac8;
            }
            0x80132ac8 => {
                c.sth(c.r[13].wrapping_add(0xffffa65c), c.r[0]);
                pc = 0x80132acc;
            }
            0x80132acc => {
                if true { return; }
                pc = 0x80132ad0;
            }
            0x80132ad0 => {
                c.stw(c.r[13].wrapping_add(0xffffa668), c.r[4]);
                pc = 0x80132ad4;
            }
            0x80132ad4 => {
                if true { return; }
                pc = 0x80132ad8;
            }
            0x80132ad8 => {
                c.stw(c.r[13].wrapping_add(0xffffa66c), c.r[4]);
                pc = 0x80132adc;
            }
            0x80132adc => {
                if true { return; }
                pc = 0x80132ae0;
            }
            0x80132ae0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x80132ae4;
            }
            0x80132ae4 => {
                c.sth(c.r[13].wrapping_add(0xffffa670), c.r[0]);
                pc = 0x80132ae8;
            }
            0x80132ae8 => {
                if true { return; }
                pc = 0x80132aec;
            }
            0x80132aec => {
                c.stw(c.r[13].wrapping_add(0xffffa660), c.r[4]);
                pc = 0x80132af0;
            }
            0x80132af0 => {
                if true { return; }
                pc = 0x80132af4;
            }
            0x80132af4 => {
                c.stw(c.r[13].wrapping_add(0xffffa664), c.r[4]);
                pc = 0x80132af8;
            }
            0x80132af8 => {
                if true { return; }
                pc = 0x80132afc;
            }
            0x80132afc => {
                c.stw(c.r[13].wrapping_add(0xffffa674), c.r[4]);
                pc = 0x80132b00;
            }
            0x80132b00 => {
                if true { return; }
                pc = 0x80132b04;
            }
            0x80132b04 => {
                c.stw(c.r[13].wrapping_add(0xffffa678), c.r[4]);
                pc = 0x80132b08;
            }
            0x80132b08 => {
                if true { return; }
                pc = 0x80132b0c;
            }
            0x80132b0c => {
                c.stw(c.r[13].wrapping_add(0xffffa67c), c.r[4]);
                pc = 0x80132b10;
            }
            0x80132b10 => {
                if true { return; }
                pc = 0x80132b14;
            }
            0x80132b14 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x80132b18;
            }
            0x80132b18 => {
                c.sth(c.r[13].wrapping_add(0xffffa680), c.r[0]);
                pc = 0x80132b1c;
            }
            0x80132b1c => {
                if true { return; }
                pc = 0x80132b20;
            }
            0x80132b20 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x80132b24;
            }
            0x80132b24 => {
                c.sth(c.r[13].wrapping_add(0xffffa682), c.r[0]);
                pc = 0x80132b28;
            }
            0x80132b28 => {
                if true { return; }
                pc = 0x80132b2c;
            }
            0x80132b2c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                pc = 0x80132b30;
            }
            0x80132b30 => {
                c.sth(c.r[13].wrapping_add(0xffffa684), c.r[0]);
                pc = 0x80132b34;
            }
            0x80132b34 => {
                if true { return; }
                pc = 0x80132b38;
            }
            0x80132b38 => {
                c.stw(c.r[13].wrapping_add(0xffffa5a8), c.r[4]);
                pc = 0x80132b3c;
            }
            0x80132b3c => {
                if true { return; }
                pc = 0x80132b40;
            }
            0x80132b40 => {
                c.stw(c.r[13].wrapping_add(0xffffa5ac), c.r[4]);
                pc = 0x80132b44;
            }
            0x80132b44 => {
                if true { return; }
                pc = 0x80132b48;
            }
            0x80132b48 => {
                c.stw(c.r[13].wrapping_add(0xffffa5b0), c.r[4]);
                pc = 0x80132b4c;
            }
            0x80132b4c => {
                if true { return; }
                pc = 0x80132b50;
            }
            0x80132b50 => {
                c.stw(c.r[13].wrapping_add(0xffffa5b4), c.r[4]);
                pc = 0x80132b54;
            }
            0x80132b54 => {
                if true { return; }
                pc = 0x80132b58;
            }
            0x80132b58 => {
                c.stw(c.r[13].wrapping_add(0xffffa5b8), c.r[4]);
                pc = 0x80132b5c;
            }
            0x80132b5c => {
                if true { return; }
                pc = 0x80132b60;
            }
            0x80132b60 => {
                c.stw(c.r[13].wrapping_add(0xffffa5bc), c.r[4]);
                pc = 0x80132b64;
            }
            0x80132b64 => {
                if true { return; }
                pc = 0x80132b68;
            }
            0x80132b68 => {
                c.stw(c.r[13].wrapping_add(0xffffa5c0), c.r[4]);
                pc = 0x80132b6c;
            }
            0x80132b6c => {
                if true { return; }
                pc = 0x80132b70;
            }
            0x80132b70 => {
                c.stw(c.r[13].wrapping_add(0xffffa5c4), c.r[4]);
                pc = 0x80132b74;
            }
            0x80132b74 => {
                if true { return; }
                pc = 0x80132b78;
            }
            0x80132b78 => {
                c.stw(c.r[13].wrapping_add(0xffffa5c8), c.r[4]);
                pc = 0x80132b7c;
            }
            0x80132b7c => {
                if true { return; }
                pc = 0x80132b80;
            }
            0x80132b80 => {
                c.stw(c.r[13].wrapping_add(0xffffa5cc), c.r[4]);
                pc = 0x80132b84;
            }
            0x80132b84 => {
                if true { return; }
                pc = 0x80132b88;
            }
            0x80132b88 => {
                c.stw(c.r[13].wrapping_add(0xffffa5d0), c.r[4]);
                pc = 0x80132b8c;
            }
            0x80132b8c => {
                if true { return; }
                pc = 0x80132b90;
            }
            0x80132b90 => {
                c.stw(c.r[13].wrapping_add(0xffffa5d4), c.r[4]);
                pc = 0x80132b94;
            }
            0x80132b94 => {
                if true { return; }
                pc = 0x80132b98;
            }
            0x80132b98 => {
                c.stw(c.r[13].wrapping_add(0xffffa5d8), c.r[4]);
                pc = 0x80132b9c;
            }
            0x80132b9c => {
                if true { return; }
                pc = 0x80132ba0;
            }
            0x80132ba0 => {
                c.stw(c.r[13].wrapping_add(0xffffa5dc), c.r[4]);
                pc = 0x80132ba4;
            }
            0x80132ba4 => {
                if true { return; }
                pc = 0x80132ba8;
            }
            0x80132ba8 => {
                c.stw(c.r[13].wrapping_add(0xffffa5e0), c.r[4]);
                pc = 0x80132bac;
            }
            0x80132bac => {
                if true { return; }
                pc = 0x80132bb0;
            }
            0x80132bb0 => {
                c.stw(c.r[13].wrapping_add(0xffffa5e4), c.r[4]);
                pc = 0x80132bb4;
            }
            0x80132bb4 => {
                if true { return; }
                pc = 0x80132bb8;
            }
            0x80132bb8 => {
                c.stw(c.r[13].wrapping_add(0xffffa5e8), c.r[4]);
                pc = 0x80132bbc;
            }
            0x80132bbc => {
                if true { return; }
                pc = 0x80132bc0;
            }
            0x80132bc0 => {
                c.stw(c.r[13].wrapping_add(0xffffa5ec), c.r[4]);
                pc = 0x80132bc4;
            }
            0x80132bc4 => {
                if true { return; }
                pc = 0x80132bc8;
            }
            0x80132bc8 => {
                c.stw(c.r[13].wrapping_add(0xffffa5f0), c.r[4]);
                pc = 0x80132bcc;
            }
            0x80132bcc => {
                if true { return; }
                pc = 0x80132bd0;
            }
            0x80132bd0 => {
                c.stw(c.r[13].wrapping_add(0xffffa5f4), c.r[4]);
                pc = 0x80132bd4;
            }
            0x80132bd4 => {
                if true { return; }
                pc = 0x80132bd8;
            }
            0x80132bd8 => {
                c.stw(c.r[13].wrapping_add(0xffffa5f8), c.r[4]);
                pc = 0x80132bdc;
            }
            0x80132bdc => {
                if true { return; }
                pc = 0x80132be0;
            }
            0x80132be0 => {
                c.stw(c.r[13].wrapping_add(0xffffa5fc), c.r[4]);
                pc = 0x80132be4;
            }
            0x80132be4 => {
                if true { return; }
                pc = 0x80132be8;
            }
            0x80132be8 => {
                c.stw(c.r[13].wrapping_add(0xffffa600), c.r[4]);
                pc = 0x80132bec;
            }
            0x80132bec => {
                if true { return; }
                pc = 0x80132bf0;
            }
            0x80132bf0 => {
                c.stw(c.r[13].wrapping_add(0xffffa604), c.r[4]);
                pc = 0x80132bf4;
            }
            0x80132bf4 => {
                if true { return; }
                pc = 0x80132bf8;
            }
            0x80132bf8 => {
                c.stw(c.r[13].wrapping_add(0xffffa608), c.r[4]);
                pc = 0x80132bfc;
            }
            0x80132bfc => {
                if true { return; }
                pc = 0x80132c00;
            }
            0x80132c00 => {
                c.stw(c.r[13].wrapping_add(0xffffa60c), c.r[4]);
                pc = 0x80132c04;
            }
            0x80132c04 => {
                if true { return; }
                pc = 0x80132c08;
            }
            0x80132c08 => {
                c.stw(c.r[13].wrapping_add(0xffffa610), c.r[4]);
                pc = 0x80132c0c;
            }
            0x80132c0c => {
                if true { return; }
                pc = 0x80132c10;
            }
            0x80132c10 => {
                c.stw(c.r[13].wrapping_add(0xffffa614), c.r[4]);
                pc = 0x80132c14;
            }
            0x80132c14 => {
                if true { return; }
                pc = 0x80132c18;
            }
            0x80132c18 => {
                c.stw(c.r[13].wrapping_add(0xffffa618), c.r[4]);
                pc = 0x80132c1c;
            }
            0x80132c1c => {
                if true { return; }
                pc = 0x80132c20;
            }
            0x80132c20 => {
                c.stw(c.r[13].wrapping_add(0xffffa61c), c.r[4]);
                pc = 0x80132c24;
            }
            0x80132c24 => {
                if true { return; }
                pc = 0x80132c28;
            }
            0x80132c28 => {
                c.stw(c.r[13].wrapping_add(0xffffa620), c.r[4]);
                pc = 0x80132c2c;
            }
            0x80132c2c => {
                if true { return; }
                pc = 0x80132c30;
            }
            0x80132c30 => {
                c.stw(c.r[13].wrapping_add(0xffffa624), c.r[4]);
                pc = 0x80132c34;
            }
            0x80132c34 => {
                if true { return; }
                pc = 0x80132c38;
            }
            0x80132c38 => {
                c.stw(c.r[13].wrapping_add(0xffffa628), c.r[4]);
                pc = 0x80132c3c;
            }
            0x80132c3c => {
                if true { return; }
                pc = 0x80132c40;
            }
            0x80132c40 => {
                c.stw(c.r[13].wrapping_add(0xffffa62c), c.r[4]);
                pc = 0x80132c44;
            }
            0x80132c44 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80132a38: bad pc {:#x}", pc),
        }
    }
}

/// msgctrlInitValue (0x80132c48, 36 bytes)
#[inline(never)]
pub fn f_80132c48(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132c48) { return; }
        if c.prof { crate::prof::enter(c, 0x80132c48); f_80132c48_b(c, 0x80132c48); crate::prof::exit(c, 0x80132c48); return; }
    }
    f_80132c48_b(c, 0x80132c48)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132c48_at(c: &mut Cpu, pc: u32) { f_80132c48_b(c, pc) }
#[inline(always)]
fn f_80132c48_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132c48;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132c48, pc); }
        match pc {
            0x80132c48 => {
                c.r[0] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa650), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa654), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa658), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa640), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa644), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa668), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa66c), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80132c48: bad pc {:#x}", pc),
        }
    }
}

/// fn_80132C6C (0x80132c6c, 784 bytes)
#[inline(never)]
pub fn f_80132c6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132c6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80132c6c); f_80132c6c_b(c, 0x80132c6c); crate::prof::exit(c, 0x80132c6c); return; }
    }
    f_80132c6c_b(c, 0x80132c6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132c6c_at(c: &mut Cpu, pc: u32) { f_80132c6c_b(c, pc) }
#[inline(always)]
fn f_80132c6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132c6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132c6c, pc); }
        match pc {
            0x80132c6c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[3] | c.r[3];
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                if c.crbit(2) { pc = 0x80132f60; continue; }
                pc = 0x80132c8c;
            }
            0x80132c8c => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x80132c98; continue; }
                pc = 0x80132c94;
            }
            0x80132c94 => {
                pc = 0x80132f60; continue;
                pc = 0x80132c98;
            }
            0x80132c98 => {
                c.r[3] = c.r[0].wrapping_mul(0x18);
                c.stw(c.r[13].wrapping_add(0xffffa694), c.r[0]);
                c.stw(c.r[13].wrapping_add(0xffffa6a0), c.r[4]);
                c.stw(c.r[13].wrapping_add(0xffffa6ac), c.r[6]);
                c.stw(c.r[13].wrapping_add(0xffffa6a8), c.r[5]);
                c.lr = 0x80132cb0; f_800e3534(c);
                pc = 0x80132cb0;
            }
            0x80132cb0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.sth(c.r[13].wrapping_add(0xffffa698), c.r[3]);
                if c.crbit(2) { pc = 0x80132f60; continue; }
                pc = 0x80132cbc;
            }
            0x80132cbc => {
                c.r[3] = c.r[0] | c.r[0];
                c.lr = 0x80132cc4; f_800e27b0(c);
                pc = 0x80132cc4;
            }
            0x80132cc4 => {
                c.r[10] = 0x0;
                c.stw(c.r[13].wrapping_add(0xffffa690), c.r[3]);
                c.r[8] = c.r[10] | c.r[10];
                c.r[11] = 0x0;
                c.r[7] = c.r[10] | c.r[10];
                c.r[6] = c.r[10] | c.r[10];
                c.r[5] = c.r[10] | c.r[10];
                c.r[4] = c.r[10] | c.r[10];
                c.r[3] = c.r[10] | c.r[10];
                c.r[0] = c.r[10] | c.r[10];
                pc = 0x80132d1c; continue;
                pc = 0x80132cf0;
            }
            0x80132cf0 => {
                c.r[9] = c.lwz(c.r[13].wrapping_add(0xffffa690));
                c.r[11] = c.r[11].wrapping_add(0x1);
                c.r[9] = c.r[9].wrapping_add(c.r[10]);
                c.r[10] = c.r[10].wrapping_add(0x18);
                c.stw(c.r[9], c.r[8]);
                c.stw(c.r[9].wrapping_add(0x4), c.r[7]);
                c.stw(c.r[9].wrapping_add(0x8), c.r[6]);
                c.stw(c.r[9].wrapping_add(0xc), c.r[5]);
                c.stw(c.r[9].wrapping_add(0x10), c.r[4]);
                c.stb(c.r[9].wrapping_add(0x14), c.r[3]);
                c.stb(c.r[9].wrapping_add(0x15), c.r[0]);
                pc = 0x80132d1c;
            }
            0x80132d1c => {
                c.r[9] = c.lwz(c.r[13].wrapping_add(0xffffa694));
                c.cmpu(0, c.r[11], c.r[9]);
                if c.crbit(0) { pc = 0x80132cf0; continue; }
                pc = 0x80132d28;
            }
            0x80132d28 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa6a0));
                c.r[31] = c.r[9].wrapping_mul(c.r[0]);
                c.r[3] = c.r[31].rotate_left(5) & 0xffffffe0;
                c.lr = 0x80132d38; f_800e3534(c);
                pc = 0x80132d38;
            }
            0x80132d38 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.sth(c.r[13].wrapping_add(0xffffa6a4), c.r[3]);
                if c.crbit(2) { pc = 0x80132f60; continue; }
                pc = 0x80132d44;
            }
            0x80132d44 => {
                c.r[3] = c.r[0] | c.r[0];
                c.lr = 0x80132d4c; f_800e27b0(c);
                pc = 0x80132d4c;
            }
            0x80132d4c => {
                c.cmpu(0, c.r[31], 0x0);
                c.stw(c.r[13].wrapping_add(0xffffa69c), c.r[3]);
                c.r[7] = 0x0;
                if !c.crbit(1) { pc = 0x80132f60; continue; }
                pc = 0x80132d5c;
            }
            0x80132d5c => {
                c.cmpu(0, c.r[31], 0x8);
                c.r[3] = c.r[31].wrapping_add(0xfffffff8);
                if !c.crbit(1) { pc = 0x80132f10; continue; }
                pc = 0x80132d68;
            }
            0x80132d68 => {
                c.r[0] = c.r[3].wrapping_add(0x7);
                c.r[6] = c.r[7] | c.r[7];
                c.r[0] = c.r[0].rotate_left(29) & 0x1fffffff;
                c.mtspr(9, c.r[0]);
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(1) { pc = 0x80132f10; continue; }
                pc = 0x80132d80;
            }
            0x80132d80 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa69c));
                c.r[4] = 0xffffffff;
                c.r[3] = 0x0;
                c.r[29] = c.r[6].wrapping_add(0x20);
                c.r[5] = c.r[0].wrapping_add(c.r[6]);
                c.r[30] = c.r[6].wrapping_add(0x40);
                c.stw(c.r[5], c.r[4]);
                c.r[12] = c.r[6].wrapping_add(0x60);
                c.r[11] = c.r[6].wrapping_add(0x80);
                c.r[10] = c.r[6].wrapping_add(0xa0);
                c.sth(c.r[5].wrapping_add(0x4), c.r[3]);
                c.r[9] = c.r[6].wrapping_add(0xc0);
                c.r[8] = c.r[6].wrapping_add(0xe0);
                c.r[6] = c.r[6].wrapping_add(0x100);
                c.sth(c.r[5].wrapping_add(0x6), c.r[3]);
                c.r[7] = c.r[7].wrapping_add(0x8);
                c.stw(c.r[5].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[5].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x10), c.r[4]);
                c.stb(c.r[5].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[5].wrapping_add(0x1c), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa69c));
                c.r[29] = c.r[0].wrapping_add(c.r[29]);
                c.stw(c.r[29], c.r[4]);
                c.sth(c.r[29].wrapping_add(0x4), c.r[3]);
                c.sth(c.r[29].wrapping_add(0x6), c.r[3]);
                c.stw(c.r[29].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[29].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[29].wrapping_add(0x10), c.r[4]);
                c.stb(c.r[29].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[29].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[29].wrapping_add(0x1c), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa69c));
                c.r[30] = c.r[0].wrapping_add(c.r[30]);
                c.stw(c.r[30], c.r[4]);
                c.sth(c.r[30].wrapping_add(0x4), c.r[3]);
                c.sth(c.r[30].wrapping_add(0x6), c.r[3]);
                c.stw(c.r[30].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[30].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[30].wrapping_add(0x10), c.r[4]);
                c.stb(c.r[30].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[30].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[30].wrapping_add(0x1c), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa69c));
                c.r[12] = c.r[0].wrapping_add(c.r[12]);
                c.stw(c.r[12], c.r[4]);
                c.sth(c.r[12].wrapping_add(0x4), c.r[3]);
                c.sth(c.r[12].wrapping_add(0x6), c.r[3]);
                c.stw(c.r[12].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[12].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[12].wrapping_add(0x10), c.r[4]);
                c.stb(c.r[12].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[12].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[12].wrapping_add(0x1c), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa69c));
                c.r[11] = c.r[0].wrapping_add(c.r[11]);
                c.stw(c.r[11], c.r[4]);
                c.sth(c.r[11].wrapping_add(0x4), c.r[3]);
                c.sth(c.r[11].wrapping_add(0x6), c.r[3]);
                c.stw(c.r[11].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[11].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[11].wrapping_add(0x10), c.r[4]);
                c.stb(c.r[11].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[11].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[11].wrapping_add(0x1c), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa69c));
                c.r[10] = c.r[0].wrapping_add(c.r[10]);
                c.stw(c.r[10], c.r[4]);
                c.sth(c.r[10].wrapping_add(0x4), c.r[3]);
                c.sth(c.r[10].wrapping_add(0x6), c.r[3]);
                c.stw(c.r[10].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[10].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[10].wrapping_add(0x10), c.r[4]);
                c.stb(c.r[10].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[10].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[10].wrapping_add(0x1c), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa69c));
                c.r[9] = c.r[0].wrapping_add(c.r[9]);
                c.stw(c.r[9], c.r[4]);
                c.sth(c.r[9].wrapping_add(0x4), c.r[3]);
                c.sth(c.r[9].wrapping_add(0x6), c.r[3]);
                c.stw(c.r[9].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[9].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[9].wrapping_add(0x10), c.r[4]);
                c.stb(c.r[9].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[9].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[9].wrapping_add(0x1c), c.r[3]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa69c));
                c.r[8] = c.r[0].wrapping_add(c.r[8]);
                c.stw(c.r[8], c.r[4]);
                c.sth(c.r[8].wrapping_add(0x4), c.r[3]);
                c.sth(c.r[8].wrapping_add(0x6), c.r[3]);
                c.stw(c.r[8].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[8].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[8].wrapping_add(0x10), c.r[4]);
                c.stb(c.r[8].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[8].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[8].wrapping_add(0x1c), c.r[3]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80132d80; continue; }
                pc = 0x80132f10;
            }
            0x80132f10 => {
                c.r[0] = c.r[31].wrapping_sub(c.r[7]);
                c.r[5] = c.r[7].rotate_left(5) & 0xffffffe0;
                c.r[4] = 0xffffffff;
                c.r[3] = 0x0;
                c.mtspr(9, c.r[0]);
                c.cmpu(0, c.r[7], c.r[31]);
                if !c.crbit(0) { pc = 0x80132f60; continue; }
                pc = 0x80132f2c;
            }
            0x80132f2c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa69c));
                c.r[6] = c.r[0].wrapping_add(c.r[5]);
                c.r[5] = c.r[5].wrapping_add(0x20);
                c.stw(c.r[6], c.r[4]);
                c.sth(c.r[6].wrapping_add(0x4), c.r[3]);
                c.sth(c.r[6].wrapping_add(0x6), c.r[3]);
                c.stw(c.r[6].wrapping_add(0x8), c.r[3]);
                c.stw(c.r[6].wrapping_add(0xc), c.r[3]);
                c.stw(c.r[6].wrapping_add(0x10), c.r[4]);
                c.stb(c.r[6].wrapping_add(0x14), c.r[3]);
                c.stw(c.r[6].wrapping_add(0x18), c.r[3]);
                c.stw(c.r[6].wrapping_add(0x1c), c.r[3]);
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80132f2c; continue; }
                pc = 0x80132f60;
            }
            0x80132f60 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80132c6c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80132F7C (0x80132f7c, 92 bytes)
#[inline(never)]
pub fn f_80132f7c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132f7c) { return; }
        if c.prof { crate::prof::enter(c, 0x80132f7c); f_80132f7c_b(c, 0x80132f7c); crate::prof::exit(c, 0x80132f7c); return; }
    }
    f_80132f7c_b(c, 0x80132f7c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132f7c_at(c: &mut Cpu, pc: u32) { f_80132f7c_b(c, pc) }
#[inline(always)]
fn f_80132f7c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132f7c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132f7c, pc); }
        match pc {
            0x80132f7c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0xab;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80132f90; f_80102620(c);
                pc = 0x80132f90;
            }
            0x80132f90 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80132fa4; continue; }
                pc = 0x80132f98;
            }
            0x80132f98 => {
                c.r[3] = 0xab;
                c.lr = 0x80132fa0; f_80102510(c);
                pc = 0x80132fa0;
            }
            0x80132fa0 => {
                pc = 0x80132fc4; continue;
                pc = 0x80132fa4;
            }
            0x80132fa4 => {
                c.r[3] = 0xab;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80132fc4; f_801026a4(c);
                pc = 0x80132fc4;
            }
            0x80132fc4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80132f7c: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuMovieTest (0x80132fd8, 120 bytes)
#[inline(never)]
pub fn f_80132fd8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80132fd8) { return; }
        if c.prof { crate::prof::enter(c, 0x80132fd8); f_80132fd8_b(c, 0x80132fd8); crate::prof::exit(c, 0x80132fd8); return; }
    }
    f_80132fd8_b(c, 0x80132fd8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80132fd8_at(c: &mut Cpu, pc: u32) { f_80132fd8_b(c, pc) }
#[inline(always)]
fn f_80132fd8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80132fd8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80132fd8, pc); }
        match pc {
            0x80132fd8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x80132ff0; continue;
                pc = 0x80132fe8;
            }
            0x80132fe8 => {
                c.lr = 0x80132fec; f_801e1810(c);
                pc = 0x80132fec;
            }
            0x80132fec => {
                c.lr = 0x80132ff0; crate::stubs::f_800f0308(c);
                pc = 0x80132ff0;
            }
            0x80132ff0 => {
                c.lr = 0x80132ff4; f_801e1874(c);
                pc = 0x80132ff4;
            }
            0x80132ff4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80132fe8; continue; }
                pc = 0x80133000;
            }
            0x80133000 => {
                c.r[3] = 0x2;
                c.r[4] = 0x1;
                c.lr = 0x8013300c; f_8010264c(c);
                pc = 0x8013300c;
            }
            0x8013300c => {
                c.cmps(0, c.r[3], 0xffffffff);
                if c.crbit(2) { pc = 0x8013303c; continue; }
                pc = 0x80133014;
            }
            0x80133014 => {
                c.r[4] = 0x80270000;
                c.r[6] = c.r[3] | c.r[3];
                c.r[4] = c.r[4].wrapping_add(0x2aa8);
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[5] = c.r[2].wrapping_add(0xffff9a48);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80133030; f_800c8520(c);
                pc = 0x80133030;
            }
            0x80133030 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = 0x0;
                c.lr = 0x8013303c; f_801e189c(c);
                pc = 0x8013303c;
            }
            0x8013303c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80132fd8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80133050 (0x80133050, 60 bytes)
#[inline(never)]
pub fn f_80133050(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133050) { return; }
        if c.prof { crate::prof::enter(c, 0x80133050); f_80133050_b(c, 0x80133050); crate::prof::exit(c, 0x80133050); return; }
    }
    f_80133050_b(c, 0x80133050)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133050_at(c: &mut Cpu, pc: u32) { f_80133050_b(c, pc) }
#[inline(always)]
fn f_80133050_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133050;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133050, pc); }
        match pc {
            0x80133050 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x3;
                c.r[4] = 0x2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0x0;
                c.r[6] = 0x2;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x80133078; f_800d37d4(c);
                pc = 0x80133078;
            }
            0x80133078 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133050: bad pc {:#x}", pc),
        }
    }
}

/// fn_8013308C (0x8013308c, 60 bytes)
#[inline(never)]
pub fn f_8013308c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013308c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013308c); f_8013308c_b(c, 0x8013308c); crate::prof::exit(c, 0x8013308c); return; }
    }
    f_8013308c_b(c, 0x8013308c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013308c_at(c: &mut Cpu, pc: u32) { f_8013308c_b(c, pc) }
#[inline(always)]
fn f_8013308c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013308c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013308c, pc); }
        match pc {
            0x8013308c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x2;
                c.r[4] = 0x2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[5] = 0x0;
                c.r[6] = 0x2;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x801330b4; f_800d37d4(c);
                pc = 0x801330b4;
            }
            0x801330b4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8013308c: bad pc {:#x}", pc),
        }
    }
}

/// fn_801330C8 (0x801330c8, 336 bytes)
#[inline(never)]
pub fn f_801330c8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801330c8) { return; }
        if c.prof { crate::prof::enter(c, 0x801330c8); f_801330c8_b(c, 0x801330c8); crate::prof::exit(c, 0x801330c8); return; }
    }
    f_801330c8_b(c, 0x801330c8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801330c8_at(c: &mut Cpu, pc: u32) { f_801330c8_b(c, pc) }
#[inline(always)]
fn f_801330c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801330c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801330c8, pc); }
        match pc {
            0x801330c8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801330dc; f_800d88dc(c);
                pc = 0x801330dc;
            }
            0x801330dc => {
                c.r[3] = 0x6;
                c.lr = 0x801330e4; f_800d888c(c);
                pc = 0x801330e4;
            }
            0x801330e4 => {
                { let ea = c.r[2].wrapping_add(0xffff9a50); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9a54); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.f[2] = c.f[1];
                { let ea = c.r[2].wrapping_add(0xffff9a58); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.lr = 0x801330f8; f_800d9b58(c);
                pc = 0x801330f8;
            }
            0x801330f8 => {
                c.r[3] = 0x0;
                c.r[4] = 0x6;
                c.r[5] = 0x7;
                c.lr = 0x80133108; f_800da4c4(c);
                pc = 0x80133108;
            }
            0x80133108 => {
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x80133118; f_800da2bc(c);
                pc = 0x80133118;
            }
            0x80133118 => {
                c.r[3] = 0x0;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x80133128; f_800da1e8(c);
                pc = 0x80133128;
            }
            0x80133128 => {
                c.r[3] = 0x0;
                c.lr = 0x80133130; f_800da028(c);
                pc = 0x80133130;
            }
            0x80133130 => {
                c.r[3] = 0x4;
                c.lr = 0x80133138; f_800d6a00(c);
                pc = 0x80133138;
            }
            0x80133138 => {
                c.r[3] = 0x0;
                c.lr = 0x80133140; f_800d7820(c);
                pc = 0x80133140;
            }
            0x80133140 => {
                c.r[3] = 0x3;
                c.lr = 0x80133148; f_800d67bc(c);
                pc = 0x80133148;
            }
            0x80133148 => {
                c.r[3] = 0x80480000;
                { let ea = c.r[2].wrapping_add(0xffff9a5c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0xffff8ac0); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[2].wrapping_add(0xffff9a50); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.f[2] = -c.f[0];
                c.lr = 0x80133160; f_800d6680(c);
                pc = 0x80133160;
            }
            0x80133160 => {
                c.r[3] = 0x0;
                c.r[4] = 0xff;
                c.r[5] = 0x80;
                c.r[6] = 0x40;
                c.r[7] = 0x0;
                c.lr = 0x80133178; f_800d5cb8(c);
                pc = 0x80133178;
            }
            0x80133178 => {
                c.r[3] = 0x80480000;
                { let ea = c.r[2].wrapping_add(0xffff9a60); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[2].wrapping_add(0xffff9a50); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[3].wrapping_add(0xffff8ac4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x8013318c; f_800d6680(c);
                pc = 0x8013318c;
            }
            0x8013318c => {
                c.r[3] = 0x0;
                c.r[4] = 0x40;
                c.r[5] = 0xff;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x801331a4; f_800d5cb8(c);
                pc = 0x801331a4;
            }
            0x801331a4 => {
                c.r[3] = 0x80480000;
                { let ea = c.r[2].wrapping_add(0xffff9a64); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[2].wrapping_add(0xffff9a50); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let ea = c.r[3].wrapping_add(0xffff8ac4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.lr = 0x801331b8; f_800d6680(c);
                pc = 0x801331b8;
            }
            0x801331b8 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x40;
                c.r[6] = 0xff;
                c.r[7] = 0x0;
                c.lr = 0x801331d0; f_800d5cb8(c);
                pc = 0x801331d0;
            }
            0x801331d0 => {
                c.r[3] = 0x80480000;
                { let ea = c.r[2].wrapping_add(0xffff9a5c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[3].wrapping_add(0xffff8ac0); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[2].wrapping_add(0xffff9a50); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.f[2] = -c.f[0];
                c.lr = 0x801331e8; f_800d6680(c);
                pc = 0x801331e8;
            }
            0x801331e8 => {
                c.r[3] = 0x0;
                c.r[4] = 0xff;
                c.r[5] = 0x80;
                c.r[6] = 0x40;
                c.r[7] = 0x0;
                c.lr = 0x80133200; f_800d5cb8(c);
                pc = 0x80133200;
            }
            0x80133200 => {
                c.lr = 0x80133204; f_800d6728(c);
                pc = 0x80133204;
            }
            0x80133204 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801330c8: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuGSmemOptimize (0x80133218, 56 bytes)
#[inline(never)]
pub fn f_80133218(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133218) { return; }
        if c.prof { crate::prof::enter(c, 0x80133218); f_80133218_b(c, 0x80133218); crate::prof::exit(c, 0x80133218); return; }
    }
    f_80133218_b(c, 0x80133218)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133218_at(c: &mut Cpu, pc: u32) { f_80133218_b(c, pc) }
#[inline(always)]
fn f_80133218_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133218;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133218, pc); }
        match pc {
            0x80133218 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80133228; f_800e1544(c);
                pc = 0x80133228;
            }
            0x80133228 => {
                c.r[5] = 0x80270000;
                c.r[4] = c.r[3] | c.r[3];
                c.r[3] = c.r[5].wrapping_add(0x2ab8);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8013323c; stubs_gen::f_800dd970(c);
                pc = 0x8013323c;
            }
            0x8013323c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133218: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuGSmemDispMap (0x80133250, 44 bytes)
#[inline(never)]
pub fn f_80133250(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133250) { return; }
        if c.prof { crate::prof::enter(c, 0x80133250); f_80133250_b(c, 0x80133250); crate::prof::exit(c, 0x80133250); return; }
    }
    f_80133250_b(c, 0x80133250)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133250_at(c: &mut Cpu, pc: u32) { f_80133250_b(c, pc) }
#[inline(always)]
fn f_80133250_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133250;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133250, pc); }
        match pc {
            0x80133250 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80133268; f_800e0e14(c);
                pc = 0x80133268;
            }
            0x80133268 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133250: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuGSmemCheck (0x8013327c, 92 bytes)
#[inline(never)]
pub fn f_8013327c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013327c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013327c); f_8013327c_b(c, 0x8013327c); crate::prof::exit(c, 0x8013327c); return; }
    }
    f_8013327c_b(c, 0x8013327c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013327c_at(c: &mut Cpu, pc: u32) { f_8013327c_b(c, pc) }
#[inline(always)]
fn f_8013327c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013327c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013327c, pc); }
        match pc {
            0x8013327c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x1;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80133294; f_800e0e14(c);
                pc = 0x80133294;
            }
            0x80133294 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x801332b4; continue; }
                pc = 0x801332a0;
            }
            0x801332a0 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x2ae0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801332b0; stubs_gen::f_800dd970(c);
                pc = 0x801332b0;
            }
            0x801332b0 => {
                pc = 0x801332c4; continue;
                pc = 0x801332b4;
            }
            0x801332b4 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x2af0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801332c4; stubs_gen::f_800dd970(c);
                pc = 0x801332c4;
            }
            0x801332c4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8013327c: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuFrameRate20 (0x801332d8, 40 bytes)
#[inline(never)]
pub fn f_801332d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801332d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801332d8); f_801332d8_b(c, 0x801332d8); crate::prof::exit(c, 0x801332d8); return; }
    }
    f_801332d8_b(c, 0x801332d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801332d8_at(c: &mut Cpu, pc: u32) { f_801332d8_b(c, pc) }
#[inline(always)]
fn f_801332d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801332d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801332d8, pc); }
        match pc {
            0x801332d8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801332ec; f_800d3074(c);
                pc = 0x801332ec;
            }
            0x801332ec => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801332d8: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuFrameRate30 (0x80133300, 40 bytes)
#[inline(never)]
pub fn f_80133300(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133300) { return; }
        if c.prof { crate::prof::enter(c, 0x80133300); f_80133300_b(c, 0x80133300); crate::prof::exit(c, 0x80133300); return; }
    }
    f_80133300_b(c, 0x80133300)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133300_at(c: &mut Cpu, pc: u32) { f_80133300_b(c, pc) }
#[inline(always)]
fn f_80133300_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133300;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133300, pc); }
        match pc {
            0x80133300 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x1;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80133314; f_800d3074(c);
                pc = 0x80133314;
            }
            0x80133314 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133300: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuSendAllMail (0x80133328, 36 bytes)
#[inline(never)]
pub fn f_80133328(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133328) { return; }
        if c.prof { crate::prof::enter(c, 0x80133328); f_80133328_b(c, 0x80133328); crate::prof::exit(c, 0x80133328); return; }
    }
    f_80133328_b(c, 0x80133328)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133328_at(c: &mut Cpu, pc: u32) { f_80133328_b(c, pc) }
#[inline(always)]
fn f_80133328_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133328;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133328, pc); }
        match pc {
            0x80133328 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80133338; f_801d216c(c);
                pc = 0x80133338;
            }
            0x80133338 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133328: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuSendMail (0x8013334c, 96 bytes)
#[inline(never)]
pub fn f_8013334c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013334c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013334c); f_8013334c_b(c, 0x8013334c); crate::prof::exit(c, 0x8013334c); return; }
    }
    f_8013334c_b(c, 0x8013334c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013334c_at(c: &mut Cpu, pc: u32) { f_8013334c_b(c, pc) }
#[inline(always)]
fn f_8013334c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013334c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013334c, pc); }
        match pc {
            0x8013334c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                pc = 0x8013337c; continue;
                pc = 0x80133360;
            }
            0x80133360 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80133368; f_801d1cc4(c);
                pc = 0x80133368;
            }
            0x80133368 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8013337c; continue; }
                pc = 0x80133370;
            }
            0x80133370 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80133378; f_801d1d58(c);
                pc = 0x80133378;
            }
            0x80133378 => {
                c.lr = 0x8013337c; f_801d268c(c);
                pc = 0x8013337c;
            }
            0x8013337c => {
                c.r[3] = 0x2;
                c.r[4] = 0x1;
                c.lr = 0x80133388; f_8010264c(c);
                pc = 0x80133388;
            }
            0x80133388 => {
                c.r[31] = c.r[3] | c.r[3];
                c.cmps(0, c.r[31], 0xffffffff);
                if !c.crbit(2) { pc = 0x80133360; continue; }
                pc = 0x80133394;
            }
            0x80133394 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8013334c: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuColisionDisp (0x801333ac, 164 bytes)
#[inline(never)]
pub fn f_801333ac(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801333ac) { return; }
        if c.prof { crate::prof::enter(c, 0x801333ac); f_801333ac_b(c, 0x801333ac); crate::prof::exit(c, 0x801333ac); return; }
    }
    f_801333ac_b(c, 0x801333ac)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801333ac_at(c: &mut Cpu, pc: u32) { f_801333ac_b(c, pc) }
#[inline(always)]
fn f_801333ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801333ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801333ac, pc); }
        match pc {
            0x801333ac => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x2;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = 0x0;
                c.lr = 0x801333cc; f_800f9318(c);
                pc = 0x801333cc;
            }
            0x801333cc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801333dc; continue; }
                pc = 0x801333d4;
            }
            0x801333d4 => {
                c.r[3] = 0x0;
                pc = 0x8013343c; continue;
                pc = 0x801333dc;
            }
            0x801333dc => {
                c.cmps(0, c.r[31], 0xb9);
                if c.crbit(2) { pc = 0x80133418; continue; }
                pc = 0x801333e4;
            }
            0x801333e4 => {
                if !c.crbit(0) { pc = 0x80133438; continue; }
                pc = 0x801333e8;
            }
            0x801333e8 => {
                c.cmps(0, c.r[31], 0xb8);
                if !c.crbit(0) { pc = 0x801333f4; continue; }
                pc = 0x801333f0;
            }
            0x801333f0 => {
                pc = 0x80133438; continue;
                pc = 0x801333f4;
            }
            0x801333f4 => {
                c.r[0] = c.lbz(c.r[3]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x8013340c; continue; }
                pc = 0x80133400;
            }
            0x80133400 => {
                c.r[0] = 0x0;
                c.stb(c.r[3], c.r[0]);
                pc = 0x80133438; continue;
                pc = 0x8013340c;
            }
            0x8013340c => {
                c.r[0] = 0x1;
                c.stb(c.r[3], c.r[0]);
                pc = 0x80133438; continue;
                pc = 0x80133418;
            }
            0x80133418 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80133430; continue; }
                pc = 0x80133424;
            }
            0x80133424 => {
                c.r[0] = 0x0;
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                pc = 0x80133438; continue;
                pc = 0x80133430;
            }
            0x80133430 => {
                c.r[0] = 0x1;
                c.stb(c.r[3].wrapping_add(0x1), c.r[0]);
                pc = 0x80133438;
            }
            0x80133438 => {
                c.r[3] = 0x0;
                pc = 0x8013343c;
            }
            0x8013343c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801333ac: bad pc {:#x}", pc),
        }
    }
}

/// debugMenuColorBarDisp (0x80133450, 88 bytes)
#[inline(never)]
pub fn f_80133450(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133450) { return; }
        if c.prof { crate::prof::enter(c, 0x80133450); f_80133450_b(c, 0x80133450); crate::prof::exit(c, 0x80133450); return; }
    }
    f_80133450_b(c, 0x80133450)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133450_at(c: &mut Cpu, pc: u32) { f_80133450_b(c, pc) }
#[inline(always)]
fn f_80133450_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133450;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133450, pc); }
        match pc {
            0x80133450 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x5;
                c.r[4] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffff8000), c.r[0]);
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80133484; f_801026a4(c);
                pc = 0x80133484;
            }
            0x80133484 => {
                c.r[3] = 0x5;
                c.lr = 0x8013348c; f_80102510(c);
                pc = 0x8013348c;
            }
            0x8013348c => {
                c.r[0] = 0x1;
                c.r[3] = 0x0;
                c.stb(c.r[13].wrapping_add(0xffff8000), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133450: bad pc {:#x}", pc),
        }
    }
}

/// debugMenuShadowBorderDisp (0x801334a8, 52 bytes)
#[inline(never)]
pub fn f_801334a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801334a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801334a8); f_801334a8_b(c, 0x801334a8); crate::prof::exit(c, 0x801334a8); return; }
    }
    f_801334a8_b(c, 0x801334a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801334a8_at(c: &mut Cpu, pc: u32) { f_801334a8_b(c, pc) }
#[inline(always)]
fn f_801334a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801334a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801334a8, pc); }
        match pc {
            0x801334a8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa6b9));
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0xff;
                c.stb(c.r[13].wrapping_add(0xffffa6b9), c.r[3]);
                c.lr = 0x801334c8; f_800e8f80(c);
                pc = 0x801334c8;
            }
            0x801334c8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801334a8: bad pc {:#x}", pc),
        }
    }
}

/// fn_801334DC (0x801334dc, 52 bytes)
#[inline(never)]
pub fn f_801334dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801334dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801334dc); f_801334dc_b(c, 0x801334dc); crate::prof::exit(c, 0x801334dc); return; }
    }
    f_801334dc_b(c, 0x801334dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801334dc_at(c: &mut Cpu, pc: u32) { f_801334dc_b(c, pc) }
#[inline(always)]
fn f_801334dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801334dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801334dc, pc); }
        match pc {
            0x801334dc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa6b8));
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0xff;
                c.stb(c.r[13].wrapping_add(0xffffa6b8), c.r[3]);
                c.lr = 0x801334fc; f_800d4610(c);
                pc = 0x801334fc;
            }
            0x801334fc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801334dc: bad pc {:#x}", pc),
        }
    }
}

/// debugMenuNodrawArea (0x80133510, 92 bytes)
#[inline(never)]
pub fn f_80133510(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133510) { return; }
        if c.prof { crate::prof::enter(c, 0x80133510); f_80133510_b(c, 0x80133510); crate::prof::exit(c, 0x80133510); return; }
    }
    f_80133510_b(c, 0x80133510)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133510_at(c: &mut Cpu, pc: u32) { f_80133510_b(c, pc) }
#[inline(always)]
fn f_80133510_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133510;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133510, pc); }
        match pc {
            0x80133510 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x7;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80133524; f_80102620(c);
                pc = 0x80133524;
            }
            0x80133524 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80133538; continue; }
                pc = 0x8013352c;
            }
            0x8013352c => {
                c.r[3] = 0x7;
                c.lr = 0x80133534; f_80102510(c);
                pc = 0x80133534;
            }
            0x80133534 => {
                pc = 0x80133558; continue;
                pc = 0x80133538;
            }
            0x80133538 => {
                c.r[3] = 0x7;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80133558; f_801026a4(c);
                pc = 0x80133558;
            }
            0x80133558 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133510: bad pc {:#x}", pc),
        }
    }
}

/// fn_8013356C (0x8013356c, 104 bytes)
#[inline(never)]
pub fn f_8013356c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013356c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013356c); f_8013356c_b(c, 0x8013356c); crate::prof::exit(c, 0x8013356c); return; }
    }
    f_8013356c_b(c, 0x8013356c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013356c_at(c: &mut Cpu, pc: u32) { f_8013356c_b(c, pc) }
#[inline(always)]
fn f_8013356c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013356c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013356c, pc); }
        match pc {
            0x8013356c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmps(0, c.r[4], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = 0xffffffff;
                if c.crbit(2) { pc = 0x8013358c; continue; }
                pc = 0x80133588;
            }
            0x80133588 => {
                pc = 0x80133590; continue;
                pc = 0x8013358c;
            }
            0x8013358c => {
                c.r[31] = 0x1;
                pc = 0x80133590;
            }
            0x80133590 => {
                c.cmps(0, c.r[31], 0x0);
                if c.crbit(0) { pc = 0x801335bc; continue; }
                pc = 0x80133598;
            }
            0x80133598 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801335a0; f_8012f11c(c);
                pc = 0x801335a0;
            }
            0x801335a0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x801335b4; continue; }
                pc = 0x801335a8;
            }
            0x801335a8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801335b0; f_8012f150(c);
                pc = 0x801335b0;
            }
            0x801335b0 => {
                pc = 0x801335bc; continue;
                pc = 0x801335b4;
            }
            0x801335b4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801335bc; f_8012f1fc(c);
                pc = 0x801335bc;
            }
            0x801335bc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8013356c: bad pc {:#x}", pc),
        }
    }
}

/// debugMenuChangeMemInfo (0x801335d4, 92 bytes)
#[inline(never)]
pub fn f_801335d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801335d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801335d4); f_801335d4_b(c, 0x801335d4); crate::prof::exit(c, 0x801335d4); return; }
    }
    f_801335d4_b(c, 0x801335d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801335d4_at(c: &mut Cpu, pc: u32) { f_801335d4_b(c, pc) }
#[inline(always)]
fn f_801335d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801335d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801335d4, pc); }
        match pc {
            0x801335d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[3] = 0x4;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x801335e8; f_80102620(c);
                pc = 0x801335e8;
            }
            0x801335e8 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x801335fc; continue; }
                pc = 0x801335f0;
            }
            0x801335f0 => {
                c.r[3] = 0x4;
                c.lr = 0x801335f8; f_80102510(c);
                pc = 0x801335f8;
            }
            0x801335f8 => {
                pc = 0x8013361c; continue;
                pc = 0x801335fc;
            }
            0x801335fc => {
                c.r[3] = 0x4;
                c.r[4] = 0x0;
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.r[7] = 0x0;
                c.r[8] = 0x0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8013361c; f_801026a4(c);
                pc = 0x8013361c;
            }
            0x8013361c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801335d4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80133630 (0x80133630, 52 bytes)
#[inline(never)]
pub fn f_80133630(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133630) { return; }
        if c.prof { crate::prof::enter(c, 0x80133630); f_80133630_b(c, 0x80133630); crate::prof::exit(c, 0x80133630); return; }
    }
    f_80133630_b(c, 0x80133630)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133630_at(c: &mut Cpu, pc: u32) { f_80133630_b(c, pc) }
#[inline(always)]
fn f_80133630_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133630;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133630, pc); }
        match pc {
            0x80133630 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa6b4));
                c.r[0] = c.r[0].leading_zeros();
                c.r[3] = c.r[0].rotate_left(27) & 0x7ffffff;
                c.stw(c.r[13].wrapping_add(0xffffa6b4), c.r[3]);
                c.lr = 0x80133650; f_80101b88(c);
                pc = 0x80133650;
            }
            0x80133650 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133630: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuCursor (0x80133664, 316 bytes)
#[inline(never)]
pub fn f_80133664(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133664) { return; }
        if c.prof { crate::prof::enter(c, 0x80133664); f_80133664_b(c, 0x80133664); crate::prof::exit(c, 0x80133664); return; }
    }
    f_80133664_b(c, 0x80133664)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133664_at(c: &mut Cpu, pc: u32) { f_80133664_b(c, pc) }
#[inline(always)]
fn f_80133664_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133664;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133664, pc); }
        match pc {
            0x80133664 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.lr = 0x80133684; f_80105624(c);
                pc = 0x80133684;
            }
            0x80133684 => {
                c.r[29] = c.lhz(c.r[3].wrapping_add(0x6));
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.lr = 0x80133694; f_80133b50(c);
                pc = 0x80133694;
            }
            0x80133694 => {
                c.r[31] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x4));
                c.lr = 0x801336a0; f_8005d9e4(c);
                pc = 0x801336a0;
            }
            0x801336a0 => {
                c.r[5] = c.r[3] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[31], c.r[5]);
                if !c.crbit(0) { pc = 0x801336b0; continue; }
                pc = 0x801336ac;
            }
            0x801336ac => {
                c.r[5] = c.r[31] | c.r[31];
                pc = 0x801336b0;
            }
            0x801336b0 => {
                c.r[3] = c.lhz(c.r[30].wrapping_add(0x94));
                c.r[0] = c.r[29].rotate_left(0) & 0x1;
                c.rc(c.r[0]);
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.sth(c.r[1].wrapping_add(0x8), c.r[3]);
                if c.crbit(2) { pc = 0x801336d4; continue; }
                pc = 0x801336c4;
            }
            0x801336c4 => {
                c.r[3] = c.lbz(c.r[1].wrapping_add(0x9));
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.stb(c.r[1].wrapping_add(0x9), c.r[0]);
                pc = 0x801336e8; continue;
                pc = 0x801336d4;
            }
            0x801336d4 => {
                c.r[0] = c.r[0].rotate_left(0) & 0x2;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x801336e8; continue; }
                pc = 0x801336dc;
            }
            0x801336dc => {
                c.r[3] = c.lbz(c.r[1].wrapping_add(0x9));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stb(c.r[1].wrapping_add(0x9), c.r[0]);
                pc = 0x801336e8;
            }
            0x801336e8 => {
                c.r[4] = c.lbz(c.r[1].wrapping_add(0x9));
                c.r[0] = c.r[4] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8013372c; continue; }
                pc = 0x801336f4;
            }
            0x801336f4 => {
                c.r[3] = c.lbz(c.r[1].wrapping_add(0x8));
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x9), c.r[0]);
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.stb(c.r[1].wrapping_add(0x8), c.r[3]);
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8013377c; continue; }
                pc = 0x80133714;
            }
            0x80133714 => {
                c.r[4] = c.r[5] as u8 as i8 as i32 as u32;
                c.r[3] = c.r[4].wrapping_add(0xffffffff);
                c.r[0] = c.r[31].wrapping_sub(c.r[4]);
                c.stb(c.r[1].wrapping_add(0x9), c.r[3]);
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x8013377c; continue;
                pc = 0x8013372c;
            }
            0x8013372c => {
                c.r[0] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[3] = c.r[5] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], c.r[3]);
                if c.crbit(0) { pc = 0x8013377c; continue; }
                pc = 0x8013373c;
            }
            0x8013373c => {
                c.r[5] = c.r[3].wrapping_add(0xffffffff);
                c.r[3] = c.lbz(c.r[1].wrapping_add(0x8));
                c.r[0] = c.r[4].wrapping_sub(c.r[5]);
                c.stb(c.r[1].wrapping_add(0x9), c.r[5]);
                c.r[4] = c.r[3].wrapping_add(c.r[0]);
                c.r[0] = c.r[5].rotate_left(0) & 0xff;
                c.stb(c.r[1].wrapping_add(0x8), c.r[4]);
                c.r[3] = c.r[4].rotate_left(0) & 0xff;
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.cmps(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x8013377c; continue; }
                pc = 0x80133770;
            }
            0x80133770 => {
                c.r[0] = 0x0;
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x9), c.r[0]);
                pc = 0x8013377c;
            }
            0x8013377c => {
                c.r[0] = c.lhz(c.r[1].wrapping_add(0x8));
                c.sth(c.r[30].wrapping_add(0x94), c.r[0]);
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80133664: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuGetEnable (0x801337a0, 8 bytes)
#[inline(never)]
pub fn f_801337a0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801337a0) { return; }
        if c.prof { crate::prof::enter(c, 0x801337a0); f_801337a0_b(c, 0x801337a0); crate::prof::exit(c, 0x801337a0); return; }
    }
    f_801337a0_b(c, 0x801337a0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801337a0_at(c: &mut Cpu, pc: u32) { f_801337a0_b(c, pc) }
#[inline(always)]
fn f_801337a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801337a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801337a0, pc); }
        match pc {
            0x801337a0 => {
                c.r[3] = c.lbz(c.r[13].wrapping_add(0xffffa6b0));
                if true { return; }
                return;
            }
            _ => panic!("f_801337a0: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuSetEnable (0x801337a8, 8 bytes)
#[inline(never)]
pub fn f_801337a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801337a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801337a8); f_801337a8_b(c, 0x801337a8); crate::prof::exit(c, 0x801337a8); return; }
    }
    f_801337a8_b(c, 0x801337a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801337a8_at(c: &mut Cpu, pc: u32) { f_801337a8_b(c, pc) }
#[inline(always)]
fn f_801337a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801337a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801337a8, pc); }
        match pc {
            0x801337a8 => {
                c.stb(c.r[13].wrapping_add(0xffffa6b0), c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_801337a8: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuIsOpen (0x801337b0, 52 bytes)
#[inline(never)]
pub fn f_801337b0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801337b0) { return; }
        if c.prof { crate::prof::enter(c, 0x801337b0); f_801337b0_b(c, 0x801337b0); crate::prof::exit(c, 0x801337b0); return; }
    }
    f_801337b0_b(c, 0x801337b0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801337b0_at(c: &mut Cpu, pc: u32) { f_801337b0_b(c, pc) }
#[inline(always)]
fn f_801337b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801337b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801337b0, pc); }
        match pc {
            0x801337b0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                c.lr = 0x801337c4; f_80102620(c);
                pc = 0x801337c4;
            }
            0x801337c4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_neg();
                c.r[0] = c.r[0] | c.r[3];
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801337b0: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuClose (0x801337e4, 44 bytes)
#[inline(never)]
pub fn f_801337e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801337e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801337e4); f_801337e4_b(c, 0x801337e4); crate::prof::exit(c, 0x801337e4); return; }
    }
    f_801337e4_b(c, 0x801337e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801337e4_at(c: &mut Cpu, pc: u32) { f_801337e4_b(c, pc) }
#[inline(always)]
fn f_801337e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801337e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801337e4, pc); }
        match pc {
            0x801337e4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = 0x0;
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                c.stb(c.r[13].wrapping_add(0xffffa6b1), c.r[0]);
                c.lr = 0x80133800; f_80102510(c);
                pc = 0x80133800;
            }
            0x80133800 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801337e4: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuMain (0x80133810, 148 bytes)
#[inline(never)]
pub fn f_80133810(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133810) { return; }
        if c.prof { crate::prof::enter(c, 0x80133810); f_80133810_b(c, 0x80133810); crate::prof::exit(c, 0x80133810); return; }
    }
    f_80133810_b(c, 0x80133810)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133810_at(c: &mut Cpu, pc: u32) { f_80133810_b(c, pc) }
#[inline(always)]
fn f_80133810_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133810;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133810, pc); }
        match pc {
            0x80133810 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8768));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133838; continue; }
                pc = 0x80133830;
            }
            0x80133830 => {
                c.r[3] = 0x0;
                pc = 0x80133840; continue;
                pc = 0x80133838;
            }
            0x80133838 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133840; (c.dispatch)(c, t); }
                pc = 0x80133840;
            }
            0x80133840 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80133890; continue; }
                pc = 0x80133848;
            }
            0x80133848 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa6b0));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80133890; continue; }
                pc = 0x80133854;
            }
            0x80133854 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                c.lr = 0x8013385c; f_80102620(c);
                pc = 0x8013385c;
            }
            0x8013385c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.r[3].wrapping_neg();
                c.r[0] = c.r[0] | c.r[3];
                c.r[0] = c.r[0].rotate_left(1) & 0x1;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80133890; continue; }
                pc = 0x80133870;
            }
            0x80133870 => {
                c.stb(c.r[13].wrapping_add(0xffffa6b1), c.r[31]);
                pc = 0x80133874;
            }
            0x80133874 => {
                c.r[3] = 0x0;
                c.lr = 0x8013387c; f_801338a4(c);
                pc = 0x8013387c;
            }
            0x8013387c => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(0) { pc = 0x80133890; continue; }
                pc = 0x80133884;
            }
            0x80133884 => {
                c.r[0] = c.lbz(c.r[13].wrapping_add(0xffffa6b1));
                c.cmpu(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80133874; continue; }
                pc = 0x80133890;
            }
            0x80133890 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133810: bad pc {:#x}", pc),
        }
    }
}

/// _dbgMenuSub__Fl (0x801338a4, 684 bytes)
#[inline(never)]
pub fn f_801338a4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801338a4) { return; }
        if c.prof { crate::prof::enter(c, 0x801338a4); f_801338a4_b(c, 0x801338a4); crate::prof::exit(c, 0x801338a4); return; }
    }
    f_801338a4_b(c, 0x801338a4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801338a4_at(c: &mut Cpu, pc: u32) { f_801338a4_b(c, pc) }
#[inline(always)]
fn f_801338a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801338a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801338a4, pc); }
        match pc {
            0x801338a4 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.r[27] = c.r[29].wrapping_add(0xffffffff);
                c.r[26] = 0x0;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                c.r[28] = c.r[0].wrapping_add(c.r[29]);
                pc = 0x801338c8;
            }
            0x801338c8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                { let v = c.r[29] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[3] = (v >> 31) as u32; }
                c.r[0] = c.r[0].wrapping_add(c.r[27]);
                c.r[25] = c.r[0] & !c.r[3];
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x801338e0; f_80133c3c(c);
                pc = 0x801338e0;
            }
            0x801338e0 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffffa6bc));
                c.r[5] = c.r[3].rotate_left(2) & 0xfffffffc;
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[25] | c.r[25];
                c.r[30] = c.r[0].wrapping_add(c.r[5]);
                c.r[6] = 0x0;
                c.r[5] = c.r[30] | c.r[30];
                c.r[7] = 0x1;
                c.r[8] = 0x0;
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8013390c; f_801026a4(c);
                pc = 0x8013390c;
            }
            0x8013390c => {
                c.r[25] = c.r[3] | c.r[3];
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80133918; f_80104704(c);
                pc = 0x80133918;
            }
            0x80133918 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x8013393c; continue; }
                pc = 0x80133920;
            }
            0x80133920 => {
                c.r[3] = c.lbz(c.r[31].wrapping_add(0x94));
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x95));
                c.r[3] = c.r[3] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                c.stw(c.r[30], c.r[0]);
                pc = 0x80133944; continue;
                pc = 0x8013393c;
            }
            0x8013393c => {
                c.r[0] = 0x0;
                c.stw(c.r[30], c.r[0]);
                pc = 0x80133944;
            }
            0x80133944 => {
                c.cmps(0, c.r[25], 0xffffffff);
                if !c.crbit(2) { pc = 0x8013395c; continue; }
                pc = 0x8013394c;
            }
            0x8013394c => {
                c.cmps(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x80133b30; continue; }
                pc = 0x80133954;
            }
            0x80133954 => {
                c.r[26] = 0xffffffff;
                pc = 0x80133b30; continue;
                pc = 0x8013395c;
            }
            0x8013395c => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x94));
                c.r[3] = c.r[31] | c.r[31];
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x95));
                c.r[4] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.lr = 0x80133978; f_80133e6c(c);
                pc = 0x80133978;
            }
            0x80133978 => {
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if !c.crbit(1) { pc = 0x801339a4; continue; }
                pc = 0x80133980;
            }
            0x80133980 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8768));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133994; continue; }
                pc = 0x8013398c;
            }
            0x8013398c => {
                c.r[3] = 0x0;
                pc = 0x8013399c; continue;
                pc = 0x80133994;
            }
            0x80133994 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x8013399c; (c.dispatch)(c, t); }
                pc = 0x8013399c;
            }
            0x8013399c => {
                c.cmps(0, c.r[3], c.r[30]);
                if c.crbit(1) { pc = 0x801339ac; continue; }
                pc = 0x801339a4;
            }
            0x801339a4 => {
                c.r[25] = 0x0;
                pc = 0x80133a14; continue;
                pc = 0x801339ac;
            }
            0x801339ac => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x801339c0; continue; }
                pc = 0x801339b8;
            }
            0x801339b8 => {
                c.r[3] = 0x0;
                pc = 0x801339cc; continue;
                pc = 0x801339c0;
            }
            0x801339c0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x801339cc; (c.dispatch)(c, t); }
                pc = 0x801339cc;
            }
            0x801339cc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801339dc; continue; }
                pc = 0x801339d4;
            }
            0x801339d4 => {
                c.r[25] = 0x0;
                pc = 0x801339e0; continue;
                pc = 0x801339dc;
            }
            0x801339dc => {
                c.r[25] = c.lha(c.r[3].wrapping_add(0x2));
                pc = 0x801339e0;
            }
            0x801339e0 => {
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(1) { pc = 0x80133a10; continue; }
                pc = 0x801339e8;
            }
            0x801339e8 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8768));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x801339fc; continue; }
                pc = 0x801339f4;
            }
            0x801339f4 => {
                c.r[3] = 0x0;
                pc = 0x80133a04; continue;
                pc = 0x801339fc;
            }
            0x801339fc => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133a04; (c.dispatch)(c, t); }
                pc = 0x80133a04;
            }
            0x80133a04 => {
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(1) { pc = 0x80133a14; continue; }
                pc = 0x80133a10;
            }
            0x80133a10 => {
                c.r[25] = 0x0;
                pc = 0x80133a14;
            }
            0x80133a14 => {
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80133ac4; continue; }
                pc = 0x80133a1c;
            }
            0x80133a1c => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133a30; continue; }
                pc = 0x80133a28;
            }
            0x80133a28 => {
                c.r[3] = 0x0;
                pc = 0x80133a3c; continue;
                pc = 0x80133a30;
            }
            0x80133a30 => {
                c.r[3] = c.r[30] | c.r[30];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133a3c; (c.dispatch)(c, t); }
                pc = 0x80133a3c;
            }
            0x80133a3c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80133a4c; continue; }
                pc = 0x80133a44;
            }
            0x80133a44 => {
                c.r[12] = 0x0;
                pc = 0x80133a50; continue;
                pc = 0x80133a4c;
            }
            0x80133a4c => {
                c.r[12] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x80133a50;
            }
            0x80133a50 => {
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x80133a7c; continue; }
                pc = 0x80133a58;
            }
            0x80133a58 => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x94));
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x95));
                c.r[4] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133a78; (c.dispatch)(c, t); }
                pc = 0x80133a78;
            }
            0x80133a78 => {
                pc = 0x80133a80; continue;
                pc = 0x80133a7c;
            }
            0x80133a7c => {
                c.r[3] = 0x1;
                pc = 0x80133a80;
            }
            0x80133a80 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80133a98; continue; }
                pc = 0x80133a88;
            }
            0x80133a88 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                c.lr = 0x80133a90; f_80102510(c);
                pc = 0x80133a90;
            }
            0x80133a90 => {
                c.r[3] = 0x1;
                pc = 0x80133b3c; continue;
                pc = 0x80133a98;
            }
            0x80133a98 => {
                c.cmps(0, c.r[3], 0xffffffff);
                if c.crbit(2) { pc = 0x801338c8; continue; }
                pc = 0x80133aa0;
            }
            0x80133aa0 => {
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x801338c8; continue; }
                pc = 0x80133aac;
            }
            0x80133aac => {
                c.r[3] = c.r[29].wrapping_add(0x1);
                c.lr = 0x80133ab4; f_801338a4(c);
                pc = 0x80133ab4;
            }
            0x80133ab4 => {
                c.cmps(0, c.r[3], 0x1);
                if !c.crbit(2) { pc = 0x801338c8; continue; }
                pc = 0x80133abc;
            }
            0x80133abc => {
                c.r[3] = 0x1;
                pc = 0x80133b3c; continue;
                pc = 0x80133ac4;
            }
            0x80133ac4 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                c.lr = 0x80133acc; f_80102510(c);
                pc = 0x80133acc;
            }
            0x80133acc => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133ae0; continue; }
                pc = 0x80133ad8;
            }
            0x80133ad8 => {
                c.r[3] = 0x0;
                pc = 0x80133aec; continue;
                pc = 0x80133ae0;
            }
            0x80133ae0 => {
                c.r[3] = c.r[30] | c.r[30];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133aec; (c.dispatch)(c, t); }
                pc = 0x80133aec;
            }
            0x80133aec => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80133afc; continue; }
                pc = 0x80133af4;
            }
            0x80133af4 => {
                c.r[12] = 0x0;
                pc = 0x80133b00; continue;
                pc = 0x80133afc;
            }
            0x80133afc => {
                c.r[12] = c.lwz(c.r[3].wrapping_add(0x8));
                pc = 0x80133b00;
            }
            0x80133b00 => {
                c.cmpu(0, c.r[12], 0x0);
                if c.crbit(2) { pc = 0x80133b28; continue; }
                pc = 0x80133b08;
            }
            0x80133b08 => {
                c.r[4] = c.lbz(c.r[31].wrapping_add(0x94));
                c.r[3] = c.r[30] | c.r[30];
                c.r[0] = c.lbz(c.r[31].wrapping_add(0x95));
                c.r[4] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133b28; (c.dispatch)(c, t); }
                pc = 0x80133b28;
            }
            0x80133b28 => {
                c.r[3] = 0x1;
                pc = 0x80133b3c; continue;
                pc = 0x80133b30;
            }
            0x80133b30 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80133b38; f_80102510(c);
                pc = 0x80133b38;
            }
            0x80133b38 => {
                c.r[3] = c.r[26] | c.r[26];
                pc = 0x80133b3c;
            }
            0x80133b3c => {
                { let mut ea = c.r[1].wrapping_add(0x14); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_801338a4: bad pc {:#x}", pc),
        }
    }
}

/// _dbgMenuGetMenuNum__FP14tagWINDOW_WORKPl (0x80133b50, 148 bytes)
#[inline(never)]
pub fn f_80133b50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133b50) { return; }
        if c.prof { crate::prof::enter(c, 0x80133b50); f_80133b50_b(c, 0x80133b50); crate::prof::exit(c, 0x80133b50); return; }
    }
    f_80133b50_b(c, 0x80133b50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133b50_at(c: &mut Cpu, pc: u32) { f_80133b50_b(c, pc) }
#[inline(always)]
fn f_80133b50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133b50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133b50, pc); }
        match pc {
            0x80133b50 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.rc(c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x80133b7c; continue; }
                pc = 0x80133b74;
            }
            0x80133b74 => {
                c.r[0] = 0x0;
                c.stw(c.r[30], c.r[0]);
                pc = 0x80133b7c;
            }
            0x80133b7c => {
                c.r[31] = 0x0;
                pc = 0x80133b80;
            }
            0x80133b80 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80133b8c; f_80133e1c(c);
                pc = 0x80133b8c;
            }
            0x80133b8c => {
                c.cmpu(0, c.r[30], 0x0);
                if c.crbit(2) { pc = 0x80133bac; continue; }
                pc = 0x80133b94;
            }
            0x80133b94 => {
                c.lr = 0x80133b98; f_800fa444(c);
                pc = 0x80133b98;
            }
            0x80133b98 => {
                c.r[0] = c.lwz(c.r[30]);
                c.r[3] = c.r[3].rotate_left(16) & 0xffff;
                c.cmps(0, c.r[0], c.r[3]);
                if !c.crbit(0) { pc = 0x80133bac; continue; }
                pc = 0x80133ba8;
            }
            0x80133ba8 => {
                c.stw(c.r[30], c.r[3]);
                pc = 0x80133bac;
            }
            0x80133bac => {
                c.r[4] = c.r[31] | c.r[31];
                c.r[3] = c.r[29] | c.r[29];
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.lr = 0x80133bbc; f_80133be4(c);
                pc = 0x80133bbc;
            }
            0x80133bbc => {
                c.cmps(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80133b80; continue; }
                pc = 0x80133bc4;
            }
            0x80133bc4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80133b50: bad pc {:#x}", pc),
        }
    }
}

/// _dbgMenuCheckTerminate__FP14tagWINDOW_WORKl (0x80133be4, 88 bytes)
#[inline(never)]
pub fn f_80133be4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133be4) { return; }
        if c.prof { crate::prof::enter(c, 0x80133be4); f_80133be4_b(c, 0x80133be4); crate::prof::exit(c, 0x80133be4); return; }
    }
    f_80133be4_b(c, 0x80133be4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133be4_at(c: &mut Cpu, pc: u32) { f_80133be4_b(c, pc) }
#[inline(always)]
fn f_80133be4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133be4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133be4, pc); }
        match pc {
            0x80133be4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80133bf4; f_80133e6c(c);
                pc = 0x80133bf4;
            }
            0x80133bf4 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133c08; continue; }
                pc = 0x80133c00;
            }
            0x80133c00 => {
                c.r[3] = 0x0;
                pc = 0x80133c10; continue;
                pc = 0x80133c08;
            }
            0x80133c08 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133c10; (c.dispatch)(c, t); }
                pc = 0x80133c10;
            }
            0x80133c10 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80133c20; continue; }
                pc = 0x80133c18;
            }
            0x80133c18 => {
                c.r[0] = 0x1;
                pc = 0x80133c28; continue;
                pc = 0x80133c20;
            }
            0x80133c20 => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(25) & 0x1;
                pc = 0x80133c28;
            }
            0x80133c28 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xff;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133be4: bad pc {:#x}", pc),
        }
    }
}

/// _dbgMenuGetMenuNo__Fl (0x80133c3c, 480 bytes)
#[inline(never)]
pub fn f_80133c3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133c3c) { return; }
        if c.prof { crate::prof::enter(c, 0x80133c3c); f_80133c3c_b(c, 0x80133c3c); crate::prof::exit(c, 0x80133c3c); return; }
    }
    f_80133c3c_b(c, 0x80133c3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133c3c_at(c: &mut Cpu, pc: u32) { f_80133c3c_b(c, pc) }
#[inline(always)]
fn f_80133c3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133c3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133c3c, pc); }
        match pc {
            0x80133c3c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.lr = 0x80133c58; f_80104704(c);
                pc = 0x80133c58;
            }
            0x80133c58 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80133d78; continue; }
                pc = 0x80133c60;
            }
            0x80133c60 => {
                c.r[4] = c.lbz(c.r[3].wrapping_add(0x94));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x95));
                c.r[4] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.lr = 0x80133c78; f_80133e6c(c);
                pc = 0x80133c78;
            }
            0x80133c78 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if !c.crbit(1) { pc = 0x80133ca4; continue; }
                pc = 0x80133c80;
            }
            0x80133c80 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8768));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133c94; continue; }
                pc = 0x80133c8c;
            }
            0x80133c8c => {
                c.r[3] = 0x0;
                pc = 0x80133c9c; continue;
                pc = 0x80133c94;
            }
            0x80133c94 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133c9c; (c.dispatch)(c, t); }
                pc = 0x80133c9c;
            }
            0x80133c9c => {
                c.cmps(0, c.r[3], c.r[31]);
                if c.crbit(1) { pc = 0x80133cac; continue; }
                pc = 0x80133ca4;
            }
            0x80133ca4 => {
                c.r[31] = 0x0;
                pc = 0x80133d14; continue;
                pc = 0x80133cac;
            }
            0x80133cac => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133cc0; continue; }
                pc = 0x80133cb8;
            }
            0x80133cb8 => {
                c.r[3] = 0x0;
                pc = 0x80133ccc; continue;
                pc = 0x80133cc0;
            }
            0x80133cc0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133ccc; (c.dispatch)(c, t); }
                pc = 0x80133ccc;
            }
            0x80133ccc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80133cdc; continue; }
                pc = 0x80133cd4;
            }
            0x80133cd4 => {
                c.r[31] = 0x0;
                pc = 0x80133ce0; continue;
                pc = 0x80133cdc;
            }
            0x80133cdc => {
                c.r[31] = c.lha(c.r[3].wrapping_add(0x2));
                pc = 0x80133ce0;
            }
            0x80133ce0 => {
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(1) { pc = 0x80133d10; continue; }
                pc = 0x80133ce8;
            }
            0x80133ce8 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8768));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133cfc; continue; }
                pc = 0x80133cf4;
            }
            0x80133cf4 => {
                c.r[3] = 0x0;
                pc = 0x80133d04; continue;
                pc = 0x80133cfc;
            }
            0x80133cfc => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133d04; (c.dispatch)(c, t); }
                pc = 0x80133d04;
            }
            0x80133d04 => {
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(1) { pc = 0x80133d14; continue; }
                pc = 0x80133d10;
            }
            0x80133d10 => {
                c.r[31] = 0x0;
                pc = 0x80133d14;
            }
            0x80133d14 => {
                c.r[31] = c.r[31] as u16 as i16 as i32 as u32;
                c.r[29] = 0x0;
                c.r[30] = 0x0;
                pc = 0x80133d6c; continue;
                pc = 0x80133d24;
            }
            0x80133d24 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133d38; continue; }
                pc = 0x80133d30;
            }
            0x80133d30 => {
                c.r[3] = 0x0;
                pc = 0x80133d44; continue;
                pc = 0x80133d38;
            }
            0x80133d38 => {
                c.r[3] = c.r[30] | c.r[30];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133d44; (c.dispatch)(c, t); }
                pc = 0x80133d44;
            }
            0x80133d44 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80133d54; continue; }
                pc = 0x80133d4c;
            }
            0x80133d4c => {
                c.r[0] = 0x1;
                pc = 0x80133d5c; continue;
                pc = 0x80133d54;
            }
            0x80133d54 => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(25) & 0x1;
                pc = 0x80133d5c;
            }
            0x80133d5c => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80133d68; continue; }
                pc = 0x80133d64;
            }
            0x80133d64 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x80133d68;
            }
            0x80133d68 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x80133d6c;
            }
            0x80133d6c => {
                c.cmps(0, c.r[30], c.r[31]);
                if c.crbit(0) { pc = 0x80133d24; continue; }
                pc = 0x80133d74;
            }
            0x80133d74 => {
                pc = 0x80133dfc; continue;
                pc = 0x80133d78;
            }
            0x80133d78 => {
                c.r[29] = 0x0;
                c.r[31] = 0x0;
                pc = 0x80133dcc; continue;
                pc = 0x80133d84;
            }
            0x80133d84 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133d98; continue; }
                pc = 0x80133d90;
            }
            0x80133d90 => {
                c.r[3] = 0x0;
                pc = 0x80133da4; continue;
                pc = 0x80133d98;
            }
            0x80133d98 => {
                c.r[3] = c.r[31] | c.r[31];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133da4; (c.dispatch)(c, t); }
                pc = 0x80133da4;
            }
            0x80133da4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80133db4; continue; }
                pc = 0x80133dac;
            }
            0x80133dac => {
                c.r[0] = 0x1;
                pc = 0x80133dbc; continue;
                pc = 0x80133db4;
            }
            0x80133db4 => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[0] = c.r[0].rotate_left(25) & 0x1;
                pc = 0x80133dbc;
            }
            0x80133dbc => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80133dc8; continue; }
                pc = 0x80133dc4;
            }
            0x80133dc4 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x80133dc8;
            }
            0x80133dc8 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x80133dcc;
            }
            0x80133dcc => {
                c.lr = 0x80133dd0; f_800057a8(c);
                pc = 0x80133dd0;
            }
            0x80133dd0 => {
                c.cmps(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x80133df0; continue; }
                pc = 0x80133dd8;
            }
            0x80133dd8 => {
                if !c.crbit(0) { pc = 0x80133de0; continue; }
                pc = 0x80133ddc;
            }
            0x80133ddc => {
                pc = 0x80133df0; continue;
                pc = 0x80133de0;
            }
            0x80133de0 => {
                c.cmps(0, c.r[3], 0x3);
                if !c.crbit(0) { pc = 0x80133df0; continue; }
                pc = 0x80133de8;
            }
            0x80133de8 => {
                c.r[0] = 0x115;
                pc = 0x80133df4; continue;
                pc = 0x80133df0;
            }
            0x80133df0 => {
                c.r[0] = 0x2;
                pc = 0x80133df4;
            }
            0x80133df4 => {
                c.cmps(0, c.r[31], c.r[0]);
                if c.crbit(0) { pc = 0x80133d84; continue; }
                pc = 0x80133dfc;
            }
            0x80133dfc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[3] = c.r[29] | c.r[29];
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80133c3c: bad pc {:#x}", pc),
        }
    }
}

/// _dbgMenuGetMsgID__FP14tagWINDOW_WORKl (0x80133e1c, 80 bytes)
#[inline(never)]
pub fn f_80133e1c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133e1c) { return; }
        if c.prof { crate::prof::enter(c, 0x80133e1c); f_80133e1c_b(c, 0x80133e1c); crate::prof::exit(c, 0x80133e1c); return; }
    }
    f_80133e1c_b(c, 0x80133e1c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133e1c_at(c: &mut Cpu, pc: u32) { f_80133e1c_b(c, pc) }
#[inline(always)]
fn f_80133e1c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133e1c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133e1c, pc); }
        match pc {
            0x80133e1c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80133e2c; f_80133e6c(c);
                pc = 0x80133e2c;
            }
            0x80133e2c => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80133e40; continue; }
                pc = 0x80133e38;
            }
            0x80133e38 => {
                c.r[3] = 0x0;
                pc = 0x80133e48; continue;
                pc = 0x80133e40;
            }
            0x80133e40 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80133e48; (c.dispatch)(c, t); }
                pc = 0x80133e48;
            }
            0x80133e48 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80133e58; continue; }
                pc = 0x80133e50;
            }
            0x80133e50 => {
                c.r[3] = 0x0;
                pc = 0x80133e5c; continue;
                pc = 0x80133e58;
            }
            0x80133e58 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                pc = 0x80133e5c;
            }
            0x80133e5c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80133e1c: bad pc {:#x}", pc),
        }
    }
}

/// _dbgMenuGetItemNo__FP14tagWINDOW_WORKl (0x80133e6c, 760 bytes)
#[inline(never)]
pub fn f_80133e6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80133e6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80133e6c); f_80133e6c_b(c, 0x80133e6c); crate::prof::exit(c, 0x80133e6c); return; }
    }
    f_80133e6c_b(c, 0x80133e6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80133e6c_at(c: &mut Cpu, pc: u32) { f_80133e6c_b(c, pc) }
#[inline(always)]
fn f_80133e6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80133e6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80133e6c, pc); }
        match pc {
            0x80133e6c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[3] = 0xffffffff;
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                c.cmps(0, c.r[0], c.r[4]);
                if c.crbit(0) { pc = 0x80133ea0; continue; }
                pc = 0x80133e9c;
            }
            0x80133e9c => {
                c.r[3] = c.r[0].wrapping_sub(c.r[4]);
                pc = 0x80133ea0;
            }
            0x80133ea0 => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x80133eb0; continue; }
                pc = 0x80133ea8;
            }
            0x80133ea8 => {
                c.r[3] = 0x0;
                pc = 0x80134148; continue;
                pc = 0x80133eb0;
            }
            0x80133eb0 => {
                if !c.crbit(2) { pc = 0x80133ee0; continue; }
                pc = 0x80133eb4;
            }
            0x80133eb4 => {
                c.lr = 0x80133eb8; f_800057a8(c);
                pc = 0x80133eb8;
            }
            0x80133eb8 => {
                c.cmps(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x80133ed8; continue; }
                pc = 0x80133ec0;
            }
            0x80133ec0 => {
                if !c.crbit(0) { pc = 0x80133ec8; continue; }
                pc = 0x80133ec4;
            }
            0x80133ec4 => {
                pc = 0x80133ed8; continue;
                pc = 0x80133ec8;
            }
            0x80133ec8 => {
                c.cmps(0, c.r[3], 0x3);
                if !c.crbit(0) { pc = 0x80133ed8; continue; }
                pc = 0x80133ed0;
            }
            0x80133ed0 => {
                c.r[0] = 0x115;
                pc = 0x80134144; continue;
                pc = 0x80133ed8;
            }
            0x80133ed8 => {
                c.r[0] = 0x2;
                pc = 0x80134144; continue;
                pc = 0x80133ee0;
            }
            0x80133ee0 => {
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.r[3] = c.r[3].wrapping_add(c.r[4]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[4] = (v >> 31) as u32; }
                c.r[0] = c.r[3].wrapping_add(0xffffffff);
                c.r[3] = c.r[0] & !c.r[4];
                c.lr = 0x80133ef8; f_80104704(c);
                pc = 0x80133ef8;
            }
            0x80133ef8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80133f30; continue; }
                pc = 0x80133f00;
            }
            0x80133f00 => {
                c.lr = 0x80133f04; f_800057a8(c);
                pc = 0x80133f04;
            }
            0x80133f04 => {
                c.cmps(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x80133f24; continue; }
                pc = 0x80133f0c;
            }
            0x80133f0c => {
                if !c.crbit(0) { pc = 0x80133f14; continue; }
                pc = 0x80133f10;
            }
            0x80133f10 => {
                pc = 0x80133f24; continue;
                pc = 0x80133f14;
            }
            0x80133f14 => {
                c.cmps(0, c.r[3], 0x3);
                if !c.crbit(0) { pc = 0x80133f24; continue; }
                pc = 0x80133f1c;
            }
            0x80133f1c => {
                c.r[0] = 0x115;
                pc = 0x80133f28; continue;
                pc = 0x80133f24;
            }
            0x80133f24 => {
                c.r[0] = 0x2;
                pc = 0x80133f28;
            }
            0x80133f28 => {
                c.r[29] = c.r[0] | c.r[0];
                pc = 0x801340a4; continue;
                pc = 0x80133f30;
            }
            0x80133f30 => {
                c.r[7] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[4] = 0xffffffff;
                c.r[6] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                c.r[5] = c.lbz(c.r[3].wrapping_add(0x94));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x95));
                c.cmps(0, c.r[7], c.r[6]);
                c.r[3] = c.r[5] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[30] = c.r[3].wrapping_add(c.r[0]);
                if c.crbit(0) { pc = 0x80133f5c; continue; }
                pc = 0x80133f58;
            }
            0x80133f58 => {
                c.r[4] = c.r[7].wrapping_sub(c.r[6]);
                pc = 0x80133f5c;
            }
            0x80133f5c => {
                c.cmps(0, c.r[4], 0x0);
                if !c.crbit(0) { pc = 0x80133f6c; continue; }
                pc = 0x80133f64;
            }
            0x80133f64 => {
                c.r[0] = 0x0;
                pc = 0x801340a0; continue;
                pc = 0x80133f6c;
            }
            0x80133f6c => {
                if !c.crbit(2) { pc = 0x80133f9c; continue; }
                pc = 0x80133f70;
            }
            0x80133f70 => {
                c.lr = 0x80133f74; f_800057a8(c);
                pc = 0x80133f74;
            }
            0x80133f74 => {
                c.cmps(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x80133f94; continue; }
                pc = 0x80133f7c;
            }
            0x80133f7c => {
                if !c.crbit(0) { pc = 0x80133f84; continue; }
                pc = 0x80133f80;
            }
            0x80133f80 => {
                pc = 0x80133f94; continue;
                pc = 0x80133f84;
            }
            0x80133f84 => {
                c.cmps(0, c.r[3], 0x3);
                if !c.crbit(0) { pc = 0x80133f94; continue; }
                pc = 0x80133f8c;
            }
            0x80133f8c => {
                c.r[0] = 0x115;
                pc = 0x8013409c; continue;
                pc = 0x80133f94;
            }
            0x80133f94 => {
                c.r[0] = 0x2;
                pc = 0x8013409c; continue;
                pc = 0x80133f9c;
            }
            0x80133f9c => {
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[3] = (v >> 31) as u32; }
                c.r[0] = c.r[6].wrapping_add(c.r[0]);
                c.r[3] = c.r[0] & !c.r[3];
                c.lr = 0x80133fb0; f_80104704(c);
                pc = 0x80133fb0;
            }
            0x80133fb0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80133fe4; continue; }
                pc = 0x80133fb8;
            }
            0x80133fb8 => {
                c.lr = 0x80133fbc; f_800057a8(c);
                pc = 0x80133fbc;
            }
            0x80133fbc => {
                c.cmps(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x80133fdc; continue; }
                pc = 0x80133fc4;
            }
            0x80133fc4 => {
                if !c.crbit(0) { pc = 0x80133fcc; continue; }
                pc = 0x80133fc8;
            }
            0x80133fc8 => {
                pc = 0x80133fdc; continue;
                pc = 0x80133fcc;
            }
            0x80133fcc => {
                c.cmps(0, c.r[3], 0x3);
                if !c.crbit(0) { pc = 0x80133fdc; continue; }
                pc = 0x80133fd4;
            }
            0x80133fd4 => {
                c.r[29] = 0x115;
                pc = 0x80134054; continue;
                pc = 0x80133fdc;
            }
            0x80133fdc => {
                c.r[29] = 0x2;
                pc = 0x80134054; continue;
                pc = 0x80133fe4;
            }
            0x80133fe4 => {
                c.r[4] = c.lbz(c.r[3].wrapping_add(0x94));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x95));
                c.r[4] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[29] = c.r[4].wrapping_add(c.r[0]);
                c.lr = 0x80133ffc; f_80134258(c);
                pc = 0x80133ffc;
            }
            0x80133ffc => {
                c.cmps(0, c.r[3], 0x0);
                if !c.crbit(0) { pc = 0x8013400c; continue; }
                pc = 0x80134004;
            }
            0x80134004 => {
                c.r[29] = 0x0;
                pc = 0x80134054; continue;
                pc = 0x8013400c;
            }
            0x8013400c => {
                if !c.crbit(2) { pc = 0x80134018; continue; }
                pc = 0x80134010;
            }
            0x80134010 => {
                c.lr = 0x80134014; f_80134274(c);
                pc = 0x80134014;
            }
            0x80134014 => {
                pc = 0x80134050; continue;
                pc = 0x80134018;
            }
            0x80134018 => {
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                c.lr = 0x80134020; f_80134228(c);
                pc = 0x80134020;
            }
            0x80134020 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80134030; continue; }
                pc = 0x80134028;
            }
            0x80134028 => {
                c.lr = 0x8013402c; f_80134274(c);
                pc = 0x8013402c;
            }
            0x8013402c => {
                pc = 0x80134048; continue;
                pc = 0x80134030;
            }
            0x80134030 => {
                c.r[4] = c.lbz(c.r[3].wrapping_add(0x94));
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x95));
                c.r[4] = c.r[4] as u8 as i8 as i32 as u32;
                c.r[0] = c.r[0] as u8 as i8 as i32 as u32;
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.lr = 0x80134048; f_80133e6c(c);
                pc = 0x80134048;
            }
            0x80134048 => {
                c.lr = 0x8013404c; f_80134164(c);
                pc = 0x8013404c;
            }
            0x8013404c => {
                c.r[3] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x80134050;
            }
            0x80134050 => {
                c.r[29] = c.r[3].wrapping_add(c.r[29]);
                pc = 0x80134054;
            }
            0x80134054 => {
                c.cmps(0, c.r[29], 0x0);
                if !c.crbit(1) { pc = 0x80134068; continue; }
                pc = 0x8013405c;
            }
            0x8013405c => {
                c.lr = 0x80134060; f_80134304(c);
                pc = 0x80134060;
            }
            0x80134060 => {
                c.cmps(0, c.r[3], c.r[29]);
                if c.crbit(1) { pc = 0x80134070; continue; }
                pc = 0x80134068;
            }
            0x80134068 => {
                c.r[29] = 0x0;
                pc = 0x80134098; continue;
                pc = 0x80134070;
            }
            0x80134070 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80134078; f_801342b8(c);
                pc = 0x80134078;
            }
            0x80134078 => {
                c.r[29] = c.r[3] | c.r[3];
                c.r[0] = c.r[29] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(1) { pc = 0x80134094; continue; }
                pc = 0x80134084;
            }
            0x80134084 => {
                c.lr = 0x80134088; f_80134304(c);
                pc = 0x80134088;
            }
            0x80134088 => {
                c.r[0] = c.r[29] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(1) { pc = 0x80134098; continue; }
                pc = 0x80134094;
            }
            0x80134094 => {
                c.r[29] = 0x0;
                pc = 0x80134098;
            }
            0x80134098 => {
                c.r[0] = c.r[29] as u16 as i16 as i32 as u32;
                pc = 0x8013409c;
            }
            0x8013409c => {
                c.r[0] = c.r[0].wrapping_add(c.r[30]);
                pc = 0x801340a0;
            }
            0x801340a0 => {
                c.r[29] = c.r[0] | c.r[0];
                pc = 0x801340a4;
            }
            0x801340a4 => {
                c.cmps(0, c.r[29], 0x0);
                if !c.crbit(1) { pc = 0x801340d0; continue; }
                pc = 0x801340ac;
            }
            0x801340ac => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8768));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x801340c0; continue; }
                pc = 0x801340b8;
            }
            0x801340b8 => {
                c.r[3] = 0x0;
                pc = 0x801340c8; continue;
                pc = 0x801340c0;
            }
            0x801340c0 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x801340c8; (c.dispatch)(c, t); }
                pc = 0x801340c8;
            }
            0x801340c8 => {
                c.cmps(0, c.r[3], c.r[29]);
                if c.crbit(1) { pc = 0x801340d8; continue; }
                pc = 0x801340d0;
            }
            0x801340d0 => {
                c.r[30] = 0x0;
                pc = 0x80134140; continue;
                pc = 0x801340d8;
            }
            0x801340d8 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x801340ec; continue; }
                pc = 0x801340e4;
            }
            0x801340e4 => {
                c.r[3] = 0x0;
                pc = 0x801340f8; continue;
                pc = 0x801340ec;
            }
            0x801340ec => {
                c.r[3] = c.r[29] | c.r[29];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x801340f8; (c.dispatch)(c, t); }
                pc = 0x801340f8;
            }
            0x801340f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80134108; continue; }
                pc = 0x80134100;
            }
            0x80134100 => {
                c.r[30] = 0x0;
                pc = 0x8013410c; continue;
                pc = 0x80134108;
            }
            0x80134108 => {
                c.r[30] = c.lha(c.r[3].wrapping_add(0x2));
                pc = 0x8013410c;
            }
            0x8013410c => {
                c.r[0] = c.r[30] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(1) { pc = 0x8013413c; continue; }
                pc = 0x80134114;
            }
            0x80134114 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8768));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80134128; continue; }
                pc = 0x80134120;
            }
            0x80134120 => {
                c.r[3] = 0x0;
                pc = 0x80134130; continue;
                pc = 0x80134128;
            }
            0x80134128 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80134130; (c.dispatch)(c, t); }
                pc = 0x80134130;
            }
            0x80134130 => {
                c.r[0] = c.r[30] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(1) { pc = 0x80134140; continue; }
                pc = 0x8013413c;
            }
            0x8013413c => {
                c.r[30] = 0x0;
                pc = 0x80134140;
            }
            0x80134140 => {
                c.r[0] = c.r[30] as u16 as i16 as i32 as u32;
                pc = 0x80134144;
            }
            0x80134144 => {
                c.r[3] = c.r[0].wrapping_add(c.r[31]);
                pc = 0x80134148;
            }
            0x80134148 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80133e6c: bad pc {:#x}", pc),
        }
    }
}

/// _dbgMenuGetLink__Fl (0x80134164, 196 bytes)
#[inline(never)]
pub fn f_80134164(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134164) { return; }
        if c.prof { crate::prof::enter(c, 0x80134164); f_80134164_b(c, 0x80134164); crate::prof::exit(c, 0x80134164); return; }
    }
    f_80134164_b(c, 0x80134164)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134164_at(c: &mut Cpu, pc: u32) { f_80134164_b(c, pc) }
#[inline(always)]
fn f_80134164_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134164;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134164, pc); }
        match pc {
            0x80134164 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if !c.crbit(1) { pc = 0x801341a0; continue; }
                pc = 0x8013417c;
            }
            0x8013417c => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8768));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80134190; continue; }
                pc = 0x80134188;
            }
            0x80134188 => {
                c.r[3] = 0x0;
                pc = 0x80134198; continue;
                pc = 0x80134190;
            }
            0x80134190 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80134198; (c.dispatch)(c, t); }
                pc = 0x80134198;
            }
            0x80134198 => {
                c.cmps(0, c.r[3], c.r[31]);
                if c.crbit(1) { pc = 0x801341a8; continue; }
                pc = 0x801341a0;
            }
            0x801341a0 => {
                c.r[3] = 0x0;
                pc = 0x80134214; continue;
                pc = 0x801341a8;
            }
            0x801341a8 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x801341bc; continue; }
                pc = 0x801341b4;
            }
            0x801341b4 => {
                c.r[3] = 0x0;
                pc = 0x801341c8; continue;
                pc = 0x801341bc;
            }
            0x801341bc => {
                c.r[3] = c.r[31] | c.r[31];
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x801341c8; (c.dispatch)(c, t); }
                pc = 0x801341c8;
            }
            0x801341c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801341d8; continue; }
                pc = 0x801341d0;
            }
            0x801341d0 => {
                c.r[31] = 0x0;
                pc = 0x801341dc; continue;
                pc = 0x801341d8;
            }
            0x801341d8 => {
                c.r[31] = c.lha(c.r[3].wrapping_add(0x2));
                pc = 0x801341dc;
            }
            0x801341dc => {
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(1) { pc = 0x8013420c; continue; }
                pc = 0x801341e4;
            }
            0x801341e4 => {
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8768));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x801341f8; continue; }
                pc = 0x801341f0;
            }
            0x801341f0 => {
                c.r[3] = 0x0;
                pc = 0x80134200; continue;
                pc = 0x801341f8;
            }
            0x801341f8 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x80134200; (c.dispatch)(c, t); }
                pc = 0x80134200;
            }
            0x80134200 => {
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[3], c.r[0]);
                if c.crbit(1) { pc = 0x80134210; continue; }
                pc = 0x8013420c;
            }
            0x8013420c => {
                c.r[31] = 0x0;
                pc = 0x80134210;
            }
            0x80134210 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80134214;
            }
            0x80134214 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80134164: bad pc {:#x}", pc),
        }
    }
}

/// _dbgMenuGetWin__Fl (0x80134228, 48 bytes)
#[inline(never)]
pub fn f_80134228(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134228) { return; }
        if c.prof { crate::prof::enter(c, 0x80134228); f_80134228_b(c, 0x80134228); crate::prof::exit(c, 0x80134228); return; }
    }
    f_80134228_b(c, 0x80134228)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134228_at(c: &mut Cpu, pc: u32) { f_80134228_b(c, pc) }
#[inline(always)]
fn f_80134228_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134228;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134228, pc); }
        match pc {
            0x80134228 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 1 != 0); c.r[4] = (v >> 31) as u32; }
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                c.r[3] = c.r[0] & !c.r[4];
                c.lr = 0x80134248; f_80104704(c);
                pc = 0x80134248;
            }
            0x80134248 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80134228: bad pc {:#x}", pc),
        }
    }
}

/// _dbgMenuGetIndex__FP14tagWINDOW_WORK (0x80134258, 28 bytes)
#[inline(never)]
pub fn f_80134258(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134258) { return; }
        if c.prof { crate::prof::enter(c, 0x80134258); f_80134258_b(c, 0x80134258); crate::prof::exit(c, 0x80134258); return; }
    }
    f_80134258_b(c, 0x80134258)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134258_at(c: &mut Cpu, pc: u32) { f_80134258_b(c, pc) }
#[inline(always)]
fn f_80134258_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134258;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134258, pc); }
        match pc {
            0x80134258 => {
                c.r[4] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[3] = 0xffffffff;
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8028));
                c.cmps(0, c.r[4], c.r[0]);
                if c.crbit(0) { return; }
                pc = 0x8013426c;
            }
            0x8013426c => {
                c.r[3] = c.r[4].wrapping_sub(c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80134258: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuGetRootMenu (0x80134274, 68 bytes)
#[inline(never)]
pub fn f_80134274(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134274) { return; }
        if c.prof { crate::prof::enter(c, 0x80134274); f_80134274_b(c, 0x80134274); crate::prof::exit(c, 0x80134274); return; }
    }
    f_80134274_b(c, 0x80134274)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134274_at(c: &mut Cpu, pc: u32) { f_80134274_b(c, pc) }
#[inline(always)]
fn f_80134274_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134274;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134274, pc); }
        match pc {
            0x80134274 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.lr = 0x80134284; f_800057a8(c);
                pc = 0x80134284;
            }
            0x80134284 => {
                c.cmps(0, c.r[3], 0x1);
                if c.crbit(2) { pc = 0x801342a4; continue; }
                pc = 0x8013428c;
            }
            0x8013428c => {
                if !c.crbit(0) { pc = 0x80134294; continue; }
                pc = 0x80134290;
            }
            0x80134290 => {
                pc = 0x801342a4; continue;
                pc = 0x80134294;
            }
            0x80134294 => {
                c.cmps(0, c.r[3], 0x3);
                if !c.crbit(0) { pc = 0x801342a4; continue; }
                pc = 0x8013429c;
            }
            0x8013429c => {
                c.r[3] = 0x115;
                pc = 0x801342a8; continue;
                pc = 0x801342a4;
            }
            0x801342a4 => {
                c.r[3] = 0x2;
                pc = 0x801342a8;
            }
            0x801342a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80134274: bad pc {:#x}", pc),
        }
    }
}

/// dbgMenuGetLink__Fl (0x801342b8, 76 bytes)
#[inline(never)]
pub fn f_801342b8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801342b8) { return; }
        if c.prof { crate::prof::enter(c, 0x801342b8); f_801342b8_b(c, 0x801342b8); crate::prof::exit(c, 0x801342b8); return; }
    }
    f_801342b8_b(c, 0x801342b8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801342b8_at(c: &mut Cpu, pc: u32) { f_801342b8_b(c, pc) }
#[inline(always)]
fn f_801342b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801342b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801342b8, pc); }
        match pc {
            0x801342b8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff876c));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x801342d8; continue; }
                pc = 0x801342d0;
            }
            0x801342d0 => {
                c.r[3] = 0x0;
                pc = 0x801342e0; continue;
                pc = 0x801342d8;
            }
            0x801342d8 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x801342e0; (c.dispatch)(c, t); }
                pc = 0x801342e0;
            }
            0x801342e0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801342f0; continue; }
                pc = 0x801342e8;
            }
            0x801342e8 => {
                c.r[3] = 0x0;
                pc = 0x801342f4; continue;
                pc = 0x801342f0;
            }
            0x801342f0 => {
                c.r[3] = c.lha(c.r[3].wrapping_add(0x2));
                pc = 0x801342f4;
            }
            0x801342f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801342b8: bad pc {:#x}", pc),
        }
    }
}

/// debugMenuGetNum__Fv (0x80134304, 56 bytes)
#[inline(never)]
pub fn f_80134304(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134304) { return; }
        if c.prof { crate::prof::enter(c, 0x80134304); f_80134304_b(c, 0x80134304); crate::prof::exit(c, 0x80134304); return; }
    }
    f_80134304_b(c, 0x80134304)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134304_at(c: &mut Cpu, pc: u32) { f_80134304_b(c, pc) }
#[inline(always)]
fn f_80134304_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134304;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134304, pc); }
        match pc {
            0x80134304 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[12] = c.lwz(c.r[13].wrapping_add(0xffff8768));
                c.cmpu(0, c.r[12], 0x0);
                if !c.crbit(2) { pc = 0x80134324; continue; }
                pc = 0x8013431c;
            }
            0x8013431c => {
                c.r[3] = 0x0;
                pc = 0x8013432c; continue;
                pc = 0x80134324;
            }
            0x80134324 => {
                c.mtspr(9, c.r[12]);
                if true { let t = c.ctr; c.lr = 0x8013432c; (c.dispatch)(c, t); }
                pc = 0x8013432c;
            }
            0x8013432c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80134304: bad pc {:#x}", pc),
        }
    }
}

/// pcboxSwapItemSlot (0x8013433c, 228 bytes)
#[inline(never)]
pub fn f_8013433c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013433c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013433c); f_8013433c_b(c, 0x8013433c); crate::prof::exit(c, 0x8013433c); return; }
    }
    f_8013433c_b(c, 0x8013433c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013433c_at(c: &mut Cpu, pc: u32) { f_8013433c_b(c, pc) }
#[inline(always)]
fn f_8013433c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013433c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013433c, pc); }
        match pc {
            0x8013433c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if !c.crbit(2) { pc = 0x80134370; continue; }
                pc = 0x80134364;
            }
            0x80134364 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134370; f_80129280(c);
                pc = 0x80134370;
            }
            0x80134370 => {
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134384; continue; }
                pc = 0x80134378;
            }
            0x80134378 => {
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0xeb);
                if c.crbit(0) { pc = 0x8013438c; continue; }
                pc = 0x80134384;
            }
            0x80134384 => {
                c.r[31] = 0x0;
                pc = 0x80134398; continue;
                pc = 0x8013438c;
            }
            0x8013438c => {
                c.r[4] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[31] = c.r[4].wrapping_add(0x6dec);
                c.r[31] = c.r[3].wrapping_add(c.r[31]);
                pc = 0x80134398;
            }
            0x80134398 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x801343a8; continue; }
                pc = 0x801343a0;
            }
            0x801343a0 => {
                c.r[3] = 0x0;
                pc = 0x80134404; continue;
                pc = 0x801343a8;
            }
            0x801343a8 => {
                c.cmpu(0, c.r[29], 0x0);
                c.r[3] = c.r[29] | c.r[29];
                if !c.crbit(2) { pc = 0x801343c0; continue; }
                pc = 0x801343b4;
            }
            0x801343b4 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x801343c0; f_80129280(c);
                pc = 0x801343c0;
            }
            0x801343c0 => {
                c.r[0] = c.r[30] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x801343d4; continue; }
                pc = 0x801343c8;
            }
            0x801343c8 => {
                c.r[0] = c.r[30] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0xeb);
                if c.crbit(0) { pc = 0x801343dc; continue; }
                pc = 0x801343d4;
            }
            0x801343d4 => {
                c.r[4] = 0x0;
                pc = 0x801343e8; continue;
                pc = 0x801343dc;
            }
            0x801343dc => {
                c.r[4] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[4] = c.r[4].wrapping_add(0x6dec);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x801343e8;
            }
            0x801343e8 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x801343f8; continue; }
                pc = 0x801343f0;
            }
            0x801343f0 => {
                c.r[3] = 0x0;
                pc = 0x80134404; continue;
                pc = 0x801343f8;
            }
            0x801343f8 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80134400; f_80140a9c(c);
                pc = 0x80134400;
            }
            0x80134400 => {
                c.r[3] = 0x1;
                pc = 0x80134404;
            }
            0x80134404 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8013433c: bad pc {:#x}", pc),
        }
    }
}

/// pcboxGetItemCapacity (0x80134420, 356 bytes)
#[inline(never)]
pub fn f_80134420(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134420) { return; }
        if c.prof { crate::prof::enter(c, 0x80134420); f_80134420_b(c, 0x80134420); crate::prof::exit(c, 0x80134420); return; }
    }
    f_80134420_b(c, 0x80134420)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134420_at(c: &mut Cpu, pc: u32) { f_80134420_b(c, pc) }
#[inline(always)]
fn f_80134420_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134420;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134420, pc); }
        match pc {
            0x80134420 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x8013445c; continue; }
                pc = 0x8013444c;
            }
            0x8013444c => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134458; f_80129280(c);
                pc = 0x80134458;
            }
            0x80134458 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8013445c;
            }
            0x8013445c => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80134464; f_801440a0(c);
                pc = 0x80134464;
            }
            0x80134464 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80134474; continue; }
                pc = 0x8013446c;
            }
            0x8013446c => {
                c.r[3] = 0x0;
                pc = 0x80134564; continue;
                pc = 0x80134474;
            }
            0x80134474 => {
                c.r[29] = c.r[31] | c.r[31];
                c.r[28] = c.r[28].rotate_left(0) & 0xffff;
                c.r[30] = 0x0;
                pc = 0x80134480;
            }
            0x80134480 => {
                c.r[3] = c.r[29].wrapping_add(0x6dec);
                c.lr = 0x80134488; f_801429e8(c);
                pc = 0x80134488;
            }
            0x80134488 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x801344ac; continue; }
                pc = 0x80134490;
            }
            0x80134490 => {
                c.r[3] = c.r[29].wrapping_add(0x6dec);
                c.r[4] = 0x0;
                c.r[5] = 0x1b;
                c.r[6] = 0x0;
                c.lr = 0x801344a4; f_80142cf4(c);
                pc = 0x801344a4;
            }
            0x801344a4 => {
                c.cmps(0, c.r[28], c.r[3]);
                if c.crbit(2) { pc = 0x801344bc; continue; }
                pc = 0x801344ac;
            }
            0x801344ac => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[29] = c.r[29].wrapping_add(0x4);
                c.cmps(0, c.r[30], 0xeb);
                if c.crbit(0) { pc = 0x80134480; continue; }
                pc = 0x801344bc;
            }
            0x801344bc => {
                c.cmps(0, c.r[30], 0xeb);
                c.r[3] = 0xffffffff;
                if !c.crbit(0) { pc = 0x801344cc; continue; }
                pc = 0x801344c8;
            }
            0x801344c8 => {
                c.r[3] = c.r[30] as u16 as i16 as i32 as u32;
                pc = 0x801344cc;
            }
            0x801344cc => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134558; continue; }
                pc = 0x801344d4;
            }
            0x801344d4 => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x801344e8; continue; }
                pc = 0x801344dc;
            }
            0x801344dc => {
                c.r[0] = c.r[3] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0xeb);
                if c.crbit(0) { pc = 0x801344f0; continue; }
                pc = 0x801344e8;
            }
            0x801344e8 => {
                c.r[30] = 0x0;
                pc = 0x801344fc; continue;
                pc = 0x801344f0;
            }
            0x801344f0 => {
                c.r[3] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[30] = c.r[3].wrapping_add(0x6dec);
                c.r[30] = c.r[31].wrapping_add(c.r[30]);
                pc = 0x801344fc;
            }
            0x801344fc => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80134510; continue; }
                pc = 0x80134504;
            }
            0x80134504 => {
                c.r[3] = 0x10000;
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x80134544; continue;
                pc = 0x80134510;
            }
            0x80134510 => {
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80134518; f_801429e8(c);
                pc = 0x80134518;
            }
            0x80134518 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8013452c; continue; }
                pc = 0x80134520;
            }
            0x80134520 => {
                c.r[3] = 0x10000;
                c.r[3] = c.r[3].wrapping_add(0xffffffff);
                pc = 0x80134544; continue;
                pc = 0x8013452c;
            }
            0x8013452c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x0;
                c.r[5] = 0x1c;
                c.r[6] = 0x0;
                c.lr = 0x80134540; f_80142cf4(c);
                pc = 0x80134540;
            }
            0x80134540 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80134544;
            }
            0x80134544 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x3e7);
                if !c.crbit(1) { pc = 0x8013455c; continue; }
                pc = 0x80134550;
            }
            0x80134550 => {
                c.r[3] = 0x0;
                pc = 0x8013455c; continue;
                pc = 0x80134558;
            }
            0x80134558 => {
                c.r[3] = 0x0;
                pc = 0x8013455c;
            }
            0x8013455c => {
                c.r[0] = c.addc(!c.r[3], 0x3e7, 1);
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                pc = 0x80134564;
            }
            0x80134564 => {
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
            _ => panic!("f_80134420: bad pc {:#x}", pc),
        }
    }
}

/// pcboxAddItem (0x80134584, 248 bytes)
#[inline(never)]
pub fn f_80134584(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134584) { return; }
        if c.prof { crate::prof::enter(c, 0x80134584); f_80134584_b(c, 0x80134584); crate::prof::exit(c, 0x80134584); return; }
    }
    f_80134584_b(c, 0x80134584)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134584_at(c: &mut Cpu, pc: u32) { f_80134584_b(c, pc) }
#[inline(always)]
fn f_80134584_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134584;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134584, pc); }
        match pc {
            0x80134584 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x801345b8; continue; }
                pc = 0x801345a8;
            }
            0x801345a8 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x801345b4; f_80129280(c);
                pc = 0x801345b4;
            }
            0x801345b4 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x801345b8;
            }
            0x801345b8 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x801345c8; continue; }
                pc = 0x801345c0;
            }
            0x801345c0 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x80134668; continue;
                pc = 0x801345c8;
            }
            0x801345c8 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x801345d0; f_801440a0(c);
                pc = 0x801345d0;
            }
            0x801345d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801345e0; continue; }
                pc = 0x801345d8;
            }
            0x801345d8 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x80134668; continue;
                pc = 0x801345e0;
            }
            0x801345e0 => {
                c.r[29] = c.r[31] | c.r[31];
                c.r[28] = c.r[26].rotate_left(0) & 0xffff;
                c.r[30] = 0x0;
                pc = 0x801345ec;
            }
            0x801345ec => {
                c.r[3] = c.r[29].wrapping_add(0x6dec);
                c.lr = 0x801345f4; f_801429e8(c);
                pc = 0x801345f4;
            }
            0x801345f4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80134618; continue; }
                pc = 0x801345fc;
            }
            0x801345fc => {
                c.r[3] = c.r[29].wrapping_add(0x6dec);
                c.r[4] = 0x0;
                c.r[5] = 0x1b;
                c.r[6] = 0x0;
                c.lr = 0x80134610; f_80142cf4(c);
                pc = 0x80134610;
            }
            0x80134610 => {
                c.cmps(0, c.r[28], c.r[3]);
                if c.crbit(2) { pc = 0x80134628; continue; }
                pc = 0x80134618;
            }
            0x80134618 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[29] = c.r[29].wrapping_add(0x4);
                c.cmps(0, c.r[30], 0xeb);
                if c.crbit(0) { pc = 0x801345ec; continue; }
                pc = 0x80134628;
            }
            0x80134628 => {
                c.cmps(0, c.r[30], 0xeb);
                c.r[7] = 0xffffffff;
                if !c.crbit(0) { pc = 0x80134638; continue; }
                pc = 0x80134634;
            }
            0x80134634 => {
                c.r[7] = c.r[30] as u16 as i16 as i32 as u32;
                pc = 0x80134638;
            }
            0x80134638 => {
                c.r[0] = c.r[7] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x80134648; continue; }
                pc = 0x80134640;
            }
            0x80134640 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x80134668; continue;
                pc = 0x80134648;
            }
            0x80134648 => {
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[27] | c.r[27];
                c.r[3] = c.r[31].wrapping_add(0x6dec);
                c.r[4] = 0xeb;
                c.r[8] = 0x3e7;
                c.r[9] = 0x0;
                c.lr = 0x80134664; f_80140acc(c);
                pc = 0x80134664;
            }
            0x80134664 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80134668;
            }
            0x80134668 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80134584: bad pc {:#x}", pc),
        }
    }
}

/// pcboxDelItem (0x8013467c, 236 bytes)
#[inline(never)]
pub fn f_8013467c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013467c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013467c); f_8013467c_b(c, 0x8013467c); crate::prof::exit(c, 0x8013467c); return; }
    }
    f_8013467c_b(c, 0x8013467c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013467c_at(c: &mut Cpu, pc: u32) { f_8013467c_b(c, pc) }
#[inline(always)]
fn f_8013467c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013467c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013467c, pc); }
        match pc {
            0x8013467c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x801346b0; continue; }
                pc = 0x801346a0;
            }
            0x801346a0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x801346ac; f_80129280(c);
                pc = 0x801346ac;
            }
            0x801346ac => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x801346b0;
            }
            0x801346b0 => {
                c.r[0] = c.r[27].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x801346c0; continue; }
                pc = 0x801346b8;
            }
            0x801346b8 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x80134754; continue;
                pc = 0x801346c0;
            }
            0x801346c0 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x801346c8; f_801440a0(c);
                pc = 0x801346c8;
            }
            0x801346c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801346d8; continue; }
                pc = 0x801346d0;
            }
            0x801346d0 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x80134754; continue;
                pc = 0x801346d8;
            }
            0x801346d8 => {
                c.r[29] = c.r[31] | c.r[31];
                c.r[28] = c.r[26].rotate_left(0) & 0xffff;
                c.r[30] = 0x0;
                pc = 0x801346e4;
            }
            0x801346e4 => {
                c.r[3] = c.r[29].wrapping_add(0x6dec);
                c.lr = 0x801346ec; f_801429e8(c);
                pc = 0x801346ec;
            }
            0x801346ec => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80134710; continue; }
                pc = 0x801346f4;
            }
            0x801346f4 => {
                c.r[3] = c.r[29].wrapping_add(0x6dec);
                c.r[4] = 0x0;
                c.r[5] = 0x1b;
                c.r[6] = 0x0;
                c.lr = 0x80134708; f_80142cf4(c);
                pc = 0x80134708;
            }
            0x80134708 => {
                c.cmps(0, c.r[28], c.r[3]);
                if c.crbit(2) { pc = 0x80134720; continue; }
                pc = 0x80134710;
            }
            0x80134710 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[29] = c.r[29].wrapping_add(0x4);
                c.cmps(0, c.r[30], 0xeb);
                if c.crbit(0) { pc = 0x801346e4; continue; }
                pc = 0x80134720;
            }
            0x80134720 => {
                c.cmps(0, c.r[30], 0xeb);
                c.r[5] = c.r[26] | c.r[26];
                c.r[6] = c.r[27] | c.r[27];
                c.r[3] = c.r[31].wrapping_add(0x6dec);
                c.r[4] = 0xeb;
                c.r[7] = 0xffffffff;
                if !c.crbit(0) { pc = 0x80134740; continue; }
                pc = 0x8013473c;
            }
            0x8013473c => {
                c.r[7] = c.r[30] as u16 as i16 as i32 as u32;
                pc = 0x80134740;
            }
            0x80134740 => {
                c.r[8] = 0x3e7;
                c.r[9] = 0x0;
                c.r[10] = 0x0;
                c.lr = 0x80134750; f_80141308(c);
                pc = 0x80134750;
            }
            0x80134750 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                pc = 0x80134754;
            }
            0x80134754 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8013467c: bad pc {:#x}", pc),
        }
    }
}

/// pcboxGetItem (0x80134768, 104 bytes)
#[inline(never)]
pub fn f_80134768(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134768) { return; }
        if c.prof { crate::prof::enter(c, 0x80134768); f_80134768_b(c, 0x80134768); crate::prof::exit(c, 0x80134768); return; }
    }
    f_80134768_b(c, 0x80134768)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134768_at(c: &mut Cpu, pc: u32) { f_80134768_b(c, pc) }
#[inline(always)]
fn f_80134768_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134768;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134768, pc); }
        match pc {
            0x80134768 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x80134790; continue; }
                pc = 0x80134784;
            }
            0x80134784 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134790; f_80129280(c);
                pc = 0x80134790;
            }
            0x80134790 => {
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x801347a4; continue; }
                pc = 0x80134798;
            }
            0x80134798 => {
                c.r[0] = c.r[31] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0xeb);
                if c.crbit(0) { pc = 0x801347ac; continue; }
                pc = 0x801347a4;
            }
            0x801347a4 => {
                c.r[0] = 0x0;
                pc = 0x801347b8; continue;
                pc = 0x801347ac;
            }
            0x801347ac => {
                c.r[4] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[0] = c.r[4].wrapping_add(0x6dec);
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x801347b8;
            }
            0x801347b8 => {
                c.r[3] = c.r[0] | c.r[0];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80134768: bad pc {:#x}", pc),
        }
    }
}

/// pcboxGetNbItemSlot (0x801347d0, 8 bytes)
#[inline(never)]
pub fn f_801347d0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801347d0) { return; }
        if c.prof { crate::prof::enter(c, 0x801347d0); f_801347d0_b(c, 0x801347d0); crate::prof::exit(c, 0x801347d0); return; }
    }
    f_801347d0_b(c, 0x801347d0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801347d0_at(c: &mut Cpu, pc: u32) { f_801347d0_b(c, pc) }
#[inline(always)]
fn f_801347d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801347d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801347d0, pc); }
        match pc {
            0x801347d0 => {
                c.r[3] = 0xeb;
                if true { return; }
                return;
            }
            _ => panic!("f_801347d0: bad pc {:#x}", pc),
        }
    }
}

/// fn_801347D8 (0x801347d8, 8 bytes)
#[inline(never)]
pub fn f_801347d8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801347d8) { return; }
        if c.prof { crate::prof::enter(c, 0x801347d8); f_801347d8_b(c, 0x801347d8); crate::prof::exit(c, 0x801347d8); return; }
    }
    f_801347d8_b(c, 0x801347d8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801347d8_at(c: &mut Cpu, pc: u32) { f_801347d8_b(c, pc) }
#[inline(always)]
fn f_801347d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801347d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801347d8, pc); }
        match pc {
            0x801347d8 => {
                c.r[3] = 0x1e;
                if true { return; }
                return;
            }
            _ => panic!("f_801347d8: bad pc {:#x}", pc),
        }
    }
}

/// pcboxGetNbPokemonBox (0x801347e0, 8 bytes)
#[inline(never)]
pub fn f_801347e0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801347e0) { return; }
        if c.prof { crate::prof::enter(c, 0x801347e0); f_801347e0_b(c, 0x801347e0); crate::prof::exit(c, 0x801347e0); return; }
    }
    f_801347e0_b(c, 0x801347e0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801347e0_at(c: &mut Cpu, pc: u32) { f_801347e0_b(c, pc) }
#[inline(always)]
fn f_801347e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801347e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801347e0, pc); }
        match pc {
            0x801347e0 => {
                c.r[3] = 0x3;
                if true { return; }
                return;
            }
            _ => panic!("f_801347e0: bad pc {:#x}", pc),
        }
    }
}

/// pcboxGetPokemonBoxNbEmptySlot (0x801347e8, 260 bytes)
#[inline(never)]
pub fn f_801347e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801347e8) { return; }
        if c.prof { crate::prof::enter(c, 0x801347e8); f_801347e8_b(c, 0x801347e8); crate::prof::exit(c, 0x801347e8); return; }
    }
    f_801347e8_b(c, 0x801347e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801347e8_at(c: &mut Cpu, pc: u32) { f_801347e8_b(c, pc) }
#[inline(always)]
fn f_801347e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801347e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801347e8, pc); }
        match pc {
            0x801347e8 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x80134820; continue; }
                pc = 0x80134814;
            }
            0x80134814 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134820; f_80129280(c);
                pc = 0x80134820;
            }
            0x80134820 => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134834; continue; }
                pc = 0x80134828;
            }
            0x80134828 => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x8013483c; continue; }
                pc = 0x80134834;
            }
            0x80134834 => {
                c.r[30] = 0xffffffff;
                pc = 0x801348b4; continue;
                pc = 0x8013483c;
            }
            0x8013483c => {
                c.r[0] = c.r[0].wrapping_mul(0x24a4);
                c.r[31] = 0x0;
                c.r[29] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x801348a8; continue;
                pc = 0x8013484c;
            }
            0x8013484c => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134860; continue; }
                pc = 0x80134854;
            }
            0x80134854 => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80134868; continue; }
                pc = 0x80134860;
            }
            0x80134860 => {
                c.r[3] = 0x0;
                pc = 0x80134888; continue;
                pc = 0x80134868;
            }
            0x80134868 => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x8013487c; continue; }
                pc = 0x80134870;
            }
            0x80134870 => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x1e);
                if c.crbit(0) { pc = 0x80134884; continue; }
                pc = 0x8013487c;
            }
            0x8013487c => {
                c.r[3] = 0x0;
                pc = 0x80134888; continue;
                pc = 0x80134884;
            }
            0x80134884 => {
                c.r[3] = c.r[29].wrapping_add(0x14);
                pc = 0x80134888;
            }
            0x80134888 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801348a0; continue; }
                pc = 0x80134890;
            }
            0x80134890 => {
                c.lr = 0x80134894; f_80123fbc(c);
                pc = 0x80134894;
            }
            0x80134894 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x801348a0; continue; }
                pc = 0x8013489c;
            }
            0x8013489c => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x801348a0;
            }
            0x801348a0 => {
                c.r[29] = c.r[29].wrapping_add(0x138);
                c.r[31] = c.r[31].wrapping_add(0x1);
                pc = 0x801348a8;
            }
            0x801348a8 => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x1e);
                if c.crbit(0) { pc = 0x8013484c; continue; }
                pc = 0x801348b4;
            }
            0x801348b4 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x801348c4; continue; }
                pc = 0x801348bc;
            }
            0x801348bc => {
                c.r[3] = 0xffffffff;
                pc = 0x801348cc; continue;
                pc = 0x801348c4;
            }
            0x801348c4 => {
                c.r[0] = c.addc(!c.r[30], 0x1e, 1);
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                pc = 0x801348cc;
            }
            0x801348cc => {
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
            _ => panic!("f_801347e8: bad pc {:#x}", pc),
        }
    }
}

/// getPokemonBoxNbUsedSlot__5PCBOXFSc (0x801348ec, 240 bytes)
#[inline(never)]
pub fn f_801348ec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801348ec) { return; }
        if c.prof { crate::prof::enter(c, 0x801348ec); f_801348ec_b(c, 0x801348ec); crate::prof::exit(c, 0x801348ec); return; }
    }
    f_801348ec_b(c, 0x801348ec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801348ec_at(c: &mut Cpu, pc: u32) { f_801348ec_b(c, pc) }
#[inline(always)]
fn f_801348ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801348ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801348ec, pc); }
        match pc {
            0x801348ec => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x80134924; continue; }
                pc = 0x80134918;
            }
            0x80134918 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134924; f_80129280(c);
                pc = 0x80134924;
            }
            0x80134924 => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134938; continue; }
                pc = 0x8013492c;
            }
            0x8013492c => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80134940; continue; }
                pc = 0x80134938;
            }
            0x80134938 => {
                c.r[3] = 0xffffffff;
                pc = 0x801349bc; continue;
                pc = 0x80134940;
            }
            0x80134940 => {
                c.r[0] = c.r[0].wrapping_mul(0x24a4);
                c.r[29] = 0x0;
                c.r[31] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x801349ac; continue;
                pc = 0x80134950;
            }
            0x80134950 => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134964; continue; }
                pc = 0x80134958;
            }
            0x80134958 => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x8013496c; continue; }
                pc = 0x80134964;
            }
            0x80134964 => {
                c.r[3] = 0x0;
                pc = 0x8013498c; continue;
                pc = 0x8013496c;
            }
            0x8013496c => {
                c.r[0] = c.r[29] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134980; continue; }
                pc = 0x80134974;
            }
            0x80134974 => {
                c.r[0] = c.r[29] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x1e);
                if c.crbit(0) { pc = 0x80134988; continue; }
                pc = 0x80134980;
            }
            0x80134980 => {
                c.r[3] = 0x0;
                pc = 0x8013498c; continue;
                pc = 0x80134988;
            }
            0x80134988 => {
                c.r[3] = c.r[31].wrapping_add(0x14);
                pc = 0x8013498c;
            }
            0x8013498c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801349a4; continue; }
                pc = 0x80134994;
            }
            0x80134994 => {
                c.lr = 0x80134998; f_80123fbc(c);
                pc = 0x80134998;
            }
            0x80134998 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x801349a4; continue; }
                pc = 0x801349a0;
            }
            0x801349a0 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x801349a4;
            }
            0x801349a4 => {
                c.r[31] = c.r[31].wrapping_add(0x138);
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x801349ac;
            }
            0x801349ac => {
                c.r[0] = c.r[29] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x1e);
                if c.crbit(0) { pc = 0x80134950; continue; }
                pc = 0x801349b8;
            }
            0x801349b8 => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x801349bc;
            }
            0x801349bc => {
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
            _ => panic!("f_801348ec: bad pc {:#x}", pc),
        }
    }
}

/// pcboxSetPokemonBoxName (0x801349dc, 188 bytes)
#[inline(never)]
pub fn f_801349dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801349dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801349dc); f_801349dc_b(c, 0x801349dc); crate::prof::exit(c, 0x801349dc); return; }
    }
    f_801349dc_b(c, 0x801349dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801349dc_at(c: &mut Cpu, pc: u32) { f_801349dc_b(c, pc) }
#[inline(always)]
fn f_801349dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801349dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801349dc, pc); }
        match pc {
            0x801349dc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x80134a0c; continue; }
                pc = 0x80134a00;
            }
            0x80134a00 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134a0c; f_80129280(c);
                pc = 0x80134a0c;
            }
            0x80134a0c => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134a20; continue; }
                pc = 0x80134a14;
            }
            0x80134a14 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80134a28; continue; }
                pc = 0x80134a20;
            }
            0x80134a20 => {
                c.r[3] = 0x0;
                pc = 0x80134a80; continue;
                pc = 0x80134a28;
            }
            0x80134a28 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80134a38; continue; }
                pc = 0x80134a30;
            }
            0x80134a30 => {
                c.r[3] = 0x0;
                pc = 0x80134a80; continue;
                pc = 0x80134a38;
            }
            0x80134a38 => {
                c.r[5] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                pc = 0x80134a4c; continue;
                pc = 0x80134a44;
            }
            0x80134a44 => {
                c.r[5] = c.r[5].wrapping_add(0x2);
                c.r[4] = c.r[4].wrapping_add(0x1);
                pc = 0x80134a4c;
            }
            0x80134a4c => {
                c.r[0] = c.lhz(c.r[5]);
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80134a44; continue; }
                pc = 0x80134a58;
            }
            0x80134a58 => {
                c.cmps(0, c.r[4], 0x8);
                if !c.crbit(1) { pc = 0x80134a68; continue; }
                pc = 0x80134a60;
            }
            0x80134a60 => {
                c.r[3] = 0x0;
                pc = 0x80134a80; continue;
                pc = 0x80134a68;
            }
            0x80134a68 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.r[4] = c.r[31] | c.r[31];
                c.r[0] = c.r[0].wrapping_mul(0x24a4);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.lr = 0x80134a7c; f_800f9e70(c);
                pc = 0x80134a7c;
            }
            0x80134a7c => {
                c.r[3] = 0x1;
                pc = 0x80134a80;
            }
            0x80134a80 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801349dc: bad pc {:#x}", pc),
        }
    }
}

/// pcboxGetPokemonBoxName (0x80134a98, 96 bytes)
#[inline(never)]
pub fn f_80134a98(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134a98) { return; }
        if c.prof { crate::prof::enter(c, 0x80134a98); f_80134a98_b(c, 0x80134a98); crate::prof::exit(c, 0x80134a98); return; }
    }
    f_80134a98_b(c, 0x80134a98)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134a98_at(c: &mut Cpu, pc: u32) { f_80134a98_b(c, pc) }
#[inline(always)]
fn f_80134a98_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134a98;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134a98, pc); }
        match pc {
            0x80134a98 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x80134ac0; continue; }
                pc = 0x80134ab4;
            }
            0x80134ab4 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134ac0; f_80129280(c);
                pc = 0x80134ac0;
            }
            0x80134ac0 => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134ad4; continue; }
                pc = 0x80134ac8;
            }
            0x80134ac8 => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80134adc; continue; }
                pc = 0x80134ad4;
            }
            0x80134ad4 => {
                c.r[3] = 0x0;
                pc = 0x80134ae4; continue;
                pc = 0x80134adc;
            }
            0x80134adc => {
                c.r[0] = c.r[0].wrapping_mul(0x24a4);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x80134ae4;
            }
            0x80134ae4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80134a98: bad pc {:#x}", pc),
        }
    }
}

/// delPokemon__5PCBOXFScSc (0x80134af8, 200 bytes)
#[inline(never)]
pub fn f_80134af8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134af8) { return; }
        if c.prof { crate::prof::enter(c, 0x80134af8); f_80134af8_b(c, 0x80134af8); crate::prof::exit(c, 0x80134af8); return; }
    }
    f_80134af8_b(c, 0x80134af8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134af8_at(c: &mut Cpu, pc: u32) { f_80134af8_b(c, pc) }
#[inline(always)]
fn f_80134af8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134af8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134af8, pc); }
        match pc {
            0x80134af8 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x80134b28; continue; }
                pc = 0x80134b1c;
            }
            0x80134b1c => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134b28; f_80129280(c);
                pc = 0x80134b28;
            }
            0x80134b28 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134b3c; continue; }
                pc = 0x80134b30;
            }
            0x80134b30 => {
                c.r[4] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[4], 0x3);
                if c.crbit(0) { pc = 0x80134b44; continue; }
                pc = 0x80134b3c;
            }
            0x80134b3c => {
                c.r[31] = 0x0;
                pc = 0x80134b74; continue;
                pc = 0x80134b44;
            }
            0x80134b44 => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134b58; continue; }
                pc = 0x80134b4c;
            }
            0x80134b4c => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x1e);
                if c.crbit(0) { pc = 0x80134b60; continue; }
                pc = 0x80134b58;
            }
            0x80134b58 => {
                c.r[31] = 0x0;
                pc = 0x80134b74; continue;
                pc = 0x80134b60;
            }
            0x80134b60 => {
                c.r[4] = c.r[4].wrapping_mul(0x24a4);
                c.r[0] = c.r[0].wrapping_mul(0x138);
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[31] = c.r[4].wrapping_add(0x14);
                c.r[31] = c.r[3].wrapping_add(c.r[31]);
                pc = 0x80134b74;
            }
            0x80134b74 => {
                c.cmpu(0, c.r[31], 0x0);
                if !c.crbit(2) { pc = 0x80134b84; continue; }
                pc = 0x80134b7c;
            }
            0x80134b7c => {
                c.r[3] = 0x0;
                pc = 0x80134ba8; continue;
                pc = 0x80134b84;
            }
            0x80134b84 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80134b8c; f_80123fbc(c);
                pc = 0x80134b8c;
            }
            0x80134b8c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80134b9c; continue; }
                pc = 0x80134b94;
            }
            0x80134b94 => {
                c.r[3] = 0x0;
                pc = 0x80134ba8; continue;
                pc = 0x80134b9c;
            }
            0x80134b9c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80134ba4; f_80124a60(c);
                pc = 0x80134ba4;
            }
            0x80134ba4 => {
                c.r[3] = 0x1;
                pc = 0x80134ba8;
            }
            0x80134ba8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80134af8: bad pc {:#x}", pc),
        }
    }
}

/// pcboxAddPokemon (0x80134bc0, 592 bytes)
#[inline(never)]
pub fn f_80134bc0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134bc0) { return; }
        if c.prof { crate::prof::enter(c, 0x80134bc0); f_80134bc0_b(c, 0x80134bc0); crate::prof::exit(c, 0x80134bc0); return; }
    }
    f_80134bc0_b(c, 0x80134bc0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134bc0_at(c: &mut Cpu, pc: u32) { f_80134bc0_b(c, pc) }
#[inline(always)]
fn f_80134bc0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134bc0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134bc0, pc); }
        match pc {
            0x80134bc0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[30] = c.r[5] | c.r[5];
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x80134bf4; continue; }
                pc = 0x80134be4;
            }
            0x80134be4 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134bf0; f_80129280(c);
                pc = 0x80134bf0;
            }
            0x80134bf0 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80134bf4;
            }
            0x80134bf4 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0xffffffff);
                if c.crbit(0) { pc = 0x80134c08; continue; }
                pc = 0x80134c00;
            }
            0x80134c00 => {
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80134c10; continue; }
                pc = 0x80134c08;
            }
            0x80134c08 => {
                c.r[3] = 0x0;
                pc = 0x80134dfc; continue;
                pc = 0x80134c10;
            }
            0x80134c10 => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x80134c20; continue; }
                pc = 0x80134c18;
            }
            0x80134c18 => {
                c.r[3] = 0x0;
                pc = 0x80134dfc; continue;
                pc = 0x80134c20;
            }
            0x80134c20 => {
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(2) { pc = 0x80134ce0; continue; }
                pc = 0x80134c28;
            }
            0x80134c28 => {
                c.r[27] = c.r[31] | c.r[31];
                c.r[30] = 0x0;
                pc = 0x80134cc0; continue;
                pc = 0x80134c34;
            }
            0x80134c34 => {
                c.r[28] = 0x0;
                pc = 0x80134c38;
            }
            0x80134c38 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                c.r[3] = c.r[28] as u8 as i8 as i32 as u32;
                if c.crbit(0) { pc = 0x80134c50; continue; }
                pc = 0x80134c44;
            }
            0x80134c44 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80134c58; continue; }
                pc = 0x80134c50;
            }
            0x80134c50 => {
                c.r[3] = 0x0;
                pc = 0x80134c7c; continue;
                pc = 0x80134c58;
            }
            0x80134c58 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134c68; continue; }
                pc = 0x80134c60;
            }
            0x80134c60 => {
                c.cmps(0, c.r[3], 0x1e);
                if c.crbit(0) { pc = 0x80134c70; continue; }
                pc = 0x80134c68;
            }
            0x80134c68 => {
                c.r[3] = 0x0;
                pc = 0x80134c7c; continue;
                pc = 0x80134c70;
            }
            0x80134c70 => {
                c.r[3] = c.r[3].wrapping_mul(0x138);
                c.r[3] = c.r[3].wrapping_add(0x14);
                c.r[3] = c.r[27].wrapping_add(c.r[3]);
                pc = 0x80134c7c;
            }
            0x80134c7c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80134c90; continue; }
                pc = 0x80134c84;
            }
            0x80134c84 => {
                c.lr = 0x80134c88; f_80123fbc(c);
                pc = 0x80134c88;
            }
            0x80134c88 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80134c9c; continue; }
                pc = 0x80134c90;
            }
            0x80134c90 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.cmps(0, c.r[28], 0x1e);
                if c.crbit(0) { pc = 0x80134c38; continue; }
                pc = 0x80134c9c;
            }
            0x80134c9c => {
                c.cmps(0, c.r[28], 0x1e);
                c.r[3] = 0xffffffff;
                if !c.crbit(0) { pc = 0x80134cac; continue; }
                pc = 0x80134ca8;
            }
            0x80134ca8 => {
                c.r[3] = c.r[28] as u8 as i8 as i32 as u32;
                pc = 0x80134cac;
            }
            0x80134cac => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                c.r[28] = c.r[3] | c.r[3];
                if !c.crbit(0) { pc = 0x80134ccc; continue; }
                pc = 0x80134cb8;
            }
            0x80134cb8 => {
                c.r[27] = c.r[27].wrapping_add(0x24a4);
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x80134cc0;
            }
            0x80134cc0 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80134c34; continue; }
                pc = 0x80134ccc;
            }
            0x80134ccc => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80134d74; continue; }
                pc = 0x80134cd8;
            }
            0x80134cd8 => {
                c.r[3] = 0x0;
                pc = 0x80134dfc; continue;
                pc = 0x80134ce0;
            }
            0x80134ce0 => {
                c.r[0] = c.r[0].wrapping_mul(0x24a4);
                c.r[28] = 0x0;
                c.r[27] = c.r[31].wrapping_add(c.r[0]);
                pc = 0x80134cec;
            }
            0x80134cec => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                c.r[3] = c.r[28] as u8 as i8 as i32 as u32;
                if c.crbit(0) { pc = 0x80134d04; continue; }
                pc = 0x80134cf8;
            }
            0x80134cf8 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x80134d0c; continue; }
                pc = 0x80134d04;
            }
            0x80134d04 => {
                c.r[3] = 0x0;
                pc = 0x80134d30; continue;
                pc = 0x80134d0c;
            }
            0x80134d0c => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134d1c; continue; }
                pc = 0x80134d14;
            }
            0x80134d14 => {
                c.cmps(0, c.r[3], 0x1e);
                if c.crbit(0) { pc = 0x80134d24; continue; }
                pc = 0x80134d1c;
            }
            0x80134d1c => {
                c.r[3] = 0x0;
                pc = 0x80134d30; continue;
                pc = 0x80134d24;
            }
            0x80134d24 => {
                c.r[3] = c.r[3].wrapping_mul(0x138);
                c.r[3] = c.r[3].wrapping_add(0x14);
                c.r[3] = c.r[27].wrapping_add(c.r[3]);
                pc = 0x80134d30;
            }
            0x80134d30 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80134d44; continue; }
                pc = 0x80134d38;
            }
            0x80134d38 => {
                c.lr = 0x80134d3c; f_80123fbc(c);
                pc = 0x80134d3c;
            }
            0x80134d3c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80134d50; continue; }
                pc = 0x80134d44;
            }
            0x80134d44 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.cmps(0, c.r[28], 0x1e);
                if c.crbit(0) { pc = 0x80134cec; continue; }
                pc = 0x80134d50;
            }
            0x80134d50 => {
                c.cmps(0, c.r[28], 0x1e);
                c.r[3] = 0xffffffff;
                if !c.crbit(0) { pc = 0x80134d60; continue; }
                pc = 0x80134d5c;
            }
            0x80134d5c => {
                c.r[3] = c.r[28] as u8 as i8 as i32 as u32;
                pc = 0x80134d60;
            }
            0x80134d60 => {
                c.r[0] = c.r[3] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                c.r[28] = c.r[3] | c.r[3];
                if !c.crbit(0) { pc = 0x80134d74; continue; }
                pc = 0x80134d6c;
            }
            0x80134d6c => {
                c.r[3] = 0x0;
                pc = 0x80134dfc; continue;
                pc = 0x80134d74;
            }
            0x80134d74 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134d88; continue; }
                pc = 0x80134d7c;
            }
            0x80134d7c => {
                c.r[3] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[3], 0x3);
                if c.crbit(0) { pc = 0x80134d90; continue; }
                pc = 0x80134d88;
            }
            0x80134d88 => {
                c.r[3] = 0x0;
                pc = 0x80134dc0; continue;
                pc = 0x80134d90;
            }
            0x80134d90 => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134da4; continue; }
                pc = 0x80134d98;
            }
            0x80134d98 => {
                c.r[0] = c.r[28] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x1e);
                if c.crbit(0) { pc = 0x80134dac; continue; }
                pc = 0x80134da4;
            }
            0x80134da4 => {
                c.r[3] = 0x0;
                pc = 0x80134dc0; continue;
                pc = 0x80134dac;
            }
            0x80134dac => {
                c.r[3] = c.r[3].wrapping_mul(0x24a4);
                c.r[0] = c.r[0].wrapping_mul(0x138);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.r[3].wrapping_add(0x14);
                c.r[3] = c.r[31].wrapping_add(c.r[3]);
                pc = 0x80134dc0;
            }
            0x80134dc0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80134dd0; continue; }
                pc = 0x80134dc8;
            }
            0x80134dc8 => {
                c.r[3] = 0x0;
                pc = 0x80134dfc; continue;
                pc = 0x80134dd0;
            }
            0x80134dd0 => {
                c.r[0] = 0x27;
                c.r[6] = c.r[3].wrapping_add(0xfffffffc);
                c.r[5] = c.r[29].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x80134de0;
            }
            0x80134de0 => {
                c.r[4] = c.lwz(c.r[5].wrapping_add(0x4));
                { let ea = c.r[5].wrapping_add(0x8); c.r[0] = c.lwz(ea); c.r[5] = ea; }
                c.stw(c.r[6].wrapping_add(0x4), c.r[4]);
                { let ea = c.r[6].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[6] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80134de0; continue; }
                pc = 0x80134df4;
            }
            0x80134df4 => {
                c.lr = 0x80134df8; f_8012086c(c);
                pc = 0x80134df8;
            }
            0x80134df8 => {
                c.r[3] = 0x1;
                pc = 0x80134dfc;
            }
            0x80134dfc => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80134bc0: bad pc {:#x}", pc),
        }
    }
}

/// setPokemon__5PCBOXFP7PokemonScSc (0x80134e10, 224 bytes)
#[inline(never)]
pub fn f_80134e10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134e10) { return; }
        if c.prof { crate::prof::enter(c, 0x80134e10); f_80134e10_b(c, 0x80134e10); crate::prof::exit(c, 0x80134e10); return; }
    }
    f_80134e10_b(c, 0x80134e10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134e10_at(c: &mut Cpu, pc: u32) { f_80134e10_b(c, pc) }
#[inline(always)]
fn f_80134e10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134e10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134e10, pc); }
        match pc {
            0x80134e10 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[6] | c.r[6];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x80134e48; continue; }
                pc = 0x80134e3c;
            }
            0x80134e3c => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134e48; f_80129280(c);
                pc = 0x80134e48;
            }
            0x80134e48 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134e5c; continue; }
                pc = 0x80134e50;
            }
            0x80134e50 => {
                c.r[4] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[4], 0x3);
                if c.crbit(0) { pc = 0x80134e64; continue; }
                pc = 0x80134e5c;
            }
            0x80134e5c => {
                c.r[6] = 0x0;
                pc = 0x80134e94; continue;
                pc = 0x80134e64;
            }
            0x80134e64 => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134e78; continue; }
                pc = 0x80134e6c;
            }
            0x80134e6c => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x1e);
                if c.crbit(0) { pc = 0x80134e80; continue; }
                pc = 0x80134e78;
            }
            0x80134e78 => {
                c.r[6] = 0x0;
                pc = 0x80134e94; continue;
                pc = 0x80134e80;
            }
            0x80134e80 => {
                c.r[4] = c.r[4].wrapping_mul(0x24a4);
                c.r[0] = c.r[0].wrapping_mul(0x138);
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[6] = c.r[4].wrapping_add(0x14);
                c.r[6] = c.r[3].wrapping_add(c.r[6]);
                pc = 0x80134e94;
            }
            0x80134e94 => {
                c.cmpu(0, c.r[6], 0x0);
                if !c.crbit(2) { pc = 0x80134ea4; continue; }
                pc = 0x80134e9c;
            }
            0x80134e9c => {
                c.r[3] = 0x0;
                pc = 0x80134ed4; continue;
                pc = 0x80134ea4;
            }
            0x80134ea4 => {
                c.r[0] = 0x27;
                c.r[5] = c.r[6].wrapping_add(0xfffffffc);
                c.r[4] = c.r[29].wrapping_add(0xfffffffc);
                c.mtspr(9, c.r[0]);
                pc = 0x80134eb4;
            }
            0x80134eb4 => {
                c.r[3] = c.lwz(c.r[4].wrapping_add(0x4));
                { let ea = c.r[4].wrapping_add(0x8); c.r[0] = c.lwz(ea); c.r[4] = ea; }
                c.stw(c.r[5].wrapping_add(0x4), c.r[3]);
                { let ea = c.r[5].wrapping_add(0x8); c.stw(ea, c.r[0]); c.r[5] = ea; }
                c.ctr = c.ctr.wrapping_sub(1); if c.ctr != 0 { pc = 0x80134eb4; continue; }
                pc = 0x80134ec8;
            }
            0x80134ec8 => {
                c.r[3] = c.r[6] | c.r[6];
                c.lr = 0x80134ed0; f_8012086c(c);
                pc = 0x80134ed0;
            }
            0x80134ed0 => {
                c.r[3] = 0x1;
                pc = 0x80134ed4;
            }
            0x80134ed4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80134e10: bad pc {:#x}", pc),
        }
    }
}

/// getPokemon__5PCBOXFScSc (0x80134ef0, 152 bytes)
#[inline(never)]
pub fn f_80134ef0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134ef0) { return; }
        if c.prof { crate::prof::enter(c, 0x80134ef0); f_80134ef0_b(c, 0x80134ef0); crate::prof::exit(c, 0x80134ef0); return; }
    }
    f_80134ef0_b(c, 0x80134ef0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134ef0_at(c: &mut Cpu, pc: u32) { f_80134ef0_b(c, pc) }
#[inline(always)]
fn f_80134ef0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134ef0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134ef0, pc); }
        match pc {
            0x80134ef0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                if !c.crbit(2) { pc = 0x80134f20; continue; }
                pc = 0x80134f14;
            }
            0x80134f14 => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134f20; f_80129280(c);
                pc = 0x80134f20;
            }
            0x80134f20 => {
                c.r[0] = c.r[30] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134f34; continue; }
                pc = 0x80134f28;
            }
            0x80134f28 => {
                c.r[4] = c.r[30] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[4], 0x3);
                if c.crbit(0) { pc = 0x80134f3c; continue; }
                pc = 0x80134f34;
            }
            0x80134f34 => {
                c.r[0] = 0x0;
                pc = 0x80134f6c; continue;
                pc = 0x80134f3c;
            }
            0x80134f3c => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80134f50; continue; }
                pc = 0x80134f44;
            }
            0x80134f44 => {
                c.r[0] = c.r[31] as u8 as i8 as i32 as u32;
                c.cmps(0, c.r[0], 0x1e);
                if c.crbit(0) { pc = 0x80134f58; continue; }
                pc = 0x80134f50;
            }
            0x80134f50 => {
                c.r[0] = 0x0;
                pc = 0x80134f6c; continue;
                pc = 0x80134f58;
            }
            0x80134f58 => {
                c.r[4] = c.r[4].wrapping_mul(0x24a4);
                c.r[0] = c.r[0].wrapping_mul(0x138);
                c.r[4] = c.r[4].wrapping_add(c.r[0]);
                c.r[0] = c.r[4].wrapping_add(0x14);
                c.r[0] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x80134f6c;
            }
            0x80134f6c => {
                c.r[3] = c.r[0] | c.r[0];
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80134ef0: bad pc {:#x}", pc),
        }
    }
}

/// pcboxInit (0x80134f88, 156 bytes)
#[inline(never)]
pub fn f_80134f88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80134f88) { return; }
        if c.prof { crate::prof::enter(c, 0x80134f88); f_80134f88_b(c, 0x80134f88); crate::prof::exit(c, 0x80134f88); return; }
    }
    f_80134f88_b(c, 0x80134f88)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80134f88_at(c: &mut Cpu, pc: u32) { f_80134f88_b(c, pc) }
#[inline(always)]
fn f_80134f88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80134f88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80134f88, pc); }
        match pc {
            0x80134f88 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                if !c.crbit(2) { pc = 0x80134fbc; continue; }
                pc = 0x80134fac;
            }
            0x80134fac => {
                c.r[3] = 0x0;
                c.r[4] = 0x3;
                c.lr = 0x80134fb8; f_80129280(c);
                pc = 0x80134fb8;
            }
            0x80134fb8 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80134fbc;
            }
            0x80134fbc => {
                c.r[30] = 0x0;
                c.r[29] = c.r[31] | c.r[31];
                pc = 0x80134fc4;
            }
            0x80134fc4 => {
                c.r[4] = c.r[30].wrapping_add(0x1);
                c.r[3] = 0x34;
                c.lr = 0x80134fd0; f_80132a38(c);
                pc = 0x80134fd0;
            }
            0x80134fd0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = 0x9;
                c.r[5] = 0x32c9;
                c.lr = 0x80134fe0; f_800f96e4(c);
                pc = 0x80134fe0;
            }
            0x80134fe0 => {
                c.r[3] = c.r[29].wrapping_add(0x14);
                c.r[4] = 0x1e;
                c.lr = 0x80134fec; f_801249f8(c);
                pc = 0x80134fec;
            }
            0x80134fec => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[29] = c.r[29].wrapping_add(0x24a4);
                c.cmps(0, c.r[30], 0x3);
                if c.crbit(0) { pc = 0x80134fc4; continue; }
                pc = 0x80134ffc;
            }
            0x80134ffc => {
                c.r[3] = c.r[31].wrapping_add(0x6dec);
                c.r[4] = 0xeb;
                c.lr = 0x80135008; f_80142a88(c);
                pc = 0x80135008;
            }
            0x80135008 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80134f88: bad pc {:#x}", pc),
        }
    }
}

/// pcboxSetStatus (0x80135024, 4 bytes)
#[inline(never)]
pub fn f_80135024(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135024) { return; }
        if c.prof { crate::prof::enter(c, 0x80135024); f_80135024_b(c, 0x80135024); crate::prof::exit(c, 0x80135024); return; }
    }
    f_80135024_b(c, 0x80135024)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135024_at(c: &mut Cpu, pc: u32) { f_80135024_b(c, pc) }
#[inline(always)]
fn f_80135024_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135024;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135024, pc); }
        match pc {
            0x80135024 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80135024: bad pc {:#x}", pc),
        }
    }
}

/// pcboxGetStatus (0x80135028, 8 bytes)
#[inline(never)]
pub fn f_80135028(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135028) { return; }
        if c.prof { crate::prof::enter(c, 0x80135028); f_80135028_b(c, 0x80135028); crate::prof::exit(c, 0x80135028); return; }
    }
    f_80135028_b(c, 0x80135028)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135028_at(c: &mut Cpu, pc: u32) { f_80135028_b(c, pc) }
#[inline(always)]
fn f_80135028_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135028;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135028, pc); }
        match pc {
            0x80135028 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80135028: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveSetStatus (0x80135030, 312 bytes)
#[inline(never)]
pub fn f_80135030(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135030) { return; }
        if c.prof { crate::prof::enter(c, 0x80135030); f_80135030_b(c, 0x80135030); crate::prof::exit(c, 0x80135030); return; }
    }
    f_80135030_b(c, 0x80135030)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135030_at(c: &mut Cpu, pc: u32) { f_80135030_b(c, pc) }
#[inline(always)]
fn f_80135030_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135030;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135030, pc); }
        match pc {
            0x80135030 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x80135034;
            }
            0x80135034 => {
                c.r[0] = c.mfspr(8);
                pc = 0x80135038;
            }
            0x80135038 => {
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x8013503c;
            }
            0x8013503c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                pc = 0x80135040;
            }
            0x80135040 => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                pc = 0x80135044;
            }
            0x80135044 => {
                c.r[31] = c.r[5] | c.r[5];
                pc = 0x80135048;
            }
            0x80135048 => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                pc = 0x8013504c;
            }
            0x8013504c => {
                c.r[30] = c.r[4] | c.r[4];
                pc = 0x80135050;
            }
            0x80135050 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                pc = 0x80135054;
            }
            0x80135054 => {
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x80135058;
            }
            0x80135058 => {
                if c.crbit(2) { pc = 0x8013514c; continue; }
                pc = 0x8013505c;
            }
            0x8013505c => {
                c.cmpu(0, c.r[0], 0xb);
                pc = 0x80135060;
            }
            0x80135060 => {
                if c.crbit(0) { pc = 0x80135068; continue; }
                pc = 0x80135064;
            }
            0x80135064 => {
                pc = 0x8013514c; continue;
                pc = 0x80135068;
            }
            0x80135068 => {
                c.cmpu(0, c.r[29], 0x0);
                pc = 0x8013506c;
            }
            0x8013506c => {
                if !c.crbit(2) { pc = 0x80135094; continue; }
                pc = 0x80135070;
            }
            0x80135070 => {
                c.r[3] = 0x0;
                pc = 0x80135074;
            }
            0x80135074 => {
                c.r[4] = 0x0;
                pc = 0x80135078;
            }
            0x80135078 => {
                c.lr = 0x8013507c; f_80129280(c);
                pc = 0x8013507c;
            }
            0x8013507c => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x80135080;
            }
            0x80135080 => {
                if c.crbit(2) { pc = 0x8013514c; continue; }
                pc = 0x80135084;
            }
            0x80135084 => {
                c.r[4] = 0x1;
                pc = 0x80135088;
            }
            0x80135088 => {
                c.lr = 0x8013508c; f_80129280(c);
                pc = 0x8013508c;
            }
            0x8013508c => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                pc = 0x80135090;
            }
            0x80135090 => {
                if c.crbit(2) { pc = 0x8013514c; continue; }
                pc = 0x80135094;
            }
            0x80135094 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x80135098;
            }
            0x80135098 => {
                c.lr = 0x8013509c; f_80135cd0(c);
                pc = 0x8013509c;
            }
            0x8013509c => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x801350a0;
            }
            0x801350a0 => {
                if c.crbit(2) { pc = 0x8013514c; continue; }
                pc = 0x801350a4;
            }
            0x801350a4 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                pc = 0x801350a8;
            }
            0x801350a8 => {
                c.cmpu(0, c.r[0], 0xa);
                pc = 0x801350ac;
            }
            0x801350ac => {
                if c.crbit(1) { pc = 0x8013514c; continue; }
                pc = 0x801350b0;
            }
            0x801350b0 => {
                c.r[4] = 0x80360000;
                pc = 0x801350b4;
            }
            0x801350b4 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x801350b8;
            }
            0x801350b8 => {
                c.r[4] = c.r[4].wrapping_add(0x3a70);
                pc = 0x801350bc;
            }
            0x801350bc => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                pc = 0x801350c0;
            }
            0x801350c0 => {
                c.mtspr(9, c.r[0]);
                pc = 0x801350c4;
            }
            0x801350c4 => {
                if true { let t = c.ctr; if (0x80135030..0x80135168).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x801350c8;
            }
            0x801350c8 => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x801350cc;
            }
            0x801350cc => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801350d0;
            }
            0x801350d0 => {
                c.lr = 0x801350d4; f_80135c90(c);
                pc = 0x801350d4;
            }
            0x801350d4 => {
                pc = 0x8013514c; continue;
                pc = 0x801350d8;
            }
            0x801350d8 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801350dc;
            }
            0x801350dc => {
                c.lr = 0x801350e0; f_80135ba0(c);
                pc = 0x801350e0;
            }
            0x801350e0 => {
                pc = 0x8013514c; continue;
                pc = 0x801350e4;
            }
            0x801350e4 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801350e8;
            }
            0x801350e8 => {
                c.lr = 0x801350ec; f_80135b7c(c);
                pc = 0x801350ec;
            }
            0x801350ec => {
                pc = 0x8013514c; continue;
                pc = 0x801350f0;
            }
            0x801350f0 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801350f4;
            }
            0x801350f4 => {
                c.lr = 0x801350f8; f_80135b6c(c);
                pc = 0x801350f8;
            }
            0x801350f8 => {
                pc = 0x8013514c; continue;
                pc = 0x801350fc;
            }
            0x801350fc => {
                c.r[4] = c.r[31] ^ 0x80000000;
                pc = 0x80135100;
            }
            0x80135100 => {
                c.r[0] = 0x43300000;
                pc = 0x80135104;
            }
            0x80135104 => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[4]);
                pc = 0x80135108;
            }
            0x80135108 => {
                { let ea = c.r[2].wrapping_add(0xffff9a68); c.f[1] = c.lfd(ea); }
                pc = 0x8013510c;
            }
            0x8013510c => {
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x80135110;
            }
            0x80135110 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                pc = 0x80135114;
            }
            0x80135114 => {
                { let v = s(c.f[0] - c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x80135118;
            }
            0x80135118 => {
                c.lr = 0x8013511c; f_80135b5c(c);
                pc = 0x8013511c;
            }
            0x8013511c => {
                pc = 0x8013514c; continue;
                pc = 0x80135120;
            }
            0x80135120 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x80135124;
            }
            0x80135124 => {
                c.lr = 0x80135128; f_80135b4c(c);
                pc = 0x80135128;
            }
            0x80135128 => {
                pc = 0x8013514c; continue;
                pc = 0x8013512c;
            }
            0x8013512c => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x80135130;
            }
            0x80135130 => {
                c.lr = 0x80135134; f_80135b3c(c);
                pc = 0x80135134;
            }
            0x80135134 => {
                pc = 0x8013514c; continue;
                pc = 0x80135138;
            }
            0x80135138 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x8013513c;
            }
            0x8013513c => {
                c.lr = 0x80135140; f_80135b2c(c);
                pc = 0x80135140;
            }
            0x80135140 => {
                pc = 0x8013514c; continue;
                pc = 0x80135144;
            }
            0x80135144 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                pc = 0x80135148;
            }
            0x80135148 => {
                c.lr = 0x8013514c; f_80135b1c(c);
                pc = 0x8013514c;
            }
            0x8013514c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x80135150;
            }
            0x80135150 => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                pc = 0x80135154;
            }
            0x80135154 => {
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                pc = 0x80135158;
            }
            0x80135158 => {
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x8013515c;
            }
            0x8013515c => {
                c.mtspr(8, c.r[0]);
                pc = 0x80135160;
            }
            0x80135160 => {
                c.r[1] = c.r[1].wrapping_add(0x20);
                pc = 0x80135164;
            }
            0x80135164 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80135030: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveGetStatus (0x80135168, 292 bytes)
#[inline(never)]
pub fn f_80135168(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135168) { return; }
        if c.prof { crate::prof::enter(c, 0x80135168); f_80135168_b(c, 0x80135168); crate::prof::exit(c, 0x80135168); return; }
    }
    f_80135168_b(c, 0x80135168)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135168_at(c: &mut Cpu, pc: u32) { f_80135168_b(c, pc) }
#[inline(always)]
fn f_80135168_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135168;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135168, pc); }
        match pc {
            0x80135168 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x8013516c;
            }
            0x8013516c => {
                c.r[0] = c.mfspr(8);
                pc = 0x80135170;
            }
            0x80135170 => {
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                pc = 0x80135174;
            }
            0x80135174 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                pc = 0x80135178;
            }
            0x80135178 => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                pc = 0x8013517c;
            }
            0x8013517c => {
                c.r[31] = 0x0;
                pc = 0x80135180;
            }
            0x80135180 => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                pc = 0x80135184;
            }
            0x80135184 => {
                c.r[30] = c.r[4] | c.r[4];
                pc = 0x80135188;
            }
            0x80135188 => {
                if c.crbit(2) { pc = 0x80135194; continue; }
                pc = 0x8013518c;
            }
            0x8013518c => {
                c.cmpu(0, c.r[0], 0xb);
                pc = 0x80135190;
            }
            0x80135190 => {
                if c.crbit(0) { pc = 0x8013519c; continue; }
                pc = 0x80135194;
            }
            0x80135194 => {
                c.r[3] = 0x0;
                pc = 0x80135198;
            }
            0x80135198 => {
                pc = 0x80135274; continue;
                pc = 0x8013519c;
            }
            0x8013519c => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x801351a0;
            }
            0x801351a0 => {
                if !c.crbit(2) { pc = 0x801351d8; continue; }
                pc = 0x801351a4;
            }
            0x801351a4 => {
                c.r[3] = 0x0;
                pc = 0x801351a8;
            }
            0x801351a8 => {
                c.r[4] = 0x0;
                pc = 0x801351ac;
            }
            0x801351ac => {
                c.lr = 0x801351b0; f_80129280(c);
                pc = 0x801351b0;
            }
            0x801351b0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                pc = 0x801351b4;
            }
            0x801351b4 => {
                if !c.crbit(2) { pc = 0x801351c0; continue; }
                pc = 0x801351b8;
            }
            0x801351b8 => {
                c.r[3] = 0x0;
                pc = 0x801351bc;
            }
            0x801351bc => {
                pc = 0x80135274; continue;
                pc = 0x801351c0;
            }
            0x801351c0 => {
                c.r[4] = 0x1;
                pc = 0x801351c4;
            }
            0x801351c4 => {
                c.lr = 0x801351c8; f_80129280(c);
                pc = 0x801351c8;
            }
            0x801351c8 => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x801351cc;
            }
            0x801351cc => {
                if !c.crbit(2) { pc = 0x801351d8; continue; }
                pc = 0x801351d0;
            }
            0x801351d0 => {
                c.r[3] = 0x0;
                pc = 0x801351d4;
            }
            0x801351d4 => {
                pc = 0x80135274; continue;
                pc = 0x801351d8;
            }
            0x801351d8 => {
                c.lr = 0x801351dc; f_80135cd0(c);
                pc = 0x801351dc;
            }
            0x801351dc => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x801351e0;
            }
            0x801351e0 => {
                if !c.crbit(2) { pc = 0x801351ec; continue; }
                pc = 0x801351e4;
            }
            0x801351e4 => {
                c.r[3] = 0x0;
                pc = 0x801351e8;
            }
            0x801351e8 => {
                pc = 0x80135274; continue;
                pc = 0x801351ec;
            }
            0x801351ec => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                pc = 0x801351f0;
            }
            0x801351f0 => {
                c.cmpu(0, c.r[0], 0xa);
                pc = 0x801351f4;
            }
            0x801351f4 => {
                if c.crbit(1) { pc = 0x80135270; continue; }
                pc = 0x801351f8;
            }
            0x801351f8 => {
                c.r[4] = 0x80360000;
                pc = 0x801351fc;
            }
            0x801351fc => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80135200;
            }
            0x80135200 => {
                c.r[4] = c.r[4].wrapping_add(0x3a9c);
                pc = 0x80135204;
            }
            0x80135204 => {
                c.r[0] = c.lwz(c.r[4].wrapping_add(c.r[0]));
                pc = 0x80135208;
            }
            0x80135208 => {
                c.mtspr(9, c.r[0]);
                pc = 0x8013520c;
            }
            0x8013520c => {
                if true { let t = c.ctr; if (0x80135168..0x8013528c).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80135210;
            }
            0x80135210 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80135214;
            }
            0x80135214 => {
                pc = 0x80135274; continue;
                pc = 0x80135218;
            }
            0x80135218 => {
                c.lr = 0x8013521c; f_80135c78(c);
                pc = 0x8013521c;
            }
            0x8013521c => {
                pc = 0x80135274; continue;
                pc = 0x80135220;
            }
            0x80135220 => {
                c.lr = 0x80135224; f_80135c40(c);
                pc = 0x80135224;
            }
            0x80135224 => {
                pc = 0x80135274; continue;
                pc = 0x80135228;
            }
            0x80135228 => {
                c.lr = 0x8013522c; f_80135c28(c);
                pc = 0x8013522c;
            }
            0x8013522c => {
                pc = 0x80135274; continue;
                pc = 0x80135230;
            }
            0x80135230 => {
                c.lr = 0x80135234; f_80135c10(c);
                pc = 0x80135234;
            }
            0x80135234 => {
                c.f[0] = fctiwz(c.f[1]);
                pc = 0x80135238;
            }
            0x80135238 => {
                { let ea = c.r[1].wrapping_add(0x8); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x8013523c;
            }
            0x8013523c => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x80135240;
            }
            0x80135240 => {
                pc = 0x80135274; continue;
                pc = 0x80135244;
            }
            0x80135244 => {
                c.lr = 0x80135248; f_80135bf8(c);
                pc = 0x80135248;
            }
            0x80135248 => {
                pc = 0x80135274; continue;
                pc = 0x8013524c;
            }
            0x8013524c => {
                c.lr = 0x80135250; f_80135be0(c);
                pc = 0x80135250;
            }
            0x80135250 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80135254;
            }
            0x80135254 => {
                pc = 0x80135274; continue;
                pc = 0x80135258;
            }
            0x80135258 => {
                c.lr = 0x8013525c; f_80135bc8(c);
                pc = 0x8013525c;
            }
            0x8013525c => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80135260;
            }
            0x80135260 => {
                pc = 0x80135274; continue;
                pc = 0x80135264;
            }
            0x80135264 => {
                c.lr = 0x80135268; f_80135bb0(c);
                pc = 0x80135268;
            }
            0x80135268 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8013526c;
            }
            0x8013526c => {
                pc = 0x80135274; continue;
                pc = 0x80135270;
            }
            0x80135270 => {
                c.r[3] = 0x0;
                pc = 0x80135274;
            }
            0x80135274 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x80135278;
            }
            0x80135278 => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                pc = 0x8013527c;
            }
            0x8013527c => {
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                pc = 0x80135280;
            }
            0x80135280 => {
                c.mtspr(8, c.r[0]);
                pc = 0x80135284;
            }
            0x80135284 => {
                c.r[1] = c.r[1].wrapping_add(0x20);
                pc = 0x80135288;
            }
            0x80135288 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80135168: bad pc {:#x}", pc),
        }
    }
}

/// gamedataCreate (0x8013528c, 172 bytes)
#[inline(never)]
pub fn f_8013528c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013528c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013528c); f_8013528c_b(c, 0x8013528c); crate::prof::exit(c, 0x8013528c); return; }
    }
    f_8013528c_b(c, 0x8013528c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013528c_at(c: &mut Cpu, pc: u32) { f_8013528c_b(c, pc) }
#[inline(always)]
fn f_8013528c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013528c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013528c, pc); }
        match pc {
            0x8013528c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = c.r[7] | c.r[7];
                if c.crbit(2) { pc = 0x80135324; continue; }
                pc = 0x801352b4;
            }
            0x801352b4 => {
                c.lr = 0x801352b8; f_80135338(c);
                pc = 0x801352b8;
            }
            0x801352b8 => {
                c.cmpu(0, c.r[27], 0x0);
                c.r[3] = c.r[27] | c.r[27];
                if !c.crbit(2) { pc = 0x801352f8; continue; }
                pc = 0x801352c4;
            }
            0x801352c4 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x801352d0; f_80129280(c);
                pc = 0x801352d0;
            }
            0x801352d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801352e0; continue; }
                pc = 0x801352d8;
            }
            0x801352d8 => {
                c.r[3] = 0x0;
                pc = 0x80135308; continue;
                pc = 0x801352e0;
            }
            0x801352e0 => {
                c.r[4] = 0x1;
                c.lr = 0x801352e8; f_80129280(c);
                pc = 0x801352e8;
            }
            0x801352e8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801352f8; continue; }
                pc = 0x801352f0;
            }
            0x801352f0 => {
                c.r[3] = 0x0;
                pc = 0x80135308; continue;
                pc = 0x801352f8;
            }
            0x801352f8 => {
                c.lr = 0x801352fc; f_80135b0c(c);
                pc = 0x801352fc;
            }
            0x801352fc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135308; continue; }
                pc = 0x80135304;
            }
            0x80135304 => {
                c.r[3] = 0x0;
                pc = 0x80135308;
            }
            0x80135308 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135324; continue; }
                pc = 0x80135310;
            }
            0x80135310 => {
                c.r[4] = c.r[28] | c.r[28];
                c.r[5] = c.r[29] | c.r[29];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = c.r[31] | c.r[31];
                c.lr = 0x80135324; f_801353c0(c);
                pc = 0x80135324;
            }
            0x80135324 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8013528c: bad pc {:#x}", pc),
        }
    }
}

/// gamedataInit (0x80135338, 136 bytes)
#[inline(never)]
pub fn f_80135338(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135338) { return; }
        if c.prof { crate::prof::enter(c, 0x80135338); f_80135338_b(c, 0x80135338); crate::prof::exit(c, 0x80135338); return; }
    }
    f_80135338_b(c, 0x80135338)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135338_at(c: &mut Cpu, pc: u32) { f_80135338_b(c, pc) }
#[inline(always)]
fn f_80135338_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135338;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135338, pc); }
        match pc {
            0x80135338 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                if !c.crbit(2) { pc = 0x80135388; continue; }
                pc = 0x80135354;
            }
            0x80135354 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80135360; f_80129280(c);
                pc = 0x80135360;
            }
            0x80135360 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135370; continue; }
                pc = 0x80135368;
            }
            0x80135368 => {
                c.r[3] = 0x0;
                pc = 0x80135398; continue;
                pc = 0x80135370;
            }
            0x80135370 => {
                c.r[4] = 0x1;
                c.lr = 0x80135378; f_80129280(c);
                pc = 0x80135378;
            }
            0x80135378 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135388; continue; }
                pc = 0x80135380;
            }
            0x80135380 => {
                c.r[3] = 0x0;
                pc = 0x80135398; continue;
                pc = 0x80135388;
            }
            0x80135388 => {
                c.lr = 0x8013538c; f_80135b0c(c);
                pc = 0x8013538c;
            }
            0x8013538c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135398; continue; }
                pc = 0x80135394;
            }
            0x80135394 => {
                c.r[3] = 0x0;
                pc = 0x80135398;
            }
            0x80135398 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801353ac; continue; }
                pc = 0x801353a0;
            }
            0x801353a0 => {
                c.lr = 0x801353a4; f_80135708(c);
                pc = 0x801353a4;
            }
            0x801353a4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801353ac; f_80135ce8(c);
                pc = 0x801353ac;
            }
            0x801353ac => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80135338: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestCreate (0x801353c0, 368 bytes)
#[inline(never)]
pub fn f_801353c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801353c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801353c0); f_801353c0_b(c, 0x801353c0); crate::prof::exit(c, 0x801353c0); return; }
    }
    f_801353c0_b(c, 0x801353c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801353c0_at(c: &mut Cpu, pc: u32) { f_801353c0_b(c, pc) }
#[inline(always)]
fn f_801353c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801353c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801353c0, pc); }
        match pc {
            0x801353c0 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                c.r[28] = c.r[4] | c.r[4];
                c.r[29] = c.r[5] | c.r[5];
                c.r[30] = c.r[6] | c.r[6];
                c.r[31] = c.r[7] | c.r[7];
                if c.crbit(2) { pc = 0x8013551c; continue; }
                pc = 0x801353e8;
            }
            0x801353e8 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8013551c; continue; }
                pc = 0x801353f0;
            }
            0x801353f0 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8013551c; continue; }
                pc = 0x801353f8;
            }
            0x801353f8 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8013551c; continue; }
                pc = 0x80135400;
            }
            0x80135400 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x8013551c; continue; }
                pc = 0x80135408;
            }
            0x80135408 => {
                c.lr = 0x8013540c; f_80135708(c);
                pc = 0x8013540c;
            }
            0x8013540c => {
                c.cmpu(0, c.r[27], 0x0);
                c.r[3] = c.r[27] | c.r[27];
                if !c.crbit(2) { pc = 0x8013543c; continue; }
                pc = 0x80135418;
            }
            0x80135418 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80135424; f_80129280(c);
                pc = 0x80135424;
            }
            0x80135424 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135450; continue; }
                pc = 0x8013542c;
            }
            0x8013542c => {
                c.r[4] = 0x1;
                c.lr = 0x80135434; f_80129280(c);
                pc = 0x80135434;
            }
            0x80135434 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135450; continue; }
                pc = 0x8013543c;
            }
            0x8013543c => {
                c.lr = 0x80135440; f_80135b0c(c);
                pc = 0x80135440;
            }
            0x80135440 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135450; continue; }
                pc = 0x80135448;
            }
            0x80135448 => {
                c.r[4] = c.r[28].rotate_left(0) & 0xff;
                c.lr = 0x80135450; f_80135a60(c);
                pc = 0x80135450;
            }
            0x80135450 => {
                c.cmpu(0, c.r[27], 0x0);
                c.r[3] = c.r[27] | c.r[27];
                if !c.crbit(2) { pc = 0x80135480; continue; }
                pc = 0x8013545c;
            }
            0x8013545c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80135468; f_80129280(c);
                pc = 0x80135468;
            }
            0x80135468 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135494; continue; }
                pc = 0x80135470;
            }
            0x80135470 => {
                c.r[4] = 0x1;
                c.lr = 0x80135478; f_80129280(c);
                pc = 0x80135478;
            }
            0x80135478 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135494; continue; }
                pc = 0x80135480;
            }
            0x80135480 => {
                c.lr = 0x80135484; f_80135b0c(c);
                pc = 0x80135484;
            }
            0x80135484 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135494; continue; }
                pc = 0x8013548c;
            }
            0x8013548c => {
                c.r[4] = c.r[29].rotate_left(0) & 0xff;
                c.lr = 0x80135494; f_80135a50(c);
                pc = 0x80135494;
            }
            0x80135494 => {
                c.cmpu(0, c.r[27], 0x0);
                c.r[3] = c.r[27] | c.r[27];
                if !c.crbit(2) { pc = 0x801354c4; continue; }
                pc = 0x801354a0;
            }
            0x801354a0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x801354ac; f_80129280(c);
                pc = 0x801354ac;
            }
            0x801354ac => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801354d8; continue; }
                pc = 0x801354b4;
            }
            0x801354b4 => {
                c.r[4] = 0x1;
                c.lr = 0x801354bc; f_80129280(c);
                pc = 0x801354bc;
            }
            0x801354bc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801354d8; continue; }
                pc = 0x801354c4;
            }
            0x801354c4 => {
                c.lr = 0x801354c8; f_80135b0c(c);
                pc = 0x801354c8;
            }
            0x801354c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801354d8; continue; }
                pc = 0x801354d0;
            }
            0x801354d0 => {
                c.r[4] = c.r[30].rotate_left(0) & 0xff;
                c.lr = 0x801354d8; f_80135a40(c);
                pc = 0x801354d8;
            }
            0x801354d8 => {
                c.cmpu(0, c.r[27], 0x0);
                c.r[3] = c.r[27] | c.r[27];
                if !c.crbit(2) { pc = 0x80135508; continue; }
                pc = 0x801354e4;
            }
            0x801354e4 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x801354f0; f_80129280(c);
                pc = 0x801354f0;
            }
            0x801354f0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8013551c; continue; }
                pc = 0x801354f8;
            }
            0x801354f8 => {
                c.r[4] = 0x1;
                c.lr = 0x80135500; f_80129280(c);
                pc = 0x80135500;
            }
            0x80135500 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8013551c; continue; }
                pc = 0x80135508;
            }
            0x80135508 => {
                c.lr = 0x8013550c; f_80135b0c(c);
                pc = 0x8013550c;
            }
            0x8013550c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8013551c; continue; }
                pc = 0x80135514;
            }
            0x80135514 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.lr = 0x8013551c; f_80135a30(c);
                pc = 0x8013551c;
            }
            0x8013551c => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_801353c0: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestCheckValid (0x80135530, 472 bytes)
#[inline(never)]
pub fn f_80135530(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135530) { return; }
        if c.prof { crate::prof::enter(c, 0x80135530); f_80135530_b(c, 0x80135530); crate::prof::exit(c, 0x80135530); return; }
    }
    f_80135530_b(c, 0x80135530)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135530_at(c: &mut Cpu, pc: u32) { f_80135530_b(c, pc) }
#[inline(always)]
fn f_80135530_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135530;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135530, pc); }
        match pc {
            0x80135530 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if !c.crbit(2) { pc = 0x80135550; continue; }
                pc = 0x80135548;
            }
            0x80135548 => {
                c.r[3] = 0x0;
                pc = 0x801356f4; continue;
                pc = 0x80135550;
            }
            0x80135550 => {
                if !c.crbit(2) { pc = 0x80135588; continue; }
                pc = 0x80135554;
            }
            0x80135554 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80135560; f_80129280(c);
                pc = 0x80135560;
            }
            0x80135560 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135570; continue; }
                pc = 0x80135568;
            }
            0x80135568 => {
                c.r[0] = 0x0;
                pc = 0x801355a4; continue;
                pc = 0x80135570;
            }
            0x80135570 => {
                c.r[4] = 0x1;
                c.lr = 0x80135578; f_80129280(c);
                pc = 0x80135578;
            }
            0x80135578 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135588; continue; }
                pc = 0x80135580;
            }
            0x80135580 => {
                c.r[0] = 0x0;
                pc = 0x801355a4; continue;
                pc = 0x80135588;
            }
            0x80135588 => {
                c.lr = 0x8013558c; f_80135b0c(c);
                pc = 0x8013558c;
            }
            0x8013558c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8013559c; continue; }
                pc = 0x80135594;
            }
            0x80135594 => {
                c.r[0] = 0x0;
                pc = 0x801355a4; continue;
                pc = 0x8013559c;
            }
            0x8013559c => {
                c.lr = 0x801355a0; f_80135ab8(c);
                pc = 0x801355a0;
            }
            0x801355a0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801355a4;
            }
            0x801355a4 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x801355b4; continue; }
                pc = 0x801355ac;
            }
            0x801355ac => {
                c.r[3] = 0x0;
                pc = 0x801356f4; continue;
                pc = 0x801355b4;
            }
            0x801355b4 => {
                c.cmpu(0, c.r[31], 0x0);
                c.r[3] = c.r[31] | c.r[31];
                if !c.crbit(2) { pc = 0x801355f4; continue; }
                pc = 0x801355c0;
            }
            0x801355c0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x801355cc; f_80129280(c);
                pc = 0x801355cc;
            }
            0x801355cc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801355dc; continue; }
                pc = 0x801355d4;
            }
            0x801355d4 => {
                c.r[0] = 0x0;
                pc = 0x80135610; continue;
                pc = 0x801355dc;
            }
            0x801355dc => {
                c.r[4] = 0x1;
                c.lr = 0x801355e4; f_80129280(c);
                pc = 0x801355e4;
            }
            0x801355e4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801355f4; continue; }
                pc = 0x801355ec;
            }
            0x801355ec => {
                c.r[0] = 0x0;
                pc = 0x80135610; continue;
                pc = 0x801355f4;
            }
            0x801355f4 => {
                c.lr = 0x801355f8; f_80135b0c(c);
                pc = 0x801355f8;
            }
            0x801355f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135608; continue; }
                pc = 0x80135600;
            }
            0x80135600 => {
                c.r[0] = 0x0;
                pc = 0x80135610; continue;
                pc = 0x80135608;
            }
            0x80135608 => {
                c.lr = 0x8013560c; f_80135aa0(c);
                pc = 0x8013560c;
            }
            0x8013560c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80135610;
            }
            0x80135610 => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80135620; continue; }
                pc = 0x80135618;
            }
            0x80135618 => {
                c.r[3] = 0x0;
                pc = 0x801356f4; continue;
                pc = 0x80135620;
            }
            0x80135620 => {
                c.cmpu(0, c.r[31], 0x0);
                c.r[3] = c.r[31] | c.r[31];
                if !c.crbit(2) { pc = 0x80135660; continue; }
                pc = 0x8013562c;
            }
            0x8013562c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80135638; f_80129280(c);
                pc = 0x80135638;
            }
            0x80135638 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135648; continue; }
                pc = 0x80135640;
            }
            0x80135640 => {
                c.r[0] = 0x0;
                pc = 0x8013567c; continue;
                pc = 0x80135648;
            }
            0x80135648 => {
                c.r[4] = 0x1;
                c.lr = 0x80135650; f_80129280(c);
                pc = 0x80135650;
            }
            0x80135650 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135660; continue; }
                pc = 0x80135658;
            }
            0x80135658 => {
                c.r[0] = 0x0;
                pc = 0x8013567c; continue;
                pc = 0x80135660;
            }
            0x80135660 => {
                c.lr = 0x80135664; f_80135b0c(c);
                pc = 0x80135664;
            }
            0x80135664 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135674; continue; }
                pc = 0x8013566c;
            }
            0x8013566c => {
                c.r[0] = 0x0;
                pc = 0x8013567c; continue;
                pc = 0x80135674;
            }
            0x80135674 => {
                c.lr = 0x80135678; f_80135a88(c);
                pc = 0x80135678;
            }
            0x80135678 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x8013567c;
            }
            0x8013567c => {
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x8013568c; continue; }
                pc = 0x80135684;
            }
            0x80135684 => {
                c.r[3] = 0x0;
                pc = 0x801356f4; continue;
                pc = 0x8013568c;
            }
            0x8013568c => {
                c.cmpu(0, c.r[31], 0x0);
                c.r[3] = c.r[31] | c.r[31];
                if !c.crbit(2) { pc = 0x801356cc; continue; }
                pc = 0x80135698;
            }
            0x80135698 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x801356a4; f_80129280(c);
                pc = 0x801356a4;
            }
            0x801356a4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801356b4; continue; }
                pc = 0x801356ac;
            }
            0x801356ac => {
                c.r[3] = 0x0;
                pc = 0x801356e8; continue;
                pc = 0x801356b4;
            }
            0x801356b4 => {
                c.r[4] = 0x1;
                c.lr = 0x801356bc; f_80129280(c);
                pc = 0x801356bc;
            }
            0x801356bc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801356cc; continue; }
                pc = 0x801356c4;
            }
            0x801356c4 => {
                c.r[3] = 0x0;
                pc = 0x801356e8; continue;
                pc = 0x801356cc;
            }
            0x801356cc => {
                c.lr = 0x801356d0; f_80135b0c(c);
                pc = 0x801356d0;
            }
            0x801356d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801356e0; continue; }
                pc = 0x801356d8;
            }
            0x801356d8 => {
                c.r[3] = 0x0;
                pc = 0x801356e8; continue;
                pc = 0x801356e0;
            }
            0x801356e0 => {
                c.lr = 0x801356e4; f_80135a70(c);
                pc = 0x801356e4;
            }
            0x801356e4 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x801356e8;
            }
            0x801356e8 => {
                c.r[0] = c.r[3].wrapping_neg();
                c.r[0] = c.r[0] | c.r[3];
                c.r[3] = c.r[0].rotate_left(1) & 0x1;
                pc = 0x801356f4;
            }
            0x801356f4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80135530: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestInit (0x80135708, 308 bytes)
#[inline(never)]
pub fn f_80135708(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135708) { return; }
        if c.prof { crate::prof::enter(c, 0x80135708); f_80135708_b(c, 0x80135708); crate::prof::exit(c, 0x80135708); return; }
    }
    f_80135708_b(c, 0x80135708)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135708_at(c: &mut Cpu, pc: u32) { f_80135708_b(c, pc) }
#[inline(always)]
fn f_80135708_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135708;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135708, pc); }
        match pc {
            0x80135708 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x80135828; continue; }
                pc = 0x80135720;
            }
            0x80135720 => {
                if !c.crbit(2) { pc = 0x80135748; continue; }
                pc = 0x80135724;
            }
            0x80135724 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80135730; f_80129280(c);
                pc = 0x80135730;
            }
            0x80135730 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8013575c; continue; }
                pc = 0x80135738;
            }
            0x80135738 => {
                c.r[4] = 0x1;
                c.lr = 0x80135740; f_80129280(c);
                pc = 0x80135740;
            }
            0x80135740 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8013575c; continue; }
                pc = 0x80135748;
            }
            0x80135748 => {
                c.lr = 0x8013574c; f_80135b0c(c);
                pc = 0x8013574c;
            }
            0x8013574c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8013575c; continue; }
                pc = 0x80135754;
            }
            0x80135754 => {
                c.r[4] = 0x0;
                c.lr = 0x8013575c; f_80135a60(c);
                pc = 0x8013575c;
            }
            0x8013575c => {
                c.cmpu(0, c.r[31], 0x0);
                c.r[3] = c.r[31] | c.r[31];
                if !c.crbit(2) { pc = 0x8013578c; continue; }
                pc = 0x80135768;
            }
            0x80135768 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80135774; f_80129280(c);
                pc = 0x80135774;
            }
            0x80135774 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801357a0; continue; }
                pc = 0x8013577c;
            }
            0x8013577c => {
                c.r[4] = 0x1;
                c.lr = 0x80135784; f_80129280(c);
                pc = 0x80135784;
            }
            0x80135784 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801357a0; continue; }
                pc = 0x8013578c;
            }
            0x8013578c => {
                c.lr = 0x80135790; f_80135b0c(c);
                pc = 0x80135790;
            }
            0x80135790 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801357a0; continue; }
                pc = 0x80135798;
            }
            0x80135798 => {
                c.r[4] = 0x0;
                c.lr = 0x801357a0; f_80135a50(c);
                pc = 0x801357a0;
            }
            0x801357a0 => {
                c.cmpu(0, c.r[31], 0x0);
                c.r[3] = c.r[31] | c.r[31];
                if !c.crbit(2) { pc = 0x801357d0; continue; }
                pc = 0x801357ac;
            }
            0x801357ac => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x801357b8; f_80129280(c);
                pc = 0x801357b8;
            }
            0x801357b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801357e4; continue; }
                pc = 0x801357c0;
            }
            0x801357c0 => {
                c.r[4] = 0x1;
                c.lr = 0x801357c8; f_80129280(c);
                pc = 0x801357c8;
            }
            0x801357c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801357e4; continue; }
                pc = 0x801357d0;
            }
            0x801357d0 => {
                c.lr = 0x801357d4; f_80135b0c(c);
                pc = 0x801357d4;
            }
            0x801357d4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801357e4; continue; }
                pc = 0x801357dc;
            }
            0x801357dc => {
                c.r[4] = 0x0;
                c.lr = 0x801357e4; f_80135a40(c);
                pc = 0x801357e4;
            }
            0x801357e4 => {
                c.cmpu(0, c.r[31], 0x0);
                c.r[3] = c.r[31] | c.r[31];
                if !c.crbit(2) { pc = 0x80135814; continue; }
                pc = 0x801357f0;
            }
            0x801357f0 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x801357fc; f_80129280(c);
                pc = 0x801357fc;
            }
            0x801357fc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135828; continue; }
                pc = 0x80135804;
            }
            0x80135804 => {
                c.r[4] = 0x1;
                c.lr = 0x8013580c; f_80129280(c);
                pc = 0x8013580c;
            }
            0x8013580c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135828; continue; }
                pc = 0x80135814;
            }
            0x80135814 => {
                c.lr = 0x80135818; f_80135b0c(c);
                pc = 0x80135818;
            }
            0x80135818 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135828; continue; }
                pc = 0x80135820;
            }
            0x80135820 => {
                c.r[4] = 0x0;
                c.lr = 0x80135828; f_80135a30(c);
                pc = 0x80135828;
            }
            0x80135828 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80135708: bad pc {:#x}", pc),
        }
    }
}

/// gamedataSetStatus (0x8013583c, 252 bytes)
#[inline(never)]
pub fn f_8013583c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013583c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013583c); f_8013583c_b(c, 0x8013583c); crate::prof::exit(c, 0x8013583c); return; }
    }
    f_8013583c_b(c, 0x8013583c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013583c_at(c: &mut Cpu, pc: u32) { f_8013583c_b(c, pc) }
#[inline(always)]
fn f_8013583c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013583c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013583c, pc); }
        match pc {
            0x8013583c => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x8013591c; continue; }
                pc = 0x80135868;
            }
            0x80135868 => {
                c.cmpu(0, c.r[0], 0x7);
                if c.crbit(0) { pc = 0x80135874; continue; }
                pc = 0x80135870;
            }
            0x80135870 => {
                pc = 0x8013591c; continue;
                pc = 0x80135874;
            }
            0x80135874 => {
                c.cmpu(0, c.r[29], 0x0);
                if !c.crbit(2) { pc = 0x801358a0; continue; }
                pc = 0x8013587c;
            }
            0x8013587c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80135888; f_80129280(c);
                pc = 0x80135888;
            }
            0x80135888 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8013591c; continue; }
                pc = 0x80135890;
            }
            0x80135890 => {
                c.r[4] = 0x1;
                c.lr = 0x80135898; f_80129280(c);
                pc = 0x80135898;
            }
            0x80135898 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x8013591c; continue; }
                pc = 0x801358a0;
            }
            0x801358a0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x801358a8; f_80135b0c(c);
                pc = 0x801358a8;
            }
            0x801358a8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8013591c; continue; }
                pc = 0x801358b0;
            }
            0x801358b0 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x801358fc; continue; }
                pc = 0x801358bc;
            }
            0x801358bc => {
                if !c.crbit(0) { pc = 0x801358d0; continue; }
                pc = 0x801358c0;
            }
            0x801358c0 => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x801358e0; continue; }
                pc = 0x801358c8;
            }
            0x801358c8 => {
                if !c.crbit(0) { pc = 0x801358f0; continue; }
                pc = 0x801358cc;
            }
            0x801358cc => {
                pc = 0x8013591c; continue;
                pc = 0x801358d0;
            }
            0x801358d0 => {
                c.cmps(0, c.r[0], 0x5);
                if c.crbit(2) { pc = 0x80135914; continue; }
                pc = 0x801358d8;
            }
            0x801358d8 => {
                if !c.crbit(0) { pc = 0x8013591c; continue; }
                pc = 0x801358dc;
            }
            0x801358dc => {
                pc = 0x80135908; continue;
                pc = 0x801358e0;
            }
            0x801358e0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x801358ec; f_80135aec(c);
                pc = 0x801358ec;
            }
            0x801358ec => {
                pc = 0x8013591c; continue;
                pc = 0x801358f0;
            }
            0x801358f0 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.lr = 0x801358f8; f_80135a60(c);
                pc = 0x801358f8;
            }
            0x801358f8 => {
                pc = 0x8013591c; continue;
                pc = 0x801358fc;
            }
            0x801358fc => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.lr = 0x80135904; f_80135a50(c);
                pc = 0x80135904;
            }
            0x80135904 => {
                pc = 0x8013591c; continue;
                pc = 0x80135908;
            }
            0x80135908 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.lr = 0x80135910; f_80135a40(c);
                pc = 0x80135910;
            }
            0x80135910 => {
                pc = 0x8013591c; continue;
                pc = 0x80135914;
            }
            0x80135914 => {
                c.r[4] = c.r[31].rotate_left(0) & 0xff;
                c.lr = 0x8013591c; f_80135a30(c);
                pc = 0x8013591c;
            }
            0x8013591c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x18));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_8013583c: bad pc {:#x}", pc),
        }
    }
}

/// gamedataGetStatus (0x80135938, 248 bytes)
#[inline(never)]
pub fn f_80135938(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135938) { return; }
        if c.prof { crate::prof::enter(c, 0x80135938); f_80135938_b(c, 0x80135938); crate::prof::exit(c, 0x80135938); return; }
    }
    f_80135938_b(c, 0x80135938)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135938_at(c: &mut Cpu, pc: u32) { f_80135938_b(c, pc) }
#[inline(always)]
fn f_80135938_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135938;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135938, pc); }
        match pc {
            0x80135938 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4] | c.r[4];
                if c.crbit(2) { pc = 0x8013595c; continue; }
                pc = 0x80135954;
            }
            0x80135954 => {
                c.cmpu(0, c.r[0], 0x7);
                if c.crbit(0) { pc = 0x80135964; continue; }
                pc = 0x8013595c;
            }
            0x8013595c => {
                c.r[3] = 0x0;
                pc = 0x80135a1c; continue;
                pc = 0x80135964;
            }
            0x80135964 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801359a0; continue; }
                pc = 0x8013596c;
            }
            0x8013596c => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.lr = 0x80135978; f_80129280(c);
                pc = 0x80135978;
            }
            0x80135978 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135988; continue; }
                pc = 0x80135980;
            }
            0x80135980 => {
                c.r[3] = 0x0;
                pc = 0x80135a1c; continue;
                pc = 0x80135988;
            }
            0x80135988 => {
                c.r[4] = 0x1;
                c.lr = 0x80135990; f_80129280(c);
                pc = 0x80135990;
            }
            0x80135990 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801359a0; continue; }
                pc = 0x80135998;
            }
            0x80135998 => {
                c.r[3] = 0x0;
                pc = 0x80135a1c; continue;
                pc = 0x801359a0;
            }
            0x801359a0 => {
                c.lr = 0x801359a4; f_80135b0c(c);
                pc = 0x801359a4;
            }
            0x801359a4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801359b4; continue; }
                pc = 0x801359ac;
            }
            0x801359ac => {
                c.r[3] = 0x0;
                pc = 0x80135a1c; continue;
                pc = 0x801359b4;
            }
            0x801359b4 => {
                c.r[0] = c.r[31].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x801359f4; continue; }
                pc = 0x801359c0;
            }
            0x801359c0 => {
                if !c.crbit(0) { pc = 0x801359d4; continue; }
                pc = 0x801359c4;
            }
            0x801359c4 => {
                c.cmps(0, c.r[0], 0x1);
                if c.crbit(2) { pc = 0x80135a1c; continue; }
                pc = 0x801359cc;
            }
            0x801359cc => {
                if !c.crbit(0) { pc = 0x801359e8; continue; }
                pc = 0x801359d0;
            }
            0x801359d0 => {
                pc = 0x80135a18; continue;
                pc = 0x801359d4;
            }
            0x801359d4 => {
                c.cmps(0, c.r[0], 0x5);
                if c.crbit(2) { pc = 0x80135a0c; continue; }
                pc = 0x801359dc;
            }
            0x801359dc => {
                if !c.crbit(0) { pc = 0x80135a18; continue; }
                pc = 0x801359e0;
            }
            0x801359e0 => {
                pc = 0x80135a00; continue;
                pc = 0x801359e4;
            }
            0x801359e4 => {
                pc = 0x80135a1c; continue;
                pc = 0x801359e8;
            }
            0x801359e8 => {
                c.lr = 0x801359ec; f_80135ab8(c);
                pc = 0x801359ec;
            }
            0x801359ec => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80135a1c; continue;
                pc = 0x801359f4;
            }
            0x801359f4 => {
                c.lr = 0x801359f8; f_80135aa0(c);
                pc = 0x801359f8;
            }
            0x801359f8 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80135a1c; continue;
                pc = 0x80135a00;
            }
            0x80135a00 => {
                c.lr = 0x80135a04; f_80135a88(c);
                pc = 0x80135a04;
            }
            0x80135a04 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80135a1c; continue;
                pc = 0x80135a0c;
            }
            0x80135a0c => {
                c.lr = 0x80135a10; f_80135a70(c);
                pc = 0x80135a10;
            }
            0x80135a10 => {
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80135a1c; continue;
                pc = 0x80135a18;
            }
            0x80135a18 => {
                c.r[3] = 0x0;
                pc = 0x80135a1c;
            }
            0x80135a1c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80135938: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestBiosSetLangareaId (0x80135a30, 16 bytes)
#[inline(never)]
pub fn f_80135a30(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135a30) { return; }
        if c.prof { crate::prof::enter(c, 0x80135a30); f_80135a30_b(c, 0x80135a30); crate::prof::exit(c, 0x80135a30); return; }
    }
    f_80135a30_b(c, 0x80135a30)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135a30_at(c: &mut Cpu, pc: u32) { f_80135a30_b(c, pc) }
#[inline(always)]
fn f_80135a30_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135a30;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135a30, pc); }
        match pc {
            0x80135a30 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135a38;
            }
            0x80135a38 => {
                c.stb(c.r[3].wrapping_add(0x3), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135a30: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestBiosSetAreaId (0x80135a40, 16 bytes)
#[inline(never)]
pub fn f_80135a40(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135a40) { return; }
        if c.prof { crate::prof::enter(c, 0x80135a40); f_80135a40_b(c, 0x80135a40); crate::prof::exit(c, 0x80135a40); return; }
    }
    f_80135a40_b(c, 0x80135a40)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135a40_at(c: &mut Cpu, pc: u32) { f_80135a40_b(c, pc) }
#[inline(always)]
fn f_80135a40_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135a40;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135a40, pc); }
        match pc {
            0x80135a40 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135a48;
            }
            0x80135a48 => {
                c.stb(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135a40: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestBiosSetGenId (0x80135a50, 16 bytes)
#[inline(never)]
pub fn f_80135a50(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135a50) { return; }
        if c.prof { crate::prof::enter(c, 0x80135a50); f_80135a50_b(c, 0x80135a50); crate::prof::exit(c, 0x80135a50); return; }
    }
    f_80135a50_b(c, 0x80135a50)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135a50_at(c: &mut Cpu, pc: u32) { f_80135a50_b(c, pc) }
#[inline(always)]
fn f_80135a50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135a50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135a50, pc); }
        match pc {
            0x80135a50 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135a58;
            }
            0x80135a58 => {
                c.stb(c.r[3].wrapping_add(0x1), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135a50: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestBiosSetVerId (0x80135a60, 16 bytes)
#[inline(never)]
pub fn f_80135a60(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135a60) { return; }
        if c.prof { crate::prof::enter(c, 0x80135a60); f_80135a60_b(c, 0x80135a60); crate::prof::exit(c, 0x80135a60); return; }
    }
    f_80135a60_b(c, 0x80135a60)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135a60_at(c: &mut Cpu, pc: u32) { f_80135a60_b(c, pc) }
#[inline(always)]
fn f_80135a60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135a60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135a60, pc); }
        match pc {
            0x80135a60 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135a68;
            }
            0x80135a68 => {
                c.stb(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135a60: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestBiosGetLangareaId (0x80135a70, 24 bytes)
#[inline(never)]
pub fn f_80135a70(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135a70) { return; }
        if c.prof { crate::prof::enter(c, 0x80135a70); f_80135a70_b(c, 0x80135a70); crate::prof::exit(c, 0x80135a70); return; }
    }
    f_80135a70_b(c, 0x80135a70)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135a70_at(c: &mut Cpu, pc: u32) { f_80135a70_b(c, pc) }
#[inline(always)]
fn f_80135a70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135a70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135a70, pc); }
        match pc {
            0x80135a70 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135a80; continue; }
                pc = 0x80135a78;
            }
            0x80135a78 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80135a80;
            }
            0x80135a80 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x3));
                if true { return; }
                return;
            }
            _ => panic!("f_80135a70: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestBiosGetAreaId (0x80135a88, 24 bytes)
#[inline(never)]
pub fn f_80135a88(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135a88) { return; }
        if c.prof { crate::prof::enter(c, 0x80135a88); f_80135a88_b(c, 0x80135a88); crate::prof::exit(c, 0x80135a88); return; }
    }
    f_80135a88_b(c, 0x80135a88)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135a88_at(c: &mut Cpu, pc: u32) { f_80135a88_b(c, pc) }
#[inline(always)]
fn f_80135a88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135a88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135a88, pc); }
        match pc {
            0x80135a88 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135a98; continue; }
                pc = 0x80135a90;
            }
            0x80135a90 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80135a98;
            }
            0x80135a98 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_80135a88: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestBiosGetGenId (0x80135aa0, 24 bytes)
#[inline(never)]
pub fn f_80135aa0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135aa0) { return; }
        if c.prof { crate::prof::enter(c, 0x80135aa0); f_80135aa0_b(c, 0x80135aa0); crate::prof::exit(c, 0x80135aa0); return; }
    }
    f_80135aa0_b(c, 0x80135aa0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135aa0_at(c: &mut Cpu, pc: u32) { f_80135aa0_b(c, pc) }
#[inline(always)]
fn f_80135aa0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135aa0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135aa0, pc); }
        match pc {
            0x80135aa0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135ab0; continue; }
                pc = 0x80135aa8;
            }
            0x80135aa8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80135ab0;
            }
            0x80135ab0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_80135aa0: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestBiosGetVerId (0x80135ab8, 24 bytes)
#[inline(never)]
pub fn f_80135ab8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135ab8) { return; }
        if c.prof { crate::prof::enter(c, 0x80135ab8); f_80135ab8_b(c, 0x80135ab8); crate::prof::exit(c, 0x80135ab8); return; }
    }
    f_80135ab8_b(c, 0x80135ab8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135ab8_at(c: &mut Cpu, pc: u32) { f_80135ab8_b(c, pc) }
#[inline(always)]
fn f_80135ab8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135ab8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135ab8, pc); }
        match pc {
            0x80135ab8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135ac8; continue; }
                pc = 0x80135ac0;
            }
            0x80135ac0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80135ac8;
            }
            0x80135ac8 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135ab8: bad pc {:#x}", pc),
        }
    }
}

/// gamedataAttestBiosCopy (0x80135ad0, 28 bytes)
#[inline(never)]
pub fn f_80135ad0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135ad0) { return; }
        if c.prof { crate::prof::enter(c, 0x80135ad0); f_80135ad0_b(c, 0x80135ad0); crate::prof::exit(c, 0x80135ad0); return; }
    }
    f_80135ad0_b(c, 0x80135ad0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135ad0_at(c: &mut Cpu, pc: u32) { f_80135ad0_b(c, pc) }
#[inline(always)]
fn f_80135ad0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135ad0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135ad0, pc); }
        match pc {
            0x80135ad0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135ad8;
            }
            0x80135ad8 => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135ae0;
            }
            0x80135ae0 => {
                c.r[0] = c.lwz(c.r[4]);
                c.stw(c.r[3], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135ad0: bad pc {:#x}", pc),
        }
    }
}

/// gamedataBiosSetGamedataAtttestPtr (0x80135aec, 32 bytes)
#[inline(never)]
pub fn f_80135aec(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135aec) { return; }
        if c.prof { crate::prof::enter(c, 0x80135aec); f_80135aec_b(c, 0x80135aec); crate::prof::exit(c, 0x80135aec); return; }
    }
    f_80135aec_b(c, 0x80135aec)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135aec_at(c: &mut Cpu, pc: u32) { f_80135aec_b(c, pc) }
#[inline(always)]
fn f_80135aec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135aec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135aec, pc); }
        match pc {
            0x80135aec => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135af4;
            }
            0x80135af4 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x80135b00; continue; }
                pc = 0x80135afc;
            }
            0x80135afc => {
                if true { return; }
                pc = 0x80135b00;
            }
            0x80135b00 => {
                c.r[0] = c.lwz(c.r[4]);
                c.stw(c.r[3], c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135aec: bad pc {:#x}", pc),
        }
    }
}

/// gamedataBiosGetGamedataAtttestPtr (0x80135b0c, 16 bytes)
#[inline(never)]
pub fn f_80135b0c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135b0c) { return; }
        if c.prof { crate::prof::enter(c, 0x80135b0c); f_80135b0c_b(c, 0x80135b0c); crate::prof::exit(c, 0x80135b0c); return; }
    }
    f_80135b0c_b(c, 0x80135b0c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135b0c_at(c: &mut Cpu, pc: u32) { f_80135b0c_b(c, pc) }
#[inline(always)]
fn f_80135b0c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135b0c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135b0c, pc); }
        match pc {
            0x80135b0c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { return; }
                pc = 0x80135b14;
            }
            0x80135b14 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80135b0c: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosSetOptionAudio (0x80135b1c, 16 bytes)
#[inline(never)]
pub fn f_80135b1c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135b1c) { return; }
        if c.prof { crate::prof::enter(c, 0x80135b1c); f_80135b1c_b(c, 0x80135b1c); crate::prof::exit(c, 0x80135b1c); return; }
    }
    f_80135b1c_b(c, 0x80135b1c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135b1c_at(c: &mut Cpu, pc: u32) { f_80135b1c_b(c, pc) }
#[inline(always)]
fn f_80135b1c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135b1c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135b1c, pc); }
        match pc {
            0x80135b1c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135b24;
            }
            0x80135b24 => {
                c.stb(c.r[3].wrapping_add(0x22), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135b1c: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosSetOptionNoVibration (0x80135b2c, 16 bytes)
#[inline(never)]
pub fn f_80135b2c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135b2c) { return; }
        if c.prof { crate::prof::enter(c, 0x80135b2c); f_80135b2c_b(c, 0x80135b2c); crate::prof::exit(c, 0x80135b2c); return; }
    }
    f_80135b2c_b(c, 0x80135b2c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135b2c_at(c: &mut Cpu, pc: u32) { f_80135b2c_b(c, pc) }
#[inline(always)]
fn f_80135b2c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135b2c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135b2c, pc); }
        match pc {
            0x80135b2c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135b34;
            }
            0x80135b34 => {
                c.stb(c.r[3].wrapping_add(0x21), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135b2c: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosSetFloorposindex (0x80135b3c, 16 bytes)
#[inline(never)]
pub fn f_80135b3c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135b3c) { return; }
        if c.prof { crate::prof::enter(c, 0x80135b3c); f_80135b3c_b(c, 0x80135b3c); crate::prof::exit(c, 0x80135b3c); return; }
    }
    f_80135b3c_b(c, 0x80135b3c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135b3c_at(c: &mut Cpu, pc: u32) { f_80135b3c_b(c, pc) }
#[inline(always)]
fn f_80135b3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135b3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135b3c, pc); }
        match pc {
            0x80135b3c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135b44;
            }
            0x80135b44 => {
                c.stb(c.r[3].wrapping_add(0x20), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135b3c: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosSetPrevfloorid (0x80135b4c, 16 bytes)
#[inline(never)]
pub fn f_80135b4c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135b4c) { return; }
        if c.prof { crate::prof::enter(c, 0x80135b4c); f_80135b4c_b(c, 0x80135b4c); crate::prof::exit(c, 0x80135b4c); return; }
    }
    f_80135b4c_b(c, 0x80135b4c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135b4c_at(c: &mut Cpu, pc: u32) { f_80135b4c_b(c, pc) }
#[inline(always)]
fn f_80135b4c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135b4c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135b4c, pc); }
        match pc {
            0x80135b4c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135b54;
            }
            0x80135b54 => {
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135b4c: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosSetPlaytime (0x80135b5c, 16 bytes)
#[inline(never)]
pub fn f_80135b5c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135b5c) { return; }
        if c.prof { crate::prof::enter(c, 0x80135b5c); f_80135b5c_b(c, 0x80135b5c); crate::prof::exit(c, 0x80135b5c); return; }
    }
    f_80135b5c_b(c, 0x80135b5c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135b5c_at(c: &mut Cpu, pc: u32) { f_80135b5c_b(c, pc) }
#[inline(always)]
fn f_80135b5c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135b5c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135b5c, pc); }
        match pc {
            0x80135b5c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135b64;
            }
            0x80135b64 => {
                { let ea = c.r[3].wrapping_add(0x18); let v = c.f[1]; c.stfs(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_80135b5c: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosSetFloorid (0x80135b6c, 16 bytes)
#[inline(never)]
pub fn f_80135b6c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135b6c) { return; }
        if c.prof { crate::prof::enter(c, 0x80135b6c); f_80135b6c_b(c, 0x80135b6c); crate::prof::exit(c, 0x80135b6c); return; }
    }
    f_80135b6c_b(c, 0x80135b6c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135b6c_at(c: &mut Cpu, pc: u32) { f_80135b6c_b(c, pc) }
#[inline(always)]
fn f_80135b6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135b6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135b6c, pc); }
        match pc {
            0x80135b6c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135b74;
            }
            0x80135b74 => {
                c.stw(c.r[3].wrapping_add(0x14), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135b6c: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosSetSavecount (0x80135b7c, 16 bytes)
#[inline(never)]
pub fn f_80135b7c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135b7c) { return; }
        if c.prof { crate::prof::enter(c, 0x80135b7c); f_80135b7c_b(c, 0x80135b7c); crate::prof::exit(c, 0x80135b7c); return; }
    }
    f_80135b7c_b(c, 0x80135b7c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135b7c_at(c: &mut Cpu, pc: u32) { f_80135b7c_b(c, pc) }
#[inline(always)]
fn f_80135b7c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135b7c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135b7c, pc); }
        match pc {
            0x80135b7c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135b84;
            }
            0x80135b84 => {
                c.stw(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135b7c: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosSetMemcardID (0x80135b8c, 20 bytes)
#[inline(never)]
pub fn f_80135b8c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135b8c) { return; }
        if c.prof { crate::prof::enter(c, 0x80135b8c); f_80135b8c_b(c, 0x80135b8c); crate::prof::exit(c, 0x80135b8c); return; }
    }
    f_80135b8c_b(c, 0x80135b8c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135b8c_at(c: &mut Cpu, pc: u32) { f_80135b8c_b(c, pc) }
#[inline(always)]
fn f_80135b8c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135b8c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135b8c, pc); }
        match pc {
            0x80135b8c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135b94;
            }
            0x80135b94 => {
                c.stw(c.r[3].wrapping_add(0x4), c.r[6]);
                c.stw(c.r[3], c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135b8c: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosSetSavernd (0x80135ba0, 16 bytes)
#[inline(never)]
pub fn f_80135ba0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135ba0) { return; }
        if c.prof { crate::prof::enter(c, 0x80135ba0); f_80135ba0_b(c, 0x80135ba0); crate::prof::exit(c, 0x80135ba0); return; }
    }
    f_80135ba0_b(c, 0x80135ba0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135ba0_at(c: &mut Cpu, pc: u32) { f_80135ba0_b(c, pc) }
#[inline(always)]
fn f_80135ba0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135ba0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135ba0, pc); }
        match pc {
            0x80135ba0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135ba8;
            }
            0x80135ba8 => {
                c.stw(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135ba0: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosGetOptionAudio (0x80135bb0, 24 bytes)
#[inline(never)]
pub fn f_80135bb0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135bb0) { return; }
        if c.prof { crate::prof::enter(c, 0x80135bb0); f_80135bb0_b(c, 0x80135bb0); crate::prof::exit(c, 0x80135bb0); return; }
    }
    f_80135bb0_b(c, 0x80135bb0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135bb0_at(c: &mut Cpu, pc: u32) { f_80135bb0_b(c, pc) }
#[inline(always)]
fn f_80135bb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135bb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135bb0, pc); }
        match pc {
            0x80135bb0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135bc0; continue; }
                pc = 0x80135bb8;
            }
            0x80135bb8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x22));
                if true { return; }
                pc = 0x80135bc0;
            }
            0x80135bc0 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80135bb0: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosGetOptionNoVibration (0x80135bc8, 24 bytes)
#[inline(never)]
pub fn f_80135bc8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135bc8) { return; }
        if c.prof { crate::prof::enter(c, 0x80135bc8); f_80135bc8_b(c, 0x80135bc8); crate::prof::exit(c, 0x80135bc8); return; }
    }
    f_80135bc8_b(c, 0x80135bc8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135bc8_at(c: &mut Cpu, pc: u32) { f_80135bc8_b(c, pc) }
#[inline(always)]
fn f_80135bc8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135bc8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135bc8, pc); }
        match pc {
            0x80135bc8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135bd8; continue; }
                pc = 0x80135bd0;
            }
            0x80135bd0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x21));
                if true { return; }
                pc = 0x80135bd8;
            }
            0x80135bd8 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80135bc8: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosGetFloorposindex (0x80135be0, 24 bytes)
#[inline(never)]
pub fn f_80135be0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135be0) { return; }
        if c.prof { crate::prof::enter(c, 0x80135be0); f_80135be0_b(c, 0x80135be0); crate::prof::exit(c, 0x80135be0); return; }
    }
    f_80135be0_b(c, 0x80135be0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135be0_at(c: &mut Cpu, pc: u32) { f_80135be0_b(c, pc) }
#[inline(always)]
fn f_80135be0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135be0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135be0, pc); }
        match pc {
            0x80135be0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135bf0; continue; }
                pc = 0x80135be8;
            }
            0x80135be8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x20));
                if true { return; }
                pc = 0x80135bf0;
            }
            0x80135bf0 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80135be0: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosGetPrevfloorid (0x80135bf8, 24 bytes)
#[inline(never)]
pub fn f_80135bf8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135bf8) { return; }
        if c.prof { crate::prof::enter(c, 0x80135bf8); f_80135bf8_b(c, 0x80135bf8); crate::prof::exit(c, 0x80135bf8); return; }
    }
    f_80135bf8_b(c, 0x80135bf8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135bf8_at(c: &mut Cpu, pc: u32) { f_80135bf8_b(c, pc) }
#[inline(always)]
fn f_80135bf8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135bf8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135bf8, pc); }
        match pc {
            0x80135bf8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135c08; continue; }
                pc = 0x80135c00;
            }
            0x80135c00 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x1c));
                if true { return; }
                pc = 0x80135c08;
            }
            0x80135c08 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80135bf8: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosGetPlaytime (0x80135c10, 24 bytes)
#[inline(never)]
pub fn f_80135c10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135c10) { return; }
        if c.prof { crate::prof::enter(c, 0x80135c10); f_80135c10_b(c, 0x80135c10); crate::prof::exit(c, 0x80135c10); return; }
    }
    f_80135c10_b(c, 0x80135c10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135c10_at(c: &mut Cpu, pc: u32) { f_80135c10_b(c, pc) }
#[inline(always)]
fn f_80135c10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135c10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135c10, pc); }
        match pc {
            0x80135c10 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135c20; continue; }
                pc = 0x80135c18;
            }
            0x80135c18 => {
                { let ea = c.r[3].wrapping_add(0x18); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { return; }
                pc = 0x80135c20;
            }
            0x80135c20 => {
                { let ea = c.r[2].wrapping_add(0xffff9a70); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                if true { return; }
                return;
            }
            _ => panic!("f_80135c10: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosGetFloorid (0x80135c28, 24 bytes)
#[inline(never)]
pub fn f_80135c28(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135c28) { return; }
        if c.prof { crate::prof::enter(c, 0x80135c28); f_80135c28_b(c, 0x80135c28); crate::prof::exit(c, 0x80135c28); return; }
    }
    f_80135c28_b(c, 0x80135c28)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135c28_at(c: &mut Cpu, pc: u32) { f_80135c28_b(c, pc) }
#[inline(always)]
fn f_80135c28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135c28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135c28, pc); }
        match pc {
            0x80135c28 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135c38; continue; }
                pc = 0x80135c30;
            }
            0x80135c30 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                if true { return; }
                pc = 0x80135c38;
            }
            0x80135c38 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80135c28: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosGetSavecount (0x80135c40, 24 bytes)
#[inline(never)]
pub fn f_80135c40(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135c40) { return; }
        if c.prof { crate::prof::enter(c, 0x80135c40); f_80135c40_b(c, 0x80135c40); crate::prof::exit(c, 0x80135c40); return; }
    }
    f_80135c40_b(c, 0x80135c40)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135c40_at(c: &mut Cpu, pc: u32) { f_80135c40_b(c, pc) }
#[inline(always)]
fn f_80135c40_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135c40;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135c40, pc); }
        match pc {
            0x80135c40 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135c50; continue; }
                pc = 0x80135c48;
            }
            0x80135c48 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { return; }
                pc = 0x80135c50;
            }
            0x80135c50 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80135c40: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosGetMemcardID (0x80135c58, 32 bytes)
#[inline(never)]
pub fn f_80135c58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135c58) { return; }
        if c.prof { crate::prof::enter(c, 0x80135c58); f_80135c58_b(c, 0x80135c58); crate::prof::exit(c, 0x80135c58); return; }
    }
    f_80135c58_b(c, 0x80135c58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135c58_at(c: &mut Cpu, pc: u32) { f_80135c58_b(c, pc) }
#[inline(always)]
fn f_80135c58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135c58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135c58, pc); }
        match pc {
            0x80135c58 => {
                c.r[4] = c.r[3] | c.r[3];
                c.rc(c.r[4]);
                if c.crbit(2) { pc = 0x80135c6c; continue; }
                pc = 0x80135c60;
            }
            0x80135c60 => {
                c.r[3] = c.lwz(c.r[4]);
                c.r[4] = c.lwz(c.r[4].wrapping_add(0x4));
                if true { return; }
                pc = 0x80135c6c;
            }
            0x80135c6c => {
                c.r[4] = 0x0;
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80135c58: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosGetSavernd (0x80135c78, 24 bytes)
#[inline(never)]
pub fn f_80135c78(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135c78) { return; }
        if c.prof { crate::prof::enter(c, 0x80135c78); f_80135c78_b(c, 0x80135c78); crate::prof::exit(c, 0x80135c78); return; }
    }
    f_80135c78_b(c, 0x80135c78)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135c78_at(c: &mut Cpu, pc: u32) { f_80135c78_b(c, pc) }
#[inline(always)]
fn f_80135c78_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135c78;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135c78, pc); }
        match pc {
            0x80135c78 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80135c88; continue; }
                pc = 0x80135c80;
            }
            0x80135c80 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0xc));
                if true { return; }
                pc = 0x80135c88;
            }
            0x80135c88 => {
                c.r[3] = 0x0;
                if true { return; }
                return;
            }
            _ => panic!("f_80135c78: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosSetPtr (0x80135c90, 64 bytes)
#[inline(never)]
pub fn f_80135c90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135c90) { return; }
        if c.prof { crate::prof::enter(c, 0x80135c90); f_80135c90_b(c, 0x80135c90); crate::prof::exit(c, 0x80135c90); return; }
    }
    f_80135c90_b(c, 0x80135c90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135c90_at(c: &mut Cpu, pc: u32) { f_80135c90_b(c, pc) }
#[inline(always)]
fn f_80135c90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135c90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135c90, pc); }
        match pc {
            0x80135c90 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135c98;
            }
            0x80135c98 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x80135ca4; continue; }
                pc = 0x80135ca0;
            }
            0x80135ca0 => {
                if true { return; }
                pc = 0x80135ca4;
            }
            0x80135ca4 => {
                { let ea = c.r[4]; c.f[1] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x8); c.f[0] = c.lfd(ea); }
                { let ea = c.r[3].wrapping_add(0x8); let v = c.f[1]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x10); let v = c.f[0]; c.stfd(ea, v); }
                { let ea = c.r[4].wrapping_add(0x10); c.f[1] = c.lfd(ea); }
                { let ea = c.r[4].wrapping_add(0x18); c.f[0] = c.lfd(ea); }
                { let ea = c.r[3].wrapping_add(0x18); let v = c.f[1]; c.stfd(ea, v); }
                { let ea = c.r[3].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                { let ea = c.r[4].wrapping_add(0x20); c.f[0] = c.lfd(ea); }
                { let ea = c.r[3].wrapping_add(0x28); let v = c.f[0]; c.stfd(ea, v); }
                if true { return; }
                return;
            }
            _ => panic!("f_80135c90: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveBiosGetPtr (0x80135cd0, 24 bytes)
#[inline(never)]
pub fn f_80135cd0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135cd0) { return; }
        if c.prof { crate::prof::enter(c, 0x80135cd0); f_80135cd0_b(c, 0x80135cd0); crate::prof::exit(c, 0x80135cd0); return; }
    }
    f_80135cd0_b(c, 0x80135cd0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135cd0_at(c: &mut Cpu, pc: u32) { f_80135cd0_b(c, pc) }
#[inline(always)]
fn f_80135cd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135cd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135cd0, pc); }
        match pc {
            0x80135cd0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135ce0; continue; }
                pc = 0x80135cd8;
            }
            0x80135cd8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80135ce0;
            }
            0x80135ce0 => {
                c.r[3] = c.r[3].wrapping_add(0x8);
                if true { return; }
                return;
            }
            _ => panic!("f_80135cd0: bad pc {:#x}", pc),
        }
    }
}

/// gamedatasaveInit (0x80135ce8, 40 bytes)
#[inline(never)]
pub fn f_80135ce8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135ce8) { return; }
        if c.prof { crate::prof::enter(c, 0x80135ce8); f_80135ce8_b(c, 0x80135ce8); crate::prof::exit(c, 0x80135ce8); return; }
    }
    f_80135ce8_b(c, 0x80135ce8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135ce8_at(c: &mut Cpu, pc: u32) { f_80135ce8_b(c, pc) }
#[inline(always)]
fn f_80135ce8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135ce8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135ce8, pc); }
        match pc {
            0x80135ce8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80135cf8; continue; }
                pc = 0x80135cf0;
            }
            0x80135cf0 => {
                c.r[3] = 0x0;
                pc = 0x80135cfc; continue;
                pc = 0x80135cf8;
            }
            0x80135cf8 => {
                c.r[3] = c.r[3].wrapping_add(0x8);
                pc = 0x80135cfc;
            }
            0x80135cfc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x80135d04;
            }
            0x80135d04 => {
                c.r[0] = 0x0;
                c.stw(c.r[3].wrapping_add(0x8), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_80135ce8: bad pc {:#x}", pc),
        }
    }
}

/// statusSetStatus (0x80135d10, 308 bytes)
#[inline(never)]
pub fn f_80135d10(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135d10) { return; }
        if c.prof { crate::prof::enter(c, 0x80135d10); f_80135d10_b(c, 0x80135d10); crate::prof::exit(c, 0x80135d10); return; }
    }
    f_80135d10_b(c, 0x80135d10)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135d10_at(c: &mut Cpu, pc: u32) { f_80135d10_b(c, pc) }
#[inline(always)]
fn f_80135d10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135d10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135d10, pc); }
        match pc {
            0x80135d10 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x80135d14;
            }
            0x80135d14 => {
                c.r[0] = c.mfspr(8);
                pc = 0x80135d18;
            }
            0x80135d18 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                pc = 0x80135d1c;
            }
            0x80135d1c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80135d20;
            }
            0x80135d20 => {
                c.cmpu(0, c.r[0], 0x9);
                pc = 0x80135d24;
            }
            0x80135d24 => {
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                pc = 0x80135d28;
            }
            0x80135d28 => {
                c.r[31] = 0x0;
                pc = 0x80135d2c;
            }
            0x80135d2c => {
                if c.crbit(1) { pc = 0x80135e2c; continue; }
                pc = 0x80135d30;
            }
            0x80135d30 => {
                c.r[3] = 0x80360000;
                pc = 0x80135d34;
            }
            0x80135d34 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80135d38;
            }
            0x80135d38 => {
                c.r[3] = c.r[3].wrapping_add(0x3ac8);
                pc = 0x80135d3c;
            }
            0x80135d3c => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x80135d40;
            }
            0x80135d40 => {
                c.mtspr(9, c.r[0]);
                pc = 0x80135d44;
            }
            0x80135d44 => {
                if true { let t = c.ctr; if (0x80135d10..0x80135e44).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80135d48;
            }
            0x80135d48 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135d4c;
            }
            0x80135d4c => {
                c.r[4] = c.r[6] | c.r[6];
                pc = 0x80135d50;
            }
            0x80135d50 => {
                c.r[5] = c.r[8] | c.r[8];
                pc = 0x80135d54;
            }
            0x80135d54 => {
                c.lr = 0x80135d58; f_8013583c(c);
                pc = 0x80135d58;
            }
            0x80135d58 => {
                pc = 0x80135e2c; continue;
                pc = 0x80135d5c;
            }
            0x80135d5c => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135d60;
            }
            0x80135d60 => {
                c.r[4] = c.r[6] | c.r[6];
                pc = 0x80135d64;
            }
            0x80135d64 => {
                c.r[5] = c.r[8] | c.r[8];
                pc = 0x80135d68;
            }
            0x80135d68 => {
                c.lr = 0x80135d6c; f_80135024(c);
                pc = 0x80135d6c;
            }
            0x80135d6c => {
                pc = 0x80135e2c; continue;
                pc = 0x80135d70;
            }
            0x80135d70 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135d74;
            }
            0x80135d74 => {
                c.r[4] = c.r[6] | c.r[6];
                pc = 0x80135d78;
            }
            0x80135d78 => {
                c.r[5] = c.r[8] | c.r[8];
                pc = 0x80135d7c;
            }
            0x80135d7c => {
                c.lr = 0x80135d80; f_8012a450(c);
                pc = 0x80135d80;
            }
            0x80135d80 => {
                pc = 0x80135e2c; continue;
                pc = 0x80135d84;
            }
            0x80135d84 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135d88;
            }
            0x80135d88 => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135d8c;
            }
            0x80135d8c => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135d90;
            }
            0x80135d90 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135d94;
            }
            0x80135d94 => {
                c.r[7] = c.r[8] | c.r[8];
                pc = 0x80135d98;
            }
            0x80135d98 => {
                c.lr = 0x80135d9c; f_80142b24(c);
                pc = 0x80135d9c;
            }
            0x80135d9c => {
                pc = 0x80135e2c; continue;
                pc = 0x80135da0;
            }
            0x80135da0 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135da4;
            }
            0x80135da4 => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135da8;
            }
            0x80135da8 => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135dac;
            }
            0x80135dac => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135db0;
            }
            0x80135db0 => {
                c.r[7] = c.r[8] | c.r[8];
                pc = 0x80135db4;
            }
            0x80135db4 => {
                c.lr = 0x80135db8; f_801254b4(c);
                pc = 0x80135db8;
            }
            0x80135db8 => {
                pc = 0x80135e2c; continue;
                pc = 0x80135dbc;
            }
            0x80135dbc => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135dc0;
            }
            0x80135dc0 => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135dc4;
            }
            0x80135dc4 => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135dc8;
            }
            0x80135dc8 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135dcc;
            }
            0x80135dcc => {
                c.r[7] = c.r[8] | c.r[8];
                pc = 0x80135dd0;
            }
            0x80135dd0 => {
                c.lr = 0x80135dd4; f_8011bbd8(c);
                pc = 0x80135dd4;
            }
            0x80135dd4 => {
                pc = 0x80135e2c; continue;
                pc = 0x80135dd8;
            }
            0x80135dd8 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135ddc;
            }
            0x80135ddc => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135de0;
            }
            0x80135de0 => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135de4;
            }
            0x80135de4 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135de8;
            }
            0x80135de8 => {
                c.r[7] = c.r[8] | c.r[8];
                pc = 0x80135dec;
            }
            0x80135dec => {
                c.lr = 0x80135df0; f_801f4c14(c);
                pc = 0x80135df0;
            }
            0x80135df0 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x80135df4;
            }
            0x80135df4 => {
                pc = 0x80135e2c; continue;
                pc = 0x80135df8;
            }
            0x80135df8 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135dfc;
            }
            0x80135dfc => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135e00;
            }
            0x80135e00 => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135e04;
            }
            0x80135e04 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135e08;
            }
            0x80135e08 => {
                c.r[7] = c.r[8] | c.r[8];
                pc = 0x80135e0c;
            }
            0x80135e0c => {
                c.lr = 0x80135e10; f_801f75f8(c);
                pc = 0x80135e10;
            }
            0x80135e10 => {
                pc = 0x80135e2c; continue;
                pc = 0x80135e14;
            }
            0x80135e14 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135e18;
            }
            0x80135e18 => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135e1c;
            }
            0x80135e1c => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135e20;
            }
            0x80135e20 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135e24;
            }
            0x80135e24 => {
                c.r[7] = c.r[8] | c.r[8];
                pc = 0x80135e28;
            }
            0x80135e28 => {
                c.lr = 0x80135e2c; f_801faa58(c);
                pc = 0x80135e2c;
            }
            0x80135e2c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x80135e30;
            }
            0x80135e30 => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80135e34;
            }
            0x80135e34 => {
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                pc = 0x80135e38;
            }
            0x80135e38 => {
                c.mtspr(8, c.r[0]);
                pc = 0x80135e3c;
            }
            0x80135e3c => {
                c.r[1] = c.r[1].wrapping_add(0x10);
                pc = 0x80135e40;
            }
            0x80135e40 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80135d10: bad pc {:#x}", pc),
        }
    }
}

/// statusGetStatus (0x80135e44, 276 bytes)
#[inline(never)]
pub fn f_80135e44(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135e44) { return; }
        if c.prof { crate::prof::enter(c, 0x80135e44); f_80135e44_b(c, 0x80135e44); crate::prof::exit(c, 0x80135e44); return; }
    }
    f_80135e44_b(c, 0x80135e44)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135e44_at(c: &mut Cpu, pc: u32) { f_80135e44_b(c, pc) }
#[inline(always)]
fn f_80135e44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135e44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135e44, pc); }
        match pc {
            0x80135e44 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x80135e48;
            }
            0x80135e48 => {
                c.r[0] = c.mfspr(8);
                pc = 0x80135e4c;
            }
            0x80135e4c => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                pc = 0x80135e50;
            }
            0x80135e50 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                pc = 0x80135e54;
            }
            0x80135e54 => {
                c.cmpu(0, c.r[0], 0x9);
                pc = 0x80135e58;
            }
            0x80135e58 => {
                if c.crbit(1) { pc = 0x80135f44; continue; }
                pc = 0x80135e5c;
            }
            0x80135e5c => {
                c.r[3] = 0x80360000;
                pc = 0x80135e60;
            }
            0x80135e60 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80135e64;
            }
            0x80135e64 => {
                c.r[3] = c.r[3].wrapping_add(0x3af0);
                pc = 0x80135e68;
            }
            0x80135e68 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x80135e6c;
            }
            0x80135e6c => {
                c.mtspr(9, c.r[0]);
                pc = 0x80135e70;
            }
            0x80135e70 => {
                if true { let t = c.ctr; if (0x80135e44..0x80135f58).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80135e74;
            }
            0x80135e74 => {
                c.r[3] = 0x0;
                pc = 0x80135e78;
            }
            0x80135e78 => {
                pc = 0x80135f48; continue;
                pc = 0x80135e7c;
            }
            0x80135e7c => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135e80;
            }
            0x80135e80 => {
                c.r[4] = c.r[6] | c.r[6];
                pc = 0x80135e84;
            }
            0x80135e84 => {
                c.lr = 0x80135e88; f_80135938(c);
                pc = 0x80135e88;
            }
            0x80135e88 => {
                pc = 0x80135f48; continue;
                pc = 0x80135e8c;
            }
            0x80135e8c => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135e90;
            }
            0x80135e90 => {
                c.r[4] = c.r[6] | c.r[6];
                pc = 0x80135e94;
            }
            0x80135e94 => {
                c.r[5] = c.r[7] | c.r[7];
                pc = 0x80135e98;
            }
            0x80135e98 => {
                c.lr = 0x80135e9c; f_80135028(c);
                pc = 0x80135e9c;
            }
            0x80135e9c => {
                pc = 0x80135f48; continue;
                pc = 0x80135ea0;
            }
            0x80135ea0 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135ea4;
            }
            0x80135ea4 => {
                c.r[4] = c.r[6] | c.r[6];
                pc = 0x80135ea8;
            }
            0x80135ea8 => {
                c.r[5] = c.r[7] | c.r[7];
                pc = 0x80135eac;
            }
            0x80135eac => {
                c.lr = 0x80135eb0; f_8012a5b0(c);
                pc = 0x80135eb0;
            }
            0x80135eb0 => {
                pc = 0x80135f48; continue;
                pc = 0x80135eb4;
            }
            0x80135eb4 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135eb8;
            }
            0x80135eb8 => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135ebc;
            }
            0x80135ebc => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135ec0;
            }
            0x80135ec0 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135ec4;
            }
            0x80135ec4 => {
                c.lr = 0x80135ec8; f_80142cf4(c);
                pc = 0x80135ec8;
            }
            0x80135ec8 => {
                pc = 0x80135f48; continue;
                pc = 0x80135ecc;
            }
            0x80135ecc => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135ed0;
            }
            0x80135ed0 => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135ed4;
            }
            0x80135ed4 => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135ed8;
            }
            0x80135ed8 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135edc;
            }
            0x80135edc => {
                c.lr = 0x80135ee0; f_8012640c(c);
                pc = 0x80135ee0;
            }
            0x80135ee0 => {
                pc = 0x80135f48; continue;
                pc = 0x80135ee4;
            }
            0x80135ee4 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135ee8;
            }
            0x80135ee8 => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135eec;
            }
            0x80135eec => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135ef0;
            }
            0x80135ef0 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135ef4;
            }
            0x80135ef4 => {
                c.lr = 0x80135ef8; f_8011beb4(c);
                pc = 0x80135ef8;
            }
            0x80135ef8 => {
                pc = 0x80135f48; continue;
                pc = 0x80135efc;
            }
            0x80135efc => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135f00;
            }
            0x80135f00 => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135f04;
            }
            0x80135f04 => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135f08;
            }
            0x80135f08 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135f0c;
            }
            0x80135f0c => {
                c.lr = 0x80135f10; f_801f54a4(c);
                pc = 0x80135f10;
            }
            0x80135f10 => {
                pc = 0x80135f48; continue;
                pc = 0x80135f14;
            }
            0x80135f14 => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135f18;
            }
            0x80135f18 => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135f1c;
            }
            0x80135f1c => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135f20;
            }
            0x80135f20 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135f24;
            }
            0x80135f24 => {
                c.lr = 0x80135f28; f_801f76b8(c);
                pc = 0x80135f28;
            }
            0x80135f28 => {
                pc = 0x80135f48; continue;
                pc = 0x80135f2c;
            }
            0x80135f2c => {
                c.r[3] = c.r[4] | c.r[4];
                pc = 0x80135f30;
            }
            0x80135f30 => {
                c.r[4] = c.r[5] | c.r[5];
                pc = 0x80135f34;
            }
            0x80135f34 => {
                c.r[5] = c.r[6] | c.r[6];
                pc = 0x80135f38;
            }
            0x80135f38 => {
                c.r[6] = c.r[7] | c.r[7];
                pc = 0x80135f3c;
            }
            0x80135f3c => {
                c.lr = 0x80135f40; f_801fb1c0(c);
                pc = 0x80135f40;
            }
            0x80135f40 => {
                pc = 0x80135f48; continue;
                pc = 0x80135f44;
            }
            0x80135f44 => {
                c.r[3] = 0x0;
                pc = 0x80135f48;
            }
            0x80135f48 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x80135f4c;
            }
            0x80135f4c => {
                c.mtspr(8, c.r[0]);
                pc = 0x80135f50;
            }
            0x80135f50 => {
                c.r[1] = c.r[1].wrapping_add(0x10);
                pc = 0x80135f54;
            }
            0x80135f54 => {
                if true { return; }
                return;
            }
            _ => panic!("f_80135e44: bad pc {:#x}", pc),
        }
    }
}

/// koukaLinkDataBiosGetKouka (0x80135f58, 56 bytes)
#[inline(never)]
pub fn f_80135f58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135f58) { return; }
        if c.prof { crate::prof::enter(c, 0x80135f58); f_80135f58_b(c, 0x80135f58); crate::prof::exit(c, 0x80135f58); return; }
    }
    f_80135f58_b(c, 0x80135f58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135f58_at(c: &mut Cpu, pc: u32) { f_80135f58_b(c, pc) }
#[inline(always)]
fn f_80135f58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135f58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135f58, pc); }
        match pc {
            0x80135f58 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8370));
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(1) { pc = 0x80135f6c; continue; }
                pc = 0x80135f64;
            }
            0x80135f64 => {
                c.cmpu(0, c.r[4], 0x8);
                if !c.crbit(1) { pc = 0x80135f74; continue; }
                pc = 0x80135f6c;
            }
            0x80135f6c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80135f74;
            }
            0x80135f74 => {
                c.r[5] = 0x80360000;
                c.r[6] = c.r[3].rotate_left(4) & 0xfffffff0;
                c.r[3] = c.r[5].wrapping_add(0x3b78);
                c.r[0] = c.r[4].rotate_left(1) & 0xfffffffe;
                c.r[3] = c.r[3].wrapping_add(c.r[6]);
                c.r[3] = c.lhz(c.r[3].wrapping_add(c.r[0]));
                if true { return; }
                return;
            }
            _ => panic!("f_80135f58: bad pc {:#x}", pc),
        }
    }
}

/// koukaDataBiosGetLink (0x80135f90, 44 bytes)
#[inline(never)]
pub fn f_80135f90(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135f90) { return; }
        if c.prof { crate::prof::enter(c, 0x80135f90); f_80135f90_b(c, 0x80135f90); crate::prof::exit(c, 0x80135f90); return; }
    }
    f_80135f90_b(c, 0x80135f90)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135f90_at(c: &mut Cpu, pc: u32) { f_80135f90_b(c, pc) }
#[inline(always)]
fn f_80135f90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135f90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135f90, pc); }
        match pc {
            0x80135f90 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8368));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x80135fa4; continue; }
                pc = 0x80135f9c;
            }
            0x80135f9c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80135fa4;
            }
            0x80135fa4 => {
                c.r[0] = c.r[3].wrapping_mul(0xa);
                c.r[3] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x3b18);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_80135f90: bad pc {:#x}", pc),
        }
    }
}

/// koukaDataBiosGetValue (0x80135fbc, 60 bytes)
#[inline(never)]
pub fn f_80135fbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135fbc) { return; }
        if c.prof { crate::prof::enter(c, 0x80135fbc); f_80135fbc_b(c, 0x80135fbc); crate::prof::exit(c, 0x80135fbc); return; }
    }
    f_80135fbc_b(c, 0x80135fbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135fbc_at(c: &mut Cpu, pc: u32) { f_80135fbc_b(c, pc) }
#[inline(always)]
fn f_80135fbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135fbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135fbc, pc); }
        match pc {
            0x80135fbc => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8368));
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(1) { pc = 0x80135fd0; continue; }
                pc = 0x80135fc8;
            }
            0x80135fc8 => {
                c.cmpu(0, c.r[4], 0x2);
                if !c.crbit(1) { pc = 0x80135fd8; continue; }
                pc = 0x80135fd0;
            }
            0x80135fd0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80135fd8;
            }
            0x80135fd8 => {
                c.r[3] = c.r[3].wrapping_mul(0xa);
                c.r[5] = 0x80360000;
                c.r[0] = c.r[4].rotate_left(1) & 0xfffffffe;
                c.r[4] = c.r[5].wrapping_add(0x3b18);
                c.r[3] = c.r[4].wrapping_add(c.r[3]);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lha(c.r[3].wrapping_add(0x6));
                if true { return; }
                return;
            }
            _ => panic!("f_80135fbc: bad pc {:#x}", pc),
        }
    }
}

/// koukaDataBiosGetVar (0x80135ff8, 44 bytes)
#[inline(never)]
pub fn f_80135ff8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80135ff8) { return; }
        if c.prof { crate::prof::enter(c, 0x80135ff8); f_80135ff8_b(c, 0x80135ff8); crate::prof::exit(c, 0x80135ff8); return; }
    }
    f_80135ff8_b(c, 0x80135ff8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80135ff8_at(c: &mut Cpu, pc: u32) { f_80135ff8_b(c, pc) }
#[inline(always)]
fn f_80135ff8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80135ff8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80135ff8, pc); }
        match pc {
            0x80135ff8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8368));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x8013600c; continue; }
                pc = 0x80136004;
            }
            0x80136004 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8013600c;
            }
            0x8013600c => {
                c.r[0] = c.r[3].wrapping_mul(0xa);
                c.r[3] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x3b18);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_80135ff8: bad pc {:#x}", pc),
        }
    }
}

/// koukaDataBiosGetStatus (0x80136024, 44 bytes)
#[inline(never)]
pub fn f_80136024(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80136024) { return; }
        if c.prof { crate::prof::enter(c, 0x80136024); f_80136024_b(c, 0x80136024); crate::prof::exit(c, 0x80136024); return; }
    }
    f_80136024_b(c, 0x80136024)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80136024_at(c: &mut Cpu, pc: u32) { f_80136024_b(c, pc) }
#[inline(always)]
fn f_80136024_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80136024;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80136024, pc); }
        match pc {
            0x80136024 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8368));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x80136038; continue; }
                pc = 0x80136030;
            }
            0x80136030 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80136038;
            }
            0x80136038 => {
                c.r[0] = c.r[3].wrapping_mul(0xa);
                c.r[3] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x3b18);
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_80136024: bad pc {:#x}", pc),
        }
    }
}

/// koukaDataBiosGetStatusKind (0x80136050, 40 bytes)
#[inline(never)]
pub fn f_80136050(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80136050) { return; }
        if c.prof { crate::prof::enter(c, 0x80136050); f_80136050_b(c, 0x80136050); crate::prof::exit(c, 0x80136050); return; }
    }
    f_80136050_b(c, 0x80136050)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80136050_at(c: &mut Cpu, pc: u32) { f_80136050_b(c, pc) }
#[inline(always)]
fn f_80136050_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80136050;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80136050, pc); }
        match pc {
            0x80136050 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8368));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x80136064; continue; }
                pc = 0x8013605c;
            }
            0x8013605c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80136064;
            }
            0x80136064 => {
                c.r[0] = c.r[3].wrapping_mul(0xa);
                c.r[3] = 0x80360000;
                c.r[3] = c.r[3].wrapping_add(0x3b18);
                c.r[3] = c.lbz(c.r[3].wrapping_add(c.r[0]));
                if true { return; }
                return;
            }
            _ => panic!("f_80136050: bad pc {:#x}", pc),
        }
    }
}

/// koukaExec (0x80136078, 196 bytes)
#[inline(never)]
pub fn f_80136078(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80136078) { return; }
        if c.prof { crate::prof::enter(c, 0x80136078); f_80136078_b(c, 0x80136078); crate::prof::exit(c, 0x80136078); return; }
    }
    f_80136078_b(c, 0x80136078)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80136078_at(c: &mut Cpu, pc: u32) { f_80136078_b(c, pc) }
#[inline(always)]
fn f_80136078_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80136078;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80136078, pc); }
        match pc {
            0x80136078 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[6] | c.r[6];
                c.rc(c.r[29]);
                c.r[26] = c.r[3] | c.r[3];
                c.r[27] = c.r[4] | c.r[4];
                c.r[28] = c.r[5] | c.r[5];
                if c.crbit(2) { pc = 0x801360a4; continue; }
                pc = 0x8013609c;
            }
            0x8013609c => {
                c.lr = 0x801360a0; f_8013613c(c);
                pc = 0x801360a0;
            }
            0x801360a0 => {
                pc = 0x801360ac; continue;
                pc = 0x801360a4;
            }
            0x801360a4 => {
                c.r[6] = 0x0;
                c.lr = 0x801360ac; f_8013613c(c);
                pc = 0x801360ac;
            }
            0x801360ac => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x801360b4; f_80135f90(c);
                pc = 0x801360b4;
            }
            0x801360b4 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x80136128; continue; }
                pc = 0x801360bc;
            }
            0x801360bc => {
                c.r[30] = 0x0;
                pc = 0x8013611c; continue;
                pc = 0x801360c4;
            }
            0x801360c4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.lr = 0x801360d0; f_80135f58(c);
                pc = 0x801360d0;
            }
            0x801360d0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if c.crbit(2) { pc = 0x80136118; continue; }
                pc = 0x801360d8;
            }
            0x801360d8 => {
                c.cmpu(0, c.r[29], 0x0);
                if c.crbit(2) { pc = 0x80136104; continue; }
                pc = 0x801360e0;
            }
            0x801360e0 => {
                c.r[4] = c.r[30].rotate_left(0) & 0xffff;
                c.r[3] = c.r[26] | c.r[26];
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.r[4] = c.r[27] | c.r[27];
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = c.r[29].wrapping_add(c.r[0]);
                c.lr = 0x80136100; f_8013613c(c);
                pc = 0x80136100;
            }
            0x80136100 => {
                pc = 0x80136118; continue;
                pc = 0x80136104;
            }
            0x80136104 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[27] | c.r[27];
                c.r[5] = c.r[28] | c.r[28];
                c.r[6] = 0x0;
                c.lr = 0x80136118; f_8013613c(c);
                pc = 0x80136118;
            }
            0x80136118 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x8013611c;
            }
            0x8013611c => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x8);
                if c.crbit(0) { pc = 0x801360c4; continue; }
                pc = 0x80136128;
            }
            0x80136128 => {
                { let mut ea = c.r[1].wrapping_add(0x8); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80136078: bad pc {:#x}", pc),
        }
    }
}

/// _koukaOneExec__FUlPvPvPl (0x8013613c, 556 bytes)
#[inline(never)]
pub fn f_8013613c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013613c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013613c); f_8013613c_b(c, 0x8013613c); crate::prof::exit(c, 0x8013613c); return; }
    }
    f_8013613c_b(c, 0x8013613c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013613c_at(c: &mut Cpu, pc: u32) { f_8013613c_b(c, pc) }
#[inline(always)]
fn f_8013613c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013613c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013613c, pc); }
        match pc {
            0x8013613c => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[23] = c.r[3] | c.r[3];
                c.rc(c.r[23]);
                c.r[26] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[28] = c.r[6] | c.r[6];
                if !c.crbit(2) { pc = 0x80136168; continue; }
                pc = 0x80136160;
            }
            0x80136160 => {
                c.r[3] = 0x0;
                pc = 0x80136354; continue;
                pc = 0x80136168;
            }
            0x80136168 => {
                c.lr = 0x8013616c; f_80136050(c);
                pc = 0x8013616c;
            }
            0x8013616c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.r[29] = c.r[0] | c.r[0];
                c.lr = 0x8013617c; f_80136024(c);
                pc = 0x8013617c;
            }
            0x8013617c => {
                c.r[0] = c.r[3] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.r[30] = c.r[0] | c.r[0];
                c.r[4] = 0x0;
                c.lr = 0x80136190; f_80135fbc(c);
                pc = 0x80136190;
            }
            0x80136190 => {
                c.r[24] = c.r[3] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.r[4] = 0x1;
                c.lr = 0x801361a0; f_80135fbc(c);
                pc = 0x801361a0;
            }
            0x801361a0 => {
                c.r[25] = c.r[3] | c.r[3];
                c.r[3] = c.r[23] | c.r[23];
                c.lr = 0x801361ac; f_80135ff8(c);
                pc = 0x801361ac;
            }
            0x801361ac => {
                c.r[23] = c.r[3].rotate_left(0) & 0xff;
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[26] | c.r[26];
                c.r[6] = c.r[30] | c.r[30];
                c.r[7] = c.r[24].rotate_left(0) & 0xffff;
                c.r[5] = 0x0;
                c.lr = 0x801361c8; f_80135e44(c);
                pc = 0x801361c8;
            }
            0x801361c8 => {
                c.cmpu(0, c.r[23], 0x0);
                c.r[31] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x801361e4; continue; }
                pc = 0x801361d4;
            }
            0x801361d4 => {
                c.cmpu(0, c.r[23], 0x2);
                if c.crbit(2) { pc = 0x801361e4; continue; }
                pc = 0x801361dc;
            }
            0x801361dc => {
                c.cmpu(0, c.r[23], 0x3);
                if !c.crbit(2) { pc = 0x8013625c; continue; }
                pc = 0x801361e4;
            }
            0x801361e4 => {
                c.r[0] = c.r[24] as u16 as i16 as i32 as u32;
                c.cmps(0, c.r[0], 0xffffffff);
                if !c.crbit(2) { pc = 0x8013621c; continue; }
                pc = 0x801361f0;
            }
            0x801361f0 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[26] | c.r[26];
                c.r[6] = c.r[25].rotate_left(0) & 0xffff;
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x80136208; f_80135e44(c);
                pc = 0x80136208;
            }
            0x80136208 => {
                c.r[0] = c.r[3].rotate_left(1) & 0x1;
                c.r[0] = c.r[0].wrapping_add(c.r[3]);
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 31 != 0); c.r[0] = (v >> 1) as u32; }
                c.r[24] = c.r[0] as u16 as i16 as i32 as u32;
                pc = 0x8013625c; continue;
                pc = 0x8013621c;
            }
            0x8013621c => {
                c.cmps(0, c.r[0], 0xfffffffe);
                if !c.crbit(2) { pc = 0x80136244; continue; }
                pc = 0x80136224;
            }
            0x80136224 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[26] | c.r[26];
                c.r[6] = c.r[25].rotate_left(0) & 0xffff;
                c.r[5] = 0x0;
                c.r[7] = 0x0;
                c.lr = 0x8013623c; f_80135e44(c);
                pc = 0x8013623c;
            }
            0x8013623c => {
                c.r[24] = c.r[3] as u16 as i16 as i32 as u32;
                pc = 0x8013625c; continue;
                pc = 0x80136244;
            }
            0x80136244 => {
                c.r[0] = c.r[24] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if c.crbit(0) { pc = 0x80136254; continue; }
                pc = 0x8013624c;
            }
            0x8013624c => {
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.rc(c.r[0]);
                if !c.crbit(0) { pc = 0x8013625c; continue; }
                pc = 0x80136254;
            }
            0x80136254 => {
                c.r[3] = 0x0;
                pc = 0x80136354; continue;
                pc = 0x8013625c;
            }
            0x8013625c => {
                c.cmps(0, c.r[23], 0x3);
                if c.crbit(2) { pc = 0x801362b8; continue; }
                pc = 0x80136264;
            }
            0x80136264 => {
                if !c.crbit(0) { pc = 0x80136280; continue; }
                pc = 0x80136268;
            }
            0x80136268 => {
                c.cmps(0, c.r[23], 0x1);
                if c.crbit(2) { pc = 0x80136298; continue; }
                pc = 0x80136270;
            }
            0x80136270 => {
                if !c.crbit(0) { pc = 0x801362ac; continue; }
                pc = 0x80136274;
            }
            0x80136274 => {
                c.cmps(0, c.r[23], 0x0);
                if !c.crbit(0) { pc = 0x80136290; continue; }
                pc = 0x8013627c;
            }
            0x8013627c => {
                pc = 0x801362f4; continue;
                pc = 0x80136280;
            }
            0x80136280 => {
                c.cmps(0, c.r[23], 0x5);
                if c.crbit(2) { pc = 0x801362dc; continue; }
                pc = 0x80136288;
            }
            0x80136288 => {
                if !c.crbit(0) { pc = 0x801362f4; continue; }
                pc = 0x8013628c;
            }
            0x8013628c => {
                pc = 0x801362c4; continue;
                pc = 0x80136290;
            }
            0x80136290 => {
                c.r[23] = c.r[24] as u16 as i16 as i32 as u32;
                pc = 0x801362fc; continue;
                pc = 0x80136298;
            }
            0x80136298 => {
                c.r[3] = c.r[24] as u16 as i16 as i32 as u32;
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[31].wrapping_mul(c.r[3]);
                c.r[23] = divw(c.r[3], c.r[0]);
                pc = 0x801362fc; continue;
                pc = 0x801362ac;
            }
            0x801362ac => {
                c.r[0] = c.r[24] as u16 as i16 as i32 as u32;
                c.r[23] = c.r[31].wrapping_add(c.r[0]);
                pc = 0x801362fc; continue;
                pc = 0x801362b8;
            }
            0x801362b8 => {
                c.r[0] = c.r[24] as u16 as i16 as i32 as u32;
                c.r[23] = c.r[31].wrapping_sub(c.r[0]);
                pc = 0x801362fc; continue;
                pc = 0x801362c4;
            }
            0x801362c4 => {
                c.r[3] = c.r[24] as u16 as i16 as i32 as u32;
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[31].wrapping_mul(c.r[3]);
                c.r[0] = divw(c.r[3], c.r[0]);
                c.r[23] = c.r[31].wrapping_add(c.r[0]);
                pc = 0x801362fc; continue;
                pc = 0x801362dc;
            }
            0x801362dc => {
                c.r[3] = c.r[24] as u16 as i16 as i32 as u32;
                c.r[0] = c.r[25] as u16 as i16 as i32 as u32;
                c.r[3] = c.r[31].wrapping_mul(c.r[3]);
                c.r[0] = divw(c.r[3], c.r[0]);
                c.r[23] = c.r[31].wrapping_sub(c.r[0]);
                pc = 0x801362fc; continue;
                pc = 0x801362f4;
            }
            0x801362f4 => {
                c.r[3] = 0x0;
                pc = 0x80136354; continue;
                pc = 0x801362fc;
            }
            0x801362fc => {
                c.cmpu(0, c.r[27], 0x0);
                if c.crbit(2) { pc = 0x8013632c; continue; }
                pc = 0x80136304;
            }
            0x80136304 => {
                c.r[3] = 0x0;
                c.r[4] = 0x0;
                c.r[5] = 0x14;
                c.r[6] = 0x0;
                c.lr = 0x80136318; f_801f54a4(c);
                pc = 0x80136318;
            }
            0x80136318 => {
                c.r[4] = c.r[3].rotate_left(0) & 0xffff;
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x80136324; f_801f0134(c);
                pc = 0x80136324;
            }
            0x80136324 => {
                c.r[7] = c.r[3] | c.r[3];
                pc = 0x80136330; continue;
                pc = 0x8013632c;
            }
            0x8013632c => {
                c.r[7] = 0x0;
                pc = 0x80136330;
            }
            0x80136330 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[26] | c.r[26];
                c.r[6] = c.r[30] | c.r[30];
                c.r[8] = c.r[23] | c.r[23];
                c.r[5] = 0x0;
                c.lr = 0x80136348; f_80135d10(c);
                pc = 0x80136348;
            }
            0x80136348 => {
                c.cmpu(0, c.r[28], 0x0);
                if c.crbit(2) { pc = 0x80136354; continue; }
                pc = 0x80136350;
            }
            0x80136350 => {
                c.stw(c.r[28], c.r[23]);
                pc = 0x80136354;
            }
            0x80136354 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 23..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_8013613c: bad pc {:#x}", pc),
        }
    }
}

/// tenkouDataBiosGetFightInitMsgId (0x80136368, 64 bytes)
#[inline(never)]
pub fn f_80136368(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80136368) { return; }
        if c.prof { crate::prof::enter(c, 0x80136368); f_80136368_b(c, 0x80136368); crate::prof::exit(c, 0x80136368); return; }
    }
    f_80136368_b(c, 0x80136368)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80136368_at(c: &mut Cpu, pc: u32) { f_80136368_b(c, pc) }
#[inline(always)]
fn f_80136368_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80136368;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80136368, pc); }
        match pc {
            0x80136368 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8378));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x80136380; continue; }
                pc = 0x80136378;
            }
            0x80136378 => {
                c.r[3] = 0x0;
                pc = 0x80136390; continue;
                pc = 0x80136380;
            }
            0x80136380 => {
                c.r[4] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0x3b88);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80136390;
            }
            0x80136390 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801363a0; continue; }
                pc = 0x80136398;
            }
            0x80136398 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x801363a0;
            }
            0x801363a0 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_80136368: bad pc {:#x}", pc),
        }
    }
}

/// tenkouDataBiosGetSolarFlag (0x801363a8, 64 bytes)
#[inline(never)]
pub fn f_801363a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801363a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801363a8); f_801363a8_b(c, 0x801363a8); crate::prof::exit(c, 0x801363a8); return; }
    }
    f_801363a8_b(c, 0x801363a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801363a8_at(c: &mut Cpu, pc: u32) { f_801363a8_b(c, pc) }
#[inline(always)]
fn f_801363a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801363a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801363a8, pc); }
        match pc {
            0x801363a8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8378));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x801363c0; continue; }
                pc = 0x801363b8;
            }
            0x801363b8 => {
                c.r[3] = 0x0;
                pc = 0x801363d0; continue;
                pc = 0x801363c0;
            }
            0x801363c0 => {
                c.r[4] = c.r[3].wrapping_mul(0x18);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0x3b88);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x801363d0;
            }
            0x801363d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801363e0; continue; }
                pc = 0x801363d8;
            }
            0x801363d8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x801363e0;
            }
            0x801363e0 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_801363a8: bad pc {:#x}", pc),
        }
    }
}

/// tikeiDataBiosGetWazaId (0x801363e8, 64 bytes)
#[inline(never)]
pub fn f_801363e8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801363e8) { return; }
        if c.prof { crate::prof::enter(c, 0x801363e8); f_801363e8_b(c, 0x801363e8); crate::prof::exit(c, 0x801363e8); return; }
    }
    f_801363e8_b(c, 0x801363e8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801363e8_at(c: &mut Cpu, pc: u32) { f_801363e8_b(c, pc) }
#[inline(always)]
fn f_801363e8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801363e8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801363e8, pc); }
        match pc {
            0x801363e8 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8380));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x80136400; continue; }
                pc = 0x801363f8;
            }
            0x801363f8 => {
                c.r[3] = 0x0;
                pc = 0x80136410; continue;
                pc = 0x80136400;
            }
            0x80136400 => {
                c.r[4] = c.r[3].wrapping_mul(0xc);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0x3c00);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80136410;
            }
            0x80136410 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80136420; continue; }
                pc = 0x80136418;
            }
            0x80136418 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80136420;
            }
            0x80136420 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_801363e8: bad pc {:#x}", pc),
        }
    }
}

/// tikeiDataBiosGetZokuseiDataId (0x80136428, 64 bytes)
#[inline(never)]
pub fn f_80136428(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80136428) { return; }
        if c.prof { crate::prof::enter(c, 0x80136428); f_80136428_b(c, 0x80136428); crate::prof::exit(c, 0x80136428); return; }
    }
    f_80136428_b(c, 0x80136428)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80136428_at(c: &mut Cpu, pc: u32) { f_80136428_b(c, pc) }
#[inline(always)]
fn f_80136428_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80136428;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80136428, pc); }
        match pc {
            0x80136428 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8380));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x80136440; continue; }
                pc = 0x80136438;
            }
            0x80136438 => {
                c.r[3] = 0x0;
                pc = 0x80136450; continue;
                pc = 0x80136440;
            }
            0x80136440 => {
                c.r[4] = c.r[3].wrapping_mul(0xc);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0x3c00);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80136450;
            }
            0x80136450 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80136460; continue; }
                pc = 0x80136458;
            }
            0x80136458 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x80136460;
            }
            0x80136460 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_80136428: bad pc {:#x}", pc),
        }
    }
}

/// tikeiDataBiosGetFightKoukaId (0x80136468, 64 bytes)
#[inline(never)]
pub fn f_80136468(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80136468) { return; }
        if c.prof { crate::prof::enter(c, 0x80136468); f_80136468_b(c, 0x80136468); crate::prof::exit(c, 0x80136468); return; }
    }
    f_80136468_b(c, 0x80136468)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80136468_at(c: &mut Cpu, pc: u32) { f_80136468_b(c, pc) }
#[inline(always)]
fn f_80136468_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80136468;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80136468, pc); }
        match pc {
            0x80136468 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8380));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(1) { pc = 0x80136480; continue; }
                pc = 0x80136478;
            }
            0x80136478 => {
                c.r[3] = 0x0;
                pc = 0x80136490; continue;
                pc = 0x80136480;
            }
            0x80136480 => {
                c.r[4] = c.r[3].wrapping_mul(0xc);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0x3c00);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                pc = 0x80136490;
            }
            0x80136490 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x801364a0; continue; }
                pc = 0x80136498;
            }
            0x80136498 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x801364a0;
            }
            0x801364a0 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_80136468: bad pc {:#x}", pc),
        }
    }
}

/// fn_801364A8 (0x801364a8, 3180 bytes)
#[inline(never)]
pub fn f_801364a8(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801364a8) { return; }
        if c.prof { crate::prof::enter(c, 0x801364a8); f_801364a8_b(c, 0x801364a8); crate::prof::exit(c, 0x801364a8); return; }
    }
    f_801364a8_b(c, 0x801364a8)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801364a8_at(c: &mut Cpu, pc: u32) { f_801364a8_b(c, pc) }
#[inline(always)]
fn f_801364a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801364a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801364a8, pc); }
        match pc {
            0x801364a8 => {
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                pc = 0x801364ac;
            }
            0x801364ac => {
                c.r[0] = c.mfspr(8);
                pc = 0x801364b0;
            }
            0x801364b0 => {
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                pc = 0x801364b4;
            }
            0x801364b4 => {
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[31]; c.stfd(ea, v); }
                pc = 0x801364b8;
            }
            0x801364b8 => {
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                pc = 0x801364bc;
            }
            0x801364bc => {
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[30]; c.stfd(ea, v); }
                pc = 0x801364c0;
            }
            0x801364c0 => {
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                pc = 0x801364c4;
            }
            0x801364c4 => {
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                pc = 0x801364c8;
            }
            0x801364c8 => {
                c.r[29] = c.r[4] | c.r[4];
                pc = 0x801364cc;
            }
            0x801364cc => {
                c.r[28] = c.r[3] | c.r[3];
                pc = 0x801364d0;
            }
            0x801364d0 => {
                c.r[30] = c.lwz(c.r[29].wrapping_add(0x4));
                pc = 0x801364d4;
            }
            0x801364d4 => {
                c.r[27] = c.r[29] | c.r[29];
                pc = 0x801364d8;
            }
            0x801364d8 => {
                c.r[4] = 0x0;
                pc = 0x801364dc;
            }
            0x801364dc => {
                c.r[5] = 0xd8;
                pc = 0x801364e0;
            }
            0x801364e0 => {
                c.r[29] = c.r[29].wrapping_add(0xc);
                pc = 0x801364e4;
            }
            0x801364e4 => {
                c.lr = 0x801364e8; f_8000540c(c);
                pc = 0x801364e8;
            }
            0x801364e8 => {
                c.r[0] = c.lwz(c.r[27]);
                pc = 0x801364ec;
            }
            0x801364ec => {
                c.stw(c.r[28], c.r[0]);
                pc = 0x801364f0;
            }
            0x801364f0 => {
                c.lr = 0x801364f4; f_800d37cc(c);
                pc = 0x801364f4;
            }
            0x801364f4 => {
                c.r[4] = 0x43300000;
                pc = 0x801364f8;
            }
            0x801364f8 => {
                c.r[3] = c.r[3] ^ 0x80000000;
                pc = 0x801364fc;
            }
            0x801364fc => {
                c.r[0] = c.r[30] ^ 0x80000000;
                pc = 0x80136500;
            }
            0x80136500 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                pc = 0x80136504;
            }
            0x80136504 => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                pc = 0x80136508;
            }
            0x80136508 => {
                c.stw(c.r[1].wrapping_add(0x10), c.r[4]);
                pc = 0x8013650c;
            }
            0x8013650c => {
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136510;
            }
            0x80136510 => {
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfd(ea); }
                pc = 0x80136514;
            }
            0x80136514 => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[0]);
                pc = 0x80136518;
            }
            0x80136518 => {
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                pc = 0x8013651c;
            }
            0x8013651c => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[4]);
                pc = 0x80136520;
            }
            0x80136520 => {
                { let ea = c.r[1].wrapping_add(0x18); c.f[1] = c.lfd(ea); }
                pc = 0x80136524;
            }
            0x80136524 => {
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x80136528;
            }
            0x80136528 => {
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x8013652c;
            }
            0x8013652c => {
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80136530;
            }
            0x80136530 => {
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x80136534;
            }
            0x80136534 => {
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x80136538;
            }
            0x80136538 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x8013653c;
            }
            0x8013653c => {
                c.stw(c.r[28].wrapping_add(0x4), c.r[0]);
                pc = 0x80136540;
            }
            0x80136540 => {
                c.r[0] = c.lwz(c.r[27]);
                pc = 0x80136544;
            }
            0x80136544 => {
                c.cmpu(0, c.r[0], 0xc);
                pc = 0x80136548;
            }
            0x80136548 => {
                if c.crbit(1) { pc = 0x801370ec; continue; }
                pc = 0x8013654c;
            }
            0x8013654c => {
                c.r[3] = 0x80360000;
                pc = 0x80136550;
            }
            0x80136550 => {
                c.r[0] = c.r[0].rotate_left(2) & 0xfffffffc;
                pc = 0x80136554;
            }
            0x80136554 => {
                c.r[3] = c.r[3].wrapping_add(0x3c70);
                pc = 0x80136558;
            }
            0x80136558 => {
                c.r[0] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                pc = 0x8013655c;
            }
            0x8013655c => {
                c.mtspr(9, c.r[0]);
                pc = 0x80136560;
            }
            0x80136560 => {
                if true { let t = c.ctr; if (0x801364a8..0x80137114).contains(&t) { pc = t; continue; } (c.dispatch)(c, t); return; }
                pc = 0x80136564;
            }
            0x80136564 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80136568;
            }
            0x80136568 => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x8013656c;
            }
            0x8013656c => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136570;
            }
            0x80136570 => {
                c.lr = 0x80136574; f_80137780(c);
                pc = 0x80136574;
            }
            0x80136574 => {
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x80136578;
            }
            0x80136578 => {
                pc = 0x801370ec; continue;
                pc = 0x8013657c;
            }
            0x8013657c => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136580;
            }
            0x80136580 => {
                c.r[4] = 0x0;
                pc = 0x80136584;
            }
            0x80136584 => {
                c.r[5] = 0x60;
                pc = 0x80136588;
            }
            0x80136588 => {
                c.lr = 0x8013658c; f_8000540c(c);
                pc = 0x8013658c;
            }
            0x8013658c => {
                c.lr = 0x80136590; f_800d37cc(c);
                pc = 0x80136590;
            }
            0x80136590 => {
                c.r[5] = 0x43300000;
                pc = 0x80136594;
            }
            0x80136594 => {
                c.r[3] = c.r[3] ^ 0x80000000;
                pc = 0x80136598;
            }
            0x80136598 => {
                c.stw(c.r[1].wrapping_add(0x24), c.r[3]);
                pc = 0x8013659c;
            }
            0x8013659c => {
                c.r[0] = c.r[30] ^ 0x80000000;
                pc = 0x801365a0;
            }
            0x801365a0 => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                pc = 0x801365a4;
            }
            0x801365a4 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x801365a8;
            }
            0x801365a8 => {
                c.stw(c.r[1].wrapping_add(0x20), c.r[5]);
                pc = 0x801365ac;
            }
            0x801365ac => {
                c.r[3] = c.r[28].wrapping_add(0x10);
                pc = 0x801365b0;
            }
            0x801365b0 => {
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801365b4;
            }
            0x801365b4 => {
                { let ea = c.r[1].wrapping_add(0x20); c.f[1] = c.lfd(ea); }
                pc = 0x801365b8;
            }
            0x801365b8 => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[0]);
                pc = 0x801365bc;
            }
            0x801365bc => {
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                pc = 0x801365c0;
            }
            0x801365c0 => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[5]);
                pc = 0x801365c4;
            }
            0x801365c4 => {
                { let ea = c.r[1].wrapping_add(0x18); c.f[1] = c.lfd(ea); }
                pc = 0x801365c8;
            }
            0x801365c8 => {
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x801365cc;
            }
            0x801365cc => {
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x801365d0;
            }
            0x801365d0 => {
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x801365d4;
            }
            0x801365d4 => {
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x801365d8;
            }
            0x801365d8 => {
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x801365dc;
            }
            0x801365dc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                pc = 0x801365e0;
            }
            0x801365e0 => {
                c.sth(c.r[28].wrapping_add(0x52), c.r[0]);
                pc = 0x801365e4;
            }
            0x801365e4 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x3c));
                pc = 0x801365e8;
            }
            0x801365e8 => {
                c.sth(c.r[28].wrapping_add(0x4c), c.r[0]);
                pc = 0x801365ec;
            }
            0x801365ec => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x40));
                pc = 0x801365f0;
            }
            0x801365f0 => {
                c.sth(c.r[28].wrapping_add(0x4e), c.r[0]);
                pc = 0x801365f4;
            }
            0x801365f4 => {
                { let ea = c.r[29].wrapping_add(0x38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801365f8;
            }
            0x801365f8 => {
                { let ea = c.r[28].wrapping_add(0x48); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801365fc;
            }
            0x801365fc => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x44));
                pc = 0x80136600;
            }
            0x80136600 => {
                c.r[0] = c.r[0].rotate_left(8) & 0xff;
                pc = 0x80136604;
            }
            0x80136604 => {
                c.stb(c.r[28].wrapping_add(0x67), c.r[0]);
                pc = 0x80136608;
            }
            0x80136608 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x44));
                pc = 0x8013660c;
            }
            0x8013660c => {
                c.r[0] = c.r[0].rotate_left(16) & 0xff;
                pc = 0x80136610;
            }
            0x80136610 => {
                c.stb(c.r[28].wrapping_add(0x66), c.r[0]);
                pc = 0x80136614;
            }
            0x80136614 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x44));
                pc = 0x80136618;
            }
            0x80136618 => {
                c.r[0] = c.r[0].rotate_left(24) & 0xff;
                pc = 0x8013661c;
            }
            0x8013661c => {
                c.stb(c.r[28].wrapping_add(0x65), c.r[0]);
                pc = 0x80136620;
            }
            0x80136620 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x44));
                pc = 0x80136624;
            }
            0x80136624 => {
                c.stb(c.r[28].wrapping_add(0x64), c.r[0]);
                pc = 0x80136628;
            }
            0x80136628 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x48));
                pc = 0x8013662c;
            }
            0x8013662c => {
                c.stw(c.r[28].wrapping_add(0x58), c.r[0]);
                pc = 0x80136630;
            }
            0x80136630 => {
                { let ea = c.r[29].wrapping_add(0x34); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136634;
            }
            0x80136634 => {
                { let ea = c.r[28].wrapping_add(0x44); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136638;
            }
            0x80136638 => {
                { let ea = c.r[29].wrapping_add(0x30); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8013663c;
            }
            0x8013663c => {
                { let ea = c.r[28].wrapping_add(0x40); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136640;
            }
            0x80136640 => {
                c.lr = 0x80136644; f_800e01d0(c);
                pc = 0x80136644;
            }
            0x80136644 => {
                c.r[3] = c.r[28].wrapping_add(0x34);
                pc = 0x80136648;
            }
            0x80136648 => {
                c.r[4] = c.r[29].wrapping_add(0x24);
                pc = 0x8013664c;
            }
            0x8013664c => {
                c.lr = 0x80136650; f_800e01d0(c);
                pc = 0x80136650;
            }
            0x80136650 => {
                c.r[3] = c.r[28].wrapping_add(0x1c);
                pc = 0x80136654;
            }
            0x80136654 => {
                c.r[4] = c.r[29].wrapping_add(0xc);
                pc = 0x80136658;
            }
            0x80136658 => {
                c.lr = 0x8013665c; f_800e01d0(c);
                pc = 0x8013665c;
            }
            0x8013665c => {
                c.r[3] = c.r[28].wrapping_add(0x28);
                pc = 0x80136660;
            }
            0x80136660 => {
                c.r[4] = c.r[29].wrapping_add(0x18);
                pc = 0x80136664;
            }
            0x80136664 => {
                c.lr = 0x80136668; f_800e01d0(c);
                pc = 0x80136668;
            }
            0x80136668 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x4c));
                pc = 0x8013666c;
            }
            0x8013666c => {
                c.r[0] = c.r[29].wrapping_add(0x73);
                pc = 0x80136670;
            }
            0x80136670 => {
                c.r[27] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x80136674;
            }
            0x80136674 => {
                c.r[4] = 0x20;
                pc = 0x80136678;
            }
            0x80136678 => {
                c.r[0] = c.r[3].wrapping_add(0x1f);
                pc = 0x8013667c;
            }
            0x8013667c => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x80136680;
            }
            0x80136680 => {
                c.lr = 0x80136684; f_800e2c04(c);
                pc = 0x80136684;
            }
            0x80136684 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                pc = 0x80136688;
            }
            0x80136688 => {
                c.r[31] = c.r[3] | c.r[3];
                pc = 0x8013668c;
            }
            0x8013668c => {
                if c.crbit(2) { pc = 0x801366c8; continue; }
                pc = 0x80136690;
            }
            0x80136690 => {
                c.lr = 0x80136694; f_800e27b0(c);
                pc = 0x80136694;
            }
            0x80136694 => {
                c.r[5] = c.lwz(c.r[29].wrapping_add(0x4c));
                pc = 0x80136698;
            }
            0x80136698 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x8013669c;
            }
            0x8013669c => {
                c.r[4] = c.r[27] | c.r[27];
                pc = 0x801366a0;
            }
            0x801366a0 => {
                c.r[0] = c.r[5].wrapping_add(0x1f);
                pc = 0x801366a4;
            }
            0x801366a4 => {
                c.r[5] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x801366a8;
            }
            0x801366a8 => {
                c.lr = 0x801366ac; f_800054f4(c);
                pc = 0x801366ac;
            }
            0x801366ac => {
                c.r[3] = c.r[30] | c.r[30];
                pc = 0x801366b0;
            }
            0x801366b0 => {
                c.lr = 0x801366b4; f_800efd3c(c);
                pc = 0x801366b4;
            }
            0x801366b4 => {
                c.stw(c.r[28].wrapping_add(0x60), c.r[3]);
                pc = 0x801366b8;
            }
            0x801366b8 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801366bc;
            }
            0x801366bc => {
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x60));
                pc = 0x801366c0;
            }
            0x801366c0 => {
                c.lr = 0x801366c4; f_800efd14(c);
                pc = 0x801366c4;
            }
            0x801366c4 => {
                pc = 0x801366d0; continue;
                pc = 0x801366c8;
            }
            0x801366c8 => {
                c.r[0] = 0x0;
                pc = 0x801366cc;
            }
            0x801366cc => {
                c.stw(c.r[28].wrapping_add(0x60), c.r[0]);
                pc = 0x801366d0;
            }
            0x801366d0 => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x4c));
                pc = 0x801366d4;
            }
            0x801366d4 => {
                c.r[3] = 0x80310000;
                pc = 0x801366d8;
            }
            0x801366d8 => {
                c.r[0] = c.r[3].wrapping_add(0x4ae8);
                pc = 0x801366dc;
            }
            0x801366dc => {
                c.r[3] = c.r[4].wrapping_add(0x1f);
                pc = 0x801366e0;
            }
            0x801366e0 => {
                c.stw(c.r[28].wrapping_add(0x5c), c.r[0]);
                pc = 0x801366e4;
            }
            0x801366e4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffffffe0;
                pc = 0x801366e8;
            }
            0x801366e8 => {
                c.r[27] = c.r[27].wrapping_add(c.r[0]);
                pc = 0x801366ec;
            }
            0x801366ec => {
                c.r[29] = c.r[27] | c.r[27];
                pc = 0x801366f0;
            }
            0x801366f0 => {
                pc = 0x801370ec; continue;
                pc = 0x801366f4;
            }
            0x801366f4 => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x801366f8;
            }
            0x801366f8 => {
                c.r[4] = 0x0;
                pc = 0x801366fc;
            }
            0x801366fc => {
                c.r[5] = 0x24;
                pc = 0x80136700;
            }
            0x80136700 => {
                c.lr = 0x80136704; f_8000540c(c);
                pc = 0x80136704;
            }
            0x80136704 => {
                { let ea = c.r[29].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136708;
            }
            0x80136708 => {
                c.r[4] = 0x43300000;
                pc = 0x8013670c;
            }
            0x8013670c => {
                c.r[3] = 0x80310000;
                pc = 0x80136710;
            }
            0x80136710 => {
                c.stw(c.r[1].wrapping_add(0x20), c.r[4]);
                pc = 0x80136714;
            }
            0x80136714 => {
                c.r[0] = c.r[3].wrapping_add(0x4638);
                pc = 0x80136718;
            }
            0x80136718 => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[1] = c.lfd(ea); }
                pc = 0x8013671c;
            }
            0x8013671c => {
                { let ea = c.r[28].wrapping_add(0x18); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136720;
            }
            0x80136720 => {
                { let ea = c.r[29].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136724;
            }
            0x80136724 => {
                c.stw(c.r[1].wrapping_add(0x18), c.r[4]);
                pc = 0x80136728;
            }
            0x80136728 => {
                { let ea = c.r[28].wrapping_add(0x1c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8013672c;
            }
            0x8013672c => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x80136730;
            }
            0x80136730 => {
                c.r[3] = c.r[3] ^ 0x80000000;
                pc = 0x80136734;
            }
            0x80136734 => {
                c.stw(c.r[1].wrapping_add(0x24), c.r[3]);
                pc = 0x80136738;
            }
            0x80136738 => {
                { let ea = c.r[1].wrapping_add(0x20); c.f[0] = c.lfd(ea); }
                pc = 0x8013673c;
            }
            0x8013673c => {
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80136740;
            }
            0x80136740 => {
                { let ea = c.r[28].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136744;
            }
            0x80136744 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                pc = 0x80136748;
            }
            0x80136748 => {
                c.r[3] = c.r[3] ^ 0x80000000;
                pc = 0x8013674c;
            }
            0x8013674c => {
                c.stw(c.r[1].wrapping_add(0x1c), c.r[3]);
                pc = 0x80136750;
            }
            0x80136750 => {
                { let ea = c.r[1].wrapping_add(0x18); c.f[0] = c.lfd(ea); }
                pc = 0x80136754;
            }
            0x80136754 => {
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80136758;
            }
            0x80136758 => {
                { let ea = c.r[28].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8013675c;
            }
            0x8013675c => {
                { let ea = c.r[29].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136760;
            }
            0x80136760 => {
                { let ea = c.r[28].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136764;
            }
            0x80136764 => {
                c.r[3] = c.lwz(c.r[29]);
                pc = 0x80136768;
            }
            0x80136768 => {
                c.r[3] = c.r[3].rotate_left(8) & 0xff;
                pc = 0x8013676c;
            }
            0x8013676c => {
                c.stb(c.r[28].wrapping_add(0xb), c.r[3]);
                pc = 0x80136770;
            }
            0x80136770 => {
                c.r[3] = c.lwz(c.r[29]);
                pc = 0x80136774;
            }
            0x80136774 => {
                c.r[3] = c.r[3].rotate_left(16) & 0xff;
                pc = 0x80136778;
            }
            0x80136778 => {
                c.stb(c.r[28].wrapping_add(0xa), c.r[3]);
                pc = 0x8013677c;
            }
            0x8013677c => {
                c.r[3] = c.lwz(c.r[29]);
                pc = 0x80136780;
            }
            0x80136780 => {
                c.r[3] = c.r[3].rotate_left(24) & 0xff;
                pc = 0x80136784;
            }
            0x80136784 => {
                c.stb(c.r[28].wrapping_add(0x9), c.r[3]);
                pc = 0x80136788;
            }
            0x80136788 => {
                c.r[3] = c.lwz(c.r[29]);
                pc = 0x8013678c;
            }
            0x8013678c => {
                c.stb(c.r[28].wrapping_add(0x8), c.r[3]);
                pc = 0x80136790;
            }
            0x80136790 => {
                c.stw(c.r[28].wrapping_add(0xc), c.r[0]);
                pc = 0x80136794;
            }
            0x80136794 => {
                c.lr = 0x80136798; f_800d37cc(c);
                pc = 0x80136798;
            }
            0x80136798 => {
                c.r[4] = 0x43300000;
                pc = 0x8013679c;
            }
            0x8013679c => {
                c.r[3] = c.r[3] ^ 0x80000000;
                pc = 0x801367a0;
            }
            0x801367a0 => {
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                pc = 0x801367a4;
            }
            0x801367a4 => {
                c.r[0] = c.r[30] ^ 0x80000000;
                pc = 0x801367a8;
            }
            0x801367a8 => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                pc = 0x801367ac;
            }
            0x801367ac => {
                c.r[29] = c.r[29].wrapping_add(0x1c);
                pc = 0x801367b0;
            }
            0x801367b0 => {
                c.stw(c.r[1].wrapping_add(0x10), c.r[4]);
                pc = 0x801367b4;
            }
            0x801367b4 => {
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801367b8;
            }
            0x801367b8 => {
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfd(ea); }
                pc = 0x801367bc;
            }
            0x801367bc => {
                c.stw(c.r[1].wrapping_add(0x2c), c.r[0]);
                pc = 0x801367c0;
            }
            0x801367c0 => {
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                pc = 0x801367c4;
            }
            0x801367c4 => {
                c.stw(c.r[1].wrapping_add(0x28), c.r[4]);
                pc = 0x801367c8;
            }
            0x801367c8 => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[1] = c.lfd(ea); }
                pc = 0x801367cc;
            }
            0x801367cc => {
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x801367d0;
            }
            0x801367d0 => {
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x801367d4;
            }
            0x801367d4 => {
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x801367d8;
            }
            0x801367d8 => {
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x801367dc;
            }
            0x801367dc => {
                { let ea = c.r[1].wrapping_add(0x30); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x801367e0;
            }
            0x801367e0 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                pc = 0x801367e4;
            }
            0x801367e4 => {
                c.sth(c.r[28].wrapping_add(0x12), c.r[0]);
                pc = 0x801367e8;
            }
            0x801367e8 => {
                pc = 0x801370ec; continue;
                pc = 0x801367ec;
            }
            0x801367ec => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x801367f0;
            }
            0x801367f0 => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x801367f4;
            }
            0x801367f4 => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x801367f8;
            }
            0x801367f8 => {
                c.lr = 0x801367fc; f_8013757c(c);
                pc = 0x801367fc;
            }
            0x801367fc => {
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x80136800;
            }
            0x80136800 => {
                pc = 0x801370ec; continue;
                pc = 0x80136804;
            }
            0x80136804 => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136808;
            }
            0x80136808 => {
                c.r[4] = 0x0;
                pc = 0x8013680c;
            }
            0x8013680c => {
                c.r[5] = 0x2c;
                pc = 0x80136810;
            }
            0x80136810 => {
                c.lr = 0x80136814; f_8000540c(c);
                pc = 0x80136814;
            }
            0x80136814 => {
                c.lr = 0x80136818; f_800d37cc(c);
                pc = 0x80136818;
            }
            0x80136818 => {
                c.r[5] = 0x43300000;
                pc = 0x8013681c;
            }
            0x8013681c => {
                c.r[0] = c.r[3] ^ 0x80000000;
                pc = 0x80136820;
            }
            0x80136820 => {
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                pc = 0x80136824;
            }
            0x80136824 => {
                c.r[3] = c.r[30] ^ 0x80000000;
                pc = 0x80136828;
            }
            0x80136828 => {
                c.r[0] = c.r[29].wrapping_add(0x37);
                pc = 0x8013682c;
            }
            0x8013682c => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                pc = 0x80136830;
            }
            0x80136830 => {
                c.stw(c.r[1].wrapping_add(0x30), c.r[5]);
                pc = 0x80136834;
            }
            0x80136834 => {
                c.r[31] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x80136838;
            }
            0x80136838 => {
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8013683c;
            }
            0x8013683c => {
                c.r[4] = 0x20;
                pc = 0x80136840;
            }
            0x80136840 => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[1] = c.lfd(ea); }
                pc = 0x80136844;
            }
            0x80136844 => {
                c.stw(c.r[1].wrapping_add(0x2c), c.r[3]);
                pc = 0x80136848;
            }
            0x80136848 => {
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                pc = 0x8013684c;
            }
            0x8013684c => {
                c.stw(c.r[1].wrapping_add(0x28), c.r[5]);
                pc = 0x80136850;
            }
            0x80136850 => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[1] = c.lfd(ea); }
                pc = 0x80136854;
            }
            0x80136854 => {
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x80136858;
            }
            0x80136858 => {
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x8013685c;
            }
            0x8013685c => {
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80136860;
            }
            0x80136860 => {
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x80136864;
            }
            0x80136864 => {
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x80136868;
            }
            0x80136868 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x8013686c;
            }
            0x8013686c => {
                c.sth(c.r[28].wrapping_add(0x28), c.r[0]);
                pc = 0x80136870;
            }
            0x80136870 => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x80136874;
            }
            0x80136874 => {
                c.r[0] = c.r[0].rotate_left(8) & 0xff;
                pc = 0x80136878;
            }
            0x80136878 => {
                c.stb(c.r[28].wrapping_add(0x23), c.r[0]);
                pc = 0x8013687c;
            }
            0x8013687c => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x80136880;
            }
            0x80136880 => {
                c.r[0] = c.r[0].rotate_left(16) & 0xff;
                pc = 0x80136884;
            }
            0x80136884 => {
                c.stb(c.r[28].wrapping_add(0x22), c.r[0]);
                pc = 0x80136888;
            }
            0x80136888 => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x8013688c;
            }
            0x8013688c => {
                c.r[0] = c.r[0].rotate_left(24) & 0xff;
                pc = 0x80136890;
            }
            0x80136890 => {
                c.stb(c.r[28].wrapping_add(0x21), c.r[0]);
                pc = 0x80136894;
            }
            0x80136894 => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x80136898;
            }
            0x80136898 => {
                c.stb(c.r[28].wrapping_add(0x20), c.r[0]);
                pc = 0x8013689c;
            }
            0x8013689c => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x4));
                pc = 0x801368a0;
            }
            0x801368a0 => {
                c.sth(c.r[28].wrapping_add(0x2a), c.r[0]);
                pc = 0x801368a4;
            }
            0x801368a4 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                pc = 0x801368a8;
            }
            0x801368a8 => {
                c.sth(c.r[28].wrapping_add(0x30), c.r[0]);
                pc = 0x801368ac;
            }
            0x801368ac => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x801368b0;
            }
            0x801368b0 => {
                c.sth(c.r[28].wrapping_add(0x32), c.r[0]);
                pc = 0x801368b4;
            }
            0x801368b4 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                pc = 0x801368b8;
            }
            0x801368b8 => {
                c.r[0] = c.r[3].wrapping_add(0x1f);
                pc = 0x801368bc;
            }
            0x801368bc => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x801368c0;
            }
            0x801368c0 => {
                c.lr = 0x801368c4; f_800e2c04(c);
                pc = 0x801368c4;
            }
            0x801368c4 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                pc = 0x801368c8;
            }
            0x801368c8 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x801368cc;
            }
            0x801368cc => {
                if c.crbit(2) { pc = 0x80136908; continue; }
                pc = 0x801368d0;
            }
            0x801368d0 => {
                c.lr = 0x801368d4; f_800e27b0(c);
                pc = 0x801368d4;
            }
            0x801368d4 => {
                c.r[5] = c.lwz(c.r[29].wrapping_add(0x10));
                pc = 0x801368d8;
            }
            0x801368d8 => {
                c.r[27] = c.r[3] | c.r[3];
                pc = 0x801368dc;
            }
            0x801368dc => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801368e0;
            }
            0x801368e0 => {
                c.r[0] = c.r[5].wrapping_add(0x1f);
                pc = 0x801368e4;
            }
            0x801368e4 => {
                c.r[5] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x801368e8;
            }
            0x801368e8 => {
                c.lr = 0x801368ec; f_800054f4(c);
                pc = 0x801368ec;
            }
            0x801368ec => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x801368f0;
            }
            0x801368f0 => {
                c.lr = 0x801368f4; f_800efd3c(c);
                pc = 0x801368f4;
            }
            0x801368f4 => {
                c.stw(c.r[28].wrapping_add(0x1c), c.r[3]);
                pc = 0x801368f8;
            }
            0x801368f8 => {
                c.r[4] = c.r[30] | c.r[30];
                pc = 0x801368fc;
            }
            0x801368fc => {
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x1c));
                pc = 0x80136900;
            }
            0x80136900 => {
                c.lr = 0x80136904; f_800efd14(c);
                pc = 0x80136904;
            }
            0x80136904 => {
                pc = 0x80136910; continue;
                pc = 0x80136908;
            }
            0x80136908 => {
                c.r[0] = 0x0;
                pc = 0x8013690c;
            }
            0x8013690c => {
                c.stw(c.r[28].wrapping_add(0x1c), c.r[0]);
                pc = 0x80136910;
            }
            0x80136910 => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x10));
                pc = 0x80136914;
            }
            0x80136914 => {
                c.r[3] = c.lwz(c.r[28].wrapping_add(0x1c));
                pc = 0x80136918;
            }
            0x80136918 => {
                c.r[0] = c.r[4].wrapping_add(0x1f);
                pc = 0x8013691c;
            }
            0x8013691c => {
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x80136920;
            }
            0x80136920 => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x80136924;
            }
            0x80136924 => {
                c.r[31] = c.r[31].wrapping_add(c.r[0]);
                pc = 0x80136928;
            }
            0x80136928 => {
                if c.crbit(2) { pc = 0x80136938; continue; }
                pc = 0x8013692c;
            }
            0x8013692c => {
                c.r[4] = 0x0;
                pc = 0x80136930;
            }
            0x80136930 => {
                c.r[5] = 0x0;
                pc = 0x80136934;
            }
            0x80136934 => {
                c.lr = 0x80136938; f_800ef590(c);
                pc = 0x80136938;
            }
            0x80136938 => {
                c.r[3] = 0x80310000;
                pc = 0x8013693c;
            }
            0x8013693c => {
                c.r[29] = c.r[31] | c.r[31];
                pc = 0x80136940;
            }
            0x80136940 => {
                c.r[0] = c.r[3].wrapping_add(0x4ae8);
                pc = 0x80136944;
            }
            0x80136944 => {
                c.stw(c.r[28].wrapping_add(0x18), c.r[0]);
                pc = 0x80136948;
            }
            0x80136948 => {
                pc = 0x801370ec; continue;
                pc = 0x8013694c;
            }
            0x8013694c => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136950;
            }
            0x80136950 => {
                c.r[4] = 0x0;
                pc = 0x80136954;
            }
            0x80136954 => {
                c.r[5] = 0x4c;
                pc = 0x80136958;
            }
            0x80136958 => {
                c.lr = 0x8013695c; f_8000540c(c);
                pc = 0x8013695c;
            }
            0x8013695c => {
                c.lr = 0x80136960; f_800d37cc(c);
                pc = 0x80136960;
            }
            0x80136960 => {
                c.r[5] = 0x43300000;
                pc = 0x80136964;
            }
            0x80136964 => {
                c.r[0] = c.r[3] ^ 0x80000000;
                pc = 0x80136968;
            }
            0x80136968 => {
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                pc = 0x8013696c;
            }
            0x8013696c => {
                c.r[4] = c.r[30] ^ 0x80000000;
                pc = 0x80136970;
            }
            0x80136970 => {
                c.r[3] = c.r[29].wrapping_add(0x5b);
                pc = 0x80136974;
            }
            0x80136974 => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                pc = 0x80136978;
            }
            0x80136978 => {
                c.stw(c.r[1].wrapping_add(0x30), c.r[5]);
                pc = 0x8013697c;
            }
            0x8013697c => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffffffe0;
                pc = 0x80136980;
            }
            0x80136980 => {
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136984;
            }
            0x80136984 => {
                c.r[0] = 0x4e20;
                pc = 0x80136988;
            }
            0x80136988 => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[1] = c.lfd(ea); }
                pc = 0x8013698c;
            }
            0x8013698c => {
                c.stw(c.r[1].wrapping_add(0x2c), c.r[4]);
                pc = 0x80136990;
            }
            0x80136990 => {
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                pc = 0x80136994;
            }
            0x80136994 => {
                c.stw(c.r[1].wrapping_add(0x28), c.r[5]);
                pc = 0x80136998;
            }
            0x80136998 => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[1] = c.lfd(ea); }
                pc = 0x8013699c;
            }
            0x8013699c => {
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x801369a0;
            }
            0x801369a0 => {
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x801369a4;
            }
            0x801369a4 => {
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x801369a8;
            }
            0x801369a8 => {
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x801369ac;
            }
            0x801369ac => {
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x801369b0;
            }
            0x801369b0 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x801369b4;
            }
            0x801369b4 => {
                c.sth(c.r[28].wrapping_add(0x46), c.r[3]);
                pc = 0x801369b8;
            }
            0x801369b8 => {
                { let ea = c.r[29]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801369bc;
            }
            0x801369bc => {
                { let ea = c.r[28].wrapping_add(0x18); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801369c0;
            }
            0x801369c0 => {
                { let ea = c.r[29].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801369c4;
            }
            0x801369c4 => {
                { let ea = c.r[28].wrapping_add(0x1c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801369c8;
            }
            0x801369c8 => {
                { let ea = c.r[29].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801369cc;
            }
            0x801369cc => {
                { let ea = c.r[28].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801369d0;
            }
            0x801369d0 => {
                { let ea = c.r[29].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801369d4;
            }
            0x801369d4 => {
                { let ea = c.r[28].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801369d8;
            }
            0x801369d8 => {
                { let ea = c.r[29].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801369dc;
            }
            0x801369dc => {
                { let ea = c.r[28].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801369e0;
            }
            0x801369e0 => {
                { let ea = c.r[29].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801369e4;
            }
            0x801369e4 => {
                { let ea = c.r[28].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801369e8;
            }
            0x801369e8 => {
                { let ea = c.r[29].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801369ec;
            }
            0x801369ec => {
                { let ea = c.r[28].wrapping_add(0x30); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801369f0;
            }
            0x801369f0 => {
                { let ea = c.r[29].wrapping_add(0x1c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801369f4;
            }
            0x801369f4 => {
                { let ea = c.r[28].wrapping_add(0x34); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801369f8;
            }
            0x801369f8 => {
                { let ea = c.r[29].wrapping_add(0x20); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x801369fc;
            }
            0x801369fc => {
                { let ea = c.r[28].wrapping_add(0x38); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136a00;
            }
            0x80136a00 => {
                { let ea = c.r[29].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136a04;
            }
            0x80136a04 => {
                { let ea = c.r[28].wrapping_add(0x3c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136a08;
            }
            0x80136a08 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x28));
                pc = 0x80136a0c;
            }
            0x80136a0c => {
                c.sth(c.r[28].wrapping_add(0x40), c.r[3]);
                pc = 0x80136a10;
            }
            0x80136a10 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2c));
                pc = 0x80136a14;
            }
            0x80136a14 => {
                c.sth(c.r[28].wrapping_add(0x42), c.r[3]);
                pc = 0x80136a18;
            }
            0x80136a18 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x30));
                pc = 0x80136a1c;
            }
            0x80136a1c => {
                c.sth(c.r[28].wrapping_add(0x52), c.r[3]);
                pc = 0x80136a20;
            }
            0x80136a20 => {
                c.sth(c.r[28].wrapping_add(0x48), c.r[0]);
                pc = 0x80136a24;
            }
            0x80136a24 => {
                c.lr = 0x80136a28; f_801db060(c);
                pc = 0x80136a28;
            }
            0x80136a28 => {
                c.sth(c.r[28].wrapping_add(0x4a), c.r[3]);
                pc = 0x80136a2c;
            }
            0x80136a2c => {
                c.lr = 0x80136a30; f_801db060(c);
                pc = 0x80136a30;
            }
            0x80136a30 => {
                c.sth(c.r[28].wrapping_add(0x4c), c.r[3]);
                pc = 0x80136a34;
            }
            0x80136a34 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x80136a38;
            }
            0x80136a38 => {
                c.r[5] = 0x4e20;
                pc = 0x80136a3c;
            }
            0x80136a3c => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x34));
                pc = 0x80136a40;
            }
            0x80136a40 => {
                c.r[6] = c.lhz(c.r[28].wrapping_add(0x4a));
                pc = 0x80136a44;
            }
            0x80136a44 => {
                c.lr = 0x80136a48; f_8010147c(c);
                pc = 0x80136a48;
            }
            0x80136a48 => {
                c.r[4] = c.lhz(c.r[28].wrapping_add(0x4a));
                pc = 0x80136a4c;
            }
            0x80136a4c => {
                c.r[3] = 0x4e20;
                pc = 0x80136a50;
            }
            0x80136a50 => {
                c.lr = 0x80136a54; f_800f9318(c);
                pc = 0x80136a54;
            }
            0x80136a54 => {
                c.r[6] = c.lhz(c.r[28].wrapping_add(0x4c));
                pc = 0x80136a58;
            }
            0x80136a58 => {
                c.r[4] = 0x4e20;
                pc = 0x80136a5c;
            }
            0x80136a5c => {
                c.r[5] = 0x0;
                pc = 0x80136a60;
            }
            0x80136a60 => {
                c.lr = 0x80136a64; f_801013a0(c);
                pc = 0x80136a64;
            }
            0x80136a64 => {
                c.r[4] = c.lhz(c.r[28].wrapping_add(0x4c));
                pc = 0x80136a68;
            }
            0x80136a68 => {
                c.r[3] = 0x4e20;
                pc = 0x80136a6c;
            }
            0x80136a6c => {
                c.lr = 0x80136a70; f_800f9318(c);
                pc = 0x80136a70;
            }
            0x80136a70 => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x80136a74;
            }
            0x80136a74 => {
                if c.crbit(2) { pc = 0x80136a80; continue; }
                pc = 0x80136a78;
            }
            0x80136a78 => {
                c.r[4] = 0x0;
                pc = 0x80136a7c;
            }
            0x80136a7c => {
                c.lr = 0x80136a80; f_800e4014(c);
                pc = 0x80136a80;
            }
            0x80136a80 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x34));
                pc = 0x80136a84;
            }
            0x80136a84 => {
                c.r[0] = c.r[3].wrapping_add(0x1f);
                pc = 0x80136a88;
            }
            0x80136a88 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x80136a8c;
            }
            0x80136a8c => {
                c.r[0] = c.r[27].wrapping_add(c.r[0]);
                pc = 0x80136a90;
            }
            0x80136a90 => {
                c.r[29] = c.r[0] | c.r[0];
                pc = 0x80136a94;
            }
            0x80136a94 => {
                pc = 0x801370ec; continue;
                pc = 0x80136a98;
            }
            0x80136a98 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80136a9c;
            }
            0x80136a9c => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80136aa0;
            }
            0x80136aa0 => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136aa4;
            }
            0x80136aa4 => {
                c.lr = 0x80136aa8; f_8013735c(c);
                pc = 0x80136aa8;
            }
            0x80136aa8 => {
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x80136aac;
            }
            0x80136aac => {
                pc = 0x801370ec; continue;
                pc = 0x80136ab0;
            }
            0x80136ab0 => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136ab4;
            }
            0x80136ab4 => {
                c.r[4] = 0x0;
                pc = 0x80136ab8;
            }
            0x80136ab8 => {
                c.r[5] = 0xd0;
                pc = 0x80136abc;
            }
            0x80136abc => {
                c.lr = 0x80136ac0; f_8000540c(c);
                pc = 0x80136ac0;
            }
            0x80136ac0 => {
                c.lr = 0x80136ac4; f_800d37cc(c);
                pc = 0x80136ac4;
            }
            0x80136ac4 => {
                c.r[5] = 0x43300000;
                pc = 0x80136ac8;
            }
            0x80136ac8 => {
                c.r[0] = c.r[3] ^ 0x80000000;
                pc = 0x80136acc;
            }
            0x80136acc => {
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                pc = 0x80136ad0;
            }
            0x80136ad0 => {
                c.r[4] = c.r[30] ^ 0x80000000;
                pc = 0x80136ad4;
            }
            0x80136ad4 => {
                c.r[3] = c.r[29].wrapping_add(0x53);
                pc = 0x80136ad8;
            }
            0x80136ad8 => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                pc = 0x80136adc;
            }
            0x80136adc => {
                c.stw(c.r[1].wrapping_add(0x30), c.r[5]);
                pc = 0x80136ae0;
            }
            0x80136ae0 => {
                c.r[31] = c.r[3].rotate_left(0) & 0xffffffe0;
                pc = 0x80136ae4;
            }
            0x80136ae4 => {
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136ae8;
            }
            0x80136ae8 => {
                c.r[0] = 0x4e20;
                pc = 0x80136aec;
            }
            0x80136aec => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[1] = c.lfd(ea); }
                pc = 0x80136af0;
            }
            0x80136af0 => {
                c.stw(c.r[1].wrapping_add(0x2c), c.r[4]);
                pc = 0x80136af4;
            }
            0x80136af4 => {
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                pc = 0x80136af8;
            }
            0x80136af8 => {
                c.stw(c.r[1].wrapping_add(0x28), c.r[5]);
                pc = 0x80136afc;
            }
            0x80136afc => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[1] = c.lfd(ea); }
                pc = 0x80136b00;
            }
            0x80136b00 => {
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x80136b04;
            }
            0x80136b04 => {
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x80136b08;
            }
            0x80136b08 => {
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80136b0c;
            }
            0x80136b0c => {
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x80136b10;
            }
            0x80136b10 => {
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x80136b14;
            }
            0x80136b14 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x80136b18;
            }
            0x80136b18 => {
                c.sth(c.r[28].wrapping_add(0xd6), c.r[3]);
                pc = 0x80136b1c;
            }
            0x80136b1c => {
                { let ea = c.r[29]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136b20;
            }
            0x80136b20 => {
                { let ea = c.r[28].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136b24;
            }
            0x80136b24 => {
                { let ea = c.r[29].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136b28;
            }
            0x80136b28 => {
                { let ea = c.r[28].wrapping_add(0x30); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136b2c;
            }
            0x80136b2c => {
                { let ea = c.r[29].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136b30;
            }
            0x80136b30 => {
                { let ea = c.r[28].wrapping_add(0x34); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136b34;
            }
            0x80136b34 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x80136b38;
            }
            0x80136b38 => {
                { let v = c.r[3] as i32; c.set_ca(v < 0 && (v as u32) << 8 != 0); c.r[3] = (v >> 24) as u32; }
                pc = 0x80136b3c;
            }
            0x80136b3c => {
                c.stb(c.r[28].wrapping_add(0x47), c.r[3]);
                pc = 0x80136b40;
            }
            0x80136b40 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x80136b44;
            }
            0x80136b44 => {
                c.r[3] = c.r[3].rotate_left(16) & 0xff;
                pc = 0x80136b48;
            }
            0x80136b48 => {
                c.stb(c.r[28].wrapping_add(0x46), c.r[3]);
                pc = 0x80136b4c;
            }
            0x80136b4c => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x80136b50;
            }
            0x80136b50 => {
                c.r[3] = c.r[3].rotate_left(24) & 0xff;
                pc = 0x80136b54;
            }
            0x80136b54 => {
                c.stb(c.r[28].wrapping_add(0x45), c.r[3]);
                pc = 0x80136b58;
            }
            0x80136b58 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x80136b5c;
            }
            0x80136b5c => {
                c.stb(c.r[28].wrapping_add(0x44), c.r[3]);
                pc = 0x80136b60;
            }
            0x80136b60 => {
                { let ea = c.r[29].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136b64;
            }
            0x80136b64 => {
                { let ea = c.r[28].wrapping_add(0x48); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136b68;
            }
            0x80136b68 => {
                { let ea = c.r[29].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136b6c;
            }
            0x80136b6c => {
                { let ea = c.r[28].wrapping_add(0x4c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136b70;
            }
            0x80136b70 => {
                { let ea = c.r[29].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136b74;
            }
            0x80136b74 => {
                { let ea = c.r[28].wrapping_add(0x50); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136b78;
            }
            0x80136b78 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x1c));
                pc = 0x80136b7c;
            }
            0x80136b7c => {
                c.sth(c.r[28].wrapping_add(0x54), c.r[3]);
                pc = 0x80136b80;
            }
            0x80136b80 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x20));
                pc = 0x80136b84;
            }
            0x80136b84 => {
                c.sth(c.r[28].wrapping_add(0x56), c.r[3]);
                pc = 0x80136b88;
            }
            0x80136b88 => {
                { let ea = c.r[29].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136b8c;
            }
            0x80136b8c => {
                { let ea = c.r[28].wrapping_add(0xcc); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136b90;
            }
            0x80136b90 => {
                { let ea = c.r[29].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136b94;
            }
            0x80136b94 => {
                { let ea = c.r[28].wrapping_add(0xd0); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136b98;
            }
            0x80136b98 => {
                c.stw(c.r[28].wrapping_add(0x58), c.r[0]);
                pc = 0x80136b9c;
            }
            0x80136b9c => {
                c.lr = 0x80136ba0; f_801db060(c);
                pc = 0x80136ba0;
            }
            0x80136ba0 => {
                c.stw(c.r[28].wrapping_add(0x60), c.r[3]);
                pc = 0x80136ba4;
            }
            0x80136ba4 => {
                c.lr = 0x80136ba8; f_801db060(c);
                pc = 0x80136ba8;
            }
            0x80136ba8 => {
                c.stw(c.r[28].wrapping_add(0x5c), c.r[3]);
                pc = 0x80136bac;
            }
            0x80136bac => {
                c.r[3] = c.r[31] | c.r[31];
                pc = 0x80136bb0;
            }
            0x80136bb0 => {
                c.r[5] = 0x4e20;
                pc = 0x80136bb4;
            }
            0x80136bb4 => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0x2c));
                pc = 0x80136bb8;
            }
            0x80136bb8 => {
                c.r[6] = c.lwz(c.r[28].wrapping_add(0x60));
                pc = 0x80136bbc;
            }
            0x80136bbc => {
                c.lr = 0x80136bc0; f_8010147c(c);
                pc = 0x80136bc0;
            }
            0x80136bc0 => {
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x60));
                pc = 0x80136bc4;
            }
            0x80136bc4 => {
                c.r[3] = 0x4e20;
                pc = 0x80136bc8;
            }
            0x80136bc8 => {
                c.lr = 0x80136bcc; f_800f9318(c);
                pc = 0x80136bcc;
            }
            0x80136bcc => {
                c.r[6] = c.lwz(c.r[28].wrapping_add(0x5c));
                pc = 0x80136bd0;
            }
            0x80136bd0 => {
                c.r[4] = 0x4e20;
                pc = 0x80136bd4;
            }
            0x80136bd4 => {
                c.r[5] = 0x0;
                pc = 0x80136bd8;
            }
            0x80136bd8 => {
                c.lr = 0x80136bdc; f_801013a0(c);
                pc = 0x80136bdc;
            }
            0x80136bdc => {
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x5c));
                pc = 0x80136be0;
            }
            0x80136be0 => {
                c.r[3] = 0x4e20;
                pc = 0x80136be4;
            }
            0x80136be4 => {
                c.lr = 0x80136be8; f_800f9318(c);
                pc = 0x80136be8;
            }
            0x80136be8 => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x80136bec;
            }
            0x80136bec => {
                if c.crbit(2) { pc = 0x80136bf8; continue; }
                pc = 0x80136bf0;
            }
            0x80136bf0 => {
                c.r[4] = 0x0;
                pc = 0x80136bf4;
            }
            0x80136bf4 => {
                c.lr = 0x80136bf8; f_800e4014(c);
                pc = 0x80136bf8;
            }
            0x80136bf8 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x2c));
                pc = 0x80136bfc;
            }
            0x80136bfc => {
                c.r[0] = c.r[3].wrapping_add(0x1f);
                pc = 0x80136c00;
            }
            0x80136c00 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x80136c04;
            }
            0x80136c04 => {
                c.r[0] = c.r[31].wrapping_add(c.r[0]);
                pc = 0x80136c08;
            }
            0x80136c08 => {
                c.r[29] = c.r[0] | c.r[0];
                pc = 0x80136c0c;
            }
            0x80136c0c => {
                pc = 0x801370ec; continue;
                pc = 0x80136c10;
            }
            0x80136c10 => {
                c.r[27] = c.lwz(c.r[29].wrapping_add(0x8));
                pc = 0x80136c14;
            }
            0x80136c14 => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136c18;
            }
            0x80136c18 => {
                c.r[4] = 0x0;
                pc = 0x80136c1c;
            }
            0x80136c1c => {
                c.r[5] = 0x18;
                pc = 0x80136c20;
            }
            0x80136c20 => {
                c.lr = 0x80136c24; f_8000540c(c);
                pc = 0x80136c24;
            }
            0x80136c24 => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x80136c28;
            }
            0x80136c28 => {
                c.stw(c.r[28].wrapping_add(0xc), c.r[0]);
                pc = 0x80136c2c;
            }
            0x80136c2c => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x4));
                pc = 0x80136c30;
            }
            0x80136c30 => {
                c.stw(c.r[28].wrapping_add(0x10), c.r[0]);
                pc = 0x80136c34;
            }
            0x80136c34 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x80136c38;
            }
            0x80136c38 => {
                c.cmps(0, c.r[0], 0x2);
                pc = 0x80136c3c;
            }
            0x80136c3c => {
                if c.crbit(2) { pc = 0x80136c50; continue; }
                pc = 0x80136c40;
            }
            0x80136c40 => {
                if !c.crbit(0) { pc = 0x80136d78; continue; }
                pc = 0x80136c44;
            }
            0x80136c44 => {
                c.cmps(0, c.r[0], 0x1);
                pc = 0x80136c48;
            }
            0x80136c48 => {
                if !c.crbit(0) { pc = 0x80136cd0; continue; }
                pc = 0x80136c4c;
            }
            0x80136c4c => {
                pc = 0x80136d78; continue;
                pc = 0x80136c50;
            }
            0x80136c50 => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[30] = c.lfd(ea); }
                pc = 0x80136c54;
            }
            0x80136c54 => {
                c.r[31] = c.r[29].wrapping_add(0x10);
                pc = 0x80136c58;
            }
            0x80136c58 => {
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                pc = 0x80136c5c;
            }
            0x80136c5c => {
                c.r[30] = 0x0;
                pc = 0x80136c60;
            }
            0x80136c60 => {
                c.r[29] = 0x43300000;
                pc = 0x80136c64;
            }
            0x80136c64 => {
                pc = 0x80136cc4; continue;
                pc = 0x80136c68;
            }
            0x80136c68 => {
                c.lr = 0x80136c6c; f_800d37cc(c);
                pc = 0x80136c6c;
            }
            0x80136c6c => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x8));
                pc = 0x80136c70;
            }
            0x80136c70 => {
                c.r[3] = c.r[3] ^ 0x80000000;
                pc = 0x80136c74;
            }
            0x80136c74 => {
                c.stw(c.r[1].wrapping_add(0x34), c.r[3]);
                pc = 0x80136c78;
            }
            0x80136c78 => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136c7c;
            }
            0x80136c7c => {
                c.r[0] = c.r[0] ^ 0x80000000;
                pc = 0x80136c80;
            }
            0x80136c80 => {
                { let ea = c.r[31]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x80136c84;
            }
            0x80136c84 => {
                c.stw(c.r[1].wrapping_add(0x30), c.r[29]);
                pc = 0x80136c88;
            }
            0x80136c88 => {
                { let ea = c.r[31].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                pc = 0x80136c8c;
            }
            0x80136c8c => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[0] = c.lfd(ea); }
                pc = 0x80136c90;
            }
            0x80136c90 => {
                c.stw(c.r[1].wrapping_add(0x2c), c.r[0]);
                pc = 0x80136c94;
            }
            0x80136c94 => {
                { let v = s(c.f[0] - c.f[30]); c.f[3] = v; c.ps1[3] = v; }
                pc = 0x80136c98;
            }
            0x80136c98 => {
                c.stw(c.r[1].wrapping_add(0x28), c.r[29]);
                pc = 0x80136c9c;
            }
            0x80136c9c => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[0] = c.lfd(ea); }
                pc = 0x80136ca0;
            }
            0x80136ca0 => {
                { let v = s(c.f[0] - c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80136ca4;
            }
            0x80136ca4 => {
                { let v = s(c.f[0] * c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80136ca8;
            }
            0x80136ca8 => {
                { let v = s(c.f[0] / c.f[31]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80136cac;
            }
            0x80136cac => {
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x80136cb0;
            }
            0x80136cb0 => {
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x80136cb4;
            }
            0x80136cb4 => {
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x80136cb8;
            }
            0x80136cb8 => {
                c.lr = 0x80136cbc; f_8013d604(c);
                pc = 0x80136cbc;
            }
            0x80136cbc => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                pc = 0x80136cc0;
            }
            0x80136cc0 => {
                c.r[31] = c.r[31].wrapping_add(0x10);
                pc = 0x80136cc4;
            }
            0x80136cc4 => {
                c.cmps(0, c.r[30], c.r[27]);
                pc = 0x80136cc8;
            }
            0x80136cc8 => {
                if c.crbit(0) { pc = 0x80136c68; continue; }
                pc = 0x80136ccc;
            }
            0x80136ccc => {
                pc = 0x80136d78; continue;
                pc = 0x80136cd0;
            }
            0x80136cd0 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                pc = 0x80136cd4;
            }
            0x80136cd4 => {
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x80136cd8;
            }
            0x80136cd8 => {
                { let ea = c.r[1].wrapping_add(0x8); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                pc = 0x80136cdc;
            }
            0x80136cdc => {
                c.lr = 0x80136ce0; f_800d37cc(c);
                pc = 0x80136ce0;
            }
            0x80136ce0 => {
                c.r[0] = c.r[3] ^ 0x80000000;
                pc = 0x80136ce4;
            }
            0x80136ce4 => {
                c.r[5] = 0x43300000;
                pc = 0x80136ce8;
            }
            0x80136ce8 => {
                c.r[4] = c.r[30] ^ 0x80000000;
                pc = 0x80136cec;
            }
            0x80136cec => {
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                pc = 0x80136cf0;
            }
            0x80136cf0 => {
                c.r[3] = 0x55550000;
                pc = 0x80136cf4;
            }
            0x80136cf4 => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[5] = c.lfd(ea); }
                pc = 0x80136cf8;
            }
            0x80136cf8 => {
                c.stw(c.r[1].wrapping_add(0x30), c.r[5]);
                pc = 0x80136cfc;
            }
            0x80136cfc => {
                c.r[0] = c.r[3].wrapping_add(0x5556);
                pc = 0x80136d00;
            }
            0x80136d00 => {
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136d04;
            }
            0x80136d04 => {
                c.f[2] = c.f[30];
                pc = 0x80136d08;
            }
            0x80136d08 => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[1] = c.lfd(ea); }
                pc = 0x80136d0c;
            }
            0x80136d0c => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136d10;
            }
            0x80136d10 => {
                c.stw(c.r[1].wrapping_add(0x2c), c.r[4]);
                pc = 0x80136d14;
            }
            0x80136d14 => {
                { let v = s(c.f[1] - c.f[5]); c.f[4] = v; c.ps1[4] = v; }
                pc = 0x80136d18;
            }
            0x80136d18 => {
                { let ea = c.r[2].wrapping_add(0xffff9a7c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x80136d1c;
            }
            0x80136d1c => {
                c.stw(c.r[1].wrapping_add(0x28), c.r[5]);
                pc = 0x80136d20;
            }
            0x80136d20 => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[3] = c.lfd(ea); }
                pc = 0x80136d24;
            }
            0x80136d24 => {
                { let v = s(c.f[3] - c.f[5]); c.f[3] = v; c.ps1[3] = v; }
                pc = 0x80136d28;
            }
            0x80136d28 => {
                { let v = s(c.f[3] * c.f[4]); c.f[3] = v; c.ps1[3] = v; }
                pc = 0x80136d2c;
            }
            0x80136d2c => {
                { let v = s(c.f[3] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80136d30;
            }
            0x80136d30 => {
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x80136d34;
            }
            0x80136d34 => {
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x80136d38;
            }
            0x80136d38 => {
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x80136d3c;
            }
            0x80136d3c => {
                c.r[4] = mulhw(c.r[0], c.r[4]);
                pc = 0x80136d40;
            }
            0x80136d40 => {
                c.r[0] = c.r[4].rotate_left(1) & 0x1;
                pc = 0x80136d44;
            }
            0x80136d44 => {
                c.r[27] = c.r[4].wrapping_add(c.r[0]);
                pc = 0x80136d48;
            }
            0x80136d48 => {
                c.r[4] = c.r[27] | c.r[27];
                pc = 0x80136d4c;
            }
            0x80136d4c => {
                c.lr = 0x80136d50; f_8013d604(c);
                pc = 0x80136d50;
            }
            0x80136d50 => {
                c.f[1] = c.f[30];
                pc = 0x80136d54;
            }
            0x80136d54 => {
                c.r[4] = c.r[27] | c.r[27];
                pc = 0x80136d58;
            }
            0x80136d58 => {
                c.f[2] = c.f[30];
                pc = 0x80136d5c;
            }
            0x80136d5c => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136d60;
            }
            0x80136d60 => {
                c.lr = 0x80136d64; f_8013d604(c);
                pc = 0x80136d64;
            }
            0x80136d64 => {
                { let ea = c.r[2].wrapping_add(0xffff9a7c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                pc = 0x80136d68;
            }
            0x80136d68 => {
                c.r[4] = c.r[27] | c.r[27];
                pc = 0x80136d6c;
            }
            0x80136d6c => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136d70;
            }
            0x80136d70 => {
                c.f[2] = c.f[1];
                pc = 0x80136d74;
            }
            0x80136d74 => {
                c.lr = 0x80136d78; f_8013d604(c);
                pc = 0x80136d78;
            }
            0x80136d78 => {
                c.r[29] = c.r[31] | c.r[31];
                pc = 0x80136d7c;
            }
            0x80136d7c => {
                pc = 0x801370ec; continue;
                pc = 0x80136d80;
            }
            0x80136d80 => {
                c.r[4] = c.r[29] | c.r[29];
                pc = 0x80136d84;
            }
            0x80136d84 => {
                c.r[5] = c.r[30] | c.r[30];
                pc = 0x80136d88;
            }
            0x80136d88 => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136d8c;
            }
            0x80136d8c => {
                c.lr = 0x80136d90; f_80137114(c);
                pc = 0x80136d90;
            }
            0x80136d90 => {
                c.r[29] = c.r[3] | c.r[3];
                pc = 0x80136d94;
            }
            0x80136d94 => {
                pc = 0x801370ec; continue;
                pc = 0x80136d98;
            }
            0x80136d98 => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136d9c;
            }
            0x80136d9c => {
                c.r[4] = 0x0;
                pc = 0x80136da0;
            }
            0x80136da0 => {
                c.r[5] = 0x34;
                pc = 0x80136da4;
            }
            0x80136da4 => {
                c.lr = 0x80136da8; f_8000540c(c);
                pc = 0x80136da8;
            }
            0x80136da8 => {
                c.lr = 0x80136dac; f_800d37cc(c);
                pc = 0x80136dac;
            }
            0x80136dac => {
                c.r[4] = 0x43300000;
                pc = 0x80136db0;
            }
            0x80136db0 => {
                c.r[0] = c.r[3] ^ 0x80000000;
                pc = 0x80136db4;
            }
            0x80136db4 => {
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                pc = 0x80136db8;
            }
            0x80136db8 => {
                c.r[3] = c.r[30] ^ 0x80000000;
                pc = 0x80136dbc;
            }
            0x80136dbc => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                pc = 0x80136dc0;
            }
            0x80136dc0 => {
                c.r[0] = 0x0;
                pc = 0x80136dc4;
            }
            0x80136dc4 => {
                c.stw(c.r[1].wrapping_add(0x30), c.r[4]);
                pc = 0x80136dc8;
            }
            0x80136dc8 => {
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136dcc;
            }
            0x80136dcc => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[1] = c.lfd(ea); }
                pc = 0x80136dd0;
            }
            0x80136dd0 => {
                c.stw(c.r[1].wrapping_add(0x2c), c.r[3]);
                pc = 0x80136dd4;
            }
            0x80136dd4 => {
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                pc = 0x80136dd8;
            }
            0x80136dd8 => {
                c.stw(c.r[1].wrapping_add(0x28), c.r[4]);
                pc = 0x80136ddc;
            }
            0x80136ddc => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[1] = c.lfd(ea); }
                pc = 0x80136de0;
            }
            0x80136de0 => {
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x80136de4;
            }
            0x80136de4 => {
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x80136de8;
            }
            0x80136de8 => {
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80136dec;
            }
            0x80136dec => {
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x80136df0;
            }
            0x80136df0 => {
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x80136df4;
            }
            0x80136df4 => {
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x80136df8;
            }
            0x80136df8 => {
                c.sth(c.r[28].wrapping_add(0x3a), c.r[3]);
                pc = 0x80136dfc;
            }
            0x80136dfc => {
                { let ea = c.r[29].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136e00;
            }
            0x80136e00 => {
                { let ea = c.r[28].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136e04;
            }
            0x80136e04 => {
                { let ea = c.r[29].wrapping_add(0x1c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136e08;
            }
            0x80136e08 => {
                { let ea = c.r[28].wrapping_add(0x28); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136e0c;
            }
            0x80136e0c => {
                { let ea = c.r[29].wrapping_add(0x20); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136e10;
            }
            0x80136e10 => {
                { let ea = c.r[28].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136e14;
            }
            0x80136e14 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x4));
                pc = 0x80136e18;
            }
            0x80136e18 => {
                c.stb(c.r[28].wrapping_add(0x20), c.r[3]);
                pc = 0x80136e1c;
            }
            0x80136e1c => {
                c.stb(c.r[28].wrapping_add(0x21), c.r[0]);
                pc = 0x80136e20;
            }
            0x80136e20 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x80136e24;
            }
            0x80136e24 => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x80136e28;
            }
            0x80136e28 => {
                if c.crbit(2) { pc = 0x80136e38; continue; }
                pc = 0x80136e2c;
            }
            0x80136e2c => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x21));
                pc = 0x80136e30;
            }
            0x80136e30 => {
                c.r[0] = c.r[0] | 0x2;
                pc = 0x80136e34;
            }
            0x80136e34 => {
                c.stb(c.r[28].wrapping_add(0x21), c.r[0]);
                pc = 0x80136e38;
            }
            0x80136e38 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x14));
                pc = 0x80136e3c;
            }
            0x80136e3c => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x80136e40;
            }
            0x80136e40 => {
                if c.crbit(2) { pc = 0x80136e50; continue; }
                pc = 0x80136e44;
            }
            0x80136e44 => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x21));
                pc = 0x80136e48;
            }
            0x80136e48 => {
                c.r[0] = c.r[0] | 0x8;
                pc = 0x80136e4c;
            }
            0x80136e4c => {
                c.stb(c.r[28].wrapping_add(0x21), c.r[0]);
                pc = 0x80136e50;
            }
            0x80136e50 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                pc = 0x80136e54;
            }
            0x80136e54 => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x80136e58;
            }
            0x80136e58 => {
                if c.crbit(2) { pc = 0x80136e68; continue; }
                pc = 0x80136e5c;
            }
            0x80136e5c => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x21));
                pc = 0x80136e60;
            }
            0x80136e60 => {
                c.r[0] = c.r[0] | 0x4;
                pc = 0x80136e64;
            }
            0x80136e64 => {
                c.stb(c.r[28].wrapping_add(0x21), c.r[0]);
                pc = 0x80136e68;
            }
            0x80136e68 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x10));
                pc = 0x80136e6c;
            }
            0x80136e6c => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x80136e70;
            }
            0x80136e70 => {
                if c.crbit(2) { pc = 0x80136e80; continue; }
                pc = 0x80136e74;
            }
            0x80136e74 => {
                c.r[0] = c.lbz(c.r[28].wrapping_add(0x21));
                pc = 0x80136e78;
            }
            0x80136e78 => {
                c.r[0] = c.r[0] | 0x1;
                pc = 0x80136e7c;
            }
            0x80136e7c => {
                c.stb(c.r[28].wrapping_add(0x21), c.r[0]);
                pc = 0x80136e80;
            }
            0x80136e80 => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x80136e84;
            }
            0x80136e84 => {
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 8 != 0); c.r[0] = (v >> 24) as u32; }
                pc = 0x80136e88;
            }
            0x80136e88 => {
                c.stb(c.r[28].wrapping_add(0x13), c.r[0]);
                pc = 0x80136e8c;
            }
            0x80136e8c => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x80136e90;
            }
            0x80136e90 => {
                c.r[0] = c.r[0].rotate_left(16) & 0xff;
                pc = 0x80136e94;
            }
            0x80136e94 => {
                c.stb(c.r[28].wrapping_add(0x12), c.r[0]);
                pc = 0x80136e98;
            }
            0x80136e98 => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x80136e9c;
            }
            0x80136e9c => {
                c.r[0] = c.r[0].rotate_left(24) & 0xff;
                pc = 0x80136ea0;
            }
            0x80136ea0 => {
                c.stb(c.r[28].wrapping_add(0x11), c.r[0]);
                pc = 0x80136ea4;
            }
            0x80136ea4 => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x80136ea8;
            }
            0x80136ea8 => {
                c.r[29] = c.r[29].wrapping_add(0x28);
                pc = 0x80136eac;
            }
            0x80136eac => {
                c.stb(c.r[28].wrapping_add(0x10), c.r[0]);
                pc = 0x80136eb0;
            }
            0x80136eb0 => {
                pc = 0x801370ec; continue;
                pc = 0x80136eb4;
            }
            0x80136eb4 => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136eb8;
            }
            0x80136eb8 => {
                c.r[4] = 0x0;
                pc = 0x80136ebc;
            }
            0x80136ebc => {
                c.r[5] = 0x24;
                pc = 0x80136ec0;
            }
            0x80136ec0 => {
                c.lr = 0x80136ec4; f_8000540c(c);
                pc = 0x80136ec4;
            }
            0x80136ec4 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x8));
                pc = 0x80136ec8;
            }
            0x80136ec8 => {
                c.sth(c.r[28].wrapping_add(0x24), c.r[0]);
                pc = 0x80136ecc;
            }
            0x80136ecc => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x10));
                pc = 0x80136ed0;
            }
            0x80136ed0 => {
                c.cmps(0, c.r[0], 0x1);
                pc = 0x80136ed4;
            }
            0x80136ed4 => {
                if c.crbit(2) { pc = 0x80136ef4; continue; }
                pc = 0x80136ed8;
            }
            0x80136ed8 => {
                if !c.crbit(0) { pc = 0x80136efc; continue; }
                pc = 0x80136edc;
            }
            0x80136edc => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x80136ee0;
            }
            0x80136ee0 => {
                if !c.crbit(0) { pc = 0x80136ee8; continue; }
                pc = 0x80136ee4;
            }
            0x80136ee4 => {
                pc = 0x80136efc; continue;
                pc = 0x80136ee8;
            }
            0x80136ee8 => {
                c.r[0] = 0x0;
                pc = 0x80136eec;
            }
            0x80136eec => {
                c.stw(c.r[28].wrapping_add(0x20), c.r[0]);
                pc = 0x80136ef0;
            }
            0x80136ef0 => {
                pc = 0x80136efc; continue;
                pc = 0x80136ef4;
            }
            0x80136ef4 => {
                c.r[0] = 0x1;
                pc = 0x80136ef8;
            }
            0x80136ef8 => {
                c.stw(c.r[28].wrapping_add(0x20), c.r[0]);
                pc = 0x80136efc;
            }
            0x80136efc => {
                c.r[4] = c.lwz(c.r[29].wrapping_add(0xc));
                pc = 0x80136f00;
            }
            0x80136f00 => {
                c.r[3] = c.r[29].wrapping_add(0x37);
                pc = 0x80136f04;
            }
            0x80136f04 => {
                c.r[0] = 0x4e20;
                pc = 0x80136f08;
            }
            0x80136f08 => {
                c.sth(c.r[28].wrapping_add(0x26), c.r[4]);
                pc = 0x80136f0c;
            }
            0x80136f0c => {
                c.r[27] = c.r[3].rotate_left(0) & 0xffffffe0;
                pc = 0x80136f10;
            }
            0x80136f10 => {
                c.stw(c.r[28].wrapping_add(0x8), c.r[0]);
                pc = 0x80136f14;
            }
            0x80136f14 => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x80136f18;
            }
            0x80136f18 => {
                c.cmps(0, c.r[0], 0x0);
                pc = 0x80136f1c;
            }
            0x80136f1c => {
                if c.crbit(2) { pc = 0x80136f90; continue; }
                pc = 0x80136f20;
            }
            0x80136f20 => {
                c.lr = 0x80136f24; f_801db060(c);
                pc = 0x80136f24;
            }
            0x80136f24 => {
                c.stw(c.r[28].wrapping_add(0xc), c.r[3]);
                pc = 0x80136f28;
            }
            0x80136f28 => {
                c.lr = 0x80136f2c; f_801db060(c);
                pc = 0x80136f2c;
            }
            0x80136f2c => {
                c.stw(c.r[28].wrapping_add(0x10), c.r[3]);
                pc = 0x80136f30;
            }
            0x80136f30 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x80136f34;
            }
            0x80136f34 => {
                c.r[5] = 0x4e20;
                pc = 0x80136f38;
            }
            0x80136f38 => {
                c.r[4] = c.lwz(c.r[29]);
                pc = 0x80136f3c;
            }
            0x80136f3c => {
                c.r[6] = c.lwz(c.r[28].wrapping_add(0xc));
                pc = 0x80136f40;
            }
            0x80136f40 => {
                c.lr = 0x80136f44; f_8010147c(c);
                pc = 0x80136f44;
            }
            0x80136f44 => {
                c.r[4] = c.lwz(c.r[28].wrapping_add(0xc));
                pc = 0x80136f48;
            }
            0x80136f48 => {
                c.r[3] = 0x4e20;
                pc = 0x80136f4c;
            }
            0x80136f4c => {
                c.lr = 0x80136f50; f_800f9318(c);
                pc = 0x80136f50;
            }
            0x80136f50 => {
                c.r[6] = c.lwz(c.r[28].wrapping_add(0x10));
                pc = 0x80136f54;
            }
            0x80136f54 => {
                c.r[4] = 0x4e20;
                pc = 0x80136f58;
            }
            0x80136f58 => {
                c.r[5] = 0x0;
                pc = 0x80136f5c;
            }
            0x80136f5c => {
                c.lr = 0x80136f60; f_801013a0(c);
                pc = 0x80136f60;
            }
            0x80136f60 => {
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x10));
                pc = 0x80136f64;
            }
            0x80136f64 => {
                c.r[3] = 0x4e20;
                pc = 0x80136f68;
            }
            0x80136f68 => {
                c.lr = 0x80136f6c; f_800f9318(c);
                pc = 0x80136f6c;
            }
            0x80136f6c => {
                c.cmpu(0, c.r[3], 0x0);
                pc = 0x80136f70;
            }
            0x80136f70 => {
                if c.crbit(2) { pc = 0x80136f7c; continue; }
                pc = 0x80136f74;
            }
            0x80136f74 => {
                c.r[4] = 0x0;
                pc = 0x80136f78;
            }
            0x80136f78 => {
                c.lr = 0x80136f7c; f_800e4014(c);
                pc = 0x80136f7c;
            }
            0x80136f7c => {
                c.r[3] = c.lwz(c.r[29]);
                pc = 0x80136f80;
            }
            0x80136f80 => {
                c.r[0] = c.r[3].wrapping_add(0x1f);
                pc = 0x80136f84;
            }
            0x80136f84 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x80136f88;
            }
            0x80136f88 => {
                c.r[27] = c.r[27].wrapping_add(c.r[0]);
                pc = 0x80136f8c;
            }
            0x80136f8c => {
                pc = 0x80136f9c; continue;
                pc = 0x80136f90;
            }
            0x80136f90 => {
                c.r[0] = 0x0;
                pc = 0x80136f94;
            }
            0x80136f94 => {
                c.stw(c.r[28].wrapping_add(0xc), c.r[0]);
                pc = 0x80136f98;
            }
            0x80136f98 => {
                c.stw(c.r[28].wrapping_add(0x10), c.r[0]);
                pc = 0x80136f9c;
            }
            0x80136f9c => {
                c.r[0] = 0x0;
                pc = 0x80136fa0;
            }
            0x80136fa0 => {
                c.r[29] = c.r[27] | c.r[27];
                pc = 0x80136fa4;
            }
            0x80136fa4 => {
                c.stw(c.r[28].wrapping_add(0x14), c.r[0]);
                pc = 0x80136fa8;
            }
            0x80136fa8 => {
                pc = 0x801370ec; continue;
                pc = 0x80136fac;
            }
            0x80136fac => {
                c.r[3] = c.r[28].wrapping_add(0x8);
                pc = 0x80136fb0;
            }
            0x80136fb0 => {
                c.r[27] = 0x0;
                pc = 0x80136fb4;
            }
            0x80136fb4 => {
                c.r[4] = 0x0;
                pc = 0x80136fb8;
            }
            0x80136fb8 => {
                c.r[5] = 0xb4;
                pc = 0x80136fbc;
            }
            0x80136fbc => {
                c.lr = 0x80136fc0; f_8000540c(c);
                pc = 0x80136fc0;
            }
            0x80136fc0 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x14));
                pc = 0x80136fc4;
            }
            0x80136fc4 => {
                c.cmps(0, c.r[0], 0x2);
                pc = 0x80136fc8;
            }
            0x80136fc8 => {
                if c.crbit(2) { pc = 0x80136fec; continue; }
                pc = 0x80136fcc;
            }
            0x80136fcc => {
                if !c.crbit(0) { pc = 0x80136ff4; continue; }
                pc = 0x80136fd0;
            }
            0x80136fd0 => {
                c.cmps(0, c.r[0], 0x1);
                pc = 0x80136fd4;
            }
            0x80136fd4 => {
                if !c.crbit(0) { pc = 0x80136fdc; continue; }
                pc = 0x80136fd8;
            }
            0x80136fd8 => {
                pc = 0x80136ff4; continue;
                pc = 0x80136fdc;
            }
            0x80136fdc => {
                { let ea = c.r[2].wrapping_add(0xffff9a80); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136fe0;
            }
            0x80136fe0 => {
                c.r[27] = 0xfffffffc;
                pc = 0x80136fe4;
            }
            0x80136fe4 => {
                { let ea = c.r[28].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136fe8;
            }
            0x80136fe8 => {
                pc = 0x80136ff4; continue;
                pc = 0x80136fec;
            }
            0x80136fec => {
                { let ea = c.r[29].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80136ff0;
            }
            0x80136ff0 => {
                { let ea = c.r[28].wrapping_add(0x24); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80136ff4;
            }
            0x80136ff4 => {
                c.lr = 0x80136ff8; f_800d37cc(c);
                pc = 0x80136ff8;
            }
            0x80136ff8 => {
                c.r[5] = 0x43300000;
                pc = 0x80136ffc;
            }
            0x80136ffc => {
                c.r[0] = c.r[3] ^ 0x80000000;
                pc = 0x80137000;
            }
            0x80137000 => {
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                pc = 0x80137004;
            }
            0x80137004 => {
                c.r[3] = c.r[30] ^ 0x80000000;
                pc = 0x80137008;
            }
            0x80137008 => {
                c.r[4] = c.r[27].wrapping_add(c.r[29]);
                pc = 0x8013700c;
            }
            0x8013700c => {
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                pc = 0x80137010;
            }
            0x80137010 => {
                c.stw(c.r[1].wrapping_add(0x30), c.r[5]);
                pc = 0x80137014;
            }
            0x80137014 => {
                c.r[0] = c.r[4].wrapping_add(0x3b);
                pc = 0x80137018;
            }
            0x80137018 => {
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x8013701c;
            }
            0x8013701c => {
                c.r[31] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x80137020;
            }
            0x80137020 => {
                { let ea = c.r[1].wrapping_add(0x30); c.f[1] = c.lfd(ea); }
                pc = 0x80137024;
            }
            0x80137024 => {
                c.r[4] = 0x20;
                pc = 0x80137028;
            }
            0x80137028 => {
                c.stw(c.r[1].wrapping_add(0x2c), c.r[3]);
                pc = 0x8013702c;
            }
            0x8013702c => {
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                pc = 0x80137030;
            }
            0x80137030 => {
                c.stw(c.r[1].wrapping_add(0x28), c.r[5]);
                pc = 0x80137034;
            }
            0x80137034 => {
                { let ea = c.r[1].wrapping_add(0x28); c.f[1] = c.lfd(ea); }
                pc = 0x80137038;
            }
            0x80137038 => {
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x8013703c;
            }
            0x8013703c => {
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                pc = 0x80137040;
            }
            0x80137040 => {
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                pc = 0x80137044;
            }
            0x80137044 => {
                c.f[0] = fctiwz(c.f[0]);
                pc = 0x80137048;
            }
            0x80137048 => {
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                pc = 0x8013704c;
            }
            0x8013704c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                pc = 0x80137050;
            }
            0x80137050 => {
                c.sth(c.r[28].wrapping_add(0xba), c.r[0]);
                pc = 0x80137054;
            }
            0x80137054 => {
                { let ea = c.r[29].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80137058;
            }
            0x80137058 => {
                { let ea = c.r[28].wrapping_add(0x20); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8013705c;
            }
            0x8013705c => {
                { let ea = c.r[28].wrapping_add(0x20); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80137060;
            }
            0x80137060 => {
                { let ea = c.r[28].wrapping_add(0x1c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x80137064;
            }
            0x80137064 => {
                { let ea = c.r[29].wrapping_add(0xc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                pc = 0x80137068;
            }
            0x80137068 => {
                { let ea = c.r[28].wrapping_add(0x2c); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x8013706c;
            }
            0x8013706c => {
                c.r[0] = c.lwz(c.r[29]);
                pc = 0x80137070;
            }
            0x80137070 => {
                c.sth(c.r[28].wrapping_add(0x30), c.r[0]);
                pc = 0x80137074;
            }
            0x80137074 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x4));
                pc = 0x80137078;
            }
            0x80137078 => {
                c.sth(c.r[28].wrapping_add(0x32), c.r[0]);
                pc = 0x8013707c;
            }
            0x8013707c => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                pc = 0x80137080;
            }
            0x80137080 => {
                c.r[0] = c.r[3].wrapping_add(0x1f);
                pc = 0x80137084;
            }
            0x80137084 => {
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x80137088;
            }
            0x80137088 => {
                c.lr = 0x8013708c; f_800e2c04(c);
                pc = 0x8013708c;
            }
            0x8013708c => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                pc = 0x80137090;
            }
            0x80137090 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x80137094;
            }
            0x80137094 => {
                if c.crbit(2) { pc = 0x801370d0; continue; }
                pc = 0x80137098;
            }
            0x80137098 => {
                c.lr = 0x8013709c; f_800e27b0(c);
                pc = 0x8013709c;
            }
            0x8013709c => {
                c.r[5] = c.lwz(c.r[29].wrapping_add(0x10));
                pc = 0x801370a0;
            }
            0x801370a0 => {
                c.r[27] = c.r[3] | c.r[3];
                pc = 0x801370a4;
            }
            0x801370a4 => {
                c.r[4] = c.r[31] | c.r[31];
                pc = 0x801370a8;
            }
            0x801370a8 => {
                c.r[0] = c.r[5].wrapping_add(0x1f);
                pc = 0x801370ac;
            }
            0x801370ac => {
                c.r[5] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x801370b0;
            }
            0x801370b0 => {
                c.lr = 0x801370b4; f_800054f4(c);
                pc = 0x801370b4;
            }
            0x801370b4 => {
                c.r[3] = c.r[27] | c.r[27];
                pc = 0x801370b8;
            }
            0x801370b8 => {
                c.lr = 0x801370bc; f_800efd3c(c);
                pc = 0x801370bc;
            }
            0x801370bc => {
                c.stw(c.r[28].wrapping_add(0xc), c.r[3]);
                pc = 0x801370c0;
            }
            0x801370c0 => {
                c.r[4] = c.r[30] | c.r[30];
                pc = 0x801370c4;
            }
            0x801370c4 => {
                c.r[3] = c.lwz(c.r[28].wrapping_add(0xc));
                pc = 0x801370c8;
            }
            0x801370c8 => {
                c.lr = 0x801370cc; f_800efd14(c);
                pc = 0x801370cc;
            }
            0x801370cc => {
                pc = 0x801370d8; continue;
                pc = 0x801370d0;
            }
            0x801370d0 => {
                c.r[0] = 0x0;
                pc = 0x801370d4;
            }
            0x801370d4 => {
                c.stw(c.r[28].wrapping_add(0xc), c.r[0]);
                pc = 0x801370d8;
            }
            0x801370d8 => {
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x10));
                pc = 0x801370dc;
            }
            0x801370dc => {
                c.r[0] = c.r[3].wrapping_add(0x1f);
                pc = 0x801370e0;
            }
            0x801370e0 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffe0;
                pc = 0x801370e4;
            }
            0x801370e4 => {
                c.r[31] = c.r[31].wrapping_add(c.r[0]);
                pc = 0x801370e8;
            }
            0x801370e8 => {
                c.r[29] = c.r[31] | c.r[31];
                pc = 0x801370ec;
            }
            0x801370ec => {
                c.r[3] = c.r[29] | c.r[29];
                pc = 0x801370f0;
            }
            0x801370f0 => {
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                pc = 0x801370f4;
            }
            0x801370f4 => {
                { let ea = c.r[1].wrapping_add(0x60); c.f[31] = c.lfd(ea); }
                pc = 0x801370f8;
            }
            0x801370f8 => {
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                pc = 0x801370fc;
            }
            0x801370fc => {
                { let ea = c.r[1].wrapping_add(0x50); c.f[30] = c.lfd(ea); }
                pc = 0x80137100;
            }
            0x80137100 => {
                { let mut ea = c.r[1].wrapping_add(0x3c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                pc = 0x80137104;
            }
            0x80137104 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x74));
                pc = 0x80137108;
            }
            0x80137108 => {
                c.mtspr(8, c.r[0]);
                pc = 0x8013710c;
            }
            0x8013710c => {
                c.r[1] = c.r[1].wrapping_add(0x70);
                pc = 0x80137110;
            }
            0x80137110 => {
                if true { return; }
                return;
            }
            _ => panic!("f_801364a8: bad pc {:#x}", pc),
        }
    }
}

/// fn_80137114 (0x80137114, 584 bytes)
#[inline(never)]
pub fn f_80137114(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80137114) { return; }
        if c.prof { crate::prof::enter(c, 0x80137114); f_80137114_b(c, 0x80137114); crate::prof::exit(c, 0x80137114); return; }
    }
    f_80137114_b(c, 0x80137114)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80137114_at(c: &mut Cpu, pc: u32) { f_80137114_b(c, pc) }
#[inline(always)]
fn f_80137114_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80137114;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80137114, pc); }
        match pc {
            0x80137114 => {
                { let ea = c.r[1].wrapping_add(0xffffffa0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x64), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x40); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[31] = c.lwz(c.r[4]);
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[5] = 0x20;
                c.lr = 0x80137150; f_8000540c(c);
                pc = 0x80137150;
            }
            0x80137150 => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x4));
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x801371fc; continue; }
                pc = 0x8013715c;
            }
            0x8013715c => {
                if !c.crbit(0) { pc = 0x8013716c; continue; }
                pc = 0x80137160;
            }
            0x80137160 => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(0) { pc = 0x80137284; continue; }
                pc = 0x80137168;
            }
            0x80137168 => {
                pc = 0x80137334; continue;
                pc = 0x8013716c;
            }
            0x8013716c => {
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(0) { pc = 0x80137334; continue; }
                pc = 0x80137174;
            }
            0x80137174 => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x8));
                c.r[30] = c.r[28].wrapping_add(0xc);
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[31] = c.lfd(ea); }
                c.r[27] = 0x0;
                c.stw(c.r[29].wrapping_add(0x10), c.r[0]);
                c.r[28] = 0x43300000;
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                pc = 0x801371f0; continue;
                pc = 0x80137194;
            }
            0x80137194 => {
                c.lr = 0x80137198; f_800d37cc(c);
                pc = 0x80137198;
            }
            0x80137198 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x8));
                c.r[3] = c.r[3] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.r[0] ^ 0x80000000;
                { let ea = c.r[30]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                { let ea = c.r[30].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x10); c.f[0] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[0]);
                { let v = s(c.f[0] - c.f[31]); c.f[3] = v; c.ps1[3] = v; }
                c.stw(c.r[1].wrapping_add(0x18), c.r[28]);
                { let ea = c.r[1].wrapping_add(0x18); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[31]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] * c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] / c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x24));
                c.lr = 0x801371e8; f_8013db64(c);
                pc = 0x801371e8;
            }
            0x801371e8 => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.r[30] = c.r[30].wrapping_add(0x10);
                pc = 0x801371f0;
            }
            0x801371f0 => {
                c.cmps(0, c.r[27], c.r[31]);
                if c.crbit(0) { pc = 0x80137194; continue; }
                pc = 0x801371f8;
            }
            0x801371f8 => {
                pc = 0x80137334; continue;
                pc = 0x801371fc;
            }
            0x801371fc => {
                c.r[0] = 0x1;
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[30] = c.lfd(ea); }
                c.stw(c.r[29].wrapping_add(0x10), c.r[0]);
                c.r[30] = c.r[28].wrapping_add(0x8);
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                c.r[27] = 0x0;
                c.r[28] = 0x43300000;
                pc = 0x80137278; continue;
                pc = 0x8013721c;
            }
            0x8013721c => {
                c.lr = 0x80137220; f_800d37cc(c);
                pc = 0x80137220;
            }
            0x80137220 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x8));
                c.r[3] = c.r[3] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[3]);
                c.r[3] = c.r[29] | c.r[29];
                c.r[0] = c.r[0] ^ 0x80000000;
                { let ea = c.r[30]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0x20), c.r[28]);
                { let ea = c.r[30].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x20); c.f[0] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x1c), c.r[0]);
                { let v = s(c.f[0] - c.f[30]); c.f[3] = v; c.ps1[3] = v; }
                c.stw(c.r[1].wrapping_add(0x18), c.r[28]);
                { let ea = c.r[1].wrapping_add(0x18); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[30]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] * c.f[3]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[0] / c.f[31]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[0]; c.stfd(ea, v); }
                c.r[4] = c.lwz(c.r[1].wrapping_add(0x14));
                c.lr = 0x80137270; f_8013db64(c);
                pc = 0x80137270;
            }
            0x80137270 => {
                c.r[27] = c.r[27].wrapping_add(0x1);
                c.r[30] = c.r[30].wrapping_add(0x10);
                pc = 0x80137278;
            }
            0x80137278 => {
                c.cmps(0, c.r[27], c.r[31]);
                if c.crbit(0) { pc = 0x8013721c; continue; }
                pc = 0x80137280;
            }
            0x80137280 => {
                pc = 0x80137334; continue;
                pc = 0x80137284;
            }
            0x80137284 => {
                c.r[0] = 0x1;
                c.stw(c.r[29].wrapping_add(0x10), c.r[0]);
                c.r[0] = c.lwz(c.r[28]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[30] = c.lfs(ea); c.ps1[30] = c.f[30]; }
                c.lr = 0x8013729c; f_800d37cc(c);
                pc = 0x8013729c;
            }
            0x8013729c => {
                c.r[0] = c.r[3] ^ 0x80000000;
                c.r[5] = 0x43300000;
                c.r[4] = c.r[27] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.r[3] = 0x55550000;
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[5] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x20), c.r[5]);
                c.r[0] = c.r[3].wrapping_add(0x5556);
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[2] = c.f[30];
                { let ea = c.r[1].wrapping_add(0x20); c.f[1] = c.lfd(ea); }
                c.r[3] = c.r[29] | c.r[29];
                c.stw(c.r[1].wrapping_add(0x1c), c.r[4]);
                { let v = s(c.f[1] - c.f[5]); c.f[4] = v; c.ps1[4] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9a7c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.stw(c.r[1].wrapping_add(0x18), c.r[5]);
                { let ea = c.r[1].wrapping_add(0x18); c.f[3] = c.lfd(ea); }
                { let v = s(c.f[3] - c.f[5]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[3] * c.f[4]); c.f[3] = v; c.ps1[3] = v; }
                { let v = s(c.f[3] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x10); let v = c.f[0]; c.stfd(ea, v); }
                c.r[27] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[4] = mulhw(c.r[0], c.r[27]);
                c.r[0] = c.r[4].rotate_left(1) & 0x1;
                c.r[28] = c.r[4].wrapping_add(c.r[0]);
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x8013730c; f_8013db64(c);
                pc = 0x8013730c;
            }
            0x8013730c => {
                c.f[1] = c.f[30];
                c.r[3] = c.r[29] | c.r[29];
                c.f[2] = c.f[30];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80137320; f_8013db64(c);
                pc = 0x80137320;
            }
            0x80137320 => {
                { let ea = c.r[2].wrapping_add(0xffff9a7c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[28] | c.r[28];
                c.f[2] = c.f[1];
                c.lr = 0x80137334; f_8013db64(c);
                pc = 0x80137334;
            }
            0x80137334 => {
                c.r[3] = c.r[30] | c.r[30];
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x48); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x40); c.f[30] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x64));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x60);
                if true { return; }
                return;
            }
            _ => panic!("f_80137114: bad pc {:#x}", pc),
        }
    }
}

/// fn_8013735C (0x8013735c, 544 bytes)
#[inline(never)]
pub fn f_8013735c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013735c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013735c); f_8013735c_b(c, 0x8013735c); crate::prof::exit(c, 0x8013735c); return; }
    }
    f_8013735c_b(c, 0x8013735c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013735c_at(c: &mut Cpu, pc: u32) { f_8013735c_b(c, pc) }
#[inline(always)]
fn f_8013735c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013735c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013735c, pc); }
        match pc {
            0x8013735c => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[28] = c.r[3] | c.r[3];
                c.r[30] = 0x0;
                c.r[31] = c.r[29] | c.r[29];
                c.r[4] = 0x0;
                c.r[5] = 0xac;
                c.lr = 0x8013738c; f_8000540c(c);
                pc = 0x8013738c;
            }
            0x8013738c => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x3c));
                c.cmps(0, c.r[0], 0x2);
                if c.crbit(2) { pc = 0x801373b8; continue; }
                pc = 0x80137398;
            }
            0x80137398 => {
                if !c.crbit(0) { pc = 0x801373b8; continue; }
                pc = 0x8013739c;
            }
            0x8013739c => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(0) { pc = 0x801373a8; continue; }
                pc = 0x801373a4;
            }
            0x801373a4 => {
                pc = 0x801373b8; continue;
                pc = 0x801373a8;
            }
            0x801373a8 => {
                c.r[0] = 0x0;
                c.r[30] = 0xfffffffc;
                c.stw(c.r[28].wrapping_add(0xa8), c.r[0]);
                pc = 0x801373c0; continue;
                pc = 0x801373b8;
            }
            0x801373b8 => {
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x40));
                c.stw(c.r[28].wrapping_add(0xa8), c.r[0]);
                pc = 0x801373c0;
            }
            0x801373c0 => {
                c.lr = 0x801373c4; f_800d37cc(c);
                pc = 0x801373c4;
            }
            0x801373c4 => {
                c.r[4] = 0x43300000;
                c.r[3] = c.r[3] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[0] = c.r[27] ^ 0x80000000;
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x8), c.r[4]);
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                c.stw(c.r[1].wrapping_add(0x10), c.r[4]);
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.sth(c.r[28].wrapping_add(0xa6), c.r[0]);
                { let ea = c.r[31]; c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28].wrapping_add(0x48); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28].wrapping_add(0x4c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28].wrapping_add(0x50); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                { let v = c.r[0] as i32; c.set_ca(v < 0 && (v as u32) << 8 != 0); c.r[0] = (v >> 24) as u32; }
                c.stb(c.r[28].wrapping_add(0x63), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(16) & 0xff;
                c.stb(c.r[28].wrapping_add(0x62), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.r[0] = c.r[0].rotate_left(24) & 0xff;
                c.stb(c.r[28].wrapping_add(0x61), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0xc));
                c.stb(c.r[28].wrapping_add(0x60), c.r[0]);
                { let ea = c.r[31].wrapping_add(0x10); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28].wrapping_add(0x64); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x14); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28].wrapping_add(0x68); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x18); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28].wrapping_add(0x6c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x1c));
                c.sth(c.r[28].wrapping_add(0x70), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x20));
                c.sth(c.r[28].wrapping_add(0x72), c.r[0]);
                c.r[4] = c.lhz(c.r[28].wrapping_add(0x70));
                c.r[3] = c.r[4].rotate_left(1) & 0x1;
                c.r[0] = c.r[4].rotate_left(0) & 0x1;
                c.r[0] = c.r[0] ^ c.r[3];
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8013749c; continue; }
                pc = 0x80137494;
            }
            0x80137494 => {
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.sth(c.r[28].wrapping_add(0x70), c.r[0]);
                pc = 0x8013749c;
            }
            0x8013749c => {
                c.r[4] = c.lhz(c.r[28].wrapping_add(0x72));
                c.r[3] = c.r[4].rotate_left(1) & 0x1;
                c.r[0] = c.r[4].rotate_left(0) & 0x1;
                c.r[0] = c.r[0] ^ c.r[3];
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x801374bc; continue; }
                pc = 0x801374b4;
            }
            0x801374b4 => {
                c.r[0] = c.r[4].wrapping_add(0x1);
                c.sth(c.r[28].wrapping_add(0x72), c.r[0]);
                pc = 0x801374bc;
            }
            0x801374bc => {
                { let ea = c.r[31].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[29] = c.r[30].wrapping_add(c.r[29]);
                c.r[3] = c.r[29].wrapping_add(0x63);
                c.r[0] = 0x4e20;
                { let ea = c.r[28].wrapping_add(0x90); let v = c.f[0]; c.stfs(ea, v); }
                c.r[29] = c.r[3].rotate_left(0) & 0xffffffe0;
                { let ea = c.r[31].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28].wrapping_add(0x94); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28].wrapping_add(0x98); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x30); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28].wrapping_add(0x9c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x34); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[28].wrapping_add(0xa0); let v = c.f[0]; c.stfs(ea, v); }
                c.stw(c.r[28].wrapping_add(0x74), c.r[0]);
                c.lr = 0x801374fc; f_801db060(c);
                pc = 0x801374fc;
            }
            0x801374fc => {
                c.stw(c.r[28].wrapping_add(0x7c), c.r[3]);
                c.lr = 0x80137504; f_801db060(c);
                pc = 0x80137504;
            }
            0x80137504 => {
                c.stw(c.r[28].wrapping_add(0x78), c.r[3]);
                c.r[3] = c.r[29] | c.r[29];
                c.r[5] = 0x4e20;
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x38));
                c.r[6] = c.lwz(c.r[28].wrapping_add(0x7c));
                c.lr = 0x8013751c; f_8010147c(c);
                pc = 0x8013751c;
            }
            0x8013751c => {
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x7c));
                c.r[3] = 0x4e20;
                c.lr = 0x80137528; f_800f9318(c);
                pc = 0x80137528;
            }
            0x80137528 => {
                c.r[6] = c.lwz(c.r[28].wrapping_add(0x78));
                c.r[4] = 0x4e20;
                c.r[5] = 0x0;
                c.lr = 0x80137538; f_801013a0(c);
                pc = 0x80137538;
            }
            0x80137538 => {
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x78));
                c.r[3] = 0x4e20;
                c.lr = 0x80137544; f_800f9318(c);
                pc = 0x80137544;
            }
            0x80137544 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80137554; continue; }
                pc = 0x8013754c;
            }
            0x8013754c => {
                c.r[4] = 0x0;
                c.lr = 0x80137554; f_800e4014(c);
                pc = 0x80137554;
            }
            0x80137554 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x38));
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[29] = c.r[29].wrapping_add(c.r[0]);
                c.r[3] = c.r[29] | c.r[29];
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_8013735c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8013757C (0x8013757c, 516 bytes)
#[inline(never)]
pub fn f_8013757c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013757c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013757c); f_8013757c_b(c, 0x8013757c); crate::prof::exit(c, 0x8013757c); return; }
    }
    f_8013757c_b(c, 0x8013757c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013757c_at(c: &mut Cpu, pc: u32) { f_8013757c_b(c, pc) }
#[inline(always)]
fn f_8013757c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013757c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013757c, pc); }
        match pc {
            0x8013757c => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[27] = c.r[5] | c.r[5];
                c.r[29] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[31] = c.r[30] | c.r[30];
                c.r[5] = 0x70;
                c.lr = 0x801375a8; f_8000540c(c);
                pc = 0x801375a8;
            }
            0x801375a8 => {
                c.lr = 0x801375ac; f_800d37cc(c);
                pc = 0x801375ac;
            }
            0x801375ac => {
                c.r[5] = 0x43300000;
                c.r[3] = c.r[3] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0xc), c.r[3]);
                c.r[0] = c.r[27] ^ 0x80000000;
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                c.r[3] = c.r[29].wrapping_add(0x34);
                c.stw(c.r[1].wrapping_add(0x8), c.r[5]);
                c.r[4] = c.r[31].wrapping_add(0xc);
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                c.stw(c.r[1].wrapping_add(0x10), c.r[5]);
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x18); let v = c.f[0]; c.stfd(ea, v); }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x1c));
                c.sth(c.r[29].wrapping_add(0x12), c.r[0]);
                c.r[0] = c.lwz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(8) & 0xff;
                c.stb(c.r[29].wrapping_add(0x23), c.r[0]);
                c.r[0] = c.lwz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(16) & 0xff;
                c.stb(c.r[29].wrapping_add(0x22), c.r[0]);
                c.r[0] = c.lwz(c.r[31]);
                c.r[0] = c.r[0].rotate_left(24) & 0xff;
                c.stb(c.r[29].wrapping_add(0x21), c.r[0]);
                c.r[0] = c.lwz(c.r[31]);
                c.stb(c.r[29].wrapping_add(0x20), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(8) & 0xff;
                c.stb(c.r[29].wrapping_add(0x27), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(16) & 0xff;
                c.stb(c.r[29].wrapping_add(0x26), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(24) & 0xff;
                c.stb(c.r[29].wrapping_add(0x25), c.r[0]);
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x4));
                c.stb(c.r[29].wrapping_add(0x24), c.r[0]);
                c.lr = 0x8013765c; f_800e01d0(c);
                pc = 0x8013765c;
            }
            0x8013765c => {
                c.r[3] = c.r[29].wrapping_add(0x40);
                c.r[4] = c.r[31].wrapping_add(0x18);
                c.lr = 0x80137668; f_800e01d0(c);
                pc = 0x80137668;
            }
            0x80137668 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x8));
                c.r[0] = c.r[30].wrapping_add(0x73);
                c.r[30] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.r[4] = 0x20;
                c.sth(c.r[29].wrapping_add(0x8), c.r[3]);
                { let ea = c.r[31].wrapping_add(0x24); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x4c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x28); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x50); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x2c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x54); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x30); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x58); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x34); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x5c); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x38); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x60); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x3c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x64); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x40); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x68); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[31].wrapping_add(0x44); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[29].wrapping_add(0x6c); let v = c.f[0]; c.stfs(ea, v); }
                c.r[0] = c.lwz(c.r[31].wrapping_add(0x48));
                c.sth(c.r[29].wrapping_add(0xe), c.r[0]);
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x4c));
                c.r[0] = c.r[3].wrapping_add(0x1f);
                c.r[3] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.lr = 0x801376dc; f_800e2c04(c);
                pc = 0x801376dc;
            }
            0x801376dc => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.r[28] = c.r[3] | c.r[3];
                if c.crbit(2) { pc = 0x80137720; continue; }
                pc = 0x801376e8;
            }
            0x801376e8 => {
                c.lr = 0x801376ec; f_800e27b0(c);
                pc = 0x801376ec;
            }
            0x801376ec => {
                c.r[5] = c.lwz(c.r[31].wrapping_add(0x4c));
                c.r[27] = c.r[3] | c.r[3];
                c.r[4] = c.r[30] | c.r[30];
                c.r[0] = c.r[5].wrapping_add(0x1f);
                c.r[5] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.lr = 0x80137704; f_800054f4(c);
                pc = 0x80137704;
            }
            0x80137704 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x8013770c; f_800efd3c(c);
                pc = 0x8013770c;
            }
            0x8013770c => {
                c.stw(c.r[29].wrapping_add(0x1c), c.r[3]);
                c.r[4] = c.r[28] | c.r[28];
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x1c));
                c.lr = 0x8013771c; f_800efd14(c);
                pc = 0x8013771c;
            }
            0x8013771c => {
                pc = 0x80137728; continue;
                pc = 0x80137720;
            }
            0x80137720 => {
                c.r[0] = 0x0;
                c.stw(c.r[29].wrapping_add(0x1c), c.r[0]);
                pc = 0x80137728;
            }
            0x80137728 => {
                c.r[4] = c.lwz(c.r[31].wrapping_add(0x4c));
                c.r[3] = c.lwz(c.r[29].wrapping_add(0x1c));
                c.r[0] = c.r[4].wrapping_add(0x1f);
                c.r[0] = c.r[0].rotate_left(0) & 0xffffffe0;
                c.cmpu(0, c.r[3], 0x0);
                c.r[30] = c.r[30].wrapping_add(c.r[0]);
                if c.crbit(2) { pc = 0x80137750; continue; }
                pc = 0x80137744;
            }
            0x80137744 => {
                c.r[4] = 0x2;
                c.r[5] = 0x2;
                c.lr = 0x80137750; f_800ef590(c);
                pc = 0x80137750;
            }
            0x80137750 => {
                c.r[3] = 0x80310000;
                c.r[4] = 0x80310000;
                c.r[0] = c.r[3].wrapping_add(0x4638);
                c.r[3] = c.r[30] | c.r[30];
                c.stw(c.r[29].wrapping_add(0x14), c.r[0]);
                c.r[0] = c.r[4].wrapping_add(0x4ae8);
                c.stw(c.r[29].wrapping_add(0x18), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x2c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_8013757c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80137780 (0x80137780, 492 bytes)
#[inline(never)]
pub fn f_80137780(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80137780) { return; }
        if c.prof { crate::prof::enter(c, 0x80137780); f_80137780_b(c, 0x80137780); crate::prof::exit(c, 0x80137780); return; }
    }
    f_80137780_b(c, 0x80137780)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80137780_at(c: &mut Cpu, pc: u32) { f_80137780_b(c, pc) }
#[inline(always)]
fn f_80137780_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80137780;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80137780, pc); }
        match pc {
            0x80137780 => {
                { let ea = c.r[1].wrapping_add(0xffffffc0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[5] = 0x5c;
                c.stw(c.r[1].wrapping_add(0x44), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x28); for n in 26..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[30] = c.r[4] | c.r[4];
                c.r[27] = c.r[3] | c.r[3];
                c.r[26] = 0x0;
                c.r[28] = 0x1;
                c.r[29] = c.lwz(c.r[4].wrapping_add(0x4));
                c.r[4] = 0x0;
                c.lr = 0x801377b0; f_8000540c(c);
                pc = 0x801377b0;
            }
            0x801377b0 => {
                c.r[0] = c.lwz(c.r[30]);
                c.stb(c.r[27].wrapping_add(0x4c), c.r[0]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x8));
                c.cmps(0, c.r[0], 0x3);
                if c.crbit(2) { pc = 0x801377dc; continue; }
                pc = 0x801377c4;
            }
            0x801377c4 => {
                if !c.crbit(0) { pc = 0x801377dc; continue; }
                pc = 0x801377c8;
            }
            0x801377c8 => {
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(0) { pc = 0x801377d4; continue; }
                pc = 0x801377d0;
            }
            0x801377d0 => {
                pc = 0x801377dc; continue;
                pc = 0x801377d4;
            }
            0x801377d4 => {
                c.r[3] = 0xfffffffc;
                pc = 0x80137804; continue;
                pc = 0x801377dc;
            }
            0x801377dc => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0xc));
                c.r[0] = c.r[3].rotate_left(0) & 0x4;
                c.cmps(0, c.r[0], 0x4);
                if !c.crbit(2) { pc = 0x801377f0; continue; }
                pc = 0x801377ec;
            }
            0x801377ec => {
                c.r[28] = 0x0;
                pc = 0x801377f0;
            }
            0x801377f0 => {
                c.r[0] = c.r[3].rotate_left(0) & 0x1;
                c.cmps(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x80137800; continue; }
                pc = 0x801377fc;
            }
            0x801377fc => {
                c.r[26] = 0x1;
                pc = 0x80137800;
            }
            0x80137800 => {
                c.r[3] = 0x0;
                pc = 0x80137804;
            }
            0x80137804 => {
                c.r[0] = c.lwz(c.r[30]);
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80137818; continue; }
                pc = 0x80137810;
            }
            0x80137810 => {
                c.r[28] = 0x1;
                c.r[26] = 0x1;
                pc = 0x80137818;
            }
            0x80137818 => {
                c.r[30] = c.r[3].wrapping_add(c.r[30]);
                c.stb(c.r[27].wrapping_add(0x4e), c.r[26]);
                c.r[30] = c.r[30].wrapping_add(0x10);
                c.r[31] = c.r[28].rotate_left(0) & 0xff;
                c.r[28] = c.r[30] | c.r[30];
                c.r[26] = c.r[26].rotate_left(0) & 0xff;
                c.r[30] = 0x0;
                pc = 0x8013794c; continue;
                pc = 0x80137838;
            }
            0x80137838 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80137874; continue; }
                pc = 0x80137840;
            }
            0x80137840 => {
                c.r[4] = c.lwz(c.r[28]);
                c.r[3] = c.r[4].rotate_left(16) & 0xff;
                c.r[0] = c.r[4].rotate_left(24) & 0xff;
                c.stb(c.r[1].wrapping_add(0xe), c.r[3]);
                c.stb(c.r[1].wrapping_add(0xd), c.r[0]);
                c.stb(c.r[1].wrapping_add(0xc), c.r[4]);
                c.r[4] = c.lwz(c.r[28].wrapping_add(0x4));
                c.r[3] = c.r[4].rotate_left(16) & 0xff;
                c.r[0] = c.r[4].rotate_left(24) & 0xff;
                c.stb(c.r[1].wrapping_add(0xa), c.r[3]);
                c.stb(c.r[1].wrapping_add(0x9), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x8), c.r[4]);
                pc = 0x80137890; continue;
                pc = 0x80137874;
            }
            0x80137874 => {
                c.r[0] = 0x7f;
                c.stb(c.r[1].wrapping_add(0xe), c.r[0]);
                c.stb(c.r[1].wrapping_add(0xd), c.r[0]);
                c.stb(c.r[1].wrapping_add(0xc), c.r[0]);
                c.stb(c.r[1].wrapping_add(0xa), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x9), c.r[0]);
                c.stb(c.r[1].wrapping_add(0x8), c.r[0]);
                pc = 0x80137890;
            }
            0x80137890 => {
                c.cmpu(0, c.r[26], 0x0);
                if c.crbit(2) { pc = 0x801378b4; continue; }
                pc = 0x80137898;
            }
            0x80137898 => {
                c.r[0] = c.lwz(c.r[28]);
                c.r[0] = c.r[0].rotate_left(8) & 0xff;
                c.stb(c.r[1].wrapping_add(0xf), c.r[0]);
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x4));
                c.r[0] = c.r[0].rotate_left(8) & 0xff;
                c.stb(c.r[1].wrapping_add(0xb), c.r[0]);
                pc = 0x801378c0; continue;
                pc = 0x801378b4;
            }
            0x801378b4 => {
                c.r[0] = 0xff;
                c.stb(c.r[1].wrapping_add(0xf), c.r[0]);
                c.stb(c.r[1].wrapping_add(0xb), c.r[0]);
                pc = 0x801378c0;
            }
            0x801378c0 => {
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x8));
                c.cmps(0, c.r[0], 0x0);
                if !c.crbit(0) { pc = 0x801378e4; continue; }
                pc = 0x801378cc;
            }
            0x801378cc => {
                c.r[3] = c.r[27] | c.r[27];
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[6] = 0xffffffff;
                c.lr = 0x801378e0; f_8013ab60(c);
                pc = 0x801378e0;
            }
            0x801378e0 => {
                pc = 0x80137944; continue;
                pc = 0x801378e4;
            }
            0x801378e4 => {
                c.lr = 0x801378e8; f_800d37cc(c);
                pc = 0x801378e8;
            }
            0x801378e8 => {
                c.r[6] = 0x43300000;
                c.r[0] = c.lwz(c.r[28].wrapping_add(0x8));
                c.r[3] = c.r[3] ^ 0x80000000;
                c.stw(c.r[1].wrapping_add(0x10), c.r[6]);
                c.r[0] = c.r[0] ^ 0x80000000;
                { let ea = c.r[2].wrapping_add(0xffff9a88); c.f[3] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x14), c.r[3]);
                c.r[3] = c.r[27] | c.r[27];
                { let ea = c.r[2].wrapping_add(0xffff9a78); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.r[1].wrapping_add(0xc);
                { let ea = c.r[1].wrapping_add(0x10); c.f[1] = c.lfd(ea); }
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[0]);
                { let v = s(c.f[1] - c.f[3]); c.f[2] = v; c.ps1[2] = v; }
                c.stw(c.r[1].wrapping_add(0x18), c.r[6]);
                { let ea = c.r[1].wrapping_add(0x18); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] / c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x20); let v = c.f[0]; c.stfd(ea, v); }
                c.r[6] = c.lwz(c.r[1].wrapping_add(0x24));
                c.lr = 0x80137944; f_8013ab60(c);
                pc = 0x80137944;
            }
            0x80137944 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[28] = c.r[28].wrapping_add(0x10);
                pc = 0x8013794c;
            }
            0x8013794c => {
                c.cmps(0, c.r[30], c.r[29]);
                if c.crbit(0) { pc = 0x80137838; continue; }
                pc = 0x80137954;
            }
            0x80137954 => {
                c.r[3] = c.r[28] | c.r[28];
                { let mut ea = c.r[1].wrapping_add(0x28); for n in 26..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x44));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x40);
                if true { return; }
                return;
            }
            _ => panic!("f_80137780: bad pc {:#x}", pc),
        }
    }
}

/// fn_8013796C (0x8013796c, 120 bytes)
#[inline(never)]
pub fn f_8013796c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013796c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013796c); f_8013796c_b(c, 0x8013796c); crate::prof::exit(c, 0x8013796c); return; }
    }
    f_8013796c_b(c, 0x8013796c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013796c_at(c: &mut Cpu, pc: u32) { f_8013796c_b(c, pc) }
#[inline(always)]
fn f_8013796c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013796c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013796c, pc); }
        match pc {
            0x8013796c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x2c;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80137984; f_80131428(c);
                pc = 0x80137984;
            }
            0x80137984 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x801379cc; continue; }
                pc = 0x8013798c;
            }
            0x8013798c => {
                c.r[5] = 0x80130000;
                c.r[6] = 0x80130000;
                c.r[7] = 0x80130000;
                c.r[8] = 0x80130000;
                c.r[4] = 0x80130000;
                c.r[5] = c.r[5].wrapping_add(0x79e4);
                c.r[9] = c.r[8].wrapping_add(0x7d14);
                c.r[6] = c.r[6].wrapping_add(0x7aa4);
                c.r[10] = c.r[4].wrapping_add(0x7f58);
                c.r[7] = c.r[7].wrapping_add(0x7a2c);
                c.r[4] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x801379c0; f_80131200(c);
                pc = 0x801379c0;
            }
            0x801379c0 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x801379cc; f_8013139c(c);
                pc = 0x801379cc;
            }
            0x801379cc => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8013796c: bad pc {:#x}", pc),
        }
    }
}

/// fn_801379E4 (0x801379e4, 72 bytes)
#[inline(never)]
pub fn f_801379e4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801379e4) { return; }
        if c.prof { crate::prof::enter(c, 0x801379e4); f_801379e4_b(c, 0x801379e4); crate::prof::exit(c, 0x801379e4); return; }
    }
    f_801379e4_b(c, 0x801379e4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801379e4_at(c: &mut Cpu, pc: u32) { f_801379e4_b(c, pc) }
#[inline(always)]
fn f_801379e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801379e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801379e4, pc); }
        match pc {
            0x801379e4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x80137a14; continue; }
                pc = 0x801379fc;
            }
            0x801379fc => {
                c.lr = 0x80137a00; f_800b8df4(c);
                pc = 0x80137a00;
            }
            0x80137a00 => {
                c.lr = 0x80137a04; f_800b856c(c);
                pc = 0x80137a04;
            }
            0x80137a04 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x14));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80137a14; continue; }
                pc = 0x80137a10;
            }
            0x80137a10 => {
                c.lr = 0x80137a14; f_800ef5a4(c);
                pc = 0x80137a14;
            }
            0x80137a14 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801379e4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80137A2C (0x80137a2c, 120 bytes)
#[inline(never)]
pub fn f_80137a2c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80137a2c) { return; }
        if c.prof { crate::prof::enter(c, 0x80137a2c); f_80137a2c_b(c, 0x80137a2c); crate::prof::exit(c, 0x80137a2c); return; }
    }
    f_80137a2c_b(c, 0x80137a2c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80137a2c_at(c: &mut Cpu, pc: u32) { f_80137a2c_b(c, pc) }
#[inline(always)]
fn f_80137a2c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80137a2c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80137a2c, pc); }
        match pc {
            0x80137a2c => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[3] | c.r[3];
                c.rc(c.r[30]);
                if c.crbit(2) { pc = 0x80137a88; continue; }
                pc = 0x80137a48;
            }
            0x80137a48 => {
                c.lr = 0x80137a4c; f_800b8df4(c);
                pc = 0x80137a4c;
            }
            0x80137a4c => {
                c.lr = 0x80137a50; f_800b856c(c);
                pc = 0x80137a50;
            }
            0x80137a50 => {
                c.r[31] = c.lhz(c.r[30].wrapping_add(0xc));
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80137a6c; continue; }
                pc = 0x80137a5c;
            }
            0x80137a5c => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80137a64; f_800e24b0(c);
                pc = 0x80137a64;
            }
            0x80137a64 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80137a6c; f_800e209c(c);
                pc = 0x80137a6c;
            }
            0x80137a6c => {
                c.r[31] = c.lhz(c.r[30].wrapping_add(0xe));
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x80137a88; continue; }
                pc = 0x80137a78;
            }
            0x80137a78 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80137a80; f_800e24b0(c);
                pc = 0x80137a80;
            }
            0x80137a80 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80137a88; f_800e209c(c);
                pc = 0x80137a88;
            }
            0x80137a88 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80137a2c: bad pc {:#x}", pc),
        }
    }
}

/// tracefxStartEffect (0x80137aa4, 624 bytes)
#[inline(never)]
pub fn f_80137aa4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80137aa4) { return; }
        if c.prof { crate::prof::enter(c, 0x80137aa4); f_80137aa4_b(c, 0x80137aa4); crate::prof::exit(c, 0x80137aa4); return; }
    }
    f_80137aa4_b(c, 0x80137aa4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80137aa4_at(c: &mut Cpu, pc: u32) { f_80137aa4_b(c, pc) }
#[inline(always)]
fn f_80137aa4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80137aa4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80137aa4, pc); }
        match pc {
            0x80137aa4 => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if c.crbit(2) { pc = 0x80137cec; continue; }
                pc = 0x80137abc;
            }
            0x80137abc => {
                c.r[28] = c.lhz(c.r[29].wrapping_add(0x20));
                c.r[27] = c.lhz(c.r[29].wrapping_add(0x22));
                c.r[3] = c.lhz(c.r[29].wrapping_add(0x24));
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x26));
                c.lr = 0x80137ad0; f_800f9318(c);
                pc = 0x80137ad0;
            }
            0x80137ad0 => {
                c.r[0] = c.lwz(c.r[29].wrapping_add(0x14));
                c.r[30] = c.r[3] | c.r[3];
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80137ae8; continue; }
                pc = 0x80137ae0;
            }
            0x80137ae0 => {
                c.r[3] = 0x0;
                pc = 0x80137d00; continue;
                pc = 0x80137ae8;
            }
            0x80137ae8 => {
                c.cmpu(0, c.r[27], 0x0);
                if c.crbit(2) { pc = 0x80137af8; continue; }
                pc = 0x80137af0;
            }
            0x80137af0 => {
                c.cmpu(0, c.r[28], 0x0);
                if !c.crbit(2) { pc = 0x80137b00; continue; }
                pc = 0x80137af8;
            }
            0x80137af8 => {
                c.r[3] = 0x0;
                pc = 0x80137d00; continue;
                pc = 0x80137b00;
            }
            0x80137b00 => {
                c.cmpu(0, c.r[30], 0x0);
                if !c.crbit(2) { pc = 0x80137b10; continue; }
                pc = 0x80137b08;
            }
            0x80137b08 => {
                c.r[3] = 0x0;
                pc = 0x80137d00; continue;
                pc = 0x80137b10;
            }
            0x80137b10 => {
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x28));
                c.lr = 0x80137b18; f_800ee150(c);
                pc = 0x80137b18;
            }
            0x80137b18 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80137b28; continue; }
                pc = 0x80137b20;
            }
            0x80137b20 => {
                c.r[3] = 0x0;
                pc = 0x80137d00; continue;
                pc = 0x80137b28;
            }
            0x80137b28 => {
                c.lr = 0x80137b2c; f_800ee828(c);
                pc = 0x80137b2c;
            }
            0x80137b2c => {
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x2a));
                c.r[3] = c.r[30] | c.r[30];
                c.lr = 0x80137b38; f_800ee150(c);
                pc = 0x80137b38;
            }
            0x80137b38 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80137b48; continue; }
                pc = 0x80137b40;
            }
            0x80137b40 => {
                c.r[3] = 0x0;
                pc = 0x80137d00; continue;
                pc = 0x80137b48;
            }
            0x80137b48 => {
                c.lr = 0x80137b4c; f_800ee828(c);
                pc = 0x80137b4c;
            }
            0x80137b4c => {
                c.r[0] = c.r[28].rotate_left(31) & 0x7fff;
                c.cmps(0, c.r[27], c.r[0]);
                if !c.crbit(1) { pc = 0x80137b60; continue; }
                pc = 0x80137b58;
            }
            0x80137b58 => {
                c.sth(c.r[29].wrapping_add(0x22), c.r[0]);
                c.r[27] = c.r[0] | c.r[0];
                pc = 0x80137b60;
            }
            0x80137b60 => {
                c.r[28] = c.r[27].rotate_left(5) & 0x1fffe0;
                c.r[30] = c.r[27].rotate_left(0) & 0xffff;
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80137b70; f_800e3534(c);
                pc = 0x80137b70;
            }
            0x80137b70 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80137b80; continue; }
                pc = 0x80137b78;
            }
            0x80137b78 => {
                c.r[3] = 0x0;
                pc = 0x80137d00; continue;
                pc = 0x80137b80;
            }
            0x80137b80 => {
                c.sth(c.r[29].wrapping_add(0xc), c.r[3]);
                c.lr = 0x80137b88; f_800e27b0(c);
                pc = 0x80137b88;
            }
            0x80137b88 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[5] = c.r[28] | c.r[28];
                c.stw(c.r[29].wrapping_add(0x4), c.r[31]);
                c.r[4] = 0x0;
                c.stw(c.r[29], c.r[31]);
                c.lr = 0x80137ba0; f_8000540c(c);
                pc = 0x80137ba0;
            }
            0x80137ba0 => {
                c.r[6] = c.r[30].wrapping_add(0xffffffff);
                c.r[0] = c.r[30] | c.r[30];
                c.r[3] = c.r[6].rotate_left(5) & 0xffffffe0;
                c.r[7] = 0x0;
                c.r[5] = c.r[31].wrapping_add(c.r[3]);
                pc = 0x80137c20; continue;
                pc = 0x80137bb8;
            }
            0x80137bb8 => {
                c.r[3] = c.r[7].rotate_left(0) & 0xffff;
                c.rc(c.r[3]);
                if !c.crbit(2) { pc = 0x80137bd0; continue; }
                pc = 0x80137bc0;
            }
            0x80137bc0 => {
                c.r[3] = c.r[7].rotate_left(5) & 0x1fffe0;
                c.r[3] = c.r[3].wrapping_add(0x1c);
                c.stw(c.r[31].wrapping_add(c.r[3]), c.r[5]);
                pc = 0x80137be8; continue;
                pc = 0x80137bd0;
            }
            0x80137bd0 => {
                c.r[4] = c.r[3].wrapping_add(0xffffffff);
                c.r[3] = c.r[7].rotate_left(5) & 0x1fffe0;
                c.r[4] = c.r[4].rotate_left(5) & 0xffffffe0;
                c.r[4] = c.r[31].wrapping_add(c.r[4]);
                c.r[3] = c.r[3].wrapping_add(0x1c);
                c.stw(c.r[31].wrapping_add(c.r[3]), c.r[4]);
                pc = 0x80137be8;
            }
            0x80137be8 => {
                c.r[3] = c.r[7].rotate_left(0) & 0xffff;
                c.cmps(0, c.r[3], c.r[6]);
                if !c.crbit(2) { pc = 0x80137c04; continue; }
                pc = 0x80137bf4;
            }
            0x80137bf4 => {
                c.r[3] = c.r[7].rotate_left(5) & 0x1fffe0;
                c.r[3] = c.r[3].wrapping_add(0x18);
                c.stw(c.r[31].wrapping_add(c.r[3]), c.r[31]);
                pc = 0x80137c1c; continue;
                pc = 0x80137c04;
            }
            0x80137c04 => {
                c.r[4] = c.r[3].wrapping_add(0x1);
                c.r[3] = c.r[7].rotate_left(5) & 0x1fffe0;
                c.r[4] = c.r[4].rotate_left(5) & 0xffffffe0;
                c.r[4] = c.r[31].wrapping_add(c.r[4]);
                c.r[3] = c.r[3].wrapping_add(0x18);
                c.stw(c.r[31].wrapping_add(c.r[3]), c.r[4]);
                pc = 0x80137c1c;
            }
            0x80137c1c => {
                c.r[7] = c.r[7].wrapping_add(0x1);
                pc = 0x80137c20;
            }
            0x80137c20 => {
                c.r[3] = c.r[7].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x80137bb8; continue; }
                pc = 0x80137c2c;
            }
            0x80137c2c => {
                c.r[3] = c.r[30].rotate_left(4) & 0xfffffff0;
                c.lr = 0x80137c34; f_800e3534(c);
                pc = 0x80137c34;
            }
            0x80137c34 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x80137c54; continue; }
                pc = 0x80137c3c;
            }
            0x80137c3c => {
                c.r[3] = c.lhz(c.r[29].wrapping_add(0xc));
                c.lr = 0x80137c44; f_800e24b0(c);
                pc = 0x80137c44;
            }
            0x80137c44 => {
                c.r[3] = c.lhz(c.r[29].wrapping_add(0xc));
                c.lr = 0x80137c4c; f_800e209c(c);
                pc = 0x80137c4c;
            }
            0x80137c4c => {
                c.r[3] = 0x0;
                pc = 0x80137d00; continue;
                pc = 0x80137c54;
            }
            0x80137c54 => {
                c.sth(c.r[29].wrapping_add(0xe), c.r[3]);
                c.lr = 0x80137c5c; f_800e27b0(c);
                pc = 0x80137c5c;
            }
            0x80137c5c => {
                c.stw(c.r[29].wrapping_add(0x8), c.r[3]);
                c.r[5] = 0x0;
                c.r[3] = 0x43300000;
                { let ea = c.r[2].wrapping_add(0xffff9a98); c.f[3] = c.lfd(ea); }
                c.sth(c.r[29].wrapping_add(0x1c), c.r[5]);
                { let ea = c.r[2].wrapping_add(0xffff9a90); c.f[4] = c.lfs(ea); c.ps1[4] = c.f[4]; }
                c.sth(c.r[29].wrapping_add(0x1e), c.r[5]);
                { let ea = c.r[2].wrapping_add(0xffff9aa0); c.f[2] = c.lfd(ea); }
                c.r[4] = c.lhz(c.r[29].wrapping_add(0x22));
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                c.r[0] = c.r[4].wrapping_add(0xffffffff);
                c.r[6] = c.lwz(c.r[29].wrapping_add(0x8));
                c.r[0] = c.r[0] ^ 0x80000000;
                { let ea = c.r[2].wrapping_add(0xffff9a94); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[4] / c.f[1]); c.f[3] = v; c.ps1[3] = v; }
                pc = 0x80137cd8; continue;
                pc = 0x80137ca8;
            }
            0x80137ca8 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.stw(c.r[1].wrapping_add(0x8), c.r[3]);
                c.r[5] = c.r[5].wrapping_add(0x1);
                c.stw(c.r[1].wrapping_add(0xc), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x8); c.f[1] = c.lfd(ea); }
                { let v = s(c.f[1] - c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[1] * c.f[3]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[6]; let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[6].wrapping_add(0x4); let v = c.f[0]; c.stfs(ea, v); }
                { let ea = c.r[6].wrapping_add(0x8); let v = c.f[1]; c.stfs(ea, v); }
                { let ea = c.r[6].wrapping_add(0xc); let v = c.f[4]; c.stfs(ea, v); }
                c.r[6] = c.r[6].wrapping_add(0x10);
                pc = 0x80137cd8;
            }
            0x80137cd8 => {
                c.r[0] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[4]);
                if c.crbit(0) { pc = 0x80137ca8; continue; }
                pc = 0x80137ce4;
            }
            0x80137ce4 => {
                c.r[3] = 0x1;
                pc = 0x80137d00; continue;
                pc = 0x80137cec;
            }
            0x80137cec => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x2b08);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80137cfc; stubs_gen::f_800dd970(c);
                pc = 0x80137cfc;
            }
            0x80137cfc => {
                c.r[3] = 0x0;
                pc = 0x80137d00;
            }
            0x80137d00 => {
                { let mut ea = c.r[1].wrapping_add(0x1c); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80137aa4: bad pc {:#x}", pc),
        }
    }
}

/// fn_80137D14 (0x80137d14, 580 bytes)
#[inline(never)]
pub fn f_80137d14(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80137d14) { return; }
        if c.prof { crate::prof::enter(c, 0x80137d14); f_80137d14_b(c, 0x80137d14); crate::prof::exit(c, 0x80137d14); return; }
    }
    f_80137d14_b(c, 0x80137d14)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80137d14_at(c: &mut Cpu, pc: u32) { f_80137d14_b(c, pc) }
#[inline(always)]
fn f_80137d14_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80137d14;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80137d14, pc); }
        match pc {
            0x80137d14 => {
                { let ea = c.r[1].wrapping_add(0xffffff90); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x74), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x60); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x50); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                c.stw(c.r[1].wrapping_add(0x4c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x48), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x44), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x40), c.r[28]);
                c.r[30] = c.r[3] | c.r[3];
                c.r[31] = c.r[4] | c.r[4];
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x20));
                c.cmpu(0, c.r[3], c.r[0]);
                if !c.crbit(0) { pc = 0x80137f24; continue; }
                pc = 0x80137d58;
            }
            0x80137d58 => {
                c.r[3] = c.lhz(c.r[30].wrapping_add(0x24));
                c.r[4] = c.lhz(c.r[30].wrapping_add(0x26));
                c.lr = 0x80137d64; f_800f9318(c);
                pc = 0x80137d64;
            }
            0x80137d64 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if !c.crbit(2) { pc = 0x80137d74; continue; }
                pc = 0x80137d6c;
            }
            0x80137d6c => {
                c.r[3] = 0x0;
                pc = 0x80137f28; continue;
                pc = 0x80137d74;
            }
            0x80137d74 => {
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x14));
                c.cmpu(0, c.r[0], 0x0);
                if !c.crbit(2) { pc = 0x80137d88; continue; }
                pc = 0x80137d80;
            }
            0x80137d80 => {
                c.r[3] = 0x0;
                pc = 0x80137f28; continue;
                pc = 0x80137d88;
            }
            0x80137d88 => {
                c.r[4] = c.lhz(c.r[30].wrapping_add(0x28));
                c.lr = 0x80137d90; f_800ee150(c);
                pc = 0x80137d90;
            }
            0x80137d90 => {
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                if !c.crbit(2) { pc = 0x80137da0; continue; }
                pc = 0x80137d98;
            }
            0x80137d98 => {
                c.r[3] = 0x0;
                pc = 0x80137f28; continue;
                pc = 0x80137da0;
            }
            0x80137da0 => {
                c.r[4] = c.r[1].wrapping_add(0x2c);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80137db0; f_800ee3bc(c);
                pc = 0x80137db0;
            }
            0x80137db0 => {
                c.r[3] = c.r[28] | c.r[28];
                c.lr = 0x80137db8; f_800ee828(c);
                pc = 0x80137db8;
            }
            0x80137db8 => {
                c.r[4] = c.lhz(c.r[30].wrapping_add(0x2a));
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80137dc4; f_800ee150(c);
                pc = 0x80137dc4;
            }
            0x80137dc4 => {
                c.r[29] = c.r[3] | c.r[3];
                c.rc(c.r[29]);
                if !c.crbit(2) { pc = 0x80137dd4; continue; }
                pc = 0x80137dcc;
            }
            0x80137dcc => {
                c.r[3] = 0x0;
                pc = 0x80137f28; continue;
                pc = 0x80137dd4;
            }
            0x80137dd4 => {
                c.r[4] = c.r[1].wrapping_add(0x20);
                c.r[5] = 0x0;
                c.r[6] = 0x0;
                c.lr = 0x80137de4; f_800ee3bc(c);
                pc = 0x80137de4;
            }
            0x80137de4 => {
                c.r[3] = c.r[29] | c.r[29];
                c.lr = 0x80137dec; f_800ee828(c);
                pc = 0x80137dec;
            }
            0x80137dec => {
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x1c));
                c.r[29] = c.lwz(c.r[30]);
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80137e34; continue; }
                pc = 0x80137dfc;
            }
            0x80137dfc => {
                c.r[4] = c.r[29] | c.r[29];
                c.r[3] = c.r[1].wrapping_add(0x14);
                c.lr = 0x80137e08; f_800e01d0(c);
                pc = 0x80137e08;
            }
            0x80137e08 => {
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[29].wrapping_add(0xc);
                c.lr = 0x80137e14; f_800e01d0(c);
                pc = 0x80137e14;
            }
            0x80137e14 => {
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x3c), c.r[31]);
                { let ea = c.r[2].wrapping_add(0xffff9aa0); c.f[1] = c.lfd(ea); }
                c.stw(c.r[1].wrapping_add(0x38), c.r[0]);
                { let ea = c.r[2].wrapping_add(0xffff9a90); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x38); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[2] / c.f[0]); c.f[31] = v; c.ps1[31] = v; }
                pc = 0x80137e34;
            }
            0x80137e34 => {
                c.r[28] = 0x0;
                pc = 0x80137eb4; continue;
                pc = 0x80137e3c;
            }
            0x80137e3c => {
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x1c));
                c.r[29] = c.lwz(c.r[29].wrapping_add(0x18));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80137e98; continue; }
                pc = 0x80137e4c;
            }
            0x80137e4c => {
                c.r[3] = c.r[28].wrapping_add(0x1);
                c.r[0] = 0x43300000;
                c.stw(c.r[1].wrapping_add(0x3c), c.r[3]);
                c.r[3] = c.r[29] | c.r[29];
                { let ea = c.r[2].wrapping_add(0xffff9aa0); c.f[1] = c.lfd(ea); }
                c.r[4] = c.r[1].wrapping_add(0x14);
                c.stw(c.r[1].wrapping_add(0x38), c.r[0]);
                c.r[5] = c.r[1].wrapping_add(0x2c);
                { let ea = c.r[1].wrapping_add(0x38); c.f[0] = c.lfd(ea); }
                { let v = s(c.f[0] - c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[31] * c.f[0]); c.f[30] = v; c.ps1[30] = v; }
                c.f[1] = c.f[30];
                c.lr = 0x80137e80; f_800e090c(c);
                pc = 0x80137e80;
            }
            0x80137e80 => {
                c.f[1] = c.f[30];
                c.r[3] = c.r[29].wrapping_add(0xc);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = c.r[1].wrapping_add(0x20);
                c.lr = 0x80137e94; f_800e090c(c);
                pc = 0x80137e94;
            }
            0x80137e94 => {
                pc = 0x80137eb0; continue;
                pc = 0x80137e98;
            }
            0x80137e98 => {
                c.r[3] = c.r[29] | c.r[29];
                c.r[4] = c.r[1].wrapping_add(0x2c);
                c.lr = 0x80137ea4; f_800e01d0(c);
                pc = 0x80137ea4;
            }
            0x80137ea4 => {
                c.r[3] = c.r[29].wrapping_add(0xc);
                c.r[4] = c.r[1].wrapping_add(0x20);
                c.lr = 0x80137eb0; f_800e01d0(c);
                pc = 0x80137eb0;
            }
            0x80137eb0 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x80137eb4;
            }
            0x80137eb4 => {
                c.cmpu(0, c.r[28], c.r[31]);
                if c.crbit(0) { pc = 0x80137e3c; continue; }
                pc = 0x80137ebc;
            }
            0x80137ebc => {
                c.stw(c.r[30], c.r[29]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x1c));
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.sth(c.r[30].wrapping_add(0x1c), c.r[0]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x1e));
                c.r[0] = c.r[0].wrapping_add(c.r[31]);
                c.sth(c.r[30].wrapping_add(0x1e), c.r[0]);
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x1c));
                c.r[3] = c.lhz(c.r[30].wrapping_add(0x20));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(1) { pc = 0x80137eec; continue; }
                pc = 0x80137ee8;
            }
            0x80137ee8 => {
                c.sth(c.r[30].wrapping_add(0x1c), c.r[3]);
                pc = 0x80137eec;
            }
            0x80137eec => {
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x1e));
                c.r[3] = c.lhz(c.r[30].wrapping_add(0x22));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(1) { pc = 0x80137f00; continue; }
                pc = 0x80137efc;
            }
            0x80137efc => {
                c.sth(c.r[30].wrapping_add(0x1e), c.r[3]);
                pc = 0x80137f00;
            }
            0x80137f00 => {
                c.r[3] = c.lhz(c.r[30].wrapping_add(0x1c));
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x20));
                c.r[4] = c.lhz(c.r[30].wrapping_add(0x1e));
                c.r[0] = c.r[0].wrapping_sub(c.r[3]);
                c.cmps(0, c.r[4], c.r[0]);
                if !c.crbit(1) { pc = 0x80137f1c; continue; }
                pc = 0x80137f18;
            }
            0x80137f18 => {
                c.sth(c.r[30].wrapping_add(0x1e), c.r[0]);
                pc = 0x80137f1c;
            }
            0x80137f1c => {
                c.r[3] = 0x1;
                pc = 0x80137f28; continue;
                pc = 0x80137f24;
            }
            0x80137f24 => {
                c.r[3] = 0x0;
                pc = 0x80137f28;
            }
            0x80137f28 => {
                { let ea = c.r[1].wrapping_add(0x68); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x60); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x58); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x50); c.f[30] = c.lfd(ea); }
                c.r[31] = c.lwz(c.r[1].wrapping_add(0x4c));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x48));
                c.r[29] = c.lwz(c.r[1].wrapping_add(0x44));
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x74));
                c.r[28] = c.lwz(c.r[1].wrapping_add(0x40));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x70);
                if true { return; }
                return;
            }
            _ => panic!("f_80137d14: bad pc {:#x}", pc),
        }
    }
}

/// fn_80137F58 (0x80137f58, 380 bytes)
#[inline(never)]
pub fn f_80137f58(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80137f58) { return; }
        if c.prof { crate::prof::enter(c, 0x80137f58); f_80137f58_b(c, 0x80137f58); crate::prof::exit(c, 0x80137f58); return; }
    }
    f_80137f58_b(c, 0x80137f58)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80137f58_at(c: &mut Cpu, pc: u32) { f_80137f58_b(c, pc) }
#[inline(always)]
fn f_80137f58_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80137f58;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80137f58, pc); }
        match pc {
            0x80137f58 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[27] = c.r[3] | c.r[3];
                c.r[0] = c.lwz(c.r[3].wrapping_add(0x14));
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                c.r[31] = c.lhz(c.r[27].wrapping_add(0x1e));
                if !c.crbit(2) { pc = 0x80137f88; continue; }
                pc = 0x80137f80;
            }
            0x80137f80 => {
                c.r[3] = 0x0;
                pc = 0x801380c0; continue;
                pc = 0x80137f88;
            }
            0x80137f88 => {
                c.cmpu(0, c.r[3], 0x1);
                if !c.crbit(1) { pc = 0x801380bc; continue; }
                pc = 0x80137f90;
            }
            0x80137f90 => {
                c.cmpu(0, c.r[31], 0x1);
                if !c.crbit(1) { pc = 0x801380bc; continue; }
                pc = 0x80137f98;
            }
            0x80137f98 => {
                c.lr = 0x80137f9c; f_800d2248(c);
                pc = 0x80137f9c;
            }
            0x80137f9c => {
                c.r[3] = 0x1;
                c.r[4] = 0x6;
                c.r[5] = 0x7;
                c.lr = 0x80137fac; f_800da4c4(c);
                pc = 0x80137fac;
            }
            0x80137fac => {
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x80137fbc; f_800da2bc(c);
                pc = 0x80137fbc;
            }
            0x80137fbc => {
                c.r[3] = 0x1;
                c.r[4] = 0x2;
                c.r[5] = 0x1;
                c.lr = 0x80137fcc; f_800da1e8(c);
                pc = 0x80137fcc;
            }
            0x80137fcc => {
                c.r[3] = 0x0;
                c.lr = 0x80137fd4; f_800da028(c);
                pc = 0x80137fd4;
            }
            0x80137fd4 => {
                c.r[3] = 0x3;
                c.lr = 0x80137fdc; f_800d88dc(c);
                pc = 0x80137fdc;
            }
            0x80137fdc => {
                c.r[3] = 0x4;
                c.lr = 0x80137fe4; f_800d888c(c);
                pc = 0x80137fe4;
            }
            0x80137fe4 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x10));
                c.lr = 0x80137fec; f_800d7820(c);
                pc = 0x80137fec;
            }
            0x80137fec => {
                c.r[4] = c.lwz(c.r[27].wrapping_add(0x14));
                c.r[3] = 0x0;
                c.lr = 0x80137ff8; f_800d85d4(c);
                pc = 0x80137ff8;
            }
            0x80137ff8 => {
                c.r[3] = 0x4;
                c.lr = 0x80138000; f_800d6a00(c);
                pc = 0x80138000;
            }
            0x80138000 => {
                c.r[3] = c.r[31].rotate_left(1) & 0xfffe;
                c.lr = 0x80138008; f_800d67bc(c);
                pc = 0x80138008;
            }
            0x80138008 => {
                c.r[0] = c.lhz(c.r[27].wrapping_add(0x22));
                c.r[28] = 0x0;
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x8));
                c.r[0] = c.r[0].wrapping_sub(c.r[31]);
                c.r[30] = c.lwz(c.r[27]);
                c.r[0] = c.r[0].rotate_left(4) & 0xfffffff0;
                c.r[29] = c.r[3].wrapping_add(c.r[0]);
                pc = 0x801380a4; continue;
                pc = 0x80138028;
            }
            0x80138028 => {
                { let ea = c.r[30]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[30].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[30].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x80138038; f_800d6680(c);
                pc = 0x80138038;
            }
            0x80138038 => {
                c.r[4] = c.lbz(c.r[27].wrapping_add(0x18));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[27].wrapping_add(0x19));
                c.r[6] = c.lbz(c.r[27].wrapping_add(0x1a));
                c.r[7] = c.lbz(c.r[27].wrapping_add(0x1b));
                c.lr = 0x80138050; f_800d5cb8(c);
                pc = 0x80138050;
            }
            0x80138050 => {
                { let ea = c.r[29]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                { let ea = c.r[29].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.lr = 0x80138060; f_800d59b8(c);
                pc = 0x80138060;
            }
            0x80138060 => {
                { let ea = c.r[30].wrapping_add(0xc); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[30].wrapping_add(0x10); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[30].wrapping_add(0x14); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x80138070; f_800d6680(c);
                pc = 0x80138070;
            }
            0x80138070 => {
                c.r[4] = c.lbz(c.r[27].wrapping_add(0x18));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[27].wrapping_add(0x19));
                c.r[6] = c.lbz(c.r[27].wrapping_add(0x1a));
                c.r[7] = c.lbz(c.r[27].wrapping_add(0x1b));
                c.lr = 0x80138088; f_800d5cb8(c);
                pc = 0x80138088;
            }
            0x80138088 => {
                { let ea = c.r[29].wrapping_add(0x8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                { let ea = c.r[29].wrapping_add(0xc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.lr = 0x80138098; f_800d59b8(c);
                pc = 0x80138098;
            }
            0x80138098 => {
                c.r[30] = c.lwz(c.r[30].wrapping_add(0x1c));
                c.r[29] = c.r[29].wrapping_add(0x10);
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x801380a4;
            }
            0x801380a4 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[31]);
                if c.crbit(0) { pc = 0x80138028; continue; }
                pc = 0x801380b0;
            }
            0x801380b0 => {
                c.lr = 0x801380b4; f_800d6728(c);
                pc = 0x801380b4;
            }
            0x801380b4 => {
                c.r[3] = 0x1;
                pc = 0x801380c0; continue;
                pc = 0x801380bc;
            }
            0x801380bc => {
                c.r[3] = 0x0;
                pc = 0x801380c0;
            }
            0x801380c0 => {
                { let mut ea = c.r[1].wrapping_add(0xc); for n in 27..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x24));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x20);
                if true { return; }
                return;
            }
            _ => panic!("f_80137f58: bad pc {:#x}", pc),
        }
    }
}

/// fn_801380D4 (0x801380d4, 120 bytes)
#[inline(never)]
pub fn f_801380d4(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801380d4) { return; }
        if c.prof { crate::prof::enter(c, 0x801380d4); f_801380d4_b(c, 0x801380d4); crate::prof::exit(c, 0x801380d4); return; }
    }
    f_801380d4_b(c, 0x801380d4)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801380d4_at(c: &mut Cpu, pc: u32) { f_801380d4_b(c, pc) }
#[inline(always)]
fn f_801380d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801380d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801380d4, pc); }
        match pc {
            0x801380d4 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x70;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x801380ec; f_80131428(c);
                pc = 0x801380ec;
            }
            0x801380ec => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x80138134; continue; }
                pc = 0x801380f4;
            }
            0x801380f4 => {
                c.r[5] = 0x80140000;
                c.r[6] = 0x80140000;
                c.r[7] = 0x80140000;
                c.r[8] = 0x80140000;
                c.r[4] = 0x80140000;
                c.r[5] = c.r[5].wrapping_add(0xffff8630);
                c.r[9] = c.r[8].wrapping_add(0xffff87c0);
                c.r[6] = c.r[6].wrapping_add(0xffff86dc);
                c.r[10] = c.r[4].wrapping_add(0xffff814c);
                c.r[7] = c.r[7].wrapping_add(0xffff8680);
                c.r[4] = 0x0;
                c.r[8] = 0x0;
                c.lr = 0x80138128; f_80131200(c);
                pc = 0x80138128;
            }
            0x80138128 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x80138134; f_8013139c(c);
                pc = 0x80138134;
            }
            0x80138134 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801380d4: bad pc {:#x}", pc),
        }
    }
}

/// _lightningRenderMain (0x8013814c, 1252 bytes)
#[inline(never)]
pub fn f_8013814c(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x8013814c) { return; }
        if c.prof { crate::prof::enter(c, 0x8013814c); f_8013814c_b(c, 0x8013814c); crate::prof::exit(c, 0x8013814c); return; }
    }
    f_8013814c_b(c, 0x8013814c)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8013814c_at(c: &mut Cpu, pc: u32) { f_8013814c_b(c, pc) }
#[inline(always)]
fn f_8013814c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8013814c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8013814c, pc); }
        match pc {
            0x8013814c => {
                { let ea = c.r[1].wrapping_add(0xffffff60); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xa4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0x90); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x98); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0x80); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let mut ea = c.r[1].wrapping_add(0x64); for n in 25..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[25] = c.r[3] | c.r[3];
                c.rc(c.r[25]);
                if c.crbit(2) { pc = 0x80138608; continue; }
                pc = 0x80138174;
            }
            0x80138174 => {
                c.r[27] = c.lhz(c.r[25].wrapping_add(0x8));
                c.r[3] = c.lhz(c.r[25].wrapping_add(0xa));
                c.r[4] = c.lhz(c.r[25].wrapping_add(0xc));
                c.lr = 0x80138184; f_800f9318(c);
                pc = 0x80138184;
            }
            0x80138184 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80138194; continue; }
                pc = 0x8013818c;
            }
            0x8013818c => {
                c.r[3] = 0x0;
                pc = 0x8013860c; continue;
                pc = 0x80138194;
            }
            0x80138194 => {
                c.r[4] = c.r[1].wrapping_add(0x1c);
                c.lr = 0x8013819c; f_800e3d98(c);
                pc = 0x8013819c;
            }
            0x8013819c => {
                c.lr = 0x801381a0; f_800d2248(c);
                pc = 0x801381a0;
            }
            0x801381a0 => {
                c.lr = 0x801381a4; f_800d2584(c);
                pc = 0x801381a4;
            }
            0x801381a4 => {
                c.r[4] = c.r[1].wrapping_add(0x10);
                c.r[26] = c.r[3] | c.r[3];
                c.lr = 0x801381b0; f_800d1f84(c);
                pc = 0x801381b0;
            }
            0x801381b0 => {
                c.r[5] = c.r[1].wrapping_add(0x8);
                c.r[3] = c.r[26] | c.r[26];
                c.r[6] = c.r[5] | c.r[5];
                c.r[4] = c.r[1].wrapping_add(0xc);
                c.r[7] = c.r[5] | c.r[5];
                c.lr = 0x801381c8; f_800d1fdc(c);
                pc = 0x801381c8;
            }
            0x801381c8 => {
                c.r[3] = c.r[1].wrapping_add(0x10);
                c.r[4] = c.r[1].wrapping_add(0x1c);
                c.lr = 0x801381d4; f_800e0040(c);
                pc = 0x801381d4;
            }
            0x801381d4 => {
                c.f[31] = c.f[1];
                c.r[3] = 0x1;
                c.r[4] = 0x6;
                c.r[5] = 0x1;
                c.lr = 0x801381e8; f_800da4c4(c);
                pc = 0x801381e8;
            }
            0x801381e8 => {
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x801381f8; f_800da2bc(c);
                pc = 0x801381f8;
            }
            0x801381f8 => {
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.r[5] = 0x1;
                c.lr = 0x80138208; f_800da1e8(c);
                pc = 0x80138208;
            }
            0x80138208 => {
                c.r[3] = 0x0;
                c.lr = 0x80138210; f_800da028(c);
                pc = 0x80138210;
            }
            0x80138210 => {
                c.r[3] = c.r[25].wrapping_add(0x28);
                c.lr = 0x80138218; f_800e008c(c);
                pc = 0x80138218;
            }
            0x80138218 => {
                { let ea = c.r[2].wrapping_add(0xffff9aa8); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[1].wrapping_add(0xc); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[0] * c.f[1]); c.f[3] = v; c.ps1[3] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9aac); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[31] * c.f[2]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[3] / c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.fcmp(0, c.f[1], c.f[0]);
                if !c.crbit(1) { pc = 0x8013823c; continue; }
                pc = 0x80138238;
            }
            0x80138238 => {
                c.f[1] = c.f[0];
                pc = 0x8013823c;
            }
            0x8013823c => {
                { let ea = c.r[2].wrapping_add(0xffff9ab0); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = 0x5;
                { let v = s(c.f[0] * c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                c.f[0] = fctiwz(c.f[0]);
                { let ea = c.r[1].wrapping_add(0x58); let v = c.f[0]; c.stfd(ea, v); }
                c.r[3] = c.lwz(c.r[1].wrapping_add(0x5c));
                c.lr = 0x80138258; f_800b9404(c);
                pc = 0x80138258;
            }
            0x80138258 => {
                c.r[3] = 0x1;
                c.lr = 0x80138260; f_800d88dc(c);
                pc = 0x80138260;
            }
            0x80138260 => {
                c.r[3] = 0x6;
                c.lr = 0x80138268; f_800d888c(c);
                pc = 0x80138268;
            }
            0x80138268 => {
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x14));
                c.lr = 0x80138270; f_800d7820(c);
                pc = 0x80138270;
            }
            0x80138270 => {
                { let ea = c.r[1].wrapping_add(0x1c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x28);
                { let ea = c.r[1].wrapping_add(0x20); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x24); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x80138284; f_800e05c0(c);
                pc = 0x80138284;
            }
            0x80138284 => {
                c.r[3] = c.r[1].wrapping_add(0x28);
                c.lr = 0x8013828c; f_800d7f14(c);
                pc = 0x8013828c;
            }
            0x8013828c => {
                { let ea = c.r[25].wrapping_add(0x28); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x28);
                { let ea = c.r[25].wrapping_add(0x2c); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[25].wrapping_add(0x30); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x801382a0; f_800e048c(c);
                pc = 0x801382a0;
            }
            0x801382a0 => {
                c.r[3] = c.r[1].wrapping_add(0x28);
                c.lr = 0x801382a8; f_800d7f14(c);
                pc = 0x801382a8;
            }
            0x801382a8 => {
                c.r[3] = 0x2;
                c.lr = 0x801382b0; f_800d6a00(c);
                pc = 0x801382b0;
            }
            0x801382b0 => {
                c.r[26] = c.lwz(c.r[25].wrapping_add(0x4));
                c.r[28] = 0x0;
                pc = 0x80138350; continue;
                pc = 0x801382bc;
            }
            0x801382bc => {
                c.r[3] = c.lwz(c.r[26].wrapping_add(0x978));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80138348; continue; }
                pc = 0x801382c8;
            }
            0x801382c8 => {
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.r[3] = c.r[0].rotate_left(0) & 0xffff;
                c.lr = 0x801382d4; f_800d67bc(c);
                pc = 0x801382d4;
            }
            0x801382d4 => {
                { let ea = c.r[26].wrapping_add(0x960); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[26].wrapping_add(0x964); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[26].wrapping_add(0x968); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x801382e4; f_800d6680(c);
                pc = 0x801382e4;
            }
            0x801382e4 => {
                c.r[4] = c.lbz(c.r[25].wrapping_add(0x20));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[25].wrapping_add(0x21));
                c.r[6] = c.lbz(c.r[25].wrapping_add(0x22));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x23));
                c.lr = 0x801382fc; f_800d5cb8(c);
                pc = 0x801382fc;
            }
            0x801382fc => {
                c.r[29] = c.lwz(c.r[26].wrapping_add(0x978));
                c.r[30] = c.r[26] | c.r[26];
                c.r[31] = 0x0;
                pc = 0x8013833c; continue;
                pc = 0x8013830c;
            }
            0x8013830c => {
                { let ea = c.r[30]; c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[30].wrapping_add(0x4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[30].wrapping_add(0x8); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x8013831c; f_800d6680(c);
                pc = 0x8013831c;
            }
            0x8013831c => {
                c.r[4] = c.lbz(c.r[25].wrapping_add(0x20));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[25].wrapping_add(0x21));
                c.r[6] = c.lbz(c.r[25].wrapping_add(0x22));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x23));
                c.lr = 0x80138334; f_800d5cb8(c);
                pc = 0x80138334;
            }
            0x80138334 => {
                c.r[31] = c.r[31].wrapping_add(0x1);
                c.r[30] = c.r[30].wrapping_add(0x18);
                pc = 0x8013833c;
            }
            0x8013833c => {
                c.cmpu(0, c.r[31], c.r[29]);
                if c.crbit(0) { pc = 0x8013830c; continue; }
                pc = 0x80138344;
            }
            0x80138344 => {
                c.lr = 0x80138348; f_800d6728(c);
                pc = 0x80138348;
            }
            0x80138348 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.r[26] = c.r[26].wrapping_add(0x97c);
                pc = 0x80138350;
            }
            0x80138350 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[27]);
                if c.crbit(0) { pc = 0x801382bc; continue; }
                pc = 0x8013835c;
            }
            0x8013835c => {
                c.r[0] = c.lwz(c.r[25].wrapping_add(0x1c));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x801385f8; continue; }
                pc = 0x80138368;
            }
            0x80138368 => {
                c.r[3] = 0x1;
                c.r[4] = 0x2;
                c.r[5] = 0x1;
                c.lr = 0x80138378; f_800da1e8(c);
                pc = 0x80138378;
            }
            0x80138378 => {
                c.r[3] = 0x1;
                c.r[4] = 0x1;
                c.r[5] = 0x0;
                c.lr = 0x80138388; f_800da2bc(c);
                pc = 0x80138388;
            }
            0x80138388 => {
                c.r[3] = 0x3;
                c.lr = 0x80138390; f_800d88dc(c);
                pc = 0x80138390;
            }
            0x80138390 => {
                c.r[3] = 0x4;
                c.lr = 0x80138398; f_800d888c(c);
                pc = 0x80138398;
            }
            0x80138398 => {
                c.r[3] = c.lwz(c.r[25].wrapping_add(0x18));
                c.lr = 0x801383a0; f_800d7820(c);
                pc = 0x801383a0;
            }
            0x801383a0 => {
                c.r[4] = c.lwz(c.r[25].wrapping_add(0x1c));
                c.r[3] = 0x0;
                c.lr = 0x801383ac; f_800d85d4(c);
                pc = 0x801383ac;
            }
            0x801383ac => {
                c.r[3] = 0x6;
                c.lr = 0x801383b4; f_800d6a00(c);
                pc = 0x801383b4;
            }
            0x801383b4 => {
                c.r[3] = c.r[27].rotate_left(3) & 0xfff8;
                c.lr = 0x801383bc; f_800d67bc(c);
                pc = 0x801383bc;
            }
            0x801383bc => {
                c.r[31] = c.lwz(c.r[25].wrapping_add(0x4));
                c.r[30] = 0x0;
                { let ea = c.r[2].wrapping_add(0xffff9ab8); c.f[31] = c.lfs(ea); c.ps1[31] = c.f[31]; }
                pc = 0x801385e8; continue;
                pc = 0x801383cc;
            }
            0x801383cc => {
                c.lr = 0x801383d0; f_800e0ba0(c);
                pc = 0x801383d0;
            }
            0x801383d0 => {
                { let ea = c.r[25].wrapping_add(0x6c); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[25].wrapping_add(0x68); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0x960); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                { let v = s(c.f[2].mul_add(c.f[1], c.f[0])); c.f[30] = v; c.ps1[30] = v; }
                { let ea = c.r[31].wrapping_add(0x968); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0x964); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[3] - c.f[30]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] - c.f[30]); c.f[3] = v; c.ps1[3] = v; }
                c.lr = 0x801383f4; f_800d6680(c);
                pc = 0x801383f4;
            }
            0x801383f4 => {
                c.r[4] = c.lbz(c.r[25].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[25].wrapping_add(0x25));
                c.r[6] = c.lbz(c.r[25].wrapping_add(0x26));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x27));
                c.lr = 0x8013840c; f_800d5cb8(c);
                pc = 0x8013840c;
            }
            0x8013840c => {
                { let ea = c.r[2].wrapping_add(0xffff9ab4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                c.f[2] = c.f[1];
                c.lr = 0x8013841c; f_800d59b8(c);
                pc = 0x8013841c;
            }
            0x8013841c => {
                { let ea = c.r[31].wrapping_add(0x960); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[31].wrapping_add(0x968); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] + c.f[30]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[31].wrapping_add(0x964); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[0] - c.f[30]); c.f[3] = v; c.ps1[3] = v; }
                c.lr = 0x80138434; f_800d6680(c);
                pc = 0x80138434;
            }
            0x80138434 => {
                c.r[4] = c.lbz(c.r[25].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[25].wrapping_add(0x25));
                c.r[6] = c.lbz(c.r[25].wrapping_add(0x26));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x27));
                c.lr = 0x8013844c; f_800d5cb8(c);
                pc = 0x8013844c;
            }
            0x8013844c => {
                { let ea = c.r[2].wrapping_add(0xffff9ab4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                { let ea = c.r[2].wrapping_add(0xffff9ab8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.lr = 0x8013845c; f_800d59b8(c);
                pc = 0x8013845c;
            }
            0x8013845c => {
                { let ea = c.r[31].wrapping_add(0x960); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[31].wrapping_add(0x968); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] + c.f[30]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[31].wrapping_add(0x964); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[0] + c.f[30]); c.f[3] = v; c.ps1[3] = v; }
                c.lr = 0x80138474; f_800d6680(c);
                pc = 0x80138474;
            }
            0x80138474 => {
                c.r[4] = c.lbz(c.r[25].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[25].wrapping_add(0x25));
                c.r[6] = c.lbz(c.r[25].wrapping_add(0x26));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x27));
                c.lr = 0x8013848c; f_800d5cb8(c);
                pc = 0x8013848c;
            }
            0x8013848c => {
                { let ea = c.r[2].wrapping_add(0xffff9ab8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                c.f[2] = c.f[1];
                c.lr = 0x8013849c; f_800d59b8(c);
                pc = 0x8013849c;
            }
            0x8013849c => {
                { let ea = c.r[31].wrapping_add(0x960); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[31].wrapping_add(0x968); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] - c.f[30]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[31].wrapping_add(0x964); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[0] + c.f[30]); c.f[3] = v; c.ps1[3] = v; }
                c.lr = 0x801384b4; f_800d6680(c);
                pc = 0x801384b4;
            }
            0x801384b4 => {
                c.r[4] = c.lbz(c.r[25].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[25].wrapping_add(0x25));
                c.r[6] = c.lbz(c.r[25].wrapping_add(0x26));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x27));
                c.lr = 0x801384cc; f_800d5cb8(c);
                pc = 0x801384cc;
            }
            0x801384cc => {
                { let ea = c.r[2].wrapping_add(0xffff9ab8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                { let ea = c.r[2].wrapping_add(0xffff9ab4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.lr = 0x801384dc; f_800d59b8(c);
                pc = 0x801384dc;
            }
            0x801384dc => {
                { let v = s(c.f[30] * c.f[31]); c.f[30] = v; c.ps1[30] = v; }
                { let ea = c.r[31].wrapping_add(0x96c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[31].wrapping_add(0x974); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let ea = c.r[31].wrapping_add(0x970); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[1] - c.f[30]); c.f[1] = v; c.ps1[1] = v; }
                { let v = s(c.f[0] - c.f[30]); c.f[3] = v; c.ps1[3] = v; }
                c.lr = 0x801384f8; f_800d6680(c);
                pc = 0x801384f8;
            }
            0x801384f8 => {
                c.r[4] = c.lbz(c.r[25].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[25].wrapping_add(0x25));
                c.r[6] = c.lbz(c.r[25].wrapping_add(0x26));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x27));
                c.lr = 0x80138510; f_800d5cb8(c);
                pc = 0x80138510;
            }
            0x80138510 => {
                { let ea = c.r[2].wrapping_add(0xffff9ab4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                c.f[2] = c.f[1];
                c.lr = 0x80138520; f_800d59b8(c);
                pc = 0x80138520;
            }
            0x80138520 => {
                { let ea = c.r[31].wrapping_add(0x96c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[31].wrapping_add(0x974); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] + c.f[30]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[31].wrapping_add(0x970); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[0] - c.f[30]); c.f[3] = v; c.ps1[3] = v; }
                c.lr = 0x80138538; f_800d6680(c);
                pc = 0x80138538;
            }
            0x80138538 => {
                c.r[4] = c.lbz(c.r[25].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[25].wrapping_add(0x25));
                c.r[6] = c.lbz(c.r[25].wrapping_add(0x26));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x27));
                c.lr = 0x80138550; f_800d5cb8(c);
                pc = 0x80138550;
            }
            0x80138550 => {
                { let ea = c.r[2].wrapping_add(0xffff9ab4); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                { let ea = c.r[2].wrapping_add(0xffff9ab8); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.lr = 0x80138560; f_800d59b8(c);
                pc = 0x80138560;
            }
            0x80138560 => {
                { let ea = c.r[31].wrapping_add(0x96c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[31].wrapping_add(0x974); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] + c.f[30]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[31].wrapping_add(0x970); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[0] + c.f[30]); c.f[3] = v; c.ps1[3] = v; }
                c.lr = 0x80138578; f_800d6680(c);
                pc = 0x80138578;
            }
            0x80138578 => {
                c.r[4] = c.lbz(c.r[25].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[25].wrapping_add(0x25));
                c.r[6] = c.lbz(c.r[25].wrapping_add(0x26));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x27));
                c.lr = 0x80138590; f_800d5cb8(c);
                pc = 0x80138590;
            }
            0x80138590 => {
                { let ea = c.r[2].wrapping_add(0xffff9ab8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                c.f[2] = c.f[1];
                c.lr = 0x801385a0; f_800d59b8(c);
                pc = 0x801385a0;
            }
            0x801385a0 => {
                { let ea = c.r[31].wrapping_add(0x96c); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                { let ea = c.r[31].wrapping_add(0x974); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[1] - c.f[30]); c.f[1] = v; c.ps1[1] = v; }
                { let ea = c.r[31].wrapping_add(0x970); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let v = s(c.f[0] + c.f[30]); c.f[3] = v; c.ps1[3] = v; }
                c.lr = 0x801385b8; f_800d6680(c);
                pc = 0x801385b8;
            }
            0x801385b8 => {
                c.r[4] = c.lbz(c.r[25].wrapping_add(0x24));
                c.r[3] = 0x0;
                c.r[5] = c.lbz(c.r[25].wrapping_add(0x25));
                c.r[6] = c.lbz(c.r[25].wrapping_add(0x26));
                c.r[7] = c.lbz(c.r[25].wrapping_add(0x27));
                c.lr = 0x801385d0; f_800d5cb8(c);
                pc = 0x801385d0;
            }
            0x801385d0 => {
                { let ea = c.r[2].wrapping_add(0xffff9ab8); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = 0x0;
                { let ea = c.r[2].wrapping_add(0xffff9ab4); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.lr = 0x801385e0; f_800d59b8(c);
                pc = 0x801385e0;
            }
            0x801385e0 => {
                c.r[30] = c.r[30].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x97c);
                pc = 0x801385e8;
            }
            0x801385e8 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[27]);
                if c.crbit(0) { pc = 0x801383cc; continue; }
                pc = 0x801385f4;
            }
            0x801385f4 => {
                c.lr = 0x801385f8; f_800d6728(c);
                pc = 0x801385f8;
            }
            0x801385f8 => {
                c.lr = 0x801385fc; f_800d7e5c(c);
                pc = 0x801385fc;
            }
            0x801385fc => {
                c.lr = 0x80138600; f_800d7e5c(c);
                pc = 0x80138600;
            }
            0x80138600 => {
                c.r[3] = 0x1;
                pc = 0x8013860c; continue;
                pc = 0x80138608;
            }
            0x80138608 => {
                c.r[3] = 0x0;
                pc = 0x8013860c;
            }
            0x8013860c => {
                { let ea = c.r[1].wrapping_add(0x98); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0x90); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0x88); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0x80); c.f[30] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x64); for n in 25..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xa4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xa0);
                if true { return; }
                return;
            }
            _ => panic!("f_8013814c: bad pc {:#x}", pc),
        }
    }
}

/// fn_80138630 (0x80138630, 80 bytes)
#[inline(never)]
pub fn f_80138630(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80138630) { return; }
        if c.prof { crate::prof::enter(c, 0x80138630); f_80138630_b(c, 0x80138630); crate::prof::exit(c, 0x80138630); return; }
    }
    f_80138630_b(c, 0x80138630)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80138630_at(c: &mut Cpu, pc: u32) { f_80138630_b(c, pc) }
#[inline(always)]
fn f_80138630_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80138630;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80138630, pc); }
        match pc {
            0x80138630 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x80138668; continue; }
                pc = 0x80138648;
            }
            0x80138648 => {
                c.lr = 0x8013864c; f_800b8df4(c);
                pc = 0x8013864c;
            }
            0x8013864c => {
                c.lr = 0x80138650; f_800b856c(c);
                pc = 0x80138650;
            }
            0x80138650 => {
                c.r[3] = c.lwz(c.r[31].wrapping_add(0x1c));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80138660; continue; }
                pc = 0x8013865c;
            }
            0x8013865c => {
                c.lr = 0x80138660; f_800ef5a4(c);
                pc = 0x80138660;
            }
            0x80138660 => {
                c.r[3] = 0x1;
                pc = 0x8013866c; continue;
                pc = 0x80138668;
            }
            0x80138668 => {
                c.r[3] = 0x0;
                pc = 0x8013866c;
            }
            0x8013866c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80138630: bad pc {:#x}", pc),
        }
    }
}

/// fn_80138680 (0x80138680, 92 bytes)
#[inline(never)]
pub fn f_80138680(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80138680) { return; }
        if c.prof { crate::prof::enter(c, 0x80138680); f_80138680_b(c, 0x80138680); crate::prof::exit(c, 0x80138680); return; }
    }
    f_80138680_b(c, 0x80138680)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80138680_at(c: &mut Cpu, pc: u32) { f_80138680_b(c, pc) }
#[inline(always)]
fn f_80138680_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80138680;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80138680, pc); }
        match pc {
            0x80138680 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                if c.crbit(2) { pc = 0x801386c4; continue; }
                pc = 0x80138698;
            }
            0x80138698 => {
                c.r[31] = c.lhz(c.r[3]);
                c.lr = 0x801386a0; f_800b8df4(c);
                pc = 0x801386a0;
            }
            0x801386a0 => {
                c.lr = 0x801386a4; f_800b856c(c);
                pc = 0x801386a4;
            }
            0x801386a4 => {
                c.cmpu(0, c.r[31], 0x0);
                if c.crbit(2) { pc = 0x801386bc; continue; }
                pc = 0x801386ac;
            }
            0x801386ac => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801386b4; f_800e24b0(c);
                pc = 0x801386b4;
            }
            0x801386b4 => {
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x801386bc; f_800e209c(c);
                pc = 0x801386bc;
            }
            0x801386bc => {
                c.r[3] = 0x1;
                pc = 0x801386c8; continue;
                pc = 0x801386c4;
            }
            0x801386c4 => {
                c.r[3] = 0x0;
                pc = 0x801386c8;
            }
            0x801386c8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80138680: bad pc {:#x}", pc),
        }
    }
}

/// fn_801386DC (0x801386dc, 228 bytes)
#[inline(never)]
pub fn f_801386dc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801386dc) { return; }
        if c.prof { crate::prof::enter(c, 0x801386dc); f_801386dc_b(c, 0x801386dc); crate::prof::exit(c, 0x801386dc); return; }
    }
    f_801386dc_b(c, 0x801386dc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801386dc_at(c: &mut Cpu, pc: u32) { f_801386dc_b(c, pc) }
#[inline(always)]
fn f_801386dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801386dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801386dc, pc); }
        match pc {
            0x801386dc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.r[4] = 0x80270000;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[4].wrapping_add(0x2b40);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                if c.crbit(2) { pc = 0x80138798; continue; }
                pc = 0x80138700;
            }
            0x80138700 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[3] = c.lhz(c.r[3].wrapping_add(0xa));
                c.r[4] = c.lhz(c.r[30].wrapping_add(0xc));
                c.lr = 0x80138710; f_800f9318(c);
                pc = 0x80138710;
            }
            0x80138710 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x80138734; continue; }
                pc = 0x80138718;
            }
            0x80138718 => {
                c.r[4] = c.lhz(c.r[30].wrapping_add(0xa));
                c.r[3] = c.r[31].wrapping_add(0x0);
                c.r[5] = c.lhz(c.r[30].wrapping_add(0xc));
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x8013872c; stubs_gen::f_800dd970(c);
                pc = 0x8013872c;
            }
            0x8013872c => {
                c.r[3] = 0x0;
                pc = 0x801387a8; continue;
                pc = 0x80138734;
            }
            0x80138734 => {
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x8));
                c.r[3] = c.r[0].wrapping_mul(0x97c);
                c.lr = 0x80138740; f_800e3534(c);
                pc = 0x80138740;
            }
            0x80138740 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.sth(c.r[30], c.r[3]);
                if !c.crbit(2) { pc = 0x80138760; continue; }
                pc = 0x8013874c;
            }
            0x8013874c => {
                c.r[3] = c.r[31].wrapping_add(0x60);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80138758; stubs_gen::f_800dd970(c);
                pc = 0x80138758;
            }
            0x80138758 => {
                c.r[3] = 0x0;
                pc = 0x801387a8; continue;
                pc = 0x80138760;
            }
            0x80138760 => {
                c.lr = 0x80138764; f_800e27b0(c);
                pc = 0x80138764;
            }
            0x80138764 => {
                c.stw(c.r[30].wrapping_add(0x4), c.r[3]);
                c.r[4] = 0x0;
                c.r[0] = c.lhz(c.r[30].wrapping_add(0x8));
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x4));
                c.r[5] = c.r[0].wrapping_mul(0x97c);
                c.lr = 0x8013877c; f_8000540c(c);
                pc = 0x8013877c;
            }
            0x8013877c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = 0x1;
                c.lr = 0x80138788; f_80138838(c);
                pc = 0x80138788;
            }
            0x80138788 => {
                c.r[0] = 0x0;
                c.r[3] = 0x1;
                c.sth(c.r[30].wrapping_add(0x10), c.r[0]);
                pc = 0x801387a8; continue;
                pc = 0x80138798;
            }
            0x80138798 => {
                c.r[3] = c.r[31].wrapping_add(0xb0);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x801387a4; stubs_gen::f_800dd970(c);
                pc = 0x801387a4;
            }
            0x801387a4 => {
                c.r[3] = 0x0;
                pc = 0x801387a8;
            }
            0x801387a8 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801386dc: bad pc {:#x}", pc),
        }
    }
}

/// fn_801387C0 (0x801387c0, 120 bytes)
#[inline(never)]
pub fn f_801387c0(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x801387c0) { return; }
        if c.prof { crate::prof::enter(c, 0x801387c0); f_801387c0_b(c, 0x801387c0); crate::prof::exit(c, 0x801387c0); return; }
    }
    f_801387c0_b(c, 0x801387c0)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_801387c0_at(c: &mut Cpu, pc: u32) { f_801387c0_b(c, pc) }
#[inline(always)]
fn f_801387c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x801387c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x801387c0, pc); }
        match pc {
            0x801387c0 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x8), c.r[30]);
                c.r[30] = c.r[4] | c.r[4];
                if c.crbit(2) { pc = 0x8013881c; continue; }
                pc = 0x801387e0;
            }
            0x801387e0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x12));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x801387fc; continue; }
                pc = 0x801387f0;
            }
            0x801387f0 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x10));
                c.cmpu(0, c.r[0], c.r[3]);
                if !c.crbit(0) { pc = 0x8013881c; continue; }
                pc = 0x801387fc;
            }
            0x801387fc => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x80138808; f_80138838(c);
                pc = 0x80138808;
            }
            0x80138808 => {
                c.r[0] = c.lhz(c.r[31].wrapping_add(0x10));
                c.r[3] = 0x1;
                c.r[0] = c.r[0].wrapping_add(c.r[30]);
                c.sth(c.r[31].wrapping_add(0x10), c.r[0]);
                pc = 0x80138820; continue;
                pc = 0x8013881c;
            }
            0x8013881c => {
                c.r[3] = 0x0;
                pc = 0x80138820;
            }
            0x80138820 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.r[30] = c.lwz(c.r[1].wrapping_add(0x8));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_801387c0: bad pc {:#x}", pc),
        }
    }
}

/// fn_80138838 (0x80138838, 712 bytes)
#[inline(never)]
pub fn f_80138838(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80138838) { return; }
        if c.prof { crate::prof::enter(c, 0x80138838); f_80138838_b(c, 0x80138838); crate::prof::exit(c, 0x80138838); return; }
    }
    f_80138838_b(c, 0x80138838)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80138838_at(c: &mut Cpu, pc: u32) { f_80138838_b(c, pc) }
#[inline(always)]
fn f_80138838_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80138838;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80138838, pc); }
        match pc {
            0x80138838 => {
                { let ea = c.r[1].wrapping_add(0xffffff30); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0xd4), c.r[0]);
                { let ea = c.r[1].wrapping_add(0xc0); let v = c.f[31]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0xc8); let (x, y) = (c.f[31], c.ps1[31]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0xb0); let v = c.f[30]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0xb8); let (x, y) = (c.f[30], c.ps1[30]); c.psq_store(ea, x, y, false, 0); }
                { let ea = c.r[1].wrapping_add(0xa0); let v = c.f[29]; c.stfd(ea, v); }
                { let ea = c.r[1].wrapping_add(0xa8); let (x, y) = (c.f[29], c.ps1[29]); c.psq_store(ea, x, y, false, 0); }
                { let mut ea = c.r[1].wrapping_add(0x80); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[24] = c.r[3] | c.r[3];
                { let ea = c.r[2].wrapping_add(0xffff9ab4); c.f[29] = c.lfs(ea); c.ps1[29] = c.f[29]; }
                c.r[30] = c.lwz(c.r[3].wrapping_add(0x4));
                c.r[25] = c.r[4] | c.r[4];
                c.r[27] = c.lhz(c.r[3].wrapping_add(0x8));
                c.r[29] = 0x0;
                pc = 0x80138ac8; continue;
                pc = 0x8013887c;
            }
            0x8013887c => {
                c.cmpu(0, c.r[25], 0x0);
                if c.crbit(2) { pc = 0x801388d0; continue; }
                pc = 0x80138884;
            }
            0x80138884 => {
                c.lr = 0x80138888; f_800e0ba0(c);
                pc = 0x80138888;
            }
            0x80138888 => {
                { let ea = c.r[24].wrapping_add(0x58); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[24].wrapping_add(0x54); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[2].mul_add(c.f[1], c.f[0])); c.f[31] = v; c.ps1[31] = v; }
                c.lr = 0x80138898; f_800e0be4(c);
                pc = 0x80138898;
            }
            0x80138898 => {
                { let ea = c.r[2].wrapping_add(0xffff9abc); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] * c.f[1]); c.f[30] = v; c.ps1[30] = v; }
                c.f[1] = c.f[30];
                c.lr = 0x801388a8; f_800cdbe0(c);
                pc = 0x801388a8;
            }
            0x801388a8 => {
                { let v = s(c.f[1]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[2].wrapping_add(0xffff9ab4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[1] = c.f[30];
                { let v = s(c.f[31] * c.f[2]); c.f[2] = v; c.ps1[2] = v; }
                { let ea = c.r[30].wrapping_add(0x960); let v = c.f[2]; c.stfs(ea, v); }
                { let ea = c.r[30].wrapping_add(0x964); let v = c.f[0]; c.stfs(ea, v); }
                c.lr = 0x801388c4; f_800ce148(c);
                pc = 0x801388c4;
            }
            0x801388c4 => {
                { let v = s(c.f[1]); c.f[0] = v; c.ps1[0] = v; }
                { let v = s(c.f[31] * c.f[0]); c.f[0] = v; c.ps1[0] = v; }
                { let ea = c.r[30].wrapping_add(0x968); let v = c.f[0]; c.stfs(ea, v); }
                pc = 0x801388d0;
            }
            0x801388d0 => {
                c.r[3] = c.r[1].wrapping_add(0x24);
                c.r[4] = c.r[30].wrapping_add(0x960);
                c.lr = 0x801388dc; f_800e0060(c);
                pc = 0x801388dc;
            }
            0x801388dc => {
                { let ea = c.r[1].wrapping_add(0x24); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[1].wrapping_add(0x3c);
                { let ea = c.r[2].wrapping_add(0xffff9aac); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                { let ea = c.r[1].wrapping_add(0x2c); c.f[3] = c.lfs(ea); c.ps1[3] = c.f[3]; }
                c.lr = 0x801388f0; f_800e01f4(c);
                pc = 0x801388f0;
            }
            0x801388f0 => {
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x801388fc; f_800e0060(c);
                pc = 0x801388fc;
            }
            0x801388fc => {
                c.r[0] = 0x0;
                c.r[31] = c.r[30] | c.r[30];
                c.stw(c.r[30].wrapping_add(0x978), c.r[0]);
                c.r[26] = c.r[30].wrapping_add(0x960);
                c.r[28] = 0x0;
                pc = 0x80138ab4; continue;
                pc = 0x80138914;
            }
            0x80138914 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.r[4] = c.r[1].wrapping_add(0x3c);
                c.lr = 0x80138920; f_800e01d0(c);
                pc = 0x80138920;
            }
            0x80138920 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                { let ea = c.r[1].wrapping_add(0x1c); let v = c.f[29]; c.stfs(ea, v); }
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80138930; f_800e0060(c);
                pc = 0x80138930;
            }
            0x80138930 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.r[4] = c.r[1].wrapping_add(0x3c);
                c.r[5] = c.r[3] | c.r[3];
                c.lr = 0x80138940; f_800dffcc(c);
                pc = 0x80138940;
            }
            0x80138940 => {
                c.r[3] = c.r[1].wrapping_add(0x18);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x8013894c; f_800e0060(c);
                pc = 0x8013894c;
            }
            0x8013894c => {
                c.lr = 0x80138950; f_800e0ba0(c);
                pc = 0x80138950;
            }
            0x80138950 => {
                { let ea = c.r[24].wrapping_add(0x4c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x8);
                c.r[4] = c.r[1].wrapping_add(0x18);
                { let v = s(c.f[0] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x80138964; f_800e0718(c);
                pc = 0x80138964;
            }
            0x80138964 => {
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[4] = c.r[1].wrapping_add(0x8);
                c.r[5] = c.r[3] | c.r[3];
                c.lr = 0x80138974; f_800dfeec(c);
                pc = 0x80138974;
            }
            0x80138974 => {
                c.lr = 0x80138978; f_800e0ba0(c);
                pc = 0x80138978;
            }
            0x80138978 => {
                { let ea = c.r[24].wrapping_add(0x50); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[3] = c.r[1].wrapping_add(0x48);
                { let v = s(c.f[0] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x80138988; f_800e0518(c);
                pc = 0x80138988;
            }
            0x80138988 => {
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[4] = c.r[1].wrapping_add(0x48);
                c.r[5] = c.r[3] | c.r[3];
                c.lr = 0x80138998; f_800dff98(c);
                pc = 0x80138998;
            }
            0x80138998 => {
                c.cmpu(0, c.r[25], 0x0);
                if c.crbit(2) { pc = 0x801389b0; continue; }
                pc = 0x801389a0;
            }
            0x801389a0 => {
                c.r[3] = c.r[31].wrapping_add(0xc);
                c.r[4] = c.r[1].wrapping_add(0x3c);
                c.lr = 0x801389ac; f_800e01d0(c);
                pc = 0x801389ac;
            }
            0x801389ac => {
                pc = 0x801389f4; continue;
                pc = 0x801389b0;
            }
            0x801389b0 => {
                { let ea = c.r[2].wrapping_add(0xffff9aac); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[3] = c.r[31].wrapping_add(0xc);
                { let ea = c.r[24].wrapping_add(0x64); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.r[3] | c.r[3];
                { let v = s(c.f[1] - c.f[0]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x801389c8; f_800e013c(c);
                pc = 0x801389c8;
            }
            0x801389c8 => {
                c.r[3] = c.r[1].wrapping_add(0x3c);
                { let ea = c.r[24].wrapping_add(0x64); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x801389d8; f_800e013c(c);
                pc = 0x801389d8;
            }
            0x801389d8 => {
                c.r[3] = c.r[31].wrapping_add(0xc);
                c.r[4] = c.r[1].wrapping_add(0x3c);
                c.r[5] = c.r[3] | c.r[3];
                c.lr = 0x801389e8; f_800e019c(c);
                pc = 0x801389e8;
            }
            0x801389e8 => {
                c.r[3] = c.r[31].wrapping_add(0xc);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x801389f4; f_800e0060(c);
                pc = 0x801389f4;
            }
            0x801389f4 => {
                c.lr = 0x801389f8; f_800e0ba0(c);
                pc = 0x801389f8;
            }
            0x801389f8 => {
                { let ea = c.r[24].wrapping_add(0x60); c.f[2] = c.lfs(ea); c.ps1[2] = c.f[2]; }
                c.r[3] = c.r[1].wrapping_add(0x3c);
                { let ea = c.r[24].wrapping_add(0x5c); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.r[4] = c.r[31].wrapping_add(0xc);
                { let v = s(c.f[2].mul_add(c.f[1], c.f[0])); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x80138a10; f_800e013c(c);
                pc = 0x80138a10;
            }
            0x80138a10 => {
                c.lr = 0x80138a14; f_800e0ba0(c);
                pc = 0x80138a14;
            }
            0x80138a14 => {
                { let ea = c.r[24].wrapping_add(0x48); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] * c.f[1]); c.f[30] = v; c.ps1[30] = v; }
                c.lr = 0x80138a20; f_800e0ba0(c);
                pc = 0x80138a20;
            }
            0x80138a20 => {
                { let ea = c.r[24].wrapping_add(0x44); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                { let v = s(c.f[0] * c.f[1]); c.f[31] = v; c.ps1[31] = v; }
                c.lr = 0x80138a2c; f_800e0ba0(c);
                pc = 0x80138a2c;
            }
            0x80138a2c => {
                { let ea = c.r[24].wrapping_add(0x40); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.f[2] = c.f[31];
                c.f[3] = c.f[30];
                c.r[3] = c.r[1].wrapping_add(0x30);
                { let v = s(c.f[0] * c.f[1]); c.f[1] = v; c.ps1[1] = v; }
                c.lr = 0x80138a44; f_800e01f4(c);
                pc = 0x80138a44;
            }
            0x80138a44 => {
                c.r[3] = c.r[1].wrapping_add(0x30);
                c.r[5] = c.r[24].wrapping_add(0x34);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80138a54; f_800e019c(c);
                pc = 0x80138a54;
            }
            0x80138a54 => {
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[5] = c.r[1].wrapping_add(0x30);
                c.r[4] = c.r[3] | c.r[3];
                c.lr = 0x80138a64; f_800e019c(c);
                pc = 0x80138a64;
            }
            0x80138a64 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = c.r[26] | c.r[26];
                c.r[5] = c.r[1].wrapping_add(0x3c);
                c.lr = 0x80138a74; f_800e019c(c);
                pc = 0x80138a74;
            }
            0x80138a74 => {
                c.r[3] = c.r[1].wrapping_add(0x3c);
                c.r[4] = c.r[31].wrapping_add(0xc);
                c.lr = 0x80138a80; f_800e01d0(c);
                pc = 0x80138a80;
            }
            0x80138a80 => {
                c.r[3] = c.lwz(c.r[30].wrapping_add(0x978));
                c.r[0] = c.r[3].wrapping_add(0x1);
                c.stw(c.r[30].wrapping_add(0x978), c.r[0]);
                { let ea = c.r[31].wrapping_add(0x4); c.f[0] = c.lfs(ea); c.ps1[0] = c.f[0]; }
                c.fcmp(0, c.f[0], c.f[29]);
                if !c.crbit(0) { pc = 0x80138aa8; continue; }
                pc = 0x80138a98;
            }
            0x80138a98 => {
                c.r[4] = c.r[26] | c.r[26];
                c.r[3] = c.r[30].wrapping_add(0x96c);
                c.lr = 0x80138aa4; f_800e01d0(c);
                pc = 0x80138aa4;
            }
            0x80138aa4 => {
                pc = 0x80138ac0; continue;
                pc = 0x80138aa8;
            }
            0x80138aa8 => {
                c.r[26] = c.r[31] | c.r[31];
                c.r[28] = c.r[28].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x18);
                pc = 0x80138ab4;
            }
            0x80138ab4 => {
                c.r[0] = c.r[28].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x64);
                if c.crbit(0) { pc = 0x80138914; continue; }
                pc = 0x80138ac0;
            }
            0x80138ac0 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[30] = c.r[30].wrapping_add(0x97c);
                pc = 0x80138ac8;
            }
            0x80138ac8 => {
                c.r[0] = c.r[29].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], c.r[27]);
                if c.crbit(0) { pc = 0x8013887c; continue; }
                pc = 0x80138ad4;
            }
            0x80138ad4 => {
                { let ea = c.r[1].wrapping_add(0xc8); let (x, y) = c.psq_load(ea, false, 0); c.f[31] = x; c.ps1[31] = y; }
                { let ea = c.r[1].wrapping_add(0xc0); c.f[31] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0xb8); let (x, y) = c.psq_load(ea, false, 0); c.f[30] = x; c.ps1[30] = y; }
                { let ea = c.r[1].wrapping_add(0xb0); c.f[30] = c.lfd(ea); }
                { let ea = c.r[1].wrapping_add(0xa8); let (x, y) = c.psq_load(ea, false, 0); c.f[29] = x; c.ps1[29] = y; }
                { let ea = c.r[1].wrapping_add(0xa0); c.f[29] = c.lfd(ea); }
                { let mut ea = c.r[1].wrapping_add(0x80); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[0] = c.lwz(c.r[1].wrapping_add(0xd4));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0xd0);
                if true { return; }
                return;
            }
            _ => panic!("f_80138838: bad pc {:#x}", pc),
        }
    }
}

/// leaffxStartEffect (0x80138b00, 116 bytes)
#[inline(never)]
pub fn f_80138b00(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80138b00) { return; }
        if c.prof { crate::prof::enter(c, 0x80138b00); f_80138b00_b(c, 0x80138b00); crate::prof::exit(c, 0x80138b00); return; }
    }
    f_80138b00_b(c, 0x80138b00)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80138b00_at(c: &mut Cpu, pc: u32) { f_80138b00_b(c, pc) }
#[inline(always)]
fn f_80138b00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80138b00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80138b00, pc); }
        match pc {
            0x80138b00 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.r[4] = 0x4c;
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.lr = 0x80138b18; f_80131428(c);
                pc = 0x80138b18;
            }
            0x80138b18 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x80138b5c; continue; }
                pc = 0x80138b20;
            }
            0x80138b20 => {
                c.r[5] = 0x80140000;
                c.r[6] = 0x80140000;
                c.r[7] = 0x80140000;
                c.r[4] = 0x80140000;
                c.r[9] = c.r[4].wrapping_add(0xffff8de4);
                c.r[5] = c.r[5].wrapping_add(0xffff8b74);
                c.r[6] = c.r[6].wrapping_add(0xffff8ccc);
                c.r[7] = c.r[7].wrapping_add(0xffff8bbc);
                c.r[4] = 0x0;
                c.r[8] = 0x0;
                c.r[10] = 0x0;
                c.lr = 0x80138b50; f_80131200(c);
                pc = 0x80138b50;
            }
            0x80138b50 => {
                c.r[3] = c.r[31] | c.r[31];
                c.r[4] = 0x0;
                c.lr = 0x80138b5c; f_8013139c(c);
                pc = 0x80138b5c;
            }
            0x80138b5c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = c.r[31] | c.r[31];
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80138b00: bad pc {:#x}", pc),
        }
    }
}

/// fn_80138B74 (0x80138b74, 72 bytes)
#[inline(never)]
pub fn f_80138b74(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80138b74) { return; }
        if c.prof { crate::prof::enter(c, 0x80138b74); f_80138b74_b(c, 0x80138b74); crate::prof::exit(c, 0x80138b74); return; }
    }
    f_80138b74_b(c, 0x80138b74)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80138b74_at(c: &mut Cpu, pc: u32) { f_80138b74_b(c, pc) }
#[inline(always)]
fn f_80138b74_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80138b74;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80138b74, pc); }
        match pc {
            0x80138b74 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x80138ba4; continue; }
                pc = 0x80138b8c;
            }
            0x80138b8c => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x40));
                c.r[4] = c.lhz(c.r[31].wrapping_add(0x44));
                c.lr = 0x80138b98; f_800f9210(c);
                pc = 0x80138b98;
            }
            0x80138b98 => {
                c.r[3] = c.lhz(c.r[31].wrapping_add(0x40));
                c.r[4] = c.lhz(c.r[31].wrapping_add(0x42));
                c.lr = 0x80138ba4; f_800f9210(c);
                pc = 0x80138ba4;
            }
            0x80138ba4 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[3] = 0x1;
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_80138b74: bad pc {:#x}", pc),
        }
    }
}

/// fn_80138BBC (0x80138bbc, 272 bytes)
#[inline(never)]
pub fn f_80138bbc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80138bbc) { return; }
        if c.prof { crate::prof::enter(c, 0x80138bbc); f_80138bbc_b(c, 0x80138bbc); crate::prof::exit(c, 0x80138bbc); return; }
    }
    f_80138bbc_b(c, 0x80138bbc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80138bbc_at(c: &mut Cpu, pc: u32) { f_80138bbc_b(c, pc) }
#[inline(always)]
fn f_80138bbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80138bbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80138bbc, pc); }
        match pc {
            0x80138bbc => {
                { let ea = c.r[1].wrapping_add(0xffffffd0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x34), c.r[0]);
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { let v = c.r[n]; c.stw(ea, v); ea = ea.wrapping_add(4); } }
                c.r[28] = c.r[3] | c.r[3];
                c.rc(c.r[28]);
                if c.crbit(2) { pc = 0x80138cb4; continue; }
                pc = 0x80138bd4;
            }
            0x80138bd4 => {
                c.r[26] = c.lhz(c.r[28].wrapping_add(0x38));
                c.r[25] = 0x0;
                c.r[27] = c.lwz(c.r[28]);
                c.r[30] = 0x0;
                c.r[24] = c.lhz(c.r[28].wrapping_add(0x4));
                pc = 0x80138c14; continue;
                pc = 0x80138bec;
            }
            0x80138bec => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x58));
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x80138c0c; continue; }
                pc = 0x80138bf8;
            }
            0x80138bf8 => {
                c.r[4] = 0x0;
                c.lr = 0x80138c00; f_800e4014(c);
                pc = 0x80138c00;
            }
            0x80138c00 => {
                c.r[3] = c.lwz(c.r[27].wrapping_add(0x58));
                c.lr = 0x80138c08; f_800e4bf4(c);
                pc = 0x80138c08;
            }
            0x80138c08 => {
                c.stw(c.r[27].wrapping_add(0x58), c.r[30]);
                pc = 0x80138c0c;
            }
            0x80138c0c => {
                c.r[25] = c.r[25].wrapping_add(0x1);
                c.r[27] = c.r[27].wrapping_add(0x5c);
                pc = 0x80138c14;
            }
            0x80138c14 => {
                c.cmpu(0, c.r[25], c.r[26]);
                if c.crbit(0) { pc = 0x80138bec; continue; }
                pc = 0x80138c1c;
            }
            0x80138c1c => {
                c.r[25] = c.lwz(c.r[28].wrapping_add(0x8));
                c.r[3] = c.r[25] | c.r[25];
                c.lr = 0x80138c28; f_800ee0e8(c);
                pc = 0x80138c28;
            }
            0x80138c28 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[28] = 0x0;
                pc = 0x80138c94; continue;
                pc = 0x80138c34;
            }
            0x80138c34 => {
                c.r[3] = c.r[25] | c.r[25];
                c.r[4] = c.r[28] | c.r[28];
                c.lr = 0x80138c40; f_800ee150(c);
                pc = 0x80138c40;
            }
            0x80138c40 => {
                c.r[26] = c.r[3] | c.r[3];
                c.rc(c.r[26]);
                if c.crbit(2) { pc = 0x80138c90; continue; }
                pc = 0x80138c48;
            }
            0x80138c48 => {
                c.lr = 0x80138c4c; f_800ee758(c);
                pc = 0x80138c4c;
            }
            0x80138c4c => {
                c.r[31] = c.r[3] | c.r[3];
                c.r[29] = 0x0;
                pc = 0x80138c80; continue;
                pc = 0x80138c58;
            }
            0x80138c58 => {
                c.r[3] = c.r[26] | c.r[26];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x80138c64; f_800ee6b4(c);
                pc = 0x80138c64;
            }
            0x80138c64 => {
                c.r[27] = c.r[3] | c.r[3];
                c.rc(c.r[27]);
                if c.crbit(2) { pc = 0x80138c7c; continue; }
                pc = 0x80138c6c;
            }
            0x80138c6c => {
                { let ea = c.r[2].wrapping_add(0xffff9ac0); c.f[1] = c.lfs(ea); c.ps1[1] = c.f[1]; }
                c.lr = 0x80138c74; f_800df21c(c);
                pc = 0x80138c74;
            }
            0x80138c74 => {
                c.r[3] = c.r[27] | c.r[27];
                c.lr = 0x80138c7c; f_800df608(c);
                pc = 0x80138c7c;
            }
            0x80138c7c => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                pc = 0x80138c80;
            }
            0x80138c80 => {
                c.cmpu(0, c.r[29], c.r[31]);
                if c.crbit(0) { pc = 0x80138c58; continue; }
                pc = 0x80138c88;
            }
            0x80138c88 => {
                c.r[3] = c.r[26] | c.r[26];
                c.lr = 0x80138c90; f_800ee828(c);
                pc = 0x80138c90;
            }
            0x80138c90 => {
                c.r[28] = c.r[28].wrapping_add(0x1);
                pc = 0x80138c94;
            }
            0x80138c94 => {
                c.cmpu(0, c.r[28], c.r[30]);
                if c.crbit(0) { pc = 0x80138c34; continue; }
                pc = 0x80138c9c;
            }
            0x80138c9c => {
                c.cmpu(0, c.r[24], 0x0);
                if c.crbit(2) { pc = 0x80138cb4; continue; }
                pc = 0x80138ca4;
            }
            0x80138ca4 => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x80138cac; f_800e24b0(c);
                pc = 0x80138cac;
            }
            0x80138cac => {
                c.r[3] = c.r[24] | c.r[24];
                c.lr = 0x80138cb4; f_800e209c(c);
                pc = 0x80138cb4;
            }
            0x80138cb4 => {
                { let mut ea = c.r[1].wrapping_add(0x10); for n in 24..32 { c.r[n] = c.lwz(ea); ea = ea.wrapping_add(4); } }
                c.r[3] = 0x1;
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x34));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x30);
                if true { return; }
                return;
            }
            _ => panic!("f_80138bbc: bad pc {:#x}", pc),
        }
    }
}

/// fn_80138CCC (0x80138ccc, 280 bytes)
#[inline(never)]
pub fn f_80138ccc(c: &mut Cpu) {
    if c.hooks {
        if c.stub_hit(0x80138ccc) { return; }
        if c.prof { crate::prof::enter(c, 0x80138ccc); f_80138ccc_b(c, 0x80138ccc); crate::prof::exit(c, 0x80138ccc); return; }
    }
    f_80138ccc_b(c, 0x80138ccc)
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_80138ccc_at(c: &mut Cpu, pc: u32) { f_80138ccc_b(c, pc) }
#[inline(always)]
fn f_80138ccc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x80138ccc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x80138ccc, pc); }
        match pc {
            0x80138ccc => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                if c.crbit(2) { pc = 0x80138db0; continue; }
                pc = 0x80138cf0;
            }
            0x80138cf0 => {
                c.r[30] = c.r[3] | c.r[3];
                c.r[28] = c.lhz(c.r[3].wrapping_add(0x38));
                c.cmpu(0, c.r[28], 0x20);
                if !c.crbit(1) { pc = 0x80138d0c; continue; }
                pc = 0x80138d00;
            }
            0x80138d00 => {
                c.r[0] = 0x20;
                c.r[28] = 0x20;
                c.sth(c.r[30].wrapping_add(0x38), c.r[0]);
                pc = 0x80138d0c;
            }
            0x80138d0c => {
                c.r[3] = c.lhz(c.r[30].wrapping_add(0x40));
                c.r[4] = c.lhz(c.r[30].wrapping_add(0x44));
                c.lr = 0x80138d18; f_800f9318(c);
                pc = 0x80138d18;
            }
            0x80138d18 => {
                c.stw(c.r[30].wrapping_add(0x8), c.r[3]);
                c.r[0] = c.lwz(c.r[30].wrapping_add(0x8));
                c.cmpu(0, c.r[0], 0x0);
                if c.crbit(2) { pc = 0x80138d90; continue; }
                pc = 0x80138d28;
            }
            0x80138d28 => {
                c.r[31] = c.r[28].wrapping_mul(0x5c);
                c.r[3] = c.r[31] | c.r[31];
                c.lr = 0x80138d34; f_800e3534(c);
                pc = 0x80138d34;
            }
            0x80138d34 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xffff;
                c.rc(c.r[0]);
                c.sth(c.r[30].wrapping_add(0x4), c.r[3]);
                if c.crbit(2) { pc = 0x80138db0; continue; }
                pc = 0x80138d40;
            }
            0x80138d40 => {
                c.lr = 0x80138d44; f_800e27b0(c);
                pc = 0x80138d44;
            }
            0x80138d44 => {
                c.stw(c.r[30], c.r[3]);
                c.r[5] = c.r[31] | c.r[31];
                c.r[31] = c.r[3] | c.r[3];
                c.r[4] = 0x0;
                c.r[3] = c.lwz(c.r[30]);
                c.lr = 0x80138d5c; f_8000540c(c);
                pc = 0x80138d5c;
            }
            0x80138d5c => {
                c.r[0] = 0x0;
                c.r[29] = 0x0;
                c.sth(c.r[30].wrapping_add(0x3c), c.r[0]);
                pc = 0x80138d80; continue;
                pc = 0x80138d6c;
            }
            0x80138d6c => {
                c.r[3] = c.r[30] | c.r[30];
                c.r[4] = c.r[31] | c.r[31];
                c.lr = 0x80138d78; f_80139378(c);
                pc = 0x80138d78;
            }
            0x80138d78 => {
                c.r[29] = c.r[29].wrapping_add(0x1);
                c.r[31] = c.r[31].wrapping_add(0x5c);
                pc = 0x80138d80;
            }
            0x80138d80 => {
                c.cmpu(0, c.r[29], c.r[28]);
                if c.crbit(0) { pc = 0x80138d6c; continue; }
                pc = 0x80138d88;
            }
            0x80138d88 => {
                c.r[3] = 0x1;
                pc = 0x80138dc4; continue;
                pc = 0x80138d90;
            }
            0x80138d90 => {
                c.r[3] = 0x80270000;
                c.r[4] = c.lhz(c.r[30].wrapping_add(0x40));
                c.r[5] = c.lhz(c.r[30].wrapping_add(0x44));
                c.r[3] = c.r[3].wrapping_add(0x2c30);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80138da8; stubs_gen::f_800dd970(c);
                pc = 0x80138da8;
            }
            0x80138da8 => {
                c.r[3] = 0x0;
                pc = 0x80138dc4; continue;
                pc = 0x80138db0;
            }
            0x80138db0 => {
                c.r[3] = 0x80270000;
                c.r[3] = c.r[3].wrapping_add(0x2c90);
                { let (a, b) = (c.crbit(6), c.crbit(6)); c.set_crbit(6, a ^ b); }
                c.lr = 0x80138dc0; stubs_gen::f_800dd970(c);
                pc = 0x80138dc0;
            }
            0x80138dc0 => {
                c.r[3] = 0x0;
                pc = 0x80138dc4;
            }
            0x80138dc4 => {
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
            _ => panic!("f_80138ccc: bad pc {:#x}", pc),
        }
    }
}

