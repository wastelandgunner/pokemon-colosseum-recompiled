#![allow(unused_mut, unused_variables, unreachable_code, unused_parens, clippy::all)]
use crate::ppc::*;
use super::*;
use super::stubs_gen;

/// wazaDataBiosGetRiskFlag (0x8011c570, 24 bytes)
#[inline(never)]
pub fn f_8011c570(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c570) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c570); p = true; }
    }
    f_8011c570_b(c, 0x8011c570);
    if p { crate::prof::exit(c, 0x8011c570); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c570_at(c: &mut Cpu, pc: u32) { f_8011c570_b(c, pc) }
#[inline(always)]
fn f_8011c570_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c570;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c570, pc); }
        match pc {
            0x8011c570 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c580; continue; }
                pc = 0x8011c578;
            }
            0x8011c578 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c580;
            }
            0x8011c580 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x13));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c570: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetTypeId (0x8011c588, 48 bytes)
#[inline(never)]
pub fn f_8011c588(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c588) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c588); p = true; }
    }
    f_8011c588_b(c, 0x8011c588);
    if p { crate::prof::exit(c, 0x8011c588); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c588_at(c: &mut Cpu, pc: u32) { f_8011c588_b(c, pc) }
#[inline(always)]
fn f_8011c588_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c588;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c588, pc); }
        match pc {
            0x8011c588 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c598; continue; }
                pc = 0x8011c590;
            }
            0x8011c590 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c598;
            }
            0x8011c598 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x8011c5ac; continue; }
                pc = 0x8011c5a4;
            }
            0x8011c5a4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c5ac;
            }
            0x8011c5ac => {
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x34));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c588: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetHidenFlag (0x8011c5b8, 16 bytes)
#[inline(never)]
pub fn f_8011c5b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c5b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c5b8); p = true; }
    }
    f_8011c5b8_b(c, 0x8011c5b8);
    if p { crate::prof::exit(c, 0x8011c5b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c5b8_at(c: &mut Cpu, pc: u32) { f_8011c5b8_b(c, pc) }
#[inline(always)]
fn f_8011c5b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c5b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c5b8, pc); }
        match pc {
            0x8011c5b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c5c0;
            }
            0x8011c5c0 => {
                c.stb(c.r[3].wrapping_add(0x12), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c5b8: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetHidenFlag (0x8011c5c8, 24 bytes)
#[inline(never)]
pub fn f_8011c5c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c5c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c5c8); p = true; }
    }
    f_8011c5c8_b(c, 0x8011c5c8);
    if p { crate::prof::exit(c, 0x8011c5c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c5c8_at(c: &mut Cpu, pc: u32) { f_8011c5c8_b(c, pc) }
#[inline(always)]
fn f_8011c5c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c5c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c5c8, pc); }
        match pc {
            0x8011c5c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c5d8; continue; }
                pc = 0x8011c5d0;
            }
            0x8011c5d0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c5d8;
            }
            0x8011c5d8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x12));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c5c8: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011C5E0 (0x8011c5e0, 16 bytes)
#[inline(never)]
pub fn f_8011c5e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c5e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c5e0); p = true; }
    }
    f_8011c5e0_b(c, 0x8011c5e0);
    if p { crate::prof::exit(c, 0x8011c5e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c5e0_at(c: &mut Cpu, pc: u32) { f_8011c5e0_b(c, pc) }
#[inline(always)]
fn f_8011c5e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c5e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c5e0, pc); }
        match pc {
            0x8011c5e0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c5e8;
            }
            0x8011c5e8 => {
                c.stb(c.r[3].wrapping_add(0x15), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c5e0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011C5F0 (0x8011c5f0, 16 bytes)
#[inline(never)]
pub fn f_8011c5f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c5f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c5f0); p = true; }
    }
    f_8011c5f0_b(c, 0x8011c5f0);
    if p { crate::prof::exit(c, 0x8011c5f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c5f0_at(c: &mut Cpu, pc: u32) { f_8011c5f0_b(c, pc) }
#[inline(always)]
fn f_8011c5f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c5f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c5f0, pc); }
        match pc {
            0x8011c5f0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c5f8;
            }
            0x8011c5f8 => {
                c.stb(c.r[3].wrapping_add(0x14), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c5f0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetDoc (0x8011c600, 16 bytes)
#[inline(never)]
pub fn f_8011c600(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c600) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c600); p = true; }
    }
    f_8011c600_b(c, 0x8011c600);
    if p { crate::prof::exit(c, 0x8011c600); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c600_at(c: &mut Cpu, pc: u32) { f_8011c600_b(c, pc) }
#[inline(always)]
fn f_8011c600_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c600;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c600, pc); }
        match pc {
            0x8011c600 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c608;
            }
            0x8011c608 => {
                c.stw(c.r[3].wrapping_add(0x2c), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c600: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetWazawzxdataId (0x8011c610, 16 bytes)
#[inline(never)]
pub fn f_8011c610(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c610) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c610); p = true; }
    }
    f_8011c610_b(c, 0x8011c610);
    if p { crate::prof::exit(c, 0x8011c610); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c610_at(c: &mut Cpu, pc: u32) { f_8011c610_b(c, pc) }
#[inline(always)]
fn f_8011c610_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c610;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c610, pc); }
        match pc {
            0x8011c610 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c618;
            }
            0x8011c618 => {
                c.stw(c.r[3].wrapping_add(0x30), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c610: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetPressure (0x8011c620, 16 bytes)
#[inline(never)]
pub fn f_8011c620(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c620) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c620); p = true; }
    }
    f_8011c620_b(c, 0x8011c620);
    if p { crate::prof::exit(c, 0x8011c620); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c620_at(c: &mut Cpu, pc: u32) { f_8011c620_b(c, pc) }
#[inline(always)]
fn f_8011c620_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c620;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c620, pc); }
        match pc {
            0x8011c620 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c628;
            }
            0x8011c628 => {
                c.stb(c.r[3].wrapping_add(0x11), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c620: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetBouon (0x8011c630, 16 bytes)
#[inline(never)]
pub fn f_8011c630(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c630) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c630); p = true; }
    }
    f_8011c630_b(c, 0x8011c630);
    if p { crate::prof::exit(c, 0x8011c630); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c630_at(c: &mut Cpu, pc: u32) { f_8011c630_b(c, pc) }
#[inline(always)]
fn f_8011c630_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c630;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c630, pc); }
        match pc {
            0x8011c630 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c638;
            }
            0x8011c638 => {
                c.stb(c.r[3].wrapping_add(0x10), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c630: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetNegoto (0x8011c640, 16 bytes)
#[inline(never)]
pub fn f_8011c640(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c640) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c640); p = true; }
    }
    f_8011c640_b(c, 0x8011c640);
    if p { crate::prof::exit(c, 0x8011c640); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c640_at(c: &mut Cpu, pc: u32) { f_8011c640_b(c, pc) }
#[inline(always)]
fn f_8011c640_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c640;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c640, pc); }
        match pc {
            0x8011c640 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c648;
            }
            0x8011c648 => {
                c.stb(c.r[3].wrapping_add(0xf), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c640: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetNekonote (0x8011c650, 16 bytes)
#[inline(never)]
pub fn f_8011c650(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c650) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c650); p = true; }
    }
    f_8011c650_b(c, 0x8011c650);
    if p { crate::prof::exit(c, 0x8011c650); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c650_at(c: &mut Cpu, pc: u32) { f_8011c650_b(c, pc) }
#[inline(always)]
fn f_8011c650_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c650;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c650, pc); }
        match pc {
            0x8011c650 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c658;
            }
            0x8011c658 => {
                c.stb(c.r[3].wrapping_add(0xe), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c650: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetMonomane (0x8011c660, 16 bytes)
#[inline(never)]
pub fn f_8011c660(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c660) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c660); p = true; }
    }
    f_8011c660_b(c, 0x8011c660);
    if p { crate::prof::exit(c, 0x8011c660); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c660_at(c: &mut Cpu, pc: u32) { f_8011c660_b(c, pc) }
#[inline(always)]
fn f_8011c660_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c660;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c660, pc); }
        match pc {
            0x8011c660 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c668;
            }
            0x8011c668 => {
                c.stb(c.r[3].wrapping_add(0xd), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c660: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetYubiwohuru (0x8011c670, 16 bytes)
#[inline(never)]
pub fn f_8011c670(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c670) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c670); p = true; }
    }
    f_8011c670_b(c, 0x8011c670);
    if p { crate::prof::exit(c, 0x8011c670); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c670_at(c: &mut Cpu, pc: u32) { f_8011c670_b(c, pc) }
#[inline(always)]
fn f_8011c670_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c670;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c670, pc); }
        match pc {
            0x8011c670 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c678;
            }
            0x8011c678 => {
                c.stb(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c670: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetOujanosirusi (0x8011c680, 16 bytes)
#[inline(never)]
pub fn f_8011c680(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c680) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c680); p = true; }
    }
    f_8011c680_b(c, 0x8011c680);
    if p { crate::prof::exit(c, 0x8011c680); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c680_at(c: &mut Cpu, pc: u32) { f_8011c680_b(c, pc) }
#[inline(always)]
fn f_8011c680_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c680;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c680, pc); }
        match pc {
            0x8011c680 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c688;
            }
            0x8011c688 => {
                c.stb(c.r[3].wrapping_add(0xb), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c680: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetOumugaesi (0x8011c690, 16 bytes)
#[inline(never)]
pub fn f_8011c690(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c690) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c690); p = true; }
    }
    f_8011c690_b(c, 0x8011c690);
    if p { crate::prof::exit(c, 0x8011c690); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c690_at(c: &mut Cpu, pc: u32) { f_8011c690_b(c, pc) }
#[inline(always)]
fn f_8011c690_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c690;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c690, pc); }
        match pc {
            0x8011c690 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c698;
            }
            0x8011c698 => {
                c.stb(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c690: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetYokodori (0x8011c6a0, 16 bytes)
#[inline(never)]
pub fn f_8011c6a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c6a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c6a0); p = true; }
    }
    f_8011c6a0_b(c, 0x8011c6a0);
    if p { crate::prof::exit(c, 0x8011c6a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c6a0_at(c: &mut Cpu, pc: u32) { f_8011c6a0_b(c, pc) }
#[inline(always)]
fn f_8011c6a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c6a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c6a0, pc); }
        match pc {
            0x8011c6a0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c6a8;
            }
            0x8011c6a8 => {
                c.stb(c.r[3].wrapping_add(0x9), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c6a0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetMajikku (0x8011c6b0, 16 bytes)
#[inline(never)]
pub fn f_8011c6b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c6b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c6b0); p = true; }
    }
    f_8011c6b0_b(c, 0x8011c6b0);
    if p { crate::prof::exit(c, 0x8011c6b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c6b0_at(c: &mut Cpu, pc: u32) { f_8011c6b0_b(c, pc) }
#[inline(always)]
fn f_8011c6b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c6b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c6b0, pc); }
        match pc {
            0x8011c6b0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c6b8;
            }
            0x8011c6b8 => {
                c.stb(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c6b0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetMamoru (0x8011c6c0, 16 bytes)
#[inline(never)]
pub fn f_8011c6c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c6c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c6c0); p = true; }
    }
    f_8011c6c0_b(c, 0x8011c6c0);
    if p { crate::prof::exit(c, 0x8011c6c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c6c0_at(c: &mut Cpu, pc: u32) { f_8011c6c0_b(c, pc) }
#[inline(always)]
fn f_8011c6c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c6c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c6c0, pc); }
        match pc {
            0x8011c6c0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c6c8;
            }
            0x8011c6c8 => {
                c.stb(c.r[3].wrapping_add(0x7), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c6c0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetDageki (0x8011c6d0, 16 bytes)
#[inline(never)]
pub fn f_8011c6d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c6d0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c6d0); p = true; }
    }
    f_8011c6d0_b(c, 0x8011c6d0);
    if p { crate::prof::exit(c, 0x8011c6d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c6d0_at(c: &mut Cpu, pc: u32) { f_8011c6d0_b(c, pc) }
#[inline(always)]
fn f_8011c6d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c6d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c6d0, pc); }
        match pc {
            0x8011c6d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c6d8;
            }
            0x8011c6d8 => {
                c.stb(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c6d0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetAddFightKoukaAvg (0x8011c6e0, 16 bytes)
#[inline(never)]
pub fn f_8011c6e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c6e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c6e0); p = true; }
    }
    f_8011c6e0_b(c, 0x8011c6e0);
    if p { crate::prof::exit(c, 0x8011c6e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c6e0_at(c: &mut Cpu, pc: u32) { f_8011c6e0_b(c, pc) }
#[inline(always)]
fn f_8011c6e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c6e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c6e0, pc); }
        match pc {
            0x8011c6e0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c6e8;
            }
            0x8011c6e8 => {
                c.stb(c.r[3].wrapping_add(0x5), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c6e0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetFightAttackMsgId (0x8011c6f0, 16 bytes)
#[inline(never)]
pub fn f_8011c6f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c6f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c6f0); p = true; }
    }
    f_8011c6f0_b(c, 0x8011c6f0);
    if p { crate::prof::exit(c, 0x8011c6f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c6f0_at(c: &mut Cpu, pc: u32) { f_8011c6f0_b(c, pc) }
#[inline(always)]
fn f_8011c6f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c6f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c6f0, pc); }
        match pc {
            0x8011c6f0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c6f8;
            }
            0x8011c6f8 => {
                c.stw(c.r[3].wrapping_add(0x28), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c6f0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetSeqId (0x8011c700, 16 bytes)
#[inline(never)]
pub fn f_8011c700(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c700) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c700); p = true; }
    }
    f_8011c700_b(c, 0x8011c700);
    if p { crate::prof::exit(c, 0x8011c700); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c700_at(c: &mut Cpu, pc: u32) { f_8011c700_b(c, pc) }
#[inline(always)]
fn f_8011c700_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c700;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c700, pc); }
        match pc {
            0x8011c700 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c708;
            }
            0x8011c708 => {
                c.sth(c.r[3].wrapping_add(0x1a), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c700: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetFightKoukaDataId (0x8011c710, 16 bytes)
#[inline(never)]
pub fn f_8011c710(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c710) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c710); p = true; }
    }
    f_8011c710_b(c, 0x8011c710);
    if p { crate::prof::exit(c, 0x8011c710); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c710_at(c: &mut Cpu, pc: u32) { f_8011c710_b(c, pc) }
#[inline(always)]
fn f_8011c710_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c710;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c710, pc); }
        match pc {
            0x8011c710 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c718;
            }
            0x8011c718 => {
                c.sth(c.r[3].wrapping_add(0x18), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c710: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetIryoku (0x8011c720, 16 bytes)
#[inline(never)]
pub fn f_8011c720(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c720) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c720); p = true; }
    }
    f_8011c720_b(c, 0x8011c720);
    if p { crate::prof::exit(c, 0x8011c720); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c720_at(c: &mut Cpu, pc: u32) { f_8011c720_b(c, pc) }
#[inline(always)]
fn f_8011c720_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c720;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c720, pc); }
        match pc {
            0x8011c720 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c728;
            }
            0x8011c728 => {
                c.sth(c.r[3].wrapping_add(0x16), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c720: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetAvg (0x8011c730, 16 bytes)
#[inline(never)]
pub fn f_8011c730(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c730) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c730); p = true; }
    }
    f_8011c730_b(c, 0x8011c730);
    if p { crate::prof::exit(c, 0x8011c730); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c730_at(c: &mut Cpu, pc: u32) { f_8011c730_b(c, pc) }
#[inline(always)]
fn f_8011c730_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c730;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c730, pc); }
        match pc {
            0x8011c730 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c738;
            }
            0x8011c738 => {
                c.stb(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c730: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetRangeId (0x8011c740, 16 bytes)
#[inline(never)]
pub fn f_8011c740(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c740) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c740); p = true; }
    }
    f_8011c740_b(c, 0x8011c740);
    if p { crate::prof::exit(c, 0x8011c740); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c740_at(c: &mut Cpu, pc: u32) { f_8011c740_b(c, pc) }
#[inline(always)]
fn f_8011c740_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c740;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c740, pc); }
        match pc {
            0x8011c740 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c748;
            }
            0x8011c748 => {
                c.stb(c.r[3].wrapping_add(0x3), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c740: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetPri (0x8011c750, 16 bytes)
#[inline(never)]
pub fn f_8011c750(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c750) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c750); p = true; }
    }
    f_8011c750_b(c, 0x8011c750);
    if p { crate::prof::exit(c, 0x8011c750); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c750_at(c: &mut Cpu, pc: u32) { f_8011c750_b(c, pc) }
#[inline(always)]
fn f_8011c750_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c750;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c750, pc); }
        match pc {
            0x8011c750 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c758;
            }
            0x8011c758 => {
                c.stb(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c750: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetZokuseiDataId (0x8011c760, 16 bytes)
#[inline(never)]
pub fn f_8011c760(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c760) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c760); p = true; }
    }
    f_8011c760_b(c, 0x8011c760);
    if p { crate::prof::exit(c, 0x8011c760); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c760_at(c: &mut Cpu, pc: u32) { f_8011c760_b(c, pc) }
#[inline(always)]
fn f_8011c760_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c760;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c760, pc); }
        match pc {
            0x8011c760 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c768;
            }
            0x8011c768 => {
                c.stb(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c760: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetPp (0x8011c770, 16 bytes)
#[inline(never)]
pub fn f_8011c770(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c770) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c770); p = true; }
    }
    f_8011c770_b(c, 0x8011c770);
    if p { crate::prof::exit(c, 0x8011c770); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c770_at(c: &mut Cpu, pc: u32) { f_8011c770_b(c, pc) }
#[inline(always)]
fn f_8011c770_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c770;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c770, pc); }
        match pc {
            0x8011c770 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c778;
            }
            0x8011c778 => {
                c.stb(c.r[3].wrapping_add(0x1), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c770: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosSetName (0x8011c780, 16 bytes)
#[inline(never)]
pub fn f_8011c780(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c780) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c780); p = true; }
    }
    f_8011c780_b(c, 0x8011c780);
    if p { crate::prof::exit(c, 0x8011c780); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c780_at(c: &mut Cpu, pc: u32) { f_8011c780_b(c, pc) }
#[inline(always)]
fn f_8011c780_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c780;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c780, pc); }
        match pc {
            0x8011c780 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011c788;
            }
            0x8011c788 => {
                c.stw(c.r[3].wrapping_add(0x20), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011c780: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011C790 (0x8011c790, 24 bytes)
#[inline(never)]
pub fn f_8011c790(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c790) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c790); p = true; }
    }
    f_8011c790_b(c, 0x8011c790);
    if p { crate::prof::exit(c, 0x8011c790); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c790_at(c: &mut Cpu, pc: u32) { f_8011c790_b(c, pc) }
#[inline(always)]
fn f_8011c790_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c790;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c790, pc); }
        match pc {
            0x8011c790 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c7a0; continue; }
                pc = 0x8011c798;
            }
            0x8011c798 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c7a0;
            }
            0x8011c7a0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x15));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c790: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011C7A8 (0x8011c7a8, 24 bytes)
#[inline(never)]
pub fn f_8011c7a8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c7a8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c7a8); p = true; }
    }
    f_8011c7a8_b(c, 0x8011c7a8);
    if p { crate::prof::exit(c, 0x8011c7a8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c7a8_at(c: &mut Cpu, pc: u32) { f_8011c7a8_b(c, pc) }
#[inline(always)]
fn f_8011c7a8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c7a8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c7a8, pc); }
        match pc {
            0x8011c7a8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c7b8; continue; }
                pc = 0x8011c7b0;
            }
            0x8011c7b0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c7b8;
            }
            0x8011c7b8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x14));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c7a8: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetDoc (0x8011c7c0, 24 bytes)
#[inline(never)]
pub fn f_8011c7c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c7c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c7c0); p = true; }
    }
    f_8011c7c0_b(c, 0x8011c7c0);
    if p { crate::prof::exit(c, 0x8011c7c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c7c0_at(c: &mut Cpu, pc: u32) { f_8011c7c0_b(c, pc) }
#[inline(always)]
fn f_8011c7c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c7c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c7c0, pc); }
        match pc {
            0x8011c7c0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c7d0; continue; }
                pc = 0x8011c7c8;
            }
            0x8011c7c8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c7d0;
            }
            0x8011c7d0 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x2c));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c7c0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetWazawzxdataId (0x8011c7d8, 24 bytes)
#[inline(never)]
pub fn f_8011c7d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c7d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c7d8); p = true; }
    }
    f_8011c7d8_b(c, 0x8011c7d8);
    if p { crate::prof::exit(c, 0x8011c7d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c7d8_at(c: &mut Cpu, pc: u32) { f_8011c7d8_b(c, pc) }
#[inline(always)]
fn f_8011c7d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c7d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c7d8, pc); }
        match pc {
            0x8011c7d8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c7e8; continue; }
                pc = 0x8011c7e0;
            }
            0x8011c7e0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c7e8;
            }
            0x8011c7e8 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x30));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c7d8: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetPressure (0x8011c7f0, 24 bytes)
#[inline(never)]
pub fn f_8011c7f0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c7f0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c7f0); p = true; }
    }
    f_8011c7f0_b(c, 0x8011c7f0);
    if p { crate::prof::exit(c, 0x8011c7f0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c7f0_at(c: &mut Cpu, pc: u32) { f_8011c7f0_b(c, pc) }
#[inline(always)]
fn f_8011c7f0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c7f0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c7f0, pc); }
        match pc {
            0x8011c7f0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c800; continue; }
                pc = 0x8011c7f8;
            }
            0x8011c7f8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c800;
            }
            0x8011c800 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x11));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c7f0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetBouon (0x8011c808, 24 bytes)
#[inline(never)]
pub fn f_8011c808(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c808) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c808); p = true; }
    }
    f_8011c808_b(c, 0x8011c808);
    if p { crate::prof::exit(c, 0x8011c808); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c808_at(c: &mut Cpu, pc: u32) { f_8011c808_b(c, pc) }
#[inline(always)]
fn f_8011c808_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c808;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c808, pc); }
        match pc {
            0x8011c808 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c818; continue; }
                pc = 0x8011c810;
            }
            0x8011c810 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c818;
            }
            0x8011c818 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x10));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c808: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetNegoto (0x8011c820, 24 bytes)
#[inline(never)]
pub fn f_8011c820(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c820) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c820); p = true; }
    }
    f_8011c820_b(c, 0x8011c820);
    if p { crate::prof::exit(c, 0x8011c820); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c820_at(c: &mut Cpu, pc: u32) { f_8011c820_b(c, pc) }
#[inline(always)]
fn f_8011c820_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c820;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c820, pc); }
        match pc {
            0x8011c820 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c830; continue; }
                pc = 0x8011c828;
            }
            0x8011c828 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c830;
            }
            0x8011c830 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xf));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c820: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetNekonote (0x8011c838, 24 bytes)
#[inline(never)]
pub fn f_8011c838(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c838) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c838); p = true; }
    }
    f_8011c838_b(c, 0x8011c838);
    if p { crate::prof::exit(c, 0x8011c838); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c838_at(c: &mut Cpu, pc: u32) { f_8011c838_b(c, pc) }
#[inline(always)]
fn f_8011c838_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c838;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c838, pc); }
        match pc {
            0x8011c838 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c848; continue; }
                pc = 0x8011c840;
            }
            0x8011c840 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c848;
            }
            0x8011c848 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xe));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c838: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetMonomane (0x8011c850, 24 bytes)
#[inline(never)]
pub fn f_8011c850(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c850) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c850); p = true; }
    }
    f_8011c850_b(c, 0x8011c850);
    if p { crate::prof::exit(c, 0x8011c850); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c850_at(c: &mut Cpu, pc: u32) { f_8011c850_b(c, pc) }
#[inline(always)]
fn f_8011c850_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c850;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c850, pc); }
        match pc {
            0x8011c850 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c860; continue; }
                pc = 0x8011c858;
            }
            0x8011c858 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c860;
            }
            0x8011c860 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xd));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c850: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetYubiwohuru (0x8011c868, 24 bytes)
#[inline(never)]
pub fn f_8011c868(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c868) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c868); p = true; }
    }
    f_8011c868_b(c, 0x8011c868);
    if p { crate::prof::exit(c, 0x8011c868); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c868_at(c: &mut Cpu, pc: u32) { f_8011c868_b(c, pc) }
#[inline(always)]
fn f_8011c868_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c868;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c868, pc); }
        match pc {
            0x8011c868 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c878; continue; }
                pc = 0x8011c870;
            }
            0x8011c870 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c878;
            }
            0x8011c878 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xc));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c868: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetOujanosirusi (0x8011c880, 24 bytes)
#[inline(never)]
pub fn f_8011c880(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c880) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c880); p = true; }
    }
    f_8011c880_b(c, 0x8011c880);
    if p { crate::prof::exit(c, 0x8011c880); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c880_at(c: &mut Cpu, pc: u32) { f_8011c880_b(c, pc) }
#[inline(always)]
fn f_8011c880_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c880;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c880, pc); }
        match pc {
            0x8011c880 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c890; continue; }
                pc = 0x8011c888;
            }
            0x8011c888 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c890;
            }
            0x8011c890 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xb));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c880: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetOumugaesi (0x8011c898, 24 bytes)
#[inline(never)]
pub fn f_8011c898(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c898) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c898); p = true; }
    }
    f_8011c898_b(c, 0x8011c898);
    if p { crate::prof::exit(c, 0x8011c898); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c898_at(c: &mut Cpu, pc: u32) { f_8011c898_b(c, pc) }
#[inline(always)]
fn f_8011c898_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c898;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c898, pc); }
        match pc {
            0x8011c898 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c8a8; continue; }
                pc = 0x8011c8a0;
            }
            0x8011c8a0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c8a8;
            }
            0x8011c8a8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xa));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c898: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetYokodori (0x8011c8b0, 24 bytes)
#[inline(never)]
pub fn f_8011c8b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c8b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c8b0); p = true; }
    }
    f_8011c8b0_b(c, 0x8011c8b0);
    if p { crate::prof::exit(c, 0x8011c8b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c8b0_at(c: &mut Cpu, pc: u32) { f_8011c8b0_b(c, pc) }
#[inline(always)]
fn f_8011c8b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c8b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c8b0, pc); }
        match pc {
            0x8011c8b0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c8c0; continue; }
                pc = 0x8011c8b8;
            }
            0x8011c8b8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c8c0;
            }
            0x8011c8c0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x9));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c8b0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetMajikku (0x8011c8c8, 24 bytes)
#[inline(never)]
pub fn f_8011c8c8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c8c8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c8c8); p = true; }
    }
    f_8011c8c8_b(c, 0x8011c8c8);
    if p { crate::prof::exit(c, 0x8011c8c8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c8c8_at(c: &mut Cpu, pc: u32) { f_8011c8c8_b(c, pc) }
#[inline(always)]
fn f_8011c8c8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c8c8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c8c8, pc); }
        match pc {
            0x8011c8c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c8d8; continue; }
                pc = 0x8011c8d0;
            }
            0x8011c8d0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c8d8;
            }
            0x8011c8d8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c8c8: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetMamoru (0x8011c8e0, 24 bytes)
#[inline(never)]
pub fn f_8011c8e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c8e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c8e0); p = true; }
    }
    f_8011c8e0_b(c, 0x8011c8e0);
    if p { crate::prof::exit(c, 0x8011c8e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c8e0_at(c: &mut Cpu, pc: u32) { f_8011c8e0_b(c, pc) }
#[inline(always)]
fn f_8011c8e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c8e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c8e0, pc); }
        match pc {
            0x8011c8e0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c8f0; continue; }
                pc = 0x8011c8e8;
            }
            0x8011c8e8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c8f0;
            }
            0x8011c8f0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x7));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c8e0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetDageki (0x8011c8f8, 24 bytes)
#[inline(never)]
pub fn f_8011c8f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c8f8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c8f8); p = true; }
    }
    f_8011c8f8_b(c, 0x8011c8f8);
    if p { crate::prof::exit(c, 0x8011c8f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c8f8_at(c: &mut Cpu, pc: u32) { f_8011c8f8_b(c, pc) }
#[inline(always)]
fn f_8011c8f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c8f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c8f8, pc); }
        match pc {
            0x8011c8f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c908; continue; }
                pc = 0x8011c900;
            }
            0x8011c900 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c908;
            }
            0x8011c908 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x6));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c8f8: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetAddFightKoukaAvg (0x8011c910, 24 bytes)
#[inline(never)]
pub fn f_8011c910(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c910) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c910); p = true; }
    }
    f_8011c910_b(c, 0x8011c910);
    if p { crate::prof::exit(c, 0x8011c910); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c910_at(c: &mut Cpu, pc: u32) { f_8011c910_b(c, pc) }
#[inline(always)]
fn f_8011c910_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c910;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c910, pc); }
        match pc {
            0x8011c910 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c920; continue; }
                pc = 0x8011c918;
            }
            0x8011c918 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c920;
            }
            0x8011c920 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x5));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c910: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetFightAttackMsgId (0x8011c928, 24 bytes)
#[inline(never)]
pub fn f_8011c928(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c928) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c928); p = true; }
    }
    f_8011c928_b(c, 0x8011c928);
    if p { crate::prof::exit(c, 0x8011c928); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c928_at(c: &mut Cpu, pc: u32) { f_8011c928_b(c, pc) }
#[inline(always)]
fn f_8011c928_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c928;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c928, pc); }
        match pc {
            0x8011c928 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c938; continue; }
                pc = 0x8011c930;
            }
            0x8011c930 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c938;
            }
            0x8011c938 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x28));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c928: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetFightAttackTunagiMsgId (0x8011c940, 24 bytes)
#[inline(never)]
pub fn f_8011c940(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c940) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c940); p = true; }
    }
    f_8011c940_b(c, 0x8011c940);
    if p { crate::prof::exit(c, 0x8011c940); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c940_at(c: &mut Cpu, pc: u32) { f_8011c940_b(c, pc) }
#[inline(always)]
fn f_8011c940_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c940;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c940, pc); }
        match pc {
            0x8011c940 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c950; continue; }
                pc = 0x8011c948;
            }
            0x8011c948 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c950;
            }
            0x8011c950 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x24));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c940: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetSeqId (0x8011c958, 24 bytes)
#[inline(never)]
pub fn f_8011c958(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c958) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c958); p = true; }
    }
    f_8011c958_b(c, 0x8011c958);
    if p { crate::prof::exit(c, 0x8011c958); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c958_at(c: &mut Cpu, pc: u32) { f_8011c958_b(c, pc) }
#[inline(always)]
fn f_8011c958_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c958;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c958, pc); }
        match pc {
            0x8011c958 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c968; continue; }
                pc = 0x8011c960;
            }
            0x8011c960 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c968;
            }
            0x8011c968 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x1a));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c958: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetFightKoukaDataId (0x8011c970, 24 bytes)
#[inline(never)]
pub fn f_8011c970(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c970) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c970); p = true; }
    }
    f_8011c970_b(c, 0x8011c970);
    if p { crate::prof::exit(c, 0x8011c970); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c970_at(c: &mut Cpu, pc: u32) { f_8011c970_b(c, pc) }
#[inline(always)]
fn f_8011c970_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c970;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c970, pc); }
        match pc {
            0x8011c970 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c980; continue; }
                pc = 0x8011c978;
            }
            0x8011c978 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c980;
            }
            0x8011c980 => {
                c.r[3] = c.lhz(c.r[3].wrapping_add(0x18));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c970: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetIryoku (0x8011c988, 24 bytes)
#[inline(never)]
pub fn f_8011c988(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c988) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c988); p = true; }
    }
    f_8011c988_b(c, 0x8011c988);
    if p { crate::prof::exit(c, 0x8011c988); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c988_at(c: &mut Cpu, pc: u32) { f_8011c988_b(c, pc) }
#[inline(always)]
fn f_8011c988_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c988;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c988, pc); }
        match pc {
            0x8011c988 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c998; continue; }
                pc = 0x8011c990;
            }
            0x8011c990 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c998;
            }
            0x8011c998 => {
                c.r[3] = c.lha(c.r[3].wrapping_add(0x16));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c988: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetAvg (0x8011c9a0, 24 bytes)
#[inline(never)]
pub fn f_8011c9a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c9a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c9a0); p = true; }
    }
    f_8011c9a0_b(c, 0x8011c9a0);
    if p { crate::prof::exit(c, 0x8011c9a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c9a0_at(c: &mut Cpu, pc: u32) { f_8011c9a0_b(c, pc) }
#[inline(always)]
fn f_8011c9a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c9a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c9a0, pc); }
        match pc {
            0x8011c9a0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c9b0; continue; }
                pc = 0x8011c9a8;
            }
            0x8011c9a8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c9b0;
            }
            0x8011c9b0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c9a0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetRangeId (0x8011c9b8, 24 bytes)
#[inline(never)]
pub fn f_8011c9b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c9b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c9b8); p = true; }
    }
    f_8011c9b8_b(c, 0x8011c9b8);
    if p { crate::prof::exit(c, 0x8011c9b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c9b8_at(c: &mut Cpu, pc: u32) { f_8011c9b8_b(c, pc) }
#[inline(always)]
fn f_8011c9b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c9b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c9b8, pc); }
        match pc {
            0x8011c9b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c9c8; continue; }
                pc = 0x8011c9c0;
            }
            0x8011c9c0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c9c8;
            }
            0x8011c9c8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x3));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c9b8: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetPri (0x8011c9d0, 28 bytes)
#[inline(never)]
pub fn f_8011c9d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c9d0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c9d0); p = true; }
    }
    f_8011c9d0_b(c, 0x8011c9d0);
    if p { crate::prof::exit(c, 0x8011c9d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c9d0_at(c: &mut Cpu, pc: u32) { f_8011c9d0_b(c, pc) }
#[inline(always)]
fn f_8011c9d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c9d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c9d0, pc); }
        match pc {
            0x8011c9d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c9e0; continue; }
                pc = 0x8011c9d8;
            }
            0x8011c9d8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c9e0;
            }
            0x8011c9e0 => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_8011c9d0: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetZokuseiDataId (0x8011c9ec, 24 bytes)
#[inline(never)]
pub fn f_8011c9ec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011c9ec) { return; }
        if c.prof { crate::prof::enter(c, 0x8011c9ec); p = true; }
    }
    f_8011c9ec_b(c, 0x8011c9ec);
    if p { crate::prof::exit(c, 0x8011c9ec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011c9ec_at(c: &mut Cpu, pc: u32) { f_8011c9ec_b(c, pc) }
#[inline(always)]
fn f_8011c9ec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011c9ec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011c9ec, pc); }
        match pc {
            0x8011c9ec => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011c9fc; continue; }
                pc = 0x8011c9f4;
            }
            0x8011c9f4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011c9fc;
            }
            0x8011c9fc => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x2));
                if true { return; }
                return;
            }
            _ => panic!("f_8011c9ec: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetPp (0x8011ca04, 24 bytes)
#[inline(never)]
pub fn f_8011ca04(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ca04) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ca04); p = true; }
    }
    f_8011ca04_b(c, 0x8011ca04);
    if p { crate::prof::exit(c, 0x8011ca04); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ca04_at(c: &mut Cpu, pc: u32) { f_8011ca04_b(c, pc) }
#[inline(always)]
fn f_8011ca04_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ca04;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ca04, pc); }
        match pc {
            0x8011ca04 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ca14; continue; }
                pc = 0x8011ca0c;
            }
            0x8011ca0c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ca14;
            }
            0x8011ca14 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_8011ca04: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetName (0x8011ca1c, 24 bytes)
#[inline(never)]
pub fn f_8011ca1c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ca1c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ca1c); p = true; }
    }
    f_8011ca1c_b(c, 0x8011ca1c);
    if p { crate::prof::exit(c, 0x8011ca1c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ca1c_at(c: &mut Cpu, pc: u32) { f_8011ca1c_b(c, pc) }
#[inline(always)]
fn f_8011ca1c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ca1c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ca1c, pc); }
        match pc {
            0x8011ca1c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ca2c; continue; }
                pc = 0x8011ca24;
            }
            0x8011ca24 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ca2c;
            }
            0x8011ca2c => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x20));
                if true { return; }
                return;
            }
            _ => panic!("f_8011ca1c: bad pc {:#x}", pc),
        }
    }
}

/// wazaDataBiosGetPtr (0x8011ca34, 44 bytes)
#[inline(never)]
pub fn f_8011ca34(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ca34) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ca34); p = true; }
    }
    f_8011ca34_b(c, 0x8011ca34);
    if p { crate::prof::exit(c, 0x8011ca34); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ca34_at(c: &mut Cpu, pc: u32) { f_8011ca34_b(c, pc) }
#[inline(always)]
fn f_8011ca34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ca34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ca34, pc); }
        match pc {
            0x8011ca34 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff85d8));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011ca50; continue; }
                pc = 0x8011ca48;
            }
            0x8011ca48 => {
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff85dc));
                if true { return; }
                pc = 0x8011ca50;
            }
            0x8011ca50 => {
                c.r[0] = c.r[3].wrapping_mul(0x38);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff85dc));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011ca34: bad pc {:#x}", pc),
        }
    }
}

/// pokemonNakigoeDataBiosGetDataAddress (0x8011ca60, 60 bytes)
#[inline(never)]
pub fn f_8011ca60(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ca60) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ca60); p = true; }
    }
    f_8011ca60_b(c, 0x8011ca60);
    if p { crate::prof::exit(c, 0x8011ca60); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ca60_at(c: &mut Cpu, pc: u32) { f_8011ca60_b(c, pc) }
#[inline(always)]
fn f_8011ca60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ca60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ca60, pc); }
        match pc {
            0x8011ca60 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ca70; continue; }
                pc = 0x8011ca68;
            }
            0x8011ca68 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ca70;
            }
            0x8011ca70 => {
                c.r[4] = c.lhz(c.r[3].wrapping_add(0x10));
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8358));
                c.cmpu(0, c.r[4], c.r[0]);
                if c.crbit(0) { pc = 0x8011ca88; continue; }
                pc = 0x8011ca80;
            }
            0x8011ca80 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ca88;
            }
            0x8011ca88 => {
                c.r[3] = 0x80360000;
                c.r[0] = c.r[4].rotate_left(2) & 0x3fffc;
                c.r[3] = c.r[3].wrapping_add(0xfffff9a8);
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                if true { return; }
                return;
            }
            _ => panic!("f_8011ca60: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDpFilterDataBiosGetValue (0x8011ca9c, 28 bytes)
#[inline(never)]
pub fn f_8011ca9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ca9c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ca9c); p = true; }
    }
    f_8011ca9c_b(c, 0x8011ca9c);
    if p { crate::prof::exit(c, 0x8011ca9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ca9c_at(c: &mut Cpu, pc: u32) { f_8011ca9c_b(c, pc) }
#[inline(always)]
fn f_8011ca9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ca9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ca9c, pc); }
        match pc {
            0x8011ca9c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011caac; continue; }
                pc = 0x8011caa4;
            }
            0x8011caa4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011caac;
            }
            0x8011caac => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_8011ca9c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDpFilterDataBiosGetPtr (0x8011cab8, 40 bytes)
#[inline(never)]
pub fn f_8011cab8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cab8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cab8); p = true; }
    }
    f_8011cab8_b(c, 0x8011cab8);
    if p { crate::prof::exit(c, 0x8011cab8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cab8_at(c: &mut Cpu, pc: u32) { f_8011cab8_b(c, pc) }
#[inline(always)]
fn f_8011cab8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cab8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cab8, pc); }
        match pc {
            0x8011cab8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8648));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011cad4; continue; }
                pc = 0x8011cacc;
            }
            0x8011cacc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cad4;
            }
            0x8011cad4 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff864c));
                c.r[3] = c.r[0].wrapping_add(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cab8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonFriendFilterDataBiosGetValue (0x8011cae0, 48 bytes)
#[inline(never)]
pub fn f_8011cae0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cae0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cae0); p = true; }
    }
    f_8011cae0_b(c, 0x8011cae0);
    if p { crate::prof::exit(c, 0x8011cae0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cae0_at(c: &mut Cpu, pc: u32) { f_8011cae0_b(c, pc) }
#[inline(always)]
fn f_8011cae0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cae0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cae0, pc); }
        match pc {
            0x8011cae0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011caf0; continue; }
                pc = 0x8011cae8;
            }
            0x8011cae8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011caf0;
            }
            0x8011caf0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x3);
                if c.crbit(0) { pc = 0x8011cb04; continue; }
                pc = 0x8011cafc;
            }
            0x8011cafc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cb04;
            }
            0x8011cb04 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(c.r[0]));
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_8011cae0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonFriendFilterDataBiosGetPtr (0x8011cb10, 44 bytes)
#[inline(never)]
pub fn f_8011cb10(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cb10) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cb10); p = true; }
    }
    f_8011cb10_b(c, 0x8011cb10);
    if p { crate::prof::exit(c, 0x8011cb10); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cb10_at(c: &mut Cpu, pc: u32) { f_8011cb10_b(c, pc) }
#[inline(always)]
fn f_8011cb10_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cb10;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cb10, pc); }
        match pc {
            0x8011cb10 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8350));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011cb28; continue; }
                pc = 0x8011cb20;
            }
            0x8011cb20 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cb28;
            }
            0x8011cb28 => {
                c.r[4] = c.r[3].wrapping_mul(0x3);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xfffff988);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cb10: bad pc {:#x}", pc),
        }
    }
}

/// pokemonTokuseiDataBiosGetDoc (0x8011cb3c, 24 bytes)
#[inline(never)]
pub fn f_8011cb3c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cb3c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cb3c); p = true; }
    }
    f_8011cb3c_b(c, 0x8011cb3c);
    if p { crate::prof::exit(c, 0x8011cb3c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cb3c_at(c: &mut Cpu, pc: u32) { f_8011cb3c_b(c, pc) }
#[inline(always)]
fn f_8011cb3c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cb3c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cb3c, pc); }
        match pc {
            0x8011cb3c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cb4c; continue; }
                pc = 0x8011cb44;
            }
            0x8011cb44 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cb4c;
            }
            0x8011cb4c => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cb3c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonTokuseiDataBiosGetName (0x8011cb54, 24 bytes)
#[inline(never)]
pub fn f_8011cb54(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cb54) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cb54); p = true; }
    }
    f_8011cb54_b(c, 0x8011cb54);
    if p { crate::prof::exit(c, 0x8011cb54); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cb54_at(c: &mut Cpu, pc: u32) { f_8011cb54_b(c, pc) }
#[inline(always)]
fn f_8011cb54_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cb54;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cb54, pc); }
        match pc {
            0x8011cb54 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cb64; continue; }
                pc = 0x8011cb5c;
            }
            0x8011cb5c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cb64;
            }
            0x8011cb64 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x4));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cb54: bad pc {:#x}", pc),
        }
    }
}

/// pokemonTokuseiDataBiosGetPtr (0x8011cb6c, 44 bytes)
#[inline(never)]
pub fn f_8011cb6c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cb6c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cb6c); p = true; }
    }
    f_8011cb6c_b(c, 0x8011cb6c);
    if p { crate::prof::exit(c, 0x8011cb6c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cb6c_at(c: &mut Cpu, pc: u32) { f_8011cb6c_b(c, pc) }
#[inline(always)]
fn f_8011cb6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cb6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cb6c, pc); }
        match pc {
            0x8011cb6c => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8348));
                c.r[3] = c.r[3].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011cb84; continue; }
                pc = 0x8011cb7c;
            }
            0x8011cb7c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cb84;
            }
            0x8011cb84 => {
                c.r[4] = c.r[3].wrapping_mul(0xc);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xfffff5e0);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cb6c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuRateDataBiosGetWaru (0x8011cb98, 24 bytes)
#[inline(never)]
pub fn f_8011cb98(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cb98) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cb98); p = true; }
    }
    f_8011cb98_b(c, 0x8011cb98);
    if p { crate::prof::exit(c, 0x8011cb98); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cb98_at(c: &mut Cpu, pc: u32) { f_8011cb98_b(c, pc) }
#[inline(always)]
fn f_8011cb98_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cb98;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cb98, pc); }
        match pc {
            0x8011cb98 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cba8; continue; }
                pc = 0x8011cba0;
            }
            0x8011cba0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cba8;
            }
            0x8011cba8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cb98: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuRateDataBiosGetKake (0x8011cbb0, 24 bytes)
#[inline(never)]
pub fn f_8011cbb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cbb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cbb0); p = true; }
    }
    f_8011cbb0_b(c, 0x8011cbb0);
    if p { crate::prof::exit(c, 0x8011cbb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cbb0_at(c: &mut Cpu, pc: u32) { f_8011cbb0_b(c, pc) }
#[inline(always)]
fn f_8011cbb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cbb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cbb0, pc); }
        match pc {
            0x8011cbb0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cbc0; continue; }
                pc = 0x8011cbb8;
            }
            0x8011cbb8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cbc0;
            }
            0x8011cbc0 => {
                c.r[3] = c.lbz(c.r[3]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cbb0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuRateDataBiosGetPtr (0x8011cbc8, 44 bytes)
#[inline(never)]
pub fn f_8011cbc8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cbc8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cbc8); p = true; }
    }
    f_8011cbc8_b(c, 0x8011cbc8);
    if p { crate::prof::exit(c, 0x8011cbc8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cbc8_at(c: &mut Cpu, pc: u32) { f_8011cbc8_b(c, pc) }
#[inline(always)]
fn f_8011cbc8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cbc8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cbc8, pc); }
        match pc {
            0x8011cbc8 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8638));
                c.r[5] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[5], c.r[0]);
                if c.crbit(0) { pc = 0x8011cbe4; continue; }
                pc = 0x8011cbdc;
            }
            0x8011cbdc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cbe4;
            }
            0x8011cbe4 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff863c));
                c.r[0] = c.r[3].rotate_left(1) & 0x1fe;
                c.r[3] = c.r[4].wrapping_add(c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cbc8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetCallRate (0x8011cbf4, 48 bytes)
#[inline(never)]
pub fn f_8011cbf4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cbf4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cbf4); p = true; }
    }
    f_8011cbf4_b(c, 0x8011cbf4);
    if p { crate::prof::exit(c, 0x8011cbf4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cbf4_at(c: &mut Cpu, pc: u32) { f_8011cbf4_b(c, pc) }
#[inline(always)]
fn f_8011cbf4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cbf4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cbf4, pc); }
        match pc {
            0x8011cbf4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cc04; continue; }
                pc = 0x8011cbfc;
            }
            0x8011cbfc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cc04;
            }
            0x8011cc04 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7);
                if c.crbit(0) { pc = 0x8011cc18; continue; }
                pc = 0x8011cc10;
            }
            0x8011cc10 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cc18;
            }
            0x8011cc18 => {
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x1f));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cbf4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetHyperRate (0x8011cc24, 48 bytes)
#[inline(never)]
pub fn f_8011cc24(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cc24) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cc24); p = true; }
    }
    f_8011cc24_b(c, 0x8011cc24);
    if p { crate::prof::exit(c, 0x8011cc24); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cc24_at(c: &mut Cpu, pc: u32) { f_8011cc24_b(c, pc) }
#[inline(always)]
fn f_8011cc24_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cc24;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cc24, pc); }
        match pc {
            0x8011cc24 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cc34; continue; }
                pc = 0x8011cc2c;
            }
            0x8011cc2c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cc34;
            }
            0x8011cc34 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x7);
                if c.crbit(0) { pc = 0x8011cc48; continue; }
                pc = 0x8011cc40;
            }
            0x8011cc40 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cc48;
            }
            0x8011cc48 => {
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x18));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cc24: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CC54 (0x8011cc54, 24 bytes)
#[inline(never)]
pub fn f_8011cc54(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cc54) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cc54); p = true; }
    }
    f_8011cc54_b(c, 0x8011cc54);
    if p { crate::prof::exit(c, 0x8011cc54); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cc54_at(c: &mut Cpu, pc: u32) { f_8011cc54_b(c, pc) }
#[inline(always)]
fn f_8011cc54_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cc54;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cc54, pc); }
        match pc {
            0x8011cc54 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cc64; continue; }
                pc = 0x8011cc5c;
            }
            0x8011cc5c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cc64;
            }
            0x8011cc64 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x11));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cc54: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CC6C (0x8011cc6c, 24 bytes)
#[inline(never)]
pub fn f_8011cc6c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cc6c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cc6c); p = true; }
    }
    f_8011cc6c_b(c, 0x8011cc6c);
    if p { crate::prof::exit(c, 0x8011cc6c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cc6c_at(c: &mut Cpu, pc: u32) { f_8011cc6c_b(c, pc) }
#[inline(always)]
fn f_8011cc6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cc6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cc6c, pc); }
        match pc {
            0x8011cc6c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cc7c; continue; }
                pc = 0x8011cc74;
            }
            0x8011cc74 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cc7c;
            }
            0x8011cc7c => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x10));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cc6c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CC84 (0x8011cc84, 24 bytes)
#[inline(never)]
pub fn f_8011cc84(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cc84) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cc84); p = true; }
    }
    f_8011cc84_b(c, 0x8011cc84);
    if p { crate::prof::exit(c, 0x8011cc84); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cc84_at(c: &mut Cpu, pc: u32) { f_8011cc84_b(c, pc) }
#[inline(always)]
fn f_8011cc84_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cc84;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cc84, pc); }
        match pc {
            0x8011cc84 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cc94; continue; }
                pc = 0x8011cc8c;
            }
            0x8011cc8c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cc94;
            }
            0x8011cc94 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xf));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cc84: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CC9C (0x8011cc9c, 24 bytes)
#[inline(never)]
pub fn f_8011cc9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cc9c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cc9c); p = true; }
    }
    f_8011cc9c_b(c, 0x8011cc9c);
    if p { crate::prof::exit(c, 0x8011cc9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cc9c_at(c: &mut Cpu, pc: u32) { f_8011cc9c_b(c, pc) }
#[inline(always)]
fn f_8011cc9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cc9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cc9c, pc); }
        match pc {
            0x8011cc9c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ccac; continue; }
                pc = 0x8011cca4;
            }
            0x8011cca4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ccac;
            }
            0x8011ccac => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xe));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cc9c: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CCB4 (0x8011ccb4, 24 bytes)
#[inline(never)]
pub fn f_8011ccb4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ccb4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ccb4); p = true; }
    }
    f_8011ccb4_b(c, 0x8011ccb4);
    if p { crate::prof::exit(c, 0x8011ccb4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ccb4_at(c: &mut Cpu, pc: u32) { f_8011ccb4_b(c, pc) }
#[inline(always)]
fn f_8011ccb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ccb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ccb4, pc); }
        match pc {
            0x8011ccb4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ccc4; continue; }
                pc = 0x8011ccbc;
            }
            0x8011ccbc => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ccc4;
            }
            0x8011ccc4 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xd));
                if true { return; }
                return;
            }
            _ => panic!("f_8011ccb4: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CCCC (0x8011cccc, 24 bytes)
#[inline(never)]
pub fn f_8011cccc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cccc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cccc); p = true; }
    }
    f_8011cccc_b(c, 0x8011cccc);
    if p { crate::prof::exit(c, 0x8011cccc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cccc_at(c: &mut Cpu, pc: u32) { f_8011cccc_b(c, pc) }
#[inline(always)]
fn f_8011cccc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cccc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cccc, pc); }
        match pc {
            0x8011cccc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ccdc; continue; }
                pc = 0x8011ccd4;
            }
            0x8011ccd4 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ccdc;
            }
            0x8011ccdc => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xc));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cccc: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CCE4 (0x8011cce4, 24 bytes)
#[inline(never)]
pub fn f_8011cce4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cce4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cce4); p = true; }
    }
    f_8011cce4_b(c, 0x8011cce4);
    if p { crate::prof::exit(c, 0x8011cce4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cce4_at(c: &mut Cpu, pc: u32) { f_8011cce4_b(c, pc) }
#[inline(always)]
fn f_8011cce4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cce4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cce4, pc); }
        match pc {
            0x8011cce4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ccf4; continue; }
                pc = 0x8011ccec;
            }
            0x8011ccec => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ccf4;
            }
            0x8011ccf4 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0xb));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cce4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetReliveNadenade (0x8011ccfc, 28 bytes)
#[inline(never)]
pub fn f_8011ccfc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ccfc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ccfc); p = true; }
    }
    f_8011ccfc_b(c, 0x8011ccfc);
    if p { crate::prof::exit(c, 0x8011ccfc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ccfc_at(c: &mut Cpu, pc: u32) { f_8011ccfc_b(c, pc) }
#[inline(always)]
fn f_8011ccfc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ccfc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ccfc, pc); }
        match pc {
            0x8011ccfc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cd0c; continue; }
                pc = 0x8011cd04;
            }
            0x8011cd04 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cd0c;
            }
            0x8011cd0c => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x4));
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_8011ccfc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetReliveSodateya (0x8011cd18, 28 bytes)
#[inline(never)]
pub fn f_8011cd18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cd18) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cd18); p = true; }
    }
    f_8011cd18_b(c, 0x8011cd18);
    if p { crate::prof::exit(c, 0x8011cd18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cd18_at(c: &mut Cpu, pc: u32) { f_8011cd18_b(c, pc) }
#[inline(always)]
fn f_8011cd18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cd18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cd18, pc); }
        match pc {
            0x8011cd18 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cd28; continue; }
                pc = 0x8011cd20;
            }
            0x8011cd20 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cd28;
            }
            0x8011cd28 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x3));
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_8011cd18: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetReliveCall (0x8011cd34, 28 bytes)
#[inline(never)]
pub fn f_8011cd34(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cd34) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cd34); p = true; }
    }
    f_8011cd34_b(c, 0x8011cd34);
    if p { crate::prof::exit(c, 0x8011cd34); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cd34_at(c: &mut Cpu, pc: u32) { f_8011cd34_b(c, pc) }
#[inline(always)]
fn f_8011cd34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cd34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cd34, pc); }
        match pc {
            0x8011cd34 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cd44; continue; }
                pc = 0x8011cd3c;
            }
            0x8011cd3c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cd44;
            }
            0x8011cd44 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x2));
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_8011cd34: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetReliveWalk (0x8011cd50, 28 bytes)
#[inline(never)]
pub fn f_8011cd50(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cd50) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cd50); p = true; }
    }
    f_8011cd50_b(c, 0x8011cd50);
    if p { crate::prof::exit(c, 0x8011cd50); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cd50_at(c: &mut Cpu, pc: u32) { f_8011cd50_b(c, pc) }
#[inline(always)]
fn f_8011cd50_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cd50;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cd50, pc); }
        match pc {
            0x8011cd50 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cd60; continue; }
                pc = 0x8011cd58;
            }
            0x8011cd58 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cd60;
            }
            0x8011cd60 => {
                c.r[0] = c.lbz(c.r[3].wrapping_add(0x1));
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_8011cd50: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetReliveFightout (0x8011cd6c, 28 bytes)
#[inline(never)]
pub fn f_8011cd6c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cd6c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cd6c); p = true; }
    }
    f_8011cd6c_b(c, 0x8011cd6c);
    if p { crate::prof::exit(c, 0x8011cd6c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cd6c_at(c: &mut Cpu, pc: u32) { f_8011cd6c_b(c, pc) }
#[inline(always)]
fn f_8011cd6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cd6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cd6c, pc); }
        match pc {
            0x8011cd6c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cd7c; continue; }
                pc = 0x8011cd74;
            }
            0x8011cd74 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cd7c;
            }
            0x8011cd7c => {
                c.r[0] = c.lbz(c.r[3]);
                c.r[3] = c.r[0] as u8 as i8 as i32 as u32;
                if true { return; }
                return;
            }
            _ => panic!("f_8011cd6c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetNimblenessRateDataId (0x8011cd88, 24 bytes)
#[inline(never)]
pub fn f_8011cd88(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cd88) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cd88); p = true; }
    }
    f_8011cd88_b(c, 0x8011cd88);
    if p { crate::prof::exit(c, 0x8011cd88); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cd88_at(c: &mut Cpu, pc: u32) { f_8011cd88_b(c, pc) }
#[inline(always)]
fn f_8011cd88_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cd88;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cd88, pc); }
        match pc {
            0x8011cd88 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cd98; continue; }
                pc = 0x8011cd90;
            }
            0x8011cd90 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cd98;
            }
            0x8011cd98 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x9));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cd88: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetSpeDefRateDataId (0x8011cda0, 24 bytes)
#[inline(never)]
pub fn f_8011cda0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cda0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cda0); p = true; }
    }
    f_8011cda0_b(c, 0x8011cda0);
    if p { crate::prof::exit(c, 0x8011cda0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cda0_at(c: &mut Cpu, pc: u32) { f_8011cda0_b(c, pc) }
#[inline(always)]
fn f_8011cda0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cda0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cda0, pc); }
        match pc {
            0x8011cda0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cdb0; continue; }
                pc = 0x8011cda8;
            }
            0x8011cda8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cdb0;
            }
            0x8011cdb0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x8));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cda0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetSpeAtkRateDataId (0x8011cdb8, 24 bytes)
#[inline(never)]
pub fn f_8011cdb8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cdb8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cdb8); p = true; }
    }
    f_8011cdb8_b(c, 0x8011cdb8);
    if p { crate::prof::exit(c, 0x8011cdb8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cdb8_at(c: &mut Cpu, pc: u32) { f_8011cdb8_b(c, pc) }
#[inline(always)]
fn f_8011cdb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cdb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cdb8, pc); }
        match pc {
            0x8011cdb8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cdc8; continue; }
                pc = 0x8011cdc0;
            }
            0x8011cdc0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cdc8;
            }
            0x8011cdc8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x7));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cdb8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetPhyDefRateDataId (0x8011cdd0, 24 bytes)
#[inline(never)]
pub fn f_8011cdd0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cdd0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cdd0); p = true; }
    }
    f_8011cdd0_b(c, 0x8011cdd0);
    if p { crate::prof::exit(c, 0x8011cdd0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cdd0_at(c: &mut Cpu, pc: u32) { f_8011cdd0_b(c, pc) }
#[inline(always)]
fn f_8011cdd0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cdd0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cdd0, pc); }
        match pc {
            0x8011cdd0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cde0; continue; }
                pc = 0x8011cdd8;
            }
            0x8011cdd8 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cde0;
            }
            0x8011cde0 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x6));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cdd0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetPhyAtkRateDataId (0x8011cde8, 24 bytes)
#[inline(never)]
pub fn f_8011cde8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cde8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cde8); p = true; }
    }
    f_8011cde8_b(c, 0x8011cde8);
    if p { crate::prof::exit(c, 0x8011cde8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cde8_at(c: &mut Cpu, pc: u32) { f_8011cde8_b(c, pc) }
#[inline(always)]
fn f_8011cde8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cde8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cde8, pc); }
        match pc {
            0x8011cde8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cdf8; continue; }
                pc = 0x8011cdf0;
            }
            0x8011cdf0 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011cdf8;
            }
            0x8011cdf8 => {
                c.r[3] = c.lbz(c.r[3].wrapping_add(0x5));
                if true { return; }
                return;
            }
            _ => panic!("f_8011cde8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetName (0x8011ce00, 24 bytes)
#[inline(never)]
pub fn f_8011ce00(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ce00) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ce00); p = true; }
    }
    f_8011ce00_b(c, 0x8011ce00);
    if p { crate::prof::exit(c, 0x8011ce00); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ce00_at(c: &mut Cpu, pc: u32) { f_8011ce00_b(c, pc) }
#[inline(always)]
fn f_8011ce00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ce00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ce00, pc); }
        match pc {
            0x8011ce00 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ce10; continue; }
                pc = 0x8011ce08;
            }
            0x8011ce08 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ce10;
            }
            0x8011ce10 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x14));
                if true { return; }
                return;
            }
            _ => panic!("f_8011ce00: bad pc {:#x}", pc),
        }
    }
}

/// pokemonSeikakuDataBiosGetPtr (0x8011ce18, 44 bytes)
#[inline(never)]
pub fn f_8011ce18(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ce18) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ce18); p = true; }
    }
    f_8011ce18_b(c, 0x8011ce18);
    if p { crate::prof::exit(c, 0x8011ce18); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ce18_at(c: &mut Cpu, pc: u32) { f_8011ce18_b(c, pc) }
#[inline(always)]
fn f_8011ce18_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ce18;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ce18, pc); }
        match pc {
            0x8011ce18 => {
                c.r[4] = c.lwz(c.r[13].wrapping_add(0xffff8640));
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.r[0] = c.lwz(c.r[4]);
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011ce34; continue; }
                pc = 0x8011ce2c;
            }
            0x8011ce2c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ce34;
            }
            0x8011ce34 => {
                c.r[0] = c.r[3].wrapping_mul(0x28);
                c.r[3] = c.lwz(c.r[13].wrapping_add(0xffff8644));
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011ce18: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGrowDataBiosGetExp (0x8011ce44, 48 bytes)
#[inline(never)]
pub fn f_8011ce44(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ce44) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ce44); p = true; }
    }
    f_8011ce44_b(c, 0x8011ce44);
    if p { crate::prof::exit(c, 0x8011ce44); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ce44_at(c: &mut Cpu, pc: u32) { f_8011ce44_b(c, pc) }
#[inline(always)]
fn f_8011ce44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ce44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ce44, pc); }
        match pc {
            0x8011ce44 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011ce54; continue; }
                pc = 0x8011ce4c;
            }
            0x8011ce4c => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ce54;
            }
            0x8011ce54 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x65);
                if c.crbit(0) { pc = 0x8011ce68; continue; }
                pc = 0x8011ce60;
            }
            0x8011ce60 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ce68;
            }
            0x8011ce68 => {
                c.r[0] = c.r[4].rotate_left(2) & 0x3fc;
                c.r[3] = c.lwz(c.r[3].wrapping_add(c.r[0]));
                if true { return; }
                return;
            }
            _ => panic!("f_8011ce44: bad pc {:#x}", pc),
        }
    }
}

/// pokemonGrowDataBiosGetPtr (0x8011ce74, 44 bytes)
#[inline(never)]
pub fn f_8011ce74(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ce74) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ce74); p = true; }
    }
    f_8011ce74_b(c, 0x8011ce74);
    if p { crate::prof::exit(c, 0x8011ce74); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ce74_at(c: &mut Cpu, pc: u32) { f_8011ce74_b(c, pc) }
#[inline(always)]
fn f_8011ce74_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ce74;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ce74, pc); }
        match pc {
            0x8011ce74 => {
                c.r[0] = c.lwz(c.r[13].wrapping_add(0xffff8340));
                c.r[3] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[3], c.r[0]);
                if c.crbit(0) { pc = 0x8011ce8c; continue; }
                pc = 0x8011ce84;
            }
            0x8011ce84 => {
                c.r[3] = 0x0;
                if true { return; }
                pc = 0x8011ce8c;
            }
            0x8011ce8c => {
                c.r[4] = c.r[3].wrapping_mul(0x194);
                c.r[3] = 0x80360000;
                c.r[0] = c.r[3].wrapping_add(0xffffe940);
                c.r[3] = c.r[0].wrapping_add(c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011ce74: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CEA0 (0x8011cea0, 16 bytes)
#[inline(never)]
pub fn f_8011cea0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cea0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cea0); p = true; }
    }
    f_8011cea0_b(c, 0x8011cea0);
    if p { crate::prof::exit(c, 0x8011cea0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cea0_at(c: &mut Cpu, pc: u32) { f_8011cea0_b(c, pc) }
#[inline(always)]
fn f_8011cea0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cea0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cea0, pc); }
        match pc {
            0x8011cea0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cea8;
            }
            0x8011cea8 => {
                c.stb(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cea0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CEB0 (0x8011ceb0, 16 bytes)
#[inline(never)]
pub fn f_8011ceb0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ceb0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ceb0); p = true; }
    }
    f_8011ceb0_b(c, 0x8011ceb0);
    if p { crate::prof::exit(c, 0x8011ceb0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ceb0_at(c: &mut Cpu, pc: u32) { f_8011ceb0_b(c, pc) }
#[inline(always)]
fn f_8011ceb0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ceb0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ceb0, pc); }
        match pc {
            0x8011ceb0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011ceb8;
            }
            0x8011ceb8 => {
                c.stb(c.r[3].wrapping_add(0x3), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011ceb0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CEC0 (0x8011cec0, 16 bytes)
#[inline(never)]
pub fn f_8011cec0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cec0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cec0); p = true; }
    }
    f_8011cec0_b(c, 0x8011cec0);
    if p { crate::prof::exit(c, 0x8011cec0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cec0_at(c: &mut Cpu, pc: u32) { f_8011cec0_b(c, pc) }
#[inline(always)]
fn f_8011cec0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cec0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cec0, pc); }
        match pc {
            0x8011cec0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cec8;
            }
            0x8011cec8 => {
                c.sth(c.r[3].wrapping_add(0x16), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cec0: bad pc {:#x}", pc),
        }
    }
}

/// fn_8011CED0 (0x8011ced0, 32 bytes)
#[inline(never)]
pub fn f_8011ced0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ced0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ced0); p = true; }
    }
    f_8011ced0_b(c, 0x8011ced0);
    if p { crate::prof::exit(c, 0x8011ced0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ced0_at(c: &mut Cpu, pc: u32) { f_8011ced0_b(c, pc) }
#[inline(always)]
fn f_8011ced0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ced0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ced0, pc); }
        match pc {
            0x8011ced0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011ced8;
            }
            0x8011ced8 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(0) { return; }
                pc = 0x8011cee4;
            }
            0x8011cee4 => {
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x6e), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011ced0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetKowaza (0x8011cef0, 36 bytes)
#[inline(never)]
pub fn f_8011cef0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cef0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cef0); p = true; }
    }
    f_8011cef0_b(c, 0x8011cef0);
    if p { crate::prof::exit(c, 0x8011cef0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cef0_at(c: &mut Cpu, pc: u32) { f_8011cef0_b(c, pc) }
#[inline(always)]
fn f_8011cef0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cef0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cef0, pc); }
        match pc {
            0x8011cef0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cef8;
            }
            0x8011cef8 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x8);
                if !c.crbit(0) { return; }
                pc = 0x8011cf04;
            }
            0x8011cf04 => {
                c.r[0] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.sth(c.r[3].wrapping_add(0x74), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cef0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetPkxDataId (0x8011cf14, 16 bytes)
#[inline(never)]
pub fn f_8011cf14(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cf14) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cf14); p = true; }
    }
    f_8011cf14_b(c, 0x8011cf14);
    if p { crate::prof::exit(c, 0x8011cf14); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cf14_at(c: &mut Cpu, pc: u32) { f_8011cf14_b(c, pc) }
#[inline(always)]
fn f_8011cf14_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cf14;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cf14, pc); }
        match pc {
            0x8011cf14 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cf1c;
            }
            0x8011cf1c => {
                c.stw(c.r[3].wrapping_add(0x2c), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cf14: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetNumPokemon (0x8011cf24, 16 bytes)
#[inline(never)]
pub fn f_8011cf24(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cf24) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cf24); p = true; }
    }
    f_8011cf24_b(c, 0x8011cf24);
    if p { crate::prof::exit(c, 0x8011cf24); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cf24_at(c: &mut Cpu, pc: u32) { f_8011cf24_b(c, pc) }
#[inline(always)]
fn f_8011cf24_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cf24;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cf24, pc); }
        match pc {
            0x8011cf24 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cf2c;
            }
            0x8011cf2c => {
                c.sth(c.r[3].wrapping_add(0x12), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cf24: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetNumZukan (0x8011cf34, 16 bytes)
#[inline(never)]
pub fn f_8011cf34(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cf34) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cf34); p = true; }
    }
    f_8011cf34_b(c, 0x8011cf34);
    if p { crate::prof::exit(c, 0x8011cf34); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cf34_at(c: &mut Cpu, pc: u32) { f_8011cf34_b(c, pc) }
#[inline(always)]
fn f_8011cf34_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cf34;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cf34, pc); }
        match pc {
            0x8011cf34 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cf3c;
            }
            0x8011cf3c => {
                c.sth(c.r[3].wrapping_add(0x14), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cf34: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetTukamaetaFlag (0x8011cf44, 44 bytes)
#[inline(never)]
pub fn f_8011cf44(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cf44) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cf44); p = true; }
    }
    f_8011cf44_b(c, 0x8011cf44);
    if p { crate::prof::exit(c, 0x8011cf44); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cf44_at(c: &mut Cpu, pc: u32) { f_8011cf44_b(c, pc) }
#[inline(always)]
fn f_8011cf44_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cf44;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cf44, pc); }
        match pc {
            0x8011cf44 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if c.crbit(2) { pc = 0x8011cf60; continue; }
                pc = 0x8011cf58;
            }
            0x8011cf58 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x28));
                c.lr = 0x8011cf60; f_8019075c(c);
                pc = 0x8011cf60;
            }
            0x8011cf60 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cf44: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetMitaFlag (0x8011cf70, 44 bytes)
#[inline(never)]
pub fn f_8011cf70(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cf70) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cf70); p = true; }
    }
    f_8011cf70_b(c, 0x8011cf70);
    if p { crate::prof::exit(c, 0x8011cf70); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cf70_at(c: &mut Cpu, pc: u32) { f_8011cf70_b(c, pc) }
#[inline(always)]
fn f_8011cf70_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cf70;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cf70, pc); }
        match pc {
            0x8011cf70 => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.cmpu(0, c.r[3], 0x0);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                if c.crbit(2) { pc = 0x8011cf8c; continue; }
                pc = 0x8011cf84;
            }
            0x8011cf84 => {
                c.r[3] = c.lwz(c.r[3].wrapping_add(0x24));
                c.lr = 0x8011cf8c; f_8019075c(c);
                pc = 0x8011cf8c;
            }
            0x8011cf8c => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cf70: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetVoice (0x8011cf9c, 16 bytes)
#[inline(never)]
pub fn f_8011cf9c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cf9c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cf9c); p = true; }
    }
    f_8011cf9c_b(c, 0x8011cf9c);
    if p { crate::prof::exit(c, 0x8011cf9c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cf9c_at(c: &mut Cpu, pc: u32) { f_8011cf9c_b(c, pc) }
#[inline(always)]
fn f_8011cf9c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cf9c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cf9c, pc); }
        match pc {
            0x8011cf9c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cfa4;
            }
            0x8011cfa4 => {
                c.sth(c.r[3].wrapping_add(0xe), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cf9c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetDoc (0x8011cfac, 16 bytes)
#[inline(never)]
pub fn f_8011cfac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cfac) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cfac); p = true; }
    }
    f_8011cfac_b(c, 0x8011cfac);
    if p { crate::prof::exit(c, 0x8011cfac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cfac_at(c: &mut Cpu, pc: u32) { f_8011cfac_b(c, pc) }
#[inline(always)]
fn f_8011cfac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cfac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cfac, pc); }
        match pc {
            0x8011cfac => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cfb4;
            }
            0x8011cfb4 => {
                c.stw(c.r[3].wrapping_add(0x20), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cfac: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetWeight (0x8011cfbc, 16 bytes)
#[inline(never)]
pub fn f_8011cfbc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cfbc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cfbc); p = true; }
    }
    f_8011cfbc_b(c, 0x8011cfbc);
    if p { crate::prof::exit(c, 0x8011cfbc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cfbc_at(c: &mut Cpu, pc: u32) { f_8011cfbc_b(c, pc) }
#[inline(always)]
fn f_8011cfbc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cfbc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cfbc, pc); }
        match pc {
            0x8011cfbc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cfc4;
            }
            0x8011cfc4 => {
                c.sth(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cfbc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetHeight (0x8011cfcc, 16 bytes)
#[inline(never)]
pub fn f_8011cfcc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cfcc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cfcc); p = true; }
    }
    f_8011cfcc_b(c, 0x8011cfcc);
    if p { crate::prof::exit(c, 0x8011cfcc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cfcc_at(c: &mut Cpu, pc: u32) { f_8011cfcc_b(c, pc) }
#[inline(always)]
fn f_8011cfcc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cfcc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cfcc, pc); }
        match pc {
            0x8011cfcc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cfd4;
            }
            0x8011cfd4 => {
                c.sth(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cfcc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetTypeName (0x8011cfdc, 16 bytes)
#[inline(never)]
pub fn f_8011cfdc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cfdc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cfdc); p = true; }
    }
    f_8011cfdc_b(c, 0x8011cfdc);
    if p { crate::prof::exit(c, 0x8011cfdc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cfdc_at(c: &mut Cpu, pc: u32) { f_8011cfdc_b(c, pc) }
#[inline(always)]
fn f_8011cfdc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cfdc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cfdc, pc); }
        match pc {
            0x8011cfdc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011cfe4;
            }
            0x8011cfe4 => {
                c.stw(c.r[3].wrapping_add(0x1c), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cfdc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetColor (0x8011cfec, 64 bytes)
#[inline(never)]
pub fn f_8011cfec(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011cfec) { return; }
        if c.prof { crate::prof::enter(c, 0x8011cfec); p = true; }
    }
    f_8011cfec_b(c, 0x8011cfec);
    if p { crate::prof::exit(c, 0x8011cfec); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011cfec_at(c: &mut Cpu, pc: u32) { f_8011cfec_b(c, pc) }
#[inline(always)]
fn f_8011cfec_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011cfec;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011cfec, pc); }
        match pc {
            0x8011cfec => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011cffc; continue; }
                pc = 0x8011cff4;
            }
            0x8011cff4 => {
                c.r[4] = 0x0;
                pc = 0x8011d01c; continue;
                pc = 0x8011cffc;
            }
            0x8011cffc => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8011d010; continue; }
                pc = 0x8011d008;
            }
            0x8011d008 => {
                c.r[4] = 0x0;
                pc = 0x8011d01c; continue;
                pc = 0x8011d010;
            }
            0x8011d010 => {
                c.r[4] = c.r[4].rotate_left(3) & 0x7fff8;
                c.r[4] = c.r[4].wrapping_add(0x10c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011d01c;
            }
            0x8011d01c => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d024;
            }
            0x8011d024 => {
                c.stb(c.r[4], c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011cfec: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetStatusFaceMenuSpriteId (0x8011d02c, 64 bytes)
#[inline(never)]
pub fn f_8011d02c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d02c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d02c); p = true; }
    }
    f_8011d02c_b(c, 0x8011d02c);
    if p { crate::prof::exit(c, 0x8011d02c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d02c_at(c: &mut Cpu, pc: u32) { f_8011d02c_b(c, pc) }
#[inline(always)]
fn f_8011d02c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d02c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d02c, pc); }
        match pc {
            0x8011d02c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d03c; continue; }
                pc = 0x8011d034;
            }
            0x8011d034 => {
                c.r[4] = 0x0;
                pc = 0x8011d05c; continue;
                pc = 0x8011d03c;
            }
            0x8011d03c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8011d050; continue; }
                pc = 0x8011d048;
            }
            0x8011d048 => {
                c.r[4] = 0x0;
                pc = 0x8011d05c; continue;
                pc = 0x8011d050;
            }
            0x8011d050 => {
                c.r[4] = c.r[4].rotate_left(3) & 0x7fff8;
                c.r[4] = c.r[4].wrapping_add(0x10c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011d05c;
            }
            0x8011d05c => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d064;
            }
            0x8011d064 => {
                c.sth(c.r[4].wrapping_add(0x2), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d02c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetPokebodyId (0x8011d06c, 64 bytes)
#[inline(never)]
pub fn f_8011d06c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d06c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d06c); p = true; }
    }
    f_8011d06c_b(c, 0x8011d06c);
    if p { crate::prof::exit(c, 0x8011d06c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d06c_at(c: &mut Cpu, pc: u32) { f_8011d06c_b(c, pc) }
#[inline(always)]
fn f_8011d06c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d06c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d06c, pc); }
        match pc {
            0x8011d06c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d07c; continue; }
                pc = 0x8011d074;
            }
            0x8011d074 => {
                c.r[4] = 0x0;
                pc = 0x8011d09c; continue;
                pc = 0x8011d07c;
            }
            0x8011d07c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if c.crbit(0) { pc = 0x8011d090; continue; }
                pc = 0x8011d088;
            }
            0x8011d088 => {
                c.r[4] = 0x0;
                pc = 0x8011d09c; continue;
                pc = 0x8011d090;
            }
            0x8011d090 => {
                c.r[4] = c.r[4].rotate_left(3) & 0x7fff8;
                c.r[4] = c.r[4].wrapping_add(0x10c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011d09c;
            }
            0x8011d09c => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d0a4;
            }
            0x8011d0a4 => {
                c.stw(c.r[4].wrapping_add(0x4), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d06c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetWazaMcn (0x8011d0ac, 32 bytes)
#[inline(never)]
pub fn f_8011d0ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d0ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d0ac); p = true; }
    }
    f_8011d0ac_b(c, 0x8011d0ac);
    if p { crate::prof::exit(c, 0x8011d0ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d0ac_at(c: &mut Cpu, pc: u32) { f_8011d0ac_b(c, pc) }
#[inline(always)]
fn f_8011d0ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d0ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d0ac, pc); }
        match pc {
            0x8011d0ac => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d0b4;
            }
            0x8011d0b4 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x3a);
                if !c.crbit(0) { return; }
                pc = 0x8011d0c0;
            }
            0x8011d0c0 => {
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x34), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d0ac: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGetWazaDataId (0x8011d0cc, 64 bytes)
#[inline(never)]
pub fn f_8011d0cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d0cc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d0cc); p = true; }
    }
    f_8011d0cc_b(c, 0x8011d0cc);
    if p { crate::prof::exit(c, 0x8011d0cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d0cc_at(c: &mut Cpu, pc: u32) { f_8011d0cc_b(c, pc) }
#[inline(always)]
fn f_8011d0cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d0cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d0cc, pc); }
        match pc {
            0x8011d0cc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d0dc; continue; }
                pc = 0x8011d0d4;
            }
            0x8011d0d4 => {
                c.r[4] = 0x0;
                pc = 0x8011d0fc; continue;
                pc = 0x8011d0dc;
            }
            0x8011d0dc => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x14);
                if c.crbit(0) { pc = 0x8011d0f0; continue; }
                pc = 0x8011d0e8;
            }
            0x8011d0e8 => {
                c.r[4] = 0x0;
                pc = 0x8011d0fc; continue;
                pc = 0x8011d0f0;
            }
            0x8011d0f0 => {
                c.r[4] = c.r[4].rotate_left(2) & 0x3fffc;
                c.r[4] = c.r[4].wrapping_add(0xba);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011d0fc;
            }
            0x8011d0fc => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d104;
            }
            0x8011d104 => {
                c.sth(c.r[4].wrapping_add(0x2), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d0cc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGetWazaLevel (0x8011d10c, 64 bytes)
#[inline(never)]
pub fn f_8011d10c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d10c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d10c); p = true; }
    }
    f_8011d10c_b(c, 0x8011d10c);
    if p { crate::prof::exit(c, 0x8011d10c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d10c_at(c: &mut Cpu, pc: u32) { f_8011d10c_b(c, pc) }
#[inline(always)]
fn f_8011d10c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d10c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d10c, pc); }
        match pc {
            0x8011d10c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d11c; continue; }
                pc = 0x8011d114;
            }
            0x8011d114 => {
                c.r[4] = 0x0;
                pc = 0x8011d13c; continue;
                pc = 0x8011d11c;
            }
            0x8011d11c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x14);
                if c.crbit(0) { pc = 0x8011d130; continue; }
                pc = 0x8011d128;
            }
            0x8011d128 => {
                c.r[4] = 0x0;
                pc = 0x8011d13c; continue;
                pc = 0x8011d130;
            }
            0x8011d130 => {
                c.r[4] = c.r[4].rotate_left(2) & 0x3fffc;
                c.r[4] = c.r[4].wrapping_add(0xba);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011d13c;
            }
            0x8011d13c => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d144;
            }
            0x8011d144 => {
                c.stb(c.r[4], c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d10c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetSinkaPokemonDataId (0x8011d14c, 64 bytes)
#[inline(never)]
pub fn f_8011d14c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d14c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d14c); p = true; }
    }
    f_8011d14c_b(c, 0x8011d14c);
    if p { crate::prof::exit(c, 0x8011d14c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d14c_at(c: &mut Cpu, pc: u32) { f_8011d14c_b(c, pc) }
#[inline(always)]
fn f_8011d14c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d14c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d14c, pc); }
        match pc {
            0x8011d14c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d15c; continue; }
                pc = 0x8011d154;
            }
            0x8011d154 => {
                c.r[4] = 0x0;
                pc = 0x8011d17c; continue;
                pc = 0x8011d15c;
            }
            0x8011d15c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x5);
                if c.crbit(0) { pc = 0x8011d170; continue; }
                pc = 0x8011d168;
            }
            0x8011d168 => {
                c.r[4] = 0x0;
                pc = 0x8011d17c; continue;
                pc = 0x8011d170;
            }
            0x8011d170 => {
                c.r[4] = c.r[0].wrapping_mul(0x6);
                c.r[4] = c.r[4].wrapping_add(0x9c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011d17c;
            }
            0x8011d17c => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d184;
            }
            0x8011d184 => {
                c.sth(c.r[4].wrapping_add(0x4), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d14c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetSinkaBuff (0x8011d18c, 64 bytes)
#[inline(never)]
pub fn f_8011d18c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d18c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d18c); p = true; }
    }
    f_8011d18c_b(c, 0x8011d18c);
    if p { crate::prof::exit(c, 0x8011d18c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d18c_at(c: &mut Cpu, pc: u32) { f_8011d18c_b(c, pc) }
#[inline(always)]
fn f_8011d18c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d18c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d18c, pc); }
        match pc {
            0x8011d18c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d19c; continue; }
                pc = 0x8011d194;
            }
            0x8011d194 => {
                c.r[4] = 0x0;
                pc = 0x8011d1bc; continue;
                pc = 0x8011d19c;
            }
            0x8011d19c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x5);
                if c.crbit(0) { pc = 0x8011d1b0; continue; }
                pc = 0x8011d1a8;
            }
            0x8011d1a8 => {
                c.r[4] = 0x0;
                pc = 0x8011d1bc; continue;
                pc = 0x8011d1b0;
            }
            0x8011d1b0 => {
                c.r[4] = c.r[0].wrapping_mul(0x6);
                c.r[4] = c.r[4].wrapping_add(0x9c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011d1bc;
            }
            0x8011d1bc => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d1c4;
            }
            0x8011d1c4 => {
                c.sth(c.r[4].wrapping_add(0x2), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d18c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetSinkaKind (0x8011d1cc, 64 bytes)
#[inline(never)]
pub fn f_8011d1cc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d1cc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d1cc); p = true; }
    }
    f_8011d1cc_b(c, 0x8011d1cc);
    if p { crate::prof::exit(c, 0x8011d1cc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d1cc_at(c: &mut Cpu, pc: u32) { f_8011d1cc_b(c, pc) }
#[inline(always)]
fn f_8011d1cc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d1cc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d1cc, pc); }
        match pc {
            0x8011d1cc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d1dc; continue; }
                pc = 0x8011d1d4;
            }
            0x8011d1d4 => {
                c.r[4] = 0x0;
                pc = 0x8011d1fc; continue;
                pc = 0x8011d1dc;
            }
            0x8011d1dc => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x5);
                if c.crbit(0) { pc = 0x8011d1f0; continue; }
                pc = 0x8011d1e8;
            }
            0x8011d1e8 => {
                c.r[4] = 0x0;
                pc = 0x8011d1fc; continue;
                pc = 0x8011d1f0;
            }
            0x8011d1f0 => {
                c.r[4] = c.r[0].wrapping_mul(0x6);
                c.r[4] = c.r[4].wrapping_add(0x9c);
                c.r[4] = c.r[3].wrapping_add(c.r[4]);
                pc = 0x8011d1fc;
            }
            0x8011d1fc => {
                c.cmpu(0, c.r[4], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d204;
            }
            0x8011d204 => {
                c.stb(c.r[4], c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d1cc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetTokuseiDataId (0x8011d20c, 32 bytes)
#[inline(never)]
pub fn f_8011d20c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d20c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d20c); p = true; }
    }
    f_8011d20c_b(c, 0x8011d20c);
    if p { crate::prof::exit(c, 0x8011d20c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d20c_at(c: &mut Cpu, pc: u32) { f_8011d20c_b(c, pc) }
#[inline(always)]
fn f_8011d20c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d20c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d20c, pc); }
        match pc {
            0x8011d20c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d214;
            }
            0x8011d214 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(0) { return; }
                pc = 0x8011d220;
            }
            0x8011d220 => {
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x32), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d20c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetZokuseiDataId (0x8011d22c, 32 bytes)
#[inline(never)]
pub fn f_8011d22c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d22c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d22c); p = true; }
    }
    f_8011d22c_b(c, 0x8011d22c);
    if p { crate::prof::exit(c, 0x8011d22c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d22c_at(c: &mut Cpu, pc: u32) { f_8011d22c_b(c, pc) }
#[inline(always)]
fn f_8011d22c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d22c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d22c, pc); }
        match pc {
            0x8011d22c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d234;
            }
            0x8011d234 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(0) { return; }
                pc = 0x8011d240;
            }
            0x8011d240 => {
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.stb(c.r[3].wrapping_add(0x30), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d22c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetItemDataId (0x8011d24c, 36 bytes)
#[inline(never)]
pub fn f_8011d24c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d24c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d24c); p = true; }
    }
    f_8011d24c_b(c, 0x8011d24c);
    if p { crate::prof::exit(c, 0x8011d24c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d24c_at(c: &mut Cpu, pc: u32) { f_8011d24c_b(c, pc) }
#[inline(always)]
fn f_8011d24c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d24c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d24c, pc); }
        match pc {
            0x8011d24c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d254;
            }
            0x8011d254 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x2);
                if !c.crbit(0) { return; }
                pc = 0x8011d260;
            }
            0x8011d260 => {
                c.r[0] = c.r[4].rotate_left(1) & 0x1fffe;
                c.r[3] = c.r[3].wrapping_add(c.r[0]);
                c.sth(c.r[3].wrapping_add(0x70), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d24c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetInitFriend (0x8011d270, 16 bytes)
#[inline(never)]
pub fn f_8011d270(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d270) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d270); p = true; }
    }
    f_8011d270_b(c, 0x8011d270);
    if p { crate::prof::exit(c, 0x8011d270); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d270_at(c: &mut Cpu, pc: u32) { f_8011d270_b(c, pc) }
#[inline(always)]
fn f_8011d270_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d270;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d270, pc); }
        match pc {
            0x8011d270 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d278;
            }
            0x8011d278 => {
                c.sth(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d270: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetSexRatio (0x8011d280, 16 bytes)
#[inline(never)]
pub fn f_8011d280(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d280) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d280); p = true; }
    }
    f_8011d280_b(c, 0x8011d280);
    if p { crate::prof::exit(c, 0x8011d280); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d280_at(c: &mut Cpu, pc: u32) { f_8011d280_b(c, pc) }
#[inline(always)]
fn f_8011d280_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d280;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d280, pc); }
        match pc {
            0x8011d280 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d288;
            }
            0x8011d288 => {
                c.stb(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d280: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGet (0x8011d290, 16 bytes)
#[inline(never)]
pub fn f_8011d290(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d290) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d290); p = true; }
    }
    f_8011d290_b(c, 0x8011d290);
    if p { crate::prof::exit(c, 0x8011d290); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d290_at(c: &mut Cpu, pc: u32) { f_8011d290_b(c, pc) }
#[inline(always)]
fn f_8011d290_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d290;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d290, pc); }
        match pc {
            0x8011d290 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d298;
            }
            0x8011d298 => {
                c.stb(c.r[3].wrapping_add(0x1), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d290: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGrowDataId (0x8011d2a0, 16 bytes)
#[inline(never)]
pub fn f_8011d2a0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d2a0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d2a0); p = true; }
    }
    f_8011d2a0_b(c, 0x8011d2a0);
    if p { crate::prof::exit(c, 0x8011d2a0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d2a0_at(c: &mut Cpu, pc: u32) { f_8011d2a0_b(c, pc) }
#[inline(always)]
fn f_8011d2a0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d2a0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d2a0, pc); }
        match pc {
            0x8011d2a0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d2a8;
            }
            0x8011d2a8 => {
                c.stb(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d2a0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGiveExp (0x8011d2b0, 16 bytes)
#[inline(never)]
pub fn f_8011d2b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d2b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d2b0); p = true; }
    }
    f_8011d2b0_b(c, 0x8011d2b0);
    if p { crate::prof::exit(c, 0x8011d2b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d2b0_at(c: &mut Cpu, pc: u32) { f_8011d2b0_b(c, pc) }
#[inline(always)]
fn f_8011d2b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d2b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d2b0, pc); }
        match pc {
            0x8011d2b0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d2b8;
            }
            0x8011d2b8 => {
                c.sth(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d2b0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGiveNimblenessEffort (0x8011d2c0, 36 bytes)
#[inline(never)]
pub fn f_8011d2c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d2c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d2c0); p = true; }
    }
    f_8011d2c0_b(c, 0x8011d2c0);
    if p { crate::prof::exit(c, 0x8011d2c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d2c0_at(c: &mut Cpu, pc: u32) { f_8011d2c0_b(c, pc) }
#[inline(always)]
fn f_8011d2c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d2c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d2c0, pc); }
        match pc {
            0x8011d2c0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d2d0; continue; }
                pc = 0x8011d2c8;
            }
            0x8011d2c8 => {
                c.r[3] = 0x0;
                pc = 0x8011d2d4; continue;
                pc = 0x8011d2d0;
            }
            0x8011d2d0 => {
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011d2d4;
            }
            0x8011d2d4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d2dc;
            }
            0x8011d2dc => {
                c.sth(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d2c0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGiveSpeDefEffort (0x8011d2e4, 36 bytes)
#[inline(never)]
pub fn f_8011d2e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d2e4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d2e4); p = true; }
    }
    f_8011d2e4_b(c, 0x8011d2e4);
    if p { crate::prof::exit(c, 0x8011d2e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d2e4_at(c: &mut Cpu, pc: u32) { f_8011d2e4_b(c, pc) }
#[inline(always)]
fn f_8011d2e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d2e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d2e4, pc); }
        match pc {
            0x8011d2e4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d2f4; continue; }
                pc = 0x8011d2ec;
            }
            0x8011d2ec => {
                c.r[3] = 0x0;
                pc = 0x8011d2f8; continue;
                pc = 0x8011d2f4;
            }
            0x8011d2f4 => {
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011d2f8;
            }
            0x8011d2f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d300;
            }
            0x8011d300 => {
                c.sth(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d2e4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGiveSpeAtkEffort (0x8011d308, 36 bytes)
#[inline(never)]
pub fn f_8011d308(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d308) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d308); p = true; }
    }
    f_8011d308_b(c, 0x8011d308);
    if p { crate::prof::exit(c, 0x8011d308); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d308_at(c: &mut Cpu, pc: u32) { f_8011d308_b(c, pc) }
#[inline(always)]
fn f_8011d308_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d308;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d308, pc); }
        match pc {
            0x8011d308 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d318; continue; }
                pc = 0x8011d310;
            }
            0x8011d310 => {
                c.r[3] = 0x0;
                pc = 0x8011d31c; continue;
                pc = 0x8011d318;
            }
            0x8011d318 => {
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011d31c;
            }
            0x8011d31c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d324;
            }
            0x8011d324 => {
                c.sth(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d308: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGivePhyDefEffort (0x8011d32c, 36 bytes)
#[inline(never)]
pub fn f_8011d32c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d32c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d32c); p = true; }
    }
    f_8011d32c_b(c, 0x8011d32c);
    if p { crate::prof::exit(c, 0x8011d32c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d32c_at(c: &mut Cpu, pc: u32) { f_8011d32c_b(c, pc) }
#[inline(always)]
fn f_8011d32c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d32c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d32c, pc); }
        match pc {
            0x8011d32c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d33c; continue; }
                pc = 0x8011d334;
            }
            0x8011d334 => {
                c.r[3] = 0x0;
                pc = 0x8011d340; continue;
                pc = 0x8011d33c;
            }
            0x8011d33c => {
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011d340;
            }
            0x8011d340 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d348;
            }
            0x8011d348 => {
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d32c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGivePhyAtkEffort (0x8011d350, 36 bytes)
#[inline(never)]
pub fn f_8011d350(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d350) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d350); p = true; }
    }
    f_8011d350_b(c, 0x8011d350);
    if p { crate::prof::exit(c, 0x8011d350); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d350_at(c: &mut Cpu, pc: u32) { f_8011d350_b(c, pc) }
#[inline(always)]
fn f_8011d350_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d350;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d350, pc); }
        match pc {
            0x8011d350 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d360; continue; }
                pc = 0x8011d358;
            }
            0x8011d358 => {
                c.r[3] = 0x0;
                pc = 0x8011d364; continue;
                pc = 0x8011d360;
            }
            0x8011d360 => {
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011d364;
            }
            0x8011d364 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d36c;
            }
            0x8011d36c => {
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d350: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetGiveMaxHpEffort (0x8011d374, 36 bytes)
#[inline(never)]
pub fn f_8011d374(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d374) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d374); p = true; }
    }
    f_8011d374_b(c, 0x8011d374);
    if p { crate::prof::exit(c, 0x8011d374); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d374_at(c: &mut Cpu, pc: u32) { f_8011d374_b(c, pc) }
#[inline(always)]
fn f_8011d374_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d374;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d374, pc); }
        match pc {
            0x8011d374 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d384; continue; }
                pc = 0x8011d37c;
            }
            0x8011d37c => {
                c.r[3] = 0x0;
                pc = 0x8011d388; continue;
                pc = 0x8011d384;
            }
            0x8011d384 => {
                c.r[3] = c.r[3].wrapping_add(0x90);
                pc = 0x8011d388;
            }
            0x8011d388 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d390;
            }
            0x8011d390 => {
                c.sth(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d374: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetBasisNimbleness (0x8011d398, 36 bytes)
#[inline(never)]
pub fn f_8011d398(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d398) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d398); p = true; }
    }
    f_8011d398_b(c, 0x8011d398);
    if p { crate::prof::exit(c, 0x8011d398); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d398_at(c: &mut Cpu, pc: u32) { f_8011d398_b(c, pc) }
#[inline(always)]
fn f_8011d398_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d398;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d398, pc); }
        match pc {
            0x8011d398 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d3a8; continue; }
                pc = 0x8011d3a0;
            }
            0x8011d3a0 => {
                c.r[3] = 0x0;
                pc = 0x8011d3ac; continue;
                pc = 0x8011d3a8;
            }
            0x8011d3a8 => {
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011d3ac;
            }
            0x8011d3ac => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d3b4;
            }
            0x8011d3b4 => {
                c.sth(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d398: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetBasisSpeDef (0x8011d3bc, 36 bytes)
#[inline(never)]
pub fn f_8011d3bc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d3bc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d3bc); p = true; }
    }
    f_8011d3bc_b(c, 0x8011d3bc);
    if p { crate::prof::exit(c, 0x8011d3bc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d3bc_at(c: &mut Cpu, pc: u32) { f_8011d3bc_b(c, pc) }
#[inline(always)]
fn f_8011d3bc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d3bc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d3bc, pc); }
        match pc {
            0x8011d3bc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d3cc; continue; }
                pc = 0x8011d3c4;
            }
            0x8011d3c4 => {
                c.r[3] = 0x0;
                pc = 0x8011d3d0; continue;
                pc = 0x8011d3cc;
            }
            0x8011d3cc => {
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011d3d0;
            }
            0x8011d3d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d3d8;
            }
            0x8011d3d8 => {
                c.sth(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d3bc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetBasisSpeAtk (0x8011d3e0, 36 bytes)
#[inline(never)]
pub fn f_8011d3e0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d3e0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d3e0); p = true; }
    }
    f_8011d3e0_b(c, 0x8011d3e0);
    if p { crate::prof::exit(c, 0x8011d3e0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d3e0_at(c: &mut Cpu, pc: u32) { f_8011d3e0_b(c, pc) }
#[inline(always)]
fn f_8011d3e0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d3e0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d3e0, pc); }
        match pc {
            0x8011d3e0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d3f0; continue; }
                pc = 0x8011d3e8;
            }
            0x8011d3e8 => {
                c.r[3] = 0x0;
                pc = 0x8011d3f4; continue;
                pc = 0x8011d3f0;
            }
            0x8011d3f0 => {
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011d3f4;
            }
            0x8011d3f4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d3fc;
            }
            0x8011d3fc => {
                c.sth(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d3e0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetBasisPhyDef (0x8011d404, 36 bytes)
#[inline(never)]
pub fn f_8011d404(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d404) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d404); p = true; }
    }
    f_8011d404_b(c, 0x8011d404);
    if p { crate::prof::exit(c, 0x8011d404); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d404_at(c: &mut Cpu, pc: u32) { f_8011d404_b(c, pc) }
#[inline(always)]
fn f_8011d404_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d404;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d404, pc); }
        match pc {
            0x8011d404 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d414; continue; }
                pc = 0x8011d40c;
            }
            0x8011d40c => {
                c.r[3] = 0x0;
                pc = 0x8011d418; continue;
                pc = 0x8011d414;
            }
            0x8011d414 => {
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011d418;
            }
            0x8011d418 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d420;
            }
            0x8011d420 => {
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d404: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetBasisPhyAtk (0x8011d428, 36 bytes)
#[inline(never)]
pub fn f_8011d428(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d428) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d428); p = true; }
    }
    f_8011d428_b(c, 0x8011d428);
    if p { crate::prof::exit(c, 0x8011d428); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d428_at(c: &mut Cpu, pc: u32) { f_8011d428_b(c, pc) }
#[inline(always)]
fn f_8011d428_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d428;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d428, pc); }
        match pc {
            0x8011d428 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d438; continue; }
                pc = 0x8011d430;
            }
            0x8011d430 => {
                c.r[3] = 0x0;
                pc = 0x8011d43c; continue;
                pc = 0x8011d438;
            }
            0x8011d438 => {
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011d43c;
            }
            0x8011d43c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d444;
            }
            0x8011d444 => {
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d428: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetBasisMaxHp (0x8011d44c, 36 bytes)
#[inline(never)]
pub fn f_8011d44c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d44c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d44c); p = true; }
    }
    f_8011d44c_b(c, 0x8011d44c);
    if p { crate::prof::exit(c, 0x8011d44c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d44c_at(c: &mut Cpu, pc: u32) { f_8011d44c_b(c, pc) }
#[inline(always)]
fn f_8011d44c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d44c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d44c, pc); }
        match pc {
            0x8011d44c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d45c; continue; }
                pc = 0x8011d454;
            }
            0x8011d454 => {
                c.r[3] = 0x0;
                pc = 0x8011d460; continue;
                pc = 0x8011d45c;
            }
            0x8011d45c => {
                c.r[3] = c.r[3].wrapping_add(0x84);
                pc = 0x8011d460;
            }
            0x8011d460 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d468;
            }
            0x8011d468 => {
                c.sth(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d44c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonDataBiosSetName (0x8011d470, 16 bytes)
#[inline(never)]
pub fn f_8011d470(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d470) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d470); p = true; }
    }
    f_8011d470_b(c, 0x8011d470);
    if p { crate::prof::exit(c, 0x8011d470); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d470_at(c: &mut Cpu, pc: u32) { f_8011d470_b(c, pc) }
#[inline(always)]
fn f_8011d470_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d470;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d470, pc); }
        match pc {
            0x8011d470 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d478;
            }
            0x8011d478 => {
                c.stw(c.r[3].wrapping_add(0x18), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d470: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetEventGetFlag (0x8011d480, 20 bytes)
#[inline(never)]
pub fn f_8011d480(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d480) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d480); p = true; }
    }
    f_8011d480_b(c, 0x8011d480);
    if p { crate::prof::exit(c, 0x8011d480); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d480_at(c: &mut Cpu, pc: u32) { f_8011d480_b(c, pc) }
#[inline(always)]
fn f_8011d480_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d480;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d480, pc); }
        match pc {
            0x8011d480 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d488;
            }
            0x8011d488 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xff;
                c.stw(c.r[3].wrapping_add(0xf8), c.r[0]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d480: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetFightTrainerPokemonDataId (0x8011d494, 16 bytes)
#[inline(never)]
pub fn f_8011d494(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d494) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d494); p = true; }
    }
    f_8011d494_b(c, 0x8011d494);
    if p { crate::prof::exit(c, 0x8011d494); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d494_at(c: &mut Cpu, pc: u32) { f_8011d494_b(c, pc) }
#[inline(always)]
fn f_8011d494_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d494;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d494, pc); }
        match pc {
            0x8011d494 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d49c;
            }
            0x8011d49c => {
                c.sth(c.r[3].wrapping_add(0xd6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d494: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPara1Amari (0x8011d4a4, 16 bytes)
#[inline(never)]
pub fn f_8011d4a4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d4a4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d4a4); p = true; }
    }
    f_8011d4a4_b(c, 0x8011d4a4);
    if p { crate::prof::exit(c, 0x8011d4a4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d4a4_at(c: &mut Cpu, pc: u32) { f_8011d4a4_b(c, pc) }
#[inline(always)]
fn f_8011d4a4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d4a4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d4a4, pc); }
        match pc {
            0x8011d4a4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d4ac;
            }
            0x8011d4ac => {
                c.sth(c.r[3].wrapping_add(0xd2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d4a4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetAmari (0x8011d4b4, 16 bytes)
#[inline(never)]
pub fn f_8011d4b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d4b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d4b4); p = true; }
    }
    f_8011d4b4_b(c, 0x8011d4b4);
    if p { crate::prof::exit(c, 0x8011d4b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d4b4_at(c: &mut Cpu, pc: u32) { f_8011d4b4_b(c, pc) }
#[inline(always)]
fn f_8011d4b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d4b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d4b4, pc); }
        match pc {
            0x8011d4b4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d4bc;
            }
            0x8011d4bc => {
                c.sth(c.r[3].wrapping_add(0xd4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d4b4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetMailId (0x8011d4c4, 16 bytes)
#[inline(never)]
pub fn f_8011d4c4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d4c4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d4c4); p = true; }
    }
    f_8011d4c4_b(c, 0x8011d4c4);
    if p { crate::prof::exit(c, 0x8011d4c4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d4c4_at(c: &mut Cpu, pc: u32) { f_8011d4c4_b(c, pc) }
#[inline(always)]
fn f_8011d4c4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d4c4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d4c4, pc); }
        match pc {
            0x8011d4c4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d4cc;
            }
            0x8011d4cc => {
                c.stb(c.r[3].wrapping_add(0xd0), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d4c4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPcboxMark (0x8011d4d4, 16 bytes)
#[inline(never)]
pub fn f_8011d4d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d4d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d4d4); p = true; }
    }
    f_8011d4d4_b(c, 0x8011d4d4);
    if p { crate::prof::exit(c, 0x8011d4d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d4d4_at(c: &mut Cpu, pc: u32) { f_8011d4d4_b(c, pc) }
#[inline(always)]
fn f_8011d4d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d4d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d4d4, pc); }
        match pc {
            0x8011d4d4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d4dc;
            }
            0x8011d4dc => {
                c.stb(c.r[3].wrapping_add(0xcf), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d4d4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetFlagAmari (0x8011d4e4, 16 bytes)
#[inline(never)]
pub fn f_8011d4e4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d4e4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d4e4); p = true; }
    }
    f_8011d4e4_b(c, 0x8011d4e4);
    if p { crate::prof::exit(c, 0x8011d4e4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d4e4_at(c: &mut Cpu, pc: u32) { f_8011d4e4_b(c, pc) }
#[inline(always)]
fn f_8011d4e4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d4e4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d4e4, pc); }
        match pc {
            0x8011d4e4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d4ec;
            }
            0x8011d4ec => {
                c.stb(c.r[3].wrapping_add(0xce), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d4e4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetFuseiFlag (0x8011d4f4, 16 bytes)
#[inline(never)]
pub fn f_8011d4f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d4f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d4f4); p = true; }
    }
    f_8011d4f4_b(c, 0x8011d4f4);
    if p { crate::prof::exit(c, 0x8011d4f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d4f4_at(c: &mut Cpu, pc: u32) { f_8011d4f4_b(c, pc) }
#[inline(always)]
fn f_8011d4f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d4f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d4f4, pc); }
        match pc {
            0x8011d4f4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d4fc;
            }
            0x8011d4fc => {
                c.stb(c.r[3].wrapping_add(0xcd), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d4f4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetTokuseiFlag (0x8011d504, 104 bytes)
#[inline(never)]
pub fn f_8011d504(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d504) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d504); p = true; }
    }
    f_8011d504_b(c, 0x8011d504);
    if p { crate::prof::exit(c, 0x8011d504); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d504_at(c: &mut Cpu, pc: u32) { f_8011d504_b(c, pc) }
#[inline(always)]
fn f_8011d504_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d504;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d504, pc); }
        match pc {
            0x8011d504 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d50c;
            }
            0x8011d50c => {
                if !c.crbit(2) { pc = 0x8011d518; continue; }
                pc = 0x8011d510;
            }
            0x8011d510 => {
                c.r[0] = 0x0;
                pc = 0x8011d51c; continue;
                pc = 0x8011d518;
            }
            0x8011d518 => {
                c.r[0] = c.lhz(c.r[3]);
                pc = 0x8011d51c;
            }
            0x8011d51c => {
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff8770));
                c.r[6] = c.r[0].rotate_left(0) & 0xffff;
                c.r[0] = c.lwz(c.r[5]);
                c.cmpu(0, c.r[6], c.r[0]);
                if c.crbit(0) { pc = 0x8011d538; continue; }
                pc = 0x8011d530;
            }
            0x8011d530 => {
                c.r[5] = 0x0;
                pc = 0x8011d544; continue;
                pc = 0x8011d538;
            }
            0x8011d538 => {
                c.r[0] = c.r[6].wrapping_mul(0x11c);
                c.r[5] = c.lwz(c.r[13].wrapping_add(0xffff8774));
                c.r[5] = c.r[5].wrapping_add(c.r[0]);
                pc = 0x8011d544;
            }
            0x8011d544 => {
                c.cmpu(0, c.r[5], 0x0);
                if !c.crbit(2) { pc = 0x8011d554; continue; }
                pc = 0x8011d54c;
            }
            0x8011d54c => {
                c.r[0] = 0x0;
                pc = 0x8011d558; continue;
                pc = 0x8011d554;
            }
            0x8011d554 => {
                c.r[0] = c.lbz(c.r[5].wrapping_add(0x33));
                pc = 0x8011d558;
            }
            0x8011d558 => {
                c.r[0] = c.r[0].rotate_left(0) & 0xff;
                c.rc(c.r[0]);
                if !c.crbit(2) { pc = 0x8011d564; continue; }
                pc = 0x8011d560;
            }
            0x8011d560 => {
                c.r[4] = 0x0;
                pc = 0x8011d564;
            }
            0x8011d564 => {
                c.stb(c.r[3].wrapping_add(0xcc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d504: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetTamagoFlag (0x8011d56c, 16 bytes)
#[inline(never)]
pub fn f_8011d56c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d56c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d56c); p = true; }
    }
    f_8011d56c_b(c, 0x8011d56c);
    if p { crate::prof::exit(c, 0x8011d56c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d56c_at(c: &mut Cpu, pc: u32) { f_8011d56c_b(c, pc) }
#[inline(always)]
fn f_8011d56c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d56c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d56c, pc); }
        match pc {
            0x8011d56c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d574;
            }
            0x8011d574 => {
                c.stb(c.r[3].wrapping_add(0xcb), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d56c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPokerus (0x8011d57c, 16 bytes)
#[inline(never)]
pub fn f_8011d57c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d57c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d57c); p = true; }
    }
    f_8011d57c_b(c, 0x8011d57c);
    if p { crate::prof::exit(c, 0x8011d57c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d57c_at(c: &mut Cpu, pc: u32) { f_8011d57c_b(c, pc) }
#[inline(always)]
fn f_8011d57c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d57c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d57c, pc); }
        match pc {
            0x8011d57c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d584;
            }
            0x8011d584 => {
                c.stb(c.r[3].wrapping_add(0xca), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d57c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetAmariRibbon (0x8011d58c, 36 bytes)
#[inline(never)]
pub fn f_8011d58c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d58c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d58c); p = true; }
    }
    f_8011d58c_b(c, 0x8011d58c);
    if p { crate::prof::exit(c, 0x8011d58c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d58c_at(c: &mut Cpu, pc: u32) { f_8011d58c_b(c, pc) }
#[inline(always)]
fn f_8011d58c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d58c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d58c, pc); }
        match pc {
            0x8011d58c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d59c; continue; }
                pc = 0x8011d594;
            }
            0x8011d594 => {
                c.r[3] = 0x0;
                pc = 0x8011d5a0; continue;
                pc = 0x8011d59c;
            }
            0x8011d59c => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d5a0;
            }
            0x8011d5a0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d5a8;
            }
            0x8011d5a8 => {
                c.stb(c.r[3].wrapping_add(0xc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d58c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetWorldRibbon (0x8011d5b0, 36 bytes)
#[inline(never)]
pub fn f_8011d5b0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d5b0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d5b0); p = true; }
    }
    f_8011d5b0_b(c, 0x8011d5b0);
    if p { crate::prof::exit(c, 0x8011d5b0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d5b0_at(c: &mut Cpu, pc: u32) { f_8011d5b0_b(c, pc) }
#[inline(always)]
fn f_8011d5b0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d5b0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d5b0, pc); }
        match pc {
            0x8011d5b0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d5c0; continue; }
                pc = 0x8011d5b8;
            }
            0x8011d5b8 => {
                c.r[3] = 0x0;
                pc = 0x8011d5c4; continue;
                pc = 0x8011d5c0;
            }
            0x8011d5c0 => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d5c4;
            }
            0x8011d5c4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d5cc;
            }
            0x8011d5cc => {
                c.stb(c.r[3].wrapping_add(0xb), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d5b0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetEarthRibbon (0x8011d5d4, 36 bytes)
#[inline(never)]
pub fn f_8011d5d4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d5d4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d5d4); p = true; }
    }
    f_8011d5d4_b(c, 0x8011d5d4);
    if p { crate::prof::exit(c, 0x8011d5d4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d5d4_at(c: &mut Cpu, pc: u32) { f_8011d5d4_b(c, pc) }
#[inline(always)]
fn f_8011d5d4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d5d4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d5d4, pc); }
        match pc {
            0x8011d5d4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d5e4; continue; }
                pc = 0x8011d5dc;
            }
            0x8011d5dc => {
                c.r[3] = 0x0;
                pc = 0x8011d5e8; continue;
                pc = 0x8011d5e4;
            }
            0x8011d5e4 => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d5e8;
            }
            0x8011d5e8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d5f0;
            }
            0x8011d5f0 => {
                c.stb(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d5d4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetNationalRibbon (0x8011d5f8, 36 bytes)
#[inline(never)]
pub fn f_8011d5f8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d5f8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d5f8); p = true; }
    }
    f_8011d5f8_b(c, 0x8011d5f8);
    if p { crate::prof::exit(c, 0x8011d5f8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d5f8_at(c: &mut Cpu, pc: u32) { f_8011d5f8_b(c, pc) }
#[inline(always)]
fn f_8011d5f8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d5f8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d5f8, pc); }
        match pc {
            0x8011d5f8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d608; continue; }
                pc = 0x8011d600;
            }
            0x8011d600 => {
                c.r[3] = 0x0;
                pc = 0x8011d60c; continue;
                pc = 0x8011d608;
            }
            0x8011d608 => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d60c;
            }
            0x8011d60c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d614;
            }
            0x8011d614 => {
                c.stb(c.r[3].wrapping_add(0x9), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d5f8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetCountryRibbon (0x8011d61c, 36 bytes)
#[inline(never)]
pub fn f_8011d61c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d61c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d61c); p = true; }
    }
    f_8011d61c_b(c, 0x8011d61c);
    if p { crate::prof::exit(c, 0x8011d61c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d61c_at(c: &mut Cpu, pc: u32) { f_8011d61c_b(c, pc) }
#[inline(always)]
fn f_8011d61c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d61c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d61c, pc); }
        match pc {
            0x8011d61c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d62c; continue; }
                pc = 0x8011d624;
            }
            0x8011d624 => {
                c.r[3] = 0x0;
                pc = 0x8011d630; continue;
                pc = 0x8011d62c;
            }
            0x8011d62c => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d630;
            }
            0x8011d630 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d638;
            }
            0x8011d638 => {
                c.stb(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d61c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetSkyRibbon (0x8011d640, 36 bytes)
#[inline(never)]
pub fn f_8011d640(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d640) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d640); p = true; }
    }
    f_8011d640_b(c, 0x8011d640);
    if p { crate::prof::exit(c, 0x8011d640); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d640_at(c: &mut Cpu, pc: u32) { f_8011d640_b(c, pc) }
#[inline(always)]
fn f_8011d640_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d640;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d640, pc); }
        match pc {
            0x8011d640 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d650; continue; }
                pc = 0x8011d648;
            }
            0x8011d648 => {
                c.r[3] = 0x0;
                pc = 0x8011d654; continue;
                pc = 0x8011d650;
            }
            0x8011d650 => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d654;
            }
            0x8011d654 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d65c;
            }
            0x8011d65c => {
                c.stb(c.r[3].wrapping_add(0x7), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d640: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetLandRibbon (0x8011d664, 36 bytes)
#[inline(never)]
pub fn f_8011d664(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d664) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d664); p = true; }
    }
    f_8011d664_b(c, 0x8011d664);
    if p { crate::prof::exit(c, 0x8011d664); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d664_at(c: &mut Cpu, pc: u32) { f_8011d664_b(c, pc) }
#[inline(always)]
fn f_8011d664_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d664;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d664, pc); }
        match pc {
            0x8011d664 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d674; continue; }
                pc = 0x8011d66c;
            }
            0x8011d66c => {
                c.r[3] = 0x0;
                pc = 0x8011d678; continue;
                pc = 0x8011d674;
            }
            0x8011d674 => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d678;
            }
            0x8011d678 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d680;
            }
            0x8011d680 => {
                c.stb(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d664: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetMarineRibbon (0x8011d688, 36 bytes)
#[inline(never)]
pub fn f_8011d688(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d688) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d688); p = true; }
    }
    f_8011d688_b(c, 0x8011d688);
    if p { crate::prof::exit(c, 0x8011d688); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d688_at(c: &mut Cpu, pc: u32) { f_8011d688_b(c, pc) }
#[inline(always)]
fn f_8011d688_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d688;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d688, pc); }
        match pc {
            0x8011d688 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d698; continue; }
                pc = 0x8011d690;
            }
            0x8011d690 => {
                c.r[3] = 0x0;
                pc = 0x8011d69c; continue;
                pc = 0x8011d698;
            }
            0x8011d698 => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d69c;
            }
            0x8011d69c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d6a4;
            }
            0x8011d6a4 => {
                c.stb(c.r[3].wrapping_add(0x5), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d688: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetGanbaRibbon (0x8011d6ac, 36 bytes)
#[inline(never)]
pub fn f_8011d6ac(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d6ac) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d6ac); p = true; }
    }
    f_8011d6ac_b(c, 0x8011d6ac);
    if p { crate::prof::exit(c, 0x8011d6ac); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d6ac_at(c: &mut Cpu, pc: u32) { f_8011d6ac_b(c, pc) }
#[inline(always)]
fn f_8011d6ac_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d6ac;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d6ac, pc); }
        match pc {
            0x8011d6ac => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d6bc; continue; }
                pc = 0x8011d6b4;
            }
            0x8011d6b4 => {
                c.r[3] = 0x0;
                pc = 0x8011d6c0; continue;
                pc = 0x8011d6bc;
            }
            0x8011d6bc => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d6c0;
            }
            0x8011d6c0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d6c8;
            }
            0x8011d6c8 => {
                c.stb(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d6ac: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetBromideRibbon (0x8011d6d0, 36 bytes)
#[inline(never)]
pub fn f_8011d6d0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d6d0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d6d0); p = true; }
    }
    f_8011d6d0_b(c, 0x8011d6d0);
    if p { crate::prof::exit(c, 0x8011d6d0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d6d0_at(c: &mut Cpu, pc: u32) { f_8011d6d0_b(c, pc) }
#[inline(always)]
fn f_8011d6d0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d6d0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d6d0, pc); }
        match pc {
            0x8011d6d0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d6e0; continue; }
                pc = 0x8011d6d8;
            }
            0x8011d6d8 => {
                c.r[3] = 0x0;
                pc = 0x8011d6e4; continue;
                pc = 0x8011d6e0;
            }
            0x8011d6e0 => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d6e4;
            }
            0x8011d6e4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d6ec;
            }
            0x8011d6ec => {
                c.stb(c.r[3].wrapping_add(0x3), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d6d0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetVictoryRibbon (0x8011d6f4, 36 bytes)
#[inline(never)]
pub fn f_8011d6f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d6f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d6f4); p = true; }
    }
    f_8011d6f4_b(c, 0x8011d6f4);
    if p { crate::prof::exit(c, 0x8011d6f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d6f4_at(c: &mut Cpu, pc: u32) { f_8011d6f4_b(c, pc) }
#[inline(always)]
fn f_8011d6f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d6f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d6f4, pc); }
        match pc {
            0x8011d6f4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d704; continue; }
                pc = 0x8011d6fc;
            }
            0x8011d6fc => {
                c.r[3] = 0x0;
                pc = 0x8011d708; continue;
                pc = 0x8011d704;
            }
            0x8011d704 => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d708;
            }
            0x8011d708 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d710;
            }
            0x8011d710 => {
                c.stb(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d6f4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetWinningRibbon (0x8011d718, 36 bytes)
#[inline(never)]
pub fn f_8011d718(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d718) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d718); p = true; }
    }
    f_8011d718_b(c, 0x8011d718);
    if p { crate::prof::exit(c, 0x8011d718); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d718_at(c: &mut Cpu, pc: u32) { f_8011d718_b(c, pc) }
#[inline(always)]
fn f_8011d718_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d718;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d718, pc); }
        match pc {
            0x8011d718 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d728; continue; }
                pc = 0x8011d720;
            }
            0x8011d720 => {
                c.r[3] = 0x0;
                pc = 0x8011d72c; continue;
                pc = 0x8011d728;
            }
            0x8011d728 => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d72c;
            }
            0x8011d72c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d734;
            }
            0x8011d734 => {
                c.stb(c.r[3].wrapping_add(0x1), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d718: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetChampRibbon (0x8011d73c, 36 bytes)
#[inline(never)]
pub fn f_8011d73c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d73c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d73c); p = true; }
    }
    f_8011d73c_b(c, 0x8011d73c);
    if p { crate::prof::exit(c, 0x8011d73c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d73c_at(c: &mut Cpu, pc: u32) { f_8011d73c_b(c, pc) }
#[inline(always)]
fn f_8011d73c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d73c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d73c, pc); }
        match pc {
            0x8011d73c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d74c; continue; }
                pc = 0x8011d744;
            }
            0x8011d744 => {
                c.r[3] = 0x0;
                pc = 0x8011d750; continue;
                pc = 0x8011d74c;
            }
            0x8011d74c => {
                c.r[3] = c.r[3].wrapping_add(0xbd);
                pc = 0x8011d750;
            }
            0x8011d750 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d758;
            }
            0x8011d758 => {
                c.stb(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d73c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetFur (0x8011d760, 16 bytes)
#[inline(never)]
pub fn f_8011d760(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d760) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d760); p = true; }
    }
    f_8011d760_b(c, 0x8011d760);
    if p { crate::prof::exit(c, 0x8011d760); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d760_at(c: &mut Cpu, pc: u32) { f_8011d760_b(c, pc) }
#[inline(always)]
fn f_8011d760_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d760;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d760, pc); }
        match pc {
            0x8011d760 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d768;
            }
            0x8011d768 => {
                c.stb(c.r[3].wrapping_add(0xbc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d760: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetStrongMedal (0x8011d770, 36 bytes)
#[inline(never)]
pub fn f_8011d770(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d770) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d770); p = true; }
    }
    f_8011d770_b(c, 0x8011d770);
    if p { crate::prof::exit(c, 0x8011d770); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d770_at(c: &mut Cpu, pc: u32) { f_8011d770_b(c, pc) }
#[inline(always)]
fn f_8011d770_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d770;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d770, pc); }
        match pc {
            0x8011d770 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d780; continue; }
                pc = 0x8011d778;
            }
            0x8011d778 => {
                c.r[3] = 0x0;
                pc = 0x8011d784; continue;
                pc = 0x8011d780;
            }
            0x8011d780 => {
                c.r[3] = c.r[3].wrapping_add(0xb7);
                pc = 0x8011d784;
            }
            0x8011d784 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d78c;
            }
            0x8011d78c => {
                c.stb(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d770: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetCleverMedal (0x8011d794, 36 bytes)
#[inline(never)]
pub fn f_8011d794(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d794) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d794); p = true; }
    }
    f_8011d794_b(c, 0x8011d794);
    if p { crate::prof::exit(c, 0x8011d794); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d794_at(c: &mut Cpu, pc: u32) { f_8011d794_b(c, pc) }
#[inline(always)]
fn f_8011d794_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d794;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d794, pc); }
        match pc {
            0x8011d794 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d7a4; continue; }
                pc = 0x8011d79c;
            }
            0x8011d79c => {
                c.r[3] = 0x0;
                pc = 0x8011d7a8; continue;
                pc = 0x8011d7a4;
            }
            0x8011d7a4 => {
                c.r[3] = c.r[3].wrapping_add(0xb7);
                pc = 0x8011d7a8;
            }
            0x8011d7a8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d7b0;
            }
            0x8011d7b0 => {
                c.stb(c.r[3].wrapping_add(0x3), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d794: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetCuteMedal (0x8011d7b8, 36 bytes)
#[inline(never)]
pub fn f_8011d7b8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d7b8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d7b8); p = true; }
    }
    f_8011d7b8_b(c, 0x8011d7b8);
    if p { crate::prof::exit(c, 0x8011d7b8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d7b8_at(c: &mut Cpu, pc: u32) { f_8011d7b8_b(c, pc) }
#[inline(always)]
fn f_8011d7b8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d7b8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d7b8, pc); }
        match pc {
            0x8011d7b8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d7c8; continue; }
                pc = 0x8011d7c0;
            }
            0x8011d7c0 => {
                c.r[3] = 0x0;
                pc = 0x8011d7cc; continue;
                pc = 0x8011d7c8;
            }
            0x8011d7c8 => {
                c.r[3] = c.r[3].wrapping_add(0xb7);
                pc = 0x8011d7cc;
            }
            0x8011d7cc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d7d4;
            }
            0x8011d7d4 => {
                c.stb(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d7b8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetBeautifulMedal (0x8011d7dc, 36 bytes)
#[inline(never)]
pub fn f_8011d7dc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d7dc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d7dc); p = true; }
    }
    f_8011d7dc_b(c, 0x8011d7dc);
    if p { crate::prof::exit(c, 0x8011d7dc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d7dc_at(c: &mut Cpu, pc: u32) { f_8011d7dc_b(c, pc) }
#[inline(always)]
fn f_8011d7dc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d7dc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d7dc, pc); }
        match pc {
            0x8011d7dc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d7ec; continue; }
                pc = 0x8011d7e4;
            }
            0x8011d7e4 => {
                c.r[3] = 0x0;
                pc = 0x8011d7f0; continue;
                pc = 0x8011d7ec;
            }
            0x8011d7ec => {
                c.r[3] = c.r[3].wrapping_add(0xb7);
                pc = 0x8011d7f0;
            }
            0x8011d7f0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d7f8;
            }
            0x8011d7f8 => {
                c.stb(c.r[3].wrapping_add(0x1), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d7dc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetStyleMedal (0x8011d800, 36 bytes)
#[inline(never)]
pub fn f_8011d800(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d800) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d800); p = true; }
    }
    f_8011d800_b(c, 0x8011d800);
    if p { crate::prof::exit(c, 0x8011d800); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d800_at(c: &mut Cpu, pc: u32) { f_8011d800_b(c, pc) }
#[inline(always)]
fn f_8011d800_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d800;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d800, pc); }
        match pc {
            0x8011d800 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d810; continue; }
                pc = 0x8011d808;
            }
            0x8011d808 => {
                c.r[3] = 0x0;
                pc = 0x8011d814; continue;
                pc = 0x8011d810;
            }
            0x8011d810 => {
                c.r[3] = c.r[3].wrapping_add(0xb7);
                pc = 0x8011d814;
            }
            0x8011d814 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d81c;
            }
            0x8011d81c => {
                c.stb(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d800: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetStrong (0x8011d824, 36 bytes)
#[inline(never)]
pub fn f_8011d824(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d824) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d824); p = true; }
    }
    f_8011d824_b(c, 0x8011d824);
    if p { crate::prof::exit(c, 0x8011d824); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d824_at(c: &mut Cpu, pc: u32) { f_8011d824_b(c, pc) }
#[inline(always)]
fn f_8011d824_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d824;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d824, pc); }
        match pc {
            0x8011d824 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d834; continue; }
                pc = 0x8011d82c;
            }
            0x8011d82c => {
                c.r[3] = 0x0;
                pc = 0x8011d838; continue;
                pc = 0x8011d834;
            }
            0x8011d834 => {
                c.r[3] = c.r[3].wrapping_add(0xb2);
                pc = 0x8011d838;
            }
            0x8011d838 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d840;
            }
            0x8011d840 => {
                c.stb(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d824: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetClever (0x8011d848, 36 bytes)
#[inline(never)]
pub fn f_8011d848(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d848) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d848); p = true; }
    }
    f_8011d848_b(c, 0x8011d848);
    if p { crate::prof::exit(c, 0x8011d848); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d848_at(c: &mut Cpu, pc: u32) { f_8011d848_b(c, pc) }
#[inline(always)]
fn f_8011d848_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d848;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d848, pc); }
        match pc {
            0x8011d848 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d858; continue; }
                pc = 0x8011d850;
            }
            0x8011d850 => {
                c.r[3] = 0x0;
                pc = 0x8011d85c; continue;
                pc = 0x8011d858;
            }
            0x8011d858 => {
                c.r[3] = c.r[3].wrapping_add(0xb2);
                pc = 0x8011d85c;
            }
            0x8011d85c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d864;
            }
            0x8011d864 => {
                c.stb(c.r[3].wrapping_add(0x3), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d848: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetCute (0x8011d86c, 36 bytes)
#[inline(never)]
pub fn f_8011d86c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d86c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d86c); p = true; }
    }
    f_8011d86c_b(c, 0x8011d86c);
    if p { crate::prof::exit(c, 0x8011d86c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d86c_at(c: &mut Cpu, pc: u32) { f_8011d86c_b(c, pc) }
#[inline(always)]
fn f_8011d86c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d86c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d86c, pc); }
        match pc {
            0x8011d86c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d87c; continue; }
                pc = 0x8011d874;
            }
            0x8011d874 => {
                c.r[3] = 0x0;
                pc = 0x8011d880; continue;
                pc = 0x8011d87c;
            }
            0x8011d87c => {
                c.r[3] = c.r[3].wrapping_add(0xb2);
                pc = 0x8011d880;
            }
            0x8011d880 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d888;
            }
            0x8011d888 => {
                c.stb(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d86c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetBeautiful (0x8011d890, 36 bytes)
#[inline(never)]
pub fn f_8011d890(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d890) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d890); p = true; }
    }
    f_8011d890_b(c, 0x8011d890);
    if p { crate::prof::exit(c, 0x8011d890); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d890_at(c: &mut Cpu, pc: u32) { f_8011d890_b(c, pc) }
#[inline(always)]
fn f_8011d890_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d890;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d890, pc); }
        match pc {
            0x8011d890 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d8a0; continue; }
                pc = 0x8011d898;
            }
            0x8011d898 => {
                c.r[3] = 0x0;
                pc = 0x8011d8a4; continue;
                pc = 0x8011d8a0;
            }
            0x8011d8a0 => {
                c.r[3] = c.r[3].wrapping_add(0xb2);
                pc = 0x8011d8a4;
            }
            0x8011d8a4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d8ac;
            }
            0x8011d8ac => {
                c.stb(c.r[3].wrapping_add(0x1), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d890: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetStyle (0x8011d8b4, 36 bytes)
#[inline(never)]
pub fn f_8011d8b4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d8b4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d8b4); p = true; }
    }
    f_8011d8b4_b(c, 0x8011d8b4);
    if p { crate::prof::exit(c, 0x8011d8b4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d8b4_at(c: &mut Cpu, pc: u32) { f_8011d8b4_b(c, pc) }
#[inline(always)]
fn f_8011d8b4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d8b4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d8b4, pc); }
        match pc {
            0x8011d8b4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d8c4; continue; }
                pc = 0x8011d8bc;
            }
            0x8011d8bc => {
                c.r[3] = 0x0;
                pc = 0x8011d8c8; continue;
                pc = 0x8011d8c4;
            }
            0x8011d8c4 => {
                c.r[3] = c.r[3].wrapping_add(0xb2);
                pc = 0x8011d8c8;
            }
            0x8011d8c8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d8d0;
            }
            0x8011d8d0 => {
                c.stb(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d8b4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetDp (0x8011d8d8, 28 bytes)
#[inline(never)]
pub fn f_8011d8d8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d8d8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d8d8); p = true; }
    }
    f_8011d8d8_b(c, 0x8011d8d8);
    if p { crate::prof::exit(c, 0x8011d8d8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d8d8_at(c: &mut Cpu, pc: u32) { f_8011d8d8_b(c, pc) }
#[inline(always)]
fn f_8011d8d8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d8d8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d8d8, pc); }
        match pc {
            0x8011d8d8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d8e0;
            }
            0x8011d8e0 => {
                c.cmps(0, c.r[4], 0x639c);
                if c.crbit(0) { pc = 0x8011d8ec; continue; }
                pc = 0x8011d8e8;
            }
            0x8011d8e8 => {
                c.r[4] = 0x639c;
                pc = 0x8011d8ec;
            }
            0x8011d8ec => {
                c.stw(c.r[3].wrapping_add(0xdc), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d8d8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetDarkpokemonDataId (0x8011d8f4, 16 bytes)
#[inline(never)]
pub fn f_8011d8f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d8f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d8f4); p = true; }
    }
    f_8011d8f4_b(c, 0x8011d8f4);
    if p { crate::prof::exit(c, 0x8011d8f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d8f4_at(c: &mut Cpu, pc: u32) { f_8011d8f4_b(c, pc) }
#[inline(always)]
fn f_8011d8f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d8f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d8f4, pc); }
        match pc {
            0x8011d8f4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d8fc;
            }
            0x8011d8fc => {
                c.sth(c.r[3].wrapping_add(0xd8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d8f4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetFriend (0x8011d904, 32 bytes)
#[inline(never)]
pub fn f_8011d904(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d904) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d904); p = true; }
    }
    f_8011d904_b(c, 0x8011d904);
    if p { crate::prof::exit(c, 0x8011d904); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d904_at(c: &mut Cpu, pc: u32) { f_8011d904_b(c, pc) }
#[inline(always)]
fn f_8011d904_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d904;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d904, pc); }
        match pc {
            0x8011d904 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d90c;
            }
            0x8011d90c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xff);
                if !c.crbit(1) { pc = 0x8011d91c; continue; }
                pc = 0x8011d918;
            }
            0x8011d918 => {
                c.r[4] = 0xff;
                pc = 0x8011d91c;
            }
            0x8011d91c => {
                c.sth(c.r[3].wrapping_add(0xb0), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d904: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetNimblenessRnd (0x8011d924, 52 bytes)
#[inline(never)]
pub fn f_8011d924(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d924) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d924); p = true; }
    }
    f_8011d924_b(c, 0x8011d924);
    if p { crate::prof::exit(c, 0x8011d924); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d924_at(c: &mut Cpu, pc: u32) { f_8011d924_b(c, pc) }
#[inline(always)]
fn f_8011d924_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d924;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d924, pc); }
        match pc {
            0x8011d924 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d934; continue; }
                pc = 0x8011d92c;
            }
            0x8011d92c => {
                c.r[3] = 0x0;
                pc = 0x8011d938; continue;
                pc = 0x8011d934;
            }
            0x8011d934 => {
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011d938;
            }
            0x8011d938 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d940;
            }
            0x8011d940 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1f);
                if !c.crbit(1) { pc = 0x8011d950; continue; }
                pc = 0x8011d94c;
            }
            0x8011d94c => {
                c.r[4] = 0x1f;
                pc = 0x8011d950;
            }
            0x8011d950 => {
                c.sth(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d924: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetSpeDefRnd (0x8011d958, 52 bytes)
#[inline(never)]
pub fn f_8011d958(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d958) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d958); p = true; }
    }
    f_8011d958_b(c, 0x8011d958);
    if p { crate::prof::exit(c, 0x8011d958); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d958_at(c: &mut Cpu, pc: u32) { f_8011d958_b(c, pc) }
#[inline(always)]
fn f_8011d958_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d958;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d958, pc); }
        match pc {
            0x8011d958 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d968; continue; }
                pc = 0x8011d960;
            }
            0x8011d960 => {
                c.r[3] = 0x0;
                pc = 0x8011d96c; continue;
                pc = 0x8011d968;
            }
            0x8011d968 => {
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011d96c;
            }
            0x8011d96c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d974;
            }
            0x8011d974 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1f);
                if !c.crbit(1) { pc = 0x8011d984; continue; }
                pc = 0x8011d980;
            }
            0x8011d980 => {
                c.r[4] = 0x1f;
                pc = 0x8011d984;
            }
            0x8011d984 => {
                c.sth(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d958: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetSpeAtkRnd (0x8011d98c, 52 bytes)
#[inline(never)]
pub fn f_8011d98c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d98c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d98c); p = true; }
    }
    f_8011d98c_b(c, 0x8011d98c);
    if p { crate::prof::exit(c, 0x8011d98c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d98c_at(c: &mut Cpu, pc: u32) { f_8011d98c_b(c, pc) }
#[inline(always)]
fn f_8011d98c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d98c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d98c, pc); }
        match pc {
            0x8011d98c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d99c; continue; }
                pc = 0x8011d994;
            }
            0x8011d994 => {
                c.r[3] = 0x0;
                pc = 0x8011d9a0; continue;
                pc = 0x8011d99c;
            }
            0x8011d99c => {
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011d9a0;
            }
            0x8011d9a0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d9a8;
            }
            0x8011d9a8 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1f);
                if !c.crbit(1) { pc = 0x8011d9b8; continue; }
                pc = 0x8011d9b4;
            }
            0x8011d9b4 => {
                c.r[4] = 0x1f;
                pc = 0x8011d9b8;
            }
            0x8011d9b8 => {
                c.sth(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d98c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPhyDefRnd (0x8011d9c0, 52 bytes)
#[inline(never)]
pub fn f_8011d9c0(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d9c0) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d9c0); p = true; }
    }
    f_8011d9c0_b(c, 0x8011d9c0);
    if p { crate::prof::exit(c, 0x8011d9c0); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d9c0_at(c: &mut Cpu, pc: u32) { f_8011d9c0_b(c, pc) }
#[inline(always)]
fn f_8011d9c0_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d9c0;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d9c0, pc); }
        match pc {
            0x8011d9c0 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011d9d0; continue; }
                pc = 0x8011d9c8;
            }
            0x8011d9c8 => {
                c.r[3] = 0x0;
                pc = 0x8011d9d4; continue;
                pc = 0x8011d9d0;
            }
            0x8011d9d0 => {
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011d9d4;
            }
            0x8011d9d4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011d9dc;
            }
            0x8011d9dc => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1f);
                if !c.crbit(1) { pc = 0x8011d9ec; continue; }
                pc = 0x8011d9e8;
            }
            0x8011d9e8 => {
                c.r[4] = 0x1f;
                pc = 0x8011d9ec;
            }
            0x8011d9ec => {
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d9c0: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPhyAtkRnd (0x8011d9f4, 52 bytes)
#[inline(never)]
pub fn f_8011d9f4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011d9f4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011d9f4); p = true; }
    }
    f_8011d9f4_b(c, 0x8011d9f4);
    if p { crate::prof::exit(c, 0x8011d9f4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011d9f4_at(c: &mut Cpu, pc: u32) { f_8011d9f4_b(c, pc) }
#[inline(always)]
fn f_8011d9f4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011d9f4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011d9f4, pc); }
        match pc {
            0x8011d9f4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011da04; continue; }
                pc = 0x8011d9fc;
            }
            0x8011d9fc => {
                c.r[3] = 0x0;
                pc = 0x8011da08; continue;
                pc = 0x8011da04;
            }
            0x8011da04 => {
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011da08;
            }
            0x8011da08 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011da10;
            }
            0x8011da10 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1f);
                if !c.crbit(1) { pc = 0x8011da20; continue; }
                pc = 0x8011da1c;
            }
            0x8011da1c => {
                c.r[4] = 0x1f;
                pc = 0x8011da20;
            }
            0x8011da20 => {
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011d9f4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetMaxHpRnd (0x8011da28, 52 bytes)
#[inline(never)]
pub fn f_8011da28(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011da28) { return; }
        if c.prof { crate::prof::enter(c, 0x8011da28); p = true; }
    }
    f_8011da28_b(c, 0x8011da28);
    if p { crate::prof::exit(c, 0x8011da28); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011da28_at(c: &mut Cpu, pc: u32) { f_8011da28_b(c, pc) }
#[inline(always)]
fn f_8011da28_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011da28;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011da28, pc); }
        match pc {
            0x8011da28 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011da38; continue; }
                pc = 0x8011da30;
            }
            0x8011da30 => {
                c.r[3] = 0x0;
                pc = 0x8011da3c; continue;
                pc = 0x8011da38;
            }
            0x8011da38 => {
                c.r[3] = c.r[3].wrapping_add(0xa4);
                pc = 0x8011da3c;
            }
            0x8011da3c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011da44;
            }
            0x8011da44 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0x1f);
                if !c.crbit(1) { pc = 0x8011da54; continue; }
                pc = 0x8011da50;
            }
            0x8011da50 => {
                c.r[4] = 0x1f;
                pc = 0x8011da54;
            }
            0x8011da54 => {
                c.sth(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011da28: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetNimblenessEffort (0x8011da5c, 52 bytes)
#[inline(never)]
pub fn f_8011da5c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011da5c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011da5c); p = true; }
    }
    f_8011da5c_b(c, 0x8011da5c);
    if p { crate::prof::exit(c, 0x8011da5c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011da5c_at(c: &mut Cpu, pc: u32) { f_8011da5c_b(c, pc) }
#[inline(always)]
fn f_8011da5c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011da5c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011da5c, pc); }
        match pc {
            0x8011da5c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011da6c; continue; }
                pc = 0x8011da64;
            }
            0x8011da64 => {
                c.r[3] = 0x0;
                pc = 0x8011da70; continue;
                pc = 0x8011da6c;
            }
            0x8011da6c => {
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011da70;
            }
            0x8011da70 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011da78;
            }
            0x8011da78 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xff);
                if !c.crbit(1) { pc = 0x8011da88; continue; }
                pc = 0x8011da84;
            }
            0x8011da84 => {
                c.r[4] = 0xff;
                pc = 0x8011da88;
            }
            0x8011da88 => {
                c.sth(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011da5c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetSpeDefEffort (0x8011da90, 52 bytes)
#[inline(never)]
pub fn f_8011da90(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011da90) { return; }
        if c.prof { crate::prof::enter(c, 0x8011da90); p = true; }
    }
    f_8011da90_b(c, 0x8011da90);
    if p { crate::prof::exit(c, 0x8011da90); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011da90_at(c: &mut Cpu, pc: u32) { f_8011da90_b(c, pc) }
#[inline(always)]
fn f_8011da90_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011da90;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011da90, pc); }
        match pc {
            0x8011da90 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011daa0; continue; }
                pc = 0x8011da98;
            }
            0x8011da98 => {
                c.r[3] = 0x0;
                pc = 0x8011daa4; continue;
                pc = 0x8011daa0;
            }
            0x8011daa0 => {
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011daa4;
            }
            0x8011daa4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011daac;
            }
            0x8011daac => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xff);
                if !c.crbit(1) { pc = 0x8011dabc; continue; }
                pc = 0x8011dab8;
            }
            0x8011dab8 => {
                c.r[4] = 0xff;
                pc = 0x8011dabc;
            }
            0x8011dabc => {
                c.sth(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011da90: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetSpeAtkEffort (0x8011dac4, 52 bytes)
#[inline(never)]
pub fn f_8011dac4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dac4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dac4); p = true; }
    }
    f_8011dac4_b(c, 0x8011dac4);
    if p { crate::prof::exit(c, 0x8011dac4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dac4_at(c: &mut Cpu, pc: u32) { f_8011dac4_b(c, pc) }
#[inline(always)]
fn f_8011dac4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011dac4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011dac4, pc); }
        match pc {
            0x8011dac4 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011dad4; continue; }
                pc = 0x8011dacc;
            }
            0x8011dacc => {
                c.r[3] = 0x0;
                pc = 0x8011dad8; continue;
                pc = 0x8011dad4;
            }
            0x8011dad4 => {
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011dad8;
            }
            0x8011dad8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011dae0;
            }
            0x8011dae0 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xff);
                if !c.crbit(1) { pc = 0x8011daf0; continue; }
                pc = 0x8011daec;
            }
            0x8011daec => {
                c.r[4] = 0xff;
                pc = 0x8011daf0;
            }
            0x8011daf0 => {
                c.sth(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011dac4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPhyDefEffort (0x8011daf8, 52 bytes)
#[inline(never)]
pub fn f_8011daf8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011daf8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011daf8); p = true; }
    }
    f_8011daf8_b(c, 0x8011daf8);
    if p { crate::prof::exit(c, 0x8011daf8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011daf8_at(c: &mut Cpu, pc: u32) { f_8011daf8_b(c, pc) }
#[inline(always)]
fn f_8011daf8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011daf8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011daf8, pc); }
        match pc {
            0x8011daf8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011db08; continue; }
                pc = 0x8011db00;
            }
            0x8011db00 => {
                c.r[3] = 0x0;
                pc = 0x8011db0c; continue;
                pc = 0x8011db08;
            }
            0x8011db08 => {
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011db0c;
            }
            0x8011db0c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011db14;
            }
            0x8011db14 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xff);
                if !c.crbit(1) { pc = 0x8011db24; continue; }
                pc = 0x8011db20;
            }
            0x8011db20 => {
                c.r[4] = 0xff;
                pc = 0x8011db24;
            }
            0x8011db24 => {
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011daf8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPhyAtkEffort (0x8011db2c, 52 bytes)
#[inline(never)]
pub fn f_8011db2c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011db2c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011db2c); p = true; }
    }
    f_8011db2c_b(c, 0x8011db2c);
    if p { crate::prof::exit(c, 0x8011db2c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011db2c_at(c: &mut Cpu, pc: u32) { f_8011db2c_b(c, pc) }
#[inline(always)]
fn f_8011db2c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011db2c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011db2c, pc); }
        match pc {
            0x8011db2c => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011db3c; continue; }
                pc = 0x8011db34;
            }
            0x8011db34 => {
                c.r[3] = 0x0;
                pc = 0x8011db40; continue;
                pc = 0x8011db3c;
            }
            0x8011db3c => {
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011db40;
            }
            0x8011db40 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011db48;
            }
            0x8011db48 => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xff);
                if !c.crbit(1) { pc = 0x8011db58; continue; }
                pc = 0x8011db54;
            }
            0x8011db54 => {
                c.r[4] = 0xff;
                pc = 0x8011db58;
            }
            0x8011db58 => {
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011db2c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetMaxHpEffort (0x8011db60, 52 bytes)
#[inline(never)]
pub fn f_8011db60(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011db60) { return; }
        if c.prof { crate::prof::enter(c, 0x8011db60); p = true; }
    }
    f_8011db60_b(c, 0x8011db60);
    if p { crate::prof::exit(c, 0x8011db60); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011db60_at(c: &mut Cpu, pc: u32) { f_8011db60_b(c, pc) }
#[inline(always)]
fn f_8011db60_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011db60;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011db60, pc); }
        match pc {
            0x8011db60 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011db70; continue; }
                pc = 0x8011db68;
            }
            0x8011db68 => {
                c.r[3] = 0x0;
                pc = 0x8011db74; continue;
                pc = 0x8011db70;
            }
            0x8011db70 => {
                c.r[3] = c.r[3].wrapping_add(0x98);
                pc = 0x8011db74;
            }
            0x8011db74 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011db7c;
            }
            0x8011db7c => {
                c.r[0] = c.r[4].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[0], 0xff);
                if !c.crbit(1) { pc = 0x8011db8c; continue; }
                pc = 0x8011db88;
            }
            0x8011db88 => {
                c.r[4] = 0xff;
                pc = 0x8011db8c;
            }
            0x8011db8c => {
                c.sth(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011db60: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetNimbleness (0x8011db94, 36 bytes)
#[inline(never)]
pub fn f_8011db94(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011db94) { return; }
        if c.prof { crate::prof::enter(c, 0x8011db94); p = true; }
    }
    f_8011db94_b(c, 0x8011db94);
    if p { crate::prof::exit(c, 0x8011db94); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011db94_at(c: &mut Cpu, pc: u32) { f_8011db94_b(c, pc) }
#[inline(always)]
fn f_8011db94_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011db94;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011db94, pc); }
        match pc {
            0x8011db94 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011dba4; continue; }
                pc = 0x8011db9c;
            }
            0x8011db9c => {
                c.r[3] = 0x0;
                pc = 0x8011dba8; continue;
                pc = 0x8011dba4;
            }
            0x8011dba4 => {
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011dba8;
            }
            0x8011dba8 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011dbb0;
            }
            0x8011dbb0 => {
                c.sth(c.r[3].wrapping_add(0xa), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011db94: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetSpeDef (0x8011dbb8, 36 bytes)
#[inline(never)]
pub fn f_8011dbb8(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dbb8) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dbb8); p = true; }
    }
    f_8011dbb8_b(c, 0x8011dbb8);
    if p { crate::prof::exit(c, 0x8011dbb8); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dbb8_at(c: &mut Cpu, pc: u32) { f_8011dbb8_b(c, pc) }
#[inline(always)]
fn f_8011dbb8_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011dbb8;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011dbb8, pc); }
        match pc {
            0x8011dbb8 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011dbc8; continue; }
                pc = 0x8011dbc0;
            }
            0x8011dbc0 => {
                c.r[3] = 0x0;
                pc = 0x8011dbcc; continue;
                pc = 0x8011dbc8;
            }
            0x8011dbc8 => {
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011dbcc;
            }
            0x8011dbcc => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011dbd4;
            }
            0x8011dbd4 => {
                c.sth(c.r[3].wrapping_add(0x8), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011dbb8: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetSpeAtk (0x8011dbdc, 36 bytes)
#[inline(never)]
pub fn f_8011dbdc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dbdc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dbdc); p = true; }
    }
    f_8011dbdc_b(c, 0x8011dbdc);
    if p { crate::prof::exit(c, 0x8011dbdc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dbdc_at(c: &mut Cpu, pc: u32) { f_8011dbdc_b(c, pc) }
#[inline(always)]
fn f_8011dbdc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011dbdc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011dbdc, pc); }
        match pc {
            0x8011dbdc => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011dbec; continue; }
                pc = 0x8011dbe4;
            }
            0x8011dbe4 => {
                c.r[3] = 0x0;
                pc = 0x8011dbf0; continue;
                pc = 0x8011dbec;
            }
            0x8011dbec => {
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011dbf0;
            }
            0x8011dbf0 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011dbf8;
            }
            0x8011dbf8 => {
                c.sth(c.r[3].wrapping_add(0x6), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011dbdc: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPhyDef (0x8011dc00, 36 bytes)
#[inline(never)]
pub fn f_8011dc00(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dc00) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dc00); p = true; }
    }
    f_8011dc00_b(c, 0x8011dc00);
    if p { crate::prof::exit(c, 0x8011dc00); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dc00_at(c: &mut Cpu, pc: u32) { f_8011dc00_b(c, pc) }
#[inline(always)]
fn f_8011dc00_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011dc00;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011dc00, pc); }
        match pc {
            0x8011dc00 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011dc10; continue; }
                pc = 0x8011dc08;
            }
            0x8011dc08 => {
                c.r[3] = 0x0;
                pc = 0x8011dc14; continue;
                pc = 0x8011dc10;
            }
            0x8011dc10 => {
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011dc14;
            }
            0x8011dc14 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011dc1c;
            }
            0x8011dc1c => {
                c.sth(c.r[3].wrapping_add(0x4), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011dc00: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPhyAtk (0x8011dc24, 36 bytes)
#[inline(never)]
pub fn f_8011dc24(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dc24) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dc24); p = true; }
    }
    f_8011dc24_b(c, 0x8011dc24);
    if p { crate::prof::exit(c, 0x8011dc24); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dc24_at(c: &mut Cpu, pc: u32) { f_8011dc24_b(c, pc) }
#[inline(always)]
fn f_8011dc24_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011dc24;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011dc24, pc); }
        match pc {
            0x8011dc24 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011dc34; continue; }
                pc = 0x8011dc2c;
            }
            0x8011dc2c => {
                c.r[3] = 0x0;
                pc = 0x8011dc38; continue;
                pc = 0x8011dc34;
            }
            0x8011dc34 => {
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011dc38;
            }
            0x8011dc38 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011dc40;
            }
            0x8011dc40 => {
                c.sth(c.r[3].wrapping_add(0x2), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011dc24: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetMaxHp (0x8011dc48, 36 bytes)
#[inline(never)]
pub fn f_8011dc48(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dc48) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dc48); p = true; }
    }
    f_8011dc48_b(c, 0x8011dc48);
    if p { crate::prof::exit(c, 0x8011dc48); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dc48_at(c: &mut Cpu, pc: u32) { f_8011dc48_b(c, pc) }
#[inline(always)]
fn f_8011dc48_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011dc48;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011dc48, pc); }
        match pc {
            0x8011dc48 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011dc58; continue; }
                pc = 0x8011dc50;
            }
            0x8011dc50 => {
                c.r[3] = 0x0;
                pc = 0x8011dc5c; continue;
                pc = 0x8011dc58;
            }
            0x8011dc58 => {
                c.r[3] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011dc5c;
            }
            0x8011dc5c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011dc64;
            }
            0x8011dc64 => {
                c.sth(c.r[3], c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011dc48: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetHp (0x8011dc6c, 72 bytes)
#[inline(never)]
pub fn f_8011dc6c(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dc6c) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dc6c); p = true; }
    }
    f_8011dc6c_b(c, 0x8011dc6c);
    if p { crate::prof::exit(c, 0x8011dc6c); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dc6c_at(c: &mut Cpu, pc: u32) { f_8011dc6c_b(c, pc) }
#[inline(always)]
fn f_8011dc6c_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011dc6c;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011dc6c, pc); }
        match pc {
            0x8011dc6c => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011dc74;
            }
            0x8011dc74 => {
                c.sth(c.r[3].wrapping_add(0x8a), c.r[4]);
                if !c.crbit(2) { pc = 0x8011dc84; continue; }
                pc = 0x8011dc7c;
            }
            0x8011dc7c => {
                c.r[4] = 0x0;
                pc = 0x8011dc88; continue;
                pc = 0x8011dc84;
            }
            0x8011dc84 => {
                c.r[4] = c.r[3].wrapping_add(0x8c);
                pc = 0x8011dc88;
            }
            0x8011dc88 => {
                c.cmpu(0, c.r[4], 0x0);
                if !c.crbit(2) { pc = 0x8011dc98; continue; }
                pc = 0x8011dc90;
            }
            0x8011dc90 => {
                c.r[5] = 0x0;
                pc = 0x8011dc9c; continue;
                pc = 0x8011dc98;
            }
            0x8011dc98 => {
                c.r[5] = c.lhz(c.r[4]);
                pc = 0x8011dc9c;
            }
            0x8011dc9c => {
                c.r[0] = c.lhz(c.r[3].wrapping_add(0x8a));
                c.r[4] = c.r[5].rotate_left(0) & 0xffff;
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(0) { return; }
                pc = 0x8011dcac;
            }
            0x8011dcac => {
                c.sth(c.r[3].wrapping_add(0x8a), c.r[5]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011dc6c: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetItemDataId (0x8011dcb4, 16 bytes)
#[inline(never)]
pub fn f_8011dcb4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dcb4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dcb4); p = true; }
    }
    f_8011dcb4_b(c, 0x8011dcb4);
    if p { crate::prof::exit(c, 0x8011dcb4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dcb4_at(c: &mut Cpu, pc: u32) { f_8011dcb4_b(c, pc) }
#[inline(always)]
fn f_8011dcb4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011dcb4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011dcb4, pc); }
        match pc {
            0x8011dcb4 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { return; }
                pc = 0x8011dcbc;
            }
            0x8011dcbc => {
                c.sth(c.r[3].wrapping_add(0x88), c.r[4]);
                if true { return; }
                return;
            }
            _ => panic!("f_8011dcb4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPokemonWazaPpCount (0x8011dcc4, 188 bytes)
#[inline(never)]
pub fn f_8011dcc4(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dcc4) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dcc4); p = true; }
    }
    f_8011dcc4_b(c, 0x8011dcc4);
    if p { crate::prof::exit(c, 0x8011dcc4); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dcc4_at(c: &mut Cpu, pc: u32) { f_8011dcc4_b(c, pc) }
#[inline(always)]
fn f_8011dcc4_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011dcc4;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011dcc4, pc); }
        match pc {
            0x8011dcc4 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8011dcf4; f_8011f260(c);
                pc = 0x8011dcf4;
            }
            0x8011dcf4 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x8011dd60; continue; }
                pc = 0x8011dcfc;
            }
            0x8011dcfc => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x3);
                if !c.crbit(1) { pc = 0x8011dd0c; continue; }
                pc = 0x8011dd08;
            }
            0x8011dd08 => {
                c.r[30] = 0x3;
                pc = 0x8011dd0c;
            }
            0x8011dd0c => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8011dd18; f_80123cd4(c);
                pc = 0x8011dd18;
            }
            0x8011dd18 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x1);
                if !c.crbit(2) { pc = 0x8011dd5c; continue; }
                pc = 0x8011dd24;
            }
            0x8011dd24 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.r[5] = 0x1;
                c.lr = 0x8011dd34; f_8011f260(c);
                pc = 0x8011dd34;
            }
            0x8011dd34 => {
                c.cmpu(0, c.r[3], 0x0);
                if !c.crbit(2) { pc = 0x8011dd44; continue; }
                pc = 0x8011dd3c;
            }
            0x8011dd3c => {
                c.r[3] = 0x0;
                pc = 0x8011dd48; continue;
                pc = 0x8011dd44;
            }
            0x8011dd44 => {
                c.r[3] = c.lhz(c.r[3]);
                pc = 0x8011dd48;
            }
            0x8011dd48 => {
                c.lr = 0x8011dd4c; f_8011ca34(c);
                pc = 0x8011dd4c;
            }
            0x8011dd4c => {
                c.lr = 0x8011dd50; f_8011ca04(c);
                pc = 0x8011dd50;
            }
            0x8011dd50 => {
                c.r[0] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[0], 0x4);
                if !c.crbit(1) { pc = 0x8011dd60; continue; }
                pc = 0x8011dd5c;
            }
            0x8011dd5c => {
                c.stb(c.r[31].wrapping_add(0x3), c.r[30]);
                pc = 0x8011dd60;
            }
            0x8011dd60 => {
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
            _ => panic!("f_8011dcc4: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPokemonWazaPp (0x8011dd80, 124 bytes)
#[inline(never)]
pub fn f_8011dd80(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011dd80) { return; }
        if c.prof { crate::prof::enter(c, 0x8011dd80); p = true; }
    }
    f_8011dd80_b(c, 0x8011dd80);
    if p { crate::prof::exit(c, 0x8011dd80); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011dd80_at(c: &mut Cpu, pc: u32) { f_8011dd80_b(c, pc) }
#[inline(always)]
fn f_8011dd80_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011dd80;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011dd80, pc); }
        match pc {
            0x8011dd80 => {
                { let ea = c.r[1].wrapping_add(0xffffffe0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x24), c.r[0]);
                c.stw(c.r[1].wrapping_add(0x1c), c.r[31]);
                c.stw(c.r[1].wrapping_add(0x18), c.r[30]);
                c.r[30] = c.r[5] | c.r[5];
                c.r[5] = 0x0;
                c.stw(c.r[1].wrapping_add(0x14), c.r[29]);
                c.r[29] = c.r[4] | c.r[4];
                c.stw(c.r[1].wrapping_add(0x10), c.r[28]);
                c.r[28] = c.r[3] | c.r[3];
                c.lr = 0x8011ddb0; f_8011f260(c);
                pc = 0x8011ddb0;
            }
            0x8011ddb0 => {
                c.r[31] = c.r[3] | c.r[3];
                c.rc(c.r[31]);
                if c.crbit(2) { pc = 0x8011dddc; continue; }
                pc = 0x8011ddb8;
            }
            0x8011ddb8 => {
                c.r[3] = c.r[28] | c.r[28];
                c.r[4] = c.r[29] | c.r[29];
                c.lr = 0x8011ddc4; f_80123e70(c);
                pc = 0x8011ddc4;
            }
            0x8011ddc4 => {
                c.r[0] = c.r[30].rotate_left(0) & 0xff;
                c.r[4] = c.r[3].rotate_left(0) & 0xff;
                c.cmpu(0, c.r[4], c.r[0]);
                if !c.crbit(0) { pc = 0x8011ddd8; continue; }
                pc = 0x8011ddd4;
            }
            0x8011ddd4 => {
                c.r[30] = c.r[3] | c.r[3];
                pc = 0x8011ddd8;
            }
            0x8011ddd8 => {
                c.stb(c.r[31].wrapping_add(0x2), c.r[30]);
                pc = 0x8011dddc;
            }
            0x8011dddc => {
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
            _ => panic!("f_8011dd80: bad pc {:#x}", pc),
        }
    }
}

/// pokemonBiosSetPokemonWazaDataId (0x8011ddfc, 60 bytes)
#[inline(never)]
pub fn f_8011ddfc(c: &mut Cpu) {
    let mut p = false;
    if c.hooks {
        if c.stub_hit(0x8011ddfc) { return; }
        if c.prof { crate::prof::enter(c, 0x8011ddfc); p = true; }
    }
    f_8011ddfc_b(c, 0x8011ddfc);
    if p { crate::prof::exit(c, 0x8011ddfc); }
}
/// Enter at a block label (a call's return address: resuming a parked GS thread). No hooks.
pub fn f_8011ddfc_at(c: &mut Cpu, pc: u32) { f_8011ddfc_b(c, pc) }
#[inline(always)]
fn f_8011ddfc_b(c: &mut Cpu, mut pc: u32) {
    c.last_fn = 0x8011ddfc;
    loop {
        c.steps += 1;
        if c.steps > c.step_limit { c.step_event(0x8011ddfc, pc); }
        match pc {
            0x8011ddfc => {
                { let ea = c.r[1].wrapping_add(0xfffffff0); c.stw(ea, c.r[1]); c.r[1] = ea; }
                c.r[0] = c.mfspr(8);
                c.stw(c.r[1].wrapping_add(0x14), c.r[0]);
                c.stw(c.r[1].wrapping_add(0xc), c.r[31]);
                c.r[31] = c.r[5] | c.r[5];
                c.r[5] = 0x0;
                c.lr = 0x8011de18; f_8011f260(c);
                pc = 0x8011de18;
            }
            0x8011de18 => {
                c.cmpu(0, c.r[3], 0x0);
                if c.crbit(2) { pc = 0x8011de24; continue; }
                pc = 0x8011de20;
            }
            0x8011de20 => {
                c.sth(c.r[3], c.r[31]);
                pc = 0x8011de24;
            }
            0x8011de24 => {
                c.r[0] = c.lwz(c.r[1].wrapping_add(0x14));
                c.r[31] = c.lwz(c.r[1].wrapping_add(0xc));
                c.mtspr(8, c.r[0]);
                c.r[1] = c.r[1].wrapping_add(0x10);
                if true { return; }
                return;
            }
            _ => panic!("f_8011ddfc: bad pc {:#x}", pc),
        }
    }
}

